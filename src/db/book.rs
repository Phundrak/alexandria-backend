use diesel::{
    insert_into, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
};
use rocket::serde::Deserialize;
use uuid::Uuid;

use crate::models::BookType;
use crate::schema::books::dsl;
use crate::{db::ApiResult, models::Book};

/// Advanced search query for books
///
/// See [`BookType`] for possible values of `booktype`.
///
/// [`BookType`]: ../../models/enum.BookType.html
#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SearchQuery {
    pub name: Option<String>,
    pub genre: Option<Vec<String>>,
    pub booktype: Option<BookType>,
}

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
    diesel::update(dsl::books.find(book.id))
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

/// Do an advanced search for books
///
/// Search a book by its title, type, and genres. Similar to [`find`].
/// See [`SearchQuery`] for more details.
///
/// # Errors
///
/// Any error returned by diesel will be forwarded to the function
/// calling `advanced_find`
///
/// [`SearchQuery`]: ./struct.SearchQuery.html
/// [`find`]: ./fn.find.html
pub fn advanced_find(
    connector: &mut PgConnection,
    search: SearchQuery,
) -> ApiResult<Vec<Book>> {
    let books: Vec<Book> = dsl::books
        .filter(
            dsl::title.ilike(format!("%{}%", search.name.unwrap_or_default())),
        )
        .load::<Book>(connector)?
        .iter()
        .filter(|book| if let Some(booktype) = search.booktype {
            booktype == book.booktype
        } else {
            false
        })
        .filter(|book| if let Some(search_genre) = &search.genre {
            if let Some(book_genre) = &book.genre {
                search_genre.iter().all(|g| book_genre.contains(&Some(g.clone())))
            } else {
                false
            }
        } else {
            false
        })
        .map(std::clone::Clone::clone)
        .collect();
    Ok(books)
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
