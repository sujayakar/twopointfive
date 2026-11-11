use crate::v3::dist_xz;
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The field of a table of the plan's lines (or of BODY_LINES and its kin) that a say key names.
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

// ---- orders

/// A new waypoint list for a man, and orders: the goal is always a fresh copy, so always a new leg.
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
    let planted = room_clear_advance(g, gd, &mut wps);
    let row = &mut g.tab.squad_room_clear[id];
    if second { row.wp_b = wps; } else { row.wp_a = wps; }
    planted
}

/// The script holds a copy of the drill's aim point, not the point the sweep moves, so
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

fn on_his_script(g: &Game, gd: GuardId) -> bool {
    g.guards.contains(&gd) && g.gd[gd].script.is_some() && !crate::condition::down(g, g.gd[gd].ch)
}

// ---- the runner

/// Side-step / back-pedal along the nav path, the body left on `face`; true once the path is consumed.
fn strafe_along_path(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> bool {
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
        { let to = v3::mad(g.ch[c].pos, vel, dt); crate::character::pos_replace(g, c, to); }
        return false;
    }
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    true
}

/// What this step of the script worked out at its start.
#[derive(Clone, Copy)]
struct Step {
    gd: GuardId,
    c: CharId,
    dt: f64,
    speed: f64,
    legs: f64,
    strafing: bool,
    fight_face: Option<f64>,
}

/// What his walk came to this step.
#[derive(Clone, Copy)]
struct Walked {
    move_speed: f64,
    arrived: bool,
    settling: bool,
    /// the way he is stepping on a long settle: the body goes with it
    far_leg: Option<f64>,
}

/// One frame of a man under a script: moved to his goal, gun and eyes on his aim, the animator fed, `arrived` written back.
pub fn run_guard_script(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    // THE SCRIPT IS READ LIVE: a rule called from here may re-order the man mid-step (follow_path's lane gate backs him out with a new order), and every read of
    // `s` after that call is of the order as it then stands (the pose's crouch, the gun, the facing, the goal the settle watches). `s` is this step's copy,
    // taken again by `live` after each call that can reach `order`. What was worked out before such a call (pace, strafing, the leg's numbers) stands.
    let mut s = g.gd[gd].script.unwrap();
    let c = g.gd[gd].ch;
    let (x0, z0) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    let ordered = match s.speed { Some(x) => x, None => g.tune.guard_walk };
    let speed = crate::guards::team_pace(g, gd, ordered);
    let legs = crate::guards::team_legs(g, gd, speed);
    let fight_face = fight_face_while_walking(g, gd, &s);
    let strafing = fight_face.is_some() || (s.strafe.unwrap_or(false) && s.face.is_some());
    let st = Step { gd, c, dt, speed, legs, strafing, fight_face };
    let mut walk = match s.goal {
        Some(goal) => walks_to_goal(g, &st, goal, &mut s),
        None => {
            g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
            g.ch[c].vel = [0.0, 0.0, 0.0];
            Walked { move_speed: 0.0, arrived: true, settling: true, far_leg: None }
        }
    };
    body_turns(g, &st, &walk, &s);
    let ch = &mut g.ch[c];
    ch.anim.reverse = walk.move_speed > 0.2 && ch.vel[0] * m::sin(ch.body_yaw) + ch.vel[2] * m::cos(ch.body_yaw) < -0.3 * walk.move_speed;
    let aim = match s.over { Some(o) => Some(o.aim_at), None => s.aim_at };
    let aim0 = g.ch[c].aim_yaw;
    aims(g, &st, aim, &mut s);
    aim_within_his_twist(g, &st, aim0, s.over.is_some());
    let (yaw, pitch) = (g.ch[c].aim_yaw, g.ch[c].aim_pitch);
    g.gd[gd].beam_dir = v3::normalize([m::sin(yaw) * m::cos(pitch), m::sin(pitch), m::cos(yaw) * m::cos(pitch)]);
    let (mut p, radius) = (g.ch[c].pos, g.ch[c].radius);
    crate::scene_collision::collide_circle(g, &mut p, radius, 0.2, 1.5, None);
    g.ch[c].pos = p;
    live(g, gd, &mut s);
    shoved_off_the_others(g, gd);
    if crate::noshove::no_shove_keep(g, gd, x0, z0, dt) { walk.move_speed = 0.0; }
    live(g, gd, &mut s);
    let (upper, stance) = if s.over.is_some() { (crate::character::UpperMode::Aim, crate::character::Stance::None) }
        else { (s.upper.unwrap_or(crate::character::UpperMode::Aim), s.stance.unwrap_or(crate::character::Stance::None)) };
    let up = upper == crate::character::UpperMode::Aim || stance == crate::character::Stance::HighReady || stance == crate::character::Stance::Stack;
    let line = if up && !s.hot.unwrap_or(false) { Some(match aim { Some(at) => at, None => crate::guards::aim_ahead(g, c) }) } else { None };
    crate::guards::muzzle_check(g, gd, line, dt, None);
    live(g, gd, &mut s);
    let gun = if upper == crate::character::UpperMode::Aim { Some(crate::gunwall::GunUp::Aim) } else if up { Some(crate::gunwall::GunUp::Port) } else { None };
    crate::gunwall::gun_wall_check(g, gd, gun, aim, dt, Some(s.over.is_some()));
    live(g, gd, &mut s);
    let low = g.gd[gd].muzzle_down || crate::teampace::team_sprinting(g, gd);
    live(g, gd, &mut s);
    let an = &mut g.ch[c].anim;
    an.speed = walk.move_speed;
    an.upper = if low { crate::character::UpperMode::None } else { upper };
    an.crouch_target = if s.crouch.unwrap_or(false) { 1.0 } else { 0.0 };
    an.lean = s.lean.unwrap_or(0.0);
    an.stance = if low { crate::character::Stance::LowReady } else { stance };
    an.stack_side = s.stack_side.unwrap_or(1.0);
    an.look_yaw_extra = 0.0;
    crate::gunwall::gun_wall_pose(g, c);
    crate::character::character_update(g, c, dt);
    crate::guards::attach_flashlight(g, gd, None);
    live(g, gd, &mut s);
    if s.light == Some(false) && !crate::guards::light_ruled(g, gd) { let l = g.gd[gd].flashlight; g.tab.light[l].enabled = false; }
    footfall(g, gd, walk.move_speed);
    crate::teampace::team_ran(g, gd);
    // (the script this step began with: one made since is not this step's to mark)
    if let Some(sc) = g.gd[gd].script.as_mut() { if sc.serial == s.serial { sc.arrived = Some(walk.arrived); } }
}

