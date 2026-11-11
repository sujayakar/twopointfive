use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// how far through an opening with no door in it the floor beyond it is asked of: the first that is a room's (a bed may stand at the hole)
const BEYOND_M: [f64; 4] = [0.9, 1.5, 2.1, 2.7];
/// a man's feet are in an opening's frame within this of its plane, and this past its half width
const FRAME_DEEP: f64 = 0.45;
const FRAME_WIDE: f64 = 0.15;
/// this near his mark a man is on it
const ON_MARK_M: f64 = 0.6;
/// a gun within this of an opening, with a line to it, is on it
const ON_IT: f64 = 25.0 * v3::DEG;

/// Does his script hold this point of ours. By value: a point the cover made is p + sin(yaw) * r, no other writer holds its bits, and every move of it is written through to the script here (poseFor, turnOut).
fn script_aim_is(g: &Game, gd: GuardId, aim: V3) -> bool {
    g.gd[gd].script.as_ref().map_or(false, |s| s.aim_at == Some(aim))
}

/// The squad's book that moves this man: his seat's.
fn trail_back(g: &mut Game, gd: GuardId, extra: f64) -> Option<V3> {
    let seat = crate::seats::view_of(g, gd);
    crate::seats::with_seat(g, seat, &mut |g: &mut Game| crate::orders::squad_orders_trail_back(g, gd, extra))
}

fn sam_stopped(g: &mut Game, gd: GuardId) -> bool {
    let seat = crate::seats::view_of(g, gd);
    crate::seats::with_seat(g, seat, &mut |g: &mut Game| crate::orders::squad_orders_sam_stopped(&crate::seats::here(g)))
}

fn rear_pin(g: &mut Game, gd: GuardId) -> Option<crate::rearwatch::RearPin> {
    let seat = crate::seats::view_of(g, gd);
    crate::seats::with_seat(g, seat, &mut |g: &mut Game| crate::rearwatch::rear_pin_of(g, &crate::seats::here(g), gd))
}

///  The constructor: the field initialisers, the file's one rear opening, the page's knobs.
pub fn cover_new(g: &mut Game) -> Cover {
    let mut c = Cover::blank();
    c.rear = Some(g.tab.cover_opening.push(Opening { id: String::from("rear"), kind: OpeningKind::Rear, at: [0.0, CHEST, 0.0], n: [0.0, 1.0], half: 1.0, door: None, pass: None, far: None }));
    c
}

/// What he was dealt.
pub(in crate::teammates) fn cover_of(g: &Game, gd: GuardId) -> Option<CoverJob> { g.cover.jobs.get(gd).copied() }

/// The level's openings, built if they are not yet.
pub fn cover_ready(g: &mut Game) -> Vec<CoverOpeningId> {
    if !g.cover.nav { cover_build(g); }
    g.cover.openings.clone()
}

/// The level's openings: its doors, and the gaps of its nav grid that are not a door's own doorway.
fn cover_build(g: &mut Game) {
    g.cover.nav = true;
    let list = g.doors.list.clone();
    let mut doors: Vec<CoverOpeningId> = Vec::new();
    for d in list {
        let th = g.dr[d].def.closed_dir;
        let fc = g.dr[d].frame_centre;
        let id = alloc::format!("door:{}", g.dr[d].def.name);
        let half = g.dr[d].def.width * 0.5;
        doors.push(g.tab.cover_opening.push(Opening { id, kind: OpeningKind::Door, at: [fc[0], CHEST, fc[2]], n: [-m::sin(th), m::cos(th)], half, door: Some(d), pass: None, far: None }));
    }
    let mut gaps: Vec<CoverOpeningId> = Vec::new();
    let nav = GapsOfNav { w: g.col.nav.w, h: g.col.nav.h, cell: g.col.nav.cell, blocked: g.col.nav.blocked.clone() };
    let k = g.k.cover;
    // (the statics' sight alone decides what a gap is)
    let walls = crate::ask::Over::statics();
    let found = {
        let mut solid = |a: V3, b: V3| crate::scene_collision::segment_blocked_over(g, &walls, a, b, None, None);
        // (an opening gets its row of g.tab.cover_opening only once it is kept, below: nothing holds the others)
        let mut found = gaps_of(&nav, true, &k, Some(&mut solid));
        found.extend(gaps_of(&nav, false, &k, Some(&mut solid)));
        found
    };
    for o in found {
        let oat = o.at;
        if doors.iter().any(|&d| v3::dist_xz(g.tab.cover_opening[d].at, oat) < 1.0) || gaps.iter().any(|&x| v3::dist_xz(g.tab.cover_opening[x].at, oat) < 0.6) { continue; }
        let o = g.tab.cover_opening.push(o);
        gaps.push(o);
    }
    doors.extend(gaps);
    g.cover.openings = doors;
}

/// His order leaves his eyes spare now.
fn cover_spare(g: &mut Game, gd: GuardId, a: TaskId) -> bool {
    if !g.tab.task[a].spare.unwrap_or(false) || g.tab.task[a].post.is_none() { return false; }
    if g.tab.task[a].from.is_none() { return false; }
    if rear_pin(g, gd).map_or(false, |p| p.out.unwrap_or(false)) { return true; }
    let door = g.tab.task[a].door;
    for o in crate::seats::team_men(g, &crate::seats::here(g)) {
        if o == gd { continue; }
        if let Some(b) = crate::duty::order(g, o) {
            let row = &g.tab.task[b];
            if row.post.is_some() && !row.spare.unwrap_or(false) && row.door == door { return true; }
        }
    }
    false
}

/// His order has given him nothing to watch and no door to be at: his eyes are his own on the walk to his mark, and on it where the order says so.
pub fn cover_eyes_free(g: &Game, gd: GuardId, a: TaskId) -> bool {
    let row = &g.tab.task[a];
    if row.watch.is_some() || row.door.is_some() { return false; }
    let walking = row.post.map_or(false, |p| v3::dist_xz(g.pos_of(gd), p) > ON_MARK_M);
    walking || crate::orders::rows::is(g, a, crate::orders::Is::LooksAbout)
}

/// Is this man free to be dealt an opening: the order he is under if so.
fn cover_free(g: &mut Game, gd: GuardId) -> Option<TaskId> {
    if crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].script.is_none() || crate::control::man::blind(g, gd) || g.gd[gd].held.is_some() { return None; }
    let a = crate::duty::order(g, gd)?;
    if !(cover_eyes_free(g, gd, a) || cover_spare(g, gd, a)) { return None; }
    if crate::roomfight::room_fight_engaged(g, gd) || crate::cleared::walks_in(g, gd) { return None; }
    if let Some(op) = crate::roomfight::room_fight_op_of(g, gd) {
        for (h, mt) in g.tab.roomfight_op[op].meters.iter() {
            if !crate::condition::down(g, g.gd[h].ch) && g.time - mt.seen_t < g.k.cover.contact_secs { return None; }
        }
    }
    Some(a)
}

/// What a rear man on a door-less mark watches.
fn cover_read_rear(g: &mut Game, gd: GuardId, a: TaskId, list: &[(GuardId, TaskId)], bodies: &[V3]) -> Option<crate::roomfit::Watchable> {
    let k = g.k.cover;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let now = g.time;
    let cached = g.cover.reads.get(a).cloned();
    if let Some(r) = &cached {
        if now >= r.t && now - r.t < k.rear_read_secs && v3::dist_xz(p, r.at) <= 0.5 { return r.pick.clone(); }
    }
    let row = &g.tab.task[a];
    let front = match row.watch { Some(w) => v3::yaw_to(w, p), None => row.face.unwrap_or(g.ch[c].body_yaw) };
    let deg = k.rear_read_deg * m::PI / 180.0;
    let same = |o: &crate::roomfit::Watchable, t: &crate::roomfit::Watchable| o.key == t.key || m::abs(v3::wrap_angle(v3::yaw_to(p, o.at) - t.yaw)) < (if o.way && t.way { 0.15 } else { deg });
    let mut theirs: Vec<crate::roomfit::Watchable> = Vec::new();
    for &(mt, b) in list {
        if mt == gd || !g.tab.task[b].spare.unwrap_or(false) { continue; }
        if let Some(r) = g.cover.reads.get(b) {
            if let Some(pk) = &r.pick {
                if now >= r.t && now - r.t <= k.rear_read_secs + 0.5 { theirs.push(pk.clone()); }
            }
        }
    }
    let all = crate::roomfit::watchables_at(g, p, front, k.rear_read_m, bodies);
    let free: Vec<crate::roomfit::Watchable> = all.iter().filter(|t| !theirs.iter().any(|o| same(o, t))).cloned().collect();
    let w = if !free.is_empty() { free } else { all };
    let was: Option<usize> = match cached.as_ref().and_then(|r| r.pick.as_ref()) {
        Some(pk) => w.iter().position(|t| t.key == pk.key || (!t.way && !pk.way && m::abs(v3::wrap_angle(t.yaw - pk.yaw)) < 0.3)),
        None => None,
    };
    let pick: Option<usize> = match was {
        Some(i) if w[0].w <= w[i].w * k.rear_read_sticky => Some(i),
        _ => if w.is_empty() { None } else { Some(0) },
    };
    let picked = pick.map(|i| w[i].clone());
    g.cover.reads.set(a, CoverReadsValue { t: now, at: [p[0], 0.0, p[2]], pick: picked.clone() });
    picked
}

/// A stack's rear man still on the walk to his mark, further out than the last steps.
pub fn cover_walk_only(g: &Game, gd: GuardId, a: TaskId) -> bool {
    let row = &g.tab.task[a];
    if !row.spare.unwrap_or(false) || row.from.is_none() { return false; }
    match row.post {
        Some(post) => v3::dist_xz(g.pos_of(gd), post) > g.k.cover.rear_walk_m,
        None => false,
    }
}

