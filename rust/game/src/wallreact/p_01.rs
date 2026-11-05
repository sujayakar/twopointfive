//! src/game/wallreact.ts lines 1 to 602
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small helpers

fn flat(a: V3, b: V3) -> f64 { v3::dist_xz(a, b) }
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn player_pos(g: &Game) -> V3 { g.ch[g.pl[g.player].ch].pos }
fn char_id(g: &Game, gd: GuardId) -> f64 { g.ch[g.gd[gd].ch].id }
fn is_dead(g: &Game, gd: GuardId) -> bool { g.gd[gd].state == crate::game::GuardState::Dead }
fn is_op(g: &Game, gd: GuardId) -> bool { g.gd[gd].team == crate::teams::Team::Operative }
fn breaches_of(g: &Game) -> Vec<WallreactBreachId> { g.m.wallreact.wall_react.breaches.clone() }
fn say(g: &mut Game, gd: GuardId, text: &str) { crate::game::game_say(g, gd, text, None, None, None) }

/// `g.seats.some(s => (s.view.wallCharge?.stats.blown ?? 0) > 0)`   (wallreact.ts:324)
fn any_seat_blown(g: &mut Game) -> bool {
    for s in g.seats.clone() {
        let pl = crate::cmd::seat_man(g, s);
        if crate::seats::with_seat(g, pl, &mut |g| g.wall_charge.stats.blown > 0.0) { return true; }
    }
    false
}

/// `x.toFixed(d)`: rounds an exact tie up in magnitude, where Rust's formatter takes the even digit; -0 has no sign, a negative rounding to nothing keeps it
fn fixed(x: f64, d: usize) -> String {
    if x.is_nan() { return String::from("NaN"); }
    let a = m::abs(x);
    let y = a * m::pow(2.0, (d + 1) as f64);
    let tie = y == m::floor(y) && js::rem(y, 2.0) == 1.0;
    let t = if tie { f64::from_bits(a.to_bits() + 1) } else { a };
    if x < 0.0 { format!("-{:.*}", d, t) } else { format!("{:.*}", d, t) }
}

/// (wallreact.ts wrap)
pub fn wrap(a: f64) -> f64 {
    let mut a = js::rem(a + m::PI, 2.0 * m::PI);
    if a <= 0.0 { a += 2.0 * m::PI; }
    a - m::PI
}

/// (wallreact.ts note)
pub fn note(g: &mut Game, s: &str) {
    let line = format!("{} {}", fixed(g.time, 2), s);
    let log = &mut g.m.wallreact.wallreact_log;
    log.push(line);
    if log.len() > 400 { log.drain(0..200); }
}

/// does the plan segment p → q cross the rect inflated by `pad`   (wallreact.ts segRect)
pub fn seg_rect(p: V3, q: V3, r: [f64; 4], pad: f64) -> bool {
    let (x0, z0, x1, z1) = (r[0] - pad, r[1] - pad, r[2] + pad, r[3] + pad);
    let (mut t0, mut t1) = (0.0, 1.0);
    let (dx, dz) = (q[0] - p[0], q[2] - p[2]);
    for [d, o, lo, hi] in [[dx, p[0], x0, x1], [dz, p[2], z0, z1]] {
        if m::abs(d) < 1e-9 {
            if o < lo || o > hi { return false; }
            continue;
        }
        let (mut a, mut b) = ((lo - o) / d, (hi - o) / d);
        if a > b { core::mem::swap(&mut a, &mut b); }
        t0 = m::max(t0, a);
        t1 = m::min(t1, b);
        if t0 > t1 { return false; }
    }
    true
}

/// (wallreact.ts rectDist)
pub fn rect_dist(p: V3, r: [f64; 4]) -> f64 {
    m::hyp2(m::max_n(&[r[0] - p[0], 0.0, p[0] - r[2]]), m::max_n(&[r[1] - p[2], 0.0, p[2] - r[3]]))
}

/// (wallreact.ts WallReact.constructor)
pub fn wall_react_new(g: &mut Game) -> WallReact {
    let mut s = WallReact::blank();
    crate::wallbreak::wall_break_on_change(g, &crate::wallbreak::ChangeListener::WallReact);
    s.unsub = true;
    s
}

/// (wallreact.ts WallReact.live)
pub fn wall_react_live(g: &mut Game) -> bool {
    g.m.wallreact.wall_react.unsub && g.ai_enabled && !g.quiet_utility && !g.touring
}

/// (wallreact.ts WallReact.fight): nothing to hand back: the reader reads g.fight
pub fn wall_react_fight(g: &mut Game) {}

/// (wallreact.ts WallReact.defOf)
pub fn wall_react_def_of(g: &mut Game, gd: GuardId) -> Option<RoomfightDefId> {
    g.fight.defs.iter().copied().find(|d| g.tab.roomfight_def[*d].gd == gd)
}

/// a restart: nothing known, nobody owed a move   (wallreact.ts WallReact.reset)
pub fn wall_react_reset(g: &mut Game) {
    let s = &mut g.m.wallreact.wall_react;
    s.pend.clear();
    s.minds.clear();
    s.watching.clear();
    s.last = -1.0;
    s.last_ev = -1.0;
    s.last_sweep = -1.0;
    for b in breaches_of(g) {
        let r = &mut g.tab.wallreact_breach[b];
        r.ops = -1.0;
        r.men.clear();
        r.said.clear();
        r.tell.clear();
    }
    wall_react_drop_openings(g);
}

/// (wallreact.ts WallReact.tick)
pub fn wall_react_tick(g: &mut Game) {
    use crate::wallbreak::LevelChangeKind as Kind;
    if g.m.wallreact.wall_react.stale {
        let s = &mut g.m.wallreact.wall_react;
        let mut k = 0;
        for (i, r) in s.queue.iter().enumerate() { if r.kind == Kind::Reset { k = i + 1; } }
        s.queue.drain(0..k);
        s.stale = false;
        wall_react_unflood(g);
        wall_react_reset(g);
        g.m.wallreact.wall_react.breaches.clear();
    }
    if !wall_react_live(g) { g.m.wallreact.wall_react.queue.clear(); return; }
    let rows = core::mem::take(&mut g.m.wallreact.wall_react.queue);
    let all: Vec<&crate::wallbreak::LevelChange> = rows.iter().filter(|r| r.kind != Kind::Reset && r.kind != Kind::Door && !r.opened.is_empty()).collect();
    g.m.wallreact.wall_react.flood_all = if all.len() > 1 { Some(all.iter().flat_map(|r| r.opened.iter().copied()).collect()) } else { None };
    for row in &rows { wall_react_take(g, row); }
    g.m.wallreact.wall_react.flood_all = None;
    if g.m.wallreact.wall_react.breaches.is_empty() && g.m.wallreact.wall_react.pend.is_empty() { return; }
    let now = g.time;
    let last_ev = g.m.wallreact.wall_react.last_ev;
    if last_ev < 0.0 || now - last_ev >= 0.1 || now < last_ev {
        let dt = if last_ev < 0.0 || now < last_ev { 0.1 } else { m::min(0.5, now - last_ev) };
        g.m.wallreact.wall_react.last_ev = now;
        wall_react_evidence(g, dt);
    }
    let last = g.m.wallreact.wall_react.last;
    if last >= 0.0 && now >= last && now - last < 0.25 { return; }
    g.m.wallreact.wall_react.last = now;
    wall_react_notice(g);
    wall_react_book(g);
    let last_sweep = g.m.wallreact.wall_react.last_sweep;
    if now - last_sweep >= 1.0 || now < last_sweep {
        g.m.wallreact.wall_react.last_sweep = now;
        wall_react_sweep(g);
    }
    // (nothing a question calls adds to the list or takes from it: the row is lent out and put back)
    let mut i = g.m.wallreact.wall_react.pend.len();
    while i > 0 {
        i -= 1;
        let mut p = g.m.wallreact.wall_react.pend[i].clone();
        if now < p.next { continue; }
        let r = wall_react_reconsider(g, &mut p);
        if r != WallReactReconsider::Later || now > p.until {
            g.m.wallreact.wall_react.pend.remove(i);
        } else {
            p.next = now + 0.5;
            g.m.wallreact.wall_react.pend[i] = p;
        }
    }
}

// ================================================================ the feed

/// (wallreact.ts:209 own) the man whose aimed fire opened it: it is not his cover that went
fn take_own(g: &mut Game, gd: GuardId, row: &crate::wallbreak::LevelChange) -> bool {
    row.cause == crate::wallbreak::LevelChangeCause::Fire && char_id(g, gd) == row.who && crate::coverfire::coverfire_on(g, gd).is_some()
}

/// (wallreact.ts:204 ask) one question a man: the newest thing, what went before it waiting behind
fn take_ask(g: &mut Game, row: &crate::wallbreak::LevelChange, b: Option<WallreactBreachId>, asked: &mut JsSet<GuardId>, gd: GuardId) {
    if asked.has(gd) { return; }
    asked.add(gd);
    let i = g.m.wallreact.wall_react.pend.iter().position(|x| x.gd == gd);
    let old = i.map(|i| g.m.wallreact.wall_react.pend[i].clone());
    let mut p = Pend { gd, also: None, until: g.time + g.k.wallreact.retry_secs, next: g.time, rect: row.rect, b, who: row.who };
    if let Some(old) = old {
        let mut ob = old.b;
        while let Some(o) = ob {
            match g.tab.wallreact_breach[o].into { Some(n) => ob = Some(n), None => break }
        }
        let a = crate::seats::order_of(g, gd);
        let at = match a.and_then(|a| g.tab.orders_assignment[a].post) { Some(post) => post, None => pos_of(g, gd) };
        let mut list = vec![Gone { rect: old.rect, b: ob, who: old.who }];
        list.extend(old.also.unwrap_or_default());
        p.also = Some(wall_react_kept(g, at, &mut list));
    }
    match i {
        Some(i) => g.m.wallreact.wall_react.pend[i] = p,
        None => g.m.wallreact.wall_react.pend.push(p),
    }
}

