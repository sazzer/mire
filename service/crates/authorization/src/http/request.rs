use super::Authenticator;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};

impl FromRequest for Authenticator {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        ok(Self {})
    }
}
