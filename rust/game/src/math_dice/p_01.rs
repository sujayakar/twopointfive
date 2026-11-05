//! src/math/dice.ts lines 1 to 63
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A closure die: the same stream as `roll(die_of(a))`.   (dice.ts mulberry32)
pub fn mulberry32(a: f64) -> crate::FnField {
    todo!("needs: a return type that can hold the word (FnField is empty): `-> Die`, with callers using roll")
}

/// A die whose whole state is one word on a record.   (dice.ts dieOf)
pub fn die_of(seed: f64) -> crate::math_dice::Die {
    Die { dice: js::to_i32(seed) as f64 }
}

/// One draw off a die kept in state, uniform in [0, 1).   (dice.ts roll)
pub fn roll(d: &mut crate::math_dice::Die) -> f64 {
    let a = js::to_i32(d.dice + 0x6D2B79F5_u32 as f64);
    d.dice = a as f64;
    let mut t = m::imul(a ^ ((a as u32 >> 15) as i32), 1 | a);
    t = t.wrapping_add(m::imul(t ^ ((t as u32 >> 7) as i32), 61 | t)) ^ t;
    (((t ^ ((t as u32 >> 14) as i32)) as u32) as f64) / 4294967296.0
}

/// `roll` as a method of the record that carries the word.   (dice.ts rollOn)
pub fn roll_on(g: &mut crate::game::Game) -> f64 {
    todo!("needs: the record's die as the parameter (`this: Die` became g): `roll_on(d: &mut Die)`, whose body is roll(d)")
}

/// FNV-1a, 32 bit, over UTF-16 code units: what is dealt by name.   (dice.ts hashStr)
pub fn hash_str(s: &str) -> f64 {
    let mut h: u32 = 0x811c9dc5;
    for c in s.encode_utf16() {
        h ^= c as u32;
        h = m::imul(h as i32, 0x01000193) as u32;
    }
    h as f64
}

/// The seeded die that stands in Math.random, remembering the one it displaced.   (dice.ts simDie)
pub fn sim_die(seed: f64, displaced: &mut dyn FnMut() -> f64) -> crate::FnField {
    todo!("needs: a die value that can hold its word and the displaced die (FnField is empty; a borrowed FnMut cannot be kept)")
}

/// A game's die at the start of `seed`'s stream.   (dice.ts SimDice.constructor; `sys` has no field: see seam_math_random_is_own)
pub fn sim_dice_new(seed: f64) -> crate::math_dice::SimDice {
    SimDice { s0: js::to_i32(seed) as f64, seed: js::to_u32(seed) as f64, ..SimDice::blank() }
}

/// Where the word stands.   (dice.ts SimDice.s)
pub fn sim_dice_s(this: &mut crate::math_dice::SimDice) -> f64 {
    match this.home {
        Some(h) => seam_home_s(h),
        None => this.s0,
    }
}

/// How many draws have been taken.   (dice.ts SimDice.n)
pub fn sim_dice_n(this: &mut crate::math_dice::SimDice) -> f64 {
    match this.home {
        Some(h) => seam_home_n(h),
        None => this.n0,
    }
}

/// Make `d` the die sim_random draws; returns the one it replaced. A die first put in use moves into the game's block (dicehome.ts `into`).   (dice.ts useDice)
pub fn use_dice(g: &mut crate::game::Game, d: Option<crate::math_dice::SimDice>) -> Option<crate::math_dice::SimDice> {
    let p = g.m.math_dice.cur;
    match d {
        None => g.m.math_dice.cur = None,
        Some(n) => {
            let same = match p { Some(o) => o.seed.to_bits() == n.seed.to_bits(), None => false };
            if !same {
                let b = &mut ::sim::state::state(g.sim).dice;
                b.s = js::to_u32(n.s0);
                b.n = n.n0;
                g.m.math_dice.cur = Some(SimDice { home: Some(DiceHome::zeroed()), ..n });
            }
        }
    }
    p
}

