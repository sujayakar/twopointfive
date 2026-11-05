//! src/game/squad.ts lines 1 to 449
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// The field of a table of the plan's lines (or of BODY_LINES and its kin) that a say key names. (squad.ts `X[key]`)
macro_rules! line_of {
    ($t:expr, $k:expr) => {
        match $k {
            RoomClearSayKey::Stack1 => $t.stack1,
            RoomClearSayKey::Stack2 => $t.stack2,
            RoomClearSayKey::Go2 => $t.go2,
            RoomClearSayKey::In1 => $t.in1,
            RoomClearSayKey::Side2 => $t.side2,
            RoomClearSayKey::Side1 => $t.side1,
            RoomClearSayKey::Push1 => $t.push1,
            RoomClearSayKey::Under2 => $t.under2,
            RoomClearSayKey::Nothing2 => $t.nothing2,
            RoomClearSayKey::Corner1 => $t.corner1,
            RoomClearSayKey::Empty2 => $t.empty2,
            RoomClearSayKey::Radio1 => $t.radio1,
        }
    };
}

// ---- seams: each step the skeleton still blocks, with the TypeScript's own arguments (notes.md, ## Skeleton)

/// g.audio.play(name, pos, vol, { rate }) (squad.ts:277, 329): `opts` is a map of Opaque, which cannot hold the rate.
fn seam_audio_play(g: &mut Game, name: &str, pos: V3, vol: f64, rate: f64) { todo!("needs: g.audio.play opts able to carry a rate") }
/// gd.flashlight.enabled = on (squad.ts:126): LightId has no table and no accessor for `enabled`.
fn seam_light_set_enabled(g: &mut Game, l: LightId, on: bool) { g.tab.light[l].enabled = on; } // (skeleton 4 has the row)

// ---- orders

/// A new waypoint list for a man, and orders: the goal is always a fresh copy, so always a new leg. (squad.ts:247, with OrderPatch's null where the public route's patch has none)
fn route_with(g: &mut Game, gd: GuardId, wps: &mut Vec<V3>, pts: &[V3], patch: OrderPatch) {
    wps.clear();
    for p in pts { wps.push(v3::copy(*p)); }
    let first = wps.first().copied();
    order(g, gd, OrderPatch { goal: Some(first), held: false, ..patch });
}

/// route() for a drill's own man: `second` picks #2 and his list. The list lives in the row, so it is lifted out for the call.
fn route_man(g: &mut Game, id: RoomClearId, second: bool, pts: &[V3], patch: OrderPatch) {
    let row = &mut g.tab.squad_room_clear[id];
    let gd = if second { row.b.unwrap() } else { row.a };
    let mut wps = core::mem::take(if second { &mut row.wp_b } else { &mut row.wp_a });
    route_with(g, gd, &mut wps, pts, patch);
    let row = &mut g.tab.squad_room_clear[id];
    if second { row.wp_b = wps; } else { row.wp_a = wps; }
}

/// advance() for a drill's own man.
fn advance_man(g: &mut Game, id: RoomClearId, second: bool) -> bool {
    let row = &mut g.tab.squad_room_clear[id];
    let gd = if second { row.b.unwrap() } else { row.a };
    let mut wps = core::mem::take(if second { &mut row.wp_b } else { &mut row.wp_a });
    let planted = room_clear_advance(g, id, gd, &mut wps);
    let row = &mut g.tab.squad_room_clear[id];
    if second { row.wp_b = wps; } else { row.wp_a = wps; }
    planted
}

/// ALIASING (squad.ts:216, 258, 288): the TypeScript's script holds the drill's own aim vector and the sweep moves it in place. Here the script holds a copy, so
/// each move of the drill's point is written through to a script still holding the point it had.
fn aim_moved(g: &mut Game, gd: GuardId, was: V3, now: V3) {
    if let Some(s) = g.gd[gd].script.as_mut() {
        if s.aim_at == Some(was) { s.aim_at = Some(now); }
    }
}

/// swA.start(aimA, pts, t, dur) / swB.start(aimB, …).
fn sweep_begin(g: &mut Game, id: RoomClearId, second: bool, pts: &[V3], t: f64, dur: f64) {
    let row = &mut g.tab.squad_room_clear[id];
    if second { let aim = row.aim_b; sweep_start(&mut row.sw_b, aim, pts, t, dur); }
    else { let aim = row.aim_a; sweep_start(&mut row.sw_a, aim, pts, t, dur); }
}

/// The rest of a man's entry sweep, once his centre check and corner are done (RoomClear.tick: `if (restA && !swA.active) …`).
fn sweep_rest(g: &mut Game, id: RoomClearId, second: bool, t: f64) {
    let row = &mut g.tab.squad_room_clear[id];
    let (rest, active) = if second { (&mut row.rest_b, row.sw_b.active) } else { (&mut row.rest_a, row.sw_a.active) };
    if active || rest.is_none() { return; }
    let list = rest.take().unwrap();
    sweep_begin(g, id, second, &list, t, rest_secs(&list));
}

/// The public route's patch as an order's: it cannot say null, so a None is "not in the patch".
fn patch_of(p: RoomClearRoutePatch) -> OrderPatch {
    OrderPatch {
        goal: p.goal.map(Some), speed: p.speed.map(Some), face: p.face.map(Some), strafe: p.strafe.map(Some), aim_at: p.aim_at.map(Some), stance: p.stance.map(Some),
        stack_side: p.stack_side.map(Some), crouch: p.crouch.map(Some), lean: p.lean.map(Some), upper: p.upper.map(Some), light: p.light.map(Some), arrived: p.arrived.map(Some),
        hot: p.hot.map(Some), flinch: p.flinch.map(Some), direct: p.direct.map(Some), smoke: p.smoke.map(Some), over: p.over.map(Some), held: false,
    }
}

fn on_his_script(g: &mut Game, gd: GuardId) -> bool {
    g.guards.contains(&gd) && g.gd[gd].script.is_some() && g.gd[gd].state != crate::game::GuardState::Dead
}

// ---- the runner

