//! src/game/cover.ts lines 233 to 744
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Does his script hold this point of ours. By value: a point the cover made is p + sin(yaw) * r, no other writer holds its bits, and every move of it is written through to the script here (poseFor, turnOut). (cover.ts:630,640,675,700 `gd.script?.aimAt === J.aim`)
fn script_aim_is(g: &Game, gd: GuardId, aim: V3) -> bool {
    g.gd[gd].script.as_ref().map_or(false, |s| s.aim_at == Some(aim))
}

/// The squad's book that moves this man: his seat's. (cover.ts:440 `ordersOf(g, gd).trailBack`)
fn trail_back(g: &mut Game, gd: GuardId, extra: f64) -> Option<V3> {
    let seat = crate::seats::view_of(g, gd);
    crate::seats::with_seat(g, seat, &mut |g: &mut Game| crate::orders::squad_orders_trail_back(g, gd, extra))
}

/// (cover.ts:559,586 `ordersOf(g, gd).samStopped`)
fn sam_stopped(g: &mut Game, gd: GuardId) -> bool {
    let seat = crate::seats::view_of(g, gd);
    crate::seats::with_seat(g, seat, &mut |g: &mut Game| crate::orders::squad_orders_sam_stopped(g))
}

/// (cover.ts:306,484 `rearPinOf(viewOf(g, gd), gd)`)
fn rear_pin(g: &mut Game, gd: GuardId) -> Option<crate::rearwatch::RearPin> {
    let seat = crate::seats::view_of(g, gd);
    crate::seats::with_seat(g, seat, &mut |g: &mut Game| crate::rearwatch::rear_pin_of(g, gd))
}

/// An optional boolean as a condition.
fn yes(o: Option<bool>) -> bool { o.unwrap_or(false) }

/// (cover.ts Cover) The constructor: the field initialisers, the file's one rear opening, the page's knobs.
pub fn cover_new(g: &mut Game) -> Cover {
    let mut c = Cover::blank();
    c.knobs = g.k.cover;
    c.rear = Some(g.tab.cover_opening.push(Opening { id: String::from("rear"), kind: OpeningKind::Rear, at: [0.0, CHEST, 0.0], n: [0.0, 1.0], half: 1.0, door: None, pass: None, far: None }));
    c
}

/// Nothing dealt, nothing known: a restart. (cover.ts reset)
pub fn cover_reset(g: &mut Game) {
    let c = &mut g.cover;
    c.jobs.clear();
    c.steps.clear();
    c.stood = JsMap::new();
    c.no_step = JsMap::new();
    c.door_try = JsMap::new();
    c.rears.clear();
    c.reads = JsMap::new();
    c.kept.clear();
    c.outs.clear();
    c.live = Vec::new();
    c.men = Vec::new();
    c.deal_at = 0.0;
    c.stats = CoverStats::zeroed();
    c.log.clear();
}

/// What he was dealt. (cover.ts of)
pub fn cover_of(g: &mut Game, gd: GuardId) -> Option<CoverJob> { g.cover.jobs.get(gd).copied() }

/// The opening his gun has been left on. (cover.ts keptJob)
pub fn cover_kept_job(g: &mut Game, gd: GuardId) -> Option<CoverJob> { g.cover.kept.get(gd).map(|k| k.j) }

/// The level's openings, built if they are not yet. (cover.ts ready)
pub fn cover_ready(g: &mut Game) -> Vec<CoverOpeningId> {
    if !g.cover.nav { cover_build(g); }
    g.cover.openings.clone()
}

/// The level's openings: its doors, and the gaps of its nav grid that are not a door's own doorway. (cover.ts build)
pub fn cover_build(g: &mut Game) {
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
    let leaves = core::mem::take(&mut g.col.dynamic_boxes);
    let found = {
        let mut solid = |a: V3, b: V3| crate::scene_collision::segment_blocked(g, a, b, None, None, None);
        let mut found = gaps_of(&nav, true, Some(k), Some(&mut solid));
        found.extend(gaps_of(&nav, false, Some(k), Some(&mut solid)));
        found
    };
    for o in found {
        let oat = g.tab.cover_opening[o].at;
        if doors.iter().any(|&d| v3::dist_xz(g.tab.cover_opening[d].at, oat) < 1.0) || gaps.iter().any(|&x| v3::dist_xz(g.tab.cover_opening[x].at, oat) < 0.6) { continue; }
        gaps.push(o);
    }
    g.col.dynamic_boxes = leaves;
    doors.extend(gaps);
    g.cover.openings = doors;
}

/// His order leaves his eyes spare now. (cover.ts spare)
pub fn cover_spare(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) -> bool {
    if !yes(g.tab.orders_assignment[a].spare) || g.tab.orders_assignment[a].post.is_none() { return false; }
    if g.tab.orders_assignment[a].from.is_none() { return false; }
    if rear_pin(g, gd).map_or(false, |p| yes(p.out)) { return true; }
    let door = g.tab.orders_assignment[a].door;
    for o in crate::seats::team_men(g) {
        if o == gd { continue; }
        if let Some(b) = crate::seats::order_of(g, o) {
            let row = &g.tab.orders_assignment[b];
            if row.post.is_some() && !yes(row.spare) && row.door == door { return true; }
        }
    }
    false
}

/// Is this man free to be dealt an opening: the order he is under if so. (cover.ts free)
pub fn cover_free(g: &mut Game, gd: GuardId) -> Option<OrdersAssignmentId> {
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].script.is_none() || g.time < g.gd[gd].dazzled_until || g.gd[gd].held.is_some() { return None; }
    let a = crate::seats::order_of(g, gd)?;
    let plain = g.tab.orders_assignment[a].order == "fallIn" || (g.tab.orders_assignment[a].order == "hold" && g.tab.orders_assignment[a].watch.is_none());
    if !(plain || cover_spare(g, gd, a)) { return None; }
    if crate::roomfight::room_fight_engaged(g, gd) { return None; }
    if let Some(op) = crate::roomfight::room_fight_op_of(g, gd) {
        for (h, mt) in g.tab.roomfight_op[op].meters.iter() {
            if g.gd[h].state != crate::game::GuardState::Dead && g.time - mt.seen_t < g.k.cover.contact_secs { return None; }
        }
    }
    Some(a)
}

