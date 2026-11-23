//! Apple's statements as it presents them (10-Q for the quarter ended
//! March 28, 2026, `aapl/10Q-Q2-2026-as-filed.pdf`; the same filing is
//! 0000320193-26-000013 in the EDGAR cache). Totals are computed from their
//! parts, so a total that disagrees with the filing's is a reading error.
//! Notes on the company: `README.md` here.

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
    /// Amounts Apple's contract manufacturers owe it for components it
    /// sold them: Apple buys parts in bulk and resells them to the
    /// assemblers, a line peculiar to it.
    pub vendor_non_trade_receivables: f64,
    pub inventories: f64,
    pub other_current_assets: f64,

    pub non_current_marketable_securities: f64,
    pub property_plant_and_equipment_net: f64,
    pub intangible_assets_net: f64,
    pub other_non_current_assets: f64,

    pub accounts_payable: f64,
    pub other_current_liabilities: f64,
    pub deferred_revenue: f64,
    pub commercial_paper: f64,
    pub term_debt: f64,

    pub non_current_term_debt: f64,
    pub other_non_current_liabilities: f64,

    pub common_stock_and_additional_paid_in_capital: f64,
    pub retained_earnings: f64,
    pub accumulated_other_comprehensive_income: f64,
}

impl Statement for BalanceSheet {
    fn ticker(&self) -> Ticker {
        Ticker::AAPL
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
            + self.vendor_non_trade_receivables
            + self.inventories
            + self.other_current_assets
    }
    fn total_assets(&self) -> f64 {
        self.total_current_assets() + self.total_non_current_assets()
    }
    fn total_current_liabilities(&self) -> f64 {
        self.accounts_payable
            + self.other_current_liabilities
            + self.deferred_revenue
            + self.commercial_paper
            + self.term_debt
    }
    fn total_liabilities(&self) -> f64 {
        self.total_current_liabilities() + self.total_non_current_liabilities()
    }
}

impl BalanceSheet {
    pub fn total_non_current_assets(&self) -> f64 {
        self.non_current_marketable_securities
            + self.property_plant_and_equipment_net
            + self.intangible_assets_net
            + self.other_non_current_assets
    }
    pub fn total_non_current_liabilities(&self) -> f64 {
        self.non_current_term_debt + self.other_non_current_liabilities
    }
    pub fn total_shareholders_equity(&self) -> f64 {
        self.common_stock_and_additional_paid_in_capital
            + self.retained_earnings
            + self.accumulated_other_comprehensive_income
    }
    /// The balance sheet's own identity: assets are claims, by creditors
    /// or shareholders. Zero when every line was read right.
    pub fn imbalance(&self) -> f64 {
        self.total_assets() - self.total_liabilities() - self.total_shareholders_equity()
    }
}

pub struct IncomeStatement {
    pub date: NaiveDate,
    pub p: Period,

    pub net_sales: f64,
    pub cost_of_sales: f64,

    pub research_and_development: f64,
    pub selling_general_and_administrative: f64,

    /// One line in Apple's statement: interest and dividend income, interest
    /// expense and the rest, net.
    pub other_income_expense_net: f64,

    pub provision_for_income_taxes: f64,
}

impl Statement for IncomeStatement {
    fn ticker(&self) -> Ticker {
        Ticker::AAPL
    }
    fn period(&self) -> Period {
        self.p
    }
}

impl IS for IncomeStatement {
    /// Apple calls it "gross margin", in dollars.
    fn gross_profit(&self) -> f64 {
        self.net_sales - self.cost_of_sales
    }
    fn total_operating_expenses(&self) -> f64 {
        self.research_and_development + self.selling_general_and_administrative
    }
    fn income_before_income_tax(&self) -> f64 {
        self.operating_income() + self.other_income_expense_net
    }
    fn net_income(&self) -> f64 {
        self.income_before_income_tax() - self.provision_for_income_taxes
    }
}

impl_reported! {
    BalanceSheet,
    [
        CashAndCashEquivalents => field cash_and_cash_equivalents,
        // The current part; the non-current marketable securities have no
        // item of their own yet, and are in TotalNonCurrentAssets.
        MarketableSecurities => field marketable_securities,
        AccountsReceivableNet => field accounts_receivable_net,
        VendorNonTradeReceivables => field vendor_non_trade_receivables,
        Inventories => field inventories,
        OtherCurrentAssets => field other_current_assets,
        TotalCurrentAssets => method total_current_assets,
        PropertyAndEquipmentNet => field property_plant_and_equipment_net,
        IntangibleAssetsNet => field intangible_assets_net,
        OtherAssets => field other_non_current_assets,
        TotalNonCurrentAssets => method total_non_current_assets,
        TotalAssets => method total_assets,
        AccountsPayable => field accounts_payable,
        OtherCurrentLiabilities => field other_current_liabilities,
        DeferredRevenue => field deferred_revenue,
        CommercialPaper => field commercial_paper,
        ShortTermDebt => field term_debt,
        TotalCurrentLiabilities => method total_current_liabilities,
        LongTermDebt => field non_current_term_debt,
        OtherLongTermLiabilities => field other_non_current_liabilities,
        TotalNonCurrentLiabilities => method total_non_current_liabilities,
        TotalLiabilities => method total_liabilities,
        StockholdersEquity => method total_shareholders_equity,
    ]
}

