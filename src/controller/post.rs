use rocket::response::Redirect;
use rocket_contrib::Template;
use rocket_contrib::Json;

use std::collections::HashMap;

use dal::models::post::*;
use dal::diesel_pool::DB;
use util::log::Ip;
use util::log::log_to_db;

const VISITOR: i32 = 0;

#[get("/show_post")]
pub fn show_post(db: DB) -> Json<Vec<PostView>> {
    let result = Post::query_latest_five_post(db.conn());
    let view_posts: Vec<PostView> = result
        .iter()
        .map(PostView::model_convert_to_postview)
        .collect::<Vec<PostView>>();
    Json(view_posts)
}

#[get("/<id>")]
pub fn get_post_by_id(id: i32, db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);

    let result = Post::query_by_id(db.conn(), id);
    if let Some(post) = result.first() {
        let hit_time = post.hit_time;
        Post::increase_hit_time(db.conn(), id, hit_time + 1);
    }
    let mut context = HashMap::new();
    context.insert("post", result.first());
    Template::render("post", &context)
    // Json(result)
}

#[get("/posts")]
pub fn get_post(db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);

    let result = Post::query_all_published_post(db.conn());
    let mut map = HashMap::new();
    map.insert("posts", result);
    Template::render("list", &map)
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
