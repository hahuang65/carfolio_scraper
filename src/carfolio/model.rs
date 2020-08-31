use scraper::element_ref::ElementRef;

use crate::error::Error;
use crate::{element_attr, element_within, inner_html};
use crate::Page;

const BASE_URL: &str = "https://carfolio.com";

use super::make::Make;

pub struct Model {
    pub make: Make,
    pub name: String,
    pub year: String,
    pub url: String
}

fn extract_url(element: ElementRef) -> Result<String, Error> {
    let path = element_attr(element, "div.card-head a", "href")?;
    Ok(format!("{}/{}", BASE_URL, path))
}

fn extract_name(element: ElementRef) -> Result<String, Error> {
    inner_html(element, "span.model.name")
}

fn extract_year(element: ElementRef) -> Result<String, Error> {
    let span = element_within(element, vec!["div.card-head a span.automobile"])?;
    let elem = element_within(span, vec!["span.Year", "span.model-year"])?;
    Ok(elem.inner_html())
}

pub(super) fn models(make: Make) -> Result<Vec<Model>, Error> {
    info!("Requesting Models data for {}", make.name);
    let page = Page::new(&make.url);

    page.elements("div.grid div.grid-card").iter().map(|div| {
        debug!("div: {:?}", div.inner_html().trim());

        let url = extract_url(*div)?;
        let name = extract_name(*div)?;
        let year = match extract_year(*div) {
            Ok(year) => year,
            Err(_)   => {
                warn!("Unable to find year for model: {} {} ({})", make.name, name, url);
                "".to_string()
            }
        };
        info!("Model: {} {} {} - {}", year, make.name, name, url);

        Ok(Model { make: make.clone(), name: name, year: year, url: url })
    }).collect::<Result<Vec<Model>, Error>>()
}
