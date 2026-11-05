//! The types of `doors`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: DoorDef, src/game/doors.ts:104)
#[derive(Clone, Debug)]
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
impl DoorDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorDef {
            name: String::new(),
            hinge: [0.0; 2],
            closed_dir: 0.0,
            width: 0.0,
            height: None,
            thickness: None,
            min_angle: None,
            max_angle: None,
            angle: None,
            swing: None,
            albedo: None,
            closer: None,
            exterior: None,
            locked: None,
            key_side: None,
            lock: None,
            steel: None,
        }
    }
}
impl Default for DoorDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DoorDrive, src/game/doors.ts:204)
#[derive(Clone, Copy, Debug)]
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
impl DoorDrive {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorDrive {
            who: 0.0,
            dir: 0.0,
            kind: crate::doors::DoorDriveKind::Body,
            t: 0.0,
            from: 0.0,
            quiet: false,
            side: 0.0,
            rate: 0.0,
            px: 0.0,
            pz: 0.0,
            hit: 0.0,
        }
    }
}
impl Default for DoorDrive { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorDriveKind {
    Body,
    Hand,
    Boot,
    Swing,
}
impl DoorDriveKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorDriveKind::Body => "body",
            DoorDriveKind::Hand => "hand",
            DoorDriveKind::Boot => "boot",
            DoorDriveKind::Swing => "swing",
        }
    }
}

/// (TypeScript: DoorStrike, src/game/doors.ts:211)
#[derive(Clone, Copy, Debug)]
pub struct DoorStrike {
    pub door: DoorId,
    pub by: f64,
    pub tier: crate::doors::DoorStrikeTier,
    pub floor: bool,
    pub dir: [f64; 2],
}
impl DoorStrike {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorStrike {
            door: DoorId(0),
            by: 0.0,
            tier: crate::doors::DoorStrikeTier::Boot,
            floor: false,
            dir: [0.0; 2],
        }
    }
}
impl Default for DoorStrike { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorStrikeTier {
    Boot,
    Open,
    Slam,
}
impl DoorStrikeTier {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorStrikeTier::Boot => "boot",
            DoorStrikeTier::Open => "open",
            DoorStrikeTier::Slam => "slam",
        }
    }
}

/// (TypeScript: DoorContact, src/game/doors.ts:219)
#[derive(Clone, Debug)]
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
impl DoorContact {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorContact {
            pos: [0.0; 3],
            radius: 0.0,
            bash: false,
            who: 0.0,
            quiet: false,
            weak: None,
            key: None,
            keys: None,
            blocked_by: None,
            hand: None,
            round: None,
            run: None,
            bashed: None,
            ripped: None,
            bashed_at: None,
            struck: None,
            moved: None,
            man: None,
            solid: None,
            vel: None,
        }
    }
}
impl Default for DoorContact { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doors.ts:230 DoorContact.run, src/game/doors.ts:230)
#[derive(Clone, Copy, Debug)]
pub struct DoorContactRun {
    pub bash: f64,
    pub rip: f64,
    pub rip_level: f64,
}
impl DoorContactRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorContactRun {
            bash: 0.0,
            rip: 0.0,
            rip_level: 0.0,
        }
    }
}
impl Default for DoorContactRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorSound {
    Creak,
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
impl DoorSound {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorSound::Creak => "creak",
            DoorSound::Bang => "bang",
            DoorSound::Latch => "latch",
            DoorSound::Bash => "bash",
            DoorSound::Rattle => "rattle",
            DoorSound::Pick => "pick",
            DoorSound::Unlock => "unlock",
            DoorSound::Kick => "kick",
            DoorSound::Clang => "clang",
            DoorSound::Thud => "thud",
        }
    }
}

/// (TypeScript: DoorEvent, src/game/doors.ts:244)
#[derive(Clone, Copy, Debug)]
pub struct DoorEvent {
    pub door: DoorId,
    pub sound: crate::doors::DoorSound,
    pub pos: V3,
    pub who: f64,
    pub level: f64,
}
impl DoorEvent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorEvent {
            door: DoorId(0),
            sound: crate::doors::DoorSound::Creak,
            pos: [0.0; 3],
            who: 0.0,
            level: 0.0,
        }
    }
}
impl Default for DoorEvent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SlabSeen, src/game/doors.ts:251)
#[derive(Clone, Copy, Debug)]
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
impl SlabSeen {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlabSeen {
            p: [0.0; 3],
            v: [0.0; 3],
            a: 0.0,
            spin: 0.0,
            tip: 0.0,
            tip_v: 0.0,
            side: 0.0,
            rest: false,
            landed: false,
            who: 0.0,
            t: 0.0,
            ang: 0.0,
            bits: None,
            wedged: None,
        }
    }
}
impl Default for SlabSeen { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SlabRow, src/game/doors.ts:255)
#[derive(Clone, Copy, Debug)]
pub struct SlabRow {
    pub door: f64,
}
impl SlabRow {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        SlabRow {
            door: 0.0,
        }
    }
}
impl Default for SlabRow { fn default() -> Self { Self::blank() } }

