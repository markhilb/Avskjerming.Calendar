#![deny(warnings)]
#![deny(rust_2018_idioms)]

mod error;
mod models;
mod adapter;
mod queries;
mod test_db;

pub use models::*;
pub use adapter::{PostgresAdapter, PsqlSettings};
pub use queries::EventsQuery;
pub use test_db::TestDb;
