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

#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate bcrypt;

#[cfg(test)]
mod tests;
pub mod dal;
pub mod util;
pub mod controller;

mod static_file;

// Used for template
use rocket_contrib::Template;

use self::controller::{index, error, post, admin, about};

// mount path
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/",
               routes![static_file::all,
                       index::get_index,
                       index::index,
                       about::get_about,
                       post::show_post,
                       post::get_post,
                       post::get_post_by_id,
                       post::get_post_list,
                       admin::index::index,
                       admin::index::index_redirect,
                       admin::post::add_post,
                       admin::post::get_posts,
                       admin::post::edit_post,
                       admin::post::add_post_page,
                       admin::post::update_post,
                       admin::post::delete_post,
                       admin::user::login_page,
                       admin::user::login,
                       admin::user::logout,
                       admin::user::add_user,
                       admin::user::query_user])
        .attach(Template::fairing())
        .catch(errors![error::not_found])
}
fn main() {
    rocket().launch();
}
