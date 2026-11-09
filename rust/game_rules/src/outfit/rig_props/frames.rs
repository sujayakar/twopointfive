//! src/game/rigProps.ts lines 1 to 419
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Frame from an origin, an 'along' axis and an up hint; falls back when the axis is (anti)parallel to the hint. (rigProps.ts frameFrom)
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

/// `v` turned `ang` about `axis`; a turn under a millionth of a radian leaves it as it is. (rigProps.ts rotAxis)
pub(super) fn rot_axis(v: V3, axis: V3, ang: f64) -> V3 {
    if m::abs(ang) < 1e-6 { v } else { v3::quat::rotate(v3::quat::axis_angle(axis, ang), v) }
}

/// Child frame: the parent moved by adj.off along its axes, turned by adj.rot (degrees about the parent's x, then y, then z). (rigProps.ts applyAdj)
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

/// A new all-zero adjustment. (rigProps.ts ZERO_ADJ)
fn zero_adj() -> crate::rig_props::Adj {
    Adj { off: [0.0, 0.0, 0.0], rot: [0.0, 0.0, 0.0] }
}

/// A character's id as the word on his boxes. (rigProps.ts `c.id & 0xff`)
fn owner_of(g: &mut Game, c: CharId) -> f64 {
    (js::to_i32(g.ch[c].id) & 0xff) as f64
}

/// A bound hostage's wrist tie: one pale strap round both wrists while they are crossed. (rigProps.ts wristTieBoxes)
pub fn wrist_tie_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<AnyBoxId>) {
    let (Some(a), Some(b)) = (g.ch[c_2].bones.get(crate::character::Bone::HandL).copied(), g.ch[c_2].bones.get(crate::character::Bone::HandR).copied()) else { return };
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
    let made = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP {
        c: v3::scale(v3::add(a, b), 0.5),
        h: [l / 2.0 + 0.048, 0.026, 0.045],
        yaw: Some(m::atan2(z[0], z[2])),
        rot: Some(v3::quat::from_basis(x, y, z)),
        albedo: Some([0.6, 0.59, 0.55]),
        flags: Some(crate::rig_props::PROP_FLAGS),
        owner: Some(owner),
        ..Default::default()
    });
    out.push(made.into());
}

/// One stick part as a box laid along its frame's z, after the part's own adjustment in the group frame. (rigProps.ts seg)
pub(super) fn seg(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>, g_2: crate::rig_props::Frame, s: crate::rig_props::Seg, albedo: V3, emissive: Option<V3>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, path: Option<&str>, label: Option<&str>, depth: Option<f64>, name: Option<&str>) {
    let adj = s.adj;
    let emissive = emissive.unwrap_or([0.0, 0.0, 0.0]);
    seg_at(g, out, g_2, s, adj, albedo, emissive, meta, path, label, depth, name);
}

/// seg() with the part's adjustment handed in. (rigProps.ts segAt)
pub(super) fn seg_at(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>, g_2: crate::rig_props::Frame, s: crate::rig_props::Seg, adj: Option<crate::rig_props::Adj>, albedo: V3, emissive: V3, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, path: Option<&str>, label: Option<&str>, depth: Option<f64>, name: Option<&str>) {
    let f = if adj.is_some() { apply_adj(g_2, adj) } else { g_2 };
    let dir = f.z;
    let mid = v3::mad(f.o, dir, (s.l0 + s.l1) / 2.0);
    let len = m::max(0.001, (s.l1 - s.l0) / 2.0);
    if let Some(meta) = meta {
        meta.push(RigNodeMeta {  });
    }
    let lit = emissive[0] + emissive[1] + emissive[2] > 0.0;
    let yaw = m::atan2(dir[0], dir[2]);
    let rot = v3::quat::from_basis(f.x, f.y, f.z);
    let stuff = crate::scene_stuff::paint_of(g, Some(&albedo[..])).and_then(|p| p.stuff);
    let owner = g.m.rig_props.cur_owner;
    let made = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP {
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
    out.push(made.into());
}

/// Flattened head 'forward'; the body's heading when the head looks nearly straight up or down. (rigProps.ts headFlatForward)
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

/// A part's box in a group's own axes, with nothing lit. (rigProps.ts gogglesBoxes `boxOf(at(…), …, [0, 0, 0], …)`)
fn goggle_part(g: &mut Game, c: V3, h: V3, yaw: f64, albedo: V3, flags: f64, owner: f64, rot: Quat) -> FrameBoxId {
    crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP {
        c,
        h,
        yaw: Some(yaw),
        albedo: Some(albedo),
        emissive: Some([0.0, 0.0, 0.0]),
        flags: Some(flags),
        owner: Some(owner),
        rot: Some(rot),
        ..Default::default()
    })
}

