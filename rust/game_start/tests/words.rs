//! THE STATE'S WORDS ARE OF EVERY FIELD: a write anywhere in the game moves the word of its own part and no other, and what is a machine's own moves none.
use game::bytes::W;
use game::game::Game;
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};

fn words(p: &mut Play) -> Vec<u32> {
    let (mut o, mut out) = (W::new(), Vec::new());
    game::state_words::state_words(&mut p.g, &mut o, &mut out);
    out
}
fn office() -> Play {
    let mut s = Scene::new("office");
    s.room("office", 10.0, 8.0, 18.0, 14.0).room("far", 26.0, 8.0, 34.0, 14.0);
    s.door("office-yard", Axis::X, 14.0, 13.4).door("far-yard", Axis::X, 14.0, 29.4).door("far-back", Axis::X, 8.0, 29.4).bulb(14.0, 11.0).bulb(30.0, 11.0).moon();
    s.team([14.0, 21.0], std::f64::consts::PI, &[[13.0, 22.0], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]).defender("far", [32.25, 10.25], "far-yard").defender("office", [17.25, 9.25], "office-yard");
    Play::scene(&s, &Opts { seed: 3, seats: 2, ..Default::default() })
}
/// the parts whose word moved, by name
fn moved(p: &mut Play, write: impl FnOnce(&mut Game)) -> Vec<String> {
    let before = words(p);
    write(&mut p.g);
    let after = words(p);
    game::state_words::state_names().into_iter().zip(before.iter().zip(&after)).filter(|(_, (a, b))| a != b).map(|(n, _)| n).collect()
}

/// A FAILURE MEANS: two machines of one game can differ in that field and nobody is told.
#[test]
fn a_write_moves_the_word_of_its_own_part() {
    let mut p = office();
    p.run(0.5);
    assert_eq!(words(&mut p).len(), game::state_words::state_names().len());
    let gd = p.g.guards[0];
    assert_eq!(moved(&mut p, |g| g.time += 1.0), ["time"]);
    assert_eq!(moved(&mut p, |g| g.gd[gd].hold = !g.gd[gd].hold), ["gd"]);
    assert_eq!(moved(&mut p, |g| g.gd[gd].path.push([1.0, 0.0, 1.0])), ["gd"]);
    assert_eq!(moved(&mut p, |g| g.k.pressure.loud_secs += 1.0), ["k"]);
    // (a system's own slot, which only its module can name)
    assert_eq!(moved(&mut p, |g| game::control::set_hold(g, gd, !g.gd[gd].hold, "here")), ["gd", "ctl"]);
    // (a seat's set, of the seat that stands on Game and of the other)
    assert_eq!(moved(&mut p, |g| g.threat_proximity += 1.0), ["seats.threat_proximity"]);
    let other = p.g.seats[1];
    assert_eq!(moved(&mut p, |g| game::seats::with_seat(g, other, &mut |g| g.threat_proximity += 1.0)), ["seats.threat_proximity"]);
}

/// A FAILURE MEANS: a chair or a can nobody is near can stand elsewhere on one machine of a game, and no state word says so.
#[test]
fn a_chair_or_a_can_elsewhere_moves_a_word() {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 7, seats: 2, ..Default::default() });
    let can = game::throwables::throwables_spawn(&mut p.g, game::throwables::ItemKind::Frag, [14.0, 1.0, 21.0], [0.0; 3], None);
    p.run(3.0);
    let chair = p.g.props.props[0];
    let names = game_start::parts::names();
    for (what, write) in [("chair", &(|p: &mut Play| { let x = game::props::prop_x(&p.g, chair); game::props::prop_set_x(&mut p.g, chair, x + 1e-9) }) as &dyn Fn(&mut Play)), ("can", &|p: &mut Play| { let v = game::throwables::item_vel(&mut p.g, can); game::throwables::item_set_vel(&mut p.g, can, [v[0] + 1e-9, v[1], v[2]]) })] {
        let before = p.words();
        let again = p.words();
        assert!(game_start::parts::differ(&names, &before, &again).is_empty(), "asked twice, the words are the same");
        write(&mut p);
        let after = p.words();
        assert!(!game_start::parts::differ(&names, &before, &after).is_empty(), "{what}: no part's word moved");
    }
}

/// A FAILURE MEANS: a word is of where something lies in one process's memory, or of the process's own count: no two processes agree on it.
#[test]
fn two_processes_agree() {
    const ASKED: &str = "WORDS_ASKED";
    let mut p = office();
    p.run(1.0);
    let mine = format!("{:?}", words(&mut p));
    if std::env::var_os(ASKED).is_some() { println!("words {mine}"); return; }
    let other = std::process::Command::new(std::env::current_exe().unwrap()).args(["two_processes_agree", "--exact", "--nocapture", "--test-threads=1"]).env(ASKED, "1").output().unwrap();
    let said = String::from_utf8_lossy(&other.stdout).into_owned();
    let theirs = said.lines().find_map(|l| l.split_once("words ")).expect("the other process says its words").1;
    let list = |s: &str| s.trim_matches(['[', ']']).split(", ").map(String::from).collect::<Vec<_>>();
    let off: Vec<String> = game::state_words::state_names().into_iter().zip(list(&mine).into_iter().zip(list(theirs))).filter(|(_, (a, b))| a != b).map(|(n, _)| n).collect();
    assert!(off.is_empty(), "{off:?}");
}

/// A FAILURE MEANS: two machines of one game would be told they differ for what only one of them shows.
#[test]
fn what_is_a_machines_own_moves_no_word() {
    let mut p = office();
    p.run(0.5);
    let (here, other) = (p.g.seats[0], p.g.seats[1]);
    assert!(moved(&mut p, |g| game::seats::show_seat(g, other)).is_empty(), "which seat it shows");
    assert!(moved(&mut p, |g| game::seats::show_seat(g, here)).is_empty());
    assert!(moved(&mut p, |g| g.out.push(game::out_events::OutEvent::SmokeCansShut)).is_empty(), "what it told its picture");
    let cam = p.g.host_cam.expect("the host's camera");
    assert!(moved(&mut p, |g| g.tab.camera[cam].yaw += 1.0).is_empty(), "where it looks");
}
