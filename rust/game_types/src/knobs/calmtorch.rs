//! The numbers of calmtorch.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// the switch and its numbers: on (?calmtorch=0 = every patrol torch burns, as it did) and by the
///  shipped stand; the harness's frozen stand leaves it off (TPS_CALMTORCH=1 turns it on). `litAt` is on the meter's 0‥1 scale (by the direct estimate the cubicles
///  under their panels read 0.3‥0.5, the dark lobby and the break room under 0.1; at 0.3 the dusk coming in by the frontage did not count and Novak burned his
///  torch a fifth of a dusk lap), `offK` × it is what the floor must read for the torch to go OFF; `spillM` = how far lit floor in plain view lights where he
///  stands, `dayM` the same for floor the hour's sun or sky lights; `aheadM` = how far down his way he looks; `onSecs` / `offSecs` = the dwell, `holdSecs` = the
///  least time between two clicks; `askSecs` = how often the place is read; `calmAw` = the awareness at which something has his attention and the torch is his to
///  look with. UNTUNED by eye (nobody has seen it on a GPU).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct TorchK {
    #[k(0.2)] pub lit_at: f64,
    #[k(1.2)] pub off_k: f64,
    #[k(3.0)] pub spill_m: f64,
    #[k(12.0)] pub day_m: f64,
    #[k(5.0)] pub ahead_m: f64,
    #[k(0.3)] pub on_secs: f64,
    #[k(2.0)] pub off_secs: f64,
    #[k(1.0)] pub hold_secs: f64,
    #[k(0.25)] pub ask_secs: f64,
    #[k(0.3)] pub calm_aw: f64,
    #[k(1.2)] pub wall_m: f64,
    #[k(1.6)] pub wall_back_m: f64,
    #[k(2.2)] pub low_tan: f64,
    #[k(6.0)] pub low_rate: f64,
}
