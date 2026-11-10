pub use ::game_types::doorway::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// The row of `states` for `gd`, there by now: a call that holds it across a `g` re-gets it.
macro_rules! st {
    ($g:expr, $gd:expr) => {
        $g.m.doorway.states.get_mut($gd).unwrap()
    };
}

mod gate;
pub use gate::doorway_gate;

/// What a new row of `states` holds.
fn fresh() -> St {
    St {
        t: -1e9, held_t: 0.0, pass: -1e9, by: None, spot: None, door: None, side: 1.0, path: None, no_spot: -1e9, round_t: -1e9,
        funnel_t: 0.0, step: None, step_by: 0.0, step_rest: -1e9, leaf_x: 0.0, leaf_z: 0.0, leaf_t: -1e9, leaf_seen: -1e9, leaf_by: -1e9,
    }
}

/// A man's row, made if new; the row itself is handed back.
fn state_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::doorway::St {
    g.m.doorway.states.get_or_insert_with(gd, fresh)
}

/// Distance on the floor.
fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// The two sides whose men keep these manners.
fn ours(g: &crate::game::Game, gd: GuardId) -> bool {
    matches!(g.team(gd), crate::teams::Team::Ours | crate::teams::Team::Hostile)
}

/// `p` in `d`'s frame's own axes.
fn rel(g: &crate::game::Game, d: DoorId, p: V3) -> crate::doorway::Rel {
    let f = crate::guards::frame_of(g, d);
    let (x, z) = (p[0] - f.hx, p[2] - f.hz);
    Rel { along: x * f.ux + z * f.uz, perp: x * f.nx + z * f.nz, f }
}

/// `p` in `d`'s opening: inside the leaf's span, within `half` of the frame line.
pub fn in_opening(g: &crate::game::Game, d: DoorId, p: V3, half: Option<f64>) -> bool {
    let half = half.unwrap_or(g.k.doormanners.half);
    let r = rel(g, d, p);
    r.along >= -0.05 && r.along <= r.f.w + 0.05 && m::abs(r.perp) <= half
}

/// `p` where `d`'s leaf swings: inside its arc between its stops, off the opening's own strip.
fn in_sweep(g: &crate::game::Game, d: DoorId, p: V3) -> bool {
    if g.dr[d].leaf.off { return false; }
    let r = rel(g, d, p);
    if r.along >= -0.05 && r.along <= r.f.w + 0.05 && m::abs(r.perp) <= g.k.doormanners.half { return false; }
    if m::hyp2(r.along, r.perp) > r.f.w + g.k.doormanners.sweep { return false; }
    let a = m::atan2(r.perp, r.along);
    a >= crate::doors::door_min_a(g, d) && a <= crate::doors::door_max_a(g, d) && m::abs(a) > 0.05
}

/// The doors whose frames are near enough to `p` for it to be in their way.
fn frames_by(g: &crate::game::Game, p: V3) -> impl Iterator<Item = DoorId> + '_ {
    g.doors.list.iter().copied().filter(move |d| flat(p, g.dr[*d].frame_centre) <= g.dr[*d].def.width + 1.0)
}

/// STAND: is `p` no place to stand about: in some frame's strip, or where its leaf swings.
pub fn doorway_stand(g: &crate::game::Game, p: V3) -> bool {
    frames_by(g, p).any(|d| in_opening(g, d, p, None) || in_sweep(g, d, p))
}

/// Something of the level stands between two floor points at shin height.
fn walled(g: &crate::game::Game, a: V3, b: V3) -> bool {
    crate::scene_collision::solid_blocked(g, [a[0], 0.45, a[2]], [b[0], 0.45, b[2]], None)
}

