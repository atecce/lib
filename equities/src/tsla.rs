use crate::Ticker;
use crate::BalanceSheet as BS;

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

impl BS for BalanceSheet {
    fn ticker(&self) -> Ticker {
        Ticker::TSLA
    }
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
