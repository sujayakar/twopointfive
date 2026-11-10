use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};


macro_rules! dop { ($g:ident, $op:expr) => { $g.tab.doortools_door_op[$op] }; }
macro_rules! asg { ($g:ident, $a:expr) => { $g.tab.orders_assignment[$a] }; }

pub(super) fn frame(g: &Game, d: DoorId) -> V3 { g.dr[d].frame_centre }
fn eye_of(g: &Game, m: GuardId) -> V3 {
    let c = g.gd[m].ch;
    g.ch[c].bone(crate::character::Bone::Head).unwrap_or([g.ch[c].pos[0], 1.6, g.ch[c].pos[2]])
}

/// the men of the orders let go on this order's word
fn word_ops(g: &Game, seat: &crate::seats::SeatRef, op: DoorOpId) -> Vec<DoorOpId> {
    match dop!(g, op).word {
        Some(w) => seat.door_tools.ops.iter().copied().filter(|o| dop!(g, *o).word == Some(w)).collect(),
        None => Vec::new(),
    }
}
fn gate_ops(g: &Game, op: DoorOpId) -> Vec<DoorOpId> {
    match dop!(g, op).gate { Some(w) => g.tab.doortools_door_gate[w].ops.clone(), None => Vec::new() }
}
fn on_entry(g: &Game, op: DoorOpId, word: &[DoorOpId], gate: &[DoorOpId], x: GuardId) -> bool {
    dop!(g, op).men.contains(&x) || word.iter().any(|o| dop!(g, *o).men.contains(&x)) || gate.iter().any(|o| dop!(g, *o).men.contains(&x))
}
fn near_door(g: &Game, d: DoorId, side: f64, q: V3) -> bool {
    off_of(g, d, q, side) > 0.0 && v3::dist_xz(q, frame(g, d)) < 5.0
}

/// Behind the bang the first man closes up to the edge of the frame.
pub(super) fn door_tools_close_up(g: &mut crate::game::Game, op: DoorOpId, gd: Option<GuardId>) {
    let gd = gd.unwrap_or(dop!(g, op).lead);
    let d = dop!(g, op).door;
    let side = dop!(g, op).side;
    let w = g.dr[d].def.width;
    let p = match dop!(g, op).post.get(gd) { Some(p) => *p, None => return };
    if !dop!(g, op).men.contains(&gd) { return; }
    let a = if along_of(g, d, p.pos) > w / 2.0 { w + g.k.doortools.edge_out } else { -g.k.doortools.edge_out };
    let off = off_of(g, d, p.pos, side);
    let q = door_pt(g, d, a, off, side, None);
    if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || crate::scene_collision::low_blocked(g, p.pos, q) { return; }
    let fc = frame(g, d);
    let name = crate::orders::door_words(g, d);
    door_tools_assign(g, op, gd, &DoorToolsAssignA { post: Some(q), face: Some(p.face), watch: Some([fc[0], 1.05, fc[2]]), stance: Some(crate::character::Stance::Stack), upper: Some(crate::character::UpperMode::None), stack_side: Some(p.stack_side), going: format!("closing up · {}", name).into(), there: format!("on the frame · {}", name).into(), ..Default::default() });
}

/// The lead back onto his own jamb, after his boot.
pub(super) fn door_tools_back_to_jamb(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) {
    let p = match dop!(g, op).post.get(gd) { Some(p) => *p, None => return };
    let d = dop!(g, op).door;
    let fc = frame(g, d);
    let name = crate::orders::door_words(g, d);
    door_tools_assign(g, op, gd, &DoorToolsAssignA { post: Some(p.pos), face: Some(p.face), watch: Some([fc[0], 1.05, fc[2]]), stance: Some(crate::character::Stance::Stack), upper: Some(crate::character::UpperMode::None), stack_side: Some(p.stack_side), going: format!("stacking · {}", name).into(), there: crate::orders::Doing::new(crate::orders::At::Stacked, format!("stacked · {}", name)), ..Default::default() });
}

/// Unlatched and shoved hard into the room from the jamb.
pub(super) fn door_tools_shove(g: &mut crate::game::Game, op: DoorOpId) {
    let d = dop!(g, op).door;
    let side = dop!(g, op).side;
    let lead = dop!(g, op).lead;
    let c = g.gd[lead].ch;
    let w = g.dr[d].def.width;
    let at = door_pt(g, d, w / 2.0, 0.6, side, None);
    let s = -crate::doors::doors_side(g, d, at);
    g.dr[d].leaf.latched = false;
    g.dr[d].leaf.closing = false;
    let vel = s * m::max(m::abs(g.dr[d].leaf.vel), g.k.doortools.shove);
    g.dr[d].leaf.vel = vel;
    g.dr[d].leaf.quiet_t = -1.0;
    g.dr[d].leaf.creak_armed = false;
    let id = g.ch[c].id;
    g.dr[d].leaf.last_who = id;
    let dp = crate::doors::door_pos(g, d);
    if crate::silent::silent_ease(g, lead) { crate::audio_iface::game_audio_like_play(g, "doorCreak", Some(dp), Some(0.5), None); } else { crate::audio_iface::game_audio_like_play(g, "doorBash", Some(dp), Some(0.5), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.15 })); }
    door_tools_beat_is(g, op, crate::doortools::Beat::Shove, "shove");
}

