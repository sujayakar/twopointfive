//! WHO IS IN THE LINE of a round: a mate, Sam, anybody it could go to.
#![allow(unused_imports)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `c.bones.upperArmR ?? c.pos + [0, 1.4, 0]`
pub(super) fn shoulder_of(g: &Game, c: CharId) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::UpperArmR) { Some(b) => *b, None => v3::add(g.ch[c].pos, [0.0, 1.4, 0.0]) }
}

/// `c.bones.head ?? c.pos + [0, 1.6, 0]`
pub(super) fn head_of(g: &Game, c: CharId) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::Head) { Some(b) => *b, None => v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]) }
}

pub(super) fn cone_friend(f: crate::friendlyfire::FriendInCone) -> RoomFightFriendInLine {
    match f { crate::friendlyfire::FriendInCone::Guard(x) => RoomFightFriendInLine::Guard(x), crate::friendlyfire::FriendInCone::Player => RoomFightFriendInLine::Player }
}

pub(super) fn line_by(f: RoomFightFriendInLine) -> OpLineBy {
    match f { RoomFightFriendInLine::Guard(x) => OpLineBy::Guard(x), RoomFightFriendInLine::Player => OpLineBy::Player }
}

/// Is this body within the clearance it is owed of the line from → to (the `hit` of roomfight.ts friendInLine)
pub(super) fn line_hit(g: &mut Game, ch: CharId, mate: bool, from: V3, dx: f64, dz: f64, l2: f64, l: f64, t_max: f64) -> bool {
    let pos = g.ch[ch].pos;
    let vel = g.ch[ch].vel;
    let h = match g.ch[ch].bones.get(crate::character::Bone::Hips) { Some(b) => *b, None => pos };
    let tr = ((h[0] - from[0]) * dx + (h[2] - from[2]) * dz) / l2;
    let t = v3::clamp(tr, 0.0, t_max);
    let sam = !mate && crate::seats::man_of(g, Some(ch)).is_some();
    let pace = m::hyp2(vel[0], vel[2]);
    let moving = if sam { m::min(pace, room_fight_sam_speed_of(g, ch)) } else { pace };
    let wide = 0.45 + m::min(0.5, 0.3 * moving);
    let clear = if mate {
        close_lane(g, wide, tr, l)
    } else if tr > 1.0 {
        m::min(wide, 0.3 + 0.2 * (tr - 1.0) * l)
    } else if tr <= 0.0 {
        0.0
    } else {
        m::min(wide, 0.3 + tr * l)
    };
    m::hyp2(from[0] + dx * t - h[0], from[2] + dz * t - h[2]) < clear
}

/// A friend, Sam or another living operative, within a body's breadth of the line from → to and a metre past it, wider for one on the move. (roomfight.ts friendInLine)
pub fn room_fight_friend_in_line(g: &mut crate::game::Game, from: V3, to: V3, self_: GuardId) -> Option<crate::roomfight::RoomFightFriendInLine> {
    let dx = to[0] - from[0];
    let dz = to[2] - from[2];
    let l2 = dx * dx + dz * dz;
    if l2 < 0.0025 { return None; }
    let l = m::sqrt(l2);
    let t_max = 1.0 + 1.0 / l;
    for pl in crate::seats::men(g) {
        if g.pl[pl].down { continue; }
        let ch = g.pl[pl].ch;
        if line_hit(g, ch, false, from, dx, dz, l2, l, t_max) { return Some(RoomFightFriendInLine::Player); }
    }
    let mine = g.gd[self_].team;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let ch = g.gd[o].ch;
        if o == self_ || g.gd[o].state == crate::game::GuardState::Dead || !g.ch[ch].alive { continue; }
        let theirs = g.gd[o].team;
        if crate::teams::is_hostile(g, mine, theirs) { continue; }
        let mate = theirs == crate::teams::Team::Operative && g.gd[o].unit.is_some();
        if line_hit(g, ch, mate, from, dx, dz, l2, l, t_max) { return Some(RoomFightFriendInLine::Guard(o)); }
    }
    None
}

/// Sam between a teammate's gun and what it is on, or a metre past it: the gun goes to the low ready while he is there. (roomfight.ts samInLine)
pub(super) fn room_fight_sam_in_line(g: &mut crate::game::Game, o: RoomfightOpId, to: V3) -> bool {
    let gd = g.tab.roomfight_op[o].gd;
    let c = g.gd[gd].ch;
    let from = shoulder_of(g, c);
    if !matches!(room_fight_friend_in_line(g, from, to, gd), Some(RoomFightFriendInLine::Player)) { return false; }
    g.gd[gd].muzzle_down = true;
    g.gd[gd].muzzle_clear_t = 0.0;
    true
}

/// True of an enemy who is NOT this covering man's: off his ordered way, and a mate has him. (roomfight.ts coverPasses)
pub(super) fn room_fight_cover_passes(g: &crate::game::Game, seat: &crate::seats::SeatRef, o: RoomfightOpId, h: GuardId) -> bool {
    let r = g.k.roomfight;
    let a = match crate::orders::squad_orders_of(g, seat, g.tab.roomfight_op[o].gd) { Some(a) => a, None => return false };
    let (watch, post) = {
        let row = &g.tab.orders_assignment[a];
        if !crate::orders::rows::id_is(&row.order, crate::orders::Is::Covers) { return false; }
        match (row.watch, row.post) { (Some(w), Some(p)) => (w, p), _ => return false }
    };
    let cp = g.ch[g.gd[g.tab.roomfight_op[o].gd].ch].pos;
    let hc = g.gd[h].ch;
    let hp = g.ch[hc].pos;
    if flat(cp, post) > 0.8 || flat(cp, hp) < r.cover_near_m { return false; } // not on his post yet · a man this close is anybody's
    if m::abs(v3::wrap_angle(v3::yaw_to(cp, hp) - v3::yaw_to(cp, watch))) <= r.cover_half_deg * crate::v3::DEG { return false; } // in the angle he was given
    if m::abs(v3::wrap_angle(v3::yaw_to(hp, cp) - g.ch[hc].aim_yaw)) < 0.35 { return false; } // a muzzle on him is his own business
    // a mate HAS the man: his gun on him and a round to send; a man this one is already ON goes only to a mate past his own beat to fire
    let g0 = g.time;
    let mine = g.tab.roomfight_op[o].tgt == Some(h);
    for &xo in g.fight.ops.iter() {
        let x = &g.tab.roomfight_op[xo];
        if xo != o && x.tgt == Some(h) && g.gd[x.gd].state != crate::game::GuardState::Dead && x.block_t < r.no_shot && x.line_t == 0.0 && !crate::control::man::changing_magazine(g, x.gd) && !g.gd[x.gd].muzzle_down && (!mine || g0 >= x.ready_at) { return true; }
    }
    false
}
