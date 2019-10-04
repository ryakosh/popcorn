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
            panic!(format!("Err: {:?}", err))
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
}
