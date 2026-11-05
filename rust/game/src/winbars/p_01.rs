//! src/game/winbars.ts lines 1 to 196
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- seams: what skeleton 3 still cannot give
fn seam_furn_break_land(cls: crate::scene_materials::BreakClass) -> crate::furnblast::BreakRowLand { todo!("needs: FURN_BREAK[cls].land (the skeleton has `pub const FURN_BREAK: bool = false`, not the table)") }
#[allow(clippy::too_many_arguments)]
fn seam_particles_emit(g: &mut Game, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, speed: f64, speed_jit: f64, spread: f64, life: f64, life_jit: f64, size: f64, size_jit: f64, aspect: f64, color: V3, owner: f64) { todo!("needs: emit on the game's own Particles (gpu_fx returns a copy) and EmitOpts' speedJit, spread, lifeJit, sizeJit, aspect, color, owner") }

// ---------------------------------------------------------------- helpers
/// the bits of two flag words or'd   (`a | b`)
fn bor(a: f64, b: f64) -> f64 { (js::to_i32(a) | js::to_i32(b)) as f64 }
/// a bar, not hidden   (`(flags & (Bar | HIDE)) === Bar`)
fn stands(flags: f64) -> bool {
    let bar = js::to_i32(crate::scene_boxes::box_flag().bar);
    (js::to_i32(flags) & (bar | js::to_i32(HIDE))) == bar
}
/// `g.col.boxes[index]`: a number that is no row of the list is undefined
fn box_of(g: &Game, index: f64) -> Option<BoxId> {
    if !(index < 1e6) || index < 0.0 || m::floor(index) != index || index as usize >= g.col.boxes.len() { return None; }
    Some(BoxId::from_ix(index as usize))
}
/// `g.col.boxes[index]` where the TypeScript takes it to be there
fn box_at(index: f64) -> BoxId { BoxId::from_ix(index as usize) }
/// `Math.min(B.length, g.level.boxes.length)`: the statics the level was built with
fn built_boxes(g: &Game) -> usize { if g.col.boxes.len() < g.level.boxes.len() { g.col.boxes.len() } else { g.level.boxes.len() } }
/// the bar's word, the level it was struck at   (breakBar's `seed`)
fn seed_word(box_seed: f64, hit_u: f64) -> f64 { js::to_u32(box_seed + 131.0 + m::floor(hit_u * 16.0) * 31.0) as f64 }
/// what the k-th choice of a break hashes   (breakBar's `R`, before hash01)
fn hash_key(seed: f64, k: f64) -> f64 { m::imul(js::to_i32(seed + k), js::to_i32(2654435761.0)) as f64 }
/// the k-th choice of a break   (breakBar's `R`)
fn choice(seed: f64, k: f64) -> f64 { crate::debris::hash01(hash_key(seed, k)) }
/// `x.toFixed(1)`: the exact value rounded, an exact tie away from zero (Rust's own format takes the even digit)
fn to_fixed1(x: f64) -> String {
    let q = x * 4.0;
    let tie = m::floor(q) == q && js::rem(q, 2.0) != 0.0;
    if tie { format!("{:.1}", if x < 0.0 { x - 0.05 } else { x + 0.05 }) } else { format!("{:.1}", x) }
}

// ---------------------------------------------------------------- the part
/// The game's row, made if it is not there.   (winbars.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::winbars::BarState { g.m.winbars.state.get_or_insert_with(BarState::zeroed) }

/// The one gate: no showcase running.   (winbars.ts barsLive)
pub fn bars_live(g: &mut crate::game::Game) -> bool { !g.touring }

/// Is static `index` a window's bar that still stands?   (winbars.ts barAt)
pub fn bar_at(g: &mut crate::game::Game, index: f64) -> bool {
    let Some(b) = box_of(g, index) else { return false };
    stands(g.col.boxes[b].flags)
}

/// Every bar of this level that still stands, ascending.   (winbars.ts barsStanding)
pub fn bars_standing(g: &mut crate::game::Game) -> Vec<f64> {
    let n = built_boxes(g);
    let mut out = Vec::new();
    for i in 0..n {
        if stands(g.col.boxes[BoxId::from_ix(i)].flags) { out.push(i as f64); }
    }
    out
}

