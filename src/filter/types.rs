use std::fmt;
use crate::error::Error;
use crate::consts::{RGX_ALPHA, RGX_NUM};
use super::Filter;

pub struct ReleaseCountry {
  release_country: String,
}

impl<'f> Filter<'f> for ReleaseCountry {
  fn new(release_country: &str) -> Result<Box<Self>, Error> {
    if release_country.len() == 2 {
      Ok(Box::new(ReleaseCountry {
        release_country: release_country.to_uppercase(),
      }))
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

impl<'g> Filter<'g> for Genres<'g> {
  fn new(genres: &'g str) -> Result<Box<Self>, Error> {
    let genres: Vec<&'g str> = genres.split("|").collect();

    if genres.iter().all(|genre| RGX_ALPHA.is_match(genre)) {
      Ok(Box::new(Genres { genres }))
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

impl<'l> Filter<'l> for Languages<'l> {
  fn new(languages: &'l str) -> Result<Box<Self>, Error> {
    let languages: Vec<&'l str> = languages.split("|").collect();

    if languages.iter().all(|language| RGX_ALPHA.is_match(language)) {
      Ok(Box::new(Languages { languages }))
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

impl<'d> Filter<'d> for Dierectors<'d> {
  fn new(dierectors: &'d str) -> Result<Box<Self>, Error> {
    let dierectors: Vec<&'d str> = dierectors.split("|").collect();

    if dierectors.iter().all(|dierector| RGX_NUM.is_match(dierector)) {
      Ok(Box::new(Dierectors { dierectors }))
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

impl<'d> fmt::Display for Dierectors<'d> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let dierectors = self.dierectors.join(", ");
    write!(f, "dierectors @> {{{}}}", dierectors)
  }
}

pub struct Writers<'w> {
  writers: Vec<&'w str>,
}

impl<'w> Filter<'w> for Writers<'w> {
  fn new(writers: &'w str) -> Result<Box<Self>, Error> {
    let writers: Vec<&'w str> = writers.split("|").collect();

    if writers.iter().all(|writer| RGX_NUM.is_match(writer)) {
      Ok(Box::new(Writers { writers }))
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

impl<'w> fmt::Display for Writers<'w> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let writers = self.writers.join(", ");
    write!(f, "writers @> {{{}}}", writers)
  }
}

pub struct Artists<'a> {
  artists: Vec<&'a str>,
}

impl<'a> Filter<'a> for Artists<'a> {
  fn new(artists: &'a str) -> Result<Box<Self>, Error> {
    let artists: Vec<&'a str> = artists.split("|").collect();

    if artists.iter().all(|artist| RGX_NUM.is_match(artist)) {
      Ok(Box::new(Artists { artists }))
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

impl<'a> fmt::Display for Artists<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let artists = self.artists.join(", ");
    write!(f, "artists @> {{{}}}", artists)
  }
}