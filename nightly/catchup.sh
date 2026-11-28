#!/bin/zsh
# Make up nightly runs missed while the Mac was off or asleep: what systemd's
# timers do with Persistent=true, which launchd lacks (it catches up runs
# missed in sleep, not across a shutdown). Run at login and hourly by the
# launchd agent pub.atec.catchup.
#
# A job is due when its last successful run (recorded by mark_ok in
# nightly/common.sh) is older than its period plus a margin, and it isn't
# running now; then it's started (launchctl kickstart). The long fetches
# run for days on a first pass: while running they're left alone, and if a
# shutdown killed one, it's started again here rather than at 23:00.
#
#     nightly/catchup.sh [--dry-run]

dry=0; [[ $1 == --dry-run ]] && dry=1
uid=$(id -u)
oks=~/Library/Caches/pub.atec.nightly
now=$(date +%s)

# label, hours after which a missed run is made up
jobs=(
  pub.atec.edgar.all 26
  pub.atec.edinet.all 26
  pub.atec.equities.nightly 26
  pub.atec.equities.nikkei 26
  pub.atec.equities.taiwan 26
  pub.atec.fred 26
  pub.atec.plaid 26
)

for label hours in $jobs; do
  state=$(launchctl print gui/$uid/$label 2>/dev/null | awk '/^\tstate = /{print $3}')
  [[ -z $state ]] && continue                 # not installed here
  [[ $state == running ]] && continue
  last=$(stat -f %m $oks/ok.$label 2>/dev/null || echo 0)
  age=$(( (now - last) / 3600 ))
  if (( age >= hours )); then
    echo "$(date '+%F %T') $label: last success ${age}h ago, starting it"
    (( dry )) || launchctl kickstart gui/$uid/$label
  fi
done
