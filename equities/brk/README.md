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
| Prices | Class A from Tiingo, 1980–2025; before 1980 the scorecard's market column; Class B in Postgres from 1996 |
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

## Market value against book

Prices per Class A share at each year end: Tiingo's closes from 1980, and
before them the scorecard's market column scaled to the 2018 close (it
meets Tiingo's 1979 close: $321). Implied $`\omega = R(P - B)/(x + P - B)`$
where residual income is positive and the price above book.

| Era | P/B (mean) | Market return a year | Book growth a year | Realized ω | Implied ω (typical) |
|---|---|---|---|---|---|
| 1965–1979 | 0.81 | +24.2% | 20.9% | 0.95 | price below book: none |
| 1980–1998 | 1.54 | +32.8% | 28.2% | 0.92 | 0.2–0.9 |
| 1999–2018 | 1.48 | +7.7% | 9.0% | −0.16 | 0.85–1.08 |
| 2019–2025 | 1.38 | +13.8% | 13.0% | −0.25 | 0.8–1.06 |

**What the price said, against what came:**

- **1965–1979: the market priced Berkshire below its book value** (P/B
  0.43–0.97, 0.81 on average) while book value per share compounded at 21%
  a year. A price below book says residual income will be negative: that
  the business will earn less than its cost of capital. It was a failing
  New England textile mill, after all, and in 1974–75, with ROE of 5% and
  the market at its worst since the Depression, the price fell to 0.43–0.51
  of book. Persistence was 0.95. This is the anomaly of section 8 at its
  starkest, and the market return (24% a year) shows what it paid.
- **1980–1998: the price rose above book but kept assuming decay.** Implied
  ω ran 0.2–0.9: in 1980–82 the price allowed the excess to fade within a
  few years (ω 0.20–0.45) while ROE ran 19–40%. Realized, ω was 0.92 with
  book growing 28% a year. The market return (33%) beat book growth (28%):
  the price-to-book rerating from about 1 to about 2 was the market slowly
  catching up to a persistence it had underpriced for thirty years.
- **1999–2018: the price got it right.** P/B averaged 1.5 and implied ω
  sat near 1 (0.85–1.08): a modest excess, lasting. What came was an
  excess of zero (ROE 9.0% against r of 9%), and the market returned 7.7%
  a year, a little under the cost of equity, as a price slightly above
  what was earned implies. The anomaly was gone, and the price no longer
  paid for one.
