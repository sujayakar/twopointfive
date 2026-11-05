//! src/game/darkdoor.ts lines 1 to 386
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

type Pool = &'static [&'static str];
/// (kind, say, he, they), in the order the TypeScript's `LINES` writes them. A pool the TypeScript leaves out is empty here: `?? []` of it.   (darkdoor.ts LINES)
static LINES: [(crate::darkdoor::LineKind, Pool, Pool, Pool); 6] = [
    (LineKind::Call, &["I'm not going in there alone. get up here.", "not in there. not on my own — get up here!"], &["he could be anywhere in that. somebody get up here."], &["they could be anywhere in that. somebody get up here."]),
    (LineKind::Self_, &["no. not in there. not on my own.", "pitch black. no. not alone.", "not a chance. not by myself."], &[], &[]),
    (LineKind::Copy, &["copy. coming.", "on my way. stay put.", "hold there — coming to you."], &[], &[]),
    (LineKind::With, &["on you. get a light in there.", "I'm here. lights on — go.", "right behind you. go on."], &[], &[]),
    (LineKind::Nobody, &["nobody's coming.", "no one. all right — leave it.", "nobody. forget that room."], &[], &[]),
    (LineKind::Alone, &["fine. fine — I'll do it myself.", "nobody ever comes. going in.", "all right. slow. going in."], &[], &[]),
];

// ---- the seam: the one step the skeleton still cannot do (notes.md, ## Skeleton)
/// `g.audio.play(name, at, gain, { rate })` (darkdoor.ts:102): `game_audio_like_play`'s opts hold `crate::Opaque`, which cannot carry a rate
fn seam_audio_play(g: &mut Game, name: &str, at: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts able to hold rate") }

// ---- the private helpers
/// a new row   (darkdoor.ts:134, the literal in mind)
fn fresh_fear() -> crate::darkdoor::Fear {
    crate::darkdoor::Fear { act: DarkDoorAct::None, t: 0.0, door: -1.0, side: 1.0, spot: None, at: false, other: None, called: false, call_t: 0.0, tried: Vec::new(), left: -1e9, told: false, lag: 0.0, armed: -1.0, arm_t: -1e9, quits: Vec::new(), pass: Vec::new(), said: 0.0, idle: 0.0, ask: 0.0 }
}
fn pos_of(g: &mut Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
/// `g.doors.list[i]`: a door's index in the level's list, -1 being none
fn door_at(g: &mut Game, door: f64) -> Option<DoorId> {
    if door < 0.0 { return None; }
    g.doors.list.get(door as usize).copied()
}
/// `g.doors.list.indexOf(d)`
fn door_index(g: &mut Game, d: DoorId) -> f64 {
    match g.doors.list.iter().position(|&x| x == d) { Some(i) => i as f64, None => -1.0 }
}
fn lit_r(r: crate::darkdoor::DarkRoom) -> crate::darkdoor::StandsLitR {
    crate::darkdoor::StandsLitR { t: r.t, dark: r.dark, share: r.share, mean: r.mean }
}

/// this file's lines: every kind's, or one kind's.   (darkdoor.ts darkDoorLines)
pub fn dark_door_lines(kind: Option<crate::darkdoor::LineKind>) -> Vec<String> {
    let mut out = Vec::new();
    for &(k, say, he, they) in LINES.iter() {
        if let Some(only) = kind { if only != k { continue; } }
        for l in say.iter().chain(he.iter()).chain(they.iter()) { out.push(String::from(*l)); }
    }
    out
}

/// one line of a kind from this man, the next of its pool by his own count: spoken, or on the net with the handset's chirp.   (darkdoor.ts bark)
pub fn bark(g: &mut crate::game::Game, gd: GuardId, kind: crate::darkdoor::LineKind, radio: bool) {
    let (_, say, he, they) = *LINES.iter().find(|l| l.0 == kind).unwrap();
    let extra: Pool = if g.gd[gd].team != crate::teams::Team::Guard { crate::words::w(g, &they, &he) } else { he };
    let mut pool: Vec<&'static str> = Vec::new();
    pool.extend_from_slice(say);
    pool.extend_from_slice(extra);
    let c = g.gd[gd].ch;
    let s = fear(g, gd);
    let said = s.said;
    s.said += 1.0;
    let i = js::rem(g.ch[c].id + said, pool.len() as f64) as usize;
    crate::game::game_say(g, gd, pool[i], Some(radio), None, None);
    if radio {
        let at = head_of(g, c);
        seam_audio_play(g, "radio", at, 0.5, 1.05);
    }
}

/// distance on the floor.   (darkdoor.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// his chest, or where it would be.   (darkdoor.ts chestOf)
pub fn chest_of(g: &mut crate::game::Game, c_2: CharId) -> V3 {
    if let Some(b) = g.ch[c_2].bones.get(crate::character::Bone::Chest) { return *b; }
    let p = g.ch[c_2].pos;
    [p[0], p[1] + 1.2, p[2]]
}

/// his head, or where it would be.   (darkdoor.ts headOf)
pub fn head_of(g: &mut crate::game::Game, c_2: CharId) -> V3 {
    if let Some(b) = g.ch[c_2].bones.get(crate::character::Bone::Head) { return *b; }
    let p = g.ch[c_2].pos;
    [p[0], p[1] + 1.6, p[2]]
}

/// a man this file is about at all: a living defender or a man of the night staff.   (darkdoor.ts ours)
pub fn ours(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let x = &g.gd[gd];
    x.state != crate::game::GuardState::Dead && x.civ.is_none() && x.held.is_none() && (x.team == crate::teams::Team::Hostile || x.team == crate::teams::Team::Guard)
}

/// one of the hostage rescue's cast.   (darkdoor.ts rescued)
pub fn rescued(g: &mut crate::game::Game, gd: GuardId) -> bool { g.rescue.active && g.rescue.hostiles.contains(&gd) }

/// he knows something is out there.   (darkdoor.ts alarmed)
pub fn alarmed(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let st = g.gd[gd].state;
    st == crate::game::GuardState::Suspicious || st == crate::game::GuardState::Search || st == crate::game::GuardState::Alert
}

/// nobody's but his own.   (darkdoor.ts own)
pub fn own(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let x = &g.gd[gd];
    x.script.is_none() && !x.hold && !x.pinned && x.standoff.is_none() && x.task.is_none() && x.body_duty.is_none() && !rescued(g, gd) && !crate::morale::morale_owns(g, gd) && !crate::netquietfeed::net_feed_holds(g, gd)
}

/// may the rule have him at all this frame.   (darkdoor.ts able)
pub fn able(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !g.ai_enabled || g.touring || g.quiet_utility || !ours(g, gd) || !alarmed(g, gd) { return false; }
    !g.gd[gd].trained && own(g, gd)
}

/// in a fight, what his walk this frame is.   (darkdoor.ts fightWalk)
pub fn fight_walk(g: &mut crate::game::Game, gd: GuardId) -> crate::darkdoor::FightWalk {
    if crate::teams::foe_down(g, gd) { return FightWalk::Other; }
    if g.time - g.gd[gd].last_seen_t < g.k.darkdoor.seen_secs { return FightWalk::Seen; }
    match (g.gd[gd].path_goal, g.gd[gd].last_known) {
        (Some(a), Some(b)) if flat(a, b) < 0.8 => FightWalk::Chase,
        _ => FightWalk::Other,
    }
}

/// free to come to a mate, or to go in with one.   (darkdoor.ts canCome)
pub fn can_come(g: &mut crate::game::Game, o: GuardId) -> bool {
    if !ours(g, o) { return false; }
    let st = g.gd[o].state;
    if (st != crate::game::GuardState::Patrol && st != crate::game::GuardState::Suspicious) || !own(g, o) || g.gd[o].leader.is_some() || g.gd[o].post.is_some() || crate::needs::needs_busy(g, o) { return false; }
    match g.m.darkdoor.state_of.get(o) { None => true, Some(so) => so.act == DarkDoorAct::None }
}

/// his row, made if he has none.   (darkdoor.ts mind; the skeleton's signature hands nothing back: `fear` below is the row)
pub fn mind(g: &mut crate::game::Game, gd: GuardId) {
    g.m.darkdoor.state_of.get_or_insert_with(gd, fresh_fear);
}
/// `mind`'s answer in the TypeScript: his row itself, made if he has none. Use it for a few lines and let it go before a call that takes `g`.
fn fear<'g>(g: &'g mut Game, gd: GuardId) -> &'g mut crate::darkdoor::Fear {
    g.m.darkdoor.state_of.get_or_insert_with(gd, fresh_fear)
}

