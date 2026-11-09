//! The numbers of rails.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RailsK {
    /// the sky seat: metres a second the walk keys shift the view at the default zoom, and how far from the squad it may go
    #[k(11.0)] pub pan_speed: f64,
    #[k(16.0)] pub pan_reach: f64,
    /// the cursor this near (px) the step's mark on the screen is ON the mark's place, whatever stands between the eye and it (orders.ts targetUnder)
    #[k(40.0)] pub snap_px: f64,
    /// the sky seat keeps the step's mark in the picture: with no walk key held, a view shifted so that the mark is further from its middle than `keep` of the
    ///  camera's distance is eased back (m/s at the default zoom) until it is not — a ring off the screen, or under the squad's panel, is a player lost.
    ///  The kept ground is an oval, not a round: `keep` of the distance to either side (short of the squad's panel on the left), `keepDeep` of that reach up and
    ///  down the screen, with its middle `keepUp` of the reach up the screen from the view's own (the step's line sits low in the middle: a ring behind it,
    ///  or behind the panel, is a ring nobody sees)
    #[k(0.21)] pub keep: f64,
    #[k(9.0)] pub keep_speed: f64,
    #[k(0.79)] pub keep_deep: f64,
    #[k(0.39)] pub keep_up: f64,
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
