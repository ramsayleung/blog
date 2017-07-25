use dal::models::post::*;
use dal::diesel_pool::DB;
#[get("/query_index")]
pub fn query_index(db: DB) {
    let posts = Post::query_latest_five(db.conn());
    for post in posts {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.subtitle);
    }
}
