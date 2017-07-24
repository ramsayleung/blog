use dal::schema::category;
use dal::schema::category::dsl::category as all_categories;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
#[table_name="category"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Cateogory {
    pub id: String,
    pub name: String,
    pub descirption: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}
impl Cateogory{
    pub fn query_all(conn: &PgConnection)-> Vec<Cateogory>{
        all_categories
            .order(category::id.desc())
            .load::<Cateogory>(conn)
            .unwrap()
    }
}