/// is he green: morale's own answer unless `who` overrides.   (darkdoor.ts greenMan)
pub fn green_man(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let who = g.k.darkdoor.who;
    if who != crate::knobs::DarkdoorKWho::Rest { return who == crate::knobs::DarkdoorKWho::All; }
    crate::morale::training_of(g, gd) == crate::morale::TrainingOf::Green
}

/// the nearest living man of his side inside `pairM` with nothing solid between his eyes and that man's chest.   (darkdoor.ts mateBy)
pub fn mate_by(g: &mut crate::game::Game, gd: GuardId, fit: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId) -> bool>) -> Option<GuardId> {
    let mut fit = fit;
    let c = g.gd[gd].ch;
    let eye = head_of(g, c);
    let mut best = None;
    let mut bd = g.k.darkdoor.pair_m;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || !ours(g, o) || g.gd[o].team != g.gd[gd].team { continue; }
        if let Some(f) = fit.as_mut() { if !f(g, o) { continue; } }
        if crate::runner::runner_nobody(g, o) { continue; }
        let d = flat(pos_of(g, o), g.ch[c].pos);
        if d <= bd {
            let oc = g.gd[o].ch;
            let chest = chest_of(g, oc);
            if !crate::scene_collision::segment_blocked(g, eye, chest, None, None, None) { bd = d; best = Some(o); }
        }
    }
    best
}

