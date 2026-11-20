uniffi::setup_scaffolding!();

pub mod date;
pub mod item;
pub mod xlsx;
pub mod pdf;
pub mod filing;
pub mod edgar;
pub mod nvda;
pub mod tsla;
pub mod sheet_info;

use std::str::FromStr;

use crate::item::Item;
use crate::item::Reported;
use crate::nvda::BalanceSheet as NVDABalanceSheet;
use crate::tsla::BalanceSheet as TSLABalanceSheet;
use crate::nvda::IncomeStatement as NVDAIncomeStatement;
use crate::tsla::IncomeStatement as TSLAIncomeStatement;

use chrono::NaiveDate;

macro_rules! count_items {
    () => { 0 };
    ($head:ident $($tail:ident)*) => { 1 + count_items!($($tail)*) };
}

/// The currency a value is reported in. Filers outside the US report in their
/// own currency (TSMC in New Taiwan dollars), sometimes with a US dollar
/// convenience translation of the latest year alongside.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
pub enum Currency {
    USD,
    TWD,
    EUR,
    GBP,
    JPY,
    CNY,
}

impl FromStr for Currency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USD" => Ok(Currency::USD),
            "TWD" => Ok(Currency::TWD),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            "CNY" => Ok(Currency::CNY),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Ticker {
    NVDA,
    TSLA,
    AAPL,
    BRK,
    TSM,
}

impl Ticker {
    /// The symbol in the SEC's ticker map. Berkshire Hathaway trades as two
    /// share classes, listed separately; both map to the same company.
    pub fn sec_symbol(&self) -> &'static str {
        match self {
            Ticker::NVDA => "NVDA",
            Ticker::TSLA => "TSLA",
            Ticker::AAPL => "AAPL",
            Ticker::BRK => "BRK-B",
            Ticker::TSM => "TSM",
        }
    }
}

impl FromStr for Ticker {
    type Err = TickerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NVDA" => Ok(Ticker::NVDA),
            "TSLA" => Ok(Ticker::TSLA),
            "AAPL" => Ok(Ticker::AAPL),
            "BRK" => Ok(Ticker::BRK),
            "TSM" => Ok(Ticker::TSM),
            _ => Err(TickerError::TickerNotFound),
        }
    }
}

pub enum TickerError {
    TickerNotFound,
}

impl std::fmt::Display for Ticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ticker::NVDA => write!(f, "NVDA"),
            Ticker::TSLA => write!(f, "TSLA"),
            Ticker::AAPL => write!(f, "AAPL"),
            Ticker::BRK => write!(f, "BRK"),
            Ticker::TSM => write!(f, "TSM"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, uniffi::Enum)]
pub enum Period {
    ThreeMonths,
    SixMonths,
    NineMonths,
    TwelveMonths,
    PointInTime,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Period::ThreeMonths => "3m",
            Period::SixMonths => "6m",
            Period::NineMonths => "9m",
            Period::TwelveMonths => "12m",
            Period::PointInTime => "pit",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Period {
    type Err = PeriodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        let lower = trimmed.to_lowercase();
        // EDGAR's Financial_Report.xlsx writes "3 Months Ended", company reports write "Three Months Ended".
        if lower.contains("three months") || lower.contains("3 months") { Ok(Period::ThreeMonths) }
        else if lower.contains("six months") || lower.contains("6 months") { Ok(Period::SixMonths) }
        else if lower.contains("nine months") || lower.contains("9 months") { Ok(Period::NineMonths) }
        else if lower.contains("year ended") || lower.contains("annual") || lower.contains("twelve months") || lower.contains("12 months") { Ok(Period::TwelveMonths) }
        else { Err(PeriodError::InvalidPeriod) }
    }
}

#[derive(Debug, uniffi::Error)]
pub enum PeriodError {
    InvalidPeriod,
}

pub trait Statement {
    fn ticker(&self) -> Ticker;
    fn period(&self) -> Period;
}

pub trait Reader {
    fn process_balance_sheet(&mut self) -> Result<Vec<Reported>, Box<dyn std::error::Error>>;
    fn process_income_statement(&mut self) -> Result<Vec<Reported>, Box<dyn std::error::Error>>;
}

macro_rules! impl_reported {
    ($name:ident, [ $($variant:ident => $kind:ident $source:ident),* $(,)? ]) => {
        impl $name {
            pub fn reported(&self) -> [Reported; count_items!($($variant)*)] {
                [
                    $(
                        Reported {
                            ticker: Statement::ticker(self),
                            date: self.date,
                            p: self.period(),
                            item: Item::$variant,
                            currency: Currency::USD,
                            val: impl_reported!(@val self, $kind $source),
                        },
                    )*
                ]
            }
        }
    };

    (@val $self:ident, field $source:ident) => { $self.$source };
    (@val $self:ident, method $source:ident) => { $self.$source() };
}

