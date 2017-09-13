use dal::models::post::*;
use dal::diesel_pool::DB;
use std::collections::HashMap;
use rocket_contrib::Template;

#[get("/about")]
pub fn get_about() -> Template {
    let mut map = HashMap::new();
    map.insert("foo", "bar");
    Template::render("about", &map)
}
