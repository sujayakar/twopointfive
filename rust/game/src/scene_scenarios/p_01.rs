//! src/scene/scenarios.ts lines 1 to 618
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A squad's barricade doors: the file's own list if it names one (an empty list means never), else every door of its rooms
/// except one shared by two of its own rooms.   (scenarios.ts squadBarricade)
pub fn squad_barricade(q: &crate::scene_scenarios::SquadData, rooms: &[crate::scene_scenarios::StructureRoom]) -> Vec<String> {
    if let Some(barricade) = &q.barricade {
        return barricade.clone();
    }
    let mut ids: Vec<&String> = Vec::new();
    for r in rooms {
        let is_mine = match &q.rooms {
            Some(mine) => mine.contains(&r.id),
            None => false,
        };
        if !is_mine {
            continue;
        }
        for d in &r.doors {
            ids.push(&d.id);
        }
    }
    let mut out: Vec<String> = Vec::new();
    for &id in &ids {
        if out.contains(id) {
            continue;
        }
        if ids.iter().filter(|&&x| x == id).count() < 2 {
            out.push(id.clone());
        }
    }
    out
}

/// A row of the lesson group, the tutorial's missions.   (scenarios.ts lessonGroup)
pub fn lesson_group(g_2: Option<&str>) -> bool {
    g_2 == Some("tutorial")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_scenarios::{SquadData, StructureDoor, StructureRoom};

    fn room(id: &str, doors: &[&str]) -> StructureRoom {
        let mut r = StructureRoom::zeroed();
        r.id = String::from(id);
        r.doors = doors
            .iter()
            .map(|d| StructureDoor { id: String::from(*d), from: String::new() })
            .collect();
        r
    }

    fn squad(rooms: Option<&[&str]>, barricade: Option<&[&str]>) -> SquadData {
        let strings = |list: &[&str]| list.iter().map(|s| String::from(*s)).collect::<Vec<String>>();
        let mut q = SquadData::zeroed();
        q.rooms = rooms.map(|l| strings(l));
        q.barricade = barricade.map(|l| strings(l));
        q
    }

    #[test]
    fn the_files_own_list_wins_even_empty() {
        let rooms = [room("a", &["d1"])];
        assert_eq!(squad_barricade(&squad(Some(&["a"]), Some(&["x"])), &rooms), vec![String::from("x")]);
        assert!(squad_barricade(&squad(Some(&["a"]), Some(&[])), &rooms).is_empty());
    }

    #[test]
    fn a_door_two_of_its_rooms_share_is_left_out_in_first_seen_order() {
        let rooms = [room("a", &["d2", "shared", "d1"]), room("other", &["d9"]), room("b", &["shared", "d3"])];
        let got = squad_barricade(&squad(Some(&["b", "a"]), None), &rooms);
        assert_eq!(got, vec![String::from("d2"), String::from("d1"), String::from("d3")]);
    }

    #[test]
    fn a_squad_with_no_rooms_has_no_doors() {
        let rooms = [room("a", &["d1"])];
        assert!(squad_barricade(&squad(None, None), &rooms).is_empty());
    }

    #[test]
    fn only_the_tutorial_is_the_lesson_group() {
        assert!(lesson_group(Some("tutorial")));
        assert!(!lesson_group(Some("raids")));
        assert!(!lesson_group(None));
    }
}
