//! src/game/fireline.ts lines 1 to 224
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The blocked write of fireline.ts:166: `FIRELINE_STATS.heldS` counts seconds and FirelineStats.held_s is an i32.
fn seam_stats_held_s(g: &mut Game, secs: f64) {
    todo!("needs: FirelineStats.held_s as f64")
}

/// Is the rule running: a round of the player's is a round to his own men. (fireline.ts live)
pub fn live(g: &mut crate::game::Game) -> bool {
    crate::friendlyfire::ff_live(g)
}

/// Once a frame before the men are posed: is the player firing, by the rounds counted off the gun in his hands. (fireline.ts fireLineTick)
pub fn fire_line_tick(g: &mut crate::game::Game) {
    let pl = g.player;
    let gun = crate::game::player_gun_in_hand(g, pl);
    let t = g.time;
    let Some(gun) = gun else { return };
    let def = g.fa[gun].def;
    let shots_fired = g.fa[gun].shots_fired;
    let afresh = match g.m.fireline.fired.get(pl) {
        None => true,
        Some(f) => f.gun.as_str() != def.as_str() || shots_fired < f.shots || t < m::max(f.live_t, f.round_t),
    };
    if afresh {
        g.m.fireline.fired.set(pl, Fired { gun: String::from(def.as_str()), shots: shots_fired, round_t: -1e9, gap: 1e9, live_t: -1e9, rel_t: -1e9 });
        return;
    }
    let f = g.m.fireline.fired.get_mut(pl).unwrap();
    if shots_fired > f.shots {
        f.gap = t - f.round_t;
        f.round_t = t;
        f.shots = shots_fired;
    }
    let (round_t, gap, live_t) = (f.round_t, f.gap, f.live_t);
    if g.pl[pl].down || g.pl[pl].sprinting {
        return;
    }
    let since = t - round_t;
    let (fire_s, pace_s, pace_k, reload_s) = (g.k.fireline.fire_s, g.k.fireline.pace_s, g.k.fireline.pace_k, g.k.fireline.reload_s);
    let mut on = since < m::max(fire_s, if gap < pace_s { m::min(pace_s, gap * pace_k) } else { 0.0 });
    if !on && crate::weapons::firearm_reloading(g, gun) && t - live_t < 0.1 {
        on = true;
        g.m.fireline.fired.get_mut(pl).unwrap().rel_t = t;
    }
    let f = g.m.fireline.fired.get_mut(pl).unwrap();
    if on || t - f.rel_t < reload_s {
        f.live_t = t;
    }
}

/// The lane of the player's fire this frame, or None: he is not firing. (fireline.ts laneOf)
pub fn lane_of(g: &mut crate::game::Game) -> Option<crate::fireline::Lane> {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let gun = crate::game::player_gun_in_hand(g, pl);
    let Some(gun) = gun else { return None };
    let Some(live_t) = g.m.fireline.fired.get(pl).map(|f| f.live_t) else { return None };
    if m::abs(g.time - live_t) > 1e-6 {
        return None;
    }
    let (yaw, pos, muzzle) = (g.ch[c].aim_yaw, g.ch[c].pos, g.ch[c].muzzle);
    let dx = m::sin(yaw);
    let dz = m::cos(yaw);
    let back = m::min(0.0, (pos[0] - muzzle[0]) * dx + (pos[2] - muzzle[2]) * dz) - 0.35;
    let k = g.k.friendlyfire.cone_k * g.k.fireline.spread;
    let half = g.k.friendlyfire.body_m + g.k.fireline.clear_m;
    // A copy of the gun's row: these calls take `g` and the row lives in it.
    let def = g.m.weapons.weapons.get(g.fa[gun].def).unwrap().clone();
    let energy = crate::glasspen::shot_energy(g, c, Some(&def));
    let thin = if crate::wallpen::wall_pen_live(g) { crate::glasspen::round_energy(g, &def) } else { -1.0 };
    Some(Lane { m: muzzle, dx, dz, back, k, half, energy, thin })
}

