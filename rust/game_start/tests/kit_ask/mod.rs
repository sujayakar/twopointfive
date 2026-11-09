//! WHAT THE KIT AND PLAYER TESTS ASK AND STAGE that `Play` has no reader or call for yet: one line each, named for what it answers, so that a test never reaches into a field.
#![allow(dead_code)]
use game::game::{Foe, GuardState};
use game::ids::*;
use game_start::play::Play;

pub trait KitAsk {
    // ---- staging
    /// A man set down at `at` facing the point `face`, still, off his route, and fighting: alert on Sam where Sam stands, the sighting `seen_ago` seconds old, his reaction beat `react` seconds off.
    fn fights_from(&mut self, gd: GuardId, at: [f64; 2], face: [f64; 2], seen_ago: f64, react: f64);
    /// No clock stands between him and his next round.
    fn gun_comes_round_at_once(&mut self, gd: GuardId);
    /// A door's leaf left standing at `angle` (radians from shut), settled and off the latch.
    fn leaf_left_at(&mut self, d: DoorId, angle: f64);
    /// Sam stands on his spot, walking back to it if pushed off, his gun on `aim` (the sandbox's puppet).
    fn sam_holds_his_spot(&mut self, aim: [f64; 2]);
    /// Every man but `gd` held on his post, deaf and blind.
    fn all_but_held(&mut self, gd: GuardId);
    /// The aim's settling model on or off (`Game.tune.aimModel`, a variable of a game).
    fn aim_model(&mut self, on: bool);
    /// A round from Sam's gun that flies exactly where it is sent (combat's own call, no spread).
    fn sam_round_at(&mut self, at: [f64; 3]);

    /// Nobody can see or hear Sam (the sandbox's ghost): for a test that is not about the senses.
    fn sam_a_ghost(&mut self);
    /// Sam crouches or stands by the puppet's word, staying where he is.
    fn sam_low(&mut self, low: bool);
    /// Sam is driven toward `goal` by the sandbox's puppet, as if his keys were held that way.
    fn sam_walks_to(&mut self, goal: [f64; 2], low: bool);
    /// The puppet's strings let go.
    fn sam_stops(&mut self);
    /// The man is sent straight for `goal` at `speed` by a script of his own, blind to all else.
    fn man_walks_to(&mut self, gd: GuardId, from: [f64; 2], goal: [f64; 2], speed: f64);
    /// Sam's lunge at a man, by the call his key makes, whatever the row said.
    fn sam_lunges(&mut self, gd: GuardId);
    /// Sam takes a man in his arm, by the call his key makes.
    fn sam_grabs(&mut self, gd: GuardId);
    /// One step of the game of a length other than a sixtieth.
    fn step(&mut self, dt: f64);
    /// A boot lands on a door from a place of the floor, the player's.
    fn door_booted(&mut self, d: DoorId, from: [f64; 2]);
    /// The floor's "clear the nearest room", on a fix at `at`; the rooms it dealt.
    fn clear_called_at(&mut self, at: [f64; 2], then: [f64; 2]) -> Vec<String>;
    /// The running clear is called off.
    fn clear_called_off(&mut self);

    /// He came with nobody and his card holds this many door charges (the count crosses by the door tools' own call).
    fn alone_with_charges(&mut self, n: f64);
    /// A shut door's leaf home on its latch.
    fn leaf_shut(&mut self, d: DoorId);
    /// Sam stood `off` metres off the middle of a shut door on the given side, a moment to settle.
    fn sam_at_door(&mut self, d: DoorId, side: f64, off: f64);
    /// His highlight put on the door's row of that verb and the interaction key pressed: true if the row was on offer.
    fn takes_door_row(&mut self, verb: game::game::Verb) -> bool;
    /// His gun comes to his hand or goes away: "pistol", "rifle", "nothing", settled.
    fn sam_arms(&mut self, what: &str);

    /// The camera turned square on the plan, as a player's is on most floors: W is north, D is east.
    fn camera_square(&mut self);
    /// Nobody's wits at work: the men stand as they are put (the sandbox's AI switch).
    fn no_wits(&mut self);
    /// A man put on his post where he stands.
    fn man_posted(&mut self, gd: GuardId);
    /// A big tool in Sam's big slot; and a teammate's card's.
    fn sam_carries(&mut self, tool: game::kit::BigKit);
    fn man_carries(&mut self, gd: GuardId, tool: game::kit::BigKit);
    /// A man let off his post, free to go.
    fn man_let_off_his_post(&mut self, gd: GuardId);
    /// A man calm again: nothing known, nobody to look for.
    fn man_calmed(&mut self, gd: GuardId);
    /// A man goes up as if he were on Sam, kept to his place, a stale fix on a far place.
    fn man_up_on_a_post(&mut self, gd: GuardId, fix: [f64; 2]);
    /// The level's civilians off the floor (their phone calls hold saves: the senses' business, not the saves').
    fn no_civilians(&mut self);

