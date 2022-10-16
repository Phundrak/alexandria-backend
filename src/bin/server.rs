use alexandria::{*, models::{Author, Book}};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::{State, serde::json::Json};

#[macro_use]
extern crate rocket;

struct DbConnection {
    pool: Pool<ConnectionManager<PgConnection>>,
}

struct AuthorJson {
    id: String,
    first_name: String,
    last_name: String,
    alias: String
}

struct BookJson {
    id: String,
    name: String,
    author: AuthorJson,
    isbns: Vec<String>,
    cover: String,
    publisher: Option<String>,
    published: Option<String>,
    genres: Vec<String>,
    synopsis: Option<String>,
    book_type: String
}

struct BookFragmentJson {
    book: BookJson,
    rank: i32,
    text: String,
    one_shot_sound_source: Option<String>,
}

// /author POST PUT GET
// /author/find GET
// /author/:id GET POST DELETE
// /book POST PUT GET
// /book/find GET
// /book/:id POST GET DELETE

#[get("/author")]
fn list_authors(db: &State<DbConnection>) -> Json<Vec<Author>> {
    let connector = &mut db.pool.get().unwrap();
    Json(alexandria::list_authors(connector))
}

#[get("/book")]
fn list_books(db: &State<DbConnection>) -> Json<Vec<Book>> {
    let connector = &mut db.pool.get().unwrap();
    Json(alexandria::list_books(connector))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![list_authors, list_books])
        .manage(DbConnection {
            pool: get_connection_pool()
        })
}
