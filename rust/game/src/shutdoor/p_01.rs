//! src/game/shutdoor.ts lines 1 to 231
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `x.toFixed(d)`: −0 prints as 0, as there. For a log's words and a cache's key only.
fn to_fixed(x: f64, d: usize) -> String {
    let x = if x == 0.0 { 0.0 } else { x };
    alloc::format!("{:.*}", d, x)
}
/// where a man stands (his body's position)
fn pos_of(g: &Game, gd: GuardId) -> V3 {
    g.ch[g.gd[gd].ch].pos
}

/// Distance on the floor. (shutdoor.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// A floor point in the door's frame: `along` from the hinge, `off` out on face `side`. (shutdoor.ts ptOf)
pub fn pt_of(g: &mut crate::game::Game, d: DoorId, along: f64, off: f64, side: f64) -> V3 {
    let f = crate::guards::frame_of(g, d);
    [f.hx + f.ux * along + f.nx * side * off, 0.0, f.hz + f.uz * along + f.nz * side * off]
}

/// Where the door's handle comes to. (shutdoor.ts handleOf)
pub fn handle_of(g: &mut crate::game::Game, d: DoorId) -> V3 {
    let w = g.dr[d].def.width;
    pt_of(g, d, w - k().handle_in, 0.0, 1.0)
}

/// A line of the order's log, newest last. (shutdoor.ts note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, d: DoorId, what: &str) {
    let row = ShutdoorModSHUTDOORLOGItem { t: g.time, who: g.gd[gd].callsign.clone(), door: g.dr[d].def.name.clone(), what: String::from(what) };
    let log = &mut g.m.shutdoor.shutdoor_log;
    log.push(row);
    if log.len() > 200 { log.drain(0..50); }
}

/// Does d stand open (off its hinges it reads wide open: its row is there, greyed). (shutdoor.ts standsOpen)
pub fn stands_open(g: &mut crate::game::Game, d: DoorId) -> bool {
    !crate::doors::door_latched(g, d) && m::abs(crate::doors::door_angle(g, d)) >= k().open
}

/// The floor the leaf crosses on its way shut from where it stands now, with `pad` m of slack. (shutdoor.ts inShutSwing)
pub fn in_shut_swing(g: &mut crate::game::Game, d: DoorId, x: f64, z: f64, pad: Option<f64>) -> bool {
    let pad = pad.unwrap_or(0.0);
    let f = crate::guards::frame_of(g, d);
    let (px, pz) = (x - f.hx, z - f.hz);
    let r = m::hyp2(px, pz);
    if r > f.w + pad { return false; }
    let along = px * f.ux + pz * f.uz;
    let t = m::max(0.0, m::min(f.w, along));
    if m::hyp2(px - f.ux * t, pz - f.uz * t) <= pad { return true; }   // on the frame's line: where the leaf comes to
    let angle = crate::doors::door_angle(g, d);
    let th = v3::wrap_angle(m::atan2(pz, px) - g.dr[d].def.closed_dir) * js::or(m::sign(angle), 1.0);
    th >= 0.0 && th <= m::abs(angle)
}

/// A wall, a sill or a pane between two floor points at shin height, the level's statics only. (shutdoor.ts walled)
pub fn walled(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    let keep = core::mem::take(&mut g.col.dynamic_boxes);
    let blocked = crate::scene_collision::solid_blocked(g, [a[0], 0.45, a[2]], [b[0], 0.45, b[2]], None);
    g.col.dynamic_boxes = keep;
    blocked
}

/// Can a man stand at p: a cell the walkers' grid calls free, nothing of the level inside his body. (shutdoor.ts stands)
pub fn stands(g: &mut crate::game::Game, p: V3) -> bool {
    if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return false; }
    let mut q: V3 = [p[0], 0.0, p[2]];
    crate::scene_collision::collide_circle(g, &mut q, 0.3, 0.2, 1.5, None);
    flat(p, q) < 0.02
}

/// shutSpot's `ok`: clear of the leaf's way, a place to stand, nothing of the level between it and the door.
fn spot_ok(g: &mut Game, d: DoorId, ref_: V3, p: V3, pad: f64) -> bool {
    !in_shut_swing(g, d, p[0], p[2], Some(pad)) && stands(g, p) && !walled(g, ref_, p)
}

