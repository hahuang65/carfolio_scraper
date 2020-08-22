mod make;
mod model;

pub(crate) fn scrape() {
    make::makes().unwrap().iter()
        .map(|make| model::models(make.to_owned()).unwrap())
        .collect::<Vec<_>>();
}
