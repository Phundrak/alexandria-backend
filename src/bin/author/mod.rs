use crate::{ApiKey, Json, JsonResponse, ServerState};

use alexandria::models::Author;
use rocket::http::Status;
use rocket::response::status;
use rocket::{log::private::info, State};
use uuid::Uuid;

#[post("/", format = "json", data = "<author>")]
pub fn new(
    author: Json<Author>,
    db: &State<ServerState>,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut db.pool.get().unwrap();
    match alexandria::author::new(connector, author.into_inner()) {
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
    let connector = &mut db.pool.get().unwrap();
    match alexandria::author::update(connector, author.into_inner()) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

#[get("/")]
pub fn list(db: &State<ServerState>) -> JsonResponse<Vec<Author>> {
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::author::list(connector))
}

#[get("/<id>")]
pub fn get(db: &State<ServerState>, id: Uuid) -> JsonResponse<Author> {
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::author::get(connector, id))
}

#[delete("/<id>")]
pub fn delete(
    db: &State<ServerState>,
    id: Uuid,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::author::delete(connector, id))
}
