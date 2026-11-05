//! src/game/handleside.ts lines 1 to 361
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- seams: the steps the skeleton cannot do yet (notes.md, ## Skeleton) ----

/// `swing.sweeps(x, z, pad)` (doors.ts:559): the closure on DoorSwing has no Rust name.
fn seam_sweeps(s: &crate::doors::DoorSwing, x: f64, z: f64, pad: f64) -> bool { todo!("needs: crate::doors::door_swing_sweeps(&DoorSwing, x, z, pad)") }

// ---- local helpers ----

/// `path.slice(from, to)`, with the language's clamping.
fn slice_path(path: &[V3], from: usize, to: usize) -> Vec<V3> {
    let to = if to > path.len() { path.len() } else { to };
    if from >= to { Vec::new() } else { path[from..to].to_vec() }
}
/// `const ok`, of findFace.
fn ok_at(g: &mut Game, ref_: V3, p: V3, nav: bool) -> bool {
    stands(g, p, Some(nav)) && !walled(g, ref_, p)
}
/// `const face`, of findFace.
fn face_yaw(hd: V3, p: V3) -> f64 {
    m::atan2(hd[0] - p[0], hd[2] - p[2])
}
/// `const al`, of nearStand and the frame's own along-the-leaf reads.
fn along_of(f: &crate::doorway::RelF, p: V3) -> f64 {
    (p[0] - f.hx) * f.ux + (p[2] - f.hz) * f.uz
}
/// `const past`, of nearStand.
fn past_jamb(g: &mut Game, d: DoorId, side: f64, min: f64, in_off: f64, a: f64, off: f64) -> V3 {
    if a >= min { pt_of(g, d, a, off, side) } else { pt_of(g, d, min, m::min(off, in_off), side) }
}
/// `const line`, of gate: his walk through the handle half, bent onto its line from afar, his feet put on it up close. True = the new row is in the table.
fn line_up(g: &mut Game, gd: GuardId, d: DoorId, side: f64, f: crate::handleside::Face, l: Option<crate::handleside::HandlesideModLinedValue>, a_: crate::handleside::Ahead) -> bool {
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let via = match f.via { Some(v) => v, None => return false };
    if let Some(l) = l { if l.door == d && g.time < l.until { return false; } }
    if off_at(g, d, pos) < 0.25 { return false; }
    let k = g.k.handle;
    let f0 = crate::guards::frame_of(g, d);
    let a = (via[0][0] - f0.hx) * f0.ux + (via[0][2] - f0.hz) * f0.uz;
    let off = off_at(g, d, pos);
    let on = pt_of(g, d, a, v3::clamp(off, k.thru_off, k.push_off), side);
    if a_.dist < k.thru_at + 0.4 && flat(pos, on) < k.thru_at && !walled(g, pos, on) {
        let now = g.time;
        g.m.handleside.states.set(gd, crate::handleside::St { door: d, side, spot: on, phase: crate::handleside::StPhase::Thru, t: 0.0, seen: now, way: None, at: None, at_t: None });
        return true;
    }
    if !walled(g, pos, via[0]) && bend(g, gd, d, crate::handleside::BendA { k: a_.k }, via[0], via[1]) { g.stats.handleside.bent += 1; }
    false
}

// ---- the file's functions ----

/// Not under a showcase: its men are choreographed. (handleside.ts live)
pub fn live(g: &mut crate::game::Game) -> bool {
    !g.touring
}

/// A man's place at a door is his only while he keeps coming for it. (handleside.ts fresh)
pub fn fresh(g: &mut crate::game::Game, st: Option<&crate::handleside::St>) -> bool {
    match st { Some(st) => g.time >= st.seen && g.time - st.seen < 0.5, None => false }
}

/// Which face of the closed leaf `p` is on. (handleside.ts sideAt)
pub fn side_at(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 {
    let f = crate::guards::frame_of(g, d);
    if (p[0] - f.hx) * f.nx + (p[2] - f.hz) * f.nz >= 0.0 { 1.0 } else { -1.0 }
}

/// How far `p` stands off the frame's line. (handleside.ts offAt)
pub fn off_at(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 {
    let f = crate::guards::frame_of(g, d);
    m::abs((p[0] - f.hx) * f.nx + (p[2] - f.hz) * f.nz)
}

/// The floor point `along` the leaf from its hinge, `off` it on face `side`. (handleside.ts ptOf)
pub fn pt_of(g: &mut crate::game::Game, d: DoorId, along: f64, off: f64, side: f64) -> V3 {
    let f = crate::guards::frame_of(g, d);
    [f.hx + f.ux * along + f.nx * side * off, 0.0, f.hz + f.uz * along + f.nz * side * off]
}

/// The handle, on the floor. (handleside.ts handleAt)
pub fn handle_at(g: &mut crate::game::Game, d: DoorId) -> V3 {
    let along = g.dr[d].def.width - g.k.handle.handle_in;
    pt_of(g, d, along, 0.0, 1.0)
}

/// Distance on the floor. (handleside.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// A wall, a sill or a pane between two floor points at shin height: the level's statics only. (handleside.ts walled)
pub fn walled(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    let keep = core::mem::take(&mut g.col.dynamic_boxes);
    let r = crate::scene_collision::solid_blocked(g, [a[0], 0.45, a[2]], [b[0], 0.45, b[2]], None);
    g.col.dynamic_boxes = keep;
    r
}

/// Can a man stand at p: nothing of the level inside his body there (and, `nav`, on a cell the walkers' grid calls free). (handleside.ts stands)
pub fn stands(g: &mut crate::game::Game, p: V3, nav: Option<bool>) -> bool {
    let nav = nav.unwrap_or(false);
    if nav && crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return false; }
    let mut q: V3 = [p[0], 0.0, p[2]];
    crate::scene_collision::collide_circle(g, &mut q, 0.3, 0.2, 1.5, None);
    flat(p, q) < 0.02
}

/// Is `p` at the handle of d: on the handle half, within reach, and on the face it opens into clear of the sweep. (handleside.ts atHandle)
pub fn at_handle(g: &mut crate::game::Game, d: DoorId, p: V3, reach: Option<f64>) -> bool {
    let reach = reach.unwrap_or(g.k.handle.reach + 0.05);
    let f = crate::guards::frame_of(g, d);
    let along = along_of(&f, p);
    if along < f.w / 2.0 || flat(p, handle_at(g, d)) > reach { return false; }
    if !crate::doors::doors_pull_face(g, d, p) { return true; }
    let s = crate::doors::doors_swing_of(g, d);
    !seam_sweeps(&s, p[0], p[2], g.k.handle.body)
}

/// A face's places, found once a face. (handleside.ts faceOf)
pub fn face_of(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::handleside::Face {
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

/// The place a man works d's handle from on face `side`, or None where that face has none. (handleside.ts handleSpot)
pub fn handle_spot(g: &mut crate::game::Game, d: DoorId, side: f64) -> Option<crate::handleside::Spot> {
    face_of(g, d, side).stand
}

/// Searches a face's floor for its stand and its way through. (handleside.ts findFace)
pub fn find_face(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::handleside::Face {
    let k = g.k.handle;
    let w = g.dr[d].def.width;
    let hd = handle_at(g, d);
    let ref_ = pt_of(g, d, w - k.handle_in, 0.2, side);
    let mid = pt_of(g, d, w / 2.0, 0.5, side);
    let pull = crate::doors::doors_pull_face(g, d, mid);
    let mut via: Option<[V3; 2]> = None;
    for inn in [k.push_in, k.push_in + 0.08, k.push_in + 0.16] {
        let a = v3::clamp(w - inn, w / 2.0, w);
        let pa = pt_of(g, d, a, k.push_off, side);
        let pb = pt_of(g, d, a, k.push_thru, -side);
        if ok_at(g, ref_, pa, false) && stands(g, pb, None) && !walled(g, pa, pb) { via = Some([pa, pb]); break; }
    }
    let mut stand: Option<crate::handleside::Spot> = None;
    if !pull {
        'search: for inn in [k.stand_in, k.stand_in + 0.08, k.stand_in + 0.16, k.stand_in + 0.24] {
            for off in [k.stand_off, k.stand_off + 0.12] {
                let p = pt_of(g, d, v3::clamp(w - inn, w / 2.0, w), off, side);
                if ok_at(g, ref_, p, false) {
                    stand = Some(crate::handleside::Spot { pos: p, yaw: face_yaw(hd, p), pull, far: None });
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
            let r = flat(p, hd);
            if r <= k.reach {
                let c = r + if a < w { 0.25 } else { 0.0 };   // beside the jamb before in front of the opening
                if !(c >= bc || seam_sweeps(&s, p[0], p[2], k.body + k.slack) || !ok_at(g, ref_, p, true)) {
                    best = Some(p);
                    bc = c;
                }
            }
            off += 0.05;
        }
        a += 0.1;
    }
    if let Some(b) = best {
        return crate::handleside::Face { stand: Some(crate::handleside::Spot { pos: b, yaw: face_yaw(hd, b), pull, far: None }), via };
    }
    // a door in a corner: the nearest floor to its handle that the leaf does not sweep, in front of its handle half: the long reach
    let mut a = w / 2.0;
    while a <= w + 0.4 + 1e-6 {
        let mut off = 0.5;
        while off <= 1.8 + 1e-6 {
            let p = pt_of(g, d, a, off, side);
            let r = flat(p, hd);
            if !(r > k.far_reach || r >= bc) && !(seam_sweeps(&s, p[0], p[2], k.body + k.slack) || !ok_at(g, ref_, p, false)) {
                best = Some(p);
                bc = r;
            }
            off += 0.05;
        }
        a += 0.1;
    }
    let stand = best.map(|b| crate::handleside::Spot { pos: b, yaw: face_yaw(hd, b), pull, far: Some(true) });
    crate::handleside::Face { stand, via }
}

/// The angle under which a leaf stands in a man's way. (handleside.ts ajarOf)
pub fn ajar_of(g: &mut crate::game::Game, d: DoorId) -> f64 {
    let open = crate::doors::doors_swing_of(g, d).open;
    m::min(g.k.handle.ajar, 0.8 * open)
}

/// A leaf that wants a hand from `gd` standing at `from`: in his way, on its hinges, not locked against him, and nobody's already. (handleside.ts wantsHand)
pub fn wants_hand(g: &mut crate::game::Game, gd: GuardId, d: DoorId, from: V3) -> bool {
    if g.gd[gd].team == crate::teams::Team::Civilian { return false; }
    if crate::doors::door_off(g, d) { return false; }
    let latched = crate::doors::door_latched(g, d);
    if !latched {
        let opened = crate::doors::door_angle(g, d) * crate::doors::door_swing(g, d);
        let under = if crate::doors::doors_pull_face(g, d, from) { m::min(g.k.handle.clear, ajar_of(g, d)) } else { ajar_of(g, d) };
        if opened >= under { return false; }
    }
    if !latched && (crate::doors::door_pull_t(g, d) > 0.0 || crate::doors::door_vel(g, d) * crate::doors::door_swing(g, d) > 0.5) { return false; }
    if g.gd[gd].team == crate::teams::Team::Operative {
        let p = g.player;
        // `new Set([...keys, ...codes])` where he holds codes, else his keys: lent by copy, the stub wants `g` too
        let mut keys: JsSet<String> = g.pl[p].keys.clone();
        if g.pl[p].codes.len() > 0 { for c in g.pl[p].codes.keys() { keys.add(c); } }
        if crate::doors::doors_locked_out(g, d, from, Some(&keys)) { return false; }
    }
    true
}

/// The first leaf his walk goes through within `look` metres. (handleside.ts ahead)
pub fn ahead(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::handleside::Ahead> {
    let c = g.gd[gd].ch;
    let look = g.k.handle.look;
    let mut a = g.ch[c].pos;
    let mut len = 0.0;
    let mut k = g.gd[gd].path_i;
    while k < g.gd[gd].path.len() && len < look {
        let b = g.gd[gd].path[k];
        let seg = flat(a, b);
        for i in 0..g.doors.list.len() {
            let d = g.doors.list[i];
            let fc = g.dr[d].frame_centre;
            if m::abs(fc[0] - a[0]) > seg + 2.0 || m::abs(fc[2] - a[2]) > seg + 2.0 { continue; }
            if let Some(x) = crate::guards::crosses_door(g, a, b, d) {
                let dist = len + seg * x.t;
                return if dist <= look { Some(crate::handleside::Ahead { d, k: k as f64, at: x.at, dist }) } else { None };
            }
        }
        len += seg;
        a = b;
        k += 1;
    }
    None
}

/// He lets go of his place at the door. (handleside.ts release)
pub fn release(g: &mut crate::game::Game, gd: GuardId, rest: bool) {
    let door = g.m.handleside.states.get(gd).unwrap().door;   // every caller holds his own row
    g.m.handleside.states.delete(gd);
    if g.m.handleside.hands.get(door).copied() == Some(gd) { g.m.handleside.hands.delete(door); }
    if rest {
        let until = g.time + g.k.handle.rest;
        g.m.handleside.rested.set(gd, crate::handleside::HandlesideModRestedValue { door, until });
    }
}

/// His hand on the leaf from the face it opens into. (handleside.ts pullOpen)
pub fn pull_open(g: &mut crate::game::Game, gd: GuardId, d: DoorId, far: bool) {
    let who = g.ch[g.gd[gd].ch].id;
    let quiet = crate::silent::silent_ease(g, gd);
    crate::doors::doors_pull(g, d, who, Some(quiet));
    g.stats.handleside.pulls += 1;
    if far { g.stats.handleside.far_pulls += 1; }
}

/// His walk through door `d` bent onto the handle half's line; made once. (handleside.ts bend)
pub fn bend(g: &mut crate::game::Game, gd: GuardId, d: DoorId, a: crate::handleside::BendA, a_2: V3, b: V3) -> bool {
    let c = g.ch[g.gd[gd].ch].pos;
    if flat(c, a_2) < 0.3 || off_at(g, d, c) < off_at(g, d, a_2) - 0.05 { return false; }
    let k = a.k as usize;
    let on_it = slice_path(&g.gd[gd].path, g.gd[gd].path_i, k + 1).iter().any(|p| flat(*p, b) < 0.2);
    if on_it { return false; }
    let mut rest = slice_path(&g.gd[gd].path, k, g.gd[gd].path.len());
    while rest.len() > 1 && flat(rest[0], b) < 0.7 { rest.remove(0); }   // the walk's own first step beyond the frame is this one
    let mut path = vec![a_2, b];
    path.extend(rest);
    g.gd[gd].path = path;
    g.gd[gd].path_i = 0;
    true
}

/// He stands still this frame. (handleside.ts halt)
pub fn halt(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let c = g.gd[gd].ch;
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let p = g.ch[c].pos;
    g.gd[gd].last_pos = [p[0], p[1], p[2]];
    g.gd[gd].stuck_t = 0.0;
}

/// The gate followPath and the scripted strafe ask: true = this file has his feet this frame. (handleside.ts handleGate)
pub fn handle_gate(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64, strafe: Option<bool>) -> bool {
    let mine = gate(g, gd, dt, speed, strafe.unwrap_or(false));
    if mine { let now = g.time; g.m.handleside.feet.set(gd, now); }
    mine
}

/// Did this file have his feet this frame or the last. (handleside.ts handleHeld)
pub fn handle_held(g: &mut crate::game::Game, gd: GuardId) -> bool {
    match g.m.handleside.feet.get(gd).copied() { Some(t) => g.time >= t && g.time - t < 0.05, None => false }
}

/// The gate's body: finds him a place at a leaf, or walks and works the one he has. (handleside.ts gate)
pub fn gate(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64, strafe: bool) -> bool {
    use crate::handleside::StPhase;
    if !live(g) { return false; }
    let k = g.k.handle;
    let c = g.gd[gd].ch;
    let mut has = false;
    if let Some(st) = g.m.handleside.states.get(gd).cloned() {
        if fresh(g, Some(&st)) { has = true; } else { release(g, gd, false); }
    }
    if !has {
        if g.gd[gd].path_i >= g.gd[gd].path.len() { return false; }
        let a_ = match ahead(g, gd) { Some(a) => a, None => return false };
        let d = a_.d;
        let pos = g.ch[c].pos;
        let side = side_at(g, d, pos);
        let mine = g.m.handleside.hands.get(d).copied();
        let pull = crate::doors::doors_pull_face(g, d, pos);
        let theirs = match mine {
            Some(m_) if m_ != gd && g.gd[m_].state != crate::game::GuardState::Dead => g.m.handleside.states.get(m_).cloned(),
            _ => None,
        };
        if fresh(g, theirs.as_ref()) && theirs.as_ref().unwrap().door == d {   // a man of his has that leaf in hand: he keeps out of where it is coming round
            if !pull || !wants_hand(g, gd, d, pos) { return false; }
            let s = crate::doors::doors_swing_of(g, d);
            let f = crate::guards::frame_of(g, d);
            let pad = k.body + 0.1;
            if seam_sweeps(&s, pos[0], pos[2], pad) {   // in it already: a step straight back off the door, where there is floor
                let bx = pos[0] + f.nx * side * 0.3;
                let bz = pos[2] + f.nz * side * 0.3;
                if !crate::scene_collision::nav_grid_is_blocked(g, bx, bz) {
                    let v = m::min(speed, 1.3);
                    let vel = [f.nx * side * v, 0.0, f.nz * side * v];
                    let np = [pos[0] + vel[0] * dt, pos[1], pos[2] + vel[2] * dt];
                    g.ch[c].vel = vel;
                    g.ch[c].pos = np;
                    g.gd[gd].speed = v;
                    g.gd[gd].last_pos = [np[0], np[1], np[2]];
                    g.gd[gd].stuck_t = 0.0;
                    g.stats.handleside.held += 1;
                    return true;
                }
                halt(g, gd, dt);
                g.stats.handleside.held += 1;
                return true;
            }
            let wp = g.gd[gd].path[g.gd[gd].path_i];
            let dx = wp[0] - pos[0];
            let dz = wp[2] - pos[2];
            let l = js::or(m::hyp2(dx, dz), 1.0);
            if seam_sweeps(&s, pos[0] + dx / l * 0.25, pos[2] + dz / l * 0.25, pad) {
                halt(g, gd, dt);
                g.stats.handleside.held += 1;
                return true;
            }
            return false;
        }
        if let Some(r) = g.m.handleside.rested.get(gd).copied() { if r.door == d && g.time < r.until { return false; } }
        let f = face_of(g, d, side);
        let l = g.m.handleside.lined.get(gd).copied();
        if !wants_hand(g, gd, d, pos) {   // it stands open: from the face it opens into he still goes through the half its handle left
            let thru = pull
                && !crate::doors::door_off(g, d)
                && !crate::doors::door_latched(g, d)
                && crate::doors::door_angle(g, d) * crate::doors::door_swing(g, d) < k.ajar + 0.1
                && g.gd[gd].team != crate::teams::Team::Civilian
                && line_up(g, gd, d, side, f, l, a_);
            if !thru { return false; }
        } else if off_at(g, d, pos) < 0.25 {   // in the frame itself: he is through it already
            return false;
        } else if !pull {   // PUSH: his walk goes through the handle half
            if !line_up(g, gd, d, side, f, l, a_) { return false; }
        } else {
            let h = match f.stand {
                Some(h) => h,
                None => {
                    g.stats.handleside.no_spot += 1;
                    let until = g.time + k.rest;
                    g.m.handleside.rested.set(gd, crate::handleside::HandlesideModRestedValue { door: d, until });
                    return false;
                }
            };
            let way = match way_to(g, pos, h.pos) {   // the floor's own way there
                Some(w) => w,
                None => {   // no way there, or not yet: round the corner first, and he is asked again in half a second
                    let near = walled(g, pos, h.pos);
                    if !near { g.stats.handleside.no_spot += 1; }
                    let until = g.time + if near { 0.5 } else { k.rest };
                    g.m.handleside.rested.set(gd, crate::handleside::HandlesideModRestedValue { door: d, until });
                    return false;
                }
            };
            let now = g.time;
            g.m.handleside.states.set(gd, crate::handleside::St { door: d, side, spot: h.pos, phase: StPhase::Go, t: 0.0, seen: now, way: Some(way), at: None, at_t: None });
            g.m.handleside.hands.set(d, gd);
        }
        if !g.m.handleside.states.has(gd) { return false; }
    }
    let now = g.time;
    let (d, side, phase) = {
        let st = g.m.handleside.states.get_mut(gd).unwrap();
        st.t += dt;
        st.seen = now;
        (st.door, st.side, st.phase)
    };
    if phase == StPhase::Thru {   // onto the line through the handle half, then straight through
        if speed < 0.05 {   // whoever moves him wants him still this frame
            halt(g, gd, dt);
            g.m.handleside.states.get_mut(gd).unwrap().t -= dt;
            return true;
        }
        let (t, spot) = { let st = g.m.handleside.states.get(gd).unwrap(); (st.t, st.spot) };
        if t < 2.0 && !step_to(g, gd, spot, dt, speed, strafe, Some(true)) { return true; }
        let a_ = ahead(g, gd);
        let via = face_of(g, d, side).via;
        if let (Some(a_), Some(via)) = (a_, via) {
            if a_.d == d {
                let mut rest = slice_path(&g.gd[gd].path, a_.k as usize, g.gd[gd].path.len());
                while rest.len() > 1 && flat(rest[0], via[1]) < 0.7 { rest.remove(0); }
                let mut path = vec![via[1]];
                path.extend(rest);
                g.gd[gd].path = path;
                g.gd[gd].path_i = 0;
                g.stats.handleside.lined += 1;
            }
        }
        let until = g.time + 1.5;
        g.m.handleside.lined.set(gd, crate::handleside::HandlesideModLinedValue { door: d, until });
        g.m.handleside.states.delete(gd);
        return false;
    }
    if phase == StPhase::Go {
        let pos = g.ch[c].pos;
        if !wants_hand(g, gd, d, pos) { release(g, gd,false); return false; }   // somebody opened it meanwhile
        if speed < 0.05 {   // whoever moves him wants him still this frame: he waits where he is
            halt(g, gd, dt);
            g.m.handleside.states.get_mut(gd).unwrap().t -= dt;
            return true;
        }
        let (t, at, at_t, spot) = { let st = g.m.handleside.states.get(gd).unwrap(); (st.t, st.at, st.at_t, st.spot) };
        if at.is_none() || t - at_t.unwrap() > 0.6 {
            let stalled = match at { Some(at) => flat(at, pos) < 0.08, None => false };
            {
                let st = g.m.handleside.states.get_mut(gd).unwrap();
                st.at = Some([pos[0], 0.0, pos[2]]);
                st.at_t = Some(t);
            }
            if stalled || t > k.go_cap || side_at(g, d, pos) != side {   // not getting there: he takes it at the long reach from where he is, if that is no place its leaf comes round through; else he gives its handle up
                let f = crate::guards::frame_of(g, d);
                let along = along_of(&f, pos);
                let hd = handle_at(g, d);
                let close = flat(pos, spot) < 0.3;   // a man of his shoulder to shoulder with him at the jamb: as good as there
                let takes = side_at(g, d, pos) == side && along >= f.w / 2.0 && flat(pos, hd) <= k.far_reach && {
                    let s = crate::doors::doors_swing_of(g, d);
                    !seam_sweeps(&s, pos[0], pos[2], if close { k.body - 0.08 } else { k.body + 0.05 })
                } && {
                    let r = pt_of(g, d, f.w - k.handle_in, 0.2, side);
                    !walled(g, pos, r)
                };
                if !takes {
                    g.stats.handleside.gave_up += 1;
                    release(g, gd,true);
                    return false;
                }
                g.m.handleside.states.get_mut(gd).unwrap().way = Some(Vec::new());
            }
        }
        let (n, first) = {
            let way = g.m.handleside.states.get_mut(gd).unwrap().way.as_mut().unwrap();
            while way.len() > 1 && flat(pos, way[0]) < 0.3 { way.remove(0); }
            (way.len(), way.first().copied())
        };
        if n > 1 { step_to(g, gd, first.unwrap(), dt, speed, strafe, Some(true)); return true; }
        if n > 0 && !step_to(g, gd, spot, dt, speed, strafe, None) { return true; }
        let st = g.m.handleside.states.get_mut(gd).unwrap();
        st.phase = StPhase::Turn;
        st.t = 0.0;
    }
    halt(g, gd, dt);
    let pos = g.ch[c].pos;
    let hd = handle_at(g, d);
    let yaw = m::atan2(hd[0] - pos[0], hd[2] - pos[2]);
    let open = crate::doors::door_angle(g, d) * crate::doors::door_swing(g, d);
    let ajar = ajar_of(g, d);
    let far = flat(pos, hd) > k.reach + 0.05;
    let (phase, t) = { let st = g.m.handleside.states.get(gd).unwrap(); (st.phase, st.t) };
    if phase == StPhase::Turn {
        if !wants_hand(g, gd, d, pos) { release(g, gd,false); return false; }
        if !strafe { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw, 7.0 * dt); }
        if !strafe && t < k.turn && m::abs(v3::wrap_angle(yaw - g.ch[c].body_yaw)) > 0.6 { return true; }
        pull_open(g, gd, d, far);
        let st = g.m.handleside.states.get_mut(gd).unwrap();
        st.phase = StPhase::Wait;
        st.t = 0.0;
        return true;
    }
    // wait: the leaf comes round past him. His hand stays on it while the handle is in his reach and it would not yet carry clear by itself; he walks on once it stands clear.
    if !strafe { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw, 7.0 * dt); }
    let vel = crate::doors::door_vel(g, d);
    if open < m::min(k.clear, ajar) && !(t > 0.3 && m::abs(vel) < 0.4) && t < k.wait {
        let u = crate::doors::door_dir(g, d);
        let (hinge, width) = (g.dr[d].def.hinge, g.dr[d].def.width);
        let hx = hinge[0] + u[0] * (width - k.handle_in);
        let hz = hinge[1] + u[1] * (width - k.handle_in);
        if open + m::abs(vel) / 2.2 < ajar + 0.05 && (far || m::hyp2(hx - pos[0], hz - pos[2]) < k.reach + 0.1) {
            let who = g.ch[c].id;
            let quiet = crate::silent::silent_ease(g, gd);
            crate::doors::doors_pull(g, d, who, Some(quiet));
        }
        return true;
    }
    let a_ = ahead(g, gd);
    let via = face_of(g, d, side).via;
    let clear_now = open >= m::min(k.clear, ajar);
    if let (true, Some(a_), Some(via)) = (clear_now, a_, via) {
        if a_.d == d {   // onto the line through the handle half, as any man goes through one standing open (the leaf is let go of: the next man's)
            let f0 = crate::guards::frame_of(g, d);
            let a = (via[0][0] - f0.hx) * f0.ux + (via[0][2] - f0.hz) * f0.uz;
            if g.m.handleside.hands.get(d).copied() == Some(gd) { g.m.handleside.hands.delete(d); }
            let off = off_at(g, d, pos);
            let spot = pt_of(g, d, a, v3::clamp(off, k.thru_off, k.push_off), side);
            let st = g.m.handleside.states.get_mut(gd).unwrap();
            st.phase = StPhase::Thru;
            st.t = 0.0;
            st.spot = spot;
            return true;
        }
    }
    release(g, gd,!clear_now);
    false
}

/// The walk from `a` to a place at a handle, or None when that is no short way. (handleside.ts wayTo)
pub fn way_to(g: &mut crate::game::Game, a: V3, to: V3) -> Option<Vec<V3>> {
    let d = flat(a, to);
    if d < 0.5 || crate::scene_collision::nav_grid_walkable(g, a, to) { return Some(vec![to]); }
    let mut p = crate::scene_collision::nav_grid_find_path(g, a, to, Some(1500.0), None)?;
    let mut len = 0.0;
    let mut q = a;
    for i in 0..p.len() {
        len += flat(q, p[i]);
        q = p[i];
    }
    if len > d * 2.0 + 1.5 { return None; }
    if flat(p[p.len() - 1], to) < 0.4 { p.pop(); }
    p.push(to);
    Some(p)
}

/// A man's feet taken the last of the way onto `to`: true once he stands on it. (handleside.ts stepTo)
pub fn step_to(g: &mut crate::game::Game, gd: GuardId, to: V3, dt: f64, speed: f64, strafe: bool, pass: Option<bool>) -> bool {
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
    g.ch[c].pos = np;
    g.gd[gd].last_pos = [np[0], np[1], np[2]];
    g.gd[gd].stuck_t = 0.0;
    false
}

/// The seconds a man's body must lean on a leaf from the face it opens into before it is pulled round; None = doors.ts's own beat. (handleside.ts handPress)
pub fn hand_press(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> {
    if !live(g) || g.gd[gd].team == crate::teams::Team::Civilian || g.gd[gd].held.is_some() { return None; }
    if let Some(r) = g.m.handleside.rested.get(gd).copied() { if g.time < r.until { return None; } }
    let st = g.m.handleside.states.get(gd).cloned();
    if fresh(g, st.as_ref()) || g.gd[gd].speed < 0.3 { Some(f64::INFINITY) } else { Some(g.k.handle.press) }
}

/// A door order's lead about to open `d` by hand: the place he works it from, or None when he stays where he is. (handleside.ts toolSpot)
pub fn tool_spot(g: &mut crate::game::Game, d: DoorId, side: f64, at: V3, peek: Option<bool>, order: Option<bool>) -> Option<crate::handleside::ToolSpot> {
    let peek = peek.unwrap_or(false);
    let order = order.unwrap_or(false);
    let k = g.k.handle;
    if !live(g) || at_handle(g, d, at, Some(k.reach - 0.07)) { return None; }
    if order && !crate::doors::doors_pull_face(g, d, at) {   // a door order's lead, whichever cast of doortools.ts gives it
        match near_stand(g, d, side, at) {
            Some(crate::handleside::NearStand::Post) => {
                if !peek { g.stats.handleside.tool_posts += 1; }
                return None;
            }
            Some(crate::handleside::NearStand::List(n)) => {
                let hd = handle_at(g, d);
                if !peek { g.stats.handleside.tool_nears += 1; }
                return Some(crate::handleside::ToolSpot { pos: n, yaw: m::atan2(hd[0] - n[0], hd[2] - n[2]), pull: false, far: false });
            }
            None => {}
        }
    }
    if k.tool_post > 0.0 && !crate::doors::doors_pull_face(g, d, at) {   // beside the latch jamb on the face it swings away from: nothing comes round at him, and the handle is an arm away
        let f = crate::guards::frame_of(g, d);
        let along = along_of(&f, at);
        if along >= f.w && side_at(g, d, at) == side && flat(at, handle_at(g, d)) <= k.tool_post && off_at(g, d, at) <= 0.8 {
            let width = g.dr[d].def.width;
            let r = pt_of(g, d, width - k.handle_in, 0.2, side);
            if !walled(g, r, at) { return None; }
        }
    }
    let h = handle_spot(g, d, side);
    let h = match h {
        Some(h) if way_to(g, at, h.pos).is_some() => h,
        _ => {   // ... or only by a way round: through another door, round a wall
            if !peek { g.stats.handleside.tool_no_spot += 1; }
            return None;
        }
    };
    if !peek { g.stats.handleside.tool_walks += 1; }
    Some(crate::handleside::ToolSpot { pos: h.pos, yaw: h.yaw, pull: h.pull, far: h.far.unwrap_or(false) })
}

/// Where a door order's lead works the handle from without standing in the opening. (handleside.ts nearStand)
pub fn near_stand(g: &mut crate::game::Game, d: DoorId, side: f64, at: V3) -> Option<crate::handleside::NearStand> {
    let k = g.k.handle;
    let f = crate::guards::frame_of(g, d);
    let hd = handle_at(g, d);
    let r = k.reach - 0.07;
    let ha = f.w - k.handle_in;
    let min = f.w + k.near_body;
    let ref_ = pt_of(g, d, ha, 0.2, side);
    let a0 = along_of(&f, at);
    let o0 = off_at(g, d, at);
    let l = flat(at, hd);
    if side_at(g, d, at) != side || a0 < f.w / 2.0 || l < 1e-3 { return None; }
    let in_off = m::sqrt(m::max(0.0, r * r - (min - ha) * (min - ha)));   // the furthest off the wall a man `nearBody` past the jamb still has the handle in reach
    let line: V3 = [hd[0] + (at[0] - hd[0]) * r / l, 0.0, hd[2] + (at[2] - hd[2]) * r / l];
    let (line_a, line_off) = (along_of(&f, line), off_at(g, d, line));
    let first = past_jamb(g, d, side, min, in_off, line_a, line_off);
    let second = past_jamb(g, d, side, min, in_off, ha + m::sqrt(m::max(0.0, r * r - o0 * o0)), o0);
    for p in [first, second] {
        if flat(p, hd) > r + 0.01 || flat(p, at) < 0.05 || !stands(g, p, Some(true)) || walled(g, ref_, p) || walled(g, at, p) { continue; }
        return Some(crate::handleside::NearStand::List(p));
    }
    if a0 >= min && l <= k.near_post && !walled(g, ref_, at) { return Some(crate::handleside::NearStand::Post); }
    None
}

/// The last stride of the walk to a tool's place, for whoever is moving him there: true once he is on it. (handleside.ts toolStep)
pub fn tool_step(g: &mut crate::game::Game, gd: GuardId, pos: V3) -> bool {
    let was = g.m.handleside.stepped.get(gd).copied();
    let now = g.time;
    let dt = match was { Some(w) if now > w => m::min(now - w, 0.05), _ => 1.0 / 60.0 };   // its callers tick without a dt of their own
    g.m.handleside.stepped.set(gd, now);
    if flat(g.ch[g.gd[gd].ch].pos, pos) > 0.6 { return false; }
    step_to(g, gd, pos, dt, 1.2, true, None)
}

/// Is a shut leaf to be stacked on from its latch jamb, whichever jamb a man is nearer. (handleside.ts latchFirst)
pub fn latch_first(g: &mut crate::game::Game, d: DoorId, stack: V3) -> bool {
    if !live(g) || !crate::doors::door_is_closed(g, d) || crate::doors::door_off(g, d) || !stands(g, stack, Some(true)) { return false; }
    let hd = handle_at(g, d);
    !walled(g, hd, stack)
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
