//! src/game/digin.ts lines 1 to 462
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- seam: what skeleton 3 cannot yet say (notes.md, Skeleton)

/// digin.ts:393 `g.audio.play(name, pos, vol, { rate })`: game_audio_like_play's opts hold crate::Opaque, which cannot carry the rate. The rate's die is drawn by the caller, before this.
fn seam_audio_play(g: &mut Game, name: &str, pos: V3, vol: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a number (crate::Opaque holds nothing)") }

// ---------------------------------------------------------------- the table of defaults and the state

/// Host work: the debugger panel's copy of the table in the browser's storage. Nothing of the game's. (digin.ts saveDigIn)
pub fn save_dig_in(g: &mut Game) {}

/// Host work: the stored copy and the address word `?digin`; the host writes g.k.digin. Nothing of the game's. (digin.ts loadDigIn)
pub fn load_dig_in(g: &mut Game) {}

/// Back to the defaults; a level's difficulty does this. The stored copy is the host's. (digin.ts resetDigIn)
pub fn reset_dig_in(g: &mut Game) { g.k.digin = crate::digin::digin_defaults(); }

/// The game's state, made on first use by the file's own literal: the row itself. (digin.ts stateOf)
pub fn state_of<'g>(g: &'g mut Game) -> &'g mut crate::digin::St {
    g.m.digin.states.get_or_insert_with(|| crate::digin::St {
        men: JsMap::new(),
        rooms: JsMap::new(),
        wedges: JsMap::new(),
        fix: None,
        called: JsSet::new(),
        used: JsSet::new(),
        away: JsSet::new(),
        frame_t: -1.0,
        bang_call: m::NAN,
    })
}

// ---------------------------------------------------------------- small things

/// Distance on the floor. (digin.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// A draw from [min, max]. (digin.ts pick)
pub fn pick(g: &mut Game, r: [f64; 2]) -> f64 { r[0] + crate::math_dice::sim_random(g) * (r[1] - r[0]) }

/// A draw from [min, max] at the table's pace. (digin.ts paced)
pub fn paced(g: &mut Game, r: [f64; 2]) -> f64 {
    let p = pick(g, r);
    p * m::max(0.1, g.k.digin.pace)
}

/// One of these, by the die. (digin.ts oneOf)
pub fn one_of<T: Clone + core::fmt::Debug>(g: &mut Game, a: &[T]) -> T {
    let i = m::floor(crate::math_dice::sim_random(g) * a.len() as f64);
    match a.get(i as usize) {
        Some(x) => x.clone(),
        None => a[0].clone(),
    }
}

/// The point at this height. (digin.ts at)
pub fn at(p: V3, y: f64) -> V3 { [p[0], y, p[2]] }

/// A line in the file's log, capped, and in the fight's. (digin.ts log)
pub fn log(g: &mut Game, gd: Option<GuardId>, what: &str) {
    let who = match gd {
        Some(id) => g.gd[id].callsign.clone(),
        None => String::from("-"),
    };
    let t = g.time;
    g.m.digin.digin_log.push(crate::digin::DiginModDIGINLOGItem { t, who, what: String::from(what) });
    if g.m.digin.digin_log.len() > 400 {
        g.m.digin.digin_log.drain(0..150);
    }
    crate::roomfight::room_fight_note(g, gd, &alloc::format!("dig-in: {}", what));
}

// ---------------------------------------------------------------- who can hear it

/// Every player who is up and the living teammates, with their ears. (digin.ts listeners)
pub fn listeners(g: &mut Game) -> Vec<crate::digin::ListenersItem> {
    let mut out = Vec::new();
    let of = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    for pl in crate::seats::men(g, &of) {
        if !g.pl[pl].down {
            let c = g.pl[pl].ch;
            out.push(crate::digin::ListenersItem { gd: None, head: at(g.ch[c].pos, 1.6), vel: g.ch[c].vel });
        }
    }
    for o in g.fight.ops.iter() {
        let id = g.tab.roomfight_op[*o].gd;
        if g.gd[id].state != crate::game::GuardState::Dead {
            let c = g.gd[id].ch;
            out.push(crate::digin::ListenersItem { gd: Some(id), head: at(g.ch[c].pos, 1.6), vel: g.ch[c].vel });
        }
    }
    out
}

