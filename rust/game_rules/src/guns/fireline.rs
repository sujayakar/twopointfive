pub use ::game_types::fireline::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// Once a frame before the men are posed: is the player firing, by the rounds counted off the gun in his hands.
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
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].sprinting {
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

/// The lane of the player's fire this frame, or None: he is not firing.
fn lane_of(g: &crate::game::Game) -> Option<crate::fireline::Lane> {
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
    let def = g.fa[gun].def;
    let energy = crate::glasspen::shot_energy(g, c, Some(def));
    let thin = crate::glasspen::round_energy(g, def);
    Some(Lane { m: muzzle, dx, dz, back, k, half, energy, thin })
}

/// Is this point of floor inside the lane by its shape alone.
fn in_shape(g: &crate::game::Game, l: crate::fireline::Lane, x: f64, z: f64, more: Option<f64>) -> bool {
    let more = more.unwrap_or(0.0);
    let rx = x - l.m[0];
    let rz = z - l.m[2];
    let a = rx * l.dx + rz * l.dz;
    a > l.back - more && a < g.k.fireline.reach_m && m::abs(rx * l.dz - rz * l.dx) <= l.half + more + m::max(0.0, a) * l.k
}

/// Would a round of this lane reach the point.
fn reaches(g: &Game, l: Lane, p: V3) -> bool {
    !crate::scene_collision::shot_blocked(g, l.m, p, Some(0.02), l.energy) || (l.thin >= 0.0 && crate::wallpen::thin_line(g, l.m, p, l.thin, None, None).is_some())
}

/// Would a round reach a man standing here: his chest, or either edge of his column.
fn in_lane(g: &crate::game::Game, l: crate::fireline::Lane, x: f64, z: f64, more: Option<f64>) -> bool {
    if !in_shape(g, l, x, z, more) {
        return false;
    }
    reaches(g, l, [x, 1.2, z]) || reaches(g, l, [x + l.dz * 0.25, 1.2, z - l.dx * 0.25]) || reaches(g, l, [x - l.dz * 0.25, 1.2, z + l.dx * 0.25])
}

/// How far along the polyline `from` → `pts` (at most `far` of it) the walk first stands where `hit` says; infinity = nowhere.
fn along(from: V3, pts: &[V3], far: f64, hit: &mut dyn FnMut(f64, f64) -> bool) -> f64 {
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

/// Where the walk first comes into the lane.
fn cross_at(g: &crate::game::Game, l: crate::fireline::Lane, from: V3, pts: &[V3], far: f64, more: Option<f64>) -> f64 {
    along(from, pts, far, &mut |x, z| in_lane(g, l, x, z, more))
}

/// Where the walk first stands in a doorway.
fn door_at(g: &crate::game::Game, from: V3, pts: &[V3], far: f64) -> f64 {
    along(from, pts, far, &mut |x, z| in_doorway(g, x, z))
}

/// Is this point of floor in a door's opening.
fn in_doorway(g: &crate::game::Game, x: f64, z: f64) -> bool {
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

/// The corners of the walk he is on to `goal`: the runner's own path while it is for that goal, else the straight leg if short, else None.
fn walk_of(g: &crate::game::Game, gd: GuardId, goal: V3) -> Option<Vec<V3>> {
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

/// Would waiting leave him standing in an enemy's line, as far as he knows.
fn in_enemy_line(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let p = g.ch[g.gd[gd].ch].pos;
    crate::picture::exposed_at(g, gd, p, None) > 0.0
}
/// A way to `goal` that keeps out of the lane, no more than `wayMoreM` longer than the way across, or None.
fn way_round(g: &crate::game::Game, gd: GuardId, l: crate::fireline::Lane, goal: V3) -> Option<Vec<V3>> {
    let cs = g.col.nav.cell;
    let p = g.ch[g.gd[gd].ch].pos;
    let berth_m = g.k.fireline.berth_m;
    let mut penalty = |g: &Game, ix: f64, iz: f64| -> f64 {
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
    if crate::places::walk_len(p, &way) > crate::places::walk_len(p, &across) + g.k.fireline.way_more_m {
        return None;
    }
    Some(way)
}

/// He stands: the wait's seconds counted as they pass.
fn hold(g: &mut crate::game::Game, gd: GuardId) -> crate::fireline::FireLineFeet {
    let t = g.time;
    g.m.fireline.states.get_mut(gd).unwrap().t = t;
    FireLineFeet { goal: None, speed: 0.0 }
}

/// Is he on a door order past the stack.
fn past_the_stack(g: &Game, seat: &crate::seats::SeatRef, gd: GuardId) -> bool {
    seat.door_tools.ops.iter().any(|&o| {
        let op = &g.tab.doortools_door_op[o];
        op.phase != crate::doortools::Phase::Stack && op.men.contains(&gd)
    })
}

/// Is he on a wall charge.
fn on_wall_charge(g: &Game, seat: &crate::seats::SeatRef, gd: GuardId) -> bool {
    seat.wall_charge.ops.iter().any(|&o| g.tab.wallcharge_wall_op[o].men.contains(&gd))
}

/// His feet while the player fires down a lane his walk would cross: speed 0 = wait where he stands, `goal` = the next corner of a way round, None = the order's own legs.
pub fn fire_line_feet(g: &mut crate::game::Game, gd: GuardId, goal: Option<V3>, speed: f64, mine: bool) -> Option<crate::fireline::FireLineFeet> {
    let t = g.time;
    if g.m.fireline.states.get(gd).is_some_and(|s| t < s.t || t - s.t > 0.25) {
        g.m.fireline.states.delete(gd);
    }
    let his = !mine && crate::friendlyfire::ff_side(g, Some(gd)) && crate::friendlyfire::ff_body(g, gd) && g.gd[gd].held.is_none();
    let goal = goal.filter(|_| his);
    let lane = if goal.is_some() { lane_of(g) } else { None };
    if goal.is_some() && lane.is_none() {
        lapses(g, gd);
        return None;
    }
    let feet = goal.zip(lane).and_then(|(goal, l)| by_lane(g, gd, l, goal, speed));
    if feet.is_none() {
        g.m.fireline.states.delete(gd);
    }
    feet
}

/// The fire has stopped, or lapsed between two slow rounds: he walks on, and for `lapseS` a lane that comes back is the same wait.
fn lapses(g: &mut Game, gd: GuardId) {
    let (t, lapse) = (g.time, g.k.fireline.lapse_s);
    let keeps = g.m.fireline.states.get_mut(gd).filter(|s| s.way.is_none()).is_some_and(|s| {
        if s.go_t < 0.0 {
            s.go_t = t;
        }
        s.t = t;
        t - s.go_t <= lapse
    });
    if !keeps {
        g.m.fireline.states.delete(gd);
    }
}

/// His feet by the lane: None lets go of him, and his wait is forgotten.
fn by_lane(g: &mut Game, gd: GuardId, l: Lane, goal: V3, speed: f64) -> Option<FireLineFeet> {
    let p = g.ch[g.gd[gd].ch].pos;
    if at_work(g, gd, p) {
        return None;
    }
    if way_elsewhere(g, gd, goal) {
        g.m.fireline.states.delete(gd);
    }
    let on_way = g.m.fireline.states.get(gd).is_some_and(|s| s.way.is_some());
    if in_lane(g, l, p[0], p[2], Some(if on_way { -g.k.fireline.clear_m } else { 0.0 })) {
        return if stands_in_it(g, gd) { Some(hold(g, gd)) } else { None };
    }
    if on_way {
        if let Some(next) = next_corner(g, gd, l, p) {
            return Some(FireLineFeet { goal: Some(next), speed });
        }
        g.m.fireline.states.delete(gd);
    }
    if !comes_in(g, gd, l, p, goal) || in_enemy_line(g, gd) {
        return None;
    }
    waits(g, gd);
    if let Some(next) = looks_round(g, gd, l, goal) {
        return Some(FireLineFeet { goal: Some(next), speed });
    }
    Some(hold(g, gd))
}

/// What he is on is not put off for it: a fight, a door, a charge, a doorway he stands in.
fn at_work(g: &Game, gd: GuardId, p: V3) -> bool {
    crate::roomfight::room_fight_busy(g, gd, 1.0) || past_the_stack(g, &crate::seats::here(g), gd) || on_wall_charge(g, &crate::seats::here(g), gd) || in_doorway(g, p[0], p[2])
}

/// His way round was to a place the order no longer sends him: asked afresh, of the new one.
fn way_elsewhere(g: &Game, gd: GuardId, goal: V3) -> bool {
    let last = g.m.fireline.states.get(gd).and_then(|s| s.way.as_ref()).and_then(|way| way.last());
    last.is_some_and(|last| m::hyp2(last[0] - goal[0], last[2] - goal[2]) > 0.75)
}

/// Already in the lane: not moved by this rule, but for a man WAITING whom the gun swings onto, who stands as he stood to the wait's ceiling.
fn stands_in_it(g: &mut Game, gd: GuardId) -> bool {
    let (t, max_s) = (g.time, g.k.fireline.max_s);
    let waiting = g.m.fireline.states.get(gd).is_some_and(|s| !(s.way.is_some() || s.go_t >= 0.0 || t - s.since >= max_s));
    waiting && !in_enemy_line(g, gd)
}

/// On a way round: the next corner, while what is left of it keeps out of the lane.
fn next_corner(g: &mut Game, gd: GuardId, l: Lane, p: V3) -> Option<V3> {
    let (t, short_m, clear_m) = (g.time, g.k.fireline.short_m, g.k.fireline.clear_m);
    let s = g.m.fireline.states.get_mut(gd).unwrap();
    let way = s.way.as_mut().unwrap();   // (he is on one)
    while way.len() > 1 && m::hyp2(way[0][0] - p[0], way[0][2] - p[2]) < 0.6 {
        way.remove(0);
    }
    s.t = t;
    let left = way.clone();
    if left.len() > 1 && cross_at(g, l, p, &left, short_m, Some(-clear_m)) == f64::INFINITY { Some(left[0]) } else { None }
}

/// Does his way come into the lane: inside `shortM` of walking (a doorway between: that far beyond the frame), or on the slant with his next step inside `gapM` of its side.
fn comes_in(g: &Game, gd: GuardId, l: Lane, p: V3, goal: V3) -> bool {
    let waiting = g.m.fireline.states.has(gd);
    let Some(walk) = walk_of(g, gd, goal).or(if waiting { Some(vec![goal]) } else { None }) else { return false };
    let k = g.k.fireline;
    let keep = if waiting { k.keep_m } else { 0.0 };
    let door = door_at(g, p, &walk, 1.0);
    let far = k.short_m + keep + (if door < f64::INFINITY { door + 0.9 } else { 0.0 });
    cross_at(g, l, p, &walk, far, None) < f64::INFINITY || (cross_at(g, l, p, &walk, 0.35 + keep, Some(k.gap_m)) < f64::INFINITY && cross_at(g, l, p, &walk, k.look_m, None) < f64::INFINITY)
}

/// He waits, and after a moment of it says so.
fn waits(g: &mut Game, gd: GuardId) {
    let (t, cue_s) = (g.time, g.k.fireline.cue_s);
    let s = g.m.fireline.states.get_or_insert_with(gd, || St { since: t, t, way: None, look_t: -1e9, go_t: -1.0, said: false });
    s.go_t = -1.0;
    if s.said || t - s.since < cue_s {
        return;
    }
    s.said = true;
    crate::game::game_say(g, gd, "your line — holding.", None, None);
    g.stats.fireline.cues += 1;
}

/// The ceiling of a wait: then a look for a way round, and again every `againS` (the men of a squad that stopped together do not all look on one frame).
fn looks_round(g: &mut Game, gd: GuardId, l: Lane, goal: V3) -> Option<V3> {
    let (t, max_s, again_s) = (g.time, g.k.fireline.max_s, g.k.fireline.again_s);
    let place: i32 = g.guards.iter().position(|&x| x == gd).map_or(-1, |i| i as i32);
    let s = g.m.fireline.states.get_mut(gd).unwrap();   // (he waits)
    if !(t - s.since >= max_s + (place & 3) as f64 * 0.3 && t - s.look_t >= again_s) {
        return None;
    }
    s.look_t = t;
    let way = way_round(g, gd, l, goal)?;
    let next = way[0];
    let s = g.m.fireline.states.get_mut(gd).unwrap();
    s.way = Some(way);
    s.t = t;
    Some(next)
}

/// Is this rule holding him where he stands: nobody calls him stuck.
pub fn fire_line_live(g: &crate::game::Game, gd: GuardId) -> bool {
    match g.m.fireline.states.get(gd) {
        Some(st) => st.way.is_none() && st.go_t < 0.0 && g.time >= st.t && g.time - st.t <= 0.25,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The walk's answer and every point it asked about.
    fn walk(pts: &[V3], far: f64, hit: &dyn Fn(f64, f64) -> bool) -> (f64, Vec<(f64, f64)>) {
        let mut asked = Vec::new();
        let s = along([0.0; 3], pts, far, &mut |x, z| {
            asked.push((x, z));
            hit(x, z)
        });
        (s, asked)
    }
    const CORNER: [V3; 2] = [[10.0, 0.0, 0.0], [10.0, 0.0, 10.0]];

    /// A doorway or a lane is never stepped over: the path is asked about every 20 cm at most, right to its end.
    #[test]
    fn the_walk_misses_nothing_on_the_way() {
        let (s, asked) = walk(&CORNER, f64::INFINITY, &|_, _| false);
        assert_eq!(s, f64::INFINITY);
        let mut was = (0.0, 0.0);
        for &(x, z) in &asked {
            assert!(m::hyp2(x - was.0, z - was.1) <= 0.2 + 1e-9, "a step from {:?} to {:?}", was, (x, z));
            was = (x, z);
        }
        assert_eq!(was, (10.0, 10.0));
    }

    /// The answer is how far along the first point that answers is, the legs behind it counted, and nothing past it is asked.
    #[test]
    fn the_walk_stops_at_the_first_point_that_answers() {
        let (s, asked) = walk(&CORNER, f64::INFINITY, &|x, _| x >= 3.0);
        assert!(s >= 3.0 && s <= 3.2 + 1e-9, "{s}");
        assert_eq!(asked.last().unwrap().0, s);
        let (s, _) = walk(&CORNER, f64::INFINITY, &|_, z| z >= 2.0);
        assert!(s >= 12.0 && s <= 12.2 + 1e-9, "{s}");
    }

    /// Past `far` the walk has found nothing, even where something would answer.
    #[test]
    fn the_walk_goes_no_further_than_it_was_told() {
        let (s, asked) = walk(&CORNER, 1.0, &|x, _| x >= 5.0);
        assert_eq!(s, f64::INFINITY);
        assert!(asked.iter().all(|p| p.0 <= 1.0), "{:?}", asked.last());
    }

    /// A point given twice, or one straight overhead, is no length of floor.
    #[test]
    fn a_leg_with_no_floor_in_it_is_no_step() {
        let plain = walk(&CORNER, f64::INFINITY, &|_, _| false).1;
        let again = walk(&[[0.0, 5.0, 0.0], CORNER[0], CORNER[0], CORNER[1]], f64::INFINITY, &|_, _| false).1;
        assert_eq!(again, plain);
    }
}
