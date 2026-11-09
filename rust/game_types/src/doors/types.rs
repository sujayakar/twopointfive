//! The types of `doors`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum DoorDriveKind {
    #[default] Boot,
    Body,
    Hand,
    Swing,
}
impl DoorDriveKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorDriveKind::Boot => "boot",
            DoorDriveKind::Body => "body",
            DoorDriveKind::Hand => "hand",
            DoorDriveKind::Swing => "swing",
        }
    }
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

#[derive(Clone, Copy, Debug, Default)]
pub struct SlabSeen {
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
    pub wedged: Option<f64>,
}

#[derive(Clone, Debug, ToBytes)]
pub struct Door {
    pub i: f64,
    pub key_side: f64,
    pub lock_id: String,
    pub height: f64,
    pub thick: f64,
    pub min_auth: f64,
    pub max_auth: f64,
    pub drive_cols: DriveColsId,
    pub box_: BoxId,
    pub handle_boxes: Vec<BoxId>,
    pub slab_row: SlabRowId,
    pub leaf_h: V3,
    pub leaf_marks: Option<Vec<crate::scene_markings::MarkingDef>>,
    pub def: crate::doors::DoorDef,
    pub pos_at: V3,
    pub frame_centre: V3,
}
impl Door {
    /// what a new one holds
    pub fn blank() -> Self {
        Door {
            i: 0.0,
            key_side: 0.0,
            lock_id: String::new(),
            height: 0.0,
            thick: 0.0,
            min_auth: 0.0,
            max_auth: 0.0,
            drive_cols: DriveColsId(0),
            box_: BoxId(0),
            handle_boxes: Vec::new(),
            slab_row: SlabRowId(0),
            leaf_h: [0.0; 3],
            leaf_marks: None,
            def: crate::doors::DoorDef::default(),
            pos_at: [0.0, 1.0, 0.0],
            frame_centre: [0.0; 3],
        }
    }
}
impl Default for Door { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorSwing {
    pub hinge: V3,
    pub handle: V3,
    pub swing: f64,
    pub into: [f64; 2],
    pub open: f64,
    pub radius: f64,
    pub closed_dir: f64,
    pub one_way: bool,
}

#[derive(Clone, Debug, ToBytes)]
pub struct Doors {
    pub list: Vec<DoorId>,
    pub store: crate::sim_doorshome::DoorStore,
    pub slabs: crate::sim_slabshome::SlabStore,
    pub rounds: f64,
    pub solids: crate::doors::DoorsSolids,
}
impl Doors {
    /// what a new one holds
    pub fn blank() -> Self {
        Doors {
            list: Vec::new(),
            store: crate::sim_doorshome::DoorStore::blank(),
            slabs: crate::sim_slabshome::SlabStore::blank(),
            rounds: 0.0,
            solids: crate::doors::DoorsSolids { carried: 0.0, stopped: 0.0, passed: 0.0, reached: 0.0, held: 0.0, aside: 0.0 },
        }
    }
}
impl Default for Doors { fn default() -> Self { Self::blank() } }

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SlabRowGK {
    T,
    A,
    Vx,
    Vz,
    By,
    Px,
    Py,
    Pz,
    Vy,
    Spin,
    Wedged,
    Tip,
    TipV,
    Ang,
    Bw,
    Bl,
    Bx,
}
impl SlabRowGK {
    pub const fn as_str(self) -> &'static str {
        match self {
            SlabRowGK::T => "t",
            SlabRowGK::A => "a",
            SlabRowGK::Vx => "vx",
            SlabRowGK::Vz => "vz",
            SlabRowGK::By => "by",
            SlabRowGK::Px => "px",
            SlabRowGK::Py => "py",
            SlabRowGK::Pz => "pz",
            SlabRowGK::Vy => "vy",
            SlabRowGK::Spin => "spin",
            SlabRowGK::Wedged => "wedged",
            SlabRowGK::Tip => "tip",
            SlabRowGK::TipV => "tipV",
            SlabRowGK::Ang => "ang",
            SlabRowGK::Bw => "bw",
            SlabRowGK::Bl => "bl",
            SlabRowGK::Bx => "bx",
        }
    }
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

/// (src/game/doors.ts:1056 DOORS_PICK_SECS)
pub const DOORS_PICK_SECS: f64 = 4.2;

/// (src/game/doors.ts:1061 DOORS_PICK_LEVEL)
pub const DOORS_PICK_LEVEL: f64 = 0.16;

/// (src/game/doors.ts:1061 DOORS_UNLOCK_LEVEL)
pub const DOORS_UNLOCK_LEVEL: f64 = 0.3;

/// (src/game/doors.ts:1061 DOORS_KEY_LEVEL)
pub const DOORS_KEY_LEVEL: f64 = 0.12;

/// (src/game/doors.ts:1183 DOORS_USE_REACH)
pub const DOORS_USE_REACH: f64 = 1.35;

/// (src/game/doors.ts:81 SIDE_KICK: a table nothing writes)
pub fn side_kick() -> crate::doors::SIDEKICK { crate::doors::SIDEKICK { past: 0.3, out: 0.55 } }

/// (src/game/doors.ts:85 PULL_VEL)
pub const PULL_VEL: f64 = 3.9;

/// (src/game/doors.ts:85 PULL_VEL_QUIET)
pub const PULL_VEL_QUIET: f64 = 2.2;

/// (src/game/doors.ts:85 PULL_SECS)
pub const PULL_SECS: f64 = 0.7;

/// (src/game/doors.ts:88 DRIVEN_SECS)
pub const DRIVEN_SECS: f64 = 0.25;

/// (src/game/doors.ts:102 SHUT_HOME)
pub const SHUT_HOME: f64 = 0.05;

/// (src/game/doors.ts:134 FLOOR_KEY)
pub const FLOOR_KEY: &str = "floor";

/// (src/game/doors.ts:208 MAN_SIDE)
pub const MAN_SIDE: f64 = 3.0;

/// (src/game/doors.ts:208 MAN_FOLDED)
pub const MAN_FOLDED: f64 = 4.0;

/// (src/game/doors.ts:208 MAN_BRACED)
pub const MAN_BRACED: f64 = 8.0;

/// (src/game/doors.ts:208 MAN_CALM)
pub const MAN_CALM: f64 = 16.0;

/// (src/game/doors.ts:208 MAN_HELD)
pub const MAN_HELD: f64 = 32.0;

/// (src/game/doors.ts:294 DRIVE_COLS: a table nothing writes)
pub fn drive_cols() -> Vec<f64> { alloc::vec![34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0] }

/// (src/game/doors.ts:526 FLOOR_Y0)
pub const FLOOR_Y0: f64 = 0.2;