/// `s` is his script as it stands now; a script that is no longer the one this step began with (another serial: ended, and a new one made) is not taken, and he finishes this
/// frame on the one he began.
fn live(g: &crate::game::Game, gd: GuardId, s: &mut crate::game::GuardScript) { if let Some(n) = g.gd[gd].script { if n.serial == s.serial { *s = n; } } }

/// A fight's walking order has his body face this way, until he has stood stuck on it for over a second.
fn fight_face_while_walking(g: &mut Game, gd: GuardId, s: &crate::game::GuardScript) -> Option<f64> {
    let face = match s.over { Some(o) if o.walk.unwrap_or(false) => o.face, _ => None };
    face?;
    let (t, p) = (g.time, g.pos_of(gd));
    match g.m.squad.walk_watch.get(gd).copied() {
        Some(w) if !(t < w.t || t - w.t > 3.0 || m::hyp2(p[0] - w.x, p[2] - w.z) > 0.25) => { if t - w.t > 1.2 { return None; } }
        _ => { g.m.squad.walk_watch.set(gd, SquadModWalkWatchValue { x: p[0], z: p[2], t }); }
    }
    face
}

/// Along his path till he is close and has a clear line, then settled onto the goal.
fn walks_to_goal(g: &mut Game, st: &Step, goal: V3, s: &mut crate::game::GuardScript) -> Walked {
    let c = st.c;
    let p = g.ch[c].pos;
    let (dx, dz) = (goal[0] - p[0], goal[2] - p[2]);
    let planar = m::hyp2(dx, dz);
    let mut walk = Walked { move_speed: 0.0, arrived: true, settling: true, far_leg: None };
    let near = planar <= SETTLE && !crate::scene_collision::low_blocked(g, p, goal);
    let home = near || walks_the_path(g, st, goal, s, &mut walk);
    if home {
        settles(g, st, Leg { goal: s.goal.unwrap_or(goal), dx, dz, planar }, s, &mut walk);
        walk.arrived = planar <= 0.3;
    } else {
        walk.arrived = false;
        walk.settling = false;
    }
    if !walk.settling && st.strafing {
        if let Some(to) = st.fight_face.or(s.face) { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, to, 6.0 * st.dt); }
    }
    walk
}

/// A step along the way to his goal, strafing or walking. True: the path is used up.
fn walks_the_path(g: &mut Game, st: &Step, goal: V3, s: &mut crate::game::GuardScript, walk: &mut Walked) -> bool {
    let (gd, dt) = (st.gd, st.dt);
    crate::guards::go_to(g, gd, goal, None);
    live(g, gd, s);
    let home = if g.gd[gd].path_i >= g.gd[gd].path.len() { true } else {
        let done = if st.strafing { strafe_along_path(g, gd, dt, st.legs) } else { crate::guards::follow_path(g, gd, dt, st.speed) };
        live(g, gd, s);
        walk.move_speed = g.gd[gd].speed;
        done && g.gd[gd].path_i >= g.gd[gd].path.len()
    };
    live(g, gd, s);
    home
}

/// The goal as it stands now, with how far off it he was at the start of the step.
#[derive(Clone, Copy)]
struct Leg {
    goal: V3,
    dx: f64,
    dz: f64,
    planar: f64,
}

/// The last of the way, straight on, while it still gets him nearer; a man who has stopped gaining stands still a while, then tries again.
fn settles(g: &mut Game, st: &Step, leg: Leg, s: &mut crate::game::GuardScript, walk: &mut Walked) {
    let (gd, c, dt) = (st.gd, st.c, st.dt);
    let Leg { goal, dx, dz, planar } = leg;
    let mut w = match g.m.squad.settle_watch.get(gd).copied() {
        Some(w) if w.goal == goal => w,
        _ => SquadModSettleWatchValue { goal, best: planar, t: 0.0 },
    };
    if planar < w.best - 0.01 { w.best = planar; w.t = 0.0; }
    else { w.t += dt; if w.t > 3.6 { w.best = planar; w.t = 0.0; } }
    g.m.squad.settle_watch.set(gd, w);
    if !(planar > 0.04 && w.t < 0.6) {
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        return;
    }
    let was = g.gd[gd].speed;
    g.gd[gd].speed = v3::damp(was, m::min(st.legs, m::max(0.35, planar * 2.2)), 6.0, dt);
    crate::guards::team_lean(g, gd, was, dt);
    live(g, gd, s);
    let sp = g.gd[gd].speed;
    let step = m::min(planar, sp * dt);
    g.ch[c].pos[0] += dx / planar * step;
    g.ch[c].pos[2] += dz / planar * step;
    g.ch[c].vel = [dx / planar * sp, 0.0, dz / planar * sp];
    walk.move_speed = sp;
    if planar > FACE_WITHIN && !st.strafing { walk.far_leg = Some(m::atan2(dx, dz)); }
}

/// The body turns to the way he steps on a long settle, else, settling, to the fight's facing or the order's.
fn body_turns(g: &mut Game, st: &Step, walk: &Walked, s: &crate::game::GuardScript) {
    let (c, dt) = (st.c, st.dt);
    let over_face = s.over.and_then(|o| o.face);
    if let Some(leg) = walk.far_leg { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, leg, 6.0 * dt); }
    else if walk.settling && over_face.is_some() { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, over_face.unwrap(), 5.0 * dt); }
    else if walk.settling && s.face.is_some() { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, s.face.unwrap(), 5.0 * dt); }
}

/// Gun and eyes come onto his aim, at once in a fight, else as fast as the team turns; with nothing to aim at they drift back to his front.
fn aims(g: &mut Game, st: &Step, aim: Option<V3>, s: &mut crate::game::GuardScript) {
    let (gd, c, dt) = (st.gd, st.c, st.dt);
    let Some(at) = aim else {
        let ch = &mut g.ch[c];
        ch.aim_yaw = v3::approach_angle(ch.aim_yaw, ch.body_yaw, 4.0 * dt);
        ch.aim_pitch = v3::damp(ch.aim_pitch, -0.1, 3.0, dt);
        return;
    };
    let to = v3::sub(at, g.ch[c].pos);
    let yaw = m::atan2(to[0], to[2]);
    let aim_yaw = if s.over.is_some() { yaw } else { crate::guards::team_turn(g, gd, yaw, dt) };
    live(g, gd, s);
    let ch = &mut g.ch[c];
    ch.aim_yaw = aim_yaw;
    ch.aim_pitch = v3::damp(ch.aim_pitch, m::atan2(at[1] - (1.45 - 0.45 * ch.anim.crouch), m::hyp2(to[0], to[2])), 6.0, dt);
}

