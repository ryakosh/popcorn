use std::fmt;

#[derive(Debug)]
pub enum MoviesFilters<'m> {
    Alpha(&'m str, String),
    Alphas(&'m str, Vec<&'m str>),
    Nums(&'m str, Vec<&'m str>),
}

impl<'m> MoviesFilters<'m> {
    pub fn new(filter: &'m str) -> Self {
        let filter = filter.split(':').collect::<Vec<&'m str>>();
        match filter[0] {
            "release_country" => MoviesFilters::Alpha(filter[0], filter[1].to_uppercase()),
            "genres" | "languages" => {
                let alphas: Vec<&'m str> = filter[1].split('|').collect();

                MoviesFilters::Alphas(filter[0], alphas)
            }
            "directors" | "writers" | "stars" => {
                let nums: Vec<&'m str> = filter[1].split('|').collect();

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
    fn moviesfilters_new_works_correctly() {
        let mf = MoviesFilters::new("release_country:Us");
        if let MoviesFilters::Alpha(k, v) = mf {
            assert_eq!(k, "release_country");
            assert_eq!(v, "US");
        } else {
            panic!("Err: {:?}", mf);
        }

        let mf = MoviesFilters::new("genres:Action|Advanture");
        if let MoviesFilters::Alphas(k, v) = mf {
            assert_eq!(k, "genres");
            assert_eq!(v, vec!["Action", "Advanture"]);
        } else {
            panic!("Err: {:?}", mf);
        }

        let mf = MoviesFilters::new("writers:1|2|3");
        if let MoviesFilters::Nums(k, v) = mf {
            assert_eq!(k, "writers");
            assert_eq!(v, vec!["1", "2", "3"]);
        } else {
            panic!("Err: {:?}", mf);
        }
    }

    #[test]
    fn moviesfilters_display_formatter() {
        let mf = MoviesFilters::new("release_country:Us");
        assert_eq!(mf.to_string(), "release_country = 'US'");

        let mf = MoviesFilters::new("genres:Action|Advanture");
        assert_eq!(mf.to_string(), "genres @> '{Action, Advanture}'");

        let mf = MoviesFilters::new("writers:1|2|3");
        assert_eq!(mf.to_string(), "writers @> '{1, 2, 3}'");
    }
}
