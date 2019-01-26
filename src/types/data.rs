use crate::consts::{RGX_UNAME, RGX_EMAIL, RGX_PWD};
use crate::error::*;

pub struct SignupData<'s> {
  uname: &'s str,
  pwd: &'s str,
  email: &'s str,
}

impl<'s> SignupData<'s> {
  fn new(uname: &'s str,
         pwd: &'s str,
         email: &'s str) -> Result<SignupData<'s>, Errors> {
    
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