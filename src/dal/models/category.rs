use chrono::NaiveDateTime;
#[table_name="category"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Cateogory {
    pub id: String,
    pub name: String,
    pub descirption: String,
    pub create_time: String,
    pub modify_time: String,
}
