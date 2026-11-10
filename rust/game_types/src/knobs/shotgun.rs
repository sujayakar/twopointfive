//! The numbers of shotgun.
use crate::bytes::ToBytes;
use alloc::vec::Vec;
use game_derive::Knobs;

/// the door tools' side of it: `on` false = a stack never takes a lock with the gun (the boot, as before) · the stand-offs tried for the man who shoots the lock,
///  safest first — [metres past the latch edge along the frame line (negative: in front of the leaf), metres off it] · he does not stand within `clear` of another
///  man's mark while a spot further down the list is free (the second man with the gun does not shoot it from inside the lead on his jamb) · the lock he aims
///  at: this far in from the latch edge, this high · shells he may put into it before the stack gives up on the gun and boots it · seconds his muzzle may take
///  to come onto it.
///  GLASS IS NOT COVER, for him too (orders.ts behindGlass — the stack's own rule; nothing of this runs while rounds stop in glass): where a spot of the list
///  stands in a line of fire through a pane he takes the lock FROM HIS OWN SIDE of the frame instead of crossing the opening to stand in front of the sidelight
///  his stack was moved off — from his own mark when it is clear of the opening, the lock within `markReach` of it, its bore on the leaf and no mate within
///  `line` of the shot, else (his mark on the hinge side) from the first of `hinge` ([metres past the HINGE edge, metres off the wall]: every one clear of the
///  opening by his arrival slack) that is clean; the list again only when that side has nothing. From such a place the shell waits `ownBeat` s for his gun
///  to come up (the mark is where he already stands; a list spot's walk was that beat) and he is within `ownSlack` m of it, not the list's 0.4. `ownSide`: the same
///  whenever his mark is on the hinge side, glass or no glass (a WALL at the latch jamb files the stack there too — the office's server room door from the
///  manager's side); off = such a door is as it was, he crosses to the list's spot.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct ShotgunK {
    #[k(alloc::vec![[0.45, 1.0], [0.25, 1.1], [0.05, 1.2], [-0.25, 1.3], [-0.5, 1.3]])] pub spots: Vec<[f64; 2]>,
    #[k(0.55)] pub clear: f64,
    #[k(alloc::vec![[0.45, 1.0], [0.45, 1.2], [0.65, 1.2]])] pub hinge: Vec<[f64; 2]>,
    #[k(1.8)] pub mark_reach: f64,
    #[k(0.45)] pub line: f64,
    #[k(0.15)] pub own_beat: f64,
    #[k(0.25)] pub own_slack: f64,
    #[k(0.2)] pub lock_in: f64,
    #[k(1.0)] pub lock_h: f64,
    #[k(2.0)] pub shells: f64,
    #[k(2.5)] pub aim_cap: f64,
}
