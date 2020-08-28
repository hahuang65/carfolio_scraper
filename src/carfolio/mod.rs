use std::time::Instant;

mod make;
mod model;

use crate::error::AppError;

pub(crate) fn scrape() -> Result<(), AppError> {
    let before = Instant::now();

    match make::makes() {
        Ok(makes) => {
            makes.iter().map(|make| {
                match model::models(make.to_owned()) {
                    Ok(model) => Ok(model),
                    Err(e) => return Err(e)
                }
            }).count()
        },
        Err(e) => return Err(e)
    };

    info!("Runtime: {:.2?}", before.elapsed());
    Ok(())
}
