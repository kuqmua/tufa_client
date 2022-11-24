#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

pub mod components;
pub mod entry;
pub mod global_variables;
pub mod helpers;
pub mod routes;
pub mod store;