/// The goggles: two tubes on a bridge, down over the eyes or flipped up on the forehead, `k` part way between. (rigProps.ts gogglesBoxes)
pub fn goggles_boxes(g: &mut crate::game::Game, c_2: CharId, nv: bool, out: &mut Vec<AnyBoxId>, g_2: crate::rig_props::RigGoggles, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, k: Option<f64>) {
    let mut meta = meta;
    let Some(head) = g.ch[c_2].bones.get(crate::character::Bone::Head).copied() else { return };
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
    if let Some(meta) = meta.as_deref_mut() {
        meta.push(RigNodeMeta {  });
    }
    let owner = owner_of(g, c_2);
    g.m.rig_props.cur_owner = owner;
    let t = g_2.tubes;
    let col = crate::rig_props::col();
    let at = |f: Frame, p: V3| -> V3 { v3::mad(v3::mad(v3::mad(f.o, f.x, p[0]), f.y, p[1]), f.z, p[2]) };
    let hs = apply_adj(cl, g_2.housing_adj);
    let rot_h = v3::quat::from_basis(hs.x, hs.y, hs.z);
    let yaw_h = m::atan2(hs.z[0], hs.z[2]);
    if let Some(meta) = meta.as_deref_mut() {
        meta.push(RigNodeMeta {  });
    }
    for (part_at, part_half) in [(t.bridge.at, t.bridge.half), (t.arm.at, t.arm.half)] {
        let made = goggle_part(g, at(hs, part_at), v3::copy(part_half), yaw_h, col.goggles_housing, crate::rig_props::PROP_FLAGS, owner, rot_h);
        out.push(made.into());
    }
    let ln = apply_adj(cl, g_2.lens_adj);
    let rot_l = v3::quat::from_basis(ln.x, ln.y, ln.z);
    let yaw_l = m::atan2(ln.z[0], ln.z[2]);
    if let Some(meta) = meta.as_deref_mut() {
        meta.push(RigNodeMeta {  });
    }
    for sx in [-t.side, t.side] {
        let tube = goggle_part(g, at(ln, [sx, t.down, (t.front - t.back) / 2.0]), [t.half, t.half, (t.front + t.back) / 2.0], yaw_l, col.goggles_housing, crate::rig_props::PROP_FLAGS, owner, rot_l);
        out.push(tube.into());
        let glass = goggle_part(g, at(ln, [sx, t.down, t.front + t.glass]), [t.half * 0.8, t.half * 0.8, t.glass], yaw_l, col.dark_glass, crate::rig_props::GLOW_FLAGS, owner, rot_l);
        out.push(glass.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The doubles of the TypeScript under EXACT.on (bun), o x y z of each frame.
    const TS: &[(&str, &[u64])] = &[
        ("ff1", &[0x3ff4000000000000, 0x3fe0000000000000, 0xc00e000000000000, 0x3febb7cad58b0cfa, 0x0000000000000000, 0xbfdffb73e2b422a8, 0x3fd846ea53ce662e, 0x3fe4d4fd401ca14e, 0x3fe50a428ce6144b, 0x3fd4d207698e3c71, 0xbfe84a5dfb25f12e, 0x3fe20b5bc1e1abda]),
        ("ff2", &[0x3ff4000000000000, 0x3fe0000000000000, 0xc00e000000000000, 0x3febed80c09bac70, 0x3f8784a2515aac30, 0xbfdf3c27940c6ca4, 0x3fd74b9ce3ddc0f9, 0x3fe4d428d4525c18, 0x3fe551a2523dabb3, 0x3fd4d207698e3c71, 0xbfe84a5dfb25f12e, 0x3fe20b5bc1e1abda]),
        ("ff_par", &[0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000]),
        ("ff_anti", &[0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xbff0000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xbff0000000000000, 0x0000000000000000]),
        ("aj_none", &[0x3ff4000000000000, 0x3fe0000000000000, 0xc00e000000000000, 0x3febed80c09bac70, 0x3f8784a2515aac30, 0xbfdf3c27940c6ca4, 0x3fd74b9ce3ddc0f9, 0x3fe4d428d4525c18, 0x3fe551a2523dabb3, 0x3fd4d207698e3c71, 0xbfe84a5dfb25f12e, 0x3fe20b5bc1e1abda]),
        ("aj_grip", &[0x3ff41ed7fbba6e72, 0x3fdc923839e0b355, 0xc00de700fa9d2a46, 0x3febed80c09bac70, 0x3f8784a2515aac30, 0xbfdf3c27940c6ca4, 0x3fc9349a2339995a, 0xbfed89bf183397d4, 0x3fd52589dc18fa6e, 0xbfdc96cd697e2239, 0xbfd89b353421690e, 0xbfe9d9fa8c1cd78e]),
        ("aj_all", &[0x3ff4c8bc9d06020e, 0x3fe369b61ad18f90, 0xc00dc976d4d6a04e, 0xbfc22d13daa9f4f0, 0x3fe4bff7d334aa41, 0x3fe7eee162b75e33, 0xbfef38f68a8e33d6, 0xbfcc09af65cf0a68, 0x3f73175aa6d27c00, 0x3fc55b4c93867d6e, 0xbfe75498c23cc024, 0x3fe53db6feba0592]),
        ("aj_tiny", &[0x3ff4000000000000, 0x3fe0000000000000, 0xc00e000000000000, 0x3febed80c09bac70, 0x3f8784a2515aac30, 0xbfdf3c27940c6ca4, 0x3fd74b9ce3ddc0f9, 0x3fe4d428d4525c18, 0x3fe551a2523dabb3, 0x3fd4d207698e3c71, 0xbfe84a5dfb25f12e, 0x3fe20b5bc1e1abda]),
        ("aj_zero", &[0x3ff4000000000000, 0x3fe0000000000000, 0xc00e000000000000, 0x3febed80c09bac70, 0x3f8784a2515aac30, 0xbfdf3c27940c6ca4, 0x3fd74b9ce3ddc0f9, 0x3fe4d428d4525c18, 0x3fe551a2523dabb3, 0x3fd4d207698e3c71, 0xbfe84a5dfb25f12e, 0x3fe20b5bc1e1abda]),
        ("rot", &[0x3fe5b24e9fcf66d9, 0x3f9813704b834e60, 0xbfe62a0e3b5ead70]),
    ];

    fn want(name: &str) -> &'static [u64] {
        TS.iter().find(|c| c.0 == name).unwrap().1
    }
    fn same_frame(name: &str, f: Frame) {
        let got: Vec<u64> = [f.o, f.x, f.y, f.z].iter().flatten().map(|x| x.to_bits()).collect();
        assert_eq!(got.as_slice(), want(name), "{name}");
    }
    const O: V3 = [1.25, 0.5, -3.75];
    const A1: V3 = [0.3, -0.7, 0.52];
    const UP: V3 = [0.1, 0.9, 0.2];

    #[test]
    fn frame_from_has_the_typescripts_bits() {
        same_frame("ff1", frame_from(O, A1, None));
        same_frame("ff2", frame_from(O, A1, Some(UP)));
        same_frame("ff_par", frame_from([0.0, 1.0, 0.0], [0.0, 2.0, 0.0], None));
        same_frame("ff_anti", frame_from([0.0, 1.0, 0.0], [0.0, -2.0, 0.0], Some([0.0, 1.0, 0.0])));
    }

    #[test]
    fn apply_adj_has_the_typescripts_bits() {
        let base = frame_from(O, A1, Some(UP));
        same_frame("aj_none", apply_adj(base, None));
        same_frame("aj_grip", apply_adj(base, rig().pistol.grip.adj));
        same_frame("aj_all", apply_adj(base, Some(Adj { off: [0.031, 0.105, -0.05], rot: [-8.6, 6.7, 119.0] })));
        same_frame("aj_tiny", apply_adj(base, Some(Adj { off: [0.0, 0.0, 0.0], rot: [1e-5, 0.0, 0.0] })));
        same_frame("aj_zero", apply_adj(base, Some(zero_adj())));
    }

    #[test]
    fn rot_axis_turns_or_leaves() {
        let v: V3 = [0.2, 0.3, -0.9];
        let got: Vec<u64> = rot_axis(v, [0.0, 0.6, 0.8], -0.75).iter().map(|x| x.to_bits()).collect();
        assert_eq!(got.as_slice(), want("rot"));
        assert_eq!(rot_axis(v, [0.0, 0.6, 0.8], 5e-7), v);
    }
}
