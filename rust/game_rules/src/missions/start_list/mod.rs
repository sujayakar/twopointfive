//! THE MISSION LIST BEFORE A GAME: its rows in the page's order, the sheet of one mission, a choice pressed on it, and the links they all make. What it holds of its own is `Sitting`: the
//! visitor's address, the hour and weather picked on it, the way in picked per mission, and what this machine keeps of the levers. The difficulty, the levers and the quiet picks are the game's
//! own tables.
use crate::difficulty::{self, Difficulty, DifficultyNow, DifficultyWordsLevel, KitRung, LeverKey};
use crate::game::Game;
use crate::hours::{self, HourId};
use crate::js;
use crate::loadout::{self, entry_of, kit_as_issued, LoadoutEntry};
use crate::query::{after_mark, Query};
use crate::quietfog::{self, QuietRung};
use crate::scene_scenario_list::{self as list, all_missions, Pick, SOLO_ID, SOLO_KEY, SCENARIO_KEYS};
use crate::scenario_run;
use crate::scene_scenarios::{MissionGroup, ScenarioEntry, ScenarioEntryKind};
use crate::ui_missions::Lean;
use crate::view::{BriefView, KeptText, MissionRow, MissionsView, Setting, SettingChoice, SheetView, ShortView, StartRow, StartView};
use crate::weather_kinds::{self, WeatherKind};
use alloc::{format, string::String, vec, vec::Vec};

/// The name the level is kept under, and the levers, the quiet fog and its cameras.
const KEPT_LEVEL: &str = difficulty::LS;
const KEPT_LEVERS: &str = difficulty::LS_LEVERS;
const KEPT_FOG: &str = quietfog::LS;
const KEPT_CAMS: &str = quietfog::LS_CAMS;

/// The groups of the list, in the order the page draws them: id, title, line.
const GROUPS: [(MissionGroup, &str, &str); 6] = [
    (MissionGroup::Tutorial, "Tutorial", "The game, a mission at a time"),
    (MissionGroup::Raids, "Raids", "Small buildings"),
    (MissionGroup::Strongholds, "Strongholds", "Big buildings"),
    (MissionGroup::Clock, "On the clock", "A hostage or a par time"),
    (MissionGroup::QuietAlone, "Quiet, alone", "Light, noise and cameras"),
    (MissionGroup::QuietTwo, "Quiet, as two", "You and one partner, silent from the start"),
];

/// One row of the list: a mission of the table, or the plain solo night (no entry).
#[derive(Clone, Debug)]
pub struct Mission {
    pub id: String,
    pub title: String,
    pub entry: Option<ScenarioEntry>,
    pub squad: f64,
    pub group: MissionGroup,
    pub place: String,
    pub blurb: String,
    pub lean: Lean,
    pub untuned: bool,
}

impl Mission {
    fn level(&self) -> Option<&str> { self.entry.as_ref().and_then(|e| e.level.as_deref()).filter(|l| !l.is_empty()) }
    fn ways(&self) -> Option<&[String]> { self.entry.as_ref().and_then(|e| e.ways.as_deref()) }
    fn is_quiet(&self) -> bool { self.entry.as_ref().is_some_and(|e| e.kind == ScenarioEntryKind::Quiet) }
    fn lesson(&self) -> bool { self.entry.as_ref().is_some_and(lesson_row) }
    /// the lever is live on it with a bottom rung of its own: a quiet mission that is no lesson, and the solo night
    fn quiet_row(&self) -> bool { self.entry.as_ref().map_or(true, |e| e.kind == ScenarioEntryKind::Quiet && !lesson_row(e)) }
    /// Standard | Hard changes something on it: a squad scenario's defenders read it
    fn diff_live(&self) -> bool { self.entry.is_some() && !self.is_quiet() }
    /// a level built for snow has its own weather: no row is drawn for it and its link carries none
    fn own_weather(&self, g: &Game) -> bool { weather_kinds::snow_level(g, self.level().map(|l| l.strip_suffix(".json").unwrap_or(l))) }
    /// the hour and the weather a row is always started in
    fn row_hour(&self) -> Option<HourId> { self.entry.as_ref().filter(|e| row_own(e)).and_then(|e| e.hour) }
    fn row_weather(&self) -> Option<WeatherKind> {
        let w = self.entry.as_ref().filter(|e| row_own(e)).and_then(|e| e.weather)?;
        weather_kinds::WEATHER_ROW.iter().flatten().copied().find(|k| k.as_str() == w.as_str())
    }
    fn kind_word(&self) -> &'static str { self.entry.as_ref().map_or(SOLO_KEY, |e| e.kind.as_str()) }
    fn loadout(&self) -> Option<LoadoutEntry> { entry_of(&self.id) }
}

