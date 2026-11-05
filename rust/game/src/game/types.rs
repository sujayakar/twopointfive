//! The types of `game`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// WHAT HIS DUTY HAS HIS GUN ON. In the TypeScript `covers` holds a vector by reference: the room fight's own D.pt, which it moves in place, or a fresh point. A copy of D.pt would go stale inside the tick, so the first case names the source instead
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Covers {
    /// the point of his own Def in the room fight (roomfight.ts, enemysquads.ts, powerout.ts: cover(gd, D.pt)): read it there
    HisPost,
    /// a point given once (nightbreach.ts:114, rescue.ts:915)
    At(V3),
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ThrowKind {
    Smoke,
    Flash,
    Frag,
}
impl ThrowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ThrowKind::Smoke => "smoke",
            ThrowKind::Flash => "flash",
            ThrowKind::Frag => "frag",
        }
    }
}

/// (TypeScript: GuardScript, src/game/game.ts:489)
#[derive(Clone, Copy, Debug)]
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
impl GuardScript {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardScript {
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
            serial: 0,
        }
    }
}
impl Default for GuardScript { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:506 GuardScript.over, src/game/game.ts:506)
#[derive(Clone, Copy, Debug)]
pub struct GuardScriptOver {
    pub aim_at: V3,
    pub face: Option<f64>,
    pub walk: Option<bool>,
}
impl GuardScriptOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardScriptOver {
            aim_at: [0.0; 3],
            face: None,
            walk: None,
        }
    }
}
impl Default for GuardScriptOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum Foe {
    Guard(GuardId),
    Player,
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GuardState {
    Patrol,
    Suspicious,
    Alert,
    Search,
    Dead,
}
impl GuardState {
    pub const fn as_str(self) -> &'static str {
        match self {
            GuardState::Patrol => "patrol",
            GuardState::Suspicious => "suspicious",
            GuardState::Alert => "alert",
            GuardState::Search => "search",
            GuardState::Dead => "dead",
        }
    }
}

/// (TypeScript: Interactable, src/game/game.ts:256)
#[derive(Clone, Debug)]
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
impl Interactable {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Interactable {
            kind: crate::game::InteractableKind::Cam,
            pos: [0.0; 3],
            line1: String::new(),
            line2: String::new(),
            in_reach: false,
            off: false,
            progress: None,
            target: None,
            door: None,
            guard: None,
            verb: None,
            running: None,
            close: None,
            past: None,
            framed: None,
        }
    }
}
impl Default for Interactable { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum InteractableKind {
    Cam,
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
impl InteractableKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            InteractableKind::Cam => "cam",
            InteractableKind::Guard => "guard",
            InteractableKind::Breaker => "breaker",
            InteractableKind::Door => "door",
            InteractableKind::Light => "light",
            InteractableKind::Body => "body",
            InteractableKind::Objective => "objective",
            InteractableKind::Held => "held",
            InteractableKind::Wall => "wall",
            InteractableKind::Switch => "switch",
            InteractableKind::Charge => "charge",
            InteractableKind::Cancel => "cancel",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
    Defuse,
    TakeCam,
    LightsOff,
    LightsOn,
    Lead,
    Stay,
}
impl Verb {
    pub const fn as_str(self) -> &'static str {
        match self {
            Verb::Push => "push",
            Verb::Hammer => "hammer",
            Verb::Rattle => "rattle",
            Verb::Pick => "pick",
            Verb::Unlock => "unlock",
            Verb::Kick => "kick",
            Verb::Code => "code",
            Verb::Drive => "drive",
            Verb::Charge => "charge",
            Verb::Cancel => "cancel",
            Verb::Pull => "pull",
            Verb::Ease => "ease",
            Verb::Nocode => "nocode",
            Verb::Wand => "wand",
            Verb::Breach => "breach",
            Verb::Grab => "grab",
            Verb::Question => "question",
            Verb::Choke => "choke",
            Verb::Whip => "whip",
            Verb::Comply => "comply",
            Verb::Drag => "drag",
            Verb::Drop => "drop",
            Verb::BreakerOff => "breakerOff",
            Verb::BreakerOn => "breakerOn",
            Verb::Blow => "blow",
            Verb::Defuse => "defuse",
            Verb::TakeCam => "takeCam",
            Verb::LightsOff => "lightsOff",
            Verb::LightsOn => "lightsOn",
            Verb::Lead => "lead",
            Verb::Stay => "stay",
        }
    }
}

/// (TypeScript: Game, src/game/game.ts:776)
#[derive(Clone, Debug)]
pub struct Game {
    pub col: crate::scene_collision::StaticCollision,
    pub snow_field: Option<crate::render_snow_field::SnowField>,
    pub snow_tracks: Option<crate::snow_tracks::SnowTracks>,
    pub snow_look_was: f64,
    pub chars: crate::anim_characters::CharacterRenderer,
    pub rig_fits: Option<JsMap<crate::Num, crate::game::GameRigFitsValue>>,
    pub rig_fit_of: JsMap<crate::Num, Option<crate::game::GameRigFitOfValue>>,
    pub mesh_shadows: crate::render_mesh_shadows::MeshShadows,
    pub player: PlayerId,
    pub guards: Vec<GuardId>,
    pub civilians: Vec<GuardId>,
    pub targets: Vec<FixtureId>,
    pub events: Vec<crate::game::GameEvent>,
    pub probe: crate::sensors::ProbeLike,
    pub sensors: crate::sensors::Sensors,
    pub cpu_smoke: crate::sense::CpuSmoke,
    pub smoke_owned: bool,
    pub journal: crate::journal::SessionRecorder,
    pub search_probe_n: f64,
    pub time: f64,
    pub dice: Option<crate::math_dice::SimDice>,
    pub men: crate::Opaque,
    pub light_home: crate::sim_lightshome::LightHome,
    pub pose_sets: Vec<crate::Opaque>,
    pub falls: crate::sim_fallshome::FallStore,
    pub ai_enabled: bool,
    pub god_mode: bool,
    pub infinite_ammo: bool,
    pub show_debug: bool,
    pub spectate: f64,
    pub player_invisible: bool,
    pub threat_proximity: f64,
    pub torch_on_t: f64,
    pub aim_point: V3,
    pub aim_target: Option<FixtureId>,
    pub aim_guard: Option<GuardId>,
    pub aim_friend: Option<GuardId>,
    pub dyn_boxes: Vec<BoxId>,
    pub dyn_raw: f64,
    pub rings: Vec<crate::launcher::Ring>,
    pub shells: Vec<crate::he40::Shell>,
    pub fn2000: bool,
    pub doors: crate::doors::Doors,
    pub door_deals: Vec<crate::doorswing::SwingDeal>,
    pub props: crate::props::PropSystem,
    pub use_door: Option<DoorId>,
    pub reach_stats: crate::game::GameReachStats,
    pub reach_interior: Option<Vec<u8>>,
    pub reach_solid: Option<Vec<u8>>,
    pub reach_lights: Vec<LightId>,
    pub reach_queue: Vec<crate::game::GameReachQueueItem>,
    pub reach_solid_boot: Option<Vec<u8>>,
    pub reach_boot: JsMap<LightId, Vec<u32>>,
    pub reach_held: Vec<[f64; 4]>,
    pub reach_hold: f64,
    pub touring: bool,
    pub hover: Option<crate::game::Interactable>,
    pub interactables: Vec<crate::game::Interactable>,
    pub nearby: Vec<crate::game::Interactable>,
    pub aim_snap: Option<V3>,
    pub act: Option<crate::game::Interactable>,
    pub puppet: Option<crate::game::GamePuppet>,
    pub m_per_px: f64,
    pub tmp_v: V3,
    pub called_in: bool,
    pub stood_down: bool,
    pub escalation: f64,
    pub escalation_t: f64,
    pub alarm: crate::game::GameAlarm,
    pub clearing: Option<crate::squad::Clearing>,
    pub clear_rooms: f64,
    pub cleared_at: JsMap<String, f64>,
    pub beacons: Vec<crate::game::GameBeaconsItem>,
    pub items: crate::throwables::Throwables,
    pub fx: Vec<crate::game::GameFxItem>,
    pub sparks: crate::sparks::Sparks,
    pub tracers: crate::tracers::Tracers,
    pub decals: crate::render_decals::Decals,
    pub weather: crate::weather::Weather,
    pub fx_seed: f64,
    pub audio: crate::audio_iface::GameAudioLike,
    pub ocp_cooldown: f64,
    pub last_sting_t: f64,
    pub event_seq: f64,
    pub messages: Vec<crate::game::GameMessagesItem>,
    pub mission: crate::game::Mission,
    pub chronicle: crate::chronicle::Chronicle,
    pub qrf: crate::qrf::Qrf,
    pub devices: crate::devices::Devices,
    pub destr: crate::destruction::Destruction,
    pub wind: crate::wind::Wind,
    pub air_dust: crate::air_dust::AirDust,
    pub moths: crate::moths::Moths,
    pub wet_ground: crate::wet_ground::WetGround,
    pub glass_break: crate::glassbreak::GlassBreak,
    pub shards: crate::shards::GlassShards,
    pub director: crate::director::Director,
    pub night_breach: crate::nightbreach::NightBreach,
    pub quiet: crate::quiet::QuietMission,
    pub lesson: crate::lesson::Lesson,
    pub rails: crate::rails::Rails,
    pub par: crate::par::Par,
    pub verdict: crate::verdict::Verdict,
    pub snatch: crate::snatch::Snatch,
    pub defuse: crate::defuse::Defuse,
    pub room_clear: crate::roomclear::RoomClearScenario,
    pub orders: crate::orders::SquadOrders,
    pub fog_of_war: crate::fogofwar::FogOfWar,
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
    pub saves: crate::savestate::Checkpoints,
    pub rack_targets: Vec<FixtureId>,
    pub mark_lights: Vec<crate::game::GameMarkLightsItem>,
    pub tune: crate::game::GameTune,
    pub engine: crate::host_api::HostApi,
    pub level: crate::scene_level::Level,
    pub rig: RigId,
    pub smoke: crate::smoke_types::SmokeSystemLike,
    pub quiet_utility: bool,
    pub mission_in_tour: bool,
    pub view_proj: Option<crate::game::GameViewProj>,
    pub rolls: Vec<crate::game::Roll>,
    pub roll_seq: f64,
    pub restart_cut: bool,
    pub step_buf: Vec<crate::cmd::PlayerCmd>,
    pub seat: SeatId,
    pub seats: Vec<SeatId>,
    pub local_seat: Option<SeatId>,
    pub ticking: bool,
    pub alone_own: crate::game::AsideOwn,
    pub cmd_buf: Option<crate::cmd::PlayerCmd>,
    pub men_all: Vec<CharId>,
    pub men_statured: bool,
    pub men_bare: bool,
    pub fog_held: Option<crate::game::GameFogHeld>,
    pub door_owns: Vec<Option<usize>>,
    pub door_was: Vec<f64>,
    pub door_hand_recs: JsMap<CharId, crate::game::GameDoorHandRecsValue>,
    /// INVENTED: what g.serial() deals
    pub serial_next: u32,
    /// INVENTED: a second cast of the squad is on Game (squadcast::with_cast): seats.ts:240 CAST_VIEW
    pub in_cast: bool,
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
    pub k: crate::knobs::Knobs,
    /// each module's own state
    pub m: Mods,
    /// the counters kept for the checks
    pub stats: Stats,
    /// which game of rust/sim's this is: every wrapper over rust/sim reads it, and the game's die lives there
    pub sim: usize,
}
impl Game {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Game {
            col: crate::scene_collision::StaticCollision::blank(),
            snow_field: None,
            snow_tracks: None,
            snow_look_was: -1.0,
            chars: crate::anim_characters::CharacterRenderer::blank(),
            rig_fits: None,
            rig_fit_of: JsMap::new(),
            mesh_shadows: crate::render_mesh_shadows::MeshShadows::blank(),
            player: PlayerId(0),
            guards: Vec::new(),
            civilians: Vec::new(),
            targets: Vec::new(),
            events: Vec::new(),
            probe: crate::sensors::ProbeLike::zeroed(),
            sensors: crate::sensors::Sensors::blank(),
            cpu_smoke: crate::sense::CpuSmoke::blank(),
            smoke_owned: false,
            journal: crate::journal::SessionRecorder::blank(),
            search_probe_n: 0.0,
            time: 0.0,
            dice: None,
            men: crate::Opaque,
            light_home: crate::sim_lightshome::LightHome::blank(),
            pose_sets: Vec::new(),
            falls: crate::sim_fallshome::FallStore::blank(),
            ai_enabled: true,
            god_mode: false,
            infinite_ammo: false,
            show_debug: false,
            spectate: -1.0,
            player_invisible: false,
            threat_proximity: 0.0,
            torch_on_t: -100.0,
            aim_point: [0.0, 0.0, 0.0],
            aim_target: None,
            aim_guard: None,
            aim_friend: None,
            dyn_boxes: Vec::new(),
            dyn_raw: 0.0,
            rings: Vec::new(),
            shells: Vec::new(),
            fn2000: true,
            doors: crate::doors::Doors::blank(),
            door_deals: Vec::new(),
            props: crate::props::PropSystem::blank(),
            use_door: None,
            reach_stats: crate::game::GameReachStats { lights: 0.0, mean_fraction: 1.0, ms: 0.0 },
            reach_interior: None,
            reach_solid: None,
            reach_lights: Vec::new(),
            reach_queue: Vec::new(),
            reach_solid_boot: None,
            reach_boot: JsMap::new(),
            reach_held: Vec::new(),
            reach_hold: 0.0,
            touring: false,
            hover: None,
            interactables: Vec::new(),
            nearby: Vec::new(),
            aim_snap: None,
            act: None,
            puppet: None,
            m_per_px: 0.01,
            tmp_v: [0.0, 0.0, 0.0],
            called_in: false,
            stood_down: false,
            escalation: 0.0,
            escalation_t: -1.0,
            alarm: crate::game::GameAlarm { episode: false, pos: [0.0, 0.0, 0.0], placed: false, apart_t: 0.0, raised_at: -100.0 },
            clearing: None,
            clear_rooms: 0.0,
            cleared_at: JsMap::new(),
            beacons: Vec::new(),
            items: crate::throwables::Throwables::blank(),
            fx: Vec::new(),
            sparks: crate::sparks::Sparks { pool: alloc::vec![crate::sparks::Spark { pos: [0.0, -10.0, 0.0], vel: [0.0, 0.0, 0.0], age: 0.0, life: 0.0, alive: false, gen_: 0.0 }; 96], live: 0.0, params: crate::sparks::SparkParams { gravity: 16.0, drag: 1.6, hold: 0.15, brightness: 2.6, max_live: 96.0, emissive: 25.0, thickness: 0.002, shutter: 0.017, min_streak: 0.02, collide: true } },
            tracers: crate::tracers::Tracers::blank(),
            decals: crate::render_decals::Decals::blank(),
            weather: crate::weather::Weather::blank(),
            fx_seed: 1.0,
            audio: crate::audio_iface::GameAudioLike::zeroed(),
            ocp_cooldown: 0.0,
            last_sting_t: -100.0,
            event_seq: 0.0,
            messages: Vec::new(),
            mission: crate::game::Mission { t0: 0.0, stage: crate::game::MissionStage::Infiltrate, drive_t: -1.0, pulled: false, alarm_on: false, sweep_at: -1.0, spotted_t: -100.0, debrief: false, down_card_at: -1.0, hands_on: false, serial: 0, stats: crate::game::MissionStats { qrf_at: -1.0, ..crate::game::MissionStats::zeroed() } },
            chronicle: crate::chronicle::Chronicle::blank(),
            qrf: crate::qrf::Qrf::blank(),
            devices: crate::devices::Devices::blank(),
            destr: crate::destruction::Destruction::blank(),
            wind: crate::wind::Wind::blank(),
            air_dust: crate::air_dust::AirDust::blank(),
            moths: crate::moths::Moths::blank(),
            wet_ground: crate::wet_ground::WetGround::blank(),
            glass_break: crate::glassbreak::GlassBreak::blank(),
            shards: crate::shards::GlassShards::blank(),
            director: crate::director::Director::blank(),
            night_breach: crate::nightbreach::NightBreach::blank(),
            quiet: crate::quiet::QuietMission::blank(),
            lesson: crate::lesson::Lesson::blank(),
            rails: crate::rails::Rails::blank(),
            par: crate::par::Par::blank(),
            verdict: crate::verdict::Verdict::blank(),
            snatch: crate::snatch::Snatch::blank(),
            defuse: crate::defuse::Defuse::blank(),
            room_clear: crate::roomclear::RoomClearScenario::blank(),
            orders: crate::orders::SquadOrders::blank(),
            fog_of_war: crate::fogofwar::FogOfWar::blank(),
            fight: crate::roomfight::RoomFight::blank(),
            cues: crate::cues::Cues::blank(),
            squad_net: crate::squadnet::SquadNet::blank(),
            cover: crate::cover::Cover::blank(),
            entry: crate::entrypoints::EntryPoints::blank(),
            cleared: crate::cleared::ClearedRooms::blank(),
            rescue: crate::rescue::Rescue::blank(),
            door_tools: crate::doortools::DoorTools::blank(),
            pair: crate::pairorders::PairOrders::blank(),
            work: crate::workthings::WorkThings::blank(),
            wall_charge: crate::wallcharge::WallCharge::blank(),
            door_path: crate::doorpath::DoorPath::blank(),
            saves: crate::savestate::Checkpoints::blank(),
            rack_targets: Vec::new(),
            mark_lights: Vec::new(),
            tune: crate::game::GameTune { devices: true, lasers: true, camera_suspicion_secs: 0.8, camera_alarm_secs: 2.0, device_cones: true, guard_walk: 1.15, guard_investigate: 1.75, guard_chase: 2.7, player_walk: 1.55, player_sprint: 4.7, player_crouch: 1.3, player_light: 60.0, flash_intensity: 70.0, flash_inner: 7.0, flash_outer: 19.0, detect_rate: 1.6, ocp_duration: 9.0, ocp_notice: crate::game::GameTuneOcpNotice::Glance, vis_slope: 0.2, vis_floor: 0.03, floor_materials: true, muzzle_discipline: true, suppression: true, carbine_guards: crate::game::CarbineDeal::None, suppress_first: true, tracers: true, aim_model: true, teams: true, needs: true, tactics: true, flank_wait: 6.0, lone_wait: 8.0, door_danger: 25.0, door_wait: 6.0, chaos: 0.5, wind: 1.0, moths: true, glass_break: true, glass_shards: true, wall_break: true, debris_impulses: true, qrf: true, qrf_eta: 75.0, qrf_size: 4.0, perimeter: true, perimeter_eta: 45.0, perimeter_size: 2.0, qrf_silent: true, qrf_squad: true, civilians: true, civilian_kill_fails: false, body_light: true },
            engine: crate::host_api::HostApi::zeroed(),
            level: crate::scene_level::Level::zeroed(),
            rig: RigId(0),
            smoke: crate::smoke_types::SmokeSystemLike::zeroed(),
            quiet_utility: false,
            mission_in_tour: false,
            view_proj: None,
            rolls: Vec::new(),
            roll_seq: 0.0,
            restart_cut: false,
            step_buf: Vec::new(),
            seat: SeatId(0),
            seats: Vec::new(),
            local_seat: None,
            ticking: false,
            alone_own: crate::game::AsideOwn::blank(),
            cmd_buf: None,
            men_all: Vec::new(),
            men_statured: false,
            men_bare: false,
            fog_held: None,
            door_owns: Vec::new(),
            door_was: Vec::new(),
            door_hand_recs: JsMap::new(),
            serial_next: 1,
            in_cast: false,
            ch: Tab::new(),
            dr: Tab::new(),
            gd: Tab::new(),
            pl: Tab::new(),
            cv: Tab::new(),
            fa: Tab::new(),
            tab: Tabs::blank(),
            k: crate::knobs::Knobs::page(),
            m: Mods::blank(),
            stats: Stats::blank(),
            sim: 0,
        }
    }
}
impl Default for Game { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/game.ts:814 Game.rigFits, src/game/game.ts:814)
#[derive(Clone, Debug)]
pub struct GameRigFitsValue {
    pub r0: Vec<f32>,
    pub r1: Vec<f32>,
}
impl GameRigFitsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameRigFitsValue {
            r0: Vec::new(),
            r1: Vec::new(),
        }
    }
}
impl Default for GameRigFitsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:815 Game.rigFitOf, src/game/game.ts:815)
#[derive(Clone, Debug)]
pub struct GameRigFitOfValue {
    pub r0: Vec<f32>,
    pub r1: Vec<f32>,
}
impl GameRigFitOfValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameRigFitOfValue {
            r0: Vec::new(),
            r1: Vec::new(),
        }
    }
}
impl Default for GameRigFitOfValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GameEvent, src/game/game.ts:242)
#[derive(Clone, Copy, Debug)]
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
impl GameEvent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameEvent {
            announced: None,
            kind: crate::director::EventKind::Shot,
            pos: [0.0; 3],
            time: 0.0,
            loud: false,
            level: None,
            id: None,
            who: None,
            weapon: None,
            from: None,
            to: None,
            cause: None,
            sw_at: None,
            blind: None,
            lure: None,
            frag: None,
            charge: None,
            ears: None,
            serial: 0,
        }
    }
}
impl Default for GameEvent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameEventCause {
    Shot,
    Ocp,
}
impl GameEventCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GameEventCause::Shot => "shot",
            GameEventCause::Ocp => "ocp",
        }
    }
}

