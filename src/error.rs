#[derive(Serialize, Debug, PartialEq)]
pub enum Error {
    UserNFound,
    AuthRequired,
    AuthorizationInvalid,
    TokenInvalid,

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

    NotFound,

    FilterInvalid,
}
