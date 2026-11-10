//! THE RUNS PLAYED TO THEIR ENDS: `played <id>`, the hands-off player on a mission, and `lesson <id>`, a man at the keys on a tutorial. These are what is left of the play-throughs, the lesson players and the soak.
use crate::pins::Pins;
use crate::rows::{held, mission, pin_of, say, stand, Flags, SEED};
use game::cmd::PlayerCmd;
use game::game::Game;
use game::scene_scenarios::MissionKind as Kind;
use game_start::figures::Track;
use game_start::hands_off::{End, HandsOff, Plan};
use game_start::lessons::{tutorials, Lesson};
use game_start::owes::{self, Owes};
use game_start::player::Script;
use std::time::Instant;

/// seconds of his run after which he stops, and of the run with his stopping in it
const CAP: f64 = 240.0;
const TICKS: usize = 245 * 60;
/// seconds of a tutorial's run
const LESSON_SECS: f64 = 700.0;
/// seconds a man reads a line for, on a tutorial's slow road
pub const READS: f64 = 6.0;
/// when a man is put in a wall (the `wall` break)
const WALL_AT: usize = 5 * 60;

/// the missions he plays: every room clear, the rescues, and the night breach. (A tutorial stages a room clear as well, and has its own player)
pub fn his_missions() -> Vec<String> {
    let staged = |e: &game::scene_scenarios::ScenarioEntry| matches!(e.kind, Kind::Roomclear | Kind::Rescue | Kind::Nightbreach) && !lessons().contains(&e.id.as_str());
    game::scene_scenario_list::all_missions().into_iter().filter(staged).map(|e| e.id).collect()
}
pub fn lessons() -> Vec<&'static str> { tutorials().into_iter().map(|t| t.id).collect() }

/// a script with something done to his hand after he has written it (what a break does to him)
struct Marred<'a> { him: &'a mut dyn Script, mar: fn(&Game, &mut PlayerCmd) }
impl Script for Marred<'_> {
    fn hand(&mut self, g: &Game, cmd: &mut PlayerCmd, dt: f64) { self.him.hand(g, cmd, dt); (self.mar)(g, cmd); }
    fn over(&self) -> bool { self.him.over() }
    fn lens(&mut self) -> [f64; 2] { self.him.lens() }
}
fn as_he_is(_: &Game, _: &mut PlayerCmd) {}
/// the squad's key is deaf: its press is dropped
fn deaf(g: &Game, cmd: &mut PlayerCmd) { let k = &g.k.orders.quick_key; cmd.pressed.retain(|x| x != k); cmd.keys.retain(|x| x != k); }
/// from the third step on nothing he does reaches the game
fn nothing_from_the_third(g: &Game, cmd: &mut PlayerCmd) {
    if g.lesson.i < 2.0 { return; }
    cmd.keys.clear();
    cmd.pressed.clear();
    (cmd.buttons, cmd.clicked) = (0.0, 0.0);
}

/// the middle of the wall nearest a man and more than a jump from him, a metre or more in from its face along its length
fn into_a_wall(g: &mut Game) {
    let man = g.guards.iter().copied().find(|&x| g.team(x).ours() && !game::condition::down(g, g.gd[x].ch)).expect("a man of the team");
    let at = g.pos_of(man);
    let far = |q: [f64; 3]| game::v3::dist_xz(q, at);
    let walls = g.col.boxes.iter().map(|&b| &g.tab.boxes[b]).filter(|b| b.h[1] > 0.8 && b.c[1] < 1.5 && far(b.c) > 2.0).map(|b| b.c);
    let c = walls.min_by(|a, b| far(*a).total_cmp(&far(*b))).expect("a wall");
    let body = g.gd[man].ch;
    (g.ch[body].pos[0], g.ch[body].pos[2]) = (c[0], c[2]);
}

