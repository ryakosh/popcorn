pub mod types;

use crate::error::Error;
use types::MoviesFilters;

pub fn filter_movies(filters: &str) -> Result<String, Error> {
    let mut vec = vec![];
    for filter in filters.split(",") {
        vec.push(MoviesFilters::new(filter)?.to_string());
    }

    let filters = vec.join(" AND ");
    Ok(filters)
}
