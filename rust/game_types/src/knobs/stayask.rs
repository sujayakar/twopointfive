//! The numbers of stayask.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// the switch and its number: on (?nostayask = squads that stay never miss another's room,
///  as it was) and by the shipped stand; the harness's frozen stand leaves it off (TPS_STAYASK=1 turns it on). UNTUNED by play.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct StayaskK {
    /// men a squad that stays sends to a silent squad's posts (it keeps a man at home, so a squad of fewer than this and one sends nobody)
    #[k(1.0)] pub men: f64,
}
