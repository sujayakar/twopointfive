//! The numbers of frag.
use crate::bytes::ToBytes;
use alloc::string::String;
use game_derive::Knobs;

/// Every number of the grenade. fuse: seconds from leaving the hand (the player's); orderFuse: a teammate's on an order — cooked a beat, it has less far to go.
///  lethalM / woundM: the two radii of the model above; woundKill: the chance of a kill at the very edge of lethalM for an exposed man, falling to 0 at woundM;
///  stagger: seconds folded, [nearest, farthest]. lift: how far over the resting grenade the burst is judged from (a casing on the floor throws its fragments up
///  and out — a 5 cm door sill is no cover). Reaction: seeM / hearM = how far off a man can notice it (a line of sight to it / its clatter by the sound's own
///  path); react = seconds before he acts on it [trained, not], + up to reactJit; shoutM = how far his 'grenade!' carries to his own side; fleeM = men in its line
///  inside this move; runSpeed = a defender's legs. on = the item exists at all (false: digit 5 says so).
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct FragK {
    #[k(2.0)] pub issue: f64,
    #[k(3.0)] pub fuse: f64,
    #[k(2.4)] pub order_fuse: f64,
    /// the squad's: how many it carries per run, the key of the door tools' 'frag and clear' row (no digit: the
    ///  page's five are taken), how far inside the frame line a teammate throws it
    #[k(2.0)] pub squad: f64,
    #[k(String::from("KeyG"))] pub order_key: String,
    #[k(3.4)] pub order_in: f64,
    #[k(4.0)] pub lethal_m: f64,
    #[k(8.0)] pub wound_m: f64,
    #[k(2.5)] pub destr_m: f64,
    #[k(0.6)] pub wound_kill: f64,
    #[k([3.2, 1.2])] pub stagger: [f64; 2],
    #[k(0.2)] pub lift: f64,
    /// THE BURST AND GLASS (only where the fracture model is live). glassM / glassPower: the size of the blast the panes round it take (GlassBreak.blast's radius and
    ///  power — strength at a pane d metres off = glassPower × (1 − d ∕ glassM), against the fracture model's own thresholds GLASSBREAK.blastRunK / blastDiceK /
    ///  blastSheetK: at the numbers shipped on 10-08, float glass runs out to 3.5 m, toughened glass dices to 2.8 m, a laminated sheet leaves its frame inside 2.0 m;
    ///  a stun charge, 2.6 m at 0.6, does 1.7 m / 0.65 m / never). glassPower 0 = the burst sends glass nothing of its own (the 'bang' every charge makes still stars
    ///  it like a stun's). paneEnergy: what a fragment has for a pane on its line, in a round's units — 0 = standing glass stops
    ///  it, which is the rule; holes pass whatever it is.
    #[k(4.0)] pub glass_m: f64,
    #[k(1.5)] pub glass_power: f64,
    #[k(0.0)] pub pane_energy: f64,
    #[k(9.0)] pub see_m: f64,
    #[k(5.0)] pub hear_m: f64,
    #[k([0.25, 0.5])] pub react: [f64; 2],
    #[k(0.25)] pub react_jit: f64,
    #[k(9.0)] pub shout_m: f64,
    #[k(9.0)] pub flee_m: f64,
    #[k(3.4)] pub run_speed: f64,
    #[k(1.0)] pub flee_keep: f64,
    #[k(1.0)] pub flee_apart_m: f64,
    #[k(0.9)] pub flee_lane_m: f64,
    #[k(2.4)] pub down_shove: f64,
    #[k(1.0)] pub flee_door_open: f64,
    #[k(1.0)] pub flee_door_in: f64,
    #[k(0.6)] pub flee_door_dip: f64,
    #[k(0.9)] pub flee_door_mouth: f64,
    #[k(0.3)] pub flee_door_leaf: f64,
    #[k(0.9)] pub flee_door_via: f64,
    #[k(0.45)] pub flee_door_via_m: f64,
    #[k(30.0)] pub flee_door_new: f64,
    #[k(2.0)] pub flee_door_most: f64,
    #[k(4.0)] pub flee_door_try: f64,
    #[k(1500.0)] pub flee_door_nodes: f64,
    #[k(3.0)] pub def_share: f64,
    #[k(3.0)] pub def_max: f64,
    #[k(4.0)] pub def_fuse: f64,
    #[k(1.0)] pub def_wind: f64,
    #[k(3.5)] pub def_near: f64,
    #[k(13.0)] pub def_far: f64,
    #[k(2.0)] pub def_past: f64,
    #[k(4.0)] pub def_fresh: f64,
    #[k(1.2)] pub def_blocked: f64,
    #[k(2.5)] pub def_bunch: f64,
    #[k(0.5)] pub def_bunch_secs: f64,
    #[k(14.0)] pub def_gap: f64,
    #[k(6.0)] pub def_own_m: f64,
    #[k([0.6, 2.2])] pub def_scatter: [f64; 2],
    #[k(12.0)] pub cue_m: f64,
}
