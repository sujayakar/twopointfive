//! The types of `roomfight`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Entry, src/game/roomfight.ts:291)
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    pub door: DoorId,
    pub at: V3,
    pub out: [f64; 2],
}
impl Entry {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Entry {
            door: DoorId(0),
            at: [0.0; 3],
            out: [0.0; 2],
        }
    }
}
impl Default for Entry { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FightSkill, src/game/roomfight.ts:228)
#[derive(Clone, Copy, Debug)]
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
impl FightSkill {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FightSkill {
            react: [0.0; 2],
            ready: 0.0,
            surprise: [0.0; 2],
            sector_deg: 0.0,
            settle_k: 0.0,
            spread_k: 0.0,
            preaim: 0.0,
            notice: 0.0,
            pair: 0.0,
            shift: [0.0; 2],
            hold: [0.0; 2],
            glance: [0.0; 2],
            turn: 0.0,
            bang_break: 0.0,
            startle: [0.0; 2],
            daze: [0.0; 2],
            nerve: [0.0; 2],
            rally: [0.0; 2],
            rally_chance: 0.0,
            funnel: 0.0,
            line: 0.0,
        }
    }
}
impl Default for FightSkill { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Room, src/game/roomfight.ts:292)
#[derive(Clone, Debug)]
pub struct Room {
    pub id: f64,
    pub cells: JsSet<crate::Num>,
    pub entries: Vec<crate::roomfight::Entry>,
}
impl Room {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Room {
            id: 0.0,
            cells: JsSet::new(),
            entries: Vec::new(),
        }
    }
}
impl Default for Room { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DefMode {
    Hold,
    Move,
    Free,
}
impl DefMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            DefMode::Hold => "hold",
            DefMode::Move => "move",
            DefMode::Free => "free",
        }
    }
}

/// (TypeScript: Def, src/game/roomfight.ts:297)
#[derive(Clone, Debug)]
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
    /// INVENTED: last_known is this very D.pt (roomfight.ts:303, off path with BELIEF on)
    pub fix_is_pt: bool,
}
impl Def {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Def {
            gd: GuardId(0),
            station: [0.0; 3],
            home: [0.0; 3],
            room: None,
            pinned0: false,
            staged: None,
            ev: Vec::new(),
            watch: 0.0,
            want: 0.0,
            want_at: 0.0,
            glance_to: 0.0,
            glance_until: 0.0,
            glance_next: 0.0,
            pt: [0.0; 3],
            heard_t: 0.0,
            cnt: crate::roomfight::DefCnt::zeroed(),
            noise_at: None,
            noise_t: 0.0,
            noise_out: false,
            last_state: String::new(),
            last_seen: 0.0,
            fix_keep: 0.0,
            pre: false,
            nerve: 0.0,
            lost_at: 0.0,
            moved_at: 0.0,
            rally_at: 0.0,
            rally_go: false,
            mode: crate::roomfight::DefMode::Hold,
            move_kind: crate::roomfight::DefMoveKind::Cover,
            move_t: 0.0,
            came_by: None,
            stall_at: [0.0; 3],
            stall_t: 0.0,
            free_at: 0.0,
            burst: crate::roomfight::DefBurst::zeroed(),
            bolt_until: 0.0,
            bolt_from: None,
            fix_is_pt: false,
        }
    }
}
impl Default for Def { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:305 Def.cnt, src/game/roomfight.ts:305)
#[derive(Clone, Copy, Debug)]
pub struct DefCnt {
    pub shots: f64,
    pub kick: f64,
    pub bang: f64,
    pub small: f64,
}
impl DefCnt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefCnt {
            shots: 0.0,
            kick: 0.0,
            bang: 0.0,
            small: 0.0,
        }
    }
}
impl Default for DefCnt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DefMoveKind {
    Cover,
    Fallback,
}
impl DefMoveKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DefMoveKind::Cover => "cover",
            DefMoveKind::Fallback => "fallback",
        }
    }
}

