//! src/game/powerout.ts lines 1 to 419
//! The lights go out and somebody goes to put them back on: a room fight's defenders notice, say what they do on their net, and walk to the breaker.
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small private helpers

/// the state in its slot: the rule's passes run only with it there. (powerout.ts `st`, once a parameter)
fn st(g: &mut Game) -> &mut State {
    g.m.powerout.states.as_mut().unwrap()
}
/// a man of the party, by his place in `st.men`
fn man(g: &mut Game, i: usize) -> &mut Man {
    &mut st(g).men[i]
}
/// a man's body's place, read now
fn pos_of(g: &Game, gd: GuardId) -> V3 {
    g.ch[g.gd[gd].ch].pos
}
/// `bo.active`: a getter over the mains' columns, read live every time as the TypeScript's `bo` is
fn bo_active(g: &mut Game) -> bool {
    crate::sim_lightshome::mains_active(g)
}
/// `bo.since`, read live
fn bo_since(g: &mut Game) -> f64 {
    crate::sim_lightshome::mains_since(g)
}
/// `bo.permanent`, read live
fn bo_permanent(g: &mut Game) -> bool {
    crate::sim_lightshome::mains_permanent(g)
}
/// `x.toFixed(d)` for the log's lines (an exact tie rounds to even here, away from zero there: the words of a log, nothing reads the digit)
fn to_fixed(x: f64, d: usize) -> String {
    alloc::format!("{:.*}", d, x)
}
/// `event`'s parameter and `PowerOutEvent.what` came out as two enums of the same words
fn what_of(what: EventWhat) -> PowerOutEventWhat {
    match what {
        EventWhat::Stay => PowerOutEventWhat::Stay,
        EventWhat::Lost => PowerOutEventWhat::Lost,
        EventWhat::Home => PowerOutEventWhat::Home,
        EventWhat::Go => PowerOutEventWhat::Go,
        EventWhat::There => PowerOutEventWhat::There,
        EventWhat::Said => PowerOutEventWhat::Said,
        EventWhat::Noticed => PowerOutEventWhat::Noticed,
        EventWhat::Nobody => PowerOutEventWhat::Nobody,
        EventWhat::Abort => PowerOutEventWhat::Abort,
        EventWhat::Wrecked => PowerOutEventWhat::Wrecked,
        EventWhat::Restored => PowerOutEventWhat::Restored,
        EventWhat::Overdue => PowerOutEventWhat::Overdue,
    }
}
/// `g.guards[i]` for an index kept in `seen` (−1 or off the end of the list: nobody)
fn guard_at(g: &Game, i: f64) -> Option<GuardId> {
    if i < 0.0 { None } else { g.guards.get(i as usize).copied() }
}
/// `gd.script.arrived = false` where he has a script
fn clear_arrived(g: &mut Game, gd: GuardId) {
    if let Some(s) = g.gd[gd].script.as_mut() {
        s.arrived = Some(false);
    }
}

// ---------------------------------------------------------------- the rule's small questions

/// distance on the floor. (powerout.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// the fractional part. (powerout.ts frac)
pub fn frac(x: f64) -> f64 {
    x - m::floor(x)
}

/// his id's share of 0‥1, no dice. (powerout.ts idShare)
pub fn id_share(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    frac(g.ch[g.gd[gd].ch].id * 0.6180339887)
}

/// a living man who has not fled the level. (powerout.ts alive)
pub fn alive(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].state != crate::game::GuardState::Dead && g.ch[g.gd[gd].ch].alive && !crate::runner::runner_fled(g, gd)
}

/// alive, nobody's arm on him, his wits about him. (powerout.ts fit)
pub fn fit(g: &mut crate::game::Game, gd: GuardId) -> bool {
    alive(g, gd) && g.gd[gd].held.is_none() && g.time >= g.gd[gd].dazzled_until && g.time >= g.gd[gd].stagger_until
}

/// alert or searching. (powerout.ts roused)
pub fn roused(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].state == crate::game::GuardState::Alert || g.gd[gd].state == crate::game::GuardState::Search
}

/// what his ears have counted of shots, kicks and bangs. (powerout.ts loudHeard)
pub fn loud_heard(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let h = &g.gd[gd].witness.heard;
    h.shots + h.kick + h.bang
}

/// his row in the room fight. (powerout.ts defOf)
pub fn def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> {
    g.fight.defs.iter().copied().find(|&d| g.tab.roomfight_def[d].gd == gd)
}

/// the rule's men: the room fight's defenders on the other side from the player, in live play. (powerout.ts ruled)
pub fn ruled(g: &mut crate::game::Game) -> Vec<RoomfightDefId> {
    if g.touring || crate::director::director_showing(g) || g.quiet_utility {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.fight.defs.len() {
        let d = g.fight.defs[i];
        i += 1;
        let gd = g.tab.roomfight_def[d].gd;
        let team = g.gd[gd].team;
        if crate::teams::is_hostile(g, team, crate::teams::Team::Player) && g.gd[gd].unit.is_some() {
            out.push(d);
        }
    }
    out
}

/// a line of the timeline and of the fight's own log. (powerout.ts event)
pub fn event(g: &mut crate::game::Game, what: crate::powerout::EventWhat, gd: Option<GuardId>, why: &str) {
    let who: String = match gd {
        Some(id) => g.gd[id].callsign.clone(),
        None => "-".into(),
    };
    let t = g.time;
    let s = st(g);
    s.events.push(PowerOutEvent { t, what: what_of(what), who: who.clone(), why: why.into() });
    if s.events.len() > 120 {
        s.events.drain(0..40);
    }
    let l = &mut g.fight.log;
    l.push(crate::roomfight::FightNote { t, who, what: alloc::format!("[powerout] {}: {}", what.as_str(), why) });
    if l.len() > 600 {
        l.drain(0..200);
    }
}

/// a mains-fed fixture that was burning, in his plain view inside `seeM`. (powerout.ts sawThemDie)
pub fn saw_them_die(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let c = pos_of(g, gd);
    let eye: V3 = [c[0], 1.5, c[2]];
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        if !g.tab.sim_lightshome_fixture[t].mains
            || !crate::sim_lightshome::fixture_on(g, t)
            || crate::sim_lightshome::fixture_broken(g, t)
            || !(crate::sim_lightshome::fixture_factor(g, t) > 0.3)
            || g.tab.sim_lightshome_fixture[t].kind == crate::sim_lightshome::FixtureKind::Breaker
            || g.tab.sim_lightshome_fixture[t].kind == crate::sim_lightshome::FixtureKind::Device
        {
            continue;
        }
        let tp = g.tab.sim_lightshome_fixture[t].pos;
        if flat(tp, pos_of(g, gd)) > g.k.powerout.see_m {
            continue;
        }
        if !crate::scene_collision::segment_blocked(g, eye, [tp[0], m::max(1.2, tp[1] - 0.15), tp[2]], None, None, None) {
            return true;
        }
    }
    false
}

