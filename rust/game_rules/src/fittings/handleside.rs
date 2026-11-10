pub use ::game_types::handleside::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

mod gate;
pub use gate::{handle_gate, handle_gate_stopped};

// ---- local helpers ----

/// `path.slice(from, to)`, with the language's clamping.
fn slice_path(path: &[V3], from: usize, to: usize) -> Vec<V3> {
    let to = if to > path.len() { path.len() } else { to };
    if from >= to { Vec::new() } else { path[from..to].to_vec() }
}
/// `const ok`, of findFace.
fn ok_at(g: &Game, ref_: V3, p: V3, nav: bool) -> bool {
    stands(g, p, Some(nav)) && !crate::scene_collision::low_walled(g, ref_, p)
}
/// `const al`, of nearStand and the frame's own along-the-leaf reads.
fn along_of(f: &crate::doorway::RelF, p: V3) -> f64 {
    (p[0] - f.hx) * f.ux + (p[2] - f.hz) * f.uz
}
/// `const past`, of nearStand.
fn past_jamb(g: &Game, d: DoorId, side: f64, min: f64, in_off: f64, a: f64, off: f64) -> V3 {
    if a >= min { pt_of(g, d, a, off, side) } else { pt_of(g, d, min, m::min(off, in_off), side) }
}
// ---- the file's functions ----

/// A man's place at a door is his only while he keeps coming for it.
fn fresh(g: &crate::game::Game, st: &crate::handleside::St) -> bool {
    g.time >= st.seen && g.time - st.seen < 0.5
}

/// Which face of the closed leaf `p` is on.
pub(crate) fn side_at(g: &crate::game::Game, d: DoorId, p: V3) -> f64 {
    let f = crate::guards::frame_of(g, d);
    if (p[0] - f.hx) * f.nx + (p[2] - f.hz) * f.nz >= 0.0 { 1.0 } else { -1.0 }
}

/// How far `p` stands off the frame's line.
fn off_at(g: &crate::game::Game, d: DoorId, p: V3) -> f64 {
    let f = crate::guards::frame_of(g, d);
    m::abs((p[0] - f.hx) * f.nx + (p[2] - f.hz) * f.nz)
}

/// The floor point `along` the leaf from its hinge, `off` it on face `side`.
pub(crate) fn pt_of(g: &crate::game::Game, d: DoorId, along: f64, off: f64, side: f64) -> V3 {
    let f = crate::guards::frame_of(g, d);
    [f.hx + f.ux * along + f.nx * side * off, 0.0, f.hz + f.uz * along + f.nz * side * off]
}

/// The handle, on the floor.
fn handle_at(g: &crate::game::Game, d: DoorId) -> V3 {
    let along = g.dr[d].def.width - g.k.handle.handle_in;
    pt_of(g, d, along, 0.0, 1.0)
}

/// Just off the leaf by its handle, on face `side`.
fn handle_face(g: &crate::game::Game, d: DoorId, side: f64) -> V3 {
    let along = g.dr[d].def.width - g.k.handle.handle_in;
    pt_of(g, d, along, 0.2, side)
}

/// How far a leaf stands open, the way it opens.
fn opened(g: &crate::game::Game, d: DoorId) -> f64 {
    g.dr[d].leaf.angle * crate::doors::door_swing(g, d)
}

/// Does the leaf come round through a body `body` about `p`.
fn swept(g: &crate::game::Game, d: DoorId, p: V3, body: f64) -> bool {
    let s = crate::doors::doors_swing_of(g, d);
    crate::doors::door_swing_sweeps(&s, p[0], p[2], Some(body))
}

/// Can a man stand at p: nothing of the level inside his body there (and, `nav`, on a cell the walkers' grid calls free).
fn stands(g: &crate::game::Game, p: V3, nav: Option<bool>) -> bool {
    let nav = nav.unwrap_or(false);
    if nav && crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return false; }
    let mut q: V3 = [p[0], 0.0, p[2]];
    crate::scene_collision::collide_circle(g, &mut q, 0.3, 0.2, 1.5, None);
    v3::dist_xz(p, q) < 0.02
}

