//! THE SAVE CONTROLLER, for a host that keeps games in memory: when a game is kept, under which name, and what becomes of one that is put back. What is kept is
//! the host's (a clone of the whole game and its block of rust/sim's): this says when, and the host answers with the game kept or put back.
//!
//! `tick` runs once after every step, on the frame as it now stands. Autosaves and saves by hand are two rings, neither of which pushes the other out; a run's saves are
//! its own, and a fresh run starts with none. F5 is read off the step's own record. A run is the mission that is up (`Mission.serial`). The books are the game's own `saves`, and go back with a game put
//! back, but for what belongs to the host and to the run as it is NOW: the shelf, the counts, the log and the switch.   (src/game/savestate.ts Checkpoints)
use super::*;
use crate::game::{Game, GuardState, MissionStage};
use crate::ids::*;
use crate::js::JsSet;
use crate::v3::{self, V3};
use alloc::rc::Rc;
use alloc::{boxed::Box, collections::BTreeMap, format, string::String, vec::Vec};

/// what the host is to do now
pub enum Do {
    /// keep the game as it stands, under this key; `rows` the run's chronicle first (game_run `checkpoint`)
    Keep { key: String, label: String },
    /// put back the game kept under this key
    Back { key: String },
    /// tell the player this
    Say(String),
}

fn key_of(slot: SlotInfoSlot, i: usize) -> String { format!("{}.{}", if slot == SlotInfoSlot::Auto { "auto" } else { "manual" }, i) }
fn ring(g: &Game, slot: SlotInfoSlot) -> usize { (if slot == SlotInfoSlot::Auto { g.k.savetimeline.auto } else { g.k.savetimeline.manual }) as usize }
fn note(g: &mut Game, what: String) {
    g.saves.log.push(CheckpointsLogItem {  });
    if g.saves.log.len() > 40 { g.saves.log.remove(0); }
}
fn here(g: &mut Game) -> String { let p = g.ch[g.pl[g.player].ch].pos; crate::chronicle::room_of(g, p) }
fn rank(s: MissionStage) -> u32 { match s { MissionStage::Infiltrate => 0, MissionStage::Drive => 1, MissionStage::Exfil => 2, MissionStage::Done | MissionStage::Failed => 3 } }

/// Why the game cannot be kept right now; `authored` = a tutorial's own checkpoint, taken whatever is on the floor.   (savestate.ts runNo, for a save in memory)
pub fn run_no(g: &Game, seat: &crate::seats::SeatRef, authored: bool) -> Option<String> {
    if g.aside || g.puppet.is_some() { return Some(String::from("nobody is playing")); }
    if g.seats.len() > 1 { return Some(String::from("two players (a checkpoint holds one player today)")); }
    if g.pl[seat.player].down { return Some(String::from("the player is down")); }
    if matches!(g.mission.stage, MissionStage::Failed | MissionStage::Done) { return Some(format!("the run is over ({}) \u{2014} nothing to come back to", g.mission.stage.as_str())); }
    if crate::verdict::verdict_ended(g).is_some() { return Some(String::from("the run is over \u{2014} nothing to come back to")); }
    if authored { return None; }
    if g.qrf.state != crate::qrf::QrfState::Idle { return Some(format!("the response team is {} \u{2014} no checkpoint inside the response", g.qrf.state.as_str())); }
    if g.qrf.silent_at >= 0.0 && !g.qrf.silent_spent { return Some(String::from("control is calling a net gone silent \u{2014} no checkpoint inside the response")); }
    None
}

/// Somebody who is no guard is about to bring the floor up.   (civilians.ts civilianCompromise)
fn civilian_compromise(g: &mut Game) -> Option<String> {
    use crate::civilians::{CivCause, CivState, PendKind};
    if !crate::civilians::civilians_on_floor(g) { return None; }
    for &cv in g.civilians.clone().iter() {
        if g.gd[cv].state == GuardState::Dead || g.gd[cv].held.is_some() { continue; }
        let Some(id) = g.gd[cv].civ else { continue };
        let c = &g.cv[id];
        match c.state {
            CivState::Startled => return Some(format!("{} has glimpsed something", c.name)),
            CivState::Alarmed | CivState::Flee | CivState::Report | CivState::Phone => return Some(format!("{} is about to bring somebody", c.name)),
            CivState::Cower if !c.reported && !c.complied && matches!(c.cause, Some(CivCause::Sam | CivCause::Gun | CivCause::Grabbed)) => return Some(format!("{} is hiding from you and nobody knows yet", c.name)),
            _ => {}
        }
    }
    let who = crate::civilians::floor_of(g).pending.iter().find(|p| !matches!(p.kind, PendKind::GreetReply | PendKind::ComplyLine)).map(|p| p.cv)?;
    Some(format!("word of {} is about to move the floor", g.gd[who].callsign))
}