fn lesson_row(e: &ScenarioEntry) -> bool { e.group == Some(MissionGroup::Tutorial) }
fn row_own(e: &ScenarioEntry) -> bool { e.level.as_deref().is_some_and(|l| !l.is_empty()) && (e.hour.is_some() || e.weather.is_some()) }

/// The plain night, alone: not a scenario, no word but `?solo`.
fn solo_night() -> Mission {
    Mission {
        id: String::from(SOLO_ID), title: String::from("Solo night"), entry: None, squad: 0.0, group: MissionGroup::QuietAlone, place: String::from("The office, after dark"),
        blurb: String::from("You're alone on the office floor. Reach the server room, pull the drive and leave through the fire exit. Watch out for lights, noise and cameras."),
        lean: Lean::Quiet, untuned: false,
    }
}

/// The heading a row is under: its own, else by what it is, so that every row is under exactly one.
fn group_of(e: &ScenarioEntry) -> MissionGroup {
    e.group.unwrap_or(match e.kind {
        ScenarioEntryKind::Quiet => if e.squad != 0.0 { MissionGroup::QuietTwo } else { MissionGroup::QuietAlone },
        ScenarioEntryKind::Rescue => MissionGroup::Clock,
        _ => MissionGroup::Raids,
    })
}

fn mission_of(e: ScenarioEntry) -> Mission {
    let lean = e.lean.unwrap_or(match e.kind {
        ScenarioEntryKind::Nightbreach => Lean::Either,
        ScenarioEntryKind::Quiet => Lean::Quiet,
        _ => Lean::Loud,
    });
    let place = e.place.clone().unwrap_or_else(|| e.level.as_deref().filter(|l| !l.is_empty()).map_or(String::from("The office"), level_words));
    Mission { id: e.id.clone(), title: e.title.clone(), squad: e.squad, group: group_of(&e), place, blurb: e.blurb.clone().unwrap_or_default(), lean, untuned: e.untuned == Some(true), entry: Some(e) }
}

/// What the level is called on the sheet.
fn level_words(file: &str) -> String { String::from(file.strip_suffix(".json").unwrap_or(file)) }

fn lean_words(l: Lean) -> &'static str { match l { Lean::Quiet => "quiet", Lean::Loud => "loud", Lean::Either => "quiet or loud" } }

fn group_rank(g: MissionGroup) -> usize { GROUPS.iter().position(|x| x.0 == g).unwrap_or(GROUPS.len()) }

/// It runs and nobody has balanced it, and the short list does not name it.
fn unplayed(m: &Mission) -> bool { m.untuned && !list::on_short_list(&m.id) }

/// The whole list in its groups: inside a group the solo night first, then the tables' order, each group's unplayed rows after its others.
pub fn mission_list() -> Vec<Mission> {
    let mut all = vec![solo_night()];
    all.extend(all_missions().into_iter().map(mission_of));
    all.sort_by_key(|m| (group_rank(m.group), unplayed(m)));
    all
}

/// A mission by its id, cut from the lists or not.
pub(in crate::missions) fn mission_by_id(id: &str) -> Option<Mission> {
    if id == SOLO_ID { return Some(solo_night()); }
    all_missions().into_iter().find(|e| e.id == id).map(mission_of)
}

/// The row of `rows` an address boots.
fn mission_of_query<'a>(search: &str, rows: &'a [Mission]) -> Option<&'a Mission> {
    let q = Query::parse(search);
    let e = list::entry_of_query(&q, rows.iter().filter_map(|m| m.entry.as_ref()));
    if let Some(e) = e { return rows.iter().find(|m| m.id == e.id); }
    if q.has(SOLO_KEY) && !SCENARIO_KEYS.iter().any(|k| *k != SOLO_KEY && q.has(k)) { return rows.iter().find(|m| m.entry.is_none()); }
    None
}

/// Every mission there is, as its row says it.
pub fn missions_view() -> MissionsView {
    let mut all = vec![solo_night()];
    all.extend(all_missions().into_iter().map(mission_of));
    MissionsView { rows: all.into_iter().map(|m| {
        let (kind, group, level) = (String::from(m.kind_word()), String::from(m.group.as_str()), String::from(m.level().unwrap_or("office.json")));
        let link = m.entry.as_ref().map_or_else(|| format!("?{SOLO_KEY}"), list::scenario_query);
        let line = list::pick_of_mission(&m.id).map_or(String::new(), |p| String::from(p.line));
        // (the solo night is no row: its place is the list's own)
        let e = m.entry.unwrap_or_else(|| ScenarioEntry { place: Some(m.place), ..ScenarioEntry::default() });
        MissionRow {
            id: m.id, title: m.title, kind, squad: m.squad, group, level, blurb: m.blurb, untuned: m.untuned, line, link,
            hour: e.hour.map_or(String::new(), |h| String::from(h.as_str())), place: e.place.unwrap_or_default(),
            scene: e.scene.unwrap_or_default(), teaches: e.teaches.unwrap_or_default(), only: e.only.unwrap_or_default(),
            defenders: e.defenders.unwrap_or(0.0), squads: e.squads == Some(true), ways: e.ways.unwrap_or_default(), off_list: e.off_list == Some(true),
        }
    }).collect() }
}