/// The aim stays within his twist, and in a paced fight swings at no more than the swing rate.
fn aim_within_his_twist(g: &mut Game, st: &Step, aim0: f64, over: bool) {
    let (gd, c) = (st.gd, st.c);
    let ch = &mut g.ch[c];
    let off = v3::wrap_angle(ch.aim_yaw - ch.body_yaw);
    if m::abs(off) > 1.5 { ch.aim_yaw = ch.body_yaw + m::sign(off) * 1.5; }
    let swing = g.k.pace.swing;
    if over && swing > 0.0 && crate::guards::team_paced(g, gd) {
        let ch = &mut g.ch[c];
        let r0 = v3::clamp(v3::wrap_angle(aim0 - ch.body_yaw), -1.5, 1.5);
        let r1 = v3::wrap_angle(ch.aim_yaw - ch.body_yaw);
        let step = swing * st.dt;
        if m::abs(r1 - r0) > step { ch.aim_yaw = ch.body_yaw + r0 + m::sign(r1 - r0) * step; }
    }
}

/// He gives way to any man on his feet he is standing in.
fn shoved_off_the_others(g: &mut Game, gd: GuardId) {
    let c = g.gd[gd].ch;
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if o == gd || crate::condition::down(g, g.gd[o].ch) { continue; }
        let oc = g.gd[o].ch;
        let mut d = v3::sub(g.ch[c].pos, g.ch[oc].pos);
        d[1] = 0.0;
        let l = v3::len(d);
        if l < 0.6 && l > 1e-4 { let to = v3::mad(g.ch[c].pos, d, (0.6 - l) / l * 0.5); crate::character::pos_replace(g, c, to); }
    }
}

/// Each time a foot comes down on a walk, the floor is told.
fn footfall(g: &mut Game, gd: GuardId, move_speed: f64) {
    let c = g.gd[gd].ch;
    let prev_phase = g.gd[gd].step_phase;
    let phase = g.ch[c].anim.phase;
    g.gd[gd].step_phase = phase;
    if move_speed > 0.4 && ((prev_phase < 0.5) != (phase < 0.5)) {
        let p = g.ch[c].pos;
        let base = crate::teampace::team_tread(g, gd, move_speed, 0.45);
        crate::floor::tread(g, p, base, false);
    }
}

/// Release a scripted guard back to the AI in a sane state; a man who died under a script is only unhooked.
pub fn end_guard_script(g: &mut crate::game::Game, gd: GuardId, state: Option<crate::game::GuardState>) {
    let state = state.unwrap_or(crate::game::GuardState::Patrol);
    g.gd[gd].script = None;
    if crate::condition::down(g, g.gd[gd].ch) { return; }
    let c = g.gd[gd].ch;
    g.ch[c].anim.stance = crate::character::Stance::None;
    g.gd[gd].muzzle_down = false;
    let an = &mut g.ch[c].anim;
    an.crouch_target = 0.0; an.lean = 0.0; an.reverse = false;
    crate::mind::set_state(g, gd, state, "squad");
    g.gd[gd].drop_path();
    if state == crate::game::GuardState::Patrol {
        crate::mind::set(g, gd, 0.0, "squad"); g.gd[gd].react_t = 0.0; g.gd[gd].search_t = 0.0; crate::mind::fix_keep(g, gd, None, "squad"); g.gd[gd].body_duty = None;
    }
}

