//! src/game/needs.ts lines 1 to 453
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use crate::chronicle::{ChronKind, ChronicleDataValue};
use crate::game::GuardState;
use crate::teams::Team;

/// The row of memOf at his key, already made. (a path, not a held reference)
macro_rules! row {
    ($g:ident, $gd:ident) => {
        $g.m.needs.mem_of.get_mut($gd).unwrap()
    };
}
/// The trip under way in his row, already known to be there.
macro_rules! trip {
    ($g:ident, $gd:ident) => {
        $g.m.needs.mem_of.get_mut($gd).unwrap().trip.as_mut().unwrap()
    };
}

/// NEED_KINDS, in the order the deal ranks them from (needs.ts:34; the enum's own order differs).
const NEED_KINDS: [NeedKind; 4] = [NeedKind::Coffee, NeedKind::Smoke, NeedKind::Water, NeedKind::Stretch];

/// Seam: `g.audio.play(name, at, gain, { rate })` (needs.ts:270).
fn seam_audio_play(g: &mut Game, name: &str, at: V3, gain: f64, rate: f64) {
    todo!("needs: game_audio_like_play's opts hold crate::Opaque, which cannot carry a rate")
}

/// `g.level.scenarios?.quiet` is there
fn has_quiet(g: &Game) -> bool {
    g.level.scenarios.as_ref().map_or(false, |s| s.quiet.is_some())
}
/// `gd.route.points[wp]`
fn route_point(g: &Game, gd: GuardId, wp: f64) -> V3 {
    g.tab.route[g.gd[gd].route].points[wp as usize]
}
/// `gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.6, 0])`
fn head_or_guess(g: &Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(crate::character::Bone::Head) {
        Some(h) => *h,
        None => v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]),
    }
}
/// `gd.stallRef = null`
fn clear_stall(g: &mut Game, gd: GuardId) {
    g.gd[gd].stall_ref = None;
}
fn text(s: &str) -> Option<ChronicleDataValue> {
    Some(ChronicleDataValue::Text(String::from(s)))
}
fn key(s: &str) -> String {
    String::from(s)
}

/// Where a need is answered on this level, or None. (needs.ts needSpot)
pub fn need_spot(g: &mut crate::game::Game, kind: crate::needs::NeedKind) -> Option<crate::needs::NeedSpot> {
    let Some(q) = g.level.scenarios.as_ref().and_then(|s| s.quiet.as_ref()) else { return need_spots().get(key(kind.as_str())).cloned() };
    let s = q.needs.as_ref().and_then(|n| match kind {
        NeedKind::Coffee => n.coffee.as_ref(),
        NeedKind::Smoke => n.smoke.as_ref(),
        NeedKind::Water => n.water.as_ref(),
        NeedKind::Stretch => n.stretch.as_ref(),
    });
    let s = s.map(|s| OwnSpotS { name: s.name.clone(), pos: s.pos, face: s.face, loop_: s.loop_.clone() })?;
    Some(own_spot(g, &s, kind))
}

/// A level's own place as a NeedSpot. (needs.ts ownSpot; `owned`'s cache is dropped, the value is a function of `s`)
pub fn own_spot(g: &mut crate::game::Game, s: &crate::needs::OwnSpotS, kind: crate::needs::NeedKind) -> crate::needs::NeedSpot {
    NeedSpot { kind, name: s.name.clone(), pos: s.pos, face: s.face, loop_: s.loop_.clone() }
}

/// A new man's row: never back from a break, so his rest is long had. (needs.ts:111)
fn blank_mem() -> NeedMem {
    NeedMem { needs: Vec::new(), trip: None, last_back_at: -1e9, trips: 0.0, cancels: 0.0, shaken: false, dealt: false }
}

/// His row, made if new. (needs.ts mem)
pub fn mem<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::needs::NeedMem {
    g.m.needs.mem_of.get_or_insert_with(gd, blank_mem)
}

