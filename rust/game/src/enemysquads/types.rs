//! The types of `enemysquads`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what RoundCtx.hot closed over that the record does not hold: the known places as of the build, and the doorways standing open then (enemysquads.ts roundCtx); rear is RoundCtx.rear
#[derive(Clone, Debug)]
pub enum RoundHot {
    Places { known: Vec<crate::enemysquads::Known>, ajar: Vec<V3> },
}
/// what RoundCtx.hotCell closed over: its memo of hot by nav cell, filled as a search reaches each cell and kept for as long as the record (enemysquads.ts roundCtx)
#[derive(Clone, Debug)]
pub enum RoundHotCell {
    Memo { hot_at: JsMap<crate::Num, bool> },
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SquadChoice {
    Flank,
    Barricade,
    Help,
}
impl SquadChoice {
    pub const fn as_str(self) -> &'static str {
        match self {
            SquadChoice::Flank => "flank",
            SquadChoice::Barricade => "barricade",
            SquadChoice::Help => "help",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SquadState {
    Down,
    Contact,
    Hold,
    Flank,
    Alerted,
    Barricade,
    Help,
}
impl SquadState {
    pub const fn as_str(self) -> &'static str {
        match self {
            SquadState::Down => "down",
            SquadState::Contact => "contact",
            SquadState::Hold => "hold",
            SquadState::Flank => "flank",
            SquadState::Alerted => "alerted",
            SquadState::Barricade => "barricade",
            SquadState::Help => "help",
        }
    }
}

/// (TypeScript: Fix, src/game/enemysquads.ts:245)
#[derive(Clone, Copy, Debug)]
pub struct Fix {
    pub at: V3,
    pub t: f64,
    pub ally: Option<EnemysquadsSquadId>,
    pub by: crate::enemysquads::FixBy,
}
impl Fix {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Fix {
            at: [0.0; 3],
            t: 0.0,
            ally: None,
            by: crate::enemysquads::FixBy::Ears,
        }
    }
}
impl Default for Fix { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FixBy {
    Ears,
    Net,
}
impl FixBy {
    pub const fn as_str(self) -> &'static str {
        match self {
            FixBy::Ears => "ears",
            FixBy::Net => "net",
        }
    }
}

/// (TypeScript: Sent, src/game/enemysquads.ts:246)
#[derive(Clone, Debug)]
pub struct Sent {
    pub gd: GuardId,
    pub role: crate::enemysquads::SentRole,
    pub phase: crate::enemysquads::SentPhase,
    pub hold: Option<bool>,
    pub spot: Option<V3>,
    pub door: Option<DoorId>,
    pub fix_at: V3,
    pub tries: f64,
    pub step_t: f64,
    pub t0: f64,
    pub there: Option<f64>,
    pub way: Option<Vec<V3>>,
    pub aim: Option<V3>,
    pub set_t: Option<f64>,
    pub from: V3,
    pub ally: Option<EnemysquadsSquadId>,
    pub gone_t: Option<f64>,
    pub back: Option<bool>,
}
impl Sent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Sent {
            gd: GuardId(0),
            role: crate::enemysquads::SentRole::Look,
            phase: crate::enemysquads::SentPhase::Set,
            hold: None,
            spot: None,
            door: None,
            fix_at: [0.0; 3],
            tries: 0.0,
            step_t: 0.0,
            t0: 0.0,
            there: None,
            way: None,
            aim: None,
            set_t: None,
            from: [0.0; 3],
            ally: None,
            gone_t: None,
            back: None,
        }
    }
}
impl Default for Sent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SentRole {
    Look,
    Flank,
    Help,
    Reinforce,
}
impl SentRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            SentRole::Look => "look",
            SentRole::Flank => "flank",
            SentRole::Help => "help",
            SentRole::Reinforce => "reinforce",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SentPhase {
    Set,
    Run,
    Loose,
}
impl SentPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            SentPhase::Set => "set",
            SentPhase::Run => "run",
            SentPhase::Loose => "loose",
        }
    }
}

