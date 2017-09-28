use diesel;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;

use util::time::get_now;
use dal::schema::post;
use dal::schema::post::dsl::post as all_posts;

// Because diesel still doesn't support enum, So for convenience, I do it by hand
const ABOUT: i32 = 1;
const POST: i32 = 2;

#[derive(Serialize, Deserialize,Queryable, Debug, Clone,AsChangeset,Identifiable)]
#[table_name = "post"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub raw_content: String,
    pub rendered_content: String,
    pub create_time: NaiveDateTime,
    #[serde(default = "get_now")]
    pub modify_time: NaiveDateTime,
    pub post_type: i32,
    pub hit_time: i32,
    pub published: bool,
}
impl Post {
    pub fn query_all(conn: &PgConnection) -> Vec<Post> {
        all_posts.order(post::id.desc()).load::<Post>(conn).unwrap()
    }
    fn published() -> post::BoxedQuery<'static, Pg> {
        all_posts
            .filter(post::published.eq(true))
            .order(post::create_time.desc())
            .into_boxed()
    }
    pub fn query_all_published_post(conn: &PgConnection) -> Vec<Post> {
        Post::published()
            .filter(post::post_type.eq(POST))
            .load::<Post>(conn)
            .expect("Error loading posts")
    }

    pub fn query_latest_about(conn: &PgConnection) -> Vec<Post> {
        Post::published()
            .filter(post::post_type.eq(ABOUT))
            .load::<Post>(conn)
            .expect("Error loading posts")
    }
    pub fn query_latest_five_post(conn: &PgConnection) -> Vec<Post> {
        Post::published()
            .filter(post::post_type.eq(POST))
            .limit(5)
            .load::<Post>(conn)
            .expect("Error loading posts")
    }
    pub fn pagination_query(conn: &PgConnection) -> Vec<Post> {
        Post::published()
            .offset(0)
            .limit(15)
            .load::<Post>(conn)
            .expect("Error loading posts")
    }
    pub fn query_by_id(conn: &PgConnection, id: i32) -> Vec<Post> {
        all_posts
            .find(id)
            .load::<Post>(conn)
            .expect("Error finding post")

    }
    pub fn delete_with_id(conn: &PgConnection, id: i32) -> bool {
        diesel::delete(all_posts.find(id)).execute(conn).is_ok()
    }
    pub fn update(conn: &PgConnection, post: &Post) -> bool {
        diesel::update(post).set(post).execute(conn).is_ok()
    }
    pub fn increase_hit_time(conn: &PgConnection, id: i32, hit_time: i32) -> bool {
        diesel::update(all_posts.find(id))
            .set(post::hit_time.eq(hit_time))
            .execute(conn)
            .is_ok()
    }
    pub fn query_ten_hottest_posts(conn: &PgConnection) -> Vec<Post> {
        all_posts
            .filter(post::post_type.eq(POST))
            .order(post::hit_time.desc())
            .limit(10)
            .load::<Post>(conn)
            .expect("Error when loading posts")
    }
}
#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "post"]
pub struct NewPost {
    pub title: String,
    pub subtitle: String,
    pub raw_content: String,
    pub rendered_content: String,
    #[serde(default = "get_now")]
    pub create_time: NaiveDateTime,
    #[serde(default = "get_now")]
    pub modify_time: NaiveDateTime,
    pub post_type: i32,
    #[serde(default )]
    pub hit_time: i32,
    pub published: bool,
}
impl NewPost {
    pub fn insert(new_post: &NewPost, conn: &PgConnection) -> bool {
        // use dal::schema::post;
        diesel::insert(new_post)
            .into(post::table)
            .execute(conn)
            .is_ok()
    }
}

#[derive( Deserialize, Serialize)]
pub struct PostView {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub create_time: NaiveDateTime,
    pub post_type: i32,
    pub hit_time: i32,
    pub published: bool,
}
impl PostView {
    pub fn model_convert_to_postview(post: &Post) -> PostView {
        PostView {
            id: post.id,
            title: post.title.to_string(),
            subtitle: post.subtitle.to_string(),
            create_time: post.create_time,
            post_type: post.post_type,
            published: post.published,
            hit_time: post.hit_time,
        }
    }
}
// and in the structure