/// The die in use; None also while somebody else's die stands in Math.random's place.   (dice.ts diceInUse)
pub fn dice_in_use(g: &mut crate::game::Game) -> Option<crate::math_dice::SimDice> {
    let cur = g.m.math_dice.cur;
    if cur.is_some() && seam_math_random_is_own(g) { cur } else { None }
}

/// A draw for the simulation, uniform in [0, 1): the game's own die, in rust/sim's block.   (dice.ts simRandom)
pub fn sim_random(g: &mut crate::game::Game) -> f64 {
    if g.m.math_dice.cur.is_none() || !seam_math_random_is_own(g) {
        return seam_math_random(g);
    }
    ::sim::dice::roll_of(g.sim)
}

/// dice.ts:45 `h.s`
fn seam_home_s(h: DiceHome) -> f64 {
    todo!("needs: g on SimDice.s: a die at home has its word in sim::state::state(g.sim).dice.s")
}

/// dice.ts:47 `h.n`
fn seam_home_n(h: DiceHome) -> f64 {
    todo!("needs: g on SimDice.n: a die at home has its count in sim::state::state(g.sim).dice.n")
}

/// dice.ts:56,59 `Math.random === d.sys`
fn seam_math_random_is_own(g: &mut Game) -> bool {
    // TEMP-BRINGUP: nothing in the Rust game puts another die in Math.random's place (with_dice and the journal's die are the host's), so a die in use is the game's own
    true
}

