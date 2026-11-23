//! Alexander's filter rules (1961, 1964), as Fama and Blume (1966) tested
//! them: "If the daily closing price of a particular security moves up at
//! least x per cent, buy and hold the security until its price moves down at
//! least x per cent from a subsequent high, at which time simultaneously sell
//! and go short. The short position is maintained until the daily closing
//! price rises at least x per cent above a subsequent low, at which time one
//! covers and buys."
//!
//! Trades happen at the close that crosses the filter, not at the filter
//! price itself: Alexander's early results assumed the latter, which prices
//! that gap past the filter make impossible (Mandelbrot's objection).
//!
//! Returns are in log terms on adjusted closes, so dividends accrue to the
//! long side and are owed by the short side, as they would be.

/// The outcome of a filter on one stock.
#[derive(Clone, Copy, Debug, Default)]
pub struct Outcome {
    /// Total log return of following the rule, costs taken out.
    pub rule: f64,
    /// Of that, from the long and the short positions (before costs).
    pub long: f64,
    pub short: f64,
    /// Total log return of buying and holding over the same days.
    pub buy_and_hold: f64,
    /// Positions taken (each a sale and a purchase after the first).
    pub trades: usize,
    /// Days from the first signal to the end, over which both are measured.
    pub days: usize,
}

/// Run filter `x` (0.05 for 5%) over adjusted closes, with `cost` taken from
/// the log return for each position change (0.01: one per cent a switch,
/// roughly Fama and Blume's commissions on a sale plus a purchase; under
/// 0.001 for a large stock today).
pub fn run(prices: &[f64], x: f64, cost: f64) -> Outcome {
    #[derive(PartialEq, Clone, Copy)]
    enum Position { None, Long, Short }
    let mut pos = Position::None;
    // The low since the last sale (or the start), and the high since the
    // last purchase: the reference points the filter is measured from.
    let (mut low, mut high) = (prices[0], prices[0]);
    let mut out = Outcome::default();
    let mut start = None;

    for t in 1..prices.len() {
        let (prev, p) = (prices[t - 1], prices[t]);
        let r = (p / prev).ln();
        match pos {
            Position::Long => out.long += r,
            Position::Short => out.short -= r,
            Position::None => {}
        }
        if pos != Position::None {
            out.buy_and_hold += r;
        }
        let up = p >= low * (1.0 + x);
        let down = p <= high * (1.0 - x);
        let switch_to = match pos {
            Position::None if up => Some(Position::Long),
            Position::None if down => Some(Position::Short),
            Position::Long if down => Some(Position::Short),
            Position::Short if up => Some(Position::Long),
            _ => None,
        };
        if let Some(next) = switch_to {
            pos = next;
            out.trades += 1;
            start.get_or_insert(t);
            (low, high) = (p, p);
        }
        low = low.min(p);
        high = high.max(p);
    }
    out.days = start.map_or(0, |s| prices.len() - 1 - s);
    // The first position is one purchase or short sale; each switch after is
    // a sale and a purchase.
    let switches = out.trades.saturating_sub(1) as f64 + if out.trades > 0 { 0.5 } else { 0.0 };
    out.rule = out.long + out.short - cost * switches;
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_a_trend() {
        // A steady rise: the rule goes long once and earns nearly what holding
        // does from the signal on.
        let up: Vec<f64> = (0..100).map(|i| 100.0 * 1.01f64.powi(i)).collect();
        let o = run(&up, 0.05, 0.0);
        assert_eq!(o.trades, 1);
        assert!((o.rule - o.buy_and_hold).abs() < 1e-9 && o.rule > 0.8);
    }

    #[test]
    fn whipsawed_by_reversals() {
        // Up 10%, down 10%, repeatedly: a 5% filter buys high and sells low.
        let mut p = vec![100.0];
        for i in 0..40 {
            let last = *p.last().unwrap();
            p.push(if i % 2 == 0 { last * 1.1 } else { last / 1.1 });
        }
        let o = run(&p, 0.05, 0.0);
        assert!(o.trades > 10);
        assert!(o.rule < o.buy_and_hold);
    }
}
