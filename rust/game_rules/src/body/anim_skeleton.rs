pub use ::game_types::anim_skeleton::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use ::sim::single::S;
use alloc::{string::String, vec, vec::Vec};

/// A layer over his animation: his poses and his rig lent it with his animator.
pub(super) fn anim_layer(g: &mut crate::game::Game, c: CharId, layer: impl FnOnce(&mut ::sim::skeleton::Body, &mut crate::character::Anim, &crate::knobs::Knobs)) {
    let Game { art, ch, k, bones, .. } = g;
    let crate::character::Character { anim, poses, .. } = &mut ch[c];
    let rig = &art.rig[anim.rig];
    debug_assert!(poses.nodes == rig.nodes.len() && rig.parent.len() == rig.nodes.len());
    bones.rot.resize(rig.nodes.len(), v3::quat::ident());
    bones.pos.resize(rig.nodes.len(), [0.0; 3]);
    let rig = ::sim::skeleton::Rig { parent: &rig.parent, order: &rig.order, roles: &rig.roles };
    layer(&mut ::sim::skeleton::Body::new(S::of_mut(&mut poses.d), fk_of(bones), rig, &art.clip), anim, k);
}
fn fk_of(fk: &mut crate::anim_skeleton::PoseFK) -> ::sim::skeleton::Fk<'_> { ::sim::skeleton::Fk { rot: fk.rot.as_flattened_mut(), pos: fk.pos.as_flattened_mut() } }

// ---------------------------------------------------------------- a pose

/// `d` set to the rig's rest pose.
pub(super) fn rest_pose(rig: &crate::anim_gltf::GltfCharacter, d: &mut [f32]) {
    let n_ = d.len() / 10;
    for (i, nd) in rig.nodes.iter().enumerate() {
        for c in 0..3 {
            d[c * n_ + i] = nd.t[c] as f32;
            d[(7 + c) * n_ + i] = nd.s[c] as f32;
        }
        for c in 0..4 {
            d[(3 + c) * n_ + i] = nd.r[c] as f32;
        }
    }
}

/// A clip at time t (seconds, wrapped if `loop_`) into a pose of his (only nodes with tracks are written).
pub(super) fn sample_clip(g: &mut crate::game::Game, c: CharId, clip: ClipId, t: f64, loop_: bool, out: crate::anim_skeleton::Pose) {
    sample_into(&g.art.clip[clip], t, loop_, g.ch[c].poses.of_mut(out));
}
pub(super) fn sample_into(clip: &crate::anim_gltf::Clip, t: f64, loop_: bool, d: &mut [f32]) { ::sim::clips::sample(clip, t, loop_, S::of_mut(d)) }

/// out = lerp(a, b, w * mask[node]); no mask means uniform weight. `a` None means out is a.
pub(super) fn blend_pose(poses: &mut crate::anim_skeleton::Poses, a: Option<crate::anim_skeleton::Pose>, b: crate::anim_skeleton::Pose, w: f64, mask: Option<&[f32]>, out: crate::anim_skeleton::Pose) {
    let one = ::sim::skeleton::singles(poses.nodes);
    let at = |p: crate::anim_skeleton::Pose| p.at as usize * one;
    // (a mask has a weight a node, and the rows are whole fours)
    let mut whole = [0.0f32; ::sim::skeleton::MAX_NODES];
    if let Some(k) = mask { whole[..k.len()].copy_from_slice(k); }
    ::sim::skeleton::blend(S::of_mut(&mut poses.d), at(a.unwrap_or(out)), at(b), w, mask.map(|_| S::of(&whole)), at(out), poses.nodes);
}

// ---------------------------------------------------------------- a body's world of joints

/// Every node's global matrix and rotation at rest values, and room for the skinning matrices.
pub(super) fn skeleton_instance_new(rig: &crate::anim_gltf::GltfCharacter, ch: RigId) -> crate::anim_skeleton::SkeletonInstance {
    SkeletonInstance {
        rig: ch,
        globals: rig.nodes.iter().map(|_| v3::m4::create()).collect(),
        global_rot: rig.nodes.iter().map(|_| v3::quat::ident()).collect(),
        world: v3::m4::create(),
    }
}

