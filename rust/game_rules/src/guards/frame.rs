//! ONE MAN'S FRAME: what has him, what he senses (senses/), how alert he is, what he does (behave/), how he is posed (with the looks laid over it, looks.rs).
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use crate::guards::behave::{Acts, Sensed};
use crate::guards::senses::Saw;
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- small helpers: the TypeScript's own repeated phrases

/// `c.bones.head ?? v3.add(c.pos, [0, 1.65, 0])`
pub(crate) fn eye_of_char(g: &Game, c: CharId) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::Head).copied() {
        Some(h) => h,
        None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]),
    }
}
/// `c.bones.head ?? c.pos`
pub(crate) fn head_or_pos(g: &Game, c: CharId) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::Head).copied() {
        Some(h) => h,
        None => g.ch[c].pos,
    }
}
/// `gd.path = []; gd.pathGoal = null;`: a whole write of both, so their identity words move with them.
pub(crate) fn clear_path(g: &mut Game, gd: GuardId) {
    let m = &mut g.gd[gd];
    m.path = Vec::new();
    m.path_serial = m.path_serial.wrapping_add(1);
    m.path_goal = None;
    m.path_goal_orbit = false;
}
/// `gd.speed = damp(gd.speed, 0, rate, dt)`
pub(crate) fn brake(g: &mut Game, gd: GuardId, rate: f64, dt: f64) {
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, rate, dt);
}
/// `const own = gd.script; gd.script = lent; gd.pieing = null; gd.aim.clear(); runGuardScript(g, gd, dt); gd.script = own;`
/// Gives back what the run left in the lent script: its lender reads that (conventions 4, an object lent), and its `<module>_script_ran` puts it home.
fn run_lent_script(g: &mut Game, gd: GuardId, lent: crate::game::GuardScript, dt: f64) -> Option<crate::game::GuardScript> {
    let own = g.gd[gd].script.take();
    g.gd[gd].script = Some(lent);
    g.gd[gd].pieing = None;
    crate::aim::aim_state_clear(&mut g.gd[gd].aim);
    crate::squad::run_guard_script(g, gd, dt);
    core::mem::replace(&mut g.gd[gd].script, own)
}

/// ONE MAN'S FRAME, part by part. Each part is a function below, in this order.   (guards.ts updateGuard)
///   incapable   dead or held: nothing below runs
///   lent        who has his body (control/): a rule of the list, or his own script; nothing below runs
///   senses      his ears, his eyes (senses/), what they do to his awareness and his fix; a body found; a forced door
///   alertness   his state moves with his awareness; the clocks that floor his reaction
///   behave      his state's behaviour (behave/): where his legs go, what his gun does
///   pose        the looks laid over his aim, the muzzle and the wall, the animator, his light, his footfall
pub fn update_guard(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    use crate::game::GuardState as St;
    let c = g.gd[gd].ch;
    crate::character::character_turn_begin(g, c);
    player_ground_vel(g);
    crate::guards::bubble_tick(g, gd);
    if incapable(g, gd, dt) { return; }
    if g.gd[gd].gun_lost_at >= 0.0 { gun_back(g, gd); }
    if g.m.guards.held_last_frame.delete(gd) {
        g.m.guards.unstick_legs.delete(gd);
        unstick_begin(g, gd);
    }
    let staff = g.gd[gd].team == crate::teams::Team::Guard;
    if !staff { unit_light_meter(g, gd, dt); }
    if lent(g, gd, dt) { return; }
    let saw = crate::guards::senses::perceive(g, gd, dt, c, staff);
    let s = alertness(g, gd, dt, c, staff, &saw);
    let acts = crate::guards::behave::act(g, gd, dt, &s);
    pose(g, gd, dt, &s, acts);
    crate::control::settle(g, gd);
}