/// The place d is shut from on face `side`, as it stands now; None: none. (shutdoor.ts shutSpot)
pub fn shut_spot(g: &mut crate::game::Game, d: DoorId, side: f64) -> Option<crate::shutdoor::Spot> {
    let w = g.dr[d].def.width;
    let hd = handle_of(g, d);
    let ref_ = pt_of(g, d, w - k().handle_in, 0.2, side);
    let pad = k().body + k().slack;
    let face = |p: V3| m::atan2(hd[0] - p[0], hd[2] - p[2]);
    let mut best: Option<V3> = None;
    let mut bc = f64::INFINITY;
    let mut a = w - 0.5;
    while a <= w + 0.8 + 1e-6 {
        let mut off = 0.4;
        while off <= 1.0 + 1e-6 {
            let p = pt_of(g, d, a, off, side);
            let r = flat(p, hd);
            if !(r > k().reach) {
                let c = r + (if a < w { 0.25 } else { 0.0 });
                if !(c >= bc || !spot_ok(g, d, ref_, p, pad)) {
                    best = Some(p);
                    bc = c;
                }
            }
            off += 0.05;
        }
        a += 0.1;
    }
    if let Some(p) = best { return Some(Spot { pos: p, yaw: face(p), side, far: false }); }
    let mut a = w / 2.0;
    while a <= w + 0.4 + 1e-6 {
        let mut off = 0.5;
        while off <= 1.8 + 1e-6 {
            let p = pt_of(g, d, a, off, side);
            let r = flat(p, hd);
            if !(r > k().far_reach || r >= bc || !spot_ok(g, d, ref_, p, pad)) {
                best = Some(p);
                bc = r;
            }
            off += 0.05;
        }
        a += 0.1;
    }
    best.map(|p| Spot { pos: p, yaw: face(p), side, far: true })
}

/// Both faces' places for the leaf as it stands, kept for the frame. (shutdoor.ts spotsOf)
pub fn spots_of(g: &mut crate::game::Game, d: DoorId) -> Vec<Option<crate::shutdoor::Spot>> {
    let angle = crate::doors::door_angle(g, d);
    let key = alloc::format!("{}|{}|{}", g.time, g.dr[d].def.name, to_fixed(angle, 3));
    if let Some(hit) = &g.m.shutdoor.spotted {
        if hit.key == key { return hit.spots.clone(); }
    }
    let first = shut_spot(g, d, 1.0);
    let second = shut_spot(g, d, -1.0);
    let spots = vec![first, second];
    g.m.shutdoor.spotted = Some(ShutdoorModSpottedValue { key, spots: spots.clone() });
    spots
}

/// The length of his walk to p by the walkers' grid (infinity: no way). (shutdoor.ts walk)
pub fn walk(g: &mut crate::game::Game, gd: GuardId, p: V3) -> f64 {
    let at = pos_of(g, gd);
    if flat(at, p) < 0.5 { return flat(at, p); }
    let path = match crate::scene_collision::nav_grid_find_path(g, at, p, None, None) { Some(path) => path, None => return f64::INFINITY };
    let mut l = 0.0;
    let mut q = at;
    for w in path {
        l += flat(q, w);
        q = w;
    }
    l + flat(q, p)
}

/// Was he taken from Sam's file (or from another door of this order's, taken from the file for that one). (shutdoor.ts fromFile)
pub fn from_file(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let a = match crate::orders::squad_orders_of(g, gd) { Some(a) => a, None => return false };
    let row = &g.tab.orders_assignment[a];
    row.post.is_none() || (row.order == SHUT_ID && g.m.shutdoor.jobs.get(gd).map_or(false, |j| j.back))
}