/// (TypeScript: inline at src/game/roomfight.ts:310 Def.burst, src/game/roomfight.ts:310)
#[derive(Clone, Copy, Debug)]
pub struct DefBurst {
    pub until: f64,
    pub n: f64,
    pub next: f64,
    pub broke: bool,
    pub daze: f64,
}
impl DefBurst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefBurst {
            until: 0.0,
            n: 0.0,
            next: 0.0,
            broke: false,
            daze: 0.0,
        }
    }
}
impl Default for DefBurst { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Meter, src/game/roomfight.ts:316)
#[derive(Clone, Copy, Debug)]
pub struct Meter {
    pub m: f64,
    pub seen_t: f64,
    pub glass_t: Option<f64>,
}
impl Meter {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Meter {
            m: 0.0,
            seen_t: 0.0,
            glass_t: None,
        }
    }
}
impl Default for Meter { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Op, src/game/roomfight.ts:317)
#[derive(Clone, Debug)]
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
    pub first: bool,
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
    pub reload_until: f64,
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
impl Op {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Op {
            gd: GuardId(0),
            meters: JsMap::new(),
            tgt: None,
            turn: None,
            pass: None,
            kept: None,
            track: 0.0,
            pre: false,
            ready_at: 0.0,
            n: 0.0,
            next: 0.0,
            lost_at: 0.0,
            fell: None,
            first: false,
            aim: [0.0; 3],
            ov: crate::roomfight::OpOv::zeroed(),
            step: None,
            hot_t: 0.0,
            step_t: 0.0,
            step_by: None,
            door_by: None,
            funnel_t: 0.0,
            line_t: 0.0,
            line_by: None,
            line_at: 0.0,
            wall_t: 0.0,
            step_until: 0.0,
            reload_until: 0.0,
            said_t: 0.0,
            rounds: 0.0,
            held_for_friend: 0.0,
            held_for_line: 0.0,
            block_t: 0.0,
            feet_sm: None,
            feet_t: None,
            short_seen: 0.0,
            short_since: 0.0,
            short_t: 0.0,
        }
    }
}
impl Default for Op { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:319 Op.turn, src/game/roomfight.ts:319)
#[derive(Clone, Copy, Debug)]
pub struct OpTurn {
    pub to: GuardId,
    pub at: f64,
    pub until: f64,
    pub pt: Option<V3>,
    pub t0: Option<f64>,
    pub door: Option<DoorId>,
}
impl OpTurn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpTurn {
            to: GuardId(0),
            at: 0.0,
            until: 0.0,
            pt: None,
            t0: None,
            door: None,
        }
    }
}
impl Default for OpTurn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:319 Op.pass, src/game/roomfight.ts:319)
#[derive(Clone, Copy, Debug)]
pub struct OpPass {
    pub h: GuardId,
    pub until: f64,
    pub no_shot: Option<bool>,
}
impl OpPass {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpPass {
            h: GuardId(0),
            until: 0.0,
            no_shot: None,
        }
    }
}
impl Default for OpPass { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:319 Op.kept, src/game/roomfight.ts:319)
#[derive(Clone, Copy, Debug)]
pub struct OpKept {
    pub h: GuardId,
    pub track: f64,
    pub t: f64,
}
impl OpKept {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpKept {
            h: GuardId(0),
            track: 0.0,
            t: 0.0,
        }
    }
}
impl Default for OpKept { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:322 Op.ov, src/game/roomfight.ts:322)
#[derive(Clone, Copy, Debug)]
pub struct OpOv {
    pub aim_at: V3,
    pub face: Option<f64>,
    pub walk: Option<bool>,
}
impl OpOv {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpOv {
            aim_at: [0.0; 3],
            face: None,
            walk: None,
        }
    }
}
impl Default for OpOv { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:324 Op.step, src/game/roomfight.ts:324)
#[derive(Clone, Copy, Debug)]
pub struct OpStep {
    pub mine: V3,
    pub goal: Option<V3>,
    pub speed: Option<f64>,
    pub strafe: Option<bool>,
    pub face: Option<f64>,
    pub door: bool,
}
impl OpStep {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpStep {
            mine: [0.0; 3],
            goal: None,
            speed: None,
            strafe: None,
            face: None,
            door: false,
        }
    }
}
impl Default for OpStep { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:331 Op.stepBy, src/game/roomfight.ts:331)
#[derive(Clone, Copy, Debug)]
pub struct OpStepBy {
    pub by: GuardId,
    pub t: f64,
    pub at: V3,
}
impl OpStepBy {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpStepBy {
            by: GuardId(0),
            t: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for OpStepBy { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:333 Op.doorBy, src/game/roomfight.ts:333)
#[derive(Clone, Copy, Debug)]
pub struct OpDoorBy {
    pub by: GuardId,
    pub t: f64,
    pub at: V3,
    pub door: DoorId,
}
impl OpDoorBy {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpDoorBy {
            by: GuardId(0),
            t: 0.0,
            at: [0.0; 3],
            door: DoorId(0),
        }
    }
}
impl Default for OpDoorBy { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum OpLineBy {
    Guard(GuardId),
    Player,
}

/// (TypeScript: FightNote, src/game/roomfight.ts:344)
#[derive(Clone, Debug)]
pub struct FightNote {
    pub t: f64,
    pub who: String,
    pub what: String,
}
impl FightNote {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FightNote {
            t: 0.0,
            who: String::new(),
            what: String::new(),
        }
    }
}
impl Default for FightNote { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoomFight, src/game/roomfight.ts:378)
#[derive(Clone, Debug)]
pub struct RoomFight {
    pub knobs: crate::knobs::RoomfightK,
    pub skill: JsMap<crate::roomfight::RoomFightSkillKey, crate::roomfight::FightSkill>,
    pub live: bool,
    pub blind_fire: bool,
    pub blind_rounds: f64,
    pub trans: JsMap<String, f64>,
    pub defs: Vec<RoomfightDefId>,
    pub ops: Vec<RoomfightOpId>,
    pub log: Vec<crate::roomfight::FightNote>,
    pub stats: crate::roomfight::RoomFightStats,
    pub rooms: Vec<RoomfightRoomId>,
    pub room_of_cell: JsMap<crate::Num, RoomfightRoomId>,
    pub fill_mark: Vec<u8>,
    pub called: JsSet<GuardId>,
    pub down_called: JsSet<GuardId>,
    pub rr: f64,
    pub hot_t: f64,
    pub reports: Vec<crate::roomfight::RoomFightReportsItem>,
    pub heard_ev: JsSet<u32>,
    pub sams: Vec<crate::roomfight::RoomFightSamsItem>,
    pub drill_said: bool,
}
impl RoomFight {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        RoomFight {
            knobs: crate::knobs::RoomfightK { on: true, evidence_secs: 12.0, sticky: 0.25, crowd: 0.2, fear_min: 0.15, fall_back: true, rally: true, fall_back_in: true, doorway: true, side_step: true, turn_walk: true, nearest: true, close_m: 2.5, no_shot: 0.4, close_lane: true, point_blank: 2.0, sam_behind: true, sam_beside: true, sam_travel: true, no_shot_go: 1.5, no_shot_pass: 4.0, keep_track: true, rally_k: 1.0, step_ears: true, arc_half_deg: 100.0, cover_keep: false, cover_half_deg: 45.0, cover_near_m: 6.0, team_hot: 45.0, cued: true, let_go: 0.8, step_look: true, step_look_secs: 5.0, door_ears: true, door_look_secs: 3.5, door_shots: false, door_shot_m: 8.0, sam_line: true, hold_short_secs: 4.0, pairs: 6.0, move_engage_m: 7.0, lamp: 0.7, preaim_deg: 12.0, sights: true, sights_foe: false, sights_deg: 45.0 },
            skill: { let mut t = JsMap::new(); t.set(crate::roomfight::RoomFightSkillKey::Operative, crate::roomfight::FightSkill { react: [0.12, 0.22], ready: 1.0, surprise: [0.35, 0.6], sector_deg: 50.0, settle_k: 0.4, spread_k: 0.5, preaim: 0.6, notice: 0.12, pair: 0.16, shift: [0.2, 0.3], hold: [f64::INFINITY, f64::INFINITY], glance: [0.0, 0.0], turn: 8.0, bang_break: 0.0, startle: [0.0, 0.0], daze: [0.0, 0.0], nerve: [2.0, 2.0], rally: [0.0, 0.0], rally_chance: 0.0, funnel: 0.35, line: 0.5 }); t.set(crate::roomfight::RoomFightSkillKey::Hostile, crate::roomfight::FightSkill { react: [0.4, 1.1], ready: 0.25, surprise: [0.5, 0.9], sector_deg: 40.0, settle_k: 1.5, spread_k: 1.0, preaim: 0.55, notice: 0.85, pair: 0.1, shift: [0.4, 0.9], hold: [5.0, 11.0], glance: [1.2, 2.6], turn: 3.0, bang_break: 0.3, startle: [0.4, 0.65], daze: [1.0, 2.0], nerve: [0.34, 0.7], rally: [1.5, 4.0], rally_chance: 0.75, funnel: f64::INFINITY, line: f64::INFINITY }); t },
            live: false,
            blind_fire: true,
            blind_rounds: 0.0,
            trans: JsMap::new(),
            defs: Vec::new(),
            ops: Vec::new(),
            log: Vec::new(),
            stats: crate::roomfight::RoomFightStats { op_rounds: 0.0, held_for_friend: 0.0, fell_back: 0.0, fall_back_out: 0.0, took_cover: 0.0, rallied: 0.0, surprised: 0.0, broke: 0.0, stepped: 0.0, startled: 0.0 },
            rooms: Vec::new(),
            room_of_cell: JsMap::new(),
            fill_mark: alloc::vec![0; 0],
            called: JsSet::new(),
            down_called: JsSet::new(),
            rr: 0.0,
            hot_t: -100.0,
            reports: Vec::new(),
            heard_ev: JsSet::new(),
            sams: Vec::new(),
            drill_said: false,
        }
    }
}
impl Default for RoomFight { fn default() -> Self { Self::blank() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoomFightSkillKey {
    Operative,
    Hostile,
}
impl RoomFightSkillKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoomFightSkillKey::Operative => "operative",
            RoomFightSkillKey::Hostile => "hostile",
        }
    }
}

