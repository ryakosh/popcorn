#[derive(Serialize, Debug, PartialEq)]
pub enum Error {
  UserNFound,

  UnameTaken,
  UnameInvalid,
  UnameRequired,

  EmailTaken,
  EmailInvalid,
  EmailRequired,

  PwdInvalid,
  PwdRequired,

  LimitInvalid,
  PageInvalid,
}

pub type Errors = Vec<Error>;