use logging_timer::time;

mod make;
mod model;

use crate::error::AppError;

#[time("info")]
pub(crate) fn scrape() -> Result<(), AppError> {
    match make::makes() {
        Ok(makes) => {
            makes.iter().map(|make| {
                match model::models(make.to_owned()) {
                    Ok(model) => Ok(model),
                    Err(e) => return Err(e)
                }
            }).count();
            Ok(())
        },
        Err(e) => return Err(e)
    }
}
