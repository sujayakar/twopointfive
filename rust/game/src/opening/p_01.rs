//! src/game/opening.ts lines 1 to 95
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `gd.routeI < 0` (opening.ts:81): the field is a usize here and the TypeScript's -1 (a spawned man) does not fit in it.
fn seam_route_i_negative(g: &mut Game, gd: GuardId) -> bool {
    todo!("needs: Guard.route_i able to hold -1")
}

/// The point `s` metres along p → q, on the floor. (opening.ts:65)
fn leg_point(p: V3, q: V3, s: f64, l: f64) -> V3 {
    [p[0] + (q[0] - p[0]) * s / l, 0.0, p[2] + (q[2] - p[2]) * s / l]
}

/// Marks the address word as read; the reading itself (?fairstart=0) is the host's and sits behind the settled row OPENING.on. (opening.ts readUrl)
pub fn read_url(g: &mut crate::game::Game) {
    g.m.opening.url_read = true;
}

/// Length on the floor. (opening.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// Metres by walking a → b over the nav grid; infinity if there is no way. (opening.ts wayLen)
pub fn way_len(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    let p = if crate::scene_collision::nav_grid_is_blocked(g, a[0], a[2]) { crate::scene_collision::nav_grid_nearest_free_point(g, a[0], a[2]).unwrap_or(a) } else { a };
    let q = if crate::scene_collision::nav_grid_is_blocked(g, b[0], b[2]) { crate::scene_collision::nav_grid_nearest_free_point(g, b[0], b[2]).unwrap_or(b) } else { b };
    let path = match crate::scene_collision::nav_grid_find_path(g, p, q, Some(8000.0), None) {
        Some(path) => path,
        None => return f64::INFINITY,
    };
    let mut l = 0.0;
    let mut at = p;
    for w in path {
        l += flat(at, w);
        at = w;
    }
    l + flat(at, q)
}

