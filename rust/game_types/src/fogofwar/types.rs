//! The types of `fogofwar`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
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

#[derive(Clone, Debug, ToBytes)]
pub struct FogOfWar {
    pub on: bool,
    pub eyes: Vec<crate::fogofwar::FogEye>,
    pub mates: Vec<CharId>,
    pub hidden_chars: JsSet<CharId>,
    pub stats: crate::fogofwar::FogOfWarStats,
    pub fired_n: JsMap<u32, f64>,
    pub fired_at: JsMap<GuardId, f64>,
    pub fire_clock: f64,
    pub shown: JsSet<GuardId>,
    pub recs: JsMap<GuardId, crate::fogofwar::Rec>,
    pub door_recs: JsMap<DoorId, crate::fogofwar::FogOfWarDoorRecsValue>,
    pub doors_held: f64,
    pub lamp_recs: JsMap<FixtureId, bool>,
    pub thing_recs: JsMap<crate::Num, crate::fogofwar::ThingRec>,
    pub props_loaded: bool,
    pub loaded: bool,
    pub thing_out: Vec<AnyBoxId>,
    pub thing_since: f64,
    pub thing_stats: crate::fogofwar::FogOfWarThingStats,
    pub rig_from: f64,
    pub rig_to: f64,
    pub span_x: f64,
    pub span_z: f64,
    pub cell_seen: JsMap<crate::Num, bool>,
    pub frame_no: f64,
    pub door_from: f64,
    pub door_n: f64,
    pub call_log: Vec<crate::fogofwar::FogOfWarCallLogItem>,
    pub call_stats: crate::fogofwar::FogOfWarCallStats,
    pub call_clock: f64,
    pub last_call: f64,
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
impl FogOfWar {
    /// what a new one holds
    pub fn blank() -> Self {
        FogOfWar {
            on: false,
            eyes: Vec::new(),
            mates: Vec::new(),
            hidden_chars: JsSet::new(),
            stats: crate::fogofwar::FogOfWarStats { frames: 0.0, eyes: 0.0, tested: 0.0, seen: 0.0, hidden: 0.0, smoke_asks: 0.0, smoke_hid: 0.0, by_fire: 0.0 },
            fired_n: JsMap::new(),
            fired_at: JsMap::new(),
            fire_clock: 0.0,
            shown: JsSet::new(),
            recs: JsMap::new(),
            door_recs: JsMap::new(),
            doors_held: 0.0,
            lamp_recs: JsMap::new(),
            thing_recs: JsMap::new(),
            props_loaded: false,
            loaded: false,
            thing_out: Vec::new(),
            thing_since: -1.0,
            thing_stats: crate::fogofwar::FogOfWarThingStats { held: 0.0, out: 0.0, tests: 0.0 },
            rig_from: 0.0,
            rig_to: 0.0,
            span_x: 0.0,
            span_z: 0.0,
            cell_seen: JsMap::new(),
            frame_no: 0.0,
            door_from: 0.0,
            door_n: 0.0,
            call_log: Vec::new(),
            call_stats: crate::fogofwar::FogOfWarCallStats { contact: 0.0, heard: 0.0 },
            call_clock: 0.0,
            last_call: -1000000000.0,
            called_t: JsMap::new(),
            known_t: JsMap::new(),
            told_step: JsMap::new(),
            call_n: 0.0,
            filing_ours: false,
            live: false,
            our_chunks: JsSet::new(),
            our_landed: JsSet::new(),
        }
    }
}
impl Default for FogOfWar { fn default() -> Self { Self::blank() } }

/// THE PICTURE'S BLOCK OF THE FOG OF WAR: one machine's, made from the game's state every tick and read by nothing in it (fogofwar/picture.rs).
/// The game lists it among the host's own, so it has no word of the state: two machines that show different seats need not agree on it.
#[derive(Clone, Debug)]
pub struct FogPicture {
    pub face_recs: JsMap<FixtureId, Vec<V3>>,
    pub full_stats: crate::fogofwar::FogOfWarFullStats,
    pub viewer: bool,
    pub raw_list: Vec<AnyBoxId>,
    pub raw_at: f64,
}
impl FogPicture {
    pub fn blank() -> Self {
        FogPicture {
            face_recs: JsMap::new(),
            full_stats: crate::fogofwar::FogOfWarFullStats { hidden: 0.0, ghosts: 0.0, held: 0.0 },
            viewer: false,
            raw_list: Vec::new(),
            raw_at: -1.0,
        }
    }
}
impl Default for FogPicture { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FogOfWarStats {
    pub frames: f64,
    pub eyes: f64,
    pub tested: f64,
    pub seen: f64,
    pub hidden: f64,
    pub smoke_asks: f64,
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
    pub t: f64,
    pub who: String,
    pub of: String,
    pub kind: crate::fogofwar::FogOfWarCallLogItemKind,
    pub text: String,
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FogofwarModFOGPAGE {
    pub desat: f64,
    pub edge: f64,
    pub air: bool,
    pub kept: bool,
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
#[derive(Clone, Debug, ToBytes)]
pub struct FogofwarMod {
    /// (src/game/fogofwar.ts:348 FOG_PAGE)
    pub fog_page: crate::fogofwar::FogofwarModFOGPAGE,
    /// (src/game/fogofwar.ts:355 THING_PASS)
    pub thing_pass: JsSet<crate::Num>,
}
impl FogofwarMod {
    pub fn blank() -> Self {
        FogofwarMod {
            fog_page: { let k = crate::knobs::FogK::page(); crate::fogofwar::FogofwarModFOGPAGE { desat: k.desat, edge: k.edge, air: k.air, kept: k.kept } },
            thing_pass: { let mut s = JsSet::new(); s.add(crate::Num(1.0)); s.add(crate::Num(4.0)); s.add(crate::Num(5.0)); s },
        }
    }
}

/// (src/game/fogofwar.ts:321 GAZE_SIGHTS: a table nothing writes)
pub fn gaze_sights() -> Vec<f64> { alloc::vec![0.7, 0.866] }

/// (src/game/fogofwar.ts:323 COS_FRONT)
pub const COS_FRONT: f64 = 0.7071067811865476;

/// (src/game/fogofwar.ts:323 COS_BEHIND)
pub const COS_BEHIND: f64 = -0.5;

/// (src/game/fogofwar.ts:325 CALL_EVERY)
pub const CALL_EVERY: f64 = 6.0;

/// (src/game/fogofwar.ts:335 FOG_EYES_WAS)
pub const FOG_EYES_WAS: f64 = 5.0;

/// (src/game/fogofwar.ts:350 DOOR_EPS)
pub const DOOR_EPS: f64 = 0.01;

/// (src/game/fogofwar.ts:354 THING_BITS: a table nothing writes)
pub fn thing_bits() -> Vec<f64> { alloc::vec![0.0, 0.0, 4.0, 3.0] }

/// (src/game/fogofwar.ts:356 THING_TEST)
pub const THING_TEST: f64 = 7.0;

/// (src/game/fogofwar.ts:360 FIXTURE_LO)
pub const FIXTURE_LO: f64 = 20480.0;

/// (src/game/fogofwar.ts:360 FIXTURE_HI)
pub const FIXTURE_HI: f64 = 28672.0;

/// (src/game/fogofwar.ts:360 FIXTURE_BODY)
pub const FIXTURE_BODY: f64 = 6.0;

/// (src/game/fogofwar.ts:371 THING_EPS)
pub const THING_EPS: f64 = 0.001;

/// (src/game/fogofwar.ts:371 THING_PRIME)
pub const THING_PRIME: f64 = 30.0;

/// (src/game/fogofwar.ts:371 THING_EVERY)
pub const THING_EVERY: f64 = 3.0;

/// (src/game/fogofwar.ts:371 THING_CAP)
pub const THING_CAP: f64 = 768.0;

/// (src/game/fogofwar.ts:381 PROP_SYS)
pub const PROP_SYS: f64 = 2.0;

/// (src/game/fogofwar.ts:406 EYE_STAND)
pub const EYE_STAND: f64 = 1.55;

/// (src/game/fogofwar.ts:406 EYE_CROUCH)
pub const EYE_CROUCH: f64 = 0.95;

/// (src/game/fogofwar.ts:412 DEG)
pub const DEG: f64 = 0.017453292519943295;

/// (src/game/fogofwar.ts:414 DARK: a table nothing writes)
pub fn dark() -> [f64; 3] { [0.0, 0.0, 0.0] }

/// (src/game/fogofwar.ts:416 FACE_HOLD)
pub const FACE_HOLD: f64 = 0.98;

/// (src/game/fogofwar.ts:418 DOOR_F: a table nothing writes)
pub fn door_f() -> Vec<f64> { alloc::vec![0.95, 0.6, 0.25] }

/// (src/game/fogofwar.ts:418 DOOR_Y: a table nothing writes)
pub fn door_y() -> Vec<f64> { alloc::vec![1.0, 1.8] }

/// (src/game/fogofwar.ts:422 IN_ORDER)
pub const IN_ORDER: f64 = 106181136.0;

/// (src/game/fogofwar.ts:422 ONE_EYE)
pub const ONE_EYE: f64 = 268435456.0;
