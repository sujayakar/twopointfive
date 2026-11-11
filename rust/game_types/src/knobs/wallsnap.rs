//! The numbers of wallsnap.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WallsnapK {
    /// ---- the take (seconds = takeBase + takePerM × the glide + takePerTurn × the turn in half turns, within [takeMin, takeMax])
    #[k(0.1)] pub take_base: f64,
    #[k(0.16)] pub take_per_m: f64,
    #[k(0.06)] pub take_per_turn: f64,
    #[k(0.12)] pub take_min: f64,
    #[k(0.3)] pub take_max: f64,
    /// the glide's ease-out power (1 − (1 − s)^p: 2 = a quarter of the way in the first eighth of the time) and the turn's (ahead of the glide)
    #[k(2.0)] pub ease_pow: f64,
    #[k(3.0)] pub yaw_pow: f64,
    /// how far off the body a face may be when he is walking AT it (m; standing still it is the hold's own 0.7), how squarely (cos), and how much that outranks a nearer face he is passing (m of score)
    #[k(1.15)] pub reach_moving: f64,
    #[k(0.5)] pub heading_min: f64,
    #[k(0.35)] pub heading_bonus: f64,
    /// Q pressed this long (s) before a face comes into reach still takes it
    #[k(0.15)] pub buffer: f64,
    /// ---- the landing
    /// pelvis dip (m) and head tip back (degrees) at the peak of the settle, the peak's time after the back lands (s), the scuff's gain (0 = silent)
    #[k(0.035)] pub dip: f64,
    #[k(4.0)] pub nod: f64,
    #[k(0.05)] pub hit_peak: f64,
    /// the feet: lift of each step (m), and how far behind a planted foot may be left before it drags (m)
    #[k(0.06)] pub step_lift: f64,
    #[k(0.5)] pub step_trail: f64,
    /// the camera's look-at while he is on the wall: into the room, and toward the side he looks (m)
    #[k(0.35)] pub cam_room: f64,
    #[k(0.3)] pub cam_side: f64,
    /// ---- the way along, and the way off
    /// the stick within this of the wall's own line (degrees) SIDLES him, whichever way the camera is turned, and a stick that sidles him is never 'held away':
    ///  only one nearer square-out than this lets go. 0 = the rule before it (along when that share was at least the square share, away from half a stick
    ///  outward — under a camera turned 35° to the wall the key that mostly runs along it threw him off in a fifth of a second)
    #[k(55.0)] pub along_deg: f64,
    /// stick held square away (within 35°) / on a diagonal (s; the diagonal's only with alongDeg 0: with it, a diagonal sidles)
    #[k(0.1)] pub away_secs: f64,
    #[k(0.18)] pub away_diag: f64,
    /// Q with nothing held: the push off the plaster (m/s, eaten by the ordinary stop within ~5 cm)
    #[k(0.7)] pub off_push: f64,
    /// the rifle stays up this long after a click (s) and takes this long to come up before a round may leave (s); no round while this much of the bore's line is in a wall (m)
    #[k(1.2)] pub up_secs: f64,
    #[k(0.16)] pub up_rise: f64,
    #[k(0.9)] pub bore: f64,
    /// stepping out round an open end: how far past his parked station along the wall (m), seconds out and back
    #[k(0.6)] pub lean_along: f64,
    #[k(0.18)] pub lean_out: f64,
    #[k(0.14)] pub lean_back: f64,
    /// the cursor's angle past what he can already see from cover that steps him out / back (degrees), its least distance from the corner (m), and behind the wall past this (degrees about the corner) stepping out would not bring it into his line either
    #[k(8.0)] pub lean_on_deg: f64,
    #[k(2.0)] pub lean_off_deg: f64,
    #[k(0.6)] pub lean_min_dist: f64,
    #[k(100.0)] pub lean_max_deg: f64,
    /// once out, the far limits give this much before he comes back (degrees further round, m nearer the corner): a point sitting on a limit does not flap him
    #[k(15.0)] pub lean_keep_deg: f64,
    #[k(0.2)] pub lean_keep_dist: f64,
    /// a cursor that lands above this height (m: a wall's top, a lintel) is read where its ray comes down to chest height behind it; off = the landing point as it is
    #[k(1.9)] pub lean_over_y: f64,
    /// he steps out only if, from where he would then stand, this much of the way to the point is open (m; 0 = the angle alone — he would step out to face a shut door)
    #[k(1.4)] pub lean_clear_m: f64,
    /// THE GUN'S SHARE OF THE STEP: leanAlong is measured for his eyes, and the rifle rides a hand's width to his right — round an end on that side it is
    ///  still in the jamb when his head is clear, and its rounds stop in the wood. Up to this much further (m) he steps, by leanGunStep, until the rifle as it
    ///  would lie from there and the first leanClearM of its round's way are clear; never less again until he is back in.
    #[k(0.42)] pub lean_gun: f64,
    #[k(0.06)] pub lean_gun_step: f64,
    /// the cursor must have stayed there this long before he steps out / back (s); once out he stays at least minOut, and afterShot after any round (s)
    #[k(0.06)] pub dwell_out: f64,
    #[k(0.15)] pub dwell_back: f64,
    #[k(0.35)] pub min_out: f64,
    #[k(0.4)] pub after_shot: f64,
    /// ---- the wall taken WITHOUT the key (its own switch; acts only with `on`; false = Q alone takes a wall, as it was)
    /// a move key held INTO a wall takes it as Q would: the stick within pressDeg of square to the face, for pressSecs, counted from pressNear (m) out — so a
    ///  man who walks squarely up to a wall has it from pressFlow (m) short of touching, his back coming round as he arrives, and one who turns into a wall he
    ///  stands against waits the whole of it there. Where the face has something to use within pressThingAt (m) of the spot he walks into (a door, a switch, the
    ///  breaker, the mission's thing, a body) the wait is pressThing, all of it within pressTouch (m) of the face: he walked up to use it.
    #[k(true)] pub press: bool,
    #[k(0.18)] pub press_secs: f64,
    #[k(0.2)] pub press_flow: f64,
    #[k(0.6)] pub press_near: f64,
    #[k(35.0)] pub press_deg: f64,
    #[k(0.08)] pub press_touch: f64,
    #[k(0.5)] pub press_thing: f64,
    #[k(0.9)] pub press_thing_at: f64,
    /// the latch a hold's end (or a refusal) leaves on the stick as it then stands: gone when the stick rests or turns by this (deg); till then a wall facing as
    ///  the one he left is not taken, nor any other within this (s)
    #[k(50.0)] pub press_turn: f64,
    #[k(0.6)] pub press_again: f64,
    /// …and not with a mouse button held, nor within this (s) of a round of his own: a man shooting who backs into a wall keeps his gun up (the stack carries the rifle at the low ready)
    #[k(1.0)] pub press_after_shot: f64,
    #[k(0.35)] pub voice_on: f64,
    #[k(0.5)] pub voice_off: f64,
    #[k(2.0)] pub slow_take: f64,
    #[k(11.0)] pub slow_in: f64,
    #[k(12.0)] pub slow_out: f64,
}
