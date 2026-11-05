//! src/game/chronicle.ts lines 1 to 459
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ------------------------------------------------------------ private helpers

/// a row's data, as `ChronEvent` holds it; the writers below build this one and the public entries convert to it
type RowData = JsMap<String, Option<ChronEventDataValue>>;

fn tx(s: &str) -> ChronEventDataValue { ChronEventDataValue::Text(String::from(s)) }
fn num(x: f64) -> ChronEventDataValue { ChronEventDataValue::Num(x) }
fn yes(b: bool) -> ChronEventDataValue { ChronEventDataValue::Bool(b) }

/// an object literal `{ a, b }`, keys in the written order
fn row_data(pairs: Vec<(&str, ChronEventDataValue)>) -> RowData {
    let mut d = JsMap::new();
    for (k, v) in pairs { d.set(String::from(k), Some(v)); }
    d
}
fn data_get<'a>(d: &'a RowData, k: &str) -> Option<&'a ChronEventDataValue> {
    d.get(String::from(k)).and_then(|v| v.as_ref())
}
fn data_set(d: &mut RowData, k: &str, v: ChronEventDataValue) { d.set(String::from(k), Some(v)); }
/// `Number(row.data[k])`; the merged counters are only ever numbers (a text is never written under those keys)
fn data_number(d: &RowData, k: &str) -> f64 {
    match data_get(d, k) {
        Some(ChronEventDataValue::Num(x)) => *x,
        Some(ChronEventDataValue::Bool(b)) => if *b { 1.0 } else { 0.0 },
        _ => f64::NAN,
    }
}
/// `String(row.data[k])`
fn data_string(d: &RowData, k: &str) -> String {
    match data_get(d, k) {
        Some(ChronEventDataValue::Text(s)) => s.clone(),
        Some(ChronEventDataValue::Num(x)) => num_str(*x),
        Some(ChronEventDataValue::Bool(b)) => String::from(if *b { "true" } else { "false" }),
        None => String::from("undefined"),
    }
}
fn data_from_add(d: &JsMap<String, Option<ChronicleAddDataValue>>) -> RowData {
    let mut o = JsMap::new();
    for (k, v) in d.iter() {
        o.set(k, v.as_ref().map(|v| match v {
            ChronicleAddDataValue::Text(s) => ChronEventDataValue::Text(s.clone()),
            ChronicleAddDataValue::Num(x) => ChronEventDataValue::Num(*x),
            ChronicleAddDataValue::Bool(b) => ChronEventDataValue::Bool(*b),
        }));
    }
    o
}
fn data_from_generic(d: &JsMap<String, Option<ChronicleDataValue>>) -> RowData {
    let mut o = JsMap::new();
    for (k, v) in d.iter() {
        o.set(k, v.as_ref().map(|v| match v {
            ChronicleDataValue::Text(s) => ChronEventDataValue::Text(s.clone()),
            ChronicleDataValue::Num(x) => ChronEventDataValue::Num(*x),
            ChronicleDataValue::Bool(b) => ChronEventDataValue::Bool(*b),
        }));
    }
    o
}
fn data_from_n(d: &JsMap<String, Option<ChronicleNDataValue>>) -> RowData {
    let mut o = JsMap::new();
    for (k, v) in d.iter() {
        o.set(k, v.as_ref().map(|v| match v {
            ChronicleNDataValue::Text(s) => ChronEventDataValue::Text(s.clone()),
            ChronicleNDataValue::Num(x) => ChronEventDataValue::Num(*x),
            ChronicleNDataValue::Bool(b) => ChronEventDataValue::Bool(*b),
        }));
    }
    o
}

/// a number in a template literal: `${n}`; −0 prints 0. (The exponent forms past 1e21 and under 1e-6 are not reached: what is printed here are counts.)
fn num_str(x: f64) -> String {
    if x == 0.0 { return String::from("0"); }
    if x != x { return String::from("NaN"); }
    if x == f64::INFINITY { return String::from("Infinity"); }
    if x == f64::NEG_INFINITY { return String::from("-Infinity"); }
    format!("{}", x)
}
/// `String(x).padStart(2, '0')`
fn pad2(x: f64) -> String {
    let s = num_str(x);
    if s.chars().count() < 2 { format!("0{}", s) } else { s }
}
/// `s.replace(/_/g, ' ')`
fn underscores_to_spaces(s: &str) -> String { s.replace('_', " ") }
/// `s.replace(/^the /, '')`
fn without_the(s: &str) -> String { String::from(s.strip_prefix("the ").unwrap_or(s)) }

/// Chronicle.add's caps: true = refused, and counted
fn refused(c: &mut Chronicle, chatter: &JsSet<ChronKind>, kind: ChronKind, data: &RowData) -> bool {
    let n = c.events.len() as f64;
    let load = matches!(data_get(data, "what"), Some(ChronEventDataValue::Text(s)) if s == "load");
    if n >= MAX_EVENTS || (n >= SOFT_EVENTS && (chatter.has(kind) || (kind == ChronKind::Checkpoint && !load))) {
        c.dropped += 1.0;
        return true;
    }
    false
}
/// Chronicle.add's push; the row's place in `events`, which never shrinks in a run: the place is a row's identity here
fn push_row(c: &mut Chronicle, t: f64, kind: ChronKind, room: String, pos: V3, data: RowData) -> usize {
    c.events.push(ChronEvent { t, kind, room, pos: [pos[0], 0.0, pos[2]], data });
    c.events.len() - 1
}
/// `g.chronicle.add(t, kind, pos, data, g.level)`, the row's place in `events`
fn add_row(g: &mut Game, t: f64, kind: ChronKind, pos: V3, data: RowData) -> Option<usize> {
    if refused(&mut g.chronicle, &g.m.chronicle.chatter, kind, &data) { return None; }
    let room = room_of(g, pos);
    Some(push_row(&mut g.chronicle, t, kind, room, pos, data))
}
/// Chronicle.recent, by place. The row's place, so a merge writes the row itself and not a copy.
fn recent_at(c: &Chronicle, kind: ChronKind, key_name: &str, key: &ChronicleRecentKey, t: f64, win: f64) -> Option<usize> {
    let ev = &c.events;
    let mut i = ev.len();
    let mut k = 0;
    while i > 0 && k < 8 {
        i -= 1;
        let e = &ev[i];
        if t - e.t > win { return None; }
        if e.kind == kind {
            let same = match (data_get(&e.data, key_name), key) {
                (Some(ChronEventDataValue::Text(a)), ChronicleRecentKey::Text(b)) => a == b,
                (Some(ChronEventDataValue::Num(a)), ChronicleRecentKey::Num(b)) => a == b,
                _ => false,
            };
            if same { return Some(i); }
        }
        k += 1;
    }
    None
}
/// Chronicle.sample on the record itself
fn sample_on(c: &mut Chronicle, x: f64, z: f64, t: f64) {
    if c.path_n >= PATH_CAP {
        let n = c.path_n as usize;
        let p = &mut c.path;
        let mut w = 0usize;
        let mut r = 0usize;
        while r < n {
            p[w * 3] = p[r * 3];
            p[w * 3 + 1] = p[r * 3 + 1];
            p[w * 3 + 2] = p[r * 3 + 2];
            r += 2;
            w += 1;
        }
        c.path_n = w as f64;
        c.path_every *= 2.0;
    }
    let i = (c.path_n as usize) * 3;
    c.path[i] = x as f32;
    c.path[i + 1] = z as f32;
    c.path[i + 2] = t as f32;
    c.path_n += 1.0;
    c.path_next = t + c.path_every;
}
/// `g.chronicle.counts[count]++`
fn bump_count(n: &mut ChronCounts, count: ChronCountKey) {
    match count {
        ChronCountKey::Heard => n.heard += 1.0,
        ChronCountKey::SeenUp => n.seen_up += 1.0,
        ChronCountKey::Esc1 => n.esc1 += 1.0,
        ChronCountKey::Esc2 => n.esc2 += 1.0,
        ChronCountKey::ShotsNearMen => n.shots_near_men += 1.0,
        ChronCountKey::LightsShot => n.lights_shot += 1.0,
        ChronCountKey::Ocps => n.ocps += 1.0,
        ChronCountKey::Blackouts => n.blackouts += 1.0,
        ChronCountKey::Kicked => n.kicked += 1.0,
        ChronCountKey::Picked => n.picked += 1.0,
        ChronCountKey::Intel => n.intel += 1.0,
        ChronCountKey::Thrown => n.thrown += 1.0,
        ChronCountKey::Grabs => n.grabs += 1.0,
        ChronCountKey::Contacts => n.contacts += 1.0,
        ChronCountKey::Radio => n.radio += 1.0,
        ChronCountKey::CivSeen => n.civ_seen += 1.0,
        ChronCountKey::CivReported => n.civ_reported += 1.0,
        ChronCountKey::CivComplied => n.civ_complied += 1.0,
        ChronCountKey::CivKO => n.civ_ko += 1.0,
        ChronCountKey::CivKilled => n.civ_killed += 1.0,
        ChronCountKey::Devices => n.devices += 1.0,
        ChronCountKey::Hits => n.hits += 1.0,
        ChronCountKey::Saves => n.saves += 1.0,
        ChronCountKey::Loads => n.loads += 1.0,
    }
}
/// `chronicle(g, kind, data, pos)` with the data already a row's
fn live_row(g: &mut Game, kind: ChronKind, data: RowData, pos: Option<V3>) -> Option<ChronEvent> {
    if !crate::mission::mission_live(g) { return None; }
    let t = now_of(g);
    let p = match pos { Some(p) => p, None => pos_of(g) };
    let at = add_row(g, t, kind, p, data)?;
    Some(g.chronicle.events[at].clone())
}
/// `chronicleN(g, kind, count, data, pos)` with the data already a row's
fn live_row_n(g: &mut Game, kind: ChronKind, count: ChronCountKey, data: RowData, pos: Option<V3>) -> Option<ChronEvent> {
    if !crate::mission::mission_live(g) { return None; }
    bump_count(&mut g.chronicle.counts, count);
    let t = now_of(g);
    let p = match pos { Some(p) => p, None => pos_of(g) };
    let at = add_row(g, t, kind, p, data)?;
    Some(g.chronicle.events[at].clone())
}
fn man_callsign(g: &Game, gd: GuardId) -> String { g.gd[gd].callsign.clone() }
fn man_pos(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }

