# Apple (AAPL)

Notes from 2026-11-23: the tests of `VALUATION.md` section 8 (entrepreneurial
anomalies, persistent residual income) run on Apple, and what they say about
its rise and Steve Jobs's stewardship. Reproduce with

```sh
cargo run --release -p backtest --example persistence -- AAPL            # r = 9%
cargo run --release -p backtest --example persistence -- AAPL --r 0.07
```

## What the data covers

| | |
|---|---|
| Statements (XBRL, EDGAR) | fiscal 2009–2026 (years end in late September), as first reported |
| Statements (10-Ks before XBRL, EDGAR) | fiscal 1991–2010: net income, equity, shares, as first reported |
| Prices (Tiingo) | 1980 (the IPO)–2026, daily |
| Industry | SIC 3571, electronic computers; group 357, computer and office equipment |

Jobs returned to Apple in 1997 (interim CEO that September), resigned as CEO
on August 24, 2011, and died on October 5, 2011. XBRL statements begin in
2009; the years before come from the 10-Ks as filed, back to fiscal 1991
(below). The 1980s, filed on paper, are in the prices only.

## Residual income and what the price implied

With $`r = 9\%`$, residual income $`x^a_t = E_t - r\,B_{t-1}`$, market value
$`M`$ the fiscal year end's close times the shares on the 10-K's cover, and
the implied decay $`\omega = R(M - B)/(x^a + M - B)`$ (section 8), where
residual income is positive and the price above book.

