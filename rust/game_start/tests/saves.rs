//! THE SAVE CONTROLLER (game::savestate::control), for a host that keeps games in memory. Here the host keeps a clone of the game and its block, as the page's
//! module does, and puts one back.
use game::savestate::control::{self, Do};
use game_start::play::{Opts, Play};

struct Host { p: Play, saved: Vec<String>, said: Vec<String>, backs: usize }

impl Host {
    fn office(seed: u32) -> Host { Host { p: Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() }), saved: Vec::new(), said: Vec::new(), backs: 0 } }
    fn frame(&mut self) {
        self.p.frame();
        for t in self.p.told.drain(..) {
            if let Some(key) = t.kept { self.saved.push(key); }
            if t.back { self.backs += 1; }
            if let Some(words) = t.said { self.said.push(words); }
        }
    }
    fn run(&mut self, secs: f64) { for _ in 0..(secs * 60.0) as usize { self.frame(); } }
}

/// THE RULE: a save by hand is made as the step it was asked in ends, it is this run's, and a load of it puts the man back where he stood with the run's counts as they
/// are now.
/// A FAILURE MEANS: F5 does nothing, or a load stands him somewhere else or gives him back the saves he had not made.
#[test]
fn a_save_by_hand_goes_back() {
    let mut h = Host::office(5);
    h.run(0.5);
    h.p.key_down("F5");
    h.frame();
    h.p.key_up("F5");
    assert_eq!(h.saved, ["manual.0"], "the key saves, as the step it went down in ends");
    let there = h.p.sam_at();
    assert_eq!(control::timeline(&h.p.g).len(), 1, "the card's strip has the save");
    h.p.key_down("KeyW");
    h.run(1.5);
    let moved = h.p.sam_at();
    assert!((moved[0] - there[0]).hypot(moved[1] - there[1]) > 0.5, "he walked away from the save");
    assert_eq!(control::ask_load(&mut h.p.g, "manual.0"), None);
    h.frame();
    assert_eq!(h.backs, 1);
    assert_eq!(h.said, ["checkpoint loaded \u{2014} by hand"], "and says so");
    let back = h.p.sam_at();
    assert!((back[0] - there[0]).hypot(back[1] - there[1]) < 0.05, "the load stands him where he saved: {there:?}, {back:?}");
    assert_eq!((h.p.g.saves.stats.manual, h.p.g.saves.stats.loads, h.p.g.chronicle.counts.saves), (1.0, 1.0, h.p.g.chronicle.counts.saves));
    assert_eq!(control::timeline(&h.p.g).len(), 1, "the save is still there to go back to again");
}

/// THE RULE: a fresh run has none of the last one's saves, and one of theirs cannot be asked for.
/// A FAILURE MEANS: a restart offers a checkpoint of the try before it.
#[test]
fn a_restart_begins_with_no_saves() {
    let mut h = Host::office(6);
    h.run(0.5);
    control::ask_save(&mut h.p.g);
    h.frame();
    game::game::game_restart(&mut h.p.g);
    h.run(0.2);
    assert!(control::timeline(&h.p.g).is_empty(), "the fresh run has the old one's save");
    assert!(control::ask_load(&mut h.p.g, "manual.0").is_some(), "and it can be loaded");
    h.p.g.saves.due = Some(String::from("manual.0"));
    h.frame();
    assert_eq!((h.backs, &h.said[..]), (0, &[String::from("checkpoint: that checkpoint is gone")][..]), "one asked for as a card does is refused in words");
}

/// THE RULE: the ring of saves by hand holds its number and takes the oldest's place; a save refused (he is down) keeps nothing and says why.
#[test]
fn the_ring_rolls_and_a_man_down_saves_nothing() {
    let mut h = Host::office(7);
    let n = h.p.g.k.savetimeline.manual as usize;
    for _ in 0..n + 1 { h.run(0.2); control::ask_save(&mut h.p.g); h.frame(); }
    assert_eq!(control::timeline(&h.p.g).len(), n);
    assert_eq!(h.saved.last().unwrap(), "manual.0", "past the ring's end the oldest goes");
    let pl = h.p.g.player;
    h.p.g.pl[pl].down = true;
    let before = h.saved.len();
    control::ask_save(&mut h.p.g);
    match control::tick(&mut h.p.g, 1.0 / 60.0) { Some(Do::Say(words)) => assert!(words.starts_with("not saved: "), "{words}"), _ => panic!("a man down was saved, or nothing was said") }
    assert_eq!((h.saved.len(), h.p.g.saves.stats.refused), (before, 1.0));
}

/// THE RULE: a game that goes back to its run's first step, as one lost to a panic does, still has the saves made after it, and they stay.
/// A FAILURE MEANS: the first step's game takes itself for a new run and lets every save go, so a second panic has nothing left but the start.
#[test]
fn back_at_the_runs_first_step_the_saves_stay() {
    let mut h = Host::office(5);
    h.run(0.5);
    h.p.key_down("F5");
    h.frame();
    h.p.key_up("F5");
    let key = h.saved[0].clone();
    let of = control::stood(&mut h.p.g);
    let kept = control::kept_of(of);
    let (_, first) = kept.iter().find(|(k, _)| k == control::START).expect("the run's first step is kept");
    let (shelf, new) = (h.p.g.saves.shelf.clone(), first.image.laid(h.p.g.sim));
    control::over(&mut h.p.g, new);
    h.p.g.saves.shelf = shelf;
    h.frame();
    assert!(h.p.g.saves.shelf.m.has(key.clone()), "the shelf let it go");
    assert!(control::kept_of(of).iter().any(|(k, _)| *k == key), "the game kept under it is gone");
}
