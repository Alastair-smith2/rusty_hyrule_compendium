use crate::domain::inputs::{CompendiumCategory, EntryIdentifier, GameMode};
use crate::domain::models::{
    CreatureEntry, EquipmentEntry, MaterialEntry, MonsterEntry, TreasureEntry,
};
use crate::domain::responses::{AllStandardEntries, CategoryResult, EntryResponse};
use crate::error::CompendiumError;
use crate::result::Result;
use reqwest::{
    blocking::{Client, Response},
    Url,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    data: T,
}

/// Sealing the trait not to be used by other consumers
pub trait CompendiumSealed {}

/// The trait that any CommpendiumClient must implement
pub trait CompendiumApiClient: CompendiumSealed {
    /// Get an entry (see [EntryResponse](crate::domain::responses::EntryResponse) for exact types that can be returned) by [identifier](crate::domain::inputs::EntryIdentifier)
    /// ```rust
    /// use rusty_hyrule_compendium::blocking::{CompendiumApiClient, CompendiumClient};
    /// use rusty_hyrule_compendium::domain::inputs::EntryIdentifier;
    /// use rusty_hyrule_compendium::domain::responses::EntryResponse;
    /// use rusty_hyrule_compendium::Result;
    ///
    /// fn main() -> Result<()> {
    ///     // Preconfigured client using v2 of the API
    ///     let client = CompendiumClient::default();
    ///     // Requests can fail for a number of reasons, see the error module for available errors
    ///     let entry = client.entry(EntryIdentifier::Id(1))?;
    ///     // "white-maned lynel"
    ///      match entry {
    ///         EntryResponse::Creature(creature) => {
    ///           // "Horse"
    ///          let name = creature.name();
    ///          }
    ///          _ => { /* Handle other EntryResponse types as desired */}
    ///     }
    ///     Ok(())
    /// }
    /// ```
    fn entry(&self, identifier: EntryIdentifier) -> Result<EntryResponse>;
    /// Get a [monster entry](crate::domain::models::MonsterEntry) by [identifier](crate::domain::inputs::EntryIdentifier)
    /// ```rust
    /// use rusty_hyrule_compendium::blocking::{CompendiumApiClient, CompendiumClient};
    /// use rusty_hyrule_compendium::domain::inputs::EntryIdentifier;
    /// use rusty_hyrule_compendium::Result;

    /// fn main() -> Result<()> {
    ///     // Preconfigured client using v2 of the API
    ///     let client = CompendiumClient::default();
    ///     // Requests can fail for a number of reasons, see the error module for available errors
    ///     let monster_entry = client.monster(EntryIdentifier::Id(123))?;
    ///     // "white-maned lynel"
    ///     let monster_name = monster_entry.name();
    ///     // "https://botw-compendium.herokuapp.com/api/v2/entry/white-maned_lynel/image"
    ///     let monster_image = monster_entry.image();
    ///     Ok(())
    /// }
    /// ```
    fn monster(&self, identifier: EntryIdentifier) -> Result<MonsterEntry>;
    /// Get specifically a [monster entry](crate::domain::models::MonsterEntry) that exists only in master mode by [identifier](crate::domain::inputs::EntryIdentifier)
    fn master_mode_monster(&self, identifier: EntryIdentifier) -> Result<MonsterEntry>;
    /// Get specifically a [treasure entry](crate::domain::models::TreasureEntry) by [identifier](crate::domain::inputs::EntryIdentifier)
    fn treasure(&self, identifier: EntryIdentifier) -> Result<TreasureEntry>;
    /// Get specifically a [creature entry](crate::domain::models::CreatureEntry) by [identifier](crate::domain::inputs::EntryIdentifier)
    fn creature(&self, identifier: EntryIdentifier) -> Result<CreatureEntry>;
    /// Get specifically a [material entry](crate::domain::models::MaterialEntry) by [identifier](crate::domain::inputs::EntryIdentifier)
    fn material(&self, identifier: EntryIdentifier) -> Result<MaterialEntry>;
    /// Get specifically an [equipment entry](crate::domain::models::EquipmentEntry) by [identifier](crate::domain::inputs::EntryIdentifier)
    fn equipment(&self, identifier: EntryIdentifier) -> Result<EquipmentEntry>;
    /// Get all entries for a given a category
    /// ```rust
    /// use rusty_hyrule_compendium::blocking::{CompendiumApiClient, CompendiumClient};
    /// use rusty_hyrule_compendium::domain::inputs::CompendiumCategory;
    /// use rusty_hyrule_compendium::domain::responses::CategoryResult;
    /// use rusty_hyrule_compendium::Result;

