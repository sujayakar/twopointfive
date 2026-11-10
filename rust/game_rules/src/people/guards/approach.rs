use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// This frame's light readings for a plan: his share of the frame's readings, round-robin over the spots still worth asking about (all of them once those
/// are done), each kept DARK_WINDOW_SECS, the darkest live one mapped through the meter's own curve.
pub(super) fn search_probes(g: &mut crate::game::Game, gd: GuardId) {
    let n = g.gd[gd].search_plan.as_ref().unwrap().spots.len();
    if n == 0 { return; }
    let mut searchers = 0.0;
    for &o in g.guards.iter() {
        if g.gd[o].state == crate::game::GuardState::Search && g.gd[o].script.is_none() && g.gd[o].search_plan.is_some() { searchers += 1.0; }
    }
    let mut own = g.gd[gd].search_plan.take().unwrap();   // (off his row for the walk below, which calls with g: nothing it calls reads his plan)
    let p = &mut own;
    let budget = PROBE_BUDGET;
    let share = m::max(1.0, m::floor(budget / m::max(1.0, searchers)));
    let live = p.spots.iter().any(|s| !s.visited && !s.unreachable);
    let (mut k, mut tries) = (0.0, 0);
    while k < share && g.search_probe_n < budget && tries < n {
        tries += 1;
        let at = js::rem(p.rr, n as f64) as usize;
        p.rr += 1.0;
        let s = &mut p.spots[at];
        if live && (s.visited || s.unreachable) { continue; }   // (the budget goes on the spots still in play while any are)
        let e = crate::guards::light_estimate_at(g, [s.pos[0], PROBE_HEIGHT, s.pos[2]], None, None, None);
        s.samples.push(SearchSpotSamplesItem { t: g.time, e });
        g.search_probe_n += 1.0;
        k += 1.0;
    }
    for s in p.spots.iter_mut() {
        while !s.samples.is_empty() && g.time - s.samples[0].t > DARK_WINDOW_SECS { s.samples.remove(0); }
        if s.samples.len() > 200 { let over = s.samples.len() - 200; s.samples.drain(0..over); }
        if !s.samples.is_empty() {
            let mut low = m::INF;
            for x in s.samples.iter() { if x.e < low { low = x.e; } }
            s.e = Some(low);
            s.dark = 1.0 - crate::game::game_visibility_of(g, low);
        } else {
            s.e = None;
            s.dark = 0.5;
        }
    }
    g.gd[gd].search_plan = Some(own);
}

/// scoreSpots on the plan in hand.
fn score_into(g: &Game, gd: GuardId, p: &mut crate::guards::SearchPlan, walk: Option<&[f32]>) {
    let elapsed = g.time - p.t0;
    let others: Vec<GuardId> = g.guards.iter().copied().filter(|&o| o != gd && g.gd[o].state == crate::game::GuardState::Search && g.gd[o].script.is_none() && g.gd[o].search_plan.is_some() && g.team(o) == g.team(gd)).collect();   // (teams: spots are split over HIS side's net)
    let at = g.ch[g.gd[gd].ch].pos;
    let (slack, ramp) = (PLAUS_SLACK_SECS, PLAUS_RAMP_SECS);
    for s in p.spots.iter_mut() {
        if !s.visited {
            for &o in others.iter() {
                let v = g.gd[o].search_plan.as_ref().unwrap().spots.iter().find(|q| q.visited && !q.unreachable && v3::dist_xz(q.pos, s.pos) < 1.0);
                if let Some(v) = v { s.visited = true; s.by = v.by; break; }
            }
        }
        s.plaus = if s.need.is_finite() { v3::clamp(1.0 - (s.need - elapsed - slack) / ramp, 0.0, 1.0) } else { 0.0 };
        let dist = match walk { Some(w) => w[s.cell as usize] as f64, None => v3::dist_xz(at, s.pos) };
        s.score = if dist.is_finite() { m::max(s.dark, 0.05) * s.cover * s.plaus / (1.0 + 0.08 * dist) } else { 0.0 };   // (no way there for him from here: nothing)
    }
}

