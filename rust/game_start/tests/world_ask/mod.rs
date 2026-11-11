//! Questions the world's tests ask of a stood game that `play.rs` has no reader for, and the few stagings that have no key. Each answers one thing, by
//! its own name, so that when a field moves one line here follows it.
#![allow(dead_code)]
use game::ids::*;
use game_start::json::{self, Json};
use game::savestate::control::Moment;
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

/// …saying what it does, whatever its paint's row says
pub fn a_box_of(id: &str, c: [f64; 3], h: [f64; 3], material: &str, does: &str) -> Json {
    json::obj(vec![("id", json::s(id)), ("group", json::s("added")), ("c", json::nums(&c)), ("h", json::nums(&h)), ("material", json::s(material)), ("does", json::s(does))])
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
        self.g.events.push(game::game::GameEvent { kind: game::director::EventKind::Prop, pos: [at[0], 0.0, at[1]], time, loud: false, level: Some(level), who: Some(0.0), lure: None, serial, ..game::game::GameEvent::default() });
    }
    fn footfall_at(&mut self, at: [f64; 2], level: f64) {
        let (time, serial) = (self.g.time, self.g.serial());
        self.g.events.push(game::game::GameEvent { kind: game::director::EventKind::Step, pos: [at[0], 0.0, at[1]], time, loud: false, level: Some(level), serial, ..game::game::GameEvent::default() });
    }
    fn slam_at(&mut self, at: [f64; 2]) {
        let (time, serial) = (self.g.time, self.g.serial());
        self.g.events.push(game::game::GameEvent { kind: game::director::EventKind::Door, pos: [at[0], 0.0, at[1]], time, loud: true, level: Some(1.0), who: Some(1.0), serial, ..game::game::GameEvent::default() });
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
        self.g.dr[d].leaf.latched = false;
        self.g.dr[d].leaf.angle = turn;
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
        let who = self.g.player_id(self.g.player);
        game::frag::blast(&mut self.g, [at[0], 0.05, at[1]], &game::frag::BlastSpec { who, name: Some(String::from("40 mm grenade")), ..game::frag::BlastSpec::default() })
    }
    fn hit_of(&self, r: &game::frag::BlastReport, gd: GuardId) -> Option<game::frag::BlastHit> { let id = self.id_of(gd); r.hits.iter().find(|h| h.id == id).cloned() }
    fn reeled_at(&self, gd: GuardId) -> f64 { self.g.cond[self.g.gd[gd].ch].fold_until }
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
        let (who, lift) = (self.g.player_id(self.g.player), self.g.k.frag.lift);
        game::frag::blast(&mut self.g, [at[0], at[1] - lift, at[2]], &game::frag::BlastSpec { who, name: Some(String::from("40 mm grenade")), ..game::frag::BlastSpec::default() })
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
        let n0 = self.g.sets[self.g.seat].wall_charge.set.len();
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
            if self.until(secs, |p| p.g.sets[p.g.seat].wall_charge.set.len() > n0).is_some() { return Ok(*self.g.sets[self.g.seat].wall_charge.set.last().unwrap()); }
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
    fn lamps_burning(&self) -> usize { self.g.guards.iter().filter(|&&m| self.g.team(m) == game::teams::Team::Hostile && self.lamp_on(m)).count() }
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
    fn mains_pulled(&mut self) { game::game::game_set_blackout(&mut self.g, 86400.0, game::lighting::SetBlackoutCause::Hand); }
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
    /// he stands calm at a post, looking a way, eyes and ears open, held there
    fn posted_calm(&mut self, gd: GuardId, at: [f64; 2], face: [f64; 2]);
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
    fn camera_state(&self) -> game::fixtures::CameraState;
    /// how much it has seen
    fn camera_seen(&self) -> f64;
    /// how many times the beam has been tripped, by its place in the level's list
    fn beam_trips(&self, k: usize) -> f64;
    /// the walk grid, as a number
    fn level_nav(&self) -> u64;
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
        (h.duty.holds, h.duty.pinned, self.g.cond[h.ch].blind_until, h.heard_up_to) = (true, true, t + 1e9, 1e12);
    }
    fn posted_calm(&mut self, gd: GuardId, at: [f64; 2], face: [f64; 2]) {
        self.man_put(gd, at, face);
        let h = &mut self.g.gd[gd];
        (h.duty.pinned, self.g.cond[h.ch].blind_until, h.heard_up_to, h.duty.holds) = (false, -1.0, 0.0, true);
        (h.state, h.awareness, h.last_known, h.react_t, h.search_t, h.search_plan) = (game::game::GuardState::Patrol, 0.0, None, 0.0, 0.0, None);
        (h.script, h.errand, h.body_duty) = (None, None, None);
    }
    fn searches(&mut self, gd: GuardId, at: [f64; 2], fix: [f64; 2]) {
        self.man_put(gd, at, fix);
        let h = &mut self.g.gd[gd];
        (h.duty.holds, h.duty.pinned, self.g.cond[h.ch].blind_until, h.heard_up_to) = (false, false, -1.0, 1e12);
        (h.script, h.errand, h.body_duty) = (None, None, None);
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
        self.g.dr[d].leaf.angle = 0.0;
        self.g.dr[d].leaf.latched = true;
        self.g.dr[d].leaf.locked = locked;
    }
    fn place_light(&mut self, at: [f64; 2]) -> (f64, f64, f64) {
        let high = |p: &mut Play, y: f64| { let q = [at[0], y, at[1]]; (game::guards::light_estimate_at(&mut p.g, q, Some(&mut |_g, _o| true), None, None), game::nvg::nvg_place_light(&mut p.g, q)) };
        let (a, b) = (high(self, 1.2), high(self, 1.6));
        let (e, m) = (a.0.max(b.0), a.1.max(b.1));
        (game::game::game_visibility_of(&self.g, e), game::game::game_visibility_of(&self.g, m), game::nvg::tube_vis(&self.g, m))
    }
    fn camera_state(&self) -> game::fixtures::CameraState { self.g.tab.devices_camera_rt[self.g.devices.cameras[0]].state }
    fn camera_seen(&self) -> f64 { self.g.tab.devices_camera_rt[self.g.devices.cameras[0]].seen }
    fn beam_trips(&self, k: usize) -> f64 { self.g.tab.devices_laser_rt[self.g.devices.lasers[k]].trips as f64 }
    fn level_nav(&self) -> u64 { self.g.col.nav.blocked.iter().fold(7u64, |h, b| h.wrapping_mul(31).wrapping_add(*b as u64)) }
    fn sent_to_look(&self) -> Option<GuardId> { self.g.guards.iter().copied().find(|&m| matches!(self.g.gd[m].errand, Some(game::game::Errand::At(game::game::ErrandAt { kind: game::game::ErrandThing::Device, .. })))) }
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

