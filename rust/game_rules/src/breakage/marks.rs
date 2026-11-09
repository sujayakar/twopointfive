#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
//! THE MARKS AN ENCOUNTER LEAVES, one arm a place something hits something: which surface took it (a static, a door leaf, a piece of furniture: the last two ride the box
//! they are on), refusing the surfaces a mark would float on (light-only geometry, glowing fixtures, glass-thin trims). Nothing here draws from the game's die.

pub use ::game_types::marks::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::vec::Vec;

#[allow(unused_imports)]
use crate::out_events::*;
use crate::render_decals::{mark_asked, Ask};

/// (src/game/marks.ts:39 bulletMark)
pub fn bullet_mark(g: &mut crate::game::Game, hit_p: V3, n: V3, s: Option<crate::marks::Surface2>, def: &crate::weapons::WeaponDef) { let e = OutBulletMark { hit_p, n, s, def: def.id }; mark_asked(g, Ask::Bullet(e)); }

/// (src/game/marks.ts:51 bangScorch)
pub fn bang_scorch(g: &mut crate::game::Game, at: V3) { let e = OutBangScorch { at }; mark_asked(g, Ask::Bang(e)); }

/// (src/game/marks.ts:90 kickScuff)
pub fn kick_scuff(g: &mut crate::game::Game, d: DoorId, from: V3) { let e = OutKickScuff { d, from }; mark_asked(g, Ask::Kick(e)); }

/// (src/game/marks.ts:104 chipMark)
pub fn chip_mark(g: &mut crate::game::Game, p: V3, n: V3, s: Option<crate::marks::Surface2>, size: Option<f64>) { let e = OutChipMark { p, n, s, size }; mark_asked(g, Ask::Chip(e)); }

use crate::render_decals::{lay, mark_set, DecalKind, DecalOpts};
use crate::scene_boxes::{box_flag, Box as WorldBox};

/// col.raycast's index → the box it names: a static by its place in the level's list, a door leaf from 1e6 up.   (marks.ts staticSurface)
pub fn static_surface(g: &crate::game::Game, index: f64) -> Option<crate::marks::Surface2> {
    if index >= 1e6 { return g.col.dynamic_boxes.get((index - 1e6) as usize).map(|b| crate::marks::Surface2 { box_: *b, dynamic: true }); }
    if index < 0.0 { return None; }
    g.col.boxes.get(index as usize).map(|b| crate::marks::Surface2 { box_: *b, dynamic: false })
}

/// 0‥1 from any number (a hash, not a die)
fn unit(x: f64) -> f64 { let t = m::sin(x * 12.9898 + 78.233) * 43758.5453; t - m::floor(t) }

/// the two in-plane half extents of the face of `b` whose normal is `n` (world). (A box with a rotation takes `n` as its own.)
fn face_half_extents(b: &WorldBox, n: V3) -> [f64; 2] {
    let (cs, sn) = (m::cos(b.yaw), m::sin(b.yaw));
    let l = if b.rot.is_some() { n } else { [cs * n[0] - sn * n[2], n[1], sn * n[0] + cs * n[2]] };
    let (x, y, z) = (m::abs(l[0]), m::abs(l[1]), m::abs(l[2]));
    if x >= y && x >= z { [b.h[1], b.h[2]] } else if y >= z { [b.h[0], b.h[2]] } else { [b.h[0], b.h[1]] }
}
/// may a mark sit on this box: not light-only or ray-invisible geometry, not a glowing fixture, not a sliver thinner than a centimetre on its narrow faces
fn markable(b: &WorldBox, n: V3) -> bool {
    let f = box_flag();
    if js::to_u32(b.flags) & js::to_u32(f.no_primary + f.no_shadow) != 0 { return false; }
    if b.emissive[0] + b.emissive[1] + b.emissive[2] > 0.0 { return false; }
    face_half_extents(b, n).iter().all(|e| *e > 0.006)
}
/// the leading number of a word ('5.7×28 mm' → 5.7)
fn leading_number(s: &str) -> Option<f64> {
    let end = s.char_indices().find(|(_, c)| !(c.is_ascii_digit() || *c == '.')).map_or(s.len(), |(i, _)| i);
    s[..end].parse().ok()
}