/// Is this point of floor inside the lane by its shape alone. (fireline.ts inShape)
pub fn in_shape(g: &mut crate::game::Game, l: crate::fireline::Lane, x: f64, z: f64, more: Option<f64>) -> bool {
    let more = more.unwrap_or(0.0);
    let rx = x - l.m[0];
    let rz = z - l.m[2];
    let a = rx * l.dx + rz * l.dz;
    a > l.back - more && a < g.k.fireline.reach_m && m::abs(rx * l.dz - rz * l.dx) <= l.half + more + m::max(0.0, a) * l.k
}

/// Would a round of this lane reach the point. (the closure `reaches` of fireline.ts inLane)
fn reaches(g: &mut Game, l: Lane, p: V3) -> bool {
    !crate::scene_collision::shot_blocked(g, l.m, p, Some(0.02), l.energy) || (l.thin >= 0.0 && crate::wallpen::thin_line(g, l.m, p, l.thin, None, None).is_some())
}

/// Would a round reach a man standing here: his chest, or either edge of his column. (fireline.ts inLane)
pub fn in_lane(g: &mut crate::game::Game, l: crate::fireline::Lane, x: f64, z: f64, more: Option<f64>) -> bool {
    if !in_shape(g, l, x, z, more) {
        return false;
    }
    reaches(g, l, [x, 1.2, z]) || reaches(g, l, [x + l.dz * 0.25, 1.2, z - l.dx * 0.25]) || reaches(g, l, [x - l.dz * 0.25, 1.2, z + l.dx * 0.25])
}

/// How far along the polyline `from` → `pts` (at most `far` of it) the walk first stands where `hit` says; infinity = nowhere. (fireline.ts along)
pub fn along(from: V3, pts: &[V3], far: f64, hit: &mut dyn FnMut(f64, f64) -> bool) -> f64 {
    let (mut ax, mut az, mut gone) = (from[0], from[2], 0.0);
    for b in pts {
        let len = m::hyp2(b[0] - ax, b[2] - az);
        if len < 1e-3 {
            continue;
        }
        let n = m::max(1.0, m::ceil(len / 0.2));
        let mut i = 1.0;
        while i <= n {
            let u = i / n;
            let s = gone + len * u;
            if s > far {
                return f64::INFINITY;
            }
            if hit(ax + (b[0] - ax) * u, az + (b[2] - az) * u) {
                return s;
            }
            i += 1.0;
        }
        gone += len;
        ax = b[0];
        az = b[2];
    }
    f64::INFINITY
}

/// Where the walk first comes into the lane. (fireline.ts crossAt)
pub fn cross_at(g: &mut crate::game::Game, l: crate::fireline::Lane, from: V3, pts: &[V3], far: f64, more: Option<f64>) -> f64 {
    along(from, pts, far, &mut |x, z| in_lane(g, l, x, z, more))
}

/// Where the walk first stands in a doorway. (fireline.ts doorAt)
pub fn door_at(g: &mut crate::game::Game, from: V3, pts: &[V3], far: f64) -> f64 {
    along(from, pts, far, &mut |x, z| in_doorway(g, x, z))
}

/// Is this point of floor in a door's opening. (fireline.ts inDoorway)
pub fn in_doorway(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        let centre = g.dr[d].frame_centre;
        if m::hyp2(x - centre[0], z - centre[2]) > g.dr[d].def.width + 1.0 {
            continue;
        }
        if crate::doorway::in_opening(g, d, [x, 0.0, z], None) {
            return true;
        }
    }
    false
}

/// The corners of the walk he is on to `goal`: the runner's own path while it is for that goal, else the straight leg if short, else None. (fireline.ts walkOf)
pub fn walk_of(g: &mut crate::game::Game, gd: GuardId, goal: V3) -> Option<Vec<V3>> {
    let p = g.ch[g.gd[gd].ch].pos;
    if let Some(pg) = g.gd[gd].path_goal {
        if m::hyp2(pg[0] - goal[0], pg[2] - goal[2]) < 0.75 && g.gd[gd].path.len() > g.gd[gd].path_i {
            let mut walk = g.gd[gd].path[g.gd[gd].path_i..].to_vec();
            walk.push(goal);
            return Some(walk);
        }
    }
    if m::hyp2(goal[0] - p[0], goal[2] - p[2]) <= 2.0 { Some(vec![goal]) } else { None }
}

