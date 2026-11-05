//! The types of `scene_look`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SceneLookModSuitAsked {
    Quiet,
}
impl SceneLookModSuitAsked {
    pub const fn as_str(self) -> &'static str {
        match self {
            SceneLookModSuitAsked::Quiet => "quiet",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SuitAs {
    Quiet,
}
impl SuitAs {
    pub const fn as_str(self) -> &'static str {
        match self {
            SuitAs::Quiet => "quiet",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SceneLookMod {
    /// (src/scene/look.ts:24 suitAsked)
    pub suit_asked: Option<crate::scene_look::SceneLookModSuitAsked>,
    /// (src/scene/look.ts:35 lookPlace)
    pub look_place: bool,
}
impl SceneLookMod {
    pub fn blank() -> Self {
        SceneLookMod {
            suit_asked: None,
            look_place: true,
        }
    }
}

/// (src/scene/look.ts:52 LOOK_LEVELS: a table nothing writes)
pub fn look_levels() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("compound.json"), String::from("compound-gwot.json")); t.set(String::from("barracks.json"), String::from("barracks-gwot.json")); t.set(String::from("warehouse.json"), String::from("warehouse-gwot.json")); t.set(String::from("depot.json"), String::from("depot-gwot.json")); t.set(String::from("registry.json"), String::from("registry-gwot.json")); t.set(String::from("block.json"), String::from("block-gwot.json")); t.set(String::from("block-squads.json"), String::from("block-squads-gwot.json")); t.set(String::from("records.json"), String::from("records-gwot.json")); t.set(String::from("consulate.json"), String::from("consulate-gwot.json")); t.set(String::from("defuse.json"), String::from("defuse-gwot.json")); t.set(String::from("floodlit.json"), String::from("floodlit-gwot.json")); t.set(String::from("mousehole.json"), String::from("mousehole-gwot.json")); t.set(String::from("killhouse.json"), String::from("killhouse-gwot.json")); t.set(String::from("hall.json"), String::from("hall-gwot.json")); t } }