/// Dead or held: his body is settled here and nothing else of his frame runs.
fn incapable(g: &mut Game, gd: GuardId, dt: f64) -> bool {
    use crate::game::GuardState as St;
    let c = g.gd[gd].ch;
    if g.gd[gd].state == St::Dead {
        crate::control::record(g, gd, crate::control::Holder::Dead);
        let m = &mut g.gd[gd];
        m.muzzle_down = false; m.pieing = None; m.search_plan = None; m.held = None; m.standoff = None; m.speed = 0.0;
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.ch[c].anim.speed = 0.0;
        crate::aim::aim_state_clear(&mut g.gd[gd].aim);
        crate::character::character_update(g, c, dt);
        crate::game::game_body_thud(g, c);
        attach_flashlight(g, gd, Some(dt));
        return true;
    }
    if g.gd[gd].held.is_some() {
        crate::control::record(g, gd, crate::control::Holder::Held);
        {
            let m = &mut g.gd[gd];
            m.pieing = None; m.search_plan = None; m.body_duty = None; m.standoff = None; m.muzzle_down = false; m.saw_player_this_frame = false;
        }
        clear_path(g, gd);
        g.gd[gd].stall_ref = None;
        crate::aim::aim_state_clear(&mut g.gd[gd].aim);
        crate::character::character_update(g, c, dt);
        attach_flashlight(g, gd, Some(dt));
        let prev_phase = g.gd[gd].step_phase;
        g.gd[gd].step_phase = g.ch[c].anim.phase;
        if g.gd[gd].speed > 0.4 && ((prev_phase < 0.5) != (g.gd[gd].step_phase < 0.5)) {
            let p = g.ch[c].pos;
            crate::floor::tread(g, p, 0.22, false);
        }
        g.gd[gd].last_pos = g.ch[c].pos;
        g.m.guards.held_last_frame.add(gd);
        return true;
    }
    false
}

/// A rule of the body list has him, or his own script: his body follows it and nothing else of his frame runs.
fn lent(g: &mut Game, gd: GuardId, dt: f64) -> bool {
    use crate::character::UpperMode;
    // WHO HAS HIS BODY (control/): the rules of the list are asked from the top; below them his own script, below that his own state
    if let Some((rule, script)) = crate::control::grant_body(g, gd, dt) {
        crate::control::record(g, gd, crate::control::Holder::Rule(rule));
        let ran = run_lent_script(g, gd, script, dt);
        crate::control::commit(g, gd, rule, crate::control::Outcome::Script(ran));
        return true;
    }
    if g.gd[gd].script.is_some() {
        crate::control::record(g, gd, crate::control::Holder::Script);
        {
            let m = &mut g.gd[gd];
            m.pieing = None; m.search_plan = None; m.standoff = None;
            crate::aim::aim_state_clear(&mut m.aim);
            let s = m.script.as_ref().unwrap();
            let drawn = s.over.is_some() || s.upper.clone().unwrap_or(UpperMode::Aim) == UpperMode::Aim;
            m.drawn = drawn;
        }
        crate::squad::run_guard_script(g, gd, dt);
        return true;
    }
    false
}