/// Read-only peek for the panel and the probes. (needs.ts needsMem)
pub fn needs_mem(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::needs::NeedsMem> {
    let r = g.m.needs.mem_of.get(gd)?;
    Some(NeedsMem { needs: r.needs.clone(), trip: r.trip.clone(), last_back_at: r.last_back_at, trips: r.trips, cancels: r.cancels, shaken: r.shaken, dealt: r.dealt })
}

/// The trip this man is on, or None. (needs.ts needsTrip)
pub fn needs_trip(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::needs::NeedsTrip> {
    let t = g.m.needs.mem_of.get(gd)?.trip.as_ref()?;
    Some(NeedsTrip {
        kind: t.kind,
        spot: t.spot.clone(),
        phase: t.phase,
        forced: t.forced,
        t0: t.t0,
        depart_at: t.depart_at,
        left_at: t.left_at,
        arrived_at: t.arrived_at,
        dwell_until: t.dwell_until,
        back_at: t.back_at,
        wp: t.wp,
        out_yaw: t.out_yaw,
        watched: t.watched,
        told: t.told,
        partner: t.partner,
        radio: t.radio,
        reply_at: t.reply_at,
        replied: t.replied,
        necked: t.necked,
        loop_i: t.loop_i,
        line: t.line.clone(),
        gap_before: t.gap_before,
    })
}

/// Is he telegraphing a need or off his route on one? (needs.ts needsBusy)
pub fn needs_busy(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.m.needs.mem_of.get(gd).map_or(false, |r| r.trip.is_some())
}

/// When anybody last walked off; made if new. (needs.ts floor)
pub fn floor<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::needs::NeedsModFloorOfValue {
    g.m.needs.floor_of.get_or_insert_with(|| NeedsModFloorOfValue { last_depart_at: -1e9 })
}

/// The last few things that happened. (needs.ts log; NEEDS_DEBUG's sink is host work)
pub fn log(g: &mut crate::game::Game, gd: GuardId, what: &str) {
    let t = g.time;
    let who = g.gd[gd].callsign.clone();
    g.m.needs.needs_log.push(NeedsModNEEDSLOGItem { t, who, what: String::from(what) });
    if g.m.needs.needs_log.len() > 200 {
        g.m.needs.needs_log.drain(0..50);
    }
}

/// (needs.ts frac)
pub fn frac(s: &str) -> f64 {
    js::rem(crate::math_dice::hash_str(s), 100000.0) / 100000.0
}

/// Metres by path; infinite if there is none. (needs.ts pathMetres)
pub fn path_metres(g: &mut crate::game::Game, from: V3, to: V3) -> f64 {
    let Some(p) = crate::scene_collision::nav_grid_find_path(g, from, to, Some(6000.0), None) else { return m::INF };
    path_len(from, &p)
}

/// The waypoint of his route a trip to `spot` starts from; wp −1 = none. (needs.ts departurePoint)
pub fn departure_point(g: &mut crate::game::Game, gd: GuardId, spot: &crate::needs::NeedSpot) -> crate::needs::DeparturePoint {
    let r = g.tab.route[g.gd[gd].route].clone();
    let mut best = -1.0;
    let mut bm = m::INF;
    for i in 0..r.points.len() {
        if r.wait.get(i).copied().unwrap_or(0.0) < g.k.needs.min_wait {
            continue;
        }
        let d = path_metres(g, r.points[i], spot.pos);
        if d < bm {
            bm = d;
            best = i as f64;
        }
    }
    DeparturePoint { wp: best, m: bm }
}

/// Deal this man his needs, once, off his callsign. (needs.ts deal)
pub fn deal(g: &mut crate::game::Game, gd: GuardId) {
    mem(g, gd).dealt = true;
    if g.gd[gd].team != Team::Guard || g.gd[gd].unit.is_some() {
        return;
    }
    let cs = g.gd[gd].callsign.clone();
    let mut ranked: Vec<(NeedKind, f64)> = NEED_KINDS.iter().map(|&k| (k, crate::math_dice::hash_str(&format!("{}:{}:rank", cs, k.as_str())))).collect();
    js::sort_by(&mut ranked, |a, b| a.1 - b.1);
    let want = 2.0 + js::rem(crate::math_dice::hash_str(&format!("{}:count", cs)), 2.0);
    for &(k, _) in ranked.iter() {
        if mem(g, gd).needs.len() as f64 >= want {
            break;
        }
        let Some(sp) = need_spot(g, k) else { continue };
        let dp = departure_point(g, gd, &sp);
        if dp.wp < 0.0 || !dp.m.is_finite() || dp.m < g.k.needs.min_trip_m {
            continue;
        }
        let ks = k.as_str();
        let [lo, hi] = *fill_min().get(key(ks)).unwrap();
        let l0 = level0();
        mem(g, gd).needs.push(Need {
            kind: k,
            level: l0[0] + frac(&format!("{}:{}:level0", cs, ks)) * (l0[1] - l0[0]),
            fill_min: lo + frac(&format!("{}:{}:fill", cs, ks)) * (hi - lo),
            wp: dp.wp,
            trip_m: dp.m,
        });
    }
    g.stats.needs.dealt += 1;
    let list: Vec<String> = mem(g, gd)
        .needs
        .iter()
        .map(|n| format!("{} (fills in {:.1} min, at {:.2}, from wp{} {:.0} m)", n.kind.as_str(), n.fill_min, n.level, n.wp, n.trip_m))
        .collect();
    let list = if list.is_empty() { String::from("nothing") } else { list.join(", ") };
    log(g, gd, &format!("dealt {}", list));
}

/// May the needs play in this run at all? (needs.ts needsLive)
pub fn needs_live(g: &mut crate::game::Game) -> bool {
    g.tune.needs
        && g.ai_enabled
        && !g.quiet_utility
        && !g.touring
        && g.puppet.is_none()
        && !crate::lighting::is_daylight(g)
        && g.qrf.state == crate::qrf::QrfState::Idle
        && !crate::seats::all_down(g)
}

/// Would a man leave his route on this floor? (needs.ts floorCalm)
pub fn floor_calm(g: &mut crate::game::Game) -> bool {
    crate::guards::escalation_of(g) == 0.0 && !g.alarm.episode && !crate::sim_lightshome::mains_active(g)
}

/// Is he calm and free? (needs.ts manCalm)
pub fn man_calm(g: &mut crate::game::Game, gd: GuardId, awareness: f64) -> bool {
    let x = &g.gd[gd];
    x.state == GuardState::Patrol
        && x.awareness < awareness
        && x.held.is_none()
        && x.script.is_none()
        && x.task.is_none()
        && x.body_duty.is_none()
        && !x.hold
        && !x.pinned
        && x.leader.is_none()
        && x.post.is_none()
        && g.guards.contains(&gd)
}

/// Why a trip under way must be broken off now, or None. (needs.ts interruptOf)
pub fn interrupt_of(g: &mut crate::game::Game, gd: GuardId) -> Option<String> {
    if !g.tune.needs {
        return Some(key("switched off"));
    }
    if g.gd[gd].state == GuardState::Dead {
        return Some(key("down"));
    }
    if !g.ai_enabled {
        return Some(key("AI off"));
    }
    if g.quiet_utility || g.touring || g.puppet.is_some() {
        return Some(key("the tour"));
    }
    if crate::lighting::is_daylight(g) {
        return Some(key("daylight"));
    }
    if g.qrf.state != crate::qrf::QrfState::Idle {
        return Some(key("the response team"));
    }
    if g.pl[g.player].down {
        return Some(key("target down"));
    }
    if crate::guards::escalation_of(g) > 0.0 {
        return Some(key("the floor went up"));
    }
    if g.alarm.episode {
        return Some(key("an alarm"));
    }
    if crate::sim_lightshome::mains_active(g) {
        return Some(key("the lights went"));
    }
    let x = &g.gd[gd];
    if x.held.is_some() {
        return Some(key("grabbed"));
    }
    if x.body_duty.is_some() {
        return Some(key("a body"));
    }
    if let Some(task) = x.task.as_ref() {
        let kind = match task {
            crate::game::GuardTask::Arm0(a) => a.kind.as_str(),
            crate::game::GuardTask::CheckOn(_) => "checkOn",
        };
        return Some(format!("an errand ({})", kind));
    }
    if x.script.is_some() {
        return Some(key("orders"));
    }
    if x.hold || x.pinned {
        return Some(key("held by a script"));
    }
    if x.leader.is_some() || x.post.is_some() {
        return Some(key("lockdown duty"));
    }
    if x.state != GuardState::Patrol {
        return Some(if x.state == GuardState::Alert { key("contact") } else { format!("went {}", x.state.as_str()) });
    }
    if x.awareness >= CALM_AWARENESS {
        return Some(key("heard something"));
    }
    if !g.guards.contains(&gd) {
        return Some(key("reset"));
    }
    None
}

/// Once a frame: levels, the after-alert cap, interrupts, the deal of a trip. (needs.ts needsTick)
pub fn needs_tick(g: &mut crate::game::Game, dt: f64) {
    if !g.tune.needs {
        let mut i = 0;
        while i < g.guards.len() {
            let gd = g.guards[i];
            i += 1;
            if needs_busy(g, gd) {
                cancel_trip(g, gd, "switched off");
            }
        }
        return;
    }
    let live = needs_live(g);
    let calm_floor = floor_calm(g);
    if !g.guards.iter().any(|&x| g.m.needs.mem_of.has(x)) {
        floor(g).last_depart_at = -1e9;
    }
    let mut away = 0;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if needs_busy(g, gd) {
            match interrupt_of(g, gd) {
                Some(why) => cancel_trip(g, gd, &why),
                None => away += 1,
            }
        }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state == GuardState::Dead || g.gd[gd].team != Team::Guard || g.gd[gd].unit.is_some() {
            continue;
        }
        if !mem(g, gd).dealt {
            deal(g, gd);
        }
        if row!(g, gd).needs.is_empty() {
            continue;
        }
        let state = g.gd[gd].state;
        if state == GuardState::Alert || state == GuardState::Search || g.alarm.episode || crate::guards::escalation_of(g) > 0.0 {
            row!(g, gd).shaken = true;
        } else if row!(g, gd).shaken && state == GuardState::Patrol {
            let after = g.k.needs.after_alert;
            let r = row!(g, gd);
            r.shaken = false;
            for n in r.needs.iter_mut() {
                n.level = m::min(n.level, after);
            }
            log(g, gd, &format!("calm again — needs capped at {}", after));
        }
        if !live || !calm_floor || row!(g, gd).trip.is_some() || !man_calm(g, gd, CALM_AWARENESS) {
            continue;
        }
        let rate = g.k.needs.rate_k;
        for n in row!(g, gd).needs.iter_mut() {
            n.level = m::min(LEVEL_CAP, n.level + dt / 60.0 / n.fill_min * rate);
        }
        let now = g.time;
        if away as f64 >= g.k.needs.max_away || now - floor(g).last_depart_at < g.k.needs.floor_gap_secs || now - row!(g, gd).last_back_at < g.k.needs.rest_secs {
            continue;
        }
        if !(g.gd[gd].wait > 0.0) || g.gd[gd].awareness >= SCHEDULE_AWARENESS || g.gd[gd].bubble.as_ref().map_or(false, |b| now - b.t < b.dur + 0.5) {
            continue;
        }
        let wp = g.gd[gd].wp as f64;
        let mut pick: Option<Need> = None;
        for n in row!(g, gd).needs.iter() {
            if n.level >= 1.0 && n.wp == wp && pick.map_or(true, |p| n.level > p.level) {
                pick = Some(*n);
            }
        }
        let Some(pick) = pick else { continue };
        if schedule_trip(g, gd, pick.kind, false) {
            away += 1;
        }
    }
}

/// Put this man on a trip, the telegraph starting now. False if the place cannot be reached. (needs.ts scheduleTrip)
pub fn schedule_trip(g: &mut crate::game::Game, gd: GuardId, kind: crate::needs::NeedKind, forced: bool) -> bool {
    let spot = need_spot(g, kind);
    let c = g.gd[gd].ch;
    let Some(spot) = spot else {
        if let Some(n) = mem(g, gd).needs.iter_mut().find(|x| x.kind == kind) {
            n.level = 0.0;
        }
        log(g, gd, &format!("{}: this floor has no place for it — dropped", kind.as_str()));
        return false;
    };
    let pos = g.ch[c].pos;
    let path = match crate::scene_collision::nav_grid_find_path(g, pos, spot.pos, Some(6000.0), None) {
        Some(p) if !p.is_empty() => p,
        _ => {
            if let Some(n) = mem(g, gd).needs.iter_mut().find(|x| x.kind == kind) {
                n.level = 0.0;
            }
            log(g, gd, &format!("{}: no way to {} from here — dropped", kind.as_str(), spot.name));
            return false;
        }
    };
    let mut q = path[0];
    for &p in path.iter() {
        q = p;
        if m::hyp2(p[0] - pos[0], p[2] - pos[2]) > 2.5 {
            break;
        }
    }
    let out_yaw = m::atan2(q[0] - pos[0], q[2] - pos[2]);
    let now = g.time;
    let last = floor(g).last_depart_at;
    let lead = g.k.needs.announce_lead;
    let wp = g.gd[gd].wp;
    let name = spot.name.clone();
    mem(g, gd).trip = Some(Trip {
        kind,
        spot,
        phase: TripPhase::Telegraph,
        forced,
        t0: now,
        depart_at: now + lead,
        left_at: -1.0,
        arrived_at: -1.0,
        dwell_until: -1.0,
        back_at: -1.0,
        wp: wp as f64,
        out_yaw,
        watched: false,
        told: false,
        partner: None,
        radio: false,
        reply_at: -1.0,
        replied: false,
        necked: false,
        loop_i: -1.0,
        line: String::new(),
        gap_before: last,
    });
    floor(g).last_depart_at = m::max(last, now + lead);
    g.stats.needs.scheduled += 1;
    if forced {
        g.stats.needs.forced += 1;
    }
    log(
        g,
        gd,
        &format!("{} {} → {} from wp{} ({:.0} m); walks at +{} s", if forced { "SENT" } else { "schedules" }, kind.as_str(), name, wp, path_len(pos, &path), lead),
    );
    true
}

/// Length of a path, walked from `from`. (needs.ts pathLen)
pub fn path_len(from: V3, p: &[V3]) -> f64 {
    let mut s = 0.0;
    let mut prev = from;
    for q in p.iter() {
        s += m::hyp2(q[0] - prev[0], q[2] - prev[2]);
        prev = *q;
    }
    s
}

/// Break the trip off wherever he is; nothing is resumed. (needs.ts cancelTrip)
pub fn cancel_trip(g: &mut crate::game::Game, gd: GuardId, why: &str) {
    let Some(t) = mem(g, gd).trip.take() else { return };
    mem(g, gd).cancels += 1.0;
    g.stats.needs.cancelled += 1;
    let after = g.k.needs.after_cancel;
    if let Some(n) = mem(g, gd).needs.iter_mut().find(|x| x.kind == t.kind) {
        n.level = m::min(n.level, after);
    }
    clear_stall(g, gd);
    let c = g.gd[gd].ch;
    crate::character::anim_cancel_tell(g, c);
    if t.phase == TripPhase::Telegraph {
        let f = floor(g);
        if f.last_depart_at == t.depart_at {
            f.last_depart_at = t.gap_before;
        }
    } else {
        let mut d = JsMap::new();
        d.set(key("who"), text(&g.gd[gd].callsign));
        d.set(key("kind"), text(t.kind.as_str()));
        d.set(key("what"), text("cancelled"));
        d.set(key("why"), text(why));
        d.set(key("phase"), text(t.phase.as_str()));
        let at = g.ch[c].pos;
        crate::chronicle::chronicle(g, ChronKind::NeedBreak, Some(&d), Some(at));
    }
    log(g, gd, &format!("{} CANCELLED in {} — {}", t.kind.as_str(), t.phase.as_str(), why));
}

/// The line for a slot, off his name and his count. (needs.ts pickLine)
pub fn pick_line(g: &mut crate::game::Game, pool: &[String], gd: GuardId, salt: &str) -> String {
    let trips = mem(g, gd).trips;
    let cancels = mem(g, gd).cancels;
    let h = crate::math_dice::hash_str(&format!("{}|{}|{}|{}", g.gd[gd].callsign, trips, cancels, salt));
    pool[js::rem(h, pool.len() as f64) as usize].clone()
}

/// The colleague he tells: the nearest staff man his words reach. (needs.ts partnerFor)
pub fn partner_for(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let mouth = head_or_guess(g, gd);
    let my_pos = g.ch[g.gd[gd].ch].pos;
    let mut best = None;
    let mut bd = m::INF;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let x = &g.gd[o];
        if o == gd || x.state == GuardState::Dead || x.held.is_some() || x.team != Team::Guard || x.unit.is_some() {
            continue;
        }
        if v3::dist(g.ch[x.ch].pos, my_pos) > g.k.needs.partner_m + 2.0 {
            continue;
        }
        let ear = head_or_guess(g, o);
        let pr = crate::weather::hear_path(g, mouth, ear, None);
        if pr.path_len < g.k.needs.partner_m * pr.carry && pr.path_len < bd {
            bd = pr.path_len;
            best = Some(o);
        }
    }
    best
}