/// With no team on the floor: what is in the air, in a man's hands or on its way down is waited out, as it was when such a save was a file.   (savestate.ts canSave)
fn unsettled(g: &mut Game) -> Option<String> {
    if g.clearing.is_some() { return Some(String::from("the lockdown pair is clearing rooms")); }
    if let Some(&x) = g.guards.iter().find(|&&x| g.gd[x].script.is_some() && g.gd[x].state != GuardState::Dead) { return Some(format!("{} is scripted", g.gd[x].callsign)); }
    if let Some(busy) = busy(g, &crate::seats::here(g)) { return Some(format!("the player is {busy}")); }
    for it in g.items.items.clone() {
        let live = matches!(crate::throwables::item_kind(g, it), crate::throwables::ItemKind::Smoke | crate::throwables::ItemKind::Flash | crate::throwables::ItemKind::Frag) && !crate::throwables::item_fired(g, it);
        if live || !crate::throwables::item_settled(g, it) { return Some(String::from("a thrown thing is still in the air or on its fuse")); }
    }
    for &x in g.guards.iter().chain(g.civilians.iter()) {
        let c = &g.ch[g.gd[x].ch];
        if c.ragdoll.is_some_and(|r| !g.tab.ragdoll[r].sleeping) { return Some(format!("{}'s body is still falling", g.gd[x].callsign)); }
    }
    if !g.rings.is_empty() || !g.shells.is_empty() || crate::stickycam::sticky_flying(g, &crate::seats::here(g)) { return Some(String::from("a round or a camera is in flight")); }
    None
}
/// what his hands or his feet are in the middle of   (savestate.ts playerBusy)
fn busy(g: &Game, seat: &crate::seats::SeatRef) -> Option<&'static str> {
    let p = &g.pl[seat.player];
    let a = &g.ch[p.ch].anim;
    if p.holding.is_some() { return Some("holding somebody"); }
    if p.takedown.is_some() { return Some("mid-takedown"); }
    if p.dragging.is_some() { return Some("hauling a body"); }
    if p.picking.is_some() { return Some("picking a lock"); }
    if p.wanding.is_some() { return Some("looking under a door"); }
    if p.coding.is_some() { return Some("at a keypad"); }
    if p.breaker_move.is_some() { return Some("at the breaker"); }
    if p.easing.is_some() { return Some("easing a door"); }
    if p.door_move.is_some() { return Some("working a door"); }
    if crate::wallcharge::wall_charge_setting(g, seat) || p.charging.is_some() || p.work_stow.is_some() { return Some("setting a charge"); }
    if p.kick.is_some() || (p.door_kick_tail && crate::character::anim_kicking(g, p.ch)) { return Some("kicking a door"); }
    if p.pending_throw.is_some() || p.throw_held || crate::character::anim_in_throw(g, p.ch) { return Some("throwing"); }
    if p.launch_held.is_some() { return Some("lining up the launcher"); }
    if crate::weapons::firearm_reloading(g, p.pistol) || crate::weapons::firearm_reloading(g, p.rifle) { return Some("reloading"); }
    if a.nv.is_some() { return Some("flipping his goggles"); }
    if a.big_tool.is_some() { return Some("putting his tool back"); }
    if crate::character::anim_kit_busy(g, p.ch) { return Some("changing what is in his hands"); }
    if g.mission.drive_t >= 0.0 { return Some("pulling the drive"); }
    if (p.sel.open && !p.sel.ring.as_ref().is_some_and(|r| r.free == Some(true))) || p.sel.down_t >= 0.0 { return Some("choosing an action"); }
    None
}

/// May an autosave be made now.   (Checkpoints.gate)
fn gate(g: &mut Game) -> Option<String> {
    if !crate::mission::mission_live(g) { return Some(String::from("not a live run")); }
    if !g.ai_enabled { return Some(String::from("AI off")); }
    if let Some(why) = run_no(g, &crate::seats::here(g), false) { return Some(why); }
    // (a team on the floor: hands and the air are in what is kept)
    if !g.guards.iter().any(|&x| g.gd[x].unit.is_some()) { if let Some(why) = unsettled(g) { return Some(why); } }
    for &x in &g.guards {
        let x = &g.gd[x];
        match x.state { GuardState::Dead => continue, GuardState::Alert | GuardState::Search => return Some(format!("{} is {}", x.callsign, x.state.as_str())), _ => {} }
        if x.awareness > g.k.ckpt.aware_max { return Some(format!("{} at awareness {:.2}", x.callsign, x.awareness)); }
    }
    civilian_compromise(g)
}

