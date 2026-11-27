#!/bin/zsh
# Nightly: bring the EDGAR cache up to date for every company with a ticker
# (one ticker per company, from the SEC's ticker map), using EDGAR's bulk
# submissions.zip for the filing lists.
#
#     all.sh
#
# Fetch only: equities/nightly/run.sh pushes to InfluxDB in the morning.
#
# The first run fills the cache over a few days (about two million requests at
# the SEC's limit); filings already on disk are skipped, so an interrupted run
# resumes where it stopped, and once caught up a night is only the day's new
# filings. launchd starts no second copy while one is still running.
#
# Companies go in chunks, each holding the cache lock only while it fetches,
# so the other nightly jobs (S&P 500, Nikkei) get their turn in between and
# together they stay under the SEC's rate limit.
LIB=${LIB:-${0:A:h:h:h}}
cd $LIB || exit 1
log=edgar/nightly/log.all.$(date +%Y%m%d-%H%M%S)
chunk=250
fetch=./target/release/fetch
map=edgar/cache/company_tickers.json

# The ticker map; fetch downloads it (and refreshes it weekly) on first use.
[ -f $map ] || $fetch AAPL >/dev/null 2>>$log

# One ticker per company, the map's first for it: the map is in order of
# market value, so the largest companies come first.
list=edgar/nightly/companies.txt
jq -r '[to_entries[] | {k: (.key | tonumber), c: .value.cik_str, t: .value.ticker}]
       | group_by(.c) | map(min_by(.k)) | sort_by(.k) | .[].t' $map > $list || exit 1
total=$(wc -l < $list | tr -d ' ')
echo "=== $total companies, $chunk at a time" >> $log

zmodload zsh/system
touch edgar/cache/.fetch.lock
failed=0
for ((i = 0; i < total; i += chunk)); do
  part=edgar/nightly/companies.part
  tail -n +$((i + 1)) $list | head -n $chunk > $part
  zsystem flock -f lock edgar/cache/.fetch.lock || exit 1
  $fetch --bulk --list $part 2>>$log | sed 's/^/new: /' >> $log
  (( pipestatus[1] )) && failed=1
  zsystem flock -u $lock
  echo "=== $((i + $(wc -l < $part))) of $total" >> $log
  # Let a job waiting on the lock take it before the next chunk does.
  sleep 5
done
rm -f $part
# Non-zero when a company could not be fetched, so launchd shows the failure.
exit $failed
