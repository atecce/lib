# MINT

The PIMCO Enhanced Short Maturity Active Exchange-Traded Fund: an actively
managed ultra-short bond fund, listed in November 2009 at $100 a share.
Series S000026751 of PIMCO ETF Trust (CIK 1450011), share class
C000080241. Net assets $16.4 billion at the end of June 2026. (Ben
Bernanke, chairman of the Federal Reserve 2006–2014, has been a senior
adviser to PIMCO since 2015; he does not manage the fund.)

It is here as the portfolio's **r for cash**: what money waiting to go into
a stock earns meanwhile, and so the hurdle a stock has to clear. In the
layers of [CAPITAL.md](../CAPITAL.md) it is the short rate plus a thin
credit spread, without the term premium or the equity premium.

## Not the company traits

A fund has financial statements, but a fund's: a statement of assets and
liabilities (investments at fair value, net assets per share), a statement
of operations (investment income, expenses, realized and unrealized gains)
and a statement of changes in net assets (shareholders' purchases and
redemptions). There is no revenue, gross profit or operating cash flow, so
`BalanceSheet`, `IncomeStatement` and `CashFlowStatement` don't fit, and
there is no `mod.rs` here. Those statements are in its annual reports
(N-CSR) and could get fund traits of their own. What the SEC's data sets
give instead, for every fund, is the portfolio and the returns: N-PORT,
read by `edgar::funds`.

## What it holds

    cargo run --release -p store --example fund_holdings -- MINT

From N-PORT for June 30, 2026 (0001410368-26-090134): 1,019 holdings from
342 issuers, as percentages of net assets.

| Asset category | % | | Issuer type | % |
|---|---|---|---|---|
| Debt (bonds and notes) | 67.7 | | Corporate | 77.1 |
| Mortgage-backed | 14.6 | | U.S. sponsored enterprises (Fannie, Freddie) | 11.2 |
| Other asset-backed (cards, autos) | 12.7 | | U.S. Treasury | 6.2 |
| Short-term investment vehicles | 6.4 | | U.S. government agencies | 2.5 |
| CLOs | 0.3 | | Foreign sovereigns | 1.4 |
| Repurchase agreements (borrowed) | −2.1 | | | |

- **61% fixed rate, 40% floating.** The floating-rate notes reset their
  coupons every month or quarter, so they carry almost no interest-rate
  risk whatever their final maturity. The fixed-rate debt is short: 30% of
  net assets matures within a year, 49% within two, and its weighted
  maturity is 1.9 years.
- **Mostly banks and finance companies.** After Freddie Mac (7.5%), the
  Treasury (6.2%), Fannie Mae and Ginnie Mae, the largest issuers are
  JPMorgan, Wells Fargo, HSBC and Goldman Sachs (1.4%, see
  [../gs/README.md](../gs/README.md)), then HCA, Deutsche Bank, CIBC, and the
  finance arms of carmakers: GM, Honda, Hyundai, BMW. It is lending to the
  same kind of borrower Marcus Goldman's commercial paper did.
- **71% American**, then Canada (7.5%), Britain (5.3%), the Netherlands,
  Spain, Japan.
- **Weighted coupon 4.22%**, against 3.87% on 3-month bills and 3.98% on
  1-year Treasuries that day: the credit spread it is paid for.

## How the portfolio has changed

    cargo run --release -p store --example fund_history -- MINT

Every N-PORT report since September 2019, as percentages of net assets
(fixed and floating can add to more than 100 where the fund borrows
through repurchase agreements). *Fixed < 1y* is the fixed-rate debt
maturing within a year; *years* is the fixed-rate debt's weighted final
maturity; *govt* is the Treasury, agencies and sponsored enterprises;
*securitized* is mortgage- and asset-backed debt and CLOs; *coupon* is the
weighted coupon of all the debt, against the 3-month Treasury yield on
the report date.

