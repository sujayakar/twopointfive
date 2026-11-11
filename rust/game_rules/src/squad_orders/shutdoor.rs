pub use ::game_types::shutdoor::types;
pub use types::*;
/// this module's own order
pub const SHUT_ID: &str = crate::orders::rows::shutdoor::ORDER.id;
use crate::v3::dist_xz;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// Where the door's handle comes to.
fn handle_of(g: &crate::game::Game, d: DoorId) -> V3 {
    let w = g.dr[d].def.width;
    crate::handleside::pt_of(g, d, w - k().handle_in, 0.0, 1.0)
}

/// Does d stand open (off its hinges it reads wide open: its row is there, greyed).
pub(super) fn stands_open(g: &crate::game::Game, d: DoorId) -> bool {
    !g.dr[d].leaf.latched && m::abs(g.dr[d].leaf.angle) >= k().open
}

/// The floor the leaf crosses on its way shut from where it stands now, with `pad` m of slack.
fn in_shut_swing(g: &crate::game::Game, d: DoorId, x: f64, z: f64, pad: Option<f64>) -> bool {
    let pad = pad.unwrap_or(0.0);
    let f = crate::guards::frame_of(g, d);
    let (px, pz) = (x - f.hx, z - f.hz);
    let r = m::hyp2(px, pz);
    if r > f.w + pad { return false; }
    let along = px * f.ux + pz * f.uz;
    let t = m::max(0.0, m::min(f.w, along));
    if m::hyp2(px - f.ux * t, pz - f.uz * t) <= pad { return true; }   // on the frame's line: where the leaf comes to
    let angle = g.dr[d].leaf.angle;
    let th = v3::wrap_angle(m::atan2(pz, px) - g.dr[d].def.closed_dir) * js::or(m::sign(angle), 1.0);
    th >= 0.0 && th <= m::abs(angle)
}

/// Can a man stand at p: a cell the walkers' grid calls free, nothing of the level inside his body.
fn stands(g: &crate::game::Game, p: V3) -> bool {
    if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return false; }
    let mut q: V3 = [p[0], 0.0, p[2]];
    crate::scene_collision::collide_circle(g, &mut q, 0.3, 0.2, 1.5, None);
    dist_xz(p, q) < 0.02
}

/// shutSpot's `ok`: clear of the leaf's way, a place to stand, nothing of the level between it and the door.
fn spot_ok(g: &Game, d: DoorId, ref_: V3, p: V3, pad: f64) -> bool {
    !in_shut_swing(g, d, p[0], p[2], Some(pad)) && stands(g, p) && !crate::scene_collision::low_walled(g, ref_, p)
}

