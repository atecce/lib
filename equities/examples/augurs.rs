use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

use augurs::outlier::{MADDetector, OutlierDetector};

use equities::reader::new_reader;

use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Observation {
    pub t: NaiveDate,
    pub val: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reported_items = Vec::new();

    for path in fs::read_dir("nvda")?
        .filter_map(|f| f.ok())
        .filter(|f| {
             let path = f.path();
             let ext = path.extension().and_then(|ext| ext.to_str());
             ext == Some("xlsx")
        })
        .map(|f| f.path()) {
        match new_reader(&path, equities::Ticker::NVDA) {
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
        reported_items.sort_by_cached_key(|item| (item.date, item.p, item.item));

        serde_json::to_writer_pretty(BufWriter::new(File::create("reported_items.json")?), &reported_items)?;

        let mut series = HashMap::<equities::item::Item, Vec<Observation>>::new();
        for reported_item in reported_items {
            series
                .entry(reported_item.item)
                .or_insert_with(Vec::new)
                .push(Observation {
                    t: reported_item.date,
                    val: reported_item.val,
                });
        }

        for item in &equities::item::Item::ALL {
            match series.get(item) {
                Some(observations) => {
                    let data = observations.clone().into_iter().map(|o| o.val).collect::<Vec<f64>>();
                    let vector: &[f64] = &data;
                    let matrix: &[&[f64]] = &[vector];

                    let detector = MADDetector::with_sensitivity(0.5).expect("mad detector failed to construct");
                    let processed = detector.preprocess(matrix).expect("input data is valid");
                    let outliers = detector.detect(&processed)?;

                    println!("outliers for {:?}", item);
                    for interval in &outliers.series_results[0].outlier_intervals.intervals {
                        println!("{:#?}", series[item][interval.start]);
                    }
                },
                None => println!("{:?} not found", item),
            }
        }
    }

    Ok(())
}
