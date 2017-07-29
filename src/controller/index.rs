use dal::models::post::*;
use dal::diesel_pool::DB;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::Template;

use rocket_contrib::Json;
use dotenv::dotenv;
// Std Imports
use std::env;
#[get("/index")]
pub fn get_index(db: DB) -> Template {
    let result = Post::query_latest_five(db.conn());
    let mut map = HashMap::new();
    map.insert("posts", result);
    Template::render("index", &map)
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/index")
}


#[get("/about")]
fn get_about() -> Template {
    let mut map = HashMap::new();
    map.insert("foo", "bar");
    Template::render("about", &map)
}

