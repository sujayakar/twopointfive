//! src/game/aim.ts lines 1 to 368
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Back to nothing; `last_shot` stays, as in the TypeScript. (aim.ts AimState.clear)
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

/// Is the moving-feet rule of aimcost.ts in force? Never under a show. (aim.ts moveCost)
pub fn move_cost(g: &mut crate::game::Game) -> bool {
    !g.touring
}

/// Is the lamp rule of aimcost.ts in force? Never under a show. (aim.ts lampCost)
pub fn lamp_cost(g: &mut crate::game::Game) -> bool {
    !g.touring
}

/// Remember the feet: `speed_sm` eased toward the ground speed over tau; `hold` feeds the remembered feet. (aim.ts feedFeet)
pub fn feed_feet(g: &mut crate::game::Game, a: &mut crate::aim::AimState, speed: f64, dt: f64, tau: f64, on: bool, hold: Option<f64>) {
    let hold = hold.unwrap_or(speed);
    a.speed_sm += (speed - a.speed_sm) * (1.0 - m::exp(-dt / m::max(tau, 1e-3)));
    if !a.speed_sm.is_finite() || a.speed_sm < 1e-4 {
        a.speed_sm = 0.0;
    }
    a.move_sm = if on { crate::aimcost::feet_hold(g, a.move_sm, hold, dt) } else { 0.0 };
}

/// What the feet term reads for this shooter right now, m/s. (aim.ts feetOf)
pub fn feet_of(g: &mut crate::game::Game, a: &crate::aim::AimState, vel: &[f64], steady_pace: f64, on: bool) -> f64 {
    let f = m::max(m::hyp2(vel[0], vel[2]), a.speed_sm - steady_pace);
    if on { m::max(f, a.move_sm) * g.k.aimcost.move_k } else { f }
}

