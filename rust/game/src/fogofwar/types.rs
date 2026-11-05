//! The types of `fogofwar`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: FogEye, src/game/fogofwar.ts:393)
#[derive(Clone, Copy, Debug)]
pub struct FogEye {
    pub id: f64,
    pub pos: V3,
    pub look: V3,
    pub at: V3,
    pub fov: Option<f64>,
    pub near: Option<f64>,
    pub wand: Option<bool>,
}
impl FogEye {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogEye {
            id: 0.0,
            pos: [0.0; 3],
            look: [0.0; 3],
            at: [0.0; 3],
            fov: None,
            near: None,
            wand: None,
        }
    }
}
impl Default for FogEye { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FogSnap, src/game/fogofwar.ts:378)
#[derive(Clone, Debug)]
pub struct FogSnap {
    pub doors: Option<Vec<(String, f64)>>,
    pub lamps: Option<Vec<(String, f64, Option<Vec<Vec<f64>>>)>>,
    pub things: Option<Vec<(f64, Vec<Vec<f64>>)>>,
    pub out: Option<Vec<f64>>,
    pub bodies: Option<Vec<f64>>,
}
impl FogSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogSnap {
            doors: None,
            lamps: None,
            things: None,
            out: None,
            bodies: None,
        }
    }
}
impl Default for FogSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FogOfWar, src/game/fogofwar.ts:425)
#[derive(Clone, Debug)]
pub struct FogOfWar {
    pub on: bool,
    pub knobs: crate::knobs::FogK,
    pub frame: crate::render_fogofwar::FogFrame,
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
    pub face_recs: JsMap<FixtureId, Vec<V3>>,
    pub lamp_recs: JsMap<FixtureId, bool>,
    pub lamp_pick: Vec<crate::fogofwar::FogOfWarLampPickItem>,
    pub lamp_n: f64,
    pub lit_stats: crate::fogofwar::FogOfWarLitStats,
    pub thing_recs: JsMap<crate::Num, crate::fogofwar::ThingRec>,
    pub props_loaded: bool,
    pub loaded: bool,
    pub thing_out: Vec<BoxId>,
    pub thing_since: f64,
    pub thing_stats: crate::fogofwar::FogOfWarThingStats,
    pub rig_from: f64,
    pub rig_to: f64,
    pub span_x: f64,
    pub span_z: f64,
    pub cell_seen: JsMap<crate::Num, bool>,
    pub frame_no: f64,
    pub asked: JsMap<String, f64>,
    pub pending: Vec<crate::fogofwar::FogOfWarPendingItem>,
    pub turn: f64,
    pub viewer: bool,
    pub bared: bool,
    pub look_frames: f64,
    pub raw_list: Vec<BoxId>,
    pub raw_at: f64,
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
    pub full_stats: crate::fogofwar::FogOfWarFullStats,
    pub full_on: bool,
    pub our_chunks: JsSet<crate::Num>,
    pub our_landed: JsSet<crate::Num>,
}
impl FogOfWar {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        FogOfWar {
            on: false,
            knobs: crate::knobs::FogK { dim: 0.4, desat: 0.0, fov_deg: 360.0, near_m: 2.0, cone: false, cone_deg: 120.0, cone_near_m: 1.5, cone_soft_deg: 12.0, cone_near_soft_m: 0.6, own: false, calls: true, call_secs: 10.0, call_gap: 3.0, call_known: 3.0, smoke: 0.3, linger: 0.25, top_pad: 0.3, doors: true, squad_doors: true, squad_doors_m: 2.0, squad_fire: true, squad_fire_secs: 1.0, lasers: true, things: true, flat: 0.12, lit: 1.0, full: true, air: true, thin: true, eyes: 7.0, kept: true, filed: true, tracks: true, mates: true, ours: true, full_fill: 0.0, edge: 0.5, lit_fill: 0.5, lit_ao: 0.75, lit_ao_m: 1.5, lit_cut: 0.006, last_seen: 0.0 },
            frame: crate::render_fogofwar::FogFrame { eyes: alloc::vec![0.0; (crate::render_fogofwar::FOG_EYES * 4.0) as usize], looks: alloc::vec![0.0; (crate::render_fogofwar::FOG_EYES * 4.0) as usize], dim: 0.4, desat: 0.75, top_pad: 0.3, lights: alloc::vec![0.0; (crate::render_fogofwar::FOG_LIGHTS * 16.0) as usize], lights_u: alloc::vec![0; (crate::render_fogofwar::FOG_LIGHTS * 16.0) as usize], thin_bits: alloc::vec![0; crate::render_fogofwar::FOG_THIN_WORDS as usize], thin_serial: -1.0, thin_count: -1.0, ..crate::render_fogofwar::FogFrame::zeroed() },
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
            face_recs: JsMap::new(),
            lamp_recs: JsMap::new(),
            lamp_pick: Vec::new(),
            lamp_n: 0.0,
            lit_stats: crate::fogofwar::FogOfWarLitStats { lamps: 0.0, lights: 0.0, held: 0.0 },
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
            asked: JsMap::new(),
            pending: Vec::new(),
            turn: 0.0,
            viewer: false,
            bared: false,
            look_frames: 0.0,
            raw_list: Vec::new(),
            raw_at: -1.0,
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
            full_stats: crate::fogofwar::FogOfWarFullStats { hidden: 0.0, ghosts: 0.0, held: 0.0 },
            full_on: false,
            our_chunks: JsSet::new(),
            our_landed: JsSet::new(),
        }
    }
}
impl Default for FogOfWar { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/fogofwar.ts:437 FogOfWar.stats, src/game/fogofwar.ts:437)
#[derive(Clone, Copy, Debug)]
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
impl FogOfWarStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarStats {
            frames: 0.0,
            eyes: 0.0,
            tested: 0.0,
            seen: 0.0,
            hidden: 0.0,
            smoke_asks: 0.0,
            smoke_hid: 0.0,
            by_fire: 0.0,
        }
    }
}
impl Default for FogOfWarStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Rec, src/game/fogofwar.ts:408)
#[derive(Clone, Copy, Debug)]
pub struct Rec {
    pub seen: bool,
    pub framed: Option<bool>,
    pub t: f64,
    pub at: Option<V3>,
    pub wand_t: Option<f64>,
    pub body: bool,
    pub smoked: bool,
    pub smoke_f: f64,
}
impl Rec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rec {
            seen: false,
            framed: None,
            t: 0.0,
            at: None,
            wand_t: None,
            body: false,
            smoked: false,
            smoke_f: 0.0,
        }
    }
}
impl Default for Rec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fogofwar.ts:447 FogOfWar.doorRecs, src/game/fogofwar.ts:447)
#[derive(Clone, Debug)]
pub struct FogOfWarDoorRecsValue {
    pub shown: f64,
    pub ghost: Option<Vec<BoxId>>,
}
impl FogOfWarDoorRecsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarDoorRecsValue {
            shown: 0.0,
            ghost: None,
        }
    }
}
impl Default for FogOfWarDoorRecsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fogofwar.ts:454 FogOfWar.lampPick, src/game/fogofwar.ts:454)
#[derive(Clone, Copy, Debug)]
pub struct FogOfWarLampPickItem {
    pub l: LightId,
    pub i: f64,
    pub d: f64,
}
impl FogOfWarLampPickItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarLampPickItem {
            l: LightId(0),
            i: 0.0,
            d: 0.0,
        }
    }
}
impl Default for FogOfWarLampPickItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fogofwar.ts:455 FogOfWar.litStats, src/game/fogofwar.ts:455)
#[derive(Clone, Copy, Debug)]
pub struct FogOfWarLitStats {
    pub lamps: f64,
    pub lights: f64,
    pub held: f64,
}
impl FogOfWarLitStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarLitStats {
            lamps: 0.0,
            lights: 0.0,
            held: 0.0,
        }
    }
}
impl Default for FogOfWarLitStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ThingRec, src/game/fogofwar.ts:379)
#[derive(Clone, Debug)]
pub struct ThingRec {
    pub ghost: Vec<BoxId>,
    pub x: f64,
    pub z: f64,
    pub ox: f64,
    pub oz: f64,
    pub f: f64,
    pub moving: bool,
    pub held: bool,
}
impl ThingRec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThingRec {
            ghost: Vec::new(),
            x: 0.0,
            z: 0.0,
            ox: 0.0,
            oz: 0.0,
            f: 0.0,
            moving: false,
            held: false,
        }
    }
}
impl Default for ThingRec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fogofwar.ts:458 FogOfWar.thingStats, src/game/fogofwar.ts:458)
#[derive(Clone, Copy, Debug)]
pub struct FogOfWarThingStats {
    pub held: f64,
    pub out: f64,
    pub tests: f64,
}
impl FogOfWarThingStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarThingStats {
            held: 0.0,
            out: 0.0,
            tests: 0.0,
        }
    }
}
impl Default for FogOfWarThingStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fogofwar.ts:464 FogOfWar.pending, src/game/fogofwar.ts:464)
#[derive(Clone, Debug)]
pub struct FogOfWarPendingItem {
    pub key: String,
    pub a: V3,
    pub b: V3,
}
impl FogOfWarPendingItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarPendingItem {
            key: String::new(),
            a: [0.0; 3],
            b: [0.0; 3],
        }
    }
}
impl Default for FogOfWarPendingItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fogofwar.ts:623 FogOfWar.callLog, src/game/fogofwar.ts:623)
#[derive(Clone, Debug)]
pub struct FogOfWarCallLogItem {
    pub t: f64,
    pub who: String,
    pub of: String,
    pub kind: crate::fogofwar::FogOfWarCallLogItemKind,
    pub text: String,
}
impl FogOfWarCallLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarCallLogItem {
            t: 0.0,
            who: String::new(),
            of: String::new(),
            kind: crate::fogofwar::FogOfWarCallLogItemKind::Heard,
            text: String::new(),
        }
    }
}
impl Default for FogOfWarCallLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FogOfWarCallLogItemKind {
    Heard,
    Contact,
}
impl FogOfWarCallLogItemKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            FogOfWarCallLogItemKind::Heard => "heard",
            FogOfWarCallLogItemKind::Contact => "contact",
        }
    }
}

