use anyhow::Result;
use reqwest;
use yt_scrape::{extract_json_from_script, parse_streams};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://www.youtube.com/@ParliamentofKenyaChannel/streams";
    let response = reqwest::get(url).await?.text().await?;

    match extract_json_from_script(&response) {
        Ok(json) => {
            let dat = parse_streams(&json);
            println!("{:#?}", dat);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    Ok(())
}