// the skeleton's gap (notes.md, ## Skeleton)
/// `Chronicle.recent` as the skeleton has it hands back a `&mut usize`, and no `usize` in `g` holds a found row's place
fn seam_recent_slot<'g>(g: &'g mut Game, at: usize) -> Option<&'g mut usize> { todo!("needs: chronicle_recent to return Option<usize>: the row's place is worked out, not held") }

// ------------------------------------------------------------ rooms

/// The rooms of the level a run is on, for the debrief's map.   (chronicle.ts roomsOf)
pub fn rooms_of(g: &mut crate::game::Game) -> Vec<crate::chronicle::RoomsOfItem> {
    if crate::dialogue::is_office(g) {
        return rooms().into_iter().map(|r| RoomsOfItem { name: r.name, x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 }).collect();
    }
    match crate::dialogue::level_rects(g) {
        Some(rects) => rects.iter().map(|r| RoomsOfItem { name: without_the(&r.name), x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 }).collect(),
        None => Vec::new(),
    }
}

/// The room a floor point is in, by name; outside the building the side of it.   (chronicle.ts roomOf)
pub fn room_of(g: &mut crate::game::Game, p: V3) -> String {
    if !crate::dialogue::is_office(g) {
        return match crate::dialogue::own_place(g, p) {
            Some(w) => without_the(&w.rect.name),
            None => String::from("building"),
        };
    }
    let (x, z) = (p[0], p[2]);
    let b = crate::floor::building();
    if x >= b.x0 && x < b.x1 && z >= b.z0 && z < b.z1 {
        for r in rooms() {
            if x >= r.x0 && x < r.x1 && z >= r.z0 && z < r.z1 { return r.name; }
        }
        return String::from("corridor");
    }
    if z >= b.z1 { return String::from("south lot"); }
    if x >= b.x1 { return String::from("east side"); }
    if z < b.z0 { return String::from("north side"); }
    String::from("west side")
}

/// Inside the building's footprint.   (chronicle.ts indoors)
pub fn indoors(p: V3) -> bool {
    let b = crate::floor::building();
    p[0] >= b.x0 && p[0] < b.x1 && p[2] >= b.z0 && p[2] < b.z1
}

// ------------------------------------------------------------ the record

/// A row let go of first at the soft cap.   (chronicle.ts isChatter)
pub fn is_chatter(g: &mut crate::game::Game, kind: crate::chronicle::ChronKind, data: &JsMap<String, Option<crate::chronicle::IsChatterDataValue>>) -> bool {
    let load = matches!(data.get(String::from("what")), Some(Some(IsChatterDataValue::Text(s))) if s == "load");
    g.m.chronicle.chatter.has(kind) || (kind == ChronKind::Checkpoint && !load)
}

/// All counters at nothing.   (chronicle.ts newCounts)
pub fn new_counts() -> crate::chronicle::ChronCounts {
    ChronCounts {
        seen_up: 0.0, esc1: 0.0, esc2: 0.0, shots_near_men: 0.0, lights_shot: 0.0, ocps: 0.0, blackouts: 0.0, kicked: 0.0, picked: 0.0, intel: 0.0,
        thrown: 0.0, grabs: 0.0, contacts: 0.0, heard: 0.0, radio: 0.0, civ_seen: 0.0, civ_reported: 0.0, civ_complied: 0.0, civ_ko: 0.0, civ_killed: 0.0,
        devices: 0.0, hits: 0.0, down: false, down_by: String::new(), saves: 0.0, loads: 0.0,
    }
}

/// The core append; `None` when the caps refuse it, else the row's place in `events`.   (chronicle.ts Chronicle.add)
pub fn chronicle_add(g: &mut crate::game::Game, t: f64, kind: crate::chronicle::ChronKind, pos: V3, data: &JsMap<String, Option<crate::chronicle::ChronicleAddDataValue>>) -> Option<usize> {
    add_row(g, t, kind, pos, data_from_add(data))
}

/// The latest row of `kind` with this key within `win` seconds, looking back no further than 8 rows.   (chronicle.ts Chronicle.recent)
pub fn chronicle_recent<'g>(g: &'g mut crate::game::Game, kind: crate::chronicle::ChronKind, key_name: &str, key: &crate::chronicle::ChronicleRecentKey, t: f64, win: f64) -> Option<&'g mut usize> {
    let at = recent_at(&g.chronicle, kind, key_name, key, t, win)?;
    seam_recent_slot(g, at)
}

/// The row a hearing event made, as its place in `events`.   (chronicle.ts Chronicle.heardRow)
pub fn chronicle_heard_row<'g>(g: &'g mut crate::game::Game, e: u32) -> Option<&'g mut usize> {
    g.chronicle.heard_of.get_mut(e)
}