/// (TypeScript: SquadEvent, src/game/enemysquads.ts:248)
#[derive(Clone, Debug)]
pub struct SquadEvent {
    pub t: f64,
    pub what: crate::enemysquads::SquadEventWhat,
    pub n: f64,
    pub why: String,
}
impl SquadEvent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadEvent {
            t: 0.0,
            what: crate::enemysquads::SquadEventWhat::Down,
            n: 0.0,
            why: String::new(),
        }
    }
}
impl Default for SquadEvent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SquadEventWhat {
    Down,
    Back,
    Contact,
    Lost,
    Quiet,
    Found,
    Said,
    Sweep,
    Flank,
    Glimpse,
    Barricade,
    Help,
    Reinforce,
    InPosition,
    Met,
    Rally,
    Stays,
    Follows,
    Check,
    Asks,
    Looks,
    NoSign,
    Calls,
    CutOff,
    NetCheck,
    NetQuery,
    NetAsks,
    NetLooks,
    NetHelp,
}
impl SquadEventWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            SquadEventWhat::Down => "down",
            SquadEventWhat::Back => "back",
            SquadEventWhat::Contact => "contact",
            SquadEventWhat::Lost => "lost",
            SquadEventWhat::Quiet => "quiet",
            SquadEventWhat::Found => "found",
            SquadEventWhat::Said => "said",
            SquadEventWhat::Sweep => "sweep",
            SquadEventWhat::Flank => "flank",
            SquadEventWhat::Glimpse => "glimpse",
            SquadEventWhat::Barricade => "barricade",
            SquadEventWhat::Help => "help",
            SquadEventWhat::Reinforce => "reinforce",
            SquadEventWhat::InPosition => "in position",
            SquadEventWhat::Met => "met",
            SquadEventWhat::Rally => "rally",
            SquadEventWhat::Stays => "stays",
            SquadEventWhat::Follows => "follows",
            SquadEventWhat::Check => "check",
            SquadEventWhat::Asks => "asks",
            SquadEventWhat::Looks => "looks",
            SquadEventWhat::NoSign => "no sign",
            SquadEventWhat::Calls => "calls",
            SquadEventWhat::CutOff => "cut off",
            SquadEventWhat::NetCheck => "net check",
            SquadEventWhat::NetQuery => "net query",
            SquadEventWhat::NetAsks => "net asks",
            SquadEventWhat::NetLooks => "net looks",
            SquadEventWhat::NetHelp => "net help",
        }
    }
}

/// (TypeScript: Check, src/game/enemysquads.ts:252)
#[derive(Clone, Debug)]
pub struct Check {
    pub next_at: f64,
    pub stage: crate::enemysquads::CheckStage,
    pub t0: f64,
    pub caller: Option<GuardId>,
    pub who: Option<GuardId>,
    pub silent: Vec<crate::enemysquads::Quiet>,
    pub known: JsSet<GuardId>,
    pub looked: JsSet<GuardId>,
    pub look: Vec<crate::enemysquads::Looker>,
    pub n: f64,
    pub die: crate::math_dice::Die,
    pub cut: Option<Vec<crate::enemysquads::CheckCutItem>>,
    pub beat: Option<f64>,
    pub late: Option<bool>,
    pub promised: Option<JsSet<GuardId>>,
    pub afield: Option<JsMap<GuardId, V3>>,
    pub out: Option<JsSet<GuardId>>,
    pub asked: Option<JsSet<GuardId>>,
    pub net_out: Option<JsMap<GuardId, crate::enemysquads::CheckNetOutValue>>,
    pub by_ear: bool,
    pub tick_t: f64,
}
impl Check {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Check {
            next_at: 0.0,
            stage: crate::enemysquads::CheckStage::Idle,
            t0: 0.0,
            caller: None,
            who: None,
            silent: Vec::new(),
            known: JsSet::new(),
            looked: JsSet::new(),
            look: Vec::new(),
            n: 0.0,
            die: crate::math_dice::Die::zeroed(),
            cut: None,
            beat: None,
            late: None,
            promised: None,
            afield: None,
            out: None,
            asked: None,
            net_out: None,
            by_ear: false,
            tick_t: 0.0,
        }
    }
}
impl Default for Check { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CheckStage {
    Idle,
    Called,
    Asked,
}
impl CheckStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            CheckStage::Idle => "idle",
            CheckStage::Called => "called",
            CheckStage::Asked => "asked",
        }
    }
}

