//! src/game/tactics.ts lines 1 to 446
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The closure tacticsPenalty returned, called: a cell of the doorways costs door_mult, a cell of the avoid-set at least CORRIDOR_COST, any other 1. (tactics.ts:360)
fn pen_at(p: &crate::tactics::TacticsPenalty, ix: f64, iz: f64) -> f64 {
    let crate::tactics::TacticsPenalty::Cells { w, door_idx, door_mult, avoid } = p;
    let i = crate::Num(iz * *w + ix);
    let mut mult = 1.0;
    if let Some(cells) = door_idx { if cells.has(i) { mult = *door_mult; } }
    if let Some(cells) = avoid { if cells.has(i) { mult = m::max(mult, CORRIDOR_COST); } }
    mult
}

/// The state as S(g) builds it, -100 on the two clocks. (tactics.ts:151)
fn fresh() -> crate::tactics::TState {
    crate::tactics::TState {
        plans: Vec::new(),
        roles: JsMap::new(),
        lone: JsMap::new(),
        danger: Vec::new(),
        gates: JsMap::new(),
        cleared: JsMap::new(),
        seen_ev: JsSet::new(),
        plan_serial: 0.0,
        last_lone_call_t: -100.0,
        deal_fail_t: -100.0,
    }
}

/// A man's body's position.
fn pos_of(g: &Game, gd: GuardId) -> V3 {
    g.ch[g.gd[gd].ch].pos
}

/// `m.path = []; m.pathGoal = null; m.stallRef = null`: a fresh leg; a whole new path is a new serial, and the standoff's point is no longer his goal. (tactics.ts:430, 441)
fn fresh_leg(g: &mut Game, m: GuardId) {
    let o = &mut g.gd[m];
    o.path = Vec::new();
    o.path_serial = o.path_serial.wrapping_add(1);
    o.path_goal = None;
    o.path_goal_orbit = false;
    o.stall_ref = None;
}

/// The kept knobs into `tune`. The game has no storage, so what the TypeScript does without one: the shipped values stand. (tactics.ts loadTactics)
pub fn load_tactics(tune: &mut crate::tactics::TacticsTune) {}

/// The kept knobs out. The game has no storage: nothing to keep them in. (tactics.ts saveTactics)
pub fn save_tactics(tune: crate::tactics::TacticsTune) {}

/// The four back to the shipped values; the stored copy is the host's. (tactics.ts resetTactics)
pub fn reset_tactics(tune: &mut crate::tactics::TacticsTune) {
    let d = crate::tactics::tactics_defaults();
    tune.flank_wait = d.flank_wait;
    tune.lone_wait = d.lone_wait;
    tune.door_danger = d.door_danger;
    tune.door_wait = d.door_wait;
}

/// The state, built if it is not there: the row itself. (tactics.ts S)
pub fn s<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::tactics::TState {
    if g.m.tactics.states.is_none() { g.m.tactics.states = Some(fresh()); }
    g.m.tactics.states.as_mut().unwrap()
}

/// Hand a man stun charges. (tactics.ts giveStuns)
pub fn give_stuns(g: &mut crate::game::Game, gd: GuardId, n: f64) {
    g.m.tactics.kit.set(gd, crate::tactics::TacticsModKitValue { stuns: m::max(0.0, n) });
}

/// The stun charges on a man. (tactics.ts stunsOf)
pub fn stuns_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    match g.m.tactics.kit.get(gd) { Some(k) => k.stuns, None => 0.0 }
}

/// Is any of it live: the switch on, not the tour, the AI on. (tactics.ts tacticsOn)
pub fn tactics_on(g: &mut crate::game::Game) -> bool {
    g.tune.tactics && !g.quiet_utility && g.ai_enabled
}

/// Distance over the floor. (tactics.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// The bearing of p about C. (tactics.ts bearingOf)
pub fn bearing_of(p: V3, c_2: V3) -> f64 {
    m::atan2(p[0] - c_2[0], p[2] - c_2[2])
}

/// Two bearings' difference in degrees. (tactics.ts degApart)
pub fn deg_apart(a: f64, b: f64) -> f64 {
    m::abs(v3::wrap_angle(a - b)) / v3::DEG
}

/// A free man the tactics may move: alive, in the list, not held / choreographed / on his marks / pinned, not in the standoff. (tactics.ts freeMan)
pub fn free_man(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let o = &g.gd[gd];
    o.state != crate::game::GuardState::Dead && g.guards.contains(&gd) && o.held.is_none() && o.script.is_none() && !o.hold && !o.pinned && o.standoff.is_none()
}

/// Eyes on his man lately: he is fighting or chasing by sight, not working a fix. (tactics.ts inContact)
pub fn in_contact(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.time - g.gd[gd].last_seen_t < crate::guards::PIE_EYES_SECS
}

/// Pinned by fire or flinching under it. (tactics.ts suppressed)
pub fn suppressed(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let sup = &g.gd[gd].sup;
    sup.latched || sup.down_t > 0.0 || sup.v >= g.k.suppression.flinch
}

/// The length of a route over the floor. (tactics.ts pathLen)
pub fn path_len(from: V3, path: &[V3]) -> f64 {
    let mut len = 0.0;
    let mut q = from;
    for &w in path { len += flat(q, w); q = w; }
    len
}

/// Points every `step` metres along from → path (the start and the very end included). (tactics.ts samplePath)
pub fn sample_path(from: V3, path: &[V3], step: Option<f64>) -> Vec<V3> {
    let step = step.unwrap_or(0.3);
    let mut out: Vec<V3> = vec![[from[0], 0.0, from[2]]];
    let mut carry = 0.0;
    let mut a = from;
    for &b in path {
        let l = flat(a, b);
        if l < 1e-6 { a = b; continue; }
        let mut d = step - carry;
        while d <= l {
            out.push([a[0] + (b[0] - a[0]) * d / l, 0.0, a[2] + (b[2] - a[2]) * d / l]);
            d += step;
        }
        carry = l - (d - step);
        a = b;
    }
    let last = if !path.is_empty() { path[path.len() - 1] } else { from };
    if flat(out[out.len() - 1], last) > 1e-3 { out.push([last[0], 0.0, last[2]]); }
    out
}

