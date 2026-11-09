pub use ::game_types::aim::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::string::String;

/// Back to nothing; `last_shot` stays, as in the TypeScript.
pub fn aim_state_clear(this: &mut crate::aim::AimState) {
    this.track = 0.0;
    this.tgt_key = String::new();
    this.last_bearing = f64::NAN;
    this.bearing_rate = 0.0;
    this.speed_sm = 0.0;
    this.move_sm = 0.0;
    this.braking = false;
    this.fouled = None;
    this.last_man = String::new();
    this.left_t = -100.0;
    this.left_track = 0.0;
}

/// Remember the feet: `speed_sm` eased toward the ground speed over tau; `hold` feeds the remembered feet.
fn feed_feet(g: &mut crate::game::Game, a: &mut crate::aim::AimState, speed: f64, dt: f64, tau: f64, hold: Option<f64>) {
    let hold = hold.unwrap_or(speed);
    a.speed_sm += (speed - a.speed_sm) * (1.0 - m::exp(-dt / m::max(tau, 1e-3)));
    if !a.speed_sm.is_finite() || a.speed_sm < 1e-4 {
        a.speed_sm = 0.0;
    }
    a.move_sm = crate::aimcost::feet_hold(g, a.move_sm, hold, dt);
}

/// What the feet term reads for this shooter right now, m/s.
fn feet_of(g: &crate::game::Game, a: &crate::aim::AimState, vel: &[f64], steady_pace: f64) -> f64 {
    let f = m::max(m::hyp2(vel[0], vel[2]), a.speed_sm - steady_pace);
    m::max(f, a.move_sm) * g.k.aimcost.move_k
}

/// Feed one frame of the target into the bookkeeping; true if the bearing jumped.
fn feed_target(a: &mut crate::aim::AimState, from: V3, at: V3, vel: &[f64], dt: f64, jump_deg: f64, tau: f64) -> bool {
    let dx = at[0] - from[0];
    let dz = at[2] - from[2];
    let dist = m::hyp2(dx, dz);
    let brg = m::atan2(dx, dz);
    let mut jumped = false;
    if a.last_bearing.is_finite() && m::abs(v3::wrap_angle(brg - a.last_bearing)) > jump_deg * v3::DEG {
        jumped = true;
        a.bearing_rate = 0.0;
    } else {
        let omega = if dist > 1e-3 { m::abs(vel[0] * dz - vel[2] * dx) / dist / m::max(dist, 0.5) } else { 0.0 };
        a.bearing_rate += (omega - a.bearing_rate) * (1.0 - m::exp(-dt / m::max(tau, 1e-3)));
    }
    a.last_bearing = brg;
    if !a.bearing_rate.is_finite() || a.bearing_rate < 0.0 {
        a.bearing_rate = 0.0;
    }
    jumped
}

