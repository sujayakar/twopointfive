//! A PICTURE OF A GAME TOUCHES NOTHING, AND HOLDS ITS PROGRAM WHOLE. A level's program in the middle of a beat goes on through a checkpoint as if none were taken, every man as he was, and a game put
//! back, in a fresh slot after another game has stood and gone, goes on as the game that was not put back did. A beat's place is plain data (`BeatFrame`), so nothing has to be stopped for a picture, or started again after one.
//! `PROGRAM_MOMENT_BREAK=1`: the moment comes back with a beat's place lost, and the tests must fail on it.
use game::game::Game;
use game::savestate::control;
use game::scene_scenario_list::{all_missions, pick_search};
use game::savestate::control::Moment;
use game_start::play::{Opts, Play};

const STAND_SECS: usize = 1;
const ON_SECS: usize = 20;
/// when the step with the checkpoint comes up
const CHECKPOINT_SECS: usize = 5;

fn stand(id: &str) -> Play {
    let e = all_missions().into_iter().find(|e| e.id == id).unwrap_or_else(|| panic!("{} is on no list", id));
    Play::mission(e.level.as_deref().unwrap_or("office.json"), &pick_search(&e, ""), &Opts { seed: 9, dress: true, ..Default::default() })
}
fn put_step(p: &mut Play, step: &str) { p.g.lesson.i = game::lesson::lesson_data(&p.g).unwrap().steps.iter().position(|t| t.id == step).unwrap_or_else(|| panic!("no step '{}'", step)) as f64; }
fn block(g: &Game) -> Vec<u8> { game_start::block::bytes(g.sim).to_vec() }
/// the game kept stands again in a fresh game's slot, as the page's module puts one back
fn lay(p: &mut Play, g: &Game, b: &[u8], slot: usize) { p.g = Moment { game: Box::new(g.clone()), block: b.to_vec() }.stands(slot); }
fn words(p: &mut Play) -> Vec<u32> {
    let (mut w, mut out) = (game::bytes::W::new(), Vec::new());
    game::state_words::state_words(&mut p.g, &mut w, &mut out);
    out.extend(game_start::block::part_hashes(p.g.sim));
    out
}
fn parting(a: &[u32], b: &[u32]) -> String {
    let names: Vec<String> = game::state_words::state_names().into_iter().chain(game_start::block::PART_NAMES.iter().map(|s| format!("sim.{}", s))).collect();
    names.iter().zip(a.iter().zip(b)).filter(|(_, (x, y))| x != y).map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(", ")
}
/// a beat of the running program that stands in its middle
fn in_the_middle(g: &Game) -> usize {
    let Some(r) = g.director.run.filter(|&r| !g.tab.director_run[r].ended) else { return 0 };
    g.tab.director_run[r].beats.iter().filter(|&&b| { let b = &g.tab.director_live_beat[b]; b.state == game::director::BeatState::Running && !matches!(b.it, game::director::BeatFrame::Spent) }).count()
}
/// the host's hands, a function of the tick: a walk, a stop, and the step with a checkpoint coming up
fn hands(p: &mut Play, i: usize, checkpoint: &str) {
    if i % 180 == 0 { p.key_down("KeyW"); }
    if i % 180 == 60 { p.key_up("KeyW"); }
    if i == CHECKPOINT_SECS * 60 { put_step(p, checkpoint); }
}
/// a frame, and what the end of its step told the host: what was kept, under which key, and how many games were put back
struct Host { kept: Vec<String>, taken: usize, back: usize }
impl Host {
    fn new() -> Host { Host { kept: Vec::new(), taken: 0, back: 0 } }
    fn frame(&mut self, p: &mut Play) {
        p.frame();
        for t in p.told.drain(..) {
            if let Some(key) = t.kept { self.kept.push(key); self.taken += 1; }
            if t.back { self.back += 1; }
        }
    }
}
/// the mission stood and on the step a beat runs on, a little way into it
fn into_a_beat(id: &str, step: &str) -> (Play, Host) {
    let (mut p, mut h) = (stand(id), Host::new());
    for _ in 0..STAND_SECS * 60 { h.frame(&mut p); }
    put_step(&mut p, step);
    for _ in 0..STAND_SECS * 60 { h.frame(&mut p); }
    assert!(in_the_middle(&p.g) > 0, "{}: no beat stands in its middle on '{}'", id, step);
    (p, h)
}

