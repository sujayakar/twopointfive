//! A DEFENDER AND HIS POST (rust/game_rules/src/defenders/roomfight/post.rs): how he leaves it and comes back, whoever decides it.
use game::ids::RoomfightDefId;
use game::owner::{self, by};
use game::roomfight::{at_post, on_leg_of, send, Leg};
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
    p.g.tab.roomfight_def[o].station
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
