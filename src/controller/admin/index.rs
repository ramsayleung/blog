use dal::diesel_pool::DB;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::Template;

use rocket_contrib::Json;
#[get("/admin/index")]
pub fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("foo", "bar");
    Template::render("admin/index", &context)
}
