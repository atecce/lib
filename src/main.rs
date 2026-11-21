use std::error::Error;

use equities::Reader;

/// Read when no tickers are given on the command line.
const DEFAULT_TICKERS: [equities::Ticker; 35] = [
    equities::Ticker::NVDA,
    equities::Ticker::TSLA,
    equities::Ticker::AAPL,
    equities::Ticker::BRK,
    equities::Ticker::TSM,
    equities::Ticker::KO,
    equities::Ticker::MCD,
    equities::Ticker::new("PEP"),
    equities::Ticker::new("OSCR"),
    equities::Ticker::new("NET"),
    // Block, formerly Square (SQ until January 2025).
    equities::Ticker::new("XYZ"),
    equities::Ticker::new("PLTR"),
    equities::Ticker::new("AMZN"),
    equities::Ticker::new("MSFT"),
    // Alphabet; also listed as GOOG (same company). Google Inc.'s filings
    // before the 2015 reorganization are under a different CIK.
    equities::Ticker::new("GOOGL"),
    equities::Ticker::new("IBM"),
    equities::Ticker::new("ORCL"),
    // Hardware behind the AI build-out: compute, memory and storage,
    // networking, servers, power and cooling, chip equipment.
    equities::Ticker::new("AMD"),
    equities::Ticker::new("AVGO"),
    equities::Ticker::new("MU"),
    equities::Ticker::new("SNDK"),
    equities::Ticker::new("WDC"),
    equities::Ticker::new("STX"),
    equities::Ticker::new("ANET"),
    equities::Ticker::new("MRVL"),
    equities::Ticker::new("CSCO"),
    equities::Ticker::new("COHR"),
    equities::Ticker::new("CIEN"),
    equities::Ticker::new("SMCI"),
    equities::Ticker::new("DELL"),
    equities::Ticker::new("HPE"),
    equities::Ticker::new("VRT"),
    equities::Ticker::new("AMAT"),
    equities::Ticker::new("LRCX"),
    equities::Ticker::new("KLAC"),
];

/// Usage: lib [--dry-run [--cash-flow] | --dump [--latest]] [TICKER ...]
/// --dump prints every value read; with --latest, only the values pushed.
/// Filings are read from the EDGAR cache (edgar/cache), which `fetch`
/// (the edgar crate's executable) brings up to date; nothing here touches
/// the network but the push to InfluxDB.

fn main() -> Result<(), Box<dyn Error>> {
    let dry_run = std::env::args().any(|a| a == "--dry-run");
    let mut reported_items = Vec::new();

    let mut tickers = Vec::new();
    for arg in std::env::args().skip(1).filter(|a| !a.starts_with("--")) {
        match arg.parse::<equities::Ticker>() {
            Ok(t) => tickers.push(t),
            Err(_) => return Err(format!("not a ticker symbol: {}", arg).into()),
        }
    }
    if tickers.is_empty() {
        tickers = DEFAULT_TICKERS.to_vec();
    }

    let cache = edgar::Cache::new(edgar::Cache::default_dir());
    let japan = edinet::Cache::new(edinet::Cache::default_dir());

    for equity in tickers {
        // Tokyo tickers ("7203.T") come from EDINET, the rest from EDGAR.
        let read = if is_edinet(equity) {
            read_edinet(&japan, equity, &mut reported_items)
        } else {
            read_edgar(&cache, equity, &mut reported_items)
        };
        if let Err(e) = read {
            eprintln!("{}: {}", equity, e);
        }
    }

    equities::merge_nearby_dates(&mut reported_items);
    equities::derive_quarters(&mut reported_items);
    let mut book = equities::book_value_per_share(&reported_items);
    reported_items.append(&mut book);

    // Every item read, one per line, sorted: for diffing the output of two versions.
    if std::env::args().any(|a| a == "--dump") {
        let reported_items = if std::env::args().any(|a| a == "--latest") { latest(reported_items) } else { reported_items };
        let mut lines: Vec<String> = reported_items.iter()
            .map(|it| format!("{}\t{}\t{}\t{}\t{}\t{}", it.ticker, it.date, it.p, it.item, it.currency, it.val))
            .collect();
        lines.sort();
        lines.dedup();
        println!("{}", lines.join("\n"));
        return Ok(());
    }

    if dry_run {
        print_series(&reported_items, std::env::args().any(|a| a == "--cash-flow"));
        return Ok(());
    }

    if !reported_items.is_empty() {
        let items = latest(reported_items);
        push_to_influx(&items)?;
        println!("Successfully pushed {} items to InfluxDB", items.len());
    }

    Ok(())
}

/// One value per (ticker, item, period, currency, date): the last one read.
/// Filings are read oldest first, so a restatement replaces the original.
/// Resolved here rather than left to InfluxDB, which does not reliably keep
/// the last of two points with the same series and timestamp in one write.
fn latest(items: Vec<equities::item::Reported>) -> Vec<equities::item::Reported> {
    let mut index = std::collections::HashMap::new();
    let mut out: Vec<equities::item::Reported> = Vec::with_capacity(items.len());
    for it in items {
        match index.entry((it.ticker, it.item, it.p, it.currency, it.date)) {
            std::collections::hash_map::Entry::Occupied(e) => out[*e.get()] = it,
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(out.len());
                out.push(it);
            }
        }
    }
    out
}

