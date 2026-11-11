//! A DEFENDER AND HIS POST (rust/game_rules/src/defenders/roomfight/post.rs): how he leaves it and comes back, whoever decides it.
use game::ids::RoomfightDefId;
use game::owner::{self, by};
use game::roomfight::{at_post, leg_ended, on_leg_of, send, stand_here, Leg, LegEnd};
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];

/// the consulate's rescue, stood and still, and a defender of it at his post whom nobody has but his squad, which lets go of him here
fn consulate(seed: u32) -> (Play, RoomfightDefId) {
    let mut p = Play::mission("consulate.json", "?levelFile=consulate.json&rescue&squad=4", &Opts { seed, ..Default::default() });
    p.sam_unseen();
    p.run(0.5);
    let d = p.g.fight.defs.iter().copied().find(|&d| at_post(&p.g, d) && owner::kept(&p.g, p.g.tab.roomfight_def[d].gd, by::SQUAD).is_none()).expect("a defender only his squad has");
    let (gd, t) = (p.g.tab.roomfight_def[d].gd, p.time());
    owner::release(&mut p.g, gd, by::SQUAD, t);
    (p, d)
}
/// where a defender of another room stands
fn elsewhere(p: &Play, d: RoomfightDefId) -> game::V3 {
    let room = p.g.tab.roomfight_def[d].room;
    let o = p.g.fight.defs.iter().copied().find(|&o| o != d && p.g.tab.roomfight_def[o].room != room).expect("a defender of another room");
    game::roomfight::station(&p.g, o)
}

/// THE RULE: a leg answers to who sent him on it. Sent somewhere else meanwhile, the first leg is over, whatever he is doing now.
/// A FAILURE MEANS: two rules each think the man on the other's leg is on theirs, and wait for him or count him there.
#[test]
fn a_leg_answers_to_who_sent_him() {
    for s in SEEDS {
        let (mut p, d) = consulate(s);
        let (to, back) = (elsewhere(&p, d), game::roomfight::station(&p.g, d));
        send(&mut p.g, d, Leg::to(to, 3.0), by::LANE);
        assert!(on_leg_of(&p.g, d, by::LANE) && !on_leg_of(&p.g, d, by::BREAKER), "seed {s}: it is the lane's leg and nobody else's");
        send(&mut p.g, d, Leg::to(back, 3.0), by::BREAKER);
        assert!(on_leg_of(&p.g, d, by::BREAKER) && !on_leg_of(&p.g, d, by::LANE), "seed {s}: sent elsewhere, the lane's leg is over");
    }
}

/// THE RULE: a man out for a look at a covered lane who is sent elsewhere by somebody stronger goes there, and the look is over.
/// A FAILURE MEANS: the lane's rule sends for him back off the other leg (before: the last man up, backing off to a corner of his own, was sent back to the lane's).
#[test]
fn a_look_is_over_when_he_is_sent_elsewhere() {
    for s in SEEDS {
        let (mut p, d) = consulate(s);
        let (to, gd, t) = (elsewhere(&p, d), p.g.tab.roomfight_def[d].gd, p.time());
        let home = game::roomfight::station(&p.g, d);
        let look = game::lanes::Peek { lane: 99.0, phase: game::lanes::PeekPhase::Out, t, home, look: to, to: home, line_t: -1.0, eye: None, fire: None, frag: None };
        p.g.m.lanes.states.as_mut().expect("the lanes are on").peek.set(gd, look);
        send(&mut p.g, d, Leg::to(to, 3.0), by::LAST_MAN);
        p.run(0.1);
        assert!(on_leg_of(&p.g, d, by::LAST_MAN), "seed {s}: he goes where he was sent");
        assert!(!p.g.m.lanes.states.as_ref().unwrap().peek.has(gd), "seed {s}: the look is over");
    }
}

