//! The numbers of doortools.
use crate::bytes::ToBytes;
use alloc::string::String;
use alloc::vec::Vec;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct DoortoolsK {
    /// per run: door charges, and the squad's stun canisters (the F drill's included)
    #[k(2.0)] pub charges: f64,
    #[k(3.0)] pub bangs: f64,
    /// the trigger: Sam's key for a set charge (the night breach's F — a set charge has it first)
    #[k(String::from("KeyF"))] pub blow_key: String,
    /// … and for this long after a word given on the key, the key does nothing else: a second tap is not the breach drill's 'stack up'
    #[k(0.6)] pub word_tap: f64,
    /// caps (s): walking to the marks (given up sooner once nobody short of his has moved for `stillSecs`) · onto the kick spot · the whole entry
    #[k(60.0)] pub stack_cap: f64,
    #[k(2.5)] pub still_secs: f64,
    #[k(5.0)] pub kick_cap: f64,
    #[k(9.0)] pub in_cap: f64,
    /// the keyed shove: leaf velocity (rad/s) — and how long a door this module opened is kept from its closer
    #[k(5.5)] pub shove: f64,
    #[k(10.0)] pub hold_open: f64,
    /// the stun canister: released this long after the leaf starts to move, thrown for a spot this far inside the frame line, and going off IN THE AIR once it
    ///  is `airIn` inside and has dropped to `airH` — else this long after it lands, or `tossCap` after it left the hand
    #[k(0.15)] pub toss_after_open: f64,
    #[k(2.4)] pub bang_in: f64,
    #[k(0.35)] pub fuse_after_land: f64,
    #[k(1.6)] pub air_in: f64,
    #[k(1.2)] pub air_h: f64,
    #[k(1.6)] pub toss_cap: f64,
    /// a door that had to be BOOTED: the can is the second man's, cooked in his fist while the lead kicks, and leaves it this long after the leaf goes in (the
    ///  breach drill's own toss) — the kicker is square in the opening until it goes off, so every tenth counts (it was the kicker's own throw, 0.3 s after)
    #[k(0.05)] pub toss_after_kick: f64,
    /// the can is the SECOND man's on every door, not only a booted one: the man nearest the frame behind the first man (within `throwReach` of it) throws it
    ///  past him, the first man's gun stays on the opening and he is first through with it ready (false = the man who opened the door threw it, then went first). `tossTurn`: seconds a thrower who was rear
    ///  security until the shove may take to come round to the door before he throws — 0, he throws as he turns: at 0.35 the door stood open with no can in
    ///  the air that much longer, and the pair holding the next room lost four of twelve where it loses none (pairs-hold)
    #[k(4.0)] pub throw_reach: f64,
    #[k(0.0)] pub toss_turn: f64,
    #[k(1.0)] pub frag_way_wait: f64,
    #[k(1.15)] pub frag_way_open: f64,
    #[k(0.6)] pub frag_way_ajar: f64,
    #[k(3.0)] pub last_out_m: f64,
    /// entry behind a bang / a charge: not before `afterBang`, then once no defender has fired for `quiet` seconds (the blind burst run dry) — at `entryCap` whatever
    #[k(0.6)] pub after_bang: f64,
    #[k(0.15)] pub quiet: f64,
    #[k(2.8)] pub entry_cap: f64,
    /// the second man starts this long after the first · on his point of domination each man's muzzle goes `sweepRad` to one hand, to the other, then onto his
    ///  sector, `sweepSecs` each
    #[k(0.25)] pub second: f64,
    #[k(0.9)] pub sweep_rad: f64,
    #[k(0.6)] pub sweep_secs: f64,
    /// CHECK THE CORNERS (the maintainer, 10-05: 'teammates clearing rooms seem to miss enemies. they need to check their corners'): from the moment a man is
    ///  through the frame his muzzle is on the NEAR corner of his own side of the door, then on that side's DEEP corner — each until it has been within
    ///  `cornerRad` of it for `cornerSecs` (given up `cornerCap` after he crossed) — and only then his point's sweep. False: muzzle wherever he walks, as it was
    #[k(0.3)] pub corner_secs: f64,
    #[k(0.3)] pub corner_rad: f64,
    #[k(1.6)] pub corner_cap: f64,
    #[k(12.0)] pub lead_seen_min: f64,
    #[k(30.0)] pub lead_seen_secs: f64,
    #[k(11.0)] pub lead_seen_m: f64,
    /// a man left holding a room swings his sector off a teammate who stands in it (off = as it was: every man on the sector his point gave him, and the gun of
    ///  any man whose sector ran through a mate at the low ready for as long as they held — in one room three of the four)
    #[k(0.6)] pub sector_secs: f64,
    #[k(1.4)] pub scan_rate: f64,
    #[k(0.3)] pub look_rad: f64,
    #[k(11.0)] pub look_m: f64,
    #[k(8.0)] pub dead_cells: f64,
    #[k(3.0)] pub search_moves: f64,
    #[k(24.0)] pub search_cap: f64,
    #[k(900.0)] pub search_cells: f64,
    /// this module's stacks stand this much further past the jamb than orders.ts's (a man 0.45 m past the latch is in the line of a round fired from deep in
    ///  the room at an angle through the opening) · behind the bang the first man closes up to `edgeOut` past the opening's edge · behind a bang, or working a
    ///  door from outside, the men who do not open it file up `bangFile` apart behind the man who does
    #[k(0.4)] pub tuck: f64,
    #[k(0.4)] pub edge_out: f64,
    #[k(1.1)] pub bang_file: f64,
    /// the charge: seconds at the door to set it · how far along the wall from the jamb the squad stands off (first that has floor)
    #[k(2.0)] pub set_secs: f64,
    #[k(alloc::vec![2.6, 2.0, 1.5])] pub standoff: Vec<f64>,
    /// … and what it does: far-side men within `knockM` of the doorway's middle go down (out cold); within `stunM` with no wall between they are flash-blind for
    ///  stunSecs[0] (at the door) down to stunSecs[1] (at stunM) AND doubled over for `staggerSecs` (Guard.staggerUntil: rooted, gun down, no round — so no blind
    ///  burst comes back through the doorway, which is the charge's edge over a canister); a teammate (or anybody) on the NEAR side within `backblastM` of it is
    ///  dazzled `backblastSecs`; the leaf leaves at `blowVel` rad/s
    #[k(2.0)] pub knock_m: f64,
    #[k(8.0)] pub stun_m: f64,
    #[k(alloc::vec![6.0, 2.5])] pub stun_secs: Vec<f64>,
    #[k(2.0)] pub stagger_secs: f64,
    #[k(1.6)] pub backblast_m: f64,
    #[k(1.5)] pub backblast_secs: f64,
    #[k(12.0)] pub blow_vel: f64,
    /// clear from outside: × the arc's distances off the frame line · how long the muzzle rests on each of a station's angles with nobody in his sight · how far
    ///  inside the jamb's edge the sight lines are taken · how deep into the room the points he aims at lie
    #[k(1.0)] pub slice_back: f64,
    #[k(0.35)] pub slice_look: f64,
    #[k(0.12)] pub slice_edge: f64,
    #[k(3.5)] pub slice_deep: f64,
    /// a slicing man's step from one station to the next, m/s — the footage's men take an angle at under a metre a second (0 = the pace of any other short leg, as it was)
    #[k(0.9)] pub slice_pace: f64,
    /// THE WORD (go()): the orders it releases wait this long for each other at the two moments they share — before anybody makes a sound (a pair still walking
    ///  onto its door), and before anybody crosses a threshold (a bang still in the air at the other door) — then go without the one that is late
    #[k(20.0)] pub word_stack: f64,
    #[k(6.0)] pub word_in: f64,
    /// the squad of four: a pair stacked on a door the OTHER pair is sent to work steps back to the first of these (metres along the wall past a jamb) that has
    ///  floor · two points of domination dealt to men of different orders are this far apart, or the later one takes another
    #[k(alloc::vec![2.8, 3.7, 4.6, 5.5])] pub room_back: Vec<f64>,
    #[k(1.2)] pub pod_apart: f64,
    #[k(alloc::vec![0.45, 1.8])] pub thr_pull_at: Vec<f64>,
    #[k(alloc::vec![0.45, 1.2])] pub thr_still: Vec<f64>,
    #[k(alloc::vec![alloc::vec![-0.45, 1.2], alloc::vec![0.25, 1.8]])] pub thr_arc: Vec<Vec<f64>>,
    #[k(4.0)] pub thr_min: f64,
    #[k(2.0)] pub thr_quiet: f64,
    #[k(25.0)] pub thr_cap: f64,
    #[k(2.0)] pub thr_bangs: f64,
    #[k(12.0)] pub thr_pocket: f64,
    #[k(8.0)] pub thr_shot_m: f64,
    #[k(5.0)] pub thr_throw_m: f64,
    #[k(7.0)] pub thr_flash_m: f64,
    #[k(0.5)] pub thr_turn: f64,
    #[k(0.3)] pub thr_next: f64,
    #[k(1.0)] pub thr_light: f64,
    #[k(4.0)] pub thr_in: f64,
    #[k(alloc::vec![alloc::vec![-0.2, 1.4], alloc::vec![-0.05, 1.6], alloc::vec![0.08, 1.8]])] pub thr_pie_arc: Vec<Vec<f64>>,
    #[k(0.65)] pub thr_pie_apart: f64,
    #[k(0.15)] pub thr_lean_m: f64,
    /// … the seconds that one lean is held (a cut's own look is `sliceLook`: too short to be seen, or to be sure his lines are asked with his head out)
    #[k(1.1)] pub thr_lean_look: f64,
    #[k(5.5)] pub thr_doors_m: f64,
    #[k(16.0)] pub thr_doors_walk: f64,
    #[k(1.0)] pub thr_doors_stay: f64,
    /// THE ROOM'S OTHER DOOR HAS A GUN ON IT FROM THE ENTRY ITSELF (`otherDoorM`, metres; 0 = as it was: every man of a bang / open and clear on his corners,
    ///  the floor and then the sector his point gave him, the room's other doors in nobody's — the warehouse's front office and the compound's garage: two men
    ///  shot in the back through the connecting door within seconds of taking the room, 72 of 167 teammates lost in 48 warehouse runs). Every other door of the
    ///  room being taken that NOBODY OF THE SQUAD HAS BEEN BEYOND is one man's from the
    ///  moment a man of the entry is on his point, his corners checked, with a line to its frame from within this many metres: the nearest such man turns his gun
    ///  onto it where he stands and keeps it there — through the rest of the search (the floor is the others': while the room is still being looked over no
    ///  more than half the entry's men are turned off it — one of two or three, two of four) and on the hold after, until an order takes him; if he drops, the
    ///  next man with a line has it. With more doors than men to spare: a door into the building before one to the outside, the nearer first. Nobody walks
    ///  anywhere for it. A door another order works, one a man already holds, or one a teammate of another order stands by or was told to watch (a man Sam
    ///  posted there himself) is left to them — and given back to them if they come to it after: its man is on his sector again; so is the man of a door the
    ///  squad has since gone through. The man on a door holds it as a held door is held: `thrDoorsLit` — with no goggles, his weapon light stays on it. (The threshold order's own far doors — `thrDoors`, with its walk — come after, for what is still nobody's.)
    #[k(9.0)] pub other_door_m: f64,
    #[k(5.0)] pub buddy_m: f64,
    #[k(12.0)] pub buddy_pair_m: f64,
    #[k(8.0)] pub buddy_left_m: f64,
    #[k(28.0)] pub flow_cells: f64,
    #[k(5.0)] pub flow_moves: f64,
    #[k(5.5)] pub flow_reach: f64,
    #[k(3.0)] pub flow_deep: f64,
    #[k(8.0)] pub flow_bound: f64,
    #[k(10.0)] pub flow_secs: f64,
    #[k(20.0)] pub flow_fight: f64,
    #[k(400.0)] pub flow_max: f64,
    #[k(8.0)] pub word_cuts_cap: f64,
    #[k(1.5)] pub bang_into_min: f64,
    #[k(0.9)] pub bang_into_lit: f64,
    #[k(6.0)] pub bang_into_deep: f64,
    #[k(4.2)] pub bang_into_wide: f64,
    #[k(0.9)] pub bang_hand_m: f64,
    #[k(6.5)] pub bang_pass_v: f64,
    #[k(0.55)] pub pod_frame_m: f64,
    #[k(6.0)] pub thr_hand_wait: f64,
    #[k(0.25)] pub thr_hand_every: f64,
    #[k(8.0)] pub frag_hold_cap: f64,
    #[k(12.0)] pub threat_secs: f64,
    #[k(1.5)] pub threat_cap: f64,
    #[k(1.5)] pub threat_near_m: f64,
    #[k(1.0)] pub threat_stuck: f64,
    /// A CAN GOES IN BEFORE ANYBODY SHOWS IN THE OPENING (`thrFirst`: 0 = never — both jamb men step onto their diagonals as the leaf swings, as it was ·
    ///  1 = on a HOT floor: a shot, a charge or a boot in the ears of a man of the order inside ROOMFIGHT.teamHot · 2 = on every door). A room that has heard
    ///  the floor go loud has its guns on its doors: the man who steps into the opening is seen by a man already settled on it, and that duel is a coin's — it
    ///  was where the stage lost its men. So on such a door the leaf is opened from the jamb and the two guns stay on their marks behind the wall; the
    ///  first of the stage's cans (one of `thrBangs`, not one more) goes where its burst has a line to the most floor; the blind burst it draws is let run
    ///  dry on a bang's own count (`afterBang` · `quiet` · `entryCap`); and only then do they step out onto the diagonals and work the room from the door —
    ///  for `thrFirstMin` s at least (the hold's `thrMin` is for a room that can still see; this one cannot for FLASH.secs — 1 s measured worse than 2: more lost
    ///  stepping in than saved at the door). No can in the kit, one of ours (a teammate, or Sam) beyond the door inside its reach, an order on a word or held
    ///  until told (the cans are the player's to call there), or a gate: the stage as it was. A can that went off on the team's side earns a plain hold. `thrFirstStale`: a hold
    ///  that runs on past this many seconds from that can ends with its flash worn off the far men — the floor it lit and no muzzle has been across since is
    ///  a blind pocket again, and gets the stage's next can before anybody steps in (0 = floor banged once stays banged: they walked in on men who could see).
    ///  `thrFirstIn`: that first can is cooked short — it goes off this far inside the frame line on its way to the pocket it was thrown at, not at the pocket
    ///  (0): thrown on the shove (`tossAfterOpen`) it bursts under half a second after the leaf moves, where the flight to a pocket three metres in took most of one.
    ///  `thrFirstOnly`: a can that went in first is the door's ONLY can — behind it they hold `thrFirstMin`, turn in to the jambs and step in (off = the stage's
    ///  own count, as the rule first went in: floor the first can left dark got the second of `thrBangs` before anybody crossed. Measured over 96 seeds the two
    ///  lose and win the same within the scatter; the one can saves about 25 of a block's 445, cans the next door has a use for — and with it on `thrFirstStale`
    ///  has nothing to do. Stepping straight in off the diagonals, with no turn in to the jambs since no throw is coming, was measured too: no better on 48 seeds,
    ///  19 teammates of 576 worse on the next 48 — the turn in stays). Sam's word and a told 'bang' are his to ask a can with: they get the stage's count
    ///  whatever this says
    #[k(1.0)] pub thr_first: f64,
    #[k(2.0)] pub thr_first_min: f64,
    #[k(0.0)] pub thr_first_stale: f64,
    #[k(0.0)] pub thr_first_in: f64,
    #[k(2.0)] pub thr_last_m: f64,
    #[k(3.0)] pub thr_last_fresh: f64,
}
