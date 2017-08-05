use diesel::result::QueryResult;
use diesel; //
use diesel::prelude::*;
use dal::diesel_pool::DB;
use dal::models::post::*;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::Template;
use chrono::prelude::*;
use rocket_contrib::Json;
#[get("/admin/post")]
pub fn get_posts(db: DB) -> Template {
    let result = Post::query_all(db.conn());
    let mut context = HashMap::new();
    context.insert("posts", result);
    Template::render("admin/post", &context)
}

#[post("/admin/post",data="<new_post>")]
pub fn add_post(db: DB, new_post: Json<NewPost>) -> &'static str {
    if NewPost::insert(&new_post.0, db.conn()) {
        "success"
    } else {
        "error"
    }
}

#[get("/admin/new_post")]
pub fn new_post() -> Template {
    let mut context = HashMap::new();
    context.insert("foo", "bar");
    Template::render("admin/form-general", &context)
}

#[delete("/admin/post/<id>")]
pub fn delete_post(id: i32, db: DB) -> &'static str {
    if Post::delete_with_id(db.conn(), id) {
        "success"
    } else {
        "error"
    }
}

// #[get("/admin/post/<id>")]
// pub fn get_post(id: i32, db: DB) -> Template {}
