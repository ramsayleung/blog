use dal::schema::post;
use dal::schema::post::dsl::post as all_posts;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
#[table_name="post"]
#[derive(Serialize,Queryable, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
    pub published: bool,
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
    pub fn query_by_id(conn: &PgConnection, id: i32) -> Vec<Post> {
        all_posts
            .find(id)
            .load::<Post>(conn)
            .expect("Error finding post")

    }
}
#[derive(Serialize, Debug, Clone)]
pub struct PostView {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub content: String,
    pub create_time: NaiveDateTime,
}
impl PostView {
    pub fn convert_to_post(post: Post) -> PostView {
        PostView {
            id: post.id,
            title: post.title,
            subtitle: post.subtitle,
            content: post.content,
            create_time: post.create_time,
        }
    }
}
