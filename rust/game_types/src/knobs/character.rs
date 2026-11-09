//! The numbers of character.
use crate::bytes::ToBytes;
use crate::js::*;
use crate::V3;
use alloc::vec::Vec;
use game_derive::Knobs;

/// The hold's contact constants. The two roots are locked HOLD.dist apart on ONE facing, so a point on the other man
///  is a constant in each figure's own model space (x = the figure's left, y up, z ahead of its root) — no bone hand-off between the characters. Timings are the
///  controller's too. First-pass numbers off the mannequin's measured joints; the viewer will get a page to tune them like RIG.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct HoldK {
    #[k(0.9)] pub regrab: f64,
    /// A HOLD WITH NO KNIFE (HeldPose.bare, fed by player.ts under game/close.ts CLOSE.noKnife): the held man's right hand was half up beside his head, open, palm
    ///  forward — frozen under a blade. With no blade it was a pale open hand waving over the pair, the brightest thing on screen. It is on the arm across his
    ///  throat instead, fingers hooked over it: this much of the choke's own clawing station, without the clawing (0 = the raised hand, as before).
    #[k(0.9)] pub bare_grip: f64,
    /// …and he is HAULED BACK onto the arm, not stood beside it (judged on stills from the game's camera: upright, Sam hid him and only his two elbows showed
    ///  either side of Sam's head). `bareArch`° more arch over the spine while he has his feet, his elbows carried down (`bareElbow` of the way to hanging). And
    ///  THE YANK (HoldPose / HeldPose.yank = seconds since Sam's step in landed, fed by player.ts with the stalk's grab): for `in` s his chest goes back `arch`°
    ///  further and Sam sits `sink` m into it, held `hold` s, let out over `out` s — the pose's share of the haul back.
    #[k(-11.0)] pub bare_arch: f64,
    #[k(0.6)] pub bare_elbow: f64,
    pub yank: HoldKYank,
    /// THE SPRING BEFORE THE LUNGE (the fourth pass; `HoldPose.step` = seconds into the stalk's step in): his hips dip `dip` m and sit `back` m for a few
    ///  frames as the step begins — [in, hold, out] s — so the lunge comes off something. And THE HOLD IS ALIVE (bare hands only): once he is held the pair
    ///  shifts its weight from foot to foot, `sway.m` m at the hips each way every `sway.secs` s, the held man a little more and `sway.twist`° through his
    ///  shoulders against the arm, scaled up by his agitation. (The stalk's polish: dip, back, sway.m, sway.twist, apart, and
    ///  yank.arch, which holdPolish writes.)
    pub lunge: HoldKLunge,
    pub sway: HoldKSway,
    pub apart: HoldKApart,
    pub catch: HoldKCatch,
    /// …AND HE IS LAID DOWN AHEAD OF SAM (the ninth pass; `layAhead` m, 0 = as it was). The choke's end is the 'sink' fall (fallGuide): onto his knees, then flat
    /// on his front — and flat, his pelvis stayed where it knelt and his legs were drawn out straight BEHIND it, 85 cm back along the floor: under Sam, who stood
    /// with a boot on the man's shin. A kneeling man who goes onto his face pivots about his knees, and Sam lets him down away from himself: the pelvis is led
    /// that much further along his facing, two fifths of it as he goes to his knees and the rest as he is laid flat, so his feet lie clear ahead of Sam's toes
    /// whichever way the fall's die tips him. takedown.ts setFall asks the level for the room, a strip as wide as his shoulders and as long as the lay and the
    /// whole of him: half the lay where only that much is clear, none where a wall, a desk or a low crate is in the way. The choke's end only — the knife from
    /// behind falls as it did.
    #[k(0.9)] pub lay_ahead: f64,
    pub hit: HoldKHit,
    pub first: HoldKFirst,
    /// …its numbers. Seconds from the grab's first frame: `take` Sam's hand has the gun (it leaves the man's) · `out` it is torn out to the side · `drop` he lets
    /// it go · `knife` the knife is in that fist (it is drawn as the hand comes back up over the man's shoulder, not before) · `home` the hand is back at its
    /// station on him and the move is over · `free` the man's wrist is let go of · `up` his hand is where the hold has it. The man, his own model space: `meet` =
    /// where his gun hand is thrown to as the arm lands — out beside his ribs, elbow flared, inside Sam's reach (in front of his belly no arm reaches it from
    /// behind him) — and `torn` where the wrench drags it. Sam, his: `over` = his wrist from the man's (over the top of the gun, a hand behind it), `wrench` = the
    /// tear from there (out to his right, down, back toward himself), `low` where he lets it go (a straight arm at his right thigh), `yaw`° his chest goes with the
    /// tear; `rap` = the pistol hold's wrist from the man's as the frame comes down on it; `hold` m from Sam's wrist to the gun's middle along his hand, [pistol,
    /// rifle]; `pitch`° the muzzle is down as he takes it · carries it out · lets it go. The fall: `rest` the bounce it lands with (dead), `push` m/s it leaves the
    /// hand at [out to his right, down], turning flat at `spin` rad/s. SECOND PASS (judged on stills from the game's camera): the gun was let go from a low hand at
    /// his thigh and fell straight down — it read as SET down, slid down his leg like a cane and lay under his right foot. He now tears it out to a straight arm at
    /// his right (`low`), his chest going with it (`yaw`), and lets it fly from there: it lands most of a metre clear of the pair at the same moment as before (the
    /// sounds' beats are the fall's time, which is unchanged). THIRD PASS: let go from a thigh-high hand with the muzzle down it still read as a drop behind him —
    /// he lets it go earlier (`drop`) from a LEVEL arm, the gun level, and the arm stays out `follow` s after it. FOURTH PASS (10-24, the maintainer's own beats,
    /// set by ear on the listening page: "strip: new 2 at 0.25 s · gun down: in the game at 0.70 s"): the take later (`take` 0.2 → 0.26), the tear in two thirds of
    /// the time (`out` 0.34 → 0.35, `drop` 0.42 → 0.41: the strip's own click is 0.13 s into it, just before he lets go), and THE FALL IS TIMED: COUNTED IN FRAMES
    /// OF THE PAGE, where the grab begins inside an update (the catch is heard on that frame and this move's clock already reads one step at its end): the strip is
    /// 15 frames after the catch (0.25 s) and the knock 42 (0.70 s; it was 0.80). `take` and `drop` sit in the middle of the frame they must fall in, and 0.311 is
    /// the middle of the falls that land 18 frames after the let-go (0.303 to 0.318). `fall` 0 or absent = the fixed push, as a recording from before it has it.
    /// With `stripFirst` up he lets go sooner and the fall is longer by that many whole frames, so the knock is on the same frame from the grab's start in either
    /// order. `push` down is still what a gun rapped out, stopped at a wall or let fall with no hold leaves at. `free` follows `take` (the man's hand is held torn
    /// until it). The sounds' page rows hold the same times (sounds/catalog.ts; grabvoice-check takes him by the key, as the page does, and ties them to the
    /// frame).
    pub strip: HoldKStrip,
    /// his root this far dead ahead of Sam's, both on Sam's facing: chest to his back — any further and no arm reaches round a throat with the forearm ACROSS it
    ///  (the mannequin's upper arm is 0.27: the elbow has to sit on his left collarbone)
    #[k(0.27)] pub dist: f64,
    /// seconds: the reach before he is held · Space held this long on his row starts the choke (arm) / the whip (gun) · of tightening before he goes limp · from the
    ///  pistol coming off his head to it landing behind his ear (the strike arrives AT this instant; Space up sooner and it never does) · E with him in the arm: the
    ///  draw / the holstering — the right hand goes back over his shoulder, changes tools there and comes in with the other (holdLayer, SWAP_RATE), and for this
    ///  long the row starts nothing (player.ts swapHold's lockout: no round, no question, no choke, no whip) · the knife across his throat, from the free hand
    ///  leaving for his mouth to him let go of (the cut lands HOLD.kill.hit of the way in)
    pub secs: HoldKSecs,
    /// ---- Sam, his model space (the man's root sits at [0, 0, dist]; his neck joint stands about [0, 1.43, dist − 0.02], the front of his throat ~7 cm ahead of that)
    /// left wrist: past the middle of the throat to its far side, the forearm's line a finger off the throat — so it lies across it, elbow on his left collarbone
    #[k([-0.11, 1.44, 0.35])] pub throat: V3,
    /// …reached round the OUTSIDE of his left shoulder and OVER his collarbone onto the throat (the grab's mid keys), never through him
    #[k([0.38, 1.42, 0.22])] pub via_l: V3,
    #[k([0.16, 1.52, 0.33])] pub over_l: V3,
    /// the choke: the elbow closes, the wrist comes back a finger toward Sam's own chest (the throat gives; the spine behind it does not)
    #[k([-0.08, 1.45, 0.33])] pub tight: V3,
    /// degrees over the three spine joints: chest onto his back · leaning back to finish it
    #[k(8.0)] pub lean: f64,
    #[k(-4.0)] pub choke_lean: f64,
    /// chin down a touch (the side the head goes and its yaw are the knife's, below: the blade hand has the right)
    #[k(6.0)] pub head_pitch: f64,
    /// degrees of roll Sam's trunk sways AGAINST the man's thrashing through a choke (he is what the man is fighting)
    #[k(2.5)] pub brace: f64,
    /// ---- the knife (the arm variant's right hand; rigProps.knifeBoxes draws it out of the fist's thumb side, edge to the palm): held, the fist rides just ahead of
    ///      his own left wrist at the right-front of the man's throat with the blade laid level across the front of it under the chin — a finger proud of the
    ///      choking forearm, which keeps the throat itself — edge in toward the neck, tip a hand past its left side, the elbow up over the man's right shoulder.
    ///      Committing to the choke the blade comes OFF him (you do not put a man out with steel under his jaw, and his chin is about to drop onto that arm): the
    ///      fist goes down onto the front of his right shoulder and turns thumb-up, so the blade stands up and out past the right of his head, clear of everyone.
    ///      Directions are [left, up, ahead], normalized on use; `blade` = where the tip points, `back` = out of the back of the hand (squared to the blade on use).
    pub knife: HoldKKnife,
    /// ---- the man, his own model space (Sam's forearm crosses in front of his throat: elbow about [0.15, 1.46, 0.02], wrist [-0.11, 1.44, 0.08])
    /// left wrist, held: the hand draped OVER the forearm where it comes off his own left collarbone, fingers hanging down its front (clear of the blade's tip) — open, not hooked round it: frozen, not fighting…
    #[k([0.14, 1.47, 0.04])] pub rest_l: V3,
    /// …and where it claws from once the choke bites: just below and ahead of the forearm, hooked up and back over it
    #[k([0.09, 1.39, 0.13])] pub claw_l: V3,
    /// right wrist, held: the hand half up beside his head, open, palm forward — 'easy…' (a pistol he was grabbed with stays in the fist, at the ceiling)
    #[k([-0.36, 1.53, 0.06])] pub raised_r: V3,
    /// the grab: both come up from wherever they were, the right round the outside
    #[k([-0.38, 1.2, 0.12])] pub via_up: V3,
    /// right wrist in the choke: onto the forearm too, both hands fighting it
    #[k([-0.12, 1.38, 0.15])] pub claw_r: V3,
    /// limp: the hands fall away (left, right)
    #[k(alloc::vec![[0.27, 0.98, 0.14], [-0.27, 0.98, 0.14]])] pub hang: Vec<V3>,
    /// degrees: spine arched back over three joints, head pulled back and tipped away to his left (off the blade hand, throat bared), shoulders hunched up
    #[k(-5.0)] pub arch: f64,
    #[k(-9.0)] pub chin_up: f64,
    #[k(8.0)] pub aside: f64,
    #[k(8.0)] pub clav_up: f64,
    /// degrees the head drops forward / tips as he goes
    #[k(26.0)] pub loll: f64,
    #[k(11.0)] pub loll_roll: f64,
    /// the choke's LOWERING: over the last stretch of the choke, as he goes limp, the
    /// man goes down onto his heels with his trunk folding forward and Sam sinks with him, bowed over him, the arm still round his neck — lowered, never dropped.
    /// `from` … `by` = the shares of the (eased) choke it starts at and is all the way down by; `up` = the spring (1/s) both come back up on when the choke is eased off (down, they follow the clock). The man: `sink` / `fwd` m the pelvis goes down / away from Sam over pinned feet, `hip` / `fold` degrees
    /// his pelvis and his spine pitch forward, `hang` = how far his hanging hands' marks go down / forward (further down and his arm runs out mid-way: the elbow snaps straight in a frame). Sam: `samSink` m of pelvis over pinned feet with the knees
    /// spread by `samKnees` (the man is between them) and `samFwd` m of it toward the man, `samLean` degrees more over him, `samLook` of that left on his head (he
    /// looks down at the man), `samArm` = how far the arm round the neck and the hand on his shoulder follow the neck (down, forward). FITTED TO THE SKELETON, the two
    /// halves together: with these the hand stays within 1.5 cm of where it sat on his neck standing, all the way down, and the arm never past 0.91 of its reach —
    /// more sink for the man, or less for Sam, and the arm runs out (the hand slides up off his neck onto his face).
    pub lower: HoldKLower,
    /// THE KILL FROM THE HOLD (phase 'kill', the arm variant; takedown.ts TAKEDOWN.kill — off, or the gun variant: the phase is drawn as 'held'). Shares of the
    /// clock (secs.kill): by `wind` Sam's free hand is off the throat and clamped over the man's mouth, his head hauled back onto Sam's shoulder, and the blade is
    /// set at the far side of his neck; the fist is drawn back across — the throat's middle at `hit` — and on OUT wide to Sam's right by `out`, elbow up (what the
    /// camera overhead reads); he stands cut for the beat between `hit` and `go`, his hands at his throat; from `go` his knees give and he goes forward and down,
    /// let down by the hand that has come onto his shoulder, never dropped — and Sam GOES WITH HIM, sinking and bowed over him, on his feet. Both layers follow an
    /// eased copy of the clock (hdKc / heKc). Let go of, Sam comes up off him at `after` 1/s while he stands still (holdOutRate).
    /// Sam, his model space: `mouth` → `chest` → `collar` = the left wrist's stations (over his mouth; across his chest for the beat he still stands; on the top
    /// of his far shoulder as he folds forward out of the arm — the hand that lets him down); `set` → `across` → `wide`
    /// = the knife fist's path, `low` where it settles after (blade / back as HOLD.knife's); lean° back into the haul · forward over him after, chest yaw° loading
    /// left · through with the cut, `samSink` m of pelvis over pinned feet, `look`° the head goes down after him.
    /// The man, his own: `reach` = his wrists FLUNG forward as the hand takes his mouth (left, right — a startle, out of the two men's outline from above, and
    /// his right arm out from under the blade's way; in at his neck they read as the choke), `clutch` = at his own throat after the cut; chin° / arch° hauled
    /// back, the head's jolt° [yaw, pitch] as the blade goes across, `sink` / `fwd` m and `hip` / `fold`° as he goes, `hang` as HOLD.lower's.
    pub kill: HoldKKill,
    /// THE CHOKE'S BEATS, off the clips (takedown.ts TAKEDOWN.chokeBeats; off = the choke as it was) — shares of the eased choke: `clamp` = over which both his hands
    /// come onto the forearm (a quarter second in, where they took 0.8 s), `off` = over which they come OFF it and fall, before the lowering starts (they fell
    /// with it, through it), `tipBy` = over which his head tips `tip`° to the side before it comes forward; eased off, the hands go back the way they came on a
    /// spring of their own, `back` 1/s (the choke's clock is back at rest in a tenth of a second: his arms are not). `lower` = HOLD.lower's table as the beats
    /// have it (both layers read this one with the half on, that one with it off — which keeps what every older digest has): started a touch earlier and all the
    /// way down by 85 %, so the last third of a second is HELD at the bottom, and deeper — his fold, Sam's lean and sink — the arm's follow refitted to it (the
    /// hand within 1.5 cm of where it sat on his neck, the arm to 0.94 of its reach). `after` = 1/s Sam comes up off him once he is out, while he stands still.
    pub beats: HoldKBeats,
    /// THE CHOKE'S HAUL (takedown.ts TAKEDOWN.haul; off = the choke as it was). The maintainer, 10-22: "the choking animation needs to be more intense. in chaos
    /// theory sam has him in a chokehold and then lifts him up and back out of the air a bit when choking". Off the clips (the choke seen from the front): a tenth
    /// of a second in Sam straightens and hauls him up, the man's head forced back into his chest, and leans back through the squeeze; the man is arched back
    /// onto him, hips ahead of his shoulders. Shares of the eased choke: `in` over which it comes on (a third of a second: quicker and a finger of his passes 5 cm in a frame), `out` over which he is
    /// set back on his feet — as his hands come off the arm, done by half way, before the lowering takes him down; let go of early it comes down at `down` 1/s. Sam: `samLean`° more over the spine (back, over planted feet — his head goes
    /// some 15 cm back, FURTHER than the man's, or the two heads meet: lifted 11 cm with Sam's hips pushed forward under the lean the man's cap covered Sam's head),
    /// `samArm` [up, back] m his two hands go with the man's neck. The man: his pelvis `lift` m up and `back` m toward Sam (under 0: FORWARD, with
    /// Sam's — his hips ahead of his shoulders, which go back with Sam's chest: tipped back from the shoulders the two heads met) and tipped `recline`°, `arch`°
    /// more arch and `chin`° more chin; his feet come `feet` [up, forward] m off where they stood, toes down `toes`°, and kick `kick`
    /// [forward, up] m with the fight. Pose only — but both men's head and chest bones move with it (sight lines and the light meter read Sam's).
    pub haul: HoldKHaul,
    /// m the pelvis sinks onto the knees: held … out on his feet at the end of the choke (no further: the arm under his jaw carries him until it lets go)
    #[k([0.02, 0.06])] pub sag: [f64; 2],
    /// held: hard breathing — chest heave (°, upper spine) and the shoulders lifting with each breath, at this rate
    pub breath: HoldKBreath,
    /// the choke's fight (heldLayer): an envelope on the eased choke amount — up fast as the arm closes, spent as he goes — driving a 2–3 Hz thrash of the trunk and
    /// head, the hands clawing up and down the forearm, the knees pumping; all off the figure's own desynced clock (deterministic), all pose-only
    pub struggle: HoldKStruggle,
    /// ---- the gun variant (HoldVariant 'gun'): the left arm exactly as above; the right hand keeps the Five-seveN. Measured, not wished: wrist to the can's mouth is
    ///      0.43 m and his temple stands 0.36 m ahead of Sam's shoulder, so a muzzle pressed INTO the temple folds the arm double behind Sam's own ear — instead the
    ///      forearm lies over the man's right shoulder and the can runs along his right temple a few centimetres off the skin, mouth just ahead of his brow, toed in
    ///      and tipped down at his head: 'gun to his head' from the game's camera, and already pointing past him when it fires. Sam's head goes out to the LEFT here
    ///      (the gun has the right). Directions are model-space [left, up, ahead] and get normalized on use.
    pub gun: HoldKGun,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKYank {
    #[k(-5.0)] pub arch: f64,
    #[k(0.09)] pub sink: f64,
    #[k(0.07)] pub in_: f64,
    #[k(0.05)] pub hold: f64,
    #[k(0.32)] pub out: f64,
}

