//! The types of `blast_look`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Seen {
    pub c: WallChargeId,
    pub t0: f64,
    pub run: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub seen: Vec<crate::blast_look::Seen>,
    pub serial: f64,
    pub first: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CHARGELOOK {
    pub frame_half: f64,
    pub frame_lo: f64,
    pub frame_hi: f64,
    pub strip: f64,
    pub proud: f64,
    pub strip_albedo: V3,
    pub lamp: f64,
    pub blink_every: f64,
    pub blink_on: f64,
    pub lamp_on: V3,
    pub lamp_off: V3,
    pub lamp_spill: f64,
    pub led_spill: f64,
    pub tape: f64,
    pub wire_tape: f64,
    pub wire: f64,
    pub wire_px: f64,
    pub run: [f64; 3],
    pub short: f64,
    pub wire_albedo: V3,
    pub reel: V3,
    pub reel_albedo: V3,
    pub led: f64,
    pub led_on: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorChargeAt {
    pub into: V3,
    pub from: V3,
    pub rest: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorBlastD {
    pub into: V3,
    pub from: V3,
    pub rest: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BurstOfK {
    pub n: f64,
    pub v: f64,
    pub l: f64,
    pub s: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct BlastLookMod {
    pub state: JsMap<PlayerId, crate::blast_look::State>,
}

