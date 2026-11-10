//! The types of `doors`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorDef {
    pub name: String,
    pub hinge: [f64; 2],
    pub closed_dir: f64,
    pub width: f64,
    pub height: Option<f64>,
    pub thickness: Option<f64>,
    pub min_angle: Option<f64>,
    pub max_angle: Option<f64>,
    pub angle: Option<f64>,
    pub swing: Option<f64>,
    pub albedo: Option<V3>,
    pub closer: Option<bool>,
    pub exterior: Option<bool>,
    pub locked: Option<bool>,
    pub key_side: Option<f64>,
    pub lock: Option<String>,
    pub steel: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorDrive {
    pub who: f64,
    pub dir: f64,
    pub kind: crate::doors::DoorDriveKind,
    pub t: f64,
    pub from: f64,
    pub quiet: bool,
    pub side: f64,
    pub rate: f64,
    pub px: f64,
    pub pz: f64,
    pub hit: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Words)]
pub enum DoorDriveKind {
    #[default] Boot,
    Body,
    Hand,
    Swing,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorStrike {
    pub door: DoorId,
    pub by: f64,
    pub tier: crate::doors::DoorStrikeTier,
    pub floor: bool,
    pub dir: [f64; 2],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum DoorStrikeTier {
    #[default] Boot,
    Open,
    Slam,
}

#[derive(Clone, Debug, Default)]
pub struct DoorContact {
    pub pos: V3,
    pub radius: f64,
    pub bash: bool,
    pub who: f64,
    pub quiet: bool,
    pub weak: Option<bool>,
    pub key: Option<bool>,
    pub keys: Option<JsSet<String>>,
    pub blocked_by: Option<DoorId>,
    pub hand: Option<f64>,
    pub round: Option<bool>,
    pub run: Option<crate::doors::DoorContactRun>,
    pub bashed: Option<DoorId>,
    pub ripped: Option<DoorId>,
    pub bashed_at: Option<f64>,
    pub struck: Option<crate::doors::DoorStrike>,
    pub moved: Option<bool>,
    pub man: Option<f64>,
    pub solid: Option<bool>,
    pub vel: Option<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorContactRun {
    pub bash: f64,
    pub rip: f64,
    pub rip_level: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum DoorSound {
    #[default] Creak,
    Bang,
    Latch,
    Bash,
    Rattle,
    Pick,
    Unlock,
    Kick,
    Clang,
    Thud,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorEvent {
    pub door: DoorId,
    pub sound: crate::doors::DoorSound,
    pub pos: V3,
    pub who: f64,
    pub level: f64,
}

/// a leaf off its hinges
#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Slab {
    pub p: [f64; 3],
    pub v: [f64; 3],
    pub a: f64,
    pub spin: f64,
    pub tip: f64,
    pub tip_v: f64,
    pub side: f64,
    pub rest: bool,
    pub landed: bool,
    pub who: f64,
    pub t: f64,
    pub ang: f64,
    pub bits: Option<[f64; 4]>,
    /// the clock a wedged slab is not searched for before
    pub wedged: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Door {
    pub i: f64,
    pub key_side: f64,
    pub lock_id: String,
    pub height: f64,
    pub thick: f64,
    pub min_auth: f64,
    pub max_auth: f64,
    #[is(BoxId(0))] pub box_: BoxId,
    pub handle_boxes: Vec<BoxId>,
    pub slab: Option<crate::doors::Slab>,
    pub leaf_h: V3,
    pub leaf_marks: Option<Vec<crate::scene_markings::MarkingDef>>,
    pub def: crate::doors::DoorDef,
    #[is([0.0, 1.0, 0.0])] pub pos_at: V3,
    pub frame_centre: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorSwing {
    pub hinge: V3,
    pub swing: f64,
    pub open: f64,
    pub radius: f64,
    pub closed_dir: f64,
    pub one_way: bool,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Doors {
    pub list: Vec<DoorId>,
    pub rounds: f64,
    #[is(crate::doors::DoorsSolids { carried: 0.0, stopped: 0.0, passed: 0.0, reached: 0.0, held: 0.0, aside: 0.0 })] pub solids: crate::doors::DoorsSolids,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorsSolids {
    pub carried: f64,
    pub stopped: f64,
    pub passed: f64,
    pub reached: f64,
    pub held: f64,
    pub aside: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SIDEKICK {
    pub past: f64,
    pub out: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorPoseAt {
    pub c: V3,
    pub yaw: f64,
    pub handles: [V3; 2],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorsWorkSpot {
    pub pos: V3,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorsKickSpot {
    pub pos: V3,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorsKickSpotSide {
    pub pos: V3,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorsFrameCoords {
    pub along: f64,
    pub off: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorsTornCause {
    Blast,
    Fire,
}

pub const DOORS_PICK_SECS: f64 = 4.2;

pub const DOORS_PICK_LEVEL: f64 = 0.16;

pub const DOORS_UNLOCK_LEVEL: f64 = 0.3;

pub const DOORS_KEY_LEVEL: f64 = 0.12;

pub const DOORS_USE_REACH: f64 = 1.35;

pub fn side_kick() -> crate::doors::SIDEKICK { crate::doors::SIDEKICK { past: 0.3, out: 0.55 } }

pub const PULL_VEL: f64 = 3.9;

pub const PULL_VEL_QUIET: f64 = 2.2;

pub const PULL_SECS: f64 = 0.7;

pub const DRIVEN_SECS: f64 = 0.25;

pub const SHUT_HOME: f64 = 0.05;

pub const FLOOR_KEY: &str = "floor";

pub const MAN_SIDE: f64 = 3.0;

pub const MAN_FOLDED: f64 = 4.0;

pub const MAN_BRACED: f64 = 8.0;

pub const MAN_CALM: f64 = 16.0;

pub const MAN_HELD: f64 = 32.0;

pub const FLOOR_Y0: f64 = 0.2;
