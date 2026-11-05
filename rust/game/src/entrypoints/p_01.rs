//! src/game/entrypoints.ts lines 1 to 176
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A new tracker: nothing known. (entrypoints.ts EntryPoints.constructor)
pub fn entry_points_new(g: &mut crate::game::Game) -> crate::entrypoints::EntryPoints {
    let mut e = EntryPoints::blank();
    e.knobs = g.k.entry;
    e
}

/// Nothing known: a restart. A point already marked moved stays so, as in the TypeScript. (entrypoints.ts EntryPoints.reset)
pub fn entry_points_reset(g: &mut crate::game::Game) {
    g.entry.been.clear();
    g.entry.last = -1.0;
    g.entry.stats = EntryPointsStats { pods: 0.0, risky: 0.0, moved: 0.0, kept: 0.0 };
    g.entry.log.clear();
}

/// The level's openings as this module counts them: the doors. (entrypoints.ts EntryPoints.openings)
pub fn entry_points_openings(g: &mut crate::game::Game) -> Vec<CoverOpeningId> {
    let all = crate::cover::cover_ready(g);
    all.into_iter().filter(|&o| g.tab.cover_opening[o].kind == crate::cover::OpeningKind::Door).collect()
}

/// Four times a second: where the squad stands is known ground. (entrypoints.ts EntryPoints.tick)
pub fn entry_points_tick(g: &mut crate::game::Game) {
    if !crate::seats::team_active(g) { return; }
    if g.entry.last >= 0.0 && m::abs(g.time - g.entry.last) < 0.25 { return; }
    g.entry.last = g.time;
    for gd in crate::seats::team_men(g) {
        if g.gd[gd].state != crate::game::GuardState::Dead {
            let c = g.gd[gd].ch;
            let p = g.ch[c].pos;
            entry_points_mark(g, p);
        }
    }
    let who = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    for pl in crate::seats::men(g, &who) {
        let c = g.pl[pl].ch;
        let p = g.ch[c].pos;
        entry_points_mark(g, p);
    }
}

/// One of ours stands at `p`: his side of every near opening he has a line to is known; in the opening, both. (entrypoints.ts EntryPoints.mark)
pub fn entry_points_mark(g: &mut crate::game::Game, p: V3) {
    let known_m = g.k.entry.known_m;
    for o in crate::cover::cover_ready(g) {
        let (at, n, half) = opening_shape(g, o);
        let dx = p[0] - at[0];
        let dz = p[2] - at[2];
        if dx * dx + dz * dz > known_m * known_m { continue; }
        let s = dx * n[0] + dz * n[1];
        let in_it = m::abs(s) < 0.3 && m::abs(dx * n[1] - dz * n[0]) <= half + 0.2;
        let had = g.entry.been.get(o).copied().unwrap_or(0.0);
        let bits: i32 = if in_it { 3 } else if s > 0.0 { 1 } else { 2 };
        let had_bits = js::to_i32(had);
        if (had_bits & bits) == bits { continue; }
        if !in_it && !entry_points_sees(g, p, o, if s >= 0.0 { 1.0 } else { -1.0 }, false) { continue; }
        g.entry.been.set(o, (had_bits | bits) as f64);
    }
}

/// A line from a man at `p` to the face of `o` on side `sg` (its middle, or either third of its width). `open`: leaves out of the way. (entrypoints.ts EntryPoints.sees)
pub fn entry_points_sees(g: &mut crate::game::Game, p: V3, o: CoverOpeningId, sg: f64, open: bool) -> bool {
    let eye: V3 = [p[0], EYE, p[2]];
    let (at, n, half) = opening_shape(g, o);
    for k in [0.0, 0.6, -0.6] {
        let q: V3 = [at[0] + n[1] * half * k + n[0] * sg * 0.3, CHEST, at[2] - n[0] * half * k + n[1] * sg * 0.3];
        let clear = if open { !crate::roomclear::wall_between(g, eye, q) } else { !crate::scene_collision::segment_blocked(g, eye, q, None, None, None) };
        if clear { return true; }
    }
    false
}

