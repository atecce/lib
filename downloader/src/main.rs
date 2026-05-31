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
    let recent = data.filings.recent;

    fs::create_dir_all("equities/tsla")?;

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
