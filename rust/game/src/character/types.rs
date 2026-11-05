//! The types of `character`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// THE NAMES OF A BODY'S JOINTS, in the order a real game's `bones` has them (read off a running game: `for k in b` walks this order)
pub const BONE_ORDER: [Bone; 22] = [Bone::Hips, Bone::Spine2, Bone::Chest, Bone::Neck, Bone::Head, Bone::UpperArmL, Bone::ForearmL, Bone::HandL, Bone::UpperArmR, Bone::ForearmR, Bone::HandR, Bone::ThighL, Bone::ShinL, Bone::FootL, Bone::ToeL, Bone::ThighR, Bone::ShinR, Bone::FootR, Bone::ToeR, Bone::FingerR, Bone::FingerL, Bone::HeadTop];
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Bone { Hips, Spine2, Chest, Neck, Head, UpperArmL, ForearmL, HandL, UpperArmR, ForearmR, HandR, ThighL, ShinL, FootL, ToeL, ThighR, ShinR, FootR, ToeR, FingerR, FingerL, HeadTop }
impl Bone {
    pub const fn as_str(self) -> &'static str {
        match self { Bone::Hips => "hips", Bone::Spine2 => "spine2", Bone::Chest => "chest", Bone::Neck => "neck", Bone::Head => "head", Bone::UpperArmL => "upperArmL", Bone::ForearmL => "forearmL", Bone::HandL => "handL", Bone::UpperArmR => "upperArmR", Bone::ForearmR => "forearmR", Bone::HandR => "handR", Bone::ThighL => "thighL", Bone::ShinL => "shinL", Bone::FootL => "footL", Bone::ToeL => "toeL", Bone::ThighR => "thighR", Bone::ShinR => "shinR", Bone::FootR => "footR", Bone::ToeR => "toeR", Bone::FingerR => "fingerR", Bone::FingerL => "fingerL", Bone::HeadTop => "headTop" }
    }
}
/// whose body this is
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Who { Guard(GuardId), Player(PlayerId) }

/// THE BLOW'S HANDS FOLLOW HIS MEN (takedown.ts: `live: () => marks(by, vic, style)`): the only closure ever given to it
#[derive(Clone, Copy, Debug)]
pub struct BlowLive { pub by: CharId, pub vic: CharId, pub style: BlowStyle }

/// THE TRUNK'S PATH IN A GUIDED FALL (character.ts `led`): the pelvis led over [p0, p1], the turn over [n0, n1]
#[derive(Clone, Copy, Debug)]
pub struct Led { pub pv: V3, pub p_t: V3, pub v_t: V3, pub p0: f64, pub p1: f64, pub n0: f64, pub n1: f64, pub v0n: V3 }
/// `frame` of a guided fall, by the fall's kind (the closures of character.ts's guided fall, with what they captured)
#[derive(Clone, Copy, Debug)]
pub enum FallFrame {
    /// 'fold' and 'sink': onto his knees, then laid on his front
    Fold { kneel: Led, t_a: f64, lay: f64, k62: V3, flat: V3, roll: f64, sd: V3, f: V3, room: f64, l_a: f64, v0n: V3 },
    /// 'cut': the beat he stands, sagging, then the sit
    Cut { sit: Led, beat: f64 },
    /// 'topple': about his heels, gathering speed
    Topple { a: V3, r0: V3, k: V3, top: f64, dur: f64, ty: f64, tw: f64 },
    /// any other: `led` alone
    Plain(Led),
}
/// `nod` of a guided fall: the head's nod, by the fall's kind
#[derive(Clone, Copy, Debug)]
pub enum FallNod {
    Fold { slow: bool, t_a: f64, lay: f64 },
    Cut { beat: f64, dur: f64, end: f64 },
    /// sign of the snap back: away·f < 0 ? 1 : -1
    Topple { sign: f64 },
    Plain,
}
/// `at` of a pin of a guided fall: where the pin wants the particle at time t. All but Lead read the guide's `frame`
#[derive(Clone, Copy, Debug)]
pub enum GuidePinAt {
    /// `rigid`: carried with the trunk
    Rigid { r: V3 },
    /// the head: carried with the trunk, turned by `nod`
    Head { v0: V3, nod_ax: V3, rh: V3 },
    /// a limb laid with him. `a` is the particle's place the first time it is asked (`a ??= P(pi)`)
    Lead { pi: usize, a: Option<V3>, e: V3, t_a: f64, lay: f64 },
    /// 'cut': a hand to the throat
    Throat { v0: V3, f: V3, lf: V3 },
    /// 'topple': a heel turning under him
    Heel { a: V3, r: V3, ty: f64, tw: f64, dur: f64 },
}
/// `release` of a guided fall ('fold' and 'sink' have none)
#[derive(Clone, Copy, Debug)]
pub enum FallRelease {
    Cut { away: V3, room: f64 },
    Topple { away: V3, room: f64, j: f64, k: V3, w: f64 },
    Plain { sd: V3 },
}
impl Spring {
    /// `new Spring(x)`
    pub fn at(x: f64) -> Self { let mut s = Self::blank(); s.x = x; s }
}
/// the inverse of the chest twist, for a point (character.ts untwist): nothing to undo, or the conjugates and pivots worked out when it was asked
#[derive(Clone, Debug)]
pub enum Untwist {
    Same,
    Back { i_e1: Quat, i_e2: Quat, i_e3: Quat, p1: V3, p2t: V3, p3t: V3 },
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitItem {
    Pistol,
    Rifle,
    Can,
    Knife,
}
impl KitItem {
    pub const fn as_str(self) -> &'static str {
        match self {
            KitItem::Pistol => "pistol",
            KitItem::Rifle => "rifle",
            KitItem::Can => "can",
            KitItem::Knife => "knife",
        }
    }
}

/// (TypeScript: character.ts N: the index of each named node of the rig)
#[derive(Clone, Copy, Debug)]
pub struct AnimNodes {
    pub hips: usize,
    pub spine1: usize,
    pub spine2: usize,
    pub spine3: usize,
    pub neck: usize,
    pub head: usize,
    pub upper_arm_l: usize,
    pub forearm_l: usize,
    pub hand_l: usize,
    pub upper_arm_r: usize,
    pub forearm_r: usize,
    pub hand_r: usize,
    pub thigh_l: usize,
    pub shin_l: usize,
    pub foot_l: usize,
    pub toe_l: usize,
    pub thigh_r: usize,
    pub shin_r: usize,
    pub foot_r: usize,
    pub toe_r: usize,
    pub shoulder_l: usize,
    pub shoulder_r: usize,
    pub middle1_r: usize,
    pub middle1_l: usize,
}
impl AnimNodes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimNodes {
            hips: 0,
            spine1: 0,
            spine2: 0,
            spine3: 0,
            neck: 0,
            head: 0,
            upper_arm_l: 0,
            forearm_l: 0,
            hand_l: 0,
            upper_arm_r: 0,
            forearm_r: 0,
            hand_r: 0,
            thigh_l: 0,
            shin_l: 0,
            foot_l: 0,
            toe_l: 0,
            thigh_r: 0,
            shin_r: 0,
            foot_r: 0,
            toe_r: 0,
            shoulder_l: 0,
            shoulder_r: 0,
            middle1_r: 0,
            middle1_l: 0,
        }
    }
}
impl Default for AnimNodes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UpperMode {
    None,
    Torch,
    Relaxed,
    Aim,
}
impl UpperMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            UpperMode::None => "none",
            UpperMode::Torch => "torch",
            UpperMode::Relaxed => "relaxed",
            UpperMode::Aim => "aim",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Stance {
    None,
    Wall,
    Stack,
    HighReady,
    LowReady,
}
impl Stance {
    pub const fn as_str(self) -> &'static str {
        match self {
            Stance::None => "none",
            Stance::Wall => "wall",
            Stance::Stack => "stack",
            Stance::HighReady => "highReady",
            Stance::LowReady => "lowReady",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LongStance {
    PatrolLong,
    LowLong,
    Shoulder,
    PortLong,
}
impl LongStance {
    pub const fn as_str(self) -> &'static str {
        match self {
            LongStance::PatrolLong => "patrolLong",
            LongStance::LowLong => "lowLong",
            LongStance::Shoulder => "shoulder",
            LongStance::PortLong => "portLong",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SignalKind {
    Hold,
    Go,
    Rally,
    Aside,
    Squeeze,
}
impl SignalKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SignalKind::Hold => "hold",
            SignalKind::Go => "go",
            SignalKind::Rally => "rally",
            SignalKind::Aside => "aside",
            SignalKind::Squeeze => "squeeze",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TellKind {
    Watch,
    Neck,
}
impl TellKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            TellKind::Watch => "watch",
            TellKind::Neck => "neck",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StrikeKind {
    Cross,
    Jab,
}
impl StrikeKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            StrikeKind::Cross => "cross",
            StrikeKind::Jab => "jab",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BlowStyle {
    Mouth,
    Sweep,
    Slash,
    Chop,
    Palm,
}
impl BlowStyle {
    pub const fn as_str(self) -> &'static str {
        match self {
            BlowStyle::Mouth => "mouth",
            BlowStyle::Sweep => "sweep",
            BlowStyle::Slash => "slash",
            BlowStyle::Chop => "chop",
            BlowStyle::Palm => "palm",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FallStyle {
    Fold,
    Topple,
    Sink,
    Cut,
    Slump,
}
impl FallStyle {
    pub const fn as_str(self) -> &'static str {
        match self {
            FallStyle::Fold => "fold",
            FallStyle::Topple => "topple",
            FallStyle::Sink => "sink",
            FallStyle::Cut => "cut",
            FallStyle::Slump => "slump",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TakenStyle {
    Arch,
    Hunch,
    Snap,
}
impl TakenStyle {
    pub const fn as_str(self) -> &'static str {
        match self {
            TakenStyle::Arch => "arch",
            TakenStyle::Hunch => "hunch",
            TakenStyle::Snap => "snap",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ReadyKind {
    Grab,
    Seat,
    Strike,
    GrabGun,
}
impl ReadyKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ReadyKind::Grab => "grab",
            ReadyKind::Seat => "seat",
            ReadyKind::Strike => "strike",
            ReadyKind::GrabGun => "grabGun",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HoldPhase {
    Held,
    Grab,
    Choke,
    Whip,
    Kill,
}
impl HoldPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            HoldPhase::Held => "held",
            HoldPhase::Grab => "grab",
            HoldPhase::Choke => "choke",
            HoldPhase::Whip => "whip",
            HoldPhase::Kill => "kill",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HoldVariant {
    Gun,
    Arm,
}
impl HoldVariant {
    pub const fn as_str(self) -> &'static str {
        match self {
            HoldVariant::Gun => "gun",
            HoldVariant::Arm => "arm",
        }
    }
}

/// (TypeScript: HoldPose, src/game/character.ts:60)
#[derive(Clone, Copy, Debug)]
pub struct HoldPose {
    pub yank: Option<f64>,
    pub step: Option<f64>,
    pub man: Option<V3>,
    pub pull: Option<f64>,
    pub phase: crate::character::HoldPhase,
    pub t: f64,
    pub variant: Option<crate::character::HoldVariant>,
    pub gun_out: Option<f64>,
    pub gun_pitch: Option<f64>,
    pub low: Option<f64>,
    pub kill: Option<bool>,
    pub beats: Option<bool>,
    pub haul: Option<bool>,
    pub strip: Option<crate::character::StripPose>,
}
impl HoldPose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldPose {
            yank: None,
            step: None,
            man: None,
            pull: None,
            phase: crate::character::HoldPhase::Held,
            t: 0.0,
            variant: None,
            gun_out: None,
            gun_pitch: None,
            low: None,
            kill: None,
            beats: None,
            haul: None,
            strip: None,
        }
    }
}
impl Default for HoldPose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: StripPose, src/game/character.ts:61)
#[derive(Clone, Copy, Debug)]
pub struct StripPose {
    pub t: f64,
    pub at: Option<V3>,
}
impl StripPose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StripPose {
            t: 0.0,
            at: None,
        }
    }
}
impl Default for StripPose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: HeldPose, src/game/character.ts:65)
#[derive(Clone, Copy, Debug)]
pub struct HeldPose {
    pub yank: Option<f64>,
    pub pull: Option<f64>,
    pub phase: crate::character::HoldPhase,
    pub t: f64,
    pub agitation: f64,
    pub variant: Option<crate::character::HoldVariant>,
    pub bare: Option<bool>,
    pub shot: Option<f64>,
    pub armed: Option<bool>,
    pub low: Option<f64>,
    pub kill: Option<bool>,
    pub beats: Option<bool>,
    pub haul: Option<bool>,
    pub strip: Option<crate::character::StripPose>,
}
impl HeldPose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HeldPose {
            yank: None,
            pull: None,
            phase: crate::character::HoldPhase::Held,
            t: 0.0,
            agitation: 0.0,
            variant: None,
            bare: None,
            shot: None,
            armed: None,
            low: None,
            kill: None,
            beats: None,
            haul: None,
            strip: None,
        }
    }
}
impl Default for HeldPose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DoorPush, src/game/character.ts:70)
#[derive(Clone, Copy, Debug)]
pub struct DoorPush {
    pub p: V3,
    pub n: V3,
    pub w: f64,
    pub hand: f64,
    pub hold_only: Option<bool>,
    pub reach: Option<bool>,
}
impl DoorPush {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorPush {
            p: [0.0; 3],
            n: [0.0; 3],
            w: 0.0,
            hand: 0.0,
            hold_only: None,
            reach: None,
        }
    }
}
impl Default for DoorPush { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WorkKind {
    Keys,
    Charge,
    Take,
    Lever,
}
impl WorkKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WorkKind::Keys => "keys",
            WorkKind::Charge => "charge",
            WorkKind::Take => "take",
            WorkKind::Lever => "lever",
        }
    }
}

/// (TypeScript: WorkPose, src/game/character.ts:78)
#[derive(Clone, Debug)]
pub struct WorkPose {
    pub kind: crate::character::WorkKind,
    pub p: V3,
    pub n: V3,
    pub t: f64,
    pub dur: f64,
    pub keys: Option<Vec<[f64; 2]>>,
    pub dir: Option<f64>,
    pub hand: Option<f64>,
}
impl WorkPose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkPose {
            kind: crate::character::WorkKind::Keys,
            p: [0.0; 3],
            n: [0.0; 3],
            t: 0.0,
            dur: 0.0,
            keys: None,
            dir: None,
            hand: None,
        }
    }
}
impl Default for WorkPose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CarryKind2 {
    Torch,
    Weapon,
}
impl CarryKind2 {
    pub const fn as_str(self) -> &'static str {
        match self {
            CarryKind2::Torch => "torch",
            CarryKind2::Weapon => "weapon",
        }
    }
}

/// (TypeScript: HaulPose, src/game/character.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct HaulPose {
    pub l: V3,
    pub r: V3,
    pub bow: f64,
    pub grip: f64,
    pub rw: Option<f64>,
    pub look: Option<f64>,
    pub tilt: Option<f64>,
}
impl HaulPose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HaulPose {
            l: [0.0; 3],
            r: [0.0; 3],
            bow: 0.0,
            grip: 0.0,
            rw: None,
            look: None,
            tilt: None,
        }
    }
}
impl Default for HaulPose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AnimParams, src/game/character.ts:353)
#[derive(Clone, Copy, Debug)]
pub struct AnimParams {
    pub walk_speed: f64,
    pub jog_speed: f64,
    pub crouch_speed: f64,
    pub sprint_speed: f64,
}
impl AnimParams {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimParams {
            walk_speed: 0.0,
            jog_speed: 0.0,
            crouch_speed: 0.0,
            sprint_speed: 0.0,
        }
    }
}
impl Default for AnimParams { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Spring, src/game/character.ts:458)
#[derive(Clone, Copy, Debug)]
pub struct Spring {
    pub v: f64,
    pub x: f64,
}
impl Spring {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Spring {
            v: 0.0,
            x: 0.0,
        }
    }
}
impl Default for Spring { fn default() -> Self { Self::blank() } }