/// the man to call: of the `mateTry` nearest free to come and not yet dropped, the shortest walk to him inside `mateM`.   (darkdoor.ts freeMate)
pub fn free_mate(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let k = g.k.darkdoor;
    let p = pos_of(g, gd);
    let tried = fear(g, gd).tried.clone();
    let mut near: Vec<GuardId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o != gd && g.gd[o].team == g.gd[gd].team && can_come(g, o) && !tried.contains(&o) && flat(pos_of(g, o), p) <= k.mate_m { near.push(o); }
    }
    js::sort_by(&mut near, |a, b| flat(g.ch[g.gd[*a].ch].pos, p) - flat(g.ch[g.gd[*b].ch].pos, p));
    near.truncate(m::max(1.0, m::round(k.mate_try)) as usize);
    let mut best = None;
    let mut bl = k.mate_m;
    for &o in near.iter() {
        let from = pos_of(g, o);
        let Some(path) = crate::scene_collision::nav_grid_find_path(g, from, p, None, None) else { continue };
        let mut l = 0.0;
        let mut a = from;
        for &b in path.iter() { l += flat(a, b); a = b; }
        if l <= bl { bl = l; best = Some(o); }
    }
    best
}

/// the level's rooms as lists of floor cells, dealt once a game (a game has one nav grid).   (darkdoor.ts roomCells)
pub fn room_cells(g: &mut crate::game::Game, room: f64) -> Vec<f64> {
    if g.m.darkdoor.cells_of.is_none() {
        g.m.darkdoor.cells_of = Some(JsMap::new());
        let (cell, w, h) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
        let mut z = 0.0;
        while z < h {
            let mut x = 0.0;
            while x < w {
                let px = (x + 0.5) * cell;
                let pz = (z + 0.5) * cell;
                let r = crate::cleared::cleared_rooms_room_at(g, [px, 0.0, pz]);
                if js::truthy(r) {
                    let l = g.m.darkdoor.cells_of.as_mut().unwrap().get_or_insert_with(crate::Num(r), Vec::new);
                    l.push(px);
                    l.push(pz);
                }
                x += 1.0;
            }
            z += 1.0;
        }
    }
    match g.m.darkdoor.cells_of.as_ref().unwrap().get(crate::Num(room)) { Some(l) => l.clone(), None => Vec::new() }
}

/// is the room dark: `samples` of its floor cells on the meter, kept `askSecs`. Open floor (room 0) is never dark.   (darkdoor.ts darkRoom)
pub fn dark_room(g: &mut crate::game::Game, room: f64) -> crate::darkdoor::DarkRoom {
    let k = g.k.darkdoor;
    let now_t = g.time;
    if !js::truthy(room) { return crate::darkdoor::DarkRoom { t: now_t, dark: false, share: 0.0, mean: 1.0 }; }
    let r = g.m.darkdoor.reads_of.get_or_insert_with(JsMap::new);
    if let Some(was) = r.get(crate::Num(room)) {
        if now_t >= was.t && now_t - was.t < k.ask_secs { return crate::darkdoor::DarkRoom { t: was.t, dark: was.dark, share: was.share, mean: was.mean }; }
    }
    let l = room_cells(g, room);
    let n = (l.len() / 2) as f64;
    let take = m::max(1.0, m::min(n, m::round(k.samples)));
    let day = crate::hours::hour_of(g) != crate::hours::HourId::Night;
    let mut under = 0.0;
    let mut sum = 0.0;
    let mut i = 0.0;
    while i < take && n > 0.0 {
        let j = m::min(n - 1.0, m::floor((i + 0.5) * n / take));
        let x = l[(j * 2.0) as usize];
        let z = l[(j * 2.0 + 1.0) as usize];
        let mut v = crate::calmtorch::place_light(g, x, z);
        if v < k.dark_at && day && crate::calmtorch::work_lit(g, x, z, Some(k.dark_at)) { v = k.dark_at; }
        sum += v;
        if v < k.dark_at { under += 1.0; }
        i += 1.0;
    }
    let share = if n > 0.0 { under / take } else { 0.0 };
    let now = crate::darkdoor::Read { t: g.time, dark: n > 0.0 && share >= k.dark_share, share, mean: if n > 0.0 { sum / take } else { 1.0 } };
    g.m.darkdoor.reads_of.get_or_insert_with(JsMap::new).set(crate::Num(room), now);
    g.stats.darkdoor.asks += 1;
    crate::darkdoor::DarkRoom { t: now.t, dark: now.dark, share: now.share, mean: now.mean }
}

/// does he stand in the light, against the room ahead.   (darkdoor.ts standsLit)
pub fn stands_lit(g: &mut crate::game::Game, gd: GuardId, r: crate::darkdoor::StandsLitR) -> bool {
    let k = g.k.darkdoor;
    let p = pos_of(g, gd);
    let here = crate::calmtorch::place_light(g, p[0], p[2]);
    here >= k.dark_at || here >= r.mean + k.lit_by || (crate::hours::hour_of(g) != crate::hours::HourId::Night && crate::calmtorch::work_lit(g, p[0], p[2], Some(k.dark_at)))
}

/// an open doorway: the leaf off its hinges, or standing `openRad` off its frame.   (darkdoor.ts open)
pub fn open(g: &mut crate::game::Game, d: DoorId) -> bool {
    crate::doors::door_off(g, d) || (!crate::doors::door_latched(g, d) && m::abs(crate::doors::door_angle(g, d)) >= g.k.darkdoor.open_rad)
}