/// the run, as a reader reads it
fn words_of_the_run(g: &Game, end: Option<End>, t: f64, orders: u32, o: &Owes) -> String {
    let mates: Vec<_> = g.guards.iter().filter(|&&x| g.team(x).ours()).collect();
    let up = mates.iter().filter(|&&&x| !game::condition::down(g, g.gd[x].ch)).count();
    let down = g.guards.iter().filter(|&&x| !g.team(x).ours() && game::condition::down(g, g.gd[x].ch)).count();
    let sam = if game::condition::down(g, g.pl[g.player].ch) { "Sam down" } else { "Sam up" };
    let end = end.map_or("no end", |e| e.as_str());
    let medal = game::par::par_result_now(g).map_or(String::new(), |r| if r.medal == game::par::Medal::None { String::from("no medal · ") } else { format!("{} · ", r.medal.as_str()) });
    format!("{} {:.1} s · {} · {} of {} up · {} down · {} orders · {}{}", end, t, sam, up, mates.len(), down, orders, medal, o.line())
}

/// THE HANDS-OFF PLAYER PLAYS THE MISSION TO ITS END, AND IT IS THE RUN IT WAS, with Sam under god mode. (a) what a run owes (`owes`), every tick. (b) It ends, and not by its cap: a run that does is in the pins' words, and said.
/// (c) Its line of words and its word a second against the pins.
/// BREAKS: `deaf` the squad's key is deaf · `wall` a man is put in a wall at 5 s.
pub fn played_row(name: &str, id: &str, pins: &Pins, f: Flags) {
    let t = Instant::now();
    let mut p = stand(&mission(id), SEED, 1);
    if let Some(why) = &p.refused { panic!("{} is refused: {}", id, why); }
    // (Sam cannot go down, the page's own switch: or most runs stop with him, in a few seconds, and hold nothing of the floor)
    p.g.god_mode = true;
    let mut h = HandsOff::new(p.g.seats[0], Plan { seed: SEED, cap: CAP, ..Default::default() });
    let (mut o, mut track) = (Owes::new(), Track::default());
    for i in 0..TICKS {
        if h.over() { break; }
        if f.breaks("deaf") { p.played_by(&mut Marred { him: &mut h, mar: deaf }); } else { p.played_by(&mut Marred { him: &mut h, mar: as_he_is }); }
        if f.breaks("wall") && i == WALL_AT { into_a_wall(&mut p.g); }
        o.tick(&mut p.g);
        if i % 60 == 59 { track.second(&p.g); }
    }
    let line = words_of_the_run(&p.g, h.end(), h.t(), h.given(), &o);
    if f.pin {
        pin_of(name, &track, &p.words(), Some(&line), false);
        return say(name, t.elapsed().as_secs_f64(), &[], "");
    }
    let mut fail: Vec<String> = o.faults.iter().map(|x| format!("{} {} at {:.1} s, {} times: {}", x.who, x.kind, x.t, x.times, x.words)).collect();
    if h.end().is_none() { fail.push(format!("he is still playing: {}", line)); }
    let (moved, note) = held(name, &track, &p.words(), Some(&line), pins, f);
    fail.extend(moved);
    say(name, t.elapsed().as_secs_f64(), &fail, &if note.is_empty() { line } else { format!("{} · {}", line, note) });
}

/// THE TUTORIAL IS PLAYED BY A MAN AT THE KEYS: every step goes by on its deed, the run ends, and it owes what a run owes.
/// BREAK: `third` nothing he does reaches the game from the third step on.
pub fn lesson_row(name: &str, id: &str, pace: f64, f: Flags) {
    let t = Instant::now();
    let mut p = stand(&mission(id), SEED, 1);
    let tutorial = tutorials().into_iter().find(|t| t.id == id).expect("the tutorial");
    let of = game::lesson::lesson_data(&p.g).expect("the mission has its lesson").steps.len();
    let mut l = Lesson::new(p.g.seats[0], tutorial, pace);
    let mut o = Owes::new();
    let mar = if f.breaks("third") { nothing_from_the_third } else { as_he_is };
    let done = owes::played_out(&mut p, &mut Marred { him: &mut l, mar }, LESSON_SECS, &mut o);
    let out = l.out.borrow().clone();
    let mut fail: Vec<String> = o.faults.iter().map(|x| format!("{} {} at {:.1} s, {} times: {}", x.who, x.kind, x.t, x.times, x.words)).collect();
    if !done || !out.ended || out.steps.len() != of { fail.push(format!("{} of {} steps went by on their deeds: {}", out.steps.len(), of, l.why().unwrap_or_else(|| String::from("he is still playing")))); }
    say(name, t.elapsed().as_secs_f64(), &fail, &format!("{} steps · {:.1} s · {}", of, l.me.t(), o.line()));
}
