//! src/game/glasspen.ts lines 1 to 401
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- local helpers

/// `g.level.boxes[bi]`: an index with no box is undefined there, None here. That list and `g.col.boxes` are one array in the TypeScript, and a static's index is its BoxId's row
fn level_box(g: &mut Game, bi: f64) -> Option<BoxId> {
    let ix = bi as usize;
    if ix as f64 != bi || ix >= g.col.boxes.len() { return None; }
    Some(BoxId::from_ix(ix))
}
/// half the sheet's thickness: `Math.min(b.h[0], b.h[2])`
fn thin_half(g: &mut Game, b: BoxId) -> f64 {
    let h = g.col.boxes[b].h;
    m::min(h[0], h[2])
}
/// `g.col.glassOpen && g.col.glassOpen(bi, p)`. The one rule hung is glassbreak's (glassbreak.ts:213): open where a piece has fallen, or where the burst's view in hand sees no glass
fn pane_open(g: &mut Game, bi: f64, p: V3) -> bool {
    if crate::scene_collision::glass_open(g).is_none() { return false; }
    if crate::glassbreak::glass_break_open_at(g, bi, p) { return true; }
    match g.glass_break.also.clone() {
        None => false,
        Some(crate::glassbreak::GlassBreakAlso::Blown) => crate::glassbreak::glass_break_going_at(g, bi, p),
        Some(crate::glassbreak::GlassBreakAlso::Wary { from, size }) => crate::glassbreak::glass_break_would_give(g, bi, from, Some(size), None),
        Some(crate::glassbreak::GlassBreakAlso::Counted { from, size }) => crate::glassbreak::glass_break_would_give(g, bi, from, Some(size), Some(p)),
    }
}
/// `GLASSPEN.energy[id] ?? GLASSPEN.byClass[cls] ?? GLASSPEN.defaultEnergy`
fn energy_by(k: &crate::knobs::GlasspenK, id: crate::weapons::WeaponId, cls: Option<crate::weapons::WeaponClass>) -> f64 {
    if let Some(&e) = k.energy.get(String::from(id.as_str())) { return e; }
    if let Some(c) = cls {
        if let Some(&e) = k.by_class.get(String::from(c.as_str())) { return e; }
    }
    k.default_energy
}
/// a gun's round: a gun names its row of WEAPONS by ID, and the row's class is read there, so nothing is cloned
fn energy_of_row(g: &mut Game, id: crate::weapons::WeaponId) -> f64 {
    let cls = g.m.weapons.weapons.get(id).map(|d| d.cls);
    energy_by(&g.k.glasspen, id, cls)
}
/// `g.col.raycast(ro, rd, tmax)` in this file's shape of a hit
fn cast(g: &mut Game, ro: V3, rd: V3, tmax: f64) -> Option<StaticHit2> {
    crate::scene_collision::raycast(g, ro, rd, tmax).map(|h| StaticHit2 { t: h.t, index: h.index, n: h.n })
}
/// a new state: no holes, no log, rev 0 (glasspen.ts:169)
fn fresh_state() -> PenState { PenState { holes: JsMap::new(), log: Vec::new(), rev: 0.0 } }
/// one step off the seed the star's look is drawn from (glasspen.ts:227)
fn turn_seed(s: f64) -> u32 {
    let mixed = js::to_i32(s) ^ ((js::to_u32(s) >> 15) as i32);
    js::to_u32(m::imul(mixed, 0x2C1B3C6D) as f64 + 0x297A2D39 as f64)
}
/// the turn once the cone `a` is known and the seed is hashed (glasspen.ts:228..234)
fn cone_out(s: u32, a: f64, d: V3, t: &PaneToll, bend_k: f64) -> V3 {
    let u1 = (s & 0xffff) as f64 / 65535.0;
    let u2 = (s >> 16) as f64 / 65535.0;
    let up: V3 = if m::abs(d[1]) < 0.95 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
    let e1 = v3::normalize(v3::cross(up, d));
    let e2 = v3::cross(d, e1);
    let r = m::tan(a * m::sqrt(u1));
    let phi = u2 * m::PI * 2.0;
    let (cp, sp) = (m::cos(phi), m::sin(phi));
    let mut out: V3 = [
        d[0] + (e1[0] * cp + e2[0] * sp) * r,
        d[1] + (e1[1] * cp + e2[1] * sp) * r,
        d[2] + (e1[2] * cp + e2[2] * sp) * r,
    ];
    let sin = m::sqrt(m::max(0.0, 1.0 - t.cos * t.cos));
    if sin > 1e-4 {
        let k = m::tan(a * bend_k * sin) / sin;
        let (px, py, pz) = (t.n[0] - d[0] * t.cos, t.n[1] - d[1] * t.cos, t.n[2] - d[2] * t.cos);
        out = [out[0] + px * k, out[1] + py * k, out[2] + pz * k];
    }
    v3::normalize(out)
}
/// a pane's crossings (x, y, z each): is `p` within r of one (glasspen.ts:177)
fn near_in(h: &[f64], p: V3, r2: f64) -> bool {
    let mut i = 0;
    while i < h.len() {
        let (dx, dy, dz) = (h[i] - p[0], h[i + 1] - p[1], h[i + 2] - p[2]);
        if dx * dx + dy * dy + dz * dz < r2 { return true; }
        i += 3;
    }
    false
}
/// a crossing onto a pane's list, the oldest going past `holes` (glasspen.ts:182)
fn push_hole(h: &mut Vec<f64>, p: V3, holes: f64) {
    h.push(p[0]);
    h.push(p[1]);
    h.push(p[2]);
    if h.len() as f64 > holes * 3.0 { h.drain(0..3); }
}
/// the shell rule on one pane's marks, five numbers each: x, y, z, went-through, filed-a-mark (glasspen.ts:292..300)
fn mark_on(mk: &mut Vec<f64>, at: V3, through: bool, r2: f64) -> ShellMark {
    let (mut marks, mut join) = (0, false);
    let mut i = 0;
    while i < mk.len() {
        let (x, y, z, went, filed) = (mk[i], mk[i + 1], mk[i + 2], mk[i + 3], mk[i + 4]);
        i += 5;
        if !js::truthy(filed) { continue; }
        marks += 1;
        if through && !js::truthy(went) { continue; }
        let (dx, dy, dz) = (x - at[0], y - at[1], z - at[2]);
        if dx * dx + dy * dy + dz * dz < r2 { join = true; break; }
    }
    mk.push(at[0]);
    mk.push(at[1]);
    mk.push(at[2]);
    mk.push(if through { 1.0 } else { 0.0 });
    mk.push(if join { 0.0 } else { 1.0 });
    if join { ShellMark::Join } else if marks == 0 { ShellMark::First } else { ShellMark::More }
}
/// `GLASSPEN_HOOKS.crossed?.(g, x, who)`: the game hangs one value (game.ts:1356), the exit jet of shards.ts
fn crossed_hook(g: &mut Game, x: &PaneCross, who: f64) {
    let like = crate::shards::PaneCrossLike { bi: x.bi, type_: x.type_, exit: x.exit, dir_out: x.dir_out, take: x.take, e_out: x.e_out, hole: x.hole };
    crate::shards::glass_shards_crossed(g, like, who);
}