/// THE SPRING BEFORE THE LUNGE (the fourth pass; `HoldPose.step` = seconds into the stalk's step in): his hips dip `dip` m and sit `back` m for a few
///  frames as the step begins — [in, hold, out] s — so the lunge comes off something. And THE HOLD IS ALIVE (bare hands only): once he is held the pair
///  shifts its weight from foot to foot, `sway.m` m at the hips each way every `sway.secs` s, the held man a little more and `sway.twist`° through his
///  shoulders against the arm, scaled up by his agitation. (The stalk's polish: dip, back, sway.m, sway.twist, apart, and
///  yank.arch, which holdPolish writes.)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKLunge {
    #[k(0.06)] pub dip: f64,
    #[k(0.035)] pub back: f64,
    #[k([0.035, 0.03, 0.1])] pub t: [f64; 3],
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKSway {
    #[k(0.014)] pub m: f64,
    #[k(1.9)] pub secs: f64,
    #[k(2.4)] pub twist: f64,
    #[k(1.5)] pub his: f64,
    #[k(0.6)] pub in_: f64,
    #[k(8.0)] pub out: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKApart {
    #[k(7.0)] pub side: f64,
    #[k(8.0)] pub tip: f64,
}

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct HoldKCatch {
    #[k([0.17, 1.49, 0.21])] pub via: V3,
    #[k([0.08, 1.465, 0.38])] pub over: V3,
    #[k([0.42, 0.74])] pub at: [f64; 2],
    #[k(0.3)] pub arm_in: f64,
    #[k([0.0, 0.3])] pub ready_out: [f64; 2],
    pub jolt: HoldKCatchJolt,
    pub fling: HoldKCatchFling,
    #[k(0.12)] pub sink: f64,
    #[k(0.13)] pub hips: f64,
    #[k([0.17, 0.27])] pub heels: [f64; 2],
    #[k([0.07, 0.05, 0.06])] pub seat: V3,
    #[k(7.0)] pub seat_lean: f64,
    #[k([0.15, 0.12])] pub brace: [f64; 2],
    #[k(0.06)] pub brace_lift: f64,
    #[k(0.5)] pub yank_sit: f64,
    #[k(alloc::vec![[0.0, 0.17], [0.15, 0.36]])] pub feet: Vec<[f64; 2]>,
    #[k(0.07)] pub lift: f64,
    #[k(alloc::vec![[0.04, 0.2], [0.0, 0.12]])] pub sam_feet: Vec<[f64; 2]>,
    #[k(0.05)] pub sam_lift: f64,
    #[k(0.4)] pub settle: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKCatchJolt {
    #[k([0.07, 0.13, 0.24, 0.36])] pub t: [f64; 4],
    #[k(12.0)] pub fwd: f64,
    #[k(-16.0)] pub head: f64,
    #[k(6.0)] pub clav: f64,
    #[k(0.03)] pub dip: f64,
}

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct HoldKCatchFling {
    #[k([0.24, 0.47])] pub t: [f64; 2],
    #[k([0.73, 0.86])] pub land: [f64; 2],
    #[k(alloc::vec![[0.44, 1.08, 0.14], [-0.36, 1.27, 0.08]])] pub to: Vec<V3>,
    #[k([0.03, 0.05, -0.04])] pub drift: V3,
    #[k([0.05, 0.2])] pub arm_in: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKHit {
    #[k([0.0, 0.04, 0.07, 0.22])] pub t: [f64; 4],
    #[k(0.15)] pub hips: f64,
    #[k(0.03)] pub dip: f64,
    #[k(1.0)] pub head: f64,
    #[k(12.0)] pub heave: f64,
    #[k(0.05)] pub sam_in: f64,
    #[k(0.06)] pub sam_dip: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKFirst {
    #[k(0.16)] pub take: f64,
    #[k(0.3)] pub drop: f64,
    #[k(0.26)] pub catch: f64,
}

