# Apple (AAPL)

Notes from 2026-11-23: the tests of `VALUATION.md` section 8 (entrepreneurial
anomalies, persistent residual income) run on Apple, and what they say about
its rise and Steve Jobs's stewardship. Reproduce with

```sh
cargo run --release -p store --example persistence -- AAPL            # r = 9%
cargo run --release -p store --example persistence -- AAPL --r 0.07
```

## What the data covers

| | |
|---|---|
| Statements (XBRL, EDGAR) | fiscal 2009–2026 (years end in late September), as first reported |
| Prices (Tiingo) | 1990–2026, daily |
| Industry | SIC 3571, electronic computers; group 357, computer and office equipment |

Jobs returned to Apple in 1997 (interim CEO that September), resigned as CEO
on August 24, 2011, and died on October 5, 2011. XBRL statements begin in
2009, so the statements cover his last years and then Tim Cook's; his
fourteen years from 1997 are, so far, in the prices only.

## Residual income and what the price implied

With $`r = 9\%`$, residual income $`x^a_t = E_t - r\,B_{t-1}`$, market value
$`M`$ the fiscal year end's close times the shares on the 10-K's cover, and
the implied decay $`\omega = R(M - B)/(x^a + M - B)`$ (section 8):

| Fiscal year | ROE | Operating income / assets | Residual income | Market value | M/B | ω implied | Implied growth |
|---|---|---|---|---|---|---|---|
| 2011 | 54% | 45% | $22bn | $376bn | 4.9 | 1.02 | +1.7% |
| 2012 | 54% | 47% | $35bn | $628bn | 5.3 | 1.02 | +2.0% |
| 2013 | 31% | 28% | $26bn | $434bn | 3.5 | 1.00 | +0.5% |
| 2014 | 32% | 25% | $28bn | $591bn | 5.3 | 1.03 | +2.9% |
| 2015 | 48% | 31% | $43bn | $640bn | 5.4 | 1.01 | +0.6% |
| 2016 | 38% | 21% | $35bn | $601bn | 4.7 | 1.01 | +1.5% |
| 2017 | 38% | 19% | $37bn | $791bn | 5.9 | 1.03 | +3.2% |
| 2018 | 44% | 19% | $48bn | $1,071bn | 10.0 | 1.04 | +3.9% |
| 2019 | 52% | 17% | $46bn | $972bn | 10.7 | 1.04 | +3.6% |
| 2020 | 63% | 20% | $49bn | $1,909bn | 29.2 | 1.06 | +6.2% |
| 2021 | 145% | 34% | $89bn | $2,410bn | 38.2 | 1.05 | +5.0% |
| 2022 | 158% | 34% | $94bn | $2,393bn | 47.2 | 1.05 | +4.8% |
| 2023 | 191% | 32% | $92bn | $2,663bn | 42.8 | 1.05 | +5.3% |
| 2024 | 151% | 35% | $88bn | $3,443bn | 60.5 | 1.06 | +6.2% |
| 2025 | 197% | 36% | $107bn | $3,775bn | 51.2 | 1.06 | +5.9% |
| 2026 | 179% | 44% | $126bn | $4,956bn | 44.2 | 1.06 | +6.2% |

**Realized:** residual income grew **12.4% a year** from 2011 to 2026, and
Apple's own $`\omega`$ fits at **1.12 ± 0.07** over 15 years.

## Against its industry

The issuers in Postgres in SIC group 357, each one's $`\omega`$ fitted the
same way (residual income next year on this year's, both over the same total
assets):

