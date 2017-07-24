use chrono::NaiveDateTime;
use dal::schema::user_role;
use dal::schema::user_role::dsl::user_role as all_user_roles;
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct RoleUser {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

impl RoleUser {
    pub fn query_all(conn: &PgConnection) -> Vec<RoleUser> {
        all_user_roles
            .order(user_role::id.desc())
            .load::<RoleUser>(conn)
            .unwrap()
    }
}
