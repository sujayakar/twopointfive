//! The numbers of combat.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// THE FLASHBANG IN A ROOM FIGHT.
///  What it does to a man is three things, each falling off from the can to the edge of bang.dazzleRadius:
///    BLIND   `secs` seconds (Guard.dazzledUntil: sees nobody, fires at nobody he can name) — to anybody the fireball has a line to: from the can itself, or from
///            `lift` metres over it when the can can see that point (a can on the floor behind a desk still fills the room over the desk);
///    STUNNED `stagger` seconds of it doubled over (Guard.staggerUntil: rooted, gun down, NO round — so the blind burst a defender owes the door he was watching
/// is mostly or wholly lost: its turns come due while he is folded) — it was a dazzle only, and the room answered every bang with
///            twelve rounds a man back through the doorway;
///    THE BANG WITHOUT THE FLASH: a man the fireball has no line to (round the end of a cabinet, behind a partition) but who shares its air — within `concussM` by
///            the sound's own path — takes `concussK` of both.
///  ITS OWN SIDE knows it is coming ('bang out': heads turned, eyes shut): the thrower's team is touched only inside `ownM`, for `ownSecs`, and never folded —
///  Sam's own can no longer blinds the stack he threw it past. And his can is a COOKED one: it goes off `fuse` seconds after it lands (the night game's sits
///  bang.fuse 1.5 s on the floor of a room whose door is already open). `on` false = the dazzle, and the fuse, as the night game has them.
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
