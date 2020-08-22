use scraper::html::Html;
use scraper::Selector;
use scraper::element_ref::ElementRef;

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

fn extract_link(div: ElementRef) -> ElementRef {
    let selector = Selector::parse("a.man").unwrap();
    div.select(&selector).next().unwrap()
}

fn extract_url(div: ElementRef) -> String {
    let link = extract_link(div);
    let path = link.value().attr("href").unwrap().to_string();

    format!("{}/{}", BASE_URL, path)
}

fn extract_name(div: ElementRef) -> String {
    let link = extract_link(div);
    link.inner_html()
}

fn extract_country(div: ElementRef) -> String {
    let selector = Selector::parse("div.footer").unwrap();
    div.select(&selector).next().unwrap().inner_html()
}

#[tokio::main]
pub async fn makes() -> Result<Vec<Make>, reqwest::Error> {
    info!("Requesting Makes data");

    let html = match crate::fetch_page(BASE_URL).await {
        Ok(html) => html,
        Err(e)   => return Err(e),
    };

    let makes = divs(&html).iter().map(|div| {
        let url = extract_url(*div);
        let name = extract_name(*div);
        let country = extract_country(*div);
        debug!("Make: {} ({}) - {}", name, country, url);

        Make { name: name, country: country, url: url }
    }).collect::<Vec<Make>>();

    Ok(makes)
}
