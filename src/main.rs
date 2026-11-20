use std::error::Error;
use std::fs;

use equities::Reader;

/// Identifies this client to the SEC, as EDGAR's fair access policy requires.
const SEC_USER_AGENT: &str = "Alessandro Tecce root@atec.pub";

fn main() -> Result<(), Box<dyn Error>> {
    let dry_run = std::env::args().any(|a| a == "--dry-run");
    // Skip EDGAR and use only the local xlsx files (offline, or to compare sources).
    let no_edgar = std::env::args().any(|a| a == "--no-edgar");
    let mut reported_items = Vec::new();

    let edgar = equities::edgar::Client::new(SEC_USER_AGENT, "equities/edgar")?;

    for equity in [equities::Ticker::NVDA, equities::Ticker::TSLA, equities::Ticker::AAPL, equities::Ticker::BRK, equities::Ticker::TSM] {
        // NVDA's own spreadsheets go back to 2001, before EDGAR rendered
        // statements, so read them first. EDGAR is the source of truth where
        // both cover a period: its values are read later and win downstream.
        if equity == equities::Ticker::NVDA || no_edgar {
            read_xlsx_dir(equity, &format!("equities/{}", equity), &mut reported_items)?;
        }
        if !no_edgar {
            read_edgar(&edgar, equity, &mut reported_items)?;
        }
    }

    equities::derive_quarters(&mut reported_items);

    // Every item read, one per line, sorted: for diffing the output of two versions.
    if std::env::args().any(|a| a == "--dump") {
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
        push_to_influx(&reported_items)?;
        println!("Successfully pushed {} items to InfluxDB", reported_items.len());
    }

    Ok(())
}

fn read_xlsx_dir(equity: equities::Ticker, dir: &str, reported_items: &mut Vec<equities::item::Reported>) -> Result<(), Box<dyn Error>> {
    let mut paths: Vec<_> = fs::read_dir(dir)?
        .filter_map(|f| f.ok())
        .map(|f| f.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("xlsx"))
        .collect();
    paths.sort();

    for path in paths {
        match equities::xlsx::new_reader(&path, equity) {
            Ok(mut r) => read_statements(&mut r, &format!("{:?}", path), reported_items),
            Err(e) => eprintln!("failed to construct new reader from path: {:?}; {}", path, e),
        }
    }
    Ok(())
}

/// Every 10-Q and 10-K the company filed with EDGAR, oldest first, so a later
/// filing's restatement of an earlier period overwrites the original.
fn read_edgar(edgar: &equities::edgar::Client, equity: equities::Ticker, reported_items: &mut Vec<equities::item::Reported>) -> Result<(), Box<dyn Error>> {
    let cik = edgar.cik(equity.sec_symbol())?;
    // Foreign filers (TSMC) file annual 20-Fs instead of 10-Ks, and no 10-Qs.
    let filings = edgar.filings(cik, &["10-Q", "10-K", "20-F"])?;
    let mut skipped = 0;
    for f in &filings {
        let name = format!("{} {} {} ({})", equity, f.form, f.report_date, f.accession);
        match edgar.fetch_filing(f) {
            Ok(dir) => match equities::filing::new_reader(&dir, equity) {
                Ok(mut r) => {
                    read_statements(&mut r, &name, reported_items);
                    match r.process_cash_flow_statement() {
                        Ok(mut ret) => reported_items.append(&mut ret),
                        Err(e) => eprintln!("failed to process cash flow statement for {}; {}", name, e),
                    }
                },
                Err(e) => eprintln!("failed to read {}: {}", name, e),
            },
            Err(e) => match e.downcast_ref::<equities::edgar::EdgarError>() {
                Some(equities::edgar::EdgarError::NoFilingSummary) => skipped += 1,
                _ => eprintln!("failed to fetch {}: {}", name, e),
            },
        }
    }
    eprintln!("{}: read {} EDGAR filings, {} predate rendered statements", equity, filings.len() - skipped, skipped);
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
                "reported_item_v13,ticker={},item={},period={},currency={} value={} {}\n",
                // lowercase: the Grafana dashboards query ticker = 'tsla' / 'nvda'
                item.ticker.to_string().to_lowercase(),
                item.item,
                item.p,
                item.currency,
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