/// (TypeScript: inline at quiet Game.smokeOwned, src/game/game.ts:830)
#[derive(Clone, Copy, Debug)]
pub struct GameSmokeOwned {
}
impl GameSmokeOwned {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameSmokeOwned {
        }
    }
}
impl Default for GameSmokeOwned { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MissionStage {
    Drive,
    Infiltrate,
    Exfil,
    Done,
    Failed,
}
impl MissionStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            MissionStage::Drive => "drive",
            MissionStage::Infiltrate => "infiltrate",
            MissionStage::Exfil => "exfil",
            MissionStage::Done => "done",
            MissionStage::Failed => "failed",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SlotKind {
    Pistol,
    Rifle,
    Smoke,
    Flash,
    Frag,
}
impl SlotKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SlotKind::Pistol => "pistol",
            SlotKind::Rifle => "rifle",
            SlotKind::Smoke => "smoke",
            SlotKind::Flash => "flash",
            SlotKind::Frag => "frag",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DownKind {
    Shot,
    Struck,
    Choked,
    Wounded,
}
impl DownKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DownKind::Shot => "shot",
            DownKind::Struck => "struck",
            DownKind::Choked => "choked",
            DownKind::Wounded => "wounded",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DownBy {
    Guard,
    Player,
    World,
}
impl DownBy {
    pub const fn as_str(self) -> &'static str {
        match self {
            DownBy::Guard => "guard",
            DownBy::Player => "player",
            DownBy::World => "world",
        }
    }
}

/// (TypeScript: Witness, src/game/game.ts:544)
#[derive(Clone, Debug)]
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
impl Witness {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Witness {
            heard: crate::game::WitnessHeard::zeroed(),
            peak_awareness: 0.0,
            alerted_by: None,
            alert_t: 0.0,
            saw_act: None,
            saw_body: None,
            called_to_body: None,
            saw_held: None,
            was_held: 0.0,
            dazzled: 0.0,
        }
    }
}
impl Default for Witness { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:547 Witness.heard, src/game/game.ts:547)
#[derive(Clone, Copy, Debug)]
pub struct WitnessHeard {
    pub shots: f64,
    pub last_shot_t: f64,
    pub kick: f64,
    pub bang: f64,
    pub small: f64,
    pub small_where: Option<V3>,
    pub last_t: f64,
}
impl WitnessHeard {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WitnessHeard {
            shots: 0.0,
            last_shot_t: 0.0,
            kick: 0.0,
            bang: 0.0,
            small: 0.0,
            small_where: None,
            last_t: 0.0,
        }
    }
}
impl Default for WitnessHeard { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WitnessAlertedBy {
    Body,
    Held,
    Radio,
    Sound,
    Sight,
}
impl WitnessAlertedBy {
    pub const fn as_str(self) -> &'static str {
        match self {
            WitnessAlertedBy::Body => "body",
            WitnessAlertedBy::Held => "held",
            WitnessAlertedBy::Radio => "radio",
            WitnessAlertedBy::Sound => "sound",
            WitnessAlertedBy::Sight => "sight",
        }
    }
}

/// (TypeScript: inline at src/game/game.ts:554 Witness.sawAct, src/game/game.ts:554)
#[derive(Clone, Debug)]
pub struct WitnessSawAct {
    pub kind: crate::game::WitnessSawActKind,
    pub victim: String,
    pub t: f64,
}
impl WitnessSawAct {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WitnessSawAct {
            kind: crate::game::WitnessSawActKind::Shot,
            victim: String::new(),
            t: 0.0,
        }
    }
}
impl Default for WitnessSawAct { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WitnessSawActKind {
    Shot,
    Struck,
    Choked,
    Wounded,
    Grab,
}
impl WitnessSawActKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WitnessSawActKind::Shot => "shot",
            WitnessSawActKind::Struck => "struck",
            WitnessSawActKind::Choked => "choked",
            WitnessSawActKind::Wounded => "wounded",
            WitnessSawActKind::Grab => "grab",
        }
    }
}

