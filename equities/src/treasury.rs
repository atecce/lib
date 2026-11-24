//! The 10-year Treasury yield, for a cost of equity that moves with rates:
//! r_t = the yield at the end of year t − 1 + an equity risk premium.
//!
//! Source: Board of Governors of the Federal Reserve System, "Market Yield on
//! U.S. Treasury Securities at 10-Year Constant Maturity" (H.15), via FRED,
//! series GS10 (monthly averages), https://fred.stlouisfed.org/series/GS10.
//! Each entry is December's average, in percent.

/// (year, December average yield, %), 1961–2025.
pub const TEN_YEAR_DECEMBER: [(i32, f64); 65] = [
    (1961, 4.06),
    (1962, 3.86),
    (1963, 4.13),
    (1964, 4.18),
    (1965, 4.62),
    (1966, 4.84),
    (1967, 5.70),
    (1968, 6.03),
    (1969, 7.65),
    (1970, 6.39),
    (1971, 5.93),
    (1972, 6.36),
    (1973, 6.74),
    (1974, 7.43),
    (1975, 8.00),
    (1976, 6.87),
    (1977, 7.69),
    (1978, 9.01),
    (1979, 10.39),
    (1980, 12.84),
    (1981, 13.72),
    (1982, 10.54),
    (1983, 11.83),
    (1984, 11.50),
    (1985, 9.26),
    (1986, 7.11),
    (1987, 8.99),
    (1988, 9.11),
    (1989, 7.84),
    (1990, 8.08),
    (1991, 7.09),
    (1992, 6.77),
    (1993, 5.77),
    (1994, 7.81),
    (1995, 5.71),
    (1996, 6.30),
    (1997, 5.81),
    (1998, 4.65),
    (1999, 6.28),
    (2000, 5.24),
    (2001, 5.09),
    (2002, 4.03),
    (2003, 4.27),
    (2004, 4.23),
    (2005, 4.47),
    (2006, 4.56),
    (2007, 4.10),
    (2008, 2.42),
    (2009, 3.59),
    (2010, 3.29),
    (2011, 1.98),
    (2012, 1.72),
    (2013, 2.90),
    (2014, 2.21),
    (2015, 2.24),
    (2016, 2.49),
    (2017, 2.40),
    (2018, 2.83),
    (2019, 1.86),
    (2020, 0.93),
    (2021, 1.47),
    (2022, 3.62),
    (2023, 4.02),
    (2024, 4.39),
    (2025, 4.14),
];

/// The 10-year yield at the end of `year`, as a fraction.
pub fn ten_year(year: i32) -> Option<f64> {
    TEN_YEAR_DECEMBER.iter().find(|e| e.0 == year).map(|e| e.1 / 100.0)
}

/// The cost of equity for `year`: the yield at the end of the year before
/// (known when the year begins) plus `premium`.
pub fn cost_of_equity(year: i32, premium: f64) -> Option<f64> {
    ten_year(year - 1).map(|y| y + premium)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_is_every_year() {
        for (i, e) in TEN_YEAR_DECEMBER.iter().enumerate() {
            assert_eq!(e.0, 1961 + i as i32);
        }
        assert!((ten_year(1981).unwrap() - 0.1372).abs() < 1e-12);
        assert!((cost_of_equity(2021, 0.04).unwrap() - 0.0493).abs() < 1e-12);
    }
}