/// the meter's reading of a point of floor, by the place's own light. (powerout.ts floorWatched's `at`)
fn floor_at(g: &mut Game, x: f64, z: f64) -> f64 {
    let e = crate::guards::light_estimate_at(g, [x, 1.2, z], Some(&mut |_g: &mut Game, _owner: f64| carried()), Some(true), None, None);
    crate::game::game_visibility_of(g, e)
}

/// the brightest floor he stands on, watches or has about him, by the place's own light. (powerout.ts floorWatched)
pub fn floor_watched(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let k = g.k.powerout;
    let c = pos_of(g, gd);
    let yaw = g.ch[g.gd[gd].ch].aim_yaw;
    let eye: V3 = [c[0], 1.5, c[2]];
    let mut best = floor_at(g, c[0], c[2]);
    for off in [0.0, -k.fan_rad, k.fan_rad] {
        let fx = m::sin(yaw + off);
        let fz = m::cos(yaw + off);
        let mut r = k.step_m;
        while r <= k.look_m + 1e-6 {
            let x = eye[0] + fx * r;
            let z = eye[2] + fz * r;
            if crate::scene_collision::segment_blocked(g, eye, [x, 1.5, z], None, None, None) {
                break;
            }
            let v = floor_at(g, x, z);
            best = m::max(best, v);
            r += k.step_m;
        }
    }
    // (… and the floor about him, whichever way he faces)
    let mut n = 0;
    while n < 8 {
        let a = (n as f64) * m::PI / 4.0;
        let fx = m::sin(a);
        let fz = m::cos(a);
        let mut r = k.step_m;
        while r <= k.about_m + 1e-6 {
            let x = eye[0] + fx * r;
            let z = eye[2] + fz * r;
            if crate::scene_collision::segment_blocked(g, eye, [x, 1.5, z], None, None, None) {
                break;
            }
            let v = floor_at(g, x, z);
            best = m::max(best, v);
            r += k.step_m;
        }
        n += 1;
    }
    best
}

/// every carried lamp is left out of the reading. (powerout.ts carried)
pub fn carried() -> bool {
    true
}

/// why a defender is not free to go (None = he is): calm, on a post the room fight gave him, and nobody else's. (powerout.ts powerOutKept)
pub fn power_out_kept(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<String> {
    let gd = g.tab.roomfight_def[d].gd;
    if !fit(g, gd) {
        return Some("not his own man".into());
    }
    if g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Hold {
        return Some("on the move".into());
    }
    if g.tab.roomfight_def[d].rally_at == m::INF {
        return Some("the mission's man".into());
    }
    if g.gd[gd].script.is_some() {
        return Some("on a script".into());
    }
    if roused(g, gd) {
        return Some("in a fight".into());
    }
    if crate::rescue::rescue_crew_hides(g, gd) || crate::rescue::rescue_captor(g) == Some(gd) || crate::laststand::last_stand_man(g, gd) {
        return Some("the captor / lying in wait / last stand".into());
    }
    if crate::digin::dig_in_phase(g, gd) != "idle" {
        return Some("dug in".into());
    }
    if crate::enemysquads::squad_defers(g, gd) {
        return Some("his squad's".into());
    }
    if crate::lanes::lane_holds(g, gd, None) || crate::netquiet::net_quiet_holds(g, gd) {
        return Some("holds a lane / the last man".into());
    }
    // (pressOwns is roomfight's hook and may stop his rally clock as it answers: asked here it changes nothing)
    let rally = g.tab.roomfight_def[d].rally_at;
    let press = crate::pressure::press_owns(g, d);
    g.tab.roomfight_def[d].rally_at = rally;
    if press || crate::buddy::buddy_owns(g, d) {
        return Some("the press's".into());
    }
    None
}

/// where the man at the lever stands. (powerout.ts leverSpot)
pub fn lever_spot(g: &mut crate::game::Game) -> V3 {
    let e = crate::workthings::breaker_errand(g);
    let bp = g.level.breaker.pos;
    match e {
        Some(e) => e.goal,
        None => [bp[0] - 0.4, 0.0, bp[2] - 0.9],
    }
}

/// the way from a man to a spot as the walk grid stands, and its length; None = none that ends there. (powerout.ts wayTo)
pub fn way_to(g: &mut crate::game::Game, from: V3, to: V3) -> Option<crate::powerout::WayTo> {
    if flat(from, to) < 0.5 {
        return Some(WayTo { path: vec![to], l: flat(from, to) });
    }
    let path = crate::scene_collision::nav_grid_find_path(g, from, to, Some(20000.0), None)?;
    if path.is_empty() {
        return None;
    }
    let mut l = 0.0;
    let mut w0 = from;
    for &w in &path {
        l += flat(w0, w);
        w0 = w;
    }
    if flat(w0, to) > 1.0 { None } else { Some(WayTo { path, l }) }
}

/// the second man's place: free floor `coverM` short of the lever with a line to it, nearest the way they came; and what he watches from it. (powerout.ts coverSpot)
pub fn cover_spot(g: &mut crate::game::Game, lever: V3, way: &[V3], from: V3) -> crate::powerout::CoverSpot {
    let cs = g.col.nav.cell;
    let [r0, r1] = g.k.powerout.cover_m;
    // the point of the way about six metres before its end: where anybody following them would come from
    let mut back = from;
    let mut left = 6.0;
    let mut i = way.len() as isize - 1;
    while i > 0 && left > 0.0 {
        let u = i as usize;
        left -= flat(way[u], way[u - 1]);
        back = way[u - 1];
        i -= 1;
    }
    let mut best: Option<V3> = None;
    let mut bd = m::INF;
    let mut r = r0;
    while r <= r1 + 1e-6 {
        let mut k = 0;
        while k < 12 {
            let a = (k as f64) * m::PI / 6.0;
            k += 1;
            let p: V3 = [lever[0] + m::sin(a) * r, 0.0, lever[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2])
                || crate::scene_collision::nav_grid_is_blocked(g, p[0] + cs, p[2])
                || crate::scene_collision::nav_grid_is_blocked(g, p[0] - cs, p[2])
                || crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2] + cs)
                || crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2] - cs)
            {
                continue;
            }
            if crate::scene_collision::segment_blocked(g, [p[0], 1.2, p[2]], [lever[0], 1.2, lever[2]], None, None, None) {
                continue;
            }
            let d = flat(p, back);
            if d < bd {
                bd = d;
                best = Some(p);
            }
        }
        r += 0.8;
    }
    CoverSpot { spot: best.unwrap_or(lever), watch: back }
}