/// Give a scripted guard new orders; a new goal drops his path, stuck timer and settle watch. A goal is a new leg even at the same spot, unless `held`
/// says the caller keeps it and moves it (then by value). Some(None) is null; None is not in the patch.
pub fn order(g: &mut crate::game::Game, gd: GuardId, patch: crate::squad::OrderPatch) {

    if g.gd[gd].script.is_none() {
        let serial = g.serial();   // (its serial is how a step tells this script from a later one)
        g.gd[gd].script = Some(crate::game::GuardScript { serial, ..crate::game::GuardScript::default() });
    }
    if let Some(goal) = patch.goal {
        let had = g.gd[gd].script.as_ref().unwrap().goal;
        let new_leg = match goal { None => had.is_some(), Some(_) => !patch.held || goal != had };
        if new_leg {
            let man = &mut g.gd[gd];
            man.drop_path(); man.stuck_t = 0.0;
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

/// Begin a muzzle sweep from wherever the point was.
pub fn sweep_start(this: &mut crate::squad::Sweep, from: V3, pts: &[V3], t0: f64, dur: f64) {
    let mut all = Vec::with_capacity(pts.len() + 1);
    all.push(v3::copy(from));
    for p in pts { all.push(v3::copy(*p)); }
    this.pts = all;
    this.t0 = t0;
    this.dur = m::max(0.01, dur);
    this.active = pts.len() > 0;
}

/// The point for time t → out; false once the sweep has finished (out left on the last corner).
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

/// A pair (or a man alone) clearing a room by a plan.
pub(super) fn room_clear_new(g: &mut crate::game::Game, plan: &crate::scene_level::RoomClearPlan, a: GuardId, b: Option<GuardId>, opts: Option<crate::squad::RoomClearNewOpts>) -> RoomClearId {
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

/// Stand the pair on their approach marks (staged) or take them as they come (live), and start them walking into the stack. Restartable.
pub(super) fn room_clear_start(g: &mut crate::game::Game, id: RoomClearId) {
    let row = &mut g.tab.squad_room_clear[id];
    row.phase = RoomClearPhase::Stack;
    row.t_kick = f64::NAN; row.t_enter = f64::NAN;  row.t_search = f64::NAN; row.t_resolve = f64::NAN;
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
            crate::character::pos_replace(g, c, v3::copy(from[i]));
            let ch = &mut g.ch[c];
            ch.vel = [0.0, 0.0, 0.0];
            let yaw = v3::yaw_to(from[i], stack[i]);
            ch.aim_yaw = yaw; ch.body_yaw = yaw;
            ch.aim_pitch = 0.0;
            g.gd[gd].speed = 0.0;
        }
        crate::duty::hold(g, gd, false, "squad"); crate::duty::pin(g, gd, false, "squad");
        crate::mind::set_state(g, gd, crate::game::GuardState::Search, "squad");
        crate::mind::set(g, gd, 0.0, "squad"); crate::mind::fix_keep(g, gd, None, "squad"); g.gd[gd].react_t = 0.0; g.gd[gd].search_t = 0.0; g.gd[gd].script = None;
        g.ch[c].anim.stance = if i == 0 { crate::character::Stance::Stack } else { crate::character::Stance::HighReady };
        if !live { crate::character::character_update(g, c, 0.0); }
    }
    route_man(g, id, false, &[stack[0]], OrderPatch { aim_at: Some(Some(aim_a)), speed: Some(Some(1.6)), face: Some(Some(stack_face[0])), stance: Some(Some(crate::character::Stance::Stack)), stack_side: Some(Some(stack_side)), upper: Some(Some(crate::character::UpperMode::None)), crouch: Some(Some(false)), ..OrderPatch::default() });
    if has_b {
        route_man(g, id, true, &[stack[1]], OrderPatch { aim_at: Some(Some(aim_b)), speed: Some(Some(1.5)), face: Some(Some(stack_face[1])), stance: Some(Some(crate::character::Stance::HighReady)), upper: Some(Some(crate::character::UpperMode::None)), crouch: Some(Some(false)), ..OrderPatch::default() });
    }
}

/// False once there is nothing left to drive.
pub(super) fn room_clear_intact(g: &crate::game::Game, id: RoomClearId) -> bool {
    let (door, a, b) = (g.tab.squad_room_clear[id].door, g.tab.squad_room_clear[id].a, g.tab.squad_room_clear[id].b);
    door.is_some() && on_his_script(g, a) && match b { None => true, Some(b) => on_his_script(g, b) }
}

/// Rallied on the door after calling it, or long enough after that a man who cannot make his mark is not waited for.
pub(super) fn room_clear_complete(g: &crate::game::Game, id: RoomClearId) -> bool {
    let row = &g.tab.squad_room_clear[id];
    let tr = row.t_now - row.t_resolve;
    row.phase == RoomClearPhase::Resolve && (tr > 7.5 || (tr > 2.8 && row.done.holds("#formup") && row.a_home && row.b_home))
}

fn room_clear_men(g: &crate::game::Game, id: RoomClearId) -> Vec<GuardId> {
    let row = &g.tab.squad_room_clear[id];
    match row.b { Some(b) => vec![row.a, b], None => vec![row.a] }
}

/// First time only.
fn room_clear_once(g: &mut crate::game::Game, id: RoomClearId, key: &str) -> bool {
    let done = &mut g.tab.squad_room_clear[id].done;
    if done.holds(key) { return false; }
    done.add(String::from(key));
    true
}

/// The plan's line for this step, once; or, with a found colleague lying near him, the version that does not call the corner empty.
fn room_clear_say(g: &mut crate::game::Game, id: RoomClearId, key: crate::squad::RoomClearSayKey, gd: GuardId, radio: Option<bool>) {
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
    crate::game::game_say(g, gd, &text, Some(radio), None);
}

/// A found colleague lying in this room within a few metres of the man, in his plain sight.
fn room_clear_body_here(g: &crate::game::Game, id: RoomClearId, gd: GuardId) -> Option<GuardId> {
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let bb = crate::guards::near_known_body(g, p, Some(3.5))?;
    let bc = g.gd[bb].ch;
    let h = g.ch[bc].bone(crate::character::Bone::Hips).unwrap_or(g.ch[bc].pos);
    if let Some(bounds) = g.tab.squad_room_clear[id].plan.bounds {
        if !in_bounds([h[0], 0.0, h[2]], Some(bounds), Some(0.3)) { return None; }
    }
    let head = g.ch[c].bone_or(crate::character::Bone::Head, 1.65);
    if crate::scene_collision::segment_blocked(g, head, [h[0], h[1] + 0.3, h[2]], None, None, None) { None } else { Some(bb) }
}

/// Pass intermediate waypoints on the fly, keep the last as the goal; true once planted on the last.
fn room_clear_advance(g: &mut crate::game::Game, gd: GuardId, wps: &mut Vec<V3>) -> bool {
    let c = g.gd[gd].ch;
    while wps.len() > 1 && dist_xz(g.ch[c].pos, wps[0]) < SETTLE + 0.1 {
        wps.remove(0);
        order(g, gd, OrderPatch { goal: Some(Some(wps[0])), ..OrderPatch::default() });
    }
    wps.len() <= 1 && g.gd[gd].script.map_or(false, |s| s.arrived.unwrap_or(false))
}

/// A point `dist` metres out along a facing from `p`, at height y.
fn room_clear_ahead(p: V3, yaw: f64, dist: f64, y: f64) -> V3 {
    [p[0] + m::sin(yaw) * dist, y, p[2] + m::cos(yaw) * dist]
}

/// #1 up and through the door, his muzzle on the middle of the room, then his corner, then his wall.
fn room_clear_enter(g: &mut crate::game::Game, id: RoomClearId, t: f64) {
    let row = &mut g.tab.squad_room_clear[id];
    row.t_enter = t;
    row.phase = RoomClearPhase::Enter;
    let (entry, face, aim, list) = (row.plan.entry[0].clone(), row.plan.pod_face[0], row.aim_a, row.plan.entry_aim[0].clone());
    route_man(g, id, false, &entry, OrderPatch { aim_at: Some(Some(aim)), speed: Some(Some(2.3)), face: Some(Some(face)), strafe: Some(Some(false)), stance: Some(Some(crate::character::Stance::None)), upper: Some(Some(crate::character::UpperMode::Aim)), ..OrderPatch::default() });
    let rest = enter_sweep(g, id, false, aim, &list, t);
    g.tab.squad_room_clear[id].rest_a = rest;
}

/// enterSweep for a drill's own man: `second` picks swB.
fn enter_sweep(g: &mut Game, id: RoomClearId, second: bool, aim: V3, list: &[V3], t: f64) -> Option<Vec<V3>> {
    let row = &mut g.tab.squad_room_clear[id];
    let centre = room_centre(&row.plan);
    sweep_start(if second { &mut row.sw_b } else { &mut row.sw_a }, aim, &[centre, list[0]], t, centre_check().corner);
    if list.len() > 1 { Some(list[1..].to_vec()) } else { None }
}

/// The keyed breach: the leaf unlatched and shoved hard into the room, and he flows in behind it.
fn room_clear_shove(g: &mut crate::game::Game, id: RoomClearId, t: f64) {
    let row = &g.tab.squad_room_clear[id];
    let (d, fling, a) = (row.door.unwrap(), row.plan.fling, row.a);
    let c = g.gd[a].ch;
    let s = js::or(m::sign(fling), -1.0);
    if !g.dr[d].leaf.off && crate::doors::doors_open_for(g, d) < 1.0 {
        g.dr[d].leaf.latched = false;
        g.dr[d].leaf.closing = false;
        let v = g.dr[d].leaf.vel;
        g.dr[d].leaf.vel = s * m::max(m::abs(v), m::abs(fling));
        g.dr[d].leaf.quiet_t = -1.0;
        g.dr[d].leaf.creak_armed = false;
        let who = g.ch[c].id;
        g.dr[d].leaf.last_who = who;
        let at = crate::doors::door_pos(g, d);
        crate::audio_iface::game_audio_like_play(g, "doorBash", Some(at), Some(0.5), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.15 }));
    }
    let row = &mut g.tab.squad_room_clear[id];
    row.t_kick = t;
    row.flung = true;
    room_clear_say(g, id, RoomClearSayKey::In1, a, None);
    room_clear_enter(g, id, t);
}