/// Has one of the squad been on the side of `o` that `p` is NOT on. (entrypoints.ts EntryPoints.farKnown)
pub fn entry_points_far_known(g: &mut crate::game::Game, o: CoverOpeningId, p: V3) -> bool {
    let (at, n, _) = opening_shape(g, o);
    let s = (p[0] - at[0]) * n[0] + (p[2] - at[2]) * n[1];
    let had = js::to_i32(g.entry.been.get(o).copied().unwrap_or(0.0));
    (had & if s >= 0.0 { 2 } else { 1 }) != 0
}

/// The nearest uncleared opening that makes `p` a bad place to stand still. `skip`: the door this entry comes in by. `gaps`: count the nav grid's gaps too. (entrypoints.ts EntryPoints.riskAt)
pub fn entry_points_risk_at(g: &mut crate::game::Game, p: V3, skip: Option<DoorId>, gaps: Option<bool>) -> Option<CoverOpeningId> {
    let gaps = gaps.unwrap_or(false);
    let k = g.k.entry;
    let tan = m::tan(k.line_deg * m::PI / 180.0);
    let mut best: Option<CoverOpeningId> = None;
    let mut bd = m::INF;
    for o in crate::cover::cover_ready(g) {
        if !gaps && g.tab.cover_opening[o].kind != crate::cover::OpeningKind::Door { continue; }
        if skip.is_some() && g.tab.cover_opening[o].door == skip { continue; }
        let (at, n, half) = opening_shape(g, o);
        let dx = p[0] - at[0];
        let dz = p[2] - at[2];
        if m::abs(dx) > k.near_m + half || m::abs(dz) > k.near_m + half { continue; }
        let s = dx * n[0] + dz * n[1];
        let b = m::abs(s);
        let l = m::abs(dx * n[1] - dz * n[0]);
        let d = m::hyp2(m::max(0.0, l - half), b);
        if d > k.near_m || d >= bd || l > half + b * tan { continue; }
        if entry_points_far_known(g, o, p) { continue; }
        let sg = if s >= 0.0 { 1.0 } else { -1.0 };
        if crate::scene_collision::nav_grid_is_blocked(g, at[0] - n[0] * sg * 0.9, at[2] - n[1] * sg * 0.9)
            && crate::scene_collision::nav_grid_is_blocked(g, at[0] - n[0] * sg * 1.5, at[2] - n[1] * sg * 1.5) { continue; }
        let to: V3 = [p[0], CHEST, p[2]];
        let mut seen = false;
        for j in [0.0, 0.7, -0.7] {
            let ax = at[0] + n[1] * half * j + n[0] * sg * 0.2;
            let az = at[2] - n[0] * half * j + n[1] * sg * 0.2;
            if !crate::roomclear::wall_between(g, [ax, EYE, az], to) { seen = true; break; }
        }
        if !seen { continue; }
        best = Some(o);
        bd = d;
    }
    best
}

