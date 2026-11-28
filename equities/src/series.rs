//! Values by day: a FRED series (a yield, an exchange rate), a stock's
//! closes, its splits. The caller reads them from wherever it keeps them
//! (fred's cache, tiingo's) and hands them over as this.

use chrono::{Datelike, NaiveDate};

/// Values by day, oldest first, one a day.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Series {
    points: Vec<(NaiveDate, f64)>,
}

impl Series {
    /// From (day, value) pairs in any order; a day given twice keeps its
    /// last value.
    pub fn new(points: impl IntoIterator<Item = (NaiveDate, f64)>) -> Series {
        let mut points: Vec<(NaiveDate, f64)> = points.into_iter().collect();
        points.sort_by_key(|(d, _)| *d);
        let mut deduped: Vec<(NaiveDate, f64)> = Vec::with_capacity(points.len());
        for p in points {
            match deduped.last_mut() {
                Some(last) if last.0 == p.0 => *last = p,
                _ => deduped.push(p),
            }
        }
        Series { points: deduped }
    }

    /// The (day, value) pairs, oldest first.
    pub fn points(&self) -> &[(NaiveDate, f64)] {
        &self.points
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// The value on `day` exactly.
    pub fn on(&self, day: NaiveDate) -> Option<f64> {
        self.points.binary_search_by_key(&day, |(d, _)| *d).ok().map(|i| self.points[i].1)
    }

    /// The value on `day` or the last day before it.
    pub fn at(&self, day: NaiveDate) -> Option<(NaiveDate, f64)> {
        let i = self.points.partition_point(|(d, _)| *d <= day);
        i.checked_sub(1).map(|i| self.points[i])
    }

    /// The first value dated in `year`'s `month`: a monthly series' value
    /// for the month (FRED dates a month by its first day).
    pub fn month(&self, year: i32, month: u32) -> Option<f64> {
        self.points.iter().find(|(d, _)| d.year() == year && d.month() == month).map(|(_, v)| *v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate { NaiveDate::from_ymd_opt(y, m, day).unwrap() }

    #[test]
    fn looks_up_by_day() {
        let s = Series::new([(d(2025, 3, 1), 3.0), (d(2025, 1, 1), 1.0), (d(2025, 1, 1), 1.5)]);
        assert_eq!(s.points(), &[(d(2025, 1, 1), 1.5), (d(2025, 3, 1), 3.0)]);
        assert_eq!(s.on(d(2025, 3, 1)), Some(3.0));
        assert_eq!(s.on(d(2025, 2, 1)), None);
        assert_eq!(s.at(d(2025, 2, 15)), Some((d(2025, 1, 1), 1.5)));
        assert_eq!(s.at(d(2024, 12, 31)), None);
        assert_eq!(s.month(2025, 3), Some(3.0));
        assert_eq!(s.month(2025, 2), None);
    }
}
