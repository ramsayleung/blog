#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
#![plugin(diesel_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate tera;

extern crate rocket_contrib;
extern crate chrono;
extern crate ipnetwork;
extern crate rocket;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate bcrypt;

// Used for template
use rocket_contrib::Template;
use self::controller::{index, error, post, admin, about};

#[cfg(test)]
mod tests;

pub mod dal;
pub mod util;
pub mod controller;

mod static_file;


// mount path
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/",
               routes![static_file::all,
                       index::get_index,
                       index::index,
                       util::log::get_ip,
                       about::get_about,
                       post::show_post,
                       post::get_post,
                       post::get_post_by_id,
                       post::get_post_list,
                       admin::index::index,
                       admin::index::index_redirect,
                       admin::post::add_post,
                       admin::post::get_post,
                       admin::post::get_posts,
                       admin::post::edit_post,
                       admin::post::add_post_page,
                       admin::post::update_post,
                       admin::post::delete_post,
                       admin::post::get_ten_hottest_posts,
                       admin::user::login,
                       admin::user::logout,
                       admin::user::signup,
                       admin::user::delete_user,
                       admin::user::update_user,
                       admin::user::change_password,
                       admin::user::upload_image,
                       admin::user::get_profile_page,
                       admin::user::get_login_page,
                       admin::user::get_user_list_page,
                       admin::visitor_log::count_daily_page_view,
                       admin::visitor_log::count_monthly_page_view,
                       ])
        .attach(Template::fairing())
        .catch(errors![error::not_found, error::unauthorised])
}
fn main() {
    rocket().launch();
}