/// (wallreact.ts WallReact.take)
pub fn wall_react_take(g: &mut Game, row: &crate::wallbreak::LevelChange) {
    use crate::wallbreak::LevelChangeKind as Kind;
    if row.kind == Kind::Reset || row.kind == Kind::Door { return; }
    g.stats.wallreact.rows += 1;
    let b = if row.kind == Kind::Cover { None } else { wall_react_upsert(g, row) };
    if !row.opened.is_empty() {
        match g.m.wallreact.wall_react.flood_all.clone() {
            None => wall_react_reflood(g, &row.opened),
            Some(all) => if !all.is_empty() {
                wall_react_reflood(g, &all);
                g.m.wallreact.wall_react.flood_all = Some(Vec::new());
            },
        }
    }
    if let Some(b) = b { wall_react_heard(g, b, row); }
    let mut asked: JsSet<GuardId> = JsSet::new();
    let near = g.k.wallreact.near_m;
    for gd in g.guards.clone() {
        if !is_dead(g, gd) && !take_own(g, gd, row) && rect_dist(pos_of(g, gd), row.rect) <= near { take_ask(g, row, b, &mut asked, gd); }
    }
    for it in wall_react_posts(g) {
        if take_own(g, it.gd, row) { continue; }
        let post = g.tab.orders_assignment[it.a].post.unwrap();
        if rect_dist(post, row.rect) <= near { take_ask(g, row, b, &mut asked, it.gd); }
    }
}

/// what a question keeps behind its newest: past goneMax - 1 the furthest is let go, the oldest of two as far   (wallreact.ts WallReact.kept)
pub fn wall_react_kept(g: &mut Game, at: V3, list: &mut Vec<Gone>) -> Vec<Gone> {
    let n = g.k.wallreact.gone_max - 1.0;
    if n <= 0.0 { return Vec::new(); }
    while list.len() as f64 > n {
        let (mut k, mut far) = (0, -1.0);
        for (i, x) in list.iter().enumerate() {
            let d = rect_dist(at, x.rect);
            if d >= far { far = d; k = i; }
        }
        list.remove(k);
    }
    list.clone()
}

/// (wallreact.ts:224 ask)
fn sweep_ask(g: &mut Game, gd: GuardId, at: V3, b: WallreactBreachId) {
    if g.m.wallreact.wall_react.pend.iter().any(|p| p.gd == gd) { return; }
    let r = &g.tab.wallreact_breach[b];
    let (bat, n, u, half) = (r.at, r.n, r.u, r.half);
    let d = m::abs((at[0] - bat[0]) * n[0] + (at[2] - bat[2]) * n[1]);
    let l = m::abs((at[0] - bat[0]) * u[0] + (at[2] - bat[2]) * u[1]);
    if d > g.k.wallreact.near_m || l > half + 0.6 { return; }
    if wall_react_beyond(g, gd, b, at).is_none() { return; }
    let h = if is_op(g, gd) { 1.2 } else { 0.9 };
    if !wall_react_open_through(g, b, at, h) { return; }
    let (ax, az, bx, bz) = (bat[0] - u[0] * half, bat[2] - u[1] * half, bat[0] + u[0] * half, bat[2] + u[1] * half);
    let now = g.time;
    g.m.wallreact.wall_react.pend.push(Pend {
        gd, also: None, until: now + 1.5, next: now,
        rect: [m::min(ax, bx) - 0.1, m::min(az, bz) - 0.1, m::max(ax, bx) + 0.1, m::max(az, bz) + 0.1],
        b: Some(b), who: 0.0,
    });
}

/// once a second: a man who has since planted himself in front of a breach he knows of is asked   (wallreact.ts WallReact.sweep)
pub fn wall_react_sweep(g: &mut Game) {
    let retry = g.k.wallreact.retry_secs;
    for b in breaches_of(g) {
        for gd in g.tab.wallreact_breach[b].men.keys() {
            if is_dead(g, gd) { continue; }
            if let Some(d) = wall_react_def_of(g, gd) {
                let row = &g.tab.roomfight_def[d];
                if row.mode == crate::roomfight::DefMode::Hold && g.time - row.moved_at > retry {
                    let at = pos_of(g, gd);
                    sweep_ask(g, gd, at, b);
                }
            }
        }
        if g.tab.wallreact_breach[b].ops >= 0.0 {
            for it in wall_react_posts(g) {
                let a = &g.tab.orders_assignment[it.a];
                if a.arrived { let at = a.post.unwrap(); sweep_ask(g, it.gd, at, b); }
            }
        }
    }
}

fn cause_of(c: crate::wallbreak::LevelChangeCause) -> BreachCause {
    use crate::wallbreak::LevelChangeCause as C;
    match c { C::Blast => BreachCause::Blast, C::Fire => BreachCause::Fire, C::Load => BreachCause::Load, C::Reset => BreachCause::Reset, C::Collapse => BreachCause::Collapse }
}

/// the breach a wall's row belongs to: a new one, or the one on that wall it widens   (wallreact.ts WallReact.upsert)
pub fn wall_react_upsert(g: &mut Game, row: &crate::wallbreak::LevelChange) -> Option<WallreactBreachId> {
    use crate::wallbreak::{LevelChangeCause, LevelChangeKind};
    let w = crate::wallbreak::wall_break_all(g).into_iter().find(|x| x.id == row.id)?;
    let (c, h, yaw) = { let bx = &g.col.boxes[w.box_]; (bx.c, bx.h, bx.yaw) };
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    let along_x = h[0] >= h[2];
    let u: [f64; 2] = if along_x { [cs, -sn] } else { [sn, cs] };
    let n: [f64; 2] = if along_x { [sn, cs] } else { [cs, -sn] };
    let c0: [f64; 2] = [c[0], c[2]];
    let l = if along_x { h[0] } else { h[2] };
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    let r = row.rect;
    for [x, z] in [[r[0], r[1]], [r[2], r[1]], [r[0], r[3]], [r[2], r[3]]] {
        let s = (x - c0[0]) * u[0] + (z - c0[1]) * u[1];
        lo = m::min(lo, s);
        hi = m::max(hi, s);
    }
    lo = m::max(-l, lo);
    hi = m::min(l, hi);
    if !(hi > lo) { return None; }
    let found = breaches_of(g).into_iter().find(|x| {
        let x = &g.tab.wallreact_breach[*x];
        x.wall == row.id && lo <= x.hi + 0.3 && hi >= x.lo - 0.3
    });
    let walk = row.kind == LevelChangeKind::Hole && row.walkable;
    let b = match found {
        Some(b) => {
            let x = &mut g.tab.wallreact_breach[b];
            x.lo = m::min(x.lo, lo);
            x.hi = m::max(x.hi, hi);
            if row.cause == LevelChangeCause::Blast { x.cause = BreachCause::Blast; x.blast_by = Some(row.who); }
            if walk && !x.walk {
                x.walk = true;
                x.ops = -1.0;
                x.men.clear();
                x.said.clear();
                x.tell.clear();
            }
            b
        }
        None => {
            let b = g.tab.wallreact_breach.push(Breach {
                key: format!("breach:{}:{}", row.id, row.serial), wall: row.id, c0, u, n, lo, hi, at: [0.0, CHEST, 0.0], half: 0.0, into: None, walk, t: g.time,
                cause: cause_of(row.cause), who: row.who, blast_by: None, ops: -1.0, men: JsMap::new(), said: JsSet::new(), tell: Vec::new(), opening: None,
            });
            g.m.wallreact.wall_react.breaches.push(b);
            g.stats.wallreact.breaches += 1;
            b
        }
    };
    // (two breaches of one wall grown into each other: one)
    let mut i = g.m.wallreact.wall_react.breaches.len();
    while i > 0 {
        i -= 1;
        let o = g.m.wallreact.wall_react.breaches[i];
        if o == b { continue; }
        {
            let (ow, x) = (&g.tab.wallreact_breach[o], &g.tab.wallreact_breach[b]);
            if ow.wall != x.wall || ow.lo > x.hi + 0.3 || ow.hi < x.lo - 0.3 { continue; }
        }
        let other = g.tab.wallreact_breach[o].clone();
        let x = &mut g.tab.wallreact_breach[b];
        x.lo = m::min(x.lo, other.lo);
        x.hi = m::max(x.hi, other.hi);
        x.walk = x.walk || other.walk;
        if other.ops >= 0.0 && x.ops < 0.0 { x.ops = other.ops; }
        for (man, t) in other.men.iter() {
            if !x.men.has(man) { x.men.set(man, *t); }
        }
        for (_, mind) in g.m.wallreact.wall_react.minds.iter_mut() {
            if let Some(e) = mind.get(o).copied() {
                mind.delete(o);
                match mind.get_mut(b) {
                    Some(k) => k.ev = m::max(k.ev, e.ev),
                    None => { mind.set(b, e); }
                }
            }
        }
        // (a set of a key already there: the walk is not disturbed)
        for man in g.m.wallreact.wall_react.watching.keys() {
            if g.m.wallreact.wall_react.watching.get(man).copied() == Some(o) { g.m.wallreact.wall_react.watching.set(man, b); }
        }
        g.tab.wallreact_breach[o].into = Some(b);
        for p in g.m.wallreact.wall_react.pend.iter_mut() {
            if let Some(also) = p.also.as_mut() {
                for x in also.iter_mut() { if x.b == Some(o) { x.b = Some(b); } }
            }
        }
        if let Some(op) = other.opening { wall_react_drop_opening(g, op); }
        g.m.wallreact.wall_react.breaches.remove(i);
    }
    let x = &mut g.tab.wallreact_breach[b];
    let mid = (x.lo + x.hi) / 2.0;
    x.at[0] = c0[0] + u[0] * mid;
    x.at[2] = c0[1] + u[1] * mid;
    x.half = (x.hi - x.lo) / 2.0;
    let (at, half, walk, opening) = (x.at, x.half, x.walk, x.opening);
    let key = x.key.clone();
    if let Some(op) = opening {
        let o = &mut g.tab.cover_opening[op];
        o.at[0] = at[0];
        o.at[2] = at[2];
        o.half = half;
    }
    let text = format!("{} {} wall #{} → {} at ({}, {}) {} m {}", row.kind.as_str(), row.cause.as_str(), row.id, key, fixed(at[0], 1), fixed(at[2], 1), fixed(half * 2.0, 1), if walk { "WALK" } else { "port" });
    note(g, &text);
    Some(b)
}

