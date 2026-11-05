//! src/game/rigProps.ts lines 1 to 419
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The tree's two value types, as `D` the side written and `S` the side read.
type D = crate::rig_props::AssignTreeDstValue;
type S = crate::rig_props::AssignTreeSrcValue;

/// Deep copy of a plain tree: a number by value, an array in place when the lengths agree, an object key by key. (rigProps.ts assignTree)
pub fn assign_tree(dst: &mut JsMap<String, crate::rig_props::AssignTreeDstValue>, src: &JsMap<String, crate::rig_props::AssignTreeSrcValue>) {
    for (k, s) in src.iter() {
        match s {
            S::Num(x) => {
                dst.set(k, D::Num(*x));
            }
            S::Array(s) => {
                if let Some(D::Array(d)) = dst.get_mut(k.clone()) {
                    if d.len() == s.len() {
                        d.copy_from_slice(s);
                        continue;
                    }
                }
                dst.set(k, D::Array(s.clone()));
            }
            S::Arm2(s) => {
                if !matches!(dst.get(k.clone()), Some(D::Arm2(_))) {
                    dst.set(k.clone(), D::Arm2(JsMap::new()));
                }
                if let Some(D::Arm2(d)) = dst.get_mut(k) {
                    assign_tree(d, s);
                }
            }
        }
    }
}

/// A deep copy: `Rig` holds nothing by reference, so the value is one. (rigProps.ts cloneRig)
pub fn clone_rig(r: crate::rig_props::Rig) -> crate::rig_props::Rig {
    r
}

/// Deep in-place copy; a key the source lacks stays as it is, so an adjustment `src` does not have survives in `dst`. (rigProps.ts assignRig)
pub fn assign_rig(dst: &mut crate::rig_props::Rig, src: crate::rig_props::Rig) {
    let mut was = *dst;
    *dst = src;
    for (now, old) in adj_slots(dst).into_iter().zip(adj_slots(&mut was)) {
        if now.is_none() {
            *now = *old;
        }
    }
}

/// Drop every free adjustment. (rigProps.ts clearAdjs)
pub fn clear_adjs(rig: crate::rig_props::Rig) {
    let mut cleared = rig;
    for slot in adj_slots(&mut cleared) {
        *slot = None;
    }
    seam_clear_adjs_write_back(cleared);
}

/// rigProps.ts:280 writes the caller's rig in place; the skeleton's parameter is a copy.
fn seam_clear_adjs_write_back(cleared: crate::rig_props::Rig) {
    todo!("needs: clear_adjs(rig: &mut Rig): the rig is taken by value, so the cleared copy has nowhere to go")
}

/// Every free adjustment of a rig: what the TypeScript finds by the key's name (/adj$/i).
fn adj_slots(r: &mut crate::rig_props::Rig) -> [&mut Option<crate::rig_props::Adj>; 61] {
    [
        &mut r.goggles.on_adj, &mut r.goggles.off_adj, &mut r.goggles.housing_adj, &mut r.goggles.lens_adj,
        &mut r.pistol.slide.adj, &mut r.pistol.suppressor.adj, &mut r.pistol.grip.adj, &mut r.pistol.rail.adj, &mut r.pistol.lens.adj, &mut r.pistol.adj,
        &mut r.torch.body.adj, &mut r.torch.lens.adj, &mut r.torch.adj,
        &mut r.guard_pistol.slide.adj, &mut r.guard_pistol.grip.adj, &mut r.guard_pistol.lens.adj, &mut r.guard_pistol.adj,
        &mut r.knife.blade.adj, &mut r.knife.edge.adj, &mut r.knife.guard.adj, &mut r.knife.handle.adj, &mut r.knife.adj,
        &mut r.sheath.body.adj,
        &mut r.can.body.adj, &mut r.can.band.adj, &mut r.can.adj,
        &mut r.holster.adj,
        &mut r.carbine.receiver.adj, &mut r.carbine.handguard.adj, &mut r.carbine.barrel.adj, &mut r.carbine.muzzle.adj, &mut r.carbine.mag.adj, &mut r.carbine.mag2.adj,
        &mut r.carbine.grip.adj, &mut r.carbine.stock_top.adj, &mut r.carbine.stock_low.adj, &mut r.carbine.light.adj, &mut r.carbine.lens.adj, &mut r.carbine.adj,
        &mut r.sc20k.hull.adj, &mut r.sc20k.butt.adj, &mut r.sc20k.mag.adj, &mut r.sc20k.grip.adj, &mut r.sc20k.fore.adj, &mut r.sc20k.suppressor.adj,
        &mut r.sc20k.rail.adj, &mut r.sc20k.optic.adj, &mut r.sc20k.foregrip.adj, &mut r.sc20k.light.adj, &mut r.sc20k.lens.adj, &mut r.sc20k.slung.adj, &mut r.sc20k.adj,
        &mut r.m4.hull.adj, &mut r.m4.butt.adj, &mut r.m4.mag.adj, &mut r.m4.fore.adj, &mut r.m4.rail.adj, &mut r.m4.optic.adj, &mut r.m4.tube.adj, &mut r.m4.peq.adj, &mut r.m4.gl.adj,
    ]
}

/// The rig as the TypeScript source of its table, laid out as the table is written, numbers rounded to 3 decimals. (rigProps.ts rigSource)
pub fn rig_source(rig: Option<crate::rig_props::Rig>) -> String {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let mut lines: Vec<String> = vec![String::from("export const RIG: Rig = {")];
    for (group, members) in rig_tree(&rig) {
        lines.push(alloc::format!("  {}: {{", group));
        if let S::Arm2(members) = &members {
            for (k, v) in members.iter().filter(|(k, v)| keep_row(k, v)) {
                lines.push(alloc::format!("    {}: {},", k, inline_source(v)));
            }
        }
        lines.push(String::from("  },"));
    }
    lines.push(String::from("};"));
    lines.join("\n")
}

/// `+n.toFixed(3)` as a string: the exact decimal digits, a tie going up in magnitude. (rigProps.ts rigSource num)
fn num_source(n: f64) -> String {
    if !n.is_finite() {
        return String::from(if n.is_nan() { "NaN" } else if n < 0.0 { "-Infinity" } else { "Infinity" });
    }
    let exact = alloc::format!("{:.1100}", m::abs(n));
    let dot = exact.find('.').unwrap();
    let mut digits: Vec<u8> = exact.as_bytes()[..dot + 4].to_vec();
    if exact.as_bytes()[dot + 4] >= b'5' {
        let mut at = digits.len();
        loop {
            if at == 0 {
                digits.insert(0, b'1');
                break;
            }
            at -= 1;
            if digits[at] == b'.' {
                continue;
            }
            if digits[at] == b'9' {
                digits[at] = b'0';
            } else {
                digits[at] += 1;
                break;
            }
        }
    }
    let rounded: f64 = core::str::from_utf8(&digits).unwrap().parse().unwrap();
    if rounded == 0.0 {
        return String::from("0");   // (-0 prints as 0)
    }
    alloc::format!("{}{}", if n < 0.0 { "-" } else { "" }, rounded)
}

