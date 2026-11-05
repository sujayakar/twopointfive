//! src/game/lanes.ts lines 439 to 872
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

const SIDE: crate::teams::Team = crate::teams::Team::Hostile;

// ---------------------------------------------------------------- the one seam left (notes.md, Skeleton)
/// (lanes.ts:863, 868) `base`: what guards.ts and frag.ts hand in is tacticsPenalty's answer, which LanePenalty holds as a value; the signature still takes a closure
fn seam_base_penalty(base: &mut dyn FnMut(&mut Game, f64, f64) -> f64) -> crate::tactics::TacticsPenalty { todo!("needs: price_of's and lane_penalty's `base` to be Option<&TacticsPenalty>") }

// ---------------------------------------------------------------- what the TypeScript's language and closures gave
/// (tactics.ts:361) the closure tacticsPenalty returns: a dangerous doorway's cell, a flanker's corridor, else plain
fn base_at(b: &crate::tactics::TacticsPenalty, ix: f64, iz: f64) -> f64 {
    let crate::tactics::TacticsPenalty::Cells { w, door_idx, door_mult, avoid } = b;
    let i = iz * *w + ix;
    let mut p = 1.0;
    if door_idx.as_ref().map_or(false, |s| s.has(cell_key(i))) { p = *door_mult; }
    if avoid.as_ref().map_or(false, |s| s.has(cell_key(i))) { p = m::max(p, crate::tactics::CORRIDOR_COST); }
    p
}
/// (lanes.ts:865) the closure priceOf returns: the price of a cell of the walk
fn pen_at(pen: &crate::lanes::LanePenalty, ix: f64, iz: f64) -> f64 {
    use crate::lanes::LanePenalty;
    let dear = |mask: &[u8], w: f64, d: f64| if mask[(iz * w + ix) as usize] != 0 { d } else { 1.0 };
    match pen {
        LanePenalty::Dear { m, w, dear: d } => dear(m, *w, *d),
        LanePenalty::DearOver { base, m, w, dear: d } => m::max(base_at(base, ix, iz), dear(m, *w, *d)),
        LanePenalty::Base { base } => base_at(base, ix, iz),
    }
}
/// `g.col.nav.findPath(a, b, maxIter, pen)` at a price this file holds as a value
fn find_path(g: &mut Game, a: V3, b: V3, max_iter: f64, pen: Option<&crate::lanes::LanePenalty>) -> Option<Vec<V3>> {
    match pen {
        Some(p) => { let mut price = |_: &mut Game, ix: f64, iz: f64| pen_at(p, ix, iz); crate::scene_collision::nav_grid_find_path(g, a, b, Some(max_iter), Some(&mut price)) }
        None => crate::scene_collision::nav_grid_find_path(g, a, b, Some(max_iter), None),
    }
}
/// `g.col.segmentBlocked(a, b)`
fn seg_blocked(g: &mut Game, a: V3, b: V3) -> bool { crate::scene_collision::segment_blocked(g, a, b, None, None, None) }
/// `g.col.solidBlocked(a, b)`
fn solid_blocked(g: &mut Game, a: V3, b: V3) -> bool { crate::scene_collision::solid_blocked(g, a, b, None) }

/// a cell's number as a key of a lane's floor (a Set has −0 and 0 as one key, `Num` by its bits)
fn cell_key(i: f64) -> crate::Num { crate::Num(i + 0.0) }
/// the middle of cell `i`
fn cell_mid(cell: f64, w: f64, i: f64) -> V3 { [(js::rem(i, w) + 0.5) * cell, 0.0, (js::to_i32(i / w) as f64 + 0.5) * cell] }
/// `gd.path = []; gd.pathGoal = null`
fn clear_path(g: &mut Game, gd: GuardId) { g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; }
/// `g.say(gd, text, true)`
fn say_net(g: &mut Game, gd: GuardId, text: &str) { crate::game::game_say(g, gd, text, Some(true), None, None); }
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn char_id_of(g: &Game, gd: GuardId) -> f64 { g.ch[g.gd[gd].ch].id }
fn goal_of(g: &Game, gd: GuardId) -> Option<V3> { g.gd[gd].script.as_ref().and_then(|s| s.goal) }
fn mode_of(g: &Game, d: RoomfightDefId) -> crate::roomfight::DefMode { g.tab.roomfight_def[d].mode }
fn is_dead(g: &Game, gd: GuardId) -> bool { g.gd[gd].state == crate::game::GuardState::Dead }
fn aim_at(g: &mut Game, gd: GuardId, at: V3) { crate::squad::order(g, gd, crate::squad::OrderPatch { aim_at: Some(Some(at)), ..Default::default() }); }

/// the men held at a lane's mouths, on their feet and with nobody on them (lanes.ts heldAt)
pub fn held_at(g: &mut crate::game::Game, l: &crate::lanes::Lane) -> Vec<GuardId> {
    let rows: Vec<(GuardId, crate::lanes::StateHeldValue)> = s(g).held.iter().map(|(gd, h)| (gd, *h)).collect();
    let mut out = Vec::new();
    for (gd, h) in rows {
        if h.lane == l.id && fit(g, gd) && g.time - g.gd[gd].last_seen_t > 1.5 && g.time - h.t > 1.0 { out.push(gd); }
    }
    out
}

/// what backOut's two closures share
struct BackOut<'a> { l: &'a crate::lanes::Lane, m: Option<Vec<u8>>, memo: JsMap<usize, bool>, w: usize }
/// floor out of every lane he knows, out of this one's guns' sight, not in a doorway (lanes.ts:447 cover)
fn back_cover(g: &mut Game, b: &mut BackOut, i: usize) -> bool {
    if let Some(&v) = b.memo.get(i) { return v; }
    let cell = g.col.nav.cell;
    let c: V3 = [((i % b.w) as f64 + 0.5) * cell, 0.0, ((i / b.w) as f64 + 0.5) * cell];
    let on_mask = b.m.as_ref().and_then(|m| m.get(i).copied()).unwrap_or(0) != 0;   // (a mask of nothing reads undefined: not on it)
    let shift = g.k.lanes.shift_m;
    let v = !on_mask && !gun_on(g, b.l, c, Some(shift)) && !crate::doorway::doorway_in_way(g, c);
    b.memo.set(i, v);
    v
}
/// every open floor about it is cover too (lanes.ts:449 deep)
fn back_deep(g: &mut Game, b: &mut BackOut, i: usize) -> bool {
    let (w, h) = (b.w as isize, g.col.nav.h as isize);
    let (cx, cz) = ((i % b.w) as isize, (i / b.w) as isize);
    for dz in -1..=1isize {
        for dx in -1..=1isize {
            if dx == 0 && dz == 0 { continue; }
            let (x, z) = (cx + dx, cz + dz);
            if x < 0 || z < 0 || x >= w || z >= h { continue; }
            let j = (z * w + x) as usize;
            if g.col.nav.blocked[j] == 0 && !back_cover(g, b, j) { return false; }
        }
    }
    true
}

/// a man in a lane's line walks to the nearest floor out of it: false = none in reach (lanes.ts backOut)
pub fn back_out(g: &mut crate::game::Game, gd: GuardId, l: &crate::lanes::Lane) -> bool {
    let d = match def_of(g, gd) { Some(d) => d, None => return false };
    let now = g.time;
    if now - s(g).fail_t.get(gd).copied().unwrap_or(-100.0) < 2.0 { return false; }
    let mask = mask_of(g, gd);
    let p0 = pos_of(g, gd);
    let start = cell_at(g, p0);
    if start < 0.0 { return false; }
    let start = start as usize;
    let (w, h, cell) = (g.col.nav.w as usize, g.col.nav.h as usize, g.col.nav.cell);
    let (wi, hi) = (w as isize, h as isize);
    let mut b = BackOut { l, m: mask, memo: JsMap::new(), w };
    // the nearest floor out of every lane he knows and out of this one's guns' sight, by walking (breadth first, no corner cut, ≤ 10 m)
    let mut seen: JsSet<usize> = JsSet::new();
    seen.add(start);
    let mut q: Vec<usize> = vec![start];
    let (mut found, mut first): (isize, isize) = (-1, -1);
    let mut first_d = 0i32;
    let mut depth = 0i32;
    while depth < 20 && found < 0 && !q.is_empty() {
        let mut nx: Vec<usize> = Vec::new();
        for &c in q.iter() {
            let (cx, cz) = ((c % w) as isize, (c / w) as isize);
            let mut dz = -1isize;
            while dz <= 1 && found < 0 {
                for dx in -1..=1isize {
                    if dx == 0 && dz == 0 { continue; }
                    let (x, z) = (cx + dx, cz + dz);
                    if x < 0 || z < 0 || x >= wi || z >= hi { continue; }
                    let i = (z * wi + x) as usize;
                    if g.col.nav.blocked[i] != 0 || seen.has(i) { continue; }
                    if dx != 0 && dz != 0 && (g.col.nav.blocked[(cz * wi + x) as usize] != 0 || g.col.nav.blocked[(z * wi + cx) as usize] != 0) { continue; }
                    seen.add(i);
                    if back_cover(g, &mut b, i) {
                        if first < 0 { first = i as isize; first_d = depth; }
                        if back_deep(g, &mut b, i) { found = i as isize; break; }
                    }
                    nx.push(i);
                }
                dz += 1;
            }
        }
        q = nx;
        if first >= 0 && depth - first_d >= 6 { break; }   // (no deeper cover within three metres of its edge: the edge it is)
        depth += 1;
    }
    if found < 0 { found = first; }
    if found < 0 { s(g).fail_t.set(gd, now); return false; }   // (no cover in reach: asked again in two seconds, not every step)
    let found = found as usize;
    let mut spot: V3 = [((found % w) as f64 + 0.5) * cell, 0.0, ((found / w) as f64 + 0.5) * cell];
    // (a step further from the lane than its very edge, where the floor allows: the same direction once more)
    {
        let (ex, ez) = (spot[0] - p0[0], spot[2] - p0[2]);
        let el = js::or(m::hyp2(ex, ez), 1.0);
        let far: V3 = [spot[0] + ex / el * 0.6, 0.0, spot[2] + ez / el * 0.6];
        let fi = cell_at(g, far);
        if fi >= 0.0 && g.col.nav.blocked[fi as usize] == 0 && back_cover(g, &mut b, fi as usize) { spot[0] = far[0]; spot[2] = far[2]; }
    }
    s(g).back_t.set(gd, now);
    let look = mouth_of(g, l, spot);
    let st = s(g);
    st.held.set(gd, crate::lanes::StateHeldValue { lane: l.id, t: now, look, aimed: false, at: spot });
    st.stats.backed_out += 1.0;
    clear_path(g, gd);
    crate::pressure::run(g, d, spot, None, Some(0.3));
    if now - s(g).said_t > 4.0 { s(g).said_t = now; say_net(g, gd, "get back — out of their line!"); }
    note(g, Some(gd), &format!("lane {}: in their line, backs out to ({}, {})", l.id, js::to_fixed(spot[0], 1), js::to_fixed(spot[2], 1)));
    true
}

