//! The types of `render_look_tex`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/render/lookTex.ts:54 STUFFS, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct Stuff {
    pub rough: f64,
    pub relief: f64,
    pub built: Option<bool>,
    pub fades: Option<f64>,
    pub soaks: Option<bool>,
    pub worn: Option<f64>,
    pub untiles: Option<bool>,
    pub unit: Option<(f64, f64, bool, f64)>,
    pub bay: Option<[f64; 2]>,
    pub runs: Option<String>,
    pub least: Option<f64>,
    pub under: Option<crate::render_look_tex::StuffUnder>,
    pub foot: Option<String>,
    pub ground: Option<bool>,
    pub sheds: Option<bool>,
    pub rolled: Option<f64>,
    pub mirrors: Option<f64>,
}
impl Stuff {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Stuff {
            rough: 0.0,
            relief: 0.0,
            built: None,
            fades: None,
            soaks: None,
            worn: None,
            untiles: None,
            unit: None,
            bay: None,
            runs: None,
            least: None,
            under: None,
            foot: None,
            ground: None,
            sheds: None,
            rolled: None,
            mirrors: None,
        }
    }
}
impl Default for Stuff { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/lookTex.ts:53 Stuff.under, src/render/lookTex.ts:53)
#[derive(Clone, Debug)]
pub struct StuffUnder {
    pub plinth: String,
    pub core: String,
    pub long: f64,
    pub tall: f64,
}
impl StuffUnder {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StuffUnder {
            plinth: String::new(),
            core: String::new(),
            long: 0.0,
            tall: 0.0,
        }
    }
}
impl Default for StuffUnder { fn default() -> Self { Self::zeroed() } }

/// (src/render/lookTex.ts:12 LOOK_NAMES: a table nothing writes)
pub fn look_names() -> [String; 30] { [String::from("mud wall"), String::from("plaster"), String::from("block"), String::from("earth"), String::from("gravel"), String::from("slab"), String::from("wood"), String::from("sheet"), String::from("cloth"), String::from("metal"), String::from("paint"), String::from("veneer"), String::from("plastic"), String::from("fabric"), String::from("steel"), String::from("carpaint"), String::from("stucco"), String::from("skim"), String::from("gloss block"), String::from("board concrete"), String::from("cast panel"), String::from("paver"), String::from("brushed"), String::from("louvre"), String::from("enamel"), String::from("carpet"), String::from("tarmac"), String::from("grass"), String::from("field stone"), String::from("mud brick")] }

