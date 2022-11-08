use crate::{ApiKey, Json, JsonResponse, ServerState};

use alexandria::db::fragment;
use alexandria::models::{Bookfragment, ImageType, SoundType};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::uuid::Uuid;
use rocket::serde::Deserialize;
use rocket::{log::private::info, State};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ToRank {
    pub to: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInput {
    pub content: String,
    pub oneshotsoundsource: Option<String>,
    pub bgsoundtype: SoundType,
    pub bgsoundsource: Option<String>,
    pub imgtype: ImageType,
    pub imgsource: Option<String>,
    pub book: Uuid,
    pub chapter: i32,
    pub rank: i32,
}

impl From<UserInput> for Bookfragment {
    fn from(other: UserInput) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: other.content,
            oneshotsoundsource: other.oneshotsoundsource,
            bgsoundtype: other.bgsoundtype,
            bgsoundsource: other.bgsoundsource,
            imgtype: other.imgtype,
            imgsource: other.imgsource,
            book: other.book,
            chapter: other.chapter,
            rank: other.rank,
        }
    }
}

/// Get all fragments of a book
///
/// Returns an array of simple fragments, see `Simple`.
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error. If the book pointed at by `book_id` does not exist, a
/// simple empty list is returned.
#[get("/<book_id>/fragments")]
pub fn list(
    db: &State<ServerState>,
    book_id: Uuid,
) -> JsonResponse<Vec<fragment::Simple>> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(fragment::list(connector, book_id))
}

/// Get a fragment by ID
///
/// # Errors
///
/// If an internal error happens, return a 500 error to the user.
/// Otherwise, send an array of books in Json format.
#[get("/<id>")]
pub fn get(db: &State<ServerState>, id: Uuid) -> JsonResponse<Bookfragment> {
    let connector = &mut get_connector!(db);
    match fragment::get(connector, id) {
        Ok(val) => Ok(Json(val)),
        Err(e) => {
            use diesel::result::Error::NotFound;
            match e {
                NotFound => server_error!(
                    Status::NotFound,
                    format!("Fragment with ID {} not found", id)
                ),
                other => server_error!(
                    Status::InternalServerError,
                    other.to_string()
                ),
            }
        }
    }
}

/// Create a new fragment
///
/// If a fragment already exists at the specified rank, shift the
/// existing fragment and all subsequent fragments by one to insert
/// the new fragment. If the fragment’s rank exceeds the amount of
/// fragments already existing, set it to the last logical rank ---
/// i.e. if a fragment at rank 999 is inserted but the last fragment
/// is at rank 41, insert the new fragment at rank 42.
///
/// # Errors
///
/// If an internal error happens, return a 500 error to the user.
/// Otherwise, send an array of books in Json format.
#[post("/", format = "json", data = "<fragment>")]
pub fn new(
    db: &State<ServerState>,
    fragment: Json<UserInput>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    match fragment::new(connector, fragment.into_inner().into()) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

/// Update an existing fragment
///
/// In case the fragment’s rank changes, shift all the necessary
/// fragments to keep continuity in the book, see `new`.
///
/// # Errors
///
/// If an internal error happens, return a 500 error to the user.
/// Otherwise, send an array of books in Json format.
#[put("/", format = "json", data = "<fragment>")]
pub fn update(
    db: &State<ServerState>,
    fragment: Json<Bookfragment>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    let fragment = fragment.into_inner();
    let id = fragment.id;
    match fragment::update(connector, fragment) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            use diesel::result::Error::NotFound;
            match e {
                NotFound => server_error!(
                    Status::NotFound,
                    format!("Fragment ID {} not found", id)
                ),
                other => server_error!(
                    Status::InternalServerError,
                    other.to_string()
                ),
            }
        }
    }
}

/// Delete a fragment by ID
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
// TODO: Handle fragment not found
#[delete("/<id>")]
pub fn delete(
    db: &State<ServerState>,
    id: Uuid,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(fragment::delete(connector, id))
}

/// Reorder a book fragment
///
/// Move a fragment to a new rank. If needed, shift other fragments.
/// If the user tries to move the fragment further than the end of the
/// book, set the fragment as its last fragment and renumber it.
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
#[put("/<id>/reorder", format = "json", data = "<to>")]
pub fn reorder(
    db: &State<ServerState>,
    id: Uuid,
    to: Json<ToRank>,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    match fragment::move_frag_id(connector, id, to.to) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            use diesel::result::Error::NotFound;
            match e {
                NotFound => server_error!(
                    Status::NotFound,
                    format!("Fragment ID {} not found", id)
                ),
                other => server_error!(
                    Status::InternalServerError,
                    other.to_string()
                ),
            }
        }
    }
}
