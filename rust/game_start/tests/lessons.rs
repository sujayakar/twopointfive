//! THE TUTORIALS, PLAYED (`game_start::lessons`): each one by a man at the keys, every step on its deed, to its end, as the page stands it. `TUT=<id>` picks one; `PACE=<secs>` has him read each line.
//! Each is ignored by `cargo test` because the longest takes a debug build over a minute: `cargo test --release --test lessons -- --ignored`.
use game::savestate::control;
use game::scene_scenario_list::{all_missions, pick_search};
use game::lesson::LessonTest;
use game_start::lessons::{tutorials, Lesson, Tutorial};
use game_start::owes::{self, Owes};
use game_start::play::{Opts, Play};

fn stand(id: &str) -> Play {
    let e = all_missions().into_iter().find(|e| e.id == id).unwrap_or_else(|| panic!("{} is on no list", id));
    Play::mission(e.level.as_deref().unwrap(), &pick_search(&e, ""), &Opts { seed: 9, dress: true, ..Default::default() })
}
fn tutorial(id: &str) -> &'static Tutorial { tutorials().into_iter().find(|t| t.id == id).unwrap() }

/// what his run did, a line a step, then the words it failed on
fn play(id: &str) {
    if std::env::var("TUT").is_ok_and(|t| t != id) { return; }
    let pace = std::env::var("PACE").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let mut p = stand(id);
    let mut l = Lesson::new(p.g.seats[0], tutorial(id), pace);
    let of = game::lesson::lesson_data(&p.g).expect("the mission has its lesson").steps.len();
    let mut held = Owes::new();
    let done = owes::played_out(&mut p, &mut l, 700.0, &mut held);
    let out = l.out.borrow().clone();
    println!("{}\n{}: {}/{} steps, {:.1} s, by the fight {:?}, {}", l.trace(), id, out.steps.len(), of, l.me.t(), out.by_fight, l.why().unwrap_or_else(|| String::from("ended")));
    println!("{}: {}", id, held.line());
    assert!(held.faults.is_empty(), "{}: {:?}", id, held.faults);
    assert!(done && out.ended, "{}: {} ({}/{} steps)\n{}", id, l.why().unwrap_or_else(|| String::from("he is still playing")), out.steps.len(), of, l.trace());
}
#[test] #[ignore = "the longest is over a minute in a debug build"] fn the_road() { play("tut-road"); }
#[test] #[ignore = "the longest is over a minute in a debug build"] fn the_team() { play("tut-team"); }
#[test] #[ignore = "the longest is over a minute in a debug build"] fn the_kit() { play("tut-kit"); }
#[test] #[ignore = "the longest is over a minute in a debug build"] fn the_queue() { play("tut-queue"); }
#[test] #[ignore = "the longest is over a minute in a debug build"] fn the_quiet() { play("tut-quiet"); }
#[test] #[ignore = "the longest is over a minute in a debug build"] fn the_pair() { play("tut-pair"); }

// ---- what is held with no run played

