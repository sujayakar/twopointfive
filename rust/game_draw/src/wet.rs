//! WET GROUND: where water lies is one rule, what lies flat under open sky is wet while the ground is. In the order it is worked out: the ground slabs a picked rain falls on
//! (the lot itself is the weather's own, soaked in place by the rules); a finish laid on such a slab, with sky over most of it; a drawn-only sheet lying on the ground, which
//! follows the topmost flat under its middle. A box is wet whole or not at all (the renderer reads one word a box). How wet shows is what the thing is made of: what soaks takes
//! no film; bare metal does not darken. Nothing of the game reads any of it: the boxes' rows are written as the box dressed for the rain, and the box itself stays dry.
use alloc::vec::Vec;
use game::game::Game;
use game::ids::*;
use game::m;
use game::scene_boxes::{box_flag, Box, SHEET_MASK};
use game::scene_materials::Surf;
use game::weather::{WET_ALBEDO, WET_ROUGH};
use crate::boxes::Was;

/// how far over a wet slab's top a finish may lie and still be the ground (m), and the highest top of a flat a sheet may lie on
const WET_OVER: f64 = 0.02;
const FLAT_TOP: f64 = 0.06;

type Rect = [f64; 4];
fn top(b: &Box) -> f64 { b.c[1] + b.h[1] }
fn in_rect(r: &Rect, x: f64, z: f64) -> bool { x >= r[0] && z >= r[1] && x <= r[2] && z <= r[3] }
/// of 25 spots over its rectangle (turned with it), how many have no roof over them, counted no further than `enough`
fn sky_spots(b: &Box, roofs: &[Rect], enough: u32, step: f64) -> u32 {
    let (cs, sn) = (m::cos(b.yaw), m::sin(b.yaw));
    let mut n = 0;
    for i in 0..5 { for j in 0..5 {
        let (lx, lz) = (b.h[0] * (i as f64 - 2.0) * step, b.h[2] * (j as f64 - 2.0) * step);
        let (x, z) = (b.c[0] + cs * lx + sn * lz, b.c[2] - sn * lx + cs * lz);
        if !roofs.iter().any(|r| in_rect(r, x, z)) { n += 1; if n >= enough { return n; } }
    } }
    n
}
fn sky_over_most(b: &Box, roofs: &[Rect]) -> bool { sky_spots(b, roofs, 13, 0.4) >= 13 }
/// a ground slab a picked rain wets
fn open_ground(b: &Box, roofs: &[Rect]) -> bool {
    let t = top(b);
    if !(t > -0.011 && t < 0.003) || b.h[0] * b.h[2] < 4.0 || crate::boxes::pack::flags(b) & (box_flag::NO_PRIMARY | box_flag::GLASS | box_flag::DYNAMIC) != 0 { return false; }
    sky_spots(b, roofs, 1, 0.45) >= 1
}
/// a sheet lying on the ground: drawn only, thin, low, not under the ground
fn ground_sheet(b: &Box) -> bool { crate::boxes::pack::flags(b) & SHEET_MASK == box_flag::NO_SHADOW && b.h[1] <= 0.02 && top(b) <= 0.03 && b.c[1] - b.h[1] >= 0.0 }
/// a flat a sheet may lie on: solid, square to the world, low, broader than a kerb
fn ground_flat(b: &Box) -> bool { crate::boxes::pack::flags(b) & SHEET_MASK == 0 && b.yaw == 0.0 && top(b) <= FLAT_TOP && b.h[0] > 0.4 && b.h[2] > 0.4 }

/// THE GROUND THAT IS WET while the weather's ground is, of these statics, as their places in the list
fn wet_ground_of(g: &Game, statics: &[BoxId], roofs: &[Rect], lot: Option<BoxId>, picked: bool) -> Vec<u32> {
    let all = &g.tab.boxes;
    let mut wet = alloc::vec![false; statics.len()];
    let mut bases: Vec<&Box> = lot.map(|l| &all[l]).into_iter().collect();
    if picked {
        for (i, &id) in statics.iter().enumerate() { if Some(id) != lot && open_ground(&all[id], roofs) { wet[i] = true; bases.push(&all[id]); } }
        let on = |b: &Box, s: &Box| { let up = top(b) - top(s); up > 0.0 && up <= WET_OVER && m::abs(b.c[0] - s.c[0]) <= s.h[0] && m::abs(b.c[2] - s.c[2]) <= s.h[2] };
        for (i, &id) in statics.iter().enumerate() {
            let b = &all[id];
            if Some(id) != lot && !wet[i] && crate::boxes::pack::flags(b) & SHEET_MASK == 0 && bases.iter().any(|s| on(b, s)) && sky_over_most(b, roofs) { wet[i] = true; }
        }
    }
    let grounds: Vec<usize> = (0..statics.len()).filter(|&i| ground_flat(&all[statics[i]])).collect();
    let mut out: Vec<u32> = (0..statics.len() as u32).filter(|&i| wet[i as usize]).collect();
    for (i, &id) in statics.iter().enumerate() {
        let d = &all[id];
        if !ground_sheet(d) { continue; }
        let (mut under, mut at) = (None, -1e9);
        for &k in &grounds { let b = &all[statics[k]]; if m::abs(b.c[0] - d.c[0]) <= b.h[0] && m::abs(b.c[2] - d.c[2]) <= b.h[2] && top(b) > at { at = top(b); under = Some(k); } }
        if let Some(k) = under { if (Some(statics[k]) == lot || wet[k]) && sky_over_most(d, roofs) { out.push(i as u32); } }
    }
    out.sort_unstable();
    out
}

