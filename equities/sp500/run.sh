#!/bin/zsh
# Resumable S&P 500 run, 25 tickers at a time, each batch pushed to InfluxDB
# when it finishes. Tickers listed in priority.txt (one per line, any symbol)
# go first; finished tickers are recorded in done_tickers.txt and skipped.
source ~/.profile
# The checkout to run in: $LIB if set, else the one this script is in
# ($LIB/equities/sp500/run.sh).
LIB=${LIB:-${0:A:h:h:h}}
cd $LIB || exit 1
S=equities/sp500
touch $S/done_tickers.txt $S/priority.txt
# The run order: the constituent list, unless an order was already written.
[ -f $S/order.txt ] || grep -v '^#' $S/all.txt | tr -s ' ' '\n' | grep . > $S/order.txt
failed=$(mktemp)   # tickers whose batch failed in this pass

# Next up to 25 tickers from a list that are not done or failed this pass.
next() {
  tr -s ' \t,' '\n' < $1 | tr a-z A-Z | grep -xE '[A-Z0-9.-]{1,10}' \
    | grep -vxF -f $S/done_tickers.txt -f $failed | awk '!seen[$0]++' | head -25
}

while true; do
  batch=$(next $S/priority.txt); src=priority
  [ -z "$batch" ] && { batch=$(next $S/order.txt); src=order; }
  [ -z "$batch" ] && break
  echo "=== $src batch: $(echo $batch | tr '\n' ' ')" | tee -a $S/progress.log
  log=$S/log.$(date +%Y%m%d-%H%M%S)
  if ./target/release/lib $(echo $batch) > $log 2>&1; then
    echo $batch >> $S/done_tickers.txt
    echo "    ok: $(tail -1 $log)" | tee -a $S/progress.log
  else
    echo $batch >> $failed
    echo "    FAILED (exit $?): $(tail -1 $log)" | tee -a $S/progress.log
  fi
done

if [ -s $failed ]; then
  echo "=== pass finished; $(wc -l < $failed | tr -d ' ') ticker(s) failed, retrying" | tee -a $S/progress.log
  rm -f $failed; exit 1
fi
rm -f $failed
echo "=== all tickers finished" | tee -a $S/progress.log
# Started by launchd: remove the agent so it does not run again at login.
if [ "$XPC_SERVICE_NAME" = pub.atec.sp500 ]; then
  rm -f ~/Library/LaunchAgents/pub.atec.sp500.plist
  launchctl bootout gui/$(id -u)/pub.atec.sp500
fi
