use crate::error::*;

#[derive(FromForm)]
pub struct MoviesQuery {
    search: Option<String>,
    limit: Option<i32>,
    page: Option<i32>,
    filters: Option<String>,
}

impl MoviesQuery {
    pub fn new(
        search: Option<String>,
        limit: Option<i32>,
        page: Option<i32>,
        filters: Option<String>,
    ) -> Result<Self, Errors> {
        let is_limit = match limit {
            Some(limit) => limit >= 1 && limit <= 20,
            None => true,
        };
        let is_page = match page {
            Some(page) => page >= 1,
            None => true,
        };

        if is_limit && is_page {
            Ok(MoviesQuery {
                search,
                limit,
                page,
                filters,
            })
        } else {
            let mut errors = Vec::new();

            if let false = is_limit {
                errors.push(Error::LimitInvalid)
            }
            if let false = is_page {
                errors.push(Error::PageInvalid)
            }

            Err(errors)
        }
    }

    pub fn search(&self) -> Option<&String> {
        self.search.as_ref()
    }
    pub fn limit(&self) -> Option<i32> {
        self.limit
    }
    pub fn page(&self) -> Option<i32> {
        self.page
    }
    pub fn filters(&self) -> Option<&String> {
        self.filters.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moviesquery_new_is_valid() {
        let test_search = Some(String::from("test"));
        let test_limit = Some(5);
        let test_page = Some(3);
        let test_filters = Some(String::from("genres:Adventure|Action,release_country:US"));

        let test_moviesquery = MoviesQuery::new(test_search.clone(),
            test_limit,
            test_page,
            test_filters.clone());

        match test_moviesquery {
            Ok(test_moviesquery) => {
                assert_eq!(test_search, test_moviesquery.search);
                assert_eq!(test_limit, test_moviesquery.limit);
                assert_eq!(test_page, test_moviesquery.page);
                assert_eq!(test_filters, test_moviesquery.filters);
            },
            Err(Errors) => panic!("{:?}", Errors)
        };
    }
}