/// An autosave is asked for; one already waiting keeps its clock and takes the newer words.   (Checkpoints.request)
pub fn request(g: &mut Game, why: &str, label: &str) {
    if !g.saves.enabled { return; }
    g.saves.stats.requests += 1.0;
    match g.saves.pending.as_mut() {
        Some(p) => { p.label = String::from(label); p.why.push('+'); p.why.push_str(why); }
        None => { g.saves.pending = Some(CheckpointsPending { why: String::from(why), label: String::from(label), at: g.time }); note(g, format!("request: {why} ({label})")); }
    }
}
/// A tutorial's step says: a checkpoint is here.   (Checkpoints.keep)
pub fn keep(g: &mut Game, label: &str) { g.saves.authored = Some(String::from(label)); }
/// Why a save by hand would not be made now.   (Checkpoints.handNo)
pub fn hand_no(g: &Game) -> Option<String> {
    if crate::rails::rails_live(g) { return Some(String::from("a tutorial saves where its steps say")); }
    run_no(g, &crate::seats::here(g), false)
}
/// A save by hand, at the end of this update.   (Checkpoints.askSave)
pub fn ask_save(g: &mut Game) { g.saves.hand = true; }
/// A game kept under this key is to be put back; why not, or nothing.   (Checkpoints.load, of a save in memory)
pub fn ask_load(g: &mut Game, key: &str) -> Option<String> { checkpoints_load_in_step(g, key) }

/// A fresh run: no saves, nothing asked for.
fn fresh(g: &mut Game) {
    let room = here(g);
    let (serial, stage, run) = (g.mission.serial, g.mission.stage, format!("{}", g.mission.serial));
    let s = &mut g.saves;
    s.mission_ref = Some(serial);
    s.stage_ref = stage;
    s.run_id = run;
    s.visited = JsSet::new();
    s.visited.add(room.clone());
    s.room_ref = room;
    s.quiet_t = 0.0;
    s.pending = None;
    s.body_from = None;
    s.drag_ref = None;
    s.last_save_t = -1e9;
    s.last_auto = None;
    s.shelf.m.clear();
    s.hand = false;
    s.due = None;
}

/// After every step: the triggers, then what is waiting against the gates.   (Checkpoints.tick)
pub fn tick(g: &mut Game, dt: f64) -> Option<Do> {
    // (two players: none is made and none goes back, and whoever asked is told)
    if g.seats.len() > 1 {
        let (back, hand) = (g.saves.due.take(), core::mem::take(&mut g.saves.hand));
        return if back.is_some() { run_no(g, &crate::seats::here(g), false).map(|w| Do::Say(format!("checkpoint: {w}"))) } else if hand { write(g, SlotInfoSlot::Manual, "by hand", false) } else { None };
    }
    if g.saves.mission_ref != Some(g.mission.serial) { fresh(g); }
    if let Some(key) = g.saves.due.take() {
        g.saves.hand = false;
        let run = g.saves.shelf.m.get(key.clone()).map(|e| e.info.run_id.clone());
        let why = match run { None => Some(String::from("that checkpoint is gone")), Some(r) => super::b_01::mem_no(g, &r) };
        match why { None => return Some(Do::Back { key }), Some(w) => return Some(Do::Say(format!("checkpoint: {w}"))) }
    }
    if crate::rails::rails_live(g) {
        // a tutorial mission's checkpoints are where its steps say, and nowhere else
        g.saves.pending = None;
        g.saves.hand = false;
        return g.saves.authored.take().and_then(|label| write(g, SlotInfoSlot::Auto, &label, true));
    }
    g.saves.authored = None;
    if crate::cmd::seat_pressed(g, "F5") && !g.pl[g.player].down { ask_save(g); }
    let hand = if core::mem::take(&mut g.saves.hand) { write(g, SlotInfoSlot::Manual, "by hand", false) } else { None };
    let auto = turn(g, dt, hand.is_none());
    hand.or(auto)
}
/// the rest of a turn: what is regretted, what is asked for, and what is waiting (`may` write it)
fn turn(g: &mut Game, dt: f64, may: bool) -> Option<Do> {
    regret(g);
    if !crate::mission::mission_live(g) { g.saves.pending = None; return None; }
    if !g.saves.enabled { g.saves.pending = None; }
    let (k, stage) = (g.k.ckpt, g.mission.stage);
    // the mission moved on
    if stage != g.saves.stage_ref {
        let was = core::mem::replace(&mut g.saves.stage_ref, stage);
        if rank(stage) > rank(was) && stage != MissionStage::Done {
            let w = crate::quiet::quiet_words(g);
            let label = match (stage, w) {
                (MissionStage::Drive, Some(w)) => format!("{} reached", w.place),
                (MissionStage::Drive, None) => String::from("server room reached"),
                (MissionStage::Exfil, Some(w)) => format!("{} taken", w.bare),
                (MissionStage::Exfil, None) => String::from("the drive is out"),
                _ => String::from(stage.as_str()),
            };
            request(g, "stage", &label);
        }
    }
    // a room entered for the first time this run
    let room = here(g);
    if room != g.saves.room_ref {
        g.saves.room_ref = room.clone();
        if !g.saves.visited.has(room.clone()) { g.saves.visited.add(room.clone()); request(g, "room", &room); }
    }
    // a body hidden: hauled some way from where it was taken hold of, and let go
    let drags = g.pl[g.player].dragging;
    let body_at = |g: &Game, gd: GuardId| -> V3 { match g.ch[g.gd[gd].ch].ragdoll { Some(r) => { let x = &g.tab.ragdoll[r].x; [x[0], 0.0, x[2]] } None => g.ch[g.pl[g.player].ch].pos } };
    if let Some(b) = drags { if g.saves.drag_ref != Some(b) { g.saves.drag_ref = Some(b); g.saves.body_from = Some(body_at(g, b)); } }
    else if let Some(b) = g.saves.drag_ref.take() {
        let at = body_at(g, b);
        if let Some(from) = g.saves.body_from.take() { if v3::dist_xz(at, from) >= k.body_hide_m { let l = format!("{}'s body moved", g.gd[b].callsign); request(g, "body", &l); } }
    }
    // quiet play: the clock runs while the gates would pass
    let blocked = gate(g);
    if blocked.is_none() {
        g.saves.quiet_t += dt;
        if g.saves.quiet_t >= k.quiet_secs { g.saves.quiet_t = 0.0; request(g, "quiet", &format!("{} s undetected", k.quiet_secs)); }
    }
    let p = g.saves.pending.clone()?;
    if g.time - p.at > k.pending_secs {
        g.saves.pending = None;
        g.saves.stats.dropped += 1.0;
        note(g, format!("dropped: {} ({})", p.why, blocked.unwrap_or_else(|| String::from("gap"))));
        return None;
    }
    if g.time - p.at < k.debounce_secs || g.time - g.saves.last_save_t < k.min_gap_secs { return None; }
    if blocked.is_some() || !may { g.saves.stats.deferred_frames += 1.0; return None; }
    g.saves.pending = None;
    write(g, SlotInfoSlot::Auto, &p.label, false)
}