/// Is `p` at the handle of d: on the handle half, within reach, and on the face it opens into clear of the sweep.
fn at_handle(g: &crate::game::Game, d: DoorId, p: V3, reach: Option<f64>) -> bool {
    let reach = reach.unwrap_or(g.k.handle.reach + 0.05);
    let f = crate::guards::frame_of(g, d);
    let along = along_of(&f, p);
    if along < f.w / 2.0 || v3::dist_xz(p, handle_at(g, d)) > reach { return false; }
    if !crate::doors::doors_pull_face(g, d, p) { return true; }
    !swept(g, d, p, g.k.handle.body)
}

/// A face's places, found once a face.
fn face_of(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::handleside::Face {
    let swing = crate::doors::door_swing(g, d);
    let stale = match g.m.handleside.faces.get(d) { Some(c) => c.swing != swing, None => true };
    if stale {
        g.m.handleside.faces.set(d, crate::handleside::HandlesideModFacesValue { swing, at: [None, None] });
    }
    let i = if side > 0.0 { 0 } else { 1 };
    if let Some(f) = g.m.handleside.faces.get(d).unwrap().at[i] { return f; }
    let f = find_face(g, d, side);
    g.m.handleside.faces.get_mut(d).unwrap().at[i] = Some(f);
    f
}

/// The place a man works d's handle from on face `side`, or None where that face has none.
pub fn handle_spot(g: &mut crate::game::Game, d: DoorId, side: f64) -> Option<crate::handleside::Spot> {
    face_of(g, d, side).stand
}

/// Searches a face's floor for its stand and its way through.
fn find_face(g: &crate::game::Game, d: DoorId, side: f64) -> crate::handleside::Face {
    let k = g.k.handle;
    let w = g.dr[d].def.width;
    let hd = handle_at(g, d);
    let ref_ = handle_face(g, d, side);
    let mid = pt_of(g, d, w / 2.0, 0.5, side);
    let pull = crate::doors::doors_pull_face(g, d, mid);
    let mut via: Option<[V3; 2]> = None;
    for inn in [k.push_in, k.push_in + 0.08, k.push_in + 0.16] {
        let a = v3::clamp(w - inn, w / 2.0, w);
        let pa = pt_of(g, d, a, k.push_off, side);
        let pb = pt_of(g, d, a, k.push_thru, -side);
        if ok_at(g, ref_, pa, false) && stands(g, pb, None) && !crate::scene_collision::low_walled(g, pa, pb) { via = Some([pa, pb]); break; }
    }
    let mut stand: Option<crate::handleside::Spot> = None;
    if !pull {
        'search: for inn in [k.stand_in, k.stand_in + 0.08, k.stand_in + 0.16, k.stand_in + 0.24] {
            for off in [k.stand_off, k.stand_off + 0.12] {
                let p = pt_of(g, d, v3::clamp(w - inn, w / 2.0, w), off, side);
                if ok_at(g, ref_, p, false) {
                    stand = Some(crate::handleside::Spot { pos: p, yaw: v3::yaw_to(p, hd), pull, far: None });
                    break 'search;
                }
            }
        }
        return crate::handleside::Face { stand, via };
    }
    let s = crate::doors::doors_swing_of(g, d);
    let mut best: Option<V3> = None;
    let mut bc = f64::INFINITY;
    let mut a = w - 0.5;
    while a <= w + 0.8 + 1e-6 {
        let mut off = 0.4;
        while off <= 1.0 + 1e-6 {
            let p = pt_of(g, d, a, off, side);
            let r = v3::dist_xz(p, hd);
            if !(r > k.reach) {
                let c = r + if a < w { 0.25 } else { 0.0 };   // beside the jamb before in front of the opening
                if !(c >= bc || crate::doors::door_swing_sweeps(&s, p[0], p[2], Some(k.body + k.slack)) || !ok_at(g, ref_, p, true)) {
                    best = Some(p);
                    bc = c;
                }
            }
            off += 0.05;
        }
        a += 0.1;
    }
    if let Some(b) = best {
        return crate::handleside::Face { stand: Some(crate::handleside::Spot { pos: b, yaw: v3::yaw_to(b, hd), pull, far: None }), via };
    }
    // a door in a corner: the nearest floor to its handle that the leaf does not sweep, in front of its handle half: the long reach
    let mut a = w / 2.0;
    while a <= w + 0.4 + 1e-6 {
        let mut off = 0.5;
        while off <= 1.8 + 1e-6 {
            let p = pt_of(g, d, a, off, side);
            let r = v3::dist_xz(p, hd);
            if !(r > k.far_reach || r >= bc) && !(crate::doors::door_swing_sweeps(&s, p[0], p[2], Some(k.body + k.slack)) || !ok_at(g, ref_, p, false)) {
                best = Some(p);
                bc = r;
            }
            off += 0.05;
        }
        a += 0.1;
    }
    let stand = best.map(|b| crate::handleside::Spot { pos: b, yaw: v3::yaw_to(b, hd), pull, far: Some(true) });
    crate::handleside::Face { stand, via }
}