/// what is wrong with a test of a step once its mission stands: the lights and the men. (A word is the read's to refuse, a door, a room and a route the file's judge's: lesson_words.rs)
fn test_faults(p: &Play, t: &LessonTest, out: &mut Vec<String>) {
    use LessonTest as K;
    let g = &p.g;
    match t {
        K::Target { name, .. } => if !g.targets.iter().any(|&f| g.tab.sim_lightshome_fixture[f].name == *name) { out.push(format!("no light or device named '{}'", name)); },
        K::Did { what, .. } => if let Some(foe) = what.split_once("@foe:").map(|x| x.1) { if !g.guards.iter().any(|&x| g.gd[x].callsign == foe) { out.push(format!("no man called '{}' on this floor", foe)); } },
        K::All { of } | K::Any { of } => for x in of { test_faults(p, x, out); },
        _ => {}
    }
}
/// A FAILURE MEANS: a line the row cannot hold, a word no key is written out for, or a step that waits on a door, a room or a man the floor does not have, which no player can do.
#[test]
fn each_block_stands_on_its_floor() {
    let mut out = Vec::new();
    for t in tutorials() {
        let p = stand(t.id);
        let scen = p.g.level.scenarios.as_ref().expect("the level has scenarios");
        if scen.quiet.is_none() && scen.room_clear.is_none() && scen.rescue.is_none() && scen.night_breach.is_none() { out.push(format!("{}: a lesson and no scenario to run it over", t.id)); }
        let steps = &game::lesson::lesson_data(&p.g).expect("the mission has its lesson").steps;
        for (i, s) in steps.iter().enumerate() {
            let at = format!("{} step {} '{}'", t.id, i + 1, s.id);
            let mut faults = Vec::new();
            test_faults(&p, &s.done, &mut faults);
            if steps[..i].iter().any(|x| x.id == s.id) { faults.push(String::from("another step has its id")); }
            if s.cue.is_some() && steps[..i].iter().any(|x| x.cue == s.cue) { faults.push(format!("the cue {:?} is another step's too: a cue starts one beat", s.cue)); }
            if s.line.chars().count() > 96 { faults.push(format!("its line is {} characters: the objective row holds about 96", s.line.chars().count())); }
            for line in [Some(&s.line), s.setup_line.as_ref(), s.ring_line.as_ref(), s.blow_line.as_ref(), s.menu_line.as_ref(), s.page_line.as_ref(), s.flat_line.as_ref()].into_iter().flatten() {
                for name in line.split('{').skip(1).filter_map(|x| x.split_once('}').map(|y| y.0)) { if !game::lesson::lesson_token(name) { faults.push(format!("{{{}}} is no move's token", name)); } }
            }
            if !s.at.is_none_or(|a| a.iter().all(|v| v.is_finite())) { faults.push(format!("its mark {:?} is nowhere", s.at)); }
            out.extend(faults.into_iter().map(|f| format!("{}: {}", at, f)));
        }
    }
    assert!(out.is_empty(), "{}", out.join("\n"));
}

/// A FAILURE MEANS: a player put back at a checkpoint is back on a line he has done, or on one he has not reached.
#[test]
fn a_checkpoint_carries_the_step_that_is_up() {
    // (a quiet run alone keeps checkpoints: the frame takes them, as each step that names one comes up)
    for id in ["tut-road", "tut-quiet"] {
        let mut p = stand(id);
        let steps = game::lesson::lesson_data(&p.g).unwrap().steps.clone();
        let k = steps.iter().position(|s| s.checkpoint.is_some()).unwrap_or_else(|| panic!("{}: a tutorial played alone names no checkpoint on any step", id));
        let tick = |p: &mut Play, kept: &mut Option<String>| {
            p.frame();
            for told in p.told.drain(..) { if told.kept.is_some() { *kept = told.kept; } }
        };
        let mut kept = None;
        for _ in 0..30 { tick(&mut p, &mut kept); }
        kept = None;
        p.g.lesson.i = k as f64;
        for _ in 0..3 { tick(&mut p, &mut kept); }
        let key = kept.clone().unwrap_or_else(|| panic!("{}: no checkpoint was kept when step {} came up", id, k + 1));
        p.g.lesson.i = 0.0;
        assert_eq!(control::ask_load(&mut p.g, &key), None, "{}", id);
        for _ in 0..3 { tick(&mut p, &mut kept); }
        assert!(p.g.lesson.i >= k as f64, "{}: saved on step {}, loaded on step {}", id, k + 1, p.g.lesson.i + 1.0);
        let row = p.objective();
        assert!(row.contains(&format!("{}/{}", p.g.lesson.i + 1.0, steps.len())), "{}: the row reads {:?}", id, row);
    }
}

/// A FAILURE MEANS: a restart comes up on the last run's line, or a checkpoint made straight after it files that line.
#[test]
fn a_restart_puts_step_one_back() {
    for t in tutorials() {
        let mut p = stand(t.id);
        let of = game::lesson::lesson_data(&p.g).unwrap().steps.len();
        p.frame();
        p.g.lesson.i = 1.0;
        game::savestate::control::restart(&mut p.g, None);
        // (between frames: before the lesson's next tick the step it still holds would be the last run's)
        let first = format!("1/{}", of);
        assert!(game::lesson::lesson_index(&p.g) == 0.0 && p.objective().contains(&first) && !p.view().lesson.is_some_and(|v| v.finished), "{}: straight after a restart, before any frame: step {}, the row {:?}", t.id, game::lesson::lesson_index(&p.g) + 1.0, p.objective());
        p.run(0.5);
        assert!(p.g.lesson.i == 0.0 && p.objective().contains(&first), "{}: step {} after a restart, the row {:?}", t.id, p.g.lesson.i + 1.0, p.objective());
    }
}
