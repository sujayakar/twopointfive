//! THE QUEUE LESSON: before its step Shift with the key is the plain key, and each step is done by the keys it names.
mod doors_ask;
use doors_ask::*;
use game::game::DownBy;
use game::ids::*;
use game_start::play::{dist, Opts, Play};

fn the_lesson(s: u32) -> Play {
    let mut p = Play::mission("tut-queue.json", "?levelFile=tut-queue.json&roomclear=1&squad4", &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.men_spared();
    p.run(0.5);
    p
}
fn up(p: &Play) -> String { p.view().lesson.and_then(|l| l.id).unwrap_or_default() }
fn ring_of(p: &Play, id: &str) -> [f64; 3] { game::lesson::lesson_data(&p.g).unwrap().steps.iter().find(|t| t.id == id).and_then(|t| t.at).unwrap_or_else(|| panic!("no ring for '{id}'")) }
/// Sam stood on a place by hand, the view at rest
fn stand(p: &mut Play, at: [f64; 3]) { p.sam_put([at[0], at[2]]); let f = p.sam_at(); p.cursor_on([f[0], 0.0, f[1]]); p.run(2.5); }
fn on(p: &Play, d: DoorId) -> Vec<GuardId> { p.men().into_iter().filter(|&m| p.doing(m).as_deref() == Some("stack") && p.order_door(m) == Some(d)).collect() }
fn watching(p: &Play, at: [f64; 3]) -> Vec<GuardId> { p.men().into_iter().filter(|&m| p.doing(m).as_deref() == Some("cover") && p.watch_of(m).map_or(false, |w| dist(w, [at[0], at[2]]) < 3.0)).collect() }
fn one_pair(p: &Play, m: &[GuardId]) -> bool { m.len() == 2 && p.callsign(m[0]).split(' ').next() == p.callsign(m[1]).split(' ').next() }

/// THE RULE: before its step Shift with the key is the plain key: on the guardroom's door all four stack. From its step it adds a goal: on the side door one whole pair goes and the other stays. The word is shut until a stack
/// stands on each door: pressed while the second pair walks it starts nothing, pressed on its own step it starts one on each door.
/// A FAILURE MEANS: the lesson's words and the keys have parted, and a player is stuck on a line, or a key does what the line has not taught.
#[test]
fn the_queue_lesson_opens_each_key_on_its_own_step() {
    for s in SEEDS {
        let mut p = the_lesson(s);
        let c = ring_of(&p, "corner");
        stand(&mut p, c);
        p.until(3.0, |p| up(p) == "door1").unwrap_or_else(|| panic!("seed {s}: the corner's step never went by: '{}'", up(&p)));
        let (a, b) = (p.door("guard"), p.door("guard_side"));
        let at = ring_of(&p, "door1");
        p.shift_tap(at);
        p.run(0.2);
        assert_eq!(on(&p, a).len(), 4, "seed {s}: before its step the plain key: all four on the door");
        p.until(3.0, |p| up(p) == "door2").unwrap_or_else(|| panic!("seed {s}: the first door's step never went by: '{}'", up(&p)));
        let at = ring_of(&p, "door2");
        p.shift_tap(at);
        p.run(0.2);
        let side = on(&p, b);
        assert!(one_pair(&p, &side) && on(&p, a).len() == 2, "seed {s}: from its step a goal: {} and {:?}", on(&p, a).len(), side);
        p.tap("KeyF");
        p.run(0.3);
        assert!(!p.any_op(), "seed {s}: the word with a pair still walking started an entry");
        p.until(60.0, |p| { let (x, y) = (on(p, a), on(p, b)); !x.is_empty() && !y.is_empty() && x.iter().chain(y.iter()).all(|&m| p.arrived(m)) }).unwrap_or_else(|| panic!("seed {s}: never stacked on both"));
        p.until(4.0, |p| up(p) == "word").unwrap_or_else(|| panic!("seed {s}: the word's step never came: '{}'", up(&p)));
        p.run(0.5);
        p.tap("KeyF");
        p.run(0.1);
        assert!(!p.ops_on(a).is_empty() && !p.ops_on(b).is_empty(), "seed {s}: on its step the word starts one on each door");
    }
}

/// THE RULE: no step lists the key tapped twice, so a double press on the first door's step is the key's order again: all four still stack on it and nobody falls in. The control is the same two taps off the lesson (goal_keys.rs:
/// the_key_tapped_twice_is_fall_in).
/// A FAILURE MEANS: a double press undoes what the step asked for.
#[test]
fn the_key_tapped_twice_is_not_the_lessons() {
    for s in SEEDS {
        let mut p = the_lesson(s);
        let c = ring_of(&p, "corner");
        stand(&mut p, c);
        p.until(3.0, |p| up(p) == "door1").unwrap_or_else(|| panic!("seed {s}: '{}'", up(&p)));
        let a = p.door("guard");
        let at = ring_of(&p, "door1");
        p.key_tapped_on(at);
        p.key_tapped_on(at);
        p.run(0.2);
        assert_eq!(on(&p, a).len(), 4, "seed {s}: still on the door");
        assert!(p.men().iter().all(|&m| p.doing(m).as_deref() != Some("fallIn")), "seed {s}: somebody fell in");
    }
}

/// the two door fights taken, the squad called and stood in the yard's ring, the first gate's step up
fn in_the_yard(s: u32) -> Play {
    let mut p = the_lesson(s);
    for m in p.g.guards.clone() { if p.g.gd[m].team != game::teams::Team::Operative && p.up(m) && ["Tango 1", "Tango 2", "Tango 3", "Tango 4"].contains(&p.callsign(m).as_str()) { p.falls(m, DownBy::Player); } }
    stand(&mut p, [28.4, 0.0, 16.0]);
    p.until(60.0, |p| up(p) == "call").unwrap_or_else(|| panic!("seed {s}: the call's step never came: '{}'", up(&p)));
    let f = p.sam_at();
    p.key_tapped_on([f[0], 1.0, f[1]]);
    p.run(0.2);
    let y = ring_of(&p, "yard");
    stand(&mut p, y);
    p.until(20.0, |p| up(p) == "gateA").unwrap_or_else(|| panic!("seed {s}: the first gate's step never came: '{}'", up(&p)));
    p.run(4.0);
    p
}

/// the keys a gate's line names, on its ring: Shift and the key kept down, the ring for a whole pair, no more than five wedges, its second wedge has them watch the gate. Who.
fn by_the_wheel(p: &mut Play, s: u32, at: [f64; 3]) -> Vec<GuardId> {
    assert!(p.shift_held(at), "seed {s}: no ring");
    let part = p.ring_part();
    let wedges = p.ring_wedges();
    assert!(one_pair(p, &part) && wedges.len() <= 5, "seed {s}: the ring: {part:?} {wedges:?}");
    p.wedge_taken(Some("cover"));
    p.run(0.2);
    part
}

/// THE RULE: both gates are the wheel's: a Shift tap gives nothing and says to keep the keys down, and so does the key with no Shift. Kept down with Shift the ring is a whole pair's, and its wedge has them watch the gate, the others staying on the
/// first. The tap on himself is never stopped: it calls them back, and the next line asks for the road gate where nobody watches it now. That wrong hand is the control for the first run's pairs: nothing is a dead end.
/// A FAILURE MEANS: a step of the yard can be a dead end.
#[test]
fn the_yards_gates_are_done_by_the_keys_their_lines_name() {
    for s in SEEDS {
        let mut p = in_the_yard(s);
        let (ga, gb) = (ring_of(&p, "gateA"), ring_of(&p, "gateB"));
        let n0 = p.orders_given();
        p.shift_tap(ga);
        p.key_tapped_on(ga);
        p.run(0.2);
        assert!(watching(&p, ga).is_empty() && p.orders_given() == n0 && up(&p) == "gateA", "seed {s}: a tap at the gate gave something: '{}'", up(&p));
        let note = p.view().lesson.and_then(|l| l.rails).and_then(|r| r.prompt).map_or(String::new(), |x| x.note.unwrap_or_default());
        assert!(note.contains("held down"), "seed {s}: it does not say to keep the keys down: '{note}'");
        let first = by_the_wheel(&mut p, s, ga);
        assert!(first.iter().all(|m| watching(&p, ga).contains(m)), "seed {s}: the first gate: {first:?}");
        p.until(4.0, |p| up(p) == "gateB").unwrap_or_else(|| panic!("seed {s}: '{}'", up(&p)));
        let second = by_the_wheel(&mut p, s, gb);
        assert!(second.iter().all(|m| !first.contains(m) && watching(&p, gb).contains(m)) && first.iter().all(|m| watching(&p, ga).contains(m)), "seed {s}: a pair on each: {first:?} {second:?}");
        p.until(6.0, |p| up(p) == "hold").unwrap_or_else(|| panic!("seed {s}: '{}'", up(&p)));
        drop(p);

        let mut p = in_the_yard(s);
        let (ga, gb) = (ring_of(&p, "gateA"), ring_of(&p, "gateB"));
        by_the_wheel(&mut p, s, ga);
        p.until(4.0, |p| up(p) == "gateB").unwrap_or_else(|| panic!("seed {s}: '{}'", up(&p)));
        let f = p.sam_at();
        p.key_tapped_on([f[0], 1.0, f[1]]);
        p.run(0.2);
        assert!(watching(&p, ga).is_empty() && p.men().iter().all(|&m| p.doing(m).as_deref() == Some("fallIn")), "seed {s}: the tap on himself calls them back");
        by_the_wheel(&mut p, s, gb);
        p.until(4.0, |p| up(p) == "split").unwrap_or_else(|| panic!("seed {s}: '{}'", up(&p)));
        let at = ring_of(&p, "split");
        by_the_wheel(&mut p, s, at);
        let (x, y) = (watching(&p, ga), watching(&p, gb));
        assert!(one_pair(&p, &x) && one_pair(&p, &y), "seed {s}: a pair on each: {x:?} {y:?}");
        p.until(6.0, |p| up(p) == "hold").unwrap_or_else(|| panic!("seed {s}: '{}'", up(&p)));
    }
}