/// A door standing open by his way, with floor beyond it the team has not been on.
fn cover_walk_door(g: &mut Game, o: CoverOpeningId, a: TaskId) -> bool {
    let d = match g.tab.cover_opening[o].door { Some(d) => d, None => return false };
    if Some(d) == g.tab.task[a].door { return false; }
    let open = g.dr[d].leaf.off || (!g.dr[d].leaf.latched && m::abs(g.dr[d].leaf.angle) >= g.k.doormanners.lane_ajar);
    if !open { return false; }
    let ra = crate::cleared::cleared_rooms_room_by(g, d, 1.0);
    let rb = crate::cleared::cleared_rooms_room_by(g, d, -1.0);
    !(js::truthy(ra) && js::truthy(rb) && crate::cleared::cleared_rooms_is_known(g, ra) && crate::cleared::cleared_rooms_is_known(g, rb))
}

/// His eye over a spot.
fn cover_eye_of(p: V3) -> V3 { [p[0], EYE, p[2]] }

/// The point of this opening a man stood at `p` can watch.
pub(in crate::teammates) fn cover_sight_from(g: &Game, p: V3, o: CoverOpeningId) -> Option<V3> {
    let eye = cover_eye_of(p);
    cover_sight(g, eye, o)
}

/// A line from this eye to the opening itself.
fn cover_line(g: &Game, eye: V3, o: CoverOpeningId) -> bool {
    cover_sight(g, eye, o) == Some(g.tab.cover_opening[o].at)
}

/// The point of this opening a man at `eye` can watch: the opening, or just past an open leaf's free edge.
fn cover_sight(g: &Game, eye: V3, o: CoverOpeningId) -> Option<V3> {
    let k = g.k.cover;
    let at = g.tab.cover_opening[o].at;
    let door = g.tab.cover_opening[o].door;
    let d = v3::dist_xz(eye, at);
    if d > k.reach { return None; }
    if d < 0.3 { return Some(at); }
    let s = if door.is_some() { m::max(0.0, 1.0 - 0.7 / d) } else { 1.0 };
    if crate::scene_collision::segment_blocked(g, eye, [eye[0] + (at[0] - eye[0]) * s, CHEST, eye[2] + (at[2] - eye[2]) * s], None, None, None) { return None; }
    let dd = match door { Some(dd) => dd, None => return Some(at) };
    if !(!g.dr[dd].leaf.off && m::abs(g.dr[dd].leaf.angle) >= 0.5 && crate::scene_collision::segment_blocked(g, eye, [at[0], CHEST, at[2]], None, None, None)) { return Some(at); }
    let u = crate::doors::door_dir(g, dd);
    let r = g.dr[dd].def.width + 0.35;
    let hinge = g.dr[dd].def.hinge;
    let t: V3 = [hinge[0] + u[0] * r, CHEST, hinge[1] + u[1] * r];
    if v3::dist_xz(eye, t) > k.in_m && !crate::scene_collision::segment_blocked(g, eye, t, None, None, None) { Some(t) } else { None }
}

/// Once a frame, before the orders pose the men.
pub fn cover_tick(g: &mut Game) {
    let k = g.k.cover;
    if !crate::seats::team_active(g) {
        if !g.cover.jobs.is_empty() || !g.cover.kept.is_empty() || !g.cover.outs.is_empty() { cover_drop_all(g); }
        g.cover.steps.clear();
        g.cover.live = Vec::new();
        return;
    }
    for gd in g.cover.jobs.keys() {
        if cover_free(g, gd).is_none() { cover_drop(g, gd, Some(true)); }
    }
    let kept: Vec<(GuardId, f64)> = g.cover.kept.iter().map(|(gd, v)| (gd, v.t)).collect();
    for (gd, t) in kept {
        if cover_free(g, gd).is_none() { cover_unkeep(g, gd, Some(true)); }
        else if m::abs(g.time - t) >= g.k.rear.secs { cover_unkeep(g, gd, None); }
    }
    for gd in g.cover.outs.keys() {
        if cover_free(g, gd).is_none() { cover_unturn(g, gd); }
    }
    for gd in crate::seats::team_men(g, &crate::seats::here(g)) {
        let v = g.ch[g.gd[gd].ch].vel;
        if m::hyp2(v[0], v[2]) > 0.3 { g.cover.heading.set(gd, m::atan2(v[0], v[2])); }
    }
    if g.time < g.cover.deal_at { return; }
    g.cover.deal_at = g.time + k.deal_secs;
    if !g.cover.nav { cover_build(g); }
    cover_deal(g);
}

/// Does it give onto floor nobody of the team has been on, with nothing shut in it.
fn onto_unknown(g: &mut Game, o: CoverOpeningId) -> bool {
    let row = &g.tab.cover_opening[o];
    if matches!(row.kind, OpeningKind::Rear | OpeningKind::Floor) { return false; }
    let (at, n, door) = (row.at, row.n, row.door);
    if door.map_or(false, |d| m::abs(g.dr[d].leaf.angle) < 0.5) { return false; }
    for s in [1.0, -1.0] {
        let room = match door {
            Some(d) => crate::cleared::cleared_rooms_room_by(g, d, s),
            None => BEYOND_M.iter().map(|&d| crate::cleared::cleared_rooms_room_at(g, [at[0] + n[0] * s * d, 0.0, at[2] + n[1] * s * d])).find(|&r| js::truthy(r)).unwrap_or(0.0),
        };
        if !crate::cleared::cleared_rooms_is_known(g, room) { return true; }
    }
    false
}

/// WHAT THE SQUAD HAS A GUN ON (nothing of the game asks it; the checks do): the open ways onto floor nobody of the team has been on, within `reach` of a man who has a line to it,
/// each with whether a gun of the team's, his player's among them, is within 25 degrees of it with a line.
pub fn cover_watched(g: &mut Game, reach: f64) -> Vec<(CoverOpeningId, bool)> {
    let mut eyes: Vec<(V3, f64)> = Vec::new();
    for gd in crate::seats::team_men(g, &crate::seats::here(g)) {
        if crate::condition::down(g, g.gd[gd].ch) { continue; }
        let c = g.gd[gd].ch;
        eyes.push((g.ch[c].pos, g.ch[c].aim_yaw));
    }
    let mut guns = eyes.clone();
    for pl in crate::seats::men_up(g) { let c = g.pl[pl].ch; guns.push((g.ch[c].pos, g.ch[c].aim_yaw)); }
    let mut out = Vec::new();
    for o in cover_ready(g) {
        let at = g.tab.cover_opening[o].at;
        if !eyes.iter().any(|&(p, _)| v3::dist_xz(p, at) <= reach) || !onto_unknown(g, o) { continue; }
        let mut near = false;
        for &(p, _) in &eyes { if !near && v3::dist_xz(p, at) <= reach { near = cover_line(g, cover_eye_of(p), o); } }
        if !near { continue; }
        let mut on = false;
        for &(p, yaw) in &guns { if !on && m::abs(v3::wrap_angle(v3::yaw_to(p, at) - yaw)) < ON_IT { on = cover_line(g, cover_eye_of(p), o); } }
        out.push((o, on));
    }
    out
}

/// What every stage of the deal reads: who is free to be dealt, in what order, where everybody stands, and how the free men are moving.
struct Cx {
    squad: Vec<GuardId>,
    sams: Vec<CharId>,
    men: Vec<GuardId>,
    /// the free men with their orders, those following first in their places in the file
    list: Vec<(GuardId, TaskId)>,
    /// the free men following, front to back
    file: Vec<GuardId>,
    bodies: Vec<(CharId, V3)>,
    asg_of: JsMap<GuardId, TaskId>,
    spd: JsMap<GuardId, f64>,
    ways: JsMap<GuardId, Option<f64>>,
}
/// What is laid out before anybody is dealt anything.
struct Laid {
    live: Vec<LiveOpening>,
    /// the open doors neither a player's gun nor a man on his mark has
    open_doors: Vec<CoverOpeningId>,
    /// the opening the file's last man has as its rear
    rear_l: Option<CoverOpeningId>,
    /// the men pinned to a way behind a stack
    pins: Vec<(GuardId, CoverOpeningId)>,
}
/// The dealing so far: the live openings, and the row each man got. (`live` no longer changes shape but for a row added at its end: a row's index is its handle)
struct Hand {
    live: Vec<LiveOpening>,
    got: JsMap<GuardId, usize>,
}

/// Does somebody stood at `p` fill the opening: his feet in its frame, not beside it. A man walking past along the wall does not, however near he goes.
fn in_frame(g: &Game, o: CoverOpeningId, p: V3) -> bool {
    let row = &g.tab.cover_opening[o];
    let (dx, dz) = (p[0] - row.at[0], p[2] - row.at[2]);
    if matches!(row.kind, OpeningKind::Rear | OpeningKind::Floor) { return m::hyp2(dx, dz) < g.k.cover.in_m; }
    m::abs(dx * row.n[0] + dz * row.n[1]) < FRAME_DEEP && m::abs(dx * row.n[1] - dz * row.n[0]) < row.half + FRAME_WIDE
}

fn near(bodies: &[(CharId, V3)], p: V3) -> f64 {
    let mut d = m::INF;
    for &(_, b) in bodies { d = m::min(d, v3::dist_xz(b, p)); }
    d
}

