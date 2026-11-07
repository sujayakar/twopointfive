//! WHAT ONE MAN SENSES THIS TICK, a file a sense, in the order they run: each takes in what it can and moves his awareness and his fix through `mind`.
//! A man with no AI, or with headphones on, senses nothing and calms.
use crate::game::{Game, GuardState as St};
use crate::ids::*;
use crate::m;

pub mod body;
pub mod foe;
pub mod hearing;
pub mod leader;
pub mod lead;
pub mod lock;

/// what his ears and eyes told him: the building's escalation, whether his eyes have his man, and how far
pub struct Saw { pub lvl: f64, pub seen: bool, pub dist: f64 }

pub fn perceive(g: &mut Game, gd: GuardId, dt: f64, c: CharId, staff: bool) -> Saw {
    let lvl = crate::guards::escalation_of(g);
    g.gd[gd].drawn = lvl >= 1.0 || !staff;
    if !g.ai_enabled || crate::doze::doze_deaf(g, gd) {
        calm(g, gd, dt * 0.5);
        return Saw { lvl, seen: false, dist: 99.0 };
    }
    let ear = crate::doze::doze_ear(g, gd);
    hearing::hearing_check(g, gd, Some(ear[0]), Some(true), Some(ear[1]));
    let eyes = foe::eyes(g, gd, dt, staff);
    lead::lead(g, gd, c, eyes.body);
    body::body(g, gd, c, staff, lvl);
    leader::leader(g, gd, lvl);
    lock::lock(g, gd, c, staff);
    if !eyes.seen {
        let by = dt * (match g.gd[gd].state { St::Alert => 0.03, St::Patrol => 0.1, _ => 0.06 });
        calm(g, gd, by);
    }
    Saw { lvl, seen: eyes.seen, dist: eyes.dist }
}

/// what he does not see fades
fn calm(g: &mut Game, gd: GuardId, by: f64) {
    let to = m::max(0.0, g.gd[gd].awareness - by);
    crate::mind::set(g, gd, to, "guards");
}
