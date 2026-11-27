# Car makers as lenders, and the US auto loan market

Notes from 2026-11-21, for the auto industry dashboard
(`equities/grafana/auto.json`) and a possible auto finance one. Figures
marked "about" are approximate; the precise ones are in the reports cited.

## Captive finance companies

Every big car maker owns a **captive finance company**, a lender whose main
job is financing its parent's cars:

- **Ford Credit**
- **GM Financial.** GM's original lender, GMAC, was spun off after the 2008
  bailout and became Ally Bank. GM rebuilt its lending arm in 2010 by buying
  AmeriCredit, a subprime lender.
- **Toyota Financial Services** (金融事業, *kin'yū jigyō*, "financial services
  business", in Toyota's own filings)
- **Honda Financial Services**, **Stellantis Financial Services**, and
  **Ferrari Financial Services** (small, for luxury buyers)
- **Tesla** is the exception. It mostly passes loans to partner banks but
  keeps leases on its own balance sheet, which is the "vehicles on operating
  leases" line on the dashboard.

### What they lend for

1. **Retail loans** to car buyers.
2. **Leases.** The lender owns the car and bets on its **residual value**,
   what it will be worth when the lease ends. If used-car prices fall, the
   lender loses.
3. **Dealer floorplan**, also called wholesale. Dealers borrow to hold their
   inventory: the car maker ships cars to dealers, and its lending arm
   finances them until they sell.

### Why car makers do it

- **To sell cars.** "0% APR" deals are *subvention*: the car maker pays its
  own lender the interest the customer doesn't. It's a price cut dressed as a
  loan.
- **To keep dealers stocked** through floorplan financing.
- **Profit and stability.** Interest income is steadier than car sales.

### How they fund it

They pool loans and leases into **asset-backed securities (ABS)** sold to
investors, and issue bonds and commercial paper. That's why a car maker's
balance sheet looks like a bank's. From our own data (items
`FinancialServicesRevenue`, `FinanceReceivablesCurrent` +
`FinanceReceivablesNonCurrent`):

| Captive                                    | Revenue | Finance receivables |
|--------------------------------------------|---------|---------------------|
| Ford Credit (FY2025)                       | $13.3B  | $110.6B             |
| GM Financial (FY2025)                      | $17.0B  | $89.7B              |
| Toyota Financial Services (FY to Mar 2026) | ¥4.8T   | ¥39.0T              |

The lending arm is small in revenue and huge in assets. Most of the car
makers' debt funds these loans, which is why Ford's balance sheet looks so
leveraged.

### The risk

It showed in 2008. Captives depend on capital markets for funding, and when
those froze, GMAC and Chrysler Financial needed bailouts, which nearly took
their parents down with them.

## The US auto loan market

- **About $1.6–1.7 trillion outstanding**, the third-largest kind of
  household debt after mortgages and student loans. The NY Fed publishes it
  quarterly in its *Household Debt and Credit* report.
- **Who lends:** banks, credit unions, captives (by far the largest share of
  *new*-car loans), independent finance companies (Ally, Credit Acceptance),
  and "buy here, pay here" dealers that lend to their own customers.
- **Conditions:**
  - Since 2022, rates have risen sharply: new-car loans average about 7%,
    used about 11%, and higher for subprime borrowers.
  - Average new-car monthly payments are over about $700.
  - Subprime delinquencies (60+ days late, tracked by Fitch's ABS indexes)
    have been at or near record highs since 2024.
  - The September 2025 bankruptcy of Tricolor, a subprime auto lender, with
    fraud allegations involving loans pledged twice as collateral, made the
    risk visible.

## Carvana

Carvana (CVNA) is as much a lender as a used-car retailer:

- **It makes loans** to its own customers, many of them subprime or
  near-prime.
- **But doesn't keep them.** It sells loans to Ally under a standing purchase
  agreement and packages others into its own ABS.
- **Its profit relies on those sales:** the *gain on sale* of loans is a large
  part of its gross profit. So its results depend on investors' appetite for
  subprime auto debt, not just on car sales.
- **The contrast:** CarMax (KMX) does the opposite, keeping its auto loans
  (CarMax Auto Finance) on its own balance sheet.

## Bringing it into the dashboards

Auto finance as its own lens rather than mixed into the car makers:

- **The lenders, side by side:**
  - the captives (Ford Credit, GM Financial, Toyota Financial Services,
    already partly in the data);
  - **Ally** (ALLY), the old GMAC and the largest independent;
  - **Credit Acceptance** (CACC), pure subprime;
  - **Carvana** (CVNA), which originates and sells;
  - **CarMax** (KMX), which originates and keeps.
- **Items** that tell the story:
  - receivables, and receivables held for sale (Carvana's pipeline);
  - gain on sale of loans;
  - provision for credit losses and allowance for loan losses: how much each
    lender expects to lose, and so the clearest signal of stress;
  - financial-services revenue.

A "lending arm" row on the auto dashboard limited to car makers (as now),
plus a separate auto finance list and dashboard for the lenders.