/// The floor came down on him straight after the last autosave: it is kept, and offered only when nothing better is.
fn regret(g: &mut Game) {
    let Some(a) = g.saves.last_auto.clone() else { return };
    if g.time - a.t > g.k.ckpt.regret_secs || g.time < a.t { g.saves.last_auto = None; return; }
    let alert = g.guards.iter().copied().find(|&x| g.gd[x].state == GuardState::Alert && crate::teams::hostile_to(g, x, crate::teams::HostileToO::Player));
    let why = match alert { Some(x) => Some(format!("{} alert {:.0} s after", g.gd[x].callsign, g.time - a.t)), None if g.alarm.episode => Some(String::from("an alarm straight after")), None => None };
    let Some(why) = why else { return };
    if let Some(e) = g.saves.shelf.m.get_mut(a.key.clone()) { if e.info.regret.is_none() { e.info.regret = Some(why.clone()); g.saves.stats.regretted += 1.0; } }
    note(g, format!("regretted {}: {}", a.key, why));
    g.saves.last_auto = None;
}

/// A save into its ring's next slot.   (Checkpoints.writeMem)
fn write(g: &mut Game, slot: SlotInfoSlot, label: &str, authored: bool) -> Option<Do> {
    if let Some(why) = run_no(g, &crate::seats::here(g), authored) {
        g.saves.stats.refused += 1.0;
        note(g, format!("refused: {why}"));
        return (slot == SlotInfoSlot::Manual).then(|| Do::Say(format!("not saved: {why}")));
    }
    let (mut n, mut at) = (0, f64::INFINITY);
    for i in 0..ring(g, slot) {
        match g.saves.shelf.m.get(key_of(slot, i)) { None => { n = i; break; } Some(e) => if e.info.saved_at < at { at = e.info.saved_at; n = i; } }
    }
    let key = key_of(slot, n);
    // (which came after which: no clock of the machine's is in a game)
    let saved_at = g.saves.shelf.m.values().map(|e| e.info.saved_at).fold(0.0, crate::m::max) + 1.0;
    let info = SlotInfo { key: key.clone(), slot, label: String::from(label), saved_at, mission_t: g.mission.stats.elapsed, run_id: g.saves.run_id.clone(), regret: None };
    g.saves.shelf.m.set(key.clone(), MemSave { info });
    g.saves.last_save_t = g.time;
    g.saves.quiet_t = 0.0;
    let t = g.time;
    if slot == SlotInfoSlot::Auto { g.saves.stats.autosaves += 1.0; g.saves.last_auto = Some(CheckpointsLastAuto { key: key.clone(), t }); } else { g.saves.stats.manual += 1.0; }
    g.saves.flash = Some(CheckpointsFlash { text: String::from(if slot == SlotInfoSlot::Auto { "CHECKPOINT" } else { "CHECKPOINT SAVED" }), t });
    note(g, format!("{}: {}", key, label));
    Some(Do::Keep { key, label: String::from(label) })
}

