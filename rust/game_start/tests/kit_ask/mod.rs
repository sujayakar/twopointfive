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
    /// what the big tools have done so far: Sam's blows, his shells; a teammate's blows, doors he has hammered
    fn big_counts(&self) -> (i32, i32, i32, i32);
    /// is a man's own gun in his hands (no tool out on a sling)
    fn own_gun_up(&self, gd: GuardId) -> bool;
    /// how many blows the frame asks of the hammer
    fn frame_blows(&self) -> f64;
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
        (m.state, m.awareness, m.react_t, m.fire_cd, m.last_known, m.last_seen_t, m.dazzled_until, m.pinned, m.hold, m.foe) =
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
        self.g.puppet = Some(game::game::GamePuppet { goal: Some([at[0], 0.0, at[1]]), aim: Some([aim[0], 1.2, aim[1]]), ..game::game::GamePuppet::zeroed() });
    }
    fn all_but_held(&mut self, gd: GuardId) {
        for o in self.g.guards.clone() { if o != gd { let m = &mut self.g.gd[o]; (m.hold, m.pinned, m.dazzled_until) = (true, true, 1e9); } }
    }
    fn aim_model(&mut self, on: bool) { self.g.tune.aim_model = on; }
    fn sam_round_at(&mut self, at: [f64; 3]) {
        let c = self.g.pl[self.g.player].ch;
        let o = game::combat::FireOpts { steady: Some(true), spread: Some(0.0), ..game::combat::FireOpts::zeroed() };
        game::combat::fire_weapon(&mut self.g, c, at, true, Some(&o));
    }

    fn sam_a_ghost(&mut self) { self.g.player_invisible = true; }
    fn sam_low(&mut self, low: bool) { let pl = self.g.player; self.g.pl[pl].crouch = low; }
    fn sam_walks_to(&mut self, goal: [f64; 2], low: bool) {
        self.g.puppet = Some(game::game::GamePuppet { goal: Some([goal[0], 0.0, goal[1]]), crouch: Some(low), ..game::game::GamePuppet::zeroed() });
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
        m.script = Some(game::game::GuardScript { goal: Some([goal[0], 0.0, goal[1]]), speed: Some(speed), upper: Some(game::character::UpperMode::Relaxed), ..game::game::GuardScript::zeroed() });
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
    fn breathing(&self, gd: GuardId) -> bool { let c = self.g.gd[gd].ch; self.g.gd[gd].state == GuardState::Dead && !self.g.ch[c].alive && matches!(self.g.gd[gd].down_kind, Some(game::game::DownKind::Struck) | Some(game::game::DownKind::Choked)) }
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
    fn folded(&self, gd: GuardId) -> f64 { let m = &self.g.gd[gd]; (m.stagger_until.min(m.dazzled_until) - self.g.time).max(0.0) }
    fn door_on_men(&self) -> (i32, i32, i32) { let t = &self.g.stats.doorthrow; (t.startled, t.carried, t.pushed + t.kicked) }
    fn cans(&self) -> Vec<[f64; 3]> { self.g.items.items.iter().filter(|&&i| game::throwables::item_kind(&self.g, i) == game::throwables::ItemKind::Flash).map(|&i| game::throwables::item_pos(&self.g, i)).collect() }
    fn last_bang(&self) -> Option<[f64; 3]> { self.g.events.iter().rev().find(|e| e.kind == game::director::EventKind::Bang).map(|e| e.pos) }
    fn sam_speed(&self) -> f64 { let v = self.g.ch[self.g.pl[self.g.player].ch].vel; v[0].hypot(v[2]) }
    fn any_blind(&self, men: &[GuardId]) -> bool { men.iter().any(|&m| self.blind(m)) }
    fn card_pace(&self, gd: GuardId) -> f64 { game::kitload::load_pace_on(&self.g, &game::seats::here(&self.g), game::kitload::LoadPaceOnWho::Guard(gd)) }
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
    fn big_counts(&self) -> (i32, i32, i32, i32) { let b = &self.g.stats.bigkit; (b.sam_blows, b.sam_shells, b.mate_blows, b.mate_hammered) }
    fn own_gun_up(&self, gd: GuardId) -> bool { self.g.gd[gd].slung.as_ref().map_or(true, |s| !s.out) && self.g.ch[self.g.gd[gd].ch].anim.big_tool.is_none() }
    fn frame_blows(&self) -> f64 { self.g.k.bigslot.swings }
    fn on_a_knee(&self, gd: GuardId) -> bool { self.g.gd[gd].door_down_until > self.g.time }
    fn walking_to_boot(&self, gd: GuardId) -> bool { self.g.m.doorforce.states.as_ref().and_then(|s| s.men.get(gd)).and_then(|m| m.kick.as_ref()).map_or(false, |k| k.phase == game::doorforce::KickPhase::Walk) }
    fn on_a_door(&self, gd: GuardId) -> bool { self.g.m.doorforce.states.as_ref().and_then(|s| s.men.get(gd)).map_or(false, |m| m.on.is_some()) }
    fn head(&self, gd: GuardId) -> [f64; 3] { let c = self.g.gd[gd].ch; self.g.ch[c].bones.get(game::character::Bone::Head).copied().expect("a man has a head") }
    fn out_cold(&self, gd: GuardId) -> bool { self.g.gd[gd].state == GuardState::Dead && game::game::is_breathing(&self.g, gd) }
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