/// (TypeScript: Quiet, src/game/enemysquads.ts:254)
#[derive(Clone, Debug)]
pub struct Quiet {
    pub gd: GuardId,
    pub at: V3,
    pub t: f64,
    pub heard: bool,
    pub held: bool,
    pub small0: JsMap<GuardId, f64>,
}
impl Quiet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Quiet {
            gd: GuardId(0),
            at: [0.0; 3],
            t: 0.0,
            heard: false,
            held: false,
            small0: JsMap::new(),
        }
    }
}
impl Default for Quiet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Looker, src/game/enemysquads.ts:256)
#[derive(Clone, Debug)]
pub struct Looker {
    pub gd: GuardId,
    pub who: GuardId,
    pub spot: V3,
    pub from: V3,
    pub back: bool,
    pub tries: f64,
    pub step_t: f64,
    pub t0: f64,
    pub news0: f64,
    pub look_t: Option<f64>,
    pub calm: crate::enemysquads::Calm,
    pub about: Option<EnemysquadsSquadId>,
    pub all: Option<bool>,
    pub stops: Option<Vec<V3>>,
    pub n: Option<f64>,
}
impl Looker {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Looker {
            gd: GuardId(0),
            who: GuardId(0),
            spot: [0.0; 3],
            from: [0.0; 3],
            back: false,
            tries: 0.0,
            step_t: 0.0,
            t0: 0.0,
            news0: 0.0,
            look_t: None,
            calm: crate::enemysquads::Calm::zeroed(),
            about: None,
            all: None,
            stops: None,
            n: None,
        }
    }
}
impl Default for Looker { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Calm, src/game/enemysquads.ts:261)
#[derive(Clone, Debug)]
pub struct Calm {
    pub state: crate::game::GuardState,
    pub awareness: f64,
    pub hold: bool,
    pub pinned: bool,
    pub station: V3,
    pub home: V3,
    pub room: Option<RoomfightRoomId>,
    pub ev: Vec<f64>,
    pub watch: f64,
    pub want: f64,
    pub pt: V3,
}
impl Calm {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Calm {
            state: crate::game::GuardState::Patrol,
            awareness: 0.0,
            hold: false,
            pinned: false,
            station: [0.0; 3],
            home: [0.0; 3],
            room: None,
            ev: Vec::new(),
            watch: 0.0,
            want: 0.0,
            pt: [0.0; 3],
        }
    }
}
impl Default for Calm { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:252 Check.cut, src/game/enemysquads.ts:252)
#[derive(Clone, Copy, Debug)]
pub struct CheckCutItem {
    pub who: GuardId,
    pub at: f64,
}
impl CheckCutItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckCutItem {
            who: GuardId(0),
            at: 0.0,
        }
    }
}
impl Default for CheckCutItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:252 Check.netOut, src/game/enemysquads.ts:252)
#[derive(Clone, Copy, Debug)]
pub struct CheckNetOutValue {
    pub about: EnemysquadsSquadId,
    pub at: V3,
    pub t: f64,
}
impl CheckNetOutValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckNetOutValue {
            about: EnemysquadsSquadId(0),
            at: [0.0; 3],
            t: 0.0,
        }
    }
}
impl Default for CheckNetOutValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Call, src/game/enemysquads.ts:260)
#[derive(Clone, Debug)]
pub struct Call {
    pub by: GuardId,
    pub t0: f64,
    pub end: f64,
    pub at: V3,
    pub room: Option<crate::scene_scenarios::StructureRoom>,
    pub body: Option<GuardId>,
    pub eyes: bool,
}
impl Call {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Call {
            by: GuardId(0),
            t0: 0.0,
            end: 0.0,
            at: [0.0; 3],
            room: None,
            body: None,
            eyes: false,
        }
    }
}
impl Default for Call { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Squad, src/game/enemysquads.ts:262)
#[derive(Clone, Debug)]
pub struct Squad {
    pub def: crate::scene_scenarios::SquadData,
    pub name: String,
    pub rooms: Vec<crate::scene_scenarios::StructureRoom>,
    pub men: Vec<GuardId>,
    pub barricade: JsSet<String>,
    pub state: crate::enemysquads::SquadState,
    pub state0: Option<crate::enemysquads::SquadChoice>,
    pub fix: Option<crate::enemysquads::Fix>,
    pub news_t: f64,
    pub decide_at: f64,
    pub decided_at: f64,
    pub contact_t: f64,
    pub called_t: f64,
    pub called_room: Option<crate::scene_scenarios::StructureRoom>,
    pub fallen: JsSet<GuardId>,
    pub ready: bool,
    pub own_said: Option<f64>,
    pub stayed: Option<JsSet<String>>,
    pub hit_room: Option<crate::scene_scenarios::StructureRoom>,
    pub rc: Option<crate::enemysquads::Check>,
    pub calls: Option<Vec<crate::enemysquads::Call>>,
    pub call_die: Option<crate::math_dice::Die>,
    pub hung: Option<Vec<crate::enemysquads::SquadHungItem>>,
    pub trouble: Option<f64>,
    pub wary: Option<bool>,
    pub ask_die: Option<crate::math_dice::Die>,
    pub sent: Vec<crate::enemysquads::Sent>,
    pub went: Vec<crate::enemysquads::Sent>,
    pub due: Vec<crate::enemysquads::SquadDueItem>,
    pub events: Vec<crate::enemysquads::SquadEvent>,
}
impl Squad {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Squad {
            def: crate::scene_scenarios::SquadData::zeroed(),
            name: String::new(),
            rooms: Vec::new(),
            men: Vec::new(),
            barricade: JsSet::new(),
            state: crate::enemysquads::SquadState::Down,
            state0: None,
            fix: None,
            news_t: 0.0,
            decide_at: 0.0,
            decided_at: 0.0,
            contact_t: 0.0,
            called_t: 0.0,
            called_room: None,
            fallen: JsSet::new(),
            ready: false,
            own_said: None,
            stayed: None,
            hit_room: None,
            rc: None,
            calls: None,
            call_die: None,
            hung: None,
            trouble: None,
            wary: None,
            ask_die: None,
            sent: Vec::new(),
            went: Vec::new(),
            due: Vec::new(),
            events: Vec::new(),
        }
    }
}
impl Default for Squad { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:272 Squad.hung, src/game/enemysquads.ts:272)
#[derive(Clone, Copy, Debug)]
pub struct SquadHungItem {
    pub who: GuardId,
    pub t: f64,
}
impl SquadHungItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadHungItem {
            who: GuardId(0),
            t: 0.0,
        }
    }
}
impl Default for SquadHungItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:275 Squad.due, src/game/enemysquads.ts:275)
#[derive(Clone, Debug)]
pub struct SquadDueItem {
    pub gd: GuardId,
    pub at: V3,
    pub t: f64,
    pub room: Option<crate::scene_scenarios::StructureRoom>,
}
impl SquadDueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadDueItem {
            gd: GuardId(0),
            at: [0.0; 3],
            t: 0.0,
            room: None,
        }
    }
}
impl Default for SquadDueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/enemysquads.ts:277)
#[derive(Clone, Debug)]
pub struct State {
    pub stay_ask: Option<bool>,
    pub s: crate::scene_scenarios::StructureData,
    pub squads: Vec<EnemysquadsSquadId>,
    pub of: JsMap<GuardId, EnemysquadsSquadId>,
    pub net: Vec<crate::enemysquads::StateNetItem>,
    pub nc: Option<crate::enemysquads::NetCheck>,
    pub word: Option<crate::enemysquads::StateWord>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            stay_ask: None,
            s: crate::scene_scenarios::StructureData::zeroed(),
            squads: Vec::new(),
            of: JsMap::new(),
            net: Vec::new(),
            nc: None,
            word: None,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:277 State.net, src/game/enemysquads.ts:277)
