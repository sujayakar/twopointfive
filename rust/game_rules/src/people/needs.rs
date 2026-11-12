pub use ::game_types::needs::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec::Vec};

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

/// `memOf.get(gd)?.trip` is there: he has a trip under way. Makes no row.
fn on_trip(g: &Game, gd: GuardId) -> bool {
    g.m.needs.mem_of.get(gd).map_or(false, |r| r.trip.is_some())
}
/// `gd.route.points[wp]`
fn route_point(g: &Game, gd: GuardId, wp: f64) -> V3 {
    g.tab.route[g.gd[gd].route].points[wp as usize]
}
/// `gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.6, 0])`
fn head_or_guess(g: &Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    g.ch[c].bone_or(crate::character::Bone::Head, 1.6)
}
/// `gd.stallRef = null`
fn clear_stall(g: &mut Game, gd: GuardId) {
    g.gd[gd].stall_ref = None;
}
fn text(s: &str) -> Option<ChronicleDataValue> {
    Some(ChronicleDataValue::Text(String::from(s)))
}

/// Where a need is answered on this level, or None.
fn need_spot(g: &crate::game::Game, kind: crate::needs::NeedKind) -> Option<crate::needs::NeedSpot> {
    if !g.level.own_places() { return need_spots().of(kind.as_str()).cloned(); }
    let s = g.level.needs().and_then(|n| match kind {
        NeedKind::Coffee => n.coffee.as_ref(),
        NeedKind::Smoke => n.smoke.as_ref(),
        NeedKind::Water => n.water.as_ref(),
        NeedKind::Stretch => n.stretch.as_ref(),
    });
    let s = s.map(|s| OwnSpotS { name: s.name.clone(), pos: s.pos, face: s.face, loop_: s.loop_.clone() })?;
    Some(own_spot(&s))
}

/// A level's own place as a NeedSpot.
fn own_spot(s: &crate::needs::OwnSpotS) -> crate::needs::NeedSpot {
    NeedSpot { name: s.name.clone(), pos: s.pos, face: s.face, loop_: s.loop_.clone() }
}

/// A new man's row: never back from a break, so his rest is long had.
fn blank_mem() -> NeedMem {
    NeedMem { needs: Vec::new(), trip: None, last_back_at: -1e9, trips: 0.0, cancels: 0.0, shaken: false, dealt: false }
}

/// His row, made if new.
pub fn mem<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::needs::NeedMem {
    g.m.needs.mem_of.get_or_insert_with(gd, blank_mem)
}

/// When anybody last walked off; made if new.
fn floor<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::needs::NeedsModFloorOfValue {
    g.m.needs.floor_of.get_or_insert_with(|| NeedsModFloorOfValue { last_depart_at: -1e9 })
}

/// A string's hash as a fraction, 0 to 1.
fn frac(s: &str) -> f64 {
    js::rem(crate::dice::hash_str(s), 100000.0) / 100000.0
}

/// Metres by path; infinite if there is none.
fn path_metres(g: &crate::game::Game, from: V3, to: V3) -> f64 {
    let Some(p) = crate::scene_collision::nav_grid_find_path(g, from, to, Some(6000.0), None) else { return m::INF };
    crate::places::walk_len(from, &p)
}

/// The waypoint of his route a trip to `spot` starts from; wp −1 = none.
fn departure_point(g: &crate::game::Game, gd: GuardId, spot: &crate::needs::NeedSpot) -> crate::needs::DeparturePoint {
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

/// Deal this man his needs, once, off his callsign.
fn deal(g: &mut crate::game::Game, gd: GuardId) {
    mem(g, gd).dealt = true;
    if g.team(gd) != Team::Guard || g.gd[gd].unit.is_some() || g.gd[gd].route_i < 0 {
        return;
    }
    let cs = g.gd[gd].callsign.clone();
    let mut ranked: Vec<(NeedKind, f64)> = need_kinds().iter().map(|&k| (k, crate::dice::hash_str(&format!("{}:{}:rank", cs, k.as_str())))).collect();
    js::sort_by(&mut ranked, |a, b| a.1 - b.1);
    let want = 2.0 + js::rem(crate::dice::hash_str(&format!("{}:count", cs)), 2.0);
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
        let [lo, hi] = *fill_min().of(ks).unwrap();
        let l0 = level0();
        mem(g, gd).needs.push(Need {
            kind: k,
            level: l0[0] + frac(&format!("{}:{}:level0", cs, ks)) * (l0[1] - l0[0]),
            fill_min: lo + frac(&format!("{}:{}:fill", cs, ks)) * (hi - lo),
            wp: dp.wp,
        });
    }
}