/// Where along his route a man who starts inside the clear radius is put, or none; kept per route. (opening.ts openingSpot)
pub fn opening_spot(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::opening::OpeningSpot> {
    let spawn = g.level.player_spawn;
    let key = alloc::format!("{}|{}|{}|{}", g.k.opening.clear_m, g.k.opening.step_m, spawn[0], spawn[2]);
    let route = g.gd[gd].route;
    if let Some(kept) = g.m.opening.spots.get(route) {
        if kept.key == key {
            return kept.at.map(|at| crate::opening::OpeningSpot { pos: at.pos, yaw: at.yaw, wp: at.wp });
        }
    }
    let at = find_spot(g, gd);
    let kept_at = at.map(|at| crate::opening::OpeningModSpotsValueAt { pos: at.pos, yaw: at.yaw, wp: at.wp });
    g.m.opening.spots.set(route, crate::opening::OpeningModSpotsValue { key, at: kept_at });
    at.map(|at| crate::opening::OpeningSpot { pos: at.pos, yaw: at.yaw, wp: at.wp })
}

/// The first walkable point along his route at least the clear radius from the spawn by walking. (opening.ts findSpot)
pub fn find_spot(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::opening::FindSpot> {
    let route = g.gd[gd].route;
    let pts = g.tab.route[route].points.clone();
    let n = pts.len();
    let spawn = g.level.player_spawn;
    if n < 2 || way_len(g, spawn, pts[0]) >= g.k.opening.clear_m {
        return None;
    }
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        let mut leg: Vec<V3> = vec![a];
        match crate::scene_collision::nav_grid_find_path(g, a, b, Some(8000.0), None) {
            Some(path) => leg.extend(path),
            None => leg.push(b),
        }
        let mut k = 0;
        while k + 1 < leg.len() {
            let p = leg[k];
            let q = leg[k + 1];
            let l = flat(p, q);
            if l < 1e-6 {
                k += 1;
                continue;
            }
            let mut s = if k == 0 && i == 0 { g.k.opening.step_m } else { 0.0 };
            while s <= l {
                let at = leg_point(p, q, s, l);
                if !crate::scene_collision::nav_grid_is_blocked(g, at[0], at[2]) {
                    let far = way_len(g, spawn, at);
                    // no way found = not known to be clear: on to the next point
                    if far.is_finite() && far >= g.k.opening.clear_m {
                        return Some(crate::opening::FindSpot { pos: at, yaw: v3::yaw_to(p, q), wp: ((i + 1) % n) as f64 });
                    }
                }
                s += g.k.opening.step_m;
            }
            k += 1;
        }
    }
    None
}

/// Puts the fresh shift on for the player's night; each man is looked at once. Returns how many were moved now. (opening.ts openingStart)
pub fn opening_start(g: &mut crate::game::Game) -> f64 {
    if !g.m.opening.url_read {
        read_url(g);
    }
    // a show's own encounter (its men start from their marks), not one it has handed to the player
    let show = g.quiet_utility || (g.touring && !crate::director::director_handed_over(g));
    let mut moved = 0.0;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.m.opening.seen.has(gd) || g.gd[gd].unit.is_some() || g.gd[gd].team != crate::teams::Team::Guard || seam_route_i_negative(g, gd) {
            continue;
        }
        if !g.m.opening.born.has(gd) {
            let now = g.time;
            g.m.opening.born.set(gd, now);
        }
        if show {
            continue;
        }
        g.m.opening.seen.add(gd);
        let c = g.gd[gd].ch;
        let born = *g.m.opening.born.get(gd).unwrap();
        // not fresh: a man already on his beat is where the night put him
        if g.time - born > FRESH_SECS
            || g.gd[gd].state != crate::game::GuardState::Patrol
            || g.gd[gd].wp != 0
            || g.gd[gd].hold
            || g.gd[gd].script.is_some()
            || flat(g.ch[c].pos, g.tab.route[g.gd[gd].route].points[0]) > 0.6
        {
            continue;
        }
        let at = match opening_spot(g, gd) {
            Some(at) => at,
            None => continue,
        };
        g.ch[c].pos = v3::copy(at.pos);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.ch[c].aim_yaw = at.yaw;
        g.ch[c].body_yaw = at.yaw;
        g.gd[gd].last_pos = v3::copy(at.pos);
        g.gd[gd].wp = at.wp as usize;
        g.gd[gd].wait = 0.0;
        g.gd[gd].path = Vec::new();
        g.gd[gd].path_i = 0;
        g.gd[gd].path_goal = None;
        crate::character::character_update(g, c, 0.0); // bones where he now stands, before anything reads them this frame
        moved += 1.0;
    }
    moved
}

#[cfg(test)]
mod tests {
    use super::*;

    // The expected bits are the TypeScript's own, under bun with EXACT.on = true.
    #[test]
    fn flat_matches_the_typescript_bits() {
        assert_eq!(flat([3.1, 0.0, 4.7], [-0.3, 0.0, 9.21]).to_bits(), 0x40169791dc4b67b8);
        assert_eq!(flat([12.5, 0.0, -7.25], [12.5, 0.0, -7.25]).to_bits(), 0);
        assert_eq!(flat([0.1, 0.0, 0.2], [0.3, 0.0, 1e-9]).to_bits(), 0x3fd21a18513d04d3);
    }

    #[test]
    fn walking_a_leg_matches_the_typescript_bits() {
        let (p, q): (V3, V3) = ([1.3, 0.0, -2.7], [8.9, 0.0, 4.1]);
        let l = flat(p, q);
        assert_eq!(l.to_bits(), 0x402465655f122ff6);
        let want: [(usize, u64, u64, u64); 3] = [
            (0, 0x3fe0000000000000, 0x3ffac30de0b68c0d, 0xc002eecd540f5841),
            (7, 0x4010000000000000, 0x40111fb55b06b1b4, 0xbfa0cddb51e3b3c0),
            (14, 0x401e000000000000, 0x401b8ea73ddfc065, 0x4002685e79803aa2),
        ];
        let (mut n, mut s) = (0, 0.5);
        while s <= l {
            let at = leg_point(p, q, s, l);
            for w in want.iter().filter(|w| w.0 == n) {
                assert_eq!((s.to_bits(), at[0].to_bits(), at[2].to_bits()), (w.1, w.2, w.3));
            }
            n += 1;
            s += 0.5;
        }
        assert_eq!(n, 20);
    }
}
