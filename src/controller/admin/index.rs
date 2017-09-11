
use std::collections::HashMap;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, Cookies, Status};
use rocket_contrib::Template;

use dal::diesel_pool::DB;
use util::auth::User;
// use std::collections::HashMap;
// use rocket::response::Redirect;
// use rocket_contrib::Template;

use rocket_contrib::Json;
#[derive( Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}


#[get("/admin/login")]
pub fn login_page() -> Template {
    let mut context = HashMap::new();
    // context.insert("user_id", user.0);
    context.insert("foo", "bar");
    Template::render("admin/login", &context)
}

#[post("/admin/login", data = "<login>")]
pub fn login(mut cookies: Cookies, login: Json<Login>) -> &'static str {
    if login.username == "Sergio" && login.password == "password" {
        cookies.add_private(Cookie::new("user_id", 1.to_string()));
        "success"
    } else {
        "error"
    }
}

#[get("/admin/index")]
pub fn index(user: User) -> Template {
    let mut context = HashMap::new();
    // context.insert("user_id", user.0);
    context.insert("foo", "bar");
    Template::render("admin/index", &context)
}

#[get("/admin")]
pub fn index_redirect(user: User) -> Redirect {
    Redirect::to("/admin/index")
}
