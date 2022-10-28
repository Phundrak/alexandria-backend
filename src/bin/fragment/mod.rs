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
pub struct FragmentToRank {
    pub to: i32,
}

// - [X] ~/book/:id/fragments~ GET
// - [ ] ~/fragment~ POST
// - [X] ~/fragment/:id~ GET
// - [ ] ~/fragment/:id~ PUT
// - [X] ~/fragment/:id~ DELETE
// - [X] ~/fragment/:id/reorder~ PUT

#[get("/<book_id>/fragments")]
pub fn list(
    db: &State<ServerState>,
    book_id: Uuid,
) -> JsonResponse<Vec<fragment::Simple>> {
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::fragment::list(connector, book_id))
}

#[get("/<id>")]
pub fn get(db: &State<ServerState>, id: Uuid) -> JsonResponse<Bookfragment> {
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::fragment::get(connector, id))
}

#[delete("/<id>")]
pub fn delete(db: &State<ServerState>, id: Uuid, _key: ApiKey<'_>) -> JsonResponse<()> {
    let connector = &mut db.pool.get().unwrap();
    json_val_or_error!(alexandria::fragment::delete(connector, id))
}


#[put("/<id>/reorder", format = "json", data = "<to>")]
pub fn reorder(
    db: &State<ServerState>,
    id: Uuid,
    to: Json<FragmentToRank>,
    _key: ApiKey<'_>,
) -> JsonResponse<()> {
    let connector = &mut db.pool.get().unwrap();
    match alexandria::fragment::move_frag(connector, id, to.to) {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            Err(status::Custom(Status::InternalServerError, e.to_string()))
        }
    }
}
