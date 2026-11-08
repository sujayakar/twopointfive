//! WHAT THERE IS TO FRAME, FOR THE CAMERA THAT RUNS ITSELF (src/host/autoframe.ts). One row a tick: the men the shown player could shoot, the squad's beat, the door under
//! his hand or the way out he walks at, the room he stands in, and what his hands are at. It knows what the picture shows: a man the fog hides is never asked about, and no
//! sound is. What it remembers from tick to tick (when a man was last in his line, when a door order changed its beat) is the host's, as `Drawn` is, and a cut clears it.
//!
//! Asked for only while the mode is on. The numbers it is made by are the page's own table (`Numbers`): the player may tune them.
use alloc::vec::Vec;
use game::ask::{Line, WorldAsk};
use game::cover::Opening;
use game::doortools::Phase;
use game::game::{Game, GuardState};
use game::ids::*;
use game::js::JsMap;
use game::m;
use game::roomread::RoomSpace;
use game::seats::SeatRef;
use game::teams::Team;
use game::V3;

section! {
    /// the shown seat's: no row while the view is not his own to frame
    pub struct Shot {
        /// where he stands
        pub at: [f32; 3],
        /// the squad's newest beat, the door's floor, the room's middle, and how much of the frame each has (0: none)
        pub beat: [f32; 3],
        pub beat_w: f32,
        pub door: [f32; 3],
        pub door_w: f32,
        pub room: [f32; 3],
        pub room_w: f32,
        /// the hand's zoom is scaled by this where he stands
        pub zoom_k: f32,
        /// a button is down
        pub hold: u32,
        /// his feet are still this tick
        pub still: u32,
        /// nothing stands in a turn's way but the clocks: no man held, no arc up, no wall taken, the camera not locked
        pub free: u32,
        /// how fast he moves over the floor (X, Z)
        pub vel: [f32; 2],
        /// hostile men the picture shows in his line, or this near in his line or not
        pub shown: u32,
        /// the zoom is the mode's to move (no lesson holds it)
        pub zoom_free: u32,
        /// seconds since he fired
        pub fired_ago: f32,
        /// the soft lock's man: 0 nobody · 1 a man, standing here · 2 a man Sam holds (no place: the frame as it is)
        pub lock: u32,
        pub lock_at: [f32; 3],
        /// how strongly he holds the frame, and whether the button down is the stalk's own held click (it stills nothing)
        pub lock_s: f32,
        pub lock_stalk: u32,
        /// somebody the picture shows is onto him: a fight, and the zoom does not come in
        pub onto: u32,
    }
}

section! {
    /// a man framed, on the floor: `threats` are those he could shoot, `fight` those of the sway's fight (shown, and in his line a moment ago)
    pub struct ShotMan {
        pub x: f32,
        pub z: f32,
    }
}

/// THE NUMBERS A READ IS MADE BY, in the order the page says them (src/host/autoframe.ts SHOT_NUMBERS)
macro_rules! numbers {
    ($($name:ident)*) => {
        #[derive(Clone, Copy, Debug, Default)]
        pub struct Numbers { $(pub $name: f64,)* }
        impl Numbers {
            pub const COUNT: usize = [$(stringify!($name)),*].len();
            pub fn read(v: &[f64]) -> Numbers { let mut i = v.iter().copied(); Numbers { $($name: i.next().unwrap_or(0.0),)* } }
        }
    };
}
numbers! { fight_m near_m sway_linger lock_m lock_near_m lock_stick lock_linger lock_calm beat_secs beat_fade beat_m beyond_m door_m door_aim door_mps door_keep_m room_max_m room_k sky_k close_k fire_k fire_at lock_in zoom_amt }

const WALK: [&str; 4] = ["KeyW", "KeyS", "KeyD", "KeyA"];
/// how far a walk from him is followed to read the space he stands in (m), and how often it is read: at the most · at the least
const SPACE_M: f64 = 7.0;
const SPACE_MOVED: f64 = 0.2;
const SPACE_STILL: f64 = 1.0;
/// how often the men are asked about (s)
const MEN_EVERY: f64 = 0.1;