/// A body posed: run forward, where it stands.
pub(super) fn skeleton_instance_update(rig: &crate::anim_gltf::GltfCharacter, this: &mut crate::anim_skeleton::SkeletonInstance, d: &[f32], world: &crate::v3::m4::Mat4, twists: Option<&[crate::anim_skeleton::TwistSpec]>, overrides: Option<&[Option<crate::anim_skeleton::NodeOverride>]>) {
    skeleton_instance_forward(rig, this, d, twists, overrides);
    this.world = *world;
}

/// Model-space matrices of all nodes from a local pose, with model-space twists and per-node overrides.
pub(super) fn skeleton_instance_forward(rig: &crate::anim_gltf::GltfCharacter, this: &mut crate::anim_skeleton::SkeletonInstance, d: &[f32], twists: Option<&[crate::anim_skeleton::TwistSpec]>, overrides: Option<&[Option<crate::anim_skeleton::NodeOverride>]>) {
    let n_ = d.len() / 10;
    // several layers may push a twist for the same node: they accumulate
    let twists = twists.unwrap_or(&[]);
    let turned = twists.iter().fold(0u128, |w, t| w | 1 << (t.node as u32 & 127));
    let twist_of = |i: usize| -> Option<TwistSpec> {
        if turned >> (i & 127) & 1 == 0 { return None; }
        twists.iter().filter(|t| t.node as usize == i).copied().reduce(|e, t| TwistSpec { node: t.node, yaw: e.yaw + t.yaw, pitch: e.pitch + t.pitch })
    };
    for &oi in &rig.order {
        let i = oi as usize;
        let parent = rig.nodes[i].parent;
        let (rx, ry, rz, rw) = (d[3 * n_ + i] as f64, d[4 * n_ + i] as f64, d[5 * n_ + i] as f64, d[6 * n_ + i] as f64);
        let (tx, ty, tz, sx, sy, sz) = (d[i] as f64, d[n_ + i] as f64, d[2 * n_ + i] as f64, d[7 * n_ + i] as f64, d[8 * n_ + i] as f64, d[9 * n_ + i] as f64);
        if let Some(Some(ov)) = overrides.and_then(|o| o.get(i)) {
            // driven from outside: global rotation as given, origin as given or where the parent puts it
            let t: V3 = [tx, ty, tz];
            let p = match ov.pos {
                Some(p) => p,
                None => {
                    if parent >= 0.0 {
                        v3::m4::transform_point(&this.globals[parent as usize], t)
                    } else {
                        t
                    }
                }
            };
            this.global_rot[i] = ov.rot;
            this.globals[i] = v3::m4::from_trss(p[0], p[1], p[2], ov.rot[0], ov.rot[1], ov.rot[2], ov.rot[3], sx, sy, sz);
            continue;
        }
        let parent_rot = if parent >= 0.0 { this.global_rot[parent as usize] } else { ident() };
        let (ax, ay, az, aw) = (parent_rot[0], parent_rot[1], parent_rot[2], parent_rot[3]);
        let tw = twist_of(i);
        let mut gr: Quat = [aw * rx + ax * rw + ay * rz - az * ry, aw * ry - ax * rz + ay * rw + az * rx, aw * rz + ax * ry - ay * rx + az * rw, aw * rw - ax * rx - ay * ry - az * rz];
        let mut r: Option<Quat> = None;
        if let Some(tw) = tw {
            // model-space yaw about +Y then pitch about the yawed model X axis, applied to this subtree
            let mut extra = v3::quat::yaw(tw.yaw);
            if tw.pitch != 0.0 {
                let right = v3::quat::rotate(extra, [1.0, 0.0, 0.0]);
                extra = v3::quat::mul(v3::quat::axis_angle(right, tw.pitch), extra);
            }
            gr = v3::quat::normalize(v3::quat::mul(extra, gr));
            // back to a local rotation consistent with the new global: r = parentRot^-1 * gr
            r = Some(v3::quat::mul(v3::quat::conj(parent_rot), gr));
        }
        this.global_rot[i] = gr;
        let local = match r {
            Some(r) => v3::m4::from_trss(tx, ty, tz, r[0], r[1], r[2], r[3], sx, sy, sz),
            None => v3::m4::from_trss(tx, ty, tz, rx, ry, rz, rw, sx, sy, sz),
        };
        this.globals[i] = if parent >= 0.0 { v3::m4::mul(&this.globals[parent as usize], &local) } else { v3::m4::copy(&local) };
    }
}

