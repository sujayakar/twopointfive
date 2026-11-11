//! The types of `game`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, crosses::Crosses, ids::*, js::*, Quat, V3};
use alloc::{string::String, sync::Arc, vec::Vec};

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
    /// dealt at the one place a man's own script is made (`squad::order`, when he has none). What runs a step of his script asks by it whether the script it began with is still his. A lent script (frag, runner, morale, startle) stays 0: his own goes back before anything can read it here
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words, Crosses)]
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

/// what put a man down, as the debrief tells it
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum With<'a> {
    /// what is in the hand: the gun, or the hand itself
    #[default] Hand,
    Ring,
    Whip,
    Knife,
    /// a charge on the wall he stood behind
    Charge,
    /// what went off, by its name
    Blast(&'a str),
}

impl With<'_> {
    /// how a man who still breathes was put down, in a word
    pub fn method(self, how: DownKind) -> &'static str {
        match (how, self) {
            (DownKind::Choked, _) => "choke",
            (_, With::Ring) => "ring",
            (_, With::Whip) => "whip",
            (_, With::Charge) => "charge",
            _ => "takedown",
        }
    }
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
impl WitnessHeard {
    /// the loud noises his ears have counted: shots, kicks and bangs
    pub fn loud(&self) -> f64 { self.shots + self.kick + self.bang }
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words, Crosses)]
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
    pub pulled: bool,
    pub alarm_on: bool,
    pub sweep_at: f64,
    pub spotted_t: f64,
    pub down_card_at: f64,
    pub hands_on: bool,
    pub stats: crate::game::MissionStats,
    /// dealt where g.mission is written whole: what asks "the same run?" holds this
    pub serial: u32,
}

#[derive(Clone, Debug, ToBytes, Default, Crosses)]
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
    pub cause: Option<crate::lighting::LightEventCause>,
    pub sw_at: Option<V3>,
    pub blind: Option<bool>,
    pub lure: Option<bool>,
    pub frag: Option<bool>,
    pub charge: Option<bool>,
    pub ears: Option<[V3; 2]>,
    /// dealt at birth: the heard sets know an event by it
    pub serial: u32,
}

