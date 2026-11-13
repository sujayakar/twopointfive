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
/// What the smoke the men see through was given last step, which starts at nothing on these stands and goes back to it.
fn lives(p: &Play) -> f64 { p.g.smoke.out }

#[test]
fn an_open_fire_in_view_files_its_smoke_and_no_man_sees_it() {
    let mut p = Play::level("pass.json", false, &Opts::default());
    let _t = (0..p.g.targets.len()).map(|i| p.g.targets[i]).find(|&t| game::fire::is_fire(&p.g, t)).expect("the pass has a brazier");
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
    let _t = (0..p.g.targets.len()).map(|i| p.g.targets[i]).find(|&t| game::fire::is_fire(&p.g, t)).expect("the pass has a brazier");
    p.run(4.0);
    let said = emitted(&p);
    assert!(!said.is_empty(), "a fire in view filed nothing");
    assert!(said.iter().all(|e| e.fire == Some(true)), "a source off a fire did not say so");
}

/// The page that draws a game it does not hold reads these sections, and a word lost here is a fire that no preset can shed or a jet that leaves ten times too fast.
#[test]
fn the_drawn_sections_say_it_the_same() {
    let mut p = Play::level("pass.json", false, &Opts::default());
    let _t = (0..p.g.targets.len()).map(|i| p.g.targets[i]).find(|&t| game::fire::is_fire(&p.g, t)).expect("the pass has a brazier");
    p.run(2.0);
    let said = p.smoke.clone();
    assert!(!said.is_empty(), "a fire in view filed nothing");
    p.g.out = said.clone();
    let mut d = game_draw::Drawn::default();
    d.take(&p.g, game_draw::want::events);
    assert_eq!(d.events.sources.len(), emitted(&p).len(), "a source was lost on the way to the page");
    assert!(d.events.sources.iter().all(|s| s.says & 16 != 0), "a fire's source did not tell the page it is a fire");

    for (e, s) in emitted(&p).iter().zip(&d.events.sources) {
        if s.push == 0 { continue; }
        let v = d.events.pushes[s.push as usize - 1].vel;
        let speed = ((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]) as f64).sqrt();
        assert!((speed - e.speed).abs() < 1e-3 * e.speed.max(1.0), "a source of {} m/s reaches the page at {}", e.speed, speed);
    }

}

#[test]
fn a_round_reaches_the_page_as_one_line() {
    let mut s = Scene::new("range");
    s.room("room", 10.0, 10.0, 30.0, 20.0).door("south", Axis::X, 20.0, 14.0);
    s.sam(13.0, 15.0);
    let mut p = Play::scene(&s, &Opts::default());
    p.sam_takes_his_rifle();
    p.sam_fires_at([28.0, 15.0], 0.5);
    let n = p.smoke.iter().filter(|e| matches!(e, OutEvent::SmokeSystemLikeRound(_))).count();
    assert!(n >= 1);
    p.g.out = p.smoke.clone();
    let mut d = game_draw::Drawn::default();
    d.take(&p.g, game_draw::want::events);
    assert_eq!(d.events.pushes.iter().filter(|a| a.shape == 5).count(), n, "the page was not told of the round's line");
}

/// One the team can't see files nothing the player could see his cigarette by.
#[test]
fn a_hidden_man_has_no_thread_to_ride() {
    let (mut p, _) = smoker();
    p.run(40.0);
    let t = threads(&p).first().and_then(|e| e.track.clone()).expect("he filed no thread");
    let game::effects::Track::Cigarette { by } = t else { unreachable!() };
    assert!(game::effects::track_at(&p.g, &t).is_some(), "a man in plain sight has no thread");
    p.g.fog_of_war.hidden_chars.add(by);
    assert!(game::effects::track_at(&p.g, &t).is_none(), "the thread rides on a man the player can't see");
}
