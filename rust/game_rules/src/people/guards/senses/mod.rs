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

/// what `sense` made of the tick, for `notice` and `fade`
pub struct Sense { lvl: f64, eyes: Option<foe::Eyes> }

/// Are his ears and eyes kept somewhere else? A squad man under an order has the room fight's (roomfight/operative.rs, its meters), and a man of the response
/// team under its script the team's own (qrf `perceive_scripted`).
fn kept_elsewhere(g: &Game, gd: GuardId) -> bool {
    g.gd[gd].script.is_some() && (g.gd[gd].team == crate::teams::Team::Operative || crate::qrf::qrf_owns(g, gd))
}

/// HIS EARS AND EYES, every tick, whoever has his body.
pub fn sense(g: &mut Game, gd: GuardId, dt: f64, staff: bool) -> Option<Sense> {
    if kept_elsewhere(g, gd) { return None; }
    let lvl = crate::guards::escalation_of(g);
    if !g.ai_enabled || crate::doze::doze_deaf(g, gd) { return Some(Sense { lvl, eyes: None }); }
    let ear = crate::doze::doze_ear(g, gd);
    hearing::hearing_check(g, gd, Some(ear[0]), Some(true), Some(ear[1]));
    let eyes = foe::eyes(g, gd, dt, staff);
    Some(Sense { lvl, eyes: Some(eyes) })
}

/// WHAT HE DOES ABOUT WHAT IS AROUND HIM where nothing has his body: where his man went, a colleague on the floor, the man he trails, a kicked lock.
pub fn notice(g: &mut Game, gd: GuardId, dt: f64, c: CharId, staff: bool, s: &Sense) -> Saw {
    let lvl = s.lvl;
    g.gd[gd].drawn = lvl >= 1.0 || !staff;
    if let Some(eyes) = &s.eyes {
        lead::lead(g, gd, c, eyes.body);
        body::body(g, gd, c, staff, lvl);
        leader::leader(g, gd, lvl);
        lock::lock(g, gd, c, staff);
    }
    fade(g, gd, dt, s);
    s.eyes.as_ref().map_or(Saw { lvl, seen: false, dist: 99.0 }, |e| Saw { lvl, seen: e.seen, dist: e.dist })
}

/// What he does not see fades.
pub fn fade(g: &mut Game, gd: GuardId, dt: f64, s: &Sense) {
    match &s.eyes {
        None => calm(g, gd, dt * 0.5),
        Some(e) if !e.seen => {
            let by = dt * (match g.gd[gd].state { St::Alert => 0.03, St::Patrol => 0.1, _ => 0.06 });
            calm(g, gd, by);
        }
        _ => {}
    }
}

fn calm(g: &mut Game, gd: GuardId, by: f64) {
    let to = m::max(0.0, g.gd[gd].awareness - by);
    crate::mind::set(g, gd, to, "guards");
}