/// The bars a round or a blast has taken, ascending.   (winbars.ts barsGone)
pub fn bars_gone(g: &mut crate::game::Game) -> Vec<f64> {
    let mut v: Vec<f64> = state_of(g).gone.keys().iter().map(|k| k.0).collect();
    js::sort_by(&mut v, |p, q| *p - *q);
    v
}

/// The flags a bar this rule took was authored with, or None.   (winbars.ts barFlags0)
pub fn bar_flags0(g: &mut crate::game::Game, index: f64) -> Option<f64> {
    g.m.winbars.state.as_ref().and_then(|s| s.gone.get(crate::Num(index)).copied())
}

/// Once, as the game is made: the line of fire's question and the blasts' list.   (winbars.ts bootBars)
pub fn boot_bars(g: &mut crate::game::Game) {
    g.col.fire_pass = Some(crate::scene_collision::FirePass::Bars);
    crate::destruction::destruction_on_blast(g, &crate::destruction::BlastListener::Bars);
}

/// A round would stop in bar `index`: hidden from the rest of its flight, nothing more until bars_settle.   (winbars.ts barMeet)
pub fn bar_meet(g: &mut crate::game::Game, index: f64, t: f64, at: V3, dir: V3) -> crate::winbars::BarMet {
    let b = box_at(index);
    let flags = g.col.boxes[b].flags;
    let m = BarMet { index, t, at, dir, flags };
    g.col.boxes[b].flags = bor(flags, HIDE);
    crate::scene_collision::wrote(g);
    state_of(g).met.push(m);
    m
}

/// fireWeapon's way out: a bar still out of a round's way stands as it stood.   (winbars.ts barsUnmeet)
pub fn bars_unmeet(g: &mut crate::game::Game) {
    let met = match g.m.winbars.state.as_ref() {
        Some(s) if !s.met.is_empty() => s.met.clone(),
        _ => return,
    };
    for m in &met {
        g.col.boxes[box_at(m.index)].flags = m.flags;
    }
    crate::scene_collision::wrote(g);
    state_of(g).met.clear();
}

/// The round's end known: every bar it reached is broken, every other stands as it stood. Returns how many it broke.   (winbars.ts barsSettle)
pub fn bars_settle(g: &mut crate::game::Game, met: &[crate::winbars::BarMet], t_hit: f64, who: f64, lead: bool) -> f64 {
    let mut n = 0.0;
    state_of(g).met.clear();
    for m in met {
        g.col.boxes[box_at(m.index)].flags = m.flags;
    }
    crate::scene_collision::wrote(g);
    for m in met {
        if m.t > t_hit + 1e-6 {
            g.stats.winbars.spared += 1;
            continue;
        }
        if break_bar(g, m.index, m.at, m.dir, who, BreakBarCause::Round) {
            n += 1.0;
            if lead { crate::audio_iface::game_audio_like_play(g, "bulletImpact", Some(m.at), Some(0.8), None); }
        }
    }
    n
}