| Report | Net assets ($bn) | Fixed | Floating | Fixed < 1y | Years | Corporate | Govt | Securitized | Coupon | 3-month | Holdings |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2019-09 | 12.79 | 26.8 | 70.0 | 22.0 | 1.20 | 88.2 | 7.0 | 22.2 | 2.82 | 1.88 | 806 |
| 2019-12 | 13.71 | 24.8 | 69.8 | 11.4 | 2.75 | 82.7 | 15.0 | 21.7 | 2.52 | 1.55 | 802 |
| 2020-03 | 11.95 | 18.4 | 77.4 | 8.6 | 4.19 | 87.7 | 11.6 | 27.4 | 2.17 | 0.11 | 786 |
| 2020-06 | 13.88 | 15.3 | 69.2 | 6.0 | 6.16 | 78.2 | 17.8 | 26.3 | 1.26 | 0.16 | 796 |
| 2020-12 | 14.32 | 23.2 | 69.0 | 6.3 | 5.48 | 75.3 | 18.8 | 25.3 | 1.14 | 0.09 | 788 |
| 2021-06 | 14.02 | 21.6 | 74.0 | 5.1 | 5.12 | 81.4 | 14.4 | 27.2 | 1.13 | 0.05 | 745 |
| 2021-09 | 14.40 | 25.2 | 68.2 | 5.8 | 4.67 | 75.6 | 17.9 | 26.4 | 1.15 | 0.04 | 715 |
| 2021-12 | 13.68 | 29.5 | 65.7 | 8.7 | 4.67 | 80.7 | 15.8 | 26.8 | 1.29 | 0.06 | 725 |
| 2022-03 | 13.20 | 47.2 | 64.8 | 11.0 | 3.60 | 85.8 | 26.8 | 29.6 | 1.60 | 0.52 | 758 |
| 2022-06 | 11.78 | 31.1 | 64.0 | 10.8 | 4.16 | 82.3 | 12.5 | 29.9 | 2.20 | 1.72 | 675 |
| 2022-09 | 11.06 | 23.2 | 61.6 | 11.5 | 4.70 | 78.5 | 9.8 | 30.2 | 2.97 | 3.33 | 643 |
| 2022-12 | 8.79 | 20.4 | 64.8 | 10.6 | 4.87 | 87.2 | 10.8 | 33.6 | 3.71 | 4.42 | 592 |
| 2023-03 | 8.42 | 23.4 | 60.5 | 14.0 | 4.51 | 88.3 | 4.2 | 36.9 | 4.18 | 4.85 | 636 |
| 2023-06 | 9.26 | 23.8 | 51.2 | 15.5 | 3.40 | 88.1 | 7.1 | 30.2 | 3.92 | 5.43 | 668 |
| 2023-12 | 10.30 | 30.5 | 57.0 | 20.4 | 2.53 | 93.3 | 3.4 | 37.9 | 4.53 | 5.40 | 691 |
| 2024-06 | 12.04 | 38.2 | 52.8 | 26.0 | 1.94 | 94.9 | 4.8 | 32.3 | 4.44 | 5.48 | 827 |
| 2024-12 | 11.89 | 33.8 | 55.6 | 24.6 | 1.81 | 89.6 | 7.7 | 29.8 | 4.18 | 4.37 | 833 |
| 2025-06 | 13.33 | 25.2 | 60.6 | 14.5 | 2.43 | 80.9 | 17.4 | 37.4 | 4.06 | 4.41 | 854 |
| 2025-12 | 14.57 | 34.5 | 59.8 | 22.0 | 2.06 | 79.8 | 17.9 | 31.7 | 3.95 | 3.67 | 922 |
| 2026-03 | 15.98 | 33.7 | 55.5 | 20.9 | 1.94 | 80.3 | 18.1 | 29.4 | 3.70 | 3.70 | 1,023 |
| 2026-06 | 16.43 | 60.8 | 41.0 | 29.9 | 1.88 | 77.1 | 19.9 | 27.6 | 4.22 | 3.87 | 1,019 |

(The example prints all 28 reports; the table keeps two a year, plus the
quarters around 2022.)

- **Mostly floating in the zero-rate years, which is why 2022 cost it as
  little as it did.** Two-thirds to three-quarters of the portfolio reset
  its coupon with short rates from 2019 to 2022. The fixed-rate part was
  small (15–30%) but long for an ultra-short fund (4–6 years of final
  maturity, much of it amortizing mortgage-backed debt, whose final
  maturity overstates its life); that is the part marked down in 2022.