/// …its numbers. Seconds from the grab's first frame: `take` Sam's hand has the gun (it leaves the man's) · `out` it is torn out to the side · `drop` he lets
/// it go · `knife` the knife is in that fist (it is drawn as the hand comes back up over the man's shoulder, not before) · `home` the hand is back at its
/// station on him and the move is over · `free` the man's wrist is let go of · `up` his hand is where the hold has it. The man, his own model space: `meet` =
/// where his gun hand is thrown to as the arm lands — out beside his ribs, elbow flared, inside Sam's reach (in front of his belly no arm reaches it from
/// behind him) — and `torn` where the wrench drags it. Sam, his: `over` = his wrist from the man's (over the top of the gun, a hand behind it), `wrench` = the
/// tear from there (out to his right, down, back toward himself), `low` where he lets it go (a straight arm at his right thigh), `yaw`° his chest goes with the
/// tear; `rap` = the pistol hold's wrist from the man's as the frame comes down on it; `hold` m from Sam's wrist to the gun's middle along his hand, [pistol,
/// rifle]; `pitch`° the muzzle is down as he takes it · carries it out · lets it go. The fall: `rest` the bounce it lands with (dead), `push` m/s it leaves the
/// hand at [out to his right, down], turning flat at `spin` rad/s. SECOND PASS (judged on stills from the game's camera): the gun was let go from a low hand at
/// his thigh and fell straight down — it read as SET down, slid down his leg like a cane and lay under his right foot. He now tears it out to a straight arm at
/// his right (`low`), his chest going with it (`yaw`), and lets it fly from there: it lands most of a metre clear of the pair at the same moment as before (the
/// sounds' beats are the fall's time, which is unchanged). THIRD PASS: let go from a thigh-high hand with the muzzle down it still read as a drop behind him —
/// he lets it go earlier (`drop`) from a LEVEL arm, the gun level, and the arm stays out `follow` s after it. FOURTH PASS (10-24, the maintainer's own beats,
/// set by ear on the listening page: "strip: new 2 at 0.25 s · gun down: in the game at 0.70 s"): the take later (`take` 0.2 → 0.26), the tear in two thirds of
/// the time (`out` 0.34 → 0.35, `drop` 0.42 → 0.41: the strip's own click is 0.13 s into it, just before he lets go), and THE FALL IS TIMED: COUNTED IN FRAMES
/// OF THE PAGE, where the grab begins inside an update (the catch is heard on that frame and this move's clock already reads one step at its end): the strip is
/// 15 frames after the catch (0.25 s) and the knock 42 (0.70 s; it was 0.80). `take` and `drop` sit in the middle of the frame they must fall in, and 0.311 is
/// the middle of the falls that land 18 frames after the let-go (0.303 to 0.318). `fall` 0 or absent = the fixed push, as a recording from before it has it.
/// With `stripFirst` up he lets go sooner and the fall is longer by that many whole frames, so the knock is on the same frame from the grab's start in either
/// order. `push` down is still what a gun rapped out, stopped at a wall or let fall with no hold leaves at. `free` follows `take` (the man's hand is held torn
/// until it). The sounds' page rows hold the same times (sounds/catalog.ts; grabvoice-check takes him by the key, as the page does, and ties them to the
/// frame).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKStrip {
    #[k(0.26)] pub take: f64,
    #[k(0.35)] pub out: f64,
    #[k(0.41)] pub drop: f64,
    #[k(0.1)] pub follow: f64,
    #[k(0.7)] pub knife: f64,
    #[k(0.95)] pub home: f64,
    #[k(0.36)] pub free: f64,
    #[k(0.68)] pub up: f64,
    #[k([-0.36, 1.15, 0.05])] pub meet: V3,
    #[k([-0.47, 1.06, -0.05])] pub torn: V3,
    #[k([0.0, 0.035, -0.05])] pub over: V3,
    #[k([-0.15, -0.1, -0.2])] pub wrench: V3,
    #[k([-0.66, 1.2, -0.08])] pub low: V3,
    #[k(-16.0)] pub yaw: f64,
    #[k([0.02, 0.08, -0.03])] pub rap: V3,
    #[k([0.08, 0.02])] pub hold: [f64; 2],
    #[k([12.0, 25.0, 15.0])] pub pitch: V3,
    #[k(0.05)] pub rest: f64,
    #[k([1.3, 1.05])] pub push: [f64; 2],
    #[k(0.311)] pub fall: f64,
    #[k(5.0)] pub spin: f64,
}

/// seconds: the reach before he is held · Space held this long on his row starts the choke (arm) / the whip (gun) · of tightening before he goes limp · from the
///  pistol coming off his head to it landing behind his ear (the strike arrives AT this instant; Space up sooner and it never does) · E with him in the arm: the
///  draw / the holstering — the right hand goes back over his shoulder, changes tools there and comes in with the other (holdLayer, SWAP_RATE), and for this
///  long the row starts nothing (player.ts swapHold's lockout: no round, no question, no choke, no whip) · the knife across his throat, from the free hand
///  leaving for his mouth to him let go of (the cut lands HOLD.kill.hit of the way in)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKSecs {
    #[k(0.45)] pub grab: f64,
    #[k(0.35)] pub choke_hold: f64,
    #[k(2.6)] pub choke: f64,
    #[k(0.6)] pub whip: f64,
    #[k(0.3)] pub swap: f64,
    #[k(1.3)] pub kill: f64,
}

/// ---- the knife (the arm variant's right hand; rigProps.knifeBoxes draws it out of the fist's thumb side, edge to the palm): held, the fist rides just ahead of
///      his own left wrist at the right-front of the man's throat with the blade laid level across the front of it under the chin — a finger proud of the
///      choking forearm, which keeps the throat itself — edge in toward the neck, tip a hand past its left side, the elbow up over the man's right shoulder.
///      Committing to the choke the blade comes OFF him (you do not put a man out with steel under his jaw, and his chin is about to drop onto that arm): the
///      fist goes down onto the front of his right shoulder and turns thumb-up, so the blade stands up and out past the right of his head, clear of everyone.
///      Directions are [left, up, ahead], normalized on use; `blade` = where the tip points, `back` = out of the back of the hand (squared to the blade on use).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKKnife {
    /// held: wrist over the blade line, the fist beside his right jaw (knuckles down, back of the hand up-forward → the flat lies all but level, the tip just past the left of his neck, toed a touch forward)
    #[k([-0.165, 1.555, 0.36])] pub hand: V3,
    #[k([1.0, 0.0, 0.12])] pub blade: V3,
    #[k([0.0, 0.28, 0.96])] pub back: V3,
    /// the grab: up behind and outside his right shoulder, then in over it — never through him
    #[k([-0.4, 1.5, 0.04])] pub via: V3,
    /// the choke: fist on the front of his right shoulder, blade up-forward-out (the knuckles come out forward-down, as a hand clamped there would)
    #[k([-0.24, 1.45, 0.26])] pub off_hand: V3,
    #[k([-0.35, 0.75, 0.56])] pub off_blade: V3,
    #[k([-0.9, -0.3, 0.0])] pub off_back: V3,
    /// about how long the blade takes to come off him once the choke starts (a spring; twice that coming back from an aborted one)
    #[k(0.4)] pub off_secs: f64,
    /// Sam's head carried out to the LEFT of the man's, looking past his left ear (as the gun hold: the right is the weapon hand's)
    #[k(-8.0)] pub side: f64,
    #[k(19.0)] pub head_yaw: f64,
    /// right clavicle up with the raised knife arm (°)
    #[k(5.0)] pub clav: f64,
}

/// the choke's LOWERING: over the last stretch of the choke, as he goes limp, the
/// man goes down onto his heels with his trunk folding forward and Sam sinks with him, bowed over him, the arm still round his neck — lowered, never dropped.
/// `from` … `by` = the shares of the (eased) choke it starts at and is all the way down by; `up` = the spring (1/s) both come back up on when the choke is eased off (down, they follow the clock). The man: `sink` / `fwd` m the pelvis goes down / away from Sam over pinned feet, `hip` / `fold` degrees
/// his pelvis and his spine pitch forward, `hang` = how far his hanging hands' marks go down / forward (further down and his arm runs out mid-way: the elbow snaps straight in a frame). Sam: `samSink` m of pelvis over pinned feet with the knees
/// spread by `samKnees` (the man is between them) and `samFwd` m of it toward the man, `samLean` degrees more over him, `samLook` of that left on his head (he
/// looks down at the man), `samArm` = how far the arm round the neck and the hand on his shoulder follow the neck (down, forward). FITTED TO THE SKELETON, the two
/// halves together: with these the hand stays within 1.5 cm of where it sat on his neck standing, all the way down, and the arm never past 0.91 of its reach —
/// more sink for the man, or less for Sam, and the arm runs out (the hand slides up off his neck onto his face).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKLower {
    #[k(0.6)] pub from: f64,
    #[k(1.0)] pub by: f64,
    #[k(7.0)] pub up: f64,
    #[k(0.4)] pub sink: f64,
    #[k(0.05)] pub fwd: f64,
    #[k(8.0)] pub hip: f64,
    #[k(20.0)] pub fold: f64,
    #[k([0.56, 0.26])] pub hang: [f64; 2],
    #[k(0.3)] pub sam_sink: f64,
    #[k(0.06)] pub sam_fwd: f64,
    #[k(0.55)] pub sam_knees: f64,
    #[k(38.0)] pub sam_lean: f64,
    #[k(0.45)] pub sam_look: f64,
    #[k([0.5, 0.22])] pub sam_arm: [f64; 2],
}

