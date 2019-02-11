use crate::error::*;

#[derive(FromForm)]
pub struct MoviesQuery {
  search: Option<String>,
  limit: Option<i32>,
  page: Option<i32>,
}

impl MoviesQuery {
  pub fn new(search: Option<String>,
         limit: Option<i32>,
         page: Option<i32>) -> Result<Self, Errors> {

    let is_limit = match limit {
      Some(limit) => limit >= 1 && limit <= 20,
      None => true,
    };
    let is_page = match page {
      Some(page) => page >= 1,
      None => true,
    };

    if is_limit && is_page {
      Ok(MoviesQuery { search, limit, page })
    } else {
      let mut errors = Vec::new();

      if let false = is_limit { errors.push(Error::LimitInvalid) }
      if let false = is_page { errors.push(Error::PageInvalid) }

      Err(errors)
    }
  }

  pub fn search(&self) -> Option<&String> { self.search.as_ref() }
  pub fn limit(&self) -> Option<i32> { self.limit }
  pub fn page(&self) -> Option<i32> { self.page }
}