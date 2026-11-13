//! WHICH WAY HE FACES: his body, his aim and his gun's pitch, by what has hold of him: a body in tow, a run, a wall, or his own feet.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3;

pub(super) fn face(g: &mut Game, t: Tick, go: Go, a: Aim) {
    let pl = t.pl;
    if g.pl[pl].tow.is_some() { tow_tick(g, t, a.speed); }
    else if g.pl[pl].sprinting { face_run(g, t); }
    else if on_stack(g, pl) { face_along_wall(g, t, a); }
    else if let Some(w) = g.pl[pl].wall { face_off_wall(g, t, w); }
    else { face_cursor(g, t, go, a); }
}

/// A run: gun down, the body goes the way he runs, and a door run open turns his head.
fn face_run(g: &mut Game, t: Tick) {
    let Tick { pl, c, dt, .. } = t;
    let lean = own_legs(g, pl, |g, m| sprint_facing(g, m, dt, None));
    let time = g.time;
    let ch = &mut g.ch[c];
    ch.aim_pitch = v3::damp(ch.aim_pitch, -0.3, 6.0, dt);
    ch.anim.reverse = false;
    if lean.is_none() { return; }
    let head = crate::sprintdoor::sprint_door_head(ch.body_yaw - v3::yaw_of(ch.vel));
    ch.anim.look_yaw_extra = head;
    g.pl[pl].door_head = head;
    g.pl[pl].door_head_t = time;
}

/// Stacked on a wall with the rifle: he faces along it, turned up to 45 degrees off that line to bring the rifle onto the cursor, and comes
/// round to the other way along it when the cursor stays behind him.
fn face_along_wall(g: &mut Game, t: Tick, a: Aim) {
    let Tick { pl, c, dt, .. } = t;
    let Some(w) = g.pl[pl].wall else { return };
    let along = m::atan2(w.n[0], w.n[2]) - w.side * m::PI / 2.0;
    let moving = m::abs(w.v) > 0.3;
    let (mut side, mut flip_t, mut body) = (w.side, 0.0, along);
    if a.aiming && a.dist > 0.25 {
        aim_at_cursor(g, t, a);
        let off = v3::wrap_angle(v3::yaw_of(a.to) - along);
        if m::abs(off) > 110.0 * v3::DEG && !moving && !w.lean.map_or(false, |l| l > 0.0) { flip_t = w.flip_t.unwrap_or(0.0) + dt; }
        if flip_t > 0.2 { side = -side; flip_t = 0.0; }
        body = along + v3::clamp(v3::wrap_angle(g.ch[c].aim_yaw - along), -0.9, 0.9) * 0.85;
    } else {
        look_toward(g, c, along, 10.0, dt);
    }
    if let Some(wall) = g.pl[pl].wall.as_mut() { wall.flip_t = Some(flip_t); wall.side = side; }
    if !wall_take_yaw(g, pl) { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, body, 10.0 * dt); }
    g.pl[pl].backpedal = false;
    g.ch[c].anim.reverse = moving && w.v * side < 0.0;
}

/// His back to a wall: flat on it when he is still, turned toward the way he goes as he sidles, looking out past its end at an edge.
fn face_off_wall(g: &mut Game, t: Tick, w: crate::player::WallHold) {
    let Tick { pl, c, dt, .. } = t;
    let away = m::atan2(w.n[0], w.n[2]);
    let moving = m::abs(w.v) > 0.3;
    let pace = if g.pl[pl].crouch { g.k.tune.player_crouch } else { crate::player::WALL_PACE };
    let turn = v3::smoothstep(0.1, 0.9, m::abs(w.v) / m::max(0.3, pace));
    if !wall_take_yaw(g, pl) { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, away - w.side * crate::player::WALL_TURN * turn, 10.0 * dt); }
    let out = if w.at_edge == w.side { crate::player::WALL_LOOK } else if moving { 15.0 * v3::DEG } else { 8.0 * v3::DEG };
    look_toward(g, c, away - w.side * out, 8.0, dt);
    g.pl[pl].backpedal = false;
    g.ch[c].anim.reverse = moving && w.v * w.side < 0.0;
}
/// On a wall with nothing raised: his eyes come round to a heading.
fn look_toward(g: &mut Game, c: CharId, yaw: f64, rate: f64, dt: f64) {
    let ch = &mut g.ch[c];
    ch.aim_yaw += v3::wrap_angle(yaw - ch.aim_yaw) * (1.0 - m::exp(-rate * dt));
    ch.aim_pitch = v3::damp(ch.aim_pitch, 0.05, 6.0, dt);
}

/// On his own feet: his aim follows the cursor and his hips follow his feet, or a leaf he pushes open.
fn face_cursor(g: &mut Game, t: Tick, go: Go, a: Aim) {
    let Tick { pl, c, dt, .. } = t;
    let orbits = go.stalk.is_some();
    let swung = !orbits && crate::aimweight::aim_weigh_turn(g, dt);   // (a gun's weight: its heading comes round to the cursor's at a rate)
    if !swung && a.dist > 0.25 { g.ch[c].aim_yaw = aim_ease(g.ch[c].aim_yaw, a.to, dt); }
    pitch_to_cursor(g, t, a);
    let push = g.ch[c].anim.door_push.clone();
    let by_hand = g.pl[pl].door_move().is_some() && push.as_ref().map_or(false, |d| d.w >= 1.0) && !a.aiming;
    let into_door = push.as_ref().and_then(|d| {
        if by_hand { Some(v3::yaw_to(g.ch[c].pos, d.p)) }
        else { leaf_facing(Some(&crate::player::LeafFacingDp { w: d.w, n: d.n.to_vec() }), a.aiming) }
    });
    own_legs(g, pl, |_, m| hips_turn(m, a.speed, a.aiming, orbits, into_door, by_hand, dt));
    g.ch[c].anim.reverse = g.pl[pl].backpedal;
}

fn aim_at_cursor(g: &mut Game, t: Tick, a: Aim) {
    g.ch[t.c].aim_yaw = aim_ease(g.ch[t.c].aim_yaw, a.to, t.dt);
    pitch_to_cursor(g, t, a);
}
fn pitch_to_cursor(g: &mut Game, t: Tick, a: Aim) {
    let pitch = cursor_pitch(g, t, a);
    let ch = &mut g.ch[t.c];
    ch.aim_pitch = v3::damp(ch.aim_pitch, pitch, 12.0, t.dt);
}
/// The gun's pitch onto what the reticle has, else level along the cursor's heading.
fn cursor_pitch(g: &Game, t: Tick, a: Aim) -> f64 {
    let seat = crate::seats::here(g);
    let y = if let Some(f) = seat.aim_target { v3::clamp(g.tab.fixture[f].pos[1], 0.3, 2.95) }
        else if let Some(gd) = seat.aim_guard { man_aim(g, &seat, gd, 1.2)[1] }
        else if level_on(g) { return 0.0; }
        else { a.at[1] + 0.03 };
    let gun_y = g.ch[t.c].pos[1] + (if g.pl[t.pl].crouch { 0.95 } else { 1.4 });
    v3::clamp(m::atan2(y - gun_y, m::max(a.dist, 0.5)), -60.0 * v3::DEG, 70.0 * v3::DEG)
}