/// Once a frame per living, free, unscripted guard: the bearing to his foe, its rate, and the track.
pub fn track_guard(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let p = g.k.aim.guard;
    let c = g.gd[gd].ch;
    let pc = crate::teams::foe_char(g, gd);
    g.gd[gd].aim.braking = false;
    if !g.tune.aim_model || crate::condition::down(g, g.gd[gd].ch) {
        aim_state_clear(&mut g.gd[gd].aim);
        return;
    }
    let Some(pc) = pc else {
        aim_state_clear(&mut g.gd[gd].aim);
        return;
    };
    let c_vel = g.ch[c].vel;
    let speed = m::hyp2(c_vel[0], c_vel[2]);
    let mut a = core::mem::take(&mut g.gd[gd].aim);
    feed_feet(g, &mut a, speed, dt, p.feet_tau, None);
    g.gd[gd].aim = a;
    let at_player = matches!(g.gd[gd].foe, Some(crate::game::Foe::Player));
    let key = if at_player { String::from("sam") } else { alloc::format!("u{}", js::num_str(g.ch[pc].id)) };
    let a = &mut g.gd[gd].aim;
    if a.tgt_key != key {
        if !a.tgt_key.is_empty() {
            a.track = 0.0;
            a.last_bearing = f64::NAN;
            a.bearing_rate = 0.0;
        }
        a.tgt_key = key;
    }
    let c_pos = g.ch[c].pos;
    let trunk = match g.ch[pc].bones.get(crate::character::Bone::Spine2).or(g.ch[pc].bones.get(crate::character::Bone::Hips)) {
        Some(b) => *b,
        None => g.ch[pc].pos,
    };
    let brg = m::atan2(trunk[0] - c_pos[0], trunk[2] - c_pos[2]);
    let tvel = if at_player { crate::guards::player_ground_vel(g) } else { g.ch[pc].vel };
    let jumped = feed_target(&mut g.gd[gd].aim, c_pos, trunk, &tvel, dt, p.jump_deg, p.rate_tau);
    if g.gd[gd].pinned && g.gd[gd].skill.is_none() {
        g.gd[gd].aim.track = 1.0;
        return;
    }
    let since_seen = g.time - g.gd[gd].last_seen_t;
    let down = crate::teams::foe_down(g, gd);
    if jumped && g.gd[gd].aim.track > 0.0 {
        g.gd[gd].aim.track = 0.0;
    }
    if since_seen > p.mem_secs || down {
        g.gd[gd].aim.track = 0.0;
        return;
    }
    if since_seen >= LOS_NOW {
        return;
    }
    let gun_yaw = crate::character::character_gun_yaw(g, c);
    let facing_err = m::abs(v3::wrap_angle(brg - gun_yaw));
    let dist = v3::dist_xz(g.ch[c].pos, g.ch[pc].pos);
    let on_tol = p.on_deg * v3::DEG + m::atan2(0.35, m::max(dist, 0.3));
    let on = g.gd[gd].state == crate::game::GuardState::Alert && g.gd[gd].react_t <= 0.0 && !crate::control::man::blind(g, gd) && facing_err < on_tol;
    let cap = if m::max(speed, g.gd[gd].aim.move_sm) > m::min(p.stop_to_shoot, g.k.aimcost.plant_at) { p.moving_track_cap } else { 1.0 };
    if on {
        let k = match &g.gd[gd].skill {
            Some(s) => s.settle_k,
            None => if g.gd[gd].trained { g.k.trained.settle_k } else { 1.0 },
        };
        let st = crate::aim::SettleTimeP { settle_secs: p.settle_secs, range_ref: p.range_ref, near_k: p.near_k };
        let a = &mut g.gd[gd].aim;
        a.track = m::min(cap, a.track + dt / (settle_time(st, dist) * k));
    } else {
        let a = &mut g.gd[gd].aim;
        a.track = m::max(0.0, m::min(cap, a.track - p.lose_rate * dt));
    }
}

/// Seconds to settle fully at this range.
pub fn settle_time(p: crate::aim::SettleTimeP, dist: f64) -> f64 {
    m::max(p.settle_secs, 1e-3) * v3::clamp(dist / m::max(p.range_ref, 0.1), p.near_k, 1.0)
}

/// The first-shot gate; true with the model off.
pub fn aim_ready(g: &crate::game::Game, gd: GuardId) -> bool {
    !g.tune.aim_model || g.gd[gd].aim.track >= g.k.aim.guard.acquire_min - 1e-9
}

/// The standoff's flank round: are the feet he remembers still enough?
pub fn feet_settled(g: &crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].aim.move_sm <= g.k.aimcost.flank_at
}

/// `gd.weapon.def.aiSpread`: the row of the gun in his hands.
fn ai_spread_of(g: &crate::game::Game, gd: GuardId) -> crate::weapons::AiSpread {
    g.m.weapons.weapons.get(g.fa[g.gd[gd].weapon].def).unwrap().ai_spread
}

/// The man he is firing at, Sam's body if nobody: `foeChar(g, gd) ?? g.player.char`.
fn foe_or_player(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId) -> CharId {
    match crate::teams::foe_char(g, gd) {
        Some(c) => c,
        None => g.pl[seat.player].ch,
    }
}

/// The terms of a guard's cone under the model.
fn guard_terms(g: &mut crate::game::Game, gd: GuardId, extra: f64) -> crate::aim::GuardTerms {
    let p = g.k.aim.guard;
    let c = g.gd[gd].ch;
    let s = ai_spread_of(g, gd);
    let tc = foe_or_player(g, &crate::seats::here(g), gd);
    let range = v3::dist(g.ch[c].pos, g.ch[tc].pos);
    let k = match &g.gd[gd].skill {
        Some(x) => x.spread_k,
        None => if g.gd[gd].trained { g.k.trained.spread_k } else { 1.0 },
    };
    let hands = (s.base + m::max(0.0, range - s.range_from) * s.range_k) * k;
    let settle = v3::lerp(p.unsettled_k, 1.0, v3::clamp(g.gd[gd].aim.track, 0.0, 1.0));
    let tgt = m::min(p.target_move_cap, g.gd[gd].aim.bearing_rate * p.target_move_k);
    let c_vel = g.ch[c].vel;
    let a = core::mem::take(&mut g.gd[gd].aim);
    let own = feet_of(g, &a, &c_vel, p.steady_pace) * p.shooter_move_k;
    g.gd[gd].aim = a;
    crate::aim::GuardTerms { spread: hands * settle + tgt + own + extra }
}

