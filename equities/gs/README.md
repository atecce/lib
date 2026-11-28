# Goldman Sachs

The Goldman Sachs Group (NYSE: GS, CIK 886982): a partnership from 1869
until its initial public offering on May 4, 1999, at $53 a share; a bank
holding company since September 2008.

## From commercial paper

Marcus Goldman, an immigrant from Bavaria, opened in lower Manhattan in
1869 buying merchants' promissory notes at a discount and selling them on to
banks: commercial paper. His son-in-law Samuel Sachs became a partner in
1882, and the firm took its name, Goldman, Sachs & Co., in 1885. The price
of the paper he dealt in is in this repository: the NBER's New York
commercial paper rate, 1857–1971 (FRED, `M13002US35620M156NNBR`; see
[CAPITAL.md](../CAPITAL.md)). It averaged 9.7% in 1869, his first year,
10.3% in the panic year 1873 (when call money reached a monthly average of
61%), and 4.1% in 1885.

Three later episodes bear on the rest of this document:

- **1928–29: the Goldman Sachs Trading Corporation**, an investment trust
  that borrowed on top of borrowing (through the Shenandoah and Blue Ridge
  corporations) and lost nearly everything in the crash (Galbraith, *The
  Great Crash*, 1955). The abuses of the 1920s investment trusts led to the
  Investment Company Act of 1940, under which today's funds file the N-CEN
  and N-PORT reports in `edgar::funds`.
- **1970: Penn Central.** Goldman was the dealer for the railroad's
  commercial paper; when it went bankrupt in June 1970, it defaulted on
  about $82 million of it, and the buyers sued Goldman.
- **2008.** In September Goldman became a bank holding company, and
  Berkshire Hathaway bought $5 billion of its preferred stock paying 10%,
  with warrants; in October the Treasury put in $10 billion more under
  TARP. Those preferred dividends are most of December 2008's $248 million
  (below).

## What the data covers

| | Years | Source |
|---|---|---|
| Book value, EPS, dividends per common share | fiscal 1999–2025 | the 10-Ks' selected financial data (five years at a time: the 2003 annual report, the 2008, 2013, 2018 and 2021 10-Ks), then each 10-K: `gs::ANNUAL`, with each figure's accession |
| The one-month transition period | December 2008 | the 2009 10-K |
| Statements, as Goldman presents them | 2025 | the 2025 10-K: `gs::BALANCE_SHEET_2025`, `INCOME_STATEMENT_2025`, `CASH_FLOW_STATEMENT_2025` |
| Net income, equity, assets (Postgres) | 2009–2025 | XBRL, through the generic reader |
| Prices | May 1999 – | Tiingo, `GS` |

The fiscal year ended on the last Friday of November through 2008, then
became the calendar year. Fiscal 2009 below therefore runs thirteen months,
from November 28, 2008, including December 2008's loss of $2.15 a share.

Before 1999 there is no book value per share: Goldman was a partnership,
and its partners' capital was not divided into shares. Fiscal 1999's
earnings include the IPO's one-time awards to employees ($2.26 billion) and
a tax benefit from the change of form.

## Residual income per share

    cargo run --release -p store --example gs_history            # r = 9%
    cargo run --release -p store --example gs_history -- --premium 0.04

Per common share, so preferred stock is left out: ROE is diluted EPS over
opening book value per common share, and residual income is
`x = EPS − r·B₋₁`. Prices are year-end closes.

