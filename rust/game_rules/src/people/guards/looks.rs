//! THE LOOKS LAID OVER HIS AIM, as his pose is made: where some module has him look while his behaviour goes on. Asked in the list's order, each over the one before:
//! the last that has a place to look is where he looks, and he is as low as the lowest of them.
//!
//! TO ADD A LOOK: a line of `LOOKS`.
use super::behave::Acts;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::V3;

/// where he is to look (None: where he did) and how low
pub struct Laid { pub aim: Option<V3>, pub crouch: f64 }

/// a look: asked of a man with the tick's length and whether his eyes are on his man; None: nothing of his
pub struct Look { pub name: &'static str, pub ask: fn(&mut Game, GuardId, f64, bool) -> Option<Laid> }

pub const LOOKS: &[Look] = &[
    Look { name: "a place his tactics watch", ask: tactics },
    Look { name: "a dark door", ask: dark_door },
    Look { name: "quiet on the net", ask: net_quiet },
];

fn tactics(g: &mut Game, gd: GuardId, _dt: f64, eyes_on_foe: bool) -> Option<Laid> {
    if !g.tune.tactics || eyes_on_foe { return None; }
    crate::tactics::tactics_look(g, gd).map(|l| Laid { aim: Some(l.aim), crouch: l.crouch })
}

fn dark_door(g: &mut Game, gd: GuardId, _dt: f64, eyes_on_foe: bool) -> Option<Laid> {
    if eyes_on_foe { return None; }
    crate::darkdoor::dark_door_look(g, gd).map(|at| Laid { aim: Some(at), crouch: 0.0 })
}

fn net_quiet(g: &mut Game, gd: GuardId, dt: f64, eyes_on_foe: bool) -> Option<Laid> {
    crate::netquiet::net_quiet_pose(g, gd, dt, eyes_on_foe).map(|l| Laid { aim: l.aim, crouch: l.crouch })
}

/// every look of the list laid over what his behaviour decided
pub(super) fn lay(g: &mut Game, gd: GuardId, dt: f64, eyes_on_foe: bool, acts: &mut Acts) {
    for l in LOOKS {
        if let Some(laid) = (l.ask)(g, gd, dt, eyes_on_foe) {
            if laid.aim.is_some() { acts.aim_at = laid.aim; }
            acts.crouch = m::max(acts.crouch, laid.crouch);
        }
    }
}