/// The nearest ear a sound of this reach gets to. (digin.ts heardBy)
pub fn heard_by(g: &mut Game, src: V3, reach_m: f64) -> Option<crate::digin::HeardBy> {
    let mut best = None;
    let mut bd = m::INF;
    for l in listeners(g) {
        if flat(src, l.head) > reach_m { continue; }
        let p = crate::weather::hear_path(g, at(src, 1.2), l.head, None);
        if p.path_len < reach_m * p.carry && p.path_len < bd {
            bd = p.path_len;
            best = Some(crate::digin::HeardBy { gd: l.gd });
        }
    }
    best
}

/// The same by the hearing model's law for door and furniture noise. (digin.ts heardNoise)
pub fn heard_noise(g: &mut Game, src: V3, level: f64) -> Option<crate::digin::HeardNoise> {
    let mut best = None;
    let mut bd = m::INF;
    for l in listeners(g) {
        if flat(src, l.head) >= 12.0 { continue; }
        let p = crate::weather::hear_path(g, at(src, 0.5), l.head, None);
        if level * m::max(0.0, 1.0 - p.path_len / 12.0) * p.carry > 0.08 && p.path_len < bd {
            bd = p.path_len;
            best = Some(crate::digin::HeardNoise { gd: l.gd });
        }
    }
    best
}

/// A cue reached the team, before they first stepped in. (digin.ts cue)
pub fn cue(g: &mut Game, r: Option<&mut crate::digin::RoomSt>, kind: crate::digin::Cue) {
    let r = match r {
        Some(r) => r,
        None => return,
    };
    if !r.entered_at.is_nan() { return; }
    r.cues.add(kind);
    if r.cue_at.is_nan() { r.cue_at = g.time; }
}

/// A line of his out loud: said only when somebody on the team's side can hear it. (digin.ts shout)
pub fn shout(g: &mut Game, gd: GuardId, text: &str, r: Option<&mut crate::digin::RoomSt>) -> bool {
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let reach = g.k.digin.shout_m;
    if heard_by(g, pos, reach).is_some() {
        crate::game::game_say(g, gd, text, None, None, None);
        g.stats.digin.shouts += 1;
        cue(g, r, crate::digin::Cue::Shout);
        log(g, Some(gd), &alloc::format!("shouts (heard): {}", text));
        return true;
    }
    log(g, Some(gd), &alloc::format!("shouts (nobody to hear it): {}", text));
    false
}

// ---------------------------------------------------------------- which door will they come by

/// A path's length on the floor. (digin.ts pathLen)
pub fn path_len(from: V3, path: &[V3]) -> f64 {
    let mut l = 0.0;
    let mut q = from;
    for w in path {
        l += flat(q, *w);
        q = *w;
    }
    l
}

/// How far on foot from `a` to `b`; infinity: no way found. (digin.ts wayTo)
pub fn way_to(g: &mut Game, a: V3, b: V3) -> f64 {
    let p = crate::roomfight::room_fight_free_near(g, a).unwrap_or(a);
    let q = crate::roomfight::room_fight_free_near(g, b).unwrap_or(b);
    match crate::scene_collision::nav_grid_find_path(g, p, q, Some(6000.0), None) {
        Some(path) => path_len(p, &path),
        None => m::INF,
    }
}

/// The room's entries by the way round from `from` to their far side, never through the room: [expected, second or -1]. (digin.ts expectEntries)
pub fn expect_entries(g: &mut Game, room: RoomfightRoomId, from: V3) -> [f64; 2] {
    let src = crate::roomfight::room_fight_free_near(g, from).unwrap_or(from);
    let entries = g.tab.roomfight_room[room].entries.clone();
    let mut l: Vec<f64> = Vec::with_capacity(entries.len());
    for e in entries.iter() {
        l.push(way_round(g, room, src, e));
    }
    if !l.iter().any(|x| x.is_finite()) {
        for (i, e) in entries.iter().enumerate() {
            l[i] = flat(e.at, from) * 1.5;
        }
    }
    let mut idx: Vec<usize> = (0..l.len()).collect();
    js::sort_by(&mut idx, |&a, &b| l[a] - l[b]);
    if idx.is_empty() { return [-1.0, -1.0]; }
    let first = idx[0];
    let second = if idx.len() > 1 && l[idx[1]] - l[first] < g.k.digin.alt_m { idx[1] as f64 } else { -1.0 };
    [first as f64, second]
}

