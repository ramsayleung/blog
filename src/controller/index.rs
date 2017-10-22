use rocket::response::Redirect;
use rocket_contrib::Template;

use dal::models::post::*;
use dal::diesel_pool::DB;
use util::log::Ip;
use util::log::log_to_db;
use util::response::footer_context;

const VISITOR: i32 = 0;

#[get("/index")]
pub fn get_index(db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);

    //get five latest posts
    let result = Post::query_latest_five_post(db.conn());
    let mut context = footer_context();
    context.add("posts", &result);
    Template::render("index", &context)
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/index")
}