/// dice.ts:59 `Math.random()`
fn seam_math_random(g: &mut Game) -> f64 {
    // TEMP-BRINGUP (b_01.rs): the host's own stream, handed in draw by draw
    super::b_01::host_draw()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NEW_ROWS: &[(f64, u64, u64)] = &[
        (0.0, 0x0000000000000000, 0x0000000000000000),
        (-1.0, 0xbff0000000000000, 0x41efffffffe00000),
        (4294967301.0, 0x4014000000000000, 0x4014000000000000),
        (2147483648.0, 0xc1e0000000000000, 0x41e0000000000000),
        (1.5, 0x3ff0000000000000, 0x3ff0000000000000),
        (-7.9, 0xc01c000000000000, 0x41efffffff200000),
    ];

    // the TypeScript's own bits: bun, EXACT.on, dieOf then roll four times; the word after them last
    const ROLL_ROWS: &[(f64, u64, [u64; 4], u64)] = &[
        (0.0, 0x0000000000000000, [0x3fd10d2d18800000, 0x3f359c3700000000, 0x3fcc942d84000000, 0x3fc2b6c082000000], 0xc1d2d4860b000000),
        (1.0, 0x3ff0000000000000, [0x3fe410fd5e600000, 0x3f66693920000000, 0x3fe0e0d89d600000, 0x3fef64c4ffa00000], 0xc1d2d4860ac00000),
        (-1.0, 0xbff0000000000000, [0x3fecaf7e7a600000, 0x3fc840d2d2000000, 0x3fe6e6a072000000, 0x3fee35bd20800000], 0xc1d2d4860b400000),
        (12345.0, 0x40c81c8000000000, [0x3fef59ef18a00000, 0x3fd3a1d440000000, 0x3fdefd38bc800000, 0x3fea2c84ca000000], 0xc1d2d479fcc00000),
        (4294967301.0, 0x4014000000000000, [0x3fe612a2d5800000, 0x3fe8ba5019e00000, 0x3fcc2131bf000000, 0x3fe3f114b6a00000], 0xc1d2d48609c00000),
        (2147483648.0, 0xc1e0000000000000, [0x3fea422bdf800000, 0x3fdcadd12ec00000, 0x3fe91357f9800000, 0x3fe062addda00000], 0x41ca56f3ea000000),
        (1.5, 0x3ff0000000000000, [0x3fe410fd5e600000, 0x3f66693920000000, 0x3fe0e0d89d600000, 0x3fef64c4ffa00000], 0xc1d2d4860ac00000),
        (-7.9, 0xc01c000000000000, [0x3fdbb7600d400000, 0x3fd4d348c4c00000, 0x3fe16aa7df000000, 0x3fdebb6ecfc00000], 0xc1d2d4860cc00000),
        (360021.0, 0x4115f95400000000, [0x3f99d105f4000000, 0x3fea4b5758c00000, 0x3fe90b527be00000, 0x3fdb26f879400000], 0xc1d2d32675c00000),
        (1e20, 0x41d8c40000000000, [0x3fc97f34e7800000, 0x3fe612fe5cc00000, 0x3fd030cea7800000, 0x3fcbc2ee99000000], 0x41b7bde7d4000000),
        (f64::NAN, 0x0000000000000000, [0x3fd10d2d18800000, 0x3f359c3700000000, 0x3fcc942d84000000, 0x3fc2b6c082000000], 0xc1d2d4860b000000),
    ];

    #[test]
    fn roll_is_the_typescripts_bits() {
        for &(seed, word, draws, after) in ROLL_ROWS {
            let mut d = die_of(seed);
            assert_eq!(d.dice.to_bits(), word, "seed {seed}");
            for x in draws {
                assert_eq!(roll(&mut d).to_bits(), x, "seed {seed}");
            }
            assert_eq!(d.dice.to_bits(), after, "seed {seed}");
        }
    }

    #[test]
    fn roll_is_rust_sims_stream() {
        let mut d = die_of(-7.0);
        let mut b = ::sim::dice::Dice { n: 0.0, s: js::to_u32(-7.0), spare: 0 };
        for _ in 0..1000 {
            assert_eq!(roll(&mut d).to_bits(), ::sim::dice::roll(&mut b).to_bits());
        }
    }

    #[test]
    fn hash_is_the_typescripts_bits() {
        let rows: &[(&str, u64)] = &[
            ("", 0x41e02393b8a00000),
            ("a", 0x41ec818525800000),
            ("civdice:Ana", 0x41da78918d800000),
            ("héllo", 0x41ee3cb6abe00000),
            ("日本語", 0x41d5018dd3800000),
            ("🙂x", 0x41e557ef32400000),
        ];
        for &(s, h) in rows {
            assert_eq!(hash_str(s).to_bits(), h, "{s}");
        }
    }

    #[test]
    fn new_die_is_the_typescripts_bits() {
        for &(seed, s0, sd) in NEW_ROWS {
            let mut d = sim_dice_new(seed);
            assert_eq!(d.s0.to_bits(), s0, "seed {seed}");
            assert_eq!(d.seed.to_bits(), sd, "seed {seed}");
            assert_eq!(d.n0.to_bits(), 0);
            assert!(d.home.is_none());
            assert_eq!(sim_dice_s(&mut d).to_bits(), s0);
            assert_eq!(sim_dice_n(&mut d).to_bits(), 0);
        }
    }

    // the trap: Game.update calls useDice(this.dice) every frame, and in Rust that is a stale copy: it must not rewind the word
    #[test]
    fn use_dice_moves_the_word_once() {
        let mut g = Game::blank();
        let die = sim_dice_new(-7.0);
        assert!(use_dice(&mut g, Some(die)).is_none());
        let b = &mut ::sim::state::state(g.sim).dice;
        assert_eq!(b.s, js::to_u32(-7.0));
        ::sim::dice::roll(b);
        let (s, n) = (b.s, b.n);
        assert!(use_dice(&mut g, Some(die)).is_some());
        let b = &::sim::state::state(g.sim).dice;
        assert_eq!((b.s, b.n), (s, n));
        assert!(use_dice(&mut g, None).is_some());
        assert!(g.m.math_dice.cur.is_none());
    }
}
