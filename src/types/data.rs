use crate::consts::{RGX_EMAIL, RGX_PWD, RGX_UNAME};
use crate::error::Error;

#[derive(Deserialize)]
pub struct SignupData<'s> {
    uname: &'s str,
    pwd: &'s str,
    email: &'s str,
}

#[derive(Deserialize)]
pub struct SigninData<'s> {
    uname: &'s str,
    pwd: &'s str,
}

#[derive(Deserialize)]
pub struct RateData<'r> {
    token: &'r str,
    movie_id: i32,
    user_rating: i16,
}

impl<'s> SignupData<'s> {
    pub fn uname(&self) -> &'s str {
        self.uname
    }
    pub fn email(&self) -> &'s str {
        self.email
    }
    pub fn pwd(&self) -> &'s str {
        self.pwd
    }

    pub fn validate(&self) -> Result<&Self, Error> {
        self.validate_uname()?;
        self.validate_pwd()?;
        self.validate_email()?;

        Ok(self)
    }

    fn validate_uname(&self) -> Result<(), Error> {
        if RGX_UNAME.is_match(self.uname()) {
            Ok(())
        } else {
            Err(Error::UnameInvalid)
        }
    }
    fn validate_pwd(&self) -> Result<(), Error> {
        if RGX_PWD.is_match(self.pwd()) {
            Ok(())
        } else {
            Err(Error::PwdInvalid)
        }
    }
    fn validate_email(&self) -> Result<(), Error> {
        if RGX_EMAIL.is_match(self.email()) {
            Ok(())
        } else {
            Err(Error::EmailInvalid)
        }
    }
}

impl<'s> SigninData<'s> {
    pub fn uname(&self) -> &'s str {
        self.uname
    }
    pub fn pwd(&self) -> &'s str {
        self.pwd
    }

    pub fn validate(&self) -> Result<&Self, Error> {
        self.validate_uname()?;
        self.validate_pwd()?;

        Ok(self)
    }

    fn validate_uname(&self) -> Result<(), Error> {
        if RGX_UNAME.is_match(self.uname()) {
            Ok(())
        } else {
            Err(Error::UnameInvalid)
        }
    }
    fn validate_pwd(&self) -> Result<(), Error> {
        if RGX_PWD.is_match(self.pwd()) {
            Ok(())
        } else {
            Err(Error::PwdInvalid)
        }
    }
}

impl<'r> RateData<'r> {
    pub fn token(&self) -> &'r str {
        self.token
    }
    pub fn movie_id(&self) -> i32 {
        self.movie_id
    }
    pub fn user_rating(&self) -> i16 {
        self.user_rating
    }

    pub fn validate(&self) -> Result<&Self, Error> {
        self.validate_movie_id()?;
        self.validate_user_rating()?;

        Ok(self)
    }

    fn validate_movie_id(&self) -> Result<(), Error> {
        if self.movie_id() >= 1 {
            Ok(())
        } else {
            Err(Error::DataInvalid)
        }
    }
    fn validate_user_rating(&self) -> Result<(), Error> {
        if self.user_rating() >= 1 && self.user_rating() <= 5 {
            Ok(())
        } else {
            Err(Error::DataInvalid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signupdata_getters_work_correctly() {
        let test_uname = "test";
        let test_pwd = "testpwd1";
        let test_email = "test@test.test";

        let data = SignupData {
            uname: test_uname,
            pwd: test_pwd,
            email: test_email,
        };

        assert_eq!(data.uname(), test_uname);
        assert_eq!(data.pwd(), test_pwd);
        assert_eq!(data.email(), test_email);
    }

    #[test]
    fn signupdata_validate_works_correctly() {
        let test_uname = "test";
        let test_pwd = "testpwd1";
        let test_email = "test@test.test";
        let data = SignupData {
            uname: test_uname,
            pwd: test_pwd,
            email: test_email,
        };

        if let Err(err) = data.validate() {
            panic!(format!("Err: {:?}", err));
        }

        let test_email = "test@";
        let data = SignupData {
            uname: test_uname,
            pwd: test_pwd,
            email: test_email,
        };

        match data.validate() {
            Ok(_) => panic!("Email is invalid"),
            Err(err) => assert_eq!(err, Error::EmailInvalid),
        }
    }

    #[test]
    fn signindata_getters_work_correctly() {
        let test_uname = "test";
        let test_pwd = "testpwd1";

        let data = SigninData {
            uname: test_uname,
            pwd: test_pwd,
        };

        assert_eq!(data.uname(), test_uname);
        assert_eq!(data.pwd(), test_pwd);
    }

    #[test]
    fn signindata_validate_works_correctly() {
        let test_uname = "test";
        let test_pwd = "testpwd1";
        let data = SigninData {
            uname: test_uname,
            pwd: test_pwd,
        };

        if let Err(err) = data.validate() {
            panic!(format!("Err: {:?}", err))
        }

        let test_pwd = "t";
        let data = SigninData {
            uname: test_uname,
            pwd: test_pwd,
        };

        match data.validate() {
            Ok(_) => panic!("Password is invalid"),
            Err(err) => assert_eq!(err, Error::PwdInvalid),
        }
    }

    #[test]
    fn ratedata_getters_work_correctly() {
        let test_token = "token";
        let test_movie_id = 1;
        let test_user_rating = 4;

        let data = RateData {
            token: test_token,
            movie_id: test_movie_id,
            user_rating: test_user_rating,
        };

        assert_eq!(data.token(), test_token);
        assert_eq!(data.movie_id(), test_movie_id);
        assert_eq!(data.user_rating(), test_user_rating);
    }

    #[test]
    fn ratedata_validate_works_correctly() {
        let test_token = "token";
        let test_movie_id = 1;
        let test_user_rating = 4;
        let data = RateData {
            token: test_token,
            movie_id: test_movie_id,
            user_rating: test_user_rating,
        };

        if let Err(err) = data.validate() {
            panic!(format!("Err: {:?}", err))
        }

        let test_user_rating = 6;
        let data = RateData {
            token: test_token,
            movie_id: test_movie_id,
            user_rating: test_user_rating,
        };

        match data.validate() {
            Ok(_) => panic!("User-rating is invalid"),
            Err(err) => assert_eq!(err, Error::DataInvalid),
        }
    }
}