/// What to hand fireWeapon for this guard's round; stamps `last_shot`.
pub fn guard_round(g: &mut crate::game::Game, gd: GuardId, extra: f64, gated: Option<bool>) -> crate::combat::FireOpts {
    if !g.tune.aim_model {
        return crate::combat::FireOpts { spread: Some(extra), ..Default::default() };
    }
    let t = guard_terms(g, gd, extra);
    let now = g.time;
    g.gd[gd].aim.last_shot = Some(crate::aim::AimStateLastShot { t: now });
    crate::combat::FireOpts { spread: Some(t.spread), steady: Some(true), ..Default::default() }
}

/// The whole cone a guard's round is drawn from.
pub fn hands_cone(g: &mut crate::game::Game, gd: GuardId, extra: f64) -> f64 {
    if !g.tune.aim_model {
        let s = ai_spread_of(g, gd);
        let tc = foe_or_player(g, &crate::seats::here(g), gd);
        let tv = g.ch[tc].vel;
        return s.base + m::min(s.move_cap, m::hyp2(tv[0], tv[2]) * s.move_k) + m::max(0.0, v3::dist(g.ch[g.gd[gd].ch].pos, g.ch[tc].pos) - s.range_from) * s.range_k + extra;
    }
    guard_terms(g, gd, extra).spread
}

/// Stop to shoot: true when the caller should brake him to a standstill.
pub fn aim_brake(g: &mut crate::game::Game, gd: GuardId, dist: f64, blocked: bool) -> bool {
    let p = g.k.aim.guard;
    let w = g.gd[gd].weapon;
    let brake = g.tune.aim_model && !g.gd[gd].pinned && !blocked && dist <= p.shoot_band && g.time - g.gd[gd].last_seen_t < LOS_NOW && g.gd[gd].aim.track < 1.0 && g.gd[gd].reload_t < 0.0 && (g.gd[gd].fire_cd <= p.brake_lead || g.fa[w].burst.is_some());
    g.gd[gd].aim.braking = brake;
    brake
}

/// Fed the round this guard has just let go: count it if it went into the frame on his half of the line.
pub fn note_bore(g: &mut crate::game::Game, gd: GuardId, aim_at: V3) {
    if !g.tune.aim_model {
        return;
    }
    let (from, to) = match g.events.last() {
        Some(e) if e.kind == crate::director::EventKind::GuardShot && e.time == g.time => match (e.from, e.to) {
            (Some(f), Some(t)) => (f, t),
            _ => return,
        },
        _ => return,
    };
    let flew = v3::dist(from, to);
    let wanted = v3::dist(from, aim_at);
    if flew < m::min(m::max(BORE_FOUL_M, 0.5 * wanted), wanted - 0.6) {
        let pos = g.ch[g.gd[gd].ch].pos;
        let a = &mut g.gd[gd].aim;
        a.fouled = match a.fouled {
            Some(f) if v3::dist_xz(f.pos, pos) < 0.5 => Some(crate::aim::AimStateFouled { n: f.n + 1.0, pos: f.pos }),
            _ => Some(crate::aim::AimStateFouled { n: 1.0, pos: v3::copy(pos) }),
        };
    } else {
        g.gd[gd].aim.fouled = None;
    }
}

/// Two rounds running into the frame from where he stands: that spot is no line.
pub fn bore_fouled(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let f = match g.gd[gd].aim.fouled {
        Some(f) if g.tune.aim_model => f,
        _ => return false,
    };
    if v3::dist_xz(f.pos, g.ch[g.gd[gd].ch].pos) > 0.5 {
        g.gd[gd].aim.fouled = None;
        return false;
    }
    f.n >= 2.0
}

