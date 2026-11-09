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
pub struct Plan {
    pub room: RoomfightRoomId,
    pub id: String,
    pub anchor: V3,
    pub inside: Vec<RoomfightDefId>,
    pub come: Vec<crate::laststand::PlanComeItem>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PlanComeItem {
    pub d: RoomfightDefId,
    pub spot: V3,
    pub l: f64,
}

#[derive(Clone, Debug, Default)]
pub struct LINESValue {
    pub cls: f64,
    pub say: Vec<String>,
    pub now: Option<Vec<String>>,
    pub guide: Option<Vec<String>>,
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

#[derive(Clone, Debug, Default)]
pub struct NetAtFRet {
    pub vars: JsMap<String, String>,
    pub then: Option<crate::laststand::LaterThen>,
}

#[derive(Clone, Debug, Default)]
pub struct TeamKnown {
    pub pts: Vec<V3>,
    pub last: V3,
}

#[derive(Clone, Debug, Default)]
pub struct StandRoom {
    pub room: RoomfightRoomId,
    pub id: String,
}

#[derive(Clone, Debug, Default)]
pub struct HomeRoom {
    pub room: RoomfightRoomId,
    pub id: String,
    pub anchor: V3,
}

#[derive(Clone, Debug, Default)]
pub struct LastStandReport {
    pub on: String,
    pub stats: JsMap<String, f64>,
    pub room: Option<String>,
    pub in_: Vec<String>,
    pub out: Vec<String>,
    pub moving: Vec<String>,
    pub known: Vec<String>,
    pub named: Vec<String>,
    pub said: Vec<crate::laststand::LastStandSaid>,
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

/// (src/game/laststand.ts:91 LASTSTAND_DEFAULTS: a table nothing writes)
pub fn laststand_defaults() -> crate::knobs::LaststandK { crate::knobs::LaststandK::page() }

/// (src/game/laststand.ts:92 LS)
pub const LS: &str = "twopointsix.laststand";

/// (src/game/laststand.ts:95 KEPT: a table nothing writes)
pub fn kept() -> [String; 7] { [String::from("regroup"), String::from("callOut"), String::from("places"), String::from("few"), String::from("every"), String::from("again"), String::from("maxM")] }

/// (src/game/laststand.ts:108 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, crate::laststand::LINESValue> { { let mut t = JsMap::new(); t.set(String::from("check1"), crate::laststand::LINESValue { cls: 4.0, say: alloc::vec![String::from("radio check — who's still up? sound off!"), String::from("all posts, radio check — who's left? sound off!")], now: None, guide: None }); t.set(String::from("checkN"), crate::laststand::LINESValue { cls: 4.0, say: alloc::vec![String::from("radio check — sound off, who's still up?"), String::from("count off — who's still with us?")], now: None, guide: None }); t.set(String::from("checkStand"), crate::laststand::LINESValue { cls: 4.0, say: alloc::vec![String::from("anyone else out there? we're holed up in {the} — get to us!"), String::from("any post still up — we're in {the}. sound off!")], now: None, guide: None }); t.set(String::from("checkLoneAt"), crate::laststand::LINESValue { cls: 4.0, say: alloc::vec![String::from("anybody? anybody still up? I'm still {at}."), String::from("if anyone's left — I'm still {at}. sound off!")], now: None, guide: None }); t.set(String::from("ansAt"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("still up — {at}."), String::from("here — holding {the}."), String::from("still here — {at}.")], now: None, guide: None }); t.set(String::from("ansFreeAt"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("still up — after them, {at}."), String::from("here — on them, {at}.")], now: None, guide: Some(alloc::vec![String::from("still up — going after them, {at}."), String::from("here — I'm after them, {at}.")]) }); t.set(String::from("fewAt"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("that's only {n} of us — I'm {at}. stay on your doors!"), String::from("{n} of us left — I'm {at}. nobody opens a door!")], now: Some(alloc::vec![String::from("that's only {n} of us — I'm {at}."), String::from("{n} of us left — I'm {at}.")]), guide: None }); t.set(String::from("lastAt"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("nobody's answering — if anyone can hear me, I'm {at}!"), String::from("no answer… it's just me. if anyone's left, I'm {at}!")], now: None, guide: None }); t.set(String::from("movedAt"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("moved — I'm {at} now."), String::from("I've moved — {at} now.")], now: None, guide: None }); t.set(String::from("movedFree"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("I'm out of {the} — going after them."), String::from("left {the} — I'm on them.")], now: None, guide: None }); t.set(String::from("lastGo"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("nobody's answering — if anyone can hear me, I'm going back to {the}!"), String::from("no answer… it's just me. I'll be in {the} if anyone's left!")], now: None, guide: None }); t.set(String::from("order"), crate::laststand::LINESValue { cls: 1.0, say: alloc::vec![String::from("that's {n} of us left — everybody to {the}, we hold it there!"), String::from("only {n} of us — fall back on {the}. we make our stand there!")], now: None, guide: None }); t.set(String::from("orderFrom"), crate::laststand::LINESValue { cls: 1.0, say: alloc::vec![String::from("that's {n} of us left — get together in {the} and hold it. I can't reach you: I'm keeping {at}."), String::from("only {n} of us — make the stand in {the}. no way through for me, I'm staying {at}.")], now: None, guide: None }); t.set(String::from("orderStay"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("that's {n} of us and no way to each other — I'm {at}. block your doors and hold where you are!")], now: None, guide: None }); t.set(String::from("orderShut"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("that's {n} of us left — I'm {at}. stay shut in where you are, nobody opens a door!")], now: None, guide: None }); t.set(String::from("orderOut"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("that's {n} of us and no way to each other — I'm {at}. hold where you are and watch every way in!")], now: None, guide: None }); t.set(String::from("go"), crate::laststand::LINESValue { cls: 1.0, say: alloc::vec![String::from("moving to {the} — don't shoot me coming in!"), String::from("on my way to {the}!")], now: None, guide: None }); t.set(String::from("cant"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("can't get to you — digging in {at}."), String::from("no way through to you. I'll hold {the}.")], now: None, guide: None }); t.set(String::from("in"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("I'm in — {n} of us in {the}."), String::from("made it. {n} of us in {the}.")], now: None, guide: None }); t.set(String::from("short"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("can't make it — holding {at}."), String::from("not getting through. holding {at}.")], now: None, guide: None }); t.set(String::from("set"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("{the} is shut — let them come."), String::from("door's blocked. we're set in {the}.")], now: None, guide: None }); t.set(String::from("setOpen"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("we're set in {the}. they'll have to come through the door."), String::from("in position in {the} — guns on the door.")], now: None, guide: None }); t.set(String::from("setLone"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("I'm shut in {the}. if anyone's left — I'm not coming out."), String::from("door's blocked, {the}. I'm staying put.")], now: None, guide: None }); t.set(String::from("setLoneOpen"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("I'm set in {the}. if anyone's left — I'm not coming out."), String::from("gun's on the door, {the}. I'm staying put.")], now: None, guide: None }); t } }
