# The nightly jobs

All launchd agents (per user, so they run while you're logged in).

| Label | When | Does | Needs |
|---|---|---|---|
| `pub.atec.edgar.all` | 23:00 | fetch EDGAR for every company (`edgar/nightly/all.sh`) | network |
| `pub.atec.edinet.all` | 23:30 | fetch EDINET for every listed company (`edinet/nightly/all.sh`) | network |
| `pub.atec.equities.nightly` | 06:00 | push the S&P 500's changed companies (`equities/nightly/run.sh`) | Postgres, InfluxDB, network |
| `pub.atec.equities.nikkei` | 06:30 | prices and push for the Nikkei list | same |
| `pub.atec.equities.taiwan` | 06:40 | MOPS fetch, prices and push for the Taiwan list | same |
| `pub.atec.catchup` | at login, hourly | make up any of the above that missed a run | |
| `pub.atec.cloudflared` | always | the tunnel to `grafana.atec.pub` (`tunnel/`) | network |
| Postgres, InfluxDB, Grafana | always | Homebrew services (`brew services`) | |

## Dependencies

launchd has no dependency graph (systemd's `Requires=`/`After=`), so:

- **Each job waits for what it needs** (`nightly/common.sh`): the fetches
  for the network, the pushes for Postgres and InfluxDB, giving up after a
  while with a failure launchd shows.
- **Order comes from the clock and from locks**: fetches at night, pushes in
  the morning; one fetch per service at a time (a lock in each cache). A
  push during a fetch takes what's there, and its stamp catches the rest the
  next morning.
- **Missed runs are made up** (`nightly/catchup.sh`, systemd's
  `Persistent=true`): launchd makes up runs missed in sleep, not across a
  shutdown. Each job records a finished run (`mark_ok`, in
  `~/Library/Caches/pub.atec.nightly/`); at login and hourly, a job not
  running whose last finished run is over 26 hours old is started. A fetch
  killed by a shutdown resumes then, rather than at its next scheduled time;
  it skips what's already cached.

## After a power cut

Everything is written so that an interruption costs only the work in
flight: downloads to a temporary name then renamed, Postgres one company per
transaction, push stamps advanced only after a successful push, and
Postgres and InfluxDB recover from their write-ahead logs.

FileVault is on, so after a power cut the Mac waits at the unlock screen
until someone enters the password; unlocking logs that user in, which starts
every agent above. System-wide daemons (started at boot, before login) would
add nothing while FileVault holds the boot, which is why the services are
user agents. Keeping `grafana.atec.pub` up means keeping the Mac powered and
unlocked once.
