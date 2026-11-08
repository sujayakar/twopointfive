//! THE DESK: what the cards before a game show and press, with no game on the floor. One blank game, done the same commands a start is done, so a man is staged with what his card shows. A
//! page asks by the list below (`asks!`, whose caller on the page is made from it by `bun run tools/draw/gen.ts`) and is answered with a view.
//!
//! The bytes of an ask: its number (u32), then its fields in order, as a command's. A PRESS changes the desk and answers `Pressed`; every other ask changes nothing.
use crate::command::{Bytes, Field};
use crate::kept;
use game::game::Game;
use game::io::{Io, W};
use game::kit::{self, CardFor, KitPreset, MemberKit};
use game::view::{CardView, Pressed};

pub struct Desk {
    pub g: Box<Game>,
    /// the copied card
    clip: Option<MemberKit>,
    /// what the mission the cards are for makes of them
    on: CardFor,
}

impl Default for Desk { fn default() -> Self { Desk::new() } }
impl Desk {
    pub fn new() -> Desk { Desk { g: Box::new(Game::blank()), clip: None, on: CardFor::default() } }

    /// a press as it went: what the cards are kept as now goes with it
    fn pressed(&mut self, ok: bool, why: String, passed: String, changed: Vec<String>) -> Pressed {
        Pressed { ok, why, passed, changed, kept: kept::write(&mut self.g) }
    }
}

macro_rules! asks {
    ($($(#[$m:meta])* $i:literal $name:ident { $($f:ident: $t:ty),* } -> $out:ty = $do:expr;)*) => {
        impl Desk {
            /// the ask in `b`, answered; a number no ask has says so
            pub fn ask(&mut self, b: &[u8]) -> Result<Vec<u8>, String> {
                let mut r = Bytes::of(b);
                match u32::read(&mut r) {
                    $($i => {
                        $(let $f = <$t as Field>::read(&mut r);)*
                        let f: fn(&mut Desk, $($t),*) -> $out = $do;
                        let mut w = W::new();
                        f(self, $($f),*).w(&mut w);
                        Ok(w.b)
                    })*
                    n => Err(format!("no ask has the number {n}")),
                }
            }
        }
    };
}

asks! {
    /// a man's card, by his place (0 the player)
    0 Card { who: u32 } -> CardView = |d, who| kit::card_view(&mut d.g, who as f64, d.clip.as_ref(), &d.on);
    /// one row of it a step on (1) or back (-1)
    1 Step { who: u32, field: &str, dir: i32 } -> Pressed = |d, who, field, dir| match kit::field_of(field) {
        Some(f) => { let r = kit::kit_try(&mut d.g, who as f64, f, dir as f64); d.pressed(r.ok, r.why, r.passed, Vec::new()) }
        None => d.pressed(false, format!("no row {field}"), String::new(), Vec::new()),
    };
    /// a gun's row at one of its picks, fitted and refused as a paste is
    2 Choose { who: u32, field: &str, value: &str } -> Pressed = |d, who, field, value| {
        let (ok, why) = kit::kit_choose(&mut d.g, who as f64, field, value);
        d.pressed(ok, why, String::new(), Vec::new())
    };
    /// the whole card as issued
    3 Reset { who: u32 } -> Pressed = |d, who| { let r = kit::kit_preset(&mut d.g, who as f64, KitPreset::Issued); d.pressed(r.ok, r.why, String::new(), Vec::new()) };
    /// his card is the copied one
    4 Copy { who: u32 } -> Pressed = |d, who| { let c = kit::kit_shown(&mut d.g, who as f64); d.clip = Some(c); d.pressed(true, String::new(), String::new(), Vec::new()) };
    /// the copied card put on him
    5 Paste { who: u32 } -> Pressed = |d, who| match d.clip.clone() {
        Some(c) => { let r = kit::kit_paste(&mut d.g, who as f64, &c); let moved = r.changed.iter().map(|f| f.as_str().to_string()).collect(); d.pressed(r.ok, r.why, String::new(), moved) }
        None => d.pressed(false, String::new(), String::new(), Vec::new()),
    };
}
