#!/usr/bin/env python3
"""Progress bar for the S&P 500 run, read from the files run.sh writes.

    python3 equities/sp500/progress.py          # redraws every 5 seconds
    python3 equities/sp500/progress.py --once   # one snapshot

Overall progress is done_tickers.txt against order.txt; the current batch is
the newest log.<timestamp>, which gets a "TICKER: CIK" line as each company
starts. Time left comes from how long recent batches took, the gaps between
their logs' timestamps.
"""

import glob
import os
import shutil
import sys
import time
from datetime import datetime

DIR = os.path.dirname(os.path.abspath(__file__))


def lines(name):
    try:
        with open(os.path.join(DIR, name)) as f:
            return [l.strip() for l in f if l.strip()]
    except FileNotFoundError:
        return []


def bar(fraction, width):
    filled = int(round(fraction * width))
    return "█" * filled + "░" * (width - filled)


def started(log):
    return datetime.strptime(os.path.basename(log)[4:], "%Y%m%d-%H%M%S")


def snapshot():
    order = lines("order.txt")
    done = set(lines("done_tickers.txt"))
    total = len(order) or 1
    finished = sum(1 for t in order if t in done)

    logs = sorted(glob.glob(os.path.join(DIR, "log.2*")))
    current, batch, now_on = None, 0, "-"
    if logs:
        current = logs[-1]
        with open(current) as f:
            starts = [l.split(":")[0] for l in f if ": CIK " in l]
        batch = len(starts)
        now_on = starts[-1] if starts else "-"

    # Batch durations from consecutive log timestamps; the newest batch is
    # still running. A gap over 2 hours is a restart or a sleep, not a batch.
    durations = [
        (started(b) - started(a)).total_seconds()
        for a, b in zip(logs, logs[1:])
        if 0 < (started(b) - started(a)).total_seconds() < 7200
    ][-4:]
    eta = "-"
    if finished >= total:
        eta = "done"
    elif durations:
        per_batch = sum(durations) / len(durations)
        # What is left of the running batch, then the batches after it.
        elapsed = (datetime.now() - started(current)).total_seconds() if current else 0
        after = max(0, total - finished - 25)
        secs = max(0, per_batch - elapsed) + after / 25 * per_batch
        eta = f"{int(secs // 3600)}h{int(secs % 3600 // 60):02d}m, around {datetime.fromtimestamp(time.time() + secs):%H:%M}"

    width = max(20, min(60, shutil.get_terminal_size().columns - 30))
    return "\n".join([
        f"S&P 500   {bar(finished / total, width)} {finished}/{total}",
        f"batch     {bar(min(batch, 25) / 25, width)} {batch}/25, now {now_on}",
        f"time left {eta}",
    ])


if __name__ == "__main__":
    if "--once" in sys.argv:
        print(snapshot())
        sys.exit()
    try:
        while True:
            # Clear the screen and draw from the top.
            sys.stdout.write("\033[H\033[2J" + snapshot() + "\n")
            sys.stdout.flush()
            time.sleep(5)
    except KeyboardInterrupt:
        print()
