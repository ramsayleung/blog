use chrono::NaiveDateTime;
use dal::schema::user;
use dal::schema::user::dsl::user as all_users;
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[table_name="user"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,
    pub avatar_url: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

impl User {
    pub fn query_all(conn: &PgConnection) -> Vec<User> {
        all_users.order(user::id.desc()).load::<User>(conn).unwrap()
    }
}
