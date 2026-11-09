//! HOW A MAN IS, whoever he is: what a blast, a flash, a blow and fire have left on him (`Cond`, on his body), what it costs him and what his figure shows of it.
pub use ::game_types::condition::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};
use alloc::{string::String, vec::Vec};

/// Is he down: dead, choked out, or a player out of hits.
pub fn down(g: &Game, c: CharId) -> bool {
    match g.ch[c].who {
        crate::character::Who::Guard(gd) => g.gd[gd].state == crate::game::GuardState::Dead,
        crate::character::Who::Player(pl) => g.pl[pl].down,
    }
}
/// Is this man hurt now; `now` is the clock asked of, the game's by default.
pub fn is_hurt(g: &Game, c: CharId, now: Option<f64>) -> bool {
    let (at, now) = (g.ch[c].cond.hurt_at, now.unwrap_or(g.time));
    if at < 0.0 || g.touring || g.puppet.is_some() { return false; }
    g.k.condition.hurt_secs <= 0.0 || now - at < g.k.condition.hurt_secs
}
/// Blind now (the fog, a can and a blow all leave the one clock).
pub fn is_blind(g: &Game, c: CharId) -> bool { g.time < g.ch[c].cond.dazzled_until }
/// Folded now: rooted, gun down, no round.
pub fn is_folded(g: &Game, c: CharId) -> bool { g.time < g.ch[c].cond.stagger_until }

/// A blast reached this man and left him up: he is hurt from now. False = not given.
pub fn condition_hurt(g: &mut Game, c: CharId) -> bool {
    let spared = match g.ch[c].who {
        crate::character::Who::Guard(gd) => g.gd[gd].civ.is_some() || g.gd[gd].god,
        crate::character::Who::Player(_) => g.god_mode,
    };
    if g.touring || g.puppet.is_some() || spared || down(g, c) { return false; }
    if !is_hurt(g, c, None) { g.stats.condition.hurt += 1; }
    g.ch[c].cond.hurt_at = g.time;
    true
}
/// Blind for `secs` more at the least, by a flash at `at`.
pub fn blind(g: &mut Game, c: CharId, secs: f64, at: Option<V3>) {
    let until = g.time + secs;
    let d = &mut g.ch[c].cond;
    d.dazzled_until = m::max(d.dazzled_until, until);
    if at.is_some() { d.dazzled_at = at; }
}
/// Folded for `secs` more at the least.
pub fn fold(g: &mut Game, c: CharId, secs: f64) {
    let until = g.time + secs;
    let d = &mut g.ch[c].cond;
    d.stagger_until = m::max(d.stagger_until, until);
}
/// Fire on him that is all it can be.
pub fn rattle(g: &mut Game, c: CharId) {
    let s = &mut g.ch[c].cond.sup;
    s.v = m::max(s.v, 1.0);
    s.kick = m::max(s.kick, 1.0);
}

/// WHAT A BLAST OR A FLASH TAKES OUT OF A MAN for `secs`, whoever did it: he looks for where it was, his legs stop and his way is forgotten.
pub fn dazed(g: &mut Game, gd: GuardId, secs: f64, at: V3, by: &'static str) {
    g.gd[gd].react_t = m::max(g.gd[gd].react_t, secs);
    crate::mind::raise(g, gd, 0.6, by);
    if g.gd[gd].state != crate::game::GuardState::Alert { crate::belief::word(g, gd, at); }
    crate::game::guard_set_path(g, gd, Vec::new());
    crate::guards::guard_set_path_goal(g, gd, None);
    g.gd[gd].speed = 0.0;
}
/// …by a flash: blind as well, and he remembers it.
pub fn dazzled(g: &mut Game, gd: GuardId, secs: f64, at: V3, by: &'static str) {
    let c = g.gd[gd].ch;
    blind(g, c, secs, None);
    dazed(g, gd, secs, at, by);
    if g.ai_enabled { g.gd[gd].witness.dazzled += 1.0; }
}

/// The speed a man's legs may be asked for: a hurt one's is capped.
pub fn condition_pace(g: &Game, c: CharId, speed: f64) -> f64 {
    if is_hurt(g, c, None) { m::min(speed, g.k.condition.hurt_pace) } else { speed }
}
/// The fire his rounds are opened by: never less than `hurt_sup` on a hurt man.
pub fn condition_sup(g: &Game, c: CharId, v: f64) -> f64 {
    if is_hurt(g, c, None) { m::max(v, g.k.condition.hurt_sup) } else { v }
}

