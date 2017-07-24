use chrono::NaiveDateTime;
use dal::schema::tag;
use dal::schema::tag::dsl::tag as all_tags;
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[table_name="tag"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub descirption: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

impl Tag {
    pub fn query_all(conn: &PgConnection) -> Vec<Tag> {
        all_tags.order(tag::id.desc()).load::<Tag>(conn).unwrap()
    }
}
