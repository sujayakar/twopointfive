//! THE BRING-UP'S OWN PART of cast: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY (the picture's: cast.ts:72, does this man wear the black sneak suit, read only for the COLOUR of the boxes hung on him): the neutral answer, the issue dress.
/// NOT what the TypeScript answers on a quiet mission; nothing but albedo follows from it.
pub fn sneak_suit(g: &mut crate::game::Game, c_2: CharId) -> bool { false }

/// BOUNDARY (the picture's: cast.ts:79, which meshes a man wears, dealt each frame into Character.parts): what he wears already. The deal itself (the variant by his
/// place in the squad, the suit, the pack's side, the goggles) is not translated; it draws no die and no rule reads `parts`.
pub fn cast_parts(g: &mut crate::game::Game, c_2: CharId) -> f64 { g.ch[c_2].parts }