/// THE KILL FROM THE HOLD (phase 'kill', the arm variant; takedown.ts TAKEDOWN.kill — off, or the gun variant: the phase is drawn as 'held'). Shares of the
/// clock (secs.kill): by `wind` Sam's free hand is off the throat and clamped over the man's mouth, his head hauled back onto Sam's shoulder, and the blade is
/// set at the far side of his neck; the fist is drawn back across — the throat's middle at `hit` — and on OUT wide to Sam's right by `out`, elbow up (what the
/// camera overhead reads); he stands cut for the beat between `hit` and `go`, his hands at his throat; from `go` his knees give and he goes forward and down,
/// let down by the hand that has come onto his shoulder, never dropped — and Sam GOES WITH HIM, sinking and bowed over him, on his feet. Both layers follow an
/// eased copy of the clock (hdKc / heKc). Let go of, Sam comes up off him at `after` 1/s while he stands still (holdOutRate).
/// Sam, his model space: `mouth` → `chest` → `collar` = the left wrist's stations (over his mouth; across his chest for the beat he still stands; on the top
/// of his far shoulder as he folds forward out of the arm — the hand that lets him down); `set` → `across` → `wide`
/// = the knife fist's path, `low` where it settles after (blade / back as HOLD.knife's); lean° back into the haul · forward over him after, chest yaw° loading
/// left · through with the cut, `samSink` m of pelvis over pinned feet, `look`° the head goes down after him.
/// The man, his own: `reach` = his wrists FLUNG forward as the hand takes his mouth (left, right — a startle, out of the two men's outline from above, and
/// his right arm out from under the blade's way; in at his neck they read as the choke), `clutch` = at his own throat after the cut; chin° / arch° hauled
/// back, the head's jolt° [yaw, pitch] as the blade goes across, `sink` / `fwd` m and `hip` / `fold`° as he goes, `hang` as HOLD.lower's.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct HoldKKill {
    #[k(0.21)] pub wind: f64,
    #[k(0.3)] pub hit: f64,
    #[k(0.48)] pub out: f64,
    #[k(0.46)] pub go: f64,
    #[k(7.0)] pub after: f64,
    #[k([0.07, 1.6, 0.36])] pub mouth: V3,
    #[k([-0.1, 1.28, 0.36])] pub chest: V3,
    #[k([0.14, 0.98, 0.5])] pub collar: V3,
    #[k([0.06, 1.48, 0.29])] pub set: V3,
    #[k([-0.2, 1.48, 0.33])] pub across: V3,
    #[k([-0.62, 1.4, 0.04])] pub wide: V3,
    #[k([-0.34, 1.05, 0.2])] pub low: V3,
    #[k([0.55, -0.15, 0.8])] pub wide_blade: V3,
    #[k([-0.2, 0.95, 0.2])] pub wide_back: V3,
    #[k([-12.0, 20.0])] pub lean: [f64; 2],
    #[k([16.0, -34.0])] pub yaw: [f64; 2],
    #[k(0.22)] pub sam_sink: f64,
    #[k(24.0)] pub look: f64,
    #[k(alloc::vec![[0.28, 1.28, 0.32], [-0.3, 1.3, 0.34]])] pub reach: Vec<V3>,
    #[k(alloc::vec![[0.05, 1.41, 0.1], [-0.05, 1.4, 0.11]])] pub clutch: Vec<V3>,
    #[k(-27.0)] pub chin: f64,
    #[k(-12.0)] pub arch: f64,
    #[k([-12.0, -7.0])] pub jolt: [f64; 2],
    #[k(0.34)] pub sink: f64,
    #[k(0.08)] pub fwd: f64,
    #[k(8.0)] pub hip: f64,
    #[k(36.0)] pub fold: f64,
    #[k([0.2, 0.2])] pub hang: [f64; 2],
}

/// THE CHOKE'S BEATS, off the clips (takedown.ts TAKEDOWN.chokeBeats; off = the choke as it was) — shares of the eased choke: `clamp` = over which both his hands
/// come onto the forearm (a quarter second in, where they took 0.8 s), `off` = over which they come OFF it and fall, before the lowering starts (they fell
/// with it, through it), `tipBy` = over which his head tips `tip`° to the side before it comes forward; eased off, the hands go back the way they came on a
/// spring of their own, `back` 1/s (the choke's clock is back at rest in a tenth of a second: his arms are not). `lower` = HOLD.lower's table as the beats
/// have it (both layers read this one with the half on, that one with it off — which keeps what every older digest has): started a touch earlier and all the
/// way down by 85 %, so the last third of a second is HELD at the bottom, and deeper — his fold, Sam's lean and sink — the arm's follow refitted to it (the
/// hand within 1.5 cm of where it sat on his neck, the arm to 0.94 of its reach). `after` = 1/s Sam comes up off him once he is out, while he stands still.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKBeats {
    #[k([0.03, 0.14])] pub clamp: [f64; 2],
    #[k([0.41, 0.6])] pub off: [f64; 2],
    #[k([0.5, 0.62])] pub tip_by: [f64; 2],
    #[k(32.0)] pub tip: f64,
    #[k(7.0)] pub back: f64,
    #[k(2.6)] pub after: f64,
    pub lower: HoldKBeatsLower,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKBeatsLower {
    #[k(0.58)] pub from: f64,
    #[k(0.85)] pub by: f64,
    #[k(7.0)] pub up: f64,
    #[k(0.4)] pub sink: f64,
    #[k(0.05)] pub fwd: f64,
    #[k(8.0)] pub hip: f64,
    #[k(38.0)] pub fold: f64,
    #[k([0.56, 0.26])] pub hang: [f64; 2],
    #[k(0.38)] pub sam_sink: f64,
    #[k(0.06)] pub sam_fwd: f64,
    #[k(0.55)] pub sam_knees: f64,
    #[k(48.0)] pub sam_lean: f64,
    #[k(0.45)] pub sam_look: f64,
    #[k([0.52, 0.29])] pub sam_arm: [f64; 2],
}

/// THE CHOKE'S HAUL (takedown.ts TAKEDOWN.haul; off = the choke as it was). The maintainer, 10-22: "the choking animation needs to be more intense. in chaos
/// theory sam has him in a chokehold and then lifts him up and back out of the air a bit when choking". Off the clips (the choke seen from the front): a tenth
/// of a second in Sam straightens and hauls him up, the man's head forced back into his chest, and leans back through the squeeze; the man is arched back
/// onto him, hips ahead of his shoulders. Shares of the eased choke: `in` over which it comes on (a third of a second: quicker and a finger of his passes 5 cm in a frame), `out` over which he is
/// set back on his feet — as his hands come off the arm, done by half way, before the lowering takes him down; let go of early it comes down at `down` 1/s. Sam: `samLean`° more over the spine (back, over planted feet — his head goes
/// some 15 cm back, FURTHER than the man's, or the two heads meet: lifted 11 cm with Sam's hips pushed forward under the lean the man's cap covered Sam's head),
/// `samArm` [up, back] m his two hands go with the man's neck. The man: his pelvis `lift` m up and `back` m toward Sam (under 0: FORWARD, with
/// Sam's — his hips ahead of his shoulders, which go back with Sam's chest: tipped back from the shoulders the two heads met) and tipped `recline`°, `arch`°
/// more arch and `chin`° more chin; his feet come `feet` [up, forward] m off where they stood, toes down `toes`°, and kick `kick`
/// [forward, up] m with the fight. Pose only — but both men's head and chest bones move with it (sight lines and the light meter read Sam's).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKHaul {
    #[k([0.02, 0.14])] pub in_: [f64; 2],
    #[k([0.38, 0.52])] pub out: [f64; 2],
    #[k(7.0)] pub down: f64,
    #[k(-26.0)] pub sam_lean: f64,
    #[k([0.07, 0.08])] pub sam_arm: [f64; 2],
    #[k(0.09)] pub lift: f64,
    #[k(-0.08)] pub back: f64,
    #[k(-14.0)] pub recline: f64,
    #[k(-4.0)] pub arch: f64,
    #[k(-6.0)] pub chin: f64,
    #[k([0.11, 0.18])] pub feet: [f64; 2],
    #[k(30.0)] pub toes: f64,
    #[k([0.07, 0.05])] pub kick: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKBreath {
    /// held: hard breathing — chest heave (°, upper spine) and the shoulders lifting with each breath, at this rate
    #[k(0.7)] pub hz: f64,
    #[k(1.6)] pub pitch: f64,
    #[k(2.2)] pub clav: f64,
}

/// the choke's fight (heldLayer): an envelope on the eased choke amount — up fast as the arm closes, spent as he goes — driving a 2–3 Hz thrash of the trunk and
/// head, the hands clawing up and down the forearm, the knees pumping; all off the figure's own desynced clock (deterministic), all pose-only
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKStruggle {
    #[k(0.1)] pub rise: f64,
    #[k(0.35)] pub fade_from: f64,
    #[k(0.92)] pub fade_to: f64,
    #[k(7.0)] pub yaw: f64,
    #[k(6.0)] pub roll: f64,
    #[k(9.0)] pub head: f64,
    #[k(0.035)] pub claw: f64,
    #[k(0.018)] pub dip: f64,
    #[k([2.3, 3.1, 1.9])] pub hz: V3,
}

/// ---- the gun variant (HoldVariant 'gun'): the left arm exactly as above; the right hand keeps the Five-seveN. Measured, not wished: wrist to the can's mouth is
///      0.43 m and his temple stands 0.36 m ahead of Sam's shoulder, so a muzzle pressed INTO the temple folds the arm double behind Sam's own ear — instead the
///      forearm lies over the man's right shoulder and the can runs along his right temple a few centimetres off the skin, mouth just ahead of his brow, toed in
///      and tipped down at his head: 'gun to his head' from the game's camera, and already pointing past him when it fires. Sam's head goes out to the LEFT here
///      (the gun has the right). Directions are model-space [left, up, ahead] and get normalized on use.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct HoldKGun {
    /// at rest: wrist over his right shoulder, bore toed in ~12° and down ~6° across his temple
    #[k([-0.2, 1.53, 0.15])] pub hand: V3,
    #[k([0.21, -0.1, 0.97])] pub bore: V3,
    /// presented to fire past him: a hand further out and forward, bore dead ahead at the aim's pitch (HoldPose.gunPitch)
    #[k([-0.24, 1.53, 0.17])] pub out_hand: V3,
    /// recoil: muzzle flip (°) and the hand coming back (m), ~35 ms up, ~0.2 s down (the drawn pistol rides the aim while the shot plays, like every carry's: keep the flip to what the wrist shows)
    #[k(6.0)] pub kick_pitch: f64,
    #[k(0.03)] pub kick_back: f64,
    /// the grab: the gun hand swung out wide to the right and up round him — never through his back
    #[k([-0.42, 1.24, 0.06])] pub via_a: V3,
    #[k([-0.4, 1.5, 0.02])] pub via_b: V3,
    /// …muzzle high while it travels
    #[k([0.1, 0.6, 0.8])] pub via_bore: V3,
    /// the whip: cocked back, up and out (muzzle skyward)…
    #[k([-0.36, 1.68, -0.04])] pub wind_hand: V3,
    #[k([-0.15, 0.8, 0.58])] pub wind_bore: V3,
    /// …and brought down onto the skull behind his right ear (the frame just ahead of and below the wrist is what lands: it meets the bone, the boxes say so)
    #[k([-0.1, 1.62, 0.15])] pub hit_hand: V3,
    #[k([0.5, 0.3, 0.8])] pub hit_bore: V3,
    /// follow-through past the contact as he drops out of the arm (played on the layer's way out)
    #[k([-0.06, 1.5, 0.2])] pub thru_hand: V3,
    /// Sam: less chest on him; head carried out to the left, looking past the man's LEFT ear
    #[k(5.0)] pub lean: f64,
    #[k(-8.0)] pub side: f64,
    #[k(19.0)] pub head_yaw: f64,
    #[k(4.0)] pub head_pitch: f64,
    /// chest loading back to the right in the wind-up, coming through with the strike (°)
    #[k(-8.0)] pub wind_yaw: f64,
    #[k(5.0)] pub hit_yaw: f64,
    /// right clavicle up with the raised gun arm (°)
    #[k(7.0)] pub clav: f64,
    /// the man, his own model space
    /// hands half raised in front of his shoulders, palms forward (the torch / pistol in the right points at the ceiling)
    #[k(alloc::vec![[0.27, 1.43, 0.31], [-0.34, 1.41, 0.29]])] pub hands_up: Vec<V3>,
    /// head tipped and turned away to his left off the can at his right temple, chin not so high
    #[k(13.0)] pub aside: f64,
    #[k(6.0)] pub away_yaw: f64,
    #[k(-6.0)] pub chin_up: f64,
    /// the flinch when a round goes past his ear (°, and how long it takes to come back)
    pub duck: HoldKGunDuck,
    /// the head going with the blow over the strike's last 0.1 s (seeds the ragdoll's head that way)
    pub knock: HoldKGunKnock,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKGunDuck {
    /// the flinch when a round goes past his ear (°, and how long it takes to come back)
    #[k(5.0)] pub yaw: f64,
    #[k(9.0)] pub roll: f64,
    #[k(10.0)] pub pitch: f64,
    #[k(6.0)] pub hunch: f64,
    #[k(0.45)] pub secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldKGunKnock {
    /// the head going with the blow over the strike's last 0.1 s (seeds the ragdoll's head that way)
    #[k(24.0)] pub yaw: f64,
    #[k(16.0)] pub roll: f64,
    #[k(18.0)] pub pitch: f64,
}

