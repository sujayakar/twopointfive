//! The knob tables, at THE PAGE'S values: the numbers the page opened with (page-values), every yes/no row from port-list.json. Written by the skeleton's program.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/aim.ts:31 AIM, src/game/aim.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct AimK {
    pub guard: crate::knobs::AimKGuard,
    pub player: crate::knobs::AimKPlayer,
}
impl AimK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AimK {
            guard: crate::knobs::AimKGuard::zeroed(),
            player: crate::knobs::AimKPlayer::zeroed(),
        }
    }
}
impl Default for AimK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aim.ts:32 AimK.guard, src/game/aim.ts:32)
#[derive(Clone, Copy, Debug)]
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
impl AimKGuard {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AimKGuard {
            settle_secs: 0.0,
            range_ref: 0.0,
            near_k: 0.0,
            acquire_min: 0.0,
            unsettled_k: 0.0,
            on_deg: 0.0,
            lose_rate: 0.0,
            mem_secs: 0.0,
            jump_deg: 0.0,
            target_move_k: 0.0,
            target_move_cap: 0.0,
            rate_tau: 0.0,
            shooter_move_k: 0.0,
            feet_tau: 0.0,
            steady_pace: 0.0,
            stop_to_shoot: 0.0,
            moving_track_cap: 0.0,
            shoot_band: 0.0,
            brake_lead: 0.0,
        }
    }
}
impl Default for AimKGuard { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aim.ts:64 AimK.player, src/game/aim.ts:64)
#[derive(Clone, Copy, Debug)]
pub struct AimKPlayer {
    pub settle_secs: f64,
    pub range_ref: f64,
    pub near_k: f64,
    pub unsettled_k: f64,
    pub lose_rate: f64,
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
    pub level: f64,
    pub low: f64,
}
impl AimKPlayer {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AimKPlayer {
            settle_secs: 0.0,
            range_ref: 0.0,
            near_k: 0.0,
            unsettled_k: 0.0,
            lose_rate: 0.0,
            base_stand: 0.0,
            base_crouch: 0.0,
            target_move_k: 0.0,
            target_move_cap: 0.0,
            rate_tau: 0.0,
            jump_deg: 0.0,
            shooter_move_k: 0.0,
            feet_tau: 0.0,
            steady_pace: 0.0,
            sprint_cap: 0.0,
            walk_cap: 0.0,
            moving_above: 0.0,
            crouch_k: 0.0,
            light_snap_px: 0.0,
            level: 0.0,
            low: 0.0,
        }
    }
}
impl Default for AimKPlayer { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aim.ts:95 TRAINED, src/game/aim.ts:95)
#[derive(Clone, Copy, Debug)]
pub struct TrainedK {
    pub settle_k: f64,
    pub spread_k: f64,
    pub react_cap: f64,
}
impl TrainedK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TrainedK {
            settle_k: 0.0,
            spread_k: 0.0,
            react_cap: 0.0,
        }
    }
}
impl Default for TrainedK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:86 AI_TUNE, src/game/guards.ts:86)
#[derive(Clone, Copy, Debug)]
pub struct AiTuneK {
    pub motion_acuity: bool,
    pub dark_search: bool,
    pub pieing: bool,
}
impl AiTuneK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AiTuneK {
            motion_acuity: false,
            dark_search: false,
            pieing: false,
        }
    }
}
impl Default for AiTuneK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:364 BODY_LIGHT, src/game/guards.ts:364)
#[derive(Clone, Copy, Debug)]
pub struct BodyLightK {
    pub lit: f64,
    pub lit_near: f64,
    pub near_m: f64,
    pub far_m: f64,
    pub smoke: f64,
}
impl BodyLightK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BodyLightK {
            lit: 0.0,
            lit_near: 0.0,
            near_m: 0.0,
            far_m: 0.0,
            smoke: 0.0,
        }
    }
}
impl Default for BodyLightK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1260 GUARD_EYE, src/game/guards.ts:1260)
#[derive(Clone, Copy, Debug)]
pub struct GuardEyeK {
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
    pub far: bool,
    pub far_calm: bool,
    pub far_fix_at: f64,
    pub far_range: f64,
    pub far_lo: f64,
    pub far_hi: f64,
    pub far_k: f64,
}
impl GuardEyeK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardEyeK {
            range: 0.0,
            half_deg: 0.0,
            near_m: 0.0,
            near_deg: 0.0,
            far: false,
            far_calm: false,
            far_fix_at: 0.0,
            far_range: 0.0,
            far_lo: 0.0,
            far_hi: 0.0,
            far_k: 0.0,
        }
    }
}
impl Default for GuardEyeK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:56 NEEDS, src/game/needs.ts:56)
#[derive(Clone, Debug)]
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
    pub from_nearest: bool,
    pub min_trip_m: f64,
    pub after_cancel: f64,
    pub after_alert: f64,
    pub rest_secs: f64,
    pub floor_gap_secs: f64,
    pub pace_k: f64,
    pub stuck_secs: f64,
}
impl NeedsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedsK {
            rate_k: 0.0,
            max_away: 0.0,
            announce_lead: 0.0,
            say_lead: 0.0,
            stretch_lead: 0.0,
            partner_m: 0.0,
            reply_after: 0.0,
            dwell: JsMap::new(),
            min_wait: 0.0,
            from_nearest: false,
            min_trip_m: 0.0,
            after_cancel: 0.0,
            after_alert: 0.0,
            rest_secs: 0.0,
            floor_gap_secs: 0.0,
            pace_k: 0.0,
            stuck_secs: 0.0,
        }
    }
}
impl Default for NeedsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/whistle.ts:30 WHISTLE, src/game/whistle.ts:30)
#[derive(Clone, Copy, Debug)]
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
impl WhistleK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WhistleK {
            level: 0.0,
            reach_k: 0.0,
            horizon: 0.0,
            cooldown: 0.0,
            twice_window: 0.0,
            awareness: 0.0,
            react_t: 0.0,
            walk_floor: 0.0,
            glance_secs: 0.0,
            glance_max: 0.0,
            gain: 0.0,
        }
    }
}
impl Default for WhistleK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wet.ts:47 WET, src/game/wet.ts:29)
#[derive(Clone, Copy, Debug)]
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
impl WetK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WetK {
            soak_per_sec: 0.0,
            dry_per_sec: 0.0,
            bead_runoff_secs: 0.0,
            bead_density: 0.0,
            bead_size: 0.0,
            trail_len: 0.0,
            rough_wet: 0.0,
            rough_dry: 0.0,
            darken: 0.0,
            darken_mul: 0.0,
            spec_gain: 0.0,
            drip_secs: 0.0,
            prints: 0.0,
            line_band: 0.0,
            print_fade_secs: 0.0,
        }
    }
}
impl Default for WetK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wind.ts:37 WIND, src/game/wind.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct WindK {
    pub def: f64,
    pub vents: bool,
    pub vent_k: f64,
    pub storm: bool,
    pub storm_k: f64,
    pub doors: bool,
    pub door_k: f64,
    pub door_reach: f64,
    pub door_spread_deg: f64,
    pub door_open_rad: f64,
    pub gusts: bool,
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
impl WindK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WindK {
            def: 0.0,
            vents: false,
            vent_k: 0.0,
            storm: false,
            storm_k: 0.0,
            doors: false,
            door_k: 0.0,
            door_reach: 0.0,
            door_spread_deg: 0.0,
            door_open_rad: 0.0,
            gusts: false,
            gust_amp: 0.0,
            gust_period: 0.0,
            storm_gust_amp: 0.0,
            storm_gust_period: 0.0,
            ramp_up: 0.0,
            ramp_down: 0.0,
            smoke_relax: 0.0,
            sheet_wake: 0.0,
            sheet_kick: 0.0,
            debris_k: 0.0,
            near_m: 0.0,
        }
    }
}
impl Default for WindK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:80 CIV, src/game/civilians.ts:80)
#[derive(Clone, Copy, Debug)]
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
    pub greet: bool,
    pub greet_m: f64,
    pub greet_pair_secs: f64,
    pub greet_floor_secs: f64,
    pub evacuate_on_qrf: bool,
}
impl CivK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivK {
            eye: crate::knobs::CivKEye::zeroed(),
            eye_busy: crate::knobs::CivKEyeBusy::zeroed(),
            eye_alarmed: crate::knobs::CivKEyeAlarmed::zeroed(),
            keen_k: 0.0,
            gun_k: 0.0,
            gun_m: 0.0,
            glimpse_at: 0.0,
            spot_at: 0.0,
            startle_secs: 0.0,
            startle_again_secs: 0.0,
            body_m: 0.0,
            freeze: [0.0; 2],
            run_speed: 0.0,
            walk_speed: 0.0,
            creep_speed: 0.0,
            report_m: 0.0,
            report_secs: 0.0,
            phone_secs: 0.0,
            guard_bonus_m: 0.0,
            exit_malus_m: 0.0,
            past_threat_m: 0.0,
            corner_m: 0.0,
            reflee_m: 0.0,
            lone_secs: 0.0,
            settle_secs: 0.0,
            comply_min: 0.0,
            scream_m: 0.0,
            see_m: 0.0,
            see_lit: 0.0,
            found_m: 0.0,
            found_secs: 0.0,
            outside_call_secs: 0.0,
            greet: false,
            greet_m: 0.0,
            greet_pair_secs: 0.0,
            greet_floor_secs: 0.0,
            evacuate_on_qrf: false,
        }
    }
}
impl Default for CivK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:82 CivK.eye, src/game/civilians.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct CivKEye {
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
}
impl CivKEye {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivKEye {
            range: 0.0,
            half_deg: 0.0,
            near_m: 0.0,
            near_deg: 0.0,
        }
    }
}
impl Default for CivKEye { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:83 CivK.eyeBusy, src/game/civilians.ts:83)
#[derive(Clone, Copy, Debug)]
pub struct CivKEyeBusy {
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
}
impl CivKEyeBusy {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivKEyeBusy {
            range: 0.0,
            half_deg: 0.0,
            near_m: 0.0,
            near_deg: 0.0,
        }
    }
}
impl Default for CivKEyeBusy { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:84 CivK.eyeAlarmed, src/game/civilians.ts:84)
#[derive(Clone, Copy, Debug)]
pub struct CivKEyeAlarmed {
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
}
impl CivKEyeAlarmed {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivKEyeAlarmed {
            range: 0.0,
            half_deg: 0.0,
            near_m: 0.0,
            near_deg: 0.0,
        }
    }
}
impl Default for CivKEyeAlarmed { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:66 CHAOS, src/game/destruction.ts:66)
#[derive(Clone, Copy, Debug)]
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
impl ChaosK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosK {
            def: 0.0,
            heat_tau: 0.0,
            cell: 0.0,
            t_decal: 0.0,
            t_paper: 0.0,
            t_burst: 0.0,
            t_puff: 0.0,
            t_knock: 0.0,
            t_tile: 0.0,
            bang_heat: 0.0,
            puffs_per_sec: 0.0,
            puffs_per_frame: 0.0,
            papers_per_sec: 0.0,
            knocks_per_sec: 0.0,
            tile_gap_secs: 0.0,
            tiles_per_encounter: 0.0,
            decals_per_sec: 0.0,
            paper_reach_m: 0.0,
            knock_reach_m: 0.0,
            sheets_per_stack: 0.0,
            puff: crate::knobs::ChaosKPuff::zeroed(),
            tile: crate::knobs::ChaosKTile::zeroed(),
            vel_low: 0.0,
            vel_high_k: 0.0,
            imp: crate::knobs::ChaosKImp::zeroed(),
            loud: crate::knobs::ChaosKLoud::zeroed(),
            pall: crate::knobs::ChaosKPall::zeroed(),
            mess: crate::knobs::ChaosKMess::zeroed(),
        }
    }
}
impl Default for ChaosK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:78 ChaosK.puff, src/game/destruction.ts:78)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKPuff {
    pub radius: f64,
    pub density: f64,
    pub ttl: f64,
    pub speed: f64,
    pub temp: f64,
    pub push: f64,
    pub expand: f64,
}
impl ChaosKPuff {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKPuff {
            radius: 0.0,
            density: 0.0,
            ttl: 0.0,
            speed: 0.0,
            temp: 0.0,
            push: 0.0,
            expand: 0.0,
        }
    }
}
impl Default for ChaosKPuff { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:79 ChaosK.tile, src/game/destruction.ts:79)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKTile {
    pub half: V3,
    pub albedo: V3,
}
impl ChaosKTile {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKTile {
            half: [0.0; 3],
            albedo: [0.0; 3],
        }
    }
}
impl Default for ChaosKTile { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:84 ChaosK.imp, src/game/destruction.ts:84)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKImp {
    pub bang: crate::knobs::ChaosKImpBang,
    pub impact: crate::knobs::ChaosKImpImpact,
    pub thud: crate::knobs::ChaosKImpThud,
    pub chaos_k: f64,
}
impl ChaosKImp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKImp {
            bang: crate::knobs::ChaosKImpBang::zeroed(),
            impact: crate::knobs::ChaosKImpImpact::zeroed(),
            thud: crate::knobs::ChaosKImpThud::zeroed(),
            chaos_k: 0.0,
        }
    }
}
impl Default for ChaosKImp { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:85 ChaosKImp.bang, src/game/destruction.ts:85)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKImpBang {
    pub radius: f64,
    pub strength: f64,
    pub up: f64,
}
impl ChaosKImpBang {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKImpBang {
            radius: 0.0,
            strength: 0.0,
            up: 0.0,
        }
    }
}
impl Default for ChaosKImpBang { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:86 ChaosKImp.impact, src/game/destruction.ts:86)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKImpImpact {
    pub radius: f64,
    pub strength: f64,
    pub up: f64,
    pub bias_k: f64,
    pub debris_m: f64,
}
impl ChaosKImpImpact {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKImpImpact {
            radius: 0.0,
            strength: 0.0,
            up: 0.0,
            bias_k: 0.0,
            debris_m: 0.0,
        }
    }
}
impl Default for ChaosKImpImpact { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:87 ChaosKImp.thud, src/game/destruction.ts:87)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKImpThud {
    pub radius: f64,
    pub strength: f64,
    pub up: f64,
    pub per_frame: f64,
}
impl ChaosKImpThud {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKImpThud {
            radius: 0.0,
            strength: 0.0,
            up: 0.0,
            per_frame: 0.0,
        }
    }
}
impl Default for ChaosKImpThud { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:92 ChaosK.loud, src/game/destruction.ts:92)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKLoud {
    pub fixture_hit_m: f64,
    pub fixture_blast_m: f64,
    pub stun_fixture_m: f64,
    pub stun_panels: bool,
    pub fixtures_per_encounter: f64,
    pub fixture_gap_secs: f64,
    pub blast_dust_floor: f64,
    pub blast_dust_puffs: f64,
    pub heat_k: f64,
    pub burst_queue: f64,
    pub burst_secs: f64,
}
impl ChaosKLoud {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKLoud {
            fixture_hit_m: 0.0,
            fixture_blast_m: 0.0,
            stun_fixture_m: 0.0,
            stun_panels: false,
            fixtures_per_encounter: 0.0,
            fixture_gap_secs: 0.0,
            blast_dust_floor: 0.0,
            blast_dust_puffs: 0.0,
            heat_k: 0.0,
            burst_queue: 0.0,
            burst_secs: 0.0,
        }
    }
}
impl Default for ChaosKLoud { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:121 ChaosK.pall, src/game/destruction.ts:121)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKPall {
    pub on: bool,
    pub solver: bool,
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
impl ChaosKPall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKPall {
            on: false,
            solver: false,
            per_window: 0.0,
            window_secs: 0.0,
            slack: 0.0,
            clear: 0.0,
            clear_y: 0.0,
            min_radius: 0.0,
            max_gain: 0.0,
            wall: crate::knobs::ChaosKPallWall::zeroed(),
            burst: crate::knobs::ChaosKPallBurst::zeroed(),
        }
    }
}
impl Default for ChaosKPall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:123 ChaosKPall.wall, src/game/destruction.ts:123)
#[derive(Clone, Copy, Debug)]
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
impl ChaosKPallWall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKPallWall {
            near: 0.0,
            far: 0.0,
            radius: 0.0,
            density: 0.0,
            ttl: 0.0,
            speed: 0.0,
            temp: 0.0,
            push: 0.0,
            expand: 0.0,
            spread: 0.0,
            out: 0.0,
            dust: [0.0; 2],
            dust_life: 0.0,
            dust_size: 0.0,
            dust_speed: 0.0,
        }
    }
}
impl Default for ChaosKPallWall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:124 ChaosKPall.burst, src/game/destruction.ts:124)
#[derive(Clone, Copy, Debug)]
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
impl ChaosKPallBurst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKPallBurst {
            ring: 0.0,
            core_gain: 0.0,
            radius: 0.0,
            density: 0.0,
            ttl: 0.0,
            speed: 0.0,
            temp: 0.0,
            push: 0.0,
            expand: 0.0,
            spread: 0.0,
            dust: 0.0,
            dust_life: 0.0,
            dust_size: 0.0,
            dust_speed: 0.0,
            smoke: [0.0; 3],
        }
    }
}
impl Default for ChaosKPallBurst { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:132 ChaosK.mess, src/game/destruction.ts:132)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKMess {
    pub on: bool,
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
impl ChaosKMess {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKMess {
            on: false,
            stand_y: 0.0,
            clear_y: 0.0,
            under: 0.0,
            min_radius: 0.0,
            crowd: 0.0,
            roof_y: 0.0,
            burst: crate::knobs::ChaosKMessBurst::zeroed(),
            wall: crate::knobs::ChaosKMessWall::zeroed(),
            foot: crate::knobs::ChaosKMessFoot::zeroed(),
            top: crate::knobs::ChaosKMessTop::zeroed(),
            rubble: crate::knobs::ChaosKMessRubble::zeroed(),
        }
    }
}
impl Default for ChaosKMess { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:138 ChaosKMess.burst, src/game/destruction.ts:138)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKMessBurst {
    pub core: f64,
    pub lobes: f64,
    pub reach: f64,
    pub lobe: f64,
    pub strength: f64,
}
impl ChaosKMessBurst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKMessBurst {
            core: 0.0,
            lobes: 0.0,
            reach: 0.0,
            lobe: 0.0,
            strength: 0.0,
        }
    }
}
impl Default for ChaosKMessBurst { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:140 ChaosKMess.wall, src/game/destruction.ts:140)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKMessWall {
    pub out: f64,
    pub near: crate::knobs::ChaosKMessWallNear,
    pub far: crate::knobs::ChaosKMessWallFar,
}
impl ChaosKMessWall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKMessWall {
            out: 0.0,
            near: crate::knobs::ChaosKMessWallNear::zeroed(),
            far: crate::knobs::ChaosKMessWallFar::zeroed(),
        }
    }
}
impl Default for ChaosKMessWall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:140 ChaosKMessWall.near, src/game/destruction.ts:140)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKMessWallNear {
    pub core: f64,
    pub lobes: f64,
    pub reach: f64,
    pub lobe: f64,
    pub strength: f64,
}
impl ChaosKMessWallNear {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKMessWallNear {
            core: 0.0,
            lobes: 0.0,
            reach: 0.0,
            lobe: 0.0,
            strength: 0.0,
        }
    }
}
impl Default for ChaosKMessWallNear { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:140 ChaosKMessWall.far, src/game/destruction.ts:140)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKMessWallFar {
    pub core: f64,
    pub lobes: f64,
    pub reach: f64,
    pub lobe: f64,
    pub strength: f64,
}
impl ChaosKMessWallFar {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKMessWallFar {
            core: 0.0,
            lobes: 0.0,
            reach: 0.0,
            lobe: 0.0,
            strength: 0.0,
        }
    }
}
impl Default for ChaosKMessWallFar { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:142 ChaosKMess.foot, src/game/destruction.ts:142)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKMessFoot {
    pub out: f64,
    pub radius: f64,
    pub per_metre: f64,
    pub strength: f64,
}
impl ChaosKMessFoot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKMessFoot {
            out: 0.0,
            radius: 0.0,
            per_metre: 0.0,
            strength: 0.0,
        }
    }
}
impl Default for ChaosKMessFoot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:144 ChaosKMess.top, src/game/destruction.ts:144)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKMessTop {
    pub lo: f64,
    pub hi: f64,
    pub strength: f64,
}
impl ChaosKMessTop {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKMessTop {
            lo: 0.0,
            hi: 0.0,
            strength: 0.0,
        }
    }
}
impl Default for ChaosKMessTop { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:146 ChaosKMess.rubble, src/game/destruction.ts:146)
#[derive(Clone, Copy, Debug)]
pub struct ChaosKMessRubble {
    pub least: f64,
    pub grow: f64,
    pub min: f64,
    pub max: f64,
    pub crowd: f64,
    pub strength: f64,
}
impl ChaosKMessRubble {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosKMessRubble {
            least: 0.0,
            grow: 0.0,
            min: 0.0,
            max: 0.0,
            crowd: 0.0,
            strength: 0.0,
        }
    }
}
impl Default for ChaosKMessRubble { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/launcher.ts:27 RING, src/game/launcher.ts:27)
#[derive(Clone, Copy, Debug)]
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
    pub lead: bool,
    pub solve_iters: f64,
}
impl RingK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RingK {
            speed: 0.0,
            gravity: 0.0,
            drag: 0.0,
            radius: 0.0,
            life: 0.0,
            substeps: 0.0,
            ko_below_neck: 0.0,
            stagger: 0.0,
            stagger_fire_cd: 0.0,
            stagger_aware: 0.0,
            stagger_back: 0.0,
            launch_noise: 0.0,
            clack_noise: 0.0,
            ko_noise: 0.0,
            hit_noise: 0.0,
            prop_j: 0.0,
            restitution: 0.0,
            keep_tangent: 0.0,
            spent_speed_max: 0.0,
            lead: false,
            solve_iters: 0.0,
        }
    }
}
impl Default for RingK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weapons.ts:337 LAUNCHER, src/game/weapons.ts:337)
#[derive(Clone, Copy, Debug)]
pub struct LauncherK {
    pub rings: f64,
    pub cd: f64,
}
impl LauncherK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LauncherK {
            rings: 0.0,
            cd: 0.0,
        }
    }
}
impl Default for LauncherK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/he40.ts:27 HE40, src/game/he40.ts:27)
#[derive(Clone, Copy, Debug)]
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
impl He40K {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        He40K {
            speed: 0.0,
            gravity: 0.0,
            drag: 0.0,
            radius: 0.0,
            life: 0.0,
            substeps: 0.0,
            solve_iters: 0.0,
            arm_m: 0.0,
            standoff: 0.0,
            launch_noise: 0.0,
            dud_noise: 0.0,
            dud_hit_noise: 0.0,
            prop_j: 0.0,
            restitution: 0.0,
            keep_tangent: 0.0,
            spent_speed_max: 0.0,
        }
    }
}
impl Default for He40K { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weapons.ts:340 LAUNCHER40, src/game/weapons.ts:340)
#[derive(Clone, Copy, Debug)]
pub struct Launcher40K {
    pub rounds: f64,
    pub cd: f64,
}
impl Launcher40K {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Launcher40K {
            rounds: 0.0,
            cd: 0.0,
        }
    }
}
impl Default for Launcher40K { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:74 FRAG, src/game/frag.ts:74)
#[derive(Clone, Debug)]
pub struct FragK {
    pub on: bool,
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
    pub sam_stun: bool,
    pub door_stun: bool,
    pub wall_stun: bool,
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
    pub flee_wise: bool,
    pub flee_keep: f64,
    pub rest_props: bool,
    pub flee_apart: bool,
    pub flee_apart_m: f64,
    pub flee_lane_m: f64,
    pub flee_onto: bool,
    pub flee_sam: bool,
    pub flee_look: bool,
    pub down_kill: bool,
    pub down_shove: f64,
    pub wall_flee: bool,
    pub flee_door: bool,
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
    pub defenders: bool,
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
impl FragK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FragK {
            on: false,
            issue: 0.0,
            fuse: 0.0,
            order_fuse: 0.0,
            squad: 0.0,
            order_key: String::new(),
            order_in: 0.0,
            lethal_m: 0.0,
            wound_m: 0.0,
            destr_m: 0.0,
            wound_kill: 0.0,
            stagger: [0.0; 2],
            lift: 0.0,
            sam_stun: false,
            door_stun: false,
            wall_stun: false,
            glass_m: 0.0,
            glass_power: 0.0,
            pane_energy: 0.0,
            see_m: 0.0,
            hear_m: 0.0,
            react: [0.0; 2],
            react_jit: 0.0,
            shout_m: 0.0,
            flee_m: 0.0,
            run_speed: 0.0,
            flee_wise: false,
            flee_keep: 0.0,
            rest_props: false,
            flee_apart: false,
            flee_apart_m: 0.0,
            flee_lane_m: 0.0,
            flee_onto: false,
            flee_sam: false,
            flee_look: false,
            down_kill: false,
            down_shove: 0.0,
            wall_flee: false,
            flee_door: false,
            flee_door_open: 0.0,
            flee_door_in: 0.0,
            flee_door_dip: 0.0,
            flee_door_mouth: 0.0,
            flee_door_leaf: 0.0,
            flee_door_via: 0.0,
            flee_door_via_m: 0.0,
            flee_door_new: 0.0,
            flee_door_most: 0.0,
            flee_door_try: 0.0,
            flee_door_nodes: 0.0,
            defenders: false,
            def_share: 0.0,
            def_max: 0.0,
            def_fuse: 0.0,
            def_wind: 0.0,
            def_near: 0.0,
            def_far: 0.0,
            def_past: 0.0,
            def_fresh: 0.0,
            def_blocked: 0.0,
            def_bunch: 0.0,
            def_bunch_secs: 0.0,
            def_gap: 0.0,
            def_own_m: 0.0,
            def_scatter: [0.0; 2],
            cue_m: 0.0,
        }
    }
}
impl Default for FragK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shotgun.ts:52 SHOTGUN, src/game/shotgun.ts:52)
#[derive(Clone, Debug)]
pub struct ShotgunK {
    pub on: bool,
    pub spots: Vec<[f64; 2]>,
    pub clear: f64,
    pub hinge: Vec<[f64; 2]>,
    pub mark_reach: f64,
    pub line: f64,
    pub own_beat: f64,
    pub own_slack: f64,
    pub own_side: bool,
    pub lock_in: f64,
    pub lock_h: f64,
    pub shells: f64,
    pub aim_cap: f64,
}
impl ShotgunK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShotgunK {
            on: false,
            spots: Vec::new(),
            clear: 0.0,
            hinge: Vec::new(),
            mark_reach: 0.0,
            line: 0.0,
            own_beat: 0.0,
            own_slack: 0.0,
            own_side: false,
            lock_in: 0.0,
            lock_h: 0.0,
            shells: 0.0,
            aim_cap: 0.0,
        }
    }
}
impl Default for ShotgunK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glass.ts:23 GLASS, src/game/glass.ts:23)
#[derive(Clone, Copy, Debug)]
pub struct GlassK {
    pub hear_m: f64,
    pub hear_level: f64,
    pub bang_m: f64,
    pub bang_cracks: f64,
    pub r_min: f64,
    pub r_max: f64,
    pub crack_gap: f64,
    pub live_gap: f64,
    pub both_faces: bool,
    pub ear_off: f64,
}
impl GlassK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassK {
            hear_m: 0.0,
            hear_level: 0.0,
            bang_m: 0.0,
            bang_cracks: 0.0,
            r_min: 0.0,
            r_max: 0.0,
            crack_gap: 0.0,
            live_gap: 0.0,
            both_faces: false,
            ear_off: 0.0,
        }
    }
}
impl Default for GlassK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glass.ts:42 GLASSFX, src/game/glass.ts:42)
#[derive(Clone, Copy, Debug)]
pub struct GlassfxK {
    pub on: bool,
}
impl GlassfxK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassfxK {
            on: false,
        }
    }
}
impl Default for GlassfxK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassbreak.ts:53 GLASSBREAK, src/game/glassbreak.ts:53)
#[derive(Clone, Debug)]
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
impl GlassbreakK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassbreakK {
            max_sites: 0.0,
            break_e: crate::knobs::GlassbreakKBreakE::zeroed(),
            punch_e: 0.0,
            run_single_e: 0.0,
            run_star: 0.0,
            run_ring_secs: 0.0,
            crack_gap: 0.0,
            rho: JsMap::new(),
            radials: JsMap::new(),
            merge: 0.0,
            rz_min: 0.0,
            think: 0.0,
            stagger_per_m: 0.0,
            stagger_jit: 0.0,
            hang_area: 0.0,
            grip: crate::knobs::GlassbreakKGrip::zeroed(),
            stand_ny: 0.0,
            stand_foot: 0.0,
            balance_tol: 0.0,
            blast_m: 0.0,
            bang_power: 0.0,
            tooth_area: 0.0,
            tooth_k: 0.0,
            blast_run_k: 0.0,
            blast_dice_k: 0.0,
            blast_sheet_k: 0.0,
            rake_open: 0.0,
            rake_m: 0.0,
            dice_grain: 0.0,
            clump_m: 0.0,
            dice_hang: 0.0,
            dice_speed: 0.0,
            clumps_per_frame: 0.0,
            releases_per_frame: 0.0,
            out: 0.0,
            spin: 0.0,
            blast_out: 0.0,
            shard_albedo: [0.0; 3],
            chip_color: [0.0; 3],
            chips_per_m2: 0.0,
            chips_min: 0.0,
            dice_chips_per_m2: 0.0,
            crumb_every: 0.0,
            gust_speed_k: 0.0,
            walk_clear_h: 0.0,
        }
    }
}
impl Default for GlassbreakK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassbreak.ts:56 GlassbreakK.breakE, src/game/glassbreak.ts:56)
#[derive(Clone, Copy, Debug)]
pub struct GlassbreakKBreakE {
    pub annealed: f64,
    pub tempered: f64,
}
impl GlassbreakKBreakE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassbreakKBreakE {
            annealed: 0.0,
            tempered: 0.0,
        }
    }
}
impl Default for GlassbreakKBreakE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: inline at src/game/glassbreak.ts:75 GlassbreakK.grip, src/game/glassbreak.ts:75)
#[derive(Clone, Copy, Debug)]
pub struct GlassbreakKGrip {
    pub side: f64,
    pub top: f64,
    pub wedged: f64,
}
impl GlassbreakKGrip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassbreakKGrip {
            side: 0.0,
            top: 0.0,
            wedged: 0.0,
        }
    }
}
impl Default for GlassbreakKGrip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glasspen.ts:60 GLASSPEN, src/game/glasspen.ts:60)
#[derive(Clone, Debug)]
pub struct GlasspenK {
    pub on: bool,
    pub energy: JsMap<String, f64>,
    pub by_class: JsMap<String, f64>,
    pub default_energy: f64,
    pub cost: JsMap<crate::glassbreak::GlassType, f64>,
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
    pub stack_least: bool,
    pub stack_least_k: f64,
    pub fracture_k: f64,
    pub foes_through: bool,
}
impl GlasspenK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlasspenK {
            on: false,
            energy: JsMap::new(),
            by_class: JsMap::new(),
            default_energy: 0.0,
            cost: JsMap::new(),
            ref_thick: 0.0,
            oblique_max: 0.0,
            graze_cos: 0.0,
            min_energy: 0.0,
            max_panes: 0.0,
            deflect_deg: 0.0,
            bend_k: 0.0,
            hole_r: 0.0,
            hole_k: 0.0,
            holes: 0.0,
            shell_r: 0.0,
            stack_eyes: 0.0,
            stack_energy: 0.0,
            stack_least: false,
            stack_least_k: 0.0,
            fracture_k: 0.0,
            foes_through: false,
        }
    }
}
impl Default for GlasspenK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shards.ts:64 SHARDS, src/game/shards.ts:64)
#[derive(Clone, Debug)]
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
    pub rest_frost: bool,
    pub cut_edge: f64,
    pub cut_see: f64,
    pub cut_rim: f64,
    pub cut_anim: bool,
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
    pub by_fall: bool,
    pub underfoot: bool,
    pub crunch_full: f64,
    pub crunch_gain: f64,
    pub crunch_add: f64,
    pub grind: f64,
}
impl ShardsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShardsK {
            mid_per_m2: 0.0,
            mid_min: 0.0,
            mid_max: 0.0,
            mid_size: 0.0,
            mid_life: 0.0,
            fine_per_m2: 0.0,
            fine_min: 0.0,
            fine_max: 0.0,
            fine_size: 0.0,
            fine_life: 0.0,
            back_share: 0.0,
            hit_fine_min: 0.0,
            hit_back_share: 0.0,
            spall_out: 0.0,
            spall_back: 0.0,
            dice_per_m2: 0.0,
            dice_min: 0.0,
            dice_max: 0.0,
            dice_life: 0.0,
            dice_fine_share: 0.0,
            crater_per_m2: 0.0,
            crater_max: 0.0,
            exit_fines: 0.0,
            exit_speed: 0.0,
            speed: 0.0,
            live_cap: 0.0,
            cap_floor: 0.0,
            over_cap: 0.0,
            press_life: 0.0,
            press_from: 0.0,
            recs_per_frame: 0.0,
            defer_max: 0.0,
            plate_min_area: 0.0,
            plate_share: 0.0,
            child_min_area: 0.0,
            shatter_v0: 0.0,
            shatter_v1: 0.0,
            rest_crumbs: 0.0,
            crumb_size: 0.0,
            rest_frost: false,
            cut_edge: 0.0,
            cut_see: 0.0,
            cut_rim: 0.0,
            cut_anim: false,
            cut_body: 0.0,
            cut_see_rest: 0.0,
            cut_rest_body: 0.0,
            max_gen: 0.0,
            contact: crate::knobs::ShardsKContact::zeroed(),
            sounds_per_frame: 0.0,
            pending_max: 0.0,
            sound_gap: JsMap::new(),
            sound_thicken: 0.0,
            sound_owed_secs: 0.0,
            level: 0.0,
            event_gap: 0.0,
            by_fall: false,
            underfoot: false,
            crunch_full: 0.0,
            crunch_gain: 0.0,
            crunch_add: 0.0,
            grind: 0.0,
        }
    }
}
impl Default for ShardsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shards.ts:101 ShardsK.contact, src/game/shards.ts:101)
#[derive(Clone, Copy, Debug)]
pub struct ShardsKContact {
    pub keep_floor: f64,
    pub keep_wall: f64,
    pub rest: f64,
}
impl ShardsKContact {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShardsKContact {
            keep_floor: 0.0,
            keep_wall: 0.0,
            rest: 0.0,
        }
    }
}
impl Default for ShardsKContact { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1259 TALK_TUNE, src/game/dialogue.ts:1259)
#[derive(Clone, Copy, Debug)]
pub struct TalkTuneK {
    pub scripts: bool,
}
impl TalkTuneK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TalkTuneK {
            scripts: false,
        }
    }
}
impl Default for TalkTuneK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:95 HOLD, src/game/character.ts:95)
#[derive(Clone, Debug)]
pub struct HoldK {
    pub drop_gun: bool,
    pub regrab: f64,
    pub bare_grip: f64,
    pub bare_arch: f64,
    pub bare_elbow: f64,
    pub yank: crate::knobs::HoldKYank,
    pub lunge: crate::knobs::HoldKLunge,
    pub sway: crate::knobs::HoldKSway,
    pub apart: crate::knobs::HoldKApart,
    pub way_shape: bool,
    pub catch_shape: bool,
    pub catch: crate::knobs::HoldKCatch,
    pub hit_shape: bool,
    pub lay_ahead: f64,
    pub hit: crate::knobs::HoldKHit,
    pub disarm: bool,
    pub strip_first: bool,
    pub first: crate::knobs::HoldKFirst,
    pub strip: crate::knobs::HoldKStrip,
    pub one_facing: bool,
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
impl HoldK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldK {
            drop_gun: false,
            regrab: 0.0,
            bare_grip: 0.0,
            bare_arch: 0.0,
            bare_elbow: 0.0,
            yank: crate::knobs::HoldKYank::zeroed(),
            lunge: crate::knobs::HoldKLunge::zeroed(),
            sway: crate::knobs::HoldKSway::zeroed(),
            apart: crate::knobs::HoldKApart::zeroed(),
            way_shape: false,
            catch_shape: false,
            catch: crate::knobs::HoldKCatch::zeroed(),
            hit_shape: false,
            lay_ahead: 0.0,
            hit: crate::knobs::HoldKHit::zeroed(),
            disarm: false,
            strip_first: false,
            first: crate::knobs::HoldKFirst::zeroed(),
            strip: crate::knobs::HoldKStrip::zeroed(),
            one_facing: false,
            dist: 0.0,
            secs: crate::knobs::HoldKSecs::zeroed(),
            throat: [0.0; 3],
            via_l: [0.0; 3],
            over_l: [0.0; 3],
            tight: [0.0; 3],
            lean: 0.0,
            choke_lean: 0.0,
            head_pitch: 0.0,
            brace: 0.0,
            knife: crate::knobs::HoldKKnife::zeroed(),
            rest_l: [0.0; 3],
            claw_l: [0.0; 3],
            raised_r: [0.0; 3],
            via_up: [0.0; 3],
            claw_r: [0.0; 3],
            hang: Vec::new(),
            arch: 0.0,
            chin_up: 0.0,
            aside: 0.0,
            clav_up: 0.0,
            loll: 0.0,
            loll_roll: 0.0,
            lower: crate::knobs::HoldKLower::zeroed(),
            kill: crate::knobs::HoldKKill::zeroed(),
            beats: crate::knobs::HoldKBeats::zeroed(),
            haul: crate::knobs::HoldKHaul::zeroed(),
            sag: [0.0; 2],
            breath: crate::knobs::HoldKBreath::zeroed(),
            struggle: crate::knobs::HoldKStruggle::zeroed(),
            gun: crate::knobs::HoldKGun::zeroed(),
        }
    }
}
impl Default for HoldK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:109 HoldK.yank, src/game/character.ts:109)
#[derive(Clone, Copy, Debug)]
pub struct HoldKYank {
    pub arch: f64,
    pub sink: f64,
    pub in_: f64,
    pub hold: f64,
    pub out: f64,
}
impl HoldKYank {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKYank {
            arch: 0.0,
            sink: 0.0,
            in_: 0.0,
            hold: 0.0,
            out: 0.0,
        }
    }
}
impl Default for HoldKYank { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:115 HoldK.lunge, src/game/character.ts:115)
#[derive(Clone, Copy, Debug)]
pub struct HoldKLunge {
    pub dip: f64,
    pub back: f64,
    pub t: [f64; 3],
}
impl HoldKLunge {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKLunge {
            dip: 0.0,
            back: 0.0,
            t: [0.0; 3],
        }
    }
}
impl Default for HoldKLunge { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:115 HoldK.sway, src/game/character.ts:115)
#[derive(Clone, Copy, Debug)]
pub struct HoldKSway {
    pub m: f64,
    pub secs: f64,
    pub twist: f64,
    pub his: f64,
    pub in_: f64,
    pub out: f64,
}
impl HoldKSway {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKSway {
            m: 0.0,
            secs: 0.0,
            twist: 0.0,
            his: 0.0,
            in_: 0.0,
            out: 0.0,
        }
    }
}
impl Default for HoldKSway { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:115 HoldK.apart, src/game/character.ts:115)
#[derive(Clone, Copy, Debug)]
pub struct HoldKApart {
    pub side: f64,
    pub tip: f64,
}
impl HoldKApart {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKApart {
            side: 0.0,
            tip: 0.0,
        }
    }
}
impl Default for HoldKApart { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:135 HoldK.catch, src/game/character.ts:135)
#[derive(Clone, Debug)]
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
impl HoldKCatch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKCatch {
            via: [0.0; 3],
            over: [0.0; 3],
            at: [0.0; 2],
            arm_in: 0.0,
            ready_out: [0.0; 2],
            jolt: crate::knobs::HoldKCatchJolt::zeroed(),
            fling: crate::knobs::HoldKCatchFling::zeroed(),
            sink: 0.0,
            hips: 0.0,
            heels: [0.0; 2],
            seat: [0.0; 3],
            seat_lean: 0.0,
            brace: [0.0; 2],
            brace_lift: 0.0,
            yank_sit: 0.0,
            feet: Vec::new(),
            lift: 0.0,
            sam_feet: Vec::new(),
            sam_lift: 0.0,
            settle: 0.0,
        }
    }
}
impl Default for HoldKCatch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:136 HoldKCatch.jolt, src/game/character.ts:136)
#[derive(Clone, Copy, Debug)]
pub struct HoldKCatchJolt {
    pub t: [f64; 4],
    pub fwd: f64,
    pub head: f64,
    pub clav: f64,
    pub dip: f64,
}
impl HoldKCatchJolt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKCatchJolt {
            t: [0.0; 4],
            fwd: 0.0,
            head: 0.0,
            clav: 0.0,
            dip: 0.0,
        }
    }
}
impl Default for HoldKCatchJolt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:136 HoldKCatch.fling, src/game/character.ts:136)
#[derive(Clone, Debug)]
pub struct HoldKCatchFling {
    pub t: [f64; 2],
    pub land: [f64; 2],
    pub to: Vec<V3>,
    pub drift: V3,
    pub arm_in: [f64; 2],
}
impl HoldKCatchFling {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKCatchFling {
            t: [0.0; 2],
            land: [0.0; 2],
            to: Vec::new(),
            drift: [0.0; 3],
            arm_in: [0.0; 2],
        }
    }
}
impl Default for HoldKCatchFling { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:157 HoldK.hit, src/game/character.ts:157)
#[derive(Clone, Copy, Debug)]
pub struct HoldKHit {
    pub t: [f64; 4],
    pub hips: f64,
    pub dip: f64,
    pub head: f64,
    pub heave: f64,
    pub sam_in: f64,
    pub sam_dip: f64,
}
impl HoldKHit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKHit {
            t: [0.0; 4],
            hips: 0.0,
            dip: 0.0,
            head: 0.0,
            heave: 0.0,
            sam_in: 0.0,
            sam_dip: 0.0,
        }
    }
}
impl Default for HoldKHit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:172 HoldK.first, src/game/character.ts:172)
#[derive(Clone, Copy, Debug)]
pub struct HoldKFirst {
    pub take: f64,
    pub drop: f64,
    pub catch: f64,
}
impl HoldKFirst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKFirst {
            take: 0.0,
            drop: 0.0,
            catch: 0.0,
        }
    }
}
impl Default for HoldKFirst { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:197 HoldK.strip, src/game/character.ts:197)
#[derive(Clone, Copy, Debug)]
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
impl HoldKStrip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKStrip {
            take: 0.0,
            out: 0.0,
            drop: 0.0,
            follow: 0.0,
            knife: 0.0,
            home: 0.0,
            free: 0.0,
            up: 0.0,
            meet: [0.0; 3],
            torn: [0.0; 3],
            over: [0.0; 3],
            wrench: [0.0; 3],
            low: [0.0; 3],
            yaw: 0.0,
            rap: [0.0; 3],
            hold: [0.0; 2],
            pitch: [0.0; 3],
            rest: 0.0,
            push: [0.0; 2],
            fall: 0.0,
            spin: 0.0,
        }
    }
}
impl Default for HoldKStrip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:214 HoldK.secs, src/game/character.ts:214)
#[derive(Clone, Copy, Debug)]
pub struct HoldKSecs {
    pub grab: f64,
    pub choke_hold: f64,
    pub choke: f64,
    pub whip: f64,
    pub swap: f64,
    pub kill: f64,
}
impl HoldKSecs {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKSecs {
            grab: 0.0,
            choke_hold: 0.0,
            choke: 0.0,
            whip: 0.0,
            swap: 0.0,
            kill: 0.0,
        }
    }
}
impl Default for HoldKSecs { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:228 HoldK.knife, src/game/character.ts:228)
#[derive(Clone, Copy, Debug)]
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
impl HoldKKnife {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKKnife {
            hand: [0.0; 3],
            blade: [0.0; 3],
            back: [0.0; 3],
            via: [0.0; 3],
            off_hand: [0.0; 3],
            off_blade: [0.0; 3],
            off_back: [0.0; 3],
            off_secs: 0.0,
            side: 0.0,
            head_yaw: 0.0,
            clav: 0.0,
        }
    }
}
impl Default for HoldKKnife { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:253 HoldK.lower, src/game/character.ts:253)
#[derive(Clone, Copy, Debug)]
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
impl HoldKLower {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKLower {
            from: 0.0,
            by: 0.0,
            up: 0.0,
            sink: 0.0,
            fwd: 0.0,
            hip: 0.0,
            fold: 0.0,
            hang: [0.0; 2],
            sam_sink: 0.0,
            sam_fwd: 0.0,
            sam_knees: 0.0,
            sam_lean: 0.0,
            sam_look: 0.0,
            sam_arm: [0.0; 2],
        }
    }
}
impl Default for HoldKLower { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:268 HoldK.kill, src/game/character.ts:268)
#[derive(Clone, Debug)]
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
impl HoldKKill {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKKill {
            wind: 0.0,
            hit: 0.0,
            out: 0.0,
            go: 0.0,
            after: 0.0,
            mouth: [0.0; 3],
            chest: [0.0; 3],
            collar: [0.0; 3],
            set: [0.0; 3],
            across: [0.0; 3],
            wide: [0.0; 3],
            low: [0.0; 3],
            wide_blade: [0.0; 3],
            wide_back: [0.0; 3],
            lean: [0.0; 2],
            yaw: [0.0; 2],
            sam_sink: 0.0,
            look: 0.0,
            reach: Vec::new(),
            clutch: Vec::new(),
            chin: 0.0,
            arch: 0.0,
            jolt: [0.0; 2],
            sink: 0.0,
            fwd: 0.0,
            hip: 0.0,
            fold: 0.0,
            hang: [0.0; 2],
        }
    }
}
impl Default for HoldKKill { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:282 HoldK.beats, src/game/character.ts:282)
#[derive(Clone, Copy, Debug)]
pub struct HoldKBeats {
    pub clamp: [f64; 2],
    pub off: [f64; 2],
    pub tip_by: [f64; 2],
    pub tip: f64,
    pub back: f64,
    pub after: f64,
    pub lower: crate::knobs::HoldKBeatsLower,
}
impl HoldKBeats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKBeats {
            clamp: [0.0; 2],
            off: [0.0; 2],
            tip_by: [0.0; 2],
            tip: 0.0,
            back: 0.0,
            after: 0.0,
            lower: crate::knobs::HoldKBeatsLower::zeroed(),
        }
    }
}
impl Default for HoldKBeats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:283 HoldKBeats.lower, src/game/character.ts:283)
#[derive(Clone, Copy, Debug)]
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
impl HoldKBeatsLower {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKBeatsLower {
            from: 0.0,
            by: 0.0,
            up: 0.0,
            sink: 0.0,
            fwd: 0.0,
            hip: 0.0,
            fold: 0.0,
            hang: [0.0; 2],
            sam_sink: 0.0,
            sam_fwd: 0.0,
            sam_knees: 0.0,
            sam_lean: 0.0,
            sam_look: 0.0,
            sam_arm: [0.0; 2],
        }
    }
}
impl Default for HoldKBeatsLower { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:295 HoldK.haul, src/game/character.ts:295)
#[derive(Clone, Copy, Debug)]
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
impl HoldKHaul {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKHaul {
            in_: [0.0; 2],
            out: [0.0; 2],
            down: 0.0,
            sam_lean: 0.0,
            sam_arm: [0.0; 2],
            lift: 0.0,
            back: 0.0,
            recline: 0.0,
            arch: 0.0,
            chin: 0.0,
            feet: [0.0; 2],
            toes: 0.0,
            kick: [0.0; 2],
        }
    }
}
impl Default for HoldKHaul { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:298 HoldK.breath, src/game/character.ts:298)
#[derive(Clone, Copy, Debug)]
pub struct HoldKBreath {
    pub hz: f64,
    pub pitch: f64,
    pub clav: f64,
}
impl HoldKBreath {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKBreath {
            hz: 0.0,
            pitch: 0.0,
            clav: 0.0,
        }
    }
}
impl Default for HoldKBreath { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:301 HoldK.struggle, src/game/character.ts:301)
#[derive(Clone, Copy, Debug)]
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
impl HoldKStruggle {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKStruggle {
            rise: 0.0,
            fade_from: 0.0,
            fade_to: 0.0,
            yaw: 0.0,
            roll: 0.0,
            head: 0.0,
            claw: 0.0,
            dip: 0.0,
            hz: [0.0; 3],
        }
    }
}
impl Default for HoldKStruggle { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:307 HoldK.gun, src/game/character.ts:307)
#[derive(Clone, Debug)]
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
impl HoldKGun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKGun {
            hand: [0.0; 3],
            bore: [0.0; 3],
            out_hand: [0.0; 3],
            kick_pitch: 0.0,
            kick_back: 0.0,
            via_a: [0.0; 3],
            via_b: [0.0; 3],
            via_bore: [0.0; 3],
            wind_hand: [0.0; 3],
            wind_bore: [0.0; 3],
            hit_hand: [0.0; 3],
            hit_bore: [0.0; 3],
            thru_hand: [0.0; 3],
            lean: 0.0,
            side: 0.0,
            head_yaw: 0.0,
            head_pitch: 0.0,
            wind_yaw: 0.0,
            hit_yaw: 0.0,
            clav: 0.0,
            hands_up: Vec::new(),
            aside: 0.0,
            away_yaw: 0.0,
            chin_up: 0.0,
            duck: crate::knobs::HoldKGunDuck::zeroed(),
            knock: crate::knobs::HoldKGunKnock::zeroed(),
        }
    }
}
impl Default for HoldKGun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:322 HoldKGun.duck, src/game/character.ts:322)
#[derive(Clone, Copy, Debug)]
pub struct HoldKGunDuck {
    pub yaw: f64,
    pub roll: f64,
    pub pitch: f64,
    pub hunch: f64,
    pub secs: f64,
}
impl HoldKGunDuck {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKGunDuck {
            yaw: 0.0,
            roll: 0.0,
            pitch: 0.0,
            hunch: 0.0,
            secs: 0.0,
        }
    }
}
impl Default for HoldKGunDuck { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:323 HoldKGun.knock, src/game/character.ts:323)
#[derive(Clone, Copy, Debug)]
pub struct HoldKGunKnock {
    pub yaw: f64,
    pub roll: f64,
    pub pitch: f64,
}
impl HoldKGunKnock {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldKGunKnock {
            yaw: 0.0,
            roll: 0.0,
            pitch: 0.0,
        }
    }
}
impl Default for HoldKGunKnock { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/suppression.ts:26 SUPPRESSION, src/game/suppression.ts:26)
#[derive(Clone, Debug)]
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
impl SuppressionK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SuppressionK {
            near_r: 0.0,
            w_near: 0.0,
            hit_r: 0.0,
            w_hit: 0.0,
            min_travel: 0.0,
            half_life: 0.0,
            cap: 0.0,
            kick_decay: 0.0,
            down_accrual: 0.0,
            friendly_k: 0.0,
            keen: JsMap::new(),
            flinch: 0.0,
            pinned: 0.0,
            release: 0.0,
            head_down: [0.0; 2],
            flinch_delay: 0.0,
            head_down_delay: 0.0,
            guard_spread: 0.0,
            gap_k: 0.0,
            standoff_pace_k: 0.0,
            displace_after: 0.0,
            displace_min: 0.0,
            displace_max: 0.0,
            cover_hold: 0.0,
            displace_cooldown: 0.0,
            displace_cap_secs: 0.0,
            displace_pace: 0.0,
            displace_crouch: 0.0,
            src_memory_secs: 0.0,
            bark_secs: 0.0,
            floor_bark_secs: 0.0,
            pair_secs: 0.0,
            floor_line_secs: 0.0,
            sam_spread: 0.0,
        }
    }
}
impl Default for SuppressionK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:226 RIFLE_SPREAD, src/game/player.ts:226)
#[derive(Clone, Copy, Debug)]
pub struct RifleSpreadK {
    pub still: f64,
    pub moving: f64,
    pub move_at: f64,
    pub crouch_k: f64,
    pub brace_k: f64,
}
impl RifleSpreadK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RifleSpreadK {
            still: 0.0,
            moving: 0.0,
            move_at: 0.0,
            crouch_k: 0.0,
            brace_k: 0.0,
        }
    }
}
impl Default for RifleSpreadK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:242 FOOTFALL, src/game/player.ts:242)
#[derive(Clone, Copy, Debug)]
pub struct FootfallK {
    pub once: bool,
    pub travel: f64,
    pub owed: bool,
    pub owe: f64,
}
impl FootfallK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FootfallK {
            once: false,
            travel: 0.0,
            owed: false,
            owe: 0.0,
        }
    }
}
impl Default for FootfallK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:206 TWOGEARS, src/game/player.ts:206)
#[derive(Clone, Copy, Debug)]
pub struct TwogearsK {
    pub on: bool,
    pub crouch: f64,
}
impl TwogearsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TwogearsK {
            on: false,
            crouch: 0.0,
        }
    }
}
impl Default for TwogearsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/player.ts:214 CROUCHRUN, src/game/player.ts:214)
#[derive(Clone, Copy, Debug)]
pub struct CrouchrunK {
    pub on: bool,
    pub pace: f64,
    pub loud: f64,
}
impl CrouchrunK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CrouchrunK {
            on: false,
            pace: 0.0,
            loud: 0.0,
        }
    }
}
impl Default for CrouchrunK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:693 RUNREADY, src/game/character.ts:693)
#[derive(Clone, Copy, Debug)]
pub struct RunreadyK {
    pub on: bool,
    pub fwd: f64,
    pub up: f64,
    pub pitch: f64,
    pub tuck: f64,
}
impl RunreadyK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunreadyK {
            on: false,
            fwd: 0.0,
            up: 0.0,
            pitch: 0.0,
            tuck: 0.0,
        }
    }
}
impl Default for RunreadyK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:704 RUNARMS, src/game/character.ts:704)
#[derive(Clone, Copy, Debug)]
pub struct RunarmsK {
    pub on: bool,
    pub at: f64,
    pub keep: f64,
    pub mid: f64,
    pub swing: f64,
    pub pump: f64,
    pub lift: f64,
    pub pole: V3,
    pub pole_w: f64,
}
impl RunarmsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunarmsK {
            on: false,
            at: 0.0,
            keep: 0.0,
            mid: 0.0,
            swing: 0.0,
            pump: 0.0,
            lift: 0.0,
            pole: [0.0; 3],
            pole_w: 0.0,
        }
    }
}
impl Default for RunarmsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/samlook.ts:28 SAMLOOK, src/game/samlook.ts:28)
#[derive(Clone, Copy, Debug)]
pub struct SamlookK {
    pub on: bool,
    pub near_m: f64,
    pub full_m: f64,
    pub back_deg: f64,
    pub keep_deg: f64,
    pub notice_m: f64,
    pub swap_m: f64,
    pub slip: bool,
    pub slip_from: f64,
    pub slip_by: f64,
    pub back_cos: [f64; 2],
}
impl SamlookK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamlookK {
            on: false,
            near_m: 0.0,
            full_m: 0.0,
            back_deg: 0.0,
            keep_deg: 0.0,
            notice_m: 0.0,
            swap_m: 0.0,
            slip: false,
            slip_from: 0.0,
            slip_by: 0.0,
            back_cos: [0.0; 2],
        }
    }
}
impl Default for SamlookK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1007 DASH, src/game/character.ts:1007)
#[derive(Clone, Copy, Debug)]
pub struct DashK {
    pub stride: f64,
    pub drop: f64,
    pub lean: f64,
    pub rate: f64,
}
impl DashK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DashK {
            stride: 0.0,
            drop: 0.0,
            lean: 0.0,
            rate: 0.0,
        }
    }
}
impl Default for DashK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:999 WATCH, src/game/character.ts:999)
#[derive(Clone, Copy, Debug)]
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
impl WatchK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WatchK {
            yaw: 0.0,
            down: 0.0,
            up: 0.0,
            share: [0.0; 3],
            rate: 0.0,
            w_rate: 0.0,
            lean: 0.0,
            drop: 0.0,
            away: 0.0,
            arm_out: 0.0,
            arm_down: 0.0,
            arm_w: 0.0,
            arm_side: [0.0; 2],
            slip_rate: 0.0,
            squeeze: 0.0,
        }
    }
}
impl Default for WatchK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stalk.ts:33 STALK, src/game/stalk.ts:33)
#[derive(Clone, Copy, Debug)]
pub struct StalkK {
    pub on: bool,
    pub only: bool,
    pub cursor_m: f64,
    pub begin_cos: f64,
    pub out_cos: f64,
    pub station_m: f64,
    pub grab_m: f64,
    pub pace: f64,
    pub close_min: f64,
    pub close_k: f64,
    pub ease_up: f64,
    pub ease_down: f64,
    pub tell: bool,
    pub reach_from: f64,
    pub reach: f64,
    pub rise_from: f64,
    pub rise_by: f64,
    pub rise: f64,
    pub catch_secs: f64,
    pub catch_m: f64,
    pub catch_long: f64,
    pub turn_in: bool,
    pub catch_stride: f64,
    pub yank: f64,
    pub yank_room: f64,
    pub yank_with: bool,
    pub corner: bool,
    pub corner_m: f64,
    pub corner_deg: f64,
    pub jolt: f64,
    pub jolt_face: f64,
    pub jolt_secs: f64,
    pub jolt_at_once: bool,
    pub corner_show: bool,
    pub corner_see: f64,
    pub corner_reach: [f64; 2],
    pub corner_coil: f64,
    pub corner_rise: f64,
    pub key: bool,
    pub key_m: f64,
    pub low: bool,
    pub low_from: f64,
    pub low_by: f64,
    pub low_k: f64,
    pub low_drop: f64,
    pub low_back: f64,
    pub low_lean: f64,
    pub low_hands: f64,
    pub low_spring: f64,
    pub low_wind: f64,
    pub wide: bool,
    pub wide_out: f64,
    pub wide_down: f64,
    pub wide_fwd: f64,
    pub wide_lead: f64,
    pub wide_by: f64,
    pub wide_pole: V3,
    pub heft: bool,
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
    pub drag: bool,
    pub drag_dead: f64,
    pub drag_full: f64,
    pub orbit: bool,
    pub orbit_k: f64,
    pub orbit_edge: f64,
    pub orbit_snap: f64,
    pub low_begin_m: f64,
    pub low_out_m: f64,
}
impl StalkK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StalkK {
            on: false,
            only: false,
            cursor_m: 0.0,
            begin_cos: 0.0,
            out_cos: 0.0,
            station_m: 0.0,
            grab_m: 0.0,
            pace: 0.0,
            close_min: 0.0,
            close_k: 0.0,
            ease_up: 0.0,
            ease_down: 0.0,
            tell: false,
            reach_from: 0.0,
            reach: 0.0,
            rise_from: 0.0,
            rise_by: 0.0,
            rise: 0.0,
            catch_secs: 0.0,
            catch_m: 0.0,
            catch_long: 0.0,
            turn_in: false,
            catch_stride: 0.0,
            yank: 0.0,
            yank_room: 0.0,
            yank_with: false,
            corner: false,
            corner_m: 0.0,
            corner_deg: 0.0,
            jolt: 0.0,
            jolt_face: 0.0,
            jolt_secs: 0.0,
            jolt_at_once: false,
            corner_show: false,
            corner_see: 0.0,
            corner_reach: [0.0; 2],
            corner_coil: 0.0,
            corner_rise: 0.0,
            key: false,
            key_m: 0.0,
            low: false,
            low_from: 0.0,
            low_by: 0.0,
            low_k: 0.0,
            low_drop: 0.0,
            low_back: 0.0,
            low_lean: 0.0,
            low_hands: 0.0,
            low_spring: 0.0,
            low_wind: 0.0,
            wide: false,
            wide_out: 0.0,
            wide_down: 0.0,
            wide_fwd: 0.0,
            wide_lead: 0.0,
            wide_by: 0.0,
            wide_pole: [0.0; 3],
            heft: false,
            heft_twist: 0.0,
            heft_swing: 0.0,
            heft_side: 0.0,
            heft_lean: 0.0,
            heft_dip: 0.0,
            heft_surge: 0.0,
            heft_out: 0.0,
            heft_down: 0.0,
            heft_fwd: 0.0,
            heft_lead: 0.0,
            heft_pole: [0.0; 3],
            drag: false,
            drag_dead: 0.0,
            drag_full: 0.0,
            orbit: false,
            orbit_k: 0.0,
            orbit_edge: 0.0,
            orbit_snap: 0.0,
            low_begin_m: 0.0,
            low_out_m: 0.0,
        }
    }
}
impl Default for StalkK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tapstep.ts:28 TAPSTEP, src/game/tapstep.ts:28)
#[derive(Clone, Copy, Debug)]
pub struct TapstepK {
    pub on: bool,
    pub tap_m: f64,
    pub beat_m: f64,
    pub reach_m: f64,
    pub secs: f64,
    pub rushed: f64,
    pub settle: f64,
    pub beat: f64,
    pub quiet: f64,
    pub scuff: f64,
    pub mask_secs: f64,
    pub mask_m: f64,
    pub mask: f64,
    pub land_m: f64,
    pub land_cos: f64,
    pub follow: f64,
    pub turn: f64,
    pub legs: f64,
}
impl TapstepK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TapstepK {
            on: false,
            tap_m: 0.0,
            beat_m: 0.0,
            reach_m: 0.0,
            secs: 0.0,
            rushed: 0.0,
            settle: 0.0,
            beat: 0.0,
            quiet: 0.0,
            scuff: 0.0,
            mask_secs: 0.0,
            mask_m: 0.0,
            mask: 0.0,
            land_m: 0.0,
            land_cos: 0.0,
            follow: 0.0,
            turn: 0.0,
            legs: 0.0,
        }
    }
}
impl Default for TapstepK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/devices.ts:55 OVERHEAR, src/game/devices.ts:55)
#[derive(Clone, Copy, Debug)]
pub struct OverhearK {
    pub path_m: f64,
    pub carry: f64,
}
impl OverhearK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OverhearK {
            path_m: 0.0,
            carry: 0.0,
        }
    }
}
impl Default for OverhearK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/chronicle.ts:351 SCORE, src/game/chronicle.ts:351)
#[derive(Clone, Copy, Debug)]
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
impl ScoreK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ScoreK {
            seen: 0.0,
            seen_up: 0.0,
            alarm: 0.0,
            heightened: 0.0,
            lockdown: 0.0,
            body: 0.0,
            kill: 0.0,
            ko: 0.0,
            shot_near: 0.0,
            shot_near_cap: 0.0,
            light: 0.0,
            light_cap: 0.0,
            hit: 0.0,
            down: 0.0,
            load: 0.0,
            civ_reported: 0.0,
            civ_ko: 0.0,
            civ_killed: 0.0,
            device: 0.0,
            device_cap: 0.0,
            par_secs: 0.0,
            per_min: 0.0,
            time_cap: 0.0,
        }
    }
}
impl Default for ScoreK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/chronicle.ts:402 BADGE, src/game/chronicle.ts:402)
#[derive(Clone, Copy, Debug)]
pub struct BadgeK {
    pub lights_out_n: f64,
    pub speedrun_secs: f64,
}
impl BadgeK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BadgeK {
            lights_out_n: 0.0,
            speedrun_secs: 0.0,
        }
    }
}
impl Default for BadgeK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:319 CKPT, src/game/savestate.ts:319)
#[derive(Clone, Copy, Debug)]
pub struct CkptK {
    pub quiet_secs: f64,
    pub aware_max: f64,
    pub debounce_secs: f64,
    pub min_gap_secs: f64,
    pub pending_secs: f64,
    pub body_hide_m: f64,
    pub auto_slots: f64,
    pub path_keep: f64,
    pub flash_secs: f64,
    pub regret_secs: f64,
}
impl CkptK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CkptK {
            quiet_secs: 0.0,
            aware_max: 0.0,
            debounce_secs: 0.0,
            min_gap_secs: 0.0,
            pending_secs: 0.0,
            body_hide_m: 0.0,
            auto_slots: 0.0,
            path_keep: 0.0,
            flash_secs: 0.0,
            regret_secs: 0.0,
        }
    }
}
impl Default for CkptK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fogofwar.ts:138 FOG, src/game/fogofwar.ts:138)
#[derive(Clone, Copy, Debug)]
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
    pub calls: bool,
    pub call_secs: f64,
    pub call_gap: f64,
    pub call_known: f64,
    pub smoke: f64,
    pub linger: f64,
    pub top_pad: f64,
    pub doors: bool,
    pub squad_doors: bool,
    pub squad_doors_m: f64,
    pub squad_fire: bool,
    pub squad_fire_secs: f64,
    pub lasers: bool,
    pub things: bool,
    pub flat: f64,
    pub lit: f64,
    pub full: bool,
    pub air: bool,
    pub thin: bool,
    pub eyes: f64,
    pub kept: bool,
    pub filed: bool,
    pub tracks: bool,
    pub mates: bool,
    pub ours: bool,
    pub full_fill: f64,
    pub edge: f64,
    pub lit_fill: f64,
    pub lit_ao: f64,
    pub lit_ao_m: f64,
    pub lit_cut: f64,
    pub last_seen: f64,
}
impl FogK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogK {
            dim: 0.0,
            desat: 0.0,
            fov_deg: 0.0,
            near_m: 0.0,
            cone: false,
            cone_deg: 0.0,
            cone_near_m: 0.0,
            cone_soft_deg: 0.0,
            cone_near_soft_m: 0.0,
            own: false,
            calls: false,
            call_secs: 0.0,
            call_gap: 0.0,
            call_known: 0.0,
            smoke: 0.0,
            linger: 0.0,
            top_pad: 0.0,
            doors: false,
            squad_doors: false,
            squad_doors_m: 0.0,
            squad_fire: false,
            squad_fire_secs: 0.0,
            lasers: false,
            things: false,
            flat: 0.0,
            lit: 0.0,
            full: false,
            air: false,
            thin: false,
            eyes: 0.0,
            kept: false,
            filed: false,
            tracks: false,
            mates: false,
            ours: false,
            full_fill: 0.0,
            edge: 0.0,
            lit_fill: 0.0,
            lit_ao: 0.0,
            lit_ao_m: 0.0,
            lit_cut: 0.0,
            last_seen: 0.0,
        }
    }
}
impl Default for FogK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:734 MOVES, src/game/character.ts:734)
#[derive(Clone, Copy, Debug)]
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
impl MovesK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MovesK {
            on: 0.0,
            recoil: 0.0,
            toss: 0.0,
            dazzle: 0.0,
            flinch: 0.0,
            gait: 0.0,
            fall: 0.0,
            squeeze: 0.0,
            reload: 0.0,
            bound: 0.0,
            clutch: 0.0,
            entry: 0.0,
            entry_eyes: 0.0,
            fold: 0.0,
            snap: 0.0,
            steady: 0.0,
            steady_run: 0.0,
        }
    }
}
impl Default for MovesK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:915 MV, src/game/character.ts:915)
#[derive(Clone, Debug)]
pub struct MvK {
    pub recoil: crate::knobs::MvKRecoil,
    pub flinch: crate::knobs::MvKFlinch,
    pub hit: crate::knobs::MvKHit,
    pub dazzle: crate::knobs::MvKDazzle,
    pub gait: crate::knobs::MvKGait,
    pub snap: crate::knobs::MvKSnap,
    pub steady: crate::knobs::MvKSteady,
}
impl MvK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MvK {
            recoil: crate::knobs::MvKRecoil::zeroed(),
            flinch: crate::knobs::MvKFlinch::zeroed(),
            hit: crate::knobs::MvKHit::zeroed(),
            dazzle: crate::knobs::MvKDazzle::zeroed(),
            gait: crate::knobs::MvKGait::zeroed(),
            snap: crate::knobs::MvKSnap::zeroed(),
            steady: crate::knobs::MvKSteady::zeroed(),
        }
    }
}
impl Default for MvK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:916 MvK.recoil, src/game/character.ts:916)
#[derive(Clone, Copy, Debug)]
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
impl MvKRecoil {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MvKRecoil {
            back: [0.0; 2],
            back_tau: [0.0; 2],
            climb: [0.0; 2],
            climb_tau: [0.0; 2],
            yaw: [0.0; 2],
            rock: [0.0; 2],
            shoulder: [0.0; 2],
            rock_tau: [0.0; 2],
            head: [0.0; 2],
        }
    }
}
impl Default for MvKRecoil { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:927 MvK.flinch, src/game/character.ts:927)
#[derive(Clone, Copy, Debug)]
pub struct MvKFlinch {
    pub amp: [f64; 2],
    pub dur: [f64; 2],
    pub pitch: f64,
    pub head: f64,
    pub away: f64,
    pub drop: f64,
    pub side: f64,
}
impl MvKFlinch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MvKFlinch {
            amp: [0.0; 2],
            dur: [0.0; 2],
            pitch: 0.0,
            head: 0.0,
            away: 0.0,
            drop: 0.0,
            side: 0.0,
        }
    }
}
impl Default for MvKFlinch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:928 MvK.hit, src/game/character.ts:928)
#[derive(Clone, Copy, Debug)]
pub struct MvKHit {
    pub dur: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
    pub shove: f64,
    pub drop: f64,
}
impl MvKHit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MvKHit {
            dur: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
            shove: 0.0,
            drop: 0.0,
        }
    }
}
impl Default for MvKHit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:929 MvK.dazzle, src/game/character.ts:929)
#[derive(Clone, Copy, Debug)]
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
impl MvKDazzle {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MvKDazzle {
            amp: [0.0; 2],
            hunch: 0.0,
            away: 0.0,
            roll: 0.0,
            head: 0.0,
            head_away: 0.0,
            drop: 0.0,
            back: 0.0,
            sway_x: 0.0,
            sway_z: 0.0,
            rate_in: 0.0,
            rate_out: 0.0,
            hand_drop: 0.0,
            hand_out: 0.0,
            hand_tilt: 0.0,
            hand_roll: 0.0,
            hand_pole: [0.0; 3],
        }
    }
}
impl Default for MvKDazzle { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:931 MvK.gait, src/game/character.ts:931)
#[derive(Clone, Copy, Debug)]
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
impl MvKGait {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MvKGait {
            drop: 0.0,
            aim_drop: 0.0,
            bob: 0.0,
            lean: 0.0,
            aim_lean: 0.0,
            elbow_in: 0.0,
            slouch_sway: 0.0,
            slouch_roll: 0.0,
            back: 0.0,
            chin: 0.0,
            wing: 0.0,
        }
    }
}
impl Default for MvKGait { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:943 MvK.snap, src/game/character.ts:943)
#[derive(Clone, Copy, Debug)]
pub struct MvKSnap {
    pub in_: f64,
    pub ease: f64,
    pub cover: [f64; 2],
    pub under: f64,
}
impl MvKSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MvKSnap {
            in_: 0.0,
            ease: 0.0,
            cover: [0.0; 2],
            under: 0.0,
        }
    }
}
impl Default for MvKSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:955 MvK.steady, src/game/character.ts:955)
#[derive(Clone, Debug)]
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
impl MvKSteady {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MvKSteady {
            roll: 0.0,
            sway: 0.0,
            feet: 0.0,
            knee: 0.0,
            knee_low: 0.0,
            deep: Vec::new(),
            knee_out: 0.0,
            rise: 0.0,
            reach: 0.0,
            rise_rate: 0.0,
        }
    }
}
impl Default for MvKSteady { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:886 TURN, src/game/character.ts:886)
#[derive(Clone, Copy, Debug)]
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
impl TurnK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TurnK {
            body_share: 0.0,
            green: 0.0,
            lead: 0.0,
            lead_rate: 0.0,
            jump: 0.0,
            aim: 0.0,
            aim_acc: 0.0,
            body: 0.0,
            body_acc: 0.0,
            on: 0.0,
        }
    }
}
impl Default for TurnK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wounds.ts:32 WOUND, src/game/wounds.ts:32)
#[derive(Clone, Copy, Debug)]
pub struct WoundK {
    pub on: bool,
    pub centre: f64,
    pub head: f64,
    pub lead: f64,
    pub depth: f64,
    pub back_room: f64,
}
impl WoundK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WoundK {
            on: false,
            centre: 0.0,
            head: 0.0,
            lead: 0.0,
            depth: 0.0,
            back_room: 0.0,
        }
    }
}
impl Default for WoundK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorpath.ts:37 DOORPATH, src/game/doorpath.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct DoorpathK {
    pub on: bool,
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
    pub gun: bool,
    pub gun_m: f64,
    pub gun_ajar: f64,
    pub gun_lead: f64,
    pub gun_side: f64,
    pub gun_over: bool,
    pub gun_still: bool,
    pub gun_down: bool,
    pub gun_lane: bool,
    pub body_m: f64,
    pub body_off: f64,
    pub body_move: f64,
}
impl DoorpathK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorpathK {
            on: false,
            at: 0.0,
            out_m: 0.0,
            in_m: 0.0,
            look: 0.0,
            leaf_clear: 0.0,
            tip_m: 0.0,
            gap_m: 0.0,
            short_m: 0.0,
            cap: 0.0,
            move_m: 0.0,
            apart_m: 0.0,
            gun: false,
            gun_m: 0.0,
            gun_ajar: 0.0,
            gun_lead: 0.0,
            gun_side: 0.0,
            gun_over: false,
            gun_still: false,
            gun_down: false,
            gun_lane: false,
            body_m: 0.0,
            body_off: 0.0,
            body_move: 0.0,
        }
    }
}
impl Default for DoorpathK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:60 WALLS, src/game/wallbreak.ts:60)
#[derive(Clone, Debug)]
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
    pub chunks_per_blast: f64,
    pub fall_max_vol: f64,
    pub falls_per_change: f64,
    pub hung: crate::knobs::WallsKHung,
}
impl WallsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallsK {
            wall_max_thick: 0.0,
            wall_min_top: 0.0,
            wall_min_len: 0.0,
            col: 0.0,
            sill: 0.0,
            tough: JsMap::new(),
            sill_k: 0.0,
            pistol_k: 0.0,
            cover_min_top: 0.0,
            cover_min_len: 0.0,
            cover_max_cells: 0.0,
            blast: crate::knobs::WallsKBlast::zeroed(),
            max_pieces: 0.0,
            max_changes: 0.0,
            chunks_per_cell: 0.0,
            chunks_per_blast: 0.0,
            fall_max_vol: 0.0,
            falls_per_change: 0.0,
            hung: crate::knobs::WallsKHung::zeroed(),
        }
    }
}
impl Default for WallsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:74 WallsK.blast, src/game/wallbreak.ts:74)
#[derive(Clone, Copy, Debug)]
pub struct WallsKBlast {
    pub radius: f64,
    pub hole_k: f64,
    pub port_k: f64,
    pub cover_k: f64,
    pub door_k: f64,
    pub min_hole: f64,
    pub contact_m: f64,
}
impl WallsKBlast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallsKBlast {
            radius: 0.0,
            hole_k: 0.0,
            port_k: 0.0,
            cover_k: 0.0,
            door_k: 0.0,
            min_hole: 0.0,
            contact_m: 0.0,
        }
    }
}
impl Default for WallsKBlast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:78 WallsK.hung, src/game/wallbreak.ts:78)
#[derive(Clone, Copy, Debug)]
pub struct WallsKHung {
    pub deep: f64,
    pub long: f64,
    pub tall: f64,
}
impl WallsKHung {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallsKHung {
            deep: 0.0,
            long: 0.0,
            tall: 0.0,
        }
    }
}
impl Default for WallsKHung { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:78 WALLCHARGE, src/game/wallcharge.ts:78)
#[derive(Clone, Debug)]
pub struct WallchargeK {
    pub on: bool,
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
    pub brace: bool,
    pub mate_m: f64,
    pub after_blow: f64,
    pub enter_cap: f64,
    pub sweep_secs: f64,
    pub mouth_in: f64,
    pub lane: f64,
    pub lane_watch: f64,
    pub pods: Vec<[f64; 2]>,
    pub hole: bool,
    pub doors: bool,
    pub hold_late: bool,
    pub cost: f64,
    pub carried: Option<f64>,
    pub hold_each: bool,
    pub hold_secs: f64,
    pub mate_line: f64,
    pub off_frame: f64,
    pub off_frame_pod: f64,
    pub off_holes: bool,
    pub outer: bool,
    pub outer_thick: f64,
    pub outer_min_len: f64,
    pub outer_yard: bool,
    pub far_furn: bool,
    pub far_slack: f64,
    pub far_reach: f64,
    pub far_mouth: bool,
    pub safe: bool,
    pub hung: bool,
    pub hung_m: f64,
    pub safe_standoff: [f64; 2],
    pub safe_reach: [f64; 2],
    pub safe_secs: [f64; 2],
    pub safe_hide_m: f64,
    pub safe_walk_m: f64,
    pub safe_back: bool,
    pub safe_back_m: f64,
    pub out_of_fire: bool,
    pub fire_known_s: f64,
    pub fire_m: f64,
    pub fire_step: f64,
    pub fire_grain: f64,
    pub men_clear: bool,
    pub men_clear_in: f64,
    pub men_clear_s: f64,
    pub sam_secs: [f64; 2],
    pub sam_sup: f64,
}
impl WallchargeK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallchargeK {
            on: false,
            menu: crate::knobs::WallchargeKMenu::Page,
            pick_m: 0.0,
            height: 0.0,
            end_m: 0.0,
            slide: Vec::new(),
            set_off: 0.0,
            set_near: 0.0,
            sam_reach: 0.0,
            sam_shift: 0.0,
            sam_marker_m: 0.0,
            standoff: Vec::new(),
            post_off: 0.0,
            file_gap: 0.0,
            back_m: Vec::new(),
            clear_m: 0.0,
            brace: false,
            mate_m: 0.0,
            after_blow: 0.0,
            enter_cap: 0.0,
            sweep_secs: 0.0,
            mouth_in: 0.0,
            lane: 0.0,
            lane_watch: 0.0,
            pods: Vec::new(),
            hole: false,
            doors: false,
            hold_late: false,
            cost: 0.0,
            carried: None,
            hold_each: false,
            hold_secs: 0.0,
            mate_line: 0.0,
            off_frame: 0.0,
            off_frame_pod: 0.0,
            off_holes: false,
            outer: false,
            outer_thick: 0.0,
            outer_min_len: 0.0,
            outer_yard: false,
            far_furn: false,
            far_slack: 0.0,
            far_reach: 0.0,
            far_mouth: false,
            safe: false,
            hung: false,
            hung_m: 0.0,
            safe_standoff: [0.0; 2],
            safe_reach: [0.0; 2],
            safe_secs: [0.0; 2],
            safe_hide_m: 0.0,
            safe_walk_m: 0.0,
            safe_back: false,
            safe_back_m: 0.0,
            out_of_fire: false,
            fire_known_s: 0.0,
            fire_m: 0.0,
            fire_step: 0.0,
            fire_grain: 0.0,
            men_clear: false,
            men_clear_in: 0.0,
            men_clear_s: 0.0,
            sam_secs: [0.0; 2],
            sam_sup: 0.0,
        }
    }
}
impl Default for WallchargeK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallchargeKMenu {
    Page,
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

/// (TypeScript: inline at src/game/wallcharge.ts:224 WALLSHOT, src/game/wallcharge.ts:224)
#[derive(Clone, Copy, Debug)]
pub struct WallshotK {
    pub on: bool,
    pub pall: f64,
    pub stagger_secs: f64,
    pub stun_secs: [f64; 2],
    pub room_m: f64,
    pub ours_secs: f64,
}
impl WallshotK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallshotK {
            on: false,
            pall: 0.0,
            stagger_secs: 0.0,
            stun_secs: [0.0; 2],
            room_m: 0.0,
            ours_secs: 0.0,
        }
    }
}
impl Default for WallshotK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:239 WALLKILL, src/game/wallcharge.ts:239)
#[derive(Clone, Copy, Debug)]
pub struct WallkillK {
    pub on: bool,
    pub near_m: f64,
}
impl WallkillK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallkillK {
            on: false,
            near_m: 0.0,
        }
    }
}
impl Default for WallkillK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:403 HARDBREATH, src/game/character.ts:403)
#[derive(Clone, Copy, Debug)]
pub struct HardbreathK {
    pub on: bool,
    pub chest_amp: f64,
    pub chest_rate: f64,
    pub chest_up: f64,
    pub chest_down: f64,
    pub every: f64,
    pub puffs: f64,
    pub count: f64,
    pub jet: f64,
    pub size: f64,
    pub carry: f64,
    pub up: f64,
    pub down: f64,
}
impl HardbreathK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HardbreathK {
            on: false,
            chest_amp: 0.0,
            chest_rate: 0.0,
            chest_up: 0.0,
            chest_down: 0.0,
            every: 0.0,
            puffs: 0.0,
            count: 0.0,
            jet: 0.0,
            size: 0.0,
            carry: 0.0,
            up: 0.0,
            down: 0.0,
        }
    }
}
impl Default for HardbreathK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/props.ts:40 CASTOR, src/game/props.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct CastorK {
    pub on: bool,
    pub decel: f64,
    pub visc: f64,
    pub spin: f64,
    pub start_v: f64,
    pub stop_v: f64,
}
impl CastorK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CastorK {
            on: false,
            decel: 0.0,
            visc: 0.0,
            spin: 0.0,
            start_v: 0.0,
            stop_v: 0.0,
        }
    }
}
impl Default for CastorK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallreact.ts:53 WALLREACT, src/game/wallreact.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct WallreactK {
    pub on: bool,
    pub see_m: f64,
    pub fov_deg: f64,
    pub hear_blast_m: f64,
    pub hear_fire_m: f64,
    pub hear_fall_m: f64,
    pub tell_m: f64,
    pub tell_secs: f64,
    pub near_m: f64,
    pub retry_secs: f64,
    pub every_gone: bool,
    pub gone_max: f64,
    pub blast_flood: bool,
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
    pub entry_risk: bool,
    pub say: bool,
}
impl WallreactK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallreactK {
            on: false,
            see_m: 0.0,
            fov_deg: 0.0,
            hear_blast_m: 0.0,
            hear_fire_m: 0.0,
            hear_fall_m: 0.0,
            tell_m: 0.0,
            tell_secs: 0.0,
            near_m: 0.0,
            retry_secs: 0.0,
            every_gone: false,
            gone_max: 0.0,
            blast_flood: false,
            step_m: 0.0,
            run_m: 0.0,
            beyond_m: 0.0,
            ev_open: 0.0,
            ev_seen: 0.0,
            ev_fire: 0.0,
            forget: 0.0,
            hole_base: 0.0,
            port_base: 0.0,
            crowd: 0.0,
            sticky: 0.0,
            fresh_secs: 0.0,
            fresh: 0.0,
            watch_m: 0.0,
            entry_risk: false,
            say: false,
        }
    }
}
impl Default for WallreactK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/coverfire.ts:54 COVERFIRE, src/game/coverfire.ts:54)
#[derive(Clone, Debug)]
pub struct CoverfireK {
    pub on: bool,
    pub stale_secs: f64,
    pub call_secs: f64,
    pub call_m: f64,
    pub call_walls: bool,
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
impl CoverfireK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverfireK {
            on: false,
            stale_secs: 0.0,
            call_secs: 0.0,
            call_m: 0.0,
            call_walls: false,
            behind_m: JsMap::new(),
            max_layers: 0.0,
            max_rounds: 0.0,
            max_m: 0.0,
            min_m: 0.0,
            near_m: 0.0,
            slack_k: 0.0,
            slack: 0.0,
            give_secs: 0.0,
            lane: 0.0,
            cone_k: 0.0,
            past_m: 0.0,
            same_m: 0.0,
            linger_secs: 0.0,
            lead_secs: 0.0,
            lead_m: 0.0,
            low_y: 0.0,
            round_secs: 0.0,
            walk_pace: 0.0,
            walk_ask_secs: 0.0,
            line: String::new(),
            say_secs: 0.0,
        }
    }
}
impl Default for CoverfireK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destructionSwitch.ts:8 FURNBLAST, src/game/destructionSwitch.ts:8)
#[derive(Clone, Copy, Debug)]
pub struct FurnblastK {
    pub on: bool,
}
impl FurnblastK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnblastK {
            on: false,
        }
    }
}
impl Default for FurnblastK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doors.ts:31 DOORWAY, src/game/doors.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct DoorwayK {
    pub one_way: bool,
    pub exits_out: bool,
    pub frame_holds: bool,
    pub side_kick: bool,
    pub kick_glass: bool,
    pub solid: bool,
    pub leaf_way: bool,
}
impl DoorwayK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorwayK {
            one_way: false,
            exits_out: false,
            frame_holds: false,
            side_kick: false,
            kick_glass: false,
            solid: false,
            leaf_way: false,
        }
    }
}
impl Default for DoorwayK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorblast.ts:43 DOORBLAST, src/game/doorblast.ts:43)
#[derive(Clone, Copy, Debug)]
pub struct DoorblastK {
    pub on: bool,
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
impl DoorblastK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorblastK {
            on: false,
            tear_m: crate::knobs::DoorblastKTearM::zeroed(),
            charge_k: 0.0,
            charge_r: 0.0,
            speed: [0.0; 2],
            up: 0.0,
            tip: 0.0,
            spin: 0.0,
            scrape: 0.0,
            slam: 0.0,
            slide: 0.0,
            bounce: 0.0,
            room: 0.0,
            land_level: 0.0,
        }
    }
}
impl Default for DoorblastK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorblast.ts:50 DoorblastK.tearM, src/game/doorblast.ts:50)
#[derive(Clone, Copy, Debug)]
pub struct DoorblastKTearM {
    pub stun: f64,
    pub frag: f64,
}
impl DoorblastKTearM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorblastKTearM {
            stun: 0.0,
            frag: 0.0,
        }
    }
}
impl Default for DoorblastKTearM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorblast.ts:90 DOORSHOT, src/game/doorblast.ts:90)
#[derive(Clone, Debug)]
pub struct DoorshotK {
    pub on: bool,
    pub toll: f64,
    pub flat: bool,
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
impl DoorshotK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorshotK {
            on: false,
            toll: 0.0,
            flat: false,
            strips: 0.0,
            lengths: 0.0,
            uneven: 0.0,
            chunks: 0.0,
            fly: Vec::new(),
            side: 0.0,
            up: Vec::new(),
            spin: 0.0,
            tilt: Vec::new(),
            topple: Vec::new(),
            chips: Vec::new(),
            chip_v: Vec::new(),
            shed: 0.0,
            slivers: Vec::new(),
            level: 0.0,
        }
    }
}
impl Default for DoorshotK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doors.ts:175 DOORTHROW, src/game/doors.ts:175)
#[derive(Clone, Copy, Debug)]
pub struct DoorthrowK {
    pub on: bool,
    pub push_secs: f64,
    pub kick_secs: f64,
    pub startle: f64,
    pub wary: f64,
    pub soft: bool,
    pub jolt: bool,
    pub min_speed: f64,
    pub hand_rate: f64,
    pub floor_rad: f64,
    pub rate: f64,
    pub slide: f64,
    pub again: f64,
    pub aware: f64,
}
impl DoorthrowK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorthrowK {
            on: false,
            push_secs: 0.0,
            kick_secs: 0.0,
            startle: 0.0,
            wary: 0.0,
            soft: false,
            jolt: false,
            min_speed: 0.0,
            hand_rate: 0.0,
            floor_rad: 0.0,
            rate: 0.0,
            slide: 0.0,
            again: 0.0,
            aware: 0.0,
        }
    }
}
impl Default for DoorthrowK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/winbars.ts:40 WINBARS, src/game/winbars.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct WinbarsK {
    pub on: bool,
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
impl WinbarsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WinbarsK {
            on: false,
            per_round: 0.0,
            pieces: 0.0,
            uneven: 0.0,
            fly: [0.0; 2],
            rest_k: 0.0,
            up: [0.0; 2],
            side: 0.0,
            spin: 0.0,
            chips: 0.0,
            chip_v: 0.0,
            blast_k: 0.0,
        }
    }
}
impl Default for WinbarsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/opening.ts:21 OPENING, src/game/opening.ts:21)
#[derive(Clone, Copy, Debug)]
pub struct OpeningK {
    pub on: bool,
    pub clear_m: f64,
    pub step_m: f64,
}
impl OpeningK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpeningK {
            on: false,
            clear_m: 0.0,
            step_m: 0.0,
        }
    }
}
impl Default for OpeningK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doors.ts:147 DOORSTAY, src/game/doors.ts:147)
#[derive(Clone, Copy, Debug)]
pub struct DoorstayK {
    pub on: bool,
}
impl DoorstayK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorstayK {
            on: false,
        }
    }
}
impl Default for DoorstayK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handleside.ts:32 HANDLE, src/game/handleside.ts:32)
#[derive(Clone, Copy, Debug)]
pub struct HandleK {
    pub on: bool,
    pub push: bool,
    pub pull: bool,
    pub tools: bool,
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
    pub tool_near: bool,
    pub near_body: f64,
    pub near_post: f64,
    pub go_cap: f64,
    pub rest: f64,
}
impl HandleK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HandleK {
            on: false,
            push: false,
            pull: false,
            tools: false,
            look: 0.0,
            ajar: 0.0,
            reach: 0.0,
            handle_in: 0.0,
            far_reach: 0.0,
            push_in: 0.0,
            push_off: 0.0,
            push_thru: 0.0,
            stand_in: 0.0,
            stand_off: 0.0,
            thru_at: 0.0,
            thru_off: 0.0,
            body: 0.0,
            slack: 0.0,
            near: 0.0,
            turn: 0.0,
            clear: 0.0,
            wait: 0.0,
            press: 0.0,
            tool_post: 0.0,
            tool_near: false,
            near_body: 0.0,
            near_post: 0.0,
            go_cap: 0.0,
            rest: 0.0,
        }
    }
}
impl Default for HandleK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorway.ts:36 DOORMANNERS, src/game/doorway.ts:36)
#[derive(Clone, Copy, Debug)]
pub struct DoormannersK {
    pub on: bool,
    pub pass: bool,
    pub stand: bool,
    pub fight: bool,
    pub swing: bool,
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
    pub own_place: bool,
    pub own_m: f64,
    pub own_end_m: f64,
    pub leaf_round: bool,
    pub leaf_stall: bool,
    pub leaf_stall_m: f64,
    pub leaf_stall_secs: f64,
    pub leaf_stall_reach: f64,
    pub leaf_stall_ahead: f64,
    pub leaf_stall_push: f64,
    pub sweep: f64,
    pub lane: bool,
    pub lane_ajar: f64,
    pub lane_m: f64,
    pub lane_out: f64,
    pub lane_flare: f64,
    pub lane_clear: f64,
    pub lane_apart: f64,
    pub lane_move: bool,
    pub lane_leaf: bool,
    pub watch: bool,
    pub stack_cover: bool,
    pub cover_m: f64,
    pub cover_deep: f64,
    pub cover_look: f64,
    pub cover_quiet: f64,
    pub cover_cap: f64,
    pub lane_walk: bool,
    pub lane_dear: f64,
    pub lane_strip: f64,
    pub lane_long: f64,
    pub funnel: f64,
    pub funnel_rest: f64,
    pub through: f64,
    pub past: f64,
    pub step_cover: bool,
    pub plane_deg: f64,
    pub rest_clear: bool,
    pub rest_secs: f64,
    pub mark_clear: bool,
    pub pod_clear: bool,
    pub clear_m: f64,
    pub short_clear: bool,
    pub held_cap: f64,
    pub own_leaf: bool,
    pub own_mark: bool,
    pub off_furniture: bool,
    pub back_on: bool,
}
impl DoormannersK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoormannersK {
            on: false,
            pass: false,
            stand: false,
            fight: false,
            swing: false,
            reach: 0.0,
            half: 0.0,
            throat: 0.0,
            gap: 0.0,
            gap_min: 0.0,
            brake: 0.0,
            patience: 0.0,
            rest: 0.0,
            beyond: 0.0,
            aside: 0.0,
            jamb_out: 0.0,
            jamb_past: 0.0,
            file_gap: 0.0,
            apart: 0.0,
            step: 0.0,
            round: 0.0,
            round_out: 0.0,
            own_place: false,
            own_m: 0.0,
            own_end_m: 0.0,
            leaf_round: false,
            leaf_stall: false,
            leaf_stall_m: 0.0,
            leaf_stall_secs: 0.0,
            leaf_stall_reach: 0.0,
            leaf_stall_ahead: 0.0,
            leaf_stall_push: 0.0,
            sweep: 0.0,
            lane: false,
            lane_ajar: 0.0,
            lane_m: 0.0,
            lane_out: 0.0,
            lane_flare: 0.0,
            lane_clear: 0.0,
            lane_apart: 0.0,
            lane_move: false,
            lane_leaf: false,
            watch: false,
            stack_cover: false,
            cover_m: 0.0,
            cover_deep: 0.0,
            cover_look: 0.0,
            cover_quiet: 0.0,
            cover_cap: 0.0,
            lane_walk: false,
            lane_dear: 0.0,
            lane_strip: 0.0,
            lane_long: 0.0,
            funnel: 0.0,
            funnel_rest: 0.0,
            through: 0.0,
            past: 0.0,
            step_cover: false,
            plane_deg: 0.0,
            rest_clear: false,
            rest_secs: 0.0,
            mark_clear: false,
            pod_clear: false,
            clear_m: 0.0,
            short_clear: false,
            held_cap: 0.0,
            own_leaf: false,
            own_mark: false,
            off_furniture: false,
            back_on: false,
        }
    }
}
impl Default for DoormannersK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:93 BREAKS, src/game/breakTable.ts:93)
#[derive(Clone, Copy, Debug)]
pub struct BreaksK {
    pub share: crate::knobs::BreaksKShare,
    pub small_pool_chip_k: f64,
    pub refuse_chips: [f64; 2],
    pub crumble_chips: [f64; 2],
    pub sized_chips: [f64; 2],
    pub sized_span: [f64; 2],
    pub china_land: f64,
    pub china_max: f64,
    pub core_tint: f64,
    pub chip_tint: f64,
    pub heap: crate::knobs::BreaksKHeap,
    pub max_standing_planes: f64,
    pub across_deg: [f64; 2],
    pub look_edges: bool,
    pub look_heap: bool,
    pub loose_min: f64,
    pub kink: crate::knobs::BreaksKKink,
    pub head: crate::knobs::BreaksKHead,
    pub steel: crate::knobs::BreaksKSteel,
    pub stud: crate::knobs::BreaksKStud,
}
impl BreaksK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksK {
            share: crate::knobs::BreaksKShare::zeroed(),
            small_pool_chip_k: 0.0,
            refuse_chips: [0.0; 2],
            crumble_chips: [0.0; 2],
            sized_chips: [0.0; 2],
            sized_span: [0.0; 2],
            china_land: 0.0,
            china_max: 0.0,
            core_tint: 0.0,
            chip_tint: 0.0,
            heap: crate::knobs::BreaksKHeap::zeroed(),
            max_standing_planes: 0.0,
            across_deg: [0.0; 2],
            look_edges: false,
            look_heap: false,
            loose_min: 0.0,
            kink: crate::knobs::BreaksKKink::zeroed(),
            head: crate::knobs::BreaksKHead::zeroed(),
            steel: crate::knobs::BreaksKSteel::zeroed(),
            stud: crate::knobs::BreaksKStud::zeroed(),
        }
    }
}
impl Default for BreaksK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:94 BreaksK.share, src/game/breakTable.ts:94)
#[derive(Clone, Copy, Debug)]
pub struct BreaksKShare {
    pub big: crate::knobs::BreaksKShareBig,
    pub small: crate::knobs::BreaksKShareSmall,
}
impl BreaksKShare {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksKShare {
            big: crate::knobs::BreaksKShareBig::zeroed(),
            small: crate::knobs::BreaksKShareSmall::zeroed(),
        }
    }
}
impl Default for BreaksKShare { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:94 BreaksKShare.big, src/game/breakTable.ts:94)
#[derive(Clone, Copy, Debug)]
pub struct BreaksKShareBig {
    pub thrown: f64,
    pub heap: f64,
    pub furniture: f64,
}
impl BreaksKShareBig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksKShareBig {
            thrown: 0.0,
            heap: 0.0,
            furniture: 0.0,
        }
    }
}
impl Default for BreaksKShareBig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:94 BreaksKShare.small, src/game/breakTable.ts:94)
#[derive(Clone, Copy, Debug)]
pub struct BreaksKShareSmall {
    pub thrown: f64,
    pub heap: f64,
    pub furniture: f64,
}
impl BreaksKShareSmall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksKShareSmall {
            thrown: 0.0,
            heap: 0.0,
            furniture: 0.0,
        }
    }
}
impl Default for BreaksKShareSmall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:107 BreaksK.heap, src/game/breakTable.ts:107)
#[derive(Clone, Copy, Debug)]
pub struct BreaksKHeap {
    pub span: [f64; 2],
    pub out: [f64; 2],
    pub tip: [f64; 2],
}
impl BreaksKHeap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksKHeap {
            span: [0.0; 2],
            out: [0.0; 2],
            tip: [0.0; 2],
        }
    }
}
impl Default for BreaksKHeap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:119 BreaksK.kink, src/game/breakTable.ts:119)
#[derive(Clone, Copy, Debug)]
pub struct BreaksKKink {
    pub deg: [f64; 2],
    pub skew: f64,
    pub off: f64,
    pub min_long: f64,
    pub gap_secs: f64,
}
impl BreaksKKink {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksKKink {
            deg: [0.0; 2],
            skew: 0.0,
            off: 0.0,
            min_long: 0.0,
            gap_secs: 0.0,
        }
    }
}
impl Default for BreaksKKink { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:124 BreaksK.head, src/game/breakTable.ts:124)
#[derive(Clone, Copy, Debug)]
pub struct BreaksKHead {
    pub on: bool,
    pub from: f64,
    pub deep: [f64; 2],
    pub planes: f64,
    pub across: [f64; 2],
    pub min_room: f64,
    pub most_of_width: f64,
}
impl BreaksKHead {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksKHead {
            on: false,
            from: 0.0,
            deep: [0.0; 2],
            planes: 0.0,
            across: [0.0; 2],
            min_room: 0.0,
            most_of_width: 0.0,
        }
    }
}
impl Default for BreaksKHead { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:129 BreaksK.steel, src/game/breakTable.ts:129)
#[derive(Clone, Copy, Debug)]
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
impl BreaksKSteel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksKSteel {
            sheet: 0.0,
            kink: [0.0; 2],
            dents: [0.0; 2],
            dent: [0.0; 2],
            dent_most: 0.0,
            rim: 0.0,
            lip: 0.0,
            inside: 0.0,
        }
    }
}
impl Default for BreaksKSteel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:132 BreaksK.stud, src/game/breakTable.ts:132)
#[derive(Clone, Copy, Debug)]
pub struct BreaksKStud {
    pub half: f64,
    pub depth_k: f64,
    pub foot: f64,
    pub albedo: V3,
    pub rough: f64,
    pub metal: bool,
    pub does: crate::scene_materials::DoesWord,
}
impl BreaksKStud {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksKStud {
            half: 0.0,
            depth_k: 0.0,
            foot: 0.0,
            albedo: [0.0; 3],
            rough: 0.0,
            metal: false,
            does: crate::scene_materials::DoesWord::Snow,
        }
    }
}
impl Default for BreaksKStud { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:56 FURN, src/game/furnblast.ts:56)
#[derive(Clone, Debug)]
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
impl FurnK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnK {
            k: JsMap::new(),
            fling: JsMap::new(),
            shatter_m: 0.0,
            apart_m: 0.0,
            throw_m: 0.0,
            throw_v: 0.0,
            prop_break_m: 0.0,
            chunks_per_blast: 0.0,
            mouth_m: 0.0,
            merge_m: 0.0,
            mouth: crate::knobs::FurnKMouth::zeroed(),
            hung: crate::knobs::FurnKHung::zeroed(),
            root: crate::knobs::FurnKRoot::zeroed(),
            rider: crate::knobs::FurnKRider::zeroed(),
            touch: 0.0,
            row_m: 0.0,
        }
    }
}
impl Default for FurnK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:83 FurnK.mouth, src/game/furnblast.ts:83)
#[derive(Clone, Copy, Debug)]
pub struct FurnKMouth {
    pub on: bool,
    pub depth: f64,
    pub top: f64,
    pub vol: f64,
    pub chunks: f64,
    pub snow: bool,
    pub base: bool,
    pub base_thick: f64,
}
impl FurnKMouth {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnKMouth {
            on: false,
            depth: 0.0,
            top: 0.0,
            vol: 0.0,
            chunks: 0.0,
            snow: false,
            base: false,
            base_thick: 0.0,
        }
    }
}
impl Default for FurnKMouth { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:88 FurnK.hung, src/game/furnblast.ts:88)
#[derive(Clone, Debug)]
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
impl FurnKHung {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnKHung {
            len: 0.0,
            max: 0.0,
            strips: 0.0,
            under_m: 0.0,
            through: Vec::new(),
            back: Vec::new(),
            side: 0.0,
            up: Vec::new(),
            spin: 0.0,
            chunks: 0.0,
            chips: Vec::new(),
            chip_v: Vec::new(),
            hole_m: 0.0,
        }
    }
}
impl Default for FurnKHung { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:90 FurnK.root, src/game/furnblast.ts:90)
#[derive(Clone, Copy, Debug)]
pub struct FurnKRoot {
    pub min_top: f64,
    pub max_top: f64,
    pub max_long: f64,
    pub max_vol: f64,
    pub apart_max_vol: f64,
}
impl FurnKRoot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnKRoot {
            min_top: 0.0,
            max_top: 0.0,
            max_long: 0.0,
            max_vol: 0.0,
            apart_max_vol: 0.0,
        }
    }
}
impl Default for FurnKRoot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:92 FurnK.rider, src/game/furnblast.ts:92)
#[derive(Clone, Copy, Debug)]
pub struct FurnKRider {
    pub max_top: f64,
    pub max_vol: f64,
}
impl FurnKRider {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnKRider {
            max_top: 0.0,
            max_vol: 0.0,
        }
    }
}
impl Default for FurnKRider { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowBog.ts:18 SNOWBOG, src/game/snowBog.ts:18)
#[derive(Clone, Copy, Debug)]
pub struct SnowbogK {
    pub on: bool,
    pub dead: f64,
    pub grip: f64,
    pub stick: f64,
    pub packed: f64,
    pub old: f64,
    pub top: f64,
    pub bank_m: f64,
    pub run: f64,
}
impl SnowbogK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowbogK {
            on: false,
            dead: 0.0,
            grip: 0.0,
            stick: 0.0,
            packed: 0.0,
            old: 0.0,
            top: 0.0,
            bank_m: 0.0,
            run: 0.0,
        }
    }
}
impl Default for SnowbogK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destructionSwitch.ts:19 RUBBLE, src/game/destructionSwitch.ts:19)
#[derive(Clone, Copy, Debug)]
pub struct RubbleK {
    pub on: bool,
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
impl RubbleK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RubbleK {
            on: false,
            radius: 0.0,
            reach: 0.0,
            lead: 0.0,
            inside: 0.0,
            floor: 0.0,
            heavy: 0.0,
            walk: 0.0,
            base: 0.0,
            gain: 0.0,
            aside: 0.0,
            v_max: 0.0,
            off: 0.0,
            slide: 0.0,
            spin: 0.0,
            per_frame: 0.0,
            cooldown: 0.0,
            give_up: 0.0,
            give_up_secs: 0.0,
            scuff: 0.0,
        }
    }
}
impl Default for RubbleK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destructionSwitch.ts:38 EVICT, src/game/destructionSwitch.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct EvictK {
    pub on: bool,
}
impl EvictK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EvictK {
            on: false,
        }
    }
}
impl Default for EvictK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destructionSwitch.ts:29 DEBRIS, src/game/destructionSwitch.ts:29)
#[derive(Clone, Copy, Debug)]
pub struct DebrisK {
    pub dress: bool,
    pub min_speed: f64,
}
impl DebrisK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DebrisK {
            dress: false,
            min_speed: 0.0,
        }
    }
}
impl Default for DebrisK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lighting.ts:30 LIGHTWALL, src/game/lighting.ts:30)
#[derive(Clone, Copy, Debug)]
pub struct LightwallK {
    pub on: bool,
    pub gap: f64,
}
impl LightwallK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LightwallK {
            on: false,
            gap: 0.0,
        }
    }
}
impl Default for LightwallK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/airDust.ts:34 AIRDUST, src/game/airDust.ts:34)
#[derive(Clone, Copy, Debug)]
pub struct AirdustK {
    pub on: bool,
    pub diffuse: f64,
    pub settle: f64,
    pub step: f64,
    pub recheck_secs: f64,
}
impl AirdustK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AirdustK {
            on: false,
            diffuse: 0.0,
            settle: 0.0,
            step: 0.0,
            recheck_secs: 0.0,
        }
    }
}
impl Default for AirdustK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallsnap.ts:27 WALLSNAP, src/game/wallsnap.ts:27)
#[derive(Clone, Copy, Debug)]
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
    pub scuff: f64,
    pub step_lift: f64,
    pub step_trail: f64,
    pub pose: bool,
    pub cam_room: f64,
    pub cam_side: f64,
    pub along_deg: f64,
    pub away_secs: f64,
    pub away_diag: f64,
    pub off_push: f64,
    pub out_rate: f64,
    pub stack: bool,
    pub stack_draw: bool,
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
    pub lean_over: bool,
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
    pub press_calm: bool,
    pub press_run: bool,
    pub voice: bool,
    pub voice_on: f64,
    pub voice_off: f64,
    pub slow: bool,
    pub slow_take: f64,
    pub slow_in: f64,
    pub slow_out: f64,
}
impl WallsnapK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallsnapK {
            on: false,
            take_base: 0.0,
            take_per_m: 0.0,
            take_per_turn: 0.0,
            take_min: 0.0,
            take_max: 0.0,
            ease_pow: 0.0,
            yaw_pow: 0.0,
            reach_moving: 0.0,
            heading_min: 0.0,
            heading_bonus: 0.0,
            buffer: 0.0,
            dip: 0.0,
            nod: 0.0,
            hit_peak: 0.0,
            scuff: 0.0,
            step_lift: 0.0,
            step_trail: 0.0,
            pose: false,
            cam_room: 0.0,
            cam_side: 0.0,
            along_deg: 0.0,
            away_secs: 0.0,
            away_diag: 0.0,
            off_push: 0.0,
            out_rate: 0.0,
            stack: false,
            stack_draw: false,
            up_secs: 0.0,
            up_rise: 0.0,
            bore: 0.0,
            lean_along: 0.0,
            lean_out: 0.0,
            lean_back: 0.0,
            lean_on_deg: 0.0,
            lean_off_deg: 0.0,
            lean_min_dist: 0.0,
            lean_max_deg: 0.0,
            lean_keep_deg: 0.0,
            lean_keep_dist: 0.0,
            lean_over: false,
            lean_over_y: 0.0,
            lean_clear_m: 0.0,
            lean_gun: 0.0,
            lean_gun_step: 0.0,
            dwell_out: 0.0,
            dwell_back: 0.0,
            min_out: 0.0,
            after_shot: 0.0,
            press: false,
            press_secs: 0.0,
            press_flow: 0.0,
            press_near: 0.0,
            press_deg: 0.0,
            press_touch: 0.0,
            press_thing: 0.0,
            press_thing_at: 0.0,
            press_turn: 0.0,
            press_again: 0.0,
            press_after_shot: 0.0,
            press_calm: false,
            press_run: false,
            voice: false,
            voice_on: 0.0,
            voice_off: 0.0,
            slow: false,
            slow_take: 0.0,
            slow_in: 0.0,
            slow_out: 0.0,
        }
    }
}
impl Default for WallsnapK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunwall.ts:45 GUNWALL, src/game/gunwall.ts:45)
#[derive(Clone, Copy, Debug)]
pub struct GunwallK {
    pub on: bool,
    pub down_m: f64,
    pub up_m: f64,
    pub lead_s: f64,
    pub at_m: f64,
    pub tuck_m: f64,
    pub tuck_lead_s: f64,
    pub fight_s: f64,
    pub mark: bool,
}
impl GunwallK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunwallK {
            on: false,
            down_m: 0.0,
            up_m: 0.0,
            lead_s: 0.0,
            at_m: 0.0,
            tuck_m: 0.0,
            tuck_lead_s: 0.0,
            fight_s: 0.0,
            mark: false,
        }
    }
}
impl Default for GunwallK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/close.ts:41 CLOSE, src/game/close.ts:41)
#[derive(Clone, Copy, Debug)]
pub struct CloseK {
    pub on: bool,
    pub mode: bool,
    pub ready: bool,
    pub dark: bool,
    pub marks: bool,
    pub side: bool,
    pub front: bool,
    pub seat: bool,
    pub alert_strike: bool,
    pub lethal: bool,
    pub held_clicks: bool,
    pub reach_up: bool,
    pub hands_low: bool,
    pub no_knife: bool,
    pub rmb_hold: bool,
    pub rmb_tap_secs: f64,
    pub rmb_loud: f64,
    pub rmb_wait_secs: f64,
    pub hands_low_neck_m: f64,
    pub mode_m: f64,
    pub mode_out_m: f64,
    pub dark_at: f64,
    pub lit_at: f64,
    pub touch_m: f64,
    pub fade_m: f64,
    pub reach: crate::knobs::CloseKReach,
    pub side_from_deg: f64,
    pub inner: f64,
    pub step_deg: f64,
    pub strike_dist: f64,
    pub seat_dist: f64,
    pub noise: crate::knobs::CloseKNoise,
    pub alert_noise: f64,
}
impl CloseK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseK {
            on: false,
            mode: false,
            ready: false,
            dark: false,
            marks: false,
            side: false,
            front: false,
            seat: false,
            alert_strike: false,
            lethal: false,
            held_clicks: false,
            reach_up: false,
            hands_low: false,
            no_knife: false,
            rmb_hold: false,
            rmb_tap_secs: 0.0,
            rmb_loud: 0.0,
            rmb_wait_secs: 0.0,
            hands_low_neck_m: 0.0,
            mode_m: 0.0,
            mode_out_m: 0.0,
            dark_at: 0.0,
            lit_at: 0.0,
            touch_m: 0.0,
            fade_m: 0.0,
            reach: crate::knobs::CloseKReach::zeroed(),
            side_from_deg: 0.0,
            inner: 0.0,
            step_deg: 0.0,
            strike_dist: 0.0,
            seat_dist: 0.0,
            noise: crate::knobs::CloseKNoise::zeroed(),
            alert_noise: 0.0,
        }
    }
}
impl Default for CloseK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/close.ts:103 CloseK.reach, src/game/close.ts:103)
#[derive(Clone, Copy, Debug)]
pub struct CloseKReach {
    pub behind: f64,
    pub side: f64,
    pub front: f64,
    pub seat: f64,
    pub seat_front: f64,
}
impl CloseKReach {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseKReach {
            behind: 0.0,
            side: 0.0,
            front: 0.0,
            seat: 0.0,
            seat_front: 0.0,
        }
    }
}
impl Default for CloseKReach { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/close.ts:113 CloseK.noise, src/game/close.ts:113)
#[derive(Clone, Copy, Debug)]
pub struct CloseKNoise {
    pub behind: f64,
    pub side: f64,
    pub front: f64,
    pub seat: f64,
}
impl CloseKNoise {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloseKNoise {
            behind: 0.0,
            side: 0.0,
            front: 0.0,
            seat: 0.0,
        }
    }
}
impl Default for CloseKNoise { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/floorplan.ts:60 PLAN, src/game/floorplan.ts:60)
#[derive(Clone, Copy, Debug)]
pub struct PlanK {
    pub on: bool,
    pub paths: bool,
    pub marks: bool,
    pub sectors: bool,
    pub who: bool,
    pub tags: bool,
    pub word: bool,
    pub throws: bool,
    pub menu_only: bool,
    pub key_lit: bool,
    pub thr_told: bool,
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
impl PlanK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlanK {
            on: false,
            paths: false,
            marks: false,
            sectors: false,
            who: false,
            tags: false,
            word: false,
            throws: false,
            menu_only: false,
            key_lit: false,
            thr_told: false,
            fresh_secs: 0.0,
            walk_dim: 0.0,
            rest_dim: 0.0,
            ring_m: 0.0,
            sector_m: 0.0,
            sector_deg: 0.0,
            sight_m: 0.0,
            reask_secs: 0.0,
            pair_bar_m: 0.0,
            others: 0.0,
        }
    }
}
impl Default for PlanK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gear.ts:24 GEAR, src/game/gear.ts:24)
#[derive(Clone, Copy, Debug)]
pub struct GearK {
    pub on: bool,
}
impl GearK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GearK {
            on: false,
        }
    }
}
impl Default for GearK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/startfile.ts:20 START, src/game/startfile.ts:20)
#[derive(Clone, Copy, Debug)]
pub struct StartK {
    pub fall_in: bool,
}
impl StartK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StartK {
            fall_in: false,
        }
    }
}
impl Default for StartK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/samestart.ts:21 SAMESTART, src/game/samestart.ts:21)
#[derive(Clone, Copy, Debug)]
pub struct SamestartK {
    pub on: bool,
}
impl SamestartK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamestartK {
            on: false,
        }
    }
}
impl Default for SamestartK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/takecheck.ts:15 TAKE, src/game/takecheck.ts:15)
#[derive(Clone, Copy, Debug)]
pub struct TakeK {
    pub wired: bool,
}
impl TakeK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TakeK {
            wired: false,
        }
    }
}
impl Default for TakeK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/autoreload.ts:23 RELOAD, src/game/autoreload.ts:23)
#[derive(Clone, Copy, Debug)]
pub struct ReloadK {
    pub auto: bool,
    pub delay: f64,
    pub last_mag: bool,
}
impl ReloadK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReloadK {
            auto: false,
            delay: 0.0,
            last_mag: false,
        }
    }
}
impl Default for ReloadK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/quickthrow.ts:10 QUICKTHROW, src/game/quickthrow.ts:10)
#[derive(Clone, Copy, Debug)]
pub struct QuickthrowK {
    pub on: bool,
}
impl QuickthrowK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuickthrowK {
            on: false,
        }
    }
}
impl Default for QuickthrowK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/samstun.ts:23 SAMSTUN, src/game/samstun.ts:23)
#[derive(Clone, Copy, Debug)]
pub struct SamstunK {
    pub on: bool,
    pub blind_roots: bool,
    pub fade: f64,
    pub own_seen: bool,
}
impl SamstunK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamstunK {
            on: false,
            blind_roots: false,
            fade: 0.0,
            own_seen: false,
        }
    }
}
impl Default for SamstunK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/deathrecap.ts:29 RECAP, src/game/deathrecap.ts:29)
#[derive(Clone, Copy, Debug)]
pub struct RecapK {
    pub on: bool,
    pub secs: f64,
    pub after: f64,
    pub hz: f64,
    pub keep: f64,
    pub live: bool,
    pub live_arrows: bool,
    pub live_see_m: f64,
    pub live_saw_secs: f64,
    pub live_hear_m: f64,
    pub live_beat: f64,
    pub live_side_m: f64,
    pub live_mark_secs: f64,
}
impl RecapK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RecapK {
            on: false,
            secs: 0.0,
            after: 0.0,
            hz: 0.0,
            keep: 0.0,
            live: false,
            live_arrows: false,
            live_see_m: 0.0,
            live_saw_secs: 0.0,
            live_hear_m: 0.0,
            live_beat: 0.0,
            live_side_m: 0.0,
            live_mark_secs: 0.0,
        }
    }
}
impl Default for RecapK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aimweight.ts:20 AIMWEIGHT, src/game/aimweight.ts:20)
#[derive(Clone, Copy, Debug)]
pub struct AimweightK {
    pub on: bool,
    pub rate: f64,
    pub smooth: f64,
    pub heft: f64,
    pub bare: f64,
    pub on_m: f64,
    pub reach: f64,
    pub launch_wait: f64,
}
impl AimweightK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AimweightK {
            on: false,
            rate: 0.0,
            smooth: 0.0,
            heft: 0.0,
            bare: 0.0,
            on_m: 0.0,
            reach: 0.0,
            launch_wait: 0.0,
        }
    }
}
impl Default for AimweightK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aimcost.ts:40 AIMCOST, src/game/aimcost.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct AimcostK {
    pub lamp: bool,
    pub move_: bool,
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
    pub ring_px: f64,
}
impl AimcostK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AimcostK {
            lamp: false,
            move_: false,
            bare: 0.0,
            pad: 0.0,
            slack: 0.0,
            move_k: 0.0,
            stop_tau: 0.0,
            stop_bleed: 0.0,
            brace_feet: 0.0,
            bare_device: 0.0,
            plant_at: 0.0,
            flank_at: 0.0,
            ring_px: 0.0,
        }
    }
}
impl Default for AimcostK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/friends.ts:27 FRIENDS, src/game/friends.ts:27)
#[derive(Clone, Copy, Debug)]
pub struct FriendsK {
    pub no_snap: bool,
    pub low_ready: bool,
    pub reach_m: f64,
    pub past_m: f64,
    pub clear_m: f64,
    pub lead_s: f64,
    pub lead_max_m: f64,
    pub up_secs: f64,
    pub quiet_s: f64,
}
impl FriendsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FriendsK {
            no_snap: false,
            low_ready: false,
            reach_m: 0.0,
            past_m: 0.0,
            clear_m: 0.0,
            lead_s: 0.0,
            lead_max_m: 0.0,
            up_secs: 0.0,
            quiet_s: 0.0,
        }
    }
}
impl Default for FriendsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/teamtally.ts:17 TEAMTALLY, src/game/teamtally.ts:17)
#[derive(Clone, Copy, Debug)]
pub struct TeamtallyK {
    pub on: bool,
}
impl TeamtallyK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TeamtallyK {
            on: false,
        }
    }
}
impl Default for TeamtallyK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/friendlyfire.ts:37 FRIENDLYFIRE, src/game/friendlyfire.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct FriendlyfireK {
    pub on: bool,
    pub cone_k: f64,
    pub body_m: f64,
    pub sure_m: f64,
    pub care: bool,
    pub words: bool,
    pub near_m: f64,
}
impl FriendlyfireK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FriendlyfireK {
            on: false,
            cone_k: 0.0,
            body_m: 0.0,
            sure_m: 0.0,
            care: false,
            words: false,
            near_m: 0.0,
        }
    }
}
impl Default for FriendlyfireK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lightswitch.ts:52 LIGHTSWITCH, src/game/lightswitch.ts:52)
#[derive(Clone, Debug)]
pub struct LightswitchK {
    pub on: bool,
    pub reach: f64,
    pub height: f64,
    pub beside: Vec<f64>,
    pub stand_m: f64,
    pub per_room: f64,
    pub apart_m: f64,
    pub scan_m: f64,
    pub strike: bool,
    pub guards: bool,
    pub guard_secs: f64,
    pub hand_secs: f64,
}
impl LightswitchK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LightswitchK {
            on: false,
            reach: 0.0,
            height: 0.0,
            beside: Vec::new(),
            stand_m: 0.0,
            per_room: 0.0,
            apart_m: 0.0,
            scan_m: 0.0,
            strike: false,
            guards: false,
            guard_secs: 0.0,
            hand_secs: 0.0,
        }
    }
}
impl Default for LightswitchK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/carlamps.ts:29 CARLAMPS, src/game/carlamps.ts:29)
#[derive(Clone, Copy, Debug)]
pub struct CarlampsK {
    pub on: bool,
    pub dark: V3,
    pub fit: f64,
    pub tone: f64,
    pub twin: f64,
}
impl CarlampsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CarlampsK {
            on: false,
            dark: [0.0; 3],
            fit: 0.0,
            tone: 0.0,
            twin: 0.0,
        }
    }
}
impl Default for CarlampsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:92 HITSTUFF, src/game/particleFx.ts:92)
#[derive(Clone, Copy, Debug)]
pub struct HitstuffK {
    pub on: bool,
}
impl HitstuffK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HitstuffK {
            on: false,
        }
    }
}
impl Default for HitstuffK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/decals.ts:121 MARKCUT, src/render/decals.ts:121)
#[derive(Clone, Copy, Debug)]
pub struct MarkcutK {
    pub on: bool,
}
impl MarkcutK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MarkcutK {
            on: false,
        }
    }
}
impl Default for MarkcutK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/decals.ts:129 MARKSIT, src/render/decals.ts:129)
#[derive(Clone, Copy, Debug)]
pub struct MarksitK {
}
impl MarksitK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MarksitK {
        }
    }
}
impl Default for MarksitK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/stuff.ts:241 COATFINISH, src/scene/stuff.ts:241)
#[derive(Clone, Copy, Debug)]
pub struct CoatfinishK {
    pub on: bool,
}
impl CoatfinishK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoatfinishK {
            on: false,
        }
    }
}
impl Default for CoatfinishK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/stuff.ts:260 SOLIDGLASS, src/scene/stuff.ts:260)
#[derive(Clone, Copy, Debug)]
pub struct SolidglassK {
    pub on: bool,
    pub lum: f64,
    pub rough: f64,
}
impl SolidglassK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SolidglassK {
            on: false,
            lum: 0.0,
            rough: 0.0,
        }
    }
}
impl Default for SolidglassK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cleanrestart.ts:28 CLEANRESTART, src/game/cleanrestart.ts:28)
#[derive(Clone, Copy, Debug)]
pub struct CleanrestartK {
    pub on: bool,
    pub picture: bool,
}
impl CleanrestartK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CleanrestartK {
            on: false,
            picture: false,
        }
    }
}
impl Default for CleanrestartK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fireline.ts:44 FIRELINE, src/game/fireline.ts:44)
#[derive(Clone, Copy, Debug)]
pub struct FirelineK {
    pub on: bool,
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
    pub cue: bool,
    pub cue_s: f64,
    pub max_s: f64,
    pub again_s: f64,
    pub way_more_m: f64,
    pub berth_m: f64,
    pub edge_m: f64,
    pub reach_m: f64,
}
impl FirelineK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FirelineK {
            on: false,
            fire_s: 0.0,
            reload_s: 0.0,
            pace_s: 0.0,
            pace_k: 0.0,
            lapse_s: 0.0,
            spread: 0.0,
            clear_m: 0.0,
            short_m: 0.0,
            keep_m: 0.0,
            gap_m: 0.0,
            look_m: 0.0,
            cue: false,
            cue_s: 0.0,
            max_s: 0.0,
            again_s: 0.0,
            way_more_m: 0.0,
            berth_m: 0.0,
            edge_m: 0.0,
            reach_m: 0.0,
        }
    }
}
impl Default for FirelineK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:344 SAVETIMELINE, src/game/savestate.ts:344)
#[derive(Clone, Copy, Debug)]
pub struct SavetimelineK {
    pub on: bool,
    pub manual: f64,
    pub auto: f64,
    pub holds: bool,
    pub turn: bool,
    pub since: bool,
    pub base: bool,
    pub pic_px: f64,
    pub pic_q: f64,
}
impl SavetimelineK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SavetimelineK {
            on: false,
            manual: 0.0,
            auto: 0.0,
            holds: false,
            turn: false,
            since: false,
            base: false,
            pic_px: 0.0,
            pic_q: 0.0,
        }
    }
}
impl Default for SavetimelineK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handslot.ts:14 HANDSLOT, src/game/handslot.ts:14)
#[derive(Clone, Copy, Debug)]
pub struct HandslotK {
    pub on: bool,
}
impl HandslotK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HandslotK {
            on: false,
        }
    }
}
impl Default for HandslotK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handring.ts:27 HANDRING, src/game/handring.ts:27)
#[derive(Clone, Copy, Debug)]
pub struct HandringK {
    pub on: bool,
    pub hold_secs: f64,
    pub early: bool,
    pub empty: bool,
    pub near: bool,
    pub lone: bool,
    pub lone_wrap: bool,
    pub on_thing: bool,
    pub bare: bool,
    pub turn: bool,
    pub turn_max: f64,
    pub corner: bool,
}
impl HandringK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HandringK {
            on: false,
            hold_secs: 0.0,
            early: false,
            empty: false,
            near: false,
            lone: false,
            lone_wrap: false,
            on_thing: false,
            bare: false,
            turn: false,
            turn_max: 0.0,
            corner: false,
        }
    }
}
impl Default for HandringK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwback.ts:14 THROWBACK, src/game/throwback.ts:14)
#[derive(Clone, Copy, Debug)]
pub struct ThrowbackK {
    pub on: bool,
    pub rest_mark: bool,
    pub reach: bool,
}
impl ThrowbackK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThrowbackK {
            on: false,
            rest_mark: false,
            reach: false,
        }
    }
}
impl Default for ThrowbackK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/ownblow.ts:28 OWNBLOW, src/game/ownblow.ts:28)
#[derive(Clone, Copy, Debug)]
pub struct OwnblowK {
    pub on: bool,
}
impl OwnblowK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OwnblowK {
            on: false,
        }
    }
}
impl Default for OwnblowK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sprintdoor.ts:32 SPRINTDOOR, src/game/sprintdoor.ts:32)
#[derive(Clone, Copy, Debug)]
pub struct SprintdoorK {
    pub on: bool,
    pub keep: f64,
    pub bash_vel: f64,
    pub lean: f64,
    pub lean_secs: f64,
    pub rip_pace: f64,
    pub rip_secs: f64,
    pub rip_vel: f64,
    pub rip_level: f64,
    pub kick: bool,
    pub kick_k: f64,
    pub kick_into: f64,
    pub kick_free: f64,
}
impl SprintdoorK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SprintdoorK {
            on: false,
            keep: 0.0,
            bash_vel: 0.0,
            lean: 0.0,
            lean_secs: 0.0,
            rip_pace: 0.0,
            rip_secs: 0.0,
            rip_vel: 0.0,
            rip_level: 0.0,
            kick: false,
            kick_k: 0.0,
            kick_into: 0.0,
            kick_free: 0.0,
        }
    }
}
impl Default for SprintdoorK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/teampace.ts:43 TEAMPACE, src/game/teampace.ts:43)
#[derive(Clone, Copy, Debug)]
pub struct TeampaceK {
    pub on: bool,
    pub k: f64,
    pub follow_k: f64,
    pub follow: bool,
    pub behind_m: f64,
    pub fight_secs: f64,
    pub back_secs: f64,
    pub own: bool,
    pub dash_m: f64,
    pub dash_seen: f64,
    pub dash_every: f64,
    pub dash_hot: f64,
    pub door_brisk: bool,
    pub pacing: bool,
    pub pace_near: f64,
    pub pace_gap: f64,
    pub pace_cos: f64,
    pub walk_aim: bool,
    pub walk_aim_off: f64,
    pub walk_aim_on: f64,
    pub run_at: f64,
    pub accel: f64,
    pub step: f64,
    pub gap: f64,
}
impl TeampaceK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TeampaceK {
            on: false,
            k: 0.0,
            follow_k: 0.0,
            follow: false,
            behind_m: 0.0,
            fight_secs: 0.0,
            back_secs: 0.0,
            own: false,
            dash_m: 0.0,
            dash_seen: 0.0,
            dash_every: 0.0,
            dash_hot: 0.0,
            door_brisk: false,
            pacing: false,
            pace_near: 0.0,
            pace_gap: 0.0,
            pace_cos: 0.0,
            walk_aim: false,
            walk_aim_off: 0.0,
            walk_aim_on: 0.0,
            run_at: 0.0,
            accel: 0.0,
            step: 0.0,
            gap: 0.0,
        }
    }
}
impl Default for TeampaceK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/noshove.ts:28 NOSHOVE, src/game/noshove.ts:28)
#[derive(Clone, Copy, Debug)]
pub struct NoshoveK {
    pub on: bool,
    pub round: bool,
    pub own: bool,
    pub through: bool,
    pub ring: f64,
    pub back: f64,
}
impl NoshoveK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NoshoveK {
            on: false,
            round: false,
            own: false,
            through: false,
            ring: 0.0,
            back: 0.0,
        }
    }
}
impl Default for NoshoveK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:47 RAILS, src/game/rails.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct RailsK {
    pub on: bool,
    pub pan_speed: f64,
    pub pan_reach: f64,
    pub snap_px: f64,
    pub keep: f64,
    pub keep_speed: f64,
    pub keep_deep: f64,
    pub keep_up: f64,
    pub way_m: f64,
    pub way_run: bool,
    pub after_secs: f64,
    pub after_max: f64,
    pub shift_tap: bool,
    pub line_clear: bool,
    pub mark_clear: bool,
    pub eye_first: bool,
    pub eye_view: bool,
    pub card_beat: bool,
    pub card_beat_secs: f64,
    pub card_quiet: f64,
    pub card_max: f64,
    pub spare_down: bool,
    pub flat_ring: bool,
    pub spare_secs: f64,
}
impl RailsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsK {
            on: false,
            pan_speed: 0.0,
            pan_reach: 0.0,
            snap_px: 0.0,
            keep: 0.0,
            keep_speed: 0.0,
            keep_deep: 0.0,
            keep_up: 0.0,
            way_m: 0.0,
            way_run: false,
            after_secs: 0.0,
            after_max: 0.0,
            shift_tap: false,
            line_clear: false,
            mark_clear: false,
            eye_first: false,
            eye_view: false,
            card_beat: false,
            card_beat_secs: 0.0,
            card_quiet: 0.0,
            card_max: 0.0,
            spare_down: false,
            flat_ring: false,
            spare_secs: 0.0,
        }
    }
}
impl Default for RailsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorforce.ts:62 DOORFORCE, src/game/doorforce.ts:62)
#[derive(Clone, Copy, Debug)]
pub struct DoorforceK {
    pub on: bool,
    pub shoot: bool,
    pub kick: bool,
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
impl DoorforceK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorforceK {
            on: false,
            shoot: false,
            kick: false,
            sure: 0.0,
            keep: 0.0,
            fade_secs: 0.0,
            stale_secs: 0.0,
            gain: 0.0,
            hear_min: 0.0,
            shot: 0.0,
            handle: 0.0,
            seen: 0.0,
            shut_carry: 0.0,
            behind_m: 0.0,
            reach_m: 0.0,
            seen_secs: 0.0,
            heard_err_m: 0.0,
            chest_y: 0.0,
            near_leaf_m: 0.0,
            past_leaf_m: 0.0,
            rounds: 0.0,
            gap_secs: 0.0,
            kick_secs: 0.0,
            kick_secs_no_shot: 0.0,
            walk_secs: 0.0,
            rest_secs: 0.0,
        }
    }
}
impl Default for DoorforceK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/slotsetup.ts:47 SLOTSETUP, src/game/slotsetup.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct SlotsetupK {
    pub on: bool,
    pub pop: bool,
    pub secs: f64,
    pub stay: bool,
    pub draw: bool,
    pub settle: f64,
    pub mags: bool,
}
impl SlotsetupK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotsetupK {
            on: false,
            pop: false,
            secs: 0.0,
            stay: false,
            draw: false,
            settle: 0.0,
            mags: false,
        }
    }
}
impl Default for SlotsetupK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassbreak.ts:113 GLASSREAL, src/game/glassbreak.ts:113)
#[derive(Clone, Copy, Debug)]
pub struct GlassrealK {
    pub drop: bool,
    pub place: bool,
    pub hang: f64,
    pub blocks: f64,
    pub low_m: f64,
    pub door_m: f64,
    pub door_foot_m: f64,
}
impl GlassrealK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassrealK {
            drop: false,
            place: false,
            hang: 0.0,
            blocks: 0.0,
            low_m: 0.0,
            door_m: 0.0,
            door_foot_m: 0.0,
        }
    }
}
impl Default for GlassrealK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/quietfog.ts:44 QUIETFOG, src/game/quietfog.ts:44)
#[derive(Clone, Copy, Debug)]
pub struct QuietfogK {
    pub on: bool,
    pub def: crate::quietfog::QuietRung,
    pub wand: bool,
    pub solo: bool,
}
impl QuietfogK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietfogK {
            on: false,
            def: crate::quietfog::QuietRung::Team,
            wand: false,
            solo: false,
        }
    }
}
impl Default for QuietfogK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/told.ts:33 TOLD, src/game/told.ts:33)
#[derive(Clone, Copy, Debug)]
pub struct ToldK {
    pub on: bool,
    pub rung: crate::told::ToldRung,
    pub guided: f64,
    pub standard: f64,
    pub fog_off: bool,
}
impl ToldK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ToldK {
            on: false,
            rung: crate::told::ToldRung::Bare,
            guided: 0.0,
            standard: 0.0,
            fog_off: false,
        }
    }
}
impl Default for ToldK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/words.ts:27 WORDS, src/game/words.ts:27)
#[derive(Clone, Copy, Debug)]
pub struct WordsK {
    pub on: bool,
    pub doors: bool,
    pub radio_home: bool,
    pub guide: bool,
}
impl WordsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WordsK {
            on: false,
            doors: false,
            radio_home: false,
            guide: false,
        }
    }
}
impl Default for WordsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kitanim.ts:45 KITANIM, src/game/kitanim.ts:45)
#[derive(Clone, Copy, Debug)]
pub struct KitanimK {
    pub on: bool,
    pub equip: bool,
    pub reload: bool,
    pub close: bool,
    pub grips: bool,
    pub sling: f64,
    pub sling_up: f64,
    pub sling_down: f64,
    pub nv: bool,
    pub early: f64,
    pub click_keep: f64,
    pub throw_keep: f64,
    pub plain_lock: f64,
    pub beats: crate::knobs::KitanimKBeats,
    pub close_near: f64,
}
impl KitanimK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitanimK {
            on: false,
            equip: false,
            reload: false,
            close: false,
            grips: false,
            sling: 0.0,
            sling_up: 0.0,
            sling_down: 0.0,
            nv: false,
            early: 0.0,
            click_keep: 0.0,
            throw_keep: 0.0,
            plain_lock: 0.0,
            beats: crate::knobs::KitanimKBeats::zeroed(),
            close_near: 0.0,
        }
    }
}
impl Default for KitanimK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kitanim.ts:76 KitanimK.beats, src/game/kitanim.ts:76)
#[derive(Clone, Copy, Debug)]
pub struct KitanimKBeats {
    pub pistol: crate::knobs::KitanimKBeatsPistol,
    pub rifle: crate::knobs::KitanimKBeatsRifle,
}
impl KitanimKBeats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitanimKBeats {
            pistol: crate::knobs::KitanimKBeatsPistol::zeroed(),
            rifle: crate::knobs::KitanimKBeatsRifle::zeroed(),
        }
    }
}
impl Default for KitanimKBeats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kitanim.ts:77 KitanimKBeats.pistol, src/game/kitanim.ts:77)
#[derive(Clone, Copy, Debug)]
pub struct KitanimKBeatsPistol {
    pub ref_: f64,
    pub empty: crate::kitanim::BeatRow,
    pub dry: crate::kitanim::BeatRow,
    pub tactical: crate::kitanim::BeatRow,
}
impl KitanimKBeatsPistol {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitanimKBeatsPistol {
            ref_: 0.0,
            empty: crate::kitanim::BeatRow::zeroed(),
            dry: crate::kitanim::BeatRow::zeroed(),
            tactical: crate::kitanim::BeatRow::zeroed(),
        }
    }
}
impl Default for KitanimKBeatsPistol { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kitanim.ts:78 KitanimKBeats.rifle, src/game/kitanim.ts:78)
#[derive(Clone, Copy, Debug)]
pub struct KitanimKBeatsRifle {
    pub ref_: f64,
    pub empty: crate::kitanim::BeatRow,
    pub dry: crate::kitanim::BeatRow,
    pub tactical: crate::kitanim::BeatRow,
}
impl KitanimKBeatsRifle {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitanimKBeatsRifle {
            ref_: 0.0,
            empty: crate::kitanim::BeatRow::zeroed(),
            dry: crate::kitanim::BeatRow::zeroed(),
            tactical: crate::kitanim::BeatRow::zeroed(),
        }
    }
}
impl Default for KitanimKBeatsRifle { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1223 KIT, src/game/character.ts:1223)
#[derive(Clone, Debug)]
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
    pub sheath_pole: f64,
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
impl KitK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitK {
            stow: JsMap::new(),
            reach: JsMap::new(),
            draw: JsMap::new(),
            away: 0.0,
            swap_k: 0.0,
            far: 0.0,
            apex_at: 0.0,
            sling_at: 0.0,
            join: 0.0,
            let_go: 0.0,
            apex: crate::knobs::KitKApex::zeroed(),
            elbow_min: 0.0,
            wrist_knee: 0.0,
            wrist_max: 0.0,
            roll_in: 0.0,
            over: [0.0; 3],
            up: [0.0; 3],
            top: crate::knobs::KitKTop::zeroed(),
            trunk: 0.0,
            belt: crate::knobs::KitKBelt::zeroed(),
            holster_pole: [0.0; 3],
            sheath_pole: 0.0,
            rest: [0.0; 3],
            arc: JsMap::new(),
            cross: [0.0; 3],
            front: crate::knobs::KitKFront::zeroed(),
            nv: crate::knobs::KitKNv::zeroed(),
            fist: Vec::new(),
            open: Vec::new(),
            mag: crate::knobs::KitKMag::zeroed(),
            close: crate::knobs::KitKClose::zeroed(),
        }
    }
}
impl Default for KitK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1228 KitK.apex, src/game/character.ts:1228)
#[derive(Clone, Copy, Debug)]
pub struct KitKApex {
    pub at: V3,
    pub fing: V3,
    pub back: V3,
    pub pole: V3,
}
impl KitKApex {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitKApex {
            at: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
            pole: [0.0; 3],
        }
    }
}
impl Default for KitKApex { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1235 KitK.top, src/game/character.ts:1235)
#[derive(Clone, Copy, Debug)]
pub struct KitKTop {
    pub fing: V3,
    pub back: V3,
}
impl KitKTop {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitKTop {
            fing: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for KitKTop { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1236 KitK.belt, src/game/character.ts:1236)
#[derive(Clone, Copy, Debug)]
pub struct KitKBelt {
    pub at: V3,
    pub fing: V3,
    pub back: V3,
    pub pole: V3,
}
impl KitKBelt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitKBelt {
            at: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
            pole: [0.0; 3],
        }
    }
}
impl Default for KitKBelt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1244 KitK.front, src/game/character.ts:1244)
#[derive(Clone, Copy, Debug)]
pub struct KitKFront {
    pub stow: f64,
    pub reach: f64,
    pub draw: f64,
    pub hand_at: f64,
    pub sling_at: f64,
    pub arc: V3,
    pub pole: V3,
}
impl KitKFront {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitKFront {
            stow: 0.0,
            reach: 0.0,
            draw: 0.0,
            hand_at: 0.0,
            sling_at: 0.0,
            arc: [0.0; 3],
            pole: [0.0; 3],
        }
    }
}
impl Default for KitKFront { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1253 KitK.nv, src/game/character.ts:1253)
#[derive(Clone, Copy, Debug)]
pub struct KitKNv {
    pub hand: crate::knobs::KitKNvHand,
    pub nod: crate::knobs::KitKNvNod,
}
impl KitKNv {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitKNv {
            hand: crate::knobs::KitKNvHand::zeroed(),
            nod: crate::knobs::KitKNvNod::zeroed(),
        }
    }
}
impl Default for KitKNv { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1254 KitKNv.hand, src/game/character.ts:1254)
#[derive(Clone, Copy, Debug)]
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
impl KitKNvHand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitKNvHand {
            dur: 0.0,
            reach: 0.0,
            out: 0.0,
            flip: [0.0; 2],
            up: [0.0; 3],
            down: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
            pole: [0.0; 3],
            shrug: 0.0,
            curl: [0.0; 4],
            thumb: 0.0,
            rate: 0.0,
            away: 0.0,
            clear: 0.0,
        }
    }
}
impl Default for KitKNvHand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1256 KitKNv.nod, src/game/character.ts:1256)
#[derive(Clone, Copy, Debug)]
pub struct KitKNvNod {
    pub dur: f64,
    pub pre: f64,
    pub peak: f64,
    pub flip: [f64; 2],
    pub on: [f64; 2],
    pub off: [f64; 2],
    pub carry: f64,
}
impl KitKNvNod {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitKNvNod {
            dur: 0.0,
            pre: 0.0,
            peak: 0.0,
            flip: [0.0; 2],
            on: [0.0; 2],
            off: [0.0; 2],
            carry: 0.0,
        }
    }
}
impl Default for KitKNvNod { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1263 KitK.mag, src/game/character.ts:1263)
#[derive(Clone, Copy, Debug)]
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
impl KitKMag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitKMag {
            well: [0.0; 3],
            strip: [0.0; 3],
            slide: [0.0; 3],
            pull: [0.0; 3],
            pouch: [0.0; 3],
            dump: [0.0; 3],
            via: [0.0; 3],
            pole: [0.0; 3],
            pole_w: 0.0,
            wf: [0.0; 3],
            wb: [0.0; 3],
            sf: [0.0; 3],
            sb: [0.0; 3],
        }
    }
}
impl Default for KitKMag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1274 KitK.close, src/game/character.ts:1274)
#[derive(Clone, Copy, Debug)]
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
impl KitKClose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitKClose {
            far: 0.0,
            hover: 0.0,
            bow: [0.0; 3],
            lead: [0.0; 3],
            left: 0.0,
            left_at: 0.0,
            left_in: 0.0,
            left_out: 0.0,
            rate: 0.0,
            flex: 0.0,
            flex_amp: 0.0,
            drift: 0.0,
            regrip: 0.0,
            curl: 0.0,
        }
    }
}
impl Default for KitKClose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1335 THROW_BY, src/game/character.ts:1335)
#[derive(Clone, Copy, Debug)]
pub struct ThrowByK {
    pub on: bool,
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
impl ThrowByK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThrowByK {
            on: false,
            toss: 0.0,
            toss_fast: 0.0,
            toss_in: 0.0,
            toss_min: 0.0,
            through: 0.0,
            toss_pivot: [0.0; 3],
            full: 0.0,
            turn_min: 0.0,
            arm_min: 0.0,
            off_min: 0.0,
            sit: 0.0,
            pivot: [0.0; 3],
        }
    }
}
impl Default for ThrowByK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/hands.ts:46 HANDS, src/game/hands.ts:46)
#[derive(Clone, Copy, Debug)]
pub struct HandsK {
    pub on: bool,
    pub body: bool,
    pub door: bool,
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
    pub carry: bool,
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
    pub team: bool,
    pub team_pre_m: f64,
    pub team_reach_m: f64,
    pub team_pace: f64,
    pub team_vel: f64,
}
impl HandsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HandsK {
            on: false,
            body: false,
            door: false,
            reach_secs: 0.0,
            lift_secs: 0.0,
            set_secs: 0.0,
            grip_m: 0.0,
            step_secs: 0.0,
            head_bias: 0.0,
            ahead: 0.0,
            ahead_head: 0.0,
            carry_feet: 0.0,
            carry_head: 0.0,
            slack_m: 0.0,
            carry: false,
            hoist_secs: 0.0,
            lower_secs: 0.0,
            carry_step_secs: 0.0,
            carry_near_m: 0.0,
            carry_side: 0.0,
            carry_fwd: 0.0,
            carry_up: 0.0,
            carry_back: 0.0,
            carry_legs: 0.0,
            carry_pace: 0.0,
            carry_turn: 0.0,
            carry_ride: 0.0,
            lay_ahead: 0.0,
            door_reach: 0.0,
            hand_secs: 0.0,
            door_rate: 0.0,
            open_to: 0.0,
            clear_m: 0.0,
            step_m: 0.0,
            round_m: 0.0,
            hand_m: 0.0,
            in_m: 0.0,
            stall_secs: 0.0,
            aim_arc: 0.0,
            team: false,
            team_pre_m: 0.0,
            team_reach_m: 0.0,
            team_pace: 0.0,
            team_vel: 0.0,
        }
    }
}
impl Default for HandsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/workthings.ts:52 WORK, src/game/workthings.ts:52)
#[derive(Clone, Copy, Debug)]
pub struct WorkK {
    pub on: bool,
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
    pub posted: bool,
    pub own_face: bool,
    pub held_pass: bool,
    pub off_walk: bool,
    pub walk_m: f64,
    pub sam_face: bool,
}
impl WorkK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkK {
            on: false,
            pick_m: 0.0,
            pick_wide_m: 0.0,
            behind_m: 0.0,
            behind_panel_m: 0.0,
            stand_m: 0.0,
            buddy_m: 0.0,
            hand_secs: 0.0,
            breaker_secs: 0.0,
            fade_secs: 0.0,
            hand_m: 0.0,
            posted: false,
            own_face: false,
            held_pass: false,
            off_walk: false,
            walk_m: 0.0,
            sam_face: false,
        }
    }
}
impl Default for WorkK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wand.ts:35 WAND, src/game/wand.ts:35)
#[derive(Clone, Debug)]
pub struct WandK {
    pub on: bool,
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
    pub nv: bool,
    pub mate: bool,
    pub order_key: String,
    pub sweep_secs: f64,
    pub quick: bool,
    pub quick_secs: f64,
    pub quick_down: f64,
}
impl WandK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WandK {
            on: false,
            feed_secs: 0.0,
            stow_secs: 0.0,
            fov_deg: 0.0,
            blind_deg: 0.0,
            pan_rate: 0.0,
            eye_y: 0.0,
            past_m: 0.0,
            mark_secs: 0.0,
            shut_rad: 0.0,
            give_up_secs: 0.0,
            nv: false,
            mate: false,
            order_key: String::new(),
            sweep_secs: 0.0,
            quick: false,
            quick_secs: 0.0,
            quick_down: 0.0,
        }
    }
}
impl Default for WandK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stickycam.ts:43 STICKY, src/game/stickycam.ts:43)
#[derive(Clone, Copy, Debug)]
pub struct StickyK {
    pub on: bool,
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
    pub take: bool,
    pub take_reach: f64,
    pub take_noise: f64,
    pub mark: bool,
}
impl StickyK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StickyK {
            on: false,
            carry: 0.0,
            max: 0.0,
            live: 0.0,
            issue: 0.0,
            speed: 0.0,
            gravity: 0.0,
            drag: 0.0,
            radius: 0.0,
            life: 0.0,
            substeps: 0.0,
            solve_iters: 0.0,
            cd: 0.0,
            aim_y: 0.0,
            fov_deg: 0.0,
            off: 0.0,
            flat_y: 0.0,
            launch_noise: 0.0,
            stick_noise: 0.0,
            take: false,
            take_reach: 0.0,
            take_noise: 0.0,
            mark: false,
        }
    }
}
impl Default for StickyK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/peeksight.ts:32 PEEKSIGHT, src/game/peeksight.ts:32)
#[derive(Clone, Copy, Debug)]
pub struct PeeksightK {
    pub on: bool,
    pub out: f64,
    pub off: f64,
    pub turn_deg: f64,
}
impl PeeksightK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PeeksightK {
            on: false,
            out: 0.0,
            off: 0.0,
            turn_deg: 0.0,
        }
    }
}
impl Default for PeeksightK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/owncharge.ts:21 OWNCHARGE, src/game/owncharge.ts:21)
#[derive(Clone, Copy, Debug)]
pub struct OwnchargeK {
    pub on: bool,
}
impl OwnchargeK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OwnchargeK {
            on: false,
        }
    }
}
impl Default for OwnchargeK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunslots.ts:16 GUNSLOTS, src/game/gunslots.ts:16)
#[derive(Clone, Copy, Debug)]
pub struct GunslotsK {
    pub on: bool,
    pub dry: bool,
    pub near: f64,
    pub out: f64,
    pub lull: f64,
    pub back: f64,
    pub heard: bool,
    pub bare: bool,
}
impl GunslotsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunslotsK {
            on: false,
            dry: false,
            near: 0.0,
            out: 0.0,
            lull: 0.0,
            back: 0.0,
            heard: false,
            bare: false,
        }
    }
}
impl Default for GunslotsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunlook.ts:16 GUNLOOK, src/game/gunlook.ts:16)
#[derive(Clone, Copy, Debug)]
pub struct GunlookK {
    pub on: bool,
    pub shells: bool,
}
impl GunlookK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunlookK {
            on: false,
            shells: false,
        }
    }
}
impl Default for GunlookK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunroles.ts:41 GUNROLES, src/game/gunroles.ts:41)
#[derive(Clone, Copy, Debug)]
pub struct GunrolesK {
    pub on: bool,
    pub deal: bool,
    pub akms: f64,
    pub belt_lane: f64,
    pub belt_min: f64,
    pub belt_squad: f64,
    pub line_max: f64,
    pub pistol_near: f64,
    pub pistol_share: f64,
    pub pistol_max: f64,
    pub captor: bool,
    pub use_: bool,
    pub entry: bool,
    pub lanes: bool,
    pub strings: bool,
    pub post: bool,
    pub lane_k: f64,
    pub file_cost: f64,
}
impl GunrolesK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunrolesK {
            on: false,
            deal: false,
            akms: 0.0,
            belt_lane: 0.0,
            belt_min: 0.0,
            belt_squad: 0.0,
            line_max: 0.0,
            pistol_near: 0.0,
            pistol_share: 0.0,
            pistol_max: 0.0,
            captor: false,
            use_: false,
            entry: false,
            lanes: false,
            strings: false,
            post: false,
            lane_k: 0.0,
            file_cost: 0.0,
        }
    }
}
impl Default for GunrolesK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kittree.ts:20 KITTREE, src/game/kittree.ts:20)
#[derive(Clone, Copy, Debug)]
pub struct KittreeK {
    pub on: bool,
    pub rings: f64,
    pub lend: f64,
}
impl KittreeK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KittreeK {
            on: false,
            rings: 0.0,
            lend: 0.0,
        }
    }
}
impl Default for KittreeK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:74 SQUADTHROW, src/game/squadthrow.ts:74)
#[derive(Clone, Debug)]
pub struct SquadthrowK {
    pub on: bool,
    pub frag_roll_k: f64,
    pub again_secs: f64,
    pub shown_secs: f64,
    pub slack_m: f64,
    pub walk: bool,
    pub walk_m: f64,
    pub min_m: f64,
    pub max_m: f64,
    pub tries: f64,
    pub keep_secs: f64,
    pub walk_secs: f64,
    pub turn_secs: f64,
    pub toss_secs: f64,
    pub back: bool,
    pub draw: bool,
    pub lob: bool,
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
    pub lob_keep_secs: f64,
    pub settle_secs: f64,
    pub lob_steps: Vec<f64>,
    pub sam_call: bool,
    pub duck: bool,
    pub duck_secs: f64,
    pub duck_throw_secs: f64,
    pub duck_cells: f64,
    pub duck_apart_m: f64,
    pub duck_min_m: f64,
}
impl SquadthrowK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadthrowK {
            on: false,
            frag_roll_k: 0.0,
            again_secs: 0.0,
            shown_secs: 0.0,
            slack_m: 0.0,
            walk: false,
            walk_m: 0.0,
            min_m: 0.0,
            max_m: 0.0,
            tries: 0.0,
            keep_secs: 0.0,
            walk_secs: 0.0,
            turn_secs: 0.0,
            toss_secs: 0.0,
            back: false,
            draw: false,
            lob: false,
            lob_paces: Vec::new(),
            lob_bank_paces: Vec::new(),
            lob_speed: 0.0,
            lob_near_m: 0.0,
            lob_up_m: 0.0,
            lob_walls: 0.0,
            lob_wall_m: 0.0,
            lob_wall_h: 0.0,
            lob_sims: 0.0,
            lob_cells: 0.0,
            lob_keep_secs: 0.0,
            settle_secs: 0.0,
            lob_steps: Vec::new(),
            sam_call: false,
            duck: false,
            duck_secs: 0.0,
            duck_throw_secs: 0.0,
            duck_cells: 0.0,
            duck_apart_m: 0.0,
            duck_min_m: 0.0,
        }
    }
}
impl Default for SquadthrowK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squad40.ts:53 SQUAD40, src/game/squad40.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct Squad40K {
    pub on: bool,
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
    pub draw: bool,
}
impl Squad40K {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Squad40K {
            on: false,
            near_m: 0.0,
            arm_slack: 0.0,
            reach_slack: 0.0,
            muzzle_ahead: 0.0,
            muzzle_up: 0.0,
            head_drop: 0.0,
            aim_rad: 0.0,
            aim_secs: 0.0,
            raise_secs: 0.0,
            after_secs: 0.0,
            shown_secs: 0.0,
            draw: false,
        }
    }
}
impl Default for Squad40K { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/autopilot.ts:42 AUTOPILOT, src/game/autopilot.ts:42)
#[derive(Clone, Copy, Debug)]
pub struct AutopilotK {
    pub on: bool,
    pub show_secs: f64,
    pub keep_bangs: f64,
    pub contact_m: f64,
    pub replan: bool,
    pub plain: bool,
    pub restack: bool,
}
impl AutopilotK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AutopilotK {
            on: false,
            show_secs: 0.0,
            keep_bangs: 0.0,
            contact_m: 0.0,
            replan: false,
            plain: false,
            restack: false,
        }
    }
}
impl Default for AutopilotK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/planlay.ts:36 PLANLAY, src/game/planlay.ts:36)
#[derive(Clone, Copy, Debug)]
pub struct PlanlayK {
    pub on: bool,
    pub keep: bool,
    pub sticky: bool,
}
impl PlanlayK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlanlayK {
            on: false,
            keep: false,
            sticky: false,
        }
    }
}
impl Default for PlanlayK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cmdring.ts:50 CMDRING, src/game/cmdring.ts:50)
#[derive(Clone, Debug)]
pub struct CmdringK {
    pub on: bool,
    pub key: String,
    pub wedges: f64,
    pub flick_px: f64,
    pub hold_secs: f64,
    pub stay: bool,
    pub stay_hold_secs: f64,
    pub stay_digit: bool,
    pub pin: bool,
    pub tap_now: bool,
    pub tap_secs: f64,
    pub edge_x: f64,
    pub edge_y: f64,
    pub man_px: f64,
    pub ref_: [f64; 2],
    pub hint: bool,
    pub hands: bool,
    pub flat: bool,
    pub plain_hint: bool,
    pub bare_hint: bool,
    pub wheel_idle: bool,
    pub whole_squad: bool,
    pub goals: bool,
    pub goal_ring: bool,
    pub goal_half: bool,
    pub twice: bool,
    pub twice_secs: f64,
    pub twice_px: f64,
    pub only: bool,
    pub hint_wall_m: f64,
    pub hint_spot_m: f64,
    pub hint_man_m: f64,
}
impl CmdringK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CmdringK {
            on: false,
            key: String::new(),
            wedges: 0.0,
            flick_px: 0.0,
            hold_secs: 0.0,
            stay: false,
            stay_hold_secs: 0.0,
            stay_digit: false,
            pin: false,
            tap_now: false,
            tap_secs: 0.0,
            edge_x: 0.0,
            edge_y: 0.0,
            man_px: 0.0,
            ref_: [0.0; 2],
            hint: false,
            hands: false,
            flat: false,
            plain_hint: false,
            bare_hint: false,
            wheel_idle: false,
            whole_squad: false,
            goals: false,
            goal_ring: false,
            goal_half: false,
            twice: false,
            twice_secs: 0.0,
            twice_px: 0.0,
            only: false,
            hint_wall_m: 0.0,
            hint_spot_m: 0.0,
            hint_man_m: 0.0,
        }
    }
}
impl Default for CmdringK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/calmtorch.ts:37 TORCH, src/game/calmtorch.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct TorchK {
    pub calm_off: bool,
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
    pub wall_low: bool,
    pub wall_dark: bool,
    pub wall_m: f64,
    pub wall_back_m: f64,
    pub low_tan: f64,
    pub low_rate: f64,
}
impl TorchK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TorchK {
            calm_off: false,
            lit_at: 0.0,
            off_k: 0.0,
            spill_m: 0.0,
            day_m: 0.0,
            ahead_m: 0.0,
            on_secs: 0.0,
            off_secs: 0.0,
            hold_secs: 0.0,
            ask_secs: 0.0,
            calm_aw: 0.0,
            wall_low: false,
            wall_dark: false,
            wall_m: 0.0,
            wall_back_m: 0.0,
            low_tan: 0.0,
            low_rate: 0.0,
        }
    }
}
impl Default for TorchK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/backkey.ts:14 BACKKEY, src/game/backkey.ts:14)
#[derive(Clone, Copy, Debug)]
pub struct BackkeyK {
    pub tab: bool,
}
impl BackkeyK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BackkeyK {
            tab: false,
        }
    }
}
impl Default for BackkeyK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/host/loop.ts:131 FIXEDSTEP, src/host/loop.ts:131)
#[derive(Clone, Copy, Debug)]
pub struct FixedstepK {
}
impl FixedstepK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FixedstepK {
        }
    }
}
impl Default for FixedstepK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/host/tween.ts:37 TWEEN, src/host/tween.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct TweenK {
}
impl TweenK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TweenK {
        }
    }
}
impl Default for TweenK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/host/savepic.ts:8 SAVEPIC, src/host/savepic.ts:8)
#[derive(Clone, Copy, Debug)]
pub struct SavepicK {
}
impl SavepicK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SavepicK {
        }
    }
}
impl Default for SavepicK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/host/pace.ts:71 EVENPACE, src/host/pace.ts:71)
#[derive(Clone, Copy, Debug)]
pub struct EvenpaceK {
}
impl EvenpaceK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EvenpaceK {
        }
    }
}
impl Default for EvenpaceK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/host/findme.ts:69 FINDTUNED, src/host/findme.ts:69)
#[derive(Clone, Copy, Debug)]
pub struct FindtunedK {
}
impl FindtunedK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FindtunedK {
        }
    }
}
impl Default for FindtunedK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/host/autocam.ts:73 AUTOCAM, src/host/autocam.ts:73)
#[derive(Clone, Copy, Debug)]
pub struct AutocamK {
}
impl AutocamK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AutocamK {
        }
    }
}
impl Default for AutocamK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/morale.ts:89 MORALE, src/game/morale.ts:89)
#[derive(Clone, Copy, Debug)]
pub struct MoraleK {
    pub on: bool,
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
impl MoraleK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MoraleK {
            on: false,
            half_life: 0.0,
            ask_secs: 0.0,
            alone: 0.0,
            alone_m: 0.0,
            dark: 0.0,
            lit_asks: 0.0,
            blast: 0.0,
            shock_cap: 0.0,
            blast_m: 0.0,
            near_m: 0.0,
            far_k: 0.0,
            jumpy: 0.0,
            broken: 0.0,
            hyst: 0.0,
            duck_secs: 0.0,
            down_secs: 0.0,
            back_until: 0.0,
            back_m: 0.0,
            back_speed: 0.0,
            burst_rounds: 0.0,
            burst_every: 0.0,
            spread: 0.0,
            call2_secs: 0.0,
            bait_m: 0.0,
            bait_short: 0.0,
            bait_every: 0.0,
            reload_secs: 0.0,
            bark_secs: 0.0,
            fan_deg: 0.0,
            fan_step: 0.0,
            straight_m: 0.0,
            training: crate::morale::Training::None,
            green_broken: 0.0,
            leader_down: 0.0,
            leader_shock: 0.0,
            flinch_secs: 0.0,
            shout_gap: 0.0,
            order_m: 0.0,
            order_beat: 0.0,
            order_secs: 0.0,
            order_gap: 0.0,
            stand_secs: 0.0,
            side_m: 0.0,
            steady_duck: 0.0,
            hold_shock: 0.0,
            hold_max: 0.0,
            nerve: crate::morale::Nerve::Steady,
            nerve_k: crate::knobs::MoraleKNerveK::zeroed(),
            dark_read: crate::knobs::MoraleKDarkRead::Place,
        }
    }
}
impl Default for MoraleK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/morale.ts:93 MoraleK.nerveK, src/game/morale.ts:93)
#[derive(Clone, Copy, Debug)]
pub struct MoraleKNerveK {
    pub brittle: f64,
    pub normal: f64,
    pub steady: f64,
}
impl MoraleKNerveK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MoraleKNerveK {
            brittle: 0.0,
            normal: 0.0,
            steady: 0.0,
        }
    }
}
impl Default for MoraleKNerveK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MoraleKDarkRead {
    Place,
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

/// (TypeScript: inline at src/game/condition.ts:26 CONDITION, src/game/condition.ts:26)
#[derive(Clone, Copy, Debug)]
pub struct ConditionK {
    pub on: bool,
    pub hurt_pace: f64,
    pub hurt_sup: f64,
    pub hurt_secs: f64,
    pub own_calm: bool,
    pub looks: bool,
}
impl ConditionK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ConditionK {
            on: false,
            hurt_pace: 0.0,
            hurt_sup: 0.0,
            hurt_secs: 0.0,
            own_calm: false,
            looks: false,
        }
    }
}
impl Default for ConditionK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/inhand.ts:73 INHAND, src/game/inhand.ts:73)
#[derive(Clone, Copy, Debug)]
pub struct InhandK {
    pub on: bool,
    pub from: f64,
    pub to: f64,
    pub wrist: f64,
    pub both: f64,
    pub rate: f64,
    pub fit: bool,
    pub torch: bool,
    pub long: bool,
    pub holster_out: f64,
    pub belt: bool,
}
impl InhandK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        InhandK {
            on: false,
            from: 0.0,
            to: 0.0,
            wrist: 0.0,
            both: 0.0,
            rate: 0.0,
            fit: false,
            torch: false,
            long: false,
            holster_out: 0.0,
            belt: false,
        }
    }
}
impl Default for InhandK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/packside.ts:39 PACKSIDE, src/game/packside.ts:39)
#[derive(Clone, Copy, Debug)]
pub struct PacksideK {
    pub on: bool,
    pub ram: bool,
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
impl PacksideK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PacksideK {
            on: false,
            ram: false,
            gap: 0.0,
            handle: 0.0,
            gun: 0.0,
            over: 0.0,
            head_min: 0.0,
            butt: 0.0,
            rest: 0.0,
            pitch0: 0.0,
            pitch1: 0.0,
        }
    }
}
impl Default for PacksideK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunhand.ts:15 GUNHAND, src/game/gunhand.ts:15)
#[derive(Clone, Copy, Debug)]
pub struct GunhandK {
    pub on: bool,
    pub whistle: bool,
    pub throw_free: bool,
    pub long_reload: bool,
}
impl GunhandK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunhandK {
            on: false,
            whistle: false,
            throw_free: false,
            long_reload: false,
        }
    }
}
impl Default for GunhandK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/runner.ts:86 RUNNER, src/game/runner.ts:86)
#[derive(Clone, Copy, Debug)]
pub struct RunnerK {
    pub on: bool,
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
    pub retries: f64,
    pub replan_secs: f64,
    pub ahead_m: f64,
    pub ask_cells: f64,
    pub ask_wide: f64,
    pub anyway: bool,
    pub nobody: bool,
}
impl RunnerK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunnerK {
            on: false,
            close_m: 0.0,
            out_secs: 0.0,
            beat_secs: 0.0,
            tell_m: 0.0,
            told_m: 0.0,
            tell_beat: 0.0,
            tell_aware: 0.0,
            shy_m: 0.0,
            mate_m: 0.0,
            mate_secs: 0.0,
            clear_m: 0.0,
            edge_m: 0.0,
            corner_m: 0.0,
            gone_secs: 0.0,
            stall_secs: 0.0,
            retry_secs: 0.0,
            retries: 0.0,
            replan_secs: 0.0,
            ahead_m: 0.0,
            ask_cells: 0.0,
            ask_wide: 0.0,
            anyway: false,
            nobody: false,
        }
    }
}
impl Default for RunnerK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/startle.ts:40 STARTLE, src/game/startle.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct StartleK {
    pub on: bool,
    pub beat: f64,
    pub draw: f64,
    pub fresh: f64,
    pub late: f64,
    pub run: bool,
    pub run_m: f64,
    pub near_m: f64,
    pub clear_secs: f64,
    pub run_secs: f64,
    pub stuck_secs: f64,
    pub stuck_m: f64,
}
impl StartleK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StartleK {
            on: false,
            beat: 0.0,
            draw: 0.0,
            fresh: 0.0,
            late: 0.0,
            run: false,
            run_m: 0.0,
            near_m: 0.0,
            clear_secs: 0.0,
            run_secs: 0.0,
            stuck_secs: 0.0,
            stuck_m: 0.0,
        }
    }
}
impl Default for StartleK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rowset.ts:27 ROWSET, src/game/rowset.ts:27)
#[derive(Clone, Copy, Debug)]
pub struct RowsetK {
    pub on: bool,
}
impl RowsetK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RowsetK {
            on: false,
        }
    }
}
impl Default for RowsetK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/smokebreak.ts:26 SMOKEBREAK, src/game/smokebreak.ts:26)
#[derive(Clone, Copy, Debug)]
pub struct SmokebreakK {
    pub on: bool,
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
    pub thread_gone: f64,
    pub breath: crate::knobs::SmokebreakKBreath,
    pub puffs: crate::knobs::SmokebreakKPuffs,
    pub color: V3,
    pub mouth_ahead: f64,
    pub mouth_down: f64,
}
impl SmokebreakK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokebreakK {
            on: false,
            light_secs: 0.0,
            light_up: [0.0; 2],
            flame_at: 0.0,
            flame: 0.0,
            every: 0.0,
            draw: 0.0,
            out_after: 0.0,
            out: 0.0,
            flame_color: [0.0; 3],
            flame_k: 0.0,
            flame_range: 0.0,
            ember_color: [0.0; 3],
            ember_k: 0.0,
            ember_draw_k: 0.0,
            ember_range: 0.0,
            ember_every: 0.0,
            tip_out: 0.0,
            tip_up: 0.0,
            thread: crate::knobs::SmokebreakKThread::zeroed(),
            thread_gone: 0.0,
            breath: crate::knobs::SmokebreakKBreath::zeroed(),
            puffs: crate::knobs::SmokebreakKPuffs::zeroed(),
            color: [0.0; 3],
            mouth_ahead: 0.0,
            mouth_down: 0.0,
        }
    }
}
impl Default for SmokebreakK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/smokebreak.ts:38 SmokebreakK.thread, src/game/smokebreak.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct SmokebreakKThread {
    pub radius: f64,
    pub cells: f64,
    pub density: f64,
    pub temp: f64,
    pub speed: f64,
    pub push: f64,
    pub jitter: f64,
}
impl SmokebreakKThread {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokebreakKThread {
            radius: 0.0,
            cells: 0.0,
            density: 0.0,
            temp: 0.0,
            speed: 0.0,
            push: 0.0,
            jitter: 0.0,
        }
    }
}
impl Default for SmokebreakKThread { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/smokebreak.ts:42 SmokebreakK.breath, src/game/smokebreak.ts:42)
#[derive(Clone, Copy, Debug)]
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
impl SmokebreakKBreath {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokebreakKBreath {
            radius: 0.0,
            cells: 0.0,
            density: 0.0,
            temp: 0.0,
            speed: 0.0,
            push: 0.0,
            expand: 0.0,
            attack: 0.0,
            jitter: 0.0,
            down: 0.0,
        }
    }
}
impl Default for SmokebreakKBreath { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/smokebreak.ts:45 SmokebreakK.puffs, src/game/smokebreak.ts:45)
#[derive(Clone, Copy, Debug)]
pub struct SmokebreakKPuffs {
    pub records: f64,
    pub count: f64,
    pub speed: f64,
    pub spread: f64,
    pub life: f64,
    pub size: f64,
    pub opacity: f64,
}
impl SmokebreakKPuffs {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokebreakKPuffs {
            records: 0.0,
            count: 0.0,
            speed: 0.0,
            spread: 0.0,
            life: 0.0,
            size: 0.0,
            opacity: 0.0,
        }
    }
}
impl Default for SmokebreakKPuffs { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/showwhy.ts:53 SHOWWHY, src/game/showwhy.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct ShowwhyK {
    pub on: bool,
    pub line: bool,
    pub stack: bool,
    pub door: bool,
    pub hurt: bool,
    pub keep: bool,
    pub say: bool,
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
impl ShowwhyK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShowwhyK {
            on: false,
            line: false,
            stack: false,
            door: false,
            hurt: false,
            keep: false,
            say: false,
            line_after: 0.0,
            line_gap: 0.0,
            wave_every: 0.0,
            wave_later: 0.0,
            line_min: 0.0,
            line_say_gap: 0.0,
            mate_half: 0.0,
            squeeze_gap: 0.0,
            squeeze_reach: 0.0,
            fist_after: 0.0,
            hold_every: 0.0,
            hold_max: 0.0,
            door_after: 0.0,
            door_every: 0.0,
            after_you_gap: 0.0,
            hurt_late: 0.0,
            lag_m: 0.0,
            lag_every: 0.0,
            lag_max: 0.0,
            keep_after: 0.0,
            keep_gap: 0.0,
            keep_every: 0.0,
            owed_for: 0.0,
        }
    }
}
impl Default for ShowwhyK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fire.ts:53 FIRE, src/game/fire.ts:53)
#[derive(Clone, Debug)]
pub struct FireK {
    pub on: bool,
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
    pub hardy: bool,
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
    pub gas: crate::knobs::FireKGas,
    pub bed: f64,
    pub crack: f64,
    pub bed_secs: f64,
    pub fog: bool,
    pub max: f64,
    pub draw_r: f64,
    pub wind_max: f64,
}
impl FireK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireK {
            on: false,
            names: Vec::new(),
            height: 0.0,
            mouth: 0.0,
            coals: 0.0,
            coal_swing: 0.0,
            bed_r: 0.0,
            depth: 0.0,
            rates: Vec::new(),
            weights: Vec::new(),
            least: 0.0,
            most: 0.0,
            wander: 0.0,
            wander_up: 0.0,
            lift: 0.0,
            kick: 0.0,
            kick_secs: 0.0,
            bright: 0.0,
            warm: Vec::new(),
            reach: 0.0,
            hardy: false,
            stir_gap: 0.0,
            size: 0.0,
            pop_rate: 0.0,
            pop_slot: 0.0,
            embers: 0.0,
            ember_life: 0.0,
            ember_speed: 0.0,
            ember_size: 0.0,
            ember_gain: 0.0,
            ember_crack: 0.0,
            ember_r: 0.0,
            look: crate::knobs::FireKLook::zeroed(),
            smoke: crate::knobs::FireKSmoke::zeroed(),
            gas: crate::knobs::FireKGas::zeroed(),
            bed: 0.0,
            crack: 0.0,
            bed_secs: 0.0,
            fog: false,
            max: 0.0,
            draw_r: 0.0,
            wind_max: 0.0,
        }
    }
}
impl Default for FireK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fire.ts:110 FireK.look, src/game/fire.ts:110)
#[derive(Clone, Copy, Debug)]
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
impl FireKLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireKLook {
            gain: 0.0,
            core: 0.0,
            sigma: 0.0,
            speed: 0.0,
            tall: 0.0,
            torn: 0.0,
            plume: 0.0,
            shimmer: 0.0,
            soot: 0.0,
            breathe: 0.0,
            react: 0.0,
            lean: 0.0,
            steps: 0.0,
        }
    }
}
impl Default for FireKLook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fire.ts:124 FireK.smoke, src/game/fire.ts:124)
#[derive(Clone, Copy, Debug)]
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
impl FireKSmoke {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireKSmoke {
            on: false,
            up: 0.0,
            radius: 0.0,
            cells: 0.0,
            density: 0.0,
            temp: 0.0,
            speed: 0.0,
            push: 0.0,
            jitter: 0.0,
            every: 0.0,
            view_r: 0.0,
            lean: 0.0,
        }
    }
}
impl Default for FireKSmoke { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fire.ts:139 FireK.gas, src/game/fire.ts:139)
#[derive(Clone, Copy, Debug)]
pub struct FireKGas {
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
    pub keep: f64,
    pub ref_: f64,
    pub lo: f64,
    pub ease: f64,
}
impl FireKGas {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireKGas {
            on: false,
            up: 0.0,
            radius: 0.0,
            cells: 0.0,
            density: 0.0,
            temp: 0.0,
            speed: 0.0,
            push: 0.0,
            jitter: 0.0,
            every: 0.0,
            view_r: 0.0,
            keep: 0.0,
            ref_: 0.0,
            lo: 0.0,
            ease: 0.0,
        }
    }
}
impl Default for FireKGas { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/flinch.ts:44 FLINCH, src/game/flinch.ts:44)
#[derive(Clone, Copy, Debug)]
pub struct FlinchK {
    pub on: bool,
    pub least: f64,
    pub full: f64,
    pub far: f64,
    pub min_travel: f64,
    pub steady_k: f64,
    pub friendly: bool,
    pub run: bool,
    pub break_secs: f64,
    pub hot_secs: f64,
    pub ease_k: f64,
    pub ask_secs: f64,
}
impl FlinchK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlinchK {
            on: false,
            least: 0.0,
            full: 0.0,
            far: 0.0,
            min_travel: 0.0,
            steady_k: 0.0,
            friendly: false,
            run: false,
            break_secs: 0.0,
            hot_secs: 0.0,
            ease_k: 0.0,
            ask_secs: 0.0,
        }
    }
}
impl Default for FlinchK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/darkdoor.ts:59 DARKDOOR, src/game/darkdoor.ts:59)
#[derive(Clone, Copy, Debug)]
pub struct DarkdoorK {
    pub on: bool,
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
impl DarkdoorK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DarkdoorK {
            on: false,
            who: crate::knobs::DarkdoorKWho::None,
            stop_m: 0.0,
            pair_m: 0.0,
            dark_at: 0.0,
            dark_share: 0.0,
            lit_by: 0.0,
            samples: 0.0,
            ask_secs: 0.0,
            open_rad: 0.0,
            arm_secs: 0.0,
            stand_m: 0.0,
            side_m: 0.0,
            sweep_secs: 0.0,
            ear_m: 0.0,
            mate_m: 0.0,
            mate_try: 0.0,
            call_secs: 0.0,
            copy_secs: 0.0,
            lag_secs: 0.0,
            with_secs: 0.0,
            keep_aw: 0.0,
            keep_secs: 0.0,
            resume_secs: 0.0,
            seen_secs: 0.0,
            wait_secs: 0.0,
            again_secs: 0.0,
            fed_up_secs: 0.0,
            fed_up_n: 0.0,
            nerve_secs: 0.0,
            slow_k: 0.0,
        }
    }
}
impl Default for DarkdoorK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DarkdoorKWho {
    None,
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

/// (TypeScript: inline at src/game/darkfoe.ts:55 DARKFOE, src/game/darkfoe.ts:55)
#[derive(Clone, Copy, Debug)]
pub struct DarkfoeK {
    pub on: bool,
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
    pub by_lamp: bool,
    pub lamp_m: f64,
    pub suspect: bool,
    pub seek: bool,
    pub blind_secs: f64,
    pub seek_m: f64,
    pub walk_m: f64,
    pub edge_m: f64,
    pub stumble_k: f64,
    pub again_secs: f64,
    pub moves: f64,
    pub quiet_secs: f64,
    pub calm_secs: f64,
    pub roam: bool,
    pub edge: bool,
}
impl DarkfoeK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DarkfoeK {
            on: false,
            carry: 0.0,
            lit_at: 0.0,
            look_m: 0.0,
            step_m: 0.0,
            fan_rad: 0.0,
            see_share: 0.0,
            ask_secs: 0.0,
            on_secs: 0.0,
            on_jitter: 0.0,
            off_secs: 0.0,
            off_jitter: 0.0,
            hold_secs: 0.0,
            by_lamp: false,
            lamp_m: 0.0,
            suspect: false,
            seek: false,
            blind_secs: 0.0,
            seek_m: 0.0,
            walk_m: 0.0,
            edge_m: 0.0,
            stumble_k: 0.0,
            again_secs: 0.0,
            moves: 0.0,
            quiet_secs: 0.0,
            calm_secs: 0.0,
            roam: false,
            edge: false,
        }
    }
}
impl Default for DarkfoeK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/powerout.ts:69 POWEROUT, src/game/powerout.ts:69)
#[derive(Clone, Copy, Debug)]
pub struct PoweroutK {
    pub on: bool,
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
    pub lone: bool,
    pub stayers: bool,
    pub parties: f64,
    pub look_secs: f64,
    pub tries: f64,
    pub lever_secs: f64,
    pub overdue: f64,
    pub ask: f64,
    pub cover_m: [f64; 2],
}
impl PoweroutK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PoweroutK {
            on: false,
            see_m: 0.0,
            think: [0.0; 2],
            beat: 0.0,
            look_m: 0.0,
            about_m: 0.0,
            step_m: 0.0,
            fan_rad: 0.0,
            lit_at: 0.0,
            drop_k: 0.0,
            dark_after: 0.0,
            men: 0.0,
            max_m: 0.0,
            lone: false,
            stayers: false,
            parties: 0.0,
            look_secs: 0.0,
            tries: 0.0,
            lever_secs: 0.0,
            overdue: 0.0,
            ask: 0.0,
            cover_m: [0.0; 2],
        }
    }
}
impl Default for PoweroutK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squaddeal.ts:28 SQUADDEAL, src/game/squaddeal.ts:28)
#[derive(Clone, Debug)]
pub struct SquaddealK {
    pub on: bool,
    pub one: f64,
    pub two: f64,
    pub names: Vec<String>,
}
impl SquaddealK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquaddealK {
            on: false,
            one: 0.0,
            two: 0.0,
            names: Vec::new(),
        }
    }
}
impl Default for SquaddealK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stayask.ts:26 STAYASK, src/game/stayask.ts:26)
#[derive(Clone, Copy, Debug)]
pub struct StayaskK {
    pub on: bool,
    pub men: f64,
}
impl StayaskK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StayaskK {
            on: false,
            men: 0.0,
        }
    }
}
impl Default for StayaskK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescuecheck.ts:55 RESCUECHECK, src/game/rescuecheck.ts:55)
#[derive(Clone, Copy, Debug)]
pub struct RescuecheckK {
    pub on: bool,
    pub first: [f64; 2],
    pub every: [f64; 2],
    pub ask_secs: f64,
    pub go_secs: f64,
    pub wait_secs: f64,
    pub tries: f64,
    pub feet_m: f64,
    pub word_secs: f64,
}
impl RescuecheckK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescuecheckK {
            on: false,
            first: [0.0; 2],
            every: [0.0; 2],
            ask_secs: 0.0,
            go_secs: 0.0,
            wait_secs: 0.0,
            tries: 0.0,
            feet_m: 0.0,
            word_secs: 0.0,
        }
    }
}
impl Default for RescuecheckK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kitload.ts:52 KITLOAD, src/game/kitload.ts:52)
#[derive(Clone, Debug)]
pub struct KitloadK {
    pub on: bool,
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
impl KitloadK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitloadK {
            on: false,
            gun_kg: 0.0,
            gun_bulk: [0.0; 2],
            small: JsMap::new(),
            big: JsMap::new(),
            pace: 0.0,
            seen: 0.0,
            heard: 0.0,
            sprint: 0.0,
            pace_top: 0.0,
        }
    }
}
impl Default for KitloadK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescueend.ts:28 RESCUEEND, src/game/rescueend.ts:28)
#[derive(Clone, Copy, Debug)]
pub struct RescueendK {
    pub on: bool,
    pub out_m: f64,
    pub clear_secs: f64,
    pub marker_m: f64,
    pub reach_m: f64,
    pub gap_m: f64,
    pub gap_step_m: f64,
}
impl RescueendK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueendK {
            on: false,
            out_m: 0.0,
            clear_secs: 0.0,
            marker_m: 0.0,
            reach_m: 0.0,
            gap_m: 0.0,
            gap_step_m: 0.0,
        }
    }
}
impl Default for RescueendK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescueflush.ts:41 RESCUEFLUSH, src/game/rescueflush.ts:41)
#[derive(Clone, Copy, Debug)]
pub struct RescueflushK {
    pub on: bool,
    pub after_secs: f64,
    pub gap_secs: f64,
    pub again_secs: f64,
    pub reach_m: f64,
    pub kill_secs: f64,
    pub busy_secs: f64,
    pub smoke_k: f64,
    pub at_door_m: f64,
    pub lanes_choose: bool,
}
impl RescueflushK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueflushK {
            on: false,
            after_secs: 0.0,
            gap_secs: 0.0,
            again_secs: 0.0,
            reach_m: 0.0,
            kill_secs: 0.0,
            busy_secs: 0.0,
            smoke_k: 0.0,
            at_door_m: 0.0,
            lanes_choose: false,
        }
    }
}
impl Default for RescueflushK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatchout.ts:31 SNATCHOUT, src/game/snatchout.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct SnatchoutK {
    pub on: bool,
    pub hold: bool,
    pub marker_m: f64,
    pub reach_m: f64,
    pub gap_m: f64,
    pub send_m: f64,
    pub send_him_m: f64,
    pub in_m: f64,
    pub carry_m: f64,
    pub trail_m: f64,
    pub close_m: f64,
    pub turn_secs: f64,
    pub lane_m: f64,
    pub off_m: f64,
    pub spot_m: f64,
}
impl SnatchoutK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchoutK {
            on: false,
            hold: false,
            marker_m: 0.0,
            reach_m: 0.0,
            gap_m: 0.0,
            send_m: 0.0,
            send_him_m: 0.0,
            in_m: 0.0,
            carry_m: 0.0,
            trail_m: 0.0,
            close_m: 0.0,
            turn_secs: 0.0,
            lane_m: 0.0,
            off_m: 0.0,
            spot_m: 0.0,
        }
    }
}
impl Default for SnatchoutK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/anim/skeleton.ts:289 POSEMATH, src/anim/skeleton.ts:289)
#[derive(Clone, Copy, Debug)]
pub struct PosemathK {
    pub singles: bool,
}
impl PosemathK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PosemathK {
            singles: false,
        }
    }
}
impl Default for PosemathK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sense.ts:33 SENSE, src/game/sense.ts:33)
#[derive(Clone, Copy, Debug)]
pub struct SenseK {
    pub cpu: bool,
    pub grid: bool,
    pub bounce: bool,
    pub drift: bool,
    pub moon: bool,
    pub glow: crate::knobs::SenseKGlow,
    pub smoke: crate::knobs::SenseKSmoke,
}
impl SenseK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SenseK {
            cpu: false,
            grid: false,
            bounce: false,
            drift: false,
            moon: false,
            glow: crate::knobs::SenseKGlow::zeroed(),
            smoke: crate::knobs::SenseKSmoke::zeroed(),
        }
    }
}
impl Default for SenseK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sense.ts:64 SenseK.glow, src/game/sense.ts:64)
#[derive(Clone, Copy, Debug)]
pub struct SenseKGlow {
    pub albedo: f64,
    pub share: f64,
    pub passes: f64,
    pub sweep: f64,
}
impl SenseKGlow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SenseKGlow {
            albedo: 0.0,
            share: 0.0,
            passes: 0.0,
            sweep: 0.0,
        }
    }
}
impl Default for SenseKGlow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sense.ts:65 SenseK.smoke, src/game/sense.ts:65)
#[derive(Clone, Copy, Debug)]
pub struct SenseKSmoke {
    pub sigma: f64,
    pub decay: f64,
    pub spread: f64,
    pub widest: f64,
    pub rise: f64,
    pub rise_rate: f64,
    pub in_wind: f64,
    pub least: f64,
    pub gone: f64,
    pub cap: f64,
    pub emitters: f64,
    pub reserve: f64,
}
impl SenseKSmoke {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SenseKSmoke {
            sigma: 0.0,
            decay: 0.0,
            spread: 0.0,
            widest: 0.0,
            rise: 0.0,
            rise_rate: 0.0,
            in_wind: 0.0,
            least: 0.0,
            gone: 0.0,
            cap: 0.0,
            emitters: 0.0,
            reserve: 0.0,
        }
    }
}
impl Default for SenseKSmoke { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cmd.ts:70 CMDCHECK, src/game/cmd.ts:70)
#[derive(Clone, Copy, Debug)]
pub struct CmdcheckK {
    pub on: bool,
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
impl CmdcheckK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CmdcheckK {
            on: false,
            keys: 0.0,
            key_len: 0.0,
            span: 0.0,
            wheel: 0.0,
            far: 0.0,
            dist_min: 0.0,
            dist_max: 0.0,
            fov_min: 0.0,
            fov_max: 0.0,
            aspect_min: 0.0,
            aspect_max: 0.0,
            reach: 0.0,
            steep: 0.0,
        }
    }
}
impl Default for CmdcheckK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/seats.ts:31 SEATS, src/game/seats.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct SeatsK {
    pub max: f64,
    pub apart: f64,
    pub keep: f64,
}
impl SeatsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SeatsK {
            max: 0.0,
            apart: 0.0,
            keep: 0.0,
        }
    }
}
impl Default for SeatsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doze.ts:69 DOZE, src/game/doze.ts:69)
#[derive(Clone, Debug)]
pub struct DozeK {
    pub on: bool,
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
    pub screen: bool,
    pub screen_m: f64,
    pub pull_m: f64,
    pub nap_wait: f64,
    pub settle_secs: f64,
    pub ears: f64,
    pub noises: f64,
    pub say_first: Vec<f64>,
    pub say_every: Vec<f64>,
    pub idle_secs: f64,
    pub lend: bool,
}
impl DozeK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DozeK {
            on: false,
            min_wait: 0.0,
            max_wait: 0.0,
            chair_m: 0.0,
            nod_secs: 0.0,
            rise_secs: 0.0,
            hear_k: 0.0,
            light_at: 0.0,
            light_secs: 0.0,
            ask_secs: 0.0,
            wake_aw: 0.0,
            bump_aw: 0.0,
            stir_secs: 0.0,
            breath_secs: 0.0,
            screen: false,
            screen_m: 0.0,
            pull_m: 0.0,
            nap_wait: 0.0,
            settle_secs: 0.0,
            ears: 0.0,
            noises: 0.0,
            say_first: Vec::new(),
            say_every: Vec::new(),
            idle_secs: 0.0,
            lend: false,
        }
    }
}
impl Default for DozeK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/netquiet.ts:81 NETQUIET, src/game/netquiet.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct NetquietK {
    pub on: bool,
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
impl NetquietK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetquietK {
            on: false,
            unanswered: 0.0,
            heard_it: 0.0,
            body: 0.0,
            body_told: 0.0,
            fall_seen: 0.0,
            fall_unseen: 0.0,
            steady_k: 0.0,
            ask_beat: 0.0,
            body_beat: 0.0,
            bark_secs: 0.0,
            say_hold: 0.0,
            shooter_secs: 0.0,
            body_m: 0.0,
            whip_shift: 0.0,
            whip_secs: 0.0,
            whip_hold: 0.0,
            whip_turn: 0.0,
            whip_turn_broken: 0.0,
            whip_m: 0.0,
            whip_near_m: 0.0,
            last_beat: 0.0,
            calm_secs: 0.0,
            alert_secs: 0.0,
            give_up: 0.0,
            mate_m: 0.0,
            arc_deg: 0.0,
            eye_deg: 0.0,
            eye_slack: 0.0,
            crouch: 0.0,
            wary: 0.0,
            back_speed: 0.0,
            max_m: 0.0,
            door_m: 0.0,
            eye_m: 0.0,
            stay_m: 0.0,
        }
    }
}
impl Default for NetquietK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/attention.ts:41 ATTENTION, src/game/attention.ts:41)
#[derive(Clone, Copy, Debug)]
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
impl AttentionK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AttentionK {
            on: false,
            focus_deg: 0.0,
            side_k: 0.0,
            ear_k: 0.0,
            break_gain: 0.0,
            calm_aw: 0.0,
            draw_aw: 0.0,
            floor: 0.0,
            whistle_secs: 0.0,
            noise_secs: 0.0,
            door_secs: 0.0,
            light_secs: 0.0,
            tell_secs: 0.0,
            dwell_sweep: 0.0,
            walk_cap: 0.0,
            wear_k: 0.0,
            memory_secs: 0.0,
            spent: 0.0,
            same_secs: 0.0,
            lent_back_secs: 0.0,
            marks: false,
            ring_m: 0.0,
            way: false,
            way_reask: 0.0,
        }
    }
}
impl Default for AttentionK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/picture.ts:30 PICTURE, src/game/picture.ts:30)
#[derive(Clone, Copy, Debug)]
pub struct PictureK {
    pub on: bool,
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
    pub standoff: bool,
    pub cover: bool,
    pub wall: bool,
    pub flee: bool,
    pub flee_cost: f64,
    pub fallback: bool,
    pub stack: bool,
    pub stack_k: f64,
    pub step: bool,
    pub step_at: f64,
    pub step_free: f64,
    pub step_m: f64,
    pub step_after: f64,
    pub step_rest: f64,
    pub step_secs: f64,
    pub shot: bool,
    pub run: bool,
    pub cover_hard: bool,
    pub show: crate::knobs::PictureKShow,
    pub show_m: f64,
}
impl PictureK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PictureK {
            on: false,
            look_secs: 0.0,
            glimpse: 0.0,
            tell_secs: 0.0,
            shout_m: 0.0,
            pop_m: 0.0,
            made_out: 0.0,
            made_out_m: 0.0,
            fresh_secs: 0.0,
            stale_secs: 0.0,
            stale_k: 0.0,
            forget_secs: 0.0,
            lost_k: 0.0,
            walk_mps: 0.0,
            walk_max: 0.0,
            heard_m: 0.0,
            maybe_k: 0.0,
            reach_m: 0.0,
            eye_h: 0.0,
            chest_h: 0.0,
            low_h: 0.0,
            head_up: 0.0,
            standoff: false,
            cover: false,
            wall: false,
            flee: false,
            flee_cost: 0.0,
            fallback: false,
            stack: false,
            stack_k: 0.0,
            step: false,
            step_at: 0.0,
            step_free: 0.0,
            step_m: 0.0,
            step_after: 0.0,
            step_rest: 0.0,
            step_secs: 0.0,
            shot: false,
            run: false,
            cover_hard: false,
            show: crate::knobs::PictureKShow::All,
            show_m: 0.0,
        }
    }
}
impl Default for PictureK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PictureKShow {
    All,
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

/// (TypeScript: inline at src/game/qrf.ts:97 RESPONSE, src/game/qrf.ts:97)
#[derive(Clone, Copy, Debug)]
pub struct ResponseK {
    pub on: bool,
    pub deal: bool,
    pub net: bool,
    pub clear_m: f64,
    pub eta_step: f64,
    pub word_secs: f64,
    pub raid: bool,
    pub call_secs: f64,
}
impl ResponseK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResponseK {
            on: false,
            deal: false,
            net: false,
            clear_m: 0.0,
            eta_step: 0.0,
            word_secs: 0.0,
            raid: false,
            call_secs: 0.0,
        }
    }
}
impl Default for ResponseK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handwork.ts:36 HANDWORK, src/game/handwork.ts:36)
#[derive(Clone, Copy, Debug)]
pub struct HandworkK {
    pub on: bool,
    pub work: bool,
    pub radio: bool,
    pub swap: bool,
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
impl HandworkK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HandworkK {
            on: false,
            work: false,
            radio: false,
            swap: false,
            stand_m: 0.0,
            step_rate: 0.0,
            step_pace: 0.0,
            stand_side: 0.0,
            wait_secs: 0.0,
            lever_secs: 0.0,
            lever_at: 0.0,
            stow_secs: 0.0,
            radio_k: 0.0,
            radio_min: 0.0,
            radio_max: 0.0,
        }
    }
}
impl Default for HandworkK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/workstow.ts:26 WORKSTOW, src/game/workstow.ts:26)
#[derive(Clone, Copy, Debug)]
pub struct WorkstowK {
    pub on: bool,
    pub mate_down: f64,
    pub mate_up: f64,
    pub show: f64,
    pub down_max: f64,
    pub sam_up: f64,
    pub fight_cap: f64,
}
impl WorkstowK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkstowK {
            on: false,
            mate_down: 0.0,
            mate_up: 0.0,
            show: 0.0,
            down_max: 0.0,
            sam_up: 0.0,
            fight_cap: 0.0,
        }
    }
}
impl Default for WorkstowK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadnet.ts:39 SQUADNET, src/game/squadnet.ts:39)
#[derive(Clone, Copy, Debug)]
pub struct SquadnetK {
    pub on: bool,
    pub why: bool,
    pub clear: bool,
    pub end_call: bool,
    pub near: f64,
    pub edge: bool,
    pub edge_in: f64,
    pub click: f64,
    pub click_rate: f64,
    pub down_gain: f64,
    pub down_rate: f64,
    pub down_secs: f64,
}
impl SquadnetK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadnetK {
            on: false,
            why: false,
            clear: false,
            end_call: false,
            near: 0.0,
            edge: false,
            edge_in: 0.0,
            click: 0.0,
            click_rate: 0.0,
            down_gain: 0.0,
            down_rate: 0.0,
            down_secs: 0.0,
        }
    }
}
impl Default for SquadnetK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/takedown.ts:37 TAKEDOWN, src/game/takedown.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct TakedownK {
    pub on: bool,
    pub blow: bool,
    pub victim: bool,
    pub fall: bool,
    pub partner: bool,
    pub breathe: bool,
    pub kill: bool,
    pub choke_beats: bool,
    pub haul: bool,
    pub lower: bool,
    pub partner_hit: f64,
    pub partner_end: f64,
    pub breath_hz: f64,
    pub breath_rise: f64,
    pub breath_in: f64,
}
impl TakedownK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TakedownK {
            on: false,
            blow: false,
            victim: false,
            fall: false,
            partner: false,
            breathe: false,
            kill: false,
            choke_beats: false,
            haul: false,
            lower: false,
            partner_hit: 0.0,
            partner_end: 0.0,
            breath_hz: 0.0,
            breath_rise: 0.0,
            breath_in: 0.0,
        }
    }
}
impl Default for TakedownK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stealthpose.ts:27 STEALTHPOSE, src/game/stealthpose.ts:27)
#[derive(Clone, Copy, Debug)]
pub struct StealthposeK {
    pub on: bool,
    pub idle: bool,
    pub walk: bool,
    pub rise: bool,
    pub creep: bool,
    pub cable: bool,
    pub nod: bool,
    pub sway_secs: f64,
    pub sway_m: f64,
    pub drift_m: f64,
    pub rise_secs: f64,
    pub cable_in: f64,
    pub cable_bow: f64,
    pub cable_step: f64,
}
impl StealthposeK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StealthposeK {
            on: false,
            idle: false,
            walk: false,
            rise: false,
            creep: false,
            cable: false,
            nod: false,
            sway_secs: 0.0,
            sway_m: 0.0,
            drift_m: 0.0,
            rise_secs: 0.0,
            cable_in: 0.0,
            cable_bow: 0.0,
            cable_step: 0.0,
        }
    }
}
impl Default for StealthposeK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/look.ts:10 LOOK, src/scene/look.ts:10)
#[derive(Clone, Copy, Debug)]
pub struct LookK {
    pub on: bool,
    pub suit: bool,
}
impl LookK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LookK {
            on: false,
            suit: false,
        }
    }
}
impl Default for LookK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/audio/pieces.ts:27 MUSIC, src/audio/pieces.ts:27)
#[derive(Clone, Copy, Debug)]
pub struct MusicK {
}
impl MusicK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MusicK {
        }
    }
}
impl Default for MusicK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/lived.ts:11 LIVED, src/scene/lived.ts:11)
#[derive(Clone, Copy, Debug)]
pub struct LivedK {
    pub on: bool,
}
impl LivedK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LivedK {
            on: false,
        }
    }
}
impl Default for LivedK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:86 UNDERFOOT, src/scene/collision.ts:86)
#[derive(Clone, Copy, Debug)]
pub struct UnderfootK {
    pub on: bool,
}
impl UnderfootK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnderfootK {
            on: false,
        }
    }
}
impl Default for UnderfootK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/round.ts:35 ROUND, src/scene/round.ts:35)
#[derive(Clone, Copy, Debug)]
pub struct RoundK {
    pub on: bool,
}
impl RoundK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoundK {
            on: false,
        }
    }
}
impl Default for RoundK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/shapes.ts:67 SHAPES, src/scene/shapes.ts:67)
#[derive(Clone, Copy, Debug)]
pub struct ShapesK {
    pub on: bool,
}
impl ShapesK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShapesK {
            on: false,
        }
    }
}
impl Default for ShapesK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/combat.ts:677 FLASH, src/game/combat.ts:677)
#[derive(Clone, Copy, Debug)]
pub struct FlashK {
    pub on: bool,
    pub secs: [f64; 2],
    pub stagger: [f64; 2],
    pub lift: f64,
    pub concuss_m: f64,
    pub concuss_k: f64,
    pub own_m: f64,
    pub own_secs: f64,
    pub fuse: f64,
}
impl FlashK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlashK {
            on: false,
            secs: [0.0; 2],
            stagger: [0.0; 2],
            lift: 0.0,
            concuss_m: 0.0,
            concuss_k: 0.0,
            own_m: 0.0,
            own_secs: 0.0,
            fuse: 0.0,
        }
    }
}
impl Default for FlashK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1514 PACE, src/game/guards.ts:1514)
#[derive(Clone, Copy, Debug)]
pub struct PaceK {
    pub on: bool,
    pub max: f64,
    pub accel: f64,
    pub turn: f64,
    pub swing: f64,
}
impl PaceK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaceK {
            on: false,
            max: 0.0,
            accel: 0.0,
            turn: 0.0,
            swing: 0.0,
        }
    }
}
impl Default for PaceK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/guards.ts:1526 LIGHTS, src/game/guards.ts:1526)
#[derive(Clone, Copy, Debug)]
pub struct LightsK {
    pub on: bool,
    pub after_secs: f64,
    pub hot_secs: f64,
    pub step_secs: f64,
    pub tail: f64,
    pub on_m: f64,
    pub alone_m: f64,
    pub look_dark: bool,
    pub look_lit: f64,
    pub look_below: f64,
    pub look_m: f64,
    pub lit_off: bool,
    pub lit_at: f64,
    pub lit_m: f64,
    pub lit_mark_m: f64,
    pub lit_man: bool,
    pub wall_off: bool,
    pub wall_m: f64,
    pub wall_back_m: f64,
    pub wall_secs: f64,
}
impl LightsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LightsK {
            on: false,
            after_secs: 0.0,
            hot_secs: 0.0,
            step_secs: 0.0,
            tail: 0.0,
            on_m: 0.0,
            alone_m: 0.0,
            look_dark: false,
            look_lit: 0.0,
            look_below: 0.0,
            look_m: 0.0,
            lit_off: false,
            lit_at: 0.0,
            lit_m: 0.0,
            lit_mark_m: 0.0,
            lit_man: false,
            wall_off: false,
            wall_m: 0.0,
            wall_back_m: 0.0,
            wall_secs: 0.0,
        }
    }
}
impl Default for LightsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/nvg.ts:41 NVG, src/game/nvg.ts:41)
#[derive(Clone, Copy, Debug)]
pub struct NvgK {
    pub on: bool,
    pub lead: bool,
    pub own: bool,
    pub hunt_frees: bool,
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
impl NvgK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NvgK {
            on: false,
            lead: false,
            own: false,
            hunt_frees: false,
            quiet_secs: 0.0,
            lead_tail: 0.0,
            dark_at: 0.0,
            black_at: 0.0,
            black_reads: 0.0,
            clear_reads: 0.0,
            albedo: 0.0,
            noise: 0.0,
            down_secs: 0.0,
            up_secs: 0.0,
            call_gap: 0.0,
        }
    }
}
impl Default for NvgK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/irlaser.ts:49 IR, src/game/irlaser.ts:49)
#[derive(Clone, Copy, Debug)]
pub struct IrK {
    pub on: bool,
    pub sam: bool,
    pub carry: bool,
    pub team_off: bool,
}
impl IrK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IrK {
            on: false,
            sam: false,
            carry: false,
            team_off: false,
        }
    }
}
impl Default for IrK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/irlight.ts:63 IRLIGHT, src/game/irlight.ts:63)
#[derive(Clone, Copy, Debug)]
pub struct IrlightK {
    pub on: bool,
    pub team: bool,
    pub auto: bool,
    pub meter: bool,
    pub show: f64,
    pub hold: f64,
    pub power: f64,
    pub reach: f64,
    pub cone: f64,
    pub core: f64,
    pub dip: f64,
    pub near: bool,
    pub near_m: f64,
    pub near_min: f64,
    pub near_up: f64,
    pub near_down: f64,
}
impl IrlightK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IrlightK {
            on: false,
            team: false,
            auto: false,
            meter: false,
            show: 0.0,
            hold: 0.0,
            power: 0.0,
            reach: 0.0,
            cone: 0.0,
            core: 0.0,
            dip: 0.0,
            near: false,
            near_m: 0.0,
            near_min: 0.0,
            near_up: 0.0,
            near_down: 0.0,
        }
    }
}
impl Default for IrlightK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallpen.ts:61 WALLPEN, src/game/wallpen.ts:61)
#[derive(Clone, Debug)]
pub struct WallpenK {
    pub on: bool,
    pub base: JsMap<crate::wallpen::ThinStuff, f64>,
    pub per_m: JsMap<crate::wallpen::ThinStuff, f64>,
    pub thin_max: JsMap<crate::wallpen::ThinStuff, f64>,
    pub max_thru: JsMap<crate::wallpen::ThinStuff, f64>,
    pub min_energy: f64,
    pub max_things: f64,
    pub deflect_deg: f64,
    pub men: bool,
    pub behind_m: f64,
    pub thru_rounds: f64,
    pub thru_rest_secs: f64,
}
impl WallpenK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallpenK {
            on: false,
            base: JsMap::new(),
            per_m: JsMap::new(),
            thin_max: JsMap::new(),
            max_thru: JsMap::new(),
            min_energy: 0.0,
            max_things: 0.0,
            deflect_deg: 0.0,
            men: false,
            behind_m: 0.0,
            thru_rounds: 0.0,
            thru_rest_secs: 0.0,
        }
    }
}
impl Default for WallpenK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/combat.ts:578 LASER, src/game/combat.ts:578)
#[derive(Clone, Copy, Debug)]
pub struct LaserK {
    pub enemy: bool,
}
impl LaserK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaserK {
            enemy: false,
        }
    }
}
impl Default for LaserK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:115 DOORTOOLS, src/game/doortools.ts:115)
#[derive(Clone, Debug)]
pub struct DoortoolsK {
    pub charges: f64,
    pub bangs: f64,
    pub blow_key: String,
    pub word_on_key: bool,
    pub word_tap: f64,
    pub thr_hold_first: bool,
    pub thr_bang_first: bool,
    pub thr_outside_row: bool,
    pub thr_leave_fall_in: bool,
    pub thr_hands_off: bool,
    pub thr_held_all: bool,
    pub thr_menu_word: bool,
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
    pub second_throws: bool,
    pub throw_reach: f64,
    pub toss_turn: f64,
    pub frag_way: bool,
    pub frag_way_wait: f64,
    pub frag_way_open: f64,
    pub frag_way_ajar: f64,
    pub bang_way: bool,
    pub open_hand: bool,
    pub gate_hand: bool,
    pub last_out: bool,
    pub last_out_m: f64,
    pub after_bang: f64,
    pub quiet: f64,
    pub entry_cap: f64,
    pub second: f64,
    pub sweep_rad: f64,
    pub sweep_secs: f64,
    pub corners: bool,
    pub corner_secs: f64,
    pub corner_rad: f64,
    pub corner_cap: f64,
    pub corner_owed: bool,
    pub collapse: bool,
    pub corner_turn: bool,
    pub lead_seen: bool,
    pub lead_seen_min: f64,
    pub lead_seen_secs: f64,
    pub lead_seen_m: f64,
    pub sectors: bool,
    pub sector_secs: f64,
    pub search: bool,
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
    pub thr_pull: bool,
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
    pub thr_cover: bool,
    pub thr_slice: bool,
    pub thr_light: f64,
    pub thr_in: f64,
    pub thr_pie: bool,
    pub thr_pie_arc: Vec<Vec<f64>>,
    pub thr_pie_apart: f64,
    pub thr_lean: bool,
    pub thr_lean_m: f64,
    pub thr_knee_in: f64,
    pub thr_pie_one: bool,
    pub thr_lean_look: f64,
    pub thr_doors: bool,
    pub thr_doors_m: f64,
    pub thr_doors_lit: bool,
    pub thr_doors_walk: f64,
    pub thr_doors_stay: f64,
    pub other_door_m: f64,
    pub own_can: bool,
    pub buddy: bool,
    pub buddy_m: f64,
    pub buddy_pair_m: f64,
    pub buddy_left_m: f64,
    pub flow_on: bool,
    pub flow_cells: f64,
    pub flow_moves: f64,
    pub flow_reach: f64,
    pub flow_deep: f64,
    pub flow_bound: f64,
    pub flow_secs: f64,
    pub flow_fight: f64,
    pub open_air: bool,
    pub flow_max: f64,
    pub test_door: bool,
    pub outside_plain: bool,
    pub word_no_can: bool,
    pub word_cuts: bool,
    pub word_cuts_cap: f64,
    pub word_small: bool,
    pub bang_into: bool,
    pub bang_into_min: f64,
    pub bang_into_lit: f64,
    pub bang_into_deep: f64,
    pub bang_into_wide: f64,
    pub bang_hand_m: f64,
    pub bang_pass_v: f64,
    pub halt_back: bool,
    pub behind_sam: bool,
    pub toss_mate: bool,
    pub charge_answer: bool,
    pub pod_frames: bool,
    pub pod_frame_m: f64,
    pub thr_hand: bool,
    pub thr_hand_wait: f64,
    pub thr_hand_every: f64,
    pub thr_no_can: bool,
    pub thr_frag_hold: bool,
    pub thr_doors_leaf: bool,
    pub frag_hold: bool,
    pub frag_hold_cap: f64,
    pub threat_first: bool,
    pub threat_secs: f64,
    pub threat_cap: f64,
    pub threat_near_m: f64,
    pub threat_stuck: f64,
    pub buddy_doors: bool,
    pub thr_set: bool,
    pub thr_set_secs: f64,
    pub thr_tube: bool,
    pub thr_dark: bool,
    pub thr_first: f64,
    pub thr_first_min: f64,
    pub thr_first_stale: f64,
    pub thr_first_in: f64,
    pub thr_first_only: bool,
    pub thr_last: bool,
    pub thr_last_m: f64,
    pub thr_last_fresh: f64,
}
impl DoortoolsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoortoolsK {
            charges: 0.0,
            bangs: 0.0,
            blow_key: String::new(),
            word_on_key: false,
            word_tap: 0.0,
            thr_hold_first: false,
            thr_bang_first: false,
            thr_outside_row: false,
            thr_leave_fall_in: false,
            thr_hands_off: false,
            thr_held_all: false,
            thr_menu_word: false,
            stack_cap: 0.0,
            still_secs: 0.0,
            kick_cap: 0.0,
            in_cap: 0.0,
            shove: 0.0,
            hold_open: 0.0,
            toss_after_open: 0.0,
            bang_in: 0.0,
            fuse_after_land: 0.0,
            air_in: 0.0,
            air_h: 0.0,
            toss_cap: 0.0,
            toss_after_kick: 0.0,
            second_throws: false,
            throw_reach: 0.0,
            toss_turn: 0.0,
            frag_way: false,
            frag_way_wait: 0.0,
            frag_way_open: 0.0,
            frag_way_ajar: 0.0,
            bang_way: false,
            open_hand: false,
            gate_hand: false,
            last_out: false,
            last_out_m: 0.0,
            after_bang: 0.0,
            quiet: 0.0,
            entry_cap: 0.0,
            second: 0.0,
            sweep_rad: 0.0,
            sweep_secs: 0.0,
            corners: false,
            corner_secs: 0.0,
            corner_rad: 0.0,
            corner_cap: 0.0,
            corner_owed: false,
            collapse: false,
            corner_turn: false,
            lead_seen: false,
            lead_seen_min: 0.0,
            lead_seen_secs: 0.0,
            lead_seen_m: 0.0,
            sectors: false,
            sector_secs: 0.0,
            search: false,
            scan_rate: 0.0,
            look_rad: 0.0,
            look_m: 0.0,
            dead_cells: 0.0,
            search_moves: 0.0,
            search_cap: 0.0,
            search_cells: 0.0,
            tuck: 0.0,
            edge_out: 0.0,
            bang_file: 0.0,
            set_secs: 0.0,
            standoff: Vec::new(),
            knock_m: 0.0,
            stun_m: 0.0,
            stun_secs: Vec::new(),
            stagger_secs: 0.0,
            backblast_m: 0.0,
            backblast_secs: 0.0,
            blow_vel: 0.0,
            slice_back: 0.0,
            slice_look: 0.0,
            slice_edge: 0.0,
            slice_deep: 0.0,
            slice_pace: 0.0,
            word_stack: 0.0,
            word_in: 0.0,
            room_back: Vec::new(),
            pod_apart: 0.0,
            thr_pull: false,
            thr_pull_at: Vec::new(),
            thr_still: Vec::new(),
            thr_arc: Vec::new(),
            thr_min: 0.0,
            thr_quiet: 0.0,
            thr_cap: 0.0,
            thr_bangs: 0.0,
            thr_pocket: 0.0,
            thr_shot_m: 0.0,
            thr_throw_m: 0.0,
            thr_flash_m: 0.0,
            thr_turn: 0.0,
            thr_next: 0.0,
            thr_cover: false,
            thr_slice: false,
            thr_light: 0.0,
            thr_in: 0.0,
            thr_pie: false,
            thr_pie_arc: Vec::new(),
            thr_pie_apart: 0.0,
            thr_lean: false,
            thr_lean_m: 0.0,
            thr_knee_in: 0.0,
            thr_pie_one: false,
            thr_lean_look: 0.0,
            thr_doors: false,
            thr_doors_m: 0.0,
            thr_doors_lit: false,
            thr_doors_walk: 0.0,
            thr_doors_stay: 0.0,
            other_door_m: 0.0,
            own_can: false,
            buddy: false,
            buddy_m: 0.0,
            buddy_pair_m: 0.0,
            buddy_left_m: 0.0,
            flow_on: false,
            flow_cells: 0.0,
            flow_moves: 0.0,
            flow_reach: 0.0,
            flow_deep: 0.0,
            flow_bound: 0.0,
            flow_secs: 0.0,
            flow_fight: 0.0,
            open_air: false,
            flow_max: 0.0,
            test_door: false,
            outside_plain: false,
            word_no_can: false,
            word_cuts: false,
            word_cuts_cap: 0.0,
            word_small: false,
            bang_into: false,
            bang_into_min: 0.0,
            bang_into_lit: 0.0,
            bang_into_deep: 0.0,
            bang_into_wide: 0.0,
            bang_hand_m: 0.0,
            bang_pass_v: 0.0,
            halt_back: false,
            behind_sam: false,
            toss_mate: false,
            charge_answer: false,
            pod_frames: false,
            pod_frame_m: 0.0,
            thr_hand: false,
            thr_hand_wait: 0.0,
            thr_hand_every: 0.0,
            thr_no_can: false,
            thr_frag_hold: false,
            thr_doors_leaf: false,
            frag_hold: false,
            frag_hold_cap: 0.0,
            threat_first: false,
            threat_secs: 0.0,
            threat_cap: 0.0,
            threat_near_m: 0.0,
            threat_stuck: 0.0,
            buddy_doors: false,
            thr_set: false,
            thr_set_secs: 0.0,
            thr_tube: false,
            thr_dark: false,
            thr_first: 0.0,
            thr_first_min: 0.0,
            thr_first_stale: 0.0,
            thr_first_in: 0.0,
            thr_first_only: false,
            thr_last: false,
            thr_last_m: 0.0,
            thr_last_fresh: 0.0,
        }
    }
}
impl Default for DoortoolsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:91 ROOMFIGHT, src/game/roomfight.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct RoomfightK {
    pub on: bool,
    pub evidence_secs: f64,
    pub sticky: f64,
    pub crowd: f64,
    pub fear_min: f64,
    pub fall_back: bool,
    pub rally: bool,
    pub fall_back_in: bool,
    pub doorway: bool,
    pub side_step: bool,
    pub turn_walk: bool,
    pub nearest: bool,
    pub close_m: f64,
    pub no_shot: f64,
    pub close_lane: bool,
    pub point_blank: f64,
    pub sam_behind: bool,
    pub sam_beside: bool,
    pub sam_travel: bool,
    pub no_shot_go: f64,
    pub no_shot_pass: f64,
    pub keep_track: bool,
    pub rally_k: f64,
    pub step_ears: bool,
    pub arc_half_deg: f64,
    pub cover_keep: bool,
    pub cover_half_deg: f64,
    pub cover_near_m: f64,
    pub team_hot: f64,
    pub cued: bool,
    pub let_go: f64,
    pub step_look: bool,
    pub step_look_secs: f64,
    pub door_ears: bool,
    pub door_look_secs: f64,
    pub door_shots: bool,
    pub door_shot_m: f64,
    pub sam_line: bool,
    pub hold_short_secs: f64,
    pub pairs: f64,
    pub move_engage_m: f64,
    pub lamp: f64,
    pub preaim_deg: f64,
    pub sights: bool,
    pub sights_foe: bool,
    pub sights_deg: f64,
}
impl RoomfightK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomfightK {
            on: false,
            evidence_secs: 0.0,
            sticky: 0.0,
            crowd: 0.0,
            fear_min: 0.0,
            fall_back: false,
            rally: false,
            fall_back_in: false,
            doorway: false,
            side_step: false,
            turn_walk: false,
            nearest: false,
            close_m: 0.0,
            no_shot: 0.0,
            close_lane: false,
            point_blank: 0.0,
            sam_behind: false,
            sam_beside: false,
            sam_travel: false,
            no_shot_go: 0.0,
            no_shot_pass: 0.0,
            keep_track: false,
            rally_k: 0.0,
            step_ears: false,
            arc_half_deg: 0.0,
            cover_keep: false,
            cover_half_deg: 0.0,
            cover_near_m: 0.0,
            team_hot: 0.0,
            cued: false,
            let_go: 0.0,
            step_look: false,
            step_look_secs: 0.0,
            door_ears: false,
            door_look_secs: 0.0,
            door_shots: false,
            door_shot_m: 0.0,
            sam_line: false,
            hold_short_secs: 0.0,
            pairs: 0.0,
            move_engage_m: 0.0,
            lamp: 0.0,
            preaim_deg: 0.0,
            sights: false,
            sights_foe: false,
            sights_deg: 0.0,
        }
    }
}
impl Default for RoomfightK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:126 ORDERS, src/game/orders.ts:126)
#[derive(Clone, Debug)]
pub struct OrdersK {
    pub key: String,
    pub mouse_opens: bool,
    pub click_gives: bool,
    pub who_key: String,
    pub quick_key: String,
    pub wheel_element: bool,
    pub wait_key: String,
    pub wait_alt: String,
    pub refresh_secs: f64,
    pub door_pick_m: f64,
    pub self_m: f64,
    pub man_m: f64,
    pub door_ray: bool,
    pub door_ray_pad: f64,
    pub door_ray_reach_m: f64,
    pub self_body: bool,
    pub floor_snap_m: f64,
    pub jamb_out: f64,
    pub jamb_past: f64,
    pub file_gap: f64,
    pub seam_m: f64,
    pub second_floor: bool,
    pub work_out: bool,
    pub work_seen_m: f64,
    pub hall_file: bool,
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
    pub file_hold: bool,
    pub file_spread: bool,
    pub file_apart: f64,
    pub file_tight: f64,
    pub off_door: f64,
    pub back_end: bool,
    pub in_way: bool,
    pub retake_goal: bool,
    pub walk_tries: bool,
    pub aside_m: f64,
    pub aside_lane: f64,
    pub aside_off: f64,
    pub aside_clear: bool,
    pub file_queue: bool,
    pub file_queue_gap: f64,
    pub file_queue_cost: f64,
    pub prop_round: bool,
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
    pub pin_posted: bool,
    pub lane_in: f64,
    pub lane_out: f64,
    pub hold_stride: f64,
    pub stuck_secs: f64,
    pub hold_group_m: f64,
    pub hold_door_m: f64,
    pub cleared: bool,
    pub cleared_dear: f64,
    pub known_face: bool,
    pub own_door: bool,
    pub round_cap: f64,
    pub round_own: bool,
    pub same_way: bool,
    pub same_near_m: f64,
    pub same_apart_m: f64,
    pub same_long: f64,
    pub trail_doors: bool,
    pub trail_reach_m: f64,
    pub trail_long: f64,
}
impl OrdersK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrdersK {
            key: String::new(),
            mouse_opens: false,
            click_gives: false,
            who_key: String::new(),
            quick_key: String::new(),
            wheel_element: false,
            wait_key: String::new(),
            wait_alt: String::new(),
            refresh_secs: 0.0,
            door_pick_m: 0.0,
            self_m: 0.0,
            man_m: 0.0,
            door_ray: false,
            door_ray_pad: 0.0,
            door_ray_reach_m: 0.0,
            self_body: false,
            floor_snap_m: 0.0,
            jamb_out: 0.0,
            jamb_past: 0.0,
            file_gap: 0.0,
            seam_m: 0.0,
            second_floor: false,
            work_out: false,
            work_seen_m: 0.0,
            hall_file: false,
            follow_back: Vec::new(),
            crumb: 0.0,
            trail_m: 0.0,
            run: 0.0,
            walk: 0.0,
            gain: 0.0,
            near_m: 0.0,
            band_m: 0.0,
            word_far_m: 0.0,
            word_hold_s: 0.0,
            file_hold: false,
            file_spread: false,
            file_apart: 0.0,
            file_tight: 0.0,
            off_door: 0.0,
            back_end: false,
            in_way: false,
            retake_goal: false,
            walk_tries: false,
            aside_m: 0.0,
            aside_lane: 0.0,
            aside_off: 0.0,
            aside_clear: false,
            file_queue: false,
            file_queue_gap: 0.0,
            file_queue_cost: 0.0,
            prop_round: false,
            prop_round_pad: 0.0,
            prop_round_cost: 0.0,
            prop_round_more: 0.0,
            prop_round_door: 0.0,
            aside_hold: 0.0,
            brisk: 0.0,
            brisk_in: 0.0,
            onto: 0.0,
            entry: 0.0,
            move_spread: 0.0,
            cold_secs: 0.0,
            pin_posted: false,
            lane_in: 0.0,
            lane_out: 0.0,
            hold_stride: 0.0,
            stuck_secs: 0.0,
            hold_group_m: 0.0,
            hold_door_m: 0.0,
            cleared: false,
            cleared_dear: 0.0,
            known_face: false,
            own_door: false,
            round_cap: 0.0,
            round_own: false,
            same_way: false,
            same_near_m: 0.0,
            same_apart_m: 0.0,
            same_long: 0.0,
            trail_doors: false,
            trail_reach_m: 0.0,
            trail_long: 0.0,
        }
    }
}
impl Default for OrdersK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cover.ts:62 COVER, src/game/cover.ts:62)
#[derive(Clone, Copy, Debug)]
pub struct CoverK {
    pub on: bool,
    pub stacked: bool,
    pub in_room: bool,
    pub rear_read: bool,
    pub rear_read_in: f64,
    pub rear_read_m: f64,
    pub rear_read_deg: f64,
    pub rear_read_secs: f64,
    pub rear_read_sticky: f64,
    pub rear_look: f64,
    pub rear_walk_m: f64,
    pub leaf: bool,
    pub edge: bool,
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
    pub lane_walk: bool,
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
    pub back_keep: bool,
    pub back_body: f64,
    pub back_turn: bool,
    pub back_turn_body: f64,
    pub back_turn_secs: f64,
    pub swing: f64,
    pub step: bool,
    pub step_m: f64,
    pub step_after: f64,
    pub step_off_m: f64,
    pub step_reach: f64,
    pub step_secs: f64,
    pub step_place_m: f64,
}
impl CoverK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverK {
            on: false,
            stacked: false,
            in_room: false,
            rear_read: false,
            rear_read_in: 0.0,
            rear_read_m: 0.0,
            rear_read_deg: 0.0,
            rear_read_secs: 0.0,
            rear_read_sticky: 0.0,
            rear_look: 0.0,
            rear_walk_m: 0.0,
            leaf: false,
            edge: false,
            reach: 0.0,
            deal_secs: 0.0,
            gap_cells: 0.0,
            through_rows: 0.0,
            alcove_rows: 0.0,
            bridge: 0.0,
            in_m: 0.0,
            lane: 0.0,
            lane_keep: 0.0,
            lane_secs: 0.0,
            lane_walk: false,
            sam_deg: 0.0,
            sam_k: 0.0,
            held_k: 0.0,
            shut_k: 0.0,
            rear_k: 0.0,
            near_m: 0.0,
            contact_secs: 0.0,
            rear_m: 0.0,
            rear_from: 0.0,
            rear_deg: 0.0,
            sticky: 0.0,
            keep_secs: 0.0,
            rear_keep: 0.0,
            behind_k: 0.0,
            twist: 0.0,
            back_in: 0.0,
            back_out: 0.0,
            back_keep: false,
            back_body: 0.0,
            back_turn: false,
            back_turn_body: 0.0,
            back_turn_secs: 0.0,
            swing: 0.0,
            step: false,
            step_m: 0.0,
            step_after: 0.0,
            step_off_m: 0.0,
            step_reach: 0.0,
            step_secs: 0.0,
            step_place_m: 0.0,
        }
    }
}
impl Default for CoverK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfit.ts:38 ROOMFIT, src/game/roomfit.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct RoomfitK {
    pub on: bool,
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
impl RoomfitK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomfitK {
            on: false,
            spread0: 0.0,
            spread_per: 0.0,
            apart: 0.0,
            loose: 0.0,
            loose_k: 0.0,
            near_k: 0.0,
            near_free: 0.0,
            wall_k: 0.0,
            corner_k: 0.0,
            funnel_m: 0.0,
            funnel_pad: 0.0,
            funnel_flare: 0.0,
            funnel_k: 0.0,
            own_k: 0.0,
            known_funnel_k: 0.0,
            shut_funnel_k: 0.0,
            ang_k: 0.0,
            ang_lo: 0.0,
            ang_hi: 0.0,
            ang_max: 0.0,
            close_m: 0.0,
            close_k: 0.0,
            lane_m: 0.0,
            lane_k: 0.0,
            reach: 0.0,
            near_m: 0.0,
            known_k: 0.0,
            shut_k: 0.0,
            locked_k: 0.0,
            front_k: 0.0,
            second_k: 0.0,
            sector_m: 0.0,
            sector_k: 0.0,
            sector_deg: 0.0,
            sector_across: 0.0,
            sector_look: 0.0,
            sector_far_m: 0.0,
            sector_far_k: 0.0,
            watch_in_m: 0.0,
            again_secs: 0.0,
            sticky: 0.0,
        }
    }
}
impl Default for RoomfitK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/entrypoints.ts:36 ENTRY, src/game/entrypoints.ts:36)
#[derive(Clone, Copy, Debug)]
pub struct EntryK {
    pub off_openings: bool,
    pub move_: bool,
    pub watch: bool,
    pub near_m: f64,
    pub line_deg: f64,
    pub gaps: bool,
    pub known_m: f64,
    pub move_m: f64,
    pub apart: f64,
    pub off_mouth: bool,
    pub mouth_m: f64,
}
impl EntryK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EntryK {
            off_openings: false,
            move_: false,
            watch: false,
            near_m: 0.0,
            line_deg: 0.0,
            gaps: false,
            known_m: 0.0,
            move_m: 0.0,
            apart: 0.0,
            off_mouth: false,
            mouth_m: 0.0,
        }
    }
}
impl Default for EntryK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cover.ts:171 REAR, src/game/cover.ts:171)
#[derive(Clone, Copy, Debug)]
pub struct RearK {
    pub settle: bool,
    pub secs: f64,
    pub file: bool,
}
impl RearK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RearK {
            settle: false,
            secs: 0.0,
            file: false,
        }
    }
}
impl Default for RearK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/silent.ts:67 SILENT, src/game/silent.ts:67)
#[derive(Clone, Copy, Debug)]
pub struct SilentK {
    pub on: bool,
    pub start: bool,
    pub pace: f64,
    pub hot_pace: bool,
    pub step: f64,
    pub lights: bool,
    pub hush: bool,
    pub doors: bool,
    pub kicks: bool,
    pub picks: bool,
    pub boot_wedged: bool,
    pub cans: bool,
    pub hold_fire: bool,
    pub noticed: f64,
    pub glass: bool,
    pub tools_loud: bool,
    pub stack_set: bool,
}
impl SilentK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SilentK {
            on: false,
            start: false,
            pace: 0.0,
            hot_pace: false,
            step: 0.0,
            lights: false,
            hush: false,
            doors: false,
            kicks: false,
            picks: false,
            boot_wedged: false,
            cans: false,
            hold_fire: false,
            noticed: 0.0,
            glass: false,
            tools_loud: false,
            stack_set: false,
        }
    }
}
impl Default for SilentK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pairorders.ts:44 PAIR, src/game/pairorders.ts:44)
#[derive(Clone, Copy, Debug)]
pub struct PairK {
    pub on: bool,
    pub max_squad: f64,
    pub pick_m: f64,
    pub stage_m: f64,
    pub reach: f64,
    pub secs: f64,
    pub scuffle: f64,
    pub grip: bool,
    pub grip_body: bool,
    pub pace: f64,
    pub go_pace: f64,
    pub crouch: bool,
    pub stalk_every: f64,
    pub stalk_wait: f64,
    pub sweep: bool,
    pub ring_m: f64,
    pub keep_way: bool,
    pub habit_stood: bool,
    pub steady: bool,
    pub steady_face_secs: f64,
    pub front_m: f64,
    pub cone_m: f64,
    pub cone_lit: f64,
    pub watched: bool,
    pub watch_secs: f64,
    pub watch_aw: f64,
    pub look_back: bool,
    pub heel_ring: f64,
    pub heel_fresh: f64,
    pub heel_wait: f64,
    pub take_secs: f64,
    pub mark_on_hands: bool,
    pub quiet_first: bool,
    pub dark_vis: f64,
    pub dark_m: f64,
    pub dark_refuse: bool,
    pub dark_crouch: bool,
    pub dark_h: f64,
    pub lit_secs: f64,
    pub lit_every: f64,
}
impl PairK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairK {
            on: false,
            max_squad: 0.0,
            pick_m: 0.0,
            stage_m: 0.0,
            reach: 0.0,
            secs: 0.0,
            scuffle: 0.0,
            grip: false,
            grip_body: false,
            pace: 0.0,
            go_pace: 0.0,
            crouch: false,
            stalk_every: 0.0,
            stalk_wait: 0.0,
            sweep: false,
            ring_m: 0.0,
            keep_way: false,
            habit_stood: false,
            steady: false,
            steady_face_secs: 0.0,
            front_m: 0.0,
            cone_m: 0.0,
            cone_lit: 0.0,
            watched: false,
            watch_secs: 0.0,
            watch_aw: 0.0,
            look_back: false,
            heel_ring: 0.0,
            heel_fresh: 0.0,
            heel_wait: 0.0,
            take_secs: 0.0,
            mark_on_hands: false,
            quiet_first: false,
            dark_vis: 0.0,
            dark_m: 0.0,
            dark_refuse: false,
            dark_crouch: false,
            dark_h: 0.0,
            lit_secs: 0.0,
            lit_every: 0.0,
        }
    }
}
impl Default for PairK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescue.ts:81 RESCUE, src/game/rescue.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct RescueK {
    pub start: crate::knobs::RescueKStart,
    pub clock: bool,
    pub resolve_secs: f64,
    pub resolve_spread: f64,
    pub second_hostage_secs: f64,
    pub aim_secs: f64,
    pub set_secs: f64,
    pub unset_react: f64,
    pub captor_set: bool,
    pub hold_secs: f64,
    pub door_man: crate::knobs::RescueKDoorMan,
    pub roamers: f64,
    pub seed_roamers: bool,
    pub stun: bool,
    pub op_lights: bool,
    pub trained: bool,
    pub blind_fire: bool,
    pub def_lights: bool,
    pub room_lights: bool,
    pub flow: bool,
    pub flow_solo: bool,
    pub marks: bool,
    pub hold_hot: bool,
    pub two_door: bool,
    pub side_pair: bool,
    pub crew_door: bool,
    pub crew_cross: bool,
    pub crew_behind: bool,
    pub crew_call_secs: f64,
    pub room_search: bool,
    pub clock_as_seconds: bool,
    pub squad: f64,
    pub sam_at: V3,
    pub sam_yaw: f64,
    pub ops_at: [V3; 2],
}
impl RescueK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueK {
            start: crate::knobs::RescueKStart::Quiet,
            clock: false,
            resolve_secs: 0.0,
            resolve_spread: 0.0,
            second_hostage_secs: 0.0,
            aim_secs: 0.0,
            set_secs: 0.0,
            unset_react: 0.0,
            captor_set: false,
            hold_secs: 0.0,
            door_man: crate::knobs::RescueKDoorMan::Inside,
            roamers: 0.0,
            seed_roamers: false,
            stun: false,
            op_lights: false,
            trained: false,
            blind_fire: false,
            def_lights: false,
            room_lights: false,
            flow: false,
            flow_solo: false,
            marks: false,
            hold_hot: false,
            two_door: false,
            side_pair: false,
            crew_door: false,
            crew_cross: false,
            crew_behind: false,
            crew_call_secs: 0.0,
            room_search: false,
            clock_as_seconds: false,
            squad: 0.0,
            sam_at: [0.0; 3],
            sam_yaw: 0.0,
            ops_at: [[0.0; 3]; 2],
        }
    }
}
impl Default for RescueK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueKStart {
    Quiet,
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueKDoorMan {
    Inside,
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

/// (TypeScript: inline at src/game/digin.ts:54 DIGIN, src/game/digin.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct DiginK {
    pub on: bool,
    pub stay: bool,
    pub reach_m: f64,
    pub hold_set: bool,
    pub keeper: bool,
    pub refuge: bool,
    pub cover: bool,
    pub brace: bool,
    pub brace_in_room: bool,
    pub doors: bool,
    pub radio: bool,
    pub noticed: bool,
    pub noticed_secs: f64,
    pub shut_open: bool,
    pub shut_secs: f64,
    pub shut_home_rad: f64,
    pub shut_doors: f64,
    pub deep: bool,
    pub far: bool,
    pub learn: bool,
    pub gather: bool,
    pub gather_m: f64,
    pub gather_shy_m: f64,
    pub steady: bool,
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
    pub lanes: bool,
    pub lane_secs: f64,
}
impl DiginK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DiginK {
            on: false,
            stay: false,
            reach_m: 0.0,
            hold_set: false,
            keeper: false,
            refuge: false,
            cover: false,
            brace: false,
            brace_in_room: false,
            doors: false,
            radio: false,
            noticed: false,
            noticed_secs: 0.0,
            shut_open: false,
            shut_secs: 0.0,
            shut_home_rad: 0.0,
            shut_doors: 0.0,
            deep: false,
            far: false,
            learn: false,
            gather: false,
            gather_m: 0.0,
            gather_shy_m: 0.0,
            steady: false,
            think: [0.0; 2],
            wedge_secs: [0.0; 2],
            wedges: 0.0,
            caught: [0.0; 2],
            try_secs: 0.0,
            pace: 0.0,
            close_m: 0.0,
            prop_m: 0.0,
            haul_speed: 0.0,
            scrape_level: 0.0,
            scrape_every: 0.0,
            shout_m: 0.0,
            alt_m: 0.0,
            better: 0.0,
            fear: 0.0,
            hold_secs: 0.0,
            redig_secs: 0.0,
            lanes: false,
            lane_secs: 0.0,
        }
    }
}
impl Default for DiginK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:171 HAULHOME, src/game/digin.ts:171)
#[derive(Clone, Copy, Debug)]
pub struct HaulhomeK {
    pub on: bool,
}
impl HaulhomeK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HaulhomeK {
            on: false,
        }
    }
}
impl Default for HaulhomeK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:65 PRESSURE, src/game/pressure.ts:65)
#[derive(Clone, Copy, Debug)]
pub struct PressureK {
    pub on: bool,
    pub press: bool,
    pub flank: bool,
    pub fall_back: bool,
    pub squad_drill: bool,
    pub second: bool,
    pub second_keeps: bool,
    pub second_set: bool,
    pub max_men: f64,
    pub keep_room: bool,
    pub memory_secs: f64,
    pub organise_k: f64,
    pub warned: bool,
    pub stand_press: bool,
    pub keeper_left: bool,
    pub report_secs: f64,
    pub stand_secs: f64,
    pub stand_grace: f64,
    pub again_secs: f64,
    pub stuns: f64,
    pub stun_known: bool,
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
    pub out_early: bool,
    pub loud_secs: f64,
    pub out_secs: [f64; 2],
    pub gather: bool,
    pub gather_secs: f64,
    pub flank_secs: f64,
    pub siege_secs: f64,
    pub listen: bool,
    pub converge: bool,
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
impl PressureK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PressureK {
            on: false,
            press: false,
            flank: false,
            fall_back: false,
            squad_drill: false,
            second: false,
            second_keeps: false,
            second_set: false,
            max_men: 0.0,
            keep_room: false,
            memory_secs: 0.0,
            organise_k: 0.0,
            warned: false,
            stand_press: false,
            keeper_left: false,
            report_secs: 0.0,
            stand_secs: 0.0,
            stand_grace: 0.0,
            again_secs: 0.0,
            stuns: 0.0,
            stun_known: false,
            stun_know_secs: 0.0,
            stun_moved_secs: 0.0,
            last_holds: false,
            picket: false,
            by_way: false,
            rescue_crew: false,
            drilled: false,
            flank_max_m: 0.0,
            flank_k: 0.0,
            flank_slack_m: 0.0,
            last_man_secs: [0.0; 2],
            out_early: false,
            loud_secs: 0.0,
            out_secs: [0.0; 2],
            gather: false,
            gather_secs: 0.0,
            flank_secs: 0.0,
            siege_secs: 0.0,
            listen: false,
            converge: false,
            word_secs: 0.0,
            way_out: false,
            way_out_m: 0.0,
            way_out_secs: 0.0,
            sweep: false,
            sweep_secs: 0.0,
            sweep_men: 0.0,
            sweep_beat: 0.0,
            sweep_pair_m: 0.0,
        }
    }
}
impl Default for PressureK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lanes.ts:62 LANES, src/game/lanes.ts:62)
#[derive(Clone, Copy, Debug)]
pub struct LanesK {
    pub on: bool,
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
    pub round: bool,
    pub round_men: f64,
    pub think_secs: f64,
    pub round_max_m: f64,
    pub round_gap: f64,
    pub round_side_deg: f64,
    pub round_show_m: f64,
    pub shift_m: f64,
    pub stun: bool,
    pub stun_m: f64,
    pub bang_m: f64,
    pub push_secs: f64,
    pub push_m: f64,
    pub push_fast: f64,
    pub stun_gap: f64,
    pub moved_m: f64,
    pub moved_secs: f64,
    pub peek: bool,
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
    pub pin: bool,
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
    pub pin_go: bool,
    pub glass: bool,
    pub stop_clear: bool,
    pub look_m: f64,
    pub roused: bool,
    pub frag: bool,
    pub frag_think: f64,
    pub frag_gap: f64,
    pub frag_look: f64,
    pub frag_step_m: f64,
    pub frag_own: bool,
    pub drill: bool,
    pub max: f64,
}
impl LanesK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LanesK {
            on: false,
            secs: 0.0,
            reach_m: 0.0,
            half_deg: 0.0,
            width_m: 0.0,
            field_m: 0.0,
            own_m: 0.0,
            min_m: 0.0,
            round_k: 0.0,
            near_m: 0.0,
            react_secs: 0.0,
            call_secs: 0.0,
            see_m: 0.0,
            body_m: 0.0,
            dear: 0.0,
            ahead_m: 0.0,
            close_m: 0.0,
            hot_secs: 0.0,
            round: false,
            round_men: 0.0,
            think_secs: 0.0,
            round_max_m: 0.0,
            round_gap: 0.0,
            round_side_deg: 0.0,
            round_show_m: 0.0,
            shift_m: 0.0,
            stun: false,
            stun_m: 0.0,
            bang_m: 0.0,
            push_secs: 0.0,
            push_m: 0.0,
            push_fast: 0.0,
            stun_gap: 0.0,
            moved_m: 0.0,
            moved_secs: 0.0,
            peek: false,
            peek_quiet: 0.0,
            peek_gap: 0.0,
            peek_max: 0.0,
            peek_lost: 0.0,
            peek_reach_m: 0.0,
            peek_near_m: 0.0,
            peek_look: 0.0,
            peek_sure: 0.0,
            peek_keep: 0.0,
            peek_pace: 0.0,
            peek_lean: 0.0,
            pin: false,
            pin_rounds: 0.0,
            pin_every: 0.0,
            pin_snap: 0.0,
            pin_gap: 0.0,
            pin_max: 0.0,
            pin_rest: 0.0,
            pin_spread: 0.0,
            pin_aim_deg: 0.0,
            pin_pace: 0.0,
            pin_show_m: 0.0,
            pin_min_m: 0.0,
            pin_go: false,
            glass: false,
            stop_clear: false,
            look_m: 0.0,
            roused: false,
            frag: false,
            frag_think: 0.0,
            frag_gap: 0.0,
            frag_look: 0.0,
            frag_step_m: 0.0,
            frag_own: false,
            drill: false,
            max: 0.0,
        }
    }
}
impl Default for LanesK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunhold.ts:46 GUNHOLD, src/game/gunhold.ts:46)
#[derive(Clone, Copy, Debug)]
pub struct GunholdK {
    pub on: bool,
    pub fire: bool,
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
impl GunholdK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunholdK {
            on: false,
            fire: false,
            cover_m: 0.0,
            near_m: 0.0,
            rear_deg: 0.0,
            keep_secs: 0.0,
            peek_every: 0.0,
            peek_secs: 0.0,
            show_m: 0.0,
            shy_s: 0.0,
            dash_m: 0.0,
            face_deg: 0.0,
            flank_max_m: 0.0,
            react: [0.0; 2],
        }
    }
}
impl Default for GunholdK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:43 HARD, src/game/difficulty.ts:43)
#[derive(Clone, Copy, Debug)]
pub struct HardK {
    pub reach_m: f64,
    pub keeper_digs: bool,
    pub warned: bool,
    pub second: bool,
    pub picket: bool,
    pub by_way: bool,
    pub rescue_crew: bool,
    pub way_out: bool,
    pub stuns: f64,
    pub drilled: bool,
    pub cone: bool,
    pub sweep: bool,
}
impl HardK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HardK {
            reach_m: 0.0,
            keeper_digs: false,
            warned: false,
            second: false,
            picket: false,
            by_way: false,
            rescue_crew: false,
            way_out: false,
            stuns: 0.0,
            drilled: false,
            cone: false,
            sweep: false,
        }
    }
}
impl Default for HardK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/difficulty.ts:92 HARD_AT, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1611)
#[derive(Clone, Copy, Debug)]
pub struct HardAtK {
}
impl HardAtK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HardAtK {
        }
    }
}
impl Default for HardAtK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomfight.ts:284 DRILLED, src/game/roomfight.ts:284)
#[derive(Clone, Copy, Debug)]
pub struct DrilledK {
    pub shaken: f64,
}
impl DrilledK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DrilledK {
            shaken: 0.0,
        }
    }
}
impl Default for DrilledK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:75 SQUADS, src/game/enemysquads.ts:75)
#[derive(Clone, Copy, Debug)]
pub struct SquadsK {
    pub on: bool,
    pub think: [f64; 2],
    pub net_lag: f64,
    pub help_men: f64,
    pub flank_men: f64,
    pub again_secs: f64,
    pub settle_secs: f64,
    pub quiet_secs: f64,
    pub recall_secs: f64,
    pub reinforce: [f64; 2],
    pub open_pair: bool,
    pub ready_beat: f64,
    pub go_round: bool,
    pub fire_m: f64,
    pub rear_m: f64,
    pub soft_m: f64,
    pub mouth_m: f64,
    pub hold_lost: bool,
    pub hold_back_m: f64,
    pub hold_secs: f64,
    pub drill_net: bool,
    pub net_k: f64,
    pub reinforce_k: f64,
    pub drill_angles: bool,
    pub drill_react: bool,
    pub react_k: f64,
    pub drill_nerve: bool,
    pub nerve_k: f64,
    pub drill_follow: bool,
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
    pub drill_by_map: bool,
    pub way: bool,
    pub radio_check: bool,
    pub check_every: f64,
    pub check_jitter: f64,
    pub check_alert_k: f64,
    pub check_ask: f64,
    pub check_send: f64,
    pub check_heard: [f64; 2],
    pub check_hear_m: f64,
    pub look_men: f64,
    pub heard_knows: bool,
    pub heard_asks: bool,
    pub noticed_eyes: bool,
    pub call_beat: bool,
    pub call_secs: [f64; 2],
    pub call_body: bool,
    pub call_cut_ask: [f64; 2],
    pub net_ask: bool,
    pub net_every: f64,
    pub net_beat: [f64; 2],
    pub net_look_men: f64,
    pub net_stops: f64,
    pub net_cut_alarm: bool,
}
impl SquadsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadsK {
            on: false,
            think: [0.0; 2],
            net_lag: 0.0,
            help_men: 0.0,
            flank_men: 0.0,
            again_secs: 0.0,
            settle_secs: 0.0,
            quiet_secs: 0.0,
            recall_secs: 0.0,
            reinforce: [0.0; 2],
            open_pair: false,
            ready_beat: 0.0,
            go_round: false,
            fire_m: 0.0,
            rear_m: 0.0,
            soft_m: 0.0,
            mouth_m: 0.0,
            hold_lost: false,
            hold_back_m: 0.0,
            hold_secs: 0.0,
            drill_net: false,
            net_k: 0.0,
            reinforce_k: 0.0,
            drill_angles: false,
            drill_react: false,
            react_k: 0.0,
            drill_nerve: false,
            nerve_k: 0.0,
            drill_follow: false,
            follow_m: 0.0,
            drill_own: false,
            own_up: 0.0,
            drill_back: false,
            back_beat: 0.0,
            drill_look: false,
            sweep_secs: 0.0,
            sweep_men: 0.0,
            sweep_beat: 0.0,
            sweep_stayers: false,
            drill_by_map: false,
            way: false,
            radio_check: false,
            check_every: 0.0,
            check_jitter: 0.0,
            check_alert_k: 0.0,
            check_ask: 0.0,
            check_send: 0.0,
            check_heard: [0.0; 2],
            check_hear_m: 0.0,
            look_men: 0.0,
            heard_knows: false,
            heard_asks: false,
            noticed_eyes: false,
            call_beat: false,
            call_secs: [0.0; 2],
            call_body: false,
            call_cut_ask: [0.0; 2],
            net_ask: false,
            net_every: 0.0,
            net_beat: [0.0; 2],
            net_look_men: 0.0,
            net_stops: 0.0,
            net_cut_alarm: false,
        }
    }
}
impl Default for SquadsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/enemysquads.ts:287 DRILL_AT, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1611)
#[derive(Clone, Copy, Debug)]
pub struct DrillAtK {
}
impl DrillAtK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DrillAtK {
        }
    }
}
impl Default for DrillAtK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cues.ts:56 CUES, src/game/cues.ts:56)
#[derive(Clone, Debug)]
pub struct CuesK {
    pub on: bool,
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
    pub overheard: bool,
    pub overheard_gap: f64,
}
impl CuesK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CuesK {
            on: false,
            rows: 0.0,
            secs: 0.0,
            min_show: 0.0,
            per_class: 0.0,
            dup_secs: 0.0,
            gaps: Vec::new(),
            squelch: 0.0,
            man_secs: 0.0,
            again_secs: 0.0,
            still_secs: 0.0,
            still_quiet: 0.0,
            stay_secs: 0.0,
            overheard: false,
            overheard_gap: 0.0,
        }
    }
}
impl Default for CuesK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/buddy.ts:38 BUDDY, src/game/buddy.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct BuddyK {
    pub on: bool,
    pub staged: bool,
    pub loud_secs: f64,
    pub beat: [f64; 2],
    pub quiet: f64,
    pub calm: f64,
    pub calm_m: f64,
    pub answer: [f64; 2],
    pub max_m: f64,
    pub clear_m: f64,
    pub loners: bool,
    pub from_pair: bool,
    pub empty_room: bool,
    pub again: f64,
    pub calls: f64,
}
impl BuddyK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BuddyK {
            on: false,
            staged: false,
            loud_secs: 0.0,
            beat: [0.0; 2],
            quiet: 0.0,
            calm: 0.0,
            calm_m: 0.0,
            answer: [0.0; 2],
            max_m: 0.0,
            clear_m: 0.0,
            loners: false,
            from_pair: false,
            empty_room: false,
            again: 0.0,
            calls: 0.0,
        }
    }
}
impl Default for BuddyK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weather.ts:56 WX_SIM, src/game/weather.ts:56)
#[derive(Clone, Copy, Debug)]
pub struct WxSimK {
    pub rain_k: f64,
    pub thunder_k: f64,
    pub thunder_secs: f64,
    pub thunder_loud: f64,
    pub roof_bed: f64,
    pub loud_dry: bool,
}
impl WxSimK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WxSimK {
            rain_k: 0.0,
            thunder_k: 0.0,
            thunder_secs: 0.0,
            thunder_loud: 0.0,
            roof_bed: 0.0,
            loud_dry: false,
        }
    }
}
impl Default for WxSimK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weatherKinds.ts:172 WX_OWN, src/game/weatherKinds.ts:172)
#[derive(Clone, Copy, Debug)]
pub struct WxOwnK {
    pub roofs: bool,
    pub wind: bool,
}
impl WxOwnK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WxOwnK {
            roofs: false,
            wind: false,
        }
    }
}
impl Default for WxOwnK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weatherKinds.ts:58 SNOW, src/game/weatherKinds.ts:58)
#[derive(Clone, Copy, Debug)]
pub struct SnowK {
    pub on: bool,
}
impl SnowK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowK {
            on: false,
        }
    }
}
impl Default for SnowK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weatherKinds.ts:105 SNOW_LOOK, src/game/weatherKinds.ts:105)
#[derive(Clone, Copy, Debug)]
pub struct SnowLookK {
    pub on: bool,
    pub wisps: bool,
    pub tracks: bool,
    pub glow: bool,
    pub sparkle: bool,
    pub breath: bool,
    pub steam: bool,
    pub grains: bool,
    pub blast: bool,
    pub one_air: bool,
    pub marks: bool,
    pub past: bool,
    pub under: bool,
}
impl SnowLookK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowLookK {
            on: false,
            wisps: false,
            tracks: false,
            glow: false,
            sparkle: false,
            breath: false,
            steam: false,
            grains: false,
            blast: false,
            one_air: false,
            marks: false,
            past: false,
            under: false,
        }
    }
}
impl Default for SnowLookK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/airSim.ts:33 AIR_SIM, src/game/airSim.ts:33)
#[derive(Clone, Copy, Debug)]
pub struct AirSimK {
    pub on: bool,
    pub ground: f64,
    pub min_top: f64,
    pub max_foot: f64,
    pub roof_share: f64,
    pub glass_open: f64,
    pub leaf_step: f64,
    pub vent_out: f64,
    pub gust_k: f64,
    pub rate: f64,
    pub sigma_min: f64,
    pub sigma_max: f64,
    pub puff_rate: f64,
    pub settle: f64,
}
impl AirSimK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AirSimK {
            on: false,
            ground: 0.0,
            min_top: 0.0,
            max_foot: 0.0,
            roof_share: 0.0,
            glass_open: 0.0,
            leaf_step: 0.0,
            vent_out: 0.0,
            gust_k: 0.0,
            rate: 0.0,
            sigma_min: 0.0,
            sigma_max: 0.0,
            puff_rate: 0.0,
            settle: 0.0,
        }
    }
}
impl Default for AirSimK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/laststand.ts:58 LASTSTAND, src/game/laststand.ts:58)
#[derive(Clone, Copy, Debug)]
pub struct LaststandK {
    pub regroup: bool,
    pub call_out: bool,
    pub places: bool,
    pub moved_secs: f64,
    pub still_secs: f64,
    pub moved_gap: f64,
    pub few: f64,
    pub quiet: f64,
    pub first: [f64; 2],
    pub every: f64,
    pub again: f64,
    pub answer: [f64; 2],
    pub between: [f64; 2],
    pub listen: [f64; 2],
    pub max_m: f64,
    pub clear_m: f64,
    pub known_secs: f64,
    pub go: [f64; 2],
    pub legs: f64,
    pub dig: [f64; 2],
    pub thump: [f64; 2],
    pub thump_level: f64,
}
impl LaststandK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaststandK {
            regroup: false,
            call_out: false,
            places: false,
            moved_secs: 0.0,
            still_secs: 0.0,
            moved_gap: 0.0,
            few: 0.0,
            quiet: 0.0,
            first: [0.0; 2],
            every: 0.0,
            again: 0.0,
            answer: [0.0; 2],
            between: [0.0; 2],
            listen: [0.0; 2],
            max_m: 0.0,
            clear_m: 0.0,
            known_secs: 0.0,
            go: [0.0; 2],
            legs: 0.0,
            dig: [0.0; 2],
            thump: [0.0; 2],
            thump_level: 0.0,
        }
    }
}
impl Default for LaststandK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/clearline.ts:49 CLEARLINE, src/game/clearline.ts:49)
#[derive(Clone, Copy, Debug)]
pub struct ClearlineK {
    pub on: bool,
    pub after: f64,
    pub reach_m: f64,
    pub pace: f64,
    pub body_m: f64,
    pub spare_m: f64,
    pub cover: bool,
    pub cover_secs: f64,
    pub kneel: bool,
    pub cue: bool,
    pub again: f64,
    pub lose: f64,
    pub hold_max: f64,
    pub rest: f64,
    pub stale: f64,
    pub after_edge: f64,
    pub in_entry: bool,
    pub hold_behind: f64,
    pub hold_step: bool,
    pub still: f64,
    pub still_far: f64,
}
impl ClearlineK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ClearlineK {
            on: false,
            after: 0.0,
            reach_m: 0.0,
            pace: 0.0,
            body_m: 0.0,
            spare_m: 0.0,
            cover: false,
            cover_secs: 0.0,
            kneel: false,
            cue: false,
            again: 0.0,
            lose: 0.0,
            hold_max: 0.0,
            rest: 0.0,
            stale: 0.0,
            after_edge: 0.0,
            in_entry: false,
            hold_behind: 0.0,
            hold_step: false,
            still: 0.0,
            still_far: 0.0,
        }
    }
}
impl Default for ClearlineK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stackroom.ts:48 STACKROOM, src/game/stackroom.ts:48)
#[derive(Clone, Copy, Debug)]
pub struct StackroomK {
    pub on: bool,
    pub apart: f64,
    pub squeeze: f64,
    pub round_m: f64,
    pub round_k: f64,
    pub walk_m: f64,
    pub next_walk_m: f64,
    pub next_m: f64,
    pub lane_back: f64,
    pub view: bool,
    pub view_hand: bool,
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
impl StackroomK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StackroomK {
            on: false,
            apart: 0.0,
            squeeze: 0.0,
            round_m: 0.0,
            round_k: 0.0,
            walk_m: 0.0,
            next_walk_m: 0.0,
            next_m: 0.0,
            lane_back: 0.0,
            view: false,
            view_hand: false,
            view_ajar: 0.0,
            view_cells: 0.0,
            view_far: 0.0,
            view_eye: 0.0,
            view_step: 0.0,
            view_past: 0.0,
            view_off: 0.0,
            view_apart: 0.0,
            view_out: 0.0,
            view_men: 0.0,
            view_wall: 0.0,
        }
    }
}
impl Default for StackroomK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rearwatch.ts:61 REARWATCH, src/game/rearwatch.ts:61)
#[derive(Clone, Copy, Debug)]
pub struct RearwatchK {
    pub on: bool,
    pub through: bool,
    pub hold: bool,
    pub hold_secs: f64,
    pub reach: f64,
    pub ajar: f64,
    pub grace: f64,
    pub short: f64,
    pub room: bool,
    pub room_secs: f64,
    pub room_m: f64,
    pub room_last: bool,
}
impl RearwatchK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RearwatchK {
            on: false,
            through: false,
            hold: false,
            hold_secs: 0.0,
            reach: 0.0,
            ajar: 0.0,
            grace: 0.0,
            short: 0.0,
            room: false,
            room_secs: 0.0,
            room_m: 0.0,
            room_last: false,
        }
    }
}
impl Default for RearwatchK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stackline.ts:31 STACKLINE, src/game/stackline.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct StacklineK {
    pub on: bool,
    pub ajar: f64,
    pub bins: f64,
    pub eye: f64,
    pub inset: f64,
    pub on_mark: f64,
}
impl StacklineK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StacklineK {
            on: false,
            ajar: 0.0,
            bins: 0.0,
            eye: 0.0,
            inset: 0.0,
            on_mark: 0.0,
        }
    }
}
impl Default for StacklineK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sneak.ts:64 SNEAK, src/game/sneak.ts:64)
#[derive(Clone, Copy, Debug)]
pub struct SneakK {
    pub on: bool,
    pub crouch: bool,
    pub min_pace: f64,
    pub low_pace: f64,
    pub cross_pace: f64,
    pub post: bool,
    pub step: f64,
    pub trail: bool,
    pub trail_m: f64,
    pub back: f64,
    pub look: bool,
    pub ahead_m: f64,
    pub lit_at: f64,
    pub by_eye: bool,
    pub seen_rate: f64,
    pub ambient: bool,
    pub stalk_ambient: bool,
    pub cone_deg: f64,
    pub near_m: f64,
    pub cross_m: f64,
    pub patience: f64,
    pub cross_secs: f64,
    pub budget: f64,
    pub margin: f64,
    pub memory: f64,
    pub cue: bool,
    pub nag_secs: f64,
    pub cue_gap: f64,
    pub stand: bool,
    pub stand_m: f64,
    pub stand_near: f64,
    pub side: bool,
    pub side_m: f64,
    pub side_stop: f64,
    pub door: bool,
    pub door_m: f64,
    pub door_pad: f64,
    pub door_settle: f64,
    pub hide_gap: f64,
    pub noise_walk: bool,
    pub noise_walk_m: f64,
    pub plan_secs: f64,
    pub skill: f64,
}
impl SneakK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SneakK {
            on: false,
            crouch: false,
            min_pace: 0.0,
            low_pace: 0.0,
            cross_pace: 0.0,
            post: false,
            step: 0.0,
            trail: false,
            trail_m: 0.0,
            back: 0.0,
            look: false,
            ahead_m: 0.0,
            lit_at: 0.0,
            by_eye: false,
            seen_rate: 0.0,
            ambient: false,
            stalk_ambient: false,
            cone_deg: 0.0,
            near_m: 0.0,
            cross_m: 0.0,
            patience: 0.0,
            cross_secs: 0.0,
            budget: 0.0,
            margin: 0.0,
            memory: 0.0,
            cue: false,
            nag_secs: 0.0,
            cue_gap: 0.0,
            stand: false,
            stand_m: 0.0,
            stand_near: 0.0,
            side: false,
            side_m: 0.0,
            side_stop: 0.0,
            door: false,
            door_m: 0.0,
            door_pad: 0.0,
            door_settle: 0.0,
            hide_gap: 0.0,
            noise_walk: false,
            noise_walk_m: 0.0,
            plan_secs: 0.0,
            skill: 0.0,
        }
    }
}
impl Default for SneakK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pairup.ts:51 PAIRUP, src/game/pairup.ts:51)
#[derive(Clone, Copy, Debug)]
pub struct PairupK {
    pub on: bool,
    pub one_eye: bool,
    pub hunted: bool,
    pub rejoin: bool,
    pub stay: bool,
    pub word: bool,
    pub hands: bool,
}
impl PairupK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairupK {
            on: false,
            one_eye: false,
            hunted: false,
            rejoin: false,
            stay: false,
            word: false,
            hands: false,
        }
    }
}
impl Default for PairupK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/spotted.ts:63 SPOTTED, src/game/spotted.ts:63)
#[derive(Clone, Copy, Debug)]
pub struct SpottedK {
    pub on: bool,
    pub hold: bool,
    pub low: bool,
    pub freeze: bool,
    pub leash_m: f64,
    pub slip: bool,
    pub slip_m: f64,
    pub lit_at: f64,
    pub slip_wait: f64,
    pub wary_m: f64,
    pub ear_m: f64,
    pub linger_secs: f64,
    pub brace: bool,
    pub brace_secs: f64,
    pub brace_m: f64,
    pub close: bool,
    pub take_m: f64,
    pub take_reach: f64,
    pub take_secs: f64,
    pub fresh_secs: f64,
    pub caught_m: f64,
    pub last_secs: f64,
    pub keen: bool,
    pub gun: bool,
    pub step: bool,
    pub step_m: f64,
    pub hold_unaware: bool,
    pub face_deg: f64,
    pub close_m: f64,
    pub cue: bool,
}
impl SpottedK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpottedK {
            on: false,
            hold: false,
            low: false,
            freeze: false,
            leash_m: 0.0,
            slip: false,
            slip_m: 0.0,
            lit_at: 0.0,
            slip_wait: 0.0,
            wary_m: 0.0,
            ear_m: 0.0,
            linger_secs: 0.0,
            brace: false,
            brace_secs: 0.0,
            brace_m: 0.0,
            close: false,
            take_m: 0.0,
            take_reach: 0.0,
            take_secs: 0.0,
            fresh_secs: 0.0,
            caught_m: 0.0,
            last_secs: 0.0,
            keen: false,
            gun: false,
            step: false,
            step_m: 0.0,
            hold_unaware: false,
            face_deg: 0.0,
            close_m: 0.0,
            cue: false,
        }
    }
}
impl Default for SpottedK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/nightbreach.ts:61 NIGHTBREACH, src/game/nightbreach.ts:61)
#[derive(Clone, Copy, Debug)]
pub struct NightbreachK {
    pub room_lights: bool,
    pub op_lights: bool,
    pub def_lights: bool,
    pub stun: bool,
    pub blind_fire: bool,
    pub loud_boot: bool,
    pub sam_at: V3,
    pub sam_yaw: f64,
    pub squad: f64,
}
impl NightbreachK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NightbreachK {
            room_lights: false,
            op_lights: false,
            def_lights: false,
            stun: false,
            blind_fire: false,
            loud_boot: false,
            sam_at: [0.0; 3],
            sam_yaw: 0.0,
            squad: 0.0,
        }
    }
}
impl Default for NightbreachK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomclear.ts:53 ROOMCLEAR, src/game/roomclear.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct RoomclearK {
    pub seed: f64,
    pub reroll: bool,
    pub announce: bool,
    pub pair_only: bool,
    pub fresh: bool,
    pub way: f64,
}
impl RoomclearK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomclearK {
            seed: 0.0,
            reroll: false,
            announce: false,
            pair_only: false,
            fresh: false,
            way: 0.0,
        }
    }
}
impl Default for RoomclearK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lesson.ts:87 LESSON, src/game/lesson.ts:87)
#[derive(Clone, Copy, Debug)]
pub struct LessonK {
    pub on: bool,
}
impl LessonK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LessonK {
            on: false,
        }
    }
}
impl Default for LessonK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatch.ts:38 SNATCH, src/game/snatch.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct SnatchK {
    pub on: bool,
    pub give_up_m: f64,
    pub halt_m: f64,
    pub see_m: f64,
    pub see_rate: f64,
    pub cuff_m: f64,
    pub cuff_secs: f64,
    pub bolt_secs: f64,
}
impl SnatchK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchK {
            on: false,
            give_up_m: 0.0,
            halt_m: 0.0,
            see_m: 0.0,
            see_rate: 0.0,
            cuff_m: 0.0,
            cuff_secs: 0.0,
            bolt_secs: 0.0,
        }
    }
}
impl Default for SnatchK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/defuse.ts:31 DEFUSE, src/game/defuse.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct DefuseK {
    pub on: bool,
    pub work_secs: f64,
    pub reach: f64,
    pub marker_m: f64,
    pub shift: f64,
    pub start_m: f64,
    pub lethal_m: f64,
    pub wound_m: f64,
    pub radius: f64,
    pub power: f64,
    pub warn_secs: f64,
}
impl DefuseK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseK {
            on: false,
            work_secs: 0.0,
            reach: 0.0,
            marker_m: 0.0,
            shift: 0.0,
            start_m: 0.0,
            lethal_m: 0.0,
            wound_m: 0.0,
            radius: 0.0,
            power: 0.0,
            warn_secs: 0.0,
        }
    }
}
impl Default for DefuseK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/par.ts:24 PAR, src/game/par.ts:24)
#[derive(Clone, Copy, Debug)]
pub struct ParK {
    pub on: bool,
}
impl ParK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParK {
            on: false,
        }
    }
}
impl Default for ParK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1170 ISSUE, src/game/kit.ts:1170)
#[derive(Clone, Copy, Debug)]
pub struct IssueK {
    pub on: bool,
    pub whole: bool,
    pub picked: bool,
}
impl IssueK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IssueK {
            on: false,
            whole: false,
            picked: false,
        }
    }
}
impl Default for IssueK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/verdict.ts:35 VERDICT, src/game/verdict.ts:35)
#[derive(Clone, Copy, Debug)]
pub struct VerdictK {
    pub on: bool,
    pub loud_card: bool,
    pub win_secs: f64,
    pub loss_secs: f64,
}
impl VerdictK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VerdictK {
            on: false,
            loud_card: false,
            win_secs: 0.0,
            loss_secs: 0.0,
        }
    }
}
impl Default for VerdictK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/unseen.ts:57 UNSEEN, src/game/unseen.ts:57)
#[derive(Clone, Copy, Debug)]
pub struct UnseenK {
    pub on: bool,
    pub pace_at: f64,
    pub linger: f64,
    pub crouch: bool,
    pub pace: bool,
    pub min_pace: f64,
    pub low_pace: f64,
    pub gain: f64,
    pub cap: f64,
    pub post: bool,
    pub wall: bool,
    pub wall_off: f64,
    pub wall_out: f64,
    pub wall_cross: f64,
    pub wall_end: f64,
    pub step: bool,
    pub low_step: f64,
    pub walk_step: f64,
    pub lights: bool,
    pub hush: bool,
    pub doors: bool,
    pub hold: bool,
    pub glass: bool,
    pub cue: bool,
    pub cue_gap: f64,
    pub wall_beat: bool,
    pub wall_secs: f64,
}
impl UnseenK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnseenK {
            on: false,
            pace_at: 0.0,
            linger: 0.0,
            crouch: false,
            pace: false,
            min_pace: 0.0,
            low_pace: 0.0,
            gain: 0.0,
            cap: 0.0,
            post: false,
            wall: false,
            wall_off: 0.0,
            wall_out: 0.0,
            wall_cross: 0.0,
            wall_end: 0.0,
            step: false,
            low_step: 0.0,
            walk_step: 0.0,
            lights: false,
            hush: false,
            doors: false,
            hold: false,
            glass: false,
            cue: false,
            cue_gap: 0.0,
            wall_beat: false,
            wall_secs: 0.0,
        }
    }
}
impl Default for UnseenK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/seenwalk.ts:33 SEENWALK, src/game/seenwalk.ts:33)
#[derive(Clone, Copy, Debug)]
pub struct SeenwalkK {
    pub on: bool,
    pub lit_at: f64,
    pub dear: f64,
    pub long: f64,
    pub less: f64,
    pub eye_step: f64,
    pub eye_y: f64,
    pub margin: f64,
}
impl SeenwalkK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SeenwalkK {
            on: false,
            lit_at: 0.0,
            dear: 0.0,
            long: 0.0,
            less: 0.0,
            eye_step: 0.0,
            eye_y: 0.0,
            margin: 0.0,
        }
    }
}
impl Default for SeenwalkK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/holdfire.ts:45 HOLDFIRE, src/game/holdfire.ts:45)
#[derive(Clone, Copy, Debug)]
pub struct HoldfireK {
    pub on: bool,
    pub lift_secs: f64,
    pub glass: bool,
    pub quiet: bool,
    pub loud: bool,
    pub stack_set: bool,
    pub stack_wait: bool,
}
impl HoldfireK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldfireK {
            on: false,
            lift_secs: 0.0,
            glass: false,
            quiet: false,
            loud: false,
            stack_set: false,
            stack_wait: false,
        }
    }
}
impl Default for HoldfireK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:292 SLOTS, src/game/kit.ts:292)
#[derive(Clone, Debug)]
pub struct SlotsK {
    pub on: bool,
    pub ok: bool,
    pub why: String,
    pub table: crate::kit::SlotTable,
}
impl SlotsK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotsK {
            on: false,
            ok: false,
            why: String::new(),
            table: crate::kit::SlotTable::zeroed(),
        }
    }
}
impl Default for SlotsK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destructionSwitch.ts:5 DESTRUCTION, src/game/destructionSwitch.ts:5)
#[derive(Clone, Copy, Debug)]
pub struct DestructionK {
    pub on: bool,
}
impl DestructionK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DestructionK {
            on: false,
        }
    }
}
impl Default for DestructionK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:313 BIGSLOT, src/game/kit.ts:313)
#[derive(Clone, Copy, Debug)]
pub struct BigslotK {
    pub on: bool,
    pub swings: f64,
    pub swing: f64,
    pub windup: f64,
    pub draw: f64,
    pub shells: f64,
    pub shut: bool,
    pub beside: bool,
    pub moves: bool,
    pub snug: bool,
    pub weight: bool,
    pub equip: f64,
    pub stow: f64,
    pub reach: f64,
    pub heft: f64,
    pub equip_w: f64,
    pub draw_w: f64,
    pub stow_w: f64,
    pub flow: f64,
}
impl BigslotK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BigslotK {
            on: false,
            swings: 0.0,
            swing: 0.0,
            windup: 0.0,
            draw: 0.0,
            shells: 0.0,
            shut: false,
            beside: false,
            moves: false,
            snug: false,
            weight: false,
            equip: 0.0,
            stow: 0.0,
            reach: 0.0,
            heft: 0.0,
            equip_w: 0.0,
            draw_w: 0.0,
            stow_w: 0.0,
            flow: 0.0,
        }
    }
}
impl Default for BigslotK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:377 KITDEAL, src/game/kit.ts:377)
#[derive(Clone, Copy, Debug)]
pub struct KitdealK {
    pub on: bool,
    pub more: bool,
    pub stealth: bool,
}
impl KitdealK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitdealK {
            on: false,
            more: false,
            stealth: false,
        }
    }
}
impl Default for KitdealK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/cut.ts:31 CUT, src/scene/cut.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct CutK {
    pub on: bool,
    pub swap: bool,
}
impl CutK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CutK {
            on: false,
            swap: false,
        }
    }
}
impl Default for CutK { fn default() -> Self { Self::zeroed() } }

pub fn page_aim() -> AimK {
    crate::knobs::AimK { guard: crate::knobs::AimKGuard { settle_secs: 1.3, range_ref: 8.0, near_k: 0.25, acquire_min: 0.55, unsettled_k: 3.0, on_deg: 18.0, lose_rate: 3.0, mem_secs: 0.6, jump_deg: 25.0, target_move_k: 0.36, target_move_cap: 0.26, rate_tau: 0.15, shooter_move_k: 0.1, feet_tau: 0.5, steady_pace: 1.3, stop_to_shoot: 1.2, moving_track_cap: 0.5, shoot_band: 14.0, brake_lead: 0.45 }, player: crate::knobs::AimKPlayer { settle_secs: 0.8, range_ref: 8.0, near_k: 0.25, unsettled_k: 2.2, lose_rate: 3.0, base_stand: 0.012, base_crouch: 0.008, target_move_k: 0.2, target_move_cap: 0.16, rate_tau: 0.15, jump_deg: 25.0, shooter_move_k: 0.08, feet_tau: 0.6, steady_pace: 1.7, sprint_cap: 0.3, walk_cap: 0.7, moving_above: 0.3, crouch_k: 1.15, light_snap_px: 14.0, level: 1.0, low: 1.0 } }
}

pub fn page_trained() -> TrainedK {
    crate::knobs::TrainedK { settle_k: 0.6, spread_k: 0.7, react_cap: 0.45 }
}

pub fn page_ai_tune() -> AiTuneK {
    crate::knobs::AiTuneK { motion_acuity: true, dark_search: true, pieing: true }
}

pub fn page_body_light() -> BodyLightK {
    crate::knobs::BodyLightK { lit: 0.22, lit_near: 0.04, near_m: 2.0, far_m: 5.0, smoke: 0.3 }
}

pub fn page_guard_eye() -> GuardEyeK {
    crate::knobs::GuardEyeK { range: 16.0, half_deg: 58.0, near_m: 3.5, near_deg: 100.0, far: true, far_calm: true, far_fix_at: 0.5, far_range: 26.0, far_lo: 0.4, far_hi: 0.8, far_k: 0.1 }
}

pub fn page_needs() -> NeedsK {
    crate::knobs::NeedsK { rate_k: 1.0, max_away: 1.0, announce_lead: 20.0, say_lead: 12.0, stretch_lead: 5.0, partner_m: 14.0, reply_after: 1.5, dwell: { let mut t = JsMap::new(); t.set(crate::needs::NeedKind::Coffee, 25.0); t.set(crate::needs::NeedKind::Smoke, 40.0); t.set(crate::needs::NeedKind::Water, 12.0); t.set(crate::needs::NeedKind::Stretch, 0.0); t }, min_wait: 2.0, from_nearest: true, min_trip_m: 4.0, after_cancel: 0.5, after_alert: 0.3, rest_secs: 90.0, floor_gap_secs: 30.0, pace_k: 1.0, stuck_secs: 6.0 }
}

pub fn page_whistle() -> WhistleK {
    crate::knobs::WhistleK { level: 0.35, reach_k: 8.0, horizon: 12.0, cooldown: 2.0, twice_window: 60.0, awareness: 0.45, react_t: 0.5, walk_floor: 0.32, glance_secs: 1.5, glance_max: 1.2, gain: 0.55 }
}

pub fn page_wet() -> WetK {
    crate::knobs::WetK { soak_per_sec: 0.16666666666666666, dry_per_sec: 0.04, bead_runoff_secs: 3.0, bead_density: 0.6, bead_size: 0.0075, trail_len: 0.11, rough_wet: 0.12, rough_dry: 0.55, darken: 1.0, darken_mul: 0.35, spec_gain: 1.0, drip_secs: 5.0, prints: 10.0, line_band: 0.12, print_fade_secs: 30.0 }
}

pub fn page_wind() -> WindK {
    crate::knobs::WindK { def: 1.0, vents: true, vent_k: 1.0, storm: true, storm_k: 1.0, doors: true, door_k: 1.0, door_reach: 2.2, door_spread_deg: 22.0, door_open_rad: 1.2, gusts: true, gust_amp: 0.25, gust_period: 3.5, storm_gust_amp: 0.35, storm_gust_period: 2.5, ramp_up: 3.0, ramp_down: 4.0, smoke_relax: 3.0, sheet_wake: 0.35, sheet_kick: 2.0, debris_k: 0.15, near_m: 14.0 }
}

pub fn page_civ() -> CivK {
    crate::knobs::CivK { eye: crate::knobs::CivKEye { range: 12.0, half_deg: 50.0, near_m: 2.5, near_deg: 95.0 }, eye_busy: crate::knobs::CivKEyeBusy { range: 8.0, half_deg: 34.0, near_m: 2.0, near_deg: 80.0 }, eye_alarmed: crate::knobs::CivKEyeAlarmed { range: 14.0, half_deg: 62.0, near_m: 3.0, near_deg: 100.0 }, keen_k: 0.6, gun_k: 2.5, gun_m: 10.0, glimpse_at: 0.3, spot_at: 0.85, startle_secs: 4.0, startle_again_secs: 20.0, body_m: 10.0, freeze: [0.4, 0.8], run_speed: 3.2, walk_speed: 1.05, creep_speed: 1.2, report_m: 2.2, report_secs: 2.6, phone_secs: 6.0, guard_bonus_m: 4.0, exit_malus_m: 6.0, past_threat_m: 15.0, corner_m: 4.0, reflee_m: 6.0, lone_secs: 45.0, settle_secs: 75.0, comply_min: 3.0, scream_m: 16.0, see_m: 12.0, see_lit: 0.25, found_m: 3.0, found_secs: 1.2, outside_call_secs: 18.0, greet: true, greet_m: 3.0, greet_pair_secs: 240.0, greet_floor_secs: 70.0, evacuate_on_qrf: true }
}

pub fn page_chaos() -> ChaosK {
    crate::knobs::ChaosK { def: 0.5, heat_tau: 2.0, cell: 2.0, t_decal: 1.0, t_paper: 1.0, t_burst: 1.5, t_puff: 1.5, t_knock: 4.0, t_tile: 9.0, bang_heat: 9.0, puffs_per_sec: 8.0, puffs_per_frame: 2.0, papers_per_sec: 6.0, knocks_per_sec: 4.0, tile_gap_secs: 2.2, tiles_per_encounter: 8.0, decals_per_sec: 12.0, paper_reach_m: 0.75, knock_reach_m: 0.45, sheets_per_stack: 26.0, puff: crate::knobs::ChaosKPuff { radius: 0.2, density: 3.5, ttl: 0.35, speed: 0.55, temp: 0.35, push: 6.0, expand: 8.0 }, tile: crate::knobs::ChaosKTile { half: [0.29, 0.0075, 0.29], albedo: [0.66, 0.66, 0.63] }, vel_low: 1.0, vel_high_k: 2.5, imp: crate::knobs::ChaosKImp { bang: crate::knobs::ChaosKImpBang { radius: 4.0, strength: 3.0, up: 2.0 }, impact: crate::knobs::ChaosKImpImpact { radius: 0.6, strength: 1.5, up: 0.5, bias_k: 0.6, debris_m: 0.5 }, thud: crate::knobs::ChaosKImpThud { radius: 1.0, strength: 0.8, up: 0.6, per_frame: 4.0 }, chaos_k: 1.5 }, loud: crate::knobs::ChaosKLoud { fixture_hit_m: 0.18, fixture_blast_m: 3.2, stun_fixture_m: 0.6, stun_panels: false, fixtures_per_encounter: 10.0, fixture_gap_secs: 0.8, blast_dust_floor: 1.2, blast_dust_puffs: 2.0, heat_k: 2.0, burst_queue: 32.0, burst_secs: 3.0 }, pall: crate::knobs::ChaosKPall { on: true, solver: true, per_window: 6.0, window_secs: 4.0, slack: 8.0, clear: 1.8, clear_y: 1.7, min_radius: 0.5, max_gain: 2.0, wall: crate::knobs::ChaosKPallWall { near: 4.0, far: 3.0, radius: 0.45, density: 0.8, ttl: 1.5, speed: 1.6, temp: 0.25, push: 5.0, expand: 1.2, spread: 0.9, out: 0.5, dust: [36.0, 24.0], dust_life: 4.5, dust_size: 0.6, dust_speed: 1.2 }, burst: crate::knobs::ChaosKPallBurst { ring: 5.0, core_gain: 1.3, radius: 0.42, density: 0.85, ttl: 1.2, speed: 1.4, temp: 0.3, push: 5.0, expand: 1.5, spread: 0.6, dust: 48.0, dust_life: 3.8, dust_size: 0.5, dust_speed: 1.8, smoke: [0.34, 0.33, 0.32] } }, mess: crate::knobs::ChaosKMess { on: true, stand_y: 1.05, clear_y: 1.7, under: 0.06, min_radius: 0.28, crowd: 0.8, roof_y: 2.2, burst: crate::knobs::ChaosKMessBurst { core: 1.15, lobes: 6.0, reach: 1.25, lobe: 0.95, strength: 0.8 }, wall: crate::knobs::ChaosKMessWall { out: 0.55, near: crate::knobs::ChaosKMessWallNear { core: 1.0, lobes: 4.0, reach: 1.1, lobe: 0.85, strength: 0.7 }, far: crate::knobs::ChaosKMessWallFar { core: 1.2, lobes: 5.0, reach: 1.4, lobe: 0.95, strength: 0.9 } }, foot: crate::knobs::ChaosKMessFoot { out: 0.32, radius: 0.5, per_metre: 0.25, strength: 0.5 }, top: crate::knobs::ChaosKMessTop { lo: 0.08, hi: 1.4, strength: 0.75 }, rubble: crate::knobs::ChaosKMessRubble { least: 0.035, grow: 1.5, min: 0.09, max: 0.42, crowd: 0.7, strength: 0.85 } } }
}

pub fn page_ring() -> RingK {
    crate::knobs::RingK { speed: 30.0, gravity: 9.81, drag: 0.4, radius: 0.04, life: 1.5, substeps: 4.0, ko_below_neck: 0.07, stagger: 1.5, stagger_fire_cd: 1.5, stagger_aware: 0.92, stagger_back: 4.0, launch_noise: 0.45, clack_noise: 0.3, ko_noise: 0.3, hit_noise: 0.35, prop_j: 3.0, restitution: 0.2, keep_tangent: 0.25, spent_speed_max: 5.0, lead: true, solve_iters: 2.0 }
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
    crate::knobs::FragK { on: true, issue: 2.0, fuse: 3.0, order_fuse: 2.4, squad: 2.0, order_key: String::from("KeyG"), order_in: 3.4, lethal_m: 4.0, wound_m: 8.0, destr_m: 2.5, wound_kill: 0.6, stagger: [3.2, 1.2], lift: 0.2, sam_stun: true, door_stun: true, wall_stun: true, glass_m: 4.0, glass_power: 1.5, pane_energy: 0.0, see_m: 9.0, hear_m: 5.0, react: [0.25, 0.5], react_jit: 0.25, shout_m: 9.0, flee_m: 9.0, run_speed: 3.4, flee_wise: true, flee_keep: 1.0, rest_props: true, flee_apart: true, flee_apart_m: 1.0, flee_lane_m: 0.9, flee_onto: true, flee_sam: true, flee_look: true, down_kill: true, down_shove: 2.4, wall_flee: true, flee_door: true, flee_door_open: 1.0, flee_door_in: 1.0, flee_door_dip: 0.6, flee_door_mouth: 0.9, flee_door_leaf: 0.3, flee_door_via: 0.9, flee_door_via_m: 0.45, flee_door_new: 30.0, flee_door_most: 2.0, flee_door_try: 4.0, flee_door_nodes: 1500.0, defenders: true, def_share: 3.0, def_max: 3.0, def_fuse: 4.0, def_wind: 1.0, def_near: 3.5, def_far: 13.0, def_past: 2.0, def_fresh: 4.0, def_blocked: 1.2, def_bunch: 2.5, def_bunch_secs: 0.5, def_gap: 14.0, def_own_m: 6.0, def_scatter: [0.6, 2.2], cue_m: 12.0 }
}

pub fn page_shotgun() -> ShotgunK {
    crate::knobs::ShotgunK { on: true, spots: alloc::vec![[0.45, 1.0], [0.25, 1.1], [0.05, 1.2], [-0.25, 1.3], [-0.5, 1.3]], clear: 0.55, hinge: alloc::vec![[0.45, 1.0], [0.45, 1.2], [0.65, 1.2]], mark_reach: 1.8, line: 0.45, own_beat: 0.15, own_slack: 0.25, own_side: false, lock_in: 0.2, lock_h: 1.0, shells: 2.0, aim_cap: 2.5 }
}

pub fn page_glass() -> GlassK {
    crate::knobs::GlassK { hear_m: 30.0, hear_level: 0.85, bang_m: 2.5, bang_cracks: 2.0, r_min: 0.075, r_max: 0.2, crack_gap: 0.045, live_gap: 0.2, both_faces: true, ear_off: 0.55 }
}

pub fn page_glassfx() -> GlassfxK {
    crate::knobs::GlassfxK { on: true }
}

pub fn page_glassbreak() -> GlassbreakK {
    crate::knobs::GlassbreakK { max_sites: 8.0, break_e: crate::knobs::GlassbreakKBreakE { annealed: 3.0, tempered: 1.0 }, punch_e: 0.5, run_single_e: 2.5, run_star: 2.0, run_ring_secs: 0.03, crack_gap: 0.25, rho: { let mut t = JsMap::new(); t.set(crate::knobs::GlassbreakKRhoKey::Annealed, [0.03, 0.07, 0.14, 0.5]); t.set(crate::knobs::GlassbreakKRhoKey::Laminated, [0.018, 0.05, 0.11, 0.22]); t }, radials: { let mut t = JsMap::new(); t.set(crate::knobs::GlassbreakKRadialsKey::Annealed, [9.0, 4.0]); t.set(crate::knobs::GlassbreakKRadialsKey::Laminated, [10.0, 3.0]); t }, merge: 0.45, rz_min: 0.05, think: 0.2, stagger_per_m: 0.18, stagger_jit: 0.12, hang_area: 0.12, grip: crate::knobs::GlassbreakKGrip { side: 2.0, top: 1.0, wedged: 0.25 }, stand_ny: 0.7, stand_foot: 0.3, balance_tol: 0.02, blast_m: 2.6, bang_power: 0.6, tooth_area: 0.03, tooth_k: 0.8, blast_run_k: 0.2, blast_dice_k: 0.45, blast_sheet_k: 0.75, rake_open: 0.5, rake_m: 0.5, dice_grain: 0.012, clump_m: 0.12, dice_hang: 0.09, dice_speed: 4.2, clumps_per_frame: 40.0, releases_per_frame: 8.0, out: 0.7, spin: 5.0, blast_out: 5.0, shard_albedo: [0.72, 0.8, 0.82], chip_color: [0.8, 0.88, 0.9], chips_per_m2: 90.0, chips_min: 8.0, dice_chips_per_m2: 260.0, crumb_every: 12.0, gust_speed_k: 0.6, walk_clear_h: 1.9 }
}

pub fn page_glasspen() -> GlasspenK {
    crate::knobs::GlasspenK { on: true, energy: { let mut t = JsMap::new(); t.set(String::from("fn2000"), 1.0); t.set(String::from("mk18bare"), 1.0); t.set(String::from("g19bare"), 0.6); t.set(String::from("ak74m"), 1.0); t.set(String::from("akms"), 1.1); t.set(String::from("pkm"), 1.3); t.set(String::from("aks74u"), 0.9); t.set(String::from("fiveseven"), 0.6); t.set(String::from("guard9mm"), 0.5); t.set(String::from("m870"), 0.3); t.set(String::from("m1014"), 0.3); t.set(String::from("mp7"), 0.6); t }, by_class: { let mut t = JsMap::new(); t.set(String::from("rifle"), 1.0); t.set(String::from("carbine"), 0.9); t.set(String::from("pistol"), 0.5); t }, default_energy: 0.5, cost: { let mut t = JsMap::new(); t.set(crate::glassbreak::GlassType::Annealed, 0.1); t.set(crate::glassbreak::GlassType::Tempered, 0.16); t.set(crate::glassbreak::GlassType::Laminated, 0.42); t }, ref_thick: 0.012, oblique_max: 3.0, graze_cos: 0.17, min_energy: 0.12, max_panes: 4.0, deflect_deg: 5.0, bend_k: 0.5, hole_r: 0.04, hole_k: 0.25, holes: 24.0, shell_r: 0.04, stack_eyes: 3.0, stack_energy: 1.0, stack_least: true, stack_least_k: 0.75, fracture_k: 2.0, foes_through: true }
}

pub fn page_shards() -> ShardsK {
    crate::knobs::ShardsK { mid_per_m2: 140.0, mid_min: 2.0, mid_max: 18.0, mid_size: 0.02, mid_life: 60.0, fine_per_m2: 520.0, fine_min: 6.0, fine_max: 70.0, fine_size: 0.0045, fine_life: 22.0, back_share: 0.18, hit_fine_min: 16.0, hit_back_share: 0.4, spall_out: 10.0, spall_back: 6.0, dice_per_m2: 520.0, dice_min: 6.0, dice_max: 160.0, dice_life: 45.0, dice_fine_share: 0.35, crater_per_m2: 20000.0, crater_max: 48.0, exit_fines: 14.0, exit_speed: 4.0, speed: 1.0, live_cap: 4000.0, cap_floor: 3.0, over_cap: 1.25, press_life: 0.3, press_from: 0.35, recs_per_frame: 24.0, defer_max: 96.0, plate_min_area: 0.012, plate_share: 0.6, child_min_area: 0.004, shatter_v0: 1.6, shatter_v1: 4.0, rest_crumbs: 12.0, crumb_size: 0.009, rest_frost: true, cut_edge: 1.0, cut_see: 0.7, cut_rim: 1.25, cut_anim: true, cut_body: 0.1, cut_see_rest: 0.45, cut_rest_body: 0.3, max_gen: 2.0, contact: crate::knobs::ShardsKContact { keep_floor: 0.72, keep_wall: 0.85, rest: 0.3 }, sounds_per_frame: 3.0, pending_max: 48.0, sound_gap: { let mut t = JsMap::new(); t.set(String::from("glassTinkle"), 0.07); t.set(String::from("glassShatter"), 0.11); t }, sound_thicken: 0.5, sound_owed_secs: 0.6, level: 0.3, event_gap: 0.4, by_fall: true, underfoot: true, crunch_full: 0.012, crunch_gain: 0.6, crunch_add: 0.12, grind: 0.9 }
}

pub fn page_talk_tune() -> TalkTuneK {
    crate::knobs::TalkTuneK { scripts: true }
}

pub fn page_hold() -> HoldK {
    crate::knobs::HoldK { drop_gun: true, regrab: 0.9, bare_grip: 0.9, bare_arch: -11.0, bare_elbow: 0.6, yank: crate::knobs::HoldKYank { arch: -5.0, sink: 0.09, in_: 0.07, hold: 0.05, out: 0.32 }, lunge: crate::knobs::HoldKLunge { dip: 0.06, back: 0.035, t: [0.035, 0.03, 0.1] }, sway: crate::knobs::HoldKSway { m: 0.014, secs: 1.9, twist: 2.4, his: 1.5, in_: 0.6, out: 8.0 }, apart: crate::knobs::HoldKApart { side: 7.0, tip: 8.0 }, way_shape: true, catch_shape: true, catch: crate::knobs::HoldKCatch { via: [0.17, 1.49, 0.21], over: [0.08, 1.465, 0.38], at: [0.42, 0.74], arm_in: 0.3, ready_out: [0.0, 0.3], jolt: crate::knobs::HoldKCatchJolt { t: [0.07, 0.13, 0.24, 0.36], fwd: 12.0, head: -16.0, clav: 6.0, dip: 0.03 }, fling: crate::knobs::HoldKCatchFling { t: [0.24, 0.47], land: [0.73, 0.86], to: alloc::vec![[0.44, 1.08, 0.14], [-0.36, 1.27, 0.08]], drift: [0.03, 0.05, -0.04], arm_in: [0.05, 0.2] }, sink: 0.12, hips: 0.13, heels: [0.17, 0.27], seat: [0.07, 0.05, 0.06], seat_lean: 7.0, brace: [0.15, 0.12], brace_lift: 0.06, yank_sit: 0.5, feet: alloc::vec![[0.0, 0.17], [0.15, 0.36]], lift: 0.07, sam_feet: alloc::vec![[0.04, 0.2], [0.0, 0.12]], sam_lift: 0.05, settle: 0.4 }, hit_shape: true, lay_ahead: 0.9, hit: crate::knobs::HoldKHit { t: [0.0, 0.04, 0.07, 0.22], hips: 0.15, dip: 0.03, head: 1.0, heave: 12.0, sam_in: 0.05, sam_dip: 0.06 }, disarm: true, strip_first: true, first: crate::knobs::HoldKFirst { take: 0.16, drop: 0.3, catch: 0.26 }, strip: crate::knobs::HoldKStrip { take: 0.26, out: 0.35, drop: 0.41, follow: 0.1, knife: 0.7, home: 0.95, free: 0.36, up: 0.68, meet: [-0.36, 1.15, 0.05], torn: [-0.47, 1.06, -0.05], over: [0.0, 0.035, -0.05], wrench: [-0.15, -0.1, -0.2], low: [-0.66, 1.2, -0.08], yaw: -16.0, rap: [0.02, 0.08, -0.03], hold: [0.08, 0.02], pitch: [12.0, 25.0, 15.0], rest: 0.05, push: [1.3, 1.05], fall: 0.311, spin: 5.0 }, one_facing: true, dist: 0.27, secs: crate::knobs::HoldKSecs { grab: 0.45, choke_hold: 0.35, choke: 2.6, whip: 0.6, swap: 0.3, kill: 1.3 }, throat: [-0.11, 1.44, 0.35], via_l: [0.38, 1.42, 0.22], over_l: [0.16, 1.52, 0.33], tight: [-0.08, 1.45, 0.33], lean: 8.0, choke_lean: -4.0, head_pitch: 6.0, brace: 2.5, knife: crate::knobs::HoldKKnife { hand: [-0.165, 1.555, 0.36], blade: [1.0, 0.0, 0.12], back: [0.0, 0.28, 0.96], via: [-0.4, 1.5, 0.04], off_hand: [-0.24, 1.45, 0.26], off_blade: [-0.35, 0.75, 0.56], off_back: [-0.9, -0.3, 0.0], off_secs: 0.4, side: -8.0, head_yaw: 19.0, clav: 5.0 }, rest_l: [0.14, 1.47, 0.04], claw_l: [0.09, 1.39, 0.13], raised_r: [-0.36, 1.53, 0.06], via_up: [-0.38, 1.2, 0.12], claw_r: [-0.12, 1.38, 0.15], hang: alloc::vec![[0.27, 0.98, 0.14], [-0.27, 0.98, 0.14]], arch: -5.0, chin_up: -9.0, aside: 8.0, clav_up: 8.0, loll: 26.0, loll_roll: 11.0, lower: crate::knobs::HoldKLower { from: 0.6, by: 1.0, up: 7.0, sink: 0.4, fwd: 0.05, hip: 8.0, fold: 20.0, hang: [0.56, 0.26], sam_sink: 0.3, sam_fwd: 0.06, sam_knees: 0.55, sam_lean: 38.0, sam_look: 0.45, sam_arm: [0.5, 0.22] }, kill: crate::knobs::HoldKKill { wind: 0.21, hit: 0.3, out: 0.48, go: 0.46, after: 7.0, mouth: [0.07, 1.6, 0.36], chest: [-0.1, 1.28, 0.36], collar: [0.14, 0.98, 0.5], set: [0.06, 1.48, 0.29], across: [-0.2, 1.48, 0.33], wide: [-0.62, 1.4, 0.04], low: [-0.34, 1.05, 0.2], wide_blade: [0.55, -0.15, 0.8], wide_back: [-0.2, 0.95, 0.2], lean: [-12.0, 20.0], yaw: [16.0, -34.0], sam_sink: 0.22, look: 24.0, reach: alloc::vec![[0.28, 1.28, 0.32], [-0.3, 1.3, 0.34]], clutch: alloc::vec![[0.05, 1.41, 0.1], [-0.05, 1.4, 0.11]], chin: -27.0, arch: -12.0, jolt: [-12.0, -7.0], sink: 0.34, fwd: 0.08, hip: 8.0, fold: 36.0, hang: [0.2, 0.2] }, beats: crate::knobs::HoldKBeats { clamp: [0.03, 0.14], off: [0.41, 0.6], tip_by: [0.5, 0.62], tip: 32.0, back: 7.0, after: 2.6, lower: crate::knobs::HoldKBeatsLower { from: 0.58, by: 0.85, up: 7.0, sink: 0.4, fwd: 0.05, hip: 8.0, fold: 38.0, hang: [0.56, 0.26], sam_sink: 0.38, sam_fwd: 0.06, sam_knees: 0.55, sam_lean: 48.0, sam_look: 0.45, sam_arm: [0.52, 0.29] } }, haul: crate::knobs::HoldKHaul { in_: [0.02, 0.14], out: [0.38, 0.52], down: 7.0, sam_lean: -26.0, sam_arm: [0.07, 0.08], lift: 0.09, back: -0.08, recline: -14.0, arch: -4.0, chin: -6.0, feet: [0.11, 0.18], toes: 30.0, kick: [0.07, 0.05] }, sag: [0.02, 0.06], breath: crate::knobs::HoldKBreath { hz: 0.7, pitch: 1.6, clav: 2.2 }, struggle: crate::knobs::HoldKStruggle { rise: 0.1, fade_from: 0.35, fade_to: 0.92, yaw: 7.0, roll: 6.0, head: 9.0, claw: 0.035, dip: 0.018, hz: [2.3, 3.1, 1.9] }, gun: crate::knobs::HoldKGun { hand: [-0.2, 1.53, 0.15], bore: [0.21, -0.1, 0.97], out_hand: [-0.24, 1.53, 0.17], kick_pitch: 6.0, kick_back: 0.03, via_a: [-0.42, 1.24, 0.06], via_b: [-0.4, 1.5, 0.02], via_bore: [0.1, 0.6, 0.8], wind_hand: [-0.36, 1.68, -0.04], wind_bore: [-0.15, 0.8, 0.58], hit_hand: [-0.1, 1.62, 0.15], hit_bore: [0.5, 0.3, 0.8], thru_hand: [-0.06, 1.5, 0.2], lean: 5.0, side: -8.0, head_yaw: 19.0, head_pitch: 4.0, wind_yaw: -8.0, hit_yaw: 5.0, clav: 7.0, hands_up: alloc::vec![[0.27, 1.43, 0.31], [-0.34, 1.41, 0.29]], aside: 13.0, away_yaw: 6.0, chin_up: -6.0, duck: crate::knobs::HoldKGunDuck { yaw: 5.0, roll: 9.0, pitch: 10.0, hunch: 6.0, secs: 0.45 }, knock: crate::knobs::HoldKGunKnock { yaw: 24.0, roll: 16.0, pitch: 18.0 } } }
}

pub fn page_suppression() -> SuppressionK {
    crate::knobs::SuppressionK { near_r: 1.8, w_near: 0.4, hit_r: 2.5, w_hit: 0.3, min_travel: 3.0, half_life: 1.1, cap: 1.0, kick_decay: 8.0, down_accrual: 0.5, friendly_k: 0.5, keen: { let mut t = JsMap::new(); t.set(String::from("steady"), 1.0); t.set(String::from("nervy"), 1.25); t.set(String::from("hard"), 0.8); t }, flinch: 0.3, pinned: 0.6, release: 0.45, head_down: [0.9, 1.5], flinch_delay: 0.25, head_down_delay: 0.6, guard_spread: 0.08, gap_k: 1.0, standoff_pace_k: 0.5, displace_after: 4.0, displace_min: 2.0, displace_max: 4.5, cover_hold: 3.0, displace_cooldown: 6.0, displace_cap_secs: 6.0, displace_pace: 1.1, displace_crouch: 0.55, src_memory_secs: 4.0, bark_secs: 6.0, floor_bark_secs: 1.5, pair_secs: 2.0, floor_line_secs: 20.0, sam_spread: 0.07 }
}

pub fn page_rifle_spread() -> RifleSpreadK {
    crate::knobs::RifleSpreadK { still: 0.01, moving: 0.028, move_at: 0.3, crouch_k: 0.7, brace_k: 0.45 }
}

pub fn page_footfall() -> FootfallK {
    crate::knobs::FootfallK { once: true, travel: 0.25, owed: true, owe: 0.5 }
}

pub fn page_twogears() -> TwogearsK {
    crate::knobs::TwogearsK { on: true, crouch: 1.45 }
}

pub fn page_crouchrun() -> CrouchrunK {
    crate::knobs::CrouchrunK { on: true, pace: 2.5, loud: 0.34 }
}

pub fn page_runready() -> RunreadyK {
    crate::knobs::RunreadyK { on: true, fwd: 0.24, up: -0.185, pitch: -50.0, tuck: 0.6 }
}

pub fn page_runarms() -> RunarmsK {
    crate::knobs::RunarmsK { on: true, at: 0.05, keep: 0.3, mid: 0.04, swing: 0.12, pump: 1.8, lift: 0.14, pole: [0.4, -0.45, -0.8], pole_w: 0.85 }
}

pub fn page_samlook() -> SamlookK {
    crate::knobs::SamlookK { on: true, near_m: 8.0, full_m: 5.5, back_deg: 120.0, keep_deg: 140.0, notice_m: 3.0, swap_m: 0.75, slip: true, slip_from: 2.6, slip_by: 1.2, back_cos: [-0.45, -0.1] }
}

pub fn page_dash() -> DashK {
    crate::knobs::DashK { stride: 0.3, drop: 0.03, lean: 7.0, rate: 7.0 }
}

pub fn page_watch() -> WatchK {
    crate::knobs::WatchK { yaw: 72.0, down: 28.0, up: 22.0, share: [0.14, 0.34, 0.52], rate: 6.0, w_rate: 5.0, lean: 9.0, drop: 0.035, away: 0.03, arm_out: 0.32, arm_down: 0.42, arm_w: 0.7, arm_side: [0.2, 0.55], slip_rate: 5.0, squeeze: 0.6 }
}

pub fn page_stalk() -> StalkK {
    crate::knobs::StalkK { on: true, only: true, cursor_m: 1.1, begin_cos: -0.25, out_cos: -0.05, station_m: 0.55, grab_m: 0.95, pace: 1.6, close_min: 0.3, close_k: 0.6, ease_up: 4.0, ease_down: 9.0, tell: true, reach_from: 2.9, reach: 1.0, rise_from: 2.3, rise_by: 1.05, rise: 0.7, catch_secs: 0.3, catch_m: 0.75, catch_long: 1.1, turn_in: true, catch_stride: 1.5, yank: 0.28, yank_room: 0.4, yank_with: true, corner: true, corner_m: 1.35, corner_deg: 110.0, jolt: 0.6, jolt_face: 50.0, jolt_secs: 0.6, jolt_at_once: false, corner_show: true, corner_see: 3.2, corner_reach: [0.25, 0.55], corner_coil: 0.3, corner_rise: 0.24, key: true, key_m: 0.8, low: true, low_from: 2.6, low_by: 1.1, low_k: 0.35, low_drop: 0.06, low_back: 0.03, low_lean: 6.0, low_hands: 1.0, low_spring: 18.0, low_wind: 0.12, wide: true, wide_out: 0.1, wide_down: 0.32, wide_fwd: 0.24, wide_lead: 0.07, wide_by: 0.35, wide_pole: [0.45, 0.15, -0.88], heft: true, heft_twist: 20.0, heft_swing: 0.11, heft_side: 0.04, heft_lean: 4.0, heft_dip: 0.02, heft_surge: 1.0, heft_out: 0.18, heft_down: 0.4, heft_fwd: -0.2, heft_lead: 0.0, heft_pole: [0.6, 0.0, -0.8], drag: false, drag_dead: 14.0, drag_full: 140.0, orbit: true, orbit_k: 3.0, orbit_edge: 0.05, orbit_snap: 0.3, low_begin_m: 4.2, low_out_m: 5.0 }
}

pub fn page_tapstep() -> TapstepK {
    crate::knobs::TapstepK { on: false, tap_m: 0.28, beat_m: 0.45, reach_m: 0.65, secs: 0.75, rushed: 0.35, settle: 0.08, beat: 0.16, quiet: 0.08, scuff: 0.4, mask_secs: 0.16, mask_m: 7.0, mask: 0.3, land_m: 0.95, land_cos: 0.8, follow: 0.6, turn: 10.0, legs: 14.0 }
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
    crate::knobs::CkptK { quiet_secs: 90.0, aware_max: 0.4, debounce_secs: 2.0, min_gap_secs: 10.0, pending_secs: 25.0, body_hide_m: 1.5, auto_slots: 3.0, path_keep: 1000.0, flash_secs: 2.6, regret_secs: 8.0 }
}

pub fn page_fog() -> FogK {
    crate::knobs::FogK { dim: 0.4, desat: 0.0, fov_deg: 360.0, near_m: 2.0, cone: false, cone_deg: 120.0, cone_near_m: 1.5, cone_soft_deg: 12.0, cone_near_soft_m: 0.6, own: false, calls: true, call_secs: 10.0, call_gap: 3.0, call_known: 3.0, smoke: 0.3, linger: 0.25, top_pad: 0.3, doors: true, squad_doors: true, squad_doors_m: 2.0, squad_fire: true, squad_fire_secs: 1.0, lasers: true, things: true, flat: 0.12, lit: 1.0, full: true, air: true, thin: true, eyes: 7.0, kept: true, filed: true, tracks: true, mates: true, ours: true, full_fill: 0.0, edge: 0.5, lit_fill: 0.5, lit_ao: 0.75, lit_ao_m: 1.5, lit_cut: 0.006, last_seen: 0.0 }
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

pub fn page_wound() -> WoundK {
    crate::knobs::WoundK { on: false, centre: 0.105, head: 0.125, lead: 0.4, depth: 0.7, back_room: 1.9 }
}

pub fn page_doorpath() -> DoorpathK {
    crate::knobs::DoorpathK { on: true, at: 0.52, out_m: 0.7, in_m: 0.7, look: 0.5, leaf_clear: 0.45, tip_m: 0.75, gap_m: 0.0, short_m: 0.6, cap: 2.5, move_m: 0.0, apart_m: 0.7, gun: false, gun_m: 0.95, gun_ajar: 0.12, gun_lead: 0.3, gun_side: 0.15, gun_over: false, gun_still: true, gun_down: true, gun_lane: true, body_m: 0.0, body_off: 0.6, body_move: 1.2 }
}

pub fn page_walls() -> WallsK {
    crate::knobs::WallsK { wall_max_thick: 0.17, wall_min_top: 2.2, wall_min_len: 0.5, col: 0.5, sill: 0.7, tough: { let mut t = JsMap::new(); t.set(String::from("drywall"), 6.0); t.set(String::from("wood"), 9.0); t.set(String::from("cardboard"), 3.0); t.set(String::from("fabric"), 4.0); t }, sill_k: 1.5, pistol_k: 0.5, cover_min_top: 0.45, cover_min_len: 0.4, cover_max_cells: 48.0, blast: crate::knobs::WallsKBlast { radius: 2.2, hole_k: 0.45, port_k: 0.8, cover_k: 0.7, door_k: 0.6, min_hole: 3.0, contact_m: 0.8 }, max_pieces: 512.0, max_changes: 256.0, chunks_per_cell: 2.0, chunks_per_blast: 10.0, fall_max_vol: 0.5, falls_per_change: 48.0, hung: crate::knobs::WallsKHung { deep: 0.1, long: 0.5, tall: 0.3 } }
}

pub fn page_wallcharge() -> WallchargeK {
    crate::knobs::WallchargeK { on: true, menu: crate::knobs::WallchargeKMenu::Page, pick_m: 0.06, height: 1.05, end_m: 0.75, slide: alloc::vec![0.0, 0.5, -0.5, 1.0, -1.0, 1.5, -1.5], set_off: 0.5, set_near: 0.9, sam_reach: 1.35, sam_shift: 0.35, sam_marker_m: 3.0, standoff: alloc::vec![2.6, 2.0], post_off: 0.55, file_gap: 0.9, back_m: alloc::vec![3.0, 3.9, 4.8], clear_m: 0.45, brace: true, mate_m: 2.0, after_blow: 0.7, enter_cap: 16.0, sweep_secs: 1.5, mouth_in: 0.6, lane: 0.35, lane_watch: 3.0, pods: alloc::vec![[-1.3, 1.9], [1.3, 1.9], [-2.1, 1.0], [2.1, 1.0]], hole: false, doors: true, hold_late: true, cost: 2.0, carried: None, hold_each: true, hold_secs: 1.5, mate_line: 0.6, off_frame: 0.8, off_frame_pod: 0.5, off_holes: true, outer: true, outer_thick: 0.26, outer_min_len: 1.5, outer_yard: true, far_furn: true, far_slack: 0.5, far_reach: 0.375, far_mouth: true, safe: true, hung: true, hung_m: 0.35, safe_standoff: [4.0, 6.0], safe_reach: [3.0, 5.0], safe_secs: [3.0, 1.0], safe_hide_m: 9.0, safe_walk_m: 16.0, safe_back: true, safe_back_m: 9.0, out_of_fire: true, fire_known_s: 30.0, fire_m: 30.0, fire_step: 0.5, fire_grain: 0.03, men_clear: true, men_clear_in: 0.1, men_clear_s: 0.75, sam_secs: [2.0, 0.6], sam_sup: 0.5 }
}

pub fn page_wallshot() -> WallshotK {
    crate::knobs::WallshotK { on: true, pall: 0.75, stagger_secs: 3.5, stun_secs: [8.0, 4.0], room_m: 12.0, ours_secs: 2.0 }
}

pub fn page_wallkill() -> WallkillK {
    crate::knobs::WallkillK { on: true, near_m: 1.5 }
}

pub fn page_hardbreath() -> HardbreathK {
    crate::knobs::HardbreathK { on: true, chest_amp: 4.2, chest_rate: 0.9, chest_up: 2.5, chest_down: 12.0, every: 0.95, puffs: 4.0, count: 9.0, jet: 3.2, size: 2.0, carry: 0.85, up: 2.5, down: 14.0 }
}

pub fn page_castor() -> CastorK {
    crate::knobs::CastorK { on: true, decel: 0.4, visc: 0.45, spin: 1.2, start_v: 0.3, stop_v: 0.1 }
}

pub fn page_wallreact() -> WallreactK {
    crate::knobs::WallreactK { on: true, see_m: 18.0, fov_deg: 116.0, hear_blast_m: 26.0, hear_fire_m: 10.0, hear_fall_m: 14.0, tell_m: 12.0, tell_secs: 1.2, near_m: 2.6, retry_secs: 5.0, every_gone: true, gone_max: 6.0, blast_flood: true, step_m: 3.5, run_m: 9.0, beyond_m: 2.5, ev_open: 1.6, ev_seen: 1.5, ev_fire: 1.0, forget: 0.12, hole_base: 0.45, port_base: 0.2, crowd: 0.35, sticky: 0.15, fresh_secs: 3.0, fresh: 3.0, watch_m: 16.0, entry_risk: true, say: true }
}

pub fn page_coverfire() -> CoverfireK {
    crate::knobs::CoverfireK { on: true, stale_secs: 5.0, call_secs: 1.2, call_m: 14.0, call_walls: false, behind_m: { let mut t = JsMap::new(); t.set(String::from("cover"), 3.0); t.set(String::from("wall"), 4.0); t }, max_layers: 3.0, max_rounds: 20.0, max_m: 30.0, min_m: 1.5, near_m: 1.0, slack_k: 1.5, slack: 3.0, give_secs: 20.0, lane: 0.6, cone_k: 0.03, past_m: 8.0, same_m: 1.5, linger_secs: 1.0, lead_secs: 0.8, lead_m: 1.5, low_y: 0.55, round_secs: 0.25, walk_pace: 3.0, walk_ask_secs: 1.5, line: String::from("he's behind that. going through it."), say_secs: 12.0 }
}

pub fn page_furnblast() -> FurnblastK {
    crate::knobs::FurnblastK { on: true }
}

pub fn page_doorway() -> DoorwayK {
    crate::knobs::DoorwayK { one_way: true, exits_out: false, frame_holds: true, side_kick: true, kick_glass: true, solid: true, leaf_way: true }
}

pub fn page_doorblast() -> DoorblastK {
    crate::knobs::DoorblastK { on: true, tear_m: crate::knobs::DoorblastKTearM { stun: 0.0, frag: 1.5 }, charge_k: 0.55, charge_r: 0.8, speed: [3.2, 6.5], up: 0.9, tip: 2.4, spin: 1.3, scrape: 4.0, slam: 0.4, slide: 12.0, bounce: 0.15, room: 0.6, land_level: 0.9 }
}

pub fn page_doorshot() -> DoorshotK {
    crate::knobs::DoorshotK { on: true, toll: 2.0, flat: true, strips: 4.0, lengths: 3.0, uneven: 0.6, chunks: 11.0, fly: alloc::vec![1.0, 3.2], side: 1.1, up: alloc::vec![0.2, 1.8], spin: 9.0, tilt: alloc::vec![0.15, 0.45], topple: alloc::vec![2.0, 5.0], chips: alloc::vec![60.0, 24.0], chip_v: alloc::vec![4.0, 2.5], shed: 8.0, slivers: alloc::vec![22.0, 3.2], level: 0.8 }
}

pub fn page_doorthrow() -> DoorthrowK {
    crate::knobs::DoorthrowK { on: true, push_secs: 2.0, kick_secs: 4.0, startle: 1.0, wary: 0.5, soft: true, jolt: true, min_speed: 0.9, hand_rate: 1.2, floor_rad: 0.6, rate: 9.0, slide: 1.5, again: 3.0, aware: 0.92 }
}

pub fn page_winbars() -> WinbarsK {
    crate::knobs::WinbarsK { on: true, per_round: 4.0, pieces: 3.0, uneven: 0.5, fly: [1.6, 3.4], rest_k: 0.45, up: [0.2, 1.3], side: 0.8, spin: 7.0, chips: 18.0, chip_v: 2.8, blast_k: 1.0 }
}

pub fn page_opening() -> OpeningK {
    crate::knobs::OpeningK { on: true, clear_m: 10.0, step_m: 0.5 }
}

pub fn page_doorstay() -> DoorstayK {
    crate::knobs::DoorstayK { on: true }
}

pub fn page_handle() -> HandleK {
    crate::knobs::HandleK { on: true, push: true, pull: true, tools: true, look: 2.6, ajar: 1.35, reach: 0.95, handle_in: 0.14, far_reach: 1.5, push_in: 0.33, push_off: 0.85, push_thru: 0.35, stand_in: 0.3, stand_off: 0.6, thru_at: 1.2, thru_off: 0.75, body: 0.34, slack: 0.1, near: 0.05, turn: 0.35, clear: 1.05, wait: 1.1, press: 1.5, tool_post: 0.0, tool_near: true, near_body: 0.38, near_post: 1.3, go_cap: 4.0, rest: 2.5 }
}

pub fn page_doormanners() -> DoormannersK {
    crate::knobs::DoormannersK { on: true, pass: true, stand: true, fight: true, swing: true, reach: 3.0, half: 0.45, throat: 1.25, gap: 1.05, gap_min: 0.7, brake: 8.0, patience: 1.1, rest: 2.0, beyond: 0.6, aside: 0.4, jamb_out: 0.6, jamb_past: 0.5, file_gap: 0.9, apart: 0.85, step: 1.4, round: 1.2, round_out: 0.85, own_place: true, own_m: 0.45, own_end_m: 2.5, leaf_round: true, leaf_stall: true, leaf_stall_m: 0.12, leaf_stall_secs: 0.6, leaf_stall_reach: 0.6, leaf_stall_ahead: 0.3, leaf_stall_push: 1.5, sweep: 0.2, lane: true, lane_ajar: 0.6, lane_m: 6.0, lane_out: 0.3, lane_flare: 0.12, lane_clear: 0.15, lane_apart: 0.7, lane_move: true, lane_leaf: true, watch: true, stack_cover: true, cover_m: 2.0, cover_deep: 2.5, cover_look: 0.8, cover_quiet: 1.0, cover_cap: 8.0, lane_walk: true, lane_dear: 6.0, lane_strip: 0.7, lane_long: 1.6, funnel: 0.6, funnel_rest: 1.5, through: 0.95, past: 0.6, step_cover: true, plane_deg: 30.0, rest_clear: true, rest_secs: 1.0, mark_clear: true, pod_clear: false, clear_m: 1.4, short_clear: true, held_cap: 12.0, own_leaf: true, own_mark: true, off_furniture: true, back_on: true }
}

pub fn page_breaks() -> BreaksK {
    crate::knobs::BreaksK { share: crate::knobs::BreaksKShare { big: crate::knobs::BreaksKShareBig { thrown: 5.0, heap: 5.0, furniture: 8.0 }, small: crate::knobs::BreaksKShareSmall { thrown: 4.0, heap: 1.0, furniture: 4.0 } }, small_pool_chip_k: 1.4285714285714286, refuse_chips: [6.0, 8.0], crumble_chips: [4.0, 6.0], sized_chips: [4.0, 14.0], sized_span: [0.05, 0.5], china_land: 3.5, china_max: 0.06, core_tint: 0.3, chip_tint: 0.5, heap: crate::knobs::BreaksKHeap { span: [0.15, 0.55], out: [0.0, 0.7], tip: [0.07, 0.24] }, max_standing_planes: 4.0, across_deg: [35.0, 55.0], look_edges: true, look_heap: true, loose_min: 0.08, kink: crate::knobs::BreaksKKink { deg: [25.0, 42.0], skew: 20.0, off: 0.4, min_long: 0.2, gap_secs: 0.5 }, head: crate::knobs::BreaksKHead { on: false, from: 1.9, deep: [0.2, 0.3], planes: 2.0, across: [14.0, 32.0], min_room: 0.3, most_of_width: 0.45 }, steel: crate::knobs::BreaksKSteel { sheet: 0.008, kink: [9.0, 24.0], dents: [3.0, 5.0], dent: [0.025, 0.06], dent_most: 0.14, rim: 0.035, lip: 0.004, inside: 0.16 }, stud: crate::knobs::BreaksKStud { half: 0.02, depth_k: 0.7, foot: 0.13, albedo: [0.56, 0.57, 0.6], rough: 0.35, metal: true, does: crate::scene_materials::DoesWord::Snow } }
}

pub fn page_furn() -> FurnK {
    crate::knobs::FurnK { k: { let mut t = JsMap::new(); t.set(crate::furnblast::BlastSource::Wall, 1.0); t.set(crate::furnblast::BlastSource::Door, 0.7); t.set(crate::furnblast::BlastSource::Frag, 0.6); t }, fling: { let mut t = JsMap::new(); t.set(crate::furnblast::BlastSource::Wall, 1.6); t.set(crate::furnblast::BlastSource::Door, 1.2); t.set(crate::furnblast::BlastSource::Frag, 1.2); t }, shatter_m: 1.6, apart_m: 4.0, throw_m: 5.5, throw_v: 6.0, prop_break_m: 1.5, chunks_per_blast: 36.0, mouth_m: 0.45, merge_m: 1.5, mouth: crate::knobs::FurnKMouth { on: true, depth: 0.9, top: 2.3, vol: 3.0, chunks: 12.0, snow: true, base: true, base_thick: 0.08 }, hung: crate::knobs::FurnKHung { len: 0.32, max: 4.0, strips: 2.0, under_m: 0.3, through: alloc::vec![7.0, 11.0], back: alloc::vec![3.5, 7.0], side: 4.5, up: alloc::vec![1.6, 3.0], spin: 24.0, chunks: 8.0, chips: alloc::vec![40.0, 24.0], chip_v: alloc::vec![5.0, 7.0], hole_m: 0.5 }, root: crate::knobs::FurnKRoot { min_top: 0.3, max_top: 2.0, max_long: 2.6, max_vol: 0.7, apart_max_vol: 0.6 }, rider: crate::knobs::FurnKRider { max_top: 2.3, max_vol: 0.3 }, touch: 0.04, row_m: 3.6 }
}

pub fn page_snowbog() -> SnowbogK {
    crate::knobs::SnowbogK { on: true, dead: 0.85, grip: 0.8, stick: 1.1, packed: 0.4, old: 0.7, top: 0.06, bank_m: 0.15, run: 2.0 }
}

pub fn page_rubble() -> RubbleK {
    crate::knobs::RubbleK { on: true, radius: 0.3, reach: 0.12, lead: 0.12, inside: 0.06, floor: 0.12, heavy: 12.0, walk: 0.35, base: 0.6, gain: 0.8, aside: 1.2, v_max: 2.2, off: 1.0, slide: 0.3, spin: 2.2, per_frame: 4.0, cooldown: 0.3, give_up: 8.0, give_up_secs: 3.0, scuff: 0.12 }
}

pub fn page_evict() -> EvictK {
    crate::knobs::EvictK { on: true }
}

pub fn page_debris() -> DebrisK {
    crate::knobs::DebrisK { dress: true, min_speed: 1.0 }
}

pub fn page_lightwall() -> LightwallK {
    crate::knobs::LightwallK { on: true, gap: 0.03 }
}

pub fn page_airdust() -> AirdustK {
    crate::knobs::AirdustK { on: true, diffuse: 1.5, settle: 0.12, step: 0.03333333333333333, recheck_secs: 3.0 }
}

pub fn page_wallsnap() -> WallsnapK {
    crate::knobs::WallsnapK { on: true, take_base: 0.1, take_per_m: 0.16, take_per_turn: 0.06, take_min: 0.12, take_max: 0.3, ease_pow: 2.0, yaw_pow: 3.0, reach_moving: 1.15, heading_min: 0.5, heading_bonus: 0.35, buffer: 0.15, dip: 0.035, nod: 4.0, hit_peak: 0.05, scuff: 0.3, step_lift: 0.06, step_trail: 0.5, pose: true, cam_room: 0.35, cam_side: 0.3, along_deg: 55.0, away_secs: 0.1, away_diag: 0.18, off_push: 0.7, out_rate: 24.0, stack: true, stack_draw: true, up_secs: 1.2, up_rise: 0.16, bore: 0.9, lean_along: 0.6, lean_out: 0.18, lean_back: 0.14, lean_on_deg: 8.0, lean_off_deg: 2.0, lean_min_dist: 0.6, lean_max_deg: 100.0, lean_keep_deg: 15.0, lean_keep_dist: 0.2, lean_over: true, lean_over_y: 1.9, lean_clear_m: 1.4, lean_gun: 0.42, lean_gun_step: 0.06, dwell_out: 0.06, dwell_back: 0.15, min_out: 0.35, after_shot: 0.4, press: true, press_secs: 0.18, press_flow: 0.2, press_near: 0.6, press_deg: 35.0, press_touch: 0.08, press_thing: 0.5, press_thing_at: 0.9, press_turn: 50.0, press_again: 0.6, press_after_shot: 1.0, press_calm: true, press_run: true, voice: true, voice_on: 0.35, voice_off: 0.5, slow: true, slow_take: 2.0, slow_in: 11.0, slow_out: 12.0 }
}

pub fn page_gunwall() -> GunwallK {
    crate::knobs::GunwallK { on: true, down_m: 0.03, up_m: 0.12, lead_s: 0.2, at_m: 0.5, tuck_m: 0.04, tuck_lead_s: 0.12, fight_s: 5.0, mark: true }
}

pub fn page_close() -> CloseK {
    crate::knobs::CloseK { on: true, mode: true, ready: true, dark: true, marks: false, side: true, front: true, seat: true, alert_strike: true, lethal: true, held_clicks: true, reach_up: true, hands_low: true, no_knife: true, rmb_hold: true, rmb_tap_secs: 0.2, rmb_loud: 0.15, rmb_wait_secs: 1.2, hands_low_neck_m: 1.15, mode_m: 3.0, mode_out_m: 3.6, dark_at: 0.1, lit_at: 0.3, touch_m: 0.55, fade_m: 0.9, reach: crate::knobs::CloseKReach { behind: 1.05, side: 0.95, front: 0.85, seat: 1.2, seat_front: 0.95 }, side_from_deg: 50.0, inner: 0.34, step_deg: 7.5, strike_dist: 0.6, seat_dist: 0.72, noise: crate::knobs::CloseKNoise { behind: 0.3, side: 0.38, front: 0.5, seat: 0.3 }, alert_noise: 0.15 }
}

pub fn page_plan() -> PlanK {
    crate::knobs::PlanK { on: true, paths: true, marks: true, sectors: true, who: true, tags: true, word: true, throws: true, menu_only: true, key_lit: true, thr_told: true, fresh_secs: 2.5, walk_dim: 0.6, rest_dim: 0.45, ring_m: 0.3, sector_m: 1.6, sector_deg: 24.0, sight_m: 14.0, reask_secs: 0.5, pair_bar_m: 0.55, others: 0.45 }
}

pub fn page_gear() -> GearK {
    crate::knobs::GearK { on: true }
}

pub fn page_start() -> StartK {
    crate::knobs::StartK { fall_in: true }
}

pub fn page_samestart() -> SamestartK {
    crate::knobs::SamestartK { on: true }
}

pub fn page_take() -> TakeK {
    crate::knobs::TakeK { wired: true }
}

pub fn page_reload() -> ReloadK {
    crate::knobs::ReloadK { auto: true, delay: 0.25, last_mag: true }
}

pub fn page_quickthrow() -> QuickthrowK {
    crate::knobs::QuickthrowK { on: false }
}

pub fn page_samstun() -> SamstunK {
    crate::knobs::SamstunK { on: true, blind_roots: false, fade: 0.6, own_seen: true }
}

pub fn page_recap() -> RecapK {
    crate::knobs::RecapK { on: true, secs: 8.0, after: 1.2, hz: 20.0, keep: 6.0, live: true, live_arrows: false, live_see_m: 25.0, live_saw_secs: 1.0, live_hear_m: 25.0, live_beat: 0.6, live_side_m: 8.0, live_mark_secs: 20.0 }
}

pub fn page_aimweight() -> AimweightK {
    crate::knobs::AimweightK { on: true, rate: 11.0, smooth: 0.05, heft: 1.5, bare: 1.25, on_m: 0.3, reach: 18.0, launch_wait: 0.5 }
}

pub fn page_aimcost() -> AimcostK {
    crate::knobs::AimcostK { lamp: true, move_: true, bare: 0.12, pad: 0.03, slack: 0.15, move_k: 2.0, stop_tau: 0.35, stop_bleed: 0.1, brace_feet: 0.5, bare_device: 0.16, plant_at: 1.2, flank_at: 0.3, ring_px: 120.0 }
}

pub fn page_friends() -> FriendsK {
    crate::knobs::FriendsK { no_snap: true, low_ready: true, reach_m: 8.0, past_m: 0.3, clear_m: 0.35, lead_s: 0.3, lead_max_m: 0.4, up_secs: 0.25, quiet_s: 5.0 }
}

pub fn page_teamtally() -> TeamtallyK {
    crate::knobs::TeamtallyK { on: true }
}

pub fn page_friendlyfire() -> FriendlyfireK {
    crate::knobs::FriendlyfireK { on: true, cone_k: 0.75, body_m: 0.34, sure_m: 0.2, care: true, words: true, near_m: 0.6 }
}

pub fn page_lightswitch() -> LightswitchK {
    crate::knobs::LightswitchK { on: true, reach: 1.15, height: 1.2, beside: alloc::vec![0.85, 1.1, 0.6, 1.5, 2.0, 2.6], stand_m: 0.5, per_room: 2.0, apart_m: 4.0, scan_m: 4.0, strike: true, guards: true, guard_secs: 1.5, hand_secs: 0.3 }
}

pub fn page_carlamps() -> CarlampsK {
    crate::knobs::CarlampsK { on: true, dark: [0.035, 0.035, 0.04], fit: 0.004, tone: 0.004, twin: 0.03 }
}

pub fn page_hitstuff() -> HitstuffK {
    crate::knobs::HitstuffK { on: true }
}

pub fn page_markcut() -> MarkcutK {
    crate::knobs::MarkcutK { on: true }
}

pub fn page_marksit() -> MarksitK {
    crate::knobs::MarksitK {  }
}

pub fn page_coatfinish() -> CoatfinishK {
    crate::knobs::CoatfinishK { on: true }
}

pub fn page_solidglass() -> SolidglassK {
    crate::knobs::SolidglassK { on: true, lum: 0.02, rough: 0.05 }
}

pub fn page_cleanrestart() -> CleanrestartK {
    crate::knobs::CleanrestartK { on: true, picture: true }
}

pub fn page_fireline() -> FirelineK {
    crate::knobs::FirelineK { on: true, fire_s: 1.0, reload_s: 0.7, pace_s: 2.5, pace_k: 1.5, lapse_s: 2.5, spread: 0.06, clear_m: 0.25, short_m: 1.4, keep_m: 0.4, gap_m: 1.0, look_m: 6.0, cue: true, cue_s: 0.35, max_s: 4.0, again_s: 2.0, way_more_m: 10.0, berth_m: 0.4, edge_m: 0.15, reach_m: 40.0 }
}

pub fn page_savetimeline() -> SavetimelineK {
    crate::knobs::SavetimelineK { on: true, manual: 5.0, auto: 5.0, holds: true, turn: true, since: true, base: true, pic_px: 256.0, pic_q: 0.6 }
}

pub fn page_handslot() -> HandslotK {
    crate::knobs::HandslotK { on: true }
}

pub fn page_handring() -> HandringK {
    crate::knobs::HandringK { on: true, hold_secs: 0.3, early: true, empty: true, near: true, lone: true, lone_wrap: true, on_thing: true, bare: true, turn: true, turn_max: 0.45, corner: true }
}

pub fn page_throwback() -> ThrowbackK {
    crate::knobs::ThrowbackK { on: true, rest_mark: true, reach: true }
}

pub fn page_ownblow() -> OwnblowK {
    crate::knobs::OwnblowK { on: true }
}

pub fn page_sprintdoor() -> SprintdoorK {
    crate::knobs::SprintdoorK { on: true, keep: 0.75, bash_vel: 9.0, lean: 0.45, lean_secs: 0.25, rip_pace: 0.3, rip_secs: 0.3, rip_vel: 6.0, rip_level: 0.55, kick: true, kick_k: 0.25, kick_into: 2.5, kick_free: 0.8 }
}

pub fn page_teampace() -> TeampaceK {
    crate::knobs::TeampaceK { on: true, k: 0.8, follow_k: 1.3, follow: true, behind_m: 1.0, fight_secs: 2.0, back_secs: 6.0, own: true, dash_m: 2.5, dash_seen: 0.5, dash_every: 0.25, dash_hot: 6.0, door_brisk: true, pacing: true, pace_near: 12.0, pace_gap: 1.0, pace_cos: 0.5, walk_aim: true, walk_aim_off: 0.25, walk_aim_on: 0.5, run_at: 2.7, accel: 6.0, step: 1.0, gap: 1.0 }
}

pub fn page_noshove() -> NoshoveK {
    crate::knobs::NoshoveK { on: true, round: true, own: true, through: true, ring: 0.6, back: 1.2 }
}

pub fn page_rails() -> RailsK {
    crate::knobs::RailsK { on: true, pan_speed: 11.0, pan_reach: 16.0, snap_px: 40.0, keep: 0.21, keep_speed: 9.0, keep_deep: 0.79, keep_up: 0.39, way_m: 6.0, way_run: true, after_secs: 8.0, after_max: 45.0, shift_tap: true, line_clear: true, mark_clear: true, eye_first: true, eye_view: true, card_beat: true, card_beat_secs: 1.2, card_quiet: 2.0, card_max: 9.0, spare_down: true, flat_ring: true, spare_secs: 6.0 }
}

pub fn page_doorforce() -> DoorforceK {
    crate::knobs::DoorforceK { on: true, shoot: true, kick: true, sure: 1.0, keep: 0.3, fade_secs: 5.0, stale_secs: 6.0, gain: 4.0, hear_min: 0.03, shot: 1.0, handle: 0.6, seen: 1.5, shut_carry: 0.5, behind_m: 2.5, reach_m: 9.0, seen_secs: 3.0, heard_err_m: 0.4, chest_y: 1.15, near_leaf_m: 1.2, past_leaf_m: 0.6, rounds: 4.0, gap_secs: 1.5, kick_secs: 4.0, kick_secs_no_shot: 2.0, walk_secs: 6.0, rest_secs: 8.0 }
}

pub fn page_slotsetup() -> SlotsetupK {
    crate::knobs::SlotsetupK { on: true, pop: true, secs: 1.5, stay: false, draw: true, settle: 0.35, mags: true }
}

pub fn page_glassreal() -> GlassrealK {
    crate::knobs::GlassrealK { drop: true, place: true, hang: 0.1, blocks: 48.0, low_m: 0.7, door_m: 0.3, door_foot_m: 1.5 }
}

pub fn page_quietfog() -> QuietfogK {
    crate::knobs::QuietfogK { on: true, def: crate::quietfog::QuietRung::Team, wand: true, solo: true }
}

pub fn page_told() -> ToldK {
    crate::knobs::ToldK { on: true, rung: crate::told::ToldRung::Standard, guided: 10.0, standard: 4.0, fog_off: true }
}

pub fn page_words() -> WordsK {
    crate::knobs::WordsK { on: true, doors: true, radio_home: true, guide: true }
}

pub fn page_kitanim() -> KitanimK {
    crate::knobs::KitanimK { on: true, equip: true, reload: true, close: true, grips: true, sling: 0.0, sling_up: 18.0, sling_down: 4.0, nv: true, early: 0.05, click_keep: 0.35, throw_keep: 1.0, plain_lock: 0.45, beats: crate::knobs::KitanimKBeats { pistol: crate::knobs::KitanimKBeatsPistol { ref_: 1.6, empty: crate::kitanim::BeatRow { dur: 1.6, out: 0.25, in_: 0.95, rack: Some(1.3), keep: None }, dry: crate::kitanim::BeatRow { dur: 1.45, out: 0.25, in_: 0.95, rack: None, keep: None }, tactical: crate::kitanim::BeatRow { dur: 2.0, out: 0.45, in_: 1.5, rack: None, keep: Some(0.95) } }, rifle: crate::knobs::KitanimKBeatsRifle { ref_: 2.4, empty: crate::kitanim::BeatRow { dur: 2.4, out: 0.4, in_: 1.45, rack: Some(1.95), keep: None }, dry: crate::kitanim::BeatRow { dur: 1.95, out: 0.4, in_: 1.45, rack: None, keep: None }, tactical: crate::kitanim::BeatRow { dur: 2.75, out: 0.55, in_: 2.1, rack: None, keep: Some(1.2) } } }, close_near: 1.1 }
}

pub fn page_kit() -> KitK {
    crate::knobs::KitK { stow: { let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, 0.3); t.set(crate::character::KitItem::Rifle, 0.54); t.set(crate::character::KitItem::Can, 0.26); t.set(crate::character::KitItem::Knife, 0.34); t }, reach: { let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, 0.17); t.set(crate::character::KitItem::Rifle, 0.28); t.set(crate::character::KitItem::Can, 0.16); t.set(crate::character::KitItem::Knife, 0.2); t }, draw: { let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, 0.26); t.set(crate::character::KitItem::Rifle, 0.43); t.set(crate::character::KitItem::Can, 0.2); t.set(crate::character::KitItem::Knife, 0.2); t }, away: 0.22, swap_k: 0.8, far: 0.28, apex_at: 0.85, sling_at: 0.55, join: 0.35, let_go: 0.3, apex: crate::knobs::KitKApex { at: [-0.03, 0.1, -0.19], fing: [0.35, -0.75, -0.55], back: [0.0, -0.2, -1.0], pole: [-1.0, 0.25, 0.0] }, elbow_min: 44.0, wrist_knee: 46.0, wrist_max: 66.0, roll_in: 0.5, over: [-0.03, 0.34, 0.22], up: [-0.14, 0.22, 0.14], top: crate::knobs::KitKTop { fing: [0.0, 1.0, 0.25], back: [-0.8, 0.0, 0.6] }, trunk: 6.0, belt: crate::knobs::KitKBelt { at: [-0.15, 0.04, 0.13], fing: [0.25, -0.9, 0.2], back: [-0.5, 0.0, 0.85], pole: [-1.0, -0.1, -0.5] }, holster_pole: [-0.7, 0.15, -1.0], sheath_pole: 0.9, rest: [0.03, -0.36, 0.2], arc: { let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, [-0.05, 0.0, 0.05]); t.set(crate::character::KitItem::Rifle, [-0.1, 0.2, 0.0]); t.set(crate::character::KitItem::Can, [-0.03, 0.0, 0.05]); t.set(crate::character::KitItem::Knife, [0.0, 0.0, 0.05]); t }, cross: [-0.16, 0.0, 0.1], front: crate::knobs::KitKFront { stow: 0.36, reach: 0.2, draw: 0.32, hand_at: 0.8, sling_at: 0.45, arc: [0.0, 0.02, 0.07], pole: [-0.75, -0.55, -0.4] }, nv: crate::knobs::KitKNv { hand: crate::knobs::KitKNvHand { dur: 0.92, reach: 0.38, out: 0.36, flip: [0.38, 0.52], up: [0.084, 0.178, 0.1], down: [0.086, 0.08, 0.12], fingers: [-0.75, 0.6, 0.1], back: [0.55, 0.1, 0.83], pole: [0.7, -0.6, 0.35], shrug: 8.0, curl: [0.45, 0.5, 0.55, 0.6], thumb: 0.35, rate: 4.2, away: 2.6, clear: 6.0 }, nod: crate::knobs::KitKNvNod { dur: 0.36, pre: 0.07, peak: 0.18, flip: [0.12, 0.22], on: [-5.0, 19.0], off: [7.0, -15.0], carry: 0.12 } }, fist: alloc::vec![0.62, 0.68, 0.72, 0.76], open: alloc::vec![0.26, 0.3, 0.34, 0.38], mag: crate::knobs::KitKMag { well: [0.085, -0.125, -0.005], strip: [0.09, -0.21, -0.02], slide: [0.05, 0.07, -0.02], pull: [0.05, 0.07, -0.085], pouch: [0.17, 0.03, 0.12], dump: [0.21, 0.0, -0.13], via: [0.31, 0.14, 0.1], pole: [0.75, -0.55, -0.3], pole_w: 0.7, wf: [-0.75, 0.55, 0.3], wb: [0.3, -0.75, 0.55], sf: [-1.0, -0.15, 0.1], sb: [0.0, 1.0, 0.0] }, close: crate::knobs::KitKClose { far: 0.4, hover: 0.045, bow: [-0.02, 0.01, 0.035], lead: [0.0, 0.0, 0.0], left: 0.3, left_at: 0.85, left_in: 0.24, left_out: 0.2, rate: 7.0, flex: 0.55, flex_amp: 0.1, drift: 0.008, regrip: 1.7, curl: 0.5 } }
}

pub fn page_throw_by() -> ThrowByK {
    crate::knobs::ThrowByK { on: true, toss: 3.0, toss_fast: 7.3, toss_in: 4.0, toss_min: 0.6, through: 0.07, toss_pivot: [-0.07, -0.46, 0.08], full: 10.0, turn_min: 0.4, arm_min: 0.5, off_min: 0.45, sit: 0.6, pivot: [-0.2, 0.06, 0.18] }
}

pub fn page_hands() -> HandsK {
    crate::knobs::HandsK { on: true, body: true, door: true, reach_secs: 0.3, lift_secs: 0.35, set_secs: 0.3, grip_m: 0.55, step_secs: 1.0, head_bias: 0.1, ahead: 0.35, ahead_head: 0.48, carry_feet: 0.42, carry_head: 0.46, slack_m: 0.4, carry: true, hoist_secs: 1.0, lower_secs: 1.1, carry_step_secs: 2.4, carry_near_m: 0.35, carry_side: 0.2, carry_fwd: -0.02, carry_up: 1.56, carry_back: 1.35, carry_legs: 1.1, carry_pace: 0.6, carry_turn: 5.0, carry_ride: 0.85, lay_ahead: 0.62, door_reach: 0.12, hand_secs: 0.3, door_rate: 2.6, open_to: 1.5, clear_m: 0.06, step_m: 1.3, round_m: 1.8, hand_m: 0.5, in_m: 0.75, stall_secs: 0.5, aim_arc: 1.2, team: true, team_pre_m: 0.55, team_reach_m: 0.95, team_pace: 0.9, team_vel: 0.25 }
}

pub fn page_work() -> WorkK {
    crate::knobs::WorkK { on: true, pick_m: 0.45, pick_wide_m: 0.9, behind_m: 0.5, behind_panel_m: 0.09, stand_m: 0.5, buddy_m: 1.4, hand_secs: 0.45, breaker_secs: 0.7, fade_secs: 0.4, hand_m: 0.85, posted: true, own_face: true, held_pass: true, off_walk: true, walk_m: 2.5, sam_face: true }
}

pub fn page_wand() -> WandK {
    crate::knobs::WandK { on: true, feed_secs: 1.5, stow_secs: 0.6, fov_deg: 90.0, blind_deg: 10.0, pan_rate: 1.6, eye_y: 0.1, past_m: 0.22, mark_secs: 15.0, shut_rad: 0.05, give_up_secs: 2.5, nv: true, mate: true, order_key: String::from("KeyV"), sweep_secs: 5.0, quick: true, quick_secs: 0.4, quick_down: 0.9 }
}

pub fn page_sticky() -> StickyK {
    crate::knobs::StickyK { on: true, carry: 3.0, max: 3.0, live: 1.0, issue: 2.0, speed: 26.0, gravity: 9.81, drag: 0.5, radius: 0.03, life: 1.6, substeps: 4.0, solve_iters: 2.0, cd: 1.2, aim_y: 1.9, fov_deg: 110.0, off: 0.06, flat_y: 0.7, launch_noise: 0.3, stick_noise: 0.12, take: true, take_reach: 1.3, take_noise: 0.1, mark: true }
}

pub fn page_peeksight() -> PeeksightK {
    crate::knobs::PeeksightK { on: true, out: 0.3, off: 0.1, turn_deg: 90.0 }
}

pub fn page_owncharge() -> OwnchargeK {
    crate::knobs::OwnchargeK { on: true }
}

pub fn page_gunslots() -> GunslotsK {
    crate::knobs::GunslotsK { on: true, dry: true, near: 10.0, out: 0.7, lull: 1.6, back: 0.65, heard: true, bare: true }
}

pub fn page_gunlook() -> GunlookK {
    crate::knobs::GunlookK { on: true, shells: true }
}

pub fn page_gunroles() -> GunrolesK {
    crate::knobs::GunrolesK { on: true, deal: true, akms: 0.5, belt_lane: 15.0, belt_min: 5.0, belt_squad: 10.0, line_max: 40.0, pistol_near: 4.0, pistol_share: 0.5, pistol_max: 0.25, captor: false, use_: true, entry: true, lanes: true, strings: true, post: true, lane_k: 0.6, file_cost: 400.0 }
}

pub fn page_kittree() -> KittreeK {
    crate::knobs::KittreeK { on: true, rings: 5.0, lend: 5.0 }
}

pub fn page_squadthrow() -> SquadthrowK {
    crate::knobs::SquadthrowK { on: true, frag_roll_k: 0.04, again_secs: 1.2, shown_secs: 2.5, slack_m: 1.0, walk: true, walk_m: 12.0, min_m: 1.5, max_m: 11.5, tries: 12.0, keep_secs: 0.4, walk_secs: 16.0, turn_secs: 1.2, toss_secs: 0.5, back: true, draw: true, lob: true, lob_paces: alloc::vec![7.5, 10.0, 5.0], lob_bank_paces: alloc::vec![10.0, 7.5], lob_speed: 11.0, lob_near_m: 1.5, lob_up_m: 1.2, lob_walls: 12.0, lob_wall_m: 3.0, lob_wall_h: 0.9, lob_sims: 48.0, lob_cells: 10.0, lob_keep_secs: 1.0, settle_secs: 0.5, lob_steps: alloc::vec![0.008333333333333333, 0.025], sam_call: true, duck: true, duck_secs: 0.5, duck_throw_secs: 0.2, duck_cells: 16.0, duck_apart_m: 1.0, duck_min_m: 5.0 }
}

pub fn page_squad40() -> Squad40K {
    crate::knobs::Squad40K { on: true, near_m: 1.2, arm_slack: 0.5, reach_slack: 0.5, muzzle_ahead: 0.55, muzzle_up: 1.35, head_drop: 0.3, aim_rad: 0.2, aim_secs: 2.0, raise_secs: 0.35, after_secs: 0.4, shown_secs: 2.5, draw: true }
}

pub fn page_autopilot() -> AutopilotK {
    crate::knobs::AutopilotK { on: true, show_secs: 4.0, keep_bangs: 1.0, contact_m: 9.0, replan: true, plain: true, restack: true }
}

pub fn page_planlay() -> PlanlayK {
    crate::knobs::PlanlayK { on: true, keep: true, sticky: true }
}

pub fn page_cmdring() -> CmdringK {
    crate::knobs::CmdringK { on: true, key: String::from("KeyB"), wedges: 5.0, flick_px: 24.0, hold_secs: 0.2, stay: true, stay_hold_secs: 0.3, stay_digit: true, pin: true, tap_now: true, tap_secs: 0.15, edge_x: 150.0, edge_y: 100.0, man_px: 24.0, ref_: [1920.0, 1080.0], hint: false, hands: false, flat: true, plain_hint: true, bare_hint: true, wheel_idle: true, whole_squad: true, goals: true, goal_ring: true, goal_half: true, twice: true, twice_secs: 0.4, twice_px: 24.0, only: true, hint_wall_m: 0.6, hint_spot_m: 0.3, hint_man_m: 0.5 }
}

pub fn page_torch() -> TorchK {
    crate::knobs::TorchK { calm_off: true, lit_at: 0.2, off_k: 1.2, spill_m: 3.0, day_m: 12.0, ahead_m: 5.0, on_secs: 0.3, off_secs: 2.0, hold_secs: 1.0, ask_secs: 0.25, calm_aw: 0.3, wall_low: true, wall_dark: false, wall_m: 1.2, wall_back_m: 1.6, low_tan: 2.2, low_rate: 6.0 }
}

pub fn page_backkey() -> BackkeyK {
    crate::knobs::BackkeyK { tab: true }
}

pub fn page_fixedstep() -> FixedstepK {
    crate::knobs::FixedstepK {  }
}

pub fn page_tween() -> TweenK {
    crate::knobs::TweenK {  }
}

pub fn page_savepic() -> SavepicK {
    crate::knobs::SavepicK {  }
}

pub fn page_evenpace() -> EvenpaceK {
    crate::knobs::EvenpaceK {  }
}

pub fn page_findtuned() -> FindtunedK {
    crate::knobs::FindtunedK {  }
}

pub fn page_autocam() -> AutocamK {
    crate::knobs::AutocamK {  }
}

pub fn page_morale() -> MoraleK {
    crate::knobs::MoraleK { on: true, half_life: 25.0, ask_secs: 0.25, alone: 0.25, alone_m: 7.0, dark: 0.25, lit_asks: 3.0, blast: 0.35, shock_cap: 0.5, blast_m: 10.0, near_m: 4.0, far_k: 0.6, jumpy: 0.3, broken: 0.7, hyst: 0.15, duck_secs: 2.5, down_secs: 0.6, back_until: 2.2, back_m: 3.5, back_speed: 2.0, burst_rounds: 5.0, burst_every: 0.2, spread: 0.3, call2_secs: 6.0, bait_m: 2.0, bait_short: 5.0, bait_every: 0.1, reload_secs: 3.2, bark_secs: 2.0, fan_deg: 70.0, fan_step: 17.5, straight_m: 1.5, training: crate::morale::Training::Difficulty, green_broken: 0.5, leader_down: 0.25, leader_shock: 0.3, flinch_secs: 0.8, shout_gap: 0.6, order_m: 4.0, order_beat: 0.5, order_secs: 1.0, order_gap: 0.15, stand_secs: 2.5, side_m: 0.75, steady_duck: 1.0, hold_shock: 0.05, hold_max: 70.0, nerve: crate::morale::Nerve::Normal, nerve_k: crate::knobs::MoraleKNerveK { brittle: 1.4, normal: 1.0, steady: 0.7 }, dark_read: crate::knobs::MoraleKDarkRead::Place }
}

pub fn page_condition() -> ConditionK {
    crate::knobs::ConditionK { on: true, hurt_pace: 1.5, hurt_sup: 0.35, hurt_secs: 0.0, own_calm: true, looks: true }
}

pub fn page_inhand() -> InhandK {
    crate::knobs::InhandK { on: true, from: 0.3, to: 0.92, wrist: 55.0, both: 0.34, rate: 9.0, fit: true, torch: true, long: true, holster_out: 0.03, belt: true }
}

pub fn page_packside() -> PacksideK {
    crate::knobs::PacksideK { on: true, ram: true, gap: 0.012, handle: -0.258, gun: -0.24, over: 0.012, head_min: 1.5, butt: 1.56, rest: 0.012, pitch0: -0.1, pitch1: 1.3 }
}

pub fn page_gunhand() -> GunhandK {
    crate::knobs::GunhandK { on: true, whistle: true, throw_free: true, long_reload: true }
}

pub fn page_runner() -> RunnerK {
    crate::knobs::RunnerK { on: true, close_m: 5.0, out_secs: 0.8, beat_secs: 0.6, tell_m: 4.0, told_m: 8.0, tell_beat: 0.8, tell_aware: 0.6, shy_m: 2.0, mate_m: 20.0, mate_secs: 10.0, clear_m: 1.5, edge_m: 2.0, corner_m: 25.0, gone_secs: 2.0, stall_secs: 3.0, retry_secs: 2.0, retries: 3.0, replan_secs: 0.5, ahead_m: 3.0, ask_cells: 60.0, ask_wide: 240.0, anyway: true, nobody: true }
}

pub fn page_startle() -> StartleK {
    crate::knobs::StartleK { on: true, beat: 0.5, draw: 0.3, fresh: 1.0, late: 0.25, run: true, run_m: 8.0, near_m: 2.5, clear_secs: 1.5, run_secs: 4.0, stuck_secs: 0.75, stuck_m: 0.3 }
}

pub fn page_rowset() -> RowsetK {
    crate::knobs::RowsetK { on: true }
}

pub fn page_smokebreak() -> SmokebreakK {
    crate::knobs::SmokebreakK { on: true, light_secs: 3.2, light_up: [0.5, 2.6], flame_at: 1.3, flame: 0.9, every: 7.5, draw: 2.1, out_after: 0.7, out: 1.3, flame_color: [1.0, 0.62, 0.24], flame_k: 2.4, flame_range: 3.2, ember_color: [1.0, 0.36, 0.1], ember_k: 0.1, ember_draw_k: 0.7, ember_range: 1.3, ember_every: 0.4, tip_out: 0.11, tip_up: 0.02, thread: crate::knobs::SmokebreakKThread { radius: 0.02, cells: 1.2, density: 5.0, temp: 5.0, speed: 0.25, push: 10.0, jitter: 0.35 }, thread_gone: 30.0, breath: crate::knobs::SmokebreakKBreath { radius: 0.04, cells: 1.5, density: 16.0, temp: 2.5, speed: 1.1, push: 12.0, expand: 2.2, attack: 0.1, jitter: 0.3, down: 0.1 }, puffs: crate::knobs::SmokebreakKPuffs { records: 4.0, count: 6.0, speed: 0.8, spread: 0.26, life: 2.4, size: 0.1, opacity: 0.8 }, color: [0.72, 0.76, 0.86], mouth_ahead: 0.1, mouth_down: 0.075 }
}

pub fn page_showwhy() -> ShowwhyK {
    crate::knobs::ShowwhyK { on: true, line: true, stack: true, door: true, hurt: true, keep: true, say: true, line_after: 0.6, line_gap: 0.3, wave_every: 2.6, wave_later: 6.0, line_min: 1.2, line_say_gap: 4.0, mate_half: 0.45, squeeze_gap: 0.35, squeeze_reach: 1.15, fist_after: 0.45, hold_every: 8.0, hold_max: 3.0, door_after: 0.5, door_every: 4.0, after_you_gap: 10.0, hurt_late: 6.0, lag_m: 6.0, lag_every: 20.0, lag_max: 2.0, keep_after: 0.4, keep_gap: 1.5, keep_every: 30.0, owed_for: 5.0 }
}

pub fn page_fire() -> FireK {
    crate::knobs::FireK { on: true, names: alloc::vec![String::from("brazier")], height: 1.15, mouth: 1.15, coals: 0.26, coal_swing: 0.6, bed_r: 1.42, depth: 0.4, rates: alloc::vec![0.43, 1.3, 2.7, 4.9, 8.3], weights: alloc::vec![0.8, 0.9, 0.7, 0.45, 0.2], least: 0.6, most: 1.6, wander: 0.14, wander_up: 0.1, lift: 0.5, kick: 0.6, kick_secs: 0.12, bright: 1.25, warm: alloc::vec![1.0, 0.72, 0.51], reach: 1.4, hardy: true, stir_gap: 0.3, size: 1.1, pop_rate: 1.1, pop_slot: 0.2, embers: 60.0, ember_life: 1.7, ember_speed: 1.1, ember_size: 0.004, ember_gain: 0.45, ember_crack: 10.0, ember_r: 30.0, look: crate::knobs::FireKLook { gain: 3.0, core: 4.0, sigma: 2.5, speed: 0.7, tall: 0.95, torn: 1.1, plume: 1.0, shimmer: 0.019, soot: 0.5, breathe: 1.0, react: 1.0, lean: 0.2, steps: 40.0 }, smoke: crate::knobs::FireKSmoke { on: true, up: 1.9, radius: 0.26, cells: 4.0, density: 1.0, temp: 21.0, speed: 0.9, push: 6.0, jitter: 0.5, every: 1.0, view_r: 18.0, lean: 0.12 }, gas: crate::knobs::FireKGas { on: false, up: 0.1, radius: 0.2, cells: 6.0, density: 0.3, temp: 22.0, speed: 1.8, push: 8.0, jitter: 0.6, every: 0.5, view_r: 12.0, keep: 0.8, ref_: 3.2, lo: 0.2, ease: 1.5 }, bed: 0.85, crack: 0.9, bed_secs: 12.0, fog: true, max: 4.0, draw_r: 70.0, wind_max: 3.0 }
}

pub fn page_flinch() -> FlinchK {
    crate::knobs::FlinchK { on: true, least: 0.06, full: 0.35, far: 0.45, min_travel: 1.0, steady_k: 0.65, friendly: false, run: true, break_secs: 2.5, hot_secs: 1.0, ease_k: 0.5, ask_secs: 0.25 }
}

pub fn page_darkdoor() -> DarkdoorK {
    crate::knobs::DarkdoorK { on: true, who: crate::knobs::DarkdoorKWho::Rest, stop_m: 2.5, pair_m: 4.0, dark_at: 0.2, dark_share: 0.7, lit_by: 0.1, samples: 40.0, ask_secs: 0.5, open_rad: 0.35, arm_secs: 3.0, stand_m: 1.3, side_m: 0.3, sweep_secs: 3.2, ear_m: 12.0, mate_m: 40.0, mate_try: 3.0, call_secs: 2.0, copy_secs: 1.2, lag_secs: 5.0, with_secs: 10.0, keep_aw: 0.3, keep_secs: 2.0, resume_secs: 12.0, seen_secs: 2.0, wait_secs: 20.0, again_secs: 45.0, fed_up_secs: 300.0, fed_up_n: 2.0, nerve_secs: 1.5, slow_k: 0.5 }
}

pub fn page_darkfoe() -> DarkfoeK {
    crate::knobs::DarkfoeK { on: true, carry: 1.0, lit_at: 0.3, look_m: 12.0, step_m: 2.0, fan_rad: 0.3, see_share: 0.34, ask_secs: 0.25, on_secs: 0.6, on_jitter: 2.5, off_secs: 15.0, off_jitter: 0.4, hold_secs: 2.0, by_lamp: true, lamp_m: 16.0, suspect: false, seek: true, blind_secs: 3.0, seek_m: 10.0, walk_m: 12.0, edge_m: 3.0, stumble_k: 0.75, again_secs: 20.0, moves: 2.0, quiet_secs: 4.0, calm_secs: 6.0, roam: false, edge: true }
}

pub fn page_powerout() -> PoweroutK {
    crate::knobs::PoweroutK { on: true, see_m: 14.0, think: [3.0, 6.0], beat: 2.0, look_m: 12.0, about_m: 4.0, step_m: 2.0, fan_rad: 0.3, lit_at: 0.3, drop_k: 0.5, dark_after: 0.3, men: 2.0, max_m: 60.0, lone: true, stayers: true, parties: 1.0, look_secs: 1.5, tries: 8.0, lever_secs: 8.0, overdue: 15.0, ask: 6.0, cover_m: [1.6, 3.2] }
}

pub fn page_squaddeal() -> SquaddealK {
    crate::knobs::SquaddealK { on: true, one: 5.0, two: 8.0, names: alloc::vec![String::from("Alpha"), String::from("Bravo"), String::from("Charlie")] }
}

pub fn page_stayask() -> StayaskK {
    crate::knobs::StayaskK { on: true, men: 1.0 }
}

pub fn page_rescuecheck() -> RescuecheckK {
    crate::knobs::RescuecheckK { on: true, first: [14.0, 22.0], every: [24.0, 36.0], ask_secs: 4.0, go_secs: 4.0, wait_secs: 5.0, tries: 4.0, feet_m: 2.5, word_secs: 2.5 }
}

pub fn page_kitload() -> KitloadK {
    crate::knobs::KitloadK { on: true, gun_kg: 0.035, gun_bulk: [0.04, 0.17], small: { let mut t = JsMap::new(); t.set(String::from("smoke"), [0.1, 0.02]); t.set(String::from("bangs"), [0.07, 0.02]); t.set(String::from("frags"), [0.08, 0.02]); t.set(String::from("shells"), [0.07, 0.01]); t.set(String::from("charges"), [0.1, 0.02]); t.set(String::from("wand"), [0.07, 0.02]); t.set(String::from("cams"), [0.04, 0.01]); t }, big: { let mut t = JsMap::new(); t.set(String::from("shotgun"), [0.18, 0.1]); t.set(String::from("hammer"), [0.3, 0.1]); t.set(String::from("wall"), [0.24, 0.06]); t }, pace: 0.4, seen: 0.6, heard: 0.7, sprint: 0.5, pace_top: 2.4 }
}

pub fn page_rescueend() -> RescueendK {
    crate::knobs::RescueendK { on: true, out_m: 3.5, clear_secs: 1.5, marker_m: 2.4, reach_m: 2.2, gap_m: 1.3, gap_step_m: 0.7 }
}

pub fn page_rescueflush() -> RescueflushK {
    crate::knobs::RescueflushK { on: true, after_secs: 4.0, gap_secs: 2.5, again_secs: 8.0, reach_m: 10.0, kill_secs: 3.0, busy_secs: 2.0, smoke_k: 0.3, at_door_m: 1.8, lanes_choose: true }
}

pub fn page_snatchout() -> SnatchoutK {
    crate::knobs::SnatchoutK { on: true, hold: true, marker_m: 2.4, reach_m: 2.2, gap_m: 1.3, send_m: 3.5, send_him_m: 8.0, in_m: 1.1, carry_m: 2.2, trail_m: 12.0, close_m: 5.0, turn_secs: 4.0, lane_m: 1.0, off_m: 0.7, spot_m: 3.4 }
}

pub fn page_posemath() -> PosemathK {
    crate::knobs::PosemathK { singles: true }
}

pub fn page_sense() -> SenseK {
    crate::knobs::SenseK { cpu: false, grid: false, bounce: false, drift: false, moon: false, glow: crate::knobs::SenseKGlow { albedo: 0.3, share: 0.1, passes: 2.0, sweep: 32.0 }, smoke: crate::knobs::SenseKSmoke { sigma: 6.0, decay: 0.14, spread: 0.2, widest: 3.5, rise: 1.0, rise_rate: 0.3, in_wind: 0.15, least: 0.05, gone: 0.02, cap: 48.0, emitters: 64.0, reserve: 12.0 } }
}

pub fn page_cmdcheck() -> CmdcheckK {
    crate::knobs::CmdcheckK { on: true, keys: 24.0, key_len: 24.0, span: 16384.0, wheel: 4096.0, far: 10000.0, dist_min: 0.05, dist_max: 2000.0, fov_min: 0.02, fov_max: 3.0, aspect_min: 0.05, aspect_max: 20.0, reach: 80.0, steep: 1.5 }
}

pub fn page_seats() -> SeatsK {
    crate::knobs::SeatsK { max: 2.0, apart: 0.9, keep: 2.0 }
}

pub fn page_doze() -> DozeK {
    crate::knobs::DozeK { on: true, min_wait: 20.0, max_wait: 90.0, chair_m: 4.5, nod_secs: 2.5, rise_secs: 2.0, hear_k: 0.6, light_at: 0.3, light_secs: 0.8, ask_secs: 0.25, wake_aw: 0.55, bump_aw: 0.7, stir_secs: 1.3, breath_secs: 3.6, screen: true, screen_m: 1.5, pull_m: 0.5, nap_wait: 40.0, settle_secs: 1.2, ears: 0.5, noises: 0.6, say_first: alloc::vec![5.0, 11.0], say_every: alloc::vec![16.0, 30.0], idle_secs: 60.0, lend: true }
}

pub fn page_netquiet() -> NetquietK {
    crate::knobs::NetquietK { on: true, unanswered: 0.12, heard_it: 0.06, body: 0.2, body_told: 0.08, fall_seen: 0.08, fall_unseen: 0.22, steady_k: 0.6, ask_beat: 1.4, body_beat: 2.0, bark_secs: 3.0, say_hold: 4.0, shooter_secs: 1.0, body_m: 10.0, whip_shift: 0.12, whip_secs: 1.5, whip_hold: 0.8, whip_turn: 10.0, whip_turn_broken: 14.0, whip_m: 15.0, whip_near_m: 4.0, last_beat: 1.5, calm_secs: 2.0, alert_secs: 15.0, give_up: 8.0, mate_m: 5.0, arc_deg: 40.0, eye_deg: 58.0, eye_slack: 4.0, crouch: 0.4, wary: 0.35, back_speed: 2.0, max_m: 9.0, door_m: 2.0, eye_m: 1.3, stay_m: 1.0 }
}

pub fn page_attention() -> AttentionK {
    crate::knobs::AttentionK { on: true, focus_deg: 25.0, side_k: 0.3, ear_k: 0.5, break_gain: 0.2, calm_aw: 0.6, draw_aw: 0.34, floor: 0.32, whistle_secs: 4.0, noise_secs: 5.0, door_secs: 4.0, light_secs: 5.0, tell_secs: 0.8, dwell_sweep: 0.3, walk_cap: 15.0, wear_k: 0.6, memory_secs: 60.0, spent: 3.0, same_secs: 2.5, lent_back_secs: 6.0, marks: true, ring_m: 0.42, way: true, way_reask: 0.5 }
}

pub fn page_picture() -> PictureK {
    crate::knobs::PictureK { on: true, look_secs: 0.2, glimpse: 0.05, tell_secs: 1.0, shout_m: 16.0, pop_m: 7.0, made_out: 0.2, made_out_m: 3.0, fresh_secs: 2.0, stale_secs: 20.0, stale_k: 0.4, forget_secs: 90.0, lost_k: 0.5, walk_mps: 1.2, walk_max: 6.0, heard_m: 2.0, maybe_k: 0.6, reach_m: 30.0, eye_h: 1.6, chest_h: 1.15, low_h: 0.7, head_up: 0.45, standoff: true, cover: true, wall: true, flee: true, flee_cost: 8.0, fallback: true, stack: true, stack_k: 0.5, step: true, step_at: 0.5, step_free: 0.25, step_m: 2.5, step_after: 0.6, step_rest: 1.5, step_secs: 5.0, shot: true, run: true, cover_hard: true, show: crate::knobs::PictureKShow::Off, show_m: 9.0 }
}

pub fn page_response() -> ResponseK {
    crate::knobs::ResponseK { on: true, deal: true, net: true, clear_m: 12.0, eta_step: 5.0, word_secs: 0.5, raid: true, call_secs: 6.0 }
}

pub fn page_handwork() -> HandworkK {
    crate::knobs::HandworkK { on: true, work: true, radio: true, swap: true, stand_m: 0.52, step_rate: 7.0, step_pace: 1.7, stand_side: 0.1, wait_secs: 0.7, lever_secs: 0.5, lever_at: 0.3, stow_secs: 0.45, radio_k: 0.6, radio_min: 0.9, radio_max: 2.4 }
}

pub fn page_workstow() -> WorkstowK {
    crate::knobs::WorkstowK { on: true, mate_down: 0.3, mate_up: 0.45, show: 0.6, down_max: 1.6, sam_up: 0.3, fight_cap: 8.0 }
}

pub fn page_squadnet() -> SquadnetK {
    crate::knobs::SquadnetK { on: true, why: true, clear: true, end_call: true, near: 0.24, edge: true, edge_in: 0.04, click: 0.3, click_rate: 1.3, down_gain: 0.7, down_rate: 0.55, down_secs: 5.0 }
}

pub fn page_takedown() -> TakedownK {
    crate::knobs::TakedownK { on: true, blow: true, victim: true, fall: true, partner: true, breathe: true, kill: true, choke_beats: true, haul: true, lower: true, partner_hit: 0.3, partner_end: 0.58, breath_hz: 0.24, breath_rise: 0.02, breath_in: 1.2 }
}

pub fn page_stealthpose() -> StealthposeK {
    crate::knobs::StealthposeK { on: true, idle: true, walk: true, rise: true, creep: true, cable: true, nod: true, sway_secs: 2.2, sway_m: 0.012, drift_m: 0.016, rise_secs: 0.9, cable_in: 0.4, cable_bow: 26.0, cable_step: 0.07 }
}

pub fn page_look() -> LookK {
    crate::knobs::LookK { on: true, suit: true }
}

pub fn page_music() -> MusicK {
    crate::knobs::MusicK {  }
}

pub fn page_lived() -> LivedK {
    crate::knobs::LivedK { on: true }
}

pub fn page_underfoot() -> UnderfootK {
    crate::knobs::UnderfootK { on: true }
}

pub fn page_round() -> RoundK {
    crate::knobs::RoundK { on: true }
}

pub fn page_shapes() -> ShapesK {
    crate::knobs::ShapesK { on: true }
}

pub fn page_flash() -> FlashK {
    crate::knobs::FlashK { on: true, secs: [7.0, 4.0], stagger: [2.5, 1.0], lift: 1.2, concuss_m: 6.0, concuss_k: 0.5, own_m: 2.5, own_secs: 1.5, fuse: 0.7 }
}

pub fn page_pace() -> PaceK {
    crate::knobs::PaceK { on: true, max: 2.2, accel: 2.6, turn: 8.0, swing: 30.0 }
}

pub fn page_lights() -> LightsK {
    crate::knobs::LightsK { on: true, after_secs: 2.5, hot_secs: 5.0, step_secs: 4.0, tail: 0.6, on_m: 0.8, alone_m: 7.0, look_dark: false, look_lit: 0.25, look_below: 0.12, look_m: 10.0, lit_off: true, lit_at: 0.5, lit_m: 15.0, lit_mark_m: 6.0, lit_man: true, wall_off: true, wall_m: 1.5, wall_back_m: 1.9, wall_secs: 0.3 }
}

pub fn page_nvg() -> NvgK {
    crate::knobs::NvgK { on: true, lead: true, own: true, hunt_frees: false, quiet_secs: 30.0, lead_tail: 1.5, dark_at: 0.5, black_at: 0.15, black_reads: 2.0, clear_reads: 3.0, albedo: 0.3, noise: 0.046, down_secs: 0.25, up_secs: 0.7, call_gap: 8.0 }
}

pub fn page_ir() -> IrK {
    crate::knobs::IrK { on: true, sam: true, carry: true, team_off: true }
}

pub fn page_irlight() -> IrlightK {
    crate::knobs::IrlightK { on: true, team: true, auto: true, meter: true, show: 0.1, hold: 1.5, power: 8.0, reach: 22.0, cone: 65.0, core: 20.0, dip: 6.0, near: true, near_m: 5.0, near_min: 0.003, near_up: 3.0, near_down: 12.0 }
}

pub fn page_wallpen() -> WallpenK {
    crate::knobs::WallpenK { on: true, base: { let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.1); t.set(crate::wallpen::ThinStuff::Wood, 0.0); t.set(crate::wallpen::ThinStuff::Fabric, 0.03); t.set(crate::wallpen::ThinStuff::Cardboard, 0.02); t.set(crate::wallpen::ThinStuff::Foliage, 0.0); t }, per_m: { let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.5); t.set(crate::wallpen::ThinStuff::Wood, 4.5); t.set(crate::wallpen::ThinStuff::Fabric, 1.0); t.set(crate::wallpen::ThinStuff::Cardboard, 1.0); t.set(crate::wallpen::ThinStuff::Foliage, 0.3); t }, thin_max: { let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.17); t.set(crate::wallpen::ThinStuff::Wood, 0.12); t.set(crate::wallpen::ThinStuff::Fabric, 0.13); t.set(crate::wallpen::ThinStuff::Cardboard, 0.7); t.set(crate::wallpen::ThinStuff::Foliage, 2.0); t }, max_thru: { let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.4); t.set(crate::wallpen::ThinStuff::Wood, 0.14); t.set(crate::wallpen::ThinStuff::Fabric, 0.3); t.set(crate::wallpen::ThinStuff::Cardboard, 0.8); t.set(crate::wallpen::ThinStuff::Foliage, 2.5); t }, min_energy: 0.12, max_things: 5.0, deflect_deg: 4.0, men: true, behind_m: 4.0, thru_rounds: 4.0, thru_rest_secs: 6.0 }
}

pub fn page_laser() -> LaserK {
    crate::knobs::LaserK { enemy: false }
}

pub fn page_doortools() -> DoortoolsK {
    crate::knobs::DoortoolsK { charges: 2.0, bangs: 3.0, blow_key: String::from("KeyF"), word_on_key: true, word_tap: 0.6, thr_hold_first: true, thr_bang_first: true, thr_outside_row: true, thr_leave_fall_in: false, thr_hands_off: false, thr_held_all: false, thr_menu_word: true, stack_cap: 60.0, still_secs: 2.5, kick_cap: 5.0, in_cap: 9.0, shove: 5.5, hold_open: 10.0, toss_after_open: 0.15, bang_in: 2.4, fuse_after_land: 0.35, air_in: 1.6, air_h: 1.2, toss_cap: 1.6, toss_after_kick: 0.05, second_throws: true, throw_reach: 4.0, toss_turn: 0.0, frag_way: true, frag_way_wait: 1.0, frag_way_open: 1.15, frag_way_ajar: 0.6, bang_way: true, open_hand: true, gate_hand: true, last_out: true, last_out_m: 3.0, after_bang: 0.6, quiet: 0.15, entry_cap: 2.8, second: 0.25, sweep_rad: 0.9, sweep_secs: 0.6, corners: true, corner_secs: 0.3, corner_rad: 0.3, corner_cap: 1.6, corner_owed: true, collapse: true, corner_turn: true, lead_seen: true, lead_seen_min: 12.0, lead_seen_secs: 30.0, lead_seen_m: 11.0, sectors: true, sector_secs: 0.6, search: true, scan_rate: 1.4, look_rad: 0.3, look_m: 11.0, dead_cells: 8.0, search_moves: 3.0, search_cap: 24.0, search_cells: 900.0, tuck: 0.4, edge_out: 0.4, bang_file: 1.1, set_secs: 2.0, standoff: alloc::vec![2.6, 2.0, 1.5], knock_m: 2.0, stun_m: 8.0, stun_secs: alloc::vec![6.0, 2.5], stagger_secs: 2.0, backblast_m: 1.6, backblast_secs: 1.5, blow_vel: 12.0, slice_back: 1.0, slice_look: 0.35, slice_edge: 0.12, slice_deep: 3.5, slice_pace: 0.9, word_stack: 20.0, word_in: 6.0, room_back: alloc::vec![2.8, 3.7, 4.6, 5.5], pod_apart: 1.2, thr_pull: true, thr_pull_at: alloc::vec![0.45, 1.8], thr_still: alloc::vec![0.45, 1.2], thr_arc: alloc::vec![alloc::vec![-0.45, 1.2], alloc::vec![0.25, 1.8]], thr_min: 4.0, thr_quiet: 2.0, thr_cap: 25.0, thr_bangs: 2.0, thr_pocket: 12.0, thr_shot_m: 8.0, thr_throw_m: 5.0, thr_flash_m: 7.0, thr_turn: 0.5, thr_next: 0.3, thr_cover: true, thr_slice: false, thr_light: 1.0, thr_in: 4.0, thr_pie: true, thr_pie_arc: alloc::vec![alloc::vec![-0.2, 1.4], alloc::vec![-0.05, 1.6], alloc::vec![0.08, 1.8]], thr_pie_apart: 0.65, thr_lean: true, thr_lean_m: 0.15, thr_knee_in: -0.1, thr_pie_one: true, thr_lean_look: 1.1, thr_doors: true, thr_doors_m: 5.5, thr_doors_lit: true, thr_doors_walk: 16.0, thr_doors_stay: 1.0, other_door_m: 9.0, own_can: true, buddy: true, buddy_m: 5.0, buddy_pair_m: 12.0, buddy_left_m: 8.0, flow_on: true, flow_cells: 28.0, flow_moves: 5.0, flow_reach: 5.5, flow_deep: 3.0, flow_bound: 8.0, flow_secs: 10.0, flow_fight: 20.0, open_air: true, flow_max: 400.0, test_door: true, outside_plain: true, word_no_can: true, word_cuts: true, word_cuts_cap: 8.0, word_small: true, bang_into: true, bang_into_min: 1.5, bang_into_lit: 0.9, bang_into_deep: 6.0, bang_into_wide: 4.2, bang_hand_m: 0.9, bang_pass_v: 6.5, halt_back: true, behind_sam: true, toss_mate: true, charge_answer: true, pod_frames: true, pod_frame_m: 0.55, thr_hand: true, thr_hand_wait: 6.0, thr_hand_every: 0.25, thr_no_can: true, thr_frag_hold: true, thr_doors_leaf: true, frag_hold: true, frag_hold_cap: 8.0, threat_first: true, threat_secs: 12.0, threat_cap: 1.5, threat_near_m: 1.5, threat_stuck: 1.0, buddy_doors: false, thr_set: false, thr_set_secs: 1.5, thr_tube: true, thr_dark: true, thr_first: 1.0, thr_first_min: 2.0, thr_first_stale: 0.0, thr_first_in: 0.0, thr_first_only: true, thr_last: true, thr_last_m: 2.0, thr_last_fresh: 3.0 }
}

pub fn page_roomfight() -> RoomfightK {
    crate::knobs::RoomfightK { on: true, evidence_secs: 12.0, sticky: 0.25, crowd: 0.2, fear_min: 0.15, fall_back: true, rally: true, fall_back_in: true, doorway: true, side_step: true, turn_walk: true, nearest: true, close_m: 2.5, no_shot: 0.4, close_lane: true, point_blank: 2.0, sam_behind: true, sam_beside: true, sam_travel: true, no_shot_go: 1.5, no_shot_pass: 4.0, keep_track: true, rally_k: 1.0, step_ears: true, arc_half_deg: 100.0, cover_keep: true, cover_half_deg: 45.0, cover_near_m: 6.0, team_hot: 45.0, cued: true, let_go: 0.8, step_look: true, step_look_secs: 5.0, door_ears: true, door_look_secs: 3.5, door_shots: true, door_shot_m: 8.0, sam_line: true, hold_short_secs: 4.0, pairs: 6.0, move_engage_m: 7.0, lamp: 0.7, preaim_deg: 12.0, sights: true, sights_foe: false, sights_deg: 45.0 }
}

pub fn page_orders() -> OrdersK {
    crate::knobs::OrdersK { key: String::from("KeyX"), mouse_opens: true, click_gives: false, who_key: String::from("KeyT"), quick_key: String::from("KeyB"), wheel_element: false, wait_key: String::from("Digit5"), wait_alt: String::from("KeyF"), refresh_secs: 1.0, door_pick_m: 1.0, self_m: 1.0, man_m: 0.8, door_ray: true, door_ray_pad: 0.08, door_ray_reach_m: 3.0, self_body: true, floor_snap_m: 1.0, jamb_out: 0.55, jamb_past: 0.45, file_gap: 0.9, seam_m: 0.03, second_floor: true, work_out: true, work_seen_m: 4.0, hall_file: true, follow_back: alloc::vec![1.3, 2.4], crumb: 0.4, trail_m: 10.0, run: 2.2, walk: 1.5, gain: 1.5, near_m: 0.6, band_m: 0.6, word_far_m: 2.0, word_hold_s: 2.5, file_hold: true, file_spread: true, file_apart: 0.9, file_tight: 0.75, off_door: 0.8, back_end: true, in_way: true, retake_goal: true, walk_tries: true, aside_m: 2.6, aside_lane: 0.8, aside_off: 0.95, aside_clear: true, file_queue: true, file_queue_gap: 1.3, file_queue_cost: 500.0, prop_round: true, prop_round_pad: 0.25, prop_round_cost: 6.0, prop_round_more: 2.0, prop_round_door: 2.5, aside_hold: 1.5, brisk: 2.0, brisk_in: 1.8, onto: 1.3, entry: 2.2, move_spread: 0.9, cold_secs: 2.5, pin_posted: true, lane_in: 0.45, lane_out: 0.7, hold_stride: 0.15, stuck_secs: 4.0, hold_group_m: 4.0, hold_door_m: 2.0, cleared: true, cleared_dear: 20.0, known_face: true, own_door: true, round_cap: 1.6, round_own: true, same_way: true, same_near_m: 5.0, same_apart_m: 6.0, same_long: 1.5, trail_doors: true, trail_reach_m: 3.0, trail_long: 1.5 }
}

pub fn page_cover() -> CoverK {
    crate::knobs::CoverK { on: true, stacked: true, in_room: false, rear_read: true, rear_read_in: 1.0, rear_read_m: 14.0, rear_read_deg: 35.0, rear_read_secs: 1.0, rear_read_sticky: 1.3, rear_look: 6.0, rear_walk_m: 5.0, leaf: true, edge: true, reach: 9.0, deal_secs: 0.25, gap_cells: 3.0, through_rows: 3.0, alcove_rows: 3.0, bridge: 3.0, in_m: 0.9, lane: 0.7, lane_keep: 0.5, lane_secs: 0.6, lane_walk: true, sam_deg: 35.0, sam_k: 0.2, held_k: 0.1, shut_k: 0.7, rear_k: 0.8, near_m: 3.0, contact_secs: 4.0, rear_m: 4.0, rear_from: 3.0, rear_deg: 60.0, sticky: 1.2, keep_secs: 1.5, rear_keep: 1.25, behind_k: 0.2, twist: 1.45, back_in: 3.0, back_out: 3.4, back_keep: true, back_body: 0.0, back_turn: true, back_turn_body: 0.7, back_turn_secs: 1.0, swing: 9.0, step: true, step_m: 1.8, step_after: 0.8, step_off_m: 0.7, step_reach: 6.0, step_secs: 5.0, step_place_m: 0.6 }
}

pub fn page_roomfit() -> RoomfitK {
    crate::knobs::RoomfitK { on: true, spread0: 1.2, spread_per: 0.5, apart: 1.1, loose: 1.7, loose_k: 0.8, near_k: 0.35, near_free: 0.4, wall_k: 0.45, corner_k: 0.15, funnel_m: 4.5, funnel_pad: 0.3, funnel_flare: 0.12, funnel_k: 1.6, own_k: 0.7, known_funnel_k: 0.4, shut_funnel_k: 0.6, ang_k: 0.5, ang_lo: 18.0, ang_hi: 60.0, ang_max: 85.0, close_m: 1.6, close_k: 0.8, lane_m: 0.7, lane_k: 4.0, reach: 10.0, near_m: 4.0, known_k: 0.35, shut_k: 0.7, locked_k: 0.3, front_k: 0.25, second_k: 0.3, sector_m: 5.0, sector_k: 0.55, sector_deg: 35.0, sector_across: 0.5, sector_look: 6.0, sector_far_m: 10.0, sector_far_k: 0.7, watch_in_m: 1.5, again_secs: 1.0, sticky: 1.3 }
}

pub fn page_entry() -> EntryK {
    crate::knobs::EntryK { off_openings: true, move_: true, watch: true, near_m: 3.0, line_deg: 45.0, gaps: false, known_m: 4.0, move_m: 3.0, apart: 0.9, off_mouth: false, mouth_m: 1.8 }
}

pub fn page_rear() -> RearK {
    crate::knobs::RearK { settle: true, secs: 0.75, file: false }
}

pub fn page_silent() -> SilentK {
    crate::knobs::SilentK { on: true, start: false, pace: 1.6, hot_pace: false, step: 0.2, lights: true, hush: true, doors: true, kicks: true, picks: true, boot_wedged: true, cans: true, hold_fire: true, noticed: 0.5, glass: true, tools_loud: true, stack_set: true }
}

pub fn page_pair() -> PairK {
    crate::knobs::PairK { on: false, max_squad: 1.0, pick_m: 0.9, stage_m: 1.2, reach: 1.05, secs: 0.6, scuffle: 0.0, grip: true, grip_body: true, pace: 1.1, go_pace: 1.6, crouch: true, stalk_every: 0.3, stalk_wait: 20.0, sweep: true, ring_m: 2.4, keep_way: true, habit_stood: true, steady: true, steady_face_secs: 1.2, front_m: 3.6, cone_m: 9.0, cone_lit: 0.3, watched: true, watch_secs: 1.2, watch_aw: 0.25, look_back: true, heel_ring: 2.9, heel_fresh: 1.2, heel_wait: 22.0, take_secs: 75.0, mark_on_hands: true, quiet_first: true, dark_vis: 0.3, dark_m: 1.5, dark_refuse: true, dark_crouch: true, dark_h: 0.9, lit_secs: 2.0, lit_every: 20.0 }
}

pub fn page_rescue() -> RescueK {
    crate::knobs::RescueK { start: crate::knobs::RescueKStart::Hot, clock: true, resolve_secs: 35.0, resolve_spread: 6.0, second_hostage_secs: 8.0, aim_secs: 1.2, set_secs: 9.0, unset_react: 1.1, captor_set: true, hold_secs: 15.0, door_man: crate::knobs::RescueKDoorMan::Corridor, roamers: 1.0, seed_roamers: true, stun: true, op_lights: true, trained: true, blind_fire: true, def_lights: false, room_lights: false, flow: true, flow_solo: true, marks: true, hold_hot: false, two_door: true, side_pair: true, crew_door: true, crew_cross: true, crew_behind: true, crew_call_secs: 1.5, room_search: true, clock_as_seconds: false, squad: 2.0, sam_at: [7.6, 0.0, 13.4], sam_yaw: 0.9, ops_at: [[6.6, 0.0, 14.25], [8.5, 0.0, 14.3]] }
}

pub fn page_digin() -> DiginK {
    crate::knobs::DiginK { on: false, stay: false, reach_m: 0.0, hold_set: true, keeper: true, refuge: true, cover: true, brace: true, brace_in_room: true, doors: true, radio: true, noticed: true, noticed_secs: 0.35, shut_open: false, shut_secs: 7.0, shut_home_rad: 0.3, shut_doors: 2.0, deep: true, far: true, learn: false, gather: false, gather_m: 24.0, gather_shy_m: 6.0, steady: true, think: [2.0, 5.0], wedge_secs: [4.0, 7.0], wedges: 2.0, caught: [0.5, 0.9], try_secs: 0.8, pace: 1.0, close_m: 6.0, prop_m: 9.0, haul_speed: 0.7, scrape_level: 0.5, scrape_every: 0.8, shout_m: 18.0, alt_m: 10.0, better: 0.6, fear: 2.4, hold_secs: 90.0, redig_secs: 6.0, lanes: true, lane_secs: 2.0 }
}

pub fn page_haulhome() -> HaulhomeK {
    crate::knobs::HaulhomeK { on: true }
}

pub fn page_pressure() -> PressureK {
    crate::knobs::PressureK { on: false, press: true, flank: true, fall_back: true, squad_drill: true, second: false, second_keeps: true, second_set: true, max_men: 4.0, keep_room: true, memory_secs: 15.0, organise_k: 1.0, warned: false, stand_press: false, keeper_left: true, report_secs: 1.4, stand_secs: 75.0, stand_grace: 5.0, again_secs: 14.0, stuns: 0.0, stun_known: true, stun_know_secs: 20.0, stun_moved_secs: 3.0, last_holds: true, picket: false, by_way: false, rescue_crew: false, drilled: false, flank_max_m: 46.0, flank_k: 3.0, flank_slack_m: 14.0, last_man_secs: [0.35, 0.9], out_early: true, loud_secs: 90.0, out_secs: [0.8, 1.6], gather: false, gather_secs: 8.0, flank_secs: 6.0, siege_secs: 25.0, listen: false, converge: true, word_secs: 0.9, way_out: false, way_out_m: 0.6, way_out_secs: 60.0, sweep: false, sweep_secs: 20.0, sweep_men: 2.0, sweep_beat: 1.5, sweep_pair_m: 12.0 }
}

pub fn page_lanes() -> LanesK {
    crate::knobs::LanesK { on: true, secs: 150.0, reach_m: 32.0, half_deg: 22.0, width_m: 1.1, field_m: 12.0, own_m: 1.0, min_m: 2.0, round_k: 0.5, near_m: 1.5, react_secs: 0.35, call_secs: 0.9, see_m: 30.0, body_m: 12.0, dear: 14.0, ahead_m: 3.5, close_m: 4.5, hot_secs: 8.0, round: true, round_men: 2.0, think_secs: 2.5, round_max_m: 80.0, round_gap: 25.0, round_side_deg: 55.0, round_show_m: 8.0, shift_m: 0.9, stun: true, stun_m: 12.0, bang_m: 5.0, push_secs: 6.0, push_m: 12.0, push_fast: 0.8, stun_gap: 20.0, moved_m: 15.0, moved_secs: 20.0, peek: true, peek_quiet: 10.0, peek_gap: 15.0, peek_max: 60.0, peek_lost: 45.0, peek_reach_m: 5.0, peek_near_m: 10.0, peek_look: 0.8, peek_sure: 45.0, peek_keep: 60.0, peek_pace: 1.3, peek_lean: 0.35, pin: true, pin_rounds: 3.0, pin_every: 0.12, pin_snap: 0.6, pin_gap: 4.0, pin_max: 4.0, pin_rest: 20.0, pin_spread: 0.24, pin_aim_deg: 9.0, pin_pace: 2.0, pin_show_m: 12.0, pin_min_m: 7.0, pin_go: false, glass: true, stop_clear: true, look_m: 1.5, roused: true, frag: true, frag_think: 4.0, frag_gap: 30.0, frag_look: 1.5, frag_step_m: 0.9, frag_own: true, drill: true, max: 6.0 }
}

pub fn page_gunhold() -> GunholdK {
    crate::knobs::GunholdK { on: true, fire: true, cover_m: 10.0, near_m: 3.0, rear_deg: 70.0, keep_secs: 14.0, peek_every: 4.0, peek_secs: 0.8, show_m: 2.5, shy_s: 6.0, dash_m: 3.5, face_deg: 50.0, flank_max_m: 40.0, react: [0.1, 0.4] }
}

pub fn page_hard() -> HardK {
    crate::knobs::HardK { reach_m: 0.0, keeper_digs: false, warned: true, second: true, picket: false, by_way: true, rescue_crew: true, way_out: true, stuns: 0.0, drilled: true, cone: true, sweep: true }
}

pub fn page_hard_at() -> HardAtK {
    crate::knobs::HardAtK {  }
}

pub fn page_drilled() -> DrilledK {
    crate::knobs::DrilledK { shaken: 0.5 }
}

pub fn page_squads() -> SquadsK {
    crate::knobs::SquadsK { on: true, think: [2.0, 4.5], net_lag: 0.8, help_men: 3.0, flank_men: 2.0, again_secs: 20.0, settle_secs: 8.0, quiet_secs: 8.0, recall_secs: 20.0, reinforce: [1.0, 2.5], open_pair: false, ready_beat: 0.4, go_round: true, fire_m: 10.0, rear_m: 6.0, soft_m: 3.0, mouth_m: 18.0, hold_lost: true, hold_back_m: 16.0, hold_secs: 40.0, drill_net: true, net_k: 2.0, reinforce_k: 2.0, drill_angles: false, drill_react: false, react_k: 0.75, drill_nerve: false, nerve_k: 0.5, drill_follow: false, follow_m: 6.0, drill_own: false, own_up: 1.0, drill_back: false, back_beat: 3.0, drill_look: true, sweep_secs: 20.0, sweep_men: 2.0, sweep_beat: 1.5, sweep_stayers: true, drill_by_map: true, way: true, radio_check: true, check_every: 40.0, check_jitter: 0.2, check_alert_k: 0.5, check_ask: 4.0, check_send: 8.0, check_heard: [1.0, 2.0], check_hear_m: 4.0, look_men: 2.0, heard_knows: false, heard_asks: true, noticed_eyes: true, call_beat: true, call_secs: [1.5, 2.5], call_body: true, call_cut_ask: [1.0, 2.0], net_ask: false, net_every: 60.0, net_beat: [6.0, 9.0], net_look_men: 2.0, net_stops: 3.0, net_cut_alarm: true }
}

pub fn page_drill_at() -> DrillAtK {
    crate::knobs::DrillAtK {  }
}

pub fn page_cues() -> CuesK {
    crate::knobs::CuesK { on: true, rows: 3.0, secs: 8.0, min_show: 2.5, per_class: 2.0, dup_secs: 10.0, gaps: alloc::vec![0.0, 1.2, 3.0, 3.0, 2.5, 3.0, 12.0], squelch: 0.35, man_secs: 2.5, again_secs: 28.0, still_secs: 8.0, still_quiet: 5.0, stay_secs: 2.0, overheard: true, overheard_gap: 4.0 }
}

pub fn page_buddy() -> BuddyK {
    crate::knobs::BuddyK { on: false, staged: false, loud_secs: 30.0, beat: [2.0, 4.5], quiet: 3.0, calm: 6.0, calm_m: 4.0, answer: [1.0, 2.5], max_m: 30.0, clear_m: 3.0, loners: true, from_pair: false, empty_room: true, again: 20.0, calls: 2.0 }
}

pub fn page_wx_sim() -> WxSimK {
    crate::knobs::WxSimK { rain_k: 1.2, thunder_k: 1.5, thunder_secs: 1.4, thunder_loud: 0.45, roof_bed: 0.32, loud_dry: true }
}

pub fn page_wx_own() -> WxOwnK {
    crate::knobs::WxOwnK { roofs: true, wind: true }
}

pub fn page_snow() -> SnowK {
    crate::knobs::SnowK { on: true }
}

pub fn page_snow_look() -> SnowLookK {
    crate::knobs::SnowLookK { on: true, wisps: true, tracks: true, glow: true, sparkle: true, breath: true, steam: true, grains: true, blast: true, one_air: true, marks: true, past: true, under: true }
}

pub fn page_air_sim() -> AirSimK {
    crate::knobs::AirSimK { on: true, ground: 0.55, min_top: 0.5, max_foot: 0.45, roof_share: 0.0, glass_open: 0.5, leaf_step: 0.04, vent_out: 0.5, gust_k: 0.7, rate: 5.0, sigma_min: 0.35, sigma_max: 2.5, puff_rate: 14.0, settle: 6.0 }
}

pub fn page_laststand() -> LaststandK {
    crate::knobs::LaststandK { regroup: false, call_out: true, places: true, moved_secs: 3.0, still_secs: 2.0, moved_gap: 8.0, few: 3.0, quiet: 3.0, first: [3.0, 6.0], every: 22.0, again: 30.0, answer: [1.2, 2.2], between: [3.2, 3.9], listen: [3.2, 3.8], max_m: 38.0, clear_m: 4.0, known_secs: 30.0, go: [1.3, 2.0], legs: 5.0, dig: [0.6, 1.6], thump: [9.0, 14.0], thump_level: 0.6 }
}

pub fn page_clearline() -> ClearlineK {
    crate::knobs::ClearlineK { on: false, after: 0.3, reach_m: 3.0, pace: 2.0, body_m: 0.6, spare_m: 0.15, cover: true, cover_secs: 3.0, kneel: true, cue: true, again: 0.5, lose: 1.0, hold_max: 10.0, rest: 8.0, stale: 1.5, after_edge: 0.0, in_entry: true, hold_behind: 2.0, hold_step: true, still: 0.0, still_far: 0.0 }
}

pub fn page_stackroom() -> StackroomK {
    crate::knobs::StackroomK { on: true, apart: 0.75, squeeze: 0.5, round_m: 2.0, round_k: 1.6, walk_m: 9.0, next_walk_m: 14.0, next_m: 3.5, lane_back: 1.6, view: true, view_hand: true, view_ajar: 0.6, view_cells: 3.0, view_far: 7.0, view_eye: 1.2, view_step: 0.3, view_past: 4.2, view_off: 0.4, view_apart: 0.85, view_out: 0.3, view_men: 12.0, view_wall: 0.35 }
}

pub fn page_rearwatch() -> RearwatchK {
    crate::knobs::RearwatchK { on: true, through: true, hold: false, hold_secs: 6.0, reach: 9.0, ajar: 0.6, grace: 1.5, short: 1.5, room: true, room_secs: 8.0, room_m: 8.0, room_last: true }
}

pub fn page_stackline() -> StacklineK {
    crate::knobs::StacklineK { on: true, ajar: 0.6, bins: 4.0, eye: 1.2, inset: 0.2, on_mark: 0.35 }
}

pub fn page_sneak() -> SneakK {
    crate::knobs::SneakK { on: false, crouch: true, min_pace: 0.5, low_pace: 1.45, cross_pace: 1.6, post: true, step: 0.14, trail: true, trail_m: 60.0, back: 0.3, look: true, ahead_m: 2.4, lit_at: 0.1, by_eye: true, seen_rate: 0.05, ambient: true, stalk_ambient: true, cone_deg: 25.0, near_m: 2.1, cross_m: 4.5, patience: 8.0, cross_secs: 8.0, budget: 0.12, margin: 1.6, memory: 12.0, cue: true, nag_secs: 7.0, cue_gap: 12.0, stand: true, stand_m: 4.0, stand_near: 0.8, side: true, side_m: 2.6, side_stop: 0.25, door: true, door_m: 1.8, door_pad: 0.4, door_settle: 1.0, hide_gap: 2.5, noise_walk: true, noise_walk_m: 2.5, plan_secs: 0.12, skill: 1.0 }
}

pub fn page_pairup() -> PairupK {
    crate::knobs::PairupK { on: false, one_eye: true, hunted: true, rejoin: true, stay: true, word: true, hands: true }
}

pub fn page_spotted() -> SpottedK {
    crate::knobs::SpottedK { on: false, hold: true, low: true, freeze: true, leash_m: 7.0, slip: true, slip_m: 3.2, lit_at: 0.3, slip_wait: 1.5, wary_m: 14.0, ear_m: 5.0, linger_secs: 1.5, brace: false, brace_secs: 3.0, brace_m: 7.0, close: true, take_m: 2.2, take_reach: 1.05, take_secs: 0.6, fresh_secs: 2.5, caught_m: 30.0, last_secs: 6.0, keen: true, gun: true, step: true, step_m: 3.2, hold_unaware: true, face_deg: 35.0, close_m: 6.0, cue: true }
}

pub fn page_nightbreach() -> NightbreachK {
    crate::knobs::NightbreachK { room_lights: false, op_lights: true, def_lights: false, stun: true, blind_fire: true, loud_boot: true, sam_at: [9.6, 0.0, 20.3], sam_yaw: 1.5707963267948966, squad: 2.0 }
}

pub fn page_roomclear() -> RoomclearK {
    crate::knobs::RoomclearK { seed: 0.0, reroll: true, announce: true, pair_only: false, fresh: true, way: 0.0 }
}

pub fn page_lesson() -> LessonK {
    crate::knobs::LessonK { on: true }
}

pub fn page_snatch() -> SnatchK {
    crate::knobs::SnatchK { on: true, give_up_m: 3.5, halt_m: 3.5, see_m: 14.0, see_rate: 0.05, cuff_m: 4.5, cuff_secs: 3.0, bolt_secs: 6.0 }
}

pub fn page_defuse() -> DefuseK {
    crate::knobs::DefuseK { on: true, work_secs: 6.0, reach: 1.3, marker_m: 5.0, shift: 0.45, start_m: 1.2, lethal_m: 5.0, wound_m: 9.0, radius: 3.5, power: 1.0, warn_secs: 10.0 }
}

pub fn page_par() -> ParK {
    crate::knobs::ParK { on: true }
}

pub fn page_issue() -> IssueK {
    crate::knobs::IssueK { on: true, whole: true, picked: true }
}

pub fn page_verdict() -> VerdictK {
    crate::knobs::VerdictK { on: true, loud_card: true, win_secs: 2.0, loss_secs: 2.6 }
}

pub fn page_unseen() -> UnseenK {
    crate::knobs::UnseenK { on: true, pace_at: 0.5, linger: 1.5, crouch: true, pace: true, min_pace: 0.5, low_pace: 1.45, gain: 1.5, cap: 1.6, post: true, wall: true, wall_off: 0.42, wall_out: 0.4, wall_cross: 0.5, wall_end: 1.6, step: true, low_step: 0.14, walk_step: 0.2, lights: false, hush: true, doors: true, hold: true, glass: true, cue: true, cue_gap: 30.0, wall_beat: true, wall_secs: 0.8 }
}

pub fn page_seenwalk() -> SeenwalkK {
    crate::knobs::SeenwalkK { on: true, lit_at: 0.1, dear: 6.0, long: 1.35, less: 1.5, eye_step: 4.0, eye_y: 1.6, margin: 8.0 }
}

pub fn page_holdfire() -> HoldfireK {
    crate::knobs::HoldfireK { on: true, lift_secs: 45.0, glass: true, quiet: true, loud: true, stack_set: true, stack_wait: true }
}

pub fn page_slots() -> SlotsK {
    crate::knobs::SlotsK { on: true, ok: true, why: String::from(""), table: crate::kit::SlotTable { v: 10.0, pouches: alloc::vec![4.0, 4.0, 4.0, 4.0, 4.0], big: alloc::vec![1.0, 1.0, 1.0, 1.0, 1.0], squad: None, items: { let mut t = JsMap::new(); t.set(String::from("smoke"), crate::kit::SlotItem { id: String::from("smoke"), pouches: 1.0, max: alloc::vec![4.0, 2.0, 2.0, 2.0, 2.0], squad_max: None, big: None, holds: None }); t.set(String::from("bangs"), crate::kit::SlotItem { id: String::from("stun"), pouches: 1.0, max: alloc::vec![4.0, 3.0, 3.0, 3.0, 3.0], squad_max: None, big: None, holds: None }); t.set(String::from("frags"), crate::kit::SlotItem { id: String::from("frag"), pouches: 1.0, max: alloc::vec![3.0, 2.0, 2.0, 2.0, 2.0], squad_max: None, big: None, holds: None }); t.set(String::from("shells"), crate::kit::SlotItem { id: String::from("shell40"), pouches: 1.0, max: alloc::vec![4.0, 1.0, 1.0, 1.0, 1.0], squad_max: None, big: None, holds: None }); t.set(String::from("charges"), crate::kit::SlotItem { id: String::from("doorCharge"), pouches: 1.0, max: alloc::vec![2.0, 2.0, 2.0, 2.0, 2.0], squad_max: None, big: None, holds: None }); t.set(String::from("walls"), crate::kit::SlotItem { id: String::from("wallCharge"), pouches: 2.0, max: alloc::vec![2.0, 2.0, 2.0, 2.0, 2.0], squad_max: None, big: Some(true), holds: None }); t.set(String::from("shotgun"), crate::kit::SlotItem { id: String::from("shotgun"), pouches: 1.0, max: alloc::vec![1.0, 1.0, 1.0, 1.0, 1.0], squad_max: None, big: Some(true), holds: None }); t.set(String::from("wand"), crate::kit::SlotItem { id: String::from("wand"), pouches: 1.0, max: alloc::vec![1.0, 1.0, 1.0, 1.0, 1.0], squad_max: None, big: None, holds: None }); t.set(String::from("cams"), crate::kit::SlotItem { id: String::from("stickyCam"), pouches: 1.0, max: alloc::vec![3.0, 0.0, 0.0, 0.0, 0.0], squad_max: None, big: None, holds: None }); t.set(String::from("rings"), crate::kit::SlotItem { id: String::from("sponge"), pouches: 1.0, max: alloc::vec![5.0, 0.0, 0.0, 0.0, 0.0], squad_max: None, big: None, holds: Some(5.0) }); t.set(String::from("hammer"), crate::kit::SlotItem { id: String::from("hammer"), pouches: 2.0, max: alloc::vec![1.0, 1.0, 1.0, 1.0, 1.0], squad_max: None, big: Some(true), holds: None }); t }, unknown: Vec::new(), loadouts: alloc::vec![crate::kit::SlotLoadout { id: String::from("wall"), name: String::from("One wall charge"), counts: alloc::vec![{ let mut t = JsMap::new(); t.set(String::from("smoke"), 1.0); t.set(String::from("bangs"), 1.0); t.set(String::from("frags"), 1.0); t.set(String::from("shells"), 1.0); t }, { let mut t = JsMap::new(); t.set(String::from("walls"), 1.0); t.set(String::from("bangs"), 2.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 1.0); t.set(String::from("frags"), 2.0); t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], cards: alloc::vec![{ let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], missing: Vec::new() }, crate::kit::SlotLoadout { id: String::from("stun"), name: String::from("Stun heavy"), counts: alloc::vec![{ let mut t = JsMap::new(); t.set(String::from("smoke"), 1.0); t.set(String::from("bangs"), 2.0); t.set(String::from("shells"), 1.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 3.0); t.set(String::from("charges"), 1.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 3.0); t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], cards: alloc::vec![{ let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], missing: Vec::new() }, crate::kit::SlotLoadout { id: String::from("frag"), name: String::from("Frag heavy"), counts: alloc::vec![{ let mut t = JsMap::new(); t.set(String::from("smoke"), 1.0); t.set(String::from("bangs"), 1.0); t.set(String::from("frags"), 1.0); t.set(String::from("shells"), 1.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 2.0); t.set(String::from("frags"), 2.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 1.0); t.set(String::from("frags"), 2.0); t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], cards: alloc::vec![{ let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], missing: Vec::new() }, crate::kit::SlotLoadout { id: String::from("quiet"), name: String::from("Quiet"), counts: alloc::vec![{ let mut t = JsMap::new(); t.set(String::from("smoke"), 3.0); t.set(String::from("frags"), 1.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 2.0); t }, { let mut t = JsMap::new(); t.set(String::from("bangs"), 1.0); t }, { let mut t = JsMap::new();  t }, { let mut t = JsMap::new();  t }], cards: alloc::vec![{ let mut t = JsMap::new(); t.set(String::from("weapon"), crate::kit::KitValue::Text(String::from("pistol"))); t.set(String::from("shells"), crate::kit::KitValue::Num(0.0)); t }, { let mut t = JsMap::new(); t.set(String::from("weapon"), crate::kit::KitValue::Text(String::from("pistol"))); t.set(String::from("suppressed"), crate::kit::KitValue::Bool(true)); t.set(String::from("light"), crate::kit::KitValue::Bool(false)); t }, { let mut t = JsMap::new(); t.set(String::from("weapon"), crate::kit::KitValue::Text(String::from("pistol"))); t.set(String::from("suppressed"), crate::kit::KitValue::Bool(true)); t.set(String::from("light"), crate::kit::KitValue::Bool(false)); t }, { let mut t = JsMap::new(); t.set(String::from("weapon"), crate::kit::KitValue::Text(String::from("pistol"))); t.set(String::from("suppressed"), crate::kit::KitValue::Bool(true)); t.set(String::from("light"), crate::kit::KitValue::Bool(false)); t }, { let mut t = JsMap::new(); t.set(String::from("weapon"), crate::kit::KitValue::Text(String::from("pistol"))); t.set(String::from("suppressed"), crate::kit::KitValue::Bool(true)); t.set(String::from("light"), crate::kit::KitValue::Bool(false)); t }], missing: Vec::new() }] } }
}

pub fn page_destruction() -> DestructionK {
    crate::knobs::DestructionK { on: true }
}

pub fn page_bigslot() -> BigslotK {
    crate::knobs::BigslotK { on: true, swings: 2.0, swing: 0.7, windup: 0.45, draw: 0.5, shells: 2.0, shut: true, beside: true, moves: true, snug: true, weight: true, equip: 0.6, stow: 0.45, reach: 2.4, heft: 0.7, equip_w: 0.8, draw_w: 0.7, stow_w: 0.75, flow: 0.2 }
}

pub fn page_kitdeal() -> KitdealK {
    crate::knobs::KitdealK { on: true, more: true, stealth: true }
}

pub fn page_cut() -> CutK {
    crate::knobs::CutK { on: true, swap: true }
}

/// every knob table
#[derive(Clone, Debug)]
pub struct Knobs {
    pub aim: AimK,
    pub trained: TrainedK,
    pub ai_tune: AiTuneK,
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
    pub glassfx: GlassfxK,
    pub glassbreak: GlassbreakK,
    pub glasspen: GlasspenK,
    pub shards: ShardsK,
    pub talk_tune: TalkTuneK,
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
    pub tapstep: TapstepK,
    pub overhear: OverhearK,
    pub score: ScoreK,
    pub badge: BadgeK,
    pub ckpt: CkptK,
    pub fog: FogK,
    pub moves: MovesK,
    pub mv: MvK,
    pub turn: TurnK,
    pub wound: WoundK,
    pub doorpath: DoorpathK,
    pub walls: WallsK,
    pub wallcharge: WallchargeK,
    pub wallshot: WallshotK,
    pub wallkill: WallkillK,
    pub hardbreath: HardbreathK,
    pub castor: CastorK,
    pub wallreact: WallreactK,
    pub coverfire: CoverfireK,
    pub furnblast: FurnblastK,
    pub doorway: DoorwayK,
    pub doorblast: DoorblastK,
    pub doorshot: DoorshotK,
    pub doorthrow: DoorthrowK,
    pub winbars: WinbarsK,
    pub opening: OpeningK,
    pub doorstay: DoorstayK,
    pub handle: HandleK,
    pub doormanners: DoormannersK,
    pub breaks: BreaksK,
    pub furn: FurnK,
    pub snowbog: SnowbogK,
    pub rubble: RubbleK,
    pub evict: EvictK,
    pub debris: DebrisK,
    pub lightwall: LightwallK,
    pub airdust: AirdustK,
    pub wallsnap: WallsnapK,
    pub gunwall: GunwallK,
    pub close: CloseK,
    pub plan: PlanK,
    pub gear: GearK,
    pub start: StartK,
    pub samestart: SamestartK,
    pub take: TakeK,
    pub reload: ReloadK,
    pub quickthrow: QuickthrowK,
    pub samstun: SamstunK,
    pub recap: RecapK,
    pub aimweight: AimweightK,
    pub aimcost: AimcostK,
    pub friends: FriendsK,
    pub teamtally: TeamtallyK,
    pub friendlyfire: FriendlyfireK,
    pub lightswitch: LightswitchK,
    pub carlamps: CarlampsK,
    pub hitstuff: HitstuffK,
    pub markcut: MarkcutK,
    pub marksit: MarksitK,
    pub coatfinish: CoatfinishK,
    pub solidglass: SolidglassK,
    pub cleanrestart: CleanrestartK,
    pub fireline: FirelineK,
    pub savetimeline: SavetimelineK,
    pub handslot: HandslotK,
    pub handring: HandringK,
    pub throwback: ThrowbackK,
    pub ownblow: OwnblowK,
    pub sprintdoor: SprintdoorK,
    pub teampace: TeampaceK,
    pub noshove: NoshoveK,
    pub rails: RailsK,
    pub doorforce: DoorforceK,
    pub slotsetup: SlotsetupK,
    pub glassreal: GlassrealK,
    pub quietfog: QuietfogK,
    pub told: ToldK,
    pub words: WordsK,
    pub kitanim: KitanimK,
    pub kit: KitK,
    pub throw_by: ThrowByK,
    pub hands: HandsK,
    pub work: WorkK,
    pub wand: WandK,
    pub sticky: StickyK,
    pub peeksight: PeeksightK,
    pub owncharge: OwnchargeK,
    pub gunslots: GunslotsK,
    pub gunlook: GunlookK,
    pub gunroles: GunrolesK,
    pub kittree: KittreeK,
    pub squadthrow: SquadthrowK,
    pub squad40: Squad40K,
    pub autopilot: AutopilotK,
    pub planlay: PlanlayK,
    pub cmdring: CmdringK,
    pub torch: TorchK,
    pub backkey: BackkeyK,
    pub fixedstep: FixedstepK,
    pub tween: TweenK,
    pub savepic: SavepicK,
    pub evenpace: EvenpaceK,
    pub findtuned: FindtunedK,
    pub autocam: AutocamK,
    pub morale: MoraleK,
    pub condition: ConditionK,
    pub inhand: InhandK,
    pub packside: PacksideK,
    pub gunhand: GunhandK,
    pub runner: RunnerK,
    pub startle: StartleK,
    pub rowset: RowsetK,
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
    pub snatchout: SnatchoutK,
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
    pub music: MusicK,
    pub lived: LivedK,
    pub underfoot: UnderfootK,
    pub round: RoundK,
    pub shapes: ShapesK,
    pub flash: FlashK,
    pub pace: PaceK,
    pub lights: LightsK,
    pub nvg: NvgK,
    pub ir: IrK,
    pub irlight: IrlightK,
    pub wallpen: WallpenK,
    pub laser: LaserK,
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
    pub haulhome: HaulhomeK,
    pub pressure: PressureK,
    pub lanes: LanesK,
    pub gunhold: GunholdK,
    pub hard: HardK,
    pub hard_at: HardAtK,
    pub drilled: DrilledK,
    pub squads: SquadsK,
    pub drill_at: DrillAtK,
    pub cues: CuesK,
    pub buddy: BuddyK,
    pub wx_sim: WxSimK,
    pub wx_own: WxOwnK,
    pub snow: SnowK,
    pub snow_look: SnowLookK,
    pub air_sim: AirSimK,
    pub laststand: LaststandK,
    pub clearline: ClearlineK,
    pub stackroom: StackroomK,
    pub rearwatch: RearwatchK,
    pub stackline: StacklineK,
    pub sneak: SneakK,
    pub pairup: PairupK,
    pub spotted: SpottedK,
    pub nightbreach: NightbreachK,
    pub roomclear: RoomclearK,
    pub lesson: LessonK,
    pub snatch: SnatchK,
    pub defuse: DefuseK,
    pub par: ParK,
    pub issue: IssueK,
    pub verdict: VerdictK,
    pub unseen: UnseenK,
    pub seenwalk: SeenwalkK,
    pub holdfire: HoldfireK,
    pub slots: SlotsK,
    pub destruction: DestructionK,
    pub bigslot: BigslotK,
    pub kitdeal: KitdealK,
    pub cut: CutK,
}
impl Knobs {
    /// THE PAGE'S values, as a game starts
    pub fn page() -> Self {
        Knobs {
            aim: page_aim(),
            trained: page_trained(),
            ai_tune: page_ai_tune(),
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
            glassfx: page_glassfx(),
            glassbreak: page_glassbreak(),
            glasspen: page_glasspen(),
            shards: page_shards(),
            talk_tune: page_talk_tune(),
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
            tapstep: page_tapstep(),
            overhear: page_overhear(),
            score: page_score(),
            badge: page_badge(),
            ckpt: page_ckpt(),
            fog: page_fog(),
            moves: page_moves(),
            mv: page_mv(),
            turn: page_turn(),
            wound: page_wound(),
            doorpath: page_doorpath(),
            walls: page_walls(),
            wallcharge: page_wallcharge(),
            wallshot: page_wallshot(),
            wallkill: page_wallkill(),
            hardbreath: page_hardbreath(),
            castor: page_castor(),
            wallreact: page_wallreact(),
            coverfire: page_coverfire(),
            furnblast: page_furnblast(),
            doorway: page_doorway(),
            doorblast: page_doorblast(),
            doorshot: page_doorshot(),
            doorthrow: page_doorthrow(),
            winbars: page_winbars(),
            opening: page_opening(),
            doorstay: page_doorstay(),
            handle: page_handle(),
            doormanners: page_doormanners(),
            breaks: page_breaks(),
            furn: page_furn(),
            snowbog: page_snowbog(),
            rubble: page_rubble(),
            evict: page_evict(),
            debris: page_debris(),
            lightwall: page_lightwall(),
            airdust: page_airdust(),
            wallsnap: page_wallsnap(),
            gunwall: page_gunwall(),
            close: page_close(),
            plan: page_plan(),
            gear: page_gear(),
            start: page_start(),
            samestart: page_samestart(),
            take: page_take(),
            reload: page_reload(),
            quickthrow: page_quickthrow(),
            samstun: page_samstun(),
            recap: page_recap(),
            aimweight: page_aimweight(),
            aimcost: page_aimcost(),
            friends: page_friends(),
            teamtally: page_teamtally(),
            friendlyfire: page_friendlyfire(),
            lightswitch: page_lightswitch(),
            carlamps: page_carlamps(),
            hitstuff: page_hitstuff(),
            markcut: page_markcut(),
            marksit: page_marksit(),
            coatfinish: page_coatfinish(),
            solidglass: page_solidglass(),
            cleanrestart: page_cleanrestart(),
            fireline: page_fireline(),
            savetimeline: page_savetimeline(),
            handslot: page_handslot(),
            handring: page_handring(),
            throwback: page_throwback(),
            ownblow: page_ownblow(),
            sprintdoor: page_sprintdoor(),
            teampace: page_teampace(),
            noshove: page_noshove(),
            rails: page_rails(),
            doorforce: page_doorforce(),
            slotsetup: page_slotsetup(),
            glassreal: page_glassreal(),
            quietfog: page_quietfog(),
            told: page_told(),
            words: page_words(),
            kitanim: page_kitanim(),
            kit: page_kit(),
            throw_by: page_throw_by(),
            hands: page_hands(),
            work: page_work(),
            wand: page_wand(),
            sticky: page_sticky(),
            peeksight: page_peeksight(),
            owncharge: page_owncharge(),
            gunslots: page_gunslots(),
            gunlook: page_gunlook(),
            gunroles: page_gunroles(),
            kittree: page_kittree(),
            squadthrow: page_squadthrow(),
            squad40: page_squad40(),
            autopilot: page_autopilot(),
            planlay: page_planlay(),
            cmdring: page_cmdring(),
            torch: page_torch(),
            backkey: page_backkey(),
            fixedstep: page_fixedstep(),
            tween: page_tween(),
            savepic: page_savepic(),
            evenpace: page_evenpace(),
            findtuned: page_findtuned(),
            autocam: page_autocam(),
            morale: page_morale(),
            condition: page_condition(),
            inhand: page_inhand(),
            packside: page_packside(),
            gunhand: page_gunhand(),
            runner: page_runner(),
            startle: page_startle(),
            rowset: page_rowset(),
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
            snatchout: page_snatchout(),
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
            music: page_music(),
            lived: page_lived(),
            underfoot: page_underfoot(),
            round: page_round(),
            shapes: page_shapes(),
            flash: page_flash(),
            pace: page_pace(),
            lights: page_lights(),
            nvg: page_nvg(),
            ir: page_ir(),
            irlight: page_irlight(),
            wallpen: page_wallpen(),
            laser: page_laser(),
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
            haulhome: page_haulhome(),
            pressure: page_pressure(),
            lanes: page_lanes(),
            gunhold: page_gunhold(),
            hard: page_hard(),
            hard_at: page_hard_at(),
            drilled: page_drilled(),
            squads: page_squads(),
            drill_at: page_drill_at(),
            cues: page_cues(),
            buddy: page_buddy(),
            wx_sim: page_wx_sim(),
            wx_own: page_wx_own(),
            snow: page_snow(),
            snow_look: page_snow_look(),
            air_sim: page_air_sim(),
            laststand: page_laststand(),
            clearline: page_clearline(),
            stackroom: page_stackroom(),
            rearwatch: page_rearwatch(),
            stackline: page_stackline(),
            sneak: page_sneak(),
            pairup: page_pairup(),
            spotted: page_spotted(),
            nightbreach: page_nightbreach(),
            roomclear: page_roomclear(),
            lesson: page_lesson(),
            snatch: page_snatch(),
            defuse: page_defuse(),
            par: page_par(),
            issue: page_issue(),
            verdict: page_verdict(),
            unseen: page_unseen(),
            seenwalk: page_seenwalk(),
            holdfire: page_holdfire(),
            slots: page_slots(),
            destruction: page_destruction(),
            bigslot: page_bigslot(),
            kitdeal: page_kitdeal(),
            cut: page_cut(),
        }
    }
}