/// Nobody is dealt an opening he would have to cover through one of his own; the man already on it keeps it to a narrower lane.
fn clear(g: &mut Game, cx: &Cx, gd: GuardId, o: CoverOpeningId, at: V3) -> bool {
    let k = g.k.cover;
    let mine = match g.cover.jobs.get(gd) { Some(j) if j.o == o => Some(*j), _ => None };
    let me = g.gd[gd].ch;
    let p = g.pos_of(gd);
    let dx = at[0] - p[0];
    let dz = at[2] - p[2];
    let len = js::or(m::hyp2(dx, dz), 1.0);
    let fx = dx / len;
    let fz = dz / len;
    let lane = if mine.is_some() { k.lane_keep } else { k.lane };
    for &(bc, b) in &cx.bodies {
        if bc == me { continue; }   // (`b === p`: his own body)
        let sx = b[0] - p[0];
        let sz = b[2] - p[2];
        let along = sx * fx + sz * fz;
        if along > 0.15 && along < len && m::abs(sx * fz - sz * fx) < lane {
            return match mine { Some(j) => g.time - j.lane_t < k.lane_secs, None => false };
        }
    }
    if mine.is_some() { let t = g.time; g.cover.jobs.get_mut(gd).unwrap().lane_t = t; }
    true
}

fn behind_of(g: &Game, cx: &Cx, gd: GuardId, at: V3) -> bool {
    match cx.ways.get(gd).copied().flatten() {
        Some(way) => m::abs(v3::wrap_angle(v3::yaw_to(g.pos_of(gd), at) - way)) > g.k.cover.twist,
        None => false,
    }
}

/// poseFor's own rule, on his pace: over backIn he does not start walking backwards, over backOut he stops.
fn fast(g: &Game, cx: &Cx, gd: GuardId) -> bool {
    let j = g.cover.jobs.get(gd);
    let pace = j.map_or(0.0, |j| j.pace);
    let back = j.map_or(false, |j| j.back);
    m::max(*cx.spd.get(gd).unwrap(), pace) > (if back { g.k.cover.back_out } else { g.k.cover.back_in })
}

fn runs(g: &Game, cx: &Cx, gd: GuardId, at: V3) -> bool { fast(g, cx, gd) && behind_of(g, cx, gd, at) }

fn sam_on(g: &Game, cx: &Cx, o: CoverOpeningId) -> bool {
    if matches!(g.tab.cover_opening[o].kind, OpeningKind::Rear | OpeningKind::Floor) { return false; }
    let at = g.tab.cover_opening[o].at;
    for &sam in &cx.sams {
        let (sp, sy) = (g.ch[sam].pos, g.ch[sam].aim_yaw);
        if m::abs(v3::wrap_angle(v3::yaw_to(sp, at) - sy)) < g.k.cover.sam_deg * m::PI / 180.0 {
            let eye = cover_eye_of(sp);
            if cover_line(g, eye, o) { return true; }
        }
    }
    false
}

fn held_by(g: &mut Game, cx: &Cx, o: CoverOpeningId) -> bool {
    if matches!(g.tab.cover_opening[o].kind, OpeningKind::Rear | OpeningKind::Floor) { return false; }
    let (at, door) = (g.tab.cover_opening[o].at, g.tab.cover_opening[o].door);
    for &gd in &cx.squad {
        if cx.men.contains(&gd) { continue; }
        let a = match crate::duty::order(g, gd) { Some(a) => a, None => continue };
        let row = &g.tab.task[a];
        let post = match row.post { Some(p) => p, None => continue };
        if v3::dist_xz(g.pos_of(gd), post) > 1.5 { continue; }
        if (door.is_some() && row.door == door) || row.watch.map_or(false, |w| v3::dist_xz(w, at) < 1.5) { return true; }
    }
    false
}

/// Weigh one opening and put it on the list if some free man can take it or sees its leaf's edge: its index in `live`.
fn add(g: &mut Game, cx: &Cx, live: &mut Vec<LiveOpening>, o: CoverOpeningId, kind_k: f64) -> Option<usize> {
    let k = g.k.cover;
    let oat = g.tab.cover_opening[o].at;
    let mut at: JsMap<GuardId, V3> = JsMap::new();
    let mut can: Vec<GuardId> = Vec::new();
    let mut edge: Vec<GuardId> = Vec::new();
    for &gd in &cx.men {
        if in_frame(g, o, g.pos_of(gd)) || runs(g, cx, gd, oat) { continue; }
        let a = *cx.asg_of.get(gd).unwrap();
        if cover_walk_only(g, gd, a) && !cover_walk_door(g, o, a) { continue; }
        let eye = cover_eye_of(g.pos_of(gd));
        let to = match cover_sight(g, eye, o) { Some(to) => to, None => continue };
        if !clear(g, cx, gd, o, to) { continue; }
        at.set(gd, to);
        if to == oat { can.push(gd); } else { edge.push(gd); }   // (`=== o.at`, by value: the edge's point is never the frame's centre)
    }
    if can.is_empty() && edge.is_empty() { return None; }
    let on_sam = sam_on(g, cx, o);
    let held = held_by(g, cx, o);
    let w = kind_k / (1.0 + near(&cx.bodies, oat) / k.near_m) * (if on_sam { k.sam_k } else { 1.0 }) * (if held { k.held_k } else { 1.0 });
    live.push(LiveOpening { o, w, can, edge, at, by: None, edge_by: None, sam: on_sam, held });
    Some(live.len() - 1)
}

/// A rear opening's point has moved: the jobs on it keep a copy, so each is written anew.
fn rear_moved(g: &mut Game, r: CoverOpeningId) {
    let at = g.tab.cover_opening[r].at;
    for (_, j) in g.cover.jobs.iter_mut() { if j.o == r { j.to = at; } }
}

/// The point his gun rests on along (dx, dz): the furthest of three his eye has a line to.
fn along(g: &Game, p: V3, dx: f64, dz: f64, l: f64, far: bool) -> Option<V3> {
    let h = m::hyp2(dx, dz);
    if !(h > 1e-6) { return None; }
    let rear_m = g.k.cover.rear_m;
    for s in [1.0, 0.6, 0.35] {
        let r = if far { l * s } else { m::min(l, rear_m * s) };
        if r < 1.2 { break; }
        let q: V3 = [p[0] + dx / h * r, 0.0, p[2] + dz / h * r];
        let eye = cover_eye_of(p);
        if !crate::scene_collision::segment_blocked(g, eye, [q[0], CHEST, q[2]], None, None, None) { return Some(q); }
    }
    None
}

fn find_live(live: &[LiveOpening], o: CoverOpeningId) -> Option<usize> { live.iter().position(|l| l.o == o) }

/// Every dealing of `hands` men to the pool tried: the best, -1 being none.
struct Search<'a> { score: &'a [Vec<f64>], used: Vec<bool>, cur: Vec<i32>, best: Vec<i32>, bs: f64 }
fn rec(z: &mut Search, i: usize, s: f64) {
    if i == z.score.len() {
        if s > z.bs + 1e-9 { z.bs = s; z.best = z.cur.clone(); }
        return;
    }
    z.cur[i] = -1;
    rec(z, i + 1, s);
    for j in 0..z.used.len() {
        let v = z.score[i][j];
        if z.used[j] || v <= 0.0 { continue; }
        z.used[j] = true;
        z.cur[i] = j as i32;
        rec(z, i + 1, s + v);
        z.used[j] = false;
    }
}

/// The deal: every live opening weighed, the free men dealt one each.
fn cover_deal(g: &mut Game) {
    let sams: Vec<CharId> = crate::seats::men_up(g).iter().map(|&pl| g.pl[pl].ch).collect();
    let squad = crate::seats::team_men(g, &crate::seats::here(g));
    let list = free_men(g, &squad);
    if list.is_empty() { g.cover.live = Vec::new(); cover_prune_steps(g); return; }
    let cx = who_is_where(g, sams, squad, list);
    let (mut live, open_doors) = live_openings(g, &cx);
    one_mouth_a_passage(g, &cx, &mut live);
    let rear_l = rear_of_the_file(g, &cx, &mut live);
    for &(gd, a) in &cx.list { rear_of_a_spare_man(g, &cx, &mut live, gd, a); }
    let pins = pinned_ways(g, &cx, &mut live);
    js::sort_by(&mut live, |a, b| b.w - a.w);
    deal_out(g, &cx, Laid { live, open_doors, rear_l, pins });
}

/// The men free to be dealt, with their orders: those following first, by their places in the file.
fn free_men(g: &mut Game, squad: &[GuardId]) -> Vec<(GuardId, TaskId)> {
    let mut list: Vec<(GuardId, TaskId)> = Vec::new();
    for &gd in squad { if let Some(a) = cover_free(g, gd) { list.push((gd, a)); } }
    let tab = &g.tab.task;
    let key = |a: TaskId| if tab[a].post.is_none() { tab[a].follow } else { 100.0 };
    js::sort_by(&mut list, |a, b| key(a.1) - key(b.1));
    list
}

/// Everything the stages of the deal read, as it stands now.
fn who_is_where(g: &Game, sams: Vec<CharId>, squad: Vec<GuardId>, list: Vec<(GuardId, TaskId)>) -> Cx {
    let men: Vec<GuardId> = list.iter().map(|x| x.0).collect();
    let mut asg_of: JsMap<GuardId, TaskId> = JsMap::new();
    for &(gd, a) in &list { asg_of.set(gd, a); }
    let file: Vec<GuardId> = list.iter().filter(|x| g.tab.task[x.1].post.is_none()).map(|x| x.0).collect();
    let mut bodies: Vec<(CharId, V3)> = squad.iter().map(|&gd| (g.gd[gd].ch, g.pos_of(gd))).collect();
    for &sam in &sams { bodies.push((sam, g.ch[sam].pos)); }
    let mut spd: JsMap<GuardId, f64> = JsMap::new();
    let mut ways: JsMap<GuardId, Option<f64>> = JsMap::new();
    for &gd in &men { let v = g.ch[g.gd[gd].ch].vel; spd.set(gd, m::hyp2(v[0], v[2])); }
    for &gd in &men {
        let v = g.ch[g.gd[gd].ch].vel;
        ways.set(gd, if *spd.get(gd).unwrap() > 0.4 { Some(m::atan2(v[0], v[2])) } else { None });
    }
    Cx { squad, sams, men, list, file, bodies, asg_of, spd, ways }
}

