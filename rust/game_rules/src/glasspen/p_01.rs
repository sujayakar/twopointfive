//! src/game/glasspen.ts lines 1 to 348
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- local helpers

/// `g.level.boxes[bi]`: an index with no box is undefined there, None here. That list and `g.col.boxes` are one array in the TypeScript, and an index is a place in it, not a BoxId
fn level_box(g: &Game, bi: f64) -> Option<BoxId> {
    let ix = bi as usize;
    if ix as f64 != bi { return None; }
    g.col.boxes.get(ix).copied()
}
/// `GLASSPEN.energy[id] ?? GLASSPEN.byClass[cls] ?? GLASSPEN.defaultEnergy`
fn energy_by(k: &crate::knobs::GlasspenK, id: crate::weapons::WeaponId, cls: Option<crate::weapons::WeaponClass>) -> f64 {
    if let Some(&e) = k.energy.get(String::from(id.as_str())) { return e; }
    if let Some(c) = cls {
        if let Some(&e) = k.by_class.get(String::from(c.as_str())) { return e; }
    }
    k.default_energy
}
/// `g.col.raycast(ro, rd, tmax)` in this file's shape of a hit
fn cast(g: &mut Game, ro: V3, rd: V3, tmax: f64) -> Option<RoundPathSt> {
    crate::scene_collision::raycast(g, ro, rd, tmax).map(|h| RoundPathSt { t: h.t, index: h.index, n: h.n })
}
/// a new state: no log, rev 0 (glasspen.ts stateOf)
fn fresh_state() -> PenState { PenState { log: Vec::new(), rev: 0.0 } }
/// `panesUntouched(g.sim)`: the part's first word. The skeleton's `panes_untouched(b: Opaque)` cannot carry the slot, and the slot is `g.sim`
fn untouched(g: &Game) -> bool { ::sim::state::state_read(g.sim).panes.used == 0 }
/// `PANE_TYPES[i]`, the module's number for a glass (rust/sim/src/panes.rs ANNEALED, TEMPERED, LAMINATED)
fn pane_type_word(i: f64) -> PaneType {
    if i == 1.0 { PaneType::Tempered } else if i == 2.0 { PaneType::Laminated } else { PaneType::Annealed }
}
/// the module's scratch as the TypeScript reads it through `f64()`, from its first double (`a = x.sim_scratch() / 8` is 0 here). The last call's answer: read it before the next call
fn scratch() -> &'static [f64] {
    unsafe { core::slice::from_raw_parts(::sim::sim_scratch() as *const f64, ::sim::sim_scratch_len()) }
}
/// what `pen_path` came to (`h = x.pen_path_head() / 8` is 0 here): legs, crossings, length, the stop, each crossing's `into`
fn path_head() -> &'static [f64] {
    unsafe { core::slice::from_raw_parts(::sim::pen_path_head() as *const f64, 8 + MAX_CROSS as usize) }
}
/// the shell rule on one pane's marks, five numbers each: x, y, z, went-through, filed-a-mark (glasspen.ts shellMark)
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
/// `GLASSPEN_HOOKS.crossed?.(g, x, who)`: the game hangs one value (game.ts:1226), the exit jet of shards.ts
fn crossed_hook(g: &mut Game, x: &PaneCross, who: f64) {
    let like = crate::shards::PaneCrossLike { bi: x.bi, type_: x.type_, exit: x.exit, dir_out: x.dir_out, take: x.take, e_out: x.e_out, hole: x.hole };
    crate::shards::glass_shards_crossed(g, like, who);
}

// ---------------------------------------------------------------- host work: nothing here on the page's path

/// the switch kept per machine: browser storage, and the rows it wrote are settled (glasspen.ts loadGlassPen)
pub fn load_glass_pen(g: &crate::game::Game) {}

// ---------------------------------------------------------------- the switch, a round's energy

/// the one gate: no showcase running (glasspen.ts glassPenLive)
pub fn glass_pen_live(g: &crate::game::Game) -> bool { !g.touring }

/// a round's energy as it leaves this row's muzzle (glasspen.ts roundEnergy)
pub fn round_energy(g: &crate::game::Game, def: crate::weapons::WeaponId) -> f64 {
    let cls = g.m.weapons.weapons.get(def).map(|d| d.cls);
    energy_by(&g.k.glasspen, def, cls)
}

/// …and of the gun in this man's hands (glasspen.ts roundEnergyOf)
pub fn round_energy_of(g: &mut crate::game::Game, c_2: CharId) -> f64 {
    if let Some(pl) = crate::seats::man_of(g, Some(c_2)) {
        let gun = match crate::game::player_gun_in_hand(g, pl) { Some(f) => f, None => g.pl[pl].pistol };
        let def = g.fa[gun].def;
        return round_energy(g, def);
    }
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if g.gd[gd].ch == c_2 {
            let def = g.fa[g.gd[gd].weapon].def;
            return round_energy(g, def);
        }
    }
    g.k.glasspen.default_energy
}

