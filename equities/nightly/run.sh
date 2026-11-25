#!/bin/zsh
# Nightly: write a list of companies to Postgres (InfluxDB was retired
# 2026-11-25; the app reads Postgres through the equities API, Grafana reads
# it directly).
#
#     run.sh LIST     e.g. equities/sp500/all.txt, equities/nikkei/all.txt
#
# EDGAR is fetched for every company by edgar/nightly/all.sh (23:00), so this
# runs in the morning, after it, and fetches only what is particular to the
# list: Tokyo tickers (7203.T) from EDINET, Taipei ones (2330.TW) from MOPS,
# and for a small list, prices.
#
# A large list (the S&P 500) pushes the companies whose filings changed since
# its last push (edgar's `changed`, against the stamp file pushed.NAME),
# whichever job fetched them; a small one pushes everything, since market
# values change every day.
#
# One fetch at a time per service, so jobs together stay under each one's
# rate limit: each fetch holds a lock in that service's cache.
source ~/.profile
# The checkout to run in: $LIB if set, else the one this script is in
# ($LIB/equities/nightly/run.sh).
LIB=${LIB:-${0:A:h:h:h}}
cd $LIB || exit 1
# Waits for what the job needs, and records success (nightly/common.sh).
source nightly/common.sh
list=${1:?usage: run.sh LIST}
name=${${list:h}:t}   # equities/nikkei/all.txt -> nikkei
log=equities/nightly/log.$name.$(date +%Y%m%d-%H%M%S)
stamp=equities/nightly/pushed.$name
# Filings written from now on count for the next run; this becomes the
# stamp once the push succeeds.
touch $stamp.next

zmodload zsh/system
# Run a command holding the lock of a service's cache (edinet/cache, ...).
locked() {
  local lockfile=$1/.fetch.lock lock; shift
  touch $lockfile
  zsystem flock -f lock $lockfile || return 1
  "$@"
  local s=$?
  zsystem flock -u $lock
  return $s
}

wait_for_network || exit 1
tickers=$(grep -v '^#' $list | tr -s ' \t,' '\n' | grep .)
fetched=0
tokyo=$(echo $tickers | grep -iE '\.T$')
if [ -n "$tokyo" ]; then
  locked edinet/cache ./target/release/edinet-fetch ${=tokyo} 2>>$log | sed 's/^/edinet new: /' >> $log
  (( pipestatus[1] )) && fetched=1
fi

# Taipei tickers (2330.TW) from MOPS, under its own lock.
taipei=$(echo $tickers | grep -iE '\.TW$')
if [ -n "$taipei" ]; then
  locked mops/cache ./target/release/mops-fetch ${=taipei} 2>>$log | sed 's/^/mops new: /' >> $log
  (( pipestatus[1] )) && fetched=1
fi

# Tiingo's free plan: 500 symbols a month, 50 requests an hour.
if [ $(echo $tickers | wc -l) -le 60 ]; then
  locked equities/tiingo/cache ./target/release/tiingo-fetch ${=tickers} >> $log 2>&1 || fetched=1
  push=$tickers
else
  push=$(./target/release/changed $stamp --list $list 2>>$log)
fi
echo "=== $name push: $(echo $push | wc -w | tr -d ' ') companies" >> $log
wait_for_stores || exit 1
if [ -n "$push" ]; then
  # One company at a time (see DATA.md).
  ./target/release/lib --postgres-only ${=push} >> $log 2>&1 || exit 1
fi
mv $stamp.next $stamp
# For catchup.sh: the launchd job this list runs as.
case $name in
  sp500) mark_ok pub.atec.equities.nightly ;;
  *) mark_ok pub.atec.equities.$name ;;
esac
# Non-zero when a fetch failed, so launchd shows the failure.
exit $fetched
