use rocket::serde::json::Json;
use rocket_dyn_templates::Template;

use crate::dal::diesel_pool::{DB, POST_CACHE};
use crate::dal::models::post::*;
use crate::util::log::log_to_db;
use crate::util::log::Ip;
use crate::util::response::footer_context;
use serde_json::json;

use rocket::get;
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
    let post = hashmap.entry(slug_url).or_insert_with_key(|key| {
        Post::query_by_slug_url(db.conn(), &key)
            .first()
            .unwrap()
            .clone()
    });

    let hit_time = post.hit_time;
    Post::increase_hit_time(db.conn(), post.id, hit_time + 1);
    context.insert("post", post);
    Template::render("post", &context.into_json())
}

#[get("/posts")]
pub fn get_post(db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);

    let result = Post::query_all_published_post(db.conn());
    let mut context = footer_context();
    context.insert("posts", &result);
    Template::render("list", &context.into_json())
}

#[get("/tag/<tag_name>")]
pub fn get_posts_by_tag(tag_name: String, db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);
    let query_tag = json!([tag_name]);
    let result = Post::query_by_tag(query_tag, db.conn());
    let view_posts: Vec<PostView> = result
        .iter()
        .map(PostView::model_convert_to_postview)
        .collect::<Vec<PostView>>();
    let mut context = footer_context();
    context.insert("posts", &view_posts);
    Template::render("list", &context.into_json())
}

#[get("/pages/<offset>")]
pub fn get_posts_pages(offset: i64, db: DB, ip: Ip) -> Template {
    log_to_db(ip, &db, VISITOR);
    let more = Post::pagination_query(offset, db.conn()).len() >= LIMIT as usize;
    let mut context = footer_context();
    context.insert(
        "posts",
        &Post::pagination_query(offset, db.conn())
            .iter()
            .map(PostView::model_convert_to_postview)
            .collect::<Vec<PostView>>(),
    );
    context.insert("more", &more);
    context.insert("page_num", &offset);
    Template::render("list", &context.into_json())
}
#[get("/post_list")]
pub fn get_post_list(db: DB) -> Json<Vec<PostView>> {
    Json(
        Post::query_all_published_post(db.conn())
            .iter()
            .map(PostView::model_convert_to_postview)
            .collect::<Vec<PostView>>(),
    )
}