// ================================================================ (c) the room fight's rooms

/// the walls are whole again: the rooms are flooded afresh, every defender's is re-read where he stands   (wallreact.ts WallReact.unflood)
pub fn wall_react_unflood(g: &mut Game) {
    g.fight.rooms.clear();
    g.fight.room_of_cell.clear();
    for d in g.fight.defs.clone() {
        let gd = g.tab.roomfight_def[d].gd;
        let at = if is_dead(g, gd) { g.tab.roomfight_def[d].station } else { pos_of(g, gd) };
        let room = crate::roomfight::room_fight_room_at(g, at);
        let ev = match room { Some(r) => vec![0.0; g.tab.roomfight_room[r].entries.len()], None => Vec::new() };
        let row = &mut g.tab.roomfight_def[d];
        row.room = room;
        row.ev = ev;
        row.watch = -1.0;
        row.want = -1.0;
        row.glance_to = -1.0;
    }
}

/// (wallreact.ts:286 same)
fn same(a: &crate::roomfight::Entry, b: &crate::roomfight::Entry) -> bool { a.door == b.door && flat(a.at, b.at) < 0.3 }

/// a room's own doors keep their places; the doors it gains come after them   (wallreact.ts:287 merged)
fn merged(old: &[crate::roomfight::Entry], fresh: &[crate::roomfight::Entry]) -> Vec<crate::roomfight::Entry> {
    let mut out: Vec<crate::roomfight::Entry> = old.iter().filter(|o| fresh.iter().any(|f| same(f, o))).copied().collect();
    out.extend(fresh.iter().filter(|f| !old.iter().any(|o| same(o, f))).copied());
    out
}

/// the first cell of a list that is not solid (wallreact.ts:290, 298)
fn first_open(g: &Game, cells: &[crate::Num]) -> Option<f64> {
    cells.iter().map(|c| c.0).find(|c| g.col.nav.blocked.get(*c as usize).copied() != Some(1))
}

/// rooms an earlier flood left sharing this room's cells (`o.cells = into.cells`, wallreact.ts:298): the TypeScript's one Set is a copy here, kept equal by hand
fn sharing_cells(g: &Game, r: RoomfightRoomId) -> Vec<RoomfightRoomId> {
    let mine = g.tab.roomfight_room[r].cells.keys();
    if mine.is_empty() { return Vec::new(); }
    g.fight.rooms.iter().copied().filter(|o| *o != r && g.tab.roomfight_room[*o].cells.keys() == mine).collect()
}

/// (wallreact.ts:303 idx)
fn entry_index(i: f64, was: &[crate::roomfight::Entry], now: &[crate::roomfight::Entry]) -> f64 {
    if i < 0.0 { return -1.0; }
    match was.get(i as usize) {
        Some(w) => now.iter().position(|e| same(e, w)).map_or(-1.0, |k| k as f64),
        None => -1.0,
    }
}

/// floor opened: the rooms it touches are flooded again; two joined by a hole are ONE, each defender's evidence carried by door   (wallreact.ts WallReact.reflood)
pub fn wall_react_reflood(g: &mut Game, opened: &[f64]) {
    use crate::Num;
    let (w, cs) = (g.col.nav.w, g.col.nav.cell);
    let mut touched: JsSet<RoomfightRoomId> = JsSet::new();
    for &c in opened {
        for n in [c, c - 1.0, c + 1.0, c - w, c + w] {
            if let Some(r) = g.fight.room_of_cell.get(Num(n)).copied() { touched.add(r); }
        }
    }
    if touched.is_empty() { return; }
    let mut olds = touched.keys();
    js::sort_by(&mut olds, |a, b| g.tab.roomfight_room[*a].id - g.tab.roomfight_room[*b].id);
    let mut before: JsMap<RoomfightRoomId, (Vec<Num>, Vec<crate::roomfight::Entry>)> = JsMap::new();
    for &r in &olds {
        let row = &g.tab.roomfight_room[r];
        before.set(r, (row.cells.keys(), row.entries.clone()));
    }
    for &r in &olds {
        for c in g.tab.roomfight_room[r].cells.keys() {
            if g.fight.room_of_cell.get(c).copied() == Some(r) { g.fight.room_of_cell.delete(c); }
        }
    }
    let mut now: JsMap<RoomfightRoomId, RoomfightRoomId> = JsMap::new();
    for &r in &olds {
        if now.has(r) { continue; }
        let seed = match first_open(g, &before.get(r).unwrap().0) { Some(s) => s, None => { now.set(r, r); continue; } };
        let n0 = g.fight.rooms.len();
        let at = [(js::rem(seed, w) + 0.5) * cs, 0.0, (js::to_i32(seed / w) as f64 + 0.5) * cs];
        let nr = match crate::roomfight::room_fight_room_at(g, at) { Some(nr) => nr, None => { now.set(r, r); continue; } };
        let mut into = nr;
        if g.fight.rooms.len() > n0 && g.fight.rooms.last().copied() == Some(nr) {
            // (a fresh flood: it lives on as the Room the men already hold)
            g.fight.rooms.pop();
            let sharing = sharing_cells(g, r);
            g.tab.roomfight_room[r].cells.clear();
            for c in g.tab.roomfight_room[nr].cells.keys() {
                g.tab.roomfight_room[r].cells.add(c);
                g.fight.room_of_cell.set(c, r);
            }
            let cells = g.tab.roomfight_room[r].cells.clone();
            for o in sharing { g.tab.roomfight_room[o].cells = cells.clone(); }
            let e = merged(&before.get(r).unwrap().1, &g.tab.roomfight_room[nr].entries);
            g.tab.roomfight_room[r].entries = e;
            into = r;
        }
        now.set(r, into);
        for &o in &olds {
            if now.has(o) { continue; }
            let c0 = first_open(g, &before.get(o).unwrap().0);
            if let Some(c0) = c0 {
                if g.tab.roomfight_room[into].cells.has(Num(c0)) {
                    let cells = g.tab.roomfight_room[into].cells.clone();
                    let e = merged(&before.get(o).unwrap().1, &g.tab.roomfight_room[into].entries);
                    let row = &mut g.tab.roomfight_room[o];
                    row.cells = cells;
                    row.entries = e;
                    now.set(o, into);
                    g.stats.wallreact.merged += 1;
                }
            }
        }
    }
    for d in g.fight.defs.clone() {
        let dr = match g.tab.roomfight_def[d].room { Some(r) if touched.has(r) => r, _ => continue };
        let was = &before.get(dr).unwrap().1;
        let room = now.get(dr).copied().unwrap_or(dr);
        let e = g.tab.roomfight_room[room].entries.clone();
        let row = &mut g.tab.roomfight_def[d];
        let ev: Vec<f64> = e.iter().map(|x| match was.iter().position(|y| same(y, x)) { Some(j) => row.ev.get(j).copied().unwrap_or(0.0), None => 0.0 }).collect();
        row.watch = entry_index(row.watch, was, &e);
        row.want = entry_index(row.want, was, &e);
        row.glance_to = entry_index(row.glance_to, was, &e);
        row.ev = ev;
        row.room = Some(room);
    }
    let ids: Vec<String> = olds.iter().map(|r| format!("{}", g.tab.roomfight_room[*r].id)).collect();
    let mut lived: JsSet<RoomfightRoomId> = JsSet::new();
    for (_, r) in now.iter() { lived.add(*r); }
    let lives: Vec<String> = lived.keys().iter().map(|r| {
        let row = &g.tab.roomfight_room[*r];
        format!("{} ({} cells, {} doors)", row.id, row.cells.len(), row.entries.len())
    }).collect();
    let text = format!("rooms {} flooded again → {}", ids.join("+"), lives.join(", "));
    note(g, &text);
}

// ================================================================ (b) noticing

/// (wallreact.ts WallReact.head)
pub fn wall_react_head(g: &mut Game, gd: WallReactHeadGd) -> V3 {
    let p = match gd { WallReactHeadGd::Sam => player_pos(g), WallReactHeadGd::Guard(gd) => pos_of(g, gd) };
    [p[0], EYE, p[2]]
}

