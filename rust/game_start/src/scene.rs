//! THE SCENE BUILDER: a few lines in, A LEVEL FILE out. A scene is a level written by a program: rooms, walls, doors, lamps and men, said in
//! metres, become the records of docs/level-format.md, and `level_text()` is a file the TypeScript loader takes, the editor opens and the Rust
//! start stands like any level of `levels/`. So a test's scene is no second way to make a game: it goes in by the level format, through the one
//! start (`stand.rs`).
//!
//! The pieces are the level kit's own (`src/scene/levelKit.ts`, `doorSize.ts`), size for size: a wall 12 cm thick and 3 m high, a doorway cut for
//! the real door inside a 1.2 m slot (on a walk-grid cell centre), its frame of two jambs, a head and two stops, a bare bulb on its flex.
//!
//! WHO STANDS IN A SCENE, one of two ways and never both (the room clear's staging takes every other man off the floor):
//!   · a NIGHT: Sam at his spawn, guards on their routes (`sam`, `patrol`, `sentry`), as a level's file has them;
//!   · a ROOM CLEAR: the team on its marks and defenders posted in rooms (`team`, `defender`), started by the game's own `startScenario`.
use crate::json::{arr, b, n, nums, obj, s, Json};
use alloc::{format, string::String, vec, vec::Vec};
use game::m;

/// THE SIZES, from the game's own tables where it has them (`scene_world::world()`, `scene_door_size::door_size()`, written from the TypeScript:
/// a door rule or a world that changes there changes here), and the level kit's own two that are no table of the game (levelKit.ts WALL_INT,
/// DOOR_SLOT; doorSize.ts JAMB_FACE)
struct Sizes { world: [f64; 2], ceiling: f64, gap: f64, head: f64, cell: f64, reach: f64, leaf_w: f64, leaf_h: f64 }
fn sizes() -> Sizes {
    let (w, d) = (game::scene_world::world(), game::scene_door_size::door_size());
    Sizes { world: [w.max[0], w.max[2]], ceiling: w.ceiling_y, gap: d.gap, head: d.head, cell: d.cell, reach: d.reach, leaf_w: d.gap - 2.0 * JAMB_FACE, leaf_h: game::scene_door_size::LEAF_H }
}
const WALL_INT: f64 = 0.12;
const DOOR_SLOT: f64 = 1.2;
const JAMB_FACE: f64 = 0.02;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Axis {
    /// a wall that runs along X, at a fixed z
    X,
    /// a wall that runs along Z, at a fixed x
    Z,
}

fn r4(v: f64) -> f64 { m::round(v * 1e4) / 1e4 }
/// THE REAL DOORWAY INSIDE A SLOT (doorSize.ts fitSlot): the gap's two ends, its middle on a walk-grid cell centre where the slot allows
fn fit_slot(a: f64, bb: f64) -> [f64; 2] {
    let k = sizes();
    let (g, cell, reach) = (k.gap, k.cell, k.reach);
    let (lo, hi, mid) = (a + g / 2.0 - reach, bb - g / 2.0 + reach, (a + bb) / 2.0);
    let (mut best, mut best_d) = (f64::NAN, f64::INFINITY);
    let mut k = m::floor(lo / cell) - 1.0;
    while k <= m::ceil(hi / cell) + 1.0 {
        let c = (k + 0.5) * cell;
        let at = m::min(hi, m::max(lo, c));
        let out = m::max(0.0, m::max(a - (at - g / 2.0), at + g / 2.0 - bb));
        let d = m::abs(c - at) * 1000.0 + r4(out) * 10.0 + m::abs(at - mid);
        if d < best_d + 1e-9 { best_d = d; best = at; }
        k += 1.0;
    }
    [r4(best - g / 2.0), r4(best + g / 2.0)]
}

struct Wall { axis: Axis, line: f64, a: f64, b: f64, holes: Vec<[f64; 2]> }
struct Door { name: String, axis: Axis, line: f64, hinge: f64, locked: bool }
struct Bulb { x: f64, z: f64, on: bool }
struct Route { name: String, points: Vec<[f64; 2]>, wait: Vec<f64> }
struct Mark { pos: [f64; 2], watch: String }
struct Room { id: String, x0: f64, z0: f64, x1: f64, z1: f64, marks: Vec<Mark> }
struct Team { sam: [f64; 2], yaw: f64, ops: Vec<[f64; 2]> }