/// The angle under which a leaf stands in a man's way.
fn ajar_of(g: &crate::game::Game, d: DoorId) -> f64 {
    let open = crate::doors::doors_swing_of(g, d).open;
    m::min(g.k.handle.ajar, 0.8 * open)
}

/// A leaf that wants a hand from `gd` standing at `from`: in his way, on its hinges, not locked against him, and nobody's already.
fn wants_hand(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId, d: DoorId, from: V3) -> bool {
    if g.team(gd) == crate::teams::Team::Civilian { return false; }
    if g.dr[d].leaf.off { return false; }
    let latched = g.dr[d].leaf.latched;
    if !latched {
        let under = if crate::doors::doors_pull_face(g, d, from) { m::min(g.k.handle.clear, ajar_of(g, d)) } else { ajar_of(g, d) };
        if opened(g, d) >= under { return false; }
    }
    if !latched && (g.dr[d].leaf.pull_t > 0.0 || g.dr[d].leaf.vel * crate::doors::door_swing(g, d) > 0.5) { return false; }
    if g.team(gd).ours() {
        let p = seat.player;
        // `new Set([...keys, ...codes])` where he holds codes, else his keys: lent by copy, the stub wants `g` too
        let mut keys: JsSet<String> = g.pl[p].keys.clone();
        if g.pl[p].codes.len() > 0 { for c in g.pl[p].codes.keys() { keys.add(c); } }
        if crate::doors::doors_locked_out(g, d, from, Some(&keys)) { return false; }
    }
    true
}

/// The walk from `a` to a place at a handle, or None when that is no short way.
fn way_to(g: &crate::game::Game, a: V3, to: V3) -> Option<Vec<V3>> {
    let d = v3::dist_xz(a, to);
    if d < 0.5 || crate::scene_collision::nav_grid_walkable(g, a, to) { return Some(vec![to]); }
    let mut p = crate::scene_collision::nav_grid_find_path(g, a, to, Some(1500.0), None)?;
    let mut len = 0.0;
    let mut q = a;
    for i in 0..p.len() {
        len += v3::dist_xz(q, p[i]);
        q = p[i];
    }
    if len > d * 2.0 + 1.5 { return None; }
    if v3::dist_xz(p[p.len() - 1], to) < 0.4 { p.pop(); }
    p.push(to);
    Some(p)
}

