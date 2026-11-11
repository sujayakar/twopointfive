//! A DEFENDER AND HIS POST: the ways he may leave it and come back to it, each said here once, whoever decides it.
//!   `send`     sent somewhere on his own legs, by a leg of the room fight (`room_fight_tick_move`): somebody close stops him to fight; there, or going nowhere, he is planted
//!   `loose`    off his post on the fight's own hunt, until he is calm again wherever that is (`room_fight_tick_free`)
//!   `back_on`  back on the post he left, as he held it
//!   `room_fight_plant` (defender.rs)  where he has got to is his post now
//! Who may take him from it is the owner's list (owner/ `kept`); these only do it.
use crate::v3::dist_xz;
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::owner::By;
use crate::V3;
use alloc::vec::Vec;

/// where a man is sent, and how he goes
#[derive(Clone, Copy, Debug)]
pub struct Leg {
    pub to: V3,
    pub pace: f64,
    /// a place to come under cover at, not to fall back to: gone to crouched, and a breath with his head down there
    pub cover: bool,
    /// the door he comes by, which he watches if it takes him into another room
    pub by: Option<DoorId>,
    /// which way he faces as he walks (backwards, if need be); None: the way he goes
    pub face: Option<f64>,
    /// where his gun is as he walks; None: ahead of him
    pub aim: Option<V3>,
    /// an errand, with a thing to do there: at its end he stands there on his sender's word. Else it is his post now.
    pub stays: bool,
    /// how many times he tries again when he gets nowhere
    pub tries: u8,
}
impl Leg {
    pub fn to(to: V3, pace: f64) -> Leg { Leg { to, pace, cover: false, by: None, face: None, aim: None, stays: false, tries: 0 } }
    /// a leg somebody must get to the end of
    pub fn errand(to: V3, pace: f64) -> Leg { Leg { tries: TRIES, ..Leg::to(to, pace) } }
}
/// how many times a man on an errand tries again
pub const TRIES: u8 = 4;
/// seconds of going nowhere that end a try · the longest a leg is walked · … an errand
const STALL: f64 = 1.5;
const MOST: f64 = 10.0;
const MOST_ERRAND: f64 = 60.0;

/// the floor he holds
pub fn station(g: &Game, d: RoomfightDefId) -> V3 { g.tab.task[g.tab.roomfight_def[d].task].post.unwrap_or_default() }
/// what he watches from it
pub fn watched(g: &Game, d: RoomfightDefId) -> V3 { g.tab.task[g.tab.roomfight_def[d].task].watch.unwrap_or_default() }
/// this floor is his to hold from now
pub(super) fn holds_at(g: &mut Game, d: RoomfightDefId, at: V3) { let task = g.tab.roomfight_def[d].task; g.tab.task[task].post = Some(at); }

/// He is no man of the room fight's any more: its row of him goes, and the post with it.
pub fn dropped(g: &mut Game, d: RoomfightDefId) {
    let (gd, task) = (g.tab.roomfight_def[d].gd, g.tab.roomfight_def[d].task);
    g.fight.defs.retain(|&x| x != d);
    crate::duty::done(g, gd, Some(task));
}

/// The room fight's row of this man, if he is one of its defenders.
pub fn def_of(g: &Game, gd: GuardId) -> Option<RoomfightDefId> { g.fight.defs.iter().copied().find(|&d| g.tab.roomfight_def[d].gd == gd) }

/// OFF ON A LEG, sent by the tasker `by` (`By::NOBODY`: the room fight itself, or a thing that happened to him).
pub fn send(g: &mut Game, d: RoomfightDefId, leg: Leg, by: By) { send_by(g, d, leg, Vec::new(), by) }
/// … by the points of `way` first.
pub fn send_by(g: &mut Game, d: RoomfightDefId, leg: Leg, mut way: Vec<V3>, by: By) {
    let gd = g.tab.roomfight_def[d].gd;
    let here = g.pos_of(gd);
    if way.last() != Some(&leg.to) { way.push(leg.to); }
    let first = way[0];
    let left = if at_post(g, d) { Some(post_now(g, d)) } else { g.tab.roomfight_def[d].leg.left.take() };
    let row = &mut g.tab.roomfight_def[d];
    row.mode = DefMode::Move;
    row.leg = OnLeg {
        by, way, kind: if leg.cover { DefMoveKind::Cover } else { DefMoveKind::Fallback }, door: leg.by, stays: leg.stays, tries: leg.tries, end: None,
        t: 0.0, stall_at: here, stall_t: 0.0, step_t: row.leg.step_t, left,
    };
    crate::duty::pin(g, gd, false, crate::owner::name_of(by).unwrap_or("roomfight"));
    crate::squad::order(g, gd, crate::squad::OrderPatch {
        goal: Some(Some(first)), speed: Some(Some(leg.pace)), face: Some(leg.face), strafe: Some(Some(leg.face.is_some())), aim_at: Some(leg.aim),
        upper: Some(Some(crate::character::UpperMode::Aim)), stance: Some(Some(crate::character::Stance::None)), crouch: Some(Some(leg.cover)),
        ..Default::default()
    });
}