/// May the needs play in this run at all?
fn needs_live(g: &crate::game::Game) -> bool {
    g.k.tune.needs
        && g.ai_enabled
        && !g.aside
        && g.qrf.state == crate::qrf::QrfState::Idle
        && !crate::seats::all_down(g)
}

/// Would a man leave his route on this floor?
fn floor_calm(g: &crate::game::Game) -> bool {
    crate::guards::escalation_of(g) == 0.0 && !g.alarm.episode && !g.light_home.mains.active
}

/// Is he calm and free?
fn man_calm(g: &crate::game::Game, gd: GuardId, awareness: f64) -> bool {
    let x = &g.gd[gd];
    x.state == GuardState::Patrol
        && x.awareness < awareness
        && x.held.is_none()
        && x.script.is_none()
        && x.errand.is_none()
        && x.body_duty.is_none()
        && !x.duty.holds
        && !x.duty.pinned
        && x.leader.is_none()
        && x.post.is_none()
        && g.guards.contains(&gd)
}

/// Why a trip under way must be broken off now, or None.
fn interrupt_of(g: &crate::game::Game, gd: GuardId) -> Option<String> {
    if !g.k.tune.needs {
        return Some(String::from("switched off"));
    }
    if crate::condition::down(g, g.gd[gd].ch) {
        return Some(String::from("down"));
    }
    if !g.ai_enabled {
        return Some(String::from("AI off"));
    }
    if g.aside {
        return Some(String::from("nobody plays"));
    }
    if g.qrf.state != crate::qrf::QrfState::Idle {
        return Some(String::from("the response team"));
    }
    if crate::seats::all_down(g) {
        return Some(String::from("target down"));
    }
    if crate::guards::escalation_of(g) > 0.0 {
        return Some(String::from("the floor went up"));
    }
    if g.alarm.episode {
        return Some(String::from("an alarm"));
    }
    if g.light_home.mains.active {
        return Some(String::from("the lights went"));
    }
    let x = &g.gd[gd];
    if x.held.is_some() {
        return Some(String::from("grabbed"));
    }
    if x.body_duty.is_some() {
        return Some(String::from("a body"));
    }
    if let Some(task) = x.errand.as_ref() {
        let kind = match task {
            crate::game::Errand::At(a) => a.kind.as_str(),
            crate::game::Errand::CheckOn(_) => "checkOn",
        };
        return Some(format!("an errand ({})", kind));
    }
    if x.script.is_some() {
        return Some(String::from("orders"));
    }
    if !x.duty.free() {
        return Some(String::from("held by a script"));
    }
    if x.leader.is_some() || x.post.is_some() {
        return Some(String::from("lockdown duty"));
    }
    if x.state != GuardState::Patrol {
        return Some(if x.state == GuardState::Alert { String::from("contact") } else { format!("went {}", x.state.as_str()) });
    }
    if x.awareness >= CALM_AWARENESS {
        return Some(String::from("heard something"));
    }
    if !g.guards.contains(&gd) {
        return Some(String::from("reset"));
    }
    None
}