/// The kind of mission an address starts ('' none), on the list or not.
pub fn kind_of_address(search: &str) -> String {
    list::spec_of(&Query::parse(search)).map_or(String::new(), |s| String::from(s.kind.as_str()))
}

/// The id of the mission an address starts, over every mission there is ('' none).
pub fn mission_of_address(search: &str) -> String {
    mission_of_query(search, &mission_list()).map_or(String::new(), |m| m.id.clone())
}

/// The rows a visit draws: the lists without the rows cut from them but the one it names, the short list's missions first in their groups, in play order.
fn start_rows(keep: &str, every: bool) -> Vec<Mission> {
    let mut rows: Vec<Mission> = mission_list().into_iter().filter(|m| every || !m.entry.as_ref().is_some_and(|e| e.off_list == Some(true)) || m.id == keep).collect();
    let play_order = |m: &Mission| list::PICKS.iter().position(|p| p.mission == m.id).unwrap_or(list::PICKS.len());
    rows.sort_by_key(|m| (group_rank(m.group), play_order(m)));
    rows
}

/// The row a visit names by name, or by being its mission's address: drawn though it is cut from the lists.
fn named_row(page: &str) -> String {
    let all = mission_list();
    if let Some(id) = Query::parse(page).get("loadout") {
        if all.iter().any(|m| m.id == id) { return String::from(id); }
    }
    mission_of_query(page, &all).map_or(String::new(), |m| m.id.clone())
}

// ---------------------------------------------------------------- what the list holds

/// What the list before a game holds of its own.
#[derive(Clone, Debug)]
pub struct Sitting {
    /// the visitor's address as it came, and as the list carries it (off a short-list row's page: without the row's name)
    page: String,
    search: String,
    hour: HourId,
    weather: Option<WeatherKind>,
    /// the picks made on the page, each for the address it was made on
    hour_pick: Option<(String, HourId)>,
    weather_pick: Option<(String, Option<WeatherKind>)>,
    /// the way in picked per mission: absent or 0 the level's own start, k >= 2 its k-th, -1 dealt
    ways: Vec<(String, f64)>,
    /// the lever words the machine's commands said, in their order: what it keeps, then what the address says over it
    said: Vec<(String, String)>,
    /// the address those words were said on (the first asked)
    booted: Option<String>,
    /// once a press has kept them: the levers this machine keeps, in the order they were moved. A link need not carry a lever the page it loads reads back
    stored: Option<Vec<LeverKey>>,
}

impl Default for Sitting {
    fn default() -> Sitting {
        Sitting { page: String::new(), search: String::new(), hour: HourId::Night, weather: None, hour_pick: None, weather_pick: None, ways: Vec::new(), said: Vec::new(), booted: None, stored: None }
    }
}

fn lever_key(word: &str) -> Option<LeverKey> { [LeverKey::Fog, LeverKey::Ai, LeverKey::Kit, LeverKey::Told].into_iter().find(|k| k.as_str() == word) }

/// Which way in an address asks for: 0 the level's own, k its k-th, -1 dealt.
fn way_of_link(ways: usize, word: Option<&str>) -> f64 {
    let Some(w) = word.filter(|w| !w.is_empty() && ways > 0) else { return 0.0 };
    if w == "any" { return -1.0; }
    if !w.bytes().all(|c| c.is_ascii_digit()) { return 0.0; }
    let n = js::number_of(w);
    if n >= 2.0 { crate::m::min(n, ways as f64) } else { 0.0 }
}

/// The way in on a link: the page's own taken off, then this row's; nothing for the level's own start.
fn way_search(href: &str, way: f64) -> String {
    let mut split = href.split('?');
    let path = split.next().unwrap_or("");
    let mut keys: Vec<String> = split.next().unwrap_or("").split('&').filter(|k| !k.is_empty() && *k != "way" && !k.starts_with("way=")).map(String::from).collect();
    if way < 0.0 { keys.push(String::from("way=any")); } else if way > 1.0 { keys.push(format!("way={}", js::num_str(way))); }
    if keys.is_empty() { String::from(path) } else { format!("{}?{}", path, keys.join("&")) }
}

fn way_note(m: &Mission, way: f64) -> String {
    String::from(if m.ways().is_none() { "this mission has one way in" }
        else if way < 0.0 { "the game picks one for you each time, along with where the defenders start" }
        else if way > 1.0 { "this side has a different first room, so plan it again" }
        else { "the level was built around this way in" })
}

fn choice(id: &str, label: &str, title: String, on: bool, shown: bool) -> SettingChoice {
    SettingChoice { id: String::from(id), label: String::from(label), title, on, shown }
}

