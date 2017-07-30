use dal::diesel_pool::DB;
use dal::models::post::*;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::Template;

use rocket_contrib::Json;
#[get("/admin/table")]
pub fn table() -> Template {
    let mut context = HashMap::new();
    context.insert("foo", "bar");
    Template::render("admin/table", &context)
}
#[get("/admin/post_list")]
pub fn post_list(db: DB) -> Template {
    let result = Post::query_all(db.conn());
    let mut context = HashMap::new();
    context.insert("posts", result);
    Template::render("admin/post_list", &context)
}
#[post("/admin/post",data="<post>")]
pub fn add_post(db: DB, post: Json<PostRequest>) {
    println!("title: {}", post.title);
    println!("subtitle: {}", post.subtitle);
    println!("raw_content: {}", post.raw_content);
    println!("rendered_content: {}", post.rendered_content);
    println!("published: {}", post.published);
}
