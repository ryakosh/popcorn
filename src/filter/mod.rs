pub mod types;

use types::MoviesFilters;

pub fn filter_movies(filters: &str) -> String {
    if !filters.is_empty() {
        let mut vec = vec![];
        for filter in filters.split(',') {
            vec.push(MoviesFilters::new(filter).to_string());
        }

        vec.join(" AND ")
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_movies_works_correctly() {
        let filters = "release_country:US,genres:Action|Adventure";
        let result = String::from("release_country = 'US' AND genres @> '{Action, Adventure}'");
        assert_eq!(filter_movies(filters), result);
    }
}
