//! SUSPICIOUS: a task, a found body, a look at where he thinks something is.
use super::{Acts, Sensed};
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use alloc::string::String;
use crate::m;
use crate::v3::{self, V3};
use crate::game::GuardState as St;
use crate::character::UpperMode;
use crate::game::{Errand, ErrandThing};

pub(in crate::people) fn suspicious(g: &mut Game, gd: GuardId, dt: f64, s: &Sensed) -> Acts {
    let &Sensed { c, staff, lvl, .. } = s;
    let upper = if hand_carry(g, gd) == crate::guards::InHand::Torch { UpperMode::Torch } else { UpperMode::Aim };
    let mut a = Acts { upper, ..Acts::still() };
    if matches!(g.gd[gd].errand, Some(Errand::CheckOn(_))) { check_on_tick(g, gd); }
    keeps_to_his_errand(g, gd);
    let duty = if g.gd[gd].body_duty.is_some() { body_aftermath(g, gd, dt) } else { None };
    if let Some(d) = duty {
        a.move_speed = d.move_;
        a.aim_at = d.aim;
    } else if g.gd[gd].react_t > 0.0 {
        g.gd[gd].react_t -= dt;
        brake(g, gd, 8.0, dt);
        if let Some(gp) = crate::belief::gun_point(g, gd) { a.aim_at = Some(v3::add(gp, [0.0, 1.0, 0.0])); }
    } else if let Some(gp) = crate::belief::gun_point(g, gd).filter(|_| g.gd[gd].duty.holds) {
        brake(g, gd, 8.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        a.aim_at = Some(v3::add(gp, [0.0, 1.0, 0.0]));
        g.gd[gd].search_t += dt;
        if g.gd[gd].search_t > 6.0 { crate::mind::cap(g, gd, 0.1, "guards"); }
    } else {
        goes_to_see(g, gd, c, dt, &mut a);
    }
    if g.gd[gd].awareness < 0.12 { calms_down(g, gd, c, staff, lvl); }
    g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, -0.12, 3.0, dt);
    a
}

/// An errand keeps his fix on its place and his suspicion up, and one not checking on a man is over once he has looked about a while.
fn keeps_to_his_errand(g: &mut Game, gd: GuardId) {
    let task_at = match &g.gd[gd].errand {
        Some(Errand::At(t)) => Some(t.pos),
        Some(Errand::CheckOn(t)) => Some(t.pos),
        None => None,
    };
    if let Some(at) = task_at {
        let off = match g.gd[gd].last_known { Some(lk) => v3::dist(lk, at) > 0.5, None => true };
        if off && g.gd[gd].awareness < 0.45 { crate::mind::fix(g, gd, Some(at), "guards"); }
        crate::mind::raise(g, gd, 0.2, "guards");
        if !matches!(g.gd[gd].errand, Some(Errand::CheckOn(_))) && g.gd[gd].search_t > 4.0 { g.gd[gd].errand = None; }
    }
}

/// Free to move: sent to the breaker he works its lever when he stands there, else he walks to his fix.
fn goes_to_see(g: &mut Game, gd: GuardId, c: CharId, dt: f64, a: &mut Acts) {
    let to_the_breaker = match &g.gd[gd].errand { Some(Errand::At(t)) => t.kind == ErrandThing::Breaker, _ => false };
    let lever = if to_the_breaker { crate::handwork::lever_stand(g, gd, dt) } else { None };
    if let Some(l) = lever {
        a.upper = UpperMode::None;
        g.gd[gd].speed = l.speed;
        a.move_speed = l.speed;
        g.ch[c].vel = [0.0, 0.0, 0.0];
        a.aim_at = None;
        g.ch[c].aim_yaw = l.yaw;
        g.ch[c].anim.look_yaw_extra = 0.0;
        g.gd[gd].stall_ref = None;
    } else if let Some(lk) = g.gd[gd].last_known {
        walks_to_his_fix(g, gd, c, lk, dt, a);
    }
}

