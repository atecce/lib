//! The Goldman Sachs Group since its initial public offering in May 1999:
//! the per-share record from the 10-Ks, and the 2025 statements as Goldman
//! presents them. Notes on the company: `README.md` here.
//!
//! Goldman has preferred stock and pays dividends, so the per-share reading
//! is per *common* share: book value per common share as Goldman reports it
//! (common equity over common shares outstanding, including vested
//! restricted stock units), and diluted earnings per common share (net
//! earnings less preferred dividends). Residual income per share is
//! `EPS_t − r·B_{t−1}`.
//!
//! Its fiscal year ended on the last Friday of November through 2008, then
//! changed to the calendar year when it became a bank holding company;
//! December 2008 was reported as a one-month transition period
//! (`DECEMBER_2008`).
//!
//! The balance sheet is a bank's, unclassified (no current and non-current
//! totals), and the income statement has no gross profit: so the statements
//! implement `BalanceSheet`, `IncomeStatement` and `CashFlowStatement`, but
//! not `ClassifiedBalanceSheet` or `MultiStepIncomeStatement`.

use chrono::NaiveDate;

use crate::Ticker;
use crate::Period;
use crate::Statement;
use crate::BalanceSheet as BS;
use crate::IncomeStatement as IS;
use crate::CashFlowStatement as CFS;

/// The IPO: May 4, 1999, at $53 a share.
pub const IPO: (NaiveDate, f64) = (NaiveDate::from_ymd_opt(1999, 5, 4).unwrap(), 53.0);

/// A fiscal year from a 10-K's selected financial data (or, from 2019, its
/// financial highlights): per common share, in dollars.
#[derive(Debug, Clone, Copy)]
pub struct Annual {
    pub year: i32,
    pub year_end: NaiveDate,
    pub book_value: f64,
    pub diluted_eps: f64,
    pub dividends_declared: f64,
    /// The filing the figures are taken from (the latest that states them).
    pub accession: &'static str,
}

