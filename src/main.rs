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

struct Page {
    url: String,
    html: Html
}

impl Page {
    fn new(url: &str) -> Page {
        match Self::get_html(url) {
            Ok(html) => Self { url: String::from(url), html: html },
            Err(e)   => panic!(e)
        }
    }

    #[tokio::main]
    async fn get_html(url: &str) -> Result<Html, reqwest::Error> {
        info!("Scraping HTML from {}", url);
        let resp = REQWEST_CLIENT.get(url).send().await?;
        let body = resp.text().await?;
        
        Ok(Html::parse_document(&body))
    }

    fn elements(&self, selector: &str) -> Vec<ElementRef> {
        let selector = Selector::parse(&selector).unwrap();
        self.html.select(&selector).collect()
    }
}

fn element_within<'a>(element: ElementRef<'a>, selectors: Vec<&'_ str>) -> Result<ElementRef<'a>, AppError> {
    let elem = selectors.iter().find_map(|selector| {
        let selector = Selector::parse(&selector).unwrap();
        element.select(&selector).next()
    });

    match elem {
        Some(elem) => Ok(elem),
        None       => Err(StandardError(ElementNotFound))
    }
}

fn element_attr(element: ElementRef, selector: &str, attr: &str) -> Result<String, AppError> {
    let elem = element_within(element, vec![selector])?;

    match elem.value().attr(attr) {
        Some(attr) => Ok(attr.to_string()),
        None       => Err(StandardError(AttributeNotFound))
    }
}

fn inner_html(element: ElementRef, selector: &str) -> Result<String, AppError> {
    let elem = element_within(element, vec![selector])?;
    Ok(elem.inner_html())
}
