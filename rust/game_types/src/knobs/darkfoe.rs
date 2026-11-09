//! The numbers of darkfoe.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// `litAt` is on the meter's 0‥1 scale. UNTUNED by eye: nobody has seen it on a GPU, and the estimate has no bounce (it errs dark, so toward the lamp on).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DarkfoeK {
    /// the share of them that carry a light (by id); 1 = every man, 0 = nobody
    #[k(1.0)] pub carry: f64,
    #[k(0.3)] pub lit_at: f64,
    #[k(12.0)] pub look_m: f64,
    #[k(2.0)] pub step_m: f64,
    #[k(0.3)] pub fan_rad: f64,
    #[k(0.34)] pub see_share: f64,
    #[k(0.25)] pub ask_secs: f64,
    #[k(0.6)] pub on_secs: f64,
    #[k(2.5)] pub on_jitter: f64,
    #[k(15.0)] pub off_secs: f64,
    #[k(0.4)] pub off_jitter: f64,
    #[k(2.0)] pub hold_secs: f64,
    /// a mate's lamp coming on in his plain view within `lampM` is the word
    #[k(16.0)] pub lamp_m: f64,
    #[k(3.0)] pub blind_secs: f64,
    #[k(10.0)] pub seek_m: f64,
    #[k(12.0)] pub walk_m: f64,
    #[k(3.0)] pub edge_m: f64,
    #[k(0.75)] pub stumble_k: f64,
    #[k(20.0)] pub again_secs: f64,
    #[k(2.0)] pub moves: f64,
    #[k(4.0)] pub quiet_secs: f64,
    #[k(6.0)] pub calm_secs: f64,
}
