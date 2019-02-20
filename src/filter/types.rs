use crate::error::Error;
use crate::consts::RGX_ALPHA;

pub struct ReleaseCountry {
  release_country: String,
}

impl ReleaseCountry {
  pub fn new(release_country: &str) -> Result<Self, Error> {
    if release_country.len() == 2 {
      Ok(ReleaseCountry {
        release_country: release_country.to_uppercase(),
      })
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

pub struct languages<'g> {
  languages: Vec<&'g str>,
}

impl<'g> languages<'g> {
  pub fn new(languages: &'g str) -> Result<Self, Error> {
    let languages: Vec<&'g str> = languages.split("|").collect();

    if languages.iter().all(|genre| RGX_ALPHA.is_match(genre)) {
      Ok(languages { languages })
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

pub struct Languages<'l> {
  languages: Vec<&'l str>,
}

impl<'l> Languages<'l> {
  pub fn new(languages: &'l str) -> Result<Self, Error> {
    let languages: Vec<&'l str> = languages.split("|").collect();

    if languages.iter().all(|language| RGX_ALPHA.is_match(language)) {
      Ok(Languages { languages })
    } else {
      Err(Error::FilterInvalid)
    }
  }
}