/// Say it: to the partner's face, else over the net. (needs.ts announce)
pub fn announce(g: &mut crate::game::Game, gd: GuardId, t: &mut crate::needs::Trip) {
    t.told = true;
    g.stats.needs.told += 1;
    let p = partner_for(g, gd);
    let quiet = has_quiet(g);
    let lines = crate::dialogue::needs_lines_table();
    let kind = t.kind.as_str();
    if let Some(p) = p {
        t.partner = Some(p);
        t.reply_at = g.time + g.k.needs.reply_after;
        let own = if quiet { lines.announce_own.get(key(kind)) } else { None };
        let was: Vec<String> = own.or_else(|| lines.announce.get(key(kind))).or_else(|| lines.announce.get(key("coffee"))).unwrap().clone();
        let now: Vec<String> = was.iter().map(|x| lines.now.get(x.clone()).cloned().unwrap_or_else(|| x.clone())).collect();
        let pool = crate::words::wg::<Vec<String>>(g, &now, &was);
        t.line = pick_line(g, &pool, gd, "announce");
        crate::game::game_say(g, gd, &t.line, None, None, None);
        let d = v3::dist_xz(g.ch[g.gd[p].ch].pos, g.ch[g.gd[gd].ch].pos);
        let pcs = g.gd[p].callsign.clone();
        log(g, gd, &format!("tells {} ({:.1} m): \"{}\"", pcs, d, t.line));
    } else {
        t.radio = true;
        g.stats.needs.radioed += 1;
        let own = if quiet { lines.short_own.get(key(kind)) } else { None };
        let short: String = own.or_else(|| lines.short.get(key(kind))).cloned().unwrap_or_else(|| key(kind));
        t.line = lines.radio.replacen("{who}", &g.gd[gd].callsign, 1).replacen("{short}", &short, 1);
        crate::game::game_say(g, gd, &t.line, Some(true), None, None);
        let c = g.gd[gd].ch;
        let at = match g.ch[c].bones.get(crate::character::Bone::Head) {
            Some(h) => *h,
            None => g.ch[c].pos,
        };
        seam_audio_play(g, "radio", at, 0.4, 1.05);
        log(g, gd, &format!("radios it: \"{}\"", t.line));
    }
}

