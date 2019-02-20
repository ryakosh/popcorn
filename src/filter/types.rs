use crate::error::Error;
use crate::consts::{RGX_ALPHA, RGX_NUM};

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

pub struct Genres<'g> {
  genres: Vec<&'g str>,
}

impl<'g> Genres<'g> {
  pub fn new(genres: &'g str) -> Result<Self, Error> {
    let genres: Vec<&'g str> = genres.split("|").collect();

    if genres.iter().all(|genre| RGX_ALPHA.is_match(genre)) {
      Ok(Genres { genres })
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

pub struct Dierectors<'d> {
  dierectors: Vec<&'d str>,
}

impl<'d> Dierectors<'d> {
  pub fn new(dierectors: &'d str) -> Result<Self, Error> {
    let dierectors: Vec<&'d str> = dierectors.split("|").collect();

    if dierectors.iter().all(|dierector| RGX_NUM.is_match(dierector)) {
      Ok(Dierectors { dierectors })
    } else {
      Err(Error::FilterInvalid)
    }
  }
}