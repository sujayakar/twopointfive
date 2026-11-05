//! The types of `anim_skeleton`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// A POSE: ten columns of `rows` singles (positions, rotations, scales). The TypeScript's are Float32Arrays and its layers do their arithmetic in singles, so the store is f32 (a question for the conventions)
#[derive(Clone, Debug)]
pub struct Pose { pub num_nodes: usize, pub rows: usize, pub cols: Vec<f32> }
impl Pose {
    pub fn blank() -> Self { Pose { num_nodes: 0, rows: 0, cols: Vec::new() } }
}
/// A BODY'S WORLD OF JOINTS (anim/skeleton.ts SkeletonInstance). tmp and local were scratch; ibms and jointViews views of the rig and of joint_data; ownRot was how globalRot's rows were reused without allocating. `joint_data` is what the picture skins with
#[derive(Clone, Debug)]
pub struct SkeletonInstance { pub rig: RigId, pub globals: Vec<crate::v3::m4::Mat4>, pub global_rot: Vec<Quat>, pub joint_data: Vec<f32> }
impl SkeletonInstance {
    pub fn blank() -> Self { SkeletonInstance { rig: RigId(0), globals: Vec::new(), global_rot: Vec::new(), joint_data: Vec::new() } }
}
/// the rig's named nodes and lists, as a layer is handed them (anim/skeleton.ts LayerCast): keys are literals in the code
#[derive(Clone, Debug)]
pub struct LayerCast { pub nodes: JsMap<&'static str, usize>, pub lists: JsMap<&'static str, Vec<i32>>, pub nums: JsMap<&'static str, Vec<f64>> }
impl LayerCast {
    pub fn blank() -> Self { LayerCast { nodes: JsMap::new(), lists: JsMap::new(), nums: JsMap::new() } }
}
/// WHICH TABLE OF A LAYER'S KEPT COLUMNS (anim/skeleton.ts LayerCols): a static table of character.ts, 26 of CharacterAnimator and Character's turnCols. It captures nothing; whose columns they are is said at the call. get and put of each arm are generated from the TypeScript's tables, as their order is the contract with rust/sim/src/layers.rs
#[derive(Clone, Debug)]
pub enum ColsTable {
    Act,
    Lean,
    Gaze,
    MvPre,
    MvPost,
    MvPostArm,
    HandsPass,
    RunPlant,
    Life,
    LongArm,
    Wall,
    Hand,
    Kick,
    Tell,
    Watch,
    Creep,
    Stealth,
    Cable,
    Kit,
    Rd,
    Door,
    Work,
    Hold,
    Held,
    Big,
    Throw,
    Turn,
}

/// (TypeScript: ColsOf, src/anim/skeleton.ts:99)
#[derive(Clone, Copy, Debug)]
pub struct ColsOf {
    pub n: f64,
}
impl ColsOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ColsOf {
            n: 0.0,
        }
    }
}
impl Default for ColsOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ClipSide, src/anim/skeleton.ts:236)
#[derive(Clone, Debug)]
pub struct ClipSide {
    pub lines: Vec<u8>,
    pub at: f64,
    pub need: f64,
}
impl ClipSide {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ClipSide {
            lines: Vec::new(),
            at: 0.0,
            need: 0.0,
        }
    }
}
impl Default for ClipSide { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: TwistSpec, src/anim/skeleton.ts:394)
#[derive(Clone, Copy, Debug)]
pub struct TwistSpec {
    pub node: f64,
    pub yaw: f64,
    pub pitch: f64,
}
impl TwistSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TwistSpec {
            node: 0.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}