/// the level's floor as a read takes it, worked out once of the game shared: a rule that has not needed it has not built it, and a read may not write it
struct Floor { doors: JsMap<game::Num, DoorId>, opens: Vec<Opening>, rooms: Vec<u16> }
/// a way out of the space he stands in
struct Way { at: V3, beyond: V3, dist: f64 }
/// the space round him as last read
struct Space { ways: Vec<Way>, mid: Option<V3> }
/// the soft lock's man as a tick asks him
struct Lock { at: Option<V3>, s: f64, stalk: bool }
/// a door order's beat: the phase it was last seen in and when that began
struct Beat { op: DoorOpId, phase: Phase, t: f64 }

/// THE HOST'S MEMORY OF WHAT WAS FRAMED, beside `Drawn`
pub struct AutoRead {
    pub rows: Vec<Shot>,
    pub threats: Vec<ShotMan>,
    pub fight: Vec<ShotMan>,
    pub numbers: Numbers,
    /// the last tick it was asked for at
    asked: Option<u32>,
    seen_t: f64,
    shown: u32,
    /// when each hostile man was last in his line
    line_t: JsMap<GuardId, f64>,
    lock_gd: Option<GuardId>,
    lock_fight: bool,
    lock_fight_t: f64,
    beats: Vec<Beat>,
    floor: Option<Floor>,
    space: Option<Space>,
    cell: f64,
    space_t: f64,
    /// the way out that had the frame as his feet stopped: the opening's place and the floor past it
    way: Option<(V3, V3)>,
}
impl Default for AutoRead {
    fn default() -> Self {
        AutoRead { rows: Vec::new(), threats: Vec::new(), fight: Vec::new(), numbers: Numbers::default(), asked: None, seen_t: -1e9, shown: 0, line_t: JsMap::new(), lock_gd: None, lock_fight: false, lock_fight_t: -1e9, beats: Vec::new(), floor: None, space: None, cell: -1.0, space_t: -1e9, way: None }
    }
}

/// the view is the shown man's own to frame: no show's man in his place, nobody else's hands on him, him on his feet, nobody spectated, nothing holding the view
pub fn view_own(g: &Game) -> bool {
    let down = g.pl[game::seats::host_man(g)].down;
    let pinned = g.host_cam.is_some_and(|c| g.tab.camera[c].pin.is_some());
    !g.touring && g.puppet.is_none() && !down && !game::rails::rails_sky_on(g) && g.spectate < 0.0 && !game::director::director_showing(g) && !pinned
}

fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }
fn f3(v: V3) -> [f32; 3] { [v[0] as f32, v[1] as f32, v[2] as f32] }
fn on_floor(v: V3) -> V3 { [v[0], 0.0, v[2]] }
/// the floor a stride past a door's frame, on the face away from `from`
fn past(g: &Game, d: DoorId, from: V3, by: f64) -> V3 {
    let (def, c) = (&g.dr[d].def, g.dr[d].frame_centre);
    let (nx, nz) = (-m::sin(def.closed_dir), m::cos(def.closed_dir));
    let s = if nx * (from[0] - def.hinge[0]) + nz * (from[2] - def.hinge[1]) >= 0.0 { 1.0 } else { -1.0 };
    [c[0] - s * nx * by, 0.0, c[2] - s * nz * by]
}

impl AutoRead {
    /// the mode came on: nothing of an earlier run of it is kept
    fn begin(&mut self) {
        self.way = None; self.beats.clear(); self.space = None; self.cell = -1.0; self.line_t.clear(); self.fight.clear(); self.lock_gd = None; self.lock_fight = false; self.lock_fight_t = -1e9;
    }
    /// not asked for: no row
    pub fn unasked(&mut self) { self.rows.clear(); }
    /// the game is another than the one last read
    pub fn cut(&mut self) { *self = AutoRead { numbers: self.numbers, ..AutoRead::default() }; }