/// His state moves with his awareness (calm, suspicious, alert), the contact call, the clocks that floor his reaction.
fn alertness(g: &mut Game, gd: GuardId, dt: f64, c: CharId, staff: bool, saw: &Saw) -> Sensed {
    use crate::game::GuardState as St;
    let Saw { lvl, seen, dist } = *saw;
    // --- state transitions ---
    let prev = g.gd[gd].state;
    if g.gd[gd].awareness >= 0.85 && g.gd[gd].state != St::Alert {
        reset_chase(g, gd);
        crate::mind::set_state(g, gd, St::Alert, "guards");
        g.gd[gd].react_t = m::max(g.gd[gd].react_t, 0.35);
        g.gd[gd].fire_cd = 0.6 + crate::math_dice::sim_random(g) * 0.4;
        let now = g.time;
        let radio_recent = match &g.gd[gd].bubble { Some(b) => b.radio && now - b.t < 1.0, None => false };
        if g.gd[gd].standoff.is_none() && !radio_recent {
            let logged = g.messages.iter().any(|m| now >= m.t && now - m.t < 1.0 && m.text.ends_with(": CONTACT!"));
            crate::game::game_say(g, gd, "CONTACT!", Some(false), Some(false), Some(!logged));
        }
        let at = head_or_pos(g, c);
        crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.8), None);
        if g.time - g.last_sting_t > 8.0 {
            crate::audio_iface::game_audio_like_play(g, "alertSting", None, Some(0.75), None);
            g.last_sting_t = g.time;
        }
        if g.ai_enabled {
            let by = match alert_cause(g, gd, seen) {
                crate::guards::AlertCause::Body => crate::game::WitnessAlertedBy::Body,
                crate::guards::AlertCause::Radio => crate::game::WitnessAlertedBy::Radio,
                crate::guards::AlertCause::Sound => crate::game::WitnessAlertedBy::Sound,
                crate::guards::AlertCause::Sight => crate::game::WitnessAlertedBy::Sight,
            };
            g.gd[gd].witness.alerted_by = Some(by);
            g.gd[gd].witness.alert_t = g.time;
        }
        let _ = crate::chronicle::chron_contact(g, gd);
    }
    if g.ai_enabled && g.gd[gd].awareness > g.gd[gd].witness.peak_awareness { g.gd[gd].witness.peak_awareness = g.gd[gd].awareness; }
    if g.gd[gd].awareness >= 0.3 && g.gd[gd].state == St::Patrol {
        crate::mind::set_state(g, gd, St::Suspicious, "guards");
        g.gd[gd].search_t = 0.0;
        if g.gd[gd].body_duty.is_some() {
            g.gd[gd].react_t = 0.5 + crate::math_dice::sim_random(g) * 0.5;
        } else {
            g.gd[gd].react_t = 0.9;
            if g.gd[gd].team != crate::teams::Team::Operative && !crate::attention::attention_again(g, gd) {
                crate::game::game_say(g, gd, "huh?", None, None, None);
            }
            let at = head_or_pos(g, c);
            crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.45), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.15 }));
        }
    }
    crate::startle::startle_sight(g, gd, prev, seen);
    if prev != g.gd[gd].state {
        clear_path(g, gd);
        g.gd[gd].stall_ref = None;
        if g.gd[gd].state != St::Suspicious { g.gd[gd].body_duty = None; }
    }
    crate::attention::attention_tick(g, gd);
    if g.gd[gd].state != St::Search { g.gd[gd].search_plan = None; }
    let pair = standoff_upkeep(g, gd);
    let now = g.time;
    crate::game::guard_track_fix(g, gd, now);
    let dazzled = crate::control::man::blind(g, gd);
    if dazzled { g.gd[gd].react_t = m::max(g.gd[gd].react_t, g.gd[gd].dazzled_until - g.time); }
    let staggered = crate::control::man::staggered(g, gd);
    if staggered { g.gd[gd].react_t = m::max(g.gd[gd].react_t, g.gd[gd].stagger_until - g.time); }
    { let st = crate::startle::startle_left(g, gd); if st > 0.0 { g.gd[gd].react_t = m::max(g.gd[gd].react_t, st); } }
    crate::aim::track_guard(g, gd, dt);
    let (p0x, p0z) = (g.ch[c].pos[0], g.ch[c].pos[2]);

    g.ch[c].anim.look_yaw_extra = 0.0;
    g.gd[gd].look_phase += dt;
    Sensed { c, route: g.gd[gd].route, staff, lvl, dist, pair, dazzled, staggered, p0x, p0z }
}

