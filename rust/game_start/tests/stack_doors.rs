//! A MOVE TRIES A DOOR NOBODY TRIED, AND A LOCK ON THE FACE IT OPENS TOWARD.
//! ("A stack refused only for Sam's body says so" is held already: world.rs sam_on_a_stacks_place_is_one_refusal_everywhere. The keypad on that face: offers.rs a_keypad_door_is_tried_then_greyed_in_the_words_it_was_refused_in.)
mod world_ask;
use game::orders::{squad_orders_give, squad_orders_target_at};
use game_start::play::{dist, Opts, Play};

const AWAY: [f64; 2] = [22.5, 11.2];
const INSIDE: [f64; 2] = [25.75, 8.5];
const CONFERENCE: [f64; 2] = [9.8, 8.0];
const SHUT: &str = "holding on it";

/// The office's room clear, the defenders kept blind (this is about feet), Sam down the corridor, the pair told to hold where it stands.
fn floor(s: u32) -> Play {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear", &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true;
    blind_all(&mut p);
    p.run(0.5);
    p.sam_put(AWAY);
    blind_all(&mut p);
    p.run(0.1);
    let at = p.sam_at();
    p.order("hold", at, game::orders::Who::All);
    blind_all(&mut p);
    p.run(1.0);
    p
}
fn blind_all(p: &mut Play) { for m in p.hostiles() { p.blinded(m, 60.0); } }
fn give(p: &mut Play, id: &str, at: [f64; 2]) -> Option<String> { let t = squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]); squad_orders_give(&mut p.g, id, Some(&t), None, None) }

/// THE RULE: a move whose only way in runs at a door locked against the team that nobody has tried is not refused with the door's name: the men walk up to it, the man on the jamb tries the handle, the order ends there as a door order ends: the same words, once, the men left on the door, on their own side of it. Only then does the team know, and the same order is refused.
/// A FAILURE MEANS: the menu reads a lock the men have not touched, or a move walks into a locked door for ever.
#[test]
fn a_move_tries_a_door_nobody_tried_and_stops_there() {
    {
        let s = 5u32;
        let mut p = floor(s);
        let d = game::doors::doors_by_name(&p.g, "manager").expect("the manager's door");
        assert!(p.g.dr[d].leaf.locked, "seed {s}: the staging: the door is not locked");
        assert_eq!(give(&mut p, "move", INSIDE), None, "seed {s}: nobody has tried it and the move was refused");
        let n = p.said.len();
        p.until(30.0, |p| p.said[n..].iter().any(|l| l.text.contains(SHUT))).unwrap_or_else(|| panic!("seed {s}: 30 s and nobody has said it is shut: {}", p.roll()));
        p.run(1.0);
        let told: Vec<_> = p.said[n..].iter().filter(|l| l.text.contains(SHUT)).map(|l| l.text.clone()).collect();
        assert_eq!(told.len(), 1, "seed {s}: {told:?}");
        for m in p.mates() { assert!(p.at(m)[1] > 9.8, "seed {s}: a man is inside the room: {:?}", p.at(m)); }
        let again = give(&mut p, "move", INSIDE);
        assert!(again.is_some(), "seed {s}: tried and found shut, and the same move is taken");
        assert!(p.g.dr[d].leaf.locked, "seed {s}: the door gave");
    }
}

/// THE CONTROL: a room whose door stands open: they walk in, and nothing is said of a door.
#[test]
fn a_move_to_an_open_room_says_nothing_of_a_door() {
    {
        let s = 5u32;
        let mut p = floor(s);
        assert_eq!(give(&mut p, "move", CONFERENCE), None, "seed {s}: the open room's move was refused");
        let n = p.said.len();
        p.until(30.0, |p| p.mates().iter().all(|&m| dist(p.at(m), CONFERENCE) < 2.5)).unwrap_or_else(|| panic!("seed {s}: 30 s and they are not in the room: {}", p.roll()));
        assert!(!p.said[n..].iter().any(|l| l.text.contains(SHUT)), "seed {s}: an open door was found shut");
    }
}

