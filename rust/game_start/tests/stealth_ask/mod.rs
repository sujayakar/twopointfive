//! Questions the enemy and stealth tests ask of a stood game that `play.rs` has no reader for. Each answers one thing, by its own name, so that
//! when a field moves one line here follows it.
#![allow(dead_code)]
use game::ids::*;
use game_start::play::{Button, Play};

pub trait StealthAsk {
    /// Sam gives his low whistle: the key, down for one frame
    fn sam_whistles(&mut self);
    /// has this man said anything on the net so far
    fn spoke_on_radio(&self, gd: GuardId) -> bool;
    /// the door of this name stands wide open, on the side it swings to
    fn door_stands_open(&mut self, name: &str);
    /// the phase of the hold Sam has a man in, if any
    fn hold_phase(&self) -> Option<game::character::HoldPhase>;
    /// the man in Sam's arm
    fn in_arm(&self) -> Option<GuardId>;
    /// is this man down as out cold and breathing
    fn out_cold(&self, gd: GuardId) -> bool;
    /// how a man went down
    fn down_kind(&self, gd: GuardId) -> Option<game::game::DownKind>;
    /// is the grab him row on offer to Sam on this man, this frame
    fn grab_row(&self, gd: GuardId) -> bool;
    /// the seconds the choke runs once the arm is closed
    fn choke_secs(&self) -> f64;
    /// the seconds a grab takes to land
    fn grab_secs(&self) -> f64;
    /// Sam takes a man by the hand: the right button kept down until he has him, and let go
    fn sam_takes(&mut self, gd: GuardId) -> bool;
    /// the radio check falls due `secs` from now (its clock runs a minute and more: this is a test of what it finds)
    fn radio_check_due(&mut self, secs: f64);
    /// is this man off the net as the radio check counts it
    fn off_the_net(&self, gd: GuardId) -> bool;
    /// the man out walking this man's route, if any
    fn checker_of(&self, gd: GuardId) -> Option<GuardId>;
    /// has somebody found this man's body
    fn body_found(&self, gd: GuardId) -> bool;
    /// the floor's alarm level
    fn alarm(&self) -> f64;
    /// the man the hard eyes of a fight have this teammate on, handed to him as the fight hands it
    fn eyes_on(&mut self, mate: GuardId, tgt: GuardId);
    /// is he on a man the place lights, from the dark (asked of the rule itself)
    fn gun_on_lit_man(&mut self, mate: GuardId) -> bool;
    /// does the light rule burn his weapon light this frame (asked of the rule itself)
    fn team_lit(&mut self, mate: GuardId) -> bool;
    /// is the light on his gun lit, as last frame posed it
    fn gun_light_on(&self, gd: GuardId) -> bool;
    /// the rule has him dark: his light is held off
    fn held_dark(&self, gd: GuardId) -> bool;
    /// are his goggles down
    fn goggles_down(&self, gd: GuardId) -> bool;
    /// the man his eyes are on, as a foe, in words for a message
    fn has_foe(&self, gd: GuardId) -> bool;
    /// did his eyes have Sam or a foe, this frame
    fn saw_his_man(&self, gd: GuardId) -> bool;
    /// a free man of a side stood here, looking this way, his light as said
    fn man_of(&mut self, team: game::teams::Team, at: [f64; 2], yaw: f64) -> GuardId;
    /// how many shots this man has heard
    fn shots_heard(&self, gd: GuardId) -> f64;
    /// what is in this man's hand
    fn hand_of(&self, gd: GuardId) -> Option<game::guards::InHand>;
    /// the thing he dropped, if he has: is it still carried, and has it come to rest
    fn dropped_gun(&mut self, gd: GuardId) -> Option<(bool, bool)>;
    /// the response team's state
    fn response(&self) -> game::qrf::QrfState;
    /// why it was called
    fn response_reason(&self) -> Option<game::qrf::QrfReason>;
    /// the men of the team, living or not
    fn response_men(&self) -> Vec<GuardId>;
    /// the mission's own team pinned to a count and a clock, so that a test holds the clock and not the deal
    fn response_on_clock(&mut self, men: f64, eta: f64);
    /// the floor's alarm raised by the game's own call
    fn alarm_raised_to(&mut self, level: f64);
    /// somebody has found this body
    fn body_is_found(&mut self, gd: GuardId);
    /// the trigger's memory of gunfire was this long ago
    fn gunfire_heard_ago(&mut self, secs: f64);
    /// another call for the team: what the game answers
    fn response_called_again(&mut self) -> bool;
    /// the men who have said the last few are holding and who is known up (the last stand's report)
    fn last_stand_checks(&self) -> f64;
    /// the lines the last stand's net has had, oldest first
    fn last_stand_lines(&self) -> Vec<(String, String)>;
    /// he breaks: the runner's own call: whether he ran
    fn breaks(&mut self, gd: GuardId) -> bool;
    /// has he dropped his gun and run
    fn fled(&self, gd: GuardId) -> bool;
    /// Sam walks to a place by the keys, as a hand does: whether he got there in time
    fn sam_goes_to(&mut self, at: [f64; 2], secs: f64) -> bool;
    /// the fog of war is on
    fn fog_up(&mut self);
    /// does the fog hide this man from the team's picture
    fn fogged(&self, gd: GuardId) -> bool;
    /// the things the fog has kept in the picture where they were last seen: (where, is it kept as it was)
    fn things_kept(&self) -> Vec<([f64; 2], bool)>;
    /// the furniture of the floor
    fn furniture(&self) -> Vec<game::ids::PropId>;
    /// where a piece of furniture stands
    fn prop_at(&self, it: game::ids::PropId) -> [f64; 2];
    /// a piece of furniture shoved to a new place, by nobody's hand
    fn prop_shoved(&mut self, it: game::ids::PropId, at: [f64; 2]);
    /// is what this doer does shown under the fog
    fn ours(&self, who: Option<f64>) -> bool;
    /// the middle of a door's frame
    fn door_at(&self, name: &str) -> [f64; 2];
    /// the door of this name is shut and latched
    fn door_shut(&mut self, name: &str);
    /// is a place in the funnel a man at `from` looks through a door by (the plan's lines from him through the leaf's two ends)
    fn in_doors_line(&self, name: &str, from: [f64; 2], at: [f64; 2]) -> bool;
    /// the pair goes silent, as a quiet mission has it
    fn pair_silent(&mut self);
    /// would this man's eyes take that man in, this frame
    fn eyes_take_in(&mut self, h: GuardId, m: GuardId) -> bool;
    /// the free floor nearest a place, within a radius
    fn free_near(&self, at: [f64; 2], r: f64) -> [f64; 2];
    /// the row of this kind and verb has the highlight, and the key takes it: whether there was one
    fn row_taken(&mut self, kind: game::game::InteractableKind, verb: game::game::Verb) -> bool;
    /// the stage the mission stands at, in the game's own word
    fn mission_stage(&self) -> game::game::MissionStage;
    /// the quiet mission's deal: start, spot, patrol
    fn quiet_deal(&self) -> (f64, f64, f64);
    /// a wall the camera has a line to this man from: where on it, its outward normal, and a place to send it from
    fn wall_for(&mut self, gd: GuardId) -> Option<([f64; 3], [f64; 3], [f64; 3])>;
    /// a camera sent off the launcher's own call, flown until it comes to rest: whether it is on a wall
    fn camera_sent(&mut self, from: [f64; 3], to: [f64; 3]) -> bool;
    /// the eyes under the fog just now
    fn fog_eyes(&self) -> usize;
    /// does the fog hold this man as seen
    fn fog_saw(&self, gd: GuardId) -> bool;
    /// is this place inside the wedge of the newest eye
    fn in_newest_wedge(&self, at: [f64; 3]) -> bool;
    /// cameras out and on the launcher
    fn cams(&self) -> (usize, f64);
    /// Sam's hands have his wand; he is told to look under this door
    fn lens_through(&self) -> Option<bool>;
    /// is his eye the lens
    fn eye_is_lens(&self) -> bool;
    /// what the smoke break has counted
    fn smoke_counts(&mut self) -> game::smokebreak::SmokeBreakStats;
    /// how far a bone of a man is from his head
    fn hand_from_head(&self, gd: GuardId, bone: game::character::Bone) -> f64;
    /// a bone's place
    fn bone_at(&self, gd: GuardId, bone: game::character::Bone) -> [f64; 3];
    /// does he hold the round on this man (the silent pair's fire rule)
    fn holds_round(&mut self, m: GuardId, h: GuardId) -> bool;
    /// the picture's leaf of this door, against where it stands: (it is kept back, standing, shown)
    fn leaf(&self, name: &str) -> (bool, f64, f64);
    /// the doors of the floor by name
    fn door_names(&self) -> Vec<String>;
    /// the partner is ordered to the breaker, by the menu's own call
    fn throws_the_breaker(&mut self);
    /// are the mains out
    fn mains_out(&self) -> bool;
    /// the man the panel's going dark has sent to it
    fn sent_to_the_breaker(&self) -> Option<GuardId>;
    /// where a man's errand takes him
    fn errand_at(&self, gd: GuardId) -> [f64; 3];
    /// no errand: he walks to that place as a man who heard something there
    fn walks_to_a_sound(&mut self, gd: GuardId, spot: [f64; 3]);
    /// one frame of Sam walking on toward a place by the keys: false once he is there
    fn sam_goes_to_step(&mut self, at: [f64; 2]) -> bool;
    /// is the follower on that man's walk, as the rule asks it
    fn follower_on_walk(&mut self, b: GuardId, man: GuardId, to_a_sound: bool) -> bool;
    /// a man out of it: held where he is, blind and deaf
    fn parked(&mut self, gd: GuardId);
    /// a man let go of the pin a test held him by, free again from here on, eyes and ears open
    fn let_go(&mut self, gd: GuardId);
    /// a round goes `off` metres past this man's chest, from `from` metres in front of him to six behind, by this shooter's hand
    fn round_past(&mut self, shooter: game::ids::CharId, man: GuardId, off: f64, from: f64);
    /// how much of the duck he shows
    fn duck(&self, gd: GuardId) -> f64;
    /// has the start hold of him
    fn startled(&mut self, gd: GuardId) -> bool;
    /// is his gun up
    fn gun_up(&self, gd: GuardId) -> bool;
    /// a civilian's state
    fn civ(&self, gd: GuardId) -> game::civilians::Civ;
    /// asleep in his chair
    fn asleep(&self, gd: GuardId) -> bool;
    /// is he in the chair
    fn seated(&self, gd: GuardId) -> bool;
    /// one of Sam's footfalls at a place, at a level
    fn footfall(&mut self, at: [f64; 2], level: f64);
    /// the trip a man is on
    fn trip(&mut self, gd: GuardId) -> Option<game::needs::Trip>;
    /// alert on a place with no sight of anybody, free to move
    fn alert_on_fix(&mut self, gd: GuardId, fix: [f64; 2]);
    /// the plans of the pursuit: (members, each member's kind and bearing, the phase)
    fn pursuit(&self) -> Vec<(Vec<(GuardId, game::tactics::RoleKind, f64)>, game::tactics::PlanPhase)>;
    /// is this door marked dangerous for his side
    fn door_dangerous(&mut self, name: &str, team: game::teams::Team) -> bool;
    /// the phase of the door drill a man is in
    fn at_the_gate(&self, gd: GuardId) -> Option<game::tactics::GatePhase>;
    /// a squad's events so far: what, and when
    fn squad_events(&self, name: &str) -> Vec<(game::enemysquads::SquadEventWhat, f64)>;
    /// how a man's run stands: (he has run at all, its phase word)
    fn run_of(&self, gd: GuardId) -> Option<game::runner::Run>;
    /// the camera's rest: on a wall, and how high
    fn last_cam(&self) -> ([f64; 3], f64, bool);
    /// a staggered man
    fn staggered(&self, gd: GuardId) -> bool;
    /// a loud report nobody can place, at Sam's boots
    fn loud_report(&mut self);
    /// a squad's men and what it has sent
    fn squad_men(&self, name: &str) -> (Vec<GuardId>, usize);
    /// the squads' names
    fn squad_names(&self) -> Vec<String>;
    /// does this place lie in that room of that squad
    fn in_squad_rooms(&self, name: &str, at: [f64; 2]) -> bool;
    /// the wand's jobs: (the man, the door he was ordered to)
    fn wand_jobs(&self) -> Vec<GuardId>;
    /// the eyes that are teammates' lenses
    fn mates_lenses(&mut self) -> usize;
}