    /// TICK `now` HAS RUN (`again`: the same tick asked again: it stands as it is). Not asked for at the tick before, the mode has just come on
    pub fn take(&mut self, g: &Game, now: u32, again: bool) {
        if !self.asked.is_some_and(|t| now.wrapping_sub(t) <= 1) { self.begin(); }
        self.asked = Some(now);
        if again { return; }
        self.rows.clear();
        // (the view is somebody else's for now: nobody is the lock's, and no way out keeps the frame for when it is his again)
        if !view_own(g) { self.way = None; self.lock_gd = None; return; }
        let Ok(ask) = WorldAsk::try_of(g) else { return };
        let seat_id = g.local_seat.unwrap_or(g.seats[0]);
        let seat = game::seats::seat(g, seat_id);
        let row = self.read(g, &seat, seat_id, &ask);
        self.rows.push(row);
    }

    fn read(&mut self, g: &Game, seat: &SeatRef, seat_id: SeatId, ask: &WorldAsk) -> Shot {
        let k = self.numbers;
        let (me, now) = (seat.player, g.time);
        let pl = &g.pl[me];
        let (p, vel) = (g.ch[pl.ch].pos, g.ch[pl.ch].vel);
        if now - self.seen_t >= MEN_EVERY || now < self.seen_t { self.ask_men(g, ask, p); }
        let hand = g.tab.cmd_seat[seat_id].cmd.as_ref();
        let buttons = hand.map_or(0, |h| h.buttons as u32);
        let walking = hand.is_some_and(|h| h.keys.iter().any(|k| WALK.contains(&k.as_str())));
        // (a stalk, and the wait at a corner, live only under the held click)
        let lmb = buttons & 1 != 0;
        let lock = self.lock(g, seat, p, if lmb { game::stalk::stalk_on(g, seat) } else { None }, lmb && game::stalk::stalk_waiting(g, me));
        let beat = self.beat(g, seat, p);
        let cell = m::floor(p[0] / g.col.nav.cell) + m::floor(p[2] / g.col.nav.cell) * g.col.nav.w;
        let age = now - self.space_t;
        if self.space.is_none() || age < 0.0 || age >= SPACE_STILL || (cell != self.cell && age >= SPACE_MOVED) {
            self.space = Some(self.read_space(g, p));
            self.cell = cell;
            self.space_t = now;
        }
        let speed = m::hyp2(vel[0], vel[2]);
        let door = self.door(g, seat, p, vel, speed);
        let mid = self.space.as_ref().and_then(|s| s.mid);
        let held = pl.holding.is_some();
        let close = g.m.close.state.get(me).is_some_and(|s| s.mode.is_some());
        let place = if close || (k.lock_in > 0.0 && k.zoom_amt > 0.0 && held && !self.lock_fight) { k.close_k } else if mid.is_some() { k.room_k } else if !game::weather::weather_under_roof_of(g, p[0], p[2]) { k.sky_k } else { 1.0 };
        let lesson_on_view = game::lesson::lesson_step(g).is_some_and(|s| matches!(s.done, game::scene_scenarios::LessonTestKind::View { .. }));
        let free = !held && pl.dragging.is_none() && pl.throw_preview.is_none() && pl.launch_held.is_none() && pl.wall.is_none() && !game::rails::rails_locks(g, "camera") && !lesson_on_view;
        let (beat_at, beat_w) = beat.unwrap_or(([0.0; 3], 0.0));
        let (door_at, door_w) = door.unwrap_or(([0.0; 3], 0.0));
        Shot {
            at: f3(p),
            beat: f3(beat_at), beat_w: beat_w as f32,
            door: f3(door_at), door_w: door_w as f32,
            room: f3(mid.unwrap_or([0.0; 3])), room_w: mid.is_some() as u32 as f32,
            zoom_k: (place * if pl.sup.v > k.fire_at { k.fire_k } else { 1.0 }) as f32,
            hold: (buttons != 0) as u32,
            still: (!walking && speed < 0.3) as u32,
            free: free as u32,
            vel: [vel[0] as f32, vel[2] as f32],
            shown: self.shown,
            zoom_free: !game::rails::rails_locks(g, "zoom") as u32,
            fired_ago: (now - pl.last_fire_t) as f32,
            lock: lock.as_ref().map_or(0, |l| if l.at.is_some() { 1 } else { 2 }),
            lock_at: f3(lock.as_ref().and_then(|l| l.at).unwrap_or([0.0; 3])),
            lock_s: lock.as_ref().map_or(0.0, |l| l.s) as f32,
            lock_stalk: lock.as_ref().is_some_and(|l| l.stalk) as u32,
            onto: self.lock_fight as u32,
        }
    }

