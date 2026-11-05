//! The types of `lanes`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a walk's price list, nav.findPath's penalty: a cell of the mask costs `dear`, any other 1, or at least the pursuit tactics' price where there is one; a man the rule leaves alone pays the tactics' price alone (lanes.ts priceOf, lanePenalty)
#[derive(Clone, Debug)]
pub enum LanePenalty {
    Dear { m: Vec<u8>, w: f64, dear: f64 },
    DearOver { base: crate::tactics::TacticsPenalty, m: Vec<u8>, w: f64, dear: f64 },
    Base { base: crate::tactics::TacticsPenalty },
}

/// (TypeScript: Lane, src/game/lanes.ts:156)
#[derive(Clone, Debug)]
pub struct Lane {
    pub id: f64,
    pub from: V3,
    pub at: V3,
    pub dx: f64,
    pub dz: f64,
    pub t0: f64,
    pub last_t: f64,
    pub until: f64,
    pub weight: f64,
    pub downs: f64,
    pub known: bool,
    pub known_at: f64,
    pub look_at: f64,
    pub guns: Vec<V3>,
    pub saw: JsMap<GuardId, f64>,
    pub knowers: JsSet<GuardId>,
    pub cells: Option<JsSet<crate::Num>>,
    pub wedges: Vec<V3>,
    pub re: Option<crate::lanes::LaneRe>,
    pub who: JsSet<crate::Num>,
    pub moved: JsSet<crate::Num>,
    pub round_at: f64,
    pub stun_at: f64,
    pub push_until: f64,
    pub can: Option<crate::lanes::LaneCan>,
    pub peek_at: f64,
    pub peek_n: f64,
    pub peek_part: f64,
    pub pin_at: f64,
    pub pin_n: f64,
    pub pin_by: f64,
    pub frag_at: f64,
    pub e: f64,
}
impl Lane {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Lane {
            id: 0.0,
            from: [0.0; 3],
            at: [0.0; 3],
            dx: 0.0,
            dz: 0.0,
            t0: 0.0,
            last_t: 0.0,
            until: 0.0,
            weight: 0.0,
            downs: 0.0,
            known: false,
            known_at: 0.0,
            look_at: 0.0,
            guns: Vec::new(),
            saw: JsMap::new(),
            knowers: JsSet::new(),
            cells: None,
            wedges: Vec::new(),
            re: None,
            who: JsSet::new(),
            moved: JsSet::new(),
            round_at: 0.0,
            stun_at: 0.0,
            push_until: 0.0,
            can: None,
            peek_at: 0.0,
            peek_n: 0.0,
            peek_part: 0.0,
            pin_at: 0.0,
            pin_n: 0.0,
            pin_by: 0.0,
            frag_at: 0.0,
            e: 0.0,
        }
    }
}
impl Default for Lane { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:172 Lane.re, src/game/lanes.ts:172)
#[derive(Clone, Debug)]
pub struct LaneRe {
    pub set: JsSet<crate::Num>,
    pub i: f64,
}
impl LaneRe {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaneRe {
            set: JsSet::new(),
            i: 0.0,
        }
    }
}
impl Default for LaneRe { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:175 Lane.can, src/game/lanes.ts:175)
#[derive(Clone, Copy, Debug)]
pub struct LaneCan {
    pub who: f64,
    pub t: f64,
    pub land: V3,
}
impl LaneCan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaneCan {
            who: 0.0,
            t: 0.0,
            land: [0.0; 3],
        }
    }
}
impl Default for LaneCan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Flank, src/game/lanes.ts:187)
#[derive(Clone, Copy, Debug)]
pub struct Flank {
    pub lane: f64,
    pub phase: crate::lanes::FlankPhase,
    pub at: V3,
    pub t: f64,
    pub n: f64,
}
impl Flank {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Flank {
            lane: 0.0,
            phase: crate::lanes::FlankPhase::Set,
            at: [0.0; 3],
            t: 0.0,
            n: 0.0,
        }
    }
}
impl Default for Flank { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FlankPhase {
    Set,
    Go,
    Walk,
}
impl FlankPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            FlankPhase::Set => "set",
            FlankPhase::Go => "go",
            FlankPhase::Walk => "walk",
        }
    }
}

