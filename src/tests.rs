extern crate bcrypt;
use bcrypt::{hash, verify};
use dal::models::user;
use util::time::get_now;
/// run this test with `cargo test -- --nocapture`
#[test]
fn hash_passwd_test() {
    let passwd = "hunter2";
    let result = hash(&passwd, 10).unwrap();
    assert_eq!(true, verify(&passwd, &result).unwrap());
    println!("{:?}", &result);
    let user = user::NewUser {
        username: String::from("ramsay"),
        hashed_password: result.clone(),
        create_time: get_now(),
        modify_time: get_now(),
        email: String::from("ramsayleung@gmail.com"),
        avatar_url: String::from("https://imgur.com/index"),
    };
    assert_eq!(&user.hashed_password, &result);
}
