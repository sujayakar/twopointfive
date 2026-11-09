//! THE GAME'S DIE as a value a state can hold: the same stream as rust/sim's (it rolls rust/sim's own function), with the derives a cloned
//! state needs. `simRandom()` in the TypeScript is `g.dice.roll()` here; a die kept on a record (`Civ.dice`) is this type too.
use crate::bytes::ToBytes;
use sim::dice::Dice;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Debug, Default, ToBytes)]
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
#[derive(Clone, Debug, Default, ToBytes)]
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

#[cfg(test)]
mod tests {
    use super::*;
    /// A thousand draws folded to one word. tools/qa/dice-check.ts folds the same thousand of src/math/dice.ts to the same word.
    #[test]
    fn the_keyed_draw_is_the_pages() {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for i in 0..1000u32 {
            let v = keyed(i.wrapping_mul(2_654_435_761), key_of(&alloc::format!("m::f#{i}"), i % 37), i as f64 / 60.0, i % 5);
            assert!((0.0..1.0).contains(&v));
            h = (h ^ v.to_bits()).wrapping_mul(0x0000_0100_0000_01b3);
        }
        assert_eq!(h, 0x698f_1e04_1d58_8223, "{h:#x}");
    }
}
