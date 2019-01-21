use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;

use dal::models::post::*;
use dal::diesel_pool::{DB, POST_CACHE};
use util::log::Ip;
use util::log::log_to_db;
use util::response::footer_context;

const VISITOR: i32 = 0;

#[get("/show_post")]
pub fn show_post(db: DB) -> Json<Vec<PostView>> {
    let (result, _more) = Post::query_latest_five_post(db.conn());
    let view_posts: Vec<PostView> = result
        .iter()
        .map(PostView::model_convert_to_postview)
        .collect::<Vec<PostView>>();
    Json(view_posts)
}

#[get("/<slug_url>")]
pub fn get_post_by_id(slug_url: String, db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);

    let mut context = footer_context();
    // if post is in cache
    let mut hashmap = POST_CACHE.lock().unwrap();
    if hashmap.contains_key(&slug_url) {
        if let Some(post) = hashmap.get(&slug_url) {
            // hit cache
            println!("hit cache");
            let hit_time = post.hit_time;
            Post::increase_hit_time(db.conn(), post.id, hit_time + 1);
            context.insert("post", post);
        }
    } else {
        let result = Post::query_by_slug_url(db.conn(), &slug_url);
        if let Some(post) = result.first() {
            let hit_time = post.hit_time;
            Post::increase_hit_time(db.conn(), post.id, hit_time + 1);
            context.insert("post", post);
            hashmap.insert(slug_url, post.clone());
        }
    }
    Template::render("post", &context)
}

#[get("/posts")]
pub fn get_post(db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);

    let result = Post::query_all_published_post(db.conn());
    let mut context = footer_context();
    context.insert("posts", &result);
    Template::render("list", &context)
}
#[get("/post_list")]
pub fn get_post_list(db: DB) -> Json<Vec<PostView>> {
    let result = Post::query_all_published_post(db.conn());
    let view_posts: Vec<PostView> = result
        .iter()
        .map(PostView::model_convert_to_postview)
        .collect::<Vec<PostView>>();
    Json(view_posts)
}
