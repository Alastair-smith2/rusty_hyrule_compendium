/// An enum representing the ways of requesting an entry
#[derive(Debug, Clone, Copy)]
pub enum EntryIdentifier<'a> {
    /// The entry's id (e.g. 1 for horse) in the compendium
    Id(i32),
    /// The entry's name (e.g. silver moblin) in the compendium
    Name(&'a str),
}

/// An enum representing all the compendium category types
#[derive(Debug, Clone, Copy)]
pub enum CompendiumCategory {
    /// The treasure category in the compendium
    Treasure,
    /// The creature category in the compendium
    Creature,
    /// The monster category in the compendium
    Monster,
    /// The material category in the compendium
    Material,
    /// The equipment category in the compendium
    Equipment,
}

/// An enum representing the two game modes available in botw, standard and master mode
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum GameMode {
    /// Standard mode
    Standard,
    /// Master mode
    MasterMode,
}
