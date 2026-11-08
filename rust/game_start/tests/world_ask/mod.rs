//! Questions the world's tests ask of a stood game that `play.rs` has no reader for, and the few stagings that have no key. Each answers one thing, by
//! its own name, so that when a field moves one line here follows it.
#![allow(dead_code)]
use game::ids::*;
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};
use game_start::scene::Scene;

/// a man's held attention, as far as a test asks it
#[derive(Clone, Copy, Debug)]
pub struct Held { pub at: [f64; 2], pub kind: game::attention::DrawKind, pub mode: game::attention::HoldMode, pub at_it: bool, pub nth: i32 }

/// A box of a level's file, for what a scene has no word for: a counter, a pane.
pub fn a_box(id: &str, c: [f64; 3], h: [f64; 3], material: &str, flags: &[&str]) -> Json {
    let mut m = vec![("id", json::s(id)), ("group", json::s("added")), ("c", json::nums(&c)), ("h", json::nums(&h)), ("material", json::s(material))];
    if !flags.is_empty() { m.push(("flags", json::arr(flags.iter().map(|f| json::s(f)).collect()))); }
    json::obj(m)
}

/// A SCENE AND A FEW BOXES OF ITS LEVEL'S FILE MORE: the file is written, stood by the one start, and taken away.
pub fn scene_with(sc: &Scene, more: Vec<Json>, o: &Opts) -> Play { scene_plus(sc, more, Vec::new(), o) }
/// …and a few devices.
pub fn scene_plus(sc: &Scene, more: Vec<Json>, devices: Vec<Json>, o: &Opts) -> Play {
    let mut level = sc.level();
    let Json::Obj(members, _) = &mut level else { panic!("a level is an object") };
    for (key, rows) in [("boxes", more), ("devices", devices)] {
        let (_, list) = members.iter_mut().find(|(k, _)| k == key).expect("a level has it");
        let Json::Arr(list) = list else { panic!("{key} is a list") };
        list.extend(rows);
    }
    let path = std::env::temp_dir().join(format!("world-scene-{}-{:?}.json", std::process::id(), std::thread::current().id()));
    std::fs::write(&path, json::to_text(&level)).unwrap();
    let p = Play::level(path.to_str().unwrap(), sc.is_room_clear(), o);
    std::fs::remove_file(&path).ok();
    p
}

pub trait WorldAsk {
    /// the place `m` metres in front of a man, on the floor
    fn ahead(&self, gd: GuardId, m: f64, side: f64) -> [f64; 2];
    /// a ring's clack at a place: the launcher's own call
    fn clack_at(&mut self, at: [f64; 2]);
    /// a knock as faint as a clack that nobody made to be looked at (a chair a door swept): an event, the game's own call for it being private to the sweep
    fn knock_at(&mut self, at: [f64; 2]);
    /// a footfall of Sam's at a place, at a level: walking 0.3, running 0.6 (the walk itself is a test of the keys)
    fn footfall_at(&mut self, at: [f64; 2], level: f64);
    /// a door slammed at a place by Sam: a loud door
    fn slam_at(&mut self, at: [f64; 2]);
    /// what holds his attention now
    fn held(&self, gd: GuardId) -> Option<Held>;
    /// how many small noises he has heard
    fn small_heard(&self, gd: GuardId) -> f64;
    /// the thread the floor is shown for his held attention: is it his way, and his own path from where he is on it
    fn thread(&self, gd: GuardId) -> Option<(bool, Vec<[f64; 2]>)>;
    /// is a walk between two places through something solid at knee height
    fn wall_between(&mut self, a: [f64; 2], b: [f64; 2]) -> bool;
    /// the door of this name stands wide open, on the side it swings to
    fn door_stands_open(&mut self, name: &str);
}

