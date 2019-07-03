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
pub struct RateData {
    rating: i32,
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

impl RateData {
    pub fn rating(&self) -> i32 {
        self.rating
    }

    pub fn validate(&self) -> Result<&Self, Error> {
        if self.rating >= 0 && self.rating <= 5 {
            Ok(self)
        } else {
           Err(Error::InputInvalid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signupdata_new_is_valid() {
        let data = SignupData::new("uname", "password", "example@example.com")
            .expect("Error instantiating SignupData");
        assert_eq!(data.uname, "uname");
        assert_eq!(data.pwd, "password");
        assert_eq!(data.email, "example@example.com");

        let data = SignupData::new("uname", "pass", "example@example");
        if let Err(errors) = data {
            assert_eq!(errors, &[Error::PwdInvalid, Error::EmailInvalid]);
        } else {
            panic!("SignupData should return Errors");
        }
    }

    #[test]
    fn signindata_new_is_valid() {
        let data = SigninData::new("uname", "password").expect("Error instantiating SigninData");
        assert_eq!(data.uname, "uname");
        assert_eq!(data.pwd, "password");

        let data = SigninData::new("uname", "pass");
        if let Err(errors) = data {
            assert_eq!(errors, &[Error::PwdInvalid]);
        } else {
            panic!("SigninData should return Errors");
        }
    }
}
