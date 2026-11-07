//! HEAD DOWN UNDER FIRE: rounds close enough, long enough, and he gets down where he is, or shifts to the cover the suppression rule found him.
//! The rule's own code and state are suppression/ (`gd.sup`); this file says it in the list's words.
use super::{Outcome, Reaction, TakeCover};
use crate::game::Game;
use crate::ids::*;
use crate::suppression::HeadDown;

/// what the suppression rule answered this tick (it answers once: asking it moves its clocks and may draw its die)
#[derive(Clone, Copy, Debug, Default)]
pub struct Row { pub want: Option<TakeCover> }

pub struct Rule;
impl Reaction for Rule {
    type Row = Row;
    type Ask = TakeCover;

    fn transition(g: &mut Game, gd: GuardId, dt: f64) {
        let want = asked(g, gd, dt);
        super::state_mut(g).rows.duck.at(gd).want = want;
    }

    fn propose(g: &Game, gd: GuardId) -> Option<TakeCover> { super::state(g).and_then(|s| s.rows.duck.get(gd)).and_then(|r| r.want) }

    /// The shift ends where he arrives, where he is going nowhere, or when it has taken too long.
    fn commit(g: &mut Game, gd: GuardId, o: Outcome) {
        match o {
            Outcome::Arrived => crate::suppression::end_displace(g, gd, "there"),
            Outcome::GoingNowhere => crate::suppression::end_displace(g, gd, "going nowhere"),
            Outcome::Moving => if g.time - g.gd[gd].sup.displaced_at > g.k.suppression.displace_cap_secs { crate::suppression::end_displace(g, gd, "out of time"); },
            Outcome::Down | Outcome::Script(_) => {}
        }
    }
}

fn asked(g: &mut Game, gd: GuardId, dt: f64) -> Option<TakeCover> {
    if !g.tune.suppression || crate::gunhold::standoff_on_feet(g, gd) { return None; }
    let now = crate::suppression::head_down(g, gd, dt);
    if matches!(now, HeadDown::None) { return None; }
    let fire_wait = g.k.suppression.head_down_delay;
    // where the rounds came from, while he still remembers
    let threat = g.gd[gd].sup.src.filter(|_| g.time - g.gd[gd].sup.src_t < g.k.suppression.src_memory_secs);
    if matches!(now, HeadDown::Down) { return Some(TakeCover { fire_wait, threat, ..TakeCover::down() }); }
    let cover = g.gd[gd].sup.cover.expect("a man shifting has his cover spot");
    let pace = g.tune.guard_investigate * g.k.suppression.displace_pace;
    Some(TakeCover { crouch: g.k.suppression.displace_crouch, fire_wait, threat, ..TakeCover::to(cover, pace) })
}