Fiscal 1991–2010 come from the 10-Ks themselves, filed before XBRL: their
five-year tables, balance sheets and statements of shareholders' equity, as
first reported (`BEFORE_XBRL` in `mod.rs`, each year with its filing;
`cargo run --release -p equities --example aapl_history`). EDGAR begins
with the fiscal 1994 10-K, whose statement of equity reaches back to
September 1991; the 1980s were filed on paper. 2011 on is from XBRL
(`backtest`'s `persistence` example). Fiscal years end in late September,
so "1997" is the year to September 1997, Jobs interim CEO for its last ten
days.

| Fiscal year | CEO | ROE | Operating income / assets | Residual income | Market value | M/B | ω implied | Implied growth |
|---|---|---|---|---|---|---|---|---|
| 1991 | Sculley |  |  |  | $5.8bn | 3.3 |  |  |
| 1992 | Sculley | 30% |  | $0.37bn | $5.4bn | 2.5 | 0.98 | −2.3% |
| 1993 | Spindler | 4% |  | −$0.11bn | $2.9bn | 1.4 |  |  |
| 1994 | Spindler | 15% |  | $0.13bn | $4.0bn | 1.7 | 1.01 | +1.2% |
| 1995 | Spindler | 18% |  | $0.21bn | $4.6bn | 1.6 | 0.97 | −3.1% |
| 1996 | Amelio | −28% |  | −$1.08bn | $2.8bn | 1.4 |  |  |
| 1997 | Jobs (interim) | −51% |  | −$1.23bn | $2.7bn | 2.3 |  |  |
| 1998 | Jobs | 26% |  | $0.20bn | $5.2bn | 3.2 | 1.03 | +3.2% |
| 1999 | Jobs | 37% |  | $0.45bn | $10.5bn | 3.4 | 1.03 | +2.7% |
| 2000 | Jobs | 25% |  | $0.51bn | $8.6bn | 2.1 | 0.98 | −1.9% |
| 2001 | Jobs | −1% |  | −$0.39bn | $5.5bn | 1.4 |  |  |
| 2002 | Jobs | 2% |  | −$0.29bn | $5.3bn | 1.3 |  |  |
| 2003 | Jobs | 2% |  | −$0.30bn | $7.6bn | 1.8 |  |  |
| 2004 | Jobs | 7% |  | −$0.10bn | $15.0bn | 3.0 |  |  |
| 2005 | Jobs | 26% |  | $0.88bn | $44.8bn | 6.0 | 1.06 | +6.5% |
| 2006 | Jobs | 27% |  | $1.3bn | $66.1bn | 6.6 | 1.06 | +6.5% |
| 2007 | Jobs | 35% |  | $2.6bn | $134bn | 9.2 | 1.07 | +6.7% |
| 2008 | Jobs | 33% |  | $3.5bn | $114bn | 5.4 | 1.05 | +5.0% |
| 2009 | Jobs | 27% |  | $3.8bn | $164bn | 5.9 | 1.06 | +6.0% |
| 2010 | Jobs | 44% |  | $11bn | $268bn | 5.6 | 1.04 | +3.7% |
| 2011 | Cook | 54% | 45% | $22bn | $376bn | 4.9 | 1.02 | +1.7% |
| 2012 | Cook | 54% | 47% | $35bn | $628bn | 5.3 | 1.02 | +2.0% |
| 2013 | Cook | 31% | 28% | $26bn | $434bn | 3.5 | 1.00 | +0.5% |
| 2014 | Cook | 32% | 25% | $28bn | $591bn | 5.3 | 1.03 | +2.9% |
| 2015 | Cook | 48% | 31% | $43bn | $640bn | 5.4 | 1.01 | +0.6% |
| 2016 | Cook | 38% | 21% | $35bn | $601bn | 4.7 | 1.01 | +1.5% |
| 2017 | Cook | 38% | 19% | $37bn | $791bn | 5.9 | 1.03 | +3.2% |
| 2018 | Cook | 44% | 19% | $48bn | $1,071bn | 10.0 | 1.04 | +3.9% |
| 2019 | Cook | 52% | 17% | $46bn | $972bn | 10.7 | 1.04 | +3.6% |
| 2020 | Cook | 63% | 20% | $49bn | $1,909bn | 29.2 | 1.06 | +6.2% |
| 2021 | Cook | 145% | 34% | $89bn | $2,410bn | 38.2 | 1.05 | +5.0% |
| 2022 | Cook | 158% | 34% | $94bn | $2,393bn | 47.2 | 1.05 | +4.8% |
| 2023 | Cook | 191% | 32% | $92bn | $2,663bn | 42.8 | 1.05 | +5.3% |
| 2024 | Cook | 151% | 35% | $88bn | $3,443bn | 60.5 | 1.06 | +6.2% |
| 2025 | Cook | 197% | 36% | $107bn | $3,775bn | 51.2 | 1.06 | +5.9% |
| 2026 | Cook | 179% | 44% | $126bn | $4,956bn | 44.2 | 1.06 | +6.2% |

**Realized:** residual income grew **12.4% a year** from 2011 to 2026, and
Apple's own $`\omega`$ fits at **1.12 ± 0.07** over 15 years. From 2005,
the first year the iPod's profits show, it grew from $0.88bn to $126bn:
**27% a year for 21 years**, against the 6.5% a year the 2005 price implied.

**What the longer table adds:**

- **The 1990s priced Apple as an ordinary company,** and it was one. Its
  implied growth sat near zero (−3% to +3%), M/B between 1.4 and 3.4, and
  ROE swung from 30% to −51%. No moat was priced, and none persisted:
  residual income turned negative within four years of each good year.
- **In the loss years the price stayed above book**, which section 8's
  formula can't read ($`\omega`$ needs positive residual income). In
  Ohlson's terms the value was in the other information $`\nu`$, not in
  current earnings: in 1997, with a loss of $1bn and book of $1.2bn, the
  market still paid $2.7bn, a bet on recovery, and on Jobs, before any
  of it showed in the statements. In July 1997, at the low ($13.75 a share,
  about 128 million shares), it paid $1.8bn: a little above book value, which fell from $2.1bn
  to $1.2bn over that fiscal year.
- **2001–2004 earned less than the cost of equity**: ROE of −1% to 7%
  while the iPod (October 2001) and the iTunes Store (April 2003) were
  being built. The price, M/B rising from 1.3 to 3.0, anticipated what
  residual income showed only from 2005.
- **From 2005 the price expected durable growth** (+5% to +7% a year
  implied), more than at any time in the 1990s and about what it expects
  now. It was still far too little: 27% a year came.

The 2006 10-K restated 2002–2005 earnings (options backdating: 2005 from
$1,335M to $1,328M), and the 2010 10-K restated 2008 and 2009 for the
iPhone's revenue recognition (2009 net income from $5,704M to $8,235M);
each year above is as its own 10-K reported it, and each ROE divides by the
prior year's equity as that same 10-K shows it.

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

## The wrong baseline

"Computer and office equipment" is a poor yardstick for Apple, and the
reason is the entrepreneurship itself. An industry code sorts companies by
what they already make; Apple was given 3571 as a computer maker, and kept
it while the iPhone became about half its revenue and services a quarter.
The rivals that mattered after 2007 sat elsewhere: Nokia and BlackBerry in
radio telephones (3663), Sony in consumer electronics (3651), Google in
services (7370), and, with the Watch and the pricing, arguably luxury goods.

Schumpeter's "new combinations" include a new good, a new quality of a good
and a new market: doing that redraws who competes with whom. Kirzner's
entrepreneur is alert to what others haven't noticed, here what people
would value and couldn't ask for. Either way competition is a discovery
procedure (Hayek), and the industry is among the things discovered, so a
fixed category can't be the null of how fast an advantage should fade.

**What the achievement was shows in gross margin**, the price over the cost
of what was sold. Apple's design, integration and brand, more an aesthetic
achievement than a technical one, let it sell hardware at about twice the
margin of the box makers it's classified with, from largely the same
suppliers:

| Gross margin | 2011 | 2016 | 2021 | 2025 | 2026 |
|---|---|---|---|---|---|
| Apple | 40% | 39% | 42% | 47% | 49% |
| Dell | | 16% | 31% | 22% | 20% |
| HP | | | | 21% | 21% |

Services lift the later years; the gap was there in 2011. Its intangible
source is expensed as incurred and never reaches book value, so residual
income is high partly *because* the asset earning it isn't recorded: the
excess over $`r`$ is the return on what the balance sheet leaves out.
Residual income doesn't care whether an advantage is technical or
aesthetic; it measures the rent, not its source. The measure holds; the
baseline doesn't.

**The market as the baseline**, as Fama and French (2000) used all firms:
pooled over every issuer in Postgres (shell years and data errors left out,
see `backtest/examples/persistence.rs`):

| | Pooled ω | Median company | At or above Apple's 1.12 |
|---|---|---|---|
| Every issuer (414 with 8+ years) | 0.84 ± 0.01 | 0.79 | 30 |
| Without finance (326) | 0.85 ± 0.01 | 0.80 | 23 |

So across the market residual income fades about 16% a year (at
$`r = 7\%`$, 13%); Apple's grew, a top-7% persistence. The market here is
mostly today's S&P 500, survivors, so the true fade is faster still.

Better peer groups, for later: **text-based industries** (Hoberg and
Phillips 2016), built each year from the product descriptions in the
companies' own 10-Ks, so that a computer maker that starts selling phones
gets phone makers as peers; and **return-based peers**, companies whose
stock returns move together, the market's own classification.

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

## The whole price history

From the IPO on December 12, 1980, adjusted closes (splits, and dividends
reinvested) from Tiingo, against SPY (the S&P 500 fund, from 1993):

```sh
cargo run --release -p backtest --example eras -- AAPL SPY 1980-12-12="IPO" 1985-09-17="Sculley" ...
```

| From | To | Era | Multiple | A year | Worst drawdown | SPY a year |
|---|---|---|---|---|---|---|
| 1980-12-12 | 1985-09-17 | IPO; Jobs chairman, until he resigns | 0.5× | −12% | −77% | |
| 1985-09-17 | 1993-06-18 | Sculley | 5.7× | +25% | −57% | |
| 1993-06-18 | 1996-02-02 | Spindler | 0.7× | −11% | −46% | +17% |
| 1996-02-02 | 1997-07-09 | Amelio | 0.5× | −41% | −56% | +31% |
| 1997-07-09 | 2011-08-24 | Jobs returns (interim CEO from September) | 110× | +39% | −82% | +4% |
| 2011-08-24 | 2026-11-23 | Cook | 31× | +25% | −44% | +15% |
| 1980-12-12 | 2026-11-23 | **the whole history** | **3,551×** | **+19%** | −82% | |

Jobs's second era, in three parts:

| From | To | | Multiple | A year | Worst drawdown | SPY a year |
|---|---|---|---|---|---|---|
| 1997-07-09 | 2000-03-22 | the iMac, to the dot-com peak | 10.5× | +139% | −56% | +22% |
| 2000-03-22 | 2003-04-17 | the bust | 0.2× | −43% | −82% | −14% |
| 2003-04-17 | 2011-08-24 | iPod, iTunes Store, iPhone, App Store | 57× | +62% | −61% | +5% |

What the prices say, before any statements:

- **Jobs's first era lost money.** From the IPO to his resignation in 1985
  the stock halved, with a 77% fall inside it: the Lisa and the early
  Macintosh sold poorly, and the board sided with Sculley. The aesthetic
  achievement was there (the Macintosh, 1984) before the business was.
- **Sculley's years were good, then not.** The Macintosh found its market
  in desktop publishing; the gains came early, and the 57% drawdown came
  at the end.
- **Under Spindler and Amelio the market priced Apple as dying**: −11% and
  −41% a year while the S&P 500 gained 17% and 31%. The July 1997 low is
  where the market's expectation of Apple's residual income was lowest,
  arguably below zero.
- **Jobs's return paid 110×** in fourteen years, against an S&P 500 that
  barely moved (+4% a year across two crashes), with an 82% drawdown in the
  middle: an investor needed to hold through losing four fifths of the
  stake. The iPod-to-App-Store stretch alone was 57×.
- **Cook's fifteen years paid 31×**, 25% a year against the market's 15%,
  with the smallest drawdown of any era: the advantage made durable, which
  is what the residual income above shows.

Whole history: one dollar at the IPO is $3,551 now, 19% a year for 46
years.

What the July 1997 price implied about persistence, in residual income
terms, needs that year's book value and earnings, from 10-Ks filed before
XBRL (below).

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

- Apple's 1980s statements (annual reports on paper, before EDGAR).
- The same table for Nokia and BlackBerry, whose advantages didn't last.
- Hoberg–Phillips text-based peers (their data is published free), and
  Fama–French industries for comparison with the literature.
- Southwest's (LUV) 2010–2011 values are stored unscaled (equity 6,237
  where the 10-K means $6,237 million): an EDGAR reader bug with older
  filings' scale, found by this market-wide fit.

## In code

`mod.rs` here is Apple's statements as it presents them (the module
`equities::aapl`), like `nvda/` and `tsla/`: its own lines, such as vendor
non-trade receivables, with the totals computed from their parts through the
generic `BalanceSheet` and `IncomeStatement` traits. Its tests check the
computed totals against the 10-Q's (fiscal Q2 2026), and that the generic
reader reads all 34 items of the same filing identically.
