//! The types of `render_look_tex`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/render/lookTex.ts:54 STUFFS, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct STUFFS {
    pub carpet: crate::render_look_tex::Stuff,
    pub grass: crate::render_look_tex::Stuff,
    pub metal: crate::render_look_tex::Stuff,
    pub wood: crate::render_look_tex::Stuff,
    pub plastic: crate::render_look_tex::Stuff,
    pub fabric: crate::render_look_tex::Stuff,
    pub steel: crate::render_look_tex::Stuff,
    pub enamel: crate::render_look_tex::Stuff,
    pub tarmac: crate::render_look_tex::Stuff,
    pub mud_wall: crate::render_look_tex::Stuff,
    pub plaster: crate::render_look_tex::Stuff,
    pub block: crate::render_look_tex::Stuff,
    pub earth: crate::render_look_tex::Stuff,
    pub gravel: crate::render_look_tex::Stuff,
    pub slab: crate::render_look_tex::Stuff,
    pub sheet: crate::render_look_tex::Stuff,
    pub cloth: crate::render_look_tex::Stuff,
    pub paint: crate::render_look_tex::Stuff,
    pub veneer: crate::render_look_tex::Stuff,
    pub carpaint: crate::render_look_tex::Stuff,
    pub stucco: crate::render_look_tex::Stuff,
    pub skim: crate::render_look_tex::Stuff,
    pub gloss_block: crate::render_look_tex::Stuff,
    pub board_concrete: crate::render_look_tex::Stuff,
    pub cast_panel: crate::render_look_tex::Stuff,
    pub paver: crate::render_look_tex::Stuff,
    pub brushed: crate::render_look_tex::Stuff,
    pub louvre: crate::render_look_tex::Stuff,
    pub field_stone: crate::render_look_tex::Stuff,
    pub mud_brick: crate::render_look_tex::Stuff,
}
impl STUFFS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STUFFS {
            carpet: crate::render_look_tex::Stuff::zeroed(),
            grass: crate::render_look_tex::Stuff::zeroed(),
            metal: crate::render_look_tex::Stuff::zeroed(),
            wood: crate::render_look_tex::Stuff::zeroed(),
            plastic: crate::render_look_tex::Stuff::zeroed(),
            fabric: crate::render_look_tex::Stuff::zeroed(),
            steel: crate::render_look_tex::Stuff::zeroed(),
            enamel: crate::render_look_tex::Stuff::zeroed(),
            tarmac: crate::render_look_tex::Stuff::zeroed(),
            mud_wall: crate::render_look_tex::Stuff::zeroed(),
            plaster: crate::render_look_tex::Stuff::zeroed(),
            block: crate::render_look_tex::Stuff::zeroed(),
            earth: crate::render_look_tex::Stuff::zeroed(),
            gravel: crate::render_look_tex::Stuff::zeroed(),
            slab: crate::render_look_tex::Stuff::zeroed(),
            sheet: crate::render_look_tex::Stuff::zeroed(),
            cloth: crate::render_look_tex::Stuff::zeroed(),
            paint: crate::render_look_tex::Stuff::zeroed(),
            veneer: crate::render_look_tex::Stuff::zeroed(),
            carpaint: crate::render_look_tex::Stuff::zeroed(),
            stucco: crate::render_look_tex::Stuff::zeroed(),
            skim: crate::render_look_tex::Stuff::zeroed(),
            gloss_block: crate::render_look_tex::Stuff::zeroed(),
            board_concrete: crate::render_look_tex::Stuff::zeroed(),
            cast_panel: crate::render_look_tex::Stuff::zeroed(),
            paver: crate::render_look_tex::Stuff::zeroed(),
            brushed: crate::render_look_tex::Stuff::zeroed(),
            louvre: crate::render_look_tex::Stuff::zeroed(),
            field_stone: crate::render_look_tex::Stuff::zeroed(),
            mud_brick: crate::render_look_tex::Stuff::zeroed(),
        }
    }
}
impl Default for STUFFS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Stuff, src/render/lookTex.ts:52)
#[derive(Clone, Copy, Debug)]
pub struct Stuff {
    pub mirrors: Option<f64>,
}
impl Stuff {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Stuff {
            mirrors: None,
        }
    }
}
impl Default for Stuff { fn default() -> Self { Self::zeroed() } }

