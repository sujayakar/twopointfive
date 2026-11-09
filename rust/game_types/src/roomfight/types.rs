//! The types of `roomfight`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Entry {
    pub door: DoorId,
    pub at: V3,
    pub out: [f64; 2],
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct RoomFight {
    #[is({ let mut t = JsMap::new(); t.set(crate::roomfight::RoomFightSkillKey::Operative, crate::roomfight::FightSkill { react: [0.12, 0.22], ready: 1.0, surprise: [0.35, 0.6], sector_deg: 50.0, settle_k: 0.4, spread_k: 0.5, preaim: 0.6, notice: 0.12, pair: 0.16, shift: [0.2, 0.3], hold: [f64::INFINITY, f64::INFINITY], glance: [0.0, 0.0], turn: 8.0, bang_break: 0.0, startle: [0.0, 0.0], daze: [0.0, 0.0], nerve: [2.0, 2.0], rally: [0.0, 0.0], rally_chance: 0.0, funnel: 0.35, line: 0.5 }); t.set(crate::roomfight::RoomFightSkillKey::Hostile, crate::roomfight::FightSkill { react: [0.4, 1.1], ready: 0.25, surprise: [0.5, 0.9], sector_deg: 40.0, settle_k: 1.5, spread_k: 1.0, preaim: 0.55, notice: 0.85, pair: 0.1, shift: [0.4, 0.9], hold: [5.0, 11.0], glance: [1.2, 2.6], turn: 3.0, bang_break: 0.3, startle: [0.4, 0.65], daze: [1.0, 2.0], nerve: [0.34, 0.7], rally: [1.5, 4.0], rally_chance: 0.75, funnel: f64::INFINITY, line: f64::INFINITY }); t })] pub skill: JsMap<crate::roomfight::RoomFightSkillKey, crate::roomfight::FightSkill>,
    pub live: bool,
    #[is(true)] pub blind_fire: bool,
    pub blind_rounds: f64,
    pub defs: Vec<RoomfightDefId>,
    pub ops: Vec<RoomfightOpId>,
    pub log: Vec<crate::roomfight::FightNote>,
    #[is(crate::roomfight::RoomFightStats { op_rounds: 0.0, held_for_friend: 0.0, fell_back: 0.0, fall_back_out: 0.0, took_cover: 0.0, rallied: 0.0, surprised: 0.0, broke: 0.0, stepped: 0.0, startled: 0.0 })] pub stats: crate::roomfight::RoomFightStats,
    pub rooms: Vec<RoomfightRoomId>,
    pub room_of_cell: JsMap<crate::Num, RoomfightRoomId>,
    #[is(alloc::vec![0; 0])] pub fill_mark: Vec<u8>,
    pub called: JsSet<GuardId>,
    pub down_called: JsSet<GuardId>,
    #[is(-100.0)] pub hot_t: f64,
    pub reports: Vec<crate::roomfight::RoomFightReportsItem>,
    pub heard_ev: JsSet<u32>,
    pub sams: Vec<crate::roomfight::RoomFightSamsItem>,
    pub drill_said: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum RoomFightSkillKey {
    Operative,
    Hostile,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FightSkill {
    pub react: [f64; 2],
    pub ready: f64,
    pub surprise: [f64; 2],
    pub sector_deg: f64,
    pub settle_k: f64,
    pub spread_k: f64,
    pub preaim: f64,
    pub notice: f64,
    pub pair: f64,
    pub shift: [f64; 2],
    pub hold: [f64; 2],
    pub glance: [f64; 2],
    pub turn: f64,
    pub bang_break: f64,
    pub startle: [f64; 2],
    pub daze: [f64; 2],
    pub nerve: [f64; 2],
    pub rally: [f64; 2],
    pub rally_chance: f64,
    pub funnel: f64,
    pub line: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FightNote {
    pub t: f64,
    pub who: String,
    pub what: String,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RoomFightStats {
    pub op_rounds: f64,
    pub held_for_friend: f64,
    pub fell_back: f64,
    pub fall_back_out: f64,
    pub took_cover: f64,
    pub rallied: f64,
    pub surprised: f64,
    pub broke: f64,
    pub stepped: f64,
    pub startled: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RoomFightReportsItem {
    pub by: GuardId,
    pub t: f64,
    pub from: Option<V3>,
    pub to: Option<V3>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RoomFightSamsItem {
    pub x: f64,
    pub z: f64,
    pub t: f64,
    pub v: [f64; 2],
    pub known: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Room {
    pub id: f64,
    pub cells: JsSet<crate::Num>,
    pub entries: Vec<crate::roomfight::Entry>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum DefMode {
    #[default] Hold,
    Move,
    Free,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Def {
    pub gd: GuardId,
    pub station: V3,
    pub home: V3,
    pub room: Option<RoomfightRoomId>,
    pub pinned0: bool,
    pub staged: Option<bool>,
    pub ev: Vec<f64>,
    pub watch: f64,
    pub want: f64,
    pub want_at: f64,
    pub glance_to: f64,
    pub glance_until: f64,
    pub glance_next: f64,
    pub pt: V3,
    pub heard_t: f64,
    pub cnt: crate::roomfight::DefCnt,
    pub noise_at: Option<V3>,
    pub noise_t: f64,
    pub noise_out: bool,
    pub last_state: String,
    pub last_seen: f64,
    pub fix_keep: f64,
    pub pre: bool,
    pub nerve: f64,
    pub lost_at: f64,
    pub moved_at: f64,
    pub rally_at: f64,
    pub rally_go: bool,
    pub mode: crate::roomfight::DefMode,
    pub move_kind: crate::roomfight::DefMoveKind,
    pub move_t: f64,
    pub came_by: Option<DoorId>,
    pub stall_at: V3,
    pub stall_t: f64,
    pub free_at: f64,
    pub burst: crate::roomfight::DefBurst,
    pub bolt_until: f64,
    pub bolt_from: Option<crate::game::Foe>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DefCnt {
    pub shots: f64,
    pub kick: f64,
    pub bang: f64,
    pub small: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DefMoveKind {
    #[default] Cover,
    Fallback,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DefBurst {
    pub until: f64,
    pub n: f64,
    pub next: f64,
    pub broke: bool,
    pub daze: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Meter {
    pub m: f64,
    pub seen_t: f64,
    pub glass_t: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Op {
    pub gd: GuardId,
    pub meters: JsMap<GuardId, crate::roomfight::Meter>,
    pub tgt: Option<GuardId>,
    pub turn: Option<crate::roomfight::OpTurn>,
    pub pass: Option<crate::roomfight::OpPass>,
    pub kept: Option<crate::roomfight::OpKept>,
    pub track: f64,
    pub pre: bool,
    pub ready_at: f64,
    pub n: f64,
    pub next: f64,
    pub lost_at: f64,
    pub fell: Option<bool>,
    pub aim: V3,
    pub ov: crate::roomfight::OpOv,
    pub step: Option<crate::roomfight::OpStep>,
    pub hot_t: f64,
    pub step_t: f64,
    pub step_by: Option<crate::roomfight::OpStepBy>,
    pub door_by: Option<crate::roomfight::OpDoorBy>,
    pub funnel_t: f64,
    pub line_t: f64,
    pub line_by: Option<crate::roomfight::OpLineBy>,
    pub line_at: f64,
    pub wall_t: f64,
    pub step_until: f64,
    pub said_t: f64,
    pub rounds: f64,
    pub held_for_friend: f64,
    pub held_for_line: f64,
    pub block_t: f64,
    pub feet_sm: Option<f64>,
    pub feet_t: Option<f64>,
    pub short_seen: f64,
    pub short_since: f64,
    pub short_t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OpTurn {
    pub to: GuardId,
    pub at: f64,
    pub until: f64,
    pub pt: Option<V3>,
    pub t0: Option<f64>,
    pub door: Option<DoorId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OpPass {
    pub h: GuardId,
    pub until: f64,
    pub no_shot: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OpKept {
    pub h: GuardId,
    pub track: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OpOv {
    pub face: Option<f64>,
    pub walk: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OpStep {
    pub mine: V3,
    pub goal: Option<V3>,
    pub speed: Option<f64>,
    pub strafe: Option<bool>,
    pub face: Option<f64>,
    pub door: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OpStepBy {
    pub by: GuardId,
    pub t: f64,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OpDoorBy {
    pub by: GuardId,
    pub t: f64,
    pub at: V3,
    pub door: DoorId,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub enum OpLineBy {
    Guard(GuardId),
    Player,
}

#[derive(Clone, Copy, Debug)]
pub enum RoomFightFriendInLine {
    Guard(GuardId),
    Player,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RoomFightDoorShot {
    pub by: GuardId,
    pub at: V3,
    pub door: DoorId,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RoomFightStepToPatch {
    pub speed: Option<f64>,
    pub face: Option<f64>,
    pub strafe: Option<bool>,
    pub aim_at: Option<V3>,
    pub stance: Option<crate::character::Stance>,
    pub stack_side: Option<f64>,
    pub crouch: Option<bool>,
    pub lean: Option<f64>,
    pub upper: Option<crate::character::UpperMode>,
    pub light: Option<bool>,
    pub arrived: Option<bool>,
    pub hot: Option<bool>,
    pub flinch: Option<bool>,
    pub direct: Option<bool>,
    pub smoke: Option<bool>,
    pub over: Option<crate::game::GuardScriptOver>,
}

/// (src/game/roomfight.ts:260 FIGHT_SKILL: a table nothing writes)
pub fn fight_skill() -> JsMap<String, crate::roomfight::FightSkill> { { let mut t = JsMap::new(); t.set(String::from("operative"), crate::roomfight::FightSkill { react: [0.12, 0.22], ready: 1.0, surprise: [0.35, 0.6], sector_deg: 50.0, settle_k: 0.4, spread_k: 0.5, preaim: 0.6, notice: 0.12, pair: 0.16, shift: [0.2, 0.3], hold: [f64::INFINITY, f64::INFINITY], glance: [0.0, 0.0], turn: 8.0, bang_break: 0.0, startle: [0.0, 0.0], daze: [0.0, 0.0], nerve: [2.0, 2.0], rally: [0.0, 0.0], rally_chance: 0.0, funnel: 0.35, line: 0.5 }); t.set(String::from("hostile"), crate::roomfight::FightSkill { react: [0.4, 1.1], ready: 0.25, surprise: [0.5, 0.9], sector_deg: 40.0, settle_k: 1.5, spread_k: 1.0, preaim: 0.55, notice: 0.85, pair: 0.1, shift: [0.4, 0.9], hold: [5.0, 11.0], glance: [1.2, 2.6], turn: 3.0, bang_break: 0.3, startle: [0.4, 0.65], daze: [1.0, 2.0], nerve: [0.34, 0.7], rally: [1.5, 4.0], rally_chance: 0.75, funnel: f64::INFINITY, line: f64::INFINITY }); t } }

/// (src/game/roomfight.ts:293 ROOM_MAX_CELLS)
pub const ROOM_MAX_CELLS: f64 = 2400.0;

/// (src/game/roomfight.ts:294 ROOM_DX: a table nothing writes)
pub fn room_dx() -> Vec<f64> { alloc::vec![1.0, -1.0, 0.0, 0.0] }

/// (src/game/roomfight.ts:294 ROOM_DZ: a table nothing writes)
pub fn room_dz() -> Vec<f64> { alloc::vec![0.0, 0.0, 1.0, -1.0] }

/// (src/game/roomfight.ts:315 GLASS_LOOK)
pub const GLASS_LOOK: f64 = 4.0;
