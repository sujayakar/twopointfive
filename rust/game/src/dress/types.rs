//! The types of `dress`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/dress.ts:11 DRESS, src/game/dress.ts:11)
#[derive(Clone, Copy, Debug)]
pub struct DressModDRESS {
    pub air: bool,
    pub moths: bool,
    pub snow: bool,
    pub rain: bool,
    pub wet_ground: bool,
    pub fires: bool,
    pub wake: bool,
    pub men: bool,
    pub cues: bool,
    pub recap: bool,
    pub blast_feel: bool,
    pub fixtures: bool,
    pub fx: bool,
    pub stature: bool,
    pub cases: bool,
    pub net: bool,
    pub wind: bool,
    pub debris: bool,
}
impl DressModDRESS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DressModDRESS {
            air: false,
            moths: false,
            snow: false,
            rain: false,
            wet_ground: false,
            fires: false,
            wake: false,
            men: false,
            cues: false,
            recap: false,
            blast_feel: false,
            fixtures: false,
            fx: false,
            stature: false,
            cases: false,
            net: false,
            wind: false,
            debris: false,
        }
    }
}
impl Default for DressModDRESS { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DressMod {
    /// (src/game/dress.ts:11 DRESS)
    pub dress: crate::dress::DressModDRESS,
}
impl DressMod {
    pub fn blank() -> Self {
        DressMod {
            dress: crate::dress::DressModDRESS { air: true, moths: true, snow: true, rain: true, wet_ground: true, fires: true, wake: true, men: true, cues: true, recap: true, blast_feel: true, fixtures: true, fx: true, stature: true, cases: true, net: true, wind: true, debris: true },
        }
    }
}
