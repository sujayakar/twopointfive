//! The gate a man's walk asks: whether his feet are his own at a leaf this frame. He finds himself a place at it, walks to it, and works it.
use super::*;

/// The man asking, and the frame he asks for.
#[derive(Clone, Copy)]
struct Asks {
    gd: GuardId,
    c: CharId,
    dt: f64,
    speed: f64,
    strafe: bool,
}

/// A leaf his walk goes through, as he comes to it.
#[derive(Clone, Copy)]
struct Comes {
    ahead: Ahead,
    pos: V3,
    side: f64,
    /// It opens toward him.
    pull: bool,
}

/// The gate followPath and the scripted strafe ask: true = this file has his feet this frame.
pub fn handle_gate(g: &mut Game, gd: GuardId, dt: f64, speed: f64, strafe: Option<bool>) -> bool {
    let man = Asks { gd, c: g.gd[gd].ch, dt, speed, strafe: strafe.unwrap_or(false) };
    let mine = gate(g, man);
    if mine { g.gd[gd].handle_gate_t = g.time; }
    mine
}

/// Did this file have his feet this frame or the last.
pub fn handle_gate_stopped(g: &Game, gd: GuardId) -> bool {
    let t = g.gd[gd].handle_gate_t;
    g.time >= t && g.time - t < 0.05
}

/// He keeps the place he has, or finds one, and then does what his place asks of him.
fn gate(g: &mut Game, man: Asks) -> bool {
    if !keeps(g, man.gd) {
        if let Some(stopped) = comes_to(g, man) { return stopped; }
    }
    let now = g.time;
    let st = st(g, man.gd);
    st.t += man.dt;
    st.seen = now;
    match st.phase {
        StPhase::Thru => thru(g, man),
        StPhase::Go => go(g, man).unwrap_or_else(|| turn(g, man)),
        StPhase::Turn => turn(g, man),
        StPhase::Wait => wait(g, man),
    }
}

/// The row of a man with a place at a door.
fn st(g: &mut Game, gd: GuardId) -> &mut St {
    g.m.handleside.states.get_mut(gd).unwrap()
}

/// The door and the face his place is at, with the spot of it.
fn place(g: &Game, gd: GuardId) -> (DoorId, f64, V3) {
    let st = g.m.handleside.states.get(gd).unwrap();
    (st.door, st.side, st.spot)
}

/// Has he a place already; one he stopped coming for is let go of.
fn keeps(g: &mut Game, gd: GuardId) -> bool {
    let has = match g.m.handleside.states.get(gd) { Some(st) => fresh(g, st), None => return false };
    if !has { release(g, gd, false); }
    has
}

/// He comes to the next leaf on his walk: None when he has taken a place at it, else whether his feet are stopped.
fn comes_to(g: &mut Game, man: Asks) -> Option<bool> {
    let gd = man.gd;
    if g.gd[gd].path_i >= g.gd[gd].path.len() { return Some(false); }
    let Some(ahead) = ahead(g, gd) else { return Some(false) };
    let (d, pos) = (ahead.d, g.ch[man.c].pos);
    let at = Comes { ahead, pos, side: side_at(g, d, pos), pull: crate::doors::doors_pull_face(g, d, pos) };
    if another_has(g, gd, d) { return Some(at.pull && wants_hand(g, &crate::seats::here(g), gd, d, pos) && keeps_clear(g, man, at)); }
    if g.m.handleside.rested.get(gd).is_some_and(|r| r.door == d && g.time < r.until) { return Some(false); }
    let face = face_of(g, d, at.side);
    let took = if !wants_hand(g, &crate::seats::here(g), gd, d, pos) {
        at.pull && g.team(gd) != crate::teams::Team::Civilian && half_open(g, d) && line_up(g, gd, at, face)   // from the face it opens into he still goes through the half its handle left
    } else if off_at(g, d, pos) < 0.25 {   // in the frame itself: he is through it already
        false
    } else if !at.pull {
        line_up(g, gd, at, face)
    } else {
        takes_handle(g, gd, at, face)
    };
    if took { None } else { Some(false) }
}

/// Another man has that leaf in hand.
fn another_has(g: &Game, gd: GuardId, d: DoorId) -> bool {
    let Some(o) = g.m.handleside.hands.get(d).copied() else { return false };
    o != gd && !crate::condition::down(g, g.gd[o].ch) && g.m.handleside.states.get(o).is_some_and(|st| fresh(g, st) && st.door == d)
}

