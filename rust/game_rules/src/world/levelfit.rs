#![allow(unused_imports, unused_variables)]
pub use ::game_types::levelfit::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::boxworld::{MAX_BOXES, MAX_CELLS, MAX_CUTS, MAX_ITEMS, MAX_LEAVES, MAX_SHEETS, MAX_THINGS, MAX_TILTED};
use alloc::{string::String, vec, vec::Vec};

// ---- the terms of the count: called by the bodies below

/// Men a game holds at once, at most.
fn men_most(g: &crate::game::Game, civilians: f64) -> f64 {
    crate::guards::MAX_SKINNED + civilians + g.k.seats.max - 1.0
}

/// Men one step can be handed: the doors' hand-over in contacts, by the contacts one man is at most, and the furniture's in men.
fn men_held() -> f64 {
    m::floor(crate::doors::DOOR_CONTACTS / (crate::game::body_door_particles().len() as f64 + 1.0))
}

/// What one step is handed of the men, at most.
fn hand_over_most(g: &crate::game::Game, civilians: f64) -> crate::levelfit::HandOverMost {
    let door_men = men_most(g, civilians) * (crate::game::body_door_particles().len() as f64 + 1.0);
    crate::levelfit::HandOverMost { door_men }
}

/// How many of each counted thing the game holds.
pub fn level_caps() -> crate::levelfit::LevelFit {
    crate::levelfit::LevelFit {
        doors: crate::doors::DOOR_CAP as f64,
        things: crate::props::PROP_CAP,
        parts: crate::props::PROP_PARTS,
        sheets: MAX_SHEETS as f64,
        fixtures: crate::fixtures::FIXTURES as f64,
        cameras: crate::fixtures::CAMERAS as f64,
        lasers: crate::fixtures::LASERS as f64,
        beacons: crate::fixtures::BEACONS as f64,
        lamps: ::sim::lights::ROWS as f64,
    }
}

/// `level.boxes[i]`: nothing where the index names no box.
fn box_at(boxes: &[BoxId], i: f64) -> Option<BoxId> {
    if i >= 0.0 && i == m::floor(i) { boxes.get(i as usize).copied() } else { None }
}

/// A fixture box's markings that can feed a lamp.
fn marked(g: &Game, group: &str, b: Option<BoxId>) -> f64 {
    let Some(b) = b else { return 0.0 };
    let row = &g.tab.boxes[b];
    if group == "emergency" || row.rot.is_some() { return 0.0; }
    match &row.marks { Some(l) => l.len() as f64, None => 0.0 }
}

/// What a level says of itself, counted before it is built.
fn level_counts(g: &crate::game::Game, level: &crate::levelfit::LevelCountsLevel) -> crate::levelfit::LevelFit {
    let (mut parts, mut sheets) = (0.0, 0.0);
    for p in level.props.iter() { parts += p.parts.len() as f64; }
    for &b in level.boxes.iter() { if crate::glass::is_sheet(g, Some(b)) { sheets += 1.0; } }
    let (mut cameras, mut lasers) = (0.0, 0.0);
    if let Some(list) = &level.devices {
        for d in list.iter() {
            if d.kind == "camera" { cameras += 1.0; } else if d.kind == "laser" { lasers += 1.0; }
        }
    }
    let lights = level.lights.as_ref().map_or(0.0, |l| l.len() as f64);
    let no_switches = Vec::new();
    let switches = level.switchables.as_ref().unwrap_or(&no_switches);
    let beacons = level.beacons.as_ref().map_or(0.0, |l| l.len() as f64);
    let cars = crate::carlamps::find_car_lamps(g, &level.boxes);
    let mut flat = 0.0;
    for s in switches.iter() {
        for &bi in s.boxes.iter() {
            let b = box_at(&level.boxes, bi);
            let panel = match b { Some(id) => { let h = g.tab.boxes[id].h; flat_panel(&s.group, &h).is_some() } None => false };
            flat += (if panel { 1.0 } else { 0.0 }) + marked(g, &s.group, b);
        }
    }
    for c in cars.iter() { flat += marked(g, crate::carlamps::VEHICLE_GROUP, box_at(&level.boxes, c.box_)); }
    crate::levelfit::LevelFit {
        doors: level.doors.len() as f64,
        things: level.props.len() as f64,
        parts,
        sheets,
        fixtures: lights + switches.len() as f64 + cars.len() as f64 + cameras + lasers + 1.0,
        cameras,
        lasers,
        beacons,
        lamps: lights + flat + beacons + cameras,
    }
}

/// Statics that carry a cut record.
fn cut_statics(g: &crate::game::Game, boxes: &[BoxId]) -> f64 {
    let mut n = 0.0;
    for &b in boxes.iter() { if g.tab.boxes[b].cut.as_ref().is_some_and(|c| !c.is_empty()) { n += 1.0; } }
    n
}

