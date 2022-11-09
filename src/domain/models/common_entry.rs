use serde::{Deserialize, Serialize};

/// A representation of the common fields that exist between entries from the compendium
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Hash, Default, Eq)]
pub(crate) struct CommonEntry {
    id: i32,
    name: String,
    description: String,
    common_locations: Option<Vec<String>>,
    image: String,
}

impl CommonEntry {
    /// Get the entry's id
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get the entry's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the entry's description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the entry's common locations
    pub fn common_locations(&self) -> Option<&Vec<String>> {
        self.common_locations.as_ref()
    }

    /// Get the entry's image
    pub fn image(&self) -> &str {
        &self.image
    }
}
