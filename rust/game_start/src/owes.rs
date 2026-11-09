//! WHAT A WHOLE RUN OWES, every tick, whoever plays it: `tick` once a frame after the step. A FAULT is a thing the game may never do (nobody a number that is none or off the level, nobody inside a static box, no man
//! jumping, no tracer past its pool, doors and the alarm's clock coherent, stages only forward). A COUNT is a smell: a teammate or a hostage killed by the team, a man stuck under an order with a mark, two of the team
//! on one spot. Counted and not judged: it is a number a reader reads. It only asks: a run with it and a run without it are the same game.
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use game::game::{Game, GuardState, MissionStage};
use game::ids::{DoorId, GuardId};
use game::teams::Team;

/// metres in a tick nobody walks: more than this with no word for it is a man popping
const POP: f64 = 1.5;
/// how far a push out of the statics may go before a man counts as inside them (a body leans a few centimetres into a wall)
const IN_BOX: f64 = 0.06;
/// a man under an order with a mark who has not covered this in that long is stuck, and is counted again no sooner than the last. (The game's own watch gives up on a walk sooner than this, and says "can't get
/// through": that is counted too, or a man who can't get there would count only where something holds that watch off)
const STUCK_M: f64 = 0.25;
const STUCK_SECS: f64 = 4.0;
const STUCK_AGAIN: f64 = 10.0;
/// two of the team this near for that long are on one spot
const OVERLAP_M: f64 = 0.3;
const OVERLAP_SECS: f64 = 0.75;
const MAX_SPEED: f64 = 6.0;
const MAX_LEAF_SPEED: f64 = 40.0;

#[derive(Clone, Copy, Default)]
struct Track { at: [f64; 3], up: bool, mark: [f64; 3], mark_t: f64, want_t: f64, wanted: bool, said_t: f64, short: bool }

/// the fault's kind, who, and in the words of its first time
#[derive(Clone, Debug)]
pub struct Fault { pub kind: &'static str, pub who: String, pub t: f64, pub words: String, pub times: u32 }

#[derive(Default)]
pub struct Owes {
    pub faults: Vec<Fault>,
    pub friendly: u32,
    pub stuck: u32,
    pub overlap: u32,
    men: Vec<(GuardId, Track)>,
    sam: Option<[f64; 3]>,
    pairs: Vec<((usize, usize), f64, f64)>,
    stage: Option<u32>,
    run: Option<u32>,
}

fn rank(s: MissionStage) -> u32 { match s { MissionStage::Infiltrate => 0, MissionStage::Drive => 1, MissionStage::Exfil => 2, MissionStage::Done | MissionStage::Failed => 3 } }
fn flat(a: [f64; 3], b: [f64; 3]) -> f64 { game::m::hyp2(a[0] - b[0], a[2] - b[2]) }
fn number(p: [f64; 3]) -> bool { p.iter().all(|v| v.is_finite()) }

impl Owes {
    pub fn new() -> Owes { Owes::default() }
    fn fault(&mut self, kind: &'static str, who: &str, t: f64, words: String) {
        if let Some(f) = self.faults.iter_mut().find(|f| f.kind == kind && f.who == who) { f.times += 1; return; }
        self.faults.push(Fault { kind, who: who.to_string(), t, words, times: 1 });
    }
    /// the counts, for a line a reader reads
    pub fn line(&self) -> String { format!("friendly {} · stuck {} · overlap {}", self.friendly, self.stuck, self.overlap) }

