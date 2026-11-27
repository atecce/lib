//! Load FRED series from the fred cache into Postgres (`fred_series`,
//! `fred_observation`):
//!
//!     store-fred                  # every series in the cache
//!     store-fred GS10 DGS3MO      # just these
//!
//! Titles come from the comments in `fred/rates.txt`. Applies the
//! migrations first, as `lib --postgres` does. Run after `fred-fetch`; only
//! new and revised observations are written.

use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cache = fred::Cache::new(fred::Cache::default_dir());
    let titles: HashMap<String, String> = fred::read_list(fred::default_list())
        .map(|l| l.into_iter().filter_map(|(id, note)| Some((id, note?))).collect())
        .unwrap_or_default();
    let mut ids: Vec<String> = std::env::args().skip(1).map(|a| a.to_uppercase()).collect();
    if ids.is_empty() {
        ids = cache.ids();
    }
    let mut store = store::Store::connect(&store::default_url())?;
    for m in store.migrate()? {
        eprintln!("applied {}", m);
    }
    let (mut series, mut written) = (0, 0);
    for id in &ids {
        let s = cache.series(id)?;
        let n = store.write_fred(&s, titles.get(id).map(|t| t.as_str()))?;
        if n > 0 {
            eprintln!("{}: {} observations written", id, n);
        }
        series += 1;
        written += n;
    }
    eprintln!("{} series, {} observations written", series, written);
    Ok(())
}
