//! The numbers of hands.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HandsK {
    /// ---- a body
    /// s for the hands to go down onto it · s for it to come up into them · s to lay it down
    #[k(0.3)] pub reach_secs: f64,
    #[k(0.35)] pub lift_secs: f64,
    #[k(0.3)] pub set_secs: f64,
    /// he steps in until the grip is this near (m), and gives up stepping after this long (s): the pins then gather it in as they always did
    #[k(0.55)] pub grip_m: f64,
    #[k(1.0)] pub step_secs: f64,
    /// the head end is taken when it is this much nearer than the feet (m)
    #[k(0.1)] pub head_bias: f64,
    /// m the grip stands ahead of him: the ankles · the shoulders (further: his head and arms hang on this side of them, and are not to hang in his knees) · carry
    ///  height of the ankles / the shoulders
    #[k(0.35)] pub ahead: f64,
    #[k(0.48)] pub ahead_head: f64,
    #[k(0.42)] pub carry_feet: f64,
    #[k(0.46)] pub carry_head: f64,
    /// the head end has no leg sticks to stretch: it is 'wedged' when a held shoulder trails its hand by this (m)
    #[k(0.4)] pub slack_m: f64,
    /// s it takes from the floor onto his shoulder · s back down onto the floor (a pin travels at most 2.4 m/s — the ragdoll's `PIN_STEP` — and both are eased: the
    ///  peak is 1.5 × the 1.5 m over the time, so neither may be much under a second or the five pins fall out of step and the body is let go still in the air)
    #[k(1.0)] pub hoist_secs: f64,
    #[k(1.1)] pub lower_secs: f64,
    /// s he may take stepping round to its hips before the lift (past it: lifted from where he is if that is near enough, else hauled along the floor) · how near is near enough beyond gripM (m)
    #[k(2.4)] pub carry_step_secs: f64,
    #[k(0.35)] pub carry_near_m: f64,
    /// where the middle of its hips rides, in his frame (m): toward his left · ahead · above his soles
    #[k(0.2)] pub carry_side: f64,
    #[k(-0.02)] pub carry_fwd: f64,
    #[k(1.56)] pub carry_up: f64,
    /// how its trunk hangs down his back (m back per m down) · how its thighs hang down his chest (m ahead per m down)
    #[k(1.35)] pub carry_back: f64,
    #[k(1.1)] pub carry_legs: f64,
    /// his pace under it, × his walk · how fast he comes round with it (rad/s: the pins are kinematic and travel at most 2.4 m/s)
    #[k(0.6)] pub carry_pace: f64,
    #[k(5.0)] pub carry_turn: f64,
    /// how much of his own step and turn its free limbs are taken through with him each frame: 0 = none, as it was — the body's damping is against
    ///  the floor, and every arm, shin and the head stream out behind each stride and turn, 40–75° off the hang · 1 = all of it: they hang off him as off a still man
    #[k(0.85)] pub carry_ride: f64,
    /// laid down: how far off his feet its hips go, on the side he lays it (m)
    #[k(0.62)] pub lay_ahead: f64,
    /// ---- a door
    /// s the hand has at least before the leaf starts · and at most, when it cannot reach it · rad/s under the hand · where an opened leaf is left (rad, short of its stop)
    #[k(0.12)] pub door_reach: f64,
    #[k(0.3)] pub hand_secs: f64,
    #[k(2.6)] pub door_rate: f64,
    #[k(1.5)] pub open_to: f64,
    /// clearance kept between his centre and the leaf's line beyond their own sizes (m) · how far he will step to clear a sweep (m)
    #[k(0.06)] pub clear_m: f64,
    #[k(1.3)] pub step_m: f64,
    /// a leaf brought round TOWARD him: how far he will walk to end on the OPENING's side of where it will stand, rather than in the pocket behind it
    #[k(1.8)] pub round_m: f64,
    /// opening a leaf away from him from further off than a hand reaches, he steps in to this far off its face (m) as his hand goes out — if that step is no longer than inM
    #[k(0.5)] pub hand_m: f64,
    #[k(0.75)] pub in_m: f64,
    /// s without the leaf making way before the move is dropped
    #[k(0.5)] pub stall_secs: f64,
    /// rad off his aim within which the leaf he works leaves his gun up and on the cursor
    #[k(1.2)] pub aim_arc: f64,
    #[k(0.55)] pub team_pre_m: f64,
    #[k(0.95)] pub team_reach_m: f64,
    #[k(0.9)] pub team_pace: f64,
    #[k(0.25)] pub team_vel: f64,
}
