use crate::Ticker;
use crate::Period;
use crate::Statement;
use crate::BalanceSheet as BS;
use crate::IncomeStatement as IS;
use crate::ClassifiedBalanceSheet as CBS;
use crate::MultiStepIncomeStatement as MIS;
use crate::CashFlowStatement as CFS;

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
    fn total_assets(&self) -> f64 {
        self.total_current_assets()
            + self.property_and_equipment_net
            + self.operating_lease_assets
            + self.goodwill
            + self.intangible_assets_net
            + self.deferred_income_tax_assets
            + self.other_assets
    }
    fn total_liabilities(&self) -> f64 {
        self.total_current_liabilities()
            + self.long_term_debt
            + self.long_term_operating_lease_liabilities
            + self.other_long_term_liabilities
    }
}

impl CBS for BalanceSheet {
    fn total_current_assets(&self) -> f64 {
        self.cash_and_cash_equivalents
            + self.marketable_securities
            + self.accounts_receivable_net
            + self.inventories
            + self.prepaid_expenses_and_other_current_assets
    }
    fn total_current_liabilities(&self) -> f64 {
        self.accounts_payable
            + self.accrued_and_other_current_liabilities
            + self.short_term_debt
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
    fn income_before_income_tax(&self) -> f64 {
        self.operating_income() + self.total_other_income_net()
    }
    fn net_income(&self) -> f64 {
        self.income_before_income_tax() - self.income_tax_expense
    }
}

impl MIS for IncomeStatement {
    fn gross_profit(&self) -> f64 {
        self.revenue - self.cost_of_revenue
    }
    fn total_operating_expenses(&self) -> f64 {
        self.research_and_development + self.sales_general_and_administrative
    }
}

impl IncomeStatement {
    pub fn total_other_income_net(&self) -> f64 {
        self.interest_income + self.interest_expense + self.other_income_net
    }
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
    BalanceSheet,
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

impl CFS for CashFlowStatement {
    fn net_cash_from_operating_activities(&self) -> f64 {
        self.net_cash_provided_by_operating_activities()
    }
    fn net_cash_from_investing_activities(&self) -> f64 {
        self.net_cash_used_in_investing_activities()
    }
    fn net_cash_from_financing_activities(&self) -> f64 {
        self.net_cash_used_in_financing_activities()
    }
}