/// Furniture, and a game kept

pub trait FurnAsk {
    /// a blast at a point, by Sam's hand, as a wall charge is told it: `power` Some(0) is one of no strength for the wall. How many walls it holed.
    fn wall_blast(&mut self, at: [f64; 3], power: Option<f64>) -> f64;
    /// the piece of furniture that stands at a place: its statics
    fn furniture_at(&mut self, at: [f64; 2]) -> Vec<f64>;
    /// how many of those statics stand
    fn standing(&self, of: &[f64]) -> usize;
    /// the prop nearest a place
    fn prop_near(&self, at: [f64; 2]) -> PropId;
    /// where a prop is
    fn prop_at(&self, it: PropId) -> [f64; 2];
    /// every static's flags, every prop's pose and presence, the walk grid: one line a part, to compare
    fn level_as_it_stands(&self) -> [String; 3];
    /// the game kept whole, as a host keeps it
    fn keep(&self) -> Moment;
    /// and put back
    fn put_back(&mut self, k: &Moment);
}

impl FurnAsk for Play {
    fn wall_blast(&mut self, at: [f64; 3], power: Option<f64>) -> f64 {
        let o = game::wallbreak::BlastOpts { who: Some(game::consts::PLAYER_ID), chaos: Some(false), power, ..Default::default() };
        game::destruction::destruction_blast(&mut self.g, at, Some(o)).walls
    }
    fn furniture_at(&mut self, at: [f64; 2]) -> Vec<f64> {
        game::furnblast::furniture_first_to_go(&mut self.g, [at[0], 1.0, at[1]], game::furnblast::BlastSource::Wall);   // (the index is made when it is first asked)
        let groups = self.g.destr.furn.groups.as_ref().expect("the furniture's index");
        let gr = groups.iter().find(|x| at[0] >= x.a.x0 - 0.05 && at[0] <= x.a.x1 + 0.05 && at[1] >= x.a.z0 - 0.05 && at[1] <= x.a.z1 + 0.05).unwrap_or_else(|| panic!("no furniture stands at {at:?}"));
        gr.members.iter().map(|&m| self.g.tab.furnblast_member[m].i).collect()
    }
    fn standing(&self, of: &[f64]) -> usize {
        let hidden = game::js::to_i32(game::scene_boxes::box_flag().no_primary);
        of.iter().filter(|&&i| game::js::to_i32(self.g.tab.boxes[self.g.col.boxes[i as usize]].flags) & hidden == 0).count()
    }
    fn prop_near(&self, at: [f64; 2]) -> PropId {
        let d = |&it: &PropId| { let q = self.prop_at(it); (q[0] - at[0]).hypot(q[1] - at[1]) };
        *self.g.props.all.iter().min_by(|a, b| d(a).partial_cmp(&d(b)).unwrap()).expect("a prop")
    }
    fn prop_at(&self, it: PropId) -> [f64; 2] { [self.g.tab.props_prop[it].x, self.g.tab.props_prop[it].z] }
    fn level_as_it_stands(&self) -> [String; 3] {
        let n = self.g.destr.walls.authored as usize;
        let flags: Vec<String> = self.g.col.boxes.iter().take(n).map(|&b| format!("{}", self.g.tab.boxes[b].flags)).collect();
        let props: Vec<String> = self.g.props.props.iter().map(|&it| { let q = self.prop_at(it); format!("{it:?}:{:.4},{:.4}", q[0], q[1]) }).collect();
        let nav: u64 = self.g.col.nav.blocked.iter().fold(7u64, |h, b| h.wrapping_mul(31).wrapping_add(*b as u64));
        [flags.join(","), props.join(" "), format!("{nav}")]
    }
    fn keep(&self) -> Moment { Moment::of(&self.g) }
    fn put_back(&mut self, k: &Moment) { self.g = k.laid(self.g.sim); }
}

