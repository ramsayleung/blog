use chrono::NaiveDateTime;
#[table_name="role"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}
