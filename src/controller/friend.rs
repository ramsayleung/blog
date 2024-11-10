use rocket_dyn_templates::Template;

use crate::dal::diesel_pool::DB;
use crate::dal::models::post::*;
use crate::util::log::log_to_db;
use crate::util::log::Ip;
use crate::util::response::footer_context;

use rocket::get;
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

    Template::render("friend", context.into_json())
}
