// Used for template
use self::controller::{about, admin, error, friend, index, post};
use fern::colors::{Color, ColoredLevelConfig};
use rocket::{catchers, routes, Rocket};
use rocket::{launch, Build};
use rocket_dyn_templates::Template;
use std::env;

#[macro_use]
extern crate diesel;

mod static_file;
#[cfg(test)]
mod tests;

pub mod controller;
pub mod dal;
pub mod util;

// mount path
#[launch]
fn rocket() -> Rocket<Build> {
    setup_log();

    rocket::build()
        .mount(
            "/",
            routes![
                static_file::all,
                index::get_index,
                index::index,
                crate::util::log::get_ip,
                about::get_about,
                friend::get_friend,
                post::show_post,
                post::get_post,
                post::get_post_by_id,
                post::get_post_list,
                post::get_posts_pages,
                post::get_posts_by_tag,
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
        .register("/", catchers![error::not_found, error::unauthorised])
}
fn setup_log() {
    #[cfg(feature = "env-file")]
    {
        dotenv::dotenv().ok();
    }
    let error_log_path = env::var("ERROR_LOG_PATH").expect("ERROR_LOG_PATH must be set");
    let app_log_path = env::var("APP_LOG_PATH").expect("APP_LOG_PATH must be set");
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .debug(Color::Magenta)
        .info(Color::Green)
        .trace(Color::BrightBlue);

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .chain(
            fern::log_file(&app_log_path)
                .unwrap_or_else(|_| panic!("Cann't use this app_log_path: {}", &app_log_path)),
        )
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
            fern::Dispatch::new().level(log::LevelFilter::Error).chain(
                fern::log_file(&error_log_path).unwrap_or_else(|_| {
                    panic!("Cann't use this error_log_path: {}", &error_log_path)
                }),
            ),
        )
        .apply()
        .unwrap()
}