/// He keeps out of where a leaf in another's hand is coming round: back off it where there is floor, or no nearer.
fn keeps_clear(g: &mut Game, man: Asks, at: Comes) -> bool {
    let (d, pos) = (at.ahead.d, at.pos);
    let pad = g.k.handle.body + 0.1;
    if swept(g, d, pos, pad) {
        if !steps_back(g, man, at) { halt(g, man.gd, man.dt); }
        return true;
    }
    let wp = g.gd[man.gd].path[g.gd[man.gd].path_i];
    let (dx, dz) = (wp[0] - pos[0], wp[2] - pos[2]);
    let l = js::or(m::hyp2(dx, dz), 1.0);
    let into = swept(g, d, [pos[0] + dx / l * 0.25, pos[1], pos[2] + dz / l * 0.25], pad);
    if into { halt(g, man.gd, man.dt); }
    into
}

/// A step straight back off the door, facing as he is; false where there is no floor for it.
fn steps_back(g: &mut Game, man: Asks, at: Comes) -> bool {
    let f = crate::guards::frame_of(g, at.ahead.d);
    let pos = at.pos;
    if crate::scene_collision::nav_grid_is_blocked(g, pos[0] + f.nx * at.side * 0.3, pos[2] + f.nz * at.side * 0.3) { return false; }
    let v = m::min(man.speed, 1.3);
    let vel = [f.nx * at.side * v, 0.0, f.nz * at.side * v];
    g.ch[man.c].vel = vel;
    g.gd[man.gd].speed = v;
    feet_to(g, man.gd, [pos[0] + vel[0] * man.dt, pos[1], pos[2] + vel[2] * man.dt]);
    true
}

/// It stands open, but not so far that its handle half is as clear as the rest.
fn half_open(g: &Game, d: DoorId) -> bool {
    !g.dr[d].leaf.off && !g.dr[d].leaf.latched && opened(g, d) < g.k.handle.ajar + 0.1
}

/// His walk through the handle half: bent onto its line from afar, his feet put on it up close. True = he has a place on it.
fn line_up(g: &mut Game, gd: GuardId, at: Comes, face: Face) -> bool {
    let (d, pos) = (at.ahead.d, at.pos);
    let Some(via) = face.via else { return false };
    if g.m.handleside.lined.get(gd).is_some_and(|l| l.door == d && g.time < l.until) { return false; }
    if off_at(g, d, pos) < 0.25 { return false; }
    let on = on_line(g, d, at.side, via, pos);
    let near = g.k.handle.thru_at;
    if at.ahead.dist < near + 0.4 && flat(pos, on) < near && !walled(g, pos, on) {
        let seen = g.time;
        g.m.handleside.states.set(gd, St { door: d, side: at.side, spot: on, phase: StPhase::Thru, t: 0.0, seen, way: None, at: None, at_t: None });
        return true;
    }
    if !walled(g, pos, via[0]) { bend(g, gd, at.ahead, via); }
    false
}

/// His walk through door `d` bent onto the handle half's line; made once.
fn bend(g: &mut Game, gd: GuardId, a: Ahead, via: [V3; 2]) {
    let pos = g.ch[g.gd[gd].ch].pos;
    if flat(pos, via[0]) < 0.3 || off_at(g, a.d, pos) < off_at(g, a.d, via[0]) - 0.05 { return; }
    let on_it = slice_path(&g.gd[gd].path, g.gd[gd].path_i, a.k + 1).iter().any(|p| flat(*p, via[1]) < 0.2);
    if !on_it { walks_by(g, gd, a.k, &via); }
}

/// He takes a leaf that opens toward him by its handle: the floor's own way to where a man works it from. False, and he rests from it, where there is none.
fn takes_handle(g: &mut Game, gd: GuardId, at: Comes, face: Face) -> bool {
    let rest = g.k.handle.rest;
    let Some(h) = face.stand else { rests(g, gd, at.ahead.d, rest); return false };
    let Some(way) = way_to(g, at.pos, h.pos) else {   // no way there, or not yet: round the corner first, and he is asked again in half a second
        let near = walled(g, at.pos, h.pos);
        rests(g, gd, at.ahead.d, if near { 0.5 } else { rest });
        return false;
    };
    let seen = g.time;
    g.m.handleside.states.set(gd, St { door: at.ahead.d, side: at.side, spot: h.pos, phase: StPhase::Go, t: 0.0, seen, way: Some(way), at: None, at_t: None });
    g.m.handleside.hands.set(at.ahead.d, gd);
    true
}

/// THRU: onto the line through the handle half, then straight through.
fn thru(g: &mut Game, man: Asks) -> bool {
    let gd = man.gd;
    if held_still(g, man) { return true; }
    let (d, side, spot) = place(g, gd);
    if st(g, gd).t < 2.0 && !step_to(g, gd, spot, man.dt, man.speed, man.strafe, Some(true)) { return true; }
    let a = ahead(g, gd);
    let via = face_of(g, d, side).via;
    if let (Some(a), Some(via)) = (a, via) {
        if a.d == d { walks_by(g, gd, a.k, &via[1..]); }
    }
    let until = g.time + 1.5;
    g.m.handleside.lined.set(gd, crate::handleside::HandlesideModLinedValue { door: d, until });
    g.m.handleside.states.delete(gd);
    false
}

