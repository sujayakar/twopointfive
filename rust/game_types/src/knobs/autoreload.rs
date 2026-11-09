//! The numbers of autoreload.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// the switch and its one number: on (?noautoreload = a dry gun waits for R, as it did) and by the
///  shipped stand; the harness's frozen stand leaves it off (TPS_AUTORELOAD=1 turns it on). `delay` = seconds after the last round before the hands move (UNTUNED by eye: the round's recoil is seen first).
/// `lastMag` = THE LAST MAGAZINE, SEATED AND NOT RACKED, IS STILL A RELOAD: a reload broken off between 'magazine in' and the rack with no spare left stood as a
///  full magazine under an empty chamber that R refused ('no spare magazines') and nothing restarted — a gun that never fired again. On, R and the rule above
///  take it: the magazine comes out, goes back in and the handle is racked (weapons.ts startReload). Off in the module and on the frozen stand
///  (TPS_LASTMAG=1), on in the page (?nolastmag = the build as it was).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ReloadK {
    #[k(0.25)] pub delay: f64,
}