/// The openings in reach that nobody of ours stands in or is going through, weighed; and the open doors nobody has a gun on.
fn live_openings(g: &mut Game, cx: &Cx) -> (Vec<LiveOpening>, Vec<CoverOpeningId>) {
    let k = g.k.cover;
    let mut live: Vec<LiveOpening> = Vec::new();
    let mut open_doors: Vec<CoverOpeningId> = Vec::new();
    for o in g.cover.openings.clone() {
        let (oat, n, door) = (g.tab.cover_opening[o].at, g.tab.cover_opening[o].n, g.tab.cover_opening[o].door);
        if near(&cx.bodies, oat) > k.reach { continue; }
        if cx.bodies.iter().any(|&(_, b)| in_frame(g, o, b)) { continue; }   // one of ours is standing in it
        if going_through(cx, oat, n) { continue; }   // … or the file is going through it
        let shut = match door { Some(d) => m::abs(g.dr[d].leaf.angle) < 0.5, None => false };
        let l = add(g, cx, &mut live, o, if shut { k.shut_k } else { 1.0 });
        if door.is_some() && !shut {
            let covered = match l { Some(i) => live[i].sam || live[i].held, None => sam_on(g, cx, o) || held_by(g, cx, o) };
            if !covered { open_doors.push(o); }
        }
    }
    (live, open_doors)
}

/// Somebody of ours stands close on either side of that plane.
fn going_through(cx: &Cx, oat: V3, n: [f64; 2]) -> bool {
    let (mut pos, mut neg) = (false, false);
    for &(_, b) in &cx.bodies {
        if v3::dist_xz(b, oat) > 2.5 { continue; }
        if (b[0] - oat[0]) * n[0] + (b[2] - oat[2]) * n[1] >= 0.0 { pos = true; } else { neg = true; }
    }
    pos && neg
}

/// A passage looked into from outside is one opening, its nearer mouth; with one of ours inside it, its two mouths are his front and his back.
fn one_mouth_a_passage(g: &Game, cx: &Cx, live: &mut Vec<LiveOpening>) {
    let keep: Vec<bool> = (0..live.len()).map(|i| mouth_stays(g, cx, live, i)).collect();
    let mut at_i = 0;
    live.retain(|_| { let y = keep[at_i]; at_i += 1; y });
}

/// Not a passage's mouth, the first of its two, or one of ours is inside the passage.
fn mouth_stays(g: &Game, cx: &Cx, live: &[LiveOpening], i: usize) -> bool {
    let l = &live[i];
    let row = &g.tab.cover_opening[l.o];
    let (Some(pass), Some(far)) = (&row.pass, row.far) else { return true };
    let twin = (0..live.len()).find(|&j| j != i && g.tab.cover_opening[live[j].o].pass.as_ref() == Some(pass));
    let first = match twin {
        None => true,
        Some(t) => live[t].w < l.w || (live[t].w == l.w && g.tab.cover_opening[live[t].o].id > row.id),
    };
    if first { return true; }
    let (oat, half) = (row.at, row.half);
    let ax = far[0] - oat[0];
    let az = far[2] - oat[2];
    let l2 = js::or(ax * ax + az * az, 1.0);
    cx.bodies.iter().any(|&(_, b)| {
        let t = ((b[0] - oat[0]) * ax + (b[2] - oat[2]) * az) / l2;
        t > 0.0 && t < 1.0 && m::abs((b[0] - oat[0]) * az - (b[2] - oat[2]) * ax) / m::sqrt(l2) < half + 0.3
    })
}

/// The rear: the way they came, back along Sam's trail past the last man. The answer is the opening that man has as the rear, if the file is long enough to give him one.
fn rear_of_the_file(g: &mut Game, cx: &Cx, live: &mut Vec<LiveOpening>) -> Option<CoverOpeningId> {
    let k = g.k.cover;
    let last_m = *cx.file.last()?;
    let lp = g.pos_of(last_m);
    let back = way_the_file_came(g, cx, last_m, lp)?;
    let brg = v3::yaw_to(lp, back);
    let on_it = live.iter().any(|l| l.can.contains(&last_m) && m::abs(v3::wrap_angle(v3::yaw_to(lp, g.tab.cover_opening[l.o].at) - brg)) < 0.35);
    if !on_it {
        let r = g.cover.rear.unwrap();
        let bl = v3::dist_xz(back, lp);
        set_rear(g, r, back, lp, bl);
        add(g, cx, live, r, k.rear_k);
    }
    // a file of three or more: its last man has the rear, the heaviest thing behind him, and he stays on the one he has unless another is clearly heavier
    if !(cx.file.len() as f64 >= k.rear_from) { return None; }
    let mine = g.cover.jobs.get(last_m).map(|j| j.o);
    let mut rear_l: Option<CoverOpeningId> = None;
    let mut best_w = 0.0;
    for l in live.iter() {
        let w = l.w * (if Some(l.o) == mine { k.rear_keep } else { 1.0 });
        let off = m::abs(v3::wrap_angle(v3::yaw_to(lp, g.tab.cover_opening[l.o].at) - brg));
        if l.can.contains(&last_m) && off < k.rear_deg * m::PI / 180.0 && (rear_l.is_none() || w > best_w) { rear_l = Some(l.o); best_w = w; }
    }
    rear_l
}

/// The furthest point back along Sam's trail the last man has a line to and is not walking towards; none if he is moving too fast to look back.
fn way_the_file_came(g: &mut Game, cx: &Cx, last_m: GuardId, lp: V3) -> Option<V3> {
    let rear_m = g.k.cover.rear_m;
    let mut back: Option<V3> = None;
    let lway = cx.ways.get(last_m).copied().flatten();
    for s in [1.0, 0.6, 0.35] {
        let q = match trail_back(g, last_m, rear_m * s) { Some(q) => q, None => continue };
        if v3::dist_xz(q, lp) <= 1.2 { continue; }
        if let Some(w) = lway { if m::abs(v3::wrap_angle(v3::yaw_to(lp, q) - w)) < m::PI / 2.0 { continue; } }
        let eye = cover_eye_of(lp);
        if !crate::scene_collision::segment_blocked(g, eye, [q[0], CHEST, q[2]], None, None, None) { back = Some(q); break; }
    }
    if fast(g, cx, last_m) { None } else { back }
}

/// A rear opening goes to `back`, facing from `p`, with the jobs on it. `bl`: the distance to divide by.
fn set_rear(g: &mut Game, r: CoverOpeningId, back: V3, p: V3, bl: f64) {
    let row = &mut g.tab.cover_opening[r];
    row.at[0] = back[0];
    row.at[2] = back[2];
    row.n = [(back[0] - p[0]) / bl, (back[2] - p[2]) / bl];
    rear_moved(g, r);
}

/// A man whose eyes are spare on a mark: HIS rear is the way he came to it, an opening only he can take, unless he already has a gun that way.
fn rear_of_a_spare_man(g: &mut Game, cx: &Cx, live: &mut Vec<LiveOpening>, gd: GuardId, a: TaskId) {
    let Some(back) = way_he_came(g, cx, gd, a) else { return };
    let p = g.pos_of(gd);
    let brg = v3::yaw_to(p, back);
    if live.iter().any(|l| l.can.contains(&gd) && m::abs(v3::wrap_angle(v3::yaw_to(p, g.tab.cover_opening[l.o].at) - brg)) < 0.35) { return; }
    let r = his_rear_row(g, gd);
    let bl = js::or(v3::dist_xz(back, p), 1.0);
    set_rear(g, r, back, p, bl);
    if let Some(i) = add(g, cx, live, r, g.k.cover.rear_k) {
        live[i].can.retain(|&m_| m_ == gd);
        if live[i].can.is_empty() { live.remove(i); }
    }
}

/// The point at his back: what he reads of his room from a door-less mark, or else the way he walked to it, or else the door's face behind him.
fn way_he_came(g: &mut Game, cx: &Cx, gd: GuardId, a: TaskId) -> Option<V3> {
    let k = g.k.cover;
    let (spare, post, from) = (g.tab.task[a].spare.unwrap_or(false), g.tab.task[a].post, g.tab.task[a].from);
    let (Some(post), Some(from), true) = (post, from, spare) else { return None };
    if cover_walk_only(g, gd, a) { return None; }
    let p = g.pos_of(gd);
    let adoor = g.tab.task[a].door;
    let margin = if g.cover.reads.get(a).map_or(false, |r| r.pick.is_some()) { 0.5 } else { 0.0 };
    let read = if adoor.is_none() && v3::dist_xz(p, post) <= k.rear_read_in + margin {
        let points: Vec<V3> = cx.bodies.iter().map(|x| x.1).collect();
        cover_read_rear(g, gd, a, &cx.list, &points)
    } else { None };
    if let Some(read) = read {
        let dx = read.at[0] - p[0];
        let dz = read.at[2] - p[2];
        let back = along(g, p, dx, dz, m::min(m::hyp2(dx, dz) - (if read.way { 0.7 } else { 0.0 }), k.rear_look), true);
        if back.is_some() { return back; }
    }
    // nothing read, or nothing of it his eye can rest on: the old rule
    let mut dx = from[0] - p[0];
    let mut dz = from[2] - p[2];
    let mut l = m::hyp2(dx, dz);
    if l < 1.5 {
        let d = match adoor { Some(d) if v3::dist_xz(p, post) <= 0.6 => d, _ => return None };
        let fc = g.dr[d].frame_centre;
        dx = p[0] - fc[0];
        dz = p[2] - fc[2];
        let th = g.dr[d].def.closed_dir;
        let nx = -m::sin(th);
        let nz = m::cos(th);
        let s = if dx * nx + dz * nz >= 0.0 { 1.0 } else { -1.0 };
        dx = nx * s;
        dz = nz * s;
        l = k.rear_m;
    }
    along(g, p, dx, dz, l, false)
}

