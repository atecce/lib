# plaid

Plaid as a source: a brokerage's holdings and investment transactions,
read-only (Plaid's Investments product; nothing here can trade or move money).

```
cargo run -p plaid --bin plaid-link       # connect an institution (once): a page on 127.0.0.1 with Plaid Link
cargo run -p plaid --bin plaid-holdings   # each connection's holdings and unrealized gains
store-plaid                               # (equities/store) holdings and transactions into Postgres
```

Secrets stay on the Mac, out of the repository, mode 0600, with copies in
Keybase (`etc/plaid`):

- `~/.plaid/env`: `PLAID_CLIENT_ID`, `PLAID_SECRET` (production),
  `PLAID_SANDBOX_SECRET`, `PLAID_ENV` (`production` or `sandbox`);
- `~/.plaid/items/<item_id>.json`: each connection's access token;
- `~/.plaid/owners`: who may see the portfolio through the equities API.

Robinhood connects through its own OAuth login. It reports a cost basis for
long positions; shorts get theirs from the short sales on record (Plaid keeps
24 months of transactions); options are shown by value only, for now.

`nightly/pub.atec.plaid.plist` runs `store-plaid` daily at 07:20; the app's
pull to refresh runs it through the equities API (Holdings, for the owner
only). A connection is revoked at my.plaid.com or in Robinhood's settings.