/// (wallreact.ts WallReact.faceOf)
pub fn wall_react_face_of(g: &mut Game, b: WallreactBreachId, from: V3, y: Option<f64>) -> V3 {
    let y = y.unwrap_or(1.3);
    let r = &g.tab.wallreact_breach[b];
    let s = if (from[0] - r.at[0]) * r.n[0] + (from[2] - r.at[2]) * r.n[1] >= 0.0 { 1.0 } else { -1.0 };
    [r.at[0] + r.n[0] * s * 0.3, y, r.at[2] + r.n[1] * s * 0.3]
}

/// (wallreact.ts WallReact.sees)
pub fn wall_react_sees(g: &mut Game, b: WallreactBreachId, from: V3, yaw: Option<f64>) -> bool {
    let (see_m, fov) = (g.k.wallreact.see_m, g.k.wallreact.fov_deg);
    let r = &g.tab.wallreact_breach[b];
    let (at, n, u, half) = (r.at, r.n, r.u, r.half);
    if flat(from, at) > see_m { return false; }
    if let Some(yaw) = yaw {
        if m::abs(wrap(v3::yaw_to(from, at) - yaw)) > fov * m::PI / 360.0 { return false; }
    }
    let eye = [from[0], EYE, from[2]];
    let s = if (from[0] - at[0]) * n[0] + (from[2] - at[2]) * n[1] >= 0.0 { 1.0 } else { -1.0 };
    for k in [0.0, 0.6, -0.6] {
        let q = [at[0] + u[0] * half * k + n[0] * s * 0.3, 1.3, at[2] + u[1] * half * k + n[1] * s * 0.3];
        if !crate::scene_collision::segment_blocked(g, eye, q, None, None, None) { return true; }
    }
    false
}

/// (wallreact.ts WallReact.opsKnow)
pub fn wall_react_ops_know(g: &mut Game, b: WallreactBreachId, by: &str) {
    if g.tab.wallreact_breach[b].ops >= 0.0 { return; }
    g.tab.wallreact_breach[b].ops = g.time;
    g.stats.wallreact.learned_ops += 1;
    let text = format!("the team knows {} ({})", g.tab.wallreact_breach[b].key, by);
    note(g, &text);
}

/// (wallreact.ts WallReact.manKnows)
pub fn wall_react_man_knows(g: &mut Game, b: WallreactBreachId, gd: GuardId, how: WallReactManKnowsHow) {
    use WallReactManKnowsHow as How;
    if g.tab.wallreact_breach[b].men.has(gd) { return; }
    let now = g.time;
    g.tab.wallreact_breach[b].men.set(gd, now);
    let ev_open = g.k.wallreact.ev_open;
    let ev = match how { How::Told => ev_open * 0.5, How::Made => 0.0, _ => ev_open };
    let t = if how == How::Made { -1e9 }
        else if g.tab.wallreact_breach[b].cause == BreachCause::Blast && any_seat_blown(g) { m::max(now, g.gd[gd].dazzled_until) }
        else { now };
    let minds = &mut g.m.wallreact.wall_react.minds;
    if !minds.has(gd) { minds.set(gd, JsMap::new()); }
    minds.get_mut(gd).unwrap().set(b, Mind { ev, line: false, line_t: -1.0, t });
    if how == How::Told { g.stats.wallreact.told += 1; return; }
    g.stats.wallreact.learned_men += 1;
    let text = format!("{} {} {}", g.gd[gd].callsign, how.as_str(), g.tab.wallreact_breach[b].key);
    note(g, &text);
    if how == How::Made { return; }
    let at = now + g.k.wallreact.tell_secs;
    g.tab.wallreact_breach[b].tell.push(BreachTellItem { by: gd, at });
    let team = g.gd[gd].team;
    if g.tab.wallreact_breach[b].said.has(team) || g.gd[gd].state == crate::game::GuardState::Patrol { return; }
    g.tab.wallreact_breach[b].said.add(team);
    let r = &g.tab.wallreact_breach[b];
    let (cause, walk, who, blast_by) = (r.cause, r.walk, r.who, r.blast_by);
    let by = if cause == BreachCause::Blast { blast_by.unwrap_or(who) } else { who };
    let own = if crate::seats::is_man_id(g, by) {
        !crate::teams::is_hostile(g, team, crate::teams::Team::Player)
    } else {
        g.guards.iter().any(|x| g.ch[g.gd[*x].ch].id == by && g.gd[*x].team == team)
    };
    let line = if own {
        if walk { "the wall's down — watch it!" } else { "hole in the wall — watch it!" }
    } else if walk {
        if cause == BreachCause::Blast { "they've blown the wall!" } else { "the wall's down — they can come through!" }
    } else if cause != BreachCause::Fire {
        "hole in the wall — watch it!"
    } else {
        "they're shooting through the wall!"
    };
    say(g, gd, line);
}

/// (wallreact.ts:339 hears)
fn hears(g: &mut Game, head: V3, src: V3, r: f64) -> bool {
    flat(head, src) < r && crate::scene_collision::sound_paths_propagate(g, src, head, None).path_len < r
}

/// the wall going, as a noise: who hears it open knows where   (wallreact.ts WallReact.heard)
pub fn wall_react_heard(g: &mut Game, b: WallreactBreachId, row: &crate::wallbreak::LevelChange) {
    use crate::wallbreak::LevelChangeCause as C;
    let k = &g.k.wallreact;
    let r = match row.cause { C::Blast => k.hear_blast_m, C::Fire => k.hear_fire_m, _ => k.hear_fall_m };
    let src = [row.pos[0], 1.3, row.pos[2]];
    let ours = crate::seats::is_man_id(g, row.who);
    let mut known = ours;
    if !known {
        for pl in crate::seats::men_up(g) {
            let p = g.ch[g.pl[pl].ch].pos;
            if hears(g, [p[0], EYE, p[2]], src, r) { known = true; break; }
        }
    }
    if known { wall_react_ops_know(g, b, if ours { "the player made it" } else { "the player heard it" }); }
    for gd in g.guards.clone() {
        if is_dead(g, gd) { continue; }
        let made = char_id(g, gd) == row.who;
        if !made {
            let head = wall_react_head(g, WallReactHeadGd::Guard(gd));
            if !hears(g, head, src, r) { continue; }
        }
        let team = g.gd[gd].team;
        if team == crate::teams::Team::Operative {
            let by = format!("{} {} it", g.gd[gd].callsign, if made { "made" } else { "heard" });
            wall_react_ops_know(g, b, &by);
        } else if team != crate::teams::Team::Civilian {
            wall_react_man_knows(g, b, gd, if made { WallReactManKnowsHow::Made } else { WallReactManKnowsHow::Heard });
        }
    }
}

/// four times a second: who sees a breach he did not know; the news passed on   (wallreact.ts WallReact.notice)
pub fn wall_react_notice(g: &mut Game) {
    use crate::game::GuardState;
    use crate::teams::Team;
    let now = g.time;
    for b in breaches_of(g) {
        if g.tab.wallreact_breach[b].ops < 0.0 {
            let pl = g.player;
            let (pos, yaw) = { let c = &g.ch[g.pl[pl].ch]; (c.pos, c.aim_yaw) };
            if !g.pl[pl].down && wall_react_sees(g, b, pos, Some(yaw)) {
                wall_react_ops_know(g, b, "the player saw it");
            } else {
                for gd in g.guards.clone() {
                    if g.gd[gd].team != Team::Operative || is_dead(g, gd) || now < g.gd[gd].dazzled_until { continue; }
                    let (pos, yaw) = { let c = &g.ch[g.gd[gd].ch]; (c.pos, c.aim_yaw) };
                    if !wall_react_sees(g, b, pos, Some(yaw)) { continue; }
                    let by = format!("{} saw it", g.gd[gd].callsign);
                    wall_react_ops_know(g, b, &by);
                    if g.gd[gd].unit.is_some() {
                        say(g, gd, if g.tab.wallreact_breach[b].walk { "hole in the wall." } else { "gap in the wall — watch it." });
                    }
                    break;
                }
            }
        }
        for gd in g.guards.clone() {
            let team = g.gd[gd].team;
            if is_dead(g, gd) || team == Team::Operative || team == Team::Civilian || g.tab.wallreact_breach[b].men.has(gd) || now < g.gd[gd].dazzled_until { continue; }
            let (pos, yaw) = { let c = &g.ch[g.gd[gd].ch]; (c.pos, c.aim_yaw) };
            if wall_react_sees(g, b, pos, Some(yaw)) { wall_react_man_knows(g, b, gd, WallReactManKnowsHow::Saw); }
        }
        // (a man told is never added to this list, so the index walk meets only what it has not passed)
        let mut i = g.tab.wallreact_breach[b].tell.len();
        while i > 0 {
            i -= 1;
            let t = g.tab.wallreact_breach[b].tell[i];
            if now < t.at { continue; }
            g.tab.wallreact_breach[b].tell.remove(i);
            if g.gd[t.by].state == GuardState::Dead { continue; }
            let tell_m = g.k.wallreact.tell_m;
            for gd in g.guards.clone() {
                if gd == t.by || is_dead(g, gd) || g.gd[gd].team != g.gd[t.by].team || g.tab.wallreact_breach[b].men.has(gd) { continue; }
                if flat(pos_of(g, gd), pos_of(g, t.by)) >= tell_m { continue; }
                let (from, to) = (wall_react_head(g, WallReactHeadGd::Guard(t.by)), wall_react_head(g, WallReactHeadGd::Guard(gd)));
                if crate::scene_collision::sound_paths_propagate(g, from, to, None).path_len < tell_m { wall_react_man_knows(g, b, gd, WallReactManKnowsHow::Told); }
            }
        }
    }
}

// ================================================================ (b) the squad's book

