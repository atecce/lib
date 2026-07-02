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
    pub customer_deposits: f64,
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
            + self.customer_deposits
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

pub struct CashFlowStatement {
    pub date: NaiveDate,
    pub period: Period,

    pub net_income: f64,

    pub depreciation_amortization_and_impairment: f64,
    pub stock_based_compensation: f64,
    pub inventory_and_purchase_commitments_write_downs: f64,
    pub foreign_currency_transaction_net_unrealized_loss: f64,
    pub deferred_income_taxes: f64,
    pub non_cash_interest_and_other_operating_activities: f64,
    pub digital_assets_gain_net: f64,

    pub changes_in_accounts_receivable: f64,
    pub changes_in_inventory: f64,
    pub changes_in_operating_lease_vehicles: f64,
    pub changes_in_prepaid_expenses_and_other_assets: f64,
    pub changes_in_accounts_payable_accrued_and_other_liabilities: f64,
    pub changes_in_deferred_revenue: f64,

    pub purchases_of_property_and_equipment_excluding_finance_leases_net_of_sales: f64,
    pub purchases_of_investments: f64,
    pub proceeds_from_maturities_of_investments: f64,
    pub proceeds_from_sales_of_investments: f64,

    pub proceeds_from_issuances_of_debt: f64,
    pub repayments_of_debt: f64,
    pub proceeds_from_exercises_of_stock_options_and_other_stock_issuances: f64,
    pub principal_payments_on_finance_leases: f64,
    pub proceeds_received_from_directors_in_shareholder_settlement: f64,
    pub payment_of_legal_fees_associated_with_shareholder_settlement: f64,
    pub debt_issuance_costs: f64,
    pub distributions_paid_to_noncontrolling_interests_in_subsidiaries: f64,
    pub payments_for_buy_outs_of_noncontrolling_interests_in_subsidiaries: f64,

    pub effect_of_exchange_rate_changes_on_cash_and_cash_equivalents_and_restricted_cash: f64,

    pub cash_and_cash_equivalents_and_restricted_cash_beginning_of_period: f64,

    pub acquisitions_of_property_and_equipment_included_in_liabilities: f64,
    pub leased_assets_obtained_in_exchange_for_finance_lease_liabilities: f64,
    pub leased_assets_obtained_in_exchange_for_operating_lease_liabilities: f64,
}

impl Statement for CashFlowStatement {
    fn ticker(&self) -> Ticker {
        Ticker::TSLA
    }
    fn period(&self) -> Period {
        self.period
    }
}

impl CashFlowStatement {
    fn net_cash_provided_by_operating_activities(&self) -> f64 {
        self.net_income
            + self.depreciation_amortization_and_impairment
            + self.stock_based_compensation
            + self.inventory_and_purchase_commitments_write_downs
            + self.foreign_currency_transaction_net_unrealized_loss
            + self.deferred_income_taxes
            + self.non_cash_interest_and_other_operating_activities
            + self.digital_assets_gain_net
            + self.changes_in_accounts_receivable
            + self.changes_in_inventory
            + self.changes_in_operating_lease_vehicles
            + self.changes_in_prepaid_expenses_and_other_assets
            + self.changes_in_accounts_payable_accrued_and_other_liabilities
            + self.changes_in_deferred_revenue
    }
    fn net_cash_used_in_investing_activities(&self) -> f64 {
        self.purchases_of_property_and_equipment_excluding_finance_leases_net_of_sales
            + self.purchases_of_investments
            + self.proceeds_from_maturities_of_investments
            + self.proceeds_from_sales_of_investments
    }
    fn net_cash_provided_by_financing_activities(&self) -> f64 {
        self.proceeds_from_issuances_of_debt
            + self.repayments_of_debt
            + self.proceeds_from_exercises_of_stock_options_and_other_stock_issuances
            + self.principal_payments_on_finance_leases
            + self.proceeds_received_from_directors_in_shareholder_settlement
            + self.payment_of_legal_fees_associated_with_shareholder_settlement
            + self.debt_issuance_costs
            + self.distributions_paid_to_noncontrolling_interests_in_subsidiaries
            + self.payments_for_buy_outs_of_noncontrolling_interests_in_subsidiaries
    }
    fn net_increase_in_cash_and_cash_equivalents_and_restricted_cash(&self) -> f64 {
        self.net_cash_provided_by_operating_activities()
            + self.net_cash_used_in_investing_activities()
            + self.net_cash_provided_by_financing_activities()
            + self.effect_of_exchange_rate_changes_on_cash_and_cash_equivalents_and_restricted_cash
    }
    fn cash_and_cash_equivalents_and_restricted_cash_end_of_period(&self) -> f64 {
        self.net_increase_in_cash_and_cash_equivalents_and_restricted_cash()
            + self.cash_and_cash_equivalents_and_restricted_cash_beginning_of_period
    }
}

pub const Q4_2019_BALANCE_SHEET: BalanceSheet = BalanceSheet {
    date: chrono::NaiveDate::from_ymd_opt(2019, 12, 31).unwrap(),

    cash_and_cash_equivalents: 6_268_000_000.0,
    marketable_securities: 0.0,
    accounts_receivable_net: 1_324_000_000.0,
    inventories: 3_552_000_000.0,
    prepaid_expenses_and_other_current_assets: 959_000_000.0,

    operating_lease_vehicles_net: 2_447_000_000.0,
    solar_energy_systems_net: 6_138_000_000.0,

    property_and_equipment_net: 10_396_000_000.0,
    operating_lease_assets: 1_218_000_000.0,
    digital_assets: 0.0,
    intangible_assets_net: 339_000_000.0,
    goodwill: 198_000_000.0,
    deferred_income_tax_assets: 0.0,
    other_assets: 1_470_000_000.0,

    accounts_payable: 3_771_000_000.0,
    accrued_and_other_current_liabilities: 3_222_000_000.0,
    deferred_revenue: 1_163_000_000.0,
    customer_deposits: 726_000_000.0,
    current_portion_of_debt_and_finance_leases: 1_785_000_000.0,

    debt_and_finance_leases_net_of_current_portion: 11_634_000_000.0,
    deferred_revenue_net_of_current_portion: 1_207_000_000.0,
    other_long_term_liabilities: 2_691_000_000.0,
};
