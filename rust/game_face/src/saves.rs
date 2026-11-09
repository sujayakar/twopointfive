//! A GAME KEPT: its own value (a clone of the whole `Game`, a closure in flight with it) and its block of rust/sim's. It lives as long as this module, and a game lost to a panic leaves
//! it whole. The save controller (game::savestate::control) says when and under which label; `face_step`'s flag SAVES is its turn.
//!
//!   face_saves(h)                      this run's saves as a card lays them out, at face_out: the mission's clock (f64), would one by hand be made now (u32), the key of the one to go back to (text), how many,
//!                                      then each: its key and label (text), 1 by hand and 2 regretted (u32), the clock it was made at and which came after which (f64)
//!   face_saves_told(h) -> n            what the save controller tells the player, at face_out: its tag while it is up, then a line for each thing it has said since it was last asked (0: nothing of either)
//!   face_recover(h, newest) -> h | 0   a game lost to a panic stands again from the best kept of it, or from its run's first step (`face_recover_what(h)`: from what)
use game::savestate::control::{self, Kept, START};
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;

type Game = game::game::Game;

thread_local! {
    /// which stand each game that keeps games is of: a game lost to a panic can't say
    static STOOD_OF: RefCell<BTreeMap<u32, u32>> = const { RefCell::new(BTreeMap::new()) };
    /// the serials dealt in this module: what a lost game told its host of after the save it stands again from is not dealt twice
    static DEALT: Cell<u32> = const { Cell::new(0) };
}
/// a step was taken
pub fn dealt(g: &Game) { DEALT.with(|d| d.set(d.get().max(g.serial_next))); }

/// this game is the one held now, as it stands in it (a moment of the game, which deals no serial twice either: it is the run again)
pub fn hold(x: &mut super::Held, g: Box<Game>) {
    control::stand(&mut x.g, g);
    x.cmds = None;
    x.drawn.cut();
}
/// game `h` is gone: what the control keeps of its stand stays, for a copy of it that stands again
pub fn shelf_off(h: u32) { STOOD_OF.with(|s| s.borrow_mut().remove(&h)); }

/// AFTER A STEP, for a host that keeps games: what the step asked of the save controller, and the controller's own turn (game::savestate::control). One put back is whole again there, and nothing is drawn through it.
pub fn saves(h: u32, x: &mut super::Held, dt: f64) {
    STOOD_OF.with(|s| s.borrow_mut().insert(h, control::stood(&mut x.g)));
    let asked = control::asked(&x.g.out);
    let told = control::after_step(&mut x.g, dt, asked);
    if told.back { x.cmds = None; }
    if told.cut { x.drawn.cut(); }
    if let Some(words) = told.said { x.said.push(words); }
}
/// …a step taken with the controller off (a tape walked): only the run's start and a restart the step asked for
pub fn restart_turn(h: u32, x: &mut super::Held) {
    STOOD_OF.with(|s| s.borrow_mut().insert(h, control::stood(&mut x.g)));
    if control::restart_turn(&mut x.g) { x.cmds = None; x.drawn.cut(); }
}
/// what is kept of game `h`, the best to go back to first
fn kept_of(h: u32) -> Vec<(String, Kept)> { STOOD_OF.with(|s| s.borrow().get(&h).copied()).map_or(Vec::new(), control::kept_of) }
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
    let mut g = e.image.laid(e.image.game.sim);
    g.serial_next = g.serial_next.max(DEALT.with(|d| d.get()));
    game::scene_collision::unstand(&mut g);   // (whatever rows its copy says it wrote were another moment's)
    let n = super::put(super::Held::new(g));
    STOOD_OF.with(|s| { let mut s = s.borrow_mut(); if let Some(of) = s.remove(&h) { s.insert(n, of); } });
    super::with(n, |x| {
        // (the saves made after the one that stands are still kept: the shelf names them all)
        for (key, e) in of.iter().filter(|(key, _)| key != START) {
            let Some(mut m) = e.image.game.saves.shelf.m.get(key.clone()).cloned() else { continue };
            if e.regret && m.info.regret.is_none() { m.info.regret = Some(String::from("the floor came down straight after")); }
            x.g.saves.shelf.m.set(key.clone(), m);
        }
        if key != START { let was = control::abandoned(&x.g); let told = control::put_back(&mut x.g, &key, was); x.said.push(told); }
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
        b.extend_from_slice(&(control::hand_no(g).is_none() as u32).to_le_bytes());
        let offer = game::savestate::checkpoints_newest(g, Some(true), Some(g.mission.stats.elapsed)).filter(|i| game::savestate::checkpoints_is_this_run(g, i));
        text(&mut b, offer.as_ref().map_or("", |i| i.key.as_str()));
        let rows = control::timeline(g);
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
