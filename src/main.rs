#![warn(clippy::style, clippy::pedantic)]
#![allow(clippy::no_effect_underscore_binding, clippy::needless_pass_by_value)]

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use dotenvy::dotenv;

#[macro_use]
extern crate rocket;
use rocket::http::{Method, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use std::env;
use std::error::Error;

pub mod db;
pub mod models;
pub mod schema;
pub mod server;

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

fn setup_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install().unwrap();
    setup_logging();

    info!("Reading environment variables");
    dotenv().ok();

    // NOTE: Maybe handle allowed origins through an env variable?
    let allowed_origins = AllowedOrigins::some_regex(&[".*"]);
    let cors = make_cors(allowed_origins)?;

    info!("Getting database connection manager pool");
    let pool = db::get_connection_pool();
    db::run_migrations(&mut pool.get()?)?;

    info!("Launching server");
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
            pool,
            api_key: env::var("ALEXANDRIA_ADMIN_KEY")
                .expect("ALEXANDRIA_ADMIN_KEY must be set!"),
        })
        .launch()
        .await?;
    Ok(())
}
