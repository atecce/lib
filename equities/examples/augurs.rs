use std::collections::HashMap;
use std::error::Error;
use std::fs;

use augurs::outlier::{MADDetector, OutlierDetector};

use equities::nvda::new_reader;

use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Observation {
    pub t: String,
    pub val: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reported_items = Vec::new();

    let paths: Vec<_> = fs::read_dir("nvda")?
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
        reported_items.sort_by_cached_key(|item| {
                NaiveDate::parse_from_str(&item.t, "&Y-&m-%d")
                    .unwrap_or(NaiveDate::MIN)
        });

        let mut series = HashMap::<equities::Item, Vec<Observation>>::new();
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

        let data = series[&equities::Item::MarketableSecurities].clone().into_iter().map(|o| o.val).collect::<Vec<f64>>();
        let vector: &[f64] = &data;
        let matrix: &[&[f64]] = &[vector];

        let detector = MADDetector::with_sensitivity(0.5).expect("mad detector failed to construct");
        let processed = detector.preprocess(matrix).expect("input data is valid");
        let outliers = detector.detect(&processed);
        println!("{:#?}", outliers);
    }

    Ok(())
}
