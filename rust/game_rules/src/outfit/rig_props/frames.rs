use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Frame from an origin, an 'along' axis and an up hint; falls back when the axis is (anti)parallel to the hint.
pub(super) fn frame_from(o: V3, along: V3, up_hint: Option<V3>) -> crate::rig_props::Frame {
    let up_hint = up_hint.unwrap_or([0.0, 1.0, 0.0]);
    let z = v3::normalize(along);
    let mut y = v3::sub(up_hint, v3::scale(z, v3::dot(up_hint, z)));
    if v3::len(y) < 1e-3 {
        y = v3::sub([1.0, 0.0, 0.0], v3::scale(z, z[0]));
    }
    y = v3::normalize(y);
    let x = v3::cross(y, z);
    Frame { o: v3::copy(o), x, y, z }
}

/// `v` turned `ang` about `axis`; a turn under a millionth of a radian leaves it as it is.
pub(super) fn rot_axis(v: V3, axis: V3, ang: f64) -> V3 {
    if m::abs(ang) < 1e-6 { v } else { v3::quat::rotate(v3::quat::axis_angle(axis, ang), v) }
}

/// Child frame: the parent moved by adj.off along its axes, turned by adj.rot (degrees about the parent's x, then y, then z).
pub(super) fn apply_adj(f: crate::rig_props::Frame, adj: Option<crate::rig_props::Adj>) -> crate::rig_props::Frame {
    let adj = match adj {
        Some(a) => a,
        None => return Frame { o: v3::copy(f.o), x: f.x, y: f.y, z: f.z },
    };
    let (fo, fx, fy, fz, a0, a1, a2) = (f.o, f.x, f.y, f.z, adj.off[0], adj.off[1], adj.off[2]);
    let o: V3 = [
        ((fo[0] + fx[0] * a0) + fy[0] * a1) + fz[0] * a2,
        ((fo[1] + fx[1] * a0) + fy[1] * a1) + fz[1] * a2,
        ((fo[2] + fx[2] * a0) + fy[2] * a1) + fz[2] * a2,
    ];
    let d = m::PI / 180.0;
    let (mut x, mut y, mut z) = (f.x, f.y, f.z);
    // the negated tests are the TypeScript's: a NaN angle does turn
    let ax = adj.rot[0] * d;
    if !(m::abs(ax) < 1e-6) {
        let q = v3::quat::axis_angle(f.x, ax);
        x = v3::quat::rotate(q, x);
        y = v3::quat::rotate(q, y);
        z = v3::quat::rotate(q, z);
    }
    let ay = adj.rot[1] * d;
    if !(m::abs(ay) < 1e-6) {
        let q = v3::quat::axis_angle(f.y, ay);
        x = v3::quat::rotate(q, x);
        y = v3::quat::rotate(q, y);
        z = v3::quat::rotate(q, z);
    }
    let az = adj.rot[2] * d;
    if !(m::abs(az) < 1e-6) {
        let q = v3::quat::axis_angle(f.z, az);
        x = v3::quat::rotate(q, x);
        y = v3::quat::rotate(q, y);
        z = v3::quat::rotate(q, z);
    }
    Frame { o, x, y, z }
}

/// A character's id as the word on his boxes.
pub(crate) fn owner_of(g: &Game, c: CharId) -> f64 {
    (js::to_i32(g.ch[c].id) & 0xff) as f64
}