/// how a man holds his post now. (powerout.ts calmOf)
pub fn calm_of(g: &mut crate::game::Game, d: RoomfightDefId) -> crate::powerout::Calm {
    let row = &g.tab.roomfight_def[d];
    let gd = &g.gd[row.gd];
    Calm {
        awareness: gd.awareness,
        hold: gd.hold,
        pinned: gd.pinned,
        station: row.station,
        home: row.home,
        room: row.room,
        ev: row.ev.clone(),
        watch: row.watch,
        want: row.want,
        pt: row.pt,
    }
}

/// a leg of the walk: pressure.ts's scripted leg at the pace of a man going to look. (powerout.ts walk)
pub fn walk(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) {
    crate::pressure::run(g, d, to, None, None);
    let gd = g.tab.roomfight_def[d].gd;
    let speed = g.tune.guard_investigate;
    crate::squad::order(g, gd, crate::squad::OrderPatch { speed: Some(Some(speed)), ..Default::default() });
}

/// back on the post he left, as he held it. (powerout.ts settle)
pub fn settle(g: &mut crate::game::Game, m: &crate::powerout::Man, d: RoomfightDefId) {
    let word = st(g).word;
    calm_back(g, d, m.calm.as_ref().unwrap(), word);
}

/// a man back on the post he left, as `calm_of` read it there; `wary`: he is not calmed down again. (powerout.ts calmBack)
pub fn calm_back(g: &mut crate::game::Game, d: RoomfightDefId, c_2: &crate::powerout::Calm, wary: bool) {
    let gd = g.tab.roomfight_def[d].gd;
    if g.gd[gd].script.is_some() {
        let state = if wary { crate::squad::EndGuardScriptState::Suspicious } else { crate::squad::EndGuardScriptState::Patrol };
        crate::squad::end_guard_script(g, gd, Some(state));
    }
    g.gd[gd].awareness = if wary { m::min(0.45, m::max(c_2.awareness, 0.35)) } else { c_2.awareness };
    g.gd[gd].hold = c_2.hold;
    g.gd[gd].pinned = c_2.pinned;
    let state: String = g.gd[gd].state.as_str().into();
    let seen = g.gd[gd].last_seen_t;
    let row = &mut g.tab.roomfight_def[d];
    // (asked before pt moves: is his fix already this very pt? then the store below is the same object written again and no new fix)
    let was_pt = row.fix_is_pt && g.gd[gd].last_known == Some(row.pt);
    row.mode = crate::roomfight::DefMode::Hold;
    row.station = c_2.station;
    row.home = c_2.home;
    row.room = c_2.room;
    row.ev = c_2.ev.clone();
    row.watch = c_2.watch;
    row.want = c_2.want;
    row.pt[0] = c_2.pt[0];
    row.pt[1] = c_2.pt[1];
    row.pt[2] = c_2.pt[2];
    row.glance_until = -1.0;
    row.fix_keep = -1.0;
    row.rally_at = m::NAN;
    row.last_state = state;
    row.last_seen = seen;
    // `gd.lastKnown = D.pt` is by reference: from here his fix IS the row's pt, and the room fight's moves of pt carry it
    row.fix_is_pt = true;
    let pt = row.pt;
    g.gd[gd].last_known = Some(pt);
    if !was_pt {
        g.gd[gd].fix_stamp = g.gd[gd].fix_stamp.wrapping_add(1);
    }
}

/// a kept post back from a checkpoint's words: its room by id. (powerout.ts calmLoad)
pub fn calm_load(g: &mut crate::game::Game, c_2: &crate::powerout::CalmSnap) -> crate::powerout::Calm {
    let room = g.fight.defs.iter().filter_map(|&d| g.tab.roomfight_def[d].room).find(|&r| g.tab.roomfight_room[r].id == c_2.room);
    Calm {
        awareness: c_2.awareness,
        hold: c_2.hold,
        pinned: c_2.pinned,
        station: c_2.station,
        home: c_2.home,
        room,
        ev: c_2.ev.clone(),
        watch: c_2.watch,
        want: c_2.want,
        pt: c_2.pt,
    }
}

// ---------------------------------------------------------------- the trip