/// An entry's points through door `d`, each risky one moved off its opening where the room has the floor and given that opening to hold a gun on. `frame(p)`: p in the door's frame. (entrypoints.ts EntryPoints.clearOf)
pub fn entry_points_clear_of(g: &mut crate::game::Game, d: DoorId, pods: &[crate::entrypoints::Pod], frame: &mut dyn FnMut(&mut crate::game::Game, V3) -> [f64; 2]) -> Vec<crate::entrypoints::Pod> {
    let k = g.k.entry;
    if pods.is_empty() { return pods.to_vec(); }
    let cs = g.col.nav.cell;
    let w = g.dr[d].def.width;
    let mut out: Vec<Pod> = pods.to_vec();
    for i in 0..out.len() {
        let pod = out[i];
        let Some(by) = entry_points_risk_at(g, pod.pos, Some(d), None) else { continue };
        let (by_at, by_n, _) = opening_shape(g, by);
        let watch: V3 = [by_at[0], 1.2, by_at[2]];
        let [a0, _b0] = frame(g, pod.pos);
        let funnel = w / 2.0 + 0.3;
        let r = m::ceil(k.move_m / cs);
        let cx = m::floor(pod.pos[0] / cs);
        let cz = m::floor(pod.pos[2] / cs);
        let room = crate::roomfight::room_fight_room_at(g, pod.pos);
        let mut best: Option<V3> = None;
        let mut bs = m::INF;
        let mut z = cz - r;
        while z <= cz + r {
            let mut x = cx - r;
            while x <= cx + r {
                let q: V3 = [(x + 0.5) * cs, 0.0, (z + 0.5) * cs];
                x += 1.0;
                let dist = m::hyp2(q[0] - pod.pos[0], q[2] - pod.pos[2]);
                if dist > k.move_m || dist >= bs || crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { continue; }
                let [a, b] = frame(g, q);
                if b < 0.3 { continue; }
                if m::abs(a0) > 0.3 && a * a0 < 0.0 { continue; }
                if m::abs(a) < m::min(m::abs(a0), funnel) { continue; }
                if out.iter().enumerate().any(|(j, o)| j != i && m::hyp2(o.pos[0] - q[0], o.pos[2] - q[2]) < k.apart) { continue; }
                if room.is_some() && crate::roomfight::room_fight_room_at(g, q) != room { continue; }
                if crate::scene_collision::solid_blocked(g, [pod.pos[0], 0.45, pod.pos[2]], [q[0], 0.45, q[2]], None) { continue; }
                if entry_points_risk_at(g, q, Some(d), None).is_some() { continue; }
                if !entry_points_sees(g, q, by, side_of(by_at, by_n, q), true) { continue; }
                best = Some(q);
                bs = dist;
            }
            z += 1.0;
        }
        let Some(best) = best else {
            // it stays where it is: its man can at least hold the opening
            if entry_points_sees(g, pod.pos, by, side_of(by_at, by_n, pod.pos), true) {
                out[i] = Pod { pos: pod.pos, face: pod.face, watch: Some(watch), moved: false };
            }
            continue;
        };
        out[i] = Pod { pos: best, face: pod.face, watch: Some(watch), moved: true };
        let line = format!(
            "{} · ({}, {}) → ({}, {}) · off {}",
            g.dr[d].def.name,
            to_fixed_1(pod.pos[0]),
            to_fixed_1(pod.pos[2]),
            to_fixed_1(best[0]),
            to_fixed_1(best[2]),
            g.tab.cover_opening[by].id
        );
        g.entry.log.push(line);
        if g.entry.log.len() > 60 { g.entry.log.remove(0); }
    }
    out
}

/// A search point taken over a better-seeing one by an uncleared opening: it counts as moved. (entrypoints.ts EntryPoints.swapped)
pub fn entry_points_swapped(g: &mut crate::game::Game, pod: crate::entrypoints::Pod) {
    todo!("needs: `pod: &mut Pod`: by value, Pod.moved cannot reach the caller's point (entrypoints.ts:170, doortools.ts:3189)")
}

/// A point an entry has just dealt to a man: the tallies. (entrypoints.ts EntryPoints.dealt)
pub fn entry_points_dealt(g: &mut crate::game::Game, pod: crate::entrypoints::Pod, skip: Option<DoorId>) {
    g.entry.stats.pods += 1.0;
    let risky = entry_points_risk_at(g, pod.pos, skip, None).is_some();
    if risky { g.entry.stats.risky += 1.0; }
    if pod.moved {
        g.entry.stats.moved += 1.0;
    } else if risky {
        g.entry.stats.kept += 1.0;
    }
}

/// The tallies in a line. (entrypoints.ts EntryPoints.summary)
pub fn entry_points_summary(g: &mut crate::game::Game) -> String {
    let s = g.entry.stats;
    let n = g.entry.been.values().filter(|&&v| js::truthy(v)).count();
    format!(
        "entry points: {} dealt · {} by an uncleared opening · {} moved off one · {} with nowhere to go · {} openings known",
        s.pods, s.risky, s.moved, s.kept, n
    )
}

fn opening_shape(g: &Game, o: CoverOpeningId) -> (V3, [f64; 2], f64) {
    let row = &g.tab.cover_opening[o];
    (row.at, row.n, row.half)
}

