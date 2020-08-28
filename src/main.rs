#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate pretty_env_logger;

use scraper::html::Html;
use scraper::Selector;
use scraper::element_ref::ElementRef;

mod error;
mod carfolio;

use error::AppError;
use error::AppError::StandardError;
use error::StandardErrorType::{ElementNotFound, AttributeNotFound};

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

fn divs(html: &Html, selector: String) -> Vec<ElementRef<'_>> {
    let selector = Selector::parse(&selector).unwrap();
    html.select(&selector).collect()
}

fn find_elem(div: ElementRef, selector: String) -> Result<ElementRef, AppError> {
    let selector = Selector::parse(&selector).unwrap();

    match div.select(&selector).next() {
        Some(elem) => Ok(elem),
        None       => Err(StandardError(ElementNotFound))
    }
}

fn search_elem(div: ElementRef, selectors: Vec<String>) -> Result<ElementRef, AppError> {
    let elem = selectors.iter().find_map(|selector| {
        match find_elem(div, selector.clone()) {
            Ok(elem) => Some(elem),
            Err(_)   => None
        }
    });

    match elem {
        Some(elem) => Ok(elem),
        None       => Err(StandardError(ElementNotFound))
    }
}

fn find_attr(div: ElementRef, attr_name: String) -> Result<String, AppError> {
    match div.value().attr(&attr_name) {
        Some(attr) => Ok(attr.to_string()),
        None       => Err(StandardError(AttributeNotFound))
    }
}