/// At a run to a place, by that door (or none), `more` over a chase.
pub fn run(g: &mut Game, d: RoomfightDefId, to: V3, door: Option<DoorId>, more: f64, by: By) {
    let pace = g.tune.guard_chase + more;
    send(g, d, Leg { by: door, ..Leg::to(to, pace) }, by);
}

/// A FRAME OF HIS LEG: his boots in their ears, by the points of his way, and tried again where he gets nowhere. How it ended, if it has.
pub(super) fn walked(g: &mut Game, d: RoomfightDefId, dt: f64) -> Option<LegEnd> {
    let gd = g.tab.roomfight_def[d].gd;
    let at = g.pos_of(gd);
    let last = g.tab.roomfight_def[d].leg.step_t;
    let step_t = boots(g, gd, last);
    let arrived = g.gd[gd].script.as_ref().is_some_and(|s| s.arrived.unwrap_or(false));
    let leg = &mut g.tab.roomfight_def[d].leg;
    leg.step_t = step_t;
    if leg.end.is_some() { return leg.end; }
    if leg.way.len() > 1 && (dist_xz(at, leg.way[0]) < 1.4 || dist_xz(at, leg.way[1]) < dist_xz(leg.way[0], leg.way[1])) {
        leg.way.remove(0);
        return walk_on(g, d);
    }
    if arrived && leg.way.len() <= 1 { return Some(LegEnd::Arrived); }
    leg.t += dt;
    if dist_xz(at, leg.stall_at) > 0.4 { leg.stall_at = at; leg.stall_t = 0.0; } else { leg.stall_t += dt; }
    let errand = leg.stays || leg.tries > 0;
    if leg.t > if errand { MOST_ERRAND } else { MOST } { return Some(LegEnd::Nowhere); }
    if leg.stall_t <= STALL { return None; }
    if leg.tries == 0 { return Some(LegEnd::Nowhere); }
    leg.tries -= 1;
    leg.stall_t = 0.0;
    walk_on(g, d)
}
/// on to the next point of his way, by a way laid again from where he is
fn walk_on(g: &mut Game, d: RoomfightDefId) -> Option<LegEnd> {
    let (gd, to) = (g.tab.roomfight_def[d].gd, g.tab.roomfight_def[d].leg.way[0]);
    crate::squad::order(g, gd, crate::squad::OrderPatch { goal: Some(Some(to)), ..Default::default() });
    None
}

/// He is on a leg, whoever sent him.
pub fn on_leg(g: &Game, d: RoomfightDefId) -> bool { g.tab.roomfight_def[d].mode == DefMode::Move }
/// He is on the leg `by` sent him on, or stands at its end. (No: he is planted, loose, or sent elsewhere meanwhile)
pub fn on_leg_of(g: &Game, d: RoomfightDefId, by: By) -> bool { on_leg(g, d) && g.tab.roomfight_def[d].leg.by == by }
/// HOW THE LEG `by` SENT HIM ON ENDED; None: he is on his way.
pub fn leg_ended(g: &Game, d: RoomfightDefId, by: By) -> Option<LegEnd> {
    let (leg, on) = (&g.tab.roomfight_def[d].leg, on_leg(g, d));
    if leg.by != by { return Some(LegEnd::Taken); }
    match leg.end {
        None if on => None,
        Some(LegEnd::Arrived | LegEnd::Nowhere) if leg.stays && !on => Some(LegEnd::Taken),
        e => Some(e.unwrap_or(LegEnd::Taken)),
    }
}
/// where he is sent
pub fn sent_to(g: &Game, d: RoomfightDefId) -> Option<V3> { g.tab.roomfight_def[d].leg.way.last().copied() }
/// He is on his way to cover, and fights as he goes.
pub fn fights_as_he_goes(g: &Game, gd: GuardId) -> bool { def_of(g, gd).is_some_and(|d| on_leg(g, d) && g.tab.roomfight_def[d].leg.kind == DefMoveKind::Cover) }
/// He is at his post.
pub fn at_post(g: &Game, d: RoomfightDefId) -> bool { g.tab.roomfight_def[d].mode == DefMode::Hold }
/// He is off it on the fight's own hunt.
pub fn is_loose(g: &Game, d: RoomfightDefId) -> bool { g.tab.roomfight_def[d].mode == DefMode::Free }