/// What a game was stood with: its own record, set once at the stand, and the one rules read
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
    /// the way in asked for: 0 the level's own, k its k-th other, however many the level has (roomclear way_taken)
    pub way: usize,
}
impl StoodWith {
    /// what a game stood with nothing said for it holds: the level's own weather, at the level's own night
    pub fn blank() -> Self {
        StoodWith { weather: None, locked: false, hour: crate::hours::HourId::Night, sun: 1.0, sun_given: false, dry: true, way: 0 }
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
    pub dice: crate::dice::Draws,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub light_home: crate::fixtures::LightHome,
    pub falls: crate::falls::Falls,
    #[is(true)] pub ai_enabled: bool,
    pub god_mode: bool,
    pub infinite_ammo: bool,
    pub show_debug: bool,
    #[is(-1.0)] pub spectate: f64,
    pub player_invisible: bool,
    pub dyn_boxes: Vec<AnyBoxId>,
    pub rings: Vec<crate::launcher::Ring>,
    pub shells: Vec<crate::he40::Shell>,
    #[is(true)] pub fn2000: bool,
    pub doors: crate::doors::Doors,
    pub door_deals: Vec<crate::doorswing::SwingDeal>,
    pub props: crate::props::PropSystem,
    pub reach_interior: Option<Vec<u8>>,
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    /// a scripted hand is at this point of the world: his cursor is on it, and his aim is it
    pub hand_at: Option<V3>,
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
    #[is(crate::sparks::Sparks { pool: alloc::vec![crate::sparks::Spark { pos: [0.0, -10.0, 0.0], vel: [0.0, 0.0, 0.0], age: 0.0, life: 0.0, alive: false, gen_: 0.0 }; 96], params: crate::sparks::SparkParams { gravity: 16.0, drag: 1.6, hold: 0.15, brightness: 2.6, max_live: 96.0, emissive: 25.0, thickness: 0.002, shutter: 0.017, min_streak: 0.02, collide: true } })] pub sparks: crate::sparks::Sparks,
    pub tracers: crate::tracers::Tracers,
    pub weather: crate::weather::Weather,
    #[is(1.0)] pub fx_seed: f64,
    #[is(-100.0)] pub last_sting_t: f64,
    pub event_seq: f64,
    #[is(crate::game::Mission { t0: 0.0, stage: crate::game::MissionStage::Infiltrate, pulled: false, alarm_on: false, sweep_at: -1.0, spotted_t: -100.0, down_card_at: -1.0, hands_on: false, serial: 0, stats: crate::game::MissionStats { qrf_at: -1.0, ..crate::game::MissionStats::default() } })] pub mission: crate::game::Mission,
    pub staged: crate::mission::Staged,
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
    /// THE HOST'S OWN CAMERA AND KEYS, handed to the game once, at its stand, by every host alike.
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
    pub pair: crate::pairorders::PairOrders,
    pub door_path: crate::doorpath::DoorPath,
    /// (this machine's own saves: in no bytes)
    #[bytes(skip)]
    pub saves: crate::savestate::Checkpoints,
    pub rack_targets: Vec<FixtureId>,
    #[is(crate::game::GameTune { devices: true, lasers: true, camera_suspicion_secs: 0.8, camera_alarm_secs: 2.0, device_cones: true, guard_walk: 1.15, guard_investigate: 1.75, guard_chase: 2.7, player_walk: 1.55, player_sprint: 4.7, player_crouch: 1.3, player_light: 60.0, flash_intensity: 70.0, flash_inner: 7.0, flash_outer: 19.0, detect_rate: 1.6, ocp_duration: 9.0, ocp_notice: crate::game::GameTuneOcpNotice::Glance, vis_slope: 0.2, vis_floor: 0.03, floor_materials: true, muzzle_discipline: true, suppression: true, carbine_guards: crate::game::CarbineDeal::None, suppress_first: true, tracers: true, aim_model: true, teams: true, needs: true, tactics: true, flank_wait: 6.0, lone_wait: 8.0, door_danger: 25.0, door_wait: 6.0, chaos: 0.5, wind: 1.0, moths: true, glass_break: true, glass_shards: true, wall_break: true, debris_impulses: true, qrf: true, qrf_eta: 75.0, qrf_size: 4.0, perimeter: true, perimeter_eta: 45.0, perimeter_size: 2.0, qrf_silent: true, civilians: true, civilian_kill_fails: false, body_light: true })] pub tune: crate::game::GameTune,
    pub engine: crate::host_api::HostApi,
    pub level: Arc<crate::scene_level::Level>,
    #[is(RigId(0))] pub rig: RigId,
    /// the game's own smoke: what every eye, every lamp's light and the fog of war see through, with the light that comes round beside it
    pub smoke: crate::smoke::Smoke,
    /// nobody plays and the floor is looked at (the title's reel): no run is kept, counted or spoken to
    pub aside: bool,
    pub rolls: Vec<crate::game::Roll>,
    pub roll_seq: f64,
    pub restart_cut: bool,
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    /// the seat a pass is about, and its man (`seats::with_seat`)
    #[is(SeatId(0))] pub seat: SeatId,
    /// each seat's own things, by its id
    #[is(crate::seats::sets_new())] pub sets: Tab<SeatId, crate::seats::SeatSet>,
    pub seats: Vec<SeatId>,
    /// the group a pass is about: the seat's own, or the one `group::with` names
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    #[is(GroupId(0))] pub group: GroupId,
    /// every group, by its id
    #[is(crate::group::groups_new())] pub groups: Tab<GroupId, crate::group::Group>,
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    pub local_seat: Option<SeatId>,
    /// (worked out again from the rest, or only counted: in no bytes)
    #[bytes(skip)]
    pub alone_own: crate::game::AsideOwn,
    /// (a machine's own: in no bytes)
    #[bytes(skip)]
    pub cmd_buf: Option<crate::cmd::PlayerCmd>,
    pub door_owns: Vec<Option<usize>>,
    pub door_was: Vec<f64>,
    pub door_hand_recs: JsMap<CharId, crate::game::GameDoorHandRecsValue>,
    /// what g.serial() deals next
    #[is(1)] pub serial_next: u32,
    /// the rows box_frame has dealt that are not free yet (boxes_frame_reset)
    pub box_frame_rows: Vec<FrameBoxId>,
    /// rows box_frame may deal again
    pub box_frame_free: Vec<FrameBoxId>,
    /// what this step did that the host shows. The host drains it (the face clears it before a step)
    /// (a step's output, and no state: in no bytes)
    #[bytes(skip)]
    pub out: Vec<crate::out_events::OutEvent>,
    /// how many with_seat calls stand open: above zero, this is a seat's view and not the bare tick. Zero between ticks
    pub view_depth: u32,
    /// the table of these things: never shrinks
    pub ch: Tab<CharId, crate::character::Character>,
    /// HOW EACH MAN IS, by his body: one table for the player, guards and civilians
    pub cond: Tab<CharId, crate::condition::Cond>,
    /// what he carries, by what it costs him (None: nothing weighs on him)
    pub load: Tab<CharId, Option<crate::kitload::KitWorn>>,
    /// the light on him: what he gives away of himself
    pub shown: Tab<CharId, crate::guards::Shown>,
    /// how his gun is: how steady his aim, and when he may fire again
    pub arms: Tab<CharId, crate::aim::Arms>,
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
    /// (what was read of character files, which nothing writes after: in no bytes, and one for every copy of a game)
    #[bytes(skip)]
    pub art: alloc::sync::Arc<crate::anim_gltf::Art>,
    /// the knob tables, at the page's values
    #[is(crate::knobs::Knobs::page())] pub k: crate::knobs::Knobs,
    /// each module's own state
    pub m: Mods,
    /// the counters kept for the checks
    /// (worked out again from the rest, or only counted: in no bytes)
    #[bytes(skip)]
    pub stats: Stats,
    /// (made again from the rest: in no bytes)
    #[bytes(skip)]
    pub made: crate::made::Tables,
    /// who has each man's body this tick, and who was asked (control/): written by the arbiter, read by the debugger's lines; not on the wire
    #[is(crate::owned::Owned::new())] pub ctl: crate::owned::Owned,
    /// the marks on the world (render_decals set.rs): what the picture draws of what happened; no rule reads them, and they are not on the wire
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    #[is(crate::owned::Owned::new())] pub mark_set: crate::owned::Owned,
    /// where a layer of the animator runs a pose forward: nobody reads it after the layer (in no bytes)
    #[bytes(skip)]
    pub bones: crate::anim_skeleton::PoseFK,
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

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Player {
    #[is(CharId(0))] pub ch: CharId,
    pub crouch: bool,
    #[is(-10.0)] pub last_fire_t: f64,
    pub backpedal: bool,
    pub sprinting: bool,
    pub dash: bool,
    #[is([0.0, 0.0])] pub shoved: [f64; 2],
    #[is(-1.0)] pub shoved_t: f64,
    #[is([0.0, 0.0])] pub shoved_from: [f64; 2],
    /// what his hands or feet are in the middle of: one thing, or nothing
    pub deed: Option<crate::game::Deed>,
    /// the body he has hold of
    pub tow: Option<crate::game::Tow>,
    pub noise: f64,
    pub bubble: Option<crate::game::PlayerBubble>,
    pub speed_sm: f64,
    pub hit_flash: f64,
    /// his hands were on a man last frame (the partner's word goes as they first are)
    pub pair_hands_was: bool,
    pub stun_posed: bool,
    #[is(-10.0)] pub door_bash_at: f64,
    pub door_bash_lean: f64,
    #[is(-1.0)] pub door_rip_until: f64,
    pub door_kick_tail: bool,
    pub door_head: f64,
    #[is(-1.0)] pub door_head_t: f64,
    pub aimw: crate::aimweight::AimWeight,
    #[is(1.0)] pub hits_left: f64,
    #[is(FirearmId(0))] pub pistol: FirearmId,
    pub ocp: crate::weapons::Ocp,
    #[is(4.0)] pub canisters: f64,
    #[is(2.0)] pub flashbangs: f64,
    #[is(2.0)] pub frags: f64,
    #[is(crate::game::SlotKind::Pistol)] pub slot: crate::game::SlotKind,
    #[is(FirearmId(0))] pub rifle: FirearmId,
    pub has_rifle: bool,
    pub equip_t: f64,
    pub setup_hand: Option<crate::game::PlayerSetupHand>,
    pub rifle_gone: bool,
    pub kit_was: Option<Option<crate::character::KitItem>>,
    pub kit_can: Option<crate::game::ThrowKind>,
    pub launch_held: Option<crate::weapons::RifleAttachment>,
    #[is(-1.0)] pub kit_owed: f64,
    pub kit_owed_for: Option<crate::game::PlayerKitOwedFor>,
    pub sling_rest: Option<crate::game::PlayerSlingRest>,
    #[is(true)] pub holstered: bool,
    pub keys: JsSet<String>,
    pub codes: JsSet<String>,
    pub hand_wait: f64,
    #[is(1.0)] pub pace: f64,
    pub nv: bool,
    pub nv_amount: f64,
    #[is(LightId(0))] pub light: LightId,
    pub pending_throw: Option<crate::game::PlayerPendingThrow>,
    /// he is low because his hands' work put him there, not by his own key
    pub knee: Option<crate::player::Knee>,
    pub wand: bool,
    #[is(crate::kit::BigKit::None)] pub big_kit: crate::kit::BigKit,
    pub cams: Vec<StickyCamId>,
    pub cam_seq: f64,
    pub work_stow: Option<crate::game::PlayerWorkStow>,
    pub wall: Option<crate::player::WallHold>,
    pub safe: Option<V3>,
    pub step_phase: f64,
    #[is(1.0)] pub step_travel: f64,
    pub step_owed: f64,
    #[is(-1.0)] pub step_at: f64,
}

/// A thing done by hand or foot that has an end of its own. He is in the middle of one at a time: `deed::begin` puts down the one before.
#[derive(Clone, Debug, ToBytes)]
pub enum Deed {
    /// a door kicked in, or the big slot's beat at one
    Kick(crate::game::PlayerKick),
    /// a lock picked
    Pick(DoorId),
    /// a look under a door
    Wand(crate::wand::Wanding),
    /// a leaf eased open
    Ease(crate::game::Easing),
    /// a code keyed in
    Code(crate::game::Coding),
    /// a charge set on a door
    Charge(crate::game::PlayerCharging),
    /// the breaker's lever thrown
    Breaker(crate::game::PlayerBreakerMove),
    /// a leaf pushed or pulled by hand
    Door(crate::hands::DoorMove),
    /// a man struck down
    Takedown(crate::game::PlayerTakedown),
    /// a man held
    Hold(crate::player::Hold),
    /// a charge set on a wall
    Wall(crate::wallcharge::WallChargeSam),
    /// the errand's thing taken from where it is kept, and for how long he has been at it
    Take(f64),
}

macro_rules! deeds {
    ($($(#[$doc:meta])* $kind:ident $ty:ty: $get:ident $get_mut:ident $over:ident;)*) => {
        impl Player {
            $(
                $(#[$doc])*
                pub fn $get(&self) -> Option<&$ty> { match &self.deed { Some(Deed::$kind(d)) => Some(d), _ => None } }
                pub fn $get_mut(&mut self) -> Option<&mut $ty> { match &mut self.deed { Some(Deed::$kind(d)) => Some(d), _ => None } }
                /// that deed is over, if he is in the middle of it
                pub fn $over(&mut self) { if let Some(Deed::$kind(_)) = self.deed { self.deed = None; } }
            )*
        }
    };
}
deeds! {
    Kick crate::game::PlayerKick: kick kick_mut kick_over;
    Pick DoorId: picking picking_mut picking_over;
    Wand crate::wand::Wanding: wanding wanding_mut wanding_over;
    Ease crate::game::Easing: easing easing_mut easing_over;
    Code crate::game::Coding: coding coding_mut coding_over;
    Charge crate::game::PlayerCharging: charging charging_mut charging_over;
    Breaker crate::game::PlayerBreakerMove: breaker_move breaker_move_mut breaker_over;
    Door crate::hands::DoorMove: door_move door_move_mut door_move_over;
    Takedown crate::game::PlayerTakedown: takedown takedown_mut takedown_over;
    Hold crate::player::Hold: holding holding_mut holding_over;
    Wall crate::wallcharge::WallChargeSam: wall_work wall_work_mut wall_work_over;
    Take f64: taking taking_mut taking_over;
}

impl Player {
    /// Does what he is in the middle of have his feet too? A leaf worked by hand leaves them his own.
    pub fn planted(&self) -> bool { !matches!(self.deed, None | Some(Deed::Ease(_) | Deed::Door(_))) }
    /// Has he a deed under way, or a body in tow?
    pub fn hands_full(&self) -> bool { self.deed.is_some() || self.tow.is_some() }
    /// Has he his hands on a man?
    pub fn on_a_man(&self) -> bool { matches!(self.deed, Some(Deed::Takedown(_) | Deed::Hold(_))) }
    /// the body he has hold of
    pub fn dragging(&self) -> Option<GuardId> { self.tow.map(|t| t.who) }
    /// how he has hold of it, once he has it by an end or on his shoulder
    pub fn haul(&self) -> Option<crate::hands::Haul> { self.tow.and_then(|t| t.haul) }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Easing {
    pub door: DoorId,
    /// the furthest it has come, and how long it has come no further
    pub best: f64,
    pub stall_t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Coding {
    pub door: DoorId,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Tow {
    pub who: GuardId,
    pub haul: Option<crate::hands::Haul>,
    pub yaw: f64,
    /// until the next scrape of it is heard
    pub noise_t: f64,
    /// how long it has been wedged
    pub stuck_t: f64,
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
    pub run_key: String,
    pub move_latch: f64,
    pub word: String,
    pub word_t: f64,
    /// the word is why a press did nothing (`interact::refuse`), and this is what it is about. None: an empty tap's, which the view reads off what he stands beside
    pub word_kind: Option<crate::view::ThingKind>,
    pub ring: Option<crate::game::PlayerSelRing>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlayerSelRing {
    pub state: crate::cmdring::RingState,
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
    pub kind: crate::kit::BigTool,
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
pub enum Errand {
    At(crate::game::ErrandAt),
    CheckOn(crate::game::ErrandCheckOn),
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ErrandAt {
    pub kind: crate::game::ErrandThing,
    pub pos: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum ErrandThing {
    #[default] Breaker,
    Device,
    Rack,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ErrandCheckOn {
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
    /// the man he has in mind: a player's or an AI man. He may be a man who has left: a body stays readable, tables never shrink
    pub foe: Option<CharId>,
    pub faces: Option<f64>,
    #[is(RouteId(0))] pub route: RouteId,
    pub unit: Option<crate::game::UnitInfo>,
    pub civ: Option<CivId>,
    pub trained: bool,
    pub skill: Option<crate::game::GuardSkill>,
    pub route_i: i32,
    pub wp: usize,
    pub wait: f64,
    pub awareness: f64,
    /// written whole: a new fix (`mind::fix`) counts in fix_stamp, one put back or carried over (`mind::fix_keep`) does not
    pub last_known: Option<V3>,
    #[is(-100.0)] pub last_seen_t: f64,
    #[is(-100.0)] pub word_t: f64,
    pub led_at: Option<V3>,
    /// what he is held to
    #[is(crate::duty::Duty::blank())] pub duty: crate::duty::Duty,
    pub god: bool,
    pub held: Option<crate::game::GuardHeld>,
    pub bubble: Option<crate::game::GuardBubble>,
    /// lines he said in the same frame as the one on his bubble, shown after it, oldest first
    pub bubble_next: Vec<crate::game::GuardBubble>,
    pub found: bool,
    pub down_kind: Option<crate::game::DownKind>,
    pub down_by: Option<crate::game::DownBy>,
    #[is(crate::game::Witness { heard: crate::game::WitnessHeard { shots: 0.0, last_shot_t: -1.0, kick: 0.0, bang: 0.0, small: 0.0, small_where: None, loud_at: None, last_t: -1.0 }, peak_awareness: 0.0, alerted_by: None, alert_t: -1.0, saw_act: None, saw_body: None, called_to_body: None, saw_held: None, dazzled: 0.0 })] pub witness: crate::game::Witness,
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
    pub fled: bool,
    #[is(-1.0)] pub gun_lost_at: f64,
    #[is(-1.0)] pub torch_lost_at: f64,
    /// replaced whole: what reads it holds path_serial to know whether it is still the one it knew
    pub path: Vec<V3>,
    pub path_i: usize,
    pub repath_t: f64,
    /// whether the standoff's back-off laid it: see path_goal_orbit
    pub path_goal: Option<V3>,
    #[is(FirearmId(0))] pub weapon: FirearmId,
    #[is(-1.0)] pub reload_t: f64,
    pub carbine: bool,
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
    #[is(-1.0)] pub door_down_until: f64,
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
    /// what a man of the night staff is off to do
    pub errand: Option<crate::game::Errand>,
    pub drawn: bool,
    /// may name a man who is dead or has left
    pub leader: Option<GuardId>,
    pub follow_t: f64,
    /// followLeader fills it in place through a local
    pub follow_goal: V3,
    pub post: Option<crate::scene_level::Chokepoint>,
    /// counts each new fix (`mind::fix`, not `fix_keep`): a rule compares it before and after to ask whether anything gave him one
    pub fix_stamp: u32,
    /// counts every whole replacement of path (`set_path`)
    pub path_serial: u32,
    /// path_goal is the very point the standoff laid
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

pub const PLAYER_MAX_HITS: f64 = 1.0;

pub const MARK_LIGHT_MIN_I0: f64 = 0.05;

pub fn body_door_particles() -> Vec<f64> { alloc::vec![0.0, 1.0, 3.0, 12.0, 13.0, 14.0, 15.0] }

pub fn callsigns() -> Vec<String> { alloc::vec![String::from("Kowalski"), String::from("Novak"), String::from("Reyes"), String::from("Okafor"), String::from("Lindqvist"), String::from("Brandt"), String::from("Marsh")] }

pub fn temperament() -> &'static [(&'static str, crate::game::Temperament)] { &[("Kowalski", crate::game::Temperament::Steady), ("Novak", crate::game::Temperament::Nervy), ("Reyes", crate::game::Temperament::Hard), ("Okafor", crate::game::Temperament::Steady), ("Lindqvist", crate::game::Temperament::Nervy), ("Brandt", crate::game::Temperament::Hard), ("Marsh", crate::game::Temperament::Steady)] }

pub fn roll_voice() -> crate::game::ROLLVOICE { crate::game::ROLLVOICE { gain: 0.5, full_v: 2.5, min_v: 0.12, grace: 0.5 } }

impl Player {
    /// His body's muzzle_ahead is the pistol's row: the caller sets it. pistol and rifle are made from the weapon table, which this blank cannot read
    pub fn new(ch: CharId, has_rifle: bool, pistol: FirearmId, rifle: FirearmId) -> Self {
        let mut p = Player::blank();
        p.ch = ch; p.has_rifle = has_rifle; p.pistol = pistol; p.rifle = rifle;
        p
    }
}
impl Guard {
    /// a new path, even none, is a new serial: what reads a path asks by it whether this is still the one it knew. An edit in place (a point
    /// taken off or put in) is the same path, and the runner lays back the one it kept with the serial it had
    pub fn set_path(&mut self, path: Vec<V3>) { self.path = path; self.path_serial = self.path_serial.wrapping_add(1); }
    /// no path, and nowhere it was going
    pub fn drop_path(&mut self) { self.set_path(Vec::new()); self.set_path_goal(None); }
    /// where his path goes. The standoff asks whether that is its own back-off point: whoever else writes it, by this, says it is not
    pub fn set_path_goal(&mut self, goal: Option<V3>) { self.path_goal = goal; self.path_goal_orbit = false; }
}
impl Game {
    /// whose man he is
    pub fn team(&self, gd: GuardId) -> crate::teams::Team { self.ch[self.gd[gd].ch].team }
    /// where he stands
    pub fn pos_of(&self, gd: GuardId) -> V3 { self.ch[self.gd[gd].ch].pos }
    /// where a seat's own man stands
    pub fn seat_pos(&self, seat: &crate::seats::SeatRef) -> V3 { self.ch[self.pl[seat.player].ch].pos }
    /// where a player's man stands
    pub fn player_pos(&self, p: PlayerId) -> V3 { self.ch[self.pl[p].ch].pos }
    /// a player's man's body's number
    pub fn player_id(&self, p: PlayerId) -> f64 { self.ch[self.pl[p].ch].id }
    /// his body's number
    pub fn char_id(&self, gd: GuardId) -> f64 { self.ch[self.gd[gd].ch].id }
    /// the body's number of a seat's own man
    pub fn seat_id(&self, seat: &crate::seats::SeatRef) -> f64 { self.ch[self.pl[seat.player].ch].id }
    /// a gun's row
    pub fn weapon(&self, w: crate::weapons::WeaponId) -> &crate::weapons::WeaponDef { self.m.weapons.weapons.get(w).unwrap() }
    /// a box of the level has moved, come or gone: what was worked out from the boxes is worked out again
    pub fn boxes_changed(&mut self) { self.engine.world.geom_serial += 1.0; }
    /// a box by its number in the level's list: none for a number below nought, a part of one or no number
    pub fn box_at(&self, i: f64) -> Option<BoxId> {
        let ix = i as usize;
        if ix as f64 != i { return None; }
        self.col.boxes.get(ix).copied()
    }
    /// a new serial, for a row or record a reader has to know from others just like it. Never rewinds in a game; 0 is nobody's
    pub fn serial(&mut self) -> u32 { let s = self.serial_next; self.serial_next += 1; s }
}

/// the tables of things that are not at Game's top level
#[derive(Clone, Debug, ToBytes, Blank)]
#[bytes(parts)]
pub struct Tabs {
    pub boxes: crate::boxworld::Boxes,
    pub frame_boxes: Tab<FrameBoxId, crate::scene_boxes::Box>,
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
    pub fixture: Tab<FixtureId, crate::fixtures::Fixture>,
    pub devices_camera_rt: Tab<CameraRtId, crate::devices::CameraRt>,
    pub devices_laser_rt: Tab<LaserRtId, crate::devices::LaserRt>,
    pub wallbreak_breakable: Tab<WallbreakBreakableId, crate::wallbreak::Breakable>,
    pub furnblast_member: Tab<FurnblastMemberId, crate::furnblast::Member>,
    pub wind_wind_source: Tab<WindWindSourceId, crate::wind::WindSource>,
    pub doortools_door_op: Tab<DoorOpId, crate::doortools::DoorOp>,
    pub task: Tab<TaskId, crate::duty::Task>,
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
impl Tabs {
    /// a box to write, of the world or of the picture
    #[inline]
    pub fn edit(&mut self, b: impl Into<AnyBoxId>) -> crate::boxworld::Edit<'_> {
        match b.into() { AnyBoxId::World(b) => self.boxes.edit(b), AnyBoxId::Frame(b) => crate::boxworld::Edit::frame(&mut self.frame_boxes[b]) }
    }
}

/// each module's top-level state: g.m.<module>
#[derive(Clone, Debug, ToBytes, Blank)]
#[bytes(parts)]
pub struct Mods {
    /// (the process's: in no bytes)
    #[bytes(skip)]
    pub math_exact: crate::math_exact::MathExactMod,
    pub scene_markings: crate::scene_markings::SceneMarkingsMod,
    pub scene_materials: crate::scene_materials::SceneMaterialsMod,
    pub scene_stuff: crate::scene_stuff::SceneStuffMod,
    pub thinline: crate::thinline::ThinlineMod,
    pub scene_look: crate::scene_look::SceneLookMod,
    pub anim_gltf: crate::anim_gltf::AnimGltfMod,
    pub character: crate::character::CharacterMod,
    pub opening: crate::opening::OpeningMod,
    pub weather_kinds: crate::weather_kinds::WeatherKindsMod,
    pub holdfire: crate::holdfire::HoldfireMod,
    pub picture: crate::picture::PictureMod,
    pub destruction_switch: crate::destruction_switch::DestructionSwitchMod,
    pub dress: crate::dress::DressMod,
    pub edge_look: crate::edge_look::EdgeLookMod,
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
    pub fireline: crate::fireline::FirelineMod,
    pub doorforce: crate::doorforce::DoorforceMod,
    pub glassbreak: crate::glassbreak::GlassbreakMod,
    pub glass: crate::glass::GlassMod,
    pub suppression: crate::suppression::SuppressionMod,
    pub enemysquads: crate::enemysquads::EnemysquadsMod,
    pub morale: crate::morale::MoraleMod,
    pub doortools: crate::doortools::DoortoolsMod,
    pub player: crate::player::PlayerMod,
    pub close: crate::close::CloseMod,
    pub spotted: crate::spotted::SpottedMod,
    pub pairorders: crate::pairorders::PairordersMod,
    pub sneak: crate::sneak::SneakMod,
    pub silent: crate::silent::SilentMod,
    pub cast: crate::cast::CastMod,
    pub stature: crate::stature::StatureMod,
    pub blast_feel: crate::blast_feel::BlastFeelMod,
    pub combat: crate::combat::CombatMod,
    pub scene_level_load: crate::scene_level_load::SceneLevelLoadMod,
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
    pub workstow: crate::workstow::WorkstowStats,
    pub picture: crate::picture::PictureStats,
    pub carlamps: crate::carlamps::CarlampsStats,
    pub doorblast: crate::doorblast::DoorblastStats,
    pub destruction: crate::destruction::DestructionStats,
    pub glasspen: crate::glasspen::GlasspenStats,
    pub doorway: crate::doorway::DoorwayStats,
    pub netquiet: crate::netquiet::NetquietStats,
    pub stackroom: crate::stackroom::StackroomStats,
    pub gunslots: crate::gunslots::GunslotsStats,
    pub coverfire: crate::coverfire::CoverfireStats,
    pub tactics: crate::tactics::TacticsStats,
    pub par: crate::par::ParStats,
    pub whistle: crate::whistle::WhistleStats,
    pub attention: crate::attention::AttentionStats,
    pub samstun: crate::samstun::SamstunStats,
    pub darkdoor: crate::darkdoor::DarkdoorStats,
    pub flinch: crate::flinch::FlinchStats,
    pub sprintdoor: crate::sprintdoor::SprintdoorStats,
    pub he40: crate::he40::He40Stats,
    pub unseen: crate::unseen::UnseenStats,
    pub kitanim: crate::kitanim::KitanimStats,
    pub friends: crate::friends::FriendsStats,
    pub friendlyfire: crate::friendlyfire::FriendlyfireStats,
    pub cleanrestart: crate::cleanrestart::CleanrestartStats,
    pub fireline: crate::fireline::FirelineStats,
    pub wallpen: crate::wallpen::WallpenStats,
    pub condition: crate::condition::ConditionStats,
    pub morale: crate::morale::MoraleStats,
    pub shotgun: crate::shotgun::ShotgunStats,
    pub bigkit: crate::bigkit::BigkitStats,
    pub takedown: crate::takedown::TakedownStats,
    pub spotted: crate::spotted::SpottedStats,
    pub pairorders: crate::pairorders::PairordersStats,
    pub pairup: crate::pairup::PairupStats,
    pub sneak: crate::sneak::SneakStats,
    pub silent: crate::silent::SilentStats,
    pub rails: crate::rails::RailsStats,
    pub frag: crate::frag::FragStats,
    pub teampace: crate::teampace::TeampaceStats,
    pub wandorder: crate::wandorder::WandorderStats,
    pub civilians: crate::civilians::CiviliansStats,
    pub deathrecap: crate::deathrecap::DeathrecapStats,
    pub doorthrow: crate::doorthrow::DoorthrowStats,
}

#[cfg(test)]
mod stands {
    #[test]
    fn a_blank_game_can_be_built() { let g = super::Game::blank(); let _ = g.k.startle.beat; let h = g.clone(); let _ = h.time; }
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
