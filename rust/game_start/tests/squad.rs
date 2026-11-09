//! THE SQUAD'S WORDS, AS A HUD IS TOLD THEM (`view::tick_view`, `squad`; rust/game_rules/src/squad_orders/orders/hud.rs): the page reads these and words nothing of its own.
use game::game::DownBy;
use game::orders::Who;
use game::view::{MateLine, SquadView};
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: f64 = std::f64::consts::PI;

/// two rooms, each with a door on the yard and one between them, a defender in each; the team in the yard. A seed is another place for the near door
fn yard(seed: u32) -> (Scene, [f64; 2]) {
    let x = 13.4 + nudge(seed, 0);
    let mut s = Scene::new("squad-yard");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, x).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([x, 22.0], NORTH, &[[x - 1.0, 23.0], [x + 1.0, 23.0], [x - 1.0, 24.0], [x + 1.0, 24.0]]);
    s.defender("west", [11.25, 11.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    (s, [x + 0.6, 16.0])
}
fn squad(p: &Play) -> SquadView { p.view().squad.expect("a squad answers to Sam") }
fn doing(p: &Play) -> Vec<String> { squad(p).mates.iter().map(|m: &MateLine| m.doing.clone()).collect() }

/// A FAILURE MEANS: a man's line is not the order he was given, or does not move on when he gets there, or a man who is down reads as anything else; the
/// control is the pair that was not sent, whose line says what it said.
#[test]
fn each_mans_line_is_his_orders_word_as_he_goes_and_when_he_gets_there() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.run(0.2);
        let (before, mates) = (doing(&p), squad(&p).mates);
        assert_eq!(mates.len(), 4, "seed {}", seed);
        assert!(mates.iter().all(|m| !m.down && !m.name.is_empty() && m.man >= 0.0), "seed {}: {:?}", seed, mates);
        let red: Vec<usize> = mates.iter().enumerate().filter(|(_, m)| m.pair == Some(game::view::Pair::Red)).map(|(i, _)| i).collect();
        let blue: Vec<usize> = mates.iter().enumerate().filter(|(_, m)| m.pair == Some(game::view::Pair::Blue)).map(|(i, _)| i).collect();
        assert!(red.len() == 2 && blue.len() == 2, "seed {}: four men are two pairs: {:?}", seed, mates);
        p.order("stack", door, Who::Red);
        let now = doing(&p);
        assert!(red.iter().all(|&i| now[i].starts_with("stacking · ")), "seed {}: the order's word, in the same tick: {:?}", seed, now);
        assert!(blue.iter().all(|&i| now[i] == before[i]), "seed {}: the other pair was not sent, in the same tick: {:?} → {:?}", seed, before, now);
        p.until(30.0, |p| { let d = doing(p); red.iter().all(|&i| d[i].starts_with("stacked · ")) }).unwrap_or_else(|| panic!("seed {}: the pair never got there: {:?}", seed, doing(&p)));
        let d = doing(&p);
        assert!(blue.iter().all(|&i| !d[i].starts_with("stack")), "seed {}: {:?}", seed, d);
        assert!(p.view().squad.is_some_and(|v| v.mates.iter().any(|m| m.doing.contains(" · on "))), "seed {}: and a man's line ends on what he covers: {:?}", seed, d);
        // a man down reads down, and only he does
        let fell = p.mates().into_iter().find(|&gd| p.id_of(gd) == mates[red[0]].man).expect("the line names him by his character's id");
        p.falls(fell, DownBy::Player);
        let v = squad(&p);
        assert!(v.mates[red[0]].down && ["down", "wounded"].contains(&v.mates[red[0]].doing.as_str()), "seed {}: {:?}", seed, v.mates[red[0]]);
        assert!(!v.mates[red[1]].down && !v.mates[blue[0]].down && !v.mates[blue[1]].down, "seed {}: {:?}", seed, v.mates);
    }
}

/// a charge set on the door by the squad, Sam well clear of it. None: it was not set in the time
fn charged(seed: u32) -> (Play, [f64; 2]) {
    let (s, door) = yard(seed);
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.2);
    assert_eq!(p.seat_door_charges(0), 0);
    p.order("charge", door, Who::All);
    p.until(60.0, |p| p.seat_door_charges(0) > 0).unwrap_or_else(|| panic!("seed {}: no charge on the door in a minute: {:?}", seed, doing(&p)));
    (p, door)
}
fn says_you(p: &Play) -> bool { p.view().corner.iter().any(|n| n.text.contains("TOO CLOSE: you")) }