/// Who shuts it, from where: the shortest walk to a place it can be shut from, or why none of them can. (shutdoor.ts plan)
pub fn plan(g: &mut crate::game::Game, d: DoorId, men: &[GuardId]) -> crate::shutdoor::Plan {
    let spots = spots_of(g, d);
    if spots[0].is_none() && spots[1].is_none() { return Plan::Text(String::from("no place to shut it from.")); }
    let mut out: Option<PlanArm1> = None;
    let mut bc = f64::INFINITY;
    let mut why: Option<String> = None;
    let sam_at = g.ch[g.pl[g.player].ch].pos;
    let sam = crate::orders::side_of(g, d, sam_at);
    for &gd in men {
        let want = if from_file(g, gd) {
            sam
        } else {
            let at = pos_of(g, gd);
            crate::orders::side_of(g, d, at)
        };
        for spot in &spots {
            let spot = match spot { Some(spot) => *spot, None => continue };
            let c = walk(g, gd, spot.pos) + (if spot.side == want { 0.0 } else { k().cross });
            if c >= bc { continue; }
            // (a door locked against the team on the way: not his walk)
            if let Some(w) = crate::orders::squad_orders_reach(g, gd, spot.pos).filter(|w| !w.is_empty()) {
                if why.is_none() { why = Some(w); }
                continue;
            }
            out = Some(PlanArm1 { gd, spot });
            bc = c;
        }
    }
    match out {
        Some(arm) => Plan::Arm1(arm),
        None => Plan::Text(why.unwrap_or_else(|| String::from("can't get there."))),
    }
}

/// inTheWay's `body`: a body lying in the leaf's way (the particles a leaf feels).
fn body_in_swing(g: &mut Game, d: DoorId, c: CharId) -> bool {
    let rag = match g.ch[c].ragdoll { Some(rag) => rag, None => return false };
    if flat(g.ch[c].pos, g.dr[d].frame_centre) > 4.0 { return false; }
    let r = crate::ragdoll::rag();
    for i in [r.pelvis, r.spine, r.head, r.knee_l, r.knee_r, r.ank_l, r.ank_r] {
        let p = crate::ragdoll::ragdoll_point(g, rag, i, None);
        let radius = crate::ragdoll::ragdoll_radius(g, rag, i);
        if in_shut_swing(g, d, p[0], p[2], Some(radius)) { return true; }
    }
    false
}

/// Who or what is in the leaf's way shut (None: nothing); `seen`: the menu's question, an enemy the picture does not show is not given away. (shutdoor.ts inTheWay)
pub fn in_the_way(g: &mut crate::game::Game, d: DoorId, but: Option<GuardId>, seen: Option<bool>) -> Option<String> {
    let seen = seen.unwrap_or(false);
    let player = g.player;
    let sam_ch = g.pl[player].ch;
    let sam_at = g.ch[sam_ch].pos;
    if !g.pl[player].down && in_shut_swing(g, d, sam_at[0], sam_at[2], Some(k().body)) { return Some(String::from("you're in its way.")); }
    let mut all = g.guards.clone();
    all.extend(crate::civilians::civilians_shown(g));
    for m in all {
        if Some(m) == but { continue; }
        let c = g.gd[m].ch;
        if g.gd[m].state == crate::game::GuardState::Dead {
            if body_in_swing(g, d, c) { return Some(String::from("a body's in its way.")); }
            continue;
        }
        let at = g.ch[c].pos;
        if !in_shut_swing(g, d, at[0], at[2], Some(k().body))
            || (seen && g.gd[m].team != crate::teams::Team::Operative && !crate::fogofwar::fog_of_war_drawn(g, m)) {
            continue;
        }
        return Some(if g.gd[m].team == crate::teams::Team::Operative {
            alloc::format!("{} is in its way.", g.gd[m].callsign)
        } else {
            String::from("somebody's in its way.")
        });
    }
    if g.pl[player].down && body_in_swing(g, d, sam_ch) { Some(String::from("a body's in its way.")) } else { None }
}

/// An enemy somebody of the team has in sight right now through that doorway. (shutdoor.ts contactThrough)
pub fn contact_through(g: &mut crate::game::Game, d: DoorId) -> bool {
    let player = g.player;
    let sam_ch = g.pl[player].ch;
    for h in g.guards.clone() {
        let team = g.gd[h].team;
        if g.gd[h].state == crate::game::GuardState::Dead || !crate::teams::is_hostile(g, crate::teams::Team::Operative, team) { continue; }
        for m in crate::roomfight::room_fight_seen_by(g, h) {
            let (m_at, h_at) = (pos_of(g, m), pos_of(g, h));
            if crate::guards::crosses_door(g, m_at, h_at, d).is_some() { return true; }
        }
        let (sam_at, h_at) = (g.ch[sam_ch].pos, pos_of(g, h));
        if !g.pl[player].down
            && crate::guards::crosses_door(g, sam_at, h_at, d).is_some()
            && flat(sam_at, h_at) < 30.0
            && crate::fogofwar::fog_of_war_drawn(g, h)
            && !crate::scene_collision::segment_blocked(g, [sam_at[0], 1.5, sam_at[2]], [h_at[0], 1.2, h_at[2]], None, None, None) {
            return true;
        }
    }
    false
}

