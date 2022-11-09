use super::CommonEntry;
use serde::{Deserialize, Serialize};

/// A representation of a treasure entry from the compendium
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TreasureEntry {
    #[serde(flatten)]
    common_fields: CommonEntry,
    drops: Option<Vec<String>>,
    #[serde(default = "default_treasure_category_type")]
    category_type: String,
}

impl TreasureEntry {
    /// Get the entry's id
    pub fn id(&self) -> i32 {
        self.common_fields.id()
    }

    /// Get the entry's name
    pub fn name(&self) -> &str {
        self.common_fields.name()
    }

    /// Get the entry's description
    pub fn description(&self) -> &str {
        self.common_fields.description()
    }

    /// Get the entry's common locations
    pub fn common_locations(&self) -> Option<&Vec<String>> {
        self.common_fields.common_locations()
    }

    /// Get the entry's image
    pub fn image(&self) -> &str {
        self.common_fields.image()
    }

    /// Get the entry's drops
    pub fn drops(&self) -> Option<&Vec<String>> {
        self.drops.as_ref()
    }

    /// Get the entry's category type
    pub fn category_type(&self) -> &str {
        self.category_type.as_str()
    }
}

fn default_treasure_category_type() -> String {
    String::from("treasure")
}
