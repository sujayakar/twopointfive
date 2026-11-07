//! THE VIEW (rust/game_rules/src/view): what a HUD is told of the game, asked of the game and never kept by it.
use game::orders::Who;
use game::roomclear::RoomClearEnd;
use game::view::MissionOf;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];

/// every word of the game's state, a word a part: the game's own parts, then rust/sim's block
fn words(p: &mut Play) -> Vec<u32> {
    let (mut w, mut words) = (game::io::W::new(), Vec::new());
    game::state_words::state_words(&mut p.g, &mut w, &mut words);
    words.extend(game_start::block::part_hashes(0));
    words
}

/// A FAILURE MEANS: asking what a HUD is told changes the game. Two machines of one game ask at different times and as often as they draw: a question
/// that writes would part them. The same run twice, one asked after every frame and one never, equal in every part of the state after every second.
#[test]
fn asking_the_view_writes_nothing() {
    for seed in SEEDS {
        let run = |ask: bool| -> Vec<Vec<u32>> {
            let mut p = Play::level("killhouse.json", true, &Opts { seed, ..Default::default() });
            let at = p.sam_at();
            let mut out = Vec::new();
            for s in 0..12 {
                if s == 2 { p.order("fallIn", at, Who::All); }
                if s == 5 { if let Some(&h) = p.hostiles().first() { p.falls(h, game::game::DownBy::Player); } }
                for _ in 0..60 { p.frame(); if ask { let v = p.view(); assert!(v.mission.is_some()); } }
                out.push(words(&mut p));
            }
            out
        };
        let (never, always) = (run(false), run(true));
        for (s, (a, b)) in never.iter().zip(always.iter()).enumerate() {
            let parts: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
            assert!(parts.is_empty(), "seed {}: after {} s the asked game differs in parts {:?}", seed, s + 1, parts);
        }
    }
}

/// A FAILURE MEANS: a HUD is told another mission's line than the one that is up, or keeps a line after what it says has changed. (The room clear's row
/// used to be worded when something told it to and kept till then: a name their net had put to a room stayed on it after the man was down.)
#[test]
fn the_missions_line_is_the_one_that_is_up_and_as_things_stand() {
    for seed in SEEDS {
        let mut p = Play::level("kit-yard.json", true, &Opts { seed, ..Default::default() });
        p.frame();
        let v = p.view().mission.expect("a mission is up");
        assert_eq!(v.of, MissionOf::Roomclear, "seed {}", seed);
        assert!(v.text.starts_with("objective ▸ clear the structure: ") && !v.text.contains('✓') && v.pos.is_some(), "seed {}: {:?}", seed, v);
        for h in p.hostiles() { p.falls(h, game::game::DownBy::Player); }
        p.until(20.0, |p| p.room_clear_end().is_some()).unwrap_or_else(|| panic!("seed {}: every defender is down and the room clear never ended", seed));
        assert_eq!(p.room_clear_end().map(|e| e.0), Some(RoomClearEnd::Taken));
        let v = p.view().mission.unwrap();
        assert!(v.text.starts_with("structure taken") && v.pos.is_none(), "seed {}: {:?}", seed, v);
        drop(p);
        // CONTROL: the same level with no room clear on it words the stealth mission's own first stage
        let mut q = Play::level("kit-yard.json", false, &Opts { seed, ..Default::default() });
        q.frame();
        let v = q.view().mission.expect("the stealth mission stands");
        assert!(v.of == MissionOf::Stages && v.text == "objective ▸ reach the server room", "seed {}: {:?}", seed, v);
    }
}

/// A GAME OF TWO: each player's HUD is told about HIM, whoever's things stand on the game. A FAILURE MEANS: the second machine is told the first player's
/// marker or key (the place of a charge is the nearest to the man who is shown it; a run that is lost asks for another key of a man who is down).
#[test]
fn in_a_game_of_two_each_hud_is_told_about_its_own_man() {
    for seed in SEEDS {
        let mut p = Play::level("defuse.json", true, &Opts { seed, seats: 2, ..Default::default() });
        p.frame();
        let charges: Vec<[f64; 2]> = p.charges();
        assert!(charges.len() >= 2, "seed {}: the level has two charges: {:?}", seed, charges);
        // CONTROL: both men on their marks, side by side: one nearest charge, one marker
        let at = |v: game::view::TickView| { let m = v.mission.expect("a mission is up"); assert_eq!(m.of, MissionOf::Defuse, "{:?}", m); let x = m.pos.expect("a live charge has a place"); [x[0], x[2]] };
        assert_eq!(at(p.view_of(0)), at(p.view_of(1)), "seed {}: side by side they are shown the same charge", seed);
        // each beside another charge
        let (a, b) = (charges[0], charges[1]);
        p.seat_stands(0, a); p.seat_stands(1, b);
        assert_eq!(at(p.view_of(0)), a, "seed {}: the first is shown the charge beside him", seed);
        assert_eq!(at(p.view_of(1)), b, "seed {}: the second is shown the charge beside HIM", seed);
    }
}