/// (src/render/lookTex.ts:54 STUFFS: a table nothing writes)
pub fn stuffs() -> crate::render_look_tex::STUFFS { crate::render_look_tex::STUFFS { carpet: crate::render_look_tex::Stuff { rough: 0.95, relief: 1.4, built: None, fades: None, soaks: None, worn: None, untiles: Some(true), unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, grass: crate::render_look_tex::Stuff { rough: 0.95, relief: 1.2, built: Some(true), fades: Some(1.0), soaks: Some(true), worn: None, untiles: Some(true), unit: None, bay: None, runs: Some(String::from("flat")), least: None, under: None, foot: None, ground: Some(true), sheds: None, rolled: None, mirrors: None }, metal: crate::render_look_tex::Stuff { rough: 0.75, relief: 1.6, built: None, fades: Some(1.0), soaks: Some(true), worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: Some(true), rolled: None, mirrors: None }, wood: crate::render_look_tex::Stuff { rough: 0.84, relief: 1.6, built: None, fades: Some(1.0), soaks: None, worn: None, untiles: None, unit: None, bay: None, runs: Some(String::from("long")), least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, plastic: crate::render_look_tex::Stuff { rough: 0.5, relief: 1.4, built: None, fades: Some(0.3), soaks: None, worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, fabric: crate::render_look_tex::Stuff { rough: 0.95, relief: 1.6, built: None, fades: Some(0.3), soaks: None, worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, steel: crate::render_look_tex::Stuff { rough: 0.42, relief: 1.4, built: None, fades: Some(0.3), soaks: None, worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, enamel: crate::render_look_tex::Stuff { rough: 0.3, relief: 1.2, built: None, fades: None, soaks: None, worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, tarmac: crate::render_look_tex::Stuff { rough: 0.86, relief: 1.3, built: Some(true), fades: Some(1.0), soaks: None, worn: None, untiles: Some(true), unit: None, bay: None, runs: Some(String::from("flat")), least: None, under: None, foot: None, ground: Some(true), sheds: None, rolled: None, mirrors: None }, mud_wall: crate::render_look_tex::Stuff { rough: 0.95, relief: 1.6, built: Some(true), fades: Some(1.0), soaks: Some(true), worn: None, untiles: Some(true), unit: None, bay: None, runs: Some(String::from("flat")), least: None, under: Some(crate::render_look_tex::StuffUnder { plinth: String::from("field stone"), core: String::from("mud brick"), long: 0.6, tall: 0.24 }), foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, plaster: crate::render_look_tex::Stuff { rough: 0.86, relief: 1.8, built: Some(true), fades: Some(1.0), soaks: Some(true), worn: Some(1.0), untiles: Some(true), unit: None, bay: None, runs: Some(String::from("flat")), least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, block: crate::render_look_tex::Stuff { rough: 0.93, relief: 1.5, built: Some(true), fades: Some(1.0), soaks: Some(true), worn: Some(0.8), untiles: None, unit: None, bay: None, runs: Some(String::from("flat")), least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, earth: crate::render_look_tex::Stuff { rough: 0.94, relief: 1.6, built: Some(true), fades: Some(1.0), soaks: Some(true), worn: None, untiles: Some(true), unit: None, bay: None, runs: Some(String::from("flat")), least: None, under: None, foot: Some(String::from("gravel")), ground: Some(true), sheds: None, rolled: None, mirrors: None }, gravel: crate::render_look_tex::Stuff { rough: 0.88, relief: 1.3, built: Some(true), fades: Some(1.0), soaks: Some(true), worn: None, untiles: Some(true), unit: None, bay: None, runs: Some(String::from("flat")), least: None, under: None, foot: None, ground: Some(true), sheds: None, rolled: None, mirrors: None }, slab: crate::render_look_tex::Stuff { rough: 0.82, relief: 1.8, built: Some(true), fades: Some(1.0), soaks: Some(true), worn: Some(1.0), untiles: None, unit: None, bay: None, runs: Some(String::from("flat")), least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, sheet: crate::render_look_tex::Stuff { rough: 0.5, relief: 1.0, built: None, fades: Some(1.0), soaks: Some(true), worn: None, untiles: None, unit: None, bay: None, runs: Some(String::from("flat")), least: Some(0.2), under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, cloth: crate::render_look_tex::Stuff { rough: 0.96, relief: 1.6, built: None, fades: Some(1.0), soaks: Some(true), worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, paint: crate::render_look_tex::Stuff { rough: 0.5, relief: 1.6, built: None, fades: Some(1.0), soaks: Some(true), worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, veneer: crate::render_look_tex::Stuff { rough: 0.4, relief: 1.4, built: None, fades: Some(0.3), soaks: None, worn: None, untiles: None, unit: None, bay: None, runs: Some(String::from("long")), least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, carpaint: crate::render_look_tex::Stuff { rough: 0.26, relief: 1.2, built: None, fades: Some(0.3), soaks: None, worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: Some(0.16), mirrors: Some(0.08) }, stucco: crate::render_look_tex::Stuff { rough: 0.42, relief: 1.5, built: Some(true), fades: None, soaks: None, worn: Some(1.0), untiles: Some(true), unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, skim: crate::render_look_tex::Stuff { rough: 0.55, relief: 1.6, built: Some(true), fades: None, soaks: None, worn: Some(1.0), untiles: Some(true), unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, gloss_block: crate::render_look_tex::Stuff { rough: 0.4, relief: 1.4, built: Some(true), fades: None, soaks: None, worn: Some(0.6), untiles: None, unit: Some((0.4, 0.2, true, 0.02)), bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, board_concrete: crate::render_look_tex::Stuff { rough: 0.42, relief: 1.6, built: Some(true), fades: None, soaks: None, worn: Some(0.8), untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, cast_panel: crate::render_look_tex::Stuff { rough: 0.68, relief: 1.4, built: Some(true), fades: None, soaks: None, worn: Some(0.8), untiles: None, unit: Some((1.0, 0.5, true, 0.07)), bay: Some([0.0, 3.0]), runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, paver: crate::render_look_tex::Stuff { rough: 0.5, relief: 1.4, built: Some(true), fades: None, soaks: None, worn: Some(1.0), untiles: None, unit: Some((0.5, 0.5, false, 0.018)), bay: Some([6.0, 6.0]), runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, brushed: crate::render_look_tex::Stuff { rough: 0.32, relief: 1.2, built: None, fades: None, soaks: None, worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, louvre: crate::render_look_tex::Stuff { rough: 0.45, relief: 1.0, built: None, fades: None, soaks: None, worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, field_stone: crate::render_look_tex::Stuff { rough: 0.8, relief: 1.0, built: None, fades: Some(1.0), soaks: Some(true), worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None }, mud_brick: crate::render_look_tex::Stuff { rough: 0.95, relief: 1.2, built: None, fades: Some(1.0), soaks: Some(true), worn: None, untiles: None, unit: None, bay: None, runs: None, least: None, under: None, foot: None, ground: None, sheds: None, rolled: None, mirrors: None } } }

/// (src/render/lookTex.ts:77 LOOK_MIRROR_PLAIN)
pub const LOOK_MIRROR_PLAIN: f64 = 0.04;

/// (src/render/lookTex.ts:79 LOOK_WORDS)
pub const LOOK_WORDS: f64 = 32.0;

/// (src/render/lookTex.ts:692 LOOK_PAINTED: a table nothing writes)
pub fn look_painted() -> f64 { todo!() }

/// (src/render/lookTex.ts:697 LOOK_MIRRORS: a table nothing writes)
pub fn look_mirrors() -> Vec<f64> { todo!() }
