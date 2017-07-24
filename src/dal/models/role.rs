use chrono::NaiveDateTime;
use dal::schema::role;
use dal::schema::role::dsl::role as all_roles;
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[table_name="role"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}
impl Role {
    pub fn query_all(conn: &PgConnection) -> Vec<Role> {
        all_roles.order(role::id.desc()).load::<Role>(conn).unwrap()
    }
}
