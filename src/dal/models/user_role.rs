use chrono::NaiveDateTime;
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct RoleUser {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}