/// Score every spot for THIS man now (dark × cover × plausible, eased toward the shorter walk), folding in what the other searchers have looked into.
/// The plan is the man's row.
pub fn score_spots(g: &mut crate::game::Game, gd: GuardId, walk: Option<&[f32]>) {
    let mut own = g.gd[gd].search_plan.take().unwrap();   // (off his row for the walk: the scoring reads only the OTHER men's plans)
    score_into(g, gd, &mut own, walk);
    g.gd[gd].search_plan = Some(own);
}

/// A spot some other searcher is walking to or looking into right now (within a spot's spacing of it): his, not ours.
fn spot_taken(g: &crate::game::Game, gd: GuardId, s: &crate::guards::SearchSpot) -> bool {
    let team = g.team(gd);
    let spacing = SEARCH_SPACING_M;
    for &o in g.guards.iter() {
        let m_ = &g.gd[o];
        if o == gd || m_.state != crate::game::GuardState::Search || m_.script.is_some() || g.ch[m_.ch].team != team { continue; }
        let q_plan = match &m_.search_plan { Some(q) => q, None => continue };
        if q_plan.current < 0.0 || (q_plan.phase != SearchPlanPhase::Walk && q_plan.phase != SearchPlanPhase::Look) { continue; }
        if let Some(q) = q_plan.spots.get(q_plan.current as usize) { if v3::dist_xz(q.pos, s.pos) < spacing { return true; } }
    }
    false
}

/// On to the best spot still worth it (scored this frame), or the plan is done.
pub(super) fn pick_next_spot(g: &mut crate::game::Game, gd: GuardId) {
    let at = g.ch[g.gd[gd].ch].pos;
    let walk = flood_from(g, at, SEARCH_RADIUS_M * 2.0 + 4.0, Some(true)).metres;   // (his own walk to each, doors and all: round the partitions, not through them)
    let mut own = g.gd[gd].search_plan.take().unwrap();   // (off his row until the line is chosen: the scoring and the tests below read only the OTHER men's plans)
    let p = &mut own;
    score_into(g, gd, p, Some(&walk));
    let (mut best, mut bs): (i32, f64) = (-1, 0.004);   // (below that, lit open floor at the edge of plausibility, is not worth the walk)
    for (i, s) in p.spots.iter().enumerate() {
        if s.visited || s.unreachable || s.plaus < 0.02 || s.score <= bs || spot_taken(g, gd, s) { continue; }
        bs = s.score;
        best = i as i32;
    }
    p.current = best as f64;
    p.phase = if best >= 0 { SearchPlanPhase::Walk } else { SearchPlanPhase::Done };
    p.phase_t = 0.0;
    {
        let me = &mut g.gd[gd];
        me.path = Vec::new();
        me.path_serial = me.path_serial.wrapping_add(1);
        me.path_goal = None;
        me.path_goal_orbit = false;
        me.stall_ref = None;
    }
    // (one man says it for a search, not each of them a second apart)
    let says = best >= 0
        && !p.said
        && !(match &g.gd[gd].bubble { Some(b) => g.time - b.t < 2.5, None => false })
        && !g.guards.iter().any(|&o| o != gd && g.gd[o].state == crate::game::GuardState::Search && match &g.gd[o].search_plan { Some(q) => q.said && g.time - q.built_at < 30.0, None => false });
    if says { p.said = true; }
    let dark_spot = best >= 0 && p.spots[best as usize].dark > 0.6;
    let any_prop = p.spots.iter().any(|s| s.kind == SearchSpotKind::Prop);
    g.gd[gd].search_plan = Some(own);
    if !says { return; }
    let text: String = if dark_spot {
        String::from(if crate::teams::defender_words(g, gd) { "they must've gone somewhere dark — check the corners" } else { "he must've gone somewhere dark — check the corners" })
    } else if any_prop {
        if !crate::dialogue::is_office(g) {
            alloc::format!("check the corners — low, behind anything {} could get behind", if crate::teams::defender_words(g, gd) { "they" } else { "he" })
        } else {
            String::from("check the corners — under the desks, behind the chairs")
        }
    } else {
        String::from("check the corners")
    };
    crate::game::game_say(g, gd, &text, None, None);
}

/// The closed frame of a door: hinge, unit along the frame hinge → latch, unit normal (+ = CCW side), width.
pub fn frame_of(g: &crate::game::Game, d: DoorId) -> crate::doorway::RelF {
    let def = &g.dr[d].def;
    frame_from(def.closed_dir, def.hinge, def.width)
}