impl WorldAsk for Play {
    fn ahead(&self, gd: GuardId, m: f64, side: f64) -> [f64; 2] {
        let (q, y) = (self.at(gd), self.facing(gd));
        let (fx, fz) = (y.sin(), y.cos());
        [q[0] + fx * m + fz * side, q[1] + fz * m - fx * side]
    }
    fn clack_at(&mut self, at: [f64; 2]) { game::launcher::clack(&mut self.g, [at[0], 0.0, at[1]], None); }
    fn knock_at(&mut self, at: [f64; 2]) {
        let (time, level, serial) = (self.g.time, self.g.k.ring.clack_noise, self.g.serial());
        self.g.events.push(game::game::GameEvent { kind: game::director::EventKind::Prop, pos: [at[0], 0.0, at[1]], time, loud: false, level: Some(level), who: Some(0.0), lure: None, serial, ..game::game::GameEvent::zeroed() });
    }
    fn footfall_at(&mut self, at: [f64; 2], level: f64) {
        let (time, serial) = (self.g.time, self.g.serial());
        self.g.events.push(game::game::GameEvent { kind: game::director::EventKind::Step, pos: [at[0], 0.0, at[1]], time, loud: false, level: Some(level), serial, ..game::game::GameEvent::zeroed() });
    }
    fn slam_at(&mut self, at: [f64; 2]) {
        let (time, serial) = (self.g.time, self.g.serial());
        self.g.events.push(game::game::GameEvent { kind: game::director::EventKind::Door, pos: [at[0], 0.0, at[1]], time, loud: true, level: Some(1.0), who: Some(1.0), serial, ..game::game::GameEvent::zeroed() });
    }
    fn held(&self, gd: GuardId) -> Option<Held> {
        game::attention::attention_of(&self.g, gd).map(|h| Held { at: [h.at[0], h.at[2]], kind: h.kind, mode: h.mode, at_it: h.until >= 0.0, nth: h.nth })
    }
    fn small_heard(&self, gd: GuardId) -> f64 { self.g.gd[gd].witness.heard.small }
    fn thread(&self, gd: GuardId) -> Option<(bool, Vec<[f64; 2]>)> {
        let id = self.id_of(gd);
        let l = self.view().attention.into_iter().find(|l| l.man == id)?;
        Some((l.walks, l.own.unwrap_or_default().iter().map(|q| [q[0], q[2]]).collect()))
    }
    fn wall_between(&mut self, a: [f64; 2], b: [f64; 2]) -> bool { game::scene_collision::segment_blocked(&mut self.g, [a[0], 0.5, a[1]], [b[0], 0.5, b[1]], None, None, None) }
    fn door_stands_open(&mut self, name: &str) {
        let d = game::doors::doors_by_name(&self.g, name).unwrap_or_else(|| panic!("no door '{name}'"));
        let turn = game::doors::door_swing(&mut self.g, d) * 1.4;
        game::doors::door_set_latched(&mut self.g, d, false);
        game::doors::door_set_angle(&mut self.g, d, turn);
    }
}

/// The office's panes and blasts
pub trait GlassAsk {
    /// the index of the pane whose middle stands at (x, z), by where it stands: never a number that a level's export moves
    fn pane_at(&mut self, x: f64, z: f64) -> f64;
    /// this pane is of this kind of glass
    fn pane_is(&mut self, bi: f64, kind: game::glasspen::PaneType);
    /// how much of it is open, 0 to 1: 1 once it has gone
    fn pane_open(&mut self, bi: f64) -> f64;
    /// a man out of everybody's way, facing a place, held by nothing
    fn man_put(&mut self, gd: GuardId, at: [f64; 2], face: [f64; 2]);
    /// the blast a 40 mm hands over, on the floor at `at`: the report of whom it reached
    fn burst(&mut self, at: [f64; 2]) -> game::frag::BlastReport;
    /// a man's line in a blast's report
    fn hit_of(&self, r: &game::frag::BlastReport, gd: GuardId) -> Option<game::frag::BlastHit>;
    /// the moment he last went reeling
    fn reeled_at(&self, gd: GuardId) -> f64;
    /// the share of a body standing at a place that a burst from `from` has a line to, as the glass stands now
    fn share_open(&mut self, from: [f64; 2], feet: [f64; 2]) -> f64;
    /// is the line from a burst on the floor at `from` to a point open, as the glass stands now
    fn burst_line_open(&mut self, from: [f64; 2], to: [f64; 3], view: game::frag::GlassView) -> bool;
}