/// SAM SPRINTS WITH BOTH HANDS ON THE PISTOL (the maintainer, 10-25: "sprinting with pistol animation is weird, should keep two hands on it and at low ready,
/// like rifle sprinting animation"). A sprint took the pistol's layer off (`upper` 'none'), so the gun rode a pumping fist while the rifle's sprint is the
/// long gun's low ready in both hands. With `on`, a figure told `runReady` (player.ts: the sprint, the pistol in his hand) and in no stance of its own takes
/// the pistol's LOW READY over the running legs — the stance every man moving between doors already has, arms only — drawn in toward the belt as a runner
/// carries it: the low ready's spring targets go to these (STANCES' units; fwd / up from the pelvis, pitch in degrees, tuck 0‥1). First numbers, off the
/// capsule strips.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RunreadyK {
    #[k(0.24)] pub fwd: f64,
    #[k(-0.185)] pub up: f64,
    #[k(-50.0)] pub pitch: f64,
    #[k(0.6)] pub tuck: f64,
}

/// A RUN CARRIES ITS ARMS CLOSE (the maintainer, 11-01: "sprint looks dainty when viewed from the back, maybe should have arms closer in?"). The sprint
/// clip's arms read as a sprint from the side and not from behind, which is where the camera is whenever he runs up the screen: the elbows stand 10 to 20
/// cm out from the shoulders, the hands swing up to 35 cm wide of them and never come higher than 22 cm under the shoulder, and in the shoulder line's own
/// frame a hand travels only 24 cm fore and aft (the shoulders' twist does the rest). A small, wide paddle. The jog's arms are as wide. With `on`, runArms
/// puts each hand of the standing pose back in that frame, by the run's share of the pose (jog through sprint, so a sprint a heavy kit slows is not half
/// the clip's again): `keep` of its sideways travel about `at` m out from the shoulder joint, and the elbow swivelled toward `pole` ([out, up, forward]:
/// behind and below him, a little out) by `poleW`; and by the SPRINT's share, `pump` × its fore-aft travel about `mid` and up to `lift` m higher at the
/// front of the swing (`swing` m ahead of `mid` = the full lift). The hand follows its forearm, so a thing in a pumping fist comes in with it. It is the
/// run's own carriage, under every layer: whatever owns an arm (a stance, a carry, both hands on a gun) still writes over it, and under a long gun that
/// has both arms it is not solved at all.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RunarmsK {
    #[k(0.05)] pub at: f64,
    #[k(0.3)] pub keep: f64,
    #[k(0.04)] pub mid: f64,
    #[k(0.12)] pub swing: f64,
    #[k(1.8)] pub pump: f64,
    #[k(0.14)] pub lift: f64,
    #[k([0.4, -0.45, -0.8])] pub pole: V3,
    #[k(0.85)] pub pole_w: f64,
}

/// THE FAST CROUCH WALK'S LOOK (CharacterAnimator.dash; the rule and its pace are player.ts CROUCHRUN): at 2.5 m/s the crouch cycle as it stands takes 3.5 steps
///  a second on a 1.44 m stride — a wound-up toy. By `dash` the stride is this much longer and the cadence that much lower (the planted foot keeps the ground's
///  speed, as the deliberate retime's does), the pelvis a little lower to pay for the reach and the trunk further over its knees, the head brought level back
///  out of it. Eased in and out at `rate` (1/s). Nothing here runs at dash 0.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DashK {
    #[k(0.3)] pub stride: f64,
    #[k(0.03)] pub drop: f64,
    #[k(7.0)] pub lean: f64,
    #[k(7.0)] pub rate: f64,
}

/// HE WATCHES A MAN, AND GIVES HIM ROOM AS HE PASSES (watchLayer; game/samlook.ts feeds `watchAt` / `slipAt`. The looks only: nothing reads them back).
///  THE LOOK: the head goes to the point — the chest a little, the neck, the head (`share`) — as far off the chest's line as `yaw`° and `up` / `down`° of
///  pitch; the angle follows the point at `rate` 1/s (a man swapped for another is a turn of the head, not a cut) and the whole comes and goes at `wRate`.
///  THE SLIP: crouched beside a man, by how near he is (`slipK`): the trunk tips `lean`° away from him with the head brought level back out of it, the pelvis
///  comes `drop` m lower and `away` m off his side over feet left where they were, and the arm on his side goes out low toward him, the hand open (`armOut` m
///  out from the shoulder, `armDown` m under it, at `armW` of the arm; none of it for a man dead ahead or dead behind — `armSide`, his bearing's sideways
///  share where it begins and is whole). A hand with something in it is left alone. Eased at `slipRate`. BETWEEN TWO MEN (`slipLean` under 1: they pull on
///  him from both sides) the lean, the step aside and the arm go by that share, and he sinks `squeeze` of the drop further: he gets small, and tips into nobody.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WatchK {
    #[k(72.0)] pub yaw: f64,
    #[k(28.0)] pub down: f64,
    #[k(22.0)] pub up: f64,
    #[k([0.14, 0.34, 0.52])] pub share: [f64; 3],
    #[k(6.0)] pub rate: f64,
    #[k(5.0)] pub w_rate: f64,
    #[k(9.0)] pub lean: f64,
    #[k(0.035)] pub drop: f64,
    #[k(0.03)] pub away: f64,
    #[k(0.32)] pub arm_out: f64,
    #[k(0.42)] pub arm_down: f64,
    #[k(0.7)] pub arm_w: f64,
    #[k([0.2, 0.55])] pub arm_side: [f64; 2],
    #[k(5.0)] pub slip_rate: f64,
    #[k(0.6)] pub squeeze: f64,
}