/// frameOf's sums.
fn frame_from(closed_dir: f64, hinge: [f64; 2], width: f64) -> crate::doorway::RelF {
    let (ux, uz) = (m::cos(closed_dir), m::sin(closed_dir));
    crate::doorway::RelF { hx: hinge[0], hz: hinge[1], ux, uz, nx: -uz, nz: ux, w: width }
}

/// Where segment a→b crosses d's frame line within the opening: t along the segment, the side a is on, the crossing point, or None.
pub fn crosses_door(g: &crate::game::Game, a: V3, b: V3, d: DoorId) -> Option<crate::guards::CrossesDoor> {
    let f = frame_of(g, d);
    crosses_frame(&f, a, b)
}

/// crossesDoor's sums.
fn crosses_frame(f: &crate::doorway::RelF, a: V3, b: V3) -> Option<crate::guards::CrossesDoor> {
    let (da, db) = ((a[0] - f.hx) * f.nx + (a[2] - f.hz) * f.nz, (b[0] - f.hx) * f.nx + (b[2] - f.hz) * f.nz);
    if da * db >= 0.0 || m::abs(da - db) < 1e-6 { return None; }
    let t = da / (da - db);
    let (ix, iz) = (a[0] + (b[0] - a[0]) * t, a[2] + (b[2] - a[2]) * t);
    let along = (ix - f.hx) * f.ux + (iz - f.hz) * f.uz;
    if along < -0.1 || along > f.w + 0.1 { return None; }
    Some(crate::guards::CrossesDoor { t, side: if da > 0.0 { 1.0 } else { -1.0 }, at: [ix, 0.0, iz] })
}

/// Doorways a hunting man goes through, pied or not: his stride since the last frame looked at cutting a frame line. Once a frame at most.
pub(super) fn note_door_crossings(g: &mut crate::game::Game, gd: GuardId) {
    let pos = g.ch[g.gd[gd].ch].pos;
    if g.gd[gd].pie_prev_t == g.time { return; }
    if let Some(prev) = g.gd[gd].pie_prev {
        if g.time - g.gd[gd].pie_prev_t < 0.5 && v3::dist_xz(prev, pos) > 1e-4 {
            for i in 0..g.doors.list.len() {
                let d = g.doors.list[i];
                if crosses_door(g, prev, pos, d).is_some() {
                    let now = g.time;
                    let me = &mut g.gd[gd];
                    me.pie_last = Some(d);
                    me.pie_last_t = now;
                    if let Some(pie) = me.pieing.as_mut() {
                        if pie.door == d && pie.phase != PieingPhase::Cross { pie.phase = PieingPhase::Cross; pie.t = 0.0; }
                    }
                }
            }
        }
    }
    let now = g.time;
    let me = &mut g.gd[gd];
    me.pie_prev = Some(v3::copy(pos));
    me.pie_prev_t = now;
}

/// The look as a bearing off the way through, about the near jamb's edge. deg: + toward the far side, − the near.
fn bearing(deg: f64, reach: f64, y: f64, far: f64, ex: f64, ez: f64, tx: f64, tz: f64, ux: f64, uz: f64) -> V3 {
    let a = deg * v3::DEG;
    let (ca, sa) = (m::cos(a), m::sin(a) * far);
    [ex + (tx * ca + ux * sa) * reach, y, ez + (tz * ca + uz * sa) * reach]
}

