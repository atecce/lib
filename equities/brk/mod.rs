//! Berkshire Hathaway's record per share since Buffett took control in 1965:
//! book value and market value, from his own scorecard, then the 10-Ks.
//! Notes on the company: `README.md` here.
//!
//! Berkshire suits a per-share, clean surplus reading. It has paid one
//! dividend since 1965 (10 cents, in 1967), and until 2018 bought back
//! almost nothing, so the change in book value per share is its
//! comprehensive earnings per share: residual income per share is
//! `B_t − (1 + r)·B_{t−1}`, and comprehensive ROE is the growth of book value
//! per share, the yardstick Buffett opened every letter with until 2018.

use chrono::NaiveDate;

use crate::Ticker;
use crate::Period;
use crate::Statement;
use crate::BalanceSheet as BS;
use crate::IncomeStatement as IS;
use crate::CashFlowStatement as CFS;

/// Book value per share when Buffett's management began (the fiscal year
/// to September 30, 1964), as his letters state it.
pub const BOOK_1964: f64 = 19.46;

/// "Berkshire's Performance vs. the S&P 500", from the 2018 letter (the last
/// with the book value column): each year's percentage change in per-share
/// book value and in per-share market value. 1965 and 1966 are years to
/// September 30; 1967 is the fifteen months to December 31. Results through
/// 1978 are restated for the 1979 rule valuing insurers' equity holdings
/// at market.
pub const LETTER_2018: [(i32, f64, f64); 54] = [
    (1965, 23.8, 49.5), (1966, 20.3, -3.4), (1967, 11.0, 13.3), (1968, 19.0, 77.8),
    (1969, 16.2, 19.4), (1970, 12.0, -4.6), (1971, 16.4, 80.5), (1972, 21.7, 8.1),
    (1973, 4.7, -2.5), (1974, 5.5, -48.7), (1975, 21.9, 2.5), (1976, 59.3, 129.3),
    (1977, 31.9, 46.8), (1978, 24.0, 14.5), (1979, 35.7, 102.5), (1980, 19.3, 32.8),
    (1981, 31.4, 31.8), (1982, 40.0, 38.4), (1983, 32.3, 69.0), (1984, 13.6, -2.7),
    (1985, 48.2, 93.7), (1986, 26.1, 14.2), (1987, 19.5, 4.6), (1988, 20.1, 59.3),
    (1989, 44.4, 84.6), (1990, 7.4, -23.1), (1991, 39.6, 35.6), (1992, 20.3, 29.8),
    (1993, 14.3, 38.9), (1994, 13.9, 25.0), (1995, 43.1, 57.4), (1996, 31.8, 6.2),
    (1997, 34.1, 34.9), (1998, 48.3, 52.2), (1999, 0.5, -19.9), (2000, 6.5, 26.6),
    (2001, -6.2, 6.5), (2002, 10.0, -3.8), (2003, 21.0, 15.8), (2004, 10.5, 4.3),
    (2005, 6.4, 0.8), (2006, 18.4, 24.1), (2007, 11.0, 28.7), (2008, -9.6, -31.8),
    (2009, 19.8, 2.7), (2010, 13.0, 21.4), (2011, 4.6, -4.7), (2012, 14.4, 16.8),
    (2013, 18.2, 32.7), (2014, 8.3, 27.0), (2015, 6.4, -12.5), (2016, 10.7, 23.4),
    (2017, 23.0, 21.9), (2018, 0.4, 2.8),
];

/// The same table's "Overall Gain – 1964-2018", in percent: what the yearly
/// changes compound to, a check on transcribing them.
pub const OVERALL_BOOK_GAIN_PCT: f64 = 1_091_899.0;
pub const OVERALL_MARKET_GAIN_PCT: f64 = 2_472_627.0;

/// A year end from a 10-K: Berkshire's shareholders' equity (millions of
/// dollars) and the shares on the cover in Class B equivalents (one A is
/// 1,500 B), as Postgres holds them from the XBRL.
pub struct YearEnd {
    pub date: NaiveDate,
    pub equity: f64,
    pub b_equivalent_shares: f64,
    pub accession: &'static str,
}

const fn year_end(y: i32, equity: f64, b_equivalent_shares: f64, accession: &'static str) -> YearEnd {
    YearEnd { date: NaiveDate::from_ymd_opt(y, 12, 31).unwrap(), equity, b_equivalent_shares, accession }
}