/// THE LOUD DEMO'S MOTION PASS (MOVES) — what a firefight shows that the stealth game never had to: a rifle that RECOILS (the gun driven back into the shoulder, the
///  muzzle climbing, the trunk rocked — controlled by a trained man, climbing away from an untrained one), the bang TOSSED underhand off the support hand with the
///  rifle still in the other (the teams' "throw" was a pointing chop), a stun charge that leaves a man bent away from it with his forearm over his eyes for as long
///  as he is blind (it was one twitch, then a statue), a FLINCH (a duck: head down and away, knees soft) that is not the same clip as taking a HIT (the torso knocked
///  along the round), a walk that tells the two sides apart (the team sunk onto bent knees and leaning into the gun, gliding; the defenders upright, shoulders
///  swinging, elbow out), falls that do not all go over backwards, and the stack's shoulder squeeze. Every part has its own switch and `on` is the master:
///  **on = 0 is the animator exactly as it was** (every new branch below is behind `mvOn`, the old statements untouched) — the headless harness stands up with it
///  off, so every frozen digest holds; the page has it on, `?nomoves` puts it off. Poses feed the game (the muzzle rides the hand, hit columns
///  ride the hips and chest), so ON is a different simulation in the small.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MovesK {
    #[k(1.0)] pub on: f64,
    #[k(1.0)] pub recoil: f64,
    #[k(1.0)] pub toss: f64,
    #[k(1.0)] pub dazzle: f64,
    #[k(1.0)] pub flinch: f64,
    #[k(1.0)] pub gait: f64,
    #[k(1.0)] pub fall: f64,
    #[k(1.0)] pub squeeze: f64,
    /// THE SECOND PASS — looks laid on states the game already has. `reload`, `bound` and `clutch` move ARMS ONLY that
    ///  nothing in the fight reads (a rifleman's support arm; an unarmed civilian's two): the trunk, the head, the trigger hand and with them the muzzle, the bore, the
    ///  eyes and the hit columns are bit for bit what they were, so these are on with the pass. `entry` lowers a teammate's muzzle past a friend's back — the muzzle is
    ///  the game's own (rounds leave it, the line-of-fire tests start at it), so it is a different simulation in the small: OFF — tallied over 144 seeds (CLAUDE.md, the MOVES second pass): no mission is measurably better for it, The block leans worse, and the bore still ends on the friend.
    ///  `entryEyes` (a part of `entry`, nothing without it): the dipped man still LOOKS where he is going — guards.ts's eyes-on-the-sights test reads his gun line
    ///  as if it were not dipped (`sightsLift`). The weapon light stays down with the muzzle, as a real one does.
    #[k(1.0)] pub reload: f64,
    #[k(1.0)] pub bound: f64,
    #[k(1.0)] pub clutch: f64,
    #[k(0.0)] pub entry: f64,
    #[k(0.0)] pub entry_eyes: f64,
    /// the clutch's FOLD: a man knocked about and still up stays doubled over the hurt, swaying, for as long as the game roots him — the TRUNK and head, so not
    ///  arms-only (his hit columns and a lowered gun move with it; he fires nothing while rooted). NOT on with the pass: 0 = off (the trunk of a staggered man is the base's), 1 = on by itself (`?fold`), -1 = never (`?fold=0`).
    #[k(0.0)] pub fold: f64,
    /// THE SNAP SHOT (a pistol fired from the relaxed carry; the same trade when the aim is raised or let go by hand). The carry and the two-hand aim are two
    ///  layers eased on two clocks (9 and 14 a second), so while they traded neither had the arm and the base clip's HANGING arm came through from under both: a
    ///  ninth of it on the fire frame (the wrist 10 cm in that one frame) and a third of it on the way back to the carry (16 cm in a frame, the hand dipping 20 cm
    ///  and more toward the thigh and up again). On: the layers TRADE — the carry stays whole under the aim until the aim has covered it, the aim stays until the
    ///  carry is back under it — and the aim comes in from rest over its first frames instead of a fifth of the way at once (MV.snap). And a shot along a PITCHED
    ///  aim keeps its pitch: the shot clip is keyed level, so blended in whole it lifted hands, trunk and head to the level pose for the length of every such shot
    ///  (aimed at his feet: the wrist 47 cm up in three frames, down again after) — its recoil alone is laid on the pitched pose instead; a level shot is the clip
    ///  to the bit. What the round is does not move: it leaves on its own frame from where the muzzle stood before the pose did (combat.ts fireWeapon reads it
    ///  ahead of the animator) toward its target, and the drawn bore is the aim's own line (Character.gunDir = aimDir()) — fired from rest it is the same round to
    ///  the bit, with the aim up the same to a micron (moves-snap holds both); one fired while the layers are still trading leaves from the hand's new path, a
    ///  few cm off the old.
    ///  Like the rest of MOVES it is a different simulation in the small: the pistol man's hands, chest and head (a guard's eyes ride his head, a torch its hand)
    ///  are where the new trade has them. Pistol in the hands only (no long gun shown, no share of one mid-swap: a rifleman's pitched shot still takes the level
    ///  clip's trunk). `?snap=0` off.
    #[k(1.0)] pub snap: f64,
    /// THE CROUCH WALK CARRIED LEVEL (the maintainer, twice: "the crouch walk animation looks like waddling"). Measured off Crouch_Fwd_Loop as the game drives it
    ///  (side to side over a cycle, against the standing walk): the feet come down 33 cm apart on 17.5 cm of hip (the walk: 18), the knees 32–38 cm apart, the
    ///  pelvis rolls 15° peak to peak (5°) and the whole trunk tips with it, 16° (6°) — so the head rides 17–18 cm from side to side (6 cm), once a stride, and at
    ///  the player's crouch pace that is nearly twice a second. A wide base with the trunk rocked over each foot in turn is what a waddle IS. On: the cycle's own
    ///  pelvis roll and the trunk's side lean are taken most of the way back out (MV.steady: the weight goes over the planted foot by the hips' few cm of travel,
    ///  as in the walk, not by tipping the body), the feet come down nearer his middle and the knees track over them. Fore and aft nothing moves: stride, cadence,
    ///  the planted foot's ground speed, the pelvis's height and the deliberate layer's stride and surge are the cycle's own. By the crouch walk's share of the
    ///  pose only — a still crouch, the kneel and every upright gait are untouched. Anybody who crouch-walks (Sam, a sneaking partner). Like the rest of MOVES a
    ///  different simulation in the small: his head and hands are a few cm from where the rocking had them. `?steady=0` off. SETTING 2 (`?steady=2`; never
    ///  the default) also rides him HIGHER through the cycle — the pelvis up to `MV.steady.rise` off the kneel wherever both legs can pay it out, so the rear knee
    ///  no longer drops to a hand off the floor on every step. That one is NOT a look-only change: the eyes that look for him read his head and chest bones, and
    ///  a head 7 cm higher is 7 cm more of him over low cover.
    ///  AND THE SPRINT WITH THE RIFLE (the maintainer, after the crouch walk: "sprint animation still has some waddling") — see `steadyRun` below.
    #[k(1.0)] pub steady: f64,
    /// THE RIFLE SPRINT WITHOUT THE SIDE ROLL (needs `steady`; `?steady=0` puts it off with the rest — a key of its own ONLY so a recording from before it,
    ///  whose MOVES row has `steady` and not this, replays on the trunk it was recorded with: journal.ts; no URL flag, no Tab row). A figure that steadies its
    ///  long gun (longSteady: Sam, and every teammate carrying the issue rifle) has the run clip's shoulder twist taken back out down the spine so the shoulders
    ///  stay on the weapon (longBody, LONG_STEADY.runYaw). That was done about the model's upright — and a SPRINTER's trunk is pitched ~25° forward, so ±25° of
    ///  twist about ITS line less ±25° about the upright left the whole trunk rolling ±11° and the head 17 cm from side to side, three steps a second (hands
    ///  empty the same sprint moves the head 1.7 cm). On, by the SPRINT's share of the pose (0 at and under the jog's speed), the twist is taken out about the
    ///  trunk's own line and the neck is handed it back: head 4–5 cm, trunk lean 2°, legs to the bit and the gun where they were. Only a sprinter changes, and
    ///  with a rifle in both hands that is the player: a teammate tops out under the jog's speed and his run is the old hold to the bit.
    #[k(1.0)] pub steady_run: f64,
}

/// The motion pass's numbers. Pairs are [untrained, trained], read by CharacterAnimator.trained (0 … 1; the game sets it per side — game.ts). First-pass values
///  judged on pose strips (tools/qa/pose-strip.ts), not yet on a GPU.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct MvK {
    pub recoil: MvKRecoil,
    pub flinch: MvKFlinch,
    pub hit: MvKHit,
    pub dazzle: MvKDazzle,
    pub gait: MvKGait,
    /// MOVES.snap, the pistol's carry ⇄ aim trade: `in` = the share of its usual first step a rising aim takes from rest, back to the whole step by weight `ease`;
    ///  `cover` = the weight band of the layer coming over (or coming back under) across which the other one is let go; `under` = the rate (1/s) more a carry
    ///  comes back at while the aim still hides it
    pub snap: MvKSnap,
    /// MOVES.steady, the crouch walk carried level (steadyLayer) — shares TAKEN OUT of the cycle's own SWING about its mean, at any weight of the crouch walk in
    ///  the pose: `roll` of the pelvis's roll (the hip joints' line off level; the trunk comes with it, and where the pose keeps the cycle's own spine — hands
    ///  empty — the chest is handed the same turn back, so shoulders the cycle carried level stay level), `sway` of the hips' own side-to-side travel; `feet` = how
    ///  far apart the ankles come down, as a share of the clip's (0.7: 33 cm → 23); `knee` = the share of each knee's sideways set off its hip that is taken out
    ///  (the plane the leg bends in swung that far toward straight fore and aft), × `kneeLow` for a knee within a hand of the floor. The narrowed base and the
    ///  swung-in knees are for the plain crouch walk: they fade out as the layers after this one sink the pelvis by `deep` (m: from → to; the near-a-threat
    ///  sneak and the close-in creep — a rear knee swung in under a pelvis sunk that far went through the floor) and under a stance that re-plants the legs
    ///  (stanceBody sends the knees out over the toes: on a narrowed base they came 4 cm lower). The level pelvis stays in all of them — and, sunk, a low knee
    ///  is set `kneeOut` of its sideways set further OUT, which hands back the 7 mm the levelled pelvis alone took off it.
    ///  Setting 2 only: `rise` = how far the pelvis is carried up off the kneel (m), never by more than leaves each leg at `reach` of its full length with its
    ///  ankle where it is, eased in and out at `riseRate` (1/s)
    pub steady: MvKSteady,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MvKRecoil {
    /// m the gun comes back along the bore per round
    #[k([0.06, 0.04])] pub back: [f64; 2],
    /// s it takes to come forward again (1/e)
    #[k([0.09, 0.055])] pub back_tau: [f64; 2],
    /// ° of muzzle climb per round (kept small: the defenders' blind burst flies along the drawn bore, and from above the climb is the least of what shows) …
    #[k([0.9, 0.4])] pub climb: [f64; 2],
    /// … and how fast it is pulled back down: at 10 rounds a second an untrained burst walks up ~2°, a trained one sits under 1°
    #[k([0.2, 0.1])] pub climb_tau: [f64; 2],
    /// ° the muzzle is thrown sideways per round (alternating, mostly to the gun side)
    #[k([1.3, 0.35])] pub yaw: [f64; 2],
    /// ° the upper trunk is rocked back per round
    #[k([3.4, 1.5])] pub rock: [f64; 2],
    /// ° the gun shoulder is driven back per round (the upper spine's yaw)
    #[k([5.0, 2.6])] pub shoulder: [f64; 2],
    #[k([0.16, 0.085])] pub rock_tau: [f64; 2],
    /// ° the head is jarred per round
    #[k([2.2, 0.6])] pub head: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MvKFlinch {
    #[k([1.0, 0.38])] pub amp: [f64; 2],
    #[k([0.7, 0.42])] pub dur: [f64; 2],
    #[k(15.0)] pub pitch: f64,
    #[k(14.0)] pub head: f64,
    #[k(11.0)] pub away: f64,
    #[k(0.055)] pub drop: f64,
    #[k(0.03)] pub side: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MvKHit {
    #[k(0.6)] pub dur: f64,
    #[k(17.0)] pub pitch: f64,
    #[k(19.0)] pub yaw: f64,
    #[k(5.0)] pub roll: f64,
    #[k(0.045)] pub shove: f64,
    #[k(0.025)] pub drop: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MvKDazzle {
    #[k([1.0, 0.6])] pub amp: [f64; 2],
    #[k(21.0)] pub hunch: f64,
    #[k(24.0)] pub away: f64,
    #[k(5.0)] pub roll: f64,
    #[k(20.0)] pub head: f64,
    #[k(22.0)] pub head_away: f64,
    #[k(0.065)] pub drop: f64,
    #[k(0.04)] pub back: f64,
    #[k(0.022)] pub sway_x: f64,
    #[k(0.016)] pub sway_z: f64,
    #[k(11.0)] pub rate_in: f64,
    #[k(4.5)] pub rate_out: f64,
    /// the hand over the eyes, from the eye point: m BELOW it and m out in front — under a hat's brim or a helmet's rim, which sit at the brow (level with the eyes and 3 cm nearer it came up through them on the cast's meshes); how far the fingers point up across the face; how far the palm is rolled down (the forearm comes up from below, so a palm square to the face bent the wrist ~60° sideways: rolled, most of that is flexion); and where the elbow hangs (low: the forearm comes up from under a wide brim, not in through its side)
    #[k(0.038)] pub hand_drop: f64,
    #[k(0.105)] pub hand_out: f64,
    #[k(0.05)] pub hand_tilt: f64,
    #[k(0.6)] pub hand_roll: f64,
    #[k([0.45, -1.0, 0.35])] pub hand_pole: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MvKGait {
    /// m the trained pelvis sinks moving / standing on the gun
    #[k(0.035)] pub drop: f64,
    #[k(0.025)] pub aim_drop: f64,
    /// share of the walk's pelvis bob taken out for a trained man
    #[k(0.65)] pub bob: f64,
    /// ° the trained trunk leans into it moving / shouldered (over the carriage's own 4°)
    #[k(5.0)] pub lean: f64,
    #[k(5.0)] pub aim_lean: f64,
    /// the trained elbows' poles pulled in under the gun
    #[k(0.42)] pub elbow_in: f64,
    /// ° the untrained shoulders swing / rock with the stride
    #[k(5.5)] pub slouch_sway: f64,
    #[k(2.2)] pub slouch_roll: f64,
    /// ° the untrained man stands back off his gun shouldered, head up off the stock
    #[k(4.5)] pub back: f64,
    #[k(5.0)] pub chin: f64,
    /// the untrained firing elbow's pole lifted out sideways
    #[k(0.85)] pub wing: f64,
}