/// A man's feet taken the last of the way onto `to`: true once he stands on it.
fn step_to(g: &mut crate::game::Game, gd: GuardId, to: V3, dt: f64, speed: f64, strafe: bool, pass: Option<bool>) -> bool {
    let pass = pass.unwrap_or(false);
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let dx = to[0] - pos[0];
    let dz = to[2] - pos[2];
    let dist = m::hyp2(dx, dz);
    if dist <= if pass { 0.08 } else { g.k.handle.near } { return true; }
    let was = g.gd[gd].speed;
    let want = speed * if pass { v3::clamp(dist / 0.3, 0.6, 1.0) } else { v3::clamp(dist / 0.6, 0.3, 1.0) };   // `pass`: a point he walks through, not one he stops on
    if strafe || dist < 0.35 {
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, want, 4.0, dt);
        crate::guards::team_lean(g, gd, was, dt);
        let sp = g.gd[gd].speed;
        g.ch[c].vel = [dx / dist * sp, 0.0, dz / dist * sp];
    } else {
        let yaw = m::atan2(dx, dz);
        let facing = m::cos(v3::wrap_angle(yaw - g.ch[c].body_yaw));
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, want * v3::clamp(facing * 1.5, 0.15, 1.0), 4.0, dt);
        crate::guards::team_lean(g, gd, was, dt);
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw, 5.5 * dt);
        let (by, sp) = (g.ch[c].body_yaw, g.gd[gd].speed);
        g.ch[c].vel = [m::sin(by) * sp, 0.0, m::cos(by) * sp];
    }
    let sp = g.gd[gd].speed;
    let vel = g.ch[c].vel;
    let step = m::min(dist, m::max(sp, 0.3) * dt);
    let vl = m::hyp2(vel[0], vel[2]);
    let ux = if vl > 1e-4 { vel[0] / vl } else { dx / dist };
    let uz = if vl > 1e-4 { vel[2] / vl } else { dz / dist };
    let p = g.ch[c].pos;
    let np = [p[0] + ux * step, p[1], p[2] + uz * step];
    feet_to(g, gd, np);
    false
}

/// His feet put there, as a step he took himself.
fn feet_to(g: &mut crate::game::Game, gd: GuardId, p: V3) {
    let c = g.gd[gd].ch;
    crate::character::pos_replace(g, c, p);
    g.gd[gd].last_pos = p;
    g.gd[gd].stuck_t = 0.0;
}

/// The seconds a man's body must lean on a leaf from the face it opens into before it is pulled round; None = doors.ts's own beat.
pub fn hand_press(g: &crate::game::Game, gd: GuardId) -> Option<f64> {
    if g.team(gd) == crate::teams::Team::Civilian || g.gd[gd].held.is_some() { return None; }
    if let Some(r) = g.m.handleside.rested.get(gd).copied() { if g.time < r.until { return None; } }
    let coming = g.m.handleside.states.get(gd).is_some_and(|st| fresh(g, st));
    if coming || g.gd[gd].speed < 0.3 { Some(f64::INFINITY) } else { Some(g.k.handle.press) }
}

/// A door order's lead about to open `d` by hand: the place he works it from, or None when he stays where he is.
pub fn tool_spot(g: &mut crate::game::Game, d: DoorId, side: f64, at: V3, order: Option<bool>) -> Option<crate::handleside::ToolSpot> {
    let order = order.unwrap_or(false);
    let k = g.k.handle;
    if at_handle(g, d, at, Some(k.reach - 0.07)) { return None; }
    if order && !crate::doors::doors_pull_face(g, d, at) {   // a door order's lead, whichever cast of doortools.ts gives it
        match near_stand(g, d, side, at) {
            Some(crate::handleside::NearStand::Post) => {
                return None;
            }
            Some(crate::handleside::NearStand::List(n)) => {
                let hd = handle_at(g, d);
                return Some(crate::handleside::ToolSpot { pos: n, yaw: m::atan2(hd[0] - n[0], hd[2] - n[2]), far: false });
            }
            None => {}
        }
    }
    if k.tool_post > 0.0 && !crate::doors::doors_pull_face(g, d, at) {   // beside the latch jamb on the face it swings away from: nothing comes round at him, and the handle is an arm away
        let f = crate::guards::frame_of(g, d);
        let along = along_of(&f, at);
        if along >= f.w && side_at(g, d, at) == side && v3::dist_xz(at, handle_at(g, d)) <= k.tool_post && off_at(g, d, at) <= 0.8 {
            let r = handle_face(g, d, side);
            if !crate::scene_collision::low_walled(g, r, at) { return None; }
        }
    }
    let h = handle_spot(g, d, side);
    let h = match h {
        Some(h) if way_to(g, at, h.pos).is_some() => h,
        _ => {   // ... or only by a way round: through another door, round a wall
            return None;
        }
    };
    Some(crate::handleside::ToolSpot { pos: h.pos, yaw: h.yaw, far: h.far.unwrap_or(false) })
}

