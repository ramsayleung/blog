use std::env;
use tera::Context;

use crate::dal::diesel_pool::DB;
use crate::dal::models::post::Post;
use crate::dal::models::user;
use crate::dal::models::visitor_log::*;
use crate::util::auth::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum ResponseEnum {
    SUCCESS,
    FAILURE,
    ERROR,
}

#[derive(Serialize)]
pub enum ContextEnum<'a, T> {
    String(String),
    Vec(Vec<T>),
    Post(Option<&'a Post>),
    User(Option<&'a user::User>),
}
pub fn template_context(db: &DB, user: User) -> Context {
    let visitor_logs = VisitorLog::query_login_user(db.conn(), user.0);
    let users = user::User::query_by_id(db.conn(), user.0);
    // let mut context = HashMap::new();
    let mut context = Context::new();
    if let Some(user) = users.first() {
        context.insert("username", &user.username);
    }
    if let Some(log) = visitor_logs.first() {
        context.insert("access_time", &log.access_time);
    }
    context
}

pub fn footer_context() -> Context {
    #[cfg(feature = "env-file")]
    {
        dotenv::dotenv().ok();
    }
    let email_url = env::var("EMAIL_URL").expect("EMAIL_URL must be set");
    let stackoverflow_url = env::var("STACKOVERFLOW_URL").expect("STACKOVERFLOW_URL must be set");
    let github_url = env::var("GITHUB_URL").expect("GITHUB_URL must be set");
    let slogan = env::var("SLOGAN").expect("SLOGAN must be set");
    let sub_slogan = env::var("SUB_SLOGAN").expect("SUB_SLOGAN must be set");
    let gitalk_client_id = env::var("GITALK_CLIENT_ID").expect("GITALK_CLIENT_ID must be set");
    let gitalk_client_secret =
        env::var("GITALK_CLIENT_SECRET").expect("GITALK_CLIENT_SECRET must be set");
    let gitalk_repository = env::var("GITALK_REPOSITORY").expect("GITALK_REPOSITORY must be set");
    let gitalk_owner = env::var("GITALK_OWNER").expect("GITALK_OWNER must be set");
    let mut context = Context::new();
    context.insert("email", &email_url);
    context.insert("stackoverflow", &stackoverflow_url);
    context.insert("github", &github_url);
    context.insert("slogan", &slogan);
    context.insert("sub_slogan", &sub_slogan);
    context.insert("gitalk_client_id", &gitalk_client_id);
    context.insert("gitalk_client_secret", &gitalk_client_secret);
    context.insert("gitalk_repository", &gitalk_repository);
    context.insert("gitalk_owner", &gitalk_owner);
    context
}
