//! The types of `fogofwar`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FogEye {
    pub id: f64,
    pub pos: V3,
    pub look: V3,
    pub at: V3,
    pub fov: Option<f64>,
    pub near: Option<f64>,
    pub wand: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct FogOfWar {
    pub on: bool,
    pub eyes: Vec<crate::fogofwar::FogEye>,
    pub mates: Vec<CharId>,
    pub hidden_chars: JsSet<CharId>,
    #[is(crate::fogofwar::FogOfWarStats { frames: 0.0, eyes: 0.0, tested: 0.0, seen: 0.0, hidden: 0.0, smoke_hid: 0.0, by_fire: 0.0 })] pub stats: crate::fogofwar::FogOfWarStats,
    pub fired_n: JsMap<u32, f64>,
    pub fired_at: JsMap<GuardId, f64>,
    pub fire_clock: f64,
    pub recs: JsMap<GuardId, crate::fogofwar::Rec>,
    pub door_recs: JsMap<DoorId, crate::fogofwar::FogOfWarDoorRecsValue>,
    pub doors_held: f64,
    pub lamp_recs: JsMap<FixtureId, bool>,
    pub thing_recs: JsMap<crate::Num, crate::fogofwar::ThingRec>,
    pub props_loaded: bool,
    pub loaded: bool,
    pub thing_out: Vec<AnyBoxId>,
    #[is(-1.0)] pub thing_since: f64,
    #[is(crate::fogofwar::FogOfWarThingStats { held: 0.0, out: 0.0, tests: 0.0 })] pub thing_stats: crate::fogofwar::FogOfWarThingStats,
    pub rig_from: f64,
    pub rig_to: f64,
    pub span_x: f64,
    pub span_z: f64,
    pub cell_seen: JsMap<crate::Num, bool>,
    pub frame_no: f64,
    pub door_from: f64,
    pub door_n: f64,
    pub call_log: Vec<crate::fogofwar::FogOfWarCallLogItem>,
    #[is(crate::fogofwar::FogOfWarCallStats { contact: 0.0, heard: 0.0 })] pub call_stats: crate::fogofwar::FogOfWarCallStats,
    pub call_clock: f64,
    #[is(-1000000000.0)] pub last_call: f64,
    pub called_t: JsMap<GuardId, f64>,
    pub known_t: JsMap<GuardId, f64>,
    pub told_step: JsMap<GuardId, f64>,
    pub call_n: f64,
    /// what is filed now is the team's own doing: said on each particle record that goes out
    pub filing_ours: bool,
    /// the fog is on and the pass has run: the team's rule is live (`ours_on`)
    pub live: bool,
    pub our_chunks: JsSet<crate::Num>,
    pub our_landed: JsSet<crate::Num>,
}

/// THE PICTURE'S BLOCK OF THE FOG OF WAR: one machine's, made from the game's state every tick and read by nothing in it (fogofwar/picture.rs).
/// The game lists it among the host's own, so it has no word of the state: two machines that show different seats need not agree on it.
#[derive(Clone, Debug, Blank)]
pub struct FogPicture {
    pub face_recs: JsMap<FixtureId, Vec<V3>>,
    #[is(crate::fogofwar::FogOfWarFullStats { hidden: 0.0, ghosts: 0.0, held: 0.0 })] pub full_stats: crate::fogofwar::FogOfWarFullStats,
    pub viewer: bool,
    pub raw_list: Vec<AnyBoxId>,
    #[is(-1.0)] pub raw_at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FogOfWarStats {
    pub frames: f64,
    pub eyes: f64,
    pub tested: f64,
    pub seen: f64,
    pub hidden: f64,
    pub smoke_hid: f64,
    pub by_fire: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Rec {
    pub seen: bool,
    pub framed: Option<bool>,
    pub t: f64,
    pub at: Option<V3>,
    pub wand_t: Option<f64>,
    pub body: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FogOfWarDoorRecsValue {
    pub shown: f64,
    pub ghost: Option<Vec<BoxId>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ThingRec {
    pub ghost: Vec<FrameBoxId>,
    pub x: f64,
    pub z: f64,
    pub ox: f64,
    pub oz: f64,
    pub f: f64,
    pub moving: bool,
    pub held: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FogOfWarThingStats {
    pub held: f64,
    pub out: f64,
    pub tests: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FogOfWarCallLogItem {
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum FogOfWarCallLogItemKind {
    #[default] Heard,
    Contact,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FogOfWarCallStats {
    pub contact: f64,
    pub heard: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FogOfWarFullStats {
    pub hidden: f64,
    pub ghosts: f64,
    pub held: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FogOfWarCallWordsKind {
    Heard,
    Contact,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FogOfWarSighting {
    pub at: V3,
    pub age: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct FogofwarMod {
    #[is({ let mut s = JsSet::new(); s.add(crate::Num(1.0)); s.add(crate::Num(4.0)); s.add(crate::Num(5.0)); s })] pub thing_pass: JsSet<crate::Num>,
}

pub fn gaze_sights() -> Vec<f64> { alloc::vec![0.7, 0.866] }

pub const COS_FRONT: f64 = 0.7071067811865476;

pub const COS_BEHIND: f64 = -0.5;

pub const CALL_EVERY: f64 = 6.0;

pub const FOG_EYES_WAS: f64 = 5.0;

pub const DOOR_EPS: f64 = 0.01;

pub fn thing_bits() -> Vec<f64> { alloc::vec![0.0, 0.0, 4.0, 3.0] }

pub const THING_TEST: f64 = 7.0;

pub const FIXTURE_LO: f64 = 20480.0;

pub const FIXTURE_HI: f64 = 28672.0;

pub const FIXTURE_BODY: f64 = 6.0;

pub const THING_EPS: f64 = 0.001;

pub const THING_PRIME: f64 = 30.0;

pub const THING_EVERY: f64 = 3.0;

pub const THING_CAP: f64 = 768.0;

pub const PROP_SYS: f64 = 2.0;

pub const EYE_STAND: f64 = 1.55;

pub const EYE_CROUCH: f64 = 0.95;

pub const DEG: f64 = 0.017453292519943295;

pub fn dark() -> [f64; 3] { [0.0, 0.0, 0.0] }

pub const FACE_HOLD: f64 = 0.98;

pub fn door_f() -> Vec<f64> { alloc::vec![0.95, 0.6, 0.25] }

pub fn door_y() -> Vec<f64> { alloc::vec![1.0, 1.8] }

pub const IN_ORDER: f64 = 106181136.0;

pub const ONE_EYE: f64 = 268435456.0;
