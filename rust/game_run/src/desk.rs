//! THE DESK: what the cards before a game show and press, with no game on the floor. One blank game, done the same commands a start is done, so a man is staged with what his card shows. A
//! page asks by the list below (`asks!`, whose caller on the page is made from it by `bun run tools/draw/gen.ts`) and is answered with a view.
//!
//! The bytes of an ask: its number (u32), then its fields in order, as a command's. A PRESS changes the desk and answers `Pressed`; every other ask changes nothing.
use crate::command::{Bytes, Field};
use crate::kept;
use game::game::Game;
use game::io::{Io, W};
use game::kit::{self, CardFor, KitPreset, MemberKit, TeamFor};
use game::loadout::entry_of;
use game::query::Query;
use game::scene_scenarios::IssueData;
use game::view::{CardView, KeptText, Pressed, TeamView};
use game_start::json::{parse, Json};
use game_start::read::FromJson;

pub struct Desk {
    pub g: Box<Game>,
    /// the copied card
    clip: Option<MemberKit>,
    /// what the mission the cards are for makes of them
    on: CardFor,
    /// the mission the cards are for (how many come is the cards' own: `kit_team`)
    mission: Option<TeamFor>,
}

impl Default for Desk { fn default() -> Self { Desk::new() } }
impl Desk {
    pub fn new() -> Desk { Desk { g: Box::new(Game::blank()), clip: None, on: CardFor::default(), mission: None } }

    /// a press as it went: what the cards are kept as now goes with it
    fn pressed(&mut self, ok: bool, why: String, passed: String, changed: Vec<String>) -> Pressed {
        Pressed { ok, why, passed, changed, kept: vec![KeptText { key: String::from(kept::KIT), text: kept::write(&mut self.g) }] }
    }

    /// The cards are for this mission, played by this link. The same mission asked again with another link has its hand moved, as the fog lever moves it; another mission is a screen
    /// closed and one opened. A mission that is none is no mission.
    fn go(&mut self, mission: &str, link: &str, issue: &str, walls: u32) {
        let Some(entry) = entry_of(mission) else { return self.leave() };
        let again = self.mission.as_ref().is_some_and(|m| m.entry.id == entry.id);
        if !again { kit::kit_mission(&mut self.g, None); }
        let words = Query::parse(link.split_once('?').map_or(link, |(_, q)| q));
        let f = TeamFor::new(&self.g, entry, words, issue_block(issue), walls as f64);
        if !again {
            let size = kit::team_opens_on(&f.entry, kit::kit_team(&mut self.g, None));
            kit::kit_team(&mut self.g, Some(size));
        }
        let hand = kit::team_hand(&self.g, &f);
        kit::kit_mission(&mut self.g, hand);
        self.on = kit::team_card_for(&self.g, &f);
        self.mission = Some(f);
    }

    /// No mission: your own cards stand again.
    fn leave(&mut self) {
        kit::kit_mission(&mut self.g, None);
        self.mission = None;
        self.on = CardFor::default();
    }

    /// How many come, or why not.
    fn size(&mut self, n: u32) -> String { kit::team_size(&mut self.g, self.mission.as_ref(), n as f64) }

    /// A named kit put on, or why not.
    fn named(&mut self, id: &str) -> Option<String> { kit::team_named(&mut self.g, id) }

    /// The team as it would go, each man's worn card written as a kept one.
    fn team(&mut self) -> TeamView {
        let mut t = kit::team_view(&mut self.g, self.mission.as_ref(), self.clip.as_ref());
        let worn = kit::team_worn(&mut self.g, self.mission.as_ref());
        for (man, w) in t.men.iter_mut().zip(worn) {
            man.worn = w.as_ref().map_or(String::new(), kept::written);
        }
        t
    }
}

/// A level file's `scenarios.issue` as its text (empty, or no block with cards: none).
fn issue_block(text: &str) -> Option<IssueData> {
    let j = parse(text).ok()?;
    if !matches!(j.get("members"), Json::Obj(..)) { return None; }
    Some(IssueData::from_json(&j, "issue"))
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
    /// the mission the cards are for, the link it would be played by, the kit its level issues (the level file's own block; empty: none) and the wall charges it hands out
    6 For { mission: &str, link: &str, issue: &str, walls: u32 } -> Pressed = |d, mission, link, issue, walls| { d.go(mission, link, issue, walls); d.pressed(true, String::new(), String::new(), Vec::new()) };
    /// no mission: your own cards stand again
    7 Done {} -> Pressed = |d| { d.leave(); d.pressed(true, String::new(), String::new(), Vec::new()) };
    /// the team as it would go
    8 Team {} -> TeamView = |d| d.team();
    /// how many come
    9 Size { n: u32 } -> Pressed = |d, n| { let why = d.size(n); d.pressed(why.is_empty(), why, String::new(), Vec::new()) };
    /// a named kit on all five cards (going alone: on yours)
    10 Named { id: &str } -> Pressed = |d, id| { let why = d.named(id); d.pressed(why.is_none(), why.unwrap_or_default(), String::new(), Vec::new()) };
}
