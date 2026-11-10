//! THE END CARD'S RECORD (rust/game_rules/src/tick/view, `debrief_view`): the rows in words, the counts, the score and the recap come out of the Rust game as one record; the
//! page lays it out. Each test holds one thing the card says over three seeds beside a control. Before this, the page worked all of it out over its own copy of the chronicle.
use game::chronicle::{self, ChronCheckpointWhat, ChronDoorKind, ChronKind};
use game::game::DownKind;
use game::view::DebriefView;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];

fn office(seed: u32, seats: u32) -> Play {
    let mut p = Play::level("office.json", false, &Opts { seed, seats, ..Default::default() });
    p.run(0.2);
    p
}
fn card(p: &Play) -> DebriefView { p.debrief_view(0) }
fn door(p: &Play, name: &str) -> game::ids::DoorId { game::doors::doors_by_name(&p.g, name).unwrap_or_else(|| panic!("the office has no door '{}'", name)) }
fn man(p: &Play) -> game::ids::GuardId { p.g.guards.iter().copied().find(|&i| !game::condition::down(&p.g, p.g.gd[i].ch) && p.g.gd[i].civ.is_none()).expect("a man on the floor") }

/// A FAILURE MEANS: the card calls a door by its id, or the name its level wrote stops reaching the row.
#[test]
fn a_door_row_calls_its_door_what_its_level_calls_it() {
    for seed in SEEDS {
        let mut p = office(seed, 1);
        let n = card(&p).rows.len();
        assert!(!card(&p).rows.iter().any(|r| r.kind == ChronKind::DoorKicked), "seed {}: control: nobody has kicked a door", seed);
        let (named, plain) = (door(&p, "breakroom_w"), door(&p, "storage"));
        chronicle::chron_door(&mut p.g, ChronDoorKind::DoorKicked, named);
        chronicle::chron_door(&mut p.g, ChronDoorKind::LockPicked, named);
        chronicle::chron_door(&mut p.g, ChronDoorKind::DoorKicked, plain);
        let rows = card(&p).rows;
        let words: Vec<&str> = rows[n..].iter().map(|r| r.text.as_str()).collect();
        assert_eq!(words, ["kicked in the cubicles-side break room door", "picked the lock on the cubicles-side break room door", "kicked in the storage door"], "seed {}", seed);
    }
}

/// A FAILURE MEANS: a man the wall charge knocked out through a wall reads as a takedown, or a man down by hand reads as the charge.
#[test]
fn a_charge_files_its_own_rows() {
    for seed in SEEDS {
        let mut p = office(seed, 1);
        let n = card(&p).rows.len();
        let gd = man(&p);
        chronicle::chron_down(&mut p.g, gd, DownKind::Struck, game::game::With::Hand).expect("a row");
        chronicle::chron_down(&mut p.g, gd, DownKind::Struck, game::game::With::Charge).expect("a row");
        let rows = card(&p).rows;
        assert!(rows[n].text.ends_with("out cold — the takedown"), "seed {}: {}", seed, rows[n].text);
        assert!(rows[n + 1].text.ends_with("out cold — the wall charge"), "seed {}: {}", seed, rows[n + 1].text);
    }
}

