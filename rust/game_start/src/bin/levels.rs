//! WHAT A LEVEL OWES WHOEVER PLAYS IT, asked of the game itself: natively, with no page and no TypeScript (tools/levels/export.ts asks two of these of every level it writes).
//!
//!   levels second-way [LEVEL…] [--list] [--walls] [--nolists]      every room the defenders can shut has another way in
//!   levels sight [LEVEL…] [--write]                                a defender sees what he watches
//!   levels walked [LEVEL…] [--write]                               its starts and its doors are on a way
//!   levels stacks [MISSION…]                                       every stack mark is by its door
//!   levels stands [LEVEL…]                                         what it names stands on its built floor, and can be walked to
//!   levels doors [LEVEL…] [--list]                                 every door is a real door's size and the walk grid goes through it
//!   levels charges [LEVEL…] [--write]                              no place for a wall charge was taken by a thing stood at a wall
//!   levels names [LEVEL…]                                          it is named by its own rooms
//!   levels rain [LEVEL…]                                           its storm rains by its own roofs
//!
//! A LEVEL is a file of levels/ by its name; none named = every level the question is about. Exit 0 every line passes, 1 a FAIL line, 2 a word or a level that is not known.
//! LEVELS_BREAK=1 = each question's red path, which must fail.
//!
//! SECOND-WAY (the maintainer, of the barricading: "i worry that it effectively requires the player to blow open the door with a door charge or 40mm. maybe we just need to design levels so there's
//! always another route in?"). A room of the room fight is the floor reached without crossing a door; its doors are its entries. A man whose room it is wedges a door that may be wedged, that opens
//! toward him, that is on his squad's list, while his room has wedged fewer than it may. With every room that has men in it wedging what it can, every choice tried, is each such room still reached
//! from the team's start and the ground round the place without a charge or a 40 mm? A wedge shuts a door one way: nobody comes into the room it was set from by it, and from inside it is pulled open.
//! A WINDOW counts where a round opens it and a man walks through it. A WALL counts only on a level that issues wall charges for the run. Furniture is not asked for: the cautious answer.
//! THE MENDS: hang one of its doors to open OUT of the room, or give it a window a man steps through.
//!
//! SIGHT ("a window a mark watches through, or a covered lane crosses, stays see-through"): whatever the file stands with a `pos` and an `aimAt`. His line to what his gun rests on is open, and of the
//! 21 lines from his eye to the ground there he has no fewer than the level's count file says (tools/levels/counts/sight). Leaves swing and are not asked.
//!
//! WALKED ("wear, paths in the ground … can all tell a story for how people walk through the level"): from every start a level stands a way leaves, and every door is on one, but for those its file
//! names as owed (tools/levels/counts/walked); no way is without a path, no name is missing, and every named mark is on open floor.
//!
//! STACKS: over every squad mission, every door, both faces, the whole squad, a pair and one man: from every mark the squad's own search reaches the doorway's front in no more than 2.5 times the
//! crow's flight and a metre, with the door shut and with its leaf at its stop.
//!
//! STANDS: each mission the file names is started, and the game's own words on its block against the floor are read (the room clear's structure, the rescue's marks, a quiet mission's block); the spawn, the
//! objectives, the patrols' points and legs and the chokepoints are on floor and joined. A man of a rescue stood in a chair is said and passes: it is pushed.
//!
//! DOORS (tools/levels/lib/doorSize.ts): each leaf's own size, the air between its jambs and under its head, the guards' own search through its frame, a stride of free floor on both faces, nothing of the level
//! in the gap, a wall at both its ends; and each doorway with no leaf, found by its head.
//!
//! CHARGES (the compound's wood piles once took 23 of 891 places and nobody chose it): how many places of the level have floor behind them as the walk grid stands, against its count file
//! (tools/levels/counts/charges: one number; a line that starts # says what was given up and why). Fewer and more are both red.
//!
//! NAMES: over the whole floor on a half-metre lattice, what a line calls a spot and what the debrief does is one of the level's own rooms and places, and the middle of each is its own name.
//!
//! RAIN (the maintainer: "rain is busted on records office"): ground under open sky is rained on, and ground under the level's ceiling slabs is not.
use game::game::{Game, GuardState};
use game::ids::*;
use game_draw::traffic::{door_mid, ways, WayKind};
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};
use game_start::files;
use std::collections::{BTreeMap, BTreeSet, HashMap};

type V3 = [f64; 3];
fn hyp(a: V3, b: V3) -> f64 { (a[0] - b[0]).hypot(a[2] - b[2]) }
fn breaks() -> bool { std::env::var("LEVELS_BREAK").as_deref() == Ok("1") }

#[derive(Default)]
struct Verdicts { fails: usize, checks: usize }
impl Verdicts {
    fn say(&mut self, ok: bool, what: &str, detail: &str) { self.checks += 1; if !ok { self.fails += 1; } println!("- {} — {what}\n    {detail}", if ok { "PASS" } else { "FAIL" }); }
    fn end(&self, name: &str) -> i32 { if self.fails > 0 { println!("=== {name}: {} of {} FAILED ===", self.fails, self.checks); 1 } else { println!("=== {name}: ALL PASS ({}) ===", self.checks); 0 } }
}

fn every_level() -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(files::root().join("levels")).expect("levels/").filter_map(|e| e.ok()?.file_name().into_string().ok()).filter(|f| f.ends_with(".json") && !f.contains(".lint.")).map(|f| f.trim_end_matches(".json").to_string()).collect();
    v.sort();
    v
}
fn data_of(level: &str) -> Json { json::parse(&files::level_text(&format!("{level}.json"))).unwrap_or_else(|e| panic!("{level}: {e}")) }
fn missions() -> Vec<game::scene_scenarios::ScenarioEntry> { game::scene_scenario_list::scenarios() }
fn level_of(e: &game::scene_scenarios::ScenarioEntry) -> String { e.level.as_deref().unwrap_or("office.json").trim_end_matches(".json").to_string() }
fn query_of(e: &game::scene_scenarios::ScenarioEntry) -> String { game::scene_scenario_list::scenario_query(e) }

