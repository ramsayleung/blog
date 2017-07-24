use chrono::NaiveDateTime;
#[table_name="post_tag"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct PostTag {
    pub id: String,
    pub post_id: String,
    pub tag_id: String,
    pub modify_time: NaiveDateTime,
    pub create_time: NaiveDateTime,
}