- **Investors left after the loss, and came back.** Net assets fell 43%,
  from $14.6 billion in March 2021 to $8.4 billion in March 2023, the low
  coming as the fund recovered, then rose to $16.4 billion by June 2026:
  money arriving for the yields of 2023–25, after the losses that produced
  them.
- **The coupon fell below the bill rate in 2022–24 while the return beat
  bills.** The coupon lags: bonds bought before the rise kept their low
  coupons but were marked down below par, and returned the difference as
  they matured (the pull to par), which is why 2023–25 beat bills by
  0.4–0.8 points with a coupon 0.5–1.5 points under them. The coupon is not
  the yield; N-PORT doesn't report yields per holding.
- **Since 2023, shorter and more corporate, then back toward government.**
  The fixed-rate maturity came down from 4.9 years (end of 2022) to under
  2, with a quarter of the fund maturing within a year. Corporate issuers
  peaked at 95% in mid-2024; government debt went from 3–9% in 2023–24 to
  18–20% in 2025–26.
- **June 2026: fixed-rate debt from 34% to 61% in a quarter**, the largest
  shift in the record, most of it maturing within a year (30%, from 21%).
  Floating fell to 41%, the lowest yet: the fund locking in rates for a
  year rather than letting them reset.

## Its return against bills

    cargo run --release -p store --example mint_treasuries

MINT's total return (Tiingo's adjusted closes, distributions reinvested)
against rolling 3-month bills (each day accruing that day's 3-month
constant-maturity yield, FRED `DGS3MO`):

| Year | MINT | 3-month bills | Difference |
|---|---|---|---|
| 2010 | +1.64% | +0.14% | +1.50 |
| 2011 | +0.45% | +0.05% | +0.39 |
| 2012 | +2.42% | +0.09% | +2.33 |
| 2013 | +0.72% | +0.06% | +0.66 |
| 2014 | +0.54% | +0.03% | +0.51 |
| 2015 | +0.43% | +0.05% | +0.38 |
| 2016 | +2.09% | +0.32% | +1.77 |
| 2017 | +1.86% | +0.95% | +0.91 |
| 2018 | +1.72% | +1.99% | −0.27 |
| 2019 | +3.33% | +2.13% | +1.21 |
| 2020 | +1.62% | +0.37% | +1.25 |
| 2021 | −0.03% | +0.05% | −0.08 |
| 2022 | −1.01% | +2.09% | −3.10 |
| 2023 | +6.25% | +5.41% | +0.84 |
| 2024 | +5.94% | +5.33% | +0.60 |
| 2025 | +4.74% | +4.30% | +0.44 |
| Nov 2009 – Nov 2026, a year | +2.13% | +1.56% | +0.56 |

Its own N-PORT filings give the same picture from mid-2019 (`fund_returns
MINT`, the fund's reported monthly returns compounded against the bill
rate): 2.96% a year against 2.78%.

- **Most of its edge came when bills paid nothing** (2010–2016, 2020): a
  credit spread of 1–2 points on a base of zero.
- **It lost to bills twice, both in years of Fed tightening:** 2018 a
  little, 2022 by 3.1 points, when its fixed-rate notes were marked down
  as the curve rose 2–4 points in nine months.
- **Since 2023 the edge has been under a point**, and narrowing: +0.84,
  +0.60, +0.44.

## Which part of the curve it moves with

Over four-week periods (leaving out March–April 2020, when it fell with
credit, not rates), MINT's return on the change in each Treasury yield:

| Maturity | 3m | 6m | 1y | 2y | 5y | 10y |
|---|---|---|---|---|---|---|
| % of price per point of yield | −0.39 | −0.50 | −0.48 | −0.35 | −0.25 | −0.16 |
| R² | 0.05 | 0.09 | 0.12 | 0.10 | 0.07 | 0.03 |

It behaves like a bond of about half a year, moving most with the 6-month
to 1-year yields. But the curve explains at most 12% of its month: the
rest is credit spreads and trading. Over a single week, almost none of it.