/// THE RULE: a man sent on an errand walks all of it, however far, and stands at its end on his sender's word; a plain leg is his post where it ends. Whoever sent him is told how it ended.
/// A FAILURE MEANS: a rule that sends a man works out for itself whether he is there, and keeps him from being planted by putting his clocks back every frame (before: the mains party and a squad's lookers).
#[test]
fn an_errand_is_walked_to_its_end_and_he_stands_there() {
    for s in SEEDS {
        let (mut p, d) = consulate(s);
        let (to, gd) = (elsewhere(&p, d), p.g.tab.roomfight_def[d].gd);
        let way = p.g.pos_of(gd);
        let far = ((to[0] - way[0]).powi(2) + (to[2] - way[2]).powi(2)).sqrt();
        send(&mut p.g, d, Leg { stays: true, ..Leg::errand(to, far / 14.0) }, by::BREAKER);
        assert_eq!(leg_ended(&p.g, d, by::BREAKER), None, "seed {s}: on his way");
        let end = p.until(59.0, |p| leg_ended(&p.g, d, by::BREAKER).is_some());
        assert_eq!(leg_ended(&p.g, d, by::BREAKER), Some(LegEnd::Arrived), "seed {s}: {far:.1} m off, after {end:?} s");
        p.run(4.0);
        assert!(on_leg_of(&p.g, d, by::BREAKER) && leg_ended(&p.g, d, by::BREAKER) == Some(LegEnd::Arrived), "seed {s}: he stands there till he is told");
        stand_here(&mut p.g, d);
        assert!(at_post(&p.g, d), "seed {s}: and that is his post when he is");
    }
}

/// THE RULE: a man who cannot get to where he is sent says so.
/// A FAILURE MEANS: he walks on the spot, or his sender thinks he is there.
#[test]
fn no_way_there_is_told() {
    for s in SEEDS {
        let (mut p, d) = consulate(s);
        let here = game::roomfight::station(&p.g, d);
        send(&mut p.g, d, Leg::errand([here[0] + 400.0, 0.0, here[2]], 3.0), by::LANE);
        let end = p.until(65.0, |p| leg_ended(&p.g, d, by::LANE).is_some()).expect("his leg ends");
        assert_eq!(leg_ended(&p.g, d, by::LANE), Some(LegEnd::Nowhere), "seed {s}: after {end} s");
        assert!(at_post(&p.g, d), "seed {s}: where he got to is his post");
    }
}

/// THE RULE: a man sent elsewhere, or let loose, is no longer on the first errand, and its sender is told.
#[test]
fn taken_off_it_is_told() {
    for s in SEEDS {
        let (mut p, d) = consulate(s);
        let (to, gd) = (elsewhere(&p, d), p.g.tab.roomfight_def[d].gd);
        send(&mut p.g, d, Leg { stays: true, ..Leg::errand(to, 3.0) }, by::SQUAD_LEG);
        send(&mut p.g, d, Leg::to(to, 3.0), by::DIG);
        assert_eq!(leg_ended(&p.g, d, by::SQUAD_LEG), Some(LegEnd::Taken), "seed {s}: sent by another");
        send(&mut p.g, d, Leg { stays: true, ..Leg::errand(to, 3.0) }, by::SQUAD_LEG);
        game::roomfight::loose(&mut p.g, gd, "test");
        assert_eq!(leg_ended(&p.g, d, by::SQUAD_LEG), Some(LegEnd::Taken), "seed {s}: let loose");
    }
}

/// THE RULE: a defender on the move is heard by the team, whoever sent him.
/// A FAILURE MEANS: he walks up to them without a sound (before: only men of three rules made steps).
#[test]
fn boots_on_a_leg_are_heard() {
    for s in SEEDS {
        let (mut p, d) = consulate(s);
        let (to, gd) = (elsewhere(&p, d), p.g.tab.roomfight_def[d].gd);
        let who = p.g.char_id(gd);
        send(&mut p.g, d, Leg::to(to, 3.0), by::LAST_MAN);
        let mut steps = 0;
        for _ in 0..180 { p.frame(); steps += p.g.events.iter().filter(|e| e.who == Some(who) && e.kind == game::director::EventKind::Step && e.time == p.g.time).count(); }
        assert!(steps >= 3, "seed {s}: {steps} steps heard in three seconds of walking");
    }
}
