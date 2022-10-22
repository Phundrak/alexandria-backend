use diesel::prelude::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

use crate::schema::{authors, books, bookfragments};

#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
pub struct Author {
    pub slug: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub penname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Booktype"]
#[serde(crate = "rocket::serde")]
pub enum BookType {
    Novel,
    ShortStory,
    Poem
}

#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub isbn: Option<Vec<Option<String>>>,
    pub cover: Option<String>,
    pub publisher: Option<String>,
    pub published: Option<chrono::NaiveDate>,
    pub genre: Option<Vec<Option<String>>>,
    pub synopsis: Option<String>,
    pub booktype: BookType,
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Imagetype"]
#[serde(crate = "rocket::serde")]
pub enum ImageType {
    None,
    Url,
    Auto,
    Same
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Soundtype"]
#[serde(crate = "rocket::serde")]
pub enum SoundType {
    None,
    Url,
    Same
}

#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
pub struct Bookfragment {
    pub id: String,
    pub content: String,
    pub oneshotsoundsource: Option<String>,
    pub bgsoundtype: SoundType,
    pub bgsoundsource: Option<String>,
    pub imgtype: ImageType,
    pub imgsource: Option<String>,
    pub book: String,
    pub chapter: i32,
    pub rank: i32,
}