/// Why these men cannot shut that door (None: one of them can): the row's `valid`. (shutdoor.ts whyNotShut)
pub fn why_not_shut(g: &mut crate::game::Game, d: DoorId, men: &[GuardId]) -> Option<String> {
    if crate::doors::door_off(g, d) { return Some(String::from("it's off its hinges.")); }
    if crate::doors::door_latched(g, d) { return Some(String::from("it's shut.")); }
    if !stands_open(g, d) { return Some(String::from("it's all but shut.")); }
    if contact_through(g, d) { return Some(String::from("contact through that door.")); }
    match plan(g, d, men) {
        Plan::Text(why) => Some(why),
        Plan::Arm1(p) => in_the_way(g, d, Some(p.gd), Some(true)),
    }
}

/// He is sent to a place of the door's. (shutdoor.ts send)
pub fn send(g: &mut crate::game::Game, gd: GuardId, d: DoorId, spot: crate::shutdoor::Spot) {
    let name = crate::orders::door_words(g, d);
    let fc = g.dr[d].frame_centre;
    let a = crate::orders::SquadOrdersAssignA {
        order: String::from(SHUT_ID),
        post: Some(spot.pos),
        face: Some(spot.yaw),
        watch: Some([fc[0], 1.05, fc[2]]),
        door: Some(d),
        stance: Some(crate::character::Stance::None),
        upper: Some(crate::character::UpperMode::Aim),
        going: alloc::format!("shutting · {}", name),
        there: alloc::format!("at · {}", name),
        ..Default::default()
    };
    crate::orders::squad_orders_assign(g, gd, &a);
}

/// His hand on it: the leaf is pulled to; a leaf somebody propped is let go of. (shutdoor.ts pull)
pub fn pull(g: &mut crate::game::Game, d: DoorId, gd: GuardId) {
    if crate::doors::door_quiet_t(g, d) < 0.0 { crate::doors::door_set_quiet_t(g, d, 0.0); }
    let who = g.ch[g.gd[gd].ch].id;
    crate::doors::doors_pull_to(g, d, who);
}

/// His order is over: back into the file if that is where he was taken from, else he holds where he stands. (shutdoor.ts finish)
pub fn finish(g: &mut crate::game::Game, gd: GuardId, j: Option<crate::shutdoor::Job>) {
    g.m.shutdoor.jobs.delete(gd);
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    if j.map_or(false, |j| j.back) {
        let a = crate::orders::SquadOrdersAssignA {
            order: String::from("fallIn"),
            post: None,
            follow: Some(99.0),
            face: None,
            going: String::from("falling in"),
            there: String::from("with you"),
            ..Default::default()
        };
        crate::orders::squad_orders_assign(g, gd, &a);
        g.stats.shutdoor.to_file += 1;
        return;
    }
    let there = match j {
        Some(j) => alloc::format!("holding · by {}", crate::orders::door_words(g, j.door)),
        None => String::from("holding"),
    };
    let a = crate::orders::SquadOrdersAssignA {
        order: String::from("hold"),
        post: Some([p[0], 0.0, p[2]]),
        face: Some(g.ch[c].body_yaw),
        going: String::from("holding"),
        there,
        ..Default::default()
    };
    crate::orders::squad_orders_assign(g, gd, &a);
    g.stats.shutdoor.stayed += 1;
}

/// tick's `end`: the word, the log, the count, and his order is over.
fn end(g: &mut Game, gd: GuardId, d: DoorId, j: Job, line: Option<&str>, what: &str, shut: bool) {
    if let Some(line) = line { crate::game::game_say(g, gd, line, None, None, None); }
    note(g, gd, d, what);
    if shut { g.stats.shutdoor.shut += 1; } else { g.stats.shutdoor.gave_up += 1; }
    finish(g, gd, Some(j));
}
/// tick's `clearOf`: is p out of the leaf's way
fn clear_of(g: &mut Game, d: DoorId, p: V3) -> bool {
    !in_shut_swing(g, d, p[0], p[2], Some(k().body))
}
/// The job is one row of the table, written in place in the TypeScript: every change of the local copy goes back to it.
fn store(g: &mut Game, gd: GuardId, j: Job) {
    if let Some(row) = g.m.shutdoor.jobs.get_mut(gd) { *row = j; }
}