/// The nav cells a route runs through, widened `dilate` cells all round. (tactics.ts cellsOfPath)
pub fn cells_of_path(g: &mut crate::game::Game, from: V3, path: &[V3], dilate: f64) -> JsSet<crate::Num> {
    let cs = g.col.nav.cell;
    let (w, h) = (g.col.nav.w, g.col.nav.h);
    let mut out = JsSet::new();
    for p in sample_path(from, path, Some(cs * 0.45)) {
        let cx = m::floor(p[0] / cs);
        let cz = m::floor(p[2] / cs);
        let mut dz = -dilate;
        while dz <= dilate {
            let mut dx = -dilate;
            while dx <= dilate {
                let x = cx + dx;
                let z = cz + dz;
                if x >= 0.0 && z >= 0.0 && x < w && z < h {
                    out.add(crate::Num(crate::scene_collision::nav_grid_idx(g, x, z)));
                }
                dx += 1.0;
            }
            dz += 1.0;
        }
    }
    out
}

/// The free cells within `r` of C whose bearing about C lies inside `half_deg` of `bearing`. (tactics.ts sectorCells)
pub fn sector_cells(g: &mut crate::game::Game, c_2: V3, bearing: f64, r: f64, half_deg: f64) -> JsSet<crate::Num> {
    let cs = g.col.nav.cell;
    let (w, h) = (g.col.nav.w, g.col.nav.h);
    let mut out = JsSet::new();
    let x0 = m::max(0.0, m::floor((c_2[0] - r) / cs));
    let x1 = m::min(w - 1.0, m::floor((c_2[0] + r) / cs));
    let z0 = m::max(0.0, m::floor((c_2[2] - r) / cs));
    let z1 = m::min(h - 1.0, m::floor((c_2[2] + r) / cs));
    let mut z = z0;
    while z <= z1 {
        let mut x = x0;
        while x <= x1 {
            let p: V3 = [(x + 0.5) * cs, 0.0, (z + 0.5) * cs];
            let d = flat(p, c_2);
            let i = crate::scene_collision::nav_grid_idx(g, x, z);
            if !(d > r || d < 0.75 || g.col.nav.blocked[i as usize] != 0) && deg_apart(bearing_of(p, c_2), bearing) < half_deg {
                out.add(crate::Num(i));
            }
            x += 1.0;
        }
        z += 1.0;
    }
    out
}

/// The bearing about T at which a walk from → path last comes INTO T's R-disc (his own bearing if he already stands inside it). (tactics.ts entryBearing)
pub fn entry_bearing(t: V3, from: V3, path: &[V3], r: f64) -> f64 {
    if flat(from, t) <= r { return bearing_of(from, t); }
    let s = sample_path(from, path, Some(0.25));
    let mut b = bearing_of(s[s.len() - 1], t);
    let mut i = 0;
    while i + 1 < s.len() {
        if flat(s[i], t) > r && flat(s[i + 1], t) <= r { b = bearing_of(s[i], t); }
        i += 1;
    }
    b
}

/// The line of FIRE from a man stood at p to the fix area at T, chest to chest height. (tactics.ts losOpen)
pub fn los_open(g: &mut crate::game::Game, p: V3, t: V3, team: crate::teams::Team) -> bool {
    let energy = crate::glasspen::side_shot_energy(g, team);
    !crate::scene_collision::shot_blocked(g, [p[0], 1.15, p[2]], [t[0], 1.0, t[2]], Some(0.02), energy)
}

/// The last point along from → path still COVERED from T before the line opens for good on the way in, at least `min_from_t` off T. (tactics.ts lastCoverPoint)
pub fn last_cover_point(g: &mut crate::game::Game, from: V3, path: &[V3], t: V3, min_from_t: f64, team: crate::teams::Team) -> Option<V3> {
    let s = sample_path(from, path, Some(0.3));
    let mut los: Vec<bool> = Vec::with_capacity(s.len());
    for &p in &s { los.push(los_open(g, p, t, team)); }
    let mut i = s.len() as isize - 2;
    while i >= 0 {
        let u = i as usize;
        if !los[u] && los[u + 1] && flat(s[u], t) >= min_from_t { return Some(s[u.saturating_sub(1)]); }
        i -= 1;
    }
    None
}

/// The point along from → path where `rem` metres of it remain (the path's end if it is shorter). (tactics.ts pointShortOfEnd)
pub fn point_short_of_end(from: V3, path: &[V3], rem: f64) -> V3 {
    let s = sample_path(from, path, Some(0.25));
    let total = path_len(from, path);
    let want = m::max(0.0, total - rem);
    let mut acc = 0.0;
    let mut i = 1;
    while i < s.len() {
        acc += flat(s[i - 1], s[i]);
        if acc >= want { return s[i]; }
        i += 1;
    }
    s[s.len() - 1]
}

/// Blocked neighbours of the cell under p. (tactics.ts wallsAbout)
pub fn walls_about(g: &mut crate::game::Game, p: V3) -> f64 {
    let cs = g.col.nav.cell;
    let (w, h) = (g.col.nav.w, g.col.nav.h);
    let cx = m::floor(p[0] / cs);
    let cz = m::floor(p[2] / cs);
    let mut n = 0.0;
    let mut dz = -1.0;
    while dz <= 1.0 {
        let mut dx = -1.0;
        while dx <= 1.0 {
            if dx != 0.0 || dz != 0.0 {
                let x = cx + dx;
                let z = cz + dz;
                if x < 0.0 || z < 0.0 || x >= w || z >= h {
                    n += 1.0;
                } else {
                    let i = crate::scene_collision::nav_grid_idx(g, x, z);
                    if g.col.nav.blocked[i as usize] != 0 { n += 1.0; }
                }
            }
            dx += 1.0;
        }
        dz += 1.0;
    }
    n
}

/// The doorway whose middle lies within `r` of p (nearest such). (tactics.ts doorNear)
pub fn door_near(g: &mut crate::game::Game, p: V3, r: Option<f64>) -> Option<DoorId> {
    let r = r.unwrap_or(DOOR_NEAR_M);
    let mut best = None;
    let mut bd = r;
    for &d in &g.doors.list {
        let dd = flat(g.dr[d].frame_centre, p);
        if dd < bd { bd = dd; best = Some(d); }
    }
    best
}

/// The compass words of a door's name.
static DOOR_DIR: &[(&str, &str)] = &[("n", "north"), ("s", "south"), ("e", "east"), ("w", "west"), ("ext", "outside")];

