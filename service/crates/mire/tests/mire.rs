#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::module_name_repetitions, clippy::wildcard_imports)
)]

mod tests;
mod testsubject;

pub use testsubject::TestSubject;
