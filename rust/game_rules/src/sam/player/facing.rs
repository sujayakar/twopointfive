//! WHICH WAY HE FACES: his body, his aim and his gun's pitch, by what has hold of him: a body in tow, a run, a wall, or his own feet.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};

pub(super) fn face(g: &mut Game, t: Tick, go: Go, a: Aim) {
    let pl = t.pl;
    if g.pl[pl].tow.is_some() { tow_tick(g, t, a.speed); }
    else if g.pl[pl].sprinting { face_run(g, t); }
    else if on_stack(g, pl) { face_along_wall(g, t, a); }
    else if let Some(w) = g.pl[pl].wall { face_off_wall(g, t, w); }
    else { face_cursor(g, t, go, a); }
}

/// A body in tow: pinned at his hands by the ankles, and he faces its hips; torn out of his hands if it wedges.
fn tow_tick(g: &mut Game, t: Tick, speed: f64) {
    let Tick { pl, c, dt, .. } = t;
    let Some(mut tow) = g.pl[pl].tow else { return };
    let body = g.gd[tow.who].ch;
    let rag = g.ch[body].ragdoll;
    let hips = g.ch[body].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[body].pos);
    let (dx, dz) = (hips[0] - g.ch[c].pos[0], hips[2] - g.ch[c].pos[2]);
    let d = m::hyp2(dx, dz);
    if d > 1e-4 { tow.yaw = m::atan2(dx, dz); }
    let strained = match rag { Some(r) => g.tab.ragdoll[r].strain > 0.2 || crate::hands::haul_slack(g), None => false };
    tow.stuck_t = if strained { tow.stuck_t + dt } else { 0.0 };
    g.pl[pl].tow = Some(tow);
    match rag {
        Some(rag) if d <= 2.2 && tow.stuck_t <= 0.25 => {
            if crate::hands::haul_set_done(g) { release_body(g, Some(true)); return; }
            if !crate::hands::haul_pins(g, dt) { pin_ankles(g, c, rag); }
            face_body(g, t, tow.yaw, speed);
            scrape(g, t, rag, hips, speed);
        }
        _ => crate::game::game_toggle_drag(g, None),
    }
}
fn pin_ankles(g: &mut Game, c: CharId, rag: RagdollId) {
    let f = crate::character::character_forward(g, c);
    let (hx, hz) = (g.ch[c].pos[0] + f[0] * 0.35, g.ch[c].pos[2] + f[2] * 0.35);
    let bones = crate::ragdoll::rag();
    crate::ragdoll::ragdoll_set_pin(g, rag, bones.ank_l, [hx + f[2] * 0.1, 0.42, hz - f[0] * 0.1]);
    crate::ragdoll::ragdoll_set_pin(g, rag, bones.ank_r, [hx - f[2] * 0.1, 0.42, hz + f[0] * 0.1]);
}
/// With it on his shoulder he faces the way he walks; hauling or dragging it he faces it, and backs off.
fn face_body(g: &mut Game, t: Tick, yaw: f64, speed: f64) {
    let Tick { pl, c, dt, .. } = t;
    if crate::hands::haul_upright(g, &crate::seats::here(g)) {
        let turn = g.k.hands.carry_turn;
        let ch = &mut g.ch[c];
        if speed > 0.2 { ch.body_yaw = v3::approach_angle(ch.body_yaw, v3::yaw_of(ch.vel), turn * dt); }
        ch.aim_yaw = ch.body_yaw;
        ch.aim_pitch = v3::damp(ch.aim_pitch, 0.0, 6.0, dt);
        g.pl[pl].backpedal = false;
        g.ch[c].anim.reverse = false;
        return;
    }
    let face_yaw = crate::hands::haul_yaw(g, &crate::seats::here(g)).unwrap_or(yaw);
    let hauling = g.pl[pl].haul().is_some();
    let ch = &mut g.ch[c];
    ch.aim_pitch = v3::damp(ch.aim_pitch, if hauling { -0.45 } else { -0.9 }, 6.0, dt);
    let rel = v3::approach_angle(v3::wrap_angle(ch.aim_yaw - ch.body_yaw), 0.0, 8.0 * dt);
    ch.body_yaw = v3::approach_angle(ch.body_yaw, face_yaw, 10.0 * dt);
    ch.aim_yaw = if hauling { ch.body_yaw + rel } else { face_yaw };
    let back = speed > 0.2 && m::cos(v3::wrap_angle(face_yaw - v3::yaw_of(ch.vel))) < 0.0;
    g.pl[pl].backpedal = back;
    g.ch[c].anim.reverse = back;
}
/// A soft scrape now and then, while the body really slides.
fn scrape(g: &mut Game, t: Tick, rag: RagdollId, hips: V3, speed: f64) {
    let Tick { pl, c, dt, .. } = t;
    let carried = g.pl[pl].haul().map_or(false, |h| h.carry.is_some());
    if speed <= 0.3 || g.tab.ragdoll[rag].sleeping || carried { return; }
    let Some(tow) = g.pl[pl].tow.as_mut() else { return };
    tow.noise_t -= dt;
    if tow.noise_t > 0.0 { return; }
    tow.noise_t = 0.55;
    crate::audio_iface::game_audio_like_play(g, "propScrape", Some(hips), Some(0.25), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.7 }));
    prop_noise(g, [hips[0], 0.0, hips[2]], 0.22);
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
    let pace = if g.pl[pl].crouch { g.tune.player_crouch } else { crate::player::WALL_PACE };
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
        if by_hand { Some(m::atan2(d.p[0] - g.ch[c].pos[0], d.p[2] - g.ch[c].pos[2])) }
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