/// (TypeScript: StanceShape, src/game/character.ts:515)
#[derive(Clone, Copy, Debug)]
pub struct StanceShape {
    pub fwd: f64,
    pub up: f64,
    pub side: f64,
    pub pitch: f64,
    pub lean: f64,
    pub head_pitch: f64,
    pub tuck: f64,
    pub blade: f64,
    pub head_yaw: f64,
    pub squat: f64,
}
impl StanceShape {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StanceShape {
            fwd: 0.0,
            up: 0.0,
            side: 0.0,
            pitch: 0.0,
            lean: 0.0,
            head_pitch: 0.0,
            tuck: 0.0,
            blade: 0.0,
            head_yaw: 0.0,
            squat: 0.0,
        }
    }
}
impl Default for StanceShape { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LongShape, src/game/character.ts:615)
#[derive(Clone, Copy, Debug)]
pub struct LongShape {
    pub side: f64,
    pub up: f64,
    pub fwd: f64,
    pub anchor: f64,
    pub pitch: f64,
    pub aim_k: f64,
    pub yaw: f64,
    pub blade: f64,
    pub lean: f64,
    pub head_yaw: f64,
    pub head_pitch: f64,
    pub head_roll: f64,
    pub elbow_r: V3,
    pub elbow_l: V3,
    pub pole_w: f64,
    pub sway: f64,
    pub steady: f64,
}
impl LongShape {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LongShape {
            side: 0.0,
            up: 0.0,
            fwd: 0.0,
            anchor: 0.0,
            pitch: 0.0,
            aim_k: 0.0,
            yaw: 0.0,
            blade: 0.0,
            lean: 0.0,
            head_yaw: 0.0,
            head_pitch: 0.0,
            head_roll: 0.0,
            elbow_r: [0.0; 3],
            elbow_l: [0.0; 3],
            pole_w: 0.0,
            sway: 0.0,
            steady: 0.0,
        }
    }
}
impl Default for LongShape { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ReloadBeat, src/game/character.ts:815)
#[derive(Clone, Copy, Debug)]
pub struct ReloadBeat {
    pub dur: f64,
    pub out: Option<f64>,
    pub in_: Option<f64>,
    pub rack: Option<f64>,
    pub keep: Option<f64>,
    pub hand: Option<bool>,
    pub exact: Option<bool>,
}
impl ReloadBeat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReloadBeat {
            dur: 0.0,
            out: None,
            in_: None,
            rack: None,
            keep: None,
            hand: None,
            exact: None,
        }
    }
}
impl Default for ReloadBeat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: MvGait, src/game/character.ts:911)
#[derive(Clone, Copy, Debug)]
pub struct MvGait {
    pub live: bool,
    pub g: f64,
    pub u: f64,
    pub st: f64,
}
impl MvGait {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MvGait {
            live: false,
            g: 0.0,
            u: 0.0,
            st: 0.0,
        }
    }
}
impl Default for MvGait { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ReadyHand, src/game/character.ts:1026)
#[derive(Clone, Copy, Debug)]
pub struct ReadyHand {
    pub pole: V3,
    pub fing: V3,
    pub back: V3,
    pub curl: f64,
    pub thumb: f64,
}
impl ReadyHand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReadyHand {
            pole: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
            curl: 0.0,
            thumb: 0.0,
        }
    }
}
impl Default for ReadyHand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ReadyShape, src/game/character.ts:1027)
#[derive(Clone, Copy, Debug)]
pub struct ReadyShape {
    pub lean: f64,
    pub blade: f64,
    pub clav: f64,
    pub l: crate::character::ReadyShapeL,
    pub knife: bool,
}
impl ReadyShape {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReadyShape {
            lean: 0.0,
            blade: 0.0,
            clav: 0.0,
            l: crate::character::ReadyShapeL::zeroed(),
            knife: false,
        }
    }
}
impl Default for ReadyShape { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1029 ReadyShape.L, src/game/character.ts:1029 ReadyShape.L)
#[derive(Clone, Copy, Debug)]
pub struct ReadyShapeL {
    pub pole: V3,
    pub fing: V3,
    pub back: V3,
    pub curl: f64,
    pub thumb: f64,
    pub spread: f64,
    pub reach: Option<crate::character::ReadyShapeLReach>,
    pub at: Option<V3>,
}
impl ReadyShapeL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReadyShapeL {
            pole: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
            curl: 0.0,
            thumb: 0.0,
            spread: 0.0,
            reach: None,
            at: None,
        }
    }
}
impl Default for ReadyShapeL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1029 ReadyShapeL.reach, src/game/character.ts:1029)
#[derive(Clone, Copy, Debug)]
pub struct ReadyShapeLReach {
    pub frac: f64,
    pub r: [f64; 2],
    pub dy: [f64; 2],
    pub bias: f64,
    pub h0: f64,
    pub y0: f64,
}
impl ReadyShapeLReach {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReadyShapeLReach {
            frac: 0.0,
            r: [0.0; 2],
            dy: [0.0; 2],
            bias: 0.0,
            h0: 0.0,
            y0: 0.0,
        }
    }
}
impl Default for ReadyShapeLReach { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BlowKey, src/game/character.ts:1063)
#[derive(Clone, Copy, Debug)]
pub struct BlowKey {
    pub t: f64,
    pub at: V3,
    pub blade: V3,
    pub fing: V3,
    pub pole: V3,
}
impl BlowKey {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlowKey {
            t: 0.0,
            at: [0.0; 3],
            blade: [0.0; 3],
            fing: [0.0; 3],
            pole: [0.0; 3],
        }
    }
}
impl Default for BlowKey { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BlowRun, src/game/character.ts:1140)
#[derive(Clone, Debug)]
pub struct BlowRun {
    pub style: crate::character::BlowStyle,
    pub t: f64,
    pub hit: f64,
    pub end: f64,
    pub tail: f64,
    pub rise: bool,
    pub at: V3,
    pub at2: Option<V3>,
    pub live: Option<crate::character::BlowLive>,
}
impl BlowRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlowRun {
            style: crate::character::BlowStyle::Mouth,
            t: 0.0,
            hit: 0.0,
            end: 0.0,
            tail: 0.0,
            rise: false,
            at: [0.0; 3],
            at2: None,
            live: None,
        }
    }
}
impl Default for BlowRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BlowOpts, src/game/character.ts:1141)
#[derive(Clone, Debug)]
pub struct BlowOpts {
    pub hit: Option<f64>,
    pub end: Option<f64>,
    pub live: Option<crate::character::BlowLive>,
    pub rise: Option<bool>,
    pub tail: Option<bool>,
}
impl BlowOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlowOpts {
            hit: None,
            end: None,
            live: None,
            rise: None,
            tail: None,
        }
    }
}
impl Default for BlowOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CharacterAnimator, src/game/character.ts:1427)
#[derive(Clone, Debug)]
pub struct Anim {
    pub pose: crate::anim_skeleton::Pose,
    pub p_a: crate::anim_skeleton::Pose,
    pub p_b: crate::anim_skeleton::Pose,
    pub p_c: crate::anim_skeleton::Pose,
    pub p_upper: crate::anim_skeleton::Pose,
    pub rest: crate::anim_skeleton::Pose,
    pub p_aim: crate::anim_skeleton::Pose,
    pub twists: Vec<crate::anim_skeleton::TwistSpec>,
    pub params: crate::character::AnimParams,
    pub speed: f64,
    pub reverse: bool,
    pub crouch_target: f64,
    pub upper: crate::character::UpperMode,
    pub aim_yaw_offset: f64,
    pub aim_pitch: f64,
    pub look_yaw_extra: f64,
    pub look_pitch_extra: f64,
    pub turn_lead: f64,
    pub stance: crate::character::Stance,
    pub stack_side: f64,
    pub wall_yaw: f64,
    pub wall_peek: f64,
    pub lean: f64,
    pub lean_sm: f64,
    pub wall_take: f64,
    pub wall_hit: f64,
    pub stance_out: f64,
    pub stack_press: f64,
    pub stance_in: f64,
    pub stack_press_sm: f64,
    pub wall_gait: f64,
    pub wtk_hold: bool,
    pub wall_take_feet: Option<crate::character::AnimWallTakeFeet>,
    pub wtk: Option<crate::character::AnimWtk>,
    pub lockpick: bool,
    pub lockpick_at: V3,
    pub deliberate: f64,
    pub watch_at: Option<V3>,
    pub watch_k: f64,
    pub slip_at: Option<V3>,
    pub slip_k: f64,
    pub slip_lean: f64,
    pub watch_shown: crate::character::AnimWatchShown,
    pub dash: f64,
    pub hold_pose: Option<crate::character::HoldPose>,
    pub held_pose: Option<crate::character::HeldPose>,
    pub door_push: Option<crate::character::DoorPush>,
    pub work: Option<crate::character::WorkPose>,
    pub carry: Option<crate::character::AnimCarry>,
    pub swap_lag: f64,
    pub haul: Option<crate::character::HaulPose>,
    pub long_gun: bool,
    pub long_stance: Option<crate::character::LongStance>,
    pub gun_wall: f64,
    pub run_ready: bool,
    pub long_grip: crate::character::AnimLongGrip,
    pub long_steady: f64,
    pub long_shown: Option<crate::character::LongStance>,
    pub hide_held_item: bool,
    pub stowed: bool,
    pub big_tool: Option<crate::character::AnimBigTool>,
    pub big_k: f64,
    pub big_clear: f64,
    pub big_side: Option<crate::character::AnimBigSide>,
    pub big_at_back: f64,
    pub big_grab: f64,
    pub hold_gun_side: bool,
    pub raw_clip: Option<crate::character::AnimRawClip>,
    pub act: Option<crate::character::AnimAct>,
    pub slump: f64,
    pub slump_w: f64,
    pub wound: Option<crate::character::AnimWound>,
    pub wd_t: f64,
    pub wd_w: f64,
    pub wd_clip: Option<ClipId>,
    pub wd_pose: String,
    pub act_t: f64,
    pub act_w: f64,
    pub act_clip: Option<ClipId>,
    pub act_name: String,
    pub act_out: f64,
    pub phase: f64,
    pub crouch: f64,
    pub upper_w: crate::character::AnimUpperW,
    pub tu_k: f64,
    pub shoot_t: f64,
    pub hit_t: f64,
    pub reload_t: f64,
    pub throw_t: f64,
    pub strike_t: f64,
    pub steady_mean: Option<[f64; 2]>,
    pub cr_walk: f64,
    pub steady_up: f64,
    pub snap_shot: bool,
    pub trained: f64,
    pub dazzle: Option<crate::character::AnimDazzle>,
    pub mvs: crate::character::AnimMvs,
    pub reload_back: bool,
    pub bound: f64,
    pub clutch: f64,
    pub dip: f64,
    pub hurt: f64,
    pub hurt_side: f64,
    pub shaken: f64,
    pub fold_too: bool,
    pub reload_mag: bool,
    pub reload_at: f64,
    pub reload_row: Option<crate::character::ReloadBeat>,
    pub feed: Option<crate::weapons::GunModelId>,
    pub feed_n: f64,
    pub feed_port: bool,
    pub rl_t: f64,
    pub rl_b: crate::character::AnimRlB,
    pub rl_src: Option<crate::character::ReloadBeat>,
    pub rlc: crate::character::AnimRlc,
    pub rl_shown: crate::character::AnimRlShown,
    pub kit: Option<crate::character::AnimKit>,
    pub kit_front: bool,
    pub kit_rest: f64,
    pub kit_hold: Option<crate::character::KitItem>,
    pub kit_stow: f64,
    pub kit_roll: f64,
    pub kit_lie: Option<Quat>,
    pub kit_gun_free: f64,
    pub close_k: f64,
    pub cl_k: f64,
    pub cl_t: f64,
    pub cl_l: f64,
    pub mv2: crate::character::AnimMv2,
    pub mv2_shown: crate::character::AnimMv2Shown,
    pub arms_pre: Option<crate::anim_skeleton::Pose>,
    pub arms_buf: Option<crate::anim_skeleton::Pose>,
    pub sights_lift: f64,
    pub kick_t: f64,
    pub kick_hit: bool,
    pub sig_t: f64,
    pub sig_kind: crate::character::SignalKind,
    pub sig_side: f64,
    pub tell_t: f64,
    pub tell_k: crate::character::TellKind,
    pub tell_shown: f64,
    pub strike_k: crate::character::StrikeKind,
    pub ready: Option<crate::character::ReadyKind>,
    pub ready_at: Option<V3>,
    pub ready_shown: f64,
    pub ready_up: f64,
    pub ready_amt: f64,
    pub rise: f64,
    pub low: f64,
    pub low_s: f64,
    pub low_shape: [f64; 3],
    pub wide: f64,
    pub wide_shape: [f64; 4],
    pub wide_pole: Option<V3>,
    pub wide_s: f64,
    pub tap_u: f64,
    pub tap_n: f64,
    pub tap_way: f64,
    pub tap_seen: f64,
    pub tap_from: f64,
    pub tap_to: f64,
    pub tap_was: f64,
    pub heft: f64,
    pub heft_shape: [f64; 6],
    pub hf: crate::character::AnimHf,
    pub hands_down: f64,
    pub spring: f64,
    pub coil: f64,
    pub creep: bool,
    pub creep_at: Option<V3>,
    pub creep_shown: f64,
    pub knife_shown: bool,
    pub reach_shown: f64,
    pub no_knife: bool,
    pub cr_t: f64,
    pub cr_v: f64,
    pub cr_pace: f64,
    pub cr_w: f64,
    pub cr_ph: Option<[f64; 2]>,
    pub bw: Option<crate::character::BlowRun>,
    pub tk: Option<crate::character::AnimTk>,
    pub kb_slash: bool,
    pub kb_back: bool,
    pub bw_rise: f64,
    pub rc_t: f64,
    pub strike_leth: bool,
    pub kb_r0: f64,
    pub kb_t: f64,
    pub rc_inv: Option<Quat>,
    pub rd_out_r: f64,
    pub rd_a: f64,
    pub hd_from: Option<V3>,
    pub hd_caught: bool,
    pub he_span: f64,
    pub rd_hand_l: Option<V3>,
    pub hd_fix: Option<crate::character::AnimHdFix>,
    pub hd_sw: f64,
    pub he_sw: f64,
    pub ready_w: f64,
    pub ready_k: crate::character::ReadyKind,
    pub rd_at: Option<V3>,
    pub rd_live: bool,
    pub rd: Vec<f64>,
    pub idle_t: f64,
    pub st: crate::character::AnimSt,
    pub wst: crate::character::AnimWst,
    pub stance_seen: crate::character::Stance,
    pub lp_w: crate::character::Spring,
    pub lp_t: f64,
    pub hl_w: crate::character::Spring,
    pub hl_last: Option<crate::character::HaulPose>,
    pub lg: crate::character::AnimLg,
    pub lg_armed_was: bool,
    pub hd_w: crate::character::Spring,
    pub hd_last: crate::character::HoldPose,
    pub hd_kn: crate::character::Spring,
    pub hd_gv: crate::character::Spring,
    pub hd_on: bool,
    pub hd_ck: crate::character::Spring,
    pub hd_lo: crate::character::Spring,
    pub hd_kc: crate::character::Spring,
    pub hd_hu: crate::character::Spring,
    pub he_hu: crate::character::Spring,
    pub he_w: crate::character::Spring,
    pub he_last: crate::character::HeldPose,
    pub he_ck: crate::character::Spring,
    pub he_gv: crate::character::Spring,
    pub he_on: bool,
    pub he_lo: crate::character::Spring,
    pub he_lk: crate::character::Spring,
    pub he_kc: crate::character::Spring,
    pub he_bo: crate::character::Spring,
    pub he_bc: crate::character::Spring,
    pub he_hr: [Option<Quat>; 2],
    pub dp: crate::character::AnimDp,
    pub wk: crate::character::AnimWk,
    pub sw: crate::character::AnimSw,
    pub hand_pre: Option<crate::anim_skeleton::Pose>,
    pub sw_buf: Option<crate::anim_skeleton::Pose>,
    pub hand_buf: Option<crate::anim_skeleton::Pose>,
    pub under_buf: Option<crate::anim_skeleton::Pose>,
    pub rx: crate::character::AnimRx,
    pub stealth: Option<crate::stealthpose::StealthFeed>,
    pub sp: Option<crate::character::AnimSp>,
    pub dlb: crate::character::Spring,
    pub gait: crate::character::AnimGait,
    pub dash_w: f64,
    pub wt: crate::character::AnimWt,
    pub jog_hips_y: Vec<f32>,
    pub jog_hips_mean: f64,
    pub jog_mid: f64,
    pub run_k: f64,
    pub sprint_mid: f64,
    pub sprint_floor_y: f64,
    pub jog_seam: Vec<crate::character::AnimJogSeamItem>,
    pub run_gait: crate::character::AnimRunGait,
    pub plant_l: crate::character::AnimPlantL,
    pub plant_r: crate::character::AnimPlantR,
    pub plant_w: f64,
    pub gz: crate::character::AnimGz,
    pub life: crate::character::AnimLife,
    pub feet: crate::character::AnimFeet,
    pub body_yaw_in: f64,
    pub thr_last: Option<crate::character::AnimThrLast>,
    pub thr_carry: Option<crate::character::AnimThrCarry>,
    pub thr_low: f64,
    pub thr_far: f64,
    pub thr_hang: f64,
    pub thr_hit0: f64,
    pub thr_off: bool,
    pub idle_feet: crate::character::AnimIdleFeet,
    pub kick_from: Option<crate::character::AnimKickFrom>,
    pub kick_leg: crate::character::AnimKickLeg,
    pub clips: JsMap<&'static str, ClipId>,
    pub upper_mask: Vec<f32>,
    pub arm_r_mask: Vec<f32>,
    pub head_mask: Vec<f32>,
    pub arms_mask: Vec<f32>,
    pub arm_r_only: Vec<f32>,
    pub torso_mask: Vec<f32>,
    pub fk: crate::anim_skeleton::PoseFK,
    pub hand_fix: Vec<Quat>,
    pub grip_up: V3,
    pub gaze_local: V3,
    pub chest_rest_inv: Quat,
    pub head_rest_inv: Quat,
    pub fingers: Vec<Vec<Vec<usize>>>,
    pub thumbs: Vec<Vec<usize>>,
    pub grip: [bool; 2],
    pub grip_can: bool,
    pub hn: crate::character::AnimHn,
    pub nodes: crate::character::AnimNodes,
    pub rig: RigId,
    pub nv: Option<crate::character::AnimNv>,
    pub nv_arm: f64,
    pub nv_nod: f64,
    pub nv_a: f64,
    pub nv_tw: [f64; 4],
    pub nv_shown: Option<f64>,
    pub smoke: Option<f64>,
    pub smoke_arm: f64,
    pub reload_dur: f64,
    pub act_loop: bool,
    pub mv_out: crate::character::MvGait,
    pub mv_po: crate::character::AnimMvPo,
    pub fk_legs: Vec<i32>,
    pub fk_arms: Vec<i32>,
    pub fk_head: Vec<i32>,
    pub cast: crate::anim_skeleton::LayerCast,
    pub hn_flat: Option<Vec<f64>>,
    pub hn_flat_of: f64,
    pub hd_idle: crate::character::AnimHdIdle,
    pub hd_busy: bool,
    pub big_buf: Option<crate::anim_skeleton::Pose>,
    pub big_under: bool,
}
impl Anim {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Anim {
            pose: crate::anim_skeleton::Pose::blank(),
            p_a: crate::anim_skeleton::Pose::blank(),
            p_b: crate::anim_skeleton::Pose::blank(),
            p_c: crate::anim_skeleton::Pose::blank(),
            p_upper: crate::anim_skeleton::Pose::blank(),
            rest: crate::anim_skeleton::Pose::blank(),
            p_aim: crate::anim_skeleton::Pose::blank(),
            twists: Vec::new(),
            params: crate::character::AnimParams { walk_speed: 1.06, jog_speed: 3.3, crouch_speed: 0.72, sprint_speed: 4.7 },
            speed: 0.0,
            reverse: false,
            crouch_target: 0.0,
            upper: crate::character::UpperMode::None,
            aim_yaw_offset: 0.0,
            aim_pitch: 0.0,
            look_yaw_extra: 0.0,
            look_pitch_extra: 0.0,
            turn_lead: 0.0,
            stance: crate::character::Stance::None,
            stack_side: 1.0,
            wall_yaw: 0.0,
            wall_peek: 0.0,
            lean: 0.0,
            lean_sm: 0.0,
            wall_take: -1.0,
            wall_hit: 0.0,
            stance_out: 0.0,
            stack_press: 0.0,
            stance_in: 0.0,
            stack_press_sm: 0.0,
            wall_gait: 0.55,
            wtk_hold: false,
            wall_take_feet: None,
            wtk: None,
            lockpick: false,
            lockpick_at: [0.02, 0.96, 0.45],
            deliberate: 0.0,
            watch_at: None,
            watch_k: 0.0,
            slip_at: None,
            slip_k: 0.0,
            slip_lean: 1.0,
            watch_shown: crate::character::AnimWatchShown { w: 0.0, yaw: 0.0, slip: 0.0, lean: 1.0 },
            dash: 0.0,
            hold_pose: None,
            held_pose: None,
            door_push: None,
            work: None,
            carry: None,
            swap_lag: 0.0,
            haul: None,
            long_gun: false,
            long_stance: None,
            gun_wall: 0.0,
            run_ready: false,
            long_grip: crate::character::AnimLongGrip { wrist: [0.1, 0.02, 0.24], fingers: [-0.6, 0.45, 0.65], back: [0.45, -0.85, -0.25] },
            long_steady: 0.0,
            long_shown: None,
            hide_held_item: false,
            stowed: false,
            big_tool: None,
            big_k: 0.0,
            big_clear: 0.0,
            big_side: None,
            big_at_back: 0.0,
            big_grab: 0.0,
            hold_gun_side: false,
            raw_clip: None,
            act: None,
            slump: 0.0,
            slump_w: 0.0,
            wound: None,
            wd_t: 0.0,
            wd_w: 0.0,
            wd_clip: None,
            wd_pose: String::from(""),
            act_t: 0.0,
            act_w: 0.0,
            act_clip: None,
            act_name: String::from(""),
            act_out: 0.35,
            phase: 0.0,
            crouch: 0.0,
            upper_w: crate::character::AnimUpperW { relaxed: 0.0, aim: 0.0, torch: 0.0 },
            tu_k: 0.0,
            shoot_t: -1.0,
            hit_t: -1.0,
            reload_t: -1.0,
            throw_t: -1.0,
            strike_t: -1.0,
            steady_mean: None,
            cr_walk: 0.0,
            steady_up: 0.0,
            snap_shot: false,
            trained: 0.5,
            dazzle: None,
            mvs: crate::character::AnimMvs { back: 0.0, climb: 0.0, yaw: 0.0, rock: 0.0, sho: 0.0, head: 0.0, n: 0.0, fl_t: -1.0, fl_k: 1.0, fl_k_to: 1.0, fl_soft: false, fl_d: [0.0, 0.0, 1.0], st_t: -1.0, st_d: [0.0, 0.0, 1.0], ht_t: -1.0, ht_d: [0.0, 0.0, -1.0], ht_side: 1.0, ts_t: -1.0, ts_rel: 0.14, arm_l: 0.0, dz: crate::character::Spring::blank(), dz_d: [0.0, 0.0, 1.0], dz_t: 0.0, dz_sd: 0.0, glide: crate::character::Spring::blank(), hip_y: f64::NAN, cf: crate::character::Spring::blank(), cf_t: 0.0, hu: crate::character::Spring::blank(), hu_t: 0.0, sk: crate::character::Spring::blank() },
            reload_back: false,
            bound: 0.0,
            clutch: 0.0,
            dip: 0.0,
            hurt: 0.0,
            hurt_side: -1.0,
            shaken: 0.0,
            fold_too: false,
            reload_mag: true,
            reload_at: -1.0,
            reload_row: None,
            feed: None,
            feed_n: 0.0,
            feed_port: false,
            rl_t: -1.0,
            rl_b: crate::character::AnimRlB { dur: 1.6, out: 0.43, in_: 1.04, rack: 1.36, pull: true, keep: -1.0, hand: false, fresh: 0.7 },
            rl_src: None,
            rlc: crate::character::AnimRlc { k: 0.0, h: 0.0, ch: None, at: -1.0, still: 0.0 },
            rl_shown: crate::character::AnimRlShown { carry: 0.0, head: 0.0 },
            kit: None,
            kit_front: false,
            kit_rest: 0.0,
            kit_hold: None,
            kit_stow: 0.0,
            kit_roll: 0.0,
            kit_lie: None,
            kit_gun_free: 0.0,
            close_k: -1.0,
            cl_k: 0.0,
            cl_t: 0.0,
            cl_l: 0.0,
            mv2: crate::character::AnimMv2 { bd: crate::character::Spring::blank(), bd_k: 1.0, cl: crate::character::Spring::blank(), dip: crate::character::Spring::blank(), rw: crate::character::Spring::blank(), rl_at: 0.0 },
            mv2_shown: crate::character::AnimMv2Shown { reload: 0.0, bound: 0.0, clutch: 0.0, dip: 0.0 },
            arms_pre: None,
            arms_buf: None,
            sights_lift: 0.0,
            kick_t: -1.0,
            kick_hit: false,
            sig_t: -1.0,
            sig_kind: crate::character::SignalKind::Hold,
            sig_side: 1.0,
            tell_t: -1.0,
            tell_k: crate::character::TellKind::Watch,
            tell_shown: 0.0,
            strike_k: crate::character::StrikeKind::Cross,
            ready: None,
            ready_at: None,
            ready_shown: 0.0,
            ready_up: 0.0,
            ready_amt: 1.0,
            rise: 0.0,
            low: 0.0,
            low_s: 0.0,
            low_shape: [0.0, 0.0, 0.0],
            wide: 0.0,
            wide_shape: [0.0, 0.0, 0.0, 0.0],
            wide_pole: None,
            wide_s: 0.0,
            tap_u: -1.0,
            tap_n: 0.0,
            tap_way: 1.0,
            tap_seen: -1.0,
            tap_from: 0.0,
            tap_to: 0.0,
            tap_was: -1.0,
            heft: 0.0,
            heft_shape: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            hf: crate::character::AnimHf { s: 0.0, sv: 0.0, a: 0.0, av: 0.0, w: 0.0, wv: 0.0, v: 0.0, vv: 0.0, t: 0.0 },
            hands_down: 0.0,
            spring: 0.0,
            coil: 0.0,
            creep: false,
            creep_at: None,
            creep_shown: 0.0,
            knife_shown: false,
            reach_shown: 0.0,
            no_knife: false,
            cr_t: 0.0,
            cr_v: 0.0,
            cr_pace: 0.0,
            cr_w: 0.0,
            cr_ph: None,
            bw: None,
            tk: None,
            kb_slash: false,
            kb_back: false,
            bw_rise: 0.0,
            rc_t: 0.0,
            strike_leth: false,
            kb_r0: 0.0,
            kb_t: 0.0,
            rc_inv: None,
            rd_out_r: 1.0,
            rd_a: 1.0,
            hd_from: None,
            hd_caught: false,
            he_span: 0.0,
            rd_hand_l: None,
            hd_fix: None,
            hd_sw: 0.0,
            he_sw: 0.0,
            ready_w: 0.0,
            ready_k: crate::character::ReadyKind::Grab,
            rd_at: None,
            rd_live: false,
            rd: alloc::vec![0.0; 36],
            idle_t: 0.0,
            st: crate::character::AnimSt { w: crate::character::Spring::blank(), fwd: crate::character::Spring::at(0.25), up: crate::character::Spring::at(0.15), side: crate::character::Spring::at(-0.05), pitch: crate::character::Spring::at(8.0), lean: crate::character::Spring::at(7.0), head_pitch: crate::character::Spring::at(0.0), tuck: crate::character::Spring::at(0.85), blade: crate::character::Spring::at(0.0), head_yaw: crate::character::Spring::at(0.0), squat: crate::character::Spring::at(0.0) },
            wst: crate::character::AnimWst { wl: crate::character::Spring::blank(), side: crate::character::Spring::at(1.0), peek: crate::character::Spring::blank(), park: crate::character::Spring::at(1.0) },
            stance_seen: crate::character::Stance::HighReady,
            lp_w: crate::character::Spring::blank(),
            lp_t: 0.0,
            hl_w: crate::character::Spring::blank(),
            hl_last: None,
            lg: crate::character::AnimLg { w: crate::character::Spring::blank(), seen: None, ch: alloc::vec![crate::character::Spring::blank(); 21], pull: [0.0, 0.0, 0.0], lat: [f64::NAN, f64::NAN], run: 0.0, sprint: 0.0 },
            lg_armed_was: false,
            hd_w: crate::character::Spring::blank(),
            hd_last: crate::character::HoldPose { yank: None, step: None, man: None, pull: None, phase: crate::character::HoldPhase::Held, t: 0.0, variant: None, gun_out: None, gun_pitch: None, low: None, kill: None, beats: None, haul: None, strip: None },
            hd_kn: crate::character::Spring::blank(),
            hd_gv: crate::character::Spring::blank(),
            hd_on: false,
            hd_ck: crate::character::Spring::blank(),
            hd_lo: crate::character::Spring::blank(),
            hd_kc: crate::character::Spring::blank(),
            hd_hu: crate::character::Spring::blank(),
            he_hu: crate::character::Spring::blank(),
            he_w: crate::character::Spring::blank(),
            he_last: crate::character::HeldPose { yank: None, pull: None, phase: crate::character::HoldPhase::Held, t: 0.0, agitation: 0.0, variant: None, bare: None, shot: None, armed: None, low: None, kill: None, beats: None, haul: None, strip: None },
            he_ck: crate::character::Spring::blank(),
            he_gv: crate::character::Spring::blank(),
            he_on: false,
            he_lo: crate::character::Spring::blank(),
            he_lk: crate::character::Spring::blank(),
            he_kc: crate::character::Spring::blank(),
            he_bo: crate::character::Spring::blank(),
            he_bc: crate::character::Spring::blank(),
            he_hr: [None, None],
            dp: crate::character::AnimDp { w_l: crate::character::Spring::blank(), w_r: crate::character::Spring::blank(), last: None, right: false, reach_ok: false, tw: crate::character::Spring::blank() },
            wk: crate::character::AnimWk { last: None, fed: false, after: 0.0, w_l: crate::character::Spring::blank(), w_r: crate::character::Spring::blank(), ok_l: false, ok_r: false, tw: crate::character::Spring::blank(), main: 1.0, two: false, shown: 0.0 },
            sw: crate::character::AnimSw { cur: None, from: None, to: None, t: -1.0, dur: 1.0, shown: false, wait: -1.0, belt: 0.0 },
            hand_pre: None,
            sw_buf: None,
            hand_buf: None,
            under_buf: None,
            rx: crate::character::AnimRx { left: 0.0, w: crate::character::Spring::blank() },
            stealth: None,
            sp: None,
            dlb: crate::character::Spring::blank(),
            gait: crate::character::AnimGait { mid: -0.15, linger: 0.08 },
            dash_w: 0.0,
            wt: crate::character::AnimWt { w: 0.0, yaw: 0.0, pitch: 0.0, sw: 0.0, sx: 0.0, sz: 1.0, side: 0.0, sl: 1.0 },
            jog_hips_y: alloc::vec![0.0; 64],
            jog_hips_mean: 0.9,
            jog_mid: -0.1,
            run_k: 1.0,
            sprint_mid: -0.1,
            sprint_floor_y: 0.09,
            jog_seam: Vec::new(),
            run_gait: crate::character::AnimRunGait { jog: crate::character::AnimRunGaitJog { c: 0.117, c_l: 0.117, c_r: 0.617 }, sprint: crate::character::AnimRunGaitSprint { c: 0.153, c_l: 0.133, c_r: 0.672 } },
            plant_l: crate::character::AnimPlantL { dev: 0.0, pz: f64::NAN },
            plant_r: crate::character::AnimPlantR { dev: 0.0, pz: f64::NAN },
            plant_w: 0.0,
            gz: crate::character::AnimGz { pitch: 0.0, yaw_ref: 0.0, yaw_init: false, w: 0.0 },
            life: crate::character::AnimLife { breath: 0.0, exert: 0.0, prev_s: 0.0, accel: 0.0, mass: crate::character::Spring::blank(), mass_v: crate::character::Spring::blank(), still_t: 0.0, shift_t: 4.0, shift_side: 1.0, shift: crate::character::Spring::blank(), yaw_prev: f64::NAN, yaw_rate: 0.0, prev_pos: [f64::NAN, 0.0, 0.0], root_v: 0.0, gate: 0.0, shift_n: 0.0 },
            feet: crate::character::AnimFeet { w: 0.0, gate: 0.0, l: crate::character::AnimFeetL { p: [0.0, 0.0, 0.0], yaw: 0.0, swing: -1.0, from: [0.0, 0.0, 0.0], from_yaw: 0.0 }, r: crate::character::AnimFeetR { p: [0.0, 0.0, 0.0], yaw: 0.0, swing: -1.0, from: [0.0, 0.0, 0.0], from_yaw: 0.0 }, next: 0.0 },
            body_yaw_in: 0.0,
            thr_last: None,
            thr_carry: None,
            thr_low: -1.0,
            thr_far: 1.0,
            thr_hang: -1.0,
            thr_hit0: 0.0,
            thr_off: false,
            idle_feet: crate::character::AnimIdleFeet::zeroed(),
            kick_from: None,
            kick_leg: crate::character::AnimKickLeg::Right,
            clips: JsMap::new(),
            upper_mask: Vec::new(),
            arm_r_mask: Vec::new(),
            head_mask: Vec::new(),
            arms_mask: Vec::new(),
            arm_r_only: Vec::new(),
            torso_mask: Vec::new(),
            fk: crate::anim_skeleton::PoseFK::blank(),
            hand_fix: Vec::new(),
            grip_up: [0.0; 3],
            gaze_local: [0.0; 3],
            chest_rest_inv: [0.0; 4],
            head_rest_inv: [0.0; 4],
            fingers: Vec::new(),
            thumbs: Vec::new(),
            grip: [false, false],
            grip_can: false,
            hn: crate::character::AnimHn { nodes: Vec::new(), fist: Vec::new(), own: Vec::new(), open: Vec::new(), trig: alloc::vec![0.0; 12], thumb: alloc::vec![alloc::vec![0.0; 12], alloc::vec![0.0; 12]], base: alloc::vec![alloc::vec![0.0; 60], alloc::vec![0.0; 60]], note: alloc::vec![alloc::vec![0.0; 5], alloc::vec![0.0; 5]], seed: 0.0, built: f64::NAN, quiet: false, grip: alloc::vec![0.0, 0.0], off: 1.0, fired: 9.0, gun: 0.0 },
            nodes: crate::character::AnimNodes::zeroed(),
            rig: RigId(0),
            nv: None,
            nv_arm: 0.0,
            nv_nod: 0.0,
            nv_a: 0.0,
            nv_tw: [0.0, 0.0, 0.0, 0.0],
            nv_shown: None,
            smoke: None,
            smoke_arm: 0.0,
            reload_dur: 1.6,
            act_loop: true,
            mv_out: crate::character::MvGait { live: false, g: 0.0, u: 0.0, st: 0.0 },
            mv_po: crate::character::AnimMvPo { stw: 0.0, stn: 1.0, dzw: 0.0, gun: false, gun_p: [0.0, 0.0, 0.0], gun_r: [0.0, 0.0, 0.0, 1.0] },
            fk_legs: Vec::new(),
            fk_arms: Vec::new(),
            fk_head: Vec::new(),
            cast: crate::anim_skeleton::LayerCast::blank(),
            hn_flat: None,
            hn_flat_of: f64::NAN,
            hd_idle: crate::character::AnimHdIdle { upper: crate::character::UpperMode::None, long: false, crouch: 0.0, carry: None },
            hd_busy: false,
            big_buf: None,
            big_under: false,
        }
    }
}
impl Default for Anim { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/character.ts:1475 CharacterAnimator.wallTakeFeet, src/game/character.ts:1475)
#[derive(Clone, Copy, Debug)]
pub struct AnimWallTakeFeet {
    pub l: crate::character::AnimWallTakeFeetL,
    pub r: crate::character::AnimWallTakeFeetR,
}
impl AnimWallTakeFeet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWallTakeFeet {
            l: crate::character::AnimWallTakeFeetL::zeroed(),
            r: crate::character::AnimWallTakeFeetR::zeroed(),
        }
    }
}
impl Default for AnimWallTakeFeet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1475 AnimWallTakeFeet.L, src/game/character.ts:1475)
#[derive(Clone, Copy, Debug)]
pub struct AnimWallTakeFeetL {
    pub p: V3,
    pub yaw: f64,
}
impl AnimWallTakeFeetL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWallTakeFeetL {
            p: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for AnimWallTakeFeetL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1475 AnimWallTakeFeet.R, src/game/character.ts:1475)
#[derive(Clone, Copy, Debug)]
pub struct AnimWallTakeFeetR {
    pub p: V3,
    pub yaw: f64,
}
impl AnimWallTakeFeetR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWallTakeFeetR {
            p: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for AnimWallTakeFeetR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1476 CharacterAnimator.wtk, src/game/character.ts:1476)
#[derive(Clone, Copy, Debug)]
pub struct AnimWtk {
    pub l: crate::character::AnimWtkL,
    pub r: crate::character::AnimWtkR,
    pub first: f64,
    pub last: f64,
}
impl AnimWtk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWtk {
            l: crate::character::AnimWtkL::zeroed(),
            r: crate::character::AnimWtkR::zeroed(),
            first: 0.0,
            last: 0.0,
        }
    }
}
impl Default for AnimWtk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1476 AnimWtk.L, src/game/character.ts:1476)
#[derive(Clone, Copy, Debug)]
pub struct AnimWtkL {
    pub p: V3,
    pub yaw: f64,
}
impl AnimWtkL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWtkL {
            p: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for AnimWtkL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1476 AnimWtk.R, src/game/character.ts:1476)
#[derive(Clone, Copy, Debug)]
pub struct AnimWtkR {
    pub p: V3,
    pub yaw: f64,
}
impl AnimWtkR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWtkR {
            p: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for AnimWtkR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1492 CharacterAnimator.watchShown, src/game/character.ts:1492)
#[derive(Clone, Copy, Debug)]
pub struct AnimWatchShown {
    pub w: f64,
    pub yaw: f64,
    pub slip: f64,
    pub lean: f64,
}
impl AnimWatchShown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWatchShown {
            w: 0.0,
            yaw: 0.0,
            slip: 0.0,
            lean: 0.0,
        }
    }
}
impl Default for AnimWatchShown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimCarry {
    Torch,
    Weapon,
}
impl AnimCarry {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimCarry::Torch => "torch",
            AnimCarry::Weapon => "weapon",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:1538 CharacterAnimator.longGrip, src/game/character.ts:1538)
#[derive(Clone, Copy, Debug)]
pub struct AnimLongGrip {
    pub wrist: V3,
    pub fingers: V3,
    pub back: V3,
}
impl AnimLongGrip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimLongGrip {
            wrist: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for AnimLongGrip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1558 CharacterAnimator.bigTool, src/game/character.ts:1558)
#[derive(Clone, Debug)]
pub struct AnimBigTool {
    pub kind: crate::character::AnimBigToolKind,
    pub t: f64,
    pub equip: f64,
    pub windup: f64,
    pub stow: f64,
    pub blows: Vec<f64>,
    pub end: f64,
    pub at: V3,
    pub snug: Option<bool>,
}
impl AnimBigTool {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimBigTool {
            kind: crate::character::AnimBigToolKind::Shotgun,
            t: 0.0,
            equip: 0.0,
            windup: 0.0,
            stow: 0.0,
            blows: Vec::new(),
            end: 0.0,
            at: [0.0; 3],
            snug: None,
        }
    }
}
impl Default for AnimBigTool { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimBigToolKind {
    Shotgun,
    Hammer,
}
impl AnimBigToolKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimBigToolKind::Shotgun => "shotgun",
            AnimBigToolKind::Hammer => "hammer",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:1567 CharacterAnimator.bigSide, src/game/character.ts:1567)