/// (TypeScript: inline at src/game/game.ts:556 Witness.sawBody, src/game/game.ts:556)
#[derive(Clone, Debug)]
pub struct WitnessSawBody {
    pub victim: String,
    pub t: f64,
    pub breathing: bool,
}
impl WitnessSawBody {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WitnessSawBody {
            victim: String::new(),
            t: 0.0,
            breathing: false,
        }
    }
}
impl Default for WitnessSawBody { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:557 Witness.calledToBody, src/game/game.ts:557)
#[derive(Clone, Debug)]
pub struct WitnessCalledToBody {
    pub victim: String,
    pub t: f64,
    pub breathing: bool,
}
impl WitnessCalledToBody {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WitnessCalledToBody {
            victim: String::new(),
            t: 0.0,
            breathing: false,
        }
    }
}
impl Default for WitnessCalledToBody { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:561 Witness.sawHeld, src/game/game.ts:561)
#[derive(Clone, Debug)]
pub struct WitnessSawHeld {
    pub who: String,
    pub t: f64,
}
impl WitnessSawHeld {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WitnessSawHeld {
            who: String::new(),
            t: 0.0,
        }
    }
}
impl Default for WitnessSawHeld { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Mission, src/game/game.ts:277)
#[derive(Clone, Debug)]
pub struct Mission {
    pub stage: crate::game::MissionStage,
    pub t0: f64,
    pub drive_t: f64,
    pub pulled: bool,
    pub alarm_on: bool,
    pub sweep_at: f64,
    pub spotted_t: f64,
    pub debrief: bool,
    pub down_card_at: f64,
    pub hands_on: bool,
    pub stats: crate::game::MissionStats,
    /// INVENTED: g.serial() where g.mission is written whole (game.ts:1638, savestate.ts:812, demo.ts:152, 190, 313, 331): dialogue.ts:1717 and lesson.ts:320 ask "the same run?" by identity
    pub serial: u32,
}
impl Mission {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Mission {
            stage: crate::game::MissionStage::Drive,
            t0: 0.0,
            drive_t: 0.0,
            pulled: false,
            alarm_on: false,
            sweep_at: 0.0,
            spotted_t: 0.0,
            debrief: false,
            down_card_at: 0.0,
            hands_on: false,
            stats: crate::game::MissionStats::zeroed(),
            serial: 0,
        }
    }
}
impl Default for Mission { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: MissionStats, src/game/game.ts:276)
#[derive(Clone, Debug)]
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
impl MissionStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MissionStats {
            qrf_who: None,
            elapsed: 0.0,
            alerts: 0.0,
            spotted: 0.0,
            bodies: 0.0,
            shots: 0.0,
            kills: 0.0,
            knockouts: 0.0,
            keys: 0.0,
            blackout: false,
            sandbox: false,
            qrf: 0.0,
            qrf_at: 0.0,
        }
    }
}
impl Default for MissionStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:888 Game.reachStats, src/game/game.ts:888)
#[derive(Clone, Copy, Debug)]
pub struct GameReachStats {
    pub lights: f64,
    pub mean_fraction: f64,
    pub ms: f64,
}
impl GameReachStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameReachStats {
            lights: 0.0,
            mean_fraction: 0.0,
            ms: 0.0,
        }
    }
}
impl Default for GameReachStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:893 Game.reachQueue, src/game/game.ts:893)
#[derive(Clone, Debug)]
pub struct GameReachQueueItem {
    pub l: LightId,
    pub rects: Option<Vec<[f64; 4]>>,
    pub boxes: Vec<[f64; 4]>,
    pub job: Option<crate::render_reach::ReachJob>,
    pub restarts: f64,
}
impl GameReachQueueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameReachQueueItem {
            l: LightId(0),
            rects: None,
            boxes: Vec::new(),
            job: None,
            restarts: 0.0,
        }
    }
}
impl Default for GameReachQueueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:1031 Game.puppet, src/game/game.ts:1031)
#[derive(Clone, Copy, Debug)]
pub struct GamePuppet {
    pub goal: Option<V3>,
    pub aim: Option<V3>,
    pub crouch: Option<bool>,
    pub walk: Option<f64>,
    pub fixtures: Option<bool>,
    pub target: Option<GuardId>,
    pub throw_at: Option<V3>,
}
impl GamePuppet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GamePuppet {
            goal: None,
            aim: None,
            crouch: None,
            walk: None,
            fixtures: None,
            target: None,
            throw_at: None,
        }
    }
}
impl Default for GamePuppet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:1040 Game.alarm, src/game/game.ts:1040)
#[derive(Clone, Copy, Debug)]
pub struct GameAlarm {
    pub episode: bool,
    pub pos: V3,
    pub placed: bool,
    pub apart_t: f64,
    pub raised_at: f64,
}
impl GameAlarm {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameAlarm {
            episode: false,
            pos: [0.0; 3],
            placed: false,
            apart_t: 0.0,
            raised_at: 0.0,
        }
    }
}
impl Default for GameAlarm { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:1054 Game.beacons, src/game/game.ts:1054)
#[derive(Clone, Copy, Debug)]
pub struct GameBeaconsItem {
    pub light: LightId,
    pub phase: f64,
    pub target: FixtureId,
}
impl GameBeaconsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameBeaconsItem {
            light: LightId(0),
            phase: 0.0,
            target: FixtureId(0),
        }
    }
}
impl Default for GameBeaconsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:1056 Game.fx, src/game/game.ts:1056)
#[derive(Clone, Copy, Debug)]
pub struct GameFxItem {
    pub box_: BoxId,
    pub ttl: f64,
}
impl GameFxItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameFxItem {
            box_: BoxId(0),
            ttl: 0.0,
        }
    }
}
impl Default for GameFxItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:1070 Game.messages, src/game/game.ts:1070)
#[derive(Clone, Debug)]
pub struct GameMessagesItem {
    pub text: String,
    pub t: f64,
    /// INVENTED by a review: see the proposals
    pub serial: u32,
}
impl GameMessagesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameMessagesItem {
            text: String::new(),
            t: 0.0,
            serial: 0,
        }
    }
}
impl Default for GameMessagesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:1160 Game.markLights, src/game/game.ts:1160)
#[derive(Clone, Debug)]
pub struct GameMarkLightsItem {
    pub target: String,
    pub box_: f64,
    pub face: String,
    pub pos: V3,
    pub i0: f64,
    pub area: f64,
    pub mean_l: V3,
}
impl GameMarkLightsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameMarkLightsItem {
            target: String::new(),
            box_: 0.0,
            face: String::new(),
            pos: [0.0; 3],
            i0: 0.0,
            area: 0.0,
            mean_l: [0.0; 3],
        }
    }
}
impl Default for GameMarkLightsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:1161 Game.tune, src/game/game.ts:1161)
#[derive(Clone, Copy, Debug)]
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
impl GameTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameTune {
            devices: false,
            lasers: false,
            camera_suspicion_secs: 0.0,
            camera_alarm_secs: 0.0,
            device_cones: false,
            guard_walk: 0.0,
            guard_investigate: 0.0,
            guard_chase: 0.0,
            player_walk: 0.0,
            player_sprint: 0.0,
            player_crouch: 0.0,
            player_light: 0.0,
            flash_intensity: 0.0,
            flash_inner: 0.0,
            flash_outer: 0.0,
            detect_rate: 0.0,
            ocp_duration: 0.0,
            ocp_notice: crate::game::GameTuneOcpNotice::Glance,
            vis_slope: 0.0,
            vis_floor: 0.0,
            floor_materials: false,
            muzzle_discipline: false,
            suppression: false,
            carbine_guards: crate::game::CarbineDeal::None,
            suppress_first: false,
            tracers: false,
            aim_model: false,
            teams: false,
            needs: false,
            tactics: false,
            flank_wait: 0.0,
            lone_wait: 0.0,
            door_danger: 0.0,
            door_wait: 0.0,
            chaos: 0.0,
            wind: 0.0,
            moths: false,
            glass_break: false,
            glass_shards: false,
            wall_break: false,
            debris_impulses: false,
            qrf: false,
            qrf_eta: 0.0,
            qrf_size: 0.0,
            perimeter: false,
            perimeter_eta: 0.0,
            perimeter_size: 0.0,
            qrf_silent: false,
            qrf_squad: false,
            civilians: false,
            civilian_kill_fails: false,
            body_light: false,
        }
    }
}
impl Default for GameTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameTuneOcpNotice {
    Glance,
    Investigate,
}
impl GameTuneOcpNotice {
    pub const fn as_str(self) -> &'static str {
        match self {
            GameTuneOcpNotice::Glance => "glance",
            GameTuneOcpNotice::Investigate => "investigate",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CarbineDeal {
    None,
    Reyes,
    All,
}
impl CarbineDeal {
    pub const fn as_str(self) -> &'static str {
        match self {
            CarbineDeal::None => "none",
            CarbineDeal::Reyes => "reyes",
            CarbineDeal::All => "all",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum GameViewProj {
    Array(Vec<f64>),
    Float32Array(Vec<f32>),
}

/// (TypeScript: Roll, src/game/game.ts:775)
#[derive(Clone, Copy, Debug)]
pub struct Roll {
    pub p: PropId,
    pub x: f64,
    pub z: f64,
    pub k: f64,
    pub move_t: f64,
    pub voice: f64,
}
impl Roll {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Roll {
            p: PropId(0),
            x: 0.0,
            z: 0.0,
            k: 0.0,
            move_t: 0.0,
            voice: 0.0,
        }
    }
}
impl Default for Roll { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AsideOwn, src/game/game.ts:300)
#[derive(Clone, Copy, Debug)]
pub struct AsideOwn {
    pub lens: Option<CameraId>,
}
impl AsideOwn {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        AsideOwn {
            lens: None,
        }
    }
}
impl Default for AsideOwn { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/game.ts:1992 Game.fogHeld, src/game/game.ts:1992)
#[derive(Clone, Debug)]
pub struct GameFogHeld {
    pub dyn_: Vec<BoxId>,
    pub on: bool,
}
impl GameFogHeld {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameFogHeld {
            dyn_: Vec::new(),
            on: false,
        }
    }
}
impl Default for GameFogHeld { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:2254 Game.doorHandRecs, src/game/game.ts:2254)
#[derive(Clone, Copy, Debug)]
pub struct GameDoorHandRecsValue {
    pub door: DoorId,
    pub t: f64,
    pub t0: f64,
    pub crack: bool,
    pub hand: f64,
    pub hold_only: Option<bool>,
    pub move_: Option<bool>,
}
impl GameDoorHandRecsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameDoorHandRecsValue {
            door: DoorId(0),
            t: 0.0,
            t0: 0.0,
            crack: false,
            hand: 0.0,
            hold_only: None,
            move_: None,
        }
    }
}
impl Default for GameDoorHandRecsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PairId {
    Red,
    Blue,
}
impl PairId {
    pub const fn as_str(self) -> &'static str {
        match self {
            PairId::Red => "red",
            PairId::Blue => "blue",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Temperament {
    Steady,
    Nervy,
    Hard,
}
impl Temperament {
    pub const fn as_str(self) -> &'static str {
        match self {
            Temperament::Steady => "steady",
            Temperament::Nervy => "nervy",
            Temperament::Hard => "hard",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MissionRating {
    Ghost,
    Panther,
    Assault,
}
impl MissionRating {
    pub const fn as_str(self) -> &'static str {
        match self {
            MissionRating::Ghost => "ghost",
            MissionRating::Panther => "panther",
            MissionRating::Assault => "assault",
        }
    }
}

/// (TypeScript: Player, src/game/game.ts:301)
#[derive(Clone, Debug)]
pub struct Player {
    pub ch: CharId,
    pub crouch: bool,
    pub aim_held: bool,
    pub last_fire_t: f64,
    pub fire_cd: f64,
    pub backpedal: bool,
    pub sprinting: bool,
    pub dash: bool,
    pub friend_low: bool,
    pub friend_t: f64,
    pub shoved: [f64; 2],
    pub shoved_t: f64,
    pub shoved_from: [f64; 2],
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
    pub stagger_until: f64,
    pub dazzled_until: f64,
    pub dazzled_at: Option<V3>,
    pub stun_posed: bool,
    pub hurt_at: f64,
    pub door_bash_at: f64,
    pub door_bash_lean: f64,
    pub door_rip_until: f64,
    pub door_kick_tail: bool,
    pub door_head: f64,
    pub door_head_t: f64,
    pub sup: crate::suppression::Sup,
    pub aim: crate::aim::AimState,
    pub aimw: crate::aimweight::AimWeight,
    pub hits_left: f64,
    pub down: bool,
    pub pistol: FirearmId,
    pub ocp: crate::weapons::Ocp,
    pub canisters: f64,
    pub flashbangs: f64,
    pub frags: f64,
    pub slot: crate::game::SlotKind,
    pub throw_kind: crate::game::ThrowKind,
    pub rifle: FirearmId,
    pub has_rifle: bool,
    pub equip_t: f64,
    pub setup: Option<crate::game::PlayerSetup>,
    pub setup_hand: Option<crate::game::PlayerSetupHand>,
    pub setup_key: Option<crate::game::PlayerSetupKey>,
    pub rifle_gone: bool,
    pub kit_was: Option<Option<crate::character::KitItem>>,
    pub kit_can: Option<crate::game::ThrowKind>,
    pub launch_held: Option<crate::weapons::RifleAttachment>,
    pub kit_owed: f64,
    pub kit_owed_for: Option<crate::game::PlayerKitOwedFor>,
    pub sling_rest: Option<crate::game::PlayerSlingRest>,
    pub ocp_hinted: bool,
    pub holstered: bool,
    pub keys: JsSet<String>,
    pub codes: JsSet<String>,
    pub coding: Option<DoorId>,
    pub code_t: f64,
    pub breaker_move: Option<crate::game::PlayerBreakerMove>,
    pub hand_wait: f64,
    pub near_sel: i32,
    pub near_key: String,
    pub sel: crate::game::PlayerSel,
    pub pace: f64,
    pub pace_shown_t: f64,
    pub nv: bool,
    pub nv_amount: f64,
    pub light: LightId,
    pub throw_eaten: bool,
    pub throw_held: bool,
    pub held_kind: crate::game::ThrowKind,
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
    pub big_kit: crate::kit::BigKit,
    pub kit_load: Option<crate::kitload::KitWorn>,
    pub cams: Vec<StickyCamId>,
    pub cam_seq: f64,
    pub charging: Option<crate::game::PlayerCharging>,
    pub work_stow: Option<crate::game::PlayerWorkStow>,
    pub wall: Option<crate::player::WallHold>,
    pub safe: Option<V3>,
    pub step_phase: f64,
    pub step_travel: f64,
    pub step_owed: f64,
    pub step_at: f64,
}
impl Player {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Player {
            ch: CharId(0),
            crouch: false,
            aim_held: false,
            last_fire_t: -10.0,
            fire_cd: 0.0,
            backpedal: false,
            sprinting: false,
            dash: false,
            friend_low: false,
            friend_t: -10.0,
            shoved: [0.0, 0.0],
            shoved_t: -1.0,
            shoved_from: [0.0, 0.0],
            haul: None,
            door_move: None,
            dragging: None,
            drag_yaw: 0.0,
            drag_noise_t: 0.0,
            drag_stuck_t: 0.0,
            noise: 0.0,
            takedown: None,
            holding: None,
            catch_due: None,
            bubble: None,
            speed_sm: 0.0,
            visibility: 0.0,
            irradiance: 0.0,
            hit_flash: 0.0,
            stagger_until: -1.0,
            dazzled_until: -1.0,
            dazzled_at: None,
            stun_posed: false,
            hurt_at: -1.0,
            door_bash_at: -10.0,
            door_bash_lean: 0.0,
            door_rip_until: -1.0,
            door_kick_tail: false,
            door_head: 0.0,
            door_head_t: -1.0,
            sup: crate::suppression::Sup::blank(),
            aim: crate::aim::AimState::blank(),
            aimw: crate::aimweight::AimWeight::blank(),
            hits_left: 1.0,
            down: false,
            pistol: FirearmId(0),
            ocp: crate::weapons::Ocp::blank(),
            canisters: 4.0,
            flashbangs: 2.0,
            frags: 2.0,
            slot: crate::game::SlotKind::Pistol,
            throw_kind: crate::game::ThrowKind::Smoke,
            rifle: FirearmId(0),
            has_rifle: false,
            equip_t: 0.0,
            setup: None,
            setup_hand: None,
            setup_key: None,
            rifle_gone: false,
            kit_was: None,
            kit_can: None,
            launch_held: None,
            kit_owed: -1.0,
            kit_owed_for: None,
            sling_rest: None,
            ocp_hinted: false,
            holstered: true,
            keys: JsSet::new(),
            codes: JsSet::new(),
            coding: None,
            code_t: 0.0,
            breaker_move: None,
            hand_wait: 0.0,
            near_sel: 0,
            near_key: String::from(""),
            sel: crate::game::PlayerSel { down_t: -1.0, open: false, moved: false, wheel_acc: 0.0, run_key: String::from(""), move_latch: 0.0, word: String::from(""), word_t: -1000000000.0, file_t: -1000000000.0, ring: None },
            pace: 1.0,
            pace_shown_t: -10.0,
            nv: false,
            nv_amount: 0.0,
            light: LightId(0),
            throw_eaten: false,
            throw_held: false,
            held_kind: crate::game::ThrowKind::Smoke,
            throw_preview: None,
            pending_throw: None,
            easing: None,
            ease_stall_t: 0.0,
            ease_best: 0.0,
            picking: None,
            kick: None,
            f_held: false,
            wand: false,
            wanding: None,
            big_kit: crate::kit::BigKit::None,
            kit_load: None,
            cams: Vec::new(),
            cam_seq: 0.0,
            charging: None,
            work_stow: None,
            wall: None,
            safe: None,
            step_phase: 0.0,
            step_travel: 1.0,
            step_owed: 0.0,
            step_at: -1.0,
        }
    }
}
impl Default for Player { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/game.ts:309 Player.takedown, src/game/game.ts:309)
#[derive(Clone, Copy, Debug)]
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
impl PlayerTakedown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerTakedown {
            g: GuardId(0),
            t: 0.0,
            kind: None,
            from: None,
            alert: None,
            yaw: None,
            at: None,
            lethal: None,
            loud: None,
        }
    }
}
impl Default for PlayerTakedown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:318 Player.bubble, src/game/game.ts:318)
#[derive(Clone, Debug)]
pub struct PlayerBubble {
    pub text: String,
    pub t: f64,
    pub dur: f64,
    pub radio: bool,
}
impl PlayerBubble {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerBubble {
            text: String::new(),
            t: 0.0,
            dur: 0.0,
            radio: false,
        }
    }
}
impl Default for PlayerBubble { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:356 Player.setup, src/game/game.ts:356)
#[derive(Clone, Copy, Debug)]
pub struct PlayerSetup {
    pub gun: crate::slotsetup::SetupGun,
    pub t: f64,
    pub seen: f64,
}
impl PlayerSetup {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerSetup {
            gun: crate::slotsetup::SetupGun::Pistol,
            t: 0.0,
            seen: 0.0,
        }
    }
}
impl Default for PlayerSetup { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:359 Player.setupHand, src/game/game.ts:359)
#[derive(Clone, Copy, Debug)]
pub struct PlayerSetupHand {
    pub gun: Option<crate::slotsetup::SetupGun>,
    pub t: f64,
    pub own: Option<f64>,
}
impl PlayerSetupHand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerSetupHand {
            gun: None,
            t: 0.0,
            own: None,
        }
    }
}
impl Default for PlayerSetupHand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:361 Player.setupKey, src/game/game.ts:361)
#[derive(Clone, Copy, Debug)]
pub struct PlayerSetupKey {
    pub gun: crate::slotsetup::SetupGun,
    pub t: f64,
}
impl PlayerSetupKey {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerSetupKey {
            gun: crate::slotsetup::SetupGun::Pistol,
            t: 0.0,
        }
    }
}
impl Default for PlayerSetupKey { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PlayerKitOwedFor {
    Pistol,
    Rifle,
    Smoke,
    Flash,
    Frag,
    Can,
    Knife,
}
impl PlayerKitOwedFor {
    pub const fn as_str(self) -> &'static str {
        match self {
            PlayerKitOwedFor::Pistol => "pistol",
            PlayerKitOwedFor::Rifle => "rifle",
            PlayerKitOwedFor::Smoke => "smoke",
            PlayerKitOwedFor::Flash => "flash",
            PlayerKitOwedFor::Frag => "frag",
            PlayerKitOwedFor::Can => "can",
            PlayerKitOwedFor::Knife => "knife",
        }
    }
}