/// The partner's grumble, if he is still one to give it. (needs.ts partnerReply)
pub fn partner_reply(g: &mut crate::game::Game, gd: GuardId, t: &mut crate::needs::Trip) {
    t.replied = true;
    let Some(p) = t.partner else { return };
    let now = g.time;
    let (state, held, aw) = (g.gd[p].state, g.gd[p].held.is_some(), g.gd[p].awareness);
    let mid_line = g.gd[p].bubble.as_ref().map_or(false, |b| now - b.t < b.dur);
    let pcs = g.gd[p].callsign.clone();
    if state == GuardState::Dead
        || held
        || (state != GuardState::Patrol && state != GuardState::Suspicious)
        || aw >= 0.45
        || !g.guards.contains(&p)
        || mid_line
        || crate::doze::doze_asleep(g, p)
    {
        log(g, gd, &format!("{} says nothing back ({}, aw {:.2})", pcs, state.as_str(), aw));
        return;
    }
    let third = g.guards.iter().copied().find(|&x| x != gd && x != p && g.gd[x].team == Team::Guard && g.gd[x].unit.is_none() && g.gd[x].state != GuardState::Dead);
    let trips = mem(g, gd).trips;
    let lines = crate::dialogue::needs_lines_table();
    let pool: Vec<String> = match third {
        Some(x) if js::rem(crate::math_dice::hash_str(&format!("{}|{}|other", pcs, trips)), 3.0) == 0.0 => {
            let other = g.gd[x].callsign.clone();
            lines.reply_other.iter().map(|s| s.replace("{other}", &other)).collect()
        }
        _ => {
            let now_pool: Vec<String> = if t.kind == NeedKind::Stretch { lines.reply.iter().filter(|s| s.as_str() != "Bring me one.").cloned().collect() } else { lines.reply.clone() };
            crate::words::w(g, &now_pool, &lines.reply)
        }
    };
    let cs = g.gd[gd].callsign.clone();
    let h = crate::math_dice::hash_str(&format!("{}|{}|{}|reply", pcs, cs, trips));
    let line = pool[js::rem(h, pool.len() as f64) as usize].clone();
    crate::game::game_say(g, p, &line, None, None, None);
    g.stats.needs.replies += 1;
    log(g, gd, &format!("{} answers: \"{}\"", pcs, line));
}