/// The skinning matrices of a body as it was posed last (world · global · inverse bind, a joint sixteen numbers): what is drawn of it, and no part of the run.
pub fn skeleton_instance_skin(g: &crate::game::Game, this: &crate::anim_skeleton::SkeletonInstance, to: &mut [f32]) {
    let rig = &g.art.rig[this.rig];
    let world = v3::m4::wide(&this.world);
    for (j, out) in to.chunks_exact_mut(16).enumerate().take(rig.skin.joints.len()) { out.copy_from_slice(&joint_skin(rig, this, &world, j)); }
}
/// …of one joint of the skin's list.
pub fn skeleton_instance_joint(g: &crate::game::Game, this: &crate::anim_skeleton::SkeletonInstance, j: usize) -> [f32; 16] {
    joint_skin(&g.art.rig[this.rig], this, &v3::m4::wide(&this.world), j)
}
fn joint_skin(rig: &crate::anim_gltf::GltfCharacter, this: &crate::anim_skeleton::SkeletonInstance, world: &[f64; 16], j: usize) -> [f32; 16] {
    let bound = v3::m4::mul(&this.globals[rig.skin.joints[j] as usize], rig.skin.inverse_bind[j * 16..].first_chunk().unwrap());
    v3::m4::mul_wide(world, &bound)
}

/// Model-space origin of a node.
pub(super) fn skeleton_instance_node_pos(this: &crate::anim_skeleton::SkeletonInstance, node: f64) -> V3 {
    let g = &this.globals[node as usize];
    [g[12] as f64, g[13] as f64, g[14] as f64]
}

/// Rewrite the rotations of `pose` so that plain FK reproduces the globals of the last update: parent⁻¹ · global per node.
pub(super) fn skeleton_instance_bake_local_rotations(rig: &crate::anim_gltf::GltfCharacter, this: &crate::anim_skeleton::SkeletonInstance, d: &mut [f32]) {
    let n_ = d.len() / 10;
    for &oi in &rig.order {
        let i = oi as usize;
        let parent = rig.nodes[i].parent;
        let r = if parent >= 0.0 { v3::quat::mul(v3::quat::conj(this.global_rot[parent as usize]), this.global_rot[i]) } else { this.global_rot[i] };
        d[3 * n_ + i] = r[0] as f32;
        d[4 * n_ + i] = r[1] as f32;
        d[5 * n_ + i] = r[2] as f32;
        d[6 * n_ + i] = r[3] as f32;
    }
}

/// buildMask's setSub: the weight on a node and all under it.
fn set_sub(nodes: &[crate::anim_gltf::GltfNode], mask: &mut [f32], i: usize, w: f64) {
    mask[i] = w as f32;
    for &c in &nodes[i].children {
        set_sub(nodes, mask, c as usize, w);
    }
}

/// A per-node mask: a weight for listed node names and (optionally) all their descendants.
pub(super) fn build_mask(rig: &crate::anim_gltf::GltfCharacter, entries: &[crate::anim_skeleton::BuildMaskEntriesItem]) -> Vec<f32> {
    let mut mask = vec![0.0f32; rig.nodes.len()];
    for e in entries {
        let Some(&i) = rig.node_by_name.get(e.name.clone()) else {
            continue;
        };
        if e.subtree {
            set_sub(&rig.nodes, &mut mask, i as usize, e.w);
        } else {
            mask[i as usize] = e.w as f32;
        }
    }
    mask
}

// ---------------------------------------------------------------- forward kinematics of a local pose