/// Every 10-Q and 10-K the company filed with EDGAR, oldest first, so a later
/// filing's restatement of an earlier period overwrites the original.
fn read_edgar(cache: &edgar::Cache, equity: equities::Ticker, reported_items: &mut Vec<equities::item::Reported>) -> Result<(), Box<dyn Error>> {
    let cik = cache.cik(equity.sec_symbol())?;
    eprintln!("{}: CIK {}", equity, cik);
    let filings = cache.filings(cik, &edgar::FORMS)?;
    let (mut skipped, mut missing) = (0, 0);
    for f in &filings {
        let name = format!("{} {} {} ({})", equity, f.form, f.report_date, f.accession);
        match cache.filing(f) {
            Ok(dir) => match equities::filing::new_reader(&dir, equity) {
                Ok(mut r) => {
                    read_statements(&mut r, &name, reported_items);
                    match r.process_cash_flow_statement() {
                        Ok(mut ret) => reported_items.append(&mut ret),
                        Err(e) => eprintln!("failed to process cash flow statement for {}; {}", name, e),
                    }
                    // Older filings' cover pages may not be fetched yet.
                    if let Ok(mut ret) = r.process_cover() {
                        reported_items.append(&mut ret);
                    }
                },
                Err(e) => eprintln!("failed to read {}: {}", name, e),
            },
            Err(e) => match e.downcast_ref::<edgar::EdgarError>() {
                Some(edgar::EdgarError::NoFilingSummary) => skipped += 1,
                Some(edgar::EdgarError::NotFetched(_)) => missing += 1,
                _ => eprintln!("failed to read {}: {}", name, e),
            },
        }
    }
    eprintln!("{}: read {} EDGAR filings, {} predate rendered statements{}", equity, filings.len() - skipped - missing, skipped,
        if missing > 0 { format!(", {} not fetched yet", missing) } else { String::new() });
    Ok(())
}

/// A Tokyo Stock Exchange ticker, read from EDINET.
fn is_edinet(t: equities::Ticker) -> bool {
    t.as_str().ends_with(".T")
}

fn read_edinet(cache: &edinet::Cache, equity: equities::Ticker, reported_items: &mut Vec<equities::item::Reported>) -> Result<(), Box<dyn Error>> {
    let (mut items, read, missing) = equities::edinet::read_company(cache, equity)?;
    eprintln!("{}: read {} EDINET reports{}", equity, read,
        if missing > 0 { format!(", {} not fetched yet", missing) } else { String::new() });
    reported_items.append(&mut items);
    Ok(())
}

fn read_statements(r: &mut impl Reader, name: &str, reported_items: &mut Vec<equities::item::Reported>) {
    match r.process_balance_sheet() {
        Ok(mut ret) => reported_items.append(&mut ret),
        Err(e) => eprintln!("failed to process balance sheet for {}; {}", name, e),
    }
    match r.process_income_statement() {
        Ok(mut ret) => reported_items.append(&mut ret),
        Err(e) => eprintln!("failed to process income statement for {}; {}", name, e),
    }
}

/// Print the series charted in Grafana, one row per (ticker, date), to check
/// the data before pushing it. Later reports restate earlier dates, so where a
/// (ticker, item, period, date) was reported more than once with different
/// values, the row is marked with '*'.
fn print_series(items: &[equities::item::Reported], cash_flow: bool) {
    use equities::item::Item;
    use equities::Period;
    use std::collections::BTreeMap;

    let cols: [(Item, Period, &str); 5] = if cash_flow { [
        (Item::NetCashProvidedByOperatingActivities, Period::ThreeMonths, "operating"),
        (Item::NetCashUsedInInvestingActivities, Period::ThreeMonths, "investing"),
        (Item::NetCashUsedInFinancingActivities, Period::ThreeMonths, "financing"),
        (Item::PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets, Period::ThreeMonths, "capex"),
        (Item::ChangeInCashAndCashEquivalents, Period::ThreeMonths, "chg cash"),
    ] } else { [
        (Item::Revenue, Period::ThreeMonths, "revenue"),
        (Item::NetIncome, Period::ThreeMonths, "net income"),
        (Item::CashAndCashEquivalents, Period::PointInTime, "cash"),
        (Item::MarketableSecurities, Period::PointInTime, "mkt sec"),
        (Item::Inventories, Period::PointInTime, "inventory"),
    ] };

    let mut table: BTreeMap<(String, chrono::NaiveDate), [Vec<f64>; 5]> = BTreeMap::new();
    for it in items {
        if let Some(i) = cols.iter().position(|(item, p, _)| *item == it.item && *p == it.p) {
            // Values in other currencies get their own rows, e.g. "TSM:TWD".
            let name = match it.currency {
                equities::Currency::USD => it.ticker.to_string(),
                c => format!("{}:{}", it.ticker, c),
            };
            let row = table.entry((name, it.date)).or_default();
            if !row[i].contains(&it.val) { row[i].push(it.val); }
        }
    }

    print!("{:<9}{:<12}", "ticker", "date");
    for (_, _, name) in &cols { print!("{:>12}", name); }
    println!();
    for ((ticker, date), row) in &table {
        print!("{:<9}{:<12}", ticker, date.to_string());
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
                "reported_item_v15,ticker={},item={},period={},currency={}{} value={} {}\n",
                // lowercase: the Grafana dashboards query ticker = 'tsla' / 'nvda'
                item.ticker.to_string().to_lowercase(),
                item.item,
                item.p,
                item.currency,
                // EDGAR values carry no source tag (they came first).
                if is_edinet(item.ticker) { ",source=edinet" } else { "" },
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