impl Sitting {
    /// What the machine's commands said of the levers.
    pub fn said(&mut self, words: Vec<(String, String)>) { self.said.extend(words); }

    /// The levers this machine keeps, by the words it said: the address's own come last, one for each it names, and a word that is no rung was never kept.
    fn kept_said(&self, g: &Game) -> Vec<(LeverKey, &str)> {
        let boot = Query::parse(self.booted.as_deref().unwrap_or(""));
        let named = difficulty::levers_now(g).iter().filter(|l| boot.get(&l.param).is_some_and(|w| !w.is_empty())).count();
        let kept = &self.said[..self.said.len().saturating_sub(named)];
        kept.iter().filter_map(|(k, r)| Some((lever_key(k)?, r.as_str()))).filter(|(k, r)| difficulty::is_rung(g, *k, Some(r))).collect()
    }

    /// This machine keeps the levers as they stand: the page a link loads reads them back, and the link need not carry them.
    fn levers_stored(&self, g: &Game) -> bool {
        if self.stored.is_some() { return true; }
        let kept = self.kept_said(g);
        [LeverKey::Fog, LeverKey::Ai, LeverKey::Kit, LeverKey::Told].into_iter().all(|k| kept.iter().rev().find(|(x, _)| *x == k).map(|(_, r)| *r) == difficulty::lever_set(g, k))
    }

    /// The levers moved, in the order they were moved: by the machine's words until a press has kept them.
    fn moved_order(&self, g: &Game) -> Vec<LeverKey> {
        if let Some(o) = &self.stored { return o.clone(); }
        let mut order: Vec<LeverKey> = Vec::new();
        for (k, r) in &self.said {
            let Some(k) = lever_key(k) else { continue };
            if !order.contains(&k) && difficulty::is_rung(g, k, Some(r)) { order.push(k); }
        }
        order.retain(|k| difficulty::lever_set(g, *k).is_some());
        order
    }

    /// The way a mission goes in by.
    fn way_of(&self, m: &Mission) -> f64 {
        if m.ways().is_none() { return 0.0; }
        let w = self.ways.iter().find(|(id, _)| *id == m.id).map_or(0.0, |x| x.1);
        if w < 0.0 { 0.0 } else { w }
    }

    /// The link a row has.
    fn href_of(&self, g: &mut Game, m: &Mission) -> String {
        let kept = self.levers_stored(g);
        let href = match &m.entry {
            Some(e) => difficulty::difficulty_href_kept(g, &list::pick_search(e, &self.search), kept),
            None => { let rest = list::quit_search(&self.search); if rest.is_empty() { format!("?{SOLO_KEY}") } else { format!("?{SOLO_KEY}&{}", &rest[1..]) } }
        };
        let quiet = m.quiet_row();
        let href = quietfog::quiet_fog_href(g, &href, quiet);
        let href = hours::hour_href(&href, self.hour);
        let wx = if m.own_weather(g) { None } else { self.weather };
        way_search(&weather_kinds::weather_search(&href, wx), self.way_of(m))
    }

    /// The level a short-list row is started at: levers moved to a mix of their own, the level they stand over; rows moved by hand, none.
    fn level_word(&self, g: &Game) -> Option<Difficulty> {
        if difficulty::levers_mixed(g) { return Some(g.m.difficulty.base); }
        match difficulty::difficulty_now(g) { DifficultyNow::Standard => Some(Difficulty::Standard), DifficultyNow::Hard => Some(Difficulty::Hard), DifficultyNow::Custom => None }
    }

    /// The settings are the short list's for this mission: its pick, else none.
    fn as_pick(&self, g: &mut Game, m: &Mission) -> Option<Pick> {
        let p = list::pick_of_mission(&m.id)?;
        // (every level there is may be its row's: the row's link names the one it is at)
        let level = list::pick_entry(&p).is_none() || self.level_word(g).is_some();
        if !level || self.hour != HourId::Night || self.weather.is_some() { return None; }
        let href = self.href_of(g, m);
        if href.contains("?way=") || href.contains("&way=") { None } else { Some(p) }
    }

    /// The link a mission's team is deployed by, without its size: the short list's own where the settings are still its own.
    pub fn base_of(&self, g: &mut Game, m: &Mission) -> String {
        let Some(p) = self.as_pick(g, m) else { return self.href_of(g, m) };
        let level = if list::pick_entry(&p).is_some() { self.level_word(g) } else { None };
        let h = list::pick_href(&p, &self.page, level);
        let kept = self.levers_stored(g);
        difficulty::levers_href_kept(g, &h, kept)
    }

