use std::fmt;
use crate::error::Error;
use crate::consts::{RGX_ALPHA, RGX_NUM};
use super::Filter;

pub enum ReleaseCountry {
  Type,
  Instance(String),
}

impl<'f> Filter<'f> for ReleaseCountry {
  fn new(&self, release_country: &str) -> Result<Box<Self>, Error> {
    if release_country.len() == 2 {
      Ok(Box::new(ReleaseCountry::Instance(release_country.to_uppercase())))
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

impl fmt::Display for ReleaseCountry {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let release_country = if let ReleaseCountry::Instance(release_country) = self {
      release_country.as_str()
    } else {
      ""
    };

    write!(f, "release_country = {}", release_country)
  }
}

pub enum Genres<'g> {
  Type,
  Instance(Vec<&'g str>),
}

impl<'g> Filter<'g> for Genres<'g> {
  fn new(&self, genres: &'g str) -> Result<Box<Self>, Error> {
    let genres: Vec<&'g str> = genres.split("|").collect();

    if genres.iter().all(|genre| RGX_ALPHA.is_match(genre)) {
      Ok(Box::new(Genres::Instance(genres)))
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

impl<'g> fmt::Display for Genres<'g> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let genres = if let Genres::Instance(genres) = self {
      genres.join(", ")
    } else {
      "".to_string()
    };

    write!(f, "genres @> {{{}}}", genres)
  }
}

pub enum Languages<'l> {
  Type,
  Instance(Vec<&'l str>),
}

impl<'l> Filter<'l> for Languages<'l> {
  fn new(&self, languages: &'l str) -> Result<Box<Self>, Error> {
    let languages: Vec<&'l str> = languages.split("|").collect();

    if languages.iter().all(|language| RGX_ALPHA.is_match(language)) {
      Ok(Box::new(Languages::Instance(languages)))
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

impl<'l> fmt::Display for Languages<'l> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let languages = if let Languages::Instance(languages) = self {
      languages.join(", ")
    } else {
      "".to_string()
    };

    write!(f, "languages @> {{{}}}", languages)
  }
}

pub enum Dierectors<'d> {
  Type,
  Instance(Vec<&'d str>),
}

impl<'d> Filter<'d> for Dierectors<'d> {
  fn new(&self, dierectors: &'d str) -> Result<Box<Self>, Error> {
    let dierectors: Vec<&'d str> = dierectors.split("|").collect();

    if dierectors.iter().all(|dierector| RGX_NUM.is_match(dierector)) {
      Ok(Box::new(Dierectors::Instance(dierectors)))
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

impl<'d> fmt::Display for Dierectors<'d> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let dierectors = if let Dierectors::Instance(dierectors) = self {
      dierectors.join(", ")
    } else {
      "".to_string()
    };

    write!(f, "dierectors @> {{{}}}", dierectors)
  }
}

pub enum Writers<'w> {
  Type,
  Instance(Vec<&'w str>),
}

impl<'w> Filter<'w> for Writers<'w> {
  fn new(&self, writers: &'w str) -> Result<Box<Self>, Error> {
    let writers: Vec<&'w str> = writers.split("|").collect();

    if writers.iter().all(|writer| RGX_NUM.is_match(writer)) {
      Ok(Box::new(Writers::Instance(writers)))
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

impl<'w> fmt::Display for Writers<'w> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let writers = if let Writers::Instance(writers) = self {
      writers.join(", ")
    } else {
      "".to_string()
    };

    write!(f, "writers @> {{{}}}", writers)
  }
}

pub enum Artists<'a> {
  Type,
  Instance(Vec<&'a str>),
}

impl<'a> Filter<'a> for Artists<'a> {
  fn new(&self, artists: &'a str) -> Result<Box<Self>, Error> {
    let artists: Vec<&'a str> = artists.split("|").collect();

    if artists.iter().all(|artist| RGX_NUM.is_match(artist)) {
      Ok(Box::new(Artists::Instance(artists)))
    } else {
      Err(Error::FilterInvalid)
    }
  }
}

impl<'a> fmt::Display for Artists<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let artists = if let Artists::Instance(artists) = self {
      artists.join(", ")
    } else {
      "".to_string()
    };

    write!(f, "artists @> {{{}}}", artists)
  }
}