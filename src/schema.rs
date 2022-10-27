// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "booktype"))]
    pub struct Booktype;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "imagetype"))]
    pub struct Imagetype;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "soundtype"))]
    pub struct Soundtype;
}

diesel::table! {
    authors (id) {
        id -> Uuid,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        penname -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Soundtype;
    use super::sql_types::Imagetype;

    bookfragments (id) {
        id -> Uuid,
        content -> Text,
        oneshotsoundsource -> Nullable<Varchar>,
        bgsoundtype -> Soundtype,
        bgsoundsource -> Nullable<Varchar>,
        imgtype -> Imagetype,
        imgsource -> Nullable<Varchar>,
        book -> Uuid,
        chapter -> Int4,
        rank -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Booktype;

    books (id) {
        id -> Uuid,
        title -> Varchar,
        author -> Uuid,
        isbn -> Nullable<Array<Nullable<Text>>>,
        cover -> Nullable<Varchar>,
        publisher -> Nullable<Varchar>,
        published -> Nullable<Date>,
        genre -> Nullable<Array<Nullable<Text>>>,
        synopsis -> Nullable<Text>,
        booktype -> Booktype,
    }
}

diesel::joinable!(bookfragments -> books (book));
diesel::joinable!(books -> authors (author));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    bookfragments,
    books,
);
