//! The types of `enemysquads`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
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
/// INVENTED: one row of DRILL_AT, what a map writes over the drill's table; None = the table's own (enemysquads.ts:287, the rows its literal names; merged as { ...SQUADS, ...row } at :306). One arm: the proposal shape has no key for a new struct
#[derive(Clone, Debug, ToBytes)]
pub enum DrillAtRow {
    Row { drill_net: Option<bool>, net_k: Option<f64>, reinforce_k: Option<f64>, drill_own: Option<bool>, own_up: Option<f64>, drill_back: Option<bool>, back_beat: Option<f64> },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum SquadChoice {
    Barricade,
    Flank,
    Help,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum SquadState {
    #[default] Down,
    Contact,
    Hold,
    Alerted,
    Barricade,
    Flank,
    Help,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Fix {
    pub at: V3,
    pub t: f64,
    pub ally: Option<EnemysquadsSquadId>,
    pub by: crate::enemysquads::FixBy,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum FixBy {
    #[default] Ears,
    Net,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum SentRole {
    #[default] Look,
    Flank,
    Help,
    Reinforce,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum SentPhase {
    #[default] Set,
    Run,
    Loose,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadEvent {
    pub t: f64,
    pub what: crate::enemysquads::SquadEventWhat,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum SquadEventWhat {
    #[default] Down,
    Back,
    Contact,
    Lost,
    Quiet,
    Found,
    Said,
    Sweep,
    Barricade,
    Flank,
    Help,
    Reinforce,
    #[word("in position")] InPosition,
    Met,
    Rally,
    Stays,
    Follows,
    Check,
    Asks,
    Looks,
    #[word("no sign")] NoSign,
    Glimpse,
    Calls,
    #[word("cut off")] CutOff,
    #[word("net check")] NetCheck,
    #[word("net query")] NetQuery,
    #[word("net asks")] NetAsks,
    #[word("net looks")] NetLooks,
    #[word("net help")] NetHelp,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CheckStage {
    #[default] Idle,
    Called,
    Asked,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Quiet {
    pub gd: GuardId,
    pub at: V3,
    pub t: f64,
    pub heard: bool,
    pub held: bool,
    pub small0: JsMap<GuardId, f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CheckCutItem {
    pub who: GuardId,
    pub at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CheckNetOutValue {
    pub about: EnemysquadsSquadId,
    pub at: V3,
    pub t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Call {
    pub by: GuardId,
    pub t0: f64,
    pub end: f64,
    pub at: V3,
    pub room: Option<crate::scene_scenarios::StructureRoom>,
    pub body: Option<GuardId>,
    pub eyes: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadHungItem {
    pub who: GuardId,
    pub t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadDueItem {
    pub gd: GuardId,
    pub at: V3,
    pub t: f64,
    pub room: Option<crate::scene_scenarios::StructureRoom>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub stay_ask: Option<bool>,
    pub s: crate::scene_scenarios::StructureData,
    pub squads: Vec<EnemysquadsSquadId>,
    pub of: JsMap<GuardId, EnemysquadsSquadId>,
    pub net: Vec<crate::enemysquads::StateNetItem>,
    pub nc: Option<crate::enemysquads::NetCheck>,
    pub word: Option<crate::enemysquads::StateWord>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateNetItem {
    pub t: f64,
    pub squad: EnemysquadsSquadId,
    pub at: V3,
    pub door: Option<DoorId>,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum NetCheckStage {
    #[default] Idle,
    Called,
    Asked,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct NetCheckSayItem {
    pub t: f64,
    pub gd: GuardId,
    pub text: String,
    pub cls: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StateWord {
    pub t: f64,
    pub at: V3,
    pub done: f64,
    pub said_t: f64,
    pub lost: bool,
    pub out: Option<Vec<GuardId>>,
    pub call: Option<crate::enemysquads::StateWordCall>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StateWordCall {
    pub q: EnemysquadsSquadId,
    pub go: Vec<RoomfightDefId>,
    pub at: V3,
    pub go_t: f64,
    pub w: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Way {
    pub spot: V3,
    pub door: Option<DoorId>,
    pub l: f64,
    pub path: Option<Vec<V3>>,
    pub aim: Option<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Known {
    pub p: V3,
    pub firm: bool,
}

#[derive(Clone, Debug, ToBytes)]
pub struct RoundCtx {
    pub t: f64,
    pub t_2: RoomfightRoomId,
    pub key: String,
    pub rear: Vec<V3>,
    pub mouths: Vec<V3>,
    /// (one round's own, made again by the next: in no bytes)
    #[bytes(skip)]
    pub hot: crate::enemysquads::RoundHot,
    /// (one round's own, made again by the next: in no bytes)
    #[bytes(skip)]
    pub hot_cell: crate::enemysquads::RoundHotCell,
    pub cost: JsMap<crate::Num, f64>,
    pub held: JsMap<crate::Num, RoomfightRoomId>,
}
impl Default for RoundCtx {
    fn default() -> Self {
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EnemysquadsModBANGSValue {
    pub n: f64,
    pub t: f64,
    pub read: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Grown {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SquadOf {
    pub def: crate::scene_scenarios::SquadData,
    pub men: Vec<GuardId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WayRound2Says {
    pub limit: Option<f64>,
    pub best: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct StayKeyRoom {
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct ContactWords {
    pub say: String,
    pub why: String,
}

#[derive(Clone, Debug, Default)]
pub struct MovedOn {
    pub at: V3,
    pub pair: Vec<GuardId>,
}

#[derive(Clone, Debug, Default)]
pub struct RadioCheck {
    pub at: V3,
    pub room: Option<crate::scene_scenarios::StructureRoom>,
}

#[derive(Clone, Debug, Default)]
pub struct NetSend {
    pub q: EnemysquadsSquadId,
    pub go: Vec<RoomfightDefId>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct EnemysquadsMod {
    /// (src/game/enemysquads.ts:281 STATES)
    pub states: Option<crate::enemysquads::State>,
    /// (src/game/enemysquads.ts:513 CTX)
    pub ctx: Option<crate::enemysquads::RoundCtx>,
    /// (src/game/enemysquads.ts:879 BANGS)
    pub bangs: JsMap<GuardId, crate::enemysquads::EnemysquadsModBANGSValue>,
    /// (src/game/enemysquads.ts:1130 GLIMPSED)
    pub glimpsed: JsMap<GuardId, f64>,
}

/// (src/game/enemysquads.ts:613 COUNT: a table nothing writes)
pub fn count() -> Vec<String> { alloc::vec![String::from("nobody"), String::from("one"), String::from("two"), String::from("three"), String::from("four")] }

/// (src/game/enemysquads.ts:616 NUMBER: a table nothing writes)
pub fn number() -> Vec<String> { alloc::vec![String::from("Zero"), String::from("One"), String::from("Two"), String::from("Three"), String::from("Four"), String::from("Five"), String::from("Six"), String::from("Seven"), String::from("Eight"), String::from("Nine"), String::from("Ten"), String::from("Eleven"), String::from("Twelve"), String::from("Thirteen"), String::from("Fourteen"), String::from("Fifteen"), String::from("Sixteen")] }

/// (src/game/enemysquads.ts:1040 LOOK_AT_M)
pub const LOOK_AT_M: f64 = 1.2;

/// (src/game/enemysquads.ts:1040 LOOK_NEAR_M)
pub const LOOK_NEAR_M: f64 = 2.5;

/// (src/game/enemysquads.ts:1040 LOOK_ROUND_M)
pub const LOOK_ROUND_M: f64 = 3.5;

/// (src/game/enemysquads.ts:1043 LOOK_SECS)
pub const LOOK_SECS: f64 = 1.2;

/// (src/game/enemysquads.ts:1193 NET_OUT_SECS)
pub const NET_OUT_SECS: f64 = 90.0;

/// (src/game/enemysquads.ts:1195 HUNG_SECS)
pub const HUNG_SECS: f64 = 10.0;

/// (src/game/enemysquads.ts:1264 NET_BREATH)
pub const NET_BREATH: f64 = 1.2;

/// (src/game/enemysquads.ts:1292 NET_TRIES)
pub const NET_TRIES: f64 = 2.0;