/// A scene, as it is being said. `level_text()` writes it down.
pub struct Scene {
    name: String,
    walls: Vec<Wall>,
    doors: Vec<Door>,
    bulbs: Vec<Bulb>,
    moon: bool,
    routes: Vec<Route>,
    rooms: Vec<Room>,
    team: Option<Team>,
    spawn: [f64; 2],
}

impl Scene {
    /// An empty lot at night: the ground, no light, Sam's spawn in its corner.
    pub fn new(name: &str) -> Scene {
        Scene { name: String::from(name), walls: Vec::new(), doors: Vec::new(), bulbs: Vec::new(), moon: false, routes: Vec::new(), rooms: Vec::new(), team: None, spawn: [2.25, 2.25] }
    }

    /// A wall along X at `z`, from `x0` to `x1`. A wall said twice over the same stretch is one wall.
    pub fn wall_x(&mut self, z: f64, x0: f64, x1: f64) -> &mut Self { self.wall(Axis::X, z, x0, x1) }
    /// A wall along Z at `x`, from `z0` to `z1`.
    pub fn wall_z(&mut self, x: f64, z0: f64, z1: f64) -> &mut Self { self.wall(Axis::Z, x, z0, z1) }
    fn wall(&mut self, axis: Axis, line: f64, a: f64, bb: f64) -> &mut Self {
        let (mut a, mut bb) = (m::min(a, bb), m::max(a, bb));
        // (a stretch that meets one already on the line joins it: two rooms side by side share their wall)
        let mut holes: Vec<[f64; 2]> = Vec::new();
        let mut i = 0;
        while i < self.walls.len() {
            let w = &self.walls[i];
            if w.axis == axis && m::abs(w.line - line) < 1e-9 && w.a <= bb + 1e-9 && w.b >= a - 1e-9 { a = m::min(a, w.a); bb = m::max(bb, w.b); holes.extend(w.holes.iter().copied()); self.walls.remove(i); } else { i += 1; }
        }
        self.walls.push(Wall { axis, line, a, b: bb, holes });
        self
    }

    /// A ROOM: four walls round the floor `x0..x1` by `z0..z1` (said by the walls' middle lines), under a ceiling. `id` is the name a defender's
    /// post and a test use for it.
    pub fn room(&mut self, id: &str, x0: f64, z0: f64, x1: f64, z1: f64) -> &mut Self {
        self.wall_x(z0, x0, x1).wall_x(z1, x0, x1).wall_z(x0, z0, z1).wall_z(x1, z0, z1);
        self.rooms.push(Room { id: String::from(id), x0, z0, x1, z1, marks: Vec::new() });
        self
    }

    /// A DOOR in the wall on `line` (a wall along `axis`), its 1.2 m slot starting at `at` along the wall: the doorway is cut and the frame built
    /// as the level kit does it, the leaf hung shut on the slot's low end.
    pub fn door(&mut self, name: &str, axis: Axis, line: f64, at: f64) -> &mut Self {
        let [lo, hi] = fit_slot(at, at + DOOR_SLOT);
        let w = self.walls.iter_mut().find(|w| w.axis == axis && m::abs(w.line - line) < 1e-9 && w.a <= lo + 1e-9 && w.b >= hi - 1e-9)
            .unwrap_or_else(|| panic!("scene: the door '{}' is said on the line {} at {}..{}, and no wall of the scene runs there", name, line, lo, hi));
        w.holes.push([lo, hi]);
        self.doors.push(Door { name: String::from(name), axis, line, hinge: r4(lo + JAMB_FACE), locked: false });
        self
    }
    /// The door `name` is locked.
    pub fn locked(&mut self, name: &str) -> &mut Self { self.door_mut(name).locked = true; self }
    fn door_mut(&mut self, name: &str) -> &mut Door { self.doors.iter_mut().find(|d| d.name == name).unwrap_or_else(|| panic!("scene: no door '{}'", name)) }
    /// the middle of a door's shut leaf, on the floor
    pub fn door_centre(&self, name: &str) -> [f64; 2] {
        let d = self.doors.iter().find(|d| d.name == name).unwrap_or_else(|| panic!("scene: no door '{}'", name));
        match d.axis { Axis::X => [d.hinge + sizes().leaf_w / 2.0, d.line], Axis::Z => [d.line, d.hinge + sizes().leaf_w / 2.0] }
    }

