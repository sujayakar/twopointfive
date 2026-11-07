//! QUESTIONS A RULE MAY ASK OF A MAN. Each is one line, takes `&Game` (asking changes nothing) and says in its comment what it does NOT
//! tell you. A rule's `propose` uses these before it reads a man's fields; a question that is missing is added here, not written in the rule.
use crate::game::{Game, GuardState};
use crate::ids::*;

/// He is dead. (A man down and bleeding, or one who has fled the level, is not: `down_kind`, `fled`.)
pub fn dead(g: &Game, gd: GuardId) -> bool { g.gd[gd].state == GuardState::Dead }

/// He fights the player's side, by the game's one table of who fights whom (teams::is_hostile). A civilian and a teammate are not; the night
/// staff of a quiet mission are.
pub fn enemy(g: &Game, gd: GuardId) -> bool { crate::teams::is_hostile(g, g.gd[gd].team, crate::teams::Team::Player) }

/// He cannot see. It does not say WHAT took his eyes: a flash, a wall charge, a door thrown in his face and the door tools all write it.
pub fn blind(g: &Game, gd: GuardId) -> bool { g.time < g.gd[gd].dazzled_until }

/// He is reeling from a blow or a blast: he neither aims nor fires until it passes.
pub fn staggered(g: &Game, gd: GuardId) -> bool { g.time < g.gd[gd].stagger_until }

/// Somebody's arm is on him: a hostage hold or a grab.
pub fn held(g: &Game, gd: GuardId) -> bool { g.gd[gd].held.is_some() }

/// He has had his man in his eyes within the last `secs` seconds. ALERT DOES NOT MEAN THIS: a bang alone makes a man Alert, so a rule that
/// means "in a fight" asks this.
pub fn saw_his_man_within(g: &Game, gd: GuardId, secs: f64) -> bool { g.time - g.gd[gd].last_seen_t < secs }

/// He has a hostage in his arms and a gun to them.
pub fn in_a_standoff(g: &Game, gd: GuardId) -> bool { g.gd[gd].standoff.is_some() }

/// His hands are changing magazine.
pub fn changing_magazine(g: &Game, gd: GuardId) -> bool { g.gd[gd].reload_t >= 0.0 }

/// His legs are still (under a tenth of a metre a second over the floor). It does not say why: at his post, braked by a rule, or stuck.
pub fn still(g: &Game, gd: GuardId) -> bool {
    let v = g.ch[g.gd[gd].ch].vel;
    v[0] * v[0] + v[2] * v[2] < 0.01
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    #[test]
    fn a_man_as_he_is_made_is_none_of_these_and_each_clock_turns_its_own_answer() {
        let mut g = Box::new(Game::blank());
        let c = g.ch.push(crate::character::Character::blank());
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        let gd = g.gd.push(man);
        g.time = 10.0;
        let all = |g: &Game| [dead(g, gd), blind(g, gd), staggered(g, gd), held(g, gd), saw_his_man_within(g, gd, 5.0), in_a_standoff(g, gd), changing_magazine(g, gd)];
        assert_eq!(all(&g), [false; 7]);
        assert!(still(&g, gd));
        g.gd[gd].dazzled_until = 10.5;
        assert_eq!(all(&g), [false, true, false, false, false, false, false]);
        g.gd[gd].dazzled_until = 10.0;                    // the moment it ends he can see
        g.gd[gd].stagger_until = 11.0;
        g.gd[gd].last_seen_t = 6.0;
        g.gd[gd].reload_t = 0.0;
        assert_eq!(all(&g), [false, false, true, false, true, false, true]);
        assert!(!saw_his_man_within(&g, gd, 4.0));
        g.ch[c].vel = [0.3, 0.0, 0.0];
        assert!(!still(&g, gd));
    }
}