// ---------------------------------------------------------------- host work: nothing here on the page's path

/// the switch kept per machine: browser storage, and the rows it wrote are settled (glasspen.ts loadGlassPen)
pub fn load_glass_pen(g: &mut crate::game::Game) {}

/// the panel's save to browser storage (glasspen.ts saveGlassPen)
pub fn save_glass_pen(g: &mut crate::game::Game) {}

/// the harness's stand-up put the switch off; the row is settled on (glasspen.ts glassPenHarness)
pub fn glass_pen_harness(g: &mut crate::game::Game) {}

// ---------------------------------------------------------------- the switch, a round's energy

/// the one gate: no showcase running (glasspen.ts glassPenLive)
pub fn glass_pen_live(g: &mut crate::game::Game) -> bool { !g.touring }

/// a round's energy as it leaves this row's muzzle (glasspen.ts roundEnergy)
pub fn round_energy(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> f64 {
    energy_by(&g.k.glasspen, def.id, Some(def.cls))
}

/// …and of the gun in this man's hands (glasspen.ts roundEnergyOf)
pub fn round_energy_of(g: &mut crate::game::Game, c_2: CharId) -> f64 {
    if let Some(pl) = crate::seats::man_of(g, Some(c_2)) {
        let gun = match crate::game::player_gun_in_hand(g, pl) { Some(f) => f, None => g.pl[pl].pistol };
        let def = g.fa[gun].def;
        return energy_of_row(g, def);
    }
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if g.gd[gd].ch == c_2 {
            let def = g.fa[g.gd[gd].weapon].def;
            return energy_of_row(g, def);
        }
    }
    g.k.glasspen.default_energy
}