/// the energy a line-of-fire question about THIS man's gun is asked with (glasspen.ts shotEnergy)
pub fn shot_energy(g: &mut crate::game::Game, c_2: CharId, def: Option<crate::weapons::WeaponId>) -> Option<f64> {
    if !glass_pen_live(g) { return None; }
    Some(match def { Some(d) => round_energy(g, d), None => round_energy_of(g, c_2) })
}

// ---------------------------------------------------------------- holes: the module's; here only the count of changes and the log

/// this game's log and count of changes, made if none yet: the row itself (glasspen.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::glasspen::PenState {
    g.m.glasspen.states.get_or_insert_with(fresh_state)
}

/// the holes go with the stars (glasspen.ts resetGlassPen)
pub fn reset_glass_pen(g: &mut crate::game::Game) {
    if let Some(s) = g.m.glasspen.states.as_mut() {
        s.log.clear();
        s.rev += 1.0;
    }
    if !untouched(g) { ::sim::panes::clear_holes(g.sim); }
}

/// a number that moves whenever what a round has to pay at any pane may have moved (glasspen.ts glassTollSig)
pub fn glass_toll_sig(g: &mut crate::game::Game) -> f64 {
    let rev = match g.m.glasspen.states.as_ref() { Some(s) => s.rev, None => 0.0 };
    rev * 65537.0 + crate::glassbreak::glass_break_open_sig(g)
}

/// remember a crossing on pane `bi`: the module keeps it, the oldest going past `holes` (glasspen.ts addHole)
pub fn add_hole(g: &mut crate::game::Game, bi: f64, p: V3) {
    state_of(g).rev += 1.0;
    crate::glassbreak::glass_break_asking(g, &mut |_g: &mut Game, s: f64, _w: f64| ::sim::pen::add_hole(s as usize, js::to_u32(bi), p));
}

// ---------------------------------------------------------------- the toll

/// a crossing as the module lists it (rust/sim/src/pen.rs write_cross; glasspen.ts crossAt)
pub fn cross_at(m: &[f64], o: f64, into: f64) -> crate::glasspen::PaneCross {
    let o = o as usize;
    PaneCross {
        type_: pane_type_word(m[o]), n: [m[o + 1], m[o + 2], m[o + 3]], cos: m[o + 4], take: m[o + 5], e_out: m[o + 6], hole: m[o + 7] != 0.0,
        bi: m[o + 8], t: m[o + 9],
        at: [m[o + 10], m[o + 11], m[o + 12]], exit: [m[o + 13], m[o + 14], m[o + 15]], dir_in: [m[o + 16], m[o + 17], m[o + 18]], dir_out: [m[o + 19], m[o + 20], m[o + 21]],
        e_in: m[o + 22], turn_deg: m[o + 23], thru: m[o + 24], into,
    }
}

/// one pane on a round's flight: what it takes and where the round leaves it, or None: spent in it (glasspen.ts paneCrossing)
pub fn pane_crossing(g: &mut crate::game::Game, bi: f64, o: V3, d: V3, t: f64, e: f64, nth: f64) -> Option<crate::glasspen::PaneCross> {
    crate::glassbreak::glass_break_asking(g, &mut |_g: &mut Game, _s: f64, _w: f64| -> Option<PaneCross> {
        if ::sim::pen_cross(js::to_u32(bi), o[0], o[1], o[2], d[0], d[1], d[2], t, e, nth as usize) == 0 { return None; }
        let m = scratch();
        Some(cross_at(m, 0.0, m[CROSS as usize]))
    })
}

// ---------------------------------------------------------------- the round's flight