/// A MOMENT OF A GAME: the game as a value and its block of rust/sim, between two ticks. Stood again it is the game that went on from there. Every game kept is one of these, for a
/// save, a run's start, a walk of a tape, a test or the gate to go back to; there is one way to put it back.
#[derive(Clone)]
pub struct Moment { pub game: Box<Game>, pub block: Vec<u8> }
impl Moment {
    /// the moment a game stands at
    pub fn of(g: &Game) -> Moment {
        let mut game = Box::new(g.clone());
        game.out.clear();
        let block = unsafe { core::slice::from_raw_parts(::sim::state::addr(g.sim) as *const u8, core::mem::size_of::<::sim::state::State>()) }.to_vec();
        Moment { game, block }
    }
    /// the moment in `slot`, and a game that holds it. The box world's copy in rust/sim stands outside the block and is the abandoned moment's: written whole again by `whole`, which
    /// `over` and `stand` do, or a host may have something to say of the game first
    pub fn laid(&self, slot: usize) -> Box<Game> {
        ::sim::state::put(slot, &self.block);
        let mut g = self.game.clone();
        crate::game::stand_in(&mut g, slot);
        g.col.mirror.put_backs = -1.0;
        g
    }
    /// the game stands at this moment again, in `slot`
    pub fn stands(&self, slot: usize) -> Box<Game> {
        let mut g = self.laid(slot);
        whole(&mut g);
        g
    }
}
/// the box world's copy with it: it stands again, of this moment, before anybody asks
pub fn whole(g: &mut Game) {
    crate::scene_collision::unstand(g);
    crate::scene_collision::stands(g);
}
/// The game `g` is this one now, standing where it stood: the serial counter never goes back in a game (what was dealt after is still held on the host's side, and a number means one thing), and the box world's copy stands again of this moment
pub fn over(g: &mut Game, mut new: Box<Game>) {
    new.serial_next = new.serial_next.max(g.serial_next);
    stand(g, new);
}
/// `new` is the game now, with its box world's copy standing of this moment.
pub fn stand(g: &mut Game, mut new: Box<Game>) {
    whole(&mut new);
    core::mem::swap(g, &mut *new);
}

/// what is kept of a game: the image, which came after which, its label, and whether the floor came down straight after it
#[derive(Clone)]
pub struct Kept { pub image: Rc<Moment>, pub at: f64, pub label: String, pub regret: bool }
/// the key of a run's first step
pub const START: &str = "start";
/// what a restart goes back to: the level before the men, with the lines that stand them on it
pub struct First { image: Moment, men: crate::standing::Men }
/// EVERY GAME THE CONTROL KEEPS OF A STAND: the first, what a stage goes back to, and what was kept as it ran
#[derive(Default)]
struct Held { first: Option<Rc<First>>, stage: Option<Rc<Moment>>, kept: BTreeMap<String, Kept>, used: u64 }
/// … by the stand of the game (`stood`). A game lost to a panic is gone while what is kept of it is not, so they are held beside the games and not in them. Only the last few stands' are held: a game dropped may stand again from a kept copy of itself (a panic's way back, a tool's slot given up), and is the same stand.
static mut IMAGES: BTreeMap<u32, Held> = BTreeMap::new();
static mut STANDS: u32 = 0;
static mut USES: u64 = 0;
const STANDS_HELD: usize = ::sim::state::GAMES + 4;
/// which stand this game is of: a game kept and put back is of its own, one imported is a new one
pub fn stood(g: &mut Game) -> u32 {
    if g.saves.stood == 0 { g.saves.stood = unsafe { STANDS += 1; STANDS }; }
    g.saves.stood
}
fn held(stood: u32) -> &'static mut Held {
    let all = unsafe { &mut *core::ptr::addr_of_mut!(IMAGES) };
    // (a game that runs asks every step, so it is never the one let go)
    while all.len() >= STANDS_HELD && !all.contains_key(&stood) {
        let Some(last) = all.iter().min_by_key(|(_, h)| h.used).map(|(k, _)| *k) else { break };
        all.remove(&last);
    }
    let h = all.entry(stood).or_default();
    h.used = unsafe { USES += 1; USES };
    h
}
fn images(stood: u32) -> &'static mut BTreeMap<String, Kept> { &mut held(stood).kept }
/// THE LEVEL AS IT STANDS BEFORE THE MEN, and the lines that stand them on it: a restart's way
pub fn keep_level(g: &mut Game, men: &crate::standing::Men) {
    let of = stood(g);
    held(of).first = Some(Rc::new(First { image: Moment::of(g), men: men.clone() }));
}
/// THIS GAME, kept as what `again` puts back: a stage's way, which sets itself up on a mission that is up
pub fn keep_first(g: &mut Game) {
    let of = stood(g);
    held(of).stage = Some(Rc::new(Moment::of(g)));
}
/// what is kept of this stand's game, the best to go back to first: the newest save nothing came down after, the newest at all, and the run's first step last
/// What a stand keeps is this again (a moment of the game going back: what was kept after it is gone, what it held comes back).
pub fn kept_again(stood: u32, of: &[(String, Kept)]) {
    *images(stood) = of.iter().cloned().collect();
}
pub fn kept_of(stood: u32) -> Vec<(String, Kept)> {
    let mut of: Vec<(String, Kept)> = images(stood).iter().map(|(k, e)| (k.clone(), e.clone())).collect();
    of.sort_by(|a, b| (a.0 == START).cmp(&(b.0 == START)).then(a.1.regret.cmp(&b.1.regret)).then(b.1.at.total_cmp(&a.1.at)));
    of
}

