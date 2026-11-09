//! THE LAST FEW OF A ROOM CLEAR: when most count as down, and who is no man of their radio check.
mod stealth_ask;
use game_start::play::{Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
fn block(s: u32) -> Play { Play::mission("block.json", "?levelFile=block.json&roomclear&squad4", &Opts { seed: s, ..Default::default() }) }

/// THE RULE: the last few are never over a third of the men staged: on the block's eight, three up says nothing and two up starts the radio check.
/// A FAILURE MEANS: a large part of a held floor talks as if it were the last of them, or the last two never do.
#[test]
fn the_last_few_are_never_over_a_third_of_the_men_staged() {
    for s in SEEDS {
        for (left, starts) in [(3, false), (2, true)] {
            let mut p = block(s);
            p.sam_unhurt();
            p.run(0.5);
            let men = p.hostiles();
            assert_eq!(men.len(), 8, "seed {s}: the block stages eight");
            let keep: Vec<_> = men[..left].to_vec();
            p.drop_all_but(&keep);
            let said = p.until(8.0, |p| p.last_stand_checks() > 0.0).is_some();
            assert_eq!(said, starts, "seed {s}, {left} up: the check started: {:?}", p.last_stand_lines());
        }
    }
}

/// THE RULE: a man who has dropped his gun and run is no man of the radio check: with three up and one of them run, the two who still fight are the last few, and
/// the check goes out; he does not call it, is not asked for by name, does not answer and is not counted.
/// A FAILURE MEANS: the fault it was found as: "a green man saw his leader shot, dropped his gun, was cornered in his room, and then led the last stand's check and
/// closed it counting himself".
#[test]
fn a_man_who_dropped_his_gun_and_ran_answers_no_radio_check() {
    for s in SEEDS {
        let mut p = block(s);
        p.sam_unhurt();
        p.run(0.5);
        let men = p.hostiles();
        let keep: Vec<_> = men[..3].to_vec();
        p.drop_all_but(&keep);
        p.run(1.0);
        assert_eq!(p.last_stand_checks(), 0.0, "seed {s}: CONTROL: three up and nothing said");
        let ran = keep.iter().copied().find(|&m| p.breaks(m)).unwrap_or_else(|| panic!("seed {s}: nobody would run: {}", p.roll()));
        p.until(2.0, |p| p.fled(ran)).unwrap_or_else(|| panic!("seed {s}: he broke and he never ran: {}", p.why(ran)));
        assert!(p.up(ran), "seed {s}: he ran and he is up");
        p.until(14.0, |p| p.last_stand_lines().len() >= 2).unwrap_or_else(|| panic!("seed {s}: 14 s with two fighting and no check: {:?}", p.last_stand_lines()));
        p.run(4.0);
        let name = p.g.gd[ran].callsign.clone();
        let lines = p.last_stand_lines();
        assert!(lines.iter().all(|(who, text)| *who != name && !text.contains(&name)), "seed {s}: {name}, who ran, is on their net: {lines:?}");
        assert!(lines.iter().all(|(_, text)| !text.contains("3 of us")), "seed {s}: he was counted: {lines:?}");
    }
}