/// A bound hostage's wrist tie: one pale strap round both wrists while they are crossed.
pub fn wrist_tie_boxes(g: &crate::game::Game, c_2: CharId, out: &mut Carried) {
    let (Some(a), Some(b)) = (g.ch[c_2].bone(crate::character::Bone::HandL), g.ch[c_2].bone(crate::character::Bone::HandR)) else { return };
    let d = v3::sub(b, a);
    let l = v3::len(d);
    if l > 0.16 {
        return;
    }
    let x: V3 = if l > 1e-4 { v3::scale(d, 1.0 / l) } else { [1.0, 0.0, 0.0] };
    let mut y: V3 = if m::abs(x[1]) > 0.9 { [0.0, 0.0, 1.0] } else { [0.0, 1.0, 0.0] };
    let z = v3::normalize(v3::cross(x, y));
    y = v3::cross(z, x);
    let owner = owner_of(g, c_2);
    out.put(&crate::scene_boxes::MakeBoxP {
        c: v3::scale(v3::add(a, b), 0.5),
        h: [l / 2.0 + 0.048, 0.026, 0.045],
        yaw: Some(m::atan2(z[0], z[2])),
        rot: Some(v3::quat::from_basis(x, y, z)),
        albedo: Some([0.6, 0.59, 0.55]),
        flags: Some(crate::rig_props::PROP_FLAGS),
        owner: Some(owner),
        ..Default::default()
    });
}

/// One stick part as a box laid along its frame's z, after the part's own adjustment in the group frame.
pub(super) fn seg(g: &crate::game::Game, out: &mut Carried, g_2: crate::rig_props::Frame, s: crate::rig_props::Seg, albedo: V3, emissive: Option<V3>, name: Option<&str>) {
    let adj = s.adj;
    let emissive = emissive.unwrap_or([0.0, 0.0, 0.0]);
    seg_at(g, out, g_2, s, adj, albedo, emissive, name);
}

/// seg() with the part's adjustment handed in.
pub(super) fn seg_at(g: &crate::game::Game, out: &mut Carried, g_2: crate::rig_props::Frame, s: crate::rig_props::Seg, adj: Option<crate::rig_props::Adj>, albedo: V3, emissive: V3, name: Option<&str>) {
    let f = if adj.is_some() { apply_adj(g_2, adj) } else { g_2 };
    let dir = f.z;
    let mid = v3::mad(f.o, dir, (s.l0 + s.l1) / 2.0);
    let len = m::max(0.001, (s.l1 - s.l0) / 2.0);
    let lit = emissive[0] + emissive[1] + emissive[2] > 0.0;
    let yaw = m::atan2(dir[0], dir[2]);
    let rot = v3::quat::from_basis(f.x, f.y, f.z);
    let stuff = crate::scene_stuff::paint_of(g, Some(&albedo[..])).and_then(|p| p.stuff);
    let owner = out.owner;
    out.put(&crate::scene_boxes::MakeBoxP {
        c: mid,
        h: [s.thick, s.tall, len],
        yaw: Some(yaw),
        albedo: Some(albedo),
        emissive: Some([emissive[0], emissive[1], emissive[2]]),
        flags: Some(if lit { crate::rig_props::GLOW_FLAGS } else { crate::rig_props::PROP_FLAGS }),
        owner: Some(owner),
        rot: Some(rot),
        name: name.map(String::from),
        stuff,
        ..Default::default()
    });
}

/// Flattened head 'forward'; the body's heading when the head looks nearly straight up or down.
fn head_flat_forward(g: &crate::game::Game, c_2: CharId) -> V3 {
    let hd = crate::character::character_head_dir(g, c_2);
    let flat = m::hyp2(hd[0], hd[2]);
    if flat > 0.2 {
        return [hd[0] / flat, 0.0, hd[2] / flat];
    }
    let fw = crate::character::character_forward(g, c_2);
    let fl = js::or(m::hyp2(fw[0], fw[2]), 1.0);
    [fw[0] / fl, 0.0, fw[2] / fl]
}