/// The order's stages, once a frame for its man before he is posed. (shutdoor.ts tick)
pub fn tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) {
    let mut j = match g.m.shutdoor.jobs.get(gd).copied() {
        Some(j) if g.tab.orders_assignment[a].door == Some(j.door) => j,
        _ => { finish(g, gd, None); return; }   // (a guard: the order's issue sets both)
    };
    let d = j.door;
    let p = pos_of(g, gd);
    let t = g.time;
    if crate::doors::door_off(g, d) { end(g, gd, d, j, Some("door's gone."), "off its hinges", false); return; }
    if crate::doors::door_latched(g, d) {
        if j.phase == JobPhase::Go {
            end(g, gd, d, j, Some("it's shut."), "shut before he got there", true);
        } else {
            end(g, gd, d, j, Some("door's shut."), &alloc::format!("shut · {} s", to_fixed(t - j.t0, 1)), true);
        }
        return;
    }
    if j.phase == JobPhase::Go {
        // (the runner has called his walk off and told Sam so: orders.ts's stuck rule)
        let stopped = match g.tab.orders_assignment[a].post { None => true, Some(post) => flat(post, j.spot.pos) > 0.05 };
        let there = !stopped && flat(p, j.spot.pos) < 0.4;
        if !stopped && !there && t - j.t0 <= k().go_cap { return; }
        if there && !clear_of(g, d, p) {   // (the last strides to his place, out of the leaf's way)
            let near = match j.near {
                Some(near) => near,
                None => { j.near = Some(t); store(g, gd, j); t }
            };
            if t - near < k().settle { return; }
        }
        if !(there && clear_of(g, d, p)) {
            // at his place and in the leaf's way: it was moved while he walked. The place for the leaf as it stands now, once or twice
            let now = if there && j.moved < 2.0 { shut_spot(g, d, j.spot.side) } else { None };
            if let Some(now) = now {
                if flat(now.pos, j.spot.pos) > 0.3 {
                    j.moved += 1.0;
                    j.spot = now;
                    j.near = None;
                    store(g, gd, j);
                    send(g, gd, d, now);
                    note(g, gd, d, &alloc::format!("the leaf moved: sent on to ({}, {})", to_fixed(now.pos[0], 2), to_fixed(now.pos[2], 2)));
                    return;
                }
            }
            // else from where he stands, if that is a place to shut it from
            let side = j.spot.side;
            let w = g.dr[d].def.width;
            let ok = crate::orders::side_of(g, d, p) == side
                && flat(p, handle_of(g, d)) <= k().far_reach
                && clear_of(g, d, p)
                && { let q = pt_of(g, d, w - k().handle_in, 0.2, side); !walled(g, p, q) };
            if !ok {
                let what = if stopped { "stopped short of his place" } else if there { "no place clear of the leaf as it stands now" } else { "never got to his place" };
                end(g, gd, d, j, if stopped { None } else { Some("can't get to it.") }, what, false);
                return;
            }
        }
        j.phase = JobPhase::Shut;
        j.t = t;
        store(g, gd, j);
    }
    let hd = handle_of(g, d);
    let face = m::atan2(hd[0] - p[0], hd[2] - p[2]);
    g.tab.orders_assignment[a].face = Some(face);
    g.tab.orders_assignment[a].watch = None;   // squared up to the handle, the gun off the leaf that is coming
    if j.phase == JobPhase::Shut {
        let way = in_the_way(g, d, Some(gd), None);
        if way.is_some() || !clear_of(g, d, p) {
            if t - j.t > k().blocked_cap {
                end(g, gd, d, j, Some("can't shut it."), way.as_deref().unwrap_or("in its swing himself"), false);
            }
            return;
        }
        let yaw = g.ch[g.gd[gd].ch].body_yaw;
        if m::abs(v3::wrap_angle(yaw - face)) > 0.6 && t - j.t < 0.5 { return; }
        pull(g, d, gd);
        j.pulls += 1.0;
        j.phase = JobPhase::Wait;
        j.t = t;
        store(g, gd, j);
        let angle = crate::doors::door_angle(g, d);
        let what = alloc::format!(
            "pulls it to from {}{}",
            if j.spot.side == js::or(m::sign(angle), 1.0) { "the leaf's face" } else { "the other face" },
            if j.spot.far { ", at the long reach" } else { "" },
        );
        note(g, gd, d, &what);
        return;
    }
    // wait: the leaf comes to and the latch catches it. One that somebody stopped is pulled again, a few times, while its way is clear
    if t - j.t > k().latch_cap { end(g, gd, d, j, Some("it won't shut."), "the leaf never latched", false); return; }
    if !crate::doors::door_closing(g, d) {
        if t - j.t > k().again && j.pulls < 4.0 && in_the_way(g, d, Some(gd), None).is_none() && clear_of(g, d, p) {
            pull(g, d, gd);
            j.pulls += 1.0;
            j.t = t;
            store(g, gd, j);
        }
    } else if m::abs(crate::doors::door_angle(g, d)) > k().hand_to && in_the_way(g, d, Some(gd), None).is_none() {
        pull(g, d, gd);   // (never against a body it has come to rest on: the leaf settles there, as it does for anybody)
    }
}

