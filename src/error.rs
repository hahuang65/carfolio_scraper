use scraper::element_ref::ElementRef;

#[derive(Debug)]
pub(crate) enum Error {
    // Errors from crates
    ReqwestError(reqwest::Error),
    // Errors from this crate
    ScraperError(ScraperErrorKind)
}

#[derive(Debug)]
pub(crate) enum ScraperErrorKind {
    ElementError(ElementNotFound),
    AttributeError(AttributeNotFound),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ReqwestError(ref err) => err.fmt(f),
            Error::ScraperError(ref err) => write!(f, "{:?}", err)
        }
    }
}

#[derive(Debug)]
pub(crate) struct ElementNotFound {
    html: String,
    elements: String
}

impl ElementNotFound {
    pub fn new(html: ElementRef, elements: Vec<&str>) -> ElementNotFound {
        let html = String::from(html.inner_html().trim());
        let elements = elements.join(", ");
        
        error!("Unable to find elements '{}' within HTML:\n{}", elements, html);
        ElementNotFound {
            html: String::from(html),
            elements: elements
        }
    }
}

#[derive(Debug)]
pub(crate) struct AttributeNotFound {
    element: String,
    attribute: String
}

impl AttributeNotFound {
    pub fn new(element: ElementRef, attribute: &str) -> AttributeNotFound {
        let element = String::from(element.html().trim());
        
        error!("Unable to find attribute '{}' within HTML:\n{}", element, attribute);
        AttributeNotFound {
            element: element,
            attribute: String::from(attribute)
        }
    }
}
