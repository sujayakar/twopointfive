//! CALM: his post, his partner, a need, his route.
use super::{Acts, Sensed};
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use crate::m;
use crate::v3;
use crate::game::GuardState as St;
use crate::character::UpperMode;

pub(in crate::people) fn patrol(g: &mut Game, gd: GuardId, dt: f64, s: &Sensed) -> Acts {
    let &Sensed { c, route, staff, lvl, .. } = s;
    let Acts { mut move_speed, mut upper, .. } = Acts::still();
    let target = g.tab.route[route].points.get(g.gd[gd].wp).copied();
    if g.gd[gd].drawn { upper = UpperMode::Aim; }
    if g.gd[gd].body_duty.is_some() && g.gd[gd].awareness < 0.3 { g.gd[gd].body_duty = None; }
    if matches!(g.gd[gd].errand, Some(crate::game::Errand::CheckOn(_))) { g.gd[gd].errand = None; }
    let walk = g.k.tune.guard_walk * (if lvl >= 1.0 { 1.15 } else { 1.0 });
    if g.gd[gd].duty.holds {
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(g.gd[gd].look_phase * 0.5) * 0.2, 2.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        brake(g, gd, 8.0, dt);
    } else if lvl >= 2.0 && g.gd[gd].leader.is_some() {
        partner_watch(g, gd);
        if follow_leader(g, gd, dt, walk) { move_speed = g.gd[gd].speed; }
    } else if lvl >= 2.0 && g.gd[gd].post.is_some() {
        if hold_post(g, gd, dt, walk) { move_speed = g.gd[gd].speed; }
    } else if g.k.tune.needs && crate::needs::needs_patrol(g, gd, dt, walk) {
        move_speed = g.gd[gd].speed;
    } else if crate::doze::doze_patrol(g, gd, dt, walk) {
        move_speed = g.gd[gd].speed;
    } else if g.gd[gd].wait > 0.0 {
        g.gd[gd].wait -= dt;
        g.gd[gd].sweep = m::sin(g.gd[gd].look_phase * 0.9) * 0.7;
        g.ch[c].anim.look_yaw_extra = m::sin(g.gd[gd].look_phase * 1.3) * 0.5;
        if g.gd[gd].wait <= 0.0 { g.gd[gd].wp = (g.gd[gd].wp + 1) % g.tab.route[route].points.len(); }
    } else {
        go_to(g, gd, target.expect("a walked route has this point"), None);
        let mut pace = walk;
        if lvl >= 2.0 {
            for i in 0..g.guards.len() {
                let o = g.guards[i];
                if g.gd[o].leader == Some(gd) && g.gd[o].state == St::Patrol && v3::dist(g.pos_of(o), g.ch[c].pos) > 4.0 { pace = walk * 0.55; }
            }
        }
        if follow_path(g, gd, dt, pace) {
            let w = g.tab.route[route].wait.get(g.gd[gd].wp).copied().unwrap_or(0.0);
            g.gd[gd].wait = w * (if lvl >= 1.0 { 0.6 } else { 1.0 });
            if g.gd[gd].wait <= 0.0 { g.gd[gd].wp = (g.gd[gd].wp + 1) % g.tab.route[route].points.len(); }
        }
        let k = if g.gd[gd].drawn { 0.4 } else { 0.25 };
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(g.gd[gd].look_phase * 0.7) * k, 2.0, dt);
        move_speed = g.gd[gd].speed;
    }
    g.ch[c].aim_yaw = g.ch[c].body_yaw + g.gd[gd].sweep;
    let low = if g.gd[gd].drawn { -0.26 } else { -0.35 };
    g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, low, 3.0, dt);
    if !g.gd[gd].duty.holds && staff { lockdown_doors(g, gd, Some(lvl < 2.0)); }
    crate::whistle::whistle_patrol_tick(g, gd);
    Acts { move_speed, upper, ..Acts::still() }
}
