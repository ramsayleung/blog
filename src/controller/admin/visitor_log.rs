use diesel; //
use diesel::prelude::*;
use rocket_contrib::Template;
use chrono::prelude::*;
use rocket_contrib::Json;
use chrono::NaiveDateTime;

use std::collections::HashMap;

use dal::diesel_pool::DB;
use dal::models::visitor_log::*;
use util::auth::User;

#[get("/admin/log/daily_pv")]
pub fn count_daily_page_view(db: DB) -> Json<Vec<(NaiveDateTime, i64)>> {
    let results = VisitorLog::count_daily_page_view(db.conn());
    Json(results)
}

#[get("/admin/log/monthly_pv")]
pub fn count_monthly_page_view(db: DB) -> Json<Vec<(NaiveDateTime, i64)>> {
    let results = VisitorLog::count_monthly_page_view(db.conn());
    Json(results)
}
