use crate::consts::{RGX_ALPHA, RGX_NUM};
use crate::error::Error;
use crate::filter::types::MoviesFilters;

#[derive(FromForm)]
pub struct MoviesQuery {
    search: Option<String>,
    limit: Option<i32>,
    page: Option<i32>,
    filters: Option<String>,
}

impl MoviesQuery {
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

    pub fn validate(&self) -> Result<&Self, Error> {
        self.validate_limit()?;
        self.validate_page()?;
        self.validate_filters()?;

        Ok(self)
    }

    fn validate_limit(&self) -> Result<(), Error> {
        if let Some(limit) = self.limit.as_ref() {
            if *limit >= 1 && *limit <= 20 {
                Ok(())
            } else {
                Err(Error::LimitInvalid)
            }
        } else {
            Ok(())
        }
    }
    fn validate_page(&self) -> Result<(), Error> {
        if let Some(page) = self.page.as_ref() {
            if *page >= 1 {
                Ok(())
            } else {
                Err(Error::PageInvalid)
            }
        } else {
            Ok(())
        }
    }
    fn validate_filters(&self) -> Result<(), Error> {
        if let Some(filters) = self.filters.as_ref() {
            for filter in filters.split(",") {
                let filter = filter.split(":").collect::<Vec<&str>>();

                match filter[0] {
                    "release_country" => {
                        if filter[1].len() != 2 {
                            return Err(Error::FilterInvalid);
                        }
                    }
                    "genres" | "languages" => {
                        let alphas: Vec<&str> = filter[1].split("|").collect();

                        if !alphas.iter().all(|alpha| RGX_ALPHA.is_match(alpha)) {
                            return Err(Error::FilterInvalid);
                        }
                    }
                    "directors" | "writers" | "stars" => {
                        let nums: Vec<&str> = filter[1].split("|").collect();

                        if !nums.iter().all(|mum| RGX_NUM.is_match(mum)) {
                            return Err(Error::FilterInvalid);
                        }
                    }
                    _ => {
                        return Err(Error::FilterInvalid);
                    }
                }
            }
            return Ok(());
        } else {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moviesquery_getters_work_correctly() {
        let test_search = Some(String::from("test"));
        let test_limit = Some(4);
        let test_page = None;
        let test_filters = Some(String::from("release_country:US,genres:Action|Advanture"));
        let query = MoviesQuery {
            search: test_search.clone(),
            limit: test_limit.clone(),
            page: test_page.clone(),
            filters: test_filters.clone(),
        };

        assert_eq!(query.search(), test_search.as_ref());
        assert_eq!(query.limit(), test_limit);
        assert_eq!(query.page(), test_page);
        assert_eq!(query.filters(), test_filters.as_ref());
    }

    #[test]
    fn moviesquery_validate_works_correctly() {
        let test_search = Some(String::from("test"));
        let test_limit = Some(4);
        let test_page = None;
        let test_filters = Some(String::from("release_country:US,genres:Action|Advanture"));
        let query = MoviesQuery {
            search: test_search.clone(),
            limit: test_limit.clone(),
            page: test_page.clone(),
            filters: test_filters.clone(),
        };

        if let Err(err) = query.validate() {
            panic!(format!("Err: {:?}", err));
        }

        let test_limit = Some(30);
        let query = MoviesQuery {
            search: test_search.clone(),
            limit: test_limit.clone(),
            page: test_page.clone(),
            filters: test_filters.clone(),
        };

        match query.validate() {
            Ok(_) => panic!("Limit is invalid"),
            Err(err) => assert_eq!(err, Error::LimitInvalid),
        }
    }
}
