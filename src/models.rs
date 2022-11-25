use diesel::{prelude::{Insertable, Queryable}, AsChangeset};
use rocket::serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::schema::{authors, bookfragments, books};

/// Rust representation of the `Autors` table in the database
///
/// It contains four elements:
/// - The identifier of the author
/// - Their first name (can include their middle name)
/// - Their last name
/// - Their pen name
/// All of them except the identifier can be null. However, the pen
/// name must be set if the first and last names aren’t, and vice
/// versa.
#[derive(Queryable, Deserialize, Serialize, Insertable, Clone, AsChangeset)]
#[serde(crate = "rocket::serde")]
pub struct Author {
    pub id: Uuid,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub penname: Option<String>,
}

/// Different types of books.
///
/// See [`Book`]
///
/// [`Book`]: ./struct.Book.html
#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum, Clone, PartialEq, Eq, Copy)]
#[DieselTypePath = "crate::schema::sql_types::Booktype"]
#[serde(crate = "rocket::serde")]
pub enum BookType {
    Novel,
    ShortStory,
    Poem,
}

/// Rust representation of the `Books` table in the database.
///
/// The table consists of ten elements:
/// - The unique identifier of the book
/// - The title of the book, including its subtitle
/// - The unique identifier of the author of the book
/// - A list of ISBNs of the book (can be null)
/// - A link to the cover of the book (can be null)
/// - The date the book was published (can be null)
/// - The synopsis of the book (can be null)
/// - The type of book it is (see [`BookType`])
///
/// [`BookType`]: ./enum.BookType.html
#[derive(Queryable, Deserialize, Serialize, Insertable, Clone, AsChangeset)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: Uuid,
    pub isbn: Option<Vec<Option<String>>>,
    pub cover: Option<String>,
    pub publisher: Option<String>,
    pub published: Option<chrono::NaiveDate>,
    pub genre: Option<Vec<Option<String>>>,
    pub synopsis: Option<String>,
    pub booktype: BookType,
}

/// The type of image used as the background for a fragment.
///
/// Four different types of images can be used for a book fragment
/// (see [`BookFragment`]):
/// - **None**: No background image to be used
/// - **Url**: A user-specified background should be used, the URL is
///   held in the `imgsource` element of [`BookFragment`].
/// - **Auto**: The background image
/// - **Same**: Keep using the previous image
///
/// [`BookFragment`]: ./struct.Bookfragment.html
#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum, Clone)]
#[DieselTypePath = "crate::schema::sql_types::Imagetype"]
#[serde(crate = "rocket::serde")]
pub enum ImageType {
    None,
    Url,
    Auto,
    Same,
}

/// The type of background sound used for a fragment
///
/// Three different types of background sounds can be used for a book
/// fragment (see [`BookFragment`])
/// - **None**: Don’t use any background sound
/// - **Url**: A user-specified background sound should be used, the
///   URL is held in the `bgsoundsource` element of [`BookFragment`]
/// - **Same**: Keep using the same background sound as the previous
///   fragment
///
/// [`BookFragment`]: ./struct.Bookfragment.html
#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum, Clone)]
#[DieselTypePath = "crate::schema::sql_types::Soundtype"]
#[serde(crate = "rocket::serde")]
pub enum SoundType {
    None,
    Url,
    Same,
}

/// Rust representation of the `BookFragments` table in the database.
///
/// The table consists of ten elements:
/// - Its unique identifier
/// - The text content of the fragment
/// - The source of the oneshot sound (can be null if none)
/// - The type of background sound (see [`SoundType`])
/// - The source of the background sound (can be null if none)
/// - The type of the background image (see [`ImageType`])
/// - The source of the background image (can be null if none). If
///   `imgtype` is of type `Url`, then it holds a direct link to an
///   online resource the frontend can directly grab. If the type of
///   `imgtype` is `Auto`, then `imgsource` contains the directions
///   for grabbing the necessary keywords in order to grab an image
///   automatically from Unsplash.
/// - Which book this fragment belongs to (references is unique id,
///   see [`Book`])
/// - The chapter it is in (1 is the first chapter)
/// - Its ranking within the chapter (1 is the first fragment of the
///   chapter)
///
/// [`ImageType`]: ./enum.ImageType.html
/// [`SoundType`]: ./enum.SoundType.html
/// [`Book`]: ./struct.Book.html
#[derive(Queryable, Deserialize, Serialize, Insertable, Clone, AsChangeset)]
#[serde(crate = "rocket::serde")]
pub struct Bookfragment {
    pub id: Uuid,
    pub content: String,
    pub oneshotsoundsource: Option<String>,
    pub bgsoundtype: SoundType,
    pub bgsoundsource: Option<String>,
    pub imgtype: ImageType,
    pub imgsource: Option<String>,
    pub book: Uuid,
    pub chapter: i32,
    pub rank: i32,
}