    /// fn main() -> Result<()> {
    ///     // Preconfigured client using v2 of the API
    ///     let client = CompendiumClient::default();
    ///     let result = client.category(CompendiumCategory::Monster)?;
    ///     match result {
    ///         CategoryResult::Monsters(monsters) => {
    ///             // monsters
    ///         }
    ///         _ => { /* Return some form of error, unexpected scenario */}
    ///     }
    ///     Ok(())
    /// }
    /// ```
    fn category(&self, category: CompendiumCategory) -> Result<CategoryResult>;
    /// Get all entries in the compendium (excluding master mode)
    /// ```rust
    /// use rusty_hyrule_compendium::blocking::{CompendiumApiClient, CompendiumClient};
    /// use rusty_hyrule_compendium::domain::inputs::CompendiumCategory;
    /// use rusty_hyrule_compendium::domain::responses::CategoryResult;
    /// use rusty_hyrule_compendium::Result;
    ///
    /// fn main() -> Result<()> {
    ///     // Preconfigured client using v2 of the API
    ///     let client = CompendiumClient::default();
    ///     let all_entries = client.all_entries()?;
    ///     /// // &Vec<CreatureEntry> that are food specific
    ///     let food_creatures = all_entries.creatures().food();
    ///     Ok(())
    /// }
    /// ```
    fn all_entries(&self) -> Result<AllStandardEntries>;
    /// Get all [master mode entries](crate::domain::models::MonsterEntry) (which are only monsters) in the compendium
    fn all_master_mode_entries(&self) -> Result<Vec<MonsterEntry>>;
}

/// The CompendiumClient that can be used to obtain relevant entries
#[derive(Debug, Clone)]
pub struct CompendiumClient {
    base_url: Url,
    network_client: Client,
}

impl Default for CompendiumClient {
    fn default() -> CompendiumClient {
        CompendiumClient {
            base_url: Url::parse("https://botw-compendium.herokuapp.com/api/v2/").unwrap(),
            network_client: Client::new(),
        }
    }
}

impl CompendiumClient {
    /// A convience method to initialise a compendium client if the CompendiumClient::Default() isn't sufficient
    pub fn new(url: &str) -> Result<Self> {
        Ok(CompendiumClient {
            base_url: Url::parse(url)
                .map_err(|_e| CompendiumError::InvalidBaseUrl(url.to_string()))?,
            network_client: Client::new(),
        })
    }

    fn create_path<S: Into<String>>(&self, url: &Url, path_to_add: S) -> Result<Url> {
        url.join(path_to_add.into().as_str())
            .map_err(|_e| CompendiumError::ErrorConstructingResourceUrl)
    }

    fn create_path_for_entry(&self, identifier: EntryIdentifier, mode: GameMode) -> Result<Url> {
        let entry_identifier = match identifier {
            EntryIdentifier::Id(id) => id.to_string(),
            EntryIdentifier::Name(name) => name.replace(' ', "_"),
        };
        if mode == GameMode::MasterMode {
            return self.create_path(
                &self.base_url,
                format!("master_mode/entry/{}", entry_identifier),
            );
        }
        self.create_path(&self.base_url, format!("entry/{}", entry_identifier))
    }

    fn make_request(&self, url: Url) -> Result<Response> {
        self.network_client
            .get(url)
            .send()
            .map_err(CompendiumError::RequestError)
            .and_then(handle_response)
    }