/// To the place he thinks something is: there, his attention stays on it, or he looks about until he lets it go; a walk that gets nowhere moves him on.
fn walks_to_his_fix(g: &mut Game, gd: GuardId, c: CharId, lk: V3, dt: f64, a: &mut Acts) {
    go_to(g, gd, lk, None);
    let arrived = follow_path(g, gd, dt, g.k.tune.guard_investigate);
    a.move_speed = g.gd[gd].speed;
    // (gd.lastKnown read again: the walk's gates may have written a new fix)
    a.aim_at = g.gd[gd].last_known.map(|k| v3::add(k, [0.0, 0.9, 0.0]));
    if arrived && crate::attention::attention_dwell(g, gd) {
        g.gd[gd].stall_ref = None;
        g.gd[gd].sweep = m::sin(g.gd[gd].look_phase * 1.1) * g.k.attention.dwell_sweep;
        a.aim_at = None;
        g.ch[c].aim_yaw = g.ch[c].body_yaw + g.gd[gd].sweep;
    } else if arrived {
        g.gd[gd].stall_ref = None;
        g.gd[gd].search_t += dt;
        g.gd[gd].sweep = m::sin(g.gd[gd].look_phase * 1.1) * 1.1;
        g.ch[c].anim.look_yaw_extra = m::sin(g.gd[gd].look_phase * 1.9) * 0.4;
        a.aim_at = None;
        g.ch[c].aim_yaw = g.ch[c].body_yaw + g.gd[gd].sweep;
        if g.gd[gd].search_t > 5.0 { crate::mind::cap(g, gd, 0.2, "guards"); }
    } else {
        g.gd[gd].search_t = 0.0;
        if crate::whistle::whistle_soft(g, gd) { let to = g.k.whistle.walk_floor; crate::mind::raise(g, gd, to, "guards"); }
        crate::attention::attention_walk(g, gd);
        let free = match &g.gd[gd].errand { None => true, Some(Errand::CheckOn(_)) => true, _ => false };
        if free && not_getting_anywhere(g, gd, dt, 4.0) {
            if g.gd[gd].errand.is_some() {
                advance_check_on(g, gd);
            } else {
                let here = g.ch[c].pos;
                crate::mind::fix(g, gd, Some(here), "guards");
                g.gd[gd].stall_ref = None;
                g.gd[gd].drop_path();
            }
        }
    }
}

/// His suspicion has gone: back on patrol, and he says why, if there is anything to say.
fn calms_down(g: &mut Game, gd: GuardId, c: CharId, staff: bool, lvl: f64) {
    let d = g.gd[gd].body_duty.as_ref().map(|d| d.body);
    crate::mind::set_state(g, gd, St::Patrol, "guards");
    g.gd[gd].drop_path();
    g.gd[gd].body_duty = None;
    let player_down = crate::condition::down(g, g.pl[g.player].ch);
    if crate::teams::foe_is_player(g, gd) && player_down {
        if !g.stood_down {
            g.stood_down = true;
            let line = if g.gd[gd].unit.is_some() { "he's down — hold where you are" } else { "back to your posts" };
            crate::game::game_say(g, gd, line, Some(true), None);
        }
    } else if let Some(b) = d {
        if !g.gd[b].body_closed && g.gd[b].body_visits > 0.0 && v3::dist_xz(g.ch[c].pos, body_pos(g, b)) < 12.0 {
            g.gd[b].body_closed = true;
            let line = if crate::game::is_breathing(g, b) {
                alloc::format!("no sign of him. {} is breathing, he'll be okay — whoever dropped him is still close", g.gd[b].callsign)
            } else {
                String::from("no sign of him — eyes open, he's close")
            };
            crate::game::game_say(g, gd, &line, None, None);
        }
    } else if !staff {
        // a unit calms without the staff's platitudes
    } else {
        let p = g.ch[c].pos;
        if near_known_body(g, p, Some(4.0)).is_none() {
            let now = g.time;
            let radio_recent = match &g.gd[gd].bubble { Some(b) => b.radio && now - b.t < 4.0, None => false };
            if !radio_recent && !crate::attention::attention_told(g, gd) {
                let line = if lvl >= 1.0 { "clear here — staying on it" } else { "must have been nothing" };
                crate::game::game_say(g, gd, line, None, None);
            }
        }
    }
}