/// A round stopped in a surface: a hole sized by its calibre, or, in solid glass that says so, a crack. Flesh, the sky and what `markable` refuses leave nothing.
pub(super) fn lay_bullet(g: &mut Game, e: &OutBulletMark) {
    let Some(s) = e.s else { return };
    if !markable(&g.tab.boxes[s.box_], e.n) { return; }
    let def = g.m.weapons.weapons.get(e.def).unwrap();
    let (mm, rifle, pellets) = (leading_number(&def.calibre).filter(|x| *x != 0.0).unwrap_or(6.0), def.cls != crate::weapons::WeaponClass::Pistol, def.pellets.is_some());
    let attach = if s.dynamic { Some(s.box_) } else { None };
    let b = &g.tab.boxes[s.box_];
    if b.stuff == Some(crate::scene_stuff::BoxStuff::Glass) && js::to_u32(b.flags) & js::to_u32(box_flag().glass) == 0 {
        let k = crate::particle_fx::solid_glass_crack();
        lay(g, DecalKind::Damage, e.hit_p, e.n, Some(if pellets { k.pellet } else if rifle { k.rifle } else { k.pistol }), &DecalOpts { param: Some(k.look + k.strength), clamp_to: Some(s.box_), attach, ..Default::default() });
        return;
    }
    let param = m::max(0.75, m::min(1.6, mm / 6.0)) * if rifle { 1.1 } else { 1.0 };
    lay(g, DecalKind::Bullet, e.hit_p, e.n, Some(if rifle { 0.031 } else { 0.025 }), &DecalOpts { param: Some(param), clamp_to: Some(s.box_), attach, ..Default::default() });
}

/// Something small and hard met a surface: a pale chip out of the finish.
pub(super) fn lay_chip(g: &mut Game, e: &OutChipMark) {
    let Some(s) = e.s else { return };
    if !markable(&g.tab.boxes[s.box_], e.n) { return; }
    lay(g, DecalKind::Chip, e.p, e.n, Some(e.size.unwrap_or(0.013)), &DecalOpts { clamp_to: Some(s.box_), attach: if s.dynamic { Some(s.box_) } else { None }, ..Default::default() });
}

fn count(g: &Game) -> f64 { mark_set(g).map_or(0.0, |s| s.list.len() as f64) }