/// One entry's cost for expect_entries: the map's closure at digin.ts:264.
fn way_round(g: &mut Game, room: RoomfightRoomId, src: V3, e: &crate::roomfight::Entry) -> f64 {
    let o = match crate::roomfight::room_fight_free_near(g, [e.at[0] + e.out[0] * 1.2, 0.0, e.at[2] + e.out[1] * 1.2]) {
        Some(o) => o,
        None => return m::INF,
    };
    let mut dear = |g: &mut Game, ix: f64, iz: f64| -> f64 {
        let i = crate::scene_collision::nav_grid_idx(g, ix, iz);
        if g.tab.roomfight_room[room].cells.has(crate::Num(i)) { 30.0 } else { 1.0 }
    };
    let path = match crate::scene_collision::nav_grid_find_path(g, src, o, Some(6000.0), Some(&mut dear)) {
        Some(path) => path,
        None => return m::INF,
    };
    let mut through = 0.0;
    for w in path.iter() {
        if crate::roomfight::room_fight_in_room(g, Some(room), *w) { through += 1.0; }
    }
    path_len(src, &path) + through * 6.0
}

// ---------------------------------------------------------------- where to hold it from

/// Another door of the room: just inside it, and outside it.
#[derive(Clone, Copy)]
struct OtherDoor { i: V3, o: V3 }
/// A mate of the room: where he is or is going, and the entry he digs in on (-1: not digging).
#[derive(Clone, Copy)]
struct Mate { p: V3, entry: f64 }
/// What coverSpot's closures captured.
struct SpotCtx {
    gd: GuardId,
    deep: bool,
    man_entry: f64,
    n: [f64; 2],
    i: V3,
    b: [V3; 3],
    k: Vec<V3>,
    outs: [V3; 3],
    others: Vec<OtherDoor>,
    mates: Vec<Mate>,
    alone: bool,
    pos: V3,
}

/// Which side of the opening: sideOf at digin.ts:288.
fn spot_side(x: &SpotCtx, p: V3) -> f64 { m::sign(x.n[0] * (p[2] - x.i[2]) - x.n[1] * (p[0] - x.i[0])) }

/// The place's score; `hope`: asking no line, the best it could come to; `bar`: give up below this. score at digin.ts:293.
fn spot_score(g: &mut Game, x: &SpotCtx, p: V3, self_: bool, hope: bool, bar: f64) -> Option<f64> {
    let d = flat(p, x.i);
    if !self_ && (d < 2.2 || d > 12.0) { return None; }
    let fail = if self_ { Some(-5.0) } else { None };
    if !hope && crate::lanes::lane_covers(g, x.gd, p) { return fail; }
    if !hope && !self_ {
        let dx = x.pos[0] - p[0];
        let dz = x.pos[2] - p[2];
        let l = m::hyp2(dx, dz);
        if l > 0.3 && crate::lanes::lane_covers(g, x.gd, [p[0] + dx / l * 0.3, p[1], p[2] + dz / l * 0.3]) { return None; }
    }
    let mut s: f64;
    if x.deep {
        let eye = at(p, 1.6);
        if !hope {
            if !crate::scene_collision::segment_blocked(g, eye, x.i, None, None, None) { return fail; }
            for b in x.b.iter() {
                if !crate::scene_collision::segment_blocked(g, *b, eye, None, None, None) { return fail; }
            }
        }
        let mut sees = 0.0;
        if hope {
            sees = x.k.len() as f64;
        } else {
            for k in x.k.iter() {
                if !crate::scene_collision::segment_blocked(g, at(p, 1.5), *k, None, None, None) { sees += 1.0; }
            }
        }
        if !hope && sees < 2.0 { return fail; }
        s = 0.4 * sees + 0.5 * m::max(0.0, 1.0 - m::abs(d - 5.0) / 5.0);
    } else {
        if !hope && crate::scene_collision::segment_blocked(g, at(p, 1.5), x.i, None, None, None) { return fail; }
        s = 0.9 * m::min(1.0, d / 9.0) + (if flat(p, x.b[1]) > 8.4 { 0.8 } else { 0.0 });
    }
    let mut walls = 0.0;
    for a in 0..8 {
        let a = a as f64;
        let wx = p[0] + m::cos(a * m::PI / 4.0) * 0.6;
        let wz = p[2] + m::sin(a * m::PI / 4.0) * 0.6;
        if crate::scene_collision::nav_grid_is_blocked(g, wx, wz) { walls += 1.0; }
    }
    let back = m::min(4.0, walls) * 0.15;
    let cut = bar > -m::INF && !hope;
    let far = if cut { 0.06 * flat(p, x.pos) } else { 0.0 };
    if cut {
        let o = spot_outside(g, x, p, s, true);
        match spot_rest(g, x, p, self_, o, back, true) {
            None => return None,
            Some(best) => { if best - far < bar { return None; } }
        }
    }
    s = spot_outside(g, x, p, s, hope);
    if cut {
        match spot_rest(g, x, p, self_, s, back, true) {
            None => return None,
            Some(best) => { if best - far < bar { return None; } }
        }
    }
    spot_rest(g, x, p, self_, s, back, hope)
}