/// MOVES.snap, the pistol's carry ⇄ aim trade: `in` = the share of its usual first step a rising aim takes from rest, back to the whole step by weight `ease`;
///  `cover` = the weight band of the layer coming over (or coming back under) across which the other one is let go; `under` = the rate (1/s) more a carry
///  comes back at while the aim still hides it
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MvKSnap {
    #[k(0.5)] pub in_: f64,
    #[k(0.2)] pub ease: f64,
    #[k([0.5, 0.95])] pub cover: [f64; 2],
    #[k(30.0)] pub under: f64,
}

/// MOVES.steady, the crouch walk carried level (steadyLayer) — shares TAKEN OUT of the cycle's own SWING about its mean, at any weight of the crouch walk in
///  the pose: `roll` of the pelvis's roll (the hip joints' line off level; the trunk comes with it, and where the pose keeps the cycle's own spine — hands
///  empty — the chest is handed the same turn back, so shoulders the cycle carried level stay level), `sway` of the hips' own side-to-side travel; `feet` = how
///  far apart the ankles come down, as a share of the clip's (0.7: 33 cm → 23); `knee` = the share of each knee's sideways set off its hip that is taken out
///  (the plane the leg bends in swung that far toward straight fore and aft), × `kneeLow` for a knee within a hand of the floor. The narrowed base and the
///  swung-in knees are for the plain crouch walk: they fade out as the layers after this one sink the pelvis by `deep` (m: from → to; the near-a-threat
///  sneak and the close-in creep — a rear knee swung in under a pelvis sunk that far went through the floor) and under a stance that re-plants the legs
///  (stanceBody sends the knees out over the toes: on a narrowed base they came 4 cm lower). The level pelvis stays in all of them — and, sunk, a low knee
///  is set `kneeOut` of its sideways set further OUT, which hands back the 7 mm the levelled pelvis alone took off it.
///  Setting 2 only: `rise` = how far the pelvis is carried up off the kneel (m), never by more than leaves each leg at `reach` of its full length with its
///  ankle where it is, eased in and out at `riseRate` (1/s)
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct MvKSteady {
    #[k(0.7)] pub roll: f64,
    #[k(0.3)] pub sway: f64,
    #[k(0.7)] pub feet: f64,
    #[k(0.5)] pub knee: f64,
    #[k(0.5)] pub knee_low: f64,
    #[k(alloc::vec![0.03, 0.05])] pub deep: Vec<f64>,
    #[k(0.25)] pub knee_out: f64,
    #[k(0.07)] pub rise: f64,
    #[k(0.95)] pub reach: f64,
    #[k(8.0)] pub rise_rate: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct TurnK {
    #[k(0.5)] pub body_share: f64,
    #[k(0.6)] pub green: f64,
    #[k(0.6)] pub lead: f64,
    #[k(16.0)] pub lead_rate: f64,
    #[k(0.75)] pub jump: f64,
    #[k(11.0)] pub aim: f64,
    #[k(150.0)] pub aim_acc: f64,
    #[k(7.0)] pub body: f64,
    #[k(60.0)] pub body_acc: f64,
    #[k(1.0)] pub on: f64,
}

/// A MAN WHO HAS RUN BREATHES HARD, AND IT SHOWS (the maintainer, 10-19: "characters should breathe more forcefully when sprinting (e.g. on snow levels)").
///  The game had two breaths and no third: the chest's (the life layer below) and, on a snow level, the steam off the mouth. There is no breath
///  SOUND and none is made here. With `on`, both of the two are stronger with effort — a sprint, and the while after it — and neither is touched at rest:
///    THE CHEST (lifeLayer): blown he heaves `chestAmp` × the resting turn at `chestRate` Hz (it was LIFE.breathAmpRun 2.6 at breathRateRun 0.62), is blown in
///      `chestUp` s of a run past 3.2 m/s (it was 4) and has his wind back in `chestDown` s (it was 8). Running, the gait still carries the chest as it did.
///    THE STEAM: blown he breathes every `every` s (it was SNOWBREATH.everyHard 1.3), each breath `puffs` records of `count`
///      puffs (3 of 5) thrown at `jet` × the resting speed (1.3) and `size` × its size (hardK 1.5; its opacity is full from half effort either way), leaving with `carry` of his own velocity (0.6);
///      blown in `up` s (effortUp 4), his wind back in `down` s (effortDown 12). Each is the resting number at effort 0 and this one at 1, straight between.
///  Effort is what it was: his ground speed and nothing else (no stamina exists and none is made); a teammate never sprints: at his fastest walk (2.2 m/s, a little over where effort begins) his steam is a seventh of the way to blown, and his chest's is the resting one (it wants 3.2 m/s).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HardbreathK {
    #[k(4.2)] pub chest_amp: f64,
    #[k(0.9)] pub chest_rate: f64,
    #[k(2.5)] pub chest_up: f64,
    #[k(12.0)] pub chest_down: f64,
}

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct KitK {
    #[k({ let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, 0.3); t.set(crate::character::KitItem::Rifle, 0.54); t.set(crate::character::KitItem::Can, 0.26); t.set(crate::character::KitItem::Knife, 0.34); t })] pub stow: JsMap<crate::character::KitItem, f64>,
    #[k({ let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, 0.17); t.set(crate::character::KitItem::Rifle, 0.28); t.set(crate::character::KitItem::Can, 0.16); t.set(crate::character::KitItem::Knife, 0.2); t })] pub reach: JsMap<crate::character::KitItem, f64>,
    #[k({ let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, 0.26); t.set(crate::character::KitItem::Rifle, 0.43); t.set(crate::character::KitItem::Can, 0.2); t.set(crate::character::KitItem::Knife, 0.2); t })] pub draw: JsMap<crate::character::KitItem, f64>,
    #[k(0.22)] pub away: f64,
    #[k(0.8)] pub swap_k: f64,
    #[k(0.28)] pub far: f64,
    #[k(0.85)] pub apex_at: f64,
    #[k(0.55)] pub sling_at: f64,
    #[k(0.35)] pub join: f64,
    #[k(0.3)] pub let_go: f64,
    pub apex: KitKApex,
    /// WHAT AN ARM CAN DO (the maintainer, 10-10, of rifle → grenade: "nonphysical for some joints + really fast"; tools/qa/kitanim-joints.ts measures every change
    ///  against these): the errand's wrist never comes nearer its own shoulder than leaves the elbow `elbowMin`° open — a path that cuts past the shoulder goes
    ///  ROUND it —, the hand is never bent more than `wristMax`° off its forearm (eased in from `wristKnee`°; what it carries turns with it), a rifle is laid along the hand
    ///  over the first `rollIn` of the hand's trip to the shoulder (0.35 for the rest) and comes back onto the aim over the last `rollIn` of the trip down, the empty hand going
    ///  between shoulder and hip bows out by `crossFar`, and while a rifle is on its way the trunk under the arms re-stands at `trunk` 1/s (the aim's own 14 squared
    ///  the chest 10° on the key's frame) with the long gun's carriage eased in, not taken at once.
    #[k(44.0)] pub elbow_min: f64,
    #[k(46.0)] pub wrist_knee: f64,
    #[k(66.0)] pub wrist_max: f64,
    #[k(0.5)] pub roll_in: f64,
    #[k([-0.03, 0.34, 0.22])] pub over: V3,
    #[k([-0.14, 0.22, 0.14])] pub up: V3,
    pub top: KitKTop,
    #[k(6.0)] pub trunk: f64,
    pub belt: KitKBelt,
    #[k([-0.7, 0.15, -1.0])] pub holster_pole: V3,
    #[k([0.03, -0.36, 0.2])] pub rest: V3,
    #[k({ let mut t = JsMap::new(); t.set(crate::character::KitItem::Pistol, [-0.05, 0.0, 0.05]); t.set(crate::character::KitItem::Rifle, [-0.1, 0.2, 0.0]); t.set(crate::character::KitItem::Can, [-0.03, 0.0, 0.05]); t.set(crate::character::KitItem::Knife, [0.0, 0.0, 0.05]); t })] pub arc: JsMap<crate::character::KitItem, V3>,
    #[k([-0.16, 0.0, 0.1])] pub cross: V3,
    /// THE RIFLE ON A SLING IN FRONT (a loud kit; anim.kitFront — rigProps SLING_FRONT is where it hangs, and the hand's place there is read off it): let down
    ///  out of the shoulder and let go of, taken up again by the grip. Its own, shorter, times (no trip over the shoulder, no `far`); `handAt` / `slingAt` as
    ///  apexAt / slingAt above; `arc` = the bow of the way down (ahead of him: clear of the chest rig); `pole` = the elbow, down and out by his right side.
    pub front: KitKFront,
    /// NIGHT VISION flipped down / up (nvFlip; the maintainer, 10-10: "should have animations for putting on and taking off night vision. if hands are busy it's a
    ///  nod of the head for flicking on and off. otherwise hand does it."). hand: the LEFT hand goes up to the goggles (reach), carries them down / up (the flip
    ///  window, s from the start) and comes away (out); `up` / `down` = the wrist's place with the goggles up / down in the HEAD's frame [his left, up, forward]
    ///  off the head joint, `fingers` / `back` = the fingers' and the back of the hand's directions in that frame, `pole` the elbow's in the body's, a shrug of the
    ///  shoulder °, the fingers' curl; rate = the most the arm's share may change a second, away = the same once something else has taken the arm mid-flip (it
    ///  has all of it inside 1 / away s; the reach is as long as it is because a hand that goes from the hip to the brow any quicker moves faster than a hand does). nod: the
    ///  head alone — on = [a small tip back first, the dip that drops them] °, off = [a small dip, the toss that throws them up] (+ = chin down), pre / peak = when
    ///  each is at its most, s; the goggles move in `flip`. FIRST-PASS numbers, never seen in the renderer.
    pub nv: KitKNv,
    #[k(alloc::vec![0.62, 0.68, 0.72, 0.76])] pub fist: Vec<f64>,
    #[k(alloc::vec![0.26, 0.3, 0.34, 0.38])] pub open: Vec<f64>,
    /// A PISTOL's magazine change, the support hand's (pistolReloadArm; marks in the gun's frame off the trigger wrist — x his left, y up out of the slide, z the
    ///  bore — and the support hand's WRIST): `well` = the palm under the butt, `strip` = clear of it with the magazine, `slide` → `pull` = over the back of the slide
    ///  and run; `pouch` = the fresh ones (left front of the belt), `dump` = where a magazine with rounds left in it is put away (left hip, behind; both guns'), `via` = the way there, out ROUND his side and not through his belly — all off the
    ///  pelvis, trunk frame.
    pub mag: KitKMag,
    /// CLOSER THAN EVER BY DISTANCE (closeK 0 = just inside the mode … 1 = in reach; the maintainer, 10-10: "sam's hand looks pretty static. and it should be
    ///  reaching for his knife … sam's state vary continuously with his distance to the enemy"): `far` = the share of the carriage (the drop, the bow, the rounded
    ///  shoulders) he has at the mode's edge — all of it in reach; the right hand goes from CREEP.R's spot to hover `hover` m off the hilt (bowed out by `bow`,
    ///  trunk frame, on the way); THE LEFT HAND STAYS WITH THE CROUCH (the maintainer, 10-10: "sam's left hand should also mostly be for the crouching animation and then only
    ///  reach out for the enemy when quite close (like when his hand is fully on the knife)"): `left` = the share of the carriage's raised lead hand the left arm takes
    ///  on the way in (the rest is the clip's own arm), `lead` = how far that hand comes up and out with the distance, and in reach its move for the man (readyLayer)
    ///  starts only once the right hand's clock is `leftAt` of the way onto the hilt, over `leftIn` s (back over `leftOut`); `rate` 1/s = how fast the shown state follows the distance; and the hands are alive:
    ///  fingers working at `flex` Hz by `flexAmp` of curl, the wrists drifting `drift` m, the fist on the hilt re-seating its grip every `regrip` s.
    pub close: KitKClose,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitKApex {
    #[k([-0.03, 0.1, -0.19])] pub at: V3,
    #[k([0.35, -0.75, -0.55])] pub fing: V3,
    #[k([0.0, -0.2, -1.0])] pub back: V3,
    #[k([-1.0, 0.25, 0.0])] pub pole: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitKTop {
    #[k([0.0, 1.0, 0.25])] pub fing: V3,
    #[k([-0.8, 0.0, 0.6])] pub back: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitKBelt {
    #[k([-0.15, 0.04, 0.13])] pub at: V3,
    #[k([0.25, -0.9, 0.2])] pub fing: V3,
    #[k([-0.5, 0.0, 0.85])] pub back: V3,
    #[k([-1.0, -0.1, -0.5])] pub pole: V3,
}

/// THE RIFLE ON A SLING IN FRONT (a loud kit; anim.kitFront — rigProps SLING_FRONT is where it hangs, and the hand's place there is read off it): let down
///  out of the shoulder and let go of, taken up again by the grip. Its own, shorter, times (no trip over the shoulder, no `far`); `handAt` / `slingAt` as
///  apexAt / slingAt above; `arc` = the bow of the way down (ahead of him: clear of the chest rig); `pole` = the elbow, down and out by his right side.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitKFront {
    #[k(0.36)] pub stow: f64,
    #[k(0.2)] pub reach: f64,
    #[k(0.32)] pub draw: f64,
    #[k(0.8)] pub hand_at: f64,
    #[k(0.45)] pub sling_at: f64,
    #[k([0.0, 0.02, 0.07])] pub arc: V3,
    #[k([-0.75, -0.55, -0.4])] pub pole: V3,
}

