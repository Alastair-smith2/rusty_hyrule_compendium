# Rusty Hyrule Compendium

A library for consuming the [Hyrule Compendium API](https://gadhagod.github.io/Hyrule-Compendium-API/#/) in Rust

[![MIT licensed][mit-badge]][mit-url]

[mit-url]: https://github.com/Alastair-Smith2/rusty_hyrule_compendium/LICENSE

## Overview

This library exposes a client that can be used to request information from the API

### Example

**Currently you'll need to clone this repository and build it yourself**

In future it's hoped that you'll be able to install as a crate below

```toml
[dependencies]
rusty_hyrule_compendium = 0.1.0
```

E.g.

```rust
use rusty_hyrule_compendium::blocking::CompendiumClient;
use rusty_hyrule_compendium::domain::inputs::EntryIdentifier;

// Preconfigured client using v2 of the API
let client = CompendiumClient::default();
/// Requests can fail for a number of reasons, see the error module for available errors
let monster_entry = client.monster(EntryIdentifier::Id(123))?;
// "white-maned lynel"
let monster_name = monster.name();
// "https://botw-compendium.herokuapp.com/api/v2/entry/white-maned_lynel/image"
let image = monster.image();
```

### Available resources

- Monsters (standard and master mode ones)
- Creatures
- Equipment
- Materials
- Treasure

These can be searched by name or id, or requested