/// The room clear of four with nobody left to fight, the squad's big slots emptied by hand (a man with the hammer or the breaching shotgun takes such a lock himself), the manager's door hung to open toward the team.
fn lock_toward_us(s: u32) -> (Play, game::ids::DoorId, [f64; 2]) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.3);
    for m in p.mates() { if let Some(mut e) = p.g.m.kit.man_of.get(m).cloned() { e.kit.big = game::kit::BigKit::None; p.g.m.kit.man_of.set(m, e); } }
    p.drop_all_but(&[]);
    p.run(1.0);
    let d = game::doors::doors_by_name(&p.g, "manager").expect("the manager's door");
    let c = p.g.dr[d].frame_centre;
    let out = [c[0], 0.0, c[2] + 2.0];
    if !game::doors::doors_pull_face(&mut p.g, d, out) { let s = game::doors::door_swing(&mut p.g, d); game::doors::door_set_swing(&mut p.g, d, -s, None); }
    (p, d, [c[0], c[2]])
}

/// THE RULE (the maintainer, 10-08: "for doors just make it realistic"): a loud squad told "open and clear" at a cylinder lock on the face the door opens toward: nobody boots it; the lead says so, picks it, the keep is whole and they go in.
/// A FAILURE MEANS: men kick a door against its own frame.
#[test]
fn a_lock_on_the_face_it_opens_toward_is_picked_and_not_booted() {
    {
        let s = 7u32;
        let (mut p, d, c) = lock_toward_us(s);
        assert!(p.g.dr[d].leaf.locked, "seed {s}: the staging: not locked");
        assert_eq!(give(&mut p, "openClear", c), None, "seed {s}: refused");
        let mut kicks = 0;
        p.until(50.0, |p| { kicks += p.g.events.iter().filter(|e| e.kind == game::director::EventKind::Kick && e.time == p.g.time).count(); !p.g.dr[d].leaf.locked && p.g.sets[p.g.seat].door_tools.stats.picks >= 1.0 }).unwrap_or_else(|| panic!("seed {s}: 50 s and it is not picked: {:?}", p.lines(p.mates()[0])));
        assert_eq!(kicks, 0, "seed {s}: boots");
        assert!(!p.g.dr[d].leaf.lock_broken && p.said.iter().any(|l| l.text.contains("opens toward us. picking it")), "seed {s}: keep broken {}, said {:?}", p.g.dr[d].leaf.lock_broken, p.said.iter().map(|l| l.text.clone()).collect::<Vec<_>>());
    }
}

/// THE CONTROL: locked by something no pick opens while they walked: the lead says it opens toward them and the order ends at the door, no boot.
#[test]
fn a_lock_no_pick_opens_ends_the_order_at_the_door() {
    {
        let s = 7u32;
        let (mut p, d, c) = lock_toward_us(s);
        p.g.dr[d].leaf.locked = false;
        assert_eq!(give(&mut p, "openClear", c), None, "seed {s}: refused");
        p.run(0.33);
        p.g.dr[d].leaf.locked = true;
        p.g.dr[d].lock_id = format!("wedge:{}", p.g.dr[d].def.name);
        p.until(40.0, |p| p.said.iter().any(|l| l.text.contains("opens toward us. no boot"))).unwrap_or_else(|| panic!("seed {s}: 40 s and the lead has not said it: {:?}", p.said.iter().map(|l| l.text.clone()).collect::<Vec<_>>()));
        p.run(0.5);
        assert!(p.g.sets[p.g.seat].door_tools.ops.is_empty() && !p.g.dr[d].leaf.lock_broken && p.g.dr[d].leaf.locked, "seed {s}: ops {}, the door gave", p.g.sets[p.g.seat].door_tools.ops.len());
    }
}
