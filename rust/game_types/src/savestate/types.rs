//! The types of `savestate`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct FirearmSnap {
    pub id: crate::weapons::WeaponId,
    pub chamber: f64,
    pub mag: Option<f64>,
    pub spare: Vec<f64>,
    pub shots_fired: f64,
    pub mode: crate::weapons::FireMode,
    pub bursts: f64,
    pub last_burst_t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct PlayerSnap {
    pub pos: [f64; 3],
    pub body_yaw: f64,
    pub aim_yaw: f64,
    pub aim_pitch: f64,
    pub crouch: bool,
    pub holstered: bool,
    pub slot: crate::game::SlotKind,
    pub throw_kind: crate::game::ThrowKind,
    pub canisters: f64,
    pub flashbangs: f64,
    pub frags: Option<f64>,
    pub wand: Option<bool>,
    pub big_kit: Option<crate::kit::BigKit>,
    pub kit_load: Option<Option<crate::kitload::KitWorn>>,
    pub own: Option<crate::savestate::OwnCharges>,
    pub pistol: crate::savestate::PlayerSnapPistol,
    pub rifle: crate::savestate::PlayerSnapRifle,
    pub cams: Option<Vec<crate::stickycam::StickySnap>>,
    pub cam_seq: Option<f64>,
    pub ocp_charge: f64,
    pub has_rifle: bool,
    pub rifle_gone: bool,
    pub ocp_hinted: bool,
    pub equip_t: f64,
    pub held_gait: Option<Vec<f64>>,
    pub close_on: Option<f64>,
    pub rip_u: Option<f64>,
    pub bash_at: Option<f64>,
    pub bash_lean: Option<f64>,
    pub stag_u: Option<f64>,
    pub hurt_at: Option<f64>,
    pub daz_u: Option<f64>,
    pub daz_at: Option<Vec<f64>>,
    pub aw: Option<Vec<f64>>,
    pub keys: Vec<String>,
    pub codes: Vec<String>,
    pub hits_left: f64,
    pub nv: bool,
    pub pace: f64,
    pub last_fire_t: f64,
    pub fire_cd: f64,
    pub noise: f64,
    pub visibility: f64,
    pub step_phase: f64,
    pub step_travel: Option<f64>,
    pub anim_phase: f64,
    pub wet: [f64; 3],
    pub bubble: Option<crate::savestate::Bubble>,
}