#[derive(Clone, Copy, Debug)]
pub struct AnimBigSide {
    pub o: V3,
    pub z: V3,
}
impl AnimBigSide {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimBigSide {
            o: [0.0; 3],
            z: [0.0; 3],
        }
    }
}
impl Default for AnimBigSide { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1577 CharacterAnimator.rawClip, src/game/character.ts:1577)
#[derive(Clone, Copy, Debug)]
pub struct AnimRawClip {
    pub clip: ClipId,
    pub time: f64,
    pub rate: f64,
    pub loop_: bool,
}
impl AnimRawClip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimRawClip {
            clip: ClipId(0),
            time: 0.0,
            rate: 0.0,
            loop_: false,
        }
    }
}
impl Default for AnimRawClip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1583 CharacterAnimator.act, src/game/character.ts:1583)
#[derive(Clone, Debug)]
pub struct AnimAct {
    pub clip: String,
    pub loop_: Option<bool>,
    pub rate: Option<f64>,
    pub in_: Option<f64>,
    pub out: Option<f64>,
}
impl AnimAct {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimAct {
            clip: String::new(),
            loop_: None,
            rate: None,
            in_: None,
            out: None,
        }
    }
}
impl Default for AnimAct { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1591 CharacterAnimator.wound, src/game/character.ts:1591)
#[derive(Clone, Copy, Debug)]
pub struct AnimWound {
    pub pose: crate::character::AnimWoundPose,
    pub at: crate::character::AnimWoundAt,
    pub settled: Option<bool>,
    pub clock: Option<f64>,
}
impl AnimWound {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWound {
            pose: crate::character::AnimWoundPose::Knee,
            at: crate::character::AnimWoundAt::Ribs,
            settled: None,
            clock: None,
        }
    }
}
impl Default for AnimWound { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimWoundPose {
    Knee,
    Back,
}
impl AnimWoundPose {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimWoundPose::Knee => "knee",
            AnimWoundPose::Back => "back",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimWoundAt {
    Ribs,
    Thigh,
}
impl AnimWoundAt {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimWoundAt::Ribs => "ribs",
            AnimWoundAt::Thigh => "thigh",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:1605 CharacterAnimator.upperW, src/game/character.ts:1605)
#[derive(Clone, Copy, Debug)]
pub struct AnimUpperW {
    pub relaxed: f64,
    pub aim: f64,
    pub torch: f64,
}
impl AnimUpperW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimUpperW {
            relaxed: 0.0,
            aim: 0.0,
            torch: 0.0,
        }
    }
}
impl Default for AnimUpperW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1615 CharacterAnimator.dazzle, src/game/character.ts:1615)
#[derive(Clone, Copy, Debug)]
pub struct AnimDazzle {
    pub from: V3,
    pub k: f64,
}
impl AnimDazzle {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimDazzle {
            from: [0.0; 3],
            k: 0.0,
        }
    }
}
impl Default for AnimDazzle { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1617 CharacterAnimator.mvs, src/game/character.ts:1617)
#[derive(Clone, Copy, Debug)]
pub struct AnimMvs {
    pub back: f64,
    pub climb: f64,
    pub yaw: f64,
    pub rock: f64,
    pub sho: f64,
    pub head: f64,
    pub n: f64,
    pub fl_t: f64,
    pub fl_k: f64,
    pub fl_k_to: f64,
    pub fl_soft: bool,
    pub fl_d: V3,
    pub st_t: f64,
    pub st_d: V3,
    pub ht_t: f64,
    pub ht_d: V3,
    pub ht_side: f64,
    pub ts_t: f64,
    pub ts_rel: f64,
    pub arm_l: f64,
    pub dz: crate::character::Spring,
    pub dz_d: V3,
    pub dz_t: f64,
    pub dz_sd: f64,
    pub glide: crate::character::Spring,
    pub hip_y: f64,
    pub cf: crate::character::Spring,
    pub cf_t: f64,
    pub hu: crate::character::Spring,
    pub hu_t: f64,
    pub sk: crate::character::Spring,
}
impl AnimMvs {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimMvs {
            back: 0.0,
            climb: 0.0,
            yaw: 0.0,
            rock: 0.0,
            sho: 0.0,
            head: 0.0,
            n: 0.0,
            fl_t: 0.0,
            fl_k: 0.0,
            fl_k_to: 0.0,
            fl_soft: false,
            fl_d: [0.0; 3],
            st_t: 0.0,
            st_d: [0.0; 3],
            ht_t: 0.0,
            ht_d: [0.0; 3],
            ht_side: 0.0,
            ts_t: 0.0,
            ts_rel: 0.0,
            arm_l: 0.0,
            dz: crate::character::Spring::blank(),
            dz_d: [0.0; 3],
            dz_t: 0.0,
            dz_sd: 0.0,
            glide: crate::character::Spring::blank(),
            hip_y: 0.0,
            cf: crate::character::Spring::blank(),
            cf_t: 0.0,
            hu: crate::character::Spring::blank(),
            hu_t: 0.0,
            sk: crate::character::Spring::blank(),
        }
    }
}
impl Default for AnimMvs { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1643 CharacterAnimator.rlB, src/game/character.ts:1643)
#[derive(Clone, Copy, Debug)]
pub struct AnimRlB {
    pub dur: f64,
    pub out: f64,
    pub in_: f64,
    pub rack: f64,
    pub pull: bool,
    pub keep: f64,
    pub hand: bool,
    pub fresh: f64,
}
impl AnimRlB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimRlB {
            dur: 0.0,
            out: 0.0,
            in_: 0.0,
            rack: 0.0,
            pull: false,
            keep: 0.0,
            hand: false,
            fresh: 0.0,
        }
    }
}
impl Default for AnimRlB { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1646 CharacterAnimator.rlc, src/game/character.ts:1646)
#[derive(Clone, Debug)]
pub struct AnimRlc {
    pub k: f64,
    pub h: f64,
    pub ch: Option<Vec<crate::character::AnimRlcChItem>>,
    pub at: f64,
    pub still: f64,
}
impl AnimRlc {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimRlc {
            k: 0.0,
            h: 0.0,
            ch: None,
            at: 0.0,
            still: 0.0,
        }
    }
}
impl Default for AnimRlc { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1646 AnimRlc.ch, src/game/character.ts:1646)
#[derive(Clone, Copy, Debug)]
pub struct AnimRlcChItem {
    pub x: f64,
}
impl AnimRlcChItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimRlcChItem {
            x: 0.0,
        }
    }
}
impl Default for AnimRlcChItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1648 CharacterAnimator.rlShown, src/game/character.ts:1648)
#[derive(Clone, Copy, Debug)]
pub struct AnimRlShown {
    pub carry: f64,
    pub head: f64,
}
impl AnimRlShown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimRlShown {
            carry: 0.0,
            head: 0.0,
        }
    }
}
impl Default for AnimRlShown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1650 CharacterAnimator.kit, src/game/character.ts:1650)
#[derive(Clone, Copy, Debug)]
pub struct AnimKit {
    pub t: f64,
    pub from: Option<crate::character::KitItem>,
    pub to: Option<crate::character::KitItem>,
    pub t_a: f64,
    pub t_r: f64,
    pub d: f64,
    pub s0: f64,
    pub roll0: f64,
    pub re: bool,
    pub front: bool,
    pub cap: crate::character::AnimKitCap,
}
impl AnimKit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimKit {
            t: 0.0,
            from: None,
            to: None,
            t_a: 0.0,
            t_r: 0.0,
            d: 0.0,
            s0: 0.0,
            roll0: 0.0,
            re: false,
            front: false,
            cap: crate::character::AnimKitCap::zeroed(),
        }
    }
}
impl Default for AnimKit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1650 AnimKit.cap, src/game/character.ts:1650)
#[derive(Clone, Copy, Debug)]
pub struct AnimKitCap {
    pub r: V3,
    pub rq: Quat,
    pub l: V3,
    pub lq: Quat,
    pub mr: Option<V3>,
    pub ml: Option<V3>,
    pub mf: Option<V3>,
    pub mb: Option<V3>,
}
impl AnimKitCap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimKitCap {
            r: [0.0; 3],
            rq: [0.0; 4],
            l: [0.0; 3],
            lq: [0.0; 4],
            mr: None,
            ml: None,
            mf: None,
            mb: None,
        }
    }
}
impl Default for AnimKitCap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1664 CharacterAnimator.mv2, src/game/character.ts:1664)
#[derive(Clone, Copy, Debug)]
pub struct AnimMv2 {
    pub bd: crate::character::Spring,
    pub bd_k: f64,
    pub cl: crate::character::Spring,
    pub dip: crate::character::Spring,
    pub rw: crate::character::Spring,
    pub rl_at: f64,
}
impl AnimMv2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimMv2 {
            bd: crate::character::Spring::blank(),
            bd_k: 0.0,
            cl: crate::character::Spring::blank(),
            dip: crate::character::Spring::blank(),
            rw: crate::character::Spring::blank(),
            rl_at: 0.0,
        }
    }
}
impl Default for AnimMv2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1666 CharacterAnimator.mv2Shown, src/game/character.ts:1666)
#[derive(Clone, Copy, Debug)]
pub struct AnimMv2Shown {
    pub reload: f64,
    pub bound: f64,
    pub clutch: f64,
    pub dip: f64,
}
impl AnimMv2Shown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimMv2Shown {
            reload: 0.0,
            bound: 0.0,
            clutch: 0.0,
            dip: 0.0,
        }
    }
}
impl Default for AnimMv2Shown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1698 CharacterAnimator.hf, src/game/character.ts:1698)
#[derive(Clone, Copy, Debug)]
pub struct AnimHf {
    pub s: f64,
    pub sv: f64,
    pub a: f64,
    pub av: f64,
    pub w: f64,
    pub wv: f64,
    pub v: f64,
    pub vv: f64,
    pub t: f64,
}
impl AnimHf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimHf {
            s: 0.0,
            sv: 0.0,
            a: 0.0,
            av: 0.0,
            w: 0.0,
            wv: 0.0,
            v: 0.0,
            vv: 0.0,
            t: 0.0,
        }
    }
}
impl Default for AnimHf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1715 CharacterAnimator.tk, src/game/character.ts:1715)
#[derive(Clone, Copy, Debug)]
pub struct AnimTk {
    pub style: crate::character::TakenStyle,
    pub t: f64,
    pub delay: f64,
    pub off: f64,
}
impl AnimTk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimTk {
            style: crate::character::TakenStyle::Arch,
            t: 0.0,
            delay: 0.0,
            off: 0.0,
        }
    }
}
impl Default for AnimTk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1719 CharacterAnimator.hdFix, src/game/character.ts:1719)
#[derive(Clone, Copy, Debug)]
pub struct AnimHdFix {
    pub tgt: V3,
    pub pole: V3,
    pub pole_w: f64,
    pub rot: Quat,
    pub w: f64,
    pub rw: f64,
}
impl AnimHdFix {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimHdFix {
            tgt: [0.0; 3],
            pole: [0.0; 3],
            pole_w: 0.0,
            rot: [0.0; 4],
            w: 0.0,
            rw: 0.0,
        }
    }
}
impl Default for AnimHdFix { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1722 CharacterAnimator.st, src/game/character.ts:1722)
#[derive(Clone, Copy, Debug)]
pub struct AnimSt {
    pub w: crate::character::Spring,
    pub fwd: crate::character::Spring,
    pub up: crate::character::Spring,
    pub side: crate::character::Spring,
    pub pitch: crate::character::Spring,
    pub lean: crate::character::Spring,
    pub head_pitch: crate::character::Spring,
    pub tuck: crate::character::Spring,
    pub blade: crate::character::Spring,
    pub head_yaw: crate::character::Spring,
    pub squat: crate::character::Spring,
}
impl AnimSt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimSt {
            w: crate::character::Spring::blank(),
            fwd: crate::character::Spring::blank(),
            up: crate::character::Spring::blank(),
            side: crate::character::Spring::blank(),
            pitch: crate::character::Spring::blank(),
            lean: crate::character::Spring::blank(),
            head_pitch: crate::character::Spring::blank(),
            tuck: crate::character::Spring::blank(),
            blade: crate::character::Spring::blank(),
            head_yaw: crate::character::Spring::blank(),
            squat: crate::character::Spring::blank(),
        }
    }
}
impl Default for AnimSt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1726 CharacterAnimator.wst, src/game/character.ts:1726)
#[derive(Clone, Copy, Debug)]
pub struct AnimWst {
    pub wl: crate::character::Spring,
    pub side: crate::character::Spring,
    pub peek: crate::character::Spring,
    pub park: crate::character::Spring,
}
impl AnimWst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWst {
            wl: crate::character::Spring::blank(),
            side: crate::character::Spring::blank(),
            peek: crate::character::Spring::blank(),
            park: crate::character::Spring::blank(),
        }
    }
}
impl Default for AnimWst { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1731 CharacterAnimator.lg, src/game/character.ts:1731)
#[derive(Clone, Debug)]
pub struct AnimLg {
    pub w: crate::character::Spring,
    pub seen: Option<crate::character::LongStance>,
    pub ch: Vec<crate::character::Spring>,
    pub pull: V3,
    pub lat: [f64; 2],
    pub run: f64,
    pub sprint: f64,
}
impl AnimLg {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimLg {
            w: crate::character::Spring::blank(),
            seen: None,
            ch: Vec::new(),
            pull: [0.0; 3],
            lat: [0.0; 2],
            run: 0.0,
            sprint: 0.0,
        }
    }
}
impl Default for AnimLg { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1743 CharacterAnimator.dp, src/game/character.ts:1743)
#[derive(Clone, Copy, Debug)]
pub struct AnimDp {
    pub w_l: crate::character::Spring,
    pub w_r: crate::character::Spring,
    pub last: Option<crate::character::DoorPush>,
    pub right: bool,
    pub reach_ok: bool,
    pub tw: crate::character::Spring,
}
impl AnimDp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimDp {
            w_l: crate::character::Spring::blank(),
            w_r: crate::character::Spring::blank(),
            last: None,
            right: false,
            reach_ok: false,
            tw: crate::character::Spring::blank(),
        }
    }
}
impl Default for AnimDp { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1747 CharacterAnimator.wk, src/game/character.ts:1747)
#[derive(Clone, Debug)]
pub struct AnimWk {
    pub last: Option<crate::character::WorkPose>,
    pub fed: bool,
    pub after: f64,
    pub w_l: crate::character::Spring,
    pub w_r: crate::character::Spring,
    pub ok_l: bool,
    pub ok_r: bool,
    pub tw: crate::character::Spring,
    pub main: f64,
    pub two: bool,
    pub shown: f64,
}
impl AnimWk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWk {
            last: None,
            fed: false,
            after: 0.0,
            w_l: crate::character::Spring::blank(),
            w_r: crate::character::Spring::blank(),
            ok_l: false,
            ok_r: false,
            tw: crate::character::Spring::blank(),
            main: 0.0,
            two: false,
            shown: 0.0,
        }
    }
}
impl Default for AnimWk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1748 CharacterAnimator.sw, src/game/character.ts:1748)
#[derive(Clone, Copy, Debug)]
pub struct AnimSw {
    pub cur: Option<crate::character::AnimSwCur>,
    pub from: Option<crate::character::AnimSwFrom>,
    pub to: Option<crate::character::AnimSwTo>,
    pub t: f64,
    pub dur: f64,
    pub shown: bool,
    pub wait: f64,
    pub belt: f64,
}
impl AnimSw {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimSw {
            cur: None,
            from: None,
            to: None,
            t: 0.0,
            dur: 0.0,
            shown: false,
            wait: 0.0,
            belt: 0.0,
        }
    }
}
impl Default for AnimSw { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimSwCur {
    Torch,
    Weapon,
}
impl AnimSwCur {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimSwCur::Torch => "torch",
            AnimSwCur::Weapon => "weapon",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimSwFrom {
    Torch,
    Weapon,
}
impl AnimSwFrom {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimSwFrom::Torch => "torch",
            AnimSwFrom::Weapon => "weapon",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimSwTo {
    Torch,
    Weapon,
}
impl AnimSwTo {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimSwTo::Torch => "torch",
            AnimSwTo::Weapon => "weapon",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:1755 CharacterAnimator.rx, src/game/character.ts:1755)
#[derive(Clone, Copy, Debug)]
pub struct AnimRx {
    pub left: f64,
    pub w: crate::character::Spring,
}
impl AnimRx {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimRx {
            left: 0.0,
            w: crate::character::Spring::blank(),
        }
    }
}
impl Default for AnimRx { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1758 CharacterAnimator.sp, src/game/character.ts:1758)
#[derive(Clone, Debug)]
pub struct AnimSp {
    pub w_a: f64,
    pub t: f64,
    pub cr: f64,
    pub fl: f64,
    pub lead: f64,
    pub fz: f64,
    pub sw: f64,
    pub w_c: f64,
    pub w_h: f64,
    pub pic: Option<crate::anim_skeleton::Pose>,
    pub pic_on: bool,
    pub nod_up: Option<bool>,
}
impl AnimSp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimSp {
            w_a: 0.0,
            t: 0.0,
            cr: 0.0,
            fl: 0.0,
            lead: 0.0,
            fz: 0.0,
            sw: 0.0,
            w_c: 0.0,
            w_h: 0.0,
            pic: None,
            pic_on: false,
            nod_up: None,
        }
    }
}
impl Default for AnimSp { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1761 CharacterAnimator.gait, src/game/character.ts:1761)
#[derive(Clone, Copy, Debug)]
pub struct AnimGait {
    pub mid: f64,
    pub linger: f64,
}
impl AnimGait {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimGait {
            mid: 0.0,
            linger: 0.0,
        }
    }
}
impl Default for AnimGait { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1763 CharacterAnimator.wt, src/game/character.ts:1763)
#[derive(Clone, Copy, Debug)]
pub struct AnimWt {
    pub w: f64,
    pub yaw: f64,
    pub pitch: f64,
    pub sw: f64,
    pub sx: f64,
    pub sz: f64,
    pub side: f64,
    pub sl: f64,
}
impl AnimWt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWt {
            w: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            sw: 0.0,
            sx: 0.0,
            sz: 0.0,
            side: 0.0,
            sl: 0.0,
        }
    }
}
impl Default for AnimWt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1769 CharacterAnimator.jogSeam, src/game/character.ts:1769)
#[derive(Clone, Copy, Debug)]
pub struct AnimJogSeamItem {
    pub node: f64,
    pub d: Option<V3>,
    pub axis: Option<V3>,
    pub ang: Option<f64>,
}
impl AnimJogSeamItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimJogSeamItem {
            node: 0.0,
            d: None,
            axis: None,
            ang: None,
        }
    }
}
impl Default for AnimJogSeamItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1772 CharacterAnimator.runGait, src/game/character.ts:1772)
#[derive(Clone, Copy, Debug)]
pub struct AnimRunGait {
    pub jog: crate::character::AnimRunGaitJog,
    pub sprint: crate::character::AnimRunGaitSprint,
}
impl AnimRunGait {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimRunGait {
            jog: crate::character::AnimRunGaitJog::zeroed(),
            sprint: crate::character::AnimRunGaitSprint::zeroed(),
        }
    }
}
impl Default for AnimRunGait { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1772 AnimRunGait.jog, src/game/character.ts:1772)
#[derive(Clone, Copy, Debug)]
pub struct AnimRunGaitJog {
    pub c: f64,
    pub c_l: f64,
    pub c_r: f64,
}
impl AnimRunGaitJog {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimRunGaitJog {
            c: 0.0,
            c_l: 0.0,
            c_r: 0.0,
        }
    }
}
impl Default for AnimRunGaitJog { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1772 AnimRunGait.sprint, src/game/character.ts:1772)
#[derive(Clone, Copy, Debug)]
pub struct AnimRunGaitSprint {
    pub c: f64,
    pub c_l: f64,
    pub c_r: f64,
}
impl AnimRunGaitSprint {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimRunGaitSprint {
            c: 0.0,
            c_l: 0.0,
            c_r: 0.0,
        }
    }
}
impl Default for AnimRunGaitSprint { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1776 CharacterAnimator.plantL, src/game/character.ts:1776)
#[derive(Clone, Copy, Debug)]
pub struct AnimPlantL {
    pub dev: f64,
    pub pz: f64,
}
impl AnimPlantL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimPlantL {
            dev: 0.0,
            pz: 0.0,
        }
    }
}
impl Default for AnimPlantL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1776 CharacterAnimator.plantR, src/game/character.ts:1776)
#[derive(Clone, Copy, Debug)]
pub struct AnimPlantR {
    pub dev: f64,
    pub pz: f64,
}
impl AnimPlantR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimPlantR {
            dev: 0.0,
            pz: 0.0,
        }
    }
}
impl Default for AnimPlantR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1777 CharacterAnimator.gz, src/game/character.ts:1777)
#[derive(Clone, Copy, Debug)]
pub struct AnimGz {
    pub pitch: f64,
    pub yaw_ref: f64,
    pub yaw_init: bool,
    pub w: f64,
}
impl AnimGz {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimGz {
            pitch: 0.0,
            yaw_ref: 0.0,
            yaw_init: false,
            w: 0.0,
        }
    }
}
impl Default for AnimGz { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1778 CharacterAnimator.life, src/game/character.ts:1778)
#[derive(Clone, Copy, Debug)]
pub struct AnimLife {
    pub breath: f64,
    pub exert: f64,
    pub prev_s: f64,
    pub accel: f64,
    pub mass: crate::character::Spring,
    pub mass_v: crate::character::Spring,
    pub still_t: f64,
    pub shift_t: f64,
    pub shift_side: f64,
    pub shift: crate::character::Spring,
    pub yaw_prev: f64,
    pub yaw_rate: f64,
    pub prev_pos: V3,
    pub root_v: f64,
    pub gate: f64,
    pub shift_n: f64,
}
impl AnimLife {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimLife {
            breath: 0.0,
            exert: 0.0,
            prev_s: 0.0,
            accel: 0.0,
            mass: crate::character::Spring::blank(),
            mass_v: crate::character::Spring::blank(),
            still_t: 0.0,
            shift_t: 0.0,
            shift_side: 0.0,
            shift: crate::character::Spring::blank(),
            yaw_prev: 0.0,
            yaw_rate: 0.0,
            prev_pos: [0.0; 3],
            root_v: 0.0,
            gate: 0.0,
            shift_n: 0.0,
        }
    }
}
impl Default for AnimLife { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1781 CharacterAnimator.feet, src/game/character.ts:1781)
#[derive(Clone, Copy, Debug)]
pub struct AnimFeet {
    pub w: f64,
    pub gate: f64,
    pub l: crate::character::AnimFeetL,
    pub r: crate::character::AnimFeetR,
    pub next: f64,
}
impl AnimFeet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimFeet {
            w: 0.0,
            gate: 0.0,
            l: crate::character::AnimFeetL::zeroed(),
            r: crate::character::AnimFeetR::zeroed(),
            next: 0.0,
        }
    }
}
impl Default for AnimFeet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1781 AnimFeet.L, src/game/character.ts:1781)
#[derive(Clone, Copy, Debug)]
pub struct AnimFeetL {
    pub p: V3,
    pub yaw: f64,
    pub swing: f64,
    pub from: V3,
    pub from_yaw: f64,
}
impl AnimFeetL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimFeetL {
            p: [0.0; 3],
            yaw: 0.0,
            swing: 0.0,
            from: [0.0; 3],
            from_yaw: 0.0,
        }
    }
}
impl Default for AnimFeetL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1781 AnimFeet.R, src/game/character.ts:1781)
#[derive(Clone, Copy, Debug)]
pub struct AnimFeetR {
    pub p: V3,
    pub yaw: f64,
    pub swing: f64,
    pub from: V3,
    pub from_yaw: f64,
}
impl AnimFeetR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimFeetR {
            p: [0.0; 3],
            yaw: 0.0,
            swing: 0.0,
            from: [0.0; 3],
            from_yaw: 0.0,
        }
    }
}
impl Default for AnimFeetR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1786 CharacterAnimator.thrLast, src/game/character.ts:1786)
#[derive(Clone, Copy, Debug)]
pub struct AnimThrLast {
    pub r: V3,
    pub l: V3,
}
impl AnimThrLast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimThrLast {
            r: [0.0; 3],
            l: [0.0; 3],
        }
    }
}
impl Default for AnimThrLast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1786 CharacterAnimator.thrCarry, src/game/character.ts:1786)
#[derive(Clone, Copy, Debug)]
pub struct AnimThrCarry {
    pub r: V3,
    pub l: V3,
}
impl AnimThrCarry {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimThrCarry {
            r: [0.0; 3],
            l: [0.0; 3],
        }
    }
}
impl Default for AnimThrCarry { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1793 CharacterAnimator.idleFeet, src/game/character.ts:1793)
#[derive(Clone, Copy, Debug)]
pub struct AnimIdleFeet {
    pub l: V3,
    pub r: V3,
    pub r_l: Quat,
    pub r_r: Quat,
}
impl AnimIdleFeet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimIdleFeet {
            l: [0.0; 3],
            r: [0.0; 3],
            r_l: [0.0; 4],
            r_r: [0.0; 4],
        }
    }
}
impl Default for AnimIdleFeet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1795 CharacterAnimator.kickFrom, src/game/character.ts:1795)
#[derive(Clone, Copy, Debug)]
pub struct AnimKickFrom {
    pub l: V3,
    pub r_l: Quat,
    pub r: V3,
    pub r_r: Quat,
}
impl AnimKickFrom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimKickFrom {
            l: [0.0; 3],
            r_l: [0.0; 4],
            r: [0.0; 3],
            r_r: [0.0; 4],
        }
    }
}
impl Default for AnimKickFrom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimKickLeg {
    Left,
    Right,
}
impl AnimKickLeg {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimKickLeg::Left => "left",
            AnimKickLeg::Right => "right",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:1823 CharacterAnimator.hn, src/game/character.ts:1823)
#[derive(Clone, Debug)]
pub struct AnimHn {
    pub nodes: Vec<Vec<i32>>,
    pub fist: Vec<Vec<f32>>,
    pub own: Vec<Vec<f32>>,
    pub open: Vec<Vec<f32>>,
    pub trig: Vec<f32>,
    pub thumb: Vec<Vec<f32>>,
    pub base: Vec<Vec<f32>>,
    pub note: Vec<Vec<f32>>,
    pub seed: f64,
    pub built: f64,
    pub quiet: bool,
    pub grip: Vec<f64>,
    pub off: f64,
    pub fired: f64,
    pub gun: f64,
}
impl AnimHn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimHn {
            nodes: Vec::new(),
            fist: Vec::new(),
            own: Vec::new(),
            open: Vec::new(),
            trig: Vec::new(),
            thumb: Vec::new(),
            base: Vec::new(),
            note: Vec::new(),
            seed: 0.0,
            built: 0.0,
            quiet: false,
            grip: Vec::new(),
            off: 0.0,
            fired: 0.0,
            gun: 0.0,
        }
    }
}
impl Default for AnimHn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimNodesKey {
    Spine2,
    Hips,
    Head,
    UpperArmR,
    UpperArmL,
    HandL,
    HandR,
    FootL,
    FootR,
    ToeL,
    ToeR,
    ForearmL,
    ForearmR,
    ThighL,
    ShinL,
    ThighR,
    ShinR,
    Neck,
    Spine1,
    Spine3,
    ShoulderL,
    ShoulderR,
    Middle1R,
    Middle1L,
}
impl AnimNodesKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimNodesKey::Spine2 => "spine2",
            AnimNodesKey::Hips => "hips",
            AnimNodesKey::Head => "head",
            AnimNodesKey::UpperArmR => "upperArmR",
            AnimNodesKey::UpperArmL => "upperArmL",
            AnimNodesKey::HandL => "handL",
            AnimNodesKey::HandR => "handR",
            AnimNodesKey::FootL => "footL",
            AnimNodesKey::FootR => "footR",
            AnimNodesKey::ToeL => "toeL",
            AnimNodesKey::ToeR => "toeR",
            AnimNodesKey::ForearmL => "forearmL",
            AnimNodesKey::ForearmR => "forearmR",
            AnimNodesKey::ThighL => "thighL",
            AnimNodesKey::ShinL => "shinL",
            AnimNodesKey::ThighR => "thighR",
            AnimNodesKey::ShinR => "shinR",
            AnimNodesKey::Neck => "neck",
            AnimNodesKey::Spine1 => "spine1",
            AnimNodesKey::Spine3 => "spine3",
            AnimNodesKey::ShoulderL => "shoulderL",
            AnimNodesKey::ShoulderR => "shoulderR",
            AnimNodesKey::Middle1R => "middle1R",
            AnimNodesKey::Middle1L => "middle1L",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:2056 CharacterAnimator.nv, src/game/character.ts:2056)
#[derive(Clone, Copy, Debug)]
pub struct AnimNv {
    pub t: f64,
    pub hand: bool,
    pub down: bool,
    pub k0: f64,
    pub c0: f64,
}
impl AnimNv {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimNv {
            t: 0.0,
            hand: false,
            down: false,
            k0: 0.0,
            c0: 0.0,
        }
    }
}
impl Default for AnimNv { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:2868 CharacterAnimator.mvPo, src/game/character.ts:2868)
#[derive(Clone, Copy, Debug)]
pub struct AnimMvPo {
    pub stw: f64,
    pub stn: f64,
    pub dzw: f64,
    pub gun: bool,
    pub gun_p: V3,
    pub gun_r: Quat,
}
impl AnimMvPo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimMvPo {
            stw: 0.0,
            stn: 0.0,
            dzw: 0.0,
            gun: false,
            gun_p: [0.0; 3],
            gun_r: [0.0; 4],
        }
    }
}
impl Default for AnimMvPo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:4176 CharacterAnimator.hdIdle, src/game/character.ts:4176)
#[derive(Clone, Copy, Debug)]
pub struct AnimHdIdle {
    pub upper: crate::character::UpperMode,
    pub long: bool,
    pub crouch: f64,
    pub carry: Option<crate::character::AnimHdIdleCarry>,
}
impl AnimHdIdle {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimHdIdle {
            upper: crate::character::UpperMode::None,
            long: false,
            crouch: 0.0,
            carry: None,
        }
    }
}
impl Default for AnimHdIdle { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimHdIdleCarry {
    Torch,
    Weapon,
}
impl AnimHdIdleCarry {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimHdIdleCarry::Torch => "torch",
            AnimHdIdleCarry::Weapon => "weapon",
        }
    }
}

