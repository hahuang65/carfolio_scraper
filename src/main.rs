#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate pretty_env_logger;

use scraper::html::Html;

mod error;
mod carfolio;

fn main() {
    pretty_env_logger::init();

    carfolio::scrape();
}

lazy_static! {
    static ref REQWEST_CLIENT: reqwest::Client = reqwest::Client::new();
}

async fn fetch_page(url: &str) -> Result<Html, reqwest::Error> {
    info!("Scraping HTML from {}", url);
    let resp = REQWEST_CLIENT.get(url).send().await?;
    let body = resp.text().await?;
    
    Ok(Html::parse_document(&body))
}
