use crate::regex::Regex;

lazy_static! {
  pub static ref RGX_EMAIL: Regex = Regex::new(r"^((([!#$%&'*+\-/=?^_`{|}~\w])|([!#$%&'*+\-/=?^_`{|}~\w][!#$%&'*+\-/=?^_`{|}~\.\w]{0,}[!#$%&'*+\-/=?^_`{|}~\w]))[@]\w+([-.]\w+)*\.\w+([-.]\w+)*)$").expect("Invalid email regex");
  pub static ref RGX_UNAME: Regex = Regex::new(r"^[a-z0-9_-]{1,30}$").expect("Invalid username regex");
  pub static ref RGX_PWD: Regex = Regex::new(r"^.{8,50}$").expect("Invalid password regex");
  pub static ref RGX_ALPHA: Regex = Regex::new(r"^[A-Za-z]+$").expect("Invalid alpha regex");
  pub static ref RGX_NUM: Regex = Regex::new(r"^[0-9]+$").expect("Invalid num regex");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgx_email_is_valid() {
        assert!(RGX_EMAIL.is_match("example@example.com"));
        assert!(RGX_EMAIL.is_match("g@google.com"));
        assert!(RGX_EMAIL.is_match("info@domain.org"));

        assert!(!RGX_EMAIL.is_match("example@.com"));
        assert!(!RGX_EMAIL.is_match("@example.com"));
        assert!(!RGX_EMAIL.is_match("example"));
        assert!(!RGX_EMAIL.is_match("تست"));
    }

    #[test]
    fn rgx_uname_is_valid() {
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
    fn rgx_pwd_is_valid() {
        assert!(RGX_PWD.is_match("example123"));
        assert!(RGX_PWD.is_match("@2password2@"));
        assert!(RGX_PWD.is_match("password"));

        assert!(!RGX_PWD.is_match(""));
        assert!(!RGX_PWD.is_match("pass"));
        assert!(!RGX_PWD.is_match("this is a very very very very very very long password"));
    }

    #[test]
    fn rgx_alpha_is_valid() {
        assert!(RGX_ALPHA.is_match("test"));
        
        assert!(!RGX_ALPHA.is_match("test2"));
        assert!(!RGX_ALPHA.is_match("tesت"));
        assert!(!RGX_ALPHA.is_match("تست"));
        assert!(!RGX_ALPHA.is_match("test!"));
    }

    #[test]
    fn rgx_num_is_valid() {
        assert!(RGX_NUM.is_match("123"));

        assert!(!RGX_NUM.is_match("۱۲۳"));
        assert!(!RGX_NUM.is_match("abc2"));
        assert!(!RGX_NUM.is_match("#3"));
    }
}
