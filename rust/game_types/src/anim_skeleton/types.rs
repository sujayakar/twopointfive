//! The types of `anim_skeleton`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// A POSE OF A MAN'S: which of his it is
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, ToBytes)]
pub struct Pose { pub at: u32 }

/// THE POSES A MAN KEEPS, one after another. A pose is ten columns of singles (tx ty tz, qx qy qz qw, sx sy sz), a node a row; the rows are whole fours, and those past the last node stay zeros.
#[derive(Clone, Debug, Default, ToBytes)]
pub struct Poses { pub nodes: usize, pub d: Vec<f32> }
/// poses a man keeps at most
const MOST: usize = 16;
impl Poses {
    pub fn of_rig(nodes: usize) -> Poses { Poses { nodes, d: Vec::new() } }
    /// rows of a column
    pub fn rows(&self) -> usize { (self.nodes + 3) & !3 }
    /// singles of a pose
    fn one(&self) -> usize { self.rows() * 10 }
    /// one more pose, all zeros
    pub fn make(&mut self) -> Pose {
        let at = self.d.len() / self.one();
        assert!(at < MOST, "a man is asked to keep one pose more than {MOST}");
        self.d.resize(self.d.len() + self.one(), 0.0);
        Pose { at: at as u32 }
    }
    /// the last one made, let go
    pub fn let_go(&mut self, p: Pose) { let n = self.one(); assert_eq!(self.d.len(), (p.at as usize + 1) * n, "a pose let go that is not the last made"); self.d.truncate(p.at as usize * n); }
    pub fn of(&self, p: Pose) -> &[f32] { &self.d[p.at as usize * self.one()..][..self.one()] }
    pub fn of_mut(&mut self, p: Pose) -> &mut [f32] { let n = self.one(); &mut self.d[p.at as usize * n..][..n] }
    /// pose `from` into pose `to`
    pub fn copy(&mut self, from: Pose, to: Pose) { let n = self.one(); self.d.copy_within(from.at as usize * n..(from.at as usize + 1) * n, to.at as usize * n); }
    /// node i's rotation
    pub fn rot(&self, p: Pose, i: usize) -> Quat { rot_in(self.of(p), i) }
    pub fn set_rot(&mut self, p: Pose, i: usize, q: Quat) {
        let n = self.rows();
        let d = self.of_mut(p);
        for (c, v) in q.into_iter().enumerate() { d[(3 + c) * n + i] = v as f32; }
    }
    /// node i's origin
    pub fn pos(&self, p: Pose, i: usize) -> V3 { pos_in(self.of(p), i) }
}
/// a node's rotation and origin in one pose's singles
pub fn rot_in(d: &[f32], i: usize) -> Quat { let n = d.len() / 10; [d[3 * n + i] as f64, d[4 * n + i] as f64, d[5 * n + i] as f64, d[6 * n + i] as f64] }
pub fn pos_in(d: &[f32], i: usize) -> V3 { let n = d.len() / 10; [d[i] as f64, d[n + i] as f64, d[2 * n + i] as f64] }
/// A BODY'S WORLD OF JOINTS. tmp and local were scratch; ibms and jointViews views of the rig; ownRot was how globalRot's rows were reused without allocating. `world`: where he stood as he was posed last, which with the joints is all a picture skins him by
#[derive(Clone, Debug, ToBytes)]
pub struct SkeletonInstance { pub rig: RigId, pub globals: Vec<crate::v3::m4::Mat4>, pub global_rot: Vec<Quat>, pub world: crate::v3::m4::Mat4 }
impl SkeletonInstance {
    pub fn blank() -> Self { SkeletonInstance { rig: RigId(0), globals: Vec::new(), global_rot: Vec::new(), world: crate::v3::m4::create() } }
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

/// A pose run forward: every node's turn and place in the model.
#[derive(Clone, Debug, Blank)]
pub struct PoseFK {
    pub rot: Vec<Quat>,
    pub pos: Vec<V3>,
}

#[derive(Clone, Debug, Default)]
pub struct BuildMaskEntriesItem {
    pub name: String,
    pub w: f64,
    pub subtree: bool,
}

pub fn ident() -> Quat { [0.0, 0.0, 0.0, 1.0] }

impl Default for SkeletonInstance { fn default() -> Self { Self::blank() } }