/// (TypeScript: RagDrive, src/game/character.ts:4270)
#[derive(Clone, Debug)]
pub struct RagDrive {
    pub pose: crate::anim_skeleton::Pose,
    pub ov: Vec<Option<crate::anim_skeleton::NodeOverride>>,
    pub hips: crate::anim_skeleton::NodeOverride,
    pub hips_rot0: Quat,
    pub rot0: Vec<Quat>,
    pub dir0: Vec<V3>,
    pub delta: Vec<Quat>,
    pub limb_ov: Vec<crate::anim_skeleton::NodeOverride>,
    pub synced: bool,
}
impl RagDrive {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RagDrive {
            pose: crate::anim_skeleton::Pose::blank(),
            ov: Vec::new(),
            hips: crate::anim_skeleton::NodeOverride::zeroed(),
            hips_rot0: [0.0; 4],
            rot0: Vec::new(),
            dir0: Vec::new(),
            delta: Vec::new(),
            limb_ov: Vec::new(),
            synced: false,
        }
    }
}
impl Default for RagDrive { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GuidePin, src/game/character.ts:4274)
#[derive(Clone, Debug)]
pub struct GuidePin {
    pub i: f64,
    pub t0: f64,
    pub off: f64,
    pub at: crate::character::GuidePinAt,
}
impl GuidePin {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuidePin {
            i: 0.0,
            t0: 0.0,
            off: 0.0,
            at: crate::character::GuidePinAt::Rigid { r: [0.0; 3] },
        }
    }
}
impl Default for GuidePin { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FallGuide, src/game/character.ts:4275)
#[derive(Clone, Debug)]
pub struct FallGuide {
    pub t: f64,
    pub end: f64,
    pub pins: Vec<crate::character::GuidePin>,
    pub release: Option<crate::character::FallRelease>,
    /// INVENTED: what the pins' closures shared
    pub frame: crate::character::FallFrame,
    /// INVENTED: the head pin's nod
    pub nod: crate::character::FallNod,
}
impl FallGuide {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FallGuide {
            t: 0.0,
            end: 0.0,
            pins: Vec::new(),
            release: None,
            frame: crate::character::FallFrame::Plain(crate::character::Led { pv: [0.0; 3], p_t: [0.0; 3], v_t: [0.0; 3], p0: 0.0, p1: 0.0, n0: 0.0, n1: 0.0, v0n: [0.0; 3] }),
            nod: crate::character::FallNod::Plain,
        }
    }
}
impl Default for FallGuide { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Breath, src/game/character.ts:4277)
#[derive(Clone, Debug)]
pub struct Breath {
    pub t: f64,
    pub due: f64,
    pub chest: f64,
    pub at: V3,
    pub rot: Quat,
    pub hold: Vec<f64>,
    pub hold_ov: Vec<crate::anim_skeleton::NodeOverride>,
    pub saved: Vec<Option<crate::anim_skeleton::NodeOverride>>,
}
impl Breath {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Breath {
            t: 0.0,
            due: 0.0,
            chest: 0.0,
            at: [0.0; 3],
            rot: [0.0; 4],
            hold: Vec::new(),
            hold_ov: Vec::new(),
            saved: Vec::new(),
        }
    }
}
impl Default for Breath { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CharacterDesc, src/game/character.ts:4279)
#[derive(Clone, Copy, Debug)]
pub struct CharacterDesc {
    pub id: f64,
    pub tint: V3,
    pub tint2: V3,
    pub pos: V3,
    pub yaw: f64,
    pub men: Option<crate::Opaque>,
}
impl CharacterDesc {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterDesc {
            id: 0.0,
            tint: [0.0; 3],
            tint2: [0.0; 3],
            pos: [0.0; 3],
            yaw: 0.0,
            men: None,
        }
    }
}
impl Default for CharacterDesc { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Character, src/game/character.ts:4294)
#[derive(Clone, Debug)]
pub struct Character {
    pub id: f64,
    pub tint: V3,
    pub tint2: V3,
    /// collideCircle and clampToWorld write the point handed to them. Door contacts move him through it
    pub pos: V3,
    pub vel: V3,
    pub body_yaw: f64,
    pub aim_yaw: f64,
    pub aim_pitch: f64,
    pub anim: crate::character::Anim,
    pub skel: crate::anim_skeleton::SkeletonInstance,
    pub world: crate::v3::m4::Mat4,
    pub radius: f64,
    pub alive: bool,
    /// NOT dressing: sight lines and shots read chest and head
    pub bones: JsMap<crate::character::Bone, V3>,
    pub bones_at: [f64; 2],
    pub gun_dir: V3,
    pub muzzle: V3,
    pub chest_dir: V3,
    pub muzzle_ahead: f64,
    pub wet: f64,
    pub beads: f64,
    pub wet_rain: f64,
    pub wet_line: f64,
    pub parts: f64,
    pub rest_chest_inv: Quat,
    pub rest_head_inv: Quat,
    pub hand_r_back: V3,
    pub hand_r_thumb: V3,
    pub hand_l_back: V3,
    pub hand_l_thumb: V3,
    pub hand_r_bore: V3,
    pub hand_r_top: V3,
    pub in_hand: Option<crate::inhand::InHandWord>,
    pub gun_look: Option<crate::gunlook::GunLook>,
    pub in_hand_cur: Option<crate::inhand::InHandWord>,
    pub in_hand_k: f64,
    pub sim_hand_r: Option<Quat>,
    pub in_hand_ov: Option<Vec<Option<crate::anim_skeleton::NodeOverride>>>,
    pub prev_bones: JsMap<crate::character::Bone, V3>,
    pub prev_dt: f64,
    pub sim_bones: Option<JsMap<crate::character::Bone, V3>>,
    pub prev_sim: Option<JsMap<crate::character::Bone, V3>>,
    pub gun_hand: Option<[V3; 2]>,
    pub ragdoll: Option<RagdollId>,
    pub rd: Option<crate::character::RagDrive>,
    pub fall_as: Option<crate::character::FallStyle>,
    pub fall_from: Option<V3>,
    pub fall_tight: bool,
    pub fall_ahead: f64,
    pub breathe: Option<crate::character::CharacterBreathe>,
    pub guide: Option<crate::character::FallGuide>,
    pub br: Option<crate::character::Breath>,
    pub turn: crate::character::CharacterTurn,
    pub tn: Option<crate::character::CharacterTn>,
    pub rig: RigId,
    /// whose body this is
    pub who: crate::character::Who,
}
impl Character {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Character {
            id: 0.0,
            tint: [0.0; 3],
            tint2: [0.0; 3],
            pos: [0.0; 3],
            vel: [0.0, 0.0, 0.0],
            body_yaw: 0.0,
            aim_yaw: 0.0,
            aim_pitch: 0.0,
            anim: crate::character::Anim::blank(),
            skel: crate::anim_skeleton::SkeletonInstance::blank(),
            world: crate::M4_IDENTITY,
            radius: 0.3,
            alive: true,
            bones: JsMap::new(),
            bones_at: [0.0, 0.0],
            gun_dir: [0.0, 0.0, 1.0],
            muzzle: [0.0, 0.0, 0.0],
            chest_dir: [0.0, 0.0, 1.0],
            muzzle_ahead: 0.26,
            wet: 0.0,
            beads: 0.0,
            wet_rain: 0.0,
            wet_line: f64::INFINITY,
            parts: 0.0,
            rest_chest_inv: [0.0; 4],
            rest_head_inv: [0.0; 4],
            hand_r_back: [0.0; 3],
            hand_r_thumb: [0.0; 3],
            hand_l_back: [0.0; 3],
            hand_l_thumb: [0.0; 3],
            hand_r_bore: [0.0, 1.0, 0.0],
            hand_r_top: [0.0, 0.0, 1.0],
            in_hand: None,
            gun_look: None,
            in_hand_cur: None,
            in_hand_k: 0.0,
            sim_hand_r: None,
            in_hand_ov: None,
            prev_bones: JsMap::new(),
            prev_dt: 0.0,
            sim_bones: None,
            prev_sim: None,
            gun_hand: None,
            ragdoll: None,
            rd: None,
            fall_as: None,
            fall_from: None,
            fall_tight: false,
            fall_ahead: 0.0,
            breathe: None,
            guide: None,
            br: None,
            turn: crate::character::CharacterTurn { on: false, h: 1.0, k: 0.0 },
            tn: None,
            rig: RigId(0),
            who: crate::character::Who::Guard(GuardId(0)),
        }
    }
}
impl Default for Character { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/character.ts:4349 Character.breathe, src/game/character.ts:4349)
#[derive(Clone, Copy, Debug)]
pub struct CharacterBreathe {
    pub breath_hz: f64,
    pub breath_rise: f64,
    pub breath_in: f64,
}
impl CharacterBreathe {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterBreathe {
            breath_hz: 0.0,
            breath_rise: 0.0,
            breath_in: 0.0,
        }
    }
}
impl Default for CharacterBreathe { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:4352 Character.turn, src/game/character.ts:4352)
#[derive(Clone, Copy, Debug)]
pub struct CharacterTurn {
    pub on: bool,
    pub h: f64,
    pub k: f64,
}
impl CharacterTurn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterTurn {
            on: false,
            h: 0.0,
            k: 0.0,
        }
    }
}
impl Default for CharacterTurn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:4353 Character.tn, src/game/character.ts:4353)
#[derive(Clone, Copy, Debug)]
pub struct CharacterTn {
    pub body: f64,
    pub aim: f64,
    pub wb: f64,
    pub wa: f64,
    pub ib: f64,
    pub ia: f64,
    pub x: f64,
    pub z: f64,
}
impl CharacterTn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterTn {
            body: 0.0,
            aim: 0.0,
            wb: 0.0,
            wa: 0.0,
            ib: 0.0,
            ia: 0.0,
            x: 0.0,
            z: 0.0,
        }
    }
}
impl Default for CharacterTn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:81 WORK, src/game/character.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct WORK {
    pub key_lead: f64,
    pub tip_len: f64,
    pub hover: f64,
    pub key_off: f64,
    pub lever: f64,
}
impl WORK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WORK {
            key_lead: 0.0,
            tip_len: 0.0,
            hover: 0.0,
            key_off: 0.0,
            lever: 0.0,
        }
    }
}
impl Default for WORK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:87 SWAP, src/game/character.ts:87)
#[derive(Clone, Copy, Debug)]
pub struct SWAP {
    pub draw: f64,
    pub stow: f64,
    pub away: f64,
    pub take: f64,
    pub lag: f64,
}
impl SWAP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SWAP {
            draw: 0.0,
            stow: 0.0,
            away: 0.0,
            take: 0.0,
            lag: 0.0,
        }
    }
}
impl Default for SWAP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:336 HOLD_POLISH_UP, src/game/character.ts:336)
#[derive(Clone, Copy, Debug)]
pub struct HOLDPOLISHUP {
    pub way_shape: bool,
    pub catch_shape: bool,
    pub hit_shape: bool,
    pub lay_ahead: f64,
    pub dip: f64,
    pub back: f64,
    pub m: f64,
    pub twist: f64,
    pub side: f64,
    pub tip: f64,
    pub arch: f64,
}
impl HOLDPOLISHUP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HOLDPOLISHUP {
            way_shape: false,
            catch_shape: false,
            hit_shape: false,
            lay_ahead: 0.0,
            dip: 0.0,
            back: 0.0,
            m: 0.0,
            twist: 0.0,
            side: 0.0,
            tip: 0.0,
            arch: 0.0,
        }
    }
}
impl Default for HOLDPOLISHUP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:337 HOLD_POLISH_DOWN, src/game/character.ts:337)
#[derive(Clone, Copy, Debug)]
pub struct HOLDPOLISHDOWN {
    pub way_shape: bool,
    pub catch_shape: bool,
    pub hit_shape: bool,
    pub lay_ahead: f64,
    pub dip: f64,
    pub back: f64,
    pub m: f64,
    pub twist: f64,
    pub side: f64,
    pub tip: f64,
    pub arch: f64,
}
impl HOLDPOLISHDOWN {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HOLDPOLISHDOWN {
            way_shape: false,
            catch_shape: false,
            hit_shape: false,
            lay_ahead: 0.0,
            dip: 0.0,
            back: 0.0,
            m: 0.0,
            twist: 0.0,
            side: 0.0,
            tip: 0.0,
            arch: 0.0,
        }
    }
}
impl Default for HOLDPOLISHDOWN { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:342 HOLD_SHOW, src/game/character.ts:342)
#[derive(Clone, Copy, Debug)]
pub struct CharacterModHOLDSHOW {
    pub kill: bool,
    pub beats: bool,
}
impl CharacterModHOLDSHOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterModHOLDSHOW {
            kill: false,
            beats: false,
        }
    }
}
impl Default for CharacterModHOLDSHOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:357 BLEND, src/game/character.ts:357)
#[derive(Clone, Copy, Debug)]
pub struct BLEND {
    pub walk_in0: f64,
    pub walk_in1: f64,
    pub walk_top: f64,
    pub jog_at: f64,
}
impl BLEND {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BLEND {
            walk_in0: 0.0,
            walk_in1: 0.0,
            walk_top: 0.0,
            jog_at: 0.0,
        }
    }
}
impl Default for BLEND { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:359 BOB, src/game/character.ts:359)
#[derive(Clone, Copy, Debug)]
pub struct BOB {
    pub keep: f64,
}
impl BOB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BOB {
            keep: 0.0,
        }
    }
}
impl Default for BOB { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:363 RUN_WARP, src/game/character.ts:363)
#[derive(Clone, Copy, Debug)]
pub struct RUNWARP {
    pub stride: f64,
}
impl RUNWARP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RUNWARP {
            stride: 0.0,
        }
    }
}
impl Default for RUNWARP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:371 SPRINT_WARP, src/game/character.ts:371)
#[derive(Clone, Copy, Debug)]
pub struct SPRINTWARP {
    pub stride: f64,
}
impl SPRINTWARP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SPRINTWARP {
            stride: 0.0,
        }
    }
}
impl Default for SPRINTWARP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:385 RUN_LINGER, src/game/character.ts:385)
#[derive(Clone, Copy, Debug)]
pub struct RUNLINGER {
    pub depth: f64,
    pub on: [f64; 2],
    pub off: [f64; 2],
    pub bleed: f64,
}
impl RUNLINGER {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RUNLINGER {
            depth: 0.0,
            on: [0.0; 2],
            off: [0.0; 2],
            bleed: 0.0,
        }
    }
}
impl Default for RUNLINGER { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:392 GAZE_EL, src/game/character.ts:392)
#[derive(Clone, Copy, Debug)]
pub struct GAZEEL {
    pub stand: f64,
    pub crouch: f64,
    pub sprint: f64,
}
impl GAZEEL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GAZEEL {
            stand: 0.0,
            crouch: 0.0,
            sprint: 0.0,
        }
    }
}
impl Default for GAZEEL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:392 GAZE, src/game/character.ts:392)
#[derive(Clone, Copy, Debug)]
pub struct GAZE {
    pub keep: f64,
    pub yaw_damp: f64,
}
impl GAZE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GAZE {
            keep: 0.0,
            yaw_damp: 0.0,
        }
    }
}
impl Default for GAZE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:406 LIFE, src/game/character.ts:406)
#[derive(Clone, Copy, Debug)]
pub struct LIFE {
    pub on: f64,
    pub breath_rate: f64,
    pub breath_rate_run: f64,
    pub breath_amp: f64,
    pub breath_amp_run: f64,
    pub mass_gain: f64,
    pub mass_back: f64,
    pub mass_fwd: f64,
    pub mass_dip: f64,
    pub mass_dip_max: f64,
    pub tip_gain: f64,
    pub tip_back: f64,
    pub tip_fwd: f64,
    pub rest_after: f64,
    pub shift_min: f64,
    pub shift_span: f64,
    pub shift_travel: f64,
    pub shift_in: f64,
    pub shift_out: f64,
    pub hip_drop: f64,
    pub spine_counter: f64,
    pub neck_counter: f64,
}
impl LIFE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LIFE {
            on: 0.0,
            breath_rate: 0.0,
            breath_rate_run: 0.0,
            breath_amp: 0.0,
            breath_amp_run: 0.0,
            mass_gain: 0.0,
            mass_back: 0.0,
            mass_fwd: 0.0,
            mass_dip: 0.0,
            mass_dip_max: 0.0,
            tip_gain: 0.0,
            tip_back: 0.0,
            tip_fwd: 0.0,
            rest_after: 0.0,
            shift_min: 0.0,
            shift_span: 0.0,
            shift_travel: 0.0,
            shift_in: 0.0,
            shift_out: 0.0,
            hip_drop: 0.0,
            spine_counter: 0.0,
            neck_counter: 0.0,
        }
    }
}
impl Default for LIFE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:422 SOFT_KNEE, src/game/character.ts:422)
#[derive(Clone, Copy, Debug)]
pub struct SOFTKNEE {
    pub on: f64,
    pub from: f64,
    pub to: f64,
}
impl SOFTKNEE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SOFTKNEE {
            on: 0.0,
            from: 0.0,
            to: 0.0,
        }
    }
}
impl Default for SOFTKNEE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:428 FEET, src/game/character.ts:428)
#[derive(Clone, Copy, Debug)]
pub struct FEET {
    pub on: f64,
    pub swing: f64,
    pub lift: f64,
    pub dist: f64,
    pub turn_deg: f64,
    pub pivot: f64,
    pub lag_cap_deg: f64,
}
impl FEET {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FEET {
            on: 0.0,
            swing: 0.0,
            lift: 0.0,
            dist: 0.0,
            turn_deg: 0.0,
            pivot: 0.0,
            lag_cap_deg: 0.0,
        }
    }
}
impl Default for FEET { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:430 N, src/game/character.ts:430)
#[derive(Clone, Debug)]
pub struct N {
    pub hips: String,
    pub spine1: String,
    pub spine2: String,
    pub spine3: String,
    pub neck: String,
    pub head: String,
    pub upper_arm_l: String,
    pub forearm_l: String,
    pub hand_l: String,
    pub upper_arm_r: String,
    pub forearm_r: String,
    pub hand_r: String,
    pub thigh_l: String,
    pub shin_l: String,
    pub foot_l: String,
    pub toe_l: String,
    pub thigh_r: String,
    pub shin_r: String,
    pub foot_r: String,
    pub toe_r: String,
    pub shoulder_l: String,
    pub shoulder_r: String,
    pub middle1_r: String,
    pub middle1_l: String,
}
impl N {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        N {
            hips: String::new(),
            spine1: String::new(),
            spine2: String::new(),
            spine3: String::new(),
            neck: String::new(),
            head: String::new(),
            upper_arm_l: String::new(),
            forearm_l: String::new(),
            hand_l: String::new(),
            upper_arm_r: String::new(),
            forearm_r: String::new(),
            hand_r: String::new(),
            thigh_l: String::new(),
            shin_l: String::new(),
            foot_l: String::new(),
            toe_l: String::new(),
            thigh_r: String::new(),
            shin_r: String::new(),
            foot_r: String::new(),
            toe_r: String::new(),
            shoulder_l: String::new(),
            shoulder_r: String::new(),
            middle1_r: String::new(),
            middle1_l: String::new(),
        }
    }
}
impl Default for N { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:449 STEALTH, src/game/character.ts:449)
#[derive(Clone, Copy, Debug)]
pub struct STEALTH {
    pub in_secs: f64,
    pub out_secs: f64,
    pub round: f64,
    pub idle: crate::character::STEALTHIdle,
    pub walk: crate::character::STEALTHWalk,
    pub rise: crate::character::STEALTHRise,
    pub creep: crate::character::STEALTHCreep,
    pub cable: crate::character::STEALTHCable,
}
impl STEALTH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STEALTH {
            in_secs: 0.0,
            out_secs: 0.0,
            round: 0.0,
            idle: crate::character::STEALTHIdle::zeroed(),
            walk: crate::character::STEALTHWalk::zeroed(),
            rise: crate::character::STEALTHRise::zeroed(),
            creep: crate::character::STEALTHCreep::zeroed(),
            cable: crate::character::STEALTHCable::zeroed(),
        }
    }
}
impl Default for STEALTH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:451 STEALTH.idle, src/game/character.ts:451)
#[derive(Clone, Copy, Debug)]
pub struct STEALTHIdle {
    pub out: f64,
    pub below: f64,
    pub fwd: f64,
    pub lead: f64,
    pub floor: f64,
    pub pole: V3,
    pub fing: V3,
    pub back: V3,
    pub curl: f64,
    pub thumb: f64,
    pub spread: f64,
}
impl STEALTHIdle {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STEALTHIdle {
            out: 0.0,
            below: 0.0,
            fwd: 0.0,
            lead: 0.0,
            floor: 0.0,
            pole: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
            curl: 0.0,
            thumb: 0.0,
            spread: 0.0,
        }
    }
}
impl Default for STEALTHIdle { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:452 STEALTH.walk, src/game/character.ts:452)
#[derive(Clone, Copy, Debug)]
pub struct STEALTHWalk {
    pub stride: f64,
    pub rate: f64,
    pub fwd: V3,
    pub back: V3,
    pub flare_pole: f64,
    pub tuck: f64,
}
impl STEALTHWalk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STEALTHWalk {
            stride: 0.0,
            rate: 0.0,
            fwd: [0.0; 3],
            back: [0.0; 3],
            flare_pole: 0.0,
            tuck: 0.0,
        }
    }
}
impl Default for STEALTHWalk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:453 STEALTH.rise, src/game/character.ts:453)
#[derive(Clone, Copy, Debug)]
pub struct STEALTHRise {
    pub down: f64,
}
impl STEALTHRise {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STEALTHRise {
            down: 0.0,
        }
    }
}
impl Default for STEALTHRise { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:454 STEALTH.creep, src/game/character.ts:454)
#[derive(Clone, Copy, Debug)]
pub struct STEALTHCreep {
    pub at: V3,
    pub floor: f64,
    pub rate: f64,
    pub swing: f64,
}
impl STEALTHCreep {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STEALTHCreep {
            at: [0.0; 3],
            floor: 0.0,
            rate: 0.0,
            swing: 0.0,
        }
    }
}
impl Default for STEALTHCreep { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:455 STEALTH.cable, src/game/character.ts:455)
#[derive(Clone, Copy, Debug)]
pub struct STEALTHCable {
    pub sink: f64,
    pub round: f64,
    pub gap: [f64; 2],
    pub short: f64,
    pub knee: V3,
    pub out: f64,
    pub head_out: f64,
    pub undo: f64,
    pub ahead: f64,
    pub nod_max: f64,
}
impl STEALTHCable {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STEALTHCable {
            sink: 0.0,
            round: 0.0,
            gap: [0.0; 2],
            short: 0.0,
            knee: [0.0; 3],
            out: 0.0,
            head_out: 0.0,
            undo: 0.0,
            ahead: 0.0,
            nod_max: 0.0,
        }
    }
}
impl Default for STEALTHCable { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:478 AT0, src/game/character.ts:478)
#[derive(Clone, Debug)]
pub struct CharacterModAT0Value {
    pub rows: f64,
    pub at: Vec<i32>,
    pub v: Vec<f32>,
}
impl CharacterModAT0Value {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterModAT0Value {
            rows: 0.0,
            at: Vec::new(),
            v: Vec::new(),
        }
    }
}
impl Default for CharacterModAT0Value { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:534 LEAN_POSE, src/game/character.ts:534)
#[derive(Clone, Copy, Debug)]
pub struct LEANPOSE {
    pub deg: f64,
    pub level: f64,
    pub rate: f64,
}
impl LEANPOSE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LEANPOSE {
            deg: 0.0,
            level: 0.0,
            rate: 0.0,
        }
    }
}
impl Default for LEANPOSE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:535 WALL_POSE, src/game/character.ts:535)
#[derive(Clone, Debug)]
pub struct WALLPOSE {
    pub back: [f64; 3],
    pub lean: [f64; 2],
    pub drop: [f64; 2],
    pub chest_turn: f64,
    pub gaze: f64,
    pub head_pitch: f64,
    pub feet: crate::character::WALLPOSEFeet,
    pub slide: crate::character::WALLPOSESlide,
    pub hands: crate::character::WALLPOSEHands,
    pub elbow: crate::character::WALLPOSEElbow,
    pub peek: crate::character::WALLPOSEPeek,
    pub mend: crate::character::WALLPOSEMend,
    pub gait: crate::character::WALLPOSEGait,
}
impl WALLPOSE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLPOSE {
            back: [0.0; 3],
            lean: [0.0; 2],
            drop: [0.0; 2],
            chest_turn: 0.0,
            gaze: 0.0,
            head_pitch: 0.0,
            feet: crate::character::WALLPOSEFeet::zeroed(),
            slide: crate::character::WALLPOSESlide::zeroed(),
            hands: crate::character::WALLPOSEHands::zeroed(),
            elbow: crate::character::WALLPOSEElbow::zeroed(),
            peek: crate::character::WALLPOSEPeek::zeroed(),
            mend: crate::character::WALLPOSEMend::zeroed(),
            gait: crate::character::WALLPOSEGait::zeroed(),
        }
    }
}
impl Default for WALLPOSE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:550 WALLPOSE.feet, src/game/character.ts:550)
#[derive(Clone, Copy, Debug)]
pub struct WALLPOSEFeet {
    pub lead: V3,
    pub trail: V3,
    pub lead_crouch: V3,
    pub trail_crouch: V3,
    pub trail_heel: f64,
}
impl WALLPOSEFeet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLPOSEFeet {
            lead: [0.0; 3],
            trail: [0.0; 3],
            lead_crouch: [0.0; 3],
            trail_crouch: [0.0; 3],
            trail_heel: 0.0,
        }
    }
}
impl Default for WALLPOSEFeet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:553 WALLPOSE.slide, src/game/character.ts:553)
#[derive(Clone, Copy, Debug)]
pub struct WALLPOSESlide {
    pub line: f64,
    pub narrow: f64,
}
impl WALLPOSESlide {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLPOSESlide {
            line: 0.0,
            narrow: 0.0,
        }
    }
}
impl Default for WALLPOSESlide { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:557 WALLPOSE.hands, src/game/character.ts:557)
#[derive(Clone, Copy, Debug)]
pub struct WALLPOSEHands {
    pub along: f64,
    pub along_crouch: f64,
    pub up: f64,
    pub up_crouch: f64,
    pub off: f64,
    pub out: f64,
    pub peek_in: f64,
}
impl WALLPOSEHands {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLPOSEHands {
            along: 0.0,
            along_crouch: 0.0,
            up: 0.0,
            up_crouch: 0.0,
            off: 0.0,
            out: 0.0,
            peek_in: 0.0,
        }
    }
}
impl Default for WALLPOSEHands { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:559 WALLPOSE.elbow, src/game/character.ts:559)
#[derive(Clone, Copy, Debug)]
pub struct WALLPOSEElbow {
    pub out: f64,
    pub room: f64,
    pub down: f64,
}
impl WALLPOSEElbow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLPOSEElbow {
            out: 0.0,
            room: 0.0,
            down: 0.0,
        }
    }
}
impl Default for WALLPOSEElbow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:564 WALLPOSE.peek, src/game/character.ts:564)
#[derive(Clone, Copy, Debug)]
pub struct WALLPOSEPeek {
    pub lean: f64,
    pub shift: f64,
    pub turn: f64,
    pub gaze: f64,
    pub tilt: f64,
}
impl WALLPOSEPeek {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLPOSEPeek {
            lean: 0.0,
            shift: 0.0,
            turn: 0.0,
            gaze: 0.0,
            tilt: 0.0,
        }
    }
}
impl Default for WALLPOSEPeek { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:569 WALLPOSE.mend, src/game/character.ts:569)
#[derive(Clone, Debug)]
pub struct WALLPOSEMend {
    pub along: [f64; 2],
    pub up: [f64; 2],
    pub elbow: crate::character::WALLPOSEMendElbow,
    pub splay: f64,
    pub sway: f64,
    pub fan: f64,
    pub curl: Vec<f64>,
    pub thumb: f64,
    pub thumb_in: f64,
}
impl WALLPOSEMend {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLPOSEMend {
            along: [0.0; 2],
            up: [0.0; 2],
            elbow: crate::character::WALLPOSEMendElbow::zeroed(),
            splay: 0.0,
            sway: 0.0,
            fan: 0.0,
            curl: Vec::new(),
            thumb: 0.0,
            thumb_in: 0.0,
        }
    }
}
impl Default for WALLPOSEMend { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:569 WALLPOSEMend.elbow, src/game/character.ts:569)
#[derive(Clone, Copy, Debug)]
pub struct WALLPOSEMendElbow {
    pub out: f64,
    pub room: f64,
    pub down: f64,
}
impl WALLPOSEMendElbow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLPOSEMendElbow {
            out: 0.0,
            room: 0.0,
            down: 0.0,
        }
    }
}
impl Default for WALLPOSEMendElbow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:583 WALLPOSE.gait, src/game/character.ts:583)
#[derive(Clone, Copy, Debug)]
pub struct WALLPOSEGait {
    pub share: [f64; 2],
    pub step: [f64; 2],
    pub duty: f64,
    pub ahead: [f64; 2],
    pub apart: [f64; 2],
    pub near: [f64; 2],
    pub far: [f64; 2],
    pub lift: [f64; 2],
    pub heel: [f64; 2],
    pub toe: [f64; 2],
    pub knee_room: [f64; 2],
    pub hips: [f64; 2],
    pub bob: [f64; 2],
}
impl WALLPOSEGait {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLPOSEGait {
            share: [0.0; 2],
            step: [0.0; 2],
            duty: 0.0,
            ahead: [0.0; 2],
            apart: [0.0; 2],
            near: [0.0; 2],
            far: [0.0; 2],
            lift: [0.0; 2],
            heel: [0.0; 2],
            toe: [0.0; 2],
            knee_room: [0.0; 2],
            hips: [0.0; 2],
            bob: [0.0; 2],
        }
    }
}
impl Default for WALLPOSEGait { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:668 LONG_RELOAD, src/game/character.ts:668)
#[derive(Clone, Copy, Debug)]
pub struct LONGRELOAD {
    pub box_: crate::character::LONGRELOADBox,
    pub back: crate::character::LONGRELOADBack,
    pub head: crate::character::LONGRELOADHead,
    pub in_: f64,
    pub out: f64,
    pub end: f64,
    pub head_in: f64,
    pub head_out: f64,
    pub rate: f64,
}
impl LONGRELOAD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LONGRELOAD {
            box_: crate::character::LONGRELOADBox::zeroed(),
            back: crate::character::LONGRELOADBack::zeroed(),
            head: crate::character::LONGRELOADHead::zeroed(),
            in_: 0.0,
            out: 0.0,
            end: 0.0,
            head_in: 0.0,
            head_out: 0.0,
            rate: 0.0,
        }
    }
}
impl Default for LONGRELOAD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:669 LONGRELOAD.box, src/game/character.ts:669)
#[derive(Clone, Copy, Debug)]
pub struct LONGRELOADBox {
    pub at: V3,
    pub pitch: f64,
    pub aim_k: f64,
    pub yaw: f64,
}
impl LONGRELOADBox {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LONGRELOADBox {
            at: [0.0; 3],
            pitch: 0.0,
            aim_k: 0.0,
            yaw: 0.0,
        }
    }
}
impl Default for LONGRELOADBox { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:670 LONGRELOAD.back, src/game/character.ts:670)
#[derive(Clone, Copy, Debug)]
pub struct LONGRELOADBack {
    pub at: V3,
    pub pitch: f64,
    pub aim_k: f64,
    pub yaw: f64,
}
impl LONGRELOADBack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LONGRELOADBack {
            at: [0.0; 3],
            pitch: 0.0,
            aim_k: 0.0,
            yaw: 0.0,
        }
    }
}
impl Default for LONGRELOADBack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:671 LONGRELOAD.head, src/game/character.ts:671)
#[derive(Clone, Copy, Debug)]
pub struct LONGRELOADHead {
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
}
impl LONGRELOADHead {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LONGRELOADHead {
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
        }
    }
}
impl Default for LONGRELOADHead { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:674 LC, src/game/character.ts:674)
#[derive(Clone, Copy, Debug)]
pub struct LC {
    pub side: f64,
    pub up: f64,
    pub fwd: f64,
    pub pitch: f64,
    pub aim_k: f64,
    pub yaw: f64,
    pub blade: f64,
    pub lean: f64,
    pub head_yaw: f64,
    pub head_pitch: f64,
    pub head_roll: f64,
    pub elbow_r: f64,
    pub elbow_l: f64,
    pub pole_w: f64,
    pub sway: f64,
    pub anchor: f64,
    pub steady: f64,
    pub count: f64,
}
impl LC {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LC {
            side: 0.0,
            up: 0.0,
            fwd: 0.0,
            pitch: 0.0,
            aim_k: 0.0,
            yaw: 0.0,
            blade: 0.0,
            lean: 0.0,
            head_yaw: 0.0,
            head_pitch: 0.0,
            head_roll: 0.0,
            elbow_r: 0.0,
            elbow_l: 0.0,
            pole_w: 0.0,
            sway: 0.0,
            anchor: 0.0,
            steady: 0.0,
            count: 0.0,
        }
    }
}
impl Default for LC { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:681 WALL_TUCK, src/game/character.ts:681)
#[derive(Clone, Copy, Debug)]
pub struct WALLTUCK {
    pub long: crate::character::WALLTUCKLong,
    pub pistol: crate::character::WALLTUCKPistol,
    pub over: f64,
}
impl WALLTUCK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLTUCK {
            long: crate::character::WALLTUCKLong::zeroed(),
            pistol: crate::character::WALLTUCKPistol::zeroed(),
            over: 0.0,
        }
    }
}
impl Default for WALLTUCK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:682 WALLTUCK.long, src/game/character.ts:682)
#[derive(Clone, Copy, Debug)]
pub struct WALLTUCKLong {
    pub side: f64,
    pub up: f64,
    pub fwd: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub aim_k: f64,
    pub elbow_r: V3,
    pub elbow_l: V3,
}
impl WALLTUCKLong {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLTUCKLong {
            side: 0.0,
            up: 0.0,
            fwd: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            aim_k: 0.0,
            elbow_r: [0.0; 3],
            elbow_l: [0.0; 3],
        }
    }
}
impl Default for WALLTUCKLong { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:683 WALLTUCK.pistol, src/game/character.ts:683)
#[derive(Clone, Copy, Debug)]
pub struct WALLTUCKPistol {
    pub fwd: f64,
    pub up: f64,
    pub pitch: f64,
    pub tuck: f64,
}
impl WALLTUCKPistol {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLTUCKPistol {
            fwd: 0.0,
            up: 0.0,
            pitch: 0.0,
            tuck: 0.0,
        }
    }
}
impl Default for WALLTUCKPistol { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:719 LONG_STEADY, src/game/character.ts:719)
#[derive(Clone, Copy, Debug)]
pub struct CharacterModLONGSTEADY {
    pub tau: f64,
    pub run_yaw: f64,
}
impl CharacterModLONGSTEADY {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterModLONGSTEADY {
            tau: 0.0,
            run_yaw: 0.0,
        }
    }
}
impl Default for CharacterModLONGSTEADY { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:785 FOLD_WITH, src/game/character.ts:785)
#[derive(Clone, Copy, Debug)]
pub struct FOLDWITH {
}
impl FOLDWITH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FOLDWITH {
        }
    }
}
impl Default for FOLDWITH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:802 HAND, src/game/character.ts:802)
#[derive(Clone, Debug)]
pub struct HAND {
    pub can: crate::character::HANDCan,
    pub curl: Vec<f64>,
    pub thumb: f64,
    pub thumb_in: f64,
    pub thumb_bend: f64,
    pub thumb_close: f64,
    pub thumb_own: f64,
    pub spread: f64,
    pub vary: f64,
    pub side_vary: f64,
    pub finger_vary: f64,
    pub far: f64,
    pub trig: Vec<f64>,
    pub trig_spread: f64,
    pub trig_trained: f64,
    pub trig_hold: f64,
}
impl HAND {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HAND {
            can: crate::character::HANDCan::zeroed(),
            curl: Vec::new(),
            thumb: 0.0,
            thumb_in: 0.0,
            thumb_bend: 0.0,
            thumb_close: 0.0,
            thumb_own: 0.0,
            spread: 0.0,
            vary: 0.0,
            side_vary: 0.0,
            finger_vary: 0.0,
            far: 0.0,
            trig: Vec::new(),
            trig_spread: 0.0,
            trig_trained: 0.0,
            trig_hold: 0.0,
        }
    }
}
impl Default for HAND { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:802 HAND.can, src/game/character.ts:802)
#[derive(Clone, Debug)]
pub struct HANDCan {
    pub curl: Vec<f64>,
    pub thumb: f64,
}
impl HANDCan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HANDCan {
            curl: Vec::new(),
            thumb: 0.0,
        }
    }
}
impl Default for HANDCan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:822 WOUND_POSE, src/game/character.ts:822)
#[derive(Clone, Debug)]
pub struct WOUNDPOSEValue {
    pub clip: String,
    pub rate: f64,
    pub hold: [f64; 2],
    pub breath: f64,
    pub in_: f64,
    pub head: f64,
}
impl WOUNDPOSEValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WOUNDPOSEValue {
            clip: String::new(),
            rate: 0.0,
            hold: [0.0; 2],
            breath: 0.0,
            in_: 0.0,
            head: 0.0,
        }
    }
}
impl Default for WOUNDPOSEValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:833 MV2, src/game/character.ts:833)
#[derive(Clone, Debug)]
pub struct MV2 {
    pub reload: crate::character::MV2Reload,
    pub bound: crate::character::MV2Bound,
    pub clutch: crate::character::MV2Clutch,
    pub hurt: crate::character::MV2Hurt,
    pub shaken: crate::character::MV2Shaken,
    pub dip: crate::character::MV2Dip,
}
impl MV2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2 {
            reload: crate::character::MV2Reload::zeroed(),
            bound: crate::character::MV2Bound::zeroed(),
            clutch: crate::character::MV2Clutch::zeroed(),
            hurt: crate::character::MV2Hurt::zeroed(),
            shaken: crate::character::MV2Shaken::zeroed(),
            dip: crate::character::MV2Dip::zeroed(),
        }
    }
}
impl Default for MV2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:834 MV2.reload, src/game/character.ts:834)
#[derive(Clone, Copy, Debug)]
pub struct MV2Reload {
    pub box_: crate::character::MV2ReloadBox,
    pub back: crate::character::MV2ReloadBack,
    pub pouch: [V3; 2],
    pub pole: V3,
    pub pole_w: f64,
    pub fumble: f64,
    pub tap: f64,
    pub flare: crate::character::MV2ReloadFlare,
    pub fling: f64,
    pub rate: f64,
}
impl MV2Reload {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2Reload {
            box_: crate::character::MV2ReloadBox::zeroed(),
            back: crate::character::MV2ReloadBack::zeroed(),
            pouch: [[0.0; 3]; 2],
            pole: [0.0; 3],
            pole_w: 0.0,
            fumble: 0.0,
            tap: 0.0,
            flare: crate::character::MV2ReloadFlare::zeroed(),
            fling: 0.0,
            rate: 0.0,
        }
    }
}
impl Default for MV2Reload { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:835 MV2Reload.box, src/game/character.ts:835)
#[derive(Clone, Copy, Debug)]
pub struct MV2ReloadBox {
    pub mag: V3,
    pub strip: V3,
    pub handle: V3,
    pub pull: V3,
    pub hf: V3,
    pub hb: V3,
}
impl MV2ReloadBox {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2ReloadBox {
            mag: [0.0; 3],
            strip: [0.0; 3],
            handle: [0.0; 3],
            pull: [0.0; 3],
            hf: [0.0; 3],
            hb: [0.0; 3],
        }
    }
}
impl Default for MV2ReloadBox { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:836 MV2Reload.back, src/game/character.ts:836)
#[derive(Clone, Copy, Debug)]
pub struct MV2ReloadBack {
    pub mag: V3,
    pub strip: V3,
    pub handle: V3,
    pub pull: V3,
    pub hf: V3,
    pub hb: V3,
}
impl MV2ReloadBack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2ReloadBack {
            mag: [0.0; 3],
            strip: [0.0; 3],
            handle: [0.0; 3],
            pull: [0.0; 3],
            hf: [0.0; 3],
            hb: [0.0; 3],
        }
    }
}
impl Default for MV2ReloadBack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:841 MV2Reload.flare, src/game/character.ts:841)
#[derive(Clone, Copy, Debug)]
pub struct MV2ReloadFlare {
    pub off: V3,
    pub tap: V3,
    pub pole: V3,
}
impl MV2ReloadFlare {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2ReloadFlare {
            off: [0.0; 3],
            tap: [0.0; 3],
            pole: [0.0; 3],
        }
    }
}
impl Default for MV2ReloadFlare { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:848 MV2.bound, src/game/character.ts:848)
#[derive(Clone, Debug)]
pub struct MV2Bound {
    pub rate: f64,
    pub back: crate::character::MV2BoundBack,
    pub head: crate::character::MV2BoundHead,
}
impl MV2Bound {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2Bound {
            rate: 0.0,
            back: crate::character::MV2BoundBack::zeroed(),
            head: crate::character::MV2BoundHead::zeroed(),
        }
    }
}
impl Default for MV2Bound { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:848 MV2Bound.back, src/game/character.ts:848)
#[derive(Clone, Debug)]
pub struct MV2BoundBack {
    pub up: f64,
    pub out: f64,
    pub behind: f64,
    pub curl: Vec<f64>,
    pub thumb: f64,
}
impl MV2BoundBack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2BoundBack {
            up: 0.0,
            out: 0.0,
            behind: 0.0,
            curl: Vec::new(),
            thumb: 0.0,
        }
    }
}
impl Default for MV2BoundBack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:848 MV2Bound.head, src/game/character.ts:848)
#[derive(Clone, Debug)]
pub struct MV2BoundHead {
    pub up: f64,
    pub out: f64,
    pub behind: f64,
    pub elbow: f64,
    pub curl: Vec<f64>,
    pub knuckle: Vec<f64>,
    pub thumb: f64,
}
impl MV2BoundHead {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2BoundHead {
            up: 0.0,
            out: 0.0,
            behind: 0.0,
            elbow: 0.0,
            curl: Vec::new(),
            knuckle: Vec::new(),
            thumb: 0.0,
        }
    }
}
impl Default for MV2BoundHead { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:849 MV2.clutch, src/game/character.ts:849)
#[derive(Clone, Copy, Debug)]
pub struct MV2Clutch {
    pub rate: f64,
    pub at: V3,
    pub pole: V3,
    pub fold: crate::character::MV2ClutchFold,
}
impl MV2Clutch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2Clutch {
            rate: 0.0,
            at: [0.0; 3],
            pole: [0.0; 3],
            fold: crate::character::MV2ClutchFold::zeroed(),
        }
    }
}
impl Default for MV2Clutch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:850 MV2Clutch.fold, src/game/character.ts:850)
#[derive(Clone, Copy, Debug)]
pub struct MV2ClutchFold {
    pub rate: f64,
    pub pitch: f64,
    pub pitch_too: f64,
    pub head: f64,
    pub drop: f64,
    pub sway: f64,
    pub hz: f64,
}
impl MV2ClutchFold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2ClutchFold {
            rate: 0.0,
            pitch: 0.0,
            pitch_too: 0.0,
            head: 0.0,
            drop: 0.0,
            sway: 0.0,
            hz: 0.0,
        }
    }
}
impl Default for MV2ClutchFold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:857 MV2.hurt, src/game/character.ts:857)
#[derive(Clone, Copy, Debug)]
pub struct MV2Hurt {
    pub rate: f64,
    pub pitch: f64,
    pub lean: f64,
    pub head: f64,
    pub head_brace: f64,
    pub drop: f64,
    pub sway: f64,
    pub hz: f64,
    pub brace: f64,
    pub step_pitch: f64,
    pub step_roll: f64,
    pub step_yaw: f64,
    pub step_head: f64,
    pub step_drop: f64,
    pub step_side: f64,
    pub arm: f64,
}
impl MV2Hurt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2Hurt {
            rate: 0.0,
            pitch: 0.0,
            lean: 0.0,
            head: 0.0,
            head_brace: 0.0,
            drop: 0.0,
            sway: 0.0,
            hz: 0.0,
            brace: 0.0,
            step_pitch: 0.0,
            step_roll: 0.0,
            step_yaw: 0.0,
            step_head: 0.0,
            step_drop: 0.0,
            step_side: 0.0,
            arm: 0.0,
        }
    }
}
impl Default for MV2Hurt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:859 MV2.shaken, src/game/character.ts:859)
#[derive(Clone, Copy, Debug)]
pub struct MV2Shaken {
    pub rate_in: f64,
    pub rate_out: f64,
    pub pitch: f64,
    pub head: f64,
    pub drop: f64,
    pub brace: f64,
}
impl MV2Shaken {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2Shaken {
            rate_in: 0.0,
            rate_out: 0.0,
            pitch: 0.0,
            head: 0.0,
            drop: 0.0,
            brace: 0.0,
        }
    }
}
impl Default for MV2Shaken { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:861 MV2.dip, src/game/character.ts:861)
#[derive(Clone, Debug)]
pub struct MV2Dip {
    pub pitch: f64,
    pub down: f64,
    pub back: f64,
    pub rate_in: f64,
    pub rate_out: f64,
    pub lane: Vec<f64>,
}
impl MV2Dip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MV2Dip {
            pitch: 0.0,
            down: 0.0,
            back: 0.0,
            rate_in: 0.0,
            rate_out: 0.0,
            lane: Vec::new(),
        }
    }
}
impl Default for MV2Dip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:881 TURN_FEEL, src/game/character.ts:881)
#[derive(Clone, Copy, Debug)]
pub struct TURNFEEL {
    pub light: crate::character::TURNFEELLight,
    pub medium: crate::character::TURNFEELMedium,
    pub heavy: crate::character::TURNFEELHeavy,
}
impl TURNFEEL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TURNFEEL {
            light: crate::character::TURNFEELLight::zeroed(),
            medium: crate::character::TURNFEELMedium::zeroed(),
            heavy: crate::character::TURNFEELHeavy::zeroed(),
        }
    }
}
impl Default for TURNFEEL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:882 TURNFEEL.light, src/game/character.ts:882)
#[derive(Clone, Copy, Debug)]
pub struct TURNFEELLight {
    pub aim: f64,
    pub aim_acc: f64,
    pub body: f64,
    pub body_acc: f64,
}
impl TURNFEELLight {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TURNFEELLight {
            aim: 0.0,
            aim_acc: 0.0,
            body: 0.0,
            body_acc: 0.0,
        }
    }
}
impl Default for TURNFEELLight { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:883 TURNFEEL.medium, src/game/character.ts:883)
#[derive(Clone, Copy, Debug)]
pub struct TURNFEELMedium {
    pub aim: f64,
    pub aim_acc: f64,
    pub body: f64,
    pub body_acc: f64,
}
impl TURNFEELMedium {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TURNFEELMedium {
            aim: 0.0,
            aim_acc: 0.0,
            body: 0.0,
            body_acc: 0.0,
        }
    }
}
impl Default for TURNFEELMedium { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:884 TURNFEEL.heavy, src/game/character.ts:884)
#[derive(Clone, Copy, Debug)]
pub struct TURNFEELHeavy {
    pub aim: f64,
    pub aim_acc: f64,
    pub body: f64,
    pub body_acc: f64,
}
impl TURNFEELHeavy {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TURNFEELHeavy {
            aim: 0.0,
            aim_acc: 0.0,
            body: 0.0,
            body_acc: 0.0,
        }
    }
}
impl Default for TURNFEELHeavy { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:969 MV_STARTLE, src/game/character.ts:969)
#[derive(Clone, Debug)]
pub struct MVSTARTLE {
    pub amp: [f64; 2],
    pub dur: [f64; 2],
    pub rear: f64,
    pub turn: f64,
    pub lean: f64,
    pub head: f64,
    pub head_turn: f64,
    pub back: f64,
    pub drop: f64,
    pub gun_amp: f64,
    pub gun_rear: f64,
    pub gun_back: f64,
    pub behind_turn: f64,
    pub behind_head: f64,
    pub behind_pitch: f64,
    pub shrug: f64,
    pub wide: [f64; 2],
    pub raise: [f64; 2],
    pub env: Vec<[f64; 2]>,
}
impl MVSTARTLE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MVSTARTLE {
            amp: [0.0; 2],
            dur: [0.0; 2],
            rear: 0.0,
            turn: 0.0,
            lean: 0.0,
            head: 0.0,
            head_turn: 0.0,
            back: 0.0,
            drop: 0.0,
            gun_amp: 0.0,
            gun_rear: 0.0,
            gun_back: 0.0,
            behind_turn: 0.0,
            behind_head: 0.0,
            behind_pitch: 0.0,
            shrug: 0.0,
            wide: [0.0; 2],
            raise: [0.0; 2],
            env: Vec::new(),
        }
    }
}
impl Default for MVSTARTLE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:981 SNEAK, src/game/character.ts:981)
#[derive(Clone, Copy, Debug)]
pub struct SNEAK {
    pub stride: f64,
    pub pace_stride: f64,
    pub linger: f64,
    pub surge: f64,
    pub drop: f64,
    pub pace_drop: f64,
    pub soft_drop: f64,
    pub lean: f64,
    pub pace_lean: f64,
    pub gun_in: V3,
    pub gun_in_stand: V3,
    pub hand_in: f64,
}
impl SNEAK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNEAK {
            stride: 0.0,
            pace_stride: 0.0,
            linger: 0.0,
            surge: 0.0,
            drop: 0.0,
            pace_drop: 0.0,
            soft_drop: 0.0,
            lean: 0.0,
            pace_lean: 0.0,
            gun_in: [0.0; 3],
            gun_in_stand: [0.0; 3],
            hand_in: 0.0,
        }
    }
}
impl Default for SNEAK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1054 REACH, src/game/character.ts:1054)
#[derive(Clone, Debug)]
pub struct REACH {
    pub in_: f64,
    pub out: f64,
    pub pole: V3,
    pub pole_up: Vec<f64>,
    pub curl: Vec<f64>,
    pub thumb: f64,
}
impl REACH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        REACH {
            in_: 0.0,
            out: 0.0,
            pole: [0.0; 3],
            pole_up: Vec::new(),
            curl: Vec::new(),
            thumb: 0.0,
        }
    }
}
impl Default for REACH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1064 KNIFE_BLOW, src/game/character.ts:1064 KNIFE_BLOW)
#[derive(Clone, Debug)]
pub struct KNIFEBLOW {
    pub snap: f64,
    pub t_clear: f64,
    pub clear: [f64; 2],
    pub az: f64,
    pub reach: f64,
    pub fade: f64,
    pub cross: crate::character::KNIFEBLOWCross,
    pub jab: crate::character::KNIFEBLOWCross,
}
impl KNIFEBLOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KNIFEBLOW {
            snap: 0.0,
            t_clear: 0.0,
            clear: [0.0; 2],
            az: 0.0,
            reach: 0.0,
            fade: 0.0,
            cross: crate::character::KNIFEBLOWCross::zeroed(),
            jab: crate::character::KNIFEBLOWCross::zeroed(),
        }
    }
}
impl Default for KNIFEBLOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at KNIFEBLOW.cross, src/game/character.ts:1064)
#[derive(Clone, Debug)]
pub struct KNIFEBLOWCross {
    pub end: f64,
    pub hit: f64,
    pub keys: Vec<crate::character::BlowKey>,
}
impl KNIFEBLOWCross {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KNIFEBLOWCross {
            end: 0.0,
            hit: 0.0,
            keys: Vec::new(),
        }
    }
}
impl Default for KNIFEBLOWCross { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1077 KNIFE_SLASH, src/game/character.ts:1077)
#[derive(Clone, Debug)]
pub struct KNIFESLASH {
    pub end: f64,
    pub hit: f64,
    pub keys: Vec<crate::character::BlowKey>,
}
impl KNIFESLASH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KNIFESLASH {
            end: 0.0,
            hit: 0.0,
            keys: Vec::new(),
        }
    }
}
impl Default for KNIFESLASH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1084 KNIFE_BACK, src/game/character.ts:1084)
#[derive(Clone, Debug)]
pub struct KNIFEBACK {
    pub end: f64,
    pub hit: f64,
    pub keys: Vec<crate::character::BlowKey>,
}
impl KNIFEBACK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KNIFEBACK {
            end: 0.0,
            hit: 0.0,
            keys: Vec::new(),
        }
    }
}
impl Default for KNIFEBACK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1091 BLOW, src/game/character.ts:1091)
#[derive(Clone, Debug)]
pub struct BLOW {
    pub rise: JsMap<String, f64>,
    pub clip: JsMap<String, f64>,
    pub reach: f64,
    pub fist: Vec<f64>,
    pub open: Vec<f64>,
    pub loose: Vec<f64>,
    pub cup: Vec<f64>,
    pub legs: crate::character::BLOWLegs,
    pub tail: f64,
    pub sink: JsMap<String, f64>,
    pub walk: Vec<f64>,
}
impl BLOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BLOW {
            rise: JsMap::new(),
            clip: JsMap::new(),
            reach: 0.0,
            fist: Vec::new(),
            open: Vec::new(),
            loose: Vec::new(),
            cup: Vec::new(),
            legs: crate::character::BLOWLegs::zeroed(),
            tail: 0.0,
            sink: JsMap::new(),
            walk: Vec::new(),
        }
    }
}
impl Default for BLOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1097 BLOW.legs, src/game/character.ts:1097)
#[derive(Clone, Copy, Debug)]
pub struct BLOWLegs {
    pub gap: f64,
    pub lift: f64,
    pub straight: f64,
    pub up: f64,
    pub fwd: f64,
    pub out: f64,
}
impl BLOWLegs {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BLOWLegs {
            gap: 0.0,
            lift: 0.0,
            straight: 0.0,
            up: 0.0,
            fwd: 0.0,
            out: 0.0,
        }
    }
}
impl Default for BLOWLegs { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1102 TAKEN, src/game/character.ts:1102)
#[derive(Clone, Debug)]
pub struct TAKEN {
    pub sag: JsMap<String, f64>,
    pub secs: f64,
}
impl TAKEN {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TAKEN {
            sag: JsMap::new(),
            secs: 0.0,
        }
    }
}
impl Default for TAKEN { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1146 CREEP, src/game/character.ts:1146)
#[derive(Clone, Debug)]
pub struct CREEP {
    pub in_: f64,
    pub out: f64,
    pub gait_in: f64,
    pub gait_out: f64,
    pub drop: Vec<f64>,
    pub back: Vec<f64>,
    pub lean: Vec<f64>,
    pub round: f64,
    pub look: f64,
    pub low: crate::character::CREEPLow,
    pub wide: crate::character::CREEPWide,
    pub heft: crate::character::CREEPHeft,
    pub l: crate::character::CREEPL,
    pub r: crate::character::CREEPR,
}
impl CREEP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CREEP {
            in_: 0.0,
            out: 0.0,
            gait_in: 0.0,
            gait_out: 0.0,
            drop: Vec::new(),
            back: Vec::new(),
            lean: Vec::new(),
            round: 0.0,
            look: 0.0,
            low: crate::character::CREEPLow::zeroed(),
            wide: crate::character::CREEPWide::zeroed(),
            heft: crate::character::CREEPHeft::zeroed(),
            l: crate::character::CREEPL::zeroed(),
            r: crate::character::CREEPR::zeroed(),
        }
    }
}
impl Default for CREEP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1151 CREEP.low, src/game/character.ts:1151)
#[derive(Clone, Copy, Debug)]
pub struct CREEPLow {
    pub rate: f64,
    pub up: f64,
}
impl CREEPLow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CREEPLow {
            rate: 0.0,
            up: 0.0,
        }
    }
}
impl Default for CREEPLow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1157 CREEP.wide, src/game/character.ts:1157)
#[derive(Clone, Copy, Debug)]
pub struct CREEPWide {
    pub in_: f64,
    pub out: f64,
    pub still: f64,
    pub pole: V3,
    pub fing: V3,
    pub back: V3,
    pub curl: f64,
    pub thumb: f64,
    pub spread: f64,
}
impl CREEPWide {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CREEPWide {
            in_: 0.0,
            out: 0.0,
            still: 0.0,
            pole: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
            curl: 0.0,
            thumb: 0.0,
            spread: 0.0,
        }
    }
}
impl Default for CREEPWide { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1163 CREEP.heft, src/game/character.ts:1163)
#[derive(Clone, Copy, Debug)]
pub struct CREEPHeft {
    pub stride: f64,
    pub lift: f64,
    pub run: f64,
    pub trunk: [f64; 2],
    pub arm: [f64; 2],
    pub body: [f64; 2],
    pub go: [f64; 2],
    pub out: f64,
    pub up: f64,
    pub breath: f64,
    pub breath_hz: f64,
    pub dt: f64,
}
impl CREEPHeft {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CREEPHeft {
            stride: 0.0,
            lift: 0.0,
            run: 0.0,
            trunk: [0.0; 2],
            arm: [0.0; 2],
            body: [0.0; 2],
            go: [0.0; 2],
            out: 0.0,
            up: 0.0,
            breath: 0.0,
            breath_hz: 0.0,
            dt: 0.0,
        }
    }
}
impl Default for CREEPHeft { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1164 CREEP.L, src/game/character.ts:1164)
#[derive(Clone, Copy, Debug)]
pub struct CREEPL {
    pub at: V3,
    pub at_c: V3,
    pub pole: V3,
    pub fing: V3,
    pub back: V3,
    pub curl: f64,
    pub thumb: f64,
    pub spread: f64,
}
impl CREEPL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CREEPL {
            at: [0.0; 3],
            at_c: [0.0; 3],
            pole: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
            curl: 0.0,
            thumb: 0.0,
            spread: 0.0,
        }
    }
}
impl Default for CREEPL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1165 CREEP.R, src/game/character.ts:1165)
#[derive(Clone, Copy, Debug)]
pub struct CREEPR {
    pub at: V3,
    pub at_c: V3,
    pub pole: V3,
    pub fing: V3,
    pub back: V3,
    pub curl: f64,
    pub thumb: f64,
    pub spread: f64,
}
impl CREEPR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CREEPR {
            at: [0.0; 3],
            at_c: [0.0; 3],
            pole: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
            curl: 0.0,
            thumb: 0.0,
            spread: 0.0,
        }
    }
}
impl Default for CREEPR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1175 READY_BARE, src/game/character.ts:1175)
#[derive(Clone, Copy, Debug)]
pub struct READYBARE {
    pub l: crate::character::READYBAREL,
    pub r: crate::character::READYBARER,
    pub way: crate::character::READYBAREWay,
    pub out: [f64; 2],
    pub out_r: f64,
}
impl READYBARE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        READYBARE {
            l: crate::character::READYBAREL::zeroed(),
            r: crate::character::READYBARER::zeroed(),
            way: crate::character::READYBAREWay::zeroed(),
            out: [0.0; 2],
            out_r: 0.0,
        }
    }
}
impl Default for READYBARE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1176 READYBARE.L, src/game/character.ts:1176)
#[derive(Clone, Copy, Debug)]
pub struct READYBAREL {
    pub pole: V3,
    pub fing: V3,
    pub back: V3,
    pub curl: f64,
    pub thumb: f64,
    pub spread: f64,
    pub frac: f64,
    pub dy: f64,
    pub r: [f64; 2],
    pub short: f64,
}
impl READYBAREL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        READYBAREL {
            pole: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
            curl: 0.0,
            thumb: 0.0,
            spread: 0.0,
            frac: 0.0,
            dy: 0.0,
            r: [0.0; 2],
            short: 0.0,
        }
    }
}
impl Default for READYBAREL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1177 READYBARE.R, src/game/character.ts:1177)
#[derive(Clone, Copy, Debug)]
pub struct READYBARER {
    pub at: V3,
    pub pole: V3,
    pub fing: V3,
    pub back: V3,
    pub curl: f64,
    pub thumb: f64,
}
impl READYBARER {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        READYBARER {
            at: [0.0; 3],
            pole: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
            curl: 0.0,
            thumb: 0.0,
        }
    }
}
impl Default for READYBARER { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1183 READYBARE.way, src/game/character.ts:1183)
#[derive(Clone, Copy, Debug)]
pub struct READYBAREWay {
    pub by: f64,
    pub low: f64,
    pub near: f64,
    pub lift: [f64; 2],
    pub right: [f64; 2],
    pub rate: f64,
    pub drop: f64,
}
impl READYBAREWay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        READYBAREWay {
            by: 0.0,
            low: 0.0,
            near: 0.0,
            lift: [0.0; 2],
            right: [0.0; 2],
            rate: 0.0,
            drop: 0.0,
        }
    }
}
impl Default for READYBAREWay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1193 READY_BARE_WAS, src/game/character.ts:1193)
#[derive(Clone, Copy, Debug)]
pub struct READYBAREWAS {
    pub l: crate::character::READYBAREWASL,
    pub r: crate::character::READYBAREWASR,
}
impl READYBAREWAS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        READYBAREWAS {
            l: crate::character::READYBAREWASL::zeroed(),
            r: crate::character::READYBAREWASR::zeroed(),
        }
    }
}
impl Default for READYBAREWAS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1194 READYBAREWAS.L, src/game/character.ts:1194)
#[derive(Clone, Copy, Debug)]
pub struct READYBAREWASL {
    pub pole: V3,
    pub fing: V3,
    pub curl: f64,
    pub spread: f64,
    pub back: V3,
    pub thumb: f64,
    pub frac: f64,
    pub dy: f64,
    pub r: [f64; 2],
    pub short: f64,
}
impl READYBAREWASL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        READYBAREWASL {
            pole: [0.0; 3],
            fing: [0.0; 3],
            curl: 0.0,
            spread: 0.0,
            back: [0.0; 3],
            thumb: 0.0,
            frac: 0.0,
            dy: 0.0,
            r: [0.0; 2],
            short: 0.0,
        }
    }
}
impl Default for READYBAREWASL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1195 READYBAREWAS.R, src/game/character.ts:1195)
#[derive(Clone, Copy, Debug)]
pub struct READYBAREWASR {
    pub at: V3,
    pub pole: V3,
    pub fing: V3,
    pub curl: f64,
    pub back: V3,
    pub thumb: f64,
}
impl READYBAREWASR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        READYBAREWASR {
            at: [0.0; 3],
            pole: [0.0; 3],
            fing: [0.0; 3],
            curl: 0.0,
            back: [0.0; 3],
            thumb: 0.0,
        }
    }
}
impl Default for READYBAREWASR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1202 READY_UP, src/game/character.ts:1202)
#[derive(Clone, Copy, Debug)]
pub struct READYUP {
    pub over: [f64; 2],
    pub rise: f64,
    pub look: f64,
    pub dy: f64,
    pub r: [f64; 2],
    pub fing: f64,
    pub slow: f64,
    pub rate: f64,
}
impl READYUP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        READYUP {
            over: [0.0; 2],
            rise: 0.0,
            look: 0.0,
            dy: 0.0,
            r: [0.0; 2],
            fing: 0.0,
            slow: 0.0,
            rate: 0.0,
        }
    }
}
impl Default for READYUP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1221 SMOKEARM, src/game/character.ts:1221)
#[derive(Clone, Copy, Debug)]
pub struct SMOKEARM {
    pub at: V3,
    pub fingers: V3,
    pub back: V3,
    pub pole: V3,
    pub shrug: f64,
    pub curl: [f64; 4],
    pub thumb: f64,
    pub rate: f64,
    pub away: f64,
}
impl SMOKEARM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SMOKEARM {
            at: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
            pole: [0.0; 3],
            shrug: 0.0,
            curl: [0.0; 4],
            thumb: 0.0,
            rate: 0.0,
            away: 0.0,
        }
    }
}
impl Default for SMOKEARM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1277 RD, src/game/character.ts:1277)
#[derive(Clone, Copy, Debug)]
pub struct RD {
    pub l: f64,
    pub r: f64,
    pub l_pole: f64,
    pub r_pole: f64,
    pub l_fing: f64,
    pub l_back: f64,
    pub r_fing: f64,
    pub r_back: f64,
    pub lean: f64,
    pub blade: f64,
    pub w_r: f64,
    pub l_curl: f64,
    pub l_thumb: f64,
    pub l_spread: f64,
    pub r_curl: f64,
    pub r_thumb: f64,
    pub clav: f64,
    pub into: f64,
    pub up: f64,
}
impl RD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RD {
            l: 0.0,
            r: 0.0,
            l_pole: 0.0,
            r_pole: 0.0,
            l_fing: 0.0,
            l_back: 0.0,
            r_fing: 0.0,
            r_back: 0.0,
            lean: 0.0,
            blade: 0.0,
            w_r: 0.0,
            l_curl: 0.0,
            l_thumb: 0.0,
            l_spread: 0.0,
            r_curl: 0.0,
            r_thumb: 0.0,
            clav: 0.0,
            into: 0.0,
            up: 0.0,
        }
    }
}
impl Default for RD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1288 THR, src/game/character.ts:1288)
#[derive(Clone, Debug)]
pub struct THR {
    pub chest_yaw: Vec<[f64; 2]>,
    pub pelvis_yaw: Vec<[f64; 2]>,
    pub pitch: Vec<[f64; 2]>,
    pub roll: Vec<[f64; 2]>,
    pub pel_x: Vec<[f64; 2]>,
    pub pel_y: Vec<[f64; 2]>,
    pub pel_z: Vec<[f64; 2]>,
    pub clav_up: Vec<[f64; 2]>,
    pub clav_fwd: Vec<[f64; 2]>,
    pub hand_r: Vec<(f64, Vec<f64>)>,
    pub swiv_r: Vec<[f64; 2]>,
    pub fing_r: Vec<(f64, Vec<f64>)>,
    pub back_r: Vec<(f64, Vec<f64>)>,
    pub hand_l: Vec<(f64, Vec<f64>)>,
    pub swiv_l: Vec<[f64; 2]>,
    pub fing_l: Vec<(f64, Vec<f64>)>,
    pub back_l: Vec<(f64, Vec<f64>)>,
}
impl THR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        THR {
            chest_yaw: Vec::new(),
            pelvis_yaw: Vec::new(),
            pitch: Vec::new(),
            roll: Vec::new(),
            pel_x: Vec::new(),
            pel_y: Vec::new(),
            pel_z: Vec::new(),
            clav_up: Vec::new(),
            clav_fwd: Vec::new(),
            hand_r: Vec::new(),
            swiv_r: Vec::new(),
            fing_r: Vec::new(),
            back_r: Vec::new(),
            hand_l: Vec::new(),
            swiv_l: Vec::new(),
            fing_l: Vec::new(),
            back_l: Vec::new(),
        }
    }
}
impl Default for THR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1344 THR_LOW, src/game/character.ts:1344)
#[derive(Clone, Debug)]
pub struct THRLOW {
    pub chest_yaw: Vec<[f64; 2]>,
    pub pelvis_yaw: Vec<[f64; 2]>,
    pub pitch: Vec<[f64; 2]>,
    pub roll: Vec<[f64; 2]>,
    pub pel_x: Vec<[f64; 2]>,
    pub pel_y: Vec<[f64; 2]>,
    pub pel_z: Vec<[f64; 2]>,
    pub clav_up: Vec<[f64; 2]>,
    pub clav_fwd: Vec<[f64; 2]>,
    pub hand_r: Vec<(f64, Vec<f64>)>,
    pub swiv_r: Vec<[f64; 2]>,
    pub fing_r: Vec<(f64, Vec<f64>)>,
    pub back_r: Vec<(f64, Vec<f64>)>,
}
impl THRLOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        THRLOW {
            chest_yaw: Vec::new(),
            pelvis_yaw: Vec::new(),
            pitch: Vec::new(),
            roll: Vec::new(),
            pel_x: Vec::new(),
            pel_y: Vec::new(),
            pel_z: Vec::new(),
            clav_up: Vec::new(),
            clav_fwd: Vec::new(),
            hand_r: Vec::new(),
            swiv_r: Vec::new(),
            fing_r: Vec::new(),
            back_r: Vec::new(),
        }
    }
}
impl Default for THRLOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1381 BIGMOVE, src/game/character.ts:1381)
#[derive(Clone, Debug)]
pub struct BIGMOVE {
    pub hide: f64,
    pub grab: f64,
    pub take: Vec<f64>,
    pub join: Vec<f64>,
    pub put: f64,
    pub put_at: f64,
    pub unjoin: f64,
    pub show: f64,
    pub snug: crate::character::BIGMOVESnug,
    pub arc: V3,
    pub wait: V3,
    pub hammer: crate::character::BIGMOVEHammer,
    pub shotgun: crate::character::BIGMOVEShotgun,
    pub fist: Vec<f64>,
    pub open: Vec<f64>,
}
impl BIGMOVE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGMOVE {
            hide: 0.0,
            grab: 0.0,
            take: Vec::new(),
            join: Vec::new(),
            put: 0.0,
            put_at: 0.0,
            unjoin: 0.0,
            show: 0.0,
            snug: crate::character::BIGMOVESnug::zeroed(),
            arc: [0.0; 3],
            wait: [0.0; 3],
            hammer: crate::character::BIGMOVEHammer::zeroed(),
            shotgun: crate::character::BIGMOVEShotgun::zeroed(),
            fist: Vec::new(),
            open: Vec::new(),
        }
    }
}
impl Default for BIGMOVE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1383 BIGMOVE.snug, src/game/character.ts:1383)
#[derive(Clone, Debug)]
pub struct BIGMOVESnug {
    pub hide: f64,
    pub take: Vec<f64>,
    pub put_at: f64,
    pub show: f64,
}
impl BIGMOVESnug {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGMOVESnug {
            hide: 0.0,
            take: Vec::new(),
            put_at: 0.0,
            show: 0.0,
        }
    }
}
impl Default for BIGMOVESnug { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1385 BIGMOVE.hammer, src/game/character.ts:1385)
#[derive(Clone, Copy, Debug)]
pub struct BIGMOVEHammer {
    pub ready: crate::character::BIGMOVEHammerReady,
    pub wound: crate::character::BIGMOVEHammerWound,
    pub over: crate::character::BIGMOVEHammerOver,
    pub grab_at: f64,
    pub raise: f64,
    pub swing: f64,
    pub swing_min: f64,
    pub swing_max: f64,
    pub rebound: f64,
    pub settle: f64,
    pub butt: V3,
    pub far: [f64; 2],
    pub wound_body: crate::character::BIGMOVEHammerWoundBody,
    pub blow_body: crate::character::BIGMOVEHammerBlowBody,
    pub face: f64,
    pub drop: f64,
    pub stance: crate::character::BIGMOVEHammerStance,
}
impl BIGMOVEHammer {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGMOVEHammer {
            ready: crate::character::BIGMOVEHammerReady::zeroed(),
            wound: crate::character::BIGMOVEHammerWound::zeroed(),
            over: crate::character::BIGMOVEHammerOver::zeroed(),
            grab_at: 0.0,
            raise: 0.0,
            swing: 0.0,
            swing_min: 0.0,
            swing_max: 0.0,
            rebound: 0.0,
            settle: 0.0,
            butt: [0.0; 3],
            far: [0.0; 2],
            wound_body: crate::character::BIGMOVEHammerWoundBody::zeroed(),
            blow_body: crate::character::BIGMOVEHammerBlowBody::zeroed(),
            face: 0.0,
            drop: 0.0,
            stance: crate::character::BIGMOVEHammerStance::zeroed(),
        }
    }
}
impl Default for BIGMOVEHammer { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1386 BIGMOVEHammer.ready, src/game/character.ts:1386)
#[derive(Clone, Copy, Debug)]
pub struct BIGMOVEHammerReady {
    pub g: V3,
    pub d: V3,
}
impl BIGMOVEHammerReady {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGMOVEHammerReady {
            g: [0.0; 3],
            d: [0.0; 3],
        }
    }
}
impl Default for BIGMOVEHammerReady { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1387 BIGMOVEHammer.wound, src/game/character.ts:1387)
#[derive(Clone, Copy, Debug)]
pub struct BIGMOVEHammerWound {
    pub g: V3,
    pub d: V3,
}
impl BIGMOVEHammerWound {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGMOVEHammerWound {
            g: [0.0; 3],
            d: [0.0; 3],
        }
    }
}
impl Default for BIGMOVEHammerWound { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1388 BIGMOVEHammer.over, src/game/character.ts:1388)
#[derive(Clone, Copy, Debug)]
pub struct BIGMOVEHammerOver {
    pub g: V3,
    pub d: V3,
}
impl BIGMOVEHammerOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGMOVEHammerOver {
            g: [0.0; 3],
            d: [0.0; 3],
        }
    }
}
impl Default for BIGMOVEHammerOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1391 BIGMOVEHammer.woundBody, src/game/character.ts:1391)
#[derive(Clone, Copy, Debug)]
pub struct BIGMOVEHammerWoundBody {
    pub yaw: f64,
    pub pel_yaw: f64,
    pub pitch: f64,
    pub back: f64,
    pub side: f64,
}
impl BIGMOVEHammerWoundBody {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGMOVEHammerWoundBody {
            yaw: 0.0,
            pel_yaw: 0.0,
            pitch: 0.0,
            back: 0.0,
            side: 0.0,
        }
    }
}
impl Default for BIGMOVEHammerWoundBody { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1392 BIGMOVEHammer.blowBody, src/game/character.ts:1392)
#[derive(Clone, Copy, Debug)]
pub struct BIGMOVEHammerBlowBody {
    pub yaw: f64,
    pub pel_yaw: f64,
    pub pitch: f64,
    pub pitch_far: f64,
    pub fwd: f64,
    pub fwd_far: f64,
    pub drop: f64,
}
impl BIGMOVEHammerBlowBody {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGMOVEHammerBlowBody {
            yaw: 0.0,
            pel_yaw: 0.0,
            pitch: 0.0,
            pitch_far: 0.0,
            fwd: 0.0,
            fwd_far: 0.0,
            drop: 0.0,
        }
    }
}
impl Default for BIGMOVEHammerBlowBody { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1393 BIGMOVEHammer.stance, src/game/character.ts:1393)
#[derive(Clone, Copy, Debug)]
pub struct BIGMOVEHammerStance {
    pub l: [f64; 2],
    pub r: [f64; 2],
    pub lift: f64,
}
impl BIGMOVEHammerStance {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGMOVEHammerStance {
            l: [0.0; 2],
            r: [0.0; 2],
            lift: 0.0,
        }
    }
}
impl Default for BIGMOVEHammerStance { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1395 BIGMOVE.shotgun, src/game/character.ts:1395)
#[derive(Clone, Copy, Debug)]
pub struct BIGMOVEShotgun {
    pub pocket: V3,
    pub yaw: f64,
    pub pel_yaw: f64,
    pub pitch: f64,
    pub kick: f64,
    pub climb: f64,
    pub kick_in: f64,
    pub kick_out: f64,
    pub kick_pitch: f64,
    pub shove: f64,
}
impl BIGMOVEShotgun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGMOVEShotgun {
            pocket: [0.0; 3],
            yaw: 0.0,
            pel_yaw: 0.0,
            pitch: 0.0,
            kick: 0.0,
            climb: 0.0,
            kick_in: 0.0,
            kick_out: 0.0,
            kick_pitch: 0.0,
            shove: 0.0,
        }
    }
}
impl Default for BIGMOVEShotgun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum AnimTablesValue {
    Num(f64),
    Arm1,
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MvOnK {
    Reload,
    Fold,
    Steady,
    Toss,
    On,
    Entry,
    Flinch,
    Fall,
    Squeeze,
    Snap,
    Recoil,
    Dazzle,
    Gait,
    Bound,
    Clutch,
    EntryEyes,
    SteadyRun,
}
impl MvOnK {
    pub const fn as_str(self) -> &'static str {
        match self {
            MvOnK::Reload => "reload",
            MvOnK::Fold => "fold",
            MvOnK::Steady => "steady",
            MvOnK::Toss => "toss",
            MvOnK::On => "on",
            MvOnK::Entry => "entry",
            MvOnK::Flinch => "flinch",
            MvOnK::Fall => "fall",
            MvOnK::Squeeze => "squeeze",
            MvOnK::Snap => "snap",
            MvOnK::Recoil => "recoil",
            MvOnK::Dazzle => "dazzle",
            MvOnK::Gait => "gait",
            MvOnK::Bound => "bound",
            MvOnK::Clutch => "clutch",
            MvOnK::EntryEyes => "entryEyes",
            MvOnK::SteadyRun => "steadyRun",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:1116 blowsNear(B), src/game/character.ts:1116)
#[derive(Clone, Debug)]
pub struct BlowsNearB {
    pub t: f64,
    pub end: f64,
    pub blows: Vec<f64>,
}
impl BlowsNearB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlowsNearB {
            t: 0.0,
            end: 0.0,
            blows: Vec::new(),
        }
    }
}
impl Default for BlowsNearB { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:1132 throwTables ->, src/game/character.ts:1132)
#[derive(Clone, Debug)]
pub struct ThrowTables {
    pub k1: Vec<Vec<[f64; 2]>>,
    pub k3: Vec<Vec<(f64, Vec<f64>)>>,
}
impl ThrowTables {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThrowTables {
            k1: Vec::new(),
            k3: Vec::new(),
        }
    }
}
impl Default for ThrowTables { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BigReboundKind {
    Shotgun,
    Hammer,
}
impl BigReboundKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            BigReboundKind::Shotgun => "shotgun",
            BigReboundKind::Hammer => "hammer",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:1402 bigOff(B), src/game/character.ts:1402)
