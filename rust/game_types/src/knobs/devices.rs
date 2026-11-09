//! The numbers of devices.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// overhearing a code said aloud: within this many metres of PATH through the air as col.sound propagates it (round the
///  corner, through the doorway) and at least this much of it carried (a shut door between them eats it) — the whistle's reach, the other way round
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct OverhearK {
    #[k(9.0)] pub path_m: f64,
    #[k(0.5)] pub carry: f64,
}