/// A local pose of a rig run forward: every node's turn and place in the model.
pub(super) fn fk_run(rig: &crate::anim_gltf::GltfCharacter, fk: &mut crate::anim_skeleton::PoseFK, d: &[f32]) {
    let n = rig.nodes.len();
    fk.rot.resize(n, v3::quat::ident());
    fk.pos.resize(n, [0.0; 3]);
    ::sim::skeleton::fk_all(&rig.parent, &rig.order, S::of(d), fk.rot.as_flattened_mut(), fk.pos.as_flattened_mut());
}
/// …of a man, from a pose of his
pub(super) fn pose_run(g: &crate::game::Game, c: CharId, pose: crate::anim_skeleton::Pose) -> crate::anim_skeleton::PoseFK {
    let mut fk = crate::anim_skeleton::PoseFK::blank();
    fk_run(&g.art.rig[g.ch[c].anim.rig], &mut fk, g.ch[c].poses.of(pose));
    fk
}

/// A node of a local pose in the model: its turn and its place, its parents run down to it and no other node.
pub(super) fn pose_node(rig: &crate::anim_gltf::GltfCharacter, d: &[f32], i: usize) -> (Quat, V3) {
    let parent = rig.nodes[i].parent;
    let up = (parent >= 0.0).then(|| pose_node(rig, d, parent as usize));
    fk_node(d, d.len() / 10, i, up)
}
/// …of a pose of a man's
pub(super) fn man_node(g: &crate::game::Game, c: CharId, pose: crate::anim_skeleton::Pose, i: usize) -> (Quat, V3) {
    pose_node(&g.art.rig[g.ch[c].anim.rig], g.ch[c].poses.of(pose), i)
}