/// his row for the rear he has on a mark
fn his_rear_row(g: &mut Game, gd: GuardId) -> CoverOpeningId {
    if let Some(&r) = g.cover.rears.get(gd) { return r; }
    let id = alloc::format!("rear:{}", g.gd[gd].callsign);
    let r = g.tab.cover_opening.push(Opening { id, kind: OpeningKind::Rear, at: [0.0, CHEST, 0.0], n: [0.0, 1.0], half: 1.0, door: None, pass: None, far: None });
    g.cover.rears.set(gd, r);
    r
}

/// The rear man of a stack with an open way at its back: that way is his, ahead of every dealing below.
fn pinned_ways(g: &mut Game, cx: &Cx, live: &mut Vec<LiveOpening>) -> Vec<(GuardId, CoverOpeningId)> {
    let mut pins: Vec<(GuardId, CoverOpeningId)> = Vec::new();
    for &(gd, _) in &cx.list {
        let Some(pin) = rear_pin(g, gd) else { continue };
        let Some(to) = where_on_the_pin(g, gd, &pin) else { continue };
        let i = live_row_of(g, cx, live, pin.o);
        if !live[i].can.contains(&gd) { live[i].can.push(gd); }
        live[i].edge.retain(|&m_| m_ != gd);
        live[i].at.set(gd, to);
        pins.push((gd, pin.o));
    }
    pins
}

/// The point of his pinned way he has his gun on: what he sees of it, what he had, what he will see from his mark, or what the pin was made with.
fn where_on_the_pin(g: &Game, gd: GuardId, pin: &crate::rearwatch::RearPin) -> Option<V3> {
    let o = pin.o;
    let j0 = g.cover.jobs.get(gd).copied();
    let post = g.tab.task[pin.a].post;
    let p = g.pos_of(gd);
    let mut to = crate::rearwatch::rear_sees(g, p, o, pin.front);
    if to.is_none() { if let Some(j) = j0 { if !j.off && j.o == o { to = Some(j.to); } } }
    if to.is_none() {
        if let Some(po) = post {
            if v3::dist_xz(p, po) < 1.0 { to = crate::rearwatch::rear_sees(g, po, o, pin.front); }
        }
    }
    to.or(pin.to)
}

/// That opening's row of the list, a row nobody can take yet if it had none.
fn live_row_of(g: &Game, cx: &Cx, live: &mut Vec<LiveOpening>, o: CoverOpeningId) -> usize {
    if let Some(i) = find_live(live, o) { return i; }
    let oat = g.tab.cover_opening[o].at;
    let w = 1.0 / (1.0 + near(&cx.bodies, oat) / g.k.cover.near_m);
    let sam = sam_on(g, cx, o);
    live.push(LiveOpening { o, w, can: Vec::new(), edge: Vec::new(), at: JsMap::new(), by: None, edge_by: None, sam, held: false });
    live.len() - 1
}

/// The second half of the deal: who gets which, and the jobs handed out.
fn deal_out(g: &mut Game, cx: &Cx, laid: Laid) {
    let Laid { live, open_doors, rear_l, pins } = laid;
    let last = cx.file.last().copied();
    let mut h = Hand { live, got: JsMap::new() };
    rear_man_first(g, &mut h, rear_l, last, &pins);
    pinned_men_next(&mut h, &pins);
    keeps_what_he_was_just_dealt(g, cx, &mut h);
    best_dealing(g, cx, &mut h);
    leaf_edge_for_the_rest(cx, &mut h);
    open_way_as_he_goes_by(g, cx, &mut h);
    floor_nobody_looked_over(g, cx, &mut h);
    for &gd in &cx.men { hand_out_job(g, cx, &h, gd); }
    g.cover.live = h.live.iter().filter(|l| !l.can.is_empty()).cloned().collect();
    let mut got_ids: JsMap<GuardId, CoverOpeningId> = JsMap::new();
    for (gd, &i) in h.got.iter() { got_ids.set(gd, h.live[i].o); }
    let bodies = &cx.bodies;
    let near_m = g.k.cover.near_m;
    let mut w_of = |g: &mut Game, o: CoverOpeningId| 1.0 / (1.0 + near(bodies, g.tab.cover_opening[o].at) / near_m);
    cover_step_deal(g, &cx.list, &cx.asg_of, &got_ids, &h.live, &open_doors, bodies, &cx.spd, &mut w_of, if rear_l.is_some() { last } else { None });
}

/// That row is his to take.
fn give(h: &mut Hand, gd: GuardId, i: usize) {
    h.got.set(gd, i);
    h.live[i].by = Some(gd);
}

/// The file's last man has its rear, unless he is keeping a taken room's door with a way pinned on him.
fn rear_man_first(g: &Game, h: &mut Hand, rear_l: Option<CoverOpeningId>, last: Option<GuardId>, pins: &[(GuardId, CoverOpeningId)]) {
    let (Some(ro), Some(last_m)) = (rear_l, last) else { return };
    if crate::rearwatch::room_kept(g, &crate::seats::here(g), last_m) && pins.iter().any(|x| x.0 == last_m) { return; }
    let i = find_live(&h.live, ro).unwrap();
    give(h, last_m, i);
}

/// A pinned man has his way, if nobody has been given it.
fn pinned_men_next(h: &mut Hand, pins: &[(GuardId, CoverOpeningId)]) {
    for &(gd, o) in pins {
        let i = find_live(&h.live, o).unwrap();
        if h.live[i].by.is_none() && !h.got.has(gd) { give(h, gd, i); }
    }
}

/// A man keeps what he was just dealt: `keep_secs` of it, while it is still live and his to take.
fn keeps_what_he_was_just_dealt(g: &Game, cx: &Cx, h: &mut Hand) {
    for &gd in &cx.men {
        let j = match g.cover.jobs.get(gd) { Some(j) => *j, None => continue };
        if h.got.has(gd) || g.time - j.t0 >= g.k.cover.keep_secs { continue; }
        if let Some(i) = h.live.iter().position(|x| x.o == j.o && x.by.is_none() && x.can.contains(&gd)) { give(h, gd, i); }
    }
}

/// The men with nothing yet and the eight heaviest openings still free: the dealing of the best total.
fn best_dealing(g: &Game, cx: &Cx, h: &mut Hand) {
    let hands: Vec<GuardId> = cx.men.iter().copied().filter(|&gd| !h.got.has(gd)).collect();
    let mut pool: Vec<usize> = (0..h.live.len()).filter(|&i| h.live[i].by.is_none() && !h.live[i].can.is_empty()).collect();
    pool.truncate(8);
    let mut score: Vec<Vec<f64>> = Vec::with_capacity(hands.len());
    for &gd in &hands {
        score.push(scores_of(g, cx, &h.live, &pool, gd));
    }
    let mut z = Search { score: &score, used: vec![false; pool.len()], cur: vec![-1; hands.len()], best: vec![-1; hands.len()], bs: -m::INF };
    rec(&mut z, 0, 0.0);
    let best = z.best;
    for (i, &gd) in hands.iter().enumerate() {
        let j = best[i];
        if j >= 0 { give(h, gd, pool[j as usize]); }
    }
}

/// What each opening of the pool is worth to this man: heavy and near, little turn from where his gun is, the one he has, his lane; not behind him unless he is the last man.
fn scores_of(g: &Game, cx: &Cx, live: &[LiveOpening], pool: &[usize], gd: GuardId) -> Vec<f64> {
    let k = g.k.cover;
    let last = cx.file.last().copied();
    let p = g.pos_of(gd);
    let job_o = g.cover.jobs.get(gd).map(|j| j.o);
    let mut row: Vec<f64> = Vec::with_capacity(pool.len());
    for &i in pool {
        if !live[i].can.contains(&gd) { row.push(0.0); continue; }
        let o = live[i].o;
        let oat = g.tab.cover_opening[o].at;
        let turn = m::abs(v3::wrap_angle(v3::yaw_to(p, oat) - g.ch[g.gd[gd].ch].aim_yaw));
        let behind = Some(gd) != last && behind_of(g, cx, gd, oat);
        let lane = crate::gunroles::gun_lane(g, gd, v3::dist_xz(p, oat), k.reach);
        row.push(m::max(0.05, 10.0 * live[i].w * (if behind { k.behind_k } else { 1.0 }) - 0.25 * v3::dist_xz(p, oat) / k.reach - 0.35 * turn / m::PI + (if job_o == Some(o) { k.sticky } else { 0.0 }) + lane));
    }
    row
}

/// A man left with nothing: the leaf's edge of the heaviest open door he sees no further into.
fn leaf_edge_for_the_rest(cx: &Cx, h: &mut Hand) {
    for &gd in &cx.men {
        if h.got.has(gd) { continue; }
        if let Some(i) = h.live.iter().position(|x| x.by.is_none() && x.edge_by.is_none() && x.edge.contains(&gd)) {
            h.got.set(gd, i);
            h.live[i].edge_by = Some(gd);
        }
    }
}

/// One still left with nothing beside an open way onto floor the team has not been on: his gun is on it as he goes by, whoever else has it.
fn open_way_as_he_goes_by(g: &mut Game, cx: &Cx, h: &mut Hand) {
    let near_m = g.k.cover.near_m;
    for &gd in &cx.men {
        if h.got.has(gd) { continue; }
        let p = g.pos_of(gd);
        let mut best: Option<(usize, f64)> = None;
        for i in 0..h.live.len() {
            let d = v3::dist_xz(p, g.tab.cover_opening[h.live[i].o].at);
            if d > near_m || !h.live[i].can.contains(&gd) || best.map_or(false, |b| b.1 <= d) || !onto_unknown(g, h.live[i].o) { continue; }
            best = Some((i, d));
        }
        if let Some((i, _)) = best { h.got.set(gd, i); }
    }
}

