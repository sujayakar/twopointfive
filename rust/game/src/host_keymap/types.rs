//! The types of `host_keymap`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: KeyAct, src/host/keymap.ts:76)
#[derive(Clone, Debug)]
pub struct KeyAct {
    pub id: String,
    pub codes: Vec<String>,
    pub cap: String,
}
impl KeyAct {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KeyAct {
            id: String::new(),
            codes: Vec::new(),
            cap: String::new(),
        }
    }
}
impl Default for KeyAct { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/host/keymap.ts:155 Words, src/host/keymap.ts:155)
#[derive(Clone, Debug)]
pub struct Words2 {
    pub re: crate::Opaque,
    pub to: JsMap<String, String>,
}
impl Words2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Words2 {
            re: crate::Opaque,
            to: JsMap::new(),
        }
    }
}
impl Default for Words2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/host/keymap.ts:91 KEYMAP, src/host/keymap.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct HostKeymapModKEYMAP {
    pub on: bool,
    pub q_free: bool,
    pub q_cmd: bool,
    pub q_only: bool,
    pub let_go: bool,
    pub only_when: Option<crate::FnField>,
    pub throw_when: Option<crate::FnField>,
    pub set_free: bool,
    pub set_when: Option<crate::FnField>,
    pub free_when: Option<crate::FnField>,
    pub cam_space: bool,
    pub pinch_only: bool,
    pub slot_when: Option<crate::FnField>,
    pub hand_when: Option<crate::FnField>,
    pub rev: f64,
}
impl HostKeymapModKEYMAP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HostKeymapModKEYMAP {
            on: false,
            q_free: false,
            q_cmd: false,
            q_only: false,
            let_go: false,
            only_when: None,
            throw_when: None,
            set_free: false,
            set_when: None,
            free_when: None,
            cam_space: false,
            pinch_only: false,
            slot_when: None,
            hand_when: None,
            rev: 0.0,
        }
    }
}
impl Default for HostKeymapModKEYMAP { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct HostKeymapMod {
    /// (src/host/keymap.ts:91 KEYMAP)
    pub keymap: crate::host_keymap::HostKeymapModKEYMAP,
    /// (src/host/keymap.ts:150 bound)
    pub bound: JsMap<String, Option<String>>,
    /// (src/host/keymap.ts:152 map)
    pub map: Option<JsMap<String, String>>,
    /// (src/host/keymap.ts:154 builtOn)
    pub built_on: bool,
    /// (src/host/keymap.ts:154 builtFree)
    pub built_free: bool,
    /// (src/host/keymap.ts:154 builtCmd)
    pub built_cmd: bool,
    /// (src/host/keymap.ts:154 builtSlot)
    pub built_slot: bool,
    /// (src/host/keymap.ts:154 builtHand)
    pub built_hand: bool,
    /// (src/host/keymap.ts:154 builtSet)
    pub built_set: bool,
    /// (src/host/keymap.ts:154 builtOnly)
    pub built_only: bool,
    /// (src/host/keymap.ts:154 builtSleep)
    pub built_sleep: f64,
    /// (src/host/keymap.ts:156 text)
    pub text: Option<crate::host_keymap::Words2>,
    /// (src/host/keymap.ts:156 textAll)
    pub text_all: Option<crate::host_keymap::Words2>,
    /// (src/host/keymap.ts:157 byId)
    pub by_id: JsMap<String, crate::host_keymap::KeyAct>,
}
impl HostKeymapMod {
    pub fn blank() -> Self {
        HostKeymapMod {
            keymap: crate::host_keymap::HostKeymapModKEYMAP { on: false, q_free: false, q_cmd: false, q_only: false, let_go: false, only_when: None, throw_when: None, set_free: false, set_when: None, free_when: None, cam_space: false, pinch_only: false, slot_when: None, hand_when: None, rev: 0.0 },
            bound: JsMap::new(),
            map: None,
            built_on: false,
            built_free: false,
            built_cmd: false,
            built_slot: false,
            built_hand: false,
            built_set: false,
            built_only: false,
            built_sleep: 0.0,
            text: None,
            text_all: None,
            by_id: JsMap::new(),
        }
    }
}

/// (src/host/keymap.ts:78 KEY_ACTS: a table nothing writes)
pub fn key_acts() -> Vec<crate::host_keymap::KeyAct> { todo!() }

/// (src/host/keymap.ts:87 SLOT_ACT: a table nothing writes)
pub fn slot_act() -> crate::host_keymap::KeyAct { todo!() }

/// (src/host/keymap.ts:87 WITH_SLOT: a table nothing writes)
pub fn with_slot() -> Vec<crate::host_keymap::KeyAct> { todo!() }

/// (src/host/keymap.ts:107 FREED: a table nothing writes)
pub fn freed() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("KeyQ"), Vec::new()); t } }

