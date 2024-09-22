//! # DataStore Module
//!
//! This module provides functionality for interacting with a SQLite database
//! to store and retrieve information about YouTube streams and their closed captions.
//!
//! The module uses sqlx for database operations and provides an abstraction layer
//! for CRUD operations on streams and their associated closed captions.

mod domain;
mod store;

pub use domain::Stream;
pub use store::DataStore;