## As a signal: 2021

MINT's total return peaked on September 3, 2021 and was slipping by
October, when the 2-year yield jumped from 0.28% to 0.48% (the 6-month and
1-year yields didn't move until December). SPY peaked on January 3, 2022
and fell 24.5% by October. Month ends, from each one's peak:

| Month | MINT | SPY | 3m | 1y | 2y | 10y |
|---|---|---|---|---|---|---|
| 2021-09 | −0.05% | −5.0% | 0.04 | 0.09 | 0.28 | 1.52 |
| 2021-10 | −0.15% | 0.0% | 0.05 | 0.15 | 0.48 | 1.55 |
| 2021-11 | −0.22% | −3.0% | 0.05 | 0.24 | 0.52 | 1.43 |
| 2021-12 | −0.27% | −0.5% | 0.06 | 0.39 | 0.73 | 1.52 |
| 2022-01 | −0.44% | −5.8% | 0.22 | 0.78 | 1.18 | 1.79 |
| 2022-03 | −1.55% | −5.2% | 0.52 | 1.63 | 2.28 | 2.32 |
| 2022-06 | −2.31% | −20.4% | 1.72 | 2.80 | 2.92 | 2.98 |
| 2022-09 | −2.11% | −24.4% | 3.33 | 4.05 | 4.22 | 3.83 |

But as a general signal it fails. Every time since 2009 that MINT fell
0.25% below its six-month high:

| First day | SPY, 3 months | 6 months | 12 months | Worst drawdown in 12 |
|---|---|---|---|---|
| 2010-12-16 | +3.3% | +3.3% | −0.2% | −18.6% |
| 2011-08-16 | +5.4% | +13.8% | +20.4% | −9.7% |
| 2013-06-25 | +7.4% | +16.3% | +25.8% | −5.7% |
| 2013-09-05 | +8.8% | +14.6% | +23.6% | −5.7% |
| 2015-09-30 | +8.1% | +9.2% | +14.5% | −12.8% |
| 2020-03-09 | +18.6% | +26.2% | +43.9% | −22.2% |
| **2021-11-23** | **−8.2%** | **−14.6%** | **−12.7%** | **−24.5%** |
| 2023-03-14 | +11.9% | +14.9% | +33.5% | −10.0% |
| Every day | | | +15.0% | (10% of years negative) |

Only 2021 preceded a bad year. Of SPY's six declines of 15% or more since
2010 (from peaks in April 2010, April 2011, September 2018, February 2020,
January 2022 and February 2025), MINT gave warning of 2022, loosely of
2011, and of none of the others: in 2020 it fell with stocks, and in 2018
and 2025 it said nothing.

What 2021 had that the others didn't: the decline was about $`r`$. The
market went from expecting no rate increases to pricing a fast series of
them, which lowers the value of every long-dated cash flow and most of all
the long-duration stocks. MINT, a half-year bond, felt it first. A decline
from earnings, credit or a shock (2020, 2025) it can't see, and a
tightening the market expects (2018, the 2-year yield rising steadily from
2.0% to 2.9%) costs it nothing, because it rolls into the higher yields as
it goes. The 2-year yield itself shows the same thing without MINT's
credit and trading noise.

## Caveats

- **Adjusted closes are market prices**, not net asset value: an ETF can
  trade a little away from its NAV, most of all in March 2020.
- **Rolling bills are approximated** from constant-maturity yields
  (`y/365` a day), not bills actually bought and held.
- **One episode** makes the 2021 signal, and the threshold (0.25% off a
  six-month high) was chosen after looking at it.
- **N-PORT starts in late 2019**: the portfolio before it (and through
  2013's taper tantrum) would need the N-Q and N-CSR schedules of
  investments, which aren't in the data sets.
- **Final maturity, not duration**, for the fixed-rate debt: mortgage- and
  asset-backed securities pay down long before it.

## To do

- Fund traits for the N-CSR statements (assets and liabilities, operations,
  changes in net assets), and MINT's from 2010.
- The same comparison for the money market funds the census flags, the
  plainer r for cash.
