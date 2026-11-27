uniffi::setup_scaffolding!();

pub mod item;
pub mod xlsx;
pub mod filing;
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

/// A stock ticker symbol, e.g. "NVDA". Any symbol is accepted; which company
/// it is comes from the SEC's ticker map (see `edgar::Cache::cik`).
///
/// Stored inline so it stays `Copy`. The tickers this crate has code for
/// specifically are associated constants, usable as `Ticker::NVDA`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ticker {
    len: u8,
    bytes: [u8; Ticker::MAX_LEN],
}

impl Ticker {
    const MAX_LEN: usize = 10;

    pub const NVDA: Ticker = Ticker::new("NVDA");
    pub const TSLA: Ticker = Ticker::new("TSLA");
    pub const AAPL: Ticker = Ticker::new("AAPL");
    pub const BRK: Ticker = Ticker::new("BRK");
    pub const TSM: Ticker = Ticker::new("TSM");
    pub const KO: Ticker = Ticker::new("KO");
    pub const MCD: Ticker = Ticker::new("MCD");

    /// For constants. Panics (at compile time, in a const) on anything that
    /// `from_str` would reject.
    pub const fn new(symbol: &str) -> Ticker {
        match Ticker::parse(symbol) {
            Some(t) => t,
            None => panic!("invalid ticker symbol"),
        }
    }

    /// Up to 10 letters, digits, '.' or '-', stored upper case.
    const fn parse(symbol: &str) -> Option<Ticker> {
        let s = symbol.as_bytes();
        if s.is_empty() || s.len() > Ticker::MAX_LEN {
            return None;
        }
        let mut bytes = [0u8; Ticker::MAX_LEN];
        let mut i = 0;
        while i < s.len() {
            let b = s[i].to_ascii_uppercase();
            if !(b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'.' || b == b'-') {
                return None;
            }
            bytes[i] = b;
            i += 1;
        }
        Some(Ticker { len: s.len() as u8, bytes })
    }

    pub fn as_str(&self) -> &str {
        // Only ASCII is ever stored.
        std::str::from_utf8(&self.bytes[..self.len as usize]).unwrap()
    }

    /// The symbol in the SEC's ticker map. Berkshire Hathaway trades as two
    /// share classes, listed separately; both map to the same company.
    pub fn sec_symbol(&self) -> &str {
        match *self {
            Ticker::BRK => "BRK-B",
            _ => self.as_str(),
        }
    }
}

impl FromStr for Ticker {
    type Err = TickerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ticker::parse(s.trim()).ok_or(TickerError::InvalidSymbol)
    }
}

#[derive(Debug)]
pub enum TickerError {
    InvalidSymbol,
}

impl std::fmt::Display for Ticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Ticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl serde::Serialize for Ticker {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
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
        // EDGAR writes "3 Months Ended", company reports write "Three Months Ended".
        // For fiscal calendars counted in weeks, EDGAR rounds to whole months:
        // PepsiCo's 36-week year to date is "8 Months Ended", so any count of
        // months maps to the nearest quarter boundary.
        if let Some(months) = MONTHS.captures(&lower).and_then(|c| c[1].parse::<u32>().ok()) {
            // Only lengths a quarter, half or year rounds to; anything else
            // (a one-month stub period after an acquisition) is not a quarter.
            match months {
                3 => Ok(Period::ThreeMonths),
                5..=6 => Ok(Period::SixMonths),
                8..=9 => Ok(Period::NineMonths),
                12..=13 => Ok(Period::TwelveMonths),
                _ => Err(PeriodError::InvalidPeriod),
            }
        }
        else if lower.contains("three months") { Ok(Period::ThreeMonths) }
        else if lower.contains("six months") { Ok(Period::SixMonths) }
        else if lower.contains("nine months") { Ok(Period::NineMonths) }
        else if lower.contains("year ended") || lower.contains("annual") || lower.contains("twelve months") { Ok(Period::TwelveMonths) }
        // Retailers and others on 52/53-week years count in weeks: PepsiCo's
        // quarters are 12, 12, 12 and 16 weeks ("24 Weeks Ended" is its first half).
        else if let Some(weeks) = WEEKS.captures(&lower).and_then(|c| c[1].parse::<u32>().ok()) {
            match weeks {
                12..=14 | 16..=17 => Ok(Period::ThreeMonths),
                24..=27 => Ok(Period::SixMonths),
                36..=40 => Ok(Period::NineMonths),
                52..=53 => Ok(Period::TwelveMonths),
                _ => Err(PeriodError::InvalidPeriod),
            }
        }
        else { Err(PeriodError::InvalidPeriod) }
    }
}

