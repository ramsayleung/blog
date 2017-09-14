use rocket::Outcome;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use std::net::IpAddr;

#[derive(Debug)]
pub struct Ip(IpAddr);

impl<'a, 'r> FromRequest<'a, 'r> for Ip {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Ip, ()> {
        if let Some(address) = request.remote() {
            Outcome::Success(Ip(address.ip()))
        } else {
            Outcome::Forward(())
        }
    }
}