/// NIGHT VISION flipped down / up (nvFlip; the maintainer, 10-10: "should have animations for putting on and taking off night vision. if hands are busy it's a
///  nod of the head for flicking on and off. otherwise hand does it."). hand: the LEFT hand goes up to the goggles (reach), carries them down / up (the flip
///  window, s from the start) and comes away (out); `up` / `down` = the wrist's place with the goggles up / down in the HEAD's frame [his left, up, forward]
///  off the head joint, `fingers` / `back` = the fingers' and the back of the hand's directions in that frame, `pole` the elbow's in the body's, a shrug of the
///  shoulder °, the fingers' curl; rate = the most the arm's share may change a second, away = the same once something else has taken the arm mid-flip (it
///  has all of it inside 1 / away s; the reach is as long as it is because a hand that goes from the hip to the brow any quicker moves faster than a hand does). nod: the
///  head alone — on = [a small tip back first, the dip that drops them] °, off = [a small dip, the toss that throws them up] (+ = chin down), pre / peak = when
///  each is at its most, s; the goggles move in `flip`. FIRST-PASS numbers, never seen in the renderer.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitKNv {
    pub hand: KitKNvHand,
    pub nod: KitKNvNod,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitKNvHand {
    #[k(0.92)] pub dur: f64,
    #[k(0.38)] pub reach: f64,
    #[k(0.36)] pub out: f64,
    #[k([0.38, 0.52])] pub flip: [f64; 2],
    #[k([0.084, 0.178, 0.1])] pub up: V3,
    #[k([0.086, 0.08, 0.12])] pub down: V3,
    #[k([-0.75, 0.6, 0.1])] pub fingers: V3,
    #[k([0.55, 0.1, 0.83])] pub back: V3,
    #[k([0.7, -0.6, 0.35])] pub pole: V3,
    #[k(8.0)] pub shrug: f64,
    #[k([0.45, 0.5, 0.55, 0.6])] pub curl: [f64; 4],
    #[k(0.35)] pub thumb: f64,
    #[k(4.2)] pub rate: f64,
    #[k(2.6)] pub away: f64,
    #[k(6.0)] pub clear: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitKNvNod {
    #[k(0.36)] pub dur: f64,
    #[k(0.07)] pub pre: f64,
    #[k(0.18)] pub peak: f64,
    #[k([0.12, 0.22])] pub flip: [f64; 2],
    #[k([-5.0, 19.0])] pub on: [f64; 2],
    #[k([7.0, -15.0])] pub off: [f64; 2],
    #[k(0.12)] pub carry: f64,
}

/// A PISTOL's magazine change, the support hand's (pistolReloadArm; marks in the gun's frame off the trigger wrist — x his left, y up out of the slide, z the
///  bore — and the support hand's WRIST): `well` = the palm under the butt, `strip` = clear of it with the magazine, `slide` → `pull` = over the back of the slide
///  and run; `pouch` = the fresh ones (left front of the belt), `dump` = where a magazine with rounds left in it is put away (left hip, behind; both guns'), `via` = the way there, out ROUND his side and not through his belly — all off the
///  pelvis, trunk frame.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitKMag {
    #[k([0.085, -0.125, -0.005])] pub well: V3,
    #[k([0.09, -0.21, -0.02])] pub strip: V3,
    #[k([0.05, 0.07, -0.02])] pub slide: V3,
    #[k([0.05, 0.07, -0.085])] pub pull: V3,
    #[k([0.17, 0.03, 0.12])] pub pouch: V3,
    #[k([0.21, 0.0, -0.13])] pub dump: V3,
    #[k([0.31, 0.14, 0.1])] pub via: V3,
    #[k([0.75, -0.55, -0.3])] pub pole: V3,
    #[k(0.7)] pub pole_w: f64,
    #[k([-0.75, 0.55, 0.3])] pub wf: V3,
    #[k([0.3, -0.75, 0.55])] pub wb: V3,
    #[k([-1.0, -0.15, 0.1])] pub sf: V3,
    #[k([0.0, 1.0, 0.0])] pub sb: V3,
}

/// CLOSER THAN EVER BY DISTANCE (closeK 0 = just inside the mode … 1 = in reach; the maintainer, 10-10: "sam's hand looks pretty static. and it should be
///  reaching for his knife … sam's state vary continuously with his distance to the enemy"): `far` = the share of the carriage (the drop, the bow, the rounded
///  shoulders) he has at the mode's edge — all of it in reach; the right hand goes from CREEP.R's spot to hover `hover` m off the hilt (bowed out by `bow`,
///  trunk frame, on the way); THE LEFT HAND STAYS WITH THE CROUCH (the maintainer, 10-10: "sam's left hand should also mostly be for the crouching animation and then only
///  reach out for the enemy when quite close (like when his hand is fully on the knife)"): `left` = the share of the carriage's raised lead hand the left arm takes
///  on the way in (the rest is the clip's own arm), `lead` = how far that hand comes up and out with the distance, and in reach its move for the man (readyLayer)
///  starts only once the right hand's clock is `leftAt` of the way onto the hilt, over `leftIn` s (back over `leftOut`); `rate` 1/s = how fast the shown state follows the distance; and the hands are alive:
///  fingers working at `flex` Hz by `flexAmp` of curl, the wrists drifting `drift` m, the fist on the hilt re-seating its grip every `regrip` s.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitKClose {
    #[k(0.4)] pub far: f64,
    #[k(0.045)] pub hover: f64,
    #[k([-0.02, 0.01, 0.035])] pub bow: V3,
    #[k([0.0, 0.0, 0.0])] pub lead: V3,
    #[k(0.3)] pub left: f64,
    #[k(0.85)] pub left_at: f64,
    #[k(0.24)] pub left_in: f64,
    #[k(0.2)] pub left_out: f64,
    #[k(7.0)] pub rate: f64,
    #[k(0.55)] pub flex: f64,
    #[k(0.1)] pub flex_amp: f64,
    #[k(0.008)] pub drift: f64,
    #[k(1.7)] pub regrip: f64,
    #[k(0.5)] pub curl: f64,
}

/// A THROW READS ITS DISTANCE (the maintainer, 10-16, over a can thrown at a wall two steps off with the full wind-up: "we should have different animations
///  based on how far you're throwing something"). throwItem's `dist` = metres over the floor from the thrower to where the arc ends; with `on` (off in the
///  module and on the frozen stand, TPS_THROWBY=1; on in the page, ?nothrowby = one swing for every throw) it picks the swing. Three reads on ONE CLOCK — the
///  can leaves every one of them at THROW_RELEASE, and where it starts and how it flies are the thrower's, not the swing's:
///   · under `toss` m, and thrown under `tossFast` m/s over the floor (a hard throw a wall stops two steps off is no toss: it is the shortest overhand) — an
///     UNDERHAND TOSS (THR_LOW): the arm swings back past the hip and through, the wrist never above the shoulder; the backswing is `tossMin` of its size at
///     arm's length, all of it at `toss` m (drawn in toward `tossPivot`, off the shoulder: the hanging hand). From a hand up in front of him (a gun) there is
///     no backswing to whip into: past what `tossIn` m/s reaches by the top, the hand drops on one arc to the swing's low point and scoops from there. The
///     other arm is left to the pose underneath, or, up on a gun's support grip, is let down to hang (TOSS_HANG) for the swing;
///   · from `toss` to `full` m — the overhand SHORT-ARMED: the body's turn, lean and weight shift from `turnMin` of their size up to all of it, the hand's path
///     drawn in toward `pivot` (out beside the shoulder: the hand cocks no further back than the ear) from `armMin` of its size up to all of it — but back at
///     its authored size for `through` s either side of the release, so it lets go where the whole throw does and the path has no corner — the balance arm's
///     path and turn toward the pose underneath from `offMin`, the crouched sit-up from `sit`, by a smoothstep of the distance;
///   · `full` m and over — the overhand as authored. No distance told, or the switch off, is that too, bit for bit.
///  With the switch on the fist also drops its can the frame the thrown one is in the air (combat.ts: `throwing` runs three frames past the release).
///  The body the picture shows is the body the guards see and shoot at (one model): a toss keeps a crouched head down where the overhand sits him up.
///  UNTUNED by eye (nobody has seen these on a GPU).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ThrowByK {
    #[k(3.0)] pub toss: f64,
    #[k(7.3)] pub toss_fast: f64,
    #[k(4.0)] pub toss_in: f64,
    #[k(0.6)] pub toss_min: f64,
    #[k(0.07)] pub through: f64,
    #[k([-0.07, -0.46, 0.08])] pub toss_pivot: V3,
    #[k(10.0)] pub full: f64,
    #[k(0.4)] pub turn_min: f64,
    #[k(0.5)] pub arm_min: f64,
    #[k(0.45)] pub off_min: f64,
    #[k(0.6)] pub sit: f64,
    #[k([-0.2, 0.06, 0.18])] pub pivot: V3,
}