/// what the rule reads of the boxes, as one word: the list is worked out again when this moves, and only then
fn standing(g: &Game, statics: &[BoxId], lot: Option<BoxId>) -> u32 {
    let hi = lot.map_or(FLAT_TOP, |l| m::max(FLAT_TOP, top(&g.tab.boxes[l]) + WET_OVER));
    const P: u32 = 0x0100_0193;
    let eat = |w: u32, v: u32| (w ^ v).wrapping_mul(P);
    let q = |v: f64| (v * 65536.0) as i64 as u32;
    let mut w = statics.len() as u32;
    for (i, &id) in statics.iter().enumerate() {
        let b = &g.tab.boxes[id];
        if top(b) > hi { continue; }
        w = eat(w, i as u32); w = eat(w, crate::boxes::pack::flags(b) & SHEET_MASK); w = eat(w, q(b.yaw));
        for k in 0..3 { w = eat(w, q(b.c[k])); w = eat(w, q(b.h[k])); }
    }
    w
}

#[derive(Default)]
pub struct Wet {
    /// the statics that are wet by this rule, by their place, in order, each as last dressed and what it was dressed from
    list: Vec<u32>,
    dressed: Vec<Option<(Was, Box)>>,
    stood: Option<(u32, Vec<Rect>, bool, Option<BoxId>)>,
}

impl Wet {
    /// THE GROUND IS AS WET AS THE WEATHER SAYS, whatever stood before
    pub fn follow(&mut self, g: &Game) {
        let w = &g.weather;
        if !w.wet_on { self.list.clear(); self.dressed.clear(); self.stood = None; return; }
        let statics = &g.engine.world.statics;
        let (lot, picked) = (w.lot, game::weather::weather_level_roofs(g));
        let now = standing(g, statics, lot);
        if let Some(s) = &self.stood { if s.0 == now && s.1 == w.roof_rects && s.2 == picked && s.3 == lot { return; } }
        self.list = wet_ground_of(g, statics, &w.roof_rects, lot, picked);
        self.dressed.clear();
        self.dressed.resize_with(self.list.len(), || None);
        self.stood = Some((now, w.roof_rects.clone(), picked, lot));
    }
    /// is this static wet by this rule
    pub fn is(&self, i: u32) -> bool { !self.list.is_empty() && self.list.binary_search(&i).is_ok() }
    /// the box as the renderer has it while wet: darker where it is not bare metal, and with a water's film where it does not drink. `was` = the box itself as it stands: it is
    /// dressed again only where that moved, so the rows of a wet box are packed again only when it changed
    pub fn dressed(&mut self, g: &Game, i: u32, b: &Box, was: Was) -> &Box {
        let at = self.list.binary_search(&i).unwrap();
        if !matches!(&self.dressed[at], Some((w, _)) if *w == was) {
            let mut o = b.clone();
            if o.wet_k.is_none() {
                o.wet_k = Some(if b.metal == Some(true) { 1.0 } else { WET_ALBEDO });
                if !soaks(g, b) { o.flags = (crate::boxes::pack::flags(b) | box_flag::GLOSS) as f64; o.rough = Some(WET_ROUGH); o.aniso = Some(true); }
            }
            self.dressed[at] = Some((was, o));
        }
        &self.dressed[at].as_ref().unwrap().1
    }
}

/// does what this box is made of soak water up: no film on it, no puddle
fn soaks(g: &Game, b: &Box) -> bool { matches!(game::scene_materials::does_of(g, b.does).surf, Surf::Carpet | Surf::Fabric | Surf::Foliage | Surf::Cardboard) }
