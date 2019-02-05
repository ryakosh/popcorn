use crate::regex::Regex;

lazy_static! {
  pub static ref RGX_EMAIL: Regex = Regex::new(r"^((([!#$%&'*+\-/=?^_`{|}~\w])|([!#$%&'*+\-/=?^_`{|}~\w][!#$%&'*+\-/=?^_`{|}~\.\w]{0,}[!#$%&'*+\-/=?^_`{|}~\w]))[@]\w+([-.]\w+)*\.\w+([-.]\w+)*)$").expect("Invalid email regex");
  pub static ref RGX_UNAME: Regex = Regex::new(r"^[a-z0-9_-]{1,30}$").expect("Invalid username regex");
  pub static ref RGX_PWD: Regex = Regex::new(r"^.{8,50}$").expect("Invalid password regex");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn email_rgx_is_valid() {
    assert!(RGX_EMAIL.is_match("example@example.com"));
    assert!(RGX_EMAIL.is_match("g@google.com"));
    assert!(RGX_EMAIL.is_match("info@domain.org"));

    assert!(!RGX_EMAIL.is_match("example@.com"));
    assert!(!RGX_EMAIL.is_match("@example.com"));
    assert!(!RGX_EMAIL.is_match("example"));
    assert!(!RGX_EMAIL.is_match("تست"));
  }

  #[test]
  fn uname_rgx_is_valid() {
    assert!(RGX_UNAME.is_match("example"));
    assert!(RGX_UNAME.is_match("e"));
    assert!(RGX_UNAME.is_match("ex-_"));
    assert!(RGX_UNAME.is_match("12345"));

    assert!(!RGX_UNAME.is_match("example@"));
    assert!(!RGX_UNAME.is_match("exampleتست"));
    assert!(!RGX_UNAME.is_match("s p a c e s"));
    assert!(!RGX_UNAME.is_match("thisisaveryveryveryverylongusername"));
  }

  #[test]
  fn pwd_rgx_is_valid() {
    assert!(RGX_PWD.is_match("example123"));
    assert!(RGX_PWD.is_match("@2password2@"));
    assert!(RGX_PWD.is_match("password"));

    assert!(!RGX_PWD.is_match(""));
    assert!(!RGX_PWD.is_match("pass"));
    assert!(!RGX_PWD.is_match("this is a very very very very very very long password"));
  }
}