/// (wallreact.ts WallReact.dropOpening)
pub fn wall_react_drop_opening(g: &mut Game, o: CoverOpeningId) {
    if let Some(i) = g.cover.openings.iter().position(|x| *x == o) { g.cover.openings.remove(i); }
}

/// (wallreact.ts WallReact.dropOpenings)
pub fn wall_react_drop_openings(g: &mut Game) {
    for b in breaches_of(g) {
        if let Some(o) = g.tab.wallreact_breach[b].opening {
            wall_react_drop_opening(g, o);
            g.tab.wallreact_breach[b].opening = None;
        }
    }
}

/// a breach the team knows is one of its openings, kept in the list whoever rebuilds it   (wallreact.ts WallReact.book)
pub fn wall_react_book(g: &mut Game) {
    if !crate::seats::team_active(g) { return; }
    let mut readied = false;
    for b in breaches_of(g) {
        if g.tab.wallreact_breach[b].ops < 0.0 { continue; }
        // (ready() hands back the live list in the TypeScript; here it builds it, and the list is g.cover.openings)
        if !readied { crate::cover::cover_ready(g); readied = true; }
        let o = match g.tab.wallreact_breach[b].opening {
            Some(o) => o,
            None => {
                let r = &g.tab.wallreact_breach[b];
                let row = crate::cover::Opening { id: r.key.clone(), kind: crate::cover::OpeningKind::Door, at: [r.at[0], CHEST, r.at[2]], n: [r.n[0], r.n[1]], half: r.half, door: None, pass: None, far: None };
                let o = g.tab.cover_opening.push(row);
                g.tab.wallreact_breach[b].opening = Some(o);
                g.stats.wallreact.openings += 1;
                o
            }
        };
        if !g.cover.openings.contains(&o) { g.cover.openings.push(o); }
    }
}

// ================================================================ (b) the defenders' watch

/// ten times a second: what each man who knows a breach has against it   (wallreact.ts WallReact.evidence)
pub fn wall_react_evidence(g: &mut Game, dt: f64) {
    let now = g.time;
    let (forget, ev_seen, ev_fire) = (g.k.wallreact.forget, g.k.wallreact.ev_seen, g.k.wallreact.ev_fire);
    // (each walk deletes only the entry it stands on: a copy of the keys is the same walk)
    for gd in g.m.wallreact.wall_react.minds.keys() {
        if is_dead(g, gd) {
            g.m.wallreact.wall_react.minds.delete(gd);
            g.m.wallreact.wall_react.watching.delete(gd);
            continue;
        }
        let p = pos_of(g, gd);
        let man = &g.gd[gd];
        let seen = if now - man.last_seen_t < 0.3 { man.last_known } else { None };
        let fire = match man.sup.src { Some(s) if now - man.sup.src_t < 1.5 => Some(s), _ => None };
        for b in g.m.wallreact.wall_react.minds.get(gd).unwrap().keys() {
            if !g.m.wallreact.wall_react.breaches.contains(&b) {
                g.m.wallreact.wall_react.minds.get_mut(gd).unwrap().delete(b);
                continue;
            }
            let mut ev = g.m.wallreact.wall_react.minds.get(gd).unwrap().get(b).unwrap().ev;
            ev = m::max(0.0, ev - forget * dt);
            if let Some(q) = seen { if wall_react_through(g, b, p, q) { ev = m::min(3.0, ev + ev_seen * dt); } }
            if let Some(q) = fire { if wall_react_through(g, b, p, q) { ev = m::min(3.0, ev + ev_fire * dt); } }
            g.m.wallreact.wall_react.minds.get_mut(gd).unwrap().get_mut(b).unwrap().ev = ev;
        }
    }
}

/// does the plan line p → q go through the breach   (wallreact.ts WallReact.through)
pub fn wall_react_through(g: &mut Game, b: WallreactBreachId, p: V3, q: V3) -> bool {
    let r = &g.tab.wallreact_breach[b];
    let sp = (p[0] - r.at[0]) * r.n[0] + (p[2] - r.at[2]) * r.n[1];
    let sq = (q[0] - r.at[0]) * r.n[0] + (q[2] - r.at[2]) * r.n[1];
    if sp * sq >= 0.0 { return false; }
    let t = sp / (sp - sq);
    let (x, z) = (p[0] + (q[0] - p[0]) * t, p[2] + (q[2] - p[2]) * t);
    m::abs((x - r.at[0]) * r.u[0] + (z - r.at[2]) * r.u[1]) <= r.half + 0.35
}

/// roomfight.ts applyWatch: a breach he knows, has a line to and has more against than the door `pt` is on, takes it   (wallreact.ts WallReact.look)
pub fn wall_react_look(g: &mut Game, gd: GuardId, pt: &mut V3) {
    match g.m.wallreact.wall_react.minds.get(gd) { Some(mind) if !mind.is_empty() => {} _ => return }
    let d = match wall_react_def_of(g, gd) {
        Some(d) if g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold => d,
        _ => { g.m.wallreact.wall_react.watching.delete(gd); return; }
    };
    if g.time < g.tab.roomfight_def[d].glance_until { return; }
    let p = pos_of(g, gd);
    let had = g.m.wallreact.wall_react.watching.get(gd).copied();
    let mut best: Option<WallreactBreachId> = None;
    let row = &g.tab.roomfight_def[d];
    let mut bs = (if row.watch >= 0.0 { row.ev.get(row.watch as usize).copied().unwrap_or(0.0) } else { 0.0 }) + 0.05;
    let now = g.time;
    for b in g.m.wallreact.wall_react.minds.get(gd).unwrap().keys() {
        if flat(p, g.tab.wallreact_breach[b].at) > g.k.wallreact.watch_m { continue; }
        let mut mi = *g.m.wallreact.wall_react.minds.get(gd).unwrap().get(b).unwrap();
        if now - mi.line_t > 0.5 || now < mi.line_t {
            mi.line_t = now;
            let face = wall_react_face_of(g, b, p, None);
            mi.line = !crate::scene_collision::segment_blocked(g, [p[0], EYE, p[2]], face, None, None, None);
            *g.m.wallreact.wall_react.minds.get_mut(gd).unwrap().get_mut(b).unwrap() = mi;
        }
        if !mi.line { continue; }
        let mut mates = 0.0;
        for (o, w) in g.m.wallreact.wall_react.watching.iter() {
            if o != gd && *w == b && !is_dead(g, o) { mates += 1.0; }
        }
        let (k, walk) = (&g.k.wallreact, g.tab.wallreact_breach[b].walk);
        let s = mi.ev + (if walk { k.hole_base } else { k.port_base }) - k.crowd * mates + (if Some(b) == had { k.sticky } else { 0.0 }) + (if now - mi.t < k.fresh_secs && now >= mi.t { k.fresh } else { 0.0 });
        if s > bs { bs = s; best = Some(b); }
    }
    let best = match best { Some(b) => b, None => { g.m.wallreact.wall_react.watching.delete(gd); return; } };
    let r = &g.tab.wallreact_breach[best];
    let (at, walk) = (r.at, r.walk);
    if Some(best) != had {
        g.m.wallreact.wall_react.watching.set(gd, best);
        g.stats.wallreact.watches += 1;
        let what = format!("watch → the {} at ({}, {})", if walk { "hole" } else { "port" }, fixed(at[0], 1), fixed(at[2], 1));
        let who = g.gd[gd].callsign.clone();
        g.fight.log.push(crate::roomfight::FightNote { t: now, who, what });
    }
    pt[0] = at[0];
    pt[2] = at[2];
}

// ================================================================ (a) cover gone

/// (wallreact.ts WallReact.posts)
pub fn wall_react_posts(g: &mut Game) -> Vec<WallReactPostsItem> {
    let mut out = Vec::new();
    if !crate::seats::team_active(g) { return out; }
    for gd in crate::seats::team_men(g) {
        if is_dead(g, gd) { continue; }
        if let Some(a) = crate::seats::order_of(g, gd) {
            if g.tab.orders_assignment[a].post.is_some() { out.push(WallReactPostsItem { gd, a }); }
        }
    }
    out
}

/// (wallreact.ts WallReact.charOf)
pub fn wall_react_char_of(g: &mut Game, id_2: f64) -> Option<WallReactCharOf> {
    if let Some(pl) = crate::seats::man_by_id(g, id_2) {
        if g.pl[pl].down { return None; }
        return Some(WallReactCharOf { pos: g.ch[g.pl[pl].ch].pos, team: crate::teams::Team::Player });
    }
    let found = g.guards.iter().copied().find(|x| g.ch[g.gd[*x].ch].id == id_2)?;
    if is_dead(g, found) { return None; }
    Some(WallReactCharOf { pos: pos_of(g, found), team: g.gd[found].team })
}

/// where the trouble is, for this man, about this change   (wallreact.ts WallReact.threatFor)
pub fn wall_react_threat_for(g: &mut Game, gd: GuardId, x: Gone, at: V3) -> Option<V3> {
    let who = wall_react_char_of(g, x.who);
    let h = if is_op(g, gd) { 1.2 } else { 0.9 };
    let man = &g.gd[gd];
    let sight = if g.time - man.sight_t < 6.0 { Some(man.sight_pos) } else { None };
    let fire = match man.sup.src { Some(s) if g.time - man.sup.src_t < g.k.suppression.src_memory_secs => Some(s), _ => None };
    let team = man.team;
    let maker = match who { Some(w) if crate::teams::is_hostile(g, team, w.team) => Some(w.pos), _ => None };
    for c in [sight, fire, maker] {
        if let Some(c) = c {
            if wall_react_open_gone(g, c, at, h, x) { return Some(c); }
        }
    }
    match x.b { Some(b) => wall_react_beyond(g, gd, b, at), None => None }
}