/// The leaf is moving or stood open: what comes next, by the order.
pub(super) fn door_tools_opened(g: &mut crate::game::Game, op: DoorOpId) {
    let t = g.time;
    if dop!(g, op).open_t.is_nan() { dop!(g, op).open_t = t; }
    dop!(g, op).t0 = t;
    match dop!(g, op).kind {
        DoorToolKind::OpenClear => door_tools_enter(g, op),
        DoorToolKind::BangClear => dop!(g, op).phase = Phase::Toss,
        DoorToolKind::Threshold => door_tools_to_threshold(g, op),
        _ => {}
    }
}

/// Where the canister is thrown to.
pub(super) fn door_tools_land_at(g: &crate::game::Game, op: DoorOpId, deep: Option<f64>) -> V3 {
    let deep = deep.unwrap_or(g.k.doortools.bang_in);
    let (d, side) = (dop!(g, op).door, dop!(g, op).side);
    door_tools_land_for(g, d, side, deep)
}

/// Where a bang and clear's can is thrown to from `from`; None: no point worth it.
pub(super) fn door_tools_bang_spot(g: &mut crate::game::Game, op: DoorOpId, from: V3) -> Option<crate::doortools::DoorToolsBangSpot> {
    let (d, side) = (dop!(g, op).door, dop!(g, op).side);
    let w = g.dr[d].def.width;
    let far = -side;
    let inn = door_pt(g, d, w / 2.0, 0.6, far, Some(1.9));
    let mid = door_pt(g, d, w / 2.0, 0.9, far, None);
    let room = crate::roomfight::room_fight_room_at(g, mid);
    let (into_deep, into_wide) = (g.k.doortools.bang_into_deep, g.k.doortools.bang_into_wide);
    let mut floor: Vec<V3> = Vec::new();
    let mut spots: Vec<V3> = Vec::new();
    let mut ib = 0i32;
    let mut b = 0.6;
    while b <= into_deep + 1e-6 {
        let mut ia = 0i32;
        let mut a = -into_wide;
        while a <= into_wide + 1e-6 {
            let q = door_pt(g, d, w / 2.0 + a, b, far, None);
            if !(crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || crate::scene_collision::solid_blocked(g, inn, [q[0], 1.9, q[2]], None)) {
                if (ia + ib) & 1 == 0 { floor.push(q); }
                if ia & 1 != 0 && (room.is_none() || crate::roomfight::room_fight_room_at(g, q) == room) { spots.push(q); }
            }
            a += 0.6;
            ia += 1;
        }
        b += 0.6;
        ib += 1;
    }
    if floor.len() < 6 { return None; }
    // ours in a flash: the men this can would blind, not the ones bang() spares
    let word = word_ops(g, &crate::seats::here(g), op);
    let gate = gate_ops(g, op);
    let sams = crate::orders::squad_orders_sams(&crate::seats::here(g));
    let mut ours: Vec<V3> = Vec::new();
    for x in g.guards.clone() {
        let mine = if sams { g.team(x).ours() } else { crate::orders::squad_orders_mine(g, &crate::seats::here(g), x) };
        if !mine || crate::condition::down(g, g.gd[x].ch) { continue; }
        let at = g.pos_of(x);
        if on_entry(g, op, &word, &gate, x) || near_door(g, d, side, at) { continue; }
        ours.push(at);
    }
    let pl = g.player;
    let sam_at = g.player_pos(pl);
    if !(crate::condition::down(g, g.pl[pl].ch) || (sams && near_door(g, d, side, sam_at))) { ours.push(crate::orders::squad_orders_sam_stood(g, &crate::seats::here(g))); }
    let (into_min, throw_m, air_h, flash_m, lit) = (g.k.doortools.bang_into_min, g.k.doortools.thr_throw_m, g.k.doortools.air_h, g.k.doortools.thr_flash_m, g.k.doortools.bang_into_lit);
    // (at, n, sc, deep)
    let mut cands: Vec<(V3, f64, f64, f64)> = Vec::new();
    for p in spots.iter().copied() {
        if -off_of(g, d, p, side) < into_min || v3::dist_xz(from, p) > throw_m { continue; }
        let at: V3 = [p[0], air_h - 0.05, p[2]];
        if crate::scene_collision::solid_blocked(g, from, at, None) { continue; }
        let mut ours_in = false;
        for q in ours.iter().copied() {
            if v3::dist_xz(q, p) <= flash_m && !crate::scene_collision::segment_blocked(g, at, [q[0], 1.45, q[2]], None, None, None) { ours_in = true; break; }
        }
        if ours_in { continue; }
        let (mut n, mut sc) = (0.0, 0.0);
        for q in floor.iter().copied() {
            let r = v3::dist_xz(p, q);
            if r <= flash_m && !crate::scene_collision::segment_blocked(g, at, [q[0], 1.45, q[2]], None, None, None) { n += 1.0; sc += 1.0 - r / flash_m; }
        }
        let deep = m::round(-off_of(g, d, at, side) * 10.0);
        cands.push((at, n, sc, deep));
    }
    let mut most = 0.0;
    for c in cands.iter() { most = m::max(most, c.1); }
    let mut good: Vec<(V3, f64, f64, f64)> = cands.iter().copied().filter(|c| c.1 >= most * lit).collect();
    let of = floor.len() as f64;
    if room.is_none() && most < of * 0.5 { return None; }
    js::sort_by(&mut good, |a, b| js::or(b.3 - a.3, b.2 - a.2));
    for c in good.iter().take(8) {
        let sol = crate::throwables::throwables_solve(g, from, c.0, Some(9.0));
        if sol.points.iter().any(|q| v3::dist_xz(*q, c.0) < 0.35 && m::abs(q[1] - c.0[1]) < 0.6) {
            return Some(crate::doortools::DoorToolsBangSpot { at: c.0, n: c.1, of });
        }
    }
    None
}