/// GO: to the handle. None once he is at it.
fn go(g: &mut Game, man: Asks) -> Option<bool> {
    let gd = man.gd;
    let (d, _, spot) = place(g, gd);
    let pos = g.ch[man.c].pos;
    if !wants_hand(g, &crate::seats::here(g), gd, d, pos) { release(g, gd, false); return Some(false); }   // somebody opened it meanwhile
    if held_still(g, man) { return Some(true); }
    if !gets_on(g, man) {   // not getting there: he takes it at the long reach from where he is, or else he gives its handle up
        if !reaches_from(g, man) { release(g, gd, true); return Some(false); }
        st(g, gd).way = Some(Vec::new());
    }
    let way = st(g, gd).way.as_mut().unwrap();
    while way.len() > 1 && flat(pos, way[0]) < 0.3 { way.remove(0); }
    let (left, next) = (way.len(), way.first().copied());
    if left > 1 { step_to(g, gd, next.unwrap(), man.dt, man.speed, man.strafe, Some(true)); return Some(true); }
    if left > 0 && !step_to(g, gd, spot, man.dt, man.speed, man.strafe, None) { return Some(true); }
    next_phase(g, gd, StPhase::Turn);
    None
}

/// Is he getting there, on his own face of the door: where he is, is marked every so often and held against the last mark.
fn gets_on(g: &mut Game, man: Asks) -> bool {
    let (d, side, _) = place(g, man.gd);
    let pos = g.ch[man.c].pos;
    let st = st(g, man.gd);
    let (t, was) = (st.t, st.at);
    if was.is_some() && t - st.at_t.unwrap() <= 0.6 { return true; }
    st.at = Some([pos[0], 0.0, pos[2]]);
    st.at_t = Some(t);
    let stalled = was.is_some_and(|at| flat(at, pos) < 0.08);
    !(stalled || t > g.k.handle.go_cap || side_at(g, d, pos) != side)
}

/// The handle is at his long reach from where he stands, and that is no place its leaf comes round through.
fn reaches_from(g: &Game, man: Asks) -> bool {
    let (d, side, spot) = place(g, man.gd);
    let pos = g.ch[man.c].pos;
    let k = g.k.handle;
    let f = crate::guards::frame_of(g, d);
    let close = flat(pos, spot) < 0.3;   // a man of his shoulder to shoulder with him at the jamb: as good as there
    let body = if close { k.body - 0.08 } else { k.body + 0.05 };
    if side_at(g, d, pos) != side || along_of(&f, pos) < f.w / 2.0 || flat(pos, handle_at(g, d)) > k.far_reach || swept(g, d, pos, body) { return false; }
    let r = handle_face(g, d, side);
    !walled(g, pos, r)
}

/// TURN: he faces the handle, and his hand goes on it.
fn turn(g: &mut Game, man: Asks) -> bool {
    let gd = man.gd;
    let (d, _, _) = place(g, gd);
    halt(g, gd, man.dt);
    let pos = g.ch[man.c].pos;
    if !wants_hand(g, &crate::seats::here(g), gd, d, pos) { release(g, gd, false); return false; }
    let off = faces(g, man, d);
    if !man.strafe && st(g, gd).t < g.k.handle.turn && off > 0.6 { return true; }
    pull_open(g, gd, d);
    next_phase(g, gd, StPhase::Wait);
    true
}

/// WAIT: the leaf comes round past him, and he walks on once it stands clear, onto the line through the handle half as any man goes through one standing open.
fn wait(g: &mut Game, man: Asks) -> bool {
    let gd = man.gd;
    let (d, side, _) = place(g, gd);
    halt(g, gd, man.dt);
    faces(g, man, d);
    let k = g.k.handle;
    let (pos, t) = (g.ch[man.c].pos, st(g, gd).t);
    let clear = opened(g, d) >= m::min(k.clear, ajar_of(g, d));
    let stopped = t > 0.3 && m::abs(g.dr[d].leaf.vel) < 0.4;
    if !clear && !stopped && t < k.wait {
        if hand_stays(g, d, pos) { pull_open(g, gd, d); }
        return true;
    }
    let a = ahead(g, gd);
    let via = face_of(g, d, side).via;
    if let (true, Some(a), Some(via)) = (clear, a, via) {
        if a.d == d {   // (the leaf is let go of: the next man's)
            if g.m.handleside.hands.get(d).copied() == Some(gd) { g.m.handleside.hands.delete(d); }
            let spot = on_line(g, d, side, via, pos);
            next_phase(g, gd, StPhase::Thru);
            st(g, gd).spot = spot;
            return true;
        }
    }
    release(g, gd, !clear);
    false
}

