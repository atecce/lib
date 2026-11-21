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

# One fetch at a time: wait for the lock (zsh's flock), hold it while fetching.
zmodload zsh/system
touch edgar/cache/.fetch.lock
zsystem flock -f lock edgar/cache/.fetch.lock || exit 1
# fetch prints the tickers with new filings on stdout, progress on stderr.
new=$(./target/release/fetch --list $list 2>>$log)
fetched=$?
zsystem flock -u $lock
echo "=== $name new filings: ${new:-none}" | tee -a $log

tickers=$(grep -v '^#' $list | tr -s ' \t,' '\n' | grep .)
# Tokyo tickers (7203.T) come from EDINET, which has no lock: its own server.
tokyo=$(echo $tickers | grep -iE '\.T$')
if [ -n "$tokyo" ]; then
  ./target/release/edinet-fetch $(echo $tokyo) 2>>$log | sed 's/^/edinet new: /' >> $log
fi

# A small list also gets daily prices (Tiingo's free plan: 500 symbols a
# month, 50 requests an hour), and all of it is pushed: market values change
# every day. A large one (the S&P 500) pushes only companies with new filings.
if [ $(echo $tickers | wc -l) -le 60 ]; then
  ./target/release/tiingo-fetch $(echo $tickers) >> $log 2>&1
  ./target/release/lib $(echo $tickers) >> $log 2>&1 || exit 1
elif [ -n "$new" ]; then
  ./target/release/lib $(echo $new) >> $log 2>&1 || exit 1
fi
# Non-zero when a company could not be fetched, so launchd shows the failure.
exit $fetched
