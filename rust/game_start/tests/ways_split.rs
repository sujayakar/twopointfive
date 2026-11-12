//! THE WAYS INTO A HELD ROOM: men of a squad with a steady man up split them, one gun a way while they have heard nobody, two on a way they have heard them at and a third man always on another.
//! Green men pick for themselves.
use game::game::{DownBy, GuardState};
use game::ids::GuardId;
use game::morale::Training;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: [f64; 2] = [15.0, 10.0];
const SOUTH: [f64; 2] = [15.0, 18.0];
const WEST: [f64; 2] = [10.0, 14.0];

/// a room with a door in three of its walls and `men` defenders in it, the player and his men far off and unseen
fn hall(seed: u32, men: usize, training: Training) -> (Play, Vec<GuardId>) {
    let mut s = Scene::new("hall");
    s.room("hall", 10.0, 10.0, 20.0, 18.0).door("north", Axis::X, 10.0, 15.0).door("south", Axis::X, 18.0, 15.0).door("west", Axis::Z, 10.0, 14.0);
    s.bulb(15.0, 14.0).moon();
    s.team([4.0, 25.0], 0.0, &[[3.0, 25.0], [5.0, 25.0], [3.0, 26.0], [5.0, 26.0]]);
    // (each with a line to every door, past the others)
    for (k, at) in [[13.0, 12.5], [18.0, 16.0], [18.5, 12.0]].into_iter().take(men).enumerate() { s.defender("hall", [at[0] + nudge(seed, k as u32), at[1]], "west"); }
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.sam_unseen();
    p.g.k.morale.training = training;
    p.run(0.5);
    let men = p.g.fight.defs.iter().map(|&d| p.g.tab.roomfight_def[d].gd).collect();
    (p, men)
}
fn roused(p: &mut Play, men: &[GuardId]) {
    for &m in men {
        game::mind::set_state(&mut p.g, m, GuardState::Alert, "test");
        game::mind::raise(&mut p.g, m, 0.9, "test");
    }
}
/// word of them outside that door reaches these men
fn word_of_them(p: &mut Play, men: &[GuardId], at: [f64; 2]) {
    roused(p, men);
    for &m in men {
        game::roomfight::room_fight_told(&mut p.g, m, [at[0], 0.0, at[1]]);
    }
}
/// the door a man watches
fn watches(p: &Play, m: GuardId) -> Option<[f64; 2]> {
    let d = game::roomfight::def_of(&p.g, m).expect("he holds a post");
    let row = &p.g.tab.roomfight_def[d];
    let e = p.g.tab.roomfight_room[row.room?].entries.get(row.watch as usize).filter(|_| row.watch >= 0.0)?;
    Some([e.at[0], e.at[2]])
}
/// how many of these men watch that door
fn on(p: &Play, men: &[GuardId], door: [f64; 2]) -> usize { men.iter().filter(|&&m| watches(p, m).is_some_and(|w| dist(w, door) < 1.0)).count() }

/// THE RULE: three men who work together, roused with no word of where: a gun on each of three ways. Word of somebody outside one: two guns on it, the third on another.
/// CONTROL: three green men with that word, and all three are on the door they heard of.
/// A FAILURE MEANS: every gun in a room is on one door, and you walk in by the other; or the door you come by has one gun on it, and the others look at doors nobody comes by.
#[test]
fn steady_men_split_the_ways_and_green_men_do_not() {
    for s in SEEDS {
        let (mut p, men) = hall(s, 3, Training::Trained);
        roused(&mut p, &men);
        p.run(2.0);
        assert_eq!([NORTH, SOUTH, WEST].map(|d| on(&p, &men, d)), [1, 1, 1], "seed {s}: with nothing heard, steady men watch {:?}", men.iter().map(|&m| watches(&p, m)).collect::<Vec<_>>());
        word_of_them(&mut p, &men, [15.0, 7.0]);
        p.run(2.0);
        assert_eq!((on(&p, &men, NORTH), on(&p, &men, SOUTH) + on(&p, &men, WEST)), (2, 1), "seed {s}: with word of the north door, steady men watch {:?}", men.iter().map(|&m| watches(&p, m)).collect::<Vec<_>>());
        drop(p);
        let (mut p, men) = hall(s, 3, Training::Green);
        roused(&mut p, &men);
        word_of_them(&mut p, &men, [15.0, 7.0]);
        p.run(2.0);
        assert_eq!(on(&p, &men, NORTH), 3, "seed {s}: green men watch {:?}", men.iter().map(|&m| watches(&p, m)).collect::<Vec<_>>());
    }
}

/// THE RULE: two men and three ways: with nothing heard they are on two of them, and with word of one, whichever it is, both are on that. CONTROL is the same two men before the word.
/// A FAILURE MEANS: one of them goes on looking where nobody is while the other fights alone.
#[test]
fn two_men_are_both_on_the_door_they_hear() {
    for s in SEEDS {
        for door in [NORTH, SOUTH, WEST] {
            let (mut p, men) = hall(s, 2, Training::Trained);
            roused(&mut p, &men);
            p.run(2.0);
            assert_eq!([NORTH, SOUTH, WEST].map(|d| on(&p, &men, d)).iter().max(), Some(&1), "seed {s}: with nothing heard two men watch {:?}", men.iter().map(|&m| watches(&p, m)).collect::<Vec<_>>());
            word_of_them(&mut p, &men, [door[0] + (door[0] - 15.0), door[1] + (door[1] - 14.0)]);
            p.run(2.0);
            assert_eq!(on(&p, &men, door), 2, "seed {s}: with word of {door:?} two men watch {:?}", men.iter().map(|&m| watches(&p, m)).collect::<Vec<_>>());
        }
    }
}

/// THE RULE: with the steady man down the others are no squad: each picks for himself. CONTROL: a green man down, and the steady man and the other still split them.
/// A FAILURE MEANS: killing the man who holds them together changes nothing.
#[test]
fn with_the_steady_man_down_each_picks_for_himself() {
    for s in SEEDS {
        for steady_falls in [true, false] {
            let (mut p, men) = hall(s, 3, Training::Mixed);
            let steady: Vec<GuardId> = men.iter().copied().filter(|&m| game::morale::training_of(&mut p.g, m) == game::morale::TrainingOf::Steady).collect();
            assert_eq!(steady.len(), 1, "seed {s}: the staging: of three men, one is steady");
            let out = if steady_falls { steady[0] } else { *men.iter().find(|m| **m != steady[0]).unwrap() };
            p.falls(out, DownBy::Player);
            let left: Vec<GuardId> = men.iter().copied().filter(|&m| m != out).collect();
            word_of_them(&mut p, &left, [15.0, 7.0]);
            p.run(2.0);
            let split = left.iter().filter(|&&m| game::cover::way_dealt(&p.g, m).is_some()).count();
            assert_eq!(split, if steady_falls { 0 } else { 2 }, "seed {s}: with the {} man down, men dealt a way", if steady_falls { "steady" } else { "green" });
        }
    }
}