/// (TypeScript: inline at src/game/fogofwar.ts:624 FogOfWar.callStats, src/game/fogofwar.ts:624)
#[derive(Clone, Copy, Debug)]
pub struct FogOfWarCallStats {
    pub contact: f64,
    pub heard: f64,
}
impl FogOfWarCallStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarCallStats {
            contact: 0.0,
            heard: 0.0,
        }
    }
}
impl Default for FogOfWarCallStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fogofwar.ts:816 FogOfWar.fullStats, src/game/fogofwar.ts:816)
#[derive(Clone, Copy, Debug)]
pub struct FogOfWarFullStats {
    pub hidden: f64,
    pub ghosts: f64,
    pub held: f64,
}
impl FogOfWarFullStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarFullStats {
            hidden: 0.0,
            ghosts: 0.0,
            held: 0.0,
        }
    }
}
impl Default for FogOfWarFullStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fogofwar.ts:349 FOG_PAGE, src/game/fogofwar.ts:349)
#[derive(Clone, Copy, Debug)]
pub struct FogofwarModFOGPAGE {
    pub desat: f64,
    pub edge: f64,
    pub air: bool,
    pub kept: bool,
}
impl FogofwarModFOGPAGE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogofwarModFOGPAGE {
            desat: 0.0,
            edge: 0.0,
            air: false,
            kept: false,
        }
    }
}
impl Default for FogofwarModFOGPAGE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FogOfWarCallWordsKind {
    Heard,
    Contact,
}
impl FogOfWarCallWordsKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            FogOfWarCallWordsKind::Heard => "heard",
            FogOfWarCallWordsKind::Contact => "contact",
        }
    }
}

