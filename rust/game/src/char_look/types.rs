//! The types of `char_look`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Look, src/game/charLook.ts:19)
#[derive(Clone, Debug)]
pub struct Look {
    pub ver: f64,
    pub side: Option<crate::anim_gltf::CastSide>,
    pub suit: bool,
    pub t: Vec<f64>,
    pub pal: Vec<f32>,
    pub prox: Option<Vec<V3>>,
    pub prox_wet: f64,
    pub prox_whole: bool,
    pub prox_mask: f64,
}
impl Look {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Look {
            ver: 0.0,
            side: None,
            suit: false,
            t: Vec::new(),
            pal: Vec::new(),
            prox: None,
            prox_wet: 0.0,
            prox_whole: false,
            prox_mask: 0.0,
        }
    }
}
impl Default for Look { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CharLookFrame, src/game/charLook.ts:25)
#[derive(Clone, Copy, Debug)]
pub struct CharLookFrame {
}
impl CharLookFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharLookFrame {
        }
    }
}
impl Default for CharLookFrame { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct CharLookMod {
    /// (src/game/charLook.ts:20 looks)
    pub looks: JsMap<CharId, crate::char_look::Look>,
    /// (src/game/charLook.ts:21 sides)
    pub sides: JsMap<CharId, crate::anim_gltf::CastSide>,
    /// (src/game/charLook.ts:23 suits)
    pub suits: JsMap<CharId, bool>,
    /// (src/game/charLook.ts:33 sd)
    pub sd: crate::render_char_materials::CharSlotData,
    /// (src/game/charLook.ts:33 ver)
    pub ver: f64,
}
impl CharLookMod {
    pub fn blank() -> Self {
        CharLookMod {
            looks: JsMap::new(),
            sides: JsMap::new(),
            suits: JsMap::new(),
            sd: crate::render_char_materials::CharSlotData::zeroed(),
            ver: 0.0,
        }
    }
}

/// (src/game/charLook.ts:44 FRAME: a table nothing writes)
pub fn frame() -> crate::char_look::CharLookFrame { crate::char_look::CharLookFrame {  } }