/// `stand()` of needsPatrol: his feet stop. (needs.ts:292)
fn stand(g: &mut Game, gd: GuardId, dt: f64) {
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
    let c = g.gd[gd].ch;
    g.ch[c].vel = [0.0, 0.0, 0.0];
}

/// If a trip owns his feet this frame, run them and return true; false = the branch walks his route. (needs.ts needsPatrol)
pub fn needs_patrol(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool {
    if !needs_busy(g, gd) {
        return false;
    }
    let c = g.gd[gd].ch;
    let now = g.time;
    let pace = walk * g.k.needs.pace_k;
    let look_phase = g.gd[gd].look_phase;
    let kind = trip!(g, gd).kind;
    let cs = g.gd[gd].callsign.clone();
    match trip!(g, gd).phase {
        TripPhase::Telegraph => {
            stand(g, gd, dt);
            g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(look_phase * 0.9) * 0.45, 2.0, dt);
            let (depart_at, t0, out_yaw) = {
                let t = trip!(g, gd);
                (t.depart_at, t.t0, t.out_yaw)
            };
            let to_go = depart_at - now;
            if !trip!(g, gd).watched {
                trip!(g, gd).watched = true;
                crate::character::anim_tell(g, c, crate::character::TellKind::Watch);
            }
            let g0 = now - t0 - GLANCE_AT;
            if g0 > 0.0 && g0 < GLANCE_SECS {
                let turn = v3::clamp(v3::wrap_angle(out_yaw - g.ch[c].body_yaw), -1.1, 1.1);
                g.ch[c].anim.look_yaw_extra = turn * m::min_n(&[1.0, g0 / 0.35, (GLANCE_SECS - g0) / 0.45]);
            } else {
                g.ch[c].anim.look_yaw_extra = m::sin(look_phase * 1.3) * 0.3;
            }
            if !trip!(g, gd).told && to_go <= g.k.needs.say_lead {
                let mut t = trip!(g, gd).clone();
                announce(g, gd, &mut t);
                row!(g, gd).trip = Some(t);
            }
            let (told, replied, reply_at) = {
                let t = trip!(g, gd);
                (t.told, t.replied, t.reply_at)
            };
            if told && !replied && reply_at >= 0.0 && now >= reply_at {
                let mut t = trip!(g, gd).clone();
                partner_reply(g, gd, &mut t);
                row!(g, gd).trip = Some(t);
            }
            if !trip!(g, gd).necked && to_go <= g.k.needs.stretch_lead {
                trip!(g, gd).necked = true;
                crate::character::anim_tell(g, c, crate::character::TellKind::Neck);
            }
            if trip!(g, gd).necked {
                g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, out_yaw, 2.2 * dt);
            }
            if to_go <= 0.0 {
                {
                    let t = trip!(g, gd);
                    t.phase = TripPhase::Going;
                    t.left_at = now;
                    t.loop_i = -1.0;
                }
                let f = floor(g);
                f.last_depart_at = m::max(f.last_depart_at, now);
                g.stats.needs.departed += 1;
                clear_stall(g, gd);
                let spot_pos = trip!(g, gd).spot.pos;
                crate::guards::go_to(g, gd, spot_pos, Some(true));
                let mut d = JsMap::new();
                d.set(key("who"), text(&cs));
                d.set(key("kind"), text(kind.as_str()));
                d.set(key("what"), text("left"));
                d.set(key("to"), text(&trip!(g, gd).spot.name));
                if has_quiet(g) {
                    d.set(key("own"), Some(ChronicleDataValue::Num(1.0)));
                }
                let at = g.ch[c].pos;
                crate::chronicle::chronicle(g, ChronKind::NeedBreak, Some(&d), Some(at));
                log(g, gd, &format!("walks ({})", kind.as_str()));
            }
            true
        }
        TripPhase::Going => {
            let loop_i = trip!(g, gd).loop_i;
            let goal = if loop_i >= 0.0 { trip!(g, gd).spot.loop_.as_ref().unwrap()[loop_i as usize] } else { trip!(g, gd).spot.pos };
            crate::guards::go_to(g, gd, goal, None);
            let arrived = crate::guards::follow_path(g, gd, dt, pace);
            g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(look_phase * 0.7) * 0.25, 2.0, dt);
            g.ch[c].anim.look_yaw_extra = 0.0;
            if arrived {
                clear_stall(g, gd);
                if v3::dist_xz(g.ch[c].pos, goal) > 1.2 {
                    g.stats.needs.stuck += 1;
                    cancel_trip(g, gd, "no way through");
                    return true;
                }
                let name = trip!(g, gd).spot.name.clone();
                let left_at = trip!(g, gd).left_at;
                let loop_len = trip!(g, gd).spot.loop_.as_ref().map(|l| l.len());
                if let Some(n) = loop_len {
                    if loop_i + 1.0 < n as f64 {
                        if loop_i < 0.0 {
                            trip!(g, gd).arrived_at = now;
                            g.stats.needs.arrived += 1;
                            log(g, gd, &format!("at {} after {:.1} s — round the loop", name, now - left_at));
                        }
                        let next = {
                            let t = trip!(g, gd);
                            t.loop_i += 1.0;
                            t.spot.loop_.as_ref().unwrap()[t.loop_i as usize]
                        };
                        crate::guards::go_to(g, gd, next, Some(true));
                        return true;
                    }
                    let (wp, arrived_at) = {
                        let t = trip!(g, gd);
                        t.phase = TripPhase::Returning;
                        t.dwell_until = now;
                        (t.wp, t.arrived_at)
                    };
                    let back = route_point(g, gd, wp);
                    crate::guards::go_to(g, gd, back, Some(true));
                    log(g, gd, &format!("loop walked in {:.1} s — heading back", now - arrived_at));
                    return true;
                }
                let dwell = g.k.needs.dwell.get(kind).copied().unwrap_or(20.0);
                {
                    let t = trip!(g, gd);
                    t.phase = TripPhase::Dwell;
                    t.arrived_at = now;
                    t.dwell_until = now + dwell;
                }
                g.stats.needs.arrived += 1;
                let until = trip!(g, gd).dwell_until;
                log(g, gd, &format!("at {} after {:.1} s — {:.0} s here", name, now - left_at, until - now));
            } else if crate::guards::not_getting_anywhere(g, gd, dt, g.k.needs.stuck_secs) {
                g.stats.needs.stuck += 1;
                cancel_trip(g, gd, "stuck on the way");
                return true;
            }
            true
        }
        TripPhase::Dwell => {
            stand(g, gd, dt);
            let face = trip!(g, gd).spot.face;
            g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, face, 2.5 * dt);
            g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(look_phase * 0.5) * 0.3, 2.0, dt);
            g.ch[c].anim.look_yaw_extra = m::sin(look_phase * 0.8) * 0.35;
            if now >= trip!(g, gd).dwell_until {
                trip!(g, gd).phase = TripPhase::Returning;
                clear_stall(g, gd);
                let wp = trip!(g, gd).wp;
                let back = route_point(g, gd, wp);
                crate::guards::go_to(g, gd, back, Some(true));
                log(g, gd, &format!("done — heading back to wp{}", wp));
            }
            true
        }
        TripPhase::Returning => {
            let wp = trip!(g, gd).wp;
            let goal = route_point(g, gd, wp);
            crate::guards::go_to(g, gd, goal, None);
            let arrived = crate::guards::follow_path(g, gd, dt, pace);
            g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(look_phase * 0.7) * 0.25, 2.0, dt);
            g.ch[c].anim.look_yaw_extra = 0.0;
            if arrived {
                clear_stall(g, gd);
                if v3::dist_xz(g.ch[c].pos, goal) > 1.5 {
                    g.stats.needs.stuck += 1;
                    cancel_trip(g, gd, "no way back");
                    return true;
                }
                let (left_at, arrived_at, dwell_until) = {
                    let t = trip!(g, gd);
                    t.back_at = now;
                    (t.left_at, t.arrived_at, t.dwell_until)
                };
                {
                    let r = row!(g, gd);
                    r.trip = None;
                    r.trips += 1.0;
                    r.last_back_at = now;
                    if let Some(n) = r.needs.iter_mut().find(|x| x.kind == kind) {
                        n.level = 0.0;
                    }
                }
                g.stats.needs.completed += 1;
                let away = m::round(now - left_at);
                let back = crate::dialogue::needs_lines_table().back;
                let line = pick_line(g, &back, gd, "back");
                crate::game::game_say(g, gd, &line, None, None, None);
                let mut d = JsMap::new();
                d.set(key("who"), text(&cs));
                d.set(key("kind"), text(kind.as_str()));
                d.set(key("what"), text("back"));
                d.set(key("away"), Some(ChronicleDataValue::Num(away)));
                let at = g.ch[c].pos;
                crate::chronicle::chronicle(g, ChronKind::NeedBreak, Some(&d), Some(at));
                log(
                    g,
                    gd,
                    &format!(
                        "back on wp{} — away {} s ({}: out {:.0} s, there {:.0} s, back {:.0} s)",
                        wp,
                        away,
                        kind.as_str(),
                        arrived_at - left_at,
                        dwell_until - arrived_at,
                        now - dwell_until
                    ),
                );
                return false;
            } else if crate::guards::not_getting_anywhere(g, gd, dt, g.k.needs.stuck_secs) {
                g.stats.needs.stuck += 1;
                cancel_trip(g, gd, "stuck coming back");
                return true;
            }
            true
        }
    }
}

