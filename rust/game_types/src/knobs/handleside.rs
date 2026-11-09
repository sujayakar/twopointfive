//! The numbers of handleside.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HandleK {
    /// metres of his walk ahead looked along for a leaf in his way; a leaf standing less open than `ajar` rad is in his way (doors.ts: under it a man on the
    ///  pull face 'presses' it)
    #[k(2.6)] pub look: f64,
    #[k(1.35)] pub ajar: f64,
    /// within this of the handle (floor metres, his middle to it) is 'at the handle' — the handle sits `handleIn` short of the latch jamb; a door in a corner
    ///  is pulled from up to `farReach`
    #[k(0.95)] pub reach: f64,
    #[k(0.14)] pub handle_in: f64,
    #[k(1.5)] pub far_reach: f64,
    /// PUSH: the line a walker goes through the opening on — metres in from the latch jamb; he squares up to it `pushOff` off the leaf and goes on to
    ///  `pushThru` beyond the frame — and inside `thruAt` m of the line his feet are put ON it, `thruOff` off the leaf, before he goes through (a walk's own
    ///  corner-cutting crossed the hinge half). `pushIn` 0.33 since the doors are real size (scene/doorSize.ts; 0.42 at the 1.16 m leaves): hard by the latch
    ///  stop, because a man who comes to that line from the latch jamb — where he pulled the leaf from — overshoots it a hand toward the leaf, and a leaf
    ///  standing square on a side wall's stop is the one thing in the opening he must not touch. A man who STANDS to shove it (a door order's lead) stands `standIn` in from the jamb, `standOff` off the leaf
    #[k(0.33)] pub push_in: f64,
    #[k(0.85)] pub push_off: f64,
    #[k(0.35)] pub push_thru: f64,
    #[k(0.3)] pub stand_in: f64,
    #[k(0.6)] pub stand_off: f64,
    #[k(1.2)] pub thru_at: f64,
    #[k(0.75)] pub thru_off: f64,
    /// PULL: his body's radius, and the slack kept between it and the leaf's sweep; the leaf is his once he is within `near` of his place
    #[k(0.34)] pub body: f64,
    #[k(0.1)] pub slack: f64,
    #[k(0.05)] pub near: f64,
    /// … he turns to the handle (this long at most), pulls, and walks on when the leaf stands `clear` rad open, has stopped, or after `wait` s; a leaf standing
    ///  `clear` open on that face wants no hand. `press`: seconds a man leaning on a leaf from that face, with no walk to its handle, takes to pull it as before
    #[k(0.35)] pub turn: f64,
    #[k(1.05)] pub clear: f64,
    #[k(1.1)] pub wait: f64,
    #[k(1.5)] pub press: f64,
    /// TOOLS, the face it opens AWAY from: a door order's lead standing past the latch jamb within this of the handle shoves it from where he stands, at arm's
    ///  length, and does not step in front of the opening for it (his jamb mark is 1.1 m from the handle: the walk put him in the doorway for the shove and
    ///  through the toss of a bang). 0 = he steps to the handle's place first, as this file first had it
    #[k(0.0)] pub tool_post: f64,
    #[k(0.38)] pub near_body: f64,
    #[k(1.3)] pub near_post: f64,
    /// seconds a walk to the place may take before he gives it up; seconds before he tries that door by the handle again
    #[k(4.0)] pub go_cap: f64,
    #[k(2.5)] pub rest: f64,
}
