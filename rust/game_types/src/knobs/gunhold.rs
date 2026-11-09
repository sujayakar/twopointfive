//! The numbers of gunhold.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct GunholdK {
    /// how far a man will run for cover, metres of floor
    #[k(10.0)] pub cover_m: f64,
    /// nobody's cover, and nobody's way round, inside this of the pair
    #[k(3.0)] pub near_m: f64,
    /// a man this close to dead astern of Sam (degrees) is where he wants to be: he does not run from there
    #[k(70.0)] pub rear_deg: f64,
    /// the standoff is kept this long with nobody of the side having seen the pair
    #[k(14.0)] pub keep_secs: f64,
    /// nobody has seen them for this long: a man in cover takes a look …
    #[k(4.0)] pub peek_every: f64,
    /// … for this long, then back
    #[k(0.8)] pub peek_secs: f64,
    /// the longest stretch of a way round across floor his gun covers in front of him: with the pistol turned this man's way …
    #[k(2.5)] pub show_m: f64,
    /// … seconds after a round from the hold that a man the pistol is turned on will cross next to none of it
    #[k(6.0)] pub shy_s: f64,
    /// … and with it on somebody else (a dash between covers while he looks the other way)
    #[k(3.5)] pub dash_m: f64,
    /// 'turned his way' = the man within this of where the pair faces, degrees
    #[k(50.0)] pub face_deg: f64,
    /// the longest way round, metres
    #[k(40.0)] pub flank_max_m: f64,
    /// a man's own beat before his legs answer the gun, seconds [least, most]
    #[k([0.1, 0.4])] pub react: [f64; 2],
}