/// This frame's threshold discipline for a man followPath is about to move at `speed`: keeps Guard.pieing, notes the doorways he goes through. `eyes_on`: no
/// drill, he is chasing. Returns the speed to walk at and a look point (None = the caller's own).
pub fn pie_doorway(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64, eyes_on: bool) -> crate::guards::PieDoorway {
    let pos = g.ch[g.gd[gd].ch].pos;
    note_door_crossings(g, gd);
    if g.gd[gd].pinned || eyes_on {
        g.gd[gd].pieing = None;
        return crate::guards::PieDoorway { speed, look: None };
    }
    let mut held = g.gd[gd].pieing;
    if let Some(q) = held {
        let stale = g.time - q.t0 > PIE_CAP_SECS
            || (q.phase != PieingPhase::Cross && g.gd[gd].path_serial != q.path_ref && !path_crosses(g, gd, q.door));   // stale, or re-planned some other way
        if stale { held = None; g.gd[gd].pieing = None; }
    }
    if held.is_none() {   // a doorway on the path within PIE_DETECT_M?
        let (mut acc, mut prev) = (0.0, pos);
        let mut i = g.gd[gd].path_i;
        while i < g.gd[gd].path.len() && acc < PIE_DETECT_M {
            let wp = g.gd[gd].path[i];
            let l = v3::dist_xz(prev, wp);
            let mut hit: Option<(DoorId, crate::guards::CrossesDoor)> = None;
            for j in 0..g.doors.list.len() {
                let d = g.doors.list[j];
                if g.gd[gd].pie_last == Some(d) && g.time - g.gd[gd].pie_last_t < PIE_AGAIN_SECS { continue; }
                if let Some(x) = crosses_door(g, prev, wp, d) {
                    if match &hit { None => true, Some(h) => x.t < h.1.t } { hit = Some((d, x)); }
                }
            }
            if let Some((d, x)) = hit {
                if acc + l * x.t <= PIE_DETECT_M {
                    let fresh = crate::guards::Pieing { door: d, phase: PieingPhase::Approach, side: x.side, at: x.at, near: 0.0, t: 0.0, t0: g.time, path_ref: g.gd[gd].path_serial, bent: false };
                    held = Some(fresh);
                    g.gd[gd].pieing = Some(fresh);
                    break;
                }
            }
            acc += l;
            prev = wp;
            i += 1;
        }
    }
    let mut p = match held { Some(p) => p, None => return crate::guards::PieDoorway { speed, look: None } };
    let d = p.door;
    let f = frame_of(g, d);
    let sg = p.side;
    let (tx, tz) = (-sg * f.nx, -sg * f.nz);   // unit, the way through
    let perp = ((pos[0] - f.hx) * f.nx + (pos[2] - f.hz) * f.nz) * sg;   // metres off the frame line on his side (− = through)
    let before = if perp > 0.0 { m::max(perp, v3::dist_xz(pos, p.at)) } else { perp };   // metres still to walk to the sill, or, through, how far past the line
    if !p.bent {
        p.bent = true;
        g.gd[gd].pieing = Some(p);
        bend_approach(g, gd, d, sg);
        p.path_ref = g.gd[gd].path_serial;
        g.gd[gd].pieing = Some(p);
        refresh_crossing(g, gd);
        p = g.gd[gd].pieing.unwrap();   // (its crossing point, written through the row)
    }
    p.t += dt;
    if p.phase == PieingPhase::Approach && before < PIE_SLICE_M {
        p.phase = PieingPhase::Slice;
        p.t = 0.0;
        let a = (pos[0] - f.hx) * f.ux + (pos[2] - f.hz) * f.uz;
        p.near = if a > 0.6 * f.w { 1.0 } else { 0.0 };   // (coming at it along the wall from the latch side, that jamb is the near one)
    }
    if p.phase == PieingPhase::Slice && before < 0.15 { p.phase = PieingPhase::Cross; p.t = 0.0; }
    if (p.phase == PieingPhase::Cross && p.t > PIE_CORNER_SECS) || before < -1.5 || before > PIE_DETECT_M + 1.5 {
        let now = g.time;
        let me = &mut g.gd[gd];
        me.pie_last = Some(d);
        me.pie_last_t = now;
        me.pieing = None;
        return crate::guards::PieDoorway { speed, look: None };
    }
    g.gd[gd].pieing = Some(p);
    let mut out = speed;
    if before < PIE_SLOW_M + 0.3 && before > -0.1 {   // the brake: a stride and he is at a walk
        out = m::min(speed, g.tune.guard_walk);
        if g.gd[gd].speed > out { g.gd[gd].speed = v3::damp(g.gd[gd].speed, out, 14.0, dt); }
    }
    // the look pivots about the near jamb's edge: the slice round past the far jamb as he closes, then, stepping through, hard back to the corner behind the near jamb
    let near = p.near != 0.0;
    let (jn, far) = (if near { f.w } else { 0.0 }, if near { -1.0 } else { 1.0 });
    let (ex, ez) = (f.hx + f.ux * jn, f.hz + f.uz * jn);
    let look = match p.phase {
        PieingPhase::Slice => Some(bearing(v3::lerp(-25.0, 80.0, v3::smoothstep(0.0, 1.0, v3::clamp(p.t / PIE_SLICE_SECS, 0.0, 1.0))), 2.5, 0.95, far, ex, ez, tx, tz, f.ux, f.uz)),   // (held past the far jamb till he is on the sill)
        PieingPhase::Cross => Some(bearing(-70.0, 1.6, 0.8, far, ex, ez, tx, tz, f.ux, f.uz)),
        PieingPhase::Approach => None,
    };
    crate::guards::PieDoorway { speed: out, look }
}

