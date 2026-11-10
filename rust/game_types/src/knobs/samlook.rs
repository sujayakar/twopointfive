//! The numbers of samlook.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SamlookK {
    #[k(8.0)] pub near_m: f64,
    #[k(5.5)] pub full_m: f64,
    #[k(120.0)] pub back_deg: f64,
    #[k(140.0)] pub keep_deg: f64,
    #[k(3.0)] pub notice_m: f64,
    #[k(0.75)] pub swap_m: f64,
    #[k(2.6)] pub slip_from: f64,
    #[k(1.2)] pub slip_by: f64,
    #[k([-0.45, -0.1])] pub back_cos: [f64; 2],
}
