//! WHAT A LEVEL OWES WHOEVER PLAYS IT, asked of the game itself: natively, with no page and no TypeScript (tools/levels/export.ts asks two of these of every level it writes).
//!
//!   levels second-way [LEVEL…] [--list] [--walls] [--nolists]      every room the defenders can shut has another way in
//!   levels sight [LEVEL…] [--write]                                a defender sees what he watches
//!   levels walked [LEVEL…] [--write]                               its starts and its doors are on a way
//!   levels stacks [MISSION…]                                       every stack mark is by its door
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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = || -> ! { eprintln!("levels second-way [LEVEL…] [--list] [--walls] [--nolists] | sight [LEVEL…] [--write] | walked [LEVEL…] [--write] | stacks [MISSION…]"); std::process::exit(2) };
    let Some(ask) = args.first() else { usage() };
    let allowed: &[&str] = match ask.as_str() { "second-way" => &["--list", "--walls", "--nolists"], "sight" | "walked" => &["--write"], "stacks" => &[], _ => usage() };
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
        _ => stacks(&names),
    });
}