- **2019–2025: about the same**, P/B 1.2–1.5, with buybacks below
  intrinsic value (Buffett's test) and above book.

Over the whole sixty years the market return beat book growth (the
scorecard's 2,472,627% against 1,091,899% to 2018) because the price
started at 0.64 of book in 1964 and ended near 1.4: the market's estimate
of Berkshire's persistence rose from "none" to "modest", and most of the
excess persistence came before it did.

<details>
<summary>Every year, 1965–2025 (r = 9%, per Class A share)</summary>

| Year | Book value | Price | P/B | ROE | ROE − r | Residual income | ω implied | Implied growth |
|---|---|---|---|---|---|---|---|---|
| 1965 | $24 | $19 | 0.77 | 23.8% | +14.8% | $3 |  |  |
| 1966 | $29 | $18 | 0.62 | 20.3% | +11.3% | $3 |  |  |
| 1967 | $32 | $20 | 0.63 | 11.0% | +2.0% | $1 |  |  |
| 1968 | $38 | $36 | 0.94 | 19.0% | +10.0% | $3 |  |  |
| 1969 | $44 | $43 | 0.97 | 16.2% | +7.2% | $3 |  |  |
| 1970 | $50 | $41 | 0.83 | 12.0% | +3.0% | $1 |  |  |
| 1971 | $58 | $74 | 1.28 | 16.4% | +7.4% | $4 | 0.89 | −11.2% |
| 1972 | $71 | $80 | 1.14 | 21.7% | +12.7% | $7 | 0.62 | −38.2% |
| 1973 | $74 | $78 | 1.06 | 4.7% | −4.3% | −$3 |  |  |
| 1974 | $78 | $40 | 0.51 | 5.5% | −3.5% | −$3 |  |  |
| 1975 | $95 | $41 | 0.43 | 21.9% | +12.9% | $10 |  |  |
| 1976 | $151 | $94 | 0.62 | 59.3% | +50.3% | $48 |  |  |
| 1977 | $200 | $138 | 0.69 | 31.9% | +22.9% | $35 |  |  |
| 1978 | $248 | $159 | 0.64 | 24.0% | +15.0% | $30 |  |  |
| 1979 | $336 | $321 | 0.96 | 35.7% | +26.7% | $66 |  |  |
| 1980 | $401 | $425 | 1.06 | 19.3% | +10.3% | $35 | 0.45 | −55.2% |
| 1981 | $527 | $560 | 1.06 | 31.4% | +22.4% | $90 | 0.29 | −70.5% |
| 1982 | $737 | $775 | 1.05 | 40.0% | +31.0% | $163 | 0.20 | −79.6% |
| 1983 | $976 | $1,310 | 1.34 | 32.3% | +23.3% | $172 | 0.72 | −28.0% |
| 1984 | $1,108 | $1,275 | 1.15 | 13.6% | +4.6% | $45 | 0.86 | −14.1% |
| 1985 | $1,642 | $2,470 | 1.50 | 48.2% | +39.2% | $434 | 0.71 | −28.5% |
| 1986 | $2,071 | $2,820 | 1.36 | 26.1% | +17.1% | $281 | 0.79 | −20.7% |
| 1987 | $2,475 | $2,950 | 1.19 | 19.5% | +10.5% | $217 | 0.75 | −25.2% |
| 1988 | $2,972 | $4,700 | 1.58 | 20.1% | +11.1% | $275 | 0.94 | −6.0% |
| 1989 | $4,292 | $8,675 | 2.02 | 44.4% | +35.4% | $1,052 | 0.88 | −12.1% |
| 1990 | $4,610 | $6,675 | 1.45 | 7.4% | −1.6% | −$69 |  |  |
| 1991 | $6,435 | $9,050 | 1.41 | 39.6% | +30.6% | $1,411 | 0.71 | −29.2% |
| 1992 | $7,742 | $11,750 | 1.52 | 20.3% | +11.3% | $727 | 0.92 | −7.7% |
| 1993 | $8,849 | $16,325 | 1.84 | 14.3% | +5.3% | $410 | 1.03 | +3.3% |
| 1994 | $10,079 | $20,400 | 2.02 | 13.9% | +4.9% | $434 | 1.05 | +4.6% |
| 1995 | $14,422 | $32,100 | 2.23 | 43.1% | +34.1% | $3,437 | 0.91 | −8.7% |
| 1996 | $19,009 | $34,100 | 1.79 | 31.8% | +22.8% | $3,288 | 0.89 | −10.5% |
| 1997 | $25,491 | $46,000 | 1.80 | 34.1% | +25.1% | $4,771 | 0.88 | −11.6% |
| 1998 | $37,803 | $70,000 | 1.85 | 48.3% | +39.3% | $10,018 | 0.83 | −16.9% |
| 1999 | $37,992 | $56,100 | 1.48 | 0.5% | −8.5% | −$3,213 |  |  |
| 2000 | $40,461 | $71,000 | 1.75 | 6.5% | −2.5% | −$950 |  |  |
| 2001 | $37,953 | $75,600 | 1.99 | -6.2% | −15.2% | −$6,150 |  |  |
| 2002 | $41,748 | $72,750 | 1.74 | 10.0% | +1.0% | $380 | 1.08 | +7.7% |
| 2003 | $50,515 | $84,250 | 1.67 | 21.0% | +12.0% | $5,010 | 0.95 | −5.1% |
| 2004 | $55,819 | $87,900 | 1.57 | 10.5% | +1.5% | $758 | 1.06 | +6.5% |
| 2005 | $59,392 | $88,620 | 1.49 | 6.4% | −2.6% | −$1,451 |  |  |
| 2006 | $70,320 | $109,990 | 1.56 | 18.4% | +9.4% | $5,583 | 0.96 | −4.4% |
| 2007 | $78,055 | $141,600 | 1.81 | 11.0% | +2.0% | $1,406 | 1.07 | +6.6% |
| 2008 | $70,562 | $96,600 | 1.37 | -9.6% | −18.6% | −$14,518 |  |  |
| 2009 | $84,533 | $99,200 | 1.17 | 19.8% | +10.8% | $7,621 | 0.72 | −28.3% |
| 2010 | $95,522 | $120,450 | 1.26 | 13.0% | +4.0% | $3,381 | 0.96 | −4.0% |
| 2011 | $99,916 | $114,755 | 1.15 | 4.6% | −4.4% | −$4,203 |  |  |
| 2012 | $114,304 | $134,060 | 1.17 | 14.4% | +5.4% | $5,395 | 0.86 | −14.4% |
| 2013 | $135,107 | $177,900 | 1.32 | 18.2% | +9.2% | $10,516 | 0.87 | −12.5% |
| 2014 | $146,321 | $226,000 | 1.54 | 8.3% | −0.7% | −$946 |  |  |
| 2015 | $155,686 | $197,800 | 1.27 | 6.4% | −2.6% | −$3,804 |  |  |
| 2016 | $172,344 | $244,121 | 1.42 | 10.7% | +1.7% | $2,647 | 1.05 | +5.1% |
| 2017 | $211,984 | $297,600 | 1.40 | 23.0% | +14.0% | $24,128 | 0.85 | −15.0% |
| 2018 | $212,832 | $306,000 | 1.44 | 0.4% | −8.6% | −$18,231 |  |  |
| 2019 | $261,919 | $339,590 | 1.30 | 23.1% | +14.1% | $29,932 | 0.79 | −21.3% |
| 2020 | $289,824 | $347,815 | 1.20 | 10.7% | +1.7% | $4,332 | 1.01 | +1.4% |
| 2021 | $343,459 | $450,662 | 1.31 | 18.5% | +9.5% | $27,552 | 0.87 | −13.3% |
| 2022 | $324,435 | $468,711 | 1.44 | -5.5% | −14.5% | −$49,936 |  |  |
| 2023 | $390,253 | $542,625 | 1.39 | 20.3% | +11.3% | $36,619 | 0.88 | −12.1% |
| 2024 | $452,217 | $680,920 | 1.51 | 15.9% | +6.9% | $26,841 | 0.98 | −2.4% |
| 2025 | $499,607 | $754,800 | 1.51 | 10.5% | +1.5% | $6,691 | 1.06 | +6.2% |

</details>

## With the Treasury rate as r

A fixed r = 9% charges 1981 and 2021 the same price for capital. Instead,
`equities::treasury` reads the 10-year Treasury yield for every December
since 1953 from the FRED cache (series GS10, the Federal Reserve's H.15;
`fred-fetch GS10`), and
`--premium P` sets each year's r to the previous December's yield plus P:
the rate known when the year began, plus an equity risk premium.

    cargo run --release -p equities --example brk_history -- --premium 0.04

With a premium of 4%, r runs from 8.2% in 1965 to 17.7% in 1982 and down to
4.9% in 2021:

| Era | r | ROE | ROE−r (was, at 9%) | P/B | Market | ω realized (was) |
|---|---|---|---|---|---|---|
| 1965–79 | 10.5% | 20.9% | +10.4% (+11.9) | 0.81 | +24.2% | 0.87 (0.95) |
| 1980–98 | 12.7% | 28.2% | +15.5% (+19.2) | 1.54 | +32.8% | 0.82 (0.92) |
| 1999–2018 | 7.6% | 9.0% | +1.4% (0.0) | 1.48 | +7.7% | 0.01 (−0.16) |
| 2019–25 | 6.7% | 13.0% | +6.2% (+4.0) | 1.38 | +13.8% | 0.00 (−0.25) |

(r by era is compounded, like the book value it is compared with.)

- **The great compounding shrinks but survives.** In 1980–82 r was 14–18%,
  and the excess in those years falls by about half. Over the era it is
  still +15.5% a year, and the price still implied quick decay: implied ω
  of 0.29–0.68 in 1980–82, against a realized 0.82.
- **The years since 1999 look a little better.** At 9% the excess was nil;
  at 2–6% Treasuries it is +1.4% a year, and +6.2% since 2019. The market
  returned 7.7% a year in 1999–2018 against an r of 7.6%: what a fairly
  priced stock with no lasting excess should return.
- **Persistence still breaks in 1999.** Realized ω falls from above 0.8 to
  about zero with either r: the excess that remains comes and goes with
  the portfolio's marks.

The premium matters less than the path. At 3.2% (the premium that makes the
average r over 1965–2024 equal 9%) the eras' excess returns are +11.2%,
+16.3%, +2.2% and +7.0%; at 5% they are +9.4%, +14.5%, +0.4% and +5.2%. The
other tables in this document keep r = 9%.

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

- **r = 9% for sixty years** is crude: Treasury yields ran from 1% to 14%
  and back. [With the Treasury rate as r](#with-the-treasury-rate-as-r)
  redoes the eras; the premium on top of it is still a guess.
- **Book value understates Berkshire more each year**: operating businesses
  at cost, and buybacks above book since 2018.
- **The scorecard is Berkshire's own**, restated through 1978; the 10-K
  years use the cover's share count rather than the year end's.
- **One company, chosen because it is famous**: the same survivorship
  caveat as Apple.

## To do

- Buffett's operating earnings (after tax, before investment gains) from
  the 10-Ks since 2018, for a residual income free of the portfolio's
  marks.
- The statements for more years than 2025.
