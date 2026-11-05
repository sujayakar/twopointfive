//! src/game/doortools.ts lines 2580 to 2963
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};


macro_rules! dop { ($g:ident, $op:expr) => { $g.tab.doortools_door_op[$op] }; }
macro_rules! asg { ($g:ident, $a:expr) => { $g.tab.orders_assignment[$a] }; }

fn pos(g: &Game, m: GuardId) -> V3 { g.ch[g.gd[m].ch].pos }
fn dead(g: &Game, m: GuardId) -> bool { g.gd[m].state == crate::game::GuardState::Dead }
fn frame(g: &Game, d: DoorId) -> V3 { g.dr[d].frame_centre }
fn over(g: &Game, m: GuardId) -> bool { g.gd[m].script.as_ref().map_or(false, |s| s.over.is_some()) }
fn eye_of(g: &Game, m: GuardId) -> V3 {
    let c = g.gd[m].ch;
    match g.ch[c].bones.get(crate::character::Bone::Head) { Some(h) => *h, None => [g.ch[c].pos[0], 1.6, g.ch[c].pos[2]] }
}
fn blocked_at(g: &mut Game, x: f64, z: f64) -> bool { crate::scene_collision::nav_grid_is_blocked(g, x, z) }
fn solid_blocked(g: &mut Game, a: V3, b: V3) -> bool { crate::scene_collision::solid_blocked(g, a, b, None) }
fn segment_blocked(g: &mut Game, a: V3, b: V3) -> bool { crate::scene_collision::segment_blocked(g, a, b, None, None, None) }

/// `{ rate }` of g.audio.play: `opts` is a map of crate::Opaque, which holds no number
fn seam_audio_play_rate(g: &mut Game, name: &str, at: V3, vol: f64, rate: f64) { todo!("needs: a rate in game_audio_like_play's opts") }
/// `op.search !== undefined`: the skeleton's Option cannot tell "never asked" from null
fn seam_search_asked(g: &mut Game, op: DoorOpId) -> bool { todo!("needs: op.search asked or not") }
fn seam_search_mark_asked(g: &mut Game, op: DoorOpId) { todo!("needs: op.search asked or not") }
/// `op.search = o.search`: one object shared by two orders
fn seam_search_share<'g>(g: &'g mut Game, op: DoorOpId, o: DoorOpId) -> Option<&'g mut crate::doortools::RoomSearch> { todo!("needs: a RoomSearch table and ID") }

/// the men of the orders let go on this order's word (doortools.ts bangSpot, bang)
fn word_ops(g: &Game, op: DoorOpId) -> Vec<DoorOpId> {
    match dop!(g, op).word {
        Some(w) => g.door_tools.ops.iter().copied().filter(|o| dop!(g, *o).word == Some(w)).collect(),
        None => Vec::new(),
    }
}
fn gate_ops(g: &Game, op: DoorOpId) -> Vec<DoorOpId> {
    match dop!(g, op).gate { Some(w) => g.tab.doortools_door_gate[w].ops.clone(), None => Vec::new() }
}
fn on_entry(g: &Game, op: DoorOpId, word: &[DoorOpId], gate: &[DoorOpId], x: GuardId) -> bool {
    dop!(g, op).men.contains(&x) || word.iter().any(|o| dop!(g, *o).men.contains(&x)) || gate.iter().any(|o| dop!(g, *o).men.contains(&x))
}
fn near_door(g: &mut Game, d: DoorId, side: f64, q: V3) -> bool {
    off_of(g, d, q, side) > 0.0 && v3::dist_xz(q, frame(g, d)) < 5.0
}

/// Behind the bang the first man closes up to the edge of the frame. (doortools.ts DoorTools.closeUp)
pub fn door_tools_close_up(g: &mut crate::game::Game, op: DoorOpId, gd: Option<GuardId>) {
    let gd = gd.unwrap_or(dop!(g, op).lead);
    let d = dop!(g, op).door;
    let side = dop!(g, op).side;
    let w = g.dr[d].def.width;
    let p = match dop!(g, op).post.get(gd) { Some(p) => *p, None => return };
    if !dop!(g, op).men.contains(&gd) { return; }
    let a = if along_of(g, d, p.pos) > w / 2.0 { w + g.k.doortools.edge_out } else { -g.k.doortools.edge_out };
    let off = off_of(g, d, p.pos, side);
    let q = door_pt(g, d, a, off, side, None);
    if blocked_at(g, q[0], q[2]) || solid_blocked(g, [p.pos[0], 0.45, p.pos[2]], [q[0], 0.45, q[2]]) { return; }
    let fc = frame(g, d);
    let name = crate::orders::door_words(g, d);
    door_tools_assign(g, op, gd, &DoorToolsAssignA { post: Some(q), face: Some(p.face), watch: Some([fc[0], 1.05, fc[2]]), stance: Some(crate::character::Stance::Stack), upper: Some(crate::character::UpperMode::None), stack_side: Some(p.stack_side), going: format!("closing up · {}", name), there: format!("on the frame · {}", name), ..Default::default() });
}

/// The lead back onto his own jamb, after his boot. (doortools.ts DoorTools.backToJamb)
pub fn door_tools_back_to_jamb(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) {
    let p = match dop!(g, op).post.get(gd) { Some(p) => *p, None => return };
    let d = dop!(g, op).door;
    let fc = frame(g, d);
    let name = crate::orders::door_words(g, d);
    door_tools_assign(g, op, gd, &DoorToolsAssignA { post: Some(p.pos), face: Some(p.face), watch: Some([fc[0], 1.05, fc[2]]), stance: Some(crate::character::Stance::Stack), upper: Some(crate::character::UpperMode::None), stack_side: Some(p.stack_side), going: format!("stacking · {}", name), there: format!("stacked · {}", name), ..Default::default() });
}

/// Unlatched and shoved hard into the room from the jamb. (doortools.ts DoorTools.shove)
pub fn door_tools_shove(g: &mut crate::game::Game, op: DoorOpId) {
    let d = dop!(g, op).door;
    let side = dop!(g, op).side;
    let lead = dop!(g, op).lead;
    let c = g.gd[lead].ch;
    let w = g.dr[d].def.width;
    let at = door_pt(g, d, w / 2.0, 0.6, side, None);
    let s = -crate::doors::doors_side(g, d, at);
    crate::doors::door_set_latched(g, d, false);
    crate::doors::door_set_closing(g, d, false);
    let vel = s * m::max(m::abs(crate::doors::door_vel(g, d)), g.k.doortools.shove);
    crate::doors::door_set_vel(g, d, vel);
    crate::doors::door_set_quiet_t(g, d, -1.0);
    crate::doors::door_set_creak_armed(g, d, false);
    let id = g.ch[c].id;
    crate::doors::door_set_last_who(g, d, id);
    let dp = crate::doors::door_pos(g, d);
    if crate::silent::silent_ease(g, lead) { crate::audio_iface::game_audio_like_play(g, "doorCreak", Some(dp), Some(0.5), None); } else { seam_audio_play_rate(g, "doorBash", dp, 0.5, 1.15); }
    door_tools_beat(g, op, "shove");
}