/// The breach plan's own landing, when this door has one: doorPlan's `toss`, which only a level's BreachPlan carries.
fn plan_toss(g: &Game, name: &str) -> Option<(V3, V3)> {
    if let Some(plans) = g.level.clear_plans.as_ref() {
        if plans.iter().any(|p| p.door == name) { return None; }
    }
    match g.level.breach.as_ref() {
        Some(b) if b.door == name => Some((b.stack[0], b.toss.land)),
        _ => None,
    }
}

/// How far in the floor runs clear from a door, for a can's landing.
pub(super) fn door_tools_land_for(g: &crate::game::Game, d: DoorId, side: f64, deep: f64) -> V3 {
    let w = g.dr[d].def.width;
    let mut best = door_pt(g, d, w / 2.0, 0.6, -side, None);
    let mut prev = best;
    let name = g.dr[d].def.name.clone();
    if let Some((stack, land)) = plan_toss(g, &name) {
        if crate::orders::side_of(g, d, stack) == side && off_of(g, d, land, side) < -0.5 { return [land[0], 0.1, land[2]]; }
    }
    let mut b = 0.9;
    while b <= deep + 1e-6 {
        let p = door_pt(g, d, w / 2.0, b, -side, None);
        if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || crate::scene_collision::low_blocked(g, prev, p) { break; }
        best = p;
        prev = p;
        b += 0.3;
    }
    [best[0], 0.1, best[2]]
}

/// The canister goes off, with this entry's own men off the list.
pub(super) fn door_tools_bang(g: &mut crate::game::Game, op: DoorOpId, it: ItemId) {
    let all = g.guards.clone();
    g.tab.item[it].fired = true;
    let (d, side) = (dop!(g, op).door, dop!(g, op).side);
    let word = word_ops(g, &crate::seats::here(g), op);
    let gate = gate_ops(g, op);
    let sams = crate::orders::squad_orders_sams(&crate::seats::here(g));
    let pl = g.player;
    let sam_pos = g.player_pos(pl);
    let sam_ours = sams && near_door(g, d, side, sam_pos);
    let team = match dop!(g, op).threw { Some(th) => g.team(th), None => crate::orders::squad_orders_team(&crate::seats::here(g)) };
    g.tab.item[it].side = Some(team);
    let mut rest: Vec<GuardId> = Vec::new();
    for x in all.iter().copied() {
        let ours = on_entry(g, op, &word, &gate, x) || ((if sams { g.team(x).ours() } else { crate::orders::squad_orders_mine(g, &crate::seats::here(g), x) }) && { let at = g.pos_of(x); near_door(g, d, side, at) });
        if !ours { rest.push(x); }
    }
    g.guards = rest.clone();
    let order_team = crate::orders::squad_orders_team(&crate::seats::here(g));
    crate::combat::detonate_stun(g, it, Some(false), Some(crate::combat::DetonateStunSam { side: Some(order_team), spare: Some(sam_ours) }));
    g.guards = all;
    let at = g.tab.item[it].pos;
    dop!(g, op).bang_t = g.time;
    dop!(g, op).bang_at = Some(v3::copy(at));
    dop!(g, op).phase = Phase::Wait;
    g.sets[g.seat].door_tools.stats.bangs += 1.0;
    if dop!(g, op).thr.is_none() { door_tools_close_up(g, op, None); }
    let off = off_of(g, d, at, side);
    door_tools_beat_is(g, op, crate::doortools::Beat::Bang, &format!("bang · {} {} m", if off < 0.0 { "inside" } else { "OUTSIDE" }, js::to_fixed(m::abs(off), 1)));
    if dop!(g, op).thr.is_some() { door_tools_thr_banged(g, op); }
}