/// The flat distance between two points. (squad.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// Side-step / back-pedal along the nav path, the body left on `face`; true once the path is consumed. (squad.ts strafeAlongPath)
pub fn strafe_along_path(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> bool {
    let c = g.gd[gd].ch;
    if crate::handleside::handle_gate(g, gd, dt, speed, Some(true)) { return false; }
    let speed = match crate::doorway::doorway_gate(g, gd, dt, speed) { Some(sp) => sp, None => return false };
    while g.gd[gd].path_i < g.gd[gd].path.len() {
        let (i, n) = (g.gd[gd].path_i, g.gd[gd].path.len());
        let (wp, p) = (g.gd[gd].path[i], g.ch[c].pos);
        let (dx, dz) = (wp[0] - p[0], wp[2] - p[2]);
        let d = m::hyp2(dx, dz);
        if d < (if i == n - 1 { 0.25 } else { 0.45 }) { g.gd[gd].path_i += 1; continue; }
        let was = g.gd[gd].speed;
        g.gd[gd].speed = v3::damp(was, speed * v3::clamp((d + (n - 1 - i) as f64) / 0.8, 0.35, 1.0), 4.0, dt);
        crate::guards::team_lean(g, gd, was, dt);
        let sp = g.gd[gd].speed;
        let vel = [dx / d * sp, 0.0, dz / d * sp];
        g.ch[c].vel = vel;
        g.ch[c].pos = v3::mad(g.ch[c].pos, vel, dt);
        return false;
    }
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    true
}

/// One frame of a man under a script: moved to his goal, gun and eyes on his aim, the animator fed, `arrived` written back. (squad.ts runGuardScript)
pub fn run_guard_script(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let s = g.gd[gd].script.unwrap();
    let c = g.gd[gd].ch;
    let (x0, z0) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    let ordered = match s.speed { Some(x) => x, None => g.tune.guard_walk };
    let speed = crate::guards::team_pace(g, gd, ordered);
    let legs = crate::guards::team_legs(g, gd, speed);
    let mut fight_face = match s.over { Some(o) if o.walk.unwrap_or(false) => o.face, _ => None };
    if fight_face.is_some() {
        let (t, p) = (g.time, g.ch[c].pos);
        match g.m.squad.walk_watch.get(gd).copied() {
            Some(w) if !(t < w.t || t - w.t > 3.0 || m::hyp2(p[0] - w.x, p[2] - w.z) > 0.25) => { if t - w.t > 1.2 { fight_face = None; } }
            _ => { g.m.squad.walk_watch.set(gd, SquadModWalkWatchValue { x: p[0], z: p[2], t }); }
        }
    }
    let strafing = fight_face.is_some() || (s.strafe.unwrap_or(false) && s.face.is_some());
    let (mut move_speed, mut arrived, mut settling) = (0.0, true, true);
    let mut far_leg: Option<f64> = None;
    if let Some(goal) = s.goal {
        let p = g.ch[c].pos;
        let (dx, dz) = (goal[0] - p[0], goal[2] - p[2]);
        let planar = m::hyp2(dx, dz);
        let mut home = planar <= SETTLE && !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [goal[0], 0.45, goal[2]], None);
        if !home {
            crate::guards::go_to(g, gd, goal, None);
            if g.gd[gd].path_i >= g.gd[gd].path.len() { home = true; }
            else {
                let done = if strafing { strafe_along_path(g, gd, dt, legs) } else { crate::guards::follow_path(g, gd, dt, speed) };
                move_speed = g.gd[gd].speed;
                home = done && g.gd[gd].path_i >= g.gd[gd].path.len();
            }
        }
        if home {
            let mut w = match g.m.squad.settle_watch.get(gd).copied() {
                Some(w) if w.goal == goal => w,
                _ => SquadModSettleWatchValue { goal, best: planar, t: 0.0 },
            };
            if planar < w.best - 0.01 { w.best = planar; w.t = 0.0; }
            else { w.t += dt; if w.t > 3.6 { w.best = planar; w.t = 0.0; } }
            g.m.squad.settle_watch.set(gd, w);
            if planar > 0.04 && w.t < 0.6 {
                let was = g.gd[gd].speed;
                g.gd[gd].speed = v3::damp(was, m::min(legs, m::max(0.35, planar * 2.2)), 6.0, dt);
                crate::guards::team_lean(g, gd, was, dt);
                let sp = g.gd[gd].speed;
                let step = m::min(planar, sp * dt);
                g.ch[c].pos[0] += dx / planar * step;
                g.ch[c].pos[2] += dz / planar * step;
                g.ch[c].vel = [dx / planar * sp, 0.0, dz / planar * sp];
                move_speed = sp;
                if planar > FACE_WITHIN && !strafing { far_leg = Some(m::atan2(dx, dz)); }
            } else {
                g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
                g.ch[c].vel = [0.0, 0.0, 0.0];
            }
            arrived = planar <= 0.3;
        } else { arrived = false; settling = false; }
        if !settling && strafing {
            let to = fight_face.or(s.face).unwrap();
            g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, to, 6.0 * dt);
        }
    } else {
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
    }
    let over_face = s.over.and_then(|o| o.face);
    if let Some(leg) = far_leg { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, leg, 6.0 * dt); }
    else if settling && over_face.is_some() { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, over_face.unwrap(), 5.0 * dt); }
    else if settling && s.face.is_some() { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, s.face.unwrap(), 5.0 * dt); }
    {
        let ch = &mut g.ch[c];
        ch.anim.reverse = move_speed > 0.2 && ch.vel[0] * m::sin(ch.body_yaw) + ch.vel[2] * m::cos(ch.body_yaw) < -0.3 * move_speed;
    }
    let aim = match s.over { Some(o) => Some(o.aim_at), None => s.aim_at };
    let aim0 = g.ch[c].aim_yaw;
    if let Some(at) = aim {
        let to = v3::sub(at, g.ch[c].pos);
        let yaw = m::atan2(to[0], to[2]);
        let aim_yaw = if s.over.is_some() { yaw } else { crate::guards::team_turn(g, gd, yaw, dt) };
        let ch = &mut g.ch[c];
        ch.aim_yaw = aim_yaw;
        ch.aim_pitch = v3::damp(ch.aim_pitch, m::atan2(at[1] - (1.45 - 0.45 * ch.anim.crouch), m::hyp2(to[0], to[2])), 6.0, dt);
    } else {
        let ch = &mut g.ch[c];
        ch.aim_yaw = v3::approach_angle(ch.aim_yaw, ch.body_yaw, 4.0 * dt);
        ch.aim_pitch = v3::damp(ch.aim_pitch, -0.1, 3.0, dt);
    }
    {
        let ch = &mut g.ch[c];
        let off = v3::wrap_angle(ch.aim_yaw - ch.body_yaw);
        if m::abs(off) > 1.5 { ch.aim_yaw = ch.body_yaw + m::sign(off) * 1.5; }
    }
    let swing = g.k.pace.swing;
    if s.over.is_some() && swing > 0.0 && crate::guards::team_paced(g, gd) {
        let ch = &mut g.ch[c];
        let r0 = v3::clamp(v3::wrap_angle(aim0 - ch.body_yaw), -1.5, 1.5);
        let r1 = v3::wrap_angle(ch.aim_yaw - ch.body_yaw);
        let step = swing * dt;
        if m::abs(r1 - r0) > step { ch.aim_yaw = ch.body_yaw + r0 + m::sign(r1 - r0) * step; }
    }
    let (yaw, pitch) = (g.ch[c].aim_yaw, g.ch[c].aim_pitch);
    g.gd[gd].beam_dir = v3::normalize([m::sin(yaw) * m::cos(pitch), m::sin(pitch), m::cos(yaw) * m::cos(pitch)]);
    let (mut p, radius) = (g.ch[c].pos, g.ch[c].radius);
    crate::scene_collision::collide_circle(g, &mut p, radius, 0.2, 1.5, None);
    g.ch[c].pos = p;
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead { continue; }
        let oc = g.gd[o].ch;
        let mut d = v3::sub(g.ch[c].pos, g.ch[oc].pos);
        d[1] = 0.0;
        let l = v3::len(d);
        if l < 0.6 && l > 1e-4 { g.ch[c].pos = v3::mad(g.ch[c].pos, d, (0.6 - l) / l * 0.5); }
    }
    if crate::noshove::no_shove_keep(g, gd, x0, z0, dt) { move_speed = 0.0; }
    let (upper, stance) = if s.over.is_some() { (crate::character::UpperMode::Aim, crate::character::Stance::None) }
        else { (s.upper.unwrap_or(crate::character::UpperMode::Aim), s.stance.unwrap_or(crate::character::Stance::None)) };
    let up = upper == crate::character::UpperMode::Aim || stance == crate::character::Stance::HighReady || stance == crate::character::Stance::Stack;
    let line = if up && !s.hot.unwrap_or(false) { Some(match aim { Some(at) => at, None => crate::guards::aim_ahead(g, c) }) } else { None };
    crate::guards::muzzle_check(g, gd, line, dt, None);
    let gun = if upper == crate::character::UpperMode::Aim { Some(crate::gunwall::GunUp::Aim) } else if up { Some(crate::gunwall::GunUp::Port) } else { None };
    crate::gunwall::gun_wall_check(g, gd, gun, aim, dt, Some(s.over.is_some()));
    let low = g.gd[gd].muzzle_down || crate::teampace::team_sprinting(g, gd);
    {
        let an = &mut g.ch[c].anim;
        an.speed = move_speed;
        an.upper = if low { crate::character::UpperMode::None } else { upper };
        an.crouch_target = if s.crouch.unwrap_or(false) { 1.0 } else { 0.0 };
        an.lean = s.lean.unwrap_or(0.0);
        an.stance = if low { crate::character::Stance::LowReady } else { stance };
        an.stack_side = s.stack_side.unwrap_or(1.0);
        an.look_yaw_extra = 0.0;
    }
    crate::gunwall::gun_wall_pose(g, c);
    crate::character::character_update(g, c, dt);
    crate::guards::attach_flashlight(g, gd, None);
    if s.light == Some(false) && !crate::guards::light_ruled(g, gd) { let l = g.gd[gd].flashlight; seam_light_set_enabled(g, l, false); }
    let prev_phase = g.gd[gd].step_phase;
    let phase = g.ch[c].anim.phase;
    g.gd[gd].step_phase = phase;
    if move_speed > 0.4 && ((prev_phase < 0.5) != (phase < 0.5)) {
        let p = g.ch[c].pos;
        let base = crate::teampace::team_tread(g, gd, move_speed, 0.45);
        crate::floor::tread(g, p, base, false);
    }
    crate::teampace::team_ran(g, gd);
    if let Some(sc) = g.gd[gd].script.as_mut() { sc.arrived = Some(arrived); }
}