| SIC | Ticker | ω | Years |
|---|---|---|---|
| 3570 | HPE | −0.35 ± 0.34 | 8 |
| 3570 | IBM | 0.87 ± 0.09 | 14 |
| 3571 | **AAPL** | **1.12 ± 0.07** | 15 |
| 3571 | SMCI | 1.08 ± 0.24 | 14 |
| 3572 | NTAP | 0.94 ± 0.15 | 15 |
| 3572 | P (Everpure, formerly Pure Storage) | 0.89 ± 0.09 | 9 |
| 3572 | STX | 0.64 ± 0.25 | 12 |
| 3572 | WDC | 0.66 ± 0.48 | 15 |
| 3576 | ANET | 1.19 ± 0.19 | 11 |
| 3576 | CSCO | 0.84 ± 0.16 | 15 |
| 3576 | FFIV | 1.01 ± 0.07 | 15 |
| 3577 | FTNT | 1.12 ± 0.24 | 11 |
| 3577 | PANW | 1.05 ± 0.29 | 13 |
| | **pooled** | **0.92 ± 0.07** | 171 company-years |

HP (HPQ), Dell and Sandisk have too few usable years: HP and Dell have had
negative book equity, where residual income on book means nothing, and
Sandisk is a recent spin-off.

## What it says

**The market never priced Apple's advantage as fading.** Every year the
implied $`\omega`$ is at or above 1: the price assumed residual income would
last forever, growing a little. But it assumed far less growth than came:

- **2011–2016**, around Jobs's death, the price assumed residual income would
  grow 0.5–3% a year, about a flat perpetuity. 2013 is the low: M/B of 3.5,
  implied growth 0.5%. At $`r = 7\%`$ that year's implied $`\omega`$ falls
  below 1 (0.98): the market was pricing a slow fade. That was the
  "Apple without Jobs is a hardware company" view: Android and Samsung would
  compete its margins down, as competition predicts.
- **What happened** was 12% a year. The realized $`\omega`$ of 1.12 sits
  above its industry's 0.92 and above what the price implied (1.00–1.03 then).
  That gap, persistence greater than priced, is the anomaly of section 8 that
  pays, and in hindsight it did.
- **Jobs's stewardship, in Schumpeter's terms:** the iPod (2001), iPhone
  (2007) and App Store (2008) were the innovation; residual income is the
  temporary monopoly they earned; $`\omega`$ measures how slowly imitation
  wore it down. Imitation came and residual income grew anyway. Measured this
  way the advantage he built outlasted him, which is the strongest thing the
  statements can say about it; whether Cook's operations (supply chain,
  services, buybacks) or Jobs's products deserve the credit, they can't say.
- **The Jobs years in prices.** Split and dividend adjusted closes from
  Postgres:

  | From | To | Multiple | A year |
  |---|---|---|---|
  | 1997-09-16, interim CEO | 2011-08-24, resigned | 69× | 35% |
  | 1997-07-08, the low | 2011-08-24 | 109× | 40% |
  | 2011-08-24 | 2026-11-20 | 30× | 25% |

  What the 1997 price implied about persistence needs that year's book value
  and earnings, from 10-Ks filed before XBRL (below).

## Caveats, seen here

- **Buybacks wreck ROE.** From 2021 ROE runs 145–197%: Apple has bought back
  $70–95 billion a year, shrinking book equity to $50–110 billion against a
  market value in the trillions. Operating income over assets (17–45%) is the
  honest profitability. Residual income stays usable, since the charge on a
  small book is small, which also means it's close to net income itself.
- **The answer moves with r.** Two points of cost of equity turn 2013 from
  "priced as a perpetuity" to "priced to fade".
- **Hindsight.** Apple is the example because it won. The test is every
  company priced as Apple was in 2013, including those whose advantage did
  collapse (Nokia, BlackBerry, HP's PCs): the CIK universe of filers without
  tickers today is what puts them back.
- **One company's ω is noisy:** 15 years give ±0.07 here only because
  Apple's residual income grew so steadily; SMCI's and PANW's are ±0.24–0.29.
- **The industry is what's loaded:** 16 of the issuers in group 357, today's
  survivors; with the EDGAR fetch done and loaded it will be every filer in
  the group since 2009.

## To do

- Apple's statements for 1997–2008, from the 10-Ks as filed (text, before
  XBRL): book value, earnings and shares, to see what the price implied
  when Jobs came back.
- The same table for Nokia and BlackBerry, whose advantages didn't last.
- Fama–French industries instead of SIC groups (`VALUATION.md` section 8).