/// the next doorway on his walk inside `reach` metres of path.   (darkdoor.ts doorAhead)
pub fn door_ahead(g: &mut crate::game::Game, gd: GuardId, reach: f64) -> Option<crate::darkdoor::DoorAhead> {
    let mut run = 0.0;
    let mut a = pos_of(g, gd);
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() && run < reach {
        let b = g.gd[gd].path[i];
        let mut hit: Option<(DoorId, crate::guards::CrossesDoor)> = None;
        let mut q = 0;
        while q < g.doors.list.len() {
            let d = g.doors.list[q];
            q += 1;
            if let Some(x) = crate::guards::crosses_door(g, a, b, d) {
                let nearer = match &hit { None => true, Some(h) => x.t < h.1.t };
                if nearer { hit = Some((d, x)); }
            }
        }
        if let Some((d, x)) = hit {
            let dist = run + flat(a, x.at);
            return if dist <= reach { Some(crate::darkdoor::DoorAhead { d, side: x.side, at: x.at, dist }) } else { None };
        }
        run += flat(a, b);
        a = b;
        i += 1;
    }
    None
}

/// where he waits: short of the frame, off the middle of the opening; None = nowhere but where he stands.   (darkdoor.ts standSpot)
pub fn stand_spot(g: &mut crate::game::Game, gd: GuardId, d: DoorId, side: f64) -> Option<V3> {
    let k = g.k.darkdoor;
    let f = crate::guards::frame_of(g, d);
    let p = pos_of(g, gd);
    let along = (p[0] - f.hx) * f.ux + (p[2] - f.hz) * f.uz;
    let s = if along <= f.w / 2.0 { -1.0 } else { 1.0 };
    for off in [k.stand_m, k.stand_m + 0.5] {
        for a in [f.w / 2.0 + s * k.side_m, f.w / 2.0 - s * k.side_m] {
            let x = f.hx + f.ux * a + f.nx * side * off;
            let z = f.hz + f.uz * a + f.nz * side * off;
            if crate::scene_collision::nav_grid_is_blocked(g, x, z) || crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [x, 0.45, z], None) { continue; }
            let mut taken = false;
            let mut i = 0;
            while i < g.guards.len() {
                let o = g.guards[i];
                i += 1;
                if o != gd && g.gd[o].state != crate::game::GuardState::Dead && flat(pos_of(g, o), [x, 0.0, z]) < 0.6 { taken = true; break; }
            }
            if taken { continue; }
            return Some([x, 0.0, z]);
        }
    }
    None
}

/// is this door one that will not stop him.   (darkdoor.ts passes)
pub fn passes(s: &crate::darkdoor::Fear, door: f64, t: f64) -> bool {
    s.pass.iter().any(|p| p.door == door && t < p.until)
}

/// this door will not stop him for `againSecs`.   (darkdoor.ts pass)
pub fn pass(g: &mut crate::game::Game, gd: GuardId, door: f64) {
    let now = g.time;
    let until = now + g.k.darkdoor.again_secs;
    let s = fear(g, gd);
    s.pass.retain(|p| p.door != door && now < p.until);
    s.pass.push(crate::darkdoor::FearPassItem { door, until });
}

/// a mate is let go where he is.   (darkdoor.ts drop)
pub fn drop(g: &mut crate::game::Game, gd: GuardId) {
    let s = fear(g, gd);
    s.act = DarkDoorAct::None;
    s.t = 0.0;
    s.other = None;
    s.told = false;
    s.spot = None;
    s.lag = 0.0;
}

/// the act is over: he is his own man again, and a mate still on his way to him with him.   (darkdoor.ts end)
pub fn end(g: &mut crate::game::Game, gd: GuardId, unresolved: Option<bool>) {
    let unresolved = unresolved.unwrap_or(false);
    if let Some(o) = fear(g, gd).other {
        let hit = match g.m.darkdoor.state_of.get(o) { Some(so) => so.other == Some(gd) && so.act == DarkDoorAct::Come, None => false };
        if hit { drop(g, o); }
    }
    let now = g.time;
    let s = fear(g, gd);
    s.left = if unresolved && s.act == DarkDoorAct::Balk { now } else { -1e9 };
    s.act = DarkDoorAct::None;
    s.spot = None;
    s.at = false;
    s.other = None;
    if !unresolved {
        s.t = 0.0;
        s.called = false;
        s.tried = Vec::new();
    }
}

/// a mate is told to come: the floor's own walk to a fix, the fix on the man who called.   (darkdoor.ts call)
pub fn call(g: &mut crate::game::Game, gd: GuardId, m: GuardId) {
    let mp = pos_of(g, m);
    let row = fear(g, m);
    row.act = DarkDoorAct::Come;
    row.t = 0.0;
    row.other = Some(gd);
    row.told = false;
    row.spot = Some(v3::copy(mp));
    row.lag = 0.0;
    fear(g, gd).other = Some(m);
    g.stats.darkdoor.calls += 1;
    let here = pos_of(g, gd);
    let x = &mut g.gd[m];
    x.last_known = Some(v3::copy(here));
    x.awareness = crate::m::max(x.awareness, 0.42);
    if x.state != crate::game::GuardState::Suspicious {
        x.state = crate::game::GuardState::Suspicious;
        x.react_t = 0.8;
        x.search_plan = None;
        x.pieing = None;
    }
    x.search_t = 0.0;
    x.path = Vec::new();
    x.path_goal = None;
    x.stall_ref = None;
    let s = fear(g, gd);
    if !s.called {
        s.called = true;
        let radio = flat(pos_of(g, m), pos_of(g, gd)) > g.k.darkdoor.ear_m;
        bark(g, gd, LineKind::Call, radio);
    }
}