/// Not seen from outside the door: outside at digin.ts:318.
fn spot_outside(g: &mut Game, x: &SpotCtx, p: V3, s: f64, hope: bool) -> f64 {
    let mut s = s;
    for (k, o) in x.outs.iter().enumerate() {
        if hope || crate::scene_collision::segment_blocked(g, *o, at(p, 1.2), None, None, None) {
            s += if k == 0 { 1.2 } else { 0.6 };
        }
    }
    s
}

/// The rest of the sum: rest at digin.ts:319.
fn spot_rest(g: &mut Game, x: &SpotCtx, p: V3, self_: bool, s: f64, back: f64, hope: bool) -> Option<f64> {
    let mut s = s + back;
    for o in x.others.iter() {
        if flat(p, o.i) < 2.0 {
            s -= 1.5;
        } else if x.alone && (hope || !crate::scene_collision::segment_blocked(g, at(p, 1.5), o.i, None, None, None)) {
            s += 0.7;
        }
        if !hope && !crate::scene_collision::segment_blocked(g, o.o, at(p, 1.2), None, None, None) { s -= 0.5; }
    }
    for mt in x.mates.iter() {
        if !self_ && flat(p, mt.p) < 1.6 { return None; }
        if mt.entry == x.man_entry && spot_side(x, p) == spot_side(x, mt.p) { s -= 0.6; }
    }
    Some(s)
}

/// Deep: the middle of the crossing floor he has in his sights from `p`. lookFrom at digin.ts:330.
fn spot_look(g: &mut Game, x: &SpotCtx, p: V3) -> Option<V3> {
    if !x.deep { return None; }
    let (mut sx, mut sz, mut k) = (0.0, 0.0, 0.0);
    for q in x.k.iter() {
        if !crate::scene_collision::segment_blocked(g, at(p, 1.5), *q, None, None, None) {
            sx += q[0];
            sz += q[2];
            k += 1.0;
        }
    }
    if k != 0.0 { Some([sx / k, 0.0, sz / k]) } else { None }
}

/// A cell and its hope: `pre` at digin.ts:337.
#[derive(Clone, Copy)]
struct PreCell { p: V3, n: f64, hope: f64 }
/// A real score: `cand` at digin.ts:345.
#[derive(Clone, Copy)]
struct CandCell { p: V3, s: f64, n: f64 }