/// the energy a line-of-fire question about a SIDE is asked with: the model's default, foesThrough being on (glasspen.ts sideShotEnergy)
pub fn side_shot_energy(g: &mut crate::game::Game, team: crate::teams::Team) -> Option<f64> { None }

/// the energy a line-of-fire question about THIS man's gun is asked with (glasspen.ts shotEnergy)
pub fn shot_energy(g: &mut crate::game::Game, c_2: CharId, def: Option<&crate::weapons::WeaponDef>) -> Option<f64> {
    if !glass_pen_live(g) { return None; }
    Some(match def { Some(d) => round_energy(g, d), None => round_energy_of(g, c_2) })
}

/// the pane's type, the fracture model's own word for it (glasspen.ts paneType)
pub fn pane_type(g: &mut crate::game::Game, b: BoxId, bi: Option<f64>) -> crate::glassbreak::GlassType {
    let bi = match bi { Some(i) => i, None => b.ix() as f64 };
    match crate::glassbreak::glass_break_type_of_pane(g, bi) {
        Some(t) => t,
        None => crate::glassbreak::glass_break_type_of(g, b, None),
    }
}

/// unit normal of a sheet: its thin axis in the world, sign arbitrary (glasspen.ts paneN)
pub fn pane_n(g: &mut crate::game::Game, b: BoxId) -> V3 {
    let (yaw, h) = (g.col.boxes[b].yaw, g.col.boxes[b].h);
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    if h[0] < h[2] { [cs, 0.0, -sn] } else { [sn, 0.0, cs] }
}

/// what a round leaving the glass at `p` along `d` is born inside: -1 open air, -2 a static it stops in, else another sheet's index (glasspen.ts bornIn)
pub fn born_in(g: &mut crate::game::Game, p: V3, d: V3, self_: f64) -> f64 {
    let mut embed = embed_scratch();
    let n = crate::scene_collision::gather(g, p[0] - 0.02, p[2] - 0.02, p[0] + 0.02, p[2] + 0.02, p[1] - 0.02, p[1] + 0.02, &mut embed);
    let no_shadow = js::to_i32(crate::scene_boxes::box_flag().no_shadow);
    let mut sheet = -1.0;
    for i in 0..(n as usize) {
        let bi = embed[i] as f64;
        if bi == self_ { continue; }
        let b = BoxId::from_ix(embed[i] as usize);
        if js::to_i32(g.col.boxes[b].flags) & no_shadow != 0 { continue; }
        let s = match crate::scene_collision::line_box(g, p, d, b) { Some(s) => s, None => continue };
        if s.depth < 0.0 || s.t1 <= 0.0 { continue; }
        if crate::glass::is_sheet(g, Some(b)) {
            if !pane_open(g, bi, p) { sheet = bi; }
        } else {
            return -2.0;
        }
    }
    sheet
}

// ---------------------------------------------------------------- holes

/// this game's holes, made if none yet: the row itself (glasspen.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::glasspen::PenState {
    g.m.glasspen.states.get_or_insert_with(fresh_state)
}

/// the holes go with the stars (glasspen.ts resetGlassPen)
pub fn reset_glass_pen(g: &mut crate::game::Game) {
    if let Some(s) = g.m.glasspen.states.as_mut() {
        s.holes.clear();
        s.log.clear();
        s.rev += 1.0;
    }
}

/// a number that moves whenever what a round has to pay at any pane may have moved (glasspen.ts glassTollSig)
pub fn glass_toll_sig(g: &mut crate::game::Game) -> f64 {
    let rev = match g.m.glasspen.states.as_ref() { Some(s) => s.rev, None => 0.0 };
    rev * 65537.0 + crate::glassbreak::glass_break_open_sig(g)
}

/// is `p` within holeR of an earlier crossing of pane `bi` (glasspen.ts nearHole)
pub fn near_hole(g: &mut crate::game::Game, bi: f64, p: V3) -> bool {
    let hole_r = g.k.glasspen.hole_r;
    let h = match g.m.glasspen.states.as_ref().and_then(|s| s.holes.get(crate::Num(bi))) { Some(h) => h, None => return false };
    near_in(h, p, hole_r * hole_r)
}

/// remember a crossing on pane `bi`, the oldest going past `holes` (glasspen.ts addHole)
pub fn add_hole(g: &mut crate::game::Game, bi: f64, p: V3) {
    let holes = g.k.glasspen.holes;
    let s = state_of(g);
    push_hole(s.holes.get_or_insert_with(crate::Num(bi), Vec::new), p, holes);
    s.rev += 1.0;
}

