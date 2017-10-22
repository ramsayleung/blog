use rocket::response::Redirect;
use rocket_contrib::Template;
use chrono::NaiveDateTime;

use util::auth::User;
use util::response::template_context;
use dal::models::user;
use dal::diesel_pool::DB;

#[derive(Deserialize, Serialize)]
enum UserInfo {
    Username(String),
    AccessTime(NaiveDateTime),
}

#[get("/admin/index")]
pub fn index(user: User, db: DB) -> Template {
    let context = template_context(db, user);
    Template::render("admin/index", &context)
}

#[get("/admin")]
pub fn index_redirect(_user: User) -> Redirect {
    Redirect::to("/admin/index")
}