/// a way round for one man (lanes.ts:476)
struct Way { gd: GuardId, cid: f64, len: f64, pin: bool, stop: Option<V3> }

/// GO ROUND: of the men held at a lane's mouths, the one (two of four) with the shortest way to the guns that never shows him to them (lanes.ts round)
pub fn round(g: &mut crate::game::Game, l: &mut crate::lanes::Lane, at: &[GuardId]) {
    l.round_at = g.time + 10.0;   // (asked again while there is no way: a real go waits `roundGap`)
    let mask = s(g).mask.clone();
    let pen = price_of(g, mask.as_deref(), None);
    if pen.is_none() { return; }
    let mut ways: Vec<Way> = Vec::new();
    let mut why = String::new();
    for &gd in at {
        if s(g).flank_of.has(gd) { continue; }   // (already round, set on the lane's other side: he holds it from there)
        let start = pos_of(g, gd);
        let sign = g.gd[gd].callsign.clone();
        let p = match find_path(g, start, l.from, 8000.0, pen.as_ref()) {
            Some(p) if !p.is_empty() => p,
            _ => { why.push_str(&format!(" {}: no way;", sign)); continue; }
        };
        if flat(p[p.len() - 1], l.from) > 2.5 { why.push_str(&format!(" {}: no way to the guns;", sign)); continue; }
        let (mut len, mut a, mut show, mut shown, mut pin) = (0.0f64, start, None::<V3>, 0.0f64, false);
        let mut pts: Vec<V3> = Vec::new();
        for &b in p.iter() {
            let d = flat(a, b);
            let k = m::max(1.0, m::ceil(d / 0.4));
            if show.is_none() {
                let mut i = 1.0;
                while i <= k {
                    let q: V3 = [a[0] + (b[0] - a[0]) * i / k, 0.0, a[2] + (b[2] - a[2]) * i / k];
                    if in_mask(g, mask.as_deref(), q) { show = Some(q); shown = len + d * i / k; break; }
                    pts.push(q);
                    i += 1.0;
                }
            }
            len += d;
            a = b;
        }
        let stop = if show.is_some() && pts.len() >= 4 { Some(pts[pts.len() - 3]) } else { None };   // (where he sets: a stride short of where the guns would first have him)
        if len > g.k.lanes.round_max_m { why.push_str(&format!(" {}: {} m, too far;", sign, js::to_fixed(len, 0))); continue; }
        // where the guns first have him on that way: only the last stretch of it, and round from the lane's own line
        if let Some(show) = show {
            let sd = flat(show, l.from);
            let ll = js::or(flat(l.from, l.at), 1.0);
            let (lx, lz) = ((l.at[0] - l.from[0]) / ll, (l.at[2] - l.from[2]) / ll);
            let cs = ((show[0] - l.from[0]) * lx + (show[2] - l.from[2]) * lz) / js::or(sd, 1.0);
            pin = len - shown > g.k.lanes.round_show_m;   // (shown to the guns for longer than a man walks in their sight: a way only as far as where they would first see him)
            let (show_m, pin_show, field, side) = (g.k.lanes.round_show_m, g.k.lanes.pin_show_m, g.k.lanes.field_m, g.k.lanes.round_side_deg);
            if len - shown > m::max(show_m, pin_show) || sd > field + 1.0 || cs > m::cos(side * m::PI / 180.0) {
                why.push_str(&format!(" {}: {} m, shown the last {} m, {}° round;", sign, js::to_fixed(len, 0), js::to_fixed(len - shown, 0), js::to_fixed(m::acos(m::max(-1.0, m::min(1.0, cs))) * 180.0 / m::PI, 0)));
                continue;
            }
        }
        if pin && stop.is_none() { why.push_str(&format!(" {}: no floor to stop on short of their sight;", sign)); continue; }
        ways.push(Way { gd, cid: char_id_of(g, gd), len, pin, stop });
    }
    if ways.is_empty() { note(g, None, &format!("lane {}: no way round —{}", l.id, why)); return; }
    js::sort_by(&mut ways, |a, b| js::or(a.len - b.len, a.cid - b.cid));
    let mut n = m::min_n(&[g.k.lanes.round_men, ways.len() as f64, m::max(1.0, at.len() as f64 - 1.0), if at.len() >= 4 { 2.0 } else { 1.0 }]);
    // a way that ends at a second angle is taken only if one of the men who stay can fire from the first; else the ways that hide their man to the end
    if ways[..n as usize].iter().any(|w| w.pin) {
        let mut stay: JsSet<GuardId> = JsSet::new();
        for w in ways[..n as usize].iter() { stay.add(w.gd); }
        let min_m = g.k.lanes.pin_min_m;
        if stepper(g, l, Some(&stay), true, Some(min_m)).is_none() {
            ways.retain(|w| !w.pin);
            if ways.is_empty() { note(g, None, &format!("lane {}: a way to its other side, but nobody placed to fire from this one —{}", l.id, why)); return; }
            n = m::min(n, ways.len() as f64);
        }
    }
    let mut i = 0usize;
    while (i as f64) < n {
        let (gd, len, way_pin, stop) = (ways[i].gd, ways[i].len, ways[i].pin, ways[i].stop);
        let first = i == 0;
        i += 1;
        let d = match def_of(g, gd) { Some(d) => d, None => continue };
        let (now, st) = (g.time, s(g));
        st.held.delete(gd);
        st.flank.set(gd, now + m::min(60.0, len / 2.0 + 10.0));
        if let (true, Some(stop)) = (way_pin, stop) {   // its end too long in their sight to walk: as far as where the guns would first see him, to hold the lane from that side
            st.flank_of.set(gd, crate::lanes::Flank { lane: l.id, phase: crate::lanes::FlankPhase::Walk, at: stop, t: now, n: 0.0 });
            clear_path(g, gd);
            crate::pressure::run(g, d, stop, None, Some(0.0));
            g.gd[gd].last_known = Some(l.from);
            s(g).stats.round += 1.0;
            if first { round_shout(g, gd); }
            note(g, Some(gd), &format!("lane {}: goes round, {} m, as far as ({}, {}) on its other side", l.id, js::to_fixed(len, 0), js::to_fixed(stop[0], 1), js::to_fixed(stop[2], 1)));
            continue;
        }
        crate::pressure::loose(g, d, l.from);   // (his way stays priced, but nothing here stops him short of the guns he was sent at)
        g.gd[gd].last_known = Some(l.from);
        s(g).stats.round += 1.0;
        if first { round_shout(g, gd); }
        note(g, Some(gd), &format!("lane {}: goes round, {} m", l.id, js::to_fixed(len, 0)));
    }
    l.round_at = g.time + g.k.lanes.round_gap;
}
/// (lanes.ts:505, 510) the first man sent round says so
fn round_shout(g: &mut Game, gd: GuardId) {
    let text = crate::words::wg(g, &"going around — keep them looking your way!", &"going round — keep them looking your way!");
    say_net(g, gd, text);
    s(g).said_t = g.time;
}

