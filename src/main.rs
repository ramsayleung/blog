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
// pub mod schema;
pub mod dal;
// pub mod models;

use dotenv::dotenv;
use std::env;
mod static_file;
// Server Imports
// Used to Setup DB Pool
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;

// Used for Routes
use rocket::Request;
use rocket::response::NamedFile;
// use rocket_contrib::Json;

use rocket_contrib::{Json, Value};
// Std Imports
use std::path::{Path, PathBuf};

// DB Imports
use diesel::prelude::*;
use diesel::update;
use diesel::pg::PgConnection;
use r2d2::{Pool, Config, PooledConnection, GetTimeout};
use r2d2_diesel::ConnectionManager;
use self::dal::models::post::Post;
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

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("admin/404", &map)
}

// DB Items
lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
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
// Routes

// #[get("/static/<file..>")]
// fn static_content(file: PathBuf) -> Result<NamedFile> {
//     NamedFile::open(Path::new("static/").join(file)).chain_err(|| "File not found!")
// }

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, get, static_file::all, admin, show_post])
        .attach(Template::fairing())
        .catch(errors![not_found])
}
pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(config, manager).expect("Failed to create pool.")
}
fn main() {
    rocket().launch();
}
