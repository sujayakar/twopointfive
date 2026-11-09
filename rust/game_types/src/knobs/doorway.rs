//! The numbers of doorway.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DoormannersK {
    /// metres of his walk ahead looked along for a frame; the strip either face of the frame line that is 'the opening'; how far short of the line a held man stops
    #[k(3.0)] pub reach: f64,
    #[k(0.45)] pub half: f64,
    #[k(1.25)] pub throat: f64,
    /// following a friend through: full pace at `gap` metres from him, none at `gapMin`; how hard he brakes for it (m/s²)
    #[k(1.05)] pub gap: f64,
    #[k(0.7)] pub gap_min: f64,
    #[k(8.0)] pub brake: f64,
    /// seconds held before he goes anyway (under every caller's own stall clock: the strafe's 1.2 s, the room fight's 1.5 s 'going nowhere', the orders' 4 s
    ///  'stuck'), and before he can be held again; seconds held by a man standing just beyond the frame (he may be there to stay)
    #[k(1.1)] pub patience: f64,
    #[k(2.0)] pub rest: f64,
    #[k(0.6)] pub beyond: f64,
    /// seconds held before he steps off to a jamb; the place: metres off the wall, past the jamb, between places, from anybody else; the pace he steps at
    #[k(0.4)] pub aside: f64,
    #[k(0.6)] pub jamb_out: f64,
    #[k(0.5)] pub jamb_past: f64,
    #[k(0.9)] pub file_gap: f64,
    #[k(0.85)] pub apart: f64,
    #[k(1.4)] pub step: f64,
    /// a friend standing on his line this near, his way on past him: he walks round, this far off the man's middle
    #[k(1.2)] pub round: f64,
    #[k(0.85)] pub round_out: f64,
    /// PASS, a friend ON HIS OWN MARK (his order's mark within `ownM` of his feet): he is a standing man whatever his feet are doing — settling onto it, or
    ///  stepping back onto it after a nudge — so a man whose way goes on past him walks round him and is never walked into him; and the rule is also asked on
    ///  the last `ownEndM` metres of a walk that ENDS by a door (a stack's second rank stands further than 2.5 m from the frame's middle: the man bound for the
    ///  wall place beyond it shoved the rear man off his mark, onto his own, and the two then stood on each other's). Off = the rule as it was
    #[k(0.45)] pub own_m: f64,
    #[k(2.5)] pub own_end_m: f64,
    #[k(0.12)] pub leaf_stall_m: f64,
    #[k(0.6)] pub leaf_stall_secs: f64,
    #[k(0.6)] pub leaf_stall_reach: f64,
    #[k(0.3)] pub leaf_stall_ahead: f64,
    #[k(1.5)] pub leaf_stall_push: f64,
    /// the swing: this far past the leaf's tip is still 'where it sweeps'
    #[k(0.2)] pub sweep: f64,
    #[k(0.6)] pub lane_ajar: f64,
    #[k(6.0)] pub lane_m: f64,
    #[k(0.3)] pub lane_out: f64,
    #[k(0.12)] pub lane_flare: f64,
    #[k(0.15)] pub lane_clear: f64,
    #[k(0.7)] pub lane_apart: f64,
    #[k(2.0)] pub cover_m: f64,
    #[k(2.5)] pub cover_deep: f64,
    #[k(0.8)] pub cover_look: f64,
    #[k(1.0)] pub cover_quiet: f64,
    #[k(8.0)] pub cover_cap: f64,
    #[k(6.0)] pub lane_dear: f64,
    #[k(0.7)] pub lane_strip: f64,
    #[k(1.6)] pub lane_long: f64,
    /// FIGHT: seconds planted in an open doorway before he steps off it, the rest after a step, metres through and past the jamb he steps to
    #[k(0.6)] pub funnel: f64,
    #[k(1.5)] pub funnel_rest: f64,
    #[k(0.95)] pub through: f64,
    #[k(0.6)] pub past: f64,
    /// FIGHT, a defender's step and cover (off = the step as it was): no spot he steps to is more exposed to a rifle at his man (or his fix) than the frame he
    ///  stands in — covered, reached only through glass (a pane is no wall: glasspen.ts, at `GLASSPEN.stackEnergy`), reached in clear air — so from a covered
    ///  frame he steps only to covered floor or stays; with nobody in his sights a covered spot comes before any other free one; and with them within
    ///  `planeDeg` of the door's own wall line (never less than half a metre of it: neither face is 'away from them') covered floor on EITHER face will do —
    ///  the face the half-plane calls theirs only ever under cover
    #[k(30.0)] pub plane_deg: f64,
    /// REST (orders.ts restClear): a fall-in follower is not left standing in a frame's strip or where a leaf swings — with Sam stopped, a place the file's
    ///  half-metre slack has left in one is given up for his place on the trail (or the nearest clear floor), and a man who has stood `restSecs` in one short
    ///  of his place steps to the nearest clear floor (within `clearM`) and waits there. MARKS (orders.ts stackPosts): the stack marks
    ///  of the men who are not at a jamb are moved off strips and swings the same way; PODS (doortools.ts offSwing, `podClear`, ships OFF): so are the points men are
    ///  left on after an entry — measured on the barracks it loses the entry (11 runs won of 12 → 4: a corner point behind an open leaf is cover). SHORT (orders.ts
    ///  restShort): a man who cannot get to his mark does not give it up standing in a strip or a swing, and the
    ///  seconds the PASS gate holds him (`heldCap` of them an order) are not counted toward 'stuck'. Each off = as it was
    #[k(1.0)] pub rest_secs: f64,
    #[k(1.4)] pub clear_m: f64,
    #[k(12.0)] pub held_cap: f64,
}
