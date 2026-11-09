//! A DEFENDER AND HIS POST (rust/game_rules/src/roomfight/post.rs, control/lost.rs, control/rally.rs): how he leaves it and comes back, whoever decides it.
use game::ids::RoomfightDefId;
use game::owner::{self, by};
use game::roomfight::{at_post, is_loose, on_leg, on_leg_of, send, Leg};
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];

/// the consulate's rescue, whose defenders are in no squad, stood and still, and a defender of it at his post whom nobody has
fn consulate(seed: u32) -> (Play, RoomfightDefId) {
    let mut p = Play::mission("consulate.json", "?levelFile=consulate.json&rescue&squad=4", &Opts { seed, ..Default::default() });
    p.sam_unseen();
    p.run(0.5);
    let d = p.g.fight.defs.iter().copied().find(|&d| at_post(&p.g, d) && owner::kept(&p.g, p.g.tab.roomfight_def[d].gd, owner::By::NOBODY).is_none()).expect("a defender nobody has");
    (p, d)
}
/// where a defender of another room stands
fn elsewhere(p: &Play, d: RoomfightDefId) -> game::V3 {
    let room = p.g.tab.roomfight_def[d].room;
    let o = p.g.fight.defs.iter().copied().find(|&o| o != d && p.g.tab.roomfight_def[o].room != room).expect("a defender of another room");
    p.g.tab.roomfight_def[o].station
}
/// he hears a fight there now, his few seconds are over and his mind is made up to go
fn hears_a_fight(p: &mut Play, d: RoomfightDefId) {
    let (at, t, gd) = (elsewhere(p, d), p.time(), p.g.tab.roomfight_def[d].gd);
    p.g.gd[gd].last_seen_t = t - 60.0;
    let row = &mut p.g.tab.roomfight_def[d];
    row.noise_at = Some(at);
    row.noise_t = t;
    row.rally_at = t;
    row.rally_go = true;
}

/// THE RULE: a man sent from his post, whoever sent him, goes on that errand alone: the clock that was to take him to a fight next door is out.
/// A FAILURE MEANS: a man sent to a lane or a breaker, planted there, runs off to the next room on a clock that began before he was sent (before: four of the eight ways to send him left it running).
#[test]
fn a_man_sent_from_his_post_goes_on_that_errand_alone() {
    for s in SEEDS {
        let (mut p, d) = consulate(s);
        hears_a_fight(&mut p, d);
        let to = elsewhere(&p, d);
        send(&mut p.g, d, Leg::to(to, 3.0), by::LANE);
        assert!(on_leg(&p.g, d), "seed {s}: he is off his post");
        assert!(p.g.tab.roomfight_def[d].rally_at.is_nan(), "seed {s}: his clock to go next door still runs on the leg");
    }
}

/// THE RULE: a leg answers to who sent him on it. Sent somewhere else meanwhile, the first leg is over, whatever he is doing now.
/// A FAILURE MEANS: two rules each think the man on the other's leg is on theirs, and wait for him or count him there.
#[test]
fn a_leg_answers_to_who_sent_him() {
    for s in SEEDS {
        let (mut p, d) = consulate(s);
        let (to, back) = (elsewhere(&p, d), p.g.tab.roomfight_def[d].station);
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
        let home = p.g.tab.roomfight_def[d].station;
        let look = game::lanes::Peek { lane: 99.0, phase: game::lanes::PeekPhase::Out, t, home, look: to, to: home, line_t: -1.0, eye: None, fire: None, frag: None };
        p.g.m.lanes.states.as_mut().expect("the lanes are on").peek.set(gd, look);
        send(&mut p.g, d, Leg::to(to, 3.0), by::LAST_MAN);
        p.run(0.1);
        assert!(on_leg_of(&p.g, d, by::LAST_MAN), "seed {s}: he goes where he was sent");
        assert!(!p.g.m.lanes.states.as_ref().unwrap().peek.has(gd), "seed {s}: the look is over");
    }
}

/// THE RULE: a man nobody has goes to a fight next door, off his post on the fight's own hunt; a man somebody has stays.
/// A FAILURE MEANS: the list is not asked at his post, or a man with an errand leaves it for a noise.
#[test]
fn a_fight_next_door_takes_a_man_nobody_has() {
    for s in SEEDS {
        let (mut p, d) = consulate(s);
        hears_a_fight(&mut p, d);
        p.run(0.1);
        assert!(is_loose(&p.g, d), "seed {s}: he goes");
        drop(p);
        let (mut p, d) = consulate(s);
        hears_a_fight(&mut p, d);
        let (gd, t) = (p.g.tab.roomfight_def[d].gd, p.time());
        owner::claim(&mut p.g, gd, by::LANE, t, None);
        p.run(0.1);
        assert!(at_post(&p.g, d), "seed {s}: he is the lane's, and stays");
    }
}