/// One with nothing at all: floor of his room that nobody has looked over.
fn floor_nobody_looked_over(g: &mut Game, cx: &Cx, h: &mut Hand) {
    let mut taken: Vec<V3> = Vec::new();
    for &gd in &cx.men {
        if h.got.has(gd) || cover_walk_only(g, gd, *cx.asg_of.get(gd).unwrap()) { continue; }
        let Some(to) = crate::cleared::unlooked_by(g, gd, &taken) else { continue };
        if runs(g, cx, gd, to) { continue; }
        taken.push(to);
        let o = floor_row(g, gd, to);
        let mut at = JsMap::new();
        at.set(gd, to);
        h.live.push(LiveOpening { o, w: 1.0, can: vec![gd], edge: Vec::new(), at, by: Some(gd), edge_by: None, sam: false, held: false });
        h.got.set(gd, h.live.len() - 1);
    }
}

/// His job: dropped with nothing dealt, moved on the same opening, or a new one that starts from where his gun is.
fn hand_out_job(g: &mut Game, cx: &Cx, h: &Hand, gd: GuardId) {
    let li = h.got.get(gd).copied();
    let j = g.cover.jobs.get(gd).copied();
    let Some(li) = li else {
        if j.is_some() { cover_drop(g, gd, None); }
        return;
    };
    let o = h.live[li].o;
    let oat = g.tab.cover_opening[o].at;
    let to = h.live[li].at.get(gd).copied().unwrap_or(oat);
    if let Some(jj) = j {
        if jj.o == o { g.cover.jobs.get_mut(gd).unwrap().to = to; return; }
    }
    let pp = match j { Some(jj) => Some(jj), None => cover_kept_of(g, gd) };
    if pp.is_none() { cover_unkeep(g, gd, None); }
    g.cover.kept.delete(gd);
    let c = g.gd[gd].ch;
    let cp = g.ch[c].pos;
    let r = m::max(1.5, v3::dist_xz(cp, oat));
    let yaw = match pp { Some(q) => q.yaw, None => g.ch[c].aim_yaw };
    let rr = match pp { Some(q) => q.r, None => r };
    let aim: V3 = match pp { Some(q) => q.aim, None => [cp[0] + m::sin(yaw) * rr, CHEST, cp[2] + m::cos(yaw) * rr] };
    let t = g.time;
    let a = *cx.asg_of.get(gd).unwrap();
    g.cover.jobs.set(gd, CoverJob { o, to, aim, yaw, r: rr, t0: t, aim_t: t, back: pp.map_or(false, |q| q.back), lane_t: t, off: false, pace: 0.0, a });
}

/// his row for the floor he looks over, at that place now
fn floor_row(g: &mut Game, gd: GuardId, to: V3) -> CoverOpeningId {
    let o = match g.cover.floors.get(gd) {
        Some(&o) => o,
        None => {
            let id = alloc::format!("floor:{}", g.gd[gd].callsign);
            let o = g.tab.cover_opening.push(Opening { id, kind: OpeningKind::Floor, at: to, n: [0.0, 1.0], half: 1.0, door: None, pass: None, far: None });
            g.cover.floors.set(gd, o);
            o
        }
    };
    g.tab.cover_opening[o].at = to;
    o
}

/// The row he was dealt: `got` names its opening, `live` holds it, one row an opening.
fn got_row<'a>(got: &JsMap<GuardId, CoverOpeningId>, live: &'a [LiveOpening], gd: GuardId) -> Option<&'a LiveOpening> {
    let o = *got.get(gd)?;
    live.iter().find(|l| l.o == o)
}

/// Step to see: every open door nobody has a line into gets one man.
fn cover_step_deal(g: &mut Game, list: &[(GuardId, TaskId)], asg_of: &JsMap<GuardId, TaskId>, got: &JsMap<GuardId, CoverOpeningId>, live: &[LiveOpening], open: &[CoverOpeningId], bodies: &[(CharId, V3)], spd: &JsMap<GuardId, f64>, w_of: &mut dyn FnMut(&mut Game, CoverOpeningId) -> f64, rear_man: Option<GuardId>) {
    let k = g.k.cover;
    let t = g.time;
    cover_prune_steps(g);
    let mut ready: Vec<GuardId> = Vec::new();
    for &(gd, a) in list {
        let p = g.pos_of(gd);
        let l = got_row(got, live, gd);
        let still = *spd.get(gd).unwrap() < 0.3;
        if !still || !g.cover.stood.has(gd) { g.cover.stood.set(gd, t); }
        let s = g.cover.steps.get(gd).copied();
        if let Some(sv) = s {
            if !sv.done {
                if v3::dist_xz(p, sv.q) < 0.35 {
                    let row = g.cover.steps.get_mut(gd).unwrap();
                    row.done = true;
                    row.t = t;
                    g.cover.no_step.set(gd, t + 2.0);
                } else if t - sv.t > k.step_secs {
                    let row = g.cover.steps.get_mut(gd).unwrap();
                    row.q = [p[0], 0.0, p[2]];
                    row.done = true;
                    row.t = t;
                    g.cover.no_step.set(gd, t + 4.0);
                }
                continue;
            }
            if let Some(l) = l {
                if !sv.saw && l.o == sv.o && l.by == Some(gd) {
                    g.cover.steps.get_mut(gd).unwrap().saw = true;
                }
            }
        }
        // (his order lets his feet go: under 'fall in' while Sam stands, or holding a spot with nothing named)
        let (post, goal, t0) = (g.tab.task[a].post, g.tab.task[a].goal, g.tab.task[a].t0);
        let may_step = if post.is_none() { sam_stopped(g, gd) } else {
            let row = &g.tab.task[a];
            crate::orders::rows::id_is(&row.order, crate::orders::Is::Holds) && row.watch.is_none() && !row.spare.unwrap_or(false) && row.via.is_empty()
        };
        let placed = post.is_some() || !(k.step_place_m > 0.0) || v3::dist_xz(p, goal) <= k.step_place_m;
        if may_step && still && placed && Some(gd) != rear_man && t - m::max(*g.cover.stood.get(gd).unwrap(), t0) >= k.step_after && t >= g.cover.no_step.get(gd).copied().unwrap_or(0.0) { ready.push(gd); }
    }
    let mut doors: Vec<(CoverOpeningId, f64)> = Vec::with_capacity(open.len());
    for &o in open { let w = w_of(g, o); doors.push((o, w)); }
    js::sort_by(&mut doors, |a, b| b.1 - a.1);
    for (o, wo) in doors {
        let x = live.iter().find(|l| l.o == o);
        if let Some(x) = x { if x.by.is_some() { continue; } }
        let none: Vec<GuardId> = Vec::new();
        let seen: &Vec<GuardId> = match x { Some(x) => &x.can, None => &none };
        let oat = g.tab.cover_opening[o].at;
        let reach = k.step_reach;
        let in_reach = |g: &Game, gd: GuardId| v3::dist_xz(g.pos_of(gd), oat) <= reach;
        if t < g.cover.door_try.get(o).copied().unwrap_or(0.0) { continue; }
        let theirs = g.cover.steps.values().any(|s| s.o == o && (!s.done || s.saw || t - s.t < 3.0));
        if theirs { continue; }   // (one man a door)
        let idle: Vec<GuardId> = ready.iter().copied().filter(|&gd| in_reach(g, gd) && !seen.contains(&gd) && got_row(got, live, gd).map_or(true, |l| l.by != Some(gd))).collect();
        let busy: Vec<GuardId> = if !seen.is_empty() { Vec::new() } else {
            ready.iter().copied().filter(|&gd| in_reach(g, gd) && got_row(got, live, gd).map_or(false, |l| l.by == Some(gd) && 10.0 * (wo - l.w) > k.sticky + 0.3)).collect()
        };
        let mut pick: Option<(GuardId, V3, f64, V3)> = None;
        for tier in [&idle, &busy] {
            if pick.is_some() { break; }
            for &gd in tier.iter() {
                let a = *asg_of.get(gd).unwrap();
                let p = g.pos_of(gd);
                let from: V3 = match (g.tab.task[a].post, g.cover.steps.get(gd)) {
                    (Some(po), _) => [po[0], 0.0, po[2]],
                    (None, Some(s)) => s.from,
                    (None, None) => [p[0], 0.0, p[2]],
                };
                if let Some(r) = cover_step_spot(g, gd, from, o, bodies) {
                    if pick.map_or(true, |c| r.c < c.2) { pick = Some((gd, r.q, r.c, from)); }
                }
            }
        }
        let (pgd, q, _, from) = match pick {
            Some(c) => c,
            None => { if !idle.is_empty() || !busy.is_empty() { g.cover.door_try.set(o, t + 2.5); } continue; }
        };
        let a = *asg_of.get(pgd).unwrap();
        g.cover.steps.set(pgd, CoverStep { q, from, o, a, t, done: false, saw: false });
        let i = ready.iter().position(|&x| x == pgd).unwrap();
        ready.remove(i);
    }
}

/// The steps that no longer stand.
fn cover_prune_steps(g: &mut Game) {
    for gd in g.cover.steps.keys() {
        let sa = g.cover.steps.get(gd).unwrap().a;
        let a = crate::duty::order(g, gd);
        let gone = crate::condition::down(g, g.gd[gd].ch) || a != Some(sa) || (g.tab.task[sa].post.is_none() && !sam_stopped(g, gd));
        if gone { g.cover.steps.delete(gd); }
    }
}

