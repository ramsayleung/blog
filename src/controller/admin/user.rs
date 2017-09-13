use diesel::result::QueryResult;
use diesel; //
use diesel::prelude::*;
use dal::diesel_pool::DB;
use dal::models::user::*;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies, Status};
use rocket_contrib::Template;
use chrono::prelude::*;
use rocket_contrib::Json;

use util::response::ResponseEnum;

#[post("/admin/user",data="<user_info>")]
pub fn add_user(db: DB, user_info: Json<UserInfo>) -> Json<ResponseEnum> {
    let new_user = UserInfo::convert_to_new_user(&user_info.0);
    if NewUser::insert(&new_user, db.conn()) {
        // "success"
        Json(ResponseEnum::SUCCESS)
    } else {
        Json(ResponseEnum::ERROR)
    }
}
#[post("/admin/query_user",data="<login>")]
pub fn query_user(db: DB, login: Json<Login>) -> Json<ResponseEnum> {
    let users = User::query_by_email(db.conn(), &login.0.email);
    if let Some(user) = users.first() {
        println!("{:?}", user);
    }

    Json(ResponseEnum::SUCCESS)
    // println!("{:?}", users.first());
    // "something"
}


#[get("/admin/login")]
pub fn login_page() -> Template {
    let mut context = HashMap::new();
    // context.insert("user_id", user.0);
    context.insert("foo", "bar");
    Template::render("admin/login", &context)
}

#[post("/admin/login", data = "<login>")]
pub fn login(db: DB, mut cookies: Cookies, login: Json<Login>) -> Json<ResponseEnum> {
    let users = User::query_by_email(db.conn(), &login.email);
    if let Some(user) = users.first() {
        let valid = match user.verify(&login.password) {
            Ok(valid) => {
                if valid {
                    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
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