/// 2018 (to join the letter's series) through 2025.
pub const FROM_10K: [YearEnd; 8] = [
    year_end(2018, 348_703.0, 2_461_462_331.0, "0001193125-19-048926"),
    year_end(2019, 424_791.0, 2_436_588_959.0, "0001564590-20-005874"),
    year_end(2020, 443_164.0, 2_297_227_609.0, "0001564590-21-009611"),
    year_end(2021, 506_199.0, 2_214_212_161.0, "0001564590-22-007322"),
    year_end(2022, 472_360.0, 2_187_352_743.0, "0000950170-23-004451"),
    year_end(2023, 561_273.0, 2_160_732_008.0, "0000950170-24-019719"),
    year_end(2024, 649_368.0, 2_157_335_139.0, "0000950170-25-025210"),
    year_end(2025, 717_419.0, 2_157_335_139.0, "0001193125-26-083899"),
];

impl YearEnd {
    /// Book value per Class A equivalent share.
    pub fn book_per_a_share(&self) -> f64 {
        self.equity * 1e6 / (self.b_equivalent_shares / 1500.0)
    }
}

/// Book value per Class A share at each year end, 1964 to 2025: the letter's
/// changes compounded from 1964's $19.46 through 2018, then the 10-Ks'
/// changes from there (the 10-K levels differ from the letter's by the
/// cover's share count, a fraction of a percent, so the changes are joined,
/// not the levels).
pub fn book_per_share() -> Vec<(i32, f64)> {
    let mut out = vec![(1964, BOOK_1964)];
    for (year, book, _) in LETTER_2018 {
        let last = out.last().unwrap().1;
        out.push((year, last * (1.0 + book / 100.0)));
    }
    let base = FROM_10K[0].book_per_a_share();
    let joined = out.last().unwrap().1;
    for y in &FROM_10K[1..] {
        out.push((y.date.year_ce().1 as i32, joined * y.book_per_a_share() / base));
    }
    out
}

/// Market value per share at each year end, 1964–2018, relative to 2018's:
/// the letter's changes compounded backward, to be scaled by an actual
/// 2018 year-end price.
pub fn market_relative_to_2018() -> Vec<(i32, f64)> {
    let mut out = vec![(2018, 1.0)];
    for (year, _, market) in LETTER_2018.iter().rev() {
        let next = out.last().unwrap().1;
        out.push((year - 1, next / (1.0 + market / 100.0)));
    }
    out.reverse();
    out
}

use chrono::Datelike;

// The statements as Berkshire presents them (10-K for 2025,
// 0001193125-26-083899): unclassified and single-step, in two segments,
// "Insurance and Other" and "Railroad, Utilities and Energy", each with its
// own assets, liabilities, revenues and costs. Income taxes, principally
// deferred, and shareholders' equity are the whole company's. A segment is
// a struct; the company composes them, so every total is computed from its
// parts.

/// Insurance and Other: the insurers (GEICO, General Re, the reinsurance
/// group), the portfolio of stocks and Treasury Bills their float buys, and
/// the manufacturing, service and retailing businesses.
pub struct InsuranceAndOtherBalanceSheet {
    pub cash_and_cash_equivalents: f64,
    pub short_term_investments_in_treasury_bills: f64,
    pub investments_in_fixed_maturity_securities: f64,
    pub investments_in_equity_securities: f64,
    pub equity_method_investments: f64,
    pub loans_and_finance_receivables: f64,
    pub other_receivables: f64,
    pub inventories: f64,
    pub property_plant_and_equipment: f64,
    pub equipment_held_for_lease: f64,
    pub goodwill: f64,
    pub other_intangible_assets: f64,
    pub deferred_charges_retroactive_reinsurance: f64,
    pub other_assets: f64,

    /// The float: claims owed and not yet paid, invested meanwhile.
    pub unpaid_losses_and_loss_adjustment_expenses: f64,
    pub unpaid_losses_and_loss_adjustment_expenses_retroactive_reinsurance: f64,
    pub unearned_insurance_premiums: f64,
    pub life_annuity_and_health_insurance_benefits: f64,
    pub other_insurance_policyholder_liabilities: f64,
    pub accounts_payable_accruals_and_other_liabilities: f64,
    pub payable_for_purchase_of_treasury_bills: f64,
    pub aircraft_repurchase_liabilities_and_unearned_lease_revenues: f64,
    pub notes_payable_and_other_borrowings: f64,
}