#[derive(Clone, Copy, Debug)]
pub struct StateNetItem {
    pub t: f64,
    pub squad: EnemysquadsSquadId,
    pub at: V3,
    pub door: Option<DoorId>,
}
impl StateNetItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateNetItem {
            t: 0.0,
            squad: EnemysquadsSquadId(0),
            at: [0.0; 3],
            door: None,
        }
    }
}
impl Default for StateNetItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: NetCheck, src/game/enemysquads.ts:280)
#[derive(Clone, Debug)]
pub struct NetCheck {
    pub next_at: f64,
    pub stage: crate::enemysquads::NetCheckStage,
    pub t0: f64,
    pub caller: Option<GuardId>,
    pub about: Option<EnemysquadsSquadId>,
    pub done: JsSet<EnemysquadsSquadId>,
    pub tried: JsMap<EnemysquadsSquadId, f64>,
    pub queried: JsMap<EnemysquadsSquadId, f64>,
    pub say: Vec<crate::enemysquads::NetCheckSayItem>,
    pub die: crate::math_dice::Die,
    pub tick_t: f64,
}
impl NetCheck {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetCheck {
            next_at: 0.0,
            stage: crate::enemysquads::NetCheckStage::Idle,
            t0: 0.0,
            caller: None,
            about: None,
            done: JsSet::new(),
            tried: JsMap::new(),
            queried: JsMap::new(),
            say: Vec::new(),
            die: crate::math_dice::Die::zeroed(),
            tick_t: 0.0,
        }
    }
}
impl Default for NetCheck { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NetCheckStage {
    Idle,
    Called,
    Asked,
}
impl NetCheckStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            NetCheckStage::Idle => "idle",
            NetCheckStage::Called => "called",
            NetCheckStage::Asked => "asked",
        }
    }
}

