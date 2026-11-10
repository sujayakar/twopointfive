#![allow(unused_imports, unused_variables)]
//! A PAR: a room clear run AGAINST A CLOCK (the Door Kickers star rating). A level file carries a `scenarios.par` block (gold / silver / bronze
//! seconds and `loss_secs`); while that level's room clear is up, the tick times the run and, once, names the medal.
//!
//!   THE CLOCK STARTS on the first thing anybody DOES: a squad's first order (a row on a seat's orders log), a player's first stride off the mark the
//!   staging stood him on (`PAR_START_M`), or the first round or throw. Reading the house from the start line is free, doing anything is not.
//!   IT STOPS when the room clear ends 'taken', at the room clear's own end time. A run that ends 'wiped' has no time.
//!   EVERY TEAMMATE LOST (down at the end: dead or out cold) adds `loss_secs`.
//!   THE MEDAL IS JUDGED ON WHAT IS SHOWN: the total in whole seconds, as the clock words it (65.4 s reads 1:05 and is inside a 1:05 gold).
//!
//! The record (`Game.par`) is the game's: two machines of one game read the same clock, the same result and the same line. It moves nobody and
//! ends nothing: on a level without the block it returns on the first line, and on one with it the game differs from the same run untimed only in
//! this record and in the line it says at the end (and in the serials given after that line). There is no switch.

pub use ::game_types::par::types;
pub use types::*;
use crate::game::Game;
use crate::m;
use alloc::{format, string::String, vec::Vec};

use crate::scene_scenarios::ParData;

const MEDALS: [Medal; 3] = [Medal::Gold, Medal::Silver, Medal::Bronze];
fn bar(d: &ParData, medal: Medal) -> f64 { match medal { Medal::Gold => d.gold, Medal::Silver => d.silver, Medal::Bronze => d.bronze, Medal::None => f64::INFINITY } }

/// m:ss
pub fn clock(s: f64) -> String { let n = m::max(0.0, m::floor(s)) as u64; format!("{}:{:02}", n / 60, n % 60) }

/// the medal a total earns against a par: judged in whole seconds, as the clock shows it
pub fn medal_of(d: &ParData, total: f64) -> Medal { let t = m::floor(total); MEDALS.iter().copied().find(|&md| t <= bar(d, md)).unwrap_or(Medal::None) }

/// a run's result against a par: `secs` on the clock, `lost` teammates of `of`
pub fn par_result(d: &ParData, secs: f64, lost: f64, of: f64) -> ParResult {
    let penalty = lost * d.loss_secs;
    let total = secs + penalty;
    let shown = m::floor(total);
    let medal = medal_of(d, total);
    // (the medal above the one earned, and how many seconds faster it needed: none above gold)
    let up = match medal { Medal::None => Some(Medal::Bronze), Medal::Bronze => Some(Medal::Silver), Medal::Silver => Some(Medal::Gold), Medal::Gold => None };
    let next = up.map(|u| ParResultNext { medal: u, bar: bar(d, u), short: shown - bar(d, u) });
    let mut line = format!("par ▸ {}", clock(secs));
    if lost != 0.0 { line.push_str(&format!(" +{} ({} lost) = {}", clock(penalty), lost, clock(total))); }
    line.push_str(" · ");
    if medal == Medal::None { line.push_str("no medal"); } else { line.push_str(&medal.as_str().to_uppercase()); }
    if let Some(n) = &next { line.push_str(&format!(" · {} was {}", n.medal.as_str(), clock(n.bar))); }
    ParResult { secs, lost, penalty, total, shown, medal, next, line }
}

/// the level's par while it is LIVE: the block there, its room clear up, no show running (None = stood down)
pub fn par_data(g: &Game) -> Option<ParData> {
    let d = g.level.scenarios.as_ref().and_then(|s| s.par)?;
    if crate::roomclear::room_clear_scenario_active(g) { Some(d) } else { None }
}

/// has the clock started, for THIS mission
pub fn par_started(g: &Game) -> bool { g.par.run == Some(g.mission.serial) && g.par.t0.is_finite() }

/// seconds on the clock now (0 before it starts; held at the end's own time once the room clear has ended)
fn par_secs(g: &Game) -> f64 {
    if !par_started(g) { return 0.0; }
    let t = if g.room_clear.end.is_some() { g.room_clear.end_at } else { g.time };
    m::max(0.0, t - g.par.t0)
}

/// teammates down now, of how many
fn lost(g: &Game) -> (f64, f64) {
    let ops = &g.staged.team;
    (ops.iter().filter(|&&o| crate::condition::down(g, g.gd[o].ch)).count() as f64, ops.len() as f64)
}

