use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Outcome::{Failure, Success};
use rocket::Request;

// DB item
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::{Pool, PooledConnection};

// Std Imports
use std::{collections::HashMap, env, lazy::SyncLazy, sync::Mutex};

use crate::dal::models::post::*;

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
pub static DB_POOL: SyncLazy<Pool<ConnectionManager<PgConnection>>> =
    SyncLazy::new(create_db_pool);
pub static POST_CACHE: SyncLazy<Mutex<HashMap<String, Post>>> = SyncLazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});
pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = ();
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(_e) => Failure((Status::InternalServerError, ())),
        }
    }
}