/// the flight of a round over statics and leaves, or None: it crosses no pane. `first` None = cast here, Some(None) = the caller cast and hit nothing (glasspen.ts roundPath)
pub fn round_path(g: &mut crate::game::Game, from: V3, dir: V3, range: f64, energy: f64, first: Option<Option<crate::glasspen::RoundPathSt>>, nth0: Option<f64>) -> Option<crate::glasspen::RoundPath> {
    let nth0 = nth0.unwrap_or(0.0);
    let hit = match first { Some(h) => h, None => cast(g, from, dir, range) };
    let hit = hit?;
    if hit.index >= 1e6 { return None; }
    let b = level_box(g, hit.index);
    if !crate::glass::is_sheet(g, b) { return None; }
    crate::glassbreak::glass_break_asking(g, &mut |_g: &mut Game, _s: f64, _w: f64| -> Option<RoundPath> {
        if ::sim::pen_path(from[0], from[1], from[2], dir[0], dir[1], dir[2], range, energy, nth0 as usize, hit.t, js::to_i32(hit.index), hit.n[0], hit.n[1], hit.n[2]) == 0 { return None; }
        let (m, h) = (scratch(), path_head());
        let (mut segs, mut cross): (Vec<RoundSeg>, Vec<PaneCross>) = (Vec::new(), Vec::new());
        let leg = LEG as usize;
        for i in 0..h[0] as usize {
            let o = i * leg;
            segs.push(RoundSeg { o: [m[o], m[o + 1], m[o + 2]], d: [m[o + 3], m[o + 4], m[o + 5]], t0: m[o + 6], len: m[o + 7], e: m[o + 8] });
        }
        let c0 = (MAX_CROSS + 1.0) * LEG;
        for i in 0..h[1] as usize {
            cross.push(cross_at(m, c0 + i as f64 * CROSS, h[8 + i]));
        }
        let st = if h[3] < 0.0 { None } else { Some(RoundPathSt { t: h[4], index: h[3], n: [h[5], h[6], h[7]] }) };
        Some(RoundPath { segs, cross, st, len: h[2] })
    })
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
pub fn shell_mark(g: &crate::game::Game, shell: &mut crate::glasspen::ShellGlass, bi: f64, at: V3, through: bool) -> crate::glasspen::ShellMark {
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
            "{}s pane {} ({}) crossed{}: energy {} → {}, turned {}°{}",
            js::to_fixed(g.time, 1), x.bi, x.type_.as_str(), if x.hole { " by its hole" } else { "" }, js::to_fixed(x.e_in, 2), js::to_fixed(x.e_out, 2), js::to_fixed(x.turn_deg, 2),
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

/// the muzzle already past the glass: what the panes across the arm left the round, filed at the arm's crossing; None: the old rule stands (glasspen.ts armThroughGlass)
pub fn arm_through_glass(g: &mut crate::game::Game, c_2: CharId, dir: V3, energy: f64, shell: Option<&mut crate::glasspen::ShellGlass>) -> Option<crate::glasspen::ArmThroughGlass> {
    if !glass_pen_live(g) { return None; }
    let from = match g.ch[c_2].bones.get(crate::character::Bone::UpperArmR) { Some(&p) => p, None => v3::add(g.ch[c_2].pos, [0.0, 1.4, 0.0]) };
    let mz = g.ch[c_2].muzzle;
    let l = v3::len(v3::sub(mz, from));
    let w = crate::glassbreak::glass_break_asking(g, &mut |_g: &mut Game, _s: f64, _w: f64| -> Option<(f64, Vec<PaneCross>)> {
        let n = ::sim::pen_arm(from[0], from[1], from[2], mz[0], mz[1], mz[2], dir[0], dir[1], dir[2], energy);
        if n < 0 { return None; }
        let m = scratch();
        let mut cross: Vec<PaneCross> = Vec::new();
        for i in 0..n { cross.push(cross_at(m, i as f64 * CROSS, -1.0)); }
        Some((m[::sim::sim_scratch_len() - 1], cross))
    });
    let (e, cross) = w?;
    if cross.is_empty() { return Some(ArmThroughGlass { e: energy, panes: 0.0 }); }
    let id = g.ch[c_2].id;
    commit_crossings(g, &cross, l, id, shell);
    g.stats.glasspen.via_arm += 1;
    Some(ArmThroughGlass { e, panes: cross.len() as f64 })
}

/// the line of fire across the panes a segment meets: does a round of `energy` get through every one (glasspen.ts shotPasses)
pub fn shot_passes(g: &mut crate::game::Game, a: V3, rd: V3, panes: &[crate::scene_collision::SegmentBlockedPanesItem], energy: Option<f64>) -> bool {
    let energy = energy.unwrap_or(g.k.glasspen.default_energy);
    g.stats.glasspen.shot_asked += 1;
    let ok = crate::glassbreak::glass_break_asking(g, &mut |_g: &mut Game, _s: f64, _w: f64| -> bool {
        let m = unsafe { core::slice::from_raw_parts_mut(::sim::sim_scratch(), ::sim::sim_scratch_len()) };
        for k in 0..panes.len() {
            m[2 * k] = panes[k].bi;
            m[2 * k + 1] = panes[k].t;
        }
        ::sim::pen_passes(a[0], a[1], a[2], rd[0], rd[1], rd[2], panes.len(), energy) == 1
    });
    if ok { g.stats.glasspen.shot_through += 1; } else { g.stats.glasspen.shot_refused += 1; }
    ok
}

/// glass is not cover: how many of `eyes` have a line of fire to `p` through glass, no door leaf in the way (glasspen.ts shotThroughGlass)
pub fn shot_through_glass(g: &mut crate::game::Game, eyes: &[V3], p: V3, energy: Option<f64>) -> f64 {
    let energy = energy.unwrap_or(g.k.glasspen.stack_energy);
    if !glass_pen_live(g) { return 0.0; }
    let panes = crate::glass::glass_panes(g);
    let walls = crate::ask::Over::statics();
    let mut n = 0.0;
    for &e in eyes.iter() {
        let d = v3::sub(p, e);
        let l = v3::len(d);
        if l < 1e-3 { continue; }
        let rd = v3::scale(d, 1.0 / l);
        let mut glass = false;
        for &bi in panes.iter() {
            let b = g.col.boxes[bi as usize];
            if let Some(s) = crate::scene_collision::line_box(g, e, rd, b) {
                if s.t1 > 0.0 && s.t0 < l { glass = true; break; }
            }
        }
        if glass && !crate::scene_collision::shot_blocked_over(g, &walls, e, p, Some(0.02), Some(energy)) { n += 1.0; }
    }
    n
}

/// is there standing glass on this line of fire (glasspen.ts glassOnLine)
pub fn glass_on_line(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    if !glass_pen_live(g) { return false; }
    let mut panes: Vec<crate::scene_collision::SegmentBlockedPanesItem> = Vec::new();
    crate::scene_collision::segment_blocked(g, a, b, Some(0.02), Some(0.0), Some(&mut panes));
    !panes.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(o: V3, t0: f64, len: f64) -> RoundSeg { RoundSeg { o, d: [1.0, 0.0, 0.0], t0, len, e: 1.0 } }

    #[test]
    fn a_path_is_walked_by_its_legs() {
        let mut p = RoundPath::default();
        p.segs = vec![seg([0.0, 0.0, 0.0], 0.0, 2.0), seg([3.0, 0.0, 0.0], 3.0, 4.0)];
        // a T inside a pane's thickness belongs to the leg before it, and stops at that leg's end
        assert_eq!(seg_at(&p, 2.5).t0, 0.0);
        assert_eq!(path_at(&p, 2.5), [2.0, 0.0, 0.0]);
        assert_eq!(path_at(&p, 4.0), [4.0, 0.0, 0.0]);
        assert_eq!(path_at(&p, 99.0), [7.0, 0.0, 0.0]);
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

    // the module's numbers for a glass and this file's table are one list
    #[test]
    fn a_glass_number_is_the_module_s() {
        assert_eq!(pane_type_word(::sim::panes::ANNEALED as f64), PaneType::Annealed);
        assert_eq!(pane_type_word(::sim::panes::TEMPERED as f64), PaneType::Tempered);
        assert_eq!(pane_type_word(::sim::panes::LAMINATED as f64), PaneType::Laminated);
        let words = pane_types();
        for i in 0..words.len() { assert_eq!(words[i].as_str(), pane_type_word(i as f64).as_str()); }
    }

    // the layout this file reads is the one rust/sim writes, field by field, and the sizes agree
    #[test]
    fn a_crossing_reads_as_the_module_writes_it() {
        assert_eq!(CROSS, ::sim::pen::CROSS as f64);
        assert_eq!(LEG, ::sim::pen::LEG as f64);
        assert_eq!(MAX_CROSS, ::sim::pen::MAX_CROSS as f64);
        let x = ::sim::pen::Crossing {
            t: ::sim::pen::Toll { ty: ::sim::panes::LAMINATED, n: [1.0, 2.0, 3.0], cos: 4.0, take: 5.0, e_out: 6.0, hole: true },
            bi: 7, at_t: 8.0, at: [9.0, 10.0, 11.0], exit: [12.0, 13.0, 14.0], dir_in: [15.0, 16.0, 17.0], dir_out: [18.0, 19.0, 20.0], e_in: 21.0, turn_deg: 22.0, thru: 23.0, into: -1,
        };
        let mut m = [0.0; 3 + ::sim::pen::CROSS];
        ::sim::pen::write_cross(&x, &mut m[3..]);
        let c = cross_at(&m, 3.0, -1.0);
        assert_eq!(c.type_, PaneType::Laminated);
        assert_eq!((c.n, c.cos, c.take, c.e_out, c.hole), ([1.0, 2.0, 3.0], 4.0, 5.0, 6.0, true));
        assert_eq!((c.bi, c.t, c.at, c.exit, c.dir_in, c.dir_out), (7.0, 8.0, [9.0, 10.0, 11.0], [12.0, 13.0, 14.0], [15.0, 16.0, 17.0], [18.0, 19.0, 20.0]));
        assert_eq!((c.e_in, c.turn_deg, c.thru, c.into), (21.0, 22.0, 23.0, -1.0));
    }
}