pub trait BalanceSheet {
    fn total_current_assets(&self) -> f64;
    fn total_assets(&self) -> f64;
    fn total_current_liabilities(&self) -> f64;
    fn total_liabilities(&self) -> f64;
}

pub trait IncomeStatement {
    fn gross_profit(&self) -> f64;
    fn total_operating_expenses(&self) -> f64;
    fn operating_income(&self) -> f64 {
        self.gross_profit() - self.total_operating_expenses()
    }
    fn income_before_income_tax(&self) -> f64;
    fn net_income(&self) -> f64;
}

pub struct CashFlowStatement {
    pub ticker: Ticker,
    pub date: NaiveDate,
    pub p: Period,

    pub net_income: f64,

    pub stock_based_compensation_expense: f64,
    pub depreciation_and_amoritization: f64,
    pub deferred_income_taxes: f64,
    pub gains_on_non_marketable_equity_securities_and_publicly_held_equity_securities_net: f64,
    pub other: f64,

    pub accounts_receivable: f64,
    pub inventories: f64,
    pub prepaid_expenses_and_other_assets:f64,
    pub accounts_payable: f64,
    pub accrued_and_other_current_liabilities: f64,
    pub other_long_term_liabilities: f64,

    pub proceeds_from_maturities_of_marketable_securities: f64,
    pub proceeds_from_sales_of_marketable_securities: f64,
    pub proceeds_from_sales_of_non_marketable_equity_securities: f64,
    pub purchases_of_marketable_securities: f64,
    pub purchases_related_to_property_and_equipment_and_intangible_assets: f64,
    pub purchases_of_non_marketable_equity_securities: f64,
    pub acquisitions_net_of_cash_acquired: f64,

    pub proceeds_related_to_employee_stock_plans: f64,
    pub payments_related_to_repurchases_of_common_stock: f64,
    pub payments_related_to_employee_stock_plan_taxes: f64,
    pub dividends_paid: f64,
    pub principal_payments_on_property_and_equipment_and_intangible_assets: f64,
    pub repayment_of_debt: f64,

    pub cash_and_cash_equivalents_at_beginning_of_period: f64,

    pub cash_paid_for_income_taxes_net: f64,
}

impl Statement for CashFlowStatement {
    fn ticker(&self) -> Ticker {
        self.ticker
    }
    fn period(&self) -> Period {
        self.p
    }
}

impl CashFlowStatement {
    fn net_cash_provided_by_operating_activities(&self) -> f64 {
        self.net_income
            + self.stock_based_compensation_expense
            + self.depreciation_and_amoritization
            + self.deferred_income_taxes
            + self.gains_on_non_marketable_equity_securities_and_publicly_held_equity_securities_net
            + self.other

            + self.accounts_receivable
            + self.inventories
            + self.prepaid_expenses_and_other_assets
            + self.accounts_payable
            + self.accrued_and_other_current_liabilities
            + self.other_long_term_liabilities
    }
    fn net_cash_used_in_investing_activities(&self) -> f64 {
        self.proceeds_from_maturities_of_marketable_securities
            + self.proceeds_from_sales_of_marketable_securities
            + self.proceeds_from_sales_of_non_marketable_equity_securities
            + self.purchases_of_marketable_securities
            + self.purchases_related_to_property_and_equipment_and_intangible_assets
            + self.purchases_of_non_marketable_equity_securities
            + self.acquisitions_net_of_cash_acquired
    }
    fn net_cash_used_in_financing_activities(&self) -> f64 {
        self.proceeds_related_to_employee_stock_plans
            + self.payments_related_to_repurchases_of_common_stock
            + self.payments_related_to_employee_stock_plan_taxes
            + self.dividends_paid
            + self.principal_payments_on_property_and_equipment_and_intangible_assets
            + self.repayment_of_debt
    }
    fn change_in_cash_and_cash_equivalents(&self) -> f64 {
        self.net_cash_provided_by_operating_activities()
            + self.net_cash_used_in_investing_activities()
            + self.net_cash_used_in_financing_activities()
    }

//    fn cash_and_cash_equivalents_at_end_of_period(&self) -> f64 {
//        self.change_in_cash_and_cash_equivalents() + self.cash_and_cash_equivalents_at_beginning_of_period
//    }
}

