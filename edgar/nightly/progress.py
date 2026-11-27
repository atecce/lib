#!/usr/bin/env python3
"""Progress bar for the EDGAR fetch of every company (all.sh), read from the
files it writes.

    python3 edgar/nightly/progress.py          # redraws every 5 seconds
    python3 edgar/nightly/progress.py --once   # one snapshot

The run's order is companies.txt, and the newest log.all.<timestamp> gets a
line per company as each finishes ("AAPL: 129 filings, 0 new ..., 0
requests"), so how far it is, is the position in companies.txt of the last
company logged. Time left assumes the rest go at the run's average pace so
far, or, while this redraws, at the pace of the last half hour.
"""

import glob
import os
import re
import shutil
import sys
import time
from datetime import datetime

DIR = os.path.dirname(os.path.abspath(__file__))
CACHE = os.path.join(DIR, "..", "cache")
CHUNK = 250
DONE = re.compile(r"^([A-Z0-9.-]+): (\d+) filings, (\d+) new .*?(\d+) requests(, (\d+) failed)?")
FAILED = re.compile(r"^([A-Z0-9.-]+): (?!\d+ filings)")


def bar(fraction, width):
    filled = int(round(min(1, fraction) * width))
    return "█" * filled + "░" * (width - filled)


def duration(secs):
    return f"{int(secs // 3600)}h{int(secs % 3600 // 60):02d}m"


def snapshot(history):
    logs = sorted(glob.glob(os.path.join(DIR, "log.all.*")))
    if not logs:
        return "no run yet (edgar/nightly/log.all.*)"
    log = logs[-1]
    started = datetime.strptime(os.path.basename(log)[8:], "%Y%m%d-%H%M%S")
    try:
        with open(os.path.join(DIR, "companies.txt")) as f:
            order = {t.strip(): i for i, t in enumerate(f) if t.strip()}
    except FileNotFoundError:
        order = {}
    total = len(order) or 1

    position, now_on, requests, new, failed, companies = 0, "-", 0, 0, 0, 0
    finished = False
    with open(log) as f:
        for line in f:
            if m := DONE.match(line):
                companies += 1
                requests += int(m[4])
                new += int(m[3])
                failed += int(m[6] or 0)
            elif m := FAILED.match(line):
                if m[1] in order and " filings" not in line and "(" not in line.split(":")[0]:
                    companies += 1
                    failed += 1
            else:
                continue
            if m[1] in order:
                position = max(position, order[m[1]] + 1)
                now_on = m[1]
    with open(log) as f:
        tail = f.read()[-2000:]
    if re.search(rf"=== {total} of {total}", tail):
        finished, position = True, total

    now = time.time()
    elapsed = now - started.timestamp()
    history.append((now, position, requests))
    # The pace of the last half hour, once there is one; else the average.
    window = [h for h in history if now - h[0] <= 1800]
    t0, p0, r0 = window[0] if len(window) > 1 and now - window[0][0] > 60 else (started.timestamp(), 0, 0)
    span = max(1, now - t0)
    per_company = span / max(1, position - p0) if position > p0 else None
    rate = (requests - r0) / span

    if finished:
        eta = "done"
    elif per_company:
        secs = (total - position) * per_company
        eta = f"{duration(secs)}, around {datetime.fromtimestamp(now + secs):%a %H:%M}"
    else:
        eta = "-"

    chunk_done = position - (position - 1) // CHUNK * CHUNK if position else 0
    free = shutil.disk_usage(DIR).free / 2**30
    width = max(20, min(60, shutil.get_terminal_size().columns - 30))
    return "\n".join([
        f"EDGAR     {bar(position / total, width)} {position}/{total}",
        f"chunk     {bar(chunk_done / CHUNK, width)} {chunk_done}/{CHUNK}, last {now_on}",
        f"requests  {requests:,} ({rate:.1f}/s), {new:,} new filings, {failed} failed",
        f"running   {duration(elapsed)} since {started:%a %H:%M}, {free:,.0f} GB free",
        f"time left {eta}",
    ])


if __name__ == "__main__":
    history = []
    if "--once" in sys.argv:
        print(snapshot(history))
        sys.exit()
    try:
        while True:
            # Clear the screen and draw from the top.
            sys.stdout.write("\033[H\033[2J" + snapshot(history) + "\n")
            sys.stdout.flush()
            time.sleep(5)
    except KeyboardInterrupt:
        print()