/// `p` in the lane of `d`'s opening, walls aside: where it is in it.
fn in_lane(g: &crate::game::Game, d: DoorId, p: V3) -> Option<Rel> {
    let k = &g.k.doormanners;
    if flat(p, g.dr[d].frame_centre) > k.lane_m + g.dr[d].def.width { return None; }
    let r = rel(g, d, p);
    let depth = m::abs(r.perp);
    let e = k.lane_out + k.lane_flare * depth;
    if depth > k.lane_m || r.along < -e || r.along > r.f.w + e { return None; }
    Some(r)
}

/// Not standing open enough to have a lane: on its hinges and latched, or under `laneAjar`.
fn no_lane(g: &crate::game::Game, d: DoorId) -> bool {
    !g.dr[d].leaf.off && (g.dr[d].leaf.latched || m::abs(g.dr[d].leaf.angle) < g.k.doormanners.lane_ajar)
}

/// STAND, the lane: the door standing open whose lane `p` is in.
pub fn doorway_lane(g: &crate::game::Game, p: V3, but: Option<DoorId>) -> Option<DoorId> {
    g.doors.list.iter().copied().find(|&d| Some(d) != but && !no_lane(g, d) && in_lane(g, d, p).is_some() && !walled(g, p, g.dr[d].frame_centre))
}

/// STAND, the lane: nav.findPath's `penalty` for a walk an order gives; `None`: no door standing open onto floor the team has not been on.
pub fn doorway_lane_price(g: &mut crate::game::Game, not: Option<&mut dyn FnMut(&mut crate::game::Game, DoorId) -> bool>) -> Option<crate::doorway::DoorwayLanePrice> {
    let mut not = not;
    let mut open: Vec<DoorId> = Vec::new();
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if no_lane(g, d) || match &mut not { Some(f) => f(g, d), None => false } { continue; }
        let a = crate::cleared::cleared_rooms_room_by(g, d, 1.0);
        let b = crate::cleared::cleared_rooms_room_by(g, d, -1.0);
        if !(js::truthy(a) && js::truthy(b) && crate::cleared::cleared_rooms_is_known(g, a) && crate::cleared::cleared_rooms_is_known(g, b)) { open.push(d); }
    }
    if open.is_empty() { return None; }
    Some(DoorwayLanePrice::Lanes { open, memo: JsMap::new() })
}

/// What a `DoorwayLanePrice` costs at a cell: the function doorwayLanePrice hands back.
pub fn doorway_lane_price_at(g: &crate::game::Game, price: &mut crate::doorway::DoorwayLanePrice, ix: f64, iz: f64) -> f64 {
    let DoorwayLanePrice::Lanes { open, memo } = price;
    let i = crate::Num(crate::scene_collision::nav_grid_idx(g, ix, iz));
    if let Some(v) = memo.get(i) { return *v; }
    let cell = g.col.nav.cell;
    let p: V3 = [(ix + 0.5) * cell, 0.0, (iz + 0.5) * cell];
    let strip = g.k.doormanners.lane_strip;
    let dear = open.iter().any(|&d| {
        let Some(r) = in_lane(g, d, p) else { return false };
        let no_leaf_here = g.dr[d].leaf.off || r.perp * g.dr[d].leaf.angle <= 0.0;
        !(m::abs(r.perp) < strip && no_leaf_here) && !walled(g, p, g.dr[d].frame_centre)
    });
    let v = if dear { g.k.doormanners.lane_dear } else { 1.0 };
    memo.set(i, v);
    v
}

