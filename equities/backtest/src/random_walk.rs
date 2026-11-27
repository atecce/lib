//! Fama (1965), "The Behavior of Stock-Market Prices": if prices follow a
//! random walk, successive changes are independent. Two tests of that.
//!
//! - **Serial correlation**: the correlation of each change with the one
//!   `lag` days later, daily, and over longer differencing intervals (Fama's
//!   4, 9 and 16 days: non-overlapping sums of daily changes). Independence
//!   means about zero; with n changes, a coefficient beyond about 2/√n is
//!   unlikely by chance.
//! - **Runs**: a run is a sequence of changes of the same sign (+, −, or 0).
//!   Dependence shows as fewer runs than chance would give (trends: long
//!   runs) or more (reversals). Fama's expected number and its variance,
//!   for three signs, are Wallis and Roberts's:
//!
//!   E(R) = [N(N+1) − Σnᵢ²] / N
//!   σ²(R) = [Σnᵢ²(Σnᵢ² + N(N+1)) − 2N Σnᵢ³ − N³] / [N²(N−1)]
//!
//!   with nᵢ the count of each sign and N the total.

/// The sample autocorrelation of `x` at `lag`.
pub fn serial_correlation(x: &[f64], lag: usize) -> f64 {
    if x.len() <= lag + 1 {
        return f64::NAN;
    }
    let m = crate::mean(x);
    let var: f64 = x.iter().map(|v| (v - m).powi(2)).sum();
    let cov: f64 = x.windows(lag + 1).map(|w| (w[0] - m) * (w[lag] - m)).sum();
    cov / var
}

/// Changes over `interval` days: sums of non-overlapping runs of `interval`
/// daily log changes (log changes add up).
pub fn differenced(daily: &[f64], interval: usize) -> Vec<f64> {
    daily.chunks_exact(interval).map(|c| c.iter().sum()).collect()
}

/// Runs of signs in `x`: (actual, expected, standard deviation of the
/// expected count under independence).
#[derive(Clone, Copy, Debug)]
pub struct Runs {
    pub actual: f64,
    pub expected: f64,
    pub sd: f64,
}

impl Runs {
    /// (actual − expected) / sd: about normal under independence.
    pub fn z(&self) -> f64 {
        (self.actual - self.expected) / self.sd
    }
}

pub fn runs(x: &[f64]) -> Runs {
    let sign = |v: f64| if v > 0.0 { 1 } else if v < 0.0 { -1 } else { 0 };
    let signs: Vec<i32> = x.iter().map(|&v| sign(v)).collect();
    let actual = if signs.is_empty() { 0 } else { 1 + signs.windows(2).filter(|w| w[0] != w[1]).count() };
    let n = signs.len() as f64;
    let counts = [-1, 0, 1].map(|s| signs.iter().filter(|&&v| v == s).count() as f64);
    let sum2: f64 = counts.iter().map(|c| c * c).sum();
    let sum3: f64 = counts.iter().map(|c| c * c * c).sum();
    let expected = (n * (n + 1.0) - sum2) / n;
    let var = (sum2 * (sum2 + n * (n + 1.0)) - 2.0 * n * sum3 - n.powi(3)) / (n * n * (n - 1.0));
    Runs { actual: actual as f64, expected, sd: var.sqrt() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correlations() {
        // Alternating: perfectly negatively correlated at lag 1.
        let x = [1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        assert!((serial_correlation(&x, 1) + 0.875).abs() < 1e-9);
        assert_eq!(differenced(&[1.0, 2.0, 3.0, 4.0, 5.0], 2), vec![3.0, 7.0]);
    }

    #[test]
    fn counts_runs() {
        let r = runs(&[1.0, 2.0, -1.0, -2.0, 0.0, 3.0]);
        assert_eq!(r.actual, 4.0);
        // Two signs, equally often, many draws: expected about N/2 + 1.
        let alternating: Vec<f64> = (0..1000).map(|i| if i % 2 == 0 { 1.0 } else { -1.0 }).collect();
        let r = runs(&alternating);
        assert!((r.expected - 501.0).abs() < 1.0);
        assert!(r.z() > 30.0); // far more runs than chance: reversals
    }
}
