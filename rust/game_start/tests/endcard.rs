//! THE END CARD IS DUE ONCE (game_draw `ended`, which a page opens it on). Each test holds one kind of end over three seeds beside a control.
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
        assert_eq!(p.end_cards.len(), 1, "seed {}: both down: one card", seed);
        p.run(3.0);
        assert_eq!(p.end_cards.len(), 1, "seed {}", seed);
    }
}

/// the seat's player taps Enter; answers whether the mission is asked for again by it
fn enter(p: &mut Play, seat: usize) -> bool {
    let i = p.g.tab.cmd_seat[p.g.seats[seat]].input;
    p.g.tab.input[i].pressed.add(String::from("Enter"));
    p.net_frame(0);
    game::input::input_end_frame(&mut p.g, i);
    core::mem::take(&mut p.g.saves.again)
}
/// whether the seat's player is told that Enter has the mission again
fn told(p: &mut Play, seat: usize) -> bool {
    let (s, me) = (p.g.seats[seat], p.g.seats[0]);
    game::seats::show_seat(&mut p.g, s);
    let k = game::kit::kit_view(&p.g, &game::seats::here(&p.g));
    game::seats::show_seat(&mut p.g, me);
    k.again
}

/// THE RULE: a player who is down has the mission again by Enter once it is over for every player: all of them down, or the mission ended, won or lost. Until then Enter does nothing, and he is told so.
/// A FAILURE MEANS: one player goes down, presses what his screen says, and the other's fight is thrown away; or a mission that is over waits for a key nobody is told of.
#[test]
fn in_a_game_of_two_enter_waits_until_it_is_over_for_both() {
    for seed in SEEDS {
        for downed in [0, 1] {
            let mut p = yard(seed, 2);
            p.run(0.5);
            p.seat_goes_down(downed);
            p.run(1.0);
            assert!(!told(&mut p, downed) && !enter(&mut p, downed), "seed {seed}: player {downed} is down and the other fights on: Enter does nothing");
            p.seat_goes_down(1 - downed);
            p.run(0.2);
            assert!(told(&mut p, downed) && enter(&mut p, downed), "seed {seed}: both are down: Enter has the mission again");
            drop(p);

            let mut p = yard(seed, 2);
            p.run(0.5);
            p.seat_goes_down(downed);
            for gd in p.hostiles() { p.falls(gd, game::game::DownBy::Player); }
            p.run(3.0);
            assert!(game::verdict::verdict_ended(&p.g).is_some() && !game::seats::all_down(&p.g), "seed {seed}: won with one down");   // (the control)
            assert!(!enter(&mut p, 1 - downed), "seed {seed}: he who is up plays on: Enter alone is no restart of his");
            assert!(told(&mut p, downed) && enter(&mut p, downed), "seed {seed}: the mission is over: the man who is down has it again");
            assert!(game::savestate::control::restart(&mut p.g) && game::verdict::verdict_ended(&p.g).is_none(), "seed {seed}: the mission that stands again has no end yet: whoever asks how it went is told it is on");
        }
    }
}
/// THE RULE: in a game of one Enter has the mission again from the moment he is down.
#[test]
fn alone_enter_is_there_at_once() {
    for seed in SEEDS {
        let mut p = yard(seed, 1);
        p.run(0.5);
        assert!(!told(&mut p, 0), "seed {seed}: up");
        p.seat_goes_down(0);
        p.frame();
        assert!(told(&mut p, 0), "seed {seed}: down");
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


/// A FAILURE MEANS: the end card's map is light where nobody is indoors, as it was while every level had the office's outline laid over it.
#[test]
fn the_map_is_light_under_the_levels_own_roofs() {
    for (level, under, open) in [("crossing", [20.0, 7.0], [20.0, 16.0]), ("block", [13.0, 18.0], [38.0, 14.0]), ("floodlit", [18.0, 9.0], [10.0, 20.0])] {
        let p = Play::level(&format!("{level}.json"), false, &Opts { seed: 1, ..Default::default() });
        let floor = game::view::walked_floor(&p.g);
        let light = |at: [f64; 2]| { let cell = |v: f64| (v / floor.cell) as usize; (floor.rows.as_bytes()[cell(at[1]) * floor.w as usize + cell(at[0])] - b'0') & 1 == 1 };
        assert!(light(under), "{level}: {under:?} is in a room");
        assert!(!light(open), "{level}: {open:?} is in the open");
    }
}
