use scraper::element_ref::ElementRef;

use crate::error::AppError;

const BASE_URL: &str = "https://carfolio.com";

use super::make::Make;

pub struct Model {
    pub make: Make,
    pub name: String,
    pub year: String,
    pub url: String
}

fn extract_data(div: ElementRef) -> Result<ElementRef, AppError> {
    crate::find_elem(div, String::from("div.card-head a span.automobile"))
}

fn extract_url(div: ElementRef) -> Result<String, AppError> {
    let elem = crate::find_elem(div, String::from("div.card-head a"))?;
    let href = crate::find_attr(elem, String::from("href"))?;

    Ok(format!("{}/{}", BASE_URL, href))
}

fn extract_name(div: ElementRef) -> Result<String, AppError> {
    let elem = crate::find_elem(div, String::from("span.model.name"))?;
    Ok(elem.inner_html())
}

fn extract_year(div: ElementRef) -> Result<String, AppError> {
    let selectors = vec![String::from("span.Year"), String::from("span.model-year")];

    let data = extract_data(div)?;
    let elem = crate::search_elem(data, selectors)?;

    Ok(elem.inner_html())
}

#[tokio::main]
pub async fn models(make: Make) -> Result<Vec<Model>, AppError> {
    info!("Requesting Models data for {}", make.name);

    let html = crate::fetch_page(&make.url).await?;
    let selector = String::from("div.grid div.grid-card");

    crate::divs(&html, selector).iter().map(|div| {
        let name = extract_name(*div)?;
        let url = extract_url(*div)?;
        let year = match extract_year(*div) {
            Ok(year) => year,
            Err(_)   => {
                warn!("Unable to find year for model: {} {} ({})", make.name, name, url);
                "".to_string()
            }
        };
        info!("Model: {} {} {} - {}", year, make.name, name, url);

        Ok(Model { make: make.clone(), name: name, year: year, url: url })
    }).collect::<Result<Vec<Model>, AppError>>()
}