/// the walk's seam: the pace he walks at this frame, or None: this file has his feet.   (darkdoor.ts darkDoorGate)
pub fn dark_door_gate(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> Option<f64> {
    let k = g.k.darkdoor;
    let act0 = g.m.darkdoor.state_of.get(gd).map(|s| s.act);
    if act0 == Some(DarkDoorAct::Come) || act0 == Some(DarkDoorAct::With) { return Some(speed); }
    if !able(g, gd) { return Some(speed); }
    let x = if act0 == Some(DarkDoorAct::Balk) { None } else { door_ahead(g, gd, k.stop_m) };
    let di = match &x { Some(x) => door_index(g, x.d), None => -1.0 };
    if act0 == Some(DarkDoorAct::Solo) {
        let door = fear(g, gd).door;
        if x.is_none() || di != door {
            pass(g, gd, door);
            end(g, gd, None);
            return Some(speed);
        }
        fear(g, gd).idle = 0.0;
        if g.gd[gd].state == crate::game::GuardState::Suspicious { g.gd[gd].awareness = m::max(g.gd[gd].awareness, k.keep_aw); }
        let s = fear(g, gd);
        s.t += dt;
        if s.t >= k.nerve_secs { return Some(speed * k.slow_k); }
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
        let c = g.gd[gd].ch;
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.gd[gd].stall_ref = None;
        g.gd[gd].alert_stall_t = 0.0;
        g.gd[gd].chase_t = 0.0;
        return None;
    }
    let fight = if g.gd[gd].state == crate::game::GuardState::Alert { fight_walk(g, gd) } else { FightWalk::Chase };
    if fight == FightWalk::Other { return Some(speed); }
    if act0 == Some(DarkDoorAct::Balk) { return hold(g, gd, dt, speed, fight == FightWalk::Seen); }
    let Some(x) = x else { return Some(speed) };
    if fight == FightWalk::Seen || crate::sim_lightshome::mains_active(g) { return Some(speed); }
    let now = g.time;
    if let Some(s0) = g.m.darkdoor.state_of.get(gd) { if passes(s0, di, now) { return Some(speed); } }
    let room = crate::cleared::cleared_rooms_room_by(g, x.d, -x.side);
    if !js::truthy(room) { return Some(speed); }
    let here = pos_of(g, gd);
    if room == crate::cleared::cleared_rooms_room_at(g, here) { return Some(speed); }
    if !green_man(g, gd) || mate_by(g, gd, None).is_some() { return Some(speed); }
    let r = dark_room(g, room);
    if !r.dark { return Some(speed); }
    if !open(g, x.d) {
        if stands_lit(g, gd, lit_r(r)) {
            let now = g.time;
            let a = fear(g, gd);
            a.armed = di;
            a.arm_t = now;
        }
        return Some(speed);
    }
    let now = g.time;
    let owed = match g.m.darkdoor.state_of.get(gd) { Some(s0) => s0.armed == di && now - s0.arm_t < k.arm_secs, None => false };
    if !owed && !stands_lit(g, gd, lit_r(r)) { return Some(speed); }
    let now = g.time;
    let s = fear(g, gd);
    let again = s.door == di && now - s.left < k.resume_secs;
    s.quits.retain(|q| now - q.t < k.fed_up_secs);
    s.door = di;
    s.side = x.side;
    s.idle = 0.0;
    s.ask = 0.0;
    s.call_t = 0.0;
    s.other = None;
    s.left = -1e9;
    s.armed = -1.0;
    if !again {
        s.t = 0.0;
        s.called = false;
        s.tried = Vec::new();
        g.stats.darkdoor.balks += 1;
    }
    let s = fear(g, gd);
    if !again && (s.quits.iter().filter(|q| q.room == room).count() as f64) >= k.fed_up_n - 1.0 {
        s.act = DarkDoorAct::Solo;
        bark(g, gd, LineKind::Alone, false);
        g.stats.darkdoor.solos += 1;
        return dark_door_gate(g, gd, 0.0, speed);
    }
    s.act = DarkDoorAct::Balk;
    let spot = stand_spot(g, gd, x.d, x.side);
    let s = fear(g, gd);
    s.spot = spot;
    s.at = spot.is_none();
    if !again {
        match free_mate(g, gd) {
            Some(m) => call(g, gd, m),
            None => bark(g, gd, LineKind::Self_, false),
        }
    }
    hold(g, gd, 0.0, speed, false)
}

/// one frame at the frame: is the wait over, else his feet. `seen` = a foe in his eyes: the wait's clock stands.   (darkdoor.ts hold)
pub fn hold(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64, seen: bool) -> Option<f64> {
    let k = g.k.darkdoor;
    let c = g.gd[gd].ch;
    let (door, side) = { let s = fear(g, gd); (s.door, s.side) };
    let Some(d) = door_at(g, door) else { return Some(speed) };
    let Some(x) = door_ahead(g, gd, k.stop_m + 4.0) else { return Some(speed) };
    if x.d != d || x.side != side { return Some(speed); }
    let room = crate::cleared::cleared_rooms_room_by(g, d, -side);
    let s = fear(g, gd);
    s.idle = 0.0;
    if !seen { s.t += dt; }
    s.ask += dt;
    s.call_t += dt;
    if s.ask >= k.ask_secs {
        s.ask = 0.0;
        if crate::sim_lightshome::mains_active(g) || !dark_room(g, room).dark {
            end(g, gd, None);
            return Some(speed);
        }
        let other = fear(g, gd).other;
        let mut fit = |g: &mut Game, o: GuardId| Some(o) == other || can_come(g, o);
        let mate = mate_by(g, gd, Some(&mut fit));
        if let Some(mt) = mate {
            mind(g, mt);
            if let Some(o) = other {
                if o != mt {
                    let hit = match g.m.darkdoor.state_of.get(o) { Some(so) => so.other == Some(gd), None => false };
                    if hit { drop(g, o); }
                }
            }
            let at = v3::copy(pos_of(g, mt));
            let row = fear(g, mt);
            row.act = DarkDoorAct::With;
            row.t = 0.0;
            row.other = Some(gd);
            row.spot = Some(at);
            row.lag = 0.0;
            bark(g, mt, LineKind::With, false);
            pass(g, gd, door);
            let s = fear(g, gd);
            s.act = DarkDoorAct::None;
            s.t = 0.0;
            s.spot = None;
            s.at = false;
            s.other = Some(mt);
            s.called = false;
            s.tried = Vec::new();
            g.stats.darkdoor.pairs += 1;
            return Some(speed);
        }
    }
    let s = fear(g, gd);
    if s.other.is_none() && s.call_t >= k.call_secs && s.t < k.wait_secs {
        s.call_t = 0.0;
        if let Some(f) = free_mate(g, gd) { call(g, gd, f); }
    }
    if fear(g, gd).t >= k.wait_secs {
        bark(g, gd, LineKind::Nobody, false);
        let now = g.time;
        fear(g, gd).quits.push(crate::darkdoor::FearQuitsItem { room, t: now });
        end(g, gd, None);
        g.stats.darkdoor.quits += 1;
        let id = g.ch[c].id;
        let n = match &g.gd[gd].search_plan { Some(p) => p.spots.len(), None => 0 };
        for i in 0..n {
            let at = g.gd[gd].search_plan.as_ref().unwrap().spots[i].pos;
            if crate::cleared::cleared_rooms_room_at(g, at) == room {
                let spot = &mut g.gd[gd].search_plan.as_mut().unwrap().spots[i];
                spot.unreachable = true;
                spot.by = id;
            }
        }
        if g.gd[gd].state == crate::game::GuardState::Alert {
            crate::guards::drop_to_search(g, gd, None);
            g.gd[gd].last_known = Some(v3::copy(g.ch[c].pos));
        } else {
            g.gd[gd].stall_ref = Some(v3::copy(g.ch[c].pos));
            g.gd[gd].stall_t = 1e3;
        }
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        return None;
    }
    g.gd[gd].stall_ref = None;
    g.gd[gd].alert_stall_t = 0.0;
    g.gd[gd].chase_t = 0.0;
    if g.gd[gd].state == crate::game::GuardState::Suspicious {
        g.gd[gd].awareness = m::max(g.gd[gd].awareness, k.keep_aw);
    } else if g.gd[gd].state == crate::game::GuardState::Search {
        g.gd[gd].search_t = m::max(0.0, g.gd[gd].search_t - dt);
    }
    let p = g.ch[c].pos;
    let (spot, at, t) = { let s = fear(g, gd); (s.spot, s.at, s.t) };
    let d_s = match spot { Some(to) => flat(p, to), None => 0.0 };
    let face = v3::yaw_to(p, g.dr[d].frame_centre);
    let stepping = match spot { Some(to) if !at && d_s > 0.15 && t < 4.0 => Some(to), _ => None };
    if let Some(to) = stepping {
        let yaw = v3::yaw_to(p, to);
        let back = d_s < 1.6 && m::cos(yaw - face) < 0.3;
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, if back { face } else { yaw }, 5.5 * dt);
        let want = m::min(speed, g.tune.guard_investigate) * v3::clamp(d_s / 0.8, 0.35, 1.0) * (if back { 0.6 } else { v3::clamp(m::cos(yaw - g.ch[c].body_yaw) * 1.5, 0.15, 1.0) });
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, want, 4.0, dt);
        let sp = g.gd[gd].speed;
        g.ch[c].vel = [m::sin(yaw) * sp, 0.0, m::cos(yaw) * sp];
        g.ch[c].pos = v3::mad(g.ch[c].pos, g.ch[c].vel, dt);
    } else {
        fear(g, gd).at = true;
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, face, 3.0 * dt);
    }
    None
}