impl InsuranceAndOtherBalanceSheet {
    pub fn total_assets(&self) -> f64 {
        self.cash_and_cash_equivalents
            + self.short_term_investments_in_treasury_bills
            + self.investments_in_fixed_maturity_securities
            + self.investments_in_equity_securities
            + self.equity_method_investments
            + self.loans_and_finance_receivables
            + self.other_receivables
            + self.inventories
            + self.property_plant_and_equipment
            + self.equipment_held_for_lease
            + self.goodwill
            + self.other_intangible_assets
            + self.deferred_charges_retroactive_reinsurance
            + self.other_assets
    }
    /// Excluding income taxes, which the company reports as a whole.
    pub fn total_liabilities(&self) -> f64 {
        self.unpaid_losses_and_loss_adjustment_expenses
            + self.unpaid_losses_and_loss_adjustment_expenses_retroactive_reinsurance
            + self.unearned_insurance_premiums
            + self.life_annuity_and_health_insurance_benefits
            + self.other_insurance_policyholder_liabilities
            + self.accounts_payable_accruals_and_other_liabilities
            + self.payable_for_purchase_of_treasury_bills
            + self.aircraft_repurchase_liabilities_and_unearned_lease_revenues
            + self.notes_payable_and_other_borrowings
    }
    /// Insurance float, in Buffett's sense: what policyholders' money the
    /// insurers hold, less the deferred charges on retroactive reinsurance.
    pub fn float(&self) -> f64 {
        self.unpaid_losses_and_loss_adjustment_expenses
            + self.unpaid_losses_and_loss_adjustment_expenses_retroactive_reinsurance
            + self.unearned_insurance_premiums
            + self.life_annuity_and_health_insurance_benefits
            + self.other_insurance_policyholder_liabilities
            - self.deferred_charges_retroactive_reinsurance
    }
}

/// Railroad, Utilities and Energy: BNSF and Berkshire Hathaway Energy,
/// regulated and capital-heavy, financed largely with their own debt.
pub struct RailroadUtilitiesAndEnergyBalanceSheet {
    pub cash_and_cash_equivalents: f64,
    pub receivables: f64,
    pub property_plant_and_equipment: f64,
    pub goodwill: f64,
    pub regulatory_assets: f64,
    pub other_assets: f64,

    pub accounts_payable_accruals_and_other_liabilities: f64,
    pub regulatory_liabilities: f64,
    pub notes_payable_and_other_borrowings: f64,
}

impl RailroadUtilitiesAndEnergyBalanceSheet {
    pub fn total_assets(&self) -> f64 {
        self.cash_and_cash_equivalents
            + self.receivables
            + self.property_plant_and_equipment
            + self.goodwill
            + self.regulatory_assets
            + self.other_assets
    }
    /// Excluding income taxes, which the company reports as a whole.
    pub fn total_liabilities(&self) -> f64 {
        self.accounts_payable_accruals_and_other_liabilities
            + self.regulatory_liabilities
            + self.notes_payable_and_other_borrowings
    }
}

pub struct BalanceSheet {
    pub date: NaiveDate,
    pub insurance_and_other: InsuranceAndOtherBalanceSheet,
    pub railroad_utilities_and_energy: RailroadUtilitiesAndEnergyBalanceSheet,

    /// Mostly the tax deferred on unrealized gains in the stock portfolio:
    /// owed only when the stocks are sold, and interest free until then.
    pub income_taxes_principally_deferred: f64,

    pub common_stock_at_par_value: f64,
    pub capital_in_excess_of_par_value: f64,
    pub accumulated_other_comprehensive_income: f64,
    pub retained_earnings: f64,
    pub treasury_stock_at_cost: f64,
    pub noncontrolling_interests: f64,
}

impl Statement for BalanceSheet {
    fn ticker(&self) -> Ticker {
        Ticker::BRK
    }
    fn period(&self) -> Period {
        Period::PointInTime
    }
}

impl BS for BalanceSheet {
    fn total_assets(&self) -> f64 {
        self.insurance_and_other.total_assets() + self.railroad_utilities_and_energy.total_assets()
    }
    fn total_liabilities(&self) -> f64 {
        self.insurance_and_other.total_liabilities()
            + self.railroad_utilities_and_energy.total_liabilities()
            + self.income_taxes_principally_deferred
    }
}