/// WHAT AN EVENING KEEPS OVER A RESTART, and nothing else goes over: the switches the player and this machine set, the tune, the men's kit, the die, whatever hands out a number, the tallies, the tables as they are now and how far each lens is out. Row numbers go back with the game
pub struct Survivors {
    god: bool, ai: bool, ammo: bool, unseen: bool, clear_rooms: f64, spectate: f64, show_debug: bool, aside: bool,
    tune: crate::game::GameTune, props_on: bool, friction: f64, fog_on: bool, fog_viewer: bool,
    kit: crate::kit::KitMod, difficulty: crate::difficulty::DifficultyMod, pending: Option<crate::quiet::QuietDeal>,
    dice: Option<crate::math_dice::SimDice>, sim_dice: ::sim::dice::Dice, fx_seed: f64, roll_seq: f64, event_seq: f64,
    saves_on: bool, stats: crate::game::Stats, k: crate::knobs::Knobs, lens: Vec<(f64, f64)>,
    out: Vec<crate::out_events::OutEvent>, cmd_buf: Option<crate::cmd::PlayerCmd>,
}
impl Survivors {
    pub fn of(g: &Game) -> Survivors {
        Survivors {
            god: g.god_mode, ai: g.ai_enabled, ammo: g.infinite_ammo, unseen: g.player_invisible, clear_rooms: g.clear_rooms, spectate: g.spectate, show_debug: g.show_debug, aside: g.aside,
            tune: g.tune.clone(), props_on: g.props.enabled, friction: g.props.friction_scale, fog_on: g.fog_of_war.on, fog_viewer: g.fog_picture.viewer,
            kit: g.m.kit.clone(), difficulty: g.m.difficulty.clone(), pending: g.quiet.pending.clone(),
            dice: g.dice.clone(), sim_dice: unsafe { core::ptr::read(&::sim::state::state_read(g.sim).dice) }, fx_seed: g.fx_seed, roll_seq: g.roll_seq, event_seq: g.event_seq,
            saves_on: g.saves.enabled, stats: g.stats.clone(), k: g.k.clone(), lens: g.tab.camera.ids().map(|c| (g.tab.camera[c].distance, g.tab.camera[c].desired_distance)).collect(),
            out: g.out.clone(), cmd_buf: g.cmd_buf.clone(),
        }
    }
    /// what the men's lines read and write on: the tables, the die, the counts and what the host is owed. They go on from here, so this is written once, before them
    fn write_under(&self, g: &mut Game) {
        put_cards(g, self.kit.clone());
        g.m.difficulty = self.difficulty.clone();
        g.quiet.pending = self.pending.clone();
        (g.dice, g.fx_seed, g.roll_seq, g.event_seq) = (self.dice.clone(), self.fx_seed, self.roll_seq, self.event_seq);
        ::sim::state::state(g.sim).dice = unsafe { core::ptr::read(&self.sim_dice) };
        (g.stats, g.k, g.out, g.cmd_buf) = (self.stats.clone(), self.k.clone(), self.out.clone(), self.cmd_buf.clone());
    }
    /// what the men's lines answered to as they did the first time (a switch set since could make a mission refuse), and what a mission's start wrote of its own is taken back: the switches, the lenses
    fn write_over(&self, g: &mut Game) {
        (g.god_mode, g.ai_enabled, g.infinite_ammo, g.player_invisible, g.clear_rooms, g.spectate, g.show_debug, g.aside) = (self.god, self.ai, self.ammo, self.unseen, self.clear_rooms, self.spectate, self.show_debug, self.aside);
        (g.tune, g.props.enabled, g.props.friction_scale, g.fog_of_war.on, g.fog_picture.viewer) = (self.tune.clone(), self.props_on, self.friction, self.fog_on, self.fog_viewer);
        g.saves.enabled = self.saves_on;
        let cams: Vec<_> = g.tab.camera.ids().collect();
        for (c, (d, want)) in cams.into_iter().zip(self.lens.iter()) { g.tab.camera[c].distance = *d; g.tab.camera[c].desired_distance = *want; }
    }
}
/// A step asks for the mission over (Enter once down, Shift+Enter): the step ends first, as a load in a step does
pub fn ask_restart(g: &mut Game, said: Option<&str>) { g.saves.again = Some(String::from(said.unwrap_or(""))); }
/// THE MISSION OVER, from outside a step: the level stands again and the men are stood on it by the stand's own lines, with what an evening keeps; a new run, the view cut to the fresh man. False where the game keeps no level (it has lost its place among the stands held)
pub fn restart(g: &mut Game, said: Option<&str>) -> bool {
    let (slot, of) = (g.sim, stood(g));
    let Some(first) = held(of).first.clone() else { return false };
    let was = Survivors::of(g);
    let host = (g.host_cam, g.host_input);
    // (no box world's copy stands yet of a level with no men: the lines open it, as they did)
    crate::scene_collision::unstand(g);
    let mut new = first.image.laid(slot);
    new.serial_next = new.serial_next.max(g.serial_next);
    core::mem::swap(g, &mut *new);
    was.write_under(g);
    // (a mission that started once and refuses now leaves the plain floor, as a refused start does, and says so)
    if let Some(why) = crate::standing::stand_men(g, &first.men) { crate::game::game_msg(g, &why); }
    crate::scene_collision::light_ready(g);
    was.write_over(g);
    g.mission.serial = g.serial();
    assert!((g.host_cam, g.host_input) == host, "the men's lines made the host's rows elsewhere than the first time");
    if let Some(s) = said { if !s.is_empty() { crate::game::game_msg(g, s); } }
    crate::game::game_cut_to_men(g);
    g.restart_cut = true;
    g.stats.cleanrestart.restarts += 1;
    true
}
/// the cards, the notes, the words on them; what is keyed by a man or a player is of the game thrown away, and new men would answer to it
fn put_cards(g: &mut Game, kit: crate::kit::KitMod) {
    let of_new = core::mem::replace(&mut g.m.kit, kit);
    (g.m.kit.man_of, g.m.kit.bare_eyed, g.m.kit.kit_staged, g.m.kit.issued_player, g.m.kit.you_bare) = (of_new.man_of, of_new.bare_eyed, of_new.kit_staged, of_new.issued_player, of_new.you_bare);
}
/// A STAGE'S WAY OF STARTING OVER: the game `keep_first` kept stands again, die, sky and all, so a picture of the same cards is the same picture, with only the cards, the tables and what the host is owed. False where none was kept
pub fn again(g: &mut Game) -> bool {
    let (slot, of) = (g.sim, stood(g));
    let Some(first) = held(of).stage.clone() else { return false };
    let (kit, k, out, cmd) = (g.m.kit.clone(), g.k.clone(), core::mem::take(&mut g.out), g.cmd_buf.take());
    over(g, first.laid(slot));
    put_cards(g, kit);
    (g.k, g.out, g.cmd_buf) = (k, out, cmd);
    true
}
/// the run's first moment is kept, once, for a game that is lost before anything else is
fn keep_start(g: &mut Game) {
    let (of, run) = (stood(g), g.mission.serial as f64);
    if images(of).get(START).is_some_and(|e| e.at == run) { return; }
    images(of).insert(String::from(START), Kept { image: Rc::new(Moment::of(g)), at: run, label: String::new(), regret: false });
}
/// a step's end: a restart it asked for is made
fn restart_asked(g: &mut Game) -> bool {
    let Some(said) = g.saves.again.take() else { return false };
    restart(g, Some(&said))
}
/// a step's end with the controller off (a tape walked): the run's first step is kept, and a restart it asked for is made
pub fn restart_turn(g: &mut Game) -> bool {
    keep_start(g);
    restart_asked(g)
}

