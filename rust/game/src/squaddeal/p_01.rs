//! src/game/squaddeal.ts lines 1 to 65
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The men dealt into a room.   (squaddeal.ts squadDeal `n`)
fn n(count: &JsMap<String, f64>, r: &crate::scene_scenarios::StructureRoom) -> f64 {
    count.get(r.id.clone()).copied().unwrap_or(0.0)
}

/// A room's middle, as [x, z].   (squaddeal.ts squadDeal `mid`)
fn mid(r: &crate::scene_scenarios::StructureRoom) -> [f64; 2] {
    [(r.bounds.x0 + r.bounds.x1) / 2.0, (r.bounds.z0 + r.bounds.z1) / 2.0]
}

/// How far the rooms' middles reach along one axis.   (squaddeal.ts squadDeal `span`)
fn span(rooms: &[crate::scene_scenarios::StructureRoom], ax: usize) -> f64 {
    let v: Vec<f64> = rooms.iter().map(|r| mid(r)[ax]).collect();
    m::max_n(&v) - m::min_n(&v)
}

/// The structure as the squads are staged on it: as it is when its file has a squads block (an empty one too), on a lesson, or with fewer than two men
/// dealt; else a copy with a block cut from its rooms (`rooms[i]` = the room defender i was dealt into).   (squaddeal.ts squadDeal)
pub fn squad_deal(g: &mut crate::game::Game, s: Option<&crate::scene_scenarios::StructureData>, rooms: &[String], lesson: Option<bool>) -> Option<crate::scene_scenarios::StructureData> {
    deal_with(&g.k.squaddeal, s, rooms, lesson)
}