/// STAND, the lane: the place for a man of the file whose place `p` is in an open door's lane; `None`: in no lane, or no such floor.
pub fn doorway_off_lane(g: &crate::game::Game, p: V3, from: V3, k: f64, taken: Option<&[V3]>) -> Option<V3> {
    let taken: &[V3] = taken.unwrap_or(&[]);
    let d = doorway_lane(g, p, None)?;
    let r = rel(g, d, p);
    let f = r.f;
    let e = g.k.doormanners.lane_out + g.k.doormanners.lane_flare * m::abs(r.perp) + g.k.doormanners.lane_clear;
    let fa = rel(g, d, from).along - f.w / 2.0;
    let first = if m::abs(fa) > 0.15 { m::sign(fa) } else if js::to_i32(k) & 1 != 0 { 1.0 } else { -1.0 };
    for s in [first, -first] {
        for j in 0..3 + 2 * taken.len() {
            let al = if s > 0.0 { f.w + e + j as f64 * 0.4 } else { -e - j as f64 * 0.4 };
            let q: V3 = [f.hx + f.ux * al + f.nx * r.perp, 0.0, f.hz + f.uz * al + f.nz * r.perp];
            if nav_blocked(g, q[0], q[2]) || walled(g, p, q) { break; }
            if behind_leaf(g, d, al, r.perp) { continue; }
            if doorway_stand(g, q) || doorway_lane(g, q, None).is_some() || taken.iter().any(|t| flat(*t, q) < g.k.doormanners.lane_apart) { continue; }
            return Some(q);
        }
    }
    None
}

/// Is the place at (along, perp) of `d`'s frame behind its open leaf.
fn behind_leaf(g: &crate::game::Game, d: DoorId, along: f64, perp: f64) -> bool {
    if g.dr[d].leaf.off { return false; }
    let w = g.dr[d].def.width;
    let angle = g.dr[d].leaf.angle;
    leaf_cuts(w, angle, along, perp)
}

/// The sum of behindLeaf: does the leaf, a body's breadth past its tip, cut the line from the place to the opening's middle.
fn leaf_cuts(w: f64, angle: f64, along: f64, perp: f64) -> bool {
    let l = w + 0.35;
    let (lx, lz) = (m::cos(angle) * l, m::sin(angle) * l);
    let (cx, cz) = (w / 2.0 - along, -perp);
    let den = cx * lz - cz * lx;
    if m::abs(den) < 1e-6 { return false; }
    let s = -(along * lz - perp * lx) / den;
    let t = -(along * cz - perp * cx) / den;
    s > 0.0 && s < 1.0 && t > 0.0 && t < 1.0
}

/// `leafRound`: does this man go round a leaf standing open across his walk.
pub fn rounds_leaves(g: &crate::game::Game, gd: GuardId) -> bool { g.team(gd).ours() }

/// leafStall: he is walking on the spot by a leaf standing open (or was, less than `leafStallPush` ago): this frame the leaf is his to push.
pub fn leaf_stalled(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !g.team(gd).ours() || crate::condition::down(g, g.gd[gd].ch) { return false; }
    state_of(g, gd);
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let t = g.time;
    let push = g.k.doormanners.leaf_stall_push;
    let (stall_m, stall_secs, stall_reach, stall_ahead) = (g.k.doormanners.leaf_stall_m, g.k.doormanners.leaf_stall_secs, g.k.doormanners.leaf_stall_reach, g.k.doormanners.leaf_stall_ahead);
    let s = st!(g, gd);
    if t < s.leaf_by && t >= s.leaf_by - push { return true; }
    if t - s.leaf_seen > 0.25 || t < s.leaf_seen || m::hyp2(p[0] - s.leaf_x, p[2] - s.leaf_z) > stall_m { s.leaf_x = p[0]; s.leaf_z = p[2]; s.leaf_t = t; }
    s.leaf_seen = t;
    if t - s.leaf_t < stall_secs { return false; }
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if g.dr[d].leaf.off || g.dr[d].leaf.latched { continue; }
        let angle = g.dr[d].leaf.angle;
        if m::abs(angle) < 0.8 || flat(p, g.dr[d].frame_centre) > g.dr[d].def.width + 1.5 { continue; }
        let r = rel(g, d, p);
        let (cx, cz) = (m::cos(angle), m::sin(angle));
        let k = v3::clamp(r.along * cx + r.perp * cz, 0.0, r.f.w);
        let (qa, qp) = (k * cx - r.along, k * cz - r.perp);
        let far = m::hyp2(qa, qp);
        if far > stall_reach { continue; }
        if (cx * r.perp - cz * r.along) * angle <= 0.0 { continue; }
        let v = g.ch[c].vel;
        let (va, vp) = (v[0] * r.f.ux + v[2] * r.f.uz, v[0] * r.f.nx + v[2] * r.f.nz);
        if va * qa + vp * qp <= stall_ahead * m::hyp2(va, vp) * far { continue; }
        let s = st!(g, gd);
        s.leaf_by = t + push;
        s.leaf_t = t;
        return true;
    }
    false
}