/// what a question is about, in the order it is asked   (wallreact.ts WallReact.things)
pub fn wall_react_things(g: &mut Game, p: &Pend) -> Vec<Gone> {
    let mut out = vec![Gone { rect: p.rect, b: p.b, who: p.who }];
    if let Some(also) = &p.also { out.extend(also.iter().copied()); }
    out
}

/// the first thing of the question with a threat he is open to, else why not   (wallreact.ts WallReact.asks)
pub fn wall_react_asks(g: &mut Game, gd: GuardId, p: &Pend, at: V3, h: f64) -> WallReactAsks {
    let mut why = String::new();
    for x in wall_react_things(g, p) {
        let threat = match wall_react_threat_for(g, gd, x, at) {
            Some(t) => t,
            None => { if why.is_empty() { why = String::from("nothing beyond it to hide from"); } continue; }
        };
        if !wall_react_open_gone(g, threat, at, h, x) {
            if why.is_empty() { why = String::from("hidden where he is"); }
            continue;
        }
        return WallReactAsks::Arm1(WallReactAsksArm1 { x, threat });
    }
    WallReactAsks::Text(why)
}

/// the floor just beyond a breach from where a man stands; none when it is no danger to him   (wallreact.ts WallReact.beyond)
pub fn wall_react_beyond(g: &mut Game, gd: GuardId, b: WallreactBreachId, at: V3) -> Option<V3> {
    let beyond_m = g.k.wallreact.beyond_m;
    let r = &g.tab.wallreact_breach[b];
    let (bat, n) = (r.at, r.n);
    let s = if (at[0] - bat[0]) * n[0] + (at[2] - bat[2]) * n[1] >= 0.0 { 1.0 } else { -1.0 };
    let mut far: Option<V3> = None;
    for d in [beyond_m, beyond_m * 0.7, beyond_m * 0.45] {
        let q = [bat[0] - n[0] * s * d, 0.0, bat[2] - n[1] * s * d];
        if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { far = Some(q); break; }
    }
    let far = far?;
    let over = |p: V3| ((p[0] - bat[0]) * n[0] + (p[2] - bat[2]) * n[1]) * s < 0.0 && flat(p, far) < 2.5;
    if is_op(g, gd) {
        if crate::cleared::cleared_rooms_room_at(g, far) != 0.0 && crate::cleared::cleared_rooms_known_at(g, far) { return None; }
        let squad = crate::orders::squad_orders_squad(g);
        if squad.iter().any(|x| *x != gd && !is_dead(g, *x) && over(pos_of(g, *x))) || (!g.pl[g.player].down && over(player_pos(g))) { return None; }
    } else {
        let team = g.gd[gd].team;
        for x in g.guards.iter().copied() {
            if x != gd && !is_dead(g, x) && g.gd[x].team == team && over(pos_of(g, x)) { return None; }
        }
    }
    Some(far)
}

/// open to `threat` along a line through what went, a chest at `h` over `at`   (wallreact.ts WallReact.openGone)
pub fn wall_react_open_gone(g: &mut Game, threat: V3, at: V3, h: f64, x: Gone) -> bool {
    seg_rect(threat, at, x.rect, 0.45) && !crate::scene_collision::segment_blocked(g, [threat[0], 1.15, threat[2]], [at[0], h, at[2]], None, None, None)
}

/// length of a path from `from`, and where it ends   (wallreact.ts:490, 529)
fn walk_len(from: V3, path: &[V3]) -> (f64, V3) {
    let (mut l, mut q) = (0.0, from);
    for x in path { l += flat(q, *x); q = *x; }
    (l, q)
}

/// `dc.has(nav.idx(floor(x / cell), floor(z / cell)))`, or a blocked cell: no floor   (wallreact.ts:482, 524)
fn no_floor(g: &mut Game, dc: &JsMap<crate::Num, DoorId>, p: V3) -> bool {
    if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return true; }
    let cell = g.col.nav.cell;
    let i = crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell));
    dc.has(crate::Num(i))
}

/// a couple of strides to somewhere a chest at `h` is hidden from `threat` (and from the face of the breach), walked to in a straight line   (wallreact.ts WallReact.stepOut)
pub fn wall_react_step_out(g: &mut Game, gd: GuardId, from: V3, threat: V3, h: f64, b: Option<WallreactBreachId>, taken: &[V3], loosely: Option<bool>) -> Option<V3> {
    let loosely = loosely.unwrap_or(false);
    let step_m = g.k.wallreact.step_m;
    let t = [threat[0], 1.15, threat[2]];
    let dc = crate::guards::door_cells(g);
    let row = b.map(|b| { let r = &g.tab.wallreact_breach[b]; (r.at, r.n, r.u, r.half) });
    let depth = match row { Some((at, n, _, _)) => m::abs((from[0] - at[0]) * n[0] + (from[2] - at[2]) * n[1]), None => 0.0 };
    let (mut best, mut bc): (Option<V3>, f64) = (None, f64::INFINITY);
    let (mut alt, mut ac): (Option<V3>, f64) = (None, f64::INFINITY);
    let mut cands: Vec<V3> = Vec::new();
    if let Some((at, n, u, half)) = row {
        let s = if (from[0] - at[0]) * n[0] + (from[2] - at[2]) * n[1] >= 0.0 { 1.0 } else { -1.0 };
        let l0 = (from[0] - at[0]) * u[0] + (from[2] - at[2]) * u[1];
        for d in [0.8, m::max(0.8, m::min(depth, 1.5))] {
            for out in [0.6, 1.0, 1.5, 2.1, 2.7] {
                for e in if l0 >= 0.0 { [1.0, -1.0] } else { [-1.0, 1.0] } {
                    let l = e * (half + out);
                    cands.push([at[0] + u[0] * l + n[0] * s * d, 0.0, at[2] + u[1] * l + n[1] * s * d]);
                }
            }
        }
    }
    for r in [1.0, 1.5, 2.0, 2.5, 3.0, step_m] {
        for a in 0..16 {
            let a = a as f64;
            cands.push([from[0] + m::sin(a * m::PI / 8.0) * r, 0.0, from[2] + m::cos(a * m::PI / 8.0) * r]);
        }
    }
    let mut plans = 0; // (path searches a question: suppression.ts's own cover search stops at twelve)
    for p in cands {
        let r = flat(from, p);
        if r > step_m + 2.0 { continue; }
        if no_floor(g, &dc, p) { continue; }
        let mut cost = r;
        if let Some((at, n, _, _)) = row {
            let s0 = (from[0] - at[0]) * n[0] + (from[2] - at[2]) * n[1];
            let s1 = (p[0] - at[0]) * n[0] + (p[2] - at[2]) * n[1];
            if s0 * s1 <= 0.0 { continue; }
            cost += 0.6 * m::abs(m::abs(s1) - depth);
        }
        if cost >= bc { continue; }
        if !crate::scene_collision::segment_blocked(g, t, [p[0], h, p[2]], None, None, None) { continue; }
        let ri = js::or(r, 1.0);
        let sh = [p[0] + (from[0] - p[0]) / ri * m::min(0.35, r), 0.0, p[2] + (from[2] - p[2]) / ri * m::min(0.35, r)];
        if let Some(b) = b {
            if wall_react_open_through(g, b, p, h) || wall_react_open_through(g, b, sh, h) { continue; }
        }
        let mut loose = false;
        for o in wall_react_known(g, gd) {
            if Some(o) != b && wall_react_beyond(g, gd, o, p).is_some() && (wall_react_open_through(g, o, p, h) || wall_react_open_through(g, o, sh, h)) { loose = true; break; }
        }
        if loose && (!loosely || best.is_some() || cost >= ac) { continue; }
        if !crate::scene_collision::nav_grid_walkable(g, from, p) {
            let path = crate::scene_collision::nav_grid_find_path(g, from, p, None, None);
            let (l, q) = walk_len(from, path.as_deref().unwrap_or(&[]));
            plans += 1;
            if plans > 8 || path.as_ref().map_or(true, |x| x.is_empty()) || l > 1.5 * r + 1.0 || flat(q, p) > 0.5 { continue; }
        }
        if taken.iter().any(|q| flat(*q, p) < 0.9) { continue; }
        if loose { alt = Some(p); ac = cost; continue; }
        best = Some(p);
        bc = cost;
    }
    if best.is_some() { return best; }
    let mv = g.m.wallreact.wall_react.moved_t.get(gd).copied();
    if !loosely || mv.map_or(false, |mv| g.time >= mv && g.time - mv < 20.0) { None } else { alt }
}

/// is a chest at `h` over `p` open THROUGH the breach to the floor just beyond it   (wallreact.ts WallReact.openThrough)
pub fn wall_react_open_through(g: &mut Game, b: WallreactBreachId, p: V3, h: f64) -> bool {
    let r = &g.tab.wallreact_breach[b];
    let (at, n, u, half) = (r.at, r.n, r.u, r.half);
    let d = (p[0] - at[0]) * n[0] + (p[2] - at[2]) * n[1];
    if m::abs(d) > 8.0 { return false; }
    let s = if d >= 0.0 { 1.0 } else { -1.0 };
    for k in [0.0, 0.8, -0.8] {
        let q = [at[0] + u[0] * half * k - n[0] * s * 0.9, 1.15, at[2] + u[1] * half * k - n[1] * s * 0.9];
        if wall_react_through(g, b, q, p) && !crate::scene_collision::segment_blocked(g, q, [p[0], h, p[2]], None, None, None) { return true; }
    }
    false
}

