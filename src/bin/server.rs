use alexandria::{
    models::{Author, Book},
    *,
};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::{serde::json::Json, State};

#[macro_use]
extern crate rocket;

struct DbConnection {
    pool: Pool<ConnectionManager<PgConnection>>,
}

struct AuthorJson {
    id: String,
    first_name: String,
    last_name: String,
    alias: String,
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
    book_type: String,
}

struct BookFragmentJson {
    book: BookJson,
    rank: i32,
    text: String,
    one_shot_sound_source: Option<String>,
}

// /author      POST PUT
// /author/find GET
// /author/:id  POST
// /book        POST PUT
// /book/find   GET
// /book/:id    POST

#[get("/author")]
fn list_authors(db: &State<DbConnection>) -> Json<Vec<Author>> {
    let connector = &mut db.pool.get().unwrap();
    Json(alexandria::list_authors(connector))
}

#[get("/author/<id>")]
fn get_author(db: &State<DbConnection>, id: String) -> Json<Author> {
    let connector = &mut db.pool.get().unwrap();
    Json(alexandria::get_author(connector, id))
}

#[delete("/author/<id>")]
fn remove_author(db: &State<DbConnection>, id: String) {
    let connector = &mut db.pool.get().unwrap();
    alexandria::delete_author(connector, id)
}

#[get("/book")]
fn list_books(db: &State<DbConnection>) -> Json<Vec<Book>> {
    let connector = &mut db.pool.get().unwrap();
    Json(alexandria::list_books(connector))
}

#[get("/book/<id>")]
fn get_book(db: &State<DbConnection>, id: String) -> Json<Book> {
    let connector = &mut db.pool.get().unwrap();
    Json(alexandria::get_book(connector, id))
}

#[delete("/book/<id>")]
fn delete_book(db: &State<DbConnection>, id: String) {
    let connector = &mut db.pool.get().unwrap();
    alexandria::delete_book(connector, id);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                list_authors,
                get_author,
                remove_author,
                list_books,
                get_book,
                delete_book
            ],
        )
        .manage(DbConnection {
            pool: get_connection_pool(),
        })
}
