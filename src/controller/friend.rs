use rocket_contrib::templates::Template;

use dal::diesel_pool::DB;
use dal::models::post::*;
use util::log::Ip;
use util::log::log_to_db;
use util::response::footer_context;

const VISITOR: i32 = 0;

#[get("/friend")]
pub fn get_friend(db: DB, ip: Ip) -> Template {
    // record visitor
    log_to_db(ip, &db, VISITOR);

    let result = Post::query_latest_friend(db.conn());
    let mut context = footer_context();
    if let Some(post) = result.first() {
        let hit_time = post.hit_time;
        Post::increase_hit_time(db.conn(), post.id, hit_time + 1);
        context.insert("post", post);
    }

    Template::render("friend", &context)
}
