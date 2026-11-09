//! The numbers of qrf.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// ================================================================ the team is the level's own (RESPONSE)
/// THE RESPONSE TEAM AS LEVEL DESIGN (the maintainer, 10-18: "let's make it part of the level design and always make sense (ideally w.r.t. the concept/design
///  of the level) and not all levels should have it"; "having some randomization and modeling their knowledge based on what they've heard on the radio could make
///  it really fun"). ON in the page (`?response=0` = the build as it was), OFF here and on the frozen harness stand (TPS_RESPONSE=1; a FROZEN_HOLDS row).
///  With `on`: a level has a team only where its file says so (tools/levels/lib/scenarios.ts ResponseData — the ways in, the marks, the words, how many and how long);
///  the office's own block (levels/office.scenarios.ts) is the marks above, number for number. `deal` and `net` act only with `on`; each off = that rule as it
///  was. Untuned by play.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ResponseK {
    /// no way is dealt whose muster an enemy stands inside this many metres of when the call goes out (nobody is stood up at his elbow); a dealt time is to
    ///  the nearest `etaStep` s; the net's word is read this often (s)
    #[k(12.0)] pub clear_m: f64,
    #[k(5.0)] pub eta_step: f64,
    #[k(0.5)] pub word_secs: f64,
    #[k(6.0)] pub call_secs: f64,
}
