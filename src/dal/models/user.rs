use chrono::NaiveDateTime;
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
