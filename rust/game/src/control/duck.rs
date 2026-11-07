//! HEAD DOWN UNDER FIRE, as a rule of the legs list: the suppression module decides (suppression::head_down), this file says it in the list's words.
use super::{LegsOutcome, LegsWant};
use crate::game::Game;
use crate::ids::*;
use crate::suppression::HeadDown;

/// Rounds close enough, long enough: he gets down where he is, or shifts to the cover the rule found him.
pub fn ask(g: &mut Game, gd: GuardId, dt: f64) -> Option<LegsWant> {
    if !g.tune.suppression || crate::gunhold::standoff_on_feet(g, gd) { return None; }
    let now = crate::suppression::head_down(g, gd, dt);
    if matches!(now, HeadDown::None) { return None; }
    let fire_wait = g.k.suppression.head_down_delay;
    // where the rounds came from, while he still remembers
    let threat = g.gd[gd].sup.src.filter(|_| g.time - g.gd[gd].sup.src_t < g.k.suppression.src_memory_secs);
    if matches!(now, HeadDown::Down) { return Some(LegsWant { to: None, pace: 0.0, crouch: 1.0, fire_wait, threat }); }
    let cover = g.gd[gd].sup.cover.expect("a man shifting has his cover spot");
    Some(LegsWant { to: Some(cover), pace: g.tune.guard_investigate * g.k.suppression.displace_pace, crouch: g.k.suppression.displace_crouch, fire_wait, threat })
}

/// The shift ends where he arrives, where he is going nowhere, or when it has taken too long.
pub fn ran(g: &mut Game, gd: GuardId, o: LegsOutcome) {
    match o {
        LegsOutcome::Down => {}
        LegsOutcome::Arrived => crate::suppression::end_displace(g, gd, "there"),
        LegsOutcome::GoingNowhere => crate::suppression::end_displace(g, gd, "going nowhere"),
        LegsOutcome::Moving => if g.time - g.gd[gd].sup.displaced_at > g.k.suppression.displace_cap_secs { crate::suppression::end_displace(g, gd, "out of time"); },
    }
}