/// lighting.ts setBlackout, on a trip that found the mains up: the answer's clock starts. (powerout.ts powerOut)
pub fn power_out(g: &mut crate::game::Game) {
    let defs = ruled(g);
    if defs.is_empty() {
        return;
    }
    let mut first: Option<GuardId> = None;
    for &d in &defs {
        let gd = g.tab.roomfight_def[d].gd;
        if fit(g, gd) && saw_them_die(g, gd) {
            first = Some(gd);
            break;
        }
    }
    let prev = g.m.powerout.states.take();
    let out = prev.as_ref().map_or(false, |p| !p.men.is_empty()); // (a party of the last blackout still out: it is this one's answer too)
    let mut new_st = State {
        since: g.time,
        decide_at: m::INF,
        go_at: if out { prev.as_ref().unwrap().go_at } else { -1.0 },
        way_m: if out { prev.as_ref().unwrap().way_m } else { 0.0 },
        overdue_at: if out { prev.as_ref().unwrap().overdue_at } else { -1.0 },
        fell: if out { prev.as_ref().unwrap().fell } else { None },
        seen: Vec::new(),
        look_at: -1.0,
        decided: out,
        word: prev.as_ref().map_or(false, |p| p.word),
        parties: prev.as_ref().map_or(0.0, |p| p.parties),
        lost: prev.as_ref().map_or(false, |p| p.lost),
        men: Vec::new(),
        events: Vec::new(),
    };
    if let Some(p) = prev {
        new_st.men = p.men;
        new_st.events = p.events;
    }
    g.m.powerout.states = Some(new_st);
    if out {
        let mut i = 0;
        while i < st(g).men.len() {
            let gd = man(g, i).gd;
            let at = i;
            i += 1;
            let Some(d) = def_of(g, gd) else { continue };
            if man(g, at).phase != Phase::Back || g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Move {
                continue;
            }
            man(g, at).phase = Phase::Out;
            man(g, at).tries = 0.0;
            let goal = man(g, at).goal;
            walk(g, d, goal);
            event(g, EventWhat::Go, Some(gd), "the lights go again on his way home: he turns back for the lever");
        }
        return;
    }
    if let Some(first) = first {
        return noticed(g, first, "the lights die in his view");
    }
    // (the lights are still burning this frame: what each man's floor reads now is read again in the dark)
    for &d in &defs {
        let gd = g.tab.roomfight_def[d].gd;
        if !fit(g, gd) {
            continue;
        }
        let lit = floor_watched(g, gd);
        if lit >= g.k.powerout.lit_at {
            let i = g.guards.iter().position(|&x| x == gd).map_or(-1.0, |p| p as f64);
            st(g).seen.push(StateSeenItem { i, lit });
        }
    }
    if !st(g).seen.is_empty() {
        let at = g.time + g.k.powerout.dark_after;
        st(g).look_at = at;
    } else {
        nobody(g);
    }
}

/// somebody knows: their word's clock, by his id. (powerout.ts noticed)
pub fn noticed(g: &mut crate::game::Game, gd: GuardId, how: &str) {
    let k = g.k.powerout;
    let share = id_share(g, gd);
    let time = g.time;
    let s = st(g);
    s.seen = Vec::new();
    s.look_at = -1.0;
    s.decide_at = s.since + k.think[0] + (k.think[1] - k.think[0]) * share;
    let why = alloc::format!("{}: their word is due {} s on", how, to_fixed(s.decide_at - time, 1));
    event(g, EventWhat::Noticed, Some(gd), &why);
}

/// nobody knows the power is out. (powerout.ts nobody)
pub fn nobody(g: &mut crate::game::Game) {
    let s = st(g);
    s.seen = Vec::new();
    s.look_at = -1.0;
    s.decided = true;
    event(g, EventWhat::Nobody, None, "no defender had a burning mains fixture in his view or lost the light on his floor: nobody knows the power is out");
}

/// lighting.ts setBlackout: is the walk to the breaker this rule's to hand out? (powerout.ts powerOutMan)
pub fn power_out_man(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].unit.is_none() {
        return false;
    }
    let defs = ruled(g);
    defs.iter().any(|&d| g.tab.roomfight_def[d].gd == gd)
}

/// guards.ts poseFlashlight: a man out for the lights burns his own while the mains are down. (powerout.ts powerOutLit)
pub fn power_out_lit(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !bo_active(g) {
        return false;
    }
    let stepped = match g.m.powerout.states.as_ref().and_then(|s| s.men.iter().find(|x| x.gd == gd)) {
        Some(x) => x.phase != Phase::Wait,
        None => false,
    };
    stepped && crate::darkfoe::dark_foe_carries(g, gd)
}

/// darkfoe.ts hasWord: their side holds this dark for the team's doing. (powerout.ts powerOutWord)
pub fn power_out_word(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !bo_active(g) || !g.m.powerout.states.as_ref().map_or(false, |s| s.word) {
        return false;
    }
    let team = g.gd[gd].team;
    crate::teams::is_hostile(g, team, crate::teams::Team::Player)
}

/// is he out for the lights, or about to step off? (powerout.ts powerOutOwns)
pub fn power_out_owns(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.m.powerout.states.as_ref().map_or(false, |s| s.men.iter().any(|x| x.gd == gd))
}

/// what was said and done, oldest first. (powerout.ts powerOutTimeline)
pub fn power_out_timeline(g: &mut crate::game::Game) -> Vec<crate::powerout::PowerOutEvent> {
    g.m.powerout.states.as_ref().map_or_else(Vec::new, |s| s.events.clone())
}

/// a fresh start. (powerout.ts powerOutReset)
pub fn power_out_reset(g: &mut crate::game::Game) {
    g.m.powerout.states = None;
}

// ---------------------------------------------------------------- their word

/// nobody goes: they say so. (powerout.ts decide's `stay`)
fn stay(g: &mut Game, speaker: GuardId, line: &str, why: &str, word: bool) {
    let s = st(g);
    s.word = s.word || word;
    crate::game::game_say(g, speaker, line, Some(true), None, None);
    event(g, EventWhat::Stay, Some(speaker), why);
}

