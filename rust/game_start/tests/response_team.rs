//! THE RESPONSE TEAM: what calls it and what does not, its clock, and that there is one wave a night.
mod stealth_ask;
use game::game::{DownBy, DownKind};
use game::qrf::{QrfReason, QrfState};
use game_start::play::{nudge, Button, Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
const ETA: f64 = 20.0;

/// The office on a night, with its response team, the team's size and clock pinned; Sam in the corridor (a metre along it by the seed).
fn night(s: u32) -> Play {
    let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.response_on_clock(4.0, ETA);
    p.run(0.3);
    p.sam_put([22.0 + nudge(s, 0), 11.2]);
    p.run(0.1);
    p
}
/// three rifle rounds into the corridor's east end
fn rifle_rounds(p: &mut Play) { for _ in 0..3 { p.sam_fires_at([35.5, 11.2], 0.2); } }
/// six rounds of the suppressed pistol at the same wall
fn pistol_rounds(p: &mut Play) {
    p.key_down("Digit1");
    p.run(0.1);
    p.key_up("Digit1");
    p.run(0.7);
    p.point_at([35.5, 11.2]);
    for _ in 0..6 { p.press(Button::Left); p.frame(); p.frame(); p.release(Button::Left); p.run(0.25); }
}

/// THE RULE: a locked-down floor that has heard Sam's rifle calls the team, on the net. The same rounds on a floor only heightened call nobody.
/// A FAILURE MEANS: going loud no longer brings the response, or it comes before the floor has locked down.
#[test]
fn a_locked_down_floor_that_heard_his_rifle_calls_the_team() {
    for s in SEEDS {
        let mut p = night(s);
        rifle_rounds(&mut p);
        p.alarm_raised_to(2.0);
        p.run(0.2);
        assert_ne!(p.response(), QrfState::Idle, "seed {s}: alarm {} and rifle fire heard: not called", p.alarm());
        assert_eq!(p.response_reason(), Some(QrfReason::Gunfire), "seed {s}");
        assert!(!p.view().radio.is_empty() || p.said.iter().any(|l| l.radio), "seed {s}: called on the net: {:?}", p.view().radio);
        drop(p);

        // CONTROL: only heightened
        let mut p = night(s);
        rifle_rounds(&mut p);
        p.alarm_raised_to(1.0);
        p.run(2.0);
        if p.alarm() < 2.0 { assert_eq!(p.response(), QrfState::Idle, "seed {s}: the floor at {} called the team", p.alarm()); }
    }
}

/// THE RULE: the suppressed pistol is an odd noise and never shots fired: emptied on a locked-down floor it calls nobody; one rifle round after it does.
/// A FAILURE MEANS: the quiet gun brings the team down on a stealth player.
#[test]
fn the_suppressed_pistol_does_not_call_the_team_and_one_rifle_round_does() {
    for s in SEEDS {
        let mut p = night(s);
        pistol_rounds(&mut p);
        p.alarm_raised_to(2.0);
        p.run(1.5);
        assert_eq!(p.response(), QrfState::Idle, "seed {s}: six rounds of the pistol called the team");
        p.sam_fires_at([35.5, 11.2], 0.2);
        p.run(0.2);
        assert_eq!((p.response() != QrfState::Idle, p.response_reason()), (true, Some(QrfReason::Gunfire)), "seed {s}: one rifle round after it");
    }
}

/// THE RULE: bodies with no shots: one found on a locked-down floor is nothing, a second found calls the team.
/// A FAILURE MEANS: one body brings the team, or no number of them does.
#[test]
fn two_bodies_found_on_a_locked_down_floor_call_the_team() {
    for s in SEEDS {
        let mut p = night(s);
        let men = p.g.guards.clone();
        p.falls_as(men[0], DownKind::Struck, true, DownBy::Player);
        p.body_is_found(men[0]);
        p.alarm_raised_to(2.0);
        p.run(1.5);
        assert_eq!(p.response(), QrfState::Idle, "seed {s}: one body called the team");
        p.falls_as(men[1], DownKind::Struck, true, DownBy::Player);
        p.body_is_found(men[1]);
        p.run(0.2);
        assert_eq!((p.response() != QrfState::Idle, p.response_reason()), (true, Some(QrfReason::Bodies)), "seed {s}: two bodies");
    }
}

/// THE RULE: quiet work never starts it. All the shift put out cold with no gunfire heard: fifteen seconds on, past when a silent net would call it, the team is idle. The same with gunfire last heard
/// 200 s before (the trigger's memory is 180): idle. CONTROL: with gunfire just heard, a net gone silent brings it by itself.
/// A FAILURE MEANS: a clean stealth run ends with a response team, or a player who shot everybody meets nobody (the fault the rule was made for: "control
/// silent with gunfire in its ears, a minute in an empty building").
#[test]
fn quiet_knockouts_and_old_gunfire_call_nobody_and_a_silent_net_after_shots_does() {
    for s in SEEDS {
        for (heard_ago, called) in [(None, false), (Some(200.0), false), (Some(0.0), true)] {
            let mut p = night(s);
            if let Some(a) = heard_ago { p.gunfire_heard_ago(a); }
            for m in p.g.guards.clone() { p.falls_as(m, DownKind::Struck, true, DownBy::Player); }
            let t = p.until(15.0, |p| p.response() != QrfState::Idle);
            assert_eq!(t.is_some(), called, "seed {s}, gunfire heard {heard_ago:?} s ago: the team is {:?} ({:?})", p.response(), p.response_reason());
            if called { assert_eq!(p.response_reason(), Some(QrfReason::Silent), "seed {s}"); }
        }
    }
}

/// THE RULE: the response comes on its clock as one element: requested at once, control's answer a beat later, on the landing ten seconds out, deployed at its
/// time, four men and no more.
/// A FAILURE MEANS: the player's clock is no longer what the radio told him.
#[test]
fn the_response_comes_on_its_clock_as_one_element() {
    for s in SEEDS {
        let mut p = night(s);
        rifle_rounds(&mut p);
        p.alarm_raised_to(2.0);
        p.until(1.0, |p| p.response() == QrfState::Requested).unwrap_or_else(|| panic!("seed {s}: not called: {:?}", p.response()));
        let t0 = p.time();
        let t = |p: &mut Play, st: QrfState| p.until(ETA + 2.0, |p| p.response() == st).map(|_| p.time() - t0).unwrap_or_else(|| panic!("seed {s}: never {st:?}: {:?}", p.response()));
        let (inbound, arriving, deployed) = (t(&mut p, QrfState::Inbound), t(&mut p, QrfState::Arriving), t(&mut p, QrfState::Deployed));
        assert!(inbound > 0.5 && inbound < 3.0, "seed {s}: control answered at {inbound:.2} s");
        assert!((arriving - (ETA - 10.0)).abs() < 0.5, "seed {s}: on the landing at {arriving:.2} s of {ETA}");
        assert!((deployed - ETA).abs() < 0.5, "seed {s}: on the floor at {deployed:.2} s of {ETA}");
        assert_eq!(p.response_men().len(), 4, "seed {s}: the team's men");
    }
}

/// THE RULE: one wave a night: a second call is refused while they are on their way, while they are on the floor, and after every one of them is dead.
/// A FAILURE MEANS: a player can be sent a second team, or killing the first resets it.
#[test]
fn there_is_one_wave_a_night() {
    for s in SEEDS {
        let mut p = night(s);
        rifle_rounds(&mut p);
        p.alarm_raised_to(2.0);
        p.until(3.0, |p| p.response() == QrfState::Inbound).unwrap_or_else(|| panic!("seed {s}: not called: {:?}", p.response()));
        assert!(!p.response_called_again(), "seed {s}: taken again while on its way");
        p.until(ETA + 2.0, |p| p.response() == QrfState::Deployed).unwrap_or_else(|| panic!("seed {s}: never on the floor"));
        p.run(1.0);
        assert!(!p.response_called_again(), "seed {s}: taken again with them on the floor");
        let men = p.response_men();
        for &m in men.iter() { p.falls(m, DownBy::Player); }
        p.run(1.0);
        assert!(!p.response_called_again(), "seed {s}: taken again with all of them dead");
        assert_eq!((p.response(), p.response_men().len()), (QrfState::Deployed, 4), "seed {s}: spent, and no more men");
    }
}

/// THE RULE: the team works by the squad's own code: on the floor, with nobody found, it goes to a room's door as one, works it by the door tools, and goes in together, a beat apart.
/// A FAILURE MEANS: the team has a door drill of its own again, or walks in one at a time.
#[test]
fn the_team_goes_into_a_room_as_a_squad() {
    for s in SEEDS {
        let mut p = night(s);
        p.sam_unseen();
        rifle_rounds(&mut p);
        p.alarm_raised_to(2.0);
        p.until(ETA + 5.0, |p| p.response() == QrfState::Deployed).unwrap_or_else(|| panic!("seed {s}: never on the floor"));
        let said = |p: &Play| p.g.qrf.log.iter().map(|l| l.what.clone()).collect::<Vec<_>>();
        let ops = |p: &Play| p.g.qrf.group.map_or(0, |id| p.g.groups[id].tools.ops.len() + p.g.groups[id].tools.done.len());
        p.until(60.0, |p| ops(p) > 0).unwrap_or_else(|| panic!("seed {s}: a minute on the floor and no door worked: {:?}", said(&p)));
        let men = p.response_men();
        let side = p.g.groups[p.g.qrf.group.unwrap()].side;
        assert!(men.iter().all(|&m| p.g.team(m) == side && game::group::of(&p.g, m) == p.g.qrf.group), "seed {s}: not one group");
        p.until(90.0, |p| p.g.qrf.op.is_some_and(|op| p.g.tab.doortools_door_op[op].crossed.len() >= 2)).unwrap_or_else(|| panic!("seed {s}: nobody is through a door with a mate: {:?}", said(&p)));
        let op = p.g.qrf.op.unwrap();
        let mut at: Vec<f64> = p.g.tab.doortools_door_op[op].crossed.values().copied().collect();
        at.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!(at[1] - at[0] < 4.0, "seed {s}: {:.1} s between the first two through", at[1] - at[0]);
    }
}
