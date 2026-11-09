//! The numbers of kitanim.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitanimK {
    /// where the rifle lives when it is not in his hands (the maintainer, 10-10: "for "loud builds" … unequipping rifle should be putting it down in a sling, not
    ///  over the back, right?"): 0 = by the mission — a squad's loud mission: let down on its sling in FRONT of him (quicker both ways: no trip over the shoulder);
    ///  a quiet mission, the plain night: across his BACK, out of the way of his hands — the very test the sneaking suit is worn by (cast.ts sneakSuit, less the
    ///  look's own switches) · 1 = always in front · 2 = always on his back. Part of `equip`: with that off (or the switch) it is the back, as the build had it.
    #[k(0.0)] pub sling: f64,
    /// the rifle hanging in front rests on a thigh that comes up under it (rigProps SLING_REST): how fast it is pushed up onto one / settles back down, 1/s (an
    ///  ease toward what the pose wants — a stride moves that a quarter turn in a few frames, and a hung rifle has weight); 0 = no ease, the pose's own answer
    #[k(18.0)] pub sling_up: f64,
    #[k(4.0)] pub sling_down: f64,
    /// s before a draw's end at which the gun may already fire (it is up; the support hand is still settling)
    #[k(0.05)] pub early: f64,
    /// A CLICK MADE WHILE A GUN IS STILL COMING UP IS KEPT: the left button pressed inside the last `clickKeep` s of a kit change's lock-out fires the round on the
    ///  frame the gun is up (kitClick — the wall stack's own rule for its rise). 0 = dropped, as the build had it: the click of a drawn pistol did nothing.
    #[k(0.35)] pub click_keep: f64,
    /// …and A CLICK MADE WHILE A CAN IS STILL COMING TO THE FIST IS KEPT for the whole fetch ('5, click' in one breath throws as the can arrives — it used to throw
    ///  nothing for up to two thirds of a second): the most such a click may wait, s. 0 = dropped, as it was.
    #[k(1.0)] pub throw_keep: f64,
    /// the most a lock-out may be once no errand is running (= player.ts RIFLE_EQUIP_SECS, the build's own rifle beat; the probes hold the two equal)
    #[k(0.45)] pub plain_lock: f64,
    /// the magazine changes, in seconds on the weapon's clock — written for the pistol's 1.6 s row and the rifle's 2.4 s row and scaled to whatever row the gun in
    ///  hand has. out = the old magazine leaves the gun · keep = it is in the dump pouch (tactical only) · in_ = the fresh one is seated · rack = slide / handle
    pub beats: KitanimKBeats,
    /// m to the close-in mode's man at which closeness is 1 (the knife's reach); it is 0 at the mode's own edge (close.ts CLOSE.modeM)
    #[k(1.1)] pub close_near: f64,
}

/// the magazine changes, in seconds on the weapon's clock — written for the pistol's 1.6 s row and the rifle's 2.4 s row and scaled to whatever row the gun in
///  hand has. out = the old magazine leaves the gun · keep = it is in the dump pouch (tactical only) · in_ = the fresh one is seated · rack = slide / handle
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitanimKBeats {
    pub pistol: KitanimKBeatsPistol,
    pub rifle: KitanimKBeatsRifle,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitanimKBeatsPistol {
    #[k(1.6)] pub ref_: f64,
    #[k(crate::kitanim::BeatRow { dur: 1.6, out: 0.25, in_: 0.95, rack: Some(1.3), keep: None })] pub empty: crate::kitanim::BeatRow,
    #[k(crate::kitanim::BeatRow { dur: 1.45, out: 0.25, in_: 0.95, rack: None, keep: None })] pub dry: crate::kitanim::BeatRow,
    #[k(crate::kitanim::BeatRow { dur: 2.0, out: 0.45, in_: 1.5, rack: None, keep: Some(0.95) })] pub tactical: crate::kitanim::BeatRow,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KitanimKBeatsRifle {
    #[k(2.4)] pub ref_: f64,
    #[k(crate::kitanim::BeatRow { dur: 2.4, out: 0.4, in_: 1.45, rack: Some(1.95), keep: None })] pub empty: crate::kitanim::BeatRow,
    #[k(crate::kitanim::BeatRow { dur: 1.95, out: 0.4, in_: 1.45, rack: None, keep: None })] pub dry: crate::kitanim::BeatRow,
    #[k(crate::kitanim::BeatRow { dur: 2.75, out: 0.55, in_: 2.1, rack: None, keep: Some(1.2) })] pub tactical: crate::kitanim::BeatRow,
}
