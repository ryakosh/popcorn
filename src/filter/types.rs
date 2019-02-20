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