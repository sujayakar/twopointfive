#![allow(unused_imports, unused_variables)]
pub use ::game_types::packside::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

// packside.ts WHOLE (148 lines; triaged dress, but its boxes are in the frame's list the rays and fog read, and PACKSIDE.on is on in the page).

fn bit_of(parts: &[String], name: &str) -> f64 {
    match parts.iter().position(|p| p == name) { Some(i) => (1u32 << i) as f64, None => 0.0 }
}
fn trunk_name(n: &str) -> bool {
    // TRUNK = /^DEF-(hips|spine\.00[1-3]|thigh\.[LR])$/
    matches!(n, "DEF-hips" | "DEF-spine.001" | "DEF-spine.002" | "DEF-spine.003" | "DEF-thigh.L" | "DEF-thigh.R")
}
/// the memo's row made if it can be: false = no cast, or a mesh with no parts
fn memo_made(g: &mut Game, ch: RigId) -> bool {
    let Some(cast) = g.tab.rig[ch].cast else { return false; };
    let mesh = g.tab.rig[ch].mesh;
    if g.tab.anim_gltf_mesh_data[mesh].part.is_none() { return false; }
    if g.m.packside.memo.has(ch) { return true; }
    let parts: Vec<String> = g.tab.anim_gltf_cast_info[cast].parts.clone();
    let n = parts.len();
    let mut ext: Vec<Option<PackExt>> = vec![None; n];
    let mut kit: u32 = 0;
    for (i, name) in parts.iter().enumerate() {
        if name.starts_with("kit_") {
            kit |= 1u32 << i;
            ext[i] = Some(PackExt { x0: m::INF, x1: -m::INF, top0: -m::INF, top1: -m::INF, z0: m::INF, z1: -m::INF });
        }
    }
    {
        let md = &g.tab.anim_gltf_mesh_data[mesh];
        let (p, part) = (&md.positions, md.part.as_ref().unwrap());
        let vc = md.vertex_count as usize;
        for v in 0..vc {
            let pi = part[v] as usize;
            if pi >= n { continue; }
            let Some(e) = ext[pi].as_mut() else { continue; };
            let (x, y, z) = (p[v * 3] as f64, p[v * 3 + 1] as f64, p[v * 3 + 2] as f64);
            if z > ZB || y < Y0 || y > Y1 { continue; }
            if x < e.x0 { e.x0 = x; }
            if x > e.x1 { e.x1 = x; }
            if z < e.z0 { e.z0 = z; }
            if z > e.z1 { e.z1 = z; }
        }
        for v in 0..vc {
            let pi = part[v] as usize;
            if pi >= n { continue; }
            let Some(e) = ext[pi].as_mut() else { continue; };
            let (x, y, z) = (p[v * 3] as f64, p[v * 3 + 1] as f64, p[v * 3 + 2] as f64);
            if z > ZB || y < Y0 || y > Y1 { continue; }
            if x < e.x0 + STRIP && y > e.top0 { e.top0 = y; }
            if x > e.x1 - STRIP && y > e.top1 { e.top1 = y; }
        }
    }
    for i in 0..n {
        if let Some(e) = ext[i] { if !(e.x0 < e.x1) { ext[i] = None; } }
    }
    let mut body: u32 = 0;
    for v in g.tab.anim_gltf_cast_info[cast].variants.clone() {
        let cv = &g.tab.anim_gltf_cast_variant[v];
        if cv.side == crate::anim_gltf::CastSide::Operator { body |= js::to_u32(cv.mask); }
    }
    if body == 0 { body = 0xffff_ffff; }
    body &= !kit;
    let rig = &g.tab.rig[ch];
    let node = rig.node_by_name.get(String::from("DEF-spine.003")).copied().unwrap_or(-1.0);
    let j3 = match rig.skin.joints.iter().position(|j| *j == node) { Some(i) => i as f64, None => -1.0 };
    let memo = Memo { j3, ext, kit: kit as f64, ram: bit_of(&parts, "kit_breacher"), pack: bit_of(&parts, "kit_rifleman"), body: body as f64, back: JsMap::new() };
    g.m.packside.memo.set(ch, memo);
    true
}
/// The pack a man drawn with this part mask wears on his back; None with the switch off, no cast, or none worn.
pub fn pack_of(g: &mut Game, ch: RigId, mask: f64) -> Option<PackExt> {
    if !memo_made(g, ch) { return None; }
    let mm = g.m.packside.memo.get(ch).unwrap();
    let mask = js::to_u32(mask);
    if mm.j3 < 0.0 || mask & js::to_u32(mm.kit) == 0 { return None; }
    for i in 0..mm.ext.len() {
        if mm.ext[i].is_some() && (mask >> (i as u32 & 31)) & 1 != 0 { return mm.ext[i]; }
    }
    None
}
/// HALF 3: the mask a man is drawn with while he carries a real tool on his back.
pub fn pack_mask(g: &mut Game, ch: RigId, mask: f64, carries: bool) -> f64 {
    if !carries { return mask; }
    if !memo_made(g, ch) { return mask; }
    let mm = g.m.packside.memo.get(ch).unwrap();
    let (ram, pack, mk) = (js::to_u32(mm.ram), js::to_u32(mm.pack), js::to_u32(mask));
    if ram == 0 || pack == 0 || mk & ram == 0 { return mask; }
    ((mk & !ram) | pack) as f64
}
/// A thing's place at a pack's side, in the mesh's bind space.
pub fn side_bind(g: &Game, e: PackExt, what: crate::rig_props::PackSideWhat, dims: SideBindDims) -> SidePlace {
    let t = g.k.packside;
    if what == crate::rig_props::PackSideWhat::Hammer {
        let head = dims.head.unwrap_or([0.085, 0.045, 0.035]);
        let grip = dims.grip.unwrap_or(0.3);
        let x = e.x0 - t.gap - dims.thick;
        let y_head = m::max(t.head_min, e.top0 + t.over + head[2]);
        return SidePlace { o: [x, y_head - grip, t.handle], x: [1.0, 0.0, 0.0], y: [0.0, 0.0, -1.0], z: [0.0, 1.0, 0.0], pivot: Some([x, y_head, t.handle]), len: Some(dims.len.unwrap_or(0.8)), r: Some(dims.thick) };
    }
    let left = what == crate::rig_props::PackSideWhat::Rifle;
    let x = if left { e.x1 + t.gap + dims.thick } else { e.x0 - t.gap - dims.thick };
    SidePlace { o: [x, t.butt - dims.rear.unwrap_or(0.3), t.gun - dims.top.unwrap_or(0.02)], x: [1.0, 0.0, 0.0], y: [0.0, 0.0, 1.0], z: [0.0, -1.0, 0.0], pivot: Some([x, t.butt, t.gun]), len: Some(dims.len.unwrap_or(0.8)), r: Some(0.0) }
}
/// HIS BACK'S LINE under a lateral strip at bind x.
// SKELETON-GAP: packside::back_line taking g (its stub is `back_line(ch, x)`: it reads the mesh and writes the memo)
fn back_line_g(g: &mut Game, ch: RigId, x: f64) -> Vec<BackPt> {
    let xc = m::max(-REACH, m::min(REACH, x));
    let key = crate::Num(m::round(xc * 1000.0));
    if let Some(l) = g.m.packside.memo.get(ch).unwrap().back.get(key) { return l.clone(); }
    let body = js::to_u32(g.m.packside.memo.get(ch).unwrap().body);
    let rig = &g.tab.rig[ch];
    let trunk: Vec<bool> = rig.skin.joints.iter().map(|n| { let i = *n as usize; if *n >= 0.0 && i < rig.nodes.len() { trunk_name(&rig.nodes[i].name) } else { false } }).collect();
    let md = &g.tab.anim_gltf_mesh_data[rig.mesh];
    let (p, part, w) = (&md.positions, md.part.as_ref().unwrap(), &md.weights);
    let jn = |i: usize| -> usize { match &md.joints { crate::anim_gltf::MeshDataJoints::Uint16Array(v) => v[i] as usize, crate::anim_gltf::MeshDataJoints::Uint8Array(v) => v[i] as usize } };
    let mut bands: Vec<(f64, BackPt)> = Vec::new();
    for v in 0..md.vertex_count as usize {
        if (body >> (part[v] as u32 & 31)) & 1 == 0 { continue; }
        let (px, y, z) = (p[v * 3] as f64, p[v * 3 + 1] as f64, p[v * 3 + 2] as f64);
        if m::abs(px - xc) > 0.04 || z > -0.02 || y < 0.45 || y > 1.62 { continue; }
        let mut b = 0;
        for k in 1..4 { if w[v * 4 + k] > w[v * 4 + b] { b = k; } }
        let j = jn(v * 4 + b);
        if !trunk.get(j).copied().unwrap_or(false) { continue; }
        let i = m::floor(y / BAND);
        match bands.iter_mut().find(|(k, _)| *k == i) {
            Some((_, had)) => { if z < had.z { *had = BackPt { j: j as f64, y, z }; } }
            None => bands.push((i, BackPt { j: j as f64, y, z })),
        }
    }
    let line: Vec<BackPt> = bands.into_iter().map(|(_, b)| b).collect();
    g.m.packside.memo.get_mut(ch).unwrap().back.set(key, line.clone());
    line
}
/// How far a thing at a pack's side turns about its top end to lie along his back as posed.
pub fn side_pitch(g: &mut Game, p: SidePlace, c: CharId) -> f64 {
    let ch = g.ch[c].rig;
    let t = g.k.packside;
    if !memo_made(g, ch) { return 0.0; }
    let j3 = g.m.packside.memo.get(ch).unwrap().j3;
    let Some(pivot) = p.pivot else { return 0.0; };
    if j3 < 0.0 { return 0.0; }
    let a = j3 as usize * 16;
    let len = p.len.unwrap_or(0.8);
    let r = p.r.unwrap_or(0.0) + t.rest;
    let mut th = t.pitch1;
    let line = back_line_g(g, ch, pivot[0]);
    let jd = &g.ch[c].skel.joint_data;
    let d = |i: usize| jd[i] as f64;
    for q in line {
        let b = q.j as usize * 16;
        let x = pivot[0];
        let wx = d(b) * x + d(b + 4) * q.y + d(b + 8) * q.z + d(b + 12) - d(a + 12);
        let wy = d(b + 1) * x + d(b + 5) * q.y + d(b + 9) * q.z + d(b + 13) - d(a + 13);
        let wz = d(b + 2) * x + d(b + 6) * q.y + d(b + 10) * q.z + d(b + 14) - d(a + 14);
        let yy = d(a + 4) * wx + d(a + 5) * wy + d(a + 6) * wz - pivot[1];
        let zz = d(a + 8) * wx + d(a + 9) * wy + d(a + 10) * wz - pivot[2];
        let rho = m::hyp2(yy, zz);
        if yy > -0.06 || rho > len + 0.04 || rho < r { continue; }
        th = m::min(th, m::atan2(zz, -yy) - m::asin(r / rho));
    }
    m::max(t.pitch0, th)
}
/// A bind-space place turned `th` about its top end, in the pack's upright plane.
pub fn side_pitched(p: SidePlace, th: f64) -> SidePlace {
    let Some(v) = p.pivot else { return p; };
    if !js::truthy(th) { return p; }
    let (c, s) = (m::cos(th), m::sin(th));
    let dir = |d: V3| -> V3 { [d[0], d[1] * c + d[2] * s, d[2] * c - d[1] * s] };
    let rel = dir([p.o[0] - v[0], p.o[1] - v[1], p.o[2] - v[2]]);
    SidePlace { o: [v[0] + rel[0], v[1] + rel[1], v[2] + rel[2]], x: p.x, y: dir(p.y), z: dir(p.z), ..p }
}
/// A bind-space place carried by a joint's skin matrix.
pub fn side_skinned(p: SidePlace, jd: &[f32], at: f64) -> SidePlace {
    let at = at as usize;
    let d = |i: usize| jd[at + i] as f64;
    let pt = |v: V3| -> V3 { [d(0) * v[0] + d(4) * v[1] + d(8) * v[2] + d(12), d(1) * v[0] + d(5) * v[1] + d(9) * v[2] + d(13), d(2) * v[0] + d(6) * v[1] + d(10) * v[2] + d(14)] };
    let dir = |v: V3| -> V3 {
        let q = [d(0) * v[0] + d(4) * v[1] + d(8) * v[2], d(1) * v[0] + d(5) * v[1] + d(9) * v[2], d(2) * v[0] + d(6) * v[1] + d(10) * v[2]];
        let l = js::or(m::hyp3(q[0], q[1], q[2]), 1.0);
        [q[0] / l, q[1] / l, q[2] / l]
    };
    SidePlace { o: pt(p.o), x: dir(p.x), y: dir(p.y), z: dir(p.z), pivot: None, len: None, r: None }
}
/// The chest joint's place in the skin's list (−1: a skeleton without one).
pub fn chest_joint(g: &mut Game, ch: RigId) -> f64 {
    if !memo_made(g, ch) { return -1.0; }
    g.m.packside.memo.get(ch).unwrap().j3
}
/// A bind-space place in the chest joint's own space.
pub fn side_local(g: &mut Game, ch: RigId, p: SidePlace) -> Option<SidePlace> {
    let j = chest_joint(g, ch);
    if j < 0.0 { return None; }
    let ib = g.tab.rig[ch].skin.inverse_bind.clone();
    Some(side_skinned(p, &ib, j * 16.0))
}