/// Would waiting leave him standing in an enemy's line, as far as he knows. (fireline.ts inEnemyLine)
pub fn in_enemy_line(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let p = g.ch[g.gd[gd].ch].pos;
    crate::picture::exposed_at(g, gd, p, None) > 0.0
}

/// The length of a walk from `from` through `pts`. (the closure `len` of fireline.ts wayRound)
fn walk_len(from: V3, pts: &[V3]) -> f64 {
    let (mut s, mut a) = (0.0, from);
    for b in pts {
        s += m::hyp2(b[0] - a[0], b[2] - a[2]);
        a = *b;
    }
    s
}

/// A way to `goal` that keeps out of the lane, no more than `wayMoreM` longer than the way across, or None. (fireline.ts wayRound)
pub fn way_round(g: &mut crate::game::Game, gd: GuardId, l: crate::fireline::Lane, goal: V3) -> Option<Vec<V3>> {
    let cs = g.col.nav.cell;
    let p = g.ch[g.gd[gd].ch].pos;
    let berth_m = g.k.fireline.berth_m;
    let mut penalty = |g: &mut Game, ix: f64, iz: f64| -> f64 {
        if in_shape(g, l, (ix + 0.5) * cs, (iz + 0.5) * cs, Some(berth_m)) { 40.0 } else { 1.0 }
    };
    let path = crate::scene_collision::nav_grid_find_path(g, p, goal, Some(2500.0), Some(&mut penalty));
    let path = match path {
        Some(path) if !path.is_empty() => path,
        _ => return None,
    };
    let mut way: Vec<V3> = path.iter().map(|q| [q[0], 0.0, q[2]]).collect();
    way.push(goal);
    let edge_m = g.k.fireline.edge_m;
    if cross_at(g, l, p, &way, f64::INFINITY, Some(edge_m)) < f64::INFINITY {
        return None;
    }
    if door_at(g, p, &way, f64::INFINITY) < f64::INFINITY {
        return None;
    }
    let across = walk_of(g, gd, goal).unwrap_or_else(|| vec![goal]);
    if walk_len(p, &way) > walk_len(p, &across) + g.k.fireline.way_more_m {
        return None;
    }
    Some(way)
}

/// The rule lets go of him: his wait, if any, is forgotten. The caller returns None. (fireline.ts end)
pub fn end(g: &mut crate::game::Game, gd: GuardId, st: Option<&crate::fireline::St>) {
    if st.is_some() {
        g.m.fireline.states.delete(gd);
    }
}

/// He stands: the wait's seconds counted as they pass. (fireline.ts hold)
pub fn hold(g: &mut crate::game::Game, gd: GuardId) -> crate::fireline::Hold {
    let t = g.time;
    let secs = m::min(0.1, m::max(0.0, t - g.m.fireline.states.get(gd).unwrap().t));
    seam_stats_held_s(g, secs);
    g.m.fireline.states.get_mut(gd).unwrap().t = t;
    // `serial` is player::Hold's, spilled here by name: this answer is only ever read for its speed.
    Hold { speed: 0.0, serial: 0 }
}

/// The row written back to the table, then `hold`, and the answer in the shape fireLineFeet returns.
fn hold_row(g: &mut Game, gd: GuardId, s: St) -> Option<FireLineFeet> {
    g.m.fireline.states.set(gd, s);
    let h = hold(g, gd);
    Some(FireLineFeet { goal: None, speed: h.speed })
}

/// Is he on a door order past the stack. (fireline.ts:185)
fn past_the_stack(g: &Game, gd: GuardId) -> bool {
    g.door_tools.ops.iter().any(|&o| {
        let op = &g.tab.doortools_door_op[o];
        op.phase != crate::doortools::Phase::Stack && op.men.contains(&gd)
    })
}

/// Is he on a wall charge. (fireline.ts:186)
fn on_wall_charge(g: &Game, gd: GuardId) -> bool {
    g.wall_charge.ops.iter().any(|&o| g.tab.wallcharge_wall_op[o].men.contains(&gd))
}