/// THE CHARGE: a man at a mouth with a stun charge on him and the guns in a throw's reach puts it down the lane (lanes.ts stun)
pub fn stun(g: &mut crate::game::Game, l: &mut crate::lanes::Lane, at: &[GuardId]) {
    l.stun_at = g.time + 2.0;
    for &o in g.guards.iter() {
        if g.gd[o].team == SIDE && g.gd[o].state != crate::game::GuardState::Dead && flat(g.ch[g.gd[o].ch].pos, l.from) < 6.0 { return; }   // (one of theirs is on the guns already)
    }
    let mut best: Option<GuardId> = None;
    let mut bd = g.k.lanes.stun_m;
    for &gd in at {
        if crate::tactics::stuns_of(g, gd) <= 0.0 { continue; }
        let d = flat(pos_of(g, gd), l.from);
        if d < bd && d > 4.0 { best = Some(gd); bd = d; }
    }
    let best = match best { Some(b) => b, None => return };
    let c = g.gd[best].ch;
    let cp = g.ch[c].pos;
    let (tx, tz) = (l.from[0] - cp[0], l.from[2] - cp[2]);
    let tl = m::hyp2(tx, tz);
    // his arm round the corner he holds: the can leaves his hand half a pace toward the lane's line, and is asked to land short of the guns, in their eyes
    let mouth = mouth_of(g, l, cp);
    let from = crate::frag::arm_round(g, cp, mouth);
    let land: V3 = [l.from[0] - tx / tl * 1.6, 0.0, l.from[2] - tz / tl * 1.6];
    let sol = crate::throwables::throwables_solve(g, from, land, Some(11.0));
    if flat(sol.land, land) > 3.0 || flat(sol.land, l.from) > 4.5 { return; }   // (no arc from here: a wall, a lintel, too far)
    let left = crate::tactics::stuns_of(g, best) - 1.0;
    crate::tactics::give_stuns(g, best, left);
    crate::character::anim_throw_item(g, c, Some(flat(cp, sol.land)), Some(m::hyp2(sol.v0[0], sol.v0[2])));
    let cid = g.ch[c].id;
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, from, sol.v0, Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(0.7), who: Some(cid), heading: Some(v3::yaw_of(sol.v0)), ..Default::default() }));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::audio_iface::game_audio_like_play(g, "throw", Some(from), Some(0.7), None);
    l.can = Some(crate::lanes::LaneCan { who: cid, t: g.time, land: sol.land });
    l.stun_at = g.time + g.k.lanes.stun_gap;
    s(g).stats.stuns += 1.0;
    say_net(g, best, "bang out — on me!");
    s(g).said_t = g.time;
    note(g, Some(best), &format!("lane {}: a charge down it toward ({}, {})", l.id, js::to_fixed(land[0], 1), js::to_fixed(land[2], 1)));
}

/// a stun charge of THEIRS has gone off on a lane's guns with nothing solid between: the lane is theirs for `pushSecs`, and the men on its mouths go (lanes.ts bangs)
pub fn bangs(g: &mut crate::game::Game) {
    let mut ei = 0usize;
    while ei < g.events.len() {
        let e = g.events[ei];
        ei += 1;
        if e.kind != crate::director::EventKind::Bang || e.frag == Some(true) || s(g).banged.has(e.serial) { continue; }
        s(g).banged.add(e.serial);
        let (epos, b_m) = (e.pos, g.k.lanes.bang_m);
        let mut ours = false;
        let mut k = 0;
        while k < s(g).cans.len() {
            let it = s(g).cans[k];
            k += 1;
            if flat(crate::throwables::item_pos(g, it), epos) < 1.5 { ours = true; break; }
        }
        let b: V3 = [epos[0], m::max(0.5, epos[1]), epos[2]];
        let mut li = 0;
        while li < s(g).lanes.len() {
            let i = li;
            li += 1;
            let now = g.time;
            let (known, push_until, id, from) = { let x = &s(g).lanes[i]; (x.known, x.push_until, x.id, x.from) };
            if !known || now < push_until { continue; }
            let mut near = false;
            if ours {
                let guns = s(g).lanes[i].guns.clone();
                for q in guns {
                    if flat(epos, q) < b_m && !solid_blocked(g, b, [q[0], 1.5, q[2]]) { near = true; break; }
                }
            }
            if !near {
                if let Some(can) = s(g).lanes[i].can {
                    if now - can.t < 4.0 && flat(epos, can.land) < 3.0 {
                        note(g, None, &format!("lane {}: the charge fell short, nobody goes", id));
                        s(g).lanes[i].can = None;
                    }
                }
                continue;
            }
            let until = now + g.k.lanes.push_secs;
            let x = &mut s(g).lanes[i];
            x.can = None;
            x.push_until = until;
            x.round_at = m::max(x.round_at, until + 2.0);
            s(g).stats.pushes += 1.0;
            let to: V3 = [epos[0], 0.0, epos[2]];
            let mut first = true;
            let rows: Vec<(GuardId, f64)> = s(g).held.iter().map(|(gd, h)| (gd, h.lane)).collect();
            for (gd, lane) in rows {
                if lane != id || !fit(g, gd) || flat(pos_of(g, gd), from) > g.k.lanes.push_m { continue; }
                let d = match def_of(g, gd) { Some(d) => d, None => continue };
                s(g).held.delete(gd);
                s(g).push.set(gd, until);
                clear_path(g, gd);
                g.gd[gd].last_known = Some(from);
                let fast = g.k.lanes.push_fast;
                crate::pressure::run(g, d, to, None, Some(fast));   // (a straight run at where the charge went off: the room fight plants him on the first man he sees)
                if first { first = false; say_net(g, gd, "they're blind — go, go!"); s(g).said_t = g.time; }
                note(g, Some(gd), &format!("lane {}: pushes down it behind the charge", id));
            }
        }
    }
}

/// (lanes.ts:564) where this body stands, if it is by the lane's guns and `eye` has a line to it
fn manned_at(g: &mut Game, l: &crate::lanes::Lane, eye: V3, c: CharId) -> Option<V3> {
    let (pos, near) = (g.ch[c].pos, g.k.lanes.peek_near_m);
    if !l.guns.iter().any(|&q| flat(q, pos) < near) { return None; }
    let head = g.ch[c].bones.get(crate::character::Bone::Head).copied().unwrap_or_else(|| v3::add(pos, [0.0, 1.65, 0.0]));
    if !seg_blocked(g, eye, head) { return Some(pos); }
    let chest = g.ch[c].bones.get(crate::character::Bone::Chest).copied().unwrap_or_else(|| v3::add(pos, [0.0, 1.1, 0.0]));
    if !seg_blocked(g, eye, chest) { Some(pos) } else { None }
}
/// one of the other side, on his feet, that `eye` has a line to within `peekNearM` of where a lane's fire came from: a LINE, so the dark never makes a lane clear (lanes.ts gunsManned)
pub fn guns_manned(g: &mut crate::game::Game, l: &crate::lanes::Lane, eye: V3) -> Option<V3> {
    let who = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    for pl in crate::seats::men(g, &who) {
        let pc = g.pl[pl].ch;
        if g.ch[pc].alive {
            let id = g.ch[pc].id;
            let t = crate::teams::team_of_id(g, id);
            if crate::teams::is_hostile(g, SIDE, t) { if let Some(p) = manned_at(g, l, eye, pc) { return Some(p); } }
        }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if is_dead(g, o) || g.gd[o].held.is_some() { continue; }
        let t = g.gd[o].team;
        if crate::teams::is_hostile(g, SIDE, t) { let c = g.gd[o].ch; if let Some(p) = manned_at(g, l, eye, c) { return Some(p); } }
    }
    None
}
/// has he the PLACE round a gun's spot in view: the spot, and all but one at most of a ring of six a man at the spot could stand on (lanes.ts spotSeen)
pub fn spot_seen(g: &mut crate::game::Game, eye: V3, m: V3) -> bool {
    let c: V3 = [m[0], 1.4, m[2]];
    if seg_blocked(g, eye, c) { return false; }
    let mut miss = 0;
    for k in 0..6 {
        let a = k as f64 * m::PI / 3.0;
        let q: V3 = [m[0] + m::cos(a) * SPOT_RING, 1.4, m[2] + m::sin(a) * SPOT_RING];
        if !solid_blocked(g, c, q) && seg_blocked(g, eye, q) { miss += 1; if miss > 1 { return false; } }
    }
    true
}
/// another of his side, fit, has had one of them in sight within 3 s somewhere by the lane's guns (lanes.ts netSees)
pub fn net_sees(g: &mut crate::game::Game, l: &crate::lanes::Lane, but: GuardId) -> bool {
    let near = g.k.lanes.peek_near_m;
    let mut i = 0;
    while i < g.fight.defs.len() {
        let o = g.tab.roomfight_def[g.fight.defs[i]].gd;
        i += 1;
        if o == but || !fit(g, o) || g.time - g.gd[o].last_seen_t > 3.0 { continue; }
        let k = match g.gd[o].last_known { Some(k) => k, None => continue };
        if l.guns.iter().any(|&q| flat(q, k) < near) { return true; }
    }
    false
}