/// What a man calls a door on the net. (tactics.ts doorLabel)
pub fn door_label(g: &mut crate::game::Game, d: DoorId) -> String {
    let n = g.dr[d].def.name.clone();
    let parts: Vec<&str> = n.split('_').collect();
    let last = parts[parts.len() - 1];
    let dir = DOOR_DIR.iter().find(|e| e.0 == last).map(|e| e.1);
    let named = if parts.len() > 1 { dir } else { None };
    let kept: &[&str] = if named.is_some() { &parts[..parts.len() - 1] } else { &parts[..] };
    let base = kept.join(" ").replacen("breakroom", "break room", 1);
    match named {
        Some(word) => format!("{} {}", base, word),
        None => base,
    }
}

/// Every live record (until > now), optionally for one side. (tactics.ts dangerDoors)
pub fn danger_doors(g: &mut crate::game::Game, team: Option<crate::teams::Team>) -> Vec<TacticsDangerRecId> {
    let now = g.time;
    s(g);
    let tab = &g.tab.tactics_danger_rec;
    g.m.tactics.states.as_ref().unwrap().danger.iter().copied().filter(|&r| tab[r].until > now && (team.is_none() || Some(tab[r].team) == team)).collect()
}

/// The live record for this (door, side). (tactics.ts dangerAt)
pub fn danger_at(g: &mut crate::game::Game, d: DoorId, team: crate::teams::Team) -> Option<TacticsDangerRecId> {
    let now = g.time;
    s(g);
    let tab = &g.tab.tactics_danger_rec;
    g.m.tactics.states.as_ref().unwrap().danger.iter().copied().find(|&r| tab[r].door == d && tab[r].team == team && tab[r].until > now)
}

/// Mark (or re-arm) a doorway dangerous for `team` for tune.doorDanger seconds. (tactics.ts markDanger)
pub fn mark_danger(g: &mut crate::game::Game, d: DoorId, team: crate::teams::Team, why: crate::tactics::DangerWhy, victim: Option<&str>) -> TacticsDangerRecId {
    s(g);
    let found = {
        let tab = &g.tab.tactics_danger_rec;
        g.m.tactics.states.as_ref().unwrap().danger.iter().copied().find(|&x| tab[x].door == d && tab[x].team == team)
    };
    let now = g.time;
    let until = now + m::max(1.0, g.tune.door_danger);
    if let Some(id) = found {
        let r = &mut g.tab.tactics_danger_rec[id];
        if r.until <= now {
            r.t0 = now;
            r.banged = false;
            r.entered = false;
            r.called = false;
            r.summoned = None;
            g.stats.tactics.dangers += 1;
        }
        r.until = m::max(r.until, until);
        r.why = why;
        if let Some(v) = victim { if !v.is_empty() { r.victim = Some(String::from(v)); } }
        return id;
    }
    let id = g.tab.tactics_danger_rec.push(crate::tactics::DangerRec { door: d, team, until, t0: now, why, victim: victim.map(String::from), banged: false, entered: false, called: false, summoned: None });
    s(g).danger.push(id);
    g.stats.tactics.dangers += 1;
    id
}

/// A restart's clean slate: everything this game's tactics held goes. (tactics.ts tacticsReset)
pub fn tactics_reset(g: &mut crate::game::Game) {
    g.m.tactics.states = None;
}

