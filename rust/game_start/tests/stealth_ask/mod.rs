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
        game::doors::door_set_latched(&mut self.g, d, false);
        game::doors::door_set_angle(&mut self.g, d, turn);
    }
    fn hold_phase(&self) -> Option<game::character::HoldPhase> { self.g.pl[self.g.player].holding.as_ref().map(|h| h.phase) }
    fn in_arm(&self) -> Option<GuardId> { self.g.pl[self.g.player].holding.as_ref().map(|h| h.g) }
    fn out_cold(&self, gd: GuardId) -> bool { self.state(gd) == game::game::GuardState::Dead && game::game::is_breathing(&self.g, gd) }
    fn down_kind(&self, gd: GuardId) -> Option<game::game::DownKind> { self.g.gd[gd].down_kind }
    fn grab_row(&self, gd: GuardId) -> bool { self.g.nearby.iter().any(|r| r.guard == Some(gd) && r.verb == Some(game::game::Verb::Grab)) }
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
    fn dropped_gun(&mut self, gd: GuardId) -> Option<(bool, bool)> { let i = self.g.gd[gd].dropped?; Some((game::throwables::item_carried(&self.g, i) == Some(true), game::throwables::item_settled(&mut self.g, i))) }
    fn response(&self) -> game::qrf::QrfState { self.g.qrf.state }
    fn response_reason(&self) -> Option<game::qrf::QrfReason> { self.g.qrf.reason }
    fn response_men(&self) -> Vec<GuardId> { self.g.qrf.men.iter().copied().collect() }
    fn response_on_clock(&mut self, men: f64, eta: f64) {
        self.g.tune.qrf = true;
        self.g.tune.qrf_eta = eta;
        self.g.tune.qrf_size = men;
        if let Some(r) = self.g.level.scenarios.as_mut().and_then(|s| s.response.as_mut()) { r.men = Some([men, men]); r.eta = Some([eta, eta]); }
    }
    fn alarm_raised_to(&mut self, level: f64) { game::guards::raise_escalation(&mut self.g, level, None, None, None); }
    fn body_is_found(&mut self, gd: GuardId) { self.g.gd[gd].found = true; }
    fn gunfire_heard_ago(&mut self, secs: f64) { self.g.qrf.loud_t = self.g.time - secs; }
    fn response_called_again(&mut self) -> bool { self.g.qrf.refused_at = -100.0; game::qrf::request_qrf(&mut self.g, game::qrf::QrfReason::Gunfire) }
    fn last_stand_checks(&self) -> f64 { self.g.m.laststand.states.as_ref().map_or(0.0, |s| s.stats.checks) }
    fn last_stand_lines(&self) -> Vec<(String, String)> { self.g.m.laststand.states.as_ref().map_or(Vec::new(), |s| s.said.iter().map(|l| (l.who.clone(), l.text.clone())).collect()) }
    fn breaks(&mut self, gd: GuardId) -> bool { game::runner::runner_break(&mut self.g, gd) }
    fn fled(&self, gd: GuardId) -> bool { self.g.gd[gd].fled }
}
