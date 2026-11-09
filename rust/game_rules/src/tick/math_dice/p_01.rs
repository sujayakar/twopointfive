//! src/math/dice.ts lines 1 to 63
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The die's bit in the block's head: which systems' state is at home there.   (dicehome.ts simTenant's `bit`)
const DICE_BIT: u32 = 1;

/// A die made and spent inside one call: the stream of `roll(die_of(a))`.   (dice.ts mulberry32)
pub fn mulberry32(a: f64) -> crate::dice::Die {
    crate::dice::Die::new(js::to_u32(a))
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

/// FNV-1a, 32 bit, over UTF-16 code units: what is dealt by name.   (dice.ts hashStr)
pub fn hash_str(s: &str) -> f64 {
    let mut h: u32 = 0x811c9dc5;
    for c in s.encode_utf16() {
        h ^= c as u32;
        h = m::imul(h as i32, 0x01000193) as u32;
    }
    h as f64
}

/// A game's die at the start of `seed`'s stream.   (dice.ts SimDice.constructor; `sys` has no field: see sim_random)
pub fn sim_dice_new(seed: f64) -> crate::math_dice::SimDice {
    SimDice { s0: js::to_i32(seed) as f64, seed: js::to_u32(seed) as f64, ..SimDice::blank() }
}

/// Where the word stands. `home` keeps the block's word as sim_random leaves it.   (dice.ts SimDice.s)
pub fn sim_dice_s(this: &crate::math_dice::SimDice) -> f64 {
    match this.home {
        Some(h) => h.s,
        None => this.s0,
    }
}

/// How many draws have been taken.   (dice.ts SimDice.n)
pub fn sim_dice_n(this: &crate::math_dice::SimDice) -> f64 {
    match this.home {
        Some(h) => h.n,
        None => this.n0,
    }
}

/// As a game's frame opens: `g.dice` is the die sim_random draws, and it moves into or out of the game's block as RUST.dice wishes.
/// (dice.ts useDice; dicehome.ts wants, into, outOf, with state.ts settle's test of the bit: neither has a part)
pub fn use_dice(g: &mut crate::game::Game) {
    let want = g.m.sim_sim.dice_on && g.dice.is_some();
    let st = ::sim::state::state(g.sim);
    let have = st.head.homes & DICE_BIT != 0;
    if want == have {
        return;
    }
    if want {
        let d = g.dice.as_mut().unwrap();
        let (s, n) = (sim_dice_s(d), sim_dice_n(d));
        st.dice.s = js::to_u32(s);
        st.dice.n = n;
        st.head.homes |= DICE_BIT;
        d.home = Some(DiceHome { s: js::to_i32(s) as f64, n });
    } else {
        st.head.homes &= !DICE_BIT;
        if let Some(d) = g.dice.as_mut() {
            let (s, n) = (sim_dice_s(d), sim_dice_n(d));
            d.home = None;
            d.s0 = js::to_i32(s) as f64;
            d.n0 = n;
            st.dice = ::sim::dice::Dice { n: 0.0, s: 0, spare: 0 };
        }
    }
}

/// The die in use: the game's own, since nothing in the Rust game can stand in Math.random's place.   (dice.ts diceInUse)
pub fn dice_in_use(g: &crate::game::Game) -> Option<crate::math_dice::SimDice> {
    g.dice
}

/// A draw for the simulation, uniform in [0, 1): off the block where the die is at home, else off its own word; no die, Math.random.   (dice.ts simRandom)
pub fn sim_random(g: &mut crate::game::Game) -> f64 {
    if g.dice.is_none() {
        return seam_math_random(g);
    }
    let slot = g.sim;
    let d = g.dice.as_mut().unwrap();
    if let Some(h) = d.home.as_mut() {
        let r = ::sim::dice::roll_of(slot);
        let b = &::sim::state::state_read(slot).dice;
        h.s = b.s as i32 as f64;
        h.n = b.n;
        return r;
    }
    d.n0 += 1.0;
    let mut word = Die { dice: d.s0 };
    let r = roll(&mut word);
    d.s0 = word.dice;
    r
}

/// dice.ts:59 `Math.random()`
fn seam_math_random(g: &Game) -> f64 {
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

    #[test]
    fn mulberry32_is_the_records_stream() {
        for seed in [0.0, 1.0, -1.0, 12345.0, 4294967301.0, 2147483648.0, -7.9, 1e20, f64::NAN] {
            let (mut a, mut b) = (mulberry32(seed), die_of(seed));
            for _ in 0..100 {
                assert_eq!(a.roll().to_bits(), roll(&mut b).to_bits(), "seed {seed}");
            }
        }
    }

    // one test: slot 0's block is shared. The trap: Game.update calls useDice every frame and must not rewind the word.
    #[test]
    fn the_die_moves_rolls_and_comes_back() {
        let mut g = Game::blank();
        let mut stream = mulberry32(-7.0);
        ::sim::state::state(g.sim).head.homes = 0;
        ::sim::state::state(g.sim).dice = ::sim::dice::Dice { n: 0.0, s: 0, spare: 0 };

        // no die: nothing moves, and the draw is the host's
        use_dice(&mut g);
        assert_eq!(::sim::state::state(g.sim).head.homes & DICE_BIT, 0);
        super::super::b_01::host_draws_set(&[0.25]);
        assert_eq!(sim_random(&mut g), 0.25);
        assert_eq!(super::super::b_01::host_draws_count(), (1, 1));

        // a die not yet at home rolls its own word (a game's construction)
        g.dice = Some(sim_dice_new(-7.0));
        assert_eq!(sim_random(&mut g).to_bits(), stream.roll().to_bits());
        assert_eq!(g.dice.unwrap().n0, 1.0);

        // the move carries the word whole, once, and every frame after leaves it
        use_dice(&mut g);
        assert_eq!(::sim::state::state(g.sim).head.homes & DICE_BIT, DICE_BIT);
        assert_eq!(::sim::state::state(g.sim).dice.s, stream.s);
        assert_eq!(sim_random(&mut g).to_bits(), stream.roll().to_bits());
        use_dice(&mut g);
        assert_eq!(sim_random(&mut g).to_bits(), stream.roll().to_bits());
        let mut d = g.dice.unwrap();
        assert_eq!(sim_dice_s(&mut d).to_bits(), (stream.s as i32 as f64).to_bits());
        assert_eq!(sim_dice_n(&mut d), 3.0);
        assert_eq!(dice_in_use(&mut g).unwrap().seed.to_bits(), d.seed.to_bits());

        // a die imported at home with its block not brought: the word is the one `home` carries, not the stale s0
        ::sim::state::state(g.sim).head.homes = 0;
        ::sim::state::state(g.sim).dice = ::sim::dice::Dice { n: 0.0, s: 0, spare: 0 };
        use_dice(&mut g);
        assert_eq!(sim_random(&mut g).to_bits(), stream.roll().to_bits());

        // the row off: home again on the die, its part of the block zeros, the same stream
        g.m.sim_sim.dice_on = false;
        use_dice(&mut g);
        let d = g.dice.unwrap();
        assert!(d.home.is_none());
        assert_eq!(d.n0, 4.0);
        assert_eq!(::sim::state::state(g.sim).head.homes & DICE_BIT, 0);
        assert_eq!((::sim::state::state(g.sim).dice.s, ::sim::state::state(g.sim).dice.n), (0, 0.0));
        assert_eq!(sim_random(&mut g).to_bits(), stream.roll().to_bits());
    }
}