/// the eyes' seam: where his aim goes this frame, None = his own.   (darkdoor.ts darkDoorLook)
pub fn dark_door_look(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> {
    let (act, door, t) = match g.m.darkdoor.state_of.get(gd) { Some(s) => (s.act, s.door, s.t), None => return None };
    if (act != DarkDoorAct::Balk && act != DarkDoorAct::Solo) || crate::netquietfeed::net_feed_holds(g, gd) { return None; }
    let d = door_at(g, door)?;
    let sweep = g.k.darkdoor.sweep_secs;
    let f = crate::guards::frame_of(g, d);
    let a = if act == DarkDoorAct::Solo { f.w / 2.0 } else { f.w * (0.5 - 0.5 * m::cos(t * m::PI / m::max(0.2, sweep))) };
    Some([f.hx + f.ux * a, 1.1, f.hz + f.uz * a])
}

/// one man's frame of the tick, his act not none.   (darkdoor.ts darkDoorTick, the loop's body past `S.act === 'none'`)
fn tick_man(g: &mut Game, gd: GuardId, dt: f64) {
    let k = g.k.darkdoor;
    let act = fear(g, gd).act;
    if !g.ai_enabled {
        if act == DarkDoorAct::Come || act == DarkDoorAct::With { lose(g, gd); } else { end(g, gd, None); }
        return;
    }
    if act == DarkDoorAct::Balk || act == DarkDoorAct::Solo {
        let s = fear(g, gd);
        s.idle += dt;
        if s.idle > k.keep_secs || crate::netquietfeed::net_feed_holds(g, gd) { end(g, gd, Some(true)); }
        return;
    }
    let s = fear(g, gd);
    let o = s.other;
    s.t += dt;
    let goal = match o {
        None => None,
        Some(o) => {
            let at = pos_of(g, o);
            Some(if act == DarkDoorAct::Come { at } else { g.gd[o].path_goal.or(g.gd[o].last_known).unwrap_or(at) })
        }
    };
    let here = pos_of(g, gd);
    let d_g = match goal { Some(a) => flat(here, a), None => 0.0 };
    let s = fear(g, gd);
    match s.spot {
        Some(sp) if !(flat(here, sp) > 0.5) => s.lag += dt,
        _ => { s.spot = Some(v3::copy(here)); s.lag = 0.0; }
    }
    let lag = s.lag;
    let (Some(o), Some(goal)) = (o, goal) else { lose(g, gd); return };
    let st = g.gd[gd].state;
    if !ours(g, o) || st == crate::game::GuardState::Alert || st == crate::game::GuardState::Search || !own(g, gd) || lag > k.lag_secs {
        lose(g, gd);
        return;
    }
    if act == DarkDoorAct::Come {
        let waiting = match g.m.darkdoor.state_of.get(o) { Some(so) => so.act == DarkDoorAct::Balk && so.other == Some(gd), None => false };
        if !waiting { drop(g, gd); return; }
        let s = fear(g, gd);
        if !s.told && s.t >= k.copy_secs {
            s.told = true;
            bark(g, gd, LineKind::Copy, d_g > k.ear_m);
        }
    } else if fear(g, gd).t >= k.with_secs || !alarmed(g, o) || d_g < 1.5 {
        drop(g, gd);
        return;
    }
    let far = match g.gd[gd].last_known { Some(l) => flat(l, goal) > 0.5, None => true };
    if far { g.gd[gd].last_known = Some(v3::copy(goal)); }
    let x = &mut g.gd[gd];
    if x.state != crate::game::GuardState::Suspicious {
        x.state = crate::game::GuardState::Suspicious;
        x.search_plan = None;
        x.pieing = None;
        x.path = Vec::new();
        x.path_goal = None;
    }
    x.awareness = m::max(x.awareness, 0.42);
    x.search_t = 0.0;
}

/// once a frame: the men called and the men going in with somebody kept on their man, or dropped; a stopped man nothing asks for is let go.   (darkdoor.ts darkDoorTick)
pub fn dark_door_tick(g: &mut crate::game::Game, dt: f64) {
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !ours(g, gd) {
            let Some(act) = g.m.darkdoor.state_of.get(gd).map(|s| s.act) else { continue };
            if act != DarkDoorAct::None {
                if act == DarkDoorAct::Come || act == DarkDoorAct::With { lose(g, gd); } else { end(g, gd, None); }
            }
            continue;
        }
        if fear(g, gd).act == DarkDoorAct::None { continue; }
        tick_man(g, gd, dt);
    }
}

