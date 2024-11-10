use bcrypt::{hash, verify, BcryptError};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
// use std::str;

use diesel::Identifiable;
use diesel::Insertable;
use diesel::Queryable;

use crate::dal::schema::user;
use crate::dal::schema::user::dsl::user as all_users;
use crate::util::time::get_now;
use serde::{Deserialize, Serialize};
const COST: u32 = 10;

#[derive(Serialize, Deserialize, Queryable, Debug, Clone, AsChangeset, Identifiable)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub create_time: NaiveDateTime,
    #[serde(default = "get_now")]
    pub modify_time: NaiveDateTime,
    pub email: String,
    pub avatar_url: Option<String>,
}

impl User {
    pub fn query_all(conn: &PgConnection) -> Vec<User> {
        all_users.order(user::id.desc()).load::<User>(conn).unwrap()
    }
    pub fn query_by_email(conn: &PgConnection, email: &str) -> Vec<User> {
        all_users
            .filter(user::email.eq(email))
            .load::<User>(conn)
            .expect("Error when finding user by email")
    }
    pub fn query_by_id(conn: &PgConnection, id: i32) -> Vec<User> {
        all_users
            .find(id)
            .load::<User>(conn)
            .expect("Error when finding user by email")
    }
    pub fn delete_with_id(conn: &PgConnection, id: i32) -> bool {
        diesel::delete(all_users.find(id)).execute(conn).is_ok()
    }
    pub fn verify(&self, password: &str) -> Result<bool, BcryptError> {
        verify(password, &self.hashed_password)
    }
    pub fn update(conn: &PgConnection, user: &User) -> bool {
        diesel::update(user).set(user).execute(conn).is_ok()
    }
    pub fn change_password(
        conn: &PgConnection,
        id: i32,
        new_raw_password: &str,
        modify_time: &NaiveDateTime,
    ) -> bool {
        let new_hashed_password = hash(new_raw_password, COST).unwrap();
        diesel::update(all_users.find(id))
            .set((
                user::modify_time.eq(modify_time),
                user::hashed_password.eq(new_hashed_password),
            ))
            .execute(conn)
            .is_ok()
    }
}
#[derive(Insertable, Deserialize, Serialize, Debug, Clone)]
#[table_name = "user"]
pub struct NewUser {
    pub username: String,
    pub hashed_password: String,
    #[serde(default = "get_now")]
    pub create_time: NaiveDateTime,
    #[serde(default = "get_now")]
    pub modify_time: NaiveDateTime,
    pub email: String,
    pub avatar_url: String,
}
impl NewUser {
    pub fn insert(new_user: &NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(user::table)
            .values(new_user)
            .execute(conn)
            .is_ok()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
    pub email: String,
    pub avatar_url: String,
}
impl UserInfo {
    pub fn convert_to_new_user(user_info: &UserInfo) -> NewUser {
        let hashed_password = hash(&user_info.password, COST).unwrap();
        NewUser {
            username: user_info.username.to_string(),
            hashed_password,
            create_time: get_now(),
            modify_time: get_now(),
            email: user_info.email.to_string(),
            avatar_url: user_info.avatar_url.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChangePassword {
    pub user_id: i32,
    pub old_password: String,
    pub new_password: String,
}
