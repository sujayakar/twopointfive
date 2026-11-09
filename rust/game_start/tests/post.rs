//! A DEFENDER AND HIS POST (rust/game_rules/src/roomfight/post.rs): how he leaves it and comes back, whoever decides it.
use game::ids::RoomfightDefId;
use game::owner;
use game::roomfight::{send, DefMode, Leg};
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];

/// the consulate's rescue, whose defenders are in no squad, stood and still, and a defender of it at his post whom nobody has
fn consulate(seed: u32) -> (Play, RoomfightDefId) {
    let mut p = Play::mission("consulate.json", "?levelFile=consulate.json&rescue&squad=4", &Opts { seed, ..Default::default() });
    p.sam_unseen();
    p.run(0.5);
    let d = p.g.fight.defs.iter().copied().find(|&d| p.g.tab.roomfight_def[d].mode == DefMode::Hold && owner::kept(&p.g, p.g.tab.roomfight_def[d].gd, owner::By::NOBODY).is_none()).expect("a defender nobody has");
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
        send(&mut p.g, d, Leg::to(to, 3.0), "a test");
        assert!(p.g.tab.roomfight_def[d].mode == DefMode::Move, "seed {s}: he is off his post");
        assert!(p.g.tab.roomfight_def[d].rally_at.is_nan(), "seed {s}: his clock to go next door still runs on the leg");
    }
}
