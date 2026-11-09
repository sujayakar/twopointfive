//! The types of `squadthrow`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a throw search's test: a frag thrown so catches nobody of ours; a stun always passes
#[derive(Clone, Debug)]
pub enum LobClean {
    Plan { kind: crate::squadthrow::SquadThrowKind, at: V3 },
    LookAgain { kind: crate::squadthrow::SquadThrowKind, at: V3, gd: GuardId },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum SquadThrowKind {
    #[default] Frag,
    Stun,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Arc {
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadThrower {
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
    pub gd: GuardId,
    pub post: Option<V3>,
}

#[derive(Clone, Debug, ToBytes)]
pub struct Walk {
    pub kind: crate::squadthrow::SquadThrowKind,
    pub at: V3,
    pub post: V3,
    pub arc: crate::squadthrow::Arc,
    pub spent: f64,
    pub last_t: f64,
    pub on_t: f64,
    pub thrown_t: f64,
    pub again: f64,
    pub held: Option<crate::squadthrow::Arc>,
    pub held_at: Option<V3>,
    pub held_t: Option<f64>,
    pub line: Vec<V3>,
    pub mates: Vec<GuardId>,
    pub gd: GuardId,
    pub t: crate::orders::OrderTarget,
    pub a: Option<OrdersAssignmentId>,
    pub home: V3,
    pub home_yaw: f64,
    pub back: Option<crate::squadthrow::WalkBack>,
}
impl Default for Walk {
    fn default() -> Self {
        Walk {
            kind: crate::squadthrow::SquadThrowKind::Frag,
            at: [0.0; 3],
            post: [0.0; 3],
            arc: crate::squadthrow::Arc::default(),
            spent: 0.0,
            last_t: 0.0,
            on_t: 0.0,
            thrown_t: 0.0,
            again: 0.0,
            held: None,
            held_at: None,
            held_t: None,
            line: Vec::new(),
            mates: Vec::new(),
            gd: GuardId(0),
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::default()),
            a: None,
            home: [0.0; 3],
            home_yaw: 0.0,
            back: None,
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WalkBack {
    pub post: Option<V3>,
    pub going: String,
    pub order: String,
    pub face: Option<f64>,
    pub there: String,
    pub t0: Option<f64>,
    pub follow: Option<f64>,
    pub watch: Option<V3>,
    pub door: Option<DoorId>,
    pub strafe: Option<bool>,
    pub stance: Option<crate::character::Stance>,
    pub upper: Option<crate::character::UpperMode>,
    pub stack_side: Option<f64>,
    pub move_: Option<crate::doortools::DoorToolsDoneItemLaneBackMove>,
    pub set_line: Option<String>,
    pub group: Option<f64>,
    pub set_said: Option<bool>,
    pub goal: Option<V3>,
    pub arrived: Option<bool>,
    pub held_set: Option<bool>,
    pub held_fire: Option<bool>,
    pub arr_t: Option<f64>,
    pub moved_t: Option<f64>,
    pub light: Option<bool>,
    pub low: Option<bool>,
    pub yield_t: Option<f64>,
    pub face_now: Option<f64>,
    pub search: Option<bool>,
    pub search_t: Option<f64>,
    pub via: Option<Vec<V3>>,
    pub anchor: Option<V3>,
    pub mark: Option<V3>,
    pub mark_t: Option<f64>,
    pub had: Option<V3>,
    pub back_n: Option<f64>,
    pub shy: Option<DoorId>,
    pub thru: Option<DoorId>,
    pub via_for: Option<V3>,
    pub via_t: Option<f64>,
    pub pace: Option<f64>,
    pub spare: Option<bool>,
    pub from: Option<V3>,
    pub foe: Option<GuardId>,
    pub crouch: Option<bool>,
    pub lean: Option<f64>,
    pub fit: Option<crate::roomfit::Fit>,
    pub tries: Option<crate::doortools::DoorToolsDoneItemLaneBackTries>,
}

#[derive(Clone, Debug)]
pub struct Lent {
    pub mates: Vec<GuardId>,
    pub gd: GuardId,
    pub t: crate::orders::OrderTarget,
    pub a: Option<OrdersAssignmentId>,
    pub home: V3,
    pub home_yaw: f64,
    pub back: Option<crate::squadthrow::LentBack>,
}
impl Default for Lent {
    fn default() -> Self {
        Lent {
            mates: Vec::new(),
            gd: GuardId(0),
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::default()),
            a: None,
            home: [0.0; 3],
            home_yaw: 0.0,
            back: None,
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LentBack {
    pub post: Option<V3>,
    pub going: String,
    pub order: String,
    pub face: Option<f64>,
    pub there: String,
    pub t0: Option<f64>,
    pub follow: Option<f64>,
    pub watch: Option<V3>,
    pub door: Option<DoorId>,
    pub strafe: Option<bool>,
    pub stance: Option<crate::character::Stance>,
    pub upper: Option<crate::character::UpperMode>,
    pub stack_side: Option<f64>,
    pub move_: Option<crate::doortools::DoorToolsDoneItemLaneBackMove>,
    pub set_line: Option<String>,
    pub group: Option<f64>,
    pub set_said: Option<bool>,
    pub goal: Option<V3>,
    pub arrived: Option<bool>,
    pub held_set: Option<bool>,
    pub held_fire: Option<bool>,
    pub arr_t: Option<f64>,
    pub moved_t: Option<f64>,
    pub light: Option<bool>,
    pub low: Option<bool>,
    pub yield_t: Option<f64>,
    pub face_now: Option<f64>,
    pub search: Option<bool>,
    pub search_t: Option<f64>,
    pub via: Option<Vec<V3>>,
    pub anchor: Option<V3>,
    pub mark: Option<V3>,
    pub mark_t: Option<f64>,
    pub had: Option<V3>,
    pub back_n: Option<f64>,
    pub shy: Option<DoorId>,
    pub thru: Option<DoorId>,
    pub via_for: Option<V3>,
    pub via_t: Option<f64>,
    pub pace: Option<f64>,
    pub spare: Option<bool>,
    pub from: Option<V3>,
    pub foe: Option<GuardId>,
    pub crouch: Option<bool>,
    pub lean: Option<f64>,
    pub fit: Option<crate::roomfit::Fit>,
    pub tries: Option<crate::doortools::DoorToolsDoneItemLaneBackTries>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Scratch {
    pub n: f64,
    pub gen_: f64,
    pub seen: Vec<i32>,
    pub leaf: Vec<i32>,
    pub dep: Vec<u8>,
    pub who: Vec<u8>,
    pub q: Vec<i32>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub last: JsMap<GuardId, f64>,
    pub shown: Option<crate::squadthrow::StateShown>,
    pub walks: JsMap<GuardId, crate::squadthrow::Walk>,
    /// the throws the menu shows, one a kind, as they were last worked out (Rust's own: not on the wire)
    pub plans: JsMap<crate::squadthrow::SquadThrowKind, crate::squadthrow::ThrowPlan>,
    pub scratch: Option<crate::squadthrow::Scratch>,
}

/// THE SQUAD'S THROW AT A SPOT, and everything anybody is shown or told of it: the menu's row, its refusal, its warning, the picture, and the order
/// given are all read off one of these (squadthrow/plan.rs).
#[derive(Clone, Debug, ToBytes)]
pub struct ThrowPlan {
    pub at: V3,
    /// the men it was asked of
    pub men: Vec<GuardId>,
    /// the game's clock when it was worked out
    pub t: f64,
    /// where everybody of ours stood then, to half a metre, and what is left: while it stands, so does the plan on the menu
    pub read: String,
    /// the man, his throw, and the spot he walks to for it; none: nobody has a throw
    pub thrower: Option<crate::squadthrow::SquadThrower>,
    /// why it may not be given, in the row's words
    pub refuse: Option<String>,
    /// what the row says after its words of a throw that may be given
    pub warn: Option<String>,
    /// his way to that spot, as the orders would walk him
    pub walk: Option<Vec<V3>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StateShown {
    pub t: f64,
    pub text: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadThrowKeptDuck {
    pub gd: GuardId,
    pub t: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub m: Option<crate::squadthrow::SquadThrowKeptDuckM>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadThrowKeptDuckM {
    pub stay: Vec<V3>,
    pub go: Vec<V3>,
}

#[derive(Clone, Debug)]
pub struct Lob {
    pub walls: Vec<V3>,
    pub lim: crate::squadthrow::LobLim,
    pub clean: crate::squadthrow::LobClean,
    pub dirty: Option<crate::squadthrow::SquadThrower>,
}
impl Default for Lob {
    fn default() -> Self {
        Lob {
            walls: Vec::new(),
            lim: crate::squadthrow::LobLim::default(),
            clean: crate::squadthrow::LobClean::Plan { kind: crate::squadthrow::SquadThrowKind::Frag, at: [0.0; 3] },
            dirty: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LobLim {
    pub n: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadthrowModSQUADTHROWLOGItem {
    pub who: String,
    pub rest: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Word {
    Frag,
    Bang,
}
impl Word {
    pub const fn as_str(self) -> &'static str {
        match self {
            Word::Frag => "frag",
            Word::Bang => "bang",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum None {
    NoBangsLeft,
    NoFragsInThisKit,
    NoFragsLeft,
}
impl None {
    pub const fn as_str(self) -> &'static str {
        match self {
            None::NoBangsLeft => "no bangs left.",
            None::NoFragsInThisKit => "no frags in this kit.",
            None::NoFragsLeft => "no frags left.",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RowOf {
    FragThere,
    StunThere,
}
impl RowOf {
    pub const fn as_str(self) -> &'static str {
        match self {
            RowOf::FragThere => "fragThere",
            RowOf::StunThere => "stunThere",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SquadThrower2 {
    pub gd: GuardId,
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}

#[derive(Clone, Debug, Default)]
pub struct StandThrow {
    pub gd: GuardId,
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadFragCatchesMoved {
    pub gd: GuardId,
    pub at: V3,
}

#[derive(Clone, Debug, Default)]
pub struct SquadFragCatchesBurst {
    pub reach: f64,
    pub no: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FragCatchesMoved {
    pub gd: GuardId,
    pub at: V3,
}

#[derive(Clone, Debug, Default)]
pub struct FragCatchesBurst {
    pub reach: f64,
    pub no: String,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SquadthrowMod {
    pub states: JsMap<PlayerId, crate::squadthrow::State>,
    pub squadthrow_log: Vec<crate::squadthrow::SquadthrowModSQUADTHROWLOGItem>,
    pub duck_mates: Option<crate::squadthrow::SquadThrowKeptDuck>,
}
impl SquadthrowMod {
    pub fn blank() -> Self {
        SquadthrowMod {
            states: JsMap::new(),
            squadthrow_log: Vec::new(),
            duck_mates: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SquadthrowStats {
    pub thrown: i32,
    pub frags: i32,
    pub stuns: i32,
    pub refused: i32,
    pub walks: i32,
    pub walked: i32,
    pub gave_up: i32,
    pub searches: i32,
    pub arcs: i32,
    pub ducks: i32,
    pub cells: i32,
}

pub const THROW_PAGE: &str = "throw";

