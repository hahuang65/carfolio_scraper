use logging_timer::time;

mod make;
mod model;

use make::makes;
use model::models;

use crate::error::Error;

#[time("info")]
pub(crate) fn scrape() -> Result<(), Error> {
    match makes() {
        Ok(makes) => {
            makes.iter().map(|make| {
                match models(make.to_owned()) {
                    Ok(model) => Ok(model),
                    Err(e) => return Err(e)
                }
            }).count();
            Ok(())
        },
        Err(e) => return Err(e)
    }
}