/// of the men this rule holds short of a lane's mouth, the nearest who can step to where it opens, and where he steps to (lanes.ts stepper)
pub fn stepper(g: &mut crate::game::Game, l: &crate::lanes::Lane, skip: Option<&JsSet<GuardId>>, armed: bool, min_gun: Option<f64>) -> Option<crate::lanes::Stepper> {
    let min_gun = min_gun.unwrap_or(0.0);
    let mut best: Option<GuardId> = None;
    let mut bd = f64::INFINITY;
    let mut to: Option<V3> = None;
    let mut di = 0;
    while di < g.fight.defs.len() {
        let dd = g.fight.defs[di];
        di += 1;
        let gd = g.tab.roomfight_def[dd].gd;
        let h = s(g).held.get(gd).copied();
        if skip.map_or(false, |x| x.has(gd)) || mode_of(g, dd) != crate::roomfight::DefMode::Hold || !fit(g, gd) || !knows(g, l, gd) || g.time - g.gd[gd].last_seen_t < 1.5 || pushing(g, gd) || flanking(g, gd) { continue; }
        let h = match h { Some(h) => h, None => continue };
        if h.lane != l.id || !h.aimed || g.time - h.t < 1.0 { continue; }   // (only a man this rule itself stopped short of THIS lane)
        if armed {
            let w = g.gd[gd].weapon;
            if g.gd[gd].reload_t >= 0.0 || !crate::weapons::firearm_can_fire(g, w) || crate::weapons::firearm_rounds_ready(g, w) < g.k.lanes.pin_rounds { continue; }
        }
        let p = pos_of(g, gd);
        let mut mouth = mouth_of(g, l, p);
        let mut d = flat(p, mouth);
        let mut on: Option<V3> = None;
        if d > g.k.lanes.peek_reach_m || d < 0.3 { continue; }
        {
            let mask = mask_of(g, gd);
            let pen = price_of(g, mask.as_deref(), None);
            let way = match find_path(g, p, mouth, 1500.0, pen.as_ref()) { Some(w) if !w.is_empty() => w, _ => continue };
            if flat(way[way.len() - 1], mouth) > 0.8 { continue; }   // (a few steps to it, not a walk round the building…
            let (mut len, mut q, mut crosses) = (0.0f64, p, false);
            for &w in way.iter() {
                len += flat(q, w);
                q = w;
                if mask.is_some() && flat(w, mouth) > 0.8 && in_mask(g, mask.as_deref(), w) { crosses = true; }
            }
            if crosses && mask.is_some() {   // the mouth he SEES lies beyond the lane's own first floor on his walk: that first floor is where it opens before his feet
                let (mut run, mut a, mut hit) = (0.0f64, p, None::<V3>);
                'walk: for &w in way.iter() {
                    let sl = flat(a, w);
                    let n = m::max(1.0, m::ceil(sl / 0.2));
                    let mut k = 1.0;
                    while k <= n {
                        let x: V3 = [a[0] + (w[0] - a[0]) * k / n, 0.0, a[2] + (w[2] - a[2]) * k / n];
                        if in_mask(g, mask.as_deref(), x) {
                            hit = Some(x);
                            run += sl * k / n;
                            on = if sl > 1e-3 { Some([(w[0] - a[0]) / sl, 0.0, (w[2] - a[2]) / sl]) } else { None };
                            break 'walk;
                        }
                        k += 1.0;
                    }
                    run += sl;
                    a = w;
                }
                let i = match hit { Some(x) => cell_at(g, x), None => -1.0 };
                if hit.is_some() && i >= 0.0 && l.cells.as_ref().map_or(false, |c| c.has(cell_key(i))) && run >= 0.3 {
                    mouth = cell_mid(g.col.nav.cell, g.col.nav.w, i);
                    d = m::max(0.3, flat(p, mouth));
                    len = run;
                    crosses = false;
                }   // (another lane's floor first: not his to cross)
            }
            if len > g.k.lanes.peek_reach_m * 1.4 || crosses { continue; }   // (…and never one that crosses a covered lane's floor on the way to this one's mouth)
        }
        if min_gun > 0.0 && l.guns.iter().any(|&q| flat(q, mouth) < min_gun) { continue; }   // (a corner that near the guns is no place to stand and fire from)
        let score = d + char_id_of(g, gd) * 1e-4;
        if score >= bd { continue; }
        best = Some(gd);
        bd = score;
        let mut t: V3 = match on {
            Some(on) => [mouth[0] + on[0] * 0.6, 0.0, mouth[2] + on[2] * 0.6],
            None => [mouth[0] + (mouth[0] - p[0]) / d * 0.6, 0.0, mouth[2] + (mouth[2] - p[2]) / d * 0.6],
        };   // (a stride on into it: he stops the moment he has a line down it)
        if crate::scene_collision::nav_grid_is_blocked(g, t[0], t[2]) { t = mouth; }
        to = Some(t);
    }
    match (best, to) { (Some(gd), Some(to)) => Some(crate::lanes::Stepper { gd, to }), _ => None }
}

/// the men sent round to a lane's other side, twice a second: on his way → SET where the guns would first see him; while one is set its men fire down it in turn (lanes.ts flankTick)
pub fn flank_tick(g: &mut crate::game::Game, l: &mut crate::lanes::Lane) {
    use crate::lanes::FlankPhase;
    let mut set = 0;
    for gd in s(g).flank_of.keys() {
        let mut f = match s(g).flank_of.get(gd) { Some(&f) => f, None => continue };
        if f.lane != l.id { continue; }
        let d = match def_of(g, gd) {
            Some(d) if fit(g, gd) && !pushing(g, gd) => d,
            _ => {   // (down, blind, or in behind a charge: no longer this errand's, and no longer let past the gate as a man going round)
                s(g).flank_of.delete(gd);
                if f.phase == FlankPhase::Walk { s(g).flank.delete(gd); }
                continue;
            }
        };
        if f.phase == FlankPhase::Walk {
            if !flanking(g, gd) { continue; }   // (too long on the way: let go of, the entry gone with it)
            if g.time - g.gd[gd].last_seen_t < 1.0 { s(g).flank_of.delete(gd); s(g).flank.delete(gd); continue; }   // (met one of them on the way: the fight's)
            if flat(pos_of(g, gd), f.at) > 0.8 {   // not there yet
                let sg = goal_of(g, gd);
                if mode_of(g, d) == crate::roomfight::DefMode::Move {   // (still walking it, or sent elsewhere meanwhile)
                    if let Some(sg) = sg { if flat(sg, f.at) < 0.8 { continue; } }
                    s(g).flank_of.delete(gd); s(g).flank.delete(gd);
                    continue;
                }
                if g.time - f.t < 2.0 { continue; }
                f.n += 1.0;
                if f.n > 4.0 { s(g).flank_of.delete(gd); s(g).flank.delete(gd); continue; }   // (walked on, a few times, then let go)
                f.t = g.time;
                s(g).flank_of.set(gd, f);
                clear_path(g, gd);
                crate::pressure::run(g, d, f.at, None, Some(0.0));
                continue;
            }
            s(g).flank.delete(gd);
            let p = pos_of(g, gd);
            f.phase = FlankPhase::Set; f.t = g.time; f.at = p;
            s(g).flank_of.set(gd, f);
            s(g).stats.sets += 1.0;
            let look = mouth_of(g, l, p);
            clear_path(g, gd);
            if crate::roomfight::room_fight_lane_stop(g, gd, look) {
                let (at, now) = (pos_of(g, gd), g.time);
                s(g).held.set(gd, crate::lanes::StateHeldValue { lane: l.id, t: now, look, aimed: true, at });
            }
            if g.time - s(g).said_t > 2.0 { s(g).said_t = g.time; say_net(g, gd, "set — I've got an angle on them!"); }
            let at = pos_of(g, gd);
            note(g, Some(gd), &format!("lane {}: round and set at ({}, {}), {} m from its guns: holds it from this side", l.id, js::to_fixed(at[0], 1), js::to_fixed(at[2], 1), js::to_fixed(flat(at, l.from), 0)));
            l.pin_at = m::max(l.pin_at, g.time + 1.5);
        }
        if f.phase == FlankPhase::Set {
            let held_here = s(g).held.get(gd).map_or(false, |h| h.lane == l.id) && mode_of(g, d) == crate::roomfight::DefMode::Hold;
            if !held_here && s(g).peek.get(gd).map(|p| p.lane) != Some(l.id) { s(g).flank_of.delete(gd); continue; }   // (taken by the fight or another errand)
            set += 1;
        }
    }
    if l.can.is_some() || g.time < l.pin_at || set == 0 { return; }
    if l.pin_n >= g.k.lanes.pin_max { l.pin_n = 0.0; l.pin_at = g.time + g.k.lanes.pin_rest; return; }   // (the volley fired: quiet a while)
    pin_start(g, l);
}

/// the run-in under the fire: LANES.pinGo is settled off on the page and nobody calls this (lanes.ts flankGo)
pub fn flank_go(g: &mut crate::game::Game, l: &mut crate::lanes::Lane) { todo!("not translated: LANES.pinGo is settled off") }