/// (TypeScript: inline at src/game/roomfight.ts:388 RoomFight.stats, src/game/roomfight.ts:388)
#[derive(Clone, Copy, Debug)]
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
impl RoomFightStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomFightStats {
            op_rounds: 0.0,
            held_for_friend: 0.0,
            fell_back: 0.0,
            fall_back_out: 0.0,
            took_cover: 0.0,
            rallied: 0.0,
            surprised: 0.0,
            broke: 0.0,
            stepped: 0.0,
            startled: 0.0,
        }
    }
}
impl Default for RoomFightStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:396 RoomFight.reports, src/game/roomfight.ts:396)
#[derive(Clone, Copy, Debug)]
pub struct RoomFightReportsItem {
    pub by: GuardId,
    pub t: f64,
    pub from: Option<V3>,
    pub to: Option<V3>,
}
impl RoomFightReportsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomFightReportsItem {
            by: GuardId(0),
            t: 0.0,
            from: None,
            to: None,
        }
    }
}
impl Default for RoomFightReportsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:399 RoomFight.sams, src/game/roomfight.ts:399)
#[derive(Clone, Copy, Debug)]
pub struct RoomFightSamsItem {
    pub x: f64,
    pub z: f64,
    pub t: f64,
    pub v: [f64; 2],
    pub known: f64,
}
impl RoomFightSamsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomFightSamsItem {
            x: 0.0,
            z: 0.0,
            t: 0.0,
            v: [0.0; 2],
            known: 0.0,
        }
    }
}
impl Default for RoomFightSamsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:429 sightOn ->, src/game/roomfight.ts:429)
#[derive(Clone, Copy, Debug)]
pub struct RoomFightSightOn {
    pub visible: bool,
    pub rate: f64,
    pub dist: f64,
    pub sighting: bool,
}
impl RoomFightSightOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomFightSightOn {
            visible: false,
            rate: 0.0,
            dist: 0.0,
            sighting: false,
        }
    }
}
impl Default for RoomFightSightOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:444 watchOf ->, src/game/roomfight.ts:444)
#[derive(Clone, Debug)]
pub struct RoomFightWatchOf {
    pub at: V3,
    pub door: Option<String>,
    pub settled: Option<String>,
    pub glancing: bool,
    pub mode: crate::roomfight::DefMode,
}
impl RoomFightWatchOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomFightWatchOf {
            at: [0.0; 3],
            door: None,
            settled: None,
            glancing: false,
            mode: crate::roomfight::DefMode::Hold,
        }
    }
}
impl Default for RoomFightWatchOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum RoomFightFriendInLine {
    Guard(GuardId),
    Player,
}

