//! The types of `char_look`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct CharLookFrame {
}

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
            sd: crate::render_char_materials::CharSlotData::default(),
            ver: 0.0,
        }
    }
}

/// (src/game/charLook.ts:44 FRAME: a table nothing writes)
pub fn frame() -> crate::char_look::CharLookFrame { crate::char_look::CharLookFrame {  } }