/// The spot in the room to hold his door from, by the five best places, best hope first. (digin.ts coverSpot)
pub fn cover_spot(g: &mut Game, d: RoomfightDefId, m: crate::digin::Man, r: &crate::digin::RoomSt, deep: bool) -> crate::digin::CoverSpot {
    let room = r.room;
    let entries = g.tab.roomfight_room[room].entries.clone();
    let man = g.tab.roomfight_def[d].gd;
    let c = g.gd[man].ch;
    let e = if m.entry >= 0.0 && (m.entry as usize) < entries.len() { entries[m.entry as usize] } else {
        return crate::digin::CoverSpot { spot: None, here: false, look: None };
    };
    let fr = crate::guards::frame_of(g, e.door);
    let n: [f64; 2] = [-e.out[0], -e.out[1]];
    let i: V3 = [e.at[0] + n[0] * 0.5, 1.2, e.at[2] + n[1] * 0.5];
    let b: [V3; 3] = [[e.at[0] + n[0] * 1.6, 0.5, e.at[2] + n[1] * 1.6], [e.at[0] + n[0] * 1.6, 1.6, e.at[2] + n[1] * 1.6], [e.at[0] + n[0] * 2.4, 1.2, e.at[2] + n[1] * 2.4]];
    let mut k_: Vec<V3> = Vec::new();
    for fd in [3.0, 4.5] {
        for l in [-2.2, 0.0, 2.2] {
            let k: V3 = [e.at[0] + n[0] * fd + fr.ux * l, 1.2, e.at[2] + n[1] * fd + fr.uz * l];
            if crate::roomfight::room_fight_in_room(g, Some(room), k) && !crate::scene_collision::nav_grid_is_blocked(g, k[0], k[2]) { k_.push(k); }
        }
    }
    let outs: [V3; 3] = [[e.at[0] + e.out[0] * 1.6, 1.5, e.at[2] + e.out[1] * 1.6], [e.at[0] + e.out[0] * 1.3 + fr.ux, 1.5, e.at[2] + e.out[1] * 1.3 + fr.uz], [e.at[0] + e.out[0] * 1.3 - fr.ux, 1.5, e.at[2] + e.out[1] * 1.3 - fr.uz]];
    let mut others: Vec<OtherDoor> = Vec::new();
    for (ix, o) in entries.iter().enumerate() {
        if ix as f64 == m.entry { continue; }
        others.push(OtherDoor { i: [o.at[0] - o.out[0] * 0.5, 1.2, o.at[2] - o.out[1] * 0.5], o: [o.at[0] + o.out[0] * 1.6, 1.5, o.at[2] + o.out[1] * 1.6] });
    }
    let mut mates: Vec<Mate> = Vec::new();
    for id in g.fight.defs.clone() {
        let (x_gd, x_room, x_station) = { let x = &g.tab.roomfight_def[id]; (x.gd, x.room, x.station) };
        if id == d || g.gd[x_gd].state == crate::game::GuardState::Dead || x_room != Some(room) { continue; }
        let row = state_of(g).men.get(x_gd).copied();
        let p = match row.and_then(|m| m.spot) { Some(s) => s, None => x_station };
        let entry = match row { Some(m) if m.phase != crate::digin::Phase::Idle => m.entry, _ => -1.0 };
        mates.push(Mate { p, entry });
    }
    let alone = mates.is_empty();
    let pos = g.ch[c].pos;
    let x = SpotCtx { gd: man, deep, man_entry: m.entry, n, i, b, k: k_, outs, others, mates, alone, pos };

    let here = spot_score(g, &x, pos, true, false, -m::INF).unwrap_or(-5.0);

    let cells: Vec<crate::Num> = g.tab.roomfight_room[room].cells.iter().collect();
    let (nav_w, nav_cell) = (g.col.nav.w, g.col.nav.cell);
    let mut pre: Vec<PreCell> = Vec::new();
    let mut nth = 0.0;
    for ci in cells {
        let ci = ci.0;
        let p: V3 = [(js::rem(ci, nav_w) + 0.5) * nav_cell, 0.0, (js::to_i32(ci / nav_w) as f64 + 0.5) * nav_cell];
        if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { continue; }
        let cell_n = nth;
        nth += 1.0;
        let h = match spot_score(g, &x, p, false, true, -m::INF) { Some(h) => h, None => continue };
        pre.push(PreCell { p, n: cell_n, hope: h - 0.06 * flat(p, pos) });
    }
    js::sort_by(&mut pre, |a, b| b.hope - a.hope);
    let mut cand: Vec<CandCell> = Vec::new();
    let mut top: Vec<f64> = Vec::new();
    let mut fifth = -m::INF;
    for q in pre.iter() {
        if top.len() >= 5 && q.hope < fifth { break; }
        let s0 = match spot_score(g, &x, q.p, false, false, fifth) { Some(s) => s, None => continue };
        let s = s0 - 0.06 * flat(q.p, pos);
        cand.push(CandCell { p: q.p, s, n: q.n });
        let mut k = top.len();
        while k > 0 && top[k - 1] < s { k -= 1; }
        if k < 5 {
            top.insert(k, s);
            if top.len() > 5 { top.truncate(5); }
            if top.len() == 5 { fifth = top[4]; }
        }
    }
    js::sort_by(&mut cand, |a, b| js::or(b.s - a.s, a.n - b.n));
    for k in cand.iter().take(5) {
        if k.s <= here + g.k.digin.better { break; }
        let path = match crate::scene_collision::nav_grid_find_path(g, pos, k.p, None, None) {
            Some(path) if !path.is_empty() => path,
            _ => continue,
        };
        let last = path[path.len() - 1];
        if flat(last, k.p) > 0.8 || path_len(pos, &path) > 18.0 { continue; }
        let mut bad = false;
        for w in path.iter() {
            if flat(*w, e.at) < 0.9 || !crate::roomfight::room_fight_in_room(g, Some(room), *w) && flat(*w, k.p) > 0.8 {
                bad = true;
                break;
            }
        }
        if bad { continue; }
        let look = spot_look(g, &x, k.p);
        return crate::digin::CoverSpot { spot: Some(k.p), here: here > -5.0, look };
    }
    let look = if here > -5.0 { spot_look(g, &x, pos) } else { None };
    crate::digin::CoverSpot { spot: None, here: here > -5.0, look }
}

