use rocket_contrib::Template;
use ipnetwork::IpNetwork;

use std::collections::HashMap;

use dal::diesel_pool::DB;
use dal::models::visitor_log::*;
use util::log::Ip;

#[get("/about")]
pub fn get_about(db: DB, ip: Ip) -> Template {
    // record visitor
    let ip_address = IpNetwork::from(ip.0);
    let new_visitor_log = NewVisitorLog::new(&ip_address, 0);
    NewVisitorLog::insert(&new_visitor_log, db.conn());

    let mut map = HashMap::new();
    map.insert("foo", "bar");
    Template::render("about", &map)
}
