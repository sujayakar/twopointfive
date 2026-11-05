//! The types of `close`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseKind {
    Behind,
    Side,
    Front,
    Seat,
}
impl CloseKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CloseKind::Behind => "behind",
            CloseKind::Side => "side",
            CloseKind::Front => "front",
            CloseKind::Seat => "seat",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseId {
    Behind,
    Front,
    Left,
    Right,
}
impl CloseId {
    pub const fn as_str(self) -> &'static str {
        match self {
            CloseId::Behind => "behind",
            CloseId::Front => "front",
            CloseId::Left => "left",
            CloseId::Right => "right",
        }
    }
}

/// (TypeScript: CloseWindow, src/game/close.ts:123)
#[derive(Clone, Copy, Debug)]
pub struct CloseWindow {
    pub id: crate::close::CloseId,
    pub kind: crate::close::CloseKind,
    pub mid: f64,
    pub half: f64,
    pub cos_half: f64,
    pub reach: f64,
    pub grab: bool,
    pub strike: bool,
}
impl CloseWindow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseWindow {
            id: crate::close::CloseId::Behind,
            kind: crate::close::CloseKind::Behind,
            mid: 0.0,
            half: 0.0,
            cos_half: 0.0,
            reach: 0.0,
            grab: false,
            strike: false,
        }
    }
}
impl Default for CloseWindow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CloseOffer, src/game/close.ts:132)
#[derive(Clone, Debug)]
pub struct CloseOffer {
    pub guard: GuardId,
    pub win: Option<crate::close::CloseWindow>,
    pub dist: f64,
    pub bearing: f64,
    pub in_reach: bool,
    pub clear: bool,
    pub grab: bool,
    pub strike: bool,
    pub can: bool,
    pub why: String,
}
impl CloseOffer {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseOffer {
            guard: GuardId(0),
            win: None,
            dist: 0.0,
            bearing: 0.0,
            in_reach: false,
            clear: false,
            grab: false,
            strike: false,
            can: false,
            why: String::new(),
        }
    }
}
impl Default for CloseOffer { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Seat, src/game/close.ts:144)
#[derive(Clone, Copy, Debug)]
pub struct Seat {
    pub at: V3,
    pub face: f64,
    pub hold: bool,
    pub down: bool,
}
impl Seat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Seat {
            at: [0.0; 3],
            face: 0.0,
            hold: false,
            down: false,
        }
    }
}
impl Default for Seat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/close.ts:145)
#[derive(Clone, Debug)]
pub struct State {
    pub t: f64,
    pub best: Option<crate::close::CloseOffer>,
    pub mode: Option<GuardId>,
    pub mode_d: f64,
    pub rmb_t: f64,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            t: 0.0,
            best: None,
            mode: None,
            mode_d: 0.0,
            rmb_t: 0.0,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CloseMark, src/game/close.ts:386)
#[derive(Clone, Debug)]
pub struct CloseMark {
    pub guard: GuardId,
    pub id: crate::close::CloseId,
    pub kind: crate::close::CloseKind,
    pub pts: Vec<V3>,
    pub label_at: V3,
    pub label: String,
    pub state: crate::close::CloseMarkState,
    pub alpha: f64,
}
impl CloseMark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseMark {
            guard: GuardId(0),
            id: crate::close::CloseId::Behind,
            kind: crate::close::CloseKind::Behind,
            pts: Vec::new(),
            label_at: [0.0; 3],
            label: String::new(),
            state: crate::close::CloseMarkState::Shut,
            alpha: 0.0,
        }
    }
}
impl Default for CloseMark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseMarkState {
    Shut,
    Open,
    In,
}
impl CloseMarkState {
    pub const fn as_str(self) -> &'static str {
        match self {
            CloseMarkState::Shut => "shut",
            CloseMarkState::Open => "open",
            CloseMarkState::In => "in",
        }
    }
}

/// (TypeScript: inline at src/game/close.ts:140 CLOSE_LOG, src/game/close.ts:140)
#[derive(Clone, Debug)]
pub struct CloseModCLOSELOGItem {
    pub t: f64,
    pub who: String,
    pub what: crate::close::CloseModCLOSELOGItemWhat,
    pub kind: Option<crate::close::CloseKind>,
    pub bearing: f64,
    pub dist: f64,
}
impl CloseModCLOSELOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseModCLOSELOGItem {
            t: 0.0,
            who: String::new(),
            what: crate::close::CloseModCLOSELOGItemWhat::Grab,
            kind: None,
            bearing: 0.0,
            dist: 0.0,
        }
    }
}
impl Default for CloseModCLOSELOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseModCLOSELOGItemWhat {
    Grab,
    Seat,
    Take,
    Unseat,
}
impl CloseModCLOSELOGItemWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            CloseModCLOSELOGItemWhat::Grab => "grab",
            CloseModCLOSELOGItemWhat::Seat => "seat",
            CloseModCLOSELOGItemWhat::Take => "take",
            CloseModCLOSELOGItemWhat::Unseat => "unseat",
        }
    }
}