    /// The link Deploy loads at a size, from `base` (empty: the mission's own).
    pub fn deploy_link(&self, g: &mut Game, id: &str, base: &str, size: f64) -> String {
        let Some(m) = mission_by_id(id) else { return String::new() };
        let base = if base.is_empty() { self.base_of(g, &m) } else { String::from(base) };
        let kept = self.levers_stored(g);
        let href = difficulty::levers_href_kept(g, &base, kept);
        let quiet = m.quiet_row();
        loadout::squad_search(&quietfog::quiet_fog_href(g, &href, quiet), size, m.squad)
    }

    /// The link a mission's kit is read by, as it would deploy now: whatever size comes, its fog and its cameras are the same.
    pub fn fog_link(&self, g: &mut Game, id: &str) -> String {
        let Some(m) = mission_by_id(id) else { return String::new() };
        let base = self.base_of(g, &m);
        let quiet = m.quiet_row();
        quietfog::quiet_fog_href(g, &base, quiet)
    }

    /// The visitor's address is this from now on: what it says of the hour, the weather and the way in, unless picked already for the same address.
    pub fn start(&mut self, g: &mut Game, page: &str) -> StartView {
        if self.booted.is_none() { self.booted = Some(String::from(page)); }
        self.page = String::from(page);
        self.search = list::pick_leave_search(page);
        let q = Query::parse(&self.search);
        self.hour = match &self.hour_pick { Some((s, h)) if *s == self.search => *h, _ => hours::hour_id_of(q.get("hour")) };
        self.weather = match &self.weather_pick { Some((s, w)) if *s == self.search => *w, _ => weather_kinds::weather_of(&q) };
        // (`offlist=0`: every row of the table drawn, as before the cut)
        let rows = start_rows(&named_row(page), Query::parse(page).get("offlist") == Some("0"));
        if let Some(m) = mission_of_query(&self.search, &rows) {
            let w = q.get("way").filter(|w| !w.is_empty());
            if let (Some(ways), true, true) = (m.ways(), w.is_some(), !self.ways.iter().any(|x| x.0 == m.id)) { self.ways.push((m.id.clone(), way_of_link(ways.len(), w))); }
        }
        let rows: Vec<StartRow> = rows.iter().map(|m| self.row_of(g, m)).collect();
        StartView { rows, named: mission_of_query(page, &mission_list()).map_or(String::new(), |m| m.id.clone()), pick: list::pick_of_query(page).map_or(String::new(), |p| String::from(p.id)), playing: list::names_mission(page), first: String::from(list::PICKS[0].mission) }
    }

    fn row_of(&self, g: &mut Game, m: &Mission) -> StartRow {
        let group = GROUPS.iter().find(|x| x.0 == m.group);
        let pick = list::pick_of_mission(&m.id);
        let e = m.entry.as_ref();
        let loadout = m.loadout();
        StartRow {
            id: m.id.clone(),
            title: m.title.clone(),
            kind: String::from(m.kind_word()),
            group: String::from(m.group.as_str()),
            group_title: String::from(group.map_or("", |x| x.1)),
            group_line: String::from(group.map_or("", |x| x.2)),
            place: m.place.clone(),
            blurb: m.blurb.clone(),
            lean: String::from(m.lean.as_str()),
            lean_words: String::from(lean_words(m.lean)),
            level: String::from(m.level().unwrap_or("office.json")),
            level_words: m.level().map_or(String::from("office"), level_words),
            squad: m.squad,
            team_words: if m.squad != 0.0 { loadout::size_words(m.squad) } else { String::from("you, alone") },
            team_tag: String::from(loadout::size_tag(m.squad)),
            unplayed: unplayed(m),
            off_list: e.is_some_and(|e| e.off_list == Some(true)),
            as_issued: loadout.as_ref().is_some_and(kit_as_issued),
            lesson: m.lesson(),
            two: e.is_some_and(|e| crate::seats::seats_kind(e.kind.as_str()) && e.only.is_none()) && !m.lesson(),
            ways: m.ways().map_or(Vec::new(), |w| w.to_vec()),
            pick: pick.map_or(String::new(), |p| String::from(p.id)),
            line: pick.map_or(String::new(), |p| String::from(p.line)),
            pick_squad: if pick.as_ref().and_then(list::pick_entry).is_some() { m.squad } else { -1.0 },
            link: self.href_of(g, m),
        }
    }

    /// The structure the mission is fought on, as its levers are lit there ('' = one with no rows of its own).
    fn structure_of(m: &Mission) -> String {
        m.entry.as_ref().filter(|e| e.kind == ScenarioEntryKind::Roomclear).map_or(String::new(), |e| difficulty::structure_of_level(e.level.as_deref()))
    }

