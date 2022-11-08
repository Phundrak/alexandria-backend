#![warn(clippy::style, clippy::pedantic)]
#![allow(clippy::no_effect_underscore_binding, clippy::needless_pass_by_value)]

#[macro_use]
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use rocket::http::{Method, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;
use std::error::Error;

macro_rules! get_connector {
    ($db:expr) => {
        match $db.pool.get() {
            Ok(val) => val,
            Err(_) => {
                return Err(status::Custom(
                    Status::InternalServerError,
                    "Failed to connect to the database".to_owned(),
                ));
            }
        }
    };
}

#[derive(Copy, Clone)]
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

macro_rules! server_error {
    ($kind:expr,$message:expr) => {
        Err(status::Custom($kind, $message))
    };
    ($kind:expr) => {
        Err(status::Custom($kind))
    }
}

macro_rules! json_val_or_error {
    ($result:expr) => {
        match $result {
            Ok(val) => Ok(Json(val)),
            Err(e) => {
                info!("Error: {}", e.to_string());
                server_error!(Status::InternalServerError, e.to_string())
            }
        }
    };
}

pub mod server;

fn make_cors(
    allowed_origins: AllowedOrigins,
) -> Result<rocket_cors::Cors, rocket_cors::Error> {
    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install().unwrap();
    dotenv().ok();

    // NOTE: Maybe handle allowed origins through an env variable?
    let allowed_origins = AllowedOrigins::some_regex(&[".*"]);
    let cors = make_cors(allowed_origins)?;

    #[allow(clippy::let_underscore_drop)]
    let _ = rocket::build()
        .attach(cors)
        .mount(
            "/author",
            routes![
                server::author::list,   // /     GET
                server::author::new,    // /     POST
                server::author::update, // /     PUT
                server::author::find,   // /find GET
                server::author::get,    // /:id  GET
                server::author::delete, // /:id  DELETE
            ],
        )
        .mount(
            "/book",
            routes![
                server::book::list,   // /     GET
                server::book::new,    // /     POST
                server::book::find,   // /find GET
                server::book::get,    // /:id  GET
                server::book::delete, // /:id  DELETE
                // Fragments
                server::fragment::list // /:id/fragments GET
            ],
        )
        .mount(
            "/fragment",
            routes![
                server::fragment::get,     // /:id         GET
                server::fragment::delete,  // /:id         DELETE
                server::fragment::reorder, // /:id/reorder PUT
            ],
        )
        .manage(ServerState {
            pool: alexandria::db::get_connection_pool(),
            api_key: env::var("ALEXANDRIA_ADMIN_KEY")
                .expect("ALEXANDRIA_ADMIN_KEY must be set!"),
        })
        .launch()
        .await?;
    Ok(())
}
