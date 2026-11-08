//! A DOOR FOUND SHUT, AND THE SIDE A DOOR IS WORKED FROM. The maintainer, of barricaded doors and of the AI at a door: "open has to be on the handle side for AI".
mod doors_ask;
use doors_ask::*;
use game::orders::Who;
use game_start::play::{Opts, Play};

fn two(seed: u32) -> Opts { Opts { seed, squad: 2, ..Default::default() } }
/// the pair on the yard, the floor clear, the west door as the scene has it or wedged from inside
fn stood(seed: u32, wedged: bool) -> (Play, game::ids::DoorId, [f64; 2]) {
    let (s, at) = yard(seed);
    let mut p = Play::scene(&s, &two(seed));
    p.sam_unhurt();
    p.run(0.2);
    p.floor_cleared();
    let d = p.door("west-yard");
    if wedged { let from = [at[0], at[1] + 1.0]; p.wedged_from(d, from); }
    p.run(0.2);
    (p, d, at)
}
fn ends(p: &mut Play, d: game::ids::DoorId, secs: f64) { p.until(3.0, |p| !p.ops_on(d).is_empty()); p.until(secs, |p| p.ops_on(d).is_empty()); }

/// THE RULE: an entry order at a door nobody knew was wedged stops at the handle: he says what he found, nothing is booted, the men stay stacked on
/// the near side. The control is the same order on the same door unwedged: it opens and they go in.
/// A FAILURE MEANS: the squad boots or walks away from a door it has only just found shut (`door_tools_halt_at`), and the player has no say in it.
#[test]
fn an_entry_order_stops_at_a_door_it_finds_wedged() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, true);
        p.order("openClear", at, Who::All);
        ends(&mut p, d, 40.0);
        let st = p.tool_stats();
        assert_eq!((st.halted, st.kicks), (1.0, 0.0), "seed {s}: halted once and not booted: {st:?}");
        assert!(p.door_shut(d), "seed {s}: the door is still shut");
        assert!(p.said_it("wedged from inside. holding on it"), "seed {s}: he says what he found: {:?}", p.said);
        let side = p.face(d, [at[0], at[1] + 1.0]);
        assert!(p.across(d, side).is_empty(), "seed {s}: nobody is across the frame");
        for m in p.men() { assert_eq!((p.doing(m).as_deref(), p.order_door(m)), (Some("stack"), Some(d)), "seed {s}: every man is left stacked on it"); }
        drop(p);

        let (mut p, d, at) = stood(s, false);
        p.order("openClear", at, Who::All);
        let side = p.face(d, [at[0], at[1] + 1.0]);
        p.until(30.0, |p| !p.across(d, side).is_empty()).unwrap_or_else(|| panic!("seed {s}: the control: 30 s and nobody went in: {}", p.roll()));
        assert_eq!(p.tool_stats().halted, 0.0, "seed {s}: the control: nothing halts at a door that is not shut");
    }
}

/// THE RULE: once it is known, the door's row says so, and the same row is now the boot, thrown from beside the latch jamb: out of the opening, on the side the leaf goes from.
/// A FAILURE MEANS: the row reads as before, or the boot was given from in front of the leaf.
#[test]
fn the_row_then_reads_kick_and_the_boot_comes_from_beside_the_latch() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, true);
        p.order("openClear", at, Who::All);
        ends(&mut p, d, 40.0);
        assert_eq!(p.row("openClear", at), ("kick and clear · wedged from inside".to_string(), None), "seed {s}");
        assert!(p.row("bangClear", at).0.starts_with("kick, bang and clear · "), "seed {s}: {:?}", p.row("bangClear", at));
        p.order("openClear", at, Who::All);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        let lead = p.op_lead(op);
        let t = p.until(25.0, |p| p.tool_stats().kicks > 0.0).unwrap_or_else(|| panic!("seed {s}: no boot in 25 s: {:?}", p.op_beats(op)));
        let (a, w) = (p.along(d, p.at(lead)), p.door_w(d));
        assert!(a > w && a < w + game::doors::side_kick().past + 0.45, "seed {s}: he kicks {a:.2} m along a {w:.2} m leaf from its hinge, {t:.1} s on");
        p.until(5.0, |p| !p.door_shut(d)).unwrap_or_else(|| panic!("seed {s}: the boot and it held"));
        assert_eq!(p.tool_stats().halted, 1.0, "seed {s}: the second order is not halted");
    }
}

/// THE RULE: a plain 'stack up' tries the handle too: the lead says it, and the entry order after it is not halted but goes straight to the boot.
/// The control is a stack on the same door unwedged: nobody has anything to report, and the door is not known shut.
/// A FAILURE MEANS: a stack no longer learns its door, so the player's next order burns a second order finding out.
#[test]
fn a_plain_stack_tries_the_handle() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, true);
        p.order("stack", at, Who::All);
        p.until(30.0, |p| p.tool_stats().tested > 0.0).unwrap_or_else(|| panic!("seed {s}: 30 s and nobody tried it: {}", p.roll()));
        assert!(p.said_it("wedged from inside."), "seed {s}: {:?}", p.said);
        p.order("openClear", at, Who::All);
        p.until(25.0, |p| !p.door_shut(d)).unwrap_or_else(|| panic!("seed {s}: the entry did not open it: {:?}", p.tool_stats()));
        let st = p.tool_stats();
        assert_eq!((st.halted, st.kicks), (0.0, 1.0), "seed {s}: {st:?}");
        drop(p);

        let (mut p, d, at) = stood(s, false);
        p.order("stack", at, Who::All);
        p.until(30.0, |p| p.tool_stats().tested > 0.0).unwrap_or_else(|| panic!("seed {s}: 30 s and nobody tried it"));
        assert!(!p.said_it("wedged"), "seed {s}: the control: {:?}", p.said);
        assert!(p.door_shut(d), "seed {s}: the control: a stack tries the handle, it does not open the door");
    }
}