    /// The rows of settings of one mission, in the sheet's order.
    pub fn sheet(&self, g: &mut Game, id: &str) -> SheetView {
        let Some(m) = mission_by_id(id) else { return SheetView::default() };
        let link = self.href_of(g, &m);
        let mut settings = vec![difficulty_row(g, &m)];
        lever_rows(g, &m, &link, &mut settings);
        settings.push(self.hour_row(g, &m));
        settings.push(self.weather_row(g, &m));
        settings.push(self.way_row(&m));
        let quiet = matches!(m.kind_word(), "quiet" | SOLO_KEY) && !Query::parse(after_mark(&self.page)).has("kitloud");
        SheetView { id: m.id.clone(), settings, base: self.base_of(g, &m), as_pick: self.as_pick(g, &m).is_some(), quiet, link }
    }

    fn hour_row(&self, g: &Game, m: &Mission) -> Setting {
        let wx = if m.own_weather(g) { None } else { m.row_weather().or(self.weather) };
        Setting {
            key: String::from("hour"), label: String::from("Hour"), live: true, shown: m.row_hour().is_none(), moved: false,
            note: hours::hour_note(self.hour, wx.is_some(), m.row_weather().is_some()),
            choices: hours::HOUR_ROW.iter().map(|h| choice(h.as_str(), &hours::hour_label(*h), String::new(), self.hour == *h, true)).collect(),
        }
    }

    fn weather_row(&self, g: &Game, m: &Mission) -> Setting {
        Setting {
            key: String::from("weather"), label: String::from("Weather"), live: true, shown: !m.own_weather(g) && m.row_weather().is_none(), moved: false,
            note: weather_kinds::weather_note(g, self.weather, m.row_hour().unwrap_or(self.hour)),
            choices: weather_kinds::WEATHER_ROW.iter().map(|k| choice(k.map_or("default", |k| k.as_str()), &weather_kinds::weather_title(g, *k), String::new(), self.weather == *k, true)).collect(),
        }
    }

    fn way_row(&self, m: &Mission) -> Setting {
        let y = self.way_of(m);
        let mut choices: Vec<SettingChoice> = m.ways().unwrap_or(&[]).iter().enumerate().map(|(k, w)| {
            let v = if k == 0 { 0.0 } else { k as f64 + 1.0 };
            choice(&js::num_str((k + 1) as f64), w, String::new(), y == v, true)
        }).collect();
        // (dealt is offered only while missions may start different ways: they all start the same way now)
        choices.push(choice("any", "Dealt", String::new(), false, false));
        let shown = m.ways().is_some();
        Setting { key: String::from("way"), label: String::from("Way in"), live: shown, shown, moved: false, note: way_note(m, y), choices }
    }

    /// One choice of one row pressed: refused (None) where the row is not drawn or not live, or the choice is not offered; else every text this machine would keep now for what moved.
    pub fn press(&mut self, g: &mut Game, id: &str, key: &str, to: &str) -> Option<Vec<KeptText>> {
        let m = mission_by_id(id)?;
        let sheet = self.sheet(g, id);
        let row = sheet.settings.iter().find(|s| s.key == key)?;
        if !row.live || !row.shown || !row.choices.iter().any(|c| c.id == to && c.shown) { return None; }
        let text = |key: &str, text: String| KeptText { key: String::from(key), text };
        match key {
            "difficulty" => {
                difficulty::set_difficulty(g, if to == "hard" { Difficulty::Hard } else { Difficulty::Standard }, Some(true));
                self.stored = Some(Vec::new());
                Some(vec![text(KEPT_LEVEL, difficulty::level_text(g)), text(KEPT_LEVERS, String::new())])
            }
            "fog" if m.quiet_row() => {
                quietfog::set_quiet_fog(g, [QuietRung::Off, QuietRung::Team, QuietRung::Cones, QuietRung::Own].into_iter().find(|r| r.as_str() == to)?, Some(true));
                Some(vec![text(KEPT_FOG, quietfog::quiet_fog_text(g))])
            }
            "cams" => {
                let on = to == "1";
                let lever_was = quietfog::quiet_fog(g);
                quietfog::set_quiet_cams(g, on, Some(true));
                let mut kept = vec![text(KEPT_CAMS, quietfog::quiet_cams_text(g, on))];
                if quietfog::quiet_fog(g) != lever_was { kept.push(text(KEPT_FOG, quietfog::quiet_fog_text(g))); }
                Some(kept)
            }
            "hour" => {
                self.hour = hours::HOUR_ROW.into_iter().find(|h| h.as_str() == to)?;
                self.hour_pick = Some((self.search.clone(), self.hour));
                Some(Vec::new())
            }
            "weather" => {
                self.weather = weather_kinds::WEATHER_ROW.into_iter().find(|k| k.map_or("default", |k| k.as_str()) == to)?;
                self.weather_pick = Some((self.search.clone(), self.weather));
                Some(Vec::new())
            }
            "way" => {
                let n = js::number_of(to);
                let v = if n >= 2.0 { n } else { 0.0 };
                match self.ways.iter_mut().find(|x| x.0 == m.id) { Some(x) => x.1 = v, None => self.ways.push((m.id.clone(), v)) }
                Some(Vec::new())
            }
            _ => {
                let lever = lever_key(key)?;
                let mut order = self.moved_order(g);
                let named = difficulty::set_lever(g, lever, Some(to), Some(true), Some(&Self::structure_of(&m)));
                order.retain(|k| difficulty::lever_set(g, *k).is_some());
                for k in [LeverKey::Fog, LeverKey::Ai, LeverKey::Kit, LeverKey::Told] {
                    if difficulty::lever_set(g, k).is_some() && !order.contains(&k) { order.push(k); }
                }
                let mut kept = vec![text(KEPT_LEVERS, difficulty::levers_text(g, &order))];
                if named { kept.push(text(KEPT_LEVEL, difficulty::level_text(g))); }
                self.stored = Some(order);
                Some(kept)
            }
        }
    }