impl GlassAsk for Play {
    fn pane_at(&mut self, x: f64, z: f64) -> f64 {
        let boxes = self.g.col.boxes.clone();
        for (i, &b) in boxes.iter().enumerate() {
            if game::glass::is_pane(&mut self.g, Some(b)) { let c = self.g.tab.boxes[b].c; if (c[0] - x).abs() < 0.01 && (c[2] - z).abs() < 0.01 { return i as f64; } }
        }
        panic!("no pane stands at ({x}, {z})");
    }
    fn pane_is(&mut self, bi: f64, kind: game::glasspen::PaneType) { game::glassbreak::glass_break_set_type(&mut self.g, bi, Some(kind)); }
    fn pane_open(&mut self, bi: f64) -> f64 {
        match game::glassbreak::glass_break_pane_info(&mut self.g, bi) { Some(i) if i.gone => 1.0, Some(i) => i.open_fraction, None => 0.0 }
    }
    fn man_put(&mut self, gd: GuardId, at: [f64; 2], face: [f64; 2]) {
        let y = (face[0] - at[0]).atan2(face[1] - at[1]);
        let c = self.g.gd[gd].ch;
        self.g.ch[c].pos = [at[0], 0.0, at[1]];
        self.g.ch[c].vel = [0.0; 3];
        self.g.ch[c].body_yaw = y;
        self.g.ch[c].aim_yaw = y;
        let h = &mut self.g.gd[gd];
        h.path.clear();
        h.path_goal = None;
        h.speed = 0.0;
    }
    fn burst(&mut self, at: [f64; 2]) -> game::frag::BlastReport {
        let who = self.g.ch[self.g.pl[self.g.player].ch].id;
        game::frag::blast(&mut self.g, [at[0], 0.05, at[1]], &game::frag::BlastSpec { who, name: Some(String::from("40 mm grenade")), ..game::frag::BlastSpec::zeroed() })
    }
    fn hit_of(&self, r: &game::frag::BlastReport, gd: GuardId) -> Option<game::frag::BlastHit> { let id = self.id_of(gd); r.hits.iter().find(|h| h.id == id).cloned() }
    fn reeled_at(&self, gd: GuardId) -> f64 { self.g.gd[gd].stagger_until }
    fn share_open(&mut self, from: [f64; 2], feet: [f64; 2]) -> f64 {
        let lift = self.g.k.frag.lift;
        game::frag::exposure(&mut self.g, [from[0], 0.05 + lift, from[1]], [feet[0], 0.0, feet[1]], [feet[0], 1.6, feet[1]], Some(game::frag::GlassView::Whole))
    }
    fn burst_line_open(&mut self, from: [f64; 2], to: [f64; 3], view: game::frag::GlassView) -> bool {
        let lift = self.g.k.frag.lift;
        game::frag::burst_line(&mut self.g, [from[0], 0.05 + lift, from[1]], to, Some(view))
    }
}

/// Walls that come apart
pub trait WallAsk {
    /// the blast a 40 mm hands over, at a point of the world
    fn burst_at(&mut self, at: [f64; 3]) -> game::frag::BlastReport;
    /// A BODY AS WIDE AS A MAN IS WALKED from a place to a place with the walkers' own collision call: where it ends
    fn body_walks(&mut self, from: [f64; 2], to: [f64; 2]) -> [f64; 2];
    /// the same body along the walk grid's own way: the length of the way, where it ends. None: the grid has no way
    fn body_walks_the_grid(&mut self, from: [f64; 2], to: [f64; 2]) -> Option<(f64, [f64; 2])>;
    /// does the grid call this place floor
    fn grid_floor(&self, at: [f64; 2]) -> bool;
    /// Sam sets a charge on a wall with his own hands: stood at its foot with the cursor on it, the row given on the interaction key and stood on. The charge, or why not.
    fn sam_sets_charge(&mut self, at: [f64; 3], from: [f64; 2]) -> Result<WallChargeId, String>;
    /// does the game find floor behind a place of a wall, as a charge there will leave it
    fn floor_behind(&mut self, at: [f64; 3], from: [f64; 2], furniture: bool) -> bool;
    /// the charge goes, by the game's own call: is there a hole
    fn charge_blows(&mut self, c: WallChargeId) -> bool;
    /// how many holes the walls have told of so far
    fn holes(&self) -> usize;
}