/// Which side of the opening at `at` facing `n` the point `q` is on (clearOf's sideOf).
fn side_of(at: V3, n: [f64; 2], q: V3) -> f64 {
    if (q[0] - at[0]) * n[0] + (q[2] - at[2]) * n[1] >= 0.0 { 1.0 } else { -1.0 }
}

/// `x.toFixed(1)`: the exact value to the nearest tenth, an exact tie going up in size; −0 has no sign. A double's only ties at a tenth are odd quarters.
fn to_fixed_1(x: f64) -> String {
    let a = m::abs(x);
    let quarters = a * 4.0;
    if m::floor(quarters) == quarters && js::rem(quarters, 2.0) != 0.0 {
        let tenths = (a * 10.0 + 0.5) as u64;
        let sign = if x < 0.0 { "-" } else { "" };
        return format!("{}{}.{}", sign, tenths / 10, tenths % 10);
    }
    if x == 0.0 { return String::from("0.0"); }
    format!("{:.1}", x)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each expected string is what `x.toFixed(1)` printed under node.
    #[test]
    fn to_fixed_1_is_javascripts() {
        let cases: [(f64, &str); 17] = [
            (0.25, "0.3"), (0.75, "0.8"), (3.25, "3.3"), (-3.25, "-3.3"), (2.5, "2.5"), (0.05, "0.1"), (0.35, "0.3"), (1.45, "1.4"), (-0.04, "-0.0"),
            (-0.0, "0.0"), (0.0, "0.0"), (12.75, "12.8"), (1.05, "1.1"), (100.25, "100.3"), (-0.25, "-0.3"), (0.15, "0.1"), (7.999, "8.0"),
        ];
        for (x, want) in cases { assert_eq!(to_fixed_1(x), want, "{x}"); }
    }

    #[test]
    fn side_of_counts_the_plane_itself_as_plus() {
        let (at, n) = ([1.0, 0.0, 2.0], [0.0, 1.0]);
        assert_eq!(side_of(at, n, [5.0, 0.0, 3.0]), 1.0);
        assert_eq!(side_of(at, n, [5.0, 0.0, 2.0]), 1.0);
        assert_eq!(side_of(at, n, [5.0, 0.0, 1.0]), -1.0);
        assert_eq!(side_of(at, n, [m::NAN, 0.0, 1.0]), -1.0);
    }

    /// Bit 1 is the +n side, bit 2 the -n side; `far_known` asks after the side `p` is not on, and `reset` forgets.
    #[test]
    fn far_known_reads_the_other_side_and_reset_forgets() {
        let mut g = Game::blank();
        let o = g.tab.cover_opening.push(crate::cover::Opening { at: [1.0, 0.0, 2.0], n: [0.0, 1.0], half: 0.5, ..crate::cover::Opening::zeroed() });
        let (plus, minus): (V3, V3) = ([1.0, 0.0, 3.0], [1.0, 0.0, 1.0]);
        assert!(!entry_points_far_known(&mut g, o, plus));
        g.entry.been.set(o, 1.0);
        assert!(entry_points_far_known(&mut g, o, minus));
        assert!(!entry_points_far_known(&mut g, o, plus));
        g.entry.been.set(o, 3.0);
        assert!(entry_points_far_known(&mut g, o, plus));
        assert!(entry_points_summary(&mut g).ends_with("· 1 openings known"));
        g.entry.last = 9.0;
        g.entry.stats.pods = 4.0;
        g.entry.log.push(String::from("x"));
        entry_points_reset(&mut g);
        assert!(!entry_points_far_known(&mut g, o, plus));
        assert_eq!((g.entry.last, g.entry.stats.pods, g.entry.log.len()), (-1.0, 0.0, 0));
    }

    /// The summary's counts print as a JavaScript template prints a whole number.
    #[test]
    fn whole_counts_print_with_no_point() {
        assert_eq!(format!("{}", 3.0_f64), "3");
        assert_eq!(format!("{}", 0.0_f64), "0");
    }
}