impl BalanceSheet {
    pub fn cash_and_cash_equivalents(&self) -> f64 {
        self.insurance_and_other.cash_and_cash_equivalents + self.railroad_utilities_and_energy.cash_and_cash_equivalents
    }
    pub fn goodwill(&self) -> f64 {
        self.insurance_and_other.goodwill + self.railroad_utilities_and_energy.goodwill
    }
    pub fn notes_payable_and_other_borrowings(&self) -> f64 {
        self.insurance_and_other.notes_payable_and_other_borrowings
            + self.railroad_utilities_and_energy.notes_payable_and_other_borrowings
    }
    pub fn berkshire_shareholders_equity(&self) -> f64 {
        self.common_stock_at_par_value
            + self.capital_in_excess_of_par_value
            + self.accumulated_other_comprehensive_income
            + self.retained_earnings
            + self.treasury_stock_at_cost
    }
    pub fn total_shareholders_equity(&self) -> f64 {
        self.berkshire_shareholders_equity() + self.noncontrolling_interests
    }
    /// Assets less liabilities less equity: zero when every line was read right.
    pub fn imbalance(&self) -> f64 {
        self.total_assets() - self.total_liabilities() - self.total_shareholders_equity()
    }
}

pub struct InsuranceAndOtherEarnings {
    pub insurance_premiums_earned: f64,
    pub sales_and_service_revenues: f64,
    pub leasing_revenues: f64,
    pub interest_dividend_and_other_investment_income: f64,

    pub insurance_losses_and_loss_adjustment_expenses: f64,
    pub life_annuity_and_health_insurance_benefits: f64,
    pub insurance_underwriting_expenses: f64,
    pub cost_of_sales_and_services: f64,
    pub cost_of_leasing: f64,
    pub selling_general_and_administrative_expenses: f64,
    pub interest_expense: f64,
}

impl InsuranceAndOtherEarnings {
    pub fn total_revenues(&self) -> f64 {
        self.insurance_premiums_earned
            + self.sales_and_service_revenues
            + self.leasing_revenues
            + self.interest_dividend_and_other_investment_income
    }
    pub fn total_costs_and_expenses(&self) -> f64 {
        self.insurance_losses_and_loss_adjustment_expenses
            + self.life_annuity_and_health_insurance_benefits
            + self.insurance_underwriting_expenses
            + self.cost_of_sales_and_services
            + self.cost_of_leasing
            + self.selling_general_and_administrative_expenses
            + self.interest_expense
    }
    /// Underwriting profit before tax: premiums earned less losses and
    /// expenses. Positive, the float costs less than nothing.
    pub fn underwriting_profit(&self) -> f64 {
        self.insurance_premiums_earned
            - self.insurance_losses_and_loss_adjustment_expenses
            - self.life_annuity_and_health_insurance_benefits
            - self.insurance_underwriting_expenses
    }
}

pub struct RailroadUtilitiesAndEnergyEarnings {
    pub railroad_transportation_revenues: f64,
    pub utility_and_energy_operating_revenues: f64,
    pub service_revenues_and_other_income: f64,

    pub freight_rail_transportation_expenses: f64,
    pub utilities_and_energy_cost_of_sales_and_other_expenses: f64,
    pub other_expenses: f64,
    pub interest_expense: f64,
}

impl RailroadUtilitiesAndEnergyEarnings {
    pub fn total_revenues(&self) -> f64 {
        self.railroad_transportation_revenues
            + self.utility_and_energy_operating_revenues
            + self.service_revenues_and_other_income
    }
    pub fn total_costs_and_expenses(&self) -> f64 {
        self.freight_rail_transportation_expenses
            + self.utilities_and_energy_cost_of_sales_and_other_expenses
            + self.other_expenses
            + self.interest_expense
    }
}

pub struct IncomeStatement {
    pub date: NaiveDate,
    pub p: Period,
    pub insurance_and_other: InsuranceAndOtherEarnings,
    pub railroad_utilities_and_energy: RailroadUtilitiesAndEnergyEarnings,

    /// Realized and, since 2018, unrealized gains on the stock portfolio:
    /// the swing that makes GAAP net income useless as a measure of
    /// Berkshire's year (Buffett's word).
    pub investment_gains: f64,
    pub equity_method_earnings: f64,
    pub income_tax_expense: f64,
    pub earnings_attributable_to_noncontrolling_interests: f64,
}

impl Statement for IncomeStatement {
    fn ticker(&self) -> Ticker {
        Ticker::BRK
    }
    fn period(&self) -> Period {
        self.p
    }
}