/// The clumps the largest sheet of a level's glass dices into, as the rule cuts them.
fn clumps_most(g: &crate::game::Game, boxes: &[BoxId]) -> f64 {
    let mut n = 0.0;
    for &b in boxes.iter() {
        if !crate::glass::is_sheet(g, Some(b)) { continue; }
        let (h, k) = (g.tab.boxes[b].h, g.k.glassbreak.clump_m);
        let across = m::max(1.0, m::round(2.0 * m::max(h[0], h[2]) / k));
        let up = m::max(1.0, m::round(2.0 * h[1] / k));
        n = m::max(n, across * up);
    }
    n
}

/// The grid's own count of the game's box world: cells, entries of them, tilted statics.
fn grid_count(g: &Game) -> (f64, f64, f64) {
    let w = g.tab.boxes.world();
    ((w.w * w.h) as f64, w.items.len() as f64, (w.tilted.len() / 5) as f64)
}

/// Boxes of the list with this flag up.
fn flagged(g: &Game, boxes: &[BoxId], flag: f64) -> f64 {
    let mut n = 0.0;
    for &b in boxes.iter() { if js::to_i32(g.tab.boxes[b].flags) & js::to_i32(flag) != 0 { n += 1.0; } }
    n
}

/// One row of the count.
fn row(what: &str, has: f64, cap: f64) -> crate::levelfit::FitRow {
    crate::levelfit::FitRow { what: String::from(what), has, cap, play: None }
}

/// Every count of a stood game against what holds it.
fn fit_rows(g: &crate::game::Game, g_2: &crate::levelfit::Stood) -> Vec<crate::levelfit::FitRow> {
    let col = &g_2.col;
    let lv = &g_2.level;
    let counted = crate::levelfit::LevelCountsLevel {
        doors: lv.doors.clone(),
        props: lv.props.clone(),
        boxes: lv.boxes.clone(),
        lights: lv.lights.clone(),
        switchables: lv.switchables.clone(),
        beacons: lv.beacons.clone(),
        devices: lv.devices.clone(),
    };
    let big = level_counts(g, &counted);
    let cap = level_caps();
    let civs = m::max(
        g_2.civilians.as_ref().map_or(0.0, |l| l.len() as f64),
        lv.civilians.as_ref().map_or(0.0, |l| l.len() as f64),
    );
    let most = hand_over_most(g, civs);
    let flag = crate::scene_boxes::box_flag();
    let glass = flagged(g, &col.boxes, flag.glass);
    let mut parts = 0.0;
    let (cells, entries, tilted) = grid_count(g);
    if let Some(p) = &g_2.props { for i in p.all.iter() { parts += i.boxes.len() as f64; } }
    let things = g_2.props.as_ref().map_or(0.0, |p| p.all.len() as f64);
    let guards = g_2.guards.as_ref().map_or(0.0, |l| l.len() as f64);
    let cuts = cut_statics(g, &col.boxes);
    let pieces = crate::wallbreak::wall_break_pieces_sure(g);
    let mut rows = vec![
        row("statics", col.boxes.len() as f64, MAX_BOXES as f64),
        row("cells of the floor grid", cells, MAX_CELLS as f64),
        row("entries of those cells", entries, MAX_ITEMS as f64),
        row("tilted statics", tilted, MAX_TILTED as f64),
        row("statics that carry a cut record (a cut or a shaped static takes a cut row; so does a piece with a ragged end)", cuts, MAX_CUTS as f64),
        crate::levelfit::FitRow { play: Some(true), ..row("pieces a broken level may stand up (the wall breaker's knob) against the pieces its room in the three tables above surely holds", g.k.walls.max_pieces, pieces) },
        row("leaves (a row a door)", col.dynamic_boxes.len() as f64, MAX_LEAVES as f64),
        row("boxes of the loose furniture (a row a box)", parts, m::min(MAX_LEAVES as f64, cap.parts)),
        row("loose things of furniture", things, m::min(MAX_THINGS as f64, cap.things)),
        row("glass one line can note (no more than the level's glass)", glass, ::sim::world::MAX_PANES as f64),
        row("sheets of glass (a row a pane that is no longer whole)", big.sheets, cap.sheets),
    ];
    let clumps = clumps_most(g, &lv.boxes);
    rows.push(row("clumps the largest sheet of glass dices into", clumps, ::sim::panes_cap(1) as f64));
    let bars = flagged(g, &col.boxes, flag.bar);
    rows.push(row("window's bars (a row a bar gone, in a game)", bars, ::sim::panes_cap(0) as f64));
    rows.push(row("doors (a row a door, and a row a leaf off its hinges)", big.doors, cap.doors));
    rows.push(row("a man a seat and the guards as the level is stood up (what guards.ts spawnUnit counts against the picture's budget of characters)", crate::seats::seat_count(g) + guards, crate::guards::MAX_SKINNED));
    let man_rows = men_most(g, civs);
    rows.push(row("rows of the men's columns against every man a game can hold (menMost: at most what a step's hand-over holds, by civMost; a man who has left keeps no row)", man_rows, ::sim::men::ROWS as f64));
    let civ_cap = civ_most(g);
    rows.push(row("civilians: the level's own, or those in the game as it stands (a rescue's hostages join in play; civilians.ts join holds every one to this number as he joins)", civs, civ_cap));
    rows.push(row("contacts one step of the doors is handed at most (every man a game holds: himself, and his body's particles near a doorway)", most.door_men, crate::doors::DOOR_CONTACTS));
    rows.push(row("light fixtures (a row of the lights' columns each)", big.fixtures, cap.fixtures));
    rows.push(row("lamps on its fixtures, at most (a row of the table a light question reads)", big.lamps, cap.lamps));
    rows.push(row("cameras", big.cameras, cap.cameras));
    rows.push(row("tripwires", big.lasers, cap.lasers));
    rows.push(row("emergency beacons", big.beacons, cap.beacons));
    rows
}

