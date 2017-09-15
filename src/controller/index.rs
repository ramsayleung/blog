use rocket::response::Redirect;
use rocket_contrib::Template;
use rocket_contrib::Json;
use ipnetwork::IpNetwork;

use std::collections::HashMap;

use dal::models::post::*;
use dal::diesel_pool::DB;
use dal::models::visitor_log::*;
use util::log::Ip;

#[get("/index")]
pub fn get_index(db: DB, ip: Ip) -> Template {
    // record visitor
    let ip_address = IpNetwork::from(ip.0);
    let new_visitor_log = NewVisitorLog::new(&ip_address, 0);
    NewVisitorLog::insert(&new_visitor_log, db.conn());

    //get five latest posts
    let result = Post::query_latest_five(db.conn());
    let mut map = HashMap::new();
    map.insert("posts", result);
    Template::render("index", &map)
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/index")
}