/// their word: who goes, or why nobody does. (powerout.ts decide)
pub fn decide(g: &mut crate::game::Game, defs: &[RoomfightDefId]) {
    st(g).decided = true;
    let mut live: Vec<RoomfightDefId> = Vec::new();
    for &d in defs {
        let gd = g.tab.roomfight_def[d].gd;
        if alive(g, gd) {
            live.push(d);
        }
    }
    let mut speaker: Option<GuardId> = None;
    for &d in &live {
        let gd = g.tab.roomfight_def[d].gd;
        if fit(g, gd) && saw_them_die(g, gd) {
            speaker = Some(gd);
            break;
        }
    }
    if speaker.is_none() {
        for &d in &live {
            let gd = g.tab.roomfight_def[d].gd;
            if fit(g, gd) {
                speaker = Some(gd);
                break;
            }
        }
    }
    let Some(speaker) = speaker else { return };
    let mut knows = false;
    for &d in &live {
        let gd = g.tab.roomfight_def[d].gd;
        if roused(g, gd) || loud_heard(g, gd) > 0.0 || g.gd[gd].witness.alerted_by.is_some() {
            knows = true;
            break;
        }
    }
    if knows {
        return stay(g, speaker, "power's out — that's them! Stay on your doors!", "a man of theirs is roused or has heard the team: nobody walks out to mend a fuse", true);
    }
    if crate::rescue::rescue_captor(g).is_some() {
        return stay(g, speaker, "power's out — that's them! Stay on your doors!", "men holding hostages take the dark for the assault (rescue.ts: every one of them has learned of the team by it)", true);
    }
    if st(g).lost {
        return stay(g, speaker, "power's out again — nobody goes. Stay on your doors!", "the last men sent for the lights did not all come back", true);
    }
    let parties = st(g).parties;
    if parties >= g.k.powerout.parties {
        let line = crate::words::wg(g, &"power's out again — that's no accident. Stay on your doors!", &"power's out again — that's no fault. Stay on your doors!");
        let why = alloc::format!("the lights have gone {} times this fight", parties + 1.0);
        return stay(g, speaker, line, &why, true);
    }
    let lever = lever_spot(g);
    let mut free: Vec<RoomfightDefId> = Vec::new();
    for &d in &live {
        if power_out_kept(g, d).is_none() {
            free.push(d);
        }
    }
    // the breaker in a defender's own room: its nearest man, alone, without leaving it
    let r = crate::roomfight::room_fight_room_at(g, lever);
    let mut own: Option<RoomfightDefId> = None;
    if let Some(r) = r {
        let mut mine: Vec<RoomfightDefId> = free.iter().copied().filter(|&d| g.tab.roomfight_def[d].room == Some(r)).collect();
        let by: &Game = g;
        js::sort_by(&mut mine, |&a, &b| flat(pos_of(by, by.tab.roomfight_def[a].gd), lever) - flat(pos_of(by, by.tab.roomfight_def[b].gd), lever));
        own = mine.first().copied();
    }
    let mut go: Vec<(RoomfightDefId, WayTo)> = Vec::new();
    if let Some(own) = own {
        let from = pos_of(g, g.tab.roomfight_def[own].gd);
        if let Some(way) = way_to(g, from, lever) {
            go.push((own, way));
        }
    }
    let own_goes = !go.is_empty();
    if go.is_empty() {
        let mut near = free.clone();
        let by: &Game = g;
        js::sort_by(&mut near, |&a, &b| flat(pos_of(by, by.tab.roomfight_def[a].gd), lever) - flat(pos_of(by, by.tab.roomfight_def[b].gd), lever));
        near.truncate(g.k.powerout.ask as usize); // (a path each is the dear part: only the nearest are asked)
        let mut ways: Vec<(RoomfightDefId, WayTo)> = Vec::new();
        for &d in &near {
            let from = pos_of(g, g.tab.roomfight_def[d].gd);
            if let Some(way) = way_to(g, from, lever) {
                if way.l <= g.k.powerout.max_m {
                    ways.push((d, way));
                }
            }
        }
        js::sort_by(&mut ways, |a, b| a.1.l - b.1.l);
        let mut in_room: JsMap<Option<RoomfightRoomId>, f64> = JsMap::new();
        for &d in &live {
            if g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold {
                let room = g.tab.roomfight_def[d].room;
                let n = in_room.get(room).copied().unwrap_or(0.0);
                in_room.set(room, n + 1.0);
            }
        }
        for x in &ways {
            if (go.len() as f64) >= g.k.powerout.men {
                break;
            }
            let room = g.tab.roomfight_def[x.0].room;
            let n = in_room.get(room).copied().unwrap_or(0.0);
            if room.is_none() || n < 2.0 {
                continue;
            }
            in_room.set(room, n - 1.0);
            go.push(x.clone());
        }
        if go.is_empty() && !ways.is_empty() {
            go = vec![ways[0].clone()];
        }
        if go.is_empty() {
            let why = if !free.is_empty() {
                alloc::format!("no way to the breaker inside {} m for a man who is free", g.k.powerout.max_m)
            } else {
                String::from("no man of theirs is free to go")
            };
            return stay(g, speaker, "power's out — nobody to spare. Stay on your doors!", &why, false);
        }
    }
    let lead = go[0].clone();
    let second = go.get(1).cloned();
    let lead_gd = g.tab.roomfight_def[lead.0].gd;
    let lead_man = Man {
        gd: lead_gd,
        role: ManRole::Lever,
        phase: Phase::Wait,
        goal: lever,
        from: g.tab.roomfight_def[lead.0].station,
        watch: None,
        tries: 0.0,
        step_t: -100.0,
        heard0: 0.0,
        there_t: -1.0,
        calm: None,
    };
    st(g).men.push(lead_man);
    if let Some(second) = &second {
        let from = pos_of(g, lead_gd);
        let c = cover_spot(g, lever, &lead.1.path, from);
        let cover_man = Man {
            gd: g.tab.roomfight_def[second.0].gd,
            role: ManRole::Cover,
            phase: Phase::Wait,
            goal: c.spot,
            from: g.tab.roomfight_def[second.0].station,
            watch: Some(c.watch),
            tries: 0.0,
            step_t: -100.0,
            heard0: 0.0,
            there_t: -1.0,
            calm: None,
        };
        st(g).men.push(cover_man);
    }
    let go_at = g.time + g.k.powerout.beat;
    let s = st(g);
    s.parties += 1.0;
    s.go_at = go_at;
    s.way_m = lead.1.l;
    let line = if second.is_some() { "power's out — two moving to the breaker!" } else { "power's out — on my way to the breaker!" };
    crate::game::game_say(g, lead_gd, line, Some(true), None, None);
    let names: Vec<&str> = go.iter().map(|x| g.gd[g.tab.roomfight_def[x.0].gd].callsign.as_str()).collect();
    let why = alloc::format!(
        "{} for the breaker, {} m off by the way he walks{}; they step off in {} s",
        names.join(" and "),
        to_fixed(lead.1.l, 0),
        if own_goes { ", in his own room" } else if second.is_some() { "" } else { ", alone" },
        g.k.powerout.beat
    );
    event(g, EventWhat::Said, Some(lead_gd), &why);
}