// ---- the functions the rest of the game calls

/// Civilians a game holds at once, at most: what a step's hand-over leaves after every other man a game may hold.
pub fn civ_most(g: &crate::game::Game) -> f64 {
    let held = men_held();
    held - men_most(g, 0.0)
}

/// A level the game will not load, said by its name where it has one.
pub fn level_too_big(g: &crate::game::Game, why: &str) -> String {
    let name = crate::scene_level_load::level_name_of(g);
    match name {
        Some(n) if !n.is_empty() => alloc::format!("the level '{}' is too big to load: {}", n, why),
        _ => alloc::format!("this level is too big to load: {}", why),
    }
}

/// A fixture's box that lights the room as a flat panel, or nothing: too thick, too small, or a kind of fixture that only glows.
pub fn flat_panel(group: &str, h: &[f64]) -> Option<crate::levelfit::FlatPanel> {
    if group == "server_leds" || group == "emergency" || group == crate::carlamps::VEHICLE_GROUP { return None; }
    let thin: usize = if h[0] <= h[1] && h[0] <= h[2] { 0 } else if h[1] <= h[2] { 1 } else { 2 };
    if h[thin] > 0.06 { return None; }
    let (a0, a1) = ((thin + 1) % 3, (thin + 2) % 3);
    let area = 4.0 * h[a0] * h[a1];
    if area < 0.01 { return None; }
    Some(crate::levelfit::FlatPanel { thin: thin as f64, a0: a0 as f64, a1: a1 as f64, area })
}

/// Why this level cannot be loaded, in words, or nothing: it fits.
pub fn level_over(g: &crate::game::Game, level: &crate::levelfit::LevelCountsLevel) -> Option<String> {
    let has = level_counts(g, level);
    let cap = level_caps();
    let n = js::num_str;
    let mut over: Vec<String> = Vec::new();
    if has.doors > cap.doors { over.push(alloc::format!("{} doors (the game holds {})", n(has.doors), n(cap.doors))); }
    if has.things > cap.things { over.push(alloc::format!("{} loose things of furniture ({})", n(has.things), n(cap.things))); }
    if has.parts > cap.parts { over.push(alloc::format!("loose furniture of {} boxes ({})", n(has.parts), n(cap.parts))); }
    if has.sheets > cap.sheets { over.push(alloc::format!("{} sheets of glass ({})", n(has.sheets), n(cap.sheets))); }
    if has.fixtures > cap.fixtures { over.push(alloc::format!("{} light fixtures ({})", n(has.fixtures), n(cap.fixtures))); }
    if has.lamps > cap.lamps { over.push(alloc::format!("lamps on its fixtures, up to {} ({})", n(has.lamps), n(cap.lamps))); }
    if has.cameras > cap.cameras { over.push(alloc::format!("{} cameras ({})", n(has.cameras), n(cap.cameras))); }
    if has.lasers > cap.lasers { over.push(alloc::format!("{} tripwires ({})", n(has.lasers), n(cap.lasers))); }
    if has.beacons > cap.beacons { over.push(alloc::format!("{} emergency beacons ({})", n(has.beacons), n(cap.beacons))); }
    if over.is_empty() { None } else { Some(level_too_big(g, &over.join(", "))) }
}

/// The room a level leaves for its pieces: how many more statics, entries of the grid's cells and cut rows a level may have than this one stands with.
pub fn break_room(g: &crate::game::Game, col: &crate::levelfit::StoodCol) -> crate::levelfit::BreakRoom {
    let (_, entries, _) = grid_count(g);
    let cut = cut_statics(g, &col.boxes);
    crate::levelfit::BreakRoom {
        boxes: MAX_BOXES as f64 - col.boxes.len() as f64,
        items: MAX_ITEMS as f64 - entries,
        cuts: MAX_CUTS as f64 - cut,
    }
}

/// Why this stood game cannot be played, in words, or nothing: every row of the count not held in play is inside what holds it.
pub fn stood_over(g: &crate::game::Game, g_2: &crate::levelfit::Stood) -> Option<String> {
    let over: Vec<String> = fit_rows(g, g_2).iter()
        .filter(|r| !r.play.unwrap_or(false) && r.has > r.cap)
        .map(|r| alloc::format!("{} {} ({})", js::num_str(r.has), r.what, js::num_str(r.cap)))
        .collect();
    if over.is_empty() { None } else { Some(level_too_big(g, &over.join(", "))) }
}
