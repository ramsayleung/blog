use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use ipnetwork::IpNetwork;

use dal::schema::visitor_log;
use dal::schema::visitor_log::dsl::visitor_log as all_visitor_log;
use util::time::get_now;

#[derive( Queryable, Debug, Clone)]
pub struct VisitorLog {
    pub id: i32,
    pub ip: IpNetwork,
    pub access_time: NaiveDateTime,
    pub user_id: i32,
}
impl VisitorLog {}

#[derive(Insertable,Debug, Clone)]
#[table_name="visitor_log"]
pub struct NewVisitorLog {
    pub ip: IpNetwork,
    pub access_time: NaiveDateTime,
    pub user_id: i32,
}
impl NewVisitorLog {
    pub fn new(ip: &IpNetwork, user_id: i32) -> NewVisitorLog {
        NewVisitorLog {
            ip: ip.to_owned(),
            access_time: get_now(),
            user_id: user_id,
        }
    }
    pub fn insert(new_visitor_log: &NewVisitorLog, conn: &PgConnection) -> bool {
        diesel::insert(new_visitor_log)
            .into(visitor_log::table)
            .execute(conn)
            .is_ok()
    }
}
// table! {
//     visitor_log {
//         id -> Integer,
//         ip -> Inet,
//         access_time -> Timestamp,
//         user_id _> Integer,
//     }
// }
