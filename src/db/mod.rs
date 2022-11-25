pub mod author;
pub mod book;
pub mod fragment;

#[macro_export]
macro_rules! get_connector {
    ($db:expr) => {
        match $db.pool.get() {
            Ok(val) => val,
            Err(_) => {
                return Err(status::Custom(
                    Status::InternalServerError,
                    "Failed to connect to the database".to_owned(),
                ));
            }
        }
    };
}

pub(crate) use get_connector;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, MigrationHarness,
};

/// List of migrations the database may have to perform when
/// alexandria is launching
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Run the list of migrations held by `MIGRATIONS`.
///
/// # Errors
///
/// If any error is encountered while running a migration, return
pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::pg::Pg>,
) -> ApiResult<()> {
    use diesel::result::{DatabaseErrorKind, Error};
    let result = connection.run_next_migration(MIGRATIONS);
    info!("{:?}", result);
    result.map(|_| ()).map_err(|e| {
        info!("{:?}", e);
        Error::DatabaseError(
            DatabaseErrorKind::Unknown,
            Box::new(format!("Error running migrations: {}", e)),
        )
    })
}

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
    info!("Connecting to {}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
