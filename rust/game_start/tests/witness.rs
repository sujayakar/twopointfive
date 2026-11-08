//! WHO KNOWS WHAT: THE WATCHER SAW IT, THE MAN AROUND THE CORNER HEARD IT, THE MAN ON THE LOT KNOWS NOTHING BUT THE CALL.
mod world_ask;
use game::game::{is_breathing, DownKind, GuardState, WitnessSawActKind};
use game::ids::GuardId;
use game_start::play::{Button, Opts, Play};
use world_ask::{YardAsk};

const CORRIDOR: [f64; 2] = [14.0, 11.2];
const LOT: [f64; 2] = [37.5, 6.0];
/// The office by night: A mid corridor with his back to Sam, who stands a pace behind him; B six metres on, facing them (or out on the lot); C out on the lot with walls between. The rest parked far away.
fn stage(s: u32, watching: bool) -> (Play, GuardId, GuardId, GuardId) {
    let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.08);
    let men = p.g.guards.clone();
    for (k, &m) in men.iter().enumerate() { p.parked_far(m, k); }
    let (a, b, c) = (men[0], men[1], men[2]);
    p.posted_calm(a, CORRIDOR, [CORRIDOR[0] + 3.0, CORRIDOR[1]]);
    if watching { p.posted_calm(b, [20.0, 11.2], CORRIDOR); } else { p.posted_calm(b, [LOT[0], LOT[1] + 1.5], [LOT[0] + 3.0, LOT[1] + 1.5]); }
    p.posted_calm(c, LOT, [LOT[0] + 3.0, LOT[1]]);
    p.sam_put([13.2, 11.2]);
    p.g.player_invisible = true;   // (Sam stands under a corridor panel: what B has eyes for is the man, not Sam, but for the moment of the act)
    p.run(0.8);   // (a man's head comes to where he looks)
    (p, a, b, c)
}
fn shoot_until_down(p: &mut Play, a: GuardId, key: Option<&str>) {
    if let Some(k) = key { p.key_down(k); p.frame(); p.key_up(k); } else { p.sam_takes_his_rifle(); }
    let q = p.at(a);
    p.point_at(q);
    p.run(0.5);
    p.g.player_invisible = false;
    for _ in 0..12 {
        p.press(Button::Left); p.frame(); p.frame(); p.release(Button::Left);
        if p.state(a) == GuardState::Dead { p.g.player_invisible = true; return; }
        p.run(0.2);
        if p.state(a) == GuardState::Dead { p.g.player_invisible = true; return; }
    }
    panic!("a dozen rounds and he stands: {}", p.roll());
}

/// THE RULE: Sam's suppressed pistol drops A: B, six metres off with A in his eye, saw it and heard an odd noise, not a shot; C on the lot heard nothing, and knows only what B calls in. CONTROL: the same man dropped by the loud rifle: both heard shots, C on the lot too.
/// A FAILURE MEANS: men know what they never perceived: the far man answers a shot he could not hear, or a loud shot is a secret.
#[test]
fn the_watcher_saw_it_and_the_man_on_the_lot_knows_only_the_call() {
    for s in [401u32, 402] {
        let (mut p, a, b, c) = stage(s, true);
        shoot_until_down(&mut p, a, Some("Digit1"));
        let victim = p.g.gd[a].callsign.clone();
        p.run(1.5);
        let (wb, wc) = (p.g.gd[b].witness.clone(), p.g.gd[c].witness.clone());
        assert!(wb.saw_act.as_ref().is_some_and(|w| w.kind == WitnessSawActKind::Shot && w.victim == victim), "seed {s}: B saw {:?}", wb.saw_act);
        assert!(wb.heard.small >= 1.0 && wb.heard.shots == 0.0 && wb.peak_awareness < 0.9, "seed {s}: B heard small {} shots {}, peak {}", wb.heard.small, wb.heard.shots, wb.peak_awareness);
        assert!(wb.saw_body.as_ref().is_some_and(|w| !w.breathing && w.victim == victim), "seed {s}: B's body {:?}", wb.saw_body);
        assert!(wc.saw_act.is_none() && wc.heard.shots == 0.0 && wc.heard.small == 0.0 && wc.heard.bang == 0.0 && wc.peak_awareness < 0.9, "seed {s}: C perceived it: {wc:?}");
        assert!(wc.called_to_body.as_ref().is_some_and(|w| !w.breathing && w.victim == victim), "seed {s}: C's call {:?}", wc.called_to_body);
        assert!(p.g.gd[a].witness.saw_act.is_none(), "seed {s}: the man himself saw his own death");
        drop(p);

        let (mut p, a, b, c) = stage(s, true);
        shoot_until_down(&mut p, a, None);
        p.run(0.5);
        assert!(p.g.gd[b].witness.heard.shots >= 1.0 && p.g.gd[c].witness.heard.shots >= 1.0, "seed {s}: the rifle: B heard {} shots, C {}", p.g.gd[b].witness.heard.shots, p.g.gd[c].witness.heard.shots);
    }
}

