//! src/game/hands.ts lines 512 to 539
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Another man's hand on a door he works: the door tools' lead at his handle, a leaf he just shoved, a leaf he pulls to. Moves nothing. (hands.ts teamDoorHand)
pub fn team_door_hand(g: &mut crate::game::Game, gd: GuardId) -> Option<DoorId> {
    if g.touring { return None; }
    let c = g.gd[gd].ch;
    if crate::character::anim_kicking(g, c) { return None; }
    let (pos, vel, id) = (g.ch[c].pos, g.ch[c].vel, g.ch[c].id);
    let seat = crate::seats::view_of(g, gd);
    let at_handle = crate::seats::with_seat(g, seat, &mut |g: &mut Game| lead_at_handle(g, &crate::seats::here(g), gd, pos));
    if at_handle.is_some() { return at_handle; }
    let slow = m::hyp2(vel[0], vel[2]) < g.k.hands.team_pace;
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        if crate::doors::door_off(g, d) || crate::doors::door_last_who(g, d) != id { continue; }
        let dv = crate::doors::door_vel(g, d);
        if m::abs(dv) < g.k.hands.team_vel { continue; }
        let angle = crate::doors::door_angle(g, d);
        let opening = dv * angle > 0.0 || m::abs(angle) < 0.05;
        if !(crate::doors::door_closing(g, d) || (opening && slow)) { continue; }
        if leaf_dist(g, d, angle, pos) < g.k.hands.team_reach_m { return Some(d); }
    }
    None
}

/// The door tools' lead a step from the handle he was sent to, among the door work on Game: the seat this pass is about. (hands.ts:523)
fn lead_at_handle(g: &Game, seat: &crate::seats::SeatRef, gd: GuardId, pos: V3) -> Option<DoorId> {
    let mut i = 0;
    while i < seat.door_tools.ops.len() {
        let row = &g.tab.doortools_door_op[seat.door_tools.ops[i]];
        i += 1;
        if row.lead != gd || row.phase != crate::doortools::Phase::Kick { continue; }
        let Some(k) = row.kick.as_ref() else { continue };
        if k.hand != Some(true) { continue; }
        let (door, kp) = (row.door, k.pos);
        if !crate::doors::door_off(g, door) && m::hyp2(pos[0] - kp[0], pos[2] - kp[2]) < g.k.hands.team_pre_m { return Some(door); }
    }
    None
}

/// A cancel or a restart: hands off everything. (hands.ts handsCancel)
pub fn hands_cancel(g: &mut crate::game::Game) {
    let p = g.player;
    g.pl[p].door_move = None;
}