/// (TypeScript: inline at src/game/game.ts:373 Player.slingRest, src/game/game.ts:373)
#[derive(Clone, Copy, Debug)]
pub struct PlayerSlingRest {
    pub a: f64,
    pub t: f64,
}
impl PlayerSlingRest {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerSlingRest {
            a: 0.0,
            t: 0.0,
        }
    }
}
impl Default for PlayerSlingRest { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:392 Player.breakerMove, src/game/game.ts:392)
#[derive(Clone, Copy, Debug)]
pub struct PlayerBreakerMove {
    pub on: bool,
    pub t: f64,
    pub thrown: bool,
}
impl PlayerBreakerMove {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerBreakerMove {
            on: false,
            t: 0.0,
            thrown: false,
        }
    }
}
impl Default for PlayerBreakerMove { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:398 Player.sel, src/game/game.ts:398)
#[derive(Clone, Debug)]
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
impl PlayerSel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerSel {
            down_t: 0.0,
            open: false,
            moved: false,
            wheel_acc: 0.0,
            run_key: String::new(),
            move_latch: 0.0,
            word: String::new(),
            word_t: 0.0,
            file_t: 0.0,
            ring: None,
        }
    }
}
impl Default for PlayerSel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:402 PlayerSel.ring, src/game/game.ts:402 PlayerSel.ring)
#[derive(Clone, Debug)]
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
impl PlayerSelRing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerSelRing {
            ax: 0.0,
            ay: 0.0,
            z: 0.0,
            t0: 0.0,
            lit: 0.0,
            armed: false,
            free: None,
            more: None,
            at: None,
            ox: None,
            oy: None,
            mx: None,
            my: None,
            clear: None,
            pt: None,
            key: String::new(),
            whistle: None,
        }
    }
}
impl Default for PlayerSelRing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:411 Player.pendingThrow, src/game/game.ts:411)
#[derive(Clone, Debug)]
pub struct PlayerPendingThrow {
    pub t: f64,
    pub sol: crate::throwables::ThrowSolution,
    pub kind: crate::game::ThrowKind,
}
impl PlayerPendingThrow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerPendingThrow {
            t: 0.0,
            sol: crate::throwables::ThrowSolution::zeroed(),
            kind: crate::game::ThrowKind::Smoke,
        }
    }
}
impl Default for PlayerPendingThrow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:419 Player.kick, src/game/game.ts:419)
#[derive(Clone, Debug)]
pub struct PlayerKick {
    pub door: DoorId,
    pub spot: V3,
    pub yaw: f64,
    pub run: Option<bool>,
    pub tool: Option<crate::game::PlayerKickTool>,
}
impl PlayerKick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerKick {
            door: DoorId(0),
            spot: [0.0; 3],
            yaw: 0.0,
            run: None,
            tool: None,
        }
    }
}
impl Default for PlayerKick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:419 PlayerKick.tool, src/game/game.ts:419)
#[derive(Clone, Debug)]
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
impl PlayerKickTool {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerKickTool {
            kind: crate::game::PlayerKickToolKind::Shotgun,
            side: 0.0,
            t: 0.0,
            lead: None,
            n: 0.0,
            need: 0.0,
            next: 0.0,
            done: 0.0,
            gave: None,
            moves: None,
        }
    }
}
impl Default for PlayerKickTool { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PlayerKickToolKind {
    Shotgun,
    Hammer,
}
impl PlayerKickToolKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            PlayerKickToolKind::Shotgun => "shotgun",
            PlayerKickToolKind::Hammer => "hammer",
        }
    }
}

/// (TypeScript: inline at src/game/game.ts:420 PlayerKickTool.moves, src/game/game.ts:420)
#[derive(Clone, Debug)]
pub struct PlayerKickToolMoves {
    pub arrived: bool,
    pub at: V3,
    pub blows: Vec<f64>,
    pub end: f64,
    pub via: Option<V3>,
}
impl PlayerKickToolMoves {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerKickToolMoves {
            arrived: false,
            at: [0.0; 3],
            blows: Vec::new(),
            end: 0.0,
            via: None,
        }
    }
}
impl Default for PlayerKickToolMoves { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:434 Player.charging, src/game/game.ts:434)
#[derive(Clone, Copy, Debug)]
pub struct PlayerCharging {
    pub door: DoorId,
    pub side: f64,
    pub t0: f64,
    pub at: V3,
}
impl PlayerCharging {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerCharging {
            door: DoorId(0),
            side: 0.0,
            t0: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for PlayerCharging { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:437 Player.workStow, src/game/game.ts:437)
#[derive(Clone, Copy, Debug)]
pub struct PlayerWorkStow {
    pub drew: bool,
    pub t: f64,
    pub down: bool,
}
impl PlayerWorkStow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerWorkStow {
            drew: false,
            t: 0.0,
            down: false,
        }
    }
}
impl Default for PlayerWorkStow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Talk, src/game/game.ts:573)
#[derive(Clone, Debug)]
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
impl Talk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Talk {
            stage: 0.0,
            presses: 0.0,
            said: JsSet::new(),
            said_n: JsMap::new(),
            given: JsSet::new(),
            last_t: 0.0,
            held_since: 0.0,
            abort_t: 0.0,
            last: None,
        }
    }
}
impl Default for Talk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:573 Talk.last, src/game/game.ts:573)
#[derive(Clone, Debug)]
pub struct TalkLast {
    pub id: String,
    pub cell: String,
    pub stage: String,
    pub kind: Option<String>,
}
impl TalkLast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TalkLast {
            id: String::new(),
            cell: String::new(),
            stage: String::new(),
            kind: None,
        }
    }
}
impl Default for TalkLast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum GuardTask {
    Arm0(crate::game::GuardTaskArm0),
    CheckOn(crate::game::GuardTaskCheckOn),
}

/// (TypeScript: inline at src/game/game.ts:577 GuardTask, src/game/game.ts:577)
#[derive(Clone, Copy, Debug)]
pub struct GuardTaskArm0 {
    pub kind: crate::game::GuardTaskArm0Kind,
    pub pos: V3,
}
impl GuardTaskArm0 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardTaskArm0 {
            kind: crate::game::GuardTaskArm0Kind::Breaker,
            pos: [0.0; 3],
        }
    }
}
impl Default for GuardTaskArm0 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GuardTaskArm0Kind {
    Breaker,
    Device,
    Rack,
}
impl GuardTaskArm0Kind {
    pub const fn as_str(self) -> &'static str {
        match self {
            GuardTaskArm0Kind::Breaker => "breaker",
            GuardTaskArm0Kind::Device => "device",
            GuardTaskArm0Kind::Rack => "rack",
        }
    }
}

