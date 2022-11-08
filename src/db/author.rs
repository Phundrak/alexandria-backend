use diesel::BoolExpressionMethods;
use diesel::{PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::schema::authors::dsl;
use crate::{models::Author, db::ApiResult};

/// List authors in the database.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `list`
pub fn list(connector: &mut PgConnection) -> ApiResult<Vec<Author>> {
    dsl::authors.load::<Author>(connector)
}

/// Add a new author in the database
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `new`
pub fn new(connector: &mut PgConnection, author: Author) -> ApiResult<usize> {
    diesel::insert_into(dsl::authors)
        .values(author)
        .execute(connector)
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
    diesel::update(dsl::authors.find(author.id))
        .set(author)
        .execute(connector)
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
    dsl::authors.find(id).first(connector)
}

/// Find authors by name
///
/// Find any author whose name contains `name`. May not work with full
/// names of if the query contains a typo.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `find`
pub fn find(
    connector: &mut PgConnection,
    name: &String,
) -> ApiResult<Vec<Author>> {
    let str_query = format!("%{}%", name);
    dsl::authors
        .filter(
            dsl::firstname.ilike(str_query.to_string()).or(dsl::lastname
                .ilike(str_query.to_string())
                .or(dsl::penname.ilike(str_query))),
        )
        .load::<Author>(connector)
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
    match diesel::delete(dsl::authors.find(id)).execute(connector) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