/// Is `x` in the lane a → b, within `lane` of the line.
fn in_lane(x: V3, a: V3, b: V3, lane: f64) -> bool {
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    let len = js::or(m::hyp2(dx, dz), 1.0);
    let sx = x[0] - a[0];
    let sz = x[2] - a[2];
    let along = (sx * dx + sz * dz) / len;
    along > 0.15 && along < len && m::abs(sx * dz - sz * dx) / len < lane
}

/// The spot a man steps to to see through open door `o`.
fn cover_step_spot(g: &Game, gd: GuardId, from: V3, o: CoverOpeningId, bodies: &[(CharId, V3)]) -> Option<CoverStepSpot> {
    let k = g.k.cover;
    let me = g.gd[gd].ch;
    let p = g.pos_of(gd);
    let (oat, n) = (g.tab.cover_opening[o].at, g.tab.cover_opening[o].n);
    let side = |x: V3| (x[0] - oat[0]) * n[0] + (x[2] - oat[2]) * n[1];
    let s = if side(p) >= 0.0 { 1.0 } else { -1.0 };
    let d0 = v3::dist_xz(p, oat);
    let beyond: V3 = [oat[0] - n[0] * s * 0.8, CHEST, oat[2] - n[1] * s * 0.8];
    // (the guns already up: every mate's line to what he was dealt, and Sam's own)
    let mut guns: Vec<(V3, V3)> = Vec::new();
    for pl in crate::seats::men_up(g) {
        let c = g.pl[pl].ch;
        let (sp, sy) = (g.ch[c].pos, g.ch[c].aim_yaw);
        guns.push((sp, [sp[0] + m::sin(sy) * 8.0, 0.0, sp[2] + m::cos(sy) * 8.0]));
    }
    for (mt, j) in g.cover.jobs.iter() {
        if mt != gd && !j.off { guns.push((g.pos_of(mt), j.to)); }
    }
    let ops: Vec<(V3, f64)> = g.cover.openings.iter().map(|&x| (g.tab.cover_opening[x].at, g.tab.cover_opening[x].half)).collect();
    let mut best: Option<V3> = None;
    let mut bc = m::INF;
    for r in [0.5, 0.9, 1.3, 1.7, 2.1, 2.5] {
        if r > k.step_m + 1e-6 { break; }
        for i in 0..16 {
            let a = (i as f64) * m::PI / 8.0;
            let q: V3 = [from[0] + m::sin(a) * r, 0.0, from[2] + m::cos(a) * r];
            let step = v3::dist_xz(p, q);
            if step < 0.4 { continue; }
            if side(q) * s < 0.5 || crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { continue; }
            if ops.iter().any(|x| v3::dist_xz(q, x.0) < x.1 + k.step_off_m) { continue; }
            if bodies.iter().any(|&(bc, b)| bc != me && v3::dist_xz(b, q) < 0.8) { continue; }
            if g.cover.steps.iter().any(|(mt, st)| mt != gd && v3::dist_xz(st.q, q) < 0.8) { continue; }
            if crate::scene_collision::low_blocked(g, p, q) || crate::scene_collision::solid_blocked(g, [p[0], CHEST, p[2]], [q[0], CHEST, q[2]], None) { continue; }
            if [0.35, 0.7].iter().any(|&f: &f64| {
                let mx = p[0] + (q[0] - p[0]) * f;
                let mz = p[2] + (q[2] - p[2]) * f;
                ops.iter().any(|x| m::hyp2(mx - x.0[0], mz - x.0[2]) < x.1 + 0.3)
            }) { continue; }   // (… nor does the walk there cut across one)
            let eye = cover_eye_of(q);
            if cover_sight(g, eye, o) != Some(oat) || crate::scene_collision::segment_blocked(g, eye, beyond, None, None, None) { continue; }
            if bodies.iter().any(|&(bc, b)| bc != me && in_lane(b, q, oat, k.lane)) || guns.iter().any(|&(ga, gb)| in_lane(q, ga, gb, k.lane)) { continue; }
            let len = js::or(v3::dist_xz(q, oat), 1.0);
            let c = step + 0.6 * m::abs(side(q)) / len + 0.5 * m::max(0.0, d0 - len);
            if c < bc { bc = c; best = Some(q); }
        }
    }
    best.map(|q| CoverStepSpot { q, c: bc })
}

/// Where the cover has this man step to under the order he has now.
pub fn cover_step_of(g: &Game, gd: GuardId, a: TaskId) -> Option<V3> {
    match g.cover.steps.get(gd) { Some(s) if s.a == a => Some(s.q), _ => None }
}

/// He cannot get to it: where he stands is where the step ends.
pub fn cover_step_halt(g: &mut Game, gd: GuardId) {
    let p = g.pos_of(gd);
    let t = g.time;
    let s = match g.cover.steps.get_mut(gd) { Some(s) => s, None => return };
    s.q = [p[0], 0.0, p[2]];
    if !s.done { s.done = true; s.t = t; g.cover.no_step.set(gd, t + 4.0); }
}

/// His job goes; `taken`: something else has him now and may not re-pose him.
fn cover_drop(g: &mut Game, gd: GuardId, taken: Option<bool>) {
    let taken = taken.unwrap_or(false);
    let j = match g.cover.jobs.get(gd).copied() { Some(j) => j, None => return };
    g.cover.jobs.delete(gd);
    if !taken && !j.off && script_aim_is(g, gd, j.aim) {
        let a = crate::duty::order(g, gd).unwrap_or(j.a);
        let row = &g.tab.task[a];
        if row.spare.unwrap_or(false) && row.post.is_some() && row.from.is_some() {
            let t = g.time;
            g.cover.kept.set(gd, CoverKeptValue { j, t });
            return;
        }
    }
    if !taken { cover_turn_from(g, gd, j); }
    if taken { cover_release(g, gd, j); }
}

/// What the cover wrote into his script goes with the job.
fn cover_release(g: &mut Game, gd: GuardId, j: CoverJob) {
    if g.gd[gd].script.is_none() || crate::condition::down(g, g.gd[gd].ch) { return; }
    let s = g.gd[gd].script.as_mut().unwrap();
    if j.back && s.strafe.unwrap_or(false) { s.strafe = Some(false); }
    if script_aim_is(g, gd, j.aim) { give_aim_back(g, gd, j.a); }
}

/// `s.aimAt = A.watch`, and the gun-up carriage if nobody has changed it since.
fn give_aim_back(g: &mut Game, gd: GuardId, a: TaskId) {
    let (watch, upper, stance) = (g.tab.task[a].watch, g.tab.task[a].upper, g.tab.task[a].stance);
    let s = g.gd[gd].script.as_mut().unwrap();
    s.aim_at = watch;
    if s.upper == Some(crate::character::UpperMode::Aim) && s.stance == Some(crate::character::Stance::None) {
        s.upper = Some(upper);
        s.stance = Some(stance);
    }
}

/// The deal has let go of a man walking backwards on his opening: from here his feet are turnOut's.
fn cover_turn_from(g: &mut Game, gd: GuardId, j: CoverJob) {
    let strafing = g.gd[gd].script.as_ref().map_or(false, |s| s.strafe.unwrap_or(false));
    if j.back && strafing && !crate::condition::down(g, g.gd[gd].ch) {
        let t = g.time;
        g.cover.outs.set(gd, CoverOutsValue { j, aim: j.aim, t0: t, t, on: false });
    }
}

/// The way he goes: at the corner of his path he is walking to, else the way he is moving.
fn cover_way_of(g: &Game, gd: GuardId) -> f64 {
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    if let Some(&wp) = g.gd[gd].path.get(g.gd[gd].path_i) {
        if v3::dist_xz(p, wp) > 0.3 { return v3::yaw_to(p, wp); }
    }
    g.cover.heading.get(gd).copied().unwrap_or(g.ch[c].body_yaw)
}

/// Is he still coming out of a back-pedal.
fn cover_turning(g: &Game, gd: GuardId, still: bool, pace: f64) -> bool {
    let o = match g.cover.outs.get(gd) { Some(o) => *o, None => return false };
    let k = g.k.cover;
    let t = g.time;
    !still && pace <= k.back_out && m::abs(t - o.t) <= 0.1 && m::abs(t - o.t0) < k.back_turn_secs && m::cos(g.ch[g.gd[gd].ch].body_yaw - cover_way_of(g, gd)) < k.back_turn_body
}

/// His pose while he is: the path's own feet, the body turning to the way he goes, the gun carried round in front of it.
fn cover_turn_out(g: &mut Game, gd: GuardId, still: bool, pace: f64) -> Option<CoverTurnOut> {
    let (was_aim, was_on) = { let o = g.cover.outs.get(gd)?; (o.aim, o.on) };
    if !cover_turning(g, gd, still, pace) { g.cover.outs.delete(gd); return None; }
    let ours = was_on && script_aim_is(g, gd, was_aim);   // (`on`: until his first pose the point is a copy of J.aim the script does not hold, however equal)
    let c = g.gd[gd].ch;
    let (pos, by) = (g.ch[c].pos, g.ch[c].body_yaw);
    let t = g.time;
    let o = g.cover.outs.get_mut(gd).unwrap();
    o.on = true;
    o.t = t;
    o.aim[0] = pos[0] + m::sin(by) * 3.0;
    o.aim[2] = pos[2] + m::cos(by) * 3.0;
    let aim = o.aim;
    if ours { g.gd[gd].script.as_mut().unwrap().aim_at = Some(aim); }
    let face = cover_way_of(g, gd);
    Some(CoverTurnOut { at: aim, face: Some(face), strafe: true })
}

