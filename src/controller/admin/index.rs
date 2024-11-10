use chrono::NaiveDateTime;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

use crate::dal::diesel_pool::DB;
use crate::util::auth::User;
use crate::util::response::template_context;
use rocket::get;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
enum UserInfo {
    Username(String),
    AccessTime(NaiveDateTime),
}

#[get("/admin/index")]
pub fn index(user: User, db: DB) -> Template {
    let context = template_context(&db, user);
    Template::render("admin/index", context.into_json())
}

#[get("/admin")]
pub fn index_redirect(_user: User) -> Redirect {
    Redirect::to("/admin/index")
}