/// (TypeScript: inline at src/game/enemysquads.ts:280 NetCheck.say, src/game/enemysquads.ts:280)
#[derive(Clone, Debug)]
pub struct NetCheckSayItem {
    pub t: f64,
    pub gd: GuardId,
    pub text: String,
    pub cls: Option<f64>,
}
impl NetCheckSayItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetCheckSayItem {
            t: 0.0,
            gd: GuardId(0),
            text: String::new(),
            cls: None,
        }
    }
}
impl Default for NetCheckSayItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:277 State.word, src/game/enemysquads.ts:277)
#[derive(Clone, Debug)]
pub struct StateWord {
    pub t: f64,
    pub at: V3,
    pub done: f64,
    pub said_t: f64,
    pub lost: bool,
    pub out: Option<Vec<GuardId>>,
    pub call: Option<crate::enemysquads::StateWordCall>,
}
impl StateWord {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateWord {
            t: 0.0,
            at: [0.0; 3],
            done: 0.0,
            said_t: 0.0,
            lost: false,
            out: None,
            call: None,
        }
    }
}
impl Default for StateWord { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:277 StateWord.call, src/game/enemysquads.ts:277)
#[derive(Clone, Debug)]
pub struct StateWordCall {
    pub q: EnemysquadsSquadId,
    pub go: Vec<RoomfightDefId>,
    pub at: V3,
    pub go_t: f64,
    pub w: f64,
}
impl StateWordCall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateWordCall {
            q: EnemysquadsSquadId(0),
            go: Vec::new(),
            at: [0.0; 3],
            go_t: 0.0,
            w: 0.0,
        }
    }
}
impl Default for StateWordCall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Way, src/game/enemysquads.ts:439)
#[derive(Clone, Debug)]
pub struct Way {
    pub spot: V3,
    pub door: Option<DoorId>,
    pub l: f64,
    pub path: Option<Vec<V3>>,
    pub aim: Option<V3>,
}
impl Way {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Way {
            spot: [0.0; 3],
            door: None,
            l: 0.0,
            path: None,
            aim: None,
        }
    }
}
impl Default for Way { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Known, src/game/enemysquads.ts:489)
#[derive(Clone, Copy, Debug)]
pub struct Known {
    pub p: V3,
    pub firm: bool,
}
impl Known {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Known {
            p: [0.0; 3],
            firm: false,
        }
    }
}
impl Default for Known { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoundCtx, src/game/enemysquads.ts:505)
#[derive(Clone, Debug)]
pub struct RoundCtx {
    pub t: f64,
    pub t_2: RoomfightRoomId,
    pub key: String,
    pub rear: Vec<V3>,
    pub mouths: Vec<V3>,
    pub hot: crate::enemysquads::RoundHot,
    pub hot_cell: crate::enemysquads::RoundHotCell,
    pub cost: JsMap<crate::Num, f64>,
    pub held: JsMap<crate::Num, RoomfightRoomId>,
}
impl RoundCtx {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoundCtx {
            t: 0.0,
            t_2: RoomfightRoomId(0),
            key: String::new(),
            rear: Vec::new(),
            mouths: Vec::new(),
            hot: crate::enemysquads::RoundHot::Places { known: Vec::new(), ajar: Vec::new() },
            hot_cell: crate::enemysquads::RoundHotCell::Memo { hot_at: JsMap::new() },
            cost: JsMap::new(),
            held: JsMap::new(),
        }
    }
}
impl Default for RoundCtx { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SquadReport, src/game/enemysquads.ts:1518)
#[derive(Clone, Debug)]
pub struct SquadReport {
    pub id: String,
    pub name: String,
    pub men: f64,
    pub up: f64,
    pub state: String,
    pub out: f64,
    pub events: Vec<crate::enemysquads::SquadEvent>,
}
impl SquadReport {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadReport {
            id: String::new(),
            name: String::new(),
            men: 0.0,
            up: 0.0,
            state: String::new(),
            out: 0.0,
            events: Vec::new(),
        }
    }
}
impl Default for SquadReport { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:872 BANGS, src/game/enemysquads.ts:872)
#[derive(Clone, Copy, Debug)]
pub struct EnemysquadsModBANGSValue {
    pub n: f64,
    pub t: f64,
    pub read: f64,
}
impl EnemysquadsModBANGSValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EnemysquadsModBANGSValue {
            n: 0.0,
            t: 0.0,
            read: 0.0,
        }
    }
}
impl Default for EnemysquadsModBANGSValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:326 grown ->, src/game/enemysquads.ts:326)
#[derive(Clone, Copy, Debug)]
pub struct Grown {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl Grown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Grown {
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for Grown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:365 squadOf ->, src/game/enemysquads.ts:365)
#[derive(Clone, Debug)]
pub struct SquadOf {
    pub def: crate::scene_scenarios::SquadData,
    pub men: Vec<GuardId>,
}
impl SquadOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOf {
            def: crate::scene_scenarios::SquadData::zeroed(),
            men: Vec::new(),
        }
    }
}
impl Default for SquadOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:547 wayRound2(says), src/game/enemysquads.ts:547)
#[derive(Clone, Copy, Debug)]
pub struct WayRound2Says {
    pub limit: Option<f64>,
    pub best: Option<f64>,
}
impl WayRound2Says {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WayRound2Says {
            limit: None,
            best: None,
        }
    }
}
impl Default for WayRound2Says { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EventWhat {
    Down,
    Back,
    Contact,
    Lost,
    Quiet,
    Found,
    Said,
    Sweep,
    Flank,
    Glimpse,
    Barricade,
    Help,
    Reinforce,
    InPosition,
    Met,
    Rally,
    Stays,
    Follows,
    Check,
    Asks,
    Looks,
    NoSign,
    Calls,
    CutOff,
    NetCheck,
    NetQuery,
    NetAsks,
    NetLooks,
    NetHelp,
}
impl EventWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            EventWhat::Down => "down",
            EventWhat::Back => "back",
            EventWhat::Contact => "contact",
            EventWhat::Lost => "lost",
            EventWhat::Quiet => "quiet",
            EventWhat::Found => "found",
            EventWhat::Said => "said",
            EventWhat::Sweep => "sweep",
            EventWhat::Flank => "flank",
            EventWhat::Glimpse => "glimpse",
            EventWhat::Barricade => "barricade",
            EventWhat::Help => "help",
            EventWhat::Reinforce => "reinforce",
            EventWhat::InPosition => "in position",
            EventWhat::Met => "met",
            EventWhat::Rally => "rally",
            EventWhat::Stays => "stays",
            EventWhat::Follows => "follows",
            EventWhat::Check => "check",
            EventWhat::Asks => "asks",
            EventWhat::Looks => "looks",
            EventWhat::NoSign => "no sign",
            EventWhat::Calls => "calls",
            EventWhat::CutOff => "cut off",
            EventWhat::NetCheck => "net check",
            EventWhat::NetQuery => "net query",
            EventWhat::NetAsks => "net asks",
            EventWhat::NetLooks => "net looks",
            EventWhat::NetHelp => "net help",
        }
    }
}

