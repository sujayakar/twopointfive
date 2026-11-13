//! A GAME KEPT: its own value (a clone of the whole `Game`, a closure in flight with it). It lives as long as this module, and a game lost to a panic leaves
//! it whole. The save controller (game::savestate::control) says when and under which label; `step`'s flag SAVES is its turn.
use crate::Held;
use game::savestate::control::{self, Kept, START};
use game::view::{Recovered, SaveRow, SavesTold, SavesView};
use game_start::host::calls::GameNo;
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
pub fn hold(x: &mut Held, g: Box<Game>) {
    control::stand(&mut x.g, g);
    x.cmds = None;
    x.drawn.cut();
}
/// game `h` is gone: what the control keeps of its stand stays, for a copy of it that stands again
pub fn shelf_off(h: u32) { STOOD_OF.with(|s| s.borrow_mut().remove(&h)); }

/// AFTER A STEP, for a host that keeps games: what the step asked of the save controller, and the controller's own turn (game::savestate::control). One put back is whole again there, and nothing is drawn through it.
pub fn saves_turn(x: &mut Held, dt: f64) {
    STOOD_OF.with(|s| s.borrow_mut().insert(x.no, control::stood(&mut x.g)));
    let asked = control::asked(&x.g.out);
    let told = control::after_step(&mut x.g, dt, asked);
    if told.back { x.cmds = None; }
    if told.cut { x.drawn.cut(); }
    if let Some(words) = told.said { x.said.push(words); }
}
/// …a step taken with the controller off (a tape walked): only the run's start and a restart the step asked for
pub fn restart_turn(x: &mut Held) {
    STOOD_OF.with(|s| s.borrow_mut().insert(x.no, control::stood(&mut x.g)));
    if control::restart_turn(&mut x.g) { x.cmds = None; x.drawn.cut(); }
}
/// what is kept of game `h`, the best to go back to first
fn kept_of(h: u32) -> Vec<(String, Kept)> { STOOD_OF.with(|s| s.borrow().get(&h).copied()).map_or(Vec::new(), control::kept_of) }
pub fn recover_what(h: GameNo) -> Option<String> { kept_of(h).first().map(|(key, e)| if key == START { String::new() } else { e.label.clone() }) }
/// What was kept of the lost game is the new one's.
pub fn recover(h: GameNo, newest: bool) -> Option<Recovered> {
    let of = kept_of(h);
    let (key, e) = of.iter().find(|(key, _)| newest || key == START).cloned()?;
    // (a game that still stands under the lost number is let go, and what is kept of it is not)
    super::let_go(h);
    let mut g = e.image.laid();
    g.serial_next = g.serial_next.max(DEALT.with(|d| d.get()));
    let n = super::put(Held::new(g));
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
    Some(Recovered { game: n, label: e.label })
}
pub fn saves_told(x: &mut Held) -> SavesTold {
    let (f, secs) = (x.g.saves.flash.as_ref(), x.g.k.ckpt.flash_secs);
    SavesTold { tag: f.filter(|f| x.g.time >= f.t && x.g.time - f.t < secs).map_or(String::new(), |f| f.text.clone()), lines: std::mem::take(&mut x.said) }
}
pub fn saves(x: &Held) -> SavesView {
    let g = &x.g;
    let elapsed = g.mission.stats.elapsed;
    let offer = game::savestate::checkpoints_newest(g, Some(true), Some(elapsed)).filter(|i| game::savestate::checkpoints_is_this_run(g, i));
    SavesView {
        elapsed, hand: control::hand_no(g).is_none(), offer: offer.map_or(String::new(), |i| i.key),
        rows: control::timeline(g).into_iter().map(|r| SaveRow { key: r.key, label: r.label, slot: r.slot, regret: r.regret.is_some(), mission_t: r.mission_t, saved_at: r.saved_at }).collect(),
    }
}