#[derive(Clone, Copy, Debug)]
pub struct BigOffB {
    pub t: f64,
    pub equip: f64,
    pub stow: f64,
    pub end: f64,
    pub snug: Option<bool>,
}
impl BigOffB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BigOffB {
            t: 0.0,
            equip: 0.0,
            stow: 0.0,
            end: 0.0,
            snug: None,
        }
    }
}
impl Default for BigOffB { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:2032 kitSwap(shown), src/game/character.ts:2032)
#[derive(Clone, Copy, Debug)]
pub struct AnimKitSwapShown {
    pub r: V3,
    pub l: V3,
    pub fingers: V3,
    pub back: V3,
}
impl AnimKitSwapShown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimKitSwapShown {
            r: [0.0; 3],
            l: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for AnimKitSwapShown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:2081 reloadShow ->, src/game/character.ts:2081)
#[derive(Clone, Copy, Debug)]
pub struct AnimReloadShow {
    pub out: bool,
    pub hand: bool,
}
impl AnimReloadShow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimReloadShow {
            out: false,
            hand: false,
        }
    }
}
impl Default for AnimReloadShow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:2088 feedClock ->, src/game/character.ts:2088)
#[derive(Clone, Copy, Debug)]
pub struct AnimFeedClock {
    pub t: f64,
    pub dur: f64,
    pub out: f64,
    pub in_: f64,
    pub rack: f64,
    pub pull: bool,
    pub keep: f64,
    pub fresh: f64,
}
impl AnimFeedClock {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimFeedClock {
            t: 0.0,
            dur: 0.0,
            out: 0.0,
            in_: 0.0,
            rack: 0.0,
            pull: false,
            keep: 0.0,
            fresh: 0.0,
        }
    }
}
impl Default for AnimFeedClock { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimThrowRead {
    Toss,
    Short,
    Full,
}
impl AnimThrowRead {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimThrowRead::Toss => "toss",
            AnimThrowRead::Short => "short",
            AnimThrowRead::Full => "full",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:2142 kickDoor(opts), src/game/character.ts:2142)
#[derive(Clone, Copy, Debug)]
pub struct AnimKickDoorOpts {
    pub leg: Option<crate::character::AnimKickDoorOptsLeg>,
}
impl AnimKickDoorOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimKickDoorOpts {
            leg: None,
        }
    }
}
impl Default for AnimKickDoorOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimKickDoorOptsLeg {
    Left,
    Right,
}
impl AnimKickDoorOptsLeg {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimKickDoorOptsLeg::Left => "left",
            AnimKickDoorOptsLeg::Right => "right",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimKickingLeg {
    Left,
    Right,
}
impl AnimKickingLeg {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimKickingLeg::Left => "left",
            AnimKickingLeg::Right => "right",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:2678 woundLimbs(wd), src/game/character.ts:2678)
#[derive(Clone, Copy, Debug)]
pub struct AnimWoundLimbsWd {
    pub pose: crate::character::AnimWoundLimbsWdPose,
    pub at: crate::character::AnimWoundLimbsWdAt,
}
impl AnimWoundLimbsWd {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWoundLimbsWd {
            pose: crate::character::AnimWoundLimbsWdPose::Knee,
            at: crate::character::AnimWoundLimbsWdAt::Ribs,
        }
    }
}
impl Default for AnimWoundLimbsWd { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimWoundLimbsWdPose {
    Knee,
    Back,
}
impl AnimWoundLimbsWdPose {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimWoundLimbsWdPose::Knee => "knee",
            AnimWoundLimbsWdPose::Back => "back",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimWoundLimbsWdAt {
    Ribs,
    Thigh,
}
impl AnimWoundLimbsWdAt {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimWoundLimbsWdAt::Ribs => "ribs",
            AnimWoundLimbsWdAt::Thigh => "thigh",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:2683 woundLimbsAsk(wd), src/game/character.ts:2683)
#[derive(Clone, Copy, Debug)]
pub struct AnimWoundLimbsAskWd {
    pub pose: crate::character::AnimWoundLimbsAskWdPose,
    pub at: crate::character::AnimWoundLimbsAskWdAt,
}
impl AnimWoundLimbsAskWd {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWoundLimbsAskWd {
            pose: crate::character::AnimWoundLimbsAskWdPose::Knee,
            at: crate::character::AnimWoundLimbsAskWdAt::Ribs,
        }
    }
}
impl Default for AnimWoundLimbsAskWd { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimWoundLimbsAskWdPose {
    Knee,
    Back,
}
impl AnimWoundLimbsAskWdPose {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimWoundLimbsAskWdPose::Knee => "knee",
            AnimWoundLimbsAskWdPose::Back => "back",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimWoundLimbsAskWdAt {
    Ribs,
    Thigh,
}
impl AnimWoundLimbsAskWdAt {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimWoundLimbsAskWdAt::Ribs => "ribs",
            AnimWoundLimbsAskWdAt::Thigh => "thigh",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:2802 trunkFrame ->, src/game/character.ts:2802)
#[derive(Clone, Copy, Debug)]
pub struct AnimTrunkFrame {
    pub left: V3,
    pub up: V3,
    pub fwd: V3,
}
impl AnimTrunkFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimTrunkFrame {
            left: [0.0; 3],
            up: [0.0; 3],
            fwd: [0.0; 3],
        }
    }
}
impl Default for AnimTrunkFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum AnimReloadArmAskOwn {
    Arm0(crate::gunreload::FeedPathArm0),
    Arm1(crate::gunreload::FeedPathArm1),
}

/// (TypeScript: inline at src/game/character.ts:3106 stanceOf ->, src/game/character.ts:3106)
#[derive(Clone, Copy, Debug)]
pub struct AnimStanceOf {
    pub c: f64,
    pub c_l: f64,
    pub c_r: f64,
}
impl AnimStanceOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimStanceOf {
            c: 0.0,
            c_l: 0.0,
            c_r: 0.0,
        }
    }
}
impl Default for AnimStanceOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:3263 lgChannels ->, src/game/character.ts:3263)
#[derive(Clone, Copy, Debug)]
pub struct AnimLgChannelsItem {
    pub x: f64,
}
impl AnimLgChannelsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimLgChannelsItem {
            x: 0.0,
        }
    }
}
impl Default for AnimLgChannelsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:3889 takenLayer(T), src/game/character.ts:3889)
#[derive(Clone, Copy, Debug)]
pub struct AnimTakenLayerT {
    pub style: crate::character::TakenStyle,
    pub t: f64,
    pub delay: f64,
    pub off: f64,
}
impl AnimTakenLayerT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimTakenLayerT {
            style: crate::character::TakenStyle::Arch,
            t: 0.0,
            delay: 0.0,
            off: 0.0,
        }
    }
}
impl Default for AnimTakenLayerT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:3895 takenLayerAsk(T), src/game/character.ts:3895)
#[derive(Clone, Copy, Debug)]
pub struct AnimTakenLayerAskT {
    pub style: crate::character::TakenStyle,
    pub off: f64,
}
impl AnimTakenLayerAskT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimTakenLayerAskT {
            style: crate::character::TakenStyle::Arch,
            off: 0.0,
        }
    }
}
impl Default for AnimTakenLayerAskT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:3912 knifePose(K), src/game/character.ts:3912)
#[derive(Clone, Debug)]
pub struct AnimKnifePoseK {
    pub end: f64,
    pub keys: Vec<crate::character::BlowKey>,
}
impl AnimKnifePoseK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimKnifePoseK {
            end: 0.0,
            keys: Vec::new(),
        }
    }
}
impl Default for AnimKnifePoseK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:3917 knifePoseAsk(K), src/game/character.ts:3917)
#[derive(Clone, Debug)]
pub struct AnimKnifePoseAskK {
    pub end: f64,
    pub keys: Vec<crate::character::BlowKey>,
}
impl AnimKnifePoseAskK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimKnifePoseAskK {
            end: 0.0,
            keys: Vec::new(),
        }
    }
}
impl Default for AnimKnifePoseAskK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:3961 doorHands ->, src/game/character.ts:3961)
#[derive(Clone, Copy, Debug)]
pub struct AnimDoorHands {
    pub l: f64,
    pub r: f64,
    pub right: bool,
}
impl AnimDoorHands {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimDoorHands {
            l: 0.0,
            r: 0.0,
            right: false,
        }
    }
}
impl Default for AnimDoorHands { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:3997 workInfo ->, src/game/character.ts:3997)
#[derive(Clone, Debug)]
pub struct AnimWorkInfo {
    pub pose: Option<crate::character::WorkPose>,
    pub w: f64,
    pub hand: f64,
    pub two: bool,
}
impl AnimWorkInfo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimWorkInfo {
            pose: None,
            w: 0.0,
            hand: 0.0,
            two: false,
        }
    }
}
impl Default for AnimWorkInfo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimSwapHeldRule {
    Torch,
    Weapon,
}
impl AnimSwapHeldRule {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimSwapHeldRule::Torch => "torch",
            AnimSwapHeldRule::Weapon => "weapon",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimSwapHeld {
    Torch,
    Weapon,
}
impl AnimSwapHeld {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimSwapHeld::Torch => "torch",
            AnimSwapHeld::Weapon => "weapon",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:4011 swapInfo ->, src/game/character.ts:4011)
#[derive(Clone, Copy, Debug)]
pub struct AnimSwapInfo {
    pub a: f64,
    pub to: Option<crate::character::AnimSwapInfoTo>,
    pub pending: bool,
}
impl AnimSwapInfo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimSwapInfo {
            a: 0.0,
            to: None,
            pending: false,
        }
    }
}
impl Default for AnimSwapInfo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimSwapInfoTo {
    Torch,
    Weapon,
}
impl AnimSwapInfoTo {
    pub const fn as_str(self) -> &'static str {
        match self {
            AnimSwapInfoTo::Torch => "torch",
            AnimSwapInfoTo::Weapon => "weapon",
        }
    }
}