/// An all-zero adjustment is the same as none: left out of the source. (rigProps.ts rigSource zeroAdj, keep)
fn keep_row(k: &str, v: &S) -> bool {
    let named_adj = k.len() >= 3 && k[k.len() - 3..].eq_ignore_ascii_case("adj");
    if !named_adj {
        return true;
    }
    let S::Arm2(o) = v else { return true };
    let (Some(S::Array(off)), Some(S::Array(rot))) = (o.get(String::from("off")), o.get(String::from("rot"))) else { return true };
    !off.iter().chain(rot.iter()).all(|x| m::abs(*x) < 5e-4)
}

/// A value on one line. (rigProps.ts rigSource inline)
fn inline_source(v: &S) -> String {
    match v {
        S::Num(n) => num_source(*n),
        S::Array(a) => {
            let items: Vec<String> = a.iter().map(|x| num_source(*x)).collect();
            alloc::format!("[{}]", items.join(", "))
        }
        S::Arm2(o) => {
            let items: Vec<String> = o.iter().filter(|(k, x)| keep_row(k, x)).map(|(k, x)| alloc::format!("{}: {}", k, inline_source(x))).collect();
            alloc::format!("{{ {} }}", items.join(", "))
        }
    }
}

fn tree_num(x: f64) -> S {
    S::Num(x)
}

fn tree_v3(x: V3) -> S {
    S::Array(x.to_vec())
}

/// An object from its rows, in the order given. An adjustment that is absent has no row.
fn tree_obj(rows: Vec<(&str, Option<S>)>) -> S {
    let mut o = JsMap::new();
    for (k, v) in rows {
        if let Some(v) = v {
            o.set(String::from(k), v);
        }
    }
    S::Arm2(o)
}

fn tree_adj(a: Option<crate::rig_props::Adj>) -> Option<S> {
    a.map(|a| tree_obj(vec![("off", Some(tree_v3(a.off))), ("rot", Some(tree_v3(a.rot)))]))
}

fn tree_seg(s: crate::rig_props::Seg) -> Option<S> {
    Some(tree_obj(vec![("l0", Some(tree_num(s.l0))), ("l1", Some(tree_num(s.l1))), ("thick", Some(tree_num(s.thick))), ("tall", Some(tree_num(s.tall))), ("adj", tree_adj(s.adj))]))
}

/// The rig's groups in the order of the written table (the TypeScript walks the object's own keys; a struct has none).
fn rig_tree(r: &crate::rig_props::Rig) -> Vec<(&'static str, S)> {
    let n = |x: f64| Some(tree_num(x));
    let a = |x: V3| Some(tree_v3(x));
    let (gg, p, t, gp, kn, sh, cn, hl, cb, sc, m4) = (r.goggles, r.pistol, r.torch, r.guard_pistol, r.knife, r.sheath, r.can, r.holster, r.carbine, r.sc20k, r.m4);
    vec![
        ("goggles", tree_obj(vec![
            ("on", Some(tree_obj(vec![("fwd", n(gg.on.fwd)), ("up", n(gg.on.up))]))),
            ("off", Some(tree_obj(vec![("fwd", n(gg.off.fwd)), ("up", n(gg.off.up))]))),
            ("lensSide", n(gg.lens_side)),
            ("lensDown", n(gg.lens_down)),
            ("lensUp", n(gg.lens_up)),
            ("lensHalf", n(gg.lens_half)),
            ("lensThick", n(gg.lens_thick)),
            ("housing", Some(tree_obj(vec![("back", n(gg.housing.back)), ("half", a(gg.housing.half))]))),
            ("glowOn", a(gg.glow_on)),
            ("glowOff", a(gg.glow_off)),
            ("tubes", Some(tree_obj(vec![
                ("side", n(gg.tubes.side)), ("down", n(gg.tubes.down)), ("half", n(gg.tubes.half)), ("back", n(gg.tubes.back)), ("front", n(gg.tubes.front)), ("glass", n(gg.tubes.glass)),
                ("bridge", Some(tree_obj(vec![("at", a(gg.tubes.bridge.at)), ("half", a(gg.tubes.bridge.half))]))),
                ("arm", Some(tree_obj(vec![("at", a(gg.tubes.arm.at)), ("half", a(gg.tubes.arm.half))]))),
            ]))),
            ("followPitch", n(gg.follow_pitch)),
            ("onAdj", tree_adj(gg.on_adj)),
            ("offAdj", tree_adj(gg.off_adj)),
            ("housingAdj", tree_adj(gg.housing_adj)),
            ("lensAdj", tree_adj(gg.lens_adj)),
        ])),
        ("pistol", tree_obj(vec![
            ("slide", tree_seg(p.slide)),
            ("grip", tree_seg(p.grip)),
            ("suppressor", tree_seg(p.suppressor)),
            ("rail", Some(tree_obj(vec![("l0", n(p.rail.l0)), ("l1", n(p.rail.l1)), ("thick", n(p.rail.thick)), ("tall", n(p.rail.tall)), ("under", n(p.rail.under)), ("adj", tree_adj(p.rail.adj))]))),
            ("lens", Some(tree_obj(vec![("l0", n(p.lens.l0)), ("l1", n(p.lens.l1)), ("thick", n(p.lens.thick)), ("tall", n(p.lens.tall)), ("glow", a(p.lens.glow)), ("adj", tree_adj(p.lens.adj))]))),
            ("lightUnder", n(p.light_under)),
            ("lightAhead", n(p.light_ahead)),
            ("adj", tree_adj(p.adj)),
        ])),
        ("torch", tree_obj(vec![
            ("body", tree_seg(t.body)),
            ("lens", tree_seg(t.lens)),
            ("lensGlow", n(t.lens_glow)),
            ("lightAhead", n(t.light_ahead)),
            ("adj", tree_adj(t.adj)),
        ])),
        ("guardPistol", tree_obj(vec![
            ("slide", tree_seg(gp.slide)),
            ("grip", tree_seg(gp.grip)),
            ("lens", tree_seg(gp.lens)),
            ("lightAhead", n(gp.light_ahead)),
            ("adj", tree_adj(gp.adj)),
        ])),
        ("knife", tree_obj(vec![
            ("blade", tree_seg(kn.blade)),
            ("edge", tree_seg(kn.edge)),
            ("guard", tree_seg(kn.guard)),
            ("handle", tree_seg(kn.handle)),
            ("grip", Some(tree_obj(vec![("fwd", n(kn.grip.fwd)), ("palm", n(kn.grip.palm)), ("side", n(kn.grip.side))]))),
            ("adj", tree_adj(kn.adj)),
        ])),
        ("sheath", tree_obj(vec![
            ("at", a(sh.at)),
            ("along", a(sh.along)),
            ("out", a(sh.out)),
            ("body", tree_seg(sh.body)),
        ])),
        ("can", tree_obj(vec![
            ("sink", n(cn.sink)),
            ("tilt", n(cn.tilt)),
            ("body", tree_seg(cn.body)),
            ("band", tree_seg(cn.band)),
            ("adj", tree_adj(cn.adj)),
        ])),
        ("holster", tree_obj(vec![("down", n(hl.down)), ("out", n(hl.out)), ("fwd", n(hl.fwd)), ("adj", tree_adj(hl.adj))])),
        ("carbine", tree_obj(vec![
            ("receiver", tree_seg(cb.receiver)),
            ("handguard", tree_seg(cb.handguard)),
            ("barrel", tree_seg(cb.barrel)),
            ("muzzle", tree_seg(cb.muzzle)),
            ("mag", tree_seg(cb.mag)),
            ("mag2", tree_seg(cb.mag2)),
            ("grip", tree_seg(cb.grip)),
            ("stockTop", tree_seg(cb.stock_top)),
            ("stockLow", tree_seg(cb.stock_low)),
            ("light", tree_seg(cb.light)),
            ("lens", tree_seg(cb.lens)),
            ("lensGlow", n(cb.lens_glow)),
            ("lightAhead", n(cb.light_ahead)),
            ("gripL", Some(tree_obj(vec![("off", a(cb.grip_l.off)), ("fingers", a(cb.grip_l.fingers)), ("back", a(cb.grip_l.back))]))),
            ("adj", tree_adj(cb.adj)),
        ])),
        ("sc20k", tree_obj(vec![
            ("hull", tree_seg(sc.hull)),
            ("butt", tree_seg(sc.butt)),
            ("mag", tree_seg(sc.mag)),
            ("grip", tree_seg(sc.grip)),
            ("fore", tree_seg(sc.fore)),
            ("suppressor", tree_seg(sc.suppressor)),
            ("rail", tree_seg(sc.rail)),
            ("optic", tree_seg(sc.optic)),
            ("foregrip", tree_seg(sc.foregrip)),
            ("light", tree_seg(sc.light)),
            ("lens", tree_seg(sc.lens)),
            ("lensGlow", n(sc.lens_glow)),
            ("lightAhead", n(sc.light_ahead)),
            ("gripL", Some(tree_obj(vec![("off", a(sc.grip_l.off)), ("fingers", a(sc.grip_l.fingers)), ("back", a(sc.grip_l.back))]))),
            ("slung", Some(tree_obj(vec![("off", a(sc.slung.off)), ("bore", a(sc.slung.bore)), ("roll", n(sc.slung.roll)), ("follow", n(sc.slung.follow)), ("adj", tree_adj(sc.slung.adj))]))),
            ("adj", tree_adj(sc.adj)),
        ])),
        ("m4", tree_obj(vec![
            ("hull", tree_seg(m4.hull)),
            ("butt", tree_seg(m4.butt)),
            ("mag", tree_seg(m4.mag)),
            ("fore", tree_seg(m4.fore)),
            ("rail", tree_seg(m4.rail)),
            ("optic", tree_seg(m4.optic)),
            ("tube", tree_seg(m4.tube)),
            ("peq", tree_seg(m4.peq)),
            ("gl", tree_seg(m4.gl)),
        ])),
    ]
}

