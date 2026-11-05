//! The types of `doornames`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DoornamesMod {
    /// (src/game/doornames.ts:24 SAID)
    pub said: JsMap<DoorId, String>,
    /// (src/game/doornames.ts:28 WRITTEN)
    pub written: JsMap<DoorId, String>,
    /// (src/game/doornames.ts:28 SAID_UP)
    pub said_up: JsMap<DoorId, String>,
    /// (src/game/doornames.ts:29 COMPASS)
    pub compass: JsSet<String>,
    /// (src/game/doornames.ts:32 JOIN)
    pub join: JsSet<String>,
    /// (src/game/doornames.ts:34 ODD)
    pub odd: JsSet<String>,
}
impl DoornamesMod {
    pub fn blank() -> Self {
        DoornamesMod {
            said: JsMap::new(),
            written: JsMap::new(),
            said_up: JsMap::new(),
            compass: { let mut s = JsSet::new(); for w in ["n", "s", "e", "w", "ne", "nw", "se", "sw", "north", "south", "east", "west"] { s.add(String::from(w)); } s },
            join: { let mut s = JsSet::new(); for w in ["to"] { s.add(String::from(w)); } s },
            odd: { let mut s = JsSet::new(); for w in ["off", "his"] { s.add(String::from(w)); } s },
        }
    }
}

/// (src/game/doornames.ts:40 BEARING)
pub const BEARING: &str = "/\\b(?:north|south|east|west)(?:ern)?\\b/";

/// (src/game/doornames.ts:42 ORDINALS: a table nothing writes)
pub fn ordinals() -> Vec<String> { alloc::vec![String::from("first"), String::from("second"), String::from("third"), String::from("fourth"), String::from("fifth")] }