/// Once a frame: levels, the after-alert cap, interrupts, the deal of a trip.
pub fn needs_tick(g: &mut crate::game::Game, dt: f64) {
    if !g.k.tune.needs {
        let mut i = 0;
        while i < g.guards.len() {
            let gd = g.guards[i];
            i += 1;
            if on_trip(g, gd) {
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
        if on_trip(g, gd) {
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
        if crate::condition::down(g, g.gd[gd].ch) || g.team(gd) != Team::Guard || g.gd[gd].unit.is_some() {
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
        if schedule_trip(g, gd, pick.kind) {
            away += 1;
        }
    }
}

/// Put this man on a trip, the telegraph starting now. False if the place cannot be reached.
fn schedule_trip(g: &mut crate::game::Game, gd: GuardId, kind: crate::needs::NeedKind) -> bool {
    let spot = need_spot(g, kind);
    let c = g.gd[gd].ch;
    let Some(spot) = spot else {
        if let Some(n) = mem(g, gd).needs.iter_mut().find(|x| x.kind == kind) {
            n.level = 0.0;
        }
        return false;
    };
    let pos = g.ch[c].pos;
    let path = match crate::scene_collision::nav_grid_find_path(g, pos, spot.pos, Some(6000.0), None) {
        Some(p) if !p.is_empty() => p,
        _ => {
            if let Some(n) = mem(g, gd).needs.iter_mut().find(|x| x.kind == kind) {
                n.level = 0.0;
            }
            return false;
        }
    };
    let mut q = path[0];
    for &p in path.iter() {
        q = p;
        if v3::dist_xz(p, pos) > 2.5 {
            break;
        }
    }
    let out_yaw = v3::yaw_to(pos, q);
    let now = g.time;
    let last = floor(g).last_depart_at;
    let lead = g.k.needs.announce_lead;
    let wp = g.gd[gd].wp;
    crate::owner::claim(g, gd, crate::owner::by::NEEDS, now, None);
    mem(g, gd).trip = Some(Trip {
        kind,
        spot,
        phase: TripPhase::Telegraph,
        t0: now,
        depart_at: now + lead,
        left_at: -1.0,
        arrived_at: -1.0,
        dwell_until: -1.0,
        wp: wp as f64,
        out_yaw,
        watched: false,
        told: false,
        partner: None,
        reply_at: -1.0,
        replied: false,
        necked: false,
        loop_i: -1.0,
        line: String::new(),
        gap_before: last,
    });
    floor(g).last_depart_at = m::max(last, now + lead);
    true
}
/// Break the trip off wherever he is; nothing is resumed.
fn cancel_trip(g: &mut crate::game::Game, gd: GuardId, why: &str) {
    if !on_trip(g, gd) {
        return;
    }
    let now = g.time;
    crate::owner::release(g, gd, crate::owner::by::NEEDS, now);
    let Some(t) = mem(g, gd).trip.take() else { return };
    mem(g, gd).cancels += 1.0;
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
        let mut d = break_note(g, gd, t.kind, "cancelled");
        d.set(String::from("why"), text(why));
        d.set(String::from("phase"), text(t.phase.as_str()));
        let at = g.ch[c].pos;
        crate::chronicle::chronicle(g, ChronKind::NeedBreak, Some(&d), Some(at));
    }
}

/// The line for a slot, off his name and his count.
fn pick_line(g: &mut crate::game::Game, pool: &[String], gd: GuardId, salt: &str) -> String {
    let trips = mem(g, gd).trips;
    let cancels = mem(g, gd).cancels;
    let h = crate::dice::hash_str(&format!("{}|{}|{}|{}", g.gd[gd].callsign, trips, cancels, salt));
    pool[js::rem(h, pool.len() as f64) as usize].clone()
}

/// The colleague he tells: the nearest staff man his words reach.
fn partner_for(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let mouth = head_or_guess(g, gd);
    let my_pos = g.pos_of(gd);
    let mut best = None;
    let mut bd = m::INF;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let x = &g.gd[o];
        if o == gd || g.cond[x.ch].down || x.held.is_some() || g.ch[x.ch].team != Team::Guard || x.unit.is_some() {
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

/// Say it: to the partner's face, else over the net.
fn announce(g: &mut crate::game::Game, gd: GuardId, t: &mut crate::needs::Trip) {
    t.told = true;
    let p = partner_for(g, gd);
    let quiet = g.level.own_places();
    let lines = crate::dialogue::needs_lines_table();
    let kind = t.kind.as_str();
    if let Some(p) = p {
        t.partner = Some(p);
        t.reply_at = g.time + g.k.needs.reply_after;
        let own = if quiet { lines.announce_own.of(kind) } else { None };
        let pool: Vec<String> = own.or_else(|| lines.announce.of(kind)).or_else(|| lines.announce.of("coffee")).unwrap().clone();
        t.line = pick_line(g, &pool, gd, "announce");
        crate::game::game_say(g, gd, &t.line, None, None);
    } else {
        let own = if quiet { lines.short_own.of(kind) } else { None };
        let short: String = own.or_else(|| lines.short.of(kind)).cloned().unwrap_or_else(|| String::from(kind));
        t.line = lines.radio.replacen("{who}", &g.gd[gd].callsign, 1).replacen("{short}", &short, 1);
        crate::game::game_say(g, gd, &t.line, Some(true), None);
        let c = g.gd[gd].ch;
        let at = g.ch[c].bone(crate::character::Bone::Head).unwrap_or(g.ch[c].pos);
        crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Radio, Some(at), Some(0.4), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.05 }));
    }
}

/// The partner's grumble, if he is still one to give it.
fn partner_reply(g: &mut crate::game::Game, gd: GuardId, t: &mut crate::needs::Trip) {
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
        return;
    }
    let third = g.guards.iter().copied().find(|&x| x != gd && x != p && g.team(x) == Team::Guard && g.gd[x].unit.is_none() && !crate::condition::down(g, g.gd[x].ch));
    let trips = mem(g, gd).trips;
    let lines = crate::dialogue::needs_lines_table();
    let pool: Vec<String> = match third {
        Some(x) if js::rem(crate::dice::hash_str(&format!("{}|{}|other", pcs, trips)), 3.0) == 0.0 => {
            let other = g.gd[x].callsign.clone();
            lines.reply_other.iter().map(|s| s.replace("{other}", &other)).collect()
        }
        _ => {
            if t.kind == NeedKind::Stretch { lines.reply.iter().filter(|s| s.as_str() != "Bring me one.").cloned().collect() } else { lines.reply.clone() }
        }
    };
    let cs = g.gd[gd].callsign.clone();
    let h = crate::dice::hash_str(&format!("{}|{}|{}|reply", pcs, cs, trips));
    let line = pool[js::rem(h, pool.len() as f64) as usize].clone();
    crate::game::game_say(g, p, &line, None, None);
}

/// `stand()` of needsPatrol: his feet stop.
fn stand(g: &mut Game, gd: GuardId, dt: f64) {
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
    let c = g.gd[gd].ch;
    g.ch[c].vel = [0.0, 0.0, 0.0];
}

/// What every stage of a trip shares this frame.
#[derive(Clone, Copy)]
struct TripFrame {
    gd: GuardId,
    c: CharId,
    now: f64,
    dt: f64,
    pace: f64,
    look_phase: f64,
    kind: NeedKind,
}

/// If a trip owns his feet this frame, run them and return true; false = the branch walks his route.
pub(super) fn needs_patrol(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool {
    if !on_trip(g, gd) {
        return false;
    }
    let f = TripFrame { gd, c: g.gd[gd].ch, now: g.time, dt, pace: walk * g.k.needs.pace_k, look_phase: g.gd[gd].look_phase, kind: trip!(g, gd).kind };
    match trip!(g, gd).phase {
        TripPhase::Telegraph => telegraph(g, f),
        TripPhase::Going => going(g, f),
        TripPhase::Dwell => dwelling(g, f),
        TripPhase::Returning => return !coming_back(g, f),
    }
    true
}

/// Before he leaves he stands, looks along his way, says it, stretches his neck and turns to go; then he sets off.
fn telegraph(g: &mut Game, f: TripFrame) {
    let TripFrame { gd, c, now, dt, look_phase, .. } = f;
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
    g.ch[c].anim.look_yaw_extra = telegraph_look(g, f, now - t0 - GLANCE_AT, out_yaw);
    say_it_and_hear_back(g, gd, now, to_go);
    if !trip!(g, gd).necked && to_go <= g.k.needs.stretch_lead {
        trip!(g, gd).necked = true;
        crate::character::anim_tell(g, c, crate::character::TellKind::Neck);
    }
    if trip!(g, gd).necked {
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, out_yaw, 2.2 * dt);
    }
    if to_go <= 0.0 {
        set_off(g, f);
    }
}

/// Where his eyes go while he waits: one glance along his way out (`g0` seconds into it), else idly about.
fn telegraph_look(g: &Game, f: TripFrame, g0: f64, out_yaw: f64) -> f64 {
    if g0 > 0.0 && g0 < GLANCE_SECS {
        let turn = v3::clamp(v3::wrap_angle(out_yaw - g.ch[f.c].body_yaw), -1.1, 1.1);
        return turn * m::min_n(&[1.0, g0 / 0.35, (GLANCE_SECS - g0) / 0.45]);
    }
    m::sin(f.look_phase * 1.3) * 0.3
}

/// He says he is going when it is nearly time, and his partner answers when his moment comes.
fn say_it_and_hear_back(g: &mut Game, gd: GuardId, now: f64, to_go: f64) {
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
}

/// He leaves: the floor's gap counts from now, and the chronicle has him gone.
fn set_off(g: &mut Game, f: TripFrame) {
    let TripFrame { gd, c, now, kind, .. } = f;
    let t = trip!(g, gd);
    t.phase = TripPhase::Going;
    t.left_at = now;
    t.loop_i = -1.0;
    let floor_row = floor(g);
    floor_row.last_depart_at = m::max(floor_row.last_depart_at, now);
    clear_stall(g, gd);
    let spot_pos = trip!(g, gd).spot.pos;
    crate::guards::go_to(g, gd, spot_pos, Some(true));
    let mut d = break_note(g, gd, kind, "left");
    d.set(String::from("to"), text(&trip!(g, gd).spot.name));
    if g.level.own_places() {
        d.set(String::from("own"), Some(ChronicleDataValue::Num(1.0)));
    }
    let at = g.ch[c].pos;
    crate::chronicle::chronicle(g, ChronKind::NeedBreak, Some(&d), Some(at));
}

/// The chronicle's row of a break: who, which need, and what just happened.
fn break_note(g: &Game, gd: GuardId, kind: NeedKind, what: &str) -> JsMap<String, Option<ChronicleDataValue>> {
    let mut d = JsMap::new();
    d.set(String::from("who"), text(&g.gd[gd].callsign));
    d.set(String::from("kind"), text(kind.as_str()));
    d.set(String::from("what"), text(what));
    d
}

/// One frame of his walk to `goal`, eyes easy; true when he has got to the end of his path.
fn walk_on(g: &mut Game, f: TripFrame, goal: V3) -> bool {
    let TripFrame { gd, c, dt, pace, look_phase, .. } = f;
    crate::guards::go_to(g, gd, goal, None);
    let arrived = crate::guards::follow_path(g, gd, dt, pace);
    g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(look_phase * 0.7) * 0.25, 2.0, dt);
    g.ch[c].anim.look_yaw_extra = 0.0;
    arrived
}

