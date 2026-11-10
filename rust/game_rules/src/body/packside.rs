#![allow(unused_imports, unused_variables)]
pub use ::game_types::packside::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

fn bit_of(parts: &[String], name: &str) -> u32 {
    match parts.iter().position(|p| p == name) { Some(i) => 1u32 << i, None => 0 }
}
fn trunk_name(n: &str) -> bool { matches!(n, "DEF-hips" | "DEF-spine.001" | "DEF-spine.002" | "DEF-spine.003" | "DEF-thigh.L" | "DEF-thigh.R") }

/// What a rig's cast has of packs, and the back they lie on. None: no cast, a mesh with no parts, or a skeleton without a chest.
pub fn back_of(art: &crate::anim_gltf::Art, rig: &crate::anim_gltf::GltfCharacter) -> Option<Back> {
    let cast = &art.cast[rig.cast?];
    let md = &art.mesh[rig.mesh];
    let part = md.part.as_ref()?;
    let node = rig.node_by_name.of("DEF-spine.003").copied()?;
    let chest = rig.skin.joints.iter().position(|j| *j == node)?;
    let at = |v: usize| -> V3 { let p = &md.positions; [p[v * 3] as f64, p[v * 3 + 1] as f64, p[v * 3 + 2] as f64] };
    let mut packs: Vec<Option<PackExt>> = cast.parts.iter().map(|name| name.starts_with("kit_").then_some(PackExt { x0: m::INF, x1: -m::INF, top0: -m::INF, top1: -m::INF, z0: m::INF, z1: -m::INF })).collect();
    let kit = packs.iter().enumerate().filter(|(_, e)| e.is_some()).fold(0u32, |k, (i, _)| k | 1 << i);
    let behind = |v: usize| { let [_, y, z] = at(v); !(z > ZB || y < Y0 || y > Y1) };
    let n = md.vertex_count as usize;
    for v in (0..n).filter(|&v| behind(v)) {
        let Some(Some(e)) = packs.get_mut(part[v] as usize) else { continue };
        let [x, _, z] = at(v);
        if x < e.x0 { e.x0 = x; }
        if x > e.x1 { e.x1 = x; }
        if z < e.z0 { e.z0 = z; }
        if z > e.z1 { e.z1 = z; }
    }
    for v in (0..n).filter(|&v| behind(v)) {
        let Some(Some(e)) = packs.get_mut(part[v] as usize) else { continue };
        let [x, y, _] = at(v);
        if x < e.x0 + STRIP && y > e.top0 { e.top0 = y; }
        if x > e.x1 - STRIP && y > e.top1 { e.top1 = y; }
    }
    for e in packs.iter_mut() { if e.is_some_and(|x| !(x.x0 < x.x1)) { *e = None; } }
    let mut body: u32 = 0;
    for &v in cast.variants.iter() {
        let cv = &art.variant[v];
        if cv.side == crate::anim_gltf::CastSide::Operator { body |= js::to_u32(cv.mask); }
    }
    if body == 0 { body = 0xffff_ffff; }
    body &= !kit;
    let trunk: Vec<bool> = rig.skin.joints.iter().map(|n| *n >= 0.0 && rig.nodes.get(*n as usize).is_some_and(|x| trunk_name(&x.name))).collect();
    let joint = |i: usize| -> usize { match &md.joints { crate::anim_gltf::MeshDataJoints::Uint16Array(v) => v[i] as usize, crate::anim_gltf::MeshDataJoints::Uint8Array(v) => v[i] as usize } };
    let mut line: Vec<BackPt> = Vec::new();
    for v in 0..n {
        if (body >> (part[v] as u32 & 31)) & 1 == 0 { continue; }
        let [x, y, z] = at(v);
        if m::abs(x) > REACH + HALF || z > -0.02 || y < 0.45 || y > 1.62 { continue; }
        let mut b = 0;
        for k in 1..4 { if md.weights[v * 4 + k] > md.weights[v * 4 + b] { b = k; } }
        let j = joint(v * 4 + b);
        if trunk.get(j).copied().unwrap_or(false) { line.push(BackPt { j, x, y, z }); }
    }
    let ends = [back_line(&line, -REACH), back_line(&line, REACH)];
    Some(Back { chest, packs, kit, ram: bit_of(&cast.parts, "kit_breacher"), pack: bit_of(&cast.parts, "kit_rifleman"), line, ends })
}
/// The pack a man drawn with this part mask wears on his back; None with no cast, or none worn.
pub fn pack_of(g: &Game, ch: RigId, mask: f64) -> Option<PackExt> {
    let back = g.art.rig[ch].back.as_ref()?;
    let mask = js::to_u32(mask);
    if mask & back.kit == 0 { return None; }
    back.packs.iter().enumerate().find_map(|(i, e)| e.filter(|_| (mask >> (i as u32 & 31)) & 1 != 0))
}
/// The mask a man is drawn with while he carries a real tool on his back.
pub fn pack_mask(g: &Game, ch: RigId, mask: f64, carries: bool) -> f64 {
    let Some(back) = g.art.rig[ch].back.as_ref().filter(|_| carries) else { return mask };
    let mk = js::to_u32(mask);
    if back.ram == 0 || back.pack == 0 || mk & back.ram == 0 { return mask; }
    ((mk & !back.ram) | back.pack) as f64
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
/// His back's line under a lateral strip at bind x: the hindmost point of each band up it.
fn back_line(line: &[BackPt], xc: f64) -> Vec<BackPt> {
    let mut bands = [None::<BackPt>; BANDS];
    for p in line.iter().filter(|p| m::abs(p.x - xc) <= HALF) {
        let had = &mut bands[m::floor(p.y / BAND) as usize];
        if had.is_none_or(|h| p.z < h.z) { *had = Some(*p); }
    }
    bands.into_iter().flatten().collect()
}
/// How far a thing at a pack's side turns about its top end to lie along his back as posed.
pub fn side_pitch(g: &Game, p: SidePlace, c: CharId) -> f64 {
    let t = g.k.packside;
    let (Some(back), Some(pivot)) = (g.art.rig[g.ch[c].rig].back.as_ref(), p.pivot) else { return 0.0 };
    let a = back.chest * 16;
    let len = p.len.unwrap_or(0.8);
    let r = p.r.unwrap_or(0.0) + t.rest;
    let mut th = t.pitch1;
    let jd = &g.ch[c].skel.joint_data;
    let d = |i: usize| jd[i] as f64;
    let own;
    let strip: &[BackPt] = if pivot[0] <= -REACH { &back.ends[0] } else if pivot[0] >= REACH { &back.ends[1] } else { own = back_line(&back.line, pivot[0]); &own };
    for q in strip {
        let b = q.j * 16;
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
pub fn side_skinned(p: SidePlace, jd: &[f32], at: usize) -> SidePlace {
    let d = |i: usize| jd[at + i] as f64;
    let pt = |v: V3| -> V3 { [d(0) * v[0] + d(4) * v[1] + d(8) * v[2] + d(12), d(1) * v[0] + d(5) * v[1] + d(9) * v[2] + d(13), d(2) * v[0] + d(6) * v[1] + d(10) * v[2] + d(14)] };
    let dir = |v: V3| -> V3 {
        let q = [d(0) * v[0] + d(4) * v[1] + d(8) * v[2], d(1) * v[0] + d(5) * v[1] + d(9) * v[2], d(2) * v[0] + d(6) * v[1] + d(10) * v[2]];
        let l = js::or(m::hyp3(q[0], q[1], q[2]), 1.0);
        [q[0] / l, q[1] / l, q[2] / l]
    };
    SidePlace { o: pt(p.o), x: dir(p.x), y: dir(p.y), z: dir(p.z), pivot: None, len: None, r: None }
}
/// A bind-space place in the chest joint's own space.
pub fn side_local(g: &Game, ch: RigId, p: SidePlace) -> Option<SidePlace> {
    let rig = &g.art.rig[ch];
    Some(side_skinned(p, &rig.skin.inverse_bind, rig.back.as_ref()?.chest * 16))
}