impl_reported! {
    CashFlowStatement,
    [
        NetIncome => field net_income,
        StockBasedCompensationExpense => field stock_based_compensation_expense,
        DepreciationAndAmoritization => field depreciation_and_amoritization,
        DeferredIncomeTaxes => field deferred_income_taxes,
        GainsOnNonMarketableEquitySecuritiesAndPubliclyHeldSecuritiesNet => field gains_on_non_marketable_equity_securities_and_publicly_held_equity_securities_net,
        Other => field other,
        AccountsReceivable => field accounts_receivable,
        ChangeInInventories => field inventories,
        PrepaidExpensesAndOtherAssets => field prepaid_expenses_and_other_assets,
        ChangeInAccountsPayable => field accounts_payable,
        ChangeInAccruedAndOtherCurrentLiabilities => field accrued_and_other_current_liabilities,
        ChangeInOtherLongTermLiabilities => field other_long_term_liabilities,
        NetCashProvidedByOperatingActivities => method net_cash_provided_by_operating_activities,
        ProceedsFromMaturitiesOfMarketableSecurities => field proceeds_from_maturities_of_marketable_securities,
        ProceedsFromSalesOfMarketableSecurities => field proceeds_from_sales_of_marketable_securities,
        ProceedsFromSalesOfNonMarketableEquitySecurities => field proceeds_from_sales_of_non_marketable_equity_securities,
        PurchasesOfMarketableSecurities => field purchases_of_marketable_securities,
        PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets => field purchases_related_to_property_and_equipment_and_intangible_assets,
        PurchasesOfNonMarketableEquitySecurities => field purchases_of_non_marketable_equity_securities,
        AcquisitionsNetOfCashAcquired => field acquisitions_net_of_cash_acquired,
        NetCashUsedInInvestingActivities => method net_cash_used_in_investing_activities,
        NetCashUsedInFinancingActivities => method net_cash_used_in_financing_activities,
        ProceedsRelatedToEmployeeStockPlans =>  field proceeds_related_to_employee_stock_plans,
        PaymentsRelatedToRepurchasesOfCommonStock => field payments_related_to_repurchases_of_common_stock,
        PaymentsRelatedToEmployeeStockPlanTaxes => field payments_related_to_employee_stock_plan_taxes,
        DividendsPaid => field dividends_paid,
        PrincipalPaymentsOnPropertyAndEquipmentAndIntangibleAssets => field principal_payments_on_property_and_equipment_and_intangible_assets,
        RepaymentOfDebt => field repayment_of_debt,
        ChangeInCashAndCashEquivalents => method change_in_cash_and_cash_equivalents,
    ]
}

impl_reported! {
    NVDABalanceSheet,
    [
        CashAndCashEquivalents => field cash_and_cash_equivalents,
        MarketableSecurities => field marketable_securities,
        AccountsReceivableNet => field accounts_receivable_net,
        Inventories => field inventories,
        PrepaidExpensesAndOtherCurrentAssets => field prepaid_expenses_and_other_current_assets,
        TotalCurrentAssets => method total_current_assets,
        PropertyAndEquipmentNet => field property_and_equipment_net,
        OperatingLeaseAssets => field operating_lease_assets,
        Goodwill => field goodwill,
        IntangibleAssetsNet => field intangible_assets_net,
        DeferredIncomeTaxAssets => field deferred_income_tax_assets,
        OtherAssets => field other_assets,
        TotalAssets => method total_assets,
        AccountsPayable => field accounts_payable,
        AccruedAndOtherCurrentLiabilities => field accrued_and_other_current_liabilities,
        ShortTermDebt => field short_term_debt,
        TotalCurrentLiabilities => method total_current_liabilities,
        LongTermDebt => field long_term_debt,
        LongTermOperatingLeaseLiabilities => field long_term_operating_lease_liabilities,
        OtherLongTermLiabilities => field other_long_term_liabilities,
        TotalLiabilities => method total_liabilities,
    ]
}

impl_reported! {
    TSLABalanceSheet,
    [
        CashAndCashEquivalents => field cash_and_cash_equivalents,
        MarketableSecurities => field marketable_securities,
        AccountsReceivableNet => field accounts_receivable_net,
        Inventories => field inventories,
        PrepaidExpensesAndOtherCurrentAssets => field prepaid_expenses_and_other_current_assets,
        TotalCurrentAssets => method total_current_assets,
        OperatingLeaseVehiclesNet => field operating_lease_vehicles_net,
        SolarEnergySystemsNet => field solar_energy_systems_net,
        PropertyAndEquipmentNet => field property_and_equipment_net,
        OperatingLeaseAssets => field operating_lease_assets,
        Goodwill => field goodwill,
        DigitalAssets => field digital_assets,
        IntangibleAssetsNet => field intangible_assets_net,
        DeferredIncomeTaxAssets => field deferred_income_tax_assets,
        OtherAssets => field other_assets,
        TotalAssets => method total_assets,
        AccountsPayable => field accounts_payable,
        AccruedAndOtherCurrentLiabilities => field accrued_and_other_current_liabilities,
        DeferredRevenue => field deferred_revenue,
        ShortTermDebt => field current_portion_of_debt_and_finance_leases,
        TotalCurrentLiabilities => method total_current_liabilities,
        DebtAndFinanceLeasesNetOfCurrentPortion => field debt_and_finance_leases_net_of_current_portion,
        DeferredRevenueNetOfCurrentPortion => field deferred_revenue_net_of_current_portion,
        OtherLongTermLiabilities => field other_long_term_liabilities,
        TotalLiabilities => method total_liabilities,
    ]
}

