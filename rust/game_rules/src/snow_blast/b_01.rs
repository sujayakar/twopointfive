//! THE BRING-UP'S OWN PART of snow_blast: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// A blast over the snow is told to the picture: the crater, the dig, the throw and the shove are its own (src/picture/snow). Nothing of the game reads it.   (snowBlast.ts:142 snowBlastAt)
pub fn snow_blast_at(g: &mut crate::game::Game, at: &[f64], radius: f64) -> bool {
    let name = crate::scene_level_load::level_name_of(g);
    if !crate::weather_kinds::snow_level(g, name.as_deref()) { return false; }
    g.out.push(crate::out_events::OutEvent::SnowBlast(crate::out_events::OutSnowBlast { at: [at[0], at[1], at[2]], radius }));
    true
}
