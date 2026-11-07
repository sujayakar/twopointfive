//! THE END CARD IS AN EVENT (rust/game_rules/src/mission, `end_card`): the game tells its host once that the card is due, and nobody clears a flag on a copy. Each test holds
//! one kind of end over three seeds beside a control. Before this, the game raised a flag no rule ever put down and each page cleared its own copy of it.
use game::game::MissionStage;
use game::mission::DOWN_CARD_SECS;
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: f64 = std::f64::consts::PI;

/// two rooms and a team in the yard: the smallest game that takes two players
fn yard(seed: u32, seats: u32) -> Play {
    let mut s = Scene::new("endcard");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([14.0, 21.0], NORTH, &[[13.0, 22.0], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    s.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&s, &Opts { seed, seats, ..Default::default() });
    p.g.ai_enabled = false;
    p.run(0.2);
    p
}

/// A FAILURE MEANS: the down card comes at once, or never, or twice; or its beat no longer runs.
#[test]
fn down_the_card_comes_once_a_beat_after_he_drops() {
    for seed in SEEDS {
        let mut p = yard(seed, 1);
        p.run(1.0);
        assert!(p.end_cards.is_empty(), "seed {}: he is up: no card", seed);
        p.seat_goes_down(0);
        let t = p.time();
        p.run(DOWN_CARD_SECS - 0.1);
        assert!(p.end_cards.is_empty() && p.g.mission.stage == MissionStage::Failed, "seed {}: the beat is not out", seed);
        p.run(0.3);
        assert_eq!(p.end_cards.len(), 1, "seed {}: the card is due", seed);
        assert!((p.end_cards[0] - t - DOWN_CARD_SECS).abs() < 0.05, "seed {}: at the end of the beat: {} s after the fall", seed, p.end_cards[0] - t);
        p.run(5.0);
        assert_eq!(p.end_cards.len(), 1, "seed {}: and once", seed);
    }
}

/// A FAILURE MEANS: a player still playing is sent to a card because the other went down, or the last one down gets none.
#[test]
fn in_a_game_of_two_the_card_waits_for_both() {
    for seed in SEEDS {
        let mut p = yard(seed, 2);
        p.run(0.5);
        p.seat_goes_down(0);
        p.run(DOWN_CARD_SECS + 1.0);
        assert!(p.end_cards.is_empty() && !p.g.recap.all_down, "seed {}: one is up: no card, and not every player is down", seed);
        p.seat_goes_down(1);
        p.frame();
        assert!(p.g.recap.all_down, "seed {}: the game says so, for the HUD", seed);
        p.run(DOWN_CARD_SECS + 0.3);
        assert_eq!(p.end_cards.len(), 1, "seed {}: both down: one card, one event for the game", seed);
        p.run(3.0);
        assert_eq!(p.end_cards.len(), 1, "seed {}", seed);
    }
}

/// A FAILURE MEANS: the mission's end no longer tells the host, or tells it more than once. (`exfil`: the run's last stage, the stage written and not played: the drive is its own test.)
fn out_past_the_exit(seed: u32) -> Play {
    let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
    p.run(0.2);
    p.g.mission.stage = MissionStage::Exfil;
    p
}
const PAST_THE_EXIT: [f64; 3] = [36.9, 0.0, 20.6];
#[test]
fn out_through_the_exit_the_card_comes_once() {
    for seed in SEEDS {
        let mut p = out_past_the_exit(seed);
        p.run(1.0);
        assert!(p.end_cards.is_empty(), "seed {}: inside the building: no card", seed);   // the control
        game::game::game_teleport_player(&mut p.g, PAST_THE_EXIT);
        p.run(0.3);
        assert_eq!(p.g.mission.stage, MissionStage::Done, "seed {}", seed);
        assert_eq!(p.end_cards.len(), 1, "seed {}: out: the card", seed);
        p.run(3.0);
        assert_eq!(p.end_cards.len(), 1, "seed {}: and once", seed);
    }
}

/// A FAILURE MEANS: a show's mission ends in front of an audience on a card nobody asked for.
#[test]
fn no_card_while_the_tour_runs() {
    for seed in SEEDS {
        let mut p = yard(seed, 1);
        p.seat_goes_down(0);
        p.frame();
        p.g.mission_in_tour = true;   // (after the fall: a fall in a show arms no beat at all, so this is the end_card's own rule)
        p.run(DOWN_CARD_SECS + 1.0);
        assert!(p.end_cards.is_empty() && p.g.mission.stage == MissionStage::Failed, "seed {}: down in a show: no card", seed);
        drop(p);
        let mut q = out_past_the_exit(seed);
        q.g.mission_in_tour = true;
        game::game::game_teleport_player(&mut q.g, PAST_THE_EXIT);
        q.run(0.3);
        assert!(q.end_cards.is_empty() && q.g.mission.stage == MissionStage::Done, "seed {}: out in a show: no card", seed);
    }
}