/// On his way to the place, or round its loop; a path that ends short or a walk that gets nowhere ends the trip.
fn going(g: &mut Game, f: TripFrame) {
    let TripFrame { gd, c, dt, .. } = f;
    let loop_i = trip!(g, gd).loop_i;
    let goal = if loop_i >= 0.0 { trip!(g, gd).spot.loop_.as_ref().unwrap()[loop_i as usize] } else { trip!(g, gd).spot.pos };
    if !walk_on(g, f, goal) {
        if crate::guards::not_getting_anywhere(g, gd, dt, g.k.needs.stuck_secs) {
            cancel_trip(g, gd, "stuck on the way");
        }
        return;
    }
    clear_stall(g, gd);
    if v3::dist_xz(g.ch[c].pos, goal) > 1.2 {
        cancel_trip(g, gd, "no way through");
        return;
    }
    reached_a_stop(g, f, loop_i);
}

/// At the place he stands out his dwell; on a loop he walks to its next point, and from its last one straight back.
fn reached_a_stop(g: &mut Game, f: TripFrame, loop_i: f64) {
    let TripFrame { gd, now, kind, .. } = f;
    let Some(n) = trip!(g, gd).spot.loop_.as_ref().map(|l| l.len()) else {
        let dwell = g.k.needs.dwell.get(kind).copied().unwrap_or(20.0);
        let t = trip!(g, gd);
        t.phase = TripPhase::Dwell;
        t.arrived_at = now;
        t.dwell_until = now + dwell;
        return;
    };
    if loop_i + 1.0 < n as f64 {
        if loop_i < 0.0 {
            trip!(g, gd).arrived_at = now;
        }
        let t = trip!(g, gd);
        t.loop_i += 1.0;
        let next = t.spot.loop_.as_ref().unwrap()[t.loop_i as usize];
        crate::guards::go_to(g, gd, next, Some(true));
        return;
    }
    let t = trip!(g, gd);
    t.phase = TripPhase::Returning;
    t.dwell_until = now;
    head_back(g, gd);
}