/// FIRE FROM AN ANGLE: of the men held on the lane with a burst in his gun and a corner no nearer the guns than `pinMinM`, in turn, one steps to it (lanes.ts pinStart)
pub fn pin_start(g: &mut crate::game::Game, l: &mut crate::lanes::Lane) {
    l.pin_at = g.time + 1.0;   // (asked again soon while nobody is placed for it)
    if s(g).peek.values().any(|p| p.lane == l.id) { return; }
    let min_m = g.k.lanes.pin_min_m;
    let mut pick = None;
    if l.pin_by >= 0.0 {   // (in turn: not the man who fired last, if another is placed)
        let mut last: JsSet<GuardId> = JsSet::new();
        for i in 0..g.fight.defs.len() { let o = g.tab.roomfight_def[g.fight.defs[i]].gd; if char_id_of(g, o) == l.pin_by { last.add(o); } }
        pick = stepper(g, l, Some(&last), true, Some(min_m));
    }
    if pick.is_none() { pick = stepper(g, l, None, true, Some(min_m)); }
    let pick = match pick { Some(p) => p, None => return };
    let gd = pick.gd;
    let d = match def_of(g, gd) { Some(d) => d, None => return };
    let p = pos_of(g, gd);
    let look = match s(g).held.get(gd).map(|h| h.look) { Some(x) => x, None => mouth_of(g, l, p) };
    l.pin_at = g.time + g.k.lanes.pin_gap;
    l.pin_n += 1.0;
    l.peek_at = m::max(l.peek_at, g.time + g.k.lanes.peek_gap);   // (he looks as he fires: no look for news on top of it)
    let row = crate::lanes::Peek { lane: l.id, phase: crate::lanes::PeekPhase::Out, t: g.time, home: p, look, to: pick.to, line_t: -1.0, eye: None,
        fire: Some(crate::lanes::PeekFire { n: g.k.lanes.pin_rounds, t: -1.0, next: 0.0, saw: false, line_t: -1.0 }), frag: None };
    s(g).held.delete(gd);
    s(g).peek.set(gd, row);
    clear_path(g, gd);
    let fast = g.k.lanes.pin_pace - g.tune.guard_chase;
    crate::pressure::run(g, d, pick.to, None, Some(fast));
    {   // (his gun on where the fire came from before he is round the corner)
        let mut q = if l.guns.is_empty() { l.from } else { l.guns[0] };
        let mut md = f64::INFINITY;
        for &x in l.guns.iter() { let dd = flat(x, pick.to); if dd < md { md = dd; q = x; } }
        aim_at(g, gd, [q[0], 1.2, q[2]]);
    }
    s(g).stats.pins += 1.0;
    l.pin_by = char_id_of(g, gd);
    note(g, Some(gd), &format!("lane {}: steps to his corner toward ({}, {}) to put fire down it", l.id, js::to_fixed(pick.to[0], 1), js::to_fixed(pick.to[2], 1)));
}

/// (lanes.ts:686 done) the burst is over: said, a magazine started if it is low, and back
fn pin_done(g: &mut Game, gd: GuardId, l: &crate::lanes::Lane, d: RoomfightDefId, fr: crate::lanes::PeekFire, why: &str) {
    let rounds = g.k.lanes.pin_rounds;
    let spent = rounds - fr.n;
    let shots = if js::truthy(spent) { format!(" — {} round{} down it", spent, if spent == 1.0 { "" } else { "s" }) } else { String::new() };
    note(g, Some(gd), &format!("lane {}: {}{}{}: back", l.id, why, shots, if fr.saw { ", they are still on it" } else { "" }));
    let w = g.gd[gd].weapon;
    if crate::weapons::firearm_rounds_ready(g, w) < rounds && g.gd[gd].reload_t < 0.0 { lane_reload(g, gd); }
    peek_back(g, gd, d);
}
/// every frame, a man out at his corner to fire down a lane: the moment his muzzle has a line he stops and puts his burst there, wide, then gets back (lanes.ts pinStep)
pub fn pin_step(g: &mut crate::game::Game, gd: GuardId, l: &mut crate::lanes::Lane, d: RoomfightDefId, seen: Option<V3>) {
    let c = g.gd[gd].ch;
    let p = match s(g).peek.get(gd) { Some(&p) => p, None => return };   // (a read of the row: its writes go by the path)
    let mut fr = match p.fire { Some(f) => f, None => return };
    if let (Some(seen), false) = (seen, fr.saw) {   // (seen still on it: the lane stands; no news asked for, so the looks' own wait is not doubled)
        fr.saw = true;
        l.until = m::max(l.until, g.time + g.k.lanes.peek_keep);
        l.peek_part = 0.0;
        if l.guns.len() < 4 && l.guns.iter().all(|&q| flat(q, seen) > 1.5) { l.guns.push([seen[0], 0.0, seen[2]]); }
        put_fire(g, gd, fr);
    }
    let mut tgt: Option<V3> = None;
    let mut bd = f64::INFINITY;
    let e = crate::glasspen::shot_energy(g, c, None);
    for i in 0..l.guns.len() {
        let q0 = l.guns[i];
        let q: V3 = [q0[0], 1.2, q0[2]];
        let muzzle = g.ch[c].muzzle;
        if crate::scene_collision::shot_blocked(g, muzzle, q, Some(0.02), e) { continue; }
        let dd = flat(q0, seen.unwrap_or(g.ch[c].pos));
        if dd < bd { bd = dd; tgt = Some(q); }
    }
    if fr.t >= 0.0 && g.time - fr.t > g.k.lanes.pin_snap { return pin_done(g, gd, l, d, fr, "his moment at the corner is up"); }   // (a SNAP burst: no longer in their sight than a look is)
    let tgt = match tgt {
        Some(t) => t,
        None => {
            let give_up = if fr.t >= 0.0 { g.time - fr.line_t > 0.2 } else { mode_of(g, d) != crate::roomfight::DefMode::Move || g.time - p.t > 6.0 };
            if give_up { pin_done(g, gd, l, d, fr, "no line down it for his fire"); }
            return;
        }
    };
    if fr.t < 0.0 {   // the first line: NOT a step further out: he fires from where his muzzle first clears the corner, and a line lost to his own recoil ends the burst
        fr.t = g.time;
        put_fire(g, gd, fr);
        let here = g.ch[c].pos;
        if let Some(r) = s(g).peek.get_mut(gd) { r.to = here; }
        clear_path(g, gd);
        crate::pressure::run(g, d, here, None, Some(0.0));
    }
    fr.line_t = g.time;
    put_fire(g, gd, fr);
    aim_at(g, gd, tgt);
    let (muzzle, gun) = (g.ch[c].muzzle, g.ch[c].gun_dir);
    let (dx, dy, dz) = (tgt[0] - muzzle[0], tgt[1] - muzzle[1], tgt[2] - muzzle[2]);
    let dl = js::or(m::hyp3(dx, dy, dz), 1.0);
    let on = (gun[0] * dx + gun[1] * dy + gun[2] * dz) / dl > m::cos(g.k.lanes.pin_aim_deg * m::PI / 180.0);
    let at = v3::mad(muzzle, gun, dl);   // (down his BORE: the round goes where the gun points, and the mates in its way are asked along that)
    let w = g.gd[gd].weapon;
    let spread = g.k.lanes.pin_spread;
    if on && g.time >= fr.next && !g.gd[gd].muzzle_down && !crate::guards::round_past_colleague(g, gd, at, None, Some(spread * 0.75)) && crate::weapons::firearm_fire(g, w) {
        crate::combat::fire_weapon(g, c, at, false, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(spread), ..Default::default() }));
        if fr.n == g.k.lanes.pin_rounds && l.pin_n == 1.0 { s(g).said_t = g.time; say_net(g, gd, "putting fire on them!"); }
        fr.n -= 1.0;
        let cd = g.m.weapons.weapons.get(g.fa[w].def).unwrap().cd;
        fr.next = g.time + m::max(g.k.lanes.pin_every, cd);
        s(g).stats.pin_rounds += 1.0;
        put_fire(g, gd, fr);
    }
    if fr.n <= 0.0 { pin_done(g, gd, l, d, fr, "his burst is fired"); }
    else if !crate::weapons::firearm_can_fire(g, w) && crate::weapons::firearm_rounds_ready(g, w) == 0.0 { pin_done(g, gd, l, d, fr, "dry"); }
}
/// (lanes.ts:682 `F`) the burst, written to the man's row
fn put_fire(g: &mut Game, gd: GuardId, fr: crate::lanes::PeekFire) {
    if let Some(r) = s(g).peek.get_mut(gd) { r.fire = Some(fr); }
}