    /// THE MEN HE COULD SHOOT: hostile, on their feet, in the picture (the fog's word) and in his line
    fn ask_men(&mut self, g: &Game, ask: &WorldAsk, p: V3) {
        let k = self.numbers;
        let now = g.time;
        self.seen_t = now; self.threats.clear(); self.fight.clear(); self.shown = 0;
        if self.line_t.len() > g.guards.len() { self.line_t.clear(); }
        let eye: V3 = [p[0], 1.5, p[2]];
        let (mut lock_d, mut onto, had) = (k.lock_m, false, self.lock_gd.take());
        for &gd in &g.guards {
            let man = &g.gd[gd];
            if man.state == GuardState::Dead || man.held.is_some() || !game::teams::is_hostile(g, Team::Player, man.team) { continue; }
            let alert = man.state == GuardState::Alert;
            onto |= alert;   // (the soft lock's fight: a man onto him anywhere, in the picture or not: the one shooting from the far end)
            let q = g.ch[man.ch].pos;
            let d = flat(p, q);
            if d > k.fight_m || game::fogofwar::fog_of_war_hides(g, gd) { continue; }
            let in_line = !ask.line(eye, [q[0], 1.3, q[2]], Line::Sight);
            let at = ShotMan { x: q[0] as f32, z: q[2] as f32 };
            if in_line { self.threats.push(at); self.line_t.set(gd, now); }
            let within = |by: f64, line_t: &JsMap<GuardId, f64>| line_t.get(gd).is_some_and(|&l| now >= l && now - l <= by);
            if within(k.sway_linger, &self.line_t) { self.fight.push(at); }   // (a man of the fight: the sway's)
            // the soft lock's man: the nearest in his line who is not onto him: the one it has until another is clearly nearer, and through a moment out of the line
            if !alert && (in_line || (had == Some(gd) && within(k.lock_linger, &self.line_t))) {
                let nearer = d - if had == Some(gd) { k.lock_stick } else { 0.0 };
                if nearer < lock_d { lock_d = nearer; self.lock_gd = Some(gd); }
            }
            if in_line || d <= k.near_m { self.shown += 1; }   // (behind cover but this near: a step puts him in the line)
        }
        // (somebody is onto him, or was a moment ago: that is a fight, and the fight's rules frame it)
        if onto || now < self.lock_fight_t { self.lock_fight_t = if onto { now } else { -1e9 }; }
        self.lock_fight = now >= self.lock_fight_t && now - self.lock_fight_t < if onto { f64::INFINITY } else { k.lock_calm };
        if self.lock_fight { self.lock_gd = None; }
    }

