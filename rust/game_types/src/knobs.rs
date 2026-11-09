//! The knob tables, at THE PAGE'S values: the numbers the page opened with (page-values), every yes/no row from port-list.json. Written by the skeleton's program.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AimK {
    pub guard: crate::knobs::AimKGuard,
    pub player: crate::knobs::AimKPlayer,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AimKGuard {
    pub settle_secs: f64,
    pub range_ref: f64,
    pub near_k: f64,
    pub acquire_min: f64,
    pub unsettled_k: f64,
    pub on_deg: f64,
    pub lose_rate: f64,
    pub mem_secs: f64,
    pub jump_deg: f64,
    pub target_move_k: f64,
    pub target_move_cap: f64,
    pub rate_tau: f64,
    pub shooter_move_k: f64,
    pub feet_tau: f64,
    pub steady_pace: f64,
    pub stop_to_shoot: f64,
    pub moving_track_cap: f64,
    pub shoot_band: f64,
    pub brake_lead: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AimKPlayer {
    pub settle_secs: f64,
    pub range_ref: f64,
    pub near_k: f64,
    pub unsettled_k: f64,
    pub base_stand: f64,
    pub base_crouch: f64,
    pub target_move_k: f64,
    pub target_move_cap: f64,
    pub rate_tau: f64,
    pub jump_deg: f64,
    pub shooter_move_k: f64,
    pub feet_tau: f64,
    pub steady_pace: f64,
    pub sprint_cap: f64,
    pub walk_cap: f64,
    pub moving_above: f64,
    pub crouch_k: f64,
    pub light_snap_px: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TrainedK {
    pub settle_k: f64,
    pub spread_k: f64,
    pub react_cap: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BodyLightK {
    pub lit: f64,
    pub lit_near: f64,
    pub near_m: f64,
    pub far_m: f64,
    pub smoke: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardEyeK {
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
    pub far_fix_at: f64,
    pub far_range: f64,
    pub far_lo: f64,
    pub far_hi: f64,
    pub far_k: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct NeedsK {
    pub rate_k: f64,
    pub max_away: f64,
    pub announce_lead: f64,
    pub say_lead: f64,
    pub stretch_lead: f64,
    pub partner_m: f64,
    pub reply_after: f64,
    pub dwell: JsMap<crate::needs::NeedKind, f64>,
    pub min_wait: f64,
    pub min_trip_m: f64,
    pub after_cancel: f64,
    pub after_alert: f64,
    pub rest_secs: f64,
    pub floor_gap_secs: f64,
    pub pace_k: f64,
    pub stuck_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WhistleK {
    pub level: f64,
    pub reach_k: f64,
    pub horizon: f64,
    pub cooldown: f64,
    pub twice_window: f64,
    pub awareness: f64,
    pub react_t: f64,
    pub walk_floor: f64,
    pub glance_secs: f64,
    pub glance_max: f64,
    pub gain: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WetK {
    pub soak_per_sec: f64,
    pub dry_per_sec: f64,
    pub bead_runoff_secs: f64,
    pub bead_density: f64,
    pub bead_size: f64,
    pub trail_len: f64,
    pub rough_wet: f64,
    pub rough_dry: f64,
    pub darken: f64,
    pub darken_mul: f64,
    pub spec_gain: f64,
    pub drip_secs: f64,
    pub prints: f64,
    pub line_band: f64,
    pub print_fade_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WindK {
    pub def: f64,
    pub vent_k: f64,
    pub storm_k: f64,
    pub door_k: f64,
    pub door_reach: f64,
    pub door_spread_deg: f64,
    pub door_open_rad: f64,
    pub gust_amp: f64,
    pub gust_period: f64,
    pub storm_gust_amp: f64,
    pub storm_gust_period: f64,
    pub ramp_up: f64,
    pub ramp_down: f64,
    pub smoke_relax: f64,
    pub sheet_wake: f64,
    pub sheet_kick: f64,
    pub debris_k: f64,
    pub near_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CivK {
    pub eye: crate::knobs::CivKEye,
    pub eye_busy: crate::knobs::CivKEyeBusy,
    pub eye_alarmed: crate::knobs::CivKEyeAlarmed,
    pub keen_k: f64,
    pub gun_k: f64,
    pub gun_m: f64,
    pub glimpse_at: f64,
    pub spot_at: f64,
    pub startle_secs: f64,
    pub startle_again_secs: f64,
    pub body_m: f64,
    pub freeze: [f64; 2],
    pub run_speed: f64,
    pub walk_speed: f64,
    pub creep_speed: f64,
    pub report_m: f64,
    pub report_secs: f64,
    pub phone_secs: f64,
    pub guard_bonus_m: f64,
    pub exit_malus_m: f64,
    pub past_threat_m: f64,
    pub corner_m: f64,
    pub reflee_m: f64,
    pub lone_secs: f64,
    pub settle_secs: f64,
    pub comply_min: f64,
    pub scream_m: f64,
    pub see_m: f64,
    pub see_lit: f64,
    pub found_m: f64,
    pub found_secs: f64,
    pub outside_call_secs: f64,
    pub greet_m: f64,
    pub greet_pair_secs: f64,
    pub greet_floor_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CivKEye {
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CivKEyeBusy {
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CivKEyeAlarmed {
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosK {
    pub def: f64,
    pub heat_tau: f64,
    pub cell: f64,
    pub t_decal: f64,
    pub t_paper: f64,
    pub t_burst: f64,
    pub t_puff: f64,
    pub t_knock: f64,
    pub t_tile: f64,
    pub bang_heat: f64,
    pub puffs_per_sec: f64,
    pub puffs_per_frame: f64,
    pub papers_per_sec: f64,
    pub knocks_per_sec: f64,
    pub tile_gap_secs: f64,
    pub tiles_per_encounter: f64,
    pub decals_per_sec: f64,
    pub paper_reach_m: f64,
    pub knock_reach_m: f64,
    pub sheets_per_stack: f64,
    pub puff: crate::knobs::ChaosKPuff,
    pub tile: crate::knobs::ChaosKTile,
    pub vel_low: f64,
    pub vel_high_k: f64,
    pub imp: crate::knobs::ChaosKImp,
    pub loud: crate::knobs::ChaosKLoud,
    pub pall: crate::knobs::ChaosKPall,
    pub mess: crate::knobs::ChaosKMess,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKPuff {
    pub radius: f64,
    pub density: f64,
    pub ttl: f64,
    pub speed: f64,
    pub temp: f64,
    pub push: f64,
    pub expand: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKTile {
    pub half: V3,
    pub albedo: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKImp {
    pub bang: crate::knobs::ChaosKImpBang,
    pub impact: crate::knobs::ChaosKImpImpact,
    pub thud: crate::knobs::ChaosKImpThud,
    pub chaos_k: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKImpBang {
    pub radius: f64,
    pub strength: f64,
    pub up: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKImpImpact {
    pub radius: f64,
    pub strength: f64,
    pub up: f64,
    pub bias_k: f64,
    pub debris_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKImpThud {
    pub radius: f64,
    pub strength: f64,
    pub up: f64,
    pub per_frame: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKLoud {
    pub fixture_hit_m: f64,
    pub fixture_blast_m: f64,
    pub stun_fixture_m: f64,
    pub fixtures_per_encounter: f64,
    pub fixture_gap_secs: f64,
    pub blast_dust_floor: f64,
    pub blast_dust_puffs: f64,
    pub heat_k: f64,
    pub burst_queue: f64,
    pub burst_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKPall {
    pub per_window: f64,
    pub window_secs: f64,
    pub slack: f64,
    pub clear: f64,
    pub clear_y: f64,
    pub min_radius: f64,
    pub max_gain: f64,
    pub wall: crate::knobs::ChaosKPallWall,
    pub burst: crate::knobs::ChaosKPallBurst,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKPallWall {
    pub near: f64,
    pub far: f64,
    pub radius: f64,
    pub density: f64,
    pub ttl: f64,
    pub speed: f64,
    pub temp: f64,
    pub push: f64,
    pub expand: f64,
    pub spread: f64,
    pub out: f64,
    pub dust: [f64; 2],
    pub dust_life: f64,
    pub dust_size: f64,
    pub dust_speed: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKPallBurst {
    pub ring: f64,
    pub core_gain: f64,
    pub radius: f64,
    pub density: f64,
    pub ttl: f64,
    pub speed: f64,
    pub temp: f64,
    pub push: f64,
    pub expand: f64,
    pub spread: f64,
    pub dust: f64,
    pub dust_life: f64,
    pub dust_size: f64,
    pub dust_speed: f64,
    pub smoke: [f64; 3],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKMess {
    pub stand_y: f64,
    pub clear_y: f64,
    pub under: f64,
    pub min_radius: f64,
    pub crowd: f64,
    pub roof_y: f64,
    pub burst: crate::knobs::ChaosKMessBurst,
    pub wall: crate::knobs::ChaosKMessWall,
    pub foot: crate::knobs::ChaosKMessFoot,
    pub top: crate::knobs::ChaosKMessTop,
    pub rubble: crate::knobs::ChaosKMessRubble,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKMessBurst {
    pub core: f64,
    pub lobes: f64,
    pub reach: f64,
    pub lobe: f64,
    pub strength: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKMessWall {
    pub out: f64,
    pub near: crate::knobs::ChaosKMessWallNear,
    pub far: crate::knobs::ChaosKMessWallFar,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKMessWallNear {
    pub core: f64,
    pub lobes: f64,
    pub reach: f64,
    pub lobe: f64,
    pub strength: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKMessWallFar {
    pub core: f64,
    pub lobes: f64,
    pub reach: f64,
    pub lobe: f64,
    pub strength: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKMessFoot {
    pub out: f64,
    pub radius: f64,
    pub per_metre: f64,
    pub strength: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKMessTop {
    pub lo: f64,
    pub hi: f64,
    pub strength: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChaosKMessRubble {
    pub least: f64,
    pub grow: f64,
    pub min: f64,
    pub max: f64,
    pub crowd: f64,
    pub strength: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RingK {
    pub speed: f64,
    pub gravity: f64,
    pub drag: f64,
    pub radius: f64,
    pub life: f64,
    pub substeps: f64,
    pub ko_below_neck: f64,
    pub stagger: f64,
    pub stagger_fire_cd: f64,
    pub stagger_aware: f64,
    pub stagger_back: f64,
    pub launch_noise: f64,
    pub clack_noise: f64,
    pub ko_noise: f64,
    pub hit_noise: f64,
    pub prop_j: f64,
    pub restitution: f64,
    pub keep_tangent: f64,
    pub spent_speed_max: f64,
    pub solve_iters: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LauncherK {
    pub rings: f64,
    pub cd: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct He40K {
    pub speed: f64,
    pub gravity: f64,
    pub drag: f64,
    pub radius: f64,
    pub life: f64,
    pub substeps: f64,
    pub solve_iters: f64,
    pub arm_m: f64,
    pub standoff: f64,
    pub launch_noise: f64,
    pub dud_noise: f64,
    pub dud_hit_noise: f64,
    pub prop_j: f64,
    pub restitution: f64,
    pub keep_tangent: f64,
    pub spent_speed_max: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Launcher40K {
    pub rounds: f64,
    pub cd: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FragK {
    pub issue: f64,
    pub fuse: f64,
    pub order_fuse: f64,
    pub squad: f64,
    pub order_key: String,
    pub order_in: f64,
    pub lethal_m: f64,
    pub wound_m: f64,
    pub destr_m: f64,
    pub wound_kill: f64,
    pub stagger: [f64; 2],
    pub lift: f64,
    pub glass_m: f64,
    pub glass_power: f64,
    pub pane_energy: f64,
    pub see_m: f64,
    pub hear_m: f64,
    pub react: [f64; 2],
    pub react_jit: f64,
    pub shout_m: f64,
    pub flee_m: f64,
    pub run_speed: f64,
    pub flee_keep: f64,
    pub flee_apart_m: f64,
    pub flee_lane_m: f64,
    pub down_shove: f64,
    pub flee_door_open: f64,
    pub flee_door_in: f64,
    pub flee_door_dip: f64,
    pub flee_door_mouth: f64,
    pub flee_door_leaf: f64,
    pub flee_door_via: f64,
    pub flee_door_via_m: f64,
    pub flee_door_new: f64,
    pub flee_door_most: f64,
    pub flee_door_try: f64,
    pub flee_door_nodes: f64,
    pub def_share: f64,
    pub def_max: f64,
    pub def_fuse: f64,
    pub def_wind: f64,
    pub def_near: f64,
    pub def_far: f64,
    pub def_past: f64,
    pub def_fresh: f64,
    pub def_blocked: f64,
    pub def_bunch: f64,
    pub def_bunch_secs: f64,
    pub def_gap: f64,
    pub def_own_m: f64,
    pub def_scatter: [f64; 2],
    pub cue_m: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShotgunK {
    pub spots: Vec<[f64; 2]>,
    pub clear: f64,
    pub hinge: Vec<[f64; 2]>,
    pub mark_reach: f64,
    pub line: f64,
    pub own_beat: f64,
    pub own_slack: f64,
    pub lock_in: f64,
    pub lock_h: f64,
    pub shells: f64,
    pub aim_cap: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GlassK {
    pub hear_m: f64,
    pub hear_level: f64,
    pub bang_m: f64,
    pub bang_cracks: f64,
    pub r_min: f64,
    pub r_max: f64,
    pub crack_gap: f64,
    pub live_gap: f64,
    pub ear_off: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct GlassbreakK {
    pub max_sites: f64,
    pub break_e: crate::knobs::GlassbreakKBreakE,
    pub punch_e: f64,
    pub run_single_e: f64,
    pub run_star: f64,
    pub run_ring_secs: f64,
    pub crack_gap: f64,
    pub rho: JsMap<crate::knobs::GlassbreakKRhoKey, [f64; 4]>,
    pub radials: JsMap<crate::knobs::GlassbreakKRadialsKey, [f64; 2]>,
    pub merge: f64,
    pub rz_min: f64,
    pub think: f64,
    pub stagger_per_m: f64,
    pub stagger_jit: f64,
    pub hang_area: f64,
    pub grip: crate::knobs::GlassbreakKGrip,
    pub stand_ny: f64,
    pub stand_foot: f64,
    pub balance_tol: f64,
    pub blast_m: f64,
    pub bang_power: f64,
    pub tooth_area: f64,
    pub tooth_k: f64,
    pub blast_run_k: f64,
    pub blast_dice_k: f64,
    pub blast_sheet_k: f64,
    pub rake_open: f64,
    pub rake_m: f64,
    pub dice_grain: f64,
    pub clump_m: f64,
    pub dice_hang: f64,
    pub dice_speed: f64,
    pub clumps_per_frame: f64,
    pub releases_per_frame: f64,
    pub out: f64,
    pub spin: f64,
    pub blast_out: f64,
    pub shard_albedo: V3,
    pub chip_color: V3,
    pub chips_per_m2: f64,
    pub chips_min: f64,
    pub dice_chips_per_m2: f64,
    pub crumb_every: f64,
    pub gust_speed_k: f64,
    pub walk_clear_h: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GlassbreakKBreakE {
    pub annealed: f64,
    pub tempered: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum GlassbreakKRhoKey {
    Annealed,
    Laminated,
}
impl GlassbreakKRhoKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassbreakKRhoKey::Annealed => "annealed",
            GlassbreakKRhoKey::Laminated => "laminated",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum GlassbreakKRadialsKey {
    Annealed,
    Laminated,
}
impl GlassbreakKRadialsKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassbreakKRadialsKey::Annealed => "annealed",
            GlassbreakKRadialsKey::Laminated => "laminated",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GlassbreakKGrip {
    pub side: f64,
    pub top: f64,
    pub wedged: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct GlasspenK {
    pub energy: JsMap<String, f64>,
    pub by_class: JsMap<String, f64>,
    pub default_energy: f64,
    pub cost: JsMap<crate::glasspen::PaneType, f64>,
    pub ref_thick: f64,
    pub oblique_max: f64,
    pub graze_cos: f64,
    pub min_energy: f64,
    pub max_panes: f64,
    pub deflect_deg: f64,
    pub bend_k: f64,
    pub hole_r: f64,
    pub hole_k: f64,
    pub holes: f64,
    pub shell_r: f64,
    pub stack_eyes: f64,
    pub stack_energy: f64,
    pub stack_least_k: f64,
    pub fracture_k: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShardsK {
    pub mid_per_m2: f64,
    pub mid_min: f64,
    pub mid_max: f64,
    pub mid_size: f64,
    pub mid_life: f64,
    pub fine_per_m2: f64,
    pub fine_min: f64,
    pub fine_max: f64,
    pub fine_size: f64,
    pub fine_life: f64,
    pub back_share: f64,
    pub hit_fine_min: f64,
    pub hit_back_share: f64,
    pub spall_out: f64,
    pub spall_back: f64,
    pub dice_per_m2: f64,
    pub dice_min: f64,
    pub dice_max: f64,
    pub dice_life: f64,
    pub dice_fine_share: f64,
    pub crater_per_m2: f64,
    pub crater_max: f64,
    pub exit_fines: f64,
    pub exit_speed: f64,
    pub speed: f64,
    pub live_cap: f64,
    pub cap_floor: f64,
    pub over_cap: f64,
    pub press_life: f64,
    pub press_from: f64,
    pub recs_per_frame: f64,
    pub defer_max: f64,
    pub plate_min_area: f64,
    pub plate_share: f64,
    pub child_min_area: f64,
    pub shatter_v0: f64,
    pub shatter_v1: f64,
    pub rest_crumbs: f64,
    pub crumb_size: f64,
    pub cut_edge: f64,
    pub cut_see: f64,
    pub cut_rim: f64,
    pub cut_body: f64,
    pub cut_see_rest: f64,
    pub cut_rest_body: f64,
    pub max_gen: f64,
    pub contact: crate::knobs::ShardsKContact,
    pub sounds_per_frame: f64,
    pub pending_max: f64,
    pub sound_gap: JsMap<String, f64>,
    pub sound_thicken: f64,
    pub sound_owed_secs: f64,
    pub level: f64,
    pub event_gap: f64,
    pub crunch_full: f64,
    pub crunch_gain: f64,
    pub crunch_add: f64,
    pub grind: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShardsKContact {
    pub keep_floor: f64,
    pub keep_wall: f64,
    pub rest: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct HoldK {
    pub regrab: f64,
    pub bare_grip: f64,
    pub bare_arch: f64,
    pub bare_elbow: f64,
    pub yank: crate::knobs::HoldKYank,
    pub lunge: crate::knobs::HoldKLunge,
    pub sway: crate::knobs::HoldKSway,
    pub apart: crate::knobs::HoldKApart,
    pub catch: crate::knobs::HoldKCatch,
    pub lay_ahead: f64,
    pub hit: crate::knobs::HoldKHit,
    pub first: crate::knobs::HoldKFirst,
    pub strip: crate::knobs::HoldKStrip,
    pub dist: f64,
    pub secs: crate::knobs::HoldKSecs,
    pub throat: V3,
    pub via_l: V3,
    pub over_l: V3,
    pub tight: V3,
    pub lean: f64,
    pub choke_lean: f64,
    pub head_pitch: f64,
    pub brace: f64,
    pub knife: crate::knobs::HoldKKnife,
    pub rest_l: V3,
    pub claw_l: V3,
    pub raised_r: V3,
    pub via_up: V3,
    pub claw_r: V3,
    pub hang: Vec<V3>,
    pub arch: f64,
    pub chin_up: f64,
    pub aside: f64,
    pub clav_up: f64,
    pub loll: f64,
    pub loll_roll: f64,
    pub lower: crate::knobs::HoldKLower,
    pub kill: crate::knobs::HoldKKill,
    pub beats: crate::knobs::HoldKBeats,
    pub haul: crate::knobs::HoldKHaul,
    pub sag: [f64; 2],
    pub breath: crate::knobs::HoldKBreath,
    pub struggle: crate::knobs::HoldKStruggle,
    pub gun: crate::knobs::HoldKGun,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKYank {
    pub arch: f64,
    pub sink: f64,
    pub in_: f64,
    pub hold: f64,
    pub out: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKLunge {
    pub dip: f64,
    pub back: f64,
    pub t: [f64; 3],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKSway {
    pub m: f64,
    pub secs: f64,
    pub twist: f64,
    pub his: f64,
    pub in_: f64,
    pub out: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKApart {
    pub side: f64,
    pub tip: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct HoldKCatch {
    pub via: V3,
    pub over: V3,
    pub at: [f64; 2],
    pub arm_in: f64,
    pub ready_out: [f64; 2],
    pub jolt: crate::knobs::HoldKCatchJolt,
    pub fling: crate::knobs::HoldKCatchFling,
    pub sink: f64,
    pub hips: f64,
    pub heels: [f64; 2],
    pub seat: V3,
    pub seat_lean: f64,
    pub brace: [f64; 2],
    pub brace_lift: f64,
    pub yank_sit: f64,
    pub feet: Vec<[f64; 2]>,
    pub lift: f64,
    pub sam_feet: Vec<[f64; 2]>,
    pub sam_lift: f64,
    pub settle: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKCatchJolt {
    pub t: [f64; 4],
    pub fwd: f64,
    pub head: f64,
    pub clav: f64,
    pub dip: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct HoldKCatchFling {
    pub t: [f64; 2],
    pub land: [f64; 2],
    pub to: Vec<V3>,
    pub drift: V3,
    pub arm_in: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKHit {
    pub t: [f64; 4],
    pub hips: f64,
    pub dip: f64,
    pub head: f64,
    pub heave: f64,
    pub sam_in: f64,
    pub sam_dip: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKFirst {
    pub take: f64,
    pub drop: f64,
    pub catch: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKStrip {
    pub take: f64,
    pub out: f64,
    pub drop: f64,
    pub follow: f64,
    pub knife: f64,
    pub home: f64,
    pub free: f64,
    pub up: f64,
    pub meet: V3,
    pub torn: V3,
    pub over: V3,
    pub wrench: V3,
    pub low: V3,
    pub yaw: f64,
    pub rap: V3,
    pub hold: [f64; 2],
    pub pitch: V3,
    pub rest: f64,
    pub push: [f64; 2],
    pub fall: f64,
    pub spin: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKSecs {
    pub grab: f64,
    pub choke_hold: f64,
    pub choke: f64,
    pub whip: f64,
    pub swap: f64,
    pub kill: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKKnife {
    pub hand: V3,
    pub blade: V3,
    pub back: V3,
    pub via: V3,
    pub off_hand: V3,
    pub off_blade: V3,
    pub off_back: V3,
    pub off_secs: f64,
    pub side: f64,
    pub head_yaw: f64,
    pub clav: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKLower {
    pub from: f64,
    pub by: f64,
    pub up: f64,
    pub sink: f64,
    pub fwd: f64,
    pub hip: f64,
    pub fold: f64,
    pub hang: [f64; 2],
    pub sam_sink: f64,
    pub sam_fwd: f64,
    pub sam_knees: f64,
    pub sam_lean: f64,
    pub sam_look: f64,
    pub sam_arm: [f64; 2],
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct HoldKKill {
    pub wind: f64,
    pub hit: f64,
    pub out: f64,
    pub go: f64,
    pub after: f64,
    pub mouth: V3,
    pub chest: V3,
    pub collar: V3,
    pub set: V3,
    pub across: V3,
    pub wide: V3,
    pub low: V3,
    pub wide_blade: V3,
    pub wide_back: V3,
    pub lean: [f64; 2],
    pub yaw: [f64; 2],
    pub sam_sink: f64,
    pub look: f64,
    pub reach: Vec<V3>,
    pub clutch: Vec<V3>,
    pub chin: f64,
    pub arch: f64,
    pub jolt: [f64; 2],
    pub sink: f64,
    pub fwd: f64,
    pub hip: f64,
    pub fold: f64,
    pub hang: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKBeats {
    pub clamp: [f64; 2],
    pub off: [f64; 2],
    pub tip_by: [f64; 2],
    pub tip: f64,
    pub back: f64,
    pub after: f64,
    pub lower: crate::knobs::HoldKBeatsLower,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKBeatsLower {
    pub from: f64,
    pub by: f64,
    pub up: f64,
    pub sink: f64,
    pub fwd: f64,
    pub hip: f64,
    pub fold: f64,
    pub hang: [f64; 2],
    pub sam_sink: f64,
    pub sam_fwd: f64,
    pub sam_knees: f64,
    pub sam_lean: f64,
    pub sam_look: f64,
    pub sam_arm: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKHaul {
    pub in_: [f64; 2],
    pub out: [f64; 2],
    pub down: f64,
    pub sam_lean: f64,
    pub sam_arm: [f64; 2],
    pub lift: f64,
    pub back: f64,
    pub recline: f64,
    pub arch: f64,
    pub chin: f64,
    pub feet: [f64; 2],
    pub toes: f64,
    pub kick: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKBreath {
    pub hz: f64,
    pub pitch: f64,
    pub clav: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKStruggle {
    pub rise: f64,
    pub fade_from: f64,
    pub fade_to: f64,
    pub yaw: f64,
    pub roll: f64,
    pub head: f64,
    pub claw: f64,
    pub dip: f64,
    pub hz: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct HoldKGun {
    pub hand: V3,
    pub bore: V3,
    pub out_hand: V3,
    pub kick_pitch: f64,
    pub kick_back: f64,
    pub via_a: V3,
    pub via_b: V3,
    pub via_bore: V3,
    pub wind_hand: V3,
    pub wind_bore: V3,
    pub hit_hand: V3,
    pub hit_bore: V3,
    pub thru_hand: V3,
    pub lean: f64,
    pub side: f64,
    pub head_yaw: f64,
    pub head_pitch: f64,
    pub wind_yaw: f64,
    pub hit_yaw: f64,
    pub clav: f64,
    pub hands_up: Vec<V3>,
    pub aside: f64,
    pub away_yaw: f64,
    pub chin_up: f64,
    pub duck: crate::knobs::HoldKGunDuck,
    pub knock: crate::knobs::HoldKGunKnock,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKGunDuck {
    pub yaw: f64,
    pub roll: f64,
    pub pitch: f64,
    pub hunch: f64,
    pub secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldKGunKnock {
    pub yaw: f64,
    pub roll: f64,
    pub pitch: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SuppressionK {
    pub near_r: f64,
    pub w_near: f64,
    pub hit_r: f64,
    pub w_hit: f64,
    pub min_travel: f64,
    pub half_life: f64,
    pub cap: f64,
    pub kick_decay: f64,
    pub down_accrual: f64,
    pub friendly_k: f64,
    pub keen: JsMap<String, f64>,
    pub flinch: f64,
    pub pinned: f64,
    pub release: f64,
    pub head_down: [f64; 2],
    pub flinch_delay: f64,
    pub head_down_delay: f64,
    pub guard_spread: f64,
    pub gap_k: f64,
    pub standoff_pace_k: f64,
    pub displace_after: f64,
    pub displace_min: f64,
    pub displace_max: f64,
    pub cover_hold: f64,
    pub displace_cooldown: f64,
    pub displace_cap_secs: f64,
    pub displace_pace: f64,
    pub displace_crouch: f64,
    pub src_memory_secs: f64,
    pub bark_secs: f64,
    pub floor_bark_secs: f64,
    pub pair_secs: f64,
    pub floor_line_secs: f64,
    pub sam_spread: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RifleSpreadK {
    pub still: f64,
    pub moving: f64,
    pub move_at: f64,
    pub crouch_k: f64,
    pub brace_k: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FootfallK {
    pub travel: f64,
    pub owe: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TwogearsK {
    pub crouch: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CrouchrunK {
    pub pace: f64,
    pub loud: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RunreadyK {
    pub fwd: f64,
    pub up: f64,
    pub pitch: f64,
    pub tuck: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RunarmsK {
    pub at: f64,
    pub keep: f64,
    pub mid: f64,
    pub swing: f64,
    pub pump: f64,
    pub lift: f64,
    pub pole: V3,
    pub pole_w: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SamlookK {
    pub near_m: f64,
    pub full_m: f64,
    pub back_deg: f64,
    pub keep_deg: f64,
    pub notice_m: f64,
    pub swap_m: f64,
    pub slip_from: f64,
    pub slip_by: f64,
    pub back_cos: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DashK {
    pub stride: f64,
    pub drop: f64,
    pub lean: f64,
    pub rate: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WatchK {
    pub yaw: f64,
    pub down: f64,
    pub up: f64,
    pub share: [f64; 3],
    pub rate: f64,
    pub w_rate: f64,
    pub lean: f64,
    pub drop: f64,
    pub away: f64,
    pub arm_out: f64,
    pub arm_down: f64,
    pub arm_w: f64,
    pub arm_side: [f64; 2],
    pub slip_rate: f64,
    pub squeeze: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StalkK {
    pub cursor_m: f64,
    pub begin_cos: f64,
    pub out_cos: f64,
    pub station_m: f64,
    pub grab_m: f64,
    pub pace: f64,
    pub close_min: f64,
    pub ease_up: f64,
    pub ease_down: f64,
    pub catch_secs: f64,
    pub catch_m: f64,
    pub catch_long: f64,
    pub catch_stride: f64,
    pub yank: f64,
    pub yank_room: f64,
    pub corner_m: f64,
    pub corner_deg: f64,
    pub jolt: f64,
    pub jolt_face: f64,
    pub jolt_secs: f64,
    pub corner_see: f64,
    pub corner_reach: [f64; 2],
    pub corner_coil: f64,
    pub corner_rise: f64,
    pub key_m: f64,
    pub low_from: f64,
    pub low_by: f64,
    pub low_k: f64,
    pub low_drop: f64,
    pub low_back: f64,
    pub low_lean: f64,
    pub low_hands: f64,
    pub low_spring: f64,
    pub low_wind: f64,
    pub wide_by: f64,
    pub heft_twist: f64,
    pub heft_swing: f64,
    pub heft_side: f64,
    pub heft_lean: f64,
    pub heft_dip: f64,
    pub heft_surge: f64,
    pub heft_out: f64,
    pub heft_down: f64,
    pub heft_fwd: f64,
    pub heft_lead: f64,
    pub heft_pole: V3,
    pub orbit_k: f64,
    pub orbit_edge: f64,
    pub orbit_snap: f64,
    pub low_begin_m: f64,
    pub low_out_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OverhearK {
    pub path_m: f64,
    pub carry: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ScoreK {
    pub seen: f64,
    pub seen_up: f64,
    pub alarm: f64,
    pub heightened: f64,
    pub lockdown: f64,
    pub body: f64,
    pub kill: f64,
    pub ko: f64,
    pub shot_near: f64,
    pub shot_near_cap: f64,
    pub light: f64,
    pub light_cap: f64,
    pub hit: f64,
    pub down: f64,
    pub load: f64,
    pub civ_reported: f64,
    pub civ_ko: f64,
    pub civ_killed: f64,
    pub device: f64,
    pub device_cap: f64,
    pub par_secs: f64,
    pub per_min: f64,
    pub time_cap: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BadgeK {
    pub lights_out_n: f64,
    pub speedrun_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CkptK {
    pub quiet_secs: f64,
    pub aware_max: f64,
    pub debounce_secs: f64,
    pub min_gap_secs: f64,
    pub pending_secs: f64,
    pub body_hide_m: f64,
    pub flash_secs: f64,
    pub regret_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FogK {
    pub dim: f64,
    pub desat: f64,
    pub fov_deg: f64,
    pub near_m: f64,
    pub cone: bool,
    pub cone_deg: f64,
    pub cone_near_m: f64,
    pub cone_soft_deg: f64,
    pub cone_near_soft_m: f64,
    pub own: bool,
    pub call_secs: f64,
    pub call_gap: f64,
    pub call_known: f64,
    pub smoke: f64,
    pub linger: f64,
    pub top_pad: f64,
    pub squad_doors_m: f64,
    pub squad_fire_secs: f64,
    pub flat: f64,
    pub lit: f64,
    pub air: bool,
    pub eyes: f64,
    pub kept: bool,
    pub full_fill: f64,
    pub edge: f64,
    pub lit_fill: f64,
    pub lit_ao: f64,
    pub lit_ao_m: f64,
    pub lit_cut: f64,
    pub last_seen: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MovesK {
    pub on: f64,
    pub recoil: f64,
    pub toss: f64,
    pub dazzle: f64,
    pub flinch: f64,
    pub gait: f64,
    pub fall: f64,
    pub squeeze: f64,
    pub reload: f64,
    pub bound: f64,
    pub clutch: f64,
    pub entry: f64,
    pub entry_eyes: f64,
    pub fold: f64,
    pub snap: f64,
    pub steady: f64,
    pub steady_run: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct MvK {
    pub recoil: crate::knobs::MvKRecoil,
    pub flinch: crate::knobs::MvKFlinch,
    pub hit: crate::knobs::MvKHit,
    pub dazzle: crate::knobs::MvKDazzle,
    pub gait: crate::knobs::MvKGait,
    pub snap: crate::knobs::MvKSnap,
    pub steady: crate::knobs::MvKSteady,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MvKRecoil {
    pub back: [f64; 2],
    pub back_tau: [f64; 2],
    pub climb: [f64; 2],
    pub climb_tau: [f64; 2],
    pub yaw: [f64; 2],
    pub rock: [f64; 2],
    pub shoulder: [f64; 2],
    pub rock_tau: [f64; 2],
    pub head: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MvKFlinch {
    pub amp: [f64; 2],
    pub dur: [f64; 2],
    pub pitch: f64,
    pub head: f64,
    pub away: f64,
    pub drop: f64,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MvKHit {
    pub dur: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
    pub shove: f64,
    pub drop: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MvKDazzle {
    pub amp: [f64; 2],
    pub hunch: f64,
    pub away: f64,
    pub roll: f64,
    pub head: f64,
    pub head_away: f64,
    pub drop: f64,
    pub back: f64,
    pub sway_x: f64,
    pub sway_z: f64,
    pub rate_in: f64,
    pub rate_out: f64,
    pub hand_drop: f64,
    pub hand_out: f64,
    pub hand_tilt: f64,
    pub hand_roll: f64,
    pub hand_pole: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MvKGait {
    pub drop: f64,
    pub aim_drop: f64,
    pub bob: f64,
    pub lean: f64,
    pub aim_lean: f64,
    pub elbow_in: f64,
    pub slouch_sway: f64,
    pub slouch_roll: f64,
    pub back: f64,
    pub chin: f64,
    pub wing: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MvKSnap {
    pub in_: f64,
    pub ease: f64,
    pub cover: [f64; 2],
    pub under: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct MvKSteady {
    pub roll: f64,
    pub sway: f64,
    pub feet: f64,
    pub knee: f64,
    pub knee_low: f64,
    pub deep: Vec<f64>,
    pub knee_out: f64,
    pub rise: f64,
    pub reach: f64,
    pub rise_rate: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TurnK {
    pub body_share: f64,
    pub green: f64,
    pub lead: f64,
    pub lead_rate: f64,
    pub jump: f64,
    pub aim: f64,
    pub aim_acc: f64,
    pub body: f64,
    pub body_acc: f64,
    pub on: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorpathK {
    pub at: f64,
    pub out_m: f64,
    pub in_m: f64,
    pub look: f64,
    pub leaf_clear: f64,
    pub tip_m: f64,
    pub gap_m: f64,
    pub short_m: f64,
    pub cap: f64,
    pub move_m: f64,
    pub apart_m: f64,
    pub body_m: f64,
    pub body_off: f64,
    pub body_move: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallsK {
    pub wall_max_thick: f64,
    pub wall_min_top: f64,
    pub wall_min_len: f64,
    pub col: f64,
    pub sill: f64,
    pub tough: JsMap<String, f64>,
    pub sill_k: f64,
    pub pistol_k: f64,
    pub cover_min_top: f64,
    pub cover_min_len: f64,
    pub cover_max_cells: f64,
    pub blast: crate::knobs::WallsKBlast,
    pub max_pieces: f64,
    pub max_changes: f64,
    pub chunks_per_cell: f64,
    pub fall_max_vol: f64,
    pub falls_per_change: f64,
    pub hung: crate::knobs::WallsKHung,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallsKBlast {
    pub radius: f64,
    pub hole_k: f64,
    pub port_k: f64,
    pub cover_k: f64,
    pub door_k: f64,
    pub min_hole: f64,
    pub contact_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallsKHung {
    pub deep: f64,
    pub long: f64,
    pub tall: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallchargeK {
    pub menu: crate::knobs::WallchargeKMenu,
    pub pick_m: f64,
    pub height: f64,
    pub end_m: f64,
    pub slide: Vec<f64>,
    pub set_off: f64,
    pub set_near: f64,
    pub sam_reach: f64,
    pub sam_shift: f64,
    pub sam_marker_m: f64,
    pub standoff: Vec<f64>,
    pub post_off: f64,
    pub file_gap: f64,
    pub back_m: Vec<f64>,
    pub clear_m: f64,
    pub mate_m: f64,
    pub after_blow: f64,
    pub enter_cap: f64,
    pub sweep_secs: f64,
    pub mouth_in: f64,
    pub lane: f64,
    pub lane_watch: f64,
    pub pods: Vec<[f64; 2]>,
    pub cost: f64,
    pub carried: Option<f64>,
    pub hold_secs: f64,
    pub mate_line: f64,
    pub off_frame: f64,
    pub off_frame_pod: f64,
    pub outer_thick: f64,
    pub outer_min_len: f64,
    pub far_slack: f64,
    pub far_reach: f64,
    pub hung_m: f64,
    pub safe_standoff: [f64; 2],
    pub safe_reach: [f64; 2],
    pub safe_secs: [f64; 2],
    pub safe_hide_m: f64,
    pub safe_walk_m: f64,
    pub safe_back_m: f64,
    pub fire_known_s: f64,
    pub fire_m: f64,
    pub fire_step: f64,
    pub fire_grain: f64,
    pub men_clear_in: f64,
    pub men_clear_s: f64,
    pub sam_secs: [f64; 2],
    pub sam_sup: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum WallchargeKMenu {
    #[default] Page,
    Direct,
}
impl WallchargeKMenu {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallchargeKMenu::Page => "page",
            WallchargeKMenu::Direct => "direct",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallshotK {
    pub pall: f64,
    pub stagger_secs: f64,
    pub stun_secs: [f64; 2],
    pub room_m: f64,
    pub ours_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallkillK {
    pub near_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HardbreathK {
    pub chest_amp: f64,
    pub chest_rate: f64,
    pub chest_up: f64,
    pub chest_down: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CastorK {
    pub decel: f64,
    pub visc: f64,
    pub spin: f64,
    pub start_v: f64,
    pub stop_v: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallreactK {
    pub see_m: f64,
    pub fov_deg: f64,
    pub hear_blast_m: f64,
    pub hear_fire_m: f64,
    pub hear_fall_m: f64,
    pub tell_m: f64,
    pub tell_secs: f64,
    pub near_m: f64,
    pub retry_secs: f64,
    pub gone_max: f64,
    pub step_m: f64,
    pub run_m: f64,
    pub beyond_m: f64,
    pub ev_open: f64,
    pub ev_seen: f64,
    pub ev_fire: f64,
    pub forget: f64,
    pub hole_base: f64,
    pub port_base: f64,
    pub crowd: f64,
    pub sticky: f64,
    pub fresh_secs: f64,
    pub fresh: f64,
    pub watch_m: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CoverfireK {
    pub stale_secs: f64,
    pub call_secs: f64,
    pub call_m: f64,
    pub behind_m: JsMap<String, f64>,
    pub max_layers: f64,
    pub max_rounds: f64,
    pub max_m: f64,
    pub min_m: f64,
    pub near_m: f64,
    pub slack_k: f64,
    pub slack: f64,
    pub give_secs: f64,
    pub lane: f64,
    pub cone_k: f64,
    pub past_m: f64,
    pub same_m: f64,
    pub linger_secs: f64,
    pub lead_secs: f64,
    pub lead_m: f64,
    pub low_y: f64,
    pub round_secs: f64,
    pub walk_pace: f64,
    pub walk_ask_secs: f64,
    pub line: String,
    pub say_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorblastK {
    pub tear_m: crate::knobs::DoorblastKTearM,
    pub charge_k: f64,
    pub charge_r: f64,
    pub speed: [f64; 2],
    pub up: f64,
    pub tip: f64,
    pub spin: f64,
    pub scrape: f64,
    pub slam: f64,
    pub slide: f64,
    pub bounce: f64,
    pub room: f64,
    pub land_level: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorblastKTearM {
    pub stun: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorshotK {
    pub toll: f64,
    pub strips: f64,
    pub lengths: f64,
    pub uneven: f64,
    pub chunks: f64,
    pub fly: Vec<f64>,
    pub side: f64,
    pub up: Vec<f64>,
    pub spin: f64,
    pub tilt: Vec<f64>,
    pub topple: Vec<f64>,
    pub chips: Vec<f64>,
    pub chip_v: Vec<f64>,
    pub shed: f64,
    pub slivers: Vec<f64>,
    pub level: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorthrowK {
    pub push_secs: f64,
    pub kick_secs: f64,
    pub startle: f64,
    pub wary: f64,
    pub min_speed: f64,
    pub hand_rate: f64,
    pub floor_rad: f64,
    pub rate: f64,
    pub slide: f64,
    pub again: f64,
    pub aware: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WinbarsK {
    pub per_round: f64,
    pub pieces: f64,
    pub uneven: f64,
    pub fly: [f64; 2],
    pub rest_k: f64,
    pub up: [f64; 2],
    pub side: f64,
    pub spin: f64,
    pub chips: f64,
    pub chip_v: f64,
    pub blast_k: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OpeningK {
    pub clear_m: f64,
    pub step_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HandleK {
    pub look: f64,
    pub ajar: f64,
    pub reach: f64,
    pub handle_in: f64,
    pub far_reach: f64,
    pub push_in: f64,
    pub push_off: f64,
    pub push_thru: f64,
    pub stand_in: f64,
    pub stand_off: f64,
    pub thru_at: f64,
    pub thru_off: f64,
    pub body: f64,
    pub slack: f64,
    pub near: f64,
    pub turn: f64,
    pub clear: f64,
    pub wait: f64,
    pub press: f64,
    pub tool_post: f64,
    pub near_body: f64,
    pub near_post: f64,
    pub go_cap: f64,
    pub rest: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoormannersK {
    pub reach: f64,
    pub half: f64,
    pub throat: f64,
    pub gap: f64,
    pub gap_min: f64,
    pub brake: f64,
    pub patience: f64,
    pub rest: f64,
    pub beyond: f64,
    pub aside: f64,
    pub jamb_out: f64,
    pub jamb_past: f64,
    pub file_gap: f64,
    pub apart: f64,
    pub step: f64,
    pub round: f64,
    pub round_out: f64,
    pub own_m: f64,
    pub own_end_m: f64,
    pub leaf_stall_m: f64,
    pub leaf_stall_secs: f64,
    pub leaf_stall_reach: f64,
    pub leaf_stall_ahead: f64,
    pub leaf_stall_push: f64,
    pub sweep: f64,
    pub lane_ajar: f64,
    pub lane_m: f64,
    pub lane_out: f64,
    pub lane_flare: f64,
    pub lane_clear: f64,
    pub lane_apart: f64,
    pub cover_m: f64,
    pub cover_deep: f64,
    pub cover_look: f64,
    pub cover_quiet: f64,
    pub cover_cap: f64,
    pub lane_dear: f64,
    pub lane_strip: f64,
    pub lane_long: f64,
    pub funnel: f64,
    pub funnel_rest: f64,
    pub through: f64,
    pub past: f64,
    pub plane_deg: f64,
    pub rest_secs: f64,
    pub clear_m: f64,
    pub held_cap: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreaksK {
    pub share: crate::knobs::BreaksKShare,
    pub refuse_chips: [f64; 2],
    pub sized_chips: [f64; 2],
    pub sized_span: [f64; 2],
    pub china_land: f64,
    pub china_max: f64,
    pub core_tint: f64,
    pub chip_tint: f64,
    pub heap: crate::knobs::BreaksKHeap,
    pub max_standing_planes: f64,
    pub across_deg: [f64; 2],
    pub loose_min: f64,
    pub kink: crate::knobs::BreaksKKink,
    pub steel: crate::knobs::BreaksKSteel,
    pub stud: crate::knobs::BreaksKStud,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreaksKShare {
    pub big: crate::knobs::BreaksKShareBig,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreaksKShareBig {
    pub thrown: f64,
    pub heap: f64,
    pub furniture: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreaksKHeap {
    pub span: [f64; 2],
    pub out: [f64; 2],
    pub tip: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreaksKKink {
    pub deg: [f64; 2],
    pub skew: f64,
    pub off: f64,
    pub min_long: f64,
    pub gap_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreaksKSteel {
    pub sheet: f64,
    pub kink: [f64; 2],
    pub dents: [f64; 2],
    pub dent: [f64; 2],
    pub dent_most: f64,
    pub rim: f64,
    pub lip: f64,
    pub inside: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreaksKStud {
    pub half: f64,
    pub depth_k: f64,
    pub foot: f64,
    pub albedo: V3,
    pub rough: f64,
    pub does: crate::scene_materials::DoesWord,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FurnK {
    pub k: JsMap<crate::furnblast::BlastSource, f64>,
    pub fling: JsMap<crate::furnblast::BlastSource, f64>,
    pub shatter_m: f64,
    pub apart_m: f64,
    pub throw_m: f64,
    pub throw_v: f64,
    pub prop_break_m: f64,
    pub chunks_per_blast: f64,
    pub mouth_m: f64,
    pub merge_m: f64,
    pub mouth: crate::knobs::FurnKMouth,
    pub hung: crate::knobs::FurnKHung,
    pub root: crate::knobs::FurnKRoot,
    pub rider: crate::knobs::FurnKRider,
    pub touch: f64,
    pub row_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FurnKMouth {
    pub depth: f64,
    pub top: f64,
    pub vol: f64,
    pub chunks: f64,
    pub base_thick: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FurnKHung {
    pub len: f64,
    pub max: f64,
    pub strips: f64,
    pub under_m: f64,
    pub through: Vec<f64>,
    pub back: Vec<f64>,
    pub side: f64,
    pub up: Vec<f64>,
    pub spin: f64,
    pub chunks: f64,
    pub chips: Vec<f64>,
    pub chip_v: Vec<f64>,
    pub hole_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FurnKRoot {
    pub min_top: f64,
    pub max_top: f64,
    pub max_long: f64,
    pub max_vol: f64,
    pub apart_max_vol: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FurnKRider {
    pub max_top: f64,
    pub max_vol: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SnowbogK {
    pub dead: f64,
    pub grip: f64,
    pub stick: f64,
    pub packed: f64,
    pub old: f64,
    pub top: f64,
    pub bank_m: f64,
    pub run: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RubbleK {
    pub radius: f64,
    pub reach: f64,
    pub lead: f64,
    pub inside: f64,
    pub floor: f64,
    pub heavy: f64,
    pub walk: f64,
    pub base: f64,
    pub gain: f64,
    pub aside: f64,
    pub v_max: f64,
    pub off: f64,
    pub slide: f64,
    pub spin: f64,
    pub per_frame: f64,
    pub cooldown: f64,
    pub give_up: f64,
    pub give_up_secs: f64,
    pub scuff: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DebrisK {
    pub min_speed: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LightwallK {
    pub gap: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AirdustK {
    pub diffuse: f64,
    pub settle: f64,
    pub step: f64,
    pub recheck_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallsnapK {
    pub on: bool,
    pub take_base: f64,
    pub take_per_m: f64,
    pub take_per_turn: f64,
    pub take_min: f64,
    pub take_max: f64,
    pub ease_pow: f64,
    pub yaw_pow: f64,
    pub reach_moving: f64,
    pub heading_min: f64,
    pub heading_bonus: f64,
    pub buffer: f64,
    pub dip: f64,
    pub nod: f64,
    pub hit_peak: f64,
    pub step_lift: f64,
    pub step_trail: f64,
    pub cam_room: f64,
    pub cam_side: f64,
    pub along_deg: f64,
    pub away_secs: f64,
    pub away_diag: f64,
    pub off_push: f64,
    pub up_secs: f64,
    pub up_rise: f64,
    pub bore: f64,
    pub lean_along: f64,
    pub lean_out: f64,
    pub lean_back: f64,
    pub lean_on_deg: f64,
    pub lean_off_deg: f64,
    pub lean_min_dist: f64,
    pub lean_max_deg: f64,
    pub lean_keep_deg: f64,
    pub lean_keep_dist: f64,
    pub lean_over_y: f64,
    pub lean_clear_m: f64,
    pub lean_gun: f64,
    pub lean_gun_step: f64,
    pub dwell_out: f64,
    pub dwell_back: f64,
    pub min_out: f64,
    pub after_shot: f64,
    pub press: bool,
    pub press_secs: f64,
    pub press_flow: f64,
    pub press_near: f64,
    pub press_deg: f64,
    pub press_touch: f64,
    pub press_thing: f64,
    pub press_thing_at: f64,
    pub press_turn: f64,
    pub press_again: f64,
    pub press_after_shot: f64,
    pub voice_on: f64,
    pub voice_off: f64,
    pub slow_take: f64,
    pub slow_in: f64,
    pub slow_out: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GunwallK {
    pub down_m: f64,
    pub up_m: f64,
    pub lead_s: f64,
    pub at_m: f64,
    pub tuck_m: f64,
    pub tuck_lead_s: f64,
    pub fight_s: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CloseK {
    pub rmb_tap_secs: f64,
    pub rmb_loud: f64,
    pub rmb_wait_secs: f64,
    pub hands_low_neck_m: f64,
    pub mode_m: f64,
    pub mode_out_m: f64,
    pub dark_at: f64,
    pub lit_at: f64,
    pub touch_m: f64,
    pub reach: crate::knobs::CloseKReach,
    pub side_from_deg: f64,
    pub strike_dist: f64,
    pub seat_dist: f64,
    pub noise: crate::knobs::CloseKNoise,
    pub alert_noise: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CloseKReach {
    pub behind: f64,
    pub side: f64,
    pub front: f64,
    pub seat: f64,
    pub seat_front: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CloseKNoise {
    pub behind: f64,
    pub side: f64,
    pub front: f64,
    pub seat: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlanK {
    pub fresh_secs: f64,
    pub walk_dim: f64,
    pub rest_dim: f64,
    pub ring_m: f64,
    pub sector_m: f64,
    pub sector_deg: f64,
    pub sight_m: f64,
    pub reask_secs: f64,
    pub pair_bar_m: f64,
    pub others: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ReloadK {
    pub delay: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SamstunK {
    pub fade: f64,
    pub own_seen: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RecapK {
    pub secs: f64,
    pub after: f64,
    pub hz: f64,
    pub keep: f64,
    pub live_see_m: f64,
    pub live_saw_secs: f64,
    pub live_hear_m: f64,
    pub live_beat: f64,
    pub live_side_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AimweightK {
    pub rate: f64,
    pub smooth: f64,
    pub heft: f64,
    pub bare: f64,
    pub on_m: f64,
    pub reach: f64,
    pub launch_wait: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AimcostK {
    pub bare: f64,
    pub pad: f64,
    pub slack: f64,
    pub move_k: f64,
    pub stop_tau: f64,
    pub stop_bleed: f64,
    pub brace_feet: f64,
    pub bare_device: f64,
    pub plant_at: f64,
    pub flank_at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FriendsK {
    pub reach_m: f64,
    pub past_m: f64,
    pub clear_m: f64,
    pub lead_s: f64,
    pub lead_max_m: f64,
    pub up_secs: f64,
    pub quiet_s: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FriendlyfireK {
    pub cone_k: f64,
    pub body_m: f64,
    pub sure_m: f64,
    pub near_m: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LightswitchK {
    pub reach: f64,
    pub height: f64,
    pub beside: Vec<f64>,
    pub stand_m: f64,
    pub per_room: f64,
    pub apart_m: f64,
    pub scan_m: f64,
    pub guard_secs: f64,
    pub hand_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CarlampsK {
    pub dark: V3,
    pub fit: f64,
    pub tone: f64,
    pub twin: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallskinK {
    pub on: bool,
    pub thick: f64,
    pub off: f64,
    pub per_skin: f64,
    pub max_pieces: f64,
    pub box_slack: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FirelineK {
    pub fire_s: f64,
    pub reload_s: f64,
    pub pace_s: f64,
    pub pace_k: f64,
    pub lapse_s: f64,
    pub spread: f64,
    pub clear_m: f64,
    pub short_m: f64,
    pub keep_m: f64,
    pub gap_m: f64,
    pub look_m: f64,
    pub cue_s: f64,
    pub max_s: f64,
    pub again_s: f64,
    pub way_more_m: f64,
    pub berth_m: f64,
    pub edge_m: f64,
    pub reach_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SavetimelineK {
    pub manual: f64,
    pub auto: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HandringK {
    pub on: bool,
    pub hold_secs: f64,
    pub turn_max: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SprintdoorK {
    pub keep: f64,
    pub bash_vel: f64,
    pub lean: f64,
    pub lean_secs: f64,
    pub rip_pace: f64,
    pub rip_secs: f64,
    pub rip_vel: f64,
    pub rip_level: f64,
    pub kick_k: f64,
    pub kick_into: f64,
    pub kick_free: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TeampaceK {
    pub k: f64,
    pub follow_k: f64,
    pub behind_m: f64,
    pub fight_secs: f64,
    pub back_secs: f64,
    pub dash_m: f64,
    pub dash_seen: f64,
    pub dash_every: f64,
    pub dash_hot: f64,
    pub door_brisk: bool,
    pub pacing: bool,
    pub pace_near: f64,
    pub pace_gap: f64,
    pub pace_cos: f64,
    pub walk_aim_off: f64,
    pub walk_aim_on: f64,
    pub run_at: f64,
    pub accel: f64,
    pub step: f64,
    pub gap: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct NoshoveK {
    pub ring: f64,
    pub back: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RailsK {
    pub pan_speed: f64,
    pub pan_reach: f64,
    pub snap_px: f64,
    pub keep: f64,
    pub keep_speed: f64,
    pub keep_deep: f64,
    pub keep_up: f64,
    pub way_m: f64,
    pub after_secs: f64,
    pub after_max: f64,
    pub line_clear: bool,
    pub mark_clear: bool,
    pub card_beat_secs: f64,
    pub card_quiet: f64,
    pub card_max: f64,
    pub spare_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorforceK {
    pub sure: f64,
    pub keep: f64,
    pub fade_secs: f64,
    pub stale_secs: f64,
    pub gain: f64,
    pub hear_min: f64,
    pub shot: f64,
    pub handle: f64,
    pub seen: f64,
    pub shut_carry: f64,
    pub behind_m: f64,
    pub reach_m: f64,
    pub seen_secs: f64,
    pub heard_err_m: f64,
    pub chest_y: f64,
    pub near_leaf_m: f64,
    pub past_leaf_m: f64,
    pub rounds: f64,
    pub gap_secs: f64,
    pub kick_secs: f64,
    pub kick_secs_no_shot: f64,
    pub walk_secs: f64,
    pub rest_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SlotsetupK {
    pub settle: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GlassrealK {
    pub hang: f64,
    pub blocks: f64,
    pub low_m: f64,
    pub door_m: f64,
    pub door_foot_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct QuietfogK {
    pub def: crate::quietfog::QuietRung,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ToldK {
    pub rung: crate::told::ToldRung,
    pub guided: f64,
    pub standard: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitanimK {
    pub sling: f64,
    pub sling_up: f64,
    pub sling_down: f64,
    pub early: f64,
    pub click_keep: f64,
    pub throw_keep: f64,
    pub plain_lock: f64,
    pub beats: crate::knobs::KitanimKBeats,
    pub close_near: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitanimKBeats {
    pub pistol: crate::knobs::KitanimKBeatsPistol,
    pub rifle: crate::knobs::KitanimKBeatsRifle,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitanimKBeatsPistol {
    pub ref_: f64,
    pub empty: crate::kitanim::BeatRow,
    pub dry: crate::kitanim::BeatRow,
    pub tactical: crate::kitanim::BeatRow,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitanimKBeatsRifle {
    pub ref_: f64,
    pub empty: crate::kitanim::BeatRow,
    pub dry: crate::kitanim::BeatRow,
    pub tactical: crate::kitanim::BeatRow,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KitK {
    pub stow: JsMap<crate::character::KitItem, f64>,
    pub reach: JsMap<crate::character::KitItem, f64>,
    pub draw: JsMap<crate::character::KitItem, f64>,
    pub away: f64,
    pub swap_k: f64,
    pub far: f64,
    pub apex_at: f64,
    pub sling_at: f64,
    pub join: f64,
    pub let_go: f64,
    pub apex: crate::knobs::KitKApex,
    pub elbow_min: f64,
    pub wrist_knee: f64,
    pub wrist_max: f64,
    pub roll_in: f64,
    pub over: V3,
    pub up: V3,
    pub top: crate::knobs::KitKTop,
    pub trunk: f64,
    pub belt: crate::knobs::KitKBelt,
    pub holster_pole: V3,
    pub rest: V3,
    pub arc: JsMap<crate::character::KitItem, V3>,
    pub cross: V3,
    pub front: crate::knobs::KitKFront,
    pub nv: crate::knobs::KitKNv,
    pub fist: Vec<f64>,
    pub open: Vec<f64>,
    pub mag: crate::knobs::KitKMag,
    pub close: crate::knobs::KitKClose,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitKApex {
    pub at: V3,
    pub fing: V3,
    pub back: V3,
    pub pole: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitKTop {
    pub fing: V3,
    pub back: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitKBelt {
    pub at: V3,
    pub fing: V3,
    pub back: V3,
    pub pole: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitKFront {
    pub stow: f64,
    pub reach: f64,
    pub draw: f64,
    pub hand_at: f64,
    pub sling_at: f64,
    pub arc: V3,
    pub pole: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitKNv {
    pub hand: crate::knobs::KitKNvHand,
    pub nod: crate::knobs::KitKNvNod,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitKNvHand {
    pub dur: f64,
    pub reach: f64,
    pub out: f64,
    pub flip: [f64; 2],
    pub up: V3,
    pub down: V3,
    pub fingers: V3,
    pub back: V3,
    pub pole: V3,
    pub shrug: f64,
    pub curl: [f64; 4],
    pub thumb: f64,
    pub rate: f64,
    pub away: f64,
    pub clear: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitKNvNod {
    pub dur: f64,
    pub pre: f64,
    pub peak: f64,
    pub flip: [f64; 2],
    pub on: [f64; 2],
    pub off: [f64; 2],
    pub carry: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitKMag {
    pub well: V3,
    pub strip: V3,
    pub slide: V3,
    pub pull: V3,
    pub pouch: V3,
    pub dump: V3,
    pub via: V3,
    pub pole: V3,
    pub pole_w: f64,
    pub wf: V3,
    pub wb: V3,
    pub sf: V3,
    pub sb: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitKClose {
    pub far: f64,
    pub hover: f64,
    pub bow: V3,
    pub lead: V3,
    pub left: f64,
    pub left_at: f64,
    pub left_in: f64,
    pub left_out: f64,
    pub rate: f64,
    pub flex: f64,
    pub flex_amp: f64,
    pub drift: f64,
    pub regrip: f64,
    pub curl: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ThrowByK {
    pub toss: f64,
    pub toss_fast: f64,
    pub toss_in: f64,
    pub toss_min: f64,
    pub through: f64,
    pub toss_pivot: V3,
    pub full: f64,
    pub turn_min: f64,
    pub arm_min: f64,
    pub off_min: f64,
    pub sit: f64,
    pub pivot: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HandsK {
    pub reach_secs: f64,
    pub lift_secs: f64,
    pub set_secs: f64,
    pub grip_m: f64,
    pub step_secs: f64,
    pub head_bias: f64,
    pub ahead: f64,
    pub ahead_head: f64,
    pub carry_feet: f64,
    pub carry_head: f64,
    pub slack_m: f64,
    pub hoist_secs: f64,
    pub lower_secs: f64,
    pub carry_step_secs: f64,
    pub carry_near_m: f64,
    pub carry_side: f64,
    pub carry_fwd: f64,
    pub carry_up: f64,
    pub carry_back: f64,
    pub carry_legs: f64,
    pub carry_pace: f64,
    pub carry_turn: f64,
    pub carry_ride: f64,
    pub lay_ahead: f64,
    pub door_reach: f64,
    pub hand_secs: f64,
    pub door_rate: f64,
    pub open_to: f64,
    pub clear_m: f64,
    pub step_m: f64,
    pub round_m: f64,
    pub hand_m: f64,
    pub in_m: f64,
    pub stall_secs: f64,
    pub aim_arc: f64,
    pub team_pre_m: f64,
    pub team_reach_m: f64,
    pub team_pace: f64,
    pub team_vel: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WorkK {
    pub pick_m: f64,
    pub pick_wide_m: f64,
    pub behind_m: f64,
    pub behind_panel_m: f64,
    pub stand_m: f64,
    pub buddy_m: f64,
    pub hand_secs: f64,
    pub breaker_secs: f64,
    pub fade_secs: f64,
    pub hand_m: f64,
    pub walk_m: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WandK {
    pub feed_secs: f64,
    pub stow_secs: f64,
    pub fov_deg: f64,
    pub blind_deg: f64,
    pub pan_rate: f64,
    pub eye_y: f64,
    pub past_m: f64,
    pub mark_secs: f64,
    pub shut_rad: f64,
    pub give_up_secs: f64,
    pub order_key: String,
    pub sweep_secs: f64,
    pub quick_secs: f64,
    pub quick_down: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StickyK {
    pub carry: f64,
    pub max: f64,
    pub live: f64,
    pub issue: f64,
    pub speed: f64,
    pub gravity: f64,
    pub drag: f64,
    pub radius: f64,
    pub life: f64,
    pub substeps: f64,
    pub solve_iters: f64,
    pub cd: f64,
    pub aim_y: f64,
    pub fov_deg: f64,
    pub off: f64,
    pub flat_y: f64,
    pub launch_noise: f64,
    pub stick_noise: f64,
    pub take_reach: f64,
    pub take_noise: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PeeksightK {
    pub out: f64,
    pub off: f64,
    pub turn_deg: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GunslotsK {
    pub near: f64,
    pub out: f64,
    pub lull: f64,
    pub back: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GunrolesK {
    pub akms: f64,
    pub belt_lane: f64,
    pub belt_min: f64,
    pub belt_squad: f64,
    pub line_max: f64,
    pub pistol_near: f64,
    pub pistol_share: f64,
    pub pistol_max: f64,
    pub lane_k: f64,
    pub file_cost: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KittreeK {
    pub lend: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadthrowK {
    pub on: bool,
    pub frag_roll_k: f64,
    pub again_secs: f64,
    pub shown_secs: f64,
    pub slack_m: f64,
    pub walk_m: f64,
    pub min_m: f64,
    pub max_m: f64,
    pub tries: f64,
    pub walk_secs: f64,
    pub turn_secs: f64,
    pub toss_secs: f64,
    pub lob_paces: Vec<f64>,
    pub lob_bank_paces: Vec<f64>,
    pub lob_speed: f64,
    pub lob_near_m: f64,
    pub lob_up_m: f64,
    pub lob_walls: f64,
    pub lob_wall_m: f64,
    pub lob_wall_h: f64,
    pub lob_sims: f64,
    pub lob_cells: f64,
    pub settle_secs: f64,
    pub lob_steps: Vec<f64>,
    pub duck_secs: f64,
    pub duck_throw_secs: f64,
    pub duck_cells: f64,
    pub duck_apart_m: f64,
    pub duck_min_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Squad40K {
    pub near_m: f64,
    pub arm_slack: f64,
    pub reach_slack: f64,
    pub muzzle_ahead: f64,
    pub muzzle_up: f64,
    pub head_drop: f64,
    pub aim_rad: f64,
    pub aim_secs: f64,
    pub raise_secs: f64,
    pub after_secs: f64,
    pub shown_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AutopilotK {
    pub show_secs: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CmdringK {
    pub key: String,
    pub wedges: f64,
    pub flick_px: f64,
    pub hold_secs: f64,
    pub stay_hold_secs: f64,
    pub tap_secs: f64,
    pub edge_x: f64,
    pub edge_y: f64,
    pub man_px: f64,
    pub ref_: [f64; 2],
    pub goal_half: bool,
    pub twice: bool,
    pub twice_secs: f64,
    pub twice_px: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TorchK {
    pub lit_at: f64,
    pub off_k: f64,
    pub spill_m: f64,
    pub day_m: f64,
    pub ahead_m: f64,
    pub on_secs: f64,
    pub off_secs: f64,
    pub hold_secs: f64,
    pub ask_secs: f64,
    pub calm_aw: f64,
    pub wall_m: f64,
    pub wall_back_m: f64,
    pub low_tan: f64,
    pub low_rate: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BackkeyK {
    pub tab: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MoraleK {
    pub half_life: f64,
    pub ask_secs: f64,
    pub alone: f64,
    pub alone_m: f64,
    pub dark: f64,
    pub lit_asks: f64,
    pub blast: f64,
    pub shock_cap: f64,
    pub blast_m: f64,
    pub near_m: f64,
    pub far_k: f64,
    pub jumpy: f64,
    pub broken: f64,
    pub hyst: f64,
    pub duck_secs: f64,
    pub down_secs: f64,
    pub back_until: f64,
    pub back_m: f64,
    pub back_speed: f64,
    pub burst_rounds: f64,
    pub burst_every: f64,
    pub spread: f64,
    pub call2_secs: f64,
    pub bait_m: f64,
    pub bait_short: f64,
    pub bait_every: f64,
    pub reload_secs: f64,
    pub bark_secs: f64,
    pub fan_deg: f64,
    pub fan_step: f64,
    pub straight_m: f64,
    pub training: crate::morale::Training,
    pub green_broken: f64,
    pub leader_down: f64,
    pub leader_shock: f64,
    pub flinch_secs: f64,
    pub shout_gap: f64,
    pub order_m: f64,
    pub order_beat: f64,
    pub order_secs: f64,
    pub order_gap: f64,
    pub stand_secs: f64,
    pub side_m: f64,
    pub steady_duck: f64,
    pub hold_shock: f64,
    pub hold_max: f64,
    pub nerve: crate::morale::Nerve,
    pub nerve_k: crate::knobs::MoraleKNerveK,
    pub dark_read: crate::knobs::MoraleKDarkRead,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MoraleKNerveK {
    pub brittle: f64,
    pub normal: f64,
    pub steady: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum MoraleKDarkRead {
    #[default] Place,
    Watch,
}
impl MoraleKDarkRead {
    pub const fn as_str(self) -> &'static str {
        match self {
            MoraleKDarkRead::Place => "place",
            MoraleKDarkRead::Watch => "watch",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ConditionK {
    pub hurt_pace: f64,
    pub hurt_sup: f64,
    pub hurt_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct InhandK {
    pub from: f64,
    pub to: f64,
    pub wrist: f64,
    pub both: f64,
    pub rate: f64,
    pub holster_out: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PacksideK {
    pub gap: f64,
    pub handle: f64,
    pub gun: f64,
    pub over: f64,
    pub head_min: f64,
    pub butt: f64,
    pub rest: f64,
    pub pitch0: f64,
    pub pitch1: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RunnerK {
    pub close_m: f64,
    pub out_secs: f64,
    pub beat_secs: f64,
    pub tell_m: f64,
    pub told_m: f64,
    pub tell_beat: f64,
    pub tell_aware: f64,
    pub shy_m: f64,
    pub mate_m: f64,
    pub mate_secs: f64,
    pub clear_m: f64,
    pub edge_m: f64,
    pub corner_m: f64,
    pub gone_secs: f64,
    pub stall_secs: f64,
    pub retry_secs: f64,
    pub replan_secs: f64,
    pub ahead_m: f64,
    pub ask_cells: f64,
    pub ask_wide: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StartleK {
    pub beat: f64,
    pub draw: f64,
    pub fresh: f64,
    pub late: f64,
    pub run_m: f64,
    pub near_m: f64,
    pub clear_secs: f64,
    pub run_secs: f64,
    pub stuck_secs: f64,
    pub stuck_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SmokebreakK {
    pub light_secs: f64,
    pub light_up: [f64; 2],
    pub flame_at: f64,
    pub flame: f64,
    pub every: f64,
    pub draw: f64,
    pub out_after: f64,
    pub out: f64,
    pub flame_color: V3,
    pub flame_k: f64,
    pub flame_range: f64,
    pub ember_color: V3,
    pub ember_k: f64,
    pub ember_draw_k: f64,
    pub ember_range: f64,
    pub ember_every: f64,
    pub tip_out: f64,
    pub tip_up: f64,
    pub thread: crate::knobs::SmokebreakKThread,
    pub breath: crate::knobs::SmokebreakKBreath,
    pub color: V3,
    pub mouth_ahead: f64,
    pub mouth_down: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SmokebreakKThread {
    pub radius: f64,
    pub cells: f64,
    pub density: f64,
    pub temp: f64,
    pub speed: f64,
    pub push: f64,
    pub jitter: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SmokebreakKBreath {
    pub radius: f64,
    pub cells: f64,
    pub density: f64,
    pub temp: f64,
    pub speed: f64,
    pub push: f64,
    pub expand: f64,
    pub attack: f64,
    pub jitter: f64,
    pub down: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShowwhyK {
    pub line_after: f64,
    pub line_gap: f64,
    pub wave_every: f64,
    pub wave_later: f64,
    pub line_min: f64,
    pub line_say_gap: f64,
    pub mate_half: f64,
    pub squeeze_gap: f64,
    pub squeeze_reach: f64,
    pub fist_after: f64,
    pub hold_every: f64,
    pub hold_max: f64,
    pub door_after: f64,
    pub door_every: f64,
    pub after_you_gap: f64,
    pub hurt_late: f64,
    pub lag_m: f64,
    pub lag_every: f64,
    pub lag_max: f64,
    pub keep_after: f64,
    pub keep_gap: f64,
    pub keep_every: f64,
    pub owed_for: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FireK {
    pub names: Vec<String>,
    pub height: f64,
    pub mouth: f64,
    pub coals: f64,
    pub coal_swing: f64,
    pub bed_r: f64,
    pub depth: f64,
    pub rates: Vec<f64>,
    pub weights: Vec<f64>,
    pub least: f64,
    pub most: f64,
    pub wander: f64,
    pub wander_up: f64,
    pub lift: f64,
    pub kick: f64,
    pub kick_secs: f64,
    pub bright: f64,
    pub warm: Vec<f64>,
    pub reach: f64,
    pub stir_gap: f64,
    pub size: f64,
    pub pop_rate: f64,
    pub pop_slot: f64,
    pub embers: f64,
    pub ember_life: f64,
    pub ember_speed: f64,
    pub ember_size: f64,
    pub ember_gain: f64,
    pub ember_crack: f64,
    pub ember_r: f64,
    pub look: crate::knobs::FireKLook,
    pub smoke: crate::knobs::FireKSmoke,
    pub bed: f64,
    pub crack: f64,
    pub bed_secs: f64,
    pub max: f64,
    pub draw_r: f64,
    pub wind_max: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FireKLook {
    pub gain: f64,
    pub core: f64,
    pub sigma: f64,
    pub speed: f64,
    pub tall: f64,
    pub torn: f64,
    pub plume: f64,
    pub shimmer: f64,
    pub soot: f64,
    pub breathe: f64,
    pub react: f64,
    pub lean: f64,
    pub steps: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FireKSmoke {
    pub on: bool,
    pub up: f64,
    pub radius: f64,
    pub cells: f64,
    pub density: f64,
    pub temp: f64,
    pub speed: f64,
    pub push: f64,
    pub jitter: f64,
    pub every: f64,
    pub view_r: f64,
    pub lean: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FlinchK {
    pub least: f64,
    pub full: f64,
    pub far: f64,
    pub min_travel: f64,
    pub steady_k: f64,
    pub break_secs: f64,
    pub hot_secs: f64,
    pub ease_k: f64,
    pub ask_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DarkdoorK {
    pub who: crate::knobs::DarkdoorKWho,
    pub stop_m: f64,
    pub pair_m: f64,
    pub dark_at: f64,
    pub dark_share: f64,
    pub lit_by: f64,
    pub samples: f64,
    pub ask_secs: f64,
    pub open_rad: f64,
    pub arm_secs: f64,
    pub stand_m: f64,
    pub side_m: f64,
    pub sweep_secs: f64,
    pub ear_m: f64,
    pub mate_m: f64,
    pub mate_try: f64,
    pub call_secs: f64,
    pub copy_secs: f64,
    pub lag_secs: f64,
    pub with_secs: f64,
    pub keep_aw: f64,
    pub keep_secs: f64,
    pub resume_secs: f64,
    pub seen_secs: f64,
    pub wait_secs: f64,
    pub again_secs: f64,
    pub fed_up_secs: f64,
    pub fed_up_n: f64,
    pub nerve_secs: f64,
    pub slow_k: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DarkdoorKWho {
    #[default] None,
    All,
    Rest,
}
impl DarkdoorKWho {
    pub const fn as_str(self) -> &'static str {
        match self {
            DarkdoorKWho::None => "none",
            DarkdoorKWho::All => "all",
            DarkdoorKWho::Rest => "rest",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DarkfoeK {
    pub carry: f64,
    pub lit_at: f64,
    pub look_m: f64,
    pub step_m: f64,
    pub fan_rad: f64,
    pub see_share: f64,
    pub ask_secs: f64,
    pub on_secs: f64,
    pub on_jitter: f64,
    pub off_secs: f64,
    pub off_jitter: f64,
    pub hold_secs: f64,
    pub lamp_m: f64,
    pub blind_secs: f64,
    pub seek_m: f64,
    pub walk_m: f64,
    pub edge_m: f64,
    pub stumble_k: f64,
    pub again_secs: f64,
    pub moves: f64,
    pub quiet_secs: f64,
    pub calm_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PoweroutK {
    pub see_m: f64,
    pub think: [f64; 2],
    pub beat: f64,
    pub look_m: f64,
    pub about_m: f64,
    pub step_m: f64,
    pub fan_rad: f64,
    pub lit_at: f64,
    pub drop_k: f64,
    pub dark_after: f64,
    pub men: f64,
    pub max_m: f64,
    pub parties: f64,
    pub look_secs: f64,
    pub tries: f64,
    pub lever_secs: f64,
    pub overdue: f64,
    pub ask: f64,
    pub cover_m: [f64; 2],
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquaddealK {
    pub one: f64,
    pub two: f64,
    pub names: Vec<String>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StayaskK {
    pub men: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescuecheckK {
    pub first: [f64; 2],
    pub every: [f64; 2],
    pub ask_secs: f64,
    pub go_secs: f64,
    pub wait_secs: f64,
    pub tries: f64,
    pub feet_m: f64,
    pub word_secs: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KitloadK {
    pub gun_kg: f64,
    pub gun_bulk: [f64; 2],
    pub small: JsMap<String, [f64; 2]>,
    pub big: JsMap<String, [f64; 2]>,
    pub pace: f64,
    pub seen: f64,
    pub heard: f64,
    pub sprint: f64,
    pub pace_top: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueendK {
    pub out_m: f64,
    pub clear_secs: f64,
    pub marker_m: f64,
    pub reach_m: f64,
    pub gap_m: f64,
    pub gap_step_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueflushK {
    pub after_secs: f64,
    pub gap_secs: f64,
    pub again_secs: f64,
    pub reach_m: f64,
    pub kill_secs: f64,
    pub busy_secs: f64,
    pub smoke_k: f64,
    pub at_door_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PosemathK {
    pub singles: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SenseK {
    pub glow: crate::knobs::SenseKGlow,
    pub smoke: crate::knobs::SenseKSmoke,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SenseKGlow {
    pub sweep: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SenseKSmoke {
    pub sigma: f64,
    pub decay: f64,
    pub spread: f64,
    pub rise: f64,
    pub rise_rate: f64,
    pub in_wind: f64,
    pub least: f64,
    pub gone: f64,
    pub reserve: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CmdcheckK {
    pub keys: f64,
    pub key_len: f64,
    pub span: f64,
    pub wheel: f64,
    pub far: f64,
    pub dist_min: f64,
    pub dist_max: f64,
    pub fov_min: f64,
    pub fov_max: f64,
    pub aspect_min: f64,
    pub aspect_max: f64,
    pub reach: f64,
    pub steep: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SeatsK {
    pub max: f64,
    pub apart: f64,
    pub keep: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DozeK {
    pub min_wait: f64,
    pub max_wait: f64,
    pub chair_m: f64,
    pub nod_secs: f64,
    pub rise_secs: f64,
    pub hear_k: f64,
    pub light_at: f64,
    pub light_secs: f64,
    pub ask_secs: f64,
    pub wake_aw: f64,
    pub bump_aw: f64,
    pub stir_secs: f64,
    pub breath_secs: f64,
    pub screen_m: f64,
    pub pull_m: f64,
    pub nap_wait: f64,
    pub settle_secs: f64,
    pub ears: f64,
    pub noises: f64,
    pub say_first: Vec<f64>,
    pub say_every: Vec<f64>,
    pub idle_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct NetquietK {
    pub unanswered: f64,
    pub heard_it: f64,
    pub body: f64,
    pub body_told: f64,
    pub fall_seen: f64,
    pub fall_unseen: f64,
    pub steady_k: f64,
    pub ask_beat: f64,
    pub body_beat: f64,
    pub bark_secs: f64,
    pub say_hold: f64,
    pub shooter_secs: f64,
    pub body_m: f64,
    pub whip_shift: f64,
    pub whip_secs: f64,
    pub whip_hold: f64,
    pub whip_turn: f64,
    pub whip_turn_broken: f64,
    pub whip_m: f64,
    pub whip_near_m: f64,
    pub last_beat: f64,
    pub calm_secs: f64,
    pub alert_secs: f64,
    pub give_up: f64,
    pub mate_m: f64,
    pub arc_deg: f64,
    pub eye_deg: f64,
    pub eye_slack: f64,
    pub crouch: f64,
    pub wary: f64,
    pub back_speed: f64,
    pub max_m: f64,
    pub door_m: f64,
    pub eye_m: f64,
    pub stay_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AttentionK {
    pub on: bool,
    pub focus_deg: f64,
    pub side_k: f64,
    pub ear_k: f64,
    pub break_gain: f64,
    pub calm_aw: f64,
    pub draw_aw: f64,
    pub floor: f64,
    pub whistle_secs: f64,
    pub noise_secs: f64,
    pub door_secs: f64,
    pub light_secs: f64,
    pub tell_secs: f64,
    pub dwell_sweep: f64,
    pub walk_cap: f64,
    pub wear_k: f64,
    pub memory_secs: f64,
    pub spent: f64,
    pub same_secs: f64,
    pub lent_back_secs: f64,
    pub marks: bool,
    pub ring_m: f64,
    pub way: bool,
    pub way_reask: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PictureK {
    pub look_secs: f64,
    pub glimpse: f64,
    pub tell_secs: f64,
    pub shout_m: f64,
    pub pop_m: f64,
    pub made_out: f64,
    pub made_out_m: f64,
    pub fresh_secs: f64,
    pub stale_secs: f64,
    pub stale_k: f64,
    pub forget_secs: f64,
    pub lost_k: f64,
    pub walk_mps: f64,
    pub walk_max: f64,
    pub heard_m: f64,
    pub maybe_k: f64,
    pub reach_m: f64,
    pub eye_h: f64,
    pub chest_h: f64,
    pub low_h: f64,
    pub head_up: f64,
    pub flee_cost: f64,
    pub stack_k: f64,
    pub step_at: f64,
    pub step_free: f64,
    pub step_m: f64,
    pub step_after: f64,
    pub step_rest: f64,
    pub step_secs: f64,
    pub show: crate::knobs::PictureKShow,
    pub show_m: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PictureKShow {
    #[default] All,
    Off,
    Cursor,
}
impl PictureKShow {
    pub const fn as_str(self) -> &'static str {
        match self {
            PictureKShow::All => "all",
            PictureKShow::Off => "off",
            PictureKShow::Cursor => "cursor",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ResponseK {
    pub clear_m: f64,
    pub eta_step: f64,
    pub word_secs: f64,
    pub call_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HandworkK {
    pub stand_m: f64,
    pub step_rate: f64,
    pub step_pace: f64,
    pub stand_side: f64,
    pub wait_secs: f64,
    pub lever_secs: f64,
    pub lever_at: f64,
    pub stow_secs: f64,
    pub radio_k: f64,
    pub radio_min: f64,
    pub radio_max: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WorkstowK {
    pub mate_down: f64,
    pub mate_up: f64,
    pub show: f64,
    pub down_max: f64,
    pub sam_up: f64,
    pub fight_cap: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadnetK {
    pub near: f64,
    pub edge: bool,
    pub edge_in: f64,
    pub click: f64,
    pub click_rate: f64,
    pub down_gain: f64,
    pub down_rate: f64,
    pub down_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TakedownK {
    pub partner_hit: f64,
    pub partner_end: f64,
    pub breath_hz: f64,
    pub breath_rise: f64,
    pub breath_in: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StealthposeK {
    pub sway_secs: f64,
    pub sway_m: f64,
    pub drift_m: f64,
    pub rise_secs: f64,
    pub cable_in: f64,
    pub cable_bow: f64,
    pub cable_step: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LookK {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LivedK {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EnvK {
    pub exposure: f64,
    pub tube_gain: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShapesK {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FlashK {
    pub secs: [f64; 2],
    pub stagger: [f64; 2],
    pub lift: f64,
    pub concuss_m: f64,
    pub concuss_k: f64,
    pub own_m: f64,
    pub own_secs: f64,
    pub fuse: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PaceK {
    pub max: f64,
    pub accel: f64,
    pub turn: f64,
    pub swing: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LightsK {
    pub after_secs: f64,
    pub hot_secs: f64,
    pub step_secs: f64,
    pub tail: f64,
    pub on_m: f64,
    pub alone_m: f64,
    pub lit_at: f64,
    pub lit_m: f64,
    pub lit_mark_m: f64,
    pub wall_m: f64,
    pub wall_back_m: f64,
    pub wall_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct NvgK {
    pub quiet_secs: f64,
    pub lead_tail: f64,
    pub dark_at: f64,
    pub black_at: f64,
    pub black_reads: f64,
    pub clear_reads: f64,
    pub albedo: f64,
    pub noise: f64,
    pub down_secs: f64,
    pub up_secs: f64,
    pub call_gap: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct IrlightK {
    pub show: f64,
    pub hold: f64,
    pub power: f64,
    pub reach: f64,
    pub cone: f64,
    pub core: f64,
    pub dip: f64,
    pub near_m: f64,
    pub near_min: f64,
    pub near_up: f64,
    pub near_down: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallpenK {
    pub base: JsMap<crate::wallpen::ThinStuff, f64>,
    pub per_m: JsMap<crate::wallpen::ThinStuff, f64>,
    pub thin_max: JsMap<crate::wallpen::ThinStuff, f64>,
    pub max_thru: JsMap<crate::wallpen::ThinStuff, f64>,
    pub min_energy: f64,
    pub max_things: f64,
    pub deflect_deg: f64,
    pub behind_m: f64,
    pub thru_rounds: f64,
    pub thru_rest_secs: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoortoolsK {
    pub charges: f64,
    pub bangs: f64,
    pub blow_key: String,
    pub word_tap: f64,
    pub stack_cap: f64,
    pub still_secs: f64,
    pub kick_cap: f64,
    pub in_cap: f64,
    pub shove: f64,
    pub hold_open: f64,
    pub toss_after_open: f64,
    pub bang_in: f64,
    pub fuse_after_land: f64,
    pub air_in: f64,
    pub air_h: f64,
    pub toss_cap: f64,
    pub toss_after_kick: f64,
    pub throw_reach: f64,
    pub toss_turn: f64,
    pub frag_way_wait: f64,
    pub frag_way_open: f64,
    pub frag_way_ajar: f64,
    pub last_out_m: f64,
    pub after_bang: f64,
    pub quiet: f64,
    pub entry_cap: f64,
    pub second: f64,
    pub sweep_rad: f64,
    pub sweep_secs: f64,
    pub corner_secs: f64,
    pub corner_rad: f64,
    pub corner_cap: f64,
    pub lead_seen_min: f64,
    pub lead_seen_secs: f64,
    pub lead_seen_m: f64,
    pub sector_secs: f64,
    pub scan_rate: f64,
    pub look_rad: f64,
    pub look_m: f64,
    pub dead_cells: f64,
    pub search_moves: f64,
    pub search_cap: f64,
    pub search_cells: f64,
    pub tuck: f64,
    pub edge_out: f64,
    pub bang_file: f64,
    pub set_secs: f64,
    pub standoff: Vec<f64>,
    pub knock_m: f64,
    pub stun_m: f64,
    pub stun_secs: Vec<f64>,
    pub stagger_secs: f64,
    pub backblast_m: f64,
    pub backblast_secs: f64,
    pub blow_vel: f64,
    pub slice_back: f64,
    pub slice_look: f64,
    pub slice_edge: f64,
    pub slice_deep: f64,
    pub slice_pace: f64,
    pub word_stack: f64,
    pub word_in: f64,
    pub room_back: Vec<f64>,
    pub pod_apart: f64,
    pub thr_pull_at: Vec<f64>,
    pub thr_still: Vec<f64>,
    pub thr_arc: Vec<Vec<f64>>,
    pub thr_min: f64,
    pub thr_quiet: f64,
    pub thr_cap: f64,
    pub thr_bangs: f64,
    pub thr_pocket: f64,
    pub thr_shot_m: f64,
    pub thr_throw_m: f64,
    pub thr_flash_m: f64,
    pub thr_turn: f64,
    pub thr_next: f64,
    pub thr_light: f64,
    pub thr_in: f64,
    pub thr_pie_arc: Vec<Vec<f64>>,
    pub thr_pie_apart: f64,
    pub thr_lean_m: f64,
    pub thr_lean_look: f64,
    pub thr_doors_m: f64,
    pub thr_doors_walk: f64,
    pub thr_doors_stay: f64,
    pub other_door_m: f64,
    pub buddy_m: f64,
    pub buddy_pair_m: f64,
    pub buddy_left_m: f64,
    pub flow_cells: f64,
    pub flow_moves: f64,
    pub flow_reach: f64,
    pub flow_deep: f64,
    pub flow_bound: f64,
    pub flow_secs: f64,
    pub flow_fight: f64,
    pub flow_max: f64,
    pub word_cuts_cap: f64,
    pub bang_into_min: f64,
    pub bang_into_lit: f64,
    pub bang_into_deep: f64,
    pub bang_into_wide: f64,
    pub bang_hand_m: f64,
    pub bang_pass_v: f64,
    pub pod_frame_m: f64,
    pub thr_hand_wait: f64,
    pub thr_hand_every: f64,
    pub frag_hold_cap: f64,
    pub threat_secs: f64,
    pub threat_cap: f64,
    pub threat_near_m: f64,
    pub threat_stuck: f64,
    pub thr_first: f64,
    pub thr_first_min: f64,
    pub thr_first_stale: f64,
    pub thr_first_in: f64,
    pub thr_last_m: f64,
    pub thr_last_fresh: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RoomfightK {
    pub evidence_secs: f64,
    pub sticky: f64,
    pub crowd: f64,
    pub fear_min: f64,
    pub close_m: f64,
    pub no_shot: f64,
    pub point_blank: f64,
    pub no_shot_go: f64,
    pub no_shot_pass: f64,
    pub rally_k: f64,
    pub arc_half_deg: f64,
    pub cover_half_deg: f64,
    pub cover_near_m: f64,
    pub team_hot: f64,
    pub let_go: f64,
    pub step_look_secs: f64,
    pub door_look_secs: f64,
    pub door_shot_m: f64,
    pub hold_short_secs: f64,
    pub move_engage_m: f64,
    pub lamp: f64,
    pub preaim_deg: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct OrdersK {
    pub who_key: String,
    pub quick_key: String,
    pub wait_key: String,
    pub wait_alt: String,
    pub refresh_secs: f64,
    pub door_pick_m: f64,
    pub self_m: f64,
    pub man_m: f64,
    pub door_ray_pad: f64,
    pub door_ray_reach_m: f64,
    pub floor_snap_m: f64,
    pub jamb_out: f64,
    pub jamb_past: f64,
    pub file_gap: f64,
    pub seam_m: f64,
    pub work_seen_m: f64,
    pub follow_back: Vec<f64>,
    pub crumb: f64,
    pub trail_m: f64,
    pub run: f64,
    pub walk: f64,
    pub gain: f64,
    pub near_m: f64,
    pub band_m: f64,
    pub word_far_m: f64,
    pub word_hold_s: f64,
    pub file_apart: f64,
    pub file_tight: f64,
    pub off_door: f64,
    pub aside_m: f64,
    pub aside_lane: f64,
    pub aside_off: f64,
    pub file_queue_gap: f64,
    pub file_queue_cost: f64,
    pub prop_round_pad: f64,
    pub prop_round_cost: f64,
    pub prop_round_more: f64,
    pub prop_round_door: f64,
    pub aside_hold: f64,
    pub brisk: f64,
    pub brisk_in: f64,
    pub onto: f64,
    pub entry: f64,
    pub move_spread: f64,
    pub cold_secs: f64,
    pub lane_in: f64,
    pub lane_out: f64,
    pub hold_stride: f64,
    pub stuck_secs: f64,
    pub hold_group_m: f64,
    pub hold_door_m: f64,
    pub cleared_dear: f64,
    pub round_cap: f64,
    pub same_near_m: f64,
    pub same_apart_m: f64,
    pub same_long: f64,
    pub trail_reach_m: f64,
    pub trail_long: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CoverK {
    pub rear_read_in: f64,
    pub rear_read_m: f64,
    pub rear_read_deg: f64,
    pub rear_read_secs: f64,
    pub rear_read_sticky: f64,
    pub rear_look: f64,
    pub rear_walk_m: f64,
    pub reach: f64,
    pub deal_secs: f64,
    pub gap_cells: f64,
    pub through_rows: f64,
    pub alcove_rows: f64,
    pub bridge: f64,
    pub in_m: f64,
    pub lane: f64,
    pub lane_keep: f64,
    pub lane_secs: f64,
    pub sam_deg: f64,
    pub sam_k: f64,
    pub held_k: f64,
    pub shut_k: f64,
    pub rear_k: f64,
    pub near_m: f64,
    pub contact_secs: f64,
    pub rear_m: f64,
    pub rear_from: f64,
    pub rear_deg: f64,
    pub sticky: f64,
    pub keep_secs: f64,
    pub rear_keep: f64,
    pub behind_k: f64,
    pub twist: f64,
    pub back_in: f64,
    pub back_out: f64,
    pub back_body: f64,
    pub back_turn_body: f64,
    pub back_turn_secs: f64,
    pub swing: f64,
    pub step_m: f64,
    pub step_after: f64,
    pub step_off_m: f64,
    pub step_reach: f64,
    pub step_secs: f64,
    pub step_place_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RoomfitK {
    pub spread0: f64,
    pub spread_per: f64,
    pub apart: f64,
    pub loose: f64,
    pub loose_k: f64,
    pub near_k: f64,
    pub near_free: f64,
    pub wall_k: f64,
    pub corner_k: f64,
    pub funnel_m: f64,
    pub funnel_pad: f64,
    pub funnel_flare: f64,
    pub funnel_k: f64,
    pub own_k: f64,
    pub known_funnel_k: f64,
    pub shut_funnel_k: f64,
    pub ang_k: f64,
    pub ang_lo: f64,
    pub ang_hi: f64,
    pub ang_max: f64,
    pub close_m: f64,
    pub close_k: f64,
    pub lane_m: f64,
    pub lane_k: f64,
    pub reach: f64,
    pub near_m: f64,
    pub known_k: f64,
    pub shut_k: f64,
    pub locked_k: f64,
    pub front_k: f64,
    pub second_k: f64,
    pub sector_m: f64,
    pub sector_k: f64,
    pub sector_deg: f64,
    pub sector_across: f64,
    pub sector_look: f64,
    pub sector_far_m: f64,
    pub sector_far_k: f64,
    pub watch_in_m: f64,
    pub again_secs: f64,
    pub sticky: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EntryK {
    pub near_m: f64,
    pub line_deg: f64,
    pub known_m: f64,
    pub move_m: f64,
    pub apart: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RearK {
    pub secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SilentK {
    pub on: bool,
    pub start: bool,
    pub pace: f64,
    pub step: f64,
    pub noticed: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PairK {
    pub on: bool,
    pub max_squad: f64,
    pub pick_m: f64,
    pub stage_m: f64,
    pub reach: f64,
    pub secs: f64,
    pub scuffle: f64,
    pub pace: f64,
    pub go_pace: f64,
    pub stalk_every: f64,
    pub stalk_wait: f64,
    pub ring_m: f64,
    pub steady_face_secs: f64,
    pub watch_secs: f64,
    pub watch_aw: f64,
    pub heel_ring: f64,
    pub heel_fresh: f64,
    pub heel_wait: f64,
    pub take_secs: f64,
    pub dark_vis: f64,
    pub dark_m: f64,
    pub dark_h: f64,
    pub lit_secs: f64,
    pub lit_every: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueK {
    pub start: crate::knobs::RescueKStart,
    pub resolve_secs: f64,
    pub second_hostage_secs: f64,
    pub aim_secs: f64,
    pub set_secs: f64,
    pub unset_react: f64,
    pub door_man: crate::knobs::RescueKDoorMan,
    pub roamers: f64,
    pub crew_call_secs: f64,
    pub squad: f64,
    pub sam_at: V3,
    pub sam_yaw: f64,
    pub ops_at: [V3; 2],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RescueKStart {
    #[default] Quiet,
    Hot,
}
impl RescueKStart {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueKStart::Quiet => "quiet",
            RescueKStart::Hot => "hot",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RescueKDoorMan {
    #[default] Inside,
    Corridor,
}
impl RescueKDoorMan {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueKDoorMan::Inside => "inside",
            RescueKDoorMan::Corridor => "corridor",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DiginK {
    pub on: bool,
    pub stay: bool,
    pub reach_m: f64,
    pub keeper: bool,
    pub cover: bool,
    pub brace_in_room: bool,
    pub noticed_secs: f64,
    pub shut_open: bool,
    pub shut_secs: f64,
    pub shut_home_rad: f64,
    pub shut_doors: f64,
    pub gather: bool,
    pub gather_m: f64,
    pub gather_shy_m: f64,
    pub think: [f64; 2],
    pub wedge_secs: [f64; 2],
    pub wedges: f64,
    pub caught: [f64; 2],
    pub try_secs: f64,
    pub pace: f64,
    pub close_m: f64,
    pub prop_m: f64,
    pub haul_speed: f64,
    pub scrape_level: f64,
    pub scrape_every: f64,
    pub shout_m: f64,
    pub alt_m: f64,
    pub better: f64,
    pub fear: f64,
    pub hold_secs: f64,
    pub redig_secs: f64,
    pub lane_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PressureK {
    pub on: bool,
    pub press: bool,
    pub flank: bool,
    pub squad_drill: bool,
    pub second: bool,
    pub max_men: f64,
    pub memory_secs: f64,
    pub organise_k: f64,
    pub warned: bool,
    pub stand_press: bool,
    pub report_secs: f64,
    pub stand_secs: f64,
    pub stand_grace: f64,
    pub again_secs: f64,
    pub stuns: f64,
    pub stun_know_secs: f64,
    pub stun_moved_secs: f64,
    pub last_holds: bool,
    pub picket: bool,
    pub by_way: bool,
    pub rescue_crew: bool,
    pub drilled: bool,
    pub flank_max_m: f64,
    pub flank_k: f64,
    pub flank_slack_m: f64,
    pub last_man_secs: [f64; 2],
    pub loud_secs: f64,
    pub out_secs: [f64; 2],
    pub gather_secs: f64,
    pub flank_secs: f64,
    pub siege_secs: f64,
    pub word_secs: f64,
    pub way_out: bool,
    pub way_out_m: f64,
    pub way_out_secs: f64,
    pub sweep: bool,
    pub sweep_secs: f64,
    pub sweep_men: f64,
    pub sweep_beat: f64,
    pub sweep_pair_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LanesK {
    pub secs: f64,
    pub reach_m: f64,
    pub half_deg: f64,
    pub width_m: f64,
    pub field_m: f64,
    pub own_m: f64,
    pub min_m: f64,
    pub round_k: f64,
    pub near_m: f64,
    pub react_secs: f64,
    pub call_secs: f64,
    pub see_m: f64,
    pub body_m: f64,
    pub dear: f64,
    pub ahead_m: f64,
    pub close_m: f64,
    pub hot_secs: f64,
    pub round_men: f64,
    pub think_secs: f64,
    pub round_max_m: f64,
    pub round_gap: f64,
    pub round_side_deg: f64,
    pub round_show_m: f64,
    pub shift_m: f64,
    pub stun_m: f64,
    pub bang_m: f64,
    pub push_secs: f64,
    pub push_m: f64,
    pub push_fast: f64,
    pub stun_gap: f64,
    pub moved_m: f64,
    pub moved_secs: f64,
    pub peek_quiet: f64,
    pub peek_gap: f64,
    pub peek_max: f64,
    pub peek_lost: f64,
    pub peek_reach_m: f64,
    pub peek_near_m: f64,
    pub peek_look: f64,
    pub peek_sure: f64,
    pub peek_keep: f64,
    pub peek_pace: f64,
    pub peek_lean: f64,
    pub pin_rounds: f64,
    pub pin_every: f64,
    pub pin_snap: f64,
    pub pin_gap: f64,
    pub pin_max: f64,
    pub pin_rest: f64,
    pub pin_spread: f64,
    pub pin_aim_deg: f64,
    pub pin_pace: f64,
    pub pin_show_m: f64,
    pub pin_min_m: f64,
    pub look_m: f64,
    pub frag_think: f64,
    pub frag_gap: f64,
    pub frag_look: f64,
    pub frag_step_m: f64,
    pub max: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GunholdK {
    pub cover_m: f64,
    pub near_m: f64,
    pub rear_deg: f64,
    pub keep_secs: f64,
    pub peek_every: f64,
    pub peek_secs: f64,
    pub show_m: f64,
    pub shy_s: f64,
    pub dash_m: f64,
    pub face_deg: f64,
    pub flank_max_m: f64,
    pub react: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HardK {
    pub reach_m: f64,
    pub stuns: f64,
}

#[derive(Clone, Debug, ToBytes)]
pub struct HardAtK {
    /// INVENTED: HARD_AT's rows by structure name, written order, each row's optional stuns (difficulty.ts:92). A row with no stuns reads as no row at all (:150, :167, :352, :354)
    pub rows: JsMap<String, Option<f64>>,
}
impl Default for HardAtK {
    fn default() -> Self {
        HardAtK {
            rows: { let mut t = JsMap::new(); t.set(String::from("the block"), Some(1.0)); t },
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DrilledK {
    pub shaken: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadsK {
    pub think: [f64; 2],
    pub net_lag: f64,
    pub help_men: f64,
    pub flank_men: f64,
    pub again_secs: f64,
    pub settle_secs: f64,
    pub quiet_secs: f64,
    pub recall_secs: f64,
    pub reinforce: [f64; 2],
    pub ready_beat: f64,
    pub fire_m: f64,
    pub rear_m: f64,
    pub soft_m: f64,
    pub mouth_m: f64,
    pub hold_back_m: f64,
    pub hold_secs: f64,
    pub drill_net: bool,
    pub net_k: f64,
    pub reinforce_k: f64,
    pub drill_angles: bool,
    pub follow_m: f64,
    pub drill_own: bool,
    pub own_up: f64,
    pub drill_back: bool,
    pub back_beat: f64,
    pub drill_look: bool,
    pub sweep_secs: f64,
    pub sweep_men: f64,
    pub sweep_beat: f64,
    pub sweep_stayers: bool,
    pub check_every: f64,
    pub check_jitter: f64,
    pub check_alert_k: f64,
    pub check_ask: f64,
    pub check_send: f64,
    pub check_heard: [f64; 2],
    pub check_hear_m: f64,
    pub look_men: f64,
    pub call_secs: [f64; 2],
    pub call_cut_ask: [f64; 2],
    pub net_every: f64,
    pub net_beat: [f64; 2],
    pub net_look_men: f64,
    pub net_stops: f64,
}

#[derive(Clone, Debug, ToBytes)]
pub struct DrillAtK {
    /// INVENTED: DRILL_AT's rows by structure name, in written order (enemysquads.ts:287-300). A map with no row reads the drill's table as it stands (:305)
    pub rows: JsMap<String, crate::enemysquads::DrillAtRow>,
}
impl Default for DrillAtK {
    fn default() -> Self {
        DrillAtK {
            rows: { let mut t = JsMap::new(); t.set(String::from("the block, three squads"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(true), net_k: Some(2.0), reinforce_k: Some(2.0), drill_own: Some(false), own_up: None, drill_back: Some(false), back_beat: None }); t.set(String::from("the warehouse"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(false), net_k: None, reinforce_k: None, drill_own: Some(false), own_up: None, drill_back: Some(false), back_beat: None }); t.set(String::from("the block"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(false), net_k: None, reinforce_k: None, drill_own: Some(false), own_up: None, drill_back: Some(false), back_beat: None }); t.set(String::from("the compound"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(true), net_k: Some(1.0), reinforce_k: Some(0.75), drill_own: Some(true), own_up: Some(1.0), drill_back: Some(false), back_beat: None }); t.set(String::from("the barracks"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(false), net_k: None, reinforce_k: None, drill_own: Some(false), own_up: None, drill_back: Some(true), back_beat: Some(3.0) }); t },
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CuesK {
    pub rows: f64,
    pub secs: f64,
    pub min_show: f64,
    pub per_class: f64,
    pub dup_secs: f64,
    pub gaps: Vec<f64>,
    pub squelch: f64,
    pub man_secs: f64,
    pub again_secs: f64,
    pub still_secs: f64,
    pub still_quiet: f64,
    pub stay_secs: f64,
    pub overheard_gap: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WxSimK {
    pub rain_k: f64,
    pub thunder_k: f64,
    pub thunder_secs: f64,
    pub thunder_loud: f64,
    pub roof_bed: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AirSimK {
    pub glass_open: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LaststandK {
    pub moved_secs: f64,
    pub still_secs: f64,
    pub moved_gap: f64,
    pub few: f64,
    pub quiet: f64,
    pub first: [f64; 2],
    pub every: f64,
    pub answer: [f64; 2],
    pub between: [f64; 2],
    pub listen: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StackroomK {
    pub apart: f64,
    pub squeeze: f64,
    pub round_m: f64,
    pub round_k: f64,
    pub walk_m: f64,
    pub next_walk_m: f64,
    pub next_m: f64,
    pub lane_back: f64,
    pub view_ajar: f64,
    pub view_cells: f64,
    pub view_far: f64,
    pub view_eye: f64,
    pub view_step: f64,
    pub view_past: f64,
    pub view_off: f64,
    pub view_apart: f64,
    pub view_out: f64,
    pub view_men: f64,
    pub view_wall: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RearwatchK {
    pub reach: f64,
    pub ajar: f64,
    pub grace: f64,
    pub short: f64,
    pub room_secs: f64,
    pub room_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StacklineK {
    pub ajar: f64,
    pub bins: f64,
    pub eye: f64,
    pub inset: f64,
    pub on_mark: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SneakK {
    pub on: bool,
    pub min_pace: f64,
    pub low_pace: f64,
    pub cross_pace: f64,
    pub step: f64,
    pub trail_m: f64,
    pub back: f64,
    pub ahead_m: f64,
    pub lit_at: f64,
    pub seen_rate: f64,
    pub cone_deg: f64,
    pub near_m: f64,
    pub cross_m: f64,
    pub patience: f64,
    pub cross_secs: f64,
    pub budget: f64,
    pub margin: f64,
    pub memory: f64,
    pub nag_secs: f64,
    pub cue_gap: f64,
    pub stand_m: f64,
    pub stand_near: f64,
    pub side_m: f64,
    pub side_stop: f64,
    pub door_m: f64,
    pub door_pad: f64,
    pub door_settle: f64,
    pub hide_gap: f64,
    pub noise_walk_m: f64,
    pub plan_secs: f64,
    pub skill: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PairupK {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SpottedK {
    pub on: bool,
    pub leash_m: f64,
    pub slip_m: f64,
    pub lit_at: f64,
    pub slip_wait: f64,
    pub wary_m: f64,
    pub ear_m: f64,
    pub linger_secs: f64,
    pub brace_secs: f64,
    pub take_m: f64,
    pub take_reach: f64,
    pub take_secs: f64,
    pub fresh_secs: f64,
    pub caught_m: f64,
    pub last_secs: f64,
    pub step_m: f64,
    pub face_deg: f64,
    pub close_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct NightbreachK {
    pub def_lights: bool,
    pub sam_at: V3,
    pub sam_yaw: f64,
    pub squad: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RoomclearK {
    pub seed: f64,
    pub way: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct VerdictK {
    pub win_secs: f64,
    pub loss_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct UnseenK {
    pub pace_at: f64,
    pub linger: f64,
    pub min_pace: f64,
    pub low_pace: f64,
    pub gain: f64,
    pub cap: f64,
    pub wall_off: f64,
    pub wall_out: f64,
    pub wall_cross: f64,
    pub wall_end: f64,
    pub low_step: f64,
    pub walk_step: f64,
    pub cue_gap: f64,
    pub wall_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SeenwalkK {
    pub lit_at: f64,
    pub dear: f64,
    pub long: f64,
    pub less: f64,
    pub eye_step: f64,
    pub eye_y: f64,
    pub margin: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldfireK {
    pub lift_secs: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SlotsK {
    pub on: bool,
    pub table: crate::kit::SlotTable,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DestructionK {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BigslotK {
    pub on: bool,
    pub swings: f64,
    pub swing: f64,
    pub shells: f64,
    pub reach: f64,
    pub heft: f64,
    pub equip_w: f64,
    pub draw_w: f64,
    pub stow_w: f64,
    pub flow: f64,
}

pub fn page_aim() -> AimK {
    crate::knobs::AimK { guard: crate::knobs::AimKGuard { settle_secs: 1.3, range_ref: 8.0, near_k: 0.25, acquire_min: 0.55, unsettled_k: 3.0, on_deg: 18.0, lose_rate: 3.0, mem_secs: 0.6, jump_deg: 25.0, target_move_k: 0.36, target_move_cap: 0.26, rate_tau: 0.15, shooter_move_k: 0.1, feet_tau: 0.5, steady_pace: 1.3, stop_to_shoot: 1.2, moving_track_cap: 0.5, shoot_band: 14.0, brake_lead: 0.45 }, player: crate::knobs::AimKPlayer { settle_secs: 0.8, range_ref: 8.0, near_k: 0.25, unsettled_k: 2.2, base_stand: 0.012, base_crouch: 0.008, target_move_k: 0.2, target_move_cap: 0.16, rate_tau: 0.15, jump_deg: 25.0, shooter_move_k: 0.08, feet_tau: 0.6, steady_pace: 1.7, sprint_cap: 0.3, walk_cap: 0.7, moving_above: 0.3, crouch_k: 1.15, light_snap_px: 14.0 } }
}

pub fn page_trained() -> TrainedK {
    crate::knobs::TrainedK { settle_k: 0.6, spread_k: 0.7, react_cap: 0.45 }
}

pub fn page_body_light() -> BodyLightK {
    crate::knobs::BodyLightK { lit: 0.22, lit_near: 0.04, near_m: 2.0, far_m: 5.0, smoke: 0.3 }
}

pub fn page_guard_eye() -> GuardEyeK {
    crate::knobs::GuardEyeK { range: 16.0, half_deg: 58.0, near_m: 3.5, near_deg: 100.0, far_fix_at: 0.5, far_range: 26.0, far_lo: 0.4, far_hi: 0.8, far_k: 0.1 }
}

pub fn page_needs() -> NeedsK {
    crate::knobs::NeedsK { rate_k: 1.0, max_away: 1.0, announce_lead: 20.0, say_lead: 12.0, stretch_lead: 5.0, partner_m: 14.0, reply_after: 1.5, dwell: { let mut t = JsMap::new(); t.set(crate::needs::NeedKind::Coffee, 25.0); t.set(crate::needs::NeedKind::Smoke, 40.0); t.set(crate::needs::NeedKind::Water, 12.0); t.set(crate::needs::NeedKind::Stretch, 0.0); t }, min_wait: 2.0, min_trip_m: 4.0, after_cancel: 0.5, after_alert: 0.3, rest_secs: 90.0, floor_gap_secs: 30.0, pace_k: 1.0, stuck_secs: 6.0 }
}

pub fn page_whistle() -> WhistleK {
    crate::knobs::WhistleK { level: 0.35, reach_k: 8.0, horizon: 12.0, cooldown: 2.0, twice_window: 60.0, awareness: 0.45, react_t: 0.5, walk_floor: 0.32, glance_secs: 1.5, glance_max: 1.2, gain: 0.55 }
}

pub fn page_wet() -> WetK {
    crate::knobs::WetK { soak_per_sec: 0.16666666666666666, dry_per_sec: 0.04, bead_runoff_secs: 3.0, bead_density: 0.6, bead_size: 0.0075, trail_len: 0.11, rough_wet: 0.12, rough_dry: 0.55, darken: 1.0, darken_mul: 0.35, spec_gain: 1.0, drip_secs: 5.0, prints: 10.0, line_band: 0.12, print_fade_secs: 30.0 }
}

pub fn page_wind() -> WindK {
    crate::knobs::WindK { def: 1.0, vent_k: 1.0, storm_k: 1.0, door_k: 1.0, door_reach: 2.2, door_spread_deg: 22.0, door_open_rad: 1.2, gust_amp: 0.25, gust_period: 3.5, storm_gust_amp: 0.35, storm_gust_period: 2.5, ramp_up: 3.0, ramp_down: 4.0, smoke_relax: 3.0, sheet_wake: 0.35, sheet_kick: 2.0, debris_k: 0.15, near_m: 14.0 }
}

pub fn page_civ() -> CivK {
    crate::knobs::CivK { eye: crate::knobs::CivKEye { range: 12.0, half_deg: 50.0, near_m: 2.5, near_deg: 95.0 }, eye_busy: crate::knobs::CivKEyeBusy { range: 8.0, half_deg: 34.0, near_m: 2.0, near_deg: 80.0 }, eye_alarmed: crate::knobs::CivKEyeAlarmed { range: 14.0, half_deg: 62.0, near_m: 3.0, near_deg: 100.0 }, keen_k: 0.6, gun_k: 2.5, gun_m: 10.0, glimpse_at: 0.3, spot_at: 0.85, startle_secs: 4.0, startle_again_secs: 20.0, body_m: 10.0, freeze: [0.4, 0.8], run_speed: 3.2, walk_speed: 1.05, creep_speed: 1.2, report_m: 2.2, report_secs: 2.6, phone_secs: 6.0, guard_bonus_m: 4.0, exit_malus_m: 6.0, past_threat_m: 15.0, corner_m: 4.0, reflee_m: 6.0, lone_secs: 45.0, settle_secs: 75.0, comply_min: 3.0, scream_m: 16.0, see_m: 12.0, see_lit: 0.25, found_m: 3.0, found_secs: 1.2, outside_call_secs: 18.0, greet_m: 3.0, greet_pair_secs: 240.0, greet_floor_secs: 70.0 }
}

pub fn page_chaos() -> ChaosK {
    crate::knobs::ChaosK { def: 0.5, heat_tau: 2.0, cell: 2.0, t_decal: 1.0, t_paper: 1.0, t_burst: 1.5, t_puff: 1.5, t_knock: 4.0, t_tile: 9.0, bang_heat: 9.0, puffs_per_sec: 8.0, puffs_per_frame: 2.0, papers_per_sec: 6.0, knocks_per_sec: 4.0, tile_gap_secs: 2.2, tiles_per_encounter: 8.0, decals_per_sec: 12.0, paper_reach_m: 0.75, knock_reach_m: 0.45, sheets_per_stack: 26.0, puff: crate::knobs::ChaosKPuff { radius: 0.2, density: 3.5, ttl: 0.35, speed: 0.55, temp: 0.35, push: 6.0, expand: 8.0 }, tile: crate::knobs::ChaosKTile { half: [0.29, 0.0075, 0.29], albedo: [0.66, 0.66, 0.63] }, vel_low: 1.0, vel_high_k: 2.5, imp: crate::knobs::ChaosKImp { bang: crate::knobs::ChaosKImpBang { radius: 4.0, strength: 3.0, up: 2.0 }, impact: crate::knobs::ChaosKImpImpact { radius: 0.6, strength: 1.5, up: 0.5, bias_k: 0.6, debris_m: 0.5 }, thud: crate::knobs::ChaosKImpThud { radius: 1.0, strength: 0.8, up: 0.6, per_frame: 4.0 }, chaos_k: 1.5 }, loud: crate::knobs::ChaosKLoud { fixture_hit_m: 0.18, fixture_blast_m: 3.2, stun_fixture_m: 0.6, fixtures_per_encounter: 10.0, fixture_gap_secs: 0.8, blast_dust_floor: 1.2, blast_dust_puffs: 2.0, heat_k: 2.0, burst_queue: 32.0, burst_secs: 3.0 }, pall: crate::knobs::ChaosKPall { per_window: 6.0, window_secs: 4.0, slack: 8.0, clear: 1.8, clear_y: 1.7, min_radius: 0.5, max_gain: 2.0, wall: crate::knobs::ChaosKPallWall { near: 4.0, far: 3.0, radius: 0.45, density: 0.8, ttl: 1.5, speed: 1.6, temp: 0.25, push: 5.0, expand: 1.2, spread: 0.9, out: 0.5, dust: [36.0, 24.0], dust_life: 4.5, dust_size: 0.6, dust_speed: 1.2 }, burst: crate::knobs::ChaosKPallBurst { ring: 5.0, core_gain: 1.3, radius: 0.42, density: 0.85, ttl: 1.2, speed: 1.4, temp: 0.3, push: 5.0, expand: 1.5, spread: 0.6, dust: 48.0, dust_life: 3.8, dust_size: 0.5, dust_speed: 1.8, smoke: [0.34, 0.33, 0.32] } }, mess: crate::knobs::ChaosKMess { stand_y: 1.05, clear_y: 1.7, under: 0.06, min_radius: 0.28, crowd: 0.8, roof_y: 2.2, burst: crate::knobs::ChaosKMessBurst { core: 1.15, lobes: 6.0, reach: 1.25, lobe: 0.95, strength: 0.8 }, wall: crate::knobs::ChaosKMessWall { out: 0.55, near: crate::knobs::ChaosKMessWallNear { core: 1.0, lobes: 4.0, reach: 1.1, lobe: 0.85, strength: 0.7 }, far: crate::knobs::ChaosKMessWallFar { core: 1.2, lobes: 5.0, reach: 1.4, lobe: 0.95, strength: 0.9 } }, foot: crate::knobs::ChaosKMessFoot { out: 0.32, radius: 0.5, per_metre: 0.25, strength: 0.5 }, top: crate::knobs::ChaosKMessTop { lo: 0.08, hi: 1.4, strength: 0.75 }, rubble: crate::knobs::ChaosKMessRubble { least: 0.035, grow: 1.5, min: 0.09, max: 0.42, crowd: 0.7, strength: 0.85 } } }
}

pub fn page_ring() -> RingK {
    crate::knobs::RingK { speed: 30.0, gravity: 9.81, drag: 0.4, radius: 0.04, life: 1.5, substeps: 4.0, ko_below_neck: 0.07, stagger: 1.5, stagger_fire_cd: 1.5, stagger_aware: 0.92, stagger_back: 4.0, launch_noise: 0.45, clack_noise: 0.3, ko_noise: 0.3, hit_noise: 0.35, prop_j: 3.0, restitution: 0.2, keep_tangent: 0.25, spent_speed_max: 5.0, solve_iters: 2.0 }
}

pub fn page_launcher() -> LauncherK {
    crate::knobs::LauncherK { rings: 5.0, cd: 0.9 }
}

pub fn page_he40() -> He40K {
    crate::knobs::He40K { speed: 76.0, gravity: 9.81, drag: 0.12, radius: 0.03, life: 2.5, substeps: 4.0, solve_iters: 2.0, arm_m: 8.0, standoff: 0.15, launch_noise: 0.7, dud_noise: 0.45, dud_hit_noise: 0.45, prop_j: 9.0, restitution: 0.15, keep_tangent: 0.3, spent_speed_max: 6.0 }
}

pub fn page_launcher40() -> Launcher40K {
    crate::knobs::Launcher40K { rounds: 3.0, cd: 2.6 }
}

pub fn page_frag() -> FragK {
    crate::knobs::FragK { issue: 2.0, fuse: 3.0, order_fuse: 2.4, squad: 2.0, order_key: String::from("KeyG"), order_in: 3.4, lethal_m: 4.0, wound_m: 8.0, destr_m: 2.5, wound_kill: 0.6, stagger: [3.2, 1.2], lift: 0.2, glass_m: 4.0, glass_power: 1.5, pane_energy: 0.0, see_m: 9.0, hear_m: 5.0, react: [0.25, 0.5], react_jit: 0.25, shout_m: 9.0, flee_m: 9.0, run_speed: 3.4, flee_keep: 1.0, flee_apart_m: 1.0, flee_lane_m: 0.9, down_shove: 2.4, flee_door_open: 1.0, flee_door_in: 1.0, flee_door_dip: 0.6, flee_door_mouth: 0.9, flee_door_leaf: 0.3, flee_door_via: 0.9, flee_door_via_m: 0.45, flee_door_new: 30.0, flee_door_most: 2.0, flee_door_try: 4.0, flee_door_nodes: 1500.0, def_share: 3.0, def_max: 3.0, def_fuse: 4.0, def_wind: 1.0, def_near: 3.5, def_far: 13.0, def_past: 2.0, def_fresh: 4.0, def_blocked: 1.2, def_bunch: 2.5, def_bunch_secs: 0.5, def_gap: 14.0, def_own_m: 6.0, def_scatter: [0.6, 2.2], cue_m: 12.0 }
}

pub fn page_shotgun() -> ShotgunK {
    crate::knobs::ShotgunK { spots: alloc::vec![[0.45, 1.0], [0.25, 1.1], [0.05, 1.2], [-0.25, 1.3], [-0.5, 1.3]], clear: 0.55, hinge: alloc::vec![[0.45, 1.0], [0.45, 1.2], [0.65, 1.2]], mark_reach: 1.8, line: 0.45, own_beat: 0.15, own_slack: 0.25, lock_in: 0.2, lock_h: 1.0, shells: 2.0, aim_cap: 2.5 }
}

pub fn page_glass() -> GlassK {
    crate::knobs::GlassK { hear_m: 30.0, hear_level: 0.85, bang_m: 2.5, bang_cracks: 2.0, r_min: 0.075, r_max: 0.2, crack_gap: 0.045, live_gap: 0.2, ear_off: 0.55 }
}

pub fn page_glassbreak() -> GlassbreakK {
    crate::knobs::GlassbreakK { max_sites: 8.0, break_e: crate::knobs::GlassbreakKBreakE { annealed: 3.0, tempered: 1.0 }, punch_e: 0.5, run_single_e: 2.5, run_star: 2.0, run_ring_secs: 0.03, crack_gap: 0.25, rho: { let mut t = JsMap::new(); t.set(crate::knobs::GlassbreakKRhoKey::Annealed, [0.03, 0.07, 0.14, 0.5]); t.set(crate::knobs::GlassbreakKRhoKey::Laminated, [0.018, 0.05, 0.11, 0.22]); t }, radials: { let mut t = JsMap::new(); t.set(crate::knobs::GlassbreakKRadialsKey::Annealed, [9.0, 4.0]); t.set(crate::knobs::GlassbreakKRadialsKey::Laminated, [10.0, 3.0]); t }, merge: 0.45, rz_min: 0.05, think: 0.2, stagger_per_m: 0.18, stagger_jit: 0.12, hang_area: 0.12, grip: crate::knobs::GlassbreakKGrip { side: 2.0, top: 1.0, wedged: 0.25 }, stand_ny: 0.7, stand_foot: 0.3, balance_tol: 0.02, blast_m: 2.6, bang_power: 0.6, tooth_area: 0.03, tooth_k: 0.8, blast_run_k: 0.2, blast_dice_k: 0.45, blast_sheet_k: 0.75, rake_open: 0.5, rake_m: 0.5, dice_grain: 0.012, clump_m: 0.12, dice_hang: 0.09, dice_speed: 4.2, clumps_per_frame: 40.0, releases_per_frame: 8.0, out: 0.7, spin: 5.0, blast_out: 5.0, shard_albedo: [0.72, 0.8, 0.82], chip_color: [0.8, 0.88, 0.9], chips_per_m2: 90.0, chips_min: 8.0, dice_chips_per_m2: 260.0, crumb_every: 12.0, gust_speed_k: 0.6, walk_clear_h: 1.9 }
}

pub fn page_glasspen() -> GlasspenK {
    crate::knobs::GlasspenK { energy: { let mut t = JsMap::new(); t.set(String::from("fn2000"), 1.0); t.set(String::from("mk18bare"), 1.0); t.set(String::from("g19bare"), 0.6); t.set(String::from("ak74m"), 1.0); t.set(String::from("akms"), 1.1); t.set(String::from("pkm"), 1.3); t.set(String::from("aks74u"), 0.9); t.set(String::from("fiveseven"), 0.6); t.set(String::from("guard9mm"), 0.5); t.set(String::from("m870"), 0.3); t.set(String::from("m1014"), 0.3); t.set(String::from("mp7"), 0.6); t }, by_class: { let mut t = JsMap::new(); t.set(String::from("rifle"), 1.0); t.set(String::from("carbine"), 0.9); t.set(String::from("pistol"), 0.5); t }, default_energy: 0.5, cost: { let mut t = JsMap::new(); t.set(crate::glasspen::PaneType::Annealed, 0.1); t.set(crate::glasspen::PaneType::Tempered, 0.16); t.set(crate::glasspen::PaneType::Laminated, 0.42); t }, ref_thick: 0.012, oblique_max: 3.0, graze_cos: 0.17, min_energy: 0.12, max_panes: 4.0, deflect_deg: 5.0, bend_k: 0.5, hole_r: 0.04, hole_k: 0.25, holes: 24.0, shell_r: 0.04, stack_eyes: 3.0, stack_energy: 1.0, stack_least_k: 0.75, fracture_k: 2.0 }
}

pub fn page_shards() -> ShardsK {
    crate::knobs::ShardsK { mid_per_m2: 140.0, mid_min: 2.0, mid_max: 18.0, mid_size: 0.02, mid_life: 60.0, fine_per_m2: 520.0, fine_min: 6.0, fine_max: 70.0, fine_size: 0.0045, fine_life: 22.0, back_share: 0.18, hit_fine_min: 16.0, hit_back_share: 0.4, spall_out: 10.0, spall_back: 6.0, dice_per_m2: 520.0, dice_min: 6.0, dice_max: 160.0, dice_life: 45.0, dice_fine_share: 0.35, crater_per_m2: 20000.0, crater_max: 48.0, exit_fines: 14.0, exit_speed: 4.0, speed: 1.0, live_cap: 4000.0, cap_floor: 3.0, over_cap: 1.25, press_life: 0.3, press_from: 0.35, recs_per_frame: 24.0, defer_max: 96.0, plate_min_area: 0.012, plate_share: 0.6, child_min_area: 0.004, shatter_v0: 1.6, shatter_v1: 4.0, rest_crumbs: 12.0, crumb_size: 0.009, cut_edge: 1.0, cut_see: 0.7, cut_rim: 1.25, cut_body: 0.1, cut_see_rest: 0.45, cut_rest_body: 0.3, max_gen: 2.0, contact: crate::knobs::ShardsKContact { keep_floor: 0.72, keep_wall: 0.85, rest: 0.3 }, sounds_per_frame: 3.0, pending_max: 48.0, sound_gap: { let mut t = JsMap::new(); t.set(String::from("glassTinkle"), 0.07); t.set(String::from("glassShatter"), 0.11); t }, sound_thicken: 0.5, sound_owed_secs: 0.6, level: 0.3, event_gap: 0.4, crunch_full: 0.012, crunch_gain: 0.6, crunch_add: 0.12, grind: 0.9 }
}

pub fn page_hold() -> HoldK {
    crate::knobs::HoldK { regrab: 0.9, bare_grip: 0.9, bare_arch: -11.0, bare_elbow: 0.6, yank: crate::knobs::HoldKYank { arch: -5.0, sink: 0.09, in_: 0.07, hold: 0.05, out: 0.32 }, lunge: crate::knobs::HoldKLunge { dip: 0.06, back: 0.035, t: [0.035, 0.03, 0.1] }, sway: crate::knobs::HoldKSway { m: 0.014, secs: 1.9, twist: 2.4, his: 1.5, in_: 0.6, out: 8.0 }, apart: crate::knobs::HoldKApart { side: 7.0, tip: 8.0 }, catch: crate::knobs::HoldKCatch { via: [0.17, 1.49, 0.21], over: [0.08, 1.465, 0.38], at: [0.42, 0.74], arm_in: 0.3, ready_out: [0.0, 0.3], jolt: crate::knobs::HoldKCatchJolt { t: [0.07, 0.13, 0.24, 0.36], fwd: 12.0, head: -16.0, clav: 6.0, dip: 0.03 }, fling: crate::knobs::HoldKCatchFling { t: [0.24, 0.47], land: [0.73, 0.86], to: alloc::vec![[0.44, 1.08, 0.14], [-0.36, 1.27, 0.08]], drift: [0.03, 0.05, -0.04], arm_in: [0.05, 0.2] }, sink: 0.12, hips: 0.13, heels: [0.17, 0.27], seat: [0.07, 0.05, 0.06], seat_lean: 7.0, brace: [0.15, 0.12], brace_lift: 0.06, yank_sit: 0.5, feet: alloc::vec![[0.0, 0.17], [0.15, 0.36]], lift: 0.07, sam_feet: alloc::vec![[0.04, 0.2], [0.0, 0.12]], sam_lift: 0.05, settle: 0.4 }, lay_ahead: 0.9, hit: crate::knobs::HoldKHit { t: [0.0, 0.04, 0.07, 0.22], hips: 0.15, dip: 0.03, head: 1.0, heave: 12.0, sam_in: 0.05, sam_dip: 0.06 }, first: crate::knobs::HoldKFirst { take: 0.16, drop: 0.3, catch: 0.26 }, strip: crate::knobs::HoldKStrip { take: 0.26, out: 0.35, drop: 0.41, follow: 0.1, knife: 0.7, home: 0.95, free: 0.36, up: 0.68, meet: [-0.36, 1.15, 0.05], torn: [-0.47, 1.06, -0.05], over: [0.0, 0.035, -0.05], wrench: [-0.15, -0.1, -0.2], low: [-0.66, 1.2, -0.08], yaw: -16.0, rap: [0.02, 0.08, -0.03], hold: [0.08, 0.02], pitch: [12.0, 25.0, 15.0], rest: 0.05, push: [1.3, 1.05], fall: 0.311, spin: 5.0 }, dist: 0.27, secs: crate::knobs::HoldKSecs { grab: 0.45, choke_hold: 0.35, choke: 2.6, whip: 0.6, swap: 0.3, kill: 1.3 }, throat: [-0.11, 1.44, 0.35], via_l: [0.38, 1.42, 0.22], over_l: [0.16, 1.52, 0.33], tight: [-0.08, 1.45, 0.33], lean: 8.0, choke_lean: -4.0, head_pitch: 6.0, brace: 2.5, knife: crate::knobs::HoldKKnife { hand: [-0.165, 1.555, 0.36], blade: [1.0, 0.0, 0.12], back: [0.0, 0.28, 0.96], via: [-0.4, 1.5, 0.04], off_hand: [-0.24, 1.45, 0.26], off_blade: [-0.35, 0.75, 0.56], off_back: [-0.9, -0.3, 0.0], off_secs: 0.4, side: -8.0, head_yaw: 19.0, clav: 5.0 }, rest_l: [0.14, 1.47, 0.04], claw_l: [0.09, 1.39, 0.13], raised_r: [-0.36, 1.53, 0.06], via_up: [-0.38, 1.2, 0.12], claw_r: [-0.12, 1.38, 0.15], hang: alloc::vec![[0.27, 0.98, 0.14], [-0.27, 0.98, 0.14]], arch: -5.0, chin_up: -9.0, aside: 8.0, clav_up: 8.0, loll: 26.0, loll_roll: 11.0, lower: crate::knobs::HoldKLower { from: 0.6, by: 1.0, up: 7.0, sink: 0.4, fwd: 0.05, hip: 8.0, fold: 20.0, hang: [0.56, 0.26], sam_sink: 0.3, sam_fwd: 0.06, sam_knees: 0.55, sam_lean: 38.0, sam_look: 0.45, sam_arm: [0.5, 0.22] }, kill: crate::knobs::HoldKKill { wind: 0.21, hit: 0.3, out: 0.48, go: 0.46, after: 7.0, mouth: [0.07, 1.6, 0.36], chest: [-0.1, 1.28, 0.36], collar: [0.14, 0.98, 0.5], set: [0.06, 1.48, 0.29], across: [-0.2, 1.48, 0.33], wide: [-0.62, 1.4, 0.04], low: [-0.34, 1.05, 0.2], wide_blade: [0.55, -0.15, 0.8], wide_back: [-0.2, 0.95, 0.2], lean: [-12.0, 20.0], yaw: [16.0, -34.0], sam_sink: 0.22, look: 24.0, reach: alloc::vec![[0.28, 1.28, 0.32], [-0.3, 1.3, 0.34]], clutch: alloc::vec![[0.05, 1.41, 0.1], [-0.05, 1.4, 0.11]], chin: -27.0, arch: -12.0, jolt: [-12.0, -7.0], sink: 0.34, fwd: 0.08, hip: 8.0, fold: 36.0, hang: [0.2, 0.2] }, beats: crate::knobs::HoldKBeats { clamp: [0.03, 0.14], off: [0.41, 0.6], tip_by: [0.5, 0.62], tip: 32.0, back: 7.0, after: 2.6, lower: crate::knobs::HoldKBeatsLower { from: 0.58, by: 0.85, up: 7.0, sink: 0.4, fwd: 0.05, hip: 8.0, fold: 38.0, hang: [0.56, 0.26], sam_sink: 0.38, sam_fwd: 0.06, sam_knees: 0.55, sam_lean: 48.0, sam_look: 0.45, sam_arm: [0.52, 0.29] } }, haul: crate::knobs::HoldKHaul { in_: [0.02, 0.14], out: [0.38, 0.52], down: 7.0, sam_lean: -26.0, sam_arm: [0.07, 0.08], lift: 0.09, back: -0.08, recline: -14.0, arch: -4.0, chin: -6.0, feet: [0.11, 0.18], toes: 30.0, kick: [0.07, 0.05] }, sag: [0.02, 0.06], breath: crate::knobs::HoldKBreath { hz: 0.7, pitch: 1.6, clav: 2.2 }, struggle: crate::knobs::HoldKStruggle { rise: 0.1, fade_from: 0.35, fade_to: 0.92, yaw: 7.0, roll: 6.0, head: 9.0, claw: 0.035, dip: 0.018, hz: [2.3, 3.1, 1.9] }, gun: crate::knobs::HoldKGun { hand: [-0.2, 1.53, 0.15], bore: [0.21, -0.1, 0.97], out_hand: [-0.24, 1.53, 0.17], kick_pitch: 6.0, kick_back: 0.03, via_a: [-0.42, 1.24, 0.06], via_b: [-0.4, 1.5, 0.02], via_bore: [0.1, 0.6, 0.8], wind_hand: [-0.36, 1.68, -0.04], wind_bore: [-0.15, 0.8, 0.58], hit_hand: [-0.1, 1.62, 0.15], hit_bore: [0.5, 0.3, 0.8], thru_hand: [-0.06, 1.5, 0.2], lean: 5.0, side: -8.0, head_yaw: 19.0, head_pitch: 4.0, wind_yaw: -8.0, hit_yaw: 5.0, clav: 7.0, hands_up: alloc::vec![[0.27, 1.43, 0.31], [-0.34, 1.41, 0.29]], aside: 13.0, away_yaw: 6.0, chin_up: -6.0, duck: crate::knobs::HoldKGunDuck { yaw: 5.0, roll: 9.0, pitch: 10.0, hunch: 6.0, secs: 0.45 }, knock: crate::knobs::HoldKGunKnock { yaw: 24.0, roll: 16.0, pitch: 18.0 } } }
}

pub fn page_suppression() -> SuppressionK {
    crate::knobs::SuppressionK { near_r: 1.8, w_near: 0.4, hit_r: 2.5, w_hit: 0.3, min_travel: 3.0, half_life: 1.1, cap: 1.0, kick_decay: 8.0, down_accrual: 0.5, friendly_k: 0.5, keen: { let mut t = JsMap::new(); t.set(String::from("steady"), 1.0); t.set(String::from("nervy"), 1.25); t.set(String::from("hard"), 0.8); t }, flinch: 0.3, pinned: 0.6, release: 0.45, head_down: [0.9, 1.5], flinch_delay: 0.25, head_down_delay: 0.6, guard_spread: 0.08, gap_k: 1.0, standoff_pace_k: 0.5, displace_after: 4.0, displace_min: 2.0, displace_max: 4.5, cover_hold: 3.0, displace_cooldown: 6.0, displace_cap_secs: 6.0, displace_pace: 1.1, displace_crouch: 0.55, src_memory_secs: 4.0, bark_secs: 6.0, floor_bark_secs: 1.5, pair_secs: 2.0, floor_line_secs: 20.0, sam_spread: 0.07 }
}

pub fn page_rifle_spread() -> RifleSpreadK {
    crate::knobs::RifleSpreadK { still: 0.01, moving: 0.028, move_at: 0.3, crouch_k: 0.7, brace_k: 0.45 }
}

pub fn page_footfall() -> FootfallK {
    crate::knobs::FootfallK { travel: 0.25, owe: 0.5 }
}

pub fn page_twogears() -> TwogearsK {
    crate::knobs::TwogearsK { crouch: 1.45 }
}

pub fn page_crouchrun() -> CrouchrunK {
    crate::knobs::CrouchrunK { pace: 2.5, loud: 0.34 }
}

pub fn page_runready() -> RunreadyK {
    crate::knobs::RunreadyK { fwd: 0.24, up: -0.185, pitch: -50.0, tuck: 0.6 }
}

pub fn page_runarms() -> RunarmsK {
    crate::knobs::RunarmsK { at: 0.05, keep: 0.3, mid: 0.04, swing: 0.12, pump: 1.8, lift: 0.14, pole: [0.4, -0.45, -0.8], pole_w: 0.85 }
}

pub fn page_samlook() -> SamlookK {
    crate::knobs::SamlookK { near_m: 8.0, full_m: 5.5, back_deg: 120.0, keep_deg: 140.0, notice_m: 3.0, swap_m: 0.75, slip_from: 2.6, slip_by: 1.2, back_cos: [-0.45, -0.1] }
}

pub fn page_dash() -> DashK {
    crate::knobs::DashK { stride: 0.3, drop: 0.03, lean: 7.0, rate: 7.0 }
}

pub fn page_watch() -> WatchK {
    crate::knobs::WatchK { yaw: 72.0, down: 28.0, up: 22.0, share: [0.14, 0.34, 0.52], rate: 6.0, w_rate: 5.0, lean: 9.0, drop: 0.035, away: 0.03, arm_out: 0.32, arm_down: 0.42, arm_w: 0.7, arm_side: [0.2, 0.55], slip_rate: 5.0, squeeze: 0.6 }
}

pub fn page_stalk() -> StalkK {
    crate::knobs::StalkK { cursor_m: 1.1, begin_cos: -0.25, out_cos: -0.05, station_m: 0.55, grab_m: 0.95, pace: 1.6, close_min: 0.3, ease_up: 4.0, ease_down: 9.0, catch_secs: 0.3, catch_m: 0.75, catch_long: 1.1, catch_stride: 1.5, yank: 0.28, yank_room: 0.4, corner_m: 1.35, corner_deg: 110.0, jolt: 0.6, jolt_face: 50.0, jolt_secs: 0.6, corner_see: 3.2, corner_reach: [0.25, 0.55], corner_coil: 0.3, corner_rise: 0.24, key_m: 0.8, low_from: 2.6, low_by: 1.1, low_k: 0.35, low_drop: 0.06, low_back: 0.03, low_lean: 6.0, low_hands: 1.0, low_spring: 18.0, low_wind: 0.12, wide_by: 0.35, heft_twist: 20.0, heft_swing: 0.11, heft_side: 0.04, heft_lean: 4.0, heft_dip: 0.02, heft_surge: 1.0, heft_out: 0.18, heft_down: 0.4, heft_fwd: -0.2, heft_lead: 0.0, heft_pole: [0.6, 0.0, -0.8], orbit_k: 3.0, orbit_edge: 0.05, orbit_snap: 0.3, low_begin_m: 4.2, low_out_m: 5.0 }
}

pub fn page_overhear() -> OverhearK {
    crate::knobs::OverhearK { path_m: 9.0, carry: 0.5 }
}

pub fn page_score() -> ScoreK {
    crate::knobs::ScoreK { seen: 12.0, seen_up: 4.0, alarm: 6.0, heightened: 5.0, lockdown: 8.0, body: 6.0, kill: 10.0, ko: 1.0, shot_near: 2.0, shot_near_cap: 12.0, light: 1.0, light_cap: 4.0, hit: 4.0, down: 25.0, load: 3.0, civ_reported: 15.0, civ_ko: 10.0, civ_killed: 40.0, device: 2.0, device_cap: 8.0, par_secs: 1200.0, per_min: 1.0, time_cap: 10.0 }
}

pub fn page_badge() -> BadgeK {
    crate::knobs::BadgeK { lights_out_n: 3.0, speedrun_secs: 240.0 }
}

pub fn page_ckpt() -> CkptK {
    crate::knobs::CkptK { quiet_secs: 90.0, aware_max: 0.4, debounce_secs: 2.0, min_gap_secs: 10.0, pending_secs: 25.0, body_hide_m: 1.5, flash_secs: 2.6, regret_secs: 8.0 }
}

pub fn page_fog() -> FogK {
    crate::knobs::FogK { dim: 0.4, desat: 0.0, fov_deg: 360.0, near_m: 2.0, cone: false, cone_deg: 120.0, cone_near_m: 1.5, cone_soft_deg: 12.0, cone_near_soft_m: 0.6, own: false, call_secs: 10.0, call_gap: 3.0, call_known: 3.0, smoke: 0.3, linger: 0.25, top_pad: 0.3, squad_doors_m: 2.0, squad_fire_secs: 1.0, flat: 0.12, lit: 1.0, air: true, eyes: 7.0, kept: true, full_fill: 0.0, edge: 0.5, lit_fill: 0.5, lit_ao: 0.75, lit_ao_m: 1.5, lit_cut: 0.006, last_seen: 0.0 }
}

pub fn page_moves() -> MovesK {
    crate::knobs::MovesK { on: 1.0, recoil: 1.0, toss: 1.0, dazzle: 1.0, flinch: 1.0, gait: 1.0, fall: 1.0, squeeze: 1.0, reload: 1.0, bound: 1.0, clutch: 1.0, entry: 0.0, entry_eyes: 0.0, fold: 0.0, snap: 1.0, steady: 1.0, steady_run: 1.0 }
}

pub fn page_mv() -> MvK {
    crate::knobs::MvK { recoil: crate::knobs::MvKRecoil { back: [0.06, 0.04], back_tau: [0.09, 0.055], climb: [0.9, 0.4], climb_tau: [0.2, 0.1], yaw: [1.3, 0.35], rock: [3.4, 1.5], shoulder: [5.0, 2.6], rock_tau: [0.16, 0.085], head: [2.2, 0.6] }, flinch: crate::knobs::MvKFlinch { amp: [1.0, 0.38], dur: [0.7, 0.42], pitch: 15.0, head: 14.0, away: 11.0, drop: 0.055, side: 0.03 }, hit: crate::knobs::MvKHit { dur: 0.6, pitch: 17.0, yaw: 19.0, roll: 5.0, shove: 0.045, drop: 0.025 }, dazzle: crate::knobs::MvKDazzle { amp: [1.0, 0.6], hunch: 21.0, away: 24.0, roll: 5.0, head: 20.0, head_away: 22.0, drop: 0.065, back: 0.04, sway_x: 0.022, sway_z: 0.016, rate_in: 11.0, rate_out: 4.5, hand_drop: 0.038, hand_out: 0.105, hand_tilt: 0.05, hand_roll: 0.6, hand_pole: [0.45, -1.0, 0.35] }, gait: crate::knobs::MvKGait { drop: 0.035, aim_drop: 0.025, bob: 0.65, lean: 5.0, aim_lean: 5.0, elbow_in: 0.42, slouch_sway: 5.5, slouch_roll: 2.2, back: 4.5, chin: 5.0, wing: 0.85 }, snap: crate::knobs::MvKSnap { in_: 0.5, ease: 0.2, cover: [0.5, 0.95], under: 30.0 }, steady: crate::knobs::MvKSteady { roll: 0.7, sway: 0.3, feet: 0.7, knee: 0.5, knee_low: 0.5, deep: alloc::vec![0.03, 0.05], knee_out: 0.25, rise: 0.07, reach: 0.95, rise_rate: 8.0 } }
}

pub fn page_turn() -> TurnK {
    crate::knobs::TurnK { body_share: 0.5, green: 0.6, lead: 0.6, lead_rate: 16.0, jump: 0.75, aim: 11.0, aim_acc: 150.0, body: 7.0, body_acc: 60.0, on: 1.0 }
}

pub fn page_doorpath() -> DoorpathK {
    crate::knobs::DoorpathK { at: 0.52, out_m: 0.7, in_m: 0.7, look: 0.5, leaf_clear: 0.45, tip_m: 0.75, gap_m: 0.0, short_m: 0.6, cap: 2.5, move_m: 0.0, apart_m: 0.7, body_m: 0.0, body_off: 0.6, body_move: 1.2 }
}

pub fn page_walls() -> WallsK {
    crate::knobs::WallsK { wall_max_thick: 0.17, wall_min_top: 2.2, wall_min_len: 0.5, col: 0.5, sill: 0.7, tough: { let mut t = JsMap::new(); t.set(String::from("drywall"), 6.0); t.set(String::from("wood"), 9.0); t.set(String::from("cardboard"), 3.0); t.set(String::from("fabric"), 4.0); t }, sill_k: 1.5, pistol_k: 0.5, cover_min_top: 0.45, cover_min_len: 0.4, cover_max_cells: 48.0, blast: crate::knobs::WallsKBlast { radius: 2.2, hole_k: 0.45, port_k: 0.8, cover_k: 0.7, door_k: 0.6, min_hole: 3.0, contact_m: 0.8 }, max_pieces: 512.0, max_changes: 256.0, chunks_per_cell: 2.0, fall_max_vol: 0.5, falls_per_change: 48.0, hung: crate::knobs::WallsKHung { deep: 0.1, long: 0.5, tall: 0.3 } }
}

pub fn page_wallcharge() -> WallchargeK {
    crate::knobs::WallchargeK { menu: crate::knobs::WallchargeKMenu::Page, pick_m: 0.06, height: 1.05, end_m: 0.75, slide: alloc::vec![0.0, 0.5, -0.5, 1.0, -1.0, 1.5, -1.5], set_off: 0.5, set_near: 0.9, sam_reach: 1.35, sam_shift: 0.35, sam_marker_m: 3.0, standoff: alloc::vec![2.6, 2.0], post_off: 0.55, file_gap: 0.9, back_m: alloc::vec![3.0, 3.9, 4.8], clear_m: 0.45, mate_m: 2.0, after_blow: 0.7, enter_cap: 16.0, sweep_secs: 1.5, mouth_in: 0.6, lane: 0.35, lane_watch: 3.0, pods: alloc::vec![[-1.3, 1.9], [1.3, 1.9], [-2.1, 1.0], [2.1, 1.0]], cost: 2.0, carried: None, hold_secs: 1.5, mate_line: 0.6, off_frame: 0.8, off_frame_pod: 0.5, outer_thick: 0.26, outer_min_len: 1.5, far_slack: 0.5, far_reach: 0.375, hung_m: 0.35, safe_standoff: [4.0, 6.0], safe_reach: [3.0, 5.0], safe_secs: [3.0, 1.0], safe_hide_m: 9.0, safe_walk_m: 16.0, safe_back_m: 9.0, fire_known_s: 30.0, fire_m: 30.0, fire_step: 0.5, fire_grain: 0.03, men_clear_in: 0.1, men_clear_s: 0.75, sam_secs: [2.0, 0.6], sam_sup: 0.5 }
}

pub fn page_wallshot() -> WallshotK {
    crate::knobs::WallshotK { pall: 0.75, stagger_secs: 3.5, stun_secs: [8.0, 4.0], room_m: 12.0, ours_secs: 2.0 }
}

pub fn page_wallkill() -> WallkillK {
    crate::knobs::WallkillK { near_m: 1.5 }
}

pub fn page_hardbreath() -> HardbreathK {
    crate::knobs::HardbreathK { chest_amp: 4.2, chest_rate: 0.9, chest_up: 2.5, chest_down: 12.0 }
}

pub fn page_castor() -> CastorK {
    crate::knobs::CastorK { decel: 0.4, visc: 0.45, spin: 1.2, start_v: 0.3, stop_v: 0.1 }
}

pub fn page_wallreact() -> WallreactK {
    crate::knobs::WallreactK { see_m: 18.0, fov_deg: 116.0, hear_blast_m: 26.0, hear_fire_m: 10.0, hear_fall_m: 14.0, tell_m: 12.0, tell_secs: 1.2, near_m: 2.6, retry_secs: 5.0, gone_max: 6.0, step_m: 3.5, run_m: 9.0, beyond_m: 2.5, ev_open: 1.6, ev_seen: 1.5, ev_fire: 1.0, forget: 0.12, hole_base: 0.45, port_base: 0.2, crowd: 0.35, sticky: 0.15, fresh_secs: 3.0, fresh: 3.0, watch_m: 16.0 }
}

pub fn page_coverfire() -> CoverfireK {
    crate::knobs::CoverfireK { stale_secs: 5.0, call_secs: 1.2, call_m: 14.0, behind_m: { let mut t = JsMap::new(); t.set(String::from("cover"), 3.0); t.set(String::from("wall"), 4.0); t }, max_layers: 3.0, max_rounds: 20.0, max_m: 30.0, min_m: 1.5, near_m: 1.0, slack_k: 1.5, slack: 3.0, give_secs: 20.0, lane: 0.6, cone_k: 0.03, past_m: 8.0, same_m: 1.5, linger_secs: 1.0, lead_secs: 0.8, lead_m: 1.5, low_y: 0.55, round_secs: 0.25, walk_pace: 3.0, walk_ask_secs: 1.5, line: String::from("he's behind that. going through it."), say_secs: 12.0 }
}

pub fn page_doorblast() -> DoorblastK {
    crate::knobs::DoorblastK { tear_m: crate::knobs::DoorblastKTearM { stun: 0.0 }, charge_k: 0.55, charge_r: 0.8, speed: [3.2, 6.5], up: 0.9, tip: 2.4, spin: 1.3, scrape: 4.0, slam: 0.4, slide: 12.0, bounce: 0.15, room: 0.6, land_level: 0.9 }
}

pub fn page_doorshot() -> DoorshotK {
    crate::knobs::DoorshotK { toll: 2.0, strips: 4.0, lengths: 3.0, uneven: 0.6, chunks: 11.0, fly: alloc::vec![1.0, 3.2], side: 1.1, up: alloc::vec![0.2, 1.8], spin: 9.0, tilt: alloc::vec![0.15, 0.45], topple: alloc::vec![2.0, 5.0], chips: alloc::vec![60.0, 24.0], chip_v: alloc::vec![4.0, 2.5], shed: 8.0, slivers: alloc::vec![22.0, 3.2], level: 0.8 }
}

pub fn page_doorthrow() -> DoorthrowK {
    crate::knobs::DoorthrowK { push_secs: 2.0, kick_secs: 4.0, startle: 1.0, wary: 0.5, min_speed: 0.9, hand_rate: 1.2, floor_rad: 0.6, rate: 9.0, slide: 1.5, again: 3.0, aware: 0.92 }
}

pub fn page_winbars() -> WinbarsK {
    crate::knobs::WinbarsK { per_round: 4.0, pieces: 3.0, uneven: 0.5, fly: [1.6, 3.4], rest_k: 0.45, up: [0.2, 1.3], side: 0.8, spin: 7.0, chips: 18.0, chip_v: 2.8, blast_k: 1.0 }
}

pub fn page_opening() -> OpeningK {
    crate::knobs::OpeningK { clear_m: 10.0, step_m: 0.5 }
}

pub fn page_handle() -> HandleK {
    crate::knobs::HandleK { look: 2.6, ajar: 1.35, reach: 0.95, handle_in: 0.14, far_reach: 1.5, push_in: 0.33, push_off: 0.85, push_thru: 0.35, stand_in: 0.3, stand_off: 0.6, thru_at: 1.2, thru_off: 0.75, body: 0.34, slack: 0.1, near: 0.05, turn: 0.35, clear: 1.05, wait: 1.1, press: 1.5, tool_post: 0.0, near_body: 0.38, near_post: 1.3, go_cap: 4.0, rest: 2.5 }
}

pub fn page_doormanners() -> DoormannersK {
    crate::knobs::DoormannersK { reach: 3.0, half: 0.45, throat: 1.25, gap: 1.05, gap_min: 0.7, brake: 8.0, patience: 1.1, rest: 2.0, beyond: 0.6, aside: 0.4, jamb_out: 0.6, jamb_past: 0.5, file_gap: 0.9, apart: 0.85, step: 1.4, round: 1.2, round_out: 0.85, own_m: 0.45, own_end_m: 2.5, leaf_stall_m: 0.12, leaf_stall_secs: 0.6, leaf_stall_reach: 0.6, leaf_stall_ahead: 0.3, leaf_stall_push: 1.5, sweep: 0.2, lane_ajar: 0.6, lane_m: 6.0, lane_out: 0.3, lane_flare: 0.12, lane_clear: 0.15, lane_apart: 0.7, cover_m: 2.0, cover_deep: 2.5, cover_look: 0.8, cover_quiet: 1.0, cover_cap: 8.0, lane_dear: 6.0, lane_strip: 0.7, lane_long: 1.6, funnel: 0.6, funnel_rest: 1.5, through: 0.95, past: 0.6, plane_deg: 30.0, rest_secs: 1.0, clear_m: 1.4, held_cap: 12.0 }
}

pub fn page_breaks() -> BreaksK {
    crate::knobs::BreaksK { share: crate::knobs::BreaksKShare { big: crate::knobs::BreaksKShareBig { thrown: 5.0, heap: 5.0, furniture: 8.0 } }, refuse_chips: [6.0, 8.0], sized_chips: [4.0, 14.0], sized_span: [0.05, 0.5], china_land: 3.5, china_max: 0.06, core_tint: 0.3, chip_tint: 0.5, heap: crate::knobs::BreaksKHeap { span: [0.15, 0.55], out: [0.0, 0.7], tip: [0.07, 0.24] }, max_standing_planes: 4.0, across_deg: [35.0, 55.0], loose_min: 0.08, kink: crate::knobs::BreaksKKink { deg: [25.0, 42.0], skew: 20.0, off: 0.4, min_long: 0.2, gap_secs: 0.5 }, steel: crate::knobs::BreaksKSteel { sheet: 0.008, kink: [9.0, 24.0], dents: [3.0, 5.0], dent: [0.025, 0.06], dent_most: 0.14, rim: 0.035, lip: 0.004, inside: 0.16 }, stud: crate::knobs::BreaksKStud { half: 0.02, depth_k: 0.7, foot: 0.13, albedo: [0.56, 0.57, 0.6], rough: 0.35, does: crate::scene_materials::DoesWord::UnnamedLightGrey } }
}

pub fn page_furn() -> FurnK {
    crate::knobs::FurnK { k: { let mut t = JsMap::new(); t.set(crate::furnblast::BlastSource::Wall, 1.0); t.set(crate::furnblast::BlastSource::Door, 0.7); t.set(crate::furnblast::BlastSource::Frag, 0.6); t }, fling: { let mut t = JsMap::new(); t.set(crate::furnblast::BlastSource::Wall, 1.6); t.set(crate::furnblast::BlastSource::Door, 1.2); t.set(crate::furnblast::BlastSource::Frag, 1.2); t }, shatter_m: 1.6, apart_m: 4.0, throw_m: 5.5, throw_v: 6.0, prop_break_m: 1.5, chunks_per_blast: 36.0, mouth_m: 0.45, merge_m: 1.5, mouth: crate::knobs::FurnKMouth { depth: 0.9, top: 2.3, vol: 3.0, chunks: 12.0, base_thick: 0.08 }, hung: crate::knobs::FurnKHung { len: 0.32, max: 4.0, strips: 2.0, under_m: 0.3, through: alloc::vec![7.0, 11.0], back: alloc::vec![3.5, 7.0], side: 4.5, up: alloc::vec![1.6, 3.0], spin: 24.0, chunks: 8.0, chips: alloc::vec![40.0, 24.0], chip_v: alloc::vec![5.0, 7.0], hole_m: 0.5 }, root: crate::knobs::FurnKRoot { min_top: 0.3, max_top: 2.0, max_long: 2.6, max_vol: 0.7, apart_max_vol: 0.6 }, rider: crate::knobs::FurnKRider { max_top: 2.3, max_vol: 0.3 }, touch: 0.04, row_m: 3.6 }
}

pub fn page_snowbog() -> SnowbogK {
    crate::knobs::SnowbogK { dead: 0.85, grip: 0.8, stick: 1.1, packed: 0.4, old: 0.7, top: 0.06, bank_m: 0.15, run: 2.0 }
}

pub fn page_rubble() -> RubbleK {
    crate::knobs::RubbleK { radius: 0.3, reach: 0.12, lead: 0.12, inside: 0.06, floor: 0.12, heavy: 12.0, walk: 0.35, base: 0.6, gain: 0.8, aside: 1.2, v_max: 2.2, off: 1.0, slide: 0.3, spin: 2.2, per_frame: 4.0, cooldown: 0.3, give_up: 8.0, give_up_secs: 3.0, scuff: 0.12 }
}

pub fn page_debris() -> DebrisK {
    crate::knobs::DebrisK { min_speed: 1.0 }
}

pub fn page_lightwall() -> LightwallK {
    crate::knobs::LightwallK { gap: 0.03 }
}

pub fn page_airdust() -> AirdustK {
    crate::knobs::AirdustK { diffuse: 1.5, settle: 0.12, step: 0.03333333333333333, recheck_secs: 3.0 }
}

pub fn page_wallsnap() -> WallsnapK {
    crate::knobs::WallsnapK { on: true, take_base: 0.1, take_per_m: 0.16, take_per_turn: 0.06, take_min: 0.12, take_max: 0.3, ease_pow: 2.0, yaw_pow: 3.0, reach_moving: 1.15, heading_min: 0.5, heading_bonus: 0.35, buffer: 0.15, dip: 0.035, nod: 4.0, hit_peak: 0.05, step_lift: 0.06, step_trail: 0.5, cam_room: 0.35, cam_side: 0.3, along_deg: 55.0, away_secs: 0.1, away_diag: 0.18, off_push: 0.7, up_secs: 1.2, up_rise: 0.16, bore: 0.9, lean_along: 0.6, lean_out: 0.18, lean_back: 0.14, lean_on_deg: 8.0, lean_off_deg: 2.0, lean_min_dist: 0.6, lean_max_deg: 100.0, lean_keep_deg: 15.0, lean_keep_dist: 0.2, lean_over_y: 1.9, lean_clear_m: 1.4, lean_gun: 0.42, lean_gun_step: 0.06, dwell_out: 0.06, dwell_back: 0.15, min_out: 0.35, after_shot: 0.4, press: true, press_secs: 0.18, press_flow: 0.2, press_near: 0.6, press_deg: 35.0, press_touch: 0.08, press_thing: 0.5, press_thing_at: 0.9, press_turn: 50.0, press_again: 0.6, press_after_shot: 1.0, voice_on: 0.35, voice_off: 0.5, slow_take: 2.0, slow_in: 11.0, slow_out: 12.0 }
}

pub fn page_gunwall() -> GunwallK {
    crate::knobs::GunwallK { down_m: 0.03, up_m: 0.12, lead_s: 0.2, at_m: 0.5, tuck_m: 0.04, tuck_lead_s: 0.12, fight_s: 5.0 }
}

pub fn page_close() -> CloseK {
    crate::knobs::CloseK { rmb_tap_secs: 0.2, rmb_loud: 0.15, rmb_wait_secs: 1.2, hands_low_neck_m: 1.15, mode_m: 3.0, mode_out_m: 3.6, dark_at: 0.1, lit_at: 0.3, touch_m: 0.55, reach: crate::knobs::CloseKReach { behind: 1.05, side: 0.95, front: 0.85, seat: 1.2, seat_front: 0.95 }, side_from_deg: 50.0, strike_dist: 0.6, seat_dist: 0.72, noise: crate::knobs::CloseKNoise { behind: 0.3, side: 0.38, front: 0.5, seat: 0.3 }, alert_noise: 0.15 }
}

pub fn page_plan() -> PlanK {
    crate::knobs::PlanK { fresh_secs: 2.5, walk_dim: 0.6, rest_dim: 0.45, ring_m: 0.3, sector_m: 1.6, sector_deg: 24.0, sight_m: 14.0, reask_secs: 0.5, pair_bar_m: 0.55, others: 0.45 }
}

pub fn page_reload() -> ReloadK {
    crate::knobs::ReloadK { delay: 0.25 }
}

pub fn page_samstun() -> SamstunK {
    crate::knobs::SamstunK { fade: 0.6, own_seen: true }
}

pub fn page_recap() -> RecapK {
    crate::knobs::RecapK { secs: 8.0, after: 1.2, hz: 20.0, keep: 6.0, live_see_m: 25.0, live_saw_secs: 1.0, live_hear_m: 25.0, live_beat: 0.6, live_side_m: 8.0 }
}

pub fn page_aimweight() -> AimweightK {
    crate::knobs::AimweightK { rate: 11.0, smooth: 0.05, heft: 1.5, bare: 1.25, on_m: 0.3, reach: 18.0, launch_wait: 0.5 }
}

pub fn page_aimcost() -> AimcostK {
    crate::knobs::AimcostK { bare: 0.12, pad: 0.03, slack: 0.15, move_k: 2.0, stop_tau: 0.35, stop_bleed: 0.1, brace_feet: 0.5, bare_device: 0.16, plant_at: 1.2, flank_at: 0.3 }
}

pub fn page_friends() -> FriendsK {
    crate::knobs::FriendsK { reach_m: 8.0, past_m: 0.3, clear_m: 0.35, lead_s: 0.3, lead_max_m: 0.4, up_secs: 0.25, quiet_s: 5.0 }
}

pub fn page_friendlyfire() -> FriendlyfireK {
    crate::knobs::FriendlyfireK { cone_k: 0.75, body_m: 0.34, sure_m: 0.2, near_m: 0.6 }
}

pub fn page_lightswitch() -> LightswitchK {
    crate::knobs::LightswitchK { reach: 1.15, height: 1.2, beside: alloc::vec![0.85, 1.1, 0.6, 1.5, 2.0, 2.6], stand_m: 0.5, per_room: 2.0, apart_m: 4.0, scan_m: 4.0, guard_secs: 1.5, hand_secs: 0.3 }
}

pub fn page_carlamps() -> CarlampsK {
    crate::knobs::CarlampsK { dark: [0.035, 0.035, 0.04], fit: 0.004, tone: 0.004, twin: 0.03 }
}

pub fn page_wallskin() -> WallskinK {
    crate::knobs::WallskinK { on: true, thick: 0.04, off: 0.08, per_skin: 12.0, max_pieces: 256.0, box_slack: 64.0 }
}

pub fn page_fireline() -> FirelineK {
    crate::knobs::FirelineK { fire_s: 1.0, reload_s: 0.7, pace_s: 2.5, pace_k: 1.5, lapse_s: 2.5, spread: 0.06, clear_m: 0.25, short_m: 1.4, keep_m: 0.4, gap_m: 1.0, look_m: 6.0, cue_s: 0.35, max_s: 4.0, again_s: 2.0, way_more_m: 10.0, berth_m: 0.4, edge_m: 0.15, reach_m: 40.0 }
}

pub fn page_savetimeline() -> SavetimelineK {
    crate::knobs::SavetimelineK { manual: 5.0, auto: 5.0 }
}

pub fn page_handring() -> HandringK {
    crate::knobs::HandringK { on: true, hold_secs: 0.3, turn_max: 0.45 }
}

pub fn page_sprintdoor() -> SprintdoorK {
    crate::knobs::SprintdoorK { keep: 0.75, bash_vel: 9.0, lean: 0.45, lean_secs: 0.25, rip_pace: 0.3, rip_secs: 0.3, rip_vel: 6.0, rip_level: 0.55, kick_k: 0.25, kick_into: 2.5, kick_free: 0.8 }
}

pub fn page_teampace() -> TeampaceK {
    crate::knobs::TeampaceK { k: 0.8, follow_k: 1.3, behind_m: 1.0, fight_secs: 2.0, back_secs: 6.0, dash_m: 2.5, dash_seen: 0.5, dash_every: 0.25, dash_hot: 6.0, door_brisk: true, pacing: true, pace_near: 12.0, pace_gap: 1.0, pace_cos: 0.5, walk_aim_off: 0.25, walk_aim_on: 0.5, run_at: 2.7, accel: 6.0, step: 1.0, gap: 1.0 }
}

pub fn page_noshove() -> NoshoveK {
    crate::knobs::NoshoveK { ring: 0.6, back: 1.2 }
}

pub fn page_rails() -> RailsK {
    crate::knobs::RailsK { pan_speed: 11.0, pan_reach: 16.0, snap_px: 40.0, keep: 0.21, keep_speed: 9.0, keep_deep: 0.79, keep_up: 0.39, way_m: 6.0, after_secs: 8.0, after_max: 45.0, line_clear: true, mark_clear: true, card_beat_secs: 1.2, card_quiet: 2.0, card_max: 9.0, spare_secs: 6.0 }
}

pub fn page_doorforce() -> DoorforceK {
    crate::knobs::DoorforceK { sure: 1.0, keep: 0.3, fade_secs: 5.0, stale_secs: 6.0, gain: 4.0, hear_min: 0.03, shot: 1.0, handle: 0.6, seen: 1.5, shut_carry: 0.5, behind_m: 2.5, reach_m: 9.0, seen_secs: 3.0, heard_err_m: 0.4, chest_y: 1.15, near_leaf_m: 1.2, past_leaf_m: 0.6, rounds: 4.0, gap_secs: 1.5, kick_secs: 4.0, kick_secs_no_shot: 2.0, walk_secs: 6.0, rest_secs: 8.0 }
}

pub fn page_slotsetup() -> SlotsetupK {
    crate::knobs::SlotsetupK { settle: 0.35 }
}

pub fn page_glassreal() -> GlassrealK {
    crate::knobs::GlassrealK { hang: 0.1, blocks: 48.0, low_m: 0.7, door_m: 0.3, door_foot_m: 1.5 }
}

pub fn page_quietfog() -> QuietfogK {
    crate::knobs::QuietfogK { def: crate::quietfog::QuietRung::Team }
}

pub fn page_told() -> ToldK {
    crate::knobs::ToldK { rung: crate::told::ToldRung::Standard, guided: 10.0, standard: 4.0 }
}

pub fn page_kitanim() -> KitanimK {
    crate::knobs::KitanimK { sling: 0.0, sling_up: 18.0, sling_down: 4.0, early: 0.05, click_keep: 0.35, throw_keep: 1.0, plain_lock: 0.45, beats: crate::knobs::KitanimKBeats { pistol: crate::knobs::KitanimKBeatsPistol { ref_: 1.6, empty: crate::kitanim::BeatRow { dur: 1.6, out: 0.25, in_: 0.95, rack: Some(1.3), keep: None }, dry: crate::kitanim::BeatRow { dur: 1.45, out: 0.25, in_: 0.95, rack: None, keep: None }, tactical: crate::kitanim::BeatRow { dur: 2.0, out: 0.45, in_: 1.5, rack: None, keep: Some(0.95) } }, rifle: crate::knobs::KitanimKBeatsRifle { ref_: 2.4, empty: crate::kitanim::BeatRow { dur: 2.4, out: 0.4, in_: 1.45, rack: Some(1.95), keep: None }, dry: crate::kitanim::BeatRow { dur: 1.95, out: 0.4, in_: 1.45, rack: None, keep: None }, tactical: crate::kitanim::BeatRow { dur: 2.75, out: 0.55, in_: 2.1, rack: None, keep: Some(1.2) } } }, close_near: 1.1 }
}

pub fn page_kit() -> KitK {
    crate::knobs::KitK { stow: { let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, 0.3); t.set(crate::character::KitItem::Rifle, 0.54); t.set(crate::character::KitItem::Can, 0.26); t.set(crate::character::KitItem::Knife, 0.34); t }, reach: { let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, 0.17); t.set(crate::character::KitItem::Rifle, 0.28); t.set(crate::character::KitItem::Can, 0.16); t.set(crate::character::KitItem::Knife, 0.2); t }, draw: { let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, 0.26); t.set(crate::character::KitItem::Rifle, 0.43); t.set(crate::character::KitItem::Can, 0.2); t.set(crate::character::KitItem::Knife, 0.2); t }, away: 0.22, swap_k: 0.8, far: 0.28, apex_at: 0.85, sling_at: 0.55, join: 0.35, let_go: 0.3, apex: crate::knobs::KitKApex { at: [-0.03, 0.1, -0.19], fing: [0.35, -0.75, -0.55], back: [0.0, -0.2, -1.0], pole: [-1.0, 0.25, 0.0] }, elbow_min: 44.0, wrist_knee: 46.0, wrist_max: 66.0, roll_in: 0.5, over: [-0.03, 0.34, 0.22], up: [-0.14, 0.22, 0.14], top: crate::knobs::KitKTop { fing: [0.0, 1.0, 0.25], back: [-0.8, 0.0, 0.6] }, trunk: 6.0, belt: crate::knobs::KitKBelt { at: [-0.15, 0.04, 0.13], fing: [0.25, -0.9, 0.2], back: [-0.5, 0.0, 0.85], pole: [-1.0, -0.1, -0.5] }, holster_pole: [-0.7, 0.15, -1.0], rest: [0.03, -0.36, 0.2], arc: { let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, [-0.05, 0.0, 0.05]); t.set(crate::character::KitItem::Rifle, [-0.1, 0.2, 0.0]); t.set(crate::character::KitItem::Can, [-0.03, 0.0, 0.05]); t.set(crate::character::KitItem::Knife, [0.0, 0.0, 0.05]); t }, cross: [-0.16, 0.0, 0.1], front: crate::knobs::KitKFront { stow: 0.36, reach: 0.2, draw: 0.32, hand_at: 0.8, sling_at: 0.45, arc: [0.0, 0.02, 0.07], pole: [-0.75, -0.55, -0.4] }, nv: crate::knobs::KitKNv { hand: crate::knobs::KitKNvHand { dur: 0.92, reach: 0.38, out: 0.36, flip: [0.38, 0.52], up: [0.084, 0.178, 0.1], down: [0.086, 0.08, 0.12], fingers: [-0.75, 0.6, 0.1], back: [0.55, 0.1, 0.83], pole: [0.7, -0.6, 0.35], shrug: 8.0, curl: [0.45, 0.5, 0.55, 0.6], thumb: 0.35, rate: 4.2, away: 2.6, clear: 6.0 }, nod: crate::knobs::KitKNvNod { dur: 0.36, pre: 0.07, peak: 0.18, flip: [0.12, 0.22], on: [-5.0, 19.0], off: [7.0, -15.0], carry: 0.12 } }, fist: alloc::vec![0.62, 0.68, 0.72, 0.76], open: alloc::vec![0.26, 0.3, 0.34, 0.38], mag: crate::knobs::KitKMag { well: [0.085, -0.125, -0.005], strip: [0.09, -0.21, -0.02], slide: [0.05, 0.07, -0.02], pull: [0.05, 0.07, -0.085], pouch: [0.17, 0.03, 0.12], dump: [0.21, 0.0, -0.13], via: [0.31, 0.14, 0.1], pole: [0.75, -0.55, -0.3], pole_w: 0.7, wf: [-0.75, 0.55, 0.3], wb: [0.3, -0.75, 0.55], sf: [-1.0, -0.15, 0.1], sb: [0.0, 1.0, 0.0] }, close: crate::knobs::KitKClose { far: 0.4, hover: 0.045, bow: [-0.02, 0.01, 0.035], lead: [0.0, 0.0, 0.0], left: 0.3, left_at: 0.85, left_in: 0.24, left_out: 0.2, rate: 7.0, flex: 0.55, flex_amp: 0.1, drift: 0.008, regrip: 1.7, curl: 0.5 } }
}

pub fn page_throw_by() -> ThrowByK {
    crate::knobs::ThrowByK { toss: 3.0, toss_fast: 7.3, toss_in: 4.0, toss_min: 0.6, through: 0.07, toss_pivot: [-0.07, -0.46, 0.08], full: 10.0, turn_min: 0.4, arm_min: 0.5, off_min: 0.45, sit: 0.6, pivot: [-0.2, 0.06, 0.18] }
}

pub fn page_hands() -> HandsK {
    crate::knobs::HandsK { reach_secs: 0.3, lift_secs: 0.35, set_secs: 0.3, grip_m: 0.55, step_secs: 1.0, head_bias: 0.1, ahead: 0.35, ahead_head: 0.48, carry_feet: 0.42, carry_head: 0.46, slack_m: 0.4, hoist_secs: 1.0, lower_secs: 1.1, carry_step_secs: 2.4, carry_near_m: 0.35, carry_side: 0.2, carry_fwd: -0.02, carry_up: 1.56, carry_back: 1.35, carry_legs: 1.1, carry_pace: 0.6, carry_turn: 5.0, carry_ride: 0.85, lay_ahead: 0.62, door_reach: 0.12, hand_secs: 0.3, door_rate: 2.6, open_to: 1.5, clear_m: 0.06, step_m: 1.3, round_m: 1.8, hand_m: 0.5, in_m: 0.75, stall_secs: 0.5, aim_arc: 1.2, team_pre_m: 0.55, team_reach_m: 0.95, team_pace: 0.9, team_vel: 0.25 }
}

pub fn page_work() -> WorkK {
    crate::knobs::WorkK { pick_m: 0.45, pick_wide_m: 0.9, behind_m: 0.5, behind_panel_m: 0.09, stand_m: 0.5, buddy_m: 1.4, hand_secs: 0.45, breaker_secs: 0.7, fade_secs: 0.4, hand_m: 0.85, walk_m: 2.5 }
}

pub fn page_wand() -> WandK {
    crate::knobs::WandK { feed_secs: 1.5, stow_secs: 0.6, fov_deg: 90.0, blind_deg: 10.0, pan_rate: 1.6, eye_y: 0.1, past_m: 0.22, mark_secs: 15.0, shut_rad: 0.05, give_up_secs: 2.5, order_key: String::from("KeyV"), sweep_secs: 5.0, quick_secs: 0.4, quick_down: 0.9 }
}

pub fn page_sticky() -> StickyK {
    crate::knobs::StickyK { carry: 3.0, max: 3.0, live: 1.0, issue: 2.0, speed: 26.0, gravity: 9.81, drag: 0.5, radius: 0.03, life: 1.6, substeps: 4.0, solve_iters: 2.0, cd: 1.2, aim_y: 1.9, fov_deg: 110.0, off: 0.06, flat_y: 0.7, launch_noise: 0.3, stick_noise: 0.12, take_reach: 1.3, take_noise: 0.1 }
}

pub fn page_peeksight() -> PeeksightK {
    crate::knobs::PeeksightK { out: 0.3, off: 0.1, turn_deg: 90.0 }
}

pub fn page_gunslots() -> GunslotsK {
    crate::knobs::GunslotsK { near: 10.0, out: 0.7, lull: 1.6, back: 0.65 }
}

pub fn page_gunroles() -> GunrolesK {
    crate::knobs::GunrolesK { akms: 0.5, belt_lane: 15.0, belt_min: 5.0, belt_squad: 10.0, line_max: 40.0, pistol_near: 4.0, pistol_share: 0.5, pistol_max: 0.25, lane_k: 0.6, file_cost: 400.0 }
}

pub fn page_kittree() -> KittreeK {
    crate::knobs::KittreeK { lend: 5.0 }
}

pub fn page_squadthrow() -> SquadthrowK {
    crate::knobs::SquadthrowK { on: true, frag_roll_k: 0.04, again_secs: 1.2, shown_secs: 2.5, slack_m: 1.0, walk_m: 12.0, min_m: 1.5, max_m: 11.5, tries: 12.0, walk_secs: 16.0, turn_secs: 1.2, toss_secs: 0.5, lob_paces: alloc::vec![7.5, 10.0, 5.0], lob_bank_paces: alloc::vec![10.0, 7.5], lob_speed: 11.0, lob_near_m: 1.5, lob_up_m: 1.2, lob_walls: 12.0, lob_wall_m: 3.0, lob_wall_h: 0.9, lob_sims: 48.0, lob_cells: 10.0, settle_secs: 0.5, lob_steps: alloc::vec![0.008333333333333333, 0.025], duck_secs: 0.5, duck_throw_secs: 0.2, duck_cells: 16.0, duck_apart_m: 1.0, duck_min_m: 5.0 }
}

pub fn page_squad40() -> Squad40K {
    crate::knobs::Squad40K { near_m: 1.2, arm_slack: 0.5, reach_slack: 0.5, muzzle_ahead: 0.55, muzzle_up: 1.35, head_drop: 0.3, aim_rad: 0.2, aim_secs: 2.0, raise_secs: 0.35, after_secs: 0.4, shown_secs: 2.5 }
}

pub fn page_autopilot() -> AutopilotK {
    crate::knobs::AutopilotK { show_secs: 4.0 }
}

pub fn page_cmdring() -> CmdringK {
    crate::knobs::CmdringK { key: String::from("KeyB"), wedges: 5.0, flick_px: 24.0, hold_secs: 0.2, stay_hold_secs: 0.3, tap_secs: 0.15, edge_x: 150.0, edge_y: 100.0, man_px: 24.0, ref_: [1920.0, 1080.0], goal_half: true, twice: true, twice_secs: 0.4, twice_px: 24.0 }
}

pub fn page_torch() -> TorchK {
    crate::knobs::TorchK { lit_at: 0.2, off_k: 1.2, spill_m: 3.0, day_m: 12.0, ahead_m: 5.0, on_secs: 0.3, off_secs: 2.0, hold_secs: 1.0, ask_secs: 0.25, calm_aw: 0.3, wall_m: 1.2, wall_back_m: 1.6, low_tan: 2.2, low_rate: 6.0 }
}

pub fn page_backkey() -> BackkeyK {
    crate::knobs::BackkeyK { tab: true }
}

pub fn page_morale() -> MoraleK {
    crate::knobs::MoraleK { half_life: 25.0, ask_secs: 0.25, alone: 0.25, alone_m: 7.0, dark: 0.25, lit_asks: 3.0, blast: 0.35, shock_cap: 0.5, blast_m: 10.0, near_m: 4.0, far_k: 0.6, jumpy: 0.3, broken: 0.7, hyst: 0.15, duck_secs: 2.5, down_secs: 0.6, back_until: 2.2, back_m: 3.5, back_speed: 2.0, burst_rounds: 5.0, burst_every: 0.2, spread: 0.3, call2_secs: 6.0, bait_m: 2.0, bait_short: 5.0, bait_every: 0.1, reload_secs: 3.2, bark_secs: 2.0, fan_deg: 70.0, fan_step: 17.5, straight_m: 1.5, training: crate::morale::Training::Difficulty, green_broken: 0.5, leader_down: 0.25, leader_shock: 0.3, flinch_secs: 0.8, shout_gap: 0.6, order_m: 4.0, order_beat: 0.5, order_secs: 1.0, order_gap: 0.15, stand_secs: 2.5, side_m: 0.75, steady_duck: 1.0, hold_shock: 0.05, hold_max: 70.0, nerve: crate::morale::Nerve::Normal, nerve_k: crate::knobs::MoraleKNerveK { brittle: 1.4, normal: 1.0, steady: 0.7 }, dark_read: crate::knobs::MoraleKDarkRead::Place }
}

pub fn page_condition() -> ConditionK {
    crate::knobs::ConditionK { hurt_pace: 1.5, hurt_sup: 0.35, hurt_secs: 0.0 }
}

pub fn page_inhand() -> InhandK {
    crate::knobs::InhandK { from: 0.3, to: 0.92, wrist: 55.0, both: 0.34, rate: 9.0, holster_out: 0.03 }
}

pub fn page_packside() -> PacksideK {
    crate::knobs::PacksideK { gap: 0.012, handle: -0.258, gun: -0.24, over: 0.012, head_min: 1.5, butt: 1.56, rest: 0.012, pitch0: -0.1, pitch1: 1.3 }
}

pub fn page_runner() -> RunnerK {
    crate::knobs::RunnerK { close_m: 5.0, out_secs: 0.8, beat_secs: 0.6, tell_m: 4.0, told_m: 8.0, tell_beat: 0.8, tell_aware: 0.6, shy_m: 2.0, mate_m: 20.0, mate_secs: 10.0, clear_m: 1.5, edge_m: 2.0, corner_m: 25.0, gone_secs: 2.0, stall_secs: 3.0, retry_secs: 2.0, replan_secs: 0.5, ahead_m: 3.0, ask_cells: 60.0, ask_wide: 240.0 }
}

pub fn page_startle() -> StartleK {
    crate::knobs::StartleK { beat: 0.5, draw: 0.3, fresh: 1.0, late: 0.25, run_m: 8.0, near_m: 2.5, clear_secs: 1.5, run_secs: 4.0, stuck_secs: 0.75, stuck_m: 0.3 }
}

pub fn page_smokebreak() -> SmokebreakK {
    crate::knobs::SmokebreakK { light_secs: 3.2, light_up: [0.5, 2.6], flame_at: 1.3, flame: 0.9, every: 7.5, draw: 2.1, out_after: 0.7, out: 1.3, flame_color: [1.0, 0.62, 0.24], flame_k: 2.4, flame_range: 3.2, ember_color: [1.0, 0.36, 0.1], ember_k: 0.1, ember_draw_k: 0.7, ember_range: 1.3, ember_every: 0.4, tip_out: 0.11, tip_up: 0.02, thread: crate::knobs::SmokebreakKThread { radius: 0.02, cells: 1.2, density: 5.0, temp: 5.0, speed: 0.25, push: 10.0, jitter: 0.35 }, breath: crate::knobs::SmokebreakKBreath { radius: 0.04, cells: 1.5, density: 16.0, temp: 2.5, speed: 1.1, push: 12.0, expand: 2.2, attack: 0.1, jitter: 0.3, down: 0.1 }, color: [0.72, 0.76, 0.86], mouth_ahead: 0.1, mouth_down: 0.075 }
}

pub fn page_showwhy() -> ShowwhyK {
    crate::knobs::ShowwhyK { line_after: 0.6, line_gap: 0.3, wave_every: 2.6, wave_later: 6.0, line_min: 1.2, line_say_gap: 4.0, mate_half: 0.45, squeeze_gap: 0.35, squeeze_reach: 1.15, fist_after: 0.45, hold_every: 8.0, hold_max: 3.0, door_after: 0.5, door_every: 4.0, after_you_gap: 10.0, hurt_late: 6.0, lag_m: 6.0, lag_every: 20.0, lag_max: 2.0, keep_after: 0.4, keep_gap: 1.5, keep_every: 30.0, owed_for: 5.0 }
}

pub fn page_fire() -> FireK {
    crate::knobs::FireK { names: alloc::vec![String::from("brazier")], height: 1.15, mouth: 1.15, coals: 0.26, coal_swing: 0.6, bed_r: 1.42, depth: 0.4, rates: alloc::vec![0.43, 1.3, 2.7, 4.9, 8.3], weights: alloc::vec![0.8, 0.9, 0.7, 0.45, 0.2], least: 0.6, most: 1.6, wander: 0.14, wander_up: 0.1, lift: 0.5, kick: 0.6, kick_secs: 0.12, bright: 1.25, warm: alloc::vec![1.0, 0.72, 0.51], reach: 1.4, stir_gap: 0.3, size: 1.1, pop_rate: 1.1, pop_slot: 0.2, embers: 60.0, ember_life: 1.7, ember_speed: 1.1, ember_size: 0.004, ember_gain: 0.45, ember_crack: 10.0, ember_r: 30.0, look: crate::knobs::FireKLook { gain: 3.0, core: 4.0, sigma: 2.5, speed: 0.7, tall: 0.95, torn: 1.1, plume: 1.0, shimmer: 0.019, soot: 0.5, breathe: 1.0, react: 1.0, lean: 0.2, steps: 40.0 }, smoke: crate::knobs::FireKSmoke { on: true, up: 1.9, radius: 0.26, cells: 4.0, density: 1.0, temp: 21.0, speed: 0.9, push: 6.0, jitter: 0.5, every: 1.0, view_r: 18.0, lean: 0.12 }, bed: 0.85, crack: 0.9, bed_secs: 12.0, max: 4.0, draw_r: 70.0, wind_max: 3.0 }
}

pub fn page_flinch() -> FlinchK {
    crate::knobs::FlinchK { least: 0.06, full: 0.35, far: 0.45, min_travel: 1.0, steady_k: 0.65, break_secs: 2.5, hot_secs: 1.0, ease_k: 0.5, ask_secs: 0.25 }
}

pub fn page_darkdoor() -> DarkdoorK {
    crate::knobs::DarkdoorK { who: crate::knobs::DarkdoorKWho::Rest, stop_m: 2.5, pair_m: 4.0, dark_at: 0.2, dark_share: 0.7, lit_by: 0.1, samples: 40.0, ask_secs: 0.5, open_rad: 0.35, arm_secs: 3.0, stand_m: 1.3, side_m: 0.3, sweep_secs: 3.2, ear_m: 12.0, mate_m: 40.0, mate_try: 3.0, call_secs: 2.0, copy_secs: 1.2, lag_secs: 5.0, with_secs: 10.0, keep_aw: 0.3, keep_secs: 2.0, resume_secs: 12.0, seen_secs: 2.0, wait_secs: 20.0, again_secs: 45.0, fed_up_secs: 300.0, fed_up_n: 2.0, nerve_secs: 1.5, slow_k: 0.5 }
}

pub fn page_darkfoe() -> DarkfoeK {
    crate::knobs::DarkfoeK { carry: 1.0, lit_at: 0.3, look_m: 12.0, step_m: 2.0, fan_rad: 0.3, see_share: 0.34, ask_secs: 0.25, on_secs: 0.6, on_jitter: 2.5, off_secs: 15.0, off_jitter: 0.4, hold_secs: 2.0, lamp_m: 16.0, blind_secs: 3.0, seek_m: 10.0, walk_m: 12.0, edge_m: 3.0, stumble_k: 0.75, again_secs: 20.0, moves: 2.0, quiet_secs: 4.0, calm_secs: 6.0 }
}

pub fn page_powerout() -> PoweroutK {
    crate::knobs::PoweroutK { see_m: 14.0, think: [3.0, 6.0], beat: 2.0, look_m: 12.0, about_m: 4.0, step_m: 2.0, fan_rad: 0.3, lit_at: 0.3, drop_k: 0.5, dark_after: 0.3, men: 2.0, max_m: 60.0, parties: 1.0, look_secs: 1.5, tries: 8.0, lever_secs: 8.0, overdue: 15.0, ask: 6.0, cover_m: [1.6, 3.2] }
}

pub fn page_squaddeal() -> SquaddealK {
    crate::knobs::SquaddealK { one: 5.0, two: 8.0, names: alloc::vec![String::from("Alpha"), String::from("Bravo"), String::from("Charlie")] }
}

pub fn page_stayask() -> StayaskK {
    crate::knobs::StayaskK { men: 1.0 }
}

pub fn page_rescuecheck() -> RescuecheckK {
    crate::knobs::RescuecheckK { first: [14.0, 22.0], every: [24.0, 36.0], ask_secs: 4.0, go_secs: 4.0, wait_secs: 5.0, tries: 4.0, feet_m: 2.5, word_secs: 2.5 }
}

pub fn page_kitload() -> KitloadK {
    crate::knobs::KitloadK { gun_kg: 0.035, gun_bulk: [0.04, 0.17], small: { let mut t = JsMap::new(); t.set(String::from("smoke"), [0.1, 0.02]); t.set(String::from("bangs"), [0.07, 0.02]); t.set(String::from("frags"), [0.08, 0.02]); t.set(String::from("shells"), [0.07, 0.01]); t.set(String::from("charges"), [0.1, 0.02]); t.set(String::from("wand"), [0.07, 0.02]); t.set(String::from("cams"), [0.04, 0.01]); t }, big: { let mut t = JsMap::new(); t.set(String::from("shotgun"), [0.18, 0.1]); t.set(String::from("hammer"), [0.3, 0.1]); t.set(String::from("wall"), [0.24, 0.06]); t }, pace: 0.4, seen: 0.6, heard: 0.7, sprint: 0.5, pace_top: 2.4 }
}

pub fn page_rescueend() -> RescueendK {
    crate::knobs::RescueendK { out_m: 3.5, clear_secs: 1.5, marker_m: 2.4, reach_m: 2.2, gap_m: 1.3, gap_step_m: 0.7 }
}

pub fn page_rescueflush() -> RescueflushK {
    crate::knobs::RescueflushK { after_secs: 4.0, gap_secs: 2.5, again_secs: 8.0, reach_m: 10.0, kill_secs: 3.0, busy_secs: 2.0, smoke_k: 0.3, at_door_m: 1.8 }
}

pub fn page_posemath() -> PosemathK {
    crate::knobs::PosemathK { singles: true }
}

pub fn page_sense() -> SenseK {
    crate::knobs::SenseK { glow: crate::knobs::SenseKGlow { sweep: 32.0 }, smoke: crate::knobs::SenseKSmoke { sigma: 6.0, decay: 0.14, spread: 0.2, rise: 1.0, rise_rate: 0.3, in_wind: 0.15, least: 0.05, gone: 0.02, reserve: 12.0 } }
}

pub fn page_cmdcheck() -> CmdcheckK {
    crate::knobs::CmdcheckK { keys: 24.0, key_len: 24.0, span: 16384.0, wheel: 4096.0, far: 10000.0, dist_min: 0.05, dist_max: 2000.0, fov_min: 0.02, fov_max: 3.0, aspect_min: 0.05, aspect_max: 20.0, reach: 80.0, steep: 1.5 }
}

pub fn page_seats() -> SeatsK {
    crate::knobs::SeatsK { max: 2.0, apart: 0.9, keep: 2.0 }
}

pub fn page_doze() -> DozeK {
    crate::knobs::DozeK { min_wait: 20.0, max_wait: 90.0, chair_m: 4.5, nod_secs: 2.5, rise_secs: 2.0, hear_k: 0.6, light_at: 0.3, light_secs: 0.8, ask_secs: 0.25, wake_aw: 0.55, bump_aw: 0.7, stir_secs: 1.3, breath_secs: 3.6, screen_m: 1.5, pull_m: 0.5, nap_wait: 40.0, settle_secs: 1.2, ears: 0.5, noises: 0.6, say_first: alloc::vec![5.0, 11.0], say_every: alloc::vec![16.0, 30.0], idle_secs: 60.0 }
}

pub fn page_netquiet() -> NetquietK {
    crate::knobs::NetquietK { unanswered: 0.12, heard_it: 0.06, body: 0.2, body_told: 0.08, fall_seen: 0.08, fall_unseen: 0.22, steady_k: 0.6, ask_beat: 1.4, body_beat: 2.0, bark_secs: 3.0, say_hold: 4.0, shooter_secs: 1.0, body_m: 10.0, whip_shift: 0.12, whip_secs: 1.5, whip_hold: 0.8, whip_turn: 10.0, whip_turn_broken: 14.0, whip_m: 15.0, whip_near_m: 4.0, last_beat: 1.5, calm_secs: 2.0, alert_secs: 15.0, give_up: 8.0, mate_m: 5.0, arc_deg: 40.0, eye_deg: 58.0, eye_slack: 4.0, crouch: 0.4, wary: 0.35, back_speed: 2.0, max_m: 9.0, door_m: 2.0, eye_m: 1.3, stay_m: 1.0 }
}

pub fn page_attention() -> AttentionK {
    crate::knobs::AttentionK { on: true, focus_deg: 25.0, side_k: 0.3, ear_k: 0.5, break_gain: 0.2, calm_aw: 0.6, draw_aw: 0.34, floor: 0.32, whistle_secs: 4.0, noise_secs: 5.0, door_secs: 4.0, light_secs: 5.0, tell_secs: 0.8, dwell_sweep: 0.3, walk_cap: 15.0, wear_k: 0.6, memory_secs: 60.0, spent: 3.0, same_secs: 2.5, lent_back_secs: 6.0, marks: true, ring_m: 0.42, way: true, way_reask: 0.5 }
}

pub fn page_picture() -> PictureK {
    crate::knobs::PictureK { look_secs: 0.2, glimpse: 0.05, tell_secs: 1.0, shout_m: 16.0, pop_m: 7.0, made_out: 0.2, made_out_m: 3.0, fresh_secs: 2.0, stale_secs: 20.0, stale_k: 0.4, forget_secs: 90.0, lost_k: 0.5, walk_mps: 1.2, walk_max: 6.0, heard_m: 2.0, maybe_k: 0.6, reach_m: 30.0, eye_h: 1.6, chest_h: 1.15, low_h: 0.7, head_up: 0.45, flee_cost: 8.0, stack_k: 0.5, step_at: 0.5, step_free: 0.25, step_m: 2.5, step_after: 0.6, step_rest: 1.5, step_secs: 5.0, show: crate::knobs::PictureKShow::Off, show_m: 9.0 }
}

pub fn page_response() -> ResponseK {
    crate::knobs::ResponseK { clear_m: 12.0, eta_step: 5.0, word_secs: 0.5, call_secs: 6.0 }
}

pub fn page_handwork() -> HandworkK {
    crate::knobs::HandworkK { stand_m: 0.52, step_rate: 7.0, step_pace: 1.7, stand_side: 0.1, wait_secs: 0.7, lever_secs: 0.5, lever_at: 0.3, stow_secs: 0.45, radio_k: 0.6, radio_min: 0.9, radio_max: 2.4 }
}

pub fn page_workstow() -> WorkstowK {
    crate::knobs::WorkstowK { mate_down: 0.3, mate_up: 0.45, show: 0.6, down_max: 1.6, sam_up: 0.3, fight_cap: 8.0 }
}

pub fn page_squadnet() -> SquadnetK {
    crate::knobs::SquadnetK { near: 0.24, edge: true, edge_in: 0.04, click: 0.3, click_rate: 1.3, down_gain: 0.7, down_rate: 0.55, down_secs: 5.0 }
}

pub fn page_takedown() -> TakedownK {
    crate::knobs::TakedownK { partner_hit: 0.3, partner_end: 0.58, breath_hz: 0.24, breath_rise: 0.02, breath_in: 1.2 }
}

pub fn page_stealthpose() -> StealthposeK {
    crate::knobs::StealthposeK { sway_secs: 2.2, sway_m: 0.012, drift_m: 0.016, rise_secs: 0.9, cable_in: 0.4, cable_bow: 26.0, cable_step: 0.07 }
}

pub fn page_look() -> LookK {
    crate::knobs::LookK { on: true }
}

pub fn page_lived() -> LivedK {
    crate::knobs::LivedK { on: true }
}

pub fn page_env() -> EnvK {
    crate::knobs::EnvK { exposure: 2.0, tube_gain: 475.0 }
}

pub fn page_shapes() -> ShapesK {
    crate::knobs::ShapesK { on: true }
}

pub fn page_flash() -> FlashK {
    crate::knobs::FlashK { secs: [7.0, 4.0], stagger: [2.5, 1.0], lift: 1.2, concuss_m: 6.0, concuss_k: 0.5, own_m: 2.5, own_secs: 1.5, fuse: 0.7 }
}

pub fn page_pace() -> PaceK {
    crate::knobs::PaceK { max: 2.2, accel: 2.6, turn: 8.0, swing: 30.0 }
}

pub fn page_lights() -> LightsK {
    crate::knobs::LightsK { after_secs: 2.5, hot_secs: 5.0, step_secs: 4.0, tail: 0.6, on_m: 0.8, alone_m: 7.0, lit_at: 0.5, lit_m: 15.0, lit_mark_m: 6.0, wall_m: 1.5, wall_back_m: 1.9, wall_secs: 0.3 }
}

pub fn page_nvg() -> NvgK {
    crate::knobs::NvgK { quiet_secs: 30.0, lead_tail: 1.5, dark_at: 0.5, black_at: 0.15, black_reads: 2.0, clear_reads: 3.0, albedo: 0.3, noise: 0.046, down_secs: 0.25, up_secs: 0.7, call_gap: 8.0 }
}

pub fn page_irlight() -> IrlightK {
    crate::knobs::IrlightK { show: 0.1, hold: 1.5, power: 8.0, reach: 22.0, cone: 65.0, core: 20.0, dip: 6.0, near_m: 5.0, near_min: 0.003, near_up: 3.0, near_down: 12.0 }
}

pub fn page_wallpen() -> WallpenK {
    crate::knobs::WallpenK { base: { let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.1); t.set(crate::wallpen::ThinStuff::Wood, 0.0); t.set(crate::wallpen::ThinStuff::Fabric, 0.03); t.set(crate::wallpen::ThinStuff::Cardboard, 0.02); t.set(crate::wallpen::ThinStuff::Foliage, 0.0); t }, per_m: { let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.5); t.set(crate::wallpen::ThinStuff::Wood, 4.5); t.set(crate::wallpen::ThinStuff::Fabric, 1.0); t.set(crate::wallpen::ThinStuff::Cardboard, 1.0); t.set(crate::wallpen::ThinStuff::Foliage, 0.3); t }, thin_max: { let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.17); t.set(crate::wallpen::ThinStuff::Wood, 0.12); t.set(crate::wallpen::ThinStuff::Fabric, 0.13); t.set(crate::wallpen::ThinStuff::Cardboard, 0.7); t.set(crate::wallpen::ThinStuff::Foliage, 2.0); t }, max_thru: { let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.4); t.set(crate::wallpen::ThinStuff::Wood, 0.14); t.set(crate::wallpen::ThinStuff::Fabric, 0.3); t.set(crate::wallpen::ThinStuff::Cardboard, 0.8); t.set(crate::wallpen::ThinStuff::Foliage, 2.5); t }, min_energy: 0.12, max_things: 5.0, deflect_deg: 4.0, behind_m: 4.0, thru_rounds: 4.0, thru_rest_secs: 6.0 }
}

pub fn page_doortools() -> DoortoolsK {
    crate::knobs::DoortoolsK { charges: 2.0, bangs: 3.0, blow_key: String::from("KeyF"), word_tap: 0.6, stack_cap: 60.0, still_secs: 2.5, kick_cap: 5.0, in_cap: 9.0, shove: 5.5, hold_open: 10.0, toss_after_open: 0.15, bang_in: 2.4, fuse_after_land: 0.35, air_in: 1.6, air_h: 1.2, toss_cap: 1.6, toss_after_kick: 0.05, throw_reach: 4.0, toss_turn: 0.0, frag_way_wait: 1.0, frag_way_open: 1.15, frag_way_ajar: 0.6, last_out_m: 3.0, after_bang: 0.6, quiet: 0.15, entry_cap: 2.8, second: 0.25, sweep_rad: 0.9, sweep_secs: 0.6, corner_secs: 0.3, corner_rad: 0.3, corner_cap: 1.6, lead_seen_min: 12.0, lead_seen_secs: 30.0, lead_seen_m: 11.0, sector_secs: 0.6, scan_rate: 1.4, look_rad: 0.3, look_m: 11.0, dead_cells: 8.0, search_moves: 3.0, search_cap: 24.0, search_cells: 900.0, tuck: 0.4, edge_out: 0.4, bang_file: 1.1, set_secs: 2.0, standoff: alloc::vec![2.6, 2.0, 1.5], knock_m: 2.0, stun_m: 8.0, stun_secs: alloc::vec![6.0, 2.5], stagger_secs: 2.0, backblast_m: 1.6, backblast_secs: 1.5, blow_vel: 12.0, slice_back: 1.0, slice_look: 0.35, slice_edge: 0.12, slice_deep: 3.5, slice_pace: 0.9, word_stack: 20.0, word_in: 6.0, room_back: alloc::vec![2.8, 3.7, 4.6, 5.5], pod_apart: 1.2, thr_pull_at: alloc::vec![0.45, 1.8], thr_still: alloc::vec![0.45, 1.2], thr_arc: alloc::vec![alloc::vec![-0.45, 1.2], alloc::vec![0.25, 1.8]], thr_min: 4.0, thr_quiet: 2.0, thr_cap: 25.0, thr_bangs: 2.0, thr_pocket: 12.0, thr_shot_m: 8.0, thr_throw_m: 5.0, thr_flash_m: 7.0, thr_turn: 0.5, thr_next: 0.3, thr_light: 1.0, thr_in: 4.0, thr_pie_arc: alloc::vec![alloc::vec![-0.2, 1.4], alloc::vec![-0.05, 1.6], alloc::vec![0.08, 1.8]], thr_pie_apart: 0.65, thr_lean_m: 0.15, thr_lean_look: 1.1, thr_doors_m: 5.5, thr_doors_walk: 16.0, thr_doors_stay: 1.0, other_door_m: 9.0, buddy_m: 5.0, buddy_pair_m: 12.0, buddy_left_m: 8.0, flow_cells: 28.0, flow_moves: 5.0, flow_reach: 5.5, flow_deep: 3.0, flow_bound: 8.0, flow_secs: 10.0, flow_fight: 20.0, flow_max: 400.0, word_cuts_cap: 8.0, bang_into_min: 1.5, bang_into_lit: 0.9, bang_into_deep: 6.0, bang_into_wide: 4.2, bang_hand_m: 0.9, bang_pass_v: 6.5, pod_frame_m: 0.55, thr_hand_wait: 6.0, thr_hand_every: 0.25, frag_hold_cap: 8.0, threat_secs: 12.0, threat_cap: 1.5, threat_near_m: 1.5, threat_stuck: 1.0, thr_first: 1.0, thr_first_min: 2.0, thr_first_stale: 0.0, thr_first_in: 0.0, thr_last_m: 2.0, thr_last_fresh: 3.0 }
}

pub fn page_roomfight() -> RoomfightK {
    crate::knobs::RoomfightK { evidence_secs: 12.0, sticky: 0.25, crowd: 0.2, fear_min: 0.15, close_m: 2.5, no_shot: 0.4, point_blank: 2.0, no_shot_go: 1.5, no_shot_pass: 4.0, rally_k: 1.0, arc_half_deg: 100.0, cover_half_deg: 45.0, cover_near_m: 6.0, team_hot: 45.0, let_go: 0.8, step_look_secs: 5.0, door_look_secs: 3.5, door_shot_m: 8.0, hold_short_secs: 4.0, move_engage_m: 7.0, lamp: 0.7, preaim_deg: 12.0 }
}

pub fn page_orders() -> OrdersK {
    crate::knobs::OrdersK { who_key: String::from("KeyT"), quick_key: String::from("KeyB"), wait_key: String::from("Digit5"), wait_alt: String::from("KeyF"), refresh_secs: 1.0, door_pick_m: 1.0, self_m: 1.0, man_m: 0.8, door_ray_pad: 0.08, door_ray_reach_m: 3.0, floor_snap_m: 1.0, jamb_out: 0.55, jamb_past: 0.45, file_gap: 0.9, seam_m: 0.03, work_seen_m: 4.0, follow_back: alloc::vec![1.3, 2.4], crumb: 0.4, trail_m: 10.0, run: 2.2, walk: 1.5, gain: 1.5, near_m: 0.6, band_m: 0.6, word_far_m: 2.0, word_hold_s: 2.5, file_apart: 0.9, file_tight: 0.75, off_door: 0.8, aside_m: 2.6, aside_lane: 0.8, aside_off: 0.95, file_queue_gap: 1.3, file_queue_cost: 500.0, prop_round_pad: 0.25, prop_round_cost: 6.0, prop_round_more: 2.0, prop_round_door: 2.5, aside_hold: 1.5, brisk: 2.0, brisk_in: 1.8, onto: 1.3, entry: 2.2, move_spread: 0.9, cold_secs: 2.5, lane_in: 0.45, lane_out: 0.7, hold_stride: 0.15, stuck_secs: 4.0, hold_group_m: 4.0, hold_door_m: 2.0, cleared_dear: 20.0, round_cap: 1.6, same_near_m: 5.0, same_apart_m: 6.0, same_long: 1.5, trail_reach_m: 3.0, trail_long: 1.5 }
}

pub fn page_cover() -> CoverK {
    crate::knobs::CoverK { rear_read_in: 1.0, rear_read_m: 14.0, rear_read_deg: 35.0, rear_read_secs: 1.0, rear_read_sticky: 1.3, rear_look: 6.0, rear_walk_m: 5.0, reach: 9.0, deal_secs: 0.25, gap_cells: 3.0, through_rows: 3.0, alcove_rows: 3.0, bridge: 3.0, in_m: 0.9, lane: 0.7, lane_keep: 0.5, lane_secs: 0.6, sam_deg: 35.0, sam_k: 0.2, held_k: 0.1, shut_k: 0.7, rear_k: 0.8, near_m: 3.0, contact_secs: 4.0, rear_m: 4.0, rear_from: 3.0, rear_deg: 60.0, sticky: 1.2, keep_secs: 1.5, rear_keep: 1.25, behind_k: 0.2, twist: 1.45, back_in: 3.0, back_out: 3.4, back_body: 0.0, back_turn_body: 0.7, back_turn_secs: 1.0, swing: 9.0, step_m: 1.8, step_after: 0.8, step_off_m: 0.7, step_reach: 6.0, step_secs: 5.0, step_place_m: 0.6 }
}

pub fn page_roomfit() -> RoomfitK {
    crate::knobs::RoomfitK { spread0: 1.2, spread_per: 0.5, apart: 1.1, loose: 1.7, loose_k: 0.8, near_k: 0.35, near_free: 0.4, wall_k: 0.45, corner_k: 0.15, funnel_m: 4.5, funnel_pad: 0.3, funnel_flare: 0.12, funnel_k: 1.6, own_k: 0.7, known_funnel_k: 0.4, shut_funnel_k: 0.6, ang_k: 0.5, ang_lo: 18.0, ang_hi: 60.0, ang_max: 85.0, close_m: 1.6, close_k: 0.8, lane_m: 0.7, lane_k: 4.0, reach: 10.0, near_m: 4.0, known_k: 0.35, shut_k: 0.7, locked_k: 0.3, front_k: 0.25, second_k: 0.3, sector_m: 5.0, sector_k: 0.55, sector_deg: 35.0, sector_across: 0.5, sector_look: 6.0, sector_far_m: 10.0, sector_far_k: 0.7, watch_in_m: 1.5, again_secs: 1.0, sticky: 1.3 }
}

pub fn page_entry() -> EntryK {
    crate::knobs::EntryK { near_m: 3.0, line_deg: 45.0, known_m: 4.0, move_m: 3.0, apart: 0.9 }
}

pub fn page_rear() -> RearK {
    crate::knobs::RearK { secs: 0.75 }
}

pub fn page_silent() -> SilentK {
    crate::knobs::SilentK { on: true, start: false, pace: 1.6, step: 0.2, noticed: 0.5 }
}

pub fn page_pair() -> PairK {
    crate::knobs::PairK { on: false, max_squad: 1.0, pick_m: 0.9, stage_m: 1.2, reach: 1.05, secs: 0.6, scuffle: 0.0, pace: 1.1, go_pace: 1.6, stalk_every: 0.3, stalk_wait: 20.0, ring_m: 2.4, steady_face_secs: 1.2, watch_secs: 1.2, watch_aw: 0.25, heel_ring: 2.9, heel_fresh: 1.2, heel_wait: 22.0, take_secs: 75.0, dark_vis: 0.3, dark_m: 1.5, dark_h: 0.9, lit_secs: 2.0, lit_every: 20.0 }
}

pub fn page_rescue() -> RescueK {
    crate::knobs::RescueK { start: crate::knobs::RescueKStart::Hot, resolve_secs: 35.0, second_hostage_secs: 8.0, aim_secs: 1.2, set_secs: 9.0, unset_react: 1.1, door_man: crate::knobs::RescueKDoorMan::Corridor, roamers: 1.0, crew_call_secs: 1.5, squad: 2.0, sam_at: [7.6, 0.0, 13.4], sam_yaw: 0.9, ops_at: [[6.6, 0.0, 14.25], [8.5, 0.0, 14.3]] }
}

pub fn page_digin() -> DiginK {
    crate::knobs::DiginK { on: false, stay: false, reach_m: 0.0, keeper: true, cover: true, brace_in_room: true, noticed_secs: 0.35, shut_open: false, shut_secs: 7.0, shut_home_rad: 0.3, shut_doors: 2.0, gather: false, gather_m: 24.0, gather_shy_m: 6.0, think: [2.0, 5.0], wedge_secs: [4.0, 7.0], wedges: 2.0, caught: [0.5, 0.9], try_secs: 0.8, pace: 1.0, close_m: 6.0, prop_m: 9.0, haul_speed: 0.7, scrape_level: 0.5, scrape_every: 0.8, shout_m: 18.0, alt_m: 10.0, better: 0.6, fear: 2.4, hold_secs: 90.0, redig_secs: 6.0, lane_secs: 2.0 }
}

pub fn page_pressure() -> PressureK {
    crate::knobs::PressureK { on: false, press: true, flank: true, squad_drill: true, second: false, max_men: 4.0, memory_secs: 15.0, organise_k: 1.0, warned: false, stand_press: false, report_secs: 1.4, stand_secs: 75.0, stand_grace: 5.0, again_secs: 14.0, stuns: 0.0, stun_know_secs: 20.0, stun_moved_secs: 3.0, last_holds: true, picket: false, by_way: false, rescue_crew: false, drilled: false, flank_max_m: 46.0, flank_k: 3.0, flank_slack_m: 14.0, last_man_secs: [0.35, 0.9], loud_secs: 90.0, out_secs: [0.8, 1.6], gather_secs: 8.0, flank_secs: 6.0, siege_secs: 25.0, word_secs: 0.9, way_out: false, way_out_m: 0.6, way_out_secs: 60.0, sweep: false, sweep_secs: 20.0, sweep_men: 2.0, sweep_beat: 1.5, sweep_pair_m: 12.0 }
}

pub fn page_lanes() -> LanesK {
    crate::knobs::LanesK { secs: 150.0, reach_m: 32.0, half_deg: 22.0, width_m: 1.1, field_m: 12.0, own_m: 1.0, min_m: 2.0, round_k: 0.5, near_m: 1.5, react_secs: 0.35, call_secs: 0.9, see_m: 30.0, body_m: 12.0, dear: 14.0, ahead_m: 3.5, close_m: 4.5, hot_secs: 8.0, round_men: 2.0, think_secs: 2.5, round_max_m: 80.0, round_gap: 25.0, round_side_deg: 55.0, round_show_m: 8.0, shift_m: 0.9, stun_m: 12.0, bang_m: 5.0, push_secs: 6.0, push_m: 12.0, push_fast: 0.8, stun_gap: 20.0, moved_m: 15.0, moved_secs: 20.0, peek_quiet: 10.0, peek_gap: 15.0, peek_max: 60.0, peek_lost: 45.0, peek_reach_m: 5.0, peek_near_m: 10.0, peek_look: 0.8, peek_sure: 45.0, peek_keep: 60.0, peek_pace: 1.3, peek_lean: 0.35, pin_rounds: 3.0, pin_every: 0.12, pin_snap: 0.6, pin_gap: 4.0, pin_max: 4.0, pin_rest: 20.0, pin_spread: 0.24, pin_aim_deg: 9.0, pin_pace: 2.0, pin_show_m: 12.0, pin_min_m: 7.0, look_m: 1.5, frag_think: 4.0, frag_gap: 30.0, frag_look: 1.5, frag_step_m: 0.9, max: 6.0 }
}

pub fn page_gunhold() -> GunholdK {
    crate::knobs::GunholdK { cover_m: 10.0, near_m: 3.0, rear_deg: 70.0, keep_secs: 14.0, peek_every: 4.0, peek_secs: 0.8, show_m: 2.5, shy_s: 6.0, dash_m: 3.5, face_deg: 50.0, flank_max_m: 40.0, react: [0.1, 0.4] }
}

pub fn page_hard() -> HardK {
    crate::knobs::HardK { reach_m: 0.0, stuns: 0.0 }
}

pub fn page_hard_at() -> HardAtK {
    crate::knobs::HardAtK { rows: { let mut t = JsMap::new(); t.set(String::from("the block"), Some(1.0)); t } }
}

pub fn page_drilled() -> DrilledK {
    crate::knobs::DrilledK { shaken: 0.5 }
}

pub fn page_squads() -> SquadsK {
    crate::knobs::SquadsK { think: [2.0, 4.5], net_lag: 0.8, help_men: 3.0, flank_men: 2.0, again_secs: 20.0, settle_secs: 8.0, quiet_secs: 8.0, recall_secs: 20.0, reinforce: [1.0, 2.5], ready_beat: 0.4, fire_m: 10.0, rear_m: 6.0, soft_m: 3.0, mouth_m: 18.0, hold_back_m: 16.0, hold_secs: 40.0, drill_net: true, net_k: 2.0, reinforce_k: 2.0, drill_angles: false, follow_m: 6.0, drill_own: false, own_up: 1.0, drill_back: false, back_beat: 3.0, drill_look: true, sweep_secs: 20.0, sweep_men: 2.0, sweep_beat: 1.5, sweep_stayers: true, check_every: 40.0, check_jitter: 0.2, check_alert_k: 0.5, check_ask: 4.0, check_send: 8.0, check_heard: [1.0, 2.0], check_hear_m: 4.0, look_men: 2.0, call_secs: [1.5, 2.5], call_cut_ask: [1.0, 2.0], net_every: 60.0, net_beat: [6.0, 9.0], net_look_men: 2.0, net_stops: 3.0 }
}

pub fn page_drill_at() -> DrillAtK {
    crate::knobs::DrillAtK { rows: { let mut t = JsMap::new(); t.set(String::from("the block, three squads"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(true), net_k: Some(2.0), reinforce_k: Some(2.0), drill_own: Some(false), own_up: None, drill_back: Some(false), back_beat: None }); t.set(String::from("the warehouse"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(false), net_k: None, reinforce_k: None, drill_own: Some(false), own_up: None, drill_back: Some(false), back_beat: None }); t.set(String::from("the block"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(false), net_k: None, reinforce_k: None, drill_own: Some(false), own_up: None, drill_back: Some(false), back_beat: None }); t.set(String::from("the compound"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(true), net_k: Some(1.0), reinforce_k: Some(0.75), drill_own: Some(true), own_up: Some(1.0), drill_back: Some(false), back_beat: None }); t.set(String::from("the barracks"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(false), net_k: None, reinforce_k: None, drill_own: Some(false), own_up: None, drill_back: Some(true), back_beat: Some(3.0) }); t } }
}

pub fn page_cues() -> CuesK {
    crate::knobs::CuesK { rows: 3.0, secs: 8.0, min_show: 2.5, per_class: 2.0, dup_secs: 10.0, gaps: alloc::vec![0.0, 1.2, 3.0, 3.0, 2.5, 3.0, 12.0], squelch: 0.35, man_secs: 2.5, again_secs: 28.0, still_secs: 8.0, still_quiet: 5.0, stay_secs: 2.0, overheard_gap: 4.0 }
}

pub fn page_wx_sim() -> WxSimK {
    crate::knobs::WxSimK { rain_k: 1.2, thunder_k: 1.5, thunder_secs: 1.4, thunder_loud: 0.45, roof_bed: 0.32 }
}

pub fn page_air_sim() -> AirSimK {
    crate::knobs::AirSimK { glass_open: 0.5 }
}

pub fn page_laststand() -> LaststandK {
    crate::knobs::LaststandK { moved_secs: 3.0, still_secs: 2.0, moved_gap: 8.0, few: 3.0, quiet: 3.0, first: [3.0, 6.0], every: 22.0, answer: [1.2, 2.2], between: [3.2, 3.9], listen: [3.2, 3.8] }
}

pub fn page_stackroom() -> StackroomK {
    crate::knobs::StackroomK { apart: 0.75, squeeze: 0.5, round_m: 2.0, round_k: 1.6, walk_m: 9.0, next_walk_m: 14.0, next_m: 3.5, lane_back: 1.6, view_ajar: 0.6, view_cells: 3.0, view_far: 7.0, view_eye: 1.2, view_step: 0.3, view_past: 4.2, view_off: 0.4, view_apart: 0.85, view_out: 0.3, view_men: 12.0, view_wall: 0.35 }
}

pub fn page_rearwatch() -> RearwatchK {
    crate::knobs::RearwatchK { reach: 9.0, ajar: 0.6, grace: 1.5, short: 1.5, room_secs: 8.0, room_m: 8.0 }
}

pub fn page_stackline() -> StacklineK {
    crate::knobs::StacklineK { ajar: 0.6, bins: 4.0, eye: 1.2, inset: 0.2, on_mark: 0.35 }
}

pub fn page_sneak() -> SneakK {
    crate::knobs::SneakK { on: false, min_pace: 0.5, low_pace: 1.45, cross_pace: 1.6, step: 0.14, trail_m: 60.0, back: 0.3, ahead_m: 2.4, lit_at: 0.1, seen_rate: 0.05, cone_deg: 25.0, near_m: 2.1, cross_m: 4.5, patience: 8.0, cross_secs: 8.0, budget: 0.12, margin: 1.6, memory: 12.0, nag_secs: 7.0, cue_gap: 12.0, stand_m: 4.0, stand_near: 0.8, side_m: 2.6, side_stop: 0.25, door_m: 1.8, door_pad: 0.4, door_settle: 1.0, hide_gap: 2.5, noise_walk_m: 2.5, plan_secs: 0.12, skill: 1.0 }
}

pub fn page_pairup() -> PairupK {
    crate::knobs::PairupK { on: false }
}

pub fn page_spotted() -> SpottedK {
    crate::knobs::SpottedK { on: false, leash_m: 7.0, slip_m: 3.2, lit_at: 0.3, slip_wait: 1.5, wary_m: 14.0, ear_m: 5.0, linger_secs: 1.5, brace_secs: 3.0, take_m: 2.2, take_reach: 1.05, take_secs: 0.6, fresh_secs: 2.5, caught_m: 30.0, last_secs: 6.0, step_m: 3.2, face_deg: 35.0, close_m: 6.0 }
}

pub fn page_nightbreach() -> NightbreachK {
    crate::knobs::NightbreachK { def_lights: false, sam_at: [9.6, 0.0, 20.3], sam_yaw: 1.5707963267948966, squad: 2.0 }
}

pub fn page_roomclear() -> RoomclearK {
    crate::knobs::RoomclearK { seed: 0.0, way: 0.0 }
}

pub fn page_verdict() -> VerdictK {
    crate::knobs::VerdictK { win_secs: 2.0, loss_secs: 2.6 }
}

pub fn page_unseen() -> UnseenK {
    crate::knobs::UnseenK { pace_at: 0.5, linger: 1.5, min_pace: 0.5, low_pace: 1.45, gain: 1.5, cap: 1.6, wall_off: 0.42, wall_out: 0.4, wall_cross: 0.5, wall_end: 1.6, low_step: 0.14, walk_step: 0.2, cue_gap: 30.0, wall_secs: 0.8 }
}

pub fn page_seenwalk() -> SeenwalkK {
    crate::knobs::SeenwalkK { lit_at: 0.1, dear: 6.0, long: 1.35, less: 1.5, eye_step: 4.0, eye_y: 1.6, margin: 8.0 }
}

pub fn page_holdfire() -> HoldfireK {
    crate::knobs::HoldfireK { lift_secs: 45.0 }
}

pub fn page_slots() -> SlotsK {
    crate::knobs::SlotsK { on: true, table: crate::kit::SlotTable { v: 10.0, pouches: alloc::vec![4.0, 4.0, 4.0, 4.0, 4.0], big: alloc::vec![1.0, 1.0, 1.0, 1.0, 1.0], squad: None, items: { let mut t = JsMap::new(); t.set(String::from("smoke"), crate::kit::SlotItem { id: String::from("smoke"), pouches: 1.0, max: alloc::vec![4.0, 2.0, 2.0, 2.0, 2.0], squad_max: None, big: None, holds: None }); t.set(String::from("bangs"), crate::kit::SlotItem { id: String::from("stun"), pouches: 1.0, max: alloc::vec![4.0, 3.0, 3.0, 3.0, 3.0], squad_max: None, big: None, holds: None }); t.set(String::from("frags"), crate::kit::SlotItem { id: String::from("frag"), pouches: 1.0, max: alloc::vec![3.0, 2.0, 2.0, 2.0, 2.0], squad_max: None, big: None, holds: None }); t.set(String::from("shells"), crate::kit::SlotItem { id: String::from("shell40"), pouches: 1.0, max: alloc::vec![4.0, 1.0, 1.0, 1.0, 1.0], squad_max: None, big: None, holds: None }); t.set(String::from("charges"), crate::kit::SlotItem { id: String::from("doorCharge"), pouches: 1.0, max: alloc::vec![2.0, 2.0, 2.0, 2.0, 2.0], squad_max: None, big: None, holds: None }); t.set(String::from("walls"), crate::kit::SlotItem { id: String::from("wallCharge"), pouches: 2.0, max: alloc::vec![2.0, 2.0, 2.0, 2.0, 2.0], squad_max: None, big: Some(true), holds: None }); t.set(String::from("shotgun"), crate::kit::SlotItem { id: String::from("shotgun"), pouches: 1.0, max: alloc::vec![1.0, 1.0, 1.0, 1.0, 1.0], squad_max: None, big: Some(true), holds: None }); t.set(String::from("wand"), crate::kit::SlotItem { id: String::from("wand"), pouches: 1.0, max: alloc::vec![1.0, 1.0, 1.0, 1.0, 1.0], squad_max: None, big: None, holds: None }); t.set(String::from("cams"), crate::kit::SlotItem { id: String::from("stickyCam"), pouches: 1.0, max: alloc::vec![3.0, 0.0, 0.0, 0.0, 0.0], squad_max: None, big: None, holds: None }); t.set(String::from("rings"), crate::kit::SlotItem { id: String::from("sponge"), pouches: 1.0, max: alloc::vec![5.0, 0.0, 0.0, 0.0, 0.0], squad_max: None, big: None, holds: Some(5.0) }); t.set(String::from("hammer"), crate::kit::SlotItem { id: String::from("hammer"), pouches: 2.0, max: alloc::vec![1.0, 1.0, 1.0, 1.0, 1.0], squad_max: None, big: Some(true), holds: None }); t }, unknown: Vec::new(), loadouts: alloc::vec![crate::kit::SlotLoadout { id: String::from("wall"), name: String::from("One wall charge"), counts: alloc::vec![{ let mut t = JsMap::new(); t.set(String::from("smoke"), 1.0); t.set(String::from("bangs"), 1.0); t.set(String::from("frags"), 1.0); t.set(String::from("shells"), 1.0); t }, { let mut t = JsMap::new(); t.set(String::from("walls"), 1.0); t.set(String::from("bangs"), 2.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 1.0); t.set(String::from("frags"), 2.0); t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], cards: alloc::vec![{ let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], missing: Vec::new() }, crate::kit::SlotLoadout { id: String::from("stun"), name: String::from("Stun heavy"), counts: alloc::vec![{ let mut t = JsMap::new(); t.set(String::from("smoke"), 1.0); t.set(String::from("bangs"), 2.0); t.set(String::from("shells"), 1.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 3.0); t.set(String::from("charges"), 1.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 3.0); t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], cards: alloc::vec![{ let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], missing: Vec::new() }, crate::kit::SlotLoadout { id: String::from("frag"), name: String::from("Frag heavy"), counts: alloc::vec![{ let mut t = JsMap::new(); t.set(String::from("smoke"), 1.0); t.set(String::from("bangs"), 1.0); t.set(String::from("frags"), 1.0); t.set(String::from("shells"), 1.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 2.0); t.set(String::from("frags"), 2.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 1.0); t.set(String::from("frags"), 2.0); t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], cards: alloc::vec![{ let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], missing: Vec::new() }, crate::kit::SlotLoadout { id: String::from("quiet"), name: String::from("Quiet"), counts: alloc::vec![{ let mut t = JsMap::new(); t.set(String::from("smoke"), 3.0); t.set(String::from("frags"), 1.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 2.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 1.0); t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], cards: alloc::vec![{ let mut t = JsMap::new(); t.set(String::from("weapon"), crate::kit::KitValue::Text(String::from("pistol"))); t.set(String::from("shells"), crate::kit::KitValue::Num(0.0)); t }, { let mut t = JsMap::new(); t.set(String::from("weapon"), crate::kit::KitValue::Text(String::from("pistol"))); t.set(String::from("suppressed"), crate::kit::KitValue::Bool(true)); t.set(String::from("light"), crate::kit::KitValue::Bool(false)); t }, { let mut t = JsMap::new(); t.set(String::from("weapon"), crate::kit::KitValue::Text(String::from("pistol"))); t.set(String::from("suppressed"), crate::kit::KitValue::Bool(true)); t.set(String::from("light"), crate::kit::KitValue::Bool(false)); t }, { let mut t = JsMap::new(); t.set(String::from("weapon"), crate::kit::KitValue::Text(String::from("pistol"))); t.set(String::from("suppressed"), crate::kit::KitValue::Bool(true)); t.set(String::from("light"), crate::kit::KitValue::Bool(false)); t }, { let mut t = JsMap::new(); t.set(String::from("weapon"), crate::kit::KitValue::Text(String::from("pistol"))); t.set(String::from("suppressed"), crate::kit::KitValue::Bool(true)); t.set(String::from("light"), crate::kit::KitValue::Bool(false)); t }], missing: Vec::new() }] } }
}

pub fn page_destruction() -> DestructionK {
    crate::knobs::DestructionK { on: true }
}

pub fn page_bigslot() -> BigslotK {
    crate::knobs::BigslotK { on: true, swings: 2.0, swing: 0.7, shells: 2.0, reach: 2.4, heft: 0.7, equip_w: 0.8, draw_w: 0.7, stow_w: 0.75, flow: 0.2 }
}

/// every knob table
#[derive(Clone, Debug, ToBytes)]
pub struct Knobs {
    pub aim: AimK,
    pub trained: TrainedK,
    pub body_light: BodyLightK,
    pub guard_eye: GuardEyeK,
    pub needs: NeedsK,
    pub whistle: WhistleK,
    pub wet: WetK,
    pub wind: WindK,
    pub civ: CivK,
    pub chaos: ChaosK,
    pub ring: RingK,
    pub launcher: LauncherK,
    pub he40: He40K,
    pub launcher40: Launcher40K,
    pub frag: FragK,
    pub shotgun: ShotgunK,
    pub glass: GlassK,
    pub glassbreak: GlassbreakK,
    pub glasspen: GlasspenK,
    pub shards: ShardsK,
    pub hold: HoldK,
    pub suppression: SuppressionK,
    pub rifle_spread: RifleSpreadK,
    pub footfall: FootfallK,
    pub twogears: TwogearsK,
    pub crouchrun: CrouchrunK,
    pub runready: RunreadyK,
    pub runarms: RunarmsK,
    pub samlook: SamlookK,
    pub dash: DashK,
    pub watch: WatchK,
    pub stalk: StalkK,
    pub overhear: OverhearK,
    pub score: ScoreK,
    pub badge: BadgeK,
    pub ckpt: CkptK,
    pub fog: FogK,
    pub moves: MovesK,
    pub mv: MvK,
    pub turn: TurnK,
    pub doorpath: DoorpathK,
    pub walls: WallsK,
    pub wallcharge: WallchargeK,
    pub wallshot: WallshotK,
    pub wallkill: WallkillK,
    pub hardbreath: HardbreathK,
    pub castor: CastorK,
    pub wallreact: WallreactK,
    pub coverfire: CoverfireK,
    pub doorblast: DoorblastK,
    pub doorshot: DoorshotK,
    pub doorthrow: DoorthrowK,
    pub winbars: WinbarsK,
    pub opening: OpeningK,
    pub handle: HandleK,
    pub doormanners: DoormannersK,
    pub breaks: BreaksK,
    pub furn: FurnK,
    pub snowbog: SnowbogK,
    pub rubble: RubbleK,
    pub debris: DebrisK,
    pub lightwall: LightwallK,
    pub airdust: AirdustK,
    pub wallsnap: WallsnapK,
    pub gunwall: GunwallK,
    pub close: CloseK,
    pub plan: PlanK,
    pub reload: ReloadK,
    pub samstun: SamstunK,
    pub recap: RecapK,
    pub aimweight: AimweightK,
    pub aimcost: AimcostK,
    pub friends: FriendsK,
    pub friendlyfire: FriendlyfireK,
    pub lightswitch: LightswitchK,
    pub carlamps: CarlampsK,
    pub wallskin: WallskinK,
    pub fireline: FirelineK,
    pub savetimeline: SavetimelineK,
    pub handring: HandringK,
    pub sprintdoor: SprintdoorK,
    pub teampace: TeampaceK,
    pub noshove: NoshoveK,
    pub rails: RailsK,
    pub doorforce: DoorforceK,
    pub slotsetup: SlotsetupK,
    pub glassreal: GlassrealK,
    pub quietfog: QuietfogK,
    pub told: ToldK,
    pub kitanim: KitanimK,
    pub kit: KitK,
    pub throw_by: ThrowByK,
    pub hands: HandsK,
    pub work: WorkK,
    pub wand: WandK,
    pub sticky: StickyK,
    pub peeksight: PeeksightK,
    pub gunslots: GunslotsK,
    pub gunroles: GunrolesK,
    pub kittree: KittreeK,
    pub squadthrow: SquadthrowK,
    pub squad40: Squad40K,
    pub autopilot: AutopilotK,
    pub cmdring: CmdringK,
    pub torch: TorchK,
    pub backkey: BackkeyK,
    pub morale: MoraleK,
    pub condition: ConditionK,
    pub inhand: InhandK,
    pub packside: PacksideK,
    pub runner: RunnerK,
    pub startle: StartleK,
    pub smokebreak: SmokebreakK,
    pub showwhy: ShowwhyK,
    pub fire: FireK,
    pub flinch: FlinchK,
    pub darkdoor: DarkdoorK,
    pub darkfoe: DarkfoeK,
    pub powerout: PoweroutK,
    pub squaddeal: SquaddealK,
    pub stayask: StayaskK,
    pub rescuecheck: RescuecheckK,
    pub kitload: KitloadK,
    pub rescueend: RescueendK,
    pub rescueflush: RescueflushK,
    pub posemath: PosemathK,
    pub sense: SenseK,
    pub cmdcheck: CmdcheckK,
    pub seats: SeatsK,
    pub doze: DozeK,
    pub netquiet: NetquietK,
    pub attention: AttentionK,
    pub picture: PictureK,
    pub response: ResponseK,
    pub handwork: HandworkK,
    pub workstow: WorkstowK,
    pub squadnet: SquadnetK,
    pub takedown: TakedownK,
    pub stealthpose: StealthposeK,
    pub look: LookK,
    pub lived: LivedK,
    pub env: EnvK,
    pub shapes: ShapesK,
    pub flash: FlashK,
    pub pace: PaceK,
    pub lights: LightsK,
    pub nvg: NvgK,
    pub irlight: IrlightK,
    pub wallpen: WallpenK,
    pub doortools: DoortoolsK,
    pub roomfight: RoomfightK,
    pub orders: OrdersK,
    pub cover: CoverK,
    pub roomfit: RoomfitK,
    pub entry: EntryK,
    pub rear: RearK,
    pub silent: SilentK,
    pub pair: PairK,
    pub rescue: RescueK,
    pub digin: DiginK,
    pub pressure: PressureK,
    pub lanes: LanesK,
    pub gunhold: GunholdK,
    pub hard: HardK,
    pub hard_at: HardAtK,
    pub drilled: DrilledK,
    pub squads: SquadsK,
    pub drill_at: DrillAtK,
    pub cues: CuesK,
    pub wx_sim: WxSimK,
    pub air_sim: AirSimK,
    pub laststand: LaststandK,
    pub stackroom: StackroomK,
    pub rearwatch: RearwatchK,
    pub stackline: StacklineK,
    pub sneak: SneakK,
    pub pairup: PairupK,
    pub spotted: SpottedK,
    pub nightbreach: NightbreachK,
    pub roomclear: RoomclearK,
    pub verdict: VerdictK,
    pub unseen: UnseenK,
    pub seenwalk: SeenwalkK,
    pub holdfire: HoldfireK,
    pub slots: SlotsK,
    pub destruction: DestructionK,
    pub bigslot: BigslotK,
}
impl Knobs {
    /// THE PAGE'S values, as a game starts
    pub fn page() -> Self {
        Knobs {
            aim: page_aim(),
            trained: page_trained(),
            body_light: page_body_light(),
            guard_eye: page_guard_eye(),
            needs: page_needs(),
            whistle: page_whistle(),
            wet: page_wet(),
            wind: page_wind(),
            civ: page_civ(),
            chaos: page_chaos(),
            ring: page_ring(),
            launcher: page_launcher(),
            he40: page_he40(),
            launcher40: page_launcher40(),
            frag: page_frag(),
            shotgun: page_shotgun(),
            glass: page_glass(),
            glassbreak: page_glassbreak(),
            glasspen: page_glasspen(),
            shards: page_shards(),
            hold: page_hold(),
            suppression: page_suppression(),
            rifle_spread: page_rifle_spread(),
            footfall: page_footfall(),
            twogears: page_twogears(),
            crouchrun: page_crouchrun(),
            runready: page_runready(),
            runarms: page_runarms(),
            samlook: page_samlook(),
            dash: page_dash(),
            watch: page_watch(),
            stalk: page_stalk(),
            overhear: page_overhear(),
            score: page_score(),
            badge: page_badge(),
            ckpt: page_ckpt(),
            fog: page_fog(),
            moves: page_moves(),
            mv: page_mv(),
            turn: page_turn(),
            doorpath: page_doorpath(),
            walls: page_walls(),
            wallcharge: page_wallcharge(),
            wallshot: page_wallshot(),
            wallkill: page_wallkill(),
            hardbreath: page_hardbreath(),
            castor: page_castor(),
            wallreact: page_wallreact(),
            coverfire: page_coverfire(),
            doorblast: page_doorblast(),
            doorshot: page_doorshot(),
            doorthrow: page_doorthrow(),
            winbars: page_winbars(),
            opening: page_opening(),
            handle: page_handle(),
            doormanners: page_doormanners(),
            breaks: page_breaks(),
            furn: page_furn(),
            snowbog: page_snowbog(),
            rubble: page_rubble(),
            debris: page_debris(),
            lightwall: page_lightwall(),
            airdust: page_airdust(),
            wallsnap: page_wallsnap(),
            gunwall: page_gunwall(),
            close: page_close(),
            plan: page_plan(),
            reload: page_reload(),
            samstun: page_samstun(),
            recap: page_recap(),
            aimweight: page_aimweight(),
            aimcost: page_aimcost(),
            friends: page_friends(),
            friendlyfire: page_friendlyfire(),
            lightswitch: page_lightswitch(),
            carlamps: page_carlamps(),
            wallskin: page_wallskin(),
            fireline: page_fireline(),
            savetimeline: page_savetimeline(),
            handring: page_handring(),
            sprintdoor: page_sprintdoor(),
            teampace: page_teampace(),
            noshove: page_noshove(),
            rails: page_rails(),
            doorforce: page_doorforce(),
            slotsetup: page_slotsetup(),
            glassreal: page_glassreal(),
            quietfog: page_quietfog(),
            told: page_told(),
            kitanim: page_kitanim(),
            kit: page_kit(),
            throw_by: page_throw_by(),
            hands: page_hands(),
            work: page_work(),
            wand: page_wand(),
            sticky: page_sticky(),
            peeksight: page_peeksight(),
            gunslots: page_gunslots(),
            gunroles: page_gunroles(),
            kittree: page_kittree(),
            squadthrow: page_squadthrow(),
            squad40: page_squad40(),
            autopilot: page_autopilot(),
            cmdring: page_cmdring(),
            torch: page_torch(),
            backkey: page_backkey(),
            morale: page_morale(),
            condition: page_condition(),
            inhand: page_inhand(),
            packside: page_packside(),
            runner: page_runner(),
            startle: page_startle(),
            smokebreak: page_smokebreak(),
            showwhy: page_showwhy(),
            fire: page_fire(),
            flinch: page_flinch(),
            darkdoor: page_darkdoor(),
            darkfoe: page_darkfoe(),
            powerout: page_powerout(),
            squaddeal: page_squaddeal(),
            stayask: page_stayask(),
            rescuecheck: page_rescuecheck(),
            kitload: page_kitload(),
            rescueend: page_rescueend(),
            rescueflush: page_rescueflush(),
            posemath: page_posemath(),
            sense: page_sense(),
            cmdcheck: page_cmdcheck(),
            seats: page_seats(),
            doze: page_doze(),
            netquiet: page_netquiet(),
            attention: page_attention(),
            picture: page_picture(),
            response: page_response(),
            handwork: page_handwork(),
            workstow: page_workstow(),
            squadnet: page_squadnet(),
            takedown: page_takedown(),
            stealthpose: page_stealthpose(),
            look: page_look(),
            lived: page_lived(),
            env: page_env(),
            shapes: page_shapes(),
            flash: page_flash(),
            pace: page_pace(),
            lights: page_lights(),
            nvg: page_nvg(),
            irlight: page_irlight(),
            wallpen: page_wallpen(),
            doortools: page_doortools(),
            roomfight: page_roomfight(),
            orders: page_orders(),
            cover: page_cover(),
            roomfit: page_roomfit(),
            entry: page_entry(),
            rear: page_rear(),
            silent: page_silent(),
            pair: page_pair(),
            rescue: page_rescue(),
            digin: page_digin(),
            pressure: page_pressure(),
            lanes: page_lanes(),
            gunhold: page_gunhold(),
            hard: page_hard(),
            hard_at: page_hard_at(),
            drilled: page_drilled(),
            squads: page_squads(),
            drill_at: page_drill_at(),
            cues: page_cues(),
            wx_sim: page_wx_sim(),
            air_sim: page_air_sim(),
            laststand: page_laststand(),
            stackroom: page_stackroom(),
            rearwatch: page_rearwatch(),
            stackline: page_stackline(),
            sneak: page_sneak(),
            pairup: page_pairup(),
            spotted: page_spotted(),
            nightbreach: page_nightbreach(),
            roomclear: page_roomclear(),
            verdict: page_verdict(),
            unseen: page_unseen(),
            seenwalk: page_seenwalk(),
            holdfire: page_holdfire(),
            slots: page_slots(),
            destruction: page_destruction(),
            bigslot: page_bigslot(),
        }
    }
}