/// Once a frame: Sam's track on what is under the reticle, its caps, and that man's rate across his view.
pub fn track_player(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let p = g.k.aim.player;
    let c = g.pl[pl].ch;
    if !g.tune.aim_model || crate::condition::down(g, g.pl[pl].ch) {
        aim_state_clear(&mut g.pl[pl].aim);
        return;
    }
    let gv = crate::guards::player_ground_vel(g);
    let speed = m::hyp2(gv[0], gv[2]);
    let c_vel = g.ch[c].vel;
    let hold = m::min(speed, m::hyp2(c_vel[0], c_vel[2]));
    let mut a = core::mem::take(&mut g.pl[pl].aim);
    feed_feet(g, &mut a, speed, dt, p.feet_tau, Some(hold));
    g.pl[pl].aim = a;
    let tg = match g.aim_guard {
        Some(x) if !crate::condition::down(g, g.gd[x].ch) => Some(x),
        _ => None,
    };
    let lt = if tg.is_none() { g.aim_target } else { None };
    let key = if let Some(t) = tg {
        alloc::format!("g{}", js::num_str(g.ch[g.gd[t].ch].id))
    } else if let Some(l) = lt {
        alloc::format!("l{}", g.tab.sim_lightshome_fixture[l].name)
    } else {
        String::from("free")
    };
    if key != g.pl[pl].aim.tgt_key {
        let now = g.time;
        let back = key == g.pl[pl].aim.last_man && now - g.pl[pl].aim.left_t < 0.35;
        let was = g.pl[pl].aim.tgt_key.clone();
        let had = g.pl[pl].aim.track;
        if lt.is_some() && !back {
            g.pl[pl].aim.track = 0.0;
        }
        if tg.is_some() && !back {
            g.pl[pl].aim.track = 0.0;
        }
        if back && was.starts_with('l') {
            let a = &mut g.pl[pl].aim;
            a.track = m::max(a.track, a.left_track);
        }
        let a = &mut g.pl[pl].aim;
        if was.starts_with('l') || was.starts_with('g') && (tg.is_none() || lt.is_some()) {
            a.last_man = was;
            a.left_t = now;
            a.left_track = had;
        }
        a.tgt_key = key;
        a.last_bearing = f64::NAN;
        a.bearing_rate = 0.0;
    }
    let t: V3 = if let Some(t) = tg {
        let tc = g.gd[t].ch;
        match g.ch[tc].bones.get(crate::character::Bone::Chest) {
            Some(b) => *b,
            None => v3::add(g.ch[tc].pos, [0.0, 1.2, 0.0]),
        }
    } else if let Some(l) = lt {
        g.tab.sim_lightshome_fixture[l].pos
    } else {
        g.aim_point
    };
    let c_pos = g.ch[c].pos;
    if let Some(tn) = tg {
        let tv = g.ch[g.gd[tn].ch].vel;
        let a = &mut g.pl[pl].aim;
        if feed_target(a, c_pos, t, &tv, dt, p.jump_deg, p.rate_tau) && a.track > 0.0 {
            a.track = 0.0;
        }
    } else {
        let a = &mut g.pl[pl].aim;
        a.last_bearing = f64::NAN;
        a.bearing_rate = 0.0;
    }
    let cap = if g.pl[pl].sprinting && speed > p.moving_above { p.sprint_cap } else if speed > p.moving_above { p.walk_cap } else { 1.0 };
    let st = crate::aim::SettleTimeP { settle_secs: p.settle_secs, range_ref: p.range_ref, near_k: p.near_k };
    let rate = (if g.pl[pl].crouch { p.crouch_k } else { 1.0 }) / settle_time(st, v3::dist_xz(c_pos, t));
    let a = &mut g.pl[pl].aim;
    a.track = m::min(cap, a.track + rate * dt);
}

/// He has you lined up: his track is past the aim model's first-shot gate.
pub fn guard_acquired(g: &crate::game::Game, gd: GuardId) -> bool {
    g.tune.aim_model && g.gd[gd].state == crate::game::GuardState::Alert && g.gd[gd].aim.track >= g.k.aim.guard.acquire_min - 1e-9
}

/// Sam's cone under the model and his track: nothing with the model off or under the puppet.
pub fn player_aim(g: &crate::game::Game, pl: PlayerId) -> crate::aim::PlayerAim {
    if !g.tune.aim_model || g.puppet.is_some() {
        return crate::aim::PlayerAim { spread: 0.0, track: 1.0, base: 0.0, settle: 1.0, tgt: 0.0, own: 0.0 };
    }
    let p = g.k.aim.player;
    let c = g.pl[pl].ch;
    let base = if g.pl[pl].crouch { p.base_crouch } else { p.base_stand };
    let track = g.pl[pl].aim.track;
    let settle = v3::lerp(p.unsettled_k, 1.0, v3::clamp(track, 0.0, 1.0));
    let tgt = m::min(p.target_move_cap, g.pl[pl].aim.bearing_rate * p.target_move_k);
    let c_vel = g.ch[c].vel;
    let own = feet_of(g, &g.pl[pl].aim, &c_vel, p.steady_pace) * p.shooter_move_k;
    // AIM_HOOKS.playerExternalK as player.ts:258 sets it at load, the only value it ever holds.
    let k = if crate::game::player_braced(g, pl) { g.k.rifle_spread.brace_k } else { 1.0 };
    let kk = if k.is_finite() { m::max(0.0, k) } else { 1.0 };
    crate::aim::PlayerAim { spread: (base * settle + tgt) * kk + own * v3::lerp(kk, 1.0, v3::clamp(1.0 - g.k.aimcost.brace_feet, 0.0, 1.0)), track, base, settle, tgt, own }
}

