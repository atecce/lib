//! The 10-year Treasury yield, for a cost of equity that moves with rates:
//! r_t = the yield at the end of year t − 1 + an equity risk premium.
//!
//! From the FRED cache (`fred-fetch GS10`): "Market Yield on U.S. Treasury
//! Securities at 10-Year Constant Maturity", the Federal Reserve's H.15,
//! monthly averages in percent, from April 1953. A year's yield is its
//! December's.

use std::error::Error;

pub const SERIES: &str = "GS10";

pub struct TenYear {
    series: fred::Series,
}

impl TenYear {
    pub fn load() -> Result<TenYear, Box<dyn Error>> {
        Ok(TenYear { series: fred::Cache::new(fred::Cache::default_dir()).series(SERIES)? })
    }

    /// The yield at the end of `year` (December's average), as a fraction.
    pub fn at_end_of(&self, year: i32) -> Option<f64> {
        self.series.month(year, 12).map(|y| y / 100.0)
    }

    /// The cost of equity for `year`: the yield at the end of the year before
    /// (known when the year begins) plus `premium`.
    pub fn cost_of_equity(&self, year: i32, premium: f64) -> Option<f64> {
        self.at_end_of(year - 1).map(|y| y + premium)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_cached_series() {
        let t = TenYear::load().expect("fred/cache/GS10.csv (tracked as a fixture)");
        assert!((t.at_end_of(1981).unwrap() - 0.1372).abs() < 1e-12);
        assert!((t.cost_of_equity(2021, 0.04).unwrap() - 0.0493).abs() < 1e-12);
        assert!((1954..=2025).all(|y| t.at_end_of(y).is_some()));
    }
}
