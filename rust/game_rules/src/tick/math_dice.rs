#![allow(unused_imports, unused_variables)]
pub use ::game_types::math_dice::types;
pub use types::*;
use crate::game::Game;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use alloc::vec::Vec;

// TEMP-BRINGUP: WHAT STANDS IN Math.random's PLACE. A game with no die of its own (`Game.dice` null: the play-through harness's games) draws off the
// HOST's Math.random, which the harness has seeded. That stream is no part of the game's state, so the bring-up's host
// hands the tick the draws the TypeScript tick took, in order, and the Rust tick must take the same ones.
// SKELETON-GAP: a host seam for Math.random (`host_api::math_random(g) -> f64`), or a rule that every game compared has its own die.
struct Draws(core::cell::UnsafeCell<(Vec<f64>, usize)>);
unsafe impl Sync for Draws {}
static DRAWS: Draws = Draws(core::cell::UnsafeCell::new((Vec::new(), 0)));

/// the host's draws for the tick about to run
fn host_draws_set(v: &[f64]) { unsafe { *DRAWS.0.get() = (v.to_vec(), 0); } }
/// one draw off the host's stream; past its end is the Rust tick drawing where the TypeScript's did not
fn host_draw() -> f64 {
    unsafe {
        let d = &mut *DRAWS.0.get();
        if d.1 >= d.0.len() { panic!("bring-up: the Rust tick asks for draw {} off Math.random and the TypeScript tick took {}", d.1 + 1, d.0.len()); }
        d.1 += 1;
        d.0[d.1 - 1]
    }
}

/// The die's bit in the block's head: which systems' state is at home there.
const DICE_BIT: u32 = 1;

/// A die made and spent inside one call: the stream of `roll(die_of(a))`.
pub fn mulberry32(a: f64) -> crate::dice::Die {
    crate::dice::Die::new(js::to_u32(a))
}

/// A die whose whole state is one word on a record.
pub fn die_of(seed: f64) -> crate::math_dice::Die {
    Die { dice: js::to_i32(seed) as f64 }
}

/// One draw off a die kept in state, uniform in [0, 1).
pub fn roll(d: &mut crate::math_dice::Die) -> f64 {
    let a = js::to_i32(d.dice + 0x6D2B79F5_u32 as f64);
    d.dice = a as f64;
    let mut t = m::imul(a ^ ((a as u32 >> 15) as i32), 1 | a);
    t = t.wrapping_add(m::imul(t ^ ((t as u32 >> 7) as i32), 61 | t)) ^ t;
    (((t ^ ((t as u32 >> 14) as i32)) as u32) as f64) / 4294967296.0
}

/// FNV-1a, 32 bit, over UTF-16 code units: what is dealt by name.
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

/// Where the word stands. `home` keeps the block's word as sim_random leaves it.
pub fn sim_dice_s(this: &crate::math_dice::SimDice) -> f64 {
    match this.home {
        Some(h) => h.s,
        None => this.s0,
    }
}

/// How many draws have been taken.
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

/// A draw for the simulation, uniform in [0, 1): off the block where the die is at home, else off its own word; no die, Math.random.
pub fn sim_random(g: &mut crate::game::Game) -> f64 {
    if g.dice.is_none() {
        return seam_math_random();
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
fn seam_math_random() -> f64 {
    // TEMP-BRINGUP (b_01.rs): the host's own stream, handed in draw by draw
    self::host_draw()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A die on a record is its word: the same word goes on with the same draws, whatever came before it.
    #[test]
    fn a_die_is_its_word_and_its_draws_lie_across_the_range() {
        let (mut d, mut sum) = (die_of(12345.0), 0.0);
        let mut twin = die_of(12345.0);
        for i in 0..1000 {
            let r = roll(&mut d);
            assert!((0.0..1.0).contains(&r), "draw {i}: {r}");
            assert_eq!(r, roll(&mut twin));
            sum += r;
        }
        assert!(m::abs(sum / 1000.0 - 0.5) < 0.05, "the mean of a thousand draws is {}", sum / 1000.0);
        let mut again = Die { dice: d.dice };
        assert_eq!(roll(&mut d), roll(&mut again), "taken up again from its word");
        assert_ne!(roll(&mut die_of(1.0)), roll(&mut die_of(2.0)), "another seed is another stream");
        assert_eq!(roll(&mut die_of(4294967301.0)), roll(&mut die_of(5.0)), "a seed is its whole word: past 32 bits it wraps");
    }

    #[test]
    fn roll_is_rust_sims_stream() {
        let mut d = die_of(-7.0);
        let mut b = ::sim::dice::Dice { n: 0.0, s: js::to_u32(-7.0), spare: 0 };
        for _ in 0..1000 {
            assert_eq!(roll(&mut d).to_bits(), ::sim::dice::roll(&mut b).to_bits());
        }
    }

    /// What is dealt by name goes with the name, wherever the game runs.
    #[test]
    fn a_name_deals_one_whole_number_of_its_own() {
        let names = ["", "a", "b", "civdice:Ana", "civdice:Ben", "héllo", "日本語", "🙂x"];
        for (i, n) in names.iter().enumerate() {
            let h = hash_str(n);
            assert_eq!(h, hash_str(n));
            assert!(h >= 0.0 && h < 4294967296.0 && h == m::floor(h), "{n}: {h}");
            assert!(names[i + 1..].iter().all(|o| hash_str(o) != h), "{n} deals what another does");
        }
    }

    #[test]
    fn a_new_die_starts_at_its_seed_with_no_draws_taken() {
        for (seed, word, unsigned) in [(0.0, 0.0, 0.0), (5.0, 5.0, 5.0), (-1.0, -1.0, 4294967295.0), (4294967301.0, 5.0, 5.0), (2147483648.0, -2147483648.0, 2147483648.0)] {
            let d = sim_dice_new(seed);
            assert_eq!((sim_dice_s(&d), d.seed, sim_dice_n(&d)), (word, unsigned, 0.0), "seed {seed}");
            assert!(d.home.is_none());
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
        super::host_draws_set(&[0.25]);
        assert_eq!(sim_random(&mut g), 0.25);

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