/// The net
pub trait NetAsk {
    /// the men the room clear dealt a room of this id
    fn men_of_room(&self, room: &str) -> Vec<GuardId>;
    /// how shaken he is
    fn shaken(&self, gd: GuardId) -> f64;
    /// is he dealt steady
    fn steady(&mut self, gd: GuardId) -> bool;
    /// is this one of the net's lines of a kind
    fn net_line(&self, text: &str, kind: &str, name: &str) -> bool;
    /// the men of his squad
    fn squad_men_of(&self, gd: GuardId) -> Vec<GuardId>;
    /// the net's check of the squads falls due
    fn net_check_due(&mut self, secs: f64);
    /// when any squad first filed an event of this kind
    fn net_event_at(&self, what: game::enemysquads::SquadEventWhat) -> Option<f64>;
    /// a man is put down without a sound, by nobody anyone heard
    fn put_down(&mut self, gd: GuardId);
}

impl NetAsk for Play {
    fn men_of_room(&self, room: &str) -> Vec<GuardId> { self.g.staged.rooms.iter().find(|r| r.def.id == room).map_or(Vec::new(), |r| r.defenders.clone()) }
    fn shaken(&self, gd: GuardId) -> f64 { self.g.m.morale.state_of.get(gd).map_or(0.0, |m| m.shock) }
    fn steady(&mut self, gd: GuardId) -> bool { game::morale::training_of(&mut self.g, gd) == game::morale::TrainingOf::Steady }
    fn net_line(&self, text: &str, kind: &str, name: &str) -> bool { game::netquiet::lines().get(String::from(kind)).map_or(false, |r| r.iter().any(|l| l.replace("{name}", name) == text)) }
    fn squad_men_of(&self, gd: GuardId) -> Vec<GuardId> { game::enemysquads::squad_of(&self.g, gd).map_or(Vec::new(), |q| q.men.clone()) }
    fn net_check_due(&mut self, secs: f64) { let at = self.g.time + secs; if let Some(st) = self.g.m.enemysquads.states.as_mut() { if let Some(n) = st.nc.as_mut() { n.next_at = at; } } }
    fn net_event_at(&self, what: game::enemysquads::SquadEventWhat) -> Option<f64> {
        let st = self.g.m.enemysquads.states.as_ref()?;
        st.squads.iter().flat_map(|&q| self.g.tab.enemysquads_squad[q].events.iter()).filter(|e| e.what == what).map(|e| e.t).fold(None, |a: Option<f64>, t| Some(a.map_or(t, |x| x.min(t))))
    }
    fn put_down(&mut self, gd: GuardId) { game::combat::kill_guard(&mut self.g, gd, [0.0, 0.0, 1.0], Some(true), None, None, game::game::With::Hand); }
}