static MONTHS: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"\b(\d{1,2}) months?\b").unwrap()
});

static WEEKS: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"\b(\d{1,2}) weeks? ended").unwrap()
});

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
        StockholdersEquity => field stockholders_equity,
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

/// Share counts at the scale their statement actually uses.
///
/// Headers often misstate it: McDonald's pages say only "$ in Millions"
/// over share counts in millions, Berkshire's "In Millions" over counts in
/// units. But basic EPS times basic weighted shares is net income (to within
/// noncontrolling interests and rounding), in every column. Where that
/// product is consistently off by a power of 1,000, the shares are rescaled
/// by it. Columns with EPS under 10 cents are too coarse to tell.
pub(crate) fn calibrate_shares(items: &mut [Reported]) {
    use std::collections::HashMap;
    use crate::item::Unit;

    let mut columns: HashMap<(NaiveDate, Period, Currency), [Option<f64>; 3]> = HashMap::new();
    for it in items.iter() {
        let slot = match it.item {
            Item::NetIncome => 0,
            Item::EarningsPerShareBasic => 1,
            Item::WeightedAverageSharesBasic => 2,
            _ => continue,
        };
        columns.entry((it.date, it.p, it.currency)).or_default()[slot] = Some(it.val);
    }

    let mut powers = Vec::new();
    for [ni, eps, shares] in columns.into_values() {
        let (Some(ni), Some(eps), Some(shares)) = (ni, eps, shares) else { continue };
        if eps.abs() < 0.10 || shares <= 0.0 { continue; }
        let ratio = ni / (eps * shares);
        if ratio <= 0.0 { continue; }
        let power = (ratio.log10() / 3.0).round() as i32;
        if (ratio / 1000f64.powi(power) - 1.0).abs() < 0.25 {
            powers.push(power);
        }
    }
    let Some(&power) = powers.first() else { return };
    if power == 0 || powers.iter().any(|&p| p != power) { return; }
    let factor = 1000f64.powi(power);
    for it in items.iter_mut().filter(|it| it.item.unit() == Unit::Shares) {
        it.val *= factor;
    }
}

/// Fill in three-month values that were not reported, from year-to-date
/// totals: Q2 = 6m - 3m, Q3 = 9m - 6m, Q4 = 12m - 9m, where the shorter
/// total ends about a quarter earlier (75 to 125 days: PepsiCo's fourth
/// quarter is 16 weeks). Annual reports only give twelve-month
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

    // Derived values with the period they came from: a fiscal year change can
    // derive the same quarter twice (ADM's 2012 transition reports both July-
    // December and the calendar year), and the shorter period wins.
    let mut derived: HashMap<(Ticker, Item, Currency, NaiveDate), (Period, f64)> = HashMap::new();
    for (&(ticker, item, currency, p, date), &total) in &latest {
        let Some(prev_p) = previous(p) else { continue };
        // A year's average share count or EPS minus nine months' is no quarter's.
        if item.unit() != crate::item::Unit::Money { continue; }
        if latest.contains_key(&(ticker, item, currency, Period::ThreeMonths, date)) {
            continue;
        }
        // Where two earlier periods fall in the window (a fiscal year change,
        // as ADM's in 2012), the one nearest a quarter back, so the result does
        // not depend on the map's iteration order.
        let prev = latest.iter()
            .filter(|((t, i, c, pp, d), _)| {
                *t == ticker && *i == item && *c == currency && *pp == prev_p && (75..=125).contains(&(date - *d).num_days())
            })
            .min_by_key(|((_, _, _, _, d), _)| ((date - *d).num_days().abs_diff(91), *d));
        if let Some((_, prev_total)) = prev {
            let entry = derived.entry((ticker, item, currency, date)).or_insert((p, total - prev_total));
            if p < entry.0 { *entry = (p, total - prev_total); }
        }
    }

    let mut derived: Vec<_> = derived.into_iter().collect();
    derived.sort_by_key(|((t, i, c, d), _)| (t.to_string(), *i, *c, *d));
    items.extend(derived.into_iter().map(|((ticker, item, currency, date), (_, val))| Reported {
        ticker, date, p: Period::ThreeMonths, item, currency, val,
    }));
}