/// (src/host/keymap.ts:109 MOVED: a table nothing writes)
pub fn moved() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("KeyB"), alloc::vec![String::from("KeyQ")]); t } }

/// (src/host/keymap.ts:111 DROPPED: a table nothing writes)
pub fn dropped() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("KeyX"), Vec::new()); t.set(String::from("KeyT"), Vec::new()); t } }

/// (src/host/keymap.ts:113 LETTERS: a table nothing writes)
pub fn letters() -> JsMap<String, bool> { { let mut t = JsMap::new(); t.set(String::from("KeyG"), true); t.set(String::from("KeyH"), true); t } }

/// (src/host/keymap.ts:114 MENU_ID)
pub const MENU_ID: &str = "KeyX";

/// (src/host/keymap.ts:119 SETFREED: a table nothing writes)
pub fn setfreed() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("KeyZ"), Vec::new()); t.set(String::from("KeyL"), Vec::new()); t } }

/// (src/host/keymap.ts:124 SLOT: a table nothing writes)
pub fn slot() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("KeyE"), Vec::new()); t.set(String::from("Digit0"), alloc::vec![String::from("Backquote")]); t.set(String::from("Backquote"), alloc::vec![String::from("Backslash")]); t } }

/// (src/host/keymap.ts:126 ALIAS: a table nothing writes)
pub fn alias() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("KeyE"), String::from("Digit0")); t } }

/// (src/host/keymap.ts:129 HAND: a table nothing writes)
pub fn hand() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("KeyV"), Vec::new()); t } }

/// (src/host/keymap.ts:129 HAND_SLOT: a table nothing writes)
pub fn hand_slot() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("Space"), alloc::vec![String::from("KeyE")]); t } }

/// (src/host/keymap.ts:130 SLOT_ID: a table nothing writes)
pub fn slot_id() -> String { todo!() }

/// (src/host/keymap.ts:146 NO_KEYS: a table nothing writes)
pub fn no_keys() -> Vec<String> { Vec::new() }

/// (src/host/keymap.ts:177 PLACES: a table nothing writes)
pub fn places() -> Vec<String> { alloc::vec![String::from("room"), String::from("door"), String::from("wing"), String::from("block"), String::from("side"), String::from("team"), String::from("gate"), String::from("stair"), String::from("stairs"), String::from("hall"), String::from("cell"), String::from("lab"), String::from("bay"), String::from("dock"), String::from("zone"), String::from("sector"), String::from("floor"), String::from("level"), String::from("exit"), String::from("post"), String::from("squad"), String::from("element"), String::from("lane"), String::from("route"), String::from("point"), String::from("site"), String::from("building"), String::from("plan"), String::from("company"), String::from("section"), String::from("junction"), String::from("change")] }

/// (src/host/keymap.ts:219 NONE)
pub const NONE: &str = "—";

/// (src/host/keymap.ts:220 NAMES: a table nothing writes)
pub fn names() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("Space"), String::from("Space")); t.set(String::from("ShiftLeft"), String::from("Shift")); t.set(String::from("ShiftRight"), String::from("R Shift")); t.set(String::from("Backquote"), String::from("`")); t.set(String::from("Minus"), String::from("-")); t.set(String::from("Equal"), String::from("=")); t.set(String::from("BracketLeft"), String::from("[")); t.set(String::from("BracketRight"), String::from("]")); t.set(String::from("Backslash"), String::from("\\")); t.set(String::from("Semicolon"), String::from(";")); t.set(String::from("Quote"), String::from("'")); t.set(String::from("Comma"), String::from(",")); t.set(String::from("Period"), String::from(".")); t.set(String::from("Slash"), String::from("/")); t.set(String::from("Backspace"), String::from("Bksp")); t.set(String::from("Delete"), String::from("Del")); t.set(String::from("Insert"), String::from("Ins")); t.set(String::from("PageUp"), String::from("PgUp")); t.set(String::from("PageDown"), String::from("PgDn")); t.set(String::from("Home"), String::from("Home")); t.set(String::from("End"), String::from("End")); t.set(String::from("IntlBackslash"), String::from("\\")); t.set(String::from("IntlRo"), String::from("Ro")); t.set(String::from("IntlYen"), String::from("¥")); t.set(String::from("NumpadAdd"), String::from("Num +")); t.set(String::from("NumpadSubtract"), String::from("Num -")); t.set(String::from("NumpadMultiply"), String::from("Num *")); t.set(String::from("NumpadDivide"), String::from("Num /")); t.set(String::from("NumpadDecimal"), String::from("Num .")); t.set(String::from("NumpadEqual"), String::from("Num =")); t.set(String::from("NumpadComma"), String::from("Num ,")); t } }

/// (src/host/keymap.ts:259 SINK_LABEL)
pub const SINK_LABEL: &str = "game input";
