//! A GAME KEPT: its own value (a clone of the whole `Game`, a closure in flight with it) and its block of rust/sim's. It lives as long as this module, and a game lost to a panic leaves
//! it whole. The save controller (game::savestate::control) says when and under which label; `face_step`'s flag SAVES is its turn.
//!
//!   face_saves(h)                      this run's saves as a card lays them out, at face_out: the mission's clock (f64), would one by hand be made now (u32), the key of the one to go back to (text), how many,
//!                                      then each: its key and label (text), 1 by hand and 2 regretted (u32), the clock it was made at and which came after which (f64)
//!   face_saves_told(h) -> n            what the save controller tells the player, at face_out: its tag while it is up, then a line for each thing it has said since it was last asked (0: nothing of either)
//!   face_recover(h, newest) -> h | 0   a game lost to a panic stands again from the best kept of it, or from its run's first step (`face_recover_what(h)`: from what)
use game_start::moment::{whole, Moment};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

type Game = game::game::Game;
/// a game as it stands, with nothing of it left to be told
fn told(g: &Game) -> Rc<Moment> { let mut m = Moment::of(g); m.game.out.clear(); Rc::new(m) }

/// a game the save controller keeps: which came after which, its label, and whether the floor came down straight after it
#[derive(Clone)]
pub struct Shelved { kept: Rc<Moment>, at: f64, label: String, regret: bool }

thread_local! {
    /// the games the save controller keeps, by the game they are of and its key: here and not with the game, so a game lost to a panic leaves them
    static SHELF: RefCell<BTreeMap<(u32, String), Shelved>> = const { RefCell::new(BTreeMap::new()) };
}
thread_local! {
    /// the serials dealt in this module: what a lost game told its host of after the save it stands again from is not dealt twice
    static DEALT: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}
/// a step was taken
pub fn dealt(g: &Game) { DEALT.with(|d| d.set(d.get().max(g.serial_next))); }
/// the key of a run's first step
const START: &str = "start";

/// the game held is that one again
fn back(x: &mut super::Held, k: &Moment) {
    let mut g = k.laid(x.g.sim);
    g.serial_next = g.serial_next.max(x.g.serial_next);   // (what was dealt after the save is still told of: a number means one thing)
    hold(x, g);
}
/// this game is the one held now
pub fn hold(x: &mut super::Held, mut g: Box<Game>) {
    whole(&mut g);
    x.g = g;
    x.cmds = None;
    x.drawn.cut();
}
/// the save controller's shelf of a game
pub fn shelf_of(h: u32) -> Vec<(String, Shelved)> { SHELF.with(|s| s.borrow().iter().filter(|((g, _), _)| *g == h).map(|((_, key), e)| (key.clone(), e.clone())).collect()) }
/// … and the shelf is that again
pub fn shelf_put(h: u32, of: Vec<(String, Shelved)>) {
    shelf_off(h, |_| true);
    for (key, e) in of { shelve(h, key, e); }
}
fn shelve(h: u32, key: String, e: Shelved) { SHELF.with(|s| s.borrow_mut().insert((h, key), e)); }
/// the games kept for game `h` that are let go
pub fn shelf_off(h: u32, go: impl Fn(&str) -> bool) { SHELF.with(|s| s.borrow_mut().retain(|(g, key), _| *g != h || !go(key))); }