impl IS for IncomeStatement {
    fn income_before_income_tax(&self) -> f64 {
        self.total_revenues() + self.investment_gains - self.total_costs_and_expenses() + self.equity_method_earnings
    }
    /// Net earnings, noncontrolling interests included (the statement's
    /// "Net earnings" line).
    fn net_income(&self) -> f64 {
        self.income_before_income_tax() - self.income_tax_expense
    }
}

impl IncomeStatement {
    pub fn total_revenues(&self) -> f64 {
        self.insurance_and_other.total_revenues() + self.railroad_utilities_and_energy.total_revenues()
    }
    pub fn total_costs_and_expenses(&self) -> f64 {
        self.insurance_and_other.total_costs_and_expenses() + self.railroad_utilities_and_energy.total_costs_and_expenses()
    }
    pub fn interest_expense(&self) -> f64 {
        self.insurance_and_other.interest_expense + self.railroad_utilities_and_energy.interest_expense
    }
    pub fn net_earnings_attributable_to_berkshire(&self) -> f64 {
        self.net_income() - self.earnings_attributable_to_noncontrolling_interests
    }
    /// Before investment gains, roughly what Buffett calls operating
    /// earnings, but before tax: the businesses' year without the market's.
    pub fn earnings_before_investment_gains_and_taxes(&self) -> f64 {
        self.income_before_income_tax() - self.investment_gains
    }
}

pub struct CashFlowStatement {
    pub date: NaiveDate,
    pub p: Period,

    pub net_earnings: f64,
    pub investment_gains: f64,
    pub depreciation_and_amortization: f64,
    pub discount_accretion_on_investments: f64,
    pub equity_method_investment_impairment_losses: f64,
    pub other_adjustments: f64,
    pub change_in_unpaid_losses_and_loss_adjustment_expenses: f64,
    pub change_in_deferred_charges_retroactive_reinsurance: f64,
    pub change_in_unearned_insurance_premiums: f64,
    pub change_in_receivables_and_originated_loans: f64,
    pub change_in_other_assets: f64,
    pub change_in_other_liabilities: f64,
    pub change_in_income_taxes: f64,

    pub purchases_of_equity_securities: f64,
    pub sales_of_equity_securities: f64,
    pub purchases_of_treasury_bills_and_fixed_maturity_securities: f64,
    pub sales_of_treasury_bills_and_fixed_maturity_securities: f64,
    pub redemptions_and_maturities_of_treasury_bills_and_fixed_maturity_securities: f64,
    pub acquisitions_of_businesses_net_of_cash_acquired: f64,
    pub purchases_of_property_plant_and_equipment_and_equipment_held_for_lease: f64,
    pub other_investing: f64,

    pub insurance_and_other_proceeds_from_borrowings: f64,
    pub insurance_and_other_repayments_of_borrowings: f64,
    pub railroad_utilities_and_energy_proceeds_from_borrowings: f64,
    pub railroad_utilities_and_energy_repayments_of_borrowings: f64,
    pub changes_in_short_term_borrowings_net: f64,
    pub acquisitions_of_treasury_stock: f64,
    pub other_financing: f64,

    pub effects_of_foreign_currency_exchange_rate_changes: f64,
}

impl Statement for CashFlowStatement {
    fn ticker(&self) -> Ticker {
        Ticker::BRK
    }
    fn period(&self) -> Period {
        self.p
    }
}

impl CFS for CashFlowStatement {
    fn net_cash_from_operating_activities(&self) -> f64 {
        self.net_earnings
            + self.investment_gains
            + self.depreciation_and_amortization
            + self.discount_accretion_on_investments
            + self.equity_method_investment_impairment_losses
            + self.other_adjustments
            + self.change_in_unpaid_losses_and_loss_adjustment_expenses
            + self.change_in_deferred_charges_retroactive_reinsurance
            + self.change_in_unearned_insurance_premiums
            + self.change_in_receivables_and_originated_loans
            + self.change_in_other_assets
            + self.change_in_other_liabilities
            + self.change_in_income_taxes
    }
    fn net_cash_from_investing_activities(&self) -> f64 {
        self.purchases_of_equity_securities
            + self.sales_of_equity_securities
            + self.purchases_of_treasury_bills_and_fixed_maturity_securities
            + self.sales_of_treasury_bills_and_fixed_maturity_securities
            + self.redemptions_and_maturities_of_treasury_bills_and_fixed_maturity_securities
            + self.acquisitions_of_businesses_net_of_cash_acquired
            + self.purchases_of_property_plant_and_equipment_and_equipment_held_for_lease
            + self.other_investing
    }
    fn net_cash_from_financing_activities(&self) -> f64 {
        self.insurance_and_other_proceeds_from_borrowings
            + self.insurance_and_other_repayments_of_borrowings
            + self.railroad_utilities_and_energy_proceeds_from_borrowings
            + self.railroad_utilities_and_energy_repayments_of_borrowings
            + self.changes_in_short_term_borrowings_net
            + self.acquisitions_of_treasury_stock
            + self.other_financing
    }
    fn change_in_cash(&self) -> f64 {
        self.net_cash_from_operating_activities()
            + self.net_cash_from_investing_activities()
            + self.net_cash_from_financing_activities()
            + self.effects_of_foreign_currency_exchange_rate_changes
    }
}

