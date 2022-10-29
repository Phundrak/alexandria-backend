use crate::{ApiKey, Json, JsonResponse, ServerState};

use alexandria::models::Author;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::Deserialize;
use rocket::{log::private::info, State};
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserInput<'r> {
    pub firstname: Option<&'r str>,
    pub lastname: Option<&'r str>,
    pub penname: Option<&'r str>,
}

impl<'r> UserInput<'r> {
    fn is_valid(&self) -> bool {
        !(self.firstname.is_none()
            && self.lastname.is_none()
            && self.penname.is_none())
    }
}

macro_rules! str_to_string_or_none {
    ($val:expr) => {
        match $val {
            Some(val) => Some(val.to_string()),
            None => None,
        }
    };
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
    json_val_or_error!(alexandria::author::list(connector))
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
    let author = author.clone().into_inner();
    if !author.is_valid() {
        return Err(status::Custom(
            Status::NotAcceptable,
            format!("At least one field must be full. Received {:?}", author),
        ));
    }
    let new_author = Author {
        id: Uuid::new_v4(),
        penname: str_to_string_or_none!(&author.penname),
        firstname: str_to_string_or_none!(&author.firstname),
        lastname: str_to_string_or_none!(&author.lastname),
    };
    match alexandria::author::new(connector, new_author) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

#[put("/", format = "json", data = "<author>")]
pub fn update(
    author: Json<Author>,
    db: &State<ServerState>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    match alexandria::author::update(connector, author.into_inner()) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

#[get("/find?<name>")]
pub fn find(
    db: &State<ServerState>,
    name: String,
) -> JsonResponse<Vec<Author>> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(alexandria::author::find(connector, &name))
}

#[get("/<id>")]
pub fn get(db: &State<ServerState>, id: Uuid) -> JsonResponse<Author> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(alexandria::author::get(connector, id))
}

#[delete("/<id>")]
pub fn delete(
    db: &State<ServerState>,
    id: Uuid,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(alexandria::author::delete(connector, id))
}