/// The leaf is moving or stood open: what comes next, by the order. (doortools.ts DoorTools.opened)
pub fn door_tools_opened(g: &mut crate::game::Game, op: DoorOpId) {
    let t = g.time;
    if dop!(g, op).open_t.is_nan() { dop!(g, op).open_t = t; }
    g.door_tools.stats.opened += 1.0;
    dop!(g, op).t0 = t;
    match dop!(g, op).kind {
        DoorToolKind::OpenClear => door_tools_enter(g, op),
        DoorToolKind::BangClear => dop!(g, op).phase = Phase::Toss,
        DoorToolKind::Outside => door_tools_to_slice(g, op),
        DoorToolKind::Threshold => door_tools_to_threshold(g, op),
        _ => {}
    }
}

/// Where the canister is thrown to. (doortools.ts DoorTools.landAt)
pub fn door_tools_land_at(g: &mut crate::game::Game, op: DoorOpId, deep: Option<f64>) -> V3 {
    let deep = deep.unwrap_or(g.k.doortools.bang_in);
    let (d, side) = (dop!(g, op).door, dop!(g, op).side);
    door_tools_land_for(g, d, side, deep)
}

/// Where a bang and clear's can is thrown to from `from`; None: no point worth it. (doortools.ts DoorTools.bangSpot)
pub fn door_tools_bang_spot(g: &mut crate::game::Game, op: DoorOpId, from: V3) -> Option<crate::doortools::DoorToolsBangSpot> {
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
            if !(blocked_at(g, q[0], q[2]) || solid_blocked(g, inn, [q[0], 1.9, q[2]])) {
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
    let word = word_ops(g, op);
    let gate = gate_ops(g, op);
    let sams = crate::orders::squad_orders_sams(g);
    let mut ours: Vec<V3> = Vec::new();
    for x in g.guards.clone() {
        let mine = if sams { g.gd[x].team == crate::teams::Team::Operative && g.gd[x].unit.is_some() } else { crate::orders::squad_orders_mine(g, x) };
        if !mine || dead(g, x) { continue; }
        let at = pos(g, x);
        if on_entry(g, op, &word, &gate, x) || near_door(g, d, side, at) { continue; }
        ours.push(at);
    }
    let pl = g.player;
    let sam_at = g.ch[g.pl[pl].ch].pos;
    if !(g.pl[pl].down || (sams && near_door(g, d, side, sam_at))) { ours.push(crate::orders::squad_orders_sam_stood(g)); }
    let (into_min, throw_m, air_h, flash_m, lit) = (g.k.doortools.bang_into_min, g.k.doortools.thr_throw_m, g.k.doortools.air_h, g.k.doortools.thr_flash_m, g.k.doortools.bang_into_lit);
    // (at, n, sc, deep)
    let mut cands: Vec<(V3, f64, f64, f64)> = Vec::new();
    for p in spots.iter().copied() {
        if -off_of(g, d, p, side) < into_min || v3::dist_xz(from, p) > throw_m { continue; }
        let at: V3 = [p[0], air_h - 0.05, p[2]];
        if solid_blocked(g, from, at) { continue; }
        let mut ours_in = false;
        for q in ours.iter().copied() {
            if v3::dist_xz(q, p) <= flash_m && !segment_blocked(g, at, [q[0], 1.45, q[2]]) { ours_in = true; break; }
        }
        if ours_in { continue; }
        let (mut n, mut sc) = (0.0, 0.0);
        for q in floor.iter().copied() {
            let r = v3::dist_xz(p, q);
            if r <= flash_m && !segment_blocked(g, at, [q[0], 1.45, q[2]]) { n += 1.0; sc += 1.0 - r / flash_m; }
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

/// How far in the floor runs clear from a door, for a can's landing. (doortools.ts DoorTools.landFor)
pub fn door_tools_land_for(g: &mut crate::game::Game, d: DoorId, side: f64, deep: f64) -> V3 {
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
        if blocked_at(g, p[0], p[2]) || solid_blocked(g, [prev[0], 0.45, prev[2]], [p[0], 0.45, p[2]]) { break; }
        best = p;
        prev = p;
        b += 0.3;
    }
    [best[0], 0.1, best[2]]
}

/// The canister goes off, with this entry's own men off the list. (doortools.ts DoorTools.bang)
pub fn door_tools_bang(g: &mut crate::game::Game, op: DoorOpId, it: ItemId) {
    let all = g.guards.clone();
    crate::throwables::item_set_fired(g, it, true);
    let (d, side) = (dop!(g, op).door, dop!(g, op).side);
    let word = word_ops(g, op);
    let gate = gate_ops(g, op);
    let sams = crate::orders::squad_orders_sams(g);
    let pl = g.player;
    let sam_pos = g.ch[g.pl[pl].ch].pos;
    let sam_ours = sams && near_door(g, d, side, sam_pos);
    let team = match dop!(g, op).threw { Some(th) => g.gd[th].team, None => crate::orders::squad_orders_team(g) };
    g.tab.item[it].side = Some(team);
    let mut rest: Vec<GuardId> = Vec::new();
    for x in all.iter().copied() {
        let ours = on_entry(g, op, &word, &gate, x) || ((if sams { g.gd[x].team == crate::teams::Team::Operative } else { crate::orders::squad_orders_mine(g, x) }) && { let at = pos(g, x); near_door(g, d, side, at) });
        if !ours { rest.push(x); }
    }
    let was: Vec<f64> = rest.iter().map(|x| g.gd[*x].stagger_until).collect();
    g.guards = rest.clone();
    let order_team = crate::orders::squad_orders_team(g);
    crate::combat::detonate_stun(g, it, Some(false), Some(crate::combat::DetonateStunSam { side: Some(order_team), spare: Some(sam_ours) }));
    g.guards = all;
    for (i, x) in rest.iter().enumerate() {
        if g.gd[*x].team == crate::teams::Team::Operative && g.gd[*x].stagger_until > was[i] { g.door_tools.stats.own_flashed += 1.0; }
    }
    let at = crate::throwables::item_pos(g, it);
    dop!(g, op).bang_t = g.time;
    dop!(g, op).bang_at = Some(v3::copy(at));
    dop!(g, op).phase = Phase::Wait;
    g.door_tools.stats.bangs += 1.0;
    if dop!(g, op).thr.is_none() { door_tools_close_up(g, op, None); }
    let off = off_of(g, d, at, side);
    door_tools_beat(g, op, &format!("bang · {} {:.1} m", if off < 0.0 { "inside" } else { "OUTSIDE" }, m::abs(off)));
    if dop!(g, op).thr.is_some() { door_tools_thr_banged(g, op); }
}

/// A man's corners on the way in and his muzzle's turn on them; true while he has one to check. (doortools.ts DoorTools.cornerStep)
pub fn door_tools_corner_step(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool {
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
    let up = over(g, gd);
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

/// Stood on the point the entry gave him, or where he stopped short of it. (doortools.ts DoorTools.onPoint)
pub fn door_tools_on_point(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool {
    let p = pos(g, gd);
    let post = asg!(g, a).post;
    !(v3::dist_xz(p, pod.pos) > 0.6 && post.is_some() && v3::dist_xz(p, post.unwrap()) > 0.6)
}

/// Has he a corner of an entry still to check, with a mate through the frame behind him. (doortools.ts DoorTools.owesCorner)
pub fn door_tools_owes_corner(g: &mut crate::game::Game, gd: GuardId) -> bool {
    for op in g.door_tools.ops.iter().copied() {
        let o = &dop!(g, op);
        if o.phase != Phase::Enter { continue; }
        if let Some(c) = o.corner.get(gd) {
            if (c.i as usize) < c.pts.len() && o.men.contains(&gd) {
                return o.men.iter().any(|x| *x != gd && !dead(g, *x) && o.crossed.has(*x));
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
fn threat_beyond(g: &mut Game, op: DoorOpId, p: V3) -> bool {
    let (d, side) = (dop!(g, op).door, dop!(g, op).side);
    let key = crate::Num(cell_of(g, p));
    let (in_room, has_room) = match dop!(g, op).threat.as_ref().and_then(|t| t.room.as_ref()) {
        Some(r) => (r.cells.has(key), true),
        None => (false, false),
    };
    if in_room { return true; }
    let reach = if has_room { 1.5 } else { g.k.doortools.look_m };
    off_of(g, d, p, side) < 0.0 && v3::dist_xz(p, frame(g, d)) < reach
}
fn same_threat(a: &crate::doortools::KnownThreat, b: &crate::doortools::KnownThreat) -> bool {
    a.by == b.by && a.how == b.how && a.t.to_bits() == b.t.to_bits() && a.at.iter().zip(b.at.iter()).all(|(x, y)| x.to_bits() == y.to_bits())
}
/// threatNote's `note`. A ThreatWatch's `th` is the same object as a row of `known` in the TypeScript: what is written on the row is written on it.
fn threat_mark(g: &mut Game, op: DoorOpId, by: GuardId, at: V3, how: crate::doortools::KnownThreatHow, t: f64) {
    let th = dop!(g, op).threat.as_mut().unwrap();
    if let Some(k) = th.known.iter_mut().find(|x| x.by == Some(by)) {
        let was = *k;
        k.at = [at[0], 0.0, at[2]];
        k.t = t;
        k.how = how;
        let now = *k;
        if let Some(deal) = th.deal.as_mut() {
            for (_, w) in deal.iter_mut() { if same_threat(&w.th, &was) { w.th = now; } }
        }
        return;
    }
    th.known.push(crate::doortools::KnownThreat { at: [at[0], 0.0, at[2]], t, by: Some(by), how });
    let line = format!("threat known · {} · {} ({:.1}, {:.1})", how.as_str(), g.gd[by].callsign, at[0], at[2]);
    door_tools_beat(g, op, &line);
}

/// What the order's men have learnt of an enemy beyond their door this frame. (doortools.ts DoorTools.threatNote)
pub fn door_tools_threat_note(g: &mut crate::game::Game, op: DoorOpId) {
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
            Some(r) if !g.tab.roomfight_room[r].cells.has(key) => Some(crate::doortools::DoorToolsDoneItemThreatRoom { cells: g.tab.roomfight_room[r].cells.clone() }),
            _ => None,
        };
        dop!(g, op).threat.as_mut().unwrap().room = room;
    }
    for gd in dop!(g, op).men.clone() {
        let o = match crate::roomfight::room_fight_op_of(g, gd) { Some(o) => o, None => continue };
        for (h, seen_t) in g.tab.roomfight_op[o].meters.iter().map(|(h, mt)| (h, mt.seen_t)).collect::<Vec<_>>() {
            if !dead(g, h) && t - seen_t < 0.1 {
                let p = pos(g, h);
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
        let by = match g.guards.iter().copied().find(|x| g.ch[g.gd[*x].ch].id == who) { Some(x) => x, None => continue };
        if g.gd[by].team == crate::teams::Team::Operative || dead(g, by) { continue; }
        let (bx, bz) = (to[0] - a[0], to[2] - a[2]);
        let l2 = js::or(bx * bx + bz * bz, 1e-6);
        for x in dop!(g, op).from.keys() {
            if !dop!(g, op).men.contains(&x) && !dead(g, x) { continue; }
            let p = pos(g, x);
            if dead(g, x) && !door_tools_sees(g, op, p) { continue; }
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
        let k = dop!(g, op).threat.as_ref().unwrap().known[i];
        let gone = t - k.t > secs || match k.by {
            Some(by) if dead(g, by) => { let p = pos(g, by); door_tools_sees(g, op, p) }
            _ => false,
        };
        if gone { dop!(g, op).threat.as_mut().unwrap().known.remove(i); }
    }
}

/// One of the order's men still up has a line from his eyes to a man on the floor there. (doortools.ts DoorTools.sees)
pub fn door_tools_sees(g: &mut crate::game::Game, op: DoorOpId, p: V3) -> bool {
    let at: V3 = [p[0], 0.4, p[2]];
    for x in dop!(g, op).men.clone() {
        if dead(g, x) { continue; }
        let eye = eye_of(g, x);
        if !segment_blocked(g, eye, at) { return true; }
    }
    false
}

fn threat_watch(th: crate::doortools::KnownThreat, lead: Option<GuardId>) -> crate::doortools::ThreatWatch {
    crate::doortools::ThreatWatch { th, lead, on: m::NAN, fight: 0.0, last: m::NAN, done_t: m::NAN, faced: None, at: None, still: None }
}

/// The entry's first frame: the known places dealt to the first two men in. (doortools.ts DoorTools.threatDeal)
pub fn door_tools_threat_deal(g: &mut crate::game::Game, op: DoorOpId) {
    match dop!(g, op).threat.as_ref() { Some(th) if th.deal.is_none() => {} _ => return }
    dop!(g, op).threat.as_mut().unwrap().deal = Some(JsMap::new());
    let fc = frame(g, dop!(g, op).door);
    let mut known = dop!(g, op).threat.as_ref().unwrap().known.clone();
    js::sort_by(&mut known, |a, b| v3::dist_xz(a.at, fc) - v3::dist_xz(b.at, fc));
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
        let by = match w.th.by { Some(b) => g.gd[b].callsign.clone(), None => String::from("a man") };
        said.push(format!("{} on {}'s place ({:.1}, {:.1}){}", g.gd[x].callsign, by, w.th.at[0], w.th.at[2], if w.lead.is_some() { " until the first man has it" } else { "" }));
    }
    dop!(g, op).threat.as_mut().unwrap().deal = Some(deal);
    door_tools_beat(g, op, &format!("threat first · {}", said.join(" · ")));
}

/// A man going in whose gun is a known place's: true while it still is. (doortools.ts DoorTools.threatStep)
pub fn door_tools_threat_step(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool {
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
    let th = w.th;
    let spot: V3 = [th.at[0], 1.2, th.at[2]];
    if t - th.t > g.k.doortools.threat_secs { return threat_done(g, op, gd, a, &pod, w, "let go: old news"); }
    if let Some(by) = th.by {
        if dead(g, by) { let p = pos(g, by); if door_tools_sees(g, op, p) { return threat_done(g, op, gd, a, &pod, w, "down"); } }
    }
    if let Some(lead) = w.lead {
        let row = dop!(g, op).threat.as_ref().unwrap().deal.as_ref().unwrap().get(lead).copied();
        if dead(g, lead) || !dop!(g, op).men.contains(&lead) || row.is_none() {
            w.lead = None;
        } else if !row.unwrap().done_t.is_nan() {
            return threat_done(g, op, gd, a, &pod, w, "dealt with by the first man");
        } else if dop!(g, op).crossed.has(lead) {
            let eye = eye_of(g, lead);
            if !segment_blocked(g, eye, spot) { return threat_done(g, op, gd, a, &pod, w, "is the first man's"); }
        }
    }
    asg!(g, a).watch = Some(spot);
    let tx = door_tools_in_at(g, op, gd);
    let p = g.ch[ch].pos;
    let up = over(g, gd);
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
            on = !segment_blocked(g, eye, spot);
        }
        if on { if w.on.is_nan() { w.on = t; } } else { w.on = m::NAN; }
        if !w.on.is_nan() && t - w.on >= g.k.doortools.corner_secs { return threat_done(g, op, gd, a, &pod, w, "looked at: nobody there"); }
        if v3::dist_xz(p, spot) < 1.0 { return threat_done(g, op, gd, a, &pod, w, "stood on"); }
    }
    if t - tx - w.fight > g.k.doortools.threat_cap { return threat_done(g, op, gd, a, &pod, w, "cap"); }
    true
}

/// Who goes in first: the front man who has seen most of the room. (doortools.ts DoorTools.firstIn)
pub fn door_tools_first_in(g: &mut crate::game::Game, op: DoorOpId, at: Option<V3>, by_gun: Option<bool>) -> GuardId {
    let by_gun = by_gun.unwrap_or(true);
    let (lead, d, side) = (dop!(g, op).lead, dop!(g, op).door, dop!(g, op).side);
    let w = g.dr[d].def.width;
    let men = dop!(g, op).men.clone();
    if men.len() < 2 { return lead; }
    if by_gun {
        if let Some(f) = dop!(g, op).first { if !dead(g, f) && men.contains(&f) { return f; } }
    }
    let fc = frame(g, d);
    let mut fronts: Vec<GuardId> = Vec::new();
    for s in [true, false] {
        let mut file: Vec<(GuardId, f64)> = Vec::new();
        for x in men.iter().copied() {
            if dead(g, x) { continue; }
            let there = match dop!(g, op).post.get(x) { Some(p) => p.pos, None => pos(g, x) };
            if (along_of(g, d, there) > w / 2.0) != s { continue; }
            let p = pos(g, x);
            if off_of(g, d, p, side) > 0.0 { file.push((x, v3::dist_xz(there, fc))); }
        }
        js::sort_by(&mut file, |a, b| a.1 - b.1);
        if let Some(f) = file.first() { fronts.push(f.0); }
    }
    let threw = dop!(g, op).threw;
    let hands: Vec<GuardId> = fronts.iter().copied().filter(|x| Some(*x) != threw || *x == lead || fronts.len() < 2).collect();
    let rw = crate::rearwatch::rear_watcher(g, op);
    let able: Vec<GuardId> = match rw {
        Some(r) if hands.iter().any(|x| *x != r) => hands.iter().copied().filter(|x| *x != r).collect(),
        _ => hands,
    };
    let list = if by_gun { crate::gunroles::gun_first(g, &able) } else { able };
    let at = match at { Some(a) => Some(a), None => door_tools_pods_as_were(g, d, side, Some(2.0)).first().map(|p| p.pos) };
    door_tools_seen_most(g, lead, &list, at)
}

/// Of `men`, the one who has seen most of the room `at` is in. (doortools.ts DoorTools.seenMost)
pub fn door_tools_seen_most(g: &mut crate::game::Game, lead: GuardId, men: &[GuardId], at: Option<V3>) -> GuardId {
    let room = match at { Some(a) if g.door_tools.seen.is_some() => crate::roomfight::room_fight_room_at(g, a), _ => None };
    let room = match room {
        Some(r) if !men.is_empty() => r,
        _ => return if men.contains(&lead) || men.is_empty() { lead } else { men[0] },
    };
    let since = g.time - g.k.doortools.lead_seen_secs;
    let cells: Vec<f64> = g.tab.roomfight_room[room].cells.iter().map(|c| c.0).collect();
    let mut best = if men.contains(&lead) { lead } else { men[0] };
    let mut bn = if best == lead {
        crate::floorseen::floor_seen_count(g, lead, &cells, since) + m::max(0.0, g.k.doortools.lead_seen_min - 1.0)
    } else {
        crate::floorseen::floor_seen_count(g, best, &cells, since)
    };
    for x in men.iter().copied() {
        if x == best { continue; }
        let k = crate::floorseen::floor_seen_count(g, x, &cells, since);
        if k > bn { bn = k; best = x; }
    }
    best
}

/// When he came through the frame on this entry; NaN until 'going in'. (doortools.ts DoorTools.inAt)
pub fn door_tools_in_at(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> f64 {
    let o = &dop!(g, op);
    match o.crossed.get(gd) {
        Some(tx) if o.phase == Phase::Enter => m::max(*tx, o.t0),
        _ => m::NAN,
    }
}

/// On his sector, as the entry leaves him. (doortools.ts DoorTools.onSector)
pub fn door_tools_on_sector(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> f64 {
    let p = pos(g, gd);
    let yaw = match pod.watch { Some(w) => v3::yaw_to(p, w), None => pod.face };
    let watch = match pod.watch { Some(w) => [w[0], w[1], w[2]], None => [p[0] + m::sin(yaw) * 4.0, 1.2, p[2] + m::cos(yaw) * 4.0] };
    asg!(g, a).watch = Some(watch);
    asg!(g, a).face = Some(yaw);
    asg!(g, a).there = String::from("holding the room");
    yaw
}

/// The men an ended entry has holding one of its room's other doors. (doortools.ts DoorTools.doorKeepers)
pub fn door_tools_door_keepers(g: &mut crate::game::Game) -> Vec<(GuardId, DoorId, OrdersAssignmentId)> {
    let mut out = Vec::new();
    for (gd, r) in g.door_tools.held.iter() {
        if let Some(d) = r.door { out.push((gd, d, r.a)); }
    }
    out
}

fn sector_hit(g: &mut Game, gd: GuardId, p: V3, y: f64) -> bool {
    crate::guards::colleague_in_line(g, gd, [p[0] + m::sin(y) * 4.0, 1.2, p[2] + m::cos(y) * 4.0], None).is_some()
}

/// A held sector is kept off a mate. (doortools.ts DoorTools.keepSectors)
pub fn door_tools_keep_sectors(g: &mut crate::game::Game) {
    let t = g.time;
    // (a copy of the keys: the body removes only the entry it stands on)
    for gd in g.door_tools.held.keys() {
        let mut r = match g.door_tools.held.get(gd) { Some(r) => *r, None => continue };
        if dead(g, gd) || crate::orders::squad_orders_of(g, gd) != Some(r.a) { g.door_tools.held.delete(gd); continue; }
        if r.door.is_some() && (!crate::nvg::nvg_worn(g, gd) || crate::nvg::nvg_black(g, gd)) { asg!(g, r.a).search_t = Some(t); }
        keep_sector(g, gd, &mut r, t);
        *g.door_tools.held.get_mut(gd).unwrap() = r;
    }
}

fn keep_sector(g: &mut Game, gd: GuardId, r: &mut crate::doortools::DoorToolsHeldValue, t: f64) {
    let a = r.a;
    if asg!(g, a).arrived {
        if r.arr_t.is_nan() { r.arr_t = asg!(g, a).arr_t; } else { asg!(g, a).arr_t = r.arr_t; }
    }
    if !asg!(g, a).arrived || over(g, gd) || r.out.unwrap_or(false) { return; }
    let p = pos(g, gd);
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
    if d != 0.0 {
        g.door_tools.stats.sectors += 1.0;
        let line = format!("{} swings his sector {}° off a mate", g.gd[gd].callsign, m::round(d * 57.3));
        door_tools_note(g, &line);
    }
}

/// Is he on one of this entry's other doors. (doortools.ts DoorTools.onOther)
pub fn door_tools_on_other(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> bool {
    if g.k.doortools.other_door_m > 0.0 {
        if let Some(other) = dop!(g, op).other.as_ref() {
            return other.values().any(|r| r.gd == gd);
        }
    }
    false
}

/// open_floor of a room in the table (the stub wants its own type, of the same cells).
fn room_open(g: &mut Game, room: RoomfightRoomId) -> bool {
    let cells = g.tab.roomfight_room[room].cells.clone();
    open_floor(g, &crate::doortools::OpenFloorRoom { cells })
}
fn opening_of(g: &mut Game, o: DoorId) -> Option<CoverOpeningId> {
    let ready = crate::cover::cover_ready(g);
    ready.into_iter().find(|x| g.tab.cover_opening[*x].door == Some(o))
}
fn inner_of(g: &Game, e: &crate::roomfight::Entry) -> V3 {
    let fc = frame(g, e.door);
    [fc[0] - e.out[0] * 0.4, 1.2, fc[2] - e.out[1] * 0.4]
}
/// otherDoors' `back`: given back, and on the sector his point gave him again.
fn other_back(g: &mut Game, op: DoorOpId, inside: &[GuardId], o: DoorId, gd: GuardId) {
    dop!(g, op).other.as_mut().unwrap().delete(o);
    let h = g.door_tools.held.get(gd).copied();
    let a = dop!(g, op).mine.get(gd).copied();
    let pod = dop!(g, op).pod.get(gd).copied();
    if let (Some(h), Some(a), Some(pod)) = (h, a, pod) {
        if h.a == a && h.door == Some(o) && inside.contains(&gd) {
            let face = door_tools_on_sector(g, gd, a, pod);
            let r = g.door_tools.held.get_mut(gd).unwrap();
            r.face = face;
            r.d = 0.0;
            r.out = Some(false);
            r.door = None;
        }
    }
}
/// otherDoors' `theirs`: another order's door, one somebody else holds, one a teammate was told to watch.
fn other_theirs(g: &mut Game, op: DoorOpId, squad: &[GuardId], o: DoorId, by: Option<GuardId>) -> bool {
    if g.door_tools.ops.iter().any(|x| *x != op && dop!(g, *x).door == o) { return true; }
    if g.door_tools.held.iter().any(|(x, r)| r.door == Some(o) && Some(x) != by) { return true; }
    let fc = frame(g, o);
    for x in squad.iter().copied() {
        if let Some(a) = crate::orders::squad_orders_of(g, x) {
            if asg!(g, a).door == Some(o) { return true; }
            if let Some(w) = asg!(g, a).watch { if v3::dist_xz(w, fc) < 1.5 { return true; } }
        }
    }
    false
}
/// otherDoors' `beside`: a teammate stands by it. (`of(m)?.post !== null`: a man with no order counts as standing.)
fn other_beside(g: &mut Game, squad: &[GuardId], o: DoorId) -> bool {
    let fc = frame(g, o);
    for x in squad.iter().copied() {
        if v3::dist_xz(pos(g, x), fc) < 3.0 {
            match crate::orders::squad_orders_of(g, x) {
                None => return true,
                Some(a) => if asg!(g, a).post.is_some() { return true; },
            }
        }
    }
    false
}

/// The room's other doors, watched: asked every frame of the entry and while its men hold the room. (doortools.ts DoorTools.otherDoors)
pub fn door_tools_other_doors(g: &mut crate::game::Game, op: DoorOpId, ended: Option<bool>, deal: Option<bool>, seen: Option<bool>) -> bool {
    let (ended, deal, seen) = (ended.unwrap_or(false), deal.unwrap_or(true), seen.unwrap_or(false));
    let d = dop!(g, op).door;
    let other_m = g.k.doortools.other_door_m;
    if dop!(g, op).other.is_none() { dop!(g, op).other = Some(JsMap::new()); }
    let mut inside: Vec<GuardId> = Vec::new();
    for x in dop!(g, op).men.clone() {
        if dead(g, x) || !dop!(g, op).pod.has(x) { continue; }
        let mine = dop!(g, op).mine.get(x).copied();
        // (`of` answers null, `mine.get` undefined: no order on either side is not a match)
        let of = crate::orders::squad_orders_of(g, x);
        if of.is_none() || of != mine { continue; }
        if ended && g.door_tools.held.get(x).map(|h| h.a) != mine { continue; }
        inside.push(x);
    }
    let rows = |g: &Game| -> Vec<(DoorId, crate::doortools::DoorToolsDoneItemOtherValue)> { dop!(g, op).other.as_ref().unwrap().iter().map(|(o, r)| (o, *r)).collect() };
    if !(other_m > 0.0) {
        for (o, r) in rows(g) { other_back(g, op, &inside, o, r.gd); }
        return false;
    }
    for (o, r) in rows(g) {
        if !inside.contains(&r.gd) { dop!(g, op).other.as_mut().unwrap().delete(o); }
    }
    if inside.is_empty() { return false; }
    let room = door_tools_room_in(g, op);
    let open = match room { Some(r) => room_open(g, r), None => true };
    if let (Some(room), false) = (room, open) {
        let mut squad: Vec<GuardId> = Vec::new();
        for x in g.guards.clone() {
            if g.gd[x].team == crate::teams::Team::Operative && g.gd[x].unit.is_some() && !dead(g, x) && !inside.contains(&x) { squad.push(x); }
        }
        let entries = g.tab.roomfight_room[room].entries.clone();
        for (o, r) in rows(g) {
            let e = entries.iter().find(|x| x.door == o);
            let ok = opening_of(g, o);
            let drop = match (e, ok) {
                (Some(e), Some(ok)) => {
                    let at = inner_of(g, e);
                    crate::entrypoints::entry_points_far_known(g, ok, at) || other_theirs(g, op, &squad, o, Some(r.gd)) || (seen && { let p = pos(g, r.gd); door_tools_leaf_across(g, o, p, at) > 0.0 })
                }
                _ => true,
            };
            if drop {
                other_back(g, op, &inside, o, r.gd);
                let line = format!("other door · {} off {}", g.gd[r.gd].callsign, g.dr[o].def.name);
                door_tools_beat(g, op, &line);
            }
        }
        let mut free: Vec<GuardId> = Vec::new();
        if deal {
            for x in inside.iter().copied() {
                let h = g.door_tools.held.get(x).copied();
                let arrived = dop!(g, op).mine.get(x).copied().map_or(false, |a| asg!(g, a).arrived);
                if (dop!(g, op).on_pod.has(x) || (ended && arrived)) && !door_tools_on_other(g, op, x) && !h.map_or(false, |h| h.out.unwrap_or(false) || h.door.is_some()) && g.time >= g.gd[x].dazzled_until { free.push(x); }
            }
        }
        let mut still = 0usize;
        for x in dop!(g, op).men.clone() {
            if dead(g, x) { continue; }
            let of = crate::orders::squad_orders_of(g, x);
            if of.is_some() && of == dop!(g, op).mine.get(x).copied() { still += 1; }
        }
        let spare = if still < 3 { 0.0 } else { m::floor(inside.len() as f64 / 2.0) };
        // (the sort's rank, worked out once: nothing moves while it runs)
        let mut ranked: Vec<(f64, crate::roomfight::Entry)> = Vec::new();
        for e in entries.iter() {
            let fc = frame(g, e.door);
            let dists: Vec<f64> = inside.iter().map(|x| v3::dist_xz(pos(g, *x), fc)).collect();
            let rank = (if g.dr[e.door].def.exterior.unwrap_or(false) { 100.0 } else { 0.0 }) + m::min_n(&dists);
            ranked.push((rank, e.clone()));
        }
        js::sort_by(&mut ranked, |a, b| a.0 - b.0);
        for (_, e) in ranked.iter() {
            let o = e.door;
            let fc = frame(g, o);
            if o == d || dop!(g, op).other.as_ref().unwrap().has(o) || free.is_empty() { continue; }
            if !ended && dop!(g, op).other.as_ref().unwrap().len() as f64 >= spare { break; }
            let ok = opening_of(g, o);
            let at = inner_of(g, e);
            let ok = match ok { Some(ok) => ok, None => continue };
            if crate::entrypoints::entry_points_far_known(g, ok, at) || other_theirs(g, op, &squad, o, None) || other_beside(g, &squad, o) { continue; }
            if blocked_at(g, fc[0] + e.out[0] * 0.9, fc[2] + e.out[1] * 0.9) && blocked_at(g, fc[0] + e.out[0] * 1.5, fc[2] + e.out[1] * 1.5) { continue; }
            let mut can: Vec<(GuardId, f64, f64)> = Vec::new();
            for x in free.iter().copied() {
                let p = pos(g, x);
                if v3::dist_xz(p, fc) > other_m || segment_blocked(g, [p[0], 1.5, p[2]], at) { continue; }
                let leaf = door_tools_leaf_across(g, o, p, at);
                if seen && leaf > 0.0 { continue; }
                can.push((x, leaf, v3::dist_xz(p, fc)));
            }
            js::sort_by(&mut can, |a, b| js::or(a.1 - b.1, a.2 - b.2));
            let gd = match can.first() { Some(c) => c.0, None => continue };
            let beyond: V3 = [fc[0] + e.out[0], 1.2, fc[2] + e.out[1]];
            let p = pos(g, gd);
            let w = if segment_blocked(g, [p[0], 1.5, p[2]], beyond) { [fc[0], 1.2, fc[2]] } else { beyond };
            dop!(g, op).other.as_mut().unwrap().set(o, crate::doortools::DoorToolsDoneItemOtherValue { gd, w });
            free.retain(|x| *x != gd);
            let line = format!("other door · {} on {}", g.gd[gd].callsign, g.dr[o].def.name);
            door_tools_beat(g, op, &line);
        }
    } else if let Some(room) = room {
        if g.tab.roomfight_room[room].cells.len() as f64 <= g.k.doortools.search_cells {
            for (o, r) in rows(g) { other_back(g, op, &inside, o, r.gd); }
        }
    }
    for (o, r) in rows(g) {
        let a = *dop!(g, op).mine.get(r.gd).unwrap();
        let yaw = v3::yaw_to(pos(g, r.gd), r.w);
        let words = crate::orders::door_words(g, o);
        asg!(g, a).watch = Some(r.w);
        asg!(g, a).face = Some(yaw);
        asg!(g, a).there = format!("holding · {}", words);
        if let Some(h) = g.door_tools.held.get_mut(r.gd) {
            h.face = yaw;
            h.d = 0.0;
            h.out = Some(true);
            h.door = Some(o);
        }
    }
    true
}

/// leafAcross's `cut`: does the leaf at angle `a` lie across the line from p to the point held?
fn leaf_cut(closed_dir: f64, a: f64, w: f64, hx: f64, hz: f64, p: V3, rx: f64, rz: f64) -> f64 {
    let th = closed_dir + a;
    let (lx, lz) = (m::cos(th) * w, m::sin(th) * w);
    let den = rx * lz - rz * lx;
    if m::abs(den) < 1e-9 { return 0.0; }
    let s = ((hx - p[0]) * lz - (hz - p[2]) * lx) / den;
    let u = ((hx - p[0]) * rz - (hz - p[2]) * rx) / den;
    if s > 0.0 && s < 1.0 && u > 0.0 && u < 1.0 { 1.0 } else { 0.0 }
}

/// How much door `o`'s own leaf, stood open, would hide the door from a man at `p`. (doortools.ts DoorTools.leafAcross)
pub fn door_tools_leaf_across(g: &mut crate::game::Game, o: DoorId, p: V3, at: V3) -> f64 {
    if crate::doors::door_off(g, o) { return 0.0; }
    let stop = if crate::doors::door_swing(g, o) > 0.0 { crate::doors::door_max_a(g, o) } else { crate::doors::door_min_a(g, o) };
    let (sg, top) = (m::sign(stop), m::abs(stop));
    let def = &g.dr[o].def;
    let (w, hx, hz, closed) = (def.width + LEAF_EDGE, def.hinge[0], def.hinge[1], def.closed_dir);
    let (rx, rz) = (at[0] - p[0], at[2] - p[2]);
    let mut n = 1.0;
    let mut hid = leaf_cut(closed, stop, w, hx, hz, p, rx, rz);
    let mut a = m::min(LEAF_FROM, top);
    while a < top {
        n += 1.0;
        hid += leaf_cut(closed, sg * a, w, hx, hz, p, rx, rz);
        a += LEAF_STEP;
    }
    hid / n
}

/// The room's other doors, held: a threshold stage's last act before 'in'. (doortools.ts DoorTools.holdDoors)
pub fn door_tools_hold_doors(g: &mut crate::game::Game, op: DoorOpId) {
    let (d, side) = (dop!(g, op).door, dop!(g, op).side);
    let room = match door_tools_room_in(g, op) { Some(r) => r, None => return };
    if room_open(g, room) { return; }
    let all_men = dop!(g, op).men.clone();
    let mut men: Vec<GuardId> = all_men.iter().copied().filter(|x| !dead(g, *x) && g.door_tools.held.get(*x).map_or(false, |h| !h.out.unwrap_or(false))).collect();
    if men.is_empty() { return; }
    let ours: Vec<GuardId> = g.guards.iter().copied().filter(|x| g.gd[*x].team == crate::teams::Team::Operative && g.gd[*x].unit.is_some() && !dead(g, *x)).collect();
    let mut stood: JsMap<GuardId, V3> = JsMap::new();
    for x in ours.iter().copied() { stood.set(x, pos(g, x)); }
    let sam = crate::orders::squad_orders_sam_stood(g);
    let fc0 = frame(g, d);
    let mut doors: Vec<crate::roomfight::Entry> = Vec::new();
    for e in g.tab.roomfight_room[room].entries.clone() {
        if e.door == d { continue; }
        let o = e.door;
        // (taken: another order's door, a door already held, a door somebody of ours stands on)
        let fc = frame(g, o);
        let taken = g.door_tools.ops.iter().any(|x| *x != op && dop!(g, *x).door == o) || g.door_tools.held.values().any(|r| r.door == Some(o)) || ours.iter().any(|x| !all_men.contains(x) && v3::dist_xz(pos(g, *x), fc) < 3.0);
        if !taken { doors.push(e); }
    }
    js::sort_by(&mut doors, |a, b| v3::dist_xz(b.at, fc0) - v3::dist_xz(a.at, fc0));
    let kept = all_men.iter().any(|x| !dead(g, *x) && g.door_tools.held.get(*x).map_or(false, |r| r.out.unwrap_or(false) && r.door.is_none()));
    let stay = if kept { 0.0 } else { g.k.doortools.thr_doors_stay };
    let mut said: Vec<String> = Vec::new();
    let mut dealt: JsMap<GuardId, V3> = JsMap::new();
    let (doors_m, doors_walk) = (g.k.doortools.thr_doors_m, g.k.doortools.thr_doors_walk);
    for e in doors.iter() {
        if men.len() as f64 <= stay { break; }
        let o = e.door;
        let w_o = g.dr[o].def.width;
        let fc = frame(g, o);
        let at: V3 = [fc[0] - e.out[0] * 0.4, 1.2, fc[2] - e.out[1] * 0.4];
        let w: V3 = [fc[0] + e.out[0], 1.2, fc[2] + e.out[1]];
        let rs = crate::orders::side_of(g, o, [fc[0] - e.out[0], 0.0, fc[2] - e.out[1]]);
        let stop = if crate::doors::door_swing(g, o) > 0.0 { crate::doors::door_max_a(g, o) } else { crate::doors::door_min_a(g, o) };
        let in_swing = !crate::doors::door_off(g, o) && m::sign(stop) == rs;
        // a man with a line to its frame from where he stands
        let mut near: Vec<(GuardId, f64)> = Vec::new();
        for x in men.iter().copied() {
            let p = pos(g, x);
            if v3::dist_xz(p, fc) > doors_m || segment_blocked(g, [p[0], 1.5, p[2]], at) { continue; }
            if js::truthy(door_tools_leaf_across(g, o, p, at)) { continue; }
            near.push((x, v3::dist_xz(p, fc)));
        }
        js::sort_by(&mut near, |a, b| a.1 - b.1);
        let mut gd: Option<GuardId> = near.first().map(|n| n.0);
        let mut post: Option<V3> = None;
        // (buddy on, buddyDoors off on the page: the walk's own loop ends on its first pass, and no pair is sent)
        if gd.is_none() {
            let mut best = m::INF;
            let mut k = 0.0;
            let mut rows: Vec<Vec<f64>> = vec![vec![1.5, 2.6], vec![2.0, 2.2], vec![1.1, 3.2], vec![2.4, 3.0], vec![1.8, 3.8], vec![0.9, 2.2]];
            if in_swing { rows.extend(leaf_posts()); }
            for row in rows.iter() {
                k += 1.0;
                for sgn in [1.0, -1.0] {
                    let p = door_pt(g, o, w_o / 2.0 + sgn * (w_o / 2.0 + row[0]), row[1], rs, None);
                    if blocked_at(g, p[0], p[2]) { continue; }
                    let cell = g.col.nav.cell;
                    let key = crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell));
                    if !g.tab.roomfight_room[room].cells.has(crate::Num(key)) || segment_blocked(g, [p[0], 1.5, p[2]], at) { continue; }
                    let hid = door_tools_leaf_across(g, o, p, at);
                    let behind = if js::truthy(hid) { LEAF_COST * (1.0 + hid) } else { 0.0 };
                    for x in men.iter().copied() {
                        let c = v3::dist_xz(pos(g, x), p) + 0.4 * k + behind;
                        let clear = v3::dist_xz(sam, p) >= 1.0 && !stood.iter().any(|(y, q)| y != x && v3::dist_xz(*q, p) < 1.0);
                        if c < best && clear { best = c; gd = Some(x); post = Some(p); }
                    }
                }
            }
            match (gd, post) {
                (Some(x), Some(to)) => {
                    let from = pos(g, x);
                    let walk = match crate::scene_collision::nav_grid_find_path(g, from, to, None, None) {
                        None => m::INF,
                        Some(path) => { let (mut l, mut q) = (0.0, from); for r in path.iter().copied() { l += v3::dist_xz(q, r); q = r; } l }
                    };
                    if walk > doors_walk { gd = None; }
                }
                _ => gd = None,
            }
        }
        let gd = match gd { Some(x) => x, None => continue };
        men.retain(|x| *x != gd);
        if let Some(to) = post {
            let mut taken: Vec<V3> = Vec::new();
            for (x, pd) in dop!(g, op).pod.iter() { if x != gd && !dead(g, x) { taken.push(pd.pos); } }
            for x in ours.iter().copied() {
                if x == gd { continue; }
                let there = match dealt.get(x) {
                    Some(q) => *q,
                    None => {
                        let of_post = crate::orders::squad_orders_of(g, x).and_then(|a| asg!(g, a).post);
                        match of_post { Some(q) => q, None => pos(g, x) }
                    }
                };
                taken.push(there);
            }
            // (offSwing moves P.pos in place, and P.pos is `post`)
            let mut moved = crate::doortools::DoorToolsOffSwingP { pos: to, watch: None };
            door_tools_off_swing(g, &mut moved, Some(d), side, &taken, Some(at));
            post = Some(moved.pos);
        }
        let from = match post { Some(p) => p, None => pos(g, gd) };
        let yaw = v3::yaw_to(from, w);
        let name = crate::orders::door_words(g, o);
        if post.is_some() { g.door_tools.stats.sent_on += 1.0; }
        if let Some(to) = post {
            dop!(g, op).pod.set(gd, crate::entrypoints::Pod { pos: to, face: yaw, watch: None, moved: false });
            dop!(g, op).on_pod.delete(gd);
            stood.set(gd, to);
            dealt.set(gd, to);
            // (DoorTools.assign inlined: its `shy: null` overrides the `shy: op.door` it adds, and DoorToolsAssignA.shy cannot say null)
            let order = String::from(dop!(g, op).kind.as_str());
            let pace = g.orders.knobs.entry;
            // (NIGHTBREACH.opLights: true on the page)
            let light = g.door_tools.lights.unwrap_or(true);
            crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA { order, door: Some(d), shy: None, post: Some(to), face: Some(yaw), watch: Some(w), thru: Some(d), pace: Some(pace), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(light), search: Some(true), going: format!("to the far door · {}", name), there: format!("holding · {}", name), ..Default::default() });
            let a = crate::orders::squad_orders_of(g, gd).unwrap();
            dop!(g, op).mine.set(gd, a);
        }
        let a = *dop!(g, op).mine.get(gd).unwrap();
        asg!(g, a).watch = Some(w);
        asg!(g, a).face = Some(yaw);
        asg!(g, a).there = format!("holding · {}", name);
        g.door_tools.held.set(gd, crate::doortools::DoorToolsHeldValue { a, face: yaw, d: 0.0, hit_t: m::NAN, arr_t: m::NAN, out: Some(true), door: Some(o) });
        said.push(format!("{} {} {}", g.gd[gd].callsign, if post.is_some() { "up to" } else { "on" }, g.dr[o].def.name));
    }
    if !said.is_empty() { door_tools_beat(g, op, &format!("doors · {}", said.join(" · "))); }
}

/// The room this entry goes into, as the room fight has it. (doortools.ts DoorTools.roomIn)
pub fn door_tools_room_in(g: &mut crate::game::Game, op: DoorOpId) -> Option<RoomfightRoomId> {
    let at = match dop!(g, op).queue.first() {
        Some(q) => q.pod.pos,
        None => dop!(g, op).pod.values().next()?.pos,
    };
    crate::roomfight::room_fight_room_at(g, at)
}

/// A man's corners: the near one on his own side of the door, then that side's deep one. (doortools.ts DoorTools.cornersOf)
pub fn door_tools_corners_of(g: &mut crate::game::Game, op: DoorOpId, pod: crate::entrypoints::Pod) -> Vec<V3> {
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

/// The room this entry is into as floor to look over, built once. (doortools.ts DoorTools.searchOf)
pub fn door_tools_search_of<'g>(g: &'g mut crate::game::Game, op: DoorOpId, at: Option<V3>) -> Option<&'g mut crate::doortools::RoomSearch> {
    if seam_search_asked(g, op) { return dop!(g, op).search.as_mut(); }
    dop!(g, op).search = None;
    seam_search_mark_asked(g, op);
    let room = match at { Some(a) => crate::roomfight::room_fight_room_at(g, a), None => door_tools_room_in(g, op) };
    let room = room?;
    if room_open(g, room) { return None; }
    for o in g.door_tools.ops.clone() {
        if o != op && dop!(g, o).search.as_ref().map_or(false, |s| s.room == room) { return seam_search_share(g, op, o); }
    }
    let (nav_w, cs) = (g.col.nav.w, g.col.nav.cell);
    let pts: Vec<V3> = g.tab.roomfight_room[room].cells.iter().map(|ci| [(js::rem(ci.0, nav_w) + 0.5) * cs, 1.0, (js::to_i32(ci.0 / nav_w) as f64 + 0.5) * cs]).collect();
    let n = pts.len();
    let found = crate::doortools::RoomSearch { room, pts, looked: vec![0u8; n], los: JsMap::new(), yaw: JsMap::new(), at: JsMap::new(), moves: 0.0, left_at_move: m::INF, left: n as f64, flows: None, closes: None, flow_said: None, late: None, cell_at: None, fight_t: None, fight_at: None };
    dop!(g, op).search = Some(found);
    dop!(g, op).search.as_mut()
}