/// (TypeScript: inline at src/game/character.ts:4375 handFrameR ->, src/game/character.ts:4375)
#[derive(Clone, Copy, Debug)]
pub struct CharacterHandFrameR {
    pub fingers: V3,
    pub back: V3,
    pub thumb: V3,
}
impl CharacterHandFrameR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterHandFrameR {
            fingers: [0.0; 3],
            back: [0.0; 3],
            thumb: [0.0; 3],
        }
    }
}
impl Default for CharacterHandFrameR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:4381 gripFrameR ->, src/game/character.ts:4381)
#[derive(Clone, Copy, Debug)]
pub struct CharacterGripFrameR {
    pub bore: V3,
    pub top: V3,
}
impl CharacterGripFrameR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterGripFrameR {
            bore: [0.0; 3],
            top: [0.0; 3],
        }
    }
}
impl Default for CharacterGripFrameR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:4393 handFrameL ->, src/game/character.ts:4393)
#[derive(Clone, Copy, Debug)]
pub struct CharacterHandFrameL {
    pub fingers: V3,
    pub back: V3,
    pub thumb: V3,
}
impl CharacterHandFrameL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterHandFrameL {
            fingers: [0.0; 3],
            back: [0.0; 3],
            thumb: [0.0; 3],
        }
    }
}
impl Default for CharacterHandFrameL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:4400 chestFrame ->, src/game/character.ts:4400)
#[derive(Clone, Copy, Debug)]
pub struct CharacterChestFrame {
    pub o: V3,
    pub left: V3,
    pub up: V3,
    pub fwd: V3,
}
impl CharacterChestFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterChestFrame {
            o: [0.0; 3],
            left: [0.0; 3],
            up: [0.0; 3],
            fwd: [0.0; 3],
        }
    }
}
impl Default for CharacterChestFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum CharacterInHandWordCur {
    Pistol,
    Torch(crate::inhand::InHandWordTorch),
    Long(crate::inhand::InHandWordLong),
}