/// The first frame his walk goes through within `reach`.
fn door_ahead(g: &crate::game::Game, gd: GuardId) -> Option<crate::doorway::DoorAhead> {
    let pos = g.ch[g.gd[gd].ch].pos;
    let reach = g.k.doormanners.reach;
    let (mut a, mut run) = (pos, 0.0);
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() && run < reach {
        let b = g.gd[gd].path[i];
        let mut best: Option<DoorAhead> = None;
        for n in 0..g.doors.list.len() {
            let d = g.doors.list[n];
            if flat(pos, g.dr[d].frame_centre) > reach + g.dr[d].def.width { continue; }
            let Some(x) = crate::guards::crosses_door(g, a, b, d) else { continue };
            let dist = run + flat(a, x.at);
            if dist <= reach && best.map_or(true, |b| dist < b.dist) { best = Some(DoorAhead { d, side: x.side, dist }); }
        }
        if best.is_some() { return best; }
        run += flat(a, b);
        a = b;
        i += 1;
    }
    None
}

/// Ease `gd` to `p` over open floor, his feet moved; true = there.
fn step_to(g: &mut crate::game::Game, gd: GuardId, p: V3, pace: f64, dt: f64, turn: bool) -> bool {
    let c = g.gd[gd].ch;
    let at = g.ch[c].pos;
    let (dx, dz) = (p[0] - at[0], p[2] - at[2]);
    let d = m::hyp2(dx, dz);
    if d < 0.1 {
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        return true;
    }
    let was = g.gd[gd].speed;
    g.gd[gd].speed = v3::damp(was, pace * v3::clamp(d / 0.45, 0.35, 1.0), 6.0, dt);
    crate::guards::team_lean(g, gd, was, dt);
    let speed = g.gd[gd].speed;
    let s = m::min(d, speed * dt);
    g.ch[c].pos[0] += dx / d * s;
    g.ch[c].pos[2] += dz / d * s;
    g.ch[c].vel = [dx / d * speed, 0.0, dz / d * speed];
    if turn { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, m::atan2(dx, dz), 5.5 * dt); }
    false
}

/// Is `p` floor nobody has, by `apart`, nor is waiting at.
fn spot_free(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId, p: V3, apart: f64) -> bool {
    if nav_blocked(g, p[0], p[2]) { return false; }
    if g.team(gd).ours() && !crate::condition::down(g, g.pl[seat.player].ch) && flat(g.ch[g.pl[seat.player].ch].pos, p) < apart { return false; }
    let t = g.time;
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if o == gd || crate::condition::down(g, g.gd[o].ch) { continue; }
        if flat(g.ch[g.gd[o].ch].pos, p) < apart { return false; }
        let s = match g.m.doorway.states.get(o) { Some(so) if t - so.t <= 0.5 && t >= so.t => so.spot, _ => None };
        if let Some(s) = s { if flat(s, p) < apart { return false; } }
    }
    true
}