impl WallAsk for Play {
    fn burst_at(&mut self, at: [f64; 3]) -> game::frag::BlastReport {
        let (who, lift) = (self.g.ch[self.g.pl[self.g.player].ch].id, self.g.k.frag.lift);
        game::frag::blast(&mut self.g, [at[0], at[1] - lift, at[2]], &game::frag::BlastSpec { who, name: Some(String::from("40 mm grenade")), ..game::frag::BlastSpec::zeroed() })
    }
    fn body_walks(&mut self, from: [f64; 2], to: [f64; 2]) -> [f64; 2] {
        let mut p = [from[0], 0.0, from[1]];
        for _ in 0..1000 {
            let (dx, dz) = (to[0] - p[0], to[1] - p[2]);
            let d = dx.hypot(dz);
            if d < 0.02 { break; }
            p[0] += dx / d * 0.02;
            p[2] += dz / d * 0.02;
            game::scene_collision::collide_circle(&mut self.g, &mut p, 0.3, 0.2, 1.5, None);
        }
        [p[0], p[2]]
    }
    fn body_walks_the_grid(&mut self, from: [f64; 2], to: [f64; 2]) -> Option<(f64, [f64; 2])> {
        let path = game::scene_collision::nav_grid_find_path(&mut self.g, [from[0], 0.0, from[1]], [to[0], 0.0, to[1]], None, None)?;
        let (mut len, mut at) = (0.0, from);
        for q in &path {
            let q = [q[0], q[2]];
            len += (q[0] - at[0]).hypot(q[1] - at[1]);
            at = self.body_walks(at, q);
        }
        Some((len, at))
    }
    fn grid_floor(&self, at: [f64; 2]) -> bool { !game::scene_collision::nav_grid_is_blocked(&self.g, at[0], at[1]) }
    fn sam_sets_charge(&mut self, at: [f64; 3], from: [f64; 2]) -> Result<WallChargeId, String> {
        game::wallcharge::wall_charge_raise_carried(&mut self.g, 2.0);
        let w = game::wallcharge::wall_at(&mut self.g, at, Some([from[0], 0.0, from[1]])).filter(|w| w.ok).ok_or_else(|| String::from("no place for a charge there"))?;
        let spot = game::wallcharge::wall_charge_set_spot(&mut self.g, &w).unwrap_or_else(|| game::wallcharge::wall_pt(&self.g, &w, 0.0, 0.55, None));
        let n0 = self.g.wall_charge.set.len();
        let mut why = String::new();
        for h in [1.0, 1.7, 0.4] {
            let on = game::wallcharge::wall_pt(&self.g, &w, 0.0, 0.0, Some(h));
            self.sam_put([spot[0], spot[2]]);
            self.cursor_on(on);
            self.run(1.5);
            if !game::game::game_select_verb(&mut self.g, game::game::InteractableKind::Wall, Some(game::game::Verb::Charge)) { why = format!("no row at its foot with the cursor {h} m up"); continue; }
            self.key_down("Space");
            self.frame();
            self.key_up("Space");
            let secs = self.g.k.doortools.set_secs + 1.0;
            if self.until(secs, |p| p.g.wall_charge.set.len() > n0).is_some() { return Ok(*self.g.wall_charge.set.last().unwrap()); }
            why = String::from("the row was given and no charge is on the wall");
        }
        Err(why)
    }
    fn floor_behind(&mut self, at: [f64; 3], from: [f64; 2], furniture: bool) -> bool {
        let w = game::wallcharge::wall_at(&mut self.g, at, Some([from[0], 0.0, from[1]])).filter(|w| w.ok).expect("a wall there that takes a charge");
        game::wallcharge::wall_charge_far_floor(&mut self.g, &w, Some(furniture)).is_some()
    }
    fn charge_blows(&mut self, c: WallChargeId) -> bool {
        let n = self.holes();
        game::wallcharge::wall_charge_blow(&mut self.g, Some(c), Some(true));
        self.run(0.2);
        self.holes() > n
    }
    fn holes(&self) -> usize { self.g.destr.walls.changes.iter().filter(|c| c.kind == game::wallbreak::LevelChangeKind::Hole).count() }
}

