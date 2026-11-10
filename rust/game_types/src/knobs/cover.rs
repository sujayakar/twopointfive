//! The numbers of cover.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// The knobs (__game.cover.knobs).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CoverK {
    #[k(1.0)] pub rear_read_in: f64,
    #[k(14.0)] pub rear_read_m: f64,
    #[k(35.0)] pub rear_read_deg: f64,
    #[k(1.0)] pub rear_read_secs: f64,
    #[k(1.3)] pub rear_read_sticky: f64,
    #[k(6.0)] pub rear_look: f64,
    /// the rear man takes the rear only inside this many metres of his mark: further out on the walk to the stack he keeps up with the file, gun forward —
    ///  or on a door standing open by his way — (a
    ///  large number = the rear is his the whole walk: he came the whole way backwards, metres behind his mate, and the pair reached the door apart). 5, not
    ///  the 3 it first shipped at: over 72 seeded runs the last three metres alone left 98 teammates of 192 standing where 5 m leaves 119, 8 m 121 and the
    ///  whole walk 119 — the men who come at a stack come while it is still forming. At 8 the stack forms a second and three quarters later than at 3, and
    ///  two pairs going on one word lost two men on a seed (pairs-go)
    #[k(5.0)] pub rear_walk_m: f64,
    /// an opening further than this from a man is not his; the deal is redone this often (game seconds)
    #[k(9.0)] pub reach: f64,
    #[k(0.25)] pub deal_secs: f64,
    /// a free run of nav cells (half a metre each) this wide or less between two blocked cells is a gap · a passage up to this many cells long is ONE opening,
    ///  longer has a mouth at each end · a blind passage this deep or deeper has a mouth worth a gun · a passage runs on past this many rows where it widens
    #[k(3.0)] pub gap_cells: f64,
    #[k(3.0)] pub through_rows: f64,
    #[k(3.0)] pub alcove_rows: f64,
    #[k(3.0)] pub bridge: f64,
    /// the squad's own men this near an opening are IN it (it is being walked through, not watched)
    #[k(0.9)] pub in_m: f64,
    /// one of the squad's own within this of the line from a man to an opening: it is not his to take (m; `laneKeep` for the one he already has — taken clear
    ///  of where orders.ts's own muzzle lane lets a gun back up, laneOut 0.7, and kept clear of where it brings one down, laneIn 0.45) · a man STOOD on his opening
    ///  keeps it this long with a mate crossing his line (s: his muzzle dips and comes back; he does not turn away and back)
    #[k(0.7)] pub lane: f64,
    #[k(0.5)] pub lane_keep: f64,
    #[k(0.6)] pub lane_secs: f64,
    /// Sam's gun within this of an opening he has a line to: his · what such an opening, and one a posted man has, still weighs (a spare man doubles up on it)
    #[k(35.0)] pub sam_deg: f64,
    #[k(0.2)] pub sam_k: f64,
    #[k(0.1)] pub held_k: f64,
    /// the weights: a shut door against an open way · the rear · how fast nearness tells (m)
    #[k(0.7)] pub shut_k: f64,
    #[k(0.8)] pub rear_k: f64,
    #[k(3.0)] pub near_m: f64,
    /// a man who had a living enemy in sight this recently is the contact's, not an opening's (s)
    #[k(4.0)] pub contact_secs: f64,
    /// the rear: this far back along the trail past the last man · the file this long gives its last man the rear outright
    #[k(4.0)] pub rear_m: f64,
    #[k(3.0)] pub rear_from: f64,
    #[k(60.0)] pub rear_deg: f64,
    /// what a man already on an opening has over a rival for it · he KEEPS a new one this long whatever else comes up, while it stays live and his to take (s) ·
    ///  the last man keeps the rear thing he has unless another behind him is this much heavier
    #[k(1.2)] pub sticky: f64,
    #[k(1.5)] pub keep_secs: f64,
    #[k(1.25)] pub rear_keep: f64,
    /// on the move, an opening behind a man who is not the file's last weighs this much of itself to him
    #[k(0.2)] pub behind_k: f64,
    /// walking: the gun goes over his shoulder to an opening within this of the way he goes (rad); past it he walks backwards / sideways while his pace is under
    ///  backIn (m/s; he turns and runs over backOut): set over a walking Sam's file closing a gap, under a running Sam's — the men cover while Sam walks and run
    ///  when he runs. (Entering the back-pedal mid-rush, backIn 2.3‥2.6, is what tipped orders-edges' lean-out check: see cover.md)
    #[k(1.45)] pub twist: f64,
    #[k(3.0)] pub back_in: f64,
    #[k(3.4)] pub back_out: f64,
    #[k(0.0)] pub back_body: f64,
    #[k(0.7)] pub back_turn_body: f64,
    #[k(1.0)] pub back_turn_secs: f64,
    /// how fast the muzzle travels to a newly dealt opening (1/s: it travels, it does not snap)
    #[k(9.0)] pub swing: f64,
    #[k(1.8)] pub step_m: f64,
    #[k(0.8)] pub step_after: f64,
    #[k(0.7)] pub step_off_m: f64,
    #[k(6.0)] pub step_reach: f64,
    #[k(5.0)] pub step_secs: f64,
    #[k(0.6)] pub step_place_m: f64,
}

/// ONE JOB AT A TIME (the knobs: Team feel ▸ 'rear ▸ …'). The deal is redone four times a second and an opening is 'live' for a man only while a dozen things
///  hold that frame (a line to it, nobody of ours in it, the file not going through it, nobody better placed) — so a job was handed out, taken back a quarter
///  of a second later and another handed out the deal after. Between the two the man's gun went back to what his ORDER has him watch: for a stack's rear man the
///  door, for a follower the way the file is going. Rear, door, rear, door: the play-through watcher's 'twitch' (four reversals in 2 s).
///  `settle`: a gun the deal has let go of STAYS on its opening for `secs` before the order's own watch has it back, the man carrying himself for it as he was
///  (squared up to it stood, over his shoulder or still backing on the walk); a job dealt inside that takes the gun straight from the one opening to the other.
///  The deal itself is not touched — who is dealt what, and when, is as it was (a first cut that kept the JOB a while cost the deal its freedom to re-deal).
///  It ends at once when anything else has him (an order that leaves him no spare eyes, the entry, a fight, a contact: free() says so), when Sam or a mate is
///  in the line of the gun or stood in the opening, when he is told to run with it behind him. settle false = the cover as it was, frame for frame
///  (threshold-identity pins it off). `file`: the same for a follower / a holder, not a stack's rear men only — SHIPS OFF: it takes most of the file's own
///  twitches too, but depot-four loses teammates with it on every seed set tried.
///  Measured (tools/qa/playthrough.ts, the two four-man scenarios × 72 seeds, off → on): a stack's rear men's twitches 151 → 8, all twitches 397 → 214,
///  won 113 → 116 of 144, teammates up 365 → 376 of 576 (inside the scatter). With `file` on as well: 10, 149, 117 and 335 (depot-four 207 → 180 of 288).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RearK {
    #[k(0.75)] pub secs: f64,
}