/// A hostage rescue
pub trait RescueAsk {
    /// the men posted with the hostages, by combat's own call: shot, loud or struck down without a sound
    fn hostage_room_falls(&mut self, loud: bool);
    /// the middle of the hostage room
    fn hostage_room_middle(&self) -> [f64; 2];
    /// the hostages who live
    fn hostages_up(&self) -> Vec<GuardId>;
    /// the men of theirs who live
    fn theirs_up(&self) -> Vec<GuardId>;
    /// is this place in the hostage room
    fn in_hostage_room(&self, at: [f64; 2]) -> bool;
    /// the rescue's own log: when a line that begins so first came
    fn rescue_said(&self, starts: &str) -> Option<f64>;
    /// every line of the rescue's own log that begins so
    fn rescue_lines(&self, starts: &str) -> Vec<String>;
    /// Sam beside a hostage, her row on the interaction key tapped: was it there to take
    fn hostage_row_taken(&mut self, h: GuardId, verb: game::game::Verb) -> bool;
    /// is she behind Sam
    fn led_by_sam(&self, h: GuardId) -> bool;
    /// where the way out is
    fn way_out(&self) -> [f64; 2];
    /// Sam walks the grid's way to a place at a walker's pace, set down a frame at a time: no sound of his, no gun of theirs
    fn sam_walks_the_grid(&mut self, to: [f64; 2]);
    /// a loud round is heard where the men of the room are
    fn a_loud_round_at_the_room(&mut self);
    /// how the run came out: rescued, and its words
    fn rescue_result(&self) -> Option<(bool, String)>;
}

