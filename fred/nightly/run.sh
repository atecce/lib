#!/bin/zsh
# Daily: bring the FRED series in fred/rates.txt up to date, then load them
# into Postgres (store-fred writes only new and revised observations).
#
#     run.sh
#
# FRED publishes the Treasury yields (H.15) for each business day late that
# afternoon; this runs in the morning, for the day before.
LIB=${LIB:-${0:A:h:h:h}}
cd $LIB || exit 1
source nightly/common.sh
log=fred/nightly/log.$(date +%Y%m%d-%H%M%S)
wait_for "curl -s -o /dev/null -m 10 -A lib-fetch https://fred.stlouisfed.org/" || exit 1
wait_for "/opt/homebrew/opt/postgresql@17/bin/pg_isready -q" 600 || exit 1
./target/release/fred-fetch --list fred/rates.txt >> $log 2>&1 || exit 1
./target/release/store-fred >> $log 2>&1 || exit 1
mark_ok pub.atec.fred
