//! A KNEE FOR HIS WORK: Sam goes down to look under a door or work its lock, and his squad keeps what it was doing.
mod stealth_ask;
use stealth_ask::StealthAsk;
use game::game::{GuardState, InteractableKind, Verb};
use game::ids::*;
use game::orders::Who;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [3, 4, 5];
const WORKS: [&str; 2] = ["wand", "lock"];

/// four men fallen in on Sam at the server room's shut door, nobody noticing: one defender left, a long way off
fn at_the_door(s: u32, work: &str) -> (Play, DoorId, [f64; 2]) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(0.3);
    let door = p.door_at("server");
    let keep = *p.hostiles().iter().max_by(|a, b| { let d = |g: GuardId| { let q = p.at(g); (q[0] - door[0]).hypot(q[1] - door[1]) }; d(**a).total_cmp(&d(**b)) }).unwrap();
    p.drop_all_but(&[keep]);
    let there = p.at(keep);
    p.pin_man(keep, there, 0.0, GuardState::Patrol, 0.0);
    let d = game::doors::doors_by_name(&p.g, "server").unwrap();
    if work == "lock" { p.g.dr[d].leaf.locked = true; }
    let sam = p.g.player;
    p.g.pl[sam].wand = true;
    p.sam_put([door[0], door[1] + 0.9]);
    p.cursor_on([door[0], 1.0, door[1]]);
    p.order("fallIn", [door[0], door[1] + 1.6], Who::All);
    p.run(4.0);
    assert!(!p.low_men().0 && !p.g.pl[p.g.player].crouch, "seed {s}: the staging: up, and no one sneaking");
    (p, d, door)
}
fn starts(p: &mut Play, d: DoorId, work: &str) {
    if work == "wand" { assert!(p.row_taken(InteractableKind::Door, Verb::Wand), "no wand row on the shut door"); }
    else { game::player::start_picking(&mut p.g, d); }
    p.frame();
    let sam = p.g.player;
    assert!(p.g.pl[sam].crouch && (p.g.pl[sam].wanding().is_some() || p.g.pl[sam].picking().is_some()), "{work}: he never went down to it");
}
fn taps(p: &mut Play, key: &str) { p.key_down(key); p.frame(); p.key_up(key); p.run(0.2); }
trait Low { fn low_men(&self) -> (bool, bool); }
/// (any of the four is low, all of them are)
impl Low for Play { fn low_men(&self) -> (bool, bool) { let l: Vec<bool> = self.g.staged.team.iter().map(|&m| self.low(m)).collect(); (l.iter().any(|x| *x), l.iter().all(|x| *x)) } }
/// a step back from his work: he breaks it off, still on his knee
fn steps_off(p: &mut Play, door: [f64; 2]) {
    p.walk_keys_to(Some([door[0] + 0.1, door[1] + 2.0]));
    p.until(3.0, |p| { let s = p.g.player; p.g.pl[s].wanding().is_none() && p.g.pl[s].picking().is_none() }).expect("a step breaks it off");
    p.run(0.3);
    p.walk_keys_to(None);
    p.run(1.0);
    assert!(p.g.pl[p.g.player].crouch, "he stays on his knee");
}

/// THE RULE: nobody sneaking, Sam kneels to his work: not one of his squad goes low, not for the work and not after it, though he is still on his knee. The control: his own crouch key, and they are all low.
/// A FAILURE MEANS: the squad kneels whenever Sam looks under a door, or it never kneels with him at all.
#[test]
fn a_knee_for_his_work_takes_nobody_down_with_him() {
    for s in SEEDS {
        for work in WORKS {
            let (mut p, d, door) = at_the_door(s, work);
            starts(&mut p, d, work);
            let mut most = 0;
            for _ in 0..60 * 2 { p.frame(); if p.low_men().0 { most += 1; } }
            assert_eq!(most, 0, "seed {s}, {work}: low at his work");
            steps_off(&mut p, door);
            assert!(!p.low_men().0, "seed {s}, {work}: low after it");
            taps(&mut p, "KeyC");   // up
            taps(&mut p, "KeyC");   // and down by his own key
            p.run(1.0);
            assert!(p.low_men().1, "seed {s}, {work}: his own key no longer takes them down");
        }
    }
}

/// THE RULE: Sam low by his own key and his squad low with him, he works the door and gets up from nothing: they stay low through the look and after it, and stand with him when his key says so.
/// A FAILURE MEANS: a squad already sneaking stands up for his work.
#[test]
fn a_squad_that_was_sneaking_stays_low() {
    for s in SEEDS {
        for work in WORKS {
            let (mut p, d, door) = at_the_door(s, work);
            taps(&mut p, "KeyC");
            p.run(1.5);
            assert!(p.low_men().1, "seed {s}, {work}: the staging: low with him");
            starts(&mut p, d, work);
            let mut up = 0;
            for _ in 0..60 * 2 { p.frame(); if !p.low_men().1 { up += 1; } }
            assert_eq!(up, 0, "seed {s}, {work}: somebody stood for his work");
            steps_off(&mut p, door);
            assert!(p.low_men().1, "seed {s}, {work}: somebody stood after it");
            taps(&mut p, "KeyC");
            p.run(3.0);
            assert!(!p.low_men().0, "seed {s}, {work}: his key no longer stands them");
        }
    }
}
