//! src/game/lanes.ts lines 367 to 807
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// (lanes.ts:220) the skeleton's SIDE is the word as a &str; a man's team is the enum (notes.md, Skeleton)
const SIDE: crate::teams::Team = crate::teams::Team::Hostile;

// ---------------------------------------------------------------- what the TypeScript's language and closures gave
/// `g.col.nav.findPath(a, b, maxIter, pen)` at a price this file holds as a value
fn find_path(g: &mut Game, a: V3, b: V3, max_iter: f64, pen: Option<&crate::lanes::LanePenalty>) -> Option<Vec<V3>> {
    match pen {
        Some(p) => { let mut price = |_: &mut Game, ix: f64, iz: f64| lane_penalty_at(p, ix, iz); crate::scene_collision::nav_grid_find_path(g, a, b, Some(max_iter), Some(&mut price)) }
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
fn clear_path(g: &mut Game, gd: GuardId) { crate::game::guard_set_path(g, gd, Vec::new()); crate::guards::guard_set_path_goal(g, gd, None); }
/// `g.say(gd, text, true)`
fn say_net(g: &mut Game, gd: GuardId, text: &str) { crate::game::game_say(g, gd, text, Some(true), None); }
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn char_id_of(g: &Game, gd: GuardId) -> f64 { g.ch[g.gd[gd].ch].id }
fn goal_of(g: &Game, gd: GuardId) -> Option<V3> { g.gd[gd].script.as_ref().and_then(|s| s.goal) }
fn mode_of(g: &Game, d: RoomfightDefId) -> crate::roomfight::DefMode { g.tab.roomfight_def[d].mode }
fn is_dead(g: &Game, gd: GuardId) -> bool { g.gd[gd].state == crate::game::GuardState::Dead }
fn aim_at(g: &mut Game, gd: GuardId, at: V3) { crate::squad::order(g, gd, crate::squad::OrderPatch { aim_at: Some(Some(at)), ..Default::default() }); }
/// `st.lanes.find(x => x.id === id)`
fn lane_by_id(g: &mut Game, id: f64) -> Option<LanesLaneId> {
    let mut i = 0;
    while i < s(g).lanes.len() {
        let l = s(g).lanes[i];
        i += 1;
        if g.tab.lanes_lane[l].id == id { return Some(l); }
    }
    None
}

/// once a frame, after the room fight's own tick and before anybody walks (lanes.ts lanesTick)
pub fn lanes_tick(g: &mut crate::game::Game, dt: f64) {
    use crate::roomfight::DefMode;
    if !live(g) {
        let busy = match &g.m.lanes.states {
            Some(st) => !st.lanes.is_empty() || !st.held.is_empty() || !st.peek.is_empty() || !st.near.is_empty(),
            None => false,
        };
        if busy { lanes_reset(g); }
        return;
    }
    let k = g.k.lanes;
    if g.time < s(g).t { lanes_reset(g); }   // (the clock has gone back: a load; nothing kept across it but what a restore puts back)
    s(g).t = g.time;
    if s(g).skip {   // (the events a load put back were read before the file was made)
        s(g).skip = false;
        for i in 0..g.events.len() {
            let key = g.events[i].serial;
            let st = s(g);
            st.seen.add(key);
            st.banged.add(key);
        }
    }
    // the men the gate stopped last frame: planted where they stand, gun on the lane's mouth (between frames of walking, never inside a man's own step)
    if !s(g).due.is_empty() {
        let due = core::mem::take(&mut s(g).due);
        for d in due { plant_short(g, d.gd, d.lane, d.look); }
    }
    // (their own charges in the air: a side knows whose can it was that burst)
    let mut ii = 0;
    while ii < g.items.items.len() {
        let it = g.items.items[ii];
        ii += 1;
        if crate::throwables::item_kind(g, it) == crate::throwables::ItemKind::Flash && !crate::throwables::item_fired(g, it) && !s(g).cans.contains(&it) {
            let who = crate::throwables::item_who(g, it);
            if crate::teams::team_of_id(g, who) == SIDE { s(g).cans.push(it); }
        }
    }
    read_rounds(g);
    bangs(g);
    if !s(g).near.is_empty() {   // rounds that passed a man who had not noticed them: his once his senses have them, nobody's after a beat
        let near = core::mem::take(&mut s(g).near);
        let mut kept = Vec::new();
        for n in near {
            if !mine(g, n.gd) { continue; }   // (dropped since: the round that dropped him is the one that counts)
            if roused(g, n.gd, n.t) { cost(g, n.from, n.at, n.gd, false, n.who); continue; }
            if g.time - n.t < k.react_secs { kept.push(n); }
        }
        s(g).near = kept;
    }
    if !s(g).cans.is_empty() {   // (a spent can lies on the floor a minute: it is nobody's bang any more)
        let cans = core::mem::take(&mut s(g).cans);
        let mut kept = Vec::new();
        for it in cans {
            if !crate::throwables::item_fired(g, it) && g.items.items.contains(&it) { kept.push(it); }
        }
        s(g).cans = kept;
    }
    let mut i = 0;
    while i < s(g).lanes.len() {
        let l = s(g).lanes[i];
        i += 1;
        if !g.tab.lanes_lane[l].known {
            let mut at = 0;
            while let Some(o) = g.tab.lanes_lane[l].saw.next_key(&mut at) {
                let t = *g.tab.lanes_lane[l].saw.get(o).unwrap();
                if !mine(g, o) { g.tab.lanes_lane[l].saw.delete(o); continue; }   // (cut down before he could act on it or say it)
                if !fit(g, o) { continue; }
                if !g.tab.lanes_lane[l].knowers.has(o) && g.time >= t + k.react_secs {
                    g.tab.lanes_lane[l].knowers.add(o);
                    s(g).ver += 1.0;
                }
                if g.time >= t + k.call_secs { become_known(g, l, o); break; }
            }
        }
        // a lane that cost a man nobody lived to tell of: a fit man who comes on the place (the bodies) knows what it means
        if !g.tab.lanes_lane[l].known && g.tab.lanes_lane[l].weight >= 1.0 {
            let place = g.tab.lanes_lane[l].at;
            let mut gi = 0;
            while gi < g.guards.len() {
                let o = g.guards[gi];
                gi += 1;
                if fit(g, o) && !g.tab.lanes_lane[l].saw.has(o) && sees(g, o, place, k.body_m) {
                    let now = g.time;
                    g.tab.lanes_lane[l].saw.set(o, now);
                }
            }
        }
        let now = g.time;
        let x = &mut g.tab.lanes_lane[l];
        if x.push_until >= 0.0 && now >= x.push_until { x.push_until = -1.0; }
        if x.can.map_or(false, |c| now - c.t > 5.0) { x.can = None; }
    }
    let old = s(g).lanes.clone();
    let n0 = old.len();
    let mut kept = Vec::with_capacity(n0);
    for l in old {   // (a few rounds past a man and no more: forgotten in ten seconds)
        let (weight, until, last_t, known, id, downs) = { let x = &g.tab.lanes_lane[l]; (x.weight, x.until, x.last_t, x.known, x.id, x.downs) };
        let keep = if weight >= 1.0 { g.time < until } else { g.time - last_t < 10.0 };
        if !keep && !known && weight >= 1.0 { s(g).stats.unknown += 1.0; }
        if !keep && known { let ago = js::to_fixed(g.time - last_t, 0); note(g, None, &format!("lane {}: forgotten ({} down, last round {} s ago)", id, downs, ago)); }
        if keep { kept.push(l); }
    }
    let n1 = kept.len();
    s(g).lanes = kept;
    if n1 != n0 {
        lanes_gone(g);
        s(g).stats.lapsed += (n0 - n1) as f64;
        remask(g);
    }
    if !s(g).peek.is_empty() {
        for gd in s(g).peek.keys() { peek_step(g, gd); }   // (a step only ever ends its own man's row)
    }
    s(g).tick_t -= dt;
    if s(g).tick_t > 0.0 { return; }
    s(g).tick_t = 0.5;
    let mut hi = 0;
    while let Some(gd) = s(g).held.next_key(&mut hi) {
        let h = *s(g).held.get(gd).unwrap();
        let def = def_of(g, gd);
        let keep = mine(g, gd) && lane_by_id(g, h.lane).is_some();
        let d = match def {
            Some(d) if keep && !crate::roomfight::is_loose(g, d) => d,
            _ => { held_off(g, gd); continue; }
        };
        let mode = mode_of(g, d);
        let pos = pos_of(g, gd);
        if h.aimed {
            if mode != DefMode::Hold || flat(pos, h.at) > 2.5 { held_off(g, gd); continue; }   // (somebody else has moved him since: no longer this rule's man)
            // …moved a step all the same onto floor a gun of the lane has, or left in a doorway: out of it again. Never a man set on the lane's other side: his place a step short of their line is the errand's own
            if fit(g, gd) && !close_fight(g, gd) && !pushing(g, gd) && !flanking(g, gd) && !s(g).flank_of.has(gd) && !s(g).peek.has(gd) && g.time - g.gd[gd].last_seen_t > 1.0 && g.time - s(g).back_t.get(gd).copied().unwrap_or(-100.0) > 6.0 {
                if let Some(l) = lane_by_id(g, h.lane) {
                    if knows(g, l, gd) && (crate::doorway::doorway_in_way(g, pos) || gun_on(g, l, pos, Some(k.shift_m))) && !back_out(g, gd, l) {
                        let now = g.time;
                        s(g).back_t.set(gd, now);   // (no cover found: asked again in six seconds, not every tick)
                    }
                }
            }
            continue;
        }
        if mode == DefMode::Move {   // (on his way to cover, or sent elsewhere meanwhile: then the gate has him again like anybody)
            let on_way = match goal_of(g, gd) { Some(sg) => !(flat(sg, h.at) > 0.8), None => false };
            if !on_way { held_off(g, gd); }
            continue;
        }
        if let Some(row) = s(g).held.get_mut(gd) { row.aimed = true; row.at = v3::copy(pos); }
        // (backed out to cover and arrived: his gun on the lane's mouth, unless he has a man in his sights, which is the fight's)
        if fit(g, gd) && g.time - g.gd[gd].last_seen_t > 1.0 {
            crate::roomfight::room_fight_lane_stop(g, gd, h.look);
            s(g).stats.held += 1.0;
        }
    }
    // a lane's floor is kept true to the doors and walls as they stand: one wedge read again each half second, its floor swapped when the last is in
    let ls: Vec<LanesLaneId> = {
        let ids = s(g).lanes.clone();
        ids.into_iter().filter(|&l| { let x = &g.tab.lanes_lane[l]; x.cells.is_some() && x.wedges.len() >= 2 }).collect()
    };
    if !ls.is_empty() {
        let st = s(g);
        let l = ls[js::rem(st.roll, ls.len() as f64) as usize];
        st.roll += 1.0;
        let mut r = g.tab.lanes_lane[l].re.take().unwrap_or_else(|| crate::lanes::LaneRe { set: JsSet::new(), i: 0.0 });
        let wedges = g.tab.lanes_lane[l].wedges.len() as f64;
        if r.i + 1.0 < wedges {
            let (from, at) = (g.tab.lanes_lane[l].wedges[r.i as usize], g.tab.lanes_lane[l].wedges[r.i as usize + 1]);
            let e = lane_e(g, l);
            wedge(g, &mut r.set, from, at, e);
            r.i += 2.0;
        }
        if r.i + 1.0 >= wedges {
            g.tab.lanes_lane[l].cells = Some(r.set);
            if g.tab.lanes_lane[l].known { remask(g); } else { s(g).ver += 1.0; }
        } else {
            g.tab.lanes_lane[l].re = Some(r);
        }
    }
    // a gun that made a lane and is down has moved for good
    let mut i = 0;
    while i < s(g).lanes.len() {
        let l = s(g).lanes[i];
        i += 1;
        let mut cut = false;
        for id in g.tab.lanes_lane[l].who.keys() {
            if !up_id(g, id.0) {
                let x = &mut g.tab.lanes_lane[l];
                x.who.delete(id);
                x.moved.delete(id);
                cut = true;
            }
        }
        let now = g.time;
        let x = &mut g.tab.lanes_lane[l];
        if cut && x.moved.len() >= x.who.len() { x.until = m::min(x.until, now + k.moved_secs); }
    }
    for i in 0..g.fight.defs.len() {
        let d = g.fight.defs[i];
        let (gd, station) = (g.tab.roomfight_def[d].gd, g.tab.roomfight_def[d].station);
        let st = s(g);
        if !st.post.has(gd) { st.post.set(gd, v3::copy(station)); }
    }
    if !s(g).flank_of.is_empty() {   // (the lane forgotten under them)
        let list: Vec<(GuardId, f64)> = s(g).flank_of.iter().map(|(gd, f)| (gd, f.lane)).collect();
        for (gd, lane) in list {
            let stands = match lane_by_id(g, lane) { Some(l) => g.tab.lanes_lane[l].known, None => false };
            if !stands { s(g).flank_of.delete(gd); }
        }
    }
    let mut i = 0;
    while i < s(g).lanes.len() {
        let l = s(g).lanes[i];
        i += 1;
        if !g.tab.lanes_lane[l].known || g.time < g.tab.lanes_lane[l].push_until { continue; }
        let at = held_at(g, l);
        if g.time >= g.tab.lanes_lane[l].stun_at && !at.is_empty() { stun(g, l, &at); }
        if g.time >= g.tab.lanes_lane[l].round_at && g.tab.lanes_lane[l].can.is_none() {
            let st = s(g);
            let free: Vec<GuardId> = if !st.flank_of.is_empty() { at.iter().copied().filter(|&x| !st.flank_of.has(x)).collect() } else { at.clone() };
            if free.len() >= 2 { round(g, l, &free); }
        }
        if g.tab.lanes_lane[l].frag_at > 0.0 && g.time >= g.tab.lanes_lane[l].frag_at && g.tab.lanes_lane[l].can.is_none() { frag_out(g, l, &at); }
        if !s(g).flank_of.is_empty() { flank_tick(g, l); }
        let (peek_at, none, last_t, known_at) = { let x = &g.tab.lanes_lane[l]; (x.peek_at, x.can.is_none(), x.last_t, x.known_at) };
        if g.time >= peek_at && none && g.time - last_t > k.peek_quiet && g.time - known_at > k.peek_quiet { peek_start(g, l); }
    }
    // a man standing where a lane's guns have him, rounds still coming down it or the guns in his own sight, and not on his own post: out of the line
    let mut di = 0;
    while di < g.fight.defs.len() {
        let d = g.fight.defs[di];
        di += 1;
        let gd = g.tab.roomfight_def[d].gd;
        if crate::roomfight::on_leg(g, d) || !fit(g, gd) || close_fight(g, gd) || pushing(g, gd) || flanking(g, gd) || s(g).peek.has(gd) || g.time - s(g).back_t.get(gd).copied().unwrap_or(-100.0) < 6.0 { continue; }
        let pos = pos_of(g, gd);
        if crate::roomfight::at_post(g, d) && flat(pos, s(g).post.get(gd).copied().unwrap_or(pos)) < 3.0 { continue; }   // (his own post: the room fight's to defend, not this rule's to move)
        let l = match lane_on(g, pos, gd) { Some(l) => l, None => continue };
        if g.time - g.tab.lanes_lane[l].last_t > k.hot_secs && g.time - g.gd[gd].last_seen_t > 1.0 { continue; }
        back_out(g, gd, l);
    }
}

/// the men held at a lane's mouths, on their feet and with nobody on them (lanes.ts heldAt)
fn held_at(g: &mut crate::game::Game, l: LanesLaneId) -> Vec<GuardId> {
    let id = g.tab.lanes_lane[l].id;
    let rows: Vec<(GuardId, crate::lanes::StateHeldValue)> = s(g).held.iter().map(|(gd, h)| (gd, *h)).collect();
    let mut out = Vec::new();
    for (gd, h) in rows {
        if h.lane == id && fit(g, gd) && g.time - g.gd[gd].last_seen_t > 1.5 && g.time - h.t > 1.0 { out.push(gd); }
    }
    out
}

/// what backOut's two closures share
struct BackOut { l: LanesLaneId, m: Option<Vec<u8>>, memo: JsMap<usize, bool>, w: usize }
/// floor out of every lane he knows, out of this one's guns' sight, not in a doorway (lanes.ts:462 cover)
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
/// every open floor about it is cover too (lanes.ts:464 deep)
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
pub(super) fn back_out(g: &mut crate::game::Game, gd: GuardId, l: LanesLaneId) -> bool {
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
    let id = g.tab.lanes_lane[l].id;
    held_on(g, gd, crate::lanes::StateHeldValue { lane: id, t: now, look, aimed: false, at: spot });
    s(g).stats.backed_out += 1.0;
    clear_path(g, gd);
    crate::roomfight::run(g, d, spot, None, 0.3, crate::owner::by::LANE);
    if now - s(g).said_t > 4.0 { s(g).said_t = now; say_net(g, gd, "get back — out of their line!"); }
    note(g, Some(gd), &format!("lane {}: in their line, backs out to ({}, {})", id, js::to_fixed(spot[0], 1), js::to_fixed(spot[2], 1)));
    true
}

/// a way round for one man (lanes.ts:491)
struct Way { gd: GuardId, cid: f64, len: f64, pin: bool, stop: Option<V3> }

/// GO ROUND: of the men held at a lane's mouths, the one (two of four) with the shortest way to the guns that never shows him to them (lanes.ts round)
fn round(g: &mut crate::game::Game, l: LanesLaneId, at: &[GuardId]) {
    g.tab.lanes_lane[l].round_at = g.time + 10.0;   // (asked again while there is no way: a real go waits `roundGap`)
    let mask = s(g).mask.clone();
    let pen = price_of(g, mask.as_deref(), None);
    if pen.is_none() { return; }
    let (id, l_from, l_at) = { let x = &g.tab.lanes_lane[l]; (x.id, x.from, x.at) };
    let mut ways: Vec<Way> = Vec::new();
    let mut why = String::new();
    for &gd in at {
        if s(g).flank_of.has(gd) { continue; }   // (already round, set on the lane's other side: he holds it from there)
        let start = pos_of(g, gd);
        let sign = g.gd[gd].callsign.clone();
        let p = match find_path(g, start, l_from, 8000.0, pen.as_ref()) {
            Some(p) if !p.is_empty() => p,
            _ => { why.push_str(&format!(" {}: no way;", sign)); continue; }
        };
        if flat(p[p.len() - 1], l_from) > 2.5 { why.push_str(&format!(" {}: no way to the guns;", sign)); continue; }
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
            let sd = flat(show, l_from);
            let ll = js::or(flat(l_from, l_at), 1.0);
            let (lx, lz) = ((l_at[0] - l_from[0]) / ll, (l_at[2] - l_from[2]) / ll);
            let cs = ((show[0] - l_from[0]) * lx + (show[2] - l_from[2]) * lz) / js::or(sd, 1.0);
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
    if ways.is_empty() { note(g, None, &format!("lane {}: no way round —{}", id, why)); return; }
    js::sort_by(&mut ways, |a, b| js::or(a.len - b.len, a.cid - b.cid));
    let mut n = m::min_n(&[g.k.lanes.round_men, ways.len() as f64, m::max(1.0, at.len() as f64 - 1.0), if at.len() >= 4 { 2.0 } else { 1.0 }]);
    // a way that ends at a second angle is taken only if one of the men who stay can fire from the first; else the ways that hide their man to the end
    if ways[..n as usize].iter().any(|w| w.pin) {
        let mut stay: JsSet<GuardId> = JsSet::new();
        for w in ways[..n as usize].iter() { stay.add(w.gd); }
        let min_m = g.k.lanes.pin_min_m;
        if stepper(g, l, Some(&stay), true, Some(min_m)).is_none() {
            ways.retain(|w| !w.pin);
            if ways.is_empty() { note(g, None, &format!("lane {}: a way to its other side, but nobody placed to fire from this one —{}", id, why)); return; }
            n = m::min(n, ways.len() as f64);
        }
    }
    let mut i = 0usize;
    while (i as f64) < n {
        let (gd, len, way_pin, stop) = (ways[i].gd, ways[i].len, ways[i].pin, ways[i].stop);
        let first = i == 0;
        i += 1;
        let d = match def_of(g, gd) { Some(d) => d, None => continue };
        held_off(g, gd);
        let now = g.time;
        s(g).flank.set(gd, now + m::min(60.0, len / 2.0 + 10.0));
        let l_from = g.tab.lanes_lane[l].from;
        if let (true, Some(stop)) = (way_pin, stop) {   // its end too long in their sight to walk: as far as where the guns would first see him, to hold the lane from that side
            s(g).flank_of.set(gd, crate::lanes::Flank { lane: id, phase: crate::lanes::FlankPhase::Walk, at: stop, t: now, n: 0.0 });
            clear_path(g, gd);
            crate::roomfight::run(g, d, stop, None, 0.0, crate::owner::by::LANE);
            crate::belief::word(g, gd, l_from);
            s(g).stats.round += 1.0;
            if first { round_shout(g, gd); }
            note(g, Some(gd), &format!("lane {}: goes round, {} m, as far as ({}, {}) on its other side", id, js::to_fixed(len, 0), js::to_fixed(stop[0], 1), js::to_fixed(stop[2], 1)));
            continue;
        }
        crate::pressure::loose(g, d, l_from);   // (his way stays priced, but nothing here stops him short of the guns he was sent at)
        crate::belief::word(g, gd, l_from);
        s(g).stats.round += 1.0;
        if first { round_shout(g, gd); }
        note(g, Some(gd), &format!("lane {}: goes round, {} m", id, js::to_fixed(len, 0)));
    }
    g.tab.lanes_lane[l].round_at = g.time + g.k.lanes.round_gap;
}
/// (lanes.ts:520, 525) the first man sent round says so
fn round_shout(g: &mut Game, gd: GuardId) {
    let text = crate::words::wg(g, &"going around — keep them looking your way!", &"going round — keep them looking your way!");
    say_net(g, gd, text);
    s(g).said_t = g.time;
}

/// THE CHARGE: a man at a mouth with a stun charge on him and the guns in a throw's reach puts it down the lane (lanes.ts stun)
fn stun(g: &mut crate::game::Game, l: LanesLaneId, at: &[GuardId]) {
    g.tab.lanes_lane[l].stun_at = g.time + 2.0;
    let (id, l_from) = (g.tab.lanes_lane[l].id, g.tab.lanes_lane[l].from);
    for &o in g.guards.iter() {
        if g.gd[o].team == SIDE && g.gd[o].state != crate::game::GuardState::Dead && flat(g.ch[g.gd[o].ch].pos, l_from) < 6.0 { return; }   // (one of theirs is on the guns already)
    }
    let mut best: Option<GuardId> = None;
    let mut bd = g.k.lanes.stun_m;
    for &gd in at {
        if crate::tactics::stuns_of(g, gd) <= 0.0 { continue; }
        let d = flat(pos_of(g, gd), l_from);
        if d < bd && d > 4.0 { best = Some(gd); bd = d; }
    }
    let best = match best { Some(b) => b, None => return };
    let c = g.gd[best].ch;
    let cp = g.ch[c].pos;
    let (tx, tz) = (l_from[0] - cp[0], l_from[2] - cp[2]);
    let tl = m::hyp2(tx, tz);
    // his arm round the corner he holds: the can leaves his hand half a pace toward the lane's line, and is asked to land short of the guns, in their eyes
    let mouth = mouth_of(g, l, cp);
    let from = crate::frag::arm_round(g, cp, mouth);
    let land: V3 = [l_from[0] - tx / tl * 1.6, 0.0, l_from[2] - tz / tl * 1.6];
    let sol = crate::throwables::throwables_solve(g, from, land, Some(11.0));
    if flat(sol.land, land) > 3.0 || flat(sol.land, l_from) > 4.5 { return; }   // (no arc from here: a wall, a lintel, too far)
    let left = crate::tactics::stuns_of(g, best) - 1.0;
    crate::tactics::give_stuns(g, best, left);
    crate::character::anim_throw_item(g, c, Some(flat(cp, sol.land)), Some(m::hyp2(sol.v0[0], sol.v0[2])));
    let cid = g.ch[c].id;
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, from, sol.v0, Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(0.7), who: Some(cid), heading: Some(v3::yaw_of(sol.v0)), ..Default::default() }));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::audio_iface::game_audio_like_play(g, "throw", Some(from), Some(0.7), None);
    let now = g.time;
    let gap = g.k.lanes.stun_gap;
    let x = &mut g.tab.lanes_lane[l];
    x.can = Some(crate::lanes::LaneCan { t: now, land: sol.land });
    x.stun_at = now + gap;
    s(g).stats.stuns += 1.0;
    say_net(g, best, "bang out — on me!");
    s(g).said_t = g.time;
    note(g, Some(best), &format!("lane {}: a charge down it toward ({}, {})", id, js::to_fixed(land[0], 1), js::to_fixed(land[2], 1)));
}