/// Release a scripted guard back to the AI in a sane state; a man who died under a script is only unhooked. (squad.ts endGuardScript)
pub fn end_guard_script(g: &mut crate::game::Game, gd: GuardId, state: Option<crate::squad::EndGuardScriptState>) {
    let state = state.unwrap_or(EndGuardScriptState::Patrol);
    g.gd[gd].script = None;
    if g.gd[gd].state == crate::game::GuardState::Dead { return; }
    let c = g.gd[gd].ch;
    g.ch[c].anim.stance = crate::character::Stance::None;
    g.gd[gd].muzzle_down = false;
    let an = &mut g.ch[c].anim;
    an.crouch_target = 0.0; an.lean = 0.0; an.reverse = false;
    let man = &mut g.gd[gd];
    man.state = match state {
        EndGuardScriptState::Patrol => crate::game::GuardState::Patrol,
        EndGuardScriptState::Suspicious => crate::game::GuardState::Suspicious,
        EndGuardScriptState::Alert => crate::game::GuardState::Alert,
        EndGuardScriptState::Search => crate::game::GuardState::Search,
    };
    man.path = Vec::new();
    man.path_goal = None;
    if state == EndGuardScriptState::Patrol {
        man.awareness = 0.0; man.react_t = 0.0; man.search_t = 0.0; man.last_known = None; man.body_duty = None;
    }
}

/// Give a scripted guard new orders; a new goal drops his path, stuck timer and settle watch. A goal is a new leg even at the same spot, unless `held`
/// says the caller keeps it and moves it (then by value). Some(None) is null; None is not in the patch. (squad.ts order)
pub fn order(g: &mut crate::game::Game, gd: GuardId, patch: crate::squad::OrderPatch) {
    if g.gd[gd].script.is_none() { g.gd[gd].script = Some(crate::game::GuardScript::zeroed()); }
    if let Some(goal) = patch.goal {
        let had = g.gd[gd].script.as_ref().unwrap().goal;
        let new_leg = match goal { None => had.is_some(), Some(_) => !patch.held || goal != had };
        if new_leg {
            let man = &mut g.gd[gd];
            man.path = Vec::new(); man.path_goal = None; man.stuck_t = 0.0;
            if let Some(s) = man.script.as_mut() { s.arrived = Some(false); }
            g.m.squad.settle_watch.delete(gd);
        }
    }
    let s = g.gd[gd].script.as_mut().unwrap();
    if let Some(x) = patch.goal { s.goal = x; }
    if let Some(x) = patch.speed { s.speed = x; }
    if let Some(x) = patch.face { s.face = x; }
    if let Some(x) = patch.strafe { s.strafe = x; }
    if let Some(x) = patch.aim_at { s.aim_at = x; }
    if let Some(x) = patch.stance { s.stance = x; }
    if let Some(x) = patch.stack_side { s.stack_side = x; }
    if let Some(x) = patch.crouch { s.crouch = x; }
    if let Some(x) = patch.lean { s.lean = x; }
    if let Some(x) = patch.upper { s.upper = x; }
    if let Some(x) = patch.light { s.light = x; }
    if let Some(x) = patch.arrived { s.arrived = x; }
    if let Some(x) = patch.hot { s.hot = x; }
    if let Some(x) = patch.flinch { s.flinch = x; }
    if let Some(x) = patch.direct { s.direct = x; }
    if let Some(x) = patch.smoke { s.smoke = x; }
    if let Some(x) = patch.over { s.over = x; }
}

/// Begin a muzzle sweep from wherever the point was. (squad.ts Sweep.start)
pub fn sweep_start(this: &mut crate::squad::Sweep, from: V3, pts: &[V3], t0: f64, dur: f64) {
    let mut all = Vec::with_capacity(pts.len() + 1);
    all.push(v3::copy(from));
    for p in pts { all.push(v3::copy(*p)); }
    this.pts = all;
    this.t0 = t0;
    this.dur = m::max(0.01, dur);
    this.active = pts.len() > 0;
}