impl RescueAsk for Play {
    fn hostage_room_falls(&mut self, loud: bool) {
        let cast: Vec<GuardId> = self.g.staged.foes.iter().copied().filter(|&m| self.g.rescue.defs.get(m).is_some_and(|d| d.in_room)).collect();
        for m in cast {
            let how = if loud { game::game::DownKind::Shot } else { game::game::DownKind::Struck };
            game::combat::kill_guard(&mut self.g, m, [1.0, 0.0, 0.0], Some(!loud), Some(how), Some(game::game::DownBy::Player), game::game::With::Hand);
        }
    }
    fn hostage_room_middle(&self) -> [f64; 2] { let b = game::rescue::rescue_bounds(&self.g).expect("a hostage room"); [(b.x0 + b.x1) / 2.0, (b.z0 + b.z1) / 2.0] }
    fn hostages_up(&self) -> Vec<GuardId> { self.g.staged.captives.iter().copied().filter(|&m| !game::condition::down(&self.g, self.g.gd[m].ch)).collect() }
    fn theirs_up(&self) -> Vec<GuardId> { self.g.staged.foes.iter().copied().filter(|&m| !game::condition::down(&self.g, self.g.gd[m].ch)).collect() }
    fn in_hostage_room(&self, at: [f64; 2]) -> bool { let b = game::rescue::rescue_bounds(&self.g).expect("a hostage room"); at[0] >= b.x0 && at[0] <= b.x1 && at[1] >= b.z0 && at[1] <= b.z1 }
    fn rescue_said(&self, starts: &str) -> Option<f64> { self.g.rescue.log.iter().find(|l| l.what.starts_with(starts)).map(|l| l.t) }
    fn rescue_lines(&self, starts: &str) -> Vec<String> { self.g.rescue.log.iter().filter(|l| l.what.starts_with(starts)).map(|l| l.what.clone()).collect() }
    fn hostage_row_taken(&mut self, h: GuardId, verb: game::game::Verb) -> bool {
        let at = self.at(h);
        for k in 0..8 {
            let a = k as f64 * std::f64::consts::FRAC_PI_4;
            self.sam_put([at[0] + a.sin() * 1.2, at[1] + a.cos() * 1.2]);
            self.run(2.0 / 60.0);
            if self.g.sets[self.g.seat].nearby.iter().any(|r| r.guard == Some(h) && r.verb == Some(verb)) && game::game::game_select_verb(&mut self.g, game::game::InteractableKind::Guard, Some(verb)) {
                self.key_down("Space");
                self.frame();
                self.key_up("Space");
                self.frame();
                return true;
            }
        }
        false
    }
    fn led_by_sam(&self, h: GuardId) -> bool { let c = self.g.gd[h].civ.expect("a civilian"); self.g.cv[c].lead == Some(self.g.pl[self.g.player].ch) }
    fn way_out(&self) -> [f64; 2] { let o = game::rescueend::rescue_end_out(&self.g).expect("a way out"); [o[0], o[2]] }
    fn sam_walks_the_grid(&mut self, to: [f64; 2]) {
        let from = self.sam_at();
        let path = game::scene_collision::nav_grid_find_path(&mut self.g, [from[0], 0.0, from[1]], [to[0], 0.0, to[1]], Some(12000.0), None).unwrap_or_else(|| vec![[to[0], 0.0, to[1]]]);
        let mut q = from;
        for w in path {
            let (w, l) = ([w[0], w[2]], (w[0] - q[0]).hypot(w[2] - q[1]));
            let n = ((l / (2.4 / 60.0)).ceil() as usize).max(1);
            for i in 1..=n { if self.g.rescue.result.is_some() { return; } self.sam_put([q[0] + (w[0] - q[0]) * i as f64 / n as f64, q[1] + (w[1] - q[1]) * i as f64 / n as f64]); self.frame(); }
            q = w;
        }
    }
    fn a_loud_round_at_the_room(&mut self) {
        let m = self.g.staged.foes.iter().copied().find(|&m| self.g.rescue.defs.get(m).is_some_and(|d| d.in_room)).expect("a man in the room");
        let (at, time, serial) = (self.at(m), self.g.time, self.g.serial());
        self.g.events.push(game::game::GameEvent { kind: game::director::EventKind::Shot, pos: [at[0], 0.0, at[1]], time, loud: true, level: Some(1.0), who: Some(game::consts::PLAYER_ID), serial, ..game::game::GameEvent::default() });
    }
    fn rescue_result(&self) -> Option<(bool, String)> { self.g.rescue.result.as_ref().map(|r| (r.state == game::rescue::RescueResultState::Rescued, r.why.clone())) }
}