/// Does the man's remaining path still run through this door.
fn path_crosses(g: &crate::game::Game, gd: GuardId, d: DoorId) -> bool {
    let mut prev = g.ch[g.gd[gd].ch].pos;
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() {
        let wp = g.gd[gd].path[i];
        if crosses_door(g, prev, wp, d).is_some() { return true; }
        prev = wp;
        i += 1;
    }
    false
}

/// The crossing point of the man's pie (his row) again, off the path as it now runs.
fn refresh_crossing(g: &mut crate::game::Game, gd: GuardId) {
    let door = g.gd[gd].pieing.as_ref().unwrap().door;
    let mut prev = g.ch[g.gd[gd].ch].pos;
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() {
        let wp = g.gd[gd].path[i];
        if let Some(x) = crosses_door(g, prev, wp, door) {
            g.gd[gd].pieing.as_mut().unwrap().at = x.at;
            return;
        }
        prev = wp;
        i += 1;
    }
}

/// Bend the approach already on his path toward the hinge side: the last mark short of the line slides to 0.42 of the width, and a mark 1.5 m out goes in ahead
/// of it unless he comes along the wall from the latch side. Only onto free cells he can walk between.
fn bend_approach(g: &mut crate::game::Game, gd: GuardId, d: DoorId, sg: f64) {
    let f = frame_of(g, d);
    let pos = g.ch[g.gd[gd].ch].pos;
    let bef = |p: V3| ((p[0] - f.hx) * f.nx + (p[2] - f.hz) * f.nz) * sg;
    let alg = |p: V3| (p[0] - f.hx) * f.ux + (p[2] - f.hz) * f.uz;
    let at = |a: f64, b: f64| -> V3 { [f.hx + f.ux * a + f.nx * sg * b, 0.0, f.hz + f.uz * a + f.nz * sg * b] };
    let path_i = g.gd[gd].path_i;
    let mut crossing: Option<(usize, V3)> = None;
    {
        let (mut prev, mut i) = (pos, path_i);
        while i < g.gd[gd].path.len() {
            let wp = g.gd[gd].path[i];
            if let Some(x) = crosses_door(g, prev, wp, d) { crossing = Some((i, x.at)); break; }
            prev = wp;
            i += 1;
        }
    }
    let (cross_i, x_at) = match crossing { Some(c) => c, None => return };
    let mut near_i: Option<usize> = None;   // the near mark: the waypoint just short of the line, if it sits in front of the opening
    if cross_i > path_i {
        let w = g.gd[gd].path[cross_i - 1];
        let (b, a) = (bef(w), alg(w));
        if b > 0.3 && b < 1.4 && a > 0.15 * f.w && a < 0.9 * f.w { near_i = Some(cross_i - 1); }
    }
    if let Some(ni) = near_i {
        let w = g.gd[gd].path[ni];
        let q = at(0.42 * f.w, v3::clamp(bef(w), 0.55, 0.8));
        let from = if ni > path_i { g.gd[gd].path[ni - 1] } else { pos };
        if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) && crate::scene_collision::nav_grid_walkable(g, from, q) { g.gd[gd].path[ni] = q; }
    }
    if v3::dist_xz(pos, x_at) > 2.1 && alg(pos) < 0.6 * f.w + 0.5 {   // the far mark (not for a man coming along the wall from the latch side)
        let q = at(0.25 * f.w, 1.5);
        let ins = match near_i { Some(ni) => ni, None => cross_i };
        let from = if ins > path_i { g.gd[gd].path[ins - 1] } else { pos };
        let to = g.gd[gd].path[ins];
        if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
            && v3::dist_xz(from, q) > 0.5
            && crate::scene_collision::nav_grid_walkable(g, from, q)
            && (bef(to) < 0.0 || crate::scene_collision::nav_grid_walkable(g, q, to))
        {
            g.gd[gd].path.insert(ins, q);   // (in place: the path keeps its identity, no path_serial)
        }
    }
}

