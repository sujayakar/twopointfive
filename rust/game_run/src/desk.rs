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
use game::query::{after_mark, Query};
use game::scene_scenarios::IssueData;
use game::start_list::{self, Sitting};
use game::view::{CardView, KeptText, Pressed, SheetView, ShortView, StartView, TeamView};
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
    /// what it was asked for by, to ask again when the list's settings move: the link handed (empty: the desk's own), the level's block, the wall charges
    asked: Option<(String, String, u32)>,
    /// the mission list's own: the address, what is picked on it, what the machine keeps of the levers
    list: Sitting,
}

impl Default for Desk { fn default() -> Self { Desk::new() } }
impl Desk {
    /// The host's commands, done as a start is done them: what this machine keeps and what the address says.
    pub fn machine(&mut self, b: &[u8]) -> Result<(), String> {
        crate::command::apply(&mut self.g, b)?;
        self.list.said(crate::command::levers_said(b));
        Ok(())
    }

    pub fn new() -> Desk { Desk { g: Box::new(Game::blank()), clip: None, on: CardFor::default(), mission: None, asked: None, list: Sitting::default() } }

    /// a press as it went: what the cards are kept as now goes with it
    fn pressed(&mut self, ok: bool, why: String, passed: String, changed: Vec<String>) -> Pressed {
        Pressed { ok, why, passed, changed, kept: vec![KeptText { key: String::from(kept::KIT), text: kept::write(&mut self.g) }] }
    }

    /// a press that changes no kept card: what the cards are for, how many come, what is copied. Nothing is written for a machine that has changed nothing
    fn left(ok: bool, why: String) -> Pressed { Pressed { ok, why, passed: String::new(), changed: Vec::new(), kept: Vec::new() } }

    /// The mission the cards are for, asked for again where its link is the list's own: a lever moves what a mission hands out.
    fn again(&mut self) {
        let (Some(f), Some((link, issue, walls))) = (&self.mission, self.asked.clone()) else { return };
        let id = f.entry.id.clone();
        if link.is_empty() { self.go(&id, &link, &issue, walls); }
    }

    /// The visitor's address is this from now on.
    fn start(&mut self, search: &str) -> StartView {
        let v = self.list.start(&mut self.g, search);
        self.again();
        v
    }

    /// One choice of one row of the sheet pressed.
    fn set(&mut self, mission: &str, key: &str, to: &str) -> Pressed {
        let Some(moved) = self.list.press(&mut self.g, mission, key, to) else { return Desk::left(false, String::new()) };
        self.again();
        Pressed { kept: moved, ..Desk::left(true, String::new()) }
    }

    /// The cards are for this mission, played by this link. The same mission asked again with another link has its hand moved, as the fog lever moves it; another mission is a screen
    /// closed and one opened. A mission that is none is no mission.
    fn go(&mut self, mission: &str, link: &str, issue: &str, walls: u32) {
        let Some(entry) = entry_of(mission) else { return self.leave() };
        let again = self.mission.as_ref().is_some_and(|m| m.entry.id == entry.id);
        if !again { kit::kit_mission(&mut self.g, None); }
        self.asked = Some((String::from(link), String::from(issue), walls));
        let own = if link.is_empty() { self.list.fog_link(&mut self.g, mission) } else { String::from(link) };
        let words = Query::parse(after_mark(&own));
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
        self.asked = None;
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
        if let (Some(f), Some((link, ..))) = (&self.mission, &self.asked) {
            t.link = self.list.deploy_link(&mut self.g, &f.entry.id, link, t.size);
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
    4 Copy { who: u32 } -> Pressed = |d, who| { let c = kit::kit_shown(&mut d.g, who as f64); d.clip = Some(c); Desk::left(true, String::new()) };
    /// the copied card put on him
    5 Paste { who: u32 } -> Pressed = |d, who| match d.clip.clone() {
        Some(c) => { let r = kit::kit_paste(&mut d.g, who as f64, &c); let moved = r.changed.iter().map(|f| f.as_str().to_string()).collect(); d.pressed(r.ok, r.why, String::new(), moved) }
        None => d.pressed(false, String::new(), String::new(), Vec::new()),
    };
    /// the mission the cards are for, the link it would be played by, the kit its level issues (the level file's own block; empty: none) and the wall charges it hands out
    6 For { mission: &str, link: &str, issue: &str, walls: u32 } -> Pressed = |d, mission, link, issue, walls| { d.go(mission, link, issue, walls); Desk::left(true, String::new()) };
    /// no mission: your own cards stand again
    7 Done {} -> Pressed = |d| { d.leave(); Desk::left(true, String::new()) };
    /// the team as it would go
    8 Team {} -> TeamView = |d| d.team();
    /// how many come
    9 Size { n: u32 } -> Pressed = |d, n| { let why = d.size(n); Desk::left(why.is_empty(), why) };
    /// a named kit on all five cards (going alone: on yours)
    10 Named { id: &str } -> Pressed = |d, id| { let why = d.named(id); d.pressed(why.is_none(), why.unwrap_or_default(), String::new(), Vec::new()) };
    /// the visitor's address is this from now on: the rows of the list in the page's order, what it names
    11 Start { search: &str } -> StartView = |d, search| d.start(search);
    /// a mission's rows of settings, in the sheet's order
    12 Sheet { mission: &str } -> SheetView = |d, mission| d.list.sheet(&mut d.g, mission);
    /// one choice of one row pressed: refused where the row is not drawn or not live
    13 Set { mission: &str, key: &str, to: &str } -> Pressed = |d, mission, key, to| d.set(mission, key, to);
    /// the id of the mission an address starts (empty: none)
    14 Mission { search: &str } -> String = |_, search| start_list::mission_of_address(search);
    /// the link a row of the short list loads, at its own difficulty (-1), standard (0) or hard (1); empty: no such row
    15 Short { id: &str, search: &str, hard: i32 } -> String = |_, id, search, hard| start_list::short_link(id, search, hard);
    /// on a page a row of the short list started: that row, the row after it and the page load that starts it
    16 Next { search: &str } -> ShortView = |d, search| d.list.short_view(&d.g, search);
    /// the address with everything that started something taken off
    17 Leave { search: &str } -> String = |_, search| game::scene_scenario_list::pick_quit_search(search);
}
