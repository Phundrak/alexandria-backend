use crate::{ApiKey, Json, JsonResponse, ServerState};

use alexandria::models::{Book, BookType};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::Deserialize;
use rocket::{log::private::info, State};
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
    json_val_or_error!(alexandria::book::list(connector))
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
    match alexandria::book::new(connector, book.into_inner().into()) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
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
    json_val_or_error!(alexandria::book::find(connector, &name))
}

/// Get a book by its ID
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
// TODO: Handle book not found
#[get("/<id>")]
pub fn get(db: &State<ServerState>, id: Uuid) -> JsonResponse<Book> {
    info!("Retrieving book {}", id);
    let connector = &mut get_connector!(db);
    json_val_or_error!(alexandria::book::get(connector, id))
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
    json_val_or_error!(alexandria::book::delete(connector, id))
}
