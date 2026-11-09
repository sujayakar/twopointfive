//! The types of `character`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// THE NAMES OF A BODY'S JOINTS, in the order a real game's `bones` has them (read off a running game: `for k in b` walks this order)
pub const BONE_ORDER: [Bone; 22] = [Bone::Hips, Bone::Spine2, Bone::Chest, Bone::Neck, Bone::Head, Bone::UpperArmL, Bone::ForearmL, Bone::HandL, Bone::UpperArmR, Bone::ForearmR, Bone::HandR, Bone::ThighL, Bone::ShinL, Bone::FootL, Bone::ToeL, Bone::ThighR, Bone::ShinR, Bone::FootR, Bone::ToeR, Bone::FingerR, Bone::FingerL, Bone::HeadTop];
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum Bone { Hips, Spine2, Chest, Neck, Head, UpperArmL, ForearmL, HandL, UpperArmR, ForearmR, HandR, ThighL, ShinL, FootL, ToeL, ThighR, ShinR, FootR, ToeR, FingerR, FingerL, HeadTop }
impl Bone {
    pub const fn as_str(self) -> &'static str {
        match self { Bone::Hips => "hips", Bone::Spine2 => "spine2", Bone::Chest => "chest", Bone::Neck => "neck", Bone::Head => "head", Bone::UpperArmL => "upperArmL", Bone::ForearmL => "forearmL", Bone::HandL => "handL", Bone::UpperArmR => "upperArmR", Bone::ForearmR => "forearmR", Bone::HandR => "handR", Bone::ThighL => "thighL", Bone::ShinL => "shinL", Bone::FootL => "footL", Bone::ToeL => "toeL", Bone::ThighR => "thighR", Bone::ShinR => "shinR", Bone::FootR => "footR", Bone::ToeR => "toeR", Bone::FingerR => "fingerR", Bone::FingerL => "fingerL", Bone::HeadTop => "headTop" }
    }
}
/// whose body this is
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, ToBytes)]
pub enum Who { Guard(GuardId), Player(PlayerId) }

/// THE BLOW'S HANDS FOLLOW HIS MEN (takedown.ts: `live: () => marks(by, vic, style)`): the only closure ever given to it
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct BlowLive { pub by: CharId, pub vic: CharId, pub style: BlowStyle }

