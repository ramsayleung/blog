use diesel; //
use diesel::prelude::*;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;
use chrono::prelude::*;
use rocket_contrib::Json;
use ipnetwork::IpNetwork;

use std::collections::HashMap;

use dal::diesel_pool::DB;
use dal::models::user::*;
use dal::models::visitor_log::*;
use util::response::ResponseEnum;
use util::auth;
use util::log::Ip;

#[post("/admin/signup",data="<user_info>")]
pub fn signup(db: DB, user_info: Json<UserInfo>) -> Json<ResponseEnum> {
    let new_user = UserInfo::convert_to_new_user(&user_info.0);
    if NewUser::insert(&new_user, db.conn()) {
        // "success"
        Json(ResponseEnum::SUCCESS)
    } else {
        Json(ResponseEnum::ERROR)
    }
}

#[get("/admin/user")]
pub fn get_user_list_page(_user: auth::User, db: DB) -> Template {
    let users = User::query_all(db.conn());
    let mut context = HashMap::new();
    // context.insert("user_id", user.0);
    context.insert("users", users);
    Template::render("admin/user_list", &context)
}

#[delete("/admin/user/<id>")]
pub fn delete_user(id: i32, db: DB) -> Json<ResponseEnum> {
    if User::delete_with_id(db.conn(), id) {
        Json(ResponseEnum::SUCCESS)
    } else {
        Json(ResponseEnum::ERROR)
    }
}
#[get("/admin/login")]
pub fn get_login_page() -> Template {
    let mut context = HashMap::new();
    // context.insert("user_id", user.0);
    context.insert("foo", "bar");
    Template::render("admin/login", &context)
}

#[post("/admin/login", data = "<login>")]
pub fn login(db: DB, mut cookies: Cookies, login: Json<Login>, ip: Ip) -> Json<ResponseEnum> {
    let users = User::query_by_email(db.conn(), &login.email);
    if let Some(user) = users.first() {
        let valid = match user.verify(&login.password) {
            Ok(valid) => {
                if valid {
                    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
                    cookies.add_private(Cookie::new("username", user.username.to_string()));

                    // record visitor
                    let ip_address = IpNetwork::from(ip.0);
                    let new_visitor_log = NewVisitorLog::new(&ip_address, user.id);
                    NewVisitorLog::insert(&new_visitor_log, db.conn());

                    Json(ResponseEnum::SUCCESS)
                } else {
                    Json(ResponseEnum::FAILURE)
                }
            }
            Err(_) => Json(ResponseEnum::ERROR),
        };
        valid
    } else {
        Json(ResponseEnum::FAILURE)
    }
}
#[get("/admin/logout")]
pub fn logout(mut cookies: Cookies) -> Json<ResponseEnum> {
    cookies.remove_private(Cookie::named("user_id"));
    cookies.remove_private(Cookie::named("username"));
    Json(ResponseEnum::SUCCESS)
}