/// The point for time t → out; false once the sweep has finished (out left on the last corner). (squad.ts Sweep.at)
pub fn sweep_at(this: &mut crate::squad::Sweep, t: f64, out: &mut V3) -> bool {
    if !this.active { return false; }
    let n = this.pts.len() as f64 - 1.0;
    let u = v3::clamp((t - this.t0) / this.dur, 0.0, 1.0) * n;
    let i = m::min(n - 1.0, m::floor(u));
    let f = u - i;
    let e = f * f * (3.0 - 2.0 * f);
    let (a, b) = (this.pts[i as usize], this.pts[i as usize + 1]);
    out[0] = a[0] + (b[0] - a[0]) * e;
    out[1] = a[1] + (b[1] - a[1]) * e;
    out[2] = a[2] + (b[2] - a[2]) * e;
    if t - this.t0 >= this.dur { this.active = false; }
    this.active
}

// ---- two-man room clear

/// A pair (or a man alone) clearing a room by a plan. (squad.ts RoomClear.constructor)
pub fn room_clear_new(g: &mut crate::game::Game, plan: &crate::scene_level::RoomClearPlan, a: GuardId, b: Option<GuardId>, opts: Option<crate::squad::RoomClearNewOpts>) -> RoomClearId {
    let door = crate::doors::doors_by_name(g, &plan.door);
    let next_door = next_door_of(g, plan);
    let mut row = RoomClear::blank();
    row.plan = plan.clone();
    row.a = a;
    row.b = b;
    row.door = door;
    row.live = opts.and_then(|o| o.live).unwrap_or(false);
    row.next_door = next_door;
    g.tab.squad_room_clear.push(row)
}

/// Stand the pair on their approach marks (staged) or take them as they come (live), and start them walking into the stack. Restartable. (squad.ts RoomClear.start)
pub fn room_clear_start(g: &mut crate::game::Game, id: RoomClearId) {
    let row = &mut g.tab.squad_room_clear[id];
    row.phase = RoomClearPhase::Stack;
    row.t_kick = f64::NAN; row.t_enter = f64::NAN; row.t_enter2 = f64::NAN; row.t_search = f64::NAN; row.t_resolve = f64::NAN;
    row.done.clear();
    row.flung = false; row.b_go = false; row.no_mark = false;
    row.t_set = f64::NAN; row.t_go = f64::NAN; row.t_now = 0.0;
    row.a_home = false; row.b_home = false;
    row.aim_a = v3::copy(row.plan.stack_aim[0]);
    row.aim_b = v3::copy(row.plan.stack_aim[1]);
    row.sw_a = Sweep::blank();
    row.sw_b = Sweep::blank();
    row.rest_a = None; row.rest_b = None;
    let (live, from, stack, stack_face, stack_side) = (row.live, row.plan.from, row.plan.stack, row.plan.stack_face, row.plan.stack_side);
    let (aim_a, aim_b, has_b) = (row.aim_a, row.aim_b, row.b.is_some());
    let men = room_clear_men(g, id);
    for (i, gd) in men.iter().copied().enumerate() {
        let c = g.gd[gd].ch;
        if let (false, Some(from)) = (live, from) {
            let ch = &mut g.ch[c];
            ch.pos = v3::copy(from[i]);
            ch.vel = [0.0, 0.0, 0.0];
            let yaw = m::atan2(stack[i][0] - from[i][0], stack[i][2] - from[i][2]);
            ch.aim_yaw = yaw; ch.body_yaw = yaw;
            ch.aim_pitch = 0.0;
            g.gd[gd].speed = 0.0;
        }
        let man = &mut g.gd[gd];
        man.state = crate::game::GuardState::Search;
        man.awareness = 0.0; man.last_known = None; man.hold = false; man.pinned = false; man.react_t = 0.0; man.search_t = 0.0; man.script = None;
        g.ch[c].anim.stance = if i == 0 { crate::character::Stance::Stack } else { crate::character::Stance::HighReady };
        if !live { crate::character::character_update(g, c, 0.0); }
    }
    route_man(g, id, false, &[stack[0]], OrderPatch { aim_at: Some(Some(aim_a)), speed: Some(Some(1.6)), face: Some(Some(stack_face[0])), stance: Some(Some(crate::character::Stance::Stack)), stack_side: Some(Some(stack_side)), upper: Some(Some(crate::character::UpperMode::None)), crouch: Some(Some(false)), ..OrderPatch::default() });
    if has_b {
        route_man(g, id, true, &[stack[1]], OrderPatch { aim_at: Some(Some(aim_b)), speed: Some(Some(1.5)), face: Some(Some(stack_face[1])), stance: Some(Some(crate::character::Stance::HighReady)), upper: Some(Some(crate::character::UpperMode::None)), crouch: Some(Some(false)), ..OrderPatch::default() });
    }
}

/// Hand both men back to the AI. (squad.ts RoomClear.end)
pub fn room_clear_end(g: &mut crate::game::Game, id: RoomClearId, state: Option<crate::squad::RoomClearEndState>) {
    let st = match state.unwrap_or(RoomClearEndState::Patrol) { RoomClearEndState::Patrol => EndGuardScriptState::Patrol, RoomClearEndState::Search => EndGuardScriptState::Search };
    let (a, b) = (g.tab.squad_room_clear[id].a, g.tab.squad_room_clear[id].b);
    end_guard_script(g, a, Some(st));
    if let Some(b) = b { end_guard_script(g, b, Some(st)); }
}

/// False once there is nothing left to drive. (squad.ts RoomClear.intact)
pub fn room_clear_intact(g: &mut crate::game::Game, id: RoomClearId) -> bool {
    let (door, a, b) = (g.tab.squad_room_clear[id].door, g.tab.squad_room_clear[id].a, g.tab.squad_room_clear[id].b);
    door.is_some() && on_his_script(g, a) && match b { None => true, Some(b) => on_his_script(g, b) }
}

/// Rallied on the door after calling it, or long enough after that a man who cannot make his mark is not waited for. (squad.ts RoomClear.complete)
pub fn room_clear_complete(g: &mut crate::game::Game, id: RoomClearId) -> bool {
    let row = &g.tab.squad_room_clear[id];
    let tr = row.t_now - row.t_resolve;
    row.phase == RoomClearPhase::Resolve && (tr > 7.5 || (tr > 2.8 && row.done.has(String::from("#formup")) && row.a_home && row.b_home))
}

/// (squad.ts RoomClear.men)
pub fn room_clear_men(g: &mut crate::game::Game, id: RoomClearId) -> Vec<GuardId> {
    let row = &g.tab.squad_room_clear[id];
    match row.b { Some(b) => vec![row.a, b], None => vec![row.a] }
}

/// First time only. (squad.ts RoomClear.once)
pub fn room_clear_once(g: &mut crate::game::Game, id: RoomClearId, key: &str) -> bool {
    let done = &mut g.tab.squad_room_clear[id].done;
    if done.has(String::from(key)) { return false; }
    done.add(String::from(key));
    true
}

