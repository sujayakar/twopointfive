//! The numbers of darkdoor.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DarkdoorKWho {
    #[default] None,
    All,
    Rest,
}

/// the switch and its numbers: on (?nodarkdoor = every man walks into the dark as he did) and by
///  the shipped stand; the harness's frozen stand leaves it off (TPS_DARKDOOR=1 turns it on — with TPS_MORALE=1: it acts only under MORALE.on). `who` = who is
///  green: 'rest' (morale.ts trainingOf: the knob, the squad's key, the difficulty's mix) · 'all' · 'none'. Metres, seconds and shares as the
///  head of the file names them; `openRad` = how far a leaf must stand off its frame to be an open door, `armSecs` how long a shut door he
///  reached in the light stays his to balk at once it opens; `keepAw` = the awareness a waiting man's suspicion is
///  held at (it must not drain away under him at the frame); `keepSecs` = how long a balk outlasts his walk not asking (a reaction beat, a clear shot), and
///  `resumeSecs` how long after that the same door takes the same wait up again without a word. UNTUNED by eye (nobody has seen it on a GPU, nobody has played it).
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
