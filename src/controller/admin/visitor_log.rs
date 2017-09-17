use diesel; //
use diesel::prelude::*;
use dal::diesel_pool::DB;
use dal::models::visitor_log::*;
use std::collections::HashMap;
use rocket_contrib::Template;
use chrono::prelude::*;
use rocket_contrib::Json;

use util::auth::User;

#[get("/admin/log/daily_pv")]
pub fn count_daily_page_view(db: DB) -> &'static str {
    // let result = VisitorLog::count_daily_page_view(db.conn());
    // println!("{:?}", result);
    "source"
}
