//! src/game/doornames.ts lines 1 to 88
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// What `BEARING` (the skeleton keeps it as the pattern's text) matches as a whole word.
static BEARINGS: &[&str] = &["north", "south", "east", "west", "northern", "southern", "eastern", "western"];

/// The language's `\s` and `trim()` set (Rust's White_Space is another set: it has U+0085 and lacks U+FEFF).
fn js_space(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

/// The test of one token of an id, over the module's word sets. (doornames.ts shorthand)
fn token_is_shorthand(w: &DoornamesMod, t: &str) -> bool {
    w.compass.has(String::from(t))
        || w.odd.has(String::from(t))
        || (!(!t.is_empty() && t.bytes().all(|b| b.is_ascii_digit()))
            && !w.join.has(String::from(t))
            && (t.encode_utf16().count() <= 2
                || t.bytes().any(|b| b.is_ascii_digit())
                || !t.bytes().any(|b| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u' | b'y'))))
}

fn id_is_shorthand(w: &DoornamesMod, id: &str) -> bool {
    id.split('_').any(|t| token_is_shorthand(w, t))
}

/// `/\b(?:north|south|east|west)(?:ern)?\b/`: a whole run of ASCII word characters is one of the bearings.
fn has_bearing(s: &str) -> bool {
    s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).any(|w| BEARINGS.contains(&w))
}

/// A token of an id that is no word: compass, one or two letters, letters with a number, no vowel. (doornames.ts shorthand)
pub fn shorthand(g: &mut crate::game::Game, t: &str) -> bool {
    token_is_shorthand(&g.m.doornames, t)
}

/// True = this door id is its author's shorthand, not words to say. (doornames.ts shorthandId)
pub fn shorthand_id(g: &mut crate::game::Game, id_2: &str) -> bool {
    id_is_shorthand(&g.m.doornames, id_2)
}

/// A name without its leading article. (doornames.ts bare)
pub fn bare(s: &str) -> String {
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

fn names_of(w: &DoornamesMod, level: &crate::scene_level::Level, ids: &[String], written: Option<&JsMap<String, String>>) -> JsMap<String, String> {
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
        if written.is_some_and(|wr| wr.has(id.clone())) || !id_is_shorthand(w, id) {
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

/// The said name of every shorthand door the level's structure lists, by door id. (doornames.ts doorNamesOf)
pub fn door_names_of(g: &mut crate::game::Game, ids: &[String], written: Option<&JsMap<String, String>>) -> JsMap<String, String> {
    names_of(&g.m.doornames, &g.level, ids, written)
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

/// The names the level gives its own doors (scenarios.doorNames); a door's first usable row wins. (doornames.ts authoredDoorNames)
pub fn authored_door_names(g: &mut crate::game::Game, ids: &[String]) -> JsMap<String, String> {
    authored_of(&g.level, ids)
}

/// Game construction: hang each named door's names on its Door. (doornames.ts nameDoors)
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

/// The name its LEVEL wrote for this door. (doornames.ts doorWrittenName)
pub fn door_written_name(g: &mut crate::game::Game, d: DoorId) -> Option<String> {
    g.m.doornames.written.get(d).cloned()
}

/// The name nameDoors gave this door: its level's written one first. (doornames.ts doorSaidName)
pub fn door_said_name(g: &mut crate::game::Game, d: DoorId) -> Option<String> {
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
        let mut r = StructureRoom::zeroed();
        r.name = name.to_string();
        r.doors = doors
            .iter()
            .map(|(id, from)| {
                let mut d = StructureDoor::zeroed();
                d.id = id.to_string();
                d.from = from.to_string();
                d
            })
            .collect();
        r
    }

    /// the word sets as a game starts with them
    fn words() -> DoornamesMod {
        DoornamesMod::blank()
    }

    fn row(door: &str, name: &str) -> LevelScenariosDoorNamesItem {
        LevelScenariosDoorNamesItem { door: door.to_string(), name: name.to_string() }
    }

    fn level() -> Level {
        let mut rc = StructureData::zeroed();
        rc.rooms = vec![
            room("The  mess", &[("w_n", "the hall"), ("b_s", "outside"), ("x_1", "east side"), ("c_d", "bunks 1"), ("m_c", "north corridor"), ("q7", "the northern wing")]),
            room("comms room", &[("m_c", "mess")]),
            room("ward", &[("hd", "hall")]),
            room("dock", &[("d1", "hall"), ("d2", "hall")]),
            room("dock", &[("d3", "yard"), ("d4", "yard"), ("d5", "yard"), ("d6", "yard"), ("d7", "yard"), ("d8", "yard")]),
            room("vault", &[("v1", "hall"), ("v2", "yard")]),
            room("pantry", &[("p1", "northerner"), ("p2", "yard")]),
        ];
        let mut s = LevelScenarios::zeroed();
        s.room_clear = Some(rc);
        s.door_names = Some(vec![row("v1", "  the strong door "), row("v1", "other"), row("nope", "x"), row("v2", " the "), row("n9", "THE\u{a0}night door")]);
        let mut l = Level::zeroed();
        l.scenarios = Some(s);
        l
    }

    fn ids() -> Vec<String> {
        ["w_n", "b_s", "x_1", "c_d", "m_c", "q7", "hd", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "v1", "v2", "p1", "p2", "ward", "radio_room", "kitchen_to_hall", "back_1", "n9", "records_west"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn pairs(m: &JsMap<String, String>) -> Vec<(String, String)> {
        m.iter().map(|(k, v)| (k, v.clone())).collect()
    }

    fn want(rows: &[(&str, &str)]) -> Vec<(String, String)> {
        rows.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    // Every expected value below is what the TypeScript printed under bun for the same level.
    const COMMON_HEAD: &[(&str, &str)] = &[
        ("w_n", "hall-side mess door"),
        ("b_s", "outer mess door"),
        ("x_1", "first mess door"),
        ("c_d", "mess door from bunks 1"),
        ("m_c", "mess to comms room door"),
        ("q7", "second mess door"),
        ("hd", "second ward door"),
        ("d1", "first dock door"),
        ("d2", "second dock door"),
        ("d3", "third dock door"),
        ("d4", "fourth dock door"),
        ("d5", "fifth dock door"),
        ("d6", "fifth dock door"),
        ("d7", "fifth dock door"),
        ("d8", "fifth dock door"),
    ];
    const VAULT: &[(&str, &str)] = &[("v1", "hall-side vault door"), ("v2", "yard-side vault door")];
    const PANTRY: &[(&str, &str)] = &[("p1", "northerner-side pantry door"), ("p2", "yard-side pantry door")];

    #[test]
    fn made_names_match_the_typescript() {
        let expect: Vec<(String, String)> = [COMMON_HEAD, VAULT, PANTRY].iter().flat_map(|r| want(r)).collect();
        assert_eq!(pairs(&names_of(&words(), &level(), &ids(), None)), expect);
    }

    #[test]
    fn written_names_match_the_typescript() {
        let written = authored_of(&level(), &ids());
        assert_eq!(pairs(&written), want(&[("v1", "strong door"), ("v2", "the"), ("n9", "night door")]));
        let expect: Vec<(String, String)> = [COMMON_HEAD, PANTRY].iter().flat_map(|r| want(r)).collect();
        assert_eq!(pairs(&names_of(&words(), &level(), &ids(), Some(&written))), expect);
    }

    #[test]
    fn no_names_for_a_lesson_or_a_level_without_structure() {
        let mut lesson = level();
        lesson.scenarios.as_mut().unwrap().lesson = Some(LessonData::zeroed());
        assert!(names_of(&words(), &lesson, &ids(), None).is_empty());
        assert!(names_of(&words(), &Level::zeroed(), &ids(), None).is_empty());
    }

    #[test]
    fn shorthand_ids_match_the_typescript() {
        let rows = [
            ("w_n", true), ("n4a", true), ("s34", true), ("mgr", true), ("hd", true), ("radio_room", false), ("fire_exit", false), ("bunks", false),
            ("back_1", false), ("kitchen_to_hall", false), ("records_west", true), ("off", true), ("his", true), ("a__b", true), ("", true), ("12", false),
            ("é", true), ("ééé", true), ("Abc", true), ("BCD", true),
        ];
        for (id, want) in rows {
            assert_eq!(id_is_shorthand(&words(), id), want, "{id}");
        }
    }

    #[test]
    fn bearing_is_a_whole_word() {
        assert!(has_bearing("north corridor") && has_bearing("the western wing") && has_bearing("é east"));
        assert!(!has_bearing("northerner") && !has_bearing("north_wing") && !has_bearing("North"));
    }

    #[test]
    fn bare_drops_a_leading_article_only() {
        assert_eq!(bare("The  mess"), "mess");
        assert_eq!(bare("THE\u{a0}x"), "x");
        assert_eq!(bare("theatre"), "theatre");
        assert_eq!(bare("the"), "the");
        assert_eq!(bare("the\u{85}x"), "the\u{85}x");
        assert_eq!(bare("é the x"), "é the x");
    }

    /// A door by its ID: nameDoors over rows of `g.dr`; every expected value is what the TypeScript printed under bun for the same doors.
    #[test]
    fn names_hang_on_the_door_id() {
        let mut g = Game::blank();
        g.level = level();
        let mut door = |name: &str| {
            let mut d = crate::doors::Door::blank();
            d.def.name = name.to_string();
            g.dr.push(d)
        };
        let (v1, hd, radio, d1) = (door("v1"), door("hd"), door("radio_room"), door("d1"));
        let d2 = door("d2");
        name_doors(&mut g, false, &[v1, hd, radio, d1, d2]);
        assert_eq!(door_written_name(&mut g, v1).as_deref(), Some("strong door"));
        assert_eq!(door_said_name(&mut g, v1).as_deref(), Some("strong door"));
        assert_eq!(door_written_name(&mut g, hd), None);
        assert_eq!(door_said_name(&mut g, hd).as_deref(), Some("ward door"));
        assert_eq!(door_said_name(&mut g, radio), None);
        assert_eq!(door_said_name(&mut g, d1).as_deref(), Some("first dock door"));
        assert_eq!(door_said_name(&mut g, d2).as_deref(), Some("second dock door"));
        // the office: only what its level wrote
        let mut g2 = g.clone();
        g2.m.doornames = DoornamesMod::blank();
        name_doors(&mut g2, true, &[v1, d1]);
        assert_eq!(door_said_name(&mut g2, v1).as_deref(), Some("strong door"));
        assert_eq!(door_said_name(&mut g2, d1), None);
    }
}