/// (src/render/lookTex.ts:12 LOOK_NAMES: a table nothing writes)
pub fn look_names() -> [String; 30] { [String::from("mud wall"), String::from("plaster"), String::from("block"), String::from("earth"), String::from("gravel"), String::from("slab"), String::from("wood"), String::from("sheet"), String::from("cloth"), String::from("metal"), String::from("paint"), String::from("veneer"), String::from("plastic"), String::from("fabric"), String::from("steel"), String::from("carpaint"), String::from("stucco"), String::from("skim"), String::from("gloss block"), String::from("board concrete"), String::from("cast panel"), String::from("paver"), String::from("brushed"), String::from("louvre"), String::from("enamel"), String::from("carpet"), String::from("tarmac"), String::from("grass"), String::from("field stone"), String::from("mud brick")] }

/// (src/render/lookTex.ts:54 STUFFS: a table nothing writes)
pub fn stuffs() -> crate::render_look_tex::STUFFS { crate::render_look_tex::STUFFS { carpet: crate::render_look_tex::Stuff { mirrors: None }, grass: crate::render_look_tex::Stuff { mirrors: None }, metal: crate::render_look_tex::Stuff { mirrors: None }, wood: crate::render_look_tex::Stuff { mirrors: None }, plastic: crate::render_look_tex::Stuff { mirrors: None }, fabric: crate::render_look_tex::Stuff { mirrors: None }, steel: crate::render_look_tex::Stuff { mirrors: None }, enamel: crate::render_look_tex::Stuff { mirrors: None }, tarmac: crate::render_look_tex::Stuff { mirrors: None }, mud_wall: crate::render_look_tex::Stuff { mirrors: None }, plaster: crate::render_look_tex::Stuff { mirrors: None }, block: crate::render_look_tex::Stuff { mirrors: None }, earth: crate::render_look_tex::Stuff { mirrors: None }, gravel: crate::render_look_tex::Stuff { mirrors: None }, slab: crate::render_look_tex::Stuff { mirrors: None }, sheet: crate::render_look_tex::Stuff { mirrors: None }, cloth: crate::render_look_tex::Stuff { mirrors: None }, paint: crate::render_look_tex::Stuff { mirrors: None }, veneer: crate::render_look_tex::Stuff { mirrors: None }, carpaint: crate::render_look_tex::Stuff { mirrors: Some(0.08) }, stucco: crate::render_look_tex::Stuff { mirrors: None }, skim: crate::render_look_tex::Stuff { mirrors: None }, gloss_block: crate::render_look_tex::Stuff { mirrors: None }, board_concrete: crate::render_look_tex::Stuff { mirrors: None }, cast_panel: crate::render_look_tex::Stuff { mirrors: None }, paver: crate::render_look_tex::Stuff { mirrors: None }, brushed: crate::render_look_tex::Stuff { mirrors: None }, louvre: crate::render_look_tex::Stuff { mirrors: None }, field_stone: crate::render_look_tex::Stuff { mirrors: None }, mud_brick: crate::render_look_tex::Stuff { mirrors: None } } }

/// (src/render/lookTex.ts:77 LOOK_MIRROR_PLAIN)
pub const LOOK_MIRROR_PLAIN: f64 = 0.04;

/// (src/render/lookTex.ts:79 LOOK_WORDS)
pub const LOOK_WORDS: f64 = 32.0;

/// (src/render/lookTex.ts:692 LOOK_PAINTED: a table nothing writes)
pub fn look_painted() -> f64 { todo!() }

/// (src/render/lookTex.ts:697 LOOK_MIRRORS: a table nothing writes)
pub fn look_mirrors() -> Vec<f64> { todo!() }