/// (TypeScript: inline at src/game/game.ts:577 GuardTask, src/game/game.ts:577)
#[derive(Clone, Copy, Debug)]
pub struct GuardTaskCheckOn {
    pub pos: V3,
    pub who: GuardId,
    pub wp: f64,
    pub left: f64,
    pub t0: f64,
}
impl GuardTaskCheckOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardTaskCheckOn {
            pos: [0.0; 3],
            who: GuardId(0),
            wp: 0.0,
            left: 0.0,
            t0: 0.0,
        }
    }
}
impl Default for GuardTaskCheckOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: UnitInfo, src/game/game.ts:580)
#[derive(Clone, Debug)]
pub struct UnitInfo {
    pub name: String,
    pub spawned_at: f64,
    pub light: bool,
    pub pair: Option<crate::game::PairId>,
    pub seat: Option<f64>,
}
impl UnitInfo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnitInfo {
            name: String::new(),
            spawned_at: 0.0,
            light: false,
            pair: None,
            seat: None,
        }
    }
}
impl Default for UnitInfo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Guard, src/game/game.ts:589)
#[derive(Clone, Debug)]
pub struct Guard {
    pub ch: CharId,
    pub state: crate::game::GuardState,
    pub callsign: String,
    pub team: crate::teams::Team,
    /// may name a man who has left: a GuardId stays readable, tables never shrink
    pub foe: Option<crate::game::Foe>,
    pub faces: Option<f64>,
    pub route: RouteId,
    pub unit: Option<crate::game::UnitInfo>,
    pub civ: Option<CivId>,
    pub trained: bool,
    pub skill: Option<crate::game::GuardSkill>,
    pub visibility: f64,
    pub irradiance: f64,
    pub route_i: usize,
    pub wp: usize,
    pub wait: f64,
    pub awareness: f64,
    /// written whole. doze.ts:244 asks whether a new fix was written: see fix_stamp
    pub last_known: Option<V3>,
    pub last_seen_t: f64,
    pub hold: bool,
    pub pinned: bool,
    pub god: bool,
    pub held: Option<crate::game::GuardHeld>,
    pub bubble: Option<crate::game::GuardBubble>,
    pub found: bool,
    pub down_kind: Option<crate::game::DownKind>,
    pub down_by: Option<crate::game::DownBy>,
    pub wound: Option<crate::wounds::Wound>,
    pub witness: crate::game::Witness,
    pub talk: GameTalkId,
    pub keys: JsSet<String>,
    pub missed_at: f64,
    pub missing_raised: bool,
    pub muzzle_down: bool,
    pub muzzle_clear_t: f64,
    pub muzzle_eval_t: f64,
    pub line_bark_t: f64,
    pub line_in_t: f64,
    pub line_side: f64,
    pub line_gain: f64,
    pub line_step_m: f64,
    pub line_stuck_t: f64,
    pub line_fail: Option<crate::game::GuardLineFail>,
    pub leader_seen_t: f64,
    pub leader_seen_pos: V3,
    pub body_visits: f64,
    pub body_visit_t: f64,
    pub body_remarked: bool,
    pub body_closed: bool,
    pub body_fan: f64,
    pub body_duty: Option<crate::game::GuardBodyDuty>,
    pub dropped: Option<ItemId>,
    pub gun_lost_at: f64,
    pub torch_lost_at: f64,
    /// compared by identity in doorway.ts, doorpath.ts, guards.ts: see path_serial
    pub path: Vec<V3>,
    pub path_i: usize,
    pub repath_t: f64,
    /// guards.ts asks whether the back-off laid it: see path_goal_orbit
    pub path_goal: Option<V3>,
    pub weapon: FirearmId,
    pub fire_cd: f64,
    pub reload_t: f64,
    pub carbine: bool,
    pub sup: crate::suppression::Sup,
    pub aim: crate::aim::AimState,
    pub react_t: f64,
    pub search_t: f64,
    pub look_phase: f64,
    pub speed: f64,
    pub flashlight: LightId,
    pub sweep: f64,
    pub beam_dir: V3,
    pub step_phase: f64,
    pub heard_up_to: f64,
    pub smoke_trans: f64,
    pub light_dead_until: f64,
    pub died_at: f64,
    pub slung: Option<crate::game::GuardSlung>,
    pub second: Option<crate::game::GuardSecond>,
    pub big_out: Option<crate::game::GuardBigOut>,
    pub work_stow: Option<crate::game::GuardWorkStow>,
    pub light_lit_at_loss: bool,
    pub dazzled_at: Option<V3>,
    pub dazzled_until: f64,
    pub stagger_until: f64,
    pub door_down_until: f64,
    pub hurt_at: f64,
    pub saw_player_this_frame: bool,
    pub last_pos: V3,
    pub stuck_t: f64,
    pub stuck_n: f64,
    pub saw_foe_this_frame: bool,
    pub killer_id: f64,
    pub saw_body_t: f64,
    pub leader_still_t: f64,
    pub alert_best: f64,
    pub alert_stall_t: f64,
    pub alert_ref: Option<V3>,
    pub chase_t: f64,
    pub shot_blocked_t: f64,
    pub sight_pos: V3,
    pub sight_vel: V3,
    pub sight_t: f64,
    pub led_t: f64,
    pub fix_t: f64,
    pub fix_ref: Option<V3>,
    pub search_plan: Option<crate::guards::SearchPlan>,
    pub pieing: Option<crate::guards::Pieing>,
    pub pie_last: Option<DoorId>,
    pub pie_last_t: f64,
    pub pie_prev: Option<V3>,
    pub pie_prev_t: f64,
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
    /// a bit for each rule with a hand on him (owner.ts)
    pub owner: u32,
    /// the place of that top bit in the ranks
    pub rank: u32,
    /// which of the top rule's reasons it gave
    pub reason: u32,
    /// the game time the claim now on top came to the top
    pub since: f64,
    /// INVENTED: bumped by every whole write of last_known, for the identity question at doze.ts:244
    pub fix_stamp: u32,
    /// INVENTED: bumped by every whole replacement of path
    pub path_serial: u32,
    /// INVENTED: path_goal is the very point the standoff laid (guards.ts)
    pub path_goal_orbit: bool,
}
impl Guard {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Guard {
            ch: CharId(0),
            state: crate::game::GuardState::Patrol,
            callsign: String::new(),
            team: crate::teams::Team::Guard,
            foe: Some(crate::game::Foe::Player),
            faces: None,
            route: RouteId(0),
            unit: None,
            civ: None,
            trained: false,
            skill: None,
            visibility: 0.0,
            irradiance: 0.0,
            route_i: 0,
            wp: 0,
            wait: 0.0,
            awareness: 0.0,
            last_known: None,
            last_seen_t: -100.0,
            hold: false,
            pinned: false,
            god: false,
            held: None,
            bubble: None,
            found: false,
            down_kind: None,
            down_by: None,
            wound: None,
            witness: crate::game::Witness { heard: crate::game::WitnessHeard { shots: 0.0, last_shot_t: -1.0, kick: 0.0, bang: 0.0, small: 0.0, small_where: None, last_t: -1.0 }, peak_awareness: 0.0, alerted_by: None, alert_t: -1.0, saw_act: None, saw_body: None, called_to_body: None, saw_held: None, was_held: 0.0, dazzled: 0.0 },
            talk: GameTalkId(0),
            keys: JsSet::new(),
            missed_at: -1.0,
            missing_raised: false,
            muzzle_down: false,
            muzzle_clear_t: 0.0,
            muzzle_eval_t: -1.0,
            line_bark_t: -100.0,
            line_in_t: 0.0,
            line_side: 0.0,
            line_gain: -1.0,
            line_step_m: 0.0,
            line_stuck_t: 0.0,
            line_fail: None,
            leader_seen_t: -1.0,
            leader_seen_pos: [0.0, 0.0, 0.0],
            body_visits: 0.0,
            body_visit_t: -1.0,
            body_remarked: false,
            body_closed: false,
            body_fan: 0.0,
            body_duty: None,
            dropped: None,
            gun_lost_at: -1.0,
            torch_lost_at: -1.0,
            path: Vec::new(),
            path_i: 0,
            repath_t: 0.0,
            path_goal: None,
            weapon: FirearmId(0),
            fire_cd: 1.0,
            reload_t: -1.0,
            carbine: false,
            sup: crate::suppression::Sup::blank(),
            aim: crate::aim::AimState::blank(),
            react_t: 0.0,
            search_t: 0.0,
            look_phase: 0.0,
            speed: 0.0,
            flashlight: LightId(0),
            sweep: 0.0,
            beam_dir: [0.0, -0.3, 1.0],
            step_phase: 0.0,
            heard_up_to: -1.0,
            smoke_trans: 1.0,
            light_dead_until: -1.0,
            died_at: -1.0,
            slung: None,
            second: None,
            big_out: None,
            work_stow: None,
            light_lit_at_loss: true,
            dazzled_at: None,
            dazzled_until: -1.0,
            stagger_until: -1.0,
            door_down_until: -1.0,
            hurt_at: -1.0,
            saw_player_this_frame: false,
            last_pos: [0.0; 3],
            stuck_t: 0.0,
            stuck_n: 0.0,
            saw_foe_this_frame: false,
            killer_id: -1.0,
            saw_body_t: -100.0,
            leader_still_t: 0.0,
            alert_best: 1000000000.0,
            alert_stall_t: 0.0,
            alert_ref: None,
            chase_t: 0.0,
            shot_blocked_t: 0.0,
            sight_pos: [0.0, 0.0, 0.0],
            sight_vel: [0.0, 0.0, 0.0],
            sight_t: -100.0,
            led_t: -100.0,
            fix_t: -100.0,
            fix_ref: None,
            search_plan: None,
            pieing: None,
            pie_last: None,
            pie_last_t: -100.0,
            pie_prev: None,
            pie_prev_t: -100.0,
            standoff: None,
            stall_ref: None,
            stall_t: 0.0,
            script: None,
            task: None,
            drawn: false,
            leader: None,
            follow_t: 0.0,
            follow_goal: [0.0, 0.0, 0.0],
            post: None,
            owner: 0,
            rank: 0,
            reason: 0,
            since: 0.0,
            fix_stamp: 0,
            path_serial: 0,
            path_goal_orbit: false,
        }
    }
}
impl Default for Guard { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/game.ts:615 Guard.skill, src/game/game.ts:615)
#[derive(Clone, Copy, Debug)]
pub struct GuardSkill {
    pub settle_k: f64,
    pub spread_k: f64,
}
impl GuardSkill {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardSkill {
            settle_k: 0.0,
            spread_k: 0.0,
        }
    }
}
impl Default for GuardSkill { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:632 Guard.held, src/game/game.ts:632)
#[derive(Clone, Copy, Debug)]
pub struct GuardHeld {
    pub by: crate::game::GuardHeldBy,
    pub since: f64,
    pub variant: crate::character::HoldVariant,
    pub phase: crate::character::HoldPhase,
    pub reverse: bool,
}
impl GuardHeld {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardHeld {
            by: crate::game::GuardHeldBy::Player,
            since: 0.0,
            variant: crate::character::HoldVariant::Gun,
            phase: crate::character::HoldPhase::Held,
            reverse: false,
        }
    }
}
impl Default for GuardHeld { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GuardHeldBy {
    Player,
}
impl GuardHeldBy {
    pub const fn as_str(self) -> &'static str {
        match self {
            GuardHeldBy::Player => "player",
        }
    }
}

/// (TypeScript: inline at src/game/game.ts:633 Guard.bubble, src/game/game.ts:633)
#[derive(Clone, Debug)]
pub struct GuardBubble {
    pub text: String,
    pub t: f64,
    pub dur: f64,
    pub radio: bool,
}
impl GuardBubble {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardBubble {
            text: String::new(),
            t: 0.0,
            dur: 0.0,
            radio: false,
        }
    }
}
impl Default for GuardBubble { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:663 Guard.lineFail, src/game/game.ts:663)
#[derive(Clone, Copy, Debug)]
pub struct GuardLineFail {
    pub pos: V3,
    pub sides: f64,
    pub t: f64,
}
impl GuardLineFail {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardLineFail {
            pos: [0.0; 3],
            sides: 0.0,
            t: 0.0,
        }
    }
}
impl Default for GuardLineFail { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:672 Guard.bodyDuty, src/game/game.ts:672)
#[derive(Clone, Copy, Debug)]
pub struct GuardBodyDuty {
    pub body: GuardId,
    pub role: crate::game::GuardBodyDutyRole,
    pub stage: crate::game::GuardBodyDutyStage,
    pub t: f64,
    pub t0: f64,
    pub spot: Option<V3>,
}
impl GuardBodyDuty {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardBodyDuty {
            body: GuardId(0),
            role: crate::game::GuardBodyDutyRole::Finder,
            stage: crate::game::GuardBodyDutyStage::Fan,
            t: 0.0,
            t0: 0.0,
            spot: None,
        }
    }
}
impl Default for GuardBodyDuty { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GuardBodyDutyRole {
    Finder,
    Called,
}
impl GuardBodyDutyRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            GuardBodyDutyRole::Finder => "finder",
            GuardBodyDutyRole::Called => "called",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GuardBodyDutyStage {
    Fan,
    To,
    Cover,
}
impl GuardBodyDutyStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            GuardBodyDutyStage::Fan => "fan",
            GuardBodyDutyStage::To => "to",
            GuardBodyDutyStage::Cover => "cover",
        }
    }
}

