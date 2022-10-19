use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use models::{Author, Book};
use std::env;

pub mod models;
pub mod schema;

type ApiResult<T> = Result<T, diesel::result::Error>;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn list_authors(connector: &mut PgConnection) -> ApiResult<Vec<Author>> {
    use self::schema::authors::dsl::*;
    authors.load::<Author>(connector)
}

pub fn get_author(
    connector: &mut PgConnection,
    id: String,
) -> ApiResult<Author> {
    use self::schema::authors::dsl::*;
    authors.find(id).first(connector)
}

pub fn delete_author(
    connector: &mut PgConnection,
    id: String,
) -> ApiResult<()> {
    use self::schema::authors::dsl::*;
    match diesel::delete(authors.find(id)).execute(connector) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn list_books(connector: &mut PgConnection) -> ApiResult<Vec<Book>> {
    use self::schema::books::dsl::*;
    books.load::<Book>(connector)
}

pub fn get_book(
    connector: &mut PgConnection,
    identifier: String,
) -> ApiResult<Book> {
    use self::schema::books::dsl::*;
    books.find(identifier).first(connector)
}

pub fn delete_book(
    connector: &mut PgConnection,
    identifier: String,
) -> ApiResult<()> {
    use self::schema::books::dsl::*;
    match diesel::delete(books.find(identifier)).execute(connector) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