/// His feet while the player fires down a lane his walk would cross: speed 0 = wait where he stands, `goal` = the next corner of a way round, None = the order's own legs. (fireline.ts fireLineFeet)
pub fn fire_line_feet(g: &mut crate::game::Game, gd: GuardId, goal: Option<V3>, speed: f64, mine: bool) -> Option<crate::fireline::FireLineFeet> {
    // `st` is a copy of his row: every path that keeps the wait writes it back by the key before it returns (nothing called in between reads the table).
    let mut st: Option<St> = g.m.fireline.states.get(gd).cloned();
    if let Some(s) = &st {
        if g.time < s.t || g.time - s.t > 0.25 {
            g.m.fireline.states.delete(gd);
            st = None;
        }
    }
    if !live(g) || mine || goal.is_none() || !crate::friendlyfire::ff_side(g, Some(gd)) || !crate::friendlyfire::ff_body(g, gd) || g.gd[gd].held.is_some() {
        end(g, gd, st.as_ref());
        return None;
    }
    let goal = goal.unwrap();
    let Some(l) = lane_of(g) else {
        // The fire has stopped, or lapsed between two slow rounds: he walks on, and for `lapseS` a lane that comes back is the same wait.
        let Some(mut s) = st.filter(|s| s.way.is_none()) else {
            g.m.fireline.states.delete(gd);
            return None;
        };
        if s.go_t < 0.0 {
            s.go_t = g.time;
        }
        if g.time - s.go_t > g.k.fireline.lapse_s {
            g.m.fireline.states.delete(gd);
            return None;
        }
        s.t = g.time;
        g.m.fireline.states.set(gd, s);
        return None;
    };
    let p = g.ch[g.gd[gd].ch].pos;
    if crate::roomfight::room_fight_busy(g, gd, 1.0) {
        g.stats.fireline.fight += 1;
        end(g, gd, st.as_ref());
        return None;
    }
    if past_the_stack(g, gd) || on_wall_charge(g, gd) || in_doorway(g, p[0], p[2]) {
        end(g, gd, st.as_ref());
        return None;
    }
    // The way round was to a place the order no longer sends him: asked afresh, of the new one.
    let off_way = st.as_ref().and_then(|s| s.way.as_ref()).map_or(false, |way| {
        let last = way[way.len() - 1];
        m::hyp2(last[0] - goal[0], last[2] - goal[2]) > 0.75
    });
    if off_way {
        g.m.fireline.states.delete(gd);
        st = None;
    }
    let clear_m = g.k.fireline.clear_m;
    let on_way = st.as_ref().map_or(false, |s| s.way.is_some());
    if in_lane(g, l, p[0], p[2], Some(if on_way { -clear_m } else { 0.0 })) {
        // Already in it: not moved by this rule, but for a man WAITING whom the gun swings onto, who stands as he stood to the wait's ceiling.
        let max_s = g.k.fireline.max_s;
        let stands = match &st {
            Some(s) => !(s.way.is_some() || s.go_t >= 0.0 || g.time - s.since >= max_s),
            None => false,
        };
        if !stands || in_enemy_line(g, gd) {
            end(g, gd, st.as_ref());
            return None;
        }
        return hold_row(g, gd, st.unwrap());
    }
    if on_way {
        // On a way round: the next corner, while what is left of it keeps out of the lane.
        let mut s = st.take().unwrap();
        let way = s.way.as_mut().unwrap();
        while way.len() > 1 && m::hyp2(way[0][0] - p[0], way[0][2] - p[2]) < 0.6 {
            way.remove(0);
        }
        let short_m = g.k.fireline.short_m;
        if way.len() > 1 && cross_at(g, l, p, way, short_m, Some(-clear_m)) == f64::INFINITY {
            let next = way[0];
            s.t = g.time;
            g.m.fireline.states.set(gd, s);
            return Some(FireLineFeet { goal: Some(next), speed });
        }
        g.m.fireline.states.delete(gd);
    }
    // Does his way come into the lane: inside `shortM` of walking (a doorway between: that far beyond the frame), or on the slant with his next step inside `gapM` of its side.
    let walk = match walk_of(g, gd, goal) {
        Some(walk) => walk,
        None if st.is_some() => vec![goal],
        None => return None,
    };
    let keep = if st.is_some() { g.k.fireline.keep_m } else { 0.0 };
    let door = door_at(g, p, &walk, 1.0);
    let far = g.k.fireline.short_m + keep + (if door < f64::INFINITY { door + 0.9 } else { 0.0 });
    let mut near = cross_at(g, l, p, &walk, far, None) < f64::INFINITY;
    let gap_m = g.k.fireline.gap_m;
    if !near && cross_at(g, l, p, &walk, 0.35 + keep, Some(gap_m)) < f64::INFINITY {
        let look_m = g.k.fireline.look_m;
        near = cross_at(g, l, p, &walk, look_m, None) < f64::INFINITY;
    }
    if !near {
        end(g, gd, st.as_ref());
        return None;
    }
    if in_enemy_line(g, gd) {
        g.stats.fireline.enemy += 1;
        end(g, gd, st.as_ref());
        return None;
    }
    let mut s = match st {
        Some(s) => s,
        None => {
            let s = St { since: g.time, t: g.time, way: None, look_t: -1e9, go_t: -1.0, said: false };
            g.m.fireline.states.set(gd, s.clone());
            g.stats.fireline.holds += 1;
            s
        }
    };
    s.go_t = -1.0;
    if !s.said && g.time - s.since >= g.k.fireline.cue_s {
        s.said = true;
        crate::game::game_say(g, gd, "your line — holding.", None, None, None);
        g.stats.fireline.cues += 1;
    }
    // The ceiling: then a look for a way round, and again every `againS` (the men of a squad that stopped together do not all look on one frame).
    let place: i32 = g.guards.iter().position(|&x| x == gd).map_or(-1, |i| i as i32);
    if g.time - s.since >= g.k.fireline.max_s + (place & 3) as f64 * 0.3 && g.time - s.look_t >= g.k.fireline.again_s {
        s.look_t = g.time;
        if let Some(way) = way_round(g, gd, l, goal) {
            g.stats.fireline.ways += 1;
            let next = way[0];
            s.way = Some(way);
            s.t = g.time;
            g.m.fireline.states.set(gd, s);
            return Some(FireLineFeet { goal: Some(next), speed });
        }
        g.stats.fireline.no_way += 1;
    }
    hold_row(g, gd, s)
}

