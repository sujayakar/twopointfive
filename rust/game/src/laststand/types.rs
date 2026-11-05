//! The types of `laststand`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what a line makes true for the HUD as it is said (laststand.ts net `then`); st and g are the caller's. NameAtUnlessFree reads d's mode when the line is said; NameAt's w was settled when the line was made
#[derive(Clone, Debug)]
pub enum LaterThen {
    MovedFree { d: RoomfightDefId },
    MovedAt { d: RoomfightDefId, w: crate::laststand::Spoken },
    NameAt { d: RoomfightDefId, w: Option<crate::laststand::Spoken> },
    NameAtUnlessFree { d: RoomfightDefId, w: crate::laststand::Spoken },
}
/// the wording a line that says where its man stands fills in, asked with spoken(g, d's position) when made and again as it is said (laststand.ts netAt `f`, Later.redo)
#[derive(Clone, Debug)]
pub enum LaterRedo {
    MovedAt { d: RoomfightDefId },
    CheckLone { d: RoomfightDefId },
    Answer { d: RoomfightDefId, free: bool },
    Tell { d: RoomfightDefId, n: i32, free: bool },
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LineKind {
    Set,
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

/// (TypeScript: Mover, src/game/laststand.ts:137)
#[derive(Clone, Copy, Debug)]
pub struct Mover {
    pub d: RoomfightDefId,
    pub phase: crate::laststand::MoverPhase,
    pub go_at: f64,
    pub spot: V3,
    pub tries: f64,
    pub step_t: f64,
    pub t0: f64,
    pub quiet: bool,
}
impl Mover {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Mover {
            d: RoomfightDefId(0),
            phase: crate::laststand::MoverPhase::Run,
            go_at: 0.0,
            spot: [0.0; 3],
            tries: 0.0,
            step_t: 0.0,
            t0: 0.0,
            quiet: false,
        }
    }
}
impl Default for Mover { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MoverPhase {
    Run,
    Beat,
}
impl MoverPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            MoverPhase::Run => "run",
            MoverPhase::Beat => "beat",
        }
    }
}

