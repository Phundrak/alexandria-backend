#[macro_use]
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::serde::json::Json;
use std::env;

pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str, server_key: &str) -> bool {
            key == server_key
        }
        let server_key =
            &request.rocket().state::<ServerState>().unwrap().api_key;
        match request.headers().get_one("x-api-key") {
            None => {
                Outcome::Failure((Status::BadRequest, ApiKeyError::Missing))
            }
            Some(key) if is_valid(key, server_key.as_str()) => {
                Outcome::Success(ApiKey(key))
            }
            Some(_) => {
                Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
            }
        }
    }
}

pub struct ServerState {
    pool: Pool<ConnectionManager<PgConnection>>,
    api_key: String,
}

type JsonResponse<T> = Result<Json<T>, status::Custom<String>>;

macro_rules! json_val_or_error {
    ($result:expr) => {
        match $result {
            Ok(val) => Ok(Json(val)),
            Err(e) => {
                info!("Error: {}", e.to_string());
                Err(status::Custom(Status::InternalServerError, e.to_string()))
            }
        }
    };
}

// * Book Fragment
// - [ ] ~/book/:id/fragments~ GET
// - [ ] ~/book/:id/fragments~ POST
// - [ ] ~/book/:id/fragments/:rank~ GET
// - [ ] ~/book/:id/fragments/:rank~ PUT
// - [ ] ~/book/:id/fragments/:rank~ DELETE
// - [ ] ~/book/:id/fragments/:rank/reorder~ PUT

mod author;
mod book;

#[launch]
fn rocket() -> _ {
    color_eyre::install().unwrap();
    dotenv().ok();
    rocket::build()
        .mount(
            "/author",
            routes![
                author::new,
                author::update,
                author::get,
                author::delete,
                author::list
            ],
        )
        .mount(
            "/book",
            routes![book::new, book::get, book::delete, book::list],
        )
        .manage(ServerState {
            pool: alexandria::utils::get_connection_pool(),
            api_key: env::var("ALEXANDRIA_ADMIN_KEY")
                .expect("ALEXANDRIA_ADMIN_KEY must be set!"),
        })
}
