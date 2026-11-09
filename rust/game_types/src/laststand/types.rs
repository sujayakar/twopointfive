//! The types of `laststand`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what a line makes true for the HUD as it is said (laststand.ts net `then`); st and g are the caller's. NameAtUnlessFree reads d's mode when the line is said; NameAt's w was settled when the line was made
#[derive(Clone, Debug, ToBytes)]
pub enum LaterThen {
    MovedFree { d: RoomfightDefId },
    MovedAt { d: RoomfightDefId, w: crate::laststand::Spoken },
    NameAt { d: RoomfightDefId, w: Option<crate::laststand::Spoken> },
    NameAtUnlessFree { d: RoomfightDefId, w: crate::laststand::Spoken },
}
/// the wording a line that says where its man stands fills in, asked with spoken(g, d's position) when made and again as it is said (laststand.ts netAt `f`, Later.redo)
#[derive(Clone, Debug, ToBytes)]
pub enum LaterRedo {
    MovedAt { d: RoomfightDefId },
    CheckLone { d: RoomfightDefId },
    Answer { d: RoomfightDefId, free: bool },
    Tell { d: RoomfightDefId, n: i32, free: bool },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum LineKind {
    #[default] Set,
    Go,
    Order,
    Short,
    LastAt,
    In,
    Check1,
    CheckN,
    CheckStand,
    CheckLoneAt,
    AnsAt,
    AnsFreeAt,
    FewAt,
    LastGo,
    MovedAt,
    MovedFree,
    OrderFrom,
    OrderStay,
    OrderShut,
    OrderOut,
    Cant,
    SetOpen,
    SetLone,
    SetLoneOpen,
}
impl LineKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineKind::Set => "set",
            LineKind::Go => "go",
            LineKind::Order => "order",
            LineKind::Short => "short",
            LineKind::LastAt => "lastAt",
            LineKind::In => "in",
            LineKind::Check1 => "check1",
            LineKind::CheckN => "checkN",
            LineKind::CheckStand => "checkStand",
            LineKind::CheckLoneAt => "checkLoneAt",
            LineKind::AnsAt => "ansAt",
            LineKind::AnsFreeAt => "ansFreeAt",
            LineKind::FewAt => "fewAt",
            LineKind::LastGo => "lastGo",
            LineKind::MovedAt => "movedAt",
            LineKind::MovedFree => "movedFree",
            LineKind::OrderFrom => "orderFrom",
            LineKind::OrderStay => "orderStay",
            LineKind::OrderShut => "orderShut",
            LineKind::OrderOut => "orderOut",
            LineKind::Cant => "cant",
            LineKind::SetOpen => "setOpen",
            LineKind::SetLone => "setLone",
            LineKind::SetLoneOpen => "setLoneOpen",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Check {
    pub caller: RoomfightDefId,
    pub t: f64,
    pub close_at: f64,
    pub due: Vec<crate::laststand::CheckDueItem>,
    pub got: Vec<RoomfightDefId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CheckDueItem {
    pub d: RoomfightDefId,
    pub at: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LastStandSaid {
    pub t: f64,
    pub who: String,
    pub kind: String,
    pub text: String,
    pub at: [f64; 2],
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Later {
    pub at: f64,
    pub t0: f64,
    pub gd: GuardId,
    pub kind: crate::laststand::LineKind,
    pub text: String,
    pub cls: f64,
    pub then: Option<crate::laststand::LaterThen>,
    pub tpl: String,
    pub redo: Option<crate::laststand::LaterRedo>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub die: crate::math_dice::Die,
    pub next_at: f64,
    pub check: Option<crate::laststand::Check>,
    pub known: JsSet<RoomfightDefId>,
    pub named: JsMap<RoomfightDefId, String>,
    pub spots: JsMap<RoomfightDefId, crate::laststand::Spot>,
    pub at: JsMap<RoomfightDefId, crate::laststand::Said>,
    pub lone_said: bool,
    pub few_n: f64,
    pub told: JsSet<RoomfightDefId>,
    pub said: Vec<crate::laststand::LastStandSaid>,
    pub stats: crate::laststand::StateStats,
    pub stale: bool,
    pub post: JsMap<GuardId, String>,
    pub key: String,
    pub rot: JsMap<crate::laststand::LineKind, f64>,
    pub cls_at: Vec<f64>,
    pub later: Vec<crate::laststand::Later>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Spot {
    pub name: String,
    pub pos: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Said {
    pub key: String,
    pub the: String,
    pub off_t: f64,
    pub still_at: V3,
    pub still_t: f64,
    pub said_t: f64,
}

/// what their net has named that still stands: the rooms by id, and the named places that are no room
#[derive(Clone, Debug, Default)]
pub struct Hint {
    pub rooms: JsSet<String>,
    pub spots: Vec<crate::laststand::Spot>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateStats {
    pub t0: f64,
    pub up0: f64,
    pub checks: f64,
    pub answers: f64,
    pub silent: f64,
    pub last_man: f64,
    pub stands: f64,
    pub gathered: f64,
    pub sent: f64,
    pub arrived: f64,
    pub short: f64,
    pub met: f64,
    pub died_on_way: f64,
    pub dropped: f64,
    pub quiet: f64,
    pub no_way: f64,
    pub moved: f64,
    pub dig_orders: f64,
    pub stand_t: f64,
    pub set_t: f64,
    pub set_men: f64,
    pub thumps: f64,
    pub lines: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Where {
    pub id: Option<String>,
    pub the: String,
    pub at: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Spoken {
    pub id: Option<String>,
    pub the: String,
    pub at: String,
    pub spot: Option<crate::laststand::Spot>,
}

#[derive(Clone, Debug, Default)]
pub struct NetRedoRet {
    pub vars: JsMap<String, String>,
    pub then: Option<crate::laststand::LaterThen>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct LaststandMod {
    /// (src/game/laststand.ts:170 STATES)
    pub states: Option<crate::laststand::State>,
}
impl LaststandMod {
    pub fn blank() -> Self {
        LaststandMod {
            states: None,
        }
    }
}

