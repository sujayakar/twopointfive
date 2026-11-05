//! THE BRING-UP'S OWN PART of dust_look: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The scattering tint of a surface's dust (round three: reached by the stun grenade's tile drop, tick 1621). A pure table; the solver never reads the field.   (dustLook.ts dustTint)
pub fn dust_tint(surf: crate::scene_materials::Surf) -> V3 {
    use crate::scene_materials::Surf as S;
    match surf {
        S::Drywall => [0.9, 0.87, 0.81],
        S::Concrete => [0.72, 0.71, 0.69],
        S::Wood | S::Cardboard => [0.78, 0.68, 0.54],
        S::Fabric | S::Carpet => [0.76, 0.75, 0.74],
        _ => [0.82, 0.8, 0.77],
    }
}
