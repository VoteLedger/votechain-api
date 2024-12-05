use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub primary_account: String,
    pub refresh_token: String,
    pub last_login: std::time::SystemTime,
    pub created_at: std::time::SystemTime,
}