/// a mate is lost on the way: dropped, and the man who called him calls the next.   (darkdoor.ts lose)
pub fn lose(g: &mut crate::game::Game, gd: GuardId) {
    let call_secs = g.k.darkdoor.call_secs;
    let s = fear(g, gd);
    let (other, act) = (s.other, s.act);
    if let Some(o) = other {
        if let Some(so) = g.m.darkdoor.state_of.get_mut(o) {
            if so.other == Some(gd) {
                so.other = None;
                if so.act == DarkDoorAct::Balk && act == DarkDoorAct::Come {
                    so.tried.push(gd);
                    so.call_t = call_secs;
                }
            }
        }
    }
    drop(g, gd);
    g.stats.darkdoor.drops += 1;
}

/// QA: what the rule has of a man (None = never met).   (darkdoor.ts darkDoorOf)
pub fn dark_door_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::darkdoor::DarkDoorOf> {
    let s = g.m.darkdoor.state_of.get(gd)?.clone();
    Some(crate::darkdoor::DarkDoorOf { act: s.act, t: s.t, door: s.door, side: s.side, spot: s.spot, at: s.at, other: s.other, called: s.called, call_t: s.call_t, tried: s.tried, left: s.left, told: s.told, lag: s.lag, armed: s.armed, arm_t: s.arm_t, quits: s.quits, pass: s.pass, said: s.said, idle: s.idle, ask: s.ask })
}

