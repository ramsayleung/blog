use rocket::response::Redirect;
use rocket_contrib::Template;

use std::collections::HashMap;

use dal::diesel_pool::DB;
use dal::models::user::{self, Login};
use util::auth::User;


#[get("/admin/index")]
pub fn index(_user: User) -> Template {
    let mut context = HashMap::new();
    // context.insert("user_id", user.0);
    context.insert("foo", "bar");
    Template::render("admin/index", &context)
}

#[get("/admin")]
pub fn index_redirect(_user: User) -> Redirect {
    Redirect::to("/admin/index")
}
