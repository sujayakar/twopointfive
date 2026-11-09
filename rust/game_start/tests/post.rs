//! A DEFENDER AND HIS POST (rust/game_rules/src/roomfight/post.rs): how he leaves it and comes back, whoever decides it.
use game::ids::RoomfightDefId;
use game::owner::by;
use game::roomfight::{at_post, on_leg_of, send, Leg};
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];

/// the block's room clear, stood and still, and a defender of it at his post
fn block(seed: u32) -> (Play, RoomfightDefId) {
    let mut p = Play::mission("block.json", "?levelFile=block.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.run(0.5);
    let d = p.g.fight.defs.iter().copied().find(|&d| at_post(&p.g, d)).expect("a defender at his post");
    (p, d)
}
/// a place a few metres off that he is sent to: where another defender stands
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
        let (mut p, d) = block(s);
        let (to, back) = (elsewhere(&p, d), p.g.tab.roomfight_def[d].station);
        send(&mut p.g, d, Leg::to(to, 3.0), by::LANE);
        assert!(on_leg_of(&p.g, d, by::LANE) && !on_leg_of(&p.g, d, by::BREAKER), "seed {s}: it is the lane's leg and nobody else's");
        send(&mut p.g, d, Leg::to(back, 3.0), by::BREAKER);
        assert!(on_leg_of(&p.g, d, by::BREAKER) && !on_leg_of(&p.g, d, by::LANE), "seed {s}: sent elsewhere, the lane's leg is over");
    }
}