/// A FAILURE MEANS: a checkpoint plays the game: the men of a stopped program handed back awake at each one, as they used to be. The player's fight then is not the one he saved.
fn a_checkpoint_touches_nothing(id: &str, step: &str, checkpoint: &str) {
    let (mut p, mut h) = into_a_beat(id, step);
    let run = p.g.director.run;
    let starts = p.g.stats.rails.starts;
    for i in 0..CHECKPOINT_SECS * 60 { hands(&mut p, i, checkpoint); h.frame(&mut p); }
    let awake = |g: &Game| g.guards.iter().filter(|&&x| g.gd[x].script.is_some() && !game::condition::down(g, g.gd[x].ch)).map(|&x| (g.gd[x].callsign.clone(), g.gd[x].state)).collect::<Vec<_>>();
    let (men, taken) = (awake(&p.g), h.taken);
    assert!(!men.is_empty(), "{}: the program holds nobody", id);
    put_step(&mut p, checkpoint);
    h.frame(&mut p);
    h.frame(&mut p);
    assert_eq!(h.taken, taken + 1, "{}: the step '{}' took no checkpoint", id, checkpoint);
    assert_eq!(awake(&p.g), men, "{}: a checkpoint moved the men the program holds", id);
    assert!(p.g.director.run == run && in_the_middle(&p.g) > 0 && p.g.stats.rails.starts == starts, "{}: a checkpoint stopped the program", id);
    // and it is the same program that comes back
    let key = h.kept[0].clone();
    for _ in 0..60 { h.frame(&mut p); }
    assert_eq!(control::ask_load(&mut p.g, &key), None, "{}", id);
    h.frame(&mut p);
    assert_eq!(h.back, 1, "{}", id);
    for _ in 0..60 { h.frame(&mut p); }
    assert!(p.g.director.run == run && in_the_middle(&p.g) > 0 && p.g.stats.rails.starts == starts, "{}: a load started the program again", id);
}

/// A FAILURE MEANS: a game put back has lost something of the level's program: a beat's place, a signal seen, a trigger fired, a man it holds. The player's checkpoint then plays another fight.
fn the_program_goes_on(id: &str, step: &str, checkpoint: &str) {
    let (mut p, mut h) = into_a_beat(id, step);
    assert_eq!(control::run_no(&p.g, &game::seats::here(&p.g), false), None, "{}: F5 and the autosave are refused with a program running", id);
    let (mut g, b) = (p.g.clone(), block(&p.g));
    g.out.clear();
    let mut rows = Vec::new();
    for i in 0..ON_SECS * 60 { hands(&mut p, i, checkpoint); h.frame(&mut p); if i % 60 == 59 { rows.push(words(&mut p)); } }
    assert!(h.taken >= 1, "{}: nothing in the twenty seconds took a checkpoint", id);
    drop(p);
    let broken = std::env::var("PROGRAM_MOMENT_BREAK").is_ok();
    drop(stand("room-clear-four"));
    let mut q = stand(id);
    lay(&mut q, &g, &b, 1);
    assert!(in_the_middle(&q.g) > 0, "{}: the program came back with no beat in its middle", id);
    if broken { let beat = q.g.tab.director_run[q.g.director.run.unwrap()].beats[0]; q.g.tab.director_live_beat[beat].it = game::director::BeatFrame::Spent; }
    let mut h = Host::new();
    for i in 0..ON_SECS * 60 {
        hands(&mut q, i, checkpoint);
        h.frame(&mut q);
        if i % 60 == 59 {
            let w = words(&mut q);
            assert!(w == rows[i / 60], "{} put back: parts {} s on, in {}", id, (i + 1) / 60, parting(&rows[i / 60], &w));
        }
    }
}

#[test] fn on_the_kit_a_checkpoint_touches_nothing() { a_checkpoint_touches_nothing("tut-kit", "arch", "pistol"); }
#[test] fn on_the_road_a_checkpoint_touches_nothing() { a_checkpoint_touches_nothing("tut-road", "wall", "arch"); }
#[test] fn the_kit_s_program_goes_on_from_a_moment() { the_program_goes_on("tut-kit", "arch", "pistol"); }
#[test] fn the_road_s_program_goes_on_from_a_moment() { the_program_goes_on("tut-road", "wall", "arch"); }
