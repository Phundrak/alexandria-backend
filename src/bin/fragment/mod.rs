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

#[get("/<book_id>/fragments")]
pub fn list(
    db: &State<ServerState>,
    book_id: Uuid,
) -> JsonResponse<Vec<fragment::Simple>> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(alexandria::fragment::list(connector, book_id))
}

#[get("/<id>")]
pub fn get(db: &State<ServerState>, id: Uuid) -> JsonResponse<Bookfragment> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(alexandria::fragment::get(connector, id))
}

#[delete("/<id>")]
pub fn delete(db: &State<ServerState>, id: Uuid, _key: ApiKey<'_>) -> JsonResponse<()> {
    let connector = &mut get_connector!(db);
    json_val_or_error!(alexandria::fragment::delete(connector, id))
}


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