/// how many through-holes pane `bi` carries (glasspen.ts holeCount)
pub fn hole_count(g: &mut crate::game::Game, bi: f64) -> f64 {
    let n = match g.m.glasspen.states.as_ref().and_then(|s| s.holes.get(crate::Num(bi))) { Some(h) => h.len() as f64, None => 0.0 };
    n / 3.0
}

// ---------------------------------------------------------------- the toll

/// what sheet `bi` takes from a round of energy `e` meeting it at `at` flying `d`, or None: spent in it (glasspen.ts paneToll)
pub fn pane_toll(g: &mut crate::game::Game, bi: f64, at: V3, d: V3, e: f64, nth: Option<f64>) -> Option<crate::glasspen::PaneToll> {
    let nth = nth.unwrap_or(0.0);
    let b = level_box(g, bi);
    if !crate::glass::is_sheet(g, b) || nth >= g.k.glasspen.max_panes { return None; }
    let b = b?;
    let mut n = pane_n(g, b);
    let mut cos = v3::dot(n, d);
    if cos < 0.0 { n = v3::scale(n, -1.0); cos = -cos; }
    if cos < g.k.glasspen.graze_cos { return None; }
    let hole = near_hole(g, bi, at);
    let type_ = pane_type(g, b, Some(bi));
    let half = thin_half(g, b);
    let k = &g.k.glasspen;
    let cost = k.cost.get(type_).copied().unwrap_or(f64::NAN);
    let take = cost * (2.0 * half / k.ref_thick) * m::min(k.oblique_max, 1.0 / cos) * (if hole { k.hole_k } else { 1.0 });
    let e_out = e - take;
    if e_out < k.min_energy { return None; }
    Some(PaneToll { type_, n, cos, take, e_out, hole })
}

/// the turn: toward the normal by the slant's share, plus the hashed scatter; unit vector out (glasspen.ts deflect)
pub fn deflect(g: &mut crate::game::Game, b: BoxId, at: V3, d: V3, t: crate::glasspen::PaneToll, e_in: f64) -> V3 {
    let (deg, bend_k) = (g.k.glasspen.deflect_deg, g.k.glasspen.bend_k);
    let a = deg * m::PI / 180.0 * v3::clamp(t.take / m::max(e_in, 1e-6), 0.0, 1.0);
    if a < 1e-6 { return d; }
    let s = turn_seed(crate::glass::hit_seed(g, b, at));
    cone_out(s, a, d, &t, bend_k)
}

/// one pane on a round's flight: what it takes and where the round leaves it, or None: spent in it (glasspen.ts paneCrossing)
pub fn pane_crossing(g: &mut crate::game::Game, bi: f64, o: V3, d: V3, t: f64, e: f64, nth: f64) -> Option<crate::glasspen::PaneCross> {
    let at = v3::mad(o, d, t);
    let toll = pane_toll(g, bi, at, d, e, Some(nth))?;
    let b = level_box(g, bi)?;
    let te = match crate::scene_collision::line_box(g, at, d, b) { Some(s) => s.t1, None => -1.0 };
    let thru = (if te > 0.0 { te } else { 2.0 * thin_half(g, b) / toll.cos }) + EXIT_EPS;
    let exit = v3::mad(at, d, thru);
    let into = born_in(g, exit, d, bi);
    if into == -2.0 { return None; }
    let dir_out = if into >= 0.0 { d } else { deflect(g, b, at, d, toll, e) };
    let turn_deg = m::acos(v3::clamp(v3::dot(d, dir_out), -1.0, 1.0)) * 180.0 / m::PI;
    Some(PaneCross {
        bi, t, at, exit, dir_in: d, dir_out, e_in: e, turn_deg, thru, into,
        type_: toll.type_, n: toll.n, cos: toll.cos, take: toll.take, e_out: toll.e_out, hole: toll.hole,
    })
}

// ---------------------------------------------------------------- the round's flight

