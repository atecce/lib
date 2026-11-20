use std::error::Error;
use std::fs;

use equities::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let dry_run = std::env::args().any(|a| a == "--dry-run");
    let mut reported_items = Vec::new();

    let equities = vec![equities::Ticker::NVDA, equities::Ticker::TSLA];

    for equity in equities {
        let dir = format!("equities/{}", equity);
        let mut paths: Vec<_> = fs::read_dir(&dir)?
            .filter_map(|f| f.ok())
            .map(|f| f.path())
            .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("xlsx"))
            .collect();
        paths.sort();

        for path in paths {
            match equities::xlsx::new_reader(&path, equity) {
                Ok(mut r) => {
                    match r.process_balance_sheet() {
                        Ok(mut ret) => reported_items.append(&mut ret),
                        Err(e) => eprintln!("failed to process balance sheet for path: {:?}; {}", path, e),
                    }
                    match r.process_income_statement() {
                        Ok(mut ret) => reported_items.append(&mut ret),
                        Err(e) => eprintln!("failed to process income statement for path {:?}; {}", path, e),
                    }
                },
                Err(e) => eprintln!("failed to construct new reader from path: {:?}; {}", path, e),
            }
        }
    }

    equities::derive_fourth_quarters(&mut reported_items);

    if dry_run {
        print_series(&reported_items);
        return Ok(());
    }

    if !reported_items.is_empty() {
        push_to_influx(&reported_items)?;
        println!("Successfully pushed {} items to InfluxDB", reported_items.len());
    }

    Ok(())
}

/// Print the series charted in Grafana, one row per (ticker, date), to check
/// the data before pushing it. Later reports restate earlier dates, so where a
/// (ticker, item, period, date) was reported more than once with different
/// values, the row is marked with '*'.
fn print_series(items: &[equities::item::Reported]) {
    use equities::item::Item;
    use equities::Period;
    use std::collections::BTreeMap;

    let cols: [(Item, Period, &str); 5] = [
        (Item::Revenue, Period::ThreeMonths, "revenue"),
        (Item::NetIncome, Period::ThreeMonths, "net income"),
        (Item::CashAndCashEquivalents, Period::PointInTime, "cash"),
        (Item::MarketableSecurities, Period::PointInTime, "mkt sec"),
        (Item::Inventories, Period::PointInTime, "inventory"),
    ];

    let mut table: BTreeMap<(String, chrono::NaiveDate), [Vec<f64>; 5]> = BTreeMap::new();
    for it in items {
        if let Some(i) = cols.iter().position(|(item, p, _)| *item == it.item && *p == it.p) {
            let row = table.entry((it.ticker.to_string(), it.date)).or_default();
            if !row[i].contains(&it.val) { row[i].push(it.val); }
        }
    }

    print!("{:<6}{:<12}", "ticker", "date");
    for (_, _, name) in &cols { print!("{:>12}", name); }
    println!();
    for ((ticker, date), row) in &table {
        print!("{:<6}{:<12}", ticker, date.to_string());
        for vals in row {
            let cell = match vals.as_slice() {
                [] => "-".to_string(),
                [v] => format!("{:.2}B", v / 1e9),
                vs => vs.iter().map(|v| format!("{:.2}", v / 1e9)).collect::<Vec<_>>().join("/"),
            };
            print!("{:>12}", cell);
        }
        println!();
    }
}

fn push_to_influx(items: &[equities::item::Reported]) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let auth_token = std::env::var("INFLUXDB3_AUTH_TOKEN")
        .map_err(|_| "INFLUXDB3_AUTH_TOKEN environment variable not set")?;

    for chunk in items.chunks(1000) {
        let mut payload = String::new();
        for item in chunk {
            let ts = item.date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_nanos_opt().unwrap();
            payload.push_str(&format!(
                "reported_item_v12,ticker={},item={},period={} value={} {}\n",
                item.ticker,
                item.item,
                item.p,
                item.val,
                ts
            ));

        }

        let res = client.post("http://localhost:8181/write?db=financials")
            .bearer_auth(&auth_token)
            .body(payload)
            .send()?;

        if !res.status().is_success() {
            return Err(format!("InfluxDB write failed: {}", res.text()?).into());
        }
    }

    Ok(())
}