/// (chronicle.ts Chronicle.bindHeard)
pub fn chronicle_bind_heard(g: &mut crate::game::Game, e: u32, row: usize) {
    g.chronicle.heard_of.set(e, row);
}

/// (chronicle.ts Chronicle.heardCounted)
pub fn chronicle_heard_counted(g: &mut crate::game::Game, e: u32) -> bool {
    g.chronicle.heard_no_row.has(e)
}

/// (chronicle.ts Chronicle.markHeardCounted)
pub fn chronicle_mark_heard_counted(g: &mut crate::game::Game, e: u32) {
    g.chronicle.heard_no_row.add(e);
}

/// One path sample at mission time t; thins itself when full.   (chronicle.ts Chronicle.sample)
pub fn chronicle_sample(g: &mut crate::game::Game, x: f64, z: f64, t: f64) {
    sample_on(&mut g.chronicle, x, z, t);
}

// ------------------------------------------------------------ writing

/// The mission clock.   (chronicle.ts nowOf)
pub fn now_of(g: &mut crate::game::Game) -> f64 { g.mission.stats.elapsed }

/// The first player's feet.   (chronicle.ts posOf)
pub fn pos_of(g: &mut crate::game::Game) -> V3 {
    let man = crate::seats::man_here(g);
    g.ch[g.pl[man].ch].pos
}

/// The generic entry: `kind` at `pos` (his feet by default) with `data`.   (chronicle.ts chronicle)
pub fn chronicle(g: &mut crate::game::Game, kind: crate::chronicle::ChronKind, data: Option<&JsMap<String, Option<crate::chronicle::ChronicleDataValue>>>, pos: Option<V3>) -> Option<crate::chronicle::ChronEvent> {
    let data = match data { Some(d) => data_from_generic(d), None => JsMap::new() };
    live_row(g, kind, data, pos)
}

/// The generic entry and one counter by name: the counter moves whenever the run is live, row kept or not.   (chronicle.ts chronicleN)
pub fn chronicle_n(g: &mut crate::game::Game, kind: crate::chronicle::ChronKind, count: crate::chronicle::ChronCountKey, data: Option<&JsMap<String, Option<crate::chronicle::ChronicleNDataValue>>>, pos: Option<V3>) -> Option<crate::chronicle::ChronEvent> {
    let data = match data { Some(d) => data_from_n(d), None => JsMap::new() };
    live_row_n(g, kind, count, data, pos)
}

/// Once a frame from updateMission: the path sample when it is due.   (chronicle.ts tickChronicle)
pub fn tick_chronicle(g: &mut crate::game::Game) {
    let t = now_of(g);
    if t >= g.chronicle.path_next {
        let p = pos_of(g);
        chronicle_sample(g, p[0], p[2], t);
    }
}

/// The stage machine moved; `text` is the row.   (chronicle.ts chronObjective)
pub fn chron_objective(g: &mut crate::game::Game, text: &str, pos: Option<V3>) -> Option<crate::chronicle::ChronEvent> {
    live_row(g, ChronKind::Objective, row_data(vec![("text", tx(text))]), pos)
}

/// An alert man has eyes on him afresh; seen again once they know counts as seenUp.   (chronicle.ts chronSpotted)
pub fn chron_spotted(g: &mut crate::game::Game, by: Option<GuardId>) -> Option<crate::chronicle::ChronEvent> {
    let lvl = g.escalation;
    let who = match by { Some(b) => man_callsign(g, b), None => String::from("a guard") };
    let data = row_data(vec![("by", tx(&who)), ("lvl", num(lvl))]);
    if lvl >= 1.0 { live_row_n(g, ChronKind::Spotted, ChronCountKey::SeenUp, data, None) } else { live_row(g, ChronKind::Spotted, data, None) }
}