/// the breaches a man knows of (his side's book for a teammate)   (wallreact.ts WallReact.known)
pub fn wall_react_known(g: &mut Game, gd: GuardId) -> Vec<WallreactBreachId> {
    let op = is_op(g, gd);
    breaches_of(g).into_iter().filter(|b| {
        let r = &g.tab.wallreact_breach[*b];
        if op { r.ops >= 0.0 } else { r.men.has(gd) }
    }).collect()
}

/// (wallreact.ts:513 hid)
fn spot_hid(g: &mut Game, gd: GuardId, t: V3, kn: &[WallreactBreachId], q: V3) -> bool {
    if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { return false; }
    if !crate::scene_collision::segment_blocked(g, t, [q[0], 0.9, q[2]], None, None, None) { return false; }
    for &o in kn {
        if wall_react_beyond(g, gd, o, q).is_some() && wall_react_open_through(g, o, q, 0.9) { return false; }
    }
    true
}

/// the fight's own cover from the threat, a stride deeper when the floor lets it   (wallreact.ts:514 own)
fn spot_own(g: &mut Game, gd: GuardId, p: V3, threat: V3, t: V3, kn: &[WallreactBreachId]) -> Option<V3> {
    let c = crate::suppression::cover_from(g, gd, threat)?;
    let r = js::or(flat(p, c), 1.0);
    let (ux, uz) = ((c[0] - p[0]) / r, (c[2] - p[2]) / r);
    let deep = [c[0] + ux * 0.35, 0.0, c[2] + uz * 0.35];
    if spot_hid(g, gd, t, kn, deep) { return Some(deep); }
    if spot_hid(g, gd, t, kn, [c[0] - ux * 0.35, 0.0, c[2] - uz * 0.35]) { Some(c) } else { None }
}

/// where a crouching man goes   (wallreact.ts WallReact.spot)
pub fn wall_react_spot(g: &mut Game, gd: GuardId, threat: V3, b: Option<WallreactBreachId>, others: &[V3]) -> Option<V3> {
    let p = pos_of(g, gd);
    let kn: Vec<WallreactBreachId> = wall_react_known(g, gd).into_iter().filter(|o| Some(*o) != b).collect();
    let t = [threat[0], 1.15, threat[2]];
    let first = if b.is_some() {
        match wall_react_step_out(g, gd, p, threat, 0.9, b, others, None) { Some(q) => Some(q), None => spot_own(g, gd, p, threat, t, &kn) }
    } else {
        match spot_own(g, gd, p, threat, t, &kn) { Some(q) => Some(q), None => wall_react_step_out(g, gd, p, threat, 0.9, None, others, None) }
    };
    if first.is_some() { return first; }
    if let Some(q) = wall_react_run_for(g, gd, p, threat, others, &kn) { return Some(q); }
    if b.is_some() { wall_react_step_out(g, gd, p, threat, 0.9, b, others, Some(true)) } else { None }
}

/// nothing within a few strides: the nearest corner he can run to that hides a crouched chest   (wallreact.ts WallReact.runFor)
pub fn wall_react_run_for(g: &mut Game, gd: GuardId, from: V3, threat: V3, taken: &[V3], kn: &[WallreactBreachId]) -> Option<V3> {
    let run_m = g.k.wallreact.run_m;
    let t = [threat[0], 1.15, threat[2]];
    let dc = crate::guards::door_cells(g);
    let (mut best, mut bl, mut plans): (Option<V3>, f64, i32) = (None, f64::INFINITY, 0);
    let mut r = 4.0;
    while r <= run_m {
        for a in 0..16 {
            let a = a as f64;
            let p = [from[0] + m::sin(a * m::PI / 8.0) * r, 0.0, from[2] + m::cos(a * m::PI / 8.0) * r];
            if no_floor(g, &dc, p) { continue; }
            if !crate::scene_collision::segment_blocked(g, t, [p[0], 0.9, p[2]], None, None, None) || taken.iter().any(|q| flat(*q, p) < 0.9) { continue; }
            if flat(p, threat) < flat(from, threat) * 0.6 { continue; }
            plans += 1;
            if plans > 10 { return best; }
            let path = crate::scene_collision::nav_grid_find_path(g, from, p, None, None);
            let (l, q) = walk_len(from, path.as_deref().unwrap_or(&[]));
            if path.as_ref().map_or(true, |x| x.is_empty()) || l > 1.6 * r + 2.0 || flat(q, p) > 0.5 || l >= bl { continue; }
            let mut open = false;
            for &o in kn {
                if wall_react_beyond(g, gd, o, p).is_some() && wall_react_open_through(g, o, p, 0.9) { open = true; break; }
            }
            if open { continue; }
            best = Some(p);
            bl = l;
        }
        if best.is_some() { break; }
        r += 1.0;
    }
    best
}

/// (wallreact.ts WallReact.reconsider)
pub fn wall_react_reconsider(g: &mut Game, p: &mut Pend) -> WallReactReconsider {
    match wall_react_reconsider1(g, p).0 {
        WallReactReconsider10::Fine => WallReactReconsider::Fine,
        WallReactReconsider10::Moved => WallReactReconsider::Moved,
        WallReactReconsider10::Later => WallReactReconsider::Later,
    }
}

fn shift_word(r: WallReactShift) -> WallReactReconsider10 {
    match r { WallReactShift::Fine => WallReactReconsider10::Fine, WallReactShift::Moved => WallReactReconsider10::Moved, WallReactShift::Later => WallReactReconsider10::Later }
}

/// (wallreact.ts WallReact.reconsider1)
pub fn wall_react_reconsider1(g: &mut Game, p: &mut Pend) -> (WallReactReconsider10, String) {
    use crate::roomfight::DefMode;
    use crate::teams::Team;
    use WallReactReconsider10 as R;
    let gd = p.gd;
    if is_dead(g, gd) { return (R::Fine, String::from("dead")); }
    let man = &g.gd[gd];
    if g.time < man.dazzled_until || g.time < man.stagger_until || man.held.is_some() { return (R::Later, String::from("blind, staggered or held")); }
    if let Some(mv) = g.m.wallreact.wall_react.moved_t.get(gd).copied() {
        if g.time >= mv && g.time - mv < g.k.wallreact.retry_secs { return (R::Later, String::from("only just moved")); }
    }
    let team = g.gd[gd].team;
    if team == Team::Operative {
        let r = if g.gd[gd].unit.is_some() {
            match wall_react_op_move(g, p) { WallReactOpMove::Fine => R::Fine, WallReactOpMove::Moved => R::Moved, WallReactOpMove::Later => R::Later }
        } else { R::Fine };
        return (r, format!("his mark: {}", r.as_str()));
    }
    if team == Team::Civilian { return (R::Fine, String::from("a civilian")); }
    let c = pos_of(g, gd);
    let q = match wall_react_asks(g, gd, p, c, 0.9) {
        WallReactAsks::Text(why) => return (R::Fine, why),
        WallReactAsks::Arm1(q) => q,
    };
    let (threat, pb) = (q.threat, q.x.b);
    let others: Vec<V3> = g.guards.iter().copied().filter(|o| *o != gd && !is_dead(g, *o)).map(|o| pos_of(g, o)).collect();
    if let Some(d) = wall_react_def_of(g, gd) {
        let mode = g.tab.roomfight_def[d].mode;
        if mode != DefMode::Hold {
            let r = if mode == DefMode::Move { R::Fine } else { shift_word(wall_react_shift(g, gd, threat, pb, &others)) };
            return (r, format!("mode {}: {}", mode.as_str(), r.as_str()));
        }
        let man = &g.gd[gd];
        if g.time - man.last_seen_t < 0.6 && man.last_known.map_or(false, |k| flat(k, c) < 5.0) { return (R::Later, String::from("a man at arm's length")); }
        let mut to = wall_react_spot(g, gd, threat, pb, &others);
        if let Some(q) = to {
            if crate::laststand::last_stand_man(g, gd) || crate::enemysquads::squad_stays_in(g, gd) || crate::digin::dig_in_set(g, gd) {
                if crate::roomfight::room_fight_room_at(g, q) != g.tab.roomfight_def[d].room { to = None; }
            }
        }
        let to = match to {
            Some(q) => q,
            None => {
                g.stats.wallreact.no_cover += 1;
                p.next = g.time + 1.5;
                return (R::Later, String::from("nowhere to go"));
            }
        };
        let now = g.time;
        let row = &mut g.tab.roomfight_def[d];
        row.mode = DefMode::Move;
        row.move_kind = crate::roomfight::DefMoveKind::Cover;
        row.came_by = None;
        row.move_t = 0.0;
        row.stall_at = v3::copy(c);
        row.stall_t = 0.0;
        row.moved_at = now;
        g.gd[gd].pinned = false;
        g.m.wallreact.wall_react.watching.delete(gd);
        let patch = crate::squad::OrderPatch {
            goal: Some(Some(to)), speed: Some(Some(g.tune.guard_chase)), face: Some(None), strafe: Some(Some(false)), aim_at: Some(None),
            upper: Some(Some(crate::character::UpperMode::Aim)), stance: Some(Some(crate::character::Stance::None)), crouch: Some(Some(true)),
            ..Default::default()
        };
        crate::squad::order(g, gd, patch);
        g.fight.stats.took_cover += 1.0;
        let who = g.gd[gd].callsign.clone();
        g.fight.log.push(crate::roomfight::FightNote { t: now, who, what: String::from("his cover is gone: takes cover") });
        g.stats.wallreact.moved_def += 1;
        g.m.wallreact.wall_react.moved_t.set(gd, now);
        let text = format!("{} (holding) leaves ({}, {}) for ({}, {})", g.gd[gd].callsign, fixed(c[0], 1), fixed(c[2], 1), fixed(to[0], 1), fixed(to[2], 1));
        note(g, &text);
        say(g, gd, "cover's gone — moving!");
        return (R::Moved, String::from("moved"));
    }
    let r = shift_word(wall_react_shift(g, gd, threat, pb, &others));
    (r, format!("the fight's own: {}", r.as_str()))
}

