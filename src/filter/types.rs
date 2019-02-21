use std::fmt;
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

impl fmt::Display for ReleaseCountry {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "release_country = {}", self.release_country)
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

impl<'g> fmt::Display for Genres<'g> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let genres = self.genres.join(", ");
    write!(f, "genres @> {{{}}}", genres)
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

impl<'l> fmt::Display for Languages<'l> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let languages = self.languages.join(", ");
    write!(f, "languages @> {{{}}}", languages)
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

pub struct Writers<'w> {
  writers: Vec<&'w str>,
}

impl<'w> Writers<'w> {
  pub fn new(writers: &'w str) -> Result<Self, Error> {
    let writers: Vec<&'w str> = writers.split("|").collect();

    if writers.iter().all(|writer| RGX_NUM.is_match(writer)) {
      Ok(Writers { writers })
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

pub struct Artists<'a> {
  artists: Vec<&'a str>,
}

impl<'a> Artists<'a> {
  pub fn new(artists: &'a str) -> Result<Self, Error> {
    let artists: Vec<&'a str> = artists.split("|").collect();

    if artists.iter().all(|artist| RGX_NUM.is_match(artist)) {
      Ok(Artists { artists })
    } else {
      Err(Error::FilterInvalid)
    }
  }
}