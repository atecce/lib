# equities-api

What the Optimates app ([atecce/optimates](https://github.com/atecce/optimates))
reads, from Postgres, as `https://api.atec.pub`.

```
phone ──► Cloudflare Access ──► tunnel ──► equities-api (127.0.0.1:8787) ──► Postgres
          email + one-time code              checks Access's signed
          (the 家人 (jiārén) policy)          assertion; holds the
                                             database credentials
```

The app's only credential is its Cloudflare Access sign-in. Access lets a
request through after its login (or a service token, for the Mac and
scripts) and attaches its verdict, `Cf-Access-Jwt-Assertion`, a JWT signed
by the team's keys. The service checks it on every request (`src/access.rs`:
signature, issuer, the application's audience tag, expiry), so a request
reaching it some other way is refused too, and nothing but the service
touches Postgres.

## Endpoints

| | |
|---|---|
| `GET /health` | `ok` (unchecked: for the tunnel) |
| `GET /tickers` | listings whose company filed in the last 400 days |
| `GET /dashboard?ticker=AAPL&items=Revenue,NetIncome&periods=3m,pit&market=market_cap` | each reported item for each period, as last reported (`listing_facts`), and each market field (`close`, `market_cap`, `price_to_book`, `return_on_equity`, `earnings_yield`), in every currency the company reports in |

A dashboard is a JSON array of `{item, period, currency, points: [{date,
value}]}`; market fields have period `market`. A whole dashboard takes
30–80 ms.

## Running it

```sh
cargo build --release -p equities-api
# local development, nothing checked:
./target/release/equities-api --no-access
# as deployed (pub.atec.equities-api.plist keeps it running):
ACCESS_TEAM=steep-snow-0aca ACCESS_AUD=<the Access app's AUD> ./target/release/equities-api
```

`STORE_URL` names another database (default `host=localhost dbname=financials`).

## Cloudflare

- The tunnel routes `api.atec.pub` to `http://localhost:8787`.
- An Access application for `api.atec.pub` allows the 家人 group (email
  one-time PIN) and the `equities-app` service token. Its audience tag is
  `ACCESS_AUD`.
