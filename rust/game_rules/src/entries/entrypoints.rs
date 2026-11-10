pub use ::game_types::entrypoints::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec::Vec};

/// A new tracker: nothing known.
pub fn entry_points_new() -> crate::entrypoints::EntryPoints {
    EntryPoints::blank()
}

/// Four times a second: where the squad stands is known ground.
pub fn entry_points_tick(g: &mut crate::game::Game) {
    if !crate::seats::team_active(g) { return; }
    if g.entry.last >= 0.0 && m::abs(g.time - g.entry.last) < 0.25 { return; }
    g.entry.last = g.time;
    for gd in crate::seats::team_men(g, &crate::seats::here(g)) {
        if !crate::condition::down(g, g.gd[gd].ch) {
            let c = g.gd[gd].ch;
            let p = g.ch[c].pos;
            entry_points_mark(g, p);
        }
    }
    for pl in crate::seats::men(g) {
        let c = g.pl[pl].ch;
        let p = g.ch[c].pos;
        entry_points_mark(g, p);
    }
}

/// One of ours stands at `p`: his side of every near opening he has a line to is known; in the opening, both.
fn entry_points_mark(g: &mut crate::game::Game, p: V3) {
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

/// A line from a man at `p` to the face of `o` on side `sg` (its middle, or either third of its width). `open`: leaves out of the way.
fn entry_points_sees(g: &crate::game::Game, p: V3, o: CoverOpeningId, sg: f64, open: bool) -> bool {
    let eye: V3 = [p[0], EYE, p[2]];
    let (at, n, half) = opening_shape(g, o);
    for k in [0.0, 0.6, -0.6] {
        let q: V3 = [at[0] + n[1] * half * k + n[0] * sg * 0.3, CHEST, at[2] - n[0] * half * k + n[1] * sg * 0.3];
        let clear = if open { !crate::scene_collision::wall_between(g, eye, q) } else { !crate::scene_collision::segment_blocked(g, eye, q, None, None, None) };
        if clear { return true; }
    }
    false
}

/// Has one of the squad been on the side of `o` that `p` is NOT on.
pub fn entry_points_far_known(g: &crate::game::Game, o: CoverOpeningId, p: V3) -> bool {
    let (at, n, _) = opening_shape(g, o);
    let s = (p[0] - at[0]) * n[0] + (p[2] - at[2]) * n[1];
    let had = js::to_i32(g.entry.been.get(o).copied().unwrap_or(0.0));
    (had & if s >= 0.0 { 2 } else { 1 }) != 0
}

/// The nearest uncleared opening that makes `p` a bad place to stand still. `skip`: the door this entry comes in by. `gaps`: count the nav grid's gaps too.
pub(super) fn entry_points_risk_at(g: &mut crate::game::Game, p: V3, skip: Option<DoorId>, gaps: Option<bool>) -> Option<CoverOpeningId> {
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
            if !crate::scene_collision::wall_between(g, [ax, EYE, az], to) { seen = true; break; }
        }
        if !seen { continue; }
        best = Some(o);
        bd = d;
    }
    best
}

/// An entry's points through door `d`, each risky one moved off its opening where the room has the floor and given that opening to hold a gun on. `frame(p)`: p in the door's frame.
pub(super) fn entry_points_clear_of(g: &mut crate::game::Game, d: DoorId, pods: &[crate::entrypoints::Pod], frame: &mut dyn FnMut(&mut crate::game::Game, V3) -> [f64; 2]) -> Vec<crate::entrypoints::Pod> {
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
    }
    out
}

/// A search point taken over a better-seeing one by an uncleared opening: it counts as moved.
pub(super) fn entry_points_swapped(pod: &mut crate::entrypoints::Pod) {
    pod.moved = true;
}

/// A point an entry has just dealt to a man: the tallies.
pub(super) fn entry_points_dealt(g: &mut crate::game::Game, pod: crate::entrypoints::Pod, skip: Option<DoorId>) {
    let risky = entry_points_risk_at(g, pod.pos, skip, None).is_some();
    if pod.moved {
    } else if risky {
    }
}

fn opening_shape(g: &Game, o: CoverOpeningId) -> (V3, [f64; 2], f64) {
    let row = &g.tab.cover_opening[o];
    (row.at, row.n, row.half)
}

/// Which side of the opening at `at` facing `n` the point `q` is on (clearOf's sideOf).
fn side_of(at: V3, n: [f64; 2], q: V3) -> f64 {
    if (q[0] - at[0]) * n[0] + (q[2] - at[2]) * n[1] >= 0.0 { 1.0 } else { -1.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn side_of_counts_the_plane_itself_as_plus() {
        let (at, n) = ([1.0, 0.0, 2.0], [0.0, 1.0]);
        assert_eq!(side_of(at, n, [5.0, 0.0, 3.0]), 1.0);
        assert_eq!(side_of(at, n, [5.0, 0.0, 2.0]), 1.0);
        assert_eq!(side_of(at, n, [5.0, 0.0, 1.0]), -1.0);
    }

}
