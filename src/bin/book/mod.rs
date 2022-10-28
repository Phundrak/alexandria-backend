use crate::{ApiKey, Json, JsonResponse, ServerState};

use alexandria::models::Book;
use rocket::http::Status;
use rocket::response::status;
use rocket::{log::private::info, State};
use uuid::Uuid;

#[post("/", format = "json", data = "<book>")]
pub fn new(
    book: Json<Book>,
    db: &State<ServerState>,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut db.pool.get().unwrap();
    match alexandria::book::new(connector, book.into_inner()) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

#[get("/")]
pub fn list(db: &State<ServerState>) -> JsonResponse<Vec<Book>> {
    info!("Listing books");
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::book::list(connector))
}

#[get("/<id>")]
pub fn get(db: &State<ServerState>, id: Uuid) -> JsonResponse<Book> {
    info!("Retrieving book {}", id);
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::book::get(connector, id))
}

#[delete("/<id>")]
pub fn delete(
    db: &State<ServerState>,
    id: Uuid,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::book::delete(connector, id))
}
