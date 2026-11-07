//! The keyed draws of a game (the data's `dice` says what they are): they step the game's die, so they stand above the data.
pub use game_wire::dice::*;
use crate::game::Game;
use crate::ids::*;
#[cfg(test)] use sim::dice::Dice;

/// A KEYED DRAW for the simulation: uniform in [0, 1), a fixed function of the game's seed, the `site` that asks, the man it is about (`who`: 0
/// for nobody, else a body's id + 1), the tick, and how many times this site has drawn for him this tick. The block's die is still stepped
/// once a draw, so its count goes on witnessing how many draws a run made; its value is not used.
pub fn draw(g: &mut Game, site: &'static str, who: u32) -> f64 {
    let old = crate::math_dice::sim_random(g);
    let Some(d) = g.dice.as_ref() else { return old };   // (no die: the host's stream, as before)
    let seed = crate::js::to_u32(d.seed);
    let now = g.time;
    let key = key_of(site, who);
    let nth = g.keyed.nth(now, key);
    keyed(seed, key, now, nth)
}
/// WHOM A DRAW IS ABOUT. A draw about a man is his own: it does not move with what other men drew this tick, nor with
/// the order the men were walked in. A helper that draws (`pick(g, who, range)`) is handed this by its caller.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Who {
    /// nobody: the world's own (a prop, a thrown thing's tumble, a sound's pitch, a deal of the whole floor)
    World,
    /// a man of the floor
    Man(GuardId),
    /// a body
    Body(CharId),
    /// a defender or an operative, by his row of the room fight
    Def(RoomfightDefId),
    Op(RoomfightOpId),
    /// a group that decides as one (a squad), by its number
    Group(u32),
}
impl Who {
    /// the word a draw is keyed by: 0 for nobody, a body's id + 1 for a man however he was named, a group's number apart from both
    pub fn word(self, g: &Game) -> u32 {
        match self {
            Who::World => 0,
            Who::Man(gd) => g.gd[gd].ch.0 + 1,
            Who::Body(c) => c.0 + 1,
            Who::Def(d) => g.gd[g.tab.roomfight_def[d].gd].ch.0 + 1,
            Who::Op(o) => g.gd[g.tab.roomfight_op[o].gd].ch.0 + 1,
            Who::Group(n) => 0x4000_0000 | n,
        }
    }
}
/// a keyed draw about whoever the caller says
pub fn draw_for(g: &mut Game, site: &'static str, who: Who) -> f64 { let w = who.word(g); draw(g, site, w) }
/// a keyed draw about a man of the floor
pub fn draw_of(g: &mut Game, site: &'static str, gd: GuardId) -> f64 { let who = g.gd[gd].ch.0 + 1; draw(g, site, who) }
/// …about a body
pub fn draw_body(g: &mut Game, site: &'static str, c: CharId) -> f64 { draw(g, site, c.0 + 1) }
/// …about a defender, by his row of the room fight
pub fn draw_def(g: &mut Game, site: &'static str, d: RoomfightDefId) -> f64 { let gd = g.tab.roomfight_def[d].gd; draw_of(g, site, gd) }
/// …about an operative, by his row of the room fight
pub fn draw_op(g: &mut Game, site: &'static str, o: RoomfightOpId) -> f64 { let gd = g.tab.roomfight_op[o].gd; draw_of(g, site, gd) }
/// …about nobody: the world's own
pub fn draw_world(g: &mut Game, site: &'static str) -> f64 { draw(g, site, 0) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_stream_as_sim() {
        let mut a = Die::new(12345);
        let mut b = Dice { n: 0.0, s: 12345, spare: 0 };
        for _ in 0..100 {
            assert_eq!(a.roll().to_bits(), sim::dice::roll(&mut b).to_bits());
        }
        assert_eq!(a.n, 100.0);
        let mut c = a;
        assert_eq!(a.roll().to_bits(), c.roll().to_bits());
    }

    #[test]
    fn a_keyed_draw_is_a_function_of_its_key_and_nothing_else() {
        let k = key_of("guards::alert#1", 7);
        let a = keyed(42, k, 12.5, 0);
        assert_eq!(a.to_bits(), keyed(42, k, 12.5, 0).to_bits());
        assert!(a >= 0.0 && a < 1.0);
        // another seed, site, man, tick or turn is another draw
        for other in [keyed(43, k, 12.5, 0), keyed(42, key_of("guards::alert#2", 7), 12.5, 0), keyed(42, key_of("guards::alert#1", 8), 12.5, 0), keyed(42, k, 12.5 + 1.0 / 60.0, 0), keyed(42, k, 12.5, 1)] {
            assert_ne!(a.to_bits(), other.to_bits());
        }
    }

    #[test]
    fn keyed_draws_are_spread_evenly_over_men_and_over_ticks() {
        // one site: 64 men over 600 ticks, in ten bins
        let mut bins = [0u32; 10];
        let mut sum = 0.0;
        for who in 1..=64u32 {
            let k = key_of("a site", who);
            for tick in 0..600 { let r = keyed(1, k, tick as f64 / 60.0, 0); assert!(r >= 0.0 && r < 1.0); bins[(r * 10.0) as usize] += 1; sum += r; }
        }
        let n = 64.0 * 600.0;
        assert!((sum / n - 0.5).abs() < 0.01, "the mean of {} draws is {}", n, sum / n);
        for b in bins { assert!((b as f64 - n / 10.0).abs() < n / 10.0 * 0.08, "a tenth of the range holds {} of {} draws", b, n); }
        // a man's draws from one tick to the next are not a ramp
        let k = key_of("a site", 3);
        let ups = (0..1000).filter(|&t| keyed(1, k, (t + 1) as f64 / 60.0, 0) > keyed(1, k, t as f64 / 60.0, 0)).count();
        assert!(ups > 430 && ups < 570, "{} of 1000 steps go up", ups);
    }

    #[test]
    fn the_same_site_asked_twice_in_a_tick_for_one_man_draws_twice() {
        let mut seen = Keyed::default();
        let k = key_of("a site", 1);
        assert_eq!((seen.nth(1.0, k), seen.nth(1.0, k), seen.nth(1.0, key_of("a site", 2)), seen.nth(1.0, k)), (0, 1, 0, 2));
        assert_eq!(seen.nth(1.0 + 1.0 / 60.0, k), 0, "a new tick starts the count again");
        assert_eq!(seen.seen.len(), 1);
    }
    #[test]
    fn a_man_is_one_man_however_the_caller_named_him() {
        let mut g = alloc::boxed::Box::new(Game::blank());
        let mut body = crate::character::Character::blank();
        body.pos = [0.0, 0.0, 0.0];
        g.ch.push(crate::character::Character::blank());
        let c = g.ch.push(body);
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        let gd = g.gd.push(man);
        assert_eq!(Who::Man(gd).word(&g), Who::Body(c).word(&g));
        assert_eq!(Who::World.word(&g), 0);
        // a group's number is never a body's
        assert_ne!(Who::Group(c.0 + 1).word(&g), Who::Body(c).word(&g));
        assert_ne!(Who::Group(0).word(&g), Who::World.word(&g));
    }
}
