//! The types of `game`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum ThrowKind {
    #[default] Smoke,
    Flash,
    Frag,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardScript {
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
    /// INVENTED: g.serial() at the one place a man's own script is made (squad.ts:145 order, when he has none). DoorPath.after asks by identity whether the script it lent to is still his (doorpath.ts:178). A lent script (frag, runner, morale, startle) stays 0: guards.ts:2143-2146 put his own back before anything can read it here
    pub serial: u32,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardScriptOver {
    pub aim_at: V3,
    pub face: Option<f64>,
    pub walk: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Interactable {
    pub kind: crate::game::InteractableKind,
    pub pos: V3,
    pub line1: String,
    pub line2: String,
    pub in_reach: bool,
    pub off: bool,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum InteractableKind {
    #[default] Cam,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum Verb {
    Push,
    Hammer,
    Rattle,
    Pick,
    Unlock,
    Kick,
    Code,
    Drive,
    Charge,
    Cancel,
    Pull,
    Ease,
    Nocode,
    Wand,
    Breach,
    Grab,
    Question,
    Choke,
    Whip,
    Comply,
    Drag,
    Drop,
    BreakerOff,
    BreakerOn,
    Blow,
    TakeCam,
    LightsOff,
    LightsOn,
    Lead,
    Stay,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum SlotKind {
    #[default] Pistol,
    Rifle,
    Smoke,
    Flash,
    Frag,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum GuardState {
    #[default] Patrol,
    Suspicious,
    Alert,
    Search,
    Dead,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum DownKind {
    #[default] Shot,
    Struck,
    Choked,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum DownBy {
    #[default] Guard,
    Player,
    World,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Witness {
    pub heard: crate::game::WitnessHeard,
    pub peak_awareness: f64,
    pub alerted_by: Option<crate::game::WitnessAlertedBy>,
    pub alert_t: f64,
    pub saw_act: Option<crate::game::WitnessSawAct>,
    pub saw_body: Option<crate::game::WitnessSawBody>,
    pub called_to_body: Option<crate::game::WitnessCalledToBody>,
    pub saw_held: Option<crate::game::WitnessSawHeld>,
    pub was_held: f64,
    pub dazzled: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WitnessHeard {
    pub shots: f64,
    pub last_shot_t: f64,
    pub kick: f64,
    pub bang: f64,
    pub small: f64,
    pub small_where: Option<V3>,
    pub loud_at: Option<V3>,
    pub last_t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum WitnessAlertedBy {
    Body,
    Held,
    Radio,
    Sound,
    Sight,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WitnessSawAct {
    pub kind: crate::game::WitnessSawActKind,
    pub victim: String,
    pub t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum WitnessSawActKind {
    #[default] Shot,
    Struck,
    Choked,
    Grab,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WitnessSawBody {
    pub victim: String,
    pub breathing: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WitnessCalledToBody {
    pub victim: String,
    pub breathing: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WitnessSawHeld {
    pub who: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum MissionStage {
    #[default] Drive,
    Infiltrate,
    Exfil,
    Done,
    Failed,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Mission {
    pub stage: crate::game::MissionStage,
    pub t0: f64,
    pub drive_t: f64,
    pub pulled: bool,
    pub alarm_on: bool,
    pub sweep_at: f64,
    pub spotted_t: f64,
    pub down_card_at: f64,
    pub hands_on: bool,
    pub stats: crate::game::MissionStats,
    /// INVENTED: g.serial() where g.mission is written whole (game.ts:1638, savestate.ts:812, demo.ts:152, 190, 313, 331): dialogue.ts:1717 and lesson.ts:320 ask "the same run?" by identity
    pub serial: u32,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct MissionStats {
    pub qrf_who: Option<String>,
    pub elapsed: f64,
    pub alerts: f64,
    pub spotted: f64,
    pub bodies: f64,
    pub shots: f64,
    pub kills: f64,
    pub knockouts: f64,
    pub keys: f64,
    pub blackout: bool,
    pub sandbox: bool,
    pub qrf: f64,
    pub qrf_at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GameEvent {
    pub announced: Option<bool>,
    pub kind: crate::director::EventKind,
    pub pos: V3,
    pub time: f64,
    pub loud: bool,
    pub level: Option<f64>,
    pub id: Option<f64>,
    pub who: Option<f64>,
    pub weapon: Option<crate::weapons::WeaponId>,
    pub from: Option<V3>,
    pub to: Option<V3>,
    pub cause: Option<crate::game::GameEventCause>,
    pub sw_at: Option<V3>,
    pub blind: Option<bool>,
    pub lure: Option<bool>,
    pub frag: Option<bool>,
    pub charge: Option<bool>,
    pub ears: Option<[V3; 2]>,
    /// INVENTED: dealt at birth, what the TypeScript asks by identity (heard sets, Rescue.noise, Def.noise0)
    pub serial: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum GameEventCause {
    Shot,
    Ocp,
}

/// What a game was stood with (src/game/stood.ts): its own record, set once at the stand, and the one rules read
#[derive(Clone, Copy, Debug, PartialEq, ToBytes)]
pub struct StoodWith {
    /// the weather the game is born in, None the level's own
    pub weather: Option<crate::weather_kinds::WeatherKind>,
    /// the weather is the address's for the life of the game
    pub locked: bool,
    /// the hour the stand lays over the level
    pub hour: crate::hours::HourId,
    /// that hour's sun's strength, and whether it was said
    pub sun: f64,
    pub sun_given: bool,
    /// the hour holds the level's own storm off
    pub dry: bool,
    /// a harness game stood plain, whose missions ask the solo card's question by kit_solo_stand
    pub solo_stand: bool,
    /// the way in asked for: 0 the level's own, k its k-th other, however many the level has (roomclear way_taken)
    pub way: usize,
}
impl StoodWith {
    /// what a game stood with nothing said for it holds: the level's own weather, at the level's own night
    pub fn blank() -> Self {
        StoodWith { weather: None, locked: false, hour: crate::hours::HourId::Night, sun: 1.0, sun_given: false, dry: true, solo_stand: false, way: 0 }
    }
}
impl Default for StoodWith { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes, Blank)]
#[bytes(parts)]
pub struct Game {
    /// what this game was stood with
    pub stood: crate::game::StoodWith,
    /// the solo night's card: your own kit on the plain night's player and on every fresh one
    pub solo_card: bool,
    /// the Team & kit picture's stage: a host's, round the tick, and read by no rule of the game. Only Rust uses it
    /// (the kit picture's, which no mission played has: in no bytes)
    #[bytes(skip)]
    pub kit_stage: Option<alloc::boxed::Box<crate::kitstage::KitStage>>,
    pub col: crate::scene_collision::StaticCollision,
    #[is(PlayerId(0))] pub player: PlayerId,
    pub guards: Vec<GuardId>,
    pub civilians: Vec<GuardId>,
    pub targets: Vec<FixtureId>,
    pub events: Vec<crate::game::GameEvent>,
    pub search_probe_n: f64,
    pub time: f64,
    pub dice: Option<crate::math_dice::SimDice>,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    #[is(crate::Opaque)] pub men: crate::Opaque,
    pub light_home: crate::sim_lightshome::LightHome,
    pub pose_sets: Vec<PoseSetId>,
    pub falls: crate::sim_fallshome::FallStore,
    #[is(true)] pub ai_enabled: bool,
    pub god_mode: bool,
    pub infinite_ammo: bool,
    pub show_debug: bool,
    #[is(-1.0)] pub spectate: f64,
    pub player_invisible: bool,
    pub threat_proximity: f64,
    #[is(-100.0)] pub torch_on_t: f64,
    pub aim_point: V3,
    pub aim_target: Option<FixtureId>,
    pub aim_guard: Option<GuardId>,
    pub aim_friend: Option<GuardId>,
    pub dyn_boxes: Vec<AnyBoxId>,
    pub rings: Vec<crate::launcher::Ring>,
    pub shells: Vec<crate::he40::Shell>,
    #[is(true)] pub fn2000: bool,
    pub doors: crate::doors::Doors,
    pub door_deals: Vec<crate::doorswing::SwingDeal>,
    pub props: crate::props::PropSystem,
    pub use_door: Option<DoorId>,
    pub reach_interior: Option<Vec<u8>>,
    pub hover: Option<crate::game::Interactable>,
    pub interactables: Vec<crate::game::Interactable>,
    pub nearby: Vec<crate::game::Interactable>,
    pub aim_snap: Option<V3>,
    pub act: Option<crate::game::Interactable>,
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    pub puppet: Option<crate::game::GamePuppet>,
    pub tmp_v: V3,
    pub called_in: bool,
    pub stood_down: bool,
    pub escalation: f64,
    #[is(-1.0)] pub escalation_t: f64,
    #[is(crate::game::GameAlarm { episode: false, pos: [0.0, 0.0, 0.0], placed: false, apart_t: 0.0, raised_at: -100.0 })] pub alarm: crate::game::GameAlarm,
    pub clearing: Option<crate::squad::Clearing>,
    pub clear_rooms: f64,
    pub cleared_at: JsMap<String, f64>,
    pub beacons: Vec<crate::game::GameBeaconsItem>,
    pub items: crate::throwables::Throwables,
    pub fx: Vec<crate::game::GameFxItem>,
    #[is(crate::sparks::Sparks { pool: alloc::vec![crate::sparks::Spark { pos: [0.0, -10.0, 0.0], vel: [0.0, 0.0, 0.0], age: 0.0, life: 0.0, alive: false, gen_: 0.0 }; 96], live: 0.0, params: crate::sparks::SparkParams { gravity: 16.0, drag: 1.6, hold: 0.15, brightness: 2.6, max_live: 96.0, emissive: 25.0, thickness: 0.002, shutter: 0.017, min_streak: 0.02, collide: true } })] pub sparks: crate::sparks::Sparks,
    pub tracers: crate::tracers::Tracers,
    pub weather: crate::weather::Weather,
    #[is(1.0)] pub fx_seed: f64,
    pub ocp_cooldown: f64,
    #[is(-100.0)] pub last_sting_t: f64,
    pub event_seq: f64,
    pub messages: Vec<crate::game::GameMessagesItem>,
    #[is(crate::game::Mission { t0: 0.0, stage: crate::game::MissionStage::Infiltrate, drive_t: -1.0, pulled: false, alarm_on: false, sweep_at: -1.0, spotted_t: -100.0, down_card_at: -1.0, hands_on: false, serial: 0, stats: crate::game::MissionStats { qrf_at: -1.0, ..crate::game::MissionStats::default() } })] pub mission: crate::game::Mission,
    pub chronicle: crate::chronicle::Chronicle,
    pub qrf: crate::qrf::Qrf,
    pub devices: crate::devices::Devices,
    pub destr: crate::destruction::Destruction,
    pub wind: crate::wind::Wind,
    pub air_dust: crate::air_dust::AirDust,
    pub moths: crate::moths::Moths,
    pub glass_break: crate::glassbreak::GlassBreak,
    pub shards: crate::shards::GlassShards,
    pub director: crate::director::Director,
    /// THE HOST'S OWN CAMERA AND KEYS, handed to the game once, at its stand, by every host alike (game.ts hostCam; the native start: game_start `host_new`).
    /// The host's: on the wire's hosts-own list, in no state word. Named here for what reaches for them: a level script's show, a rails level's first
    /// view, the view's matrix between two ticks.
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    pub host_cam: Option<CameraId>,
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    pub host_input: Option<InputId>,
    pub night_breach: crate::nightbreach::NightBreach,
    pub quiet: crate::quiet::QuietMission,
    pub lesson: crate::lesson::Lesson,
    pub rails: crate::rails::Rails,
    pub par: crate::par::Par,
    pub recap: crate::deathrecap::Recap,
    pub verdict: crate::verdict::Verdict,
    pub room_clear: crate::roomclear::RoomClearScenario,
    pub orders: crate::orders::SquadOrders,
    pub fog_of_war: crate::fogofwar::FogOfWar,
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    pub fog_picture: crate::fogofwar::FogPicture,
    pub fight: crate::roomfight::RoomFight,
    pub cues: crate::cues::Cues,
    pub squad_net: crate::squadnet::SquadNet,
    pub cover: crate::cover::Cover,
    pub entry: crate::entrypoints::EntryPoints,
    pub cleared: crate::cleared::ClearedRooms,
    pub rescue: crate::rescue::Rescue,
    pub door_tools: crate::doortools::DoorTools,
    pub pair: crate::pairorders::PairOrders,
    pub work: crate::workthings::WorkThings,
    pub wall_charge: crate::wallcharge::WallCharge,
    pub door_path: crate::doorpath::DoorPath,
    /// (this machine's own saves: in no bytes)
    #[bytes(skip)]
    pub saves: crate::savestate::Checkpoints,
    pub rack_targets: Vec<FixtureId>,
    #[is(crate::game::GameTune { devices: true, lasers: true, camera_suspicion_secs: 0.8, camera_alarm_secs: 2.0, device_cones: true, guard_walk: 1.15, guard_investigate: 1.75, guard_chase: 2.7, player_walk: 1.55, player_sprint: 4.7, player_crouch: 1.3, player_light: 60.0, flash_intensity: 70.0, flash_inner: 7.0, flash_outer: 19.0, detect_rate: 1.6, ocp_duration: 9.0, ocp_notice: crate::game::GameTuneOcpNotice::Glance, vis_slope: 0.2, vis_floor: 0.03, floor_materials: true, muzzle_discipline: true, suppression: true, carbine_guards: crate::game::CarbineDeal::None, suppress_first: true, tracers: true, aim_model: true, teams: true, needs: true, tactics: true, flank_wait: 6.0, lone_wait: 8.0, door_danger: 25.0, door_wait: 6.0, chaos: 0.5, wind: 1.0, moths: true, glass_break: true, glass_shards: true, wall_break: true, debris_impulses: true, qrf: true, qrf_eta: 75.0, qrf_size: 4.0, perimeter: true, perimeter_eta: 45.0, perimeter_size: 2.0, qrf_silent: true, qrf_squad: true, civilians: true, civilian_kill_fails: false, body_light: true })] pub tune: crate::game::GameTune,
    pub engine: crate::host_api::HostApi,
    pub level: crate::scene_level::Level,
    #[is(RigId(0))] pub rig: RigId,
    pub smoke: crate::smoke_types::SmokeSystemLike,
    /// nobody plays and the floor is looked at (the title's reel): no run is kept, counted or spoken to
    pub aside: bool,
    /// (the host's camera: in no bytes)
    #[bytes(skip)]
    pub view_proj: Option<crate::game::GameViewProj>,
    pub rolls: Vec<crate::game::Roll>,
    pub roll_seq: f64,
    pub restart_cut: bool,
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    #[is(SeatId(0))] pub seat: SeatId,
    pub seats: Vec<SeatId>,
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    pub local_seat: Option<SeatId>,
    pub ticking: bool,
    /// (worked out again from the rest, or only counted: in no bytes)
    #[bytes(skip)]
    pub alone_own: crate::game::AsideOwn,
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    pub cmd_buf: Option<crate::cmd::PlayerCmd>,
    pub door_owns: Vec<Option<usize>>,
    pub door_was: Vec<f64>,
    pub door_hand_recs: JsMap<CharId, crate::game::GameDoorHandRecsValue>,
    /// INVENTED: what g.serial() deals
    #[is(1)] pub serial_next: u32,
    /// INVENTED: a second cast of the squad is on Game (squadcast::with_cast): seats.ts:240 CAST_VIEW
    pub in_cast: bool,
    /// INVENTED: the rows box_frame has dealt since boxes_frame_reset
    pub box_frame_rows: Vec<FrameBoxId>,
    /// INVENTED: rows box_frame may deal again
    pub box_frame_free: Vec<FrameBoxId>,
    /// INVENTED: what the bridge carries of poses that have no place in the pool: Pose.at is Pose::LOOSE + the row
    pub loose_poses: Vec<crate::anim_skeleton::LoosePose>,
    /// what this step did that the host shows. The host drains it (the face clears it before a step)
    /// (a step's output, and no state: in no bytes)
    #[bytes(skip)]
    pub out: Vec<crate::out_events::OutEvent>,
    /// INVENTED: how many with_seat calls stand open: seats.ts tells a seat's view from the bare Game by identity (`rawGame(g) !== g`). Zero between ticks, so the bridge does not carry it
    pub view_depth: u32,
    /// state.ts:136 SimState.stamp, for what is made from the block (the fog's grid, the air): a native game has one block, and whatever writes it wholesale raises this
    /// the table of these things: never shrinks
    pub ch: Tab<CharId, crate::character::Character>,
    /// the table of these things: never shrinks
    pub dr: Tab<DoorId, crate::doors::Door>,
    /// the table of these things: never shrinks
    pub gd: Tab<GuardId, crate::game::Guard>,
    /// the table of these things: never shrinks
    pub pl: Tab<PlayerId, crate::game::Player>,
    /// the table of these things: never shrinks
    pub cv: Tab<CivId, crate::civilians::Civ>,
    /// the table of these things: never shrinks
    pub fa: Tab<FirearmId, crate::weapons::Firearm>,
    /// the other tables of things
    pub tab: Tabs,
    /// the knob tables, at the page's values
    #[is(crate::knobs::Knobs::page())] pub k: crate::knobs::Knobs,
    /// each module's own state
    pub m: Mods,
    /// the counters kept for the checks
    /// (worked out again from the rest, or only counted: in no bytes)
    #[bytes(skip)]
    pub stats: Stats,
    /// which game of rust/sim's this is: every wrapper over rust/sim reads it, and the game's die lives there
    /// (which of rust/sim's slots it stands in is the host's: in no bytes)
    #[bytes(skip)]
    pub sim: usize,
    /// who has each man's body this tick, and who was asked (control/): written by the arbiter, read by the debugger's lines; not on the wire
    #[is(crate::owned::Owned::new())] pub ctl: crate::owned::Owned,
    /// the marks on the world (render_decals set.rs): what the picture draws of what happened; no rule reads them, and they are not on the wire
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    #[is(crate::owned::Owned::new())] pub mark_set: crate::owned::Owned,
    /// the ray grid (scene_boxes `RayGrid`): made from the statics as a tick opens, so it is not on the wire
    /// (made from the statics: in no bytes)
    #[bytes(skip)]
    #[is(crate::owned::Owned::new())] pub ray_grid: crate::owned::Owned,
    /// the stages to come of a wall charge's feel, the picture's half (blast_feel picture.rs)
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    #[is(crate::owned::Owned::new())] pub feel: crate::owned::Owned,
    /// this tick's count of keyed draws a (site, man) (dice.rs): empty between ticks; not on the wire
    pub keyed: crate::dice::Keyed,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GamePuppet {
    pub goal: Option<V3>,
    pub aim: Option<V3>,
    pub crouch: Option<bool>,
    pub walk: Option<f64>,
    pub fixtures: Option<bool>,
    pub target: Option<GuardId>,
    pub throw_at: Option<V3>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GameAlarm {
    pub episode: bool,
    pub pos: V3,
    pub placed: bool,
    pub apart_t: f64,
    pub raised_at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GameBeaconsItem {
    pub light: LightId,
    pub phase: f64,
    pub target: FixtureId,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GameFxItem {
    pub box_: FrameBoxId,
    pub ttl: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct GameMessagesItem {
    pub text: String,
    pub t: f64,
    /// INVENTED by a review: see the proposals
    pub serial: u32,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GameTune {
    pub devices: bool,
    pub lasers: bool,
    pub camera_suspicion_secs: f64,
    pub camera_alarm_secs: f64,
    pub device_cones: bool,
    pub guard_walk: f64,
    pub guard_investigate: f64,
    pub guard_chase: f64,
    pub player_walk: f64,
    pub player_sprint: f64,
    pub player_crouch: f64,
    pub player_light: f64,
    pub flash_intensity: f64,
    pub flash_inner: f64,
    pub flash_outer: f64,
    pub detect_rate: f64,
    pub ocp_duration: f64,
    pub ocp_notice: crate::game::GameTuneOcpNotice,
    pub vis_slope: f64,
    pub vis_floor: f64,
    pub floor_materials: bool,
    pub muzzle_discipline: bool,
    pub suppression: bool,
    pub carbine_guards: crate::game::CarbineDeal,
    pub suppress_first: bool,
    pub tracers: bool,
    pub aim_model: bool,
    pub teams: bool,
    pub needs: bool,
    pub tactics: bool,
    pub flank_wait: f64,
    pub lone_wait: f64,
    pub door_danger: f64,
    pub door_wait: f64,
    pub chaos: f64,
    pub wind: f64,
    pub moths: bool,
    pub glass_break: bool,
    pub glass_shards: bool,
    pub wall_break: bool,
    pub debris_impulses: bool,
    pub qrf: bool,
    pub qrf_eta: f64,
    pub qrf_size: f64,
    pub perimeter: bool,
    pub perimeter_eta: f64,
    pub perimeter_size: f64,
    pub qrf_silent: bool,
    pub qrf_squad: bool,
    pub civilians: bool,
    pub civilian_kill_fails: bool,
    pub body_light: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum GameTuneOcpNotice {
    #[default] Glance,
    Investigate,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CarbineDeal {
    #[default] None,
    Reyes,
    All,
}

#[derive(Clone, Debug)]
pub enum GameViewProj {
    Array(Vec<f64>),
    Float32Array(Vec<f32>),
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Roll {
    pub p: PropId,
    pub x: f64,
    pub z: f64,
    pub k: f64,
    pub move_t: f64,
    pub voice: f64,
}

#[derive(Clone, Copy, Debug, Blank)]
pub struct AsideOwn {
    pub lens: Option<CameraId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GameDoorHandRecsValue {
    pub door: DoorId,
    pub t: f64,
    pub t0: f64,
    pub crack: bool,
    pub hand: f64,
    pub move_: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, ToBytes)]
pub enum Foe {
    Guard(GuardId),
    Player,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum PairId {
    #[default] Red,
    Blue,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum Temperament {
    #[default] Steady,
    Nervy,
    Hard,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MissionRating {
    Ghost,
    Panther,
    Assault,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Player {
    #[is(CharId(0))] pub ch: CharId,
    pub crouch: bool,
    pub aim_held: bool,
    #[is(-10.0)] pub last_fire_t: f64,
    pub fire_cd: f64,
    pub backpedal: bool,
    pub sprinting: bool,
    pub dash: bool,
    pub friend_low: bool,
    #[is(-10.0)] pub friend_t: f64,
    #[is([0.0, 0.0])] pub shoved: [f64; 2],
    #[is(-1.0)] pub shoved_t: f64,
    #[is([0.0, 0.0])] pub shoved_from: [f64; 2],
    pub haul: Option<crate::hands::Haul>,
    pub door_move: Option<crate::hands::DoorMove>,
    pub dragging: Option<GuardId>,
    pub drag_yaw: f64,
    pub drag_noise_t: f64,
    pub drag_stuck_t: f64,
    pub noise: f64,
    pub takedown: Option<crate::game::PlayerTakedown>,
    pub holding: Option<crate::player::Hold>,
    pub catch_due: Option<f64>,
    pub bubble: Option<crate::game::PlayerBubble>,
    pub speed_sm: f64,
    pub visibility: f64,
    pub irradiance: f64,
    pub hit_flash: f64,
    /// his hands were on a man last frame (the partner's word goes as they first are)
    pub pair_hands_was: bool,
    #[is(-1.0)] pub stagger_until: f64,
    #[is(-1.0)] pub dazzled_until: f64,
    pub dazzled_at: Option<V3>,
    pub stun_posed: bool,
    #[is(-1.0)] pub hurt_at: f64,
    #[is(-10.0)] pub door_bash_at: f64,
    pub door_bash_lean: f64,
    #[is(-1.0)] pub door_rip_until: f64,
    pub door_kick_tail: bool,
    pub door_head: f64,
    #[is(-1.0)] pub door_head_t: f64,
    pub sup: crate::suppression::Sup,
    pub aim: crate::aim::AimState,
    pub aimw: crate::aimweight::AimWeight,
    #[is(1.0)] pub hits_left: f64,
    pub down: bool,
    #[is(FirearmId(0))] pub pistol: FirearmId,
    pub ocp: crate::weapons::Ocp,
    #[is(4.0)] pub canisters: f64,
    #[is(2.0)] pub flashbangs: f64,
    #[is(2.0)] pub frags: f64,
    #[is(crate::game::SlotKind::Pistol)] pub slot: crate::game::SlotKind,
    #[is(crate::game::ThrowKind::Smoke)] pub throw_kind: crate::game::ThrowKind,
    #[is(FirearmId(0))] pub rifle: FirearmId,
    pub has_rifle: bool,
    pub equip_t: f64,
    pub setup: Option<crate::game::PlayerSetup>,
    pub setup_hand: Option<crate::game::PlayerSetupHand>,
    pub setup_key: Option<crate::game::PlayerSetupKey>,
    pub rifle_gone: bool,
    pub kit_was: Option<Option<crate::character::KitItem>>,
    pub kit_can: Option<crate::game::ThrowKind>,
    pub launch_held: Option<crate::weapons::RifleAttachment>,
    #[is(-1.0)] pub kit_owed: f64,
    pub kit_owed_for: Option<crate::game::PlayerKitOwedFor>,
    pub sling_rest: Option<crate::game::PlayerSlingRest>,
    pub ocp_hinted: bool,
    #[is(true)] pub holstered: bool,
    pub keys: JsSet<String>,
    pub codes: JsSet<String>,
    pub coding: Option<DoorId>,
    pub code_t: f64,
    pub breaker_move: Option<crate::game::PlayerBreakerMove>,
    pub hand_wait: f64,
    pub near_sel: i32,
    #[is(String::from(""))] pub near_key: String,
    #[is(crate::game::PlayerSel { down_t: -1.0, open: false, moved: false, wheel_acc: 0.0, run_key: String::from(""), move_latch: 0.0, word: String::from(""), word_t: -1000000000.0, file_t: -1000000000.0, ring: None })] pub sel: crate::game::PlayerSel,
    #[is(1.0)] pub pace: f64,
    pub nv: bool,
    pub nv_amount: f64,
    #[is(LightId(0))] pub light: LightId,
    pub throw_eaten: bool,
    pub throw_held: bool,
    #[is(crate::game::ThrowKind::Smoke)] pub held_kind: crate::game::ThrowKind,
    pub throw_preview: Option<crate::throwables::ThrowSolution>,
    pub pending_throw: Option<crate::game::PlayerPendingThrow>,
    pub easing: Option<DoorId>,
    pub ease_stall_t: f64,
    pub ease_best: f64,
    pub picking: Option<DoorId>,
    pub kick: Option<crate::game::PlayerKick>,
    pub f_held: bool,
    pub wand: bool,
    pub wanding: Option<crate::wand::Wanding>,
    #[is(crate::kit::BigKit::None)] pub big_kit: crate::kit::BigKit,
    pub kit_load: Option<crate::kitload::KitWorn>,
    pub cams: Vec<StickyCamId>,
    pub cam_seq: f64,
    pub charging: Option<crate::game::PlayerCharging>,
    pub work_stow: Option<crate::game::PlayerWorkStow>,
    pub wall: Option<crate::player::WallHold>,
    pub safe: Option<V3>,
    pub step_phase: f64,
    #[is(1.0)] pub step_travel: f64,
    pub step_owed: f64,
    #[is(-1.0)] pub step_at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlayerTakedown {
    pub g: GuardId,
    pub t: f64,
    pub kind: Option<crate::close::CloseKind>,
    pub from: Option<f64>,
    pub alert: Option<bool>,
    pub yaw: Option<f64>,
    pub at: Option<V3>,
    pub lethal: Option<bool>,
    pub loud: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlayerBubble {
    pub text: String,
    pub t: f64,
    pub dur: f64,
    pub radio: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlayerSetup {
    pub gun: crate::slotsetup::SetupGun,
    pub t: f64,
    pub seen: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlayerSetupHand {
    pub gun: Option<crate::slotsetup::SetupGun>,
    pub t: f64,
    pub own: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlayerSetupKey {
    pub gun: crate::slotsetup::SetupGun,
    pub t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum PlayerKitOwedFor {
    Pistol,
    Rifle,
    Smoke,
    Flash,
    Frag,
    Can,
    Knife,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlayerSlingRest {
    pub a: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlayerBreakerMove {
    pub on: bool,
    pub t: f64,
    pub thrown: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlayerSel {
    pub down_t: f64,
    pub open: bool,
    pub moved: bool,
    pub wheel_acc: f64,
    pub run_key: String,
    pub move_latch: f64,
    pub word: String,
    pub word_t: f64,
    pub file_t: f64,
    pub ring: Option<crate::game::PlayerSelRing>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlayerSelRing {
    pub ax: f64,
    pub ay: f64,
    pub z: f64,
    pub t0: f64,
    pub lit: f64,
    pub armed: bool,
    pub free: Option<bool>,
    pub more: Option<bool>,
    pub at: Option<V3>,
    pub ox: Option<f64>,
    pub oy: Option<f64>,
    pub mx: Option<f64>,
    pub my: Option<f64>,
    pub clear: Option<bool>,
    pub pt: Option<f64>,
    pub key: String,
    pub whistle: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlayerPendingThrow {
    pub t: f64,
    pub sol: crate::throwables::ThrowSolution,
    pub kind: crate::game::ThrowKind,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlayerKick {
    pub door: DoorId,
    pub spot: V3,
    pub yaw: f64,
    pub run: Option<bool>,
    pub tool: Option<crate::game::PlayerKickTool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlayerKickTool {
    pub kind: crate::game::PlayerKickToolKind,
    pub side: f64,
    pub t: f64,
    pub lead: Option<f64>,
    pub n: f64,
    pub need: f64,
    pub next: f64,
    pub done: f64,
    pub gave: Option<bool>,
    pub moves: Option<crate::game::PlayerKickToolMoves>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PlayerKickToolKind {
    #[default] Shotgun,
    Hammer,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlayerKickToolMoves {
    pub arrived: bool,
    pub at: V3,
    pub blows: Vec<f64>,
    pub end: f64,
    pub via: Option<V3>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlayerCharging {
    pub door: DoorId,
    pub side: f64,
    pub t0: f64,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlayerWorkStow {
    pub drew: bool,
    pub t: f64,
    pub down: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Talk {
    pub stage: f64,
    pub presses: f64,
    pub said: JsSet<String>,
    pub said_n: JsMap<String, f64>,
    pub given: JsSet<String>,
    pub last_t: f64,
    pub held_since: f64,
    pub abort_t: f64,
    pub last: Option<crate::game::TalkLast>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct TalkLast {
    pub id: String,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub enum GuardTask {
    Arm0(crate::game::GuardTaskArm0),
    CheckOn(crate::game::GuardTaskCheckOn),
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardTaskArm0 {
    pub kind: crate::game::GuardTaskArm0Kind,
    pub pos: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum GuardTaskArm0Kind {
    #[default] Breaker,
    Device,
    Rack,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardTaskCheckOn {
    pub pos: V3,
    pub who: GuardId,
    pub wp: f64,
    pub left: f64,
    pub t0: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct UnitInfo {
    pub light: bool,
    pub pair: Option<crate::game::PairId>,
    pub seat: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Guard {
    #[is(CharId(0))] pub ch: CharId,
    #[is(crate::game::GuardState::Patrol)] pub state: crate::game::GuardState,
    pub callsign: String,
    #[is(crate::teams::Team::Guard)] pub team: crate::teams::Team,
    /// may name a man who has left: a GuardId stays readable, tables never shrink
    #[is(Some(crate::game::Foe::Player))] pub foe: Option<crate::game::Foe>,
    pub faces: Option<f64>,
    #[is(RouteId(0))] pub route: RouteId,
    pub unit: Option<crate::game::UnitInfo>,
    pub civ: Option<CivId>,
    pub trained: bool,
    pub skill: Option<crate::game::GuardSkill>,
    pub visibility: f64,
    pub irradiance: f64,
    pub route_i: i32,
    pub wp: usize,
    pub wait: f64,
    pub awareness: f64,
    /// written whole. doze.ts:244 asks whether a new fix was written: see fix_stamp
    pub last_known: Option<V3>,
    #[is(-100.0)] pub last_seen_t: f64,
    pub covers: Option<V3>,
    #[is(-100.0)] pub word_t: f64,
    pub led_at: Option<V3>,
    pub hold: bool,
    pub pinned: bool,
    pub god: bool,
    pub held: Option<crate::game::GuardHeld>,
    pub bubble: Option<crate::game::GuardBubble>,
    /// lines he said in the same frame as the one on his bubble, shown after it, oldest first
    pub bubble_next: Vec<crate::game::GuardBubble>,
    pub found: bool,
    pub down_kind: Option<crate::game::DownKind>,
    pub down_by: Option<crate::game::DownBy>,
    #[is(crate::game::Witness { heard: crate::game::WitnessHeard { shots: 0.0, last_shot_t: -1.0, kick: 0.0, bang: 0.0, small: 0.0, small_where: None, loud_at: None, last_t: -1.0 }, peak_awareness: 0.0, alerted_by: None, alert_t: -1.0, saw_act: None, saw_body: None, called_to_body: None, saw_held: None, was_held: 0.0, dazzled: 0.0 })] pub witness: crate::game::Witness,
    #[is(GameTalkId(0))] pub talk: GameTalkId,
    pub keys: JsSet<String>,
    #[is(-1.0)] pub missed_at: f64,
    pub missing_raised: bool,
    pub muzzle_down: bool,
    pub muzzle_clear_t: f64,
    #[is(-1.0)] pub muzzle_eval_t: f64,
    #[is(-100.0)] pub line_bark_t: f64,
    pub line_in_t: f64,
    pub line_side: f64,
    #[is(-1.0)] pub line_gain: f64,
    pub line_step_m: f64,
    pub line_stuck_t: f64,
    pub line_fail: Option<crate::game::GuardLineFail>,
    #[is(-1.0)] pub leader_seen_t: f64,
    pub leader_seen_pos: V3,
    pub body_visits: f64,
    #[is(-1.0)] pub body_visit_t: f64,
    pub body_remarked: bool,
    pub body_closed: bool,
    pub body_fan: f64,
    pub body_duty: Option<crate::game::GuardBodyDuty>,
    pub dropped: Option<ItemId>,
    #[is(f64::NEG_INFINITY)] pub handle_gate_t: f64,
    pub stays_in_rooms: bool,
    pub fled: bool,
    #[is(-1.0)] pub gun_lost_at: f64,
    #[is(-1.0)] pub torch_lost_at: f64,
    /// compared by identity in doorway.ts, doorpath.ts, guards.ts: see path_serial
    pub path: Vec<V3>,
    pub path_i: usize,
    pub repath_t: f64,
    /// guards.ts asks whether the back-off laid it: see path_goal_orbit
    pub path_goal: Option<V3>,
    #[is(FirearmId(0))] pub weapon: FirearmId,
    #[is(1.0)] pub fire_cd: f64,
    #[is(-1.0)] pub reload_t: f64,
    pub carbine: bool,
    pub sup: crate::suppression::Sup,
    pub aim: crate::aim::AimState,
    pub react_t: f64,
    pub search_t: f64,
    pub look_phase: f64,
    pub speed: f64,
    #[is(LightId(0))] pub flashlight: LightId,
    pub sweep: f64,
    #[is([0.0, -0.3, 1.0])] pub beam_dir: V3,
    pub step_phase: f64,
    #[is(-1.0)] pub heard_up_to: f64,
    #[is(1.0)] pub smoke_trans: f64,
    #[is(-1.0)] pub light_dead_until: f64,
    #[is(-1.0)] pub died_at: f64,
    pub slung: Option<crate::game::GuardSlung>,
    pub second: Option<crate::game::GuardSecond>,
    pub big_out: Option<crate::game::GuardBigOut>,
    pub work_stow: Option<crate::game::GuardWorkStow>,
    #[is(true)] pub light_lit_at_loss: bool,
    pub dazzled_at: Option<V3>,
    #[is(-1.0)] pub dazzled_until: f64,
    #[is(-1.0)] pub stagger_until: f64,
    #[is(-1.0)] pub door_down_until: f64,
    #[is(-1.0)] pub hurt_at: f64,
    pub saw_player_this_frame: bool,
    pub last_pos: V3,
    pub stuck_t: f64,
    pub stuck_n: f64,
    pub saw_foe_this_frame: bool,
    #[is(-1.0)] pub killer_id: f64,
    #[is(-100.0)] pub saw_body_t: f64,
    pub leader_still_t: f64,
    #[is(1000000000.0)] pub alert_best: f64,
    pub alert_stall_t: f64,
    pub alert_ref: Option<V3>,
    pub chase_t: f64,
    pub shot_blocked_t: f64,
    pub sight_pos: V3,
    pub sight_vel: V3,
    #[is(-100.0)] pub sight_t: f64,
    #[is(-100.0)] pub led_t: f64,
    #[is(-100.0)] pub fix_t: f64,
    pub fix_ref: Option<V3>,
    pub search_plan: Option<crate::guards::SearchPlan>,
    pub pieing: Option<crate::guards::Pieing>,
    pub pie_last: Option<DoorId>,
    #[is(-100.0)] pub pie_last_t: f64,
    pub pie_prev: Option<V3>,
    #[is(-100.0)] pub pie_prev_t: f64,
    pub standoff: Option<GuardsStandoffId>,
    pub stall_ref: Option<V3>,
    pub stall_t: f64,
    pub script: Option<crate::game::GuardScript>,
    pub task: Option<crate::game::GuardTask>,
    pub drawn: bool,
    /// may name a man who is dead or has left
    pub leader: Option<GuardId>,
    pub follow_t: f64,
    /// followLeader fills it in place through a local
    pub follow_goal: V3,
    pub post: Option<crate::scene_level::Chokepoint>,
    /// INVENTED: bumped by every whole write of last_known, for the identity question at doze.ts:244
    pub fix_stamp: u32,
    /// INVENTED: bumped by every whole replacement of path
    pub path_serial: u32,
    /// INVENTED: path_goal is the very point the standoff laid (guards.ts)
    pub path_goal_orbit: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardSkill {
    pub settle_k: f64,
    pub spread_k: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardHeld {
    pub by: crate::game::GuardHeldBy,
    pub since: f64,
    pub variant: crate::character::HoldVariant,
    pub phase: crate::character::HoldPhase,
    pub reverse: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum GuardHeldBy {
    #[default] Player,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct GuardBubble {
    pub text: String,
    pub t: f64,
    pub dur: f64,
    pub radio: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardLineFail {
    pub pos: V3,
    pub sides: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardBodyDuty {
    pub body: GuardId,
    pub role: crate::game::GuardBodyDutyRole,
    pub stage: crate::game::GuardBodyDutyStage,
    pub t: f64,
    pub t0: f64,
    pub spot: Option<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum GuardBodyDutyRole {
    #[default] Finder,
    Called,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum GuardBodyDutyStage {
    #[default] Fan,
    To,
    Cover,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardSlung {
    pub own: FirearmId,
    pub own_id: Option<crate::weapons::WeaponId>,
    pub ahead: f64,
    pub gun: FirearmId,
    pub out: bool,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardSecond {
    pub gun: FirearmId,
    pub id: crate::weapons::WeaponId,
    pub ahead: f64,
    pub out: bool,
    pub t: f64,
    pub lock: f64,
    pub shots: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct GuardBigOut {
    pub t0: f64,
    pub blows: Vec<f64>,
    pub end: f64,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardWorkStow {
    pub up_t: f64,
    pub fed: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ROLLVOICE {
    pub gain: f64,
    pub full_v: f64,
    pub min_v: f64,
    pub grace: f64,
}

#[derive(Clone, Debug, Default)]
pub struct GuardNewLook {
    pub tint2: Option<V3>,
    pub yaw: Option<f64>,
    pub name: Option<String>,
    pub team: Option<crate::teams::Team>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GameNewOpts {
    pub seats: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameFreeHeldWhy {
    Shot,
    Cancel,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameSetBlackoutCause {
    Shot,
    Ocp,
    Debug,
    Hand,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameLightEventCause {
    Shot,
    Ocp,
}

/// (src/game/game.ts:341 PLAYER_MAX_HITS)
pub const PLAYER_MAX_HITS: f64 = 1.0;

/// (src/game/game.ts:42 MARK_LIGHT_MIN_I0)
pub const MARK_LIGHT_MIN_I0: f64 = 0.05;

/// (src/game/game.ts:296 BODY_DOOR_PARTICLES: a table nothing writes)
pub fn body_door_particles() -> Vec<f64> { alloc::vec![0.0, 1.0, 3.0, 12.0, 13.0, 14.0, 15.0] }

/// (src/game/game.ts:508 CALLSIGNS: a table nothing writes)
pub fn callsigns() -> Vec<String> { alloc::vec![String::from("Kowalski"), String::from("Novak"), String::from("Reyes"), String::from("Okafor"), String::from("Lindqvist"), String::from("Brandt"), String::from("Marsh")] }

/// (src/game/game.ts:516 TEMPERAMENT: a table nothing writes)
pub fn temperament() -> &'static [(&'static str, crate::game::Temperament)] { &[("Kowalski", crate::game::Temperament::Steady), ("Novak", crate::game::Temperament::Nervy), ("Reyes", crate::game::Temperament::Hard), ("Okafor", crate::game::Temperament::Steady), ("Lindqvist", crate::game::Temperament::Nervy), ("Brandt", crate::game::Temperament::Hard), ("Marsh", crate::game::Temperament::Steady)] }

/// (src/game/game.ts:778 ROLL_VOICE: a table nothing writes)
pub fn roll_voice() -> crate::game::ROLLVOICE { crate::game::ROLLVOICE { gain: 0.5, full_v: 2.5, min_v: 0.12, grace: 0.5 } }

impl Guard {
    /// (game.ts Guard's constructor) look_phase: simRandom() * 10, DRAWN BEFORE his Character is made, as the field initialiser runs first. foe goes None for a side not hostile to the player. His body's muzzle_ahead is the weapon's row: the caller sets it, as the constructor does
    pub fn new(ch: CharId, callsign: String, team: Option<crate::teams::Team>, route: RouteId, route_i: i32, flashlight: LightId, keys: JsSet<String>, weapon: FirearmId, pos: V3, look_phase: f64, talk: GameTalkId) -> Self {
        let mut g = Guard::blank();
        g.ch = ch; g.look_phase = look_phase; g.last_pos = pos; g.route_i = route_i; g.route = route; g.flashlight = flashlight; g.callsign = callsign;
        if let Some(t) = team { g.team = t; }
        if g.team != crate::teams::Team::Guard && g.team != crate::teams::Team::Hostile { g.foe = None; }
        g.keys = keys; g.weapon = weapon; g.talk = talk;
        g
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_new_man_holds_what_the_typescript_gives_him() {
        let g = Guard::blank();
        assert!(matches!(g.foe, Some(Foe::Player)));
        assert_eq!((g.last_seen_t, g.gun_lost_at, g.torch_lost_at, g.killer_id, g.fire_cd), (-100.0, -1.0, -1.0, -1.0, 1.0));
        assert_eq!(g.state, GuardState::Patrol);
        assert_eq!(g.beam_dir, [0.0, -0.3, 1.0]);
        assert!(crate::aim::AimState::blank().last_bearing.is_nan());
        assert_eq!(Player::blank().hits_left, 1.0);
        let keys = JsSet::new();
        let a = Guard::new(CharId(1), String::new(), Some(crate::teams::Team::Operative), RouteId(0), 0, LightId(0), keys, FirearmId(0), [1.0, 0.0, 2.0], 0.5, GameTalkId(0));
        assert!(a.foe.is_none());
        assert_eq!((a.last_pos, a.look_phase), ([1.0, 0.0, 2.0], 0.5));
    }
    #[test]
    fn a_word_prints_as_the_typescript_wrote_it() {
        assert_eq!(GuardState::Patrol.as_str(), "patrol");
        assert_eq!(crate::character::Bone::HeadTop.as_str(), "headTop");
    }
}
impl Player {
    /// (game.ts Player's constructor) His body's muzzle_ahead is the pistol's row: the caller sets it. pistol and rifle are made by weapons.ts's rows, which this blank cannot read
    pub fn new(ch: CharId, has_rifle: bool, pistol: FirearmId, rifle: FirearmId) -> Self {
        let mut p = Player::blank();
        p.ch = ch; p.has_rifle = has_rifle; p.pistol = pistol; p.rifle = rifle;
        p
    }
}
impl Game {
    /// INVENTED: a new serial, for a row or record the TypeScript asks by identity. Never rewinds in a game; 0 is nobody's
    pub fn serial(&mut self) -> u32 { let s = self.serial_next; self.serial_next += 1; s }
}

/// the tables of things that are not at Game's top level
#[derive(Clone, Debug, ToBytes, Blank)]
#[bytes(parts)]
pub struct Tabs {
    pub boxes: Tab<BoxId, crate::scene_boxes::Box>,
    pub frame_boxes: Tab<FrameBoxId, crate::scene_boxes::Box>,
    pub clip: Tab<ClipId, crate::anim_gltf::Clip>,
    pub anim_gltf_mesh_data: Tab<AnimGltfMeshDataId, crate::anim_gltf::MeshData>,
    pub anim_gltf_cast_variant: Tab<AnimGltfCastVariantId, crate::anim_gltf::CastVariant>,
    pub anim_gltf_cast_info: Tab<AnimGltfCastInfoId, crate::anim_gltf::CastInfo>,
    pub rig: Tab<RigId, crate::anim_gltf::GltfCharacter>,
    pub pose_set: Tab<PoseSetId, crate::anim_skeleton::PoseSet>,
    pub ragdoll: Tab<RagdollId, crate::ragdoll::Ragdoll>,
    pub props_prop: Tab<PropId, crate::props::Prop>,
    /// (a host's hand, which differs by machine: in no bytes)
    #[bytes(skip)]
    pub input: Tab<InputId, crate::input::Input>,
    /// (a host's hand, which differs by machine: in no bytes)
    #[bytes(skip)]
    pub camera: Tab<CameraId, crate::camera::FollowCamera>,
    pub item: Tab<ItemId, crate::throwables::Item>,
    pub debris_chunk: Tab<DebrisChunkId, crate::debris::Chunk>,
    pub cmd_seat: Tab<SeatId, crate::cmd::Seat>,
    pub light: Tab<LightId, crate::render_lights::RtLight>,
    pub sim_lightshome_fixture: Tab<FixtureId, crate::sim_lightshome::Fixture>,
    pub devices_camera_rt: Tab<CameraRtId, crate::devices::CameraRt>,
    pub devices_laser_rt: Tab<LaserRtId, crate::devices::LaserRt>,
    pub wallbreak_breakable: Tab<WallbreakBreakableId, crate::wallbreak::Breakable>,
    pub furnblast_member: Tab<FurnblastMemberId, crate::furnblast::Member>,
    pub wind_wind_source: Tab<WindWindSourceId, crate::wind::WindSource>,
    pub doortools_door_op: Tab<DoorOpId, crate::doortools::DoorOp>,
    pub orders_assignment: Tab<OrdersAssignmentId, crate::orders::Assignment>,
    pub doortools_room_search: Tab<DoortoolsRoomSearchId, crate::doortools::RoomSearch>,
    pub doortools_word: Tab<WordId, crate::doortools::Word>,
    pub doortools_door_gate: Tab<DoorGateId, crate::doortools::DoorGate>,
    pub doortools_known_threat: Tab<DoortoolsKnownThreatId, crate::doortools::KnownThreat>,
    pub doortools_set_charge: Tab<ChargeId, crate::doortools::SetCharge>,
    pub orders_order_def: Tab<OrdersOrderDefId, crate::orders::OrderDef>,
    pub roomfight_def: Tab<RoomfightDefId, crate::roomfight::Def>,
    pub roomfight_room: Tab<RoomfightRoomId, crate::roomfight::Room>,
    pub cover_opening: Tab<CoverOpeningId, crate::cover::Opening>,
    pub wallreact_breach: Tab<WallreactBreachId, crate::wallreact::Breach>,
    pub tactics_role: Tab<TacticsRoleId, crate::tactics::Role>,
    pub tactics_plan: Tab<TacticsPlanId, crate::tactics::Plan>,
    pub tactics_danger_rec: Tab<TacticsDangerRecId, crate::tactics::DangerRec>,
    pub lanes_lane: Tab<LanesLaneId, crate::lanes::Lane>,
    pub pressure_man: Tab<PressureManId, crate::pressure::Man>,
    pub stickycam_sticky_cam: Tab<StickyCamId, crate::stickycam::StickyCam>,
    pub rescueflow_region: Tab<RescueflowRegionId, crate::rescueflow::Region>,
    pub squad_room_clear: Tab<RoomClearId, crate::squad::RoomClear>,
    pub director_run: Tab<DirectorRunId, crate::director::Run>,
    pub roomfight_op: Tab<RoomfightOpId, crate::roomfight::Op>,
    pub wallcharge_wall_op: Tab<WallOpId, crate::wallcharge::WallOp>,
    pub wallcharge_set_wall_charge: Tab<WallChargeId, crate::wallcharge::SetWallCharge>,
    pub route: Tab<RouteId, crate::scene_level::PatrolRoute>,
    pub enemysquads_squad: Tab<EnemysquadsSquadId, crate::enemysquads::Squad>,
    pub director_live_beat: Tab<DirectorLiveBeatId, crate::director::LiveBeat>,
    pub guards_standoff: Tab<GuardsStandoffId, crate::guards::Standoff>,
    pub game_talk: Tab<GameTalkId, crate::game::Talk>,
}
impl core::ops::Index<AnyBoxId> for Tabs {
    type Output = crate::scene_boxes::Box;
    #[inline] fn index(&self, id: AnyBoxId) -> &Self::Output { match id { AnyBoxId::World(b) => &self.boxes[b], AnyBoxId::Frame(b) => &self.frame_boxes[b] } }
}
impl core::ops::IndexMut<AnyBoxId> for Tabs {
    #[inline] fn index_mut(&mut self, id: AnyBoxId) -> &mut Self::Output { match id { AnyBoxId::World(b) => &mut self.boxes[b], AnyBoxId::Frame(b) => &mut self.frame_boxes[b] } }
}

/// each module's top-level state: g.m.<module>
#[derive(Clone, Debug, ToBytes, Blank)]
#[bytes(parts)]
pub struct Mods {
    /// (the process's: in no bytes)
    #[bytes(skip)]
    pub math_side: crate::math_side::MathSideMod,
    /// (the process's: in no bytes)
    #[bytes(skip)]
    pub math_exact: crate::math_exact::MathExactMod,
    pub scene_markings: crate::scene_markings::SceneMarkingsMod,
    pub scene_materials: crate::scene_materials::SceneMaterialsMod,
    pub scene_stuff: crate::scene_stuff::SceneStuffMod,
    pub thinline: crate::thinline::ThinlineMod,
    pub scene_look: crate::scene_look::SceneLookMod,
    pub anim_gltf: crate::anim_gltf::AnimGltfMod,
    /// (where rust/sim's rows lie in this process, and counts: in no bytes)
    #[bytes(skip)]
    pub sim_sim: crate::sim_sim::SimSimMod,
    /// (where rust/sim's rows lie in this process, and counts: in no bytes)
    #[bytes(skip)]
    pub sim_propshome: crate::sim_propshome::SimPropshomeMod,
    /// (where rust/sim's rows lie in this process, and counts: in no bytes)
    #[bytes(skip)]
    pub scene_box_mirror: crate::scene_box_mirror::SceneBoxMirrorMod,
    /// (counts of what was asked: in no bytes)
    #[bytes(skip)]
    pub scene_collision: crate::scene_collision::SceneCollisionMod,
    pub anim_skeleton: crate::anim_skeleton::AnimSkeletonMod,
    /// (a memo, worked out again from the rig: in no bytes)
    #[bytes(skip)]
    pub packside: crate::packside::PacksideMod,
    pub rig_props: crate::rig_props::RigPropsMod,
    pub character: crate::character::CharacterMod,
    /// (where rust/sim's rows lie in this process, and counts: in no bytes)
    #[bytes(skip)]
    pub sim_doorshome: crate::sim_doorshome::SimDoorshomeMod,
    /// (where rust/sim's rows lie in this process, and counts: in no bytes)
    #[bytes(skip)]
    pub sim_slabshome: crate::sim_slabshome::SimSlabshomeMod,
    pub teams: crate::teams::TeamsMod,
    pub opening: crate::opening::OpeningMod,
    pub weather_kinds: crate::weather_kinds::WeatherKindsMod,
    pub kitload: crate::kitload::KitloadMod,
    pub holdfire: crate::holdfire::HoldfireMod,
    pub picture: crate::picture::PictureMod,
    pub destruction_switch: crate::destruction_switch::DestructionSwitchMod,
    pub dress: crate::dress::DressMod,
    /// (where rust/sim's rows lie in this process, and counts: in no bytes)
    #[bytes(skip)]
    pub sim_fallshome: crate::sim_fallshome::SimFallshomeMod,
    pub edge_look: crate::edge_look::EdgeLookMod,
    pub wallbreak: crate::wallbreak::WallbreakMod,
    pub furnblast: crate::furnblast::FurnblastMod,
    pub doorblast: crate::doorblast::DoorblastMod,
    pub fire: crate::fire::FireMod,
    pub winbars: crate::winbars::WinbarsMod,
    pub destruction: crate::destruction::DestructionMod,
    pub wind: crate::wind::WindMod,
    pub glasspen: crate::glasspen::GlasspenMod,
    pub doorway: crate::doorway::DoorwayMod,
    pub nightbreach: crate::nightbreach::NightbreachMod,
    pub handleside: crate::handleside::HandlesideMod,
    pub doornames: crate::doornames::DoornamesMod,
    pub runner: crate::runner::RunnerMod,
    pub netquietfeed: crate::netquietfeed::NetquietfeedMod,
    pub netquiet: crate::netquiet::NetquietMod,
    pub laststand: crate::laststand::LaststandMod,
    pub stackroom: crate::stackroom::StackroomMod,
    pub rearwatch: crate::rearwatch::RearwatchMod,
    pub coverfire: crate::coverfire::CoverfireMod,
    pub wallreact: crate::wallreact::WallreactMod,
    pub tactics: crate::tactics::TacticsMod,
    pub lanes: crate::lanes::LanesMod,
    pub digin: crate::digin::DiginMod,
    pub pressure: crate::pressure::PressureMod,
    pub difficulty: crate::difficulty::DifficultyMod,
    pub stickycam: crate::stickycam::StickycamMod,
    pub quietfog: crate::quietfog::QuietfogMod,
    pub calmtorch: crate::calmtorch::CalmtorchMod,
    pub powerout: crate::powerout::PoweroutMod,
    pub darkfoe: crate::darkfoe::DarkfoeMod,
    pub whistle: crate::whistle::WhistleMod,
    pub attention: crate::attention::AttentionMod,
    pub doze: crate::doze::DozeMod,
    pub needs: crate::needs::NeedsMod,
    pub gunwall: crate::gunwall::GunwallMod,
    pub lightswitch: crate::lightswitch::LightswitchMod,
    pub stalk: crate::stalk::StalkMod,
    pub darkdoor: crate::darkdoor::DarkdoorMod,
    pub startle: crate::startle::StartleMod,
    pub flinch: crate::flinch::FlinchMod,
    pub rescue: crate::rescue::RescueMod,
    pub seenwalk: crate::seenwalk::SeenwalkMod,
    pub wet: crate::wet::WetMod,
    pub he40: crate::he40::He40Mod,
    pub samlook: crate::samlook::SamlookMod,
    pub irlight: crate::irlight::IrlightMod,
    pub nvg: crate::nvg::NvgMod,
    pub irlaser: crate::irlaser::IrlaserMod,
    pub gunhold: crate::gunhold::GunholdMod,
    pub unseen: crate::unseen::UnseenMod,
    pub squadsmoke: crate::squadsmoke::SquadsmokeMod,
    pub squad40: crate::squad40::Squad40Mod,
    pub squadthrow: crate::squadthrow::SquadthrowMod,
    pub smokebreak: crate::smokebreak::SmokebreakMod,
    pub showwhy: crate::showwhy::ShowwhyMod,
    pub cmd: crate::cmd::CmdMod,
    pub fireline: crate::fireline::FirelineMod,
    pub doorforce: crate::doorforce::DoorforceMod,
    /// (where rust/sim's rows lie in this process, and counts: in no bytes)
    #[bytes(skip)]
    pub rustknobs: crate::rustknobs::RustknobsMod,
    pub glassbreak: crate::glassbreak::GlassbreakMod,
    pub glass: crate::glass::GlassMod,
    pub wallpen: crate::wallpen::WallpenMod,
    pub suppression: crate::suppression::SuppressionMod,
    pub enemysquads: crate::enemysquads::EnemysquadsMod,
    pub morale: crate::morale::MoraleMod,
    pub doortools: crate::doortools::DoortoolsMod,
    pub shotgun: crate::shotgun::ShotgunMod,
    pub player: crate::player::PlayerMod,
    pub close: crate::close::CloseMod,
    pub spotted: crate::spotted::SpottedMod,
    pub pairorders: crate::pairorders::PairordersMod,
    pub pairup: crate::pairup::PairupMod,
    pub sneak: crate::sneak::SneakMod,
    pub silent: crate::silent::SilentMod,
    pub cast: crate::cast::CastMod,
    pub stature: crate::stature::StatureMod,
    pub blast_feel: crate::blast_feel::BlastFeelMod,
    pub combat: crate::combat::CombatMod,
    pub scene_level_load: crate::scene_level_load::SceneLevelLoadMod,
    pub snow_bog: crate::snow_bog::SnowBogMod,
    /// (where rust/sim's rows lie in this process, and counts: in no bytes)
    #[bytes(skip)]
    pub sim_itemshome: crate::sim_itemshome::SimItemshomeMod,
    pub frag: crate::frag::FragMod,
    pub teampace: crate::teampace::TeampaceMod,
    pub squad: crate::squad::SquadMod,
    pub quiet: crate::quiet::QuietMod,
    pub workthings: crate::workthings::WorkthingsMod,
    pub lighting: crate::lighting::LightingMod,
    pub hours: crate::hours::HoursMod,
    pub chronicle: crate::chronicle::ChronicleMod,
    pub kit: crate::kit::KitMod,
    pub wandorder: crate::wandorder::WandorderMod,
    pub fogofwar: crate::fogofwar::FogofwarMod,
    pub guards: crate::guards::GuardsMod,
    pub dialogue: crate::dialogue::DialogueMod,
    pub civilians: crate::civilians::CiviliansMod,
    pub orders: crate::orders::OrdersMod,
    pub wallcharge: crate::wallcharge::WallchargeMod,
    pub blast_look: crate::blast_look::BlastLookMod,
    pub particle_fx: crate::particle_fx::ParticleFxMod,
    pub shutdoor: crate::shutdoor::ShutdoorMod,
    pub fixture_break: crate::fixture_break::FixtureBreakMod,
    pub weapons: crate::weapons::WeaponsMod,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub render_frame: crate::render_frame::RenderFrameMod,
}

/// each module's counters: g.stats.<module>
#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub wand: crate::wand::WandStats,
    pub cmdring: crate::cmdring::CmdringStats,
    pub workstow: crate::workstow::WorkstowStats,
    pub holdfire: crate::holdfire::HoldfireStats,
    pub picture: crate::picture::PictureStats,
    pub shards: crate::shards::ShardsStats,
    pub carlamps: crate::carlamps::CarlampsStats,
    pub edge_look: crate::edge_look::EdgeLookStats,
    pub wallbreak: crate::wallbreak::WallbreakStats,
    pub furnblast: crate::furnblast::FurnblastStats,
    pub doorblast: crate::doorblast::DoorblastStats,
    pub winbars: crate::winbars::WinbarsStats,
    pub destruction: crate::destruction::DestructionStats,
    pub break_table: crate::break_table::BreakTableStats,
    pub wind: crate::wind::WindStats,
    pub glasspen: crate::glasspen::GlasspenStats,
    pub gunroles: crate::gunroles::GunrolesStats,
    pub doorway: crate::doorway::DoorwayStats,
    pub handleside: crate::handleside::HandlesideStats,
    pub aimcost: crate::aimcost::AimcostStats,
    pub runner: crate::runner::RunnerStats,
    pub netquiet: crate::netquiet::NetquietStats,
    pub stackroom: crate::stackroom::StackroomStats,
    pub rearwatch: crate::rearwatch::RearwatchStats,
    pub roomfit: crate::roomfit::RoomfitStats,
    pub gunslots: crate::gunslots::GunslotsStats,
    pub coverfire: crate::coverfire::CoverfireStats,
    pub wallreact: crate::wallreact::WallreactStats,
    pub tactics: crate::tactics::TacticsStats,
    pub digin: crate::digin::DiginStats,
    pub stickycam: crate::stickycam::StickycamStats,
    pub par: crate::par::ParStats,
    pub aimweight: crate::aimweight::AimweightStats,
    pub calmtorch: crate::calmtorch::CalmtorchStats,
    pub darkfoe: crate::darkfoe::DarkfoeStats,
    pub whistle: crate::whistle::WhistleStats,
    pub attention: crate::attention::AttentionStats,
    pub doze: crate::doze::DozeStats,
    pub needs: crate::needs::NeedsStats,
    pub gunwall: crate::gunwall::GunwallStats,
    pub lightswitch: crate::lightswitch::LightswitchStats,
    pub samstun: crate::samstun::SamstunStats,
    pub stalk: crate::stalk::StalkStats,
    pub darkdoor: crate::darkdoor::DarkdoorStats,
    pub startle: crate::startle::StartleStats,
    pub flinch: crate::flinch::FlinchStats,
    pub sprintdoor: crate::sprintdoor::SprintdoorStats,
    pub seenwalk: crate::seenwalk::SeenwalkStats,
    pub wet: crate::wet::WetStats,
    pub he40: crate::he40::He40Stats,
    pub stackline: crate::stackline::StacklineStats,
    pub unseen: crate::unseen::UnseenStats,
    pub squadsmoke: crate::squadsmoke::SquadsmokeStats,
    pub squad40: crate::squad40::Squad40Stats,
    pub throwplan: crate::throwplan::ThrowplanStats,
    pub squadthrow: crate::squadthrow::SquadthrowStats,
    pub autopilot: crate::autopilot::AutopilotStats,
    pub kitanim: crate::kitanim::KitanimStats,
    pub peeksight: crate::peeksight::PeeksightStats,
    pub friends: crate::friends::FriendsStats,
    pub friendlyfire: crate::friendlyfire::FriendlyfireStats,
    pub cleanrestart: crate::cleanrestart::CleanrestartStats,
    pub fireline: crate::fireline::FirelineStats,
    pub noshove: crate::noshove::NoshoveStats,
    pub doorforce: crate::doorforce::DoorforceStats,
    pub wallpen: crate::wallpen::WallpenStats,
    pub condition: crate::condition::ConditionStats,
    pub morale: crate::morale::MoraleStats,
    pub shotgun: crate::shotgun::ShotgunStats,
    pub bigkit: crate::bigkit::BigkitStats,
    pub close: crate::close::CloseStats,
    pub takedown: crate::takedown::TakedownStats,
    pub spotted: crate::spotted::SpottedStats,
    pub pairorders: crate::pairorders::PairordersStats,
    pub pairup: crate::pairup::PairupStats,
    pub sneak: crate::sneak::SneakStats,
    pub silent: crate::silent::SilentStats,
    pub lesson: crate::lesson::LessonStats,
    pub rails: crate::rails::RailsStats,
    pub frag: crate::frag::FragStats,
    pub teampace: crate::teampace::TeampaceStats,
    pub quiet: crate::quiet::QuietStats,
    pub workthings: crate::workthings::WorkthingsStats,
    pub lighting: crate::lighting::LightingStats,
    pub handwork: crate::handwork::HandworkStats,
    pub wandorder: crate::wandorder::WandorderStats,
    pub civilians: crate::civilians::CiviliansStats,
    pub deathrecap: crate::deathrecap::DeathrecapStats,
    pub planmode: crate::planmode::PlanmodeStats,
    pub doorthrow: crate::doorthrow::DoorthrowStats,
    pub shutdoor: crate::shutdoor::ShutdoorStats,
    pub fixture_break: crate::fixture_break::FixtureBreakStats,
    pub moths: crate::moths::MothsStats,
}
impl Stats {
    pub fn blank() -> Self { Self::default() }
}

#[cfg(test)]
mod stands {
    #[test]
    fn a_blank_game_can_be_built() { let g = super::Game::blank(); let _ = g.k.startle.beat; let h = g.clone(); let _ = h.time; }
}