/// what a step's end told its host
#[derive(Default)]
pub struct Told {
    /// a save was made under this key
    pub kept: Option<String>,
    /// a game was put back
    pub back: bool,
    /// the picture of the game before this is not to be blended into it
    pub cut: bool,
    /// tell the player this
    pub said: Option<String>,
}
/// what of a step's out events asks something of the control
pub fn asked(out: &[crate::out_events::OutEvent]) -> Vec<crate::out_events::OutEvent> {
    use crate::out_events::OutEvent as E;
    out.iter().filter(|e| matches!(e, E::CheckpointsKeep(_) | E::CheckpointsRequest(_))).cloned().collect()
}
/// AFTER A STEP, on every host: what the step asked, the control's own turn, the game kept or put back, and the run's first step kept for a game that is lost before anything else is
pub fn after_step(g: &mut Box<Game>, dt: f64, asked: Vec<crate::out_events::OutEvent>) -> Told {
    use crate::out_events::OutEvent as E;
    for e in asked { match e { E::CheckpointsKeep(e) => keep(g, &e.label), E::CheckpointsRequest(e) => request(g, &e.why, &e.label), _ => {} } }
    let back = restart_asked(g);
    let mut told = Told { back, cut: core::mem::replace(&mut g.restart_cut, false), ..Told::default() };
    let (slot, of) = (g.sim, stood(g));
    match tick(g, dt) {
        Some(Do::Keep { key, label }) => {
            checkpoint(g, &label);
            let at = g.saves.shelf.m.values().map(|e| e.info.saved_at).fold(0.0, crate::m::max);
            images(of).insert(key.clone(), Kept { image: Rc::new(Moment::of(g)), at, label, regret: false });
            told.kept = Some(key);
        }
        Some(Do::Back { key }) => {
            let was = abandoned(g);
            told.said = Some(match images(of).get(&key).map(|e| e.image.laid(slot)) {
                Some(new) => { over(g, new); told.back = true; told.cut = true; put_back(g, &key, was) }
                None => gone(g, &key),
            });
        }
        Some(Do::Say(words)) => told.said = Some(words),
        None => {}
    }
    // (a run's saves go with the run; its first step is kept, for a game that is lost before anything else is, after the turn that makes the run's saves its own)
    keep_start(g);
    images(of).retain(|key, _| key == START || g.saves.shelf.m.has(key.clone()));
    for (key, e) in images(of).iter_mut() { if !e.regret { if let Some(m) = g.saves.shelf.m.get(key.clone()) { e.regret = m.info.regret.is_some(); } } }
    told
}
/// A save is taken: the chronicle takes its row
pub fn checkpoint(g: &mut Game, label: &str) {
    let _ = crate::chronicle::chron_checkpoint(g, crate::chronicle::ChronCheckpointWhat::Save, Some(label));
}

