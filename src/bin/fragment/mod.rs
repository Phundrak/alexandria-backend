use crate::{Json, JsonResponse, ServerState};

use alexandria::fragment;
use alexandria::models::Bookfragment;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::uuid::Uuid;
use rocket::{log::private::info, State};

// - [X] ~/book/:id/fragments~ GET
// - [ ] ~/fragment~ POST
// - [X] ~/fragment/:id~ GET
// - [ ] ~/fragment/:id~ PUT
// - [ ] ~/fragment/:id~ DELETE
// - [ ] ~/fragment/:id/reorder~ PUT

#[get("/<book_id>/fragments")]
pub fn list(
    db: &State<ServerState>,
    book_id: Uuid,
) -> JsonResponse<Vec<fragment::Simple>> {
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::fragment::list(connector, book_id))
}

#[get("/<id>")]
pub fn get(
    db: &State<ServerState>,
    id: Uuid,
) -> JsonResponse<Bookfragment> {
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::fragment::get(connector, id))
}
