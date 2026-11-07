//! THE GAME'S DIE as a value a state can hold: the same stream as rust/sim's (it rolls rust/sim's own function), with the derives a cloned
//! state needs. `simRandom()` in the TypeScript is `g.dice.roll()` here; a die kept on a record (`Civ.dice`) is this type too.
use sim::dice::Dice;
use alloc::vec::Vec;
use crate::game::Game;
use crate::ids::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Die {
    /// draws taken off it
    pub n: f64,
    /// the mulberry32 word
    pub s: u32,
}
impl Die {
    pub const fn new(seed: u32) -> Die {
        Die { n: 0.0, s: seed }
    }
    /// one draw, uniform in [0, 1)   (dice.ts simRandom / rollOn)
    pub fn roll(&mut self) -> f64 {
        let mut d = Dice { n: self.n, s: self.s, spare: 0 };
        let r = sim::dice::roll(&mut d);
        self.n = d.n;
        self.s = d.s;
        r
    }
}

/// KEYED DRAWS. The game's one die is a single sequence: a draw added anywhere moves every draw after it, for every man. A keyed draw is a
/// fixed function of the game's seed, of the SITE that asks (its name: "guards::alert#1"), of the MAN it is about (0: nobody, the world's
/// own), of the tick's clock, and of how many times that site has drawn for that man this tick. So a rule's draws do not depend on what any
/// other rule or any other man drew, and a rule added tomorrow re-deals nothing but itself. (`draw` below and its forms by a man's id.)
///
/// `seen` is this tick's count a (site, man): empty between ticks, so nothing of it need be saved.
#[derive(Clone, Debug, Default)]
pub struct Keyed {
    /// the clock of the tick `seen` belongs to
    pub t: f64,
    pub seen: Vec<(u64, u32)>,
}
impl Keyed {
    /// how many times (site, who) has drawn this tick before now; counts this one
    pub fn nth(&mut self, now: f64, key: u64) -> u32 {
        if self.t != now { self.t = now; self.seen.clear(); }
        match self.seen.iter_mut().find(|x| x.0 == key) {
            Some(x) => { x.1 += 1; x.1 }
            None => { self.seen.push((key, 0)); 0 }
        }
    }
}

/// a site's name as a word (FNV-1a, 64 bit)
pub fn site_word(site: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in site.bytes() { h ^= b as u64; h = h.wrapping_mul(0x0000_0100_0000_01b3); }
    h
}

/// one word well stirred (the finaliser of splitmix64)
fn stir(mut x: u64) -> u64 {
    x ^= x >> 30; x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27; x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

/// (site, who) as one key
pub fn key_of(site: &str, who: u32) -> u64 { stir(site_word(site) ^ stir(who as u64 ^ 0x9e37_79b9_7f4a_7c15)) }

/// THE DRAW: uniform in [0, 1), a fixed function of its five arguments.
pub fn keyed(seed: u32, key: u64, time: f64, nth: u32) -> f64 {
    let x = stir(stir(stir(seed as u64 ^ 0x2545_f491_4f6c_dd1d) ^ key) ^ stir(time.to_bits()) ^ ((nth as u64) << 1 | 1).wrapping_mul(0xd6e8_feb8_6659_fd93));
    (stir(x) >> 11) as f64 / 9_007_199_254_740_992.0
}

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
}
