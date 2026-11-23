//! Progress of fetching a list of companies from MOPS: each company's
//! quarters on disk against the quarters there are, from the cache itself
//! (a company's line in the nightly log comes only when it's done, about
//! 25 minutes each).
//!
//!     mops-progress [--once] [LIST]     # default equities/taiwan/all.txt; redraws every 10 s
//!
//! A quarter counts as done when its report is cached, or MOPS was found
//! not to have it (a company listed after 2013 has none before).

use std::collections::HashSet;
use std::error::Error;
use std::path::Path;
use std::time::{Duration, Instant};

use chrono::Datelike;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let once = args.iter().any(|a| a == "--once");
    let list = args.iter().find(|a| !a.starts_with("--")).cloned()
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../equities/taiwan/all.txt").display().to_string());
    let text = std::fs::read_to_string(&list)?;
    let mut seen = HashSet::new();
    let codes: Vec<String> = text.lines()
        .map(|l| l.split('#').next().unwrap_or(""))
        .flat_map(|l| l.split([' ', '\t', ',']).map(str::to_string).collect::<Vec<_>>())
        .filter(|t| t.to_uppercase().ends_with(".TW"))
        .map(|t| mops::code(&t))
        .filter(|c| seen.insert(c.clone()))
        .collect();
    let cache = mops::Cache::new(mops::Cache::default_dir());
    let mut history: Vec<(Instant, usize)> = Vec::new();
    loop {
        let text = snapshot(&cache, &codes, &mut history);
        if once {
            println!("{}", text);
            return Ok(());
        }
        print!("\x1b[H\x1b[2J{}\n", text);
        std::io::Write::flush(&mut std::io::stdout())?;
        std::thread::sleep(Duration::from_secs(10));
    }
}

/// The quarters that have ended, 2013 Q1 on.
fn quarters() -> usize {
    let today = chrono::Local::now().date_naive();
    (mops::FIRST_YEAR..=today.year())
        .flat_map(|y| (1..=4).map(move |q| (y, q)))
        .filter(|&(y, q)| mops::quarter_end(y, q) < today)
        .count()
}

fn snapshot(cache: &mops::Cache, codes: &[String], history: &mut Vec<(Instant, usize)>) -> String {
    let expected = quarters();
    let done_of = |code: &str| std::fs::read_dir(cache.dir().join(code)).map(|d| d.flatten()
        .filter(|e| { let n = e.file_name(); let n = n.to_string_lossy(); n.ends_with(".html") || n.ends_with(".none") })
        .count()).unwrap_or(0).min(expected);
    let per: Vec<(String, usize)> = codes.iter().map(|c| (c.clone(), done_of(c))).collect();
    let done: usize = per.iter().map(|(_, n)| n).sum();
    let total = (codes.len() * expected).max(1);
    let complete = per.iter().filter(|(_, n)| *n >= expected).count();
    let current = per.iter().find(|(_, n)| *n < expected);

    let now = Instant::now();
    history.push((now, done));
    history.retain(|h| now.duration_since(h.0) <= Duration::from_secs(1800));
    let eta = match history.first() {
        Some(&(t0, d0)) if done > d0 && now.duration_since(t0) > Duration::from_secs(60) => {
            let per_quarter = now.duration_since(t0).as_secs_f64() / (done - d0) as f64;
            let secs = (total - done) as f64 * per_quarter;
            let at = chrono::Local::now() + chrono::Duration::seconds(secs as i64);
            format!("{}h{:02}m, around {} ({:.0} s a report)", secs as u64 / 3600, secs as u64 % 3600 / 60,
                at.format("%a %H:%M"), per_quarter)
        },
        _ if done >= total => "done".into(),
        _ => "measuring (redraw for an estimate)".into(),
    };
    let width = std::env::var("COLUMNS").ok().and_then(|c| c.parse::<usize>().ok()).unwrap_or(80).saturating_sub(30).clamp(20, 50);
    let mut lines = vec![
        format!("Taiwan    {} {}/{} reports", bar(done as f64 / total as f64, width), done, total),
        format!("companies {} {}/{} complete", bar(complete as f64 / codes.len().max(1) as f64, width), complete, codes.len()),
    ];
    if let Some((code, n)) = current {
        lines.push(format!("now       {}.TW  {} {}/{} quarters", code, bar(*n as f64 / expected as f64, 20), n, expected));
    }
    lines.push(format!("time left {}", eta));
    lines.join("\n")
}

fn bar(fraction: f64, width: usize) -> String {
    let filled = (fraction.clamp(0.0, 1.0) * width as f64).round() as usize;
    "█".repeat(filled) + &"░".repeat(width - filled)
}
