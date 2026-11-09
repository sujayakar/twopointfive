pub use ::game_types::winbars::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- seams: what this skeleton still cannot give
/// a box's row. This skeleton has `g.col.boxes: Vec<BoxId>` and no table of Box rows; where the table comes, this is the one line `&mut g.tab.boxes[b]`
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

// ---------------------------------------------------------------- helpers
/// the bits of two flag words or'd   (`a | b`)
fn bor(a: f64, b: f64) -> f64 { (js::to_i32(a) | js::to_i32(b)) as f64 }
/// a bar, not hidden   (`(flags & (Bar | HIDE)) === Bar`)
fn stands(flags: f64) -> bool {
    let bar = js::to_i32(crate::scene_boxes::box_flag().bar);
    (js::to_i32(flags) & (bar | js::to_i32(HIDE))) == bar
}
/// `g.col.boxes[index]`: a number that is no place in the list is undefined
fn box_of(g: &Game, index: f64) -> Option<BoxId> {
    if !(index < 1e6) || index < 0.0 || m::floor(index) != index { return None; }
    g.col.boxes.get(index as usize).copied()
}
/// `g.col.boxes[index]` where the TypeScript takes it to be there
fn box_at(g: &Game, index: f64) -> BoxId { g.col.boxes[index as usize] }
/// `Math.min(B.length, g.level.boxes.length)`: that list and `g.col.boxes` are one array, so it is the list's length
fn built_boxes(g: &Game) -> usize { g.col.boxes.len() }
/// the bar's word, the level it was struck at   (breakBar's `seed`)
fn seed_word(box_seed: f64, hit_u: f64) -> f64 { js::to_u32(box_seed + 131.0 + m::floor(hit_u * 16.0) * 31.0) as f64 }
/// what the k-th choice of a break hashes   (breakBar's `R`, before hash01)
fn hash_key(seed: f64, k: f64) -> f64 { m::imul(js::to_i32(seed + k), js::to_i32(2654435761.0)) as f64 }
/// the k-th choice of a break   (breakBar's `R`)
fn choice(seed: f64, k: f64) -> f64 { crate::debris::hash01(hash_key(seed, k)) }
/// nothing of a pane's stands in this game's block   (glassbreak.ts panesUntouched; its stub takes no slot, see notes)
fn untouched(g: &Game) -> bool { ::sim::state::state_read(g.sim).panes.used == 0 }
/// the module's answers: `x.sim_scratch()` over `f64()`, its `o` being 0 here. Read before any other call into the module
fn scratch() -> &'static [f64] {
    unsafe { core::slice::from_raw_parts(::sim::sim_scratch() as *const f64, ::sim::sim_scratch_len()) }
}

// ---------------------------------------------------------------- the part
/// The game's row, made if it is not there.
fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::winbars::BarState { g.m.winbars.state.get_or_insert_with(|| BarState { met: Vec::new() }) }

/// Is static `index` a window's bar that still stands?
pub fn bar_at(g: &mut crate::game::Game, index: f64) -> bool {
    let Some(b) = box_of(g, index) else { return false };
    stands(seam_box(g, b).flags)
}

/// The flags a bar this rule took was authored with, or None.
pub(super) fn bar_flags0(g: &mut crate::game::Game, index: f64) -> Option<f64> {
    if untouched(g) { return None; }
    let s = g.sim;
    for i in 0..::sim::panes::bars_gone(s) as usize {
        if ::sim::panes::bar_gone(s, i) as f64 == index { return Some(::sim::panes::bar_flags(s, i) as f64); }
    }
    None
}

/// Once, as the game is made: the line of fire's question and the blasts' list.
pub fn boot_bars(g: &mut crate::game::Game) {
    g.col.fire_pass = Some(crate::scene_collision::FirePass::Bars);
    crate::destruction::destruction_on_blast(g, crate::destruction::BlastListener::Bars);
}

/// A round would stop in bar `index`: hidden from the rest of its flight, nothing more until bars_settle.
pub fn bar_meet(g: &mut crate::game::Game, index: f64, t: f64, at: V3, dir: V3) -> crate::winbars::BarMet {
    let b = box_at(g, index);
    let flags = seam_box(g, b).flags;
    let m = BarMet { index, t, at, dir, flags };
    seam_box(g, b).flags = bor(flags, HIDE);
    crate::scene_collision::wrote(g);
    state_of(g).met.push(m);
    m
}

/// fireWeapon's way out: a bar still out of a round's way stands as it stood.
pub fn bars_unmeet(g: &mut crate::game::Game) {
    let met = match g.m.winbars.state.as_ref() {
        Some(s) if !s.met.is_empty() => s.met.clone(),
        _ => return,
    };
    for m in &met {
        let b = box_at(g, m.index);
        seam_box(g, b).flags = m.flags;
    }
    crate::scene_collision::wrote(g);
    state_of(g).met.clear();
}

