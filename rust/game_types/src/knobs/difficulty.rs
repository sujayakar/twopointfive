//! The numbers of difficulty.
use crate::bytes::ToBytes;
use crate::js::*;
use alloc::string::String;
use game_derive::Knobs;

/// what HARD is beside 'both switches on': the reach that joins them (m on foot; 0 = no reach rule — the press takes whom it wants; whether the men it leaves
///  at home dig in is `keeperDigs`). Picked by measurement: 18 left the office EASIER than standard, 0 clearly harder
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HardK {
    #[k(0.0)] pub reach_m: f64,
    /// `stuns` 0: stun charges a defender carries (pressure.ts `PRESSURE.stuns`). None in the build's hard; The block's hard gives one each — `HARD_AT` below
    #[k(0.0)] pub stuns: f64,
}

/// WHAT HARD IS ON ONE LEVEL beside the table above: rows of the press written over hard's while the game stands on that level's structure (the name of its
///  `scenarios.roomClear` block), so a rule measured on one level leaves every other level's hard as it was, run for run. roomclear.ts start() says where the
///  game stands (`difficultyHere`); the row the card shows still reads hard there, since `tablesOf` carries these.
///  The block: A STUN CHARGE EACH. Step three of the order the maintainer set (smarter enemies, then where they start, then kit, then more of them): the first two were
///  measured there and neither showed.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct HardAtK {
    #[k({ let mut t = JsMap::new(); t.set(String::from("the block"), Some(1.0)); t })] pub rows: JsMap<String, Option<f64>>,
}
