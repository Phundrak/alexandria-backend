use crate::db::author;
use crate::db::get_connector;
use crate::models::Author;
use crate::server::{json_val_or_error, make_error};
use crate::{ApiKey, Json, JsonResponse, ServerState};

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::Deserialize;
use rocket::State;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserInput {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub penname: Option<String>,
}

impl UserInput {
    fn is_valid(&self) -> bool {
        !(self.firstname.is_none()
            && self.lastname.is_none()
            && self.penname.is_none())
    }
}

impl From<UserInput> for Author {
    fn from(other: UserInput) -> Self {
        Self {
            id: Uuid::new_v4(),
            firstname: other.firstname,
            lastname: other.lastname,
            penname: other.penname,
        }
    }
}

/// List all authors in the database
///
/// # Errors
///
/// In case of an error, return to the user a Json file containing the
/// error message as well as the appropriate HTTP response.
#[get("/")]
pub fn list(db: &State<ServerState>) -> JsonResponse<Vec<Author>> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(author::list(connector))
}

/// Create a new author
///
/// Receive a complete `UserInput` structure by Json and insert it in
/// the database. The UUID is optional in this case.
///
/// # Error
///
/// Two errors may arise from this function. Either the user did not
/// specify any name for an author (at least one of the fields must be
/// set), in which case the user will receive a 405 response, or an
/// internal server error arises in which case the user will receive a
/// 500 response.
#[post("/", format = "json", data = "<author>")]
pub fn new(
    author: Json<UserInput>,
    db: &State<ServerState>,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    let author = author.into_inner();
    if !author.is_valid() {
        return Err(status::Custom(
            Status::NotAcceptable,
            format!("At least one field must be full. Received {:?}", author),
        ));
    }
    match author::new(connector, author.into()) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

/// Update an existing author
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
#[put("/", format = "json", data = "<author>")]
pub fn update(
    author: Json<Author>,
    db: &State<ServerState>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    match author::update(connector, author.into_inner()) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

/// Find an author by name
///
/// May not work with full names and typos.
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
#[get("/find?<name>")]
pub fn find(
    db: &State<ServerState>,
    name: String,
) -> JsonResponse<Vec<Author>> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(author::find(connector, &name))
}

/// Get an author by ID.
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
#[get("/<id>")]
pub fn get(db: &State<ServerState>, id: Uuid) -> JsonResponse<Author> {
    let connector = &mut get_connector!(db);
    match author::get(connector, id) {
        Ok(val) => Ok(Json(val)),
        Err(e) => {
            use diesel::result::Error::NotFound;
            match e {
                NotFound => make_error!(
                    Status::NotFound,
                    format!("Author ID {} not found", id)
                ),
                other => {
                    make_error!(Status::InternalServerError, other.to_string())
                }
            }
        }
    }
}

/// Delete an author
///
/// # Errors
///
/// Any error from the server will be returned to the user as a 500
/// HTTP error.
// TODO: Handle author not found
#[delete("/<id>")]
pub fn delete(
    db: &State<ServerState>,
    id: Uuid,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(author::delete(connector, id))
}
