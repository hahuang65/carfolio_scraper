use scraper::html::Html;
use scraper::Selector;
use scraper::element_ref::ElementRef;

use crate::error::AppError;
use crate::error::AppError::*;
use crate::error::StandardErrorType::*;

const BASE_URL: &str = "https://carfolio.com/specifications";

#[derive(Clone)]
pub struct Make {
    pub name: String,
    pub country: String,
    pub url: String
}

fn divs(html: &Html) -> Vec<ElementRef<'_>> {
    let selector = Selector::parse("div.grid div[class^=\"m\"]").unwrap();
    html.select(&selector).collect()
}

fn extract_link(div: ElementRef) -> Result<ElementRef, AppError> {
    let selector = Selector::parse("a.man").unwrap();

    match div.select(&selector).next() {
        Some(elem) => Ok(elem),
        None       => Err(StandardError(ElementNotFound))
    }
}

fn extract_url(div: ElementRef) -> Result<String, AppError> {
    let link = extract_link(div)?;

    match link.value().attr("href") {
        Some(href) => Ok(format!("{}/{}", BASE_URL, href.to_string())),
        None       => Err(StandardError(AttributeNotFound))
    }
}

fn extract_name(div: ElementRef) -> Result<String, AppError> {
    let link = extract_link(div)?;
    Ok(link.inner_html())
}

fn extract_country(div: ElementRef) -> Result<String, AppError> {
    let selector = Selector::parse("div.footer").unwrap();

    match div.select(&selector).next() {
        Some(elem) => Ok(elem.inner_html()),
        None       => Err(StandardError(ElementNotFound))
    }
}

#[tokio::main]
pub async fn makes() -> Result<Vec<Make>, AppError> {
    info!("Requesting Makes data");

    let html = match crate::fetch_page(BASE_URL).await {
        Ok(html) => html,
        Err(e)   => return Err(ReqwestError(e)),
    };

    divs(&html).iter().map(|div| {
        debug!("HTML div: {:?}", div.inner_html().trim());

        let url = match extract_url(*div) {
            Ok(url) => url,
            Err(e)  => return Err(e)
        };
        
        let name = match extract_name(*div) {
            Ok(url) => url,
            Err(e)  => return Err(e)
        };

        let country = match extract_country(*div) {
            Ok(url) => url,
            Err(_)  => {
                warn!("Unable to find country for make: {}", name);
                "".to_string()
            }
        };
        info!("Make: {} ({}) - {}", name, country, url);

        Ok(Make { name: name, country: country, url: url })
    }).collect::<Result<Vec<Make>, AppError>>()
}
