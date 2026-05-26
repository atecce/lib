use std::collections::HashMap;
use std::error::Error;
use std::fs;

use nvda::{new_reader, ReportedItem};

use chrono::NaiveDate;

#[derive(Debug)]
pub struct Observation {
    pub t: String,
    pub val: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reported_items = Vec::new();

    let paths: Vec<_> = fs::read_dir("./nvda")?
        .filter_map(|f| f.ok())
        .filter(|f| {
             let path = f.path();
             let ext = path.extension().and_then(|ext| ext.to_str());
             ext == Some("xlsx")
        })
        .map(|f| f.path())
        .collect();

    for path in paths {
        match new_reader(&path) {
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

    if !reported_items.is_empty() {
        push_to_influx(&reported_items)?;
        println!("Successfully pushed {} items to InfluxDB", reported_items.len());

        let mut series = HashMap::<nvda::Item, Vec<Observation>>::new();
        for reported_item in reported_items {
            series
                .entry(reported_item.item)
                .or_insert_with(Vec::new)
                .push(Observation {
                    t: reported_item.t,
                    val: reported_item.val,
                });
        }
        println!("{:#?}", series);
    }

    Ok(())
}

fn push_to_influx(items: &[ReportedItem]) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let auth_token = std::env::var("INFLUXDB3_AUTH_TOKEN")
        .map_err(|_| "INFLUXDB3_AUTH_TOKEN environment variable not set")?;

    for chunk in items.chunks(1000) {
        let mut payload = String::new();
        for item in chunk {
            let t = NaiveDate::parse_from_str(&item.t, "%Y-%m-%d").unwrap();
            let ts = t.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_nanos_opt().unwrap();
            payload.push_str(&format!(
                "reported_item_v3,item={},period={} value={} {}\n",
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
