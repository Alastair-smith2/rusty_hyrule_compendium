# Rusty Hyrule Compendium

A library for consuming the [Hyrule Compendium API](https://gadhagod.github.io/Hyrule-Compendium-API/#/) in Rust

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/Alastair-smith2/rusty_hyrule_compendium/blob/main/LICENSE

## Overview

This library exposes a client that can be used to request information from the API

### Example

**Currently you'll need to clone this repository and build it yourself**

In future it's hoped that you'll be able to install as a crate below in your `Cargo.toml`

```toml
[dependencies]
rusty_hyrule_compendium = "0.1.0"
```

To use this library, you'll need to instantiate the Compendium client. `CompendiumClient::default();` preconfigures the underlying HTTP client and API url with sensible values.

```rust
use rusty_hyrule_compendium::blocking::CompendiumClient;
use rusty_hyrule_compendium::domain::inputs::EntryIdentifier;

// Preconfigured client using v2 of the API
let client = CompendiumClient::default();
// Requests can fail for a number of reasons, see the error module for available errors
let monster_entry = client.monster(EntryIdentifier::Id(123))?;
// "white-maned lynel"
let monster_name = monster.name();
// "https://botw-compendium.herokuapp.com/api/v2/entry/white-maned_lynel/image"
let image = monster.image();
```

Each of the resources (see below for comprehensive list) have a [struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) representation with helper methods to the underlying data (e.g. `.name()`, `.image()` etc)

[Here](https://gadhagod.github.io/Hyrule-Compendium-API/#/?id=concept) contains the raw JSON response for the example

Furthermore it's possbile to request all of the above by category but pattern matching is required to get the entries.

```rust
let result = client.category(CompendiumCategory::Treasure)?;
match result {
            CategoryResult::Treasure(treasure) => {
                // Do something with the Vec<TreasureEntry>
            }
            _ => panic!("Unexpected result while search for treasure category"),
        }
```

It's also possible to get all entries by the `.all_entries()` method

E.g.

```rust
let all_entries = client.all_entries()?;
// Get all creature entries that are food specific, &Vec<CreatureEntry> type
let food_creatures = all_entries.creatures().food();
```

### Available resources

- Monsters (standard and master mode ones), `.monster()`
- Creatures `.creature()`
- Equipment
- Materials
- Treasure

These can be searched by name (e.g. `EntryIdentifier::Name("Moblin"))` or id (e.g. `EntryIdentifier::Id(1)`)
