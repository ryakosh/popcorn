use crate::consts::{RGX_ALPHA, RGX_NUM};
use crate::error::Error;
use std::fmt;

pub enum MoviesFilters<'m> {
    Alpha(&'m str, String),
    Alphas(&'m str, Vec<&'m str>),
    Nums(&'m str, Vec<&'m str>),
}

impl<'m> MoviesFilters<'m> {
    pub fn new(filter: &'m str) -> Self {
        let filter = filter.split(":").collect::<Vec<&'m str>>();
        match filter[0] {
            "release_country" => MoviesFilters::Alpha(filter[0], filter[1].to_uppercase()),
            "genres" | "languages" => {
                let alphas: Vec<&'m str> = filter[1].split("|").collect();

                MoviesFilters::Alphas(filter[0], alphas)
            }
            "directors" | "writers" | "stars" => {
                let nums: Vec<&'m str> = filter[1].split("|").collect();

                MoviesFilters::Nums(filter[0], nums)
            }
            _ => {
                panic!("Error invalid input");
            }
        }
    }
}

impl<'m> fmt::Display for MoviesFilters<'m> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoviesFilters::Alpha(k, v) => write!(f, "{} = '{}'", k, v),
            MoviesFilters::Alphas(k, v) => write!(f, "{} @> '{{{}}}'", k, v.join(", ")),
            MoviesFilters::Nums(k, v) => write!(f, "{} @> '{{{}}}'", k, v.join(", ")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moviesfilters_new_is_valid() {
        let test_filter = "release_country:uS";
        let test_moviesfilters = MoviesFilters::new(test_filter);

        match test_moviesfilters {
            Ok(test_moviesfilters) => match test_moviesfilters {
                MoviesFilters::Alpha(k, v) => {
                    let test_filter = test_filter.split(":").collect::<Vec<&str>>();
                    assert_eq!(k, test_filter[0]);
                    assert_eq!(v, test_filter[1].to_uppercase());
                }
                _ => panic!("Error, wrong variant"),
            },
            Err(error) => panic!("{:?}", error),
        }
    }
}
