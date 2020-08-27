use std::time::Instant;

mod make;
mod model;

pub(crate) fn scrape() {
    let before = Instant::now();

    make::makes();//.unwrap().iter()
        // .map(|make| model::models(make.to_owned()).unwrap())
        // .collect::<Vec<_>>();
    
    info!("Runtime: {:.2?}", before.elapsed())
}
