use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{json::Json, Serialize};
use std::env;

#[macro_use]
extern crate rocket;

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

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse {
    code: i32,
    message: String,
}

type JsonResponse<T> = Result<Json<T>, Json<ApiResponse>>;

macro_rules! json_val_or_error {
    ($result:expr) => {
        match $result {
            Ok(val) => Ok(Json(val)),
            Err(e) => {
                info!("Error: {}", e.to_string());
                Err(Json(ApiResponse {
                    code: 500,
                    message: e.to_string(),
                }))
            }
        }
    };
}

//* Authors
// - [X] ~/author~ POST
// - [ ] ~/author~ PUT
// - [ ] ~/author/find~ GET
// - [ ] ~/author/findByLastName~ GET
// - [X] ~/author/:id~ GET
// - [ ] ~/author/:id~ POST
// - [X] ~/author/:id~ DELETE
// - [X] ~/authors~ GET
// - [ ] ~/authors~ POST
// - [ ] ~/authors~ PUT
//
// * Book
// - [ ] ~/book~ POST
// - [ ] ~/book~ PUT
// - [ ] ~/book/find~ GET
// - [X] ~/book/:id~ GET
// - [ ] ~/book/:id~ POST
// - [X] ~/book/:id~ DELETE
// - [X] ~/books~ GET
// - [ ] ~/books~ POST
// - [ ] ~/books~ PUT
//
// * Book Fragment
// - [ ] ~/book/:id/fragments~ GET
// - [ ] ~/book/:id/fragments~ POST
// - [ ] ~/book/:id/fragments/:rank~ GET
// - [ ] ~/book/:id/fragments/:rank~ PUT
// - [ ] ~/book/:id/fragments/:rank~ DELETE
// - [ ] ~/book/:id/fragments/:rank/reorder~ PUT

mod author {
    use std::str::FromStr;

    use crate::{ApiKey, ApiResponse, Json, JsonResponse, ServerState};
    use alexandria::models::Author;
    use rocket::{log::private::info, State};

    use uuid::Uuid;

    #[post("/", format = "json", data = "<author>")]
    pub fn new(
        author: Json<Author>,
        db: &State<ServerState>,
        _key: ApiKey<'_>,
    ) -> JsonResponse<()> {
        let connector = &mut db.pool.get().unwrap();
        match alexandria::new_author(connector, author.into_inner()) {
            Ok(_) => Ok(Json(())),
            Err(e) => Err(Json(ApiResponse {
                code: 500,
                message: e.to_string(),
            })),
        }
    }

    #[put("/", format = "json", data = "<author>")]
    pub fn update(
        author: Json<Author>,
        db: &State<ServerState>,
    ) -> JsonResponse<()> {
        let connector = &mut db.pool.get().unwrap();
        match alexandria::update_author(connector, author.into_inner()) {
            Ok(_) => Ok(Json(())),
            Err(e) => Err(Json(ApiResponse {
                code: 500,
                message: e.to_string(),
            })),
        }
    }

    #[get("/")]
    pub fn list(db: &State<ServerState>) -> JsonResponse<Vec<Author>> {
        let connector = &mut db.pool.get().unwrap();
        json_val_or_error!(alexandria::list_authors(connector))
    }

    #[get("/<id>")]
    pub fn get(db: &State<ServerState>, id: String) -> JsonResponse<Author> {
        let connector = &mut db.pool.get().unwrap();
        match Uuid::from_str(&id) {
            Ok(val) => {
                json_val_or_error!(alexandria::get_author(connector, val))
            }
            Err(e) => Err(Json(ApiResponse {
                code: 406,
                message: e.to_string(),
            })),
        }
    }

    #[delete("/<id>")]
    pub fn delete(
        db: &State<ServerState>,
        id: String,
        _key: ApiKey<'_>,
    ) -> JsonResponse<()> {
        let connector = &mut db.pool.get().unwrap();
        match Uuid::from_str(&id) {
            Ok(val) => {
                json_val_or_error!(alexandria::delete_author(connector, val))
            }
            Err(e) => Err(Json(ApiResponse {
                code: 406,
                message: e.to_string(),
            })),
        }
    }
}

mod book {
    use std::str::FromStr;

    use crate::{ApiKey, ApiResponse, Json, JsonResponse, ServerState};
    use alexandria::models::Book;
    use rocket::{log::private::info, State};

    use uuid::Uuid;

    #[post("/", format = "json", data = "<book>")]
    pub fn new(
        book: Json<Book>,
        db: &State<ServerState>,
        _key: ApiKey<'_>,
    ) -> JsonResponse<()> {
        let connector = &mut db.pool.get().unwrap();
        match alexandria::new_book(connector, book.into_inner()) {
            Ok(_) => Ok(Json(())),
            Err(e) => Err(Json(ApiResponse {
                code: 500,
                message: e.to_string(),
            })),
        }
    }

    #[get("/")]
    pub fn list(db: &State<ServerState>) -> JsonResponse<Vec<Book>> {
        info!("Listing books");
        let connector = &mut db.pool.get().unwrap();
        json_val_or_error!(alexandria::list_books(connector))
    }

    #[get("/<id>")]
    pub fn get(db: &State<ServerState>, id: String) -> JsonResponse<Book> {
        info!("Retrieving book {}", id);
        let connector = &mut db.pool.get().unwrap();
        match Uuid::from_str(&id) {
            Ok(id) => json_val_or_error!(alexandria::get_book(connector, id)),
            Err(e) => Err(Json(ApiResponse {
                code: 406,
                message: e.to_string(),
            })),
        }
    }

    #[delete("/<id>")]
    pub fn delete(
        db: &State<ServerState>,
        id: String,
        _key: ApiKey<'_>,
    ) -> JsonResponse<()> {
        let connector = &mut db.pool.get().unwrap();
        match Uuid::from_str(&id) {
            Ok(id) => {
                json_val_or_error!(alexandria::delete_book(connector, id))
            }
            Err(e) => Err(Json(ApiResponse {
                code: 406,
                message: e.to_string(),
            })),
        }
    }
}

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
            pool: alexandria::get_connection_pool(),
            api_key: env::var("ALEXANDRIA_ADMIN_KEY")
                .expect("ALEXANDRIA_ADMIN_KEY must be set!"),
        })
}
