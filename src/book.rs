use diesel::{insert_into, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{models::Book, utils::ApiResult};
use crate::schema::books::dsl::books;

/// Add a new book in the database
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `new`
pub fn new(connector: &mut PgConnection, book: Book) -> ApiResult<usize> {
    insert_into(books).values(book).execute(connector)
}

/// List books in the database.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `list`
pub fn list(connector: &mut PgConnection) -> ApiResult<Vec<Book>> {
    books.load::<Book>(connector)
}

/// Get a specific book from the database
///
/// Find an book with holding the specific identifier `id`.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `get`
pub fn get(connector: &mut PgConnection, identifier: Uuid) -> ApiResult<Book> {
    books.find(identifier).first(connector)
}

/// Delete a specific book from the database
///
/// Delete the book holding the identifier `id`.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `delete`
pub fn delete(connector: &mut PgConnection, identifier: Uuid) -> ApiResult<()> {
    match diesel::delete(books.find(identifier)).execute(connector) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