/// the flight of a round over statics and leaves, or None: it crosses no pane. `first` None = cast here (glasspen.ts roundPath)
pub fn round_path(g: &mut crate::game::Game, from: V3, dir: V3, range: f64, energy: f64, first: Option<crate::glasspen::StaticHit2>, nth0: Option<f64>) -> Option<crate::glasspen::RoundPath> {
    let nth0 = nth0.unwrap_or(0.0);
    let mut hit = match first { Some(h) => Some(h), None => cast(g, from, dir, range) };
    let h0 = hit?;
    if h0.index >= 1e6 { return None; }
    let b0 = level_box(g, h0.index);
    if !crate::glass::is_sheet(g, b0) { return None; }
    let mut segs: Vec<RoundSeg> = Vec::new();
    let mut cross: Vec<PaneCross> = Vec::new();
    let (mut o, mut d, mut t0, mut e) = (from, dir, 0.0, energy);
    loop {
        let h = match hit {
            Some(h) => h,
            None => {
                segs.push(RoundSeg { o, d, t0, len: range - t0, e });
                return if !cross.is_empty() { Some(RoundPath { segs, cross, st: None, len: range }) } else { None };
            }
        };
        let x = if h.index < 1e6 { pane_crossing(g, h.index, o, d, h.t, e, nth0 + cross.len() as f64) } else { None };
        segs.push(RoundSeg { o, d, t0, len: h.t, e });
        let mut x = match x {
            Some(x) => x,
            None => {
                return if !cross.is_empty() {
                    Some(RoundPath { segs, cross, st: Some(RoundPathSt { t: t0 + h.t, index: h.index, n: h.n }), len: t0 + h.t })
                } else { None };
            }
        };
        x.t = t0 + h.t;
        cross.push(x);
        t0 = x.t + x.thru;
        o = x.exit;
        d = x.dir_out;
        e = x.e_out;
        if t0 >= range { return Some(RoundPath { segs, cross, st: None, len: range }); }
        hit = if x.into >= 0.0 { Some(StaticHit2 { t: 0.0, index: x.into, n: v3::scale(d, -1.0) }) } else { cast(g, o, d, range - t0) };
    }
}

/// the leg holding path distance `t` (glasspen.ts segAt)
pub fn seg_at(p: &crate::glasspen::RoundPath, t: f64) -> crate::glasspen::RoundSeg {
    let mut s = p.segs[0];
    for k in p.segs.iter() {
        if k.t0 > t { break; }
        s = *k;
    }
    s
}

/// the point `t` metres along the path (glasspen.ts pathAt)
pub fn path_at(p: &crate::glasspen::RoundPath, t: f64) -> V3 {
    let s = seg_at(p, t);
    v3::mad(s.o, s.d, v3::clamp(t - s.t0, 0.0, s.len))
}

/// the impact call, one per pane a round meets, entry side (glasspen.ts paneImpact)
pub fn pane_impact(g: &mut crate::game::Game, bi: f64, at: V3, dir: V3, energy: f64, who: f64, through: bool, quiet: Option<bool>) {
    let e = energy * g.k.glasspen.fracture_k;
    crate::glass::glass_impact(g, bi, at, Some(dir), e, who, Some(crate::glass::GlassImpactCause::Shot), Some(quiet.unwrap_or(false)), Some(crate::glass::GlassImpactOpts { through: Some(through) }));
}

/// the shell rule's one question: did this pellet land on a mark its own shell made, else book it (glasspen.ts shellMark)
pub fn shell_mark(g: &mut crate::game::Game, shell: &mut crate::glasspen::ShellGlass, bi: f64, at: V3, through: bool) -> crate::glasspen::ShellMark {
    let shell_r = g.k.glasspen.shell_r;
    mark_on(shell.glass.get_or_insert_with(crate::Num(bi), Vec::new), at, through, shell_r * shell_r)
}

/// the round got as far as `t_hit` metres of path: every pane it crossed before that is filed (glasspen.ts commitCrossings)
pub fn commit_crossings(g: &mut crate::game::Game, cross: &[crate::glasspen::PaneCross], t_hit: f64, who: f64, shell: Option<&mut crate::glasspen::ShellGlass>) {
    let mut shell = shell;
    for x in cross.iter() {
        if x.t > t_hit + 1e-6 { break; }
        let mark = match shell.as_deref_mut() { Some(s) => shell_mark(g, s, x.bi, x.at, true), None => ShellMark::First };
        if mark == ShellMark::Join {
            g.stats.glasspen.crossed += 1;
            g.stats.glasspen.shell_joined += 1;
            if x.hole { g.stats.glasspen.via_hole += 1; }
            crate::glassbreak::glass_break_widen(g, x.bi, x.at);
            continue;
        }
        pane_impact(g, x.bi, x.at, x.dir_in, x.e_in, who, true, Some(mark == ShellMark::More));
        add_hole(g, x.bi, x.at);
        g.stats.glasspen.crossed += 1;
        if x.hole { g.stats.glasspen.via_hole += 1; }
        let line = format!(
            "{:.1}s pane {} ({}) crossed{}: energy {:.2} → {:.2}, turned {:.2}°{}",
            g.time, x.bi, x.type_.as_str(), if x.hole { " by its hole" } else { "" }, x.e_in, x.e_out, x.turn_deg,
            if js::truthy(who) { format!(" (who {})", who) } else { String::new() },
        );
        let s = state_of(g);
        s.log.push(line);
        if s.log.len() > 24 { s.log.remove(0); }
        crossed_hook(g, x, who);
    }
}

