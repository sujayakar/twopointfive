//! A LEVEL WHOSE MISSION WAS CUT: what the mission laid over the room clear has left the file and the game, and what is left is a room clear like any other.
use game::game::DownBy;
use game::roomclear::RoomClearEnd;
use game::teams::Team;
use game::view::MissionOf;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [5, 6, 7];

fn stood(level: &str, seed: u32) -> Play {
    let mut p = Play::mission(level, &format!("?levelFile={level}&roomclear&squad4"), &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.5);
    p
}

/// THE RULE: the snatch's level stands with no man to take and no van to reach: an armed house, nobody unarmed, and when the last of them falls the room clear
/// is taken, on the room clear's own objective.
/// A FAILURE MEANS: something of the snatch outlived it (a man on the floor), or the file no longer stands as a room clear.
#[test]
fn the_house_in_the_yard_is_a_plain_room_clear_with_nobody_to_take() {
    for seed in SEEDS {
        let mut p = stood("snatch.json", seed);
        assert!(p.g.civilians.is_empty(), "seed {seed}: somebody unarmed stands in the house");
        assert_eq!(p.view().mission.expect("a mission is up").of, MissionOf::Roomclear, "seed {seed}");
        let men: Vec<_> = p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team == Team::Hostile).collect();
        assert!(!men.is_empty() && men.len() <= 8, "seed {seed}: {} men in the house", men.len());
        for gd in men { p.falls(gd, DownBy::Player); }
        assert!(p.until(10.0, |p| p.room_clear_end().is_some()).is_some(), "seed {seed}: the house fell and nothing ended");
        assert_eq!(p.room_clear_end().map(|e| e.0), Some(RoomClearEnd::Taken), "seed {seed}");
    }
}
