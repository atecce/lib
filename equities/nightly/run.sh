#!/bin/zsh
# Nightly: bring the EDGAR cache up to date for the S&P 500 (the edgar
# crate's `fetch`), then push the companies that got new filings to InfluxDB.
# SEC rebuilds its indexes around 05:15 UTC (21:15 Pacific); the agent runs
# after that, or on wake if the Mac was asleep.
source ~/.profile
# The checkout to run in: $LIB if set, else the one this script is in
# ($LIB/equities/nightly/run.sh).
LIB=${LIB:-${0:A:h:h:h}}
cd $LIB || exit 1
N=equities/nightly
log=$N/log.$(date +%Y%m%d-%H%M%S)

# fetch prints the tickers with new filings on stdout, progress on stderr.
new=$(./target/release/fetch --list equities/sp500/all.txt 2>>$log)
fetched=$?
echo "=== new filings: ${new:-none}" | tee -a $log
if [ -n "$new" ]; then
  ./target/release/lib $(echo $new) >> $log 2>&1 || exit 1
fi
# Non-zero when a company could not be fetched, so launchd shows the failure.
exit $fetched
