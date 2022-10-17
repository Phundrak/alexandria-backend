use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Author {
    pub slug: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub pen_name: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub isbn: Option<Vec<Option<String>>>,
    pub cover: Option<String>,
    pub publisher: Option<String>,
    pub published: Option<String>,
    pub genre: Option<Vec<Option<String>>>,
    pub synopsis: Option<String>,
    pub book_type: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BookFragment {
    pub id: String,
    pub content: String,
    pub one_shot_sound_source: Option<String>,
    pub bg_sound_type: Option<String>,
    pub bg_sound_source: Option<String>,
    pub img_type: Option<String>,
    pub img_source: Option<String>,
    pub book: String,
    pub chapter: i32,
    pub rank: i32,
}
