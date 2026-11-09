//! The types of `doornames`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct NameDoorsDoorsItem {
    pub def: crate::doornames::NameDoorsDoorsItemDef,
}

#[derive(Clone, Debug, Default)]
pub struct NameDoorsDoorsItemDef {
    pub name: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorWrittenNameD {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorSaidNameD {
}

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