/// He has his break, facing the place's way, until his time is up.
fn dwelling(g: &mut Game, f: TripFrame) {
    let TripFrame { gd, c, now, dt, look_phase, .. } = f;
    stand(g, gd, dt);
    let face = trip!(g, gd).spot.face;
    g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, face, 2.5 * dt);
    g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(look_phase * 0.5) * 0.3, 2.0, dt);
    g.ch[c].anim.look_yaw_extra = m::sin(look_phase * 0.8) * 0.35;
    if now >= trip!(g, gd).dwell_until {
        trip!(g, gd).phase = TripPhase::Returning;
        clear_stall(g, gd);
        head_back(g, gd);
    }
}

/// His path is set for the waypoint he left from.
fn head_back(g: &mut Game, gd: GuardId) {
    let wp = trip!(g, gd).wp;
    let back = route_point(g, gd, wp);
    crate::guards::go_to(g, gd, back, Some(true));
}

/// On his way back; true when he is back on his route and the trip is over.
fn coming_back(g: &mut Game, f: TripFrame) -> bool {
    let TripFrame { gd, c, dt, .. } = f;
    let wp = trip!(g, gd).wp;
    let goal = route_point(g, gd, wp);
    if !walk_on(g, f, goal) {
        if crate::guards::not_getting_anywhere(g, gd, dt, g.k.needs.stuck_secs) {
            cancel_trip(g, gd, "stuck coming back");
        }
        return false;
    }
    clear_stall(g, gd);
    if v3::dist_xz(g.ch[c].pos, goal) > 1.5 {
        cancel_trip(g, gd, "no way back");
        return false;
    }
    trip_over(g, f);
    true
}