impl StealthAsk for Play {
    fn sam_whistles(&mut self) {
        self.key_down("KeyV");
        self.frame();
        self.key_up("KeyV");
    }
    fn spoke_on_radio(&self, gd: GuardId) -> bool { self.said.iter().any(|l| l.who == gd && l.radio) }
    fn door_stands_open(&mut self, name: &str) {
        let d = game::doors::doors_by_name(&self.g, name).unwrap_or_else(|| panic!("no door '{name}'"));
        let turn = game::doors::door_swing(&mut self.g, d) * 1.4;
        self.g.dr[d].leaf.latched = false;
        self.g.dr[d].leaf.angle = turn;
    }
    fn hold_phase(&self) -> Option<game::character::HoldPhase> { self.g.pl[self.g.player].holding().map(|h| h.phase) }
    fn in_arm(&self) -> Option<GuardId> { self.g.pl[self.g.player].holding().map(|h| h.g) }
    fn out_cold(&self, gd: GuardId) -> bool { self.state(gd) == game::game::GuardState::Dead && game::game::is_breathing(&self.g, gd) }
    fn down_kind(&self, gd: GuardId) -> Option<game::game::DownKind> { self.g.gd[gd].down_kind }
    fn grab_row(&self, gd: GuardId) -> bool { self.g.sets[self.g.seat].nearby.iter().any(|r| r.guard == Some(gd) && r.verb == Some(game::game::Verb::Grab)) }
    fn choke_secs(&self) -> f64 { self.g.k.hold.secs.choke }
    fn grab_secs(&self) -> f64 { self.g.k.hold.secs.grab }
    fn sam_takes(&mut self, gd: GuardId) -> bool {
        self.press(Button::Right);
        let took = self.until(1.0, |p| p.in_arm() == Some(gd)).is_some();
        self.release(Button::Right);
        took && self.until(self.grab_secs() + 0.5, |p| p.hold_phase() == Some(game::character::HoldPhase::Held)).is_some()
    }
    fn radio_check_due(&mut self, secs: f64) { let at = self.g.time + secs; game::guards::rollcall_state(&mut self.g).next_at = at; }
    fn off_the_net(&self, gd: GuardId) -> bool { game::guards::off_net(&self.g, gd) }
    fn checker_of(&self, gd: GuardId) -> Option<GuardId> { game::guards::checker_for(&self.g, gd) }
    fn body_found(&self, gd: GuardId) -> bool { self.g.gd[gd].found }
    fn alarm(&self) -> f64 { self.g.escalation }
    fn eyes_on(&mut self, mate: GuardId, tgt: GuardId) {
        let o = game::roomfight::room_fight_op_of(&self.g, mate).expect("a man of the fight");
        self.g.tab.roomfight_op[o].tgt = Some(tgt);
        self.g.time += 0.3;   // (the rule's answer is kept a quarter of a second per man: a question here is a fresh one)
    }
    fn gun_on_lit_man(&mut self, mate: GuardId) -> bool { game::guards::gun_on_lit_man(&mut self.g, mate) }
    fn team_lit(&mut self, mate: GuardId) -> bool { game::guards::team_lit(&mut self.g, mate) }
    fn gun_light_on(&self, gd: GuardId) -> bool { self.g.tab.light[self.g.gd[gd].flashlight].enabled }
    fn held_dark(&self, gd: GuardId) -> bool { game::nvg::nvg_dark(&self.g, gd) }
    fn goggles_down(&self, gd: GuardId) -> bool { game::nvg::nvg_down(&self.g, gd) }
    fn has_foe(&self, gd: GuardId) -> bool { self.g.gd[gd].foe.is_some() }
    fn saw_his_man(&self, gd: GuardId) -> bool { self.g.gd[gd].saw_foe_this_frame || self.g.gd[gd].saw_player_this_frame }
    fn man_of(&mut self, team: game::teams::Team, at: [f64; 2], yaw: f64) -> GuardId {
        game::guards::spawn_unit(&mut self.g, &game::guards::UnitOpts { team, pos: [at[0], 0.0, at[1]], yaw: Some(yaw), weapon: Some(game::weapons::WeaponId::Fn2000), light: Some(false), ..Default::default() }).expect("a man stood")
    }
    fn shots_heard(&self, gd: GuardId) -> f64 { self.g.gd[gd].witness.heard.shots }
    fn hand_of(&self, gd: GuardId) -> Option<game::guards::InHand> { game::guards::in_hand(&self.g, gd) }
    fn dropped_gun(&mut self, gd: GuardId) -> Option<(bool, bool)> { let i = self.g.gd[gd].dropped?; Some((self.g.tab.item[i].carried == Some(true), self.g.tab.item[i].settled)) }
    fn response(&self) -> game::qrf::QrfState { self.g.qrf.state }
    fn response_reason(&self) -> Option<game::qrf::QrfReason> { self.g.qrf.reason }
    fn response_men(&self) -> Vec<GuardId> { self.g.qrf.men.iter().copied().collect() }
    fn response_on_clock(&mut self, men: f64, eta: f64) {
        self.g.tune.qrf = true;
        self.g.tune.qrf_eta = eta;
        self.g.tune.qrf_size = men;
        if let Some(r) = std::sync::Arc::make_mut(&mut self.g.level).scenarios.as_mut().and_then(|s| s.response.as_mut()) { r.men = Some([men, men]); r.eta = Some([eta, eta]); }
    }
    fn alarm_raised_to(&mut self, level: f64) { game::guards::raise_escalation(&mut self.g, level, None, None, None); }
    fn body_is_found(&mut self, gd: GuardId) { self.g.gd[gd].found = true; }
    fn gunfire_heard_ago(&mut self, secs: f64) { self.g.qrf.loud_t = self.g.time - secs; }
    fn response_called_again(&mut self) -> bool { self.g.qrf.refused_at = -100.0; game::qrf::request_qrf(&mut self.g, game::qrf::QrfReason::Gunfire) }
    fn last_stand_checks(&self) -> f64 { self.g.m.laststand.states.as_ref().map_or(0.0, |s| s.stats.checks) }
    fn last_stand_lines(&self) -> Vec<(String, String)> { self.g.m.laststand.states.as_ref().map_or(Vec::new(), |s| s.said.iter().map(|l| (l.who.clone(), l.text.clone())).collect()) }
    fn breaks(&mut self, gd: GuardId) -> bool { game::runner::runner_break(&mut self.g, gd) }
    fn fled(&self, gd: GuardId) -> bool { self.g.gd[gd].fled }
    fn sam_goes_to(&mut self, at: [f64; 2], secs: f64) -> bool {
        let keys = ["KeyW", "KeyS", "KeyA", "KeyD"];
        let mut got = false;
        for _ in 0..(secs * 60.0) as usize {
            let (me, b) = (self.sam_at(), game::camera::follow_camera_planar_basis(&self.g, self.host.cam));
            let (dx, dz) = (at[0] - me[0], at[1] - me[1]);
            let d = (dx * dx + dz * dz).sqrt();
            if d < 0.15 { got = true; break; }
            let (a, r) = ((dx * b.fwd[0] + dz * b.fwd[2]) / d, (dx * b.right[0] + dz * b.right[2]) / d);
            for (k, on) in keys.iter().zip([a > 0.38, a < -0.38, r < -0.38, r > 0.38]) { if on { self.key_down(k); } else { self.key_up(k); } }
            self.frame();
        }
        for k in keys { self.key_up(k); }
        got
    }
    fn fog_up(&mut self) { self.g.fog_of_war.on = true; }
    fn fogged(&self, gd: GuardId) -> bool { game::fogofwar::fog_of_war_hidden(&self.g, gd) }
    fn things_kept(&self) -> Vec<([f64; 2], bool)> { self.g.fog_of_war.thing_recs.values().map(|r| ([r.x, r.z], r.held)).collect() }
    fn furniture(&self) -> Vec<game::ids::PropId> { self.g.props.props.iter().copied().collect() }
    fn prop_at(&self, it: game::ids::PropId) -> [f64; 2] { [self.g.tab.props_prop[it].x, self.g.tab.props_prop[it].z] }
    fn prop_shoved(&mut self, it: game::ids::PropId, at: [f64; 2]) {
        game::props::prop_move_to(&mut self.g, it, at[0], at[1]);
        self.g.tab.props_prop[it].dirty = true;
        game::props::prop_place(&mut self.g, it);
    }
    fn ours(&self, who: Option<f64>) -> bool { game::fogofwar::fog_of_war_ours_id(&self.g, who) }
    fn door_at(&self, name: &str) -> [f64; 2] { let d = game::doors::doors_by_name(&self.g, name).unwrap_or_else(|| panic!("no door '{name}'")); let c = self.g.dr[d].frame_centre; [c[0], c[2]] }
    fn door_shut(&mut self, name: &str) {
        let d = game::doors::doors_by_name(&self.g, name).unwrap_or_else(|| panic!("no door '{name}'"));
        self.g.dr[d].leaf.angle = 0.0;
        self.g.dr[d].leaf.latched = true;
    }
    fn in_doors_line(&self, name: &str, from: [f64; 2], at: [f64; 2]) -> bool {
        let d = game::doors::doors_by_name(&self.g, name).unwrap_or_else(|| panic!("no door '{name}'"));
        let (h, th, w) = (self.g.dr[d].def.hinge, self.g.dr[d].def.closed_dir, self.g.dr[d].def.width);
        let (a, b) = ([h[0], h[1]], [h[0] + th.cos() * w, h[1] + th.sin() * w]);
        let o = |p: [f64; 2], q: [f64; 2], r: [f64; 2]| (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0]);
        o(from, at, a) * o(from, at, b) <= 0.0 && o(a, b, from) * o(a, b, at) <= 0.0
    }
    fn eyes_take_in(&mut self, h: GuardId, m: GuardId) -> bool {
        let c = self.g.gd[m].ch;
        let v = game::guards::can_see_char(&mut self.g, h, c);
        v.visible && game::guards::sight_gain_on(&mut self.g, h, m, v.dist) > 0.05
    }
    fn free_near(&self, at: [f64; 2], r: f64) -> [f64; 2] {
        let mut best = None;
        let mut dx = -r;
        while dx <= r {
            let mut dz = -r;
            while dz <= r {
                let q = [at[0] + dx, at[1] + dz];
                let d = dx.hypot(dz);
                if !game::scene_collision::nav_grid_is_blocked(&self.g, q[0], q[1]) && best.map_or(true, |(b, _)| d < b) { best = Some((d, q)); }
                dz += 0.2;
            }
            dx += 0.2;
        }
        best.unwrap_or_else(|| panic!("no free floor within {r} m of {at:?}")).1
    }
    fn row_taken(&mut self, kind: game::game::InteractableKind, verb: game::game::Verb) -> bool {
        if !game::game::game_select_verb(&mut self.g, kind, Some(verb)) { return false; }
        self.key_down("Space");
        self.frame();
        self.key_up("Space");
        true
    }
    fn mission_stage(&self) -> game::game::MissionStage { self.g.mission.stage }
    fn quiet_deal(&self) -> (f64, f64, f64) { let d = &self.g.quiet.deal; (d.start, d.spot, d.patrol) }
    fn wall_for(&mut self, gd: GuardId) -> Option<([f64; 3], [f64; 3], [f64; 3])> {
        let (at, y, off) = (self.g.pos_of(gd), self.g.k.sticky.aim_y, self.g.k.sticky.off);
        let chest = [at[0], 1.1, at[2]];
        for k in 0..32 {
            let a = k as f64 * std::f64::consts::PI / 16.0;
            let (c, d) = ([at[0], y, at[2]], [a.sin(), 0.0, a.cos()]);
            let Some(h) = game::scene_collision::raycast(&mut self.g, c, d, 12.0) else { continue };
            if h.t < 2.2 || h.n[1].abs() > 0.3 || -(h.n[0] * d[0] + h.n[2] * d[2]) < 0.5 { continue; }
            let p = [c[0] + d[0] * h.t, c[1], c[2] + d[2] * h.t];
            let out = |k: f64| [p[0] + h.n[0] * k, p[1] + h.n[1] * k, p[2] + h.n[2] * k];
            if game::scene_collision::segment_blocked(&mut self.g, out(1.3), out(0.05), None, None, None) || game::scene_collision::segment_blocked(&mut self.g, out(off), chest, None, None, None) { continue; }
            return Some((p, h.n, out(1.3)));
        }
        None
    }
    fn camera_sent(&mut self, from: [f64; 3], to: [f64; 3]) -> bool {
        let pl = self.g.player;
        let c = game::stickycam::fire_cam(&mut self.g, pl, from, to);
        for _ in 0..180 { if self.g.tab.stickycam_sticky_cam[c].vel.is_none() { break; } self.frame(); }
        self.g.tab.stickycam_sticky_cam[c].vel.is_none() && self.g.tab.stickycam_sticky_cam[c].si >= 0.0
    }
    fn fog_eyes(&self) -> usize { self.g.fog_of_war.eyes.len() }
    fn fog_saw(&self, gd: GuardId) -> bool { game::fogofwar::fog_of_war_seen(&self.g, gd) }
    fn in_newest_wedge(&self, at: [f64; 3]) -> bool { let e = *self.g.fog_of_war.eyes.last().expect("an eye"); game::fogofwar::fog_of_war_in_field(&self.g, e, at) }
    fn cams(&self) -> (usize, f64) { let pl = &self.g.pl[self.g.player]; (pl.cams.len(), self.g.fa[pl.rifle].cams) }
    fn lens_through(&self) -> Option<bool> { self.g.pl[self.g.player].wanding().map(|w| w.live) }
    fn eye_is_lens(&self) -> bool { self.g.fog_of_war.eyes.iter().any(|e| e.wand == Some(true)) }
    fn smoke_counts(&mut self) -> game::smokebreak::SmokeBreakStats { game::smokebreak::state_of(&mut self.g).stats }
    fn hand_from_head(&self, gd: GuardId, bone: game::character::Bone) -> f64 {
        let (h, b) = (self.bone_at(gd, game::character::Bone::Head), self.bone_at(gd, bone));
        ((h[0] - b[0]).powi(2) + (h[1] - b[1]).powi(2) + (h[2] - b[2]).powi(2)).sqrt()
    }
    fn bone_at(&self, gd: GuardId, bone: game::character::Bone) -> [f64; 3] { *self.g.ch[self.g.gd[gd].ch].bones.get(bone).expect("the bone") }
    fn holds_round(&mut self, m: GuardId, h: GuardId) -> bool { game::spotted::spotted_holds_round(&mut self.g, m, h) }
    fn leaf(&self, name: &str) -> (bool, f64, f64) {
        let d = game::doors::doors_by_name(&self.g, name).unwrap_or_else(|| panic!("no door '{name}'"));
        (game::fogofwar::fog_of_war_door_held(&self.g, d), self.g.dr[d].leaf.angle, game::fogofwar::fog_of_war_door_shown(&self.g, d))
    }
    fn door_names(&self) -> Vec<String> { self.g.doors.list.iter().map(|&d| self.g.dr[d].def.name.clone()).collect() }
    fn throws_the_breaker(&mut self) {
        let seat = game::seats::here(&self.g);
        let th = game::workthings::work_things_things(&seat).into_iter().find(|x| x.kind == game::workthings::WorkThingKind::Breaker).expect("a breaker to work");
        let at = th.stand.expect("a place to stand at it");
        let t = game::orders::OrderTarget::Floor(game::orders::OrderTargetFloor { at, wall: None, thing: Some(th), serial: 0 });
        if let Some(why) = game::orders::squad_orders_give(&mut self.g, "work", Some(&t), Some(game::orders::Who::All), None) { panic!("the order was refused: {why}"); }
    }
    fn mains_out(&self) -> bool { self.g.light_home.mains.active }
    fn sent_to_the_breaker(&self) -> Option<GuardId> { self.g.guards.iter().copied().find(|&m| matches!(self.g.gd[m].task, Some(game::game::GuardTask::Arm0(game::game::GuardTaskArm0 { kind: game::game::GuardTaskArm0Kind::Breaker, .. })))) }
    fn errand_at(&self, gd: GuardId) -> [f64; 3] { match &self.g.gd[gd].task { Some(game::game::GuardTask::Arm0(k)) => k.pos, _ => panic!("no errand") } }
    fn walks_to_a_sound(&mut self, gd: GuardId, spot: [f64; 3]) {
        let floor = self.g.k.whistle.walk_floor;
        let (at, h) = (self.at(gd), &mut self.g.gd[gd]);
        if h.state == game::game::GuardState::Dead { return; }
        h.task = None;
        h.state = game::game::GuardState::Suspicious;
        h.last_known = Some(spot);
        h.awareness = if (at[0] - spot[0]).hypot(at[1] - spot[2]) > 1.2 { floor } else { h.awareness.min(floor) };
    }
    fn sam_goes_to_step(&mut self, at: [f64; 2]) -> bool {
        let keys = ["KeyW", "KeyS", "KeyA", "KeyD"];
        let (me, b) = (self.sam_at(), game::camera::follow_camera_planar_basis(&self.g, self.host.cam));
        let (dx, dz) = (at[0] - me[0], at[1] - me[1]);
        let d = dx.hypot(dz);
        if d < 0.3 { for k in keys { self.key_up(k); } return false; }
        let (a, r) = ((dx * b.fwd[0] + dz * b.fwd[2]) / d, (dx * b.right[0] + dz * b.right[2]) / d);
        for (k, on) in keys.iter().zip([a > 0.38, a < -0.38, r < -0.38, r > 0.38]) { if on { self.key_down(k); } else { self.key_up(k); } }
        self.frame();
        true
    }
    fn follower_on_walk(&mut self, b: GuardId, man: GuardId, to_a_sound: bool) -> bool {
        if to_a_sound { game::sneak::sneak_coming(&mut self.g, b, man) } else { game::workthings::work_coming(&mut self.g, b, man) }
    }
    fn parked(&mut self, gd: GuardId) {
        let h = &mut self.g.gd[gd];
        h.duty.holds = true;
        h.duty.pinned = true;
        self.g.cond[h.ch].blind_until = 1e9;
        h.heard_up_to = 1e12;
    }
    fn let_go(&mut self, gd: GuardId) {
        let seq = self.g.event_seq;
        let h = &mut self.g.gd[gd];
        h.duty.holds = false;
        h.duty.pinned = false;
        self.g.cond[h.ch].blind_until = -1.0;
        h.heard_up_to = seq;
    }
    fn round_past(&mut self, shooter: game::ids::CharId, man: GuardId, off: f64, from: f64) {
        let c = self.g.gd[man].ch;
        let (at, y) = (self.g.ch[c].pos, self.g.ch[c].body_yaw);
        let (fx, fz) = (y.sin(), y.cos());
        let (sx, sz) = (-fz, fx);
        let h = 1.1 + at[1];
        game::suppression::note_round(&mut self.g, shooter, [at[0] + fx * from + sx * off, h, at[2] + fz * from + sz * off], [at[0] - fx * 6.0 + sx * off, h, at[2] - fz * 6.0 + sz * off], None);
    }
    fn duck(&self, gd: GuardId) -> f64 { let m = &self.g.ch[self.g.gd[gd].ch].anim.mvs; if m.fl_t >= 0.0 { m.fl_k_to } else { 0.0 } }
    fn startled(&mut self, gd: GuardId) -> bool { game::startle::startled(&mut self.g, gd) }
    fn gun_up(&self, gd: GuardId) -> bool { self.g.ch[self.g.gd[gd].ch].anim.upper == game::character::UpperMode::Aim }
    fn civ(&self, gd: GuardId) -> game::civilians::Civ { self.g.cv[self.g.gd[gd].civ.expect("a civilian")].clone() }
    fn asleep(&self, gd: GuardId) -> bool { game::doze::doze_asleep(&self.g, gd) }
    fn seated(&self, gd: GuardId) -> bool { game::close::close_seated(&self.g, gd) }
    fn footfall(&mut self, at: [f64; 2], level: f64) {
        let (time, serial) = (self.g.time, self.g.serial());
        self.g.events.push(game::game::GameEvent { kind: game::director::EventKind::Step, pos: [at[0], 0.0, at[1]], time, loud: false, level: Some(level), serial, ..game::game::GameEvent::default() });
    }
    fn trip(&mut self, gd: GuardId) -> Option<game::needs::Trip> { game::needs::mem(&mut self.g, gd).trip.clone() }
    fn alert_on_fix(&mut self, gd: GuardId, fix: [f64; 2]) {
        self.let_go(gd);
        let h = &mut self.g.gd[gd];
        h.state = game::game::GuardState::Alert;
        h.awareness = 1.0;
        h.react_t = 0.0;
        self.g.arms[h.ch].fire_cd = 0.5;
        h.last_known = Some([fix[0], 0.0, fix[1]]);
        h.last_seen_t = -100.0;
        h.sight_t = -100.0;
        h.search_t = 0.0;
        h.task = None;
        h.script = None;
    }
    fn pursuit(&self) -> Vec<(Vec<(GuardId, game::tactics::RoleKind, f64)>, game::tactics::PlanPhase)> {
        game::tactics::plans_of(&self.g).into_iter().map(|id| {
            let plan = &self.g.tab.tactics_plan[id];
            (plan.members.iter().map(|&m| { let r = &self.g.tab.tactics_role[*plan.roles.get(m).expect("a role")]; (m, r.kind, r.bearing) }).collect(), plan.phase)
        }).collect()
    }
    fn door_dangerous(&mut self, name: &str, team: game::teams::Team) -> bool { let d = game::doors::doors_by_name(&self.g, name).expect("the door"); game::tactics::danger_at(&mut self.g, d, team).is_some() }
    fn at_the_gate(&self, gd: GuardId) -> Option<game::tactics::GatePhase> { self.g.m.tactics.states.as_ref().and_then(|s| s.gates.get(gd).map(|g| g.phase)) }
    fn squad_events(&self, name: &str) -> Vec<(game::enemysquads::SquadEventWhat, f64)> {
        let st = self.g.m.enemysquads.states.as_ref().expect("the level's squads");
        let q = st.squads.iter().copied().find(|&q| self.g.tab.enemysquads_squad[q].name == name).unwrap_or_else(|| panic!("no squad {name}"));
        self.g.tab.enemysquads_squad[q].events.iter().map(|e| (e.what, e.t)).collect()
    }
    fn run_of(&self, gd: GuardId) -> Option<game::runner::Run> { self.g.m.runner.state_of.get(gd).cloned() }
    fn last_cam(&self) -> ([f64; 3], f64, bool) { let pl = &self.g.pl[self.g.player]; let c = *pl.cams.last().expect("a camera out"); let r = &self.g.tab.stickycam_sticky_cam[c]; (r.pos, r.si, r.vel.is_none()) }
    fn staggered(&self, gd: GuardId) -> bool { self.g.time < self.g.cond[self.g.gd[gd].ch].fold_until }
    fn loud_report(&mut self) {
        let (at, time, serial, who) = (self.g.player_pos(self.g.player), self.g.time, self.g.serial(), self.g.player_id(self.g.player));
        self.g.events.push(game::game::GameEvent { kind: game::director::EventKind::Shot, pos: [at[0], 0.0, at[2]], time, loud: true, level: Some(1.0), who: Some(who), serial, ..game::game::GameEvent::default() });
    }
    fn squad_men(&self, name: &str) -> (Vec<GuardId>, usize) {
        let st = self.g.m.enemysquads.states.as_ref().expect("the level's squads");
        let q = st.squads.iter().copied().find(|&q| self.g.tab.enemysquads_squad[q].name == name).unwrap_or_else(|| panic!("no squad {name}"));
        let r = &self.g.tab.enemysquads_squad[q];
        (r.men.clone(), r.sent.len())
    }
    fn squad_names(&self) -> Vec<String> {
        let st = self.g.m.enemysquads.states.as_ref().expect("the level's squads");
        st.squads.iter().map(|&q| self.g.tab.enemysquads_squad[q].name.clone()).collect()
    }
    fn in_squad_rooms(&self, name: &str, at: [f64; 2]) -> bool {
        let st = self.g.m.enemysquads.states.as_ref().expect("the level's squads");
        let q = st.squads.iter().copied().find(|&q| self.g.tab.enemysquads_squad[q].name == name).unwrap_or_else(|| panic!("no squad {name}"));
        self.g.tab.enemysquads_squad[q].rooms.iter().any(|r| { let b = &r.bounds; at[0] >= b.x0 && at[0] <= b.x1 && at[1] >= b.z0 && at[1] <= b.z1 })
    }
    fn wand_jobs(&self) -> Vec<GuardId> {
        let k = game::seats::view_key(&self.g, &game::seats::here(&self.g));
        self.g.m.wandorder.states.get(k).map_or(Vec::new(), |st| st.jobs.keys())
    }
    fn mates_lenses(&mut self) -> usize { game::wandorder::mate_wand_eyes(&mut self.g).len() }
    fn pair_silent(&mut self) {
        self.g.k.silent.on = true;
        self.g.k.sneak.on = true;
        game::silent::set_silent(&mut self.g, true);
    }
}