/// the round stopped in static `bi`: if that is glass it gets the impact call with the energy it arrived with (glasspen.ts spentInPane)
pub fn spent_in_pane(g: &mut crate::game::Game, bi: f64, at: V3, dir: V3, energy: f64, who: f64, shell: Option<&mut crate::glasspen::ShellGlass>) {
    let b = level_box(g, bi);
    let sheet = crate::glass::is_sheet(g, b);
    if sheet { g.stats.glasspen.spent += 1; }
    let mark = match shell {
        Some(s) if sheet => shell_mark(g, s, bi, at, false),
        _ => ShellMark::First,
    };
    if mark == ShellMark::Join {
        g.stats.glasspen.shell_joined += 1;
        let e = energy * g.k.glasspen.fracture_k;
        crate::glassbreak::glass_break_impact(g, bi, at, Some(dir), e, who, Some(false));
        return;
    }
    pane_impact(g, bi, at, dir, energy, who, false, Some(mark == ShellMark::More));
}

/// the arm's line met nothing: is that because every sheet across it has fallen where the line crosses it, and there is one (glasspen.ts armInHole)
pub fn arm_in_hole(g: &mut crate::game::Game, from: V3, rd: V3, l: f64) -> bool {
    if crate::scene_collision::glass_open(g).is_none() { return false; }
    let mut any = false;
    let panes = crate::glass::glass_panes(g);
    for &bi in panes.iter() {
        let b = BoxId::from_ix(bi as usize);
        let s = match crate::scene_collision::line_box(g, from, rd, b) { Some(s) => s, None => continue };
        if s.t1 < 0.0 || s.t0 > l { continue; }
        if !pane_open(g, bi, v3::mad(from, rd, m::max(0.0, s.t0))) { return false; }
        any = true;
    }
    any
}

/// the muzzle already past the glass: what the panes across the arm left the round, filed at the arm's crossing; None: the old rule stands (glasspen.ts armThroughGlass)
pub fn arm_through_glass(g: &mut crate::game::Game, c_2: CharId, dir: V3, energy: f64, shell: Option<&mut crate::glasspen::ShellGlass>) -> Option<crate::glasspen::ArmThroughGlass> {
    if !glass_pen_live(g) { return None; }
    let from = match g.ch[c_2].bones.get(crate::character::Bone::UpperArmR) { Some(&p) => p, None => v3::add(g.ch[c_2].pos, [0.0, 1.4, 0.0]) };
    let dd = v3::sub(g.ch[c_2].muzzle, from);
    let l = v3::len(dd);
    if l < 1e-3 { return None; }
    let rd = v3::scale(dd, 1.0 / l);
    let mut cross: Vec<PaneCross> = Vec::new();
    let (mut o, mut t0, mut e) = (from, 0.0, energy);
    let mut k = 0;
    while k < 8 && t0 < l {
        k += 1;
        let h = match cast(g, o, rd, l - t0) { Some(h) => h, None => break };
        if h.index >= 1e6 { return None; }
        let at = v3::mad(o, rd, h.t);
        let toll = pane_toll(g, h.index, at, dir, e, Some(cross.len() as f64))?;
        let b = level_box(g, h.index)?;
        let thru = (match crate::scene_collision::line_box(g, at, rd, b) { Some(s) => s.t1, None => 0.0 }) + EXIT_EPS;
        cross.push(PaneCross {
            bi: h.index, t: t0 + h.t, at, exit: v3::mad(at, rd, thru), dir_in: dir, dir_out: dir, e_in: e, turn_deg: 0.0, thru, into: -1.0,
            type_: toll.type_, n: toll.n, cos: toll.cos, take: toll.take, e_out: toll.e_out, hole: toll.hole,
        });
        e = toll.e_out;
        t0 += h.t + thru;
        o = v3::mad(at, rd, thru);
    }
    if cross.is_empty() {
        return if arm_in_hole(g, from, rd, l) { Some(ArmThroughGlass { e: energy, panes: 0.0 }) } else { None };
    }
    let id = g.ch[c_2].id;
    commit_crossings(g, &cross, l, id, shell);
    g.stats.glasspen.via_arm += 1;
    Some(ArmThroughGlass { e, panes: cross.len() as f64 })
}

