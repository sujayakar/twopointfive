//! The numbers of darkdoor.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DarkdoorKWho {
    #[default] None,
    All,
    Rest,
}

/// A green man balks at a dark door. `who` = who is green: 'rest' (`training_of`: the knob, the squad's key, the difficulty's mix) · 'all' · 'none'. `stop_m` = how far along
/// his way a door starts the balk; `pair_m` = how near a mate must be for the two to go together. THE DARK: a room reads dark when `dark_share` of `samples` of its floor cells read under
/// `dark_at`, asked every `ask_secs`; a man `lit_by` over the room's mean, or at `dark_at`, stands in light. `open_rad` = how far a leaf must stand off its frame to be an open door,
/// `arm_secs` how long a shut door he reached in the light stays his to balk at once it opens. WAITING: `stand_m` off the frame and `side_m` to the side; `keep_aw` = the awareness a
/// waiting man's suspicion is held at (it must not drain away under him at the frame); `keep_secs` = how long a balk outlasts his walk not asking (a reaction beat, a clear shot),
/// `resume_secs` how long after that the same door takes the same wait up again without a word; he calls at once and again every `call_secs` while nobody has answered, gives up at `wait_secs`, and a door he gave up is
/// left `again_secs`. `sweep_secs` = how long his look takes across the frame; `seen_secs` = how lately a man seen keeps him off all this. THE MATE: within `mate_m`, the
/// nearest `mate_try` asked; past `ear_m` by radio; he copies after `copy_secs`, is let go past `lag_secs` and goes along at most `with_secs`. at his `fed_up_n`th balk at a room inside
/// `fed_up_secs` he goes in alone: he stands `nerve_secs`, then goes at `slow_k` of his pace. UNTUNED by eye (nobody has seen it on a GPU, nobody has played it).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DarkdoorK {
    #[k(DarkdoorKWho::Rest)] pub who: DarkdoorKWho,
    #[k(2.5)] pub stop_m: f64,
    #[k(4.0)] pub pair_m: f64,
    #[k(0.2)] pub dark_at: f64,
    #[k(0.7)] pub dark_share: f64,
    #[k(0.1)] pub lit_by: f64,
    #[k(40.0)] pub samples: f64,
    #[k(0.5)] pub ask_secs: f64,
    #[k(0.35)] pub open_rad: f64,
    #[k(3.0)] pub arm_secs: f64,
    #[k(1.3)] pub stand_m: f64,
    #[k(0.3)] pub side_m: f64,
    #[k(3.2)] pub sweep_secs: f64,
    #[k(12.0)] pub ear_m: f64,
    #[k(40.0)] pub mate_m: f64,
    #[k(3.0)] pub mate_try: f64,
    #[k(2.0)] pub call_secs: f64,
    #[k(1.2)] pub copy_secs: f64,
    #[k(5.0)] pub lag_secs: f64,
    #[k(10.0)] pub with_secs: f64,
    #[k(0.3)] pub keep_aw: f64,
    #[k(2.0)] pub keep_secs: f64,
    #[k(12.0)] pub resume_secs: f64,
    #[k(2.0)] pub seen_secs: f64,
    #[k(20.0)] pub wait_secs: f64,
    #[k(45.0)] pub again_secs: f64,
    #[k(300.0)] pub fed_up_secs: f64,
    #[k(2.0)] pub fed_up_n: f64,
    #[k(1.5)] pub nerve_secs: f64,
    #[k(0.5)] pub slow_k: f64,
}
