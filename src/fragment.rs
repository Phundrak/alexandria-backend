use std::cmp::Ordering;

use diesel::expression_methods::ExpressionMethods;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use rayon::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::bookfragments::{self, dsl};
use crate::{models::Bookfragment, utils::ApiResult};

#[derive(Serialize, Deserialize, Copy, Clone, Eq)]
#[serde(crate = "rocket::serde")]
pub struct Simple {
    pub uuid: Uuid,
    pub rank: i32,
}

impl Ord for Simple {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Simple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Simple {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

/// List all fragments of a book
///
/// In order to preserve bandwidth, only the UUID and the rank of all
/// fragments are returned. If the user wishes to get a full fragment,
/// they can instead use the function `get`.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `list`
pub fn list(
    connector: &mut PgConnection,
    book_id: Uuid,
) -> ApiResult<Vec<Simple>> {
    let mut list = dsl::bookfragments
        .filter(dsl::book.eq(book_id))
        .load::<Bookfragment>(connector)?
        .par_iter()
        .map(|f| Simple {
            uuid: f.id,
            rank: f.rank,
        })
        .collect::<Vec<Simple>>();
    list.sort();
    Ok(list)
}

/// Return a full fragment
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `get`
pub fn get(connector: &mut PgConnection, id: Uuid) -> ApiResult<Bookfragment> {
    dsl::bookfragments.find(id).first(connector)
}

/// Shift fragments in a book
///
/// Shift all fragments by a set amount in a book starting from and to
/// a set rank. The rank specified by the value of `from` is included
/// while `to` is excluded.
///
/// Two values are optional:
/// - If `None` is passed as the value of `to`, treat all fragments
///   from `from` until the end of the book to be shifted.
/// - If `None` is passed to `shift`, treat it as a positive shift of
///   one rank only.
///
/// # Errors
///
/// Any error returned by diesel will be forwarded to the caller of
/// `shift_fragments`
pub fn shift_fragments(
    connector: &mut PgConnection,
    book: Uuid,
    from: i32,
    to: Option<i32>,
    shift: Option<i32>,
) -> ApiResult<usize> {
    diesel::update(dsl::bookfragments)
        .filter(dsl::book.eq(book))
        .filter(dsl::rank.ge(from))
        .filter(dsl::rank.lt(to.unwrap_or(i32::MAX)))
        .set(dsl::rank.eq(dsl::rank + shift.unwrap_or(1)))
        .execute(connector)
}

/// Move a fragment inside a book
///
/// Move an existing fragment to a new set rank, moving all the
/// fragments between its current rank and its new rank if needed. If
/// the new rank exceeds the amount of ranks existing in a book, the
/// fragment’s new rank will simply be set to the last rank available
/// --- i.e. if a fragment is moved to the rank 999 but the last
/// existing rank is 41 after fragment shifts, it will be moved to
/// rank 42.
///
/// # Errors
///
/// Any error returned by diesel will be forwarded to the callor of
/// `move_fragment`
///
/// # Returns
///
/// Returns either a `usize` indicating how many fragments were
/// shifted or a diesel [`Error`].
///
/// [`Error`]: ../../diesel/result/enum.Error.html
pub fn move_frag_id(
    connector: &mut PgConnection,
    fragment: Uuid,
    to: i32,
) -> ApiResult<usize> {
    let full_fragment: Bookfragment = bookfragments::dsl::bookfragments
        .find(fragment)
        .first(connector)?;
    let shift_direction = if full_fragment.rank < to { -1 } else { 1 };
    let moved_fragments = shift_fragments(
        connector,
        fragment,
        full_fragment.rank,
        Some(to),
        Some(shift_direction),
    )?;
    diesel::update(bookfragments::dsl::bookfragments)
        .filter(bookfragments::dsl::id.eq(fragment))
        .set(bookfragments::dsl::rank.eq(to))
        .execute(connector)?;
    Ok(moved_fragments)
}

/// Create a new fragment
///
/// If a fragment already exists in the same book at the same rank as
/// the new rank, treat this as an insert meant to move all the other
/// fragments.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `new`
pub fn new(
    connector: &mut PgConnection,
    fragment: Bookfragment,
) -> ApiResult<usize> {
    let book_fragments = bookfragments::dsl::bookfragments
        .filter(bookfragments::dsl::book.eq(fragment.book))
        .load::<Bookfragment>(connector)?;
    // Check if there is already a fragment with the same rank.
    // If yes, shift the existing fragments in order to place the new
    // fragment
    if !book_fragments.is_empty() {
        shift_fragments(connector, fragment.book, fragment.rank, None, None)?;
    }
    diesel::insert_into(dsl::bookfragments)
        .values(fragment)
        .execute(connector)
}

/// Update a fragment
///
/// As with new fragments, if the current fragment’s rank has changed,
/// shift the necessary fragments.
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `delete`
pub fn update(connector: &mut PgConnection, fragment: Bookfragment) -> ApiResult<usize> {
    let book_fragments = dsl::bookfragments
        .filter(dsl::rank.eq(fragment.rank))
        .filter(dsl::id.ne(fragment.id))
        .load::<Bookfragment>(connector)?;
    // Move the fragment before updating it
    // TODO: Check if the fragment exists first
    if book_fragments.len() > 1 {
        move_frag_id(connector, fragment.id, fragment.rank)?;
    };
    diesel::update(dsl::bookfragments)
        .set(fragment)
        .execute(connector)
}

/// Delete a book fragment
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `delete`
pub fn delete(connector: &mut PgConnection, id: Uuid) -> ApiResult<()> {
    match diesel::delete(dsl::bookfragments.find(id)).execute(connector) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
