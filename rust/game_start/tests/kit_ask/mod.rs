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
