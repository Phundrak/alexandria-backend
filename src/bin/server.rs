use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::serde::{json::Json, Serialize};

#[macro_use]
extern crate rocket;

pub struct DbConnection {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse {
    code: i32,
    message: String,
}

pub struct AuthorJson {
    id: String,
    first_name: String,
    last_name: String,
    alias: String,
}

pub struct BookJson {
    id: String,
    name: String,
    author: AuthorJson,
    isbns: Vec<String>,
    cover: String,
    publisher: Option<String>,
    published: Option<String>,
    genres: Vec<String>,
    synopsis: Option<String>,
    book_type: String,
}

pub struct BookFragmentJson {
    book: BookJson,
    rank: i32,
    text: String,
    one_shot_sound_source: Option<String>,
}

type JsonResponse<T> = Result<Json<T>, Json<ApiResponse>>;

macro_rules! json_val_or_error {
    ($result:expr) => {
        match $result {
            Ok(val) => Ok(Json(val)),
            Err(e) => Err(Json(ApiResponse {
                code: 500,
                message: e.to_string(),
            })),
        }
    };
}

//* Authors
// - [ ] ~/author~ POST
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
    use crate::{ApiResponse, DbConnection, Json, JsonResponse};
    use alexandria::models::Author;
    use rocket::State;

    #[get("/<id>")]
    pub fn get_author(
        db: &State<DbConnection>,
        id: String,
    ) -> JsonResponse<Author> {
        let connector = &mut db.pool.get().unwrap();
        json_val_or_error!(alexandria::get_author(connector, id))
    }

    #[delete("/<id>")]
    pub fn remove_author(
        db: &State<DbConnection>,
        id: String,
    ) -> JsonResponse<()> {
        let connector = &mut db.pool.get().unwrap();
        json_val_or_error!(alexandria::delete_author(connector, id))
    }
}

mod authors {
    use crate::{ApiResponse, DbConnection, Json, JsonResponse};
    use alexandria::models::Author;
    use rocket::State;

    #[get("/")]
    pub fn list_authors(db: &State<DbConnection>) -> JsonResponse<Vec<Author>> {
        let connector = &mut db.pool.get().unwrap();
        json_val_or_error!(alexandria::list_authors(connector))
    }
}

mod book {
    use crate::{ApiResponse, DbConnection, Json, JsonResponse};
    use alexandria::models::Book;
    use rocket::State;

    #[get("/<id>")]
    pub fn get_book(
        db: &State<DbConnection>,
        id: String,
    ) -> JsonResponse<Book> {
        let connector = &mut db.pool.get().unwrap();
        json_val_or_error!(alexandria::get_book(connector, id))
    }

    #[delete("/<id>")]
    pub fn delete_book(
        db: &State<DbConnection>,
        id: String,
    ) -> JsonResponse<()> {
        let connector = &mut db.pool.get().unwrap();
        json_val_or_error!(alexandria::delete_book(connector, id))
    }
}

mod books {
    use crate::{ApiResponse, DbConnection, Json, JsonResponse};
    use alexandria::models::Book;
    use rocket::State;

    #[get("/")]
    pub fn list_books(db: &State<DbConnection>) -> JsonResponse<Vec<Book>> {
        let connector = &mut db.pool.get().unwrap();
        json_val_or_error!(alexandria::list_books(connector))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/author",
            routes![author::get_author, author::remove_author],
        )
        .mount("/authors", routes![authors::list_authors,])
        .mount("/book", routes![book::get_book, book::delete_book])
        .mount("/books", routes![books::list_books])
        .manage(DbConnection {
            pool: alexandria::get_connection_pool(),
        })
}