/// Frame from an origin, an 'along' axis and an up hint; falls back when the axis is (anti)parallel to the hint. (rigProps.ts frameFrom)
pub fn frame_from(o: V3, along: V3, up_hint: Option<V3>) -> crate::rig_props::Frame {
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
pub fn rot_axis(v: V3, axis: V3, ang: f64) -> V3 {
    if m::abs(ang) < 1e-6 { v } else { v3::quat::rotate(v3::quat::axis_angle(axis, ang), v) }
}

/// Child frame: the parent moved by adj.off along its axes, turned by adj.rot (degrees about the parent's x, then y, then z). (rigProps.ts applyAdj)
pub fn apply_adj(f: crate::rig_props::Frame, adj: Option<crate::rig_props::Adj>) -> crate::rig_props::Frame {
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
pub fn zero_adj() -> crate::rig_props::Adj {
    Adj { off: [0.0, 0.0, 0.0], rot: [0.0, 0.0, 0.0] }
}

/// The viewer's lazy maker of an Adj on its owner. (rigProps.ts adjOf)
pub fn adj_of<T: Clone + core::fmt::Debug>(owner: &mut T, key: crate::Opaque) -> crate::FnField {
    seam_adj_of(owner, key)
}

/// rigProps.ts:330: a closure that keeps its owner and writes a field by name.
fn seam_adj_of<T>(owner: &mut T, key: crate::Opaque) -> crate::FnField {
    todo!("needs: nothing to translate into: `key` and the answer are empty types, and RigNodeMeta.getAdj, the one holder of the closure, left the skeleton")
}

/// A character's id as the word on his boxes. (rigProps.ts `c.id & 0xff`)
fn owner_of(g: &mut Game, c: CharId) -> f64 {
    (js::to_i32(g.ch[c].id) & 0xff) as f64
}

/// A bound hostage's wrist tie: one pale strap round both wrists while they are crossed. (rigProps.ts wristTieBoxes)
pub fn wrist_tie_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>) {
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
    let made = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: v3::scale(v3::add(a, b), 0.5),
        h: [l / 2.0 + 0.048, 0.026, 0.045],
        yaw: Some(m::atan2(z[0], z[2])),
        rot: Some(v3::quat::from_basis(x, y, z)),
        albedo: Some([0.6, 0.59, 0.55]),
        flags: Some(crate::rig_props::PROP_FLAGS),
        owner: Some(owner),
        ..Default::default()
    });
    out.push(made);
}

/// One stick part as a box laid along its frame's z, after the part's own adjustment in the group frame. (rigProps.ts seg)
pub fn seg(g: &mut crate::game::Game, out: &mut Vec<BoxId>, g_2: crate::rig_props::Frame, s: &mut crate::rig_props::Seg, albedo: V3, emissive: Option<V3>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, path: Option<&str>, label: Option<&str>, depth: Option<f64>, name: Option<&str>) {
    let adj = s.adj;
    let emissive = emissive.unwrap_or([0.0, 0.0, 0.0]);
    seg_at(g, out, g_2, s, adj, albedo, emissive, meta, path, label, depth, name);
}

/// seg() with the part's adjustment handed in. (rigProps.ts segAt)
pub fn seg_at(g: &mut crate::game::Game, out: &mut Vec<BoxId>, g_2: crate::rig_props::Frame, s: &mut crate::rig_props::Seg, adj: Option<crate::rig_props::Adj>, albedo: V3, emissive: V3, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, path: Option<&str>, label: Option<&str>, depth: Option<f64>, name: Option<&str>) {
    let f = if adj.is_some() { apply_adj(g_2, adj) } else { g_2 };
    let dir = f.z;
    let mid = v3::mad(f.o, dir, (s.l0 + s.l1) / 2.0);
    let len = m::max(0.001, (s.l1 - s.l0) / 2.0);
    if let Some(meta) = meta {
        meta.push(RigNodeMeta { path: String::from(path.unwrap_or("")), label: String::from(label.unwrap_or("")), depth: depth.unwrap_or(0.0), parent: g_2, frame: Frame { o: mid, ..f }, box_start: out.len() as f64, box_count: 1.0 });
    }
    let lit = emissive[0] + emissive[1] + emissive[2] > 0.0;
    let yaw = m::atan2(dir[0], dir[2]);
    let rot = v3::quat::from_basis(f.x, f.y, f.z);
    let stuff = crate::scene_stuff::paint_of(g, Some(&albedo[..])).and_then(|p| p.stuff);
    let owner = g.m.rig_props.cur_owner;
    let made = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
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
    out.push(made);
}

