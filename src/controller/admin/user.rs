use log::info;
use rocket::data::{Data, ToByteUnit};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;

use std::collections::HashMap;
use std::env;
use std::io;

use crate::dal::diesel_pool::DB;
use crate::dal::models::user::*;
use crate::util::auth;
use crate::util::log::log_to_db;
use crate::util::log::Ip;
use crate::util::response::template_context;
use crate::util::response::ResponseEnum;
use crate::util::time::get_now;

use rocket::{delete, get, post, put};
#[post("/admin/signup", data = "<user_info>")]
pub fn signup(db: DB, user_info: Json<UserInfo>) -> Json<ResponseEnum> {
    let new_user = UserInfo::convert_to_new_user(&user_info.0);
    if NewUser::insert(&new_user, db.conn()) {
        // "success"
        Json(ResponseEnum::Success)
    } else {
        Json(ResponseEnum::Error)
    }
}

#[get("/admin/profile")]
pub fn get_profile_page(user: auth::User, db: DB) -> Template {
    let users = User::query_by_id(db.conn(), user.0);
    let mut context = template_context(&db, user);
    if let Some(user) = users.first() {
        context.insert("user", user);
    }
    Template::render("admin/profile", &context.into_json())
}

#[get("/admin/user")]
pub fn get_user_list_page(user: auth::User, db: DB) -> Template {
    let users = User::query_all(db.conn());
    // context.insert("user_id", user.0);
    let mut context = template_context(&db, user);
    context.insert("users", &users);
    Template::render("admin/user_list", &context.into_json())
}

#[put("/admin/user", data = "<update_user>")]
pub fn update_user(update_user: Json<User>, db: DB) -> Json<ResponseEnum> {
    info!("Call update");
    if User::update(db.conn(), &update_user.0) {
        Json(ResponseEnum::Success)
    } else {
        Json(ResponseEnum::Error)
    }
}
#[delete("/admin/user/<id>")]
pub fn delete_user(id: i32, db: DB) -> Json<ResponseEnum> {
    if User::delete_with_id(db.conn(), id) {
        Json(ResponseEnum::Success)
    } else {
        Json(ResponseEnum::Error)
    }
}
#[get("/admin/login")]
pub fn get_login_page() -> Template {
    let mut context = HashMap::new();
    // context.insert("user_id", user.0);
    context.insert("foo", "bar");
    Template::render("admin/login", &context)
}

#[post("/admin/login", data = "<login>")]
pub fn login(db: DB, jar: &CookieJar<'_>, login: Json<Login>, ip: Ip) -> Json<ResponseEnum> {
    let users = User::query_by_email(db.conn(), &login.email);
    if let Some(user) = users.first() {
        match user.verify(&login.password) {
            Ok(valid) => {
                if valid {
                    jar.add_private(Cookie::new("user_id", user.id.to_string()));
                    jar.add_private(Cookie::new("username", user.username.to_string()));

                    // record visitor
                    log_to_db(ip, &db, user.id);

                    Json(ResponseEnum::Success)
                } else {
                    Json(ResponseEnum::Failure)
                }
            }
            Err(_) => Json(ResponseEnum::Error),
        }
    } else {
        Json(ResponseEnum::Failure)
    }
}
#[get("/admin/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Json<ResponseEnum> {
    jar.remove_private(Cookie::named("user_id"));
    jar.remove_private(Cookie::named("username"));
    Json(ResponseEnum::Success)
}

#[post("/admin/user/change_password", data = "<change_password>")]
pub fn change_password(db: DB, change_password: Json<ChangePassword>) -> Json<ResponseEnum> {
    let users = User::query_by_id(db.conn(), change_password.user_id);
    if let Some(user) = users.first() {
        match user.verify(&change_password.old_password) {
            Ok(valid) => {
                if valid {
                    if User::change_password(
                        db.conn(),
                        change_password.user_id,
                        &change_password.new_password,
                        &get_now(),
                    ) {
                        Json(ResponseEnum::Success)
                    } else {
                        Json(ResponseEnum::Error)
                    }
                // password verify failed
                } else {
                    Json(ResponseEnum::Failure)
                }
            }
            Err(_) => Json(ResponseEnum::Error),
        }
    } else {
        Json(ResponseEnum::Failure)
    }
}
#[post("/admin/image/upload", format = "image/*", data = "<data>")]
pub async fn upload_image(data: Data<'_>) -> io::Result<String> {
    // We assume that we are in a valid directory.
    let path = env::current_dir().unwrap();
    info!("The current directory is {}", path.display());
    data.open(10_i32.megabytes())
        .into_file("/tmp/file.png")
        .await
        .map(|file| format!("Wrote {} bytes to /static/file", file.n))
    // Ok(Redirect::to("/admin/images"))
}
