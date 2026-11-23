//! Progress of a load into the financials database: the companies written in
//! the current run against the list being loaded, and each table's rows and
//! how fast they change.
//!
//!     store-progress [LIST]          # redraws every 5 seconds
//!     store-progress --once [LIST]
//!
//! LIST defaults to equities/sp500/all.txt (tickers separated by spaces,
//! commas or newlines, `#` comments). The current run is the latest stretch
//! of `listing.written_at` without a gap of 15 minutes; the rates are
//! Postgres's own counts of rows inserted and deleted (pg_stat_user_tables),
//! since a rewrite replaces rows without changing the totals.

use std::collections::HashSet;
use std::error::Error;
use std::path::Path;
use std::time::{Duration, Instant};

const TABLES: [&str; 6] = ["issuer", "listing", "filing", "fact", "price", "market"];

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let once = args.iter().any(|a| a == "--once");
    let list = args.iter().find(|a| !a.starts_with("--")).cloned()
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../sp500/all.txt").display().to_string());
    let text = std::fs::read_to_string(&list).unwrap_or_default();
    let wanted: HashSet<String> = text.lines()
        .map(|l| l.split('#').next().unwrap_or(""))
        .flat_map(|l| l.split([' ', '\t', ',']).filter(|t| !t.is_empty()).map(|t| t.to_uppercase()).collect::<Vec<_>>())
        .collect();

    let mut db = postgres::Client::connect(&store::default_url(), postgres::NoTls)?;
    let mut history: Vec<(Instant, i64, i64)> = Vec::new();
    loop {
        let text = snapshot(&mut db, &wanted, &mut history)?;
        if once {
            println!("{}", text);
            return Ok(());
        }
        print!("\x1b[H\x1b[2J{}\n", text);
        std::io::Write::flush(&mut std::io::stdout())?;
        std::thread::sleep(Duration::from_secs(5));
    }
}

fn snapshot(db: &mut postgres::Client, wanted: &HashSet<String>, history: &mut Vec<(Instant, i64, i64)>)
    -> Result<String, Box<dyn Error>> {
    // The current run: the latest stretch of writes without a 15-minute gap.
    let start: Option<chrono::DateTime<chrono::Utc>> = db.query_one(
        "WITH w AS (SELECT written_at, lag(written_at) OVER (ORDER BY written_at) AS prev
                    FROM listing WHERE written_at IS NOT NULL)
         SELECT max(written_at) FROM w WHERE prev IS NULL OR written_at - prev > interval '15 minutes'", &[])?.get(0);
    let written: Vec<(String, chrono::DateTime<chrono::Utc>)> = match start {
        Some(s) => db.query("SELECT ticker, written_at FROM listing WHERE written_at >= $1 ORDER BY written_at", &[&s])?
            .iter().map(|r| (r.get(0), r.get(1))).collect(),
        None => Vec::new(),
    };
    let done = written.iter().filter(|(t, _)| wanted.contains(t)).count();
    let total = wanted.len().max(1);

    let mut lines = Vec::new();
    if let (Some(s), Some((last, at))) = (start, written.last()) {
        let elapsed = (*at - s).num_seconds().max(1) as f64;
        let left = if done > 0 && done < total {
            let secs = (total - done) as f64 * elapsed / done as f64;
            format!(", about {}m left", (secs / 60.0) as u64)
        } else if done >= total { ", done".into() } else { String::new() };
        let local = |t: &chrono::DateTime<chrono::Utc>| t.with_timezone(&chrono::Local).format("%H:%M:%S").to_string();
        lines.push(format!("companies {} {}/{}{}", bar(done as f64 / total as f64, 40), done, total, left));
        lines.push(format!("          run since {}, last {} at {}", local(&s), last, local(at)));
    } else {
        lines.push("no run recorded yet (listing.written_at)".into());
    }
    lines.push(String::new());

    let counts: Vec<i64> = TABLES.iter()
        .map(|t| db.query_one(&format!("SELECT count(*) FROM {}", t), &[]).map(|r| r.get(0)))
        .collect::<Result<_, _>>()?;
    let churn: (i64, i64) = {
        let r = db.query_one("SELECT coalesce(sum(n_tup_ins), 0)::bigint, coalesce(sum(n_tup_del), 0)::bigint
                              FROM pg_stat_user_tables WHERE relname = 'fact'", &[])?;
        (r.get(0), r.get(1))
    };
    let now = Instant::now();
    history.push((now, churn.0, churn.1));
    history.retain(|h| now.duration_since(h.0) <= Duration::from_secs(60));
    for (t, n) in TABLES.iter().zip(&counts) {
        let rate = match (*t, history.first()) {
            ("fact", Some(&(t0, ins, del))) if now.duration_since(t0) > Duration::from_secs(1) => {
                let s = now.duration_since(t0).as_secs_f64();
                format!("  +{:.0}/s written, −{:.0}/s replaced", (churn.0 - ins) as f64 / s, (churn.1 - del) as f64 / s)
            },
            _ => String::new(),
        };
        lines.push(format!("  {:<8} {:>12}{}", t, thousands(*n), rate));
    }
    lines.push(format!("\n{}", chrono::Local::now().format("%H:%M:%S")));
    Ok(lines.join("\n"))
}

fn bar(fraction: f64, width: usize) -> String {
    let filled = (fraction.clamp(0.0, 1.0) * width as f64).round() as usize;
    "█".repeat(filled) + &"░".repeat(width - filled)
}

fn thousands(n: i64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 { out.push(','); }
        out.push(c);
    }
    out
}
