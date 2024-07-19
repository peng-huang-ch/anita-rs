//! High level error types for the application.
//!
//! ## Feature Flags
//!
//! - `test-utils`: Export utilities for testing

// #![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub use error::{SrvError, SrvErrorKind, SrvResult};

mod error;

#[cfg(feature = "actix")]
mod actix;