/// The rule itself: it reaches nothing of the game but its own knob row.   (squaddeal.ts squadDeal)
fn deal_with(knobs: &crate::knobs::SquaddealK, s: Option<&crate::scene_scenarios::StructureData>, rooms: &[String], lesson: Option<bool>) -> Option<crate::scene_scenarios::StructureData> {
    let lesson = lesson.unwrap_or(false);
    let Some(s) = s else { return None };
    if s.squads.is_some() || lesson || s.rooms.is_empty() {
        return Some(s.clone());
    }
    let mut count: JsMap<String, f64> = JsMap::new();
    for id in rooms {
        let c = count.get(id.clone()).copied().unwrap_or(0.0);
        count.set(id.clone(), c + 1.0);
    }
    let men = s.rooms.iter().fold(0.0, |a, r| a + n(&count, r));
    let held = s.rooms.iter().filter(|r| n(&count, r) > 0.0).count() as f64;
    if men < 2.0 {
        return Some(s.clone());
    }
    let wanted = if men <= knobs.one { 1.0 } else if men <= knobs.two { 2.0 } else { 3.0 };
    let k = m::min_n(&[wanted, held, knobs.names.len() as f64]);
    let ax = if span(&s.rooms, 0) >= span(&s.rooms, 1) { 0 } else { 1 };
    let mut line: Vec<&crate::scene_scenarios::StructureRoom> = s.rooms.iter().collect();
    js::sort_by(&mut line, |a, b| {
        let (ma, mb) = (mid(a), mid(b));
        let along = js::or(ma[ax] - mb[ax], ma[1 - ax] - mb[1 - ax]);
        js::or(along, if a.id < b.id { -1.0 } else { 1.0 })
    });
    // the cuts: after the room where the men counted so far come nearest q/k of them, each run keeping a man and leaving one for every run after it
    let mut cuts: Vec<usize> = Vec::new();
    let mut from = 0usize;
    let mut q = 1.0;
    while q < k {
        let mut best: Option<usize> = None;
        let mut best_d = m::INF;
        let mut sum = line[..from].iter().fold(0.0, |a, r| a + n(&count, r));
        let mut own = 0.0;
        let mut after = line[from..].iter().filter(|r| n(&count, r) > 0.0).count() as f64;
        for i in from..line.len() - 1 {
            let here = n(&count, line[i]);
            sum += here;
            own += here;
            if here > 0.0 {
                after -= 1.0; // (`after`: the rooms with a man past this one)
            }
            if own < 1.0 {
                continue;
            }
            if after < k - q {
                break;
            }
            let d = m::abs(sum - men * q / k);
            if d < best_d {
                best_d = d;
                best = Some(i);
            }
        }
        let Some(best) = best else { break };
        cuts.push(best + 1);
        from = best + 1;
        q += 1.0;
    }
    let mut squads: Vec<crate::scene_scenarios::SquadData> = Vec::new();
    let mut i0 = 0usize;
    cuts.push(line.len());
    for &end in &cuts {
        squads.push(crate::scene_scenarios::SquadData {
            id: alloc::format!("dealt{}", squads.len() + 1),
            name: knobs.names.get(squads.len()).cloned(),
            rooms: Some(line[i0..end].iter().map(|r| r.id.clone()).collect()),
            ..Default::default()
        });
        i0 = end;
    }
    let mut out = s.clone();
    out.squads = Some(squads);
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_scenarios::{Rect, StructureData, StructureRoom};

    fn room(id: &str, x0: f64, x1: f64, z0: f64, z1: f64) -> StructureRoom {
        StructureRoom { id: String::from(id), name: String::from(id), bounds: Rect { x0, x1, z0, z1 }, ..StructureRoom::zeroed() }
    }
    fn house(rooms: Vec<StructureRoom>) -> StructureData {
        StructureData { name: String::from("x"), rooms, ..StructureData::zeroed() }
    }
    fn line() -> Vec<StructureRoom> {
        vec![room("c", 20.0, 30.0, 0.0, 4.0), room("a", 0.0, 10.0, 0.0, 4.0), room("e", 40.0, 50.0, 0.0, 4.0), room("b", 10.0, 20.0, 0.0, 4.0), room("d", 30.0, 40.0, 0.0, 4.0)]
    }
    fn page() -> crate::knobs::SquaddealK {
        crate::knobs::SquaddealK { on: true, one: 5.0, two: 8.0, names: vec![String::from("Alpha"), String::from("Bravo"), String::from("Charlie")] }
    }
    /// the block as "id name: rooms", one a squad, or None for no block
    fn deal(rooms: Vec<StructureRoom>, men: &[&str]) -> Option<Vec<String>> {
        let men: Vec<String> = men.iter().map(|m| String::from(*m)).collect();
        let out = deal_with(&page(), Some(&house(rooms)), &men, None).unwrap();
        out.squads.map(|q| q.iter().map(|s| alloc::format!("{} {}: {}", s.id, s.name.as_deref().unwrap_or("-"), s.rooms.as_ref().unwrap().join(","))).collect())
    }
    fn block(want: &[&str]) -> Option<Vec<String>> {
        Some(want.iter().map(|w| String::from(*w)).collect())
    }

    // Every expected block below is what src/game/squaddeal.ts itself printed under bun for the same arguments.
    #[test]
    fn cuts_match_the_typescript() {
        assert_eq!(deal(line(), &["a", "a", "b", "c", "d", "d", "e"]), block(&["dealt1 Alpha: a,b", "dealt2 Bravo: c,d,e"]));
        assert_eq!(deal(line(), &["a", "a", "a", "b", "c", "c", "d", "e", "e", "e"]), block(&["dealt1 Alpha: a", "dealt2 Bravo: b,c,d", "dealt3 Charlie: e"]));
        assert_eq!(deal(line(), &["a", "a", "a", "a", "a", "a", "a", "a", "a", "a"]), block(&["dealt1 Alpha: a,b,c,d,e"]));
        assert_eq!(deal(line(), &["a", "b", "c", "d", "e"]), block(&["dealt1 Alpha: a,b,c,d,e"]));
        assert_eq!(deal(line(), &["a"]), None);
        assert_eq!(deal(line(), &["a", "a", "zz", "e", "e", "e", "e", "e", "e", "b"]), block(&["dealt1 Alpha: a", "dealt2 Bravo: b", "dealt3 Charlie: c,d,e"]));
    }
    #[test]
    fn ties_fall_to_the_other_axis_then_the_id() {
        let rooms = vec![room("p", 0.0, 2.0, 10.0, 20.0), room("o", 0.0, 2.0, 0.0, 8.0), room("n", 0.0, 2.0, 0.0, 8.0)];
        assert_eq!(deal(rooms, &["p", "p", "p", "o", "o", "o", "n", "n", "n"]), block(&["dealt1 Alpha: n", "dealt2 Bravo: o", "dealt3 Charlie: p"]));
    }
    #[test]
    fn squad_deal_reads_the_page_row_of_the_game() {
        let mut g = crate::game::Game::blank();
        let men: Vec<String> = ["a", "a", "b", "c", "d", "d", "e"].iter().map(|m| String::from(*m)).collect();
        let out = squad_deal(&mut g, Some(&house(line())), &men, None).unwrap();
        let names: Vec<_> = out.squads.unwrap().iter().map(|q| q.name.clone().unwrap()).collect();
        assert_eq!(names, vec![String::from("Alpha"), String::from("Bravo")]);
    }
    #[test]
    fn the_file_wins_and_a_lesson_is_left_alone() {
        let men: Vec<String> = ["a", "a", "b"].iter().map(|m| String::from(*m)).collect();
        let mut pool = house(line());
        pool.squads = Some(Vec::new()); // `squads: []` is a level saying it is one pool
        assert!(deal_with(&page(), Some(&pool), &men, None).unwrap().squads.unwrap().is_empty());
        assert!(deal_with(&page(), Some(&house(line())), &men, Some(true)).unwrap().squads.is_none());
        assert!(deal_with(&page(), None, &men, None).is_none());
    }
}