/// The dark rules
pub trait DarkAsk {
    /// has the dark room stopped him: he stands short of it
    fn balks(&self, gd: GuardId) -> bool;
    /// is this one of the dark door's lines of a kind
    fn dark_line(&self, text: &str, kind: game::darkdoor::LineKind) -> bool;
    /// is this one of the dark door's lines at all
    fn dark_any(&self, text: &str) -> bool;
    /// is the light on his gun lit
    fn lamp_on(&self, gd: GuardId) -> bool;
    /// how many of the men of the other side have theirs lit
    fn lamps_burning(&self) -> usize;
    /// gunfire has reached him: written on his witness, so that nobody else has it (what a round fired would tell every man)
    fn gunfire_reached(&mut self, gd: GuardId);
    /// nothing has reached him
    fn heard_nothing(&self, gd: GuardId) -> bool;
    /// two men look at each other
    fn face_each_other(&mut self, a: GuardId, b: GuardId);
    /// does he know for good, as the lamps' rule has it
    fn dark_knows(&self, gd: GuardId) -> bool;
    /// the mains go, pulled by hand
    fn mains_pulled(&mut self);
    /// when an event of the lights-out's timeline first came
    fn powerout_at(&self, what: game::powerout::PowerOutEventWhat) -> Option<f64>;
    /// the men the lights-out's timeline has sent
    fn powerout_sent(&self) -> Vec<GuardId>;
    /// how many of them the timeline has home again
    fn powerout_homes(&self) -> usize;
    /// the timeline, in words
    fn powerout_line(&self) -> String;
}

impl DarkAsk for Play {
    fn balks(&self, gd: GuardId) -> bool { self.g.m.darkdoor.state_of.get(gd).map_or(false, |s| s.act == game::darkdoor::DarkDoorAct::Balk) }
    fn dark_line(&self, text: &str, kind: game::darkdoor::LineKind) -> bool { game::darkdoor::lines().get(String::from(kind.as_str())).map_or(false, |r| r.say.iter().chain(r.he.iter().flatten()).chain(r.they.iter().flatten()).any(|l| l == text)) }
    fn dark_any(&self, text: &str) -> bool { game::darkdoor::lines().values().any(|r| r.say.iter().chain(r.he.iter().flatten()).chain(r.they.iter().flatten()).any(|l| l == text)) }
    fn lamp_on(&self, gd: GuardId) -> bool { self.g.tab.light[self.g.gd[gd].flashlight].enabled }
    fn lamps_burning(&self) -> usize { self.g.guards.iter().filter(|&&m| self.g.gd[m].team == game::teams::Team::Hostile && self.lamp_on(m)).count() }
    fn gunfire_reached(&mut self, gd: GuardId) { self.g.gd[gd].witness.heard.shots = self.g.gd[gd].witness.heard.shots.max(1.0); }
    fn heard_nothing(&self, gd: GuardId) -> bool { let h = &self.g.gd[gd].witness.heard; h.shots + h.kick + h.bang == 0.0 && self.g.gd[gd].witness.alerted_by.is_none() && self.state(gd) == game::game::GuardState::Patrol }
    fn face_each_other(&mut self, a: GuardId, b: GuardId) {
        let (pa, pb) = (self.at(a), self.at(b));
        let (ca, cb) = (self.g.gd[a].ch, self.g.gd[b].ch);
        let y = (pb[0] - pa[0]).atan2(pb[1] - pa[1]);
        self.g.ch[ca].aim_yaw = y; self.g.ch[ca].body_yaw = y;
        let y = (pa[0] - pb[0]).atan2(pa[1] - pb[1]);
        self.g.ch[cb].aim_yaw = y; self.g.ch[cb].body_yaw = y;
    }
    fn dark_knows(&self, gd: GuardId) -> bool { self.g.m.darkfoe.state_of.get(gd).map_or(false, |s| s.knows) }
    fn mains_pulled(&mut self) { game::game::game_set_blackout(&mut self.g, 86400.0, game::game::GameSetBlackoutCause::Hand); }
    fn powerout_at(&self, what: game::powerout::PowerOutEventWhat) -> Option<f64> { self.g.m.powerout.states.as_ref()?.events.iter().find(|e| e.what == what).map(|e| e.t) }
    fn powerout_sent(&self) -> Vec<GuardId> {
        let Some(st) = self.g.m.powerout.states.as_ref() else { return Vec::new() };
        let names: Vec<&str> = st.events.iter().filter(|e| e.what == game::powerout::PowerOutEventWhat::Go).map(|e| e.who.as_str()).collect();
        self.g.guards.iter().copied().filter(|&m| names.contains(&self.g.gd[m].callsign.as_str())).collect()
    }
    fn powerout_homes(&self) -> usize { self.g.m.powerout.states.as_ref().map_or(0, |st| st.events.iter().filter(|e| e.what == game::powerout::PowerOutEventWhat::Home).count()) }
    fn powerout_line(&self) -> String { self.g.m.powerout.states.as_ref().map_or(String::new(), |st| st.events.iter().map(|e| format!("{:.1} {} {} {}", e.t, e.what.as_str(), e.who, e.why)).collect::<Vec<_>>().join(" | ")) }
}

