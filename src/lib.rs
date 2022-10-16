use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use models::{Author, Book};
use std::env;

pub mod models;
pub mod schema;

pub fn get_connection_pool () -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn list_authors(connector: &mut PgConnection) -> Vec<Author> {
    use self::schema::authors::dsl::*;
    authors.load::<Author>(connector).expect("Error fetching authors")
}

pub fn list_books(connector: &mut PgConnection) -> Vec<Book> {
    use self::schema::books::dsl::*;
    books.load::<Book>(connector).expect("Error fetching authors")
}
