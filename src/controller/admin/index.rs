use rocket::response::Redirect;
use rocket_contrib::Template;
use chrono::NaiveDateTime;

use std::collections::HashMap;

use util::auth::User;
use dal::models::visitor_log::*;
use dal::models::user;
use dal::diesel_pool::DB;

#[derive(Deserialize, Serialize)]
enum UserInfo {
    Username(String),
    AccessTime(NaiveDateTime),
}

#[get("/admin/index")]
pub fn index(user: User, db: DB) -> Template {
    let visitor_logs = VisitorLog::query_login_user(db.conn(), user.0);
    let users = user::User::query_by_id(db.conn(), user.0);
    let mut context = HashMap::new();
    if let Some(user) = users.first() {
        // context.insert("username", UserInfo::Username(user.username.to_string()));
        context.insert("username", user.username.to_string());
    }
    if let Some(log) = visitor_logs.first() {
        // context.insert("access_time", UserInfo::AccessTime(log.access_time));
        context.insert("access_time", log.access_time.to_string());
    }
    Template::render("admin/index", &context)
}

#[get("/admin")]
pub fn index_redirect(_user: User) -> Redirect {
    Redirect::to("/admin/index")
}
