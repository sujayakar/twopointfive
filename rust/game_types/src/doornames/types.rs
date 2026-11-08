//! The types of `doornames`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/doornames.ts:75 nameDoors(doors), src/game/doornames.ts:75)
#[derive(Clone, Debug)]
pub struct NameDoorsDoorsItem {
    pub def: crate::doornames::NameDoorsDoorsItemDef,
}
impl NameDoorsDoorsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NameDoorsDoorsItem {
            def: crate::doornames::NameDoorsDoorsItemDef::zeroed(),
        }
    }
}
impl Default for NameDoorsDoorsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doornames.ts:75 NameDoorsDoorsItem.def, src/game/doornames.ts:75)
#[derive(Clone, Debug)]
pub struct NameDoorsDoorsItemDef {
    pub name: String,
}
impl NameDoorsDoorsItemDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NameDoorsDoorsItemDef {
            name: String::new(),
        }
    }
}
impl Default for NameDoorsDoorsItemDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doornames.ts:85 doorWrittenName(d), src/game/doornames.ts:85 doorWrittenName(d))
#[derive(Clone, Copy, Debug)]
pub struct DoorWrittenNameD {
}
impl DoorWrittenNameD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorWrittenNameD {
        }
    }
}
impl Default for DoorWrittenNameD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doornames.ts:87 doorSaidName(d), src/game/doornames.ts:87 doorSaidName(d))
#[derive(Clone, Copy, Debug)]
pub struct DoorSaidNameD {
}
impl DoorSaidNameD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorSaidNameD {
        }
    }
}
impl Default for DoorSaidNameD { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct DoornamesMod {
    /// (src/game/doornames.ts:24 SAID)
    pub said: JsMap<DoorId, String>,
    /// (src/game/doornames.ts:28 WRITTEN)
    pub written: JsMap<DoorId, String>,
    /// (src/game/doornames.ts:28 SAID_UP)
    pub said_up: JsMap<DoorId, String>,
}
impl DoornamesMod {
    pub fn blank() -> Self {
        DoornamesMod {
            said: JsMap::new(),
            written: JsMap::new(),
            said_up: JsMap::new(),
        }
    }
}

/// (src/game/doornames.ts:40 BEARING)
pub const BEARING: &str = "/\\b(?:north|south|east|west)(?:ern)?\\b/";

/// (src/game/doornames.ts:42 ORDINALS: a table nothing writes)
pub fn ordinals() -> Vec<String> { alloc::vec![String::from("first"), String::from("second"), String::from("third"), String::from("fourth"), String::from("fifth")] }