/// QA: is he green as the rule deals it.   (darkdoor.ts darkDoorGreen)
pub fn dark_door_green(g: &mut crate::game::Game, gd: GuardId) -> bool { green_man(g, gd) }

/// the debug panel's line: every man the rule has.   (darkdoor.ts darkDoorSummary)
pub fn dark_door_summary(g: &mut crate::game::Game) -> String {
    let mut men: Vec<String> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let Some((act, t, door, other)) = g.m.darkdoor.state_of.get(gd).map(|s| (s.act, s.t, s.door, s.other)) else { continue };
        if act == DarkDoorAct::None { continue; }
        let mut line = alloc::format!("{} {}", g.gd[gd].callsign, act.as_str());
        if act == DarkDoorAct::Balk || act == DarkDoorAct::Solo {
            let name = match door_at(g, door) { Some(d) => g.dr[d].def.name.clone(), None => String::from("?") };
            line.push_str(&alloc::format!(" {:.1} s at {}", t, name));
        }
        if let Some(o) = other {
            let who = &g.gd[o].callsign;
            line.push_str(&if act == DarkDoorAct::Balk { alloc::format!(" ({} coming)", who) } else { alloc::format!(" (with {})", who) });
        }
        men.push(line);
    }
    let body = if men.is_empty() { String::from("nobody stopped") } else { men.join(" · ") };
    alloc::format!("dark door ▸ {}", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_come_in_the_order_written() {
        let all = dark_door_lines(None);
        assert_eq!(all.len(), 4 + 3 + 3 + 3 + 3 + 3);
        assert_eq!(all[0], "I'm not going in there alone. get up here.");
        assert_eq!(all[3], "they could be anywhere in that. somebody get up here.");
        assert_eq!(all[all.len() - 1], "all right. slow. going in.");
        let copy = dark_door_lines(Some(LineKind::Copy));
        assert_eq!(copy.len(), 3);
        assert_eq!(copy[2], "hold there — coming to you.");
    }

    #[test]
    fn a_pass_holds_for_its_door_until_its_time() {
        let mut s = fresh_fear();
        s.pass.push(crate::darkdoor::FearPassItem { door: 2.0, until: 10.0 });
        assert!(passes(&s, 2.0, 9.0));
        assert!(!passes(&s, 2.0, 10.0));
        assert!(!passes(&s, 3.0, 9.0));
    }

    #[test]
    fn a_new_row_is_the_typescripts_literal_not_blank() {
        let s = fresh_fear();
        assert_eq!((s.door, s.side, s.left, s.armed, s.arm_t), (-1.0, 1.0, -1e9, -1.0, -1e9));
    }

    /// a write through each function lands in the table: a call of the row's owner by key, read back from the table
    #[test]
    fn writes_land_in_the_row() {
        let mut gm = Game::blank();
        let g = &mut gm;
        let (a, b) = (GuardId(0), GuardId(1));
        g.time = 100.0;
        let again = g.k.darkdoor.again_secs;
        mind(g, a);
        assert_eq!(g.m.darkdoor.state_of.get(a).unwrap().door, -1.0);
        fear(g, a).act = DarkDoorAct::Balk;
        mind(g, a);
        assert_eq!(g.m.darkdoor.state_of.get(a).unwrap().act, DarkDoorAct::Balk);
        pass(g, a, 3.0);
        assert!(passes(fear(g, a), 3.0, 100.0 + again - 1.0));
        assert!(!passes(fear(g, a), 4.0, 100.0));
        // a balker with a mate on his way: `end`, unresolved, keeps his clock and drops the mate
        { let s = fear(g, a); s.other = Some(b); s.t = 6.0; s.called = true; }
        { let m = fear(g, b); m.act = DarkDoorAct::Come; m.other = Some(a); m.lag = 2.0; }
        end(g, a, Some(true));
        let s = fear(g, a);
        assert_eq!((s.act, s.other, s.left, s.t, s.called), (DarkDoorAct::None, None, 100.0, 6.0, true));
        let m = fear(g, b);
        assert_eq!((m.act, m.other, m.lag), (DarkDoorAct::None, None, 0.0));
        // a mate lost on the way: his caller will not call him again this wait
        { let s = fear(g, a); s.act = DarkDoorAct::Balk; s.other = Some(b); s.call_t = 0.0; }
        { let m = fear(g, b); m.act = DarkDoorAct::Come; m.other = Some(a); }
        lose(g, b);
        let call_secs = g.k.darkdoor.call_secs;
        let s = fear(g, a);
        assert_eq!((s.other, s.call_t), (None, call_secs));
        assert_eq!(s.tried, vec![b]);
        assert_eq!(fear(g, b).act, DarkDoorAct::None);
        assert_eq!(g.stats.darkdoor.drops, 1);
    }
}
