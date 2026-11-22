#!/bin/zsh
# Nightly: bring the EDINET cache up to date for every listed company: the
# day lists, then the reports (annual, semiannual and quarterly, as CSV) of
# each company not yet on disk.
#
#     all.sh
#
# Fetch only: equities/nightly/run.sh pushes to InfluxDB in the morning.
#
# The first run fills the cache over a day or two (about 140,000 reports at
# two requests a second); reports already on disk are skipped, so an
# interrupted run resumes where it stopped, and once caught up a night is
# only the day's new reports. launchd starts no second copy while one runs.
#
# Companies go in chunks, each holding EDINET's lock (edinet/cache, not
# EDGAR's: a different server) only while it fetches, so another job's
# EDINET fetch gets its turn in between.
source ~/.profile   # EDINET_API_KEY
LIB=${LIB:-${0:A:h:h:h}}
cd $LIB || exit 1
log=edinet/nightly/log.all.$(date +%Y%m%d-%H%M%S)
chunk=250
fetch=./target/release/edinet-fetch

zmodload zsh/system
touch edinet/cache/.fetch.lock
lock() { zsystem flock -f lock edinet/cache/.fetch.lock }

# The day lists, then every listed company in them, in securities code order.
list=edinet/nightly/companies.txt
lock || exit 1
$fetch --listed > $list.new 2>>$log
s=$?
zsystem flock -u $lock
(( s )) && exit 1
mv $list.new $list
total=$(wc -l < $list | tr -d ' ')
echo "=== $total companies, $chunk at a time" >> $log

failed=0
part=edinet/nightly/companies.part
for ((i = 0; i < total; i += chunk)); do
  tail -n +$((i + 1)) $list | head -n $chunk > $part
  lock || exit 1
  # The day lists are fresh from above: start them at today, so each chunk
  # fetches only today's again.
  $fetch --from $(date +%Y-%m-%d) --list $part 2>>$log | sed 's/^/new: /' >> $log
  (( pipestatus[1] )) && failed=1
  zsystem flock -u $lock
  echo "=== $((i + $(wc -l < $part))) of $total" >> $log
  # Let a job waiting on the lock take it before the next chunk does.
  sleep 5
done
rm -f $part
# Non-zero when a company could not be fetched, so launchd shows the failure.
exit $failed