const fn ymd(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

const fn year(year: i32, year_end: NaiveDate, book_value: f64, diluted_eps: f64, dividends_declared: f64, accession: &'static str) -> Annual {
    Annual { year, year_end, book_value, diluted_eps, dividends_declared, accession }
}

/// 1999 is the first year with a book value per share: until the IPO,
/// Goldman was a partnership. Its 1999 earnings include the IPO's one-time
/// awards to employees ($2.26 billion) and a tax benefit from leaving
/// partnership form.
pub const ANNUAL: [Annual; 27] = [
    // The 2003 annual report's selected financial data (Exhibit 13.4 to the 10-K).
    year(1999, ymd(1999, 11, 26), 20.94, 5.57, 0.24, "0000950123-04-002239"),
    year(2000, ymd(2000, 11, 24), 32.18, 6.00, 0.48, "0000950123-04-002239"),
    year(2001, ymd(2001, 11, 30), 36.33, 4.26, 0.48, "0000950123-04-002239"),
    year(2002, ymd(2002, 11, 29), 38.69, 4.03, 0.48, "0000950123-04-002239"),
    year(2003, ymd(2003, 11, 28), 43.60, 5.87, 0.74, "0000950123-04-002239"),
    // The 2008 10-K.
    year(2004, ymd(2004, 11, 26), 50.77, 8.92, 1.00, "0000950123-09-001278"),
    year(2005, ymd(2005, 11, 25), 57.02, 11.21, 1.00, "0000950123-09-001278"),
    year(2006, ymd(2006, 11, 24), 72.62, 19.69, 1.30, "0000950123-09-001278"),
    year(2007, ymd(2007, 11, 30), 90.43, 24.73, 1.40, "0000950123-09-001278"),
    year(2008, ymd(2008, 11, 28), 98.68, 4.47, 1.40, "0000950123-09-001278"),
    // The 2013 10-K. 2009 is the calendar year; December 2008 is the stub below.
    year(2009, ymd(2009, 12, 31), 117.48, 22.13, 1.05, "0001193125-14-073792"),
    year(2010, ymd(2010, 12, 31), 128.72, 13.18, 1.40, "0001193125-14-073792"),
    year(2011, ymd(2011, 12, 31), 130.31, 4.51, 1.40, "0001193125-14-073792"),
    year(2012, ymd(2012, 12, 31), 144.67, 14.13, 1.77, "0001193125-14-073792"),
    year(2013, ymd(2013, 12, 31), 152.48, 15.46, 2.05, "0001193125-14-073792"),
    // The 2018 10-K.
    year(2014, ymd(2014, 12, 31), 163.01, 17.07, 2.25, "0001193125-19-050198"),
    year(2015, ymd(2015, 12, 31), 171.03, 12.14, 2.55, "0001193125-19-050198"),
    year(2016, ymd(2016, 12, 31), 182.47, 16.29, 2.60, "0001193125-19-050198"),
    year(2017, ymd(2017, 12, 31), 181.00, 9.01, 2.90, "0001193125-19-050198"),
    year(2018, ymd(2018, 12, 31), 207.36, 25.27, 3.15, "0001193125-19-050198"),
    // The 2021 10-K (2019's book value from the 2019 10-K, 0001193125-20-043853).
    year(2019, ymd(2019, 12, 31), 218.52, 21.03, 4.15, "0001193125-22-052682"),
    year(2020, ymd(2020, 12, 31), 236.15, 24.74, 5.00, "0001193125-22-052682"),
    year(2021, ymd(2021, 12, 31), 284.39, 59.45, 6.50, "0001193125-22-052682"),
    // Each year's own 10-K.
    year(2022, ymd(2022, 12, 31), 303.55, 30.06, 9.00, "0000886982-23-000003"),
    year(2023, ymd(2023, 12, 31), 313.56, 22.87, 10.50, "0000886982-24-000006"),
    year(2024, ymd(2024, 12, 31), 336.77, 40.54, 11.50, "0000886982-25-000005"),
    year(2025, ymd(2025, 12, 31), 357.60, 51.32, 14.00, "0000886982-26-000091"),
];

/// The one month ended December 26, 2008, between the last November fiscal
/// year and the first calendar one (the 2009 10-K, 0000950123-10-018464):
/// a net loss of $780 million, $1,028 million after $248 million of
/// preferred dividends (most of them on the $10 billion of TARP preferred
/// and Berkshire's $5 billion), and the dividend declared for the four
/// months to March 2009.
pub const DECEMBER_2008_EPS: f64 = -2.15;
pub const DECEMBER_2008_DIVIDEND: f64 = 0.4666666;

/// One year of the per-share record, ready for residual income: the book
/// value it started and ended with, and what it earned and paid between.
#[derive(Debug, Clone, Copy)]
pub struct Year {
    pub year: i32,
    pub year_end: NaiveDate,
    pub opening_book: f64,
    pub closing_book: f64,
    pub eps: f64,
    pub dividends: f64,
    /// 12, or 13 for 2009 (December 2008 included).
    pub months: u32,
}

impl Year {
    /// Return on opening book value, for the period (not annualized).
    pub fn roe(&self) -> f64 {
        self.eps / self.opening_book
    }

    /// EPS less the cost of the opening book over the period, at annual `r`.
    pub fn residual_income(&self, r: f64) -> f64 {
        self.eps - ((1.0 + r).powf(self.months as f64 / 12.0) - 1.0) * self.opening_book
    }
}

/// 2000 to 2025, each year opening with the previous one's book value; 2009
/// runs from November 2008 and includes the December stub.
pub fn years() -> Vec<Year> {
    ANNUAL.windows(2).map(|w| {
        let stub = w[1].year == 2009;
        Year {
            year: w[1].year,
            year_end: w[1].year_end,
            opening_book: w[0].book_value,
            closing_book: w[1].book_value,
            eps: w[1].diluted_eps + if stub { DECEMBER_2008_EPS } else { 0.0 },
            dividends: w[1].dividends_declared + if stub { DECEMBER_2008_DIVIDEND } else { 0.0 },
            months: if stub { 13 } else { 12 },
        }
    }).collect()
}

/// The consolidated statement of financial condition (balance sheet), in
/// Goldman's order: assets by liquidity, no current/non-current split.
pub struct BalanceSheet {
    pub date: NaiveDate,

    pub cash_and_cash_equivalents: f64,
    pub securities_purchased_under_agreements_to_resell: f64,
    pub securities_borrowed: f64,
    pub customer_and_other_receivables: f64,
    pub trading_assets: f64,
    pub available_for_sale_securities: f64,
    pub held_to_maturity_securities: f64,
    pub other_investments: f64,
    pub loans: f64,
    pub other_assets: f64,

    pub deposits: f64,
    pub securities_sold_under_agreements_to_repurchase: f64,
    pub securities_loaned: f64,
    pub other_secured_financings: f64,
    pub customer_and_other_payables: f64,
    pub trading_liabilities: f64,
    pub unsecured_short_term_borrowings: f64,
    pub unsecured_long_term_borrowings: f64,
    pub other_liabilities: f64,

    pub preferred_stock: f64,
    pub common_stock: f64,
    pub share_based_awards: f64,
    pub additional_paid_in_capital: f64,
    pub retained_earnings: f64,
    pub accumulated_other_comprehensive_loss: f64,
    pub stock_held_in_treasury: f64,
    pub common_shares_outstanding: f64,
}

impl Statement for BalanceSheet {
    fn ticker(&self) -> Ticker {
        Ticker::GS
    }
    fn period(&self) -> Period {
        Period::PointInTime
    }
}

impl BalanceSheet {
    /// Repurchase agreements and securities borrowed: lending against
    /// collateral, the modern form of the short-term paper Goldman began in.
    pub fn collateralized_agreements(&self) -> f64 {
        self.securities_purchased_under_agreements_to_resell + self.securities_borrowed
    }
    pub fn investments(&self) -> f64 {
        self.available_for_sale_securities + self.held_to_maturity_securities + self.other_investments
    }
    pub fn collateralized_financings(&self) -> f64 {
        self.securities_sold_under_agreements_to_repurchase + self.securities_loaned + self.other_secured_financings
    }
    pub fn total_shareholders_equity(&self) -> f64 {
        self.preferred_stock + self.common_stock + self.share_based_awards + self.additional_paid_in_capital
            + self.retained_earnings + self.accumulated_other_comprehensive_loss + self.stock_held_in_treasury
    }
    pub fn common_shareholders_equity(&self) -> f64 {
        self.total_shareholders_equity() - self.preferred_stock
    }
    /// Assets over equity: about 14 at the end of 2025.
    pub fn leverage(&self) -> f64 {
        self.total_assets() / self.total_shareholders_equity()
    }
    pub fn imbalance(&self) -> f64 {
        self.total_assets() - self.total_liabilities() - self.total_shareholders_equity()
    }
}

impl BS for BalanceSheet {
    fn total_assets(&self) -> f64 {
        self.cash_and_cash_equivalents
            + self.collateralized_agreements()
            + self.customer_and_other_receivables
            + self.trading_assets
            + self.investments()
            + self.loans
            + self.other_assets
    }
    fn total_liabilities(&self) -> f64 {
        self.deposits
            + self.collateralized_financings()
            + self.customer_and_other_payables
            + self.trading_liabilities
            + self.unsecured_short_term_borrowings
            + self.unsecured_long_term_borrowings
            + self.other_liabilities
    }
}

/// The consolidated statement of earnings: revenues by business, then net
/// interest income, then expenses by kind.
pub struct IncomeStatement {
    pub date: NaiveDate,
    pub p: Period,

    pub investment_banking: f64,
    pub investment_management: f64,
    pub commissions_and_fees: f64,
    pub market_making: f64,
    pub other_principal_transactions: f64,
    pub interest_income: f64,
    pub interest_expense: f64,
    pub provision_for_credit_losses: f64,

    pub compensation_and_benefits: f64,
    pub transaction_based: f64,
    pub market_development: f64,
    pub communications_and_technology: f64,
    pub depreciation_and_amortization: f64,
    pub occupancy: f64,
    pub professional_fees: f64,
    pub other_expenses: f64,

    pub provision_for_taxes: f64,
    pub preferred_stock_dividends: f64,
    pub diluted_average_common_shares: f64,
}

impl Statement for IncomeStatement {
    fn ticker(&self) -> Ticker {
        Ticker::GS
    }
    fn period(&self) -> Period {
        self.p
    }
}

impl IncomeStatement {
    pub fn total_non_interest_revenues(&self) -> f64 {
        self.investment_banking + self.investment_management + self.commissions_and_fees
            + self.market_making + self.other_principal_transactions
    }
    pub fn net_interest_income(&self) -> f64 {
        self.interest_income - self.interest_expense
    }
    pub fn total_net_revenues(&self) -> f64 {
        self.total_non_interest_revenues() + self.net_interest_income()
    }
    pub fn total_operating_expenses(&self) -> f64 {
        self.compensation_and_benefits + self.transaction_based + self.market_development
            + self.communications_and_technology + self.depreciation_and_amortization + self.occupancy
            + self.professional_fees + self.other_expenses
    }
    /// Compensation over net revenues: the partners' share, in a public
    /// company's accounts.
    pub fn compensation_ratio(&self) -> f64 {
        self.compensation_and_benefits / self.total_net_revenues()
    }
    pub fn net_earnings_applicable_to_common_shareholders(&self) -> f64 {
        self.net_income() - self.preferred_stock_dividends
    }
    pub fn diluted_eps(&self) -> f64 {
        self.net_earnings_applicable_to_common_shareholders() / self.diluted_average_common_shares
    }
}

impl IS for IncomeStatement {
    fn income_before_income_tax(&self) -> f64 {
        self.total_net_revenues() - self.provision_for_credit_losses - self.total_operating_expenses()
    }
    fn net_income(&self) -> f64 {
        self.income_before_income_tax() - self.provision_for_taxes
    }
}

/// The consolidated statement of cash flows. A dealer's operating cash flow
/// is mostly the change in its trading book and is not a measure of
/// earnings: in 2025 it was −$45 billion on net earnings of $17 billion.
pub struct CashFlowStatement {
    pub date: NaiveDate,
    pub p: Period,

    pub net_earnings: f64,
    pub depreciation_and_amortization: f64,
    pub deferred_income_taxes: f64,
    pub share_based_compensation: f64,
    pub provision_for_credit_losses: f64,
    pub customer_and_other_receivables_and_payables_net: f64,
    pub collateralized_transactions_net: f64,
    pub trading_assets: f64,
    pub trading_liabilities: f64,
    pub loans_held_for_sale_net: f64,
    pub other_operating_net: f64,

    pub purchase_of_property_leasehold_improvements_and_equipment: f64,
    pub proceeds_from_sales_of_property_leasehold_improvements_and_equipment: f64,
    pub net_cash_received_from_business_dispositions: f64,
    pub available_for_sale_purchases: f64,
    pub available_for_sale_proceeds_from_sales: f64,
    pub available_for_sale_proceeds_from_paydowns_and_maturities: f64,
    pub held_to_maturity_purchases: f64,
    pub held_to_maturity_proceeds_from_paydowns_and_maturities: f64,
    pub other_investments_purchases: f64,
    pub other_investments_proceeds: f64,
    pub loans_net: f64,

    pub unsecured_short_term_borrowings_net: f64,
    pub other_secured_financings_short_term_net: f64,
    pub proceeds_from_other_secured_financings_long_term: f64,
    pub repayment_of_other_secured_financings_long_term: f64,
    pub proceeds_from_unsecured_long_term_borrowings: f64,
    pub repayment_of_unsecured_long_term_borrowings: f64,
    pub derivative_contracts_with_a_financing_element_net: f64,
    pub deposits_net: f64,
    pub preferred_stock_redemption: f64,
    pub common_stock_repurchased: f64,
    pub settlement_of_share_based_awards_for_withholding_tax: f64,
    pub dividends_paid: f64,
    pub proceeds_from_issuance_of_preferred_stock: f64,
    pub other_financing_net: f64,

    pub effect_of_exchange_rate_changes: f64,
}

impl Statement for CashFlowStatement {
    fn ticker(&self) -> Ticker {
        Ticker::GS
    }
    fn period(&self) -> Period {
        self.p
    }
}

impl CFS for CashFlowStatement {
    fn net_cash_from_operating_activities(&self) -> f64 {
        self.net_earnings
            + self.depreciation_and_amortization
            + self.deferred_income_taxes
            + self.share_based_compensation
            + self.provision_for_credit_losses
            + self.customer_and_other_receivables_and_payables_net
            + self.collateralized_transactions_net
            + self.trading_assets
            + self.trading_liabilities
            + self.loans_held_for_sale_net
            + self.other_operating_net
    }
    fn net_cash_from_investing_activities(&self) -> f64 {
        self.purchase_of_property_leasehold_improvements_and_equipment
            + self.proceeds_from_sales_of_property_leasehold_improvements_and_equipment
            + self.net_cash_received_from_business_dispositions
            + self.available_for_sale_purchases
            + self.available_for_sale_proceeds_from_sales
            + self.available_for_sale_proceeds_from_paydowns_and_maturities
            + self.held_to_maturity_purchases
            + self.held_to_maturity_proceeds_from_paydowns_and_maturities
            + self.other_investments_purchases
            + self.other_investments_proceeds
            + self.loans_net
    }
    fn net_cash_from_financing_activities(&self) -> f64 {
        self.unsecured_short_term_borrowings_net
            + self.other_secured_financings_short_term_net
            + self.proceeds_from_other_secured_financings_long_term
            + self.repayment_of_other_secured_financings_long_term
            + self.proceeds_from_unsecured_long_term_borrowings
            + self.repayment_of_unsecured_long_term_borrowings
            + self.derivative_contracts_with_a_financing_element_net
            + self.deposits_net
            + self.preferred_stock_redemption
            + self.common_stock_repurchased
            + self.settlement_of_share_based_awards_for_withholding_tax
            + self.dividends_paid
            + self.proceeds_from_issuance_of_preferred_stock
            + self.other_financing_net
    }
    fn change_in_cash(&self) -> f64 {
        self.net_cash_from_operating_activities()
            + self.net_cash_from_investing_activities()
            + self.net_cash_from_financing_activities()
            + self.effect_of_exchange_rate_changes
    }
}

impl_reported! {
    BalanceSheet,
    [
        CashAndCashEquivalents => field cash_and_cash_equivalents,
        TotalAssets => method total_assets,
        TotalLiabilities => method total_liabilities,
        StockholdersEquity => method total_shareholders_equity,
    ]
}

impl_reported! {
    IncomeStatement,
    [
        Revenue => method total_net_revenues,
        InterestIncome => field interest_income,
        InterestExpense => field interest_expense,
        TotalOperatingExpenses => method total_operating_expenses,
        IncomeBeforeIncomeTax => method income_before_income_tax,
        IncomeTaxExpense => field provision_for_taxes,
        NetIncome => method net_income,
    ]
}

impl_reported! {
    CashFlowStatement,
    [
        DepreciationAndAmoritization => field depreciation_and_amortization,
        StockBasedCompensationExpense => field share_based_compensation,
        NetCashProvidedByOperatingActivities => method net_cash_from_operating_activities,
        NetCashUsedInInvestingActivities => method net_cash_from_investing_activities,
        PaymentsRelatedToRepurchasesOfCommonStock => field common_stock_repurchased,
        NetCashUsedInFinancingActivities => method net_cash_from_financing_activities,
        ChangeInCashAndCashEquivalents => method change_in_cash,
    ]
}

const M: f64 = 1e6;

/// December 31, 2025, from the 10-K (0000886982-26-000091; dollars, the
/// filing states millions).
pub const BALANCE_SHEET_2025: BalanceSheet = BalanceSheet {
    date: ymd(2025, 12, 31),
    cash_and_cash_equivalents: 164_259.0 * M,
    securities_purchased_under_agreements_to_resell: 126_007.0 * M,
    securities_borrowed: 208_208.0 * M,
    customer_and_other_receivables: 185_842.0 * M,
    trading_assets: 656_796.0 * M,
    available_for_sale_securities: 99_244.0 * M,
    held_to_maturity_securities: 69_193.0 * M,
    other_investments: 25_825.0 * M,
    loans: 237_734.0 * M,
    other_assets: 36_212.0 * M,

    deposits: 501_422.0 * M,
    securities_sold_under_agreements_to_repurchase: 223_384.0 * M,
    securities_loaned: 53_644.0 * M,
    other_secured_financings: 28_021.0 * M,
    customer_and_other_payables: 231_865.0 * M,
    trading_liabilities: 262_552.0 * M,
    unsecured_short_term_borrowings: 70_459.0 * M,
    unsecured_long_term_borrowings: 285_500.0 * M,
    other_liabilities: 27_501.0 * M,

    preferred_stock: 15_153.0 * M,
    common_stock: 9.0 * M,
    share_based_awards: 5_795.0 * M,
    additional_paid_in_capital: 61_906.0 * M,
    retained_earnings: 165_288.0 * M,
    accumulated_other_comprehensive_loss: -2_260.0 * M,
    stock_held_in_treasury: -120_919.0 * M,
    common_shares_outstanding: 296_476_742.0,
};

pub const INCOME_STATEMENT_2025: IncomeStatement = IncomeStatement {
    date: ymd(2025, 12, 31),
    p: Period::TwelveMonths,
    investment_banking: 9_348.0 * M,
    investment_management: 11_749.0 * M,
    commissions_and_fees: 4_042.0 * M,
    market_making: 17_993.0 * M,
    other_principal_transactions: 1_592.0 * M,
    interest_income: 80_373.0 * M,
    interest_expense: 66_814.0 * M,
    provision_for_credit_losses: -1_113.0 * M,

    compensation_and_benefits: 18_906.0 * M,
    transaction_based: 7_997.0 * M,
    market_development: 710.0 * M,
    communications_and_technology: 2_170.0 * M,
    depreciation_and_amortization: 2_182.0 * M,
    occupancy: 958.0 * M,
    professional_fees: 1_770.0 * M,
    other_expenses: 2_851.0 * M,

    provision_for_taxes: 4_676.0 * M,
    preferred_stock_dividends: 876.0 * M,
    diluted_average_common_shares: 317.6 * M,
};

pub const CASH_FLOW_STATEMENT_2025: CashFlowStatement = CashFlowStatement {
    date: ymd(2025, 12, 31),
    p: Period::TwelveMonths,
    net_earnings: 17_176.0 * M,
    depreciation_and_amortization: 2_182.0 * M,
    deferred_income_taxes: 204.0 * M,
    share_based_compensation: 3_445.0 * M,
    provision_for_credit_losses: -1_113.0 * M,
    customer_and_other_receivables_and_payables_net: -43_206.0 * M,
    collateralized_transactions_net: -12_891.0 * M,
    trading_assets: -69_866.0 * M,
    trading_liabilities: 57_560.0 * M,
    loans_held_for_sale_net: -504.0 * M,
    other_operating_net: 1_859.0 * M,

    purchase_of_property_leasehold_improvements_and_equipment: -2_064.0 * M,
    proceeds_from_sales_of_property_leasehold_improvements_and_equipment: 803.0 * M,
    net_cash_received_from_business_dispositions: 1_536.0 * M,
    available_for_sale_purchases: -77_429.0 * M,
    available_for_sale_proceeds_from_sales: 61_325.0 * M,
    available_for_sale_proceeds_from_paydowns_and_maturities: 43.0 * M,
    held_to_maturity_purchases: -11_279.0 * M,
    held_to_maturity_proceeds_from_paydowns_and_maturities: 21_687.0 * M,
    other_investments_purchases: -9_392.0 * M,
    other_investments_proceeds: 9_892.0 * M,
    loans_net: -39_349.0 * M,

    unsecured_short_term_borrowings_net: 2_988.0 * M,
    other_secured_financings_short_term_net: -3_877.0 * M,
    proceeds_from_other_secured_financings_long_term: 6_510.0 * M,
    repayment_of_other_secured_financings_long_term: -3_331.0 * M,
    proceeds_from_unsecured_long_term_borrowings: 88_196.0 * M,
    repayment_of_unsecured_long_term_borrowings: -71_502.0 * M,
    derivative_contracts_with_a_financing_element_net: 2_313.0 * M,
    deposits_net: 62_783.0 * M,
    preferred_stock_redemption: 0.0,
    common_stock_repurchased: -12_360.0 * M,
    settlement_of_share_based_awards_for_withholding_tax: -2_156.0 * M,
    dividends_paid: -5_277.0 * M,
    proceeds_from_issuance_of_preferred_stock: 1_895.0 * M,
    other_financing_net: -82.0 * M,

    effect_of_exchange_rate_changes: 5_448.0 * M,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// Goldman's top line is "Total net revenues", tagged
    /// `RevenuesNetOfInterestExpense` (not `Revenues`): a bank's revenues are
    /// its interest income net of interest expense, plus its fees and
    /// trading. It reads as Revenue (item.rs lists the element last, after
    /// the usual totals). Its 2025 net revenues: $58.28 billion.
    #[test]
    fn net_revenues_are_revenue() {
        let row = crate::filing::row_item(Some("us-gaap_RevenuesNetOfInterestExpense"), Some("Total net revenues"));
        assert_eq!(row.map(|(item, _)| item), Some(crate::item::Item::Revenue));
    }

    /// The computed totals equal the 10-K's printed ones.
    #[test]
    fn statements_match_the_filing() {
        let b = &BALANCE_SHEET_2025;
        assert_eq!(b.total_assets(), 1_809_320.0 * M);
        assert_eq!(b.total_liabilities(), 1_684_348.0 * M);
        assert_eq!(b.total_shareholders_equity(), 124_972.0 * M);
        assert_eq!(b.imbalance(), 0.0);

        let i = &INCOME_STATEMENT_2025;
        assert_eq!(i.total_non_interest_revenues(), 44_724.0 * M);
        assert_eq!(i.net_interest_income(), 13_559.0 * M);
        assert_eq!(i.total_net_revenues(), 58_283.0 * M);
        assert_eq!(i.total_operating_expenses(), 37_544.0 * M);
        assert_eq!(i.income_before_income_tax(), 21_852.0 * M);
        assert_eq!(i.net_income(), 17_176.0 * M);
        assert_eq!(i.net_earnings_applicable_to_common_shareholders(), 16_300.0 * M);
        // EPS from rounded average shares (317.6 million), to the cent.
        assert!((i.diluted_eps() - ANNUAL[26].diluted_eps).abs() < 0.01, "{}", i.diluted_eps());

        let c = &CASH_FLOW_STATEMENT_2025;
        assert_eq!(c.net_cash_from_operating_activities(), -45_154.0 * M);
        assert_eq!(c.net_cash_from_investing_activities(), -44_227.0 * M);
        assert_eq!(c.net_cash_from_financing_activities(), 66_100.0 * M);
        assert_eq!(c.change_in_cash(), -17_833.0 * M);
        assert_eq!(c.net_earnings, i.net_income());
    }

    /// The per-share record runs fiscal 1999 to 2025 without a gap, and its
    /// last year agrees with the 2025 statements: common equity over the
    /// shares outstanding (which exclude the RSUs Goldman's book value per
    /// share counts) is a little above the reported $357.60.
    #[test]
    fn the_record_is_whole() {
        for (i, a) in ANNUAL.iter().enumerate() {
            assert_eq!(a.year, 1999 + i as i32);
        }
        let ys = years();
        assert_eq!(ys.len(), 26);
        assert_eq!(ys.iter().find(|y| y.year == 2009).unwrap().months, 13);
        let b = &BALANCE_SHEET_2025;
        let per_outstanding = b.common_shareholders_equity() / b.common_shares_outstanding;
        let reported = ANNUAL[26].book_value;
        assert!(per_outstanding > reported && per_outstanding / reported < 1.05, "{}", per_outstanding);
    }

    /// Where the generic reader and these statements name the same item,
    /// they agree.
    #[test]
    fn the_reader_agrees() {
        let path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../edgar/cache/886982/000088698226000091"));
        if !path.exists() { return; }
        let mut r = crate::filing::new_reader(path, Ticker::GS).unwrap();
        let read: Vec<crate::item::Reported> = [
            crate::Reader::process_balance_sheet(&mut r).unwrap(),
            crate::Reader::process_income_statement(&mut r).unwrap(),
        ].concat();
        let mine = BALANCE_SHEET_2025.reported().into_iter().chain(INCOME_STATEMENT_2025.reported());
        let mut agreed = 0;
        let mut differ = Vec::new();
        for m in mine {
            if let Some(r) = read.iter().find(|r| r.item == m.item && r.date == m.date && r.p == m.p) {
                if r.val == m.val { agreed += 1 } else { differ.push((m.item, r.val, m.val)) }
            }
        }
        assert!(differ.is_empty(), "the reader differs: {:?}", differ);
        assert!(agreed >= 5, "only {} items agreed", agreed);
    }
}