/// What state a man is in, the stamps read as they stand.
fn condition_of(g: &Game, c: CharId) -> Condition {
    let down = down(g, c);
    Condition {
        shaken: !down && g.tune.suppression && g.ch[c].cond.sup.v >= g.k.suppression.flinch,
        blind: !down && is_blind(g, c),
        folded: !down && is_folded(g, c),
        hurt: !down && is_hurt(g, c, None),
        down,
    }
}

/// The panel's line: every man in anything but good order.
fn condition_summary(g: &Game, seat: &crate::seats::SeatRef) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut say = |name: &str, c: CharId| {
        let w = word(condition_of(g, c));
        if !w.is_empty() && !down(g, c) { parts.push(alloc::format!("{name} {w}")); }
    };
    say("you", g.pl[seat.player].ch);
    for &gd in g.guards.iter().filter(|&&gd| g.gd[gd].civ.is_none()) { say(&g.gd[gd].callsign, g.gd[gd].ch); }
    alloc::format!("condition \u{25b8} {}", if parts.is_empty() { String::from("all sound") } else { parts.join(" \u{b7} ") })
}

/// One man's states as words, in the panel's order.
fn word(c: Condition) -> String {
    [(c.shaken, "shaken"), (c.blind, "blind"), (c.folded, "folded"), (c.hurt, "hurt")].iter().filter(|x| x.0).map(|x| x.1).collect::<Vec<_>>().join(" ")
}

/// Every frame, after the stun poses are dealt: each man's condition to his figure, by one rule.
pub fn condition_pose(g: &mut Game) {
    let on = !g.touring && !crate::director::director_showing(g);
    let mut men: Vec<CharId> = crate::seats::men(g).iter().map(|&pl| g.pl[pl].ch).collect();
    men.extend(g.guards.iter().filter(|&&gd| g.gd[gd].civ.is_none()).map(|&gd| g.gd[gd].ch));
    for c in men { deal(g, on, c); }
}

/// One man's condition to his figure; off, whatever it put there comes off.
fn deal(g: &mut Game, on: bool, c: CharId) {
    if !on {
        let an = &mut g.ch[c].anim;
        if an.hurt != 0.0 || an.shaken != 0.0 || an.fold_too { (an.hurt, an.shaken, an.fold_too) = (0.0, 0.0, false); }
        return;
    }
    let up = !down(g, c);
    let hurt = if up && is_hurt(g, c, None) { 1.0 } else { 0.0 };
    let (flinch, pinned) = (g.k.suppression.flinch, g.k.suppression.pinned);
    let shaken = if up && g.tune.suppression { v3::clamp((g.ch[c].cond.sup.v - (flinch - 0.1)) / m::max(1e-3, pinned - (flinch - 0.1)), 0.0, 1.0) } else { 0.0 };
    let an = &mut g.ch[c].anim;
    (an.fold_too, an.hurt, an.shaken) = (true, hurt, shaken);
    an.hurt_side = -1.0; // his free hand is his left and goes across to his right ribs: the side he favours, every man
}