    pub fn tick(&mut self, g: &mut Game) {
        let t = g.time;
        // (a restart is a new run: nobody jumped)
        if self.run != Some(g.mission.serial) { self.run = Some(g.mission.serial); self.men.clear(); self.sam = None; self.pairs.clear(); self.stage = None; }
        self.sam_owes(g, t);
        self.men_owe(g, t);
        self.doors_owe(g, t);
        self.alarm_owes(g, t);
        self.tracers_owe(g, t);
        let r = rank(g.mission.stage);
        if let Some(was) = self.stage { if r < was { self.fault("stage", "the mission", t, format!("the stage went back to {}", g.mission.stage.as_str())); } }
        self.stage = Some(r);
        self.overlaps(g, t);
    }
    fn in_a_box(&mut self, g: &mut Game, who: &str, at: [f64; 3], r: f64, y1: f64, t: f64) {
        let mut q = at;
        if game::scene_collision::collide_circle(g, &mut q, r, 0.3, y1, Some(1.0)) && flat(q, at) > IN_BOX { self.fault("in a box", who, t, format!("{:.2} m inside a static box at ({:.2}, {:.2})", flat(q, at), at[0], at[2])); }
    }
    fn on_the_level(&mut self, who: &str, p: [f64; 3], t: f64) {
        if !number(p) { self.fault("no number", who, t, format!("stands at {:?}", p)); return; }
        let w = game::scene_world::world();
        if p[0] < w.min[0] - 0.5 || p[0] > w.max[0] + 0.5 || p[2] < w.min[2] - 0.5 || p[2] > w.max[2] + 0.5 { self.fault("off the level", who, t, format!("stands at ({:.1}, {:.1})", p[0], p[2])); }
    }
    fn sam_owes(&mut self, g: &mut Game, t: f64) {
        let pl = g.player;
        let (p, down) = (g.ch[g.pl[pl].ch].pos, g.pl[pl].down);
        self.on_the_level("Sam", p, t);
        if !number(p) { return; }
        if let (Some(was), false) = (self.sam, down) { if flat(was, p) > POP { self.fault("popped", "Sam", t, format!("{:.2} m in a tick", flat(was, p))); } }
        self.sam = Some(p);
        if !down { self.in_a_box(g, "Sam", p, 0.22, 1.4, t); }
    }
    fn men_owe(&mut self, g: &mut Game, t: f64) {
        for gd in g.guards.clone() {
            let (c, who) = (g.gd[gd].ch, g.gd[gd].callsign.clone());
            let (p, up) = (g.ch[c].pos, g.gd[gd].state != GuardState::Dead);
            self.on_the_level(&who, p, t);
            if !number(p) { continue; }
            let (speed, aware) = (g.gd[gd].speed, g.gd[gd].awareness);
            if !speed.is_finite() || speed.abs() > MAX_SPEED { self.fault("speed", &who, t, format!("{} m/s", speed)); }
            if !(-1e-6..=1.0 + 1e-6).contains(&aware) { self.fault("awareness", &who, t, format!("{}", aware)); }
            if !up && g.gd[gd].script.is_some() { self.fault("dead and scripted", &who, t, String::from("a dead man has a script")); }
            if !up != !g.ch[c].alive { self.fault("alive", &who, t, format!("{:?} and his body alive {}", g.gd[gd].state, g.ch[c].alive)); }
            if !up && (g.gd[gd].down_kind.is_none() || g.gd[gd].down_by.is_none()) { self.fault("down with no how", &who, t, String::from("a dead man with no word of how")); }
            if up && (g.gd[gd].down_kind.is_some() || g.gd[gd].down_by.is_some()) { self.fault("how on the living", &who, t, String::from("a living man carries how he went down")); }
            if up { self.in_a_box(g, &who, p, 0.22, 1.4, t); }
            let k = match self.men.iter().position(|m| m.0 == gd) {
                Some(k) => k,
                None => { self.men.push((gd, Track { at: p, up, mark: p, mark_t: t, want_t: t, wanted: false, said_t: -100.0, short: false })); continue; }
            };
            let was = self.men[k].1;
            if up && was.up && flat(was.at, p) > POP { self.fault("popped", &who, t, format!("{:.2} m in a tick, from ({:.2}, {:.2}) to ({:.2}, {:.2})", flat(was.at, p), was.at[0], was.at[2], p[0], p[2])); }
            if was.up && !up && self.by_the_team(g, gd) { self.friendly += 1; }
            let (wants, short) = self.under_order(g, gd, p, t);
            let m = &mut self.men[k].1;
            (m.at, m.up) = (p, up);
            if flat(p, m.mark) > STUCK_M { (m.mark, m.mark_t) = (p, t); }
            if !wants || !m.wanted { m.want_t = t; }
            m.wanted = wants;
            let gave_up = short && !m.short;
            m.short = short;
            if gave_up || (m.wanted && t - m.mark_t.max(m.want_t) > STUCK_SECS && t - m.said_t > STUCK_AGAIN) { m.said_t = t; self.stuck += 1; }
        }
    }
    /// a teammate, or a hostage, down by one of the team
    fn by_the_team(&self, g: &Game, gd: GuardId) -> bool {
        if g.gd[gd].team != Team::Operative && !g.rescue.hostages.contains(&gd) { return false; }
        let k = g.gd[gd].killer_id;
        k == game::consts::PLAYER_ID || g.guards.iter().any(|&x| g.gd[x].team == Team::Operative && g.ch[g.gd[x].ch].id == k)
    }
    /// under an order with a mark he is not on, not in a fight, not blinded: does he want to be walking, and has the game given up on that walk
    fn under_order(&self, g: &Game, gd: GuardId, p: [f64; 3], t: f64) -> (bool, bool) {
        let seat = game::seats::here(g);
        let Some(a) = game::orders::squad_orders_of(g, &seat, gd).map(|a| &g.tab.orders_assignment[a]) else { return (false, false); };
        let h = &g.gd[gd];
        let fighting = game::roomfight::room_fight_engaged(g, gd) || (h.script.is_none() && matches!(h.state, GuardState::Alert | GuardState::Search));
        let blind = h.dazzled_until > t || h.stagger_until > t;
        let off_it = !a.arrived && a.post.is_some_and(|q| flat(p, q) > 0.6);
        (h.state != GuardState::Dead && off_it && !fighting && !blind, a.there == "stopped short")
    }
    fn overlaps(&mut self, g: &Game, t: f64) {
        let mut team: Vec<(usize, [f64; 3])> = Vec::new();
        team.push((usize::MAX, g.ch[g.pl[g.player].ch].pos));
        for (i, &gd) in g.guards.iter().enumerate() { if g.gd[gd].team == Team::Operative && g.gd[gd].state != GuardState::Dead { team.push((i, g.ch[g.gd[gd].ch].pos)); } }
        for a in 0..team.len() { for b in a + 1..team.len() {
            let key = (team[a].0, team[b].0);
            let at = self.pairs.iter().position(|p| p.0 == key);
            if flat(team[a].1, team[b].1) >= OVERLAP_M { if let Some(k) = at { if t - self.pairs[k].2 > STUCK_AGAIN { self.pairs.swap_remove(k); } else { self.pairs[k].1 = f64::INFINITY; } } continue; }
            match at {
                None => self.pairs.push((key, t, -100.0)),
                Some(k) => {
                    if self.pairs[k].1 == f64::INFINITY { self.pairs[k].1 = t; }
                    if t - self.pairs[k].1 > OVERLAP_SECS && t - self.pairs[k].2 > STUCK_AGAIN { self.pairs[k].2 = t; self.overlap += 1; }
                }
            }
        } }
    }
    fn doors_owe(&mut self, g: &Game, t: f64) {
        use game::doors as d;
        for k in 0..g.doors.list.len() {
            let id: DoorId = g.doors.list[k];
            let (a, name) = (d::door_angle(g, id), &g.dr[id].def.name);
            if d::door_latched(g, id) && a.abs() > 1e-3 { self.fault("door", name, t, format!("latched at angle {:.4}", a)); }
            if d::door_locked(g, id) && d::door_lock_broken(g, id) { self.fault("door", name, t, String::from("locked and its lock broken")); }
            let pick = d::door_pick(g, id);
            if !(0.0..=1.0).contains(&pick) { self.fault("door", name, t, format!("pick {}", pick)); }
            if d::door_off(g, id) { continue; }
            if a < d::door_min_a(g, id) - 1e-6 || a > d::door_max_a(g, id) + 1e-6 { self.fault("door", name, t, format!("angle {:.3} outside [{}, {}]", a, d::door_min_a(g, id), d::door_max_a(g, id))); }
            if d::door_vel(g, id).abs() > MAX_LEAF_SPEED { self.fault("door", name, t, format!("swinging at {:.1}", d::door_vel(g, id))); }
        }
    }
    fn alarm_owes(&mut self, g: &Game, t: f64) {
        let (e, clock) = (g.escalation, g.escalation_t);
        if e != 0.0 && e != 1.0 && e != 2.0 { self.fault("alarm", "the floor", t, format!("level {}", e)); }
        if e == 0.0 && clock != -1.0 { self.fault("alarm", "the floor", t, format!("level 0 with a clock at {}", clock)); }
        if e > 0.0 && !(clock > 0.0) { self.fault("alarm", "the floor", t, format!("level {} with no clock", e)); }
        if e < 2.0 { for &gd in &g.guards { if g.gd[gd].state != GuardState::Dead && (g.gd[gd].leader.is_some() || g.gd[gd].post.is_some()) { let w = g.gd[gd].callsign.clone(); self.fault("lockdown duty", &w, t, format!("a leader or a post at level {}", e)); } } }
    }
    fn tracers_owe(&mut self, g: &Game, t: f64) {
        let pool = &g.tracers;
        let n = pool.pool.iter().filter(|x| x.alive).count() as f64;
        if n != pool.live || n > pool.params.max_live { self.fault("tracers", "the pool", t, format!("{} alive, {} said, of {}", n, pool.live, pool.params.max_live)); }
        for x in pool.pool.iter().filter(|x| x.alive) {
            if !(x.s >= 0.0 && x.s <= x.l + 1e-6) || !number(x.from) || !number(x.dir) || !x.v.is_finite() { self.fault("tracers", "a round", t, format!("s {} of {}, from {:?}", x.s, x.l, x.from)); }
        }
    }
}

/// `Play::played_out` with the run held to its owes every tick: did he get there
#[cfg(feature = "std")]
pub fn played_out(p: &mut crate::play::Play, script: &mut dyn crate::player::Script, secs: f64, owes: &mut Owes) -> bool {
    for _ in 0..(secs * 60.0).round() as usize {
        if script.over() { return true; }
        p.played_by(script);
        owes.tick(&mut p.g);
    }
    script.over()
}