/// (TypeScript: inline at src/game/character.ts:4788 breathFrame(k), src/game/character.ts:4788)
#[derive(Clone, Copy, Debug)]
pub struct CharacterBreathFrameK {
    pub breath_hz: f64,
    pub breath_rise: f64,
    pub breath_in: f64,
}
impl CharacterBreathFrameK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterBreathFrameK {
            breath_hz: 0.0,
            breath_rise: 0.0,
            breath_in: 0.0,
        }
    }
}
impl Default for CharacterBreathFrameK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/character.ts:4822 capsules ->, src/game/character.ts:4822)
#[derive(Clone, Debug)]
pub struct CharacterCapsules {
    pub caps: Vec<Vec<f64>>,
    pub bound: [f64; 4],
}
impl CharacterCapsules {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharacterCapsules {
            caps: Vec::new(),
            bound: [0.0; 4],
        }
    }
}
impl Default for CharacterCapsules { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct CharacterMod {
    /// (src/game/character.ts:342 HOLD_SHOW)
    pub hold_show: crate::character::CharacterModHOLDSHOW,
    /// (src/game/character.ts:478 AT0)
    pub at0: JsMap<ClipId, crate::character::CharacterModAT0Value>,
    /// (src/game/character.ts:719 LONG_STEADY)
    pub long_steady: crate::character::CharacterModLONGSTEADY,
}
impl CharacterMod {
    pub fn blank() -> Self {
        CharacterMod {
            hold_show: crate::character::CharacterModHOLDSHOW { kill: false, beats: false },
            at0: JsMap::new(),
            long_steady: crate::character::CharacterModLONGSTEADY { tau: 0.5, run_yaw: 0.85 },
        }
    }
}

/// (src/game/character.ts:28 LONG_STANCES: a table nothing writes)
pub fn long_stances() -> Vec<String> { alloc::vec![String::from("patrolLong"), String::from("lowLong"), String::from("shoulder"), String::from("portLong")] }

/// (src/game/character.ts:81 WORK: a table nothing writes)
pub fn work() -> crate::character::WORK { crate::character::WORK { key_lead: 0.2, tip_len: 0.215, hover: 0.022, key_off: 0.03, lever: 0.035 } }

/// (src/game/character.ts:87 SWAP: a table nothing writes)
pub fn swap() -> crate::character::SWAP { crate::character::SWAP { draw: 0.6, stow: 0.8, away: 0.3, take: 0.5, lag: 0.2 } }

/// (src/game/character.ts:88 NO_KEYS: a table nothing writes)
pub fn no_keys() -> Vec<[f64; 2]> { alloc::vec![[0.0, 0.0]] }

/// (src/game/character.ts:328 WHIP_THRU)
pub const WHIP_THRU: f64 = 0.14;

/// (src/game/character.ts:331 SWAP_RATE)
pub const SWAP_RATE: f64 = 14.0;

/// (src/game/character.ts:336 HOLD_POLISH_UP: a table nothing writes)
pub fn hold_polish_up() -> crate::character::HOLDPOLISHUP { crate::character::HOLDPOLISHUP { way_shape: true, catch_shape: true, hit_shape: true, lay_ahead: 0.9, dip: 0.06, back: 0.035, m: 0.014, twist: 2.4, side: 7.0, tip: 8.0, arch: -5.0 } }

/// (src/game/character.ts:337 HOLD_POLISH_DOWN: a table nothing writes)
pub fn hold_polish_down() -> crate::character::HOLDPOLISHDOWN { crate::character::HOLDPOLISHDOWN { way_shape: false, catch_shape: false, hit_shape: false, lay_ahead: 0.0, dip: 0.0, back: 0.0, m: 0.0, twist: 0.0, side: 0.0, tip: 0.0, arch: -11.0 } }

/// (src/game/character.ts:354 defaultAnimParams: a table nothing writes)
pub fn default_anim_params() -> crate::character::AnimParams { crate::character::AnimParams { walk_speed: 1.06, jog_speed: 3.3, crouch_speed: 0.72, sprint_speed: 4.7 } }

/// (src/game/character.ts:357 BLEND: a table nothing writes)
pub fn blend() -> crate::character::BLEND { crate::character::BLEND { walk_in0: 0.12, walk_in1: 0.55, walk_top: 1.6, jog_at: 2.7 } }

/// (src/game/character.ts:359 BOB: a table nothing writes)
pub fn bob() -> crate::character::BOB { crate::character::BOB { keep: 0.55 } }

/// (src/game/character.ts:360 BOB_KEEP)
pub const BOB_KEEP: f64 = 0.55;

/// (src/game/character.ts:363 RUN_WARP: a table nothing writes)
pub fn run_warp() -> crate::character::RUNWARP { crate::character::RUNWARP { stride: 0.62 } }

/// (src/game/character.ts:371 SPRINT_WARP: a table nothing writes)
pub fn sprint_warp() -> crate::character::SPRINTWARP { crate::character::SPRINTWARP { stride: 0.57 } }

/// (src/game/character.ts:385 RUN_LINGER: a table nothing writes)
pub fn run_linger() -> crate::character::RUNLINGER { crate::character::RUNLINGER { depth: 0.42, on: [-0.085, -0.04], off: [0.065, 0.105], bleed: 0.09 } }

/// (src/game/character.ts:392 GAZE_EL: a table nothing writes)
pub fn gaze_el() -> crate::character::GAZEEL { crate::character::GAZEEL { stand: 4.0, crouch: 9.0, sprint: 12.0 } }

/// (src/game/character.ts:392 GAZE: a table nothing writes)
pub fn gaze() -> crate::character::GAZE { crate::character::GAZE { keep: 0.8, yaw_damp: 0.7 } }

/// (src/game/character.ts:406 LIFE: a table nothing writes)
pub fn life() -> crate::character::LIFE { crate::character::LIFE { on: 1.0, breath_rate: 0.25, breath_rate_run: 0.62, breath_amp: 1.0, breath_amp_run: 2.6, mass_gain: 0.011, mass_back: 0.035, mass_fwd: 0.03, mass_dip: 0.004, mass_dip_max: 0.025, tip_gain: 0.9, tip_back: 5.0, tip_fwd: 6.0, rest_after: 1.6, shift_min: 4.0, shift_span: 5.0, shift_travel: 0.028, shift_in: 3.2, shift_out: 9.0, hip_drop: 2.2, spine_counter: 1.4, neck_counter: 0.6 } }

/// (src/game/character.ts:417 TOE_FLOOR_Y)
pub const TOE_FLOOR_Y: f64 = 0.0147;

/// (src/game/character.ts:422 SOFT_KNEE: a table nothing writes)
pub fn soft_knee() -> crate::character::SOFTKNEE { crate::character::SOFTKNEE { on: 1.0, from: 0.95, to: 0.985 } }

/// (src/game/character.ts:427 WALK_PHASE)
pub const WALK_PHASE: f64 = 0.22;

/// (src/game/character.ts:428 FEET: a table nothing writes)
pub fn feet() -> crate::character::FEET { crate::character::FEET { on: 1.0, swing: 0.22, lift: 0.045, dist: 0.11, turn_deg: 26.0, pivot: 2.2, lag_cap_deg: 55.0 } }

/// (src/game/character.ts:430 N: a table nothing writes)
pub fn n() -> crate::character::N { crate::character::N { hips: String::from("DEF-hips"), spine1: String::from("DEF-spine.001"), spine2: String::from("DEF-spine.002"), spine3: String::from("DEF-spine.003"), neck: String::from("DEF-neck"), head: String::from("DEF-head"), upper_arm_l: String::from("DEF-upper_arm.L"), forearm_l: String::from("DEF-forearm.L"), hand_l: String::from("DEF-hand.L"), upper_arm_r: String::from("DEF-upper_arm.R"), forearm_r: String::from("DEF-forearm.R"), hand_r: String::from("DEF-hand.R"), thigh_l: String::from("DEF-thigh.L"), shin_l: String::from("DEF-shin.L"), foot_l: String::from("DEF-foot.L"), toe_l: String::from("DEF-toe.L"), thigh_r: String::from("DEF-thigh.R"), shin_r: String::from("DEF-shin.R"), foot_r: String::from("DEF-foot.R"), toe_r: String::from("DEF-toe.R"), shoulder_l: String::from("DEF-shoulder.L"), shoulder_r: String::from("DEF-shoulder.R"), middle1_r: String::from("DEF-f_middle.01.R"), middle1_l: String::from("DEF-f_middle.01.L") } }

/// (src/game/character.ts:436 DEGR)
pub const DEGR: f64 = 0.017453292519943295;

/// (src/game/character.ts:439 AX: a table nothing writes)
pub fn ax() -> V3 { [1.0, 0.0, 0.0] }

/// (src/game/character.ts:439 AY: a table nothing writes)
pub fn ay() -> V3 { [0.0, 1.0, 0.0] }

/// (src/game/character.ts:439 AZ: a table nothing writes)
pub fn az() -> V3 { [0.0, 0.0, 1.0] }

/// (src/game/character.ts:449 STEALTH: a table nothing writes)
pub fn stealth() -> crate::character::STEALTH { crate::character::STEALTH { in_secs: 0.4, out_secs: 0.27, round: 10.0, idle: crate::character::STEALTHIdle { out: 0.05, below: 0.1, fwd: 0.11, lead: 0.05, floor: 0.2, pole: [0.8, -0.05, -0.6], fing: [-0.12, -0.9, 0.4], back: [0.45, 0.25, 0.85], curl: 0.2, thumb: 0.15, spread: 0.3 }, walk: crate::character::STEALTHWalk { stride: 0.6, rate: 12.0, fwd: [0.0, -0.02, 0.18], back: [0.05, -0.3, -0.26], flare_pole: 0.5, tuck: 0.55 }, rise: crate::character::STEALTHRise { down: 5.0 }, creep: crate::character::STEALTHCreep { at: [-0.04, -0.2, 0.27], floor: 0.22, rate: 9.0, swing: 0.25 }, cable: crate::character::STEALTHCable { sink: 0.05, round: 15.0, gap: [-0.07, 0.11], short: 0.1, knee: [0.07, -0.05, 0.1], out: 0.45, head_out: 0.18, undo: 28.0, ahead: 0.3, nod_max: 82.0 } } }

/// (src/game/character.ts:508 SWIV_REF_R: a table nothing writes)
pub fn swiv_ref_r() -> V3 { [0.92, 0.276, -0.276] }

/// (src/game/character.ts:508 SWIV_REF_L: a table nothing writes)
pub fn swiv_ref_l() -> V3 { [-0.92, 0.276, -0.276] }

/// (src/game/character.ts:531 WALL_GAIT_REST)
pub const WALL_GAIT_REST: f64 = 0.55;

/// (src/game/character.ts:534 LEAN_POSE: a table nothing writes)
pub fn lean_pose() -> crate::character::LEANPOSE { crate::character::LEANPOSE { deg: 28.0, level: 0.5, rate: 9.0 } }

/// (src/game/character.ts:535 WALL_POSE: a table nothing writes)
pub fn wall_pose() -> crate::character::WALLPOSE { crate::character::WALLPOSE { back: [0.135, 0.125, 0.21], lean: [4.5, 7.0], drop: [0.03, 0.02], chest_turn: 8.0, gaze: 58.0, head_pitch: 3.0, feet: crate::character::WALLPOSEFeet { lead: [0.16, 0.21, 25.0], trail: [-0.14, 0.16, 8.0], lead_crouch: [0.19, 0.31, 30.0], trail_crouch: [-0.15, 0.27, 12.0], trail_heel: 0.5 }, slide: crate::character::WALLPOSESlide { line: 0.36, narrow: 0.6 }, hands: crate::character::WALLPOSEHands { along: 0.25, along_crouch: 0.29, up: 0.02, up_crouch: 0.12, off: 0.06, out: 0.35, peek_in: 0.1 }, elbow: crate::character::WALLPOSEElbow { out: 0.9, room: 0.35, down: 0.3 }, peek: crate::character::WALLPOSEPeek { lean: 19.0, shift: 0.09, turn: 26.0, gaze: 70.0, tilt: 7.0 }, mend: crate::character::WALLPOSEMend { along: [0.24, 0.235], up: [0.0, 0.0], elbow: crate::character::WALLPOSEMendElbow { out: 0.5, room: -0.3, down: 0.2 }, splay: 0.08, sway: 0.025, fan: -0.3, curl: alloc::vec![0.1, 0.12, 0.16, 0.22], thumb: 0.1, thumb_in: 40.0 }, gait: crate::character::WALLPOSEGait { share: [1.0, 1.0], step: [0.36, 0.34], duty: 0.6, ahead: [0.02, 0.05], apart: [0.21, 0.22], near: [0.17, 0.2], far: [0.3, 0.36], lift: [0.07, 0.06], heel: [22.0, 24.0], toe: [40.0, 30.0], knee_room: [0.2, 0.45], hips: [0.875, 0.5], bob: [0.02, 0.012] } } }

/// (src/game/character.ts:587 STANCES: a table nothing writes)
pub fn stances() -> JsMap<String, crate::character::StanceShape> { { let mut t = JsMap::new(); t.set(String::from("highReady"), crate::character::StanceShape { fwd: 0.25, up: 0.15, side: -0.05, pitch: 8.0, lean: 7.0, head_pitch: 2.0, tuck: 0.85, blade: 0.0, head_yaw: 0.0, squat: 0.015 }); t.set(String::from("lowReady"), crate::character::StanceShape { fwd: 0.3, up: -0.215, side: -0.07, pitch: -42.0, lean: 3.0, head_pitch: 1.0, tuck: 0.3, blade: 0.0, head_yaw: 0.0, squat: 0.0 }); t.set(String::from("stack"), crate::character::StanceShape { fwd: 0.19, up: 0.13, side: -0.03, pitch: 22.0, lean: 9.0, head_pitch: 4.0, tuck: 1.0, blade: 22.0, head_yaw: 14.0, squat: 0.07 }); t.set(String::from("wall"), crate::character::StanceShape { fwd: 0.19, up: 0.13, side: -0.03, pitch: 22.0, lean: 4.5, head_pitch: 3.0, tuck: 1.0, blade: 0.0, head_yaw: 0.0, squat: 0.0 }); t } }

/// (src/game/character.ts:597 WALL_ROOT_OFF)
pub const WALL_ROOT_OFF: f64 = 0.45;

/// (src/game/character.ts:623 LONG: a table nothing writes)
pub fn long() -> JsMap<String, crate::character::LongShape> { { let mut t = JsMap::new(); t.set(String::from("patrolLong"), crate::character::LongShape { side: -0.245, up: -0.16, fwd: 0.13, anchor: 0.0, pitch: -42.0, aim_k: 0.0, yaw: 20.0, blade: -8.0, lean: 0.0, head_yaw: 0.0, head_pitch: 0.0, head_roll: 0.0, elbow_r: [-0.45, -1.0, -0.5], elbow_l: [0.35, -1.0, -0.3], pole_w: 0.65, sway: 1.0, steady: 0.5 }); t.set(String::from("lowLong"), crate::character::LongShape { side: -0.165, up: -0.085, fwd: 0.2, anchor: 0.05, pitch: -40.0, aim_k: 0.05, yaw: 24.0, blade: -10.0, lean: 2.0, head_yaw: 0.0, head_pitch: 2.0, head_roll: 0.0, elbow_r: [-0.55, -1.0, -0.35], elbow_l: [0.4, -1.0, -0.1], pole_w: 0.75, sway: 0.25, steady: 1.0 }); t.set(String::from("shoulder"), crate::character::LongShape { side: -0.15, up: -0.015, fwd: 0.255, anchor: 0.5, pitch: -2.0, aim_k: 1.0, yaw: 0.0, blade: -36.0, lean: 4.0, head_yaw: -10.0, head_pitch: 6.0, head_roll: 7.0, elbow_r: [-0.8, -1.0, -0.2], elbow_l: [0.6, -1.0, 0.35], pole_w: 0.6, sway: 0.1, steady: 1.0 }); t.set(String::from("portLong"), crate::character::LongShape { side: -0.115, up: -0.15, fwd: 0.19, anchor: 0.0, pitch: 50.0, aim_k: 0.0, yaw: 35.0, blade: 0.0, lean: 0.0, head_yaw: 0.0, head_pitch: 0.0, head_roll: 0.0, elbow_r: [-0.7, -0.8, -0.3], elbow_l: [0.9, -0.6, 0.1], pole_w: 0.75, sway: 0.5, steady: 0.7 }); t } }

/// (src/game/character.ts:642 LONG_CROUCH: a table nothing writes)
pub fn long_crouch() -> JsMap<String, crate::character::LongShape> { crate::character::long() }

/// (src/game/character.ts:668 LONG_RELOAD: a table nothing writes)
pub fn long_reload() -> crate::character::LONGRELOAD { crate::character::LONGRELOAD { box_: crate::character::LONGRELOADBox { at: [-0.15, -0.075, 0.19], pitch: 20.0, aim_k: 0.0, yaw: 12.0 }, back: crate::character::LONGRELOADBack { at: [-0.16, -0.06, 0.21], pitch: 12.0, aim_k: 0.0, yaw: 4.0 }, head: crate::character::LONGRELOADHead { yaw: -10.0, pitch: 22.0, roll: 0.0 }, in_: 0.35, out: 0.35, end: 0.06, head_in: 0.3, head_out: 0.3, rate: 8.0 } }

/// (src/game/character.ts:674 LC: a table nothing writes)
pub fn lc() -> crate::character::LC { crate::character::LC { side: 0.0, up: 1.0, fwd: 2.0, pitch: 3.0, aim_k: 4.0, yaw: 5.0, blade: 6.0, lean: 7.0, head_yaw: 8.0, head_pitch: 9.0, head_roll: 10.0, elbow_r: 11.0, elbow_l: 14.0, pole_w: 17.0, sway: 18.0, anchor: 19.0, steady: 20.0, count: 21.0 } }

/// (src/game/character.ts:681 WALL_TUCK: a table nothing writes)
pub fn wall_tuck() -> crate::character::WALLTUCK { crate::character::WALLTUCK { long: crate::character::WALLTUCKLong { side: -0.165, up: -0.245, fwd: 0.185, pitch: -76.0, yaw: 15.0, aim_k: 0.0, elbow_r: [-0.75, -1.0, -0.45], elbow_l: [0.6, -1.0, -0.25] }, pistol: crate::character::WALLTUCKPistol { fwd: 0.2, up: -0.19, pitch: -70.0, tuck: 0.8 }, over: 0.12 } }

/// (src/game/character.ts:724 LONG_KICK_BACK)
pub const LONG_KICK_BACK: f64 = 0.03;

/// (src/game/character.ts:785 FOLD_WITH: a table nothing writes)
pub fn fold_with() -> crate::character::FOLDWITH { crate::character::FOLDWITH {} }

/// (src/game/character.ts:802 HAND: a table nothing writes)
pub fn hand() -> crate::character::HAND { crate::character::HAND { can: crate::character::HANDCan { curl: alloc::vec![0.56, 0.58, 0.62, 0.68], thumb: 0.3 }, curl: alloc::vec![0.3, 0.35, 0.41, 0.48], thumb: 0.38, thumb_in: 34.0, thumb_bend: 15.0, thumb_close: 10.0, thumb_own: 1.0, spread: -0.15, vary: 0.05, side_vary: 0.035, finger_vary: 0.025, far: 45.0, trig: alloc::vec![6.0, 10.0, 6.0], trig_spread: 6.0, trig_trained: 0.6, trig_hold: 0.7 } }

/// (src/game/character.ts:822 WOUND_POSE: a table nothing writes)
pub fn wound_pose() -> JsMap<String, crate::character::WOUNDPOSEValue> { { let mut t = JsMap::new(); t.set(String::from("knee"), crate::character::WOUNDPOSEValue { clip: String::from("Fixing_Kneeling"), rate: 1.15, hold: [0.86, 1.0], breath: 1.6, in_: 0.14, head: 14.0 }); t.set(String::from("back"), crate::character::WOUNDPOSEValue { clip: String::from("Death01"), rate: 1.0, hold: [1.42, 1.54], breath: 1.3, in_: 0.12, head: 44.0 }); t } }

/// (src/game/character.ts:827 WOUND_AT: a table nothing writes)
pub fn wound_at() -> V3 { [-0.05, 0.03, 0.15] }

/// (src/game/character.ts:833 MV2: a table nothing writes)
pub fn mv2() -> crate::character::MV2 { crate::character::MV2 { reload: crate::character::MV2Reload { box_: crate::character::MV2ReloadBox { mag: [0.085, -0.17, 0.125], strip: [0.09, -0.26, 0.15], handle: [0.05, -0.08, 0.07], pull: [0.05, -0.08, -0.03], hf: [-0.7, 0.6, 0.1], hb: [0.5, -0.5, 0.6] }, back: crate::character::MV2ReloadBack { mag: [0.085, -0.15, -0.115], strip: [0.09, -0.23, -0.17], handle: [0.1, 0.01, 0.17], pull: [0.1, 0.01, 0.08], hf: [-0.9, 0.1, 0.3], hb: [0.0, 1.0, 0.0] }, pouch: [[0.15, 0.04, 0.215], [0.1, -0.03, 0.195]], pole: [0.7, -0.6, -0.25], pole_w: 0.7, fumble: 0.055, tap: 0.11, flare: crate::character::MV2ReloadFlare { off: [0.12, 0.12, -0.03], tap: [0.13, 0.04, 0.0], pole: [1.0, 0.35, -0.2] }, fling: 0.06, rate: 14.0 }, bound: crate::character::MV2Bound { rate: 6.0, back: crate::character::MV2BoundBack { up: 0.07, out: 0.045, behind: 0.15, curl: alloc::vec![0.64, 0.7, 0.76, 0.82], thumb: 0.7 }, head: crate::character::MV2BoundHead { up: 0.2, out: 0.095, behind: 0.065, elbow: -0.2, curl: alloc::vec![0.28, 0.32, 0.36, 0.4], knuckle: alloc::vec![16.0, 5.0], thumb: 0.3 } }, clutch: crate::character::MV2Clutch { rate: 7.0, at: [-0.14, 0.12, 0.12], pole: [1.0, -0.05, -0.35], fold: crate::character::MV2ClutchFold { rate: 6.0, pitch: 24.0, pitch_too: 34.0, head: 20.0, drop: 0.05, sway: 3.0, hz: 1.0 } }, hurt: crate::character::MV2Hurt { rate: 4.0, pitch: 11.0, lean: 8.0, head: 9.0, head_brace: 0.5, drop: 0.025, sway: 1.5, hz: 0.45, brace: 0.7, step_pitch: 5.0, step_roll: 10.0, step_yaw: 5.0, step_head: 4.0, step_drop: 0.04, step_side: 0.06, arm: 1.0 }, shaken: crate::character::MV2Shaken { rate_in: 12.0, rate_out: 3.0, pitch: 11.0, head: 18.0, drop: 0.1, brace: 0.75 }, dip: crate::character::MV2Dip { pitch: -36.0, down: 0.035, back: 0.02, rate_in: 13.0, rate_out: 9.0, lane: alloc::vec![0.25, 1.9, 0.34] } } }

/// (src/game/character.ts:881 TURN_FEEL: a table nothing writes)
pub fn turn_feel_table() -> crate::character::TURNFEEL { crate::character::TURNFEEL { light: crate::character::TURNFEELLight { aim: 11.0, aim_acc: 150.0, body: 7.0, body_acc: 60.0 }, medium: crate::character::TURNFEELMedium { aim: 8.75, aim_acc: 75.0, body: 6.0, body_acc: 40.0 }, heavy: crate::character::TURNFEELHeavy { aim: 7.0, aim_acc: 50.0, body: 5.0, body_acc: 30.0 } } }

/// (src/game/character.ts:912 MV_NONE: a table nothing writes)
pub fn mv_none() -> crate::character::MvGait { crate::character::MvGait { live: false, g: 0.0, u: 0.0, st: 0.0 } }

/// (src/game/character.ts:958 TOSS_RELEASE)
pub const TOSS_RELEASE: f64 = 0.14;

/// (src/game/character.ts:958 TOSS_AFTER)
pub const TOSS_AFTER: f64 = 0.66;

/// (src/game/character.ts:960 TOSS_QUICK)
pub const TOSS_QUICK: f64 = 0.08;

/// (src/game/character.ts:962 MV_FLINCH_ENV: a table nothing writes)
pub fn mv_flinch_env() -> Vec<[f64; 2]> { alloc::vec![[0.0, 0.0], [0.14, 1.0], [0.34, 0.9], [0.7, 0.22], [1.0, 0.0]] }

/// (src/game/character.ts:963 MV_HIT_ENV: a table nothing writes)
pub fn mv_hit_env() -> Vec<[f64; 2]> { alloc::vec![[0.0, 0.0], [0.07, 1.0], [0.16, 0.8], [0.32, 0.28], [0.45, -0.07], [0.6, 0.0]] }

/// (src/game/character.ts:969 MV_STARTLE: a table nothing writes)
pub fn mv_startle() -> crate::character::MVSTARTLE { crate::character::MVSTARTLE { amp: [1.0, 0.5], dur: [0.65, 0.45], rear: 10.0, turn: 8.0, lean: 3.0, head: 9.0, head_turn: 32.0, back: 0.075, drop: 0.035, gun_amp: 0.7, gun_rear: 0.5, gun_back: 1.0, behind_turn: 1.8, behind_head: 0.8, behind_pitch: 0.5, shrug: 11.0, wide: [30.0, 19.0], raise: [56.0, 40.0], env: alloc::vec![[0.0, 0.0], [0.12, 1.0], [0.4, 0.86], [0.72, 0.18], [1.0, 0.0]] } }

/// (src/game/character.ts:974 LONG_PIVOT: a table nothing writes)
pub fn long_pivot() -> V3 { [0.0, 0.13, -0.06] }

/// (src/game/character.ts:976 LONG_PULL_MAX)
pub const LONG_PULL_MAX: f64 = 0.06;

/// (src/game/character.ts:979 LONG_ANCHOR_SIDE_MAX)
pub const LONG_ANCHOR_SIDE_MAX: f64 = 0.215;

/// (src/game/character.ts:981 SNEAK: a table nothing writes)
pub fn sneak() -> crate::character::SNEAK { crate::character::SNEAK { stride: 0.282, pace_stride: 0.1, linger: 0.16, surge: 0.7, drop: 0.04, pace_drop: 0.03, soft_drop: 0.01, lean: 5.0, pace_lean: 1.5, gun_in: [0.0, -0.07, -0.06], gun_in_stand: [0.0, -0.03, -0.03], hand_in: 0.1 } }

/// (src/game/character.ts:1009 KICK_DUR)
pub const KICK_DUR: f64 = 1.0;

/// (src/game/character.ts:1009 KICK_IMPACT)
pub const KICK_IMPACT: f64 = 0.38;

/// (src/game/character.ts:1011 KICK_PLANT)
pub const KICK_PLANT: f64 = 0.72;

/// (src/game/character.ts:1012 SIGNAL_DUR: a table nothing writes)
pub fn signal_dur() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("hold"), 0.9); t.set(String::from("go"), 0.95); t.set(String::from("rally"), 1.3); t.set(String::from("squeeze"), 0.9); t.set(String::from("aside"), 1.15); t } }

/// (src/game/character.ts:1014 SIGNAL_NO: a table nothing writes)
pub fn signal_no() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("hold"), 0.0); t.set(String::from("go"), 1.0); t.set(String::from("rally"), 2.0); t.set(String::from("squeeze"), 3.0); t.set(String::from("aside"), 4.0); t } }

/// (src/game/character.ts:1016 TELL_DUR: a table nothing writes)
pub fn tell_dur() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("watch"), 1.05); t.set(String::from("neck"), 1.35); t } }

/// (src/game/character.ts:1039 READY: a table nothing writes)
pub fn ready() -> JsMap<String, crate::character::ReadyShape> { { let mut t = JsMap::new(); t.set(String::from("grab"), crate::character::ReadyShape { lean: 5.0, blade: 0.0, clav: 8.0, l: crate::character::ReadyShapeL { pole: [0.8, -0.6, -0.1], fing: [-0.1, 0.15, 1.0], back: [0.9, 0.45, 0.0], curl: 0.16, thumb: 0.1, spread: 0.4, reach: Some(crate::character::ReadyShapeLReach { frac: 0.65, r: [0.36, 0.46], dy: [-0.06, 0.14], bias: 0.0, h0: 0.75, y0: 0.05 }), at: None }, knife: true }); t.set(String::from("grabGun"), crate::character::ReadyShape { lean: 0.0, blade: 0.0, clav: 6.0, l: crate::character::ReadyShapeL { pole: [0.8, -0.6, -0.1], fing: [-0.1, 0.1, 1.0], back: [0.9, 0.45, 0.0], curl: 0.16, thumb: 0.1, spread: 0.4, reach: Some(crate::character::ReadyShapeLReach { frac: 0.65, r: [0.36, 0.46], dy: [-0.16, 0.04], bias: -0.07, h0: 0.75, y0: 0.05 }), at: None }, knife: false }); t.set(String::from("strike"), crate::character::ReadyShape { lean: 3.0, blade: 13.0, clav: 0.0, l: crate::character::ReadyShapeL { pole: [0.6, -1.0, -0.2], fing: [-0.15, 0.55, 1.0], back: [0.8, 0.5, 0.0], curl: 0.4, thumb: 0.35, spread: 0.15, reach: None, at: Some([-0.1, -0.12, 0.3]) }, knife: true }); t.set(String::from("seat"), crate::character::ReadyShape { lean: 9.0, blade: 0.0, clav: 6.0, l: crate::character::ReadyShapeL { pole: [0.7, -0.5, -0.3], fing: [0.0, -0.35, 1.0], back: [0.5, 0.8, 0.0], curl: 0.22, thumb: 0.15, spread: 0.35, reach: Some(crate::character::ReadyShapeLReach { frac: 0.7, r: [0.3, 0.42], dy: [-0.36, -0.12], bias: -0.04, h0: 0.6, y0: -0.28 }), at: None }, knife: true }); t } }