/// The alert chase on a fix with no eyes on him: path to the fix at `pace`, pie the doorways on the way, and the three no-progress exits into the search.
/// Returns his move speed, aim point and that a pie may stand.
pub fn chase_fix(g: &mut crate::game::Game, gd: GuardId, dt: f64, pace: f64) -> crate::guards::ChaseFix {
    let c = g.gd[gd].ch;
    let fix = g.gd[gd].last_known.unwrap();
    go_to(g, gd, fix, None);
    let eyes_on = g.time - g.gd[gd].last_seen_t < PIE_EYES_SECS;
    let pie = pie_doorway(g, gd, dt, pace, eyes_on);   // a doorway on the way to the fix: down to a walk, slice it, check the corner as he goes through
    let mut arrived = follow_path(g, gd, dt, pie.speed);
    let move_ = g.gd[gd].speed;
    let aim = if arrived { None } else { Some(match pie.look { Some(l) => l, None => v3::add(fix, [0.0, 1.1, 0.0]) }) };
    // no way of getting nearer: after a few seconds of no progress he searches from where he is. Progress is measured against THIS fix; four seconds of no ground
    // covered ends it too; and no chase runs past 25 s without a sight of him or a new fix.
    let same_fix = match g.gd[gd].alert_ref { Some(r) => !(v3::dist_xz(r, fix) > 0.05), None => false };
    if !same_fix {
        let me = &mut g.gd[gd];
        me.alert_ref = Some(v3::copy(fix));
        me.alert_best = 1e9;
        me.alert_stall_t = 0.0;
        me.chase_t = 0.0;
    }
    let d_fix = v3::dist_xz(fix, g.ch[c].pos);
    if d_fix < g.gd[gd].alert_best - 0.5 {
        g.gd[gd].alert_best = d_fix;
        g.gd[gd].alert_stall_t = 0.0;
    } else {
        g.gd[gd].alert_stall_t += dt;
        if g.gd[gd].alert_stall_t > 5.0 { arrived = true; }
    }
    if !arrived && not_getting_anywhere(g, gd, dt, 4.0) { arrived = true; }
    if !arrived {
        g.gd[gd].chase_t += dt;
        if g.gd[gd].chase_t > 25.0 { arrived = true; }
    }
    if arrived {
        let bark = if crate::teams::defender_words(g, gd) { "where did they go?" } else { "where did he go?" };
        drop_to_search(g, gd, Some(bark));
    }
    crate::guards::ChaseFix { move_, aim, pied: true, crouch: 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The expected bits are the TypeScript's own: guards.ts frameOf / crossesDoor under bun with EXACT.on.
    #[test]
    fn frame_is_the_typescripts() {
        let f = frame_from(0.7, [3.25, -1.5], 1.1);
        assert_eq!(f.ux.to_bits(), 0x3fe87996529f9d92);
        assert_eq!(f.uz.to_bits(), 0x3fe49d6e694619b8);
        assert_eq!(f.nx.to_bits(), 0xbfe49d6e694619b8);
        assert_eq!(f.nz.to_bits(), 0x3fe87996529f9d92);
    }

    #[test]
    fn crossing_is_the_typescripts() {
        let f = frame_from(0.7, [3.25, -1.5], 1.1);
        let (a, b) = ([3.03, 0.0, -0.38], [4.31, 0.0, -1.91]);
        let x = crosses_frame(&f, a, b).unwrap();
        assert_eq!((x.t.to_bits(), x.side, x.at[0].to_bits(), x.at[2].to_bits()), (0x3fe003e429914a22, 1.0, 0x400d5d67bb61b152, 0xbff254e5958f2582));
        let x = crosses_frame(&f, b, a).unwrap();
        assert_eq!((x.t.to_bits(), x.side, x.at[0].to_bits(), x.at[2].to_bits()), (0x3fdff837acdd6bbb, -1.0, 0x400d5d67bb61b152, 0xbff254e5958f2584));
        assert!(crosses_frame(&f, [5.33, 0.0, 1.55], [6.61, 0.0, 0.02]).is_none());   // past the opening along the line
        assert!(crosses_frame(&f, a, a).is_none());   // no stride
    }
}