/// Bar `index` goes: hidden to every question, its pieces thrown the way `dir` from where it was struck. False = gone already or no standing bar.   (winbars.ts breakBar)
pub fn break_bar(g: &mut crate::game::Game, index: f64, at: V3, dir: V3, who: f64, cause: crate::winbars::BreakBarCause) -> bool {
    state_of(g);
    let Some(b) = box_of(g, index) else { return false };
    if state_of(g).gone.has(crate::Num(index)) || !bar_at(g, index) { return false; }
    let flags = g.col.boxes[b].flags;
    state_of(g).gone.set(crate::Num(index), flags);
    g.col.boxes[b].flags = bor(flags, HIDE);
    changed(g, b);
    g.stats.winbars.broken += 1;
    if cause == BreakBarCause::Round { g.stats.winbars.by_round += 1; } else { g.stats.winbars.by_blast += 1; }

    // the pieces: the bar cut across into uneven lengths, the struck one thrown hardest
    let k_ = g.k.winbars;
    let (bc, bh, yaw) = (g.col.boxes[b].c, g.col.boxes[b].h, g.col.boxes[b].yaw);
    let n = m::max(1.0, m::round(k_.pieces));
    let y0 = bc[1] - bh[1];
    let h = 2.0 * bh[1];
    let does = g.col.boxes[b].does.map(|w| String::from(w.as_str()));
    let cls = crate::furnblast::break_class_of(g, &crate::furnblast::BreakClassOfB { does });
    let land = seam_furn_break_land(cls);
    let seed = seed_word(crate::glass::box_seed(g, b), v3::clamp((at[1] - y0) / h, 0.0, 1.0));
    let mut cuts = vec![0.0];
    let mut j = 1.0;
    while j < n {
        cuts.push((j + (choice(seed, 10.0 + j) - 0.5) * k_.uneven) / n);
        j += 1.0;
    }
    cuts.push(1.0);
    let (mut dx, mut dz) = (dir[0], dir[2]);
    let l = m::hyp2(dx, dz);
    if l < 0.2 {
        let (c, s) = (m::cos(yaw), m::sin(yaw));
        if bh[0] > bh[2] { dx = c; dz = -s; } else { dx = s; dz = c; }
    } else {
        dx /= l;
        dz /= l;
    }
    let p = crate::particle_fx::gpu_fx(g).is_some();
    let hit_u = v3::clamp((at[1] - y0) / h, 0.0, 1.0);
    let tab = crate::scene_boxes::box_flag();
    let mut k = 100.0;
    for j in 0..(n as usize) {
        let (u0, u1) = (cuts[j], cuts[j + 1]);
        let struck = hit_u >= u0 && hit_u <= u1;
        let r1 = choice(seed, k);
        k += 1.0;
        let r2 = choice(seed, k);
        k += 1.0;
        let r3 = choice(seed, k);
        k += 1.0;
        let f = crate::scene_boxes::copy_box(g, b);
        let fc: V3 = [bc[0], y0 + (u0 + u1) / 2.0 * h, bc[2]];
        let fh: V3 = [bh[0], (u1 - u0) / 2.0 * h, bh[2]];
        {
            let piece = &mut g.col.boxes[f];
            piece.c = fc;
            piece.h = fh;
            piece.flags = tab.dynamic;
            piece.marks = None;
        }
        let sp = v3::lerp(k_.fly[0], k_.fly[1], r1) * (if struck { 1.0 } else { k_.rest_k });
        let sd = (r2 - 0.5) * 2.0 * k_.side;
        let v: V3 = [dx * sp - dz * sd, v3::lerp(k_.up[0], k_.up[1], r3), dz * sp + dx * sd];
        let w: V3 = [(r1 - 0.5) * k_.spin, (r2 - 0.5) * 0.5 * k_.spin, (r3 - 0.5) * k_.spin];
        let got = crate::debris::debris_spawn(g, crate::debris::ChunkKind::Chunk, f, v, w, Some(who), Some(land.level));
        if let Some(c) = got {
            g.stats.winbars.chunks += 1;
            let chunk = &mut g.tab.debris_chunk[c];
            chunk.level = land.level;
            chunk.sfx = land.sfx.iter().map(|s| crate::debris::ChunkSfxItem { name: s.name.clone(), rate: s.rate, gain: s.gain }).collect();
            match cls {
                crate::scene_materials::BreakClass::Wood => crate::break_table::breaks_retag(g, c, crate::break_table::Mat::Wood),
                crate::scene_materials::BreakClass::Metal => crate::break_table::breaks_retag(g, c, crate::break_table::Mat::Metal),
                _ => {}
            }
            crate::break_table::breaks_dress(g, c);
        } else {
            g.stats.winbars.demoted += 1;
            if p {
                let color = v3::copy(g.col.boxes[b].albedo);
                seam_particles_emit(g, crate::render_particles::PKind::Debris, fc, v3::normalize([v[0], m::max(0.3, v[1]), v[2]]), 10.0, 2.2, 0.7, 1.1, 18.0, 0.3, 0.006, 0.6, 3.0, color, who);
            }
        }
    }
    if p {
        let color = v3::copy(g.col.boxes[b].albedo);
        seam_particles_emit(g, crate::render_particles::PKind::Debris, [bc[0], v3::clamp(at[1], y0, y0 + h), bc[2]], v3::normalize([dx, 0.25, dz]), k_.chips, k_.chip_v, 0.7, 1.0, 18.0, 0.3, 0.007, 0.6, 4.0, color, who);
    }
    let line = format!("{}s bar {} broken by a {} (who {})", to_fixed1(g.time), index, cause.as_str(), who);
    let log = &mut g.m.winbars.log;
    log.push(line);
    if log.len() > 24 { log.remove(0); }
    true
}

