//! THE PANEL'S ROWS OF THE GAME'S NUMBERS (game_start host/shown.rs): each is a number of the game, and a row's slider turns it.
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};

fn play(seed: u32) -> Play {
    let mut s = Scene::new("rows");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([14.0, 21.0], std::f64::consts::PI, &[[13.0, 22.0], [15.0, 22.0]]);
    s.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    Play::scene(&s, &Opts { seed, squad: 2, ..Default::default() })
}

/// THE RULE: every row names a number the game has and starts inside its range; turned to a value in its range it reads back that value, a tick later too, and turned back it is as it was. A FAILURE
/// MEANS: the panel has a slider that moves nothing, a row of a number that left the tables, or a range its number starts outside. The control is a name no number has, which nothing turns.
#[test]
fn every_row_turns_a_number_of_the_game() {
    for seed in [1, 2, 3] {
        let mut p = play(seed);
        p.run(0.2);
        for r in game_start::host::shown::ROWS {
            let was = game_start::host::knobs::get(&p.g, r.name).unwrap_or_else(|| panic!("seed {seed}: {} is no number of the game", r.name));
            assert!(r.lo <= r.hi && r.step > 0.0 && r.scale > 0.0, "seed {seed}: {}: its range", r.name);
            assert!(was * r.scale >= r.lo - 1e-9 && was * r.scale <= r.hi + 1e-9, "seed {seed}: {} starts at {} on a row of {} to {}", r.name, was * r.scale, r.lo, r.hi);
            let to = (if was * r.scale > (r.lo + r.hi) / 2.0 { r.lo } else { r.hi }) / r.scale;
            assert!(game_start::host::knobs::set(&mut p.g, r.name, to), "seed {seed}: {} could not be turned", r.name);
            if r.name != "tune.flash_inner" { assert_eq!(game_start::host::knobs::get(&p.g, r.name), Some(to), "seed {seed}: {} reads another number than it was turned to", r.name); }
            p.frame();
            if r.name != "tune.flash_inner" { assert_eq!(game_start::host::knobs::get(&p.g, r.name), Some(to), "seed {seed}: {} did not last a tick", r.name); }
            assert!(game_start::host::knobs::set(&mut p.g, r.name, was));
            assert_eq!(game_start::host::knobs::get(&p.g, r.name), Some(was), "seed {seed}: {} turned back", r.name);
        }
        assert!(!game_start::host::knobs::set(&mut p.g, "no.such.number", 1.0), "seed {seed}");
        assert_eq!(game_start::host::knobs::get(&p.g, "no.such.number"), None, "seed {seed}");
    }
}

/// THE RULE: a kept row's number is one of the tables', so its revert dot can say what the game ships. A FAILURE MEANS: a kept row's dot would go to whatever the address or the last visit said.
#[test]
fn a_kept_row_knows_what_the_game_ships() {
    let p = play(1);
    for r in game_start::host::shown::ROWS.iter().filter(|r| r.kept) { assert_eq!(game_start::host::knobs::shipped(r.name), Some(game_start::host::knobs::get(&p.g, r.name).unwrap()), "{}", r.name); }
    assert_eq!(game_start::host::knobs::shipped("no.such.number"), None);
}

/// THE RULE: the flashlight's inner cone stays two degrees inside the outer, whichever is turned. A FAILURE MEANS: a row can put the torch's cone inside out.
#[test]
fn the_torchs_inner_cone_stays_inside_the_outer() {
    let mut p = play(1);
    assert!(game_start::host::knobs::set(&mut p.g, "tune.flash_outer", 40.0));
    assert!(game_start::host::knobs::set(&mut p.g, "tune.flash_inner", 30.0));
    assert!(game_start::host::knobs::set(&mut p.g, "tune.flash_outer", 20.0));
    assert_eq!(game_start::host::knobs::get(&p.g, "tune.flash_inner"), Some(18.0));
    assert!(game_start::host::knobs::set(&mut p.g, "tune.flash_inner", 25.0));
    assert_eq!(game_start::host::knobs::get(&p.g, "tune.flash_inner"), Some(18.0));
}

/// THE RULE: every address word names a number the game has, and it turns that number as it says; `?knob.<name>=<v>` turns a number of the tables. A word with nothing to say leaves it alone. A
/// FAILURE MEANS: a person's word for a switch does nothing on the page.
#[test]
fn an_address_word_turns_its_number() {
    use game_start::host::words::{apply, Says, WORDS};
    for w in WORDS {
        let mut p = play(1);
        let was = game_start::host::knobs::get(&p.g, w.knob).unwrap_or_else(|| panic!("{} is no number of the game", w.knob));
        apply(&mut p.g, "?nothing=2&other=1");
        assert_eq!(game_start::host::knobs::get(&p.g, w.knob), Some(was), "{}: an address without it", w.word);
        let (word, to) = match w.says { Says::Share => (format!("?{}=0.25", w.word), 0.25), Says::ZeroIs(z) => (format!("?{}=0", w.word), z), _ => (format!("?{}=0", w.word), 0.0) };
        apply(&mut p.g, &word);
        assert_eq!(game_start::host::knobs::get(&p.g, w.knob), Some(to), "{}", w.word);
    }
    let mut p = play(1);
    let name = "sneak.seen_rate";
    apply(&mut p.g, &format!("?knob.{name}=0.125"));
    assert_eq!(game_start::host::knobs::get(&p.g, name), Some(0.125));
    for no in ["?seenrate=0", "?seenrate=-1", "?seenrate=fast"] { apply(&mut p.g, no); assert_eq!(game_start::host::knobs::get(&p.g, name), Some(0.125), "{no} says nothing"); }
    apply(&mut p.g, "?seenrate=5");
    assert_eq!(game_start::host::knobs::get(&p.g, name), Some(1.0), "a share is 1 at most");
    assert!(game_start::host::knobs::get(&p.g, "cmdring.twice") == Some(1.0), "the staging: the double tap is on");
    apply(&mut p.g, "?qtwice=1");
    assert_eq!(game_start::host::knobs::get(&p.g, "cmdring.twice"), Some(1.0), "only 0 puts it off");
    let was = game_start::host::knobs::get(&p.g, "doorshot.toll");
    apply(&mut p.g, "?doorcost=1");
    assert_eq!(game_start::host::knobs::get(&p.g, "doorshot.toll"), was);
    let lean = game_start::host::knobs::get(&p.g, "fire.look.lean");
    apply(&mut p.g, "?firelean=1");
    assert_eq!(game_start::host::knobs::get(&p.g, "fire.look.lean"), lean, "only 0 is the old lean");
}