    /// A bare bulb on its flex at (x, z): a warm point light on the mains, lit.
    pub fn bulb(&mut self, x: f64, z: f64) -> &mut Self { self.bulbs.push(Bulb { x, z, on: true }); self }
    /// …and one that hangs there switched off (the room is dark until someone finds the switch).
    pub fn bulb_off(&mut self, x: f64, z: f64) -> &mut Self { self.bulbs.push(Bulb { x, z, on: false }); self }
    /// The moon over the lot (a level's own: a faint blue light from the south-west). Without it open ground is black.
    pub fn moon(&mut self) -> &mut Self { self.moon = true; self }

    /// Sam's spawn (a night: the man the hand moves).
    pub fn sam(&mut self, x: f64, z: f64) -> &mut Self { self.spawn = [x, z]; self }
    /// A guard who walks `points` in turn, waiting `wait` seconds at each.
    pub fn patrol(&mut self, name: &str, points: &[[f64; 2]], wait: f64) -> &mut Self {
        self.routes.push(Route { name: String::from(name), points: points.to_vec(), wait: vec![wait; points.len()] });
        self
    }
    /// A guard posted at `at`, looking toward `toward`: a route of two points a step apart with a wait at the first that outlasts any test (a
    /// route's man faces its next point; the level format has no word for a facing).
    pub fn sentry(&mut self, name: &str, at: [f64; 2], toward: [f64; 2]) -> &mut Self {
        let (dx, dz) = (toward[0] - at[0], toward[1] - at[1]);
        let l = m::hyp2(dx, dz);
        if !(l > 0.0) { panic!("scene: the sentry '{}' looks toward the place he stands on", name); }
        self.routes.push(Route { name: String::from(name), points: vec![at, [at[0] + dx / l * 0.5, at[1] + dz / l * 0.5]], wait: vec![1e6, 0.0] });
        self
    }

    /// The team's start for a room clear: Sam, the way he and the others face, and the teammates' marks (two or four). A yaw of 0 looks along +z
    /// (south), a quarter turn (π/2) along +x (east), π north: a man's forward is (sin yaw, cos yaw).
    pub fn team(&mut self, sam: [f64; 2], yaw: f64, ops: &[[f64; 2]]) -> &mut Self { self.team = Some(Team { sam, yaw, ops: ops.to_vec() }); self.spawn = sam; self }
    /// A defender posted in the room `room` at `at`, his gun on the door `watch` (one of that room's).
    pub fn defender(&mut self, room: &str, at: [f64; 2], watch: &str) -> &mut Self {
        let r = self.rooms.iter_mut().find(|r| r.id == room).unwrap_or_else(|| panic!("scene: no room '{}'", room));
        r.marks.push(Mark { pos: at, watch: String::from(watch) });
        self
    }
    /// is this a room clear (a team and posted defenders), not a night
    pub fn is_room_clear(&self) -> bool { self.team.is_some() }

    /// the rooms a door stands in the wall of: (room id, the other side's name)
    fn rooms_of_door(&self, d: &Door) -> Vec<(usize, String)> {
        let c = d.hinge + sizes().leaf_w / 2.0;
        let touching: Vec<usize> = self.rooms.iter().enumerate().filter(|(_, r)| match d.axis {
            Axis::X => (m::abs(r.z0 - d.line) < 1e-9 || m::abs(r.z1 - d.line) < 1e-9) && c > r.x0 && c < r.x1,
            Axis::Z => (m::abs(r.x0 - d.line) < 1e-9 || m::abs(r.x1 - d.line) < 1e-9) && c > r.z0 && c < r.z1,
        }).map(|(i, _)| i).collect();
        touching.iter().map(|&i| (i, touching.iter().find(|&&j| j != i).map(|&j| self.rooms[j].id.clone()).unwrap_or_else(|| String::from("outside")))).collect()
    }

