#![allow(unused_variables)]

#[macro_use]
pub(crate) mod macros;
pub(crate) mod utils;

pub mod api;
pub mod data;
pub mod error;
pub mod queries;
pub(crate) mod tests;
pub mod traits;

pub use api::Api;