/// THE RULE: every action of the panel runs on a game and does what its row says, and it goes as a command. A FAILURE MEANS: a button on the panel throws or does nothing. The control is a name no
/// action has, which does nothing.
#[test]
fn every_action_runs_and_does_what_it_says() {
    use game_start::host::actions::{act, Takes, ACTIONS};
    for seed in [1, 2, 3] {
        let mut p = play(seed);
        p.sam_unhurt();
        p.run(0.2);
        for a in ACTIONS {
            let word = match &a.takes { Takes::Choice(all, _) | Takes::Each(all) => all(&p.g).first().cloned().unwrap_or_default(), _ => String::new() };
            let value = match a.takes { Takes::Count { start, .. } => start, _ => 0.0 };
            act(&mut p.g, a.name, value, &word);
            p.frame();
            if let Takes::Choice(_, now) = a.takes { assert_eq!(now(&p.g), word, "seed {seed}: {}", a.name); }
        }
        act(&mut p.g, "repair", 0.0, "");
        p.run(0.2);
        // the mains go and come back by the same row
        let up = p.g.light_home.mains.active;
        act(&mut p.g, "mains", 0.0, "");
        assert_ne!(p.g.light_home.mains.active, up, "seed {seed}: the mains row trips them");
        act(&mut p.g, "mains", 0.0, "");
        assert_eq!(p.g.light_home.mains.active, up, "seed {seed}: and restores them");
        // the player goes to the cursor
        p.g.sets[p.g.seat].aim_point = [15.0, 0.0, 14.0];
        act(&mut p.g, "teleport", 0.0, "");
        let at = p.g.ch[p.g.pl[p.g.player].ch].pos;
        assert_eq!((at[0], at[2]), (15.0, 14.0), "seed {seed}");
        // the camera steps through the men
        let was = p.g.spectate;
        act(&mut p.g, "spectate", 0.0, "");
        assert_ne!(p.g.spectate, was, "seed {seed}: spectate steps on");
        // more men stand, on floor, none on another; a mission with rooms counts them
        let men = p.g.guards.len();
        let placed = p.g.room_clear.placement.len();
        act(&mut p.g, "extra", 3.0, "");
        assert_eq!(p.g.guards.len(), men + 3, "seed {seed}: three more men stand");
        if game::roomclear::room_clear_scenario_active(&p.g) { assert_eq!(p.g.room_clear.placement.len(), placed + 3, "seed {seed}: the mission's"); }
        let spots: Vec<[f64; 3]> = p.g.guards[men..].iter().map(|&x| p.g.ch[p.g.gd[x].ch].pos).collect();
        for (i, a) in spots.iter().enumerate() {
            assert!(!game::scene_collision::nav_grid_is_blocked(&p.g, a[0], a[2]), "seed {seed}: on floor a man stands on: {a:?}");
            for b in &spots[..i] { assert!((a[0] - b[0]).hypot(a[2] - b[2]) >= 1.0, "seed {seed}: {a:?} on {b:?}"); }
        }
        p.run(0.5);
        // wet, and the footprints that come with it
        let c = p.g.pl[p.g.player].ch;
        act(&mut p.g, "soak", 0.0, "");
        assert_eq!(p.g.ch[c].wet, 1.0, "seed {seed}");
        assert_eq!(p.g.m.wet.mem.get(c).map(|m| m.prints_left), Some(p.g.k.wet.prints), "seed {seed}: a soaked man leaves prints");
        act(&mut p.g, "dryall", 0.0, "");
        assert_eq!(p.g.ch[c].wet, 0.0, "seed {seed}");
        // a pick is the game's, and what the row reads
        act(&mut p.g, "nerve", 0.0, "brittle");
        assert!(matches!(p.g.k.morale.nerve, game::morale::Nerve::Brittle), "seed {seed}");
        let g0 = p.g.guards.len();
        act(&mut p.g, "no such action", 0.0, "");
        assert_eq!(p.g.guards.len(), g0, "seed {seed}");
        // as a command, what a tape carries
        let mut cmd = Vec::new();
        cmd.extend_from_slice(&35u32.to_le_bytes());
        let t = "soak"; cmd.extend_from_slice(&(t.len() as u32).to_le_bytes()); cmd.extend_from_slice(t.as_bytes());
        cmd.extend_from_slice(&0.0f64.to_le_bytes());
        cmd.extend_from_slice(&0u32.to_le_bytes());
        game_start::host::command::apply(&mut p.g, &cmd).unwrap();
        assert_eq!(p.g.ch[c].wet, 1.0, "seed {seed}: an action goes as a command");
    }
}
