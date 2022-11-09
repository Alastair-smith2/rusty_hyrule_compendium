use crate::domain::models::{
    CreatureEntry, EquipmentEntry, MaterialEntry, MonsterEntry, TreasureEntry,
};
use serde::{Deserialize, Serialize};

/// A representation of all entries from the compendium
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllStandardEntries {
    creatures: AllCreatureEntries,
    equipment: Vec<EquipmentEntry>,
    materials: Vec<MaterialEntry>,
    monsters: Vec<MonsterEntry>,
    treasure: Vec<TreasureEntry>,
}

impl AllStandardEntries {
    /// A reference to the creature entries from the compendium
    pub fn creatures(&self) -> &AllCreatureEntries {
        &self.creatures
    }

    /// A reference to the equipment entries from the compendium
    pub fn equipment(&self) -> &Vec<EquipmentEntry> {
        &self.equipment
    }

    /// A reference to the material entries from the compendium
    pub fn materials(&self) -> &Vec<MaterialEntry> {
        &self.materials
    }

    /// A reference to the monster entries from the compendium
    pub fn monsters(&self) -> &Vec<MonsterEntry> {
        &self.monsters
    }

    /// A reference to the treasure entries from the compendium
    pub fn treasure(&self) -> &Vec<TreasureEntry> {
        &self.treasure
    }

    /// A mutable reference to the creature entries from the compendium
    pub fn creatures_mut(&mut self) -> &mut AllCreatureEntries {
        &mut self.creatures
    }

    /// A mutable reference to the equipment entries from the compendium
    pub fn equipment_mut(&mut self) -> &mut Vec<EquipmentEntry> {
        &mut self.equipment
    }

    /// A mutable reference to the materials entries from the compendium
    pub fn materials_mut(&mut self) -> &mut Vec<MaterialEntry> {
        &mut self.materials
    }

    /// A mutable reference to the monster entries from the compendium
    pub fn monsters_mut(&mut self) -> &mut Vec<MonsterEntry> {
        &mut self.monsters
    }

    /// A mutable reference to the treasure entries from the compendium
    pub fn treasure_mut(&mut self) -> &mut Vec<TreasureEntry> {
        &mut self.treasure
    }
}

/// A representation of possible responses from the compendium API.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "category")]
pub enum EntryResponse {
    /// The entry obtained was of the monster category
    #[serde(rename = "monsters")]
    Monster(MonsterEntry),
    /// The entry obtained was of the creature category
    #[serde(rename = "creatures")]
    Creature(CreatureEntry),
    /// The entry obtained was of the equipment category
    #[serde(rename = "equipment")]
    Equipment(EquipmentEntry),
    /// The entry obtained was of the treasure category
    #[serde(rename = "treasure")]
    Treasure(TreasureEntry),
    /// The entry obtained was of the material category
    #[serde(rename = "materials")]
    Material(MaterialEntry),
}

/// A representation of all creatures that can be returned from the compendium API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllCreatureEntries {
    food: Vec<CreatureEntry>,
    non_food: Vec<CreatureEntry>,
}

impl AllCreatureEntries {
    /// Get a reference to the creature food entries
    pub fn food(&self) -> &Vec<CreatureEntry> {
        &self.food
    }

    /// Get a reference to the creature non-food entries
    pub fn non_food(&self) -> &Vec<CreatureEntry> {
        &self.non_food
    }

    /// Get a mutable reference to the creature food entries
    pub fn food_mut(&mut self) -> &mut Vec<CreatureEntry> {
        &mut self.food
    }

    /// Get a mutable reference to the creature non-food entries
    pub fn non_food_mut(&mut self) -> &mut Vec<CreatureEntry> {
        &mut self.non_food
    }
}

/// An enum containing the possible responses while retrieving a category.
/// In the case where the search is for the treasure category, it should be expected that the treasure variant of this enum will be returned.
#[derive(Debug, Clone)]
pub enum CategoryResult {
    /// All entries from the treasure category
    Treasure(Vec<TreasureEntry>),
    /// All entries from the creature category
    Creatures(AllCreatureEntries),
    /// All entries from the monster category
    Monsters(Vec<MonsterEntry>),
    /// All entries from the material category
    Materials(Vec<MaterialEntry>),
    /// All entries from the equipment category
    Equipment(Vec<EquipmentEntry>),
}
