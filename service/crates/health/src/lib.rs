#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::module_name_repetitions, clippy::wildcard_imports)
)]

pub mod config;
mod endpoints;
mod healthchecker;
mod model;
mod service;

pub use healthchecker::Healthchecker;
pub use model::*;
pub use service::HealthService;
