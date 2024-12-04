use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}