/// (TypeScript: DriveCols, src/game/doors.ts:276)
#[derive(Clone, Copy, Debug)]
pub struct DriveCols {
    pub i: f64,
}
impl DriveCols {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        DriveCols {
            i: 0.0,
        }
    }
}
impl Default for DriveCols { fn default() -> Self { Self::blank() } }

/// (TypeScript: Door, src/game/doors.ts:298)
#[derive(Clone, Debug)]
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
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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
            def: crate::doors::DoorDef::zeroed(),
            pos_at: [0.0, 1.0, 0.0],
            frame_centre: [0.0; 3],
        }
    }
}
impl Default for Door { fn default() -> Self { Self::blank() } }

/// (TypeScript: DoorSwing, src/game/doors.ts:500)
#[derive(Clone, Copy, Debug)]
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
impl DoorSwing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorSwing {
            hinge: [0.0; 3],
            handle: [0.0; 3],
            swing: 0.0,
            into: [0.0; 2],
            open: 0.0,
            radius: 0.0,
            closed_dir: 0.0,
            one_way: false,
        }
    }
}
impl Default for DoorSwing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DoorsHost, src/game/doors.ts:518)
#[derive(Clone, Copy, Debug)]
pub struct DoorsHost {
}
impl DoorsHost {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorsHost {
        }
    }
}
impl Default for DoorsHost { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Doors, src/game/doors.ts:528)
#[derive(Clone, Debug)]
pub struct Doors {
    pub list: Vec<DoorId>,
    pub store: crate::sim_doorshome::DoorStore,
    pub slabs: crate::sim_slabshome::SlabStore,
    pub rounds: f64,
    pub solids: crate::doors::DoorsSolids,
}
impl Doors {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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

/// (TypeScript: inline at src/game/doors.ts:588 Doors.solids, src/game/doors.ts:588)
#[derive(Clone, Copy, Debug)]
pub struct DoorsSolids {
    pub carried: f64,
    pub stopped: f64,
    pub passed: f64,
    pub reached: f64,
    pub held: f64,
    pub aside: f64,
}
impl DoorsSolids {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorsSolids {
            carried: 0.0,
            stopped: 0.0,
            passed: 0.0,
            reached: 0.0,
            held: 0.0,
            aside: 0.0,
        }
    }
}
impl Default for DoorsSolids { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doors.ts:81 SIDE_KICK, src/game/doors.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct SIDEKICK {
    pub past: f64,
    pub out: f64,
}
impl SIDEKICK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SIDEKICK {
            past: 0.0,
            out: 0.0,
        }
    }
}
impl Default for SIDEKICK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SlabRowGK {
    T,
    A,
    Vx,
    Vz,
    By,
    Spin,
    Px,
    Py,
    Pz,
    Vy,
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
            SlabRowGK::Spin => "spin",
            SlabRowGK::Px => "px",
            SlabRowGK::Py => "py",
            SlabRowGK::Pz => "pz",
            SlabRowGK::Vy => "vy",
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DriveColsKind {
    Body,
    Hand,
    Boot,
    Swing,
}
impl DriveColsKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DriveColsKind::Body => "body",
            DriveColsKind::Hand => "hand",
            DriveColsKind::Boot => "boot",
            DriveColsKind::Swing => "swing",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DriveColsSetKindV {
    Body,
    Hand,
    Boot,
    Swing,
}
impl DriveColsSetKindV {
    pub const fn as_str(self) -> &'static str {
        match self {
            DriveColsSetKindV::Body => "body",
            DriveColsSetKindV::Hand => "hand",
            DriveColsSetKindV::Boot => "boot",
            DriveColsSetKindV::Swing => "swing",
        }
    }
}

/// (TypeScript: inline at src/game/doors.ts:472 poseAt ->, src/game/doors.ts:472)
#[derive(Clone, Copy, Debug)]
pub struct DoorPoseAt {
    pub c: V3,
    pub yaw: f64,
    pub handles: [V3; 2],
}
impl DoorPoseAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorPoseAt {
            c: [0.0; 3],
            yaw: 0.0,
            handles: [[0.0; 3]; 2],
        }
    }
}
impl Default for DoorPoseAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorsDrivenKind {
    Body,
    Hand,
    Boot,
    Swing,
}
impl DoorsDrivenKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorsDrivenKind::Body => "body",
            DoorsDrivenKind::Hand => "hand",
            DoorsDrivenKind::Boot => "boot",
            DoorsDrivenKind::Swing => "swing",
        }
    }
}

