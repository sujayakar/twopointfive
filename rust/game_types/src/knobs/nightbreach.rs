//! The numbers of nightbreach.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

/// The scenario's knobs (Tab ▸ Sandbox ▸ Night breach, __nightbreach.knobs; read at the next staging — 'restage' applies them).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct NightbreachK {
    /// the defenders' weapon lights
    #[k(false)] pub def_lights: bool,
    /// where Sam stands when it is staged, and the way he faces: the LEVEL's (scene/scenarios.ts TeamStart — start() writes the team start of the level it stages
    ///  on here; these are the office's until it has: beside the pair on the approach marks, in the cubicles' south aisle, facing east down it)
    #[k([9.6, 0.0, 20.3])] pub sam_at: V3,
    #[k(1.5707963267948966)] pub sam_yaw: f64,
    /// how many teammates are staged: 2 = the pair, exactly as it always was · 4 = the FOUR-MAN SQUAD (?squad4): a RED pair and a BLUE pair (SQUAD4 below), each
    ///  man's pair in his papers (Guard.unit.pair; orders.ts men() is the one place to ask for the squad, a pair or a man). Red is the first pair: F's drill on the
    ///  breach plan's door is theirs, Blue stands by behind them until ordered; the order menu goes to all four, a pair or one man (orders.ts `who`: T).
    ///  THE LOADOUT'S OTHER SIZES (game/loadout.ts; the page's ?squad=N): 3 = the four's first three (the red pair and Blue One) · 1 = Blue One alone and 0 = nobody,
    ///  both with NO F drill even where the level has a breach plan (the drill takes two men): whoever came is the order menu's from the first frame.
    #[k(2.0)] pub squad: f64,
}
