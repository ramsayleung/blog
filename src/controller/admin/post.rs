use crate::dal::diesel_pool::{DB, POST_CACHE};
use crate::dal::models::post::*;
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;

use crate::util::auth::User;
use crate::util::response::template_context;
use crate::util::response::ResponseEnum;

use rocket::{delete, get, post, put};
#[get("/admin/post_list")]
pub fn get_posts(user: User, db: DB) -> Template {
    let result = Post::query_all(db.conn());
    // let mut context = HashMap::new();
    let mut context = template_context(&db, user);
    let post_views: Vec<PostView> = result
        .into_iter()
        .map(|post| PostView::model_convert_to_postview(&post))
        .collect();
    context.insert("posts", &post_views);
    Template::render("admin/post_list", context.into_json())
}
#[get("/admin/post/<id>")]
pub fn get_post(id: i32, db: DB) -> Json<Option<Post>> {
    let result = Post::query_by_id(db.conn(), id);
    Json(result.first().cloned())
}

#[post("/admin/post", data = "<new_post>")]
pub fn add_post(db: DB, new_post: Json<NewPost>) -> Json<ResponseEnum> {
    if NewPost::insert(&new_post.0, db.conn()) {
        Json(ResponseEnum::Success)
    } else {
        Json(ResponseEnum::Error)
    }
}

#[get("/admin/new_post")]
pub fn add_post_page(user: User, db: DB) -> Template {
    let context = template_context(&db, user);
    Template::render("admin/post", context.into_json())
}

#[get("/admin/<id>")]
pub fn edit_post(id: i32, db: DB, user: User) -> Template {
    let result = Post::query_by_id(db.conn(), id);
    let mut context = template_context(&db, user);
    if let Some(post) = result.first() {
        context.insert("post", post);
    }
    Template::render("admin/post", context.into_json())
}
#[delete("/admin/post/<id>")]
pub fn delete_post(id: i32, db: DB) -> Json<ResponseEnum> {
    if Post::delete_with_id(db.conn(), id) {
        Json(ResponseEnum::Success)
    } else {
        Json(ResponseEnum::Error)
    }
}
#[put("/admin/post", data = "<update_post>")]
pub fn update_post(update_post: Json<Post>, db: DB) -> Json<ResponseEnum> {
    if Post::update(db.conn(), &update_post.0) {
        // clear cache
        let mut hashmap = POST_CACHE.lock().unwrap();
        if hashmap.contains_key(&update_post.0.slug_url) {
            hashmap.remove(&update_post.0.slug_url);
        }
        Json(ResponseEnum::Success)
    } else {
        Json(ResponseEnum::Error)
    }
}

#[get("/admin/post/ten_hottest")]
pub fn get_ten_hottest_posts(db: DB) -> Json<Vec<PostView>> {
    let result = Post::query_ten_hottest_posts(db.conn());
    let view_posts: Vec<PostView> = result
        .iter()
        .map(PostView::model_convert_to_postview)
        .collect::<Vec<PostView>>();
    Json(view_posts)
}