    /// What the pause and end cards say of the row after the one the address is, and the page load that starts it.
    pub fn short_view(&self, g: &Game, search: &str) -> ShortView {
        let Some((now, next, carried)) = list::pick_after(search) else { return ShortView::default() };
        let mut v = ShortView { now: String::from(now.id), now_scenario: list::pick_entry(&now).is_some(), ..ShortView::default() };
        let Some(next) = next else { return v };
        let entry = list::pick_entry(&next);
        let kept = self.levers_stored(g);
        v.next = String::from(next.id);
        v.next_mission = String::from(next.mission);
        v.title = entry.as_ref().map_or(String::from("Solo night"), |e| e.title.clone());
        if let Some(d) = carried { v.title = format!("{} \u{b7} {}", v.title, d.as_str()); }
        v.line = String::from(next.line);
        v.link = difficulty::levers_href_kept(g, &list::pick_href(&next, search, carried), kept);
        // (a row left on its own difficulty offers the other beside it; the solo night has no switch)
        if entry.is_some() { v.other_link = difficulty::levers_href_kept(g, &list::pick_href(&next, search, Some(Difficulty::Hard)), kept); }
        v
    }
}

/// Standard | Hard, greyed where the pick changes nothing.
fn difficulty_row(g: &Game, m: &Mission) -> Setting {
    let now = difficulty::difficulty_now(g);
    let level = match now { DifficultyNow::Standard => DifficultyWordsLevel::Standard, DifficultyNow::Hard => DifficultyWordsLevel::Hard, DifficultyNow::Custom => DifficultyWordsLevel::Custom };
    let note = match &m.entry {
        Some(e) if e.kind == ScenarioEntryKind::Quiet => String::from("a quiet mission uses the level's own shift on either setting"),
        Some(e) => difficulty::difficulty_words(g, Some(level), Some(&difficulty::squads_held(e))),
        None => String::from("the night shift is the same on either setting"),
    };
    Setting {
        key: String::from("difficulty"), label: String::from("Difficulty"), live: m.diff_live(), shown: true, moved: false, note,
        choices: vec![choice("standard", "standard", String::new(), now == DifficultyNow::Standard, true), choice("hard", "hard", String::new(), now == DifficultyNow::Hard, true)],
    }
}

/// The cameras' one line: the note under the row while it is lit, and its button's title.
const CAM_WORDS: &str = "The last one you put on a wall is one more eye in the fog. An optical wand for doors comes with them.";

