//! A DEFENDER AND HIS POST: the ways he may leave it and come back to it, each said here once, whoever decides it.
//!   `send`     sent somewhere on his own legs, by a leg of the room fight (`room_fight_tick_move`): somebody close stops him to fight; there, or going nowhere, he is planted
//!   `loose`    off his post on the fight's own hunt, until he is calm again wherever that is (`room_fight_tick_free`)
//!   `back_on`  back on the post he left, as he held it
//!   `room_fight_plant` (defender.rs)  where he has got to is his post now
//! Who may take him from it is the owner's list (owner/ `kept`); these only do it.
use super::*;
use crate::game::Game;
use crate::ids::*;
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
}
impl Leg {
    pub fn to(to: V3, pace: f64) -> Leg { Leg { to, pace, cover: false, by: None, face: None, aim: None } }
}

/// The room fight's row of this man, if he is one of its defenders.
pub fn def_of(g: &Game, gd: GuardId) -> Option<RoomfightDefId> { g.fight.defs.iter().copied().find(|&d| g.tab.roomfight_def[d].gd == gd) }

/// OFF ON A LEG, sent by the tasker `by` (`By::NOBODY`: the room fight itself, or a thing that happened to him).
pub fn send(g: &mut Game, d: RoomfightDefId, leg: Leg, by: By) {
    let gd = g.tab.roomfight_def[d].gd;
    let here = g.ch[g.gd[gd].ch].pos;
    let row = &mut g.tab.roomfight_def[d];
    row.mode = DefMode::Move;
    row.move_kind = if leg.cover { DefMoveKind::Cover } else { DefMoveKind::Fallback };
    row.leg_by = by;
    row.came_by = leg.by;
    row.move_t = 0.0;
    row.stall_at = here;
    row.stall_t = 0.0;
    crate::control::set_pinned(g, gd, false, crate::owner::name_of(by).unwrap_or("roomfight"));
    crate::squad::order(g, gd, crate::squad::OrderPatch {
        goal: Some(Some(leg.to)), speed: Some(Some(leg.pace)), face: Some(leg.face), strafe: Some(Some(leg.face.is_some())), aim_at: Some(leg.aim),
        upper: Some(Some(crate::character::UpperMode::Aim)), stance: Some(Some(crate::character::Stance::None)), crouch: Some(Some(leg.cover)),
        ..Default::default()
    });
}

/// At a run to a place, by that door (or none), `more` over a chase.
pub fn run(g: &mut Game, d: RoomfightDefId, to: V3, door: Option<DoorId>, more: f64, by: By) {
    let pace = g.tune.guard_chase + more;
    send(g, d, Leg { by: door, ..Leg::to(to, pace) }, by);
}

/// He is on a leg, whoever sent him.
pub fn on_leg(g: &Game, d: RoomfightDefId) -> bool { g.tab.roomfight_def[d].mode == DefMode::Move }
/// He is on the leg `by` sent him on. (No: he is planted, loose, or sent elsewhere meanwhile)
pub fn on_leg_of(g: &Game, d: RoomfightDefId, by: By) -> bool { on_leg(g, d) && g.tab.roomfight_def[d].leg_by == by }
/// He is on his way to cover, and fights as he goes.
pub fn fights_as_he_goes(g: &Game, gd: GuardId) -> bool { def_of(g, gd).is_some_and(|d| on_leg(g, d) && g.tab.roomfight_def[d].move_kind == DefMoveKind::Cover) }
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
        row.free_at = now;
    }
    crate::control::set_hold(g, gd, false, by);
    crate::control::set_pinned(g, gd, false, by);
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
    crate::game::guard_set_path(g, gd, Vec::new());
    let man = &mut g.gd[gd];
    man.path_goal = None; man.path_goal_orbit = false;
}

/// a man on a scripted leg files his own boots, so the team's ears get them.
/// a man on a leg files his own boots, so the team's ears get them: `last`, when he last did
pub fn boots(g: &mut Game, gd: GuardId, last: f64) -> f64 {
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

/// a post as a man held it, kept while he is away from it
#[derive(Clone, Debug)]
pub struct Held { pub hold: bool, pub pinned: bool, pub station: V3, pub home: V3, pub room: Option<RoomfightRoomId>, pub ev: Vec<f64>, pub watch: f64, pub want: f64, pub pt: V3 }

/// BACK ON HIS POST, as he held it. What he is aware of and in what state is the caller's, written before this.
pub fn back_on(g: &mut Game, d: RoomfightDefId, h: Held, by: &'static str) {
    let gd = g.tab.roomfight_def[d].gd;
    crate::control::set_hold(g, gd, h.hold, by);
    crate::control::set_pinned(g, gd, h.pinned, by);
    let (state, seen) = (g.gd[gd].state, g.gd[gd].last_seen_t);
    let row = &mut g.tab.roomfight_def[d];
    row.mode = DefMode::Hold;
    row.station = h.station;
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