/// THE RULE: a quiet takedown with B watching: he saw A dropped by a blow, then found him breathing; C knows only the call, breathing too. CONTROL: the same takedown with nobody looking and nobody near: both records stay as they were.
/// A FAILURE MEANS: a blow in the dark reaches the whole floor, or a man found breathing is called in dead.
#[test]
fn a_takedown_is_known_only_to_who_watched_it() {
    for s in [411u32, 412] {
        let (mut p, a, b, c) = stage(s, true);
        p.g.player_invisible = false;
        game::player::start_takedown(&mut p.g, a, None, None, None);
        p.until(1.5, |p| p.state(a) == GuardState::Dead).unwrap_or_else(|| panic!("seed {s}: the blow did not land"));
        p.g.player_invisible = true;
        let victim = p.g.gd[a].callsign.clone();
        p.run(1.5);
        assert!(p.g.gd[a].down_kind == Some(DownKind::Struck) && is_breathing(&p.g, a), "seed {s}: {:?}", p.g.gd[a].down_kind);
        let (wb, wc) = (p.g.gd[b].witness.clone(), p.g.gd[c].witness.clone());
        assert!(wb.saw_act.as_ref().is_some_and(|w| w.kind == WitnessSawActKind::Struck && w.victim == victim), "seed {s}: B saw {:?}", wb.saw_act);
        assert!(wb.saw_body.as_ref().is_some_and(|w| w.breathing), "seed {s}: B's body {:?}", wb.saw_body);
        assert!(wc.saw_act.is_none() && wc.called_to_body.as_ref().is_some_and(|w| w.breathing), "seed {s}: C: {:?} {:?}", wc.saw_act, wc.called_to_body);
        drop(p);

        let (mut p, a, b, c) = stage(s, false);
        p.g.player_invisible = false;
        game::player::start_takedown(&mut p.g, a, None, None, None);
        p.until(1.5, |p| p.state(a) == GuardState::Dead).unwrap_or_else(|| panic!("seed {s}: the blow did not land"));
        p.run(6.0);
        for (who, m) in [("B", b), ("C", c)] {
            let w = &p.g.gd[m].witness;
            assert!(w.saw_act.is_none() && w.saw_body.is_none() && w.called_to_body.is_none() && w.heard.small == 0.0 && w.heard.shots == 0.0 && w.peak_awareness < 0.02, "seed {s}: {who} on the lot knows something: {w:?}");
        }
    }
}

/// THE RULE: struck and choked are out cold and breathing, a knockout on the card; shot is dead, a kill. CONTROL: the same man, each way.
/// A FAILURE MEANS: the record cannot tell who lives, and the card counts wrong.
#[test]
fn struck_and_choked_are_out_cold_and_shot_is_dead() {
    for (kind, breathing, ko, kills) in [(DownKind::Struck, true, 1.0, 0.0), (DownKind::Choked, true, 1.0, 0.0), (DownKind::Shot, false, 0.0, 1.0)] {
        let (mut p, a, _, _) = stage(5, false);
        p.falls_as(a, kind, true, game::game::DownBy::Player);
        p.frame();
        assert_eq!(is_breathing(&p.g, a), breathing, "{kind:?}: breathing");
        let stats = &p.g.mission.stats;
        assert_eq!((stats.knockouts, stats.kills), (ko, kills), "{kind:?}: on the card");
    }
}
