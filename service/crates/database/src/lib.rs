#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::module_name_repetitions, clippy::wildcard_imports)
)]

mod health;
pub mod migrate;
mod postgres;

pub use postgres::Database;
