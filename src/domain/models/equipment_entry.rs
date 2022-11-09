use super::CommonEntry;
use serde::{Deserialize, Serialize};

/// A representation of an equipment entry from the compendium
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquipmentEntry {
    #[serde(flatten)]
    common_fields: CommonEntry,
    attack: Option<i32>,
    defense: Option<i32>,
    #[serde(default = "default_equipment_category_type")]
    category_type: String,
}

impl EquipmentEntry {
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

    /// Get the entry's attack
    pub fn attack(&self) -> Option<i32> {
        self.attack
    }

    /// Get the entry's defense
    pub fn defense(&self) -> Option<i32> {
        self.defense
    }

    /// Get the entry's category type
    pub fn category_type(&self) -> &str {
        self.category_type.as_str()
    }
}

fn default_equipment_category_type() -> String {
    String::from("equipment")
}
