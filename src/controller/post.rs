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

#[get("/post/<id>")]
pub fn get_post_by_id(id: String, db: DB) -> Json<Vec<Post>> {
    let result = Post::query_by_id(db.conn(), id);
    Json(result)

}

#[get("/post")]
pub fn get_post() -> Template {
    let mut context = HashMap::new();
    context.insert("foo", "bar");
    Template::render("post", &context)
}