/// The level changed where bar `b` stands: whoever keeps an answer about the level's boxes asks again.   (winbars.ts changed)
pub fn changed(g: &mut crate::game::Game, b: BoxId) {
    g.engine.world.geom_serial += 1.0;
    crate::scene_collision::wrote(g);
    crate::scene_collision::sound_paths_invalidate(g);
    let (c, h) = (g.col.boxes[b].c, g.col.boxes[b].h);
    crate::smoke_types::smoke_system_like_obstacles_changed(g, c, m::hyp3(h[0], h[1], h[2]) + 0.6);
}

/// The blasts' list: every standing bar with a point of its length inside the radius and nothing solid between goes, thrown out from the blast.   (winbars.ts barsBlast)
pub fn bars_blast(g: &mut crate::game::Game, ev: crate::wallbreak::BlastEvent) {
    if !bars_live(g) || g.k.winbars.blast_k <= 0.0 { return; }
    let r = ev.radius * g.k.winbars.blast_k;
    let from: V3 = [ev.at[0], m::max(0.3, ev.at[1]), ev.at[2]];
    let mut near: Vec<f64> = Vec::new();
    for i in bars_standing(g) {
        let (c, h) = (g.col.boxes[box_at(i)].c, g.col.boxes[box_at(i)].h);
        if m::hyp2(c[0] - from[0], c[2] - from[2]) <= r + m::max(h[0], h[2]) { near.push(i); }
    }
    if near.is_empty() { return; }
    let mut took: Vec<(f64, V3)> = Vec::new();
    let was: Vec<f64> = near.iter().map(|&i| g.col.boxes[box_at(i)].flags).collect();
    for &i in &near {
        let b = box_at(i);
        g.col.boxes[b].flags = bor(g.col.boxes[b].flags, HIDE);
    }
    crate::scene_collision::wrote(g);
    for &i in &near {
        let (c, h) = (g.col.boxes[box_at(i)].c, g.col.boxes[box_at(i)].h);
        for u in [0.0, 0.6, -0.6] {
            let q: V3 = [c[0], c[1] + u * h[1], c[2]];
            if v3::dist(from, q) > r + m::max(h[0], h[2]) { continue; }
            if crate::scene_collision::solid_blocked(g, from, q, None) { continue; }
            took.push((i, q));
            break;
        }
    }
    for (k, &i) in near.iter().enumerate() {
        g.col.boxes[box_at(i)].flags = was[k];
    }
    crate::scene_collision::wrote(g);
    for (i, at) in took {
        let c = g.col.boxes[box_at(i)].c;
        let (dx, dz) = (c[0] - from[0], c[2] - from[2]);
        let l = m::hyp2(dx, dz);
        let dir: V3 = if l > 1e-3 { [dx / l, 0.0, dz / l] } else { [0.0, 0.0, 0.0] };
        break_bar(g, i, at, dir, js::or(ev.who, crate::consts::NOBODY_ID), BreakBarCause::Blast);
    }
}