/// (TypeScript: Peek, src/game/lanes.ts:188)
#[derive(Clone, Copy, Debug)]
pub struct Peek {
    pub lane: f64,
    pub phase: crate::lanes::PeekPhase,
    pub t: f64,
    pub home: V3,
    pub look: V3,
    pub to: V3,
    pub line_t: f64,
    pub eye: Option<V3>,
    pub fire: Option<crate::lanes::PeekFire>,
    pub frag: Option<crate::lanes::PeekFrag>,
}
impl Peek {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Peek {
            lane: 0.0,
            phase: crate::lanes::PeekPhase::Out,
            t: 0.0,
            home: [0.0; 3],
            look: [0.0; 3],
            to: [0.0; 3],
            line_t: 0.0,
            eye: None,
            fire: None,
            frag: None,
        }
    }
}
impl Default for Peek { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PeekPhase {
    Out,
    Back,
}
impl PeekPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            PeekPhase::Out => "out",
            PeekPhase::Back => "back",
        }
    }
}

/// (TypeScript: inline at src/game/lanes.ts:190 Peek.fire, src/game/lanes.ts:190)
#[derive(Clone, Copy, Debug)]
pub struct PeekFire {
    pub n: f64,
    pub t: f64,
    pub next: f64,
    pub saw: bool,
    pub line_t: f64,
}
impl PeekFire {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PeekFire {
            n: 0.0,
            t: 0.0,
            next: 0.0,
            saw: false,
            line_t: 0.0,
        }
    }
}
impl Default for PeekFire { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:192 Peek.frag, src/game/lanes.ts:192)
#[derive(Clone, Copy, Debug)]
pub struct PeekFrag {
    pub at: Option<V3>,
    pub k: f64,
    pub next: f64,
    pub walk: bool,
    pub going: bool,
    pub edge: bool,
    pub own: bool,
    pub until: f64,
    pub had: f64,
}
impl PeekFrag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PeekFrag {
            at: None,
            k: 0.0,
            next: 0.0,
            walk: false,
            going: false,
            edge: false,
            own: false,
            until: 0.0,
            had: 0.0,
        }
    }
}
impl Default for PeekFrag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/lanes.ts:193)
#[derive(Clone, Debug)]
pub struct State {
    pub lanes: Vec<crate::lanes::Lane>,
    pub serial: f64,
    pub mask: Option<Vec<u8>>,
    pub ver: f64,
    pub own: JsMap<GuardId, crate::lanes::StateOwnValue>,
    pub seen: JsSet<u32>,
    pub banged: JsSet<u32>,
    pub push: JsMap<GuardId, f64>,
    pub flank: JsMap<GuardId, f64>,
    pub flank_of: JsMap<GuardId, crate::lanes::Flank>,
    pub dropped: JsSet<GuardId>,
    pub fail_t: JsMap<GuardId, f64>,
    pub skip: bool,
    pub held: JsMap<GuardId, crate::lanes::StateHeldValue>,
    pub cans: Vec<ItemId>,
    pub roll: f64,
    pub due: Vec<crate::lanes::StateDueItem>,
    pub look: JsMap<GuardId, f64>,
    pub post: JsMap<GuardId, V3>,
    pub back_t: JsMap<GuardId, f64>,
    pub said_t: f64,
    pub t: f64,
    pub tick_t: f64,
    pub stats: crate::lanes::StateStats,
    pub peek: JsMap<GuardId, crate::lanes::Peek>,
    pub reloads: JsMap<GuardId, f64>,
    pub frag_no: JsMap<GuardId, crate::lanes::StateFragNoValue>,
    pub near: Vec<crate::lanes::StateNearItem>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            lanes: Vec::new(),
            serial: 0.0,
            mask: None,
            ver: 0.0,
            own: JsMap::new(),
            seen: JsSet::new(),
            banged: JsSet::new(),
            push: JsMap::new(),
            flank: JsMap::new(),
            flank_of: JsMap::new(),
            dropped: JsSet::new(),
            fail_t: JsMap::new(),
            skip: false,
            held: JsMap::new(),
            cans: Vec::new(),
            roll: 0.0,
            due: Vec::new(),
            look: JsMap::new(),
            post: JsMap::new(),
            back_t: JsMap::new(),
            said_t: 0.0,
            t: 0.0,
            tick_t: 0.0,
            stats: crate::lanes::StateStats::zeroed(),
            peek: JsMap::new(),
            reloads: JsMap::new(),
            frag_no: JsMap::new(),
            near: Vec::new(),
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:197 State.own, src/game/lanes.ts:197)
#[derive(Clone, Debug)]
pub struct StateOwnValue {
    pub ver: f64,
    pub m: Vec<u8>,
}
impl StateOwnValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateOwnValue {
            ver: 0.0,
            m: Vec::new(),
        }
    }
}
impl Default for StateOwnValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:200 State.held, src/game/lanes.ts:200)
#[derive(Clone, Copy, Debug)]
pub struct StateHeldValue {
    pub lane: f64,
    pub t: f64,
    pub look: V3,
    pub aimed: bool,
    pub at: V3,
}
impl StateHeldValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateHeldValue {
            lane: 0.0,
            t: 0.0,
            look: [0.0; 3],
            aimed: false,
            at: [0.0; 3],
        }
    }
}
impl Default for StateHeldValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:200 State.due, src/game/lanes.ts:200)
#[derive(Clone, Copy, Debug)]
pub struct StateDueItem {
    pub gd: GuardId,
    pub lane: f64,
    pub look: V3,
}
impl StateDueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateDueItem {
            gd: GuardId(0),
            lane: 0.0,
            look: [0.0; 3],
        }
    }
}
impl Default for StateDueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:201 State.stats, src/game/lanes.ts:201)
#[derive(Clone, Copy, Debug)]
pub struct StateStats {
    pub lanes: f64,
    pub known: f64,
    pub unknown: f64,
    pub held: f64,
    pub rerouted: f64,
    pub backed_out: f64,
    pub round: f64,
    pub stuns: f64,
    pub pushes: f64,
    pub lapsed: f64,
    pub peeks: f64,
    pub peek_there: f64,
    pub peek_clear: f64,
    pub peek_lost: f64,
    pub pins: f64,
    pub pin_rounds: f64,
    pub pin_lost: f64,
    pub sets: f64,
    pub goes: f64,
    pub frags: f64,
    pub frag_lost: f64,
    pub frag_kept: f64,
}
impl StateStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateStats {
            lanes: 0.0,
            known: 0.0,
            unknown: 0.0,
            held: 0.0,
            rerouted: 0.0,
            backed_out: 0.0,
            round: 0.0,
            stuns: 0.0,
            pushes: 0.0,
            lapsed: 0.0,
            peeks: 0.0,
            peek_there: 0.0,
            peek_clear: 0.0,
            peek_lost: 0.0,
            pins: 0.0,
            pin_rounds: 0.0,
            pin_lost: 0.0,
            sets: 0.0,
            goes: 0.0,
            frags: 0.0,
            frag_lost: 0.0,
            frag_kept: 0.0,
        }
    }
}
impl Default for StateStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:207 State.fragNo, src/game/lanes.ts:207)
#[derive(Clone, Copy, Debug)]
pub struct StateFragNoValue {
    pub lane: f64,
    pub at: V3,
}
impl StateFragNoValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateFragNoValue {
            lane: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for StateFragNoValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:209 State.near, src/game/lanes.ts:209)
