pub mod types;

use types::MoviesFilters;
use crate::error::Error;

pub fn filter_movies(filters: &str) -> Result<String, Error> {
  let mut vec = vec![];
  for filter in filters.split(",") {
    vec.push(MoviesFilters::new(filter)?.to_string());
  }

  Ok(vec.join(" AND "))
}