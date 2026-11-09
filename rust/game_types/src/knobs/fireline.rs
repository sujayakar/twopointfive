//! The numbers of fireline.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FirelineK {
    /// s: the player is firing while a round left his gun inside this long — and through a reload begun while he was, and `reloadS` after it (a magazine
    ///  changed in the middle of a string is the string)
    #[k(1.0)] pub fire_s: f64,
    #[k(0.7)] pub reload_s: f64,
    /// slow aimed fire is fire too: two rounds inside `paceS` of each other, and the lane outlasts the last of them by `paceK` × the gap between them (at most
    ///  `paceS`) — a round every second and a bit is one string, not a string of chances to cross
    #[k(2.5)] pub pace_s: f64,
    #[k(1.5)] pub pace_k: f64,
    /// s: a lane that comes back this soon after it lapsed on a waiting man is the same wait (no second word; the ceiling's clock kept)
    #[k(2.5)] pub lapse_s: f64,
    /// the widest a round of his is thrown: the rifle on the move, a string's growth and the aim model's cone on top
    #[k(0.06)] pub spread: f64,
    /// m: room either side of the round's lane on top of a body's (FRIENDLYFIRE.bodyM) — he stops clear of it, not on its edge
    #[k(0.25)] pub clear_m: f64,
    /// m: he stops when his way enters the lane inside this far of walking (`keepM` more once he is waiting: no dithering on the edge)
    #[k(1.4)] pub short_m: f64,
    #[k(0.4)] pub keep_m: f64,
    /// m: …and, however shallow his way comes at it, before his next step is this near the lane's side (`lookM`: as far up his way as the lane is looked for)
    #[k(1.0)] pub gap_m: f64,
    #[k(6.0)] pub look_m: f64,
    /// he says so once, `cueS` into the wait (a man who stops for no reason he gives reads as stuck; a stop that short is not worth a word)
    #[k(0.35)] pub cue_s: f64,
    /// s: the longest he waits before he looks for a way round, and how often he looks again
    #[k(4.0)] pub max_s: f64,
    #[k(2.0)] pub again_s: f64,
    /// m: a way round may be this much longer than the way across; the floor this near the lane is dear to the search for one (over half a nav cell's diagonal: a corner cut between two cells clear of it stays out of the lane)
    #[k(10.0)] pub way_more_m: f64,
    #[k(0.4)] pub berth_m: f64,
    /// m: a way round is asked of a lane this much wider than the one a wait keeps off (a walker cuts his corners), and — once he is on it — of the round's own lane, `clearM` narrower (the gun sways)
    #[k(0.15)] pub edge_m: f64,
    /// m: the lane's length at most
    #[k(40.0)] pub reach_m: f64,
}