#[derive(Clone, Copy, Debug)]
pub struct StateNearItem {
    pub gd: GuardId,
    pub from: V3,
    pub at: V3,
    pub who: f64,
    pub t: f64,
}
impl StateNearItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateNearItem {
            gd: GuardId(0),
            from: [0.0; 3],
            at: [0.0; 3],
            who: 0.0,
            t: 0.0,
        }
    }
}
impl Default for StateNearItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LanesSave, src/game/lanes.ts:984)
#[derive(Clone, Debug)]
pub struct LanesSave {
    pub serial: f64,
    pub lanes: Vec<crate::lanes::LanesSaveLanesItem>,
}
impl LanesSave {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LanesSave {
            serial: 0.0,
            lanes: Vec::new(),
        }
    }
}
impl Default for LanesSave { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:984 LanesSave.lanes, src/game/lanes.ts:984)
#[derive(Clone, Debug)]
pub struct LanesSaveLanesItem {
    pub id: f64,
    pub from: Vec<f64>,
    pub at: Vec<f64>,
    pub t0: f64,
    pub last_t: f64,
    pub until: f64,
    pub weight: f64,
    pub downs: f64,
    pub known: bool,
    pub known_at: f64,
    pub guns: Vec<Vec<f64>>,
    pub wedges: Vec<Vec<f64>>,
    pub round_at: f64,
    pub stun_at: f64,
    pub who: Option<Vec<f64>>,
    pub peek_at: Option<f64>,
    pub peek_n: Option<f64>,
    pub peek_part: Option<f64>,
    pub frag_at: Option<f64>,
    pub e: Option<f64>,
}
impl LanesSaveLanesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LanesSaveLanesItem {
            id: 0.0,
            from: Vec::new(),
            at: Vec::new(),
            t0: 0.0,
            last_t: 0.0,
            until: 0.0,
            weight: 0.0,
            downs: 0.0,
            known: false,
            known_at: 0.0,
            guns: Vec::new(),
            wedges: Vec::new(),
            round_at: 0.0,
            stun_at: 0.0,
            who: None,
            peek_at: None,
            peek_n: None,
            peek_part: None,
            frag_at: None,
            e: None,
        }
    }
}
impl Default for LanesSaveLanesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:591 stepper ->, src/game/lanes.ts:591)
#[derive(Clone, Copy, Debug)]
pub struct Stepper {
    pub gd: GuardId,
    pub to: V3,
}
impl Stepper {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Stepper {
            gd: GuardId(0),
            to: [0.0; 3],
        }
    }
}
impl Default for Stepper { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum FragPlace {
    List(V3),
    Edge,
}

/// (TypeScript: inline at src/game/lanes.ts:980 lanesList ->, src/game/lanes.ts:980)
#[derive(Clone, Copy, Debug)]
pub struct LanesListItem {
    pub id: f64,
    pub from: V3,
    pub at: V3,
    pub known: bool,
    pub weight: f64,
    pub downs: f64,
    pub cells: f64,
    pub until: f64,
    pub push_until: f64,
}
impl LanesListItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LanesListItem {
            id: 0.0,
            from: [0.0; 3],
            at: [0.0; 3],
            known: false,
            weight: 0.0,
            downs: 0.0,
            cells: 0.0,
            until: 0.0,
            push_until: 0.0,
        }
    }
}
impl Default for LanesListItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct LanesMod {
    /// (src/game/lanes.ts:211 states)
    pub states: Option<crate::lanes::State>,
}
impl LanesMod {
    pub fn blank() -> Self {
        LanesMod {
            states: None,
        }
    }
}

/// (src/game/lanes.ts:154 LANES_DEFAULTS: a table nothing writes)
pub fn lanes_defaults() -> crate::knobs::LanesK { crate::knobs::page_lanes() }

/// (src/game/lanes.ts:217 SIDE)
pub const SIDE: &str = "hostile";

/// (src/game/lanes.ts:574 SPOT_RING)
pub const SPOT_RING: f64 = 0.7;