/// What the abandoned game hands to the one put back: what is the host's and the run's as it is NOW, and not the saved moment's to take back.
pub struct Abandoned { saves: Checkpoints, loads: f64, kept: f64, god: bool, ai: bool, ammo: bool, unseen: bool, labels: bool, tune: crate::game::GameTune }
pub fn abandoned(g: &Game) -> Abandoned {
    let c = &g.chronicle.counts;
    Abandoned { saves: g.saves.clone(), loads: c.loads, kept: c.saves, god: g.god_mode, ai: g.ai_enabled, ammo: g.infinite_ammo, unseen: g.player_invisible, labels: g.show_debug, tune: g.tune.clone() }
}
/// There is no game kept under this key any more: it is off the shelf. What the player is told of it.
pub fn gone(g: &mut Game, key: &str) -> String {
    g.saves.shelf.m.delete(String::from(key));
    String::from("checkpoint: that checkpoint is gone")
}
/// The game kept under `key` stands again: the lines that follow a load, and what the player is told of it.   (Checkpoints.loadMem, after the snapshot is restored)
pub fn put_back(g: &mut Game, key: &str, was: Abandoned) -> String {
    // (a ring that merely stood at the save, and a gun's setup that was up, are nothing of the saved moment)
    let pl = g.player;
    if g.pl[pl].sel.ring.as_ref().is_some_and(|r| r.free == Some(true)) {
        let s = &mut g.pl[pl].sel;
        s.open = false;
        s.ring = None;
        s.down_t = -1.0;
        s.moved = false;
        g.pl[pl].near_sel = 0;
    }
    g.pl[pl].setup = None;
    crate::orders::squad_orders_loaded(g);
    let label = was.saves.shelf.m.get(String::from(key)).map(|e| e.info.label.clone()).unwrap_or_default();
    g.saves.shelf = was.saves.shelf;
    g.saves.stats = was.saves.stats;
    g.saves.log = was.saves.log;
    g.saves.enabled = was.saves.enabled;
    g.saves.pending = None;
    g.saves.last_auto = None;
    g.saves.hand = false;
    g.saves.due = None;
    g.saves.authored = None;
    g.saves.last_save_t = g.time;
    (g.god_mode, g.ai_enabled, g.infinite_ammo, g.player_invisible, g.show_debug, g.tune) = (was.god, was.ai, was.ammo, was.unseen, was.labels, was.tune);
    g.chronicle.counts.loads = was.loads;
    g.chronicle.counts.saves = was.kept;
    let _ = crate::chronicle::chron_checkpoint(g, crate::chronicle::ChronCheckpointWhat::Load, Some(&label));
    g.saves.stats.loads += 1.0;
    let t = g.time;
    g.saves.flash = Some(CheckpointsFlash { text: String::from("CHECKPOINT LOADED"), t });
    note(g, format!("loaded {key}"));
    if label.is_empty() { String::from("checkpoint loaded") } else { format!("checkpoint loaded \u{2014} {label}") }
}
/// This run's saves, oldest first on the mission's clock: what a card lays out.   (Checkpoints.timeline)
pub fn timeline(g: &Game) -> Vec<SlotInfo> {
    let mut all: Vec<SlotInfo> = g.saves.shelf.m.values().filter(|e| e.info.run_id == g.saves.run_id).map(|e| e.info.clone()).collect();
    all.sort_by(|a, b| a.mission_t.total_cmp(&b.mission_t).then(a.saved_at.total_cmp(&b.saved_at)));
    all
}