/// A man's corners on the way in and his muzzle's turn on them; true while he has one to check.
pub(super) fn door_tools_corner_step(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool {
    if !dop!(g, op).corner.has(gd) {
        let pts = door_tools_corners_of(g, op, pod);
        dop!(g, op).corner.set(gd, crate::doortools::DoorToolsDoneItemCornerValue { pts, i: 0.0, on: m::NAN, fight: None, last: None, turned: None });
    }
    let mut c = dop!(g, op).corner.get(gd).unwrap().clone();
    let more = corner_turn(g, op, gd, a, pod, &mut c);
    *dop!(g, op).corner.get_mut(gd).unwrap() = c;
    more
}

fn corner_turn(g: &mut Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod, c: &mut crate::doortools::DoorToolsDoneItemCornerValue) -> bool {
    let t = g.time;
    let n = c.pts.len() as f64;
    if c.i >= n { return false; }
    let pt = c.pts[c.i as usize];
    let ch = g.gd[gd].ch;
    let p = g.ch[ch].pos;
    let yaw = v3::yaw_to(p, pt);
    asg!(g, a).watch = Some(pt);
    let done = dop!(g, op).threat.as_ref().and_then(|th| th.deal.as_ref()).and_then(|de| de.get(gd)).map_or(-m::INF, |w| w.done_t);
    let tx = m::max(door_tools_in_at(g, op, gd), done);
    if tx.is_nan() { return true; }
    let up = crate::roomfight::has_over(g, gd);
    if up { c.fight = Some(c.fight.unwrap_or(0.0) + v3::clamp(t - c.last.unwrap_or(t), 0.0, 0.05)); }
    c.last = Some(t);
    if !up && door_tools_on_point(g, gd, a, pod) && (c.turned.unwrap_or(false) || m::abs(v3::wrap_angle(yaw - g.ch[ch].body_yaw)) > 1.3) {
        c.turned = Some(true);
        asg!(g, a).face = Some(yaw);
    }
    if !up && m::abs(v3::wrap_angle(g.ch[ch].aim_yaw - yaw)) < g.k.doortools.corner_rad {
        if c.on.is_nan() { c.on = t; }
    } else {
        c.on = m::NAN;
    }
    if (!c.on.is_nan() && t - c.on >= g.k.doortools.corner_secs) || t - tx - c.fight.unwrap_or(0.0) > g.k.doortools.corner_cap * (c.i + 1.0) || v3::dist_xz(p, pt) < 1.0 {
        c.i += 1.0;
        c.on = m::NAN;
        if c.i >= n { asg!(g, a).watch = None; }
    }
    c.i < n
}

/// Stood on the point the entry gave him, or where he stopped short of it.
pub(super) fn door_tools_on_point(g: &crate::game::Game, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool {
    let p = g.pos_of(gd);
    let post = asg!(g, a).post;
    !(v3::dist_xz(p, pod.pos) > 0.6 && post.is_some() && v3::dist_xz(p, post.unwrap()) > 0.6)
}

/// Has he a corner of an entry still to check, with a mate through the frame behind him.
pub fn door_tools_owes_corner(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId) -> bool {
    for op in seat.door_tools.ops.iter().copied() {
        let o = &dop!(g, op);
        if o.phase != Phase::Enter { continue; }
        if let Some(c) = o.corner.get(gd) {
            if (c.i as usize) < c.pts.len() && o.men.contains(&gd) {
                return o.men.iter().any(|x| *x != gd && !crate::condition::down(g, g.gd[*x].ch) && o.crossed.has(*x));
            }
        }
    }
    false
}

fn cell_of(g: &Game, p: V3) -> f64 {
    let (cell, w) = (g.col.nav.cell, g.col.nav.w);
    // (+ 0.0: a JavaScript Set holds -0 and 0 as one key, a Num does not)
    m::floor(p[2] / cell) * w + m::floor(p[0] / cell) + 0.0
}
/// threatNote's `beyond`: in the room behind the door, or in the doorway / past the frame line.
fn threat_beyond(g: &Game, op: DoorOpId, p: V3) -> bool {
    let (d, side) = (dop!(g, op).door, dop!(g, op).side);
    let key = crate::Num(cell_of(g, p));
    let room = dop!(g, op).threat.as_ref().and_then(|t| t.room);
    if let Some(r) = room {
        if g.tab.roomfight_room[r].cells.has(key) { return true; }
    }
    let reach = if room.is_some() { 1.5 } else { g.k.doortools.look_m };
    off_of(g, d, p, side) < 0.0 && v3::dist_xz(p, frame(g, d)) < reach
}
/// threatNote's `note`: the row is a thing of its own, so a ThreatWatch dealt it sees what is written on it.
fn threat_mark(g: &mut Game, op: DoorOpId, by: GuardId, at: V3, how: crate::doortools::KnownThreatHow, t: f64) {
    let found = dop!(g, op).threat.as_ref().unwrap().known.iter().copied().find(|x| g.tab.doortools_known_threat[*x].by == Some(by));
    if let Some(id) = found {
        let k = &mut g.tab.doortools_known_threat[id];
        k.at = [at[0], 0.0, at[2]];
        k.t = t;
        return;
    }
    let id = g.tab.doortools_known_threat.push(crate::doortools::KnownThreat { at: [at[0], 0.0, at[2]], t, by: Some(by) });
    dop!(g, op).threat.as_mut().unwrap().known.push(id);
    let line = format!("threat known · {} · {} ({}, {})", how.as_str(), g.gd[by].callsign, js::to_fixed(at[0], 1), js::to_fixed(at[2], 1));
    door_tools_beat(g, op, &line);
}

/// What the order's men have learnt of an enemy beyond their door this frame.
pub(super) fn door_tools_threat_note(g: &mut crate::game::Game, op: DoorOpId) {
    let t = g.time;
    let (d, side) = (dop!(g, op).door, dop!(g, op).side);
    if dop!(g, op).threat.is_none() {
        dop!(g, op).threat = Some(crate::doortools::DoorToolsDoneItemThreat { known: Vec::new(), ev_t: t - 0.5, room: None, room_t: None, deal: None });
    }
    // (room_t none is `T.room === undefined`: the two are only ever written here, together)
    if dop!(g, op).threat.as_ref().unwrap().room_t.map_or(true, |rt| t - rt > 2.0) {
        dop!(g, op).threat.as_mut().unwrap().room_t = Some(t);
        let w = g.dr[d].def.width;
        let at = door_pt(g, d, w / 2.0, 0.9, -side, None);
        let room = crate::roomfight::room_fight_room_at(g, at);
        let near = door_pt(g, d, w / 2.0, 0.9, side, None);
        let key = crate::Num(cell_of(g, near));
        let room = match room {
            Some(r) if g.tab.roomfight_room[r].cells.has(key) => None,
            other => other,
        };
        dop!(g, op).threat.as_mut().unwrap().room = room;
    }
    for gd in dop!(g, op).men.clone() {
        let o = match crate::roomfight::room_fight_op_of(g, gd) { Some(o) => o, None => continue };
        for (h, seen_t) in g.tab.roomfight_op[o].meters.iter().map(|(h, mt)| (h, mt.seen_t)).collect::<Vec<_>>() {
            if !crate::condition::down(g, g.gd[h].ch) && t - seen_t < 0.1 {
                let p = g.pos_of(h);
                if threat_beyond(g, op, p) { threat_mark(g, op, h, p, crate::doortools::KnownThreatHow::Seen, t); }
            }
        }
    }
    let ev_t = dop!(g, op).threat.as_ref().unwrap().ev_t;
    let near_m = g.k.doortools.threat_near_m;
    let mut i = g.events.len();
    while i > 0 {
        i -= 1;
        let (time, kind, from, to, who) = { let e = &g.events[i]; (e.time, e.kind, e.from, e.to, e.who) };
        if time < ev_t { break; }
        if kind != crate::director::EventKind::GuardShot { continue; }
        let (a, to, who) = match (from, to, who) { (Some(a), Some(b), Some(w)) => (a, b, w), _ => continue };
        if !threat_beyond(g, op, a) { continue; }
        let by = match g.guards.iter().copied().find(|x| g.char_id(*x) == who) { Some(x) => x, None => continue };
        if g.team(by).ours() || crate::condition::down(g, g.gd[by].ch) { continue; }
        let (bx, bz) = (to[0] - a[0], to[2] - a[2]);
        let l2 = js::or(bx * bx + bz * bz, 1e-6);
        for x in dop!(g, op).from.keys() {
            if !dop!(g, op).men.contains(&x) && !crate::condition::down(g, g.gd[x].ch) { continue; }
            let p = g.pos_of(x);
            if crate::condition::down(g, g.gd[x].ch) && !door_tools_sees(g, op, p) { continue; }
            let k = v3::clamp(((p[0] - a[0]) * bx + (p[2] - a[2]) * bz) / l2, 0.0, 1.0);
            if m::hyp2(a[0] + bx * k - p[0], a[2] + bz * k - p[2]) <= near_m {
                threat_mark(g, op, by, a, crate::doortools::KnownThreatHow::Shot, t);
                break;
            }
        }
    }
    dop!(g, op).threat.as_mut().unwrap().ev_t = t;
    let secs = g.k.doortools.threat_secs;
    let mut i = dop!(g, op).threat.as_ref().unwrap().known.len();
    while i > 0 {
        i -= 1;
        let k = g.tab.doortools_known_threat[dop!(g, op).threat.as_ref().unwrap().known[i]];
        let gone = t - k.t > secs || match k.by {
            Some(by) if crate::condition::down(g, g.gd[by].ch) => { let p = g.pos_of(by); door_tools_sees(g, op, p) }
            _ => false,
        };
        if gone { dop!(g, op).threat.as_mut().unwrap().known.remove(i); }
    }
}

/// One of the order's men still up has a line from his eyes to a man on the floor there.
fn door_tools_sees(g: &crate::game::Game, op: DoorOpId, p: V3) -> bool {
    let at: V3 = [p[0], 0.4, p[2]];
    for x in dop!(g, op).men.clone() {
        if crate::condition::down(g, g.gd[x].ch) { continue; }
        let eye = eye_of(g, x);
        if !crate::scene_collision::segment_blocked(g, eye, at, None, None, None) { return true; }
    }
    false
}

fn threat_watch(th: DoortoolsKnownThreatId, lead: Option<GuardId>) -> crate::doortools::ThreatWatch {
    crate::doortools::ThreatWatch { th, lead, on: m::NAN, fight: 0.0, last: m::NAN, done_t: m::NAN, faced: None, at: None, still: None }
}

/// The entry's first frame: the known places dealt to the first two men in.
pub(super) fn door_tools_threat_deal(g: &mut crate::game::Game, op: DoorOpId) {
    match dop!(g, op).threat.as_ref() { Some(th) if th.deal.is_none() => {} _ => return }
    dop!(g, op).threat.as_mut().unwrap().deal = Some(JsMap::new());
    let fc = frame(g, dop!(g, op).door);
    let mut known = dop!(g, op).threat.as_ref().unwrap().known.clone();
    js::sort_by(&mut known, |a, b| v3::dist_xz(g.tab.doortools_known_threat[*a].at, fc) - v3::dist_xz(g.tab.doortools_known_threat[*b].at, fc));
    if known.is_empty() || dop!(g, op).queue.is_empty() { return; }
    let m1 = dop!(g, op).queue[0].gd;
    let m2 = dop!(g, op).queue.get(1).map(|q| q.gd);
    let mut said: Vec<String> = Vec::new();
    let mut deal: JsMap<GuardId, crate::doortools::ThreatWatch> = JsMap::new();
    deal.set(m1, threat_watch(known[0], None));
    if let Some(m2) = m2 {
        deal.set(m2, if known.len() > 1 { threat_watch(known[1], None) } else { threat_watch(known[0], Some(m1)) });
    }
    for (x, w) in deal.iter() {
        let th = g.tab.doortools_known_threat[w.th];
        let by = match th.by { Some(b) => g.gd[b].callsign.clone(), None => String::from("a man") };
        said.push(format!("{} on {}'s place ({}, {}){}", g.gd[x].callsign, by, js::to_fixed(th.at[0], 1), js::to_fixed(th.at[2], 1), if w.lead.is_some() { " until the first man has it" } else { "" }));
    }
    dop!(g, op).threat.as_mut().unwrap().deal = Some(deal);
    door_tools_beat(g, op, &format!("threat first · {}", said.join(" · ")));
}

/// A man going in whose gun is a known place's: true while it still is.
pub(super) fn door_tools_threat_step(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool {
    let mut w = match dop!(g, op).threat.as_ref().unwrap().deal.as_ref().unwrap().get(gd) { Some(w) if w.done_t.is_nan() => *w, _ => return false };
    let more = threat_watching(g, op, gd, a, pod, &mut w);
    *dop!(g, op).threat.as_mut().unwrap().deal.as_mut().unwrap().get_mut(gd).unwrap() = w;
    more
}

/// threatStep's `done`
fn threat_done(g: &mut Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: &crate::entrypoints::Pod, w: &mut crate::doortools::ThreatWatch, why: &str) -> bool {
    w.done_t = g.time;
    asg!(g, a).watch = None;
    if w.faced.unwrap_or(false) { asg!(g, a).strafe = Some(false); asg!(g, a).face = Some(pod.face); }
    let line = format!("threat {} · {}", why, g.gd[gd].callsign);
    door_tools_beat(g, op, &line);
    false
}

fn threat_watching(g: &mut Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod, w: &mut crate::doortools::ThreatWatch) -> bool {
    let t = g.time;
    let ch = g.gd[gd].ch;
    let th = g.tab.doortools_known_threat[w.th];
    let spot: V3 = [th.at[0], 1.2, th.at[2]];
    if t - th.t > g.k.doortools.threat_secs { return threat_done(g, op, gd, a, &pod, w, "let go: old news"); }
    if let Some(by) = th.by {
        if crate::condition::down(g, g.gd[by].ch) { let p = g.pos_of(by); if door_tools_sees(g, op, p) { return threat_done(g, op, gd, a, &pod, w, "down"); } }
    }
    if let Some(lead) = w.lead {
        let row = dop!(g, op).threat.as_ref().unwrap().deal.as_ref().unwrap().get(lead).copied();
        if crate::condition::down(g, g.gd[lead].ch) || !dop!(g, op).men.contains(&lead) || row.is_none() {
            w.lead = None;
        } else if !row.unwrap().done_t.is_nan() {
            return threat_done(g, op, gd, a, &pod, w, "dealt with by the first man");
        } else if dop!(g, op).crossed.has(lead) {
            let eye = eye_of(g, lead);
            if !crate::scene_collision::segment_blocked(g, eye, spot, None, None, None) { return threat_done(g, op, gd, a, &pod, w, "is the first man's"); }
        }
    }
    asg!(g, a).watch = Some(spot);
    let tx = door_tools_in_at(g, op, gd);
    let p = g.ch[ch].pos;
    let up = crate::roomfight::has_over(g, gd);
    let since = |last: f64| v3::clamp(t - (if last.is_nan() { t } else { last }), 0.0, 0.05);
    if tx.is_nan() {
        let stuck = g.k.doortools.threat_stuck;
        if stuck > 0.0 {
            if w.at.is_none() || v3::dist_xz(w.at.unwrap(), p) > 0.3 {
                w.at = Some([p[0], 0.0, p[2]]);
                w.still = Some(0.0);
            } else if !up {
                w.still = Some(w.still.unwrap_or(0.0) + since(w.last));
            }
            w.last = t;
            if w.still.unwrap_or(0.0) > stuck { return threat_done(g, op, gd, a, &pod, w, "let go: held up at the frame"); }
        }
        return true;
    }
    let yaw = v3::yaw_to(p, spot);
    if up { w.fight += since(w.last); }
    w.last = t;
    if w.lead.is_none() || door_tools_on_point(g, gd, a, pod) {
        asg!(g, a).face = Some(yaw);
        asg!(g, a).strafe = Some(w.lead.is_none());
        w.faced = Some(true);
    }
    if w.lead.is_none() {
        let mut on = false;
        if !up && m::abs(v3::wrap_angle(g.ch[ch].aim_yaw - yaw)) < g.k.doortools.corner_rad {
            let eye = eye_of(g, gd);
            on = !crate::scene_collision::segment_blocked(g, eye, spot, None, None, None);
        }
        if on { if w.on.is_nan() { w.on = t; } } else { w.on = m::NAN; }
        if !w.on.is_nan() && t - w.on >= g.k.doortools.corner_secs { return threat_done(g, op, gd, a, &pod, w, "looked at: nobody there"); }
        if v3::dist_xz(p, spot) < 1.0 { return threat_done(g, op, gd, a, &pod, w, "stood on"); }
    }
    if t - tx - w.fight > g.k.doortools.threat_cap { return threat_done(g, op, gd, a, &pod, w, "cap"); }
    true
}

/// Who goes in first: the front man who has seen most of the room.
pub(super) fn door_tools_first_in(g: &mut crate::game::Game, op: DoorOpId, at: Option<V3>, by_gun: Option<bool>) -> GuardId {
    let by_gun = by_gun.unwrap_or(true);
    let (lead, d, side) = (dop!(g, op).lead, dop!(g, op).door, dop!(g, op).side);
    let w = g.dr[d].def.width;
    let men = dop!(g, op).men.clone();
    if men.len() < 2 { return lead; }
    if by_gun {
        if let Some(f) = dop!(g, op).first { if !crate::condition::down(g, g.gd[f].ch) && men.contains(&f) { return f; } }
    }
    let fc = frame(g, d);
    let mut fronts: Vec<GuardId> = Vec::new();
    for s in [true, false] {
        let mut file: Vec<(GuardId, f64)> = Vec::new();
        for x in men.iter().copied() {
            if crate::condition::down(g, g.gd[x].ch) { continue; }
            let there = match dop!(g, op).post.get(x) { Some(p) => p.pos, None => g.pos_of(x) };
            if (along_of(g, d, there) > w / 2.0) != s { continue; }
            let p = g.pos_of(x);
            if off_of(g, d, p, side) > 0.0 { file.push((x, v3::dist_xz(there, fc))); }
        }
        js::sort_by(&mut file, |a, b| a.1 - b.1);
        if let Some(f) = file.first() { fronts.push(f.0); }
    }
    let threw = dop!(g, op).threw;
    let hands: Vec<GuardId> = fronts.iter().copied().filter(|x| Some(*x) != threw || *x == lead || fronts.len() < 2).collect();
    let rw = crate::rearwatch::rear_watcher(g, &crate::seats::here(g), op);
    let able: Vec<GuardId> = match rw {
        Some(r) if hands.iter().any(|x| *x != r) => hands.iter().copied().filter(|x| *x != r).collect(),
        _ => hands,
    };
    let list = if by_gun { crate::gunroles::gun_first(g, &able) } else { able };
    let at = match at { Some(a) => Some(a), None => door_tools_pods_as_were(g, d, side, Some(2.0)).first().map(|p| p.pos) };
    door_tools_seen_most(g, lead, &list, at)
}

/// Of `men`, the one who has seen most of the room `at` is in.
pub(super) fn door_tools_seen_most(g: &mut crate::game::Game, lead: GuardId, men: &[GuardId], at: Option<V3>) -> GuardId {
    let room = match at { Some(a) if g.sets[g.seat].door_tools.seen.is_some() => crate::roomfight::room_fight_room_at(g, a), _ => None };
    let room = match room {
        Some(r) if !men.is_empty() => r,
        _ => return if men.contains(&lead) || men.is_empty() { lead } else { men[0] },
    };
    let since = g.time - g.k.doortools.lead_seen_secs;
    let cells: Vec<f64> = g.tab.roomfight_room[room].cells.iter().map(|c| c.0).collect();
    let mut best = if men.contains(&lead) { lead } else { men[0] };
    let mut bn = if best == lead {
        crate::floorseen::floor_seen_count(&crate::seats::here(g), lead, &cells, since) + m::max(0.0, g.k.doortools.lead_seen_min - 1.0)
    } else {
        crate::floorseen::floor_seen_count(&crate::seats::here(g), best, &cells, since)
    };
    for x in men.iter().copied() {
        if x == best { continue; }
        let k = crate::floorseen::floor_seen_count(&crate::seats::here(g), x, &cells, since);
        if k > bn { bn = k; best = x; }
    }
    best
}

/// When he came through the frame on this entry; NaN until 'going in'.
pub(super) fn door_tools_in_at(g: &crate::game::Game, op: DoorOpId, gd: GuardId) -> f64 {
    let o = &dop!(g, op);
    match o.crossed.get(gd) {
        Some(tx) if o.phase == Phase::Enter => m::max(*tx, o.t0),
        _ => m::NAN,
    }
}

/// On his sector, as the entry leaves him.
pub(super) fn door_tools_on_sector(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> f64 {
    let p = g.pos_of(gd);
    let yaw = match pod.watch { Some(w) => v3::yaw_to(p, w), None => pod.face };
    let watch = match pod.watch { Some(w) => [w[0], w[1], w[2]], None => [p[0] + m::sin(yaw) * 4.0, 1.2, p[2] + m::cos(yaw) * 4.0] };
    asg!(g, a).watch = Some(watch);
    asg!(g, a).face = Some(yaw);
    asg!(g, a).there = "holding the room".into();
    yaw
}

/// The men an ended entry has holding one of its room's other doors.
pub fn door_tools_door_keepers(seat: &crate::seats::SeatRef) -> Vec<(GuardId, DoorId, OrdersAssignmentId)> {
    let mut out = Vec::new();
    for (gd, r) in seat.door_tools.held.iter() {
        if let Some(d) = r.door { out.push((gd, d, r.a)); }
    }
    out
}

fn sector_hit(g: &Game, gd: GuardId, p: V3, y: f64) -> bool {
    crate::guards::colleague_in_line(g, gd, [p[0] + m::sin(y) * 4.0, 1.2, p[2] + m::cos(y) * 4.0], None).is_some()
}

/// A held sector is kept off a mate.
pub(super) fn door_tools_keep_sectors(g: &mut crate::game::Game) {
    let t = g.time;
    // (a copy of the keys: the body removes only the entry it stands on)
    for gd in g.sets[g.seat].door_tools.held.keys() {
        let mut r = match g.sets[g.seat].door_tools.held.get(gd) { Some(r) => *r, None => continue };
        if crate::condition::down(g, g.gd[gd].ch) || crate::orders::squad_orders_of(&crate::seats::here(g), gd) != Some(r.a) { g.sets[g.seat].door_tools.held.delete(gd); continue; }
        if r.door.is_some() && (!crate::nvg::nvg_worn(g, gd) || crate::nvg::nvg_black(g, gd)) { asg!(g, r.a).search_t = Some(t); }
        keep_sector(g, gd, &mut r, t);
        *g.sets[g.seat].door_tools.held.get_mut(gd).unwrap() = r;
    }
}

fn keep_sector(g: &mut Game, gd: GuardId, r: &mut crate::doortools::DoorToolsHeldValue, t: f64) {
    let a = r.a;
    if asg!(g, a).arrived {
        if r.arr_t.is_nan() { r.arr_t = asg!(g, a).arr_t; } else { asg!(g, a).arr_t = r.arr_t; }
    }
    if !asg!(g, a).arrived || crate::roomfight::has_over(g, gd) || r.out.unwrap_or(false) { return; }
    let p = g.pos_of(gd);
    let want = if !sector_hit(g, gd, p, r.face) { Some(0.0) } else if r.d != 0.0 && !sector_hit(g, gd, p, r.face + r.d) { Some(r.d) } else { None };
    if want == Some(r.d) { r.hit_t = m::NAN; return; }
    if r.hit_t.is_nan() { r.hit_t = t; }
    if t - r.hit_t < g.k.doortools.sector_secs { return; }
    let mut d = want;
    if d.is_none() {
        for x in [0.25, -0.25, 0.5, -0.5, 0.75, -0.75, 1.0, -1.0] {
            if !sector_hit(g, gd, p, r.face + x) { d = Some(x); break; }
        }
    }
    let d = match d { Some(d) => d, None => return };
    r.d = d;
    r.hit_t = m::NAN;
    let yaw = r.face + d;
    asg!(g, a).watch = Some([p[0] + m::sin(yaw) * 4.0, 1.2, p[2] + m::cos(yaw) * 4.0]);
    asg!(g, a).face = Some(yaw);
}

/// The room this entry goes into, as the room fight has it.
pub(super) fn door_tools_room_in(g: &mut crate::game::Game, op: DoorOpId) -> Option<RoomfightRoomId> {
    let at = match dop!(g, op).queue.first() {
        Some(q) => q.pod.pos,
        None => dop!(g, op).pod.values().next()?.pos,
    };
    crate::roomfight::room_fight_room_at(g, at)
}

/// A man's corners: the near one on his own side of the door, then that side's deep one.
fn door_tools_corners_of(g: &mut crate::game::Game, op: DoorOpId, pod: crate::entrypoints::Pod) -> Vec<V3> {
    let (d, side) = (dop!(g, op).door, dop!(g, op).side);
    let w = g.dr[d].def.width;
    let room = match crate::roomfight::room_fight_room_at(g, pod.pos) { Some(r) => r, None => return Vec::new() };
    let (nav_w, cs) = (g.col.nav.w, g.col.nav.cell);
    let s = if along_of(g, d, pod.pos) >= w / 2.0 { 1.0 } else { -1.0 };
    let (mut near, mut nb, mut deep, mut db): (Option<V3>, f64, Option<V3>, f64) = (None, 0.0, None, 0.0);
    for ci in g.tab.roomfight_room[room].cells.keys() {
        let ci = ci.0;
        let p: V3 = [(js::rem(ci, nav_w) + 0.5) * cs, 1.1, (js::to_i32(ci / nav_w) as f64 + 0.5) * cs];
        let a = s * (along_of(g, d, p) - w / 2.0);
        let b = -off_of(g, d, p, side);
        if b < 0.2 || a < 0.0 || a > 9.0 || b > 10.0 { continue; }
        if b <= 1.8 {
            if a >= 0.8 && a - b * 0.5 > nb { nb = a - b * 0.5; near = Some(p); }
        } else if a * 0.7 + b > db {
            db = a * 0.7 + b;
            deep = Some(p);
        }
    }
    [near, deep].iter().filter_map(|p| *p).collect()
}

/// The room this entry is into as floor to look over, built once.
pub(super) fn door_tools_search_of(g: &mut crate::game::Game, op: DoorOpId, at: Option<V3>) -> Option<DoortoolsRoomSearchId> {
    // (None: never asked. Some(None): asked, no floor. Some(Some(id)): one floor, shared by every order in the room)
    if let Some(asked) = dop!(g, op).search { return asked; }
    dop!(g, op).search = Some(None);
    let room = match at { Some(a) => crate::roomfight::room_fight_room_at(g, a), None => door_tools_room_in(g, op) };
    let room = room?;
    if open_floor(g, room) { return None; }
    for o in g.sets[g.seat].door_tools.ops.clone() {
        if o == op { continue; }
        if let Some(Some(s)) = dop!(g, o).search {
            if g.tab.doortools_room_search[s].room == room {
                dop!(g, op).search = Some(Some(s));
                return Some(s);
            }
        }
    }
    let (nav_w, cs) = (g.col.nav.w, g.col.nav.cell);
    let pts: Vec<V3> = g.tab.roomfight_room[room].cells.iter().map(|ci| [(js::rem(ci.0, nav_w) + 0.5) * cs, 1.0, (js::to_i32(ci.0 / nav_w) as f64 + 0.5) * cs]).collect();
    let n = pts.len();
    let id = g.tab.doortools_room_search.push(crate::doortools::RoomSearch { room, pts, looked: vec![0u8; n], los: JsMap::new(), yaw: JsMap::new(), at: JsMap::new(), moves: 0.0, left_at_move: m::INF, left: n as f64, flows: None, closes: None, flow_said: None, late: None, cell_at: None, fight_t: None, fight_at: None });
    dop!(g, op).search = Some(Some(id));
    Some(id)
}