/// AFTER A STEP, for a host that keeps games: what the step asked of the save controller, and the controller's own turn. One put back is whole again here, and nothing is drawn through it.
pub fn saves(h: u32, x: &mut super::Held, dt: f64) {
    use game::out_events::OutEvent as E;
    use game::savestate::control::{self, Do};
    let asked: Vec<E> = x.g.out.iter().filter(|e| matches!(e, E::CheckpointsKeep(_) | E::CheckpointsRequest(_))).cloned().collect();
    for e in asked { match e { E::CheckpointsKeep(e) => control::keep(&mut x.g, &e.label), E::CheckpointsRequest(e) => control::request(&mut x.g, &e.why, &e.label), _ => {} } }
    if core::mem::replace(&mut x.g.restart_cut, false) { x.drawn.cut(); }
    match control::tick(&mut x.g, dt) {
        Some(Do::Keep { key, label }) => {
            game_run::checkpoint(&mut x.g, &label);
            let at = x.g.saves.shelf.m.values().map(|e| e.info.saved_at).fold(0.0, f64::max);
            shelve(h, key, Shelved { kept: told(&x.g), at, label, regret: false });
        }
        Some(Do::Back { key }) => {
            let was = control::abandoned(&x.g);
            let kept = SHELF.with(|s| s.borrow().get(&(h, key.clone())).map(|e| e.kept.clone()));
            let told = if let Some(k) = kept { back(x, &k); control::put_back(&mut x.g, &key, was) } else { control::gone(&mut x.g, &key) };
            x.said.push(told);
        }
        Some(Do::Say(words)) => x.said.push(words),
        None => {}
    }
    // (a run's saves go with the run; its first step is kept, for a game that is lost before anything else is)
    shelf_off(h, |key| key != START && !x.g.saves.shelf.m.has(String::from(key)));
    let run = x.g.mission.serial as f64;
    if SHELF.with(|s| s.borrow().get(&(h, String::from(START))).map(|e| e.at)) != Some(run) { shelve(h, String::from(START), Shelved { kept: told(&x.g), at: run, label: String::new(), regret: false }); }
    SHELF.with(|s| for ((of, key), e) in s.borrow_mut().iter_mut() { if *of == h && !e.regret { if let Some(m) = x.g.saves.shelf.m.get(key.clone()) { e.regret = m.info.regret.is_some(); } } });
}
/// what is kept of game `h`, the best to go back to first: the newest save nothing came down after, the newest at all, and the run's first step last
fn kept_of(h: u32) -> Vec<(String, Shelved)> {
    let mut of = shelf_of(h);
    of.sort_by(|a, b| (a.0 == START).cmp(&(b.0 == START)).then(a.1.regret.cmp(&b.1.regret)).then(b.1.at.total_cmp(&a.1.at)));
    of
}
/// what a game lost to a panic would stand again from: 0 nothing, 1 its run's first step, 2 a save (its label at face_out)
#[no_mangle]
pub extern "C" fn face_recover_what(h: u32) -> u32 {
    match kept_of(h).first() { None => 0, Some((key, _)) if key == START => 1, Some((_, e)) => { super::out(e.label.clone().into_bytes()); 2 } }
}
/// A GAME LOST TO A PANIC STANDS AGAIN, from the best save of it (`newest` = 0: from its run's first step), under a new number; 0 = nothing of it is kept. What was
/// kept of the lost game is the new one's.
#[no_mangle]
pub extern "C" fn face_recover(h: u32, newest: u32) -> u32 {
    super::hook();
    let of = kept_of(h);
    let Some((key, e)) = of.iter().find(|(key, _)| newest != 0 || key == START).cloned() else { return 0 };
    // (a game that still stands under the lost number would share the slot: it is let go, and what is kept of it is not)
    super::GAMES.with(|v| if let Some(x) = v.borrow_mut().get_mut((h as usize).wrapping_sub(1)) { *x = None; });
    let mut g = e.kept.laid(e.kept.game.sim);
    g.serial_next = g.serial_next.max(DEALT.with(|d| d.get()));
    game::scene_collision::unstand(&mut g);   // (whatever rows its copy says it wrote were another moment's)
    let n = super::put(super::Held::new(g));
    SHELF.with(|s| { let mut s = s.borrow_mut(); for (key, _) in &of { if let Some(e) = s.remove(&(h, key.clone())) { s.insert((n, key.clone()), e); } } });
    super::with(n, |x| {
        // (the saves made after the one that stands are still kept: the shelf names them all)
        for (key, e) in of.iter().filter(|(key, _)| key != START) {
            let Some(mut m) = e.kept.game.saves.shelf.m.get(key.clone()).cloned() else { continue };
            if e.regret && m.info.regret.is_none() { m.info.regret = Some(String::from("the floor came down straight after")); }
            x.g.saves.shelf.m.set(key.clone(), m);
        }
        if key != START { let was = game::savestate::control::abandoned(&x.g); let told = game::savestate::control::put_back(&mut x.g, &key, was); x.said.push(told); }
        x.drawn.cut();
    });
    super::out(e.label.into_bytes());
    n
}
#[no_mangle]
pub extern "C" fn face_saves_told(h: u32) -> usize {
    let t = super::with(h, |x| {
        let (f, secs) = (x.g.saves.flash.as_ref(), x.g.k.ckpt.flash_secs);
        let mut t = String::from(f.filter(|f| x.g.time >= f.t && x.g.time - f.t < secs).map_or("", |f| f.text.as_str()));
        for s in x.said.drain(..) { t.push('\n'); t.push_str(&s); }
        t
    });
    let n = t.len();
    super::out(t.into_bytes());
    n
}
#[no_mangle]
pub extern "C" fn face_saves(h: u32) {
    let b = super::with_ref(h, |x| {
        let g = &x.g;
        let mut b = Vec::new();
        let text = |b: &mut Vec<u8>, s: &str| { b.extend_from_slice(&(s.len() as u32).to_le_bytes()); b.extend_from_slice(s.as_bytes()); };
        b.extend_from_slice(&g.mission.stats.elapsed.to_le_bytes());
        b.extend_from_slice(&(game::savestate::control::hand_no(g).is_none() as u32).to_le_bytes());
        let offer = game::savestate::checkpoints_newest(g, Some(true), Some(g.mission.stats.elapsed)).filter(|i| game::savestate::checkpoints_is_this_run(g, i));
        text(&mut b, offer.as_ref().map_or("", |i| i.key.as_str()));
        let rows = game::savestate::control::timeline(g);
        b.extend_from_slice(&(rows.len() as u32).to_le_bytes());
        for r in rows {
            text(&mut b, &r.key);
            text(&mut b, &r.label);
            b.extend_from_slice(&((r.slot == game::savestate::SlotInfoSlot::Manual) as u32 | (r.regret.is_some() as u32) << 1).to_le_bytes());
            b.extend_from_slice(&r.mission_t.to_le_bytes());
            b.extend_from_slice(&r.saved_at.to_le_bytes());
        }
        b
    });
    super::out(b);
}
