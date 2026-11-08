//! WHAT THE DOOR AND SQUAD TESTS ASK OF A GAME, each question once (rust/game_start/README.md rule 1), and their one scene: two rooms on a yard.
#![allow(dead_code)]
use game::doortools::{DoorToolsStats, Phase};
use game::game::GuardState;
use game::ids::*;
use game_start::play::{nudge, Play};
use game_start::scene::{Axis, Scene};

pub const SEEDS: [u32; 3] = [1, 2, 3];
pub const NORTH: f64 = std::f64::consts::PI;
/// a man's body, for the leaf; and how far the handle sits in from the leaf's end (tools' handle watcher's numbers: 1 m is "at the handle")
pub const BODY_R: f64 = 0.34;
pub const HANDLE_IN: f64 = 0.14;
pub const HANDLE_REACH: f64 = 1.0;

/// Two rooms side by side, each with a door on the yard south of them and one door between them, a bulb in each and a defender in each; the team
/// in the yard under the moon, facing the building. The west room's yard door stands up to a metre either way along its wall, by the seed.
/// The scene, and the middle of that door.
pub fn yard(seed: u32) -> (Scene, [f64; 2]) {
    let x = 13.4 + nudge(seed, 40);
    let mut s = Scene::new("door-yard");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, x).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([x, 23.0], NORTH, &[[x - 1.0, 21.0], [x + 1.0, 21.0], [x - 1.0, 22.0], [x + 1.0, 22.0]]);
    s.defender("west", [11.25, 11.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let door = s.door_centre("west-yard");
    (s, door)
}

pub trait DoorsAsk {
    /// a door of the level by its name
    fn door(&self, name: &str) -> DoorId;
    /// is the leaf home and latched, on its hinges
    fn door_shut(&self, d: DoorId) -> bool;
    /// is the leaf off its hinges
    fn door_gone(&self, d: DoorId) -> bool;
    /// how many of each thing the squad's door tools have done
    fn tool_stats(&self) -> DoorToolsStats;
    /// the door orders running on that door
    fn ops_on(&self, d: DoorId) -> Vec<DoorOpId>;
    /// the phase of a running door order
    fn op_phase(&self, op: DoorOpId) -> Phase;
    /// the lead of a door order, running or over
    fn op_lead(&self, op: DoorOpId) -> GuardId;
    /// what a door order's own log has, in order (`can waits`, `bang · inside`, `first:…`)
    fn op_beats(&self, op: DoorOpId) -> Vec<String>;
    /// the order a man is on now, by its id
    fn doing(&self, gd: GuardId) -> Option<String>;
    /// the door a man's order is on
    fn order_door(&self, gd: GuardId) -> Option<DoorId>;
    /// how far along the shut leaf from its hinge a place is (the leaf's width is `door_w`), and how far off it on the side `side` (+1 or -1)
    fn along(&self, d: DoorId, at: [f64; 2]) -> f64;
    fn off(&self, d: DoorId, at: [f64; 2], side: f64) -> f64;
    fn door_w(&self, d: DoorId) -> f64;
    /// which face of a door a place is on (the game's own side)
    fn face(&self, d: DoorId, at: [f64; 2]) -> f64;
    /// does the leaf open toward that place
    fn pull_face(&self, d: DoorId, at: [f64; 2]) -> bool;
    /// the live men of the squad
    fn men(&self) -> Vec<GuardId>;
    /// the men of the squad past a door's frame on the far side from `side`
    fn across(&self, d: DoorId, side: f64) -> Vec<GuardId>;
    /// every man of the other side goes down, quietly: the door's own story is all that plays
    fn floor_cleared(&mut self);
    /// STAGING: furniture is wedged against the door on the room's side, as a defender wedges one (digin's `wedge`), locked against the face `from` stands on
    fn wedged_from(&mut self, d: DoorId, from: [f64; 2]);
    /// what the row of that id on the order menu reads on what stands at `at`, and why the game would not take it there (None: live)
    fn row(&mut self, id: &str, at: [f64; 2]) -> (String, Option<String>);
    /// an order by the menu's call, that may be refused: the game's words
    fn try_order(&mut self, id: &str, at: [f64; 2], who: game::orders::Who) -> Option<String>;
    /// has a man said a line holding these words
    fn said_it(&self, words: &str) -> bool;
    /// when a door order's log first had a line beginning so, on the game's clock
    fn beat_t(&self, op: DoorOpId, starts: &str) -> Option<f64>;
    /// when each man of a door order came over the frame line
    fn crossed_t(&self, op: DoorOpId) -> Vec<f64>;
    /// how a door order ended ('in', 'found shut', …): None while it runs
    fn op_ended(&self, op: DoorOpId) -> Option<String>;
    /// the door order that went last onto that door, running or over
    fn last_op(&self, d: DoorId) -> Option<DoorOpId>;
    /// the men of a door order, as it has them now
    fn op_men(&self, op: DoorOpId) -> Vec<GuardId>;
    /// where the burst of a door order's can was
    fn op_bang_at(&self, op: DoorOpId) -> Option<[f64; 2]>;
    /// how many charges, bangs and frags the squad has left
    fn left(&self) -> (f64, f64, f64);
    /// STAGING: the squad starts with this many charges and bangs
    fn carries(&mut self, charges: f64, bangs: f64);
    /// is he out cold from a blast
    fn struck(&self, gd: GuardId) -> bool;
    /// is he doubled over
    fn staggered(&self, gd: GuardId) -> bool;
    /// the door's frame centre, on the floor
    fn frame_at(&self, d: DoorId) -> [f64; 2];
    /// is the lock's keep broken
    fn keep_broken(&self, d: DoorId) -> bool;
    /// the leaf is put home and latched by a hand (no order is touched)
    fn leaf_put_shut(&mut self, d: DoorId);
    /// the leaf is left standing open, its own way, by a hand
    fn leaf_put_open(&mut self, d: DoorId, rad: f64);
    /// the leaf is hung the other way, so that the face it opens toward is `toward`'s
    fn hung_toward(&mut self, d: DoorId, toward: [f64; 2]);
    /// where a man's order has him stand
    fn post_of(&self, gd: GuardId) -> Option<[f64; 2]>;
    /// what a man's order says he is doing there (the order's own line)
    fn there_of(&self, gd: GuardId) -> String;
    /// has a man's order brought him to his mark
    fn arrived(&self, gd: GuardId) -> bool;
    /// the door tools' knobs
    fn tools_k(&self) -> game::knobs::DoortoolsK;
    /// is the leaf on its latch
    fn latched(&self, d: DoorId) -> bool;
    /// the leaf's angle, radians off shut
    fn leaf_angle(&self, d: DoorId) -> f64;
    /// is the leaf swinging
    fn leaf_vel(&self, d: DoorId) -> f64;
    /// the leaf is kept from closing itself for the while of a test
    fn leaf_stays(&mut self, d: DoorId);
    /// the encounter starts again, as the restart key has it
    fn restarted(&mut self);
    /// is any door order running
    fn any_op(&self) -> bool;
    /// how long after a round the game holds a man short of the lane
    fn fire_lasts(&self) -> f64;
    /// every door of the level
    fn doors(&self) -> Vec<DoorId>;
    /// a door's name
    fn door_name(&self, d: DoorId) -> String;
    /// the place the game deals a man at a face of a door: where, on the pull face, and is it only a long reach
    fn handle_place(&mut self, d: DoorId, side: f64) -> Option<([f64; 2], bool, bool)>;
    /// is this door order a can's (bang or frag)
    fn op_is_can(&self, op: DoorOpId) -> bool;
    /// the can a door order has out, its fuse left, and has it gone off
    fn can_out(&self, op: DoorOpId) -> Option<(ItemId, f64, bool)>;
    /// its fuse once the order is gone: the short one
    fn can_now(&self, can: ItemId) -> (f64, bool);
    /// the way his gun points, radians (0 looks south)
    fn gun_yaw(&self, gd: GuardId) -> f64;
    /// what his order has him watch
    fn watch_of(&self, gd: GuardId) -> Option<[f64; 2]>;
    /// is that floor free to stand on
    fn floor_free(&self, at: [f64; 2]) -> bool;
    /// how many orders the squad has been given
    fn orders_given(&self) -> usize;
    /// the order menu as it stands, if one is up: its page and how many rows
    fn menu_up(&self) -> Option<(Option<String>, usize)>;
    /// the menu opens on what stands at `at`, as its key does, optionally turned to a page
    fn menu_opens(&mut self, at: [f64; 2], page: Option<&str>);
    /// the key on the row of that id on the open page
    fn row_key(&self, id: &str) -> Option<f64>;
    /// the slot Sam has in his hands
    fn sam_slot(&self) -> game::game::SlotKind;
    /// a key tapped for two frames
    fn tap(&mut self, code: &str);
    /// the floor `along` the shut leaf from its hinge and `off` from it (+ on the frame's normal side)
    fn on_face(&self, d: DoorId, along: f64, off: f64) -> [f64; 2];
    /// STAGING: what a quiet mission with one partner throws: the pair rules on and the squad silent
    fn quiet_pair(&mut self);
    /// a partner's take job on a man: is he set, has the mark gone, who has eyes on the man
    fn job(&self, gd: GuardId) -> Option<(bool, f64, Option<GuardId>)>;
    /// is the man in the partner's hands
    fn gripped(&mut self, gd: GuardId) -> bool;
    /// an order on a man of the other side, by the menu's call
    fn order_on_foe(&mut self, id: &str, foe: GuardId) -> Option<String>;
    /// how many rounds the squad's men have fired
    fn squad_rounds(&self) -> f64;
    /// a man is turned to face that place, head and chest, as he stands
    fn turned_to(&mut self, gd: GuardId, at: [f64; 2]);
    /// a door order given by the menu's call on the word: it waits for it
    fn order_on_word(&mut self, id: &str, at: [f64; 2], who: game::orders::Who);
    /// the places inside a door order has dealt its men
    fn op_pods(&self, op: DoorOpId) -> Vec<[f64; 2]>;
    /// has his order got his muzzle down
    fn muzzle_down(&self, gd: GuardId) -> bool;
    /// is a man's light on, by his order
    fn light_on(&self, gd: GuardId) -> bool;
    /// a man's callsign
    fn callsign(&self, gd: GuardId) -> String;
    /// is the squad on its silent word
    fn silent(&self) -> bool;
    /// how far before a man's gun, along its heading at its own height, something stands within `reach`
    fn gun_walled(&mut self, gd: GuardId, reach: f64) -> bool;
    /// the movable thing of the level nearest a place, and where it stands
    fn prop_near(&self, at: [f64; 2]) -> PropId;
    fn prop_at(&self, prop: PropId) -> [f64; 2];
    /// STAGING: it stands here, at rest
    fn prop_put(&mut self, prop: PropId, at: [f64; 2]);
    /// an order on the floor at a place, whatever stands near it
    fn order_floor(&mut self, id: &str, at: [f64; 2], who: game::orders::Who);
    /// where the leaf's far end is now, on the floor
    fn leaf_tip(&self, d: DoorId) -> [f64; 2];
    /// the hinge, on the floor
    fn hinge(&self, d: DoorId) -> [f64; 2];
    /// the men of a door order not on rear security
    fn op_front_men(&self, op: DoorOpId) -> Vec<GuardId>;
    /// where the game would have the squad's lead boot that face from, and the square spot in front of the leaf, and the spot beside the latch
    fn boot_spots(&mut self, d: DoorId, side: f64) -> ([f64; 2], [f64; 2], [f64; 2]);
    /// how many lines through glass reach a man there from the floor beyond the door, and how many it takes
    fn glass_lines(&mut self, d: DoorId, side: f64, at: [f64; 2]) -> f64;
    fn glass_bar(&self) -> (f64, f64);
    /// a door order given on a named face
    fn order_on_face(&mut self, id: &str, d: DoorId, from: [f64; 2]) -> Option<String>;
    /// does the picture show the leaf open: the fog holds nothing of it back
    fn picture_shows_open(&self, d: DoorId) -> bool;
    /// STAGING: the fog of war on, off
    fn fog_on(&mut self);
    fn fog_off(&mut self);
    /// the last order given, and who took it (their callsigns)
    fn last_order(&self) -> Option<(String, Vec<String>)>;
    /// the squad's quick key tapped where the cursor is, on a place of the world
    fn key_tapped_on(&mut self, at: [f64; 3]);
    /// STAGING: Sam's choice of who takes an order is left on that pair
    fn choice_on(&mut self, who: game::orders::Who);
    /// a man's walk comes to a corner the other side of him: left in his order as a way round a room leaves it
    fn corner_in_his_way(&mut self, gd: GuardId, at: [f64; 2]);
    /// the fight takes him, by the seam it takes a man: his script ended and he alert. Is his order waiting on the fight
    fn fight_has(&mut self, gd: GuardId);
    fn order_waits_on_fight(&self, gd: GuardId) -> bool;
    /// THE RESCUE, as the game keeps it: its log, each line and when
    fn rescue_log(&self) -> Vec<(f64, String)>;
    /// when its clock started
    fn rescue_t0(&self) -> f64;
    /// how far the captor's clock has run, 0 to 1
    fn rescue_clock(&self) -> f64;
    /// is it over, and lost
    fn rescue_lost(&self) -> Option<bool>;
    /// its men: the captor first
    fn rescue_men(&self) -> Vec<GuardId>;
    fn rescue_hostages(&self) -> Vec<GuardId>;
    /// how many blind rounds the room has fired through its door
    fn rescue_blind(&self) -> f64;
    /// STAGING: the clock's knobs, and the mission staged again on them: seconds to the first shot, then the second, seconds for a man to be set
    fn rescue_on(&mut self, first: f64, second: f64, set: f64);
    /// the room fight's notes of a man
    fn fight_notes(&self, name: &str) -> Vec<String>;
    /// how long he lines up on her
    fn rescue_aim(&self) -> f64;
    /// STAGING: a flash has his eyes for so long
    fn dazzled_for(&mut self, gd: GuardId, secs: f64);
    /// a blast of a frag's tearing reach at a place, by the doors' own call: the doors it took off
    fn blast_tears(&mut self, at: [f64; 2]) -> Vec<DoorId>;
    /// how wide Sam's line of fire is reckoned `along` metres down it, either side of its middle
    fn lane_half(&self, along: f64) -> f64;
    /// how many rounds have left Sam's rifle
    fn sam_rounds(&self) -> f64;
    /// is the man of a squad fallen in on Sam, his place in the file
    fn file_place(&self, gd: GuardId) -> Option<f64>;
    /// Sam pulls the trigger for one round, whatever he last fired
    fn sam_clicks(&mut self);
    /// is a place in a door's opening
    fn in_doorway(&self, d: DoorId, at: [f64; 2]) -> bool;
    /// is a place inside the leaf's sweep, a body's width in hand
    fn in_sweep(&self, d: DoorId, at: [f64; 2]) -> bool;
    /// how far a place is from the leaf's handle
    fn from_handle(&self, d: DoorId, at: [f64; 2]) -> f64;
    /// the rows on offer to Sam for this door
    fn door_rows(&self, d: DoorId) -> usize;
    /// the leaf off its hinges: at rest, and how far past the frame's line on `side` it lies
    fn slab_rest(&mut self, d: DoorId, side: f64) -> Option<(bool, f64)>;
    /// does nothing stand between these two places, chest high
    fn clear_line(&mut self, a: [f64; 2], b: [f64; 2]) -> bool;
    /// the man of a door order who came over the frame line first
    fn first_over(&self, op: DoorOpId) -> Option<GuardId>;
    /// how much of the room behind `at` a man has seen lately, in cells
    fn seen_of_room(&mut self, gd: GuardId, at: [f64; 2]) -> f64;
}

impl DoorsAsk for Play {
    fn door(&self, name: &str) -> DoorId { game::doors::doors_by_name(&self.g, name).unwrap_or_else(|| panic!("no door '{name}'")) }
    fn door_shut(&self, d: DoorId) -> bool { !game::doors::door_off(&self.g, d) && game::doors::door_latched(&self.g, d) && game::doors::door_angle(&self.g, d).abs() < 0.05 }
    fn door_gone(&self, d: DoorId) -> bool { game::doors::door_off(&self.g, d) }
    fn tool_stats(&self) -> DoorToolsStats { self.g.door_tools.stats }
    fn ops_on(&self, d: DoorId) -> Vec<DoorOpId> { self.g.door_tools.ops.iter().copied().filter(|&o| self.g.tab.doortools_door_op[o].door == d).collect() }
    fn op_phase(&self, op: DoorOpId) -> Phase { self.g.tab.doortools_door_op[op].phase }
    fn op_lead(&self, op: DoorOpId) -> GuardId { self.g.tab.doortools_door_op[op].lead }
    fn op_beats(&self, op: DoorOpId) -> Vec<String> { self.g.tab.doortools_door_op[op].beats.iter().map(|b| b.what.clone()).collect() }
    fn doing(&self, gd: GuardId) -> Option<String> { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).map(|a| self.g.tab.orders_assignment[a].order.clone()) }
    fn order_door(&self, gd: GuardId) -> Option<DoorId> { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).and_then(|a| self.g.tab.orders_assignment[a].door) }
    fn along(&self, d: DoorId, at: [f64; 2]) -> f64 { let f = game::guards::frame_of(&self.g, d); (at[0] - f.hx) * f.ux + (at[1] - f.hz) * f.uz }
    fn off(&self, d: DoorId, at: [f64; 2], side: f64) -> f64 { let f = game::guards::frame_of(&self.g, d); ((at[0] - f.hx) * f.nx + (at[1] - f.hz) * f.nz) * side }
    fn door_w(&self, d: DoorId) -> f64 { self.g.dr[d].def.width }
    fn face(&self, d: DoorId, at: [f64; 2]) -> f64 { game::doors::doors_side(&self.g, d, [at[0], 0.0, at[1]]) }
    fn pull_face(&self, d: DoorId, at: [f64; 2]) -> bool { game::doors::doors_pull_face(&self.g, d, [at[0], 0.0, at[1]]) }
    fn men(&self) -> Vec<GuardId> { self.mates().into_iter().filter(|&m| self.state(m) != GuardState::Dead).collect() }
    fn across(&self, d: DoorId, side: f64) -> Vec<GuardId> { self.men().into_iter().filter(|&m| self.off(d, self.at(m), side) < -0.15).collect() }
    fn floor_cleared(&mut self) { self.drop_all_but(&[]); }
    fn wedged_from(&mut self, d: DoorId, from: [f64; 2]) {
        let side = self.face(d, from);
        self.g.dr[d].key_side = side;
        self.g.dr[d].lock_id = format!("wedge:{}", self.g.dr[d].def.name);
        game::doors::door_set_locked(&mut self.g, d, true);
        game::doors::door_set_pick(&mut self.g, d, 0.0);
    }
    fn row(&mut self, id: &str, at: [f64; 2]) -> (String, Option<String>) {
        let t = game::orders::squad_orders_target_at(&mut self.g, [at[0], 0.0, at[1]]);
        let d = self.g.tab.orders_order_def.iter().map(|(i, _)| i).find(|&d| self.g.tab.orders_order_def[d].id == id).unwrap_or_else(|| panic!("no row '{id}'"));
        (game::orders::squad_orders_label_of(&mut self.g, d, &t), game::orders::squad_orders_why_not(&mut self.g, d, &t, None))
    }
    fn try_order(&mut self, id: &str, at: [f64; 2], who: game::orders::Who) -> Option<String> {
        let t = game::orders::squad_orders_target_at(&mut self.g, [at[0], 0.0, at[1]]);
        game::orders::squad_orders_give(&mut self.g, id, Some(&t), Some(who), None)
    }
    fn said_it(&self, words: &str) -> bool { self.said.iter().any(|l| l.text.contains(words)) }
    fn beat_t(&self, op: DoorOpId, starts: &str) -> Option<f64> { self.g.tab.doortools_door_op[op].beats.iter().find(|b| b.what.starts_with(starts)).map(|b| b.t) }
    fn crossed_t(&self, op: DoorOpId) -> Vec<f64> { self.g.tab.doortools_door_op[op].crossed.values().copied().collect() }
    fn op_ended(&self, op: DoorOpId) -> Option<String> { self.g.tab.doortools_door_op[op].ended.clone() }
    fn last_op(&self, d: DoorId) -> Option<DoorOpId> { self.g.door_tools.ops.iter().chain(self.g.door_tools.done.iter()).copied().filter(|&o| self.g.tab.doortools_door_op[o].door == d).max_by(|a, b| self.g.tab.doortools_door_op[*a].t0.partial_cmp(&self.g.tab.doortools_door_op[*b].t0).unwrap()) }
    fn op_men(&self, op: DoorOpId) -> Vec<GuardId> { self.g.tab.doortools_door_op[op].men.clone() }
    fn op_bang_at(&self, op: DoorOpId) -> Option<[f64; 2]> { self.g.tab.doortools_door_op[op].bang_at.map(|p| [p[0], p[2]]) }
    fn left(&self) -> (f64, f64, f64) { (self.g.door_tools.charges, self.g.door_tools.bangs, self.g.door_tools.frags) }
    fn carries(&mut self, charges: f64, bangs: f64) { self.g.door_tools.charges = charges; self.g.door_tools.bangs = bangs; }
    fn struck(&self, gd: GuardId) -> bool { self.g.gd[gd].down_kind == Some(game::game::DownKind::Struck) }
    fn staggered(&self, gd: GuardId) -> bool { self.g.time < self.g.gd[gd].stagger_until }
    fn frame_at(&self, d: DoorId) -> [f64; 2] { let c = self.g.dr[d].frame_centre; [c[0], c[2]] }
    fn keep_broken(&self, d: DoorId) -> bool { game::doors::door_lock_broken(&self.g, d) }
    fn leaf_put_shut(&mut self, d: DoorId) {
        game::doors::door_set_angle(&mut self.g, d, 0.0);
        game::doors::door_set_vel(&mut self.g, d, 0.0);
        game::doors::door_set_latched(&mut self.g, d, true);
        game::doors::door_set_closing(&mut self.g, d, false);
        game::doors::door_set_pull_t(&mut self.g, d, 0.0);
    }
    fn leaf_put_open(&mut self, d: DoorId, rad: f64) {
        let s = game::doors::door_swing(&self.g, d);
        game::doors::door_set_latched(&mut self.g, d, false);
        game::doors::door_set_angle(&mut self.g, d, s * rad);
    }
    fn hung_toward(&mut self, d: DoorId, toward: [f64; 2]) {
        if !self.pull_face(d, toward) { let s = game::doors::door_swing(&self.g, d); game::doors::door_set_swing(&mut self.g, d, -s, None, None); }
    }
    fn post_of(&self, gd: GuardId) -> Option<[f64; 2]> { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).and_then(|a| self.g.tab.orders_assignment[a].post).map(|p| [p[0], p[2]]) }
    fn there_of(&self, gd: GuardId) -> String { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).map_or(String::new(), |a| self.g.tab.orders_assignment[a].there.clone()) }
    fn arrived(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).is_some_and(|a| self.g.tab.orders_assignment[a].arrived) }
    fn tools_k(&self) -> game::knobs::DoortoolsK { self.g.k.doortools.clone() }
    fn latched(&self, d: DoorId) -> bool { game::doors::door_latched(&self.g, d) }
    fn leaf_angle(&self, d: DoorId) -> f64 { game::doors::door_angle(&self.g, d) }
    fn leaf_vel(&self, d: DoorId) -> f64 { game::doors::door_vel(&self.g, d) }
    fn leaf_stays(&mut self, d: DoorId) { game::doors::door_set_quiet_t(&mut self.g, d, -3600.0); }
    fn restarted(&mut self) { game::game::game_restart_encounter(&mut self.g); }
    fn any_op(&self) -> bool { !self.g.door_tools.ops.is_empty() }
    fn fire_lasts(&self) -> f64 { self.g.k.fireline.fire_s }
    fn doors(&self) -> Vec<DoorId> { self.g.doors.list.clone() }
    fn door_name(&self, d: DoorId) -> String { self.g.dr[d].def.name.clone() }
    fn handle_place(&mut self, d: DoorId, side: f64) -> Option<([f64; 2], bool, bool)> { game::handleside::handle_spot(&mut self.g, d, side).map(|s| ([s.pos[0], s.pos[2]], s.pull, s.far.unwrap_or(false))) }
    fn op_is_can(&self, op: DoorOpId) -> bool { self.g.tab.doortools_door_op[op].kind == game::doortools::DoorToolKind::BangClear }
    fn can_out(&self, op: DoorOpId) -> Option<(ItemId, f64, bool)> { let c = self.g.tab.doortools_door_op[op].can?; Some((c, game::throwables::item_fuse(&self.g, c), game::throwables::item_fired(&self.g, c))) }
    fn can_now(&self, can: ItemId) -> (f64, bool) { (game::throwables::item_fuse(&self.g, can), game::throwables::item_fired(&self.g, can)) }
    fn gun_yaw(&self, gd: GuardId) -> f64 { self.g.ch[self.g.gd[gd].ch].aim_yaw }
    fn watch_of(&self, gd: GuardId) -> Option<[f64; 2]> { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).and_then(|a| self.g.tab.orders_assignment[a].watch).map(|p| [p[0], p[2]]) }
    fn floor_free(&self, at: [f64; 2]) -> bool { !game::scene_collision::nav_grid_is_blocked(&self.g, at[0], at[1]) }
    fn orders_given(&self) -> usize { self.g.orders.log.len() }
    fn menu_up(&self) -> Option<(Option<String>, usize)> { self.g.orders.menu.as_ref().map(|m| (m.page.clone(), m.rows.len())) }
    fn menu_opens(&mut self, at: [f64; 2], page: Option<&str>) {
        let t = game::orders::squad_orders_target_at(&mut self.g, [at[0], 0.0, at[1]]);
        assert!(game::orders::squad_orders_open(&mut self.g, Some(&t), None, None), "the menu did not open on {at:?}");
        if page.is_some() { game::orders::squad_orders_turn(&mut self.g, page, None); }
    }
    fn row_key(&self, id: &str) -> Option<f64> { self.g.orders.menu.as_ref()?.rows.iter().map(|&d| &self.g.tab.orders_order_def[d]).find(|r| r.id == id)?.key }
    fn sam_slot(&self) -> game::game::SlotKind { self.g.pl[self.g.player].slot }
    fn tap(&mut self, code: &str) { self.key_down(code); self.frame(); self.key_up(code); self.frame(); }
    fn on_face(&self, d: DoorId, along: f64, off: f64) -> [f64; 2] { let f = game::guards::frame_of(&self.g, d); [f.hx + f.ux * along + f.nx * off, f.hz + f.uz * along + f.nz * off] }
    fn quiet_pair(&mut self) {
        self.g.k.pairup.on = true;
        game::pairup::pair_up_apply(&mut self.g, None);
        self.g.k.silent.on = true;
        game::silent::set_silent(&mut self.g, true, None);
    }
    fn job(&self, gd: GuardId) -> Option<(bool, f64, Option<GuardId>)> { game::pairorders::pair_orders_job_of(&self.g, gd).map(|j| (j.set, j.t0, j.watcher)) }
    fn gripped(&mut self, gd: GuardId) -> bool { game::pairorders::pair_gripped(&mut self.g, gd) }
    fn order_on_foe(&mut self, id: &str, foe: GuardId) -> Option<String> {
        let a = self.at(foe);
        let t = game::orders::OrderTarget::Foe(game::orders::OrderTargetFoe { at: [a[0], 0.0, a[1]], foe });
        game::orders::squad_orders_give(&mut self.g, id, Some(&t), Some(game::orders::Who::All), None)
    }
    fn squad_rounds(&self) -> f64 { self.mates().iter().map(|&m| self.rounds_fired(m)).sum() }
    fn turned_to(&mut self, gd: GuardId, at: [f64; 2]) {
        let y = yaw_to(self.at(gd), at);
        let c = self.g.gd[gd].ch;
        self.g.ch[c].body_yaw = y;
        self.g.ch[c].aim_yaw = y;
    }
    fn order_on_word(&mut self, id: &str, at: [f64; 2], who: game::orders::Who) {
        let t = game::orders::squad_orders_target_at(&mut self.g, [at[0], 0.0, at[1]]);
        let o = game::orders::GiveOpts { on_word: Some(true), ..game::orders::GiveOpts::zeroed() };
        if let Some(why) = game::orders::squad_orders_give(&mut self.g, id, Some(&t), Some(who), Some(&o)) { panic!("'{id}' on the word was refused: {why}"); }
    }
    fn op_pods(&self, op: DoorOpId) -> Vec<[f64; 2]> { self.g.tab.doortools_door_op[op].pod.values().map(|p| [p.pos[0], p.pos[2]]).collect() }
    fn muzzle_down(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).is_some_and(|a| self.g.tab.orders_assignment[a].low) }
    fn light_on(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).is_some_and(|a| self.g.tab.orders_assignment[a].light.unwrap_or(false)) }
    fn callsign(&self, gd: GuardId) -> String { self.g.gd[gd].callsign.clone() }
    fn silent(&self) -> bool { game::silent::squad_silent(&self.g, &game::seats::here(&self.g)) }
    fn gun_walled(&mut self, gd: GuardId, reach: f64) -> bool {
        let (a, y) = (self.at(gd), self.gun_yaw(gd));
        game::scene_collision::segment_blocked(&mut self.g, [a[0], 1.3, a[1]], [a[0] + y.sin() * reach, 1.3, a[1] + y.cos() * reach], None, None, None)
    }
    fn prop_near(&self, at: [f64; 2]) -> PropId {
        let d = |p: PropId| { let q = self.prop_at(p); (q[0] - at[0]).hypot(q[1] - at[1]) };
        self.g.props.props.iter().copied().min_by(|a, b| d(*a).partial_cmp(&d(*b)).unwrap()).expect("the level has movable things")
    }
    fn prop_at(&self, prop: PropId) -> [f64; 2] { [game::props::prop_x(&self.g, prop), game::props::prop_z(&self.g, prop)] }
    fn prop_put(&mut self, prop: PropId, at: [f64; 2]) {
        game::props::prop_set_x(&mut self.g, prop, at[0]);
        game::props::prop_set_z(&mut self.g, prop, at[1]);
        game::props::prop_set_vx(&mut self.g, prop, 0.0);
        game::props::prop_set_vz(&mut self.g, prop, 0.0);
        game::props::prop_set_w(&mut self.g, prop, 0.0);
    }
    fn order_floor(&mut self, id: &str, at: [f64; 2], who: game::orders::Who) {
        let t = game::orders::OrderTarget::Floor(game::orders::OrderTargetFloor { at: [at[0], 0.0, at[1]], ..game::orders::OrderTargetFloor::zeroed() });
        if let Some(why) = game::orders::squad_orders_give(&mut self.g, id, Some(&t), Some(who), None) { panic!("'{id}' on the floor at {at:?} was refused: {why}"); }
    }
    fn leaf_tip(&self, d: DoorId) -> [f64; 2] { let f = game::guards::frame_of(&self.g, d); let (c, a) = (self.g.dr[d].def.closed_dir, game::doors::door_angle(&self.g, d)); [f.hx + (c + a).cos() * f.w, f.hz + (c + a).sin() * f.w] }
    fn hinge(&self, d: DoorId) -> [f64; 2] { let f = game::guards::frame_of(&self.g, d); [f.hx, f.hz] }
    fn op_front_men(&self, op: DoorOpId) -> Vec<GuardId> { let o = &self.g.tab.doortools_door_op[op]; o.men.iter().copied().filter(|&m| self.state(m) != GuardState::Dead && !o.rear.has(m)).collect() }
    fn boot_spots(&mut self, d: DoorId, side: f64) -> ([f64; 2], [f64; 2], [f64; 2]) {
        let xz = |p: [f64; 3]| [p[0], p[2]];
        let w = self.door_w(d);
        let f = game::guards::frame_of(&self.g, d);
        let from = [f.hx + f.ux * (w - 0.25) + f.nx * side, 0.0, f.hz + f.uz * (w - 0.25) + f.nz * side];
        let sq = game::doors::doors_kick_spot(&mut self.g, d, from);
        let s = game::doors::doors_kick_spot_side(&mut self.g, d, sq.pos);
        let now = game::doortools::door_tools_boot_spot(&mut self.g, d, side);
        (xz(now.pos), xz(sq.pos), xz(s.pos))
    }
    fn glass_lines(&mut self, d: DoorId, side: f64, at: [f64; 2]) -> f64 { game::orders::squad_orders_glass_eyes(&mut self.g, d, side, [at[0], 0.0, at[1]], None) }
    fn glass_bar(&self) -> (f64, f64) { (self.g.k.glasspen.stack_eyes, self.g.k.glasspen.stack_least_k) }
    fn order_on_face(&mut self, id: &str, d: DoorId, from: [f64; 2]) -> Option<String> {
        let side = self.face(d, from);
        let c = self.frame_at(d);
        let t = game::orders::OrderTarget::Door(game::orders::OrderTargetDoor { at: [c[0], 0.0, c[1]], door: d, side, ..game::orders::OrderTargetDoor::zeroed() });
        game::orders::squad_orders_give(&mut self.g, id, Some(&t), Some(game::orders::Who::All), None)
    }
    fn picture_shows_open(&self, d: DoorId) -> bool { !game::fogofwar::fog_of_war_door_held(&self.g, d) }
    fn fog_on(&mut self) { self.g.fog_of_war.on = true; }
    fn fog_off(&mut self) { self.g.fog_of_war.on = false; }
    fn last_order(&self) -> Option<(String, Vec<String>)> { self.g.orders.log.last().map(|r| (r.order.clone(), r.who.clone())) }
    fn key_tapped_on(&mut self, at: [f64; 3]) {
        self.cursor_on(at);
        self.run(3.0 / 60.0);
        let key = self.g.k.cmdring.key.clone();
        self.key_down(&key);
        self.run(2.0 / 60.0);
        self.key_up(&key);
        self.run(6.0 / 60.0);
    }
    fn choice_on(&mut self, who: game::orders::Who) { self.g.orders.who = who; }
    fn corner_in_his_way(&mut self, gd: GuardId, at: [f64; 2]) {
        let a = game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).expect("his order");
        let c = [at[0], 0.0, at[1]];
        self.g.tab.orders_assignment[a].via = vec![c];
        self.g.tab.orders_assignment[a].goal = c;
    }
    fn fight_has(&mut self, gd: GuardId) { game::squad::end_guard_script(&mut self.g, gd, Some(game::squad::EndGuardScriptState::Alert)); }
    fn order_waits_on_fight(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).is_some_and(|a| self.g.tab.orders_assignment[a].yield_t >= 0.0) }
    fn rescue_log(&self) -> Vec<(f64, String)> { self.g.rescue.log.iter().map(|l| (l.t, l.what.clone())).collect() }
    fn rescue_t0(&self) -> f64 { self.g.rescue.t0 }
    fn rescue_clock(&self) -> f64 { self.g.rescue.resolve }
    fn rescue_lost(&self) -> Option<bool> { self.g.rescue.result.as_ref().map(|r| r.state == game::rescue::RescueResultState::Lost) }
    fn rescue_men(&self) -> Vec<GuardId> { self.g.rescue.hostiles.clone() }
    fn rescue_hostages(&self) -> Vec<GuardId> { self.g.rescue.hostages.clone() }
    fn rescue_blind(&self) -> f64 { self.g.rescue.blind_rounds }
    fn rescue_on(&mut self, first: f64, second: f64, set: f64) {
        let k = &mut self.g.k.rescue;
        (k.resolve_spread, k.resolve_secs, k.second_hostage_secs, k.set_secs) = (0.0, first, second, set);
        game::rescue::rescue_restage(&mut self.g);
    }
    fn fight_notes(&self, name: &str) -> Vec<String> { self.g.fight.log.iter().filter(|n| n.who == name).map(|n| n.what.clone()).collect() }
    fn rescue_aim(&self) -> f64 { self.g.k.rescue.aim_secs }
    fn dazzled_for(&mut self, gd: GuardId, secs: f64) { self.g.gd[gd].dazzled_until = self.g.time + secs; }
    fn blast_tears(&mut self, at: [f64; 2]) -> Vec<DoorId> { let r = self.g.k.doorblast.tear_m.frag; game::doors::doors_blast(&mut self.g, [at[0], 1.0, at[1]], r, -1.0) }
    fn lane_half(&self, along: f64) -> f64 { let (ff, fl) = (&self.g.k.friendlyfire, &self.g.k.fireline); ff.body_m + fl.clear_m + along * ff.cone_k * fl.spread }
    fn sam_rounds(&self) -> f64 { self.g.fa[self.g.pl[self.g.player].rifle].shots_fired }
    fn file_place(&self, gd: GuardId) -> Option<f64> { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).map(|a| self.g.tab.orders_assignment[a].follow) }
    fn sam_clicks(&mut self) { let pl = self.g.player; self.g.pl[pl].fire_cd = 0.0; self.press(game_start::play::Button::Left); }
    fn in_doorway(&self, d: DoorId, at: [f64; 2]) -> bool { let (a, o) = (self.along(d, at), self.off(d, at, 1.0)); a > -0.05 && a < self.door_w(d) + 0.05 && o.abs() < 0.3 }
    fn in_sweep(&self, d: DoorId, at: [f64; 2]) -> bool { game::doors::door_swing_sweeps(&game::doors::doors_swing_of(&self.g, d), at[0], at[1], Some(BODY_R)) }
    fn from_handle(&self, d: DoorId, at: [f64; 2]) -> f64 { let f = game::guards::frame_of(&self.g, d); let (a, o) = (self.along(d, at), self.off(d, at, 1.0)); ((a - (f.w - HANDLE_IN)).powi(2) + o * o).sqrt() }
    fn door_rows(&self, d: DoorId) -> usize { self.g.nearby.iter().filter(|r| r.door == Some(d)).count() }
    fn slab_rest(&mut self, d: DoorId, side: f64) -> Option<(bool, f64)> { let s = game::doors::door_slab(&mut self.g, d)?; let at = [s.p[0], s.p[2]]; Some((s.rest, self.off(d, at, side))) }
    fn clear_line(&mut self, a: [f64; 2], b: [f64; 2]) -> bool { !game::scene_collision::segment_blocked(&mut self.g, [a[0], 1.1, a[1]], [b[0], 1.1, b[1]], None, None, None) }
    fn first_over(&self, op: DoorOpId) -> Option<GuardId> { let c = &self.g.tab.doortools_door_op[op].crossed; c.keys().into_iter().min_by(|a, b| c.get(*a).unwrap().partial_cmp(c.get(*b).unwrap()).unwrap()) }
    fn seen_of_room(&mut self, gd: GuardId, at: [f64; 2]) -> f64 {
        let Some(room) = game::roomfight::room_fight_room_at(&mut self.g, [at[0], 0.0, at[1]]) else { return 0.0 };
        let cells: Vec<f64> = self.g.tab.roomfight_room[room].cells.iter().map(|c| c.0).collect();
        game::floorseen::floor_seen_count(&self.g, &game::seats::here(&self.g), gd, &cells, self.g.time - self.g.k.doortools.lead_seen_secs)
    }
}

/// how far apart two ways are, radians
pub fn off_by(a: f64, b: f64) -> f64 { let d = (a - b).rem_euclid(std::f64::consts::TAU); d.min(std::f64::consts::TAU - d) }
/// the yaw from one place to another
pub fn yaw_to(a: [f64; 2], b: [f64; 2]) -> f64 { (b[0] - a[0]).atan2(b[1] - a[1]) }
