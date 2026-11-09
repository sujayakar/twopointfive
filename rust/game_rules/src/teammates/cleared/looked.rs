//! THE FLOOR THE TEAM HAS LOOKED OVER: a mark a cell of the level's rooms, made by whoever of the team has the cell in his eyes with a line to it. A room somebody of theirs has been in
//! may still have a corner nobody has looked into; cover/ has a man with nothing better to watch look there.
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};
use alloc::vec::Vec;

/// how far floor is looked over, and how wide
const LOOK_M: f64 = 11.0;
const HALF: f64 = 45.0 * v3::DEG;
/// floor this near him needs no look
const NEAR_M: f64 = 1.2;
const EYE: f64 = 1.5;
/// what could be there: a man's middle
const THERE: f64 = 1.0;
/// two men do not look at the same corner: this far from another's
const APART_M: f64 = 2.0;

/// the cells of rooms within LOOK_M of a place that nobody has looked at, `room` of them only if it is said: where each is, and its number
fn unseen_by(g: &Game, p: V3, room: Option<u16>) -> Vec<(usize, V3)> {
    let nav = &g.col.nav;
    let (w, h, cs) = (nav.w as usize, nav.h as usize, nav.cell);
    let Some(rooms) = g.cleared.room_of_cell.as_deref() else { return Vec::new() };
    let lo = |v: f64| m::max(0.0, m::floor((v - LOOK_M) / cs)) as usize;
    let hi = |v: f64, n: usize| (m::floor((v + LOOK_M) / cs) as usize + 1).min(n);
    let mut out = Vec::new();
    for iz in lo(p[2])..hi(p[2], h) {
        for ix in lo(p[0])..hi(p[0], w) {
            let i = iz * w + ix;
            if rooms[i] == 0 || room.map_or(false, |r| r != rooms[i]) || nav.blocked[i] == 1 || g.cleared.looked[i] != 0 { continue; }
            let q = [(ix as f64 + 0.5) * cs, THERE, (iz as f64 + 0.5) * cs];
            if v3::dist_xz(p, q) <= LOOK_M { out.push((i, q)); }
        }
    }
    out
}

/// What the team has in its eyes now is looked over. (With the known rooms, four times a second.)
pub fn look_over(g: &mut Game) {
    super::cleared_rooms_build(g);
    let n = g.cleared.room_of_cell.as_ref().map_or(0, |r| r.len());
    if g.cleared.looked.len() != n { g.cleared.looked = alloc::vec![0; n]; }
    let mut eyes: Vec<CharId> = Vec::new();
    for gd in crate::seats::team_squad(g, &crate::seats::here(g)) {
        if g.gd[gd].state != crate::game::GuardState::Dead && !crate::control::man::blind(g, gd) { eyes.push(g.gd[gd].ch); }
    }
    for pl in crate::seats::men_up(g) { eyes.push(g.pl[pl].ch); }
    for c in eyes {
        let p = g.ch[c].pos;
        let gaze = crate::fogofwar::gaze_of(g, c);
        let way = m::atan2(gaze[0], gaze[2]);
        for (i, q) in unseen_by(g, p, None) {
            if v3::dist_xz(p, q) > NEAR_M
                && (m::abs(v3::wrap_angle(v3::yaw_to(p, q) - way)) > HALF || crate::scene_collision::segment_blocked(g, [p[0], EYE, p[2]], q, None, None, None)) { continue; }
            g.cleared.looked[i] = 1;
        }
    }
}

/// Where in the room he is in this man should look: floor nobody has looked over that he has a line to, the least turn of his gun off it, and not where one of `taken` looks already.
pub fn unlooked_by(g: &mut Game, gd: GuardId, taken: &[V3]) -> Option<V3> {
    let c = g.gd[gd].ch;
    let (p, aim) = (g.ch[c].pos, g.ch[c].aim_yaw);
    let room = super::cleared_rooms_room_at(g, p) as u16;
    if room == 0 || g.cleared.looked.is_empty() { return None; }
    let turn = |q: V3| m::abs(v3::wrap_angle(v3::yaw_to(p, q) - aim));
    let mut cells: Vec<V3> = unseen_by(g, p, Some(room)).into_iter().map(|(_, q)| q).filter(|&q| v3::dist_xz(p, q) > NEAR_M && !taken.iter().any(|&t| v3::dist_xz(t, q) < APART_M)).collect();
    crate::js::sort_by(&mut cells, |a, b| turn(*a) - turn(*b));
    cells.into_iter().find(|&q| !crate::scene_collision::segment_blocked(g, [p[0], EYE, p[2]], q, None, None, None))
}