/// A FAILURE MEANS: at the timeline's caps the score or the counters stop moving, or chatter crowds out a row the card needs.
#[test]
fn at_the_caps_the_card_still_prices_what_happened() {
    let (max_events, soft_events) = (chronicle::MAX_EVENTS as usize, chronicle::SOFT_EVENTS as usize);
    for seed in SEEDS {
        let mut p = office(seed, 1);
        while p.g.chronicle.events.len() < soft_events { chronicle::chron_objective(&mut p.g, "filler", None); }
        let n = p.g.chronicle.events.len();
        let saved = p.g.chronicle.counts.saves;
        chronicle::chron_checkpoint(&mut p.g, ChronCheckpointWhat::Save, Some("x"));
        assert_eq!((p.g.chronicle.events.len(), p.g.chronicle.counts.saves), (n, saved + 1.0), "seed {}: a save past the soft cap is chatter, and counted", seed);
        chronicle::chron_checkpoint(&mut p.g, ChronCheckpointWhat::Load, Some("x"));
        assert_eq!((p.g.chronicle.events.len(), p.g.chronicle.counts.loads), (n + 1, 1.0), "seed {}: a load past the soft cap lands", seed);
        while p.g.chronicle.events.len() < max_events { chronicle::chron_objective(&mut p.g, "filler", None); }
        let (score, esc2, dropped) = (card(&p).debrief.score, p.g.chronicle.counts.esc2, card(&p).dropped);
        game::guards::raise_escalation(&mut p.g, 2.0, None, Some(game::guards::RaiseEscalationCause::Search), None);
        p.run(1.0 / 60.0);
        let after = card(&p);
        assert_eq!(after.rows.len(), max_events, "seed {}: no row lands at the hard cap", seed);
        assert_eq!(p.g.chronicle.counts.esc2, esc2 + 1.0, "seed {}: the lockdown counts", seed);
        assert_eq!(after.debrief.score, score - p.g.k.score.lockdown, "seed {}: and is priced", seed);
        assert!(after.dropped > dropped, "seed {}: and the card says rows were dropped", seed);
    }
}

/// A FAILURE MEANS: the reel behind the title writes on the card, or the run does not write on it again once somebody plays.
#[test]
fn with_nobody_playing_nothing_lands_on_the_card() {
    for seed in SEEDS {
        let mut p = office(seed, 1);
        p.run(2.0);
        p.g.aside = true;
        let (rows, path, loads) = (p.g.chronicle.events.len(), p.g.chronicle.path_n, p.g.chronicle.counts.loads);
        chronicle::chron_checkpoint(&mut p.g, ChronCheckpointWhat::Load, Some("x"));
        p.run(2.0);
        assert_eq!((p.g.chronicle.events.len(), p.g.chronicle.path_n, p.g.chronicle.counts.loads), (rows, path, loads), "seed {}", seed);
        p.g.aside = false;
        chronicle::chron_checkpoint(&mut p.g, ChronCheckpointWhat::Load, Some("x"));
        p.run(2.0);
        assert_eq!(p.g.chronicle.counts.loads, loads + 1.0, "seed {}: the flag down, it counts", seed);
        assert!(p.g.chronicle.path_n > path, "seed {}: and the path goes on", seed);
    }
}

/// A FAILURE MEANS: the path or the map's rooms stop reaching the card, or an alarm going down wears the alarm's pin.
#[test]
fn the_map_gets_the_path_the_rooms_and_the_rows_pins() {
    for seed in SEEDS {
        let mut p = office(seed, 1);
        p.run(6.0);
        let v = card(&p);
        assert_eq!(v.path.len() as f64, p.g.chronicle.path_n * 3.0, "seed {}: every sample and none past it", seed);
        assert!(!v.path.is_empty(), "seed {}: he stood somewhere", seed);
        chronicle::chron_alarm(&mut p.g, 1.0, true, Some("heard"));
        chronicle::chron_alarm(&mut p.g, 0.0, false, None);
        let alarms: Vec<bool> = card(&p).rows.iter().filter(|r| r.kind == ChronKind::Alarm).map(|r| r.down).collect();
        assert_eq!(alarms, [false, true], "seed {}: up, then down", seed);
    }
}

/// A FAILURE MEANS: two players read two different runs, a player's own number goes to the other, or the recap shows before the end.
#[test]
fn two_seats_read_one_run_and_the_recap_waits_for_the_end() {
    for seed in SEEDS {
        let mut p = office(seed, 2);
        p.run(1.0);
        p.seat_goes_down(1);
        p.run(0.5);
        let (a, b) = (p.debrief_view(0), p.debrief_view(1));
        assert!(a.recap.is_none() && b.recap.is_none(), "seed {}: one of them still plays", seed);
        assert!(!a.debrief.counts.down && b.debrief.counts.down, "seed {}: each his own fall", seed);
        p.seat_goes_down(0);
        p.run(0.5);
        let (a, b) = (p.debrief_view(0), p.debrief_view(1));
        assert!(a.recap.is_some() && b.recap.is_some(), "seed {}: both down, the card shows it to both", seed);
        assert_eq!(a.recap.unwrap().takes.len(), b.recap.unwrap().takes.len(), "seed {}", seed);
    }
}
