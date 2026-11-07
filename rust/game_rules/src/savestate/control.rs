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
use alloc::{format, string::String, vec::Vec};

/// what the host is to do now
pub enum Do {
    /// keep the game as it stands, under this key; `rows` the run's chronicle first (game_run `checkpoint`)
    Keep { key: String, label: String },
    /// put back the game kept under this key
    Back { key: String },
}

fn key_of(slot: SlotInfoSlot, i: usize) -> String { format!("{}.{}", if slot == SlotInfoSlot::Auto { "auto" } else { "manual" }, i) }
fn ring(g: &Game, slot: SlotInfoSlot) -> usize { (if slot == SlotInfoSlot::Auto { g.k.savetimeline.auto } else { g.k.savetimeline.manual }) as usize }
fn note(g: &mut Game, what: String) {
    let t = g.time;
    g.saves.log.push(CheckpointsLogItem { t, what });
    if g.saves.log.len() > 40 { g.saves.log.remove(0); }
}
fn here(g: &mut Game) -> String { let p = g.ch[g.pl[g.player].ch].pos; crate::chronicle::room_of(g, p) }
fn rank(s: MissionStage) -> u32 { match s { MissionStage::Infiltrate => 0, MissionStage::Drive => 1, MissionStage::Exfil => 2, MissionStage::Done | MissionStage::Failed => 3 } }

/// Why the game cannot be kept right now; `authored` = a tutorial's own checkpoint, taken whatever is on the floor.   (savestate.ts runNo, for a save in memory)
pub fn run_no(g: &Game, seat: &crate::seats::SeatRef, authored: bool) -> Option<String> {
    if g.touring || g.quiet_utility || g.puppet.is_some() { return Some(String::from("a show is running")); }
    if !authored { if let Some(r) = g.director.run { let r = &g.tab.director_run[r]; if !r.ended { return Some(format!("the level script's program '{}' is running", r.name)); } } }
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
    if crate::lighting::is_daylight(g) { return Some(String::from("daylight is in")); }
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
    if crate::defuse::defuse_working(g) { return Some("at a charge"); }
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
    if g.seats.len() > 1 { return None; }
    if g.saves.mission_ref != Some(g.mission.serial) { fresh(g); }
    if let Some(key) = g.saves.due.take() {
        g.saves.hand = false;
        let run = g.saves.shelf.m.get(key.clone()).map(|e| e.info.run_id.clone());
        let why = match run { None => Some(String::from("that checkpoint is gone")), Some(r) => super::b_01::mem_no(g, &r) };
        match why { None => return Some(Do::Back { key }), Some(w) => { crate::game::game_msg(g, &format!("checkpoint: {w}")); return None; } }
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
        if slot == SlotInfoSlot::Manual { crate::game::game_msg(g, &format!("not saved: {why}")); }
        return None;
    }
    let (mut n, mut at) = (0, f64::INFINITY);
    for i in 0..ring(g, slot) {
        match g.saves.shelf.m.get(key_of(slot, i)) { None => { n = i; break; } Some(e) => if e.info.saved_at < at { at = e.info.saved_at; n = i; } }
    }
    let key = key_of(slot, n);
    // (which came after which: no clock of the machine's is in a game)
    let saved_at = g.saves.shelf.m.values().map(|e| e.info.saved_at).fold(0.0, crate::m::max) + 1.0;
    let kind = if slot == SlotInfoSlot::Auto { SlotInfoKind::Auto } else { SlotInfoKind::Manual };
    let info = SlotInfo { key: key.clone(), slot, n: n as f64, label: String::from(label), kind, saved_at, mission_t: g.mission.stats.elapsed, stage: String::from(g.mission.stage.as_str()), run_id: g.saves.run_id.clone(), bytes: 0.0, regret: None, mem: Some(true) };
    g.saves.shelf.m.set(key.clone(), MemSave { info, pic: None, take: 0.0 });
    g.saves.last_save_t = g.time;
    g.saves.quiet_t = 0.0;
    let t = g.time;
    if slot == SlotInfoSlot::Auto { g.saves.stats.autosaves += 1.0; g.saves.last_auto = Some(CheckpointsLastAuto { key: key.clone(), t }); } else { g.saves.stats.manual += 1.0; }
    g.saves.flash = Some(CheckpointsFlash { text: String::from(if slot == SlotInfoSlot::Auto { "CHECKPOINT" } else { "CHECKPOINT SAVED" }), t });
    note(g, format!("{}: {}", key, label));
    Some(Do::Keep { key, label: String::from(label) })
}

/// What the abandoned game hands to the one put back: what is the host's and the run's as it is NOW, and not the saved moment's to take back.
pub struct Abandoned { saves: Checkpoints, loads: f64, kept: f64, god: bool, ai: bool, ammo: bool, unseen: bool, tune: crate::game::GameTune }
pub fn abandoned(g: &Game) -> Abandoned {
    let c = &g.chronicle.counts;
    Abandoned { saves: g.saves.clone(), loads: c.loads, kept: c.saves, god: g.god_mode, ai: g.ai_enabled, ammo: g.infinite_ammo, unseen: g.player_invisible, tune: g.tune.clone() }
}
/// There is no game kept under this key any more: it is off the shelf, and he is told.
pub fn gone(g: &mut Game, key: &str) {
    g.saves.shelf.m.delete(String::from(key));
    crate::game::game_msg(g, "checkpoint: that checkpoint is gone");
}
/// The game kept under `key` stands again: the lines that follow a load.   (Checkpoints.loadMem, after the snapshot is restored)
pub fn put_back(g: &mut Game, key: &str, was: Abandoned) {
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
    (g.god_mode, g.ai_enabled, g.infinite_ammo, g.player_invisible, g.tune) = (was.god, was.ai, was.ammo, was.unseen, was.tune);
    g.chronicle.counts.loads = was.loads;
    g.chronicle.counts.saves = was.kept;
    let _ = crate::chronicle::chron_checkpoint(g, crate::chronicle::ChronCheckpointWhat::Load, Some(&label));
    crate::game::game_msg(g, &if label.is_empty() { String::from("checkpoint loaded") } else { format!("checkpoint loaded \u{2014} {label}") });
    g.saves.stats.loads += 1.0;
    let t = g.time;
    g.saves.flash = Some(CheckpointsFlash { text: String::from("CHECKPOINT LOADED"), t });
    note(g, format!("loaded {key}"));
    crate::rails::rails_put_back(g);
}
/// This run's saves, oldest first on the mission's clock: what a card lays out.   (Checkpoints.timeline)
pub fn timeline(g: &Game) -> Vec<SlotInfo> {
    let mut all: Vec<SlotInfo> = g.saves.shelf.m.values().filter(|e| e.info.run_id == g.saves.run_id).map(|e| e.info.clone()).collect();
    all.sort_by(|a, b| a.mission_t.total_cmp(&b.mission_t).then(a.saved_at.total_cmp(&b.saved_at)));
    all
}
