use crate::error::Error;

pub struct ReleaseCountry<'r> {
  release_country: &'r str,
}

impl<'r> ReleaseCountry<'r> {
  fn new(release_country: &'r str) -> Result<Self, Error> {
    if release_country.len() == 2 {
      Ok(ReleaseCountry {
        release_country: release_country.to_uppercase(),
      })
    } else {
      Err(Error::InvalidFilter)
    }
  }
}