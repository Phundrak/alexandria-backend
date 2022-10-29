use crate::{ApiKey, Json, JsonResponse, ServerState};

use alexandria::fragment;
use alexandria::models::Bookfragment;
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

/// Get all fragments of a book
///
/// Returns an array of simple fragments, see `Simple`.
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
// TODO: Handle book not found
#[get("/<book_id>/fragments")]
pub fn list(
    db: &State<ServerState>,
    book_id: Uuid,
) -> JsonResponse<Vec<fragment::Simple>> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(alexandria::fragment::list(connector, book_id))
}

/// Get a fragment by ID
///
///
/// # Errors
///
/// If an internal error happens, return a 500 error to the user.
/// Otherwise, send an array of books in Json format.
// TODO: Handle fragment not found
#[get("/<id>")]
pub fn get(db: &State<ServerState>, id: Uuid) -> JsonResponse<Bookfragment> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(alexandria::fragment::get(connector, id))
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
    json_val_or_error!(alexandria::fragment::delete(connector, id))
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
// TODO: Handle fragment not found
#[put("/<id>/reorder", format = "json", data = "<to>")]
pub fn reorder(
    db: &State<ServerState>,
    id: Uuid,
    to: Json<ToRank>,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    match alexandria::fragment::move_frag(connector, id, to.to) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
        }
    }
}