    // ---- asking
    /// how a man went down, if he did
    fn down_how(&self, gd: GuardId) -> Option<game::game::DownKind>;
    /// the mission's card: kills, knockouts
    fn card(&self) -> (f64, f64);
    /// has the player a blow under way
    fn sam_mid_blow(&self) -> bool;
    /// is the man in Sam's arm, and held
    fn in_the_arm(&self) -> bool;
    /// is the man still breathing, down
    fn breathing(&self, gd: GuardId) -> bool;
    /// is the row for this man in Sam's reach (None: no row for him)
    fn man_row_in_reach(&self, gd: GuardId) -> Option<bool>;
    /// is there a solid thing between these two places, knee and chest high
    fn wall_between(&mut self, a: [f64; 2], b: [f64; 2]) -> bool;
    /// How far one exact push out of the walls would move Sam now: 0 where he stands free.
    fn sam_in_a_wall(&mut self) -> f64;
    /// does the leaf open toward this side of its door
    fn opens_toward(&self, d: DoorId, at: [f64; 2]) -> bool;
    /// is Sam on a wall, and the wall's way out (its normal) and how far his back stands off its plane along it
    fn on_wall(&self) -> Option<[f64; 2]>;
    /// A key tapped, as a finger taps it: down one frame and up.
    fn tap(&mut self, code: &str);
    /// what is in his right hand by the game's rules: "pistol", "rifle" or "nothing"
    fn in_hand(&self) -> &'static str;
    /// the rounds ready in both his guns
    fn sam_rounds(&self) -> f64;
    /// are his hands on a charge going on
    fn sam_charging(&self) -> bool;
    /// does the character stand with both hands on the work
    fn two_hands_on_work(&self) -> bool;
    /// how many door charges are left in the squad's bag (alone: on his card)
    fn charges_left(&self) -> f64;
    /// has this door gone in
    fn door_in(&self, d: DoorId) -> bool;
    /// the rows on his list now, kind by verb
    fn rows(&self) -> Vec<(game::game::InteractableKind, Option<game::game::Verb>)>;
    /// a place of a door's own frame: `along` metres from the hinge along the shut leaf, `out` metres off it to the side it opens into
    fn leaf_pt(&self, d: DoorId, along: f64, out: f64) -> [f64; 2];
    /// a door's leaf width
    fn leaf_w(&self, d: DoorId) -> f64;
    /// where this place stands to the leaf as it stands now: inside its length, on which face, and how deep a man's body there stands in it
    fn to_leaf(&self, d: DoorId, at: [f64; 2]) -> (bool, f64, f64);
    /// the middle of a door's frame, in plan
    fn frame_mid(&self, d: DoorId) -> [f64; 2];
    /// is a man folded and blind off a door (seconds left, 0 if not)
    fn folded(&self, gd: GuardId) -> f64;
    /// what leaves have done to men so far: startled, frames carried, thrown
    fn door_on_men(&self) -> (i32, i32, i32);
    /// where each stun can in the air or on the floor is now
    fn cans(&self) -> Vec<[f64; 3]>;
    /// where the last stun can went off, if one has
    fn last_bang(&self) -> Option<[f64; 3]>;
    /// how fast Sam is moving over the floor
    fn sam_speed(&self) -> f64;
    /// is any man of these blind
    fn any_blind(&self, men: &[GuardId]) -> bool;
    /// what his card lets a man's legs do, as a share of the player's own
    fn card_pace(&self, gd: GuardId) -> f64;
    /// the player's own walk, metres a second; and the flat share the team walks with him behind them
    fn walks(&self) -> (f64, f64);
    /// the most a follower's legs take to close on a player at a walk: more than that is a run
    fn closing_pace(&self) -> f64;
    /// is he at a run, and why (None: not at a run)
    fn at_a_run(&mut self, gd: GuardId) -> Option<game::teampace::RunWhy>;
    /// is he on his order's mark
    fn on_his_mark(&self, gd: GuardId) -> bool;
    /// the frag's two reaches and the reach of what it opens: lethal, wound, walls; its fuse
    fn frag_reach(&self) -> (f64, f64, f64, f64);
    /// how much of a man at `at` a burst at `from` has a line to, as the walls stand; and as the burst will leave them
    fn in_a_burst(&mut self, from: [f64; 2], at: [f64; 2]) -> (f64, f64);
    /// what the last burst did to whom
    fn burst_hits(&self) -> Vec<(game::teams::Team, game::frag::BlastResult)>;
    /// bursts so far; men of his own side a burst of his side has caught
    fn bursts(&self) -> (i32, i32);
    /// rounds of his own side's a man would not press with a friend in the way, so far
    fn rounds_held_back(&self) -> i32;
    /// Sam's rounds that have found a teammate so far
    fn sam_on_mate(&self) -> i32;
    /// the place of the floor on the line from a man's muzzle to another man's chest, `short` metres short of him
    fn on_the_line(&self, from: GuardId, to: GuardId, short: f64) -> [f64; 2];
    /// where the last burst was, in plan
    fn burst_at(&self) -> Option<[f64; 2]>;
    /// which of a door's two faces (+1, -1) lies to the north: the corridor's side of the office's rooms
    fn corridor_side(&self, d: DoorId) -> f64;
    /// a place `along` metres along a door's wall from the middle of its leaf and `off` metres out on that face
    fn off_the_door(&self, d: DoorId, side: f64, along: f64, off: f64) -> [f64; 2];
    /// how long the hold's gun waits between two rounds, by what the hold says; the rounds in his pistol
    fn pistol_rounds(&self) -> f64;
    /// where the light of his gun stands, and how wide the emitter is; his gun shoulder
    fn gun_light(&self) -> ([f64; 3], f64, bool);
    /// his gun shoulder, in the world
    fn sam_shoulder(&self) -> [f64; 3];
    /// is there anything solid between these two points
    fn solid_between(&mut self, a: [f64; 3], b: [f64; 3]) -> bool;
    /// how lit a place is to the light estimate
    fn light_at(&mut self, at: [f64; 3]) -> f64;
    /// has the wall at his muzzle put this man's light out
    fn wall_has_his_light(&self, gd: GuardId) -> bool;
    /// is his muzzle down for a wall
    fn muzzle_down(&self, gd: GuardId) -> bool;
    /// the wall switch of the room at this place of the plan, if the level has one
    fn switch_near(&mut self, at: [f64; 2]) -> Option<game::lightswitch::WallSwitch>;
    /// is the room's light on, by its switch
    fn switch_on(&self, sw: &game::lightswitch::WallSwitch) -> bool;
    /// every row of the orders table: its id, its digit, whether a key of its own carries it
    fn order_keys(&self) -> Vec<(String, Option<f64>, bool)>;
    /// the name each door is said by, as the level stands
    fn door_names(&self) -> Vec<String>;
    /// what a man calls each of the level's routes
    fn route_words(&self) -> Vec<String>;
    /// what the big tools have done so far: Sam's blows, his shells; a teammate's blows, doors he has hammered
    fn big_counts(&self) -> (i32, i32, i32, i32);
    /// is a man's own gun in his hands (no tool out on a sling)
    fn own_gun_up(&self, gd: GuardId) -> bool;
    /// how many blows the frame asks of the hammer
    fn frame_blows(&self) -> f64;
    /// the floor is told something is up, once
    fn floor_on_edge(&mut self);
    /// a locked door found picked: its lock open, nothing broken
    fn door_found_picked(&mut self, d: DoorId);
    /// a locked door found kicked in: its lock broken
    fn door_found_forced(&mut self, d: DoorId);
    /// he has been walked to where he suspects something, on his own wits from here
    fn man_suspects(&mut self, gd: GuardId, at: [f64; 2]);
    /// has his body been found by the floor
    fn body_found(&self, gd: GuardId) -> bool;
    /// how many lines have been said so far
    fn said_so_far(&self) -> usize;
    /// the lines said since that count: who, what
    fn said_since(&self, n: usize) -> Vec<(GuardId, String)>;
    /// the room the running clear is drilling now
    fn clear_drilling(&self) -> Option<String>;
    /// every man still up on the level who is not ours
    fn men_on_the_floor(&self) -> Vec<GuardId>;
    /// the opening a man is dealt, if any
    fn opening_of(&self, gd: GuardId) -> Option<game::ids::CoverOpeningId>;
    /// a man dealt an opening with his gun on it within ten degrees and his feet still
    fn squared_up(&self, gd: GuardId) -> bool;
    /// the openings Sam's own gun is counted as covering
    fn openings_of_sam(&self) -> Vec<game::ids::CoverOpeningId>;
    /// where an opening is
    fn opening_at(&self, o: game::ids::CoverOpeningId) -> [f64; 2];
    /// is there an opening nobody has that this man could take
    fn opening_to_spare_for(&self, gd: GuardId) -> bool;
    /// how often the openings are dealt
    fn deal_secs(&self) -> f64;
    /// a rear man of a stack who has still more than the last steps to his mark to walk: nobody deals him an opening yet
    fn stacks_on_his_walk(&self, gd: GuardId) -> bool;
    /// holds a stack's jamb: the door's, and never an opening's
    fn stacks_at_the_jamb(&self, gd: GuardId) -> bool;
    /// a man rounds pass through
    fn man_cannot_fall(&mut self, gd: GuardId);
    /// the men dealt a grenade of their own, once the floor's book is opened
    fn frag_carriers(&self) -> Vec<GuardId>;
    /// the floor's defenders, the men who may carry one; and the rule that deals them: one in this many, at most this many
    fn frag_deal(&self) -> (usize, f64, f64);
    /// the fuse and the wind-up of a defender's own grenade
    fn their_frag(&self) -> (f64, f64);
    /// a grenade in play: its fuse left
    fn live_frag_fuse(&self) -> Option<f64>;
    /// where he would go, reading the grenade's cue
    fn frag_step_off(&mut self) -> Option<[f64; 2]>;
    /// is he down
    fn sam_down(&self) -> bool;
    /// the sidearm's table: inside how many metres he goes to it, how long his hands are busy, how long with nobody in front before it goes back
    fn side_table(&self) -> (f64, f64, f64);
    /// a man's gun in his hands is a pistol
    fn pistol_in_hand(&self, gd: GuardId) -> bool;
    /// the sidearm has been drawn over the long gun
    fn side_out(&self, gd: GuardId) -> bool;
    /// the long gun hangs on its sling, whichever it is
    fn long_gun_hung(&self, gd: GuardId) -> bool;
    /// the man's hands are between guns
    fn hands_busy(&self, gd: GuardId) -> bool;
    /// rounds ready in the gun in his hands
    fn rounds_ready(&self, gd: GuardId) -> f64;
    /// is the gun in his hands full
    fn gun_full(&self, gd: GuardId) -> bool;
    /// a man's magazine change, seconds, for the gun in his hands
    fn reload_secs(&self, gd: GuardId) -> f64;
    /// the game's own call when a man runs dry on a man that far off: true if the sidearm came out
    fn goes_to_his_sidearm(&mut self, gd: GuardId, dist: f64) -> bool;
    /// the game's own call for a man in front of him this frame
    fn man_in_front_of(&mut self, gd: GuardId);
    /// sidearms drawn, sidearms put back, since the game stood
    fn sidearm_counts(&self) -> (i32, i32);
    /// his gun has nothing in it at all, chamber or magazine
    fn gun_dry(&mut self, gd: GuardId);
    /// the tube is the 40 mm, not the ring
    fn forty_up(&self) -> bool;
    /// rounds of the 40 mm in the air, in the tube, how far one flies before it arms
    fn forty(&self) -> (usize, f64, f64);
    /// the 40 mm's duds lying on the floor
    fn forty_duds(&self) -> usize;
    /// how the last round ended: armed, flown, did it end on a man
    fn forty_end(&self) -> Option<(bool, f64, bool)>;
    /// how many rounds of the 40 mm have ended
    fn forty_ends(&self) -> usize;
    /// where on a man the round is aimed
    fn chest(&self, gd: GuardId) -> [f64; 3];
    /// reports of blasts so far, and the last with who was in it
    fn blasts(&self) -> usize;
    /// did he go down by the last blast, and was Sam in it
    fn burst_took(&self, gd: GuardId) -> (bool, bool);
    /// gunfire events in the game's list, and blast events
    fn gunfire_and_bangs(&self) -> (usize, usize);
    /// he has folded from a blow and not got up
    fn folded_by_a_blow(&self, gd: GuardId) -> bool;
    /// the tube of his rifle holds shells for the 40 mm
    fn sam_issued_the_forty(&mut self);
    /// shoulders through a door, handles hauled at the run, boots at the run, since the game stood
    fn sprint_door_counts(&self) -> (i32, i32, i32);
    /// his sprint pace, off the table
    fn sprint_pace(&self) -> f64;
    /// the share of his speed a shoulder keeps
    fn shoulder_keeps(&self) -> f64;
    /// is a boot in the air
    fn sam_kicking(&self) -> bool;
    /// is the door off its hinges
    fn door_off_hinges(&self, d: DoorId) -> bool;
    /// is the lock of this door broken
    fn door_broken(&self, d: DoorId) -> bool;
    /// the door locked against the side he stands on
    fn door_locked_to(&mut self, d: DoorId, at: [f64; 2]);
    /// a pair on the floor, the room's rule silent: staged as a quiet mission with a squad does it
    fn quiet_pair(&mut self);
    /// the squad told 'take him now' on this man; None if it was given
    fn take_him_now(&mut self, foe: GuardId) -> Option<String>;
    /// blows from a partner, counted
    fn partner_blows(&self) -> i32;
    /// how long after the hands go on the blow lands
    fn partner_hit_secs(&self) -> f64;
    /// the lowest of a body's joints above the floor, the feet and toes not counted
    fn lowest_joint(&self, c: game::ids::CharId) -> f64;
    /// a man's body and Sam's
    fn body_of(&self, gd: GuardId) -> game::ids::CharId;
    fn body_of_sam(&self) -> game::ids::CharId;
    /// where a body's hips are
    fn hips_of(&self, c: game::ids::CharId) -> [f64; 3];
    /// all a body's joints as they stand now
    fn joints(&self, c: game::ids::CharId) -> Vec<[f64; 3]>;
    /// the paces off the tables: the crouch walk, the fast crouch, a walk, a sprint, a patrol's walk
    fn paces(&self) -> (f64, f64, f64, f64, f64);
    /// footfalls filed since the list was last asked: their levels (a game event each)
    fn footfalls_now(&self) -> Vec<f64>;
    /// is he sprinting, as the game has it
    fn sam_sprinting(&self) -> bool;
    /// does he count as sneaking
    fn sam_sneaking(&self) -> bool;
    /// is he down in a crouch
    fn sam_is_low(&self) -> bool;
    /// the moves he can be in the middle of: a boot, a lock under his tools, a blow at a man, a body in his hands
    fn sam_in_a_move(&self) -> (bool, bool, bool, bool);
    /// a boot started at this door
    fn sam_starts_kick(&mut self, d: DoorId);
    /// a lock started
    fn sam_starts_pick(&mut self, d: DoorId);
    /// a blow started at this man
    fn sam_starts_takedown(&mut self, gd: GuardId);
    /// a body taken up
    fn sam_starts_drag(&mut self, gd: GuardId);
    /// a door's lock: locked, broken, how far it is picked
    fn lock_state(&self, d: DoorId) -> (bool, bool, f64);
    /// where a boot is struck from at this door, near this spot
    fn kick_spot(&mut self, d: DoorId, near: [f64; 2]) -> [f64; 2];
    /// where a man works a lock at this door, near this spot
    fn work_spot(&mut self, d: DoorId, near: [f64; 2]) -> [f64; 2];
    /// is this a nav cell no plan can leave
    fn stranded(&self, at: [f64; 2]) -> bool;
    /// is this nav cell blocked
    fn cell_blocked(&self, at: [f64; 2]) -> bool;
    /// a man with his fix here, suspicious, on his own feet
    fn man_goes_to_look(&mut self, gd: GuardId, at: [f64; 2]);
    /// is he on a knee off a boot
    fn on_a_knee(&self, gd: GuardId) -> bool;
    /// has a man's belief in a door come to a boot on its way
    fn walking_to_boot(&self, gd: GuardId) -> bool;
    /// is a man working on a door out of what he believes
    fn on_a_door(&self, gd: GuardId) -> bool;
    /// the head of a man, in the world
    fn head(&self, gd: GuardId) -> [f64; 3];
    /// is he out cold and breathing
    fn out_cold(&self, gd: GuardId) -> bool;
    /// what kinds of thing lie on the floor as items
    fn rings_on_floor(&self) -> usize;
    /// how many rings are in the air
    fn rings_in_air(&self) -> usize;
    /// the save rules' clocks: the debounce and the least gap between two saves, seconds
    fn save_clocks(&self) -> (f64, f64);
    /// is an autosave waiting, for what
    fn save_waiting(&self) -> Option<String>;
    /// is a clear running
    fn clearing(&self) -> bool;
    /// the door of that name in the level
    fn door(&self, name: &str) -> DoorId;
    /// how far a leaf stands from shut, radians
    fn leaf(&self, d: DoorId) -> f64;
    /// did a round reach Sam this frame (his hit flash has just been lit)
    fn sam_just_hit(&self) -> bool;
    /// did this man see Sam on this frame
    fn saw_sam(&self, gd: GuardId) -> bool;
    /// a man's walking speed as his legs have it
    fn legs(&self, gd: GuardId) -> f64;
    /// the rounds a man has let go of this frame, by their events: where each left his gun and stopped
    fn shots_now(&self) -> Vec<([f64; 3], [f64; 3])>;
    /// the seconds a man's gun must settle on a man this far off before the gate lets a first round go
    fn gate_after_settling(&self, range: f64) -> f64;
    /// the fastest a man may be moving and still count as planted for a shot
    fn stop_to_shoot(&self) -> f64;
    /// the seconds of a man's reaction still to run: his own beat, or the start and the draw he got on first sight
    fn beat_on(&self, gd: GuardId) -> f64;
    /// the width of the aiming ring's cone as the HUD is told it (None: no ring up)
    fn ring_cone(&self) -> Option<f64>;
    /// the lamp whose fixture stands nearest this place of the plan, and its point
    fn lamp_at(&self, at: [f64; 2]) -> (FixtureId, [f64; 3]);
    /// is that lamp shot out
    fn lamp_out(&self, t: FixtureId) -> bool;
}