/// What a phase of the drill works from, this beat.
#[derive(Clone, Copy)]
struct Beat {
    id: RoomClearId,
    t: f64,
    a: GuardId,
    b: Option<GuardId>,
    ca: CharId,
    cb: Option<CharId>,
    d: DoorId,
    live: bool,
    kick_from: V3,
    kick_face: f64,
    stack_a: V3,
    fling: f64,
    /// the plan's breach is a kick
    kick: bool,
    /// #1 is planted on the end of his route; a new route takes it off him
    a_in: bool,
    b_in: bool,
}

/// One beat of the drill, on the caller's clock: the men's routes and muzzles go on, then whichever phase it is in.
pub(super) fn room_clear_tick(g: &mut crate::game::Game, id: RoomClearId, t: f64, dt: f64) {
    let (a, b, door) = { let row = &g.tab.squad_room_clear[id]; (row.a, row.b, row.door) };
    let Some(d) = door else { return };
    if !room_clear_intact(g, id) { return; }
    g.tab.squad_room_clear[id].t_now = t;
    let a_in = advance_man(g, id, false);
    let b_in = if b.is_some() { advance_man(g, id, true) } else { true };
    muzzle_sweeps_on(g, id, false, Some(a), t);
    muzzle_sweeps_on(g, id, true, b, t);
    let row = &g.tab.squad_room_clear[id];
    let p = &row.plan;
    let kick = p.breach.unwrap_or(crate::scene_level::PlanBreach::Kick) == crate::scene_level::PlanBreach::Kick;
    let (ca, cb) = (g.gd[a].ch, b.map(|x| g.gd[x].ch));
    let mut beat = Beat { id, t, a, b, ca, cb, d, live: row.live, kick_from: p.kick_from, kick_face: p.kick_face, stack_a: p.stack[0], fling: p.fling, kick, a_in, b_in };
    match row.phase {
        RoomClearPhase::Stack => on_the_stack(g, &mut beat),
        RoomClearPhase::Breach => at_the_breach(g, &mut beat, dt),
        RoomClearPhase::Kick => door_kicked(g, &mut beat),
        RoomClearPhase::Enter => going_in(g, &mut beat),
        RoomClearPhase::Search => searching(g, &mut beat),
        RoomClearPhase::Resolve => calling_it(g, &mut beat),
    }
    let row = &mut g.tab.squad_room_clear[id];
    row.a_home = beat.a_in;
    row.b_home = beat.b_in;
}

/// A man's sweep moves the drill's aim point on, and his script with it (`gd`: the man, if there is one).
fn muzzle_sweeps_on(g: &mut Game, id: RoomClearId, second: bool, gd: Option<GuardId>, t: f64) {
    let row = &mut g.tab.squad_room_clear[id];
    let (sw, aim) = if second { (&mut row.sw_b, &mut row.aim_b) } else { (&mut row.sw_a, &mut row.aim_a) };
    if !sw.active { return; }
    let was = *aim;
    let mut out = was;
    sweep_at(sw, t, &mut out);
    *aim = out;
    if let Some(gd) = gd { aim_moved(g, gd, was, out); }
}

/// On the stack: #1 says so, #2 squeezes his shoulder and says so; on the squeeze they go.
fn on_the_stack(g: &mut Game, beat: &mut Beat) {
    if beat.t > 0.6 { room_clear_say(g, beat.id, RoomClearSayKey::Stack1, beat.a, None); }
    let squeeze = if beat.live { squeeze_when_set(g, beat) } else { squeeze_on_the_clock(g, beat) };
    if squeeze { moves_to_the_door(g, beat); }
}

/// A staged drill goes by the clock.
fn squeeze_on_the_clock(g: &mut Game, beat: &Beat) -> bool {
    let Beat { id, t, b, cb, b_in, .. } = *beat;
    if let Some(cb) = cb {
        if t > 1.2 && t < 1.6 && b_in && room_clear_once(g, id, "#hold") { crate::character::anim_signal(g, cb, crate::character::SignalKind::Squeeze, None); }
    }
    if let Some(bb) = b { if t > 1.6 { room_clear_say(g, id, RoomClearSayKey::Stack2, bb, None); } }
    t >= 2.5 && match cb { None => true, Some(cb) => !crate::character::anim_signalling(g, cb) || t > 2.9 }
}

/// A live drill goes when both men are in, or have been too long about it.
fn squeeze_when_set(g: &mut Game, beat: &Beat) -> bool {
    let Beat { id, t, b, cb, a_in, b_in, .. } = *beat;
    if g.tab.squad_room_clear[id].t_set.is_nan() && ((a_in && b_in) || t > STACK_CAP) {
        g.tab.squad_room_clear[id].t_set = t;
        if let Some(cb) = cb { if b_in && room_clear_once(g, id, "#hold") { crate::character::anim_signal(g, cb, crate::character::SignalKind::Squeeze, None); } }
    }
    if let Some(bb) = b { if (b_in || t > 2.2) && t > 1.0 { room_clear_say(g, id, RoomClearSayKey::Stack2, bb, None); } }
    let t_set = g.tab.squad_room_clear[id].t_set;
    !t_set.is_nan() && t >= m::max(2.5, t_set + 1.0) && match cb { None => true, Some(cb) => !crate::character::anim_signalling(g, cb) || t > t_set + 1.5 }
}

/// #2 gives the word and #1 goes to his mark, unless the door is open already and there is nothing to kick from.
fn moves_to_the_door(g: &mut Game, beat: &mut Beat) {
    let Beat { id, t, b, cb, d, kick_from, kick_face, kick, .. } = *beat;
    if let (Some(bb), Some(cb)) = (b, cb) {
        crate::character::anim_signal(g, cb, crate::character::SignalKind::Go, None);
        room_clear_say(g, id, RoomClearSayKey::Go2, bb, None);
    }
    let no_mark = !kick && !crate::doors::door_is_closed(g, d);
    let row = &mut g.tab.squad_room_clear[id];
    row.t_go = t;
    row.phase = RoomClearPhase::Breach;
    row.no_mark = no_mark;
    if no_mark { return; }
    route_man(g, id, false, &[kick_from], OrderPatch {
        speed: Some(Some(1.2)), face: Some(Some(kick_face)), stance: Some(Some(crate::character::Stance::HighReady)), upper: Some(Some(crate::character::UpperMode::None)),
        ..OrderPatch::default()
    });
    beat.a_in = false;
}