/// The place d is shut from on face `side`, as it stands now; None: none.
fn shut_spot(g: &crate::game::Game, d: DoorId, side: f64) -> Option<crate::shutdoor::Spot> {
    let w = g.dr[d].def.width;
    let hd = handle_of(g, d);
    let ref_ = crate::handleside::pt_of(g, d, w - k().handle_in, 0.2, side);
    let pad = k().body + k().slack;
    let face = |p: V3| v3::yaw_to(p, hd);
    let mut best: Option<V3> = None;
    let mut bc = f64::INFINITY;
    let mut a = w - 0.5;
    while a <= w + 0.8 + 1e-6 {
        let mut off = 0.4;
        while off <= 1.0 + 1e-6 {
            let p = crate::handleside::pt_of(g, d, a, off, side);
            let r = dist_xz(p, hd);
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
    if let Some(p) = best { return Some(Spot { pos: p, yaw: face(p), side }); }
    let mut a = w / 2.0;
    while a <= w + 0.4 + 1e-6 {
        let mut off = 0.5;
        while off <= 1.8 + 1e-6 {
            let p = crate::handleside::pt_of(g, d, a, off, side);
            let r = dist_xz(p, hd);
            if !(r > k().far_reach || r >= bc || !spot_ok(g, d, ref_, p, pad)) {
                best = Some(p);
                bc = r;
            }
            off += 0.05;
        }
        a += 0.1;
    }
    best.map(|p| Spot { pos: p, yaw: face(p), side })
}

/// Both faces' places for the leaf as it stands, kept for the frame.
fn spots_of(g: &mut crate::game::Game, d: DoorId) -> Vec<Option<crate::shutdoor::Spot>> {
    let angle = g.dr[d].leaf.angle;
    let key = alloc::format!("{}|{}|{}", js::num_str(g.time), g.dr[d].def.name, js::to_fixed(angle, 3));
    if let Some(hit) = &g.m.shutdoor.spotted {
        if hit.key == key { return hit.spots.clone(); }
    }
    let first = shut_spot(g, d, 1.0);
    let second = shut_spot(g, d, -1.0);
    let spots = vec![first, second];
    g.m.shutdoor.spotted = Some(ShutdoorModSpottedValue { key, spots: spots.clone() });
    spots
}

/// The length of his walk to p by the walkers' grid (infinity: no way).
fn walk(g: &crate::game::Game, gd: GuardId, p: V3) -> f64 {
    let at = g.pos_of(gd);
    if dist_xz(at, p) < 0.5 { return dist_xz(at, p); }
    let path = match crate::scene_collision::nav_grid_find_path(g, at, p, None, None) { Some(path) => path, None => return f64::INFINITY };
    crate::places::walk_len(at, &path) + dist_xz(path.last().copied().unwrap_or(at), p)
}

/// Was he taken from Sam's file (or from another door of this order's, taken from the file for that one).
fn from_file(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId) -> bool {
    let a = match crate::orders::squad_orders_of(seat, gd) { Some(a) => a, None => return false };
    let row = &g.tab.task[a];
    row.post.is_none() || (row.order == SHUT_ID && g.m.shutdoor.jobs.get(gd).map_or(false, |j| j.back))
}

/// Who shuts it, from where: the shortest walk to a place it can be shut from, or why none of them can.
pub(super) fn plan(g: &mut crate::game::Game, d: DoorId, men: &[GuardId]) -> crate::shutdoor::Plan {
    let spots = spots_of(g, d);
    if spots[0].is_none() && spots[1].is_none() { return Plan::Text(String::from("no place to shut it from.")); }
    let mut out: Option<PlanArm1> = None;
    let mut bc = f64::INFINITY;
    let mut why: Option<String> = None;
    let sam_at = g.player_pos(g.player);
    let sam = crate::orders::side_of(g, d, sam_at);
    for &gd in men {
        let want = if from_file(g, &crate::seats::here(g), gd) {
            sam
        } else {
            let at = g.pos_of(gd);
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
fn body_in_swing(g: &Game, d: DoorId, c: CharId) -> bool {
    let rag = match g.ch[c].ragdoll { Some(rag) => rag, None => return false };
    if dist_xz(g.ch[c].pos, g.dr[d].frame_centre) > 4.0 { return false; }
    let r = crate::ragdoll::rag();
    for i in [r.pelvis, r.spine, r.head, r.knee_l, r.knee_r, r.ank_l, r.ank_r] {
        let p = crate::ragdoll::ragdoll_point(g, rag, i, None);
        let radius = crate::ragdoll::ragdoll_radius(i);
        if in_shut_swing(g, d, p[0], p[2], Some(radius)) { return true; }
    }
    false
}

/// Who or what is in the leaf's way shut (None: nothing); `seen`: the menu's question, an enemy the picture does not show is not given away.
fn in_the_way(g: &crate::game::Game, d: DoorId, but: Option<GuardId>, seen: Option<bool>) -> Option<String> {
    let seen = seen.unwrap_or(false);
    let player = g.player;
    let sam_ch = g.pl[player].ch;
    let sam_at = g.ch[sam_ch].pos;
    if !crate::condition::down(g, g.pl[player].ch) && in_shut_swing(g, d, sam_at[0], sam_at[2], Some(k().body)) { return Some(String::from("you're in its way.")); }
    let mut all = g.guards.clone();
    all.extend(crate::civilians::civilians_shown(g));
    for m in all {
        if Some(m) == but { continue; }
        let c = g.gd[m].ch;
        if crate::condition::down(g, g.gd[m].ch) {
            if body_in_swing(g, d, c) { return Some(String::from("a body's in its way.")); }
            continue;
        }
        let at = g.ch[c].pos;
        if !in_shut_swing(g, d, at[0], at[2], Some(k().body))
            || (seen && !g.team(m).ours() && !crate::fogofwar::fog_of_war_drawn(g, m)) {
            continue;
        }
        return Some(if g.team(m).ours() {
            alloc::format!("{} is in its way.", g.gd[m].callsign)
        } else {
            String::from("somebody's in its way.")
        });
    }
    if crate::condition::down(g, g.pl[player].ch) && body_in_swing(g, d, sam_ch) { Some(String::from("a body's in its way.")) } else { None }
}

/// An enemy somebody of the team has in sight right now through that doorway.
fn contact_through(g: &crate::game::Game, d: DoorId) -> bool {
    let player = g.player;
    let sam_ch = g.pl[player].ch;
    for h in g.guards.clone() {
        if crate::condition::down(g, g.gd[h].ch) || !g.team(h).theirs() { continue; }
        for m in crate::roomfight::room_fight_seen_by(g, h) {
            let (m_at, h_at) = (g.pos_of(m), g.pos_of(h));
            if crate::guards::crosses_door(g, m_at, h_at, d).is_some() { return true; }
        }
        let (sam_at, h_at) = (g.ch[sam_ch].pos, g.pos_of(h));
        if !crate::condition::down(g, g.pl[player].ch)
            && crate::guards::crosses_door(g, sam_at, h_at, d).is_some()
            && dist_xz(sam_at, h_at) < 30.0
            && crate::fogofwar::fog_of_war_drawn(g, h)
            && !crate::scene_collision::segment_blocked(g, [sam_at[0], 1.5, sam_at[2]], [h_at[0], 1.2, h_at[2]], None, None, None) {
            return true;
        }
    }
    false
}

/// Why these men cannot shut that door (None: one of them can): the row's `valid`.
pub(super) fn why_not_shut(g: &mut crate::game::Game, d: DoorId, men: &[GuardId]) -> Option<String> {
    if g.dr[d].leaf.off { return Some(String::from("it's off its hinges.")); }
    if g.dr[d].leaf.latched { return Some(String::from("it's shut.")); }
    if !stands_open(g, d) { return Some(String::from("it's all but shut.")); }
    if contact_through(g, d) { return Some(String::from("contact through that door.")); }
    match plan(g, d, men) {
        Plan::Text(why) => Some(why),
        Plan::Arm1(p) => in_the_way(g, d, Some(p.gd), Some(true)),
    }
}

/// He is sent to a place of the door's.
fn send(g: &mut crate::game::Game, gd: GuardId, d: DoorId, spot: crate::shutdoor::Spot) {
    let name = crate::orders::door_words(g, d);
    let fc = g.dr[d].frame_centre;
    let a = crate::duty::TaskGiven {
        order: String::from(SHUT_ID),
        post: Some(spot.pos),
        face: Some(spot.yaw),
        watch: Some([fc[0], 1.05, fc[2]]),
        door: Some(d),
        stance: Some(crate::character::Stance::None),
        upper: Some(crate::character::UpperMode::Aim),
        going: alloc::format!("shutting · {}", name).into(),
        there: alloc::format!("at · {}", name).into(),
        ..Default::default()
    };
    crate::orders::squad_orders_assign(g, gd, &a);
}

/// His hand on it: the leaf is pulled to; a leaf somebody propped is let go of.
fn pull(g: &mut crate::game::Game, d: DoorId, gd: GuardId) {
    if g.dr[d].leaf.quiet_t < 0.0 { g.dr[d].leaf.quiet_t = 0.0; }
    let who = g.char_id(gd);
    crate::doors::doors_pull_to(g, d, who);
}

/// His order is over: back into the file if that is where he was taken from, else he holds where he stands.
fn finish(g: &mut crate::game::Game, gd: GuardId, j: Option<crate::shutdoor::Job>) {
    g.m.shutdoor.jobs.delete(gd);
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    if j.map_or(false, |j| j.back) {
        let a = crate::duty::TaskGiven {
            order: String::from(crate::orders::rows::fallin::ORDER.id),
            post: None,
            follow: Some(99.0),
            face: None,
            going: "falling in".into(),
            there: "with you".into(),
            ..Default::default()
        };
        crate::orders::squad_orders_assign(g, gd, &a);
        return;
    }
    let there = match j {
        Some(j) => alloc::format!("holding · by {}", crate::orders::door_words(g, j.door)),
        None => String::from("holding"),
    };
    let a = crate::duty::TaskGiven {
        order: String::from(crate::orders::rows::hold::ORDER.id),
        post: Some([p[0], 0.0, p[2]]),
        face: Some(g.ch[c].body_yaw),
        going: "holding".into(),
        there: there.into(),
        ..Default::default()
    };
    crate::orders::squad_orders_assign(g, gd, &a);
}

/// tick's `end`: the word, the log, the count, and his order is over.
fn end(g: &mut Game, gd: GuardId, j: Job, line: Option<&str>) {
    if let Some(line) = line { crate::game::game_say(g, gd, line, None, None); }
    finish(g, gd, Some(j));
}
/// tick's `clearOf`: is p out of the leaf's way
fn clear_of(g: &Game, d: DoorId, p: V3) -> bool {
    !in_shut_swing(g, d, p[0], p[2], Some(k().body))
}
/// The job is one row of the table: every change of the local copy goes back to it.
fn store(g: &mut Game, gd: GuardId, j: Job) {
    if let Some(row) = g.m.shutdoor.jobs.get_mut(gd) { *row = j; }
}

/// The order's stages, once a frame for its man before he is posed.
pub fn tick(g: &mut crate::game::Game, gd: GuardId, a: TaskId) {
    let mut j = match g.m.shutdoor.jobs.get(gd).copied() {
        Some(j) if g.tab.task[a].door == Some(j.door) => j,
        _ => { finish(g, gd, None); return; }   // (a guard: the order's issue sets both)
    };
    let d = j.door;
    let p = g.pos_of(gd);
    let t = g.time;
    if g.dr[d].leaf.off { end(g, gd, j, Some("door's gone.")); return; }
    if g.dr[d].leaf.latched {
        if j.phase == JobPhase::Go {
            end(g, gd, j, Some("it's shut."));
        } else {
            end(g, gd, j, Some("door's shut."));
        }
        return;
    }
    if j.phase == JobPhase::Go {
        // (the runner has called his walk off and told Sam so)
        let stopped = match g.tab.task[a].post { None => true, Some(post) => dist_xz(post, j.spot.pos) > 0.05 };
        let there = !stopped && dist_xz(p, j.spot.pos) < 0.4;
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
                if dist_xz(now.pos, j.spot.pos) > 0.3 {
                    j.moved += 1.0;
                    j.spot = now;
                    j.near = None;
                    store(g, gd, j);
                    send(g, gd, d, now);
                    return;
                }
            }
            // else from where he stands, if that is a place to shut it from
            let side = j.spot.side;
            let w = g.dr[d].def.width;
            let ok = crate::orders::side_of(g, d, p) == side
                && dist_xz(p, handle_of(g, d)) <= k().far_reach
                && clear_of(g, d, p)
                && { let q = crate::handleside::pt_of(g, d, w - k().handle_in, 0.2, side); !crate::scene_collision::low_walled(g, p, q) };
            if !ok {
                end(g, gd, j, if stopped { None } else { Some("can't get to it.") });
                return;
            }
        }
        j.phase = JobPhase::Shut;
        j.t = t;
        store(g, gd, j);
    }
    let hd = handle_of(g, d);
    let face = v3::yaw_to(p, hd);
    g.tab.task[a].face = Some(face);
    g.tab.task[a].watch = None;   // squared up to the handle, the gun off the leaf that is coming
    if j.phase == JobPhase::Shut {
        let way = in_the_way(g, d, Some(gd), None);
        if way.is_some() || !clear_of(g, d, p) {
            if t - j.t > k().blocked_cap {
                end(g, gd, j, Some("can't shut it."));
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
        return;
    }
    // wait: the leaf comes to and the latch catches it. One that somebody stopped is pulled again, a few times, while its way is clear
    if t - j.t > k().latch_cap { end(g, gd, j, Some("it won't shut.")); return; }
    if !g.dr[d].leaf.closing {
        if t - j.t > k().again && j.pulls < 4.0 && in_the_way(g, d, Some(gd), None).is_none() && clear_of(g, d, p) {
            pull(g, d, gd);
            j.pulls += 1.0;
            j.t = t;
            store(g, gd, j);
        }
    } else if m::abs(g.dr[d].leaf.angle) > k().hand_to && in_the_way(g, d, Some(gd), None).is_none() {
        pull(g, d, gd);   // (never against a body it has come to rest on: the leaf settles there, as it does for anybody)
    }
}

/// The def's `issue`: the man is sent and his job set; Some: why it cannot be done.
pub(super) fn order_issue(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    let d = match t { crate::orders::OrderTarget::Door(t) => t.door, _ => return Some(String::from("no door")) };
    let (gd, spot) = match plan(g, d, men) {
        Plan::Text(why) => return Some(why),
        Plan::Arm1(p) => (p.gd, p.spot),
    };
    let back = from_file(g, &crate::seats::here(g), gd);
    send(g, gd, d, spot);
    let now = g.time;
    g.m.shutdoor.jobs.set(gd, Job { door: d, spot, back, phase: JobPhase::Go, t0: now, t: now, pulls: 0.0, moved: 0.0, near: None });
    None
}

/// The row, on the door tools' page in the charge's place while the door stands open.
pub fn register_shut_door(g: &mut crate::game::Game) {
    let def = crate::orders::rows::def(&crate::orders::rows::shutdoor::ORDER, crate::orders::OrderDef {
        targets: vec![crate::orders::TargetKind::Door],
        ack: Some(vec![String::from("shutting it."), String::from("got the door.")]),
        stands_in: Some(crate::orders::OrderDefStandsIn { for_: String::from(crate::orders::rows::charge::ORDER.id) }),
        ..crate::orders::OrderDef::default()
    });
    let id = crate::orders::squad_orders_row(g, def);
    crate::orders::squad_orders_register(g, id, None);
}
