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

    pub operating_lease_vehicles_net: f64,
    pub solar_energy_systems_net: f64,

    pub property_and_equipment_net: f64,
    pub operating_lease_assets: f64,
    pub digital_assets: f64,
    pub intangible_assets_net: f64,
    pub goodwill: f64,
    pub deferred_income_tax_assets: f64,
    pub other_assets: f64,

    pub accounts_payable: f64,
    pub accrued_and_other_current_liabilities: f64,
    pub deferred_revenue: f64,
    pub current_portion_of_debt_and_finance_leases: f64,

    pub debt_and_finance_leases_net_of_current_portion: f64,
    pub deferred_revenue_net_of_current_portion: f64,
    pub other_long_term_liabilities: f64,
}

impl Statement for BalanceSheet {
    fn ticker(&self) -> Ticker {
        Ticker::TSLA
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
            + self.operating_lease_vehicles_net
            + self.solar_energy_systems_net
            + self.property_and_equipment_net
            + self.operating_lease_assets
            + self.digital_assets
            + self.intangible_assets_net
            + self.goodwill
            + self.deferred_income_tax_assets
            + self.other_assets
    }
    fn total_current_liabilities(&self) -> f64 {
        self.accounts_payable
            + self.accrued_and_other_current_liabilities
            + self.deferred_revenue
            + self.current_portion_of_debt_and_finance_leases
    }
    fn total_liabilities(&self) -> f64 {
        self.total_current_liabilities()
            + self.debt_and_finance_leases_net_of_current_portion
            + self.deferred_revenue_net_of_current_portion
            + self.other_long_term_liabilities
    }
}

pub struct IncomeStatement {
    pub date: NaiveDate,
    pub period: Period,

    pub automotive_sales_revenue: f64,
    pub automotive_regulatory_credits: f64,
    pub automotive_leasing_revenue: f64,

    pub energy_generation_and_storage_revenue: f64,
    pub services_and_other_revenue: f64,

    pub automotive_sales_cost_of_revenue: f64,
    pub automotive_leasing_cost_of_revenue: f64,

    pub energy_generation_and_storage_cost_of_revenue: f64,
    pub services_and_other_cost_of_revenue: f64,

    pub research_and_development: f64,
    pub selling_general_and_administrative: f64,
    pub restructuring_and_other: f64,

    pub interest_income: f64,
    pub interest_expense: f64,
    pub other_expense_income_net: f64,

    pub provision_for_income_taxes: f64,
}

impl Statement for IncomeStatement {
    fn ticker(&self) -> Ticker {
        Ticker::TSLA
    }
    fn period(&self) -> Period {
        self.period
    }
}

impl IS for IncomeStatement {
    fn gross_profit(&self) -> f64 {
        self.total_revenues() - self.total_cost_of_revenues()
    }
    fn total_operating_expenses(&self) -> f64 {
        self.research_and_development
            + self.selling_general_and_administrative
            + self.restructuring_and_other
    }
    fn income_before_income_tax(&self) -> f64 {
        self.operating_income()
            + self.interest_income
            + self.interest_expense
            + self.other_expense_income_net
    }
    fn net_income(&self) -> f64 {
        self.income_before_income_tax() - self.provision_for_income_taxes
    }
}

impl IncomeStatement {
    pub fn total_automotive_revenues(&self) -> f64 {
        self.automotive_sales_revenue
            + self.automotive_regulatory_credits
            + self.automotive_leasing_revenue
    }
    pub fn total_revenues(&self) -> f64 {
        self.total_automotive_revenues()
            + self.energy_generation_and_storage_revenue
            + self.services_and_other_revenue
    }
    pub fn total_automotive_cost_of_revenues(&self) -> f64 {
        self.automotive_sales_cost_of_revenue
            + self.automotive_leasing_cost_of_revenue
    }
    pub fn total_cost_of_revenues(&self) -> f64 {
        self.total_automotive_cost_of_revenues()
            + self.energy_generation_and_storage_cost_of_revenue
            + self.services_and_other_cost_of_revenue
    }
}