/// THE TRUNK'S PATH IN A GUIDED FALL (character.ts `led`): the pelvis led over [p0, p1], the turn over [n0, n1]
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Led { pub pv: V3, pub p_t: V3, pub v_t: V3, pub p0: f64, pub p1: f64, pub n0: f64, pub n1: f64, pub v0n: V3 }
/// `frame` of a guided fall, by the fall's kind (the closures of character.ts's guided fall, with what they captured)
#[derive(Clone, Copy, Debug, ToBytes)]
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
#[derive(Clone, Copy, Debug, ToBytes)]
pub enum FallNod {
    Fold { slow: bool, t_a: f64, lay: f64 },
    Cut { beat: f64, dur: f64, end: f64 },
    /// sign of the snap back: away·f < 0 ? 1 : -1
    Topple { sign: f64 },
    Plain,
}
/// `at` of a pin of a guided fall: where the pin wants the particle at time t. All but Lead read the guide's `frame`
#[derive(Clone, Copy, Debug, ToBytes)]
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
#[derive(Clone, Copy, Debug, ToBytes)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Words)]
pub enum KitItem {
    Pistol,
    Rifle,
    Can,
    Knife,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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
    pub middle1_r: usize,
    pub middle1_l: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum UpperMode {
    #[default] None,
    Torch,
    Aim,
    Relaxed,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum Stance {
    #[default] None,
    Wall,
    Stack,
    HighReady,
    LowReady,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum LongStance {
    PatrolLong,
    LowLong,
    Shoulder,
    PortLong,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum SignalKind {
    Hold,
    Go,
    Rally,
    Aside,
    Squeeze,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum TellKind {
    Watch,
    Neck,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum StrikeKind {
    Cross,
    Jab,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum BlowStyle {
    #[default] Mouth,
    Sweep,
    Slash,
    Chop,
    Palm,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum FallStyle {
    Fold,
    Topple,
    Sink,
    Cut,
    Slump,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum TakenStyle {
    #[default] Arch,
    Hunch,
    Snap,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum ReadyKind {
    #[default] Grab,
    Seat,
    Strike,
    GrabGun,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum HoldPhase {
    #[default] Held,
    Grab,
    Choke,
    Whip,
    Kill,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum HoldVariant {
    #[default] Gun,
    Arm,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StripPose {
    pub t: f64,
    pub at: Option<V3>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorPush {
    pub p: V3,
    pub n: V3,
    pub w: f64,
    pub hand: f64,
    pub hold_only: Option<bool>,
    pub reach: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum WorkKind {
    #[default] Keys,
    Charge,
    Take,
    Lever,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum CarryKind2 {
    Torch,
    Weapon,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HaulPose {
    pub l: V3,
    pub r: V3,
    pub bow: f64,
    pub grip: f64,
    pub rw: Option<f64>,
    pub look: Option<f64>,
    pub tilt: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimParams {
    pub walk_speed: f64,
    pub jog_speed: f64,
    pub crouch_speed: f64,
    pub sprint_speed: f64,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Spring {
    pub v: f64,
    pub x: f64,
}
impl Spring {
    /// what a new one holds
    pub fn blank() -> Self {
        Spring {
            v: 0.0,
            x: 0.0,
        }
    }
}
impl Default for Spring { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ReloadBeat {
    pub dur: f64,
    pub out: Option<f64>,
    pub in_: Option<f64>,
    pub rack: Option<Option<f64>>,
    pub keep: Option<f64>,
    pub hand: Option<bool>,
    pub exact: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MvGait {
    pub live: bool,
    pub g: f64,
    pub u: f64,
    pub st: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ReadyShape {
    pub lean: f64,
    pub blade: f64,
    pub clav: f64,
    pub l: crate::character::ReadyShapeL,
    pub knife: bool,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct ReadyShapeLReach {
    pub frac: f64,
    pub r: [f64; 2],
    pub dy: [f64; 2],
    pub bias: f64,
    pub h0: f64,
    pub y0: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlowKey {
    pub t: f64,
    pub at: V3,
    pub blade: V3,
    pub fing: V3,
    pub pole: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct BlowOpts {
    pub hit: Option<f64>,
    pub end: Option<f64>,
    pub live: Option<crate::character::BlowLive>,
    pub rise: Option<bool>,
    pub tail: Option<bool>,
}

#[derive(Clone, Debug, ToBytes)]
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
    pub carry: Option<Option<crate::character::CarryKind2>>,
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
    pub act: Option<crate::character::AnimAct>,
    pub slump: f64,
    pub slump_w: f64,
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
    pub rlc: crate::character::AnimRlc,
    pub rl_shown: crate::character::AnimRlShown,
    pub kit: Option<crate::character::AnimKit>,
    pub kit_front: bool,
    pub kit_rest: f64,
    pub kit_hold: Option<Option<crate::character::KitItem>>,
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
    pub poses: PoseSetId,
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
    /// INVENTED by a review: see the proposals
    pub pos_in: V3,
}
impl Anim {
    /// what a new one holds
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
            act: None,
            slump: 0.0,
            slump_w: 0.0,
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
            idle_feet: crate::character::AnimIdleFeet::default(),
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
            nodes: crate::character::AnimNodes::default(),
            poses: PoseSetId(0),
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
            pos_in: [0.0, 0.0, 0.0],
        }
    }
}
impl Default for Anim { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimWallTakeFeet {
    pub l: crate::character::AnimWallTakeFeetL,
    pub r: crate::character::AnimWallTakeFeetR,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimWallTakeFeetL {
    pub p: V3,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimWallTakeFeetR {
    pub p: V3,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimWtk {
    pub l: crate::character::AnimWtkL,
    pub r: crate::character::AnimWtkR,
    pub first: f64,
    pub last: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimWtkL {
    pub p: V3,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimWtkR {
    pub p: V3,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimWatchShown {
    pub w: f64,
    pub yaw: f64,
    pub slip: f64,
    pub lean: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum AnimCarry {
    Torch,
    Weapon,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimLongGrip {
    pub wrist: V3,
    pub fingers: V3,
    pub back: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum AnimBigToolKind {
    #[default] Shotgun,
    Hammer,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimBigSide {
    pub o: V3,
    pub z: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct AnimAct {
    pub clip: String,
    pub loop_: Option<bool>,
    pub rate: Option<f64>,
    pub in_: Option<f64>,
    pub out: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimUpperW {
    pub relaxed: f64,
    pub aim: f64,
    pub torch: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimDazzle {
    pub from: V3,
    pub k: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct AnimRlc {
    pub k: f64,
    pub h: f64,
    pub ch: Option<Vec<crate::character::AnimRlcChItem>>,
    pub at: f64,
    pub still: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimRlcChItem {
    pub x: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimRlShown {
    pub carry: f64,
    pub head: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimMv2 {
    pub bd: crate::character::Spring,
    pub bd_k: f64,
    pub cl: crate::character::Spring,
    pub dip: crate::character::Spring,
    pub rw: crate::character::Spring,
    pub rl_at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimMv2Shown {
    pub reload: f64,
    pub bound: f64,
    pub clutch: f64,
    pub dip: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimTk {
    pub style: crate::character::TakenStyle,
    pub t: f64,
    pub delay: f64,
    pub off: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimHdFix {
    pub tgt: V3,
    pub pole: V3,
    pub pole_w: f64,
    pub rot: Quat,
    pub w: f64,
    pub rw: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimWst {
    pub wl: crate::character::Spring,
    pub side: crate::character::Spring,
    pub peek: crate::character::Spring,
    pub park: crate::character::Spring,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct AnimLg {
    pub w: crate::character::Spring,
    pub seen: Option<crate::character::LongStance>,
    pub ch: Vec<crate::character::Spring>,
    pub pull: V3,
    pub lat: [f64; 2],
    pub run: f64,
    pub sprint: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimDp {
    pub w_l: crate::character::Spring,
    pub w_r: crate::character::Spring,
    pub last: Option<crate::character::DoorPush>,
    pub right: bool,
    pub reach_ok: bool,
    pub tw: crate::character::Spring,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimSw {
    pub cur: Option<Option<crate::character::AnimCarry>>,
    pub from: Option<crate::character::AnimCarry>,
    pub to: Option<crate::character::AnimCarry>,
    pub t: f64,
    pub dur: f64,
    pub shown: bool,
    pub wait: f64,
    pub belt: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimRx {
    pub left: f64,
    pub w: crate::character::Spring,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimGait {
    pub mid: f64,
    pub linger: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimJogSeamItem {
    pub node: f64,
    pub d: Option<V3>,
    pub axis: Option<V3>,
    pub ang: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimRunGait {
    pub jog: crate::character::AnimRunGaitJog,
    pub sprint: crate::character::AnimRunGaitSprint,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimRunGaitJog {
    pub c: f64,
    pub c_l: f64,
    pub c_r: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimRunGaitSprint {
    pub c: f64,
    pub c_l: f64,
    pub c_r: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimPlantL {
    pub dev: f64,
    pub pz: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimPlantR {
    pub dev: f64,
    pub pz: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimGz {
    pub pitch: f64,
    pub yaw_ref: f64,
    pub yaw_init: bool,
    pub w: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimFeet {
    pub w: f64,
    pub gate: f64,
    pub l: crate::character::AnimFeetL,
    pub r: crate::character::AnimFeetR,
    pub next: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimFeetL {
    pub p: V3,
    pub yaw: f64,
    pub swing: f64,
    pub from: V3,
    pub from_yaw: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimFeetR {
    pub p: V3,
    pub yaw: f64,
    pub swing: f64,
    pub from: V3,
    pub from_yaw: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimThrLast {
    pub r: V3,
    pub l: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimThrCarry {
    pub r: V3,
    pub l: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimIdleFeet {
    pub l: V3,
    pub r: V3,
    pub r_l: Quat,
    pub r_r: Quat,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimKickFrom {
    pub l: V3,
    pub r_l: Quat,
    pub r: V3,
    pub r_r: Quat,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum AnimKickLeg {
    Left,
    Right,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimNv {
    pub t: f64,
    pub hand: bool,
    pub down: bool,
    pub k0: f64,
    pub c0: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimMvPo {
    pub stw: f64,
    pub stn: f64,
    pub dzw: f64,
    pub gun: bool,
    pub gun_p: V3,
    pub gun_r: Quat,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimHdIdle {
    pub upper: crate::character::UpperMode,
    pub long: bool,
    pub crouch: f64,
    pub carry: Option<Option<crate::character::AnimCarry>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RagDrive {
    pub pose: crate::anim_skeleton::Pose,
    pub ov: Vec<Option<crate::anim_skeleton::NodeOverride>>,
    pub hips: usize,
    pub hips_rot0: Quat,
    pub rot0: Vec<Quat>,
    pub dir0: Vec<V3>,
    pub delta: Vec<Quat>,
    pub limb_ov: Vec<usize>,
    pub synced: bool,
}

#[derive(Clone, Debug, ToBytes)]
pub struct GuidePin {
    pub i: f64,
    pub t0: f64,
    pub off: f64,
    pub at: crate::character::GuidePinAt,
}
impl Default for GuidePin {
    fn default() -> Self {
        GuidePin {
            i: 0.0,
            t0: 0.0,
            off: 0.0,
            at: crate::character::GuidePinAt::Rigid { r: [0.0; 3] },
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
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
impl Default for FallGuide {
    fn default() -> Self {
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

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct CharacterDesc {
    pub id: f64,
    pub tint: V3,
    pub tint2: V3,
    pub pos: V3,
    pub yaw: f64,
    pub men: Option<crate::Opaque>,
}

#[derive(Clone, Debug, ToBytes)]
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
    /// INVENTED: his row of the block's men (ManCols.r). None (0xffffffff) once he has left, or for a man of no game
    pub man_row: u32,
    /// INVENTED: owner, rank, reason, since of a man with no row: the numbers he took with him (manLeaves)
    pub man_kept: Option<[f64; 4]>,
}
impl Character {
    /// what a new one holds
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
            man_row: 0xffffffff,
            man_kept: None,
        }
    }
}
impl Default for Character { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CharacterBreathe {
    pub breath_hz: f64,
    pub breath_rise: f64,
    pub breath_in: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CharacterTurn {
    pub on: bool,
    pub h: f64,
    pub k: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct WORK {
    pub key_lead: f64,
    pub tip_len: f64,
    pub hover: f64,
    pub key_off: f64,
    pub lever: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SWAP {
    pub draw: f64,
    pub stow: f64,
    pub away: f64,
    pub take: f64,
    pub lag: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HOLDPOLISHUP {
    pub lay_ahead: f64,
    pub dip: f64,
    pub back: f64,
    pub m: f64,
    pub twist: f64,
    pub side: f64,
    pub tip: f64,
    pub arch: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CharacterModHOLDSHOW {
    pub kill: bool,
    pub beats: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BLEND {
    pub walk_in0: f64,
    pub walk_in1: f64,
    pub walk_top: f64,
    pub jog_at: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BOB {
    pub keep: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RUNWARP {
    pub stride: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SPRINTWARP {
    pub stride: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RUNLINGER {
    pub depth: f64,
    pub on: [f64; 2],
    pub off: [f64; 2],
    pub bleed: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GAZEEL {
    pub stand: f64,
    pub crouch: f64,
    pub sprint: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GAZE {
    pub keep: f64,
    pub yaw_damp: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct SOFTKNEE {
    pub on: f64,
    pub from: f64,
    pub to: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FEET {
    pub on: f64,
    pub swing: f64,
    pub lift: f64,
    pub dist: f64,
    pub turn_deg: f64,
    pub pivot: f64,
    pub lag_cap_deg: f64,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct STEALTHWalk {
    pub stride: f64,
    pub rate: f64,
    pub fwd: V3,
    pub back: V3,
    pub flare_pole: f64,
    pub tuck: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct STEALTHRise {
    pub down: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct STEALTHCreep {
    pub at: V3,
    pub floor: f64,
    pub rate: f64,
    pub swing: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CharacterModAT0Value {
    pub rows: f64,
    pub at: Vec<i32>,
    pub v: Vec<f32>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LEANPOSE {
    pub deg: f64,
    pub level: f64,
    pub rate: f64,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct WALLPOSEFeet {
    pub lead: V3,
    pub trail: V3,
    pub lead_crouch: V3,
    pub trail_crouch: V3,
    pub trail_heel: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WALLPOSESlide {
    pub line: f64,
    pub narrow: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WALLPOSEHands {
    pub along: f64,
    pub along_crouch: f64,
    pub up: f64,
    pub up_crouch: f64,
    pub off: f64,
    pub out: f64,
    pub peek_in: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WALLPOSEElbow {
    pub out: f64,
    pub room: f64,
    pub down: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WALLPOSEPeek {
    pub lean: f64,
    pub shift: f64,
    pub turn: f64,
    pub gaze: f64,
    pub tilt: f64,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct WALLPOSEMendElbow {
    pub out: f64,
    pub room: f64,
    pub down: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct LONGRELOAD {
    pub box_: crate::character::LONGRELOADBox,
    pub back: crate::character::LONGRELOADBox,
    pub head: crate::character::LONGRELOADHead,
    pub in_: f64,
    pub out: f64,
    pub end: f64,
    pub head_in: f64,
    pub head_out: f64,
    pub rate: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LONGRELOADBox {
    pub at: V3,
    pub pitch: f64,
    pub aim_k: f64,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LONGRELOADHead {
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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
    pub count: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WALLTUCK {
    pub long: crate::character::WALLTUCKLong,
    pub pistol: crate::character::WALLTUCKPistol,
    pub over: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct WALLTUCKPistol {
    pub fwd: f64,
    pub up: f64,
    pub pitch: f64,
    pub tuck: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CharacterModLONGSTEADY {
    pub tau: f64,
    pub run_yaw: f64,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct HANDCan {
    pub curl: Vec<f64>,
    pub thumb: f64,
}

#[derive(Clone, Debug, Default)]
pub struct MV2 {
    pub reload: crate::character::MV2Reload,
    pub bound: crate::character::MV2Bound,
    pub clutch: crate::character::MV2Clutch,
    pub hurt: crate::character::MV2Hurt,
    pub shaken: crate::character::MV2Shaken,
    pub dip: crate::character::MV2Dip,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MV2Reload {
    pub box_: crate::character::MV2ReloadBox,
    pub back: crate::character::MV2ReloadBox,
    pub pouch: [V3; 2],
    pub pole: V3,
    pub pole_w: f64,
    pub fumble: f64,
    pub tap: f64,
    pub flare: crate::character::MV2ReloadFlare,
    pub fling: f64,
    pub rate: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MV2ReloadBox {
    pub mag: V3,
    pub strip: V3,
    pub handle: V3,
    pub pull: V3,
    pub hf: V3,
    pub hb: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MV2ReloadFlare {
    pub off: V3,
    pub tap: V3,
    pub pole: V3,
}

#[derive(Clone, Debug, Default)]
pub struct MV2Bound {
    pub rate: f64,
    pub back: crate::character::MV2BoundBack,
    pub head: crate::character::MV2BoundHead,
}

#[derive(Clone, Debug, Default)]
pub struct MV2BoundBack {
    pub up: f64,
    pub out: f64,
    pub behind: f64,
    pub curl: Vec<f64>,
    pub thumb: f64,
}

#[derive(Clone, Debug, Default)]
pub struct MV2BoundHead {
    pub up: f64,
    pub out: f64,
    pub behind: f64,
    pub elbow: f64,
    pub curl: Vec<f64>,
    pub knuckle: Vec<f64>,
    pub thumb: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MV2Clutch {
    pub rate: f64,
    pub at: V3,
    pub pole: V3,
    pub fold: crate::character::MV2ClutchFold,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MV2ClutchFold {
    pub rate: f64,
    pub pitch: f64,
    pub pitch_too: f64,
    pub head: f64,
    pub drop: f64,
    pub sway: f64,
    pub hz: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct MV2Shaken {
    pub rate_in: f64,
    pub rate_out: f64,
    pub pitch: f64,
    pub head: f64,
    pub drop: f64,
    pub brace: f64,
}

#[derive(Clone, Debug, Default)]
pub struct MV2Dip {
    pub pitch: f64,
    pub down: f64,
    pub back: f64,
    pub rate_in: f64,
    pub rate_out: f64,
    pub lane: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct REACH {
    pub in_: f64,
    pub out: f64,
    pub pole: V3,
    pub pole_up: Vec<f64>,
    pub curl: Vec<f64>,
    pub thumb: f64,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct KNIFEBLOWCross {
    pub end: f64,
    pub keys: Vec<crate::character::BlowKey>,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct BLOWLegs {
    pub gap: f64,
    pub lift: f64,
    pub straight: f64,
    pub up: f64,
    pub fwd: f64,
}

#[derive(Clone, Debug, Default)]
pub struct TAKEN {
    pub sag: JsMap<String, f64>,
    pub secs: f64,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct CREEPLow {
    pub rate: f64,
    pub up: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct READYBARE {
    pub l: crate::character::READYBAREL,
    pub r: crate::character::READYBARER,
    pub way: crate::character::READYBAREWay,
    pub out: [f64; 2],
    pub out_r: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct READYBARER {
    pub at: V3,
    pub pole: V3,
    pub fing: V3,
    pub back: V3,
    pub curl: f64,
    pub thumb: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct READYBAREWay {
    pub by: f64,
    pub low: f64,
    pub near: f64,
    pub lift: [f64; 2],
    pub right: [f64; 2],
    pub rate: f64,
    pub drop: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct RD {
    pub up: f64,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct BIGMOVE {
    pub grab: f64,
    pub take: Vec<f64>,
    pub join: Vec<f64>,
    pub put: f64,
    pub put_at: f64,
    pub unjoin: f64,
    pub snug: crate::character::BIGMOVESnug,
    pub arc: V3,
    pub wait: V3,
    pub hammer: crate::character::BIGMOVEHammer,
    pub shotgun: crate::character::BIGMOVEShotgun,
    pub fist: Vec<f64>,
    pub open: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct BIGMOVESnug {
    pub hide: f64,
    pub take: Vec<f64>,
    pub put_at: f64,
    pub show: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct BIGMOVEHammerReady {
    pub g: V3,
    pub d: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BIGMOVEHammerWound {
    pub g: V3,
    pub d: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BIGMOVEHammerOver {
    pub g: V3,
    pub d: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BIGMOVEHammerWoundBody {
    pub yaw: f64,
    pub pel_yaw: f64,
    pub pitch: f64,
    pub back: f64,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BIGMOVEHammerBlowBody {
    pub yaw: f64,
    pub pel_yaw: f64,
    pub pitch: f64,
    pub pitch_far: f64,
    pub fwd: f64,
    pub fwd_far: f64,
    pub drop: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BIGMOVEHammerStance {
    pub l: [f64; 2],
    pub r: [f64; 2],
    pub lift: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MvOnK {
    Reload,
    Fold,
    Steady,
    On,
    Entry,
    Flinch,
    Fall,
    Toss,
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

#[derive(Clone, Debug, Default)]
pub struct BlowsNearB {
    pub t: f64,
    pub end: f64,
    pub blows: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct ThrowTables {
    pub k1: Vec<Vec<[f64; 2]>>,
    pub k3: Vec<Vec<(f64, Vec<f64>)>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BigReboundKind {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BigOffB {
    pub t: f64,
    pub equip: f64,
    pub stow: f64,
    pub end: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimKitSwapShown {
    pub r: V3,
    pub l: V3,
    pub fingers: V3,
    pub back: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimReloadShow {
    pub out: bool,
    pub hand: bool,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimKickDoorOpts {
    pub leg: Option<crate::character::AnimKickDoorOptsLeg>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnimKickDoorOptsLeg {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimTrunkFrame {
    pub left: V3,
    pub up: V3,
    pub fwd: V3,
}

#[derive(Clone, Debug)]
pub enum AnimReloadArmAskOwn {
    Arm0(crate::gunreload::FeedPathArm0),
    Arm1(crate::gunreload::FeedPathArm1),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimStanceOf {
    pub c: f64,
    pub c_l: f64,
    pub c_r: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimLgChannelsItem {
    pub x: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimTakenLayerT {
    pub style: crate::character::TakenStyle,
    pub t: f64,
    pub delay: f64,
    pub off: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimTakenLayerAskT {
    pub style: crate::character::TakenStyle,
    pub off: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimDoorHands {
    pub l: f64,
    pub r: f64,
}

#[derive(Clone, Debug, Default)]
pub struct AnimWorkInfo {
    pub pose: Option<crate::character::WorkPose>,
    pub w: f64,
    pub hand: f64,
    pub two: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimSwapInfo {
    pub a: f64,
    pub pending: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CharacterHandFrameR {
    pub fingers: V3,
    pub back: V3,
    pub thumb: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CharacterGripFrameR {
    pub bore: V3,
    pub top: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CharacterHandFrameL {
    pub fingers: V3,
    pub back: V3,
    pub thumb: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CharacterChestFrame {
    pub o: V3,
    pub left: V3,
    pub up: V3,
    pub fwd: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CharacterBreathFrameK {
    pub breath_hz: f64,
    pub breath_rise: f64,
    pub breath_in: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CharacterCapsules {
    pub caps: Vec<Vec<f64>>,
    pub bound: [f64; 4],
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
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
pub fn hold_polish_up() -> crate::character::HOLDPOLISHUP { crate::character::HOLDPOLISHUP { lay_ahead: 0.9, dip: 0.06, back: 0.035, m: 0.014, twist: 2.4, side: 7.0, tip: 8.0, arch: -5.0 } }

/// (src/game/character.ts:357 BLEND: a table nothing writes)
pub fn blend() -> crate::character::BLEND { crate::character::BLEND { walk_in0: 0.12, walk_in1: 0.55, walk_top: 1.6, jog_at: 2.7 } }

/// (src/game/character.ts:359 BOB: a table nothing writes)
pub fn bob() -> crate::character::BOB { crate::character::BOB { keep: 0.55 } }

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
pub fn long_reload() -> crate::character::LONGRELOAD { crate::character::LONGRELOAD { box_: crate::character::LONGRELOADBox { at: [-0.15, -0.075, 0.19], pitch: 20.0, aim_k: 0.0, yaw: 12.0 }, back: crate::character::LONGRELOADBox { at: [-0.16, -0.06, 0.21], pitch: 12.0, aim_k: 0.0, yaw: 4.0 }, head: crate::character::LONGRELOADHead { yaw: -10.0, pitch: 22.0, roll: 0.0 }, in_: 0.35, out: 0.35, end: 0.06, head_in: 0.3, head_out: 0.3, rate: 8.0 } }

/// (src/game/character.ts:674 LC: a table nothing writes)
pub fn lc() -> crate::character::LC { crate::character::LC { side: 0.0, up: 1.0, fwd: 2.0, pitch: 3.0, aim_k: 4.0, yaw: 5.0, blade: 6.0, lean: 7.0, head_yaw: 8.0, head_pitch: 9.0, head_roll: 10.0, elbow_r: 11.0, elbow_l: 14.0, count: 21.0 } }

/// (src/game/character.ts:681 WALL_TUCK: a table nothing writes)
pub fn wall_tuck() -> crate::character::WALLTUCK { crate::character::WALLTUCK { long: crate::character::WALLTUCKLong { side: -0.165, up: -0.245, fwd: 0.185, pitch: -76.0, yaw: 15.0, aim_k: 0.0, elbow_r: [-0.75, -1.0, -0.45], elbow_l: [0.6, -1.0, -0.25] }, pistol: crate::character::WALLTUCKPistol { fwd: 0.2, up: -0.19, pitch: -70.0, tuck: 0.8 }, over: 0.12 } }

/// (src/game/character.ts:724 LONG_KICK_BACK)
pub const LONG_KICK_BACK: f64 = 0.03;

/// (src/game/character.ts:802 HAND: a table nothing writes)
pub fn hand() -> crate::character::HAND { crate::character::HAND { can: crate::character::HANDCan { curl: alloc::vec![0.56, 0.58, 0.62, 0.68], thumb: 0.3 }, curl: alloc::vec![0.3, 0.35, 0.41, 0.48], thumb: 0.38, thumb_in: 34.0, thumb_bend: 15.0, thumb_close: 10.0, thumb_own: 1.0, spread: -0.15, vary: 0.05, side_vary: 0.035, finger_vary: 0.025, far: 45.0, trig: alloc::vec![6.0, 10.0, 6.0], trig_spread: 6.0, trig_trained: 0.6, trig_hold: 0.7 } }

/// (src/game/character.ts:833 MV2: a table nothing writes)
pub fn mv2() -> crate::character::MV2 { crate::character::MV2 { reload: crate::character::MV2Reload { box_: crate::character::MV2ReloadBox { mag: [0.085, -0.17, 0.125], strip: [0.09, -0.26, 0.15], handle: [0.05, -0.08, 0.07], pull: [0.05, -0.08, -0.03], hf: [-0.7, 0.6, 0.1], hb: [0.5, -0.5, 0.6] }, back: crate::character::MV2ReloadBox { mag: [0.085, -0.15, -0.115], strip: [0.09, -0.23, -0.17], handle: [0.1, 0.01, 0.17], pull: [0.1, 0.01, 0.08], hf: [-0.9, 0.1, 0.3], hb: [0.0, 1.0, 0.0] }, pouch: [[0.15, 0.04, 0.215], [0.1, -0.03, 0.195]], pole: [0.7, -0.6, -0.25], pole_w: 0.7, fumble: 0.055, tap: 0.11, flare: crate::character::MV2ReloadFlare { off: [0.12, 0.12, -0.03], tap: [0.13, 0.04, 0.0], pole: [1.0, 0.35, -0.2] }, fling: 0.06, rate: 14.0 }, bound: crate::character::MV2Bound { rate: 6.0, back: crate::character::MV2BoundBack { up: 0.07, out: 0.045, behind: 0.15, curl: alloc::vec![0.64, 0.7, 0.76, 0.82], thumb: 0.7 }, head: crate::character::MV2BoundHead { up: 0.2, out: 0.095, behind: 0.065, elbow: -0.2, curl: alloc::vec![0.28, 0.32, 0.36, 0.4], knuckle: alloc::vec![16.0, 5.0], thumb: 0.3 } }, clutch: crate::character::MV2Clutch { rate: 7.0, at: [-0.14, 0.12, 0.12], pole: [1.0, -0.05, -0.35], fold: crate::character::MV2ClutchFold { rate: 6.0, pitch: 24.0, pitch_too: 34.0, head: 20.0, drop: 0.05, sway: 3.0, hz: 1.0 } }, hurt: crate::character::MV2Hurt { rate: 4.0, pitch: 11.0, lean: 8.0, head: 9.0, head_brace: 0.5, drop: 0.025, sway: 1.5, hz: 0.45, brace: 0.7, step_pitch: 5.0, step_roll: 10.0, step_yaw: 5.0, step_head: 4.0, step_drop: 0.04, step_side: 0.06, arm: 1.0 }, shaken: crate::character::MV2Shaken { rate_in: 12.0, rate_out: 3.0, pitch: 11.0, head: 18.0, drop: 0.1, brace: 0.75 }, dip: crate::character::MV2Dip { pitch: -36.0, down: 0.035, back: 0.02, rate_in: 13.0, rate_out: 9.0, lane: alloc::vec![0.25, 1.9, 0.34] } } }

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
pub fn knife_blow() -> crate::character::KNIFEBLOW { crate::character::KNIFEBLOW { snap: 0.16, t_clear: 0.06, clear: [0.14, 0.05], az: 0.7, reach: 0.96, fade: 0.12, cross: crate::character::KNIFEBLOWCross { end: 0.6, keys: alloc::vec![crate::character::BlowKey { t: 0.16, at: [0.27, 0.0, 0.3], blade: [0.1, 0.9, 0.5], fing: [0.7, 0.1, 0.6], pole: [-0.5, -0.8, 0.1] }, crate::character::BlowKey { t: 0.27, at: [-0.05, 0.03, 0.5], blade: [-0.1, 0.8, 0.6], fing: [0.2, 0.0, 1.0], pole: [-0.7, -0.7, 0.0] }, crate::character::BlowKey { t: 0.36, at: [-0.22, -0.06, 0.36], blade: [-0.4, 0.8, 0.45], fing: [-0.2, -0.1, 1.0], pole: [-0.8, -0.6, -0.1] }] }, jab: crate::character::KNIFEBLOWCross { end: 0.52, keys: alloc::vec![crate::character::BlowKey { t: 0.15, at: [-0.02, -0.32, 0.08], blade: [0.0, 0.5, 1.0], fing: [0.05, -0.5, 0.8], pole: [-0.5, -0.3, -0.8] }, crate::character::BlowKey { t: 0.24, at: [0.12, -0.2, 0.56], blade: [0.0, 1.0, 0.8], fing: [0.2, 0.1, 1.0], pole: [-0.6, -0.8, -0.1] }, crate::character::BlowKey { t: 0.33, at: [0.0, -0.28, 0.2], blade: [0.0, 0.6, 1.0], fing: [0.05, -0.4, 0.9], pole: [-0.5, -0.5, -0.7] }] } } }

/// (src/game/character.ts:1077 KNIFE_SLASH: a table nothing writes)
pub fn knife_slash() -> crate::character::KNIFEBLOWCross { crate::character::KNIFEBLOWCross { end: 0.52, keys: alloc::vec![crate::character::BlowKey { t: 0.14, at: [0.3, -0.38, 0.12], blade: [-0.3, 0.6, 0.7], fing: [0.6, -0.3, 0.6], pole: [-0.3, -0.6, -0.6] }, crate::character::BlowKey { t: 0.24, at: [0.1, -0.08, 0.62], blade: [-0.6, 0.5, 0.6], fing: [0.1, 0.0, 1.0], pole: [-0.7, -0.6, 0.0] }, crate::character::BlowKey { t: 0.34, at: [-0.46, 0.3, 0.3], blade: [-0.8, 0.5, 0.2], fing: [-0.7, 0.3, 0.6], pole: [-0.8, -0.5, -0.2] }, crate::character::BlowKey { t: 0.42, at: [-0.46, 0.28, 0.28], blade: [-0.8, 0.5, 0.2], fing: [-0.7, 0.3, 0.6], pole: [-0.8, -0.5, -0.2] }] } }

/// (src/game/character.ts:1084 KNIFE_BACK: a table nothing writes)
pub fn knife_back() -> crate::character::KNIFEBLOWCross { crate::character::KNIFEBLOWCross { end: 0.6, keys: alloc::vec![crate::character::BlowKey { t: 0.15, at: [-0.02, -0.4, -0.04], blade: [0.0, 0.25, 1.0], fing: [0.0, -1.0, 0.25], pole: [-0.4, 0.0, -1.0] }, crate::character::BlowKey { t: 0.27, at: [0.08, -0.34, 0.33], blade: [0.0, 0.2, 1.0], fing: [0.0, -0.9, 0.35], pole: [-0.4, -0.2, -1.0] }, crate::character::BlowKey { t: 0.44, at: [0.08, -0.36, 0.31], blade: [0.0, 0.2, 1.0], fing: [0.0, -0.9, 0.35], pole: [-0.4, -0.2, -1.0] }] } }

/// (src/game/character.ts:1091 BLOW: a table nothing writes)
pub fn blow() -> crate::character::BLOW { crate::character::BLOW { rise: { let mut t = JsMap::new(); t.set(String::from("chop"), 0.85); t.set(String::from("palm"), 0.6); t.set(String::from("mouth"), 0.9); t.set(String::from("sweep"), 0.8); t.set(String::from("slash"), 0.7); t }, clip: { let mut t = JsMap::new(); t.set(String::from("chop"), 0.25); t.set(String::from("palm"), 0.35); t.set(String::from("mouth"), 1.0); t.set(String::from("sweep"), 1.0); t.set(String::from("slash"), 1.0); t }, reach: 0.98, fist: alloc::vec![0.92, 0.95, 0.95, 0.95], open: alloc::vec![0.08, 0.06, 0.08, 0.12], loose: alloc::vec![0.25, 0.3, 0.3, 0.35], cup: alloc::vec![0.35, 0.35, 0.4, 0.45], legs: crate::character::BLOWLegs { gap: 0.2, lift: 0.05, straight: 0.985, up: 0.4, fwd: 0.6 }, tail: 1.0, sink: { let mut t = JsMap::new(); t.set(String::from("chop"), 0.8); t.set(String::from("palm"), 0.6); t.set(String::from("mouth"), 1.0); t.set(String::from("sweep"), 0.75); t.set(String::from("slash"), 0.75); t }, walk: alloc::vec![0.4, 1.5] } }

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
pub fn kit_of() -> Vec<Option<crate::character::KitItem>> { alloc::vec![None, Some(crate::character::KitItem::Pistol), Some(crate::character::KitItem::Rifle), Some(crate::character::KitItem::Can), Some(crate::character::KitItem::Knife)] }

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

/// (src/game/character.ts:1202 READY_UP: a table nothing writes)
pub fn ready_up() -> crate::character::READYUP { crate::character::READYUP { over: [0.25, 0.45], rise: 38.0, look: 0.5, dy: 0.36, r: [0.32, 0.42], fing: 0.3, slow: 0.8, rate: 6.0 } }

/// (src/game/character.ts:1221 SMOKEARM: a table nothing writes)
pub fn smokearm() -> crate::character::SMOKEARM { crate::character::SMOKEARM { at: [0.075, -0.15, 0.125], fingers: [-0.55, 0.8, -0.1], back: [0.35, 0.1, 0.9], pole: [0.7, -0.6, 0.35], shrug: 5.0, curl: [0.3, 0.34, 0.6, 0.66], thumb: 0.3, rate: 1.9, away: 3.0 } }

/// (src/game/character.ts:1277 RD: a table nothing writes)
pub fn rd() -> crate::character::RD { crate::character::RD { up: 35.0 } }

/// (src/game/character.ts:1277 RD_N)
pub const RD_N: f64 = 36.0;

/// (src/game/character.ts:1283 THROW_DUR)
pub const THROW_DUR: f64 = 0.75;

/// (src/game/character.ts:1283 THROW_RELEASE)
pub const THROW_RELEASE: f64 = 0.28;

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
pub fn bigmove() -> crate::character::BIGMOVE { crate::character::BIGMOVE { grab: 0.5, take: alloc::vec![0.47, 0.82], join: alloc::vec![0.65, 0.96], put: 0.58, put_at: 0.45, unjoin: 0.28, snug: crate::character::BIGMOVESnug { hide: 0.06, take: alloc::vec![0.44, 0.56], put_at: 0.8, show: 0.94 }, arc: [-0.16, 0.12, 0.04], wait: [-0.1, -0.34, 0.2], hammer: crate::character::BIGMOVEHammer { ready: crate::character::BIGMOVEHammerReady { g: [0.06, -0.1, 0.34], d: [-0.42, 0.9, -0.1] }, wound: crate::character::BIGMOVEHammerWound { g: [-0.12, 0.2, -0.05], d: [-0.45, 0.3, -0.84] }, over: crate::character::BIGMOVEHammerOver { g: [0.04, 0.14, 0.24], d: [-0.08, 0.93, 0.36] }, grab_at: 0.1, raise: 0.6, swing: 0.38, swing_min: 0.1, swing_max: 0.2, rebound: 0.25, settle: 0.5, butt: [0.18, 0.05, 0.18], far: [0.6, 1.1], wound_body: crate::character::BIGMOVEHammerWoundBody { yaw: -50.0, pel_yaw: -17.0, pitch: -7.0, back: 0.09, side: -0.03 }, blow_body: crate::character::BIGMOVEHammerBlowBody { yaw: 16.0, pel_yaw: 7.0, pitch: 15.0, pitch_far: 11.0, fwd: 0.07, fwd_far: 0.08, drop: 0.03 }, face: 1.0, drop: 0.035, stance: crate::character::BIGMOVEHammerStance { l: [0.06, 0.07], r: [-0.07, -0.1], lift: 0.025 } }, shotgun: crate::character::BIGMOVEShotgun { pocket: [0.035, -0.05, 0.06], yaw: -22.0, pel_yaw: -10.0, pitch: 6.0, kick: 0.08, climb: 12.0, kick_in: 0.04, kick_out: 0.13, kick_pitch: -6.0, shove: 0.035 }, fist: alloc::vec![0.74, 0.8, 0.84, 0.88], open: alloc::vec![0.26, 0.3, 0.34, 0.38] } }

/// (src/game/character.ts:1426 ANIM_POSES)
pub const ANIM_POSES: f64 = 13.0;

/// (src/game/character.ts:4262 RAG_LIMBS: a table nothing writes)
pub fn rag_limbs() -> Vec<(String, f64, f64, f64)> { alloc::vec![(String::from("thighL"), -1.0, 10.0, 12.0), (String::from("shinL"), 0.0, 12.0, 14.0), (String::from("thighR"), -1.0, 11.0, 13.0), (String::from("shinR"), 2.0, 13.0, 15.0), (String::from("upperArmL"), -1.0, 4.0, 6.0), (String::from("forearmL"), 4.0, 6.0, 8.0), (String::from("upperArmR"), -1.0, 5.0, 7.0), (String::from("forearmR"), 6.0, 7.0, 9.0), (String::from("neck"), -1.0, 2.0, 3.0)] }

/// (src/game/character.ts:4269 RAG_BONES: a table nothing writes)
pub fn rag_bones() -> Vec<crate::character::Bone> { alloc::vec![crate::character::Bone::Hips, crate::character::Bone::Spine2, crate::character::Bone::Neck, crate::character::Bone::Head, crate::character::Bone::UpperArmL, crate::character::Bone::UpperArmR, crate::character::Bone::ForearmL, crate::character::Bone::ForearmR, crate::character::Bone::HandL, crate::character::Bone::HandR, crate::character::Bone::ThighL, crate::character::Bone::ThighR, crate::character::Bone::ShinL, crate::character::Bone::ShinR, crate::character::Bone::FootL, crate::character::Bone::FootR] }