/// The panel's button and the probes: send a man now; a note says why not. (needs.ts needsSend)
pub fn needs_send(g: &mut crate::game::Game, gd: Option<GuardId>, kind: Option<crate::needs::NeedKind>) -> String {
    let kind = kind.unwrap_or(NeedKind::Coffee);
    let Some(gd) = gd else { return key("no such man") };
    if !g.tune.needs {
        return key("needs are switched off");
    }
    let cs = g.gd[gd].callsign.clone();
    if g.gd[gd].state == GuardState::Dead {
        return format!("{} is down", cs);
    }
    if g.gd[gd].team != Team::Guard || g.gd[gd].unit.is_some() {
        return format!("{} is not on the night staff", cs);
    }
    if !needs_live(g) {
        return key("not now (the tour / daylight / the response team / the AI off / the player down)");
    }
    if !floor_calm(g) {
        return key("not now — the floor is up (or in the dark)");
    }
    let why = if man_calm(g, gd, CALM_AWARENESS) { None } else { Some(interrupt_of(g, gd).unwrap_or_else(|| key("busy"))) };
    if let Some(why) = why {
        return format!("{} would not go now: {}", cs, why);
    }
    if !mem(g, gd).dealt {
        deal(g, gd);
    }
    if let Some(t) = mem(g, gd).trip.as_ref() {
        return format!("{} is already {} ({})", cs, if t.phase == TripPhase::Telegraph { "about to go" } else { "away" }, t.kind.as_str());
    }
    if !schedule_trip(g, gd, kind, true) {
        let place = match need_spot(g, kind) {
            Some(s) => s.name,
            None => format!("a place for {} (this floor has none)", kind.as_str()),
        };
        return format!("{} cannot get to {} from where he stands", cs, place);
    }
    format!("{}: {} — watch him (walks in {} s)", cs, kind.as_str(), g.k.needs.announce_lead)
}

