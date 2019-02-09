use crate::error::*;

pub struct MoviesQuery {
  search: Option<String>,
  limit: Option<u8>,
  page: Option<u32>,
}

impl MoviesQuery {
  fn new(search: Option<String>,
         limit: Option<u8>,
         page: Option<u32>) -> Result<Self, Errors> {

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
}