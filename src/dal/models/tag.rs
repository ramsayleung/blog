use chrono::NaiveDateTime;
#[table_name="tag"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub descirption: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}