/// A man went to 'alert': who, and what did it.   (chronicle.ts chronContact)
pub fn chron_contact(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::chronicle::ChronEvent> {
    let cause = match g.gd[gd].witness.alerted_by { Some(c) => c.as_str(), None => "radio" };
    let data = row_data(vec![("who", tx(&man_callsign(g, gd))), ("cause", tx(cause))]);
    let p = man_pos(g, gd);
    live_row_n(g, ChronKind::Contact, ChronCountKey::Contacts, data, Some(p))
}

/// A noise of his making raised a man: one row per noise however many hear it, a run of one kind merged.   (chronicle.ts chronHeard)
pub fn chron_heard(g: &mut crate::game::Game, gd: GuardId, ev: crate::game::GameEvent, where_: Option<V3>) -> Option<crate::chronicle::ChronEvent> {
    if !crate::mission::mission_live(g) { return None; }
    if let Some(who) = ev.who {
        if !crate::seats::is_man_id(g, who) && who != 0.0 { return None; }
    }
    let t = now_of(g);
    let awareness = g.gd[gd].awareness;
    let react = if awareness >= 0.85 { "alerted" } else if awareness >= 0.3 { "investigates" } else { "pricks up" };
    let callsign = man_callsign(g, gd);
    let key = ev.serial;
    let bound = chronicle_heard_row(g, key).map(|at| *at);
    let at = match bound {
        Some(at) => at,
        None => {
            if chronicle_heard_counted(g, key) { return None; }
            let what = heard_what(ev);
            let merged = recent_at(&g.chronicle, ChronKind::Heard, "what", &ChronicleRecentKey::Text(what.clone()), t, HEARD_MERGE_SECS);
            let got = match merged {
                Some(at) => {
                    let n = js::or(data_number(&g.chronicle.events[at].data, "n"), 1.0) + 1.0;
                    data_set(&mut g.chronicle.events[at].data, "n", num(n));
                    Some(at)
                }
                None => {
                    g.chronicle.counts.heard += 1.0;
                    let data = row_data(vec![("what", tx(&what)), ("by", tx(&callsign)), ("k", num(1.0)), ("n", num(1.0)), ("react", tx(react))]);
                    let r = add_row(g, t, ChronKind::Heard, where_.unwrap_or(ev.pos), data);
                    if r.is_none() { chronicle_mark_heard_counted(g, key); }
                    r
                }
            };
            let at = got?;
            chronicle_bind_heard(g, key, at);
            at
        }
    };
    let ranks = rank();
    let rank_of = |word: &str| ranks.get(String::from(word)).copied().unwrap_or(f64::NAN);
    let row = &mut g.chronicle.events[at];
    let names = data_string(&row.data, "by");
    if !names.split(", ").any(|n| n == callsign) {
        let k = js::or(data_number(&row.data, "k"), 1.0);
        let by = if k < 3.0 { format!("{}, {}", names, callsign) } else { names };
        data_set(&mut row.data, "by", ChronEventDataValue::Text(by));
        data_set(&mut row.data, "k", num(k + 1.0));
    }
    if rank_of(react) > rank_of(&data_string(&row.data, "react")) { data_set(&mut row.data, "react", tx(react)); }
    Some(row.clone())
}
/// What a hearing-model event of his is, in the timeline's words.   (chronicle.ts heardWhat)
pub fn heard_what(e: crate::game::GameEvent) -> String {
    use crate::director::EventKind;
    let level = e.level.unwrap_or(0.0);
    String::from(match e.kind {
        EventKind::Shot => if level >= 1.0 { "gunfire" } else if e.weapon.is_some() { "a muffled shot" } else { "a canister" },
        EventKind::GuardShot => "gunfire",
        EventKind::Kick => "a door kicked in",
        EventKind::Bang => if e.frag == Some(true) { "a grenade" } else { "a stun charge" },
        EventKind::Door => if level >= 0.6 { "a door slammed" } else { "a door" },
        EventKind::Prop => if e.who == Some(0.0) { "a clatter" } else if level >= 0.5 { "a crash of furniture" } else { "a scrape" },
        EventKind::Step => "footsteps",
        EventKind::LightOut => "a light going out",
        EventKind::Glass => "a round into the glass",
        _ => "a noise",
    })
}

/// The alert chase is over without him.   (chronicle.ts chronLost)
pub fn chron_lost(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::chronicle::ChronEvent> {
    let data = row_data(vec![("who", tx(&man_callsign(g, gd)))]);
    let p = man_pos(g, gd);
    live_row(g, ChronKind::Lost, data, Some(p))
}

/// He gives the search up.   (chronicle.ts chronCalm)
pub fn chron_calm(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::chronicle::ChronEvent> {
    let data = row_data(vec![("who", tx(&man_callsign(g, gd)))]);
    let p = man_pos(g, gd);
    live_row(g, ChronKind::Calm, data, Some(p))
}

/// The floor's alarm level moved to `stage`; pinned at the alarm's fix when there is one.   (chronicle.ts chronAlarm)
pub fn chron_alarm(g: &mut crate::game::Game, stage: f64, up: bool, cause: Option<&str>) -> Option<crate::chronicle::ChronEvent> {
    let cause = cause.unwrap_or("");
    let pos = if g.alarm.placed { g.alarm.pos } else { pos_of(g) };
    let data = row_data(vec![("stage", num(stage)), ("up", yes(up)), ("cause", tx(cause))]);
    if up {
        live_row_n(g, ChronKind::Alarm, if stage >= 2.0 { ChronCountKey::Esc2 } else { ChronCountKey::Esc1 }, data, Some(pos))
    } else {
        live_row(g, ChronKind::Alarm, data, Some(pos))
    }
}

/// `/^(\w+: )?radio check — sound off$|^radio check/`
fn is_radio_check(text: &str) -> bool {
    if text.starts_with("radio check") { return true; }
    let Some(colon) = text.find(": ") else { return false; };
    let word = &text[..colon];
    !word.is_empty() && word.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') && &text[colon + 2..] == "radio check — sound off"
}
/// `text.length > 96 ? text.slice(0, 94) + '…' : text`, counted in UTF-16 units as the language does
fn clip_line(text: &str) -> String {
    let units: Vec<u16> = text.encode_utf16().collect();
    if units.len() > 96 { format!("{}…", String::from_utf16_lossy(&units[..94])) } else { String::from(text) }
}
/// A line over the guards' net, the routine radio check excepted.   (chronicle.ts chronRadio)
pub fn chron_radio(g: &mut crate::game::Game, gd: GuardId, text: &str) -> Option<crate::chronicle::ChronEvent> {
    if is_radio_check(text) { return None; }
    let data = row_data(vec![("who", tx(&man_callsign(g, gd))), ("text", ChronEventDataValue::Text(clip_line(text)))]);
    let p = man_pos(g, gd);
    live_row_n(g, ChronKind::Radio, ChronCountKey::Radio, data, Some(p))
}

/// His round: which weapon, what it went into, a living man near where it stopped; a burst into one sort of thing is one row.   (chronicle.ts chronShot)
pub fn chron_shot(g: &mut crate::game::Game, weapon: &str, hit_guard: Option<GuardId>, hit_p: V3) -> Option<crate::chronicle::ChronEvent> {
    if !crate::mission::mission_live(g) { return None; }
    let t = now_of(g);
    if hit_guard.is_some() { g.chronicle.counts.shots_near_men += 1.0; return None; }
    let mut near = false;
    let mut nearest = String::new();
    {
        let mut bd = SHOT_NEAR_M;
        for i in 0..g.guards.len() {
            let gd = g.guards[i];
            if g.gd[gd].state == crate::game::GuardState::Dead { continue; }
            let c = g.gd[gd].ch;
            let at = match g.ch[c].bones.get(crate::character::Bone::Chest) { Some(b) => *b, None => g.ch[c].pos };
            let d = v3::dist(at, hit_p);
            if d < bd { bd = d; near = true; nearest = g.gd[gd].callsign.clone(); }
        }
    }
    if near { g.chronicle.counts.shots_near_men += 1.0; }
    let into = if near { format!("past {}", nearest) } else { String::from("geometry") };
    let prev = recent_at(&g.chronicle, ChronKind::Shot, "into", &ChronicleRecentKey::Text(into.clone()), t, SHOT_MERGE_SECS);
    if let Some(at) = prev {
        let same = matches!(data_get(&g.chronicle.events[at].data, "weapon"), Some(ChronEventDataValue::Text(w)) if w == weapon);
        if same {
            let n = js::or(data_number(&g.chronicle.events[at].data, "n"), 1.0) + 1.0;
            data_set(&mut g.chronicle.events[at].data, "n", num(n));
            return Some(g.chronicle.events[at].clone());
        }
    }
    let p = pos_of(g);
    let data = row_data(vec![("weapon", tx(weapon)), ("into", tx(&into)), ("near", yes(near)), ("n", num(1.0))]);
    let at = add_row(g, t, ChronKind::Shot, p, data)?;
    Some(g.chronicle.events[at].clone())
}

/// A fixture shot out.   (chronicle.ts chronLightShot)
pub fn chron_light_shot(g: &mut crate::game::Game, tg: FixtureId) -> Option<crate::chronicle::ChronEvent> {
    let row = &g.tab.sim_lightshome_fixture[tg];
    let name = fixture_name(&FixtureNameTg { name: row.name.clone(), group: row.group.clone() });
    let pos = row.pos;
    live_row_n(g, ChronKind::LightShot, ChronCountKey::LightsShot, row_data(vec![("name", tx(&name))]), Some(pos))
}

/// The OCP put something out.   (chronicle.ts chronOcp)
pub fn chron_ocp(g: &mut crate::game::Game, what: &str, pos: V3) -> Option<crate::chronicle::ChronEvent> {
    live_row_n(g, ChronKind::Ocp, ChronCountKey::Ocps, row_data(vec![("what", tx(what))]), Some(pos))
}

/// The mains went.   (chronicle.ts chronBlackout)
pub fn chron_blackout(g: &mut crate::game::Game, cause: &str) -> Option<crate::chronicle::ChronEvent> {
    if crate::mission::mission_live(g) && cause == "ocp" { g.chronicle.counts.ocps += 1.0; }
    let pos = g.level.breaker.pos;
    live_row_n(g, ChronKind::Blackout, ChronCountKey::Blackouts, row_data(vec![("cause", tx(cause))]), Some(pos))
}

/// `s.replace(/_?\d+$/, '')`
fn without_trailing_number(s: &str) -> &str {
    let t = s.trim_end_matches(|c: char| c.is_ascii_digit());
    if t.len() == s.len() { return s; }
    t.strip_suffix('_').unwrap_or(t)
}
/// A fixture's name in words.   (chronicle.ts fixtureName)
pub fn fixture_name(tg: &crate::chronicle::FixtureNameTg) -> String {
    let n = tg.name.as_str();
    if n.starts_with("panel_") { return format!("a {} ceiling panel", tg.group); }
    if n.starts_with("monitor") { return String::from("a monitor"); }
    if n.starts_with("beacon") { return String::from("an emergency beacon"); }
    if n.starts_with("exit_sign") { return String::from("an exit sign"); }
    if n.starts_with("vending") { return String::from("a vending machine"); }
    format!("the {}", underscores_to_spaces(without_trailing_number(n)))
}

/// A canister is away.   (chronicle.ts chronThrown)
pub fn chron_thrown(g: &mut crate::game::Game, kind: &str) -> Option<crate::chronicle::ChronEvent> {
    live_row_n(g, ChronKind::Thrown, ChronCountKey::Thrown, row_data(vec![("kind", tx(kind))]), None)
}

/// A man taken into the arm.   (chronicle.ts chronGrab)
pub fn chron_grab(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::chronicle::ChronEvent> {
    let data = row_data(vec![("who", tx(&man_callsign(g, gd)))]);
    let p = man_pos(g, gd);
    live_row_n(g, ChronKind::Grab, ChronCountKey::Grabs, data, Some(p))
}

/// One exchange: who, the stage, the useful thing's kind if he gave one.   (chronicle.ts chronInterrogated)
pub fn chron_interrogated(g: &mut crate::game::Game, gd: GuardId, kind: Option<&str>, stage: f64) -> Option<crate::chronicle::ChronEvent> {
    if !crate::mission::mission_live(g) { return None; }
    let callsign = man_callsign(g, gd);
    let civ = g.gd[gd].civ;
    if civ.is_none() { g.chronicle.questioned.add(callsign.clone()); }
    if matches!(kind, Some(k) if !k.is_empty()) { g.chronicle.counts.intel += 1.0; }
    let mut pairs = vec![("who", tx(&callsign)), ("gave", tx(kind.unwrap_or(""))), ("stage", num(stage))];
    if let Some(c) = civ {
        pairs.push(("civilian", yes(true)));
        pairs.push(("she", tx(&g.cv[c].she)));
        pairs.push(("her", tx(&g.cv[c].her)));
    }
    let p = man_pos(g, gd);
    live_row(g, ChronKind::Interrogated, row_data(pairs), Some(p))
}

/// A fresh key off a questioned man's belt.   (chronicle.ts chronKey)
pub fn chron_key(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::chronicle::ChronEvent> {
    let data = row_data(vec![("who", tx(&man_callsign(g, gd)))]);
    let p = man_pos(g, gd);
    live_row(g, ChronKind::KeyTaken, data, Some(p))
}

/// A man put down by the player's hand: shot dead ('kill') or out cold ('ko').   (chronicle.ts chronDown)
pub fn chron_down(g: &mut crate::game::Game, gd: GuardId, how: crate::game::DownKind, line: Option<&str>) -> Option<crate::chronicle::ChronEvent> {
    use crate::game::DownKind;
    let l = line.unwrap_or("");
    let callsign = man_callsign(g, gd);
    let p = man_pos(g, gd);
    if how == DownKind::Shot {
        let weapon = if l.starts_with("knife") {
            String::from("knife")
        } else if l.ends_with("at the charge when it went") {
            String::from("a wall charge")
        } else {
            let man = crate::seats::man_here(g);
            match crate::game::player_gun_in_hand(g, man) {
                Some(gun) => {
                    let row = g.m.weapons.weapons.get(g.fa[gun].def).unwrap();
                    crate::gear::plain_short(&crate::gear::PlainShortDef { id: String::from(row.id.as_str()), short: row.short.clone() })
                }
                None => String::from("a round"),
            }
        };
        return live_row(g, ChronKind::Kill, row_data(vec![("who", tx(&callsign)), ("weapon", ChronEventDataValue::Text(weapon))]), Some(p));
    }
    let method = if how == DownKind::Wounded {
        "wounded"
    } else if how == DownKind::Choked {
        "choke"
    } else if l.starts_with("ring") {
        "ring"
    } else if l.contains("whip") {
        "whip"
    } else if l.ends_with("behind the wall when it went") {
        "charge"
    } else {
        "takedown"
    };
    live_row(g, ChronKind::Ko, row_data(vec![("who", tx(&callsign)), ("method", tx(method))]), Some(p))
}

/// A body discovered: whose, by whom, found breathing or not.   (chronicle.ts chronBodyFound)
pub fn chron_body_found(g: &mut crate::game::Game, body: GuardId, by: GuardId) -> Option<crate::chronicle::ChronEvent> {
    let c = g.gd[body].ch;
    let h = match g.ch[c].bones.get(crate::character::Bone::Hips) { Some(b) => *b, None => g.ch[c].pos };
    let breathing = match g.gd[body].down_kind { Some(k) => k != crate::game::DownKind::Shot, None => false };
    let mut pairs = vec![("who", tx(&man_callsign(g, body))), ("by", tx(&man_callsign(g, by))), ("breathing", yes(breathing))];
    if g.gd[body].wound.is_some() { pairs.push(("wounded", yes(true))); }
    live_row(g, ChronKind::BodyFound, row_data(pairs), Some([h[0], 0.0, h[2]]))
}

/// A round found him: 'down' the once, 'hit' while the vest still has something left.   (chronicle.ts chronHit)
pub fn chron_hit(g: &mut crate::game::Game, by: Option<&str>) -> Option<crate::chronicle::ChronEvent> {
    if g.chronicle.counts.down || !crate::mission::mission_live(g) { return None; }
    let by = by.unwrap_or("");
    let pl = g.player;
    if g.pl[pl].down {
        g.chronicle.counts.down = true;
        g.chronicle.counts.down_by = String::from(by);
        return live_row(g, ChronKind::Down, row_data(vec![("by", tx(by))]), None);
    }
    let left = g.pl[pl].hits_left;
    live_row_n(g, ChronKind::Hit, ChronCountKey::Hits, row_data(vec![("by", tx(by)), ("left", num(left))]), None)
}

/// A door kicked in or a lock picked.   (chronicle.ts chronDoor)
pub fn chron_door(g: &mut crate::game::Game, kind: crate::chronicle::ChronDoorKind, d: DoorId) -> Option<crate::chronicle::ChronEvent> {
    let (row, count) = match kind {
        ChronDoorKind::DoorKicked => (ChronKind::DoorKicked, ChronCountKey::Kicked),
        ChronDoorKind::LockPicked => (ChronKind::LockPicked, ChronCountKey::Picked),
    };
    let name = underscores_to_spaces(&g.dr[d].def.name);
    let pos = g.dr[d].frame_centre;
    live_row_n(g, row, count, row_data(vec![("door", ChronEventDataValue::Text(name))]), Some(pos))
}

/// A checkpoint written or restored; a save is chatter to the caps, a load stays.   (chronicle.ts chronCheckpoint)
pub fn chron_checkpoint(g: &mut crate::game::Game, what: crate::chronicle::ChronCheckpointWhat, label: Option<&str>) -> Option<crate::chronicle::ChronEvent> {
    let count = if what == ChronCheckpointWhat::Load { ChronCountKey::Loads } else { ChronCountKey::Saves };
    live_row_n(g, ChronKind::Checkpoint, count, row_data(vec![("what", tx(what.as_str())), ("label", tx(label.unwrap_or("")))]), None)
}

// ------------------------------------------------------------ reading

/// Everything the card and the score read, in one flat record.   (chronicle.ts runCounts)
pub fn run_counts(g: &mut crate::game::Game) -> crate::chronicle::RunCounts {
    use crate::game::MissionStage;
    let st = g.mission.stage;
    let s = &g.mission.stats;
    let c = &g.chronicle;
    let n = &c.counts;
    let mut roster = 0.0;
    for i in 0..g.guards.len() {
        if g.gd[g.guards[i]].unit.is_none() { roster += 1.0; }
    }
    RunCounts {
        elapsed: s.elapsed, stage: String::from(st.as_str()), done: st == MissionStage::Done, failed: st == MissionStage::Failed, sandbox: s.sandbox, blackout: s.blackout,
        seen: s.spotted, seen_up: n.seen_up, alarms: s.alerts, esc1: n.esc1, esc2: n.esc2, bodies: s.bodies,
        kills: s.kills, kos: s.knockouts, shots: s.shots, shots_near_men: n.shots_near_men, lights_shot: n.lights_shot, ocps: n.ocps,
        kicked: n.kicked, picked: n.picked, keys: s.keys, intel: n.intel, questioned: c.questioned.len() as f64, roster,
        hits: n.hits, down: n.down || (g.pl[g.player].down && st != MissionStage::Done), loads: n.loads, thrown: n.thrown, grabs: n.grabs, dropped: c.dropped, devices: n.devices,
        civ_seen: n.civ_seen, civ_reported: n.civ_reported, civ_complied: n.civ_complied, civ_ko: n.civ_ko, civ_killed: n.civ_killed,
    }
}

/// A blank record: what the probes' synthetic runs start from.   (chronicle.ts blankCounts)
pub fn blank_counts() -> crate::chronicle::RunCounts {
    RunCounts {
        elapsed: 0.0, stage: String::from("infiltrate"), done: false, failed: false, sandbox: false, blackout: false, seen: 0.0, seen_up: 0.0, alarms: 0.0, esc1: 0.0, esc2: 0.0, bodies: 0.0,
        kills: 0.0, kos: 0.0, shots: 0.0, shots_near_men: 0.0, lights_shot: 0.0, ocps: 0.0, kicked: 0.0, picked: 0.0, keys: 0.0, intel: 0.0, questioned: 0.0, roster: 3.0,
        hits: 0.0, down: false, loads: 0.0, thrown: 0.0, grabs: 0.0, dropped: 0.0, devices: 0.0, civ_seen: 0.0, civ_reported: 0.0, civ_complied: 0.0, civ_ko: 0.0, civ_killed: 0.0,
    }
}

/// `n === 1 ? a : b`
fn plural<'a>(n: f64, one: &'a str, many: &'a str) -> &'a str { if n == 1.0 { one } else { many } }
fn take(lines: &mut Vec<ScoreLine>, pts: f64, label: String) {
    if pts > 0.0 { lines.push(ScoreLine { label, pts }); }
}
fn score_of(s: &crate::knobs::ScoreK, c: &RunCounts) -> StealthScore {
    let mut lines: Vec<ScoreLine> = Vec::new();
    let up = if js::truthy(c.seen_up) { format!(" ({} with the floor already up)", num_str(c.seen_up)) } else { String::new() };
    take(&mut lines, c.seen * s.seen + c.seen_up * s.seen_up, format!("seen {}{}", times(c.seen), up));
    take(&mut lines, c.alarms * s.alarm, format!("{} alarm{} raised", num_str(c.alarms), plural(c.alarms, "", "s")));
    let locked = if js::truthy(c.esc2) { format!(" ({} to lockdown)", num_str(c.esc2)) } else { String::new() };
    take(&mut lines, c.esc1 * s.heightened + c.esc2 * s.lockdown, format!("the floor went up {}{}", times(c.esc1 + c.esc2), locked));
    take(&mut lines, c.bodies * s.body, format!("{} bod{} found", num_str(c.bodies), plural(c.bodies, "y", "ies")));
    take(&mut lines, c.kills * s.kill, format!("{} killed", num_str(c.kills)));
    take(&mut lines, c.kos * s.ko, format!("{} knocked out", num_str(c.kos)));
    take(&mut lines, m::min(s.shot_near_cap, c.shots_near_men * s.shot_near), format!("{} round{} at or near men", num_str(c.shots_near_men), plural(c.shots_near_men, "", "s")));
    take(&mut lines, m::min(s.light_cap, c.lights_shot * s.light), format!("{} light{} shot out", num_str(c.lights_shot), plural(c.lights_shot, "", "s")));
    take(&mut lines, m::min(s.device_cap, c.devices * s.device), format!("called in by security {}", times(c.devices)));
    take(&mut lines, c.civ_reported * s.civ_reported, format!("a civilian told them {}", times(c.civ_reported)));
    take(&mut lines, c.civ_ko * s.civ_ko, format!("{} civilian{} knocked out", num_str(c.civ_ko), plural(c.civ_ko, "", "s")));
    take(&mut lines, c.civ_killed * s.civ_killed, format!("{} CIVILIAN{} KILLED", num_str(c.civ_killed), plural(c.civ_killed, "", "S")));
    take(&mut lines, c.hits * s.hit, format!("hit {}", times(c.hits)));
    take(&mut lines, if c.down { s.down } else { 0.0 }, String::from("went down"));
    take(&mut lines, c.loads * s.load, format!("{} checkpoint load{}", num_str(c.loads), plural(c.loads, "", "s")));
    let over = m::floor(m::max(0.0, c.elapsed - s.par_secs) / 60.0);
    take(&mut lines, m::min(s.time_cap, over * s.per_min), format!("{} min over par", num_str(over)));
    let mut off = 0.0;
    for l in lines.iter() { off = off + l.pts; }
    js::sort_by(&mut lines, |a, b| b.pts - a.pts);
    StealthScore { score: v3::clamp(m::round(100.0 - off), 0.0, 100.0), lines }
}
/// 0..100 and the itemised deductions that made it, largest first.   (chronicle.ts stealthScore)
pub fn stealth_score(g: &mut crate::game::Game, c_2: &crate::chronicle::RunCounts) -> crate::chronicle::StealthScore {
    score_of(&g.k.score, c_2)
}

/// (chronicle.ts times)
pub fn times(n: f64) -> String {
    if n == 1.0 { String::from("once") } else if n == 2.0 { String::from("twice") } else { format!("{} times", num_str(n)) }
}

/// The rating word for a score.   (chronicle.ts ratingOf)
pub fn rating_of(score: f64) -> crate::chronicle::RatingOf {
    let table = bands();
    for b in table.iter() {
        if score >= b.min { return RatingOf { word: b.word.clone(), blurb: b.blurb.clone() }; }
    }
    let last = &table[table.len() - 1];
    RatingOf { word: last.word.clone(), blurb: last.blurb.clone() }
}

fn badge(id: &str, name: &str, on: bool, hint: String) -> Badge {
    Badge { id: String::from(id), name: String::from(name), on, hint }
}
fn badges_for(b: &crate::knobs::BadgeK, c: &RunCounts) -> Vec<Badge> {
    vec![
        badge("unseen", "never seen", c.seen == 0.0, String::from("no alert man ever had eyes on you")),
        badge("quiet", "no alarms", c.alarms == 0.0 && c.esc1 + c.esc2 == 0.0, String::from("nobody went to alert, the floor never went up")),
        badge("nokills", "no kills", c.kills == 0.0, String::from("nobody shot dead (a man out cold is not a kill)")),
        badge("pacifist", "pacifist", c.shots_near_men == 0.0 && c.kills == 0.0, format!("not one round into or within {} m of a man", num_str(SHOT_NEAR_M))),
        badge("lightsout", "lights out", c.lights_shot >= b.lights_out_n, format!("{}+ fixtures shot out", num_str(b.lights_out_n))),
        badge("talkative", "talkative", c.roster > 0.0 && c.questioned >= c.roster, String::from("every man on the floor questioned")),
        badge("unseenStaff", "unseen by civilians", c.civ_seen == 0.0 && c.civ_ko == 0.0 && c.civ_killed == 0.0, String::from("no cleaner or late worker ever laid eyes on you (or a hand of yours on them)")),
        badge("locksmith", "locksmith", c.picked >= 1.0 && c.kicked == 0.0, String::from("a lock picked and none kicked in")),
        badge("speedrun", "speedrun", c.done && c.elapsed < b.speedrun_secs, format!("out inside {}", fmt_mmss(b.speedrun_secs))),
    ]
}
/// The run's badges, each a plain yes / no off the counts.   (chronicle.ts badgesOf)
pub fn badges_of(g: &mut crate::game::Game, c_2: &crate::chronicle::RunCounts) -> Vec<crate::chronicle::Badge> {
    badges_for(&g.k.badge, c_2)
}

/// mm:ss off seconds.   (chronicle.ts fmtMMSS)
pub fn fmt_mmss(sec: f64) -> String {
    let s = m::max(0.0, m::floor(sec));
    format!("{}:{}", pad2(m::floor(s / 60.0)), pad2(js::rem(s, 60.0)))
}

/// The whole debrief in one record. mission.ts reads its `grade` for the end line.   (chronicle.ts debriefOf)
pub fn debrief_of(g: &mut crate::game::Game) -> crate::chronicle::Debrief {
    let counts = run_counts(g);
    let StealthScore { score, lines } = stealth_score(g, &counts);
    let r = rating_of(score);
    let badges = badges_of(g, &counts);
    let (state, line_state) = if counts.done {
        (DebriefState::Done, SummaryLineState::Done)
    } else if counts.failed || counts.down {
        (DebriefState::Down, SummaryLineState::Down)
    } else {
        (DebriefState::Sofar, SummaryLineState::Sofar)
    };
    let scored = !crate::verdict::loud_row(g);
    let summary = summary_line(&counts, score, &r.word, &badges, line_state, Some(scored));
    let grade = if scored { format!("{} {}", r.word, num_str(score)) } else { String::new() };
    Debrief { counts, score, lines, rating: r.word, blurb: r.blurb, badges, summary, state, scored, grade }
}

/// The one line the 'copy summary' button puts on the clipboard.   (chronicle.ts summaryLine)
pub fn summary_line(c_2: &crate::chronicle::RunCounts, score: f64, word: &str, badges: &[crate::chronicle::Badge], state: crate::chronicle::SummaryLineState, scored: Option<bool>) -> String {
    let c = c_2;
    let scored = scored.unwrap_or(true);
    let mut parts: Vec<String> = Vec::new();
    if scored { parts.push(format!("{} {}", word, num_str(score))); }
    parts.push(fmt_mmss(c.elapsed));
    parts.push(format!("seen {}", num_str(c.seen)));
    parts.push(format!("alarms {}", num_str(c.alarms)));
    parts.push(format!("bodies found {}", num_str(c.bodies)));
    parts.push(format!("KOs {}", num_str(c.kos)));
    parts.push(format!("kills {}", num_str(c.kills)));
    parts.push(format!("shots {}", num_str(c.shots)));
    if js::truthy(c.civ_seen) || js::truthy(c.civ_reported) || js::truthy(c.civ_ko) || js::truthy(c.civ_killed) {
        let ko = if js::truthy(c.civ_ko) { format!(" · KO {}", num_str(c.civ_ko)) } else { String::new() };
        let killed = if js::truthy(c.civ_killed) { format!(" · KILLED {}", num_str(c.civ_killed)) } else { String::new() };
        parts.push(format!("civilians: saw you {} · told them {}{}{}", num_str(c.civ_seen), num_str(c.civ_reported), ko, killed));
    }
    if js::truthy(c.ocps) { parts.push(format!("OCP {}", num_str(c.ocps))); }
    if js::truthy(c.kicked) || js::truthy(c.picked) { parts.push(format!("doors kicked {} / picked {}", num_str(c.kicked), num_str(c.picked))); }
    if js::truthy(c.keys) { parts.push(format!("keys {}", num_str(c.keys))); }
    if js::truthy(c.intel) { parts.push(format!("intel {}", num_str(c.intel))); }
    if js::truthy(c.loads) { parts.push(format!("loads {}", num_str(c.loads))); }
    if scored {
        let on: Vec<&str> = badges.iter().filter(|b| b.on).map(|b| b.name.as_str()).collect();
        if !on.is_empty() { parts.push(on.join(", ")); }
    }
    if state != SummaryLineState::Done { parts.push(String::from(if state == SummaryLineState::Down { "went down" } else { "run so far" })); }
    if c.sandbox { parts.push(String::from("sandbox flags on")); }
    format!("twopointsix — {}", parts.join(" · "))
}

/// One timeline row in words.   (chronicle.ts fmtEvent)
pub fn fmt_event(g: &mut crate::game::Game, e: &crate::chronicle::ChronEvent) -> String {
    format!("{} · {} · {}", fmt_mmss(e.t), e.room, event_text(g, e))
}

/// The name a door row's door was WRITTEN, asked when the row is read; `None`: the row reads its id's words.   (chronicle.ts doorWritten)
pub fn door_written(g: &mut crate::game::Game, door: &str) -> Option<String> {
    let d = match crate::doors::doors_by_name(g, door) {
        Some(d) => Some(d),
        None => crate::doors::doors_by_name(g, &door.replace(' ', "_")),
    };
    match d {
        Some(d) => crate::doornames::door_written_name(g, d),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(c: &RunCounts, s: &crate::knobs::ScoreK) -> String {
        let r = score_of(s, c);
        let mut out = format!("{}", num_str(r.score));
        for l in r.lines.iter() { out = format!("{}|{}={}", out, l.label, num_str(l.pts)); }
        out
    }

    #[test]
    fn words() {
        assert_eq!(times(1.0), "once");
        assert_eq!(times(2.0), "twice");
        assert_eq!(times(7.0), "7 times");
        assert_eq!(fmt_mmss(0.0), "00:00");
        assert_eq!(fmt_mmss(-3.0), "00:00");
        assert_eq!(fmt_mmss(240.0), "04:00");
        assert_eq!(fmt_mmss(692.9), "11:32");
        assert_eq!(fmt_mmss(6000.0), "100:00");
        assert_eq!(rating_of(94.0).word, "GHOST");
        assert_eq!(rating_of(70.0).word, "SHADOW");
        assert_eq!(rating_of(39.0).word, "ASSAULT");
        assert_eq!(rating_of(-1.0).word, "ASSAULT");
    }

    #[test]
    fn fixture_names() {
        let n = |name: &str, group: &str| fixture_name(&FixtureNameTg { name: String::from(name), group: String::from(group) });
        assert_eq!(n("panel_corridor_12", "corridor"), "a corridor ceiling panel");
        assert_eq!(n("monitor3", "x"), "a monitor");
        assert_eq!(n("beacon", "x"), "an emergency beacon");
        assert_eq!(n("exit_sign_2", "x"), "an exit sign");
        assert_eq!(n("vending_1", "x"), "a vending machine");
        assert_eq!(n("desk_lamp", "x"), "the desk lamp");
        assert_eq!(n("desk_lamp_07", "x"), "the desk lamp");
        assert_eq!(n("desk_lamp7", "x"), "the desk lamp");
        assert_eq!(n("a__1", "x"), "the a ");
        assert_eq!(n("1", "x"), "the ");
    }

    #[test]
    fn radio_checks_and_clipping() {
        assert!(is_radio_check("radio check — sound off"));
        assert!(is_radio_check("radio check, anyone"));
        assert!(is_radio_check("Bravo: radio check — sound off"));
        assert!(!is_radio_check("Bravo: radio check"));
        assert!(!is_radio_check("Bravo two: radio check — sound off"));
        assert!(!is_radio_check(": radio check — sound off"));
        assert!(!is_radio_check("contact, radio check"));
        let ninety_six: String = core::iter::repeat('—').take(96).collect();
        assert_eq!(clip_line(&ninety_six), ninety_six);
        let long: String = core::iter::repeat('—').take(97).collect();
        let clipped = clip_line(&long);
        assert_eq!(clipped.encode_utf16().count(), 95);
        assert!(clipped.ends_with('…'));
    }

    #[test]
    fn heard_words() {
        use crate::director::EventKind;
        let mut e = crate::game::GameEvent::zeroed();
        e.kind = EventKind::Shot;
        assert_eq!(heard_what(e), "a canister");
        e.level = Some(1.0);
        assert_eq!(heard_what(e), "gunfire");
        e = crate::game::GameEvent::zeroed();
        e.kind = EventKind::Prop;
        e.who = Some(0.0);
        e.level = Some(0.9);
        assert_eq!(heard_what(e), "a clatter");
        e.who = None;
        assert_eq!(heard_what(e), "a crash of furniture");
        e.kind = EventKind::Whistle;
        assert_eq!(heard_what(e), "a noise");
    }

    #[test]
    fn caps_and_recent() {
        let chatter = ChronicleMod::blank().chatter;
        let mut c = Chronicle::blank();
        let d = |k: &str| row_data(vec![("what", tx(k))]);
        for i in 0..520 { push_row(&mut c, i as f64, ChronKind::Objective, String::new(), [1.0, 2.0, 3.0], JsMap::new()); }
        assert!(refused(&mut c, &chatter, ChronKind::Heard, &JsMap::new()));
        assert!(refused(&mut c, &chatter, ChronKind::Checkpoint, &d("save")));
        assert!(!refused(&mut c, &chatter, ChronKind::Checkpoint, &d("load")));
        assert!(!refused(&mut c, &chatter, ChronKind::Alarm, &JsMap::new()));
        assert_eq!(c.dropped, 2.0);
        assert_eq!(c.events[0].pos, [1.0, 0.0, 3.0]);
        for i in 0..180 { push_row(&mut c, 520.0 + i as f64, ChronKind::Objective, String::new(), [0.0; 3], JsMap::new()); }
        assert!(refused(&mut c, &chatter, ChronKind::Alarm, &JsMap::new()));

        let mut c = Chronicle::blank();
        push_row(&mut c, 10.0, ChronKind::Shot, String::new(), [0.0; 3], row_data(vec![("into", tx("geometry"))]));
        let key = ChronicleRecentKey::Text(String::from("geometry"));
        assert_eq!(recent_at(&c, ChronKind::Shot, "into", &key, 12.5, 2.5), Some(0));
        assert_eq!(recent_at(&c, ChronKind::Shot, "into", &key, 12.6, 2.5), None);
        for _ in 0..7 { push_row(&mut c, 10.0, ChronKind::Radio, String::new(), [0.0; 3], JsMap::new()); }
        assert_eq!(recent_at(&c, ChronKind::Shot, "into", &key, 10.0, 2.5), Some(0));
        push_row(&mut c, 10.0, ChronKind::Radio, String::new(), [0.0; 3], JsMap::new());
        assert_eq!(recent_at(&c, ChronKind::Shot, "into", &key, 10.0, 2.5), None);
    }

    #[test]
    fn the_path_thins_itself() {
        let mut c = Chronicle::blank();
        for i in 0..4000 { let x = i as f64; sample_on(&mut c, x, x + 0.1, x * 0.5); }
        assert_eq!(c.path_n, 4000.0);
        assert_eq!(c.path_every, 0.5);
        sample_on(&mut c, 9.0, 8.0, 2000.0);
        assert_eq!(c.path_n, 2001.0);
        assert_eq!(c.path_every, 1.0);
        assert_eq!(c.path_next, 2001.0);
        assert_eq!(c.path[3], 2.0_f32);
        assert_eq!(c.path[4].to_bits(), (2.1_f64 as f32).to_bits());
        assert_eq!(c.path[2000 * 3], 9.0_f32);
    }

    #[test]
    fn the_scores_of_the_typescript() {
        // (the expected strings are what the TypeScript's own stealthScore, summaryLine and badgesOf printed under bun)
        let s = crate::knobs::page_score();
        let b = crate::knobs::page_badge();
        let mut c = blank_counts();
        c.seen = 2.0; c.seen_up = 1.0; c.alarms = 1.0; c.esc1 = 1.0; c.esc2 = 1.0; c.bodies = 1.0; c.kills = 1.0; c.kos = 3.0; c.shots_near_men = 9.0; c.lights_shot = 1.0; c.devices = 5.0;
        c.hits = 1.0; c.down = true; c.loads = 2.0; c.elapsed = 1500.0; c.civ_ko = 1.0;
        assert_eq!(line(&c, &s), "0|seen twice (1 with the floor already up)=28|went down=25|the floor went up twice (1 to lockdown)=13|9 rounds at or near men=12|1 killed=10|1 civilian knocked out=10|called in by security 5 times=8|1 alarm raised=6|1 body found=6|2 checkpoint loads=6|5 min over par=5|hit once=4|3 knocked out=3|1 light shot out=1");

        let a = blank_counts();
        assert_eq!(score_of(&s, &a).score, 100.0);
        assert_eq!(summary_line(&a, 100.0, "GHOST", &badges_for(&b, &a), SummaryLineState::Sofar, None), "twopointsix — GHOST 100 · 00:00 · seen 0 · alarms 0 · bodies found 0 · KOs 0 · kills 0 · shots 0 · never seen, no alarms, no kills, pacifist, unseen by civilians · run so far");

        let mut c = blank_counts();
        c.seen = 1.0; c.alarms = 2.0; c.kos = 1.0; c.civ_seen = 1.0; c.civ_reported = 1.0; c.civ_killed = 2.0; c.ocps = 1.0; c.picked = 1.0; c.keys = 1.0; c.intel = 1.0; c.loads = 1.0; c.sandbox = true; c.done = true; c.elapsed = 692.9;
        assert_eq!(line(&c, &s), "0|2 CIVILIANS KILLED=80|a civilian told them once=15|seen once=12|2 alarms raised=12|1 checkpoint load=3|1 knocked out=1");
        let badges = badges_for(&b, &c);
        assert_eq!(summary_line(&c, 0.0, "ASSAULT", &badges, SummaryLineState::Done, Some(false)), "twopointsix — 11:32 · seen 1 · alarms 2 · bodies found 0 · KOs 1 · kills 0 · shots 0 · civilians: saw you 1 · told them 1 · KILLED 2 · OCP 1 · doors kicked 0 / picked 1 · keys 1 · intel 1 · loads 1 · sandbox flags on");
        assert_eq!(summary_line(&c, 3.0, "ASSAULT", &badges, SummaryLineState::Done, None), "twopointsix — ASSAULT 3 · 11:32 · seen 1 · alarms 2 · bodies found 0 · KOs 1 · kills 0 · shots 0 · civilians: saw you 1 · told them 1 · KILLED 2 · OCP 1 · doors kicked 0 / picked 1 · keys 1 · intel 1 · loads 1 · no kills, pacifist, locksmith · sandbox flags on");
        let shown: Vec<String> = badges.iter().map(|x| format!("{}:{}:{}", x.id, x.on, x.hint)).collect();
        assert_eq!(shown.join("|"), "unseen:false:no alert man ever had eyes on you|quiet:false:nobody went to alert, the floor never went up|nokills:true:nobody shot dead (a man out cold is not a kill)|pacifist:true:not one round into or within 3 m of a man|lightsout:false:3+ fixtures shot out|talkative:false:every man on the floor questioned|unseenStaff:false:no cleaner or late worker ever laid eyes on you (or a hand of yours on them)|locksmith:true:a lock picked and none kicked in|speedrun:false:out inside 04:00");
    }
}