| Year | Book | Price | P/B | ROE | ROE − 9% | ω implied |
|---|---|---|---|---|---|---|
| 2000 | 32.18 | 81.50 | 2.53 | 28.7% | +19.7 | 1.01 |
| 2001 | 36.33 | 89.00 | 2.45 | 13.2% | +4.2 | 1.06 |
| 2002 | 38.69 | 78.87 | 2.04 | 11.1% | +2.1 | 1.07 |
| 2003 | 43.60 | 96.08 | 2.20 | 15.2% | +6.2 | 1.04 |
| 2004 | 50.77 | 104.84 | 2.06 | 20.5% | +11.5 | 1.00 |
| 2005 | 57.02 | 134.12 | 2.35 | 22.1% | +13.1 | 1.00 |
| 2006 | 72.62 | 201.60 | 2.78 | 34.5% | +25.5 | 0.98 |
| 2007 | 90.43 | 226.64 | 2.51 | 34.1% | +25.1 | 0.96 |
| 2008 | 98.68 | 78.99 | 0.80 | 4.9% | −4.1 | |
| 2009 (13 months) | 117.48 | 168.84 | 1.44 | 18.7% | +9.7 | 0.91 |
| 2010 | 128.72 | 168.16 | 1.31 | 11.2% | +2.2 | 1.02 |
| 2011 | 130.31 | 90.43 | 0.69 | 3.5% | −5.5 | |
| 2012 | 144.67 | 127.56 | 0.88 | 10.8% | +1.8 | |
| 2013 | 152.48 | 177.26 | 1.16 | 10.7% | +1.7 | 0.99 |
| 2014 | 163.01 | 193.83 | 1.19 | 11.2% | +2.2 | 0.98 |
| 2015 | 171.03 | 180.23 | 1.05 | 7.4% | −1.6 | |
| 2016 | 182.47 | 239.45 | 1.31 | 9.5% | +0.5 | 1.07 |
| 2017 | 181.00 | 254.76 | 1.41 | 4.9% | −4.1 | |
| 2018 | 207.36 | 167.05 | 0.81 | 14.0% | +5.0 | |
| 2019 | 218.52 | 229.93 | 1.05 | 10.1% | +1.1 | 0.90 |
| 2020 | 236.15 | 263.71 | 1.12 | 11.3% | +2.3 | 0.92 |
| 2021 | 284.39 | 382.55 | 1.35 | 25.2% | +16.2 | 0.78 |
| 2022 | 303.55 | 343.38 | 1.13 | 10.6% | +1.6 | 0.98 |
| 2023 | 313.56 | 385.77 | 1.23 | 7.5% | −1.5 | |
| 2024 | 336.77 | 572.62 | 1.70 | 12.9% | +3.9 | 1.04 |
| 2025 | 357.60 | 879.00 | 2.46 | 15.2% | +6.2 | 1.05 |

(2017's ROE carries the one-time charge for the Tax Cuts and Jobs Act;
2009's is annualized. The implied ω is blank where the excess or the
premium to book is negative.)

### By chief executive

| Era | ROE | ROE − 9% | Mean P/B | Stock, a year | ω realized |
|---|---|---|---|---|---|
| 2000–06, Paulson | 20.7% | +11.7 | 2.35 | +15.6% | 1.04 |
| 2007–18, Blankfein | 11.8% | +2.8 | 1.21 | −0.3% | −0.23 |
| 2019–25, Solomon | 13.3% | +4.3 | 1.43 | +29.8% | 0.32 |
| IPO to 2025 | | | | +11.5% (18.0×) | |

With r from the Treasury rate plus 4% (`--premium 0.04`), the excess is
+11.9, +4.9 and +6.5 points, and realized ω 1.12, −0.08 and 0.60: lower
rates since 2008 make the later years look better, but the break in 2008
stands either way.

## What it says

**Before 2008, the market priced Goldman's excess as permanent, and it
wasn't.** From 2000 to 2007 the stock traded at 2–2.8 times book, and the
implied ω was about 1.0 every year: the price assumed the excess return
would last indefinitely, like Apple's. It even grew (realized ω 1.04),
peaking at 34% ROE in 2006–07. Then 2008 ended it, and in the Blankfein
years after the crisis the excess was about zero and didn't persist.

This is the opposite of Berkshire, which the market priced for decay while
its excess persisted (see [../brk/README.md](../brk/README.md)). The
difference is what the excess rested on. Goldman's balance sheet shows it:
total assets over shareholders' equity, from the selected financial data,

