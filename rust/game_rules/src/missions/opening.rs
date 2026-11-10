pub use ::game_types::opening::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{vec, vec::Vec};

/// The point `s` metres along p → q, on the floor.
fn leg_point(p: V3, q: V3, s: f64, l: f64) -> V3 {
    [p[0] + (q[0] - p[0]) * s / l, 0.0, p[2] + (q[2] - p[2]) * s / l]
}

/// Marks the address word as read; the reading itself (?fairstart=0) is the host's and sits behind the settled row OPENING.on.
fn read_url(g: &mut crate::game::Game) {
    g.m.opening.url_read = true;
}

/// Length on the floor.
fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// Metres by walking a → b over the nav grid; infinity if there is no way.
fn way_len(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    let p = if crate::scene_collision::nav_grid_is_blocked(g, a[0], a[2]) { crate::scene_collision::nav_grid_nearest_free_point(g, a[0], a[2]).unwrap_or(a) } else { a };
    let q = if crate::scene_collision::nav_grid_is_blocked(g, b[0], b[2]) { crate::scene_collision::nav_grid_nearest_free_point(g, b[0], b[2]).unwrap_or(b) } else { b };
    let path = match crate::scene_collision::nav_grid_find_path(g, p, q, Some(8000.0), None) {
        Some(path) => path,
        None => return f64::INFINITY,
    };
    crate::places::walk_len(p, &path) + flat(path.last().copied().unwrap_or(p), q)
}

/// Where along his route a man who starts inside the clear radius is put, or none; kept per route.
fn opening_spot(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::opening::OpeningSpot> {
    let spawn = g.level.player_spawn;
    let key = alloc::format!("{}|{}|{}|{}", js::num_str(g.k.opening.clear_m), js::num_str(g.k.opening.step_m), js::num_str(spawn[0]), js::num_str(spawn[2]));
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

/// The first walkable point along his route at least the clear radius from the spawn by walking.
fn find_spot(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::opening::FindSpot> {
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

/// Puts the fresh shift on for the player's night; each man is looked at once. Returns how many were moved now.
pub fn opening_start(g: &mut crate::game::Game) -> f64 {
    if !g.m.opening.url_read {
        read_url(g);
    }
    let mut moved = 0.0;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.m.opening.seen.has(gd) || g.gd[gd].unit.is_some() || g.gd[gd].team != crate::teams::Team::Guard || g.gd[gd].route_i < 0 {
            continue;
        }
        if !g.m.opening.born.has(gd) {
            let now = g.time;
            g.m.opening.born.set(gd, now);
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
        { let to = v3::copy(at.pos); crate::character::pos_replace(g, c, to); }
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.ch[c].aim_yaw = at.yaw;
        g.ch[c].body_yaw = at.yaw;
        g.gd[gd].last_pos = v3::copy(at.pos);
        g.gd[gd].wp = at.wp as usize;
        g.gd[gd].wait = 0.0;
        crate::game::guard_set_path(g, gd, Vec::new());
        g.gd[gd].path_i = 0;
        crate::guards::guard_set_path_goal(g, gd, None);
        crate::character::character_update(g, c, 0.0); // bones where he now stands, before anything reads them this frame
        moved += 1.0;
    }
    moved
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A FAILURE MEANS: the guard the fair start moves is put off his own route, or off the floor: a place along a leg lies between its ends, at its share, at height nothing.
    #[test]
    fn a_place_along_a_leg_lies_on_it_on_the_floor() {
        let (p, q): (V3, V3) = ([1.0, 2.0, -2.0], [4.0, 3.0, 2.0]);
        let l = flat(p, q);
        assert_eq!(leg_point(p, q, 0.0, l), [1.0, 0.0, -2.0]);
        assert_eq!(leg_point(p, q, l, l), [4.0, 0.0, 2.0]);
        assert_eq!(leg_point(p, q, l / 2.0, l), [2.5, 0.0, 0.0]);
    }
}