impl_reported! {
    NVDAIncomeStatement,
    [
        Revenue => field revenue,
        CostOfRevenue => field cost_of_revenue,
        GrossProfit => method gross_profit,
        ResearchAndDevelopment => field research_and_development,
        SalesGeneralAndAdministrative => field sales_general_and_administrative,
        TotalOperatingExpenses => method total_operating_expenses,
        OperatingIncome => method operating_income,
        InterestIncome => field interest_income,
        InterestExpense => field interest_expense,
        OtherIncomeNet => field other_income_net,
        TotalOtherIncomeNet => method total_other_income_net,
        IncomeBeforeIncomeTax => method income_before_income_tax,
        IncomeTaxExpense => field income_tax_expense,
        NetIncome => method net_income,
    ]
}

impl_reported! {
    TSLAIncomeStatement,
    [
//        AutomotiveSalesRevenue => field automotive_sales_revenue,
//        AutomotiveRegulatoryCredits => field automotive_regulatory_credits,
//        AutomotiveLeasingRevenue => field automotive_leasing_revenue,
//        EnergyGenerationAndStorageRevenue => field energy_generation_and_storage_revenue,
//        ServicesAndOtherRevenue => field services_and_other_revenue,
        Revenue => method total_revenues,
//        AutomotiveSalesCost => field automotive_sales_cost_of_revenue,
//        AutomotiveLeasingCost => field automotive_leasing_cost_of_revenue,
//        EnergyGenerationAndStorageCost => field energy_generation_and_storage_cost_of_revenue,
//        ServicesAndOtherCost => field services_and_other_cost_of_revenue,
        CostOfRevenue => method total_cost_of_revenues,
        GrossProfit => method gross_profit,
        ResearchAndDevelopment => field research_and_development,
        SalesGeneralAndAdministrative => field selling_general_and_administrative,
        RestructuringAndOther => field restructuring_and_other,
        TotalOperatingExpenses => method total_operating_expenses,
        OperatingIncome => method operating_income,
        InterestIncome => field interest_income,
        InterestExpense => field interest_expense,
        OtherIncomeNet => field other_expense_income_net,
        IncomeBeforeIncomeTax => method income_before_income_tax,
        IncomeTaxExpense => field provision_for_income_taxes,
        NetIncome => method net_income,
    ]
}

/// Fill in three-month values that were not reported, from year-to-date
/// totals: Q2 = 6m - 3m, Q3 = 9m - 6m, Q4 = 12m - 9m, where the shorter
/// total ends about a quarter earlier. Annual reports only give twelve-month
/// figures (so every Q4 is derived), and 10-Q cash flow statements only give
/// year-to-date figures (so cash flow Q2-Q4 are derived). Reported
/// three-month values are never replaced. When a value was reported more than
/// once (restatements), the last one wins, matching how later writes
/// overwrite earlier ones downstream.
pub fn derive_quarters(items: &mut Vec<Reported>) {
    use std::collections::HashMap;

    let mut latest: HashMap<(Ticker, Item, Currency, Period, NaiveDate), f64> = HashMap::new();
    for it in items.iter() {
        latest.insert((it.ticker, it.item, it.currency, it.p, it.date), it.val);
    }

    let previous = |p: Period| match p {
        Period::SixMonths => Some(Period::ThreeMonths),
        Period::NineMonths => Some(Period::SixMonths),
        Period::TwelveMonths => Some(Period::NineMonths),
        _ => None,
    };

    let mut derived: HashMap<(Ticker, Item, Currency, NaiveDate), f64> = HashMap::new();
    for (&(ticker, item, currency, p, date), &total) in &latest {
        let Some(prev_p) = previous(p) else { continue };
        if latest.contains_key(&(ticker, item, currency, Period::ThreeMonths, date)) {
            continue;
        }
        let prev = latest.iter().find(|((t, i, c, pp, d), _)| {
            *t == ticker && *i == item && *c == currency && *pp == prev_p && (80..=100).contains(&(date - *d).num_days())
        });
        if let Some((_, prev_total)) = prev {
            derived.insert((ticker, item, currency, date), total - prev_total);
        }
    }

    let mut derived: Vec<_> = derived.into_iter().collect();
    derived.sort_by_key(|((t, i, c, d), _)| (t.to_string(), *i, *c, *d));
    items.extend(derived.into_iter().map(|((ticker, item, currency, date), val)| Reported {
        ticker, date, p: Period::ThreeMonths, item, currency, val,
    }));
}
