use dal::schema::post;
use dal::schema::post::dsl::post as all_posts;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
#[table_name="post"]
#[derive(Serialize,Queryable, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub create_time: NaiveDateTime,
    pub publish_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
    pub published: bool,
    pub user_id: String,
}
impl Post {
    pub fn query_all(conn: &PgConnection) -> Vec<Post> {
        all_posts.order(post::id.desc()).load::<Post>(conn).unwrap()
    }
    pub fn query_latest_five(conn: &PgConnection) -> Vec<Post> {
        all_posts
            // .filter(published.eq(true))
            .limit(5)
            .load::<Post>(conn)
            .expect("Error loading posts")
    }
}
