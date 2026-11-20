use std::error::Error;
use std::fs;
use std::io::copy;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Submissions {
    filings: Filings,
}

#[derive(Deserialize, Debug)]
struct Filings {
    recent: RecentFilings,
    /// Older filings beyond the ~1000 in `recent`, paged into separate JSON files.
    #[serde(default)]
    files: Vec<FilingsFile>,
}

#[derive(Deserialize, Debug)]
struct FilingsFile {
    name: String,
}

#[derive(Deserialize, Debug)]
struct RecentFilings {
    #[serde(rename = "accessionNumber")]
    accession_number: Vec<String>,
    form: Vec<String>,
    #[serde(rename = "reportDate")]
    report_date: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cik = "0001318605"; // Tesla
    let user_agent = "Alessandro Tecce root@atec.pub";
    let client = Client::builder()
        .user_agent(user_agent)
        .build()?;

    let url = format!("https://data.sec.gov/submissions/CIK{}.json", cik);
    println!("Fetching filings for CIK {}...", cik);
    let res = client.get(&url).send()?;
    
    if !res.status().is_success() {
        return Err(format!("Failed to fetch submissions: {}", res.status()).into());
    }

    let data: Submissions = res.json()?;

    fs::create_dir_all("equities/tsla")?;

    download(&client, cik, &data.filings.recent)?;

    // `recent` only covers the latest ~1000 filings; Tesla's many Form 4s push
    // older 10-Q/10-Ks out of it, so walk the paged history too.
    for file in &data.filings.files {
        let url = format!("https://data.sec.gov/submissions/{}", file.name);
        println!("Fetching older filings from {}...", file.name);
        let page: RecentFilings = client.get(&url).send()?.error_for_status()?.json()?;
        download(&client, cik, &page)?;
        sleep(Duration::from_millis(200));
    }

    Ok(())
}

fn download(client: &Client, cik: &str, recent: &RecentFilings) -> Result<(), Box<dyn Error>> {
    for i in 0..recent.form.len() {
        let form = &recent.form[i];
        if form == "10-K" || form == "10-Q" {
            let accession = &recent.accession_number[i];
            let accession_clean = accession.replace("-", "");
            let report_date = &recent.report_date[i];
            
            let filename = format!("equities/tsla/{}_{}.xlsx", report_date, form.replace("-", ""));
            let path = Path::new(&filename);
            
            if path.exists() {
                println!("File {} already exists, skipping.", filename);
                continue;
            }

            let excel_url = format!(
                "https://www.sec.gov/Archives/edgar/data/{}/{}/Financial_Report.xlsx",
                cik.trim_start_matches('0'),
                accession_clean
            );

            println!("Downloading {} for {}...", form, report_date);
            let mut file_res = client.get(&excel_url).send()?;
            
            if file_res.status().is_success() {
                let mut out = fs::File::create(path)?;
                copy(&mut file_res, &mut out)?;
                println!("Saved as {}", filename);
            } else {
                eprintln!("Failed to download {} (Status: {})", excel_url, file_res.status());
            }

            // Respect SEC rate limits
            sleep(Duration::from_millis(200));
        }
    }

    Ok(())
}