/// A search, a yard and what watches it
pub trait YardAsk {
    /// a man is out of it: parked far off, held, blind and deaf
    fn parked_far(&mut self, gd: GuardId, k: usize);
    /// he starts a search round a fix this instant, stood at `at`, deaf: what is asked is where he looks
    fn searches(&mut self, gd: GuardId, at: [f64; 2], fix: [f64; 2]);
    /// the spots of his plan: (where, how dark, how much cover, is it open floor, how plausible)
    fn spots(&self, gd: GuardId) -> Vec<([f64; 2], f64, f64, bool, f64)>;
    /// the spot he is looking into, if he is
    fn looking_at(&self, gd: GuardId) -> Option<([f64; 2], f64, f64, bool)>;
    /// a door is shut and latched, and locked if said
    fn door_shut_locked(&mut self, name: &str, locked: bool);
    /// a place's own light, every carried light left out, by the meter: (bare, with the moon, through a tube)
    fn place_light(&mut self, at: [f64; 2]) -> (f64, f64, f64);
    /// how the first camera stands
    fn camera_state(&self) -> game::sim_lightshome::CameraState;
    /// how much it has seen
    fn camera_seen(&self) -> f64;
    /// how many times the beam has been tripped, by its place in the level's list
    fn beam_trips(&self, k: usize) -> f64;
    /// the man sent on a device's errand
    fn sent_to_look(&self) -> Option<GuardId>;
    /// Sam walks to a place by the keys, as a hand does: whether he got there in time
    fn sam_goes_to(&mut self, at: [f64; 2], secs: f64) -> bool;
    /// Sam gets down, or up, by the key
    fn sam_crouched(&mut self, down: bool);
}