impl KitAsk for Play {
    fn fights_from(&mut self, gd: GuardId, at: [f64; 2], face: [f64; 2], seen_ago: f64, react: f64) {
        let c = self.g.gd[gd].ch;
        let yaw = f64::atan2(face[0] - at[0], face[1] - at[1]);
        self.g.ch[c].pos = [at[0], 0.0, at[1]];
        self.g.ch[c].vel = [0.0; 3];
        (self.g.ch[c].body_yaw, self.g.ch[c].aim_yaw) = (yaw, yaw);
        let sam = self.sam_at();
        let t0 = self.g.time;
        let m = &mut self.g.gd[gd];
        (m.state, m.awareness, m.react_t, m.fire_cd, m.last_known, m.last_seen_t, self.g.cond[m.ch].blind_until, m.pinned, m.hold, m.foe) =
            (GuardState::Alert, 1.0, react, 0.3, Some([sam[0], 0.0, sam[1]]), t0 - seen_ago, -1.0, false, false, Some(Foe::Player));
        m.path.clear();
        m.path_goal = None;
    }
    fn gun_comes_round_at_once(&mut self, gd: GuardId) { self.g.gd[gd].fire_cd = 0.0; }
    fn leaf_left_at(&mut self, d: DoorId, angle: f64) {
        use game::doors::*;
        door_set_angle(&mut self.g, d, angle);
        door_set_vel(&mut self.g, d, 0.0);
        door_set_closing(&mut self.g, d, false);
        door_set_latched(&mut self.g, d, angle.abs() < 1e-3);
        door_set_quiet_t(&mut self.g, d, 5.0);
        door_place(&mut self.g, d);
    }
    fn sam_holds_his_spot(&mut self, aim: [f64; 2]) {
        let at = self.sam_at();
        self.g.puppet = Some(game::game::GamePuppet { goal: Some([at[0], 0.0, at[1]]), aim: Some([aim[0], 1.2, aim[1]]), ..game::game::GamePuppet::default() });
    }
    fn all_but_held(&mut self, gd: GuardId) {
        for o in self.g.guards.clone() { if o != gd { let m = &mut self.g.gd[o]; (m.hold, m.pinned, self.g.cond[m.ch].blind_until) = (true, true, 1e9); } }
    }
    fn aim_model(&mut self, on: bool) { self.g.tune.aim_model = on; }
    fn sam_round_at(&mut self, at: [f64; 3]) {
        let c = self.g.pl[self.g.player].ch;
        let o = game::combat::FireOpts { steady: Some(true), spread: Some(0.0), ..game::combat::FireOpts::default() };
        game::combat::fire_weapon(&mut self.g, c, at, true, Some(&o));
    }

