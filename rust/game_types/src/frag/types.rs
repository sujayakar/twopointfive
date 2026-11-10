//! The types of `frag`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BlastReport {
    pub at: V3,
    pub hits: Vec<crate::frag::BlastHit>,
    pub killed: f64,
    pub finished: f64,
    pub player: Option<crate::frag::BlastResult>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BlastHit {
    pub id: f64,
    pub team: crate::teams::Team,
    pub exposed: f64,
    pub result: crate::frag::BlastResult,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum BlastResult {
    #[default] Wounded,
    Killed,
    Shielded,
    Stunned,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum BlastHitBehind {
    Door,
    Cover,
}

#[derive(Clone, Debug, Default)]
pub struct BlastSpec {
    pub who: f64,
    pub lethal_m: Option<f64>,
    pub wound_m: Option<f64>,
    pub serial: Option<f64>,
    pub name: Option<String>,
    pub fx: Option<bool>,
    pub destr: Option<crate::frag::BlastSpecDestr>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlastSpecDestr {
    pub radius: Option<f64>,
    pub power: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassView {
    Blown,
    Wary,
    Counted,
    Whole,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Live {
    pub it: ItemId,
    pub side: crate::teams::Team,
    pub knows: JsMap<GuardId, f64>,
    pub shouted: JsSet<crate::teams::Team>,
    pub flee: JsMap<GuardId, Option<crate::game::GuardScript>>,
    pub via: Option<JsSet<GuardId>>,
    pub flee_at: JsMap<GuardId, V3>,
    pub deal_t: JsMap<GuardId, f64>,
    pub counted: JsSet<GuardId>,
    pub cued: bool,
    pub rest: Option<crate::frag::Rest>,
    pub made: Option<JsMap<GuardId, crate::frag::LiveMadeValue>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Rest {
    pub t: f64,
    pub still: bool,
    pub at: V3,
    pub path: Vec<V3>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LiveMadeValue {
    pub at: V3,
    pub t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Carrier {
    pub gd: GuardId,
    pub frags: f64,
    pub blocked_t: f64,
    pub bunch_t: f64,
    pub try_at: f64,
    pub wind: Option<crate::frag::CarrierWind>,
    pub away: Option<f64>,
    pub spots: Option<crate::frag::CarrierSpots>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CarrierWind {
    pub until: f64,
    pub at: V3,
    pub swung: bool,
    pub last_t: f64,
    pub hand: Option<V3>,
    pub toward: Option<V3>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CarrierSpots {
    pub at: V3,
    pub from: V3,
    pub t: f64,
    pub list: Vec<V3>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DefBook {
    pub men: Vec<GuardId>,
    pub carriers: Vec<crate::frag::Carrier>,
    pub last_t: f64,
    pub serial: f64,
    pub thrown: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FragModWALLSFORValue {
    pub time: f64,
    pub serial: f64,
    pub at: JsMap<String, Vec<crate::wallbreak::WallOpening>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FragModTORNValue {
    pub time: f64,
    pub doors: Vec<DoorId>,
}

#[derive(Clone, Debug, Default)]
pub struct RollOn {
    pub at: V3,
    pub path: Vec<V3>,
}

#[derive(Clone, Debug, Default)]
pub struct RestOfThrow {
    pub at: V3,
    pub path: Vec<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FleeSpotC {
    pub pos: V3,
}

#[derive(Clone, Debug, Default)]
pub struct FleeSpotWise {
    pub path: Vec<V3>,
    pub keep: Option<V3>,
    pub not: Option<V3>,
    pub apart: Option<crate::frag::FleeSpotWiseApart>,
}

#[derive(Clone, Debug, Default)]
pub struct FleeSpotWiseApart {
    pub stay: Vec<V3>,
    pub go: Vec<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FleeByDoorC {
    pub pos: V3,
}

#[derive(Clone, Debug, Default)]
pub struct FleeByDoorWise {
    pub path: Vec<V3>,
    pub keep: Option<V3>,
    pub not: Option<V3>,
    pub apart: Option<crate::frag::FleeByDoorWiseApart>,
}

#[derive(Clone, Debug, Default)]
pub struct FleeByDoorWiseApart {
    pub stay: Vec<V3>,
    pub go: Vec<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FleeByDoor {
    pub goal: V3,
    pub via: bool,
}

#[derive(Clone, Debug, Default)]
pub struct TakenBy {
    pub stay: Vec<V3>,
    pub go: Vec<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct OwnBangE {
    pub frag: Option<bool>,
    pub who: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum WindUp {
    Own,
    Arc,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FragThrowAt {
    Thrown,
    Own,
    Arc,
    Busy,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct FragMod {
    pub frag_log: Vec<crate::frag::BlastReport>,
    pub frag_why: JsMap<String, f64>,
    pub walls_for: Option<crate::frag::FragModWALLSFORValue>,
    pub torn: Option<crate::frag::FragModTORNValue>,
    pub warned: JsMap<GuardId, f64>,
    pub books: Option<Vec<crate::frag::Live>>,
    pub def_books: Option<crate::frag::DefBook>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FragStats {
    pub bursts: i32,
    pub fled: i32,
    pub by_door: i32,
    pub own: i32,
    pub cues: i32,
}

pub fn shouts() -> Vec<String> { alloc::vec![String::from("frag out!"), String::from("grenade out!"), String::from("fire in the hole!")] }
