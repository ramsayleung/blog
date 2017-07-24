use chrono::NaiveDateTime;
use dal::schema::post_tag;
use dal::schema::post::dsl::post_tag as all_post_tas;
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[table_name="post_tag"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct PostTag {
    pub id: String,
    pub post_id: String,
    pub tag_id: String,
    pub modify_time: NaiveDateTime,
    pub create_time: NaiveDateTime,
}

impl PostTag {
    pub fn query_all(conn: &PgConnection) -> Vec<PostTag> {
        all_post_tags
            .order(post_tag::id.desc())
            .load::<PostTag>(conn)
            .unwrap()
    }
}
