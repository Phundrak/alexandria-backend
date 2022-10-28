use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub type ApiResult<T> = Result<T, diesel::result::Error>;

/// Create a connection pool to the database.
///
/// The pool and the connection manager are both handled by Diesel’s
/// r2d2 crate. It ensures the database connection manager can be
/// accessed asynchronously across the server.
#[must_use]
pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
