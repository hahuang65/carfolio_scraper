#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate pretty_env_logger;

use logging_timer::time;

use scraper::html::Html;
use scraper::Selector;
use scraper::element_ref::ElementRef;

mod error;
mod carfolio;

use error::Result;
use error::Error::ScraperError;
use error::ScraperErrorKind::{ElementError, AttributeError};
use error::{ElementNotFound, AttributeNotFound};

fn main() -> Result<()> {
    pretty_env_logger::init();

    match carfolio::scrape() {
        Err(e) => {
            error!("{}", e);
            Ok(())
        }
        _      => Ok(())
    }
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
            Err(e)   => panic!("{}", e)
        }
    }

    #[tokio::main]
    #[time("info")]
    async fn get_html(url: &str) -> Result<Html> {
        info!("Fetching HTML from {}", url);
        let resp = REQWEST_CLIENT.get(url).send().await?;
        let body = resp.text().await?;
        
        Ok(Html::parse_document(&body))
    }

    fn elements(&self, selector_str: &str) -> Vec<ElementRef> {
        let selector = Selector::parse(&selector_str).unwrap();
        
        let results: Vec<ElementRef> = self.html.select(&selector).collect();

        if results.is_empty() {
            warn!("No elements found for selector '{}' on {}", selector_str, self.url)
        }

        results
    }
}

fn element_within<'a>(element: ElementRef<'a>, selectors: &[&str]) -> Result<ElementRef<'a>> {
    let elem = selectors.iter().find_map(|selector| {
        let selector = Selector::parse(&selector).unwrap();
        element.select(&selector).next()
    });

    match elem {
        Some(elem) => Ok(elem),
        None       => Err(ScraperError(ElementError(ElementNotFound::new(element, selectors))))
    }
}

fn element_attr(element: ElementRef, selector: &str, attr: &str) -> Result<String> {
    let elem = element_within(element, &[selector])?;

    match elem.value().attr(attr) {
        Some(attr) => Ok(attr.to_string()),
        None       => Err(ScraperError(AttributeError(AttributeNotFound::new(elem, attr))))
    }
}

fn inner_html(element: ElementRef, selector: &str) -> Result<String> {
    let elem = element_within(element, &[selector])?;
    Ok(elem.inner_html())
}

fn inner_text(element: ElementRef) -> String {
    element.text().into_iter().collect::<Vec<&str>>().join("")
}
