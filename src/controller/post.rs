use dal::models::post::*;
use dal::diesel_pool::DB;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::Template;

use rocket_contrib::Json;

#[get("/show_post")]
pub fn show_post(db: DB) -> Json<Vec<Post>> {
    let result = Post::query_latest_five(db.conn());
    Json(result)
}

#[get("/<id>")]
pub fn get_post_by_id(id: i32, db: DB) -> Template {
    let result = Post::query_by_id(db.conn(), id);
    let mut context = HashMap::new();
    context.insert("post", result.first());
    Template::render("post", &context)
    // Json(result)
}

#[get("/post")]
pub fn get_post() -> Template {
    let mut context = HashMap::new();
    context.insert("foo", "bar");
    Template::render("post", &context)
}
