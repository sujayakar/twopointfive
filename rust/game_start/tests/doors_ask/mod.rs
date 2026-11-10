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
    /// the order menu as it stands, if one is up: how many rows
    fn menu_up(&self) -> Option<usize>;
    /// the menu opens on what stands at `at`, as its key does
    fn menu_opens(&mut self, at: [f64; 2]);
    /// the wedge the row of that id is on
    fn wedge(&self, id: &str) -> Option<usize>;
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
    /// STAGING: no man abroad to meet the pair, and the door's man inside
    fn rescue_quiet_floor(&mut self);
    /// is he on his way out of a room he has given up
    fn falls_back(&self, name: &str) -> bool;
    /// how long he lines up on her
    fn rescue_aim(&self) -> f64;
    /// STAGING: a flash has his eyes for so long
    fn dazzled_for(&mut self, gd: GuardId, secs: f64);
    /// is the room of that id on the room clear's list cleared
    fn room_cleared(&self, id: &str) -> bool;
    /// the defenders that room was dealt
    fn room_men(&self, id: &str) -> Vec<GuardId>;
    /// who the room fight has this man's gun on
    fn gun_on(&self, gd: GuardId) -> Option<GuardId>;
    /// STAGING, each frame: a man the probe stands where it wants him, blind and not to be killed; nobody's round goes; the floor is hot
    fn puppet(&mut self, foe: GuardId, at: [f64; 2], yaw: f64);
    /// STAGING: this man sees nothing for now
    fn blind_now(&mut self, gd: GuardId);
    /// an order to these men by the menu's call
    fn order_men(&mut self, id: &str, at: [f64; 2], men: &[GuardId]) -> Option<String>;
    /// his script as his order wrote it: what his arms and body are told, and is the fight's overlay on him
    fn script_says(&self, gd: GuardId) -> Option<(Option<game::character::UpperMode>, Option<game::character::Stance>, Option<f64>, bool)>;
    /// what his arms are doing as shown
    fn arms_shown(&self, gd: GuardId) -> game::character::UpperMode;
    /// is that door locked against a man with no key from that place, and with the key
    fn locked_out(&self, d: DoorId, from: [f64; 2]) -> (bool, bool);
    /// a hand works the door from that place, with the key or without
    fn door_used(&mut self, d: DoorId, from: [f64; 2], key: bool);
    fn lock_on(&self, d: DoorId) -> bool;
    /// the face of the door its lock is on
    fn key_face(&self, d: DoorId) -> f64;
    /// the team's walking pace
    fn team_pace(&self) -> f64;
    /// Sam can be hurt again
    fn sam_hurtable(&mut self);
    /// is the door locked as it stands
    fn door_locked_now(&self, d: DoorId) -> bool;
    /// the man whose hands are on a lock
    fn hands_on_lock(&self) -> Option<GuardId>;
    /// how many times a line with these words was said, and whether every one went over the net
    fn said_count(&self, words: &str) -> (usize, bool);
    /// is the silent squad compromised
    fn compromised(&self) -> bool;
    /// a pinned man let go where he stands, his head his own again
    fn let_go(&mut self, m: GuardId);
    /// Sam's arm settled round a man, from behind him
    fn sam_holds(&mut self, m: GuardId) -> bool;
    /// is a man in a standoff with a pair, and has he a clean line
    fn in_standoff(&self, m: GuardId) -> Option<bool>;
    /// the man he saw held
    fn saw_held(&self, m: GuardId) -> Option<String>;
    /// how far the floor has been raised
    fn escalation(&self) -> f64;
    /// the choke chosen on the row, and the key tapped
    fn choke_him(&mut self);
    /// a point in every room of the level, as the middle-most floor of the room, with its room's cells
    fn room_points(&mut self) -> Vec<(f64, [f64; 2])>;
    /// the marks a 'move there' would deal `n` men at `at`, come from `from`: where each stands and what his gun is on
    fn fit_marks(&mut self, at: [f64; 2], from: [f64; 2], n: f64) -> Option<Vec<([f64; 2], Option<[f64; 2]>, f64)>>;
    /// the marks a plain line abreast would be
    fn line_marks(&mut self, at: [f64; 2], from: [f64; 2], n: f64) -> Vec<[f64; 2]>;
    /// is a mark in a door's frame or on floor nobody can stand on
    fn no_place_to_stand(&mut self, at: [f64; 2]) -> bool;
    /// how far the eye sees along a bearing, up to `most`
    fn run_along(&mut self, at: [f64; 2], yaw: f64, most: f64) -> f64;
    /// the marks of a charge's stand-off
    fn standoff_marks(&mut self, d: DoorId, side: f64, men: &[GuardId]) -> Vec<[f64; 2]>;
    /// the threshold as a mission's own order gives it, to go on in by itself
    fn threshold_given(&mut self, at: [f64; 2]);
    /// what the door's mark on the plan says while a stack waits on it: its key, and would it go now
    fn stack_mark(&mut self, d: DoorId) -> Option<(Option<String>, bool)>;
    /// how many a door order has left to spend, and the cap on the word's wait for the cuts
    fn word_cap_put(&mut self, secs: f64);
    /// which tool a door order is, and when each of its beats came
    fn op_kind(&self, op: DoorOpId) -> game::doortools::DoorToolKind;
    fn beat_times(&self, op: DoorOpId) -> Vec<f64>;
    /// is he walking backwards facing his way, and is his next step of his path behind the way he is moving
    fn walks_backwards(&self, m: GuardId) -> bool;
    fn walks_away_from_his_path(&self, m: GuardId) -> bool;
    /// how many leaves have been gone round
    fn leaves_gone_round(&self) -> f64;
    /// seconds of blindness and of doubling over a man has left
    fn blind_secs(&self, m: GuardId) -> f64;
    fn fold_secs(&self, m: GuardId) -> f64;
    /// STAGING: a man of the other side is a quiet mission's staff
    fn is_staff(&mut self, m: GuardId);
    /// the points an entry by a door is dealt, before and at a threshold
    fn entry_pods(&mut self, d: DoorId, side: f64, n: f64) -> Vec<[f64; 2]>;
    fn shallow_pods(&mut self, d: DoorId, side: f64, n: f64) -> Vec<[f64; 2]>;
    /// the stack the game deals a door: by 'stack up', by the door tools
    fn stack_marks(&mut self, d: DoorId, side: f64, men: &[GuardId]) -> Vec<[f64; 2]>;
    fn tool_marks(&mut self, d: DoorId, side: f64, men: &[GuardId]) -> Vec<[f64; 2]>;
    /// the walls' own marks, with glass not counted
    fn walls_own_marks(&mut self, d: DoorId, side: f64, men: &[GuardId]) -> Vec<[f64; 2]>;
    /// where an item is now
    fn item_at(&self, it: ItemId) -> [f64; 2];
    /// Shift and the squad's key tapped as a hand does it, the cursor on the thing
    fn shift_tap(&mut self, at: [f64; 3]);
    /// Shift and the key kept down: did the ring come up
    fn shift_held(&mut self, at: [f64; 3]) -> bool;
    /// a wedge of the ring taken by its digit and every key let go: its words
    fn wedge_taken(&mut self, words: Option<&str>) -> String;
    fn keys_let_go(&mut self);
    /// the squad's smoke left, the smoke cans in the world: where, and is it venting
    fn smoke_left(&self) -> f64;
    fn smoke_cans(&mut self) -> Vec<([f64; 2], bool)>;
    /// the kit's cards set to carry these cans, by card, and the game restarted as the page does on the kit
    fn cards_carry(&mut self, cards: &[(usize, f64)]);
    /// a round fired dead straight from a man's shoulder height at a place, whoever is in the way (a pinned man's pose is not the point)
    fn round_from(&mut self, m: GuardId, at: [f64; 3]);
    /// is Sam down, and what he has left
    fn sam_down_hits(&self) -> (bool, f64);
    /// how a man went down, and by whom
    fn down_how(&self, m: GuardId) -> Option<(game::game::DownKind, game::game::DownBy)>;
    /// the rescue's mode, and its result
    fn rescue_free(&self) -> bool;
    fn rescue_jobs(&self) -> usize;
    fn cans_left(&self) -> f64;
    fn frags_left(&self) -> f64;
    /// who has a throw on a spot now, and who after a walk: the man and where he would stand
    fn throw_now(&mut self, men: &[GuardId], at: [f64; 2]) -> bool;
    fn throw_after_walk(&mut self, men: &[GuardId], at: [f64; 2]) -> Option<(GuardId, [f64; 2])>;
    /// frag cans in the world: where they came to rest
    fn frags_out(&self) -> Vec<[f64; 2]>;
    /// the shotgun's stats: pellets into men, men dropped, men peppered
    fn shell_stats(&self) -> (i32, i32, i32);
    /// a shell from a man's bore at `from`, at a place, its spread as a man's hands make it: did it take him
    fn shell_from(&mut self, m: GuardId, from: [f64; 2], at: [f64; 3], spread: f64);
    fn shotgun_rule(&mut self) -> (f64, f64, bool, Vec<bool>);
    /// has a man a line from his eyes to the doorway as he sees it
    fn sees_doorway(&mut self, m: GuardId, d: DoorId) -> bool;
    /// has a door order been and gone on this door
    fn op_ended_on(&self, d: DoorId) -> bool;
    /// is the squad's ring up, who is it for, what it says under it, its wedges
    fn ring_up(&self) -> bool;
    fn ring_part(&self) -> Vec<GuardId>;
    fn ring_caption(&mut self) -> String;
    fn ring_wedges(&mut self) -> Vec<(f64, String)>;
    /// STAGING: a ring switch off, for a control
    fn goal_half_off(&mut self);
    fn twice_off(&mut self);
    /// how long a second tap of the key counts as one
    fn twice_secs(&self) -> f64;
    /// the panes on a door's own wall beside its frame: their span along the wall from the hinge
    fn panes_by(&self, d: DoorId) -> Vec<(f64, f64)>;
    /// a run with something asked each frame
    fn run_each(&mut self, secs: f64, each: impl FnMut(&mut Play));
    /// a threshold's two jamb men, and is the door called held
    fn thr_jamb(&self, op: DoorOpId) -> Vec<GuardId>;
    fn thr_told(&self, op: DoorOpId) -> bool;
    /// the cuts he was given: how many, the one he is on, its place, his cuts done
    fn cut_of(&self, op: DoorOpId, m: GuardId) -> Option<(usize, usize, [f64; 2], bool)>;
    /// is his script leaning him, how low he is, how far his head is past his hips along a line
    fn leaning(&self, m: GuardId) -> bool;
    fn knee_of(&self, m: GuardId) -> f64;
    fn head_past_hips(&self, m: GuardId, along: [f64; 2]) -> f64;
    /// nothing of a blast takes the squad, for a rule about where they go and not who it kills
    fn men_spared(&mut self);
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
    fn door_shut(&self, d: DoorId) -> bool { !self.g.dr[d].leaf.off && self.g.dr[d].leaf.latched && self.g.dr[d].leaf.angle.abs() < 0.05 }
    fn door_gone(&self, d: DoorId) -> bool { self.g.dr[d].leaf.off }
    fn tool_stats(&self) -> DoorToolsStats { self.g.sets[self.g.seat].door_tools.stats }
    fn ops_on(&self, d: DoorId) -> Vec<DoorOpId> { self.g.sets[self.g.seat].door_tools.ops.iter().copied().filter(|&o| self.g.tab.doortools_door_op[o].door == d).collect() }
    fn op_phase(&self, op: DoorOpId) -> Phase { self.g.tab.doortools_door_op[op].phase }
    fn op_lead(&self, op: DoorOpId) -> GuardId { self.g.tab.doortools_door_op[op].lead }
    fn op_beats(&self, op: DoorOpId) -> Vec<String> { self.g.tab.doortools_door_op[op].beats.iter().map(|b| b.what.clone()).collect() }
    fn doing(&self, gd: GuardId) -> Option<String> { game::orders::squad_orders_of(&game::seats::here(&self.g), gd).map(|a| self.g.tab.orders_assignment[a].order.clone()) }
    fn order_door(&self, gd: GuardId) -> Option<DoorId> { game::orders::squad_orders_of(&game::seats::here(&self.g), gd).and_then(|a| self.g.tab.orders_assignment[a].door) }
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
        self.g.dr[d].leaf.locked = true;
        self.g.dr[d].leaf.pick = 0.0;
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
    fn op_ended(&self, op: DoorOpId) -> Option<String> { self.g.tab.doortools_door_op[op].ended.map(|e| e.as_str().to_string()) }
    fn last_op(&self, d: DoorId) -> Option<DoorOpId> { self.g.sets[self.g.seat].door_tools.ops.iter().chain(self.g.sets[self.g.seat].door_tools.done.iter()).copied().filter(|&o| self.g.tab.doortools_door_op[o].door == d).max_by(|a, b| self.g.tab.doortools_door_op[*a].t0.partial_cmp(&self.g.tab.doortools_door_op[*b].t0).unwrap()) }
    fn op_men(&self, op: DoorOpId) -> Vec<GuardId> { self.g.tab.doortools_door_op[op].men.clone() }
    fn op_bang_at(&self, op: DoorOpId) -> Option<[f64; 2]> { self.g.tab.doortools_door_op[op].bang_at.map(|p| [p[0], p[2]]) }
    fn left(&self) -> (f64, f64, f64) { (self.g.sets[self.g.seat].door_tools.charges, self.g.sets[self.g.seat].door_tools.bangs, self.g.sets[self.g.seat].door_tools.frags) }
    fn carries(&mut self, charges: f64, bangs: f64) { self.g.sets[self.g.seat].door_tools.charges = charges; self.g.sets[self.g.seat].door_tools.bangs = bangs; }
    fn struck(&self, gd: GuardId) -> bool { self.g.gd[gd].down_kind == Some(game::game::DownKind::Struck) }
    fn staggered(&self, gd: GuardId) -> bool { self.g.time < self.g.cond[self.g.gd[gd].ch].fold_until }
    fn frame_at(&self, d: DoorId) -> [f64; 2] { let c = self.g.dr[d].frame_centre; [c[0], c[2]] }
    fn keep_broken(&self, d: DoorId) -> bool { self.g.dr[d].leaf.lock_broken }
    fn leaf_put_shut(&mut self, d: DoorId) {
        self.g.dr[d].leaf.angle = 0.0;
        self.g.dr[d].leaf.vel = 0.0;
        self.g.dr[d].leaf.latched = true;
        self.g.dr[d].leaf.closing = false;
        self.g.dr[d].leaf.pull_t = 0.0;
    }
    fn leaf_put_open(&mut self, d: DoorId, rad: f64) {
        let s = game::doors::door_swing(&self.g, d);
        self.g.dr[d].leaf.latched = false;
        self.g.dr[d].leaf.angle = s * rad;
    }
    fn hung_toward(&mut self, d: DoorId, toward: [f64; 2]) {
        if !self.pull_face(d, toward) { let s = game::doors::door_swing(&self.g, d); game::doors::door_set_swing(&mut self.g, d, -s, None); }
    }
    fn post_of(&self, gd: GuardId) -> Option<[f64; 2]> { game::orders::squad_orders_of(&game::seats::here(&self.g), gd).and_then(|a| self.g.tab.orders_assignment[a].post).map(|p| [p[0], p[2]]) }
    fn there_of(&self, gd: GuardId) -> String { game::orders::squad_orders_of(&game::seats::here(&self.g), gd).map_or(String::new(), |a| self.g.tab.orders_assignment[a].there.words.clone()) }
    fn arrived(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&game::seats::here(&self.g), gd).is_some_and(|a| self.g.tab.orders_assignment[a].arrived) }
    fn tools_k(&self) -> game::knobs::DoortoolsK { self.g.k.doortools.clone() }
    fn latched(&self, d: DoorId) -> bool { self.g.dr[d].leaf.latched }
    fn leaf_angle(&self, d: DoorId) -> f64 { self.g.dr[d].leaf.angle }
    fn leaf_vel(&self, d: DoorId) -> f64 { self.g.dr[d].leaf.vel }
    fn leaf_stays(&mut self, d: DoorId) { self.g.dr[d].leaf.quiet_t = -3600.0; }
    fn restarted(&mut self) { game::savestate::control::restart(&mut self.g); }
    fn any_op(&self) -> bool { !self.g.sets[self.g.seat].door_tools.ops.is_empty() }
    fn fire_lasts(&self) -> f64 { self.g.k.fireline.fire_s }
    fn doors(&self) -> Vec<DoorId> { self.g.doors.list.clone() }
    fn door_name(&self, d: DoorId) -> String { self.g.dr[d].def.name.clone() }
    fn handle_place(&mut self, d: DoorId, side: f64) -> Option<([f64; 2], bool, bool)> { game::handleside::handle_spot(&mut self.g, d, side).map(|s| ([s.pos[0], s.pos[2]], s.pull, s.far.unwrap_or(false))) }
    fn op_is_can(&self, op: DoorOpId) -> bool { self.g.tab.doortools_door_op[op].kind == game::doortools::DoorToolKind::BangClear }
    fn can_out(&self, op: DoorOpId) -> Option<(ItemId, f64, bool)> { let c = self.g.tab.doortools_door_op[op].can?; Some((c, self.g.tab.item[c].fuse, self.g.tab.item[c].fired)) }
    fn can_now(&self, can: ItemId) -> (f64, bool) { (self.g.tab.item[can].fuse, self.g.tab.item[can].fired) }
    fn gun_yaw(&self, gd: GuardId) -> f64 { self.g.ch[self.g.gd[gd].ch].aim_yaw }
    fn watch_of(&self, gd: GuardId) -> Option<[f64; 2]> { game::orders::squad_orders_of(&game::seats::here(&self.g), gd).and_then(|a| self.g.tab.orders_assignment[a].watch).map(|p| [p[0], p[2]]) }
    fn floor_free(&self, at: [f64; 2]) -> bool { !game::scene_collision::nav_grid_is_blocked(&self.g, at[0], at[1]) }
    fn orders_given(&self) -> usize { self.g.sets[self.g.seat].orders.log.len() }
    fn menu_up(&self) -> Option<usize> { self.g.sets[self.g.seat].orders.menu.as_ref().map(|m| m.rows.len()) }
    fn menu_opens(&mut self, at: [f64; 2]) {
        let t = game::orders::squad_orders_target_at(&mut self.g, [at[0], 0.0, at[1]]);
        assert!(game::orders::squad_orders_open(&mut self.g, Some(&t), None), "the menu did not open on {at:?}");
        self.ring_over_menu();
    }
    fn wedge(&self, id: &str) -> Option<usize> {
        let s = &self.g.sets[self.g.seat];
        let m = s.orders.menu.as_ref()?;
        let i = m.rows.iter().position(|&d| self.g.tab.orders_order_def[d].id == id)?;
        game::orders::squad_orders_seats(&self.g, m, s.orders.cmd_ring).row.iter().position(|&r| r == i as f64)
    }
    fn sam_slot(&self) -> game::game::SlotKind { self.g.pl[self.g.player].slot }
    fn tap(&mut self, code: &str) { self.key_down(code); self.frame(); self.key_up(code); self.frame(); }
    fn on_face(&self, d: DoorId, along: f64, off: f64) -> [f64; 2] { let f = game::guards::frame_of(&self.g, d); [f.hx + f.ux * along + f.nx * off, f.hz + f.uz * along + f.nz * off] }
    fn quiet_pair(&mut self) {
        self.g.k.pairup.on = true;
        game::pairup::pair_up_apply(&mut self.g, None);
        self.g.k.silent.on = true;
        game::silent::set_silent(&mut self.g, true);
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
    fn op_pods(&self, op: DoorOpId) -> Vec<[f64; 2]> { self.g.tab.doortools_door_op[op].pod.values().map(|p| [p.pos[0], p.pos[2]]).collect() }
    fn muzzle_down(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&game::seats::here(&self.g), gd).is_some_and(|a| self.g.tab.orders_assignment[a].low) }
    fn light_on(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&game::seats::here(&self.g), gd).is_some_and(|a| self.g.tab.orders_assignment[a].light.unwrap_or(false)) }
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
    fn prop_at(&self, prop: PropId) -> [f64; 2] { [self.g.tab.props_prop[prop].x, self.g.tab.props_prop[prop].z] }
    fn prop_put(&mut self, prop: PropId, at: [f64; 2]) {
        game::props::prop_move_to(&mut self.g, prop, at[0], at[1]);
        self.g.tab.props_prop[prop].vx = 0.0;
        self.g.tab.props_prop[prop].vz = 0.0;
        self.g.tab.props_prop[prop].w = 0.0;
    }
    fn order_floor(&mut self, id: &str, at: [f64; 2], who: game::orders::Who) {
        let t = game::orders::OrderTarget::Floor(game::orders::OrderTargetFloor { at: [at[0], 0.0, at[1]], ..game::orders::OrderTargetFloor::default() });
        if let Some(why) = game::orders::squad_orders_give(&mut self.g, id, Some(&t), Some(who), None) { panic!("'{id}' on the floor at {at:?} was refused: {why}"); }
    }
    fn leaf_tip(&self, d: DoorId) -> [f64; 2] { let f = game::guards::frame_of(&self.g, d); let (c, a) = (self.g.dr[d].def.closed_dir, self.g.dr[d].leaf.angle); [f.hx + (c + a).cos() * f.w, f.hz + (c + a).sin() * f.w] }
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
        let t = game::orders::OrderTarget::Door(game::orders::OrderTargetDoor { at: [c[0], 0.0, c[1]], door: d, side, ..game::orders::OrderTargetDoor::default() });
        game::orders::squad_orders_give(&mut self.g, id, Some(&t), Some(game::orders::Who::All), None)
    }
    fn picture_shows_open(&self, d: DoorId) -> bool { !game::fogofwar::fog_of_war_door_held(&self.g, d) }
    fn fog_on(&mut self) { self.g.fog_of_war.on = true; }
    fn fog_off(&mut self) { self.g.fog_of_war.on = false; }
    fn last_order(&self) -> Option<(String, Vec<String>)> { self.g.sets[self.g.seat].orders.log.last().map(|r| (r.order.clone(), r.who.clone())) }
    fn key_tapped_on(&mut self, at: [f64; 3]) {
        self.cursor_on(at);
        self.run(3.0 / 60.0);
        let key = self.g.k.cmdring.key.clone();
        self.key_down(&key);
        self.run(2.0 / 60.0);
        self.key_up(&key);
        self.run(6.0 / 60.0);
    }
    fn choice_on(&mut self, who: game::orders::Who) { self.g.sets[self.g.seat].orders.who = who; }
    fn corner_in_his_way(&mut self, gd: GuardId, at: [f64; 2]) {
        let a = game::orders::squad_orders_of(&game::seats::here(&self.g), gd).expect("his order");
        let c = [at[0], 0.0, at[1]];
        self.g.tab.orders_assignment[a].via = vec![c];
        self.g.tab.orders_assignment[a].goal = c;
    }
    fn fight_has(&mut self, gd: GuardId) { game::squad::end_guard_script(&mut self.g, gd, Some(game::game::GuardState::Alert)); }
    fn order_waits_on_fight(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&game::seats::here(&self.g), gd).is_some_and(|a| self.g.tab.orders_assignment[a].yield_t >= 0.0) }
    fn rescue_log(&self) -> Vec<(f64, String)> { self.g.rescue.log.iter().map(|l| (l.t, l.what.clone())).collect() }
    fn rescue_t0(&self) -> f64 { self.g.rescue.t0 }
    fn rescue_clock(&self) -> f64 { self.g.rescue.resolve }
    fn rescue_lost(&self) -> Option<bool> { self.g.rescue.result.as_ref().map(|r| r.state == game::rescue::RescueResultState::Lost) }
    fn rescue_men(&self) -> Vec<GuardId> { self.g.staged.foes.clone() }
    fn rescue_hostages(&self) -> Vec<GuardId> { self.g.staged.captives.clone() }
    fn rescue_blind(&self) -> f64 { self.g.rescue.blind_rounds }
    fn rescue_quiet_floor(&mut self) { self.g.k.rescue.roamers = 0.0; self.g.k.rescue.door_man = game::knobs::RescueKDoorMan::Inside; }
    fn rescue_on(&mut self, first: f64, second: f64, set: f64) {
        let k = &mut self.g.k.rescue;
        (k.resolve_secs, k.second_hostage_secs, k.set_secs) = (first, second, set);
        game::savestate::control::restart(&mut self.g);
    }
    fn falls_back(&self, name: &str) -> bool {
        self.g.fight.defs.iter().any(|&d| { let r = &self.g.tab.roomfight_def[d]; self.g.gd[r.gd].callsign == name && game::roomfight::on_leg(&self.g, d) && r.move_kind == game::roomfight::DefMoveKind::Fallback })
    }
    fn rescue_aim(&self) -> f64 { self.g.k.rescue.aim_secs }
    fn dazzled_for(&mut self, gd: GuardId, secs: f64) { self.g.cond[self.g.gd[gd].ch].blind_until = self.g.time + secs; }
    fn room_cleared(&self, id: &str) -> bool { self.g.staged.rooms.iter().find(|r| r.def.id == id).is_some_and(|r| r.cleared) }
    fn room_men(&self, id: &str) -> Vec<GuardId> { self.g.staged.rooms.iter().find(|r| r.def.id == id).map_or(Vec::new(), |r| r.defenders.clone()) }
    fn gun_on(&self, gd: GuardId) -> Option<GuardId> { self.g.fight.ops.iter().map(|&o| &self.g.tab.roomfight_op[o]).find(|o| o.gd == gd).and_then(|o| o.tgt) }
    fn puppet(&mut self, foe: GuardId, at: [f64; 2], yaw: f64) {
        self.pin_man(foe, at, yaw, GuardState::Patrol, 0.0);
        let t = self.g.time;
        self.g.cond[self.g.gd[foe].ch].blind_until = t + 9.0;
        self.g.gd[foe].god = true;
        for o in self.g.fight.ops.clone() { self.g.tab.roomfight_op[o].ready_at = t + 1.0; }
        self.g.fight.hot_t = t;
        for m in self.mates() { let _c = self.g.gd[m].ch; self.g.gd[foe].god = true; }
    }
    fn blind_now(&mut self, gd: GuardId) { self.g.cond[self.g.gd[gd].ch].blind_until = self.g.time + 9.0; }
    fn order_men(&mut self, id: &str, at: [f64; 2], men: &[GuardId]) -> Option<String> {
        let t = game::orders::squad_orders_target_at(&mut self.g, [at[0], 0.0, at[1]]);
        let o = game::orders::GiveOpts { men: Some(men.to_vec()), ..game::orders::GiveOpts::default() };
        game::orders::squad_orders_give(&mut self.g, id, Some(&t), None, Some(&o))
    }
    fn script_says(&self, gd: GuardId) -> Option<(Option<game::character::UpperMode>, Option<game::character::Stance>, Option<f64>, bool)> { self.g.gd[gd].script.as_ref().map(|s| (s.upper, s.stance, s.face, s.over.is_some())) }
    fn arms_shown(&self, gd: GuardId) -> game::character::UpperMode { self.g.ch[self.g.gd[gd].ch].anim.upper }
    fn locked_out(&self, d: DoorId, from: [f64; 2]) -> (bool, bool) {
        let at = [from[0], 0.0, from[1]];
        let mut keys = game::js::JsSet::new();
        keys.add(self.g.dr[d].lock_id.clone());
        (game::doors::doors_locked_out(&self.g, d, at, None), game::doors::doors_locked_out(&self.g, d, at, Some(&keys)))
    }
    fn door_used(&mut self, d: DoorId, from: [f64; 2], key: bool) {
        let mut keys = game::js::JsSet::new();
        if key { keys.add(self.g.dr[d].lock_id.clone()); }
        let who = self.g.ch[self.g.pl[self.g.player].ch].id;
        game::doors::doors_use(&mut self.g, d, [from[0], 0.0, from[1]], who, Some(&keys));
    }
    fn lock_on(&self, d: DoorId) -> bool { self.g.dr[d].leaf.locked }
    fn key_face(&self, d: DoorId) -> f64 { self.g.dr[d].key_side }
    fn team_pace(&self) -> f64 { self.g.k.pace.max }
    fn sam_hurtable(&mut self) { self.g.god_mode = false; }
    fn door_locked_now(&self, d: DoorId) -> bool { self.g.dr[d].leaf.locked }
    fn hands_on_lock(&self) -> Option<GuardId> { self.men().into_iter().find(|&m| self.g.ch[self.g.gd[m].ch].anim.lockpick) }
    fn said_count(&self, words: &str) -> (usize, bool) { let l: Vec<_> = self.said.iter().filter(|l| l.text.contains(words)).collect(); (l.len(), l.iter().all(|x| x.radio)) }
    fn compromised(&self) -> bool { game::silent::silent_compromised(&self.g, &game::seats::here(&self.g)) }
    fn let_go(&mut self, m: GuardId) { let h = &mut self.g.gd[m]; (h.hold, h.pinned) = (false, false); }
    fn sam_holds(&mut self, m: GuardId) -> bool {
        game::player::start_grab(&mut self.g, m, None);
        self.until(2.5, |p| p.g.pl[p.g.player].holding().map_or(false, |h| h.phase == game::character::HoldPhase::Held)).is_some()
    }
    fn in_standoff(&self, m: GuardId) -> Option<bool> { self.g.gd[m].standoff.map(|s| self.g.tab.guards_standoff[s].line_open) }
    fn saw_held(&self, m: GuardId) -> Option<String> { self.g.gd[m].witness.saw_held.as_ref().map(|w| w.who.clone()) }
    fn escalation(&self) -> f64 { self.g.escalation as f64 }
    fn choke_him(&mut self) { assert!(game::game::game_select_verb(&mut self.g, game::game::InteractableKind::Held, Some(game::game::Verb::Choke)), "no choke on the row"); self.tap("Space"); }
    fn room_points(&mut self) -> Vec<(f64, [f64; 2])> {
        use std::collections::BTreeMap;
        let (cs, w, h) = (self.g.col.nav.cell, self.g.col.nav.w as usize, self.g.col.nav.h as usize);
        let mut sum: BTreeMap<i64, (f64, f64, f64)> = BTreeMap::new();
        let mut rows = Vec::new();
        for z in 0..h { for x in 0..w {
            let q = [(x as f64 + 0.5) * cs, (z as f64 + 0.5) * cs];
            let r = game::cleared::cleared_rooms_room_at(&mut self.g, [q[0], 0.0, q[1]]);
            if r == 0.0 { continue; }
            let e = sum.entry(r as i64).or_insert((0.0, 0.0, 0.0));
            (e.0, e.1, e.2) = (e.0 + q[0], e.1 + q[1], e.2 + 1.0);
            rows.push((r as i64, q));
        } }
        sum.into_iter().filter(|(_, e)| e.2 >= 12.0).map(|(r, e)| {
            let mid = [e.0 / e.2, e.1 / e.2];
            let best = rows.iter().filter(|x| x.0 == r).min_by(|a, b| game_start::play::dist(a.1, mid).partial_cmp(&game_start::play::dist(b.1, mid)).unwrap()).unwrap().1;
            (r as f64, best)
        }).collect()
    }
    fn fit_marks(&mut self, at: [f64; 2], from: [f64; 2], n: f64) -> Option<Vec<([f64; 2], Option<[f64; 2]>, f64)>> {
        let front = if game_start::play::dist(from, at) > 0.3 { yaw_to(from, at) } else { 0.0 };
        let f = game::roomfit::fit_spots(&mut self.g, [at[0], 0.0, at[1]], n, front, [from[0], 0.0, from[1]], None, None)?;
        Some(f.iter().map(|m| ([m.pos[0], m.pos[2]], m.watch.map(|w| [w[0], w[2]]), m.face)).collect())
    }
    fn line_marks(&mut self, at: [f64; 2], from: [f64; 2], n: f64) -> Vec<[f64; 2]> {
        let front = if game_start::play::dist(from, at) > 0.3 { yaw_to(from, at) } else { 0.0 };
        game::orders::squad_orders_spots_at(&mut self.g, [at[0], 0.0, at[1]], front, n).iter().map(|p| [p[0], p[2]]).collect()
    }
    fn no_place_to_stand(&mut self, at: [f64; 2]) -> bool { game::doorway::doorway_in_way(&self.g, [at[0], 0.0, at[1]]) || game::scene_collision::nav_grid_is_blocked(&self.g, at[0], at[1]) }
    fn run_along(&mut self, at: [f64; 2], yaw: f64, most: f64) -> f64 { game::scene_collision::raycast(&mut self.g, [at[0], 1.55, at[1]], [yaw.sin(), 0.0, yaw.cos()], most).map_or(most, |h| h.t) }
    fn standoff_marks(&mut self, d: DoorId, side: f64, men: &[GuardId]) -> Vec<[f64; 2]> {
        game::doortools::door_tools_standoff_posts(&mut self.g, d, side, men.len() as f64, men).unwrap_or_default().iter().map(|p| [p.pos[0], p.pos[2]]).collect()
    }
    fn threshold_given(&mut self, at: [f64; 2]) {
        let (men, t) = (self.men(), game::orders::squad_orders_target_at(&mut self.g, [at[0], 0.0, at[1]]));
        assert!(!matches!(game::doortools::door_tools_threshold(&mut self.g, &men, &t, Some(false)), game::doortools::DoorToolsThreshold::Text(_)), "the threshold was refused");
    }
    fn stack_mark(&mut self, d: DoorId) -> Option<(Option<String>, bool)> {
        let name = self.door_name(d);
        self.view().plan.own.doors.iter().find(|x| x.door.as_deref() == Some(name.as_str()) && x.why == game::view::WaitWhy::Stack).map(|x| (x.key.clone(), x.now))
    }
    fn word_cap_put(&mut self, secs: f64) { self.g.k.doortools.word_cuts_cap = secs; }
    fn op_kind(&self, op: DoorOpId) -> game::doortools::DoorToolKind { self.g.tab.doortools_door_op[op].kind }
    fn beat_times(&self, op: DoorOpId) -> Vec<f64> { self.g.tab.doortools_door_op[op].beats.iter().map(|b| b.t).collect() }
    fn walks_backwards(&self, m: GuardId) -> bool { self.g.gd[m].script.as_ref().map_or(false, |s| s.strafe == Some(true)) }
    fn walks_away_from_his_path(&self, m: GuardId) -> bool {
        let h = &self.g.gd[m];
        let c = &self.g.ch[h.ch];
        let Some(w) = h.path.get(h.path_i as usize) else { return false };
        c.vel[0].hypot(c.vel[2]) > 0.3 && c.vel[0] * (w[0] - c.pos[0]) + c.vel[2] * (w[2] - c.pos[2]) < 0.0
    }
    fn leaves_gone_round(&self) -> f64 { self.g.doors.rounds }
    fn blind_secs(&self, m: GuardId) -> f64 { (self.g.cond[self.g.gd[m].ch].blind_until - self.g.time).max(0.0) }
    fn fold_secs(&self, m: GuardId) -> f64 { (self.g.cond[self.g.gd[m].ch].fold_until - self.g.time).max(0.0) }
    fn is_staff(&mut self, m: GuardId) { self.g.ch[self.g.gd[m].ch].team = game::teams::Team::Guard; }
    fn entry_pods(&mut self, d: DoorId, side: f64, n: f64) -> Vec<[f64; 2]> { game::doortools::door_tools_pods_as_were(&mut self.g, d, side, Some(n)).iter().map(|q| [q.pos[0], q.pos[2]]).collect() }
    fn shallow_pods(&mut self, d: DoorId, side: f64, n: f64) -> Vec<[f64; 2]> { game::doortools::door_tools_shallow_pods(&mut self.g, d, side, Some(n)).iter().map(|q| [q.pos[0], q.pos[2]]).collect() }
    fn stack_marks(&mut self, d: DoorId, side: f64, men: &[GuardId]) -> Vec<[f64; 2]> { game::orders::squad_orders_stack_posts(&mut self.g, d, side, men, None).unwrap_or_default().iter().map(|q| [q.pos[0], q.pos[2]]).collect() }
    fn tool_marks(&mut self, d: DoorId, side: f64, men: &[GuardId]) -> Vec<[f64; 2]> { game::doortools::door_tools_stack_posts(&mut self.g, d, side, men, None).unwrap_or_default().iter().map(|q| [q.pos[0], q.pos[2]]).collect() }
    fn walls_own_marks(&mut self, d: DoorId, side: f64, men: &[GuardId]) -> Vec<[f64; 2]> { game::orders::squad_orders_stack_posts_on(&mut self.g, &game::ask::Over::OWN, d, side, men, &[], false, None).unwrap_or_default().iter().map(|q| [q.pos[0], q.pos[2]]).collect() }
    fn item_at(&self, it: ItemId) -> [f64; 2] { let q = self.g.tab.item[it].pos; [q[0], q[2]] }
    fn shift_tap(&mut self, at: [f64; 3]) {
        self.cursor_on(at);
        self.run(3.0 / 60.0);
        self.key_down("ShiftLeft");
        let key = self.g.k.cmdring.key.clone();
        self.key_down(&key);
        self.run(2.0 / 60.0);
        self.key_up(&key);
        self.run(2.0 / 60.0);
        self.key_up("ShiftLeft");
        self.run(4.0 / 60.0);
    }
    fn shift_held(&mut self, at: [f64; 3]) -> bool {
        self.cursor_on(at);
        self.run(3.0 / 60.0);
        self.key_down("ShiftLeft");
        let key = self.g.k.cmdring.key.clone();
        self.key_down(&key);
        self.until(1.5, |p| p.ring_up()).is_some()
    }
    fn wedge_taken(&mut self, words: Option<&str>) -> String {
        let w = self.ring_wedges();
        let row = words.map_or(w.first(), |t| w.iter().find(|x| x.1.contains(t))).unwrap_or_else(|| panic!("no wedge {words:?} in {w:?}")).clone();
        let digit = format!("Digit{}", row.0);
        self.key_down(&digit);
        self.frame();
        self.key_up(&digit);
        self.keys_let_go();
        row.1
    }
    fn keys_let_go(&mut self) { let key = self.g.k.cmdring.key.clone(); self.key_up(&key); self.key_up("ShiftLeft"); self.run(4.0 / 60.0); }
    fn smoke_left(&self) -> f64 { game::squadsmoke::squad_smoke_left(&self.g, &game::seats::here(&self.g)) }
    fn smoke_cans(&mut self) -> Vec<([f64; 2], bool)> {
        let list: Vec<ItemId> = self.g.items.items.iter().copied().filter(|&i| self.g.tab.item[i].kind == game::throwables::ItemKind::Smoke).collect();
        list.into_iter().map(|i| { let q = self.g.tab.item[i].pos; ([q[0], q[2]], self.g.tab.item[i].emit_t > 0.0) }).collect()
    }
    fn cards_carry(&mut self, cards: &[(usize, f64)]) {
        for &(i, n) in cards {
            let c = &mut self.g.m.kit.kit.members[i];
            c.smoke = n;
            if n > 0.0 { (c.frags, c.charges) = (0.0, 0.0); }   // (the pouches: room made for it, as the card's own row asks)
        }
        game::game::game_restart(&mut self.g);
    }
    fn round_from(&mut self, m: GuardId, at: [f64; 3]) { let c = self.g.gd[m].ch; let q = self.g.ch[c].pos; self.g.ch[c].muzzle = [q[0], 1.35, q[2]]; let o = game::combat::FireOpts { spread: Some(0.0), weapon: None, steady: Some(true), blind: None }; game::combat::fire_weapon(&mut self.g, c, at, Some(&o)); }
    fn sam_down_hits(&self) -> (bool, f64) { let p = &self.g.pl[self.g.player]; (self.g.cond[p.ch].down, p.hits_left) }
    fn down_how(&self, m: GuardId) -> Option<(game::game::DownKind, game::game::DownBy)> { let h = &self.g.gd[m]; Some((h.down_kind?, h.down_by?)) }
    fn rescue_free(&self) -> bool { game::rescue::rescue_mode(&self.g) == game::rescue::RescueMode::Free }
    fn rescue_jobs(&self) -> usize { self.g.rescue.flow.jobs.len() }
    fn cans_left(&self) -> f64 { self.g.sets[self.g.seat].door_tools.bangs }
    fn frags_left(&self) -> f64 { self.g.sets[self.g.seat].door_tools.frags }
    fn throw_now(&mut self, men: &[GuardId], at: [f64; 2]) -> bool { game::squadthrow::squad_thrower(&mut self.g, men, [at[0], 0.0, at[1]], game::squadthrow::SquadThrowKind::Frag).is_some() }
    fn throw_after_walk(&mut self, men: &[GuardId], at: [f64; 2]) -> Option<(GuardId, [f64; 2])> {
        let t = game::squadthrow::squad_throw_plan(&mut self.g, men, [at[0], 0.0, at[1]], game::squadthrow::SquadThrowKind::Frag, Some(true), None)?;
        let q = t.post?;
        Some((t.gd, [q[0], q[2]]))
    }
    fn frags_out(&self) -> Vec<[f64; 2]> { self.g.items.items.iter().copied().filter(|&i| self.g.tab.item[i].kind == game::throwables::ItemKind::Frag).map(|i| { let q = self.g.tab.item[i].pos; [q[0], q[2]] }).collect() }
    fn shell_stats(&self) -> (i32, i32, i32) { let s = &self.g.stats.shotgun; (s.in_men, s.dropped, s.peppered) }
    fn shell_from(&mut self, m: GuardId, from: [f64; 2], at: [f64; 3], spread: f64) {
        let c = self.g.gd[m].ch;
        self.g.ch[c].muzzle = [from[0], 1.35, from[1]];
        let def = self.g.m.weapons.weapons.get(game::weapons::WeaponId::M870).unwrap().clone();
        let o = game::combat::FireOpts { spread: Some(spread), weapon: Some(def), steady: Some(true), blind: None };
        game::combat::fire_weapon(&mut self.g, c, at, Some(&o));
    }
    fn shotgun_rule(&mut self) -> (f64, f64, bool, Vec<bool>) {
        let def = self.g.m.weapons.weapons.get(game::weapons::WeaponId::M870).unwrap().clone();
        let p = def.pellets.unwrap();
        let mk = |d: &game::weapons::WeaponDef| game::shotgun::Shell { def: d.clone(), i: 0.0, dir: None, men: game::js::JsMap::new(), ends: Vec::new(), glass: game::js::JsMap::new(), late: Vec::new() };
        let (mut a, mut b) = (mk(&def), mk(&def));
        let near = game::shotgun::pellet_drops(&mut self.g, &mut a, CharId(0), p.drop_m - 0.5, [1.0, 0.0, 0.0]);
        let far: Vec<bool> = (0..p.need as usize).map(|_| game::shotgun::pellet_drops(&mut self.g, &mut b, CharId(0), p.drop_m + 6.0, [1.0, 0.0, 0.0])).collect();
        (p.drop_m, p.need, near, far)
    }
    fn sees_doorway(&mut self, m: GuardId, d: DoorId) -> bool {
        let (p, c) = (self.at(m), self.frame_at(d));
        let q = 0.35 / game_start::play::dist(p, c);
        let at = [c[0] + (p[0] - c[0]) * q, 1.1, c[1] + (p[1] - c[1]) * q];
        !game::scene_collision::segment_blocked(&mut self.g, [p[0], 1.55, p[1]], at, None, None, None)
    }
    fn op_ended_on(&self, d: DoorId) -> bool { self.ops_on(d).is_empty() && self.last_op(d).is_some() }
    fn ring_up(&self) -> bool { self.g.sets[self.g.seat].orders.cmd_ring.is_some() }
    fn ring_part(&self) -> Vec<GuardId> { self.g.sets[self.g.seat].orders.menu.as_ref().and_then(|m| m.part.clone()).unwrap_or_default() }
    fn ring_caption(&mut self) -> String { self.view().ring.map_or(String::new(), |r| r.caption) }
    fn ring_wedges(&mut self) -> Vec<(f64, String)> { self.view().ring.map_or(Vec::new(), |r| r.wedges.iter().map(|w| (w.key, w.text.clone())).collect()) }
    fn goal_half_off(&mut self) { self.g.k.cmdring.goal_half = false; }
    fn twice_off(&mut self) { self.g.k.cmdring.twice = false; }
    fn twice_secs(&self) -> f64 { self.g.k.cmdring.twice_secs }
    fn panes_by(&self, d: DoorId) -> Vec<(f64, f64)> {
        let (f, w) = (game::guards::frame_of(&self.g, d), self.door_w(d));
        let mut out = Vec::new();
        for &b in &self.g.col.boxes {
            let x = &self.g.tab.boxes[b];
            if (x.flags as u32) & (game::scene_boxes::box_flag().glass as u32) == 0 { continue; }
            let (a, o, ha) = ((x.c[0] - f.hx) * f.ux + (x.c[2] - f.hz) * f.uz, (x.c[0] - f.hx) * f.nx + (x.c[2] - f.hz) * f.nz, f.ux.abs() * x.h[0] + f.uz.abs() * x.h[2]);
            if o.abs() < 0.3 && a + ha > -2.5 && a - ha < w + 2.5 { out.push((a - ha, a + ha)); }
        }
        out
    }
    fn run_each(&mut self, secs: f64, mut each: impl FnMut(&mut Play)) { for _ in 0..(secs * 60.0) as i32 { self.frame(); each(self); } }
    fn thr_jamb(&self, op: DoorOpId) -> Vec<GuardId> { self.g.tab.doortools_door_op[op].thr.as_ref().map_or(Vec::new(), |t| t.jamb.clone()) }
    fn thr_told(&self, op: DoorOpId) -> bool { self.g.tab.doortools_door_op[op].thr.as_ref().map_or(false, |t| t.told) }
    fn cut_of(&self, op: DoorOpId, m: GuardId) -> Option<(usize, usize, [f64; 2], bool)> {
        let s = self.g.tab.doortools_door_op[op].slice.get(m)?;
        let i = s.i as usize;
        let at = s.pts.get(i).map_or([0.0, 0.0], |q| [q[0], q[2]]);
        Some((s.pts.len(), i, at, s.pie.as_ref().map_or(false, |x| x.fin)))
    }
    fn leaning(&self, m: GuardId) -> bool { self.g.gd[m].script.as_ref().map_or(false, |s| s.lean.map_or(false, |l| l != 0.0)) }
    fn knee_of(&self, m: GuardId) -> f64 { self.g.ch[self.g.gd[m].ch].anim.crouch }
    fn head_past_hips(&self, m: GuardId, along: [f64; 2]) -> f64 {
        let b = &self.g.ch[self.g.gd[m].ch].bones;
        match (b.get(game::character::Bone::Head), b.get(game::character::Bone::Hips)) { (Some(h), Some(q)) => (h[0] - q[0]) * along[0] + (h[2] - q[2]) * along[1], _ => 0.0 }
    }
    fn men_spared(&mut self) { for m in self.mates() { self.g.gd[m].god = true; } }
    fn blast_tears(&mut self, at: [f64; 2]) -> Vec<DoorId> { game::doors::doors_blast(&mut self.g, [at[0], 1.0, at[1]], 1.5, -1.0) }
    fn lane_half(&self, along: f64) -> f64 { let (ff, fl) = (&self.g.k.friendlyfire, &self.g.k.fireline); ff.body_m + fl.clear_m + along * ff.cone_k * fl.spread }
    fn sam_rounds(&self) -> f64 { self.g.fa[self.g.pl[self.g.player].rifle].shots_fired }
    fn file_place(&self, gd: GuardId) -> Option<f64> { game::orders::squad_orders_of(&game::seats::here(&self.g), gd).map(|a| self.g.tab.orders_assignment[a].follow) }
    fn sam_clicks(&mut self) { let pl = self.g.player; self.g.arms[self.g.pl[pl].ch].fire_cd = 0.0; self.press(game_start::play::Button::Left); }
    fn in_doorway(&self, d: DoorId, at: [f64; 2]) -> bool { let (a, o) = (self.along(d, at), self.off(d, at, 1.0)); a > -0.05 && a < self.door_w(d) + 0.05 && o.abs() < 0.3 }
    fn in_sweep(&self, d: DoorId, at: [f64; 2]) -> bool { game::doors::door_swing_sweeps(&game::doors::doors_swing_of(&self.g, d), at[0], at[1], Some(BODY_R)) }
    fn from_handle(&self, d: DoorId, at: [f64; 2]) -> f64 { let f = game::guards::frame_of(&self.g, d); let (a, o) = (self.along(d, at), self.off(d, at, 1.0)); ((a - (f.w - HANDLE_IN)).powi(2) + o * o).sqrt() }
    fn door_rows(&self, d: DoorId) -> usize { self.g.sets[self.g.seat].nearby.iter().filter(|r| r.door == Some(d)).count() }
    fn slab_rest(&mut self, d: DoorId, side: f64) -> Option<(bool, f64)> { let s = self.g.dr[d].slab?; let at = [s.p[0], s.p[2]]; Some((s.rest, self.off(d, at, side))) }
    fn clear_line(&mut self, a: [f64; 2], b: [f64; 2]) -> bool { !game::scene_collision::segment_blocked(&mut self.g, [a[0], 1.1, a[1]], [b[0], 1.1, b[1]], None, None, None) }
    fn first_over(&self, op: DoorOpId) -> Option<GuardId> { let c = &self.g.tab.doortools_door_op[op].crossed; c.keys().into_iter().min_by(|a, b| c.get(*a).unwrap().partial_cmp(c.get(*b).unwrap()).unwrap()) }
    fn seen_of_room(&mut self, gd: GuardId, at: [f64; 2]) -> f64 {
        let Some(room) = game::roomfight::room_fight_room_at(&mut self.g, [at[0], 0.0, at[1]]) else { return 0.0 };
        let cells: Vec<f64> = self.g.tab.roomfight_room[room].cells.iter().map(|c| c.0).collect();
        game::floorseen::floor_seen_count(&game::seats::here(&self.g), gd, &cells, self.g.time - self.g.k.doortools.lead_seen_secs)
    }
}

/// how far apart two ways are, radians
pub fn off_by(a: f64, b: f64) -> f64 { let d = (a - b).rem_euclid(std::f64::consts::TAU); d.min(std::f64::consts::TAU - d) }
/// the yaw from one place to another
pub fn yaw_to(a: [f64; 2], b: [f64; 2]) -> f64 { (b[0] - a[0]).atan2(b[1] - a[1]) }