impl_reported! {
    BalanceSheet,
    [
        CashAndCashEquivalents => method cash_and_cash_equivalents,
        Goodwill => method goodwill,
        TotalAssets => method total_assets,
        TotalLiabilities => method total_liabilities,
        StockholdersEquity => method berkshire_shareholders_equity,
    ]
}

impl_reported! {
    IncomeStatement,
    [
        Revenue => method total_revenues,
        InterestExpense => method interest_expense,
        IncomeBeforeIncomeTax => method income_before_income_tax,
        IncomeTaxExpense => field income_tax_expense,
        NetIncome => method net_income,
    ]
}

impl_reported! {
    CashFlowStatement,
    [
        DepreciationAndAmoritization => field depreciation_and_amortization,
        NetCashProvidedByOperatingActivities => method net_cash_from_operating_activities,
        AcquisitionsNetOfCashAcquired => field acquisitions_of_businesses_net_of_cash_acquired,
        PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets => field purchases_of_property_plant_and_equipment_and_equipment_held_for_lease,
        NetCashUsedInInvestingActivities => method net_cash_from_investing_activities,
        PaymentsRelatedToRepurchasesOfCommonStock => field acquisitions_of_treasury_stock,
        NetCashUsedInFinancingActivities => method net_cash_from_financing_activities,
        ChangeInCashAndCashEquivalents => method change_in_cash,
    ]
}

const M: f64 = 1e6;

/// December 31, 2025, from the 10-K (dollars; the filing states millions).
pub const BALANCE_SHEET_2025: BalanceSheet = BalanceSheet {
    date: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
    insurance_and_other: InsuranceAndOtherBalanceSheet {
        cash_and_cash_equivalents: 47_719.0 * M,
        short_term_investments_in_treasury_bills: 321_434.0 * M,
        investments_in_fixed_maturity_securities: 17_816.0 * M,
        investments_in_equity_securities: 297_778.0 * M,
        equity_method_investments: 19_978.0 * M,
        loans_and_finance_receivables: 29_836.0 * M,
        other_receivables: 44_331.0 * M,
        inventories: 24_424.0 * M,
        property_plant_and_equipment: 31_885.0 * M,
        equipment_held_for_lease: 18_535.0 * M,
        goodwill: 55_945.0 * M,
        other_intangible_assets: 33_802.0 * M,
        deferred_charges_retroactive_reinsurance: 8_104.0 * M,
        other_assets: 24_413.0 * M,
        unpaid_losses_and_loss_adjustment_expenses: 120_713.0 * M,
        unpaid_losses_and_loss_adjustment_expenses_retroactive_reinsurance: 31_048.0 * M,
        unearned_insurance_premiums: 31_339.0 * M,
        life_annuity_and_health_insurance_benefits: 17_890.0 * M,
        other_insurance_policyholder_liabilities: 10_312.0 * M,
        accounts_payable_accruals_and_other_liabilities: 38_019.0 * M,
        payable_for_purchase_of_treasury_bills: 167.0 * M,
        aircraft_repurchase_liabilities_and_unearned_lease_revenues: 10_686.0 * M,
        notes_payable_and_other_borrowings: 45_763.0 * M,
    },
    railroad_utilities_and_energy: RailroadUtilitiesAndEnergyBalanceSheet {
        cash_and_cash_equivalents: 4_158.0 * M,
        receivables: 4_387.0 * M,
        property_plant_and_equipment: 184_740.0 * M,
        goodwill: 27_129.0 * M,
        regulatory_assets: 4_821.0 * M,
        other_assets: 20_941.0 * M,
        accounts_payable_accruals_and_other_liabilities: 19_250.0 * M,
        regulatory_liabilities: 7_013.0 * M,
        notes_payable_and_other_borrowings: 83_318.0 * M,
    },
    income_taxes_principally_deferred: 86_955.0 * M,
    common_stock_at_par_value: 8.0 * M,
    capital_in_excess_of_par_value: 35_612.0 * M,
    accumulated_other_comprehensive_income: -2_448.0 * M,
    retained_earnings: 763_186.0 * M,
    treasury_stock_at_cost: -78_939.0 * M,
    noncontrolling_interests: 2_284.0 * M,
};