/// #1 gets planted on his mark while #2 brings his muzzle round to the door; then the kick, or the shove.
fn at_the_breach(g: &mut Game, beat: &mut Beat, dt: f64) {
    let Beat { id, t, a, b, ca, d, live, kick_from, kick_face, kick, a_in, .. } = *beat;
    let (p, yaw) = (g.ch[ca].pos, g.ch[ca].body_yaw);
    let planted = a_in && dist_xz(p, kick_from) < 0.07 && m::abs(v3::wrap_angle(yaw - kick_face)) < 0.1;
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
    let due = if no_mark { t > t0 + 0.45 } else { (planted && t > t0 + 0.5) || t > t0 + 1.4 };
    if !due { return; }
    if !kick {
        room_clear_shove(g, id, t);
        beat.a_in = false;
        return;
    }
    order(g, a, OrderPatch { goal: Some(None), aim_at: Some(None), ..OrderPatch::default() });
    crate::character::anim_kick_door(g, ca, None);
    let row = &mut g.tab.squad_room_clear[id];
    row.t_kick = t;
    row.phase = RoomClearPhase::Kick;
}

/// The boot lands and the leaf flies; when the kick is over he goes in.
fn door_kicked(g: &mut Game, beat: &mut Beat) {
    let Beat { id, t, ca, .. } = *beat;
    if !g.tab.squad_room_clear[id].flung
        && (crate::character::anim_kick_impact(g, ca) || crate::character::anim_kick_time(g, ca) >= crate::character::KICK_IMPACT || !crate::character::anim_kicking(g, ca))
    {
        boot_lands(g, beat);
    }
    let (flung, t_kick) = { let row = &g.tab.squad_room_clear[id]; (row.flung, row.t_kick) };
    if flung && (!crate::character::anim_kicking(g, ca) || t > t_kick + 1.3) {
        room_clear_enter(g, id, t);
        beat.a_in = false;
    }
}

/// The leaf is flung, in full if he stands square on it and it can be heard and seen; he says the word and steps back to let it swing.
fn boot_lands(g: &mut Game, beat: &mut Beat) {
    let Beat { id, a, ca, d, kick_from, kick_face, stack_a, fling, .. } = *beat;
    g.tab.squad_room_clear[id].flung = true;
    let (p, yaw) = (g.ch[ca].pos, g.ch[ca].body_yaw);
    let on_it = dist_xz(p, kick_from) < 0.4 && m::abs(v3::wrap_angle(yaw - kick_face)) < 0.35;
    if m::abs(g.dr[d].leaf.angle) < 1.4 {
        g.dr[d].leaf.latched = false;
        g.dr[d].leaf.closing = false;
        g.dr[d].leaf.vel = if on_it { fling } else { fling * 0.32 };
        if on_it { g.dr[d].leaf.since_bash = 0.0; }
        g.dr[d].leaf.quiet_t = -0.5;
        g.dr[d].leaf.creak_armed = false;
    }
    let who = g.ch[ca].id;
    g.dr[d].leaf.last_who = who;
    if on_it && !g.dr[d].leaf.off {
        let at = crate::doors::door_pos(g, d);
        crate::audio_iface::game_audio_like_play(g, "doorBash", Some(at), Some(1.0), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.9 }));
        crate::marks::kick_scuff(g, d, p);
        crate::particle_fx::kick_dust_fx(g, d, p);
    }
    room_clear_say(g, id, RoomClearSayKey::In1, a, None);
    let k = m::min(1.0, KICK_BACK / m::max(1e-3, dist_xz(kick_from, stack_a)));
    route_man(g, id, false, &[v3::lerp3(kick_from, stack_a, k)], OrderPatch {
        aim_at: Some(None), speed: Some(Some(1.6)), face: Some(Some(kick_face)), strafe: Some(Some(true)), stance: Some(Some(crate::character::Stance::HighReady)), upper: Some(Some(crate::character::UpperMode::None)),
        ..OrderPatch::default()
    });
    beat.a_in = false;
}

/// #1 is through; #2 follows; each finishes his sweep and calls his side; then #1 goes on to search.
fn going_in(g: &mut Game, beat: &mut Beat) {
    let Beat { id, t, a, b, .. } = *beat;
    let te = t - g.tab.squad_room_clear[id].t_enter;
    second_man_goes_in(g, beat, te);
    sweep_rest(g, id, false, t);
    if b.is_some() { sweep_rest(g, id, true, t); }
    if beat.a_in && te > 1.5 { room_clear_say(g, id, RoomClearSayKey::Side1, a, None); }
    if let Some(bb) = b { if g.tab.squad_room_clear[id].b_go && beat.b_in && te > 1.5 { room_clear_say(g, id, RoomClearSayKey::Side2, bb, None); } }
    if te > 3.4 { search_begins(g, beat); }
}

/// #2 goes when #1 is far enough through, a moment on; once.
fn second_man_goes_in(g: &mut Game, beat: &mut Beat, te: f64) {
    let Beat { id, t, ca, live, kick_from, .. } = *beat;
    let Some(cb) = beat.cb else { return };
    let pa = g.ch[ca].pos;
    let go = !g.tab.squad_room_clear[id].b_go
        && if live { te > 0.85 && dist_xz(pa, g.ch[cb].pos) > 1.5 } else { te > 0.85 || dist_xz(pa, kick_from) > 1.6 };
    if !go { return; }
    let row = &mut g.tab.squad_room_clear[id];
    row.b_go = true;
    let (entry, face, aim, list) = (row.plan.entry[1].clone(), row.plan.pod_face[1], row.aim_b, row.plan.entry_aim[1].clone());
    route_man(g, id, true, &entry, OrderPatch {
        speed: Some(Some(2.2)), face: Some(Some(face)), stance: Some(Some(crate::character::Stance::None)), upper: Some(Some(crate::character::UpperMode::Aim)),
        ..OrderPatch::default()
    });
    beat.b_in = false;
    let rest = enter_sweep(g, id, true, aim, &list, t);
    g.tab.squad_room_clear[id].rest_b = rest;
}

/// #1 walks the search path, his muzzle along it.
fn search_begins(g: &mut Game, beat: &mut Beat) {
    let Beat { id, t, .. } = *beat;
    let row = &mut g.tab.squad_room_clear[id];
    row.t_search = t;
    row.phase = RoomClearPhase::Search;
    row.rest_a = None;
    let (path, face, walk) = (row.plan.search_path.clone(), row.plan.search_face, row.plan.search_aim_walk.clone());
    route_man(g, id, false, &path, OrderPatch { speed: Some(Some(1.3)), face: Some(Some(face)), upper: Some(Some(crate::character::UpperMode::Aim)), ..OrderPatch::default() });
    beat.a_in = false;
    sweep_begin(g, id, false, &walk, t, 3.0);
}