/// What a player would call the floor, a part a line: what a restart has to put back
pub fn the_floor(p: &mut Play) -> Vec<(&'static str, String)> {
    let danger = game::tactics::danger_doors(&mut p.g, None).len();
    let hits = game::glass::glass_state(&mut p.g).hits;
    let [statics, props, nav] = p.level_as_it_stands();
    let g = &p.g;
    let men: Vec<String> = g.guards.iter().map(|&m| { let q = p.at(m); format!("{} {:.3},{:.3} {:?} {:.3} {:?} {:?}", g.gd[m].callsign, q[0], q[1], p.state(m), p.awareness(m), p.fix(m), g.gd[m].errand.is_some()) }).collect();
    let doors: Vec<String> = g.doors.list.iter().map(|&d| format!("{:.3} {} {}", g.dr[d].leaf.angle, g.dr[d].leaf.latched, g.dr[d].leaf.locked)).collect();
    let lamps: Vec<String> = g.targets.iter().map(|&t| format!("{} {}", g.tab.fixture[t].on, g.tab.fixture[t].broken)).collect();
    vec![("men", men.join("; ")), ("doors", doors.join("; ")), ("lamps", lamps.join("; ")), ("mains", format!("{}", g.light_home.mains.active)), ("statics", statics), ("props", props), ("walk grid", nav),
        ("things thrown", format!("{}", g.items.items.len())), ("alarm", format!("{}", g.escalation)), ("dangerous doors", format!("{danger}")), ("charges", format!("{}", g.sets[g.seat].wall_charge.set.len())), ("glass hits", format!("{hits}"))]
}

/// What a review found
pub trait ReviewAsk {
    /// the fixture that is the first lit lamp of the floor
    fn first_lamp(&self) -> FixtureId;
    /// Sam's trigger on the box: the OCP knocks this lamp out
    fn lamp_knocked_out(&mut self, t: FixtureId);
    /// the game told that a lamp was shot dark
    fn lamp_shot_dark(&mut self, t: FixtureId);
    /// is it out for now
    fn lamp_disabled(&self, t: FixtureId) -> bool;
    /// a thing spawned as the game spawns it: (fired, seconds left to vent)
    fn thing(&mut self, kind: game::throwables::ItemKind, fired: Option<bool>, emit: Option<f64>) -> ItemId;
    /// the list of things
    fn things(&self) -> Vec<ItemId>;
    /// the list is emptied
    fn things_gone(&mut self);
    /// the errand a man is on, if any
    fn on_errand(&self, gd: GuardId) -> bool;
    /// a civilian's state
    fn civ_state(&self, gd: GuardId) -> game::civilians::CivState;
    /// has the chronicle a row of a kind
    fn civ_ko_rows(&self) -> usize;
}

impl ReviewAsk for Play {
    fn first_lamp(&self) -> FixtureId { self.g.targets.iter().copied().find(|&t| self.g.tab.fixture[t].on && self.g.tab.fixture[t].kind != game::fixtures::FixtureKind::Breaker).expect("a lamp lit") }
    fn lamp_knocked_out(&mut self, t: FixtureId) { let pl = self.g.player; self.g.pl[pl].ocp.charge = 1.0; self.g.sets[self.g.seat].aim_target = Some(t); game::game::game_ocp(&mut self.g, Some(t)); }
    fn lamp_shot_dark(&mut self, t: FixtureId) { game::game::game_light_event(&mut self.g, t, Some(game::lighting::LightEventCause::Shot), None); }
    fn lamp_disabled(&self, t: FixtureId) -> bool { self.g.tab.fixture[t].blind_until > self.g.time }
    fn thing(&mut self, kind: game::throwables::ItemKind, fired: Option<bool>, emit: Option<f64>) -> ItemId {
        let o = game::throwables::ThrowablesSpawnOpts { fired, emit_t: emit, ..game::throwables::ThrowablesSpawnOpts::default() };
        game::throwables::throwables_spawn(&mut self.g, kind, [5.0, 0.5, 5.0], [0.0; 3], Some(o))
    }
    fn things(&self) -> Vec<ItemId> { self.g.items.items.clone() }
    fn things_gone(&mut self) { self.g.items.items.clear(); }
    fn on_errand(&self, gd: GuardId) -> bool { self.g.gd[gd].errand.is_some() }
    fn civ_state(&self, gd: GuardId) -> game::civilians::CivState { self.g.cv[self.g.gd[gd].civ.expect("a civilian")].state }
    fn civ_ko_rows(&self) -> usize { self.g.chronicle.events.iter().filter(|e| format!("{:?}", e.kind).contains("CivilianKo") || format!("{:?}", e.kind).contains("CivilianKO")).count() }
}
