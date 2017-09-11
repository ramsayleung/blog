use std::collections::HashMap;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, Cookies, Status};
use rocket_contrib::Template;

#[derive( Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct User(usize);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        // let user_id = request.cookies().get_private("user_id");
        // match user_id {
        //     Some(cookie) => {
        //         let id = cookie.value().parse().ok();
        //         match id {
        //             Some(value) => Outcome::Success(User(id.map())),
        //             None => Outcome::Failure((Status::Unauthorized, ())),
        //         }
        //     }
        //     None => Outcome::Failure((Status::Unauthorized, ())),
        // }
        // let uri = request.uri().as_str();
        // println!("uri: {:?}", uri);
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User(id))
            .or_forward(())
    }
}
