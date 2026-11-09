//! The numbers of stealthpose.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// STEALTH POSES — how Sam carries himself with his hands empty and nobody on to him, posed against Chaos Theory's own clips (the maintainer's clips, cut into
/// beats: rebuild/takedowns-like-chaos-theory/clips/stealth-moves.md). The PLAYER only; picture and pose only.
///   IDLE    the crouch at a stand, hands empty: both arms hang forward of the knees off the shoulders, elbows a little out, forearms dropping in front of the
///           knees, hands open and loose between shin and knee height — and never a held frame: the weight goes foot to foot on a slow cycle (the pelvis a
///           centimetre to each side, feet where they were) with the hands drifting.
///   WALK    the crouch walk, hands empty: the hand opposite the forward foot comes forward low to knee height, fingers open; the other elbow goes back past
///           the hip. Read off the feet themselves, so it holds the stride he stopped in.
///   RISE    standing up: the hips come up as they always did and the ARMS follow over `riseSecs`, hanging low and forward until the trunk is up. Crouching
///           down: the elbows flare for the beat it takes the hands to come forward to the idle's place. Arms only: his trunk and head are the build's.
///   CREEP   the close-in mode near a man (close.ts): each step of the left foot led by the free (left) hand reaching low and forward. The right hand is
///           the knife's (close.ts, KIT.close) and the ready tell takes the left when it wants it.
///   CABLE   the look under a door (wand.ts): OVER the lock work's kneel, and it IS the pose (`bones`, the capsules and his eye go down with what is drawn): he shuffles half a step of pose in,
///           folds the trunk, bows the head below the shoulder line toward the foot of the door, shoulders rounded in, one arm forward and low at the gap, the
///           other by the knee. Out with the head first. `anim.lockpick`, wand.ts's rules, lens, clocks and rows are untouched.
/// One switch, STEALTHPOSE.on — the page: on (`?nostealthpose` = as it was); the frozen harness stand: off (`TPS_STEALTHPOSE=1` on) — and a half for each of
/// the five. No pace, key, reach, range or clock moves: `pos`, the yaws and the gait's cadence are the build's. What DOES differ for the simulation with it on
/// (so the frozen stand holds it off): poses feed the game — his arms are somewhere else (capsules), and the idle's sway moves his pelvis, chest and head a
/// centimetre. Outside the look under a door the trunk is never rounded and the pelvis never sunk; in it his head is some 30 cm lower than the lock work's kneel, never higher. No dice. No state of its own: the
/// animator's eased weights are the animator's (not in a checkpoint).
/// THIS FILE IS A LEAF (character.ts imports it, and character.ts is a leaf of the game's import graph): the table and the feed's shape only. The feed itself —
/// `stealthFeed(g)`, what the GAME knows — is stealthfeed.ts; character.ts `stealthLayer` / `stealthPicture` do the rest and stand down, eased, for whatever
/// else the animator itself knows has his arms. Nothing fed = nothing posed: with the switch off the layer is never entered.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct StealthposeK {
    /// the idle's weight shift: seconds a cycle · metres the pelvis goes to each side · metres the hands drift
    #[k(2.2)] pub sway_secs: f64,
    #[k(0.012)] pub sway_m: f64,
    #[k(0.016)] pub drift_m: f64,
    /// standing up: seconds the arms take to follow the hips
    #[k(0.9)] pub rise_secs: f64,
    /// the look under a door (the pose itself — his head and body really go down): seconds in · degrees the trunk bows past level · metres of pose he shuffles in
    #[k(0.4)] pub cable_in: f64,
    #[k(26.0)] pub cable_bow: f64,
    #[k(0.07)] pub cable_step: f64,
}