    /// THE SOFT LOCK'S MAN, where he stands now: the man being stalked (`stalked`), or else the one the last asking chose, a man he could take who is not onto him; with a man
    /// in his arm, no place. Nobody in a fight, and but for a stalk nobody while he is not sneaking. `wait`: the held click is the corner wait's
    fn lock(&self, g: &Game, seat: &SeatRef, p: V3, stalked: Option<GuardId>, wait: bool) -> Option<Lock> {
        let pl = &g.pl[seat.player];
        if self.lock_fight || pl.down { return None; }
        if pl.holding.is_some() { return Some(Lock { at: None, s: 1.0, stalk: false }); }
        let busy = g.touring || game::game::player_rifle_up(g, seat.player) || pl.sprinting || pl.dragging.is_some() || pl.kick.is_some() || pl.picking.is_some() || pl.wanding.is_some() || pl.coding.is_some()
            || game::handwork::sam_work_of(g, seat).is_some() || pl.throw_preview.is_some() || pl.launch_held.is_some();
        // (still a man of this floor: a load swaps them between askings)
        let ok = |m: Option<GuardId>| m.filter(|&m| g.guards.contains(&m) && game::close::close_subject_of(g, m) && g.gd[m].state != GuardState::Alert);
        let man = ok(stalked).or(if busy { None } else { ok(self.lock_gd) })?;
        let q = g.ch[g.gd[man].ch].pos;
        Some(Lock { at: Some(on_floor(q)), s: game::v3::smoothstep(self.numbers.lock_m, self.numbers.lock_near_m, flat(p, q)), stalk: stalked == Some(man) || wait })
    }

    /// THE SQUAD: the newest beat, a door order's phase as it changes (the order given, its men set, the word, the entry), or men just sent somewhere
    fn beat(&mut self, g: &Game, seat: &SeatRef, p: V3) -> Option<(V3, f64)> {
        if !game::orders::squad_orders_active(g, seat) { self.beats.clear(); return None; }
        let (k, now) = (self.numbers, g.time);
        let fade = |t: f64| { let age = now - t; if age < 0.0 { 0.0 } else if age <= k.beat_secs { 1.0 } else { m::max(0.0, 1.0 - (age - k.beat_secs) / k.beat_fade) } };
        let ops = &seat.door_tools.ops;
        let (mut beat, mut best_t) = (None, -1e9);
        for &op in ops {
            let o = &g.tab.doortools_door_op[op];
            // (an order's own `t0` moves under one phase, a shot at the door or a swing: a beat is its phase changing)
            let i = match self.beats.iter().position(|b| b.op == op) {
                Some(i) => i,
                None => { self.beats.push(Beat { op, phase: o.phase, t: m::max(o.t0, now - k.beat_secs) }); self.beats.len() - 1 }
            };
            let b = &mut self.beats[i];
            if b.phase != o.phase { b.phase = o.phase; b.t = now; }
            let (t, w) = (b.t, fade(b.t));
            if w <= 0.0 || t <= best_t { continue; }
            let at = match o.phase {
                Phase::Stack | Phase::Ready | Phase::Place | Phase::Standoff | Phase::Armed => on_floor(g.dr[o.door].frame_centre),
                _ => past(g, o.door, g.ch[g.gd[o.lead].ch].pos, k.beyond_m),
            };
            if flat(p, at) > k.beat_m { continue; }
            beat = Some((at, w));
            best_t = t;
        }
        if self.beats.len() > ops.len() { self.beats.retain(|b| ops.contains(&b.op)); }
        // (men sent to a mark with no door order of their own: the middle of their marks)
        let (mut n, mut sx, mut sz, mut t0) = (0.0, 0.0, 0.0, -1e9);
        for gd in game::orders::squad_orders_men(g, seat, None, None) {
            let Some(s) = game::orders::squad_orders_of(g, seat, gd).map(|a| &g.tab.orders_assignment[a]) else { continue };
            let Some(post) = s.post else { continue };
            if fade(s.t0) <= 0.0 || ops.iter().any(|&o| g.tab.doortools_door_op[o].men.contains(&gd)) { continue; }
            if s.t0 > t0 + 1e-6 { t0 = s.t0; n = 0.0; sx = 0.0; sz = 0.0; }
            if s.t0 >= t0 - 1e-6 { n += 1.0; sx += post[0]; sz += post[2]; }
        }
        if n > 0.0 && t0 > best_t {
            let at: V3 = [sx / n, 0.0, sz / n];
            if flat(p, at) <= k.beat_m { beat = Some((at, fade(t0))); }
        }
        beat
    }