/// rounds and throws so far this mission (the mission's own tally and the chronicle's count: both fresh with the mission)
fn fired(g: &Game) -> f64 { g.mission.stats.shots + g.chronicle.counts.thrown }

/// every seat's mark as the clock is armed, in seat order: where its man stands and its log's last row. The first seat's is the staging's own
/// mark (where the room clear stood him), so a man the staging is still walking to his mark has not "moved"
fn marks_now(g: &mut Game) -> Vec<ParMark> {
    let start = crate::roomclear::room_clear_scenario_start_at(g);
    let mut out: Vec<ParMark> = Vec::new();
    crate::seats::each_seat(g, &mut |g| {
        let p = g.ch[g.pl[g.player].ch].pos;
        out.push(ParMark { origin: p, order0: g.orders.log.last().map(|r| r.serial) });
    });
    if let (Some(s), Some(first)) = (start, out.first_mut()) { first.origin = s; }
    out
}

/// what the seats have done since their marks: (one has strode off his, one has given an order)
fn done_since(g: &mut Game) -> (bool, bool) {
    let (mut moved, mut ordered, mut i) = (false, false, 0usize);
    crate::seats::each_seat(g, &mut |g| {
        if let Some(mk) = g.par.marks.get(i).copied() {
            let p = g.ch[g.pl[g.player].ch].pos;
            moved |= m::hyp2(p[0] - mk.origin[0], p[2] - mk.origin[2]) > PAR_START_M;
            ordered |= g.orders.log.last().map(|r| r.serial) != mk.order0;
        }
        i += 1;
    });
    (moved, ordered)
}

/// ONCE A TICK, right after the mission's update: this tick's orders and the room clear's end are in.
pub fn par_tick(g: &mut Game) {
    let Some(d) = par_data(g) else { g.par.live = false; return; };
    g.par.live = true;
    if g.par.run != Some(g.mission.serial) {
        // this mission's marks: what "the first thing done" is measured from
        let marks = marks_now(g);
        let f0 = fired(g);
        let p = &mut g.par;
        p.run = Some(g.mission.serial); p.res = None; p.marks = marks; p.fired0 = f0; p.t0 = f64::NAN;
    }
    if !g.par.t0.is_finite() {
        let (moved, ordered) = done_since(g);
        let shot = fired(g) != g.par.fired0;
        let ended = g.room_clear.end.is_some();
        if !(moved || shot || ordered || ended) { return; }
        g.par.t0 = if ended { m::min(g.time, g.room_clear.end_at) } else { g.time };
        g.stats.par.started += 1;
    }
    if g.par.res.is_some() || g.room_clear.end != Some(crate::roomclear::RoomClearEnd::Taken) { return; }
    let (lost, of) = lost(g);
    let r = par_result(&d, par_secs(g), lost, of);
    g.stats.par.finished += 1;
    g.par.res = Some(r);
}

/// the finished run (None: not taken yet, wiped, no par here, or another mission's)
pub fn par_result_now(g: &Game) -> Option<&ParResult> {
    if par_data(g).is_some() && g.par.run == Some(g.mission.serial) { g.par.res.as_ref() } else { None }
}

/// what leads the room clear's row while the run is on (empty: no par live, or the run is over)
fn par_row(g: &Game) -> String {
    let Some(d) = par_data(g) else { return String::new(); };
    if g.room_clear.end.is_some() { return String::new(); }   // (taken: par_objective words it · wiped: no time)
    if !par_started(g) { return format!("par {} / {} / {} · ", clock(d.gold), clock(d.silver), clock(d.bronze)); }
    let (lost, _) = lost(g);
    let secs = par_secs(g);
    let total = secs + lost * d.loss_secs;
    let up = MEDALS.iter().copied().find(|&md| m::floor(total) <= bar(&d, md));
    let mut s = format!("⏱ {}", clock(secs));
    if lost != 0.0 { s.push_str(&format!(" +{}", clock(lost * d.loss_secs))); }
    s.push_str(" → ");
    match up { Some(u) => s.push_str(&format!("{} {}", u.as_str(), clock(bar(&d, u)))), None => s.push_str("past bronze") }
    s.push_str(" · ");
    s
}

/// the room clear's objective row with the clock on it: the clock FIRST, then the room clear's own words; at the end the result
pub(super) fn par_objective(g: &Game, text: &str) -> String {
    match par_result_now(g) {
        None => { let mut s = par_row(g); s.push_str(text); s }
        Some(r) => format!("structure taken · {} — press shift+enter to restart", r.line.strip_prefix("par ▸ ").unwrap_or(&r.line)),
    }
}
