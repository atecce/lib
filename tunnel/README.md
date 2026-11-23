# Cloudflare tunnel

InfluxDB and Grafana on this Mac, reachable from anywhere through Cloudflare,
with nothing open on the router. Set up 2026-11-22.

```
phone / browser ──HTTPS──▶ Cloudflare ──▶ Access (who is asking?)
                                              │
                        tunnel "lib", opened outward by the Mac
                                              ▼
                 cloudflared ──▶ influx.atec.pub  → localhost:8181 (InfluxDB)
                             └─▶ grafana.atec.pub → localhost:3000 (Grafana)
```

## Who gets in

| Address | Access rule | Then |
|---|---|---|
| `influx.atec.pub` | the service token `equities-app` only (headers `CF-Access-Client-Id`, `CF-Access-Client-Secret`) | InfluxDB's own token |
| `grafana.atec.pub` | the Access group 家人 (*jiā rén*, family), by one-time PIN: a code sent by email | signed in to Grafana as that email (below) |

To let someone see Grafana, add their email to the group 家人 (Zero Trust →
Access → Access groups); the Grafana application's policy admits the group.
They type their email at the login page and get a code; no account anywhere.
They become a Grafana Viewer, named by their email until their name is set
in Grafana (Administration → Users) after their first visit. (The members'
addresses live in Cloudflare, not here: this repository is public.)

## Where things are

- **Cloudflare** (Zero Trust, team `steep-snow-0aca.cloudflareaccess.com`):
  the tunnel and its routes, both Access applications, the service token
  (expires 2027-11-22), the one-time PIN login, DNS records `influx` and
  `grafana` (CNAMEs to the tunnel, proxied).
- **This Mac, not in git** (`~/.cloudflared/`, readable only by its owner):
  - `lib-tunnel.token`: the tunnel's token, for cloudflared.
  - `equities-app.env`: the service token, as `CF_ACCESS_CLIENT_ID` and
    `CF_ACCESS_CLIENT_SECRET`.
  - `launchd.log`: cloudflared's log.
- **This directory:** `pub.atec.cloudflared.plist`, the launchd agent that
  keeps cloudflared running (install instructions inside).
- **Grafana** (`/opt/homebrew/etc/grafana/grafana.ini`, `[auth.jwt]`):
  trusts the JWT Access sends in `Cf-Access-Jwt-Assertion`, checked against
  the team's keys and the Grafana application's audience tag, and signs that
  email in, creating the user the first time: `root@atec.pub` as Admin,
  others as Viewer. Requests to localhost:3000, without the header, log in
  as before. The config before this change is `grafana.ini.before-jwt`.

## Changing it

The routes, Access rules and DNS records are changed in the dashboard or
through the API with `CLOUDFLARE_API_TOKEN` (in `~/.profile`; account
permissions Access apps/policies, service tokens, identity providers, and
Cloudflare Tunnel, all edit; DNS edit on `atec.pub` only).

A route without an Access application in front of it is open to the
internet: create the application first, then the DNS record.