/// (TypeScript: inline at src/game/enemysquads.ts:612 stayKey(room), src/game/enemysquads.ts:612)
#[derive(Clone, Debug)]
pub struct StayKeyRoom {
    pub name: String,
}
impl StayKeyRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StayKeyRoom {
            name: String::new(),
        }
    }
}
impl Default for StayKeyRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:712 contact(words), src/game/enemysquads.ts:712)
#[derive(Clone, Debug)]
pub struct ContactWords {
    pub say: String,
    pub why: String,
}
impl ContactWords {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ContactWords {
            say: String::new(),
            why: String::new(),
        }
    }
}
impl Default for ContactWords { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:745 movedOn ->, src/game/enemysquads.ts:745)
#[derive(Clone, Debug)]
pub struct MovedOn {
    pub at: V3,
    pub pair: Vec<GuardId>,
}
impl MovedOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MovedOn {
            at: [0.0; 3],
            pair: Vec::new(),
        }
    }
}
impl Default for MovedOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:885 radioCheck ->, src/game/enemysquads.ts:885)
#[derive(Clone, Debug)]
pub struct RadioCheck {
    pub at: V3,
    pub room: Option<crate::scene_scenarios::StructureRoom>,
}
impl RadioCheck {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RadioCheck {
            at: [0.0; 3],
            room: None,
        }
    }
}
impl Default for RadioCheck { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:1242 squadCalls ->, src/game/enemysquads.ts:1242)
#[derive(Clone, Debug)]
pub struct SquadCallsItem {
    pub squad: String,
    pub by: GuardId,
    pub t0: f64,
    pub end: f64,
    pub body: Option<GuardId>,
}
impl SquadCallsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadCallsItem {
            squad: String::new(),
            by: GuardId(0),
            t0: 0.0,
            end: 0.0,
            body: None,
        }
    }
}
impl Default for SquadCallsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:1265 netSend ->, src/game/enemysquads.ts:1265)
#[derive(Clone, Debug)]
pub struct NetSend {
    pub q: EnemysquadsSquadId,
    pub go: Vec<RoomfightDefId>,
}
impl NetSend {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetSend {
            q: EnemysquadsSquadId(0),
            go: Vec::new(),
        }
    }
}
impl Default for NetSend { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:1535 squadWaysRound ->, src/game/enemysquads.ts:1535)
#[derive(Clone, Debug)]
pub struct SquadWaysRoundItem {
    pub squad: String,
    pub man: String,
    pub from: String,
    pub room: String,
    pub door: String,
    pub hold: bool,
    pub team: bool,
    pub way: Option<String>,
    pub l: f64,
    pub limit: Option<f64>,
    pub best: Option<f64>,
}
impl SquadWaysRoundItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadWaysRoundItem {
            squad: String::new(),
            man: String::new(),
            from: String::new(),
            room: String::new(),
            door: String::new(),
            hold: false,
            team: false,
            way: None,
            l: 0.0,
            limit: None,
            best: None,
        }
    }
}
impl Default for SquadWaysRoundItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct EnemysquadsMod {
    /// (src/game/enemysquads.ts:281 STATES)
    pub states: Option<crate::enemysquads::State>,
    /// (src/game/enemysquads.ts:506 CTX)
    pub ctx: Option<crate::enemysquads::RoundCtx>,
    /// (src/game/enemysquads.ts:872 BANGS)
    pub bangs: JsMap<GuardId, crate::enemysquads::EnemysquadsModBANGSValue>,
    /// (src/game/enemysquads.ts:1123 GLIMPSED)
    pub glimpsed: JsMap<GuardId, f64>,
}
impl EnemysquadsMod {
    pub fn blank() -> Self {
        EnemysquadsMod {
            states: None,
            ctx: None,
            bangs: JsMap::new(),
            glimpsed: JsMap::new(),
        }
    }
}

