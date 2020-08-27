use scraper::html::Html;
use scraper::Selector;
use scraper::element_ref::ElementRef;

const BASE_URL: &str = "https://carfolio.com";

use super::make::Make;

pub struct Model {
    pub make: Make,
    pub name: String,
    pub year: String,
    pub url: String
}

fn divs(html: &Html) -> Vec<ElementRef<'_>> {
    let selector = Selector::parse("div.grid div.grid-card").unwrap();
    html.select(&selector).collect()
}

fn extract_data(div: ElementRef) -> ElementRef {
    let selector = Selector::parse("div.card-head a span.automobile").unwrap();
    div.select(&selector).next().unwrap()
}

fn extract_url(div: ElementRef) -> String {
    let selector = Selector::parse("div.card-head a").unwrap();
    let link = div.select(&selector).next().unwrap();
    let path = link.value().attr("href").unwrap().to_string();

    format!("{}/{}", BASE_URL, path)
}

fn extract_name(div: ElementRef) -> String {
    let selector = Selector::parse("span.model.name").unwrap();

    let data = extract_data(div);
    match data.select(&selector).next() {
        Some(inner) => inner.inner_html(),
        None        => "".to_string()
    }
}

fn extract_year(div: ElementRef) -> String {
    let selector = Selector::parse("span.Year").unwrap();
    let alt_selector = Selector::parse("span.model-year").unwrap();
    let data = extract_data(div);

    match data.select(&selector).next() {
        Some(inner) => inner.inner_html(),
        None        => match data.select(&alt_selector).next() {
                         Some(inner) => inner.inner_html(),
                         None        => {
                             error!("Could not find year info in {:?}", data.inner_html());
                             "".to_string()
                         }
                       }
    }
}

#[tokio::main]
pub async fn models(make: Make) -> Result<Vec<Model>, reqwest::Error> {
    info!("Requesting Models data for {}", make.name);

    let html = match crate::fetch_page(&make.url).await {
        Ok(html)  => html,
        Err(e)    => return Err(e),
    };

    let models = divs(&html).iter().map(|div| {
        let year = extract_year(*div);
        let name = extract_name(*div);
        let url = extract_url(*div);

        debug!("Model: {} {} {} - {}", year, make.name, name, url);
        Model {
            make: make.clone(),
            name: name,
            year: year,
            url: url
        }
    }).collect::<Vec<Model>>();

    Ok(models)
}