/// (TypeScript: inline at src/game/fogofwar.ts:1098 sighting ->, src/game/fogofwar.ts:1098)
#[derive(Clone, Copy, Debug)]
pub struct FogOfWarSighting {
    pub at: V3,
    pub age: f64,
}
impl FogOfWarSighting {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarSighting {
            at: [0.0; 3],
            age: 0.0,
        }
    }
}
impl Default for FogOfWarSighting { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fogofwar.ts:1114 lastSeen ->, src/game/fogofwar.ts:1114)
#[derive(Clone, Copy, Debug)]
pub struct FogOfWarLastSeenItem {
    pub gd: GuardId,
    pub at: V3,
    pub age: f64,
    pub life: f64,
}
impl FogOfWarLastSeenItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogOfWarLastSeenItem {
            gd: GuardId(0),
            at: [0.0; 3],
            age: 0.0,
            life: 0.0,
        }
    }
}
impl Default for FogOfWarLastSeenItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct FogofwarMod {
    /// (src/game/fogofwar.ts:349 FOG_PAGE)
    pub fog_page: crate::fogofwar::FogofwarModFOGPAGE,
    /// (src/game/fogofwar.ts:356 THING_PASS)
    pub thing_pass: JsSet<crate::Num>,
}
impl FogofwarMod {
    pub fn blank() -> Self {
        FogofwarMod {
            fog_page: { let k = crate::knobs::page_fog(); crate::fogofwar::FogofwarModFOGPAGE { desat: k.desat, edge: k.edge, air: k.air, kept: k.kept } },
            thing_pass: { let mut s = JsSet::new(); s.add(crate::Num(1.0)); s.add(crate::Num(4.0)); s.add(crate::Num(5.0)); s },
        }
    }
}

/// (src/game/fogofwar.ts:322 GAZE_SIGHTS: a table nothing writes)
pub fn gaze_sights() -> Vec<f64> { alloc::vec![0.7, 0.866] }

/// (src/game/fogofwar.ts:324 COS_FRONT)
pub const COS_FRONT: f64 = 0.7071067811865476;

/// (src/game/fogofwar.ts:324 COS_BEHIND)
pub const COS_BEHIND: f64 = -0.5;