    /// THE SCENE AS A LEVEL FILE'S VALUE (docs/level-format.md, version 1).
    pub fn level(&self) -> Json {
        let K = sizes();
        let (WORLD, KIT_H, DOOR_HEAD, LEAF_W, LEAF_H) = (K.world, K.ceiling, K.head, K.leaf_w, K.leaf_h);
        if self.doors.is_empty() { panic!("scene '{}': a level's mission names the door a man goes in by and the one he leaves by, so a scene has at least one door", self.name); }
        if self.team.is_some() && !self.routes.is_empty() { panic!("scene '{}': a room clear and guards on routes in one scene: the room clear's staging takes every man who is not its own off the floor. Post defenders in rooms, or say a night with no team", self.name); }
        if self.team.is_none() && self.rooms.iter().any(|r| !r.marks.is_empty()) { panic!("scene '{}': defenders are posted and no team is said: a defender is a room clear's man (`team`)", self.name); }
        let mut boxes: Vec<Json> = Vec::new();
        let mut add = |group: &str, c: [f64; 3], h: [f64; 3], paint: (&str, Json), more: Vec<(&str, Json)>| -> String {
            let id = format!("box_{:04}", boxes.len());
            let mut rec = vec![("id", s(&id)), ("group", s(group)), ("c", nums(&c)), ("h", nums(&h)), paint];
            rec.extend(more);
            boxes.push(obj(rec));
            id
        };
        let slab = |x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64| ([(x0 + x1) / 2.0, (y0 + y1) / 2.0, (z0 + z1) / 2.0], [m::abs(x1 - x0) / 2.0, m::abs(y1 - y0) / 2.0, m::abs(z1 - z0) / 2.0]);
        let mat = |name: &'static str| ("material", s(name));
        // the ground under everything, then each room's floor finish and ceiling
        add("ground", [WORLD[0] / 2.0, -0.125, WORLD[1] / 2.0], [WORLD[0] / 2.0, 0.125, WORLD[1] / 2.0], mat("asphalt"), vec![]);
        for r in self.rooms.iter() {
            let (c, h) = slab(r.x0, 0.0, r.z0, r.x1, 0.004, r.z1); add("floors", c, h, mat("carpet"), vec![]);
            let (c, h) = slab(r.x0, KIT_H, r.z0, r.x1, KIT_H + 0.2, r.z1); add("ceiling", c, h, mat("ceiling"), vec![("flags", arr(vec![s("NoPrimary")]))]);
        }
        // walls, with their doorways (levelKit.ts wallX / wallZ)
        let t = WALL_INT;
        for w in self.walls.iter() {
            let mut holes = w.holes.clone();
            holes.sort_by(|p, q| p[0].partial_cmp(&q[0]).unwrap());
            let mut piece = |a: f64, bb: f64, y0: f64, y1: f64| { let (c, h) = match w.axis { Axis::X => slab(a, y0, w.line - t / 2.0, bb, y1, w.line + t / 2.0), Axis::Z => slab(w.line - t / 2.0, y0, a, w.line + t / 2.0, y1, bb) }; add("walls", c, h, mat("wall"), vec![]); };
            let mut cur = w.a;
            for o in holes.iter() {
                if o[0] > cur { piece(cur, o[0], 0.0, KIT_H); }
                piece(o[0], o[1], DOOR_HEAD, KIT_H);
                cur = o[1];
            }
            if cur < w.b { piece(cur, w.b, 0.0, KIT_H); }
        }
        // each door's frame: two jambs, the head, the two stops that close the sight slots at the hinge and the latch (levelKit.ts door)
        let frame = ("albedo", nums(&[0.3, 0.24, 0.17]));
        for d in self.doors.iter() {
            let group = format!("doorframe.{}", d.name);
            let (a, gap, hy, w) = (d.hinge - JAMB_FACE, LEAF_W + 2.0 * JAMB_FACE, DOOR_HEAD / 2.0, d.line);
            let mut along = |u: f64, y: f64, hu: f64, hyy: f64, hw: f64| { let (c, h) = match d.axis { Axis::X => ([u, y, w], [hu, hyy, hw]), Axis::Z => ([w, y, u], [hw, hyy, hu]) }; add(&group, c, h, frame.clone(), vec![]); };
            along(a - 0.01, hy, 0.03, hy, t / 2.0 + 0.012);
            along(a + gap + 0.01, hy, 0.03, hy, t / 2.0 + 0.012);
            let hb = LEAF_H + 0.005;
            along(a + gap / 2.0, (hb + DOOR_HEAD + 0.065) / 2.0, gap / 2.0 + 0.04, (DOOR_HEAD + 0.065 - hb) / 2.0, t / 2.0 + 0.012);
            let mut stop = |u0: f64, u1: f64| along(d.hinge + (u0 + u1) / 2.0, hy, (u1 - u0) / 2.0, hy, 0.02);
            stop(-0.012, 0.024);
            stop(LEAF_W - 0.004, LEAF_W + 0.012);
        }
        // the lamps: a flex and a bulb each, the bulb the light's fixture (levelKit.ts bulb)
        let mut lights: Vec<Json> = Vec::new();
        if self.moon {
            lights.push(obj(vec![("id", s("moon")), ("kind", s("dir")), ("pos", nums(&[0.0, 0.0, 0.0])), ("dir", nums(&[0.44997750168735945, -0.49997500187484384, -0.7399630027747688])), ("color", nums(&[0.55, 0.7, 1.0])), ("intensity", n(0.9)),
                ("range", n(1000000.0)), ("innerDeg", n(0.0)), ("outerDeg", n(0.0)), ("group", s("exterior")), ("enabled", b(true)), ("volumetric", n(0.0)), ("radius", n(0.009)), ("mains", b(false))]));
        }
        for (i, l) in self.bulbs.iter().enumerate() {
            add("lights", [l.x, 2.7, l.z], [0.005, 0.3, 0.005], mat("black"), vec![("flags", arr(vec![s("NoShadow")]))]);
            let fixture = add("lights", [l.x, 2.36, l.z], [0.04, 0.05, 0.04], mat("white"), vec![("emissive", nums(&if l.on { [30.0, 22.0, 12.0] } else { [0.0, 0.0, 0.0] })), ("flags", arr(vec![s("NoShadow")]))]);
            lights.push(obj(vec![("id", s(&format!("bulb{}", i))), ("kind", s("point")), ("pos", nums(&[l.x, 2.33, l.z])), ("dir", nums(&[0.0, -1.0, 0.0])), ("color", nums(&[1.0, 0.7, 0.42])), ("intensity", n(28.0)), ("range", n(7.0)),
                ("innerDeg", n(0.0)), ("outerDeg", n(180.0)), ("group", s("room")), ("enabled", b(l.on)), ("volumetric", n(0.25)), ("radius", n(0.06)), ("fixture", s(&fixture))]));
        }
        // the mains breaker's panel and lamp, on a post in the lot's far corner (a level always has one)
        let panel = add("breaker", [WORLD[0] - 1.0, 1.35, WORLD[1] - 0.94], [0.28, 0.36, 0.04], ("albedo", nums(&[0.42, 0.44, 0.45])), vec![("name", s("breaker_panel")), ("stuff", s("sheet"))]);
        let led = add("breaker", [WORLD[0] - 1.18, 1.62, WORLD[1] - 0.99], [0.015, 0.015, 0.01], mat("black"), vec![("emissive", nums(&[0.3, 4.0, 0.6])), ("flags", arr(vec![s("NoShadow")]))]);
        let doors: Vec<Json> = self.doors.iter().map(|d| {
            let mut rec = vec![("id", s(&d.name)), ("hinge", nums(&match d.axis { Axis::X => [d.hinge, d.line], Axis::Z => [d.line, d.hinge] })), ("closedDir", n(match d.axis { Axis::X => 0.0, Axis::Z => m::PI / 2.0 })), ("width", n(LEAF_W))];
            if d.locked { rec.push(("locked", b(true))); }
            obj(rec)
        }).collect();
        let routes: Vec<Json> = self.routes.iter().map(|r| obj(vec![("id", s(&r.name)), ("points", arr(r.points.iter().map(|p| nums(&[p[0], 0.0, p[1]])).collect())), ("wait", nums(&r.wait))])).collect();
        let first = &self.doors[0];
        let last = &self.doors[self.doors.len() - 1];
        let area = self.rooms.first().map(|r| [r.x0, r.x1, r.z0, r.z1]).unwrap_or([0.0, 1.0, 0.0, 1.0]);
        let rect = |r: [f64; 4]| obj(vec![("x0", n(r[0])), ("x1", n(r[1])), ("z0", n(r[2])), ("z1", n(r[3]))]);
        let mut top = vec![
            ("version", n(1.0)), ("name", s(&self.name)),
            ("meta", obj(vec![("bounds", obj(vec![("min", nums(&[0.0, -0.25, 0.0])), ("max", nums(&[WORLD[0], 8.0, WORLD[1]]))])), ("generator", s("rust/game_start scene.rs")), ("notes", s("A scene a test wrote: a level file like any other."))])),
            ("floors", arr(vec![obj(vec![("id", s("f0")), ("name", s("ground floor")), ("y", n(0.0)), ("ceiling", n(KIT_H))])])),
            ("materials", obj(vec![
                ("asphalt", obj(vec![("albedo", nums(&[0.09, 0.09, 0.1])), ("rough", n(0.92)), ("stuff", s("tarmac")), ("does", s("tarmac"))])),
                ("carpet", obj(vec![("albedo", nums(&[0.13, 0.14, 0.17])), ("rough", n(0.95)), ("stuff", s("cloth")), ("does", s("carpet"))])),
                ("wall", obj(vec![("albedo", nums(&[0.66, 0.64, 0.6])), ("rough", n(0.85)), ("stuff", s("plaster")), ("does", s("plasterboard"))])),
                ("ceiling", obj(vec![("albedo", nums(&[0.72, 0.72, 0.7])), ("rough", n(0.9)), ("stuff", s("plaster")), ("does", s("ceiling"))])),
                ("black", obj(vec![("albedo", nums(&[0.04, 0.04, 0.045])), ("rough", n(0.6))])),
                ("white", obj(vec![("albedo", nums(&[0.8, 0.8, 0.78])), ("rough", n(0.5)), ("does", s("laminate"))])),
            ])),
            ("boxes", arr(boxes)), ("props", arr(vec![])), ("lights", arr(lights)), ("switchables", arr(vec![])), ("doors", arr(doors)), ("routes", arr(routes)),
            ("chokepoints", arr(vec![])), ("volumes", arr(vec![])), ("devices", arr(vec![])), ("connectors", arr(vec![])),
            ("mission", obj(vec![("playerSpawn", nums(&[self.spawn[0], 0.0, self.spawn[1]])), ("objectives", arr(vec![
                obj(vec![("id", s("reach_room")), ("kind", s("reach")), ("area", rect(area)), ("door", s(&first.name))]),
                obj(vec![("id", s("pull_drive")), ("kind", s("drive")), ("rackIndex", n(0.0)), ("front", nums(&[area[0] + 0.5, 1.0, area[2] + 0.5])), ("halfW", n(0.3))]),
                obj(vec![("id", s("exfil")), ("kind", s("exfil")), ("door", s(&last.name)), ("x", n(self.spawn[0])), ("z", n(self.spawn[1]))]),
            ]))])),
            ("breaker", obj(vec![("pos", nums(&[WORLD[0] - 1.0, 1.35, WORLD[1] - 1.4])), ("panel", s(&panel)), ("led", s(&led))])),
            ("beacons", arr(vec![])),
            ("sky", obj(vec![("zenith", nums(&[0.015, 0.025, 0.05])), ("horizon", nums(&[0.04, 0.05, 0.08]))])),
            ("plans", obj(vec![("roomClear", arr(vec![]))])),
        ];
        if let Some(tm) = self.team.as_ref() {
            let v = |p: [f64; 2]| nums(&[p[0], 0.0, p[1]]);
            let rooms: Vec<Json> = self.rooms.iter().enumerate().map(|(i, r)| {
                let my: Vec<Json> = self.doors.iter().flat_map(|d| self.rooms_of_door(d).into_iter().filter(|(ri, _)| *ri == i).map(|(_, from)| obj(vec![("id", s(&d.name)), ("from", s(&from))])).collect::<Vec<_>>()).collect();
                let marks: Vec<Json> = r.marks.iter().map(|mk| { let c = self.door_centre(&mk.watch); obj(vec![("pos", v(mk.pos)), ("watch", s(&mk.watch)), ("aimAt", nums(&[c[0], 1.1, c[1]]))]) }).collect();
                // (the floor inside its walls)
                obj(vec![("id", s(&r.id)), ("name", s(&r.id)), ("bounds", rect([r.x0 + t / 2.0, r.x1 - t / 2.0, r.z0 + t / 2.0, r.z1 - t / 2.0])), ("garrison", n(r.marks.len() as f64)), ("doors", arr(my)), ("marks", arr(marks))])
            }).collect();
            top.push(("scenarios", obj(vec![("roomClear", obj(vec![("name", s(&self.name)), ("team", obj(vec![("sam", v(tm.sam)), ("samYaw", n(tm.yaw)), ("ops", arr(tm.ops.iter().map(|p| v(*p)).collect())), ("opsYaw", n(tm.yaw))])), ("rooms", arr(rooms))]))])));
        }
        obj(top)
    }

    /// THE SCENE AS A LEVEL FILE: text that can be saved under `levels/`.
    pub fn level_text(&self) -> String { crate::json::to_text(&self.level()) }
}