/// The plan's line for this step, once; or, with a found colleague lying near him, the version that does not call the corner empty. (squad.ts RoomClear.say)
pub fn room_clear_say(g: &mut crate::game::Game, id: RoomClearId, key: crate::squad::RoomClearSayKey, gd: GuardId, radio: Option<bool>) {
    let radio = radio.unwrap_or(false);
    if !room_clear_once(g, id, key.as_str()) { return; }
    let body = if line_of!(body_lines(), key).is_some() { room_clear_body_here(g, id, gd) } else { None };
    let line = match body {
        Some(bb) => {
            if crate::game::is_breathing(g, bb) { line_of!(body_lines_cold_now(), key).or(line_of!(body_lines_cold(), key)) }
            else { line_of!(body_lines_now(), key).or(line_of!(body_lines(), key)) }
        }
        None => None,
    };
    let text = match line { Some(x) => x, None => line_of!(g.tab.squad_room_clear[id].plan.lines.clone(), key) };
    crate::game::game_say(g, gd, &text, Some(radio), None, None);
}

/// A found colleague lying in this room within a few metres of the man, in his plain sight. (squad.ts RoomClear.bodyHere)
pub fn room_clear_body_here(g: &mut crate::game::Game, id: RoomClearId, gd: GuardId) -> Option<GuardId> {
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let bb = crate::guards::near_known_body(g, p, Some(3.5))?;
    let bc = g.gd[bb].ch;
    let h = match g.ch[bc].bones.get(crate::character::Bone::Hips) { Some(x) => *x, None => g.ch[bc].pos };
    if let Some(bounds) = g.tab.squad_room_clear[id].plan.bounds {
        if !in_bounds([h[0], 0.0, h[2]], Some(bounds), Some(0.3)) { return None; }
    }
    let head = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(x) => *x, None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]) };
    if crate::scene_collision::segment_blocked(g, head, [h[0], h[1] + 0.3, h[2]], None, None, None) { None } else { Some(bb) }
}

/// New waypoint list for a man, and orders; the drill's own null aim (squad.ts:333) goes through `route_with`. (squad.ts RoomClear.route)
pub fn room_clear_route(g: &mut crate::game::Game, id: RoomClearId, gd: GuardId, wps: &mut Vec<V3>, pts: &[V3], patch: Option<crate::squad::RoomClearRoutePatch>) {
    route_with(g, gd, wps, pts, patch_of(patch.unwrap_or_default()));
}

/// Pass intermediate waypoints on the fly, keep the last as the goal; true once planted on the last. (squad.ts RoomClear.advance)
pub fn room_clear_advance(g: &mut crate::game::Game, id: RoomClearId, gd: GuardId, wps: &mut Vec<V3>) -> bool {
    let c = g.gd[gd].ch;
    while wps.len() > 1 && flat(g.ch[c].pos, wps[0]) < SETTLE + 0.1 {
        wps.remove(0);
        order(g, gd, OrderPatch { goal: Some(Some(wps[0])), ..OrderPatch::default() });
    }
    wps.len() <= 1 && g.gd[gd].script.map_or(false, |s| s.arrived.unwrap_or(false))
}

/// A point `dist` metres out along a facing from `p`, at height y. (squad.ts RoomClear.ahead)
pub fn room_clear_ahead(p: V3, yaw: f64, dist: f64, y: f64) -> V3 {
    [p[0] + m::sin(yaw) * dist, y, p[2] + m::cos(yaw) * dist]
}

/// #1 up and through the door, his muzzle on the middle of the room, then his corner, then his wall. (squad.ts RoomClear.enter)
pub fn room_clear_enter(g: &mut crate::game::Game, id: RoomClearId, t: f64) {
    let row = &mut g.tab.squad_room_clear[id];
    row.t_enter = t;
    row.phase = RoomClearPhase::Enter;
    let (entry, face, aim, list) = (row.plan.entry[0].clone(), row.plan.pod_face[0], row.aim_a, row.plan.entry_aim[0].clone());
    route_man(g, id, false, &entry, OrderPatch { aim_at: Some(Some(aim)), speed: Some(Some(2.3)), face: Some(Some(face)), strafe: Some(Some(false)), stance: Some(Some(crate::character::Stance::None)), upper: Some(Some(crate::character::UpperMode::Aim)), ..OrderPatch::default() });
    let rest = enter_sweep(g, id, false, aim, &list, t);
    g.tab.squad_room_clear[id].rest_a = rest;
}

/// enterSweep for a drill's own man: `second` picks swB. (squad.ts:264)
fn enter_sweep(g: &mut Game, id: RoomClearId, second: bool, aim: V3, list: &[V3], t: f64) -> Option<Vec<V3>> {
    let row = &mut g.tab.squad_room_clear[id];
    let centre = room_centre(&row.plan);
    sweep_start(if second { &mut row.sw_b } else { &mut row.sw_a }, aim, &[centre, list[0]], t, centre_check().corner);
    if list.len() > 1 { Some(list[1..].to_vec()) } else { None }
}

/// A man's entry sweep: the room's centre, then his own corner; the rest of the list is handed back. `sw_key` names the row and cannot say which of its two
/// sweeps; as the index reads, it is swA. The drill itself calls `enter_sweep`. (squad.ts RoomClear.enterSweep)
pub fn room_clear_enter_sweep(g: &mut crate::game::Game, id: RoomClearId, sw_key: RoomClearId, aim: V3, list: &[V3], t: f64) -> Option<Vec<V3>> {
    enter_sweep(g, sw_key, false, aim, list, t)
}

/// The keyed breach: the leaf unlatched and shoved hard into the room, and he flows in behind it. (squad.ts RoomClear.shove)
pub fn room_clear_shove(g: &mut crate::game::Game, id: RoomClearId, t: f64) {
    let row = &g.tab.squad_room_clear[id];
    let (d, fling, a) = (row.door.unwrap(), row.plan.fling, row.a);
    let c = g.gd[a].ch;
    let s = js::or(m::sign(fling), -1.0);
    if !crate::doors::door_off(g, d) && crate::doors::doors_open_for(g, d, s) < 1.0 {
        crate::doors::door_set_latched(g, d, false);
        crate::doors::door_set_closing(g, d, false);
        let v = crate::doors::door_vel(g, d);
        crate::doors::door_set_vel(g, d, s * m::max(m::abs(v), m::abs(fling)));
        crate::doors::door_set_quiet_t(g, d, -1.0);
        crate::doors::door_set_creak_armed(g, d, false);
        let who = g.ch[c].id;
        crate::doors::door_set_last_who(g, d, who);
        let at = crate::doors::door_pos(g, d);
        seam_audio_play(g, "doorBash", at, 0.5, 1.15);
    }
    let row = &mut g.tab.squad_room_clear[id];
    row.t_kick = t;
    row.flung = true;
    room_clear_say(g, id, RoomClearSayKey::In1, a, None);
    room_clear_enter(g, id, t);
}

