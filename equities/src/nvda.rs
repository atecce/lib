use crate::Ticker;
use crate::Period;
use crate::Statement;
use crate::BalanceSheet as BS;
use crate::IncomeStatement as IS;

use chrono::NaiveDate;

pub struct BalanceSheet {
    pub date: NaiveDate,

    pub cash_and_cash_equivalents: f64,
    pub marketable_securities: f64,
    pub accounts_receivable_net: f64,
    pub inventories: f64,
    pub prepaid_expenses_and_other_current_assets: f64,

    pub property_and_equipment_net: f64,
    pub operating_lease_assets: f64,
    pub goodwill: f64,
    pub intangible_assets_net: f64,
    pub deferred_income_tax_assets: f64,
    pub other_assets: f64,

    pub accounts_payable: f64,
    pub accrued_and_other_current_liabilities: f64,
    pub short_term_debt: f64,

    pub long_term_debt: f64,
    pub long_term_operating_lease_liabilities: f64,
    pub other_long_term_liabilities: f64,
}

impl Statement for BalanceSheet {
    fn ticker(&self) -> Ticker {
        Ticker::NVDA
    }
    fn period(&self) -> Period {
        Period::PointInTime
    }
}

impl BS for BalanceSheet {
    fn total_current_assets(&self) -> f64 {
        self.cash_and_cash_equivalents
            + self.marketable_securities
            + self.accounts_receivable_net
            + self.inventories
            + self.prepaid_expenses_and_other_current_assets
    }
    fn total_assets(&self) -> f64 {
        self.total_current_assets()
            + self.property_and_equipment_net
            + self.operating_lease_assets
            + self.goodwill
            + self.intangible_assets_net
            + self.deferred_income_tax_assets
            + self.other_assets
    }
    fn total_current_liabilities(&self) -> f64 {
        self.accounts_payable
            + self.accrued_and_other_current_liabilities
            + self.short_term_debt
    }
    fn total_liabilities(&self) -> f64 {
        self.total_current_liabilities()
            + self.long_term_debt
            + self.long_term_operating_lease_liabilities
            + self.other_long_term_liabilities
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

impl IS for IncomeStatement {
    fn gross_profit(&self) -> f64 {
        self.revenue - self.cost_of_revenue
    }
    fn total_operating_expenses(&self) -> f64 {
        self.research_and_development + self.sales_general_and_administrative
    }
    fn income_before_income_tax(&self) -> f64 {
        self.operating_income() + self.total_other_income_net()
    }
    fn net_income(&self) -> f64 {
        self.income_before_income_tax() - self.income_tax_expense
    }
}

impl IncomeStatement {
    pub fn total_other_income_net(&self) -> f64 {
        self.interest_income + self.interest_expense + self.other_income_net
    }
}