impl YardAsk for Play {
    fn parked_far(&mut self, gd: GuardId, k: usize) {
        self.man_put(gd, [37.5, 6.0 + k as f64], [0.0, 0.0]);
        let (h, t) = (&mut self.g.gd[gd], self.g.time);
        (h.hold, h.pinned, h.dazzled_until, h.heard_up_to) = (true, true, t + 1e9, 1e12);
    }
    fn searches(&mut self, gd: GuardId, at: [f64; 2], fix: [f64; 2]) {
        self.man_put(gd, at, fix);
        let h = &mut self.g.gd[gd];
        (h.hold, h.pinned, h.dazzled_until, h.heard_up_to) = (false, false, -1.0, 1e12);
        (h.script, h.task, h.body_duty) = (None, None, None);
        (h.state, h.search_t, h.react_t, h.awareness, h.search_plan, h.fix_ref) = (game::game::GuardState::Search, 0.0, 0.0, 0.8, None, None);
        h.last_known = Some([fix[0], 0.0, fix[1]]);
    }
    fn spots(&self, gd: GuardId) -> Vec<([f64; 2], f64, f64, bool, f64)> {
        self.g.gd[gd].search_plan.as_ref().map_or(Vec::new(), |p| p.spots.iter().map(|s| ([s.pos[0], s.pos[2]], s.dark, s.cover, s.kind == game::guards::SearchSpotKind::Open, s.plaus)).collect())
    }
    fn looking_at(&self, gd: GuardId) -> Option<([f64; 2], f64, f64, bool)> {
        let p = self.g.gd[gd].search_plan.as_ref()?;
        if p.phase != game::guards::SearchPlanPhase::Look || p.current < 0.0 { return None; }
        let s = &p.spots[p.current as usize];
        Some(([s.pos[0], s.pos[2]], s.dark, s.cover, s.kind == game::guards::SearchSpotKind::Open))
    }
    fn door_shut_locked(&mut self, name: &str, locked: bool) {
        let d = game::doors::doors_by_name(&self.g, name).unwrap_or_else(|| panic!("no door '{name}'"));
        game::doors::door_set_angle(&mut self.g, d, 0.0);
        game::doors::door_set_latched(&mut self.g, d, true);
        game::doors::door_set_locked(&mut self.g, d, locked);
    }
    fn place_light(&mut self, at: [f64; 2]) -> (f64, f64, f64) {
        let high = |p: &mut Play, y: f64| { let q = [at[0], y, at[1]]; (game::guards::light_estimate_at(&mut p.g, q, Some(&mut |_g, _o| true), None, None), game::nvg::nvg_place_light(&mut p.g, q)) };
        let (a, b) = (high(self, 1.2), high(self, 1.6));
        let (e, m) = (a.0.max(b.0), a.1.max(b.1));
        (game::game::game_visibility_of(&self.g, e), game::game::game_visibility_of(&self.g, m), game::nvg::tube_vis(&self.g, m))
    }
    fn camera_state(&self) -> game::sim_lightshome::CameraState { game::sim_lightshome::camera_machine_state(&self.g, self.g.devices.cameras[0]) }
    fn camera_seen(&self) -> f64 { game::sim_lightshome::camera_machine_seen(&self.g, self.g.devices.cameras[0]) }
    fn beam_trips(&self, k: usize) -> f64 { game::sim_lightshome::laser_machine_trips(&self.g, self.g.devices.lasers[k]) }
    fn sent_to_look(&self) -> Option<GuardId> { self.g.guards.iter().copied().find(|&m| matches!(self.g.gd[m].task, Some(game::game::GuardTask::Arm0(game::game::GuardTaskArm0 { kind: game::game::GuardTaskArm0Kind::Device, .. })))) }
    fn sam_goes_to(&mut self, at: [f64; 2], secs: f64) -> bool {
        let keys = ["KeyW", "KeyS", "KeyA", "KeyD"];
        let mut got = false;
        for _ in 0..(secs * 60.0) as usize {
            let (me, b) = (self.sam_at(), game::camera::follow_camera_planar_basis(&self.g, self.host.cam));
            let (dx, dz) = (at[0] - me[0], at[1] - me[1]);
            let d = (dx * dx + dz * dz).sqrt();
            if d < 0.2 { got = true; break; }
            let (a, r) = ((dx * b.fwd[0] + dz * b.fwd[2]) / d, (dx * b.right[0] + dz * b.right[2]) / d);
            for (k, on) in keys.iter().zip([a > 0.38, a < -0.38, r < -0.38, r > 0.38]) { if on { self.key_down(k); } else { self.key_up(k); } }
            self.frame();
        }
        for k in keys { self.key_up(k); }
        got
    }
    fn sam_crouched(&mut self, down: bool) {
        let pl = self.g.player;
        if self.g.pl[pl].crouch != down { self.key_down(CROUCH_KEY); self.frame(); self.key_up(CROUCH_KEY); self.run(0.5); }
    }
}
const CROUCH_KEY: &str = "KeyC";