/// His side has met a stun canister: his own ears, or the word on the net. (digin.ts knowsBangs)
pub fn knows_bangs(g: &mut Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    if g.gd[gd].witness.heard.bang > 0.0 { return true; }
    let bang_call = state_of(g).bang_call;
    !bang_call.is_nan() && g.time - bang_call > 1.0
}

/// Off the door's line: every second living man of the room. (digin.ts wantsDeep)
pub fn wants_deep(g: &mut Game, d: RoomfightDefId, r: &crate::digin::RoomSt) -> bool {
    let me = g.tab.roomfight_def[d].gd;
    let mut at_ = 0usize;
    for id in r.men.iter() {
        if g.gd[*id].state == crate::game::GuardState::Dead { continue; }
        if *id == me { return at_ % 2 == 1; }
        at_ += 1;
    }
    false
}

// ---------------------------------------------------------------- moving

/// The room fight's own 'move' mode, by his script's legs. (digin.ts move)
pub fn move_(g: &mut Game, d: RoomfightDefId, goal: V3, speed: f64) {
    let gd = g.tab.roomfight_def[d].gd;
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let row = &mut g.tab.roomfight_def[d];
    row.mode = crate::roomfight::DefMode::Move;
    row.move_kind = crate::roomfight::DefMoveKind::Fallback;
    row.came_by = None;
    row.move_t = 0.0;
    row.stall_at = pos;
    row.stall_t = 0.0;
    g.gd[gd].pinned = false;
    crate::squad::order(g, gd, crate::squad::OrderPatch {
        goal: Some(Some(goal)),
        speed: Some(Some(speed)),
        face: Some(None),
        strafe: Some(Some(false)),
        aim_at: Some(None),
        upper: Some(Some(crate::character::UpperMode::Aim)),
        stance: Some(Some(crate::character::Stance::None)),
        crouch: Some(Some(false)),
        ..Default::default()
    });
}

/// The fear he keeps on his door. (digin.ts fear)
pub fn fear(g: &mut Game, d: RoomfightDefId, m: crate::digin::Man) {
    let fear = g.k.digin.fear;
    let ev = &mut g.tab.roomfight_def[d].ev;
    if m.entry >= 0.0 && m.entry < ev.len() as f64 {
        let i = m.entry as usize;
        ev[i] = m::max(ev[i], fear);
    }
}

/// The nearest unused furniture of his room within reach of him and of the door. (digin.ts nearestProp)
pub fn nearest_prop(g: &mut Game, d: RoomfightDefId, e: crate::roomfight::Entry, skip: Option<&JsSet<PropId>>) -> Option<PropId> {
    let (gd, room) = { let row = &g.tab.roomfight_def[d]; (row.gd, row.room) };
    let c = g.gd[gd].ch;
    let mut best = None;
    let mut bd = m::INF;
    for p in g.props.props.clone() {
        if state_of(g).used.has(p) || skip.map_or(false, |s| s.has(p)) { continue; }
        let pp: V3 = [crate::props::prop_x(g, p), 0.0, crate::props::prop_z(g, p)];
        if !crate::roomfight::room_fight_in_room(g, room, pp) {
            let inside = match crate::roomfight::room_fight_free_near(g, pp) {
                Some(q) => crate::roomfight::room_fight_in_room(g, room, q),
                None => false,
            };
            if !inside { continue; }
        }
        let a = flat(g.ch[c].pos, pp);
        let b = flat(pp, e.at);
        if a > g.k.digin.prop_m || b > g.k.digin.prop_m { continue; }
        if a + b < bd {
            bd = a + b;
            best = Some(p);
        }
    }
    best
}

