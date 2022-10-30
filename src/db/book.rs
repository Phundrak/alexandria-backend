use diesel::{
    insert_into, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::books::dsl;
use crate::{models::Book, db::ApiResult};

/// Add a new book in the database
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `new`
pub fn new(connector: &mut PgConnection, book: Book) -> ApiResult<usize> {
    insert_into(dsl::books).values(book).execute(connector)
}

/// List books in the database.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `list`
pub fn list(connector: &mut PgConnection) -> ApiResult<Vec<Book>> {
    dsl::books.load::<Book>(connector)
}

/// Get a specific book from the database
///
/// Find a book with holding the specific identifier `id`.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `get`
pub fn get(connector: &mut PgConnection, identifier: Uuid) -> ApiResult<Book> {
    dsl::books.find(identifier).first(connector)
}

/// Update a book
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `get`
pub fn update(connector: &mut PgConnection, book: Book) -> ApiResult<usize> {
    diesel::update(dsl::books)
        .set(book)
        .execute(connector)
}

/// Find a book by title
///
/// Find a book whose title contains `name`. May not work with typos
/// in the query.
///
/// # Errors
///
/// Any error returned by diesel will be forwarded to the function
/// calling `find`
pub fn find(
    connector: &mut PgConnection,
    name: &String,
) -> ApiResult<Vec<Book>> {
    let query = format!("%{}%", name);
    dsl::books
        .filter(dsl::title.ilike(query))
        .load::<Book>(connector)
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
    match diesel::delete(dsl::books.find(identifier)).execute(connector) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
