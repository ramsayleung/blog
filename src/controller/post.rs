use rocket_contrib::Template;
use rocket_contrib::Json;

use dal::models::post::*;
use dal::diesel_pool::DB;
use util::log::Ip;
use util::log::log_to_db;
use util::response::footer_context;

const VISITOR: i32 = 0;

#[get("/show_post")]
pub fn show_post(db: DB) -> Json<Vec<PostView>> {
    let (result,_more) = Post::query_latest_five_post(db.conn());
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

    let result = Post::query_by_slug_url(db.conn(), &slug_url);
    if let Some(post) = result.first() {
        let hit_time = post.hit_time;
        Post::increase_hit_time(db.conn(), post.id, hit_time + 1);
    }
    let mut context = footer_context();
    if let Some(post) = result.first() {
        context.add("post", post);
    }
    Template::render("post", &context)
    // Json(result)
}

#[get("/posts")]
pub fn get_post(db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);

    let result = Post::query_all_published_post(db.conn());
    let mut context = footer_context();
    context.add("posts", &result);
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