/// One beat of the drill, on the caller's clock. (squad.ts RoomClear.tick)
pub fn room_clear_tick(g: &mut crate::game::Game, id: RoomClearId, t: f64, dt: f64) {
    let (a, b, door) = { let row = &g.tab.squad_room_clear[id]; (row.a, row.b, row.door) };
    let d = match door { Some(d) => d, None => return };
    if !room_clear_intact(g, id) { return; }
    g.tab.squad_room_clear[id].t_now = t;
    let mut a_in = advance_man(g, id, false);
    let mut b_in = if b.is_some() { advance_man(g, id, true) } else { true };
    {
        let row = &mut g.tab.squad_room_clear[id];
        if row.sw_a.active {
            let was = row.aim_a;
            let mut out = was;
            sweep_at(&mut row.sw_a, t, &mut out);
            row.aim_a = out;
            aim_moved(g, a, was, out);
        }
        let row = &mut g.tab.squad_room_clear[id];
        if row.sw_b.active {
            let was = row.aim_b;
            let mut out = was;
            sweep_at(&mut row.sw_b, t, &mut out);
            row.aim_b = out;
            if let Some(bb) = b { aim_moved(g, bb, was, out); }
        }
    }
    let (live, kick_from, kick_face, stack, fling, kick, phase) = {
        let row = &g.tab.squad_room_clear[id];
        let p = &row.plan;
        (row.live, p.kick_from, p.kick_face, p.stack, p.fling, p.breach.unwrap_or(crate::scene_level::RoomClearPlanBreach::Kick) == crate::scene_level::RoomClearPlanBreach::Kick, row.phase)
    };
    let ca = g.gd[a].ch;
    let cb = b.map(|x| g.gd[x].ch);
    match phase {
        RoomClearPhase::Stack => {
            if t > 0.6 { room_clear_say(g, id, RoomClearSayKey::Stack1, a, None); }
            let squeeze;
            if !live {
                if let (Some(_), Some(cb)) = (b, cb) {
                    if t > 1.2 && t < 1.6 && b_in && room_clear_once(g, id, "#hold") { crate::character::anim_signal(g, cb, crate::character::SignalKind::Squeeze, None); }
                }
                if let Some(bb) = b { if t > 1.6 { room_clear_say(g, id, RoomClearSayKey::Stack2, bb, None); } }
                squeeze = t >= 2.5 && match cb { None => true, Some(cb) => !crate::character::anim_signalling(g, cb) || t > 2.9 };
            } else {
                if g.tab.squad_room_clear[id].t_set.is_nan() && ((a_in && b_in) || t > STACK_CAP) {
                    g.tab.squad_room_clear[id].t_set = t;
                    if let Some(cb) = cb { if b_in && room_clear_once(g, id, "#hold") { crate::character::anim_signal(g, cb, crate::character::SignalKind::Squeeze, None); } }
                }
                if let Some(bb) = b { if (b_in || t > 2.2) && t > 1.0 { room_clear_say(g, id, RoomClearSayKey::Stack2, bb, None); } }
                let t_set = g.tab.squad_room_clear[id].t_set;
                squeeze = !t_set.is_nan() && t >= m::max(2.5, t_set + 1.0) && match cb { None => true, Some(cb) => !crate::character::anim_signalling(g, cb) || t > t_set + 1.5 };
            }
            if squeeze {
                if let (Some(bb), Some(cb)) = (b, cb) {
                    crate::character::anim_signal(g, cb, crate::character::SignalKind::Go, None);
                    room_clear_say(g, id, RoomClearSayKey::Go2, bb, None);
                }
                let no_mark = !kick && !crate::doors::door_is_closed(g, d);
                let row = &mut g.tab.squad_room_clear[id];
                row.t_go = t;
                row.phase = RoomClearPhase::Breach;
                row.no_mark = no_mark;
                if !no_mark {
                    route_man(g, id, false, &[kick_from], OrderPatch { speed: Some(Some(1.2)), face: Some(Some(kick_face)), stance: Some(Some(crate::character::Stance::HighReady)), upper: Some(Some(crate::character::UpperMode::None)), ..OrderPatch::default() });
                    a_in = false;
                }
            }
        }
        RoomClearPhase::Breach => {
            let (p, yaw) = (g.ch[ca].pos, g.ch[ca].body_yaw);
            let planted = a_in && flat(p, kick_from) < 0.07 && m::abs(v3::wrap_angle(yaw - kick_face)) < 0.1;
            let row = &g.tab.squad_room_clear[id];
            let t0 = if live { row.t_go } else { 2.5 };
            let no_mark = row.no_mark;
            if let Some(bb) = b {
                let fc = g.dr[d].frame_centre;
                let row = &mut g.tab.squad_room_clear[id];
                let was = row.aim_b;
                row.aim_b[0] = v3::damp(was[0], fc[0], 3.0, dt);
                row.aim_b[1] = v3::damp(was[1], 1.5, 3.0, dt);
                row.aim_b[2] = v3::damp(was[2], fc[2], 3.0, dt);
                let now = row.aim_b;
                aim_moved(g, bb, was, now);
            }
            if if no_mark { t > t0 + 0.45 } else { (planted && t > t0 + 0.5) || t > t0 + 1.4 } {
                if kick {
                    order(g, a, OrderPatch { goal: Some(None), aim_at: Some(None), ..OrderPatch::default() });
                    crate::character::anim_kick_door(g, ca, None);
                    let row = &mut g.tab.squad_room_clear[id];
                    row.t_kick = t;
                    row.phase = RoomClearPhase::Kick;
                } else {
                    room_clear_shove(g, id, t);
                    a_in = false;
                }
            }
        }
        RoomClearPhase::Kick => {
            if !g.tab.squad_room_clear[id].flung
                && (crate::character::anim_kick_impact(g, ca) || crate::character::anim_kick_time(g, ca) >= crate::character::KICK_IMPACT || !crate::character::anim_kicking(g, ca))
            {
                g.tab.squad_room_clear[id].flung = true;
                let (p, yaw) = (g.ch[ca].pos, g.ch[ca].body_yaw);
                let on_it = flat(p, kick_from) < 0.4 && m::abs(v3::wrap_angle(yaw - kick_face)) < 0.35;
                if m::abs(crate::doors::door_angle(g, d)) < 1.4 {
                    crate::doors::door_set_latched(g, d, false);
                    crate::doors::door_set_closing(g, d, false);
                    crate::doors::door_set_vel(g, d, if on_it { fling } else { fling * 0.32 });
                    if on_it { crate::doors::door_set_since_bash(g, d, 0.0); }
                    crate::doors::door_set_quiet_t(g, d, -0.5);
                    crate::doors::door_set_creak_armed(g, d, false);
                }
                let who = g.ch[ca].id;
                crate::doors::door_set_last_who(g, d, who);
                if on_it && !crate::doors::door_off(g, d) {
                    let at = crate::doors::door_pos(g, d);
                    seam_audio_play(g, "doorBash", at, 1.0, 0.9);
                    crate::marks::kick_scuff(g, d, p);
                    crate::particle_fx::kick_dust_fx(g, d, p);
                }
                room_clear_say(g, id, RoomClearSayKey::In1, a, None);
                let k = m::min(1.0, KICK_BACK / m::max(1e-3, flat(kick_from, stack[0])));
                route_man(g, id, false, &[v3::lerp3(kick_from, stack[0], k)], OrderPatch { aim_at: Some(None), speed: Some(Some(1.6)), face: Some(Some(kick_face)), strafe: Some(Some(true)), stance: Some(Some(crate::character::Stance::HighReady)), upper: Some(Some(crate::character::UpperMode::None)), ..OrderPatch::default() });
                a_in = false;
            }
            let (flung, t_kick) = { let row = &g.tab.squad_room_clear[id]; (row.flung, row.t_kick) };
            if flung && (!crate::character::anim_kicking(g, ca) || t > t_kick + 1.3) {
                room_clear_enter(g, id, t);
                a_in = false;
            }
        }
        RoomClearPhase::Enter => {
            let te = t - g.tab.squad_room_clear[id].t_enter;
            if let (Some(_), Some(cb)) = (b, cb) {
                let pa = g.ch[ca].pos;
                let go = !g.tab.squad_room_clear[id].b_go
                    && if live { te > 0.85 && flat(pa, g.ch[cb].pos) > 1.5 } else { te > 0.85 || flat(pa, kick_from) > 1.6 };
                if go {
                    let row = &mut g.tab.squad_room_clear[id];
                    row.b_go = true;
                    row.t_enter2 = t;
                    let (entry, face, aim, list) = (row.plan.entry[1].clone(), row.plan.pod_face[1], row.aim_b, row.plan.entry_aim[1].clone());
                    route_man(g, id, true, &entry, OrderPatch { speed: Some(Some(2.2)), face: Some(Some(face)), stance: Some(Some(crate::character::Stance::None)), upper: Some(Some(crate::character::UpperMode::Aim)), ..OrderPatch::default() });
                    b_in = false;
                    let rest = enter_sweep(g, id, true, aim, &list, t);
                    g.tab.squad_room_clear[id].rest_b = rest;
                }
            }
            sweep_rest(g, id, false, t);
            if b.is_some() { sweep_rest(g, id, true, t); }
            if a_in && te > 1.5 { room_clear_say(g, id, RoomClearSayKey::Side1, a, None); }
            if let Some(bb) = b { if g.tab.squad_room_clear[id].b_go && b_in && te > 1.5 { room_clear_say(g, id, RoomClearSayKey::Side2, bb, None); } }
            if te > 3.4 {
                let row = &mut g.tab.squad_room_clear[id];
                row.t_search = t;
                row.phase = RoomClearPhase::Search;
                row.rest_a = None;
                let (path, face, walk) = (row.plan.search_path.clone(), row.plan.search_face, row.plan.search_aim_walk.clone());
                route_man(g, id, false, &path, OrderPatch { speed: Some(Some(1.3)), face: Some(Some(face)), upper: Some(Some(crate::character::UpperMode::Aim)), ..OrderPatch::default() });
                a_in = false;
                sweep_begin(g, id, false, &walk, t, 3.0);
            }
        }
        RoomClearPhase::Search => {
            let ts = t - g.tab.squad_room_clear[id].t_search;
            if b.is_some() { sweep_rest(g, id, true, t); }
            if ts > 0.35 && b.is_some() { room_clear_say(g, id, RoomClearSayKey::Push1, a, None); }
            if let Some(bb) = b {
                if ts > 0.5 && g.tab.squad_room_clear[id].rest_b.is_none() && room_clear_once(g, id, "#kneel") {
                    order(g, bb, OrderPatch { crouch: Some(Some(true)), ..OrderPatch::default() });
                    let under = g.tab.squad_room_clear[id].plan.under_aim.clone();
                    sweep_begin(g, id, true, &under, t, 3.2);
                }
                if ts > 0.9 { room_clear_say(g, id, RoomClearSayKey::Under2, bb, None); }
            }
            if ts > 3.0 && (a_in || ts > 4.0) && room_clear_once(g, id, "#across") {
                let there = g.tab.squad_room_clear[id].plan.search_aim_there.clone();
                sweep_begin(g, id, false, &there, t, 2.4);
            }
            if let Some(bb) = b {
                if ts > 3.4 { room_clear_say(g, id, RoomClearSayKey::Nothing2, bb, None); }
                if ts > 3.7 && room_clear_once(g, id, "#standup") {
                    let row = &g.tab.squad_room_clear[id];
                    let (nd, up_aim) = (row.next_door, row.plan.up_aim);
                    if let Some(nd) = nd {
                        let fc = g.dr[nd].frame_centre;
                        let pb = g.ch[g.gd[bb].ch].pos;
                        order(g, bb, OrderPatch { crouch: Some(Some(false)), face: Some(Some(m::atan2(fc[0] - pb[0], fc[2] - pb[2]))), ..OrderPatch::default() });
                        sweep_begin(g, id, true, &[[fc[0], 1.0, fc[2]]], t, 1.2);
                    } else {
                        order(g, bb, OrderPatch { crouch: Some(Some(false)), ..OrderPatch::default() });
                        sweep_begin(g, id, true, &[up_aim], t, 1.2);
                    }
                }
            }
            if ts > 4.1 && (a_in || ts > 4.5) { room_clear_say(g, id, RoomClearSayKey::Corner1, a, None); }
            if ts > 4.7 {
                let row = &mut g.tab.squad_room_clear[id];
                row.t_resolve = t;
                row.phase = RoomClearPhase::Resolve;
            }
        }
        RoomClearPhase::Resolve => {
            let tr = t - g.tab.squad_room_clear[id].t_resolve;
            room_clear_say(g, id, RoomClearSayKey::Empty2, b.unwrap_or(a), None);
            if tr > 0.9 { room_clear_say(g, id, RoomClearSayKey::Radio1, a, Some(true)); }
            if tr > 1.0 && room_clear_once(g, id, "#lower") {
                order(g, a, OrderPatch { stance: Some(Some(crate::character::Stance::LowReady)), upper: Some(Some(crate::character::UpperMode::None)), ..OrderPatch::default() });
                if let Some(bb) = b { order(g, bb, OrderPatch { stance: Some(Some(crate::character::Stance::LowReady)), upper: Some(Some(crate::character::UpperMode::None)), crouch: Some(Some(false)), ..OrderPatch::default() }); }
            }
            if b.is_some() && tr > 1.15 && room_clear_once(g, id, "#rally") { crate::character::anim_signal(g, ca, crate::character::SignalKind::Rally, None); }
            if tr > 1.3 && room_clear_once(g, id, "#formup") {
                let row = &g.tab.squad_room_clear[id];
                let (fa, fb, ff) = (row.plan.form_up[0].clone(), row.plan.form_up[1].clone(), row.plan.form_face);
                let ea = fa[fa.len() - 1];
                route_man(g, id, false, &fa, OrderPatch { speed: Some(Some(1.8)), face: Some(Some(ff[0])), stance: Some(Some(crate::character::Stance::LowReady)), upper: Some(Some(crate::character::UpperMode::None)), ..OrderPatch::default() });
                a_in = false;
                sweep_begin(g, id, false, &[room_clear_ahead(ea, ff[0], 3.0, 0.4)], t, 2.0);
                if b.is_some() {
                    route_man(g, id, true, &fb, OrderPatch { speed: Some(Some(1.2)), face: Some(Some(ff[1])), stance: Some(Some(crate::character::Stance::LowReady)), upper: Some(Some(crate::character::UpperMode::None)), ..OrderPatch::default() });
                    b_in = false;
                    let eb = fb[fb.len() - 1];
                    sweep_begin(g, id, true, &[room_clear_ahead(eb, ff[1], 3.0, 0.4)], t, 1.5);
                }
            }
        }
    }
    let row = &mut g.tab.squad_room_clear[id];
    row.a_home = a_in;
    row.b_home = b_in;
}

