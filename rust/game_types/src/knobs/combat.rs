//! The numbers of combat.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// THE FLASHBANG IN A ROOM FIGHT (`fight_live`; anywhere else a stun can dazzles and goes off by `effects::bang`'s numbers alone).
///  What it does to a man is three things, each falling off from the can to the edge of `dazzle_radius`:
///    BLIND   `secs` seconds [at the can, at the edge]: sees nobody, fires at nobody he can name — anybody the fireball has a line to: from the can itself, or from
///            `lift` metres over it when the can can see that point (a can on the floor behind a desk still fills the room over the desk);
///    STUNNED `stagger` seconds of it doubled over [at the can, at the edge]: rooted, gun down, NO round — so the blind burst a defender owes the door he was watching
///             is mostly or wholly lost: its turns come due while he is folded;
///    THE BANG WITHOUT THE FLASH: a man the fireball has no line to (round the end of a cabinet, behind a partition) but who shares its air — within `concuss_m` by
///            the sound's own path (over furniture, round corners, never through a wall or a shut leaf) — takes `concuss_k` of both.
///  ITS OWN SIDE knows it is coming ('bang out': heads turned, eyes shut): the thrower's team is touched only inside `own_m`, for `own_secs`, and never folded (Sam himself, with a line to it, takes it whole where `samstun.own_seen` is up),
///  so Sam's own can does not blind the stack he threw it past. And his can is a COOKED one: it goes off `fuse` seconds after it lands.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FlashK {
    #[k([7.0, 4.0])] pub secs: [f64; 2],
    #[k([2.5, 1.0])] pub stagger: [f64; 2],
    #[k(1.2)] pub lift: f64,
    #[k(6.0)] pub concuss_m: f64,
    #[k(0.5)] pub concuss_k: f64,
    #[k(2.5)] pub own_m: f64,
    #[k(1.5)] pub own_secs: f64,
    #[k(0.7)] pub fuse: f64,
}