impl Default for TwistSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: NodeOverride, src/anim/skeleton.ts:397)
#[derive(Clone, Copy, Debug)]
pub struct NodeOverride {
    pub rot: Quat,
    pub pos: Option<V3>,
}
impl NodeOverride {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NodeOverride {
            rot: [0.0; 4],
            pos: None,
        }
    }
}
impl Default for NodeOverride { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PoseFK, src/anim/skeleton.ts:494)
#[derive(Clone, Debug)]
pub struct PoseFK {
    pub rot: Vec<Quat>,
    pub pos: Vec<V3>,
    pub rig: RigId,
    pub sub_lists: Vec<Option<Vec<i32>>>,
    pub pr_scratch: Quat,
}
impl PoseFK {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        PoseFK {
            rot: Vec::new(),
            pos: Vec::new(),
            rig: RigId(0),
            sub_lists: Vec::new(),
            pr_scratch: [0.0, 0.0, 0.0, 1.0],
        }
    }
}
impl Default for PoseFK { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/anim/skeleton.ts:21 POSE_BENCH, src/anim/skeleton.ts:21)
#[derive(Clone, Copy, Debug)]
pub struct AnimSkeletonModPOSEBENCH {
    pub x: Option<crate::sim_sim::SimExports>,
    pub rot: f64,
    pub pos: f64,
    pub args: f64,
    pub pose: f64,
    pub mask: f64,
    pub parent: f64,
    pub order: f64,
    pub rig: Option<RigId>,
    pub asked: f64,
    pub in_place: f64,
    pub owed: f64,
}
impl AnimSkeletonModPOSEBENCH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimSkeletonModPOSEBENCH {
            x: None,
            rot: 0.0,
            pos: 0.0,
            args: 0.0,
            pose: 0.0,
            mask: 0.0,
            parent: 0.0,
            order: 0.0,
            rig: None,
            asked: 0.0,
            in_place: 0.0,
            owed: 0.0,
        }
    }
}
impl Default for AnimSkeletonModPOSEBENCH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/anim/skeleton.ts:33 LAYERS, src/anim/skeleton.ts:33)
#[derive(Clone, Debug)]
pub struct AnimSkeletonModLAYERS {
    pub ran: f64,
    pub roles: Option<RigId>,
    pub names: Option<crate::anim_skeleton::AnimSkeletonModLAYERSNames>,
    pub lists: Vec<Vec<i32>>,
}
impl AnimSkeletonModLAYERS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimSkeletonModLAYERS {
            ran: 0.0,
            roles: None,
            names: None,
            lists: Vec::new(),
        }
    }
}
impl Default for AnimSkeletonModLAYERS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/anim/skeleton.ts:33 AnimSkeletonModLAYERS.names, src/anim/skeleton.ts:33)
#[derive(Clone, Debug)]
pub struct AnimSkeletonModLAYERSNames {
    pub roles: Vec<String>,
    pub lists: Vec<String>,
    pub nums: Vec<String>,
}
impl AnimSkeletonModLAYERSNames {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimSkeletonModLAYERSNames {
            roles: Vec::new(),
            lists: Vec::new(),
            nums: Vec::new(),
        }
    }
}
impl Default for AnimSkeletonModLAYERSNames { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/anim/skeleton.ts:198 STAGE, src/anim/skeleton.ts:198)
#[derive(Clone, Copy, Debug)]
pub struct AnimSkeletonModSTAGE {
    pub st: Option<crate::Opaque>,
    pub lent: f64,
}
impl AnimSkeletonModSTAGE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimSkeletonModSTAGE {
            st: None,
            lent: 0.0,
        }
    }
}
impl Default for AnimSkeletonModSTAGE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LayerCols, src/anim/skeleton.ts:95)
#[derive(Clone, Copy, Debug)]
pub struct LayerCols {
    pub n: f64,
}
impl LayerCols {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LayerCols {
            n: 0.0,
        }
    }
}
impl Default for LayerCols { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/anim/skeleton.ts:480 buildMask(entries), src/anim/skeleton.ts:480)
#[derive(Clone, Debug)]
pub struct BuildMaskEntriesItem {
    pub name: String,
    pub w: f64,
    pub subtree: bool,
}
impl BuildMaskEntriesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BuildMaskEntriesItem {
            name: String::new(),
            w: 0.0,
            subtree: false,
        }
    }
}
impl Default for BuildMaskEntriesItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct AnimSkeletonMod {
    /// (src/anim/skeleton.ts:21 POSE_BENCH)
    pub pose_bench: crate::anim_skeleton::AnimSkeletonModPOSEBENCH,
    /// (src/anim/skeleton.ts:33 LAYERS)
    pub layers: crate::anim_skeleton::AnimSkeletonModLAYERS,
    /// (src/anim/skeleton.ts:198 STAGE)
    pub stage: crate::anim_skeleton::AnimSkeletonModSTAGE,
    /// (src/anim/skeleton.ts:238 CLIP_SIDE)
    pub clip_side: JsMap<ClipId, crate::anim_skeleton::ClipSide>,
    /// (src/anim/skeleton.ts:299 ON_HOW)
    pub on_how: Vec<u8>,
    /// (src/anim/skeleton.ts:299 ON_LO)
    pub on_lo: Vec<i32>,
    /// (src/anim/skeleton.ts:299 ON_F)
    pub on_f: Vec<f64>,
}
impl AnimSkeletonMod {
    pub fn blank() -> Self {
        AnimSkeletonMod {
            pose_bench: crate::anim_skeleton::AnimSkeletonModPOSEBENCH { x: None, rot: 0.0, pos: 0.0, args: 0.0, pose: 0.0, mask: 0.0, parent: 0.0, order: 0.0, rig: None, asked: 0.0, in_place: 0.0, owed: 0.0 },
            layers: crate::anim_skeleton::AnimSkeletonModLAYERS { ran: 0.0, roles: None, names: None, lists: Vec::new() },
            stage: crate::anim_skeleton::AnimSkeletonModSTAGE { st: None, lent: 0.0 },
            clip_side: JsMap::new(),
            on_how: alloc::vec![0; 8],
            on_lo: alloc::vec![0; 8],
            on_f: alloc::vec![0.0; 8],
        }
    }
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
