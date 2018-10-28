use diesel;
use diesel::prelude::*;
use diesel::sql_types::{Timestamp, BigInt};
use diesel::pg::PgConnection;
use diesel::expression::sql_literal::sql;
use chrono::NaiveDateTime;
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
impl VisitorLog {
    pub fn query_login_user(conn: &PgConnection, user_id: i32) -> Vec<VisitorLog> {
        all_visitor_log
            .filter(visitor_log::user_id.eq(user_id))
            .order(visitor_log::access_time.desc())
            .load::<VisitorLog>(conn)
            .expect("Error when finding loginned user")
    }

    pub fn count_daily_page_view(conn: &PgConnection) -> Vec<(NaiveDateTime, i64)> {
        sql::<(Timestamp, BigInt)>("SELECT date_trunc('day', access_time) ,
    count(*) FROM visitor_log GROUP BY 1  ORDER BY 1")
                .get_results(conn)
                .expect("Error executing raw SQL")
    }

    pub fn count_daily_user_view(conn: &PgConnection) -> Vec<(NaiveDateTime, i64)> {
        sql::<(Timestamp, BigInt)>("SELECT date_trunc('day', access_time) ,
    count(DISTINCT(ip)) FROM visitor_log GROUP BY 1 ORDER BY  1;")
                .get_results(conn)
                .expect("Error executing raw SQL")
    }

    pub fn count_monthly_page_view(conn: &PgConnection) -> Vec<(NaiveDateTime, i64)> {
        sql::<(Timestamp, BigInt)>("SELECT date_trunc('month', access_time) ,
    count(*) FROM visitor_log GROUP BY 1  ORDER BY 1")
                .get_results(conn)
                .expect("Error executing raw SQL")
    }

    pub fn count_monthly_user_view(conn: &PgConnection) -> Vec<(NaiveDateTime, i64)> {
        sql::<(Timestamp, BigInt)>("SELECT date_trunc('month',access_time) ,
    count(DISTINCT ip) FROM visitor_log GROUP BY 1 ORDER BY 1;")
                .get_results(conn)
                .expect("Error executing raw SQL")
    }
}

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
        diesel::insert_into(visitor_log::table)
            .values(new_visitor_log)
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
