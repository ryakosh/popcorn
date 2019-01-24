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
}

pub type Errors = Vec<Error>;