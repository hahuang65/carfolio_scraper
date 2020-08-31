use scraper::element_ref::ElementRef;

use crate::error::Error;
use crate::{element_attr, inner_html};
use crate::Page;

const BASE_URL: &str = "https://carfolio.com/specifications";

#[derive(Clone)]
pub struct Make {
    pub name: String,
    pub country: String,
    pub url: String
}

fn extract_url(element: ElementRef) -> Result<String, Error> {
    let path = element_attr(element, "a.man", "href")?;
    Ok(format!("{}/{}", BASE_URL, path))
}

fn extract_name(element: ElementRef) -> Result<String, Error> {
    inner_html(element, "a.man")
}

fn extract_country(element: ElementRef) -> Result<String, Error> {
    inner_html(element, "div.footer")
}

pub(super) fn makes() -> Result<Vec<Make>, Error> {
    info!("Requesting Makes data");
    let page = Page::new(BASE_URL);

    page.elements("div.grid div[class^=\"m\"]").iter().map(|div| {
        debug!("div: {:?}", div.inner_html().trim());

        let url = extract_url(*div)?;
        let name = extract_name(*div)?;
        let country = match extract_country(*div) {
            Ok(country) => country,
            Err(_)  => {
                warn!("Unable to find country for make: {} ({})", name, url);
                "".to_string()
            }
        };
        info!("Make: {} ({}) - {}", name, country, url);

        Ok(Make { name: name, country: country, url: url })
    }).collect::<Result<Vec<Make>, Error>>()
}