/// A FAILURE MEANS: the corner does not warn a man who stands at a charge he can blow, or it still warns him after he has stepped away: a warning kept by
/// the second (it was kept a quarter of one) is a warning of where he was. Nothing runs between the two questions here.
#[test]
fn too_close_is_as_he_stands_in_the_same_tick() {
    for seed in SEEDS {
        let (mut p, door) = charged(seed);
        assert!(!says_you(&p), "seed {}: he is six metres off: {:?}", seed, p.view().corner);
        assert!(squad(&p).notes.iter().any(|n| n.text.starts_with("charge set · ")), "seed {}: {:?}", seed, squad(&p).notes);
        let (near, far) = ([door[0], door[1] + 0.9], [door[0], door[1] + 9.0]);
        for _ in 0..3 {
            p.sam_put(near);
            assert!(says_you(&p), "seed {}: at the door, in the same tick: {:?}", seed, p.view().corner);
            p.sam_put(far);
            assert!(!says_you(&p), "seed {}: away from it, in the same tick: {:?}", seed, p.view().corner);
        }
        // CONTROL: no charge, nothing to warn of, wherever he stands
        drop(p);
        let (s, _) = yard(seed);
        let mut q = Play::scene(&s, &Opts { seed, ..Default::default() });
        q.run(0.2);
        q.sam_put(near);
        assert!(q.view().corner.is_empty(), "seed {}: {:?}", seed, q.view().corner);
    }
}

/// A FAILURE MEANS: a question of the squad's words writes the game (with a charge set, so that the lines ask the box world too). Two machines ask at different times.
#[test]
fn asking_writes_nothing_with_a_charge_set() {
    for seed in SEEDS {
        let run = |ask: bool| -> Vec<Vec<u32>> {
            let (mut p, door) = charged(seed);
            p.sam_put([door[0], door[1] + 0.9]);
            let mut out = Vec::new();
            for _ in 0..4 {
                for _ in 0..30 { p.frame(); if ask { assert!(says_you(&p), "seed {}: the question must reach the box world: {:?}", seed, p.view().corner); } }
                out.push(p.words());
            }
            out
        };
        let (never, always) = (run(false), run(true));
        for (s, (a, b)) in never.iter().zip(always.iter()).enumerate() {
            let parts: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
            assert!(parts.is_empty(), "seed {}: after step {} the asked game differs in parts {:?}", seed, s, parts);
        }
    }
}

/// A GAME OF TWO: each player's block is about his men and his closeness, whoever's things stand on the game. A FAILURE MEANS: the second machine reads the first
/// player's men, or is warned of where the first player stands. The control is the first player's block, which does not move with the second's order.
#[test]
fn in_a_game_of_two_each_is_told_of_his_own_men_and_his_own_closeness() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, seats: 2, squad: 2, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        let (one, two) = (p.view_of(0).squad.expect("the first player's squad"), p.view_of(1).squad.expect("the second player's squad"));
        assert!(!one.mates.is_empty() && !two.mates.is_empty(), "seed {}: {:?} {:?}", seed, one.mates, two.mates);
        assert!(one.mates.iter().all(|a| two.mates.iter().all(|b| a.man != b.man)), "seed {}: each has his own men: {:?} {:?}", seed, one.mates, two.mates);
        p.seat_orders(1, "stack", door, Who::All);
        let (v1, v2) = (p.view_of(0).squad.unwrap(), p.view_of(1).squad.unwrap());
        assert!(v2.mates.iter().all(|m| m.doing.starts_with("stacking · ")), "seed {}: the second's men have their order's word: {:?}", seed, v2.mates);
        assert_eq!(v1.mates.iter().map(|m| m.doing.clone()).collect::<Vec<_>>(), one.mates.iter().map(|m| m.doing.clone()).collect::<Vec<_>>(), "seed {}: the first's men did not", seed);
        p.seat_orders(1, "charge", door, Who::All);
        p.until(60.0, |p| p.seat_door_charges(1) > 0).unwrap_or_else(|| panic!("seed {}: no charge in a minute: {:?}", seed, p.view_of(1).squad.map(|v| v.mates)));
        assert_eq!(p.seat_door_charges(0), 0, "seed {}", seed);
        assert!(p.view_of(1).squad.unwrap().notes.iter().any(|n| n.text.starts_with("charge set · ")), "seed {}", seed);
        assert!(!p.view_of(0).squad.unwrap().notes.iter().any(|n| n.text.starts_with("charge set · ")), "seed {}: it is not his charge", seed);
        let (near, far) = ([door[0], door[1] + 0.9], [door[0], door[1] + 9.0]);
        let you = |p: &Play, seat: usize| p.view_of(seat).corner.iter().any(|n| n.text.contains("TOO CLOSE: you"));
        // his own man at the door, the other away
        p.seat_stands(1, near); p.seat_stands(0, far);
        assert!(you(&p, 1) && !you(&p, 0), "seed {}: the second man stands at his charge", seed);
        // the other at the door, his own away
        p.seat_stands(1, far); p.seat_stands(0, near);
        assert!(!you(&p, 1) && !you(&p, 0), "seed {}: it is the first man who stands there: {:?}", seed, p.view_of(1).corner);
    }
}

