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
    pub last: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BLAST {
    pub on: bool,
    pub stun: bool,
    pub door: bool,
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
pub struct REF {
    pub sparks: f64,
    pub speed: f64,
    pub life: f64,
    pub size: f64,
}

#[derive(Clone, Debug, Default)]
pub struct WALLBLAST {
    pub jet: crate::blast_look::WALLBLASTJet,
    pub core: crate::blast_look::WALLBLASTCore,
    pub shower: crate::blast_look::WALLBLASTShower,
    pub embers: crate::blast_look::WALLBLASTEmbers,
    pub chips: crate::blast_look::WALLBLASTChips,
    pub dust: crate::blast_look::WALLBLASTDust,
}

#[derive(Clone, Debug, Default)]
pub struct WALLBLASTJet {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Debug, Default)]
pub struct WALLBLASTCore {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Debug, Default)]
pub struct WALLBLASTShower {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Debug, Default)]
pub struct WALLBLASTEmbers {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Debug, Default)]
pub struct WALLBLASTChips {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
    pub aspect: f64,
}

#[derive(Clone, Debug, Default)]
pub struct WALLBLASTDust {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub jit: f64,
    pub cone: Vec<f64>,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct STUNSPARKS {
    pub lobes: crate::blast_look::STUNSPARKSLobes,
    pub fan: crate::blast_look::STUNSPARKSFan,
    pub fountain: crate::blast_look::STUNSPARKSFountain,
    pub embers: crate::blast_look::STUNSPARKSEmbers,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct STUNSPARKSLobes {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub cone: f64,
    pub fan: f64,
    pub rise: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct STUNSPARKSFan {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub lift: f64,
    pub spread: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct STUNSPARKSFountain {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub spread: f64,
    pub lift: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct STUNSPARKSEmbers {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub spread: f64,
    pub lift: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DOORSPARKS {
    pub jet: crate::blast_look::DOORSPARKSJet,
    pub skitter: crate::blast_look::DOORSPARKSSkitter,
    pub back: crate::blast_look::DOORSPARKSBack,
    pub fountain: f64,
    pub ahead: f64,
    pub lead: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DOORSPARKSJet {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub cone: f64,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DOORSPARKSSkitter {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub cone: f64,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DOORSPARKSBack {
    pub count: f64,
    pub speed: f64,
    pub jit: f64,
    pub cone: f64,
    pub tilt: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
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

pub fn blast() -> crate::blast_look::BLAST { crate::blast_look::BLAST { on: true, stun: true, door: true } }

pub fn ref_() -> crate::blast_look::REF { crate::blast_look::REF { sparks: 160.0, speed: 20.0, life: 0.55, size: 0.0025 } }

pub fn wall_blast_table() -> crate::blast_look::WALLBLAST { crate::blast_look::WALLBLAST { jet: crate::blast_look::WALLBLASTJet { count: alloc::vec![30.0, 14.0], speed: alloc::vec![21.0, 15.0], jit: 0.65, cone: alloc::vec![0.36, 0.5], tilt: -0.2, life: 0.24, life_jit: 0.5, size: 0.003, size_jit: 0.5 }, core: crate::blast_look::WALLBLASTCore { count: alloc::vec![14.0, 7.0], speed: alloc::vec![1.6, 1.1], jit: 0.6, cone: alloc::vec![1.3, 1.3], tilt: 0.1, life: 0.95, life_jit: 0.5, size: 0.04, size_jit: 0.5 }, shower: crate::blast_look::WALLBLASTShower { count: alloc::vec![18.0, 10.0], speed: alloc::vec![10.0, 8.0], jit: 0.7, cone: alloc::vec![0.85, 0.95], tilt: -0.3, life: 0.55, life_jit: 0.5, size: 0.004, size_jit: 0.5 }, embers: crate::blast_look::WALLBLASTEmbers { count: alloc::vec![18.0, 9.0], speed: alloc::vec![6.0, 4.5], jit: 0.85, cone: alloc::vec![1.1, 1.2], tilt: -0.1, life: 1.2, life_jit: 0.8, size: 0.0035, size_jit: 0.8 }, chips: crate::blast_look::WALLBLASTChips { count: alloc::vec![48.0, 22.0], speed: alloc::vec![7.5, 4.5], jit: 0.75, cone: alloc::vec![0.7, 0.85], tilt: 0.12, life: 20.0, life_jit: 0.3, size: 0.04, size_jit: 0.7, aspect: 1.7 }, dust: crate::blast_look::WALLBLASTDust { count: alloc::vec![44.0, 24.0], speed: alloc::vec![5.5, 3.6], jit: 0.6, cone: alloc::vec![0.55, 0.7], tilt: 0.05, life: 1.7, life_jit: 0.3, size: 0.3, size_jit: 0.35 } } }

pub fn stun_sparks_table() -> crate::blast_look::STUNSPARKS { crate::blast_look::STUNSPARKS { lobes: crate::blast_look::STUNSPARKSLobes { count: 34.0, speed: 17.0, jit: 0.6, cone: 0.17, fan: 0.3, rise: 0.05, life: 0.3, life_jit: 0.5, size: 0.0025, size_jit: 0.4 }, fan: crate::blast_look::STUNSPARKSFan { count: 40.0, speed: 10.0, jit: 0.7, lift: 0.12, spread: 0.5, life: 0.5, life_jit: 0.5, size: 0.003, size_jit: 0.4 }, fountain: crate::blast_look::STUNSPARKSFountain { count: 22.0, speed: 6.0, jit: 0.5, spread: 0.55, lift: 0.5, life: 0.6, life_jit: 0.4, size: 0.003, size_jit: 0.4 }, embers: crate::blast_look::STUNSPARKSEmbers { count: 18.0, speed: 3.0, jit: 0.6, spread: 0.8, lift: 0.2, life: 1.5, life_jit: 0.5, size: 0.0045, size_jit: 0.4 } } }

pub fn door_sparks() -> crate::blast_look::DOORSPARKS { crate::blast_look::DOORSPARKS { jet: crate::blast_look::DOORSPARKSJet { count: 56.0, speed: 19.0, jit: 0.65, cone: 0.4, tilt: -0.08, life: 0.45, life_jit: 0.5, size: 0.003, size_jit: 0.5 }, skitter: crate::blast_look::DOORSPARKSSkitter { count: 44.0, speed: 10.0, jit: 0.7, cone: 0.9, tilt: -0.3, life: 0.55, life_jit: 0.5, size: 0.004, size_jit: 0.5 }, back: crate::blast_look::DOORSPARKSBack { count: 22.0, speed: 12.0, jit: 0.65, cone: 0.5, tilt: -0.2, life: 0.3, life_jit: 0.5, size: 0.003, size_jit: 0.5 }, fountain: 16.0, ahead: 0.2, lead: 1.5 } }

