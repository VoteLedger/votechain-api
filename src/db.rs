use diesel::pg::PgConnection;

struct Db {
    conn: Connection,
}