/// The looks laid over his aim, the muzzle and the wall, the animator, his light, his footfall.
fn pose(g: &mut Game, gd: GuardId, dt: f64, s: &Sensed, acts: Acts) {
    use crate::game::GuardState as St;
    use crate::character::{Stance, UpperMode};
    let &Sensed { c, staff, staggered, p0x, p0z, .. } = s;
    let mut acts = acts;
    if !acts.pied || (g.gd[gd].state != St::Alert && g.gd[gd].state != St::Search) { g.gd[gd].pieing = None; }
    let eyes_on_foe = g.gd[gd].state == St::Alert && acts.aim_at.is_some() && g.time - g.gd[gd].last_seen_t < 0.5;
    crate::guards::looks::lay(g, gd, dt, eyes_on_foe, &mut acts);
    let Acts { move_speed, mut upper, aim_at, mut crouch, mut duck, .. } = acts;
    if let Some(a) = aim_at {
        let pos = g.ch[c].pos;
        let yaw_to = m::atan2(a[0] - pos[0], a[2] - pos[2]);
        g.ch[c].aim_yaw = v3::approach_angle(g.ch[c].aim_yaw, yaw_to, 6.0 * dt);
        let gun_y = pos[1] + 1.4;
        let hd = m::hyp2(a[0] - pos[0], a[2] - pos[2]);
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, m::atan2(a[1] - gun_y, m::max(hd, 0.5)), 5.0, dt);
        let off = v3::wrap_angle(g.ch[c].aim_yaw - g.ch[c].body_yaw);
        if move_speed < 0.3 && m::abs(off) > 0.5 { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, g.ch[c].aim_yaw, 5.0 * dt); }
    }
    let off = v3::wrap_angle(g.ch[c].aim_yaw - g.ch[c].body_yaw);
    if m::abs(off) > 1.5 { g.ch[c].aim_yaw = g.ch[c].body_yaw + m::sign(off) * 1.5; }
    collide_guard(g, gd, p0x, p0z);
    if crate::startle::startle_gun_down(g, gd) {
        upper = UpperMode::None;
        duck = duck || hand_carry(g, gd) != crate::guards::HandCarry::Torch;
    }
    let is_aim = upper == UpperMode::Aim;
    if g.gd[gd].muzzle_eval_t != g.time {
        let to = if is_aim { Some(match aim_at { Some(a) => a, None => aim_ahead(g, c) }) } else { None };
        muzzle_check(g, gd, to, dt, None);
    }
    if !g.gd[gd].pinned {
        let hot = g.gd[gd].state == St::Alert;
        crate::gunwall::gun_wall_check(g, gd, if is_aim { Some(crate::gunwall::GunUp::Aim) } else { None }, aim_at, dt, Some(hot));
    }
    if g.time < g.gd[gd].door_down_until { crouch = 1.0; }
    let lowered = g.gd[gd].muzzle_down || duck || staggered;
    {
        let a = &mut g.ch[c].anim;
        a.speed = move_speed;
        a.upper = if lowered { UpperMode::None } else { upper };
        a.crouch_target = crouch;
        a.lean = 0.0;
        a.stance = if lowered { Stance::LowReady } else { Stance::None };
    }
    crate::gunwall::gun_wall_pose(g, c);
    {
        let (vel, by) = (g.ch[c].vel, g.ch[c].body_yaw);
        g.ch[c].anim.reverse = move_speed > 0.2 && vel[0] * m::sin(by) + vel[2] * m::cos(by) < -0.3 * move_speed;
    }
    crate::character::character_update(g, c, dt);
    attach_flashlight(g, gd, Some(dt));
    let prev_phase = g.gd[gd].step_phase;
    g.gd[gd].step_phase = g.ch[c].anim.phase;
    if move_speed > 0.4 && ((prev_phase < 0.5) != (g.gd[gd].step_phase < 0.5)) {
        let pos = g.ch[c].pos;
        let base = crate::teampace::team_tread(g, gd, move_speed, 0.33);
        let loud = crate::floor::tread(g, pos, base, false);
        if !staff {
            let (time, who, serial) = (g.time, g.ch[c].id, g.serial());
            g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Step, pos, time, loud: false, level: Some(loud), who: Some(who), serial, ..crate::game::GameEvent::zeroed() });
        }
    }
}

