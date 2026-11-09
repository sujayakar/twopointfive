//! THE ADDRESS'S WORDS FOR THE GAME'S NUMBERS. A word here turns a knob as the game is made, over what the machine keeps. `?knob.<name>=<number>` turns any number of the
//! tables by its name (game_types `knobs`), and these are the short words people already use. `?training=` and `?nerve=` pick as the panel's rows do (actions.rs).
use game::game::Game;
use game::query::Query;

/// what the word's presence or value does to its knob
#[derive(Clone, Copy)]
pub enum Says {
    /// `?word=0` (or `off`) sets it to 0
    AtZero,
    /// `?word` sets it to 0, whatever its value
    IfThere,
    /// `?word=0` sets it to this: the number it was before its rule
    ZeroIs(f64),
    /// `?word=<a number above 0>` sets it to that, 1 at most
    Share,
}
pub struct Word { pub word: &'static str, pub knob: &'static str, pub says: Says }

pub static WORDS: &[Word] = &[
    Word { word: "qtwice", knob: "cmdring.twice", says: Says::AtZero },
    Word { word: "goalhalf", knob: "cmdring.goal_half", says: Says::AtZero },
    Word { word: "doorbrisk", knob: "teampace.door_brisk", says: Says::AtZero },
    Word { word: "pacing", knob: "teampace.pacing", says: Says::AtZero },
    Word { word: "wallpress", knob: "wallsnap.press", says: Says::AtZero },
    Word { word: "wallskin", knob: "wallskin.on", says: Says::AtZero },
    Word { word: "posesingles", knob: "posemath.singles", says: Says::AtZero },
    Word { word: "nowallsnap", knob: "wallsnap.on", says: Says::IfThere },
    Word { word: "noleangun", knob: "wallsnap.lean_gun", says: Says::IfThere },
    Word { word: "nosquadthrow", knob: "squadthrow.on", says: Says::IfThere },
    Word { word: "nobigslot", knob: "bigslot.on", says: Says::IfThere },
    Word { word: "shapes", knob: "shapes.on", says: Says::AtZero },
    Word { word: "doorcost", knob: "doorshot.toll", says: Says::ZeroIs(1.0) },
    Word { word: "firesway", knob: "fire.wander", says: Says::ZeroIs(0.06) },
    Word { word: "firesway", knob: "fire.wander_up", says: Says::ZeroIs(0.04) },
    Word { word: "firebright", knob: "fire.bright", says: Says::ZeroIs(1.0) },
    Word { word: "firebright", knob: "fire.reach", says: Says::ZeroIs(1.0) },
    Word { word: "firebright", knob: "fire.lift", says: Says::ZeroIs(0.25) },
    Word { word: "firesize", knob: "fire.size", says: Says::AtZero },
    Word { word: "fireheat", knob: "fire.look.shimmer", says: Says::AtZero },
    Word { word: "fireplume", knob: "fire.look.plume", says: Says::AtZero },
    Word { word: "fireembers", knob: "fire.embers", says: Says::AtZero },
    Word { word: "firereact", knob: "fire.look.react", says: Says::AtZero },
    Word { word: "firelean", knob: "fire.look.lean", says: Says::ZeroIs(0.07) },
    Word { word: "seenrate", knob: "sneak.seen_rate", says: Says::Share },
];

/// the words of an address laid on a game before it is made
/// what a start does with what its host hands it: the words first, so that the commands' kit stands under `?nobigslot` and its like, then the commands
pub fn at_start(g: &mut Game, words: &str, commands: &[u8]) -> Result<(), String> {
    apply(g, words);
    crate::command::apply(g, commands)
}
pub fn apply(g: &mut Game, words: &str) {
    let q = Query::parse(words);
    for w in WORDS {
        let v = match w.says {
            Says::AtZero => if matches!(q.get(w.word), Some("0" | "off")) { Some(0.0) } else { None },
            Says::IfThere => if q.has(w.word) { Some(0.0) } else { None },
            Says::ZeroIs(z) => if q.get(w.word) == Some("0") { Some(z) } else { None },
            Says::Share => q.get(w.word).and_then(|t| t.parse::<f64>().ok()).filter(|n| *n > 0.0).map(|n| n.min(1.0)),
        };
        if let Some(v) = v { crate::knobs::set(g, w.knob, v); }
    }
    for (k, v) in q.words() {
        if let (Some(name), Ok(v)) = (k.strip_prefix("knob."), v.parse::<f64>()) { crate::knobs::set_table(g, name, v); }
    }
    for (word, name) in [("training", "training"), ("nerve", "nerve")] {
        if let Some(pick) = q.get(word) { crate::actions::act(g, name, 0.0, pick); }
    }
}