// ---------------------------------------------------------------- once a frame

/// once a frame, before the room fight's own tick: their word when it is due, and every man out a step on. (powerout.ts powerOutTick)
pub fn power_out_tick(g: &mut crate::game::Game, _dt: f64) {
    if g.m.powerout.states.is_none() {
        return;
    }
    let defs = ruled(g);
    if defs.is_empty() {
        if !st(g).men.is_empty() {
            stand_down(g);
        }
        return;
    }
    let (decided, look_at) = (st(g).decided, st(g).look_at);
    if !decided && look_at >= 0.0 && g.time >= look_at {
        let mut hit: Option<GuardId> = None;
        let since = st(g).since;
        if bo_active(g) && bo_since(g) == since {
            let mut j = 0;
            while j < st(g).seen.len() {
                let s = st(g).seen[j];
                j += 1;
                let Some(gd) = guard_at(g, s.i) else { continue };
                if fit(g, gd) && floor_watched(g, gd) < s.lit * g.k.powerout.drop_k {
                    hit = Some(gd);
                    break;
                }
            }
        }
        match hit {
            Some(gd) => noticed(g, gd, "the floor he is watching goes dark"),
            None => nobody(g),
        }
    }
    let (decided, since, decide_at) = (st(g).decided, st(g).since, st(g).decide_at);
    if !decided && bo_active(g) && bo_since(g) == since && g.time >= decide_at {
        decide(g, &defs);
    }
    let mut fight = false;
    for &d in &defs {
        let gd = g.tab.roomfight_def[d].gd;
        if alive(g, gd) && roused(g, gd) {
            fight = true;
            break;
        }
    }
    if st(g).men.is_empty() {
        missed(g, &defs, fight);
        return;
    }
    let go_at = st(g).go_at;
    if st(g).men.iter().any(|x| x.phase == Phase::Wait) && g.time >= go_at {
        step_off(g, fight);
    }
    // `lever` is the man himself in the TypeScript and stays so under the walk: st.men is only cut when the walk is over, so its place is its name
    let lever = st(g).men.iter().position(|x| x.role == ManRole::Lever);
    let n = st(g).men.len();
    let mut keep = Vec::with_capacity(n);
    let mut i = 0;
    while i < n {
        keep.push(step_man(g, i, lever, &defs, fight));
        i += 1;
    }
    let mut at = 0;
    st(g).men.retain(|_| {
        let k = keep[at];
        at += 1;
        k
    });
}

/// every man out goes back to his post at a walk. (powerout.ts powerOutTick, the two lines alike at 346 and 352)
fn all_back(g: &mut Game) {
    let mut j = 0;
    while j < st(g).men.len() {
        let x = man(g, j).gd;
        if let Some(xd) = def_of(g, x) {
            if g.tab.roomfight_def[xd].mode == crate::roomfight::DefMode::Move && man(g, j).phase != Phase::Wait {
                man(g, j).phase = Phase::Back;
                man(g, j).tries = 0.0;
                let from = man(g, j).from;
                walk(g, xd, from);
            }
        }
        j += 1;
    }
}

