extern crate pretty_env_logger;
#[macro_use] extern crate log;

use scraper::html::Html;

mod carfolio;

fn main() {
    pretty_env_logger::init();

    carfolio::scrape();
}

async fn fetch_page(url: &str) -> Result<Html, reqwest::Error> {
    info!("Scraping HTML from {}", url);
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    
    Ok(Html::parse_document(&body))
}