/// the line of fire across the panes a segment meets: does a round of `energy` get through every one (glasspen.ts shotPasses)
pub fn shot_passes(g: &mut crate::game::Game, a: V3, rd: V3, panes: &[crate::glasspen::ShotPassesPanesItem], energy: Option<f64>) -> bool {
    let mut e = energy.unwrap_or(g.k.glasspen.default_energy);
    g.stats.glasspen.shot_asked += 1;
    for k in 0..panes.len() {
        match pane_toll(g, panes[k].bi, v3::mad(a, rd, panes[k].t), rd, e, Some(k as f64)) {
            Some(toll) => e = toll.e_out,
            None => { g.stats.glasspen.shot_refused += 1; return false; }
        }
    }
    g.stats.glasspen.shot_through += 1;
    true
}

/// glass is not cover: how many of `eyes` have a line of fire to `p` through glass, the leaves taken out for the asking (glasspen.ts shotThroughGlass)
pub fn shot_through_glass(g: &mut crate::game::Game, eyes: &[V3], p: V3, energy: Option<f64>) -> f64 {
    let energy = energy.unwrap_or(g.k.glasspen.stack_energy);
    if !glass_pen_live(g) { return 0.0; }
    let panes = crate::glass::glass_panes(g);
    let leaves = core::mem::replace(&mut g.col.dynamic_boxes, Vec::new());
    let mut n = 0.0;
    for &e in eyes.iter() {
        let d = v3::sub(p, e);
        let l = v3::len(d);
        if l < 1e-3 { continue; }
        let rd = v3::scale(d, 1.0 / l);
        let mut glass = false;
        for &bi in panes.iter() {
            let b = BoxId::from_ix(bi as usize);
            if let Some(s) = crate::scene_collision::line_box(g, e, rd, b) {
                if s.t1 > 0.0 && s.t0 < l { glass = true; break; }
            }
        }
        if glass && !crate::scene_collision::shot_blocked(g, e, p, Some(0.02), Some(energy)) { n += 1.0; }
    }
    g.col.dynamic_boxes = leaves;
    n
}

/// is there standing glass on this line of fire (glasspen.ts glassOnLine)
pub fn glass_on_line(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    if !glass_pen_live(g) { return false; }
    let mut panes: Vec<crate::scene_collision::SegmentBlockedPanesItem> = Vec::new();
    crate::scene_collision::segment_blocked(g, a, b, Some(0.02), Some(0.0), Some(&mut panes));
    !panes.is_empty()
}

