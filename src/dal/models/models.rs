use chrono::NaiveDateTime;
#[table_name="post"]
#[derive(Serialize,Queryable, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub create_time: NaiveDateTime,
    pub publish_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
    pub published: bool,
    pub user_id: String,
}
#[table_name="post_tag"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct PostTag {
    pub id: String,
    pub post_id: String,
    pub tag_id: String,
    pub modify_time: String,
    pub create_time: String,
}
#[table_name="tag"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub descirption: String,
    pub create_time: String,
    pub modify_time: String,
}
#[table_name="role"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
    pub create_time: String,
    pub modify_time: String,
}
#[table_name="user"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,
    pub avatar_url: String,
    pub create_time: String,
    pub modify_time: String,
}
#[table_name="user_role"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct RoleUser {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub create_time: String,
    pub modify_time: String,
}
#[table_name="category"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Cateogory {
    pub id: String,
    pub name: String,
    pub descirption: String,
    pub create_time: String,
    pub modify_time: String,
}
#[derive(Queryable, Debug)]
pub struct Count {
    pub id: i32,
    pub clicks: i32,
}

#[derive(Serialize, Debug, Clone)]
enum Status {
    Draft,
    Published,
    Deleted,
}