/// A node hangs off its parent: his place is the parent's and his offset turned by the parent's turn, his turn is the parent's and his own.
#[inline]
pub(super) fn fk_node(d: &[f32], n: usize, i: usize, up: Option<(Quat, V3)>) -> (Quat, V3) {
    let (rx, ry, rz, rw) = (d[3 * n + i] as f64, d[4 * n + i] as f64, d[5 * n + i] as f64, d[6 * n + i] as f64);
    let (v0, v1, v2) = (d[i] as f64, d[n + i] as f64, d[2 * n + i] as f64);
    let Some((pr, pp)) = up else { return ([rx, ry, rz, rw], [v0, v1, v2]) };
    let (qx, qy, qz, qw) = (pr[0], pr[1], pr[2], pr[3]);
    let (tx, ty, tz) = (2.0 * (qy * v2 - qz * v1), 2.0 * (qz * v0 - qx * v2), 2.0 * (qx * v1 - qy * v0));
    let (t0, t1, t2) = (v0 + qw * tx + (qy * tz - qz * ty), v1 + qw * ty + (qz * tx - qx * tz), v2 + qw * tz + (qx * ty - qy * tx));
    (
        [qw * rx + qx * rw + qy * rz - qz * ry, qw * ry - qx * rz + qy * rw + qz * rx, qw * rz + qx * ry - qy * rx + qz * rw, qw * rw - qx * rx - qy * ry - qz * rz],
        [pp[0] + t0, pp[1] + t1, pp[2] + t2],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROWS: usize = 8;
    /// singles of a pose of five nodes (eight rows a column), each its own
    fn cols() -> Vec<f32> { (0..80).map(|i| if i % 8 < 5 { (((i * 37 + 11) % 101) as f64 / 101.0 - 0.5) as f32 } else { 0.0 }).collect() }
    fn folded(a: &[f32]) -> Vec<u32> { a.iter().map(|v| v.to_bits()).collect() }
    /// a pose of five nodes standing at (x, i, 0) with its scale s and the turn `yaw` about the vertical
    fn stood(x: f32, yaw: f64, s: f32) -> Vec<f32> {
        let q = v3::quat::yaw(yaw);
        let mut d = vec![0.0f32; 10 * ROWS];
        for i in 0..5 {
            let col = [x, i as f32, 0.0, q[0] as f32, q[1] as f32, q[2] as f32, q[3] as f32, s, s, s];
            for c in 0..10 { d[c * ROWS + i] = col[c]; }
        }
        d
    }
    fn yaw_at(d: &[f32], i: usize) -> f64 {
        2.0 * m::atan2(d[4 * ROWS + i] as f64, d[6 * ROWS + i] as f64)
    }
    fn near(a: f64, b: f64, what: &str) { assert!(m::abs(a - b) < 1e-5, "{what}: {a} against {b}"); }
    /// a man's two poses and a third, `a` blended into `b` by `w` written to the pose of number `out`: that pose
    fn blended(a: &[f32], b: &[f32], w: f64, mask: Option<&[f32]>, out: u32) -> Vec<f32> {
        let mut p = Poses::of_rig(5);
        let (pa, pb, _) = (p.make(), p.make(), p.make());
        p.of_mut(pa).copy_from_slice(a);
        p.of_mut(pb).copy_from_slice(b);
        blend_pose(&mut p, Some(pa), pb, w, mask, Pose { at: out });
        p.of(Pose { at: out }).to_vec()
    }

    /// A move going into the next: the whole of one at nought, the whole of the other at one, and on the way a real pose.
    #[test]
    fn a_blend_goes_from_one_pose_to_the_other_by_its_weight() {
        let (a, b) = (stood(1.0, 0.2, 1.0), stood(3.0, 1.0, 2.0));
        for (w, want) in [(0.0, &a), (-1.0, &a), (1.0, &b), (2.0, &b)] {
            assert_eq!(folded(&blended(&a, &b, w, None, 2)), folded(want), "weight {w}");
        }
        let o = &blended(&a, &b, 0.25, None, 2);
        near(o[2] as f64, 1.5, "a quarter of the way between the places");
        near(o[7 * ROWS + 2] as f64, 1.25, "and the scales");
        assert!(m::abs(yaw_at(o, 2) - 0.4) < 0.01, "and about that of the turn: {}", yaw_at(o, 2));
        let l: f64 = (3..7).map(|c| { let x = o[c * ROWS + 2] as f64; x * x }).sum();
        near(l, 1.0, "a turn stays a turn");
    }

    /// A rotation and its negative are one rotation: a blend between them stands still, it does not go round the long way or through nothing.
    #[test]
    fn a_blend_takes_the_short_way_round() {
        let a = stood(0.0, 0.3, 1.0);
        let mut b = a.clone();
        for i in 0..5 { for c in 3..7 { b[c * ROWS + i] = -b[c * ROWS + i]; } }
        near(yaw_at(&blended(&a, &b, 0.5, None, 2), 1), 0.3, "half way between a turn and itself");
    }

    /// A layer on part of the body: each node takes the weight times his share, and a node with no share stays the first pose.
    #[test]
    fn a_mask_gives_each_node_his_share_of_a_blend() {
        let (a, b) = (stood(0.0, 0.0, 1.0), stood(2.0, 0.0, 1.0));
        let mask = [0.0f32, 0.25, 0.5, 1.0, 1.5];
        let d = blended(&a, &b, 0.8, Some(&mask), 2);
        let got: Vec<f64> = (0..5).map(|i| d[i] as f64).collect();
        for (i, want) in [0.0, 0.4, 0.8, 1.6, 2.0].iter().enumerate() { near(got[i], *want, "a node's share"); }
        // a blend written over its own first pose comes out the same, and with no share leaves the node as it stood
        assert_eq!(folded(&blended(&a, &b, 0.8, Some(&mask), 0)), folded(&d));
    }

    use crate::anim_gltf::Path;
    fn track(node: usize, path: Path, times: &[f32], values: &[f32], step: bool) -> crate::anim_gltf::Track {
        crate::anim_gltf::Track { node, path, times: times.to_vec(), values: values.to_vec(), step, line: 0 }
    }
    /// A clip of 1.25 s: a move and a half turn on two nodes, a stepped scale, and a node with one key.
    fn clip_of() -> crate::anim_gltf::Clip {
        let s = core::f32::consts::FRAC_1_SQRT_2;
        let tracks = vec![
            track(0, Path::Move, &[0.0, 0.5, 1.0], &[0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0], false),
            track(1, Path::Turn, &[0.0, 1.0], &[0.0, 0.0, 0.0, 1.0, 0.0, s, 0.0, s], false),
            track(2, Path::Scale, &[0.25, 0.75], &[1.0, 1.0, 1.0, 2.0, 2.0, 2.0], true),
            track(3, Path::Turn, &[0.2], &[0.0, 0.0, 0.0, 1.0], false),
        ];
        crate::anim_gltf::Clip::new(String::from("c"), 1.25, tracks)
    }

    #[test]
    fn a_sample_goes_between_a_clips_keys_and_holds_at_its_ends() {
        let clip = clip_of();
        let read = |t: f64, looped: bool| { let mut o = cols(); sample_into(&clip, t, looped, &mut o); o };
        let o = read(0.25, false);
        near(o[0] as f64, 0.5, "half way between two keys: half way between their places");
        near(o[ROWS] as f64, 1.0, "in each column");
        near(read(0.5, false)[0] as f64, 1.0, "on a key: the key");
        near(yaw_at(&read(0.5, false), 1), m::PI / 4.0, "half of a quarter turn");
        near(read(1.2, false)[0] as f64, 3.0, "past the last key: the last");
        near(read(-1.0, false)[0] as f64, 0.0, "before the first key, and before the clip: the first");
        near(read(9.0, false)[0] as f64, 3.0, "past the clip's end: its end");
        // a stepped track holds its key until the next
        near(read(0.74, false)[7 * ROWS + 2] as f64, 1.0, "a step before its next key");
        near(read(0.76, false)[7 * ROWS + 2] as f64, 2.0, "a step past it");
        // a loop comes round
        near(read(0.25 + 1.25, true)[0] as f64, 0.5, "once round");
        near(read(0.25 - 1.25, true)[0] as f64, 0.5, "back round");
        // what no track names stands as it stood
        let was = cols();
        let o = read(0.25, false);
        assert_eq!(o[4].to_bits(), was[4].to_bits(), "node 4 has no track");
        assert_eq!(o[ROWS + 4].to_bits(), was[ROWS + 4].to_bits());
    }

    fn rig() -> crate::anim_gltf::GltfCharacter {
        let mut ch = crate::anim_gltf::GltfCharacter::default();
        for parent in [-1.0, 0.0, 1.0, 1.0, 3.0] {
            ch.nodes.push(crate::anim_gltf::GltfNode { parent, t: [0.0; 3], r: [0.0, 0.0, 0.0, 1.0], s: [1.0; 3], ..crate::anim_gltf::GltfNode::default() });
        }
        ch.parent = ch.nodes.iter().map(|n| n.parent as i32).collect();
        ch.order = vec![0, 1, 2, 3, 4];
        ch.skin.joints = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        ch.skin.inverse_bind = (0..80).map(|i| if (i % 16) % 5 == 0 { 1.0 } else { 0.0 }).collect();
        ch
    }
    /// the rig's five nodes each a metre along +z from their parent, the given yaw on each, the root at (1, 2, 3)
    fn chain(yaws: [f64; 5]) -> Vec<f32> {
        let mut d = stood(0.0, 0.0, 1.0);
        for i in 0..5 {
            let q = v3::quat::yaw(yaws[i]);
            let t = if i == 0 { [1.0, 2.0, 3.0] } else { [0.0, 0.0, 1.0] };
            for c in 0..3 { d[c * ROWS + i] = t[c]; }
            for c in 0..4 { d[(3 + c) * ROWS + i] = q[c] as f32; }
        }
        d
    }
    fn at_near(a: V3, b: V3, what: &str) { assert!(v3::dist(a, b) < 1e-5, "{what}: {a:?} against {b:?}"); }

    /// Each joint hangs off its parent, by the parent's whole turn so far. The rig is 0 - 1 - (2, 3 - 4).
    #[test]
    fn a_pose_run_forward_hangs_each_joint_off_its_parent() {
        let ch = rig();
        let mut fk = PoseFK { rot: vec![v3::quat::ident(); 5], pos: vec![[0.0; 3]; 5], ..PoseFK::blank() };
        let quarter = m::PI / 2.0;
        fk_run(&ch, &mut fk, &chain([quarter, 0.0, 0.0, -quarter, 0.0]));
        at_near(fk.pos[0], [1.0, 2.0, 3.0], "the root");
        at_near(fk.pos[1], [2.0, 2.0, 3.0], "a metre along the way the root faces");
        at_near(fk.pos[2], [3.0, 2.0, 3.0], "and another");
        at_near(fk.pos[3], [3.0, 2.0, 3.0], "a child of the same parent, the same place");
        at_near(fk.pos[4], [3.0, 2.0, 4.0], "and a joint that turned back takes his child with him");
    }

    #[test]
    fn a_node_hangs_off_its_parent_by_the_parents_turn() {
        let yaw = |a: f64| v3::quat::axis_angle([0.0, 1.0, 0.0], a);
        let (n, i) = (2usize, 1usize);
        let mut d = [0f32; 14];
        d[n + i] = 0.5;
        d[2 * n + i] = 1.0;
        let own = yaw(m::PI / 2.0);
        for c in 0..4 { d[(3 + c) * n + i] = own[c] as f32; }
        let (q, p) = fk_node(&d, n, i, Some((yaw(m::PI / 2.0), [1.0, 2.0, 3.0])));
        assert!(v3::dist(p, [2.0, 2.5, 3.0]) < 1e-6, "a metre ahead of a parent who faces his left: {p:?}");
        let half = yaw(m::PI);
        let dot = q[0] * half[0] + q[1] * half[1] + q[2] * half[2] + q[3] * half[3];
        assert!(m::abs(dot) > 1.0 - 1e-6, "two quarter turns make a half: {q:?}");
    }

    /// What a body draws with: plain, a turn laid on a subtree, a joint driven from outside, and a pose made again from what was drawn.
    #[test]
    fn a_body_is_posed_from_its_joints_and_a_bake_keeps_what_it_drew() {
        let ch = rig();
        let mut si = SkeletonInstance { rig: RigId(0), globals: vec![v3::m4::create(); 5], global_rot: vec![v3::quat::ident(); 5], world: v3::m4::create() };
        let w = v3::m4::from_trss(10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0);
        let quarter = m::PI / 2.0;
        let mut p = chain([0.0; 5]);

        // plain: where a run forward puts them, the skinning matrices in the world
        skeleton_instance_update(&ch, &mut si, &p, &w, None, None);
        at_near(skeleton_instance_node_pos(&si, 2.0), [1.0, 2.0, 5.0], "two joints along");
        near(joint_skin(&ch, &si, &v3::m4::wide(&si.world), 2)[12] as f64, 11.0, "the skin of that joint stands in the world");

        // a twist turns that joint and all under it, about the vertical, and two on one joint add
        let twists = [TwistSpec { node: 1.0, yaw: quarter * 0.5, pitch: 0.0 }, TwistSpec { node: 1.0, yaw: quarter * 0.5, pitch: 0.0 }];
        skeleton_instance_update(&ch, &mut si, &p, &w, Some(&twists), None);
        at_near(skeleton_instance_node_pos(&si, 1.0), [1.0, 2.0, 4.0], "the twisted joint stays where it is");
        at_near(skeleton_instance_node_pos(&si, 2.0), [2.0, 2.0, 4.0], "and what hangs off it swings round");
        at_near(skeleton_instance_node_pos(&si, 4.0), [3.0, 2.0, 4.0], "all the way down");
        at_near(skeleton_instance_node_pos(&si, 0.0), [1.0, 2.0, 3.0], "what is above it does not");

        // driven from outside: the joint's rotation and place as given, or where its parent puts it; what hangs off it follows
        let q = v3::quat::yaw(quarter);
        let mut ov = [None; 5];
        ov[3] = Some(NodeOverride { rot: [q[0], q[1], q[2], q[3]], pos: Some([7.0, 7.0, 7.0]) });
        skeleton_instance_update(&ch, &mut si, &p, &w, None, Some(&ov));
        at_near(skeleton_instance_node_pos(&si, 3.0), [7.0, 7.0, 7.0], "a place from outside");
        at_near(skeleton_instance_node_pos(&si, 4.0), [8.0, 7.0, 7.0], "his child follows");
        ov[3] = Some(NodeOverride { rot: [q[0], q[1], q[2], q[3]], pos: None });
        skeleton_instance_update(&ch, &mut si, &p, &w, None, Some(&ov));
        at_near(skeleton_instance_node_pos(&si, 3.0), [1.0, 2.0, 5.0], "no place from outside: where his parent has him");

        // a bake writes the pose so that a plain pose run forward draws what the twists and the drive drew
        skeleton_instance_update(&ch, &mut si, &p, &w, Some(&twists), Some(&ov));
        let drawn: Vec<V3> = (0..5).map(|i| skeleton_instance_node_pos(&si, i as f64)).collect();
        skeleton_instance_bake_local_rotations(&ch, &si, &mut p);
        skeleton_instance_update(&ch, &mut si, &p, &w, None, None);
        for i in 0..5 { at_near(skeleton_instance_node_pos(&si, i as f64), drawn[i], "the bake"); }
    }

    /// Each pose of a man's is ten columns of its own: writing one leaves the others as they stand, and the last made is let go without a trace.
    #[test]
    fn a_mans_poses_stand_one_after_another() {
        let mut m = Poses::of_rig(5);
        let (p, q) = (m.make(), m.make());
        m.set_rot(p, 2, [0.5, 0.25, -0.5, 1.0]);
        m.of_mut(p)[8 + 2] = 3.0;
        assert_eq!((m.rot(p, 2), m.pos(p, 2)), ([0.5, 0.25, -0.5, 1.0], [0.0, 3.0, 0.0]));
        assert_eq!(m.of(p).iter().filter(|v| **v != 0.0).count(), 5);
        assert!(m.of(q).iter().all(|v| *v == 0.0));
        m.copy(q, p);
        assert_eq!(m.rot(q, 2), [0.5, 0.25, -0.5, 1.0]);
        let r = m.make();
        m.let_go(r);
        assert_eq!(m.d.len(), 160);
    }

    #[test]
    fn rest_pose_fills_the_rigs_nodes() {
        let mut ch = rig();
        ch.nodes[1].t = [1.0, 2.0, 3.0];
        ch.nodes[1].r = [0.5, 0.5, 0.5, 0.5];
        ch.nodes[1].s = [4.0, 5.0, 6.0];
        let mut d = vec![0.0f32; 80];
        rest_pose(&ch, &mut d);
        assert_eq!((pos_in(&d, 1), rot_in(&d, 1), d[8 * 9 + 1]), ([1.0, 2.0, 3.0], [0.5; 4], 6.0));
    }

    /// Tracks with the same times are on one line, single for single: a nought and its negative are two.
    #[test]
    fn a_clips_lines_are_its_sets_of_times() {
        let t = |a: &[f32]| track(0, Path::Move, a, &[0.0; 6], false);
        let clip = crate::anim_gltf::Clip::new(String::new(), 1.0, vec![t(&[0.0, 1.0]), t(&[0.0, 0.5]), t(&[0.0, 1.0]), t(&[-0.0, 1.0])]);
        assert_eq!(clip.tracks.iter().map(|x| x.line).collect::<Vec<_>>(), [0, 1, 0, 2]);
    }

    #[test]
    fn a_mask_reaches_a_subtree() {
        let mut ch = rig();
        ch.nodes[1].children = vec![2.0, 3.0];
        ch.nodes[3].children = vec![4.0];
        ch.node_by_name.set(String::from("arm"), 3.0);
        ch.node_by_name.set(String::from("root"), 0.0);
        let e = |name: &str, w: f64, subtree: bool| BuildMaskEntriesItem { name: String::from(name), w, subtree };
        assert_eq!(build_mask(&ch, &[e("arm", 0.5, true), e("root", 1.0, false), e("none", 1.0, true)]), vec![1.0, 0.0, 0.0, 0.5, 0.5]);
    }
}