/// What a rear man on a door-less mark watches. (cover.ts readRear)
pub fn cover_read_rear(g: &mut Game, gd: GuardId, a: OrdersAssignmentId, list: &[(GuardId, OrdersAssignmentId)], bodies: &[V3]) -> Option<crate::roomfit::Watchable> {
    let k = g.k.cover;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let now = g.time;
    let cached = g.cover.reads.get(a).cloned();
    if let Some(r) = &cached {
        if now >= r.t && now - r.t < k.rear_read_secs && v3::dist_xz(p, r.at) <= 0.5 { return r.pick.clone(); }
    }
    let row = &g.tab.orders_assignment[a];
    let front = match row.watch { Some(w) => v3::yaw_to(w, p), None => row.face.unwrap_or(g.ch[c].body_yaw) };
    let deg = k.rear_read_deg * m::PI / 180.0;
    let same = |o: &crate::roomfit::Watchable, t: &crate::roomfit::Watchable| o.key == t.key || m::abs(v3::wrap_angle(v3::yaw_to(p, o.at) - t.yaw)) < (if o.way && t.way { 0.15 } else { deg });
    let mut theirs: Vec<crate::roomfit::Watchable> = Vec::new();
    for &(mt, b) in list {
        if mt == gd || !yes(g.tab.orders_assignment[b].spare) { continue; }
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
    if pick.is_some() && pick != was { g.cover.stats.rear_read += 1.0; }
    picked
}

/// A stack's rear man still on the walk to his mark, further out than the last steps. (cover.ts walkOnly)
pub fn cover_walk_only(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) -> bool {
    let row = &g.tab.orders_assignment[a];
    if !yes(row.spare) || row.from.is_none() { return false; }
    match row.post {
        Some(post) => v3::dist_xz(g.ch[g.gd[gd].ch].pos, post) > g.k.cover.rear_walk_m,
        None => false,
    }
}

/// A door standing open by his way, with floor beyond it the team has not been on. (cover.ts walkDoor)
pub fn cover_walk_door(g: &mut Game, o: CoverOpeningId, a: OrdersAssignmentId) -> bool {
    let d = match g.tab.cover_opening[o].door { Some(d) => d, None => return false };
    if Some(d) == g.tab.orders_assignment[a].door { return false; }
    let open = crate::doors::door_off(g, d) || (!crate::doors::door_latched(g, d) && m::abs(crate::doors::door_angle(g, d)) >= g.k.doormanners.lane_ajar);
    if !open { return false; }
    let ra = crate::cleared::cleared_rooms_room_by(g, d, 1.0);
    let rb = crate::cleared::cleared_rooms_room_by(g, d, -1.0);
    !(js::truthy(ra) && js::truthy(rb) && crate::cleared::cleared_rooms_is_known(g, ra) && crate::cleared::cleared_rooms_is_known(g, rb))
}

/// His eye over a spot. (cover.ts eyeOf)
pub fn cover_eye_of(g: &mut Game, p: V3) -> V3 { [p[0], EYE, p[2]] }

/// The point of this opening a man stood at `p` can watch. (cover.ts sightFrom)
pub fn cover_sight_from(g: &mut Game, p: V3, o: CoverOpeningId) -> Option<V3> {
    let eye = cover_eye_of(g, p);
    cover_sight(g, eye, o)
}

/// A line from this eye to the opening itself. (cover.ts line)
pub fn cover_line(g: &mut Game, eye: V3, o: CoverOpeningId) -> bool {
    cover_sight(g, eye, o) == Some(g.tab.cover_opening[o].at)
}

/// The point of this opening a man at `eye` can watch: the opening, or just past an open leaf's free edge. (cover.ts sight)
pub fn cover_sight(g: &mut Game, eye: V3, o: CoverOpeningId) -> Option<V3> {
    let k = g.k.cover;
    let at = g.tab.cover_opening[o].at;
    let door = g.tab.cover_opening[o].door;
    let d = v3::dist_xz(eye, at);
    if d > k.reach { return None; }
    if d < 0.3 { return Some(at); }
    let s = if door.is_some() { m::max(0.0, 1.0 - 0.7 / d) } else { 1.0 };
    if crate::scene_collision::segment_blocked(g, eye, [eye[0] + (at[0] - eye[0]) * s, CHEST, eye[2] + (at[2] - eye[2]) * s], None, None, None) { return None; }
    let dd = match door { Some(dd) => dd, None => return Some(at) };
    if !(!crate::doors::door_off(g, dd) && m::abs(crate::doors::door_angle(g, dd)) >= 0.5 && crate::scene_collision::segment_blocked(g, eye, [at[0], CHEST, at[2]], None, None, None)) { return Some(at); }
    let u = crate::doors::door_dir(g, dd);
    let r = g.dr[dd].def.width + 0.35;
    let hinge = g.dr[dd].def.hinge;
    let t: V3 = [hinge[0] + u[0] * r, CHEST, hinge[1] + u[1] * r];
    if v3::dist_xz(eye, t) > k.in_m && !crate::scene_collision::segment_blocked(g, eye, t, None, None, None) { Some(t) } else { None }
}

/// Once a frame, before the orders pose the men. (cover.ts tick)
pub fn cover_tick(g: &mut Game) {
    let k = g.k.cover;
    if !crate::seats::team_active(g) {
        if !g.cover.jobs.is_empty() || !g.cover.kept.is_empty() || !g.cover.outs.is_empty() { cover_drop_all(g); }
        g.cover.steps.clear();
        g.cover.live = Vec::new();
        g.cover.men = Vec::new();
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
    for gd in crate::seats::team_men(g) {
        let v = g.ch[g.gd[gd].ch].vel;
        if m::hyp2(v[0], v[2]) > 0.3 { g.cover.heading.set(gd, m::atan2(v[0], v[2])); }
    }
    if g.time < g.cover.deal_at { return; }
    g.cover.deal_at = g.time + k.deal_secs;
    if !g.cover.nav { cover_build(g); }
    cover_deal(g);
}

/// What the deal's closures captured.
struct Cx {
    squad: Vec<GuardId>,
    sams: Vec<CharId>,
    men: Vec<GuardId>,
    bodies: Vec<(CharId, V3)>,
    asg_of: JsMap<GuardId, OrdersAssignmentId>,
    spd: JsMap<GuardId, f64>,
    ways: JsMap<GuardId, Option<f64>>,
}

fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }

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
    let p = pos_of(g, gd);
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
        Some(way) => m::abs(v3::wrap_angle(v3::yaw_to(pos_of(g, gd), at) - way)) > g.k.cover.twist,
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

fn sam_on(g: &mut Game, cx: &Cx, o: CoverOpeningId) -> bool {
    if g.tab.cover_opening[o].kind == OpeningKind::Rear { return false; }
    let at = g.tab.cover_opening[o].at;
    for &sam in &cx.sams {
        let (sp, sy) = (g.ch[sam].pos, g.ch[sam].aim_yaw);
        if m::abs(v3::wrap_angle(v3::yaw_to(sp, at) - sy)) < g.k.cover.sam_deg * m::PI / 180.0 {
            let eye = cover_eye_of(g, sp);
            if cover_line(g, eye, o) { return true; }
        }
    }
    false
}

fn held_by(g: &mut Game, cx: &Cx, o: CoverOpeningId) -> bool {
    if g.tab.cover_opening[o].kind == OpeningKind::Rear { return false; }
    let (at, door) = (g.tab.cover_opening[o].at, g.tab.cover_opening[o].door);
    for &gd in &cx.squad {
        if cx.men.contains(&gd) { continue; }
        let a = match crate::seats::order_of(g, gd) { Some(a) => a, None => continue };
        let row = &g.tab.orders_assignment[a];
        let post = match row.post { Some(p) => p, None => continue };
        if v3::dist_xz(pos_of(g, gd), post) > 1.5 { continue; }
        if (door.is_some() && row.door == door) || row.watch.map_or(false, |w| v3::dist_xz(w, at) < 1.5) { return true; }
    }
    false
}

/// Weigh one opening and put it on the list if some free man can take it or sees its leaf's edge: its index in `live`. (cover.ts deal: add)
fn add(g: &mut Game, cx: &Cx, live: &mut Vec<LiveOpening>, o: CoverOpeningId, kind_k: f64) -> Option<usize> {
    let k = g.k.cover;
    let oat = g.tab.cover_opening[o].at;
    let mut at: JsMap<GuardId, V3> = JsMap::new();
    let mut can: Vec<GuardId> = Vec::new();
    let mut edge: Vec<GuardId> = Vec::new();
    for &gd in &cx.men {
        if v3::dist_xz(pos_of(g, gd), oat) <= k.in_m || runs(g, cx, gd, oat) { continue; }
        let a = *cx.asg_of.get(gd).unwrap();
        if cover_walk_only(g, gd, a) && !cover_walk_door(g, o, a) { continue; }
        let eye = cover_eye_of(g, pos_of(g, gd));
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

/// A rear opening's point has moved: the jobs on it hold that very point in the TypeScript (J.to is o.at), and it moves under them.
fn rear_moved(g: &mut Game, r: CoverOpeningId) {
    let at = g.tab.cover_opening[r].at;
    for (_, j) in g.cover.jobs.iter_mut() { if j.o == r { j.to = at; } }
}

/// The point his gun rests on along (dx, dz): the furthest of three his eye has a line to. (cover.ts deal: along)
fn along(g: &mut Game, p: V3, dx: f64, dz: f64, l: f64, far: bool) -> Option<V3> {
    let h = m::hyp2(dx, dz);
    if !(h > 1e-6) { return None; }
    let rear_m = g.k.cover.rear_m;
    for s in [1.0, 0.6, 0.35] {
        let r = if far { l * s } else { m::min(l, rear_m * s) };
        if r < 1.2 { break; }
        let q: V3 = [p[0] + dx / h * r, 0.0, p[2] + dz / h * r];
        let eye = cover_eye_of(g, p);
        if !crate::scene_collision::segment_blocked(g, eye, [q[0], CHEST, q[2]], None, None, None) { return Some(q); }
    }
    None
}

fn find_live(live: &[LiveOpening], o: CoverOpeningId) -> Option<usize> { live.iter().position(|l| l.o == o) }

/// Every dealing of `hands` men to the pool tried: the best, -1 being none. (cover.ts deal: rec)
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

/// The deal: every live opening weighed, the free men dealt one each. (cover.ts deal)
pub fn cover_deal(g: &mut Game) {
    let k = g.k.cover;
    let sams: Vec<CharId> = crate::seats::men_up(g).iter().map(|&pl| g.pl[pl].ch).collect();
    let squad = crate::seats::team_men(g);
    let mut list: Vec<(GuardId, OrdersAssignmentId)> = Vec::new();
    for &gd in &squad { if let Some(a) = cover_free(g, gd) { list.push((gd, a)); } }
    {
        let tab = &g.tab.orders_assignment;
        let key = |a: OrdersAssignmentId| if tab[a].post.is_none() { tab[a].follow } else { 100.0 };
        js::sort_by(&mut list, |a, b| key(a.1) - key(b.1));
    }
    let men: Vec<GuardId> = list.iter().map(|x| x.0).collect();
    g.cover.men = men.clone();
    let mut asg_of: JsMap<GuardId, OrdersAssignmentId> = JsMap::new();
    for &(gd, a) in &list { asg_of.set(gd, a); }
    g.cover.stats.deals += 1.0;
    if men.is_empty() { g.cover.live = Vec::new(); cover_prune_steps(g); return; }
    let file: Vec<GuardId> = list.iter().filter(|x| g.tab.orders_assignment[x.1].post.is_none()).map(|x| x.0).collect();
    let mut bodies: Vec<(CharId, V3)> = squad.iter().map(|&gd| (g.gd[gd].ch, pos_of(g, gd))).collect();
    for &sam in &sams { bodies.push((sam, g.ch[sam].pos)); }
    let mut spd: JsMap<GuardId, f64> = JsMap::new();
    let mut ways: JsMap<GuardId, Option<f64>> = JsMap::new();
    for &gd in &men { let v = g.ch[g.gd[gd].ch].vel; spd.set(gd, m::hyp2(v[0], v[2])); }
    for &gd in &men {
        let v = g.ch[g.gd[gd].ch].vel;
        ways.set(gd, if *spd.get(gd).unwrap() > 0.4 { Some(m::atan2(v[0], v[2])) } else { None });
    }
    let cx = Cx { squad, sams, men, bodies, asg_of, spd, ways };
    // ---- the live openings
    let mut live: Vec<LiveOpening> = Vec::new();
    let mut open_doors: Vec<CoverOpeningId> = Vec::new();
    for o in g.cover.openings.clone() {
        let (oat, n, door) = (g.tab.cover_opening[o].at, g.tab.cover_opening[o].n, g.tab.cover_opening[o].door);
        if near(&cx.bodies, oat) > k.reach { continue; }
        if cx.bodies.iter().any(|&(_, b)| v3::dist_xz(b, oat) < k.in_m) { continue; }   // one of ours is standing in it
        let (mut pos, mut neg) = (false, false);
        for &(_, b) in &cx.bodies {
            if v3::dist_xz(b, oat) > 2.5 { continue; }
            if (b[0] - oat[0]) * n[0] + (b[2] - oat[2]) * n[1] >= 0.0 { pos = true; } else { neg = true; }
        }
        if pos && neg { continue; }   // … or the file is going through it
        let shut = match door { Some(d) => m::abs(crate::doors::door_angle(g, d)) < 0.5, None => false };
        let l = add(g, &cx, &mut live, o, if shut { k.shut_k } else { 1.0 });
        if door.is_some() && !shut {
            let covered = match l { Some(i) => live[i].sam || live[i].held, None => sam_on(g, &cx, o) || held_by(g, &cx, o) };
            if !covered { open_doors.push(o); }
        }
    }
    // a passage looked into from outside is one opening, its nearer mouth; with one of ours inside it, its two mouths are his front and his back
    let mut keep: Vec<bool> = Vec::with_capacity(live.len());
    for i in 0..live.len() {
        let l = &live[i];
        let row = &g.tab.cover_opening[l.o];
        let (pass, far) = match (&row.pass, row.far) { (Some(p), Some(f)) => (p, f), _ => { keep.push(true); continue; } };
        let twin = (0..live.len()).find(|&j| j != i && g.tab.cover_opening[live[j].o].pass.as_ref() == Some(pass));
        let first = match twin {
            None => true,
            Some(t) => live[t].w < l.w || (live[t].w == l.w && g.tab.cover_opening[live[t].o].id > row.id),
        };
        if first { keep.push(true); continue; }
        let (oat, half) = (row.at, row.half);
        let ax = far[0] - oat[0];
        let az = far[2] - oat[2];
        let l2 = js::or(ax * ax + az * az, 1.0);
        keep.push(cx.bodies.iter().any(|&(_, b)| {
            let t = ((b[0] - oat[0]) * ax + (b[2] - oat[2]) * az) / l2;
            t > 0.0 && t < 1.0 && m::abs((b[0] - oat[0]) * az - (b[2] - oat[2]) * ax) / m::sqrt(l2) < half + 0.3
        }));
    }
    let mut at_i = 0;
    live.retain(|_| { let y = keep[at_i]; at_i += 1; y });
    // ---- the rear: the way they came, back along Sam's trail past the last man
    let mut rear_l: Option<CoverOpeningId> = None;
    let last: Option<GuardId> = file.last().copied();
    if let Some(last_m) = last {
        let lp = pos_of(g, last_m);
        let mut back: Option<V3> = None;
        let lway = cx.ways.get(last_m).copied().flatten();
        for s in [1.0, 0.6, 0.35] {
            let q = match trail_back(g, last_m, k.rear_m * s) { Some(q) => q, None => continue };
            if v3::dist_xz(q, lp) <= 1.2 { continue; }
            if let Some(w) = lway { if m::abs(v3::wrap_angle(v3::yaw_to(lp, q) - w)) < m::PI / 2.0 { continue; } }
            let eye = cover_eye_of(g, lp);
            if !crate::scene_collision::segment_blocked(g, eye, [q[0], CHEST, q[2]], None, None, None) { back = Some(q); break; }
        }
        if fast(g, &cx, last_m) { back = None; }
        if let Some(back) = back {
            let brg = v3::yaw_to(lp, back);
            let on_it = live.iter().any(|l| l.can.contains(&last_m) && m::abs(v3::wrap_angle(v3::yaw_to(lp, g.tab.cover_opening[l.o].at) - brg)) < 0.35);
            if !on_it {
                let r = g.cover.rear.unwrap();
                let bl = v3::dist_xz(back, lp);
                let row = &mut g.tab.cover_opening[r];
                row.at[0] = back[0];
                row.at[2] = back[2];
                row.n = [(back[0] - lp[0]) / bl, (back[2] - lp[2]) / bl];
                rear_moved(g, r);
                add(g, &cx, &mut live, r, k.rear_k);
            }
            // a file of three or more: its last man has the rear, the heaviest thing behind him, and he stays on the one he has unless another is clearly heavier
            let mine = g.cover.jobs.get(last_m).map(|j| j.o);
            if file.len() as f64 >= k.rear_from {
                let mut best_w = 0.0;
                for l in &live {
                    let w = l.w * (if Some(l.o) == mine { k.rear_keep } else { 1.0 });
                    let off = m::abs(v3::wrap_angle(v3::yaw_to(lp, g.tab.cover_opening[l.o].at) - brg));
                    if l.can.contains(&last_m) && off < k.rear_deg * m::PI / 180.0 && (rear_l.is_none() || w > best_w) { rear_l = Some(l.o); best_w = w; }
                }
            }
        }
    }
    // ---- a man whose eyes are spare on a mark: HIS rear is the way he came to it
    for &(gd, a) in &list {
        let (spare, post, from) = (yes(g.tab.orders_assignment[a].spare), g.tab.orders_assignment[a].post, g.tab.orders_assignment[a].from);
        let (post, from) = match (post, from) { (Some(po), Some(fr)) if spare => (po, fr), _ => continue };
        if cover_walk_only(g, gd, a) { continue; }
        let p = pos_of(g, gd);
        let adoor = g.tab.orders_assignment[a].door;
        let mut back: Option<V3> = None;
        let margin = if g.cover.reads.get(a).map_or(false, |r| r.pick.is_some()) { 0.5 } else { 0.0 };
        let read = if adoor.is_none() && v3::dist_xz(p, post) <= k.rear_read_in + margin {
            let points: Vec<V3> = cx.bodies.iter().map(|x| x.1).collect();
            cover_read_rear(g, gd, a, &list, &points)
        } else { None };
        if let Some(read) = read {
            let dx = read.at[0] - p[0];
            let dz = read.at[2] - p[2];
            back = along(g, p, dx, dz, m::min(m::hyp2(dx, dz) - (if read.way { 0.7 } else { 0.0 }), k.rear_look), true);
        }
        if back.is_none() {   // nothing read, or nothing of it his eye can rest on: the old rule
            let mut dx = from[0] - p[0];
            let mut dz = from[2] - p[2];
            let mut l = m::hyp2(dx, dz);
            if l < 1.5 {
                let d = match adoor { Some(d) if v3::dist_xz(p, post) <= 0.6 => d, _ => continue };
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
            back = along(g, p, dx, dz, l, false);
        }
        let back = match back { Some(b) => b, None => continue };
        let brg = v3::yaw_to(p, back);
        if live.iter().any(|l| l.can.contains(&gd) && m::abs(v3::wrap_angle(v3::yaw_to(p, g.tab.cover_opening[l.o].at) - brg)) < 0.35) { continue; }
        let r = match g.cover.rears.get(gd) {
            Some(&r) => r,
            None => {
                let id = alloc::format!("rear:{}", g.gd[gd].callsign);
                let r = g.tab.cover_opening.push(Opening { id, kind: OpeningKind::Rear, at: [0.0, CHEST, 0.0], n: [0.0, 1.0], half: 1.0, door: None, pass: None, far: None });
                g.cover.rears.set(gd, r);
                r
            }
        };
        let bl = js::or(v3::dist_xz(back, p), 1.0);
        let row = &mut g.tab.cover_opening[r];
        row.at[0] = back[0];
        row.at[2] = back[2];
        row.n = [(back[0] - p[0]) / bl, (back[2] - p[2]) / bl];
        rear_moved(g, r);
        if let Some(i) = add(g, &cx, &mut live, r, k.rear_k) {
            live[i].can.retain(|&m_| m_ == gd);
            if live[i].can.is_empty() { live.remove(i); }
        }
    }
    // ---- the rear man of a stack with an open way at its back: that way is his, ahead of every dealing below
    let mut pins: Vec<(GuardId, CoverOpeningId)> = Vec::new();
    for &(gd, _) in &list {
        let pin = match rear_pin(g, gd) { Some(p) => p, None => continue };
        let o = pin.o;
        let j0 = g.cover.jobs.get(gd).copied();
        let post = g.tab.orders_assignment[pin.a].post;
        let p = pos_of(g, gd);
        let mut to = crate::rearwatch::rear_sees(g, p, o, pin.front);
        if to.is_none() { if let Some(j) = j0 { if !j.off && j.o == o { to = Some(j.to); } } }
        if to.is_none() {
            if let Some(po) = post {
                if m::hyp2(p[0] - po[0], p[2] - po[2]) < 1.0 { to = crate::rearwatch::rear_sees(g, po, o, pin.front); }
            }
        }
        let to = match to.or(pin.to) { Some(t) => t, None => continue };
        let i = match find_live(&live, o) {
            Some(i) => i,
            None => {
                let oat = g.tab.cover_opening[o].at;
                let w = 1.0 / (1.0 + near(&cx.bodies, oat) / k.near_m);
                let sam = sam_on(g, &cx, o);
                live.push(LiveOpening { o, w, can: Vec::new(), edge: Vec::new(), at: JsMap::new(), by: None, edge_by: None, sam, held: false });
                live.len() - 1
            }
        };
        if !live[i].can.contains(&gd) { live[i].can.push(gd); }
        live[i].edge.retain(|&m_| m_ != gd);
        live[i].at.set(gd, to);
        pins.push((gd, o));
    }
    js::sort_by(&mut live, |a, b| b.w - a.w);
    deal_out(g, &cx, &list, live, open_doors, rear_l, last, pins);
}

/// The second half of the deal: who gets which, and the jobs handed out. (cover.ts deal)
fn deal_out(g: &mut Game, cx: &Cx, list: &[(GuardId, OrdersAssignmentId)], mut live: Vec<LiveOpening>, open_doors: Vec<CoverOpeningId>, rear_l: Option<CoverOpeningId>, last: Option<GuardId>, pins: Vec<(GuardId, CoverOpeningId)>) {
    let k = g.k.cover;
    // (`live` no longer changes shape from here: a row's index is its handle)
    let mut got: JsMap<GuardId, usize> = JsMap::new();
    if let (Some(ro), Some(last_m)) = (rear_l, last) {
        if !(crate::rearwatch::room_kept(g, last_m) && pins.iter().any(|x| x.0 == last_m)) {
            let i = find_live(&live, ro).unwrap();
            got.set(last_m, i);
            live[i].by = Some(last_m);
        }
    }
    for &(gd, o) in &pins {
        let i = find_live(&live, o).unwrap();
        if live[i].by.is_none() && !got.has(gd) { got.set(gd, i); live[i].by = Some(gd); }
    }
    // (a man keeps what he was just dealt: COVER.keepSecs of it, while it is still live and his to take)
    for &gd in &cx.men {
        let j = match g.cover.jobs.get(gd) { Some(j) => *j, None => continue };
        if got.has(gd) || g.time - j.t0 >= k.keep_secs { continue; }
        if let Some(i) = live.iter().position(|x| x.o == j.o && x.by.is_none() && x.can.contains(&gd)) { got.set(gd, i); live[i].by = Some(gd); }
    }
    let hands: Vec<GuardId> = cx.men.iter().copied().filter(|&gd| !got.has(gd)).collect();
    let mut pool: Vec<usize> = (0..live.len()).filter(|&i| live[i].by.is_none() && !live[i].can.is_empty()).collect();
    pool.truncate(8);
    let mut score: Vec<Vec<f64>> = Vec::with_capacity(hands.len());
    for &gd in &hands {
        let p = pos_of(g, gd);
        let job_o = g.cover.jobs.get(gd).map(|j| j.o);
        let mut row: Vec<f64> = Vec::with_capacity(pool.len());
        for &i in &pool {
            if !live[i].can.contains(&gd) { row.push(0.0); continue; }
            let o = live[i].o;
            let oat = g.tab.cover_opening[o].at;
            let turn = m::abs(v3::wrap_angle(v3::yaw_to(p, oat) - g.ch[g.gd[gd].ch].aim_yaw));
            let behind = Some(gd) != last && behind_of(g, cx, gd, oat);
            let lane = crate::gunroles::gun_lane(g, gd, v3::dist_xz(p, oat), k.reach);
            row.push(m::max(0.05, 10.0 * live[i].w * (if behind { k.behind_k } else { 1.0 }) - 0.25 * v3::dist_xz(p, oat) / k.reach - 0.35 * turn / m::PI + (if job_o == Some(o) { k.sticky } else { 0.0 }) + lane));
        }
        score.push(row);
    }
    let mut z = Search { score: &score, used: vec![false; pool.len()], cur: vec![-1; hands.len()], best: vec![-1; hands.len()], bs: -m::INF };
    rec(&mut z, 0, 0.0);
    let best = z.best;
    for (i, &gd) in hands.iter().enumerate() {
        let j = best[i];
        if j >= 0 { let li = pool[j as usize]; got.set(gd, li); live[li].by = Some(gd); }
    }
    // (a man left with nothing: the leaf's edge of the heaviest open door he sees no further into)
    for &gd in &cx.men {
        if got.has(gd) { continue; }
        if let Some(i) = live.iter().position(|x| x.by.is_none() && x.edge_by.is_none() && x.edge.contains(&gd)) { got.set(gd, i); live[i].edge_by = Some(gd); }
    }
    // ---- hand the jobs out
    for &gd in &cx.men {
        let li = got.get(gd).copied();
        let j = g.cover.jobs.get(gd).copied();
        let li = match li {
            Some(i) => i,
            None => { if j.is_some() { cover_drop(g, gd, None); } continue; }
        };
        let o = live[li].o;
        let oat = g.tab.cover_opening[o].at;
        let to = live[li].at.get(gd).copied().unwrap_or(oat);
        if let Some(jj) = j {
            if jj.o == o { g.cover.jobs.get_mut(gd).unwrap().to = to; continue; }
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
        if j.is_some() { g.cover.stats.swaps += 1.0; } else { g.cover.stats.takes += 1.0; }
        if g.tab.cover_opening[o].kind == OpeningKind::Rear { g.cover.stats.rear += 1.0; }
        let id = g.tab.cover_opening[o].id.clone();
        cover_note(g, gd, Some(&id));
    }
    g.cover.live = live.iter().filter(|l| !l.can.is_empty()).cloned().collect();
    let uncovered = g.cover.live.iter().filter(|l| l.by.is_none() && !l.sam && !l.held).count();
    g.cover.stats.uncovered += uncovered as f64;
    let mut got_rows: JsMap<GuardId, LiveOpening> = JsMap::new();
    for (gd, &i) in got.iter() { got_rows.set(gd, live[i].clone()); }
    let bodies = &cx.bodies;
    let near_m = k.near_m;
    let mut w_of = |g: &mut Game, o: CoverOpeningId| 1.0 / (1.0 + near(bodies, g.tab.cover_opening[o].at) / near_m);
    cover_step_deal(g, list, &cx.asg_of, &got_rows, &live, &open_doors, bodies, &cx.spd, &mut w_of, if rear_l.is_some() { last } else { None });
}

/// Step to see: every open door nobody has a line into gets one man. (cover.ts stepDeal)
pub fn cover_step_deal(g: &mut Game, list: &[(GuardId, OrdersAssignmentId)], asg_of: &JsMap<GuardId, OrdersAssignmentId>, got: &JsMap<GuardId, LiveOpening>, live: &[LiveOpening], open: &[CoverOpeningId], bodies: &[(CharId, V3)], spd: &JsMap<GuardId, f64>, w_of: &mut dyn FnMut(&mut Game, CoverOpeningId) -> f64, rear_man: Option<GuardId>) {
    let k = g.k.cover;
    let t = g.time;
    cover_prune_steps(g);
    let mut ready: Vec<GuardId> = Vec::new();
    for &(gd, a) in list {
        let p = pos_of(g, gd);
        let l = got.get(gd);
        let still = *spd.get(gd).unwrap() < 0.3;
        if !still || !g.cover.stood.has(gd) { g.cover.stood.set(gd, t); }
        if let Some(l) = l {
            if l.edge_by == Some(gd) && g.tab.cover_opening[l.o].door.is_some() { g.cover.stats.blind += 1.0; }
        }
        let s = g.cover.steps.get(gd).copied();
        if let Some(sv) = s {
            if !sv.done {
                if v3::dist_xz(p, sv.q) < 0.35 {
                    let row = g.cover.steps.get_mut(gd).unwrap();
                    row.done = true;
                    row.t = t;
                    g.cover.stats.stepped += 1.0;
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
                    g.cover.stats.step_saw += 1.0;
                }
            }
        }
        // (his order lets his feet go: under 'fall in' while Sam stands, or holding a spot with nothing named)
        let (post, goal, t0) = (g.tab.orders_assignment[a].post, g.tab.orders_assignment[a].goal, g.tab.orders_assignment[a].t0);
        let may_step = if post.is_none() { sam_stopped(g, gd) } else {
            let row = &g.tab.orders_assignment[a];
            row.order == "hold" && row.watch.is_none() && !yes(row.spare) && row.via.is_empty()
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
        let in_reach = |g: &Game, gd: GuardId| v3::dist_xz(pos_of(g, gd), oat) <= reach;
        if seen.is_empty() && list.iter().any(|&(gd, _)| in_reach(g, gd)) { g.cover.stats.unseen += 1.0; }
        if t < g.cover.door_try.get(o).copied().unwrap_or(0.0) { continue; }
        let theirs = g.cover.steps.values().any(|s| s.o == o && (!s.done || s.saw || t - s.t < 3.0));
        if theirs { continue; }   // (one man a door)
        let idle: Vec<GuardId> = ready.iter().copied().filter(|&gd| in_reach(g, gd) && !seen.contains(&gd) && got.get(gd).map_or(true, |l| l.edge_by == Some(gd))).collect();
        let busy: Vec<GuardId> = if !seen.is_empty() { Vec::new() } else {
            ready.iter().copied().filter(|&gd| in_reach(g, gd) && got.get(gd).map_or(false, |l| l.by == Some(gd) && 10.0 * (wo - l.w) > k.sticky + 0.3)).collect()
        };
        let mut pick: Option<(GuardId, V3, f64, V3)> = None;
        for tier in [&idle, &busy] {
            if pick.is_some() { break; }
            for &gd in tier.iter() {
                let a = *asg_of.get(gd).unwrap();
                let p = pos_of(g, gd);
                let from: V3 = match (g.tab.orders_assignment[a].post, g.cover.steps.get(gd)) {
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
        g.cover.stats.steps += 1.0;
        let i = ready.iter().position(|&x| x == pgd).unwrap();
        ready.remove(i);
    }
}

/// The steps that no longer stand. (cover.ts pruneSteps)
pub fn cover_prune_steps(g: &mut Game) {
    for gd in g.cover.steps.keys() {
        let sa = g.cover.steps.get(gd).unwrap().a;
        let a = crate::seats::order_of(g, gd);
        let gone = g.gd[gd].state == crate::game::GuardState::Dead || a != Some(sa) || (g.tab.orders_assignment[sa].post.is_none() && !sam_stopped(g, gd));
        if gone { g.cover.steps.delete(gd); }
    }
}

/// Is `x` in the lane a → b, within `lane` of the line. (cover.ts stepSpot: inLane)
fn in_lane(x: V3, a: V3, b: V3, lane: f64) -> bool {
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    let len = js::or(m::hyp2(dx, dz), 1.0);
    let sx = x[0] - a[0];
    let sz = x[2] - a[2];
    let along = (sx * dx + sz * dz) / len;
    along > 0.15 && along < len && m::abs(sx * dz - sz * dx) / len < lane
}

/// The spot a man steps to to see through open door `o`. (cover.ts stepSpot)
pub fn cover_step_spot(g: &mut Game, gd: GuardId, from: V3, o: CoverOpeningId, bodies: &[(CharId, V3)]) -> Option<CoverStepSpot> {
    let k = g.k.cover;
    let me = g.gd[gd].ch;
    let p = pos_of(g, gd);
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
        if mt != gd && !j.off { guns.push((g.ch[g.gd[mt].ch].pos, j.to)); }
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
            if crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [q[0], 0.45, q[2]], None) || crate::scene_collision::solid_blocked(g, [p[0], CHEST, p[2]], [q[0], CHEST, q[2]], None) { continue; }
            if [0.35, 0.7].iter().any(|&f: &f64| {
                let mx = p[0] + (q[0] - p[0]) * f;
                let mz = p[2] + (q[2] - p[2]) * f;
                ops.iter().any(|x| m::hyp2(mx - x.0[0], mz - x.0[2]) < x.1 + 0.3)
            }) { continue; }   // (… nor does the walk there cut across one)
            let eye = cover_eye_of(g, q);
            if cover_sight(g, eye, o) != Some(oat) || crate::scene_collision::segment_blocked(g, eye, beyond, None, None, None) { continue; }
            if bodies.iter().any(|&(bc, b)| bc != me && in_lane(b, q, oat, k.lane)) || guns.iter().any(|&(ga, gb)| in_lane(q, ga, gb, k.lane)) { continue; }
            let len = js::or(v3::dist_xz(q, oat), 1.0);
            let c = step + 0.6 * m::abs(side(q)) / len + 0.5 * m::max(0.0, d0 - len);
            if c < bc { bc = c; best = Some(q); }
        }
    }
    best.map(|q| CoverStepSpot { q, c: bc })
}

/// Where the cover has this man step to under the order he has now. (cover.ts stepOf)
pub fn cover_step_of(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) -> Option<V3> {
    match g.cover.steps.get(gd) { Some(s) if s.a == a => Some(s.q), _ => None }
}

/// He cannot get to it: where he stands is where the step ends. (cover.ts stepHalt)
pub fn cover_step_halt(g: &mut Game, gd: GuardId) {
    let p = pos_of(g, gd);
    let t = g.time;
    let s = match g.cover.steps.get_mut(gd) { Some(s) => s, None => return };
    s.q = [p[0], 0.0, p[2]];
    if !s.done { s.done = true; s.t = t; g.cover.no_step.set(gd, t + 4.0); }
}

/// A change of hands, for the probes' record: the newest 400. (cover.ts note)
pub fn cover_note(g: &mut Game, gd: GuardId, id_2: Option<&str>) {
    let (t, who) = (g.time, g.gd[gd].callsign.clone());
    let log = &mut g.cover.log;
    log.push(CoverLogItem { t, who, id: id_2.map(String::from) });
    if log.len() > 400 { let extra = log.len() - 400; log.drain(0..extra); }
}

/// His job goes; `taken`: something else has him now and may not re-pose him. (cover.ts drop)
pub fn cover_drop(g: &mut Game, gd: GuardId, taken: Option<bool>) {
    let taken = taken.unwrap_or(false);
    let j = match g.cover.jobs.get(gd).copied() { Some(j) => j, None => return };
    g.cover.jobs.delete(gd);
    cover_note(g, gd, None);
    if !taken && !j.off && script_aim_is(g, gd, j.aim) {
        let a = crate::seats::order_of(g, gd).unwrap_or(j.a);
        let row = &g.tab.orders_assignment[a];
        if yes(row.spare) && row.post.is_some() && row.from.is_some() {
            let t = g.time;
            g.cover.kept.set(gd, CoverKeptValue { j, t });
            g.cover.stats.kept += 1.0;
            return;
        }
    }
    if !taken { cover_turn_from(g, gd, j); }
    if taken { cover_release(g, gd, j); }
}

/// What the cover wrote into his script goes with the job. (cover.ts release)
pub fn cover_release(g: &mut Game, gd: GuardId, j: CoverJob) {
    if g.gd[gd].script.is_none() || g.gd[gd].state == crate::game::GuardState::Dead { return; }
    let s = g.gd[gd].script.as_mut().unwrap();
    if j.back && yes(s.strafe) { s.strafe = Some(false); }
    if script_aim_is(g, gd, j.aim) { give_aim_back(g, gd, j.a); }
}

/// `s.aimAt = A.watch`, and the gun-up carriage if nobody has changed it since. (cover.ts release, unturn)
fn give_aim_back(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) {
    let (watch, upper, stance) = (g.tab.orders_assignment[a].watch, g.tab.orders_assignment[a].upper, g.tab.orders_assignment[a].stance);
    let s = g.gd[gd].script.as_mut().unwrap();
    s.aim_at = watch;
    if s.upper == Some(crate::character::UpperMode::Aim) && s.stance == Some(crate::character::Stance::None) {
        s.upper = Some(upper);
        s.stance = Some(stance);
    }
}

/// The deal has let go of a man walking backwards on his opening: from here his feet are turnOut's. (cover.ts turnFrom)
pub fn cover_turn_from(g: &mut Game, gd: GuardId, j: CoverJob) {
    let strafing = g.gd[gd].script.as_ref().map_or(false, |s| yes(s.strafe));
    if j.back && strafing && g.gd[gd].state != crate::game::GuardState::Dead {
        let t = g.time;
        g.cover.outs.set(gd, CoverOutsValue { j, aim: j.aim, t0: t, t, on: false });
    }
}

/// The way he goes: at the corner of his path he is walking to, else the way he is moving. (cover.ts wayOf)
pub fn cover_way_of(g: &mut Game, gd: GuardId) -> f64 {
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    if let Some(&wp) = g.gd[gd].path.get(g.gd[gd].path_i) {
        if v3::dist_xz(p, wp) > 0.3 { return v3::yaw_to(p, wp); }
    }
    g.cover.heading.get(gd).copied().unwrap_or(g.ch[c].body_yaw)
}

/// Is he still coming out of a back-pedal. (cover.ts turning)
pub fn cover_turning(g: &mut Game, gd: GuardId, still: bool, pace: f64) -> bool {
    let o = match g.cover.outs.get(gd) { Some(o) => *o, None => return false };
    let k = g.k.cover;
    let t = g.time;
    !still && pace <= k.back_out && m::abs(t - o.t) <= 0.1 && m::abs(t - o.t0) < k.back_turn_secs && m::cos(g.ch[g.gd[gd].ch].body_yaw - cover_way_of(g, gd)) < k.back_turn_body
}

/// His pose while he is: the path's own feet, the body turning to the way he goes, the gun carried round in front of it. (cover.ts turnOut)
pub fn cover_turn_out(g: &mut Game, gd: GuardId, still: bool, pace: f64) -> Option<CoverTurnOut> {
    let (was_aim, was_on) = { let o = g.cover.outs.get(gd)?; (o.aim, o.on) };
    if !cover_turning(g, gd, still, pace) { g.cover.outs.delete(gd); return None; }
    let ours = was_on && script_aim_is(g, gd, was_aim);   // (`on`: until his first pose the point is a copy of J.aim the script does not hold, however equal)
    let c = g.gd[gd].ch;
    let (pos, by) = (g.ch[c].pos, g.ch[c].body_yaw);
    let t = g.time;
    let o = g.cover.outs.get_mut(gd).unwrap();
    let first = !o.on;
    o.on = true;
    o.t = t;
    o.aim[0] = pos[0] + m::sin(by) * 3.0;
    o.aim[2] = pos[2] + m::cos(by) * 3.0;
    let aim = o.aim;
    if first { g.cover.stats.turned += 1.0; }
    if ours { g.gd[gd].script.as_mut().unwrap().aim_at = Some(aim); }   // (the script holds that very point in the TypeScript)
    let face = cover_way_of(g, gd);
    Some(CoverTurnOut { at: aim, face: Some(face), strafe: true })
}

/// Something else has him: what the turn-out wrote into his script goes. (cover.ts unturn)
pub fn cover_unturn(g: &mut Game, gd: GuardId) {
    let o = match g.cover.outs.delete(gd) { Some(o) => o, None => return };
    if g.gd[gd].script.is_none() || g.gd[gd].state == crate::game::GuardState::Dead { return; }
    let s = g.gd[gd].script.as_mut().unwrap();
    if yes(s.strafe) { s.strafe = Some(false); }
    if o.on && script_aim_is(g, gd, o.aim) { give_aim_back(g, gd, o.j.a); }
}

/// The gun comes off an opening the deal let go of. (cover.ts unkeep)
pub fn cover_unkeep(g: &mut Game, gd: GuardId, taken: Option<bool>) {
    let k = match g.cover.kept.delete(gd) { Some(k) => k, None => return };
    if taken.unwrap_or(false) { cover_release(g, gd, k.j); } else { cover_turn_from(g, gd, k.j); }
}

/// The opening the deal last took off this man, while his gun is still on it. (cover.ts keptOf)
pub fn cover_kept_of(g: &mut Game, gd: GuardId) -> Option<CoverJob> {
    let kv = *g.cover.kept.get(gd)?;
    let p = pos_of(g, gd);
    if m::abs(g.time - kv.t) >= g.k.rear.secs || !script_aim_is(g, gd, kv.j.aim) { return None; }
    let a = crate::seats::order_of(g, gd)?;
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
        if ours(g.ch[g.pl[pl].ch].pos) { return None; }
    }
    for mt in crate::seats::team_men(g) {
        if mt != gd && g.gd[mt].state != crate::game::GuardState::Dead && ours(pos_of(g, mt)) { return None; }
    }
    Some(j)
}

/// Everything goes: the knob, or no team. (cover.ts dropAll)
pub fn cover_drop_all(g: &mut Game) {
    for gd in g.cover.jobs.keys() { cover_drop(g, gd, Some(true)); }
    for gd in g.cover.kept.keys() { cover_unkeep(g, gd, Some(true)); }
    for gd in g.cover.outs.keys() { cover_unturn(g, gd); }
}

/// The job back where it lives: dealt, or kept.
fn put_job(g: &mut Game, gd: GuardId, j: CoverJob) {
    if let Some(x) = g.cover.jobs.get_mut(gd) { *x = j; }
    else if let Some(x) = g.cover.kept.get_mut(gd) { x.j = j; }
}

/// Where his gun goes and how he carries himself for the opening he was dealt: None, the order's own pose stands. (cover.ts poseFor)
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
    if ours { g.gd[gd].script.as_mut().unwrap().aim_at = Some(j.aim); }   // (the script holds that very point in the TypeScript)
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

/// An opening in a few words, as this man would say it. (cover.ts nameOf)
pub fn cover_name_of(g: &mut Game, o: CoverOpeningId, from: Option<V3>) -> String {
    let (kind, door, at) = (g.tab.cover_opening[o].kind, g.tab.cover_opening[o].door, g.tab.cover_opening[o].at);
    if kind == OpeningKind::Rear { return String::from("the rear"); }
    if let Some(d) = door { return alloc::format!("the {}", crate::orders::door_words(g, d)); }
    let from = match from { Some(f) => f, None => return alloc::format!("{} ({:.1}, {:.1})", kind.as_str(), at[0], at[2]) };
    let what = if kind == OpeningKind::Mouth { "aisle" } else { "gap" };
    let yaw = v3::yaw_to(from, at);
    match crate::dialogue::watched_place(g, from, [at[0] + m::sin(yaw), 0.0, at[2] + m::cos(yaw)]) {
        Some(p) if !p.is_empty() => alloc::format!("the {} to {}", what, p),
        _ => alloc::format!("the {}", what),
    }
}

/// Has he an opening dealt, and is on it. (cover.ts has)
pub fn cover_has(g: &mut Game, gd: GuardId) -> bool { cover_on_of(g, gd).map_or(false, |j| !j.off) }

/// The opening his gun is on: dealt, or let go of and still his for the moment. (cover.ts onOf)
pub fn cover_on_of(g: &mut Game, gd: GuardId) -> Option<CoverJob> {
    match g.cover.jobs.get(gd) { Some(j) => Some(*j), None => g.cover.kept.get(gd).map(|k| k.j) }
}

/// What he is on, in words. (cover.ts words)
pub fn cover_words(g: &mut Game, gd: GuardId) -> String {
    match cover_on_of(g, gd) {
        Some(j) if !j.off => { let p = pos_of(g, gd); alloc::format!("on {}", cover_name_of(g, j.o, Some(p))) }
        _ => String::new(),
    }
}

/// The tail of his row in the squad block. (cover.ts tail)
pub fn cover_tail(g: &mut Game, gd: GuardId) -> String {
    let w = cover_words(g, gd);
    if w.is_empty() { w } else { alloc::format!(" · {}", w) }
}

/// Is he facing what he was dealt: his gun within `deg` of it. (cover.ts facing)
pub fn cover_facing(g: &mut Game, gd: GuardId, deg: Option<f64>) -> bool {
    let deg = deg.unwrap_or(15.0);
    match cover_on_of(g, gd) {
        Some(j) if !j.off => m::abs(v3::wrap_angle(v3::yaw_to(pos_of(g, gd), j.to) - g.ch[g.gd[gd].ch].aim_yaw)) < deg * m::PI / 180.0,
        _ => false,
    }
}

/// One line for the console. (cover.ts summary)
pub fn cover_summary(g: &mut Game) -> String {
    if g.cover.men.is_empty() { return alloc::format!("cover: nobody free · {} openings on the level", g.cover.openings.len()); }
    let mut on: Vec<String> = Vec::new();
    for gd in g.cover.men.clone() {
        let name = match g.cover.jobs.get(gd).map(|j| j.o) { Some(o) => cover_name_of(g, o, None), None => String::from("—") };
        on.push(alloc::format!("{} {}", g.gd[gd].callsign, name));
    }
    let mut open: Vec<String> = Vec::new();
    let rows: Vec<(CoverOpeningId, bool, bool)> = g.cover.live.iter().filter(|l| l.by.is_none()).map(|l| (l.o, l.sam, l.held)).collect();
    for (o, sam, held) in rows {
        open.push(alloc::format!("{}{}", cover_name_of(g, o, None), if sam { " (player)" } else if held { " (posted)" } else { "" }));
    }
    let tail = if open.is_empty() { String::new() } else { alloc::format!(" · nobody on: {}", open.join(", ")) };
    alloc::format!("cover: {} · {} live{} · {} takes, {} swaps", on.join(" · "), g.cover.live.len(), tail, g.cover.stats.takes, g.cover.stats.swaps)
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