/// The furniture's noise: sound, an event, a cue if the team hears it, a teammate's call. (digin.ts scrape)
pub fn scrape(g: &mut Game, gd: GuardId, m: crate::digin::Man, r: Option<&mut crate::digin::RoomSt>, thud: Option<bool>) {
    let thud = thud.unwrap_or(false);
    let p = m.prop.unwrap();
    let (px, pz) = (crate::props::prop_x(g, p), crate::props::prop_z(g, p));
    let pos: V3 = [px, 0.3, pz];
    let level = if thud { m::min(1.0, g.k.digin.scrape_level + 0.1) } else { g.k.digin.scrape_level };
    let tune_rate = if thud { g.tab.props_prop[p].tune.thud_rate } else { g.tab.props_prop[p].tune.scrape_rate };
    let rate = tune_rate * (0.95 + crate::math_dice::sim_random(g) * 0.1);
    seam_audio_play(g, if thud { "propThud" } else { "propScrape" }, pos, if thud { 0.6 } else { 0.5 }, rate);
    let c = g.gd[gd].ch;
    let (time, who, serial) = (g.time, g.ch[c].id, g.serial());
    g.events.push(crate::game::GameEvent {
        kind: crate::director::EventKind::Prop,
        pos: [px, 0.0, pz],
        time,
        loud: false,
        level: Some(level),
        who: Some(who),
        serial,
        ..crate::game::GameEvent::zeroed()
    });
    g.stats.digin.scrapes += 1;
    let l = match heard_noise(g, pos, level) {
        Some(l) => l,
        None => return,
    };
    let mut r = r;
    cue(g, r.as_deref_mut(), crate::digin::Cue::Scrape);
    if let (Some(by), Some(rm)) = (l.gd, r) {
        if !rm.callout {
            rm.callout = true;
            g.stats.digin.callouts += 1;
            let line = one_of(g, &["movement inside. they're blocking the door.", "furniture moving in there. they know we're coming."]);
            crate::game::game_say(g, by, line, Some(true), None, None);
            log(g, Some(by), "calls the furniture he hears");
        }
    }
}

/// The furniture goes under the handle: the door's own lock, against its far face, to a lock no ring fits. (digin.ts wedge)
pub fn wedge(g: &mut Game, d: RoomfightDefId, m: crate::digin::Man, r: &mut crate::digin::RoomSt, e: &mut crate::roomfight::Entry) {
    let gd = g.tab.roomfight_def[d].gd;
    let door = e.door;
    if !state_of(g).wedges.has(door) {
        let row = crate::digin::StWedgesValue { key_side: g.dr[door].key_side, lock_id: g.dr[door].lock_id.clone(), kicked: false, tried: false, by_t: m::NAN };
        state_of(g).wedges.set(door, row);
    }
    let side = crate::doors::doors_side(g, door, [e.at[0] + e.out[0] * 0.8, 0.0, e.at[2] + e.out[1] * 0.8]);
    g.dr[door].key_side = side;
    let name = g.dr[door].def.name.clone();
    g.dr[door].lock_id = alloc::format!("wedge:{}", name);
    crate::doors::door_set_locked(g, door, true);
    crate::doors::door_set_pick(g, door, 0.0);
    r.wedged += 1.0;
    if !crate::doors::doors_pull_face(g, door, [e.at[0] - e.out[0] * 0.8, 0.0, e.at[2] - e.out[1] * 0.8]) {
        g.stats.digin.wedged_away += 1;
    }
    g.stats.digin.braced += 1;
    scrape(g, gd, m, Some(r), Some(true));
    log(g, Some(gd), &alloc::format!("wedges {}", name));
}

/// May this door be wedged. (digin.ts wedgeable)
pub fn wedgeable(g: &mut Game, d: DoorId) -> bool {
    !g.dr[d].def.steel.unwrap_or(false)
        && !crate::doors::door_locked(g, d)
        && !crate::doors::door_lock_broken(g, d)
        && !g.dr[d].def.locked.unwrap_or(false)
        && crate::devices::devices_pad_of(g, d).is_none()
}

/// Where his own ears last put them. (digin.ts heardAt)
pub fn heard_at(g: &mut Game, d: RoomfightDefId, m: crate::digin::Man) -> V3 {
    let row = &g.tab.roomfight_def[d];
    match row.noise_at {
        Some(at_) if row.noise_out && row.noise_t > m.know_t => at_,
        _ => m.know,
    }
}

// ---------------------------------------------------------------- doors (DIGIN.shutOpen is settled off on the page: the rule folds away)

/// The rule of the doors that stay as left: off on the page. (digin.ts shutsOpen)
pub fn shuts_open(g: &mut Game) -> bool { false }

/// The entry his hands are on or he is on his way to. (digin.ts doorOf)
pub fn door_of(g: &mut Game, m: crate::digin::Man) -> f64 {
    if m.brace >= 0.0 { m.brace } else { m.entry }
}

/// Seconds a hand waits on a pulled leaf. (digin.ts latchSecs)
pub fn latch_secs(g: &mut Game) -> f64 { 2.5 }

