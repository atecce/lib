use equities::{BalanceSheet, CashFlowStatement, IncomeStatement, item::ReportedItem};

#[test]
fn report() {

//    let balance_sheets = vec![
//        BalanceSheet {
//            ticker: equities::Ticker::TSLA,
//            date: chrono::NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
//
//            cash_and_cash_equivalents: 16_139_000_000.0,
//            // short_term_investments: 20_424_000_000.0,
//            accounts_receivable_net: 4_418_000_000.0,
//            inventories: 12_017_000_000.0,
//            prepaid_expenses_and_other_current_assets: 5_362_000_000.0,
//
//            // operating_lease_vehicles_net: 5_581_000_000.0,
//            // solar_energy_systems_net: 4_924_000_000.0,
//
//            // property_plant_and_equipment_net: 35_836_000_000.0,
//            // operating_lease_right_of_use_assets: 5_160_000_000.0,
//            // digital assets: 1_076_000_000.0,
//            intangible_assets_net: 150_000_000.0,
//            goodwill: 244_000_000.0,
//            // deferred_tax_assets: 6_524_000_000.0,
//            // other_non_current_assets: 4_215_000_000.0,
//
//            accounts_payable: 12_474_000_000.0,
//            // accrued_liabilities_and_other: 10_723_000_000.0,
//            // deferred_revenue: 3_168_000_000.0,
//            // current_portion_of_debt_and_finance_leases: 2_456_000_000.0,
//            // debt_and_finance_leases_net_of_current_portion: 5_757_000_000.0,
//            // deferred_revenue_net_of_current_portion: 3_317_000_000.0,
//
//            other_long_term_liabilities: 10_495_000_000.0,
//        },
//    ];

//    let mut r = equities::reader::new_reader(std::path::Path::new("nvda/2025-11-19.xlsx"), equities::Ticker::NVDA).unwrap();

//    assert_reported_items(r.process_balance_sheet().unwrap(), balance_sheets.into_iter().map(|sheet| sheet.reported_items()).flatten().collect());
}

fn assert_reported_items(mut actual: Vec<ReportedItem>, expected: Vec<ReportedItem>) {
    actual.sort_by_cached_key(|item| (item.date, item.p, item.item));

    let zipped = actual.into_iter().zip(expected);
    println!("{:#?}", zipped);

    for (actual_reported_item, expected_reported_item) in zipped {
        assert!(
            actual_reported_item == expected_reported_item,
            "\nactual:\n{:#?}\n\nexpected:\n{:#?}\n",
            actual_reported_item, expected_reported_item,
        )
    }
}
