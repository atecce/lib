# Berkshire Hathaway (BRK)

Notes from 2026-11-23: residual income on Berkshire, as `aapl/README.md` did
for Apple, over the sixty years since Buffett took control in 1965.
Reproduce with

```sh
cargo run --release -p equities --example brk_history            # r = 9%
cargo run --release -p equities --example brk_history -- 0.07
cargo run --release -p backtest --example persistence -- BRK     # GAAP, against insurers
cargo test -p equities --lib brk
```

## What the data covers

| | |
|---|---|
| Book value per share, 1965–2018 | Buffett's scorecard, "Berkshire's Performance vs. the S&P 500", from the 2018 letter (the last with the book value column), compounded from 1964's $19.46 |
| Market value per share, 1965–2018 | the same table's market column |
| Book value, 2019–2025 | the 10-Ks (XBRL, in Postgres): shareholders' equity over Class A equivalent shares |
| Statements | the 2025 10-K, as Berkshire presents them (`mod.rs`) |
| Prices | Class A from Tiingo (fetch pending, see below); Class B in Postgres from 1996 |
| Industry | SIC 6331, fire, marine and casualty insurance |

The scorecard's yearly changes are transcribed into `mod.rs`, and a test
checks they compound to the table's own totals (book value ×10,920,
market value ×24,727, 1964–2018), and that the scorecard's 2018 book value
agrees with the 10-K's to within the share count's difference (0.2%).
1965 and 1966 are years to September 30, 1967 fifteen months; results
through 1978 are restated by Berkshire for the 1979 rule that valued
insurers' stocks at market.

## Why per share, and why book value

Berkshire suits the residual income model unusually well. It has paid one
dividend since 1965 (10 cents a share, in 1967), and until 2018 bought back
almost no stock, so **clean surplus holds per share**: the change in book
value per share *is* comprehensive earnings per share, and

```math
x_t = B_t - (1 + r)\,B_{t-1}, \qquad \mathrm{ROE}_t = \frac{B_t}{B_{t-1}} - 1
```

Comprehensive ROE is the growth of book value per share: the yardstick
Buffett opened every letter with for three decades.

GAAP net income doesn't work for Berkshire. Since 2018 accounting rules
(ASU 2016-01) put unrealized gains and losses on the stock portfolio
through net income, so it swings with the market: residual income on GAAP
earnings was +$50bn in 2019, −$68bn in 2022, +$55bn in 2023
(`persistence`), and its fitted ω is −0.43 ± 0.27. That is the market's
noise, not the businesses'. Book value always carried the stocks at market
(since 1979), so it is unaffected by the rule; it is noisy too, but it was
noisy all along.

## Excess return by era

With $`r = 9\%`$ (per share, Class A):

| Era | Book value per share | Comprehensive ROE | ROE − r | Persistence ω |
|---|---|---|---|---|
| 1965–1979, textile mill to insurer | $19 → $336 | 20.9% | +11.9 points | 0.95 |
| 1980–1998, the great compounding | $336 → $37,803 | 28.2% | +19.2 points | 0.92 |
| 1999–2018, size as an anchor | $37,803 → $212,832 | 9.0% | +0.0 points | −0.16 |
| 2019–2025, operating businesses, buybacks | $212,832 → $499,607 | 13.0% | +4.0 points | −0.25 |

Persistence ω is section 8's: next year's residual income per share on
this year's, both over this year's opening book. Above 0.9 means the excess
carried from year to year (and, with book growing 20–28% a year, grew in
dollars); below zero, that what remains is noise around nothing.

**What it says:**

- **For 34 years Berkshire was the entrepreneurial anomaly of section 8 in
  its purest form**: an excess return of 12–19 points a year over a 9% cost
  of equity, persisting decade after decade (ω above 0.9), from a source no
  competitor copied: Buffett and Munger's capital allocation, funded by
  insurance float that cost less than nothing when underwriting was
  profitable, and taxes deferred on gains not sold.
- **From 1999 the excess vanished.** Comprehensive ROE averaged 9.0%, the
  cost of equity to the tenth of a point, for twenty years. Not competition
  in the usual sense: size. Buffett warned of it in letter after letter: at
  hundreds of billions there are few things large enough to matter, and the float's returns converge on
  the market's. The decay competition predicts arrived through scale
  instead.
- **1999 itself marks the turn**: General Re, bought in 1998 with stock at
  the top of the market (book grew 48% that year, partly from issuing
  shares well above book value), then book value grew 0.5%, 6.5% and
  −6.2% over the next three years.
- **2019–2025 is +4 points**, mostly 2019, 2021, 2023 and 2024's market
  years, with −14.5 points in 2022: the operating businesses (BNSF,
  Berkshire Hathaway Energy, the manufacturers) now hold most of the value
  and are carried at cost, so book value understates them more each year.

Buffett retired book value as the scorecard in the 2018 letter for three
reasons, each visible here: Berkshire had become mostly operating
businesses rather than marketable stocks; those businesses are carried far
below their value; and buybacks above book value but below intrinsic value
raise value per share while lowering book value per share.

