//! The numbers of autoreload.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// `delay` = seconds after the last round before the hands move (UNTUNED by eye: the round's recoil is seen first). `lastMag` = THE LAST MAGAZINE, SEATED AND NOT RACKED, IS STILL A RELOAD: a reload broken
/// off between 'magazine in' and the rack with no spare left stood as a full magazine under an empty chamber that R refused ('no spare magazines') and nothing restarted — a gun that never fired again. On,
/// R and the rule above take it: the magazine comes out, goes back in and the handle is racked.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ReloadK {
    #[k(0.25)] pub delay: f64,
}
