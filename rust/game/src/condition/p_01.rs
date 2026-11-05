//! src/game/condition.ts lines 1 to 114
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A guard rather than a player.   (condition.ts isGuard)
pub fn is_guard(g: &mut crate::game::Game, m: crate::condition::Man) -> bool {
    matches!(m, Man::Guard(_))
}

/// Is this man hurt now; `now` is the clock asked of, the game's by default.   (condition.ts isHurt)
pub fn is_hurt(g: &mut crate::game::Game, m: crate::condition::Man, now: Option<f64>) -> bool {
    let now = now.unwrap_or(g.time);
    let hurt_at = match m {
        Man::Guard(id) => g.gd[id].hurt_at,
        Man::Player(id) => g.pl[id].hurt_at,
    };
    if hurt_at < 0.0 || g.touring || g.puppet.is_some() { return false; }
    g.k.condition.hurt_secs <= 0.0 || now - hurt_at < g.k.condition.hurt_secs
}

/// A blast reached this man and left him up: he is hurt from now. False = not given.   (condition.ts conditionHurt)
pub fn condition_hurt(g: &mut crate::game::Game, m: &mut crate::condition::Man) -> bool {
    let man = *m;
    if g.touring || g.puppet.is_some() { return false; }
    let refused = match man {
        Man::Guard(id) => g.gd[id].civ.is_some() || g.gd[id].god || g.gd[id].state == crate::game::GuardState::Dead,
        Man::Player(id) => g.god_mode || g.pl[id].down,
    };
    if refused { return false; }
    if !is_hurt(g, man, None) { g.stats.condition.hurt += 1; }
    let now = g.time;
    match man {
        Man::Guard(id) => g.gd[id].hurt_at = now,
        Man::Player(id) => g.pl[id].hurt_at = now,
    }
    true
}

/// The speed a man's legs may be asked for: a hurt one's is capped.   (condition.ts conditionPace)
pub fn condition_pace(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 {
    if is_hurt(g, Man::Guard(gd), None) { m::min(speed, g.k.condition.hurt_pace) } else { speed }
}

/// The fire his rounds are opened by: never less than `hurt_sup` on a hurt man.   (condition.ts conditionSup)
pub fn condition_sup(g: &mut crate::game::Game, m: crate::condition::Man, v: f64) -> f64 {
    if is_hurt(g, m, None) { m::max(v, g.k.condition.hurt_sup) } else { v }
}

/// The score leaves this man's wariness at rest out of its tension: one of the player's own.   (condition.ts conditionCalm)
pub fn condition_calm(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative
}

/// What state a man is in, the stamps read as they stand.   (condition.ts conditionOf)
pub fn condition_of(g: &mut crate::game::Game, m: crate::condition::Man) -> crate::condition::Condition {
    let (down, sup, dazzled_until, stagger_until) = man_marks(g, m);
    Condition {
        shaken: !down && g.tune.suppression && sup >= g.k.suppression.flinch,
        blind: !down && g.time < dazzled_until,
        folded: !down && g.time < stagger_until,
        hurt: !down && is_hurt(g, m, None),
        down,
    }
}

/// A man's down flag, fire on him, and the stamps of his blinding and his fold.
fn man_marks(g: &mut Game, m: Man) -> (bool, f64, f64, f64) {
    match m {
        Man::Guard(id) => {
            let gd = &g.gd[id];
            (gd.state == crate::game::GuardState::Dead, gd.sup.v, gd.dazzled_until, gd.stagger_until)
        }
        Man::Player(id) => {
            let pl = &g.pl[id];
            (pl.down, pl.sup.v, pl.dazzled_until, pl.stagger_until)
        }
    }
}

/// The panel's line: every man in anything but good order.   (condition.ts conditionSummary)
pub fn condition_summary(g: &mut crate::game::Game) -> String {
    let mut parts: Vec<String> = Vec::new();
    let me = g.player;
    let mine = word(condition_of(g, Man::Player(me)));
    if !mine.is_empty() && !g.pl[me].down {
        let mut part = String::from("you ");
        part.push_str(&mine);
        parts.push(part);
    }
    // nothing here changes the list: a copy walks the same men.
    for gd in g.guards.clone() {
        if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].civ.is_some() { continue; }
        let w = word(condition_of(g, Man::Guard(gd)));
        if !w.is_empty() {
            let mut part = g.gd[gd].callsign.clone();
            part.push(' ');
            part.push_str(&w);
            parts.push(part);
        }
    }
    let mut out = String::from("condition \u{25b8} ");
    if parts.is_empty() { out.push_str("all sound"); } else { out.push_str(&parts.join(" \u{b7} ")); }
    out
}