/// one man of the party a step on; false = he is off the list. (powerout.ts powerOutTick, the filter's callback)
fn step_man(g: &mut Game, i: usize, lever: Option<usize>, defs: &[RoomfightDefId], fight: bool) -> bool {
    let gd = man(g, i).gd;
    let d = def_of(g, gd);
    if !alive(g, gd) {
        st(g).lost = true;
        if man(g, i).phase != Phase::Wait {
            let p = pos_of(g, gd);
            st(g).fell = Some(p);
        }
        event(g, EventWhat::Lost, Some(gd), "down while he was out for the lights: nobody is sent again this fight");
        return false;
    }
    if d.is_none() || g.gd[gd].held.is_some() {
        // (in Sam's arm, or no defender of the fight's any more: whoever has him has him)
        st(g).lost = true;
        if g.gd[gd].script.is_some() && man(g, i).phase != Phase::Wait {
            let state = match g.gd[gd].state {
                crate::game::GuardState::Suspicious => crate::squad::EndGuardScriptState::Suspicious,
                crate::game::GuardState::Alert => crate::squad::EndGuardScriptState::Alert,
                crate::game::GuardState::Search => crate::squad::EndGuardScriptState::Search,
                _ => crate::squad::EndGuardScriptState::Patrol,
            };
            crate::squad::end_guard_script(g, gd, Some(state));
        }
        let why = if g.gd[gd].held.is_some() { "taken while he was out for the lights: nobody is sent again this fight" } else { "no man of the fight's any more" };
        event(g, EventWhat::Lost, Some(gd), why);
        return false;
    }
    let d = d.unwrap();
    if man(g, i).phase == Phase::Wait {
        return true;
    }
    let step_t = man(g, i).step_t;
    let step_t = crate::pressure::boots(g, gd, step_t);
    man(g, i).step_t = step_t;
    let role = man(g, i).role;
    let from = man(g, i).from;
    let heard0 = man(g, i).heard0;
    let at_lever = role == ManRole::Lever && man(g, i).phase == Phase::There;
    if g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Move {
        // (met on the way: tickMove planted him, roused — its man)
        event(g, EventWhat::Abort, Some(gd), "the room fight has him where he stands");
        return false;
    }
    if let Some(fell) = st(g).fell {
        // his mate on the floor in his plain view, or he is set short of the lever, a line to it, and no mate at it
        let p = pos_of(g, gd);
        if (man(g, i).phase == Phase::There && lever.is_none())
            || (flat(p, fell) <= g.k.powerout.see_m && !crate::scene_collision::segment_blocked(g, [p[0], 1.5, p[2]], [fell[0], 0.4, fell[2]], None, None, None))
        {
            let s = st(g);
            s.fell = None;
            s.word = true;
            crate::game::game_say(g, gd, "he's down at the breaker — that's them! Stay on your doors!", Some(true), None, None);
            crate::pressure::run(g, d, from, None, None);
            event(g, EventWhat::Abort, Some(gd), "sees the man he came with on the floor: says so, and back to his post at a run");
            return false;
        }
    }
    if (fight || roused(g, gd) || loud_heard(g, gd) > heard0) && !(at_lever && bo_active(g) && !bo_permanent(g)) {
        crate::pressure::run(g, d, from, None, None); // (the leg ends in the room fight's plant: its man from here)
        let why = if roused(g, gd) {
            String::from("he is roused")
        } else if loud_heard(g, gd) > heard0 {
            String::from("a shot, a boot or a charge in his ears")
        } else {
            let mut who: String = "undefined".into();
            for &x in defs {
                let xg = g.tab.roomfight_def[x].gd;
                if alive(g, xg) && roused(g, xg) {
                    who = g.gd[xg].callsign.clone();
                    break;
                }
            }
            alloc::format!("a fight ({} is roused)", who)
        };
        event(g, EventWhat::Abort, Some(gd), &alloc::format!("{}: back to his post at a run", why));
        return false;
    }
    if man(g, i).phase != Phase::Back && !bo_active(g) {
        // the mains are back: his mate's hand, the trip's own clock, anybody's
        man(g, i).phase = Phase::Back;
        man(g, i).tries = 0.0;
        walk(g, d, from);
        if role == ManRole::Lever || lever.is_none() {
            let why = alloc::format!("the mains are back {} s after they went: back to their posts", to_fixed(g.time - st(g).since, 1));
            event(g, EventWhat::Restored, Some(gd), &why);
        }
        return true;
    }
    let goal = if man(g, i).phase == Phase::Back { from } else { man(g, i).goal };
    let dist = flat(pos_of(g, gd), goal);
    let arrived = g.gd[gd].script.as_ref().and_then(|s| s.arrived).unwrap_or(false);
    let reached = arrived || dist < 0.5 || (man(g, i).phase != Phase::Back && role == ManRole::Lever && crate::lighting::breaker_reset_near(g, gd));
    if man(g, i).phase == Phase::There {
        // he stands where he was sent (still his leg: the room fight plants a man who has arrived, or stood a second and a half)
        clear_arrived(g, gd);
        g.tab.roomfight_def[d].stall_t = 0.0;
        g.tab.roomfight_def[d].move_t = 0.0;
        let there_t = man(g, i).there_t;
        if role == ManRole::Lever {
            if bo_active(g) && !bo_permanent(g) && g.time - there_t > g.k.powerout.lever_secs {
                // (a lever that will not take his hand: he does not stand at it for ever)
                let why = alloc::format!("{} s at a lever he cannot get a hand to: back to their posts", g.k.powerout.lever_secs);
                event(g, EventWhat::Abort, Some(gd), &why);
                all_back(g);
                return true;
            }
            let bp = g.level.breaker.pos;
            let p = pos_of(g, gd);
            let face = m::atan2(bp[0] - p[0], bp[2] - p[2]);
            crate::squad::order(g, gd, crate::squad::OrderPatch { face: Some(Some(face)), aim_at: Some(None), upper: Some(Some(crate::character::UpperMode::None)), ..Default::default() });
            if bo_permanent(g) && g.time - there_t > g.k.powerout.look_secs {
                st(g).word = true;
                crate::game::game_say(g, gd, "breaker's wrecked — that's them! Stay on your doors!", Some(true), None, None);
                event(g, EventWhat::Wrecked, Some(gd), "the breaker is shot to scrap: no fault, and nothing to mend");
                all_back(g);
            }
        } else if let Some(w) = man(g, i).watch {
            crate::squad::order(g, gd, crate::squad::OrderPatch { aim_at: Some(Some([w[0], 1.2, w[2]])), upper: Some(Some(crate::character::UpperMode::Aim)), ..Default::default() });
        }
        if role == ManRole::Cover && (lever.is_none() || man(g, lever.unwrap()).phase == Phase::Back) {
            man(g, i).phase = Phase::Back;
            man(g, i).tries = 0.0;
            walk(g, d, from);
        }
        return true;
    }
    if !reached {
        if g.tab.roomfight_def[d].stall_t <= 1.3 && g.tab.roomfight_def[d].move_t <= 8.0 {
            return true; // on his way
        }
        if man(g, i).tries < g.k.powerout.tries {
            // (the room fight ends a leg at ten seconds, or a second and a half of going nowhere: a longer way is walked in legs, a shut way tried again)
            man(g, i).tries += 1.0;
            walk(g, d, goal);
            return true;
        }
        if man(g, i).phase == Phase::Back {
            event(g, EventWhat::Abort, Some(gd), "no way back to his post: the room fight plants him where he is");
            return false;
        }
        man(g, i).phase = Phase::Back;
        man(g, i).tries = 0.0;
        walk(g, d, from);
        event(g, EventWhat::Abort, Some(gd), "no way to the breaker: back to his post");
        return true;
    }
    if man(g, i).phase == Phase::Back {
        let m_ = man(g, i).clone();
        settle(g, &m_, d);
        let why = alloc::format!("back on his post {} s after the lights went", to_fixed(g.time - st(g).since, 1));
        event(g, EventWhat::Home, Some(gd), &why);
        return false;
    }
    man(g, i).phase = Phase::There;
    let time = g.time;
    man(g, i).there_t = time;
    clear_arrived(g, gd);
    g.tab.roomfight_def[d].stall_t = 0.0;
    g.tab.roomfight_def[d].move_t = 0.0;
    let why = if role == ManRole::Lever {
        alloc::format!("at the lever {} s after the lights went", to_fixed(g.time - st(g).since, 1))
    } else {
        String::from("set short of the lever, watching the way they came")
    };
    event(g, EventWhat::There, Some(gd), &why);
    true
}

