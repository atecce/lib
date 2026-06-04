use equities::BalanceSheet;
use equities::item::Item;
use equities::Period::PointInTime;
use equities::ReportedItem;

#[test]
fn balance_sheet() {

    let balance_sheets = vec![
        BalanceSheet {
            ticker: "NVDA".to_string(),

            t: "2025-01-26".to_string(),

            cash_and_cash_equivalents: 8_589_000_000.0,
            marketable_securities: 34_621_000_000.0,
            accounts_receivable_net: 23_065_000_000.0,
            inventories: 10_080_000_000.0,
            prepaid_expenses_and_other_current_assets: 3_771_000_000.0,

            property_and_equipment_net: 6_283_000_000.0,
            operating_lease_assets: 1_793_000_000.0,
            goodwill: 5_188_000_000.0,
            intangible_assets_net: 807_000_000.0,
            deferred_income_tax_assets: 10_979_000_000.0,
            other_assets: 6_425_000_000.0,

            accounts_payable: 6_310_000_000.0,
            accrued_and_other_current_liabilities: 11_737_000_000.0,
            short_term_debt: 0.0,

            long_term_debt: 8_463_000_000.0,
            long_term_operating_lease_liabilities: 1_519_000_000.0,
            other_long_term_liabilities: 4_245_000_000.0,
        },
        BalanceSheet {
            ticker: "NVDA".to_string(),

            t: "2025-10-26".to_string(),

            cash_and_cash_equivalents: 11_486_000_000.0,
            marketable_securities: 49_122_000_000.0,
            accounts_receivable_net: 33_391_000_000.0,
            inventories: 19_784_000_000.0,
            prepaid_expenses_and_other_current_assets: 2_709_000_000.0,

            property_and_equipment_net: 9_780_000_000.0,
            operating_lease_assets: 2_281_000_000.0,
            goodwill: 6_261_000_000.0,
            intangible_assets_net: 936_000_000.0,
            deferred_income_tax_assets: 13_674_000_000.0,
            other_assets: 11_724_000_000.0,

            accounts_payable: 8_624_000_000.0,
            accrued_and_other_current_liabilities: 16_452_000_000.0,
            short_term_debt: 999_000_000.0,

            long_term_debt: 7_468_000_000.0,
            long_term_operating_lease_liabilities: 2_014_000_000.0,
            other_long_term_liabilities: 6_694_000_000.0,
        },
    ];

    let expected = [balance_sheets[0].reported_items(), balance_sheets[1].reported_items()].concat();

    let mut r = equities::reader::new_reader(std::path::Path::new("nvda/2025-11-19.xlsx"), equities::Ticker::NVDA).unwrap();
    let mut actual = r.process_balance_sheet().unwrap();

    actual.sort_by_cached_key(|item| {
        match chrono::NaiveDate::parse_from_str(&item.t.trim(), "%Y-%m-%d") {
            Ok(date) => (date, item.p, item.item),
            Err(e) => {
                eprintln!("failed to parse {} into date: {}", &item.t, e);
                (chrono::NaiveDate::MIN, item.p, item.item)
            },
        }
    });

    println!("{:#?}", actual);

    for (i, actual_reported_item) in actual.into_iter()
        .filter(|item| !item.item.is_aggregate())
        .enumerate() {
        assert!(
            actual_reported_item == expected[i],
            "\nactual:\n{:#?}\n\nexpected:\n{:#?}\n",
            actual_reported_item,
            expected[i],
        )
    }
}