/// The stun charge went off at `at`: the big scorch on whatever it lay on (a chair's seat: that one rides the chair), and weaker splashes up what stands within a metre.
pub(super) fn lay_bang(g: &mut Game, at: V3) {
    if !g.decals.enabled { return; }
    let from = [at[0], at[1] + 0.25, at[2]];
    let down = [0.0, -1.0, 0.0];
    // what it lay on: the nearer of a static or leaf under it and a prop's part under it
    let st = crate::scene_collision::raycast(g, from, down, 1.5);
    let ph = crate::props::prop_system_raycast(g, from, down, 1.5);
    match (&ph, &st) {
        (Some(p), _) if st.as_ref().map_or(true, |s| p.t < s.t) => {
            let e = face_half_extents(&g.tab.boxes[p.box_], p.n);
            if markable(&g.tab.boxes[p.box_], p.n) { lay(g, DecalKind::Scorch, v3::mad(from, down, p.t), p.n, Some(m::min(0.55, m::max(e[0], e[1]) * 1.15)), &DecalOpts { param: Some(1.0), attach: Some(p.box_), ..Default::default() }); }
        }
        (_, Some(h)) => {
            if let Some(s) = static_surface(g, h.index) {
                if markable(&g.tab.boxes[s.box_], h.n) { let size = 0.62 + 0.28 * unit(count(g) * 7.31); lay(g, DecalKind::Scorch, v3::mad(from, down, h.t), h.n, Some(size), &DecalOpts { param: Some(1.0), attach: if s.dynamic { Some(s.box_) } else { None }, ..Default::default() }); }
            }
        }
        _ => {}
    }
    // the splash: sixteen looks out level from the burst; each wall, leaf or prop face met within a metre takes ONE smaller, weaker scorch, at the nearest point any look found on that box
    let eye = [at[0], m::max(at[1], 0.06) + 0.06, at[2]];
    let mut nearest: Vec<(BoxId, f64, V3, V3, bool)> = Vec::new();
    for i in 0..16 {
        let a = (i as f64 / 16.0) * m::PI * 2.0;
        let dir = [m::cos(a), 0.0, m::sin(a)];
        let sh = crate::scene_collision::raycast(g, eye, dir, 1.0);
        let pp = crate::props::prop_system_raycast(g, eye, dir, 1.0);
        let (t, n, s) = match (&pp, &sh) {
            (Some(p), _) if sh.as_ref().map_or(true, |h| p.t < h.t) => (p.t, p.n, Some(crate::marks::Surface2 { box_: p.box_, dynamic: true })),
            (_, Some(h)) => (h.t, h.n, static_surface(g, h.index)),
            _ => continue,
        };
        let Some(s) = s else { continue };
        if m::abs(n[1]) > 0.3 || !markable(&g.tab.boxes[s.box_], n) { continue; }
        let at = v3::mad(eye, dir, t);
        match nearest.iter().position(|x| x.0 == s.box_) {
            Some(i) => { if nearest[i].1 > t { nearest[i] = (s.box_, t, at, n, s.dynamic); } }
            None => nearest.push((s.box_, t, at, n, s.dynamic)),
        }
    }
    nearest.sort_by(|a, b| a.1.total_cmp(&b.1));
    let mut placed: Vec<V3> = Vec::new();
    for (b, t, p, n, dynamic) in nearest {
        if placed.iter().any(|q| v3::dist(*q, p) < 0.5) { continue; }   // (two boxes meeting at a corner right by the can: one splash does for both)
        placed.push(p);
        let k = 1.0 - t;
        let on = if dynamic { Some(b) } else { None };
        lay(g, DecalKind::Scorch, [p[0], p[1] + 0.22 * k, p[2]], n, Some(0.22 + 0.34 * k), &DecalOpts { param: Some(0.35 + 0.6 * k), attach: on, clamp_to: on, ..Default::default() });
    }
}

/// A boot through the lock of a leaf from `from`'s side: a rubber scuff by the lock at kicking height on that face, riding the leaf as it flies.
pub(super) fn lay_kick(g: &mut Game, d: DoorId, from: V3) {
    if !g.decals.enabled { return; }
    let side = crate::doors::doors_side(g, d, from);
    let u = crate::doors::door_dir(g, d);
    let (w, hinge, thick, leaf) = (g.dr[d].def.width, g.dr[d].def.hinge, g.dr[d].thick, g.dr[d].box_);
    let along = m::max(w * 0.5, m::min(w - 0.2, (from[0] - hinge[0]) * u[0] + (from[2] - hinge[1]) * u[1]));   // by the lock, on the outer half
    let n = [-u[1] * side, 0.0, u[0] * side];   // out of the leaf on the kicker's side
    let k = unit(count(g) * 3.77 + along);
    let p = [hinge[0] + u[0] * along + n[0] * (thick / 2.0), 0.8 + 0.12 * k, hinge[1] + u[1] * along + n[2] * (thick / 2.0)];
    let slant = -0.25 - 0.35 * unit(k * 5.1);   // the sole comes across and down a little
    let t = v3::normalize([u[0] * side, slant, u[1] * side]);
    lay(g, DecalKind::Scuff, p, n, None, &DecalOpts { param: Some(0.13 + 0.06 * k), tangent: Some(t), attach: Some(leaf), clamp_to: Some(leaf), ..Default::default() });
}
