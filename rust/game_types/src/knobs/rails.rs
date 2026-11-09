//! The numbers of rails.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RailsK {
    /// the cursor this near (px) the step's mark on the screen is ON the mark's place, whatever stands between the eye and it
    #[k(40.0)] pub snap_px: f64,
    /// a step's `onWay`: a man of the squad under an order whose mark is within this many metres of the step's own is on his way to it (0: never)
    #[k(6.0)] pub way_m: f64,
    /// a step's `after`: seconds its line stands in for the step's own once the order it asks for has gone out and its men stand where it sent them (while one
    ///  is still walking the line stays) — then the step asks again: the men could not do it, or it was the wrong place, and a line that only ever says 'they
    ///  are on it' is a player left waiting. 0: never shown
    #[k(8.0)] pub after_secs: f64,
    /// …and however long they walk, the step asks again this many seconds after the order went out (men who cannot get there never arrive)
    #[k(45.0)] pub after_max: f64,
    /// the big line goes to the bottom edge from over the player and his teammates where they stand under it (ui/railclear.ts; picture only)
    #[k(true)] pub line_clear: bool,
    /// a step's mark is not drawn where a ring lies over it, the squad's or his own (ui/overlay.ts asks ui/cmdring.ts `covers`; picture only): the mark
    ///  is on the spot a ring opens on, so its circle lay over the ring's middle and the digits of its inner wedges — and with the ring up there the
    ///  place is found, and the line says what to press (`?markclear=0`; a knob of the frame's table like lineClear, and the frame is off on the
    ///  frozen stand)
    #[k(true)] pub mark_clear: bool,
    #[k(1.2)] pub card_beat_secs: f64,
    #[k(2.0)] pub card_quiet: f64,
    #[k(9.0)] pub card_max: f64,
    /// seconds a spared teammate is given when the level names none
    #[k(6.0)] pub spare_secs: f64,
}
