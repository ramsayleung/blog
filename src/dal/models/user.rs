use chrono::NaiveDateTime;
use chrono::prelude::*;
use dal::schema::user;
use dal::schema::user::dsl::user as all_users;
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[table_name="user"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub hashed_password: [u8; 24],
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
    pub salt: [u8; 16],
    pub email: String,
    pub avatar_url: String,
}

impl User {
    pub fn query_all(conn: &PgConnection) -> Vec<User> {
        all_users.order(user::id.desc()).load::<User>(conn).unwrap()
    }
}
#[derive(Deserialize, Serialize)]
pub struct NewUser {
    pub username: String,
    pub raw_password: String,
    #[serde(default = "get_now")]
    pub create_time: NaiveDateTime,
    #[serde(default = "get_now")]
    pub modify_time: NaiveDateTime,
    pub email: String,
    pub avatar_url: String,
}
impl NewUser {
    pub fn insert(new_user: &NewUser, conn: &PgConnection) -> bool {
        diesel::insert(new_user)
            .into(user::table)
            .execute(conn)
            .is_ok()
    }
}
fn get_now() -> NaiveDateTime {
    let dt = Local::now();
    let d = NaiveDate::from_ymd(dt.year(), dt.month(), dt.day());
    let t = NaiveTime::from_hms(dt.hour(), dt.minute(), dt.second());
    NaiveDateTime::new(d, t)
}
