//! src/game/game.ts lines 974 to 1271
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A line for the player's log; the log keeps five. (game.ts Game.msg)
#[doc(hidden)] pub fn game_msg__body(g: &mut crate::game::Game, text: &str) {
    let (t, serial) = (g.time, g.serial());
    g.messages.push(crate::game::GameMessagesItem { text: String::from(text), t, serial });
    if g.messages.len() > 5 { g.messages.remove(0); }
    crate::cues::cues_raw(g, text);
}

/// A man says something. (game.ts Game.say)
#[doc(hidden)] pub fn game_say__body(g: &mut crate::game::Game, g_2: GuardId, text: &str, radio: Option<bool>, force: Option<bool>, row: Option<bool>) {
    crate::guards::say(g, g_2, text, radio, force, row);
}

/// A utility or sandbox confirmation: the showcase's own plumbing stays out of the log. (game.ts Game.note)
#[doc(hidden)] pub fn game_note__body(g: &mut crate::game::Game, text: &str) {
    if !g.quiet_utility { game_msg(g, text); }
}


#[doc(hidden)] pub fn game_msg(g: &mut crate::game::Game, text: &str) {
    use crate::io::Io;
    if g.taps & (1u64 << 0) == 0 { return game_msg__body(g, text); }
    let mut o = crate::io::W::new(); alloc::string::String::from(text).w(&mut o);
    game_msg__body(g, text);
    crate::out_events::push_tap(g, 0, &o.b, &[]);
}


#[doc(hidden)] pub fn game_say(g: &mut crate::game::Game, g_2: GuardId, text: &str, radio: Option<bool>, force: Option<bool>, row: Option<bool>) {
    use crate::io::Io;
    if g.taps & (1u64 << 1) == 0 { return game_say__body(g, g_2, text, radio, force, row); }
    let mut o = crate::io::W::new(); g_2.w(&mut o); alloc::string::String::from(text).w(&mut o); radio.w(&mut o); force.w(&mut o); row.w(&mut o);
    game_say__body(g, g_2, text, radio, force, row);
    crate::out_events::push_tap(g, 1, &o.b, &[]);
}


#[doc(hidden)] pub fn game_note(g: &mut crate::game::Game, text: &str) {
    use crate::io::Io;
    if g.taps & (1u64 << 2) == 0 { return game_note__body(g, text); }
    let mut o = crate::io::W::new(); alloc::string::String::from(text).w(&mut o);
    game_note__body(g, text);
    crate::out_events::push_tap(g, 2, &o.b, &[]);
}