/// Where a door order's lead works the handle from without standing in the opening.
fn near_stand(g: &crate::game::Game, d: DoorId, side: f64, at: V3) -> Option<crate::handleside::NearStand> {
    let k = g.k.handle;
    let f = crate::guards::frame_of(g, d);
    let hd = handle_at(g, d);
    let r = k.reach - 0.07;
    let ha = f.w - k.handle_in;
    let min = f.w + k.near_body;
    let ref_ = handle_face(g, d, side);
    let a0 = along_of(&f, at);
    let o0 = off_at(g, d, at);
    let l = v3::dist_xz(at, hd);
    if side_at(g, d, at) != side || a0 < f.w / 2.0 || l < 1e-3 { return None; }
    let in_off = m::sqrt(m::max(0.0, r * r - (min - ha) * (min - ha)));   // the furthest off the wall a man `nearBody` past the jamb still has the handle in reach
    let line: V3 = [hd[0] + (at[0] - hd[0]) * r / l, 0.0, hd[2] + (at[2] - hd[2]) * r / l];
    let (line_a, line_off) = (along_of(&f, line), off_at(g, d, line));
    let first = past_jamb(g, d, side, min, in_off, line_a, line_off);
    let second = past_jamb(g, d, side, min, in_off, ha + m::sqrt(m::max(0.0, r * r - o0 * o0)), o0);
    for p in [first, second] {
        if v3::dist_xz(p, hd) > r + 0.01 || v3::dist_xz(p, at) < 0.05 || !stands(g, p, Some(true)) || crate::scene_collision::low_walled(g, ref_, p) || crate::scene_collision::low_walled(g, at, p) { continue; }
        return Some(crate::handleside::NearStand::List(p));
    }
    if a0 >= min && l <= k.near_post && !crate::scene_collision::low_walled(g, ref_, at) { return Some(crate::handleside::NearStand::Post); }
    None
}

/// The last stride of the walk to a tool's place, for whoever is moving him there: true once he is on it.
pub fn tool_step(g: &mut crate::game::Game, gd: GuardId, pos: V3) -> bool {
    let was = g.m.handleside.stepped.get(gd).copied();
    let now = g.time;
    let dt = match was { Some(w) if now > w => m::min(now - w, 0.05), _ => 1.0 / 60.0 };   // its callers tick without a dt of their own
    g.m.handleside.stepped.set(gd, now);
    if v3::dist_xz(g.ch[g.gd[gd].ch].pos, pos) > 0.6 { return false; }
    step_to(g, gd, pos, dt, 1.2, true, None)
}

/// Is a shut leaf to be stacked on from its latch jamb, whichever jamb a man is nearer.
pub fn latch_first(g: &crate::game::Game, d: DoorId, stack: V3) -> bool {
    if !crate::doors::door_is_closed(g, d) || g.dr[d].leaf.off || !stands(g, stack, Some(true)) { return false; }
    let hd = handle_at(g, d);
    !crate::scene_collision::low_walled(g, hd, stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_path_clamps_as_the_language_does() {
        let p: Vec<V3> = vec![[0.0; 3], [1.0; 3], [2.0; 3]];
        assert_eq!(slice_path(&p, 1, 2), vec![[1.0; 3]]);
        assert_eq!(slice_path(&p, 1, 9).len(), 2);
        assert!(slice_path(&p, 2, 1).is_empty());
        assert!(slice_path(&p, 5, 9).is_empty());
    }
}
