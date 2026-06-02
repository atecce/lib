use equities::item::Item;
use equities::Period::PointInTime;
use equities::ReportedItem;

#[test]
fn report() {
    let expected = [

        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::CashAndCashEquivalents,
            val: 8589000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::MarketableSecurities,
            val: 34621000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::AccountsReceivableNet,
            val: 23065000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::Inventories,
            val: 10080000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::PrepaidExpensesAndOtherCurrentAssets,
            val: 3771000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::PropertyAndEquipmentNet,
            val: 6283000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::OperatingLeaseAssets,
            val: 1793000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::Goodwill,
            val: 5188000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::IntangibleAssetsNet,
            val: 807000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::DeferredIncomeTaxAssets,
            val: 10979000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::OtherAssets,
            val: 6425000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::AccountsPayable,
            val: 6310000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::AccruedAndOtherCurrentLiabilities,
            val: 11737000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::ShortTermDebt,
            val: 0.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::LongTermDebt,
            val: 8463000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::LongTermOperatingLeaseLiabilities,
            val: 1519000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-01-26".to_string(),
            p: PointInTime,
            item: Item::OtherLongTermLiabilities,
            val: 4245000000.0,
        },

        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::CashAndCashEquivalents,
            val: 11486000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::MarketableSecurities,
            val: 49122000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::AccountsReceivableNet,
            val: 33391000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::Inventories,
            val: 19784000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::PrepaidExpensesAndOtherCurrentAssets,
            val: 2709000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::PropertyAndEquipmentNet,
            val: 9780000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::OperatingLeaseAssets,
            val: 2281000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::Goodwill,
            val: 6261000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::IntangibleAssetsNet,
            val: 936000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::DeferredIncomeTaxAssets,
            val: 13674000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::OtherAssets,
            val: 11724000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::AccountsPayable,
            val: 8624000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::AccruedAndOtherCurrentLiabilities,
            val: 16452000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::ShortTermDebt,
            val: 999000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::LongTermDebt,
            val: 7468000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::LongTermOperatingLeaseLiabilities,
            val: 2014000000.0,
        },
        ReportedItem {
            ticker: "NVDA".to_string(),
            t: "2025-10-26".to_string(),
            p: PointInTime,
            item: Item::OtherLongTermLiabilities,
            val: 6694000000.0,
        },
    ];

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

    for (i, actual_reported_item) in actual.into_iter().enumerate() {
        assert!(
            actual_reported_item == expected[i],
            "\nactual:\n{:#?}\n\nexpected:\n{:#?}\n",
            actual_reported_item,
            expected[i],
        )
    }
}
