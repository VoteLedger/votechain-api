use crate::schema::users::dsl::*;
use diesel::{debug_query, prelude::*};
use log::debug;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub primary_account: String,
    pub refresh_token: String,
    pub last_login: Option<std::time::SystemTime>,
    pub created_at: Option<std::time::SystemTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UserUpdate {
    pub last_login: Option<std::time::SystemTime>,
    pub refresh_token: String,
}

impl User {
    pub fn get_user_by_address(
        conn: &mut PgConnection,
        account_address: &String,
    ) -> QueryResult<User> {
        // Query the database
        users
            .filter(primary_account.eq(account_address))
            .limit(1)
            .get_result::<User>(conn)
    }

    pub fn save(&self, conn: &mut PgConnection) -> QueryResult<usize> {
        // Query the database
        diesel::insert_into(users).values(self).execute(conn)
    }

    pub fn update(&self, conn: &mut PgConnection) -> QueryResult<usize> {
        // Query the database
        diesel::update(users.filter(primary_account.eq(&self.primary_account)))
            .set(&UserUpdate {
                last_login: self.last_login,
                refresh_token: self.refresh_token.clone(),
            })
            .execute(conn)
    }
}
