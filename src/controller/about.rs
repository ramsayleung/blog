use rocket_contrib::Template;

use std::collections::HashMap;

use dal::diesel_pool::DB;
use dal::models::post::*;
use util::log::Ip;
use util::log::log_to_db;

const VISITOR: i32 = 0;

#[get("/about")]
pub fn get_about(db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);

    let result = Post::query_latest_about(db.conn());

    let mut map = HashMap::new();
    map.insert("post", result.first());
    Template::render("about", &map)
}
