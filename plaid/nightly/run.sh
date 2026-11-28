#!/bin/zsh
# Daily: MINT's and SPY's prices from Tiingo (the portfolio's r, and the market:
# what each position is measured against), then every Plaid connection into Postgres.
#
#     run.sh
LIB=${LIB:-${0:A:h:h:h}}
cd $LIB || exit 1
# TIINGO_API_KEY.
source ~/.profile
source nightly/common.sh
log=plaid/nightly/plaid.log
echo "$(date '+%F %T') run" >> $log
./target/release/tiingo-fetch MINT SPY >> $log 2>&1
./target/release/store-plaid >> $log 2>&1 || exit 1
mark_ok pub.atec.plaid
