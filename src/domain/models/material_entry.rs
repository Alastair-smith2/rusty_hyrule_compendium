use super::CommonEntry;
use serde::{Deserialize, Serialize};

/// A representation of a monster entry from the compendium
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterialEntry {
    #[serde(flatten)]
    common_fields: CommonEntry,
    hearts_recovered: Option<f32>,
    #[serde(default = "default_material_category_type")]
    category_type: String,
}

impl MaterialEntry {
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

    /// Get the entry's hearts recovered
    pub fn hearts_recovered(&self) -> Option<f32> {
        self.hearts_recovered
    }

    /// Get the entry's category type
    pub fn category_type(&self) -> &str {
        self.category_type.as_str()
    }
}

fn default_material_category_type() -> String {
    String::from("materials")
}