/// (TypeScript: inline at src/game/doors.ts:1071 workSpot ->, src/game/doors.ts:1071)
#[derive(Clone, Copy, Debug)]
pub struct DoorsWorkSpot {
    pub pos: V3,
    pub yaw: f64,
}
impl DoorsWorkSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorsWorkSpot {
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for DoorsWorkSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doors.ts:1077 kickSpot ->, src/game/doors.ts:1077)
#[derive(Clone, Copy, Debug)]
pub struct DoorsKickSpot {
    pub pos: V3,
    pub yaw: f64,
}
impl DoorsKickSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorsKickSpot {
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for DoorsKickSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doors.ts:1084 kickSpotSide ->, src/game/doors.ts:1084)
#[derive(Clone, Copy, Debug)]
pub struct DoorsKickSpotSide {
    pub pos: V3,
    pub yaw: f64,
}
impl DoorsKickSpotSide {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorsKickSpotSide {
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for DoorsKickSpotSide { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doors.ts:1091 frameCoords ->, src/game/doors.ts:1091)
#[derive(Clone, Copy, Debug)]
pub struct DoorsFrameCoords {
    pub along: f64,
    pub off: f64,
}
impl DoorsFrameCoords {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorsFrameCoords {
            along: 0.0,
            off: 0.0,
        }
    }
}
impl Default for DoorsFrameCoords { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorsOnTornFnCause {
    Blast,
    Fire,
}
impl DoorsOnTornFnCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorsOnTornFnCause::Blast => "blast",
            DoorsOnTornFnCause::Fire => "fire",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorsTornCause {
    Blast,
    Fire,
}
impl DoorsTornCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorsTornCause::Blast => "blast",
            DoorsTornCause::Fire => "fire",
        }
    }
}

/// (src/game/doors.ts:76 SOLID_SLIDE)
pub const SOLID_SLIDE: f64 = 2.2;

/// (src/game/doors.ts:76 SOLID_TRIES)
pub const SOLID_TRIES: f64 = 3.0;

/// (src/game/doors.ts:76 SOLID_FADE)
pub const SOLID_FADE: f64 = 0.5;

/// (src/game/doors.ts:76 SOLID_SKIN)
pub const SOLID_SKIN: f64 = 0.02;

/// (src/game/doors.ts:76 SOLID_RATE)
pub const SOLID_RATE: f64 = 18.0;

/// (src/game/doors.ts:76 SOLID_REACH)
pub const SOLID_REACH: f64 = 0.6;

/// (src/game/doors.ts:76 SOLID_INTO)
pub const SOLID_INTO: f64 = 0.4;

/// (src/game/doors.ts:76 SOLID_BACK)
pub const SOLID_BACK: f64 = 3.0;

/// (src/game/doors.ts:81 SIDE_KICK: a table nothing writes)
pub fn side_kick() -> crate::doors::SIDEKICK { crate::doors::SIDEKICK { past: 0.3, out: 0.55 } }

/// (src/game/doors.ts:85 PULL_VEL)
pub const PULL_VEL: f64 = 3.9;

/// (src/game/doors.ts:85 PULL_VEL_QUIET)
pub const PULL_VEL_QUIET: f64 = 2.2;

/// (src/game/doors.ts:85 PULL_SECS)
pub const PULL_SECS: f64 = 0.7;

/// (src/game/doors.ts:88 PRESS_SECS)
pub const PRESS_SECS: f64 = 0.08;

/// (src/game/doors.ts:88 DRIVEN_SECS)
pub const DRIVEN_SECS: f64 = 0.25;

/// (src/game/doors.ts:88 DRIVEN_VEL)
pub const DRIVEN_VEL: f64 = 0.5;

/// (src/game/doors.ts:93 STOP_BODY)
pub const STOP_BODY: f64 = 0.3;

/// (src/game/doors.ts:97 ROUND_SLIDE)
pub const ROUND_SLIDE: f64 = 1.5;

/// (src/game/doors.ts:97 ROUND_SECS)
pub const ROUND_SECS: f64 = 1.2;

/// (src/game/doors.ts:97 ROUND_STILL)
pub const ROUND_STILL: f64 = 1.0;

/// (src/game/doors.ts:100 SHOVE_OPENS_BELOW)
pub const SHOVE_OPENS_BELOW: f64 = 1.2;

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
pub fn drive_cols() -> Vec<f64> { alloc::vec![37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0, 47.0] }

/// (src/game/doors.ts:486 SHOVE_RATE)
pub const SHOVE_RATE: f64 = 7.2;

/// (src/game/doors.ts:488 TURN_RATE)
pub const TURN_RATE: f64 = 15.0;

/// (src/game/doors.ts:494 KICK_SUBSTEP)
pub const KICK_SUBSTEP: f64 = 0.16;

/// (src/game/doors.ts:526 FLOOR_Y0)
pub const FLOOR_Y0: f64 = 0.2;
