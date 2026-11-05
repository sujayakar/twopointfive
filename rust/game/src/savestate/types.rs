//! The types of `savestate`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SaveState, src/game/savestate.ts:235)
#[derive(Clone, Debug)]
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
    pub par: Option<crate::par::ParSnap>,
    pub lay: Option<crate::planlay::LaySnap>,
    pub snatch: Option<crate::snatch::SnatchSnap>,
    pub defuse: Option<crate::defuse::DefuseSnap>,
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
impl SaveState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveState {
            v: 0.0,
            saved_at: 0.0,
            label: String::new(),
            kind: crate::savestate::SaveStateKind::Auto,
            level: crate::savestate::SaveStateLevel::zeroed(),
            mission_t: 0.0,
            stage: crate::game::MissionStage::Drive,
            run_id: String::new(),
            time: 0.0,
            event_seq: 0.0,
            fx_seed: 0.0,
            ocp_cooldown: 0.0,
            called_in: false,
            stood_down: false,
            breaker_fix_t: 0.0,
            last_sting_t: 0.0,
            escalation: 0.0,
            escalation_t: 0.0,
            alarm: crate::savestate::SaveStateAlarm::zeroed(),
            cleared_at: Vec::new(),
            clear_rooms: 0.0,
            blackout: crate::savestate::SaveStateBlackout::zeroed(),
            flags: crate::savestate::SaveStateFlags::zeroed(),
            fog: None,
            quiet: None,
            lesson: None,
            par: None,
            lay: None,
            snatch: None,
            defuse: None,
            player: crate::savestate::PlayerSnap::zeroed(),
            guards: Vec::new(),
            items: Vec::new(),
            doors: Vec::new(),
            props: Vec::new(),
            props_time: 0.0,
            targets: Vec::new(),
            devices: None,
            mission: crate::game::Mission::zeroed(),
            chronicle: crate::savestate::ChronSnap::zeroed(),
            events: Vec::new(),
            messages: Vec::new(),
            rollcall: crate::savestate::RollcallSnap::zeroed(),
            needs: crate::needs::NeedsSnap::zeroed(),
            whistle: crate::whistle::WhistleSnap::zeroed(),
            tactics: crate::tactics::TacticsSnap::zeroed(),
            dialogue: crate::dialogue::DialogueSnap::zeroed(),
            weather: None,
            qrf: crate::savestate::SaveStateQrf::zeroed(),
            civilians: None,
            walls: None,
            lanes: None,
            furn: None,
            bars: None,
            air: None,
            decals: Vec::new(),
            autosave: crate::savestate::AutosaveSnap::zeroed(),
            glass_pen: None,
            gun_wall: None,
            wall_hold: None,
            calm_torch: None,
            doze: None,
            morale: None,
            runner: None,
            startle: None,
            flinch: None,
            darkdoor: None,
            net_quiet: None,
            power_out: None,
            rescue_check: None,
            attention: None,
            picture: None,
            dice: None,
            hour: None,
            forced: Vec::new(),
        }
    }
}
impl Default for SaveState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SaveStateKind {
    Auto,
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

/// (TypeScript: inline at src/game/savestate.ts:237 SaveState.level, src/game/savestate.ts:237)
#[derive(Clone, Debug)]
pub struct SaveStateLevel {
    pub fp: String,
    pub boxes: f64,
    pub routes: f64,
    pub doors: f64,
}
impl SaveStateLevel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveStateLevel {
            fp: String::new(),
            boxes: 0.0,
            routes: 0.0,
            doors: 0.0,
        }
    }
}
impl Default for SaveStateLevel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:241 SaveState.alarm, src/game/savestate.ts:241)
#[derive(Clone, Copy, Debug)]
pub struct SaveStateAlarm {
    pub episode: bool,
    pub pos: [f64; 3],
    pub placed: bool,
    pub apart_t: f64,
    pub raised_at: f64,
}
impl SaveStateAlarm {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveStateAlarm {
            episode: false,
            pos: [0.0; 3],
            placed: false,
            apart_t: 0.0,
            raised_at: 0.0,
        }
    }
}
impl Default for SaveStateAlarm { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:242 SaveState.blackout, src/game/savestate.ts:242)
#[derive(Clone, Copy, Debug)]
pub struct SaveStateBlackout {
    pub active: bool,
    pub until: Option<f64>,
    pub since: f64,
    pub permanent: bool,
    pub restored_at: f64,
}
impl SaveStateBlackout {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveStateBlackout {
            active: false,
            until: None,
            since: 0.0,
            permanent: false,
            restored_at: 0.0,
        }
    }
}
impl Default for SaveStateBlackout { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:243 SaveState.flags, src/game/savestate.ts:243)
#[derive(Clone, Copy, Debug)]
pub struct SaveStateFlags {
    pub ai_enabled: bool,
    pub god_mode: bool,
    pub infinite_ammo: bool,
    pub player_invisible: bool,
}
impl SaveStateFlags {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveStateFlags {
            ai_enabled: false,
            god_mode: false,
            infinite_ammo: false,
            player_invisible: false,
        }
    }
}
impl Default for SaveStateFlags { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PlayerSnap, src/game/savestate.ts:174)
#[derive(Clone, Debug)]
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
impl PlayerSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerSnap {
            pos: [0.0; 3],
            body_yaw: 0.0,
            aim_yaw: 0.0,
            aim_pitch: 0.0,
            crouch: false,
            holstered: false,
            slot: crate::game::SlotKind::Pistol,
            throw_kind: crate::game::ThrowKind::Smoke,
            canisters: 0.0,
            flashbangs: 0.0,
            frags: None,
            wand: None,
            big_kit: None,
            kit_load: None,
            own: None,
            pistol: crate::savestate::PlayerSnapPistol::zeroed(),
            rifle: crate::savestate::PlayerSnapRifle::zeroed(),
            cams: None,
            cam_seq: None,
            ocp_charge: 0.0,
            has_rifle: false,
            rifle_gone: false,
            ocp_hinted: false,
            equip_t: 0.0,
            held_gait: None,
            close_on: None,
            rip_u: None,
            bash_at: None,
            bash_lean: None,
            stag_u: None,
            hurt_at: None,
            daz_u: None,
            daz_at: None,
            aw: None,
            keys: Vec::new(),
            codes: Vec::new(),
            hits_left: 0.0,
            nv: false,
            pace: 0.0,
            last_fire_t: 0.0,
            fire_cd: 0.0,
            noise: 0.0,
            visibility: 0.0,
            step_phase: 0.0,
            step_travel: None,
            anim_phase: 0.0,
            wet: [0.0; 3],
            bubble: None,
        }
    }
}
impl Default for PlayerSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: OwnCharges, src/game/savestate.ts:462)
#[derive(Clone, Debug)]
pub struct OwnCharges {
    pub charges: f64,
    pub walls: Option<f64>,
    pub doors: Option<Vec<[f64; 2]>>,
    pub spent: Option<f64>,
}
impl OwnCharges {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OwnCharges {
            charges: 0.0,
            walls: None,
            doors: None,
            spent: None,
        }
    }
}
impl Default for OwnCharges { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:184 PlayerSnap.pistol, src/game/savestate.ts:184 PlayerSnap.pistol)
#[derive(Clone, Debug)]
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
impl PlayerSnapPistol {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerSnapPistol {
            id: crate::weapons::WeaponId::Fiveseven,
            chamber: 0.0,
            mag: None,
            spare: Vec::new(),
            shots_fired: 0.0,
            mode: crate::weapons::FireMode::Semi,
            bursts: 0.0,
            last_burst_t: 0.0,
            light_on: false,
        }
    }
}
impl Default for PlayerSnapPistol { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:184 PlayerSnap.rifle, src/game/savestate.ts:184 PlayerSnap.rifle)
#[derive(Clone, Debug)]
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
impl PlayerSnapRifle {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerSnapRifle {
            id: crate::weapons::WeaponId::Fiveseven,
            chamber: 0.0,
            mag: None,
            spare: Vec::new(),
            shots_fired: 0.0,
            mode: crate::weapons::FireMode::Semi,
            bursts: 0.0,
            last_burst_t: 0.0,
            light_on: false,
            att_idx: 0.0,
            rings: 0.0,
            ring_cd: 0.0,
            gun: None,
            shells: None,
            cams: None,
        }
    }
}
impl Default for PlayerSnapRifle { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Bubble, src/game/savestate.ts:193)
#[derive(Clone, Debug)]
pub struct Bubble {
    pub text: String,
    pub t: f64,
    pub dur: f64,
    pub radio: bool,
}
impl Bubble {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Bubble {
            text: String::new(),
            t: 0.0,
            dur: 0.0,
            radio: false,
        }
    }
}
impl Default for Bubble { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GuardSnap, src/game/savestate.ts:197)
#[derive(Clone, Debug)]
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
    pub wound: Option<crate::savestate::GuardSnapWound>,
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
impl GuardSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardSnap {
            id: 0.0,
            route_i: 0.0,
            callsign: String::new(),
            state: crate::game::GuardState::Patrol,
            pos: [0.0; 3],
            vel: [0.0; 3],
            body_yaw: 0.0,
            aim_yaw: 0.0,
            aim_pitch: 0.0,
            crouch: 0.0,
            anim_phase: 0.0,
            turn: None,
            wp: 0.0,
            wait: 0.0,
            awareness: 0.0,
            last_known: None,
            last_seen_t: 0.0,
            found: false,
            down_kind: None,
            down_by: None,
            keys: Vec::new(),
            wound: None,
            missed_at: 0.0,
            missing_raised: false,
            muzzle_down: false,
            muzzle_clear_t: 0.0,
            muzzle_eval_t: 0.0,
            line_bark_t: 0.0,
            line_in_t: 0.0,
            leader_seen_t: 0.0,
            leader_seen_pos: [0.0; 3],
            leader_still_t: 0.0,
            body_visits: 0.0,
            body_visit_t: 0.0,
            body_remarked: false,
            body_closed: false,
            body_fan: 0.0,
            body_duty: None,
            dropped: 0.0,
            gun_lost_at: 0.0,
            torch_lost_at: 0.0,
            light_lit_at_loss: false,
            light_dead_until: 0.0,
            flashlight_on: false,
            sweep: 0.0,
            beam_dir: [0.0; 3],
            path: Vec::new(),
            path_i: 0.0,
            repath_t: 0.0,
            path_goal: None,
            weapon: crate::savestate::FirearmSnap::zeroed(),
            fire_cd: 0.0,
            react_t: 0.0,
            search_t: 0.0,
            look_phase: 0.0,
            speed: 0.0,
            step_phase: 0.0,
            heard_up_to: 0.0,
            died_at: 0.0,
            dazzled_until: 0.0,
            stagger_until: 0.0,
            hurt_at: None,
            door_down_u: None,
            last_pos: [0.0; 3],
            stuck_t: 0.0,
            stuck_n: 0.0,
            killer_id: 0.0,
            saw_body_t: 0.0,
            alert_best: 0.0,
            alert_stall_t: 0.0,
            alert_ref: None,
            chase_t: 0.0,
            shot_blocked_t: 0.0,
            sight_pos: [0.0; 3],
            sight_vel: [0.0; 3],
            sight_t: 0.0,
            led_t: 0.0,
            fix_t: 0.0,
            fix_ref: None,
            pie_last: 0.0,
            pie_last_t: 0.0,
            pie_prev: None,
            pie_prev_t: 0.0,
            stall_ref: None,
            stall_t: 0.0,
            task: None,
            drawn: false,
            leader: 0.0,
            follow_t: 0.0,
            follow_goal: [0.0; 3],
            post: None,
            witness: crate::game::Witness::zeroed(),
            talk: crate::savestate::TalkSnap::zeroed(),
            bubble: None,
            wet: [0.0; 3],
            rag: None,
        }
    }
}
impl Default for GuardSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:205 GuardSnap.wound, src/game/savestate.ts:205 GuardSnap.wound)
#[derive(Clone, Copy, Debug)]
pub struct GuardSnapWound {
    pub zone: crate::savestate::GuardSnapWoundZone,
    pub pose: crate::savestate::GuardSnapWoundPose,
    pub t: f64,
    pub clock: Option<f64>,
}
impl GuardSnapWound {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardSnapWound {
            zone: crate::savestate::GuardSnapWoundZone::Side,
            pose: crate::savestate::GuardSnapWoundPose::Knee,
            t: 0.0,
            clock: None,
        }
    }
}
impl Default for GuardSnapWound { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GuardSnapWoundZone {
    Side,
    Leg,
}
impl GuardSnapWoundZone {
    pub const fn as_str(self) -> &'static str {
        match self {
            GuardSnapWoundZone::Side => "side",
            GuardSnapWoundZone::Leg => "leg",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GuardSnapWoundPose {
    Knee,
    Back,
}
impl GuardSnapWoundPose {
    pub const fn as_str(self) -> &'static str {
        match self {
            GuardSnapWoundPose::Knee => "knee",
            GuardSnapWoundPose::Back => "back",
        }
    }
}

/// (TypeScript: inline at src/game/savestate.ts:210 GuardSnap.bodyDuty, src/game/savestate.ts:210)
#[derive(Clone, Copy, Debug)]
pub struct GuardSnapBodyDuty {
    pub body: f64,
    pub role: crate::savestate::GuardSnapBodyDutyRole,
    pub stage: crate::savestate::GuardSnapBodyDutyStage,
    pub t: f64,
    pub t0: f64,
    pub spot: Option<[f64; 3]>,
}
impl GuardSnapBodyDuty {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardSnapBodyDuty {
            body: 0.0,
            role: crate::savestate::GuardSnapBodyDutyRole::Finder,
            stage: crate::savestate::GuardSnapBodyDutyStage::Fan,
            t: 0.0,
            t0: 0.0,
            spot: None,
        }
    }
}
impl Default for GuardSnapBodyDuty { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GuardSnapBodyDutyRole {
    Finder,
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GuardSnapBodyDutyStage {
    Fan,
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

/// (TypeScript: FirearmSnap, src/game/savestate.ts:173)
#[derive(Clone, Debug)]
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
impl FirearmSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FirearmSnap {
            id: crate::weapons::WeaponId::Fiveseven,
            chamber: 0.0,
            mag: None,
            spare: Vec::new(),
            shots_fired: 0.0,
            mode: crate::weapons::FireMode::Semi,
            bursts: 0.0,
            last_burst_t: 0.0,
        }
    }
}
impl Default for FirearmSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum TaskSnap {
    Arm0(crate::savestate::TaskSnapArm0),
    CheckOn(crate::savestate::TaskSnapCheckOn),
}

/// (TypeScript: inline at src/game/savestate.ts:220 GuardSnap.task, src/game/savestate.ts:194)
#[derive(Clone, Copy, Debug)]
pub struct TaskSnapArm0 {
    pub kind: crate::savestate::TaskSnapArm0Kind,
    pub pos: [f64; 3],
}
impl TaskSnapArm0 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TaskSnapArm0 {
            kind: crate::savestate::TaskSnapArm0Kind::Breaker,
            pos: [0.0; 3],
        }
    }
}
impl Default for TaskSnapArm0 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TaskSnapArm0Kind {
    Breaker,
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

/// (TypeScript: inline at src/game/savestate.ts:220 GuardSnap.task, src/game/savestate.ts:194)
#[derive(Clone, Copy, Debug)]
pub struct TaskSnapCheckOn {
    pub pos: [f64; 3],
    pub who: f64,
    pub wp: f64,
    pub left: f64,
    pub t0: f64,
}
impl TaskSnapCheckOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TaskSnapCheckOn {
            pos: [0.0; 3],
            who: 0.0,
            wp: 0.0,
            left: 0.0,
            t0: 0.0,
        }
    }
}
impl Default for TaskSnapCheckOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: TalkSnap, src/game/savestate.ts:195)
#[derive(Clone, Debug)]
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
impl TalkSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TalkSnap {
            stage: 0.0,
            presses: 0.0,
            said: Vec::new(),
            said_n: Vec::new(),
            given: Vec::new(),
            last_t: 0.0,
            held_since: 0.0,
            abort_t: 0.0,
            script: None,
        }
    }
}
impl Default for TalkSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RagSnap, src/game/savestate.ts:196)
#[derive(Clone, Debug)]
pub struct RagSnap {
    pub x: Vec<f64>,
    pub rot: [f64; 4],
    pub sleeping: bool,
    pub own: Option<bool>,
}
impl RagSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RagSnap {
            x: Vec::new(),
            rot: [0.0; 4],
            sleeping: false,
            own: None,
        }
    }
}
impl Default for RagSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ItemSnap, src/game/savestate.ts:225)
#[derive(Clone, Copy, Debug)]
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
impl ItemSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ItemSnap {
            kind: crate::throwables::ItemKind::Pistol,
            pos: [0.0; 3],
            yaw: 0.0,
            tumble: 0.0,
            heading: 0.0,
            fill: 0.0,
            life: 0.0,
            age: 0.0,
            fired: false,
            emit_t: 0.0,
            who: 0.0,
            on: 0.0,
            on_pose: 0.0,
            radius: 0.0,
            restitution: 0.0,
            friction: 0.0,
        }
    }
}
impl Default for ItemSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DoorSnap, src/game/savestate.ts:226)
#[derive(Clone, Debug)]
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
impl DoorSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorSnap {
            name: String::new(),
            angle: 0.0,
            vel: 0.0,
            quiet_t: 0.0,
            latched: false,
            last_who: 0.0,
            since_bash: 0.0,
            closing: false,
            locked: false,
            lock_broken: false,
            noticed: false,
            noticed_t: 0.0,
            pick: 0.0,
            pick_idle: 0.0,
            pick_clock: 0.0,
            pick_n: 0.0,
            guard_t: 0.0,
            creak_armed: false,
            pull_t: None,
            press_t: None,
            solid_t: None,
            open_t: None,
            passers: None,
            swept: None,
            shot: None,
            off: None,
            drive: None,
        }
    }
}
impl Default for DoorSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PropSnap, src/game/savestate.ts:227)
#[derive(Clone, Copy, Debug)]
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
impl PropSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropSnap {
            x: 0.0,
            z: 0.0,
            yaw: 0.0,
            vx: 0.0,
            vz: 0.0,
            w: 0.0,
            asleep: false,
            idle: 0.0,
            last_who: 0.0,
            noise_t: 0.0,
            push_t: 0.0,
        }
    }
}
impl Default for PropSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: TargetSnap, src/game/savestate.ts:228)
#[derive(Clone, Debug)]
pub struct TargetSnap {
    pub name: String,
    pub on: bool,
    pub broken: bool,
    pub disabled_until: f64,
    pub hand: Option<f64>,
}
impl TargetSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TargetSnap {
            name: String::new(),
            on: false,
            broken: false,
            disabled_until: 0.0,
            hand: None,
        }
    }
}
impl Default for TargetSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ChronSnap, src/game/savestate.ts:232)
#[derive(Clone, Debug)]
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
impl ChronSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChronSnap {
            events: Vec::new(),
            dropped: 0.0,
            counts: crate::chronicle::ChronCounts::zeroed(),
            questioned: Vec::new(),
            path: Vec::new(),
            path_n: 0.0,
            path_every: 0.0,
            path_next: 0.0,
        }
    }
}
impl Default for ChronSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:262 SaveState.messages, src/game/savestate.ts:262)
#[derive(Clone, Debug)]
pub struct SaveStateMessagesItem {
    pub text: String,
    pub t: f64,
}
impl SaveStateMessagesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveStateMessagesItem {
            text: String::new(),
            t: 0.0,
        }
    }
}
impl Default for SaveStateMessagesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RollcallSnap, src/game/savestate.ts:233)
#[derive(Clone, Copy, Debug)]
pub struct RollcallSnap {
    pub next_at: f64,
    pub stage: crate::savestate::RollcallSnapStage,
    pub t0: f64,
    pub caller: f64,
    pub missing: f64,
    pub answered: bool,
    pub checks: f64,
}
impl RollcallSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RollcallSnap {
            next_at: 0.0,
            stage: crate::savestate::RollcallSnapStage::Idle,
            t0: 0.0,
            caller: 0.0,
            missing: 0.0,
            answered: false,
            checks: 0.0,
        }
    }
}
impl Default for RollcallSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RollcallSnapStage {
    Idle,
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

/// (TypeScript: inline at src/game/savestate.ts:263 SaveState.qrf, src/game/savestate.ts:263)
#[derive(Clone, Debug)]
pub struct SaveStateQrf {
    pub loud_t: f64,
    pub word: Option<Vec<crate::qrf::ResponseWordSnap>>,
}
impl SaveStateQrf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveStateQrf {
            loud_t: 0.0,
            word: None,
        }
    }
}
impl Default for SaveStateQrf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DecalSnap, src/game/savestate.ts:231)
#[derive(Clone, Copy, Debug)]
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
impl DecalSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DecalSnap {
            kind: crate::render_decals::DecalKind::Bullet,
            pos: [0.0; 3],
            n: [0.0; 3],
            t: [0.0; 3],
            radius: 0.0,
            param: 0.0,
            seed: 0.0,
            pane: None,
            door: None,
            prop: None,
            part: None,
            look: None,
        }
    }
}
impl Default for DecalSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AutosaveSnap, src/game/savestate.ts:234)
#[derive(Clone, Debug)]
pub struct AutosaveSnap {
    pub run_id: String,
    pub visited: Vec<String>,
    pub quiet_t: f64,
    pub last_save_t: f64,
    pub body_from: Option<[f64; 3]>,
}
impl AutosaveSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AutosaveSnap {
            run_id: String::new(),
            visited: Vec::new(),
            quiet_t: 0.0,
            last_save_t: 0.0,
            body_from: None,
        }
    }
}
impl Default for AutosaveSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:310 SaveState.dice, src/game/savestate.ts:310)
#[derive(Clone, Copy, Debug)]
pub struct SaveStateDice {
    pub s: f64,
    pub n: f64,
    pub seed: f64,
}
impl SaveStateDice {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveStateDice {
            s: 0.0,
            n: 0.0,
            seed: 0.0,
        }
    }
}
impl Default for SaveStateDice { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:313 SaveState.hour, src/game/savestate.ts:313)
#[derive(Clone, Copy, Debug)]
pub struct SaveStateHour {
    pub id: crate::hours::HourId,
    pub sun: f64,
    pub given: Option<bool>,
}
impl SaveStateHour {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveStateHour {
            id: crate::hours::HourId::Overcast,
            sun: 0.0,
            given: None,
        }
    }
}
impl Default for SaveStateHour { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Checkpoints, src/game/savestate.ts:900)
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
}
impl Checkpoints {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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
        }
    }
}
impl Default for Checkpoints { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/savestate.ts:909 Checkpoints.pending, src/game/savestate.ts:909)
#[derive(Clone, Debug)]
pub struct CheckpointsPending {
    pub why: String,
    pub label: String,
    pub at: f64,
}
impl CheckpointsPending {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckpointsPending {
            why: String::new(),
            label: String::new(),
            at: 0.0,
        }
    }
}
impl Default for CheckpointsPending { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:911 Checkpoints.flash, src/game/savestate.ts:911)
#[derive(Clone, Debug)]
pub struct CheckpointsFlash {
    pub text: String,
    pub t: f64,
}
impl CheckpointsFlash {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckpointsFlash {
            text: String::new(),
            t: 0.0,
        }
    }
}
impl Default for CheckpointsFlash { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:913 Checkpoints.log, src/game/savestate.ts:913)
#[derive(Clone, Debug)]
pub struct CheckpointsLogItem {
    pub t: f64,
    pub what: String,
}
impl CheckpointsLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckpointsLogItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for CheckpointsLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:914 Checkpoints.stats, src/game/savestate.ts:914)
#[derive(Clone, Copy, Debug)]
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
impl CheckpointsStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckpointsStats {
            requests: 0.0,
            autosaves: 0.0,
            manual: 0.0,
            harness: 0.0,
            loads: 0.0,
            dropped: 0.0,
            refused: 0.0,
            deferred_frames: 0.0,
            regretted: 0.0,
        }
    }
}
impl Default for CheckpointsStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet Checkpoints.missionRef, quiet Checkpoints.missionRef)
#[derive(Clone, Copy, Debug)]
pub struct CheckpointsMissionRef {
}
impl CheckpointsMissionRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckpointsMissionRef {
        }
    }
}
impl Default for CheckpointsMissionRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet Checkpoints.dragRef, quiet Checkpoints.dragRef)
#[derive(Clone, Copy, Debug)]
pub struct CheckpointsDragRef {
}
impl CheckpointsDragRef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckpointsDragRef {
        }
    }
}
impl Default for CheckpointsDragRef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:919 Checkpoints.lastAuto, src/game/savestate.ts:919)
#[derive(Clone, Debug)]
pub struct CheckpointsLastAuto {
    pub key: String,
    pub t: f64,
}
impl CheckpointsLastAuto {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckpointsLastAuto {
            key: String::new(),
            t: 0.0,
        }
    }
}
impl Default for CheckpointsLastAuto { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: MemShelf, src/game/savestate.ts:877)
#[derive(Clone, Debug)]
pub struct MemShelf {
    pub m: JsMap<String, crate::savestate::MemSave>,
    pub warmed: f64,
}
impl MemShelf {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        MemShelf {
            m: JsMap::new(),
            warmed: 0.0,
        }
    }
}
impl Default for MemShelf { fn default() -> Self { Self::blank() } }