/// (src/game/character.ts:1054 REACH: a table nothing writes)
pub fn reach() -> crate::character::REACH { crate::character::REACH { in_: 0.3, out: 0.32, pole: [-1.0, 0.0, -0.1], pole_up: alloc::vec![0.35, 0.5], curl: alloc::vec![0.62, 0.68, 0.72, 0.76], thumb: 0.6 } }

/// (src/game/character.ts:1064 KNIFE_BLOW: a table nothing writes)
pub fn knife_blow() -> crate::character::KNIFEBLOW { crate::character::KNIFEBLOW { snap: 0.16, t_clear: 0.06, clear: [0.14, 0.05], az: 0.7, reach: 0.96, fade: 0.12, cross: crate::character::KNIFEBLOWCross { end: 0.6, hit: 0.27, keys: alloc::vec![crate::character::BlowKey { t: 0.16, at: [0.27, 0.0, 0.3], blade: [0.1, 0.9, 0.5], fing: [0.7, 0.1, 0.6], pole: [-0.5, -0.8, 0.1] }, crate::character::BlowKey { t: 0.27, at: [-0.05, 0.03, 0.5], blade: [-0.1, 0.8, 0.6], fing: [0.2, 0.0, 1.0], pole: [-0.7, -0.7, 0.0] }, crate::character::BlowKey { t: 0.36, at: [-0.22, -0.06, 0.36], blade: [-0.4, 0.8, 0.45], fing: [-0.2, -0.1, 1.0], pole: [-0.8, -0.6, -0.1] }] }, jab: crate::character::KNIFEBLOWCross { end: 0.52, hit: 0.24, keys: alloc::vec![crate::character::BlowKey { t: 0.15, at: [-0.02, -0.32, 0.08], blade: [0.0, 0.5, 1.0], fing: [0.05, -0.5, 0.8], pole: [-0.5, -0.3, -0.8] }, crate::character::BlowKey { t: 0.24, at: [0.12, -0.2, 0.56], blade: [0.0, 1.0, 0.8], fing: [0.2, 0.1, 1.0], pole: [-0.6, -0.8, -0.1] }, crate::character::BlowKey { t: 0.33, at: [0.0, -0.28, 0.2], blade: [0.0, 0.6, 1.0], fing: [0.05, -0.4, 0.9], pole: [-0.5, -0.5, -0.7] }] } } }

/// (src/game/character.ts:1077 KNIFE_SLASH: a table nothing writes)
pub fn knife_slash() -> crate::character::KNIFESLASH { crate::character::KNIFESLASH { end: 0.52, hit: 0.24, keys: alloc::vec![crate::character::BlowKey { t: 0.14, at: [0.3, -0.38, 0.12], blade: [-0.3, 0.6, 0.7], fing: [0.6, -0.3, 0.6], pole: [-0.3, -0.6, -0.6] }, crate::character::BlowKey { t: 0.24, at: [0.1, -0.08, 0.62], blade: [-0.6, 0.5, 0.6], fing: [0.1, 0.0, 1.0], pole: [-0.7, -0.6, 0.0] }, crate::character::BlowKey { t: 0.34, at: [-0.46, 0.3, 0.3], blade: [-0.8, 0.5, 0.2], fing: [-0.7, 0.3, 0.6], pole: [-0.8, -0.5, -0.2] }, crate::character::BlowKey { t: 0.42, at: [-0.46, 0.28, 0.28], blade: [-0.8, 0.5, 0.2], fing: [-0.7, 0.3, 0.6], pole: [-0.8, -0.5, -0.2] }] } }

/// (src/game/character.ts:1084 KNIFE_BACK: a table nothing writes)
pub fn knife_back() -> crate::character::KNIFEBACK { crate::character::KNIFEBACK { end: 0.6, hit: 0.27, keys: alloc::vec![crate::character::BlowKey { t: 0.15, at: [-0.02, -0.4, -0.04], blade: [0.0, 0.25, 1.0], fing: [0.0, -1.0, 0.25], pole: [-0.4, 0.0, -1.0] }, crate::character::BlowKey { t: 0.27, at: [0.08, -0.34, 0.33], blade: [0.0, 0.2, 1.0], fing: [0.0, -0.9, 0.35], pole: [-0.4, -0.2, -1.0] }, crate::character::BlowKey { t: 0.44, at: [0.08, -0.36, 0.31], blade: [0.0, 0.2, 1.0], fing: [0.0, -0.9, 0.35], pole: [-0.4, -0.2, -1.0] }] } }

/// (src/game/character.ts:1091 BLOW: a table nothing writes)
pub fn blow() -> crate::character::BLOW { crate::character::BLOW { rise: { let mut t = JsMap::new(); t.set(String::from("chop"), 0.85); t.set(String::from("palm"), 0.6); t.set(String::from("mouth"), 0.9); t.set(String::from("sweep"), 0.8); t.set(String::from("slash"), 0.7); t }, clip: { let mut t = JsMap::new(); t.set(String::from("chop"), 0.25); t.set(String::from("palm"), 0.35); t.set(String::from("mouth"), 1.0); t.set(String::from("sweep"), 1.0); t.set(String::from("slash"), 1.0); t }, reach: 0.98, fist: alloc::vec![0.92, 0.95, 0.95, 0.95], open: alloc::vec![0.08, 0.06, 0.08, 0.12], loose: alloc::vec![0.25, 0.3, 0.3, 0.35], cup: alloc::vec![0.35, 0.35, 0.4, 0.45], legs: crate::character::BLOWLegs { gap: 0.2, lift: 0.05, straight: 0.985, up: 0.4, fwd: 0.6, out: 0.4 }, tail: 1.0, sink: { let mut t = JsMap::new(); t.set(String::from("chop"), 0.8); t.set(String::from("palm"), 0.6); t.set(String::from("mouth"), 1.0); t.set(String::from("sweep"), 0.75); t.set(String::from("slash"), 0.75); t }, walk: alloc::vec![0.4, 1.5] } }

/// (src/game/character.ts:1102 TAKEN: a table nothing writes)
pub fn taken() -> crate::character::TAKEN { crate::character::TAKEN { sag: { let mut t = JsMap::new(); t.set(String::from("arch"), 0.1); t.set(String::from("hunch"), 0.65); t.set(String::from("snap"), 0.15); t }, secs: 1.0 } }

/// (src/game/character.ts:1105 KNIFE_KEYS_MAX)
pub const KNIFE_KEYS_MAX: f64 = 4.0;

/// (src/game/character.ts:1107 BLOW_NO: a table nothing writes)
pub fn blow_no() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("chop"), 0.0); t.set(String::from("palm"), 1.0); t.set(String::from("mouth"), 2.0); t.set(String::from("slash"), 3.0); t.set(String::from("sweep"), 4.0); t } }

/// (src/game/character.ts:1109 HOLD_PH: a table nothing writes)
pub fn hold_ph() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("grab"), 0.0); t.set(String::from("held"), 1.0); t.set(String::from("choke"), 2.0); t.set(String::from("whip"), 3.0); t.set(String::from("kill"), 4.0); t } }

/// (src/game/character.ts:1112 THROW_KEYS_MAX)
pub const THROW_KEYS_MAX: f64 = 16.0;

/// (src/game/character.ts:1131 THROW_NUMS: a table nothing writes)
pub fn throw_nums() -> [String; 11] { [String::from("chestYaw"), String::from("pelvisYaw"), String::from("pitch"), String::from("roll"), String::from("pelX"), String::from("pelY"), String::from("pelZ"), String::from("clavUp"), String::from("clavFwd"), String::from("swivR"), String::from("swivL")] }

/// (src/game/character.ts:1131 THROW_VECS: a table nothing writes)
pub fn throw_vecs() -> [String; 6] { [String::from("handR"), String::from("fingR"), String::from("backR"), String::from("handL"), String::from("fingL"), String::from("backL")] }

/// (src/game/character.ts:1136 WORK_NO: a table nothing writes)
pub fn work_no() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("keys"), 0.0); t.set(String::from("charge"), 1.0); t.set(String::from("lever"), 2.0); t.set(String::from("take"), 3.0); t } }

/// (src/game/character.ts:1138 KIT_NO: a table nothing writes)
pub fn kit_no() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("pistol"), 1.0); t.set(String::from("rifle"), 2.0); t.set(String::from("can"), 3.0); t.set(String::from("knife"), 4.0); t } }

/// (src/game/character.ts:1138 KIT_OF: a table nothing writes)
pub fn kit_of() -> Vec<Option<String>> { alloc::vec![None, Some(String::from("pistol")), Some(String::from("rifle")), Some(String::from("can")), Some(String::from("knife"))] }

/// (src/game/character.ts:1146 CREEP: a table nothing writes)
pub fn creep() -> crate::character::CREEP { crate::character::CREEP { in_: 6.0, out: 5.0, gait_in: 0.2, gait_out: 0.4, drop: alloc::vec![0.09, 0.03], back: alloc::vec![0.03, 0.01], lean: alloc::vec![14.0, 6.0], round: 10.0, look: 0.85, low: crate::character::CREEPLow { rate: 8.0, up: 24.0 }, wide: crate::character::CREEPWide { in_: 5.0, out: 14.0, still: 0.03, pole: [0.75, 0.1, -0.65], fing: [0.3, -0.5, 0.8], back: [0.15, 0.9, 0.35], curl: 0.1, thumb: 0.15, spread: 0.55 }, heft: crate::character::CREEPHeft { stride: 0.75, lift: 0.1, run: 1.6, trunk: [11.0, 0.7], arm: [7.0, 0.55], body: [9.0, 0.75], go: [5.0, 0.5], out: 0.15, up: 0.3, breath: 1.3, breath_hz: 0.32, dt: 0.03333333333333333 }, l: crate::character::CREEPL { at: [-0.05, -0.27, 0.3], at_c: [0.0, -0.17, 0.3], pole: [0.7, -0.6, -0.5], fing: [-0.2, 0.35, 1.0], back: [0.7, 0.7, 0.0], curl: 0.32, thumb: 0.25, spread: 0.2 }, r: crate::character::CREEPR { at: [0.05, -0.33, 0.27], at_c: [0.0, -0.21, 0.28], pole: [-0.7, -0.6, -0.5], fing: [0.2, 0.3, 1.0], back: [-0.7, 0.7, 0.0], curl: 0.38, thumb: 0.3, spread: 0.15 } } }

/// (src/game/character.ts:1167 READY_RATE)
pub const READY_RATE: f64 = 12.0;

/// (src/game/character.ts:1167 READY_REACH)
pub const READY_REACH: f64 = 0.85;

/// (src/game/character.ts:1167 READY_AZ: a table nothing writes)
pub fn ready_az() -> [f64; 2] { [0.5, 0.9] }

/// (src/game/character.ts:1175 READY_BARE: a table nothing writes)
pub fn ready_bare() -> crate::character::READYBARE { crate::character::READYBARE { l: crate::character::READYBAREL { pole: [0.32, -0.93, -0.2], fing: [-0.2, 0.3, 1.0], back: [0.8, 0.55, -0.1], curl: 0.1, thumb: 0.28, spread: 0.05, frac: 0.9, dy: 0.3, r: [0.4, 0.54], short: 0.32 }, r: crate::character::READYBARER { at: [0.1, -0.3, 0.2], pole: [-0.4, -0.9, -0.25], fing: [0.15, 0.2, 1.0], back: [-0.85, 0.5, 0.0], curl: 0.14, thumb: 0.3 }, way: crate::character::READYBAREWay { by: 0.4, low: -0.2, near: 0.32, lift: [0.3, 1.0], right: [0.62, 0.97], rate: 12.0, drop: 14.0 }, out: [0.25, 0.8], out_r: 0.3 } }

/// (src/game/character.ts:1193 READY_BARE_WAS: a table nothing writes)
pub fn ready_bare_was() -> crate::character::READYBAREWAS { crate::character::READYBAREWAS { l: crate::character::READYBAREWASL { pole: [0.5, -0.85, -0.2], fing: [-0.2, 0.12, 1.0], curl: 0.3, spread: 0.12, back: [0.8, 0.55, -0.1], thumb: 0.28, frac: 0.9, dy: 0.3, r: [0.4, 0.54], short: 0.32 }, r: crate::character::READYBAREWASR { at: [0.07, 0.0, 0.4], pole: [-0.5, -0.85, -0.25], fing: [0.15, 0.05, 1.0], curl: 0.34, back: [-0.85, 0.5, 0.0], thumb: 0.3 } } }

/// (src/game/character.ts:1202 READY_UP: a table nothing writes)
pub fn ready_up() -> crate::character::READYUP { crate::character::READYUP { over: [0.25, 0.45], rise: 38.0, look: 0.5, dy: 0.36, r: [0.32, 0.42], fing: 0.3, slow: 0.8, rate: 6.0 } }

/// (src/game/character.ts:1221 SMOKEARM: a table nothing writes)
pub fn smokearm() -> crate::character::SMOKEARM { crate::character::SMOKEARM { at: [0.075, -0.15, 0.125], fingers: [-0.55, 0.8, -0.1], back: [0.35, 0.1, 0.9], pole: [0.7, -0.6, 0.35], shrug: 5.0, curl: [0.3, 0.34, 0.6, 0.66], thumb: 0.3, rate: 1.9, away: 3.0 } }

/// (src/game/character.ts:1277 RD: a table nothing writes)
pub fn rd() -> crate::character::RD { crate::character::RD { l: 0.0, r: 3.0, l_pole: 6.0, r_pole: 9.0, l_fing: 12.0, l_back: 15.0, r_fing: 18.0, r_back: 21.0, lean: 24.0, blade: 25.0, w_r: 26.0, l_curl: 27.0, l_thumb: 28.0, l_spread: 29.0, r_curl: 30.0, r_thumb: 31.0, clav: 32.0, into: 33.0, up: 35.0 } }

/// (src/game/character.ts:1277 RD_N)
pub const RD_N: f64 = 36.0;

/// (src/game/character.ts:1283 THROW_DUR)
pub const THROW_DUR: f64 = 0.75;

/// (src/game/character.ts:1283 THROW_RELEASE)
pub const THROW_RELEASE: f64 = 0.28;

/// (src/game/character.ts:1284 THROW_TOP)
pub const THROW_TOP: f64 = 0.165;

/// (src/game/character.ts:1288 THR: a table nothing writes)
pub fn thr() -> crate::character::THR { crate::character::THR { chest_yaw: alloc::vec![[0.0, 0.0], [0.07, -9.0], [0.165, -26.0], [0.19, -25.0], [0.225, -14.0], [0.26, -3.0], [0.3, 6.0], [0.36, 10.0], [0.48, 7.0], [0.62, 2.0], [0.75, 0.0]], pelvis_yaw: alloc::vec![[0.0, 0.0], [0.07, -3.5], [0.145, -9.0], [0.17, -8.0], [0.21, 1.0], [0.25, 6.5], [0.31, 8.0], [0.44, 6.0], [0.6, 1.5], [0.75, 0.0]], pitch: alloc::vec![[0.0, 0.0], [0.165, -4.0], [0.28, 4.0], [0.38, 9.0], [0.48, 7.0], [0.62, 2.0], [0.75, 0.0]], roll: alloc::vec![[0.0, 0.0], [0.165, 3.0], [0.28, -7.0], [0.38, -8.0], [0.48, -4.0], [0.66, 0.0], [0.75, 0.0]], pel_x: alloc::vec![[0.0, 0.0], [0.165, -0.02], [0.3, 0.025], [0.48, 0.02], [0.7, 0.0], [0.75, 0.0]], pel_y: alloc::vec![[0.0, 0.0], [0.165, -0.015], [0.28, -0.01], [0.4, -0.02], [0.58, -0.008], [0.75, 0.0]], pel_z: alloc::vec![[0.0, 0.0], [0.165, -0.03], [0.28, 0.02], [0.39, 0.045], [0.54, 0.03], [0.75, 0.0]], clav_up: alloc::vec![[0.0, 0.0], [0.165, 9.0], [0.28, 7.0], [0.4, 3.0], [0.62, 0.0], [0.75, 0.0]], clav_fwd: alloc::vec![[0.0, 0.0], [0.165, -4.0], [0.28, 5.0], [0.39, 8.0], [0.54, 3.0], [0.7, 0.0], [0.75, 0.0]], hand_r: alloc::vec![(0.165, alloc::vec![-0.19, 0.21, -0.2]), (0.205, alloc::vec![-0.21, 0.27, -0.1]), (0.245, alloc::vec![-0.16, 0.27, 0.14]), (0.28, alloc::vec![-0.06, 0.16, 0.42]), (0.31, alloc::vec![0.06, -0.08, 0.47]), (0.37, alloc::vec![0.13, -0.27, 0.37]), (0.44, alloc::vec![0.16, -0.35, 0.28]), (0.5, alloc::vec![0.14, -0.38, 0.29])], swiv_r: alloc::vec![[0.0, -62.0], [0.06, -103.0], [0.125, -166.0], [0.165, -176.0], [0.205, -149.0], [0.245, -184.6], [0.28, -184.6], [0.31, -232.9], [0.37, -240.9], [0.44, -234.7], [0.5, -214.4], [0.62, -105.4], [0.75, -62.0]], fing_r: alloc::vec![(0.0, alloc::vec![0.0, -1.0, 0.15]), (0.07, alloc::vec![-0.1, -0.75, -0.65]), (0.115, alloc::vec![0.0, 0.6, -0.8]), (0.165, alloc::vec![0.15, 1.0, -0.35]), (0.23, alloc::vec![0.1, 1.0, -0.5]), (0.28, alloc::vec![0.05, 0.75, 0.65]), (0.32, alloc::vec![0.1, -0.35, 0.95]), (0.38, alloc::vec![0.3, -0.75, 0.6]), (0.48, alloc::vec![0.25, -1.0, 0.25]), (0.62, alloc::vec![0.05, -1.0, 0.0]), (0.75, alloc::vec![0.0, -1.0, 0.15])], back_r: alloc::vec![(0.0, alloc::vec![-1.0, 0.0, 0.0]), (0.07, alloc::vec![-1.0, 0.1, -0.2]), (0.115, alloc::vec![-0.8, -0.1, -0.6]), (0.165, alloc::vec![-0.3, -0.3, -1.0]), (0.23, alloc::vec![-0.4, -0.4, -0.9]), (0.28, alloc::vec![-0.2, 0.5, -0.8]), (0.32, alloc::vec![-0.3, 0.9, 0.3]), (0.38, alloc::vec![-0.6, 0.5, 0.6]), (0.48, alloc::vec![-0.7, 0.1, 0.7]), (0.62, alloc::vec![-1.0, 0.0, 0.2]), (0.75, alloc::vec![-1.0, 0.0, 0.0])], hand_l: alloc::vec![(0.17, alloc::vec![0.1, -0.06, 0.44]), (0.24, alloc::vec![0.1, -0.12, 0.38]), (0.31, alloc::vec![0.06, -0.28, 0.22]), (0.4, alloc::vec![0.05, -0.33, 0.18]), (0.52, alloc::vec![0.07, -0.4, 0.13])], swiv_l: alloc::vec![[0.0, 83.0], [0.09, 136.5], [0.17, 149.2], [0.24, 156.6], [0.31, 142.0], [0.4, 134.9], [0.52, 123.3], [0.63, 106.8], [0.75, 83.0]], fing_l: alloc::vec![(0.0, alloc::vec![0.0, -1.0, 0.15]), (0.09, alloc::vec![0.1, -0.4, 0.9]), (0.17, alloc::vec![0.05, 0.15, 1.0]), (0.25, alloc::vec![0.1, 0.35, 0.9]), (0.33, alloc::vec![0.15, 0.6, 0.8]), (0.52, alloc::vec![0.1, 0.3, 0.9]), (0.63, alloc::vec![0.1, -0.7, 0.6]), (0.75, alloc::vec![0.0, -1.0, 0.15])], back_l: alloc::vec![(0.0, alloc::vec![1.0, 0.0, 0.0]), (0.09, alloc::vec![0.5, 0.8, 0.3]), (0.17, alloc::vec![0.3, 1.0, -0.1]), (0.25, alloc::vec![0.7, 0.7, -0.2]), (0.33, alloc::vec![1.0, 0.1, -0.3]), (0.52, alloc::vec![1.0, 0.2, -0.2]), (0.63, alloc::vec![1.0, 0.2, 0.0]), (0.75, alloc::vec![1.0, 0.0, 0.0])] } }

/// (src/game/character.ts:1341 TOSS_TOP)
pub const TOSS_TOP: f64 = 0.1;

/// (src/game/character.ts:1341 TOSS_LOW)
pub const TOSS_LOW: f64 = 0.165;

/// (src/game/character.ts:1343 TOSS_HANG: a table nothing writes)
pub fn toss_hang() -> V3 { [0.1, -0.47, -0.02] }

/// (src/game/character.ts:1344 THR_LOW: a table nothing writes)
pub fn thr_low() -> crate::character::THRLOW { crate::character::THRLOW { chest_yaw: alloc::vec![[0.0, 0.0], [0.05, -4.0], [0.1, -9.0], [0.17, -5.0], [0.25, 1.0], [0.36, 5.0], [0.55, 2.0], [0.75, 0.0]], pelvis_yaw: alloc::vec![[0.0, 0.0], [0.09, -4.0], [0.16, -1.0], [0.24, 2.0], [0.4, 3.0], [0.6, 0.5], [0.75, 0.0]], pitch: alloc::vec![[0.0, 0.0], [0.11, 9.0], [0.2, 7.0], [0.29, 1.0], [0.4, -2.0], [0.6, 0.0], [0.75, 0.0]], roll: alloc::vec![[0.0, 0.0], [0.1, 3.0], [0.22, 2.0], [0.45, 0.0], [0.75, 0.0]], pel_x: alloc::vec![[0.0, 0.0], [0.1, -0.012], [0.28, 0.012], [0.5, 0.006], [0.75, 0.0]], pel_y: alloc::vec![[0.0, 0.0], [0.12, -0.045], [0.2, -0.035], [0.31, 0.0], [0.42, 0.005], [0.6, 0.0], [0.75, 0.0]], pel_z: alloc::vec![[0.0, 0.0], [0.1, -0.015], [0.28, 0.03], [0.5, 0.015], [0.75, 0.0]], clav_up: alloc::vec![[0.0, 0.0], [0.1, -3.0], [0.3, 5.0], [0.6, 0.0], [0.75, 0.0]], clav_fwd: alloc::vec![[0.0, 0.0], [0.1, -5.0], [0.28, 6.0], [0.5, 2.0], [0.75, 0.0]], hand_r: alloc::vec![(0.1, alloc::vec![-0.13, -0.41, -0.32]), (0.165, alloc::vec![-0.1, -0.53, -0.02]), (0.225, alloc::vec![-0.07, -0.36, 0.37]), (0.28, alloc::vec![-0.05, -0.08, 0.5]), (0.34, alloc::vec![-0.04, -0.06, 0.52]), (0.46, alloc::vec![-0.04, -0.08, 0.49])], swiv_r: alloc::vec![[0.0, -62.0], [0.1, -78.8], [0.165, -82.2], [0.225, -107.2], [0.28, -121.9], [0.34, -122.7], [0.46, -121.7], [0.62, -84.0], [0.75, -62.0]], fing_r: alloc::vec![(0.0, alloc::vec![0.0, -1.0, 0.15]), (0.06, alloc::vec![0.0, -0.95, -0.3]), (0.1, alloc::vec![-0.1, -0.8, -0.6]), (0.165, alloc::vec![0.0, -1.0, 0.0]), (0.225, alloc::vec![0.0, -0.7, 0.7]), (0.28, alloc::vec![0.0, -0.1, 1.0]), (0.34, alloc::vec![0.0, 0.2, 1.0]), (0.46, alloc::vec![0.0, 0.1, 1.0]), (0.62, alloc::vec![0.0, -0.8, 0.55]), (0.75, alloc::vec![0.0, -1.0, 0.15])], back_r: alloc::vec![(0.0, alloc::vec![-1.0, 0.0, 0.0]), (0.06, alloc::vec![-1.0, 0.0, -0.1]), (0.1, alloc::vec![-0.9, 0.1, -0.4]), (0.165, alloc::vec![-0.6, -0.1, -0.8]), (0.225, alloc::vec![-0.35, -0.7, -0.6]), (0.28, alloc::vec![-0.25, -0.95, -0.1]), (0.34, alloc::vec![-0.25, -0.95, 0.15]), (0.46, alloc::vec![-0.4, -0.9, 0.1]), (0.62, alloc::vec![-0.9, -0.3, 0.0]), (0.75, alloc::vec![-1.0, 0.0, 0.0])] } }

/// (src/game/character.ts:1381 BIGMOVE: a table nothing writes)
pub fn bigmove() -> crate::character::BIGMOVE { crate::character::BIGMOVE { hide: 0.28, grab: 0.5, take: alloc::vec![0.47, 0.82], join: alloc::vec![0.65, 0.96], put: 0.58, put_at: 0.45, unjoin: 0.28, show: 0.85, snug: crate::character::BIGMOVESnug { hide: 0.06, take: alloc::vec![0.44, 0.56], put_at: 0.8, show: 0.94 }, arc: [-0.16, 0.12, 0.04], wait: [-0.1, -0.34, 0.2], hammer: crate::character::BIGMOVEHammer { ready: crate::character::BIGMOVEHammerReady { g: [0.06, -0.1, 0.34], d: [-0.42, 0.9, -0.1] }, wound: crate::character::BIGMOVEHammerWound { g: [-0.12, 0.2, -0.05], d: [-0.45, 0.3, -0.84] }, over: crate::character::BIGMOVEHammerOver { g: [0.04, 0.14, 0.24], d: [-0.08, 0.93, 0.36] }, grab_at: 0.1, raise: 0.6, swing: 0.38, swing_min: 0.1, swing_max: 0.2, rebound: 0.25, settle: 0.5, butt: [0.18, 0.05, 0.18], far: [0.6, 1.1], wound_body: crate::character::BIGMOVEHammerWoundBody { yaw: -50.0, pel_yaw: -17.0, pitch: -7.0, back: 0.09, side: -0.03 }, blow_body: crate::character::BIGMOVEHammerBlowBody { yaw: 16.0, pel_yaw: 7.0, pitch: 15.0, pitch_far: 11.0, fwd: 0.07, fwd_far: 0.08, drop: 0.03 }, face: 1.0, drop: 0.035, stance: crate::character::BIGMOVEHammerStance { l: [0.06, 0.07], r: [-0.07, -0.1], lift: 0.025 } }, shotgun: crate::character::BIGMOVEShotgun { pocket: [0.035, -0.05, 0.06], yaw: -22.0, pel_yaw: -10.0, pitch: 6.0, kick: 0.08, climb: 12.0, kick_in: 0.04, kick_out: 0.13, kick_pitch: -6.0, shove: 0.035 }, fist: alloc::vec![0.74, 0.8, 0.84, 0.88], open: alloc::vec![0.26, 0.3, 0.34, 0.38] } }

/// (src/game/character.ts:1426 ANIM_POSES)
pub const ANIM_POSES: f64 = 13.0;

/// (src/game/character.ts:4262 RAG_LIMBS: a table nothing writes)
pub fn rag_limbs() -> Vec<(String, f64, f64, f64)> { alloc::vec![(String::from("thighL"), -1.0, 10.0, 12.0), (String::from("shinL"), 0.0, 12.0, 14.0), (String::from("thighR"), -1.0, 11.0, 13.0), (String::from("shinR"), 2.0, 13.0, 15.0), (String::from("upperArmL"), -1.0, 4.0, 6.0), (String::from("forearmL"), 4.0, 6.0, 8.0), (String::from("upperArmR"), -1.0, 5.0, 7.0), (String::from("forearmR"), 6.0, 7.0, 9.0), (String::from("neck"), -1.0, 2.0, 3.0)] }

/// (src/game/character.ts:4268 RAG_BONE: a table nothing writes)
pub fn rag_bone() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("pelvis"), String::from("hips")); t.set(String::from("spine"), String::from("spine2")); t.set(String::from("neck"), String::from("neck")); t.set(String::from("head"), String::from("head")); t.set(String::from("shoL"), String::from("upperArmL")); t.set(String::from("shoR"), String::from("upperArmR")); t.set(String::from("elbL"), String::from("forearmL")); t.set(String::from("elbR"), String::from("forearmR")); t.set(String::from("handL"), String::from("handL")); t.set(String::from("handR"), String::from("handR")); t.set(String::from("hipL"), String::from("thighL")); t.set(String::from("hipR"), String::from("thighR")); t.set(String::from("kneeL"), String::from("shinL")); t.set(String::from("kneeR"), String::from("shinR")); t.set(String::from("ankL"), String::from("footL")); t.set(String::from("ankR"), String::from("footR")); t } }

/// (src/game/character.ts:4269 RAG_BONES: a table nothing writes)
pub fn rag_bones() -> Vec<String> { Vec::new() }

impl Character {
    /// (character.ts Character's constructor, up to what needs the rig: rest_chest_inv, rest_head_inv, the hand frames, anim and skel are the part's to fill from the rig)
    pub fn new(rig: RigId, who: Who, id: f64, tint: V3, tint2: V3, pos: V3, yaw: f64) -> Self {
        let mut c = Character::blank();
        c.rig = rig; c.who = who; c.id = id; c.tint = tint; c.tint2 = tint2; c.pos = pos; c.body_yaw = yaw; c.aim_yaw = yaw;
        c
    }
}