/// One man's states as words, in the panel's order.   (condition.ts conditionSummary's word)
fn word(c: Condition) -> String {
    let mut words: Vec<&str> = Vec::new();
    if c.shaken { words.push("shaken"); }
    if c.blind { words.push("blind"); }
    if c.folded { words.push("folded"); }
    if c.hurt { words.push("hurt"); }
    words.join(" ")
}

/// Every frame, after the stun poses are dealt: each man's condition to his figure, by one rule.   (condition.ts conditionPose)
pub fn condition_pose(g: &mut crate::game::Game) {
    let on = !g.touring && !crate::director::director_showing(g);
    let view = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    for pl in crate::seats::men(g, &view) { deal(g, on, Man::Player(pl)); }
    // deal changes no list: a copy walks the same men.
    for gd in g.guards.clone() {
        if g.gd[gd].civ.is_none() { deal(g, on, Man::Guard(gd)); }
    }
}

/// One man's condition to his figure; off, whatever it put there comes off.   (condition.ts conditionPose's deal)
fn deal(g: &mut Game, on: bool, m: Man) {
    let c = match m {
        Man::Guard(id) => g.gd[id].ch,
        Man::Player(id) => g.pl[id].ch,
    };
    if !on {
        let an = &mut g.ch[c].anim;
        if an.hurt != 0.0 || an.shaken != 0.0 || an.fold_too {
            an.hurt = 0.0;
            an.shaken = 0.0;
            an.fold_too = false;
        }
        return;
    }
    let (down, sup, _, _) = man_marks(g, m);
    let hurt = if !down && is_hurt(g, m, None) { 1.0 } else { 0.0 };
    let (flinch, pinned) = (g.k.suppression.flinch, g.k.suppression.pinned);
    let shaken = if down || !g.tune.suppression {
        0.0
    } else {
        v3::clamp((sup - (flinch - 0.1)) / m::max(1e-3, pinned - (flinch - 0.1)), 0.0, 1.0)
    };
    let an = &mut g.ch[c].anim;
    an.fold_too = true;
    an.hurt = hurt;
    an.hurt_side = -1.0; // his free hand is his left and goes across to his right ribs: the side he favours, every man
    an.shaken = shaken;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Guard, GuardState, Player};

    fn game() -> (Game, GuardId, PlayerId) {
        let mut g = Game::blank();
        let gd = g.gd.push(Guard::blank());
        let pl = g.pl.push(Player::blank());
        g.player = pl;
        g.guards = vec![gd];
        g.gd[gd].hurt_at = -1.0;
        g.pl[pl].hurt_at = -1.0;
        g.gd[gd].callsign = String::from("Reyes");
        g.time = 10.0;
        (g, gd, pl)
    }

    #[test]
    fn a_blast_leaves_a_man_hurt_and_writes_his_stamp() {
        let (mut g, gd, pl) = game();
        assert!(!is_hurt(&mut g, Man::Guard(gd), None));
        assert!(condition_hurt(&mut g, &mut Man::Guard(gd)));
        assert_eq!(g.gd[gd].hurt_at, 10.0);
        assert!(condition_hurt(&mut g, &mut Man::Player(pl)));
        assert_eq!(g.pl[pl].hurt_at, 10.0);
        assert_eq!(g.stats.condition.hurt, 2);
        g.time = 11.0;
        assert!(condition_hurt(&mut g, &mut Man::Guard(gd)));
        assert_eq!(g.gd[gd].hurt_at, 11.0);
        assert_eq!(g.stats.condition.hurt, 2, "a man already hurt is not counted again");
    }

    #[test]
    fn who_a_blast_cannot_hurt() {
        let (mut g, gd, pl) = game();
        g.gd[gd].god = true;
        assert!(!condition_hurt(&mut g, &mut Man::Guard(gd)));
        g.gd[gd].god = false;
        g.gd[gd].state = GuardState::Dead;
        assert!(!condition_hurt(&mut g, &mut Man::Guard(gd)));
        g.god_mode = true;
        assert!(!condition_hurt(&mut g, &mut Man::Player(pl)));
        g.god_mode = false;
        g.touring = true;
        assert!(!condition_hurt(&mut g, &mut Man::Player(pl)));
        assert_eq!(g.pl[pl].hurt_at, -1.0);
        assert_eq!(g.gd[gd].hurt_at, -1.0);
    }

    #[test]
    fn the_costs_and_how_long_they_last() {
        let (mut g, gd, _) = game();
        assert_eq!(condition_pace(&mut g, gd, 2.2), 2.2);
        assert_eq!(condition_sup(&mut g, Man::Guard(gd), 0.1), 0.1);
        g.gd[gd].hurt_at = 9.0;
        assert_eq!(condition_pace(&mut g, gd, 2.2), g.k.condition.hurt_pace);
        assert_eq!(condition_pace(&mut g, gd, 1.15), 1.15);
        assert_eq!(condition_sup(&mut g, Man::Guard(gd), 0.1), g.k.condition.hurt_sup);
        assert_eq!(condition_sup(&mut g, Man::Guard(gd), 0.9), 0.9);
        g.k.condition.hurt_secs = 1.0;
        assert!(!is_hurt(&mut g, Man::Guard(gd), None), "a limit: exactly at it he is sound");
        assert!(is_hurt(&mut g, Man::Guard(gd), Some(9.5)));
    }

    #[test]
    fn the_panel_line() {
        let (mut g, gd, pl) = game();
        assert_eq!(condition_summary(&mut g), "condition \u{25b8} all sound");
        g.gd[gd].hurt_at = 9.0;
        g.gd[gd].stagger_until = 11.0;
        g.pl[pl].hurt_at = 9.0;
        assert_eq!(condition_summary(&mut g), "condition \u{25b8} you hurt \u{b7} Reyes folded hurt");
        g.pl[pl].down = true;
        assert_eq!(condition_summary(&mut g), "condition \u{25b8} Reyes folded hurt");
        assert!(condition_of(&mut g, Man::Player(pl)).down);
        assert!(!condition_of(&mut g, Man::Player(pl)).hurt);
    }

    #[test]
    fn the_figure_is_dealt_and_taken_off() {
        let (mut g, gd, _) = game();
        let c = g.ch.push(crate::character::Character::blank());
        g.gd[gd].ch = c;
        g.gd[gd].hurt_at = 9.0;
        g.gd[gd].sup.v = g.k.suppression.pinned;
        deal(&mut g, true, Man::Guard(gd));
        assert!(g.ch[c].anim.fold_too);
        assert_eq!((g.ch[c].anim.hurt, g.ch[c].anim.hurt_side, g.ch[c].anim.shaken), (1.0, -1.0, 1.0));
        deal(&mut g, false, Man::Guard(gd));
        assert!(!g.ch[c].anim.fold_too);
        assert_eq!((g.ch[c].anim.hurt, g.ch[c].anim.shaken), (0.0, 0.0));
    }
}