/// he has shouted and frag.ts's wind-up has his arm (half a second past its time at the most) (lanes.ts winding)
pub fn winding(g: &mut crate::game::Game, gd: GuardId, f: crate::lanes::PeekFrag) -> bool {
    f.at.is_some() && crate::frag::frag_winding(g, gd) && g.time < f.until + 0.5
}
/// A GRENADE DOWN IT: of the men held on the lane who carry one and may throw this moment, one goes looking for an arc to the middle of where its fire came from (lanes.ts fragOut)
pub fn frag_out(g: &mut crate::game::Game, l: &mut crate::lanes::Lane, at: &[GuardId]) {
    l.frag_at = g.time + 2.0;   // (asked again soon while nobody is placed for it)
    if at.is_empty() { return; }
    if s(g).peek.values().any(|p| p.lane == l.id) { return; }
    if s(g).flank_of.values().any(|f| f.lane == l.id && f.phase != crate::lanes::FlankPhase::Set) { return; }   // (not on the head of a mate walking round to them)
    let mid = guns_mid(l);
    let mut can: Vec<GuardId> = Vec::new();
    for &gd in at {
        let (no, h) = (s(g).frag_no.get(gd).copied(), s(g).held.get(gd).copied());   // (`at` was read before this tick's charge and its men sent round: one of them may be held no longer)
        let h = match h { Some(h) => h, None => continue };
        if !(h.lane == l.id && h.aimed) { continue; }
        let holds = match def_of(g, gd) { Some(d) => mode_of(g, d) == crate::roomfight::DefMode::Hold, None => false };
        if holds && !pushing(g, gd) && !flanking(g, gd) && !no.map_or(false, |no| no.lane == l.id && flat(no.at, mid) < 1.5) && crate::frag::frag_ready(g, gd, mid) { can.push(gd); }
    }
    if can.is_empty() { return; }
    let mut skip: JsSet<GuardId> = JsSet::new();
    for i in 0..g.fight.defs.len() { let o = g.tab.roomfight_def[g.fight.defs[i]].gd; if !can.contains(&o) { skip.add(o); } }
    let near = g.k.frag.def_near;
    let pick = stepper(g, l, Some(&skip), false, Some(near));
    let gd = match pick { Some(p) => p.gd, None => can[0] };
    let p = pos_of(g, gd);
    let look = match s(g).held.get(gd).map(|h| h.look) { Some(x) => x, None => mouth_of(g, l, p) };
    s(g).held.delete(gd);
    let to = match pick { Some(x) => x.to, None => mouth_of(g, l, p) };
    let row = crate::lanes::Peek { lane: l.id, phase: crate::lanes::PeekPhase::Out, t: g.time, home: p, look, to, line_t: -1.0, eye: None, fire: None,
        frag: Some(crate::lanes::PeekFrag { at: None, k: 0.0, next: 0.0, walk: pick.is_some(), going: false, edge: false, own: false, until: 0.0, had: 0.0 }) };
    s(g).peek.set(gd, row);
}
/// the middle of where a lane's fire came from: of the places its guns fired from, the one nearest their mean (lanes.ts gunsMid)
pub fn guns_mid(l: &crate::lanes::Lane) -> V3 {
    if l.guns.is_empty() { return l.from; }
    let (mut mx, mut mz) = (0.0f64, 0.0f64);
    for q in l.guns.iter() { mx += q[0]; mz += q[2]; }
    mx /= l.guns.len() as f64;
    mz /= l.guns.len() as f64;
    let (mut best, mut bd) = (l.guns[0], f64::INFINITY);
    for q in l.guns.iter() { let d = m::hyp2(q[0] - mx, q[2] - mz); if d < bd { bd = d; best = *q; } }
    best
}
/// where his feet would be `s` metres on: along the walk he is on (`going`), else straight toward `to`. Edge = a covered lane's floor, or past the end of his walk; None = a wall on the straight line (lanes.ts fragPlace)
pub fn frag_place(g: &mut crate::game::Game, gd: GuardId, to: V3, s: f64, going: bool) -> Option<crate::lanes::FragPlace> {
    use crate::lanes::FragPlace;
    let c = pos_of(g, gd);
    if s <= 0.0 { return Some(FragPlace::List(c)); }
    let b: V3;
    if going && g.gd[gd].path_i < g.gd[gd].path.len() {
        let (mut left, mut a) = (0.0f64, c);
        let mut i = g.gd[gd].path_i;
        while i < g.gd[gd].path.len() && left < s { let q = g.gd[gd].path[i]; left += flat(a, q); a = q; i += 1; }
        if left < s { return Some(FragPlace::Edge); }
        b = along(g, gd, s);
    } else {
        let (dx, dz) = (to[0] - c[0], to[2] - c[2]);
        let d = m::hyp2(dx, dz);
        if s > d { return Some(FragPlace::Edge); }
        b = [c[0] + dx / d * s, 0.0, c[2] + dz / d * s];
        if crate::scene_collision::nav_grid_is_blocked(g, b[0], b[2]) || solid_blocked(g, [c[0], 1.1, c[2]], [b[0], 1.1, b[2]]) { return None; }
    }
    let mask = mask_of(g, gd);
    if in_mask(g, mask.as_deref(), b) { Some(FragPlace::Edge) } else { Some(FragPlace::List(b)) }
}
/// every frame, a man out to put a grenade down a lane: an arc from where he is held, else a place at a time on along his walk; he shouts, throws, goes back (lanes.ts fragStep)
pub fn frag_step(g: &mut crate::game::Game, gd: GuardId, l: &mut crate::lanes::Lane, d: RoomfightDefId) {
    let p = match s(g).peek.get(gd) { Some(&p) => p, None => return };
    let mut f = match p.frag { Some(f) => f, None => return };
    let mut to = p.to;
    frag_step_on(g, gd, p.t, &mut to, &mut f, l, d);
    if let Some(r) = s(g).peek.get_mut(gd) { r.frag = Some(f); r.to = to; }   // (only these two fields: peek_back has written phase and t on the row)
}
fn frag_step_on(g: &mut Game, gd: GuardId, t0: f64, to: &mut V3, f: &mut crate::lanes::PeekFrag, l: &mut crate::lanes::Lane, d: RoomfightDefId) {
    use crate::frag::FragThrowAt;
    use crate::lanes::FragPlace;
    let pace = g.k.lanes.peek_pace - g.tune.guard_chase;
    let big_n = if f.going { m::max(1.0, m::round(g.k.lanes.frag_step_m / 0.3)) + 1.0 } else { 1.0 };
    if let Some(was) = f.at {
        if winding(g, gd, *f) { return; }
        let went = crate::frag::frag_away(g, gd) > f.had;
        if went { s(g).stats.frags += 1.0; }
        else {   // (it stayed in his hand: he does not shout that place again; another carrier may)
            s(g).stats.frag_kept += 1.0;
            l.frag_at = g.time + 6.0;
            s(g).frag_no.set(gd, crate::lanes::StateFragNoValue { lane: l.id, at: was });
        }
        note(g, Some(gd), &format!("lane {}: {}: back", l.id, if went { "the grenade is away" } else { "the grenade stayed in his hand" }));
        peek_back(g, gd, d);
        return;
    }
    let at = guns_mid(l);
    if l.can.is_some() || g.time < l.push_until || !crate::frag::frag_ready(g, gd, at) {
        l.frag_at = g.time + 2.0;
        if f.going { note(g, Some(gd), &format!("lane {}: not now for his grenade — back", l.id)); }
        peek_back(g, gd, d);
        return;
    }
    if g.time < f.next { return; }
    f.next = g.time + 0.045;   // (a place every third frame: a frame pays for one place's arcs)
    let n = js::rem(f.k, big_n);
    let b = frag_place(g, gd, *to, n * 0.3, f.going);
    let r = match b { Some(FragPlace::List(x)) => Some(crate::frag::frag_throw_at(g, gd, at, x, *to)), _ => None };
    f.k += 1.0;
    if let (Some(FragThrowAt::Thrown), Some(FragPlace::List(x))) = (r, b) {
        // he has shouted: to the place it leaves his hand from and not a step further; no mate steps out onto the lane or goes round to its guns until it has gone off
        let quiet = g.time + g.k.frag.def_wind + g.k.frag.def_fuse + g.k.lanes.frag_look;
        f.at = Some(at);
        f.until = g.time + g.k.frag.def_wind;
        f.had = crate::frag::frag_away(g, gd);
        s(g).said_t = g.time;
        if f.going {
            *to = x;
            clear_path(g, gd);
            crate::pressure::run(g, d, x, None, Some(pace));
            aim_at(g, gd, [at[0], 1.2, at[2]]);
        }
        l.frag_at = g.time + g.k.lanes.frag_gap;
        l.peek_at = m::max(l.peek_at, quiet);
        l.pin_at = m::max(l.pin_at, quiet);
        l.stun_at = m::max(l.stun_at, quiet);
        l.round_at = m::max(l.round_at, quiet);
        let off = flat(pos_of(g, gd), at);
        note(g, Some(gd), &format!("lane {}: a grenade down it {}, for ({}, {}), {} m off", l.id, if f.going { "from the edge of its floor" } else { "from behind his corner" }, js::to_fixed(at[0], 1), js::to_fixed(at[2], 1), js::to_fixed(off, 0)));
        return;
    }
    if matches!(r, Some(FragThrowAt::Own) | Some(FragThrowAt::Busy)) { f.own = true; }
    if let Some(FragPlace::Edge) = b {   // (nothing further out is his to stand on: this round of places is done; with not even a step of his own left, so is the errand)
        if n <= 1.0 { f.edge = true; }
        f.k += big_n - 1.0 - n;
    }
    if js::truthy(js::rem(f.k, big_n)) { return; }
    if !f.going && f.walk && !f.own {
        f.going = true;
        f.k = 0.0;
        clear_path(g, gd);
        crate::pressure::run(g, d, *to, None, Some(pace));
        note(g, Some(gd), &format!("lane {}: no arc for his grenade from where he is held — toward its mouth at ({}, {}) for one", l.id, js::to_fixed(to[0], 1), js::to_fixed(to[2], 1)));
        return;
    }
    if f.going && !f.edge && mode_of(g, d) == crate::roomfight::DefMode::Move && g.time - t0 < 6.0 { return; }
    if f.own { l.frag_at = g.time + 6.0; } else { s(g).frag_no.set(gd, crate::lanes::StateFragNoValue { lane: l.id, at }); }   // (one of his own in its reach is no news about the arc: asked again soon)
    let what = if f.own { String::from("one of his own in his grenade's reach") } else { format!("no arc down it for his grenade{}", if f.going { " as far as the edge of its floor" } else { "" }) };
    note(g, Some(gd), &format!("lane {}: {} — {}", l.id, what, if f.going { "back" } else { "stays" }));
    peek_back(g, gd, d);
}
/// A LOOK ROUND THE CORNER: of the men this rule holds short of a quiet lane's mouth, the nearest steps to where it opens and looks down it (lanes.ts peekStart)
pub fn peek_start(g: &mut crate::game::Game, l: &mut crate::lanes::Lane) {
    l.peek_at = g.time + 3.0;   // (asked again soon while nobody is placed for it)
    if s(g).peek.values().any(|p| p.lane == l.id) { return; }
    let pick = match stepper(g, l, None, false, None) { Some(p) => p, None => return };
    let (best, to) = (pick.gd, pick.to);
    let d = match def_of(g, best) { Some(d) => d, None => return };
    let p = pos_of(g, best);
    let look = match s(g).held.get(best).map(|h| h.look) { Some(x) => x, None => mouth_of(g, l, p) };
    l.peek_at = g.time + g.k.lanes.peek_gap;
    let row = crate::lanes::Peek { lane: l.id, phase: crate::lanes::PeekPhase::Out, t: g.time, home: p, look, to, line_t: -1.0, eye: None, fire: None, frag: None };
    s(g).held.delete(best);
    s(g).peek.set(best, row);
    clear_path(g, best);
    let fast = g.k.lanes.peek_pace - g.tune.guard_chase;
    crate::pressure::run(g, d, to, None, Some(fast));
    s(g).stats.peeks += 1.0;
    note(g, Some(best), &format!("lane {}: quiet {} s — steps to its mouth toward ({}, {}) for a look", l.id, js::to_fixed(g.time - l.last_t, 0), js::to_fixed(to[0], 1), js::to_fixed(to[2], 1)));
}