/// A FAILURE MEANS: the block does not say who a set wall charge would catch, or says it of where he stood. The control is the block before the charge, which names no wall. Nothing runs between two questions.
#[test]
fn a_set_wall_charge_names_who_is_too_close_in_the_same_tick() {
    const WALL: [f64; 3] = [12.06, 0.0, 13.78];   // (on the office's partition, as tests/wallcharge.rs has it)
    for seed in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.1);
        p.sam_put([14.6, 15.2]);
        p.frame();
        let wall = |p: &Play| squad(p).notes.into_iter().find(|n| n.text.starts_with("blow the charge · the wall"));
        assert!(wall(&p).is_none(), "seed {}: nothing on a wall yet", seed);
        p.orders_wall_charge(WALL);
        p.until(90.0, |p| p.wall_charges() > 0).unwrap_or_else(|| panic!("seed {}: no charge on the wall in time: {:?}", seed, doing(&p)));
        assert_eq!(wall(&p).expect("a charge is on the wall").key, "F", "seed {}", seed);
        let (near, far) = (p.off_the_wall_charge(0.8), p.off_the_wall_charge(9.0));
        let says = |p: &Play| wall(p).unwrap().text.contains("TOO CLOSE: you");
        for _ in 0..3 {
            p.sam_put(near);
            assert!(says(&p), "seed {}: at the wall, in the same tick: {:?}", seed, wall(&p));
            p.sam_put(far);
            assert!(!says(&p), "seed {}: away from it, in the same tick: {:?}", seed, wall(&p));
        }
    }
}

/// A FAILURE MEANS: the key's line for the word is not up while men wait on it, or names the wrong men. The control is the block before any order.
#[test]
fn the_word_is_on_its_key_while_men_wait_on_it() {
    let go = |p: &Play| squad(p).notes.into_iter().find(|n| n.text.starts_with("go · "));
    for seed in SEEDS {
        // a squad of four, each pair on a door of its own
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.run(0.2);
        assert!(go(&p).is_none(), "seed {}: nobody waits yet", seed);
        p.order("stack", door, Who::Red);
        let n = go(&p).expect("the first pair waits on the word");
        assert_eq!((n.key.as_str(), n.text.as_str()), ("F", "go · red waiting"), "seed {}", seed);
        p.order("stack", [22.0, 16.0], Who::Blue);
        let n = go(&p).expect("both pairs wait on the word");
        assert_eq!((n.key.as_str(), n.text.as_str()), ("F", "go · red + blue waiting"), "seed {}", seed);
        drop(p);
        // a squad of two: its plain stack
        let (s, door) = yard(seed);
        let mut q = Play::scene(&s, &Opts { seed, squad: 2, ..Default::default() });
        q.run(0.2);
        assert!(go(&q).is_none(), "seed {}: nobody waits yet", seed);
        q.order("stack", door, Who::All);
        let n = go(&q).expect("the pair waits on the word");
        assert_eq!(n.key, "F", "seed {}", seed);
        assert!(n.text.ends_with(" waiting"), "seed {}: {:?}", seed, n);
    }
}
