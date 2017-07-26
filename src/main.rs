#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket_contrib;
extern crate chrono;
extern crate rocket;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
use rocket::response::Redirect;
use rocket_contrib::Template;

#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[cfg(test)]
mod tests;
pub mod dal;
pub mod controller;

mod static_file;

// Used for Routes
use rocket::Request;

use rocket_contrib::{Json, Value};

// DB Imports
use diesel::prelude::*;
use diesel::update;
use self::dal::models::post::Post;
use self::controller::{index,error};
use self::dal::diesel_pool::*;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<String>,
}
#[derive(Serialize)]
struct Context {}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/index")
}

#[get("/admin/index")]
fn admin() -> Template {
    let name = String::from("Helloworld");
    let context = TemplateContext {
        name: name,
        items: vec!["One", "Two", "Three"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };

    Template::render("admin/index", &context)
}
#[get("/index")]
fn get() -> Template {
    let context = Context {};
    Template::render("index", &context)
}

#[get("/show_post")]
fn show_post(db: DB) -> Json<Post> {
    use self::dal::schema::post::dsl::post as all_posts;
    let result = Post::query_all(db.conn());
    for post in result {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.subtitle);
    }
    let result = all_posts
        .first::<Post>(db.conn())
        .expect("could not load post");
    Json(Post {
             id: result.id,
             title: result.title,
             subtitle: result.subtitle,
             published: result.published,
             user_id: result.user_id,
             create_time: result.create_time,
             modify_time: result.modify_time,
             publish_time: result.publish_time,
         })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, get, static_file::all, admin, show_post])
        .attach(Template::fairing())
        .catch(errors![error::not_found])
}
fn main() {
    rocket().launch();
}