/// a man in the fight AI: its own shift to cover   (wallreact.ts WallReact.shift)
pub fn wall_react_shift(g: &mut Game, gd: GuardId, threat: V3, b: Option<WallreactBreachId>, others: &[V3]) -> WallReactShift {
    use crate::game::GuardState;
    let man = &g.gd[gd];
    if man.script.is_some() || man.pinned || man.standoff.is_some() || !g.tune.suppression { return WallReactShift::Fine; }
    if man.state != GuardState::Alert {
        return if man.state == GuardState::Search || man.state == GuardState::Suspicious { WallReactShift::Later } else { WallReactShift::Fine };
    }
    if man.sup.cover.is_some() { return WallReactShift::Fine; }
    let to = match wall_react_spot(g, gd, threat, b, others) {
        Some(q) => q,
        None => { g.stats.wallreact.no_cover += 1; return WallReactShift::Later; }
    };
    let now = g.time;
    let man = &mut g.gd[gd];
    man.sup.cover = Some(to);
    man.sup.displaced_at = now;
    man.sup.displaces += 1.0;
    man.sup.pinned_t = 0.0;
    man.sup.down_t = 0.0;
    man.sup.latched = false;
    man.path = Vec::new();
    man.path_goal = None;
    man.stall_ref = None;
    g.stats.wallreact.moved_ai += 1;
    g.m.wallreact.wall_react.moved_t.set(gd, now);
    let text = format!("{} (fighting) shifts to ({}, {})", g.gd[gd].callsign, fixed(to[0], 1), fixed(to[2], 1));
    note(g, &text);
    WallReactShift::Moved
}

/// a teammate on a mark (or walking to one): the mark is moved out of the line   (wallreact.ts WallReact.opMove)
pub fn wall_react_op_move(g: &mut Game, p: &mut Pend) -> WallReactOpMove {
    let gd = p.gd;
    let v = crate::seats::view_of(g, gd);
    let a = match crate::seats::with_seat(g, v, &mut |g| crate::orders::squad_orders_of(g, gd)) { Some(a) => a, None => return WallReactOpMove::Fine };
    let post = match g.tab.orders_assignment[a].post { Some(q) => q, None => return WallReactOpMove::Fine };
    if crate::seats::with_seat(g, v, &mut |g| g.door_tools.ops.iter().any(|op| g.tab.doortools_door_op[*op].men.contains(&gd))) { return WallReactOpMove::Fine; }
    if crate::seats::with_seat(g, v, &mut |g| g.wall_charge.ops.iter().any(|op| g.tab.wallcharge_wall_op[*op].men.contains(&gd))) { return WallReactOpMove::Fine; }
    if g.tab.orders_assignment[a].order == crate::wallcharge::WALL_ORDER && g.tab.orders_assignment[a].watch.is_some() { return WallReactOpMove::Fine; }
    if g.tab.orders_assignment[a].yield_t >= 0.0 || crate::roomfight::room_fight_engaged(g, gd) { return WallReactOpMove::Later; }
    let q = match wall_react_asks(g, gd, p, post, 1.2) { WallReactAsks::Text(_) => return WallReactOpMove::Fine, WallReactAsks::Arm1(q) => q };
    let threat = q.threat;
    let taken: Vec<V3> = crate::seats::with_seat(g, v, &mut |g| {
        let mut taken = vec![player_pos(g)];
        for m in crate::orders::squad_orders_squad(g) {
            if m == gd || is_dead(g, m) { continue; }
            taken.push(pos_of(g, m));
            if let Some(b) = crate::orders::squad_orders_of(g, m) {
                if let Some(q) = g.tab.orders_assignment[b].post { taken.push(q); }
            }
        }
        taken
    });
    let pb = q.x.b;
    let to = match wall_react_step_out(g, gd, post, threat, 1.2, pb, &taken, None) {
        Some(q) => Some(q),
        None => wall_react_step_out(g, gd, post, threat, 1.2, pb, &taken, Some(true)),
    };
    let to = match to { Some(q) => q, None => { g.stats.wallreact.no_cover += 1; return WallReactOpMove::Later; } };
    let text = format!("{} ({}) mark ({}, {}) → ({}, {})", g.gd[gd].callsign, g.tab.orders_assignment[a].order, fixed(post[0], 1), fixed(post[2], 1), fixed(to[0], 1), fixed(to[2], 1));
    note(g, &text);
    let now = g.time;
    let at = pos_of(g, gd);
    let row = &mut g.tab.orders_assignment[a];
    row.post = Some(to);
    row.goal = v3::copy(to);
    row.via.clear();
    row.arrived = false;
    row.moved_t = now;
    row.mark_t = now;
    row.mark[0] = at[0];
    row.mark[2] = at[2];
    g.stats.wallreact.moved_op += 1;
    g.m.wallreact.wall_react.moved_t.set(gd, now);
    if flat(at, to) > 0.6 { crate::seats::with_seat(g, v, &mut |g| say(g, gd, "moving off the hole.")); }
    WallReactOpMove::Moved
}

/// tactics.ts: one of a plan's men has learned, since it was dealt, of a hole a man fits through   (wallreact.ts WallReact.replan)
pub fn wall_react_replan(g: &mut Game, team: crate::teams::Team, since: f64, men: &[GuardId]) -> bool {
    for b in breaches_of(g) {
        let r = &g.tab.wallreact_breach[b];
        if !r.walk { continue; }
        for m in men {
            let t = if team == crate::teams::Team::Operative { r.ops } else { r.men.get(*m).copied().unwrap_or(-1.0) };
            if t > since { g.stats.wallreact.replans += 1; return true; }
        }
    }
    false
}

/// (wallreact.ts WallReact.summary)
pub fn wall_react_summary(g: &mut Game) -> String {
    let live = wall_react_live(g);
    let list = breaches_of(g);
    let walk = list.iter().filter(|b| g.tab.wallreact_breach[**b].walk).count();
    let ops = list.iter().filter(|b| g.tab.wallreact_breach[**b].ops >= 0.0).count();
    let s = &g.stats.wallreact;
    format!("wall react {} · {} breaches ({} walk-through, team knows {}) · rows {} · rooms joined {} · moved: holding {}, fighting {}, team {} (nowhere {}) · watches {} · replans {}",
        if live { "on" } else { "off" }, list.len(), walk, ops, s.rows, s.merged, s.moved_def, s.moved_ai, s.moved_op, s.no_cover, s.watches, s.replans)
}

#[cfg(test)]
mod tests {
    use super::*;
    // The expected values are the TypeScript's own, run under bun with EXACT.on.

    #[test]
    fn wrap_has_the_typescripts_bits() {
        for (a, bits) in [
            (0.0, 0x0000000000000000u64), (1.0, 0x3ff0000000000000), (-1.0, 0xbff0000000000000), (3.2, 0xc008aa5d0eeec096), (-3.2, 0x4008aa5d0eeec096),
            (m::PI, 0x400921fb54442d18), (-m::PI, 0x400921fb54442d18), (7.5, 0x3ff37812aeef4ba0), (-100.25, 0x3fd1fb54442d1840), (1e6, 0xbfd6e254d0ed6540),
        ] {
            assert_eq!(wrap(a).to_bits(), bits, "wrap({a})");
        }
    }

    #[test]
    fn rect_dist_has_the_typescripts_bits() {
        let r = [3.0, 4.0, 5.0, 6.0];
        assert_eq!(rect_dist([1.5, 0.0, 2.25], r).to_bits(), 0x4002706821902e9a);
        assert_eq!(rect_dist([4.0, 0.0, 5.0], r).to_bits(), 0);
        assert_eq!(rect_dist([10.1, 0.0, -3.7], r).to_bits(), 0x402278bac4162300);
    }

    #[test]
    fn seg_rect_answers_as_the_typescript_does() {
        let r = [4.0, 4.0, 6.0, 6.0];
        assert!(seg_rect([0.0, 0.0, 0.0], [10.0, 0.0, 10.0], r, 0.45));
        assert!(!seg_rect([0.0, 0.0, 0.0], [10.0, 0.0, 0.0], r, 0.45));
        assert!(seg_rect([0.0, 0.0, 5.0], [10.0, 0.0, 5.0], r, 0.0));
        assert!(!seg_rect([5.0, 0.0, 0.0], [5.0, 0.0, 3.0], r, 0.45));
        assert!(seg_rect([5.0, 0.0, 0.0], [5.0, 0.0, 3.6], r, 0.45));
    }

    #[test]
    fn fixed_rounds_as_to_fixed_does() {
        for (x, d, s) in [
            (0.125, 2, "0.13"), (0.25, 1, "0.3"), (-0.125, 2, "-0.13"), (-0.001, 2, "-0.00"), (-0.0, 2, "0.00"),
            (12.25, 1, "12.3"), (1.005, 2, "1.00"), (3.14159, 2, "3.14"), (100.75, 1, "100.8"), (2.5, 1, "2.5"),
        ] {
            assert_eq!(fixed(x, d), s, "{x}.toFixed({d})");
        }
    }
}