    fn sam_a_ghost(&mut self) { self.g.player_invisible = true; }
    fn sam_low(&mut self, low: bool) { let pl = self.g.player; self.g.pl[pl].crouch = low; }
    fn sam_walks_to(&mut self, goal: [f64; 2], low: bool) {
        self.g.puppet = Some(game::game::GamePuppet { goal: Some([goal[0], 0.0, goal[1]]), crouch: Some(low), ..game::game::GamePuppet::default() });
    }
    fn sam_stops(&mut self) { self.g.puppet = None; }
    fn man_walks_to(&mut self, gd: GuardId, from: [f64; 2], goal: [f64; 2], speed: f64) {
        let c = self.g.gd[gd].ch;
        let yaw = f64::atan2(goal[0] - from[0], goal[1] - from[1]);
        self.g.ch[c].pos = [from[0], 0.0, from[1]];
        self.g.ch[c].vel = [0.0; 3];
        (self.g.ch[c].body_yaw, self.g.ch[c].aim_yaw) = (yaw, yaw);
        let m = &mut self.g.gd[gd];
        (m.hold, m.pinned, m.path_goal, m.state, m.awareness) = (false, false, None, GuardState::Patrol, 0.0);
        m.path.clear();
        m.script = Some(game::game::GuardScript { goal: Some([goal[0], 0.0, goal[1]]), speed: Some(speed), upper: Some(game::character::UpperMode::Relaxed), ..game::game::GuardScript::default() });
    }
    fn sam_lunges(&mut self, gd: GuardId) { game::player::start_takedown(&mut self.g, gd, None, None, None); }
    fn sam_grabs(&mut self, gd: GuardId) { game::player::start_grab(&mut self.g, gd, None); }
    fn step(&mut self, dt: f64) { let _ = game_start::stand::frame(&mut self.g, &self.host, dt); self.frames += 1; }
    fn door_booted(&mut self, d: DoorId, from: [f64; 2]) { game::doors::doors_kick_in(&mut self.g, d, [from[0], 0.0, from[1]], game::consts::PLAYER_ID, None); }
    fn clear_called_at(&mut self, at: [f64; 2], then: [f64; 2]) -> Vec<String> {
        self.sam_put(at);
        game::game::game_clear_nearest_room(&mut self.g);
        self.sam_put(then);
        self.g.clearing.as_ref().map_or(Vec::new(), |c| c.rooms.iter().map(|r| r[0].room.clone().unwrap_or_else(|| r[0].door.clone())).collect())
    }
    fn clear_called_off(&mut self) { if self.g.clearing.is_some() { game::squad::clearing_cancel(&mut self.g); self.g.clearing = None; } }

