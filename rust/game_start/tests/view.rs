//! THE VIEW (rust/game_rules/src/tick/view): what a HUD is told of the game, asked of the game and never kept by it.
use game::orders::Who;
use game::roomclear::RoomClearEnd;
use game::view::MissionOf;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];


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
                out.push(p.words());
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
/// room, his man or his line.
#[test]
fn in_a_game_of_two_each_hud_is_told_about_its_own_man() {
    for seed in SEEDS {
        let mut p = Play::level("office.json", true, &Opts { seed, seats: 2, ..Default::default() });
        p.frame();
        // CONTROL: both men on their marks, side by side: one room
        assert_eq!(p.view_of(0).place, p.view_of(1).place, "seed {}: side by side they are told the same room", seed);
        assert_ne!(p.view_of(0).me, p.view_of(1).me, "seed {}: each is told his own man", seed);
        let names = game::dialogue::place_rects(&p.g);
        let spots: Vec<[f64; 2]> = names.iter().map(|r| [(r.x0 + r.x1) / 2.0, (r.z0 + r.z1) / 2.0]).collect();
        let a = spots[0];
        p.seat_stands(0, a);
        let room_a = p.view_of(0).place;
        let b = spots.iter().copied().find(|&at| { p.seat_stands(1, at); p.view_of(1).place != room_a }).unwrap_or_else(|| panic!("seed {}: the level has one room", seed));
        p.seat_stands(1, b);
        let room_b = p.view_of(1).place;
        assert_eq!(p.view_of(0).place, room_a, "seed {}: the first is told the room he stands in", seed);
        // the two change places, and each view follows its own man
        p.seat_stands(0, b); p.seat_stands(1, a);
        assert_eq!(p.view_of(0).place, room_b, "seed {}: the first is told the room he stands in now", seed);
        assert_eq!(p.view_of(1).place, room_a, "seed {}: the second is told the room HE stands in now", seed);
        // the mission's line is his too: a room clear wiped with the second man down tells him one key and the first another
        p.seat_goes_down(1);
        p.g.room_clear.end = Some(game::roomclear::RoomClearEnd::Wiped);
        let line = |v: game::view::TickView| v.mission.expect("a mission is up").text;
        let (first, second) = (line(p.view_of(0)), line(p.view_of(1)));
        assert!(first.contains("shift+enter"), "seed {}: the first, still up: {}", seed, first);
        assert!(second.contains("enter") && !second.contains("shift+enter"), "seed {}: the second, down: {}", seed, second);
    }
}
