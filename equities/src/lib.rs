uniffi::setup_scaffolding!();

pub mod item;
pub mod nvda;
pub mod reader;
pub mod sheet_info;

use std::str::FromStr;

use crate::item::Item;
use crate::item::Reported;
use crate::nvda::NVDABalanceSheet;

use chrono::NaiveDate;

macro_rules! count_items {
    () => { 0 };
    ($head:ident $($tail:ident)*) => { 1 + count_items!($($tail)*) };
}

pub trait Statement {
    fn ticker(&self) -> Ticker;
    fn period(&self) -> Period;
}

macro_rules! impl_reported_items {
    ($name:ident, [ $($variant:ident => $kind:ident $source:ident),* $(,)? ]) => {
        impl $name {
            pub fn reported_items(&self) -> [Reported; count_items!($($variant)*)] {
                [
                    $(
                        Reported {
                            ticker: Statement::ticker(self),
                            date: self.date,
                            p: self.period(),
                            item: Item::$variant,
                            val: impl_reported_items!(@val self, $kind $source),
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
    fn ticker(&self) -> Ticker;
}

impl<T: BalanceSheet> Statement for T {
    fn ticker(&self) -> Ticker {
        self.ticker()
    }
    fn period(&self) -> Period {
        Period::PointInTime
    }
}

pub struct IncomeStatement {
    pub ticker: Ticker,
    pub date: NaiveDate,
    pub p: Period,

    pub revenue: f64,
    pub cost_of_revenue: f64,

    pub research_and_development: f64,
    pub sales_general_and_administrative: f64,

    pub interest_income: f64,
    pub interest_expense: f64,
    pub other_income_net: f64,

    pub income_tax_expense: f64,
}

impl Statement for IncomeStatement {
    fn ticker(&self) -> Ticker {
        self.ticker
    }
    fn period(&self) -> Period {
        self.p
    }
}

impl IncomeStatement {
    fn gross_profit(&self) -> f64 {
        self.revenue - self.cost_of_revenue
    }
    fn total_operating_expenses(&self) -> f64 {
        self.research_and_development + self.sales_general_and_administrative
    }
    fn operating_income(&self) -> f64 {
        self.gross_profit() - self.total_operating_expenses()
    }
    fn total_other_income_net(&self) -> f64 {
        self.interest_income + self.interest_expense + self.other_income_net
    }
    fn income_before_income_tax(&self) -> f64 {
        self.operating_income() + self.total_other_income_net()
    }
    pub fn net_income(&self) -> f64 {
        self.income_before_income_tax() - self.income_tax_expense
    }
}

impl_reported_items! {
    IncomeStatement,
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

    fn cash_and_cash_equivalents_at_end_of_period(&self) -> f64 {
        self.change_in_cash_and_cash_equivalents() + self.cash_and_cash_equivalents_at_beginning_of_period
    }
}

impl_reported_items! {
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
        ProceedsRelatedToEmployeeStockPlans =>  field proceeds_related_to_employee_stock_plans,
        PaymentsRelatedToRepurchasesOfCommonStock => field payments_related_to_repurchases_of_common_stock,
        PaymentsRelatedToEmployeeStockPlanTaxes => field payments_related_to_employee_stock_plan_taxes,
        DividendsPaid => field dividends_paid,
        PrincipalPaymentsOnPropertyAndEquipmentAndIntangibleAssets => field principal_payments_on_property_and_equipment_and_intangible_assets,
        RepaymentOfDebt => field repayment_of_debt,
        ChangeInCashAndCashEquivalents => method change_in_cash_and_cash_equivalents,
    ]
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
        if lower.contains("three months") { Ok(Period::ThreeMonths) }
        else if lower.contains("six months") { Ok(Period::SixMonths) }
        else if lower.contains("nine months") { Ok(Period::NineMonths) }
        else if lower.contains("year ended") || lower.contains("annual") || lower.contains("twelve months") { Ok(Period::TwelveMonths) }
        else { Err(PeriodError::InvalidPeriod) }
    }
}

#[derive(Debug, uniffi::Error)]
pub enum PeriodError {
    InvalidPeriod,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
pub enum Ticker {
    NVDA,
    TSLA,
}

impl FromStr for Ticker {
    type Err = TickerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NVDA" => Ok(Ticker::NVDA),
            "TSLA" => Ok(Ticker::TSLA),
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
        }
    }
}

impl_reported_items! {
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