/// His hand goes on the leaf: pulled to, the wait begun. (digin.ts handOn)
pub fn hand_on(g: &mut Game, d: RoomfightDefId, m: &mut crate::digin::Man, d_2: DoorId) {
    let gd = g.tab.roomfight_def[d].gd;
    let c = g.gd[gd].ch;
    let who = g.ch[c].id;
    crate::doors::doors_pull_to(g, d_2, who);
    g.stats.digin.pulled_to += 1;
    let now = g.time;
    m.phase = crate::digin::Phase::Shut;
    m.until = now + latch_secs(g);
    m.hand_t = Some(now);
    m.pull_t = Some(now);
    m.pushes = Some(0.0);
    m.pull_a = Some(m::abs(crate::doors::door_angle(g, d_2)));
    m.stalls = Some(0.0);
}

/// …and stays on it: true = still waiting for the latch. (digin.ts handWaits)
pub fn hand_waits(g: &mut Game, d: RoomfightDefId, m: &mut crate::digin::Man, d_2: DoorId) -> bool {
    !(crate::doors::door_latched(g, d_2) || g.time > m.until)
}

/// They are at a door of his room by the sound of it. (digin.ts atADoor)
pub fn at_a_door(g: &mut Game, d: RoomfightDefId, m: crate::digin::Man, r: &crate::digin::RoomSt) -> bool {
    let heard = heard_at(g, d, m);
    let close = g.k.digin.close_m;
    g.tab.roomfight_room[r.room].entries.iter().any(|e| flat(heard, e.at) <= close)
}

/// The nearest other open door of his room: never, with the rule off on the page. (digin.ts shutMore)
pub fn shut_more(g: &mut Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &crate::digin::RoomSt, not: f64) -> bool { false }

#[cfg(test)]
mod tests {
    use super::*;
    // the bits from the TypeScript's own lines under bun with EXACT.on
    #[test]
    fn flat_and_path_len_have_the_typescripts_bits() {
        assert_eq!(flat([1.1, 9.0, 2.3], [-4.7, 0.0, 8.9]).to_bits(), 0x4021929cde1931e6);
        assert_eq!(path_len([0.1, 0.0, 0.2], &[[1.3, 0.0, 2.7], [3.9, 5.0, -1.1], [-2.25, 0.0, 0.7]]).to_bits(), 0x402b92245733eab8);
    }
    #[test]
    fn heard_at_takes_his_ears_only_for_a_newer_noise_outside() {
        let mut g = Game::blank();
        let d = g.tab.roomfight_def.push(crate::roomfight::Def::zeroed());
        let mut man = crate::digin::Man::zeroed();
        man.know = [1.0, 0.0, 1.0];
        man.know_t = 5.0;
        assert_eq!(heard_at(&mut g, d, man), man.know);
        g.tab.roomfight_def[d].noise_at = Some([2.0, 0.0, 2.0]);
        g.tab.roomfight_def[d].noise_t = 6.0;
        assert_eq!(heard_at(&mut g, d, man), man.know);
        g.tab.roomfight_def[d].noise_out = true;
        assert_eq!(heard_at(&mut g, d, man), [2.0, 0.0, 2.0]);
        g.tab.roomfight_def[d].noise_t = 5.0;
        assert_eq!(heard_at(&mut g, d, man), man.know);
    }
    #[test]
    fn a_write_through_the_state_lands_in_the_game() {
        let mut g = Game::blank();
        assert_eq!(state_of(&mut g).frame_t, -1.0);
        assert!(state_of(&mut g).bang_call.is_nan());
        state_of(&mut g).bang_call = 3.0;
        assert_eq!(state_of(&mut g).bang_call, 3.0);
        assert_eq!(g.m.digin.states.as_ref().unwrap().bang_call, 3.0);
    }
    #[test]
    fn fear_lands_on_his_door_of_the_room_fights_row() {
        let mut g = Game::blank();
        let d = g.tab.roomfight_def.push(crate::roomfight::Def::zeroed());
        g.tab.roomfight_def[d].ev = vec![0.5, 2.9];
        let mut man = crate::digin::Man::zeroed();
        man.entry = 0.0;
        fear(&mut g, d, man);
        man.entry = 1.0;
        fear(&mut g, d, man);
        man.entry = -1.0;
        fear(&mut g, d, man);
        assert_eq!(g.tab.roomfight_def[d].ev, vec![g.k.digin.fear, 2.9]);
    }
}
