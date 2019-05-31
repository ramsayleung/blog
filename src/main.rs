#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate lazy_static;
extern crate tera;

extern crate chrono;
extern crate ipnetwork;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

// #[macro_use]
// extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;
extern crate fern;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;

// Used for template
use self::controller::{about, admin, error, friend, index, post};
use dotenv::dotenv;
use fern::colors::{Color, ColoredLevelConfig};
use rocket_contrib::templates::Template;
use std::env;

#[cfg(test)]
mod tests;

pub mod controller;
pub mod dal;
pub mod util;

mod static_file;

// mount path
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                static_file::all,
                index::get_index,
                index::index,
                util::log::get_ip,
                about::get_about,
                friend::get_friend,
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
                admin::visitor_log::count_daily_user_view,
                admin::visitor_log::count_monthly_page_view,
                admin::visitor_log::count_monthly_user_view,
            ],
        )
        .attach(Template::fairing())
        .register(catchers![error::not_found, error::unauthorised])
}
fn setup_log() {
    dotenv().ok();
    let error_log_path = env::var("ERROR_LOG_PATH").expect("ERROR_LOG_PATH must be set");
    let app_log_path = env::var("APP_LOG_PATH").expect("APP_LOG_PATH must be set");
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .debug(Color::Magenta)
        .info(Color::Green)
        .trace(Color::BrightBlue);

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .chain(fern::log_file(&app_log_path).unwrap())
        .level(log::LevelFilter::Debug)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}] [{level}][{target}] [{message}]",
                date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                level = colors.color(record.level()),
                target = record.target(),
                message = message
            ))
        })
        .chain(
            fern::Dispatch::new()
                .level(log::LevelFilter::Error)
                .chain(fern::log_file(&error_log_path).unwrap()),
        )
        .apply()
        .unwrap()
}

fn main() {
    setup_log();
    info!("Starting up ");

    // Startup the controller
    info!("Starting web service controller");
    rocket().launch();
}