/// (TypeScript: inline at src/game/game.ts:696 Guard.slung, src/game/game.ts:696)
#[derive(Clone, Copy, Debug)]
pub struct GuardSlung {
    pub own: FirearmId,
    pub own_id: Option<crate::weapons::WeaponId>,
    pub ahead: f64,
    pub gun: FirearmId,
    pub out: bool,
    pub t: f64,
}
impl GuardSlung {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardSlung {
            own: FirearmId(0),
            own_id: None,
            ahead: 0.0,
            gun: FirearmId(0),
            out: false,
            t: 0.0,
        }
    }
}
impl Default for GuardSlung { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:699 Guard.second, src/game/game.ts:699)
#[derive(Clone, Copy, Debug)]
pub struct GuardSecond {
    pub gun: FirearmId,
    pub id: crate::weapons::WeaponId,
    pub ahead: f64,
    pub out: bool,
    pub t: f64,
    pub lock: f64,
    pub shots: f64,
}
impl GuardSecond {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardSecond {
            gun: FirearmId(0),
            id: crate::weapons::WeaponId::Fiveseven,
            ahead: 0.0,
            out: false,
            t: 0.0,
            lock: 0.0,
            shots: 0.0,
        }
    }
}
impl Default for GuardSecond { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:702 Guard.bigOut, src/game/game.ts:702)
#[derive(Clone, Debug)]
pub struct GuardBigOut {
    pub t0: f64,
    pub blows: Vec<f64>,
    pub end: f64,
    pub at: V3,
}
impl GuardBigOut {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardBigOut {
            t0: 0.0,
            blows: Vec::new(),
            end: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for GuardBigOut { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:705 Guard.workStow, src/game/game.ts:705)
#[derive(Clone, Copy, Debug)]
pub struct GuardWorkStow {
    pub up_t: f64,
    pub fed: bool,
}
impl GuardWorkStow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardWorkStow {
            up_t: 0.0,
            fed: false,
        }
    }
}
impl Default for GuardWorkStow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:769 ROLL_VOICE, src/game/game.ts:769)
#[derive(Clone, Copy, Debug)]
pub struct ROLLVOICE {
    pub gain: f64,
    pub full_v: f64,
    pub min_v: f64,
    pub grace: f64,
}
impl ROLLVOICE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ROLLVOICE {
            gain: 0.0,
            full_v: 0.0,
            min_v: 0.0,
            grace: 0.0,
        }
    }
}
impl Default for ROLLVOICE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:773 SMOKE_STIR, src/game/game.ts:773)
#[derive(Clone, Copy, Debug)]
pub struct SMOKESTIR {
    pub body: f64,
    pub foot: f64,
    pub above: f64,
    pub back: f64,
    pub low: f64,
    pub kick_r: f64,
    pub kick: f64,
    pub most: f64,
    pub gap: f64,
}
impl SMOKESTIR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SMOKESTIR {
            body: 0.0,
            foot: 0.0,
            above: 0.0,
            back: 0.0,
            low: 0.0,
            kick_r: 0.0,
            kick: 0.0,
            most: 0.0,
            gap: 0.0,
        }
    }
}
impl Default for SMOKESTIR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:752 constructor(look), src/game/game.ts:752)
#[derive(Clone, Debug)]
pub struct GuardNewLook {
    pub tint2: Option<V3>,
    pub yaw: Option<f64>,
    pub name: Option<String>,
    pub team: Option<crate::teams::Team>,
}
impl GuardNewLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardNewLook {
            tint2: None,
            yaw: None,
            name: None,
            team: None,
        }
    }
}
impl Default for GuardNewLook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:1201 constructor(opts), src/game/game.ts:1201)
#[derive(Clone, Copy, Debug)]
pub struct GameNewOpts {
    pub seats: Option<f64>,
}
impl GameNewOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameNewOpts {
            seats: None,
        }
    }
}
impl Default for GameNewOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameFreeHeldWhy {
    Shot,
    Cancel,
}
impl GameFreeHeldWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            GameFreeHeldWhy::Shot => "shot",
            GameFreeHeldWhy::Cancel => "cancel",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameSetBlackoutCause {
    Shot,
    Ocp,
    Debug,
    Hand,
}
impl GameSetBlackoutCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GameSetBlackoutCause::Shot => "shot",
            GameSetBlackoutCause::Ocp => "ocp",
            GameSetBlackoutCause::Debug => "debug",
            GameSetBlackoutCause::Hand => "hand",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameLightEventCause {
    Shot,
    Ocp,
}
impl GameLightEventCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GameLightEventCause::Shot => "shot",
            GameLightEventCause::Ocp => "ocp",
        }
    }
}

/// (TypeScript: inline at src/game/game.ts:1988 viewPx(cam), src/game/game.ts:1988)
#[derive(Clone, Copy, Debug)]
pub struct GameViewPxCam {
    pub distance: f64,
    pub fov: f64,
}
impl GameViewPxCam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameViewPxCam {
            distance: 0.0,
            fov: 0.0,
        }
    }
}
impl Default for GameViewPxCam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/game.ts:1988 viewPx(canvas), src/game/game.ts:1988)
#[derive(Clone, Copy, Debug)]
pub struct GameViewPxCanvas {
    pub height: f64,
}
impl GameViewPxCanvas {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameViewPxCanvas {
            height: 0.0,
        }
    }
}
impl Default for GameViewPxCanvas { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameSelectVerbKind {
    Cam,
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
impl GameSelectVerbKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            GameSelectVerbKind::Cam => "cam",
            GameSelectVerbKind::Guard => "guard",
            GameSelectVerbKind::Breaker => "breaker",
            GameSelectVerbKind::Door => "door",
            GameSelectVerbKind::Light => "light",
            GameSelectVerbKind::Body => "body",
            GameSelectVerbKind::Objective => "objective",
            GameSelectVerbKind::Held => "held",
            GameSelectVerbKind::Wall => "wall",
            GameSelectVerbKind::Switch => "switch",
            GameSelectVerbKind::Charge => "charge",
            GameSelectVerbKind::Cancel => "cancel",
        }
    }
}

/// (src/game/game.ts:41 MARK_LIGHT_MIN_I0)
pub const MARK_LIGHT_MIN_I0: f64 = 0.05;

/// (src/game/game.ts:249 KIT_ORDER: a table nothing writes)
pub fn kit_order() -> Vec<String> { alloc::vec![String::from("pistol"), String::from("rifle"), String::from("smoke"), String::from("flash"), String::from("frag")] }

/// (src/game/game.ts:296 BODY_DOOR_PARTICLES: a table nothing writes)
pub fn body_door_particles() -> Vec<f64> { alloc::vec![0.0, 1.0, 3.0, 12.0, 13.0, 14.0, 15.0] }

/// (src/game/game.ts:508 CALLSIGNS: a table nothing writes)
pub fn callsigns() -> Vec<String> { alloc::vec![String::from("Kowalski"), String::from("Novak"), String::from("Reyes"), String::from("Okafor"), String::from("Lindqvist"), String::from("Brandt"), String::from("Marsh")] }

/// (src/game/game.ts:511 CARBINE_DEALS: a table nothing writes)
pub fn carbine_deals() -> Vec<String> { alloc::vec![String::from("none"), String::from("reyes"), String::from("all")] }

/// (src/game/game.ts:516 TEMPERAMENT: a table nothing writes)
pub fn temperament() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("Kowalski"), String::from("steady")); t.set(String::from("Novak"), String::from("nervy")); t.set(String::from("Reyes"), String::from("hard")); t.set(String::from("Okafor"), String::from("steady")); t.set(String::from("Lindqvist"), String::from("nervy")); t.set(String::from("Brandt"), String::from("hard")); t.set(String::from("Marsh"), String::from("steady")); t } }

/// (src/game/game.ts:769 ROLL_VOICE: a table nothing writes)
pub fn roll_voice() -> crate::game::ROLLVOICE { crate::game::ROLLVOICE { gain: 0.5, full_v: 2.5, min_v: 0.12, grace: 0.5 } }

/// (src/game/game.ts:773 SMOKE_STIR: a table nothing writes)
pub fn smoke_stir() -> crate::game::SMOKESTIR { crate::game::SMOKESTIR { body: 0.24, foot: 0.12, above: 0.5, back: 0.3, low: 0.2, kick_r: 0.17, kick: 0.9, most: 2.2, gap: 0.35 } }

