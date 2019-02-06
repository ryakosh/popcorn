use crate::consts::{RGX_UNAME, RGX_EMAIL, RGX_PWD};
use crate::error::*;

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
  fn new(uname: &'s str,
         pwd: &'s str,
         email: &'s str) -> Result<Self, Errors> {
    
    let is_uname = RGX_UNAME.is_match(uname);
    let is_pwd   = RGX_PWD.is_match(pwd);
    let is_email = RGX_EMAIL.is_match(email);

    if is_uname && is_pwd && is_email {
      Ok(SignupData { uname, pwd, email })
    } else {
      let mut errors = Vec::new();

      if let false = is_uname { errors.push(Error::UnameInvalid) }
      if let false = is_pwd { errors.push(Error::PwdInvalid) }
      if let false = is_email { errors.push(Error::EmailInvalid) }

      Err(errors)
    }
  }

  pub fn uname(&self) -> &'s str { self.uname }
  pub fn email(&self) -> &'s str { self.email }
  pub fn pwd(&self) -> &'s str { self.pwd }
}

impl<'s> SigninData<'s> {
  fn new(uname: &'s str, pwd: &'s str) -> Result<Self, Errors> {
    let is_uname = RGX_UNAME.is_match(uname);
    let is_pwd   = RGX_PWD.is_match(pwd);

    if is_uname && is_pwd {
      Ok(SigninData { uname, pwd })
    } else {
      let mut errors = Vec::new();

      if let false = is_uname { errors.push(Error::UnameInvalid) }
      if let false = is_pwd { errors.push(Error::PwdInvalid) }
      
      Err(errors)
    }
  }

  pub fn uname(&self) -> &'s str { self.uname }
  pub fn pwd(&self) -> &'s str { self.pwd }
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
    let data = SigninData::new("uname", "password")
      .expect("Error instantiating SigninData");
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