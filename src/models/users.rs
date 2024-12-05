use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub primary_account: String,
    pub refresh_token: String,
    pub last_login: std::time::SystemTime,
    pub created_at: std::time::SystemTime,
}
