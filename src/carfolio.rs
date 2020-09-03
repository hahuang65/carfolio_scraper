use logging_timer::time;
use scraper::element_ref::ElementRef;

use crate::error::Error;
use crate::{element_attr, element_within, inner_html};
use crate::Page;

static BASE_URL: &str = "https://carfolio.com";
lazy_static! {
    static ref MAKES: std::collections::BTreeSet<&'static str> = [
        "Acura",
        "Alfa Romeo",
        "Ariel",
        "Aston Martin",
        "Audi",
        "BAC",
        "BMW",
        "Bugatti",
        "Buick",
        "Cadillac",
        "Caterham",
        "Chevrolet",
        "Chrysler",
        "Dodge",
        "Ferrari",
        "Fiat",
        "Ford",
        "Honda",
        "Hyundai",
        "Infiniti",
        "Jaguar",
        "Jeep",
        "Kia",
        "Koenigsegg",
        "Lamborghini",
        "Land Rover",
        "Lexus",
        "Lincoln",
        "Lotus",
        "Maserati",
        "Mazda",
        "McLaren",
        "Mercedes-Benz",
        "MINI",
        "Mitsubishi",
        "Nissan",
        "Oldsmobile",
        "Pagani",
        "Plymouth",
        "Polestar",
        "Pontiac",
        "Porsche",
        "Rolls-Royce",
        "Saab",
        "Saturn",
        "Scion",
        "Shelby",
        "Shelby Super Cars",
        "smart",
        "Subaru",
        "Suzuki",
        "Tesla",
        "Toyota",
        "Volkswagen",
        "Volvo",
        "Zenvo"
    ].iter().cloned().collect();
}

#[time("info")]
pub(crate) fn scrape() -> Result<(), Error> {
    let makes_page = Page::new(&format!("{}/specifications", BASE_URL));

    let make_links = make_links(makes_page)?;

    for link in &make_links {
        let make_page = Page::new(link);

        let _model_links = model_links(make_page)?;
    };
    Ok(())
}

fn extract_make_url(element: ElementRef) -> Result<String, Error> {
    let path = element_attr(element, "a.man", "href")?;
    Ok(format!("{}/specifications/{}", BASE_URL, path))
}

fn extract_make_name(element: ElementRef) -> Result<String, Error> {
    inner_html(element, "a.man")
}

fn extract_make_country(element: ElementRef) -> Result<String, Error> {
    inner_html(element, "div.footer")
}

#[time("info")]
fn make_links(page: Page) -> Result<Vec<String>, Error> {
    info!("Parsing for Make links...");

    let mut links = vec![];

    for div in page.elements("div.grid div[class^=\"m\"]") {
        debug!("HTML: {:?}", div.inner_html().trim());

        let name = extract_make_name(div)?;

        if MAKES.contains(name.as_str()) {
            let url = extract_make_url(div)?;
            let country = extract_make_country(div)?;
            info!("Link found for Make: {} ({}) - {}", name, country, url);

            links.push(url);
        }
    }

    Ok(links)
}

fn extract_model_url(element: ElementRef) -> Result<String, Error> {
    let path = element_attr(element, "div.card-head a", "href")?;
    Ok(format!("{}/{}", BASE_URL, path))
}

fn extract_model_make(element: ElementRef) -> Result<String, Error> {
    inner_html(element, "div.manufacturer h2")
}

fn extract_model_name(element: ElementRef) -> Result<String, Error> {
    inner_html(element, "span.model.name")
}

fn extract_model_year(element: ElementRef) -> Result<String, Error> {
    let span = element_within(element, vec!["div.card-head a span.automobile"])?;
    let elem = element_within(span, vec!["span.Year", "span.model-year"])?;
    Ok(elem.inner_html())
}

#[time("info")]
fn model_links(page: Page) -> Result<Vec<String>, Error> {
    let make = extract_model_make(page.html.root_element())?;
    info!("Parsing for Model links for {}...", make);

    page.elements("div.grid div.grid-card").iter().map(|div| {
        debug!("HTML: {:?}", div.inner_html().trim());

        let url = extract_model_url(*div)?;
        let name = match extract_model_name(*div) {
            Ok(name) => name,
            Err(_)   => {
                warn!("Unable to find name for model: {}", url);
                "".to_string()
            }
        };
        let year = match extract_model_year(*div) {
            Ok(year) => year,
            Err(_)   => {
                warn!("Unable to find year for model: {}", url);
                "".to_string()
            }
        };
        info!("Link found for Model: {} {} {} - {}", year, make, name, url);

        Ok(url)
    }).collect::<Result<Vec<String>, Error>>()
}