/// Seconds for the rest of an entry sweep: what is left of the old three-second sweep, never under minLeg a leg. (squad.ts restSecs)
pub fn rest_secs(rest: &[V3]) -> f64 {
    let c = centre_check();
    m::max(c.min_leg * rest.len() as f64, c.total - c.corner)
}

/// The middle of the room at muzzle height: the plan's bounds, else the centroid of both men's entry sweeps. (squad.ts roomCentre)
pub fn room_centre(p: &crate::scene_level::RoomClearPlan) -> V3 {
    if let Some(b) = p.bounds { return [(b.x0 + b.x1) / 2.0, 1.0, (b.z0 + b.z1) / 2.0]; }
    let (mut x, mut z) = (0.0, 0.0);
    for q in p.entry_aim[0].iter().chain(p.entry_aim[1].iter()) { x += q[0]; z += q[2]; }
    let n = (p.entry_aim[0].len() + p.entry_aim[1].len()) as f64;
    [x / n, 1.0, z / n]
}

/// Another door of the room the plan clears, for #2 to hold: the nearest to his point of domination, or None. (squad.ts nextDoorOf)
pub fn next_door_of(g: &mut crate::game::Game, p: &crate::scene_level::RoomClearPlan) -> Option<DoorId> {
    let b = p.bounds?;
    let pod = match p.entry[1].last() { Some(x) => *x, None => p.stack[1] };
    let mut best = None;
    let mut bd = f64::INFINITY;
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if g.dr[d].def.name == p.door { continue; }
        let fc = g.dr[d].frame_centre;
        if fc[0] < b.x0 - 0.3 || fc[0] > b.x1 + 0.3 || fc[2] < b.z0 - 0.3 || fc[2] > b.z1 + 0.3 { continue; }
        let dd = flat(fc, pod);
        if dd < bd { bd = dd; best = Some(d); }
    }
    best
}