| Year end | Assets ($bn) | Equity ($bn) | Assets / equity |
|---|---|---|---|
| Nov 1999 | 248 | 10.1 | 24.5 |
| Nov 2003 | 404 | 21.6 | 18.7 |
| Nov 2007 | 1,120 | 42.8 | 26.2 |
| Nov 2008 | 885 | 64.4 | 13.7 |
| Dec 2013 | 912 | 78.5 | 11.6 |
| Dec 2018 | 932 | 90.2 | 10.3 |
| Dec 2025 | 1,809 | 125.0 | 14.5 |

ROE is return on assets times leverage. The 34% of 2006–07 was earned on
26 times equity, in the years of the credit boom, low short rates and
narrow spreads that [CAPITAL.md](../CAPITAL.md) describes. Leverage was
halved in 2008 (capital raised from Berkshire, the Treasury and the market)
and kept down by the capital rules for bank holding companies, and ROE
came down with it. In Marx's terms the excess was not a waterfall: it could
be competed away, or regulated away. In Hayek's it looks like a boom
financed on cheap credit: the structure of production in finance itself
lengthened, and was cut back when the credit stopped.

**Since 2019 it has come back, and the market is paying for it again.**
ROE rose to 15% in 2025 with leverage at 14.5, and the stock went from 0.81
times book at the end of 2018 to 2.46 at the end of 2025, about 30% a year.
At the end of 2025 the price again implies ω above 1 (1.05): the same
belief as in 2006, on half the leverage.

**Against its industry.** `persistence GS` (net income and total equity,
2011–2025, from Postgres) finds realized ω 0.12 ± 0.32, among the lowest in
the market: 416 of 462 companies are at or above it. The brokers in SIC
621 split: Schwab (1.14), Interactive Brokers (1.13) and Raymond James
(1.11), platforms collecting fees and interest on clients' balances,
persist like Apple; Morgan Stanley (0.80), which moved toward wealth
management after 2008, is in between; Goldman and Nomura (0.27), the
traders, don't. Pooled, SIC 621 is 0.63.

## The statements, in code

`gs::BALANCE_SHEET_2025`, `INCOME_STATEMENT_2025` and
`CASH_FLOW_STATEMENT_2025` hold the 2025 10-K's statements line for line;
the tests check that their totals equal the filing's and that the generic
reader agrees with them where both name an item.

- **The balance sheet** implements `BalanceSheet` but not
  `ClassifiedBalanceSheet`: a bank's statement of financial condition lists
  assets by liquidity with no current and non-current totals. Its
  collateralized agreements (repurchase agreements and securities borrowed,
  $334 billion) are short-term secured lending, what commercial paper became.
- **The income statement** implements `IncomeStatement` but not
  `MultiStepIncomeStatement`: there is no cost of revenue or gross profit.
  Revenue is by business (investment banking, investment management,
  commissions, market making), plus net interest income; compensation is
  32% of net revenues (`compensation_ratio`), the partners' share in a public
  company's accounts.
- **The cash flow statement** implements `CashFlowStatement`. Operating
  cash flow (−$45 billion in 2025, on earnings of $17 billion) is mostly the
  change in the trading book, not a measure of earnings.

## Caveats

- **Diluted EPS, not comprehensive income.** Goldman's book value per share
  moves with other comprehensive income (its own credit spread, for one) and
  with buybacks and share awards, so it is not clean surplus: ROE here is
  EPS over opening book.
- **r = 9%** for a firm whose risk changed with its leverage; the Treasury
  version is above.
- **One firm, and a famous one**: the same survivorship caveat as Apple and
  Berkshire. Bear Stearns and Lehman Brothers, the firms of the same kind
  that failed in 2008, are not in the table.

## To do

- The leverage table from the Postgres facts, for every year.
- Morgan Stanley beside it, the same way; and Lehman Brothers' and Bear
  Stearns's 10-Ks until 2007, for the firms the survivorship caveat leaves
  out.
- The commercial paper market itself since 1971: FRED's commercial paper
  rates and outstanding amounts.
