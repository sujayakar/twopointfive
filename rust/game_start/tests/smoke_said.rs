//! THE GAME SAYS WHAT SMOKE IT WANTS, AND ASKS NOTHING BACK: a fire files its source, a round its line, a smoking man his thread and what he breathes
//! out, whatever a host's solver is doing; the solver takes it or drops it. A failure means that smoke never reaches the page. Those sources are `look`:
//! they are the picture's, so the smoke the game's own men see through hears nothing of them.
use game::out_events::OutEvent;
use game_start::play::{Opts, Play};
use game::ids::GuardId;
use game_start::scene::{Axis, Scene};

fn emitted(p: &Play) -> Vec<game::smoke_types::SmokeEmitter> {
    p.smoke.iter().filter_map(|e| if let OutEvent::SmokeSystemLikeEmit(x) = e { Some(x.e.clone()) } else { None }).collect()
}
/// What the smoke the men see through has been given, all told.
fn lives(p: &Play) -> f64 { sim::state::state_read(p.g.sim).smoke.st[sim::bricks::H_OUT] }

#[test]
fn an_open_fire_in_view_files_its_smoke_and_no_man_sees_it() {
    let mut p = Play::level("pass.json", false, &Opts::default());
    let t = (0..p.g.targets.len()).map(|i| p.g.targets[i]).find(|&t| game::fire::is_fire(&p.g, t)).expect("the pass has a brazier");
    p.g.engine.view_focus = Some(game::fire::mouth_of(&mut p.g, t).pos);
    let before = lives(&p);
    p.run(4.0);
    let filed = emitted(&p);
    assert!(filed.iter().any(|e| e.look == Some(true)), "a fire in view filed no smoke in four seconds");
    assert_eq!(lives(&p), before, "a fire's smoke reached the smoke the men see through");
}

#[test]
fn a_round_files_one_line_and_no_row_of_dots() {
    let mut s = Scene::new("range");
    s.room("room", 10.0, 10.0, 30.0, 20.0).door("south", Axis::X, 20.0, 14.0);
    s.sam(13.0, 15.0);
    let mut p = Play::scene(&s, &Opts::default());
    p.sam_takes_his_rifle();
    p.sam_fires_at([28.0, 15.0], 0.5);
    let lines = p.smoke.iter().filter(|e| matches!(e, OutEvent::SmokeSystemLikeRound(_))).count();
    assert!(lines >= 1, "a round through the air filed no line");
    assert_eq!(p.smoke.iter().filter(|e| matches!(e, OutEvent::SmokeSystemLikePush(_))).count(), 0, "a round still shoved a row of dots");
    if let Some(OutEvent::SmokeSystemLikeRound(l)) = p.smoke.iter().find(|e| matches!(e, OutEvent::SmokeSystemLikeRound(_))) {
        assert!(l.len > 1.0 && (l.dir[0] * l.dir[0] + l.dir[1] * l.dir[1] + l.dir[2] * l.dir[2] - 1.0).abs() < 1e-6, "the line is {} m along a dir of the wrong length", l.len);
    }
}

/// A scene with one sentry told to smoke, run for forty seconds.
fn smoker() -> (Play, GuardId) {
    let mut s = Scene::new("yard");
    s.room("room", 10.0, 10.0, 20.0, 18.0).door("south", Axis::X, 18.0, 14.0);
    s.sentry("post", [12.0, 14.0], [18.0, 14.0]).sam(19.0, 16.0);
    let mut p = Play::scene(&s, &Opts::default());
    let gd = p.guard("post");
    p.g.gd[gd].script.get_or_insert_with(Default::default).smoke = Some(true);
    p.g.gd[gd].script.as_mut().unwrap().goal = None;
    (p, gd)
}
fn threads(p: &Play) -> Vec<game::smoke_types::SmokeEmitter> {
    emitted(p).into_iter().filter(|e| matches!(e.track, Some(game::effects::Track::Cigarette { .. }))).collect()
}

/// The solver is told nothing and asked nothing, so a thread it lost has to come back by itself: he files one a second, each living that second.
#[test]
fn a_man_who_smokes_keeps_filing_his_thread_and_breathes_out() {
    let (mut p, gd) = smoker();
    let before = lives(&p);
    p.run(40.0);
    assert!(game::smokebreak::smoking(&p.g, gd), "the man stopped smoking");
    let t = threads(&p);
    assert!(t.len() >= 30 && t.len() <= 40, "he filed {} threads in forty seconds of standing there", t.len());
    assert!(t.iter().all(|e| e.ttl <= 1.0), "a thread outlives the next one, so a lost word would double it");
    assert!(emitted(&p).iter().any(|e| e.track.is_none() && e.look == Some(true)), "he never breathed out");
    assert_eq!(lives(&p), before, "his smoke reached the smoke the men see through");
    let n = t.len();
    p.g.gd[gd].script.as_mut().unwrap().smoke = Some(false);
    p.run(3.0);
    assert_eq!(threads(&p).len(), n, "he went on filing a thread after he stopped");
}

#[test]
fn a_fire_says_it_is_a_fire_so_a_preset_can_shed_it() {
    let mut p = Play::level("pass.json", false, &Opts::default());
    let t = (0..p.g.targets.len()).map(|i| p.g.targets[i]).find(|&t| game::fire::is_fire(&p.g, t)).expect("the pass has a brazier");
    p.g.engine.view_focus = Some(game::fire::mouth_of(&mut p.g, t).pos);
    p.run(4.0);
    assert!(emitted(&p).iter().all(|e| e.fire == Some(true)), "a source off a fire did not say so");
}