/// The year 2025, from the 10-K.
pub const INCOME_STATEMENT_2025: IncomeStatement = IncomeStatement {
    date: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
    p: Period::TwelveMonths,
    insurance_and_other: InsuranceAndOtherEarnings {
        insurance_premiums_earned: 88_902.0 * M,
        sales_and_service_revenues: 199_524.0 * M,
        leasing_revenues: 10_034.0 * M,
        interest_dividend_and_other_investment_income: 23_261.0 * M,
        insurance_losses_and_loss_adjustment_expenses: 57_307.0 * M,
        life_annuity_and_health_insurance_benefits: 4_379.0 * M,
        insurance_underwriting_expenses: 17_756.0 * M,
        cost_of_sales_and_services: 160_062.0 * M,
        cost_of_leasing: 7_685.0 * M,
        selling_general_and_administrative_expenses: 29_735.0 * M,
        interest_expense: 1_329.0 * M,
    },
    railroad_utilities_and_energy: RailroadUtilitiesAndEnergyEarnings {
        railroad_transportation_revenues: 23_330.0 * M,
        utility_and_energy_operating_revenues: 21_856.0 * M,
        service_revenues_and_other_income: 4_537.0 * M,
        freight_rail_transportation_expenses: 15_366.0 * M,
        utilities_and_energy_cost_of_sales_and_other_expenses: 16_959.0 * M,
        other_expenses: 4_155.0 * M,
        interest_expense: 3_740.0 * M,
    },
    investment_gains: 39_078.0 * M,
    equity_method_earnings: -9_590.0 * M,
    income_tax_expense: 15_199.0 * M,
    earnings_attributable_to_noncontrolling_interests: 292.0 * M,
};

