//! A man's mind is written through one door.
//!
//! How aware a man is (`Guard.awareness`) and what he is doing about it (`Guard.state`) were assigned in place by
//! every module that had a reason to: about 126 writes of the one and 80 of the other. Each is now one of the calls
//! below, which writes the same value at the same place and keeps WHO moved him and WHEN, so that "why is he alert"
//! has an answer (`why`, on the debugger's line for a man beside `control::control_line`).
//!
//! The calls say what the writer means:
//!
//!   raise(g, man, floor, by)   he is at least this aware (told, startled, a sound): never lowers him
//!   cap(g, man, ceiling, by)   he is at most this aware (stood down, lost his man): never raises him
//!   set(g, man, to, by)        exactly this (his own senses' sum for the tick, a reset)
//!   set_state(g, man, to, by)  patrol, suspicious, alert, search, dead
//!
//! `by` is the writer's module. The body files are moved to these calls by a program
//! (tools/blitz/sweeps/mind-door.py), which can be run again on a later tree. A new rule calls them itself.
//!
//! What this is a step towards: `raise` and `cap` are the only two things most writers ever meant, so the next step
//! is that they hand over a percept ("told at this place by that man") and this module alone decides what it does
//! to him. Nothing here changes play.
use crate::game::{Game, GuardState};
use crate::ids::GuardId;
use crate::m;
use alloc::format;
use alloc::string::String;

/// One write that moved him: who, when (the game's clock) and to what.
#[derive(Clone, Copy, Debug, Default)]
pub struct Mark {
    pub by: &'static str,
    pub t: f64,
    pub to: f64,
}

/// The last writers that moved a man's mind. A write that left him where he was is not kept.
#[derive(Clone, Copy, Debug, Default)]
pub struct Note {
    /// the last write that made him MORE aware
    pub up: Mark,
    /// the last write that made him LESS aware
    pub down: Mark,
    /// the last write that changed his state, and the state it took him out of
    pub state: Mark,
    pub state_from: Option<GuardState>,
}

fn write(g: &mut Game, gd: GuardId, was: f64, now: f64, by: &'static str) {
    g.gd[gd].awareness = now;
    if now == was { return; }
    let mark = Mark { by, t: g.time, to: now };
    let n = g.ctl.mind.at(gd);
    if now > was { n.up = mark; } else if now < was { n.down = mark; }
}

/// He is at least this aware. Never lowers him.
pub fn raise(g: &mut Game, gd: GuardId, floor: f64, by: &'static str) {
    let was = g.gd[gd].awareness;
    write(g, gd, was, m::max(was, floor), by);
}

/// He is at most this aware. Never raises him.
pub fn cap(g: &mut Game, gd: GuardId, ceiling: f64, by: &'static str) {
    let was = g.gd[gd].awareness;
    write(g, gd, was, m::min(was, ceiling), by);
}

/// He is exactly this aware.
pub fn set(g: &mut Game, gd: GuardId, to: f64, by: &'static str) {
    let was = g.gd[gd].awareness;
    write(g, gd, was, to, by);
}

/// His state. (Dead is his life and not his mind; it is written here until life has a record of its own.)
pub fn set_state(g: &mut Game, gd: GuardId, to: GuardState, by: &'static str) {
    let was = g.gd[gd].state;
    g.gd[gd].state = to;
    if was == to { return; }
    let now = g.time;
    let n = g.ctl.mind.at(gd);
    n.state = Mark { by, t: now, to: 0.0 };
    n.state_from = Some(was);
}

/// Why he is as aware as he is, in one line: his awareness and state, and who last raised him, lowered him and
/// changed his state, with how long ago.
pub fn why(g: &Game, gd: GuardId) -> String {
    let man = &g.gd[gd];
    let n = g.ctl.mind.get(gd).copied().unwrap_or_default();
    let mut s = format!("{}: {} at {:.2}", man.callsign, man.state.as_str(), man.awareness);
    let ago = |t: f64| g.time - t;
    if let Some(from) = n.state_from { s.push_str(&format!(" · {} since {:.1} s ago by {} (was {})", man.state.as_str(), ago(n.state.t), n.state.by, from.as_str())); }
    if !n.up.by.is_empty() { s.push_str(&format!(" · raised to {:.2} by {} {:.1} s ago", n.up.to, n.up.by, ago(n.up.t))); }
    if !n.down.by.is_empty() { s.push_str(&format!(" · lowered to {:.2} by {} {:.1} s ago", n.down.to, n.down.by, ago(n.down.t))); }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one_man() -> (alloc::boxed::Box<Game>, GuardId) {
        let mut g = alloc::boxed::Box::new(Game::blank());
        let gd = g.gd.push(crate::game::Guard::blank());
        (g, gd)
    }

    #[test]
    fn raise_never_lowers_and_cap_never_raises() {
        let (mut g, gd) = one_man();
        set(&mut g, gd, 0.5, "test");
        raise(&mut g, gd, 0.3, "low");
        assert_eq!(g.gd[gd].awareness, 0.5);
        cap(&mut g, gd, 0.8, "high");
        assert_eq!(g.gd[gd].awareness, 0.5);
        raise(&mut g, gd, 0.9, "told");
        cap(&mut g, gd, 0.6, "stood down");
        assert_eq!(g.gd[gd].awareness, 0.6);
    }

    #[test]
    fn the_note_keeps_only_a_write_that_moved_him() {
        let (mut g, gd) = one_man();
        g.time = 4.0;
        raise(&mut g, gd, 0.9, "told");
        g.time = 6.0;
        raise(&mut g, gd, 0.4, "a sound");
        cap(&mut g, gd, 0.95, "nobody");
        let n = *g.ctl.mind.get(gd).expect("he has a note");
        assert_eq!((n.up.by, n.up.t, n.up.to), ("told", 4.0, 0.9));
        assert_eq!(n.down.by, "");
        g.time = 9.0;
        cap(&mut g, gd, 0.2, "lost him");
        let n = *g.ctl.mind.get(gd).expect("he has a note");
        assert_eq!((n.down.by, n.down.t, n.down.to), ("lost him", 9.0, 0.2));
    }

    #[test]
    fn a_change_of_state_says_who_and_what_from() {
        let (mut g, gd) = one_man();
        g.time = 2.0;
        set_state(&mut g, gd, GuardState::Alert, "guards");
        g.time = 3.0;
        set_state(&mut g, gd, GuardState::Alert, "squad");
        let n = *g.ctl.mind.get(gd).expect("he has a note");
        assert_eq!((n.state.by, n.state.t, n.state_from), ("guards", 2.0, Some(GuardState::Patrol)));
        g.time = 5.0;
        let line = why(&g, gd);
        assert!(line.contains("alert since 3.0 s ago by guards (was patrol)"), "{}", line);
    }
}
