//! The types of `anim_skeleton`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// A POSE IS A HANDLE: its place in the pool of rust/sim's block (poses.rs new_of, holds) and how many nodes it has. Its columns are reached by functions that take g. rust/sim's layers reach it by its place
#[derive(Clone, Copy, PartialEq, Eq, Debug, ToBytes)]
pub struct Pose { pub at: u32, pub nodes: u32 }
impl Pose {
    pub fn blank() -> Self { Pose { at: 0, nodes: 0 } }
}
impl Default for Pose { fn default() -> Self { Self::blank() } }
/// A BODY'S WORLD OF JOINTS (anim/skeleton.ts SkeletonInstance). tmp and local were scratch; ibms and jointViews views of the rig and of joint_data; ownRot was how globalRot's rows were reused without allocating. `joint_data` is what the picture skins with
#[derive(Clone, Debug, ToBytes)]
pub struct SkeletonInstance { pub rig: RigId, pub globals: Vec<crate::v3::m4::Mat4>, pub global_rot: Vec<Quat>, pub joint_data: Vec<f32> }
impl SkeletonInstance {
    pub fn blank() -> Self { SkeletonInstance { rig: RigId(0), globals: Vec::new(), global_rot: Vec::new(), joint_data: Vec::new() } }
}
/// the rig's named nodes and lists, as a layer is handed them (anim/skeleton.ts LayerCast): keys are literals in the code
#[derive(Clone, Debug, ToBytes)]
pub struct LayerCast { pub nodes: JsMap<&'static str, usize>, pub lists: JsMap<&'static str, Vec<i32>>, pub nums: JsMap<&'static str, Vec<f64>> }
impl LayerCast {
    pub fn blank() -> Self { LayerCast { nodes: JsMap::new(), lists: JsMap::new(), nums: JsMap::new() } }
}
/// A POSE THE TYPESCRIPT KEEPS OUTSIDE THE POOL, as the bridge carries it (Game.loose_poses). In Rust every pose lives in the pool: `pose_take_home` gives it a place when a body first needs one.
#[derive(Clone, Debug, Default, ToBytes)]
pub struct LoosePose { pub nodes: u32, pub floats: Vec<f32> }
impl Pose { /** a place at or past this names a row of Game.loose_poses, no place in the pool */ pub const LOOSE: u32 = 0xffff0000; }

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct PoseSet {
    pub list: Vec<crate::anim_skeleton::Pose>,
    pub most: f64,
    pub home: Option<crate::Opaque>,
    pub stray: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ClipSide {
    pub lines: Vec<u8>,
    /// what the clip store (rust/sim/src/clips.rs) knows this clip by: folded from the clip alone, so the same on every host. On no wire and in no file: it is worked out again where it is 0.
    pub word: u32,
    pub need: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TwistSpec {
    pub node: f64,
    pub yaw: f64,
    pub pitch: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct NodeOverride {
    pub rot: Quat,
    pub pos: Option<V3>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct PoseFK {
    pub rot: Vec<Quat>,
    pub pos: Vec<V3>,
    #[is(RigId(0))] pub rig: RigId,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimSkeletonModPOSEBENCH {
    pub rig: Option<RigId>,
    pub in_place: f64,
    pub owed: f64,
}

#[derive(Clone, Debug, Default)]
pub struct AnimSkeletonModLAYERS {
    pub ran: f64,
    pub roles: Option<RigId>,
    pub names: Option<crate::anim_skeleton::AnimSkeletonModLAYERSNames>,
    pub lists: Vec<Vec<i32>>,
}

#[derive(Clone, Debug, Default)]
pub struct AnimSkeletonModLAYERSNames {
    pub roles: Vec<String>,
    pub lists: Vec<String>,
    pub nums: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct BuildMaskEntriesItem {
    pub name: String,
    pub w: f64,
    pub subtree: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct AnimSkeletonMod {
    /// (src/anim/skeleton.ts:21 POSE_BENCH)
    /// (what rust/sim holds outside the block is written again in a fresh module: in no bytes)
    #[bytes(skip)]
    #[is(crate::anim_skeleton::AnimSkeletonModPOSEBENCH { rig: None, in_place: 0.0, owed: 0.0 })] pub pose_bench: crate::anim_skeleton::AnimSkeletonModPOSEBENCH,
    /// (src/anim/skeleton.ts:33 LAYERS)
    /// (what rust/sim holds outside the block is written again in a fresh module: in no bytes)
    #[bytes(skip)]
    #[is(crate::anim_skeleton::AnimSkeletonModLAYERS { ran: 0.0, roles: None, names: None, lists: Vec::new() })] pub layers: crate::anim_skeleton::AnimSkeletonModLAYERS,
    /// (src/anim/skeleton.ts:238 CLIP_SIDE)
    pub clip_side: JsMap<ClipId, crate::anim_skeleton::ClipSide>,
}

/// (src/anim/skeleton.ts:17 BENCH_NODES)
pub const BENCH_NODES: f64 = 128.0;

/// (src/anim/skeleton.ts:17 BENCH_SLOT)
pub const BENCH_SLOT: f64 = 1280.0;

/// (src/anim/skeleton.ts:102 LAYER_COLS_AT)
pub const LAYER_COLS_AT: f64 = 64.0;

/// (src/anim/skeleton.ts:104 HAND_COLS_AT)
pub const HAND_COLS_AT: f64 = 128.0;

/// (src/anim/skeleton.ts:106 LAYER_KEYS_AT)
pub const LAYER_KEYS_AT: f64 = 264.0;

/// (src/anim/skeleton.ts:106 LAYER_KEY)
pub const LAYER_KEY: f64 = 7.0;

/// (src/anim/skeleton.ts:106 LAYER_KEYS_MAX)
pub const LAYER_KEYS_MAX: f64 = 48.0;

/// (src/anim/skeleton.ts:132 IDENT: a table nothing writes)
pub fn ident() -> Quat { [0.0, 0.0, 0.0, 1.0] }

/// (src/anim/skeleton.ts:237 LINES)
pub const LINES: f64 = 8.0;
impl Default for SkeletonInstance { fn default() -> Self { Self::blank() } }
impl Default for LayerCast { fn default() -> Self { Self::blank() } }
