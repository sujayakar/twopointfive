//! The numbers of orders.
use crate::bytes::ToBytes;
use alloc::string::String;
use alloc::vec::Vec;
use game_derive::Knobs;

/// The knobs (__orders.knobs).
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct OrdersK {
    /// press: who the orders go to — gold → red → blue for a squad in pairs (all → each man for the one pair) · on a teammate under the cursor: that man
    #[k(String::from("KeyT"))] pub who_key: String,
    /// press: the target's own first order at once, no menu (RoN's 'issue default command') — move there on floor, stack up on a door, fall in on nothing.
    ///  THE CODE the game asks for, as every recording and tape holds it. The KEY that gives it on the page's board is Q (host/keymap.ts `qCmd`; the
    ///  maintainer, 10-21, once the wall hold had left Q: "let's move the squad command button to Q then too", "let's unmap B too then", "so it's just Q, and
    ///  the quick tap should be exactly the same as B"): the board moved, this read did not
    #[k(String::from("KeyB"))] pub quick_key: String,
    /// press, with the menu up on a door's tools (the squad of four): the door tools given from this page WAIT on the door for the word (a row of its own,
    ///  toggled — never a key held down). The menu stays up; the rows it changes say so. `waitKey`: the row's digit, on its page (taken only while no order of
    ///  the page is on it; a digit above ORDER_DIGITS is no key at all — nothing of the menu is up there; a key that is not a digit works and is shown as itself). `waitAlt`: a second key for it — the word's own, so 'on F' is one key to learn: with a DOOR menu up it is
    ///  'wait for my word' (on the door's first page it turns to the tools with the row already on; on the tools it flips it) and it gives the word everywhere
    ///  else ('' = none)
    #[k(String::from("Digit5"))] pub wait_key: String,
    #[k(String::from("KeyF"))] pub wait_alt: String,
    /// the open menu's men, door face and why-nots are re-read this often (game seconds; a path search per man per row each time)
    #[k(1.0)] pub refresh_secs: f64,
    /// the reticle within this of a doorway's centre (or of its leaf) is ON that door; within `selfM` of Sam or `manM` of a teammate it is on nobody's floor
    #[k(1.0)] pub door_pick_m: f64,
    #[k(1.0)] pub self_m: f64,
    #[k(0.8)] pub man_m: f64,
    #[k(0.08)] pub door_ray_pad: f64,
    #[k(3.0)] pub door_ray_reach_m: f64,
    /// a reticle on something a man cannot stand on (a wall's top, a desk) means the nearest free floor within this of it
    #[k(1.0)] pub floor_snap_m: f64,
    /// the stack: how far off the wall a man stands, how far past the jamb, and the gap to the second man when both file on one side
    #[k(0.55)] pub jamb_out: f64,
    #[k(0.45)] pub jamb_past: f64,
    #[k(0.9)] pub file_gap: f64,
    /// a stack place needs its line to the doorway's front clear this far to either hand as well (metres; 0 = the one line, as it was): a line along the seam
    ///  between two abutting boxes of a wall is no line (stackPostsOn)
    #[k(0.03)] pub seam_m: f64,
    #[k(4.0)] pub work_seen_m: f64,
    /// fall in: each man's place back along Sam's trail (metres), a crumb every `crumb` metres of his walking, `trailM` of trail kept
    #[k(alloc::vec![1.3, 2.4])] pub follow_back: Vec<f64>,
    #[k(0.4)] pub crumb: f64,
    #[k(10.0)] pub trail_m: f64,
    /// fall in's pace: Sam's own speed plus `gain` × how far the man is off his place, between the walk and `run` — which is no run: the most a teammate's legs
    ///  are asked for, so a Sam who sprints (4.7) leaves his file behind and it closes up at its own pace when he stops (it was 4.7: the file
    ///  sprinted with him) — no gear changes in view. With teampace.ts on the pace is teamFollow's: a share of his walk, and a run while he runs
    #[k(2.2)] pub run: f64,
    #[k(1.5)] pub walk: f64,
    #[k(1.5)] pub gain: f64,
    /// Sam stopped: a follower already between `nearM` and his file distance + `bandM` of him, with nothing between them, stays where he is — he does not walk
    ///  round Sam to an exact place (Sam leans out past a corner and back: nobody follows the lean into the open)
    #[k(0.6)] pub near_m: f64,
    #[k(0.6)] pub band_m: f64,
    /// the squad row's WORD for a follower ('with you' / 'falling in'), which is not `arrived`: 'in place' is a bare 0.8 m line and his goal is dealt again in half
    ///  metre jumps, so on a walk it flipped five times a second. The word is latched: on when he is in place, off only once he is `wordFarM` off his place or has
    ///  been out of place for `wordHoldS` seconds of game time. Both 0 = the word is `arrived` again. Only the HUD row and the replay trace read it. (Measured
    ///  on a 1.55 m/s walk, four men, both stands: out of place 0.05‥0.25 s at a time and up to 1.2 m, three times a second — and, every few seconds a man,
    ///  for 1.3‥1.7 s and up to 1.7 m, which is still the file walking: 1.5 m / 0.5 s let those through, four to seven word changes in ten seconds)
    #[k(2.0)] pub word_far_m: f64,
    #[k(2.5)] pub word_hold_s: f64,
    #[k(0.9)] pub file_apart: f64,
    #[k(0.75)] pub file_tight: f64,
    /// Sam stopped: nobody's place in the file is IN an open doorway — a place that falls within this of a frame's line, inside its span, goes back along the
    ///  trail to the first floor clear of it, and the men behind him go back with him (0: places fall where they fall — a man stood in the frame for as long as
    ///  Sam stood still: the one way in or out blocked, and the fatal funnel his to stand in)
    #[k(0.8)] pub off_door: f64,
    /// Sam walking at a follower inside `asideM` of him and within `asideLane` of his line: the man stands `asideOff` off that line until Sam is past
    #[k(2.6)] pub aside_m: f64,
    #[k(0.8)] pub aside_lane: f64,
    #[k(0.95)] pub aside_off: f64,
    #[k(1.3)] pub file_queue_gap: f64,
    #[k(500.0)] pub file_queue_cost: f64,
    #[k(0.25)] pub prop_round_pad: f64,
    #[k(6.0)] pub prop_round_cost: f64,
    #[k(2.0)] pub prop_round_more: f64,
    #[k(2.5)] pub prop_round_door: f64,
    /// … and he keeps going for that spot this long after Sam's own pace last said so: a man at the team's walk is not always off the line before Sam is on him,
    ///  and Sam stopped BY him reads as Sam stopped — the man then took where he stood for his place and the two stood chest to chest in a doorway
    #[k(1.5)] pub aside_hold: f64,
    /// a leg to a mark (move, stack, a point of domination): a brisk, deliberate walk (it was the breach drill's 3.4 run, and 1.7 onto the mark) down to the last
    ///  `briskIn` metres, then the slow walk onto it
    #[k(2.0)] pub brisk: f64,
    #[k(1.8)] pub brisk_in: f64,
    #[k(1.3)] pub onto: f64,
    /// THROUGH A DOOR: the one leg taken at the team's full pace, all the way onto the
    ///  point — a deliberate stack, then a committed entry
    #[k(2.2)] pub entry: f64,
    /// move there, to the pair: the gap between the two spots
    #[k(0.9)] pub move_spread: f64,
    /// a man the fight AI has (the seam in the head comment) is back on his order this long after he last saw anybody / heard a shot / was handed over — never
    ///  mid-reload; `pinPosted`: he fights from his post (false: the fight layer moves him as it likes and he walks back after)
    #[k(2.5)] pub cold_secs: f64,
    /// Sam within this of the line a man's gun is on (`laneOut` to come back up): the muzzle goes down until he is clear
    #[k(0.45)] pub lane_in: f64,
    #[k(0.7)] pub lane_out: f64,
    /// hold: a man on the move stops a stride on (this × his velocity), not on the spot
    #[k(0.15)] pub hold_stride: f64,
    /// a man stopped this long short of his mark is as near as he will get: the mark moves to him
    #[k(4.0)] pub stuck_secs: f64,
    /// the squad of four, left holding (holdArcs): men within `holdGroupM` of one another share the approaches out · a doorway this near in a man's arc has his
    ///  gun on it (further off, the gun — and its light — is up along the arc: a light on a door three metres away leaves the corridor past it dark)
    #[k(4.0)] pub hold_group_m: f64,
    #[k(2.0)] pub hold_door_m: f64,
    #[k(20.0)] pub cleared_dear: f64,
    #[k(1.6)] pub round_cap: f64,
    #[k(5.0)] pub same_near_m: f64,
    #[k(6.0)] pub same_apart_m: f64,
    #[k(1.5)] pub same_long: f64,
    #[k(3.0)] pub trail_reach_m: f64,
    #[k(1.5)] pub trail_long: f64,
}