/// Flattened head 'forward'; the body's heading when the head looks nearly straight up or down. (rigProps.ts headFlatForward)
pub fn head_flat_forward(g: &mut crate::game::Game, c_2: CharId) -> V3 {
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
fn goggle_part(g: &mut Game, c: V3, h: V3, yaw: f64, albedo: V3, flags: f64, owner: f64, rot: Quat) -> BoxId {
    crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
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
pub fn goggles_boxes(g: &mut crate::game::Game, c_2: CharId, nv: bool, out: &mut Vec<BoxId>, g_2: &mut crate::rig_props::RigGoggles, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, k: Option<f64>) {
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
    let path = if nv { "goggles.on" } else { "goggles.off" };
    if let Some(meta) = meta.as_deref_mut() {
        meta.push(RigNodeMeta { path: String::from(path), label: String::from(if nv { "goggles (down, NV on)" } else { "goggles (up, NV off)" }), depth: 1.0, parent: head_f, frame: cl, box_start: out.len() as f64, box_count: 6.0 });
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
        meta.push(RigNodeMeta { path: alloc::format!("{}.bridge", path), label: String::from("bridge + mount arm"), depth: 2.0, parent: cl, frame: hs, box_start: out.len() as f64, box_count: 2.0 });
    }
    for (part_at, part_half) in [(t.bridge.at, t.bridge.half), (t.arm.at, t.arm.half)] {
        let made = goggle_part(g, at(hs, part_at), v3::copy(part_half), yaw_h, col.goggles_housing, crate::rig_props::PROP_FLAGS, owner, rot_h);
        out.push(made);
    }
    let ln = apply_adj(cl, g_2.lens_adj);
    let rot_l = v3::quat::from_basis(ln.x, ln.y, ln.z);
    let yaw_l = m::atan2(ln.z[0], ln.z[2]);
    if let Some(meta) = meta.as_deref_mut() {
        meta.push(RigNodeMeta { path: alloc::format!("{}.tubes", path), label: String::from("the two tubes"), depth: 2.0, parent: cl, frame: ln, box_start: out.len() as f64, box_count: 4.0 });
    }
    for sx in [-t.side, t.side] {
        let tube = goggle_part(g, at(ln, [sx, t.down, (t.front - t.back) / 2.0]), [t.half, t.half, (t.front + t.back) / 2.0], yaw_l, col.goggles_housing, crate::rig_props::PROP_FLAGS, owner, rot_l);
        out.push(tube);
        let glass = goggle_part(g, at(ln, [sx, t.down, t.front + t.glass]), [t.half * 0.8, t.half * 0.8, t.glass], yaw_l, col.dark_glass, crate::rig_props::GLOW_FLAGS, owner, rot_l);
        out.push(glass);
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
    /// The TypeScript's own `rigSource()` under bun: the shipped table, an edited one (see `edited`), and the shipped one after clearAdjs.
    const TS_PLAIN: &str = r#"export const RIG: Rig = {
  goggles: {
    on: { fwd: 0.105, up: 0.1 },
    off: { fwd: 0.075, up: 0.185 },
    lensSide: 0.03,
    lensDown: -0.012,
    lensUp: 0.026,
    lensHalf: 0.013,
    lensThick: 0.008,
    housing: { back: 0.03, half: [0.055, 0.028, 0.03] },
    glowOn: [1.5, 14, 3],
    glowOff: [0.35, 3.2, 0.7],
    tubes: { side: 0.032, down: -0.012, half: 0.015, back: 0.06, front: 0.012, glass: 0.003, bridge: { at: [0, 0.012, -0.03], half: [0.03, 0.007, 0.012] }, arm: { at: [0, 0.034, -0.045], half: [0.009, 0.018, 0.009] } },
    followPitch: 1,
    onAdj: { off: [-0.004, 0.04, 0.02], rot: [4.761, 0, 1.1] },
    offAdj: { off: [-0.006, 0.048, 0.032], rot: [-17.2, 0, 0.7] },
  },
  pistol: {
    slide: { l0: 0.02, l1: 0.21, thick: 0.016, tall: 0.026 },
    grip: { l0: 0, l1: 0.095, thick: 0.014, tall: 0.017, adj: { off: [0, -0.024, 0.05], rot: [108, 0, 0] } },
    suppressor: { l0: 0.21, l1: 0.36, thick: 0.019, tall: 0.019 },
    rail: { l0: 0.09, l1: 0.2, thick: 0.013, tall: 0.012, under: 0.034, adj: { off: [0, 0, 0.064], rot: [0, 0, 0] } },
    lens: { l0: 0.2, l1: 0.215, thick: 0.014, tall: 0.013, glow: [14, 14, 13] },
    lightUnder: 0.038,
    lightAhead: 0.2,
    adj: { off: [0.039, 0.08, 0.035], rot: [-1.3, 0, 0] },
  },
  torch: {
    body: { l0: -0.06, l1: 0.15, thick: 0.022, tall: 0.022 },
    lens: { l0: 0.15, l1: 0.18, thick: 0.026, tall: 0.026 },
    lensGlow: 40,
    lightAhead: 0.18,
    adj: { off: [0.076, 0.04, 0.092], rot: [0, 6.7, 0] },
  },
  guardPistol: {
    slide: { l0: 0.02, l1: 0.2, thick: 0.016, tall: 0.026, adj: { off: [0.028, -0.034, 0.073], rot: [11.2, 0, 0.5] } },
    grip: { l0: 0, l1: 0.095, thick: 0.014, tall: 0.017, adj: { off: [0.028, -0.06, 0.118], rot: [119, 0, 0.5] } },
    lens: { l0: 0.2, l1: 0.23, thick: 0.015, tall: 0.015, adj: { off: [0.029, -0.064, 0.023], rot: [11.8, 0, 0] } },
    lightAhead: 0.25,
    adj: { off: [0.031, 0.105, -0.05], rot: [-8.6, 0, 0] },
  },
  knife: {
    blade: { l0: 0.045, l1: 0.215, thick: 0.002, tall: 0.009, adj: { off: [0, 0.003, 0], rot: [0, 0, 0] } },
    edge: { l0: 0.045, l1: 0.215, thick: 0.002, tall: 0.003, adj: { off: [0, -0.009, 0], rot: [0, 0, 0] } },
    guard: { l0: 0.04, l1: 0.048, thick: 0.005, tall: 0.019 },
    handle: { l0: -0.065, l1: 0.045, thick: 0.011, tall: 0.013 },
    grip: { fwd: 0.08, palm: 0.03, side: 0 },
  },
  sheath: {
    at: [0.07, -0.085, 0.2],
    along: [0.26, 1, -0.2],
    out: [0, 0.18, 1],
    body: { l0: 0.046, l1: 0.228, thick: 0.006, tall: 0.016 },
  },
  can: {
    sink: -0.02,
    tilt: 0.35,
    body: { l0: -0.02, l1: 0.12, thick: 0.032, tall: 0.032 },
    band: { l0: 0.085, l1: 0.109, thick: 0.034, tall: 0.034 },
  },
  holster: {
    down: 0.06,
    out: 0.115,
    fwd: 0.01,
  },
  carbine: {
    receiver: { l0: -0.09, l1: 0.155, thick: 0.017, tall: 0.032, adj: { off: [0, -0.012, 0], rot: [0, 0, 0] } },
    handguard: { l0: 0.155, l1: 0.275, thick: 0.02, tall: 0.025, adj: { off: [0, -0.008, 0], rot: [0, 0, 0] } },
    barrel: { l0: 0.275, l1: 0.34, thick: 0.007, tall: 0.007 },
    muzzle: { l0: 0.34, l1: 0.41, thick: 0.015, tall: 0.015 },
    mag: { l0: 0, l1: 0.11, thick: 0.013, tall: 0.028, adj: { off: [0, -0.04, 0.085], rot: [78, 0, 0] } },
    mag2: { l0: 0, l1: 0.1, thick: 0.013, tall: 0.027, adj: { off: [0, -0.143, 0.106], rot: [60, 0, 0] } },
    grip: { l0: 0, l1: 0.09, thick: 0.013, tall: 0.016, adj: { off: [0, -0.045, -0.015], rot: [108, 0, 0] } },
    stockTop: { l0: -0.32, l1: -0.09, thick: 0.007, tall: 0.008, adj: { off: [0, 0.005, 0], rot: [0, 0, 0] } },
    stockLow: { l0: 0, l1: 0.235, thick: 0.007, tall: 0.008, adj: { off: [0, -0.04, -0.09], rot: [171, 0, 0] } },
    light: { l0: 0.18, l1: 0.265, thick: 0.011, tall: 0.011, adj: { off: [-0.032, -0.006, 0], rot: [0, 0, 0] } },
    lens: { l0: 0.265, l1: 0.28, thick: 0.012, tall: 0.012, adj: { off: [-0.032, -0.006, 0], rot: [0, 0, 0] } },
    lensGlow: 40,
    lightAhead: 0.285,
    gripL: { off: [0.099, -0.059, 0.149], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] },
    adj: { off: [0.06, 0.08, 0.09], rot: [0, 0, 0] },
  },
  sc20k: {
    hull: { l0: -0.29, l1: 0.12, thick: 0.024, tall: 0.04, adj: { off: [0, -0.012, 0], rot: [0, 0, 0] } },
    butt: { l0: -0.31, l1: -0.29, thick: 0.026, tall: 0.048, adj: { off: [0, -0.018, 0], rot: [0, 0, 0] } },
    mag: { l0: 0, l1: 0.13, thick: 0.012, tall: 0.03, adj: { off: [0, -0.045, -0.12], rot: [84, 0, 0] } },
    grip: { l0: 0, l1: 0.09, thick: 0.013, tall: 0.016, adj: { off: [0, -0.045, -0.015], rot: [108, 0, 0] } },
    fore: { l0: 0.12, l1: 0.3, thick: 0.022, tall: 0.03, adj: { off: [0, -0.014, 0], rot: [0, 0, 0] } },
    suppressor: { l0: 0.3, l1: 0.43, thick: 0.019, tall: 0.019 },
    rail: { l0: -0.16, l1: 0.16, thick: 0.011, tall: 0.008, adj: { off: [0, 0.036, 0], rot: [0, 0, 0] } },
    optic: { l0: -0.04, l1: 0.08, thick: 0.017, tall: 0.02, adj: { off: [0, 0.062, 0], rot: [0, 0, 0] } },
    foregrip: { l0: 0, l1: 0.085, thick: 0.012, tall: 0.012, adj: { off: [0, -0.044, 0.2], rot: [90, 0, 0] } },
    light: { l0: 0.19, l1: 0.27, thick: 0.011, tall: 0.011, adj: { off: [-0.034, -0.008, 0], rot: [0, 0, 0] } },
    lens: { l0: 0.27, l1: 0.285, thick: 0.012, tall: 0.012, adj: { off: [-0.034, -0.008, 0], rot: [0, 0, 0] } },
    lensGlow: 40,
    lightAhead: 0.29,
    gripL: { off: [0.06, -0.085, 0.145], fingers: [-0.3, 0.05, 0.95], back: [0.95, -0.15, 0.25] },
    slung: { off: [-0.02, 0.17, -0.18], bore: [0.5, -0.866, 0.2], roll: 180, follow: 0.5 },
    adj: { off: [0.06, 0.08, 0.09], rot: [0, 0, 0] },
  },
  m4: {
    hull: { l0: -0.075, l1: 0.115, thick: 0.017, tall: 0.036, adj: { off: [0, -0.012, 0], rot: [0, 0, 0] } },
    butt: { l0: -0.31, l1: -0.19, thick: 0.017, tall: 0.036, adj: { off: [0, -0.02, 0], rot: [0, 0, 0] } },
    mag: { l0: 0, l1: 0.14, thick: 0.011, tall: 0.03, adj: { off: [0, -0.045, 0.075], rot: [80, 0, 0] } },
    fore: { l0: 0.115, l1: 0.3, thick: 0.022, tall: 0.022, adj: { off: [0, -0.003, 0], rot: [0, 0, 0] } },
    rail: { l0: -0.07, l1: 0.3, thick: 0.009, tall: 0.005, adj: { off: [0, 0.027, 0], rot: [0, 0, 0] } },
    optic: { l0: 0, l1: 0.07, thick: 0.016, tall: 0.02, adj: { off: [0, 0.052, 0], rot: [0, 0, 0] } },
    tube: { l0: -0.19, l1: -0.075, thick: 0.012, tall: 0.012, adj: { off: [0, -0.002, 0], rot: [0, 0, 0] } },
    peq: { l0: 0.215, l1: 0.29, thick: 0.017, tall: 0.011, adj: { off: [0, 0.043, 0], rot: [0, 0, 0] } },
    gl: { l0: 0.14, l1: 0.3, thick: 0.02, tall: 0.024, adj: { off: [0, -0.05, 0], rot: [0, 0, 0] } },
  },
};"#;
    const TS_EDITED: &str = r#"export const RIG: Rig = {
  goggles: {
    on: { fwd: 0.105, up: 0.1 },
    off: { fwd: 0.075, up: 0.185 },
    lensSide: 0.03,
    lensDown: -0.012,
    lensUp: 0.026,
    lensHalf: 0.013,
    lensThick: 0.008,
    housing: { back: 0.03, half: [0.055, 0.028, 0.03] },
    glowOn: [1.5, 14, 3],
    glowOff: [0.35, 3.2, 0.7],
    tubes: { side: 0.032, down: -0.012, half: 0.015, back: 0.06, front: 0.012, glass: 0.003, bridge: { at: [0, 0.012, -0.03], half: [0.03, 0.007, 0.012] }, arm: { at: [0, 0.034, -0.045], half: [0.009, 0.018, 0.009] } },
    followPitch: 1,
    onAdj: { off: [-0.004, 0.04, 0.02], rot: [4.761, 0, 1.1] },
    offAdj: { off: [-0.006, 0.048, 0.032], rot: [-17.2, 0, 0.7] },
    lensAdj: { off: [1234.568, 0, 0], rot: [2.001, 0.002, 1] },
  },
  pistol: {
    slide: { l0: 0.02, l1: 0.21, thick: 0.016, tall: 0.026, adj: { off: [0.063, -0.001, 0], rot: [0, 0, 0.001] } },
    grip: { l0: 0, l1: 0.095, thick: 0.014, tall: 0.017, adj: { off: [0, -0.024, 0.05], rot: [108, 0, 0] } },
    suppressor: { l0: 0.21, l1: 0.36, thick: 0.019, tall: 0.019 },
    rail: { l0: 0.09, l1: 0.2, thick: 0.013, tall: 0.012, under: 0.034, adj: { off: [0, 0, 0.064], rot: [0, 0, 0] } },
    lens: { l0: 0.2, l1: 0.215, thick: 0.014, tall: 0.013, glow: [14, 14, 13], adj: { off: [1, 2, 3], rot: [0.188, 0.313, -0.063] } },
    lightUnder: 0.038,
    lightAhead: 0.2,
  },
  torch: {
    body: { l0: -0.06, l1: 0.15, thick: 0.022, tall: 0.022 },
    lens: { l0: 0.15, l1: 0.18, thick: 0.026, tall: 0.026 },
    lensGlow: 40,
    lightAhead: 0.18,
    adj: { off: [0.076, 0.04, 0.092], rot: [0, 6.7, 0] },
  },
  guardPistol: {
    slide: { l0: 0.02, l1: 0.2, thick: 0.016, tall: 0.026, adj: { off: [0.028, -0.034, 0.073], rot: [11.2, 0, 0.5] } },
    grip: { l0: 0, l1: 0.095, thick: 0.014, tall: 0.017, adj: { off: [0.028, -0.06, 0.118], rot: [119, 0, 0.5] } },
    lens: { l0: 0.2, l1: 0.23, thick: 0.015, tall: 0.015, adj: { off: [0.029, -0.064, 0.023], rot: [11.8, 0, 0] } },
    lightAhead: 0.25,
    adj: { off: [0.031, 0.105, -0.05], rot: [-8.6, 0, 0] },
  },
  knife: {
    blade: { l0: 0.045, l1: 0.215, thick: 0.002, tall: 0.009, adj: { off: [0, 0.003, 0], rot: [0, 0, 0] } },
    edge: { l0: 0.045, l1: 0.215, thick: 0.002, tall: 0.003, adj: { off: [0, -0.009, 0], rot: [0, 0, 0] } },
    guard: { l0: 0.04, l1: 0.048, thick: 0.005, tall: 0.019 },
    handle: { l0: -0.065, l1: 0.045, thick: 0.011, tall: 0.013 },
    grip: { fwd: 0.08, palm: 0.03, side: 0 },
    adj: { off: [0.001, 0, 0], rot: [0, 0, 0] },
  },
  sheath: {
    at: [0.07, -0.085, 0.2],
    along: [0.26, 1, -0.2],
    out: [0, 0.18, 1],
    body: { l0: 0.046, l1: 0.228, thick: 0.006, tall: 0.016, adj: { off: [0, 0.25, 0], rot: [0, 0, 0] } },
  },
  can: {
    sink: -0.02,
    tilt: 0.35,
    body: { l0: 0.001, l1: -0.063, thick: 0.032, tall: 0.032 },
    band: { l0: 0.085, l1: 0.109, thick: 0.034, tall: 0.034 },
  },
  holster: {
    down: 0.06,
    out: 0.115,
    fwd: 0.01,
  },
  carbine: {
    receiver: { l0: -0.09, l1: 0.155, thick: 0.017, tall: 0.032, adj: { off: [0, -0.012, 0], rot: [0, 0, 0] } },
    handguard: { l0: 0.155, l1: 0.275, thick: 0.02, tall: 0.025, adj: { off: [0, -0.008, 0], rot: [0, 0, 0] } },
    barrel: { l0: 0.275, l1: 0.34, thick: 0.007, tall: 0.007 },
    muzzle: { l0: 0.34, l1: 0.41, thick: 0.015, tall: 0.015 },
    mag: { l0: 0, l1: 0.11, thick: 0.013, tall: 0.028, adj: { off: [0, -0.04, 0.085], rot: [78, 0, 0] } },
    mag2: { l0: 0, l1: 0.1, thick: 0.013, tall: 0.027, adj: { off: [0, -0.143, 0.106], rot: [60, 0, 0] } },
    grip: { l0: 0, l1: 0.09, thick: 0.013, tall: 0.016, adj: { off: [0, -0.045, -0.015], rot: [108, 0, 0] } },
    stockTop: { l0: -0.32, l1: -0.09, thick: 0.007, tall: 0.008, adj: { off: [0, 0.005, 0], rot: [0, 0, 0] } },
    stockLow: { l0: 0, l1: 0.235, thick: 0.007, tall: 0.008, adj: { off: [0, -0.04, -0.09], rot: [171, 0, 0] } },
    light: { l0: 0.18, l1: 0.265, thick: 0.011, tall: 0.011, adj: { off: [-0.032, -0.006, 0], rot: [0, 0, 0] } },
    lens: { l0: 0.265, l1: 0.28, thick: 0.012, tall: 0.012, adj: { off: [-0.032, -0.006, 0], rot: [0, 0, 0] } },
    lensGlow: 40,
    lightAhead: 0.285,
    gripL: { off: [0.099, -0.059, 0.149], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] },
    adj: { off: [0.06, 0.08, 0.09], rot: [0, 0, 0] },
  },
  sc20k: {
    hull: { l0: -0.29, l1: 0.12, thick: 0.024, tall: 0.04, adj: { off: [0, -0.012, 0], rot: [0, 0, 0] } },
    butt: { l0: -0.31, l1: -0.29, thick: 0.026, tall: 0.048, adj: { off: [0, -0.018, 0], rot: [0, 0, 0] } },
    mag: { l0: 0, l1: 0.13, thick: 0.012, tall: 0.03, adj: { off: [0, -0.045, -0.12], rot: [84, 0, 0] } },
    grip: { l0: 0, l1: 0.09, thick: 0.013, tall: 0.016, adj: { off: [0, -0.045, -0.015], rot: [108, 0, 0] } },
    fore: { l0: 0.12, l1: 0.3, thick: 0.022, tall: 0.03, adj: { off: [0, -0.014, 0], rot: [0, 0, 0] } },
    suppressor: { l0: 0.3, l1: 0.43, thick: 0.019, tall: 0.019 },
    rail: { l0: -0.16, l1: 0.16, thick: 0.011, tall: 0.008, adj: { off: [0, 0.036, 0], rot: [0, 0, 0] } },
    optic: { l0: -0.04, l1: 0.08, thick: 0.017, tall: 0.02, adj: { off: [0, 0.062, 0], rot: [0, 0, 0] } },
    foregrip: { l0: 0, l1: 0.085, thick: 0.012, tall: 0.012, adj: { off: [0, -0.044, 0.2], rot: [90, 0, 0] } },
    light: { l0: 0.19, l1: 0.27, thick: 0.011, tall: 0.011, adj: { off: [-0.034, -0.008, 0], rot: [0, 0, 0] } },
    lens: { l0: 0.27, l1: 0.285, thick: 0.012, tall: 0.012, adj: { off: [-0.034, -0.008, 0], rot: [0, 0, 0] } },
    lensGlow: 40,
    lightAhead: 0.29,
    gripL: { off: [0.06, -0.085, 0.145], fingers: [-0.3, 0.05, 0.95], back: [0.95, -0.15, 0.25] },
    slung: { off: [-0.02, 0.17, -0.18], bore: [0.5, -0.866, 0.2], roll: 180, follow: 0.5, adj: { off: [0.5, 0, 0], rot: [0, 90, 0] } },
    adj: { off: [0.06, 0.08, 0.09], rot: [0, 0, 0] },
  },
  m4: {
    hull: { l0: -0.075, l1: 0.115, thick: 0.017, tall: 0.036, adj: { off: [0, -0.012, 0], rot: [0, 0, 0] } },
    butt: { l0: -0.31, l1: -0.19, thick: 0.017, tall: 0.036, adj: { off: [0, -0.02, 0], rot: [0, 0, 0] } },
    mag: { l0: 0, l1: 0.14, thick: 0.011, tall: 0.03, adj: { off: [0, -0.045, 0.075], rot: [80, 0, 0] } },
    fore: { l0: 0.115, l1: 0.3, thick: 0.022, tall: 0.022, adj: { off: [0, -0.003, 0], rot: [0, 0, 0] } },
    rail: { l0: -0.07, l1: 0.3, thick: 0.009, tall: 0.005, adj: { off: [0, 0.027, 0], rot: [0, 0, 0] } },
    optic: { l0: 0, l1: 0.07, thick: 0.016, tall: 0.02, adj: { off: [0, 0.052, 0], rot: [0, 0, 0] } },
    tube: { l0: -0.19, l1: -0.075, thick: 0.012, tall: 0.012, adj: { off: [0, -0.002, 0], rot: [0, 0, 0] } },
    peq: { l0: 0.215, l1: 0.29, thick: 0.017, tall: 0.011, adj: { off: [0, 0.043, 0], rot: [0, 0, 0] } },
    gl: { l0: 0.14, l1: 0.3, thick: 0.02, tall: 0.123, adj: { off: [0, -0.05, 0], rot: [0, 0, 0] } },
  },
};"#;
    const TS_CLEARED: &str = r#"export const RIG: Rig = {
  goggles: {
    on: { fwd: 0.105, up: 0.1 },
    off: { fwd: 0.075, up: 0.185 },
    lensSide: 0.03,
    lensDown: -0.012,
    lensUp: 0.026,
    lensHalf: 0.013,
    lensThick: 0.008,
    housing: { back: 0.03, half: [0.055, 0.028, 0.03] },
    glowOn: [1.5, 14, 3],
    glowOff: [0.35, 3.2, 0.7],
    tubes: { side: 0.032, down: -0.012, half: 0.015, back: 0.06, front: 0.012, glass: 0.003, bridge: { at: [0, 0.012, -0.03], half: [0.03, 0.007, 0.012] }, arm: { at: [0, 0.034, -0.045], half: [0.009, 0.018, 0.009] } },
    followPitch: 1,
  },
  pistol: {
    slide: { l0: 0.02, l1: 0.21, thick: 0.016, tall: 0.026 },
    grip: { l0: 0, l1: 0.095, thick: 0.014, tall: 0.017 },
    suppressor: { l0: 0.21, l1: 0.36, thick: 0.019, tall: 0.019 },
    rail: { l0: 0.09, l1: 0.2, thick: 0.013, tall: 0.012, under: 0.034 },
    lens: { l0: 0.2, l1: 0.215, thick: 0.014, tall: 0.013, glow: [14, 14, 13] },
    lightUnder: 0.038,
    lightAhead: 0.2,
  },
  torch: {
    body: { l0: -0.06, l1: 0.15, thick: 0.022, tall: 0.022 },
    lens: { l0: 0.15, l1: 0.18, thick: 0.026, tall: 0.026 },
    lensGlow: 40,
    lightAhead: 0.18,
  },
  guardPistol: {
    slide: { l0: 0.02, l1: 0.2, thick: 0.016, tall: 0.026 },
    grip: { l0: 0, l1: 0.095, thick: 0.014, tall: 0.017 },
    lens: { l0: 0.2, l1: 0.23, thick: 0.015, tall: 0.015 },
    lightAhead: 0.25,
  },
  knife: {
    blade: { l0: 0.045, l1: 0.215, thick: 0.002, tall: 0.009 },
    edge: { l0: 0.045, l1: 0.215, thick: 0.002, tall: 0.003 },
    guard: { l0: 0.04, l1: 0.048, thick: 0.005, tall: 0.019 },
    handle: { l0: -0.065, l1: 0.045, thick: 0.011, tall: 0.013 },
    grip: { fwd: 0.08, palm: 0.03, side: 0 },
  },
  sheath: {
    at: [0.07, -0.085, 0.2],
    along: [0.26, 1, -0.2],
    out: [0, 0.18, 1],
    body: { l0: 0.046, l1: 0.228, thick: 0.006, tall: 0.016 },
  },
  can: {
    sink: -0.02,
    tilt: 0.35,
    body: { l0: -0.02, l1: 0.12, thick: 0.032, tall: 0.032 },
    band: { l0: 0.085, l1: 0.109, thick: 0.034, tall: 0.034 },
  },
  holster: {
    down: 0.06,
    out: 0.115,
    fwd: 0.01,
  },
  carbine: {
    receiver: { l0: -0.09, l1: 0.155, thick: 0.017, tall: 0.032 },
    handguard: { l0: 0.155, l1: 0.275, thick: 0.02, tall: 0.025 },
    barrel: { l0: 0.275, l1: 0.34, thick: 0.007, tall: 0.007 },
    muzzle: { l0: 0.34, l1: 0.41, thick: 0.015, tall: 0.015 },
    mag: { l0: 0, l1: 0.11, thick: 0.013, tall: 0.028 },
    mag2: { l0: 0, l1: 0.1, thick: 0.013, tall: 0.027 },
    grip: { l0: 0, l1: 0.09, thick: 0.013, tall: 0.016 },
    stockTop: { l0: -0.32, l1: -0.09, thick: 0.007, tall: 0.008 },
    stockLow: { l0: 0, l1: 0.235, thick: 0.007, tall: 0.008 },
    light: { l0: 0.18, l1: 0.265, thick: 0.011, tall: 0.011 },
    lens: { l0: 0.265, l1: 0.28, thick: 0.012, tall: 0.012 },
    lensGlow: 40,
    lightAhead: 0.285,
    gripL: { off: [0.099, -0.059, 0.149], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] },
  },
  sc20k: {
    hull: { l0: -0.29, l1: 0.12, thick: 0.024, tall: 0.04 },
    butt: { l0: -0.31, l1: -0.29, thick: 0.026, tall: 0.048 },
    mag: { l0: 0, l1: 0.13, thick: 0.012, tall: 0.03 },
    grip: { l0: 0, l1: 0.09, thick: 0.013, tall: 0.016 },
    fore: { l0: 0.12, l1: 0.3, thick: 0.022, tall: 0.03 },
    suppressor: { l0: 0.3, l1: 0.43, thick: 0.019, tall: 0.019 },
    rail: { l0: -0.16, l1: 0.16, thick: 0.011, tall: 0.008 },
    optic: { l0: -0.04, l1: 0.08, thick: 0.017, tall: 0.02 },
    foregrip: { l0: 0, l1: 0.085, thick: 0.012, tall: 0.012 },
    light: { l0: 0.19, l1: 0.27, thick: 0.011, tall: 0.011 },
    lens: { l0: 0.27, l1: 0.285, thick: 0.012, tall: 0.012 },
    lensGlow: 40,
    lightAhead: 0.29,
    gripL: { off: [0.06, -0.085, 0.145], fingers: [-0.3, 0.05, 0.95], back: [0.95, -0.15, 0.25] },
    slung: { off: [-0.02, 0.17, -0.18], bore: [0.5, -0.866, 0.2], roll: 180, follow: 0.5 },
  },
  m4: {
    hull: { l0: -0.075, l1: 0.115, thick: 0.017, tall: 0.036 },
    butt: { l0: -0.31, l1: -0.19, thick: 0.017, tall: 0.036 },
    mag: { l0: 0, l1: 0.14, thick: 0.011, tall: 0.03 },
    fore: { l0: 0.115, l1: 0.3, thick: 0.022, tall: 0.022 },
    rail: { l0: -0.07, l1: 0.3, thick: 0.009, tall: 0.005 },
    optic: { l0: 0, l1: 0.07, thick: 0.016, tall: 0.02 },
    tube: { l0: -0.19, l1: -0.075, thick: 0.012, tall: 0.012 },
    peq: { l0: 0.215, l1: 0.29, thick: 0.017, tall: 0.011 },
    gl: { l0: 0.14, l1: 0.3, thick: 0.02, tall: 0.024 },
  },
};"#;
    /// `String(+n.toFixed(3))` under bun.
    const TS_NUMS: &[(f64, &str)] = &[(f64::from_bits(0x0000000000000000), "0"), (f64::from_bits(0x8000000000000000), "0"), (f64::from_bits(0x3ff0000000000000), "1"), (f64::from_bits(0xbff0000000000000), "-1"), (f64::from_bits(0x3f40624dd2f1a9fc), "0.001"), (f64::from_bits(0xbf40624dd2f1a9fc), "-0.001"), (f64::from_bits(0x3fb0000000000000), "0.063"), (f64::from_bits(0xbfb0000000000000), "-0.063"), (f64::from_bits(0x3fc8000000000000), "0.188"), (f64::from_bits(0x3fd4000000000000), "0.313"), (f64::from_bits(0x3ff0020c49ba5e35), "1"), (f64::from_bits(0x4000010624dd2f1b), "2.001"), (f64::from_bits(0x3f1a36e2eb1c432d), "0"), (f64::from_bits(0xbf1a36e2eb1c432d), "0"), (f64::from_bits(0x40fe240ca1cac083), "123456.789"), (f64::from_bits(0x3fd3333333333334), "0.3"), (f64::from_bits(0x40130b4395810625), "4.761")];

    fn adj(off: V3, rot: V3) -> Option<Adj> {
        Some(Adj { off, rot })
    }

    fn edited() -> Rig {
        let mut e = crate::rig_props::rig();
        e.pistol.adj = adj([0.0, 0.0004, 0.0], [0.0, 0.0, -0.0004]);
        e.pistol.slide.adj = adj([0.0625, -0.0005, 1e-4], [0.0, 0.0, 0.0005]);
        e.pistol.lens.adj = adj([1.0, 2.0, 3.0], [0.1875, 0.3125, -0.0625]);
        e.holster.adj = adj([0.0004, 0.0, 0.0], [0.0, 0.0, 0.0]);
        e.goggles.housing_adj = adj([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        e.goggles.lens_adj = adj([1234.5678, 1e-5, -1e-5], [2.0005, 0.0015, 1.0005]);
        e.knife.adj = adj([0.001, 0.0, 0.0], [0.0, 0.0, 0.0]);
        e.sc20k.slung.adj = adj([0.5, 0.0, 0.0], [0.0, 90.0, 0.0]);
        e.sheath.body.adj = adj([0.0, 0.25, 0.0], [0.0, 0.0, 0.0]);
        e.can.body.l0 = 0.0005;
        e.can.body.l1 = -0.0625;
        e.m4.gl.tall = 0.1234;
        e
    }

    #[test]
    fn rig_source_is_the_typescripts_text() {
        assert_eq!(rig_source(None), TS_PLAIN);
        assert_eq!(rig_source(Some(edited())), TS_EDITED);
        let mut cleared = crate::rig_props::rig();
        for slot in adj_slots(&mut cleared) {
            *slot = None;
        }
        assert_eq!(rig_source(Some(cleared)), TS_CLEARED);
    }

    #[test]
    fn numbers_round_as_to_fixed_does() {
        for (n, s) in TS_NUMS {
            assert_eq!(num_source(*n), *s, "{n}");
        }
    }

    #[test]
    fn adj_slots_holds_every_adjustment() {
        let mut r = crate::rig_props::rig();
        let text = alloc::format!("{:?}", r);
        assert_eq!(text.matches("adj: ").count(), adj_slots(&mut r).len());
    }

    #[test]
    fn assign_rig_keeps_what_the_source_lacks() {
        let mut dst = edited();
        let mut src = crate::rig_props::rig();
        src.pistol.adj = None;
        src.pistol.slide.l0 = 0.5;
        src.torch.adj = adj([9.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        let kept = dst.pistol.adj.unwrap();
        assign_rig(&mut dst, src);
        assert_eq!(dst.pistol.adj.unwrap().rot, kept.rot);
        assert_eq!(dst.pistol.slide.l0, 0.5);
        assert_eq!(dst.torch.adj.unwrap().off[0], 9.0);
        assert_eq!(dst.can.body.l0, crate::rig_props::rig().can.body.l0);
    }

    #[test]
    fn assign_tree_copies_as_the_typescript_does() {
                let mut inner = JsMap::new();
        inner.set(String::from("x"), S::Num(1.0));
        let mut src = JsMap::new();
        src.set(String::from("a"), S::Array(vec![1.0, 2.0]));
        src.set(String::from("b"), S::Array(vec![3.0]));
        src.set(String::from("c"), S::Arm2(inner.clone()));
        src.set(String::from("d"), S::Arm2(inner));
        src.set(String::from("n"), S::Num(7.0));
        let mut dst = JsMap::new();
        dst.set(String::from("z"), D::Num(5.0));
        dst.set(String::from("a"), D::Array(vec![0.0, 0.0]));
        dst.set(String::from("b"), D::Array(vec![0.0, 0.0]));
        dst.set(String::from("c"), D::Array(vec![0.0]));
        assign_tree(&mut dst, &src);
        assert_eq!(dst.keys(), ["z", "a", "b", "c", "d", "n"].map(String::from).to_vec());
        assert!(matches!(dst.get(String::from("z")), Some(D::Num(x)) if *x == 5.0));
        assert!(matches!(dst.get(String::from("a")), Some(D::Array(a)) if a.as_slice() == [1.0, 2.0]));
        assert!(matches!(dst.get(String::from("b")), Some(D::Array(a)) if a.as_slice() == [3.0]));
        assert!(matches!(dst.get(String::from("c")), Some(D::Arm2(o)) if matches!(o.get(String::from("x")), Some(D::Num(x)) if *x == 1.0)));
        assert!(matches!(dst.get(String::from("d")), Some(D::Arm2(_))));
    }
}