/// (TypeScript: MemSave, src/game/savestate.ts:871)
#[derive(Clone, Debug)]
pub struct MemSave {
    pub info: crate::savestate::SlotInfo,
    pub pic: Option<String>,
    pub take: f64,
}
impl MemSave {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MemSave {
            info: crate::savestate::SlotInfo::zeroed(),
            pic: None,
            take: 0.0,
        }
    }
}
impl Default for MemSave { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SlotInfo, src/game/savestate.ts:869)
#[derive(Clone, Debug)]
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
impl SlotInfo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotInfo {
            key: String::new(),
            slot: crate::savestate::SlotInfoSlot::Auto,
            n: 0.0,
            label: String::new(),
            kind: crate::savestate::SlotInfoKind::Auto,
            saved_at: 0.0,
            mission_t: 0.0,
            stage: String::new(),
            run_id: String::new(),
            bytes: 0.0,
            regret: None,
            mem: None,
        }
    }
}
impl Default for SlotInfo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SlotInfoSlot {
    Auto,
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SlotInfoKind {
    Auto,
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

/// (TypeScript: SaveOpts, src/game/savestate.ts:506)
#[derive(Clone, Debug)]
pub struct SaveOpts {
    pub label: Option<String>,
    pub kind: Option<crate::savestate::SaveOptsKind>,
    pub force: Option<bool>,
    pub run_id: Option<String>,
}
impl SaveOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveOpts {
            label: None,
            kind: None,
            force: None,
            run_id: None,
        }
    }
}
impl Default for SaveOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: SaveStore, src/game/savestate.ts:860)
#[derive(Clone, Copy, Debug)]
pub struct SaveStore {
}
impl SaveStore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveStore {
        }
    }
}
impl Default for SaveStore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: TapeSlots, src/game/savestate.ts:873)
#[derive(Clone, Debug)]
pub struct TapeSlots {
    pub index: Option<String>,
    pub last_auto: Option<crate::journal::SessionRecorderSlotsLastAuto>,
}
impl TapeSlots {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TapeSlots {
            index: None,
            last_auto: None,
        }
    }
}
impl Default for TapeSlots { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SlotIndex, src/game/savestate.ts:890)
#[derive(Clone, Debug)]
pub struct SlotIndex {
    pub v: f64,
    pub next: f64,
    pub slots: JsMap<String, crate::savestate::SlotInfo>,
}
impl SlotIndex {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotIndex {
            v: 0.0,
            next: 0.0,
            slots: JsMap::new(),
        }
    }
}
impl Default for SlotIndex { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SaveResult, src/game/savestate.ts:891)
#[derive(Clone, Debug)]
pub struct SaveResult {
    pub ok: bool,
    pub why: Option<String>,
    pub info: Option<crate::savestate::SlotInfo>,
    pub bytes: Option<f64>,
    pub state: Option<crate::savestate::SaveState>,
}
impl SaveResult {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveResult {
            ok: false,
            why: None,
            info: None,
            bytes: None,
            state: None,
        }
    }
}
impl Default for SaveResult { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:405 fpCache, src/game/savestate.ts:405)
#[derive(Clone, Debug)]
pub struct SavestateModFpCacheValue {
    pub st: f64,
    pub fp: crate::savestate::SaveStateLevel,
}
impl SavestateModFpCacheValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SavestateModFpCacheValue {
            st: 0.0,
            fp: crate::savestate::SaveStateLevel::zeroed(),
        }
    }
}
impl Default for SavestateModFpCacheValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: inline at src/game/savestate.ts:464 ownCharges ->, src/game/savestate.ts:464)
#[derive(Clone, Debug)]
pub struct OwnCharges2 {
    pub own: Option<crate::savestate::OwnCharges>,
}
impl OwnCharges2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OwnCharges2 {
            own: None,
        }
    }
}
impl Default for OwnCharges2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:669 loadState(opts), src/game/savestate.ts:669)
#[derive(Clone, Debug)]
pub struct LoadStateOpts {
    pub label: Option<String>,
}
impl LoadStateOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LoadStateOpts {
            label: None,
        }
    }
}
impl Default for LoadStateOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:688 applyState(opts), src/game/savestate.ts:688)
#[derive(Clone, Debug)]
pub struct ApplyStateOpts {
    pub label: Option<String>,
}
impl ApplyStateOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ApplyStateOpts {
            label: None,
        }
    }
}
impl Default for ApplyStateOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:868 memoryStore ->, src/game/savestate.ts:868 memoryStore ->)
#[derive(Clone, Debug)]
pub struct MemoryStore {
    pub map: JsMap<String, String>,
}
impl MemoryStore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MemoryStore {
            map: JsMap::new(),
        }
    }
}
impl Default for MemoryStore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: words)
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

/// (TypeScript: a union)
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
    /// (src/game/savestate.ts:894 runSerial)
    pub run_serial: f64,
}
impl SavestateMod {
    pub fn blank() -> Self {
        SavestateMod {
            fp_cache: JsMap::new(),
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

/// (src/game/savestate.ts:862 localStore: a table nothing writes)
pub fn local_store() -> crate::savestate::SaveStore { crate::savestate::SaveStore::zeroed() }

/// (src/game/savestate.ts:1205 STAGE_RANK: a table nothing writes)
pub fn stage_rank() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("infiltrate"), 0.0); t.set(String::from("drive"), 1.0); t.set(String::from("exfil"), 2.0); t.set(String::from("done"), 3.0); t.set(String::from("failed"), 3.0); t } }
