//! The numbers of cmdring.
use crate::bytes::ToBytes;
use alloc::string::String;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct CmdringK {
    /// the key: the squad's command key — the quick order's own code, which the page's board has on Q (host/keymap.ts; the
    ///  maintainer, 10-21: "let's move the squad command button to Q then too", "from tab"). It was Tab until then: a recording carries its own table, so
    ///  one made on Tab replays on Tab. Tab itself is the page's again — the way back and the objectives card (ui/tabcard.ts)
    #[k(String::from("KeyB"))] pub key: String,
    /// wedges on the ring, clockwise from the top: the digits 1 … this (never above ORDER_DIGITS)
    #[k(5.0)] pub wedges: f64,
    /// the ring's middle: the cursor this far (CSS px) out of it lights the wedge it is out toward — once it has been back inside since the ring last changed
    ///  under it (a page turned, a digit, a man clicked, or a ring opened at the view's edge with the cursor out of its middle)
    #[k(24.0)] pub flick_px: f64,
    /// A TAP IS THE QUICK KEY'S ORDER AND NO RING (the maintainer, 10-20, after playing it: "we shouldn't open the radial menu on just a single tab press,
    ///  only after some (very short) delay"): the key let go inside this long (s) gives what the quick key gives on what the cursor was on when it went
    ///  down — no menu was ever up, so nothing is drawn and no other key or button of those moments is the menu's — and the key still down after it puts
    ///  the ring up for that thing, at the cursor. 0 = the ring the moment the key goes down, as it was first built (`tapSecs` then guards the tap)
    #[k(0.2)] pub hold_secs: f64,
    /// … and how long that press is (s), in place of `holdSecs`: a ring that stays costs a press to put away, so it asks for a plainer hold than one that goes with the key
    #[k(0.3)] pub stay_hold_secs: f64,
    /// (with `holdSecs` 0) a TAP IS THE FIRST WEDGE HOWEVER THE HAND IS MOVING: the cursor lights nothing until the key has been down this long (s), and one already out of the
    ///  middle by then lights nothing until it has been back. A digit counts from the first frame. 0 = the cursor counts at once
    #[k(0.15)] pub tap_secs: f64,
    /// a ring opened nearer the view's sides (edgeX) or its top and bottom (edgeY) than this stands this far in, whole (CSS px)
    #[k(150.0)] pub edge_x: f64,
    #[k(100.0)] pub edge_y: f64,
    /// a left click within this many CSS px of a man's feet on the screen is a click on him
    #[k(24.0)] pub man_px: f64,
    /// the window the ring's px are for: a larger one makes every px above — and the ring as drawn — larger by the same ratio, never smaller (the page's
    ///  words grow the same way, by the page's own rule; this is the window alone, so what the cursor does on the ring is the same wherever a recording is played)
    #[k([1920.0, 1080.0])] pub ref_: [f64; 2],
    /// THE FIRST SHIFT + KEY SENDS HALF THE SQUAD (the maintainer, 11-02, passing on a play tester's note: "first shift Q should just send half the
    ///  squad, whoever's closest"): with nothing going a goal was everyone's, the key's own order, and only the second one split the squad. Now a
    ///  squad with nothing going gives up half of itself as any group does: a whole pair where it holds two, the pair nearer
    ///  what the cursor is on, else the nearer half of its men (two teammates: the
    ///  nearer man; three: the nearer of the whole pair and the odd man). The rest stay with him, and the next goal is theirs. One teammate alone is
    ///  still everyone. false: with nothing going the goal is everyone's, as
    ///  `goals` was built. Needs `goals`. Off here and on the frozen stand (`TPS_GOALHALF=1`, beside `TPS_GOALS=1`); the page raises it with `goals`
    ///  (`?goalhalf=0` off)
    #[k(true)] pub goal_half: bool,
    /// THE KEY TAPPED TWICE IS 'FALL IN' FOR EVERYONE (the same note: "double tap Q -> cancel everything and fall in"). A tap still gives its order as
    ///  the key comes up, with no wait for a second one ("tapping Q just always sends the order"); a second plain tap inside `twiceSecs` of it, on the
    ///  same place (a cursor that has not moved more than `twicePx` of the view, or the same door), gives 'fall in' to every teammate in its place:
    ///  an order without Shift, so it is everyone's and every goal goes with it. A hand that moved to another thing gives that thing's order. A tap
    ///  with Shift is never half of the pair, a third tap starts again, and a pause between the two ends the pair. On a tutorial mission only from
    ///  a step that lists `twice`. false: the second tap is the key's order again, as it was. Off here and on the frozen
    ///  stand (`TPS_QTWICE=1`, beside `TPS_QRING=1`); the page raises it with the ring (`?qtwice=0` off)
    #[k(true)] pub twice: bool,
    #[k(0.4)] pub twice_secs: f64,
    #[k(24.0)] pub twice_px: f64,
}
