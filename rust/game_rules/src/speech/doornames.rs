pub use ::game_types::doornames::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use alloc::{format, string::String, vec, vec::Vec};

/// COMPASS: a whole compass word or letter.
static COMPASS: &[&str] = &["n", "s", "e", "w", "ne", "nw", "se", "sw", "north", "south", "east", "west"];
/// JOIN: two letters and a word all the same.
static JOIN: &[&str] = &["to"];
/// ODD: id words that name no door when said.
static ODD: &[&str] = &["off", "his"];
static BEARINGS: &[&str] = &["north", "south", "east", "west", "northern", "southern", "eastern", "western"];

/// The language's `\s` and `trim()` set (Rust's White_Space is another set: it has U+0085 and lacks U+FEFF).
fn js_space(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

/// The test of one token of an id.
fn token_is_shorthand(t: &str) -> bool {
    COMPASS.contains(&t)
        || ODD.contains(&t)
        || (!(!t.is_empty() && t.bytes().all(|b| b.is_ascii_digit()))
            && !JOIN.contains(&t)
            && (t.encode_utf16().count() <= 2
                || t.bytes().any(|b| b.is_ascii_digit())
                || !t.bytes().any(|b| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u' | b'y'))))
}

fn id_is_shorthand(id: &str) -> bool {
    id.split('_').any(token_is_shorthand)
}

/// `/\b(?:north|south|east|west)(?:ern)?\b/`: a whole run of ASCII word characters is one of the bearings.
fn has_bearing(s: &str) -> bool {
    s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).any(|w| BEARINGS.contains(&w))
}

/// True = this door id is its author's shorthand, not words to say.
pub fn shorthand_id(g: &crate::game::Game, id_2: &str) -> bool {
    id_is_shorthand(id_2)
}

/// A name without its leading article.
fn bare(s: &str) -> String {
    let b = s.as_bytes();
    if b.len() >= 3 && b[..3].eq_ignore_ascii_case(b"the") {
        let rest = &s[3..];
        let after = rest.trim_start_matches(js_space);
        if after.len() < rest.len() {
            return String::from(after);
        }
    }
    String::from(s)
}

fn names_of(level: &crate::scene_level::Level, ids: &[String], written: Option<&JsMap<String, String>>) -> JsMap<String, String> {
    let mut out: JsMap<String, String> = JsMap::new();
    let ordinals = ordinals();
    let Some(scenarios) = level.scenarios.as_ref() else { return out };
    let rooms: &[crate::scene_scenarios::StructureRoom] = match scenarios.room_clear.as_ref() {
        Some(rc) => &rc.rooms,
        None => &[],
    };
    if rooms.is_empty() || scenarios.lesson.is_some() {
        return out;
    }
    // (id, name), in the file's order
    let mut made: Vec<(String, String)> = Vec::new();
    for id in ids {
        if written.is_some_and(|wr| wr.has(id.clone())) || !id_is_shorthand(id) {
            continue; // (a door its level has named is not made a name)
        }
        let of: Vec<&crate::scene_scenarios::StructureRoom> = rooms.iter().filter(|r| r.doors.iter().any(|d| d.id == *id)).collect();
        if of.is_empty() {
            continue;
        }
        let r = of[0];
        let a = bare(&r.name);
        if of.len() > 1 {
            made.push((id.clone(), format!("{} to {} door", a, bare(&of[1].name))));
            continue;
        }
        let from = bare(&r.doors.iter().find(|d| d.id == *id).unwrap().from);
        let plain = r.doors.len() == 1 || r.doors.iter().all(|d| bare(&d.from) == from) || has_bearing(&from);
        let name = if plain {
            format!("{} door", a)
        } else if from == "outside" {
            format!("outer {} door", a)
        } else if from.ends_with(" side") {
            format!("{} {} door", from, a)
        } else if from.chars().any(|c| js_space(c) || c.is_ascii_digit()) {
            format!("{} door from {}", a, from)
        } else {
            format!("{}-side {} door", from, a)
        };
        made.push((id.clone(), name));
    }
    // two doors that still read the same are told apart in the file's order; so is a name another door's own id already reads as
    let mut taken: JsSet<String> = JsSet::new();
    for id in ids {
        if made.iter().any(|m| m.0 == *id) {
            continue;
        }
        let name = match written.and_then(|wr| wr.get(id.clone())) {
            Some(n) => n.clone(),
            None => format!("{} door", id.replace('_', " ")),
        };
        taken.add(name);
    }
    for (i, m) in made.iter().enumerate() {
        let same = made.iter().filter(|o| o.1 == m.1).count();
        // same.indexOf(m): the rows of this name before this row (rows, not ids: `ids` may repeat one)
        let index = made[..i].iter().filter(|o| o.1 == m.1).count();
        let is_taken = taken.has(m.1.clone());
        let name = if same > 1 || is_taken {
            let at = core::cmp::min(index + if is_taken { 1 } else { 0 }, ordinals.len() - 1);
            format!("{} {}", ordinals[at], m.1)
        } else {
            m.1.clone()
        };
        out.set(m.0.clone(), name);
    }
    out
}

/// The said name of every shorthand door the level's structure lists, by door id.
fn door_names_of(g: &crate::game::Game, ids: &[String], written: Option<&JsMap<String, String>>) -> JsMap<String, String> {
    names_of(&g.level, ids, written)
}

fn authored_of(level: &crate::scene_level::Level, ids: &[String]) -> JsMap<String, String> {
    let mut out: JsMap<String, String> = JsMap::new();
    let Some(rows) = level.scenarios.as_ref().and_then(|s| s.door_names.as_ref()) else { return out };
    for r in rows {
        let n = bare(r.name.trim_matches(js_space));
        if !n.is_empty() && ids.contains(&r.door) && !out.has(r.door.clone()) {
            out.set(r.door.clone(), n);
        }
    }
    out
}

/// The names the level gives its own doors (scenarios.doorNames); a door's first usable row wins.
fn authored_door_names(g: &crate::game::Game, ids: &[String]) -> JsMap<String, String> {
    authored_of(&g.level, ids)
}

/// Game construction: hang each named door's names on its Door.
pub fn name_doors(g: &mut crate::game::Game, office: bool, doors: &[DoorId]) {
    let ids: Vec<String> = doors.iter().map(|&d| g.dr[d].def.name.clone()).collect();
    let written = authored_door_names(g, &ids);
    let none: JsMap<String, String> = JsMap::new();
    let names = if office { none.clone() } else { door_names_of(g, &ids, None) };
    let up = if office {
        none
    } else if written.len() > 0 {
        door_names_of(g, &ids, Some(&written))
    } else {
        names.clone()
    };
    for (&d, id) in doors.iter().zip(ids.iter()) {
        if let Some(n) = names.get(id.clone()) {
            g.m.doornames.said.set(d, n.clone());
        }
        if let Some(u) = up.get(id.clone()) {
            g.m.doornames.said_up.set(d, u.clone());
        }
        if let Some(wr) = written.get(id.clone()) {
            g.m.doornames.written.set(d, wr.clone());
        }
    }
}

/// The name its LEVEL wrote for this door.
pub fn door_written_name(g: &crate::game::Game, d: DoorId) -> Option<String> {
    g.m.doornames.written.get(d).cloned()
}

/// The name nameDoors gave this door: its level's written one first.
pub fn door_said_name(g: &crate::game::Game, d: DoorId) -> Option<String> {
    match g.m.doornames.written.get(d) {
        Some(n) => Some(n.clone()),
        None => g.m.doornames.said_up.get(d).cloned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use crate::scene_level::Level;
    use crate::scene_scenarios::{LessonData, LevelScenarios, LevelScenariosDoorNamesItem, StructureData, StructureDoor, StructureRoom};

    fn room(name: &str, doors: &[(&str, &str)]) -> StructureRoom {
        let mut r = StructureRoom::default();
        r.name = name.to_string();
        r.doors = doors
            .iter()
            .map(|(id, from)| {
                let mut d = StructureDoor::default();
                d.id = id.to_string();
                d.from = from.to_string();
                d
            })
            .collect();
        r
    }

    fn row(door: &str, name: &str) -> LevelScenariosDoorNamesItem {
        LevelScenariosDoorNamesItem { door: door.to_string(), name: name.to_string() }
    }

    fn level(rooms: Vec<StructureRoom>, written: Vec<LevelScenariosDoorNamesItem>) -> Level {
        let mut rc = StructureData::default();
        rc.rooms = rooms;
        let mut s = LevelScenarios::default();
        s.room_clear = Some(rc);
        s.door_names = Some(written);
        let mut l = Level::default();
        l.scenarios = Some(s);
        l
    }

    fn ids(list: &[&str]) -> Vec<String> { list.iter().map(|s| s.to_string()).collect() }

    /// What the doors of these rooms are said by.
    fn made(rooms: Vec<StructureRoom>, list: &[&str]) -> Vec<String> {
        let m = names_of(&level(rooms, Vec::new()), &ids(list), None);
        list.iter().map(|id| m.get(id.to_string()).cloned().unwrap_or_default()).collect()
    }

    /// A FAILURE MEANS: a door of a room with more than one way in is said so that a player cannot tell which.
    #[test]
    fn a_shorthand_door_is_said_by_its_room_and_what_lies_behind_it() {
        let doors = [("w_n", "the hall"), ("b_s", "outside"), ("c_d", "bunks 1"), ("x_1", "annex side")];
        assert_eq!(made(vec![room("The mess", &doors)], &["w_n", "b_s", "c_d", "x_1"]), ["hall-side mess door", "outer mess door", "mess door from bunks 1", "annex side mess door"]);
    }

    /// A FAILURE MEANS: a door between two rooms is said as if it were only one room's.
    #[test]
    fn a_door_two_rooms_list_names_both() {
        let rooms = vec![room("mess", &[("m_c", "comms"), ("q7", "hall")]), room("comms room", &[("m_c", "mess")])];
        assert_eq!(made(rooms, &["m_c"]), ["mess to comms room door"]);
    }

    /// A FAILURE MEANS: "hall-side ward door" on a ward with no other door, or two doors of a room said alike.
    #[test]
    fn a_rooms_only_way_is_just_its_door_and_doors_that_read_alike_are_told_apart() {
        assert_eq!(made(vec![room("ward", &[("hd", "hall")])], &["hd"]), ["ward door"]);
        // all to the same place, in the file's order, and the ordinals stop at the last there is
        let dock = [("d1", "hall"), ("d2", "hall"), ("d3", "hall"), ("d4", "hall"), ("d5", "hall"), ("d6", "hall")];
        assert_eq!(made(vec![room("dock", &dock)], &["d1", "d2", "d3", "d4", "d5", "d6"]), ["first dock door", "second dock door", "third dock door", "fourth dock door", "fifth dock door", "fifth dock door"]);
        // a name another door's own id already reads as
        assert_eq!(made(vec![room("ward", &[("hd", "hall")])], &["hd", "ward"]), ["second ward door", ""]);
    }

    /// A FAILURE MEANS: "north corridor-side mess door" on a HUD that has no compass; and a word that only starts like a bearing is lost.
    #[test]
    fn a_bearing_behind_a_door_is_left_out_of_its_name() {
        assert_eq!(made(vec![room("mess", &[("m_n", "north corridor"), ("b_e", "the western wing")])], &["m_n", "b_e"]), ["first mess door", "second mess door"]);
        assert_eq!(made(vec![room("pantry", &[("p1", "northerner"), ("p2", "yard")])], &["p1", "p2"]), ["northerner-side pantry door", "yard-side pantry door"]);
        assert!(has_bearing("north corridor") && has_bearing("the western wing"));
        assert!(!has_bearing("northerner"));
    }

    /// A FAILURE MEANS: a door whose id is its author's shorthand is read out ("w n door"), or a door whose id is words loses them.
    #[test]
    fn an_id_of_words_is_said_and_an_id_of_letters_is_not() {
        for id in ["radio_room", "fire_exit", "back_1", "kitchen_to_hall", "bunks"] { assert!(!id_is_shorthand(id), "{id}"); }
        for id in ["w_n", "n4a", "mgr", "hd", "records_west", "off"] { assert!(id_is_shorthand(id), "{id}"); }
    }

    /// A FAILURE MEANS: "the mess door" is said "the the mess door", or a word that starts like the article loses its head.
    #[test]
    fn a_name_loses_a_leading_article_only() {
        assert_eq!(bare("The  mess"), "mess");
        assert_eq!(bare("theatre"), "theatre");
        assert_eq!(bare("the"), "the");
    }

    /// A FAILURE MEANS: a level's author writes a door's name and the game says something else.
    #[test]
    fn a_doors_first_usable_written_name_wins_and_none_is_made_for_it() {
        let l = level(vec![room("vault", &[("v1", "hall"), ("v2", "yard")])], vec![row("v1", "  "), row("v1", "  the strong door "), row("v1", "other"), row("nope", "x")]);
        let list = ids(&["v1", "v2"]);
        let written = authored_of(&l, &list);
        assert_eq!(written.iter().map(|(k, v)| (k, v.clone())).collect::<Vec<_>>(), [("v1".to_string(), "strong door".to_string())]);
        let m = names_of(&l, &list, Some(&written));
        assert_eq!((m.get("v1".to_string()), m.get("v2".to_string()).map(String::as_str)), (None, Some("yard-side vault door")));
    }

    /// A FAILURE MEANS: a lesson's doors, which it speaks of itself, get names of the mission kind.
    #[test]
    fn no_names_are_made_for_a_lesson_or_a_level_without_rooms() {
        let mut lesson = level(vec![room("ward", &[("hd", "hall")])], Vec::new());
        assert!(!names_of(&lesson, &ids(&["hd"]), None).is_empty());
        lesson.scenarios.as_mut().unwrap().lesson = Some(LessonData::default().into());
        assert!(names_of(&lesson, &ids(&["hd"]), None).is_empty());
        assert!(names_of(&Level::default(), &ids(&["hd"]), None).is_empty());
    }

    /// A FAILURE MEANS: a door carries no name, or the wrong name, once the level is stood; the office says only what its level wrote.
    #[test]
    fn a_door_says_its_written_name_first_and_the_office_says_only_that() {
        let mut g = Game::blank();
        g.level = level(vec![room("vault", &[("v1", "hall"), ("v2", "yard")]), room("ward", &[("hd", "hall")])], vec![row("v1", "the strong door")]);
        let mut door = |name: &str| {
            let mut d = crate::doors::Door::blank();
            d.def.name = name.to_string();
            g.dr.push(d)
        };
        let (v1, hd, radio) = (door("v1"), door("hd"), door("radio_room"));
        name_doors(&mut g, false, &[v1, hd, radio]);
        assert_eq!(door_written_name(&mut g, v1).as_deref(), Some("strong door"));
        assert_eq!(door_said_name(&mut g, v1).as_deref(), Some("strong door"));
        assert_eq!(door_written_name(&mut g, hd), None);
        assert_eq!(door_said_name(&mut g, hd).as_deref(), Some("ward door"));
        assert_eq!(door_said_name(&mut g, radio), None);
        let mut office = g.clone();
        office.m.doornames = DoornamesMod::blank();
        name_doors(&mut office, true, &[v1, hd]);
        assert_eq!(door_said_name(&mut office, v1).as_deref(), Some("strong door"));
        assert_eq!(door_said_name(&mut office, hd), None);
    }
}
