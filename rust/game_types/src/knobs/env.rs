//! The numbers of env.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// the level's own night's exposure (the linear gain the scene is taken at: the picture's settings.exposure before any turn of Brightness) and the
///  intensifier's log gain (the picture's settings.nvGain)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EnvK {
    #[k(2.0)] pub exposure: f64,
    #[k(475.0)] pub tube_gain: f64,
}