#[cfg(test)]
mod ticker_tests {
    use super::Ticker;

    #[test]
    fn parse() {
        assert_eq!("ko".parse::<Ticker>().unwrap(), Ticker::KO);
        assert_eq!("BRK-B".parse::<Ticker>().unwrap().as_str(), "BRK-B");
        assert_eq!(Ticker::BRK.sec_symbol(), "BRK-B");
        assert_eq!(Ticker::NVDA.to_string(), "NVDA");
        assert!("".parse::<Ticker>().is_err());
        assert!("TOOLONGSYMBOL".parse::<Ticker>().is_err());
        assert!("A B".parse::<Ticker>().is_err());
    }
}

#[cfg(test)]
mod period_tests {
    use super::Period;

    #[test]
    fn weeks() {
        assert_eq!("12 Weeks Ended".parse::<Period>().unwrap(), Period::ThreeMonths);
        assert_eq!("24 Weeks Ended".parse::<Period>().unwrap(), Period::SixMonths);
        assert_eq!("36 Weeks Ended".parse::<Period>().unwrap(), Period::NineMonths);
        assert_eq!("53 Weeks Ended".parse::<Period>().unwrap(), Period::TwelveMonths);
        assert_eq!("3 Months Ended".parse::<Period>().unwrap(), Period::ThreeMonths);
        assert_eq!("5 Months Ended".parse::<Period>().unwrap(), Period::SixMonths);
        assert_eq!("8 Months Ended".parse::<Period>().unwrap(), Period::NineMonths);
        assert_eq!("12 Months Ended".parse::<Period>().unwrap(), Period::TwelveMonths);
        assert_eq!("Nine Months Ended".parse::<Period>().unwrap(), Period::NineMonths);
        assert!("2 Months Ended".parse::<Period>().is_err());
        assert!("0 Months Ended".parse::<Period>().is_err());
        assert!("4 Weeks Ended".parse::<Period>().is_err());
    }
}

/// Treat period end dates less than a week apart, within one company, as the
/// same date: the one from the filing read last. Companies on 52/53-week
/// years date the same quarter differently from filing to filing (Ciena:
/// "Oct. 31, 2018" in one, "Nov. 03, 2018" in the next), which would chart
/// the quarter twice. No two quarters end within a week of each other.
pub fn merge_nearby_dates(items: &mut [Reported]) {
    use std::collections::HashMap;
    const WITHIN_DAYS: i64 = 7;

    // Each company's dates, in the order they were read, so a later filing's
    // date becomes the one its cluster keeps.
    let mut order: HashMap<Ticker, Vec<NaiveDate>> = HashMap::new();
    for it in items.iter() {
        let dates = order.entry(it.ticker).or_default();
        dates.retain(|d| *d != it.date);
        dates.push(it.date);
    }

    let mut canonical: HashMap<(Ticker, NaiveDate), NaiveDate> = HashMap::new();
    for (ticker, dates) in order {
        let mut sorted = dates.clone();
        sorted.sort();
        // Clusters of dates each within a week of the previous one.
        let mut clusters: Vec<Vec<NaiveDate>> = Vec::new();
        for d in sorted {
            match clusters.last_mut() {
                Some(c) if (d - *c.last().unwrap()).num_days() < WITHIN_DAYS => c.push(d),
                _ => clusters.push(vec![d]),
            }
        }
        let last_read = |c: &Vec<NaiveDate>| *dates.iter().rev().find(|d| c.contains(d)).unwrap();
        for c in &clusters {
            let keep = last_read(c);
            for d in c {
                canonical.insert((ticker, *d), keep);
            }
        }
    }
    for it in items.iter_mut() {
        it.date = canonical[&(it.ticker, it.date)];
    }
}