/// The trip is done: that need is empty, his rest starts now, and he says he is back.
fn trip_over(g: &mut Game, f: TripFrame) {
    let TripFrame { gd, c, now, kind, .. } = f;
    let left_at = trip!(g, gd).left_at;
    crate::owner::release(g, gd, crate::owner::by::NEEDS, now);
    let r = row!(g, gd);
    r.trip = None;
    r.trips += 1.0;
    r.last_back_at = now;
    if let Some(n) = r.needs.iter_mut().find(|x| x.kind == kind) {
        n.level = 0.0;
    }
    let away = m::round(now - left_at);
    let back = crate::dialogue::needs_lines_table().back;
    let line = pick_line(g, &back, gd, "back");
    crate::game::game_say(g, gd, &line, None, None);
    let mut d = break_note(g, gd, kind, "back");
    d.set(String::from("away"), Some(ChronicleDataValue::Num(away)));
    let at = g.ch[c].pos;
    crate::chronicle::chronicle(g, ChronKind::NeedBreak, Some(&d), Some(at));
}

/// The probes: put a man's need at a level; false if he does not carry that kind.
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

#[cfg(test)]
mod tests {
    use super::*;

    /// `path_len` against pinned bits.
    #[test]
    fn path_len_bits() {
        let from = [1.25, 0.0, -3.5];
        let path = [[4.75, 0.0, 2.125], [10.1, 1.0, 9.7], [-3.3, 0.0, 0.4]];
        assert_eq!(crate::places::walk_len(from, &path).to_bits(), BITS);
    }
    const BITS: u64 = 0x40401adb95a66fb4;
}
