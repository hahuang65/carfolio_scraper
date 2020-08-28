use scraper::element_ref::ElementRef;

use crate::error::AppError;

const BASE_URL: &str = "https://carfolio.com/specifications";

#[derive(Clone)]
pub struct Make {
    pub name: String,
    pub country: String,
    pub url: String
}

fn extract_link(div: ElementRef) -> Result<ElementRef, AppError> {
    crate::find_elem(div, String::from("a.man"))
}

fn extract_url(div: ElementRef) -> Result<String, AppError> {
    let link = extract_link(div)?;
    let href = crate::find_attr(link, String::from("href"))?;

    Ok(format!("{}/{}", BASE_URL, href))
}

fn extract_name(div: ElementRef) -> Result<String, AppError> {
    let link = extract_link(div)?;
    Ok(link.inner_html())
}

fn extract_country(div: ElementRef) -> Result<String, AppError> {
    let elem = crate::find_elem(div, String::from("div.footer"))?;
    Ok(elem.inner_html())
}

#[tokio::main]
pub async fn makes() -> Result<Vec<Make>, AppError> {
    info!("Requesting Makes data");

    let html = crate::fetch_page(BASE_URL).await?;
    let selector = String::from("div.grid div[class^=\"m\"]");

    crate::divs(&html, selector).iter().map(|div| {
        debug!("HTML div: {:?}", div.inner_html().trim());

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
    }).collect::<Result<Vec<Make>, AppError>>()
}
