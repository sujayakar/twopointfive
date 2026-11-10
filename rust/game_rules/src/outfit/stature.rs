#![allow(unused_imports, unused_variables)]
//! STATURE: the cast drawn at slightly different heights a man. LOOKS ONLY by its own word: the skeleton, the bones, the hit
//! capsules and every box a ray meets are untouched. What it writes that the tick's own state holds: `Box.dc / dh` of the boxes bolted to a man
//! (rows of the frame's list, where the picture is to draw them) and the near end of his weapon laser (a box of the frame's list made from the
//! DRAWN muzzle). Those are fields and boxes of the game's own tables, so they are written here, under the DRESS row, as the TypeScript writes them.
//!
//! LEFT TO THE PICTURE'S SIDE: the joint palette, the shadow proxies and the round-cone fit as drawn (`drawn_joints`, `drawn_capsules`, `drawn_fit`):
//! what the renderer is HANDED, which the Rust tick hands nobody.

pub use ::game_types::stature::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, vec::Vec};

fn hash01(n: f64) -> f64 {
    // (`(n | 0) * 0x9E3779B1` is a product of doubles, then `x ^= x >>> 15` takes it to an int32)
    let mut x = js::to_i32(js::to_i32(n) as f64 * 2654435761.0);
    x ^= ((x as u32) >> 15) as i32;
    x = m::imul(x, 0x85EBCA6Bu32 as i32);
    x ^= ((x as u32) >> 13) as i32;
    ((x as u32) % 4096) as f64 / 4095.0
}

/// The man's own stature (1 for a player's man, and for everybody with no cast loaded).
fn stature_base(g: &mut crate::game::Game, c_2: CharId) -> f64 {
    if crate::character::is_player(g, c_2) { return 1.0; }
    let Some(v) = crate::cast::cast_variant_of(g, c_2) else { return 1.0; };
    let t = stature_tune();
    let name = g.tab.anim_gltf_cast_variant[v].name.clone();
    let s = stature().get(name).copied().unwrap_or(1.0) + (hash01(g.ch[c_2].id) * 2.0 - 1.0) * t.jitter;
    m::round(m::min(t.max, m::max(t.min, s)) * 1000.0) / 1000.0
}

/// Two bodies that must meet: he is in a player's arm, or is the body one hauls.
fn paired(g: &crate::game::Game, c_2: CharId) -> bool {
    for pl in crate::seats::men(g) { if let Some(d) = g.pl[pl].dragging() { if g.gd[d].ch == c_2 { return true; } } }
    let rec = g.guards.iter().copied().find(|x| g.gd[*x].ch == c_2).or_else(|| g.civilians.iter().copied().find(|x| g.gd[*x].ch == c_2));
    match rec { Some(r) => g.gd[r].held.is_some(), None => false }
}

/// Once a frame, before anything of the men is drawn: every man's scale for this frame. False with no cast loaded.
pub fn stature_tick(g: &mut crate::game::Game, men: &[CharId], dt: f64) -> bool {
    if g.tab.rig[g.rig].cast.is_none() { return false; }
    for &c in men {
        let want = if paired(g, c) { 1.0 } else { stature_base(g, c) };
        match g.m.stature.shown.get(c).copied() {
            None => { g.m.stature.shown.set(c, want); }
            Some(had) => if had != want {
                let step = stature_tune().ease_per_sec * m::max(0.0, dt);
                g.m.stature.shown.set(c, if m::abs(want - had) <= step { want } else { had + m::sign(want - had) * step });
            }
        }
    }
    true
}

pub fn stature_of(g: &crate::game::Game, c_2: CharId) -> f64 { g.m.stature.shown.get(c_2).copied().unwrap_or(1.0) }

/// Where a point of the simulation's man is DRAWN: scaled about his feet.
fn drawn_point(g: &crate::game::Game, c_2: CharId, p: V3, s: Option<f64>) -> V3 {
    let s = match s { Some(s) => s, None => stature_of(g, c_2) };
    let f = g.ch[c_2].pos;
    if s == 1.0 { p } else { [f[0] + (p[0] - f[0]) * s, f[1] + (p[1] - f[1]) * s, f[2] + (p[2] - f[2]) * s] }
}

/// The boxes out[from‥] that carry a drawn man's owner byte (his rig, a can in his fist): where the picture is to draw them (Box.dc / dh). The box
/// itself is not moved.
pub fn drawn_boxes(g: &mut crate::game::Game, men: &[CharId], out: &Vec<AnyBoxId>, from: f64) {
    let mut any = false;
    for &c in men { if stature_of(g, c) != 1.0 { let i = (js::to_i32(g.ch[c].id) & 0xff) as usize; g.m.stature.by_id[i] = Some(c); any = true; } }
    let mut k = from as usize;
    while k < out.len() {
        let b = out[k]; k += 1;
        let owner = g.tab[b].owner;
        let c = if any && js::truthy(owner) { g.m.stature.by_id[(js::to_i32(owner) & 0xff) as usize] } else { None };
        let Some(c) = c else {   // (a box that outlives the frame, a row of Game.fx, whose man is back at 1)
            let x = &mut g.tab[b]; if x.dc.is_some() { x.dc = None; x.dh = None; }
            continue;
        };
        let s = stature_of(g, c); let (bc, bh) = { let x = &g.tab[b]; (x.c, x.h) };
        let dc = drawn_point(g, c, bc, Some(s));
        let x = &mut g.tab[b]; x.dc = Some(dc); x.dh = Some([bh[0] * s, bh[1] * s, bh[2] * s]);
    }
    if any { for x in g.m.stature.by_id.iter_mut() { *x = None; } }
}