/// panel / console line (glasspen.ts glassPenSummary)
pub fn glass_pen_summary(g: &mut crate::game::Game) -> String {
    let word = if glass_pen_live(g) { "ON" } else { "on — inert while a showcase runs" };
    let st = &g.stats.glasspen;
    let last = match g.m.glasspen.states.as_ref().and_then(|s| s.log.last()) { Some(l) => format!(" · last: {}", l), None => String::new() };
    format!(
        "rounds through glass {} ▸ {} panes crossed ({} by a hole, {} muzzle-past) · {} rounds spent in a pane{}",
        word, st.crossed, st.via_hole, st.via_arm, st.spent, last,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(o: V3, t0: f64, len: f64) -> RoundSeg { RoundSeg { o, d: [1.0, 0.0, 0.0], t0, len, e: 1.0 } }

    #[test]
    fn a_path_is_walked_by_its_legs() {
        let mut p = RoundPath::zeroed();
        p.segs = vec![seg([0.0, 0.0, 0.0], 0.0, 2.0), seg([3.0, 0.0, 0.0], 3.0, 4.0)];
        // a T inside a pane's thickness belongs to the leg before it, and stops at that leg's end
        assert_eq!(seg_at(&p, 2.5).t0, 0.0);
        assert_eq!(path_at(&p, 2.5), [2.0, 0.0, 0.0]);
        assert_eq!(path_at(&p, 4.0), [4.0, 0.0, 0.0]);
        assert_eq!(path_at(&p, 99.0), [7.0, 0.0, 0.0]);
    }

    #[test]
    fn holes_are_kept_up_to_their_number() {
        let mut h = Vec::new();
        push_hole(&mut h, [0.0; 3], 2.0);
        push_hole(&mut h, [1.0, 0.0, 0.0], 2.0);
        push_hole(&mut h, [2.0, 0.0, 0.0], 2.0);
        assert_eq!(h.len(), 6);
        let r2 = 0.04 * 0.04;
        assert!(!near_in(&h, [0.0; 3], r2));
        assert!(near_in(&h, [2.0, 0.01, 0.0], r2));
    }

    #[test]
    fn the_shell_rule() {
        let r2 = 0.04 * 0.04;
        let mut mk = Vec::new();
        assert_eq!(mark_on(&mut mk, [0.0; 3], false, r2), ShellMark::First);
        // a pellet going through does not pass by the hole of one that stopped: its own mark, not the first
        assert_eq!(mark_on(&mut mk, [0.0; 3], true, r2), ShellMark::More);
        // one that goes through joins one that went through; one that stops joins either
        assert_eq!(mark_on(&mut mk, [0.01, 0.0, 0.0], true, r2), ShellMark::Join);
        assert_eq!(mark_on(&mut mk, [0.01, 0.0, 0.0], false, r2), ShellMark::Join);
        // far off on the pane: another mark
        assert_eq!(mark_on(&mut mk, [1.0, 0.0, 0.0], false, r2), ShellMark::More);
        assert_eq!(mk.len(), 25);
    }

    // the TypeScript's own bits: deflect's arithmetic under bun with EXACT.on (see notes.md, Decisions)
    #[test]
    fn the_turn_has_the_typescripts_bits() {
        for &(seed, want) in BITS_SEED.iter() { assert_eq!(turn_seed(seed), want, "seed {}", seed); }
        for row in BITS_TURN.iter() {
            let (seed, a, d, cos, n, bend, want) = (row.0, f64::from_bits(row.1), row.2.map(f64::from_bits), f64::from_bits(row.3), row.4.map(f64::from_bits), f64::from_bits(row.5), row.6);
            let t = PaneToll { type_: crate::glassbreak::GlassType::Annealed, n, cos, take: 0.0, e_out: 0.0, hole: false };
            let out = cone_out(turn_seed(seed), a, d, &t, bend);
            assert_eq!(out.map(f64::to_bits), want);
        }
    }

    const BITS_SEED: [(f64, u32); 7] = [
        (0.0, 695872825),
        (1.0, 1435855270),
        (123456789.0, 1199337779),
        (2147483648.0, 3857132857),
        (4294967295.0, 2963287353),
        (3735928559.0, 2643587293),
        (2882400001.0, 992403147),
    ];
    const BITS_TURN: [(f64, u64, [u64; 3], u64, [u64; 3], u64, [u64; 3]); 4] = [
        (123456789.0, 0x3f81df406cd799af, [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000], 0x3ff0000000000000, [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000], 0x3fe0000000000000, [0x3fefffdcb09d67de, 0x3f775e6a34c937f5, 0x3f515e8588a900e6]),
        (3735928559.0, 0x3f91df406cd799af, [0x3fe31ace8a44e87f, 0x3fb9791363068b55, 0x3fe9791363068b55], 0x3fe9791363068b55, [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000], 0x3fe0000000000000, [0x3fe2aa38eb811068, 0x3fb66daf56dd44be, 0x3fe9d74fc0c7417c]),
        (2882400001.0, 0x3fa1de69ad42c3ca, [0xbfd239175be95fe2, 0xbfee11668adaab02, 0x3fc84c1f25372a83], 0x3fd239175be95fe2, [0xbff0000000000000, 0x8000000000000000, 0x8000000000000000], 0x3fe0000000000000, [0xbfd4b53ac24a91ca, 0xbfed8860718c0aab, 0x3fcab7a8e1008d0b]),
        (4294967295.0, 0x3fb47ae147ae147b, [0x3fcbb9f25e1ee4d5, 0x3fabb9f25e1ee4d5, 0xbfef3130a9e2c16f], 0x3fee7a96f17d3c66, [0x3fe0001711c6615f, 0x0000000000000000, 0xbfebb66d96981b56], 0x3fd0000000000000, [0x3fce0d8a2a83d6d9, 0x3f9668081e0c6014, 0xbfef18fcd31564e7]),
    ];
}
