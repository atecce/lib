mod reported;

use equities::Reader;
use equities::tsla::BalanceSheet;

#[test]
fn report() {

    let balance_sheets = vec![
        BalanceSheet {
            date: chrono::NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),

            cash_and_cash_equivalents: 16_139_000_000.0,
            marketable_securities: 20_424_000_000.0,
            accounts_receivable_net: 4_418_000_000.0,
            inventories: 12_017_000_000.0,
            prepaid_expenses_and_other_current_assets: 5_362_000_000.0,

            operating_lease_vehicles_net: 5_581_000_000.0,
            solar_energy_systems_net: 4_924_000_000.0,

            property_and_equipment_net: 35_836_000_000.0,
            operating_lease_assets: 5_160_000_000.0,
            digital_assets: 1_076_000_000.0,
            intangible_assets_net: 150_000_000.0,
            goodwill: 244_000_000.0,
            deferred_income_tax_assets: 6_524_000_000.0,
            other_assets: 4_215_000_000.0,

            accounts_payable: 12_474_000_000.0,
            accrued_and_other_current_liabilities: 10_723_000_000.0,
            deferred_revenue: 3_168_000_000.0,
            current_portion_of_debt_and_finance_leases: 2_456_000_000.0,

            debt_and_finance_leases_net_of_current_portion: 5_757_000_000.0,
            deferred_revenue_net_of_current_portion: 3_317_000_000.0,
            other_long_term_liabilities: 10_495_000_000.0,
        },
    ];

    let mut r = equities::pdf::new_reader(std::path::Path::new("tsla/tsla-20250930-gen.pdf"), equities::Ticker::TSLA).unwrap();
    reported::assert(r.process_balance_sheet().unwrap(), balance_sheets.into_iter().map(|sheet| sheet.reported()).flatten().collect());
}