#[derive(Clone, Debug, Default)]
pub struct OwnCharges {
    pub charges: f64,
    pub walls: Option<f64>,
    pub doors: Option<Vec<[f64; 2]>>,
    pub spent: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct PlayerSnapPistol {
    pub id: crate::weapons::WeaponId,
    pub chamber: f64,
    pub mag: Option<f64>,
    pub spare: Vec<f64>,
    pub shots_fired: f64,
    pub mode: crate::weapons::FireMode,
    pub bursts: f64,
    pub last_burst_t: f64,
    pub light_on: bool,
}

#[derive(Clone, Debug, Default)]
pub struct PlayerSnapRifle {
    pub id: crate::weapons::WeaponId,
    pub chamber: f64,
    pub mag: Option<f64>,
    pub spare: Vec<f64>,
    pub shots_fired: f64,
    pub mode: crate::weapons::FireMode,
    pub bursts: f64,
    pub last_burst_t: f64,
    pub light_on: bool,
    pub att_idx: f64,
    pub rings: f64,
    pub ring_cd: f64,
    pub gun: Option<crate::weapons::WeaponId>,
    pub shells: Option<f64>,
    pub cams: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct Bubble {
    pub text: String,
    pub t: f64,
    pub dur: f64,
    pub radio: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum TaskSnap {
    Arm0(crate::savestate::TaskSnapArm0),
    CheckOn(crate::savestate::TaskSnapCheckOn),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TaskSnapArm0 {
    pub kind: crate::savestate::TaskSnapArm0Kind,
    pub pos: [f64; 3],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum TaskSnapArm0Kind {
    #[default] Breaker,
    Device,
    Rack,
}
impl TaskSnapArm0Kind {
    pub const fn as_str(self) -> &'static str {
        match self {
            TaskSnapArm0Kind::Breaker => "breaker",
            TaskSnapArm0Kind::Device => "device",
            TaskSnapArm0Kind::Rack => "rack",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TaskSnapCheckOn {
    pub pos: [f64; 3],
    pub who: f64,
    pub wp: f64,
    pub left: f64,
    pub t0: f64,
}

#[derive(Clone, Debug, Default)]
pub struct TalkSnap {
    pub stage: f64,
    pub presses: f64,
    pub said: Vec<String>,
    pub said_n: Vec<(String, f64)>,
    pub given: Vec<String>,
    pub last_t: f64,
    pub held_since: f64,
    pub abort_t: f64,
    pub script: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct RagSnap {
    pub x: Vec<f64>,
    pub rot: [f64; 4],
    pub sleeping: bool,
    pub own: Option<bool>,
}

#[derive(Clone, Debug, Default)]
pub struct GuardSnap {
    pub id: f64,
    pub route_i: f64,
    pub callsign: String,
    pub state: crate::game::GuardState,
    pub pos: [f64; 3],
    pub vel: [f64; 3],
    pub body_yaw: f64,
    pub aim_yaw: f64,
    pub aim_pitch: f64,
    pub crouch: f64,
    pub anim_phase: f64,
    pub turn: Option<Vec<f64>>,
    pub wp: f64,
    pub wait: f64,
    pub awareness: f64,
    pub last_known: Option<[f64; 3]>,
    pub last_seen_t: f64,
    pub found: bool,
    pub down_kind: Option<crate::game::DownKind>,
    pub down_by: Option<crate::game::DownBy>,
    pub keys: Vec<String>,
    pub missed_at: f64,
    pub missing_raised: bool,
    pub muzzle_down: bool,
    pub muzzle_clear_t: f64,
    pub muzzle_eval_t: f64,
    pub line_bark_t: f64,
    pub line_in_t: f64,
    pub leader_seen_t: f64,
    pub leader_seen_pos: [f64; 3],
    pub leader_still_t: f64,
    pub body_visits: f64,
    pub body_visit_t: f64,
    pub body_remarked: bool,
    pub body_closed: bool,
    pub body_fan: f64,
    pub body_duty: Option<crate::savestate::GuardSnapBodyDuty>,
    pub dropped: f64,
    pub gun_lost_at: f64,
    pub torch_lost_at: f64,
    pub light_lit_at_loss: bool,
    pub light_dead_until: f64,
    pub flashlight_on: bool,
    pub sweep: f64,
    pub beam_dir: [f64; 3],
    pub path: Vec<[f64; 3]>,
    pub path_i: f64,
    pub repath_t: f64,
    pub path_goal: Option<[f64; 3]>,
    pub weapon: crate::savestate::FirearmSnap,
    pub fire_cd: f64,
    pub react_t: f64,
    pub search_t: f64,
    pub look_phase: f64,
    pub speed: f64,
    pub step_phase: f64,
    pub heard_up_to: f64,
    pub died_at: f64,
    pub dazzled_until: f64,
    pub stagger_until: f64,
    pub hurt_at: Option<f64>,
    pub door_down_u: Option<f64>,
    pub last_pos: [f64; 3],
    pub stuck_t: f64,
    pub stuck_n: f64,
    pub killer_id: f64,
    pub saw_body_t: f64,
    pub alert_best: f64,
    pub alert_stall_t: f64,
    pub alert_ref: Option<[f64; 3]>,
    pub chase_t: f64,
    pub shot_blocked_t: f64,
    pub sight_pos: [f64; 3],
    pub sight_vel: [f64; 3],
    pub sight_t: f64,
    pub led_t: f64,
    pub fix_t: f64,
    pub fix_ref: Option<[f64; 3]>,
    pub led_at: Option<[f64; 3]>,
    pub word_t: Option<f64>,
    pub pie_last: f64,
    pub pie_last_t: f64,
    pub pie_prev: Option<[f64; 3]>,
    pub pie_prev_t: f64,
    pub stall_ref: Option<[f64; 3]>,
    pub stall_t: f64,
    pub task: Option<crate::savestate::TaskSnap>,
    pub drawn: bool,
    pub leader: f64,
    pub follow_t: f64,
    pub follow_goal: [f64; 3],
    pub post: Option<String>,
    pub witness: crate::game::Witness,
    pub talk: crate::savestate::TalkSnap,
    pub bubble: Option<crate::savestate::Bubble>,
    pub wet: [f64; 3],
    pub rag: Option<crate::savestate::RagSnap>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GuardSnapBodyDuty {
    pub body: f64,
    pub role: crate::savestate::GuardSnapBodyDutyRole,
    pub stage: crate::savestate::GuardSnapBodyDutyStage,
    pub t: f64,
    pub t0: f64,
    pub spot: Option<[f64; 3]>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum GuardSnapBodyDutyRole {
    #[default] Finder,
    Called,
}
impl GuardSnapBodyDutyRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            GuardSnapBodyDutyRole::Finder => "finder",
            GuardSnapBodyDutyRole::Called => "called",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum GuardSnapBodyDutyStage {
    #[default] Fan,
    To,
    Cover,
}
impl GuardSnapBodyDutyStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            GuardSnapBodyDutyStage::Fan => "fan",
            GuardSnapBodyDutyStage::To => "to",
            GuardSnapBodyDutyStage::Cover => "cover",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ItemSnap {
    pub kind: crate::throwables::ItemKind,
    pub pos: [f64; 3],
    pub yaw: f64,
    pub tumble: f64,
    pub heading: f64,
    pub fill: f64,
    pub life: f64,
    pub age: f64,
    pub fired: bool,
    pub emit_t: f64,
    pub who: f64,
    pub on: f64,
    pub on_pose: f64,
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
}

#[derive(Clone, Debug, Default)]
pub struct DoorSnap {
    pub name: String,
    pub angle: f64,
    pub vel: f64,
    pub quiet_t: f64,
    pub latched: bool,
    pub last_who: f64,
    pub since_bash: f64,
    pub closing: bool,
    pub locked: bool,
    pub lock_broken: bool,
    pub noticed: bool,
    pub noticed_t: f64,
    pub pick: f64,
    pub pick_idle: f64,
    pub pick_clock: f64,
    pub pick_n: f64,
    pub guard_t: f64,
    pub creak_armed: bool,
    pub pull_t: Option<f64>,
    pub press_t: Option<f64>,
    pub solid_t: Option<f64>,
    pub open_t: Option<f64>,
    pub passers: Option<f64>,
    pub swept: Option<bool>,
    pub shot: Option<f64>,
    pub off: Option<crate::doorblast::SlabSnap>,
    pub drive: Option<crate::doors::DoorDrive>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PropSnap {
    pub x: f64,
    pub z: f64,
    pub yaw: f64,
    pub vx: f64,
    pub vz: f64,
    pub w: f64,
    pub asleep: bool,
    pub idle: f64,
    pub last_who: f64,
    pub noise_t: f64,
    pub push_t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct TargetSnap {
    pub name: String,
    pub on: bool,
    pub broken: bool,
    pub disabled_until: f64,
    pub hand: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DecalSnap {
    pub kind: crate::render_decals::DecalKind,
    pub pos: [f64; 3],
    pub n: [f64; 3],
    pub t: [f64; 3],
    pub radius: f64,
    pub param: f64,
    pub seed: f64,
    pub pane: Option<f64>,
    pub door: Option<f64>,
    pub prop: Option<f64>,
    pub part: Option<f64>,
    pub look: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct ChronSnap {
    pub events: Vec<crate::chronicle::ChronEvent>,
    pub dropped: f64,
    pub counts: crate::chronicle::ChronCounts,
    pub questioned: Vec<String>,
    pub path: Vec<f64>,
    pub path_n: f64,
    pub path_every: f64,
    pub path_next: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RollcallSnap {
    pub next_at: f64,
    pub stage: crate::savestate::RollcallSnapStage,
    pub t0: f64,
    pub caller: f64,
    pub missing: f64,
    pub answered: bool,
    pub checks: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum RollcallSnapStage {
    #[default] Idle,
    Called,
    Asked,
}
impl RollcallSnapStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            RollcallSnapStage::Idle => "idle",
            RollcallSnapStage::Called => "called",
            RollcallSnapStage::Asked => "asked",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct AutosaveSnap {
    pub run_id: String,
    pub visited: Vec<String>,
    pub quiet_t: f64,
    pub last_save_t: f64,
    pub body_from: Option<[f64; 3]>,
}

#[derive(Clone, Debug, Default)]
pub struct SaveState {
    pub v: f64,
    pub saved_at: f64,
    pub label: String,
    pub kind: crate::savestate::SaveStateKind,
    pub level: crate::savestate::SaveStateLevel,
    pub mission_t: f64,
    pub stage: crate::game::MissionStage,
    pub run_id: String,
    pub time: f64,
    pub event_seq: f64,
    pub fx_seed: f64,
    pub ocp_cooldown: f64,
    pub called_in: bool,
    pub stood_down: bool,
    pub breaker_fix_t: f64,
    pub last_sting_t: f64,
    pub escalation: f64,
    pub escalation_t: f64,
    pub alarm: crate::savestate::SaveStateAlarm,
    pub cleared_at: Vec<(String, f64)>,
    pub clear_rooms: f64,
    pub blackout: crate::savestate::SaveStateBlackout,
    pub flags: crate::savestate::SaveStateFlags,
    pub fog: Option<crate::fogofwar::FogSnap>,
    pub quiet: Option<crate::quiet::QuietSnap>,
    pub lesson: Option<crate::lesson::LessonSnap>,
    pub player: crate::savestate::PlayerSnap,
    pub guards: Vec<crate::savestate::GuardSnap>,
    pub items: Vec<crate::savestate::ItemSnap>,
    pub doors: Vec<crate::savestate::DoorSnap>,
    pub props: Vec<crate::savestate::PropSnap>,
    pub props_time: f64,
    pub targets: Vec<crate::savestate::TargetSnap>,
    pub devices: Option<crate::devices::DevicesSnap>,
    pub mission: crate::game::Mission,
    pub chronicle: crate::savestate::ChronSnap,
    pub events: Vec<crate::game::GameEvent>,
    pub messages: Vec<crate::savestate::SaveStateMessagesItem>,
    pub rollcall: crate::savestate::RollcallSnap,
    pub needs: crate::needs::NeedsSnap,
    pub whistle: crate::whistle::WhistleSnap,
    pub tactics: crate::tactics::TacticsSnap,
    pub dialogue: crate::dialogue::DialogueSnap,
    pub weather: Option<crate::weather::WeatherSnap>,
    pub qrf: crate::savestate::SaveStateQrf,
    pub civilians: Option<crate::civilians::CivSnap>,
    pub walls: Option<crate::wallbreak::WallsSave>,
    pub lanes: Option<crate::lanes::LanesSave>,
    pub furn: Option<crate::furnblast::FurnSave>,
    pub bars: Option<crate::winbars::BarsSave>,
    pub air: Option<crate::air_dust::AirDustSnap>,
    pub decals: Vec<crate::savestate::DecalSnap>,
    pub autosave: crate::savestate::AutosaveSnap,
    pub glass_pen: Option<Vec<(f64, Vec<f64>)>>,
    pub gun_wall: Option<Vec<Vec<f64>>>,
    pub wall_hold: Option<crate::player::WallSnap>,
    pub calm_torch: Option<Vec<Option<Vec<f64>>>>,
    pub doze: Option<crate::doze::DozeSnap>,
    pub morale: Option<crate::morale::MoraleSnap>,
    pub runner: Option<crate::runner::RunnerSnap>,
    pub startle: Option<crate::startle::StartleSnap>,
    pub flinch: Option<crate::flinch::FlinchSnap>,
    pub darkdoor: Option<crate::darkdoor::DarkDoorSnap>,
    pub net_quiet: Option<crate::netquiet::NetQuietSnap>,
    pub power_out: Option<crate::powerout::PowerOutSnap>,
    pub rescue_check: Option<crate::rescuecheck::RescueCheckSnap>,
    pub attention: Option<Vec<Option<crate::attention::AttentionSnapItem>>>,
    pub picture: Option<crate::picture::PictureSnap>,
    pub dice: Option<crate::savestate::SaveStateDice>,
    pub hour: Option<crate::savestate::SaveStateHour>,
    pub forced: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SaveStateKind {
    #[default] Auto,
    Manual,
    Harness,
}
impl SaveStateKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SaveStateKind::Auto => "auto",
            SaveStateKind::Manual => "manual",
            SaveStateKind::Harness => "harness",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SaveStateLevel {
    pub fp: String,
    pub boxes: f64,
    pub routes: f64,
    pub doors: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SaveStateAlarm {
    pub episode: bool,
    pub pos: [f64; 3],
    pub placed: bool,
    pub apart_t: f64,
    pub raised_at: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SaveStateBlackout {
    pub active: bool,
    pub until: Option<f64>,
    pub since: f64,
    pub permanent: bool,
    pub restored_at: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SaveStateFlags {
    pub ai_enabled: bool,
    pub god_mode: bool,
    pub infinite_ammo: bool,
    pub player_invisible: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SaveStateMessagesItem {
    pub text: String,
    pub t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SaveStateQrf {
    pub loud_t: f64,
    pub word: Option<Vec<crate::qrf::ResponseWordSnap>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SaveStateDice {
    pub s: f64,
    pub n: f64,
    pub seed: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SaveStateHour {
    pub id: crate::hours::HourId,
    pub sun: f64,
    pub given: Option<bool>,
}

#[derive(Clone, Debug, Default)]
pub struct SaveOpts {
    pub label: Option<String>,
    pub kind: Option<crate::savestate::SaveOptsKind>,
    pub force: Option<bool>,
    pub run_id: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SaveOptsKind {
    Auto,
    Manual,
    Harness,
}
impl SaveOptsKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SaveOptsKind::Auto => "auto",
            SaveOptsKind::Manual => "manual",
            SaveOptsKind::Harness => "harness",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SaveStore {
}

#[derive(Clone, Debug, Default)]
pub struct SlotInfo {
    pub key: String,
    pub slot: crate::savestate::SlotInfoSlot,
    pub n: f64,
    pub label: String,
    pub kind: crate::savestate::SlotInfoKind,
    pub saved_at: f64,
    pub mission_t: f64,
    pub stage: String,
    pub run_id: String,
    pub bytes: f64,
    pub regret: Option<String>,
    pub mem: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SlotInfoSlot {
    #[default] Auto,
    Manual,
}
impl SlotInfoSlot {
    pub const fn as_str(self) -> &'static str {
        match self {
            SlotInfoSlot::Auto => "auto",
            SlotInfoSlot::Manual => "manual",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SlotInfoKind {
    #[default] Auto,
    Manual,
    Harness,
}
impl SlotInfoKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SlotInfoKind::Auto => "auto",
            SlotInfoKind::Manual => "manual",
            SlotInfoKind::Harness => "harness",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemSave {
    pub info: crate::savestate::SlotInfo,
    pub pic: Option<String>,
}

#[derive(Clone, Debug)]
pub struct MemShelf {
    pub m: JsMap<String, crate::savestate::MemSave>,
    pub warmed: f64,
}
impl MemShelf {
    /// what a new one holds
    pub fn blank() -> Self {
        MemShelf {
            m: JsMap::new(),
            warmed: 0.0,
        }
    }
}
impl Default for MemShelf { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, Default)]
pub struct SlotIndex {
    pub v: f64,
    pub next: f64,
    pub slots: JsMap<String, crate::savestate::SlotInfo>,
}

#[derive(Clone, Debug, Default)]
pub struct SaveResult {
    pub ok: bool,
    pub why: Option<String>,
    pub info: Option<crate::savestate::SlotInfo>,
    pub bytes: Option<f64>,
    pub state: Option<crate::savestate::SaveState>,
}

#[derive(Clone, Debug)]
pub struct Checkpoints {
    pub enabled: bool,
    pub run_id: String,
    pub visited: JsSet<String>,
    pub quiet_t: f64,
    pub body_from: Option<V3>,
    pub last_save_t: f64,
    pub pending: Option<crate::savestate::CheckpointsPending>,
    pub flash: Option<crate::savestate::CheckpointsFlash>,
    pub log: Vec<crate::savestate::CheckpointsLogItem>,
    pub stats: crate::savestate::CheckpointsStats,
    pub on_saved: Option<crate::FnField>,
    pub mission_ref: Option<u32>,
    pub stage_ref: crate::game::MissionStage,
    pub room_ref: String,
    pub drag_ref: Option<GuardId>,
    pub last_auto: Option<crate::savestate::CheckpointsLastAuto>,
    pub shelf: crate::savestate::MemShelf,
    pub picture: Option<crate::FnField>,
    pub on_put_back: Option<crate::FnField>,
    pub hand: bool,
    pub due: Option<String>,
    pub authored: Option<String>,
    /// a step asked for the mission over, in these words, as a load in a step asks
    pub again: Option<String>,
    /// which stand this game is of, under which the control holds what it keeps (0: none yet)
    pub stood: u32,
}
impl Checkpoints {
    /// what a new one holds
    pub fn blank() -> Self {
        Checkpoints {
            enabled: true,
            run_id: String::from(""),
            visited: JsSet::new(),
            quiet_t: 0.0,
            body_from: None,
            last_save_t: -1000000000.0,
            pending: None,
            flash: None,
            log: Vec::new(),
            stats: crate::savestate::CheckpointsStats { requests: 0.0, autosaves: 0.0, manual: 0.0, harness: 0.0, loads: 0.0, dropped: 0.0, refused: 0.0, deferred_frames: 0.0, regretted: 0.0 },
            on_saved: None,
            mission_ref: None,
            stage_ref: crate::game::MissionStage::Infiltrate,
            room_ref: String::from(""),
            drag_ref: None,
            last_auto: None,
            shelf: crate::savestate::MemShelf::blank(),
            picture: None,
            on_put_back: None,
            hand: false,
            due: None,
            authored: None,
            again: None,
            stood: 0,
        }
    }
}
impl Default for Checkpoints { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, Default)]
pub struct CheckpointsPending {
    pub why: String,
    pub label: String,
    pub at: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CheckpointsFlash {
    pub text: String,
    pub t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CheckpointsLogItem {
    pub t: f64,
    pub what: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CheckpointsStats {
    pub requests: f64,
    pub autosaves: f64,
    pub manual: f64,
    pub harness: f64,
    pub loads: f64,
    pub dropped: f64,
    pub refused: f64,
    pub deferred_frames: f64,
    pub regretted: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CheckpointsMissionRef {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CheckpointsDragRef {
}

#[derive(Clone, Debug, Default)]
pub struct CheckpointsLastAuto {
    pub key: String,
    pub t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SavestateModFpCacheValue {
    pub st: f64,
    pub fp: crate::savestate::SaveStateLevel,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SavestateModSNAPS {
    pub own: bool,
    pub take: crate::FnField,
    pub put: crate::FnField,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KeyMemSlot {
    Auto,
    Manual,
}
impl KeyMemSlot {
    pub const fn as_str(self) -> &'static str {
        match self {
            KeyMemSlot::Auto => "auto",
            KeyMemSlot::Manual => "manual",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct OwnCharges2 {
    pub own: Option<crate::savestate::OwnCharges>,
}

#[derive(Clone, Debug, Default)]
pub struct LoadStateOpts {
    pub label: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ApplyStateOpts {
    pub label: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct MemoryStore {
    pub map: JsMap<String, String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CheckpointsWriteSlot {
    Auto,
    Manual,
    Harness,
}
impl CheckpointsWriteSlot {
    pub const fn as_str(self) -> &'static str {
        match self {
            CheckpointsWriteSlot::Auto => "auto",
            CheckpointsWriteSlot::Manual => "manual",
            CheckpointsWriteSlot::Harness => "harness",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CheckpointsWriteMemSlot {
    Auto,
    Manual,
}
impl CheckpointsWriteMemSlot {
    pub const fn as_str(self) -> &'static str {
        match self {
            CheckpointsWriteMemSlot::Auto => "auto",
            CheckpointsWriteMemSlot::Manual => "manual",
        }
    }
}

#[derive(Clone, Debug)]
pub enum CheckpointsLoadFrom {
    Text(String),
    SaveState(crate::savestate::SaveState),
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SavestateMod {
    /// (src/game/savestate.ts:405 fpCache)
    pub fp_cache: JsMap<crate::ObjKey, crate::savestate::SavestateModFpCacheValue>,
    /// (src/game/savestate.ts:900 SNAPS)
    pub snaps: crate::savestate::SavestateModSNAPS,
    /// (src/game/savestate.ts:901 runSerial)
    pub run_serial: f64,
}
impl SavestateMod {
    pub fn blank() -> Self {
        SavestateMod {
            fp_cache: JsMap::new(),
            snaps: crate::savestate::SavestateModSNAPS::default(),
            run_serial: 0.0,
        }
    }
}

/// (src/game/savestate.ts:169 SAVE_VERSION)
pub const SAVE_VERSION: f64 = 1.0;

/// (src/game/savestate.ts:376 KEY_PREFIX)
pub const KEY_PREFIX: &str = "twopointsix.save.";

/// (src/game/savestate.ts:377 KEY_INDEX: a table nothing writes)
pub fn key_index() -> String { String::from("twopointsix.save.index") }

/// (src/game/savestate.ts:377 KEY_MANUAL: a table nothing writes)
pub fn key_manual() -> String { String::from("twopointsix.save.manual") }

/// (src/game/savestate.ts:377 KEY_AUTOSAVE_ON)
pub const KEY_AUTOSAVE_ON: &str = "twopointsix.autosave";

/// (src/game/savestate.ts:863 localStore: a table nothing writes)
pub fn local_store() -> crate::savestate::SaveStore { crate::savestate::SaveStore::default() }

/// (src/game/savestate.ts:1221 STAGE_RANK: a table nothing writes)
pub fn stage_rank() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("infiltrate"), 0.0); t.set(String::from("drive"), 1.0); t.set(String::from("exfil"), 2.0); t.set(String::from("done"), 3.0); t.set(String::from("failed"), 3.0); t } }