/// The goggles: two tubes on a bridge, down over the eyes or flipped up on the forehead, `k` part way between.
pub fn goggles_boxes(g: &crate::game::Game, c_2: CharId, nv: bool, out: &mut Carried, g_2: crate::rig_props::RigGoggles, k: Option<f64>) {
    let Some(head) = g.ch[c_2].bone(crate::character::Bone::Head) else { return };
    let f = head_flat_forward(g, c_2);
    let (mut fwd, mut up, mut side): (V3, V3, V3) = (f, [0.0, 1.0, 0.0], [f[2], 0.0, -f[0]]);
    if g_2.follow_pitch >= 0.5 {
        let q = crate::character::character_head_rot(g, c_2);
        fwd = v3::quat::rotate(q, [0.0, 0.0, 1.0]);
        up = v3::quat::rotate(q, [0.0, 1.0, 0.0]);
        side = v3::quat::rotate(q, [1.0, 0.0, 0.0]);
    }
    let head_f = Frame { o: v3::copy(head), x: side, y: up, z: fwd };
    let (p_fwd, p_up, p_adj) = if nv { (g_2.on.fwd, g_2.on.up, g_2.on_adj) } else { (g_2.off.fwd, g_2.off.up, g_2.off_adj) };
    let mut cl = apply_adj(Frame { o: v3::mad(v3::mad(head, fwd, p_fwd), up, p_up), ..head_f }, p_adj);
    if let Some(k) = k {
        if k > 0.0 && k < 1.0 {
            let a = apply_adj(Frame { o: v3::mad(v3::mad(head, fwd, g_2.off.fwd), up, g_2.off.up), ..head_f }, g_2.off_adj);
            let b = apply_adj(Frame { o: v3::mad(v3::mad(head, fwd, g_2.on.fwd), up, g_2.on.up), ..head_f }, g_2.on_adj);
            let z = v3::normalize(v3::lerp3(a.z, b.z, k));
            let x = v3::normalize(v3::cross(v3::lerp3(a.y, b.y, k), z));
            let y = v3::cross(z, x);
            cl = Frame { o: v3::mad(v3::lerp3(a.o, b.o, k), fwd, crate::rig_props::NV_SWING * m::sin(m::PI * k)), x, y, z };
        }
    }
    let owner = owner_of(g, c_2);
    out.owner = owner;
    let t = g_2.tubes;
    let col = crate::rig_props::col();
    let at = |f: Frame, p: V3| -> V3 { v3::mad(v3::mad(v3::mad(f.o, f.x, p[0]), f.y, p[1]), f.z, p[2]) };
    let part = |out: &mut Carried, c: V3, h: V3, yaw: f64, albedo: V3, flags: f64, rot: Quat| out.put(&crate::scene_boxes::MakeBoxP { c, h, yaw: Some(yaw), albedo: Some(albedo), emissive: Some([0.0, 0.0, 0.0]), flags: Some(flags), owner: Some(owner), rot: Some(rot), ..Default::default() });
    let hs = apply_adj(cl, g_2.housing_adj);
    let rot_h = v3::quat::from_basis(hs.x, hs.y, hs.z);
    let yaw_h = m::atan2(hs.z[0], hs.z[2]);
    for (part_at, part_half) in [(t.bridge.at, t.bridge.half), (t.arm.at, t.arm.half)] {
        part(out, at(hs, part_at), part_half, yaw_h, col.goggles_housing, crate::rig_props::PROP_FLAGS, rot_h);
    }
    let ln = apply_adj(cl, g_2.lens_adj);
    let rot_l = v3::quat::from_basis(ln.x, ln.y, ln.z);
    let yaw_l = m::atan2(ln.z[0], ln.z[2]);
    for sx in [-t.side, t.side] {
        part(out, at(ln, [sx, t.down, (t.front - t.back) / 2.0]), [t.half, t.half, (t.front + t.back) / 2.0], yaw_l, col.goggles_housing, crate::rig_props::PROP_FLAGS, rot_l);
        part(out, at(ln, [sx, t.down, t.front + t.glass]), [t.half * 0.8, t.half * 0.8, t.glass], yaw_l, col.dark_glass, crate::rig_props::GLOW_FLAGS, rot_l);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: V3, b: V3) -> bool { v3::len(v3::sub(a, b)) < 1e-9 }
    /// three unit axes at right angles, right-handed: anything else shears a gun or a hand off its model
    fn is_a_frame(f: Frame) {
        for a in [f.x, f.y, f.z] { assert!((v3::len(a) - 1.0).abs() < 1e-9, "{:?}", f); }
        assert!(v3::dot(f.x, f.y).abs() < 1e-9 && v3::dot(f.y, f.z).abs() < 1e-9 && v3::dot(f.x, f.z).abs() < 1e-9, "{:?}", f);
        assert!(near(v3::cross(f.y, f.z), f.x), "{:?}", f);
    }
    const O: V3 = [1.25, 0.5, -3.75];
    const A1: V3 = [0.3, -0.7, 0.52];
    const UP: V3 = [0.1, 0.9, 0.2];

    /// A FAILURE MEANS: a gun or a limb is drawn skewed, or rolled off the way it is meant to lean.
    #[test]
    fn a_frame_lies_along_its_axis_and_leans_to_its_up() {
        for (along, up) in [(A1, None), (A1, Some(UP))] {
            let f = frame_from(O, along, up);
            is_a_frame(f);
            assert_eq!(f.o, O);
            assert!(near(f.z, v3::normalize(along)));
            assert!(v3::dot(f.y, up.unwrap_or([0.0, 1.0, 0.0])) > 0.0);
        }
    }

    /// A FAILURE MEANS: a gun held straight up or down turns to NaN and vanishes.
    #[test]
    fn an_axis_along_its_up_still_makes_a_frame() {
        is_a_frame(frame_from([0.0, 1.0, 0.0], [0.0, 2.0, 0.0], None));
        is_a_frame(frame_from([0.0, 1.0, 0.0], [0.0, -2.0, 0.0], Some([0.0, 1.0, 0.0])));
    }

    /// A FAILURE MEANS: a tuned adjustment moves the model along the world and not along the thing it sits on, or turns it the wrong way, or shears it.
    #[test]
    fn an_adjustment_moves_and_turns_a_frame_in_its_own_axes() {
        let base = frame_from(O, A1, Some(UP));
        let moved = apply_adj(base, Some(Adj { off: [0.1, 0.2, -0.3], rot: [0.0, 0.0, 0.0] }));
        assert!(near(moved.o, v3::add(v3::add(v3::add(O, v3::scale(base.x, 0.1)), v3::scale(base.y, 0.2)), v3::scale(base.z, -0.3))));
        assert!(near(moved.x, base.x) && near(moved.z, base.z));
        let roll = apply_adj(base, Some(Adj { off: [0.0; 3], rot: [0.0, 0.0, 90.0] }));
        assert!(near(roll.z, base.z) && near(roll.x, base.y));
        let pitch = apply_adj(base, Some(Adj { off: [0.0; 3], rot: [90.0, 0.0, 0.0] }));
        assert!(near(pitch.x, base.x) && near(pitch.y, base.z));
        let yaw = apply_adj(base, Some(Adj { off: [0.0; 3], rot: [0.0, 90.0, 0.0] }));
        assert!(near(yaw.y, base.y) && near(yaw.z, base.x));
        is_a_frame(apply_adj(base, Some(Adj { off: [0.031, 0.105, -0.05], rot: [-8.6, 6.7, 119.0] })));
    }

    /// A FAILURE MEANS: a model with no adjustment moves at all.
    #[test]
    fn no_adjustment_and_a_nothing_of_one_leave_a_frame_as_it_is() {
        let base = frame_from(O, A1, Some(UP));
        for adj in [None, Some(crate::rig_props::adj_none()), Some(Adj { off: [0.0; 3], rot: [1e-5, 0.0, 0.0] })] {
            let f = apply_adj(base, adj);
            assert_eq!((f.o, f.x, f.y, f.z), (base.o, base.x, base.y, base.z));
        }
    }

    /// A FAILURE MEANS: a turn changes a limb's length, does not leave its own axis be, or runs the wrong way.
    #[test]
    fn a_turn_keeps_a_length_and_leaves_its_axis_alone() {
        let v: V3 = [0.2, 0.3, -0.9];
        let axis: V3 = [0.0, 0.6, 0.8];
        assert!((v3::len(rot_axis(v, axis, -0.75)) - v3::len(v)).abs() < 1e-9);
        assert!(near(rot_axis(axis, axis, -0.75), axis));
        assert!(near(rot_axis([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], m::PI / 2.0), [0.0, 1.0, 0.0]));
        assert_eq!(rot_axis(v, axis, 5e-7), v);
    }
}