/// the beat is up: the party steps off as one, or does not. (powerout.ts stepOff)
pub fn step_off(g: &mut crate::game::Game, fight: bool) {
    let wait: Vec<usize> = (0..st(g).men.len()).filter(|&i| man(g, i).phase == Phase::Wait).collect();
    let all: Option<&str> = if !bo_active(g) {
        Some("the mains are back")
    } else if fight {
        Some("a fight has started")
    } else {
        None
    };
    // `why` is a Map keyed by the man in the TypeScript: here a list beside `wait`
    let mut why: Vec<Option<String>> = Vec::with_capacity(wait.len());
    for &i in &wait {
        let gd = man(g, i).gd;
        let d = def_of(g, gd);
        why.push(match (all, d) {
            (Some(a), _) => Some(String::from(a)),
            (None, None) => Some(String::from("no man of the fight's any more")),
            (None, Some(d)) => power_out_kept(g, d),
        });
    }
    let mut dropped = vec![false; st(g).men.len()];
    for (w, &i) in why.iter().zip(wait.iter()) {
        if let Some(w) = w {
            dropped[i] = true;
            let gd = man(g, i).gd;
            event(g, EventWhat::Abort, Some(gd), &alloc::format!("does not step off: {}", w));
        }
    }
    let mut at = 0;
    st(g).men.retain(|_| {
        let d = dropped[at];
        at += 1;
        !d
    });
    // what is left at 'wait' now is exactly who goes, in the same order
    let goes: Vec<usize> = (0..st(g).men.len()).filter(|&i| man(g, i).phase == Phase::Wait).collect();
    if goes.is_empty() {
        if st(g).men.is_empty() {
            let s = st(g);
            s.parties = m::max(0.0, s.parties - 1.0);
        }
        return;
    }
    if !st(g).men.iter().any(|x| x.role == ManRole::Lever) {
        let first = goes[0];
        man(g, first).role = ManRole::Lever;
        let spot = lever_spot(g);
        man(g, first).goal = spot;
        man(g, first).watch = None;
    }
    for &i in &goes {
        let gd = man(g, i).gd;
        let d = def_of(g, gd).unwrap();
        let calm = calm_of(g, d);
        man(g, i).calm = Some(calm);
        let heard = loud_heard(g, gd);
        man(g, i).heard0 = heard;
        man(g, i).phase = Phase::Out;
        let goal = man(g, i).goal;
        walk(g, d, goal);
        let how = if man(g, i).role == ManRole::Lever { "walks to the lever" } else { "goes with him to cover it" };
        let why = alloc::format!("{}, {} m off, {} s after the lights went", how, to_fixed(flat(pos_of(g, gd), goal), 0), to_fixed(g.time - st(g).since, 1));
        event(g, EventWhat::Go, Some(gd), &why);
    }
    let at = g.time + st(g).way_m / m::max(0.5, g.tune.guard_investigate) + g.k.powerout.overdue;
    st(g).overdue_at = at;
}

/// nobody is out any more: a party that lost a man and sent no word is missed once it is `overdue`. (powerout.ts missed)
pub fn missed(g: &mut crate::game::Game, defs: &[RoomfightDefId], fight: bool) {
    if st(g).overdue_at < 0.0 {
        return;
    }
    if !st(g).lost || st(g).word || !bo_active(g) {
        st(g).overdue_at = -1.0;
        return;
    }
    if g.time < st(g).overdue_at {
        return;
    }
    st(g).overdue_at = -1.0;
    st(g).word = true;
    let mut speaker: Option<GuardId> = None;
    for &d in defs {
        let gd = g.tab.roomfight_def[d].gd;
        if fit(g, gd) {
            speaker = Some(gd);
            break;
        }
    }
    let Some(speaker) = speaker else { return };
    if fight {
        return;
    }
    crate::game::game_say(g, speaker, "nothing from the breaker — that's them! Stay on your doors!", Some(true), None, None);
    event(g, EventWhat::Overdue, Some(speaker), "the men sent for the lights have not been heard from and the mains are still down");
}

/// the rule let go of in the middle of a walk: every man out goes back to his post at a run. (powerout.ts standDown)
pub fn stand_down(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < st(g).men.len() {
        let gd = man(g, i).gd;
        if let Some(d) = def_of(g, gd) {
            if g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Move && man(g, i).phase != Phase::Wait && alive(g, gd) && g.gd[gd].held.is_none() {
                let from = man(g, i).from;
                crate::pressure::run(g, d, from, None, None);
            }
        }
        i += 1;
    }
    let s = st(g);
    s.men = Vec::new();
    s.overdue_at = -1.0;
    s.fell = None;
}