    fn fetch_data_for_specified_type<T>(&self, url: Url) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.make_request(url)?;
        response
            .json::<ApiResponse<T>>()
            .map(|api_response| api_response.data)
            .map_err(CompendiumError::ResponseParsingError)
    }

    fn category_path_for_type(&self, category: &CompendiumCategory) -> &str {
        match category {
            CompendiumCategory::Creature => "creatures",
            CompendiumCategory::Monster => "monsters",
            CompendiumCategory::Material => "materials",
            CompendiumCategory::Treasure => "treasure",
            CompendiumCategory::Equipment => "equipment",
        }
    }

    fn fetch_data_for_specific_category(
        &self,
        url: Url,
        entry_type: CompendiumCategory,
    ) -> Result<CategoryResult> {
        match entry_type {
            CompendiumCategory::Monster => Ok(CategoryResult::Monsters(
                self.fetch_data_for_specified_type(url)?,
            )),
            CompendiumCategory::Material => Ok(CategoryResult::Materials(
                self.fetch_data_for_specified_type(url)?,
            )),
            CompendiumCategory::Treasure => Ok(CategoryResult::Treasure(
                self.fetch_data_for_specified_type(url)?,
            )),
            CompendiumCategory::Creature => Ok(CategoryResult::Creatures(
                self.fetch_data_for_specified_type(url)?,
            )),
            CompendiumCategory::Equipment => Ok(CategoryResult::Equipment(
                self.fetch_data_for_specified_type(url)?,
            )),
        }
    }

    fn fetch_data_for_specified_entry<T>(
        &self,
        identifier: EntryIdentifier,
        game_mode: GameMode,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.create_path_for_entry(identifier, game_mode)?;
        self.fetch_data_for_specified_type(url)
    }
}

impl CompendiumApiClient for CompendiumClient {
    fn entry(&self, identifier: EntryIdentifier) -> Result<EntryResponse> {
        self.fetch_data_for_specified_entry(identifier, GameMode::Standard)
    }

    fn monster(&self, identifier: EntryIdentifier) -> Result<MonsterEntry> {
        self.fetch_data_for_specified_entry(identifier, GameMode::Standard)
    }

    fn master_mode_monster(&self, identifier: EntryIdentifier) -> Result<MonsterEntry> {
        self.fetch_data_for_specified_entry(identifier, GameMode::MasterMode)
    }

    fn treasure(&self, identifier: EntryIdentifier) -> Result<TreasureEntry> {
        self.fetch_data_for_specified_entry(identifier, GameMode::Standard)
    }

    fn creature(&self, identifier: EntryIdentifier) -> Result<CreatureEntry> {
        self.fetch_data_for_specified_entry(identifier, GameMode::Standard)
    }

    fn equipment(&self, identifier: EntryIdentifier) -> Result<EquipmentEntry> {
        self.fetch_data_for_specified_entry(identifier, GameMode::Standard)
    }

    fn material(&self, identifier: EntryIdentifier) -> Result<MaterialEntry> {
        self.fetch_data_for_specified_entry(identifier, GameMode::Standard)
    }

    fn category(&self, category: CompendiumCategory) -> Result<CategoryResult> {
        let category_url = self.create_path(
            &self.base_url,
            format!("category/{}", self.category_path_for_type(&category)),
        )?;
        self.fetch_data_for_specific_category(category_url, category)
    }

    fn all_entries(&self) -> Result<AllStandardEntries> {
        let all_normal_mode_entries_url = self.create_path(&self.base_url, "all")?;
        self.fetch_data_for_specified_type(all_normal_mode_entries_url)
    }

    fn all_master_mode_entries(&self) -> Result<Vec<MonsterEntry>> {
        let all_master_mode_entries_url = self.create_path(&self.base_url, "master_mode/all")?;
        self.fetch_data_for_specified_type(all_master_mode_entries_url)
    }
}

impl CompendiumSealed for CompendiumClient {}