/// The round's end known: every bar it reached is broken, every other stands as it stood. Returns how many it broke.
pub fn bars_settle(g: &mut crate::game::Game, met: &[crate::winbars::BarMet], t_hit: f64, who: f64, lead: bool) -> f64 {
    let mut n = 0.0;
    state_of(g).met.clear();
    for m in met {
        let b = box_at(g, m.index);
        seam_box(g, b).flags = m.flags;
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

/// Bar `index` goes: hidden to every question, its pieces thrown the way `dir` from where it was struck. False = gone already or no standing bar.
fn break_bar(g: &mut crate::game::Game, index: f64, at: V3, dir: V3, who: f64, cause: crate::winbars::BreakBarCause) -> bool {
    let Some(b) = box_of(g, index) else { return false };
    let taken = crate::glassbreak::glass_break_asking(g, &mut |g: &mut Game, _s: f64, _w: f64| {
        let flags = seam_box(g, b).flags;
        ::sim::bars_take(js::to_u32(index), js::to_u32(flags)) == 1
    });
    if !taken { return false; }
    let flags = seam_box(g, b).flags;
    seam_box(g, b).flags = bor(flags, HIDE);
    changed(g);
    g.stats.winbars.broken += 1;
    if cause == BreakBarCause::Round { g.stats.winbars.by_round += 1; } else { g.stats.winbars.by_blast += 1; }

    // the pieces: the bar cut across into uneven lengths, the struck one thrown hardest
    let k_ = g.k.winbars;
    let (bc, bh, yaw, albedo, does) = { let r = seam_box(g, b); (r.c, r.h, r.yaw, r.albedo, r.does.map(|w| String::from(w.as_str()))) };
    let n = m::max(1.0, m::round(k_.pieces));
    let y0 = bc[1] - bh[1];
    let h = 2.0 * bh[1];
    let cls = crate::furnblast::break_class_of(g, &crate::furnblast::BreakClassOfB { does });
    let land = crate::furnblast::furn_break_row(cls).land;
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
            let piece = seam_box(g, f);
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
            {
                let o = crate::render_particles::EmitOpts { speed: 2.2, speed_jit: Some(0.7), spread: Some(1.1), life: 18.0, life_jit: Some(0.3), size: 0.006, size_jit: Some(0.6), aspect: Some(3.0), color: Some(v3::copy(albedo)), owner: Some(who), ..Default::default() };
                crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, fc, v3::normalize([v[0], m::max(0.3, v[1]), v[2]]), 10.0, o);
            }
        }
    }
    let o = crate::render_particles::EmitOpts { speed: k_.chip_v, speed_jit: Some(0.7), spread: Some(1.0), life: 18.0, life_jit: Some(0.3), size: 0.007, size_jit: Some(0.6), aspect: Some(4.0), color: Some(v3::copy(albedo)), owner: Some(who), ..Default::default() };
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, [bc[0], v3::clamp(at[1], y0, y0 + h), bc[2]], v3::normalize([dx, 0.25, dz]), k_.chips, o);
    let line = format!("{}s bar {} broken by a {} (who {})", js::to_fixed(g.time, 1), js::num_str(index), cause.as_str(), js::num_str(who));
    let log = &mut g.m.winbars.log;
    log.push(line);
    if log.len() > 24 { log.remove(0); }
    true
}

/// The level changed where a bar stood: whoever keeps an answer about the level's boxes asks again.
fn changed(g: &mut crate::game::Game) {
    g.engine.world.geom_serial += 1.0;
    crate::scene_collision::wrote(g);
    crate::scene_collision::sound_paths_invalidate(g);
}

/// The blasts' list: every standing bar with a point of its length inside the radius and nothing solid between goes, thrown out from the blast.
pub(super) fn bars_blast(g: &mut crate::game::Game, ev: crate::wallbreak::BlastEvent) {
    if g.k.winbars.blast_k <= 0.0 { return; }
    let r = ev.radius * g.k.winbars.blast_k;
    let from: V3 = [ev.at[0], m::max(0.3, ev.at[1]), ev.at[2]];
    let took: Vec<(f64, V3)> = crate::glassbreak::glass_break_asking(g, &mut |g: &mut Game, _s: f64, _w: f64| {
        let n = ::sim::bars_blast(from[0], from[1], from[2], r, built_boxes(g), &g.k);
        let m = scratch();
        let mut out = Vec::new();
        for k in 0..n { out.push((m[4 * k], [m[4 * k + 1], m[4 * k + 2], m[4 * k + 3]])); }
        out
    });
    for (i, at) in took {
        let b = box_at(g, i);
        let c = seam_box(g, b).c;
        let (dx, dz) = (c[0] - from[0], c[2] - from[2]);
        let l = m::hyp2(dx, dz);
        let dir: V3 = if l > 1e-3 { [dx / l, 0.0, dz / l] } else { [0.0, 0.0, 0.0] };
        break_bar(g, i, at, dir, js::or(ev.who, crate::consts::NOBODY_ID), BreakBarCause::Blast);
    }
}

/// As a level resets: every bar stands again.
pub fn reset_bars(g: &mut crate::game::Game) {
    if let Some(s) = g.m.winbars.state.as_mut() { s.met.clear(); }
    if untouched(g) { return; }
    let s = g.sim;
    for k in 0..::sim::panes::bars_gone(s) as usize {
        if let Some(b) = box_of(g, ::sim::panes::bar_gone(s, k) as f64) {
            seam_box(g, b).flags = ::sim::panes::bar_flags(s, k) as f64;
            changed(g);
        }
    }
    ::sim::panes::bars_clear(s);
}

#[cfg(test)]
mod tests {
    use super::*;

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