/// As a level resets: every bar stands again.   (winbars.ts resetBars)
pub fn reset_bars(g: &mut crate::game::Game) {
    let Some(s) = g.m.winbars.state.as_mut() else { return };
    s.met.clear();
    if s.gone.is_empty() { return; }
    let gone: Vec<(crate::Num, f64)> = s.gone.iter().map(|(k, v)| (k, *v)).collect();
    for (i, f) in gone {
        if let Some(b) = box_of(g, i.0) {
            g.col.boxes[b].flags = f;
            changed(g, b);
        }
    }
    state_of(g).gone.clear();
}

/// Can this file's slice be applied to this level? None = yes.   (winbars.ts barsCheck)
pub fn bars_check(g: &mut crate::game::Game, s: Option<&crate::winbars::BarsSave>) -> Option<String> {
    let s = s?;
    if s.v != 1.0 { return Some(String::from("window bars: unknown slice")); }
    let n = built_boxes(g) as f64;
    let bar = js::to_i32(crate::scene_boxes::box_flag().bar);
    for &i in &s.s {
        let integer = i.is_finite() && m::floor(i) == i;
        let bad = !integer || i < 0.0 || i >= n || (js::to_i32(g.col.boxes[box_at(i)].flags) & bar) == 0;
        if bad { return Some(format!("window bars: static {} is not a window's bar of this level", i)); }
    }
    None
}

/// Panel and console line.   (winbars.ts winBarsSummary)
pub fn win_bars_summary(g: &mut crate::game::Game) -> String {
    let up = bars_standing(g).len();
    let gone = state_of(g).gone.len();
    let state = if bars_live(g) { "ON" } else { "on — inert while a showcase runs" };
    let st = g.stats.winbars.clone();
    let last = match g.m.winbars.log.last() {
        Some(l) => format!(" · last: {}", l),
        None => String::new(),
    };
    format!("window bars {} ▸ {} standing, {} gone · {} by a round, {} by a blast · {} pieces (+{} as chips){}", state, up, gone, st.by_round, st.by_blast, st.chunks, st.demoted, last)
}

/// QA: the last breaks filed, oldest first.   (winbars.ts winBarsLog)
pub fn win_bars_log(g: &mut crate::game::Game) -> Vec<String> { g.m.winbars.log.clone() }

#[cfg(test)]
mod tests {
    use super::*;

    // the expected values are the TypeScript's own, printed under bun from the expressions of winbars.ts:117 and Number.prototype.toFixed
    #[test]
    fn seed_word_is_the_typescripts() {
        for (s, h, want) in [(0.0, 0.0, 131.0), (4294967295.0, 1.0, 626.0), (123456789.0, 0.37, 123457075.0), (2147483648.0, 0.999, 2147484244.0), (7.0, f64::NAN, 0.0)] {
            assert_eq!(seed_word(s, h).to_bits(), f64::to_bits(want));
        }
    }
    #[test]
    fn hash_key_is_the_typescripts() {
        for (s, k, want) in [(0.0, 100.0, -844396252.0), (4294967295.0, 102.0, 1810039509.0), (3735928559.0, 11.0, 1763390682.0), (131.0, 10.0, 613287549.0)] {
            assert_eq!(hash_key(s, k).to_bits(), f64::to_bits(want));
        }
    }
    #[test]
    fn to_fixed1_is_the_typescripts() {
        for (x, want) in [(0.0, "0.0"), (0.25, "0.3"), (0.75, "0.8"), (1.25, "1.3"), (2.5, "2.5"), (0.05, "0.1"), (0.15, "0.1"), (12.34, "12.3"), (99.95, "100.0"), (1234.5678, "1234.6"), (16.666666666666668, "16.7")] {
            assert_eq!(to_fixed1(x), want);
        }
    }
    #[test]
    fn a_bar_stands_until_either_hide_flag_is_up() {
        let t = crate::scene_boxes::box_flag();
        assert_eq!(HIDE, bor(t.no_primary, t.no_shadow));
        assert!(stands(t.bar));
        assert!(stands(bor(t.bar, t.dynamic)));
        assert!(!stands(bor(t.bar, t.no_primary)));
        assert!(!stands(bor(t.bar, t.no_shadow)));
        assert!(!stands(t.dynamic));
        assert_eq!(bor(t.bar, HIDE), t.bar + 3.0);
    }
}