/// a stun charge of THEIRS has gone off on a lane's guns with nothing solid between: the lane is theirs for `pushSecs`, and the men on its mouths go (lanes.ts bangs)
fn bangs(g: &mut crate::game::Game) {
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
            let l = s(g).lanes[li];
            li += 1;
            let now = g.time;
            let (known, push_until, id, from) = { let x = &g.tab.lanes_lane[l]; (x.known, x.push_until, x.id, x.from) };
            if !known || now < push_until { continue; }
            let mut near = false;
            if ours {
                let guns = g.tab.lanes_lane[l].guns.clone();
                for q in guns {
                    if flat(epos, q) < b_m && !solid_blocked(g, b, [q[0], 1.5, q[2]]) { near = true; break; }
                }
            }
            if !near {
                if let Some(can) = g.tab.lanes_lane[l].can {
                    if now - can.t < 4.0 && flat(epos, can.land) < 3.0 {
                        note(g, None, &format!("lane {}: the charge fell short, nobody goes", id));
                        g.tab.lanes_lane[l].can = None;
                    }
                }
                continue;
            }
            let until = now + g.k.lanes.push_secs;
            let x = &mut g.tab.lanes_lane[l];
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
                held_off(g, gd);
                s(g).push.set(gd, until);
                clear_path(g, gd);
                crate::belief::word(g, gd, from);
                let fast = g.k.lanes.push_fast;
                crate::roomfight::run(g, d, to, None, fast, crate::owner::by::LANE);   // (a straight run at where the charge went off: the room fight plants him on the first man he sees)
                if first { first = false; say_net(g, gd, "they're blind — go, go!"); s(g).said_t = g.time; }
                note(g, Some(gd), &format!("lane {}: pushes down it behind the charge", id));
            }
        }
    }
}