/// (TypeScript: inline at src/game/roomfight.ts:1137 doorShot ->, src/game/roomfight.ts:1137)
#[derive(Clone, Copy, Debug)]
pub struct RoomFightDoorShot {
    pub by: GuardId,
    pub at: V3,
    pub door: DoorId,
}
impl RoomFightDoorShot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomFightDoorShot {
            by: GuardId(0),
            at: [0.0; 3],
            door: DoorId(0),
        }
    }
}
impl Default for RoomFightDoorShot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:1181 stepTo(patch), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct RoomFightStepToPatch {
    pub goal: Option<V3>,
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
impl RoomFightStepToPatch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomFightStepToPatch {
            goal: None,
            speed: None,
            face: None,
            strafe: None,
            aim_at: None,
            stance: None,
            stack_side: None,
            crouch: None,
            lean: None,
            upper: None,
            light: None,
            arrived: None,
            hot: None,
            flinch: None,
            direct: None,
            smoke: None,
            over: None,
        }
    }
}
impl Default for RoomFightStepToPatch { fn default() -> Self { Self::zeroed() } }

/// (src/game/roomfight.ts:88 COVER_BREAK)
pub const COVER_BREAK: bool = false;

/// (src/game/roomfight.ts:90 DOORSHOT_BREAK)
pub const DOORSHOT_BREAK: bool = false;

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