/// His hand stays on a leaf that would not yet carry clear by itself, while its handle is in his reach, or where he works it from further off than that.
fn hand_stays(g: &Game, d: DoorId, pos: V3) -> bool {
    let k = g.k.handle;
    let far = flat(pos, handle_at(g, d)) > k.reach + 0.05;
    let (u, hinge, along) = (crate::doors::door_dir(g, d), g.dr[d].def.hinge, g.dr[d].def.width - k.handle_in);
    let (hx, hz) = (hinge[0] + u[0] * along, hinge[1] + u[1] * along);
    opened(g, d) + m::abs(g.dr[d].leaf.vel) / 2.2 < ajar_of(g, d) + 0.05 && (far || m::hyp2(hx - pos[0], hz - pos[2]) < k.reach + 0.1)
}

/// He turns to the handle, unless whoever moves him has his facing; how far off it he still is.
fn faces(g: &mut Game, man: Asks, d: DoorId) -> f64 {
    let yaw = face_yaw(handle_at(g, d), g.ch[man.c].pos);
    if !man.strafe { g.ch[man.c].body_yaw = v3::approach_angle(g.ch[man.c].body_yaw, yaw, 7.0 * man.dt); }
    m::abs(v3::wrap_angle(yaw - g.ch[man.c].body_yaw))
}

/// Whoever moves him wants him still this frame: he waits where he is, his place's clock with him.
fn held_still(g: &mut Game, man: Asks) -> bool {
    if man.speed >= 0.05 { return false; }
    halt(g, man.gd, man.dt);
    st(g, man.gd).t -= man.dt;
    true
}

/// On to the next thing his place asks of him.
fn next_phase(g: &mut Game, gd: GuardId, phase: StPhase) {
    let st = st(g, gd);
    st.phase = phase;
    st.t = 0.0;
}

/// The spot on the line through the handle half that a man at `pos` steps onto.
fn on_line(g: &Game, d: DoorId, side: f64, via: [V3; 2], pos: V3) -> V3 {
    let along = along_of(&crate::guards::frame_of(g, d), via[0]);
    pt_of(g, d, along, v3::clamp(off_at(g, d, pos), g.k.handle.thru_off, g.k.handle.push_off), side)
}

/// The rest of his walk from its `k`th point, by way of `through`.
fn walks_by(g: &mut Game, gd: GuardId, k: usize, through: &[V3]) {
    let last = through[through.len() - 1];
    let mut rest = slice_path(&g.gd[gd].path, k, g.gd[gd].path.len());
    while rest.len() > 1 && flat(rest[0], last) < 0.7 { rest.remove(0); }   // the walk's own first step beyond the frame is this one
    let mut path = through.to_vec();
    path.extend(rest);
    crate::game::guard_set_path(g, gd, path);
    g.gd[gd].path_i = 0;
}

/// The first leaf his walk goes through within `look` metres.
fn ahead(g: &Game, gd: GuardId) -> Option<Ahead> {
    let look = g.k.handle.look;
    let mut a = g.ch[g.gd[gd].ch].pos;
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
                return if dist <= look { Some(Ahead { d, k, dist }) } else { None };
            }
        }
        len += seg;
        a = b;
        k += 1;
    }
    None
}

/// He lets go of his place at the door.
fn release(g: &mut Game, gd: GuardId, rest: bool) {
    let door = st(g, gd).door;
    g.m.handleside.states.delete(gd);
    if g.m.handleside.hands.get(door).copied() == Some(gd) { g.m.handleside.hands.delete(door); }
    if rest {
        let secs = g.k.handle.rest;
        rests(g, gd, door, secs);
    }
}

/// He leaves that door alone for so many seconds.
fn rests(g: &mut Game, gd: GuardId, door: DoorId, secs: f64) {
    let until = g.time + secs;
    g.m.handleside.rested.set(gd, crate::handleside::HandlesideModRestedValue { door, until });
}

/// His hand on the leaf from the face it opens into.
fn pull_open(g: &mut Game, gd: GuardId, d: DoorId) {
    let who = g.ch[g.gd[gd].ch].id;
    let quiet = crate::silent::silent_ease(g, gd);
    crate::doors::doors_pull(g, d, who, Some(quiet));
}

/// He stands still this frame.
fn halt(g: &mut Game, gd: GuardId, dt: f64) {
    let c = g.gd[gd].ch;
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let p = g.ch[c].pos;
    g.gd[gd].last_pos = [p[0], p[1], p[2]];
    g.gd[gd].stuck_t = 0.0;
}
