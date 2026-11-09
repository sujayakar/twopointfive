//! The numbers of guards.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// ------------------------------------------------------------ bodies are found by the light on them (tune.bodyLight — review A6.2, Chaos Theory's rule)
/// A body in the dark is HIDDEN; one under a lamp, in a lit doorway or in the pool of a torch — the finder's own included — is found. What bodyInView asks on top of
/// its range / cone / line: the light where the body lies, read the way every eye in the game reads a man — the renderer's irradiance at the body's trunk (a probe
/// query keyed by the body, answered a frame or two later) and until it answers (the browser's first frames; headless, always) the CPU estimate of the same direct
/// light (lightEstimateAt: fixtures, lamps, the torches and weapon lights in the set — his own beam on it is simply one of those lights) — through the meter's own
/// curve (Game.visibilityOf), eased like everybody's meter; and the smoke between his eye and it (the probe's segment transmittance, as canSee marches it to Sam).
/// Off (Tab ▸ Sandbox) = range, cone and line alone: found in the pitch dark at twelve metres, as before.
/// the meter reading (0‥1) at the body at or above which it is plain from FAR_M and beyond (0.22 ≈ the fringe of a corridor panel's pool as the meter reads it; a
///  torch beam squarely on it reads 0.7+, open floor between two panels at night ~0.35, a dark office 0); inside NEAR_M he all but trips on it — found in any light
///  — and from NEAR to FAR the bar eases up from LIT_NEAR to LIT; SMOKE: eye → body transmittance under which the plume hides it (canSee's own line for a standing man)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BodyLightK {
    #[k(0.22)] pub lit: f64,
    #[k(0.04)] pub lit_near: f64,
    #[k(2.0)] pub near_m: f64,
    #[k(5.0)] pub far_m: f64,
    #[k(0.3)] pub smoke: f64,
}

/// a man's eye: 16 m of reach, 58° either side of where chest and head point between them, and inside 3.5 m a near field of 100° (a shape at his elbow is seen)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct GuardEyeK {
    #[k(16.0)] pub range: f64,
    #[k(58.0)] pub half_deg: f64,
    #[k(3.5)] pub near_m: f64,
    #[k(100.0)] pub near_deg: f64,
    /// …the awareness from which a WARY man's glimpse from past `range` is his fix (under it he has not watched long enough to know where)
    #[k(0.5)] pub far_fix_at: f64,
    /// …how far the eye reaches for a man in full light (m); the light on him (his meter, 0‥1) under which the reach is `range` and nothing changes, and from which it is whole
    #[k(26.0)] pub far_range: f64,
    #[k(0.4)] pub far_lo: f64,
    #[k(0.8)] pub far_hi: f64,
    /// …and how slowly the notice builds out there: sightGain's distance term runs on from its 0.2 at `range` down to this at `farRange`
    #[k(0.1)] pub far_k: f64,
}

/// HOW THE TEAM CARRIES ITSELF. A teammate (an operative unit, while the room fight is live) is a man with a rifle and a
///  plate carrier: `max` is the most his legs are ever asked for, whoever asks (an order's leg, a drill's flow, the fight AI's chase, a step off a doorway) — a
///  purposeful walk with some jog in it, none of the sprint clip (character.ts BLEND: the walk carries to 1.6 m/s, the sprint starts past 2.7) — and he LEANS INTO a
///  move: his speed rises by no more than `accel` m/s² (he stops as crisply as ever: a man arriving on a mark is on it). And his GUN TRAVELS: the runner
///   put a scripted man's muzzle on its aim point the frame the point changed — an order given, a corner checked, the fight's own read taking over
///  from behind his shoulder: a rifle, a head and a weapon light's cone through 60‥150° in one frame, a few times a minute per man (tools/qa/playthrough.ts
///  counted them) — now it comes round to a point an ORDER (or the unasked cover) gives it at `turn` rad/s at most (0 = on the point at once, as it was): 8 is
///  the operatives' own body turn — a quarter turn in a fifth of a second. A point his own EYES have given him
///  (GuardScript.over: the room fight's read, which eases itself — roomfight.ts point()) is not slowed: throttled too, the pair lost three more rooms in six.
///  What that point may not do is TELEPORT: when it passes behind his back the waist's twist clamp flips from one shoulder to the other, 150° and more in a
///  frame — that crossing is swung through his front at `swing` rad/s (0 = the flip, as it was; 30 is a half turn in a tenth of a second).
///  `on` false = everybody as they were.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PaceK {
    #[k(2.2)] pub max: f64,
    #[k(2.6)] pub accel: f64,
    #[k(8.0)] pub turn: f64,
    #[k(30.0)] pub swing: f64,
}

/// LIGHT DISCIPLINE (the maintainer, 10-05, off the merged build: "friendlies also need to have flashlight discipline"): a teammate under one of Sam's orders in
///  a live room fight burns his weapon light only while it helps — while he SEARCHES (the order sent him into a room or has him slicing one from outside, and
///  he has not yet stood on his mark with nothing new to look at for `afterSecs`), while his own eyes have his gun (roomfight's read: a man seen, a look at a
///  shot or a call), while the fight AI has him, and for `hotSecs` after a shot, a charge or a boot last reached his ears (never that last while he is posted
///  on a door he has not gone through: light under a door is the knock), for `stepSecs` after an ENEMY's footfall reached them along open air (roomfight's
///  Op.stepT — a man holding a room in the dark, or stacked with his back to a corridor, hears the man coming and lights him), and while he WALKS SOMEWHERE ON HIS OWN — to a mark an order gave him, more than
///  `onM` from it yet, or back to Sam from more than `aloneM` away: a man's eyes are what his light is on (sightGainOn), and a pair sent down a dark corridor
///  to a door with their lights off walked up to the man standing in it and were shot at the stack (rescue-probes, seed 14). In file behind Sam, on his mark,
///  stacked on a door, holding a corridor: dark (it was: on from the first entry and never off again — on the next door's stack, in every file after).
///  `tail` keeps it from flickering. Off (`on` false): the light as the order's flag and the unit's switch have it, as it was. Men a drill or the rescue's flow
///  moves (no order of Sam's on them) are not ruled: their lights are the drill's.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct LightsK {
    #[k(2.5)] pub after_secs: f64,
    #[k(5.0)] pub hot_secs: f64,
    #[k(4.0)] pub step_secs: f64,
    #[k(0.6)] pub tail: f64,
    #[k(0.8)] pub on_m: f64,
    #[k(7.0)] pub alone_m: f64,
    #[k(0.5)] pub lit_at: f64,
    #[k(15.0)] pub lit_m: f64,
    #[k(6.0)] pub lit_mark_m: f64,
    #[k(1.5)] pub wall_m: f64,
    #[k(1.9)] pub wall_back_m: f64,
    #[k(0.3)] pub wall_secs: f64,
}
