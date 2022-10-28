use std::cmp::Ordering;

use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use rayon::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use diesel::expression_methods::ExpressionMethods;

use crate::schema::bookfragments::dsl::{book, bookfragments};
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
    match bookfragments
        .filter(book.eq(book_id))
        .load::<Bookfragment>(connector)
    {
        Ok(fragments) => {
            let mut list = fragments
                .par_iter()
                .map(|fragment| Simple {
                    uuid: fragment.id,
                    rank: fragment.rank,
                })
                .collect::<Vec<Simple>>();
            list.sort();
            Ok(list)
        }
        Err(e) => Err(e),
    }
}

/// Return a full fragment
///
/// # Errors
///
/// If an error is returned by diesel, forward it to the function
/// calling `get`
pub fn get(
    connector: &mut PgConnection,
    id: Uuid,
) -> ApiResult<Bookfragment> {
    bookfragments
        .find(id)
        .first(connector)
}