/// What a fold and a flash show on a man while they run: his hands up to his head, or turned from the light. `from` where no flash was kept
pub fn stun_pose(g: &mut Game, c: CharId, live: bool, from: V3) -> (bool, bool) {
    let (blind, folded) = (live && is_blind(g, c), live && is_folded(g, c));
    let k = m::min(1.0, (g.ch[c].cond.dazzled_until - g.time) / 0.5);
    let from = g.ch[c].cond.dazzled_at.unwrap_or(from);
    let an = &mut g.ch[c].anim;
    an.clutch = if folded && !blind { 1.0 } else { 0.0 };
    an.dazzle = if blind { Some(crate::character::AnimDazzle { from, k }) } else { None };
    (blind, folded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use crate::game::{Guard, GuardState, Player};

    fn game() -> (Game, GuardId, PlayerId, CharId, CharId) {
        let mut g = Game::blank();
        let gd = g.gd.push(Guard::blank());
        let pl = g.pl.push(Player::blank());
        let (cg, cp) = (g.ch.push(crate::character::Character::blank()), g.ch.push(crate::character::Character::blank()));
        (g.gd[gd].ch, g.pl[pl].ch) = (cg, cp);
        (g.ch[cg].who, g.ch[cp].who) = (crate::character::Who::Guard(gd), crate::character::Who::Player(pl));
        g.player = pl;
        g.guards = vec![gd];
        g.ch[g.gd[gd].ch].cond.hurt_at = -1.0;
        g.ch[g.pl[pl].ch].cond.hurt_at = -1.0;
        g.gd[gd].callsign = String::from("Reyes");
        g.time = 10.0;
        (g, gd, pl, cg, cp)
    }

    #[test]
    fn a_blast_leaves_a_man_hurt_and_writes_his_stamp() {
        let (mut g, gd, pl, cg, cp) = game();
        assert!(!is_hurt(&mut g, cg, None));
        assert!(condition_hurt(&mut g, cg));
        assert_eq!(g.ch[g.gd[gd].ch].cond.hurt_at, 10.0);
        assert!(condition_hurt(&mut g, cp));
        assert_eq!(g.ch[g.pl[pl].ch].cond.hurt_at, 10.0);
        assert_eq!(g.stats.condition.hurt, 2);
        g.time = 11.0;
        assert!(condition_hurt(&mut g, cg));
        assert_eq!(g.ch[g.gd[gd].ch].cond.hurt_at, 11.0);
        assert_eq!(g.stats.condition.hurt, 2, "a man already hurt is not counted again");
    }

    #[test]
    fn who_a_blast_cannot_hurt() {
        let (mut g, gd, pl, cg, cp) = game();
        g.gd[gd].god = true;
        assert!(!condition_hurt(&mut g, cg));
        g.gd[gd].god = false;
        g.gd[gd].state = GuardState::Dead;
        assert!(!condition_hurt(&mut g, cg));
        g.god_mode = true;
        assert!(!condition_hurt(&mut g, cp));
        g.god_mode = false;
        g.touring = true;
        assert!(!condition_hurt(&mut g, cp));
        assert_eq!(g.ch[g.pl[pl].ch].cond.hurt_at, -1.0);
        assert_eq!(g.ch[g.gd[gd].ch].cond.hurt_at, -1.0);
    }

    #[test]
    fn the_costs_and_how_long_they_last() {
        let (mut g, gd, _, cg, _) = game();
        assert_eq!(condition_pace(&mut g, cg, 2.2), 2.2);
        assert_eq!(condition_sup(&mut g, cg, 0.1), 0.1);
        g.ch[g.gd[gd].ch].cond.hurt_at = 9.0;
        assert_eq!(condition_pace(&mut g, cg, 2.2), g.k.condition.hurt_pace);
        assert_eq!(condition_pace(&mut g, cg, 1.15), 1.15);
        assert_eq!(condition_sup(&mut g, cg, 0.1), g.k.condition.hurt_sup);
        assert_eq!(condition_sup(&mut g, cg, 0.9), 0.9);
        g.k.condition.hurt_secs = 1.0;
        assert!(!is_hurt(&mut g, cg, None), "a limit: exactly at it he is sound");
        assert!(is_hurt(&mut g, cg, Some(9.5)));
    }

    #[test]
    fn the_panel_line() {
        let (mut g, gd, pl, cg, cp) = game();
        assert_eq!(condition_summary(&g, &crate::seats::here(&g)), "condition \u{25b8} all sound");
        g.ch[g.gd[gd].ch].cond.hurt_at = 9.0;
        g.ch[g.gd[gd].ch].cond.stagger_until = 11.0;
        g.ch[g.pl[pl].ch].cond.hurt_at = 9.0;
        assert_eq!(condition_summary(&g, &crate::seats::here(&g)), "condition \u{25b8} you hurt \u{b7} Reyes folded hurt");
        g.pl[pl].down = true;
        assert_eq!(condition_summary(&g, &crate::seats::here(&g)), "condition \u{25b8} Reyes folded hurt");
        assert!(condition_of(&mut g, cp).down);
        assert!(!condition_of(&mut g, cp).hurt);
    }

    #[test]
    fn the_figure_is_dealt_and_taken_off() {
        let (mut g, gd, _, cg, _) = game();
        let c = cg;
        g.ch[g.gd[gd].ch].cond.hurt_at = 9.0;
        g.ch[g.gd[gd].ch].cond.sup.v = g.k.suppression.pinned;
        deal(&mut g, true, cg);
        assert!(g.ch[c].anim.fold_too);
        assert_eq!((g.ch[c].anim.hurt, g.ch[c].anim.hurt_side, g.ch[c].anim.shaken), (1.0, -1.0, 1.0));
        deal(&mut g, false, cg);
        assert!(!g.ch[c].anim.fold_too);
        assert_eq!((g.ch[c].anim.hurt, g.ch[c].anim.shaken), (0.0, 0.0));
    }
}
