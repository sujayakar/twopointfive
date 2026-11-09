//! The numbers of player.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// Sam's hands on the FN2000 (combat.ts fireWeapon's `spread`: ± half of it per axis on the unit direction): a shouldered long gun
///  standing still, on the move (any ground speed over `moveAt`), × crouched, × braced on the foregrip — which he now ALWAYS is with it in the hands (Player.braced:
///  no brace input any more, so braceK is simply the rifle's in-hand steadiness and `still` × braceK the cone a standing round is drawn with) — then + the AUTO string's
///  own growth off the row and + his suppression's, as the pistol's rounds get. The pistol stays dead-on.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RifleSpreadK {
    #[k(0.01)] pub still: f64,
    #[k(0.028)] pub moving: f64,
    #[k(0.3)] pub move_at: f64,
    #[k(0.7)] pub crouch_k: f64,
    #[k(0.45)] pub brace_k: f64,
}

/// ONE FOOTFALL A STEP (`once`; off = every crossing of the cycle's half or its turn files one, as it always did). A footfall is filed when the walk cycle
///  crosses its half or turns over. The cycle runs BACKWARDS while he backpedals, and backpedalling is decided from where he aims against where he goes: when
///  that flips frame by frame the cycle rocks over the one edge, and every frame of it filed a footfall — seven running in a scripted creep up to a man's back,
///  which is seven crouched steps in a tenth of a second to the man's ears. On: a crossing files a footfall only when the cycle has TRAVELLED `travel` of a turn
///  (either way, added up) since the last footfall filed. A plain walk travels half a turn between its edges and files what it always did; a cycle rocking on
///  an edge files at twice a walk's rate at the most, not sixty a second; a turn about within an eighth of a cycle of an edge loses that one footfall. (A FIRST
///  CUT asked only 'is this the edge last crossed?': a man pacing, or waving the cursor across himself twice a second, then walked the whole floor in silence.)
///  A FOOTFALL THE LEGS OWE (`owed`, a key of its own UNDER `once`: with the first key down it does nothing). A cycle that is turned about again and again
///  (the cursor shaken across him three times a second: forwards, backwards, forwards) either rocks between the two edges and reaches neither, or crosses one
///  with too little travel behind it to count: with the first key alone that walk files a third to two thirds of a plain walk's footfalls over the same
///  floor. On: the cycle's travel is also added up since the last EDGE PASSED (filed or not) or footfall filed, on the frames he moves at a footfall's speed,
///  and when it comes to `owe` of a turn (a half: one step's worth of leg) that step is filed where the legs are. A steady walk comes to its edge first,
///  every step, and files what it always did; after a footfall owed mid-stride the next edge passed puts the count back on the feet. The count starts over
///  when he slows under a footfall's speed (the cycle easing to a stop is not a step, and a debt is not kept while he stands) and on the first frame the
///  test is asked again after frames it was not (a takedown, a keypad, a kick: those beats walk or root him without it).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FootfallK {
    #[k(0.25)] pub travel: f64,
    #[k(0.5)] pub owe: f64,
}

/// TWO GEARS, NOT A DIAL (`on`; off = the wheel sets his pace and the crouch-walk is tune.playerCrouch, as it always was). The wheel's notches bought nothing a
/// man sneaking could use: a footfall's level is the stance's × the floor's whatever the notch, a crouched step is under a man's ear at any pace, and one notch
/// down a crouched man no longer gains on a patrol (1.13 against tune.guardWalk 1.15). On: the wheel is nobody's pace (his pace is whole, a saved notch
/// included), the walk and the sprint are as they were, and the crouch-walk IS the sneak, at `crouch` m/s — under the walk, so the walk keeps its job, and 0.3
/// m/s over a patrol's (the two metres from the close-in mode's edge to reach in about six seconds; it was thirteen). The sidle along a wall and a hauled body
/// keep tune.playerCrouch.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct TwogearsK {
    #[k(1.45)] pub crouch: f64,
}

/// SHIFT IN A CROUCH IS A FAST CROUCH WALK (the maintainer, 10-23: "we can make sprint when crouching a really fast crouch walk, but you have to stand up to sprint
/// properly"). With `on`, Shift held while he is crouched no longer stands him up into the sprint: he stays down and covers ground at `pace` (m/s; between the
/// walk's 1.55 and the run), each footfall at `loud` (the crouch walk's is 0.125, the walk's 0.26, a run's 0.6: quicker than walking and heard further than
/// walking, a good deal less than running), and it is no sneak. The sprint is the standing man's: [C] first. He is not `sprinting` — his hands are still his own (a
/// door, a switch, a grab) — and a hold on left click is the creep whatever Shift says, as before. A body in tow is let go of, as a sprint's is. Not under a puppet
/// (its Shift is the old one). Off = Shift stands him up, as it did. Its look: character.ts DASH.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CrouchrunK {
    #[k(2.5)] pub pace: f64,
    #[k(0.34)] pub loud: f64,
}
