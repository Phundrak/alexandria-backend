use crate::db::book::{self, SearchQuery};
use crate::db::get_connector;
use crate::models::{Book, BookType};
use crate::server::{json_val_or_error, make_error};
use crate::{ApiKey, Json, JsonResponse, ServerState};

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::Deserialize;
use rocket::State;
use tracing::info;
use uuid::Uuid;

/// Data the user can send to create or update a book
#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInput {
    pub title: String,
    pub author: Uuid,
    pub isbn: Option<Vec<Option<String>>>,
    pub cover: Option<String>,
    pub publisher: Option<String>,
    pub published: Option<chrono::NaiveDate>,
    pub genre: Option<Vec<Option<String>>>,
    pub synopsis: Option<String>,
    pub booktype: BookType,
}

impl From<UserInput> for Book {
    fn from(other: UserInput) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: other.title,
            author: other.author,
            isbn: other.isbn,
            cover: other.cover,
            publisher: other.publisher,
            published: other.published,
            genre: other.genre,
            synopsis: other.synopsis,
            booktype: other.booktype,
        }
    }
}

/// List all books in the database
///
/// # Errors
///
/// If an internal error happens, return a 500 error to the user.
/// Otherwise, send an array of books in Json format.
#[get("/")]
pub fn list(db: &State<ServerState>) -> JsonResponse<Vec<Book>> {
    info!("Listing books");
    let connector = &mut get_connector!(db);
    json_val_or_error!(book::list(connector))
}

/// Create a new book.
///
/// Create a new book based on `book` received as Json data.
///
/// # Errors
///
/// Any server error is returned to the user as a 500 HTTP error.
#[post("/", format = "json", data = "<book>")]
pub fn new(
    book: Json<UserInput>,
    db: &State<ServerState>,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    match book::new(connector, book.into_inner().into()) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

/// Update a book
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
#[put("/", format = "json", data = "<book>")]
pub fn update(book: Json<Book>, db: &State<ServerState>) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    let book = book.into_inner();
    let id = book.id;
    match book::update(connector, book) {
        Ok(val) => {
            if val == 1 {
                Ok(Json(()))
            } else {
                make_error!(
                    Status::InternalServerError,
                    format!("Something went wrong, {} books updated", val)
                )
            }
        }
        Err(e) => {
            use diesel::result::Error::NotFound;
            match e {
                NotFound => make_error!(
                    Status::NotFound,
                    format!("Book ID {} not found", id)
                ),
                other => {
                    make_error!(Status::InternalServerError, other.to_string())
                }
            }
        }
    }
}

/// Find books matching the title `name`
///
/// Return in a vector all books whose title contain `name`. Typos are
/// not implemented as for now.
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
#[get("/find?<name>")]
pub fn find(db: &State<ServerState>, name: String) -> JsonResponse<Vec<Book>> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(book::find(connector, &name))
}

/// Perform an advanced search query for books.
///
/// Search for books matching a certain title, type, or genres. See
/// `SearchQuery` for more details.
#[get("/find", format = "json", data = "<search>")]
pub fn advanced_find(
    db: &State<ServerState>,
    search: Json<SearchQuery>,
) -> JsonResponse<Vec<Book>> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(book::advanced_find(connector, search.into_inner()))
}

/// Get a book by its ID
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
#[get("/<id>")]
pub fn get(db: &State<ServerState>, id: Uuid) -> JsonResponse<Book> {
    info!("Retrieving book {}", id);
    let connector = &mut get_connector!(db);
    match book::get(connector, id) {
        Ok(val) => Ok(Json(val)),
        Err(e) => {
            use diesel::result::Error::NotFound;
            match e {
                NotFound => make_error!(
                    Status::NotFound,
                    format!("Book ID {} not found", id)
                ),
                other => {
                    make_error!(Status::InternalServerError, other.to_string())
                }
            }
        }
    }
}

/// Delete the book with a set ID
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
// TODO: Handle book not found
#[delete("/<id>")]
pub fn delete(
    db: &State<ServerState>,
    id: Uuid,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(book::delete(connector, id))
}
