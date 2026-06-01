use equities::item::Item;
use equities::Period::PointInTime;
use equities::ReportedItem;

#[test]
fn report() {
    let reported_items = [
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::CashAndCashEquivalents,
            val: 11486.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::MarketableSecurities,
            val: 49122.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::AccountsReceivableNet,
            val: 33391.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::Inventories,
            val: 19784.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::PrepaidExpensesAndOtherCurrentAssets,
            val: 2709.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::PropertyAndEquipmentNet,
            val: 9780.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::OperatingLeaseAssets,
            val: 2281.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::Goodwill,
            val: 6261.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::IntangibleAssetsNet,
            val: 936.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::DeferredIncomeTaxAssets,
            val: 13674.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::OtherAssets,
            val: 11724.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::AccountsPayable,
            val: 8624.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::AccruedAndOtherCurrentLiabilities,
            val: 16452.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::ShortTermDebt,
            val: 999.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::LongTermDebt,
            val: 7468.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::LongTermOperatingLeaseLiabilities,
            val: 2014.0,
        },
        ReportedItem {
            ticker: "nvda".to_string(),
            t: "2026-10-26".to_string(),
            p: PointInTime,
            item: Item::OtherLongTermLiabilities,
            val: 6694.0,
        },
    ];
}
