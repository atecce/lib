//! Connect an institution through Plaid Link, Plaid's login widget:
//!
//!     cargo run -p plaid --bin plaid-link
//!
//! Serves a page on 127.0.0.1 (this Mac only) that opens Link; the login
//! itself happens with the institution (Robinhood: its own OAuth window).
//! Link hands the page a public token, the page posts it back here, and it is
//! exchanged for the Item's access token, kept in ~/.plaid/items. Then exits.

use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

use plaid::{Client, Item};
use serde_json::Value;

const ADDRESS: &str = "127.0.0.1:8789";

fn page(link_token: &str) -> String {
    format!(r#"<!doctype html>
<html><head><meta charset="utf-8"><title>Connect a brokerage</title>
<style>body {{ font: 16px -apple-system, sans-serif; margin: 3em; color: #222 }} button {{ font: inherit; padding: .5em 1em }}</style>
<script src="https://cdn.plaid.com/link/v2/stable/link-initialize.js"></script></head>
<body><p id="status">Opening Plaid…</p><button id="open">Connect</button>
<script>
const status = document.getElementById('status');
const handler = Plaid.create({{
  token: {token},
  onSuccess: (public_token, metadata) => {{
    status.textContent = 'Saving the connection…';
    fetch('/exchange', {{ method: 'POST', headers: {{ 'Content-Type': 'application/json' }},
      body: JSON.stringify({{ public_token, institution: metadata.institution && metadata.institution.name }}) }})
      .then(r => r.text()).then(t => status.textContent = t);
  }},
  onExit: (err) => {{ status.textContent = err ? 'Not connected: ' + (err.display_message || err.error_message || err.error_code) : 'Closed without connecting.'; }},
}});
document.getElementById('open').onclick = () => handler.open();
handler.open();
</script></body></html>"#, token = serde_json::to_string(link_token).unwrap())
}

fn respond(stream: &mut impl Write, status: &str, kind: &str, body: &str) -> std::io::Result<()> {
    write!(stream, "HTTP/1.1 {}\r\nContent-Type: {}; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
           status, kind, body.len(), body)
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::from_env_file()?;
    let link_token = client.link_token()?;
    let listener = TcpListener::bind(ADDRESS)?;
    let url = format!("http://{}/", ADDRESS);
    eprintln!("Plaid ({}): open {} to connect", if client.is_production() { "production" } else { "sandbox" }, url);
    let _ = std::process::Command::new("open").arg(&url).status();

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;
        let mut length = 0usize;
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line)? == 0 || line == "\r\n" { break }
            if let Some(v) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                length = v.trim().parse().unwrap_or(0);
            }
        }
        let mut body = vec![0; length.min(1 << 16)];
        reader.read_exact(&mut body)?;

        match request_line.split_whitespace().take(2).collect::<Vec<_>>()[..] {
            ["GET", "/"] => respond(&mut stream, "200 OK", "text/html", &page(&link_token))?,
            ["POST", "/exchange"] => {
                let v: Value = serde_json::from_slice(&body)?;
                let public_token = v["public_token"].as_str().ok_or("no public_token")?;
                match client.exchange(public_token) {
                    Ok((access_token, item_id)) => {
                        let item = Item { item_id, access_token, institution: v["institution"].as_str().map(String::from) };
                        let path = item.save()?;
                        let name = item.institution.as_deref().unwrap_or("the institution");
                        eprintln!("connected {}: {}", name, path.display());
                        respond(&mut stream, "200 OK", "text/plain", &format!("Connected {}. You can close this tab.", name))?;
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("exchange failed: {}", e);
                        respond(&mut stream, "500 Internal Server Error", "text/plain", &format!("Not saved: {}", e))?;
                    }
                }
            }
            _ => respond(&mut stream, "404 Not Found", "text/plain", "not found")?,
        }
    }
    Ok(())
}
