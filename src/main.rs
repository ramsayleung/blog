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

#[cfg(test)]
mod tests;
pub mod dal;
pub mod controller;

mod static_file;

// Used for template
use rocket_contrib::Template;

use self::controller::{index, error, post, admin};

// mount path
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/",
               routes![index::get_index,
                       index::index,
                       static_file::all,
                       admin::index::index,
                       admin::index::form,
                       post::show_post,
                       post::get_post,
                       post::get_post_by_id,
                       index::get_about])
        .attach(Template::fairing())
        .catch(errors![error::not_found])
}
fn main() {
    rocket().launch();
}