/// (src/game/enemysquads.ts:231 SQUADS_DEFAULTS: a table nothing writes)
pub fn squads_defaults() -> crate::knobs::SquadsK { crate::knobs::page_squads() }

/// (src/game/enemysquads.ts:606 COUNT: a table nothing writes)
pub fn count() -> Vec<String> { alloc::vec![String::from("nobody"), String::from("one"), String::from("two"), String::from("three"), String::from("four")] }

/// (src/game/enemysquads.ts:609 NUMBER: a table nothing writes)
pub fn number() -> Vec<String> { alloc::vec![String::from("Zero"), String::from("One"), String::from("Two"), String::from("Three"), String::from("Four"), String::from("Five"), String::from("Six"), String::from("Seven"), String::from("Eight"), String::from("Nine"), String::from("Ten"), String::from("Eleven"), String::from("Twelve"), String::from("Thirteen"), String::from("Fourteen"), String::from("Fifteen"), String::from("Sixteen")] }

/// (src/game/enemysquads.ts:1033 LOOK_AT_M)
pub const LOOK_AT_M: f64 = 1.2;

/// (src/game/enemysquads.ts:1033 LOOK_NEAR_M)
pub const LOOK_NEAR_M: f64 = 2.5;

/// (src/game/enemysquads.ts:1033 LOOK_ROUND_M)
pub const LOOK_ROUND_M: f64 = 3.5;

/// (src/game/enemysquads.ts:1036 LOOK_SECS)
pub const LOOK_SECS: f64 = 1.2;

/// (src/game/enemysquads.ts:1186 NET_OUT_SECS)
pub const NET_OUT_SECS: f64 = 90.0;

/// (src/game/enemysquads.ts:1188 HUNG_SECS)
pub const HUNG_SECS: f64 = 10.0;

/// (src/game/enemysquads.ts:1257 NET_BREATH)
pub const NET_BREATH: f64 = 1.2;

/// (src/game/enemysquads.ts:1285 NET_TRIES)
pub const NET_TRIES: f64 = 2.0;
