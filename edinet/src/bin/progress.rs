//! Progress of the EDINET fetch of every listed company (edinet/nightly/all.sh),
//! read from the files it writes.
//!
//!     edinet-progress          # redraws every 5 seconds
//!     edinet-progress --once   # one snapshot
//!
//! The run's order is edinet/nightly/companies.txt, and the newest
//! log.all.<timestamp> gets a line per company as each finishes ("7203.T
//! トヨタ自動車株式会社: 41 reports, 0 new, 0 requests"; Toyota Jidōsha
//! Kabushiki-gaisha),
//! so how far it is, is the position in companies.txt of the last company
//! logged. Time left assumes the rest go at the pace of the last half hour
//! (while redrawing) or the run's average.

use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use chrono::{Local, NaiveDateTime, TimeZone};

const CHUNK: usize = 250;

fn main() -> Result<(), Box<dyn Error>> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("nightly");
    let once = std::env::args().any(|a| a == "--once");
    let mut history: Vec<(Instant, usize, u64)> = Vec::new();
    loop {
        let text = snapshot(&dir, &mut history).unwrap_or_else(|e| format!("{}", e));
        if once {
            println!("{}", text);
            return Ok(());
        }
        // Clear the screen and draw from the top.
        print!("\x1b[H\x1b[2J{}\n", text);
        std::io::Write::flush(&mut std::io::stdout())?;
        std::thread::sleep(Duration::from_secs(5));
    }
}

/// What a log line says about a company that finished.
struct Line<'a> {
    ticker: &'a str,
    name: &'a str,
    requests: u64,
    new: u64,
    failed: u64,
}

/// "7203.T トヨタ自動車株式会社: 41 reports, 0 new, 0 requests[, 1 failed]", or
/// "1234.T: no reports found" for a company that could not be fetched.
fn parse(line: &str) -> Option<Line<'_>> {
    let (head, rest) = line.split_once(": ")?;
    let (ticker, name) = head.split_once(' ').unwrap_or((head, ""));
    if !ticker.ends_with(".T") {
        return None;
    }
    let number_before = |word: &str| -> Option<u64> {
        let i = rest.find(word)?;
        rest[..i].trim_end().rsplit([' ', ',', '(']).next()?.parse().ok()
    };
    if rest.contains(" reports, ") {
        Some(Line { ticker, name, requests: number_before(" requests").unwrap_or(0), new: number_before(" new").unwrap_or(0),
            failed: number_before(" failed").unwrap_or(0) })
    } else if rest.starts_with("no reports found") {
        Some(Line { ticker, name, requests: 0, new: 0, failed: 1 })
    } else {
        None
    }
}

fn snapshot(dir: &Path, history: &mut Vec<(Instant, usize, u64)>) -> Result<String, Box<dyn Error>> {
    let log = newest_log(dir)?.ok_or("no run yet (edinet/nightly/log.all.*)")?;
    let name = log.file_name().unwrap().to_string_lossy().to_string();
    let started = NaiveDateTime::parse_from_str(&name["log.all.".len()..], "%Y%m%d-%H%M%S")?;
    let started = Local.from_local_datetime(&started).single().ok_or("bad start time")?;
    let order: HashMap<String, usize> = std::fs::read_to_string(dir.join("companies.txt")).unwrap_or_default()
        .lines().map(str::trim).filter(|l| !l.is_empty()).enumerate().map(|(i, t)| (t.to_string(), i)).collect();
    let total = order.len().max(1);

    let text = std::fs::read_to_string(&log)?;
    let (mut position, mut requests, mut new, mut failed) = (0usize, 0u64, 0u64, 0u64);
    let (mut now_on, mut now_name) = ("-", "");
    for line in text.lines() {
        let Some(l) = parse(line) else { continue };
        let Some(&i) = order.get(l.ticker) else { continue };
        requests += l.requests;
        new += l.new;
        failed += l.failed;
        position = position.max(i + 1);
        now_on = l.ticker;
        now_name = l.name;
    }
    let finished = text.contains(&format!("=== {} of {}", total, total));
    if finished {
        position = total;
    }

    // Pace: the last half hour's while redrawing, else the run's average.
    let now = Instant::now();
    history.push((now, position, requests));
    history.retain(|h| now.duration_since(h.0) <= Duration::from_secs(1800));
    let elapsed = (Local::now() - started).num_seconds().max(1) as f64;
    let (span, done, reqs) = match history.first() {
        Some(&(t0, p0, r0)) if now.duration_since(t0) > Duration::from_secs(60) && position > p0 =>
            (now.duration_since(t0).as_secs_f64(), (position - p0) as f64, (requests - r0) as f64),
        _ => (elapsed, position as f64, requests as f64),
    };
    let eta = if finished {
        "done".to_string()
    } else if done > 0.0 {
        let secs = (total - position) as f64 * span / done;
        let at = Local::now() + chrono::Duration::seconds(secs as i64);
        format!("{}, around {}", duration(secs), at.format("%a %H:%M"))
    } else {
        "-".to_string()
    };

    let chunk_done = if position == 0 { 0 } else { position - (position - 1) / CHUNK * CHUNK };
    let width = std::env::var("COLUMNS").ok().and_then(|c| c.parse::<usize>().ok()).unwrap_or(80).saturating_sub(30).clamp(20, 50);
    Ok([
        format!("EDINET    {} {}/{}", bar(position as f64 / total as f64, width), position, total),
        format!("chunk     {} {}/{}, last {} {}", bar(chunk_done as f64 / CHUNK as f64, width), chunk_done, CHUNK, now_on, now_name),
        format!("requests  {} ({:.1}/s), {} new reports, {} failed", thousands(requests), reqs / span, thousands(new), failed),
        format!("running   {} since {}, {} free", duration(elapsed), started.format("%a %H:%M"), free_space(dir)),
        format!("time left {}", eta),
    ].join("\n"))
}

fn newest_log(dir: &Path) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let mut logs: Vec<PathBuf> = std::fs::read_dir(dir)?.flatten().map(|e| e.path())
        .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("log.all.")))
        .collect();
    logs.sort();
    Ok(logs.pop())
}

fn bar(fraction: f64, width: usize) -> String {
    let filled = (fraction.clamp(0.0, 1.0) * width as f64).round() as usize;
    "█".repeat(filled) + &"░".repeat(width - filled)
}

fn duration(secs: f64) -> String {
    let s = secs as u64;
    format!("{}h{:02}m", s / 3600, s % 3600 / 60)
}

fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 { out.push(','); }
        out.push(c);
    }
    out
}

/// Free space on the disk holding `dir`, from df (no crate for statvfs).
fn free_space(dir: &Path) -> String {
    let out = std::process::Command::new("df").arg("-k").arg(dir).output().ok();
    let kb = out.and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.lines().nth(1).and_then(|l| l.split_whitespace().nth(3).and_then(|v| v.parse::<u64>().ok())));
    kb.map(|k| format!("{} GB", k / 1024 / 1024)).unwrap_or_else(|| "? GB".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_lines() {
        let l = parse("7203.T トヨタ自動車株式会社: 41 reports, 2 new, 3 requests, 1 failed").unwrap();
        assert_eq!((l.ticker, l.name, l.requests, l.new, l.failed), ("7203.T", "トヨタ自動車株式会社", 3, 2, 1));
        assert_eq!(parse("130A.T: no reports found").unwrap().failed, 1);
        assert!(parse("7203.T S100ABCD 有価証券報告書: error sending request").is_none());
        assert!(parse("=== 250 of 4460").is_none());
        assert!(parse("day lists: 1 fetched, 1 requests").is_none());
    }
}