/// Has this man taken this doorway as a drill already inside its current danger. (tactics.ts clearedFor)
pub fn cleared_for(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool {
    let now = g.time;
    match s(g).cleared.get(gd).and_then(|m| m.get(d)) {
        Some(&t) => t > now,
        None => false,
    }
}

/// QA / panel: clearedFor without building the state. (tactics.ts tacticsCleared)
pub fn tactics_cleared(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool {
    g.m.tactics.states.is_some() && cleared_for(g, gd, d)
}

/// Note that this man has taken this doorway, until `until`. (tactics.ts setCleared)
pub fn set_cleared(g: &mut crate::game::Game, gd: GuardId, d: DoorId, until: f64) {
    s(g).cleared.get_or_insert_with(gd, JsMap::new).set(d, until);
}

/// The frame a man goes down: by a doorway AND in the open view of a living man of his side → a kill zone for them. (tactics.ts tacticsManDown)
pub fn tactics_man_down(g: &mut crate::game::Game, victim: GuardId) {
    if !tactics_on(g) { return; }
    let c = g.gd[victim].ch;
    let pos = g.ch[c].pos;
    let Some(d) = door_near(g, pos, None) else { return; };
    let at = match g.ch[c].bones.get(crate::character::Bone::Chest) {
        Some(&b) => b,
        None => v3::add(pos, [0.0, 1.1, 0.0]),
    };
    let team = g.gd[victim].team;
    let mut seen = false;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o != victim
            && g.gd[o].state != crate::game::GuardState::Dead
            && g.gd[o].team == team
            && g.gd[o].held.is_none()
            && g.time >= g.gd[o].dazzled_until
            && crate::guards::point_in_view(g, o, at, Some(16.0), Some(60.0))
        {
            seen = true;
            break;
        }
    }
    if seen {
        let who = g.gd[victim].callsign.clone();
        mark_danger(g, d, team, crate::tactics::DangerWhy::Drop, Some(&who));
    }
}

/// The body discovery: lying by a doorway → a kill zone for the finder's side. (tactics.ts tacticsBodyFound)
pub fn tactics_body_found(g: &mut crate::game::Game, finder: GuardId, body: GuardId) {
    if !tactics_on(g) { return; }
    let at = crate::guards::body_pos(g, body);
    if let Some(d) = door_near(g, at, None) {
        let team = g.gd[finder].team;
        let who = g.gd[body].callsign.clone();
        mark_danger(g, d, team, crate::tactics::DangerWhy::Body, Some(&who));
    }
}

/// Is this body lying in a doorway dangerous to the man's side right now. (tactics.ts bodyDutyBlocked)
pub fn body_duty_blocked(g: &mut crate::game::Game, gd: GuardId, body: GuardId) -> bool {
    if !tactics_on(g) { return false; }
    let at = crate::guards::body_pos(g, body);
    match door_near(g, at, None) {
        Some(d) => {
            let team = g.gd[gd].team;
            danger_at(g, d, team).is_some()
        }
        None => false,
    }
}

/// Rounds through doorways: every shot not yet looked at whose far segment passes a man of a side the shooter is hostile to. (tactics.ts scanRounds)
pub fn scan_rounds(g: &mut crate::game::Game) {
    s(g);
    let mut at = 0;
    while at < g.events.len() {
        let here = at;
        at += 1;
        let e = g.events[here];
        if e.kind != crate::director::EventKind::Shot && e.kind != crate::director::EventKind::GuardShot { continue; }
        let (Some(from), Some(to)) = (e.from, e.to) else { continue; };
        let seen = &mut s(g).seen_ev;
        if seen.has(e.serial) { continue; }
        seen.add(e.serial);
        let shooter = if e.kind == crate::director::EventKind::Shot {
            crate::teams::Team::Player
        } else {
            crate::teams::team_of_id(g, e.who.unwrap_or(-1.0))
        };
        let mut di = 0;
        while di < g.doors.list.len() {
            let d = g.doors.list[di];
            di += 1;
            let Some(x) = crate::guards::crosses_door(g, from, to, d) else { continue; };
            let mut oi = 0;
            while oi < g.guards.len() {
                let o = g.guards[oi];
                oi += 1;
                let team = g.gd[o].team;
                if !crate::teams::is_hostile(g, shooter, team) || g.gd[o].held.is_some() { continue; }
                if g.gd[o].state == crate::game::GuardState::Dead && !(g.gd[o].died_at >= e.time - 1e-6) { continue; }
                if dist_to_segment(pos_of(g, o), x.at, to) <= ROUND_AT_M {
                    mark_danger(g, d, team, crate::tactics::DangerWhy::Rounds, None);
                }
            }
        }
    }
}

/// The distance over the floor from p to the segment a → b. (tactics.ts distToSegment)
pub fn dist_to_segment(p: V3, a: V3, b: V3) -> f64 {
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    let l2 = dx * dx + dz * dz;
    let t = if l2 > 1e-6 { v3::clamp(((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2, 0.0, 1.0) } else { 0.0 };
    m::hyp2(a[0] + dx * t - p[0], a[2] + dz * t - p[2])
}

/// The per-cell multiplier this man's plan pays from here to `goal`, or None = plain A*. (tactics.ts tacticsPenalty)
pub fn tactics_penalty(g: &mut crate::game::Game, gd: GuardId, goal: V3) -> Option<crate::tactics::TacticsPenalty> {
    if !tactics_on(g) || g.gd[gd].script.is_some() || g.gd[gd].state == crate::game::GuardState::Dead { return None; }
    let role = s(g).roles.get(gd).copied();
    let avoid = role.and_then(|r| g.tab.tactics_role[r].avoid.clone());
    let team = g.gd[gd].team;
    let mut recs: Vec<DoorId> = Vec::new();
    for r in danger_doors(g, Some(team)) {
        let door = g.tab.tactics_danger_rec[r].door;
        if !cleared_for(g, gd, door) && s(g).gates.get(gd).map(|x| x.door) != Some(door) {
            recs.push(door);
        }
    }
    if avoid.is_none() && recs.is_empty() { return None; }
    let w = g.col.nav.w;
    let mut door_idx = None;
    let mut door_mult = DOOR_COST_MIN;
    if !recs.is_empty() {
        let mut cells = JsSet::new();
        let dc = crate::guards::door_cells(g);
        for (i, d) in dc.iter() {
            if recs.contains(d) { cells.add(i); }
        }
        door_idx = Some(cells);
        let from = pos_of(g, gd);
        let plain = crate::scene_collision::nav_grid_find_path(g, from, goal, None, None);
        let l = match &plain { Some(p) => path_len(pos_of(g, gd), p), None => 10.0 };
        door_mult = m::max(DOOR_COST_MIN, 1.0 + 1.5 * l);
    }
    Some(crate::tactics::TacticsPenalty::Cells { w, door_idx, door_mult, avoid })
}

/// The roster's role of a man, if any, for the panel / soak / probes. (tactics.ts tacticsRoleOf)
pub fn tactics_role_of(g: &mut crate::game::Game, gd: GuardId) -> Option<String> {
    let st = g.m.tactics.states.as_ref()?;
    if let Some(&r) = st.roles.get(gd) { return Some(String::from(g.tab.tactics_role[r].kind.as_str())); }
    if let Some(x) = st.gates.get(gd) { return Some(format!("door:{}", x.role.as_str())); }
    if let Some(l) = st.lone.get(gd) { if !l.released { return Some(String::from("lone")); } }
    None
}

/// His role. (tactics.ts roleRecord)
pub fn role_record(g: &mut crate::game::Game, gd: GuardId) -> Option<TacticsRoleId> {
    g.m.tactics.states.as_ref()?.roles.get(gd).copied()
}

/// His lone hold, a copy. (tactics.ts loneRecord)
pub fn lone_record(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::tactics::Lone> {
    g.m.tactics.states.as_ref()?.lone.get(gd).cloned()
}

/// His gate, a copy. (tactics.ts gateRecord)
pub fn gate_record(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::tactics::Gate> {
    g.m.tactics.states.as_ref()?.gates.get(gd).cloned()
}

/// The live plans. (tactics.ts plansOf)
pub fn plans_of(g: &mut crate::game::Game) -> Vec<TacticsPlanId> {
    match g.m.tactics.states.as_ref() { Some(st) => st.plans.clone(), None => Vec::new() }
}

/// The flanker's call: 'coming at him / them' only if there is somebody to come at. (tactics.ts flankCall)
pub fn flank_call(g: &mut crate::game::Game, plan: TacticsPlanId, m: GuardId, place: &str) -> String {
    let who = if crate::teams::defender_words(g, m) { "them" } else { "him" };
    let mut since = f64::INFINITY;
    let members = g.tab.tactics_plan[plan].members.clone();
    for o in members {
        let t = match crate::pressure::look_since(g, o) {
            Some(t) => Some(t),
            None => crate::enemysquads::squad_look_since(g, o),
        };
        let Some(t) = t else {
            let moving = crate::words::wg(g, &"going around", &"moving round");
            return format!("{} — coming at {} from {}", moving, who, place);
        };
        since = m::min(since, t);
    }
    let team = g.tab.tactics_plan[plan].team;
    let seen = g.guards.iter().any(|&o| g.gd[o].team == team && g.gd[o].last_seen_t > since);
    let moving = crate::words::wg(g, &"going around", &"moving round");
    if seen {
        format!("{} — coming at {} from {}", moving, who, place)
    } else {
        format!("{} — coming in from {}", moving, place)
    }
}

/// A role as dealt, a new row. (tactics.ts:410 mk)
fn mk(g: &mut Game, plan: TacticsPlanId, kind: crate::tactics::RoleKind, hold_at: Option<V3>, route: Vec<V3>, len: f64, bearing: f64, av: Option<JsSet<crate::Num>>) -> TacticsRoleId {
    let t0 = g.time;
    g.tab.tactics_role.push(crate::tactics::Role { kind, plan, hold_at, route, len, bearing, avoid: av, set: false, holding: false, hold_t: 0.0, arrived: false, said_moving: false, said_set: false, t0, ground: None, last_pos: None })
}

/// His side's dangerous doorways priced in, no avoid-set, made once a man; the roster's own delete / set kept as it was (it moves him to the map's end). (tactics.ts:399 dangerOnly)
fn danger_only(g: &mut Game, gd: GuardId, t: V3, pens: &mut JsMap<GuardId, Option<crate::tactics::TacticsPenalty>>) -> Option<crate::tactics::TacticsPenalty> {
    let r = s(g).roles.delete(gd);
    if !pens.has(gd) {
        let p = tactics_penalty(g, gd, t);
        pens.set(gd, p);
    }
    if let Some(r) = r { s(g).roles.set(gd, r); }
    pens.get(gd).cloned().flatten()
}

/// Deal roles to `men` over the nav grid toward `target`, register the plan and return it; None if fewer than two have a route. (tactics.ts planApproach)
pub fn plan_approach(g: &mut crate::game::Game, men: &[GuardId], target: V3, why: Option<&str>, prefer: Option<GuardId>) -> Option<TacticsPlanId> {
    let why = why.unwrap_or("formed");
    s(g);
    let team = g.gd[men[0]].team;
    let t_pt: V3 = [target[0], 0.0, target[2]];
    // any old plan these men were in goes
    for &m in men {
        let Some(r) = s(g).roles.get(m).copied() else { continue; };
        let old = g.tab.tactics_role[r].plan;
        if g.tab.tactics_plan[old].phase != crate::tactics::PlanPhase::Done {
            let said = format!("re-dealt ({})", why);
            end_plan(g, old, &said, Some(false));
        }
    }
    let mut pens: JsMap<GuardId, Option<crate::tactics::TacticsPenalty>> = JsMap::new();
    let mut routes: Vec<(GuardId, Vec<V3>, f64)> = Vec::new();
    for &m in men {
        let from = pos_of(g, m);
        let base = danger_only(g, m, t_pt, &mut pens);
        let path = match &base {
            Some(p) => {
                let mut price = |_g: &mut Game, ix: f64, iz: f64| pen_at(p, ix, iz);
                crate::scene_collision::nav_grid_find_path(g, from, t_pt, Some(6000.0), Some(&mut price))
            }
            None => crate::scene_collision::nav_grid_find_path(g, from, t_pt, Some(6000.0), None),
        };
        if let Some(path) = path {
            let len = path_len(pos_of(g, m), &path);
            if len.is_finite() { routes.push((m, path, len)); }
        }
    }
    if routes.len() < 2 {
        let now = g.time;
        s(g).deal_fail_t = now;
        return None;
    }
    js::sort_by(&mut routes, |a, b| a.2 - b.2);
    if let Some(pref) = prefer {
        if let Some(i) = routes.iter().position(|r| r.0 == pref) {
            if i > 0 { let r = routes.remove(i); routes.insert(0, r); }
        }
    }
    let (p0_gd, p0_path, p0_len) = routes[0].clone();
    let p0_pos = pos_of(g, p0_gd);
    let corridor = cells_of_path(g, p0_pos, &p0_path, CORRIDOR_DILATE);
    let p_bear = entry_bearing(t_pt, p0_pos, &p0_path, DISC_M);
    let sector = sector_cells(g, t_pt, p_bear, SECTOR_M, FLANK_MIN_DEG);
    let mut avoid = corridor.clone();
    for c in sector.iter() { avoid.add(c); }
    let now = g.time;
    let id = {
        let st = s(g);
        st.plan_serial += 1.0;
        st.plan_serial
    };
    let plan = g.tab.tactics_plan.push(crate::tactics::Plan {
        id,
        team,
        target: t_pt,
        t0: now,
        replanned_at: now,
        replans: 0.0,
        members: routes.iter().map(|r| r.0).collect(),
        roles: JsMap::new(),
        phase: crate::tactics::PlanPhase::Approach,
        go_t: -1.0,
        why: String::from(why),
        presser_bearing: p_bear,
        presser_len: p0_len,
        corridor,
        first_arrive_t: -1.0,
        last_radio_t: -100.0,
        contact: false,
    });
    // the presser: up the short route to the last corner that still hides him (else the disc's edge)
    let p0_team = g.gd[p0_gd].team;
    let p_hold = match last_cover_point(g, p0_pos, &p0_path, t_pt, 1.0, p0_team) {
        Some(p) => p,
        None => point_short_of_end(p0_pos, &p0_path, DISC_M),
    };
    let role = mk(g, plan, crate::tactics::RoleKind::Presser, Some(p_hold), p0_path.clone(), p0_len, p_bear, None);
    g.tab.tactics_plan[plan].roles.set(p0_gd, role);
    // the others: a flank if the dear-corridor search finds a way in from the other side inside the ratio, else a second presser
    let w = g.col.nav.w;
    for r in routes.iter().skip(1) {
        let (r_gd, r_path, r_len) = (r.0, &r.1, r.2);
        let from = pos_of(g, r_gd);
        let base = danger_only(g, r_gd, t_pt, &mut pens);
        let fp = {
            let mut price = |_g: &mut Game, ix: f64, iz: f64| {
                let b = match &base { Some(p) => pen_at(p, ix, iz), None => 1.0 };
                m::max(b, if avoid.has(crate::Num(iz * w + ix)) { CORRIDOR_COST } else { 1.0 })
            };
            crate::scene_collision::nav_grid_find_path(g, from, t_pt, Some(6000.0), Some(&mut price))
        };
        let flen = match &fp { Some(p) => path_len(from, p), None => f64::INFINITY };
        let f_bear = match &fp { Some(p) => entry_bearing(t_pt, from, p, DISC_M), None => p_bear };
        let apart = deg_apart(f_bear, p_bear);
        let flank = match fp {
            Some(p) if flen <= FLANK_MAX_RATIO * m::max(p0_len, r_len) && apart >= FLANK_MIN_DEG - 0.5 => Some(p),
            _ => None,
        };
        if let Some(p) = flank {
            let r_team = g.gd[r_gd].team;
            let set_at = match last_cover_point(g, from, &p, t_pt, 1.0, r_team) {
                Some(q) => q,
                None => point_short_of_end(from, &p, DISC_M + 0.5),
            };
            let role = mk(g, plan, crate::tactics::RoleKind::Flanker, Some(set_at), p, flen, f_bear, Some(avoid.clone()));
            g.tab.tactics_plan[plan].roles.set(r_gd, role);
            g.stats.tactics.flankers += 1;
        } else {
            let role = mk(g, plan, crate::tactics::RoleKind::Second, None, r_path.clone(), r_len, p_bear, None);
            g.tab.tactics_plan[plan].roles.set(r_gd, role);
            g.stats.tactics.seconds += 1;
        }
    }
    // a fresh leg for everybody: goTo plans it with the new costs
    let dealt = g.tab.tactics_plan[plan].roles.keys();
    for &m in &dealt {
        let role = *g.tab.tactics_plan[plan].roles.get(m).unwrap();
        let st = s(g);
        st.roles.set(m, role);
        st.lone.delete(m);
        fresh_leg(g, m);
    }
    s(g).plans.push(plan);
    g.stats.tactics.plans += 1;
    if why != "formed" { g.stats.tactics.replans += 1; }
    // the flanker says so (radio: the debrief hears it)
    for &m in &dealt {
        let role = *g.tab.tactics_plan[plan].roles.get(m).unwrap();
        let (kind, said, hold) = {
            let r = &g.tab.tactics_role[role];
            (r.kind, r.said_moving, r.hold_at)
        };
        if kind != crate::tactics::RoleKind::Flanker || said { continue; }
        g.tab.tactics_role[role].said_moving = true;
        let place = crate::dialogue::place_name(g, hold.unwrap_or(t_pt));
        let text = flank_call(g, plan, m, &place);
        crate::game::game_say(g, m, &text, Some(true), None, None);
        let now = g.time;
        g.tab.tactics_plan[plan].last_radio_t = now;
    }
    Some(plan)
}

/// Dissolve a plan: its roles come off the roster (the men chase / search as they always did from where they stand). (tactics.ts endPlan)
pub fn end_plan(g: &mut crate::game::Game, plan: TacticsPlanId, why: &str, reset_men: Option<bool>) {
    let reset_men = reset_men.unwrap_or(true);
    s(g);
    if g.tab.tactics_plan[plan].phase == crate::tactics::PlanPhase::Done { return; }
    g.tab.tactics_plan[plan].phase = crate::tactics::PlanPhase::Done;
    for m in g.tab.tactics_plan[plan].roles.keys() {
        let r = g.tab.tactics_plan[plan].roles.get(m).copied();
        if s(g).roles.get(m).copied() != r { continue; }
        s(g).roles.delete(m);
        if reset_men && g.gd[m].state != crate::game::GuardState::Dead {
            fresh_leg(g, m);
            crate::guards::reset_chase(g, m);
        }
    }
    s(g).plans.retain(|&p| p != plan);
}

/// A member still fit to hold a role: free, alert, a fix, his man not down. (tactics.ts fitForRole)
pub fn fit_for_role(g: &mut crate::game::Game, m: GuardId) -> bool {
    free_man(g, m)
        && g.gd[m].state == crate::game::GuardState::Alert
        && g.gd[m].last_known.is_some()
        && !crate::teams::foe_down(g, m)
        && !(g.time < g.gd[m].dazzled_until)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(xs: &[f64]) -> Vec<u64> {
        xs.iter().map(|x| x.to_bits()).collect()
    }

    const PATH: [V3; 4] = [[3.7, 0.0, 1.1], [3.7, 0.0, 1.1], [9.25, 0.0, -2.4], [12.0, 0.0, 6.125]];
    const FROM: V3 = [0.3, 1.0, 0.7];
    const T: V3 = [11.0, 0.0, 3.0];

    // printed by bun with EXACT.on from a verbatim copy of tactics.ts:166-168, 177-189, 211-216, 231-235, 336-340
    const FLAT: u64 = 0x4025e3890c1ed489u64;
    const BEARING: u64 = 0xbffc853be4bcdd19u64;
    const DEG_APART: u64 = 0x403039aec352e85fu64;
    const PATH_LEN: u64 = 0x4032f14539dddd5cu64;
    const SAMPLE: [u64; 195] = [0x3fd3333333333333u64, 0x0u64, 0x3fe6666666666666u64, 0x3fe3225deecf10eeu64, 0x0u64, 0x3fe7858c8e8acecau64, 0x3fecab2244048842u64, 0x0u64, 0x3fe8a4b2b6af372fu64, 0x3ff319f34c9cffcbu64, 0x0u64, 0x3fe9c3d8ded39f93u64, 0x3ff7de557737bb76u64, 0x0u64, 0x3feae2ff06f807f8u64, 0x3ffca2b7a1d27721u64, 0x0u64, 0x3fec02252f1c705cu64, 0x4000b38ce6369966u64, 0x0u64, 0x3fed214b5740d8c0u64, 0x400315bdfb83f73bu64, 0x0u64, 0x3fee40717f654125u64, 0x400577ef10d1550fu64, 0x0u64, 0x3fef5f97a789a989u64, 0x4007da20261eb2e4u64, 0x0u64, 0x3ff03f5ee7d708f7u64, 0x400a3c513b6c10b9u64, 0x0u64, 0x3ff0cef1fbe93d29u64, 0x400c9e8250b96e8du64, 0x0u64, 0x3ff15e850ffb715bu64, 0x400ecb70c6190294u64, 0x0u64, 0x3ff017daee2955c7u64, 0x40106990ce27d822u64, 0x0u64, 0x3feb10c75a7ca267u64, 0x40116d6939432ef9u64, 0x0u64, 0x3fe5f1d8d8a69940u64, 0x40127141a45e85d1u64, 0x0u64, 0x3fe0d2ea56d0901au64, 0x4013751a0f79dca8u64, 0x0u64, 0x3fd767f7a9f50de6u64, 0x401478f27a953380u64, 0x0u64, 0x3fca54354c91f72cu64, 0x40157ccae5b08a58u64, 0x0u64, 0x3fa761ed14e74a40u64, 0x401680a350cbe12fu64, 0x0u64, 0xbfbd467d843ca410u64, 0x4017847bbbe73807u64, 0x0u64, 0xbfd18f7c64bb3b4cu64, 0x4018885427028edeu64, 0x0u64, 0xbfdbcd5968674d98u64, 0x40198c2c921de5b6u64, 0x0u64, 0xbfe3059b3609aff2u64, 0x401a9004fd393c8eu64, 0x0u64, 0xbfe82489b7dfb91au64, 0x401b93dd68549364u64, 0x0u64, 0xbfed437839b5c240u64, 0x401c97b5d36fea3cu64, 0x0u64, 0xbff131335dc5e5b2u64, 0x401d9b8e3e8b4114u64, 0x0u64, 0xbff3c0aa9eb0ea46u64, 0x401e9f66a9a697ebu64, 0x0u64, 0xbff65021df9beed8u64, 0x401fa33f14c1eec3u64, 0x0u64, 0xbff8df992086f36au64, 0x4020538bbfeea2cdu64, 0x0u64, 0xbffb6f106171f7feu64, 0x4020d577f57c4e39u64, 0x0u64, 0xbffdfe87a25cfc90u64, 0x402157642b09f9a4u64, 0x0u64, 0xc00046ff71a40092u64, 0x4021d9506097a510u64, 0x0u64, 0xc0018ebb121982dcu64, 0x40225b3c9625507cu64, 0x0u64, 0xc002d676b28f0525u64, 0x4022a1cff45b5c20u64, 0x0u64, 0xc0018fed5d2cbcdcu64, 0x4022d0f7cb8716e7u64, 0x0u64, 0xbffe8e6515502e71u64, 0x4023001fa2b2d1aeu64, 0x0u64, 0xbff9fcef7046e329u64, 0x40232f4779de8c75u64, 0x0u64, 0xbff56b79cb3d97e2u64, 0x40235e6f510a473cu64, 0x0u64, 0xbff0da0426344c99u64, 0x40238d9728360203u64, 0x0u64, 0xbfe8911d025602a2u64, 0x4023bcbeff61bccau64, 0x0u64, 0xbfdedc637086d820u64, 0x4023ebe6d68d7791u64, 0x0u64, 0xbfc92d19b8c35610u64, 0x40241b0eadb93258u64, 0x0u64, 0x3fb6bd26df0e0860u64, 0x40244a3684e4ed1fu64, 0x0u64, 0x3fd7f5204be8af38u64, 0x4024795e5c10a7e6u64, 0x0u64, 0x3fe51d7b7006ee28u64, 0x4024a886333c62adu64, 0x0u64, 0x3fee4066ba1984b4u64, 0x4024d7ae0a681d74u64, 0x0u64, 0x3ff3b1a902160da2u64, 0x402506d5e193d83bu64, 0x0u64, 0x3ff8431ea71f58ecu64, 0x402535fdb8bf9302u64, 0x0u64, 0x3ffcd4944c28a432u64, 0x402565258feb4dc9u64, 0x0u64, 0x4000b304f898f7bdu64, 0x4025944d67170890u64, 0x0u64, 0x4002fbbfcb1d9d61u64, 0x4025c3753e42c357u64, 0x0u64, 0x4005447a9da24305u64, 0x4025f29d156e7e1eu64, 0x0u64, 0x40078d357026e8a9u64, 0x402621c4ec9a38e5u64, 0x0u64, 0x4009d5f042ab8e4bu64, 0x402650ecc3c5f3acu64, 0x0u64, 0x400c1eab153033efu64, 0x402680149af1ae73u64, 0x0u64, 0x400e6765e7b4d993u64, 0x4026af3c721d693au64, 0x0u64, 0x401058105d1cbf9cu64, 0x4026de6449492401u64, 0x0u64, 0x40117c6dc65f126eu64, 0x40270d8c2074dec8u64, 0x0u64, 0x4012a0cb2fa1653eu64, 0x40273cb3f7a0998fu64, 0x0u64, 0x4013c52898e3b810u64, 0x40276bdbcecc5456u64, 0x0u64, 0x4014e98602260ae4u64, 0x40279b03a5f80f1du64, 0x0u64, 0x40160de36b685db6u64, 0x4027ca2b7d23c9e4u64, 0x0u64, 0x40173240d4aab088u64, 0x4027f953544f84acu64, 0x0u64, 0x4018569e3ded035au64, 0x4028000000000000u64, 0x0u64, 0x4018800000000000u64];
    const ENTRY: u64 = 0xc0069f2469ff3b92u64;
    const SHORT: [u64; 3] = [0x4025944d67170891u64, 0x0u64, 0x4002fbbfcb1d9d6bu64];
    const DIST_SEG: u64 = 0x4016b38e0c879b36u64;
    const DIST_POINT: u64 = 0x4025e3890c1ed489u64;

    #[test]
    fn the_typescripts_own_bits() {
        assert_eq!(flat(FROM, T).to_bits(), FLAT);
        assert_eq!(bearing_of(FROM, T).to_bits(), BEARING);
        assert_eq!(deg_apart(2.9, -3.1).to_bits(), DEG_APART);
        assert_eq!(path_len(FROM, &PATH).to_bits(), PATH_LEN);
        let s: Vec<f64> = sample_path(FROM, &PATH, None).iter().flat_map(|p| [p[0], p[1], p[2]]).collect();
        assert_eq!(bits(&s), SAMPLE.to_vec());
        assert_eq!(entry_bearing(T, FROM, &PATH, 4.0).to_bits(), ENTRY);
        let p = point_short_of_end(FROM, &PATH, 4.0);
        assert_eq!(bits(&p), SHORT.to_vec());
        assert_eq!(dist_to_segment(T, FROM, PATH[2]).to_bits(), DIST_SEG);
        assert_eq!(dist_to_segment(T, FROM, [FROM[0], 0.0, FROM[2]]).to_bits(), DIST_POINT);
    }

    #[test]
    fn an_empty_path_samples_its_start_and_a_huge_rem_stops_at_the_first_sample() {
        assert_eq!(sample_path(FROM, &[], None), vec![[FROM[0], 0.0, FROM[2]]]);
        assert_eq!(point_short_of_end(FROM, &PATH, 1e9), sample_path(FROM, &PATH, Some(0.25))[1]);
    }
    #[test]
    fn a_cell_costs_the_doors_price_the_corridor_at_least_six_and_else_one() {
        let mut doors = JsSet::new();
        doors.add(crate::Num(2.0 * 4.0 + 1.0));
        doors.add(crate::Num(3.0));
        let mut avoid = JsSet::new();
        avoid.add(crate::Num(3.0));
        avoid.add(crate::Num(5.0));
        let p = crate::tactics::TacticsPenalty::Cells { w: 4.0, door_idx: Some(doors.clone()), door_mult: 3.0, avoid: Some(avoid.clone()) };
        assert_eq!(pen_at(&p, 1.0, 2.0), 3.0);
        assert_eq!(pen_at(&p, 3.0, 0.0), CORRIDOR_COST);
        assert_eq!(pen_at(&p, 1.0, 1.0), CORRIDOR_COST);
        assert_eq!(pen_at(&p, 0.0, 0.0), 1.0);
        let dear = crate::tactics::TacticsPenalty::Cells { w: 4.0, door_idx: Some(doors), door_mult: 20.0, avoid: Some(avoid) };
        assert_eq!(pen_at(&dear, 3.0, 0.0), 20.0);
    }

    #[test]
    fn the_state_is_the_row_and_starts_with_the_clocks_back() {
        let mut g = Game::blank();
        assert_eq!(s(&mut g).last_lone_call_t, -100.0);
        assert_eq!(s(&mut g).deal_fail_t, -100.0);
        s(&mut g).plan_serial = 7.0;
        assert_eq!(g.m.tactics.states.as_ref().unwrap().plan_serial, 7.0);
    }

    #[test]
    fn a_danger_rearmed_keeps_its_row_and_only_an_expired_one_starts_over() {
        use crate::teams::Team;
        use crate::tactics::DangerWhy;
        let mut g = Game::blank();
        g.time = 10.0;
        g.tune.door_danger = 25.0;
        let d = DoorId(3);
        let r = mark_danger(&mut g, d, Team::Guard, DangerWhy::Drop, Some("A1"));
        assert_eq!(g.tab.tactics_danger_rec[r].until, 35.0);
        assert_eq!(g.stats.tactics.dangers, 1);
        g.tab.tactics_danger_rec[r].banged = true;
        // live: same row, flags kept, the clock only ever goes out, no empty name over a name
        g.time = 12.0;
        assert_eq!(mark_danger(&mut g, d, Team::Guard, DangerWhy::Rounds, Some("")), r);
        assert!(g.tab.tactics_danger_rec[r].banged);
        assert_eq!(g.tab.tactics_danger_rec[r].until, 37.0);
        assert_eq!(g.tab.tactics_danger_rec[r].why, DangerWhy::Rounds);
        assert_eq!(g.tab.tactics_danger_rec[r].victim.as_deref(), Some("A1"));
        assert_eq!(g.tab.tactics_danger_rec[r].t0, 10.0);
        assert_eq!(g.stats.tactics.dangers, 1);
        // the other side has a row of its own, and the readers tell them apart
        let other = mark_danger(&mut g, d, Team::Player, DangerWhy::Press, None);
        assert_ne!(other, r);
        assert_eq!(danger_at(&mut g, d, Team::Guard), Some(r));
        assert_eq!(danger_at(&mut g, DoorId(4), Team::Guard), None);
        assert_eq!(danger_doors(&mut g, Some(Team::Player)), vec![other]);
        assert_eq!(danger_doors(&mut g, None), vec![r, other]);
        // run out: not live, and a new mark starts it over in the same row
        g.time = 100.0;
        assert_eq!(danger_at(&mut g, d, Team::Guard), None);
        assert!(danger_doors(&mut g, None).is_empty());
        assert_eq!(mark_danger(&mut g, d, Team::Guard, DangerWhy::Body, None), r);
        assert!(!g.tab.tactics_danger_rec[r].banged);
        assert_eq!(g.tab.tactics_danger_rec[r].t0, 100.0);
        assert_eq!(g.tab.tactics_danger_rec[r].until, 125.0);
        assert_eq!(g.stats.tactics.dangers, 3);
        assert_eq!(s(&mut g).danger.len(), 2);
    }

    #[test]
    fn ending_a_plan_takes_off_its_own_roles_only() {
        use crate::tactics::{Plan, PlanPhase, RoleKind};
        let mut g = Game::blank();
        let (a, b): (GuardId, GuardId) = (GuardId(0), GuardId(1));
        let old = g.tab.tactics_plan.push(Plan::zeroed());
        let new = g.tab.tactics_plan.push(Plan::zeroed());
        g.tab.tactics_plan[old].phase = PlanPhase::Approach;
        g.tab.tactics_plan[new].phase = PlanPhase::Approach;
        let ra = mk(&mut g, old, RoleKind::Presser, None, Vec::new(), 0.0, 0.0, None);
        let rb_old = mk(&mut g, old, RoleKind::Second, None, Vec::new(), 0.0, 0.0, None);
        let rb_new = mk(&mut g, new, RoleKind::Presser, None, Vec::new(), 0.0, 0.0, None);
        g.tab.tactics_plan[old].roles.set(a, ra);
        g.tab.tactics_plan[old].roles.set(b, rb_old);
        g.tab.tactics_plan[new].roles.set(b, rb_new);
        {
            let st = s(&mut g);
            st.roles.set(a, ra);
            st.roles.set(b, rb_new);
            st.plans.push(old);
            st.plans.push(new);
        }
        end_plan(&mut g, old, "test", Some(false));
        assert_eq!(g.tab.tactics_plan[old].phase, PlanPhase::Done);
        assert_eq!(s(&mut g).roles.get(a).copied(), None);
        assert_eq!(s(&mut g).roles.get(b).copied(), Some(rb_new));
        assert_eq!(plans_of(&mut g), vec![new]);
        assert_eq!(role_record(&mut g, b), Some(rb_new));
        assert_eq!(tactics_role_of(&mut g, b).as_deref(), Some("presser"));
        // already done: nothing moves
        s(&mut g).roles.set(a, ra);
        end_plan(&mut g, old, "again", Some(false));
        assert_eq!(s(&mut g).roles.get(a).copied(), Some(ra));
    }

    #[test]
    fn a_doorway_taken_stays_taken_until_its_time() {
        let mut g = Game::blank();
        g.time = 5.0;
        assert!(!tactics_cleared(&mut g, GuardId(0), DoorId(1)));
        assert!(g.m.tactics.states.is_none());
        set_cleared(&mut g, GuardId(0), DoorId(1), 8.0);
        assert!(tactics_cleared(&mut g, GuardId(0), DoorId(1)));
        assert!(!cleared_for(&mut g, GuardId(0), DoorId(2)));
        assert!(!cleared_for(&mut g, GuardId(1), DoorId(1)));
        g.time = 8.0;
        assert!(!cleared_for(&mut g, GuardId(0), DoorId(1)));
    }
}