/// #1 pushes through while #2 kneels and looks under things, stands and takes the next door; then #1 calls his corner and the phase is over.
fn searching(g: &mut Game, beat: &mut Beat) {
    let Beat { id, t, a, b, a_in, .. } = *beat;
    let ts = t - g.tab.squad_room_clear[id].t_search;
    if b.is_some() { sweep_rest(g, id, true, t); }
    if ts > 0.35 && b.is_some() { room_clear_say(g, id, RoomClearSayKey::Push1, a, None); }
    if let Some(bb) = b { looks_under_things(g, beat, bb, ts); }
    if ts > 3.0 && (a_in || ts > 4.0) && room_clear_once(g, id, "#across") {
        let there = g.tab.squad_room_clear[id].plan.search_aim_there.clone();
        sweep_begin(g, id, false, &there, t, 2.4);
    }
    if let Some(bb) = b { stands_for_the_next_door(g, beat, bb, ts); }
    if ts > 4.1 && (a_in || ts > 4.5) { room_clear_say(g, id, RoomClearSayKey::Corner1, a, None); }
    if ts > 4.7 {
        let row = &mut g.tab.squad_room_clear[id];
        row.t_resolve = t;
        row.phase = RoomClearPhase::Resolve;
    }
}

/// When his entry sweep is done #2 kneels and sweeps under things, and says what he is doing.
fn looks_under_things(g: &mut Game, beat: &Beat, bb: GuardId, ts: f64) {
    let Beat { id, t, .. } = *beat;
    if ts > 0.5 && g.tab.squad_room_clear[id].rest_b.is_none() && room_clear_once(g, id, "#kneel") {
        order(g, bb, OrderPatch { crouch: Some(Some(true)), ..OrderPatch::default() });
        let under = g.tab.squad_room_clear[id].plan.under_aim.clone();
        sweep_begin(g, id, true, &under, t, 3.2);
    }
    if ts > 0.9 { room_clear_say(g, id, RoomClearSayKey::Under2, bb, None); }
}

/// Nothing under there: #2 says so and stands, on the room's next door if it has one.
fn stands_for_the_next_door(g: &mut Game, beat: &Beat, bb: GuardId, ts: f64) {
    let Beat { id, t, .. } = *beat;
    if ts > 3.4 { room_clear_say(g, id, RoomClearSayKey::Nothing2, bb, None); }
    if !(ts > 3.7 && room_clear_once(g, id, "#standup")) { return; }
    let row = &g.tab.squad_room_clear[id];
    let (nd, up_aim) = (row.next_door, row.plan.up_aim);
    let Some(nd) = nd else {
        order(g, bb, OrderPatch { crouch: Some(Some(false)), ..OrderPatch::default() });
        sweep_begin(g, id, true, &[up_aim], t, 1.2);
        return;
    };
    let fc = g.dr[nd].frame_centre;
    let pb = g.pos_of(bb);
    order(g, bb, OrderPatch { crouch: Some(Some(false)), face: Some(Some(v3::yaw_to(pb, fc))), ..OrderPatch::default() });
    sweep_begin(g, id, true, &[[fc[0], 1.0, fc[2]]], t, 1.2);
}

/// The room is called empty and radioed in, guns come down, and they form up on the door.
fn calling_it(g: &mut Game, beat: &mut Beat) {
    let Beat { id, t, a, b, ca, .. } = *beat;
    let tr = t - g.tab.squad_room_clear[id].t_resolve;
    room_clear_say(g, id, RoomClearSayKey::Empty2, b.unwrap_or(a), None);
    if tr > 0.9 { room_clear_say(g, id, RoomClearSayKey::Radio1, a, Some(true)); }
    if tr > 1.0 && room_clear_once(g, id, "#lower") {
        order(g, a, OrderPatch { stance: Some(Some(crate::character::Stance::LowReady)), upper: Some(Some(crate::character::UpperMode::None)), ..OrderPatch::default() });
        if let Some(bb) = b { order(g, bb, OrderPatch {
            stance: Some(Some(crate::character::Stance::LowReady)), upper: Some(Some(crate::character::UpperMode::None)), crouch: Some(Some(false)),
            ..OrderPatch::default()
        }); }
    }
    if b.is_some() && tr > 1.15 && room_clear_once(g, id, "#rally") { crate::character::anim_signal(g, ca, crate::character::SignalKind::Rally, None); }
    if tr > 1.3 && room_clear_once(g, id, "#formup") { form_up(g, beat); }
}

/// Each man walks to his form-up mark with his muzzle out ahead of where he will stand.
fn form_up(g: &mut Game, beat: &mut Beat) {
    let Beat { id, t, b, .. } = *beat;
    let row = &g.tab.squad_room_clear[id];
    let (fa, fb, ff) = (row.plan.form_up[0].clone(), row.plan.form_up[1].clone(), row.plan.form_face);
    let ea = fa[fa.len() - 1];
    route_man(g, id, false, &fa, OrderPatch {
        speed: Some(Some(1.8)), face: Some(Some(ff[0])), stance: Some(Some(crate::character::Stance::LowReady)), upper: Some(Some(crate::character::UpperMode::None)),
        ..OrderPatch::default()
    });
    beat.a_in = false;
    sweep_begin(g, id, false, &[room_clear_ahead(ea, ff[0], 3.0, 0.4)], t, 2.0);
    if b.is_none() { return; }
    route_man(g, id, true, &fb, OrderPatch {
        speed: Some(Some(1.2)), face: Some(Some(ff[1])), stance: Some(Some(crate::character::Stance::LowReady)), upper: Some(Some(crate::character::UpperMode::None)),
        ..OrderPatch::default()
    });
    beat.b_in = false;
    let eb = fb[fb.len() - 1];
    sweep_begin(g, id, true, &[room_clear_ahead(eb, ff[1], 3.0, 0.4)], t, 1.5);
}

/// Seconds for the rest of an entry sweep: what is left of the old three-second sweep, never under minLeg a leg.
fn rest_secs(rest: &[V3]) -> f64 {
    let c = centre_check();
    m::max(c.min_leg * rest.len() as f64, c.total - c.corner)
}

/// The middle of the room at muzzle height: the plan's bounds, else the centroid of both men's entry sweeps.
fn room_centre(p: &crate::scene_level::RoomClearPlan) -> V3 {
    if let Some(b) = p.bounds { return [(b.x0 + b.x1) / 2.0, 1.0, (b.z0 + b.z1) / 2.0]; }
    let (mut x, mut z) = (0.0, 0.0);
    for q in p.entry_aim[0].iter().chain(p.entry_aim[1].iter()) { x += q[0]; z += q[2]; }
    let n = (p.entry_aim[0].len() + p.entry_aim[1].len()) as f64;
    [x / n, 1.0, z / n]
}

