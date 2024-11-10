use ipnetwork::IpNetwork;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::Json;

use std::net::IpAddr;

use crate::dal::diesel_pool::DB;
use crate::dal::models::visitor_log::*;
use crate::util::response::ResponseEnum;
use crate::util::time::get_now;
use rocket::get;
#[derive(Debug)]
pub struct Ip(pub IpAddr);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Ip {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Ip, ()> {
        if let Some(address) = request.remote() {
            Outcome::Success(Ip(address.ip()))
        } else {
            Outcome::Forward(Status::MovedPermanently)
        }
    }
}

#[get("/ip")]
pub fn get_ip(ip: Ip, db: DB) -> Json<ResponseEnum> {
    let ip_address = IpNetwork::from(ip.0);
    let new_vistor_log = NewVisitorLog {
        ip: ip_address,
        access_time: get_now(),
        user_id: 0,
    };
    if NewVisitorLog::insert(&new_vistor_log, db.conn()) {
        Json(ResponseEnum::Success)
    } else {
        Json(ResponseEnum::Error)
    }
}

pub fn log_to_db(ip: Ip, db: &DB, user_id: i32) {
    let ip_address = IpNetwork::from(ip.0);
    let new_visitor_log = NewVisitorLog::new(&ip_address, user_id);
    NewVisitorLog::insert(&new_visitor_log, db.conn());
}