fn handle_response(response_data: Response) -> Result<Response> {
    let status_code = response_data.status();
    // Would response_data.error_for_status() be better?
    if status_code.is_server_error() {
        return Err(CompendiumError::ServerError);
    }
    if status_code.is_client_error() {
        return Err(CompendiumError::NoDataFound(
            response_data.url().path().to_string(),
        ));
    }
    Ok(response_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url, Mock};

    fn silver_moblin_data<'a>() -> &'a str {
        r#"{"data":{"category":"monsters","common_locations":null,"description":"The strongest of all Moblins, Ganon's fiendish magic has allowed them to surpass even the Black Moblins in strength and resilience. They're called \"silver\" for both their body color as well as their rarity. The purple patterns on their bodies also help them to stand out.","drops":["moblin horn","moblin fang","moblin guts","amber","opal","topaz","ruby","sapphire","diamond"],"id":112,"image":"https://botw-compendium.herokuapp.com/api/v2/entry/silver_moblin/image","name":"silver moblin"}}"#
    }

    fn monster_category_data<'a>() -> &'a str {
        r#"{"data":[{"category":"monsters","common_locations":null,"description":"The strongest of all Moblins, Ganon's fiendish magic has allowed them to surpass even the Black Moblins in strength and resilience. They're called \"silver\" for both their body color as well as their rarity. The purple patterns on their bodies also help them to stand out.","drops":["moblin horn","moblin fang","moblin guts","amber","opal","topaz","ruby","sapphire","diamond"],"id":112,"image":"https://botw-compendium.herokuapp.com/api/v2/entry/silver_moblin/image","name":"silver moblin"}]}"#
    }

    fn winterwing_butterfly_data<'a>() -> &'a str {
        r#"{"data":{"category":"creatures","common_locations":["Hyrule Ridge","Tabantha Frontier"],"cooking_effect":"heat resistance","description":"The powdery scales of this butterfly's wings cool the air around it. Watching it flutter around snowflakes is a thing of beauty. Cook it with monster parts for a heat-resistant elixir.","hearts_recovered":0,"id":67,"image":"https://botw-compendium.herokuapp.com/api/v2/entry/winterwing_butterfly/image","name":"winterwing butterfly"}}"#
    }

    fn missing_data_response<'a>() -> &'a str {
        r#"{"data":{},"message":"no results"}"#
    }

    fn create_successful_mock(path: &str, mock_body_response: &str) -> Mock {
        mock("GET", path)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_body_response)
            .create()
    }

    fn create_missing_data_mock(path: &str) -> Mock {
        mock("GET", path)
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(missing_data_response())
            .create()
    }

    fn create_server_error_data_mock(path: &str, mock_body_response: &str) -> Mock {
        mock("GET", path)
            .with_status(500)
            .with_header("content-type", "application/json")
            .with_body(mock_body_response)
            .create()
    }

    fn create_compendium() -> CompendiumClient {
        CompendiumClient::new(server_url().as_str()).unwrap()
    }

    #[test]
    fn test_compendium_client_monster_entry_search() {
        let mock = create_successful_mock("/entry/silver_moblin", silver_moblin_data());
        let compendium = create_compendium();
        let identifier = EntryIdentifier::Name("silver_moblin");
        let result = compendium.entry(identifier).unwrap();
        match result {
            EntryResponse::Monster(monster) => {
                assert_eq!(112, monster.id());
                mock.assert()
            }
            _ => panic!("Unexpected result while search for silver mobiln"),
        }
    }

    #[test]
    fn test_compendium_client_creature_entry_search() {
        let mock =
            create_successful_mock("/entry/winterwing_butterfly", winterwing_butterfly_data());
        let compendium = create_compendium();
        let identifier = EntryIdentifier::Name("winterwing_butterfly");
        let result = compendium.entry(identifier).unwrap();
        match result {
            EntryResponse::Creature(creature) => {
                assert_eq!(67, creature.id());
                assert_eq!("winterwing butterfly", creature.name());
                mock.assert()
            }
            _ => panic!("Unexpected result while search for winter butterfly"),
        }
    }

    #[test]
    fn test_compendium_client_monster_category_search() {
        let mock = create_successful_mock("/category/monsters", monster_category_data());
        let compendium = create_compendium();
        let result = compendium.category(CompendiumCategory::Monster).unwrap();
        match result {
            CategoryResult::Monsters(monsters) => {
                assert_eq!(1, monsters.len());
                assert_eq!(112, monsters.first().unwrap().id());
                mock.assert()
            }
            _ => panic!("Unexpected result while search for monster category"),
        }
    }

    #[test]
    fn test_compendium_client_missing_monster_response() {
        let mock = create_missing_data_mock("/entry/example_monster");
        let compendium = create_compendium();
        let identifier = EntryIdentifier::Name("example_monster");
        assert!(compendium.entry(identifier).is_err());
        mock.assert()
    }

    #[test]
    fn test_compendium_client_internal_server_error_response() {
        let mock = create_server_error_data_mock("/entry/silver_moblin", silver_moblin_data());
        let compendium = create_compendium();
        let identifier = EntryIdentifier::Name("silver_moblin");
        assert!(compendium.entry(identifier).is_err());
        mock.assert()
    }

    #[test]
    fn test_compendium_client_unexpected_server_response() {
        let mock = create_successful_mock("/entry/silver_moblin", missing_data_response());
        let compendium = create_compendium();
        let identifier = EntryIdentifier::Name("silver_moblin");
        assert!(compendium.entry(identifier).is_err());
        mock.assert()
    }
}
