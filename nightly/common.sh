# Sourced by the nightly jobs (edgar/nightly/all.sh, edinet/nightly/all.sh,
# equities/nightly/run.sh). launchd has no dependencies between jobs, as
# systemd's Requires=/After= would give, so each job waits for what it needs
# itself, and records a successful run for nightly/catchup.sh (systemd's
# Persistent=true: runs missed while the Mac was off are made up at login).

# Wait up to $2 seconds (default 1800) for `$1` to succeed, every 10 s.
wait_for() {
  local check=$1 limit=${2:-1800} waited=0
  until eval "$check" >/dev/null 2>&1; do
    (( waited >= limit )) && { echo "gave up waiting for: $check" >&2; return 1 }
    sleep 10; (( waited += 10 ))
  done
}

# The network: EDGAR and EDINET's servers answer.
wait_for_network() {
  wait_for "curl -s -o /dev/null -m 10 -A 'lib nightly' https://www.sec.gov/robots.txt" ${1:-1800}
}

# The stores the push writes: Postgres, and InfluxDB until it's retired.
wait_for_stores() {
  wait_for "/opt/homebrew/opt/postgresql@17/bin/pg_isready -q" 600
}

# Record a successful run of job $1 (its launchd label), for catchup.sh.
mark_ok() {
  mkdir -p ~/Library/Caches/pub.atec.nightly && touch ~/Library/Caches/pub.atec.nightly/ok.$1
}
