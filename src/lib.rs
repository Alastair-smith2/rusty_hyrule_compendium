//! This crate is a Rust library for the [Hryule Compendium API](https://github.com/gadhagod/Hyrule-Compendium-API)
//!
//! The resources provided by the above API as of version two and this create exposes a client `CompendiumClient` that has convienent methods to fetch associated data:
//!
//! ```rust
//! use rusty_hyrule_compendium::blocking::CompendiumClient;
//! use rusty_hyrule_compendium::domain::inputs::EntryIdentifier;
//!
//! // Preconfigured client using v2 of the API
//! let client = CompendiumClient::default();
//! /// Requests can fail for a number of reasons, see the error module for available errors
//! let monster_entry = client.monster(EntryIdentifier::Id(123))?;
//! // "white-maned lynel"
//! let monster_name = monster.name();
//! // "https://botw-compendium.herokuapp.com/api/v2/entry/white-maned_lynel/image"
//! let image = monster.image();
//! ```
//!
#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

pub mod blocking;
pub mod domain;
mod error;
mod result;

pub use error::CompendiumError;
pub use result::Result;