/// (src/game/fogofwar.ts:326 CALL_EVERY)
pub const CALL_EVERY: f64 = 6.0;

/// (src/game/fogofwar.ts:328 FOG_FLAT_ON)
pub const FOG_FLAT_ON: f64 = 0.12;

/// (src/game/fogofwar.ts:330 FOG_LIT_ON)
pub const FOG_LIT_ON: f64 = 1.0;

/// (src/game/fogofwar.ts:332 FOG_DESAT_WAS)
pub const FOG_DESAT_WAS: f64 = 0.75;

/// (src/game/fogofwar.ts:334 FOG_EDGE_ON)
pub const FOG_EDGE_ON: f64 = 0.5;

/// (src/game/fogofwar.ts:336 FOG_EYES_ON)
pub const FOG_EYES_ON: f64 = 7.0;

/// (src/game/fogofwar.ts:336 FOG_EYES_WAS)
pub const FOG_EYES_WAS: f64 = 5.0;

/// (src/game/fogofwar.ts:351 DOOR_EPS)
pub const DOOR_EPS: f64 = 0.01;

/// (src/game/fogofwar.ts:355 THING_BITS: a table nothing writes)
pub fn thing_bits() -> Vec<f64> { alloc::vec![0.0, 0.0, 4.0, 3.0] }

/// (src/game/fogofwar.ts:357 THING_TEST)
pub const THING_TEST: f64 = 7.0;

/// (src/game/fogofwar.ts:361 FIXTURE_LO)
pub const FIXTURE_LO: f64 = 20480.0;

/// (src/game/fogofwar.ts:361 FIXTURE_HI)
pub const FIXTURE_HI: f64 = 28672.0;

/// (src/game/fogofwar.ts:361 FIXTURE_BODY)
pub const FIXTURE_BODY: f64 = 6.0;

/// (src/game/fogofwar.ts:372 THING_EPS)
pub const THING_EPS: f64 = 0.001;

/// (src/game/fogofwar.ts:372 THING_PRIME)
pub const THING_PRIME: f64 = 30.0;

/// (src/game/fogofwar.ts:372 THING_EVERY)
pub const THING_EVERY: f64 = 3.0;

/// (src/game/fogofwar.ts:372 THING_CAP)
pub const THING_CAP: f64 = 768.0;

/// (src/game/fogofwar.ts:382 PROP_SYS)
pub const PROP_SYS: f64 = 2.0;

/// (src/game/fogofwar.ts:386 BEAM_STEP)
pub const BEAM_STEP: f64 = 0.75;

/// (src/game/fogofwar.ts:386 BEAM_FINE)
pub const BEAM_FINE: f64 = 0.1;

/// (src/game/fogofwar.ts:407 EYE_STAND)
pub const EYE_STAND: f64 = 1.55;

/// (src/game/fogofwar.ts:407 EYE_CROUCH)
pub const EYE_CROUCH: f64 = 0.95;

/// (src/game/fogofwar.ts:410 SMOKE_ASKS)
pub const SMOKE_ASKS: f64 = 6.0;

/// (src/game/fogofwar.ts:410 SMOKE_EYES)
pub const SMOKE_EYES: f64 = 2.0;

/// (src/game/fogofwar.ts:412 SMOKE_FRESH)
pub const SMOKE_FRESH: f64 = 8.0;

/// (src/game/fogofwar.ts:412 SMOKE_KEEP)
pub const SMOKE_KEEP: f64 = 30.0;

/// (src/game/fogofwar.ts:413 DEG)
pub const DEG: f64 = 0.017453292519943295;

/// (src/game/fogofwar.ts:415 DARK: a table nothing writes)
pub fn dark() -> [f64; 3] { [0.0, 0.0, 0.0] }

/// (src/game/fogofwar.ts:417 FACE_HOLD)
pub const FACE_HOLD: f64 = 0.98;

/// (src/game/fogofwar.ts:419 DOOR_F: a table nothing writes)
pub fn door_f() -> Vec<f64> { alloc::vec![0.95, 0.6, 0.25] }

/// (src/game/fogofwar.ts:419 DOOR_Y: a table nothing writes)
pub fn door_y() -> Vec<f64> { alloc::vec![1.0, 1.8] }

/// (src/game/fogofwar.ts:421 nearEyes: a table nothing writes)
pub fn near_eyes() -> Vec<crate::fogofwar::FogEye> { Vec::new() }

/// (src/game/fogofwar.ts:421 nearD2: a table nothing writes)
pub fn near_d2() -> Vec<f64> { Vec::new() }

/// (src/game/fogofwar.ts:423 IN_ORDER)
pub const IN_ORDER: f64 = 106181136.0;

/// (src/game/fogofwar.ts:423 ONE_EYE)
pub const ONE_EYE: f64 = 268435456.0;
