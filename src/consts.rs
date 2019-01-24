use crate::regex::Regex;

lazy_static! {
  pub static ref RGX_EMAIL: Regex = Regex::new(r"^((([!#$%&'*+\-/=?^_`{|}~\w])|([!#$%&'*+\-/=?^_`{|}~\w][!#$%&'*+\-/=?^_`{|}~\.\w]{0,}[!#$%&'*+\-/=?^_`{|}~\w]))[@]\w+([-.]\w+)*\.\w+([-.]\w+)*)$").expect("Invalid email regex");
  pub static ref RGX_UNAME: Regex = Regex::new(r"^[a-z0-9_-]+$").expect("Invalid username regex");
  pub static ref RGX_PWD: Regex = Regex::new(r"^(?=.*[0-9]+.*)(?=.*[a-zA-Z]+.*)[0-9a-zA-Z]{6,}$").expect("Invalid password regex");
}