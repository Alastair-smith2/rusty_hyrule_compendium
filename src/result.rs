//! A utility result type for the compendium crate

use crate::error::CompendiumError;
/// The Compendium result type
pub type Result<T> = std::result::Result<T, CompendiumError>;