## Market value against book (pending the Class A prices)

`brk_history` scales the scorecard's market column to the actual 2018
year-end Class A close and uses Tiingo's closes from 2019, giving each
year's price to book, the implied ω and implied growth, and the market's
return by era. The Class A history (`tiingo-fetch --full BRK-A`) is queued
behind the S&P 500 price batch on the free plan's 50 requests an hour;
this section is filled in when it arrives.

## Against its industry (GAAP, 2011–2025)

`persistence` on Berkshire and the twelve insurers in Postgres in SIC
group 633, residual income from GAAP net income:

| | ω |
|---|---|
| Berkshire | −0.43 ± 0.27 |
| Insurers, pooled (158 company-years) | 0.36 ± 0.09 |
| W. R. Berkley, Progressive, Travelers | 1.06–1.09 |
| AIG, Everest, Cincinnati Financial | −0.21 to 0.35 |
| The whole market | 0.84 ± 0.01 |

Insurance profits persist less than the market's (0.36 against 0.84):
catastrophe years and reserve changes make them lumpy, and the insurers
that hold stocks (Berkshire, Cincinnati, Loews) carry their portfolios'
noise into GAAP earnings since 2018. The consistent underwriters
(Progressive, Travelers, Berkley) persist like the best industrial
companies.

## The statements, in code

`mod.rs` has Berkshire's 2025 statements as it presents them, implementing
the generic traits, and tests that every total equals the 10-K's:

- **Two segments**, each a struct with its own assets, liabilities,
  revenues and costs: *Insurance and Other* (the insurers, the portfolio
  their float buys, and the manufacturing, service and retailing
  businesses) and *Railroad, Utilities and Energy* (BNSF and Berkshire
  Hathaway Energy, capital heavy and financed largely with their own debt).
  The company composes them; income taxes and equity are the whole
  company's.
- **Unclassified and single-step.** Like banks' and insurers', Berkshire's
  balance sheet has no current and non-current split, and its income
  statement no gross profit. So the generic traits were split
  (`src/lib.rs`): `BalanceSheet` (total assets and liabilities) and
  `IncomeStatement` (income before tax, net income) are what every company
  has; `ClassifiedBalanceSheet` and `MultiStepIncomeStatement` add the
  current totals and gross profit for companies that present them (Apple,
  NVIDIA, Tesla); and `CashFlowStatement` is new, for all four.
- **What the structure exposes** (2025, in millions):
  - investment gains of $39,078 in pretax earnings of $82,459: before them,
    the businesses earned $43,381;
  - underwriting profit (premiums less losses and expenses) of $9,460, so
    the float cost less than nothing;
  - insurance liabilities less deferred charges of $203,198 (`float()`, a
    simple definition; Berkshire's own reckoning of float nets other items
    and is lower);
  - $321,434 in Treasury Bills, about a quarter of total assets;
  - income taxes, principally deferred, of $86,955.

**The generic reader misnames some of Berkshire's lines**, found by
comparing: `TotalOperatingExpenses` is the railroad's freight expenses
alone ($15,366), `CostOfRevenue` and `SalesGeneralAndAdministrative` are
the Insurance and Other segment's lines alone, and `NetIncome` includes
noncontrolling interests ($67,260, against $66,968 for Berkshire's
shareholders). Where the reader and the hand-written statements name the
same item, they agree; these four the hand-written statements don't claim.

## Taxes on unrealized gains

Unrealized gains are not taxed until the stock is sold, but GAAP books the
tax as it accrues: net income since 2018 carries a deferred tax charge on
each year's unrealized gains, and the balance sheet a deferred tax
liability ($86,955 million at 2025, mostly on the portfolio). Book value is
net of it, and so is the scorecard (Buffett notes Berkshire's figures are
after tax, the S&P 500's before). The liability bears no interest and
falls due only on sale: an interest-free loan from the Treasury, in
Buffett's phrase, and part of the excess return above, since gains compound
undiminished while held. When Berkshire sold much of its Apple stake in
2023–2024, the deferred tax became cash.

## Caveats

- **r = 9% for sixty years** is crude: Treasury yields ran from 4% to 15%
  and back. A cost of equity from the Treasury rate plus a premium would
  raise the 1980s' cost and lower the 2010s'.
- **Book value understates Berkshire more each year**: operating businesses
  at cost, and buybacks above book since 2018.
- **The scorecard is Berkshire's own**, restated through 1978; the 10-K
  years use the cover's share count rather than the year end's.
- **One company, chosen because it is famous**: the same survivorship
  caveat as Apple.

## To do

- The market side, when the Class A prices arrive (above).
- The Treasury rate as r, year by year.
- Buffett's operating earnings (after tax, before investment gains) from
  the 10-Ks since 2018, for a residual income free of the portfolio's
  marks.
- The statements for more years than 2025.