/// The probes: put a man's need at a level; false if he does not carry that kind. (needs.ts needsSetLevel)
pub fn needs_set_level(g: &mut crate::game::Game, gd: GuardId, kind: crate::needs::NeedKind, level: f64) -> bool {
    if !mem(g, gd).dealt {
        deal(g, gd);
    }
    match mem(g, gd).needs.iter_mut().find(|x| x.kind == kind) {
        Some(n) => {
            n.level = level;
            true
        }
        None => false,
    }
}

/// Keep the switch and the fairness knobs per machine. This crate has no browser storage: the TypeScript's "no storage, nothing to keep". (needs.ts saveNeeds)
pub fn save_needs(g: &mut crate::game::Game, tune: crate::needs::SaveNeedsTune) {}

/// Read back what `save_needs` kept. Nothing is stored here: the TypeScript's "nothing stored, the shipped values stand". (needs.ts loadNeeds)
pub fn load_needs(g: &mut crate::game::Game, tune: &mut crate::needs::LoadNeedsTune) {}

/// The switch back on and the knobs back to the shipped values; no stored copy to drop. (needs.ts resetNeeds)
pub fn reset_needs(g: &mut crate::game::Game, tune: &mut crate::needs::ResetNeedsTune) {
    // `tune` is game.tune in the TypeScript, and this type is a copy of it: both are written.
    tune.needs = true;
    g.tune.needs = true;
    let d = needs_defaults();
    g.k.needs.rate_k = d.rate_k;
    g.k.needs.max_away = d.max_away;
    g.k.needs.announce_lead = d.announce_lead;
    g.k.needs.dwell.set(NeedKind::Coffee, d.dwell.coffee);
    g.k.needs.dwell.set(NeedKind::Smoke, d.dwell.smoke);
    g.k.needs.dwell.set(NeedKind::Water, d.dwell.water);
    g.k.needs.dwell.set(NeedKind::Stretch, d.dwell.stretch);
}

