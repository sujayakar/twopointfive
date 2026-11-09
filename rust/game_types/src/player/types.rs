//! The types of `player`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Hold {
    pub g: GuardId,
    pub variant: crate::character::HoldVariant,
    pub phase: crate::character::HoldPhase,
    pub t: f64,
    pub since: f64,
    pub wind_t: f64,
    pub swap_t: f64,
    pub remarked: bool,
    pub gun_out: f64,
    pub fire_queued: bool,
    pub shot_t: f64,
    pub ask: Option<crate::player::HoldAsk>,
    pub after: Option<crate::player::HoldAfter>,
    pub strip: Option<crate::disarm::Strip>,
    pub from: Option<V3>,
    pub hit_at: Option<f64>,
    pub yank_m: Option<f64>,
    pub yanked: Option<f64>,
    pub step_e: Option<f64>,
    pub step_m: Option<f64>,
    pub yaw0: Option<f64>,
    pub coil0: Option<f64>,
    pub rmb_choke: Option<f64>,
    pub rmb_t: Option<f64>,
    /// INVENTED: dealt when a hold starts; a new hold of the same man counts as new
    pub serial: u32,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HoldAsk {
    pub t: f64,
    pub dur: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct HoldAfter {
    pub text: String,
}

#[derive(Clone, Copy, Debug)]
pub struct Legs {
    pub char: crate::player::LegsChar,
    pub safe: Option<V3>,
    pub crouch: bool,
    pub sprinting: bool,
    pub dash: bool,
    pub backpedal: bool,
    pub speed_sm: f64,
    pub pace: f64,
    pub dragging: Option<GuardId>,
    pub haul: Option<crate::hands::Haul>,
    pub shoved: [f64; 2],
    pub shoved_t: f64,
    pub shoved_from: [f64; 2],
    pub down: bool,
    pub door_rip_until: f64,
    pub door_bash_at: f64,
    pub door_bash_lean: f64,
}
impl Default for Legs {
    fn default() -> Self {
        Legs {
            char: crate::player::LegsChar::default(),
            safe: None,
            crouch: false,
            sprinting: false,
            dash: false,
            backpedal: false,
            speed_sm: 0.0,
            pace: 0.0,
            dragging: None,
            haul: None,
            shoved: [0.0; 2],
            shoved_t: 0.0,
            shoved_from: [0.0; 2],
            down: false,
            door_rip_until: -1.0,
            door_bash_at: -10.0,
            door_bash_lean: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LegsChar {
    pub pos: V3,
    pub vel: V3,
    pub body_yaw: f64,
    pub aim_yaw: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallHold {
    pub n: V3,
    pub along: V3,
    pub d: f64,
    pub lo: f64,
    pub hi: f64,
    pub open_lo: bool,
    pub open_hi: bool,
    pub u: f64,
    pub v: f64,
    pub side: f64,
    pub off: f64,
    pub away_t: f64,
    pub settle: f64,
    pub from: V3,
    pub block_dir: f64,
    pub block_t: f64,
    pub seated: bool,
    pub drawn: bool,
    pub at_edge: f64,
    pub peek: f64,
    pub peek_dir: f64,
    pub crouched: bool,
    pub take_t: Option<f64>,
    pub yaw0: Option<f64>,
    pub yaw_d: Option<f64>,
    pub hit_t: Option<f64>,
    pub kind: Option<crate::player::WallHoldKind>,
    pub up: Option<f64>,
    pub lean: Option<f64>,
    pub lean_want: Option<bool>,
    pub lean_dir: Option<f64>,
    pub lean_dwell: Option<f64>,
    pub lean_out_t: Option<f64>,
    pub lean_off: Option<f64>,
    pub lean_more: Option<f64>,
    pub flip_t: Option<f64>,
    pub say_t: Option<f64>,
    pub rise: Option<f64>,
    pub owed: Option<f64>,
    pub cam_along: Option<f64>,
    pub cam: bool,
    pub ran: Option<f64>,
    /// INVENTED: g.serial() where wallSnap makes the hold (player.ts:1081): unseen.ts:184 and :195 ask by identity whether this is the same take of the wall (State.hold, placed). A copy of the hold keeps it
    pub serial: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum WallHoldKind {
    Stack,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DOORFIT {
    pub lat: [f64; 2],
    pub out: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlayerModWsPendValue {
    pub t: f64,
    pub at: f64,
    pub blocked: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PlayerModWsPressValue {
    pub t: f64,
    pub at: f64,
    pub n: Option<V3>,
    pub dx: f64,
    pub dz: f64,
    pub held: bool,
    pub until: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CollideManPl {
    pub char: crate::player::CollideManPlChar,
    pub safe: Option<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CollideManPlChar {
    pub pos: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShellBreak {
    Fire,
    Bolt,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StrideStepStalk {
    pub pace: f64,
    pub keep: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GunRaisedM {
    pub sprinting: bool,
}

#[derive(Clone, Debug, Default)]
pub struct LeafFacingDp {
    pub w: f64,
    pub n: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FindWallWhy {
    pub blocked: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FindWall {
    pub n: V3,
    pub t: V3,
    pub d: f64,
    pub lo: f64,
    pub hi: f64,
    pub u: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallSnapWhy {
    pub blocked: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallStepOffHow {
    Run,
    Q,
    Away,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallSnapLaterWhy {
    pub blocked: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PressFace {
    pub n: V3,
    pub at: V3,
    pub off: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ThrowRest {
    pub at: V3,
    pub path: Vec<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HeldTalk {
    pub asking: bool,
    pub sam_up: bool,
    pub ready: bool,
    pub settled: bool,
    pub speaking: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FreeHeldWhy {
    Shot,
    Cancel,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StartKickTool {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SideKickSpot {
    pub pos: V3,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AimLevelC {
    pub pos: V3,
    pub body_yaw: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CursorHitRay {
    pub ro: V3,
    pub rd: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CursorPointRay {
    pub ro: V3,
    pub rd: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CursorPointHit {
    pub t: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct PlayerMod {
    /// (src/game/player.ts:1135 wsPend)
    pub ws_pend: JsMap<PlayerId, crate::player::PlayerModWsPendValue>,
    /// (src/game/player.ts:1165 wsPress)
    pub ws_press: JsMap<PlayerId, crate::player::PlayerModWsPressValue>,
    /// (src/game/player.ts:1210 PRESS_THINGS)
    #[is({ let mut s = JsSet::new(); for k in ["door", "switch", "breaker", "objective", "cam", "body"] { s.add(String::from(k)); } s })] pub press_things: JsSet<String>,
    /// (src/game/player.ts:2301 lightAt)
    pub light_at: JsMap<PlayerId, [f64; 2]>,
}

/// (src/game/player.ts:92 BODY_R)
pub const BODY_R: f64 = 0.42;

/// (src/game/player.ts:94 KEEP_OFF)
pub const KEEP_OFF: f64 = 0.55;

/// (src/game/player.ts:101 HOLD_PACE)
pub const HOLD_PACE: f64 = 0.85;

/// (src/game/player.ts:101 HOLD_PACE_BACK)
pub const HOLD_PACE_BACK: f64 = 0.65;

/// (src/game/player.ts:101 HOLD_TURN)
pub const HOLD_TURN: f64 = 2.6;

/// (src/game/player.ts:106 INTERROGATE_GAP)
pub const INTERROGATE_GAP: f64 = 1.2;

/// (src/game/player.ts:106 ANSWER_TAIL)
pub const ANSWER_TAIL: f64 = 0.5;

/// (src/game/player.ts:111 GRAB_NOISE)
pub const GRAB_NOISE: f64 = 0.12;

/// (src/game/player.ts:111 CHOKE_NOISE)
pub const CHOKE_NOISE: f64 = 0.15;

/// (src/game/player.ts:111 FOLD_NOISE)
pub const FOLD_NOISE: f64 = 0.3;

/// (src/game/player.ts:111 WHIP_NOISE)
pub const WHIP_NOISE: f64 = 0.35;

/// (src/game/player.ts:114 CHOKE_GRAIN)
pub const CHOKE_GRAIN: f64 = 0.4;

/// (src/game/player.ts:117 HOLD_FIRE_CD)
pub const HOLD_FIRE_CD: f64 = 0.7;

/// (src/game/player.ts:117 HOLD_SPREAD)
pub const HOLD_SPREAD: f64 = 0.09;

/// (src/game/player.ts:149 DOOR_R)
pub const DOOR_R: f64 = 0.3;

/// (src/game/player.ts:149 DOOR_FIT: a table nothing writes)
pub fn door_fit() -> crate::player::DOORFIT { crate::player::DOORFIT { lat: [0.12, 0.5], out: [0.55, 1.0] } }

/// (src/game/player.ts:167 SWEEP_STEP)
pub const SWEEP_STEP: f64 = 0.2;

/// (src/game/player.ts:261 RIFLE_EQUIP_SECS)
pub const RIFLE_EQUIP_SECS: f64 = 0.45;

/// (src/game/player.ts:291 LOW_AT)
pub const LOW_AT: f64 = 1.25;

/// (src/game/player.ts:291 LOW_UNDER)
pub const LOW_UNDER: f64 = 0.25;

/// (src/game/player.ts:291 LOW_TOP)
pub const LOW_TOP: f64 = 0.15;

/// (src/game/player.ts:306 LEVEL_REACH)
pub const LEVEL_REACH: f64 = 20.0;

/// (src/game/player.ts:896 WALL_REACH)
pub const WALL_REACH: f64 = 0.7;

/// (src/game/player.ts:898 WALL_GAP)
pub const WALL_GAP: f64 = 0.03;

/// (src/game/player.ts:901 KNEE_Y)
pub const KNEE_Y: f64 = 0.45;

/// (src/game/player.ts:901 CHEST_Y)
pub const CHEST_Y: f64 = 1.35;

/// (src/game/player.ts:901 HIGH_Y)
pub const HIGH_Y: f64 = 2.7;

/// (src/game/player.ts:903 COPLANAR)
pub const COPLANAR: f64 = 0.01;

/// (src/game/player.ts:905 UPPER_TOL)
pub const UPPER_TOL: f64 = 0.08;

/// (src/game/player.ts:907 WALL_BRIDGE)
pub const WALL_BRIDGE: f64 = 0.15;

/// (src/game/player.ts:909 WALL_MIN_FACE)
pub const WALL_MIN_FACE: f64 = 0.8999999999999999;

/// (src/game/player.ts:911 WALL_PACE)
pub const WALL_PACE: f64 = 1.4;

/// (src/game/player.ts:913 WALL_TURN)
pub const WALL_TURN: f64 = 0.9599310885968813;

/// (src/game/player.ts:915 WALL_LOOK)
pub const WALL_LOOK: f64 = 0.4363323129985824;

/// (src/game/player.ts:915 WALL_PEEK_LOOK)
pub const WALL_PEEK_LOOK: f64 = 0.5235987755982988;

/// (src/game/player.ts:918 WALL_SHOVE_OFF)
pub const WALL_SHOVE_OFF: f64 = 0.25;

/// (src/game/player.ts:918 WALL_AWAY_SECS)
pub const WALL_AWAY_SECS: f64 = 0.25;

/// (src/game/player.ts:920 WALL_SETTLE)
pub const WALL_SETTLE: f64 = 0.22;

/// (src/game/player.ts:920 WALL_PEEK_SECS)
pub const WALL_PEEK_SECS: f64 = 0.3;

/// (src/game/player.ts:992 PROP_BODY_R)
pub const PROP_BODY_R: f64 = 0.3;

/// (src/game/player.ts:1382 LEAN_CHEST)
pub const LEAN_CHEST: f64 = 1.2;

/// (src/game/player.ts:1384 STACK_PRESS)
pub const STACK_PRESS: f64 = 0.12;

/// (src/game/player.ts:2554 INTERACT_PRIORITY: a table nothing writes)
pub fn interact_priority() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("guard"), 0.0); t.set(String::from("held"), 0.0); t.set(String::from("body"), 1.0); t.set(String::from("objective"), 2.0); t.set(String::from("door"), 3.0); t.set(String::from("light"), 9.0); t.set(String::from("breaker"), 2.5); t.set(String::from("switch"), 3.2); t.set(String::from("charge"), 9.4); t.set(String::from("wall"), 9.5); t.set(String::from("cam"), 9.6); t.set(String::from("cancel"), 10.0); t } }

/// (src/game/player.ts:2562 NEARBY_M)
pub const NEARBY_M: f64 = 3.0;

/// (src/game/player.ts:2564 MAX_NEAR)
pub const MAX_NEAR: f64 = 3.0;

/// (src/game/player.ts:2568 EASE_RATE)
pub const EASE_RATE: f64 = 0.55;

/// (src/game/player.ts:2568 EASE_MAX)
pub const EASE_MAX: f64 = 1.45;

/// (src/game/player.ts:2568 EASE_OFFER)
pub const EASE_OFFER: f64 = 0.62;