    /// THE SPACE ROUND HIM, and its middle where it is a room and not a hall
    fn read_space(&mut self, g: &Game, p: V3) -> Space {
        let room_max = self.numbers.room_max_m;
        let floor = self.floor.get_or_insert_with(|| {
            let doors = game::guards::door_cells_of(g);
            let opens = if g.cover.nav { g.cover.openings.iter().map(|&o| g.tab.cover_opening[o].clone()).collect() } else { game::cover::cover_openings_of(g).unwrap_or_default() };
            let rooms = game::cleared::cleared_rooms_grid_of(g, &doors);
            Floor { doors, opens, rooms }
        });
        let rows: Vec<&Opening> = floor.opens.iter().collect();
        let Some(RoomSpace { cells, edge, .. }) = game::roomread::room_space(g, &rows, &floor.doors, p, SPACE_M, p) else { return Space { ways: Vec::new(), mid: None } };
        let mut ways: Vec<(Way, &str)> = edge.iter().map(|&(i, c)| {
            let o = rows[i];
            let (_, beyond, _, _) = game::roomread::way_beyond(g, o, c);
            (Way { at: on_floor(o.at), beyond, dist: m::hyp2(o.at[0] - p[0], o.at[2] - p[2]) }, o.id.as_str())
        }).collect();
        ways.sort_by(|a, b| a.0.dist.partial_cmp(&b.0.dist).unwrap_or(core::cmp::Ordering::Equal).then_with(|| a.1.cmp(b.1)));
        let room = game::cleared::cleared_room_in(g, &floor.rooms, p);
        let mut mid = None;
        if room > 0.0 && !cells.is_empty() {
            let (w, cs) = (g.col.nav.w, g.col.nav.cell);
            let (mut x0, mut x1, mut z0, mut z1) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
            for &i in &cells {
                let (x, z) = ((game::js::rem(i, w) + 0.5) * cs, (m::floor(i / w) + 0.5) * cs);
                x0 = m::min(x0, x); x1 = m::max(x1, x); z0 = m::min(z0, z); z1 = m::max(z1, z);
            }
            if m::max(x1 - x0, z1 - z0) <= room_max { mid = Some([(x0 + x1) / 2.0, 0.0, (z0 + z1) / 2.0]); }
        }
        Space { ways: ways.into_iter().map(|w| w.0).collect(), mid }
    }

    /// THE DOOR: the one under his hand, or the way out he is walking at (kept while he stands by it)
    fn door(&mut self, g: &Game, seat: &SeatRef, p: V3, vel: V3, speed: f64) -> Option<(V3, f64)> {
        let k = self.numbers;
        if let Some(d) = seat.use_door { self.way = None; return Some((past(g, d, p, k.beyond_m), 1.0)); }
        let mut best = 0.0;
        if speed > k.door_mps {
            for w in self.space.as_ref().map_or(&[][..], |s| &s.ways[..]) {
                if w.dist > k.door_m { continue; }
                let (dx, dz) = (w.at[0] - p[0], w.at[2] - p[2]);
                let d = m::hyp2(dx, dz);
                if d < 1e-3 { continue; }
                let toward = (vel[0] * dx + vel[2] * dz) / (d * speed);
                if toward < k.door_aim { continue; }
                let s = m::sqrt(m::max(0.0, 1.0 - d / k.door_m)) * toward;
                if s > best { best = s; self.way = Some((w.at, w.beyond)); }
            }
        }
        let (at, beyond) = self.way?;
        if best > 0.0 { return Some((beyond, m::min(1.0, best * 1.5))); }
        if flat(p, at) <= k.door_keep_m { return Some((beyond, 0.8)); }
        self.way = None;
        None
    }
}