impl Guard {
    /// (game.ts Guard's constructor) look_phase: simRandom() * 10, DRAWN BEFORE his Character is made, as the field initialiser runs first. foe goes None for a side not hostile to the player. His body's muzzle_ahead is the weapon's row: the caller sets it, as the constructor does
    pub fn new(ch: CharId, callsign: String, team: Option<crate::teams::Team>, route: RouteId, route_i: usize, flashlight: LightId, keys: JsSet<String>, weapon: FirearmId, pos: V3, look_phase: f64, talk: GameTalkId) -> Self {
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
#[derive(Clone, Debug)]
pub struct Tabs {
    pub clip: Tab<ClipId, crate::anim_gltf::Clip>,
    pub anim_gltf_mesh_data: Tab<AnimGltfMeshDataId, crate::anim_gltf::MeshData>,
    pub anim_gltf_cast_variant: Tab<AnimGltfCastVariantId, crate::anim_gltf::CastVariant>,
    pub anim_gltf_cast_info: Tab<AnimGltfCastInfoId, crate::anim_gltf::CastInfo>,
    pub rig: Tab<RigId, crate::anim_gltf::GltfCharacter>,
    pub ragdoll: Tab<RagdollId, crate::ragdoll::Ragdoll>,
    pub props_prop: Tab<PropId, crate::props::Prop>,
    pub input: Tab<InputId, crate::input::Input>,
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
    pub roomfight_def: Tab<RoomfightDefId, crate::roomfight::Def>,
    pub roomfight_room: Tab<RoomfightRoomId, crate::roomfight::Room>,
    pub stickycam_sticky_cam: Tab<StickyCamId, crate::stickycam::StickyCam>,
    pub doortools_door_op: Tab<DoorOpId, crate::doortools::DoorOp>,
    pub orders_assignment: Tab<OrdersAssignmentId, crate::orders::Assignment>,
    pub doortools_room_search: Tab<DoortoolsRoomSearchId, crate::doortools::RoomSearch>,
    pub doortools_word: Tab<WordId, crate::doortools::Word>,
    pub doortools_door_gate: Tab<DoorGateId, crate::doortools::DoorGate>,
    pub doortools_known_threat: Tab<DoortoolsKnownThreatId, crate::doortools::KnownThreat>,
    pub doortools_set_charge: Tab<ChargeId, crate::doortools::SetCharge>,
    pub orders_order_def: Tab<OrdersOrderDefId, crate::orders::OrderDef>,
    pub cover_opening: Tab<CoverOpeningId, crate::cover::Opening>,
    pub wallreact_breach: Tab<WallreactBreachId, crate::wallreact::Breach>,
    pub tactics_role: Tab<TacticsRoleId, crate::tactics::Role>,
    pub tactics_plan: Tab<TacticsPlanId, crate::tactics::Plan>,
    pub tactics_danger_rec: Tab<TacticsDangerRecId, crate::tactics::DangerRec>,
    pub pressure_man: Tab<PressureManId, crate::pressure::Man>,
    pub roomfight_op: Tab<RoomfightOpId, crate::roomfight::Op>,
    pub enemysquads_squad: Tab<EnemysquadsSquadId, crate::enemysquads::Squad>,
    pub rescueflow_region: Tab<RescueflowRegionId, crate::rescueflow::Region>,
    pub wallcharge_set_wall_charge: Tab<WallChargeId, crate::wallcharge::SetWallCharge>,
    pub director_live_beat: Tab<DirectorLiveBeatId, crate::director::LiveBeat>,
    pub squad_room_clear: Tab<RoomClearId, crate::squad::RoomClear>,
    pub director_run: Tab<DirectorRunId, crate::director::Run>,
    pub wallcharge_wall_op: Tab<WallOpId, crate::wallcharge::WallOp>,
    pub route: Tab<RouteId, crate::scene_level::PatrolRoute>,
    pub guards_standoff: Tab<GuardsStandoffId, crate::guards::Standoff>,
    pub game_talk: Tab<GameTalkId, crate::game::Talk>,
}
impl Tabs {
    pub fn blank() -> Self {
        Tabs {
            clip: Tab::new(),
            anim_gltf_mesh_data: Tab::new(),
            anim_gltf_cast_variant: Tab::new(),
            anim_gltf_cast_info: Tab::new(),
            rig: Tab::new(),
            ragdoll: Tab::new(),
            props_prop: Tab::new(),
            input: Tab::new(),
            camera: Tab::new(),
            item: Tab::new(),
            debris_chunk: Tab::new(),
            cmd_seat: Tab::new(),
            light: Tab::new(),
            sim_lightshome_fixture: Tab::new(),
            devices_camera_rt: Tab::new(),
            devices_laser_rt: Tab::new(),
            wallbreak_breakable: Tab::new(),
            furnblast_member: Tab::new(),
            wind_wind_source: Tab::new(),
            roomfight_def: Tab::new(),
            roomfight_room: Tab::new(),
            stickycam_sticky_cam: Tab::new(),
            doortools_door_op: Tab::new(),
            orders_assignment: Tab::new(),
            doortools_room_search: Tab::new(),
            doortools_word: Tab::new(),
            doortools_door_gate: Tab::new(),
            doortools_known_threat: Tab::new(),
            doortools_set_charge: Tab::new(),
            orders_order_def: Tab::new(),
            cover_opening: Tab::new(),
            wallreact_breach: Tab::new(),
            tactics_role: Tab::new(),
            tactics_plan: Tab::new(),
            tactics_danger_rec: Tab::new(),
            pressure_man: Tab::new(),
            roomfight_op: Tab::new(),
            enemysquads_squad: Tab::new(),
            rescueflow_region: Tab::new(),
            wallcharge_set_wall_charge: Tab::new(),
            director_live_beat: Tab::new(),
            squad_room_clear: Tab::new(),
            director_run: Tab::new(),
            wallcharge_wall_op: Tab::new(),
            route: Tab::new(),
            guards_standoff: Tab::new(),
            game_talk: Tab::new(),
        }
    }
}

/// each module's top-level state: g.m.<module>
#[derive(Clone, Debug)]
pub struct Mods {
    pub gpu_wgsl_trim: crate::gpu_wgsl_trim::GpuWgslTrimMod,
    pub cloud_report: crate::cloud_report::CloudReportMod,
    pub ui_shaderrows: crate::ui_shaderrows::UiShaderrowsMod,
    pub gpu_keep: crate::gpu_keep::GpuKeepMod,
    pub gpu_device: crate::gpu_device::GpuDeviceMod,
    pub math_side: crate::math_side::MathSideMod,
    pub scene_cut: crate::scene_cut::SceneCutMod,
    pub scene_ground_field: crate::scene_ground_field::SceneGroundFieldMod,
    pub math_exact: crate::math_exact::MathExactMod,
    pub scene_markings: crate::scene_markings::SceneMarkingsMod,
    pub scene_round: crate::scene_round::SceneRoundMod,
    pub scene_materials: crate::scene_materials::SceneMaterialsMod,
    pub thinline: crate::thinline::ThinlineMod,
    pub scene_look: crate::scene_look::SceneLookMod,
    pub anim_gltf: crate::anim_gltf::AnimGltfMod,
    pub render_sdfrig: crate::render_sdfrig::RenderSdfrigMod,
    pub sim_sim: crate::sim_sim::SimSimMod,
    pub sim_state: crate::sim_state::SimStateMod,
    pub sim_propshome: crate::sim_propshome::SimPropshomeMod,
    pub scene_box_mirror: crate::scene_box_mirror::SceneBoxMirrorMod,
    pub scene_collision: crate::scene_collision::SceneCollisionMod,
    pub render_place_field: crate::render_place_field::RenderPlaceFieldMod,
    pub render_snow_field: crate::render_snow_field::RenderSnowFieldMod,
    pub render_char_materials: crate::render_char_materials::RenderCharMaterialsMod,
    pub render_mesh_shadows: crate::render_mesh_shadows::RenderMeshShadowsMod,
    pub anim_skeleton: crate::anim_skeleton::AnimSkeletonMod,
    pub sim_men_store: crate::sim_men_store::SimMenStoreMod,
    pub packside: crate::packside::PacksideMod,
    pub rig_props: crate::rig_props::RigPropsMod,
    pub math_dice: crate::math_dice::MathDiceMod,
    pub character: crate::character::CharacterMod,
    pub sim_doorshome: crate::sim_doorshome::SimDoorshomeMod,
    pub sim_slabshome: crate::sim_slabshome::SimSlabshomeMod,
    pub props: crate::props::PropsMod,
    pub teams: crate::teams::TeamsMod,
    pub host_keymap: crate::host_keymap::HostKeymapMod,
    pub opening: crate::opening::OpeningMod,
    pub weather_kinds: crate::weather_kinds::WeatherKindsMod,
    pub kitload: crate::kitload::KitloadMod,
    pub holdfire: crate::holdfire::HoldfireMod,
    pub destruction_switch: crate::destruction_switch::DestructionSwitchMod,
    pub dress: crate::dress::DressMod,
    pub sim_fallshome: crate::sim_fallshome::SimFallshomeMod,
    pub look_decals: crate::look_decals::LookDecalsMod,
    pub sim_lightshome: crate::sim_lightshome::SimLightshomeMod,
    pub wallbreak: crate::wallbreak::WallbreakMod,
    pub furnblast: crate::furnblast::FurnblastMod,
    pub doorblast: crate::doorblast::DoorblastMod,
    pub fire: crate::fire::FireMod,
    pub edge_look: crate::edge_look::EdgeLookMod,
    pub winbars: crate::winbars::WinbarsMod,
    pub destruction: crate::destruction::DestructionMod,
    pub glass: crate::glass::GlassMod,
    pub glasspen: crate::glasspen::GlasspenMod,
    pub netquietfeed: crate::netquietfeed::NetquietfeedMod,
    pub netquiet: crate::netquiet::NetquietMod,
    pub laststand: crate::laststand::LaststandMod,
    pub doorway: crate::doorway::DoorwayMod,
    pub difficulty: crate::difficulty::DifficultyMod,
    pub stickycam: crate::stickycam::StickycamMod,
    pub quietfog: crate::quietfog::QuietfogMod,
    pub nightbreach: crate::nightbreach::NightbreachMod,
    pub handleside: crate::handleside::HandlesideMod,
    pub doornames: crate::doornames::DoornamesMod,
    pub stackroom: crate::stackroom::StackroomMod,
    pub rearwatch: crate::rearwatch::RearwatchMod,
    pub wallpen: crate::wallpen::WallpenMod,
    pub coverfire: crate::coverfire::CoverfireMod,
    pub wallreact: crate::wallreact::WallreactMod,
    pub lanes: crate::lanes::LanesMod,
    pub tactics: crate::tactics::TacticsMod,
    pub pressure: crate::pressure::PressureMod,
    pub digin: crate::digin::DiginMod,
    pub runner: crate::runner::RunnerMod,
    pub picture: crate::picture::PictureMod,
    pub flinch: crate::flinch::FlinchMod,
    pub suppression: crate::suppression::SuppressionMod,
    pub buddy: crate::buddy::BuddyMod,
    pub clearline: crate::clearline::ClearlineMod,
    pub gunwall: crate::gunwall::GunwallMod,
    pub enemysquads: crate::enemysquads::EnemysquadsMod,
    pub powerout: crate::powerout::PoweroutMod,
    pub darkfoe: crate::darkfoe::DarkfoeMod,
    pub morale: crate::morale::MoraleMod,
    pub irlight: crate::irlight::IrlightMod,
    pub nvg: crate::nvg::NvgMod,
    pub calmtorch: crate::calmtorch::CalmtorchMod,
    pub whistle: crate::whistle::WhistleMod,
    pub attention: crate::attention::AttentionMod,
    pub doze: crate::doze::DozeMod,
    pub lightswitch: crate::lightswitch::LightswitchMod,
    pub doortools: crate::doortools::DoortoolsMod,
    pub shotgun: crate::shotgun::ShotgunMod,
    pub kitanim: crate::kitanim::KitanimMod,
    pub he40: crate::he40::He40Mod,
    pub rescuecheck: crate::rescuecheck::RescuecheckMod,
    pub rescue: crate::rescue::RescueMod,
    pub stalk: crate::stalk::StalkMod,
    pub tapstep: crate::tapstep::TapstepMod,
    pub wet: crate::wet::WetMod,
    pub player: crate::player::PlayerMod,
    pub close: crate::close::CloseMod,
    pub takedown: crate::takedown::TakedownMod,
    pub spotted: crate::spotted::SpottedMod,
    pub pairorders: crate::pairorders::PairordersMod,
    pub irlaser: crate::irlaser::IrlaserMod,
    pub teamfeel: crate::teamfeel::TeamfeelMod,
    pub pairup: crate::pairup::PairupMod,
    pub sneak: crate::sneak::SneakMod,
    pub unseen: crate::unseen::UnseenMod,
    pub silent: crate::silent::SilentMod,
    pub cast: crate::cast::CastMod,
    pub char_look: crate::char_look::CharLookMod,
    pub stature: crate::stature::StatureMod,
    pub blast_feel: crate::blast_feel::BlastFeelMod,
    pub combat: crate::combat::CombatMod,
    pub scene_level_load: crate::scene_level_load::SceneLevelLoadMod,
    pub snow_bog: crate::snow_bog::SnowBogMod,
    pub sim_itemshome: crate::sim_itemshome::SimItemshomeMod,
    pub frag: crate::frag::FragMod,
    pub teampace: crate::teampace::TeampaceMod,
    pub squad: crate::squad::SquadMod,
    pub quiet: crate::quiet::QuietMod,
    pub workthings: crate::workthings::WorkthingsMod,
    pub lighting: crate::lighting::LightingMod,
    pub hours: crate::hours::HoursMod,
    pub chronicle: crate::chronicle::ChronicleMod,
    pub mission: crate::mission::MissionMod,
    pub squadsmoke: crate::squadsmoke::SquadsmokeMod,
    pub squadthrow: crate::squadthrow::SquadthrowMod,
    pub squad40: crate::squad40::Squad40Mod,
    pub kit: crate::kit::KitMod,
    pub wandorder: crate::wandorder::WandorderMod,
    pub fogofwar: crate::fogofwar::FogofwarMod,
    pub needs: crate::needs::NeedsMod,
    pub darkdoor: crate::darkdoor::DarkdoorMod,
    pub startle: crate::startle::StartleMod,
    pub doorforce: crate::doorforce::DoorforceMod,
    pub gunhold: crate::gunhold::GunholdMod,
    pub guards: crate::guards::GuardsMod,
    pub dialogue: crate::dialogue::DialogueMod,
    pub civilians: crate::civilians::CiviliansMod,
    pub deathrecap: crate::deathrecap::DeathrecapMod,
    pub seenwalk: crate::seenwalk::SeenwalkMod,
    pub fireline: crate::fireline::FirelineMod,
    pub planlay: crate::planlay::PlanlayMod,
    pub orders: crate::orders::OrdersMod,
    pub cmd: crate::cmd::CmdMod,
    pub seats: crate::seats::SeatsMod,
    pub wallcharge: crate::wallcharge::WallchargeMod,
    pub blast_look: crate::blast_look::BlastLookMod,
    pub particle_fx: crate::particle_fx::ParticleFxMod,
    pub snow_breath: crate::snow_breath::SnowBreathMod,
    pub snow_tracks: crate::snow_tracks::SnowTracksMod,
    pub air_sim: crate::air_sim::AirSimMod,
    pub snow_blast: crate::snow_blast::SnowBlastMod,
    pub snow_fx: crate::snow_fx::SnowFxMod,
    pub rain_fx: crate::rain_fx::RainFxMod,
    pub smokebreak: crate::smokebreak::SmokebreakMod,
    pub snow_marks: crate::snow_marks::SnowMarksMod,
    pub showwhy: crate::showwhy::ShowwhyMod,
    pub samlook: crate::samlook::SamlookMod,
    pub shutdoor: crate::shutdoor::ShutdoorMod,
    pub rollback: crate::rollback::RollbackMod,
    pub savestate: crate::savestate::SavestateMod,
    pub fixture_break: crate::fixture_break::FixtureBreakMod,
    pub render_reach: crate::render_reach::RenderReachMod,
    pub rustknobs: crate::rustknobs::RustknobsMod,
    pub weapons: crate::weapons::WeaponsMod,
    pub smoke_smoke_grid: crate::smoke_smoke_grid::SmokeSmokeGridMod,
    pub render_frame: crate::render_frame::RenderFrameMod,
}
impl Mods {
    pub fn blank() -> Self {
        Mods {
            gpu_wgsl_trim: crate::gpu_wgsl_trim::GpuWgslTrimMod::blank(),
            cloud_report: crate::cloud_report::CloudReportMod::blank(),
            ui_shaderrows: crate::ui_shaderrows::UiShaderrowsMod::blank(),
            gpu_keep: crate::gpu_keep::GpuKeepMod::blank(),
            gpu_device: crate::gpu_device::GpuDeviceMod::blank(),
            math_side: crate::math_side::MathSideMod::blank(),
            scene_cut: crate::scene_cut::SceneCutMod::blank(),
            scene_ground_field: crate::scene_ground_field::SceneGroundFieldMod::blank(),
            math_exact: crate::math_exact::MathExactMod::blank(),
            scene_markings: crate::scene_markings::SceneMarkingsMod::blank(),
            scene_round: crate::scene_round::SceneRoundMod::blank(),
            scene_materials: crate::scene_materials::SceneMaterialsMod::blank(),
            thinline: crate::thinline::ThinlineMod::blank(),
            scene_look: crate::scene_look::SceneLookMod::blank(),
            anim_gltf: crate::anim_gltf::AnimGltfMod::blank(),
            render_sdfrig: crate::render_sdfrig::RenderSdfrigMod::blank(),
            sim_sim: crate::sim_sim::SimSimMod::blank(),
            sim_state: crate::sim_state::SimStateMod::blank(),
            sim_propshome: crate::sim_propshome::SimPropshomeMod::blank(),
            scene_box_mirror: crate::scene_box_mirror::SceneBoxMirrorMod::blank(),
            scene_collision: crate::scene_collision::SceneCollisionMod::blank(),
            render_place_field: crate::render_place_field::RenderPlaceFieldMod::blank(),
            render_snow_field: crate::render_snow_field::RenderSnowFieldMod::blank(),
            render_char_materials: crate::render_char_materials::RenderCharMaterialsMod::blank(),
            render_mesh_shadows: crate::render_mesh_shadows::RenderMeshShadowsMod::blank(),
            anim_skeleton: crate::anim_skeleton::AnimSkeletonMod::blank(),
            sim_men_store: crate::sim_men_store::SimMenStoreMod::blank(),
            packside: crate::packside::PacksideMod::blank(),
            rig_props: crate::rig_props::RigPropsMod::blank(),
            math_dice: crate::math_dice::MathDiceMod::blank(),
            character: crate::character::CharacterMod::blank(),
            sim_doorshome: crate::sim_doorshome::SimDoorshomeMod::blank(),
            sim_slabshome: crate::sim_slabshome::SimSlabshomeMod::blank(),
            props: crate::props::PropsMod::blank(),
            teams: crate::teams::TeamsMod::blank(),
            host_keymap: crate::host_keymap::HostKeymapMod::blank(),
            opening: crate::opening::OpeningMod::blank(),
            weather_kinds: crate::weather_kinds::WeatherKindsMod::blank(),
            kitload: crate::kitload::KitloadMod::blank(),
            holdfire: crate::holdfire::HoldfireMod::blank(),
            destruction_switch: crate::destruction_switch::DestructionSwitchMod::blank(),
            dress: crate::dress::DressMod::blank(),
            sim_fallshome: crate::sim_fallshome::SimFallshomeMod::blank(),
            look_decals: crate::look_decals::LookDecalsMod::blank(),
            sim_lightshome: crate::sim_lightshome::SimLightshomeMod::blank(),
            wallbreak: crate::wallbreak::WallbreakMod::blank(),
            furnblast: crate::furnblast::FurnblastMod::blank(),
            doorblast: crate::doorblast::DoorblastMod::blank(),
            fire: crate::fire::FireMod::blank(),
            edge_look: crate::edge_look::EdgeLookMod::blank(),
            winbars: crate::winbars::WinbarsMod::blank(),
            destruction: crate::destruction::DestructionMod::blank(),
            glass: crate::glass::GlassMod::blank(),
            glasspen: crate::glasspen::GlasspenMod::blank(),
            netquietfeed: crate::netquietfeed::NetquietfeedMod::blank(),
            netquiet: crate::netquiet::NetquietMod::blank(),
            laststand: crate::laststand::LaststandMod::blank(),
            doorway: crate::doorway::DoorwayMod::blank(),
            difficulty: crate::difficulty::DifficultyMod::blank(),
            stickycam: crate::stickycam::StickycamMod::blank(),
            quietfog: crate::quietfog::QuietfogMod::blank(),
            nightbreach: crate::nightbreach::NightbreachMod::blank(),
            handleside: crate::handleside::HandlesideMod::blank(),
            doornames: crate::doornames::DoornamesMod::blank(),
            stackroom: crate::stackroom::StackroomMod::blank(),
            rearwatch: crate::rearwatch::RearwatchMod::blank(),
            wallpen: crate::wallpen::WallpenMod::blank(),
            coverfire: crate::coverfire::CoverfireMod::blank(),
            wallreact: crate::wallreact::WallreactMod::blank(),
            lanes: crate::lanes::LanesMod::blank(),
            tactics: crate::tactics::TacticsMod::blank(),
            pressure: crate::pressure::PressureMod::blank(),
            digin: crate::digin::DiginMod::blank(),
            runner: crate::runner::RunnerMod::blank(),
            picture: crate::picture::PictureMod::blank(),
            flinch: crate::flinch::FlinchMod::blank(),
            suppression: crate::suppression::SuppressionMod::blank(),
            buddy: crate::buddy::BuddyMod::blank(),
            clearline: crate::clearline::ClearlineMod::blank(),
            gunwall: crate::gunwall::GunwallMod::blank(),
            enemysquads: crate::enemysquads::EnemysquadsMod::blank(),
            powerout: crate::powerout::PoweroutMod::blank(),
            darkfoe: crate::darkfoe::DarkfoeMod::blank(),
            morale: crate::morale::MoraleMod::blank(),
            irlight: crate::irlight::IrlightMod::blank(),
            nvg: crate::nvg::NvgMod::blank(),
            calmtorch: crate::calmtorch::CalmtorchMod::blank(),
            whistle: crate::whistle::WhistleMod::blank(),
            attention: crate::attention::AttentionMod::blank(),
            doze: crate::doze::DozeMod::blank(),
            lightswitch: crate::lightswitch::LightswitchMod::blank(),
            doortools: crate::doortools::DoortoolsMod::blank(),
            shotgun: crate::shotgun::ShotgunMod::blank(),
            kitanim: crate::kitanim::KitanimMod::blank(),
            he40: crate::he40::He40Mod::blank(),
            rescuecheck: crate::rescuecheck::RescuecheckMod::blank(),
            rescue: crate::rescue::RescueMod::blank(),
            stalk: crate::stalk::StalkMod::blank(),
            tapstep: crate::tapstep::TapstepMod::blank(),
            wet: crate::wet::WetMod::blank(),
            player: crate::player::PlayerMod::blank(),
            close: crate::close::CloseMod::blank(),
            takedown: crate::takedown::TakedownMod::blank(),
            spotted: crate::spotted::SpottedMod::blank(),
            pairorders: crate::pairorders::PairordersMod::blank(),
            irlaser: crate::irlaser::IrlaserMod::blank(),
            teamfeel: crate::teamfeel::TeamfeelMod::blank(),
            pairup: crate::pairup::PairupMod::blank(),
            sneak: crate::sneak::SneakMod::blank(),
            unseen: crate::unseen::UnseenMod::blank(),
            silent: crate::silent::SilentMod::blank(),
            cast: crate::cast::CastMod::blank(),
            char_look: crate::char_look::CharLookMod::blank(),
            stature: crate::stature::StatureMod::blank(),
            blast_feel: crate::blast_feel::BlastFeelMod::blank(),
            combat: crate::combat::CombatMod::blank(),
            scene_level_load: crate::scene_level_load::SceneLevelLoadMod::blank(),
            snow_bog: crate::snow_bog::SnowBogMod::blank(),
            sim_itemshome: crate::sim_itemshome::SimItemshomeMod::blank(),
            frag: crate::frag::FragMod::blank(),
            teampace: crate::teampace::TeampaceMod::blank(),
            squad: crate::squad::SquadMod::blank(),
            quiet: crate::quiet::QuietMod::blank(),
            workthings: crate::workthings::WorkthingsMod::blank(),
            lighting: crate::lighting::LightingMod::blank(),
            hours: crate::hours::HoursMod::blank(),
            chronicle: crate::chronicle::ChronicleMod::blank(),
            mission: crate::mission::MissionMod::blank(),
            squadsmoke: crate::squadsmoke::SquadsmokeMod::blank(),
            squadthrow: crate::squadthrow::SquadthrowMod::blank(),
            squad40: crate::squad40::Squad40Mod::blank(),
            kit: crate::kit::KitMod::blank(),
            wandorder: crate::wandorder::WandorderMod::blank(),
            fogofwar: crate::fogofwar::FogofwarMod::blank(),
            needs: crate::needs::NeedsMod::blank(),
            darkdoor: crate::darkdoor::DarkdoorMod::blank(),
            startle: crate::startle::StartleMod::blank(),
            doorforce: crate::doorforce::DoorforceMod::blank(),
            gunhold: crate::gunhold::GunholdMod::blank(),
            guards: crate::guards::GuardsMod::blank(),
            dialogue: crate::dialogue::DialogueMod::blank(),
            civilians: crate::civilians::CiviliansMod::blank(),
            deathrecap: crate::deathrecap::DeathrecapMod::blank(),
            seenwalk: crate::seenwalk::SeenwalkMod::blank(),
            fireline: crate::fireline::FirelineMod::blank(),
            planlay: crate::planlay::PlanlayMod::blank(),
            orders: crate::orders::OrdersMod::blank(),
            cmd: crate::cmd::CmdMod::blank(),
            seats: crate::seats::SeatsMod::blank(),
            wallcharge: crate::wallcharge::WallchargeMod::blank(),
            blast_look: crate::blast_look::BlastLookMod::blank(),
            particle_fx: crate::particle_fx::ParticleFxMod::blank(),
            snow_breath: crate::snow_breath::SnowBreathMod::blank(),
            snow_tracks: crate::snow_tracks::SnowTracksMod::blank(),
            air_sim: crate::air_sim::AirSimMod::blank(),
            snow_blast: crate::snow_blast::SnowBlastMod::blank(),
            snow_fx: crate::snow_fx::SnowFxMod::blank(),
            rain_fx: crate::rain_fx::RainFxMod::blank(),
            smokebreak: crate::smokebreak::SmokebreakMod::blank(),
            snow_marks: crate::snow_marks::SnowMarksMod::blank(),
            showwhy: crate::showwhy::ShowwhyMod::blank(),
            samlook: crate::samlook::SamlookMod::blank(),
            shutdoor: crate::shutdoor::ShutdoorMod::blank(),
            rollback: crate::rollback::RollbackMod::blank(),
            savestate: crate::savestate::SavestateMod::blank(),
            fixture_break: crate::fixture_break::FixtureBreakMod::blank(),
            render_reach: crate::render_reach::RenderReachMod::blank(),
            rustknobs: crate::rustknobs::RustknobsMod::blank(),
            weapons: crate::weapons::WeaponsMod::blank(),
            smoke_smoke_grid: crate::smoke_smoke_grid::SmokeSmokeGridMod::blank(),
            render_frame: crate::render_frame::RenderFrameMod::blank(),
        }
    }
}

/// each module's counters: g.stats.<module>
#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub wand: crate::wand::WandStats,
    pub cmdring: crate::cmdring::CmdringStats,
    pub workstow: crate::workstow::WorkstowStats,
    pub holdfire: crate::holdfire::HoldfireStats,
    pub shards: crate::shards::ShardsStats,
    pub carlamps: crate::carlamps::CarlampsStats,
    pub wallbreak: crate::wallbreak::WallbreakStats,
    pub furnblast: crate::furnblast::FurnblastStats,
    pub doorblast: crate::doorblast::DoorblastStats,
    pub edge_look: crate::edge_look::EdgeLookStats,
    pub winbars: crate::winbars::WinbarsStats,
    pub destruction: crate::destruction::DestructionStats,
    pub floormess: crate::floormess::FloormessStats,
    pub break_table: crate::break_table::BreakTableStats,
    pub wind: crate::wind::WindStats,
    pub glassbreak: crate::glassbreak::GlassbreakStats,
    pub glasspen: crate::glasspen::GlasspenStats,
    pub netquiet: crate::netquiet::NetquietStats,
    pub doorway: crate::doorway::DoorwayStats,
    pub gunroles: crate::gunroles::GunrolesStats,
    pub stickycam: crate::stickycam::StickycamStats,
    pub handleside: crate::handleside::HandlesideStats,
    pub aimcost: crate::aimcost::AimcostStats,
    pub stackroom: crate::stackroom::StackroomStats,
    pub rearwatch: crate::rearwatch::RearwatchStats,
    pub roomfit: crate::roomfit::RoomfitStats,
    pub wallpen: crate::wallpen::WallpenStats,
    pub gunslots: crate::gunslots::GunslotsStats,
    pub coverfire: crate::coverfire::CoverfireStats,
    pub wallreact: crate::wallreact::WallreactStats,
    pub tactics: crate::tactics::TacticsStats,
    pub digin: crate::digin::DiginStats,
    pub runner: crate::runner::RunnerStats,
    pub picture: crate::picture::PictureStats,
    pub flinch: crate::flinch::FlinchStats,
    pub condition: crate::condition::ConditionStats,
    pub clearline: crate::clearline::ClearlineStats,
    pub gunwall: crate::gunwall::GunwallStats,
    pub friendlyfire: crate::friendlyfire::FriendlyfireStats,
    pub darkfoe: crate::darkfoe::DarkfoeStats,
    pub morale: crate::morale::MoraleStats,
    pub autopilot: crate::autopilot::AutopilotStats,
    pub calmtorch: crate::calmtorch::CalmtorchStats,
    pub whistle: crate::whistle::WhistleStats,
    pub attention: crate::attention::AttentionStats,
    pub doze: crate::doze::DozeStats,
    pub lightswitch: crate::lightswitch::LightswitchStats,
    pub samstun: crate::samstun::SamstunStats,
    pub shotgun: crate::shotgun::ShotgunStats,
    pub kitanim: crate::kitanim::KitanimStats,
    pub bigkit: crate::bigkit::BigkitStats,
    pub he40: crate::he40::He40Stats,
    pub friends: crate::friends::FriendsStats,
    pub noshove: crate::noshove::NoshoveStats,
    pub sprintdoor: crate::sprintdoor::SprintdoorStats,
    pub aimweight: crate::aimweight::AimweightStats,
    pub defuse: crate::defuse::DefuseStats,
    pub stalk: crate::stalk::StalkStats,
    pub tapstep: crate::tapstep::TapstepStats,
    pub wet: crate::wet::WetStats,
    pub close: crate::close::CloseStats,
    pub takedown: crate::takedown::TakedownStats,
    pub spotted: crate::spotted::SpottedStats,
    pub pairorders: crate::pairorders::PairordersStats,
    pub pairup: crate::pairup::PairupStats,
    pub sneak: crate::sneak::SneakStats,
    pub unseen: crate::unseen::UnseenStats,
    pub silent: crate::silent::SilentStats,
    pub lesson: crate::lesson::LessonStats,
    pub rails: crate::rails::RailsStats,
    pub wounds: crate::wounds::WoundsStats,
    pub frag: crate::frag::FragStats,
    pub teampace: crate::teampace::TeampaceStats,
    pub quiet: crate::quiet::QuietStats,
    pub workthings: crate::workthings::WorkthingsStats,
    pub lighting: crate::lighting::LightingStats,
    pub par: crate::par::ParStats,
    pub handwork: crate::handwork::HandworkStats,
    pub squadsmoke: crate::squadsmoke::SquadsmokeStats,
    pub throwplan: crate::throwplan::ThrowplanStats,
    pub squadthrow: crate::squadthrow::SquadthrowStats,
    pub squad40: crate::squad40::Squad40Stats,
    pub wandorder: crate::wandorder::WandorderStats,
    pub peeksight: crate::peeksight::PeeksightStats,
    pub needs: crate::needs::NeedsStats,
    pub darkdoor: crate::darkdoor::DarkdoorStats,
    pub startle: crate::startle::StartleStats,
    pub doorforce: crate::doorforce::DoorforceStats,
    pub civilians: crate::civilians::CiviliansStats,
    pub deathrecap: crate::deathrecap::DeathrecapStats,
    pub seenwalk: crate::seenwalk::SeenwalkStats,
    pub stackline: crate::stackline::StacklineStats,
    pub fireline: crate::fireline::FirelineStats,
    pub planlay: crate::planlay::PlanlayStats,
    pub planmode: crate::planmode::PlanmodeStats,
    pub doorthrow: crate::doorthrow::DoorthrowStats,
    pub shutdoor: crate::shutdoor::ShutdoorStats,
    pub cleanrestart: crate::cleanrestart::CleanrestartStats,
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
