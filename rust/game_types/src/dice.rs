//! THE DICE. Every draw of a game is keyed: a fixed function of its seed and of who asks what when (`Keyed`). A die kept on a record (`Civ.dice`, a squad's) is a word of its own, and
//! moves with nobody else's.
use crate::bytes::ToBytes;
use alloc::vec::Vec;

/// A DIE ON A RECORD, its whole state one word: mulberry32
#[derive(Clone, Copy, PartialEq, Debug, Default, ToBytes)]
pub struct Die(pub u32);
impl Die {
    /// one draw, uniform in [0, 1)
    pub fn roll(&mut self) -> f64 {
        let a = self.0.wrapping_add(0x6D2B79F5);
        self.0 = a;
        let mut t = (a ^ (a >> 15)).wrapping_mul(1 | a);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        ((t ^ (t >> 14)) as f64) / 4294967296.0
    }
    /// what the draw after the next `n` will be
    pub fn ahead(mut self, n: u32) -> f64 {
        for _ in 0..n { self.roll(); }
        self.roll()
    }
}

/// what a name is dealt by: FNV-1a, 32 bit, over its UTF-16 code units
pub fn hash_str(s: &str) -> f64 {
    let mut h: u32 = 0x811c9dc5;
    for c in s.encode_utf16() {
        h ^= c as u32;
        h = h.wrapping_mul(0x01000193);
    }
    h as f64
}

/// THE GAME'S OWN DRAWS: the seed they are keyed by, and how many it has taken (what two machines of a game, or two plays of a run, are compared by first)
#[derive(Clone, Copy, PartialEq, Debug, Default, ToBytes)]
pub struct Draws {
    pub seed: u32,
    pub n: f64,
}

/// KEYED DRAWS. One die for a whole game is a single sequence: a draw added anywhere moves every draw after it, for every man. A keyed draw is a
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