/// Feed one frame of the target into the bookkeeping; true if the bearing jumped. (aim.ts feedTarget)
pub fn feed_target(a: &mut crate::aim::AimState, from: V3, at: V3, vel: &[f64], dt: f64, jump_deg: f64, tau: f64) -> bool {
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

/// Once a frame per living, free, unscripted guard: the bearing to his foe, its rate, and the track. (aim.ts trackGuard)
pub fn track_guard(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let p = g.k.aim.guard;
    let c = g.gd[gd].ch;
    let pc = crate::teams::foe_char(g, gd);
    g.gd[gd].aim.braking = false;
    if !g.tune.aim_model || g.gd[gd].state == crate::game::GuardState::Dead {
        aim_state_clear(&mut g.gd[gd].aim);
        return;
    }
    let Some(pc) = pc else {
        aim_state_clear(&mut g.gd[gd].aim);
        return;
    };
    let c_vel = g.ch[c].vel;
    let speed = m::hyp2(c_vel[0], c_vel[2]);
    let on_cost = move_cost(g);
    let mut a = core::mem::take(&mut g.gd[gd].aim);
    feed_feet(g, &mut a, speed, dt, p.feet_tau, on_cost, None);
    g.gd[gd].aim = a;
    let at_player = matches!(g.gd[gd].foe, Some(crate::game::Foe::Player));
    let key = if at_player { String::from("sam") } else { format!("u{}", g.ch[pc].id) };
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
    let on = g.gd[gd].state == crate::game::GuardState::Alert && g.gd[gd].react_t <= 0.0 && g.time >= g.gd[gd].dazzled_until && facing_err < on_tol;
    let cap = if if move_cost(g) { m::max(speed, g.gd[gd].aim.move_sm) > m::min(p.stop_to_shoot, g.k.aimcost.plant_at) } else { speed > p.stop_to_shoot } { p.moving_track_cap } else { 1.0 };
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

/// Seconds to settle fully at this range. (aim.ts settleTime)
pub fn settle_time(p: crate::aim::SettleTimeP, dist: f64) -> f64 {
    m::max(p.settle_secs, 1e-3) * v3::clamp(dist / m::max(p.range_ref, 0.1), p.near_k, 1.0)
}

/// The first-shot gate; true with the model off. (aim.ts aimReady)
pub fn aim_ready(g: &mut crate::game::Game, gd: GuardId) -> bool {
    !g.tune.aim_model || g.gd[gd].aim.track >= g.k.aim.guard.acquire_min - 1e-9
}

/// The standoff's flank round: are the feet he remembers still enough? (aim.ts feetSettled)
pub fn feet_settled(g: &mut crate::game::Game, gd: GuardId) -> bool {
    !move_cost(g) || g.gd[gd].aim.move_sm <= g.k.aimcost.flank_at
}

/// An alert man whose track has passed the gate. (aim.ts guardAcquired)
pub fn guard_acquired(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.tune.aim_model && g.gd[gd].state == crate::game::GuardState::Alert && g.gd[gd].aim.track >= g.k.aim.guard.acquire_min - 1e-9
}

/// `gd.weapon.def.aiSpread`: the row of the gun in his hands.
fn ai_spread_of(g: &crate::game::Game, gd: GuardId) -> crate::weapons::AiSpread {
    g.m.weapons.weapons.get(g.fa[g.gd[gd].weapon].def).unwrap().ai_spread
}

/// The man he is firing at, Sam's body if nobody: `foeChar(g, gd) ?? g.player.char`.
fn foe_or_player(g: &mut crate::game::Game, gd: GuardId) -> CharId {
    match crate::teams::foe_char(g, gd) {
        Some(c) => c,
        None => g.pl[g.player].ch,
    }
}

/// The terms of a guard's cone under the model. (aim.ts guardTerms)
pub fn guard_terms(g: &mut crate::game::Game, gd: GuardId, extra: f64) -> crate::aim::GuardTerms {
    let p = g.k.aim.guard;
    let c = g.gd[gd].ch;
    let s = ai_spread_of(g, gd);
    let tc = foe_or_player(g, gd);
    let range = v3::dist(g.ch[c].pos, g.ch[tc].pos);
    let k = match &g.gd[gd].skill {
        Some(x) => x.spread_k,
        None => if g.gd[gd].trained { g.k.trained.spread_k } else { 1.0 },
    };
    let hands = (s.base + m::max(0.0, range - s.range_from) * s.range_k) * k;
    let settle = v3::lerp(p.unsettled_k, 1.0, v3::clamp(g.gd[gd].aim.track, 0.0, 1.0));
    let tgt = m::min(p.target_move_cap, g.gd[gd].aim.bearing_rate * p.target_move_k);
    let c_vel = g.ch[c].vel;
    let speed = m::hyp2(c_vel[0], c_vel[2]);
    let on_cost = move_cost(g);
    let a = core::mem::take(&mut g.gd[gd].aim);
    let own = feet_of(g, &a, &c_vel, p.steady_pace, on_cost) * p.shooter_move_k;
    g.gd[gd].aim = a;
    crate::aim::GuardTerms { hands, settle, tgt, own, speed, spread: hands * settle + tgt + own + extra }
}

/// What to hand fireWeapon for this guard's round; stamps `last_shot`. (aim.ts guardRound)
pub fn guard_round(g: &mut crate::game::Game, gd: GuardId, extra: f64, gated: Option<bool>) -> crate::combat::FireOpts {
    let gated = gated.unwrap_or(true);
    if !g.tune.aim_model {
        return crate::combat::FireOpts { spread: Some(extra), ..Default::default() };
    }
    let t = guard_terms(g, gd, extra);
    let (now, track) = (g.time, g.gd[gd].aim.track);
    g.gd[gd].aim.last_shot = Some(crate::aim::AimStateLastShot { t: now, track, spread: t.spread, speed: t.speed, gated, hands: t.hands, settle: t.settle, tgt: t.tgt, own: t.own });
    crate::combat::FireOpts { spread: Some(t.spread), steady: Some(true), ..Default::default() }
}

/// The whole cone a guard's round is drawn from. (aim.ts handsCone)
pub fn hands_cone(g: &mut crate::game::Game, gd: GuardId, extra: f64) -> f64 {
    if !g.tune.aim_model {
        let s = ai_spread_of(g, gd);
        let tc = foe_or_player(g, gd);
        let tv = g.ch[tc].vel;
        return s.base + m::min(s.move_cap, m::hyp2(tv[0], tv[2]) * s.move_k) + m::max(0.0, v3::dist(g.ch[g.gd[gd].ch].pos, g.ch[tc].pos) - s.range_from) * s.range_k + extra;
    }
    guard_terms(g, gd, extra).spread
}

/// Stop to shoot: true when the caller should brake him to a standstill. (aim.ts aimBrake)
pub fn aim_brake(g: &mut crate::game::Game, gd: GuardId, dist: f64, blocked: bool) -> bool {
    let p = g.k.aim.guard;
    let w = g.gd[gd].weapon;
    let brake = g.tune.aim_model && !g.gd[gd].pinned && !blocked && dist <= p.shoot_band && g.time - g.gd[gd].last_seen_t < LOS_NOW && g.gd[gd].aim.track < 1.0 && g.gd[gd].reload_t < 0.0 && (g.gd[gd].fire_cd <= p.brake_lead || g.fa[w].burst.is_some());
    g.gd[gd].aim.braking = brake;
    brake
}

/// Fed the round this guard has just let go: count it if it went into the frame on his half of the line. (aim.ts noteBore)
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

/// Two rounds running into the frame from where he stands: that spot is no line. (aim.ts boreFouled)
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

/// Once a frame: Sam's track on what is under the reticle, its caps, and that man's rate across his view. (aim.ts trackPlayer)
pub fn track_player(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let p = g.k.aim.player;
    let c = g.pl[pl].ch;
    if !g.tune.aim_model || g.pl[pl].down {
        aim_state_clear(&mut g.pl[pl].aim);
        return;
    }
    let gv = crate::guards::player_ground_vel(g);
    let speed = m::hyp2(gv[0], gv[2]);
    let on_cost = move_cost(g);
    let c_vel = g.ch[c].vel;
    let hold = m::min(speed, m::hyp2(c_vel[0], c_vel[2]));
    let mut a = core::mem::take(&mut g.pl[pl].aim);
    feed_feet(g, &mut a, speed, dt, p.feet_tau, on_cost, Some(hold));
    g.pl[pl].aim = a;
    let tg = match g.aim_guard {
        Some(x) if g.gd[x].state != crate::game::GuardState::Dead => Some(x),
        _ => None,
    };
    let lt = if tg.is_none() && lamp_cost(g) { g.aim_target } else { None };
    let key = if let Some(t) = tg {
        format!("g{}", g.ch[g.gd[t].ch].id)
    } else if let Some(l) = lt {
        format!("l{}", g.tab.sim_lightshome_fixture[l].name)
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
        if back && was.starts_with('l') && lamp_cost(g) {
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

/// Sam's cone under the model and his track: nothing with the model off or under the puppet. (aim.ts playerAim)
pub fn player_aim(g: &mut crate::game::Game) -> crate::aim::PlayerAim {
    if !g.tune.aim_model || g.puppet.is_some() {
        return crate::aim::PlayerAim { spread: 0.0, track: 1.0, base: 0.0, settle: 1.0, tgt: 0.0, own: 0.0 };
    }
    let pl = g.player;
    let p = g.k.aim.player;
    let c = g.pl[pl].ch;
    let base = if g.pl[pl].crouch { p.base_crouch } else { p.base_stand };
    let track = g.pl[pl].aim.track;
    let settle = v3::lerp(p.unsettled_k, 1.0, v3::clamp(track, 0.0, 1.0));
    let tgt = m::min(p.target_move_cap, g.pl[pl].aim.bearing_rate * p.target_move_k);
    let c_vel = g.ch[c].vel;
    let on_cost = move_cost(g);
    let a = core::mem::take(&mut g.pl[pl].aim);
    let own = feet_of(g, &a, &c_vel, p.steady_pace, on_cost) * p.shooter_move_k;
    g.pl[pl].aim = a;
    // AIM_HOOKS.playerExternalK as player.ts:258 sets it at load, the only value it ever holds.
    let k = if crate::game::player_braced(g, pl) { g.k.rifle_spread.brace_k } else { 1.0 };
    let kk = if k.is_finite() { m::max(0.0, k) } else { 1.0 };
    if move_cost(g) {
        return crate::aim::PlayerAim { spread: (base * settle + tgt) * kk + own * v3::lerp(kk, 1.0, v3::clamp(1.0 - g.k.aimcost.brace_feet, 0.0, 1.0)), track, base, settle, tgt, own };
    }
    crate::aim::PlayerAim { spread: (base * settle + tgt + own) * kk, track, base, settle, tgt, own }
}

/// The one term Sam's fire sites add to their spread; stamps `last_shot`. (aim.ts samAim)
pub fn sam_aim(g: &mut crate::game::Game) -> f64 {
    let r = player_aim(g);
    if !g.tune.aim_model || g.puppet.is_some() {
        return 0.0;
    }
    let pl = g.player;
    let v = g.ch[g.pl[pl].ch].vel;
    let now = g.time;
    g.pl[pl].aim.last_shot = Some(crate::aim::AimStateLastShot { t: now, track: r.track, spread: r.spread, speed: m::hyp2(v[0], v[2]), gated: false, hands: r.base, settle: r.settle, tgt: r.tgt, own: r.own });
    r.spread
}

/// The panel's save of the kept knobs: localStorage is the host's, the game holds nothing to do here. (aim.ts saveAim)
pub fn save_aim(g: &mut crate::game::Game) {}

/// The load of the kept knobs: localStorage is the host's, which writes `g.k.aim` itself if it keeps any. (aim.ts loadAim)
pub fn load_aim(g: &mut crate::game::Game) {}

/// Every aim knob back to the shipped values; dropping the stored copy is the host's. (aim.ts resetAim)
pub fn reset_aim(g: &mut crate::game::Game) {
    g.k.aim = aim_defaults();
}

/// `x.toFixed(2)` for the panel's line. (aim.ts aimSummary f2)
fn f2(x: f64) -> String {
    format!("{:.2}", x)
}

/// `x.toFixed(3)` for the panel's line. (aim.ts aimSummary f3)
fn f3(x: f64) -> String {
    format!("{:.3}", x)
}

/// The panel's line: Sam, then every living alert man, or the nearest at rest. (aim.ts aimSummary)
pub fn aim_summary(g: &mut crate::game::Game) -> String {
    if !g.tune.aim_model {
        return String::from("aim ▸ model OFF (every round as it was: guards' rows, the player dead-on)");
    }
    let r = player_aim(g);
    let key = g.pl[g.player].aim.tgt_key.clone();
    let on = if !key.is_empty() && key != "free" {
        if key.starts_with('l') { format!(" on lamp {}", &key[1..]) } else { format!(" on #{}", &key[1..]) }
    } else {
        String::new()
    };
    let puppet = if g.puppet.is_some() { " [puppet: dead-on]" } else { "" };
    let mut parts: Vec<String> = vec![format!(
        "you: track {}{} · cone {} (hands {} ×{} + crossing {} + feet {}){}",
        f2(g.pl[g.player].aim.track), on, f3(r.spread), f3(r.base), f2(r.settle), f3(r.tgt), f3(r.own), puppet
    )];
    let mut men: Vec<GuardId> = g.guards.iter().copied().filter(|x| g.gd[*x].state == crate::game::GuardState::Alert).collect();
    if men.is_empty() {
        if let Some(n) = crate::game::game_nearest_living_guard(g, None, None) {
            men = vec![n];
        }
    }
    for gd in men {
        let t = guard_terms(g, gd, 0.0);
        let acquired = if guard_acquired(g, gd) { " ◎" } else { "" };
        let now = g.time;
        let x = &g.gd[gd];
        let pinned = if x.pinned { " (pinned)" } else { "" };
        let braking = if x.aim.braking { " · BRAKING" } else { "" };
        let last = match &x.aim.last_shot {
            Some(ls) if now - ls.t < 3.0 => format!(" · last round {} s ago at track {}, {} rad, {} m/s", f2(now - ls.t), f2(ls.track), f3(ls.spread), f2(ls.speed)),
            _ => String::new(),
        };
        parts.push(format!(
            "{}{} {} track {}{} · brg {} rad/s · cone {} (hands {} ×{} + crossing {} + feet {}){}{}",
            x.callsign, acquired, x.state.as_str(), f2(x.aim.track), pinned, f2(x.aim.bearing_rate), f3(t.spread), f3(t.hands), f2(t.settle), f3(t.tgt), f3(t.own), braking, last
        ));
    }
    format!("aim ▸ {}", parts.join(" · "))
}

pub mod aim_hooks {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/aim.ts:145 AIM_HOOKS.playerExternalK)
    pub fn player_external_k() -> f64 {
        todo!("needs: g (its only real value, player.ts:258, reads g.player.braced); player_aim reads that directly, nothing calls this")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The TypeScript's own feedTarget under EXACT, run on one AimState in sequence: first frame, a step, a jump, a zero tau, a tiny range, other knobs.
    #[test]
    fn feed_target_keeps_the_typescript_s_bits() {
        let mut a = crate::aim::AimState::blank();
        let steps: [(V3, V3, V3, f64, f64, f64, bool, u64, u64); 6] = [
            ([0.0, 0.0, 0.0], [3.0, 0.0, 7.0], [1.2, 0.0, -0.4], 1.0 / 60.0, 25.0, 0.15, false, 0x3fd9e9bf3d20dc70, 0x3f91d2da973e52f0),
            ([0.1, 0.0, 0.0], [3.05, 1.0, 7.02], [1.2, 0.0, -0.4], 1.0 / 60.0, 25.0, 0.15, false, 0x3fd975e93acd59a5, 0x3fa0e48fa7ebbb08),
            ([0.2, 0.0, 0.1], [-5.0, 1.0, -2.0], [4.7, 0.0, 0.3], 1.0 / 120.0, 25.0, 0.15, true, 0xbfff461946fe9f2c, 0x0000000000000000),
            ([0.2, 0.0, 0.1], [-5.01, 1.0, -2.03], [4.7, 0.0, 0.3], 1.0 / 120.0, 25.0, 0.0, false, 0xbfff579de4a1f697, 0x3fd10fe0ec0632ec),
            ([1.0, 0.0, 1.0], [1.0005, 0.0, 1.0002], [2.0, 0.0, 3.0], 1.0 / 60.0, 25.0, 0.15, true, 0x3ff30b6d796a4da8, 0x0000000000000000),
            ([1.0, 0.0, 1.0], [1.3, 0.0, 1.1], [0.3, 0.0, 0.3], 0.25, 18.0, 0.3, false, 0x3ff3fc176b7a855f, 0x3fcb7688e78f7a1f),
        ];
        for (from, at, vel, dt, jump, tau, jumped, brg, rate) in steps {
            assert_eq!(feed_target(&mut a, from, at, &vel, dt, jump, tau), jumped);
            assert_eq!(a.last_bearing.to_bits(), brg);
            assert_eq!(a.bearing_rate.to_bits(), rate);
        }
    }

    /// The TypeScript's own settleTime under EXACT: inside the range, at the floor, past it, and with the knobs at nothing.
    #[test]
    fn settle_time_keeps_the_typescript_s_bits() {
        let p = |settle_secs, range_ref, near_k| crate::aim::SettleTimeP { settle_secs, range_ref, near_k };
        assert_eq!(settle_time(p(1.3, 8.0, 0.25), 5.3).to_bits(), 0x3feb8f5c28f5c28f);
        assert_eq!(settle_time(p(1.3, 8.0, 0.25), 0.7).to_bits(), 0x3fd4cccccccccccd);
        assert_eq!(settle_time(p(0.8, 8.0, 0.25), 12.0).to_bits(), 0x3fe999999999999a);
        assert_eq!(settle_time(p(0.0, 0.0, 0.25), 3.3).to_bits(), 0x3f50624dd2f1a9fc);
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
        a.last_shot = Some(crate::aim::AimStateLastShot::zeroed());
        aim_state_clear(&mut a);
        assert_eq!(a.track, 0.0);
        assert!(a.tgt_key.is_empty() && a.last_bearing.is_nan() && !a.braking && a.fouled.is_none());
        assert_eq!(a.left_t, -100.0);
        assert!(a.last_shot.is_some());
    }
}
