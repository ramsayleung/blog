use diesel::result::QueryResult;
use diesel; //
use diesel::prelude::*;
use dal::diesel_pool::DB;
use dal::models::user::*;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::Template;
use chrono::prelude::*;
use rocket_contrib::Json;

use util::auth::User;

use crypto::bcrypt;
use std::io::Result;
use rand::{Rng, StdRng};
use std::str;
const DEFAULT_COST: u32 = 10;
const MAX_SALT_SIZE: usize = 16;
const OUTPUT_SIZE: usize = 24;

// fn main() {
//     let (output, salt) = hash("foobar").unwrap();
//     println!("compare : {:?}", verify("foobar", &output, &salt));
// }

pub fn hash(password: &str) -> Result<([u8; OUTPUT_SIZE], [u8; MAX_SALT_SIZE])> {

    let salt = {
        let mut unencoded = [0u8; MAX_SALT_SIZE];
        let mut rng = try!(StdRng::new());
        rng.fill_bytes(&mut unencoded);
        unencoded
    };
    let mut output = [0u8; OUTPUT_SIZE];

    bcrypt::bcrypt(DEFAULT_COST, &salt, password.as_bytes(), &mut output);

    Ok((output, salt))
}
pub fn verify(raw_password: &str, hashed_password: &[u8], salt: &[u8]) -> bool {
    let mut output = [0u8; OUTPUT_SIZE];
    bcrypt::bcrypt(DEFAULT_COST, &salt, raw_password.as_bytes(), &mut output);
    hashed_password
        .iter()
        .zip(output.iter())
        .all(|(a, b)| a == b)
}

#[post("/admin/user",data="<new_user>")]
pub fn add_user(db: DB, new_user: Json<NewUser>) -> &'static str {
    println!("{:?}", &new_user.0);
    "get"
    // if NewUser::insert(&new_user.0, db.conn()) {
    //     "success"
    // } else {
    //     "error"
    // }
}
