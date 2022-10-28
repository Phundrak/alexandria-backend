use diesel::{insert_into, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{models::Author, utils::ApiResult};
use crate::schema::authors::dsl::authors;

/// List authors in the database.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `list`
pub fn list(connector: &mut PgConnection) -> ApiResult<Vec<Author>> {
    authors.load::<Author>(connector)
}

/// Add a new author in the database
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `new`
pub fn new(connector: &mut PgConnection, author: Author) -> ApiResult<usize> {
    insert_into(authors).values(author).execute(connector)
}

/// Update an author in the database
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `update`
pub fn update(
    connector: &mut PgConnection,
    author: Author,
) -> ApiResult<usize> {
    insert_into(authors).values(author).execute(connector)
}

/// Get a specific author from the database
///
/// Find an author with holding the specific identifier `id`.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `get`
pub fn get(connector: &mut PgConnection, id: Uuid) -> ApiResult<Author> {
    authors.find(id).first(connector)
}

/// Delete a specific author from the database
///
/// Delete the author holding the identifier `id`.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `delete`
pub fn delete(connector: &mut PgConnection, id: Uuid) -> ApiResult<()> {
    match diesel::delete(authors.find(id)).execute(connector) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