/// The levers under it, each with the line of its lit rung; on a quiet row the fog's is the quiet pick's, with the cameras' row right under it.
fn lever_rows(g: &mut Game, m: &Mission, link: &str, out: &mut Vec<Setting>) {
    let quiet = m.quiet_row();
    // (the fog of war is up on a room clear and a rescue; a lesson is taught on the level as it is, and reads no lever)
    let fog_on = m.entry.is_some() && scenario_run::fog_of_link(g, &Query::parse(after_mark(link)));
    let structure = Sitting::structure_of(m);
    let shown = difficulty::levers_shown(g, Some(&structure));
    let cams_on = quiet && quietfog::quiet_cams(g);
    let quiet_lit = quietfog::quiet_fog(g);
    for l in difficulty::levers_now(g) {
        let live = !m.lesson() && if matches!(l.key, LeverKey::Fog | LeverKey::Told) { fog_on || quiet } else { m.diff_live() };
        let mine = quiet && l.key == LeverKey::Fog;
        let lit: Option<&str> = if mine { Some(quiet_lit.as_str()) } else {
            match l.key {
                LeverKey::Fog => shown.fog.map(|r| r.as_str()),
                LeverKey::Ai => shown.ai.map(|r| r.as_str()),
                LeverKey::Kit => shown.kit.map(|r: KitRung| r.as_str()),
                LeverKey::Told => shown.told.map(|r| r.as_str()),
            }
        };
        let moved = if mine { quiet_lit != quietfog::quiet_fog_default(g) } else { difficulty::lever_set(g, l.key).is_some() };
        let note = if !live { String::new() }
            else if mine { quietfog::quiet_fog_words(lit) }
            else { let w = difficulty::lever_words(g, l.key, lit); if w.is_empty() { String::from("moved by hand in the debugger") } else { w } };
        let choices = l.rungs.iter().map(|r| {
            let on = lit == Some(r.id.as_str());
            let title = if l.key != LeverKey::Fog { r.words.clone() } else if mine { quietfog::quiet_fog_words(Some(&r.id)) } else { difficulty::lever_words(g, LeverKey::Fog, Some(&r.id)) };
            // (the bottom rung: the quiet pick's on a quiet row, on a squad's row where the fog's lever is live, and wherever it is the one lit)
            let offered = l.key != LeverKey::Fog || r.id != "off" || mine || live || on;
            choice(&r.id, &r.label, title, on, offered)
        }).collect();
        out.push(Setting { key: String::from(l.key.as_str()), label: l.label.clone(), live, shown: true, moved, note, choices });
        if l.key != LeverKey::Fog { continue; }
        let issue = js::num_str(g.k.sticky.issue);
        out.push(Setting {
            key: String::from("cams"), label: String::from("Sticky cameras"), live, shown: mine,
            moved: mine && quiet_lit != QuietRung::Off && cams_on != quietfog::quiet_cams_default(g),
            note: if cams_on { String::from(CAM_WORDS) } else { String::new() },
            choices: vec![choice("0", "None", String::new(), !cams_on, true), choice("1", &issue, String::from(CAM_WORDS), cams_on, true)],
        });
    }
}

/// A row of the short list's link ('' where the row is none): `hard` -1 the row's own difficulty, 0 standard, 1 hard.
pub fn short_link(id: &str, search: &str, hard: i32) -> String {
    let Some(p) = list::pick_by_id(id) else { return String::new() };
    let d = match hard { 0 => Some(Difficulty::Standard), 1 => Some(Difficulty::Hard), _ => None };
    list::pick_href(&p, search, d)
}

/// What a page says of the mission its address starts, while it loads: that mission's own row, the team as this address deploys it. A row deployed at a size no row of its kind has is the row of
/// that kind on that level; nothing where the address names no mission of the list, or picks its level by `level=` (a level being built is played the moment it loads).
pub fn brief_of(search: &str) -> BriefView {
    let q = Query::parse(search);
    if q.has("level") { return BriefView::default(); }
    let spec = list::spec_of(&q);
    let mut rows = vec![solo_night()];
    rows.extend(all_missions().into_iter().map(mission_of));
    let row = mission_of_query(search, &rows).or_else(|| {
        let s = spec.as_ref()?;
        let here = || rows.iter().filter(|m| m.entry.as_ref().is_some_and(|e| e.kind.as_str() == s.kind.as_str() && e.level.as_deref().unwrap_or("") == s.level.as_deref().unwrap_or("")));
        here().find(|m| m.squad == s.squad).or_else(|| here().next())
    });
    let Some(m) = row else { return BriefView::default() };
    let squad = spec.as_ref().map_or(0.0, |s| s.squad);
    let e = m.entry.as_ref();
    BriefView {
        id: m.id.clone(),
        title: m.title.clone(),
        kind: String::from(m.kind_word()),
        group_title: String::from(GROUPS.iter().find(|x| x.0 == m.group).map_or("", |x| x.1)),
        place: m.place.clone(),
        blurb: m.blurb.clone(),
        lean_words: String::from(lean_words(m.lean)),
        team: if squad != 0.0 { loadout::size_words(squad) } else { String::from("you, alone") },
        level: String::from(m.level().unwrap_or("office.json")),
        scene: e.and_then(|e| e.scene.clone()).unwrap_or_default(),
        teaches: e.and_then(|e| e.teaches.clone()).unwrap_or_default(),
        // (dealt is marked on no one way: the level's own)
        way: crate::m::max(way_of_link(m.ways().map_or(0, |w| w.len()), q.get("way")), 0.0),
    }
}

/// The link Deploy would load starts what a page at this address already stands: the plain night, at its hour, in its weather, in the same fog, with nothing else asked of the page. So the start
/// is no page load.
pub fn same_start(g: &Game, link: &str, page: &str) -> bool {
    let (a, b) = (Query::parse(link), Query::parse(page));
    let fog = |q: &Query| (scenario_run::fog_of_query(g, q), scenario_run::quiet_rung_of_query(g, q), scenario_run::quiet_cams_of_link(g, q));
    hours::hour_id_of(a.get("hour")) == hours::hour_id_of(b.get("hour"))
        && weather_kinds::weather_of(&a) == weather_kinds::weather_of(&b)
        && !SCENARIO_KEYS.iter().any(|k| *k != SOLO_KEY && b.has(k))
        && fog(&a) == fog(&b)
}
