#![warn(clippy::style, clippy::pedantic)]
#![allow(clippy::no_effect_underscore_binding)]

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use models::{Author, Book};
use std::env;

pub mod models;
pub mod schema;

type ApiResult<T> = Result<T, diesel::result::Error>;

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

/// List authors in the database.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `list_authors`
pub fn list_authors(connector: &mut PgConnection) -> ApiResult<Vec<Author>> {
    use self::schema::authors::dsl::authors;
    authors.load::<Author>(connector)
}

/// Get a specific author from the database
///
/// Find an author with holding the specific identifier `id`.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `get_author`
pub fn get_author(
    connector: &mut PgConnection,
    id: String,
) -> ApiResult<Author> {
    use self::schema::authors::dsl::authors;
    authors.find(id).first(connector)
}

/// Delete a specific author from the database
///
/// Delete the author holding the identifier `id`.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `delete_author`
pub fn delete_author(
    connector: &mut PgConnection,
    id: String,
) -> ApiResult<()> {
    use self::schema::authors::dsl::authors;
    match diesel::delete(authors.find(id)).execute(connector) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// List books in the database.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `list_books`
pub fn list_books(connector: &mut PgConnection) -> ApiResult<Vec<Book>> {
    use self::schema::books::dsl::books;
    books.load::<Book>(connector)
}

/// Get a specific book from the database
///
/// Find an book with holding the specific identifier `id`.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `get_book`
pub fn get_book(
    connector: &mut PgConnection,
    identifier: String,
) -> ApiResult<Book> {
    use self::schema::books::dsl::books;
    books.find(identifier).first(connector)
}

/// Delete a specific book from the database
///
/// Delete the book holding the identifier `id`.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `delete_book`
pub fn delete_book(
    connector: &mut PgConnection,
    identifier: String,
) -> ApiResult<()> {
    use self::schema::books::dsl::books;
    match diesel::delete(books.find(identifier)).execute(connector) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