/// Break a scripted man off into the AI with something on his mind: patrol for exactly one frame carrying this awareness and fix. (squad.ts breakOff)
pub fn break_off(g: &mut crate::game::Game, gd: GuardId, awareness: f64, last_known: Option<V3>) {
    end_guard_script(g, gd, Some(EndGuardScriptState::Suspicious));
    if g.gd[gd].state == crate::game::GuardState::Dead { return; }
    let man = &mut g.gd[gd];
    man.state = crate::game::GuardState::Patrol;
    man.awareness = m::max(man.awareness, awareness);
    if let Some(k) = last_known { man.last_known = Some(v3::copy(k)); }
    man.react_t = 0.0;
    man.search_t = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn same(got: V3, want: [u64; 3]) {
        for i in 0..3 { assert_eq!(got[i].to_bits(), want[i], "component {i}: {}", got[i]); }
    }

    /// The beats and points of Sweep against the TypeScript's own bits (bun, EXACT.on).
    #[test]
    fn sweep_runs_the_typescripts_bits() {
        let mut s = Sweep::blank();
        sweep_start(&mut s, [0.1, 1.2, -3.3], &[[2.7, 0.3, 4.1], [-1.9, 1.7, 0.6], [5.3, 0.9, -2.2]], 10.25, 3.1);
        let mut out = [9.0, 9.0, 9.0];
        let rows: [(f64, bool, [u64; 3]); 8] = [
            (10.25, true, [0x3fb999999999999a, 0x3ff3333333333333, 0xc00a666666666666]),
            (10.9, true, [0x3ffe45db180ba545, 0x3fe28caf363ed47a, 0x3ffcceb287641954]),
            (11.37, true, [0x4004dde8a705927b, 0x3fd4fc2ff1212e48, 0x40101efee8b9372b]),
            (12.01, true, [0xbfed99eb71197a44, 0x3ff673ce1fae2c35, 0x3ff578164a662b14]),
            (13.0, true, [0x400b0d578048d0b5, 0x3ff1cf8627c24ec6, 0xbff743440e7185e4]),
            (13.35, true, [0x4015333333333332, 0x3feccccccccccccd, 0xc00199999999999a]),
            (13.36, false, [0x4015333333333332, 0x3feccccccccccccd, 0xc00199999999999a]),
            (15.0, false, [0x4015333333333332, 0x3feccccccccccccd, 0xc00199999999999a]),
        ];
        for (t, active, want) in rows {
            assert_eq!(sweep_at(&mut s, t, &mut out), active, "t {t}");
            assert_eq!(s.active, active, "t {t}");
            same(out, want);
        }
    }

    #[test]
    fn a_sweep_with_no_points_is_not_active_and_leaves_out_alone() {
        let mut s = Sweep::blank();
        sweep_start(&mut s, [1.0, 2.0, 3.0], &[], 0.0, 1.0);
        let mut out = [9.0, 9.0, 9.0];
        assert!(!sweep_at(&mut s, 0.5, &mut out));
        assert_eq!(out, [9.0, 9.0, 9.0]);
    }

    #[test]
    fn ahead_is_the_typescripts_bits() {
        same(room_clear_ahead([1.5, 0.0, -2.25], 0.7, 3.0, 0.4), [0x400b7612cef4934a, 0x3fd999999999999a, 0x3fa6cc2f7ded8b80]);
    }

    /// The minLeg floor against the old three-second sweep's remainder.
    #[test]
    fn rest_secs_floors_at_a_leg_each() {
        assert_eq!(rest_secs(&[[0.0; 3]; 3]).to_bits(), (3.0f64 - 0.95).to_bits());
        assert_eq!(rest_secs(&[[0.0; 3]; 4]).to_bits(), (0.6f64 * 4.0).to_bits());
    }
}