/// A place off a jamb of `d` on face `side` for a held man: the nearest he can walk straight to that is free, off every swing and every opening.
fn jamb_spot(g: &crate::game::Game, gd: GuardId, d: DoorId, side: f64) -> Option<V3> {
    let f = crate::guards::frame_of(g, d);
    let pos = g.ch[g.gd[gd].ch].pos;
    let (jamb_past, file_gap, jamb_out, apart) = (g.k.doormanners.jamb_past, g.k.doormanners.file_gap, g.k.doormanners.jamb_out, g.k.doormanners.apart);
    let mut best: Option<V3> = None;
    let mut bd = m::INF;
    for k in 0..3 {
        let kf = k as f64;
        for lat in [f.w + jamb_past + kf * file_gap, -(jamb_past + kf * file_gap)] {
            let p: V3 = [f.hx + f.ux * lat + f.nx * side * jamb_out, 0.0, f.hz + f.uz * lat + f.nz * side * jamb_out];
            let dd = flat(pos, p);
            if dd >= bd || !spot_free(g, &crate::seats::here(g), gd, p, apart) { continue; }
            if walled(g, pos, p) { continue; }
            if frames_by(g, p).any(|o| in_opening(g, o, p, Some(0.6)) || in_sweep(g, o, p)) { continue; }
            best = Some(p);
            bd = dd;
        }
    }
    best
}

/// Leave the place off a jamb; `replan` walks him afresh if he had one.
fn release(g: &mut crate::game::Game, gd: GuardId, replan: bool) {
    let s = st!(g, gd);
    let was = s.spot.is_some();
    s.spot = None;
    s.door = None;
    if was && replan {
        if let Some(goal) = g.gd[gd].path_goal { crate::guards::go_to(g, gd, goal, Some(true)); }
    }
}

/// Pace by the gap to the man ahead: full at `gap`, none at `gapMin`.
fn gap_k(g: &crate::game::Game, d: f64) -> f64 {
    v3::clamp((d - g.k.doormanners.gap_min) / (g.k.doormanners.gap - g.k.doormanners.gap_min), 0.0, 1.0)
}

/// The mark his order gave him, if any.
fn order_post(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId) -> Option<V3> {
    let a = crate::orders::squad_orders_of(seat, gd)?;
    g.tab.orders_assignment[a].post
}

/// On the mark his order gave him.
fn on_own(g: &crate::game::Game, seat: &crate::seats::SeatRef, o: Option<GuardId>) -> bool {
    let Some(o) = o else { return false };
    let Some(post) = order_post(g, seat, o) else { return false };
    flat(g.ch[g.gd[o].ch].pos, post) < g.k.doormanners.own_m
}