/// The panel's line: per man his needs and levels, or the trip he is on. (needs.ts needsSummary)
pub fn needs_summary(g: &mut crate::game::Game) -> String {
    if !g.tune.needs {
        return key("needs ▸ off");
    }
    let mut parts: Vec<String> = Vec::new();
    let mut away = 0;
    let (now, rest_secs) = (g.time, g.k.needs.rest_secs);
    for &gd in g.guards.iter() {
        let Some(r) = g.m.needs.mem_of.get(gd) else { continue };
        if g.gd[gd].state == GuardState::Dead || r.needs.is_empty() {
            continue;
        }
        let cs = &g.gd[gd].callsign;
        if let Some(t) = r.trip.as_ref() {
            away += 1;
            let s = match t.phase {
                TripPhase::Telegraph => {
                    let told = if !t.told {
                        String::new()
                    } else if t.radio {
                        key(" (radioed)")
                    } else {
                        format!(" (told {})", t.partner.map_or("?", |p| g.gd[p].callsign.as_str()))
                    };
                    format!("about to go — {} in {:.0} s{}", t.kind.as_str(), m::max(0.0, t.depart_at - now), told)
                }
                TripPhase::Going => format!("walking to {}", t.spot.name),
                TripPhase::Dwell => format!("at {}, {:.0} s left", t.spot.name, m::max(0.0, t.dwell_until - now)),
                TripPhase::Returning => format!("coming back to wp{}", t.wp),
            };
            parts.push(format!("{} AWAY {}{}", cs, s, if t.forced { " [sent]" } else { "" }));
        } else {
            let mut full: Vec<Need> = r.needs.iter().copied().filter(|n| n.level >= 1.0).collect();
            js::sort_by(&mut full, |a, b| b.level - a.level);
            let levels: Vec<String> = r.needs.iter().map(|n| format!("{} {:.2}", n.kind.as_str(), n.level)).collect();
            let waits = full.first().map_or(String::new(), |n| format!(" — waits for wp{}", n.wp));
            let rests = if now - r.last_back_at < rest_secs { format!(" (rests {} s)", m::ceil(rest_secs - (now - r.last_back_at))) } else { String::new() };
            parts.push(format!("{} {}{}{}", cs, levels.join(" "), waits, rests));
        }
    }
    let gap = g.k.needs.floor_gap_secs - (now - floor(g).last_depart_at);
    let frozen = if needs_live(g) && floor_calm(g) { "" } else { "FROZEN · " };
    let men = if parts.is_empty() { key("nobody dealt yet") } else { parts.join(" | ") };
    let gap_note = if gap > 0.0 { format!(" · floor gap {} s", m::ceil(gap)) } else { String::new() };
    format!("needs ▸ {}{} · slot {}/{}{}", frozen, men, away, g.k.needs.max_away, gap_note)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `pathLen` against the TypeScript's own bits (EXACT on).
    #[test]
    fn path_len_bits() {
        let from = [1.25, 0.0, -3.5];
        let path = [[4.75, 0.0, 2.125], [10.1, 1.0, 9.7], [-3.3, 0.0, 0.4]];
        assert_eq!(path_len(from, &path).to_bits(), BITS);
    }
    const BITS: u64 = 0x40401adb95a66fb4;
}