/// OFF HIS POST, on the fight's own hunt: nothing holds him, and whatever script he was on is over. (A man the room fight has no row of is let go the same.)
pub fn loose(g: &mut Game, gd: GuardId, by: &'static str) {
    if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::game::GuardState::Alert)); }
    if let Some(d) = def_of(g, gd) {
        let now = g.time;
        let row = &mut g.tab.roomfight_def[d];
        row.mode = DefMode::Free;
        row.leg.left = None;
        row.free_at = now;
    }
    crate::duty::hold(g, gd, false, by);
    crate::duty::pin(g, gd, false, by);
}

/// handed to the fight AI as the stock rally hands a man over: off his post, roused, the fight's place his fix.
/// LET LOOSE AT A PLACE: roused, the fight there his fix.
pub fn loose_at(g: &mut Game, d: RoomfightDefId, at: V3) {
    let gd = g.tab.roomfight_def[d].gd;
    let now = g.time;
    loose(g, gd, "roomfight");
    crate::mind::set_state(g, gd, crate::game::GuardState::Alert, "roomfight");
    crate::mind::raise(g, gd, 0.92, "roomfight");
    if now - g.gd[gd].last_seen_t > 1.0 { crate::belief::word(g, gd, at); }
    let man = &mut g.gd[gd];
    man.drop_path();
}

/// a man on a leg files his own boots, so the team's ears get them: `last`, when he last did
fn boots(g: &mut Game, gd: GuardId, last: f64) -> f64 {
    let speed = g.gd[gd].speed;
    if speed < 0.4 || g.time - last < (if speed > 2.0 { 0.33 } else { 0.5 }) { return last; }
    let c = g.gd[gd].ch;
    let serial = g.serial();
    let ev = crate::game::GameEvent {
        kind: crate::director::EventKind::Step, pos: g.ch[c].pos, time: g.time, loud: false,
        level: Some(if speed > 2.0 { 0.8 } else { 0.4 }), who: Some(g.ch[c].id), serial,
        ..crate::game::GameEvent::default()
    };
    g.events.push(ev);
    g.time
}

/// the post he holds now, kept on his leg while he is away from it
fn post_now(g: &Game, d: RoomfightDefId) -> PostLeft {
    let row = &g.tab.roomfight_def[d];
    let man = &g.gd[row.gd];
    PostLeft {
        t: g.time, state: man.state, awareness: man.awareness, hold: man.duty.holds, pinned: man.duty.pinned, station: station(g, d), home: row.home, room: row.room, ev: row.ev.clone(),
        watch: row.watch, want: row.want, pt: watched(g, d),
    }
}

/// BACK ON THE POST HE LEFT, as he held it. `wary`: something is wrong, and he is wary and no more; a man who was roused when he left, or has heard a shot since, is not calmed down.
pub fn back_on(g: &mut Game, d: RoomfightDefId, wary: bool, by: &'static str) {
    use crate::game::GuardState as S;
    let gd = g.tab.roomfight_def[d].gd;
    let Some(h) = g.tab.roomfight_def[d].leg.left.take() else { return super::stand_here(g, d) };
    let roused = matches!(h.state, S::Alert | S::Search);
    let aw = g.gd[gd].awareness;
    let heard = wary && (g.gd[gd].witness.heard.last_shot_t > h.t || aw >= 0.85);
    if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(if roused { h.state } else if wary { S::Suspicious } else { S::Patrol })); }
    let now = if roused { m::max(aw, h.awareness) } else if !wary { h.awareness } else {
        let a = m::max_n(&[aw, h.awareness, 0.35]);
        if heard { a } else { m::min(0.45, a) }
    };
    crate::mind::set(g, gd, now, by);
    crate::duty::hold(g, gd, h.hold, by);
    crate::duty::pin(g, gd, h.pinned, by);
    let (state, seen) = (g.gd[gd].state, g.gd[gd].last_seen_t);
    holds_at(g, d, h.station);
    let row = &mut g.tab.roomfight_def[d];
    row.mode = DefMode::Hold;
    row.home = h.home;
    row.room = h.room;
    row.ev = h.ev;
    row.watch = h.watch;
    row.want = h.want;
    row.glance_until = -1.0;
    row.fix_keep = -1.0;
    row.last_state = state;
    row.last_seen = seen;
    // (his watched point moves in place: a fix that is it moves with it)
    def_move_pt(g, d, h.pt);
    crate::belief::cover(g, gd, h.pt);
}
