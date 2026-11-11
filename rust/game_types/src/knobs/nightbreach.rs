//! The numbers of nightbreach.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

/// The scenario's knobs, read at the next staging.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct NightbreachK {
    /// where Sam stands when it is staged, and the way he faces: the LEVEL's (tools/levels/lib/scenarios.ts TeamStart — staging writes the team start of the level it stages
    ///  on here; these are the office's until it has: beside the pair on the approach marks, in the cubicles' south aisle, facing east down it)
    #[k([9.6, 0.0, 20.3])] pub sam_at: V3,
    #[k(1.5707963267948966)] pub sam_yaw: f64,
    /// how many teammates are staged: 2 = the pair · 4 = the FOUR-MAN SQUAD: a RED pair and a BLUE pair (`squad4()`), each
    ///  man's pair in his papers (Guard.unit.pair; `squad_orders_men` is the one place to ask for the squad, a pair or a man). Red is the first pair: F's drill on the
    ///  breach plan's door is theirs, Blue stands by behind them until ordered; the order menu goes to all four, a pair or one man.
    ///  THE LOADOUT'S OTHER SIZES: 3 = the four's first three (the red pair and Blue One) · 1 = Blue One alone and 0 = nobody,
    ///  both with NO F drill even where the level has a breach plan (the drill takes two men): whoever came is the order menu's from the first frame.
    #[k(2.0)] pub squad: f64,
}
