pub mod types;

use crate::error::Error;
use types::MoviesFilters;

pub fn filter_movies(filters: &str) -> String {
    if !filters.is_empty() {
        let mut vec = vec![];
        for filter in filters.split(",") {
            vec.push(MoviesFilters::new(filter).to_string());
        }

        vec.join(" AND ")
    } else {
        "".to_string()
    }
}