/// The def's standsIn.when: on a door that stands open with no charge on it.
pub(crate) fn order_when(g: &mut Game, t: &crate::orders::OrderTarget) -> bool {
    match t {
        crate::orders::OrderTarget::Door(t) => stands_open(g, t.door) && crate::doortools::door_tools_charge_on(g, t.door).is_none(),
        _ => false,
    }
}
/// The def's `only`: the one man of `plan` (where nobody can, the men as addressed go on to `valid`, which says why).
pub(crate) fn order_only(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    let d = match t { crate::orders::OrderTarget::Door(t) => t.door, _ => return men.to_vec() };
    if crate::doors::door_off(g, d) || !stands_open(g, d) { return men.to_vec(); }
    match plan(g, d, men) {
        Plan::Text(_) => men.to_vec(),
        Plan::Arm1(p) => vec![p.gd],
    }
}
/// The def's `valid`: `boolean | string` is Ok(the boolean) or Err(the word).
pub(crate) fn order_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Result<bool, String> {
    let d = match t { crate::orders::OrderTarget::Door(t) => t.door, _ => return Ok(false) };
    match why_not_shut(g, d, men) {
        Some(why) => Err(why),
        None => Ok(true),
    }
}
/// The def's `issue`: the man is sent and his job set; Some: why it cannot be done.
pub(crate) fn order_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    let d = match t { crate::orders::OrderTarget::Door(t) => t.door, _ => return Some(String::from("no door")) };
    let (gd, spot) = match plan(g, d, men) {
        Plan::Text(why) => return Some(why),
        Plan::Arm1(p) => (p.gd, p.spot),
    };
    let back = from_file(g, gd);
    send(g, gd, d, spot);
    let now = g.time;
    g.m.shutdoor.jobs.set(gd, Job { door: d, spot, back, phase: JobPhase::Go, t0: now, t: now, pulls: 0.0, moved: 0.0, near: None });
    g.stats.shutdoor.given += 1;
    let what = alloc::format!(
        "sent to ({}, {}) on its {} face{}",
        to_fixed(spot.pos[0], 2),
        to_fixed(spot.pos[2], 2),
        if spot.side > 0.0 { "+" } else { "−" },
        if back { " · from the file" } else { "" },
    );
    note(g, gd, d, &what);
    None
}

/// The row, on the door tools' page in the charge's place while the door stands open. (shutdoor.ts registerShutDoor)
pub fn register_shut_door(g: &mut crate::game::Game) {
    let def = crate::orders::OrderDef {
        id: String::from(SHUT_ID),
        label: crate::orders::OrderLabel::Text { text: String::from("shut that door") },
        targets: vec![crate::orders::TargetKind::Door],
        ack: Some(vec![String::from("shutting it."), String::from("got the door.")]),
        stands_in: Some(crate::orders::OrderDefStandsIn { for_: String::from(STOOD), when: crate::orders::OrderWhen::ShutDoor }),
        only: Some(crate::orders::OrderOnly::ShutDoor),
        valid: Some(crate::orders::OrderValid::ShutDoor),
        issue: crate::orders::OrderIssue::ShutDoor,
        tick: Some(crate::orders::OrderTick::ShutDoor),
        ..crate::orders::OrderDef::zeroed()
    };
    let id = g.tab.orders_order_def.push(def);
    crate::orders::squad_orders_register(g, id, None);
}