    fn camera_square(&mut self) { let c = self.host.cam; self.g.tab.camera[c].yaw = 0.0; game::camera::follow_camera_rebuild(&mut self.g, c); }
    fn no_wits(&mut self) { self.g.ai_enabled = false; }
    fn man_posted(&mut self, gd: GuardId) { let m = &mut self.g.gd[gd]; (m.hold, m.pinned) = (true, true); }
    fn sam_carries(&mut self, tool: game::kit::BigKit) { let pl = self.g.player; self.g.pl[pl].big_kit = tool; }
    fn man_carries(&mut self, gd: GuardId, tool: game::kit::BigKit) { if let Some(mut e) = self.g.m.kit.man_of.get(gd).cloned() { e.kit.big = tool; self.g.m.kit.man_of.set(gd, e); } }
    fn man_let_off_his_post(&mut self, gd: GuardId) { let m = &mut self.g.gd[gd]; (m.hold, m.pinned) = (false, false); }
    fn man_calmed(&mut self, gd: GuardId) {
        let m = &mut self.g.gd[gd];
        (m.state, m.awareness, m.last_known, m.pinned, m.hold) = (GuardState::Patrol, 0.0, None, false, true);
        m.path.clear();
        self.g.alarm.episode = false;
        self.g.escalation = 0.0;
        self.g.escalation_t = -1.0;
    }
    fn man_up_on_a_post(&mut self, gd: GuardId, fix: [f64; 2]) {
        let t = self.g.time;
        let m = &mut self.g.gd[gd];
        (m.hold, m.pinned, m.state, m.awareness, m.last_known, m.last_seen_t) = (false, true, GuardState::Alert, 1.0, Some([fix[0], 0.0, fix[1]]), t - 30.0);
    }
    fn no_civilians(&mut self) { self.g.tune.civilians = false; self.g.civilians.clear(); }
    fn alone_with_charges(&mut self, n: f64) { game::doortools::door_tools_alone(&mut self.g, n); }
    fn leaf_shut(&mut self, d: DoorId) {
        use game::doors::*;
        door_set_angle(&mut self.g, d, 0.0);
        door_set_vel(&mut self.g, d, 0.0);
        door_set_latched(&mut self.g, d, true);
    }
    fn sam_at_door(&mut self, d: DoorId, side: f64, off: f64) {
        let w = self.g.dr[d].def.width;
        let at = game::doortools::door_pt(&self.g, d, w / 2.0, off, side, None);
        self.sam_put([at[0], at[2]]);
        self.leaf_shut(d);
        self.run(0.2);
    }
    fn takes_door_row(&mut self, verb: game::game::Verb) -> bool {
        if !game::game::game_select_verb(&mut self.g, game::game::InteractableKind::Door, Some(verb)) { return false; }
        self.tap("Space");
        true
    }
    fn sam_arms(&mut self, what: &str) {
        let pl = self.g.player;
        match what {
            "pistol" => { self.g.pl[pl].slot = game::game::SlotKind::Pistol; self.g.pl[pl].holstered = false; }
            "rifle" => { self.g.pl[pl].slot = game::game::SlotKind::Rifle; self.g.pl[pl].holstered = false; }
            _ => self.g.pl[pl].holstered = true,
        }
        self.g.pl[pl].equip_t = 0.0;
        self.run(0.8);
    }
    fn down_how(&self, gd: GuardId) -> Option<game::game::DownKind> { self.g.gd[gd].down_kind }
    fn card(&self) -> (f64, f64) { (self.g.mission.stats.kills, self.g.mission.stats.knockouts) }
    fn sam_mid_blow(&self) -> bool { self.g.pl[self.g.player].takedown.is_some() }
    fn in_the_arm(&self) -> bool { self.g.pl[self.g.player].holding.as_ref().map_or(false, |h| h.phase == game::character::HoldPhase::Held) }
    fn breathing(&self, gd: GuardId) -> bool { let c = self.g.gd[gd].ch; game::condition::down(&self.g, self.g.gd[gd].ch) && !self.g.ch[c].alive && matches!(self.g.gd[gd].down_kind, Some(game::game::DownKind::Struck) | Some(game::game::DownKind::Choked)) }
    fn man_row_in_reach(&self, gd: GuardId) -> Option<bool> { self.g.nearby.iter().chain(self.g.interactables.iter()).find(|r| r.guard == Some(gd)).map(|r| r.in_reach) }
    fn wall_between(&mut self, a: [f64; 2], b: [f64; 2]) -> bool {
        game::scene_collision::solid_blocked(&mut self.g, [a[0], 0.5, a[1]], [b[0], 0.5, b[1]], None) && game::scene_collision::solid_blocked(&mut self.g, [a[0], 1.0, a[1]], [b[0], 1.0, b[1]], None)
    }
    fn sam_in_a_wall(&mut self) -> f64 {
        let pl = self.g.player;
        let p = self.g.ch[self.g.pl[pl].ch].pos;
        let (r, y1) = (game::player::body_r(&self.g, p), if self.g.pl[pl].crouch { 1.0 } else { 1.7 });
        let mut q = p;
        game::scene_collision::collide_circle(&mut self.g, &mut q, r, 0.2, y1, Some(1.0));
        (q[0] - p[0]).hypot(q[2] - p[2])
    }
    fn opens_toward(&self, d: DoorId, at: [f64; 2]) -> bool { game::doors::doors_pull_face(&self.g, d, [at[0], 0.0, at[1]]) }
    fn on_wall(&self) -> Option<[f64; 2]> { self.g.pl[self.g.player].wall.as_ref().map(|w| [w.n[0], w.n[2]]) }
    fn tap(&mut self, code: &str) { self.key_down(code); self.frame(); self.key_up(code); }
    fn in_hand(&self) -> &'static str {
        let pl = self.g.player;
        if game::game::player_pistol_up(&self.g, pl) { "pistol" } else if game::game::player_rifle_up(&self.g, pl) { "rifle" } else { "nothing" }
    }
    fn sam_rounds(&self) -> f64 { let pl = &self.g.pl[self.g.player]; game::weapons::firearm_rounds_ready(&self.g, pl.pistol) + game::weapons::firearm_rounds_ready(&self.g, pl.rifle) }
    fn sam_charging(&self) -> bool { self.g.pl[self.g.player].charging.is_some() }
    fn two_hands_on_work(&self) -> bool { let c = self.g.pl[self.g.player].ch; self.g.ch[c].anim.wk.two }
    fn charges_left(&self) -> f64 { self.g.door_tools.charges }
    fn door_in(&self, d: DoorId) -> bool { game::doors::door_off(&self.g, d) || (!game::doors::door_latched(&self.g, d) && game::doors::door_lock_broken(&self.g, d)) }
    fn rows(&self) -> Vec<(game::game::InteractableKind, Option<game::game::Verb>)> { self.g.nearby.iter().map(|r| (r.kind, r.verb)).collect() }
    fn leaf_pt(&self, d: DoorId, along: f64, out: f64) -> [f64; 2] {
        let (th, h, sw) = (self.g.dr[d].def.closed_dir, self.g.dr[d].def.hinge, game::doors::door_swing(&self.g, d));
        let (u, n) = ([th.cos(), th.sin()], [-th.sin() * sw, th.cos() * sw]);
        [h[0] + u[0] * along + n[0] * out, h[1] + u[1] * along + n[1] * out]
    }
    fn leaf_w(&self, d: DoorId) -> f64 { self.g.dr[d].def.width }
    fn to_leaf(&self, d: DoorId, at: [f64; 2]) -> (bool, f64, f64) {
        const BODY: f64 = 0.34;
        let (h, w, u) = (self.g.dr[d].def.hinge, self.g.dr[d].def.width, game::doors::door_dir(&self.g, d));
        let (ux, uz, px, pz) = (u[0], u[1], at[0] - h[0], at[1] - h[1]);
        let (along, perp, t) = (px * ux + pz * uz, ux * pz - uz * px, (px * ux + pz * uz).clamp(0.0, w));
        (along > 0.14 && along < w, perp.signum(), BODY + 0.02 - (px - ux * t).hypot(pz - uz * t))
    }
    fn frame_mid(&self, d: DoorId) -> [f64; 2] { let c = self.g.dr[d].frame_centre; [c[0], c[2]] }
    fn folded(&self, gd: GuardId) -> f64 { let m = &self.g.gd[gd]; (self.g.cond[m.ch].fold_until.min(self.g.cond[m.ch].blind_until) - self.g.time).max(0.0) }
    fn door_on_men(&self) -> (i32, i32, i32) { let t = &self.g.stats.doorthrow; (t.startled, t.carried, t.pushed + t.kicked) }
    fn cans(&self) -> Vec<[f64; 3]> { self.g.items.items.iter().filter(|&&i| game::throwables::item_kind(&self.g, i) == game::throwables::ItemKind::Flash).map(|&i| game::throwables::item_pos(&self.g, i)).collect() }
    fn last_bang(&self) -> Option<[f64; 3]> { self.g.events.iter().rev().find(|e| e.kind == game::director::EventKind::Bang).map(|e| e.pos) }
    fn sam_speed(&self) -> f64 { let v = self.g.ch[self.g.pl[self.g.player].ch].vel; v[0].hypot(v[2]) }
    fn any_blind(&self, men: &[GuardId]) -> bool { men.iter().any(|&m| self.blind(m)) }
    fn card_pace(&self, gd: GuardId) -> f64 { game::kitload::load_pace_on(&self.g, self.g.gd[gd].ch) }
    fn closing_pace(&self) -> f64 { self.g.tune.player_walk * self.g.k.teampace.follow_k }
    fn walks(&self) -> (f64, f64) { (self.g.tune.player_walk, game::teampace::team_walk(&self.g)) }
    fn at_a_run(&mut self, gd: GuardId) -> Option<game::teampace::RunWhy> { game::teampace::run_why(&mut self.g, gd) }
    fn on_his_mark(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).map_or(false, |a| self.g.tab.orders_assignment[a].arrived) }
    fn frag_reach(&self) -> (f64, f64, f64, f64) { let k = &self.g.k.frag; (k.lethal_m, k.wound_m, k.destr_m, k.fuse) }
    fn in_a_burst(&mut self, from: [f64; 2], at: [f64; 2]) -> (f64, f64) {
        let lift = self.g.k.frag.lift;
        let (f, m, h) = ([from[0], 0.05 + lift, from[1]], [at[0], 0.0, at[1]], [at[0], 1.65, at[1]]);
        (game::frag::exposure(&mut self.g, f, m, h, None), game::frag::exposed_to(&mut self.g, f, m, h, None))
    }
    fn burst_hits(&self) -> Vec<(game::teams::Team, game::frag::BlastResult)> { self.g.m.frag.frag_log.last().map_or(Vec::new(), |r| r.hits.iter().map(|h| (h.team, h.result)).collect()) }
    fn bursts(&self) -> (i32, i32) { (self.g.stats.frag.bursts, self.g.stats.frag.own) }
    fn rounds_held_back(&self) -> i32 { self.g.stats.friendlyfire.unpressed }
    fn sam_on_mate(&self) -> i32 { self.g.stats.friendlyfire.sam_on_mate }
    fn on_the_line(&self, from: GuardId, to: GuardId, short: f64) -> [f64; 2] {
        let (m, t) = (self.g.ch[self.g.gd[from].ch].muzzle, self.at(to));
        let (dx, dz) = (t[0] - m[0], t[1] - m[2]);
        let l = dx.hypot(dz);
        [t[0] - dx / l * short, t[1] - dz / l * short]
    }
    fn burst_at(&self) -> Option<[f64; 2]> { self.g.m.frag.frag_log.last().map(|r| [r.at[0], r.at[2]]) }
    fn corridor_side(&self, d: DoorId) -> f64 { let at = game::doortools::door_pt(&self.g, d, 0.4, 1.0, 1.0, None); if at[2] < self.g.dr[d].frame_centre[2] { 1.0 } else { -1.0 } }
    fn off_the_door(&self, d: DoorId, side: f64, along: f64, off: f64) -> [f64; 2] { let at = game::doortools::door_pt(&self.g, d, self.g.dr[d].def.width / 2.0 + along, off, side, None); [at[0], at[2]] }
    fn pistol_rounds(&self) -> f64 { game::weapons::firearm_rounds_ready(&self.g, self.g.pl[self.g.player].pistol) }
    fn gun_light(&self) -> ([f64; 3], f64, bool) { let l = self.g.pl[self.g.player].light; let t = &self.g.tab.light[l]; (t.pos, t.radius, t.enabled) }
    fn sam_shoulder(&self) -> [f64; 3] { let c = self.g.pl[self.g.player].ch; self.g.ch[c].bones.get(game::character::Bone::UpperArmR).copied().expect("he has a shoulder") }
    fn solid_between(&mut self, a: [f64; 3], b: [f64; 3]) -> bool {
        let (d, l) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2) + (b[2] - a[2]).powi(2)).sqrt());
        game::scene_collision::raycast(&mut self.g, a, [d[0] / l, d[1] / l, d[2] / l], l).is_some()
    }
    fn light_at(&mut self, at: [f64; 3]) -> f64 { game::guards::light_estimate_at(&mut self.g, at, None, None, None) }
    fn wall_has_his_light(&self, gd: GuardId) -> bool { self.g.m.guards.wall_seen.get(gd).map_or(false, |w| w.dark) }
    fn muzzle_down(&self, gd: GuardId) -> bool { self.g.gd[gd].muzzle_down }
    fn switch_near(&mut self, at: [f64; 2]) -> Option<game::lightswitch::WallSwitch> {
        let mut v = game::lightswitch::wall_switches(&mut self.g);
        v.sort_by(|a, b| { let d = |s: &game::lightswitch::WallSwitch| (s.stand[0] - at[0]).hypot(s.stand[2] - at[1]); d(a).partial_cmp(&d(b)).unwrap() });
        v.into_iter().next()
    }
    fn switch_on(&self, sw: &game::lightswitch::WallSwitch) -> bool { game::lightswitch::switch_lit(&self.g, sw) }
    fn order_keys(&self) -> Vec<(String, Option<f64>, bool)> { self.g.tab.orders_order_def.iter().map(|(_, r)| (r.id.clone(), r.key, r.hot_key.is_some())).collect() }
    fn door_names(&self) -> Vec<String> { self.g.doors.list.iter().filter_map(|&d| game::doornames::door_said_name(&self.g, d)).collect() }
    fn route_words(&self) -> Vec<String> { (0..self.g.level.routes.len()).map(|i| game::dialogue::route_name(&self.g, i as f64)).collect() }
    fn big_counts(&self) -> (i32, i32, i32, i32) { let b = &self.g.stats.bigkit; (b.sam_blows, b.sam_shells, b.mate_blows, b.mate_hammered) }
    fn own_gun_up(&self, gd: GuardId) -> bool { self.g.gd[gd].slung.as_ref().map_or(true, |s| !s.out) && self.g.ch[self.g.gd[gd].ch].anim.big_tool.is_none() }
    fn frame_blows(&self) -> f64 { self.g.k.bigslot.swings }
    fn floor_on_edge(&mut self) { game::game::game_escalate(&mut self.g); }
    fn door_found_picked(&mut self, d: DoorId) { game::doors::door_set_locked(&mut self.g, d, false); }
    fn door_found_forced(&mut self, d: DoorId) { game::doors::door_set_locked(&mut self.g, d, false); game::doors::door_set_lock_broken(&mut self.g, d, true); }
    fn man_suspects(&mut self, gd: GuardId, at: [f64; 2]) {
        game::mind::fix(&mut self.g, gd, Some([at[0], 0.0, at[1]]), "guards");
        game::mind::raise(&mut self.g, gd, 0.5, "guards");
        self.g.gd[gd].state = GuardState::Suspicious;
    }
    fn body_found(&self, gd: GuardId) -> bool { self.g.gd[gd].found }
    fn said_so_far(&self) -> usize { self.said.len() }
    fn said_since(&self, n: usize) -> Vec<(GuardId, String)> { self.said[n..].iter().map(|s| (s.who, s.text.clone())).collect() }
    fn clear_drilling(&self) -> Option<String> {
        let c = self.g.clearing.as_ref()?;
        if c.stage != game::squad::ClearStage::Drill || c.drill.is_none() { return None; }
        c.cur.as_ref().map(|r| r.room.clone().unwrap_or_else(|| r.door.clone()))
    }
    fn men_on_the_floor(&self) -> Vec<GuardId> { self.g.guards.iter().copied().filter(|&gd| self.g.gd[gd].team != game::teams::Team::Operative && self.up(gd)).collect() }
    fn opening_of(&self, gd: GuardId) -> Option<game::ids::CoverOpeningId> { self.g.cover.jobs.get(gd).map(|j| j.o) }
    fn squared_up(&self, gd: GuardId) -> bool {
        let Some(o) = self.opening_of(gd) else { return false };
        let (to, at) = (self.opening_at(o), self.at(gd));
        let want = (to[0] - at[0]).atan2(to[1] - at[1]);
        let off = (want - self.g.ch[self.g.gd[gd].ch].aim_yaw + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI) - std::f64::consts::PI;
        off.abs() < 10f64.to_radians() && self.speed(gd) < 0.2
    }
    fn openings_of_sam(&self) -> Vec<game::ids::CoverOpeningId> { self.g.cover.live.iter().filter(|l| l.sam).map(|l| l.o).collect() }
    fn opening_at(&self, o: game::ids::CoverOpeningId) -> [f64; 2] { let a = self.g.tab.cover_opening[o].at; [a[0], a[2]] }
    fn opening_to_spare_for(&self, gd: GuardId) -> bool { self.g.cover.live.iter().any(|l| l.by.is_none() && !l.sam && !l.held && l.can.contains(&gd)) }
    fn deal_secs(&self) -> f64 { self.g.k.cover.deal_secs }
    fn stacks_on_his_walk(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).is_some_and(|a| game::cover::cover_walk_only(&self.g, gd, a)) }
    fn stacks_at_the_jamb(&self, gd: GuardId) -> bool { game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd).is_some_and(|a| { let r = &self.g.tab.orders_assignment[a]; r.order == "stack" && r.spare != Some(true) }) }
    fn man_cannot_fall(&mut self, gd: GuardId) { self.g.gd[gd].god = true; }
    fn frag_carriers(&self) -> Vec<GuardId> { self.g.m.frag.def_books.as_ref().map_or(Vec::new(), |b| b.carriers.iter().map(|c| c.gd).collect()) }
    fn frag_deal(&self) -> (usize, f64, f64) { (game::frag::defenders_of(&self.g).len(), self.g.k.frag.def_share, self.g.k.frag.def_max) }
    fn their_frag(&self) -> (f64, f64) { (self.g.k.frag.def_fuse, self.g.k.frag.def_wind) }
    fn live_frag_fuse(&self) -> Option<f64> { self.g.items.items.iter().copied().find(|&i| game::throwables::item_kind(&self.g, i) == game::throwables::ItemKind::Frag).map(|i| game::throwables::item_fuse(&self.g, i)) }
    fn frag_step_off(&mut self) -> Option<[f64; 2]> { game::frag::frag_step_off(&mut self.g, None).map(|p| [p[0], p[2]]) }
    fn sam_down(&self) -> bool { game::condition::down(&self.g, self.g.pl[self.g.player].ch) }
    fn side_table(&self) -> (f64, f64, f64) { let k = &self.g.k.gunslots; (k.near, k.out, k.lull) }
    fn pistol_in_hand(&self, gd: GuardId) -> bool { let d = self.g.fa[self.g.gd[gd].weapon].def; self.g.m.weapons.weapons.get(d).unwrap().cls == game::weapons::WeaponClass::Pistol }
    fn side_out(&self, gd: GuardId) -> bool { self.g.gd[gd].second.is_some_and(|s| s.out) }
    fn long_gun_hung(&self, gd: GuardId) -> bool { game::gunslots::side_hung(&self.g, gd).is_some() }
    fn hands_busy(&self, gd: GuardId) -> bool { game::gunslots::side_locked(&self.g, gd) }
    fn rounds_ready(&self, gd: GuardId) -> f64 { game::weapons::firearm_rounds_ready(&self.g, self.g.gd[gd].weapon) }
    fn gun_full(&self, gd: GuardId) -> bool { let w = self.g.gd[gd].weapon; game::weapons::firearm_rounds_ready(&self.g, w) >= self.g.m.weapons.weapons.get(self.g.fa[w].def).unwrap().mag_capacity }
    fn reload_secs(&self, gd: GuardId) -> f64 { let d = self.g.fa[self.g.gd[gd].weapon].def; self.g.m.weapons.weapons.get(d).unwrap().reload.dur }
    fn goes_to_his_sidearm(&mut self, gd: GuardId, dist: f64) -> bool { game::gunslots::side_out(&mut self.g, gd, dist) }
    fn man_in_front_of(&mut self, gd: GuardId) { game::gunslots::side_seen(&mut self.g, gd); }
    fn sidearm_counts(&self) -> (i32, i32) { (self.g.stats.gunslots.out, self.g.stats.gunslots.back) }
    fn gun_dry(&mut self, gd: GuardId) { let w = self.g.gd[gd].weapon; self.g.fa[w].chamber = 0.0; self.g.fa[w].mag = Some(0.0); }
    fn forty_up(&self) -> bool { game::weapons::rifle_attachment(&self.g, self.g.pl[self.g.player].rifle) == Some(game::weapons::RifleAttachment::He40) }
    fn forty(&self) -> (usize, f64, f64) { (self.g.shells.len(), self.g.fa[self.g.pl[self.g.player].rifle].shells, self.g.k.he40.arm_m) }
    fn forty_duds(&self) -> usize { self.g.items.items.iter().filter(|&&i| game::throwables::item_kind(&self.g, i) == game::throwables::ItemKind::Shell).count() }
    fn forty_end(&self) -> Option<(bool, f64, bool)> { self.g.m.he40.he40_log.last().map(|e| (e.armed, e.flown, e.met == game::he40::ShellEndMet::Man)) }
    fn forty_ends(&self) -> usize { self.g.m.he40.he40_log.len() }
    fn chest(&self, gd: GuardId) -> [f64; 3] { let c = self.g.gd[gd].ch; self.g.ch[c].bones.get(game::character::Bone::Chest).copied().unwrap_or([self.g.ch[c].pos[0], 1.2, self.g.ch[c].pos[2]]) }
    fn blasts(&self) -> usize { self.g.m.frag.frag_log.len() }
    fn burst_took(&self, gd: GuardId) -> (bool, bool) {
        let id = self.id_of(gd);
        self.g.m.frag.frag_log.last().map_or((false, false), |r| (r.hits.iter().any(|h| h.id == id && h.result == game::frag::BlastResult::Killed), r.player.is_some()))
    }
    fn gunfire_and_bangs(&self) -> (usize, usize) {
        use game::director::EventKind as K;
        (self.g.events.iter().filter(|e| e.kind == K::Shot || e.kind == K::GuardShot).count(), self.g.events.iter().filter(|e| e.kind == K::Bang).count())
    }
    fn folded_by_a_blow(&self, gd: GuardId) -> bool { self.g.cond[self.g.gd[gd].ch].fold_until > self.g.time }
    fn sam_issued_the_forty(&mut self) { let r = self.g.pl[self.g.player].rifle; self.g.fa[r].shells = 3.0; }
    fn sprint_door_counts(&self) -> (i32, i32, i32) { let t = &self.g.stats.sprintdoor; (t.bashes, t.rips, t.kicks) }
    fn sprint_pace(&self) -> f64 { self.g.tune.player_sprint }
    fn shoulder_keeps(&self) -> f64 { self.g.k.sprintdoor.keep }
    fn sam_kicking(&self) -> bool { self.g.pl[self.g.player].kick.is_some() }
    fn door_off_hinges(&self, d: DoorId) -> bool { game::doors::door_off(&self.g, d) }
    fn door_broken(&self, d: DoorId) -> bool { game::doors::door_lock_broken(&self.g, d) }
    fn door_locked_to(&mut self, d: DoorId, at: [f64; 2]) { game::doors::door_set_locked(&mut self.g, d, true); self.g.dr[d].key_side = game::doors::doors_side(&self.g, d, [at[0], 0.0, at[1]]); }
    fn quiet_pair(&mut self) {
        self.g.k.pairup.on = true;
        game::pairup::pair_up_apply(&mut self.g, None);
        self.g.k.silent.on = true;
        self.run(0.3);
        game::silent::set_silent(&mut self.g, true, None);
    }
    fn take_him_now(&mut self, foe: GuardId) -> Option<String> {
        let at = self.at(foe);
        let t = game::orders::OrderTarget::Foe(game::orders::OrderTargetFoe { at: [at[0], 0.0, at[1]], foe });
        game::orders::squad_orders_give(&mut self.g, "pairTake", Some(&t), Some(game::orders::Who::All), None)
    }
    fn partner_blows(&self) -> i32 { self.g.stats.takedown.partner }
    fn partner_hit_secs(&self) -> f64 { self.g.k.takedown.partner_hit }
    fn lowest_joint(&self, c: game::ids::CharId) -> f64 {
        self.g.ch[c].bones.iter().filter(|(b, _)| { let n = b.as_str(); !n.starts_with("foot") && !n.starts_with("toe") }).map(|(_, p)| p[1]).fold(f64::INFINITY, f64::min)
    }
    fn body_of(&self, gd: GuardId) -> game::ids::CharId { self.g.gd[gd].ch }
    fn body_of_sam(&self) -> game::ids::CharId { self.g.pl[self.g.player].ch }
    fn hips_of(&self, c: game::ids::CharId) -> [f64; 3] { self.g.ch[c].bones.get(game::character::Bone::Hips).copied().unwrap_or(self.g.ch[c].pos) }
    fn joints(&self, c: game::ids::CharId) -> Vec<[f64; 3]> { self.g.ch[c].bones.iter().map(|(_, p)| *p).collect() }
    fn paces(&self) -> (f64, f64, f64, f64, f64) { let t = &self.g.tune; (self.g.k.twogears.crouch, self.g.k.crouchrun.pace, t.player_walk, t.player_sprint, t.guard_walk) }
    fn footfalls_now(&self) -> Vec<f64> { let now = self.g.time; self.g.events.iter().filter(|e| e.kind == game::director::EventKind::Step && e.time == now).filter_map(|e| e.level).collect() }
    fn sam_sprinting(&self) -> bool { self.g.pl[self.g.player].sprinting }
    fn sam_sneaking(&self) -> bool { game::unseen::sam_sneaks(&self.g, self.g.player) }
    fn sam_is_low(&self) -> bool { self.g.pl[self.g.player].crouch }
    fn sam_in_a_move(&self) -> (bool, bool, bool, bool) { let p = &self.g.pl[self.g.player]; (p.kick.is_some(), p.picking.is_some(), p.takedown.is_some(), p.dragging.is_some()) }
    fn sam_starts_kick(&mut self, d: DoorId) { game::player::start_kick(&mut self.g, d, None, None); }
    fn sam_starts_pick(&mut self, d: DoorId) { game::player::start_picking(&mut self.g, d); }
    fn sam_starts_takedown(&mut self, gd: GuardId) { game::player::start_takedown(&mut self.g, gd, None, None, None); }
    fn sam_starts_drag(&mut self, gd: GuardId) { game::game::game_toggle_drag(&mut self.g, Some(gd)); }
    fn lock_state(&self, d: DoorId) -> (bool, bool, f64) { (game::doors::door_locked(&self.g, d), game::doors::door_lock_broken(&self.g, d), game::doors::door_pick(&self.g, d)) }
    fn kick_spot(&mut self, d: DoorId, near: [f64; 2]) -> [f64; 2] { let s = game::doors::doors_kick_spot(&mut self.g, d, [near[0], 0.0, near[1]]); [s.pos[0], s.pos[2]] }
    fn work_spot(&mut self, d: DoorId, near: [f64; 2]) -> [f64; 2] { let s = game::doors::doors_work_spot(&mut self.g, d, [near[0], 0.0, near[1]], None, None); [s.pos[0], s.pos[2]] }
    fn stranded(&self, at: [f64; 2]) -> bool { game::guards::stranded_at(&self.g, at[0], at[1]) }
    fn cell_blocked(&self, at: [f64; 2]) -> bool { game::scene_collision::nav_grid_is_blocked(&self.g, at[0], at[1]) }
    fn man_goes_to_look(&mut self, gd: GuardId, at: [f64; 2]) {
        self.man_suspects(gd, at);
        let m = &mut self.g.gd[gd];
        (m.react_t, m.search_t, m.hold, m.pinned) = (0.0, 0.0, false, false);
    }
    fn on_a_knee(&self, gd: GuardId) -> bool { self.g.gd[gd].door_down_until > self.g.time }
    fn walking_to_boot(&self, gd: GuardId) -> bool { self.g.m.doorforce.states.as_ref().and_then(|s| s.men.get(gd)).and_then(|m| m.kick.as_ref()).map_or(false, |k| k.phase == game::doorforce::KickPhase::Walk) }
    fn on_a_door(&self, gd: GuardId) -> bool { self.g.m.doorforce.states.as_ref().and_then(|s| s.men.get(gd)).map_or(false, |m| m.on.is_some()) }
    fn head(&self, gd: GuardId) -> [f64; 3] { let c = self.g.gd[gd].ch; self.g.ch[c].bones.get(game::character::Bone::Head).copied().expect("a man has a head") }
    fn out_cold(&self, gd: GuardId) -> bool { game::condition::down(&self.g, self.g.gd[gd].ch) && game::game::is_breathing(&self.g, gd) }
    fn rings_on_floor(&self) -> usize { self.g.items.items.iter().filter(|&&i| game::throwables::item_kind(&self.g, i) == game::throwables::ItemKind::Ring).count() }
    fn rings_in_air(&self) -> usize { self.g.rings.len() }
    fn save_clocks(&self) -> (f64, f64) { (self.g.k.ckpt.debounce_secs, self.g.k.ckpt.min_gap_secs) }
    fn save_waiting(&self) -> Option<String> { self.g.saves.pending.as_ref().map(|p| p.why.clone()) }
    fn clearing(&self) -> bool { self.g.clearing.is_some() }
    fn door(&self, name: &str) -> DoorId { game::doors::doors_by_name(&self.g, name).unwrap_or_else(|| panic!("no door '{name}' on this level")) }
    fn leaf(&self, d: DoorId) -> f64 { game::doors::door_angle(&self.g, d) }
    fn sam_just_hit(&self) -> bool { self.g.pl[self.g.player].hit_flash > 0.94 }
    fn saw_sam(&self, gd: GuardId) -> bool { self.g.gd[gd].saw_player_this_frame }
    fn legs(&self, gd: GuardId) -> f64 { self.g.gd[gd].speed }
    fn shots_now(&self) -> Vec<([f64; 3], [f64; 3])> {
        let now = self.g.time;
        self.g.events.iter().filter(|e| e.kind == game::director::EventKind::GuardShot && e.time == now).filter_map(|e| Some((e.from?, e.to?))).collect()
    }
    fn gate_after_settling(&self, range: f64) -> f64 {
        let p = self.g.k.aim.guard;
        p.acquire_min * game::aim::settle_time(game::aim::SettleTimeP { settle_secs: p.settle_secs, range_ref: p.range_ref, near_k: p.near_k }, range)
    }
    fn stop_to_shoot(&self) -> f64 { self.g.k.aim.guard.stop_to_shoot }
    fn beat_on(&self, gd: GuardId) -> f64 { self.g.gd[gd].react_t.max(game::startle::startle_left(&self.g, gd)) }
    fn ring_cone(&self) -> Option<f64> { self.view().aim.ring.and_then(|r| r.cone).map(|c| c.spread) }
    fn lamp_at(&self, at: [f64; 2]) -> (FixtureId, [f64; 3]) {
        let d = |t: FixtureId| { let p = self.g.tab.sim_lightshome_fixture[t].pos; (p[0] - at[0]).hypot(p[2] - at[1]) };
        let t = self.g.targets.iter().copied().min_by(|a, b| d(*a).partial_cmp(&d(*b)).unwrap()).expect("a lamp on the level");
        (t, self.g.tab.sim_lightshome_fixture[t].pos)
    }
    fn lamp_out(&self, t: FixtureId) -> bool { game::sim_lightshome::fixture_broken(&self.g, t) }
}