/// (TypeScript: inline at src/game/close.ts:446 SIT_CLIP, src/game/close.ts:446)
#[derive(Clone, Debug)]
pub struct SITCLIP {
    pub clip: String,
    pub loop_: bool,
    pub rate: f64,
    pub in_: f64,
    pub out: f64,
}
impl SITCLIP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SITCLIP {
            clip: String::new(),
            loop_: false,
            rate: 0.0,
            in_: 0.0,
            out: 0.0,
        }
    }
}
impl Default for SITCLIP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NoteWhat {
    Grab,
    Seat,
    Take,
    Unseat,
}
impl NoteWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            NoteWhat::Grab => "grab",
            NoteWhat::Seat => "seat",
            NoteWhat::Take => "take",
            NoteWhat::Unseat => "unseat",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseRmb {
    Press,
    Tap,
    Hold,
}
impl CloseRmb {
    pub const fn as_str(self) -> &'static str {
        match self {
            CloseRmb::Press => "press",
            CloseRmb::Tap => "tap",
            CloseRmb::Hold => "hold",
        }
    }
}

/// (TypeScript: inline at src/game/close.ts:253 closeRows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct CloseRowsRowOver {
    pub kind: Option<crate::close::CloseRowsRowOverKind>,
    pub pos: Option<V3>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub in_reach: Option<bool>,
    pub off: Option<bool>,
    pub progress: Option<f64>,
    pub target: Option<FixtureId>,
    pub door: Option<DoorId>,
    pub guard: Option<GuardId>,
    pub verb: Option<crate::game::Verb>,
    pub running: Option<bool>,
    pub close: Option<crate::close::CloseKind>,
    pub past: Option<bool>,
    pub framed: Option<bool>,
}
impl CloseRowsRowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseRowsRowOver {
            kind: None,
            pos: None,
            line1: None,
            line2: None,
            in_reach: None,
            off: None,
            progress: None,
            target: None,
            door: None,
            guard: None,
            verb: None,
            running: None,
            close: None,
            past: None,
            framed: None,
        }
    }
}
impl Default for CloseRowsRowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseRowsRowOverKind {
    Cam,
    Guard,
    Breaker,
    Door,
    Light,
    Body,
    Objective,
    Held,
    Wall,
    Switch,
    Charge,
    Cancel,
}
impl CloseRowsRowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CloseRowsRowOverKind::Cam => "cam",
            CloseRowsRowOverKind::Guard => "guard",
            CloseRowsRowOverKind::Breaker => "breaker",
            CloseRowsRowOverKind::Door => "door",
            CloseRowsRowOverKind::Light => "light",
            CloseRowsRowOverKind::Body => "body",
            CloseRowsRowOverKind::Objective => "objective",
            CloseRowsRowOverKind::Held => "held",
            CloseRowsRowOverKind::Wall => "wall",
            CloseRowsRowOverKind::Switch => "switch",
            CloseRowsRowOverKind::Charge => "charge",
            CloseRowsRowOverKind::Cancel => "cancel",
        }
    }
}

/// (TypeScript: inline at src/game/close.ts:296 closeCivGate ->, src/game/close.ts:296)
#[derive(Clone, Copy, Debug)]
pub struct CloseCivGate {
    pub behind: bool,
    pub near: bool,
    pub kind: crate::close::CloseKind,
}
impl CloseCivGate {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseCivGate {
            behind: false,
            near: false,
            kind: crate::close::CloseKind::Behind,
        }
    }
}
impl Default for CloseCivGate { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/close.ts:307 closeReady ->, src/game/close.ts:307)
#[derive(Clone, Copy, Debug)]
pub struct CloseReady {
    pub kind: crate::character::ReadyKind,
    pub at: V3,
}
impl CloseReady {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseReady {
            kind: crate::character::ReadyKind::Grab,
            at: [0.0; 3],
        }
    }
}
impl Default for CloseReady { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/close.ts:375 closeStation ->, src/game/close.ts:375)
#[derive(Clone, Copy, Debug)]
pub struct CloseStation {
    pub pos: V3,
    pub yaw: f64,
}
impl CloseStation {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseStation {
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for CloseStation { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseTookWhat {
    Grab,
    Take,
}
impl CloseTookWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            CloseTookWhat::Grab => "grab",
            CloseTookWhat::Take => "take",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct CloseMod {
    /// (src/game/close.ts:140 CLOSE_LOG)
    pub close_log: Vec<crate::close::CloseModCLOSELOGItem>,
    /// (src/game/close.ts:146 STATE)
    pub state: JsMap<PlayerId, crate::close::State>,
    /// (src/game/close.ts:148 SEATED)
    pub seated: Option<JsMap<GuardId, SeatId>>,
}
impl CloseMod {
    pub fn blank() -> Self {
        CloseMod {
            close_log: Vec::new(),
            state: JsMap::new(),
            seated: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CloseStats {
    pub takes: [i32; 4],
    pub grabs: [i32; 4],
    pub blows: [i32; 2],
    pub ready_frames: i32,
    pub mode_frames: i32,
    pub dark_passes: i32,
    pub seats: i32,
    pub unseats: i32,
}

/// (src/game/close.ts:40 DEGR)
pub const DEGR: f64 = 0.017453292519943295;

/// (src/game/close.ts:115 CLOSE_DEFAULTS: a table nothing writes)
pub fn close_defaults() -> crate::knobs::CloseK { crate::knobs::page_close() }

/// (src/game/close.ts:117 BEHIND_COS)
pub const BEHIND_COS: f64 = -0.25;

/// (src/game/close.ts:446 SIT_CLIP: a table nothing writes)
pub fn sit_clip() -> crate::close::SITCLIP { crate::close::SITCLIP { clip: String::from("Sitting_Idle_Loop"), loop_: true, rate: 1.0, in_: 0.7, out: 0.8 } }
