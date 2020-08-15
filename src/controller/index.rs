use rocket_contrib::templates::Template;

use dal::diesel_pool::DB;
use dal::models::post::*;
use util::log::log_to_db;
use util::log::Ip;
use util::response::footer_context;

const VISITOR: i32 = 0;

#[get("/index")]
pub fn get_index(db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);

    //get five latest posts
    let (result, more) = Post::query_latest_five_post(db.conn());
    let mut context = footer_context();
    context.insert("posts", &result);
    context.insert("more", &more);
    Template::render("index", &context.into_json())
}

#[get("/")]
pub fn index(db: DB, ip: Ip) -> Template {
    get_index(db, ip)
}