impl_reported! {
    IncomeStatement,
    [
        Revenue => field net_sales,
        CostOfRevenue => field cost_of_sales,
        GrossProfit => method gross_profit,
        ResearchAndDevelopment => field research_and_development,
        SalesGeneralAndAdministrative => field selling_general_and_administrative,
        TotalOperatingExpenses => method total_operating_expenses,
        OperatingIncome => method operating_income,
        TotalOtherIncomeNet => field other_income_expense_net,
        IncomeBeforeIncomeTax => method income_before_income_tax,
        IncomeTaxExpense => field provision_for_income_taxes,
        NetIncome => method net_income,
    ]
}

/// The balance sheet at March 28, 2026, from the 10-Q (dollars; the filing
/// states millions).
pub const Q2_2026_BALANCE_SHEET: BalanceSheet = BalanceSheet {
    date: NaiveDate::from_ymd_opt(2026, 3, 28).unwrap(),

    cash_and_cash_equivalents: 45_572_000_000.0,
    marketable_securities: 22_935_000_000.0,
    accounts_receivable_net: 30_339_000_000.0,
    vendor_non_trade_receivables: 23_172_000_000.0,
    inventories: 6_747_000_000.0,
    other_current_assets: 15_349_000_000.0,

    non_current_marketable_securities: 78_088_000_000.0,
    property_plant_and_equipment_net: 50_116_000_000.0,
    intangible_assets_net: 21_334_000_000.0,
    other_non_current_assets: 77_430_000_000.0,

    accounts_payable: 57_349_000_000.0,
    other_current_liabilities: 57_654_000_000.0,
    deferred_revenue: 9_331_000_000.0,
    commercial_paper: 1_997_000_000.0,
    term_debt: 8_310_000_000.0,

    non_current_term_debt: 74_404_000_000.0,
    other_non_current_liabilities: 55_546_000_000.0,

    common_stock_and_additional_paid_in_capital: 99_507_000_000.0,
    retained_earnings: 12_359_000_000.0,
    accumulated_other_comprehensive_income: -5_375_000_000.0,
};

/// The quarter ended March 28, 2026 (fiscal Q2), from the 10-Q.
pub const Q2_2026_INCOME_STATEMENT: IncomeStatement = IncomeStatement {
    date: NaiveDate::from_ymd_opt(2026, 3, 28).unwrap(),
    p: Period::ThreeMonths,

    net_sales: 111_184_000_000.0,
    cost_of_sales: 56_403_000_000.0,
    research_and_development: 11_419_000_000.0,
    selling_general_and_administrative: 7_477_000_000.0,
    other_income_expense_net: -52_000_000.0,
    provision_for_income_taxes: 6_255_000_000.0,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// The computed totals equal the 10-Q's printed ones.
    #[test]
    fn totals_match_the_filing() {
        let b = &Q2_2026_BALANCE_SHEET;
        assert_eq!(b.total_current_assets(), 144_114_000_000.0);
        assert_eq!(b.total_non_current_assets(), 226_968_000_000.0);
        assert_eq!(b.total_assets(), 371_082_000_000.0);
        assert_eq!(b.total_current_liabilities(), 134_641_000_000.0);
        assert_eq!(b.total_non_current_liabilities(), 129_950_000_000.0);
        assert_eq!(b.total_liabilities(), 264_591_000_000.0);
        assert_eq!(b.total_shareholders_equity(), 106_491_000_000.0);
        assert_eq!(b.imbalance(), 0.0);

        let i = &Q2_2026_INCOME_STATEMENT;
        assert_eq!(i.gross_profit(), 54_781_000_000.0);
        assert_eq!(i.total_operating_expenses(), 18_896_000_000.0);
        assert_eq!(i.operating_income(), 35_885_000_000.0);
        assert_eq!(i.income_before_income_tax(), 35_833_000_000.0);
        assert_eq!(i.net_income(), 29_578_000_000.0);
    }

    /// The generic reader, on the same filing, agrees with the hand-written
    /// statement, item by item.
    #[test]
    fn the_reader_agrees() {
        let path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../edgar/cache/320193/000032019326000013"));
        if !path.exists() { return; }
        let mut r = crate::filing::new_reader(path, Ticker::AAPL).unwrap();
        let read: Vec<crate::item::Reported> = [
            crate::Reader::process_balance_sheet(&mut r).unwrap(),
            crate::Reader::process_income_statement(&mut r).unwrap(),
        ].concat();
        let mine = Q2_2026_BALANCE_SHEET.reported().into_iter().chain(Q2_2026_INCOME_STATEMENT.reported());
        let mut agreed = 0;
        for m in mine {
            let found = read.iter().find(|r| r.item == m.item && r.date == m.date && r.p == m.p);
            match found {
                Some(r) => { assert_eq!(r.val, m.val, "{:?}", m.item); agreed += 1; },
                None => eprintln!("the reader has no {:?} for {}", m.item, m.date),
            }
        }
        assert!(agreed >= 30, "only {} items agreed", agreed);
    }
}
