use rocket::http::Status;
use rocket::outcome::Outcome::{Error, Success};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

// DB item
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::{Pool, PooledConnection};

// Std Imports
use crate::dal::models::post::*;
use lazy_static::lazy_static;
use std::{collections::HashMap, env, sync::Mutex};

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    #[cfg(feature = "env-file")]
    {
        dotenv::dotenv().ok();
    }
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create pool.")
}

// DB Items
lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
    pub static ref POST_CACHE: Mutex<HashMap<String, Post>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}
pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DB {
    type Error = ();
    async fn from_request(_: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(_e) => Error((Status::InternalServerError, ())),
        }
    }
}