/// (lanes.ts:579) where this body stands, if it is by the lane's guns and `eye` has a line to it
fn manned_at(g: &mut Game, l: LanesLaneId, eye: V3, c: CharId) -> Option<V3> {
    let (pos, near) = (g.ch[c].pos, g.k.lanes.peek_near_m);
    if !g.tab.lanes_lane[l].guns.iter().any(|&q| flat(q, pos) < near) { return None; }
    let head = g.ch[c].bones.get(crate::character::Bone::Head).copied().unwrap_or_else(|| v3::add(pos, [0.0, 1.65, 0.0]));
    if !seg_blocked(g, eye, head) { return Some(pos); }
    let chest = g.ch[c].bones.get(crate::character::Bone::Chest).copied().unwrap_or_else(|| v3::add(pos, [0.0, 1.1, 0.0]));
    if !seg_blocked(g, eye, chest) { Some(pos) } else { None }
}
/// one of the other side, on his feet, that `eye` has a line to within `peekNearM` of where a lane's fire came from: a LINE, so the dark never makes a lane clear (lanes.ts gunsManned)
pub(super) fn guns_manned(g: &mut crate::game::Game, l: LanesLaneId, eye: V3) -> Option<V3> {
    for pl in crate::seats::men(g) {
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
pub(super) fn spot_seen(g: &mut crate::game::Game, eye: V3, m: V3) -> bool {
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
pub(super) fn net_sees(g: &mut crate::game::Game, l: LanesLaneId, but: GuardId) -> bool {
    let near = g.k.lanes.peek_near_m;
    let mut i = 0;
    while i < g.fight.defs.len() {
        let o = g.tab.roomfight_def[g.fight.defs[i]].gd;
        i += 1;
        if o == but || !fit(g, o) || g.time - g.gd[o].last_seen_t > 3.0 { continue; }
        let k = match g.gd[o].last_known { Some(k) => k, None => continue };
        if g.tab.lanes_lane[l].guns.iter().any(|&q| flat(q, k) < near) { return true; }
    }
    false
}

/// of the men this rule holds short of a lane's mouth, the nearest who can step to where it opens, and where he steps to (lanes.ts stepper)
fn stepper(g: &mut crate::game::Game, l: LanesLaneId, skip: Option<&JsSet<GuardId>>, armed: bool, min_gun: Option<f64>) -> Option<crate::lanes::Stepper> {
    let min_gun = min_gun.unwrap_or(0.0);
    let id = g.tab.lanes_lane[l].id;
    let mut best: Option<GuardId> = None;
    let mut bd = f64::INFINITY;
    let mut to: Option<V3> = None;
    let mut di = 0;
    while di < g.fight.defs.len() {
        let dd = g.fight.defs[di];
        di += 1;
        let gd = g.tab.roomfight_def[dd].gd;
        let h = s(g).held.get(gd).copied();
        if skip.map_or(false, |x| x.has(gd)) || !crate::roomfight::at_post(g, dd) || !fit(g, gd) || !knows(g, l, gd) || g.time - g.gd[gd].last_seen_t < 1.5 || pushing(g, gd) || flanking(g, gd) { continue; }
        let h = match h { Some(h) => h, None => continue };
        if h.lane != id || !h.aimed || g.time - h.t < 1.0 { continue; }   // (only a man this rule itself stopped short of THIS lane)
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
                if hit.is_some() && i >= 0.0 && g.tab.lanes_lane[l].cells.as_ref().map_or(false, |c| c.has(cell_key(i))) && run >= 0.3 {
                    mouth = cell_mid(g.col.nav.cell, g.col.nav.w, i);
                    d = m::max(0.3, flat(p, mouth));
                    len = run;
                    crosses = false;
                }   // (another lane's floor first: not his to cross)
            }
            if len > g.k.lanes.peek_reach_m * 1.4 || crosses { continue; }   // (…and never one that crosses a covered lane's floor on the way to this one's mouth)
        }
        if min_gun > 0.0 && g.tab.lanes_lane[l].guns.iter().any(|&q| flat(q, mouth) < min_gun) { continue; }   // (a corner that near the guns is no place to stand and fire from)
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
fn flank_tick(g: &mut crate::game::Game, l: LanesLaneId) {
    use crate::lanes::FlankPhase;
    let (id, l_from) = (g.tab.lanes_lane[l].id, g.tab.lanes_lane[l].from);
    let mut set = 0;
    for gd in s(g).flank_of.keys() {
        let mut f = match s(g).flank_of.get(gd) { Some(&f) => f, None => continue };
        if f.lane != id { continue; }
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
                if crate::roomfight::on_leg(g, d) {   // (still walking it, or sent elsewhere meanwhile)
                    if crate::roomfight::on_leg_of(g, d, crate::owner::by::LANE) && goal_of(g, gd).map_or(false, |to| flat(to, f.at) < 0.8) { continue; }
                    s(g).flank_of.delete(gd); s(g).flank.delete(gd);
                    continue;
                }
                if g.time - f.t < 2.0 { continue; }
                f.n += 1.0;
                if f.n > 4.0 { s(g).flank_of.delete(gd); s(g).flank.delete(gd); continue; }   // (walked on, a few times, then let go)
                f.t = g.time;
                s(g).flank_of.set(gd, f);
                clear_path(g, gd);
                crate::roomfight::run(g, d, f.at, None, 0.0, crate::owner::by::LANE);
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
                held_on(g, gd, crate::lanes::StateHeldValue { lane: id, t: now, look, aimed: true, at });
            }
            if g.time - s(g).said_t > 2.0 { s(g).said_t = g.time; say_net(g, gd, "set — I've got an angle on them!"); }
            let at = pos_of(g, gd);
            note(g, Some(gd), &format!("lane {}: round and set at ({}, {}), {} m from its guns: holds it from this side", id, js::to_fixed(at[0], 1), js::to_fixed(at[2], 1), js::to_fixed(flat(at, l_from), 0)));
            let x = g.tab.lanes_lane[l].pin_at;
            g.tab.lanes_lane[l].pin_at = m::max(x, g.time + 1.5);
        }
        if f.phase == FlankPhase::Set {
            let held_here = s(g).held.get(gd).map_or(false, |h| h.lane == id) && crate::roomfight::at_post(g, d);
            if !held_here && s(g).peek.get(gd).map(|p| p.lane) != Some(id) { s(g).flank_of.delete(gd); continue; }   // (taken by the fight or another errand)
            set += 1;
        }
    }
    if g.tab.lanes_lane[l].can.is_some() || g.time < g.tab.lanes_lane[l].pin_at || set == 0 { return; }
    if g.tab.lanes_lane[l].pin_n >= g.k.lanes.pin_max {   // (the volley fired: quiet a while)
        g.tab.lanes_lane[l].pin_n = 0.0;
        g.tab.lanes_lane[l].pin_at = g.time + g.k.lanes.pin_rest;
        return;
    }
    pin_start(g, l);
}


/// FIRE FROM AN ANGLE: of the men held on the lane with a burst in his gun and a corner no nearer the guns than `pinMinM`, in turn, one steps to it (lanes.ts pinStart)
fn pin_start(g: &mut crate::game::Game, l: LanesLaneId) {
    g.tab.lanes_lane[l].pin_at = g.time + 1.0;   // (asked again soon while nobody is placed for it)
    let id = g.tab.lanes_lane[l].id;
    if s(g).peek.values().any(|p| p.lane == id) { return; }
    let min_m = g.k.lanes.pin_min_m;
    let mut pick = None;
    let pin_by = g.tab.lanes_lane[l].pin_by;
    if pin_by >= 0.0 {   // (in turn: not the man who fired last, if another is placed)
        let mut last: JsSet<GuardId> = JsSet::new();
        for i in 0..g.fight.defs.len() { let o = g.tab.roomfight_def[g.fight.defs[i]].gd; if char_id_of(g, o) == pin_by { last.add(o); } }
        pick = stepper(g, l, Some(&last), true, Some(min_m));
    }
    if pick.is_none() { pick = stepper(g, l, None, true, Some(min_m)); }
    let pick = match pick { Some(p) => p, None => return };
    let gd = pick.gd;
    let d = match def_of(g, gd) { Some(d) => d, None => return };
    let p = pos_of(g, gd);
    let look = match s(g).held.get(gd).map(|h| h.look) { Some(x) => x, None => mouth_of(g, l, p) };
    let now = g.time;
    let (pin_gap, peek_gap) = (g.k.lanes.pin_gap, g.k.lanes.peek_gap);
    let x = &mut g.tab.lanes_lane[l];
    x.pin_at = now + pin_gap;
    x.pin_n += 1.0;
    x.peek_at = m::max(x.peek_at, now + peek_gap);   // (he looks as he fires: no look for news on top of it)
    let row = crate::lanes::Peek { lane: id, phase: crate::lanes::PeekPhase::Out, t: now, home: p, look, to: pick.to, line_t: -1.0, eye: None,
        fire: Some(crate::lanes::PeekFire { n: g.k.lanes.pin_rounds, t: -1.0, next: 0.0, saw: false, line_t: -1.0 }), frag: None };
    held_off(g, gd);
    peek_on(g, gd, row);
    clear_path(g, gd);
    let fast = g.k.lanes.pin_pace - g.tune.guard_chase;
    crate::roomfight::run(g, d, pick.to, None, fast, crate::owner::by::LANE);
    {   // (his gun on where the fire came from before he is round the corner)
        let (guns, from) = (g.tab.lanes_lane[l].guns.clone(), g.tab.lanes_lane[l].from);
        let mut q = if guns.is_empty() { from } else { guns[0] };
        let mut md = f64::INFINITY;
        for &x in guns.iter() { let dd = flat(x, pick.to); if dd < md { md = dd; q = x; } }
        aim_at(g, gd, [q[0], 1.2, q[2]]);
    }
    s(g).stats.pins += 1.0;
    g.tab.lanes_lane[l].pin_by = char_id_of(g, gd);
    note(g, Some(gd), &format!("lane {}: steps to his corner toward ({}, {}) to put fire down it", id, js::to_fixed(pick.to[0], 1), js::to_fixed(pick.to[2], 1)));
}

/// (lanes.ts:701 done) the burst is over: said, a magazine started if it is low, and back
fn pin_done(g: &mut Game, gd: GuardId, l: LanesLaneId, d: RoomfightDefId, fr: crate::lanes::PeekFire, why: &str) {
    let rounds = g.k.lanes.pin_rounds;
    let spent = rounds - fr.n;
    let shots = if js::truthy(spent) { format!(" — {} round{} down it", spent, if spent == 1.0 { "" } else { "s" }) } else { String::new() };
    let id = g.tab.lanes_lane[l].id;
    note(g, Some(gd), &format!("lane {}: {}{}{}: back", id, why, shots, if fr.saw { ", they are still on it" } else { "" }));
    let w = g.gd[gd].weapon;
    if crate::weapons::firearm_rounds_ready(g, w) < rounds && g.gd[gd].reload_t < 0.0 { crate::guards::start_reload_beat(g, gd); }
    peek_back(g, gd, d);
}
/// every frame, a man out at his corner to fire down a lane: the moment his muzzle has a line he stops and puts his burst there, wide, then gets back (lanes.ts pinStep)
pub(super) fn pin_step(g: &mut crate::game::Game, gd: GuardId, l: LanesLaneId, d: RoomfightDefId, seen: Option<V3>) {
    let c = g.gd[gd].ch;
    let p = match s(g).peek.get(gd) { Some(&p) => p, None => return };   // (a read of the row: its writes go by the path)
    let mut fr = match p.fire { Some(f) => f, None => return };
    if let (Some(seen), false) = (seen, fr.saw) {   // (seen still on it: the lane stands; no news asked for, so the looks' own wait is not doubled)
        fr.saw = true;
        let (now, keep) = (g.time, g.k.lanes.peek_keep);
        let x = &mut g.tab.lanes_lane[l];
        x.until = m::max(x.until, now + keep);
        x.peek_part = 0.0;
        if x.guns.len() < 4 && x.guns.iter().all(|&q| flat(q, seen) > 1.5) { x.guns.push([seen[0], 0.0, seen[2]]); }
        put_fire(g, gd, fr);
    }
    let mut tgt: Option<V3> = None;
    let mut bd = f64::INFINITY;
    let e = crate::glasspen::shot_energy(g, c, None);
    for i in 0..g.tab.lanes_lane[l].guns.len() {
        let q0 = g.tab.lanes_lane[l].guns[i];
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
            let give_up = if fr.t >= 0.0 { g.time - fr.line_t > 0.2 } else { !crate::roomfight::on_leg_of(g, d, crate::owner::by::LANE) || g.time - p.t > 6.0 };
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
        crate::roomfight::run(g, d, here, None, 0.0, crate::owner::by::LANE);
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
        if fr.n == g.k.lanes.pin_rounds && g.tab.lanes_lane[l].pin_n == 1.0 { s(g).said_t = g.time; say_net(g, gd, "putting fire on them!"); }
        fr.n -= 1.0;
        let cd = g.m.weapons.weapons.get(g.fa[w].def).unwrap().cd;
        fr.next = g.time + m::max(g.k.lanes.pin_every, cd);
        s(g).stats.pin_rounds += 1.0;
        put_fire(g, gd, fr);
    }
    if fr.n <= 0.0 { pin_done(g, gd, l, d, fr, "his burst is fired"); }
    else if !crate::weapons::firearm_can_fire(g, w) && crate::weapons::firearm_rounds_ready(g, w) == 0.0 { pin_done(g, gd, l, d, fr, "dry"); }
}
/// (lanes.ts:697 `F`) the burst, written to the man's row
fn put_fire(g: &mut Game, gd: GuardId, fr: crate::lanes::PeekFire) {
    if let Some(r) = s(g).peek.get_mut(gd) { r.fire = Some(fr); }
}

/// he has shouted and frag.ts's wind-up has his arm (half a second past its time at the most) (lanes.ts winding)
pub(super) fn winding(g: &crate::game::Game, gd: GuardId, f: crate::lanes::PeekFrag) -> bool {
    f.at.is_some() && crate::frag::frag_winding(g, gd) && g.time < f.until + 0.5
}
/// A GRENADE DOWN IT: of the men held on the lane who carry one and may throw this moment, one goes looking for an arc to the middle of where its fire came from (lanes.ts fragOut)
fn frag_out(g: &mut crate::game::Game, l: LanesLaneId, at: &[GuardId]) {
    g.tab.lanes_lane[l].frag_at = g.time + 2.0;   // (asked again soon while nobody is placed for it)
    if at.is_empty() { return; }
    let id = g.tab.lanes_lane[l].id;
    if s(g).peek.values().any(|p| p.lane == id) { return; }
    if s(g).flank_of.values().any(|f| f.lane == id && f.phase != crate::lanes::FlankPhase::Set) { return; }   // (not on the head of a mate walking round to them)
    let mid = guns_mid(g, l);
    let mut can: Vec<GuardId> = Vec::new();
    for &gd in at {
        let (no, h) = (s(g).frag_no.get(gd).copied(), s(g).held.get(gd).copied());   // (`at` was read before this tick's charge and its men sent round: one of them may be held no longer)
        let h = match h { Some(h) => h, None => continue };
        if !(h.lane == id && h.aimed) { continue; }
        let holds = match def_of(g, gd) { Some(d) => crate::roomfight::at_post(g, d), None => false };
        if holds && !pushing(g, gd) && !flanking(g, gd) && !no.map_or(false, |no| no.lane == id && flat(no.at, mid) < 1.5) && crate::frag::frag_ready(g, gd, mid) { can.push(gd); }
    }
    if can.is_empty() { return; }
    let mut skip: JsSet<GuardId> = JsSet::new();
    for i in 0..g.fight.defs.len() { let o = g.tab.roomfight_def[g.fight.defs[i]].gd; if !can.contains(&o) { skip.add(o); } }
    let near = g.k.frag.def_near;
    let pick = stepper(g, l, Some(&skip), false, Some(near));
    let gd = match pick { Some(p) => p.gd, None => can[0] };
    let p = pos_of(g, gd);
    let look = match s(g).held.get(gd).map(|h| h.look) { Some(x) => x, None => mouth_of(g, l, p) };
    held_off(g, gd);
    let to = match pick { Some(x) => x.to, None => mouth_of(g, l, p) };
    let row = crate::lanes::Peek { lane: id, phase: crate::lanes::PeekPhase::Out, t: g.time, home: p, look, to, line_t: -1.0, eye: None, fire: None,
        frag: Some(crate::lanes::PeekFrag { at: None, k: 0.0, next: 0.0, walk: pick.is_some(), going: false, edge: false, own: false, until: 0.0, had: 0.0 }) };
    peek_on(g, gd, row);
}
/// (gunsMid over a lane's row)
fn mid_of(l: &crate::lanes::Lane) -> V3 {
    if l.guns.is_empty() { return l.from; }
    let (mut mx, mut mz) = (0.0f64, 0.0f64);
    for q in l.guns.iter() { mx += q[0]; mz += q[2]; }
    mx /= l.guns.len() as f64;
    mz /= l.guns.len() as f64;
    let (mut best, mut bd) = (l.guns[0], f64::INFINITY);
    for q in l.guns.iter() { let d = m::hyp2(q[0] - mx, q[2] - mz); if d < bd { bd = d; best = *q; } }
    best
}
/// the middle of where a lane's fire came from: of the places its guns fired from, the one nearest their mean (lanes.ts gunsMid)
fn guns_mid(g: &crate::game::Game, l: LanesLaneId) -> V3 { mid_of(&g.tab.lanes_lane[l]) }
/// where his feet would be `s` metres on: along the walk he is on (`going`), else straight toward `to`. Edge = a covered lane's floor, or past the end of his walk; None = a wall on the straight line (lanes.ts fragPlace)
fn frag_place(g: &mut crate::game::Game, gd: GuardId, to: V3, s: f64, going: bool) -> Option<crate::lanes::FragPlace> {
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
pub(super) fn frag_step(g: &mut crate::game::Game, gd: GuardId, l: LanesLaneId, d: RoomfightDefId) {
    let p = match s(g).peek.get(gd) { Some(&p) => p, None => return };
    let mut f = match p.frag { Some(f) => f, None => return };
    let mut to = p.to;
    frag_step_on(g, gd, p.t, &mut to, &mut f, l, d);
    if let Some(r) = s(g).peek.get_mut(gd) { r.frag = Some(f); r.to = to; }   // (only these two fields: peek_back has written phase and t on the row)
}
fn frag_step_on(g: &mut Game, gd: GuardId, t0: f64, to: &mut V3, f: &mut crate::lanes::PeekFrag, l: LanesLaneId, d: RoomfightDefId) {
    use crate::frag::FragThrowAt;
    use crate::lanes::FragPlace;
    let id = g.tab.lanes_lane[l].id;
    let pace = g.k.lanes.peek_pace - g.tune.guard_chase;
    let big_n = if f.going { m::max(1.0, m::round(g.k.lanes.frag_step_m / 0.3)) + 1.0 } else { 1.0 };
    if let Some(was) = f.at {
        if winding(g, gd, *f) { return; }
        let went = crate::frag::frag_away(g, gd) > f.had;
        if went { s(g).stats.frags += 1.0; }
        else {   // (it stayed in his hand: he does not shout that place again; another carrier may)
            s(g).stats.frag_kept += 1.0;
            g.tab.lanes_lane[l].frag_at = g.time + 6.0;
            s(g).frag_no.set(gd, crate::lanes::StateFragNoValue { lane: id, at: was });
        }
        note(g, Some(gd), &format!("lane {}: {}: back", id, if went { "the grenade is away" } else { "the grenade stayed in his hand" }));
        peek_back(g, gd, d);
        return;
    }
    let at = guns_mid(g, l);
    if g.tab.lanes_lane[l].can.is_some() || g.time < g.tab.lanes_lane[l].push_until || !crate::frag::frag_ready(g, gd, at) {
        g.tab.lanes_lane[l].frag_at = g.time + 2.0;
        if f.going { note(g, Some(gd), &format!("lane {}: not now for his grenade — back", id)); }
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
            crate::roomfight::run(g, d, x, None, pace, crate::owner::by::LANE);
            aim_at(g, gd, [at[0], 1.2, at[2]]);
        }
        let (now, gap) = (g.time, g.k.lanes.frag_gap);
        let y = &mut g.tab.lanes_lane[l];
        y.frag_at = now + gap;
        y.peek_at = m::max(y.peek_at, quiet);
        y.pin_at = m::max(y.pin_at, quiet);
        y.stun_at = m::max(y.stun_at, quiet);
        y.round_at = m::max(y.round_at, quiet);
        let off = flat(pos_of(g, gd), at);
        note(g, Some(gd), &format!("lane {}: a grenade down it {}, for ({}, {}), {} m off", id, if f.going { "from the edge of its floor" } else { "from behind his corner" }, js::to_fixed(at[0], 1), js::to_fixed(at[2], 1), js::to_fixed(off, 0)));
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
        crate::roomfight::run(g, d, *to, None, pace, crate::owner::by::LANE);
        note(g, Some(gd), &format!("lane {}: no arc for his grenade from where he is held — toward its mouth at ({}, {}) for one", id, js::to_fixed(to[0], 1), js::to_fixed(to[2], 1)));
        return;
    }
    if f.going && !f.edge && crate::roomfight::on_leg_of(g, d, crate::owner::by::LANE) && g.time - t0 < 6.0 { return; }
    if f.own { g.tab.lanes_lane[l].frag_at = g.time + 6.0; } else { s(g).frag_no.set(gd, crate::lanes::StateFragNoValue { lane: id, at }); }   // (one of his own in its reach is no news about the arc: asked again soon)
    let what = if f.own { String::from("one of his own in his grenade's reach") } else { format!("no arc down it for his grenade{}", if f.going { " as far as the edge of its floor" } else { "" }) };
    note(g, Some(gd), &format!("lane {}: {} — {}", id, what, if f.going { "back" } else { "stays" }));
    peek_back(g, gd, d);
}
/// A LOOK ROUND THE CORNER: of the men this rule holds short of a quiet lane's mouth, the nearest steps to where it opens and looks down it (lanes.ts peekStart)
fn peek_start(g: &mut crate::game::Game, l: LanesLaneId) {
    g.tab.lanes_lane[l].peek_at = g.time + 3.0;   // (asked again soon while nobody is placed for it)
    let id = g.tab.lanes_lane[l].id;
    if s(g).peek.values().any(|p| p.lane == id) { return; }
    let pick = match stepper(g, l, None, false, None) { Some(p) => p, None => return };
    let (best, to) = (pick.gd, pick.to);
    let d = match def_of(g, best) { Some(d) => d, None => return };
    let p = pos_of(g, best);
    let look = match s(g).held.get(best).map(|h| h.look) { Some(x) => x, None => mouth_of(g, l, p) };
    g.tab.lanes_lane[l].peek_at = g.time + g.k.lanes.peek_gap;
    let row = crate::lanes::Peek { lane: id, phase: crate::lanes::PeekPhase::Out, t: g.time, home: p, look, to, line_t: -1.0, eye: None, fire: None, frag: None };
    held_off(g, best);
    peek_on(g, best, row);
    clear_path(g, best);
    let fast = g.k.lanes.peek_pace - g.tune.guard_chase;
    crate::roomfight::run(g, d, to, None, fast, crate::owner::by::LANE);
    s(g).stats.peeks += 1.0;
    let last_t = g.tab.lanes_lane[l].last_t;
    note(g, Some(best), &format!("lane {}: quiet {} s — steps to its mouth toward ({}, {}) for a look", id, js::to_fixed(g.time - last_t, 0), js::to_fixed(to[0], 1), js::to_fixed(to[2], 1)));
}

/// (lanes.ts:818 back) on his way back to where he stood, for pinStep and fragStep. The row is there at every call: only peekStep and lanesReset delete one
fn peek_back(g: &mut Game, gd: GuardId, d: RoomfightDefId) {
    let now = g.time;
    let home = { let r = s(g).peek.get_mut(gd).unwrap(); r.phase = crate::lanes::PeekPhase::Back; r.t = now; r.home };
    clear_path(g, gd);
    let fast = g.k.lanes.push_fast;
    crate::roomfight::run(g, d, home, None, fast, crate::owner::by::LANE);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_middle_is_the_gun_nearest_the_mean() {
        let mut l = crate::lanes::Lane::default();
        l.from = [9.0, 0.0, 9.0];
        assert_eq!(mid_of(&l), [9.0, 0.0, 9.0]);
        l.guns = vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [3.0, 0.0, 0.0]];   // (mean x 7/3: nearest is 3)
        assert_eq!(mid_of(&l), [3.0, 0.0, 0.0]);
    }
    #[test]
    fn the_middle_of_a_cell() {
        assert_eq!(cell_mid(0.5, 10.0, 23.0), [1.75, 0.0, 1.25]);
    }
    #[test]
    fn a_key_of_minus_zero_is_zero() { assert!(cell_key(-0.0) == crate::Num(0.0)); }
}