/// (lanes.ts:803 back) on his way back to where he stood. The row is there at every call: only peek_step and lanesReset delete one
fn peek_back(g: &mut Game, gd: GuardId, d: RoomfightDefId) {
    let now = g.time;
    let home = { let r = s(g).peek.get_mut(gd).unwrap(); r.phase = crate::lanes::PeekPhase::Back; r.t = now; r.home };
    clear_path(g, gd);
    let fast = g.k.lanes.push_fast;
    crate::pressure::run(g, d, home, None, Some(fast));
}
/// (lanes.ts:813, 818) fragStep (`seen` None) or pinStep. The lane goes by a clone left in place, since the callee's maskOf reads this lane's row in st.lanes, and is written back whole
fn hand_on(g: &mut Game, gd: GuardId, li: usize, d: RoomfightDefId, seen: Option<Option<V3>>) {
    let mut lane = s(g).lanes[li].clone();
    match seen {
        Some(seen) => pin_step(g, gd, &mut lane, d, seen),
        None => frag_step(g, gd, &mut lane, d),
    }
    s(g).lanes[li] = lane;
}
/// every frame, a man out for a look: one of them in sight by the guns = the lane stands; a line down it and nobody there for `peekLook` = clear, forgotten, and the men held on it go on together (lanes.ts peekStep)
pub fn peek_step(g: &mut crate::game::Game, gd: GuardId) {
    use crate::lanes::{PeekPhase, StateHeldValue};
    let p = match s(g).peek.get(gd) { Some(&p) => p, None => return };   // (a read of the row as this frame began: every write goes by the path)
    let want = p.lane;
    let li = s(g).lanes.iter().position(|x| x.id == want);
    let dd = def_of(g, gd);
    if is_dead(g, gd) {
        s(g).peek.delete(gd);
        let now = g.time;
        if p.frag.is_some() {
            s(g).stats.frag_lost += 1.0;
            if let Some(i) = li { let gap = g.k.lanes.frag_gap; let r = &mut s(g).lanes[i]; r.frag_at = m::max(r.frag_at, now + gap); let id = r.id; note(g, Some(gd), &format!("lane {}: the grenade cost him", id)); }
        } else if p.fire.is_some() {
            s(g).stats.pin_lost += 1.0;
            if let Some(i) = li { let rest = g.k.lanes.pin_rest; let r = &mut s(g).lanes[i]; r.pin_n = 0.0; r.pin_at = now + rest; let id = r.id; note(g, Some(gd), &format!("lane {}: firing down it cost him", id)); }
        } else {
            s(g).stats.peek_lost += 1.0;
            if let Some(i) = li { let lost = g.k.lanes.peek_lost; let r = &mut s(g).lanes[i]; r.peek_n += 1.0; r.peek_at = now + lost; let id = r.id; note(g, Some(gd), &format!("lane {}: the look cost him", id)); }
        }
        return;
    }
    let off = match dd { Some(_) => !fit(g, gd) || pushing(g, gd), None => true };
    if off {
        s(g).peek.delete(gd);
        let now = g.time;
        if let Some(f) = p.frag {   // (blind, folded, or in behind a charge with a grenade errand on him: asked again soon; a man who never left his corner is still held on the lane)
            if let Some(i) = li {
                s(g).lanes[i].frag_at = now + if f.at.is_some() { 6.0 } else { 2.0 };
                let here = pos_of(g, gd);
                if dd.is_some() && !f.going && flat(here, p.home) < 0.5 { let lane = s(g).lanes[i].id; s(g).held.set(gd, StateHeldValue { lane, t: now, look: p.look, aimed: true, at: here }); }
            }
        } else if let Some(i) = li {   // (no longer this errand's man: the gate has his feet again, and the next look waits its gap)
            let gap = g.k.lanes.peek_gap;
            let r = &mut s(g).lanes[i];
            r.peek_at = m::max(r.peek_at, now + gap);
        }
        return;
    }
    let d = match dd { Some(d) => d, None => return };
    if li.is_none() && p.phase != PeekPhase::Back { peek_back(g, gd, d); return; }   // (the lane gone meanwhile: back to where he stood)
    if p.phase == PeekPhase::Back {
        if mode_of(g, d) == crate::roomfight::DefMode::Move && flat(pos_of(g, gd), p.home) > 0.5 && g.time - p.t < 6.0 { return; }
        s(g).peek.delete(gd);
        let here = pos_of(g, gd);
        if lane_on(g, here, gd).is_some() { return; }   // (stopped in the line: the fight's, and the tick's own way out of a line)
        let i = match li { Some(i) => i, None => return };   // (his own look cleared it: nothing left to hold him on)
        if g.time - g.gd[gd].last_seen_t > 1.0 && crate::roomfight::room_fight_lane_stop(g, gd, p.look) {
            let (at, now) = (pos_of(g, gd), g.time);
            let lane = s(g).lanes[i].id;
            s(g).held.set(gd, StateHeldValue { lane, t: now, look: p.look, aimed: true, at });
        }
        return;
    }
    let i = match li { Some(i) => i, None => return };
    if p.frag.is_some() { hand_on(g, gd, i, d, None); return; }
    // his head round the corner ahead of his feet: the look is taken from his eyes leant `peekLean` toward the mouth (nothing solid between the two)
    let mut eye = eye_of(g, gd);
    {
        let here = pos_of(g, gd);
        let (dx, dz) = (p.to[0] - here[0], p.to[2] - here[2]);
        let dl = m::hyp2(dx, dz);
        if dl > 0.05 && p.line_t < 0.0 {
            let lean = g.k.lanes.peek_lean;
            let out: V3 = [eye[0] + dx / dl * lean, eye[1], eye[2] + dz / dl * lean];
            if !solid_blocked(g, eye, out) { eye = out; }
            if let Some(r) = s(g).peek.get_mut(gd) { r.eye = Some(eye); }
        } else if let (Some(e), true) = (p.eye, p.line_t >= 0.0) { eye = e; }
    }
    let lane = s(g).lanes[i].clone();   // (what the asks below read; every write goes to the row)
    let seen = guns_manned(g, &lane, eye);
    // out to FIRE down it: his burst goes unless he can see all of where they stood and nobody is there (then it is a look: clear)
    if p.fire.map_or(false, |f| f.n > 0.0) && (seen.is_some() || !all_spots_seen(g, &lane, eye)) { hand_on(g, gd, i, d, Some(seen)); return; }
    let id = lane.id;
    let gap = g.k.lanes.peek_gap;
    let now = g.time;
    if let Some(seen) = seen {
        let (keep, most) = (g.k.lanes.peek_keep, g.k.lanes.peek_max);
        let r = &mut s(g).lanes[i];
        r.until = m::max(r.until, now + keep);
        r.peek_part = 0.0;
        r.peek_n += 1.0;
        r.peek_at = now + m::min(most, gap * m::pow(2.0, r.peek_n));
        if r.guns.len() < 4 && r.guns.iter().all(|&q| flat(q, seen) > 1.5) { r.guns.push([seen[0], 0.0, seen[2]]); }   // (where they stand now: a place to stop is out of that one's sight too)
        let next = r.peek_at - now;
        s(g).stats.peek_there += 1.0;
        if now - s(g).said_t > 2.0 { s(g).said_t = now; say_net(g, gd, "they're still on it — stay back!"); }
        note(g, Some(gd), &format!("lane {}: a look — still held, from ({}, {}); next no sooner than {} s", id, js::to_fixed(seen[0], 1), js::to_fixed(seen[2], 1), js::to_fixed(next, 0)));
        peek_back(g, gd, d);
        return;
    }
    if mode_of(g, d) != crate::roomfight::DefMode::Move && now - g.gd[gd].last_seen_t < 1.0 { s(g).peek.delete(gd); s(g).lanes[i].peek_at = now + gap; return; }   // (a man in his sights somewhere else: the fight has him)
    // 'clear' wants the whole of it: the place round EVERY spot in his view, and nobody on their net with eyes on one of them. The spots but not all the floor round them is HALF a look
    let full = all_spots_seen(g, &lane, eye);
    let done = mode_of(g, d) != crate::roomfight::DefMode::Move || now - p.t > 8.0;
    let mut part = !full && done;
    if part {
        for &q in lane.guns.iter() {
            if seg_blocked(g, eye, [q[0], 1.4, q[2]]) { part = false; break; }
        }
    }
    if (full || part) && net_sees(g, &lane, gd) {
        let r = &mut s(g).lanes[i];
        r.peek_at = now + gap;
        r.peek_part = 0.0;
        note(g, Some(gd), &format!("lane {}: a look — nobody in his view, but a mate has eyes on them there: back", id));
        peek_back(g, gd, d);
        return;
    }
    if full || part {
        let mut line_t = p.line_t;
        if line_t < 0.0 {   // (far enough: he looks from here)
            line_t = now;
            let to = pos_of(g, gd);
            if let Some(r) = s(g).peek.get_mut(gd) { r.line_t = line_t; r.to = to; }
            clear_path(g, gd);
            crate::pressure::run(g, d, to, None, Some(0.0));
        }
        if now - line_t < g.k.lanes.peek_look { return; }
        if !full {
            let sure = g.k.lanes.peek_sure;
            let r = &mut s(g).lanes[i];
            r.peek_part += 1.0;
            if r.peek_part < 2.0 || now - r.last_t < sure {
                r.peek_at = now + gap;
                let n = r.peek_part;
                note(g, Some(gd), &format!("lane {}: a look — nobody in his view, but not all of where they stood ({}): back", id, n));
                peek_back(g, gd, d);
                return;
            }
        }
        let st = s(g);
        let kept = st.peek.get(gd).copied();   // (the row as it stands, for a man who goes back alone: :846 puts it in again)
        st.peek.delete(gd);
        st.stats.peek_clear += 1.0;
        st.stats.lapsed += 1.0;
        let (downs, last_t, from) = (st.lanes[i].downs, st.lanes[i].last_t, st.lanes[i].from);
        st.lanes.remove(i);
        st.due.retain(|x| x.lane != id);
        remask(g);   // (forgotten here and now: nobody is stopped short of it on his way out)
        let mut mates: Vec<GuardId> = Vec::new();
        let mut at = 0usize;
        while let Some(o) = s(g).held.next_key(&mut at) {
            if s(g).held.get(o).map_or(false, |h| h.lane == id) {
                s(g).held.delete(o);
                if o != gd && fit(g, o) { mates.push(o); }
            }
        }
        let tail = if !mates.is_empty() { format!(", {} move up together", mates.len() + 1) } else { String::from(", back to his place") };
        note(g, Some(gd), &format!("lane {}: a look — nobody on it: clear{}", id, tail));
        note(g, None, &format!("lane {}: forgotten ({} down, last round {} s ago)", id, downs, js::to_fixed(now - last_t, 0)));
        // with a mate held on it too they go on TOGETHER to where the guns were; a man by himself goes nowhere on it: back to his place
        if !mates.is_empty() {
            say_net(g, gd, "they're off it — move up, together!");
            for &o in mates.iter() { if let Some(dm) = def_of(g, o) { crate::pressure::loose(g, dm, from); } }
            crate::pressure::loose(g, d, from);
        } else {
            say_net(g, gd, "they've moved off it — eyes open!");
            if let Some(kept) = kept { s(g).peek.set(gd, kept); }
            peek_back(g, gd, d);
        }
        s(g).said_t = now;
        return;
    }
    if done {
        s(g).lanes[i].peek_at = now + gap;
        note(g, Some(gd), &format!("lane {}: no line down it from its mouth — back", id));
        peek_back(g, gd, d);
    }
}
/// (lanes.ts:818, 830) L.guns.every(m => spotSeen(g, eye, m))
fn all_spots_seen(g: &mut Game, l: &crate::lanes::Lane, eye: V3) -> bool {
    for &q in l.guns.iter() { if !spot_seen(g, eye, q) { return false; } }
    true
}
/// where a lane opens before a man standing at `p`: the nearest of its floor he has a line to, of the ten nearest; else the nearest (lanes.ts mouthOf)
pub fn mouth_of(g: &mut crate::game::Game, l: &crate::lanes::Lane, p: V3) -> V3 {
    let (cell, w) = (g.col.nav.cell, g.col.nav.w);
    let mut near: Vec<(f64, f64, f64)> = Vec::new();   // (d, x, z)
    if let Some(cells) = l.cells.as_ref() {
        for i in cells.iter() {
            let i = i.0;
            let x = (js::rem(i, w) + 0.5) * cell;
            let z = (js::to_i32(i / w) as f64 + 0.5) * cell;
            let d = m::hyp2(x - p[0], z - p[2]);
            if near.len() >= 10 && d >= near[9].0 { continue; }
            let mut k = near.len();
            while k > 0 && near[k - 1].0 > d { k -= 1; }
            near.insert(k, (d, x, z));
            if near.len() > 10 { near.pop(); }
        }
    }
    if near.is_empty() { return l.at; }
    let eye: V3 = [p[0], 1.5, p[2]];
    for &(_, x, z) in near.iter() { if !solid_blocked(g, eye, [x, 1.0, z]) { return [x, 0.0, z]; } }
    [near[0].1, 0.0, near[0].2]
}
/// the price of a walk with this mask's floor made dear, on top of `base` (lanes.ts priceOf)
pub fn price_of(g: &mut crate::game::Game, m: Option<&[u8]>, base: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<crate::lanes::LanePenalty> {
    use crate::lanes::LanePenalty;
    let base = base.map(|b| seam_base_penalty(b));
    let mask = match m { Some(x) => x, None => return base.map(|base| LanePenalty::Base { base }) };
    let (w, dear) = (g.col.nav.w, g.k.lanes.dear);
    Some(match base {
        Some(base) => LanePenalty::DearOver { base, m: mask.to_vec(), w, dear },
        None => LanePenalty::Dear { m: mask.to_vec(), w, dear },
    })
}
/// guards.ts goTo: the price of a walk for this man: `base` with the floor of every lane his side knows made dear (lanes.ts lanePenalty)
pub fn lane_penalty(g: &mut crate::game::Game, gd: GuardId, base: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<crate::lanes::LanePenalty> {
    if g.gd[gd].script.as_ref().and_then(|x| x.direct) == Some(true) { return price_of(g, None, base); }   // (a walk a level's script sent the short way)
    let none = g.m.lanes.states.as_ref().map_or(true, |st| st.lanes.is_empty());
    if none || !live(g) || !mine(g, gd) || pushing(g, gd) { return price_of(g, None, base); }
    let mask = mask_of(g, gd);
    price_of(g, mask.as_deref(), base)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_middle_is_the_gun_nearest_the_mean() {
        let mut l = crate::lanes::Lane::zeroed();
        l.from = [9.0, 0.0, 9.0];
        assert_eq!(guns_mid(&l), [9.0, 0.0, 9.0]);
        l.guns = vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [3.0, 0.0, 0.0]];   // (mean x 7/3: nearest is 3)
        assert_eq!(guns_mid(&l), [3.0, 0.0, 0.0]);
    }
    #[test]
    fn a_cell_of_a_lane_is_dear() {
        let pen = crate::lanes::LanePenalty::Dear { m: vec![0u8, 1, 0, 0], w: 2.0, dear: 14.0 };
        assert_eq!(pen_at(&pen, 1.0, 0.0), 14.0);
        assert_eq!(pen_at(&pen, 0.0, 1.0), 1.0);
    }
    #[test]
    fn a_lane_prices_over_the_tactics() {
        let mut door = JsSet::new();
        door.add(crate::Num(0.0));
        door.add(crate::Num(1.0));
        let mut avoid = JsSet::new();
        avoid.add(crate::Num(2.0));
        let base = crate::tactics::TacticsPenalty::Cells { w: 2.0, door_idx: Some(door), door_mult: 20.0, avoid: Some(avoid) };
        let pen = crate::lanes::LanePenalty::DearOver { base: base.clone(), m: vec![0u8, 1, 0, 0], w: 2.0, dear: 14.0 };
        assert_eq!(pen_at(&pen, 0.0, 0.0), 20.0);   // (the doorway's price, the lane's floor not under it)
        assert_eq!(pen_at(&pen, 1.0, 0.0), 20.0);   // (the dearer of the two)
        assert_eq!(pen_at(&pen, 0.0, 1.0), crate::tactics::CORRIDOR_COST);
        assert_eq!(pen_at(&pen, 1.0, 1.0), 1.0);
        assert_eq!(pen_at(&crate::lanes::LanePenalty::Base { base }, 1.0, 0.0), 20.0);
    }
    #[test]
    fn a_key_of_minus_zero_is_zero() { assert!(cell_key(-0.0) == crate::Num(0.0)); }
}