/// Something else has him: what the turn-out wrote into his script goes.
fn cover_unturn(g: &mut Game, gd: GuardId) {
    let o = match g.cover.outs.delete(gd) { Some(o) => o, None => return };
    if g.gd[gd].script.is_none() || crate::condition::down(g, g.gd[gd].ch) { return; }
    let s = g.gd[gd].script.as_mut().unwrap();
    if s.strafe.unwrap_or(false) { s.strafe = Some(false); }
    if o.on && script_aim_is(g, gd, o.aim) { give_aim_back(g, gd, o.j.a); }
}

/// The gun comes off an opening the deal let go of.
fn cover_unkeep(g: &mut Game, gd: GuardId, taken: Option<bool>) {
    let k = match g.cover.kept.delete(gd) { Some(k) => k, None => return };
    if taken.unwrap_or(false) { cover_release(g, gd, k.j); } else { cover_turn_from(g, gd, k.j); }
}

/// The opening the deal last took off this man, while his gun is still on it.
fn cover_kept_of(g: &mut Game, gd: GuardId) -> Option<CoverJob> {
    let kv = *g.cover.kept.get(gd)?;
    let p = g.pos_of(gd);
    if m::abs(g.time - kv.t) >= g.k.rear.secs || !script_aim_is(g, gd, kv.j.aim) { return None; }
    let a = crate::duty::order(g, gd)?;
    g.cover.kept.get_mut(gd).unwrap().j.a = a;
    let mut j = kv.j;
    j.a = a;
    let k = g.k.cover;
    let dx = j.to[0] - p[0];
    let dz = j.to[2] - p[2];
    let len = js::or(m::hyp2(dx, dz), 1.0);
    let fx = dx / len;
    let fz = dz / len;
    if len < k.in_m { return None; }
    let ours = |b: V3| {
        let sx = b[0] - p[0];
        let sz = b[2] - p[2];
        let along = sx * fx + sz * fz;
        (along > 0.15 && along < len + k.in_m && m::abs(sx * fz - sz * fx) < k.lane_keep) || v3::dist_xz(b, j.to) < k.in_m
    };
    for pl in crate::seats::men_up(g) {
        if crate::group::is_seats(g) && ours(g.player_pos(pl)) { return None; }
    }
    for mt in crate::seats::team_men(g, &crate::seats::here(g)) {
        if mt != gd && !crate::condition::down(g, g.gd[mt].ch) && ours(g.pos_of(mt)) { return None; }
    }
    Some(j)
}

/// Everything goes: the knob, or no team.
fn cover_drop_all(g: &mut Game) {
    for gd in g.cover.jobs.keys() { cover_drop(g, gd, Some(true)); }
    for gd in g.cover.kept.keys() { cover_unkeep(g, gd, Some(true)); }
    for gd in g.cover.outs.keys() { cover_unturn(g, gd); }
}

/// The job back where it lives: dealt, or kept.
fn put_job(g: &mut Game, gd: GuardId, j: CoverJob) {
    if let Some(x) = g.cover.jobs.get_mut(gd) { *x = j; }
    else if let Some(x) = g.cover.kept.get_mut(gd) { x.j = j; }
}

/// Where his gun goes and how he carries himself for the opening he was dealt: None, the order's own pose stands.
pub fn cover_pose_for(g: &mut Game, gd: GuardId, still: bool, pace: f64) -> Option<CoverPoseFor> {
    let found = match g.cover.jobs.get(gd).copied() { Some(j) => Some(j), None => cover_kept_of(g, gd) };
    let mut j = match found {
        Some(j) => j,
        None => {
            cover_unkeep(g, gd, None);
            return cover_turn_out(g, gd, still, pace).map(|o| CoverPoseFor { at: o.at, face: o.face, strafe: o.strafe });
        }
    };
    let k = g.k.cover;
    let c = g.gd[gd].ch;
    let (p, aim_yaw, body_yaw) = (g.ch[c].pos, g.ch[c].aim_yaw, g.ch[c].body_yaw);
    let to = j.to;
    if g.cover.outs.has(gd) {
        if cover_turning(g, gd, still, pace) { j.back = true; }
        g.cover.outs.delete(gd);
    }
    let dt = m::min(0.1, m::max(0.0, g.time - j.aim_t));
    j.aim_t = g.time;
    let brg = v3::yaw_to(p, to);
    let f = 1.0 - m::exp(-k.swing * dt);
    let ours = script_aim_is(g, gd, j.aim);
    if !ours { j.yaw = aim_yaw; }   // (his gun was not ours last frame: it starts from where it is)
    j.pace = pace;
    // (round his front: eased against the body's own yaw it comes through the front as the body turns under it)
    let rel = v3::wrap_angle(j.yaw - body_yaw);
    let want = v3::wrap_angle(brg - body_yaw);
    j.yaw = v3::wrap_angle(body_yaw + rel + (want - rel) * f);
    j.r += (m::max(1.5, v3::dist_xz(p, to)) - j.r) * f;
    j.aim[0] = p[0] + m::sin(j.yaw) * j.r;
    j.aim[1] = v3::damp(j.aim[1], to[1], k.swing, dt);
    j.aim[2] = p[2] + m::cos(j.yaw) * j.r;
    if ours { g.gd[gd].script.as_mut().unwrap().aim_at = Some(j.aim); }
    j.off = false;
    if still {
        j.back = false;
        put_job(g, gd, j);
        return Some(CoverPoseFor { at: j.aim, face: Some(brg), strafe: false });
    }
    let way = g.cover.heading.get(gd).copied().unwrap_or(body_yaw);
    if m::abs(v3::wrap_angle(brg - way)) <= k.twist && !(j.back && pace <= k.back_out && m::cos(body_yaw - way) < k.back_body) {
        j.back = false;
        put_job(g, gd, j);
        return Some(CoverPoseFor { at: j.aim, face: None, strafe: false });
    }
    j.back = pace <= (if j.back { k.back_out } else { k.back_in });
    if j.back {
        put_job(g, gd, j);
        return Some(CoverPoseFor { at: j.aim, face: Some(brg), strafe: true });
    }
    j.off = true;
    put_job(g, gd, j);
    g.cover.kept.delete(gd);
    None
}

/// An opening in a few words, as this man would say it.
fn cover_name_of(g: &Game, o: CoverOpeningId, from: Option<V3>) -> String {
    let (kind, door, at) = (g.tab.cover_opening[o].kind, g.tab.cover_opening[o].door, g.tab.cover_opening[o].at);
    if kind == OpeningKind::Rear { return String::from("the rear"); }
    if kind == OpeningKind::Floor { return String::from("the corner"); }
    if let Some(d) = door { return alloc::format!("the {}", crate::orders::door_words(g, d)); }
    let from = match from { Some(f) => f, None => return alloc::format!("{} ({}, {})", kind.as_str(), js::to_fixed(at[0], 1), js::to_fixed(at[2], 1)) };
    let what = if kind == OpeningKind::Mouth { "aisle" } else { "gap" };
    let yaw = v3::yaw_to(from, at);
    match crate::dialogue::watched_place(g, from, [at[0] + m::sin(yaw), 0.0, at[2] + m::cos(yaw)]) {
        Some(p) if !p.is_empty() => alloc::format!("the {} to {}", what, p),
        _ => alloc::format!("the {}", what),
    }
}

/// Has he an opening dealt, and is on it.
pub fn cover_has(g: &Game, gd: GuardId) -> bool { cover_on_of(g, gd).map_or(false, |j| !j.off) }

/// The opening his gun is on: dealt, or let go of and still his for the moment.
fn cover_on_of(g: &Game, gd: GuardId) -> Option<CoverJob> {
    match g.cover.jobs.get(gd) { Some(j) => Some(*j), None => g.cover.kept.get(gd).map(|k| k.j) }
}

/// What he is on, in words.
fn cover_words(g: &Game, gd: GuardId) -> String {
    match cover_on_of(g, gd) {
        Some(j) if !j.off => { let p = g.pos_of(gd); alloc::format!("on {}", cover_name_of(g, j.o, Some(p))) }
        _ => String::new(),
    }
}

/// The tail of his row in the squad block.
pub fn cover_tail(g: &Game, gd: GuardId) -> String {
    let w = cover_words(g, gd);
    if w.is_empty() { w } else { alloc::format!(" · {}", w) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn search(score: Vec<Vec<f64>>, pool: usize) -> Vec<i32> {
        let n = score.len();
        let mut z = Search { score: &score, used: vec![false; pool], cur: vec![-1; n], best: vec![-1; n], bs: -m::INF };
        rec(&mut z, 0, 0.0);
        z.best
    }

    #[test]
    fn the_best_dealing_gives_nobody_the_same_opening() {
        // (each man's own best is opening 0, the best total hands the second man opening 1)
        assert_eq!(search(vec![vec![5.0, 1.0], vec![4.0, 3.0]], 2), vec![0, 1]);
    }

    #[test]
    fn a_tie_stays_with_the_first_dealing_tried() {
        // (a later dealing must beat the best by 1e-9: the first of two equal totals in the walk's order stands)
        assert_eq!(search(vec![vec![2.0, 2.0]], 2), vec![0]);
    }

    #[test]
    fn a_man_with_no_score_is_dealt_nothing() {
        assert_eq!(search(vec![vec![0.0, 0.0], vec![1.0, 0.0]], 2), vec![-1, 0]);
    }

    #[test]
    fn the_lane_is_ahead_of_him_short_of_the_end() {
        let (a, b) = ([0.0, 0.0, 0.0], [10.0, 0.0, 0.0]);
        assert!(in_lane([5.0, 0.0, 0.5], a, b, 0.7));
        assert!(!in_lane([5.0, 0.0, 0.8], a, b, 0.7));
        assert!(!in_lane([0.1, 0.0, 0.0], a, b, 0.7));
        assert!(!in_lane([10.0, 0.0, 0.0], a, b, 0.7));
    }
}