/// everything a level file says of one kind, outside its boxes and its props, each by where the file has it: a thing of a list by its place in it, or by its `id` in place of the list's name
fn said_in<T>(data: &Json, take: &dyn Fn(&Json) -> Option<T>) -> Vec<(String, T)> {
    fn walk<T>(o: &Json, path: &str, take: &dyn Fn(&Json) -> Option<T>, out: &mut Vec<(String, T)>) {
        match o {
            Json::Arr(a) => for (k, x) in a.iter().enumerate() {
                let p = match x.get("id").str() { Some(id) => format!("{}{}", &path[..path.rfind('.').map_or(0, |i| i + 1)], id.replace(char::is_whitespace, "_")), None => format!("{path}#{k}") };
                walk(x, &p, take, out);
            },
            Json::Obj(..) => {
                if let Some(v) = take(o) { out.push((path.to_string(), v)); }
                for (k, w) in o.members() { walk(w, &if path.is_empty() { k.clone() } else { format!("{path}.{k}") }, take, out); }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for (k, v) in data.members() { if k == "scenarios" { walk(v, "", take, &mut out); } else if k != "boxes" && k != "props" { walk(v, k, take, &mut out); } }
    out
}

// ---------------------------------------------------------------------------------------------------------------- second-way
const OPEN: usize = 2400;
/// a window a man steps through: its bottom under this (a kerb that high stops him) and its top at this or over
const WINDOW_SILL: f64 = 0.2;
const WINDOW_HEAD: f64 = 1.85;
struct Room { id: RoomfightRoomId, cells: BTreeSet<usize>, entries: Vec<game::roomfight::Entry> }
struct Door { d: DoorId, name: String, a: usize, b: Option<usize>, into: Option<usize>, pad: bool, locked_steel: bool, plain: &'static str }
struct Window { a: usize, b: usize, at: [f64; 2] }

fn cell_at(g: &Game, x: f64, z: f64) -> bool { let n = &g.col.nav; x >= 0.0 && z >= 0.0 && x < n.w * n.cell && z < n.h * n.cell && !game::scene_collision::nav_grid_is_blocked(g, x, z) }
fn room_at(g: &mut Game, x: f64, z: f64) -> Option<RoomfightRoomId> { if cell_at(g, x, z) { game::roomfight::room_fight_room_at(g, [x, 0.0, z]) } else { None } }
fn cells_of(g: &Game, r: RoomfightRoomId) -> BTreeSet<usize> { g.tab.roomfight_room[r].cells.keys().iter().map(|c| c.0 as usize).collect() }
fn key_of(g: &Game, r: RoomfightRoomId) -> usize { *cells_of(g, r).first().expect("a room has floor") }
/// a mission staged and let stand: nobody thinks, so everybody is where it put him
fn staged(level: &str, query: &str) -> Play {
    let mut p = if query.is_empty() { Play::level(&format!("{level}.json"), false, &Opts::default()) } else { Play::mission(&format!("{level}.json"), query, &Opts::default()) };
    p.g.ai_enabled = false;
    for _ in 0..20 { p.frame(); }
    p
}
/// every k of the list, or the list where it is no longer
fn subsets(l: &[usize], k: usize) -> Vec<Vec<usize>> {
    if k == 0 || l.is_empty() { return vec![vec![]]; }
    if l.len() <= k { return vec![l.to_vec()]; }
    let mut out: Vec<Vec<usize>> = subsets(&l[1..], k - 1).into_iter().map(|mut s| { s.insert(0, l[0]); s }).collect();
    out.extend(subsets(&l[1..], k));
    out
}
struct SecondWay { list: bool, walls: bool, nolists: bool }
fn second_way(levels: &[String], o: &SecondWay) -> i32 {
    let mut v = Verdicts::default();
    // every kind of squad mission a level has is staged, each at its largest squad: who stands where differs
    let mut by_level: BTreeMap<String, Vec<game::scene_scenarios::ScenarioEntry>> = BTreeMap::new();
    for e in missions() {
        let l = by_level.entry(level_of(&e)).or_default();
        match l.iter_mut().find(|w| w.kind == e.kind) { None => l.push(e), Some(w) => if e.squad > w.squad { *w = e; } }
    }
    let levels: Vec<String> = if levels.is_empty() { by_level.keys().cloned().collect() } else { levels.to_vec() };
    for level in &levels { second_way_of(level, by_level.get(level).map_or(&[][..], |l| l), o, &mut v); }
    v.end("second-way")
}
fn second_way_of(level: &str, entries: &[game::scene_scenarios::ScenarioEntry], o: &SecondWay, v: &mut Verdicts) {
    let data = data_of(level);
    let (clear, rescue) = (data.get("scenarios").get("roomClear"), data.get("scenarios").get("rescue"));
    let queries: Vec<String> = if entries.is_empty() { vec![if !clear.is_null() { "?roomclear&squad4".into() } else if !rescue.is_null() { "?rescue".into() } else { String::new() }] } else { entries.iter().map(query_of).collect() };
    // each mission staged: who stands in which room, where the team starts. The last staging's doors, panes and walls are asked
    let (mut men, mut starts): (HashMap<usize, usize>, BTreeSet<usize>) = (HashMap::new(), BTreeSet::new());
    let mut p: Option<Play> = None;
    for q in queries.iter().rev() {
        drop(p.take());
        let mut s = staged(level, q);
        let at: Vec<[f64; 2]> = s.g.guards.iter().filter(|&&gd| s.g.gd[gd].team != game::teams::Team::Operative && s.g.gd[gd].state != GuardState::Dead).map(|&gd| s.at(gd)).collect();
        let mut here: HashMap<usize, usize> = HashMap::new();
        for a in at { if let Some(r) = room_at(&mut s.g, a[0], a[1]) { *here.entry(key_of(&s.g, r)).or_default() += 1; } }
        for (k, c) in here { let m = men.entry(k).or_default(); *m = (*m).max(c); }
        let sam = s.sam_at();
        if let Some(r) = room_at(&mut s.g, sam[0], sam[1]) { starts.insert(key_of(&s.g, r)); }
        p = Some(s);
    }
    let mut p = p.expect("a mission was staged");
    let g: &mut Game = &mut p.g;
    let (nw, nh, cs) = (g.col.nav.w as usize, g.col.nav.h as usize, g.col.nav.cell);
    let middle = |c: usize| -> [f64; 2] { [((c % nw) as f64 + 0.5) * cs, ((c / nw) as f64 + 0.5) * cs] };
    let mut ids: Vec<RoomfightRoomId> = Vec::new();
    for c in 0..nw * nh { if g.col.nav.blocked[c] == 1 { continue; } let [x, z] = middle(c); if let Some(r) = game::roomfight::room_fight_room_at(g, [x, 0.0, z]) { if !ids.contains(&r) { ids.push(r); } } }
    let rooms: Vec<Room> = ids.iter().map(|&id| Room { id, cells: cells_of(g, id), entries: g.tab.roomfight_room[id].entries.clone() }).collect();
    let index: HashMap<RoomfightRoomId, usize> = rooms.iter().enumerate().map(|(i, r)| (r.id, i)).collect();
    let key = |i: usize| *rooms[i].cells.first().unwrap();
    // what a room is called: the structure's own name where it lies in its bounds
    let names: Vec<(String, bool)> = rooms.iter().map(|r| {
        let (mut best, mut n) = (None, 0);
        for s in clear.get("rooms").items() {
            let b = s.get("bounds"); let e = |k: &str| b.get(k).num().unwrap_or(0.0);
            let k = r.cells.iter().filter(|&&c| { let [x, z] = middle(c); x >= e("x0") && x <= e("x1") && z >= e("z0") && z <= e("z1") }).count();
            if k > n { n = k; best = Some(s); }
        }
        match best { Some(s) if n as f64 >= r.cells.len() as f64 * 0.3 => (s.get("name").str().or(s.get("id").str()).unwrap_or("?").to_string(), true),
            _ if r.cells.len() >= OPEN => ("open floor".into(), false),
            _ => (format!("floor by {}", r.entries.iter().take(2).map(|e| g.dr[e.door].def.name.clone()).collect::<Vec<_>>().join(" / ")), false) }
    }).collect();
    // a rescue's cast is dealt: every place the level may stand a man counts, whatever this staging drew
    let mut may: Vec<&Json> = vec![rescue.get("captor"), rescue.get("doorMan").get("inside"), rescue.get("doorMan").get("outside")];
    may.extend(rescue.get("roamers").items()); may.extend(rescue.get("guards").items());
    for m in may { if let Some(at) = m.get("pos").v3() { if let Some(r) = room_at(g, at[0], at[2]) { let m = men.entry(key_of(g, r)).or_default(); *m = (*m).max(1); } } }
    let manned: Vec<bool> = (0..rooms.len()).map(|i| rooms[i].cells.len() < OPEN && !rooms[i].entries.is_empty() && (names[i].1 || men.get(&key(i)).copied().unwrap_or(0) > 0)).collect();
    // which squad's list a door is on
    let sc = g.level.scenarios.as_ref().and_then(|s| s.room_clear.as_ref());
    let lists: Vec<Vec<String>> = sc.map_or(vec![], |c| c.squads.iter().flatten().map(|s| game::scene_scenarios::squad_barricade(s, &c.rooms)).collect());
    let on_list = |name: &str| o.nolists || lists.is_empty() || lists.iter().any(|l| l.iter().any(|n| n == name));
    // doors: the two rooms each joins, the one it opens into, whether anybody may wedge it
    let mut doors: Vec<Door> = Vec::new();
    for i in 0..rooms.len() { for e in rooms[i].entries.clone() {
        let k = match doors.iter().position(|r| r.d == e.door) { Some(k) => k, None => {
            let mut other = None;
            for far in [0.6, 0.8, 1.0, 1.3, 1.7] { other = room_at(g, e.at[0] + e.out[0] * far, e.at[2] + e.out[1] * far).filter(|r| *r != rooms[i].id); if other.is_some() { break; } }
            let (pad, name) = (game::devices::devices_pad_of(g, e.door).is_some(), g.dr[e.door].def.name.clone());
            let (steel, locked) = (g.dr[e.door].def.steel == Some(true), g.dr[e.door].def.locked == Some(true) || game::doors::door_locked(g, e.door));
            let plain = if steel { "steel" } else if locked { "locked in the level" } else if pad { "keypad" } else if !on_list(&name) { "on no squad's list" } else { "" };
            doors.push(Door { d: e.door, name, a: i, b: other.and_then(|r| index.get(&r).copied()), into: None, pad, locked_steel: steel && locked, plain });
            doors.len() - 1
        } };
        if game::doors::doors_wedge_holds(g, e.door, [e.at[0] - e.out[0] * 0.8, 0.0, e.at[2] - e.out[1] * 0.8]) { doors[k].into = Some(i); }
    } }
    let wedgeable: Vec<Vec<usize>> = (0..rooms.len()).map(|i| if manned[i] { (0..doors.len()).filter(|&k| doors[k].into == Some(i) && doors[k].plain.is_empty()).collect() } else { vec![] }).collect();
    // windows: a sheet a round opens and a man walks through, with floor of two rooms at its faces
    let glass = game::scene_boxes::box_flag().glass as i64;
    let mut windows: Vec<Window> = Vec::new();
    for bi in 0..g.col.boxes.len() {
        let id = g.col.boxes[bi];
        let (c, h, flags) = { let b = &g.tab.boxes[id]; (b.c, b.h, b.flags as i64) };
        if flags & glass == 0 { continue; }
        let thin_x = h[0] < h[2];
        if h[0].min(h[2]) > 0.05 || c[1] - h[1] >= WINDOW_SILL - 1e-6 || c[1] + h[1] < WINDOW_HEAD || h[0].max(h[2]) < 0.3 { continue; }
        if game::glassbreak::glass_break_type_of_pane(g, bi as f64) == Some(game::glasspen::PaneType::Laminated) { continue; }
        for off in [0.45, 0.75] {
            let (dx, dz) = if thin_x { (off, 0.0) } else { (0.0, off) };
            let (a, b) = (room_at(g, c[0] - dx, c[2] - dz), room_at(g, c[0] + dx, c[2] + dz));
            let (Some(a), Some(b)) = (a, b) else { continue };
            if a == b { continue; }
            // the player's own circle and height band taken across the sheet with the sheet gone (a kerb, a lintel or furniture against it stops him)
            g.tab.boxes[id].c[1] += 100.0; game::scene_collision::wrote(g);
            let mut walks = true; let mut s = -0.7;
            while s <= 0.7 + 1e-6 { let at = if thin_x { [c[0] + s, 0.0, c[2]] } else { [c[0], 0.0, c[2] + s] }; let mut q = at; game::scene_collision::collide_circle(g, &mut q, 0.42, 0.2, 1.7, Some(2.0)); if hyp(q, at) > 0.02 { walks = false; break; } s += 0.1; }
            g.tab.boxes[id].c[1] = c[1]; game::scene_collision::wrote(g);
            if walks { windows.push(Window { a: index[&a], b: index[&b], at: [c[0], c[2]] }); }
            break;
        }
    }
    // wall places: where a wall charge is accepted, from which room into which
    let issued = clear.get("wallCharges").num().unwrap_or(0.0) > 0.0;
    let mut walls_into: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); rooms.len()];
    if issued || o.walls {
        let n = (g.destr.walls.authored as usize).min(g.col.boxes.len());
        for bi in 0..n {
            let (c, h) = { let b = &g.tab.boxes[g.col.boxes[bi]]; (b.c, b.h) };
            if h[1] < 0.9 || h[0].min(h[2]) > 0.2 || h[0].max(h[2]) < 0.75 { continue; }
            let along_x = h[0] >= h[2]; let l = if along_x { h[0] } else { h[2] };
            let mut u = -l + 0.1;
            while u <= l - 0.1 + 1e-6 {
                for side in [1.0, -1.0] {
                    let at: V3 = if along_x { [c[0] + u, 0.0, c[2]] } else { [c[0], 0.0, c[2] + u] };
                    let (from, past): (V3, V3) = if along_x { ([at[0], 0.0, c[2] + side], [at[0], 0.0, c[2] - side]) } else { ([c[0] + side, 0.0, at[2]], [c[0] - side, 0.0, at[2]]) };
                    let (Some(a), Some(b)) = (room_at(g, from[0], from[2]), room_at(g, past[0], past[2])) else { continue };
                    if a == b || rooms[index[&b]].cells.len() >= OPEN { continue; }
                    let Some(w0) = game::wallcharge::wall_at(g, at, Some(from)) else { continue };
                    if w0.index as usize != bi || !w0.ok { continue; }
                    let w = game::wallcharge::wall_charge_settle(g, &w0);
                    if game::wallcharge::wall_charge_set_spot(g, &w).is_some() && !game::wallcharge::wall_charge_open_at(g, &w) && game::wallcharge::wall_charge_far_floor(g, &w, None).is_some() { *walls_into[index[&b]].entry(index[&a]).or_default() += 1; }
                }
                u += 0.5;
            }
        }
    }
    // the walk in: from the open floor, the team's starts and the ground round the place, through whatever is not wedged
    let edge = |i: usize| rooms[i].cells.iter().any(|&c| { let (x, z) = (c % nw, c / nw); x <= 1 || z <= 1 || x + 2 >= nw || z + 2 >= nh });
    let sources: Vec<bool> = (0..rooms.len()).map(|i| rooms[i].cells.len() >= OPEN || starts.contains(&key(i)) || (edge(i) && !manned[i])).collect();
    let opens = |r: &Door, to: usize| !r.locked_steel && (!r.pad || r.into == Some(to));
    let broken = breaks();
    let walk = |wedged: &BTreeSet<usize>, through_glass: bool| -> Vec<bool> {
        let mut got = sources.clone();
        loop {
            let mut more = false;
            for (k, r) in doors.iter().enumerate() {
                let Some(b) = r.b else { continue };
                let w = wedged.contains(&k);
                if got[r.a] && !got[b] && opens(r, b) && !(w && r.into == Some(b)) { got[b] = true; more = true; }
                if got[b] && !got[r.a] && opens(r, r.a) && !(w && r.into == Some(r.a)) { got[r.a] = true; more = true; }
            }
            if !broken {
                if through_glass { for w in &windows { if got[w.a] != got[w.b] { got[w.a] = true; got[w.b] = true; more = true; } } }
                if issued { for i in 0..rooms.len() { if !got[i] && walls_into[i].keys().any(|&a| got[a]) { got[i] = true; more = true; } } }
            }
            if !more { return got; }
        }
    };
    // every choice the defenders have: a room with more doors than wedges picks which
    let most = g.k.digin.wedges as usize;
    let mut fixed: BTreeSet<usize> = BTreeSet::new();
    let mut choosers: Vec<Vec<Vec<usize>>> = Vec::new();
    for l in &wedgeable { if l.len() <= most { fixed.extend(l); } else { choosers.push(subsets(l, most)); } }
    let combos: f64 = choosers.iter().map(|c| c.len() as f64).product();
    let (mut unreached, mut glass_only, mut tried) = (vec![String::new(); rooms.len()], vec![false; rooms.len()], 0usize);
    let mut tell = |wedged: &BTreeSet<usize>| {
        tried += 1;
        let got = walk(wedged, true);
        for i in 0..rooms.len() { if !got[i] && unreached[i].is_empty() { let by: Vec<&str> = wedged.iter().map(|&k| &doors[k]).filter(|r| r.a == i || r.b == Some(i)).map(|r| r.name.as_str()).collect(); unreached[i] = if by.is_empty() { "no door leads to it".into() } else { by.join(", ") }; } }
        if !windows.is_empty() && !broken { let dry = walk(wedged, false); for i in 0..rooms.len() { if got[i] && !dry[i] { glass_only[i] = true; } } }
    };
    if combos <= 20000.0 {
        let mut pick = vec![0usize; choosers.len()];
        'all: loop {
            let mut wedged = fixed.clone();
            for (k, c) in choosers.iter().enumerate() { wedged.extend(&c[pick[k]]); }
            tell(&wedged);
            let mut k = choosers.len();
            loop { if k == 0 { break 'all; } k -= 1; pick[k] += 1; if pick[k] < choosers[k].len() { break; } pick[k] = 0; }
        }
    } else { let mut wedged = fixed.clone(); wedged.extend(choosers.iter().flatten().flatten()); tell(&wedged); }   // (too many to try: every door any of them could wedge taken as wedged, the cautious answer)
    // the list and the verdict
    let asked: Vec<usize> = (0..rooms.len()).filter(|&i| !rooms[i].entries.is_empty()).collect();   // (floor no door leads onto is not asked)
    let at = |i: usize| -> String { let (mut x, mut z) = (0.0, 0.0); for &c in &rooms[i].cells { let m = middle(c); x += m[0]; z += m[1]; } let n = rooms[i].cells.len() as f64; format!("({:.1}, {:.1})", x / n, z / n) };
    if o.list { for &i in &asked {
        let ds: Vec<String> = doors.iter().filter(|r| r.a == i || r.b == Some(i)).map(|r| { let to = if r.a == i { r.b } else { Some(r.a) }; let by = if r.plain.is_empty() { r.into.filter(|&k| manned[k]).map(|k| format!(" [wedged by {}]", names[k].0)).unwrap_or_default() } else { format!(" [{}]", r.plain) }; format!("{} → {}{by}", r.name, to.map_or("?", |k| &names[k].0)) }).collect();
        let ws: Vec<String> = windows.iter().filter(|w| w.a == i || w.b == i).map(|w| format!("({:.1}, {:.1}) → {}", w.at[0], w.at[1], names[if w.a == i { w.b } else { w.a }].0)).collect();
        println!("    {level}: {} at {}, {} m², men {}{}{} · doors {} · windows {} · walls {}{}", names[i].0, at(i), rooms[i].cells.len() / 4, men.get(&key(i)).copied().unwrap_or(0), if manned[i] { ", manned" } else { "" }, if sources[i] { ", walked to begin with" } else { "" },
            ds.join(", "), if ws.is_empty() { "none".into() } else { ws.join(", ") }, walls_into[i].values().sum::<usize>(), if unreached[i].is_empty() { String::new() } else { format!(" · SHUT BY {}", unreached[i]) });
    } }
    let bad: Vec<usize> = asked.iter().copied().filter(|&i| manned[i] && !unreached[i].is_empty()).collect();
    let empty: Vec<usize> = asked.iter().copied().filter(|&i| !manned[i] && rooms[i].cells.len() < OPEN && !unreached[i].is_empty()).collect();   // (nobody is at home in it to shut it: noted, not failed)
    let by_glass: Vec<usize> = asked.iter().copied().filter(|&i| manned[i] && unreached[i].is_empty() && glass_only[i]).collect();
    let say = |l: &[usize]| l.iter().map(|&i| format!("{} at {}", names[i].0, at(i))).collect::<Vec<_>>().join(", ");
    let mut detail = format!("{} rooms, {} with men at home, {} mission{} staged, {tried} wedge choice{} tried", asked.len(), asked.iter().filter(|&&i| manned[i]).count(), queries.len(), if queries.len() > 1 { "s" } else { "" }, if tried > 1 { "s" } else { "" });
    for &i in &bad { detail += &format!("\n    NO WAY IN: {} at {}, shut by {}", names[i].0, at(i), unreached[i]); }
    if !by_glass.is_empty() { detail += &format!("\n    by a window alone: {}", say(&by_glass)); }
    let by_wall: Vec<usize> = if issued { asked.iter().copied().filter(|&i| manned[i] && unreached[i].is_empty() && !glass_only[i] && !walls_into[i].is_empty() && doors.iter().filter(|r| r.a == i || r.b == Some(i)).all(|r| r.plain.is_empty() && r.into.is_some_and(|k| manned[k]))).collect() } else { vec![] };
    if !by_wall.is_empty() { detail += &format!("\n    by its walls alone (wall charges are issued here): {}", say(&by_wall)); }
    if !empty.is_empty() { detail += &format!("\n    nobody's, and shut: {}", say(&empty)); }
    v.say(bad.is_empty(), &format!("{level}: every room they can shut has another way in"), &detail);
}

// ---------------------------------------------------------------------------------------------------------------- sight
const EYE: f64 = 1.5;
const HEIGHTS: [f64; 3] = [0.5, 1.0, 1.5];
const ACROSS: [f64; 7] = [-1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5];
fn counts(of: &str, level: &str) -> std::path::PathBuf { files::root().join(format!("tools/levels/counts/{of}/{level}.txt")) }
/// a count file's lines, its first (what the file is) left out
fn held(of: &str, level: &str) -> Option<Vec<String>> { let t = std::fs::read_to_string(counts(of, level)).ok()?; Some(t.lines().enumerate().filter(|(k, l)| !(*k == 0 && l.starts_with('#')) && !l.trim().is_empty()).map(|(_, l)| l.trim_end().to_string()).collect()) }
fn keep(of: &str, level: &str, text: &str) -> bool { let f = counts(of, level); if std::fs::read_to_string(&f).ok().as_deref() == Some(text) { return false; } std::fs::create_dir_all(f.parent().unwrap()).unwrap(); std::fs::write(f, text).unwrap(); true }

fn sight(levels: &[String], write: bool) -> i32 {
    let (mut v, broken, named) = (Verdicts::default(), breaks(), !levels.is_empty());
    let levels: Vec<String> = if named { levels.to_vec() } else { every_level() };
    let (mut stood, mut marks, mut more, mut uncounted) = (0, 0, 0, 0);
    let (mut shut, mut less, mut unjudged, mut wrote): (Vec<String>, Vec<String>, Vec<&str>, Vec<&str>) = (vec![], vec![], vec![], vec![]);
    for level in &levels {
        let data = data_of(level);
        let men = said_in(&data, &|o: &Json| Some((o.get("pos").v3()?, o.get("aimAt").v3()?)));
        if men.is_empty() { unjudged.push(level); continue; }
        let mut p = Play::level(&format!("{level}.json"), false, &Opts::default());
        stood += 1;
        p.g.col.dynamic_boxes.clear();   // (the leaves out: they swing)
        if broken {   // a post in the middle of the first man's line, by a box of the level moved there
            let (a, b) = men[0].1; let half = (hyp(a, b) / 4.0).min(0.8); let id = p.g.col.boxes[0];
            let x = &mut p.g.tab.boxes[id]; x.c = [(a[0] + b[0]) / 2.0, 1.0, (a[2] + b[2]) / 2.0]; x.h = [half, 1.0, half]; x.yaw = 0.0; x.rot = None; x.flags = 0.0;
        }
        game::scene_collision::wrote(&mut p.g);
        let file = held("sight", level);
        let was: HashMap<String, usize> = file.iter().flatten().filter(|l| !l.starts_with('#')).filter_map(|l| { let (n, c) = l.rsplit_once(' ')?; Some((n.to_string(), c.parse().ok()?)) }).collect();
        let mut now: Vec<String> = vec![];
        for (id, (pos, aim)) in &men {
            marks += 1;
            let eye = [pos[0], pos[1] + EYE, pos[2]];
            if game::scene_collision::segment_blocked(&mut p.g, eye, *aim, None, None, None) { shut.push(format!("{level}: {id} (from {}, {} to {:?})", pos[0], pos[2], aim)); }
            let (dx, dz) = (aim[0] - eye[0], aim[2] - eye[2]); let l = dx.hypot(dz); let l = if l == 0.0 { 1.0 } else { l };
            let mut n = 0;
            for y in HEIGHTS { for s in ACROSS { if !game::scene_collision::segment_blocked(&mut p.g, eye, [aim[0] - dz / l * s, pos[1] + y, aim[2] + dx / l * s], None, None, None) { n += 1; } } }
            now.push(format!("{id} {n}"));
            match was.get(id) { None => if file.is_some() && !write { uncounted += 1; }, Some(&w) if n < w => less.push(format!("{level}: {id} {n} of 21, the file says {w}")), Some(&w) if n > w => more += 1, _ => {} }
        }
        if file.is_none() && !write { less.push(format!("{level}: no file (--write)")); }
        if write && !broken {
            let notes: Vec<String> = file.into_iter().flatten().filter(|l| l.starts_with('#')).collect();
            let text = format!("# how many of 21 lines each man of {level} has open to the ground his gun rests on: his name, his count (levels sight --write; a line that starts # under this one is a note and is kept)\n{}{}\n", notes.iter().map(|l| format!("{l}\n")).collect::<String>(), now.join("\n"));
            if keep("sight", level, &text) { wrote.push(level); }
        }
    }
    v.say(shut.is_empty() && (stood > 0 || named), "each man's line from his eye to what his gun rests on is open", &format!("{stood} levels, {marks} men, {} blocked{}", shut.len(), list(&shut)));
    v.say(write || less.is_empty(), "no man sees less of the ground he watches than his level's file says", &format!("{} see less, {more} see more, {uncounted} the file has no count for{}{}{}", less.len(), list(&less),
        if wrote.is_empty() { String::new() } else { format!("\n    wrote {}", wrote.join(", ")) }, if unjudged.is_empty() { String::new() } else { format!("\n    nobody to ask on {}", unjudged.join(", ")) }));
    v.end("sight")
}
fn list(l: &[String]) -> String { l.iter().map(|x| format!("\n    {x}")).collect() }

// ---------------------------------------------------------------------------------------------------------------- walked
fn walked(levels: &[String], write: bool) -> i32 {
    let (mut v, broken, named) = (Verdicts::default(), breaks(), !levels.is_empty());
    let every = every_level();
    let levels: Vec<String> = if named { levels.to_vec() } else { every.clone() };
    let mut listed: BTreeSet<String> = game::scene_scenario_list::all_missions().iter().map(level_of).collect();
    listed.insert("office".into());
    let (mut n_ways, mut n_starts, mut n_doors, mut owed, mut judged) = (0, 0, 0, 0, 0);
    let (mut sick, mut fresh, mut mended, mut wrote): (Vec<String>, Vec<String>, Vec<String>, Vec<&str>) = (vec![], vec![], vec![], vec![]);
    if !named { if let Ok(d) = std::fs::read_dir(files::root().join("tools/levels/counts/walked")) { for f in d.filter_map(|e| e.ok()?.file_name().into_string().ok()) { if !every.iter().any(|l| format!("{l}.txt") == f) { sick.push(format!("{f} is the count file of no level")); } } } }
    for level in &levels {
        let data = data_of(level);
        let mut p = Play::level(&format!("{level}.json"), false, &Opts::default());
        if broken { p.g.level.routes.clear(); p.g.level.scenarios = None; }
        let w = ways(&p.g);
        n_ways += w.list.len();
        if w.lost > 0 { sick.push(format!("{level}: {} with no path", w.lost)); }
        if !w.unknown.is_empty() { sick.push(format!("{level}: no such {}", w.unknown.join(", "))); }
        for m in p.g.level.marks.iter().flatten() { if game::scene_collision::nav_grid_is_blocked(&p.g, m.pos[0], m.pos[2]) { sick.push(format!("{level}: the mark '{}' at ({}, {}) stands where nobody can", m.name, m.pos[0], m.pos[2])); } }
        let long: Vec<_> = w.list.iter().filter(|x| x.points.windows(2).map(|q| hyp(q[0], q[1])).sum::<f64>() >= 2.0).collect();
        let leaves = |s: V3| long.iter().any(|x| matches!(x.kind, WayKind::Team | WayKind::Hint) && hyp(x.points[0], s) <= 0.3);
        let on_way = |d: &game::scene_level::LevelDoorDef| -> bool {
            let (c, ax, az) = (door_mid(d), d.closed_dir.cos(), d.closed_dir.sin());
            let side = |q: V3| -az * (q[0] - c[0]) + ax * (q[2] - c[2]);
            // a point that lies ON the frame's line is where the way crosses, if it comes out the other side
            let crosses = |pts: &[V3]| -> bool {
                let (mut prev, mut on): (Option<(V3, f64)>, Option<V3>) = (None, None);
                for &q in pts {
                    let s = side(q);
                    if s.abs() < 1e-9 { on = Some(q); continue; }
                    if let Some((a, sa)) = prev { if sa * s < 0.0 {
                        let t = sa / (sa - s);
                        let (x, z) = on.map_or((a[0] + (q[0] - a[0]) * t, a[2] + (q[2] - a[2]) * t), |o| (o[0], o[2]));
                        if (ax * (x - c[0]) + az * (z - c[2])).abs() <= d.width / 2.0 + 0.15 { return true; }
                    } }
                    prev = Some((q, s)); on = None;
                }
                false
            };
            long.iter().any(|x| hyp(x.points[0], c) <= 0.75 || hyp(*x.points.last().unwrap(), c) <= 0.75 || crosses(&x.points))
        };
        let (mut owes, mut known): (Vec<String>, BTreeSet<String>) = (vec![], BTreeSet::new());
        if listed.contains(level) {
            judged += 1;
            let mut starts: Vec<(String, V3)> = said_in(&json::obj(vec![("scenarios", data.get("scenarios").clone())]), &|o: &Json| o.get("sam").v3());
            if starts.is_empty() || level == "office" { starts.push(("spawn".into(), p.g.level.player_spawn)); }
            n_starts += starts.len();
            for (id, at) in &starts { let w = format!("start {id}"); known.insert(w.clone()); if !leaves(*at) { owes.push(w); } }
        }
        for d in &p.g.level.doors { n_doors += 1; let w = format!("door {}", d.name.replace(char::is_whitespace, "_")); known.insert(w.clone()); if !on_way(d) { owes.push(w); } }
        owed += owes.len();
        let file = held("walked", level).unwrap_or_default();
        let said: Vec<&String> = file.iter().filter(|l| !l.starts_with('#')).collect();
        let news: Vec<String> = owes.iter().filter(|x| !said.contains(x)).cloned().collect();
        let gone: Vec<String> = said.iter().filter(|x| known.contains(**x) && !owes.contains(x)).map(|x| x.to_string()).collect();
        let strange: Vec<String> = if broken { vec![] } else { said.iter().filter(|x| !known.contains(**x)).map(|x| x.to_string()).collect() };
        if !news.is_empty() { fresh.push(format!("{level}: {}", news.join(", "))); }
        if !gone.is_empty() { mended.push(format!("{level}: {}", gone.join(", "))); }
        if !strange.is_empty() && !write { sick.push(format!("{level}.txt names what {level} does not have: {}", strange.join(", "))); }
        if write && !broken {
            let mut body: Vec<String> = vec![];
            for l in &file { if l.starts_with('#') || (owes.contains(l) && !body.contains(l)) { body.push(l.clone()); } }
            body.extend(news);
            if body.is_empty() { if std::fs::remove_file(counts("walked", level)).is_ok() { wrote.push(level); } }
            else if keep("walked", level, &format!("# what the ways of {level} do not reach yet, as the page loads it: a start no way leaves from, a door no way crosses or ends at (levels walked --write rewrites this first line; a line that starts # under it is a note and is kept)\n{}\n", body.join("\n"))) { wrote.push(level); }
        }
    }
    v.say(sick.is_empty(), "no way is without a path, no name a level's ways ask for is missing, every mark is on open floor, and every count file is a level's and names what it has", &format!("{} levels, {n_ways} ways{}", levels.len(), list(&sick)));
    v.say(write || fresh.is_empty(), "from every start a level stands a way leaves, and every door is on a way, but for those its file names as owed", &format!("{n_starts} starts on the {judged} levels the mission list draws, {n_doors} doors, {owed} owed{}{}{}", list(&fresh),
        if mended.is_empty() { String::new() } else { format!("\n    has a way now (--write drops it): {}", mended.join(" · ")) }, if wrote.is_empty() { String::new() } else { format!("\n    wrote {}", wrote.join(", ")) }));
    v.end("walked")
}

// ---------------------------------------------------------------------------------------------------------------- stacks
fn stacks(ids: &[String]) -> i32 {
    let mut v = Verdicts::default();
    for e in missions().into_iter().filter(|e| ids.is_empty() || ids.contains(&e.id)) {
        let mut p = staged(&level_of(&e), &query_of(&e));
        let g: &mut Game = &mut p.g;
        let men: Vec<GuardId> = g.guards.iter().copied().filter(|&gd| g.gd[gd].team == game::teams::Team::Operative && g.gd[gd].state != GuardState::Dead).collect();
        let crews: Vec<(&[GuardId], &str)> = if men.len() > 2 { vec![(&men[..], ""), (&men[..2], " pair"), (&men[..1], " one")] } else if men.is_empty() { vec![] } else { vec![(&men[..], ""), (&men[..1], " one")] };
        let (mut marks, mut faces, mut refused, mut far): (usize, usize, usize, Vec<String>) = (0, 0, 0, vec![]);
        let list_ = g.doors.list.clone();
        for &d in &list_ {
            let a0 = game::doors::door_angle(g, d);
            let stops: Vec<f64> = [game::doors::door_max_a(g, d), game::doors::door_min_a(g, d)].into_iter().filter(|a| a.abs() > 0.3).collect();
            let set = |g: &mut Game, a: f64| { game::doors::door_set_angle(g, d, a); game::doors::door_place(g, d); };
            for side in [1.0, -1.0] { for (crew, who) in &crews {
                set(g, 0.0);
                let Some(posts) = game::orders::squad_orders_stack_posts(g, d, side, crew, None) else { continue };
                let mut near = |g: &mut Game, posts: &[game::orders::StackPost], how: &str| {
                    let (c, th) = (g.dr[d].frame_centre, g.dr[d].def.closed_dir);
                    let out = g.k.orders.jamb_out;
                    let (times, more) = if breaks() { (0.5, 0.0) } else { (2.5, 1.0) };   // (the red path: a walk may be half as long as the crow's flight)
                    let front = [c[0] - th.sin() * side * out, 0.0, c[2] + th.cos() * side * out];
                    for q in posts {
                        // a mark on no floor is walked to as the order walks a man to it: the nearest floor there is
                        let seat = game::seats::here(g);
                        let from = if game::scene_collision::nav_grid_is_blocked(g, q.pos[0], q.pos[2]) { game::orders::squad_orders_free_spot(g, &seat, q.pos).unwrap_or(q.pos) } else { q.pos };
                        let path = game::scene_collision::nav_grid_find_path(g, from, front, None, None);
                        let len = path.as_ref().map(|w| { let mut n = 0.0; let mut a = from; for &b in w { n += hyp(a, b); a = b; } n });
                        marks += 1;
                        if len.is_none_or(|n| n > hyp(q.pos, front) * times + more) { far.push(format!("{} {}{who}{how}: the mark ({:.2}, {:.2}) is {} from the door", g.dr[d].def.name, if side > 0.0 { "+" } else { "-" }, q.pos[0], q.pos[2], len.map_or("no walk".into(), |n| format!("{n:.1} m' walk")))); }
                    }
                };
                near(g, &posts, "");
                faces += 1;
                for &a in &stops { set(g, a); match game::orders::squad_orders_stack_posts(g, d, side, crew, None) { None => refused += 1, Some(q) => near(g, &q, " (open)") } }
            } }
            set(g, a0);
        }
        v.say(far.is_empty(), &format!("{}: every stack mark is by its door", e.id), &format!("{marks} marks on {} doors, {faces} faces stacked shut, {refused} of them refused open{}", list_.len(), list(&far)));
    }
    v.end("stacks")
}

// ---------------------------------------------------------------------------------------------------------------- stands
fn kinds(data: &Json) -> Vec<(&'static str, &'static str)> {
    let s = data.get("scenarios");
    [("roomClear", "roomclear&squad4"), ("rescue", "rescue&squad=2"), ("nightBreach", "nightbreach&squad=2"), ("quiet", "quiet")].into_iter().filter(|(k, _)| !s.get(k).is_null()).collect()
}
fn floor_of(g: &mut Game, p: V3) -> Option<V3> { game::scene_collision::nav_grid_nearest_free_point(g, p[0], p[2]) }
fn walk_len(g: &mut Game, a: V3, b: V3) -> Option<f64> { let w = game::scene_collision::nav_grid_find_path(g, a, b, Some(20000.0), None)?; let mut n = 0.0; let mut q = a; for p in w { n += hyp(q, p); q = p; } Some(n) }
/// a response block against its built floor: free marks apart from each other, a walk in for each man from his muster to his fan and on to the level's floor, a door of the level between the two, posts and
/// places that can be reached, a way clear of every start
const RESPONSE_CLEAR: f64 = 12.0;
fn response(g: &mut Game, sc: &Json, bad: &mut Vec<String>) {
    let r = sc.get("response");
    if r.is_null() { return; }
    let pts = |j: &Json| -> Vec<V3> { j.items().iter().filter_map(|p| p.v3()).collect() };
    let free = |g: &Game, p: V3| !game::scene_collision::nav_grid_is_blocked(g, p[0], p[2]);
    let flat = |p: V3| [p[0], 0.0, p[2]];
    let floor = g.level.routes.first().and_then(|r| g.tab.route[*r].points.first().copied()).unwrap_or(g.level.player_spawn);
    let mut firsts: Vec<V3> = vec![];
    let mut fan0: Option<V3> = None;
    for (k, y) in r.get("ways").items().iter().enumerate() {
        let w = format!("the response's way {} '{}'", k + 1, y.get("name").str().unwrap_or("?"));
        let (muster, entry) = (pts(y.get("muster")), pts(y.get("entry")));
        let door = y.get("door").str().map(|n| (n, g.doors.list.iter().copied().find(|d| g.dr[*d].def.name == n)));
        if let Some((n, None)) = door { bad.push(format!("{w}: no door '{n}' on this level")); }
        for (what, ps) in [("muster", &muster), ("fan", &entry)] {
            for (i, p) in ps.iter().enumerate() { if !free(g, *p) { bad.push(format!("{w}: {what} mark {} is not free floor", i + 1)); } for q in &ps[i + 1..] { if hyp(*p, *q) < 0.6 { bad.push(format!("{w}: two {what} marks are {:.2} m apart", hyp(*p, *q))); } } }
        }
        for (i, (p, e)) in muster.iter().zip(&entry).enumerate() { if free(g, *p) && free(g, *e) { match walk_len(g, flat(*p), flat(*e)) { Some(l) if l <= 30.0 => {} Some(l) => bad.push(format!("{w}: man {}'s walk in is {l:.1} m", i + 1)), None => bad.push(format!("{w}: man {} has no walk in", i + 1)) } } }
        if let Some(e) = entry.first() { if free(g, *e) && walk_len(g, flat(*e), floor).is_none() { bad.push(format!("{w}: no walk from its fan to the level's floor")); } }
        if let (Some((n, Some(d))), Some(m), Some(e)) = (door, muster.first(), entry.first()) { if game::doors::doors_pull_face(g, d, *m) == game::doors::doors_pull_face(g, d, *e) { bad.push(format!("{w}: its muster and its fan are on the same side of '{n}'")); } }
        firsts.extend(muster.first());
        if fan0.is_none() { fan0 = entry.first().copied(); }
    }
    for s in sc.get("quiet").get("starts").items() { if let Some(p) = s.get("sam").v3() { if !firsts.iter().any(|m| hyp(p, *m) >= RESPONSE_CLEAR) { bad.push("a start with every way's muster beside it".into()); } } }
    let outside = pts(r.get("outside"));
    for (i, p) in outside.iter().enumerate() { if !free(g, *p) { bad.push(format!("the response's outside mark {} is not free floor", i + 1)); } }
    for (i, p) in r.get("posts").items().iter().enumerate() { let Some(q) = p.get("pos").v3() else { continue }; if !free(g, q) { bad.push(format!("the response's post {} is not free floor", i + 1)); } else if !outside.iter().any(|o| free(g, *o) && walk_len(g, *o, q).is_some()) { bad.push(format!("the response's post {} has no walk from an outside mark", i + 1)); } }
    for (i, p) in r.get("sweep").items().iter().enumerate() { let Some(q) = p.get("pos").v3() else { continue }; if !free(g, q) { bad.push(format!("the response's place {} is not free floor", i + 1)); } else if let Some(e) = fan0 { if walk_len(g, flat(e), q).is_none() { bad.push(format!("the response's place {} has no walk from its first fan", i + 1)); } } }
}
fn stands(levels: &[String]) -> i32 {
    let mut v = Verdicts::default();
    let levels: Vec<String> = if levels.is_empty() { every_level() } else { levels.to_vec() };
    for level in &levels {
        let data = data_of(level);
        let (mut bad, mut said): (Vec<String>, Vec<String>) = (vec![], vec![]);
        for (key, words) in kinds(&data) {
            let mut p = Play::mission(&format!("{level}.json"), &format!("?levelFile={level}.json&{words}"), &Opts { squad: 2, ..Default::default() });
            let (on, faults) = match key {
                "roomClear" => (game::roomclear::room_clear_scenario_active(&p.g), game::roomclear::check_structure(&mut p.g, None)),
                "rescue" => (p.g.rescue.active, game::rescue::check_rescue(&mut p.g, None)),
                "nightBreach" => (p.g.night_breach.active, vec![]),
                _ => { let on = p.g.quiet.active; drop(p); p = Play::level(&format!("{level}.json"), false, &Opts::default()); (on, game::quiet::check_quiet(&mut p.g)) }
            };
            if !on { bad.push(format!("{key}: it did not start")); }
            for f in faults { if f.contains(": standing in ") { said.push(f); } else { bad.push(format!("{key}: {f}")); } }
        }
        let mut p = Play::level(&format!("{level}.json"), false, &Opts::default());
        let g: &mut Game = &mut p.g;
        let free = |g: &Game, q: V3| !game::scene_collision::nav_grid_is_blocked(g, q[0], q[2]);
        let spawn = if breaks() { g.tab.boxes[g.col.boxes.iter().copied().find(|b| { let b = &g.tab.boxes[*b]; b.h[1] > 0.9 && b.c[1] < 2.0 && (b.flags as i64 & 2) == 0 }).expect("a wall")].c } else { g.level.player_spawn };
        if !free(g, spawn) { bad.push(format!("the spawn ({}, {}) is not on floor", spawn[0], spawn[2])); }
        let mut from = spawn;
        for o in data.get("mission").get("objectives").items() {
            let n = |k: &str| o.get(k).num().unwrap_or(0.0);
            let to: V3 = match o.get("kind").str() { Some("reach") => { let a = o.get("area"); let e = |k: &str| a.get(k).num().unwrap_or(0.0); [(e("x0") + e("x1")) / 2.0, 0.0, (e("z0") + e("z1")) / 2.0] } Some("drive") => { let f = o.get("front").nums(); [f[0], 0.0, f[2] + 0.6] } _ => [n("x"), 0.0, n("z")] };
            let id = o.get("id").str().unwrap_or("?");
            match floor_of(g, to).filter(|q| hyp(*q, to) <= 1.5) { None => bad.push(format!("the objective '{id}' has no floor within 1.5 m")), Some(q) => { if walk_len(g, from, q).is_none() { bad.push(format!("no way on foot to the objective '{id}'")); } from = q; } }
        }
        for r in g.level.routes.clone() {
            let (name, pts) = { let r = &g.tab.route[r]; (r.name.clone(), r.points.clone()) };
            for (i, q) in pts.iter().enumerate() { if !free(g, *q) { if floor_of(g, *q).is_some() { said.push(format!("the route '{name}' stands its point {i} off free floor")); } else { bad.push(format!("the route '{name}' has no floor at its point {i}")); } } }
            for i in 0..pts.len() { let j = (i + 1) % pts.len(); if i == j { break; } if let (Some(a), Some(b)) = (floor_of(g, pts[i]), floor_of(g, pts[j])) { if walk_len(g, a, b).is_none() { bad.push(format!("the route '{name}' has no way from its point {i} to its point {j}")); } } }
        }
        response(g, data.get("scenarios"), &mut bad);
        // a walkable cell of the building with no slab under it is a man walking on nothing
        let s = data.get("surfaces").get("building");
        if !s.is_null() {
            let e = |k: &str| s.get(k).num().unwrap_or(0.0);
            let slabs: Vec<(V3, V3, f64)> = g.col.boxes.iter().map(|b| &g.tab.boxes[*b]).filter(|b| { let top = b.c[1] + b.h[1]; (-0.6..=0.3).contains(&top) }).map(|b| (b.c, b.h, b.yaw)).collect();
            let mut holes: Vec<String> = vec![];
            let mut z = e("z0") + 0.25;
            while z < e("z1") { let mut x = e("x0") + 0.25; while x < e("x1") {
                if !game::scene_collision::nav_grid_is_blocked(g, x, z) && !slabs.iter().any(|(c, h, yaw)| { let (dx, dz, k, n) = (x - c[0], z - c[2], yaw.cos(), yaw.sin()); (k * dx - n * dz).abs() <= h[0] && (n * dx + k * dz).abs() <= h[2] }) { holes.push(format!("({x:.2}, {z:.2})")); }
                x += 0.5; } z += 0.5; }
            if !holes.is_empty() { bad.push(format!("{} walkable cells with no floor under them: {}", holes.len(), holes.iter().take(8).cloned().collect::<Vec<_>>().join(" "))); }
        }
        for c in data.get("chokepoints").items() { if let Some(q) = c.get("pos").v3() { if !free(g, q) { said.push(format!("the chokepoint '{}' is off free floor", c.get("id").str().unwrap_or("?"))); } } }
        v.say(bad.is_empty(), &format!("{level}: what it names stands on its floor and can be walked to"), &format!("{} missions, {} routes{}{}", kinds(&data).len(), p.g.level.routes.len(), list(&bad), if said.is_empty() { String::new() } else { format!("\n    said, and passes: {}", said.join(" · ")) }));
    }
    v.end("stands")
}

// ---------------------------------------------------------------------------------------------------------------- doors
// tools/levels/lib/doorSize.ts DOOR_SIZE: the hole, its head, the widest that is a door's, the leaf
const GAP: f64 = 0.9;
const HEAD: f64 = 2.05;
const SLOT_MAX: f64 = 1.3;
const LEAF_W: f64 = GAP - 0.04;
const LEAF_H: f64 = HEAD - 0.04;
fn doors(levels: &[String], rows: bool) -> i32 {
    let mut v = Verdicts::default();
    let levels: Vec<String> = if levels.is_empty() { every_level() } else { levels.to_vec() };
    let (mut leaves, mut bare, mut wide) = (0, 0, 0);
    for level in &levels {
        let data = data_of(level);
        let mut p = Play::level(&format!("{level}.json"), false, &Opts::default());
        let g: &mut Game = &mut p.g;
        g.col.dynamic_boxes.clear(); game::scene_collision::wrote(g);   // (the frames are measured with no leaf in them)
        let air = |g: &mut Game, p: V3, d: V3, max: f64| game::scene_collision::raycast(g, p, d, max).map_or(max, |h| h.t);
        let mut bad: Vec<String> = vec![];
        let mut mids: Vec<[f64; 2]> = vec![];
        for (k, d) in g.level.doors.clone().into_iter().enumerate() {
            leaves += 1;
            let u = [d.closed_dir.cos(), d.closed_dir.sin()]; let n = [-u[1], u[0]];
            let width = if breaks() { d.width + 0.2 } else { d.width };
            let m = [d.hinge[0] + u[0] * d.width / 2.0, d.hinge[1] + u[1] * d.width / 2.0]; mids.push(m);
            let clear = air(g, [m[0], 1.0, m[1]], [u[0], 0.0, u[1]], 3.0) + air(g, [m[0], 1.0, m[1]], [-u[0], 0.0, -u[1]], 3.0);
            let head = 0.1 + air(g, [m[0], 0.1, m[1]], [0.0, 1.0, 0.0], 6.0);
            let tall = data.get("doors").items().get(k).and_then(|x| x.get("height").num()).unwrap_or(LEAF_H);
            let mut why: Vec<String> = vec![];
            if (width - LEAF_W).abs() > 0.005 { why.push(format!("a leaf {width:.2} m wide, the rule's is {LEAF_W:.2}")); }
            if (clear - width).abs() > 0.03 { why.push(format!("{clear:.2} m between the jamb faces for a {width:.2} m leaf")); }
            if head > HEAD + 0.01 || head < tall - 0.005 { why.push(format!("{head:.2} m under the head, the rule's is {tall:.2} to {HEAD:.2}")); }
            if head > tall + 0.01 + 1e-4 { why.push(format!("a {:.1} cm slot between the shut leaf's top and the head", (head - tall - 0.01) * 100.0)); }
            if (tall - LEAF_H).abs() > 0.005 { why.push(format!("a leaf {tall:.2} m tall, the rule's is {LEAF_H:.2}")); }
            for o in [-0.6, 0.6] { if game::scene_collision::nav_grid_is_blocked(g, m[0] + n[0] * o, m[1] + n[1] * o) { why.push(format!("the floor a stride {} the frame's middle is not free", if o < 0.0 { "behind" } else { "before" })); } }
            // the guards' own search from a metre out on one side to a metre out on the other comes through the doorway
            let (a, b) = ([m[0] - n[0], 0.0, m[1] - n[1]], [m[0] + n[0], 0.0, m[1] + n[1]]);
            let through = game::scene_collision::nav_grid_find_path(g, a, b, None, None).is_some_and(|w| { let (mut len, mut far, mut q) = (0.0, 0.0f64, a); for p in w { len += hyp(q, p); far = far.max((p[0] - m[0]).hypot(p[2] - m[1])); q = p; } len < 3.5 && far < 1.8 });
            if !through { why.push("the walk grid does not pass through the frame".into()); }
            let mut s = 0.15;
            while s < d.width - 0.1 { let mut q = [d.hinge[0] + u[0] * s, 0.0, d.hinge[1] + u[1] * s]; if game::scene_collision::collide_circle(g, &mut q, 0.02, 0.4, 1.9, Some(1.0)) { why.push("a thing of the level stands in the doorway".into()); break; } s += 0.2; }
            for (end, s) in [("hinge", -0.14), ("latch", d.width + 0.14)] { let mut q = [d.hinge[0] + u[0] * s, 0.0, d.hinge[1] + u[1] * s]; if !game::scene_collision::collide_circle(g, &mut q, 0.06, 1.0, 1.6, Some(1.0)) { why.push(format!("no wall meets the leaf's {end} end: shut, it does not seal")); } }
            if rows { println!("      {level} {}: air {clear:.2} by {head:.2}, leaf {width:.2} by {tall:.2}", d.name); }
            if !why.is_empty() { bad.push(format!("{}: {}", d.name, why.join("; "))); }
        }
        let flag = game::scene_boxes::box_flag();
        let skip = flag.no_shadow as i64 | flag.dynamic as i64 | flag.glass as i64;
        for bi in 0..g.col.boxes.len() {
            let (c, h, yaw, flags) = { let b = &g.tab.boxes[g.col.boxes[bi]]; (b.c, b.h, b.yaw, b.flags as i64) };
            if flags & skip != 0 || (2.0 * yaw).sin().abs() > 1e-3 { continue; }
            let (y0, y1) = (c[1] - h[1], c[1] + h[1]);
            if y0 < 1.9 || y0 > 2.3 || y1 < y0 + 0.1 { continue; }
            let turned = yaw.sin().abs() > 0.5; let (hx, hz) = if turned { (h[2], h[0]) } else { (h[0], h[2]) };
            let (thin, long) = (hx.min(hz), hx.max(hz));
            if thin > 0.13 || thin < 0.05 || long < 0.3 || long > 2.6 { continue; }
            if mids.iter().any(|q| (q[0] - c[0]).hypot(q[1] - c[2]) < 0.5) { continue; }   // (a leaf's own head)
            if game::scene_collision::raycast(g, [c[0], y0 - 0.02, c[2]], [0.0, -1.0, 0.0], y0 - 0.3).is_some() { continue; }   // (something stands under it: no doorway)
            let u = if hx > hz { [1.0, 0.0] } else { [0.0, 1.0] };
            let clear = air(g, [c[0], 1.0, c[2]], [u[0], 0.0, u[1]], 4.0) + air(g, [c[0], 1.0, c[2]], [-u[0], 0.0, -u[1]], 4.0);
            if (clear - 2.0 * long).abs() > 0.2 { continue; }   // (the air under it is not its own opening)
            if clear > SLOT_MAX + 0.01 { wide += 1; continue; }
            bare += 1;
            let mut why: Vec<String> = vec![];
            if clear > GAP + 0.01 { why.push(format!("a doorway {clear:.2} m wide, the rule's is {GAP:.2}")); }
            if y0 > HEAD + 0.01 { why.push(format!("{y0:.2} m under the head, the rule's is {HEAD:.2}")); }
            if game::scene_collision::nav_grid_is_blocked(g, c[0], c[2]) { why.push("no free walk cell in it".into()); }
            if !why.is_empty() { bad.push(format!("the doorway at {:.2}, {:.2}: {}", c[0], c[2], why.join("; "))); }
        }
        v.say(bad.is_empty(), &format!("{level}: every door is a real door's size, and the walk grid goes through it"), &format!("{} leaves{}", mids.len(), list(&bad)));
    }
    println!("    {} levels, {leaves} leaves, {bare} doorways without a leaf, {wide} wider openings left as drawn", levels.len());
    v.end("doors")
}

// ---------------------------------------------------------------------------------------------------------------- charges
fn charges(levels: &[String], write: bool) -> i32 {
    let mut v = Verdicts::default();
    let levels: Vec<String> = if levels.is_empty() { every_level().into_iter().filter(|l| counts("charges", l).exists() || game::scene_scenario_list::all_missions().iter().any(|e| level_of(e) == *l)).collect() } else { levels.to_vec() };
    for level in &levels {
        let clear = !data_of(level).get("scenarios").get("roomClear").is_null();
        let mut p = if clear { Play::mission(&format!("{level}.json"), &format!("?levelFile={level}.json&roomclear&squad4"), &Opts::default()) } else { Play::level(&format!("{level}.json"), false, &Opts::default()) };
        p.g.ai_enabled = false;
        for _ in 0..30 { p.frame(); }
        let g: &mut Game = &mut p.g;
        let mut places: BTreeSet<(usize, i64, i64)> = BTreeSet::new();
        for bi in 0..(g.destr.walls.authored as usize).min(g.col.boxes.len()) {
            let (c, h) = { let b = &g.tab.boxes[g.col.boxes[bi]]; (b.c, b.h) };
            if h[1] < 0.9 || h[0].min(h[2]) > 0.2 || h[0].max(h[2]) < 0.75 { continue; }
            let along_x = h[0] >= h[2]; let l = if along_x { h[0] } else { h[2] };
            let mut u = -l + 0.1;
            while u <= l - 0.1 + 1e-6 {
                for side in [1.0, -1.0] {
                    let at: V3 = if along_x { [c[0] + u, 0.0, c[2]] } else { [c[0], 0.0, c[2] + u] };
                    let from: V3 = if along_x { [at[0], 0.0, c[2] + side * 2.0] } else { [c[0] + side * 2.0, 0.0, at[2]] };
                    let Some(w0) = game::wallcharge::wall_at(g, at, Some(from)) else { continue };
                    if w0.index as usize != bi || !w0.ok { continue; }
                    let w = game::wallcharge::wall_charge_settle(g, &w0);
                    if game::wallcharge::wall_charge_set_spot(g, &w).is_none() || game::wallcharge::wall_charge_open_at(g, &w) { continue; }
                    if game::wallcharge::wall_charge_far_floor(g, &w, Some(false)).is_some() { places.insert((bi, w.side as i64, (w.u * 1000.0).round() as i64)); }
                }
                u += 0.25;
            }
        }
        let n = places.len() - usize::from(breaks());
        let file = held("charges", level);
        let want: Option<usize> = file.iter().flatten().filter(|l| !l.starts_with('#')).find_map(|l| l.parse().ok());
        if write && !breaks() {
            let notes: String = std::fs::read_to_string(counts("charges", level)).unwrap_or_default().lines().filter(|l| l.starts_with('#')).map(|l| format!("{l}\n")).collect();
            let notes = if notes.is_empty() { format!("# places for a wall charge on levels/{level}.json (floor behind them as the walk grid stands)\n") } else { notes };
            keep("charges", level, &format!("{notes}{n}\n"));
        }
        v.say(write || want == Some(n), &format!("{level}: it has the places for a wall charge its file holds"), &format!("{n} places, the file holds {}{}", want.map_or("none".into(), |w| w.to_string()), match want { Some(w) if n < w => " · FEWER: something took a place where a charge could be set", Some(w) if n > w => " · MORE: --write keeps it", None => " (--write)", _ => "" }));
    }
    v.end("charges")
}

// ---------------------------------------------------------------------------------------------------------------- names
/// every room the office's tables name: a word is wrong only on a level that names no such place itself
const OFFICE_WORDS: [&str; 15] = ["server room", "back offices", "car park", "cubicles", "break room", "lobby", "corridor", "manager's office", "conference room", "storage room", "storage", "south lot", "east side", "north side", "west side"];
fn bare(n: &str) -> String { n.strip_prefix("the ").unwrap_or(n).to_lowercase() }
fn word_in(text: &str, w: &str) -> bool { let t = text.to_lowercase(); t.match_indices(w).any(|(i, _)| { let edge = |c: Option<char>| !c.is_some_and(|c| c.is_ascii_lowercase()); edge(t[..i].chars().last()) && edge(t[i + w.len()..].chars().next()) }) }
fn names(levels: &[String]) -> i32 {
    let mut v = Verdicts::default();
    let named = !levels.is_empty();
    let levels: Vec<String> = if named { levels.to_vec() } else { every_level() };
    for level in &levels {
        let data = data_of(level);
        let sc = data.get("scenarios");
        let office = ["corridor", "cubicles", "lobby_break"].iter().all(|n| data.get("routes").items().iter().any(|r| r.get("id").str().or(r.get("name").str()) == Some(n)));
        if office || sc.is_null() { continue; }
        let rect = |b: &Json| { let e = |k: &str| b.get(k).num().unwrap_or(0.0); [e("x0"), e("x1"), e("z0"), e("z1")] };
        let mut rects: Vec<(String, [f64; 4])> = vec![];
        for r in sc.get("roomClear").get("rooms").items().iter().chain(sc.get("places").get("areas").items()) { rects.push((bare(r.get("name").str().unwrap_or("?")), rect(r.get("bounds")))); }
        for q in sc.get("quiet").get("spots").items() { rects.push((bare(q.get("place").str().unwrap_or("?")), rect(q.get("area")))); }
        let own: BTreeSet<String> = rects.iter().map(|r| r.0.clone()).collect();
        let foreign: Vec<&str> = OFFICE_WORDS.into_iter().filter(|w| breaks() || !own.iter().any(|o| o == w || o.ends_with(&format!(" {w}")))).collect();
        let p = Play::level(&format!("{level}.json"), false, &Opts::default());
        let g: &Game = &p.g;
        let legal = |how: &str, text: &str| if own.is_empty() { bare(text) == "building" } else { own.contains(&bare(text)) || (how == "a line's place" && own.iter().any(|o| text.to_lowercase() == format!("outside the {o}") || text.to_lowercase() == format!("outside {o}"))) };
        let (mut x0, mut x1, mut z0, mut z1) = (f64::INFINITY, -f64::INFINITY, f64::INFINITY, -f64::INFINITY);
        for b in data.get("boxes").items() { let (c, h) = (b.get("c").nums(), b.get("h").nums()); x0 = x0.min(c[0] - h[0]); x1 = x1.max(c[0] + h[0]); z0 = z0.min(c[2] - h[2]); z1 = z1.max(c[2] + h[2]); }
        let (mut wrong, mut pts): (BTreeMap<String, String>, usize) = (BTreeMap::new(), 0);
        let mut z = z0.max(-20.0) + 0.25;
        while z < z1.min(80.0) {
            let mut x = x0.max(-20.0) + 0.25;
            while x < x1.min(80.0) {
                pts += 1;
                let at = [x, 0.0, z];
                for (how, text) in [("a line's room", game::dialogue::room_name(g, at)), ("a line's place", game::dialogue::place_name(g, at)), ("the debrief", game::chronicle::room_of(g, at))] {
                    if foreign.iter().any(|w| word_in(&text, w)) || !legal(how, &text) { wrong.entry(format!("{how} says '{text}'")).or_insert(format!("({x:.1}, {z:.1})")); }
                }
                x += 0.5;
            }
            z += 0.5;
        }
        for (name, r) in &rects {
            let c = [(r[0] + r[1]) / 2.0, 0.0, (r[2] + r[3]) / 2.0];
            let first = &rects.iter().find(|q| c[0] >= q.1[0] && c[0] <= q.1[1] && c[2] >= q.1[2] && c[2] <= q.1[3]).unwrap().0;   // (two over one spot: the first listed is its name)
            for (how, got) in [("a line", bare(&game::dialogue::room_name(g, c))), ("the debrief", bare(&game::chronicle::room_of(g, c)))] { if got != *first { wrong.insert(format!("the middle of {name}: {how} says '{got}'"), format!("({:.1}, {:.1})", c[0], c[2])); } }
        }
        let bad: Vec<String> = wrong.into_iter().map(|(k, at)| format!("{k} at {at}")).collect();
        v.say(bad.is_empty(), &format!("{level}: it is named by its own rooms"), &format!("{pts} points, {} names{}", own.len(), list(&bad)));
    }
    v.end("names")
}

// ---------------------------------------------------------------------------------------------------------------- rain
fn rain(levels: &[String]) -> i32 {
    let mut v = Verdicts::default();
    let levels: Vec<String> = if levels.is_empty() { every_level() } else { levels.to_vec() };
    let wd = game::scene_world::world();
    let ceiling = 3.0;   // (src/scene/world.ts WORLD.ceilingY)
    let (mut walls, mut faces, mut inners) = (0, 0, 0);
    for level in &levels {
        let mut p = Play::level(&format!("{level}.json"), false, &Opts { dress: true, ..Default::default() });
        for _ in 0..3 { p.frame(); }
        let g: &mut Game = &mut p.g;
        if !g.weather.own { continue; }   // (a level whose roofs are the office's two, or a still night)
        // rain shows in front of an outer wall's outer face and not of its inner one: the picture hides it from a point 0.2 m or more inside the roofs on every side (common.wgsl wxUnderRoof)
        let roofs = g.weather.roof_rects.clone();
        let under_any = |x: f64, z: f64| roofs.iter().any(|r| x >= r[0] && z >= r[1] && x <= r[2] && z <= r[3]);
        let hidden = |x: f64, z: f64| under_any(x, z) && (breaks() || [[0.2, 0.2], [-0.2, 0.2], [0.2, -0.2], [-0.2, -0.2]].iter().all(|d| under_any(x + d[0], z + d[1])));
        for b in &g.col.boxes {
            let (c, h) = { let b = &g.tab.boxes[*b]; (b.c, b.h) };
            if h[1] < 1.0 || c[1] + h[1] < ceiling - 0.01 { continue; }
            let along = if h[0] < 0.2 && h[2] > 0.3 { 0 } else if h[2] < 0.2 && h[0] > 0.3 { 2 } else { continue };
            for s in [-1.0, 1.0] {
                let at = |d: f64| if along == 0 { [c[0] + s * d, c[2]] } else { [c[0], c[2] + s * d] };
                let (t, sky, deep) = (h[along], at(h[along] + 0.5), at(-h[along] - 0.5));
                if under_any(sky[0], sky[1]) || !under_any(deep[0], deep[1]) { continue; }
                walls += 1;
                let (o, i) = (at(t), at(-t));
                if !hidden(o[0], o[1]) { faces += 1; }
                if hidden(i[0], i[1]) { inners += 1; }
            }
        }
        let (mut open, mut dry, mut roofed, mut rained) = (0, 0, 0, 0);
        let under = |g: &mut Game, x: f64, z: f64| if breaks() { x > 4.0 && x < 36.0 && z > 4.0 && z < 24.0 } else { game::weather::weather_under_roof(g, x, z) };
        let blocked = |g: &mut Game, a: V3, b: V3| game::scene_collision::solid_blocked(g, a, b, None);
        let mut x = wd.min[0] + 1.5;
        while x < wd.max[0] - 1.0 {
            let mut z = wd.min[2] + 1.5;
            while z < wd.max[2] - 1.0 {
                let sky = !blocked(g, [x, 0.3, z], [x, ceiling + 6.0, z]);
                let slab = !sky && !blocked(g, [x, 0.3, z], [x, ceiling - 0.4, z]) && [-0.45, 0.45].iter().all(|d| blocked(g, [x + d, ceiling - 0.4, z + d], [x + d, ceiling + 0.6, z + d]));
                let u = under(g, x, z);
                if sky { open += 1; if u && [[-0.5, 0.0], [0.5, 0.0], [0.0, -0.5], [0.0, 0.5]].iter().all(|d| under(g, x + d[0], z + d[1])) { dry += 1; } }   // (half a metre of slack at a roof's edge)
                else if slab { roofed += 1; if !u { rained += 1; } }
                z += 1.0;
            }
            x += 1.0;
        }
        v.say(dry == 0 && rained as f64 <= (roofed as f64 * 0.02).ceil(), &format!("{level}: open ground is rained on, and ground under a ceiling is not"), &format!("{open} points of open ground, {dry} kept dry; {roofed} under a ceiling, {rained} rained on"));
    }
    if levels.len() > 1 { v.say(walls >= 100 && faces as f64 >= walls as f64 * 0.85 && inners as f64 >= walls as f64 * 0.97, "rain shows in front of nearly every outer wall's face, and of next to no inner one", &format!("{walls} outer walls, {faces} faces shown, {inners} inner faces dry")); }
    v.end("rain")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = || -> ! { eprintln!("levels second-way [LEVEL…] [--list] [--walls] [--nolists] | sight [LEVEL…] [--write] | walked [LEVEL…] [--write] | stacks [MISSION…] | stands | doors [--list] | charges [--write] | names | rain [LEVEL…]"); std::process::exit(2) };
    let Some(ask) = args.first() else { usage() };
    let allowed: &[&str] = match ask.as_str() { "second-way" => &["--list", "--walls", "--nolists"], "sight" | "walked" | "charges" => &["--write"], "doors" => &["--list"], "stacks" | "stands" | "names" | "rain" => &[], _ => usage() };
    let (flags, names): (Vec<&String>, Vec<&String>) = args[1..].iter().partition(|a| a.starts_with("--"));
    if flags.iter().any(|f| !allowed.contains(&f.as_str())) { usage(); }
    let has = |f: &str| flags.iter().any(|x| *x == f);
    let names: Vec<String> = names.iter().map(|n| n.trim_end_matches(".json").to_string()).collect();
    let known: Vec<String> = if ask == "stacks" { missions().into_iter().map(|e| e.id).collect() } else { every_level() };
    for n in &names { if !known.contains(n) { eprintln!("levels: no such {} '{n}'", if ask == "stacks" { "mission" } else { "level" }); std::process::exit(2); } }
    std::process::exit(match ask.as_str() {
        "second-way" => second_way(&names, &SecondWay { list: has("--list"), walls: has("--walls"), nolists: has("--nolists") }),
        "sight" => sight(&names, has("--write")),
        "walked" => walked(&names, has("--write")),
        "stands" => stands(&names),
        "doors" => doors(&names, has("--list")),
        "charges" => charges(&names, has("--write")),
        "names" => self::names(&names),
        "rain" => rain(&names),
        _ => stacks(&names),
    });
}