/// Is this rule holding him where he stands: nobody calls him stuck. (fireline.ts fireLineLive)
pub fn fire_line_live(g: &mut crate::game::Game, gd: GuardId) -> bool {
    match g.m.fireline.states.get(gd) {
        Some(st) => st.way.is_none() && st.go_t < 0.0 && g.time >= st.t && g.time - st.t <= 0.25,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bits, the hits and the last point asked, against fireline.ts `along` under bun with EXACT.on.
    fn run(far: f64, f: &dyn Fn(f64, f64) -> bool) -> (u64, i32, u64, u64) {
        let from = [0.1, 0.0, 0.3];
        let pts = [[1.7, 0.0, 2.9], [1.7, 0.0, 2.9], [-3.3, 0.0, 4.4]];
        let (mut calls, mut lx, mut lz) = (0, 0.0f64, 0.0f64);
        let s = along(from, &pts, far, &mut |x, z| {
            calls += 1;
            lx = x;
            lz = z;
            f(x, z)
        });
        (s.to_bits(), calls, lx.to_bits(), lz.to_bits())
    }

    #[test]
    fn along_matches_the_typescripts_bits() {
        assert_eq!(run(f64::INFINITY, &|x, _| x < -1.0), (0x4017cfd2d1826696, 31, 0xbff13e93e93e93e9, 0x400dddddddddddde));
        assert_eq!(run(f64::INFINITY, &|_, _| false), (0x7ff0000000000000, 43, 0xc00a666666666666, 0x401199999999999a));
        assert_eq!(run(1.0, &|_, _| false), (0x7ff0000000000000, 5, 0x3fe3333333333333, 0x3ff1cccccccccccd));
    }
}