/// The year 2025, from the 10-K.
pub const CASH_FLOW_STATEMENT_2025: CashFlowStatement = CashFlowStatement {
    date: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
    p: Period::TwelveMonths,
    net_earnings: 67_260.0 * M,
    investment_gains: -39_078.0 * M,
    depreciation_and_amortization: 13_476.0 * M,
    discount_accretion_on_investments: -11_964.0 * M,
    equity_method_investment_impairment_losses: 10_681.0 * M,
    other_adjustments: 3_239.0 * M,
    change_in_unpaid_losses_and_loss_adjustment_expenses: 3_088.0 * M,
    change_in_deferred_charges_retroactive_reinsurance: 693.0 * M,
    change_in_unearned_insurance_premiums: 432.0 * M,
    change_in_receivables_and_originated_loans: -2_426.0 * M,
    change_in_other_assets: -861.0 * M,
    change_in_other_liabilities: 429.0 * M,
    change_in_income_taxes: 1_000.0 * M,
    purchases_of_equity_securities: -16_923.0 * M,
    sales_of_equity_securities: 30_686.0 * M,
    purchases_of_treasury_bills_and_fixed_maturity_securities: -586_129.0 * M,
    sales_of_treasury_bills_and_fixed_maturity_securities: 44_769.0 * M,
    redemptions_and_maturities_of_treasury_bills_and_fixed_maturity_securities: 503_954.0 * M,
    acquisitions_of_businesses_net_of_cash_acquired: -1_074.0 * M,
    purchases_of_property_plant_and_equipment_and_equipment_held_for_lease: -20_927.0 * M,
    other_investing: 1_157.0 * M,
    insurance_and_other_proceeds_from_borrowings: 3_071.0 * M,
    insurance_and_other_repayments_of_borrowings: -3_050.0 * M,
    railroad_utilities_and_energy_proceeds_from_borrowings: 6_101.0 * M,
    railroad_utilities_and_energy_repayments_of_borrowings: -3_974.0 * M,
    changes_in_short_term_borrowings_net: 878.0 * M,
    acquisitions_of_treasury_stock: 0.0,
    other_financing: -793.0 * M,
    effects_of_foreign_currency_exchange_rate_changes: 478.0 * M,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// The computed totals equal the 10-K's printed ones.
    #[test]
    fn statements_match_the_filing() {
        let b = &BALANCE_SHEET_2025;
        assert_eq!(b.insurance_and_other.total_assets(), 976_000.0 * M);
        assert_eq!(b.insurance_and_other.total_liabilities(), 305_937.0 * M);
        assert_eq!(b.railroad_utilities_and_energy.total_assets(), 246_176.0 * M);
        assert_eq!(b.railroad_utilities_and_energy.total_liabilities(), 109_581.0 * M);
        assert_eq!(b.total_assets(), 1_222_176.0 * M);
        assert_eq!(b.total_liabilities(), 502_473.0 * M);
        assert_eq!(b.berkshire_shareholders_equity(), 717_419.0 * M);
        assert_eq!(b.total_shareholders_equity(), 719_703.0 * M);
        assert_eq!(b.imbalance(), 0.0);

        let i = &INCOME_STATEMENT_2025;
        assert_eq!(i.insurance_and_other.total_revenues(), 321_721.0 * M);
        assert_eq!(i.insurance_and_other.total_costs_and_expenses(), 278_253.0 * M);
        assert_eq!(i.railroad_utilities_and_energy.total_revenues(), 49_723.0 * M);
        assert_eq!(i.railroad_utilities_and_energy.total_costs_and_expenses(), 40_220.0 * M);
        assert_eq!(i.total_revenues(), 371_444.0 * M);
        assert_eq!(i.income_before_income_tax(), 82_459.0 * M);
        assert_eq!(i.net_income(), 67_260.0 * M);
        assert_eq!(i.net_earnings_attributable_to_berkshire(), 66_968.0 * M);

        let c = &CASH_FLOW_STATEMENT_2025;
        assert_eq!(c.net_cash_from_operating_activities(), 45_969.0 * M);
        assert_eq!(c.net_cash_from_investing_activities(), -44_487.0 * M);
        assert_eq!(c.net_cash_from_financing_activities(), 2_233.0 * M);
        assert_eq!(c.change_in_cash(), 4_193.0 * M);
        // The cash flow statement starts from the income statement's net earnings.
        assert_eq!(c.net_earnings, i.net_income());
    }

    /// Where the generic reader and these statements name the same item,
    /// they agree. (The reader's other Berkshire items are partial or
    /// misnamed: see README.md.)
    #[test]
    fn the_reader_agrees() {
        let path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../edgar/cache/1067983/000119312526083899"));
        if !path.exists() { return; }
        let mut r = crate::testing::filing_dir(path, Ticker::BRK).unwrap();
        let read: Vec<crate::item::Reported> = [
            crate::Reader::process_balance_sheet(&mut r).unwrap(),
            crate::Reader::process_income_statement(&mut r).unwrap(),
        ].concat();
        let mine = BALANCE_SHEET_2025.reported().into_iter().chain(INCOME_STATEMENT_2025.reported());
        let mut agreed = 0;
        for m in mine {
            if let Some(r) = read.iter().find(|r| r.item == m.item && r.date == m.date && r.p == m.p) {
                assert_eq!(r.val, m.val, "{:?}", m.item);
                agreed += 1;
            }
        }
        assert!(agreed >= 6, "only {} items agreed", agreed);
    }

    /// The transcribed yearly changes compound to the letter's overall gains
    /// (each change is rounded to a tenth of a percent, so to within 1%).
    #[test]
    fn the_table_compounds_to_its_totals() {
        let book: f64 = LETTER_2018.iter().map(|y| 1.0 + y.1 / 100.0).product();
        let market: f64 = LETTER_2018.iter().map(|y| 1.0 + y.2 / 100.0).product();
        assert!((book / (1.0 + OVERALL_BOOK_GAIN_PCT / 100.0) - 1.0).abs() < 0.01, "book {}", book);
        assert!((market / (1.0 + OVERALL_MARKET_GAIN_PCT / 100.0) - 1.0).abs() < 0.01, "market {}", market);
    }

    /// The letter's 2018 book value and the 10-K's agree to within the
    /// difference of share counts (year end against the cover).
    #[test]
    fn letter_and_10k_agree_in_2018() {
        let letter = book_per_share().iter().find(|y| y.0 == 2018).unwrap().1;
        let filed = FROM_10K[0].book_per_a_share();
        assert!((letter / filed - 1.0).abs() < 0.01, "letter {} 10-K {}", letter, filed);
    }
}
