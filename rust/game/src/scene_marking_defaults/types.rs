//! The types of `scene_marking_defaults`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/scene/markingDefaults.ts:19 MK, src/scene/markingDefaults.ts:19)
#[derive(Clone, Copy, Debug)]
pub struct MK {
    pub tape: V3,
    pub ink: V3,
    pub label: V3,
    pub text_grey: V3,
    pub marker_blue: V3,
    pub marker_red: V3,
    pub marker_green: V3,
    pub key_deck: V3,
    pub key_cap: V3,
    pub steel: V3,
    pub led_green: V3,
    pub led_amber: V3,
    pub led_glass: V3,
    pub screen_off: V3,
}
impl MK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MK {
            tape: [0.0; 3],
            ink: [0.0; 3],
            label: [0.0; 3],
            text_grey: [0.0; 3],
            marker_blue: [0.0; 3],
            marker_red: [0.0; 3],
            marker_green: [0.0; 3],
            key_deck: [0.0; 3],
            key_cap: [0.0; 3],
            steel: [0.0; 3],
            led_green: [0.0; 3],
            led_amber: [0.0; 3],
            led_glass: [0.0; 3],
            screen_off: [0.0; 3],
        }
    }
}
impl Default for MK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/markingDefaults.ts:31 P, src/scene/markingDefaults.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct P {
    pub tape: crate::scene_markings::MarkPal,
    pub ink: crate::scene_markings::MarkPal,
    pub label: crate::scene_markings::MarkPal,
    pub grey: crate::scene_markings::MarkPal,
    pub steel: crate::scene_markings::MarkPal,
}
impl P {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        P {
            tape: crate::scene_markings::MarkPal::zeroed(),
            ink: crate::scene_markings::MarkPal::zeroed(),
            label: crate::scene_markings::MarkPal::zeroed(),
            grey: crate::scene_markings::MarkPal::zeroed(),
            steel: crate::scene_markings::MarkPal::zeroed(),
        }
    }
}
impl Default for P { fn default() -> Self { Self::zeroed() } }

/// (src/scene/markingDefaults.ts:11 T)
pub const T: f64 = 0.025;

/// (src/scene/markingDefaults.ts:19 MK: a table nothing writes)
pub fn mk() -> crate::scene_marking_defaults::MK { crate::scene_marking_defaults::MK { tape: [0.36, 0.22, 0.1], ink: [0.16, 0.12, 0.08], label: [0.86, 0.85, 0.8], text_grey: [0.42, 0.42, 0.41], marker_blue: [0.2, 0.28, 0.5], marker_red: [0.55, 0.16, 0.12], marker_green: [0.2, 0.4, 0.25], key_deck: [0.035, 0.035, 0.04], key_cap: [0.15, 0.15, 0.16], steel: [0.5, 0.5, 0.48], led_green: [0.25, 6.0, 0.7], led_amber: [6.0, 2.2, 0.2], led_glass: [0.02, 0.05, 0.025], screen_off: [0.02, 0.02, 0.025] } }

/// (src/scene/markingDefaults.ts:31 P: a table nothing writes)
pub fn p() -> crate::scene_marking_defaults::P { crate::scene_marking_defaults::P { tape: crate::scene_markings::MarkPal { albedo: Some([0.36, 0.22, 0.1]), mul: None, rough: Some(0.35), emit: None }, ink: crate::scene_markings::MarkPal { albedo: Some([0.16, 0.12, 0.08]), mul: None, rough: None, emit: None }, label: crate::scene_markings::MarkPal { albedo: Some([0.86, 0.85, 0.8]), mul: None, rough: None, emit: None }, grey: crate::scene_markings::MarkPal { albedo: Some([0.42, 0.42, 0.41]), mul: None, rough: None, emit: None }, steel: crate::scene_markings::MarkPal { albedo: Some([0.5, 0.5, 0.48]), mul: None, rough: Some(0.32), emit: None } } }