/// (TypeScript: Stand, src/game/laststand.ts:138)
#[derive(Clone, Debug)]
pub struct Stand {
    pub room: Option<RoomfightRoomId>,
    pub id: Option<String>,
    pub the: String,
    pub anchor: V3,
    pub t: f64,
    pub in_: JsSet<RoomfightDefId>,
    pub out: JsSet<RoomfightDefId>,
    pub movers: Vec<crate::laststand::Mover>,
    pub set_said: bool,
    pub set_t: f64,
    pub thump_at: f64,
    pub look_t: f64,
    pub orders: JsMap<RoomfightDefId, f64>,
    pub homed: JsSet<RoomfightDefId>,
}
impl Stand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Stand {
            room: None,
            id: None,
            the: String::new(),
            anchor: [0.0; 3],
            t: 0.0,
            in_: JsSet::new(),
            out: JsSet::new(),
            movers: Vec::new(),
            set_said: false,
            set_t: 0.0,
            thump_at: 0.0,
            look_t: 0.0,
            orders: JsMap::new(),
            homed: JsSet::new(),
        }
    }
}
impl Default for Stand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Check, src/game/laststand.ts:143)
#[derive(Clone, Debug)]
pub struct Check {
    pub caller: RoomfightDefId,
    pub t: f64,
    pub close_at: f64,
    pub due: Vec<crate::laststand::CheckDueItem>,
    pub got: Vec<RoomfightDefId>,
}
impl Check {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Check {
            caller: RoomfightDefId(0),
            t: 0.0,
            close_at: 0.0,
            due: Vec::new(),
            got: Vec::new(),
        }
    }
}
impl Default for Check { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:143 Check.due, src/game/laststand.ts:143)
#[derive(Clone, Copy, Debug)]
pub struct CheckDueItem {
    pub d: RoomfightDefId,
    pub at: f64,
}
impl CheckDueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckDueItem {
            d: RoomfightDefId(0),
            at: 0.0,
        }
    }
}
impl Default for CheckDueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LastStandSaid, src/game/laststand.ts:144)
#[derive(Clone, Debug)]
pub struct LastStandSaid {
    pub t: f64,
    pub who: String,
    pub kind: String,
    pub text: String,
    pub at: [f64; 2],
}
impl LastStandSaid {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LastStandSaid {
            t: 0.0,
            who: String::new(),
            kind: String::new(),
            text: String::new(),
            at: [0.0; 2],
        }
    }
}
impl Default for LastStandSaid { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Later, src/game/laststand.ts:153)
#[derive(Clone, Debug)]
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
impl Later {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Later {
            at: 0.0,
            t0: 0.0,
            gd: GuardId(0),
            kind: crate::laststand::LineKind::Set,
            text: String::new(),
            cls: 0.0,
            then: None,
            tpl: String::new(),
            redo: None,
        }
    }
}
impl Default for Later { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/laststand.ts:154)
#[derive(Clone, Debug)]
pub struct State {
    pub die: crate::math_dice::Die,
    pub next_at: f64,
    pub check: Option<crate::laststand::Check>,
    pub known: JsSet<RoomfightDefId>,
    pub named: JsMap<RoomfightDefId, String>,
    pub stand: Option<crate::laststand::Stand>,
    pub spots: JsMap<RoomfightDefId, crate::laststand::Spot>,
    pub at: JsMap<RoomfightDefId, crate::laststand::Said>,
    pub lone_said: bool,
    pub few_n: f64,
    pub told: JsSet<RoomfightDefId>,
    pub hint: Option<crate::laststand::Hint>,
    pub said: Vec<crate::laststand::LastStandSaid>,
    pub stats: crate::laststand::StateStats,
    pub stale: bool,
    pub post: JsMap<GuardId, String>,
    pub key: String,
    pub rot: JsMap<crate::laststand::LineKind, f64>,
    pub cls_at: Vec<f64>,
    pub later: Vec<crate::laststand::Later>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            die: crate::math_dice::Die::zeroed(),
            next_at: 0.0,
            check: None,
            known: JsSet::new(),
            named: JsMap::new(),
            stand: None,
            spots: JsMap::new(),
            at: JsMap::new(),
            lone_said: false,
            few_n: 0.0,
            told: JsSet::new(),
            hint: None,
            said: Vec::new(),
            stats: crate::laststand::StateStats::zeroed(),
            stale: false,
            post: JsMap::new(),
            key: String::new(),
            rot: JsMap::new(),
            cls_at: Vec::new(),
            later: Vec::new(),
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Spot, src/game/laststand.ts:164)
#[derive(Clone, Debug)]
pub struct Spot {
    pub name: String,
    pub pos: V3,
}
impl Spot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Spot {
            name: String::new(),
            pos: [0.0; 3],
        }
    }
}
impl Default for Spot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Said, src/game/laststand.ts:167)
#[derive(Clone, Debug)]
pub struct Said {
    pub key: String,
    pub the: String,
    pub off_t: f64,
    pub still_at: V3,
    pub still_t: f64,
    pub said_t: f64,
}
impl Said {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Said {
            key: String::new(),
            the: String::new(),
            off_t: 0.0,
            still_at: [0.0; 3],
            still_t: 0.0,
            said_t: 0.0,
        }
    }
}
impl Default for Said { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Hint, src/game/laststand.ts:168)
#[derive(Clone, Debug)]
pub struct Hint {
    pub key: String,
    pub rooms: JsSet<String>,
    pub spots: Vec<crate::laststand::Spot>,
}
impl Hint {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Hint {
            key: String::new(),
            rooms: JsSet::new(),
            spots: Vec::new(),
        }
    }
}
impl Default for Hint { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:158 State.stats, src/game/laststand.ts:145)
#[derive(Clone, Copy, Debug)]
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
impl StateStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateStats {
            t0: 0.0,
            up0: 0.0,
            checks: 0.0,
            answers: 0.0,
            silent: 0.0,
            last_man: 0.0,
            stands: 0.0,
            gathered: 0.0,
            sent: 0.0,
            arrived: 0.0,
            short: 0.0,
            met: 0.0,
            died_on_way: 0.0,
            dropped: 0.0,
            quiet: 0.0,
            no_way: 0.0,
            moved: 0.0,
            dig_orders: 0.0,
            stand_t: 0.0,
            set_t: 0.0,
            set_men: 0.0,
            thumps: 0.0,
            lines: 0.0,
        }
    }
}
impl Default for StateStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Plan, src/game/laststand.ts:326)
#[derive(Clone, Debug)]
pub struct Plan {
    pub room: RoomfightRoomId,
    pub id: String,
    pub anchor: V3,
    pub inside: Vec<RoomfightDefId>,
    pub come: Vec<crate::laststand::PlanComeItem>,
}
impl Plan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Plan {
            room: RoomfightRoomId(0),
            id: String::new(),
            anchor: [0.0; 3],
            inside: Vec::new(),
            come: Vec::new(),
        }
    }
}
impl Default for Plan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:326 Plan.come, src/game/laststand.ts:326)
#[derive(Clone, Copy, Debug)]
pub struct PlanComeItem {
    pub d: RoomfightDefId,
    pub spot: V3,
    pub l: f64,
}
impl PlanComeItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlanComeItem {
            d: RoomfightDefId(0),
            spot: [0.0; 3],
            l: 0.0,
        }
    }
}
impl Default for PlanComeItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:107 LINES, src/game/laststand.ts:107)
#[derive(Clone, Debug)]
pub struct LINESValue {
    pub cls: f64,
    pub say: Vec<String>,
    pub now: Option<Vec<String>>,
    pub guide: Option<Vec<String>>,
}
impl LINESValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINESValue {
            cls: 0.0,
            say: Vec::new(),
            now: None,
            guide: None,
        }
    }
}
impl Default for LINESValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:187 where ->, src/game/laststand.ts:187)
#[derive(Clone, Debug)]
pub struct Where {
    pub id: Option<String>,
    pub the: String,
    pub at: String,
}
impl Where {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Where {
            id: None,
            the: String::new(),
            at: String::new(),
        }
    }
}
impl Default for Where { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:200 spoken ->, src/game/laststand.ts:200)
#[derive(Clone, Debug)]
pub struct Spoken {
    pub id: Option<String>,
    pub the: String,
    pub at: String,
    pub spot: Option<crate::laststand::Spot>,
}
impl Spoken {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Spoken {
            id: None,
            the: String::new(),
            at: String::new(),
            spot: None,
        }
    }
}
impl Default for Spoken { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:218 net(redo), src/game/laststand.ts:153)
#[derive(Clone, Debug)]
pub struct NetRedoRet {
    pub vars: JsMap<String, String>,
    pub then: Option<crate::laststand::LaterThen>,
}
impl NetRedoRet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetRedoRet {
            vars: JsMap::new(),
            then: None,
        }
    }
}
impl Default for NetRedoRet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:226 netAt(f), src/game/laststand.ts:226)
#[derive(Clone, Debug)]
pub struct NetAtFRet {
    pub vars: JsMap<String, String>,
    pub then: Option<crate::laststand::LaterThen>,
}
impl NetAtFRet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetAtFRet {
            vars: JsMap::new(),
            then: None,
        }
    }
}
impl Default for NetAtFRet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:270 teamKnown ->, src/game/laststand.ts:270)
#[derive(Clone, Debug)]
pub struct TeamKnown {
    pub pts: Vec<V3>,
    pub last: V3,
}
impl TeamKnown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TeamKnown {
            pts: Vec::new(),
            last: [0.0; 3],
        }
    }
}
impl Default for TeamKnown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:311 standRoom ->, src/game/laststand.ts:311)
#[derive(Clone, Debug)]
pub struct StandRoom {
    pub room: RoomfightRoomId,
    pub id: String,
}
impl StandRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StandRoom {
            room: RoomfightRoomId(0),
            id: String::new(),
        }
    }
}
impl Default for StandRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:318 homeRoom ->, src/game/laststand.ts:318)
#[derive(Clone, Debug)]
pub struct HomeRoom {
    pub room: RoomfightRoomId,
    pub id: String,
    pub anchor: V3,
}
impl HomeRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HomeRoom {
            room: RoomfightRoomId(0),
            id: String::new(),
            anchor: [0.0; 3],
        }
    }
}
impl Default for HomeRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:613 lastStandReport ->, src/game/laststand.ts:613)
#[derive(Clone, Debug)]
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
impl LastStandReport {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LastStandReport {
            on: String::new(),
            stats: JsMap::new(),
            room: None,
            in_: Vec::new(),
            out: Vec::new(),
            moving: Vec::new(),
            known: Vec::new(),
            named: Vec::new(),
            said: Vec::new(),
        }
    }
}
impl Default for LastStandReport { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct LaststandMod {
    /// (src/game/laststand.ts:169 STATES)
    pub states: Option<crate::laststand::State>,
}
impl LaststandMod {
    pub fn blank() -> Self {
        LaststandMod {
            states: None,
        }
    }
}

/// (src/game/laststand.ts:90 LASTSTAND_DEFAULTS: a table nothing writes)
pub fn laststand_defaults() -> crate::knobs::LaststandK { crate::knobs::page_laststand() }

/// (src/game/laststand.ts:91 LS)
pub const LS: &str = "twopointsix.laststand";

/// (src/game/laststand.ts:94 KEPT: a table nothing writes)
pub fn kept() -> [String; 7] { [String::from("regroup"), String::from("callOut"), String::from("places"), String::from("few"), String::from("every"), String::from("again"), String::from("maxM")] }

/// (src/game/laststand.ts:107 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, crate::laststand::LINESValue> { { let mut t = JsMap::new(); t.set(String::from("check1"), crate::laststand::LINESValue { cls: 4.0, say: alloc::vec![String::from("radio check — who's still up? sound off!"), String::from("all posts, radio check — who's left? sound off!")], now: None, guide: None }); t.set(String::from("checkN"), crate::laststand::LINESValue { cls: 4.0, say: alloc::vec![String::from("radio check — sound off, who's still up?"), String::from("count off — who's still with us?")], now: None, guide: None }); t.set(String::from("checkStand"), crate::laststand::LINESValue { cls: 4.0, say: alloc::vec![String::from("anyone else out there? we're holed up in {the} — get to us!"), String::from("any post still up — we're in {the}. sound off!")], now: None, guide: None }); t.set(String::from("checkLoneAt"), crate::laststand::LINESValue { cls: 4.0, say: alloc::vec![String::from("anybody? anybody still up? I'm still {at}."), String::from("if anyone's left — I'm still {at}. sound off!")], now: None, guide: None }); t.set(String::from("ansAt"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("still up — {at}."), String::from("here — holding {the}."), String::from("still here — {at}.")], now: None, guide: None }); t.set(String::from("ansFreeAt"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("still up — after them, {at}."), String::from("here — on them, {at}.")], now: None, guide: Some(alloc::vec![String::from("still up — going after them, {at}."), String::from("here — I'm after them, {at}.")]) }); t.set(String::from("fewAt"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("that's only {n} of us — I'm {at}. stay on your doors!"), String::from("{n} of us left — I'm {at}. nobody opens a door!")], now: Some(alloc::vec![String::from("that's only {n} of us — I'm {at}."), String::from("{n} of us left — I'm {at}.")]), guide: None }); t.set(String::from("lastAt"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("nobody's answering — if anyone can hear me, I'm {at}!"), String::from("no answer… it's just me. if anyone's left, I'm {at}!")], now: None, guide: None }); t.set(String::from("movedAt"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("moved — I'm {at} now."), String::from("I've moved — {at} now.")], now: None, guide: None }); t.set(String::from("movedFree"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("I'm out of {the} — going after them."), String::from("left {the} — I'm on them.")], now: None, guide: None }); t.set(String::from("lastGo"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("nobody's answering — if anyone can hear me, I'm going back to {the}!"), String::from("no answer… it's just me. I'll be in {the} if anyone's left!")], now: None, guide: None }); t.set(String::from("order"), crate::laststand::LINESValue { cls: 1.0, say: alloc::vec![String::from("that's {n} of us left — everybody to {the}, we hold it there!"), String::from("only {n} of us — fall back on {the}. we make our stand there!")], now: None, guide: None }); t.set(String::from("orderFrom"), crate::laststand::LINESValue { cls: 1.0, say: alloc::vec![String::from("that's {n} of us left — get together in {the} and hold it. I can't reach you: I'm keeping {at}."), String::from("only {n} of us — make the stand in {the}. no way through for me, I'm staying {at}.")], now: None, guide: None }); t.set(String::from("orderStay"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("that's {n} of us and no way to each other — I'm {at}. block your doors and hold where you are!")], now: None, guide: None }); t.set(String::from("orderShut"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("that's {n} of us left — I'm {at}. stay shut in where you are, nobody opens a door!")], now: None, guide: None }); t.set(String::from("orderOut"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("that's {n} of us and no way to each other — I'm {at}. hold where you are and watch every way in!")], now: None, guide: None }); t.set(String::from("go"), crate::laststand::LINESValue { cls: 1.0, say: alloc::vec![String::from("moving to {the} — don't shoot me coming in!"), String::from("on my way to {the}!")], now: None, guide: None }); t.set(String::from("cant"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("can't get to you — digging in {at}."), String::from("no way through to you. I'll hold {the}.")], now: None, guide: None }); t.set(String::from("in"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("I'm in — {n} of us in {the}."), String::from("made it. {n} of us in {the}.")], now: None, guide: None }); t.set(String::from("short"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("can't make it — holding {at}."), String::from("not getting through. holding {at}.")], now: None, guide: None }); t.set(String::from("set"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("{the} is shut — let them come."), String::from("door's blocked. we're set in {the}.")], now: None, guide: None }); t.set(String::from("setOpen"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("we're set in {the}. they'll have to come through the door."), String::from("in position in {the} — guns on the door.")], now: None, guide: None }); t.set(String::from("setLone"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("I'm shut in {the}. if anyone's left — I'm not coming out."), String::from("door's blocked, {the}. I'm staying put.")], now: None, guide: None }); t.set(String::from("setLoneOpen"), crate::laststand::LINESValue { cls: 3.0, say: alloc::vec![String::from("I'm set in {the}. if anyone's left — I'm not coming out."), String::from("gun's on the door, {the}. I'm staying put.")], now: None, guide: None }); t } }