/// `g.col.nav.isBlocked`.
fn nav_blocked(g: &crate::game::Game, x: f64, z: f64) -> bool {
    crate::scene_collision::nav_grid_is_blocked(g, x, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf_cuts_as_the_typescript() {
        // (width, angle, along, perp, answer): behindLeaf's lines (doorway.ts:231-234, not exported: copied to a script) run under bun with EXACT.on
        for (w, a, al, pe, want) in LEAF { assert_eq!(leaf_cuts(*w, *a, *al, *pe), *want, "{w} {a} {al} {pe}"); }
    }
    #[test]
    fn flat_bits_as_the_typescript() {
        assert_eq!(flat([1.25, 9.0, -3.5], [0.1, -2.0, 7.3]).to_bits(), FLAT_BITS);
    }
    const LEAF: &[(f64, f64, f64, f64, bool)] = &[(0.9000, 1.5700, 0.2000, 0.6000, false), (0.9000, 1.5700, -0.3000, 0.6000, true), (0.9000, -1.2000, 0.1000, -0.5000, true), (0.9000, 1.2000, 0.1000, -0.5000, false), (1.2000, 0.7000, 0.5000, 0.1000, false), (0.9000, 1.5700, 0.4500, 0.0000, false), (0.9000, 2.0000, -0.4000, 0.3000, true), (0.9000, 0.0000, 0.2000, 0.0000, false), (0.9000, 1.5700, 1.4000, 0.3000, false)];
    const FLAT_BITS: u64 = 0x4025b8dc1b2c73b7;
}

/// before a door: this far off its leaf, this far past its frame
const BEFORE_DEEP: f64 = 1.1;
const BEFORE_WIDE: f64 = 0.25;
/// against a leaf: a man's own radius and a little
const BRUSH_M: f64 = 0.6;

/// FIGHT: a man planted in an open doorway `funnel` seconds steps off the opening. True = this file moved his feet this frame.
pub fn off_threshold(g: &mut crate::game::Game, gd: GuardId, aim_at: Option<V3>, dt: f64, need_shot: bool) -> bool {
    if !ours(g, gd) || g.gd[gd].script.is_some() { return false; }
    let aim_at = match aim_at { Some(a) => a, None => return false };
    let _ = state_of(g, gd);   // makes the row if it is new; every read and write below goes through the table
    let c = g.gd[gd].ch;
    let t = g.time;
    let pace = m::min(g.tune.guard_investigate, g.k.doormanners.step);
    let (step, step_by, step_rest) = { let st = g.m.doorway.states.get(gd).unwrap(); (st.step, st.step_by, st.step_rest) };
    if let Some(step) = step {
        if t > step_by || step_to(g, gd, step, pace, dt, false) {
            let funnel_rest = g.k.doormanners.funnel_rest;
            let st = g.m.doorway.states.get_mut(gd).unwrap();
            st.step = None;
            st.step_rest = t + funnel_rest;
            return false;
        }
        return true;
    }
    if t < step_rest { return false; }
    let pos = g.ch[c].pos;   // nothing below moves him before the last step_to
    let open = frames_by(g, pos).find(|d| !g.dr[*d].leaf.latched && m::abs(g.dr[*d].leaf.angle) >= 0.6 && in_opening(g, *d, pos, None));
    let Some(door) = open else { st!(g, gd).funnel_t = 0.0; return false };
    let funnel = g.k.doormanners.funnel;
    {
        let st = g.m.doorway.states.get_mut(gd).unwrap();
        st.funnel_t += dt;
        if st.funnel_t < funnel { return false; }
        st.funnel_t = 0.0;
    }
    let f = crate::guards::frame_of(g, door);
    let off = (aim_at[0] - f.hx) * f.nx + (aim_at[2] - f.hz) * f.nz;
    let sgn: f64 = if off >= 0.0 { 1.0 } else { -1.0 };
    // the squad steps THROUGH, to the side its man is on (failing that, back off it); a defender steps back on his own side, never out toward them
    let foe = g.team(gd) == crate::teams::Team::Hostile;
    let cov = foe;   // (stepCover: settled on)
    // them ALONG the door's wall: the half-plane says nothing there, so covered floor on either face will do
    let along = (aim_at[0] - f.hx) * f.ux + (aim_at[2] - f.hz) * f.uz - f.w / 2.0;
    let flat_on = cov && m::abs(off) < m::max(0.5, m::abs(along) * m::tan(g.k.doormanners.plane_deg * m::PI / 180.0));
    let sides: Vec<f64> = if foe { if flat_on { vec![-sgn, sgn] } else { vec![-sgn] } } else { vec![sgn, -sgn] };
    let here = if cov { exposed(g, aim_at, pos) } else { 0 };
    // the passes (line, cover): a spot with the shot; (a defender) a covered spot; then, with nobody in his sights, any free spot off the opening
    let passes: &[(bool, bool)] = if need_shot { &[(true, false)] } else if cov { &[(true, false), (false, true), (false, false)] } else { &[(true, false), (false, false)] };
    let (past, thru) = (g.k.doormanners.past, g.k.doormanners.through);
    for &(line, cover) in passes {
        for &side in &sides {
            for lat in [f.w + past, -past] {
                if foe && side == sgn && !cover { continue; }   // (the face the half-plane calls theirs: only ever under cover)
                let p: V3 = [f.hx + f.ux * lat + f.nx * side * thru, 0.0, f.hz + f.uz * lat + f.nz * side * thru];
                if !spot_free(g, &crate::seats::here(g), gd, p, 0.8)
                    || walled(g, pos, p)
                    || (line && crate::scene_collision::shot_blocked(g, [p[0], 1.4, p[2]], aim_at, None, None)) { continue; }
                if frames_by(g, p).any(|o| in_opening(g, o, p, None) || (o != door && in_sweep(g, o, p))) { continue; }
                // never to a spot more exposed to them than the frame he stands in, and the cover pass takes covered floor only
                if cov {
                    let e = exposed(g, aim_at, p);
                    if if cover { e > 0 } else { e > here } {
                        continue;
                    }
                }
                {
                    let st = g.m.doorway.states.get_mut(gd).unwrap();
                    st.step = Some(p);
                    st.step_by = t + 1.5;
                }
                step_to(g, gd, p, pace, dt, false);
                return true;
            }
        }
    }
    g.m.doorway.states.get_mut(gd).unwrap().step_rest = t + 1.0;
    false
}

/// How a man stood at p (chest and head) is exposed to a rifle at `aim_at`: 0 covered, 1 reached only through glass, 2 reached in clear air. (the closure `exposed` of offThreshold)
fn exposed(g: &Game, aim_at: V3, p: V3) -> i32 {
    let mut e = 0;
    for y in [1.1, 1.5] {
        let at: V3 = [p[0], y, p[2]];
        let energy = g.k.glasspen.stack_energy;
        if crate::scene_collision::shot_blocked(g, aim_at, at, Some(0.02), Some(energy)) { continue; }
        if !crate::scene_collision::solid_blocked(g, aim_at, at, None) { return 2; }
        e = 1;
    }
    e
}

/// `p` is before a door that is not `but`: in front of its leaf, a step off either face. Nobody is posted there by an order of another door: he is in the way of whoever works it, and in
/// front of whatever comes through.
pub fn before_another_door(g: &crate::game::Game, p: V3, but: DoorId) -> bool {
    for &d in &g.doors.list {
        if d == but || g.dr[d].leaf.off { continue; }
        let r = rel(g, d, p);
        if m::abs(r.perp) < BEFORE_DEEP && r.along > -BEFORE_WIDE && r.along < r.f.w + BEFORE_WIDE { return true; }
    }
    false
}

/// HE MEANS TO GO THROUGH IT: what is left of his way, or his step to where he is sent, crosses the door's frame.
fn means_to_pass(g: &crate::game::Game, gd: GuardId, d: DoorId) -> bool {
    let man = &g.gd[gd];
    let mut a = g.ch[man.ch].pos;
    for &q in man.path.iter().skip(man.path_i) {
        if crate::guards::crosses_door(g, a, q, d).is_some() { return true; }
        a = q;
    }
    man.script.as_ref().and_then(|s| s.goal).map_or(false, |q| crate::guards::crosses_door(g, a, q, d).is_some())
}

/// HE ONLY BRUSHES IT: a leaf on its latch he is against and does not mean to go through. He is put off it, and it stays shut: a door is opened by somebody going through it, or by what
/// works it (a hand, a boot, a charge).
pub fn only_brushes(g: &crate::game::Game, gd: GuardId) -> bool {
    let p = g.ch[g.gd[gd].ch].pos;
    let (mut shut, mut open) = (false, false);
    for &d in &g.doors.list {
        if g.dr[d].leaf.off { continue; }
        let fc = g.dr[d].frame_centre;
        if v3::dist_xz(p, fc) > g.dr[d].def.width + BRUSH_M { continue; }
        if !g.dr[d].leaf.latched { open = true; continue; }
        let r = rel(g, d, p);
        if m::abs(r.perp) > BRUSH_M || r.along < -BRUSH_M || r.along > r.f.w + BRUSH_M { continue; }
        if means_to_pass(g, gd, d) { return false; }
        shut = true;
    }
    shut && !open   // (an open leaf beside it he moves as ever)
}

/// Who the PASS gate has `gd` held for this frame: 'Player', a man's callsign, or None when he is not held short of a man.
pub fn doorway_held_by(g: &crate::game::Game, gd: GuardId) -> Option<String> {
    let time = g.time;
    match g.m.doorway.states.get(gd) {
        Some(st) if time - st.t <= 0.25 && time >= st.t && st.held_t > 0.0 => st.by.clone(),
        _ => None,
    }
}

/// The PASS gate has `gd` held this frame: short of a frame for a man in it, or stepped off to a jamb.
pub fn door_gate_stopped(g: &crate::game::Game, gd: GuardId) -> bool {
    let time = g.time;
    match g.m.doorway.states.get(gd) {
        Some(st) => time - st.t <= 0.25 && time >= st.t && (st.held_t > 0.0 || st.spot.is_some()),
        None => false,
    }
}

/// The nearest floor to `p`, within `clearM`, that is not `bad`, walkable straight from `p`, never through a frame to its other face, `apart` clear of every
/// `taken`; of the nearest ring that has any, the spot farthest from the doors by it.
pub fn doorway_clear_near(g: &mut crate::game::Game, p: V3, bad: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool, taken: Option<&[V3]>, apart: Option<f64>, face: Option<V3>) -> Option<V3> {
    let taken: &[V3] = taken.unwrap_or(&[]);
    let apart = apart.unwrap_or(g.k.doormanners.apart);
    let (half, clear_m) = (g.k.doormanners.half, g.k.doormanners.clear_m);
    // the doors by p, with p's place against each (`face`: with p in a frame's own strip, the face of that frame the answer is on)
    let mut by: Vec<(DoorId, Rel)> = Vec::new();
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if !(flat(p, g.dr[d].frame_centre) <= g.dr[d].def.width + 1.0 + clear_m) { continue; }
        let at = rel(g, d, p);
        let at = match face { Some(fc) if m::abs(at.perp) <= half => rel(g, d, fc), _ => at };
        by.push((d, at));
    }
    let mut r = 0.2;
    while r <= clear_m + 1e-6 {
        let mut best: Option<V3> = None;
        let mut bs = -1.0;
        for k in 0..12 {
            let a = k as f64 * m::PI / 6.0;
            let q: V3 = [p[0] + m::sin(a) * r, 0.0, p[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
                || bad(g, q)
                || taken.iter().any(|t| flat(*t, q) < apart)
                || walled(g, p, q) { continue; }
            let mut ok = true;
            let mut far = m::INF;
            for &(d, at) in &by {
                let rq = rel(g, d, q);
                far = m::min(far, flat(q, g.dr[d].frame_centre));
                if m::abs(at.perp) > half && rq.perp * at.perp < 0.0 && rq.along > -0.3 && rq.along < rq.f.w + 0.3 { ok = false; break; }
            }
            if ok && far > bs { best = Some(q); bs = far; }
        }
        if best.is_some() { return best; }
        r += 0.2;
    }
    None
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    /// The held window: a row counts only if written this quarter second, and not from the future (a load).
    #[test]
    fn held_window() {
        let mut g = Game::blank();
        let gd = GuardId(0);
        assert!(!door_gate_stopped(&mut g, gd));
        assert_eq!(doorway_held_by(&mut g, gd), None);
        let mut st = St::default();
        st.t = 10.0;
        st.held_t = 1.0;
        st.by = Some(String::from("Player"));
        g.m.doorway.states.set(gd, st);
        g.time = 10.25;
        assert!(door_gate_stopped(&mut g, gd));
        assert_eq!(doorway_held_by(&mut g, gd).as_deref(), Some("Player"));
        g.time = 10.26;
        assert!(!door_gate_stopped(&mut g, gd));
        assert_eq!(doorway_held_by(&mut g, gd), None);
        g.time = 9.9;
        assert!(!door_gate_stopped(&mut g, gd));
        // stepped off to a jamb: held, but not short of a man
        g.time = 10.0;
        let st = g.m.doorway.states.get_mut(gd).unwrap();
        st.held_t = 0.0;
        st.spot = Some([1.0, 0.0, 1.0]);
        assert!(door_gate_stopped(&mut g, gd));
        assert_eq!(doorway_held_by(&mut g, gd), None);
    }
}
