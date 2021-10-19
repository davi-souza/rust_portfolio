use actix_web::{
    dev::Payload,
    error::{Error, ErrorUnauthorized},
    http::{header::AUTHORIZATION, HeaderValue},
    FromRequest, HttpRequest,
};
use futures::future::{err, ok, Ready};

#[derive(Debug)]
pub struct AuthUser {
    pub id: i32,
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let headers = req.headers();
        let maybe_auth_header = headers.get(AUTHORIZATION);
        match maybe_auth_header {
            Some(auth_header) => {
                if HeaderValue::from_str("Bearer 123").unwrap() != auth_header {
                    return err(ErrorUnauthorized("not authorized"));
                }
            }
            None => return err(ErrorUnauthorized("not authorized")),
        };
        ok(Self { id: 1 })
    }
}
