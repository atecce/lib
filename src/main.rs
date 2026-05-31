use std::error::Error;
use std::fs;

use equities::reader::new_reader;

use chrono::NaiveDate;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reported_items = Vec::new();

    let equities = vec!["nvda", "tsla"];

    for equity in equities {
        let dir = format!("equities/{}", equity);
        let paths: Vec<_> = fs::read_dir(&dir)?
            .filter_map(|f| f.ok())
            .filter(|f| {
                 let path = f.path();
                 let ext = path.extension().and_then(|ext| ext.to_str());
                 ext == Some("xlsx")
            })
            .map(|f| f.path())
            .collect();

        for path in paths {
            match new_reader(&path, equity.to_string()) {
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
                Err(e) => eprintln!("failed to construst new reader from path: {:?}; {}", path, e),
            }
        }
    }

    if !reported_items.is_empty() {
        push_to_influx(&reported_items)?;
        println!("Successfully pushed {} items to InfluxDB", reported_items.len());
    }

    Ok(())
}

fn push_to_influx(items: &[equities::ReportedItem]) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let auth_token = std::env::var("INFLUXDB3_AUTH_TOKEN")
        .map_err(|_| "INFLUXDB3_AUTH_TOKEN environment variable not set")?;

    for chunk in items.chunks(1000) {
        let mut payload = String::new();
        for item in chunk {
            let t = NaiveDate::parse_from_str(&item.t, "%Y-%m-%d").unwrap();
            let ts = t.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_nanos_opt().unwrap();
            payload.push_str(&format!(
                "reported_item_v11,ticker={},item={},period={} value={} {}\n",
                item.ticker,
                item.item.as_str(),
                item.p.as_str(),
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
