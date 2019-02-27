use std::fmt;
use crate::error::Error;
use crate::consts::{RGX_ALPHA, RGX_NUM};

pub enum MoviesFilters<'m> {
  Alpha(&'m str, String),
  Alphas(&'m str, Vec<&'m str>),
  Nums(&'m str, Vec<&'m str>),
}

impl<'m> MoviesFilters<'m> {
  pub fn new(filter: &'m str) -> Result<Self, Error> {
    let filter = filter.split(":").collect::<Vec<&'m str>>();
    match filter[0] {
      "release_country" => {
        if filter[1].len() == 2 {
          Ok(MoviesFilters::Alpha(filter[0], filter[1].to_uppercase()))
        } else {
          Err(Error::FilterInvalid)
        }
      },
      "genres" | "languages" => {
        let alphas: Vec<&'m str> = filter[1].split("|").collect();

        if alphas.iter().all(|alpha| RGX_ALPHA.is_match(alpha)) {
          Ok(MoviesFilters::Alphas(filter[0], alphas))
        } else {
          Err(Error::FilterInvalid)
        }
      },
      "directors" | "writers" | "stars" => {
        let nums: Vec<&'m str> = filter[1].split("|").collect();

        if nums.iter().all(|mum| RGX_NUM.is_match(mum)) {
          Ok(MoviesFilters::Nums(filter[0], nums))
        } else {
          Err(Error::FilterInvalid)
        }
      },
      _ => { Err(Error::FilterInvalid) },
    }
  }
}

impl<'m> fmt::Display for MoviesFilters<'m> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MoviesFilters::Alpha(k, v) => {
        write!(f, "{} = '{}'", k , v)
      },
      MoviesFilters::Alphas(k, v) => {
        write!(f, "{} @> '{{{}}}'", k, v.join(", "))
      },
      MoviesFilters::Nums(k, v) => {
        write!(f, "{} @> '{{{}}}'", k, v.join(", "))
      }
    }
  }
}