/// The one term Sam's fire sites add to their spread; stamps `last_shot`.
pub(super) fn sam_aim(g: &mut crate::game::Game) -> f64 {
    let pl = g.player;
    let r = player_aim(g, pl);
    if !g.tune.aim_model || g.puppet.is_some() {
        return 0.0;
    }
    let now = g.time;
    g.pl[pl].aim.last_shot = Some(crate::aim::AimStateLastShot { t: now });
    r.spread
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A failure means a man's lead on a moving target is wrong: a shooter holds off for one who crosses his sights, not for one who walks straight at him, and starts over when the target jumps.
    #[test]
    fn the_bearing_rate_rises_for_a_man_who_crosses_and_a_jump_is_a_new_target() {
        let mut a = crate::aim::AimState::blank();
        let (from, ahead): (V3, V3) = ([0.0; 3], [0.0, 0.0, 5.0]);
        assert!(!feed_target(&mut a, from, ahead, &[0.0, 0.0, 2.0], 1.0 / 60.0, 25.0, 0.15));
        assert_eq!(a.bearing_rate, 0.0, "walking straight at the shooter");
        assert!(!feed_target(&mut a, from, ahead, &[2.0, 0.0, 0.0], 1.0 / 60.0, 25.0, 0.15));
        let eased = a.bearing_rate;
        assert!(eased > 0.0 && eased < 0.4, "crossing at 2 m/s at 5 m eases up toward 0.4 rad/s: {eased}");
        assert!(!feed_target(&mut a, from, ahead, &[2.0, 0.0, 0.0], 1.0 / 60.0, 25.0, 0.0));
        assert!((a.bearing_rate - 0.4).abs() < 1e-6, "with no easing it is there at once: {}", a.bearing_rate);
        assert!(feed_target(&mut a, from, [-5.0, 0.0, -2.0], &[2.0, 0.0, 0.0], 1.0 / 60.0, 25.0, 0.15));
        assert_eq!(a.bearing_rate, 0.0, "a jump past the limit starts the rate over");
        // at no range to speak of nothing crosses
        assert!(!feed_target(&mut a, from, [-5e-4, 0.0, -2e-4], &[2.0, 0.0, 0.0], 1.0 / 60.0, 25.0, 0.0));
        assert_eq!(a.bearing_rate, 0.0);
    }

    /// A man settles quicker on a near target, down to a floor; at the range of reference and past it he takes the whole time, and never no time at all.
    #[test]
    fn a_near_target_settles_quicker_down_to_a_floor() {
        let p = |settle_secs, range_ref, near_k| crate::aim::SettleTimeP { settle_secs, range_ref, near_k };
        let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-12, "{a} {b}");
        near(settle_time(p(1.2, 8.0, 0.25), 8.0), 1.2);
        near(settle_time(p(1.2, 8.0, 0.25), 20.0), 1.2);
        near(settle_time(p(1.2, 8.0, 0.25), 4.0), 0.6);
        near(settle_time(p(1.2, 8.0, 0.25), 0.1), 0.3);
        assert!(settle_time(p(0.0, 0.0, 0.25), 3.3) > 0.0);
    }

    /// clear() empties everything but the last round's record.
    #[test]
    fn clear_keeps_the_last_shot() {
        let mut a = crate::aim::AimState::blank();
        a.track = 0.7;
        a.tgt_key = String::from("sam");
        a.last_bearing = 1.0;
        a.braking = true;
        a.left_t = 3.0;
        a.fouled = Some(crate::aim::AimStateFouled { n: 2.0, pos: [1.0, 0.0, 1.0] });
        a.last_shot = Some(crate::aim::AimStateLastShot::default());
        aim_state_clear(&mut a);
        assert_eq!(a.track, 0.0);
        assert!(a.tgt_key.is_empty() && a.last_bearing.is_nan() && !a.braking && a.fouled.is_none());
        assert_eq!(a.left_t, -100.0);
        assert!(a.last_shot.is_some());
    }
}