/// Another door of the room the plan clears, for #2 to hold: the nearest to his point of domination, or None.
fn next_door_of(g: &crate::game::Game, p: &crate::scene_level::RoomClearPlan) -> Option<DoorId> {
    let b = p.bounds?;
    let pod = match p.entry[1].last() { Some(x) => *x, None => p.stack[1] };
    let mut best = None;
    let mut bd = f64::INFINITY;
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if g.dr[d].def.name == p.door { continue; }
        let fc = g.dr[d].frame_centre;
        if fc[0] < b.x0 - 0.3 || fc[0] > b.x1 + 0.3 || fc[2] < b.z0 - 0.3 || fc[2] > b.z1 + 0.3 { continue; }
        let dd = dist_xz(fc, pod);
        if dd < bd { bd = dd; best = Some(d); }
    }
    best
}

/// Break a scripted man off into the AI with something on his mind: patrol for exactly one frame carrying this awareness and fix.
pub(super) fn break_off(g: &mut crate::game::Game, gd: GuardId, awareness: f64, last_known: Option<V3>) {
    end_guard_script(g, gd, Some(crate::game::GuardState::Suspicious));
    if crate::condition::down(g, g.gd[gd].ch) { return; }
    crate::mind::set_state(g, gd, crate::game::GuardState::Patrol, "squad");
    crate::mind::raise(g, gd, awareness, "squad");
    if let Some(k) = last_known { crate::mind::fix_keep(g, gd, Some(v3::copy(k)), "squad"); }
    g.gd[gd].react_t = 0.0;
    g.gd[gd].search_t = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each leg gets the same time. A failure means a man's muzzle jumps from corner to corner, or stops short of the last.
    #[test]
    fn a_sweep_eases_from_corner_to_corner_in_even_beats_and_stops_on_the_last() {
        let (from, a, b, c): (V3, V3, V3, V3) = ([0.0, 1.0, 0.0], [2.0, 1.0, 0.0], [2.0, 1.0, 2.0], [0.0, 1.0, 2.0]);
        let mut s = Sweep::blank();
        sweep_start(&mut s, from, &[a, b, c], 10.0, 3.0);
        let mut out = [9.0, 9.0, 9.0];
        assert!(sweep_at(&mut s, 10.0, &mut out));
        assert_eq!(out, from);
        assert!(sweep_at(&mut s, 11.0, &mut out));
        assert_eq!(out, a);
        assert!(sweep_at(&mut s, 11.5, &mut out));
        assert_eq!(out, [2.0, 1.0, 1.0]);       // half way along its leg at half its time
        assert!(sweep_at(&mut s, 12.25, &mut out));
        assert!(out[0] > 1.5 && out[2] == 2.0, "a quarter of the time is less than a quarter of the way: it starts slow, {out:?}");
        assert!(!sweep_at(&mut s, 13.0, &mut out));
        assert_eq!(out, c);
        assert!(!s.active);
        out = [9.0, 9.0, 9.0];
        assert!(!sweep_at(&mut s, 15.0, &mut out));
        assert_eq!(out, [9.0, 9.0, 9.0], "a finished sweep moves nothing");
    }

    #[test]
    fn a_sweep_with_no_points_is_not_active_and_leaves_out_alone() {
        let mut s = Sweep::blank();
        sweep_start(&mut s, [1.0, 2.0, 3.0], &[], 0.0, 1.0);
        let mut out = [9.0, 9.0, 9.0];
        assert!(!sweep_at(&mut s, 0.5, &mut out));
        assert_eq!(out, [9.0, 9.0, 9.0]);
    }

    /// A facing of 0 is +z, the level's own way of saying a yaw.
    #[test]
    fn a_point_ahead_is_along_his_facing_at_the_height_asked() {
        assert_eq!(room_clear_ahead([1.0, 0.0, 2.0], 0.0, 3.0, 0.4), [1.0, 0.4, 5.0]);
        let r = room_clear_ahead([1.0, 0.0, 2.0], m::PI / 2.0, 3.0, 0.4);
        assert!((r[0] - 4.0).abs() < 1e-12 && (r[2] - 2.0).abs() < 1e-12, "{r:?}");
    }

    /// A man with no script gets his own, with its own serial; a script he has keeps it. What is a new leg: any goal not `held`, even at the same spot; a held one by value; null only against a goal.
    #[test]
    fn order_deals_a_serial_once_and_says_what_is_a_new_leg() {
        let mut g = Game::blank();
        let (x, y) = (g.gd.push(crate::game::Guard::blank()), g.gd.push(crate::game::Guard::blank()));
        let goal = |at: V3, held: bool| OrderPatch { goal: Some(Some(at)), held, ..OrderPatch::default() };
        order(&mut g, x, OrderPatch::default());
        order(&mut g, y, OrderPatch::default());
        let first = g.gd[x].script.unwrap().serial;
        assert_ne!(first, g.gd[y].script.unwrap().serial);
        let (a, b) = ([1.0, 0.0, 2.0], [3.0, 0.0, 4.0]);
        let new_leg = |g: &mut Game, patch: OrderPatch| {
            g.gd[x].stuck_t = 7.0;
            order(g, x, patch);
            g.gd[x].stuck_t == 0.0
        };
        assert!(!new_leg(&mut g, OrderPatch { goal: Some(None), ..OrderPatch::default() }), "null over null");
        assert!(new_leg(&mut g, goal(a, false)), "first goal");
        assert!(new_leg(&mut g, goal(a, false)), "a fresh goal at the same spot");
        assert!(!new_leg(&mut g, goal(a, true)), "held, not moved");
        assert!(new_leg(&mut g, goal(b, true)), "held, moved");
        assert!(!new_leg(&mut g, OrderPatch { crouch: Some(Some(true)), ..OrderPatch::default() }), "no goal in the patch");
        assert!(new_leg(&mut g, OrderPatch { goal: Some(None), ..OrderPatch::default() }), "null over a goal");
        assert_eq!(g.gd[x].script.unwrap().serial, first);
    }

    /// What is left of a three-second sweep, and never under a leg's own time for each leg.
    #[test]
    fn rest_secs_floors_at_a_leg_each() {
        assert_eq!(rest_secs(&[[0.0; 3]; 3]), 3.0 - 0.95);
        assert_eq!(rest_secs(&[[0.0; 3]; 4]), 0.6 * 4.0);
    }
}
