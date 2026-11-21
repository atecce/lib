#!/bin/zsh
# Nightly: bring the EDGAR cache up to date for a list of companies (the edgar
# crate's `fetch`), then push the ones that got new filings to InfluxDB.
#
#     run.sh LIST     e.g. equities/sp500/all.txt, equities/nikkei/all.txt
#
# SEC rebuilds its indexes around 05:15 UTC (21:15 Pacific); the agents run
# after that, or on wake if the Mac was asleep. One fetch at a time across
# jobs (a lock on the cache), so together they stay under SEC's rate limit.
source ~/.profile
# The checkout to run in: $LIB if set, else the one this script is in
# ($LIB/equities/nightly/run.sh).
LIB=${LIB:-${0:A:h:h:h}}
cd $LIB || exit 1
list=${1:?usage: run.sh LIST}
name=${${list:h}:t}   # equities/nikkei/all.txt -> nikkei
log=equities/nightly/log.$name.$(date +%Y%m%d-%H%M%S)

# fetch prints the tickers with new filings on stdout, progress on stderr.
new=$(/usr/bin/lockf edgar/cache/.fetch.lock ./target/release/fetch --list $list 2>>$log)
fetched=$?
echo "=== $name new filings: ${new:-none}" | tee -a $log
if [ -n "$new" ]; then
  ./target/release/lib $(echo $new) >> $log 2>&1 || exit 1
fi
# Non-zero when a company could not be fetched, so launchd shows the failure.
exit $fetched
