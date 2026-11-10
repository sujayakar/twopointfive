use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small readers

fn pos_of(g: &Game, m: GuardId) -> V3 {
    g.ch[g.gd[m].ch].pos
}
/// the men's callsigns, joined
fn calls(g: &Game, men: &[GuardId]) -> String {
    men.iter().map(|&m| g.gd[m].callsign.clone()).collect::<Vec<String>>().join(", ")
}
/// `men.filter(m => m.state !== 'dead')`
fn alive(g: &Game, men: &[GuardId]) -> Vec<GuardId> {
    men.iter().copied().filter(|&m| !crate::condition::down(g, g.gd[m].ch)).collect()
}
/// `[...new Set(words)].join(' + ')`: first sight's order
fn uniq_plus(words: Vec<String>) -> String {
    let mut seen: Vec<String> = Vec::new();
    for w in words {
        if !seen.contains(&w) {
            seen.push(w);
        }
    }
    seen.join(" + ")
}
/// the key's letter: `DOORTOOLS.blowKey.replace('Key', '')`
fn blow_letter(g: &Game) -> String {
    g.k.doortools.blow_key.replacen("Key", "", 1)
}
/// `closeWords(c)` with the warning's head in capitals (OWNCHARGE.on)
fn close_caps(g: &crate::game::Game, seat: &crate::seats::SeatRef, c: ChargeId) -> String {
    door_tools_close_words_of(g, seat, c).replacen("too close:", "TOO CLOSE:", 1).replacen("behind the door:", "BEHIND THE DOOR:", 1)
}
/// every other cell each way: `!((floor(x / cell) | floor(z / cell)) & 1)`, a metre's lattice of the room
fn on_lattice(x: f64, z: f64, cell: f64) -> bool {
    ((js::to_i32(m::floor(x / cell)) | js::to_i32(m::floor(z / cell))) & 1) == 0
}
/// `pos === pos` of the arrays S.pair hands back: the bits of the coordinates stand in for the array's identity
fn same_pos(a: V3, b: V3) -> bool {
    a[0].to_bits() == b[0].to_bits() && a[1].to_bits() == b[1].to_bits() && a[2].to_bits() == b[2].to_bits()
}
/// `v.slice(a, b)`
fn js_slice<T: Copy>(v: &[T], a: usize, b: usize) -> Vec<T> {
    let n = v.len();
    v[a.min(n)..b.min(n).max(a.min(n))].to_vec()
}

// ---------------------------------------------------------------- the grenade's wait

/// The names of who ran and is not back.
pub(super) fn fled_names(g: &Game, fled: &Option<JsSet<GuardId>>) -> String { calls(g, &fled.as_ref().map(|f| f.keys()).unwrap_or_default()) }
/// The first of the men who did not run says so.
pub(super) fn fled_said(g: &mut Game, op: DoorOpId, fled: Option<JsSet<GuardId>>) {
    door_tools_beat(g, op, &format!("the door waits: {} not back", fled_names(g, &fled)));
    let r = &g.tab.doortools_door_op[op];
    let who = r.men.iter().copied().find(|&m| !fled.as_ref().is_some_and(|f| f.has(m))).unwrap_or(r.lead);
    crate::game::game_say(g, who, "hold — back on the door.", None, None);
}

/// Once a tick for an order whose door is still to be opened: who ran from a grenade and is not back, and true when the order does not move
/// this tick.
pub(super) fn door_tools_frag_step(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let t = g.time;
    let o = &mut g.tab.doortools_door_op[op];
    if o.fled.is_none() {
        o.fled = Some(JsSet::new());
    }
    let men = o.men.clone();
    let f = o.fled.as_mut().unwrap();
    let n0 = f.len();
    let mut came = false;
    for m in f.keys() {
        if !men.contains(&m) {
            f.delete(m);
        }
    }
    for &m in &men {
        if crate::frag::frag_running(g, m) {
            g.tab.doortools_door_op[op].fled.as_mut().unwrap().add(m);
        } else if g.tab.doortools_door_op[op].fled.as_ref().unwrap().has(m) && door_tools_frag_back(g, op, m) {
            g.tab.doortools_door_op[op].fled.as_mut().unwrap().delete(m);
            came = true;
        }
    }
    let size = g.tab.doortools_door_op[op].fled.as_ref().unwrap().len();
    if size > 0 && n0 == 0 {
        let what = format!("grenade on the stack · {} off the door", fled_names(g, &g.tab.doortools_door_op[op].fled));
        door_tools_beat(g, op, &what);
    }
    if size == 0 && n0 > 0 {
        door_tools_beat(g, op, if came { "the men are back" } else { "nobody left to wait for" });
    }
    let holds = door_tools_frag_holds(g, op);
    if !holds || !door_tools_frag_wait(g, op) {
        if holds || g.tab.doortools_door_op[op].fled.as_ref().unwrap().is_empty() {
            door_tools_frag_said(g, op);
        }
        g.tab.doortools_door_op[op].fled_t = None;
        return false;
    }
    let o = &mut g.tab.doortools_door_op[op];
    let dt = match o.fled_t {
        None => 0.0,
        Some(ft) => m::min(t - ft, 0.1),
    };
    o.fled_t = Some(t);
    o.fled_w = Some(o.fled_w.unwrap_or(0.0) + dt);
    o.t0 += dt;
    if o.phase == Phase::Place {
        o.set_t = f64::NAN;
    }
    if !o.fled_said.unwrap_or(false) {
        o.fled_said = Some(true);
        let fled = g.tab.doortools_door_op[op].fled.clone();
        fled_said(g, op, fled);
    }
    true
}

// ---------------------------------------------------------------- the threshold: who throws, what is lit, where a can goes

/// Is he at the door, on the order's side of it.
pub(super) fn door_tools_thr_at(g: &crate::game::Game, op: DoorOpId, m: GuardId) -> bool {
    let (d, side) = (g.tab.doortools_door_op[op].door, g.tab.doortools_door_op[op].side);
    let p = pos_of(g, m);
    v3::dist_xz(p, g.dr[d].frame_centre) < g.k.doortools.throw_reach && off_of(g, d, p, side) > 0.0
}

/// The one order of men a can is thrown by; with `hand`, the hand a can may leave right now, or none.
pub(super) fn door_tools_thr_pick(g: &mut crate::game::Game, op: DoorOpId, hand: bool, fit: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId) -> bool>) -> Option<GuardId> {
    let mut fit = fit;
    let reach = g.k.doortools.throw_reach;
    let o = &g.tab.doortools_door_op[op];
    let (d, lead, last_stack, men) = (o.door, o.lead, o.last_stack.unwrap_or(false), o.men.clone());
    let jamb = o.thr.as_ref().unwrap().jamb.clone();
    let fc = g.dr[d].frame_centre;

    let mut others: Vec<GuardId> = Vec::new();
    for &m in &men {
        if !jamb.contains(&m) && door_tools_thr_at(g, op, m) {
            others.push(m);
        }
    }
    {
        let g: &Game = g;
        js::sort_by(&mut others, |&a, &b| v3::dist_xz(pos_of(g, a), fc) - v3::dist_xz(pos_of(g, b), fc));
    }
    for &m in &others {
        if thr_pick_fit(g, &mut fit, m) {
            return Some(m);
        }
    }
    for &m in &jamb {
        if m == lead {
            continue;
        }
        // (a stack that went without its far-jamb man: he throws nothing from down the wall, the lead's own hand)
        let ok = if hand { door_tools_thr_at(g, op, m) } else { !last_stack || v3::dist_xz(pos_of(g, m), fc) < reach };
        if ok && thr_pick_fit(g, &mut fit, m) {
            return Some(m);
        }
    }
    if !hand {
        return Some(lead);
    }
    if !door_tools_thr_at(g, op, lead) || !thr_pick_fit(g, &mut fit, lead) {
        return None;
    }
    for &m in &men {
        if m != lead && !door_tools_thr_back(g, op, m) {
            return None;
        }
    }
    Some(lead)
}
/// `fit(m)`, its default being yes
fn thr_pick_fit(g: &mut Game, fit: &mut Option<&mut dyn FnMut(&mut Game, GuardId) -> bool>, m: GuardId) -> bool {
    match fit {
        Some(f) => f(g, m),
        None => true,
    }
}

/// Is this cell of the room lit by the place itself (asked once a cell an order).
pub(super) fn door_tools_thr_lit_cell(g: &mut crate::game::Game, t_key: DoorOpId, sr: DoortoolsRoomSearchId, i: f64) -> bool {
    let i = i as usize;
    let (n, q) = (g.tab.doortools_room_search[sr].pts.len(), g.tab.doortools_room_search[sr].pts[i]);
    let c = g.tab.doortools_door_op[t_key].thr.as_mut().unwrap().lit_c.get_or_insert_with(|| vec![-1i8; n])[i];
    if c >= 0 {
        return c == 1;
    }
    let mut every = |_: &Game, _: f64| true;
    let e = crate::guards::light_estimate_at(g, [q[0], 1.2, q[2]], Some(&mut every), None, None);
    let c = if crate::game::game_visibility_of(g, e) >= g.k.lights.lit_at { 1 } else { 0 };
    g.tab.doortools_door_op[t_key].thr.as_mut().unwrap().lit_c.as_mut().unwrap()[i] = c;
    c == 1
}

/// Does this cell show a standing man through the goggles by the place's own light (asked once a cell an order).
pub(super) fn door_tools_thr_tube_cell(g: &mut crate::game::Game, t_key: DoorOpId, sr: DoortoolsRoomSearchId, i: f64) -> bool {
    let i = i as usize;
    let (n, q) = (g.tab.doortools_room_search[sr].pts.len(), g.tab.doortools_room_search[sr].pts[i]);
    let c = g.tab.doortools_door_op[t_key].thr.as_mut().unwrap().tube_c.get_or_insert_with(|| vec![-1i8; n])[i];
    if c >= 0 {
        return c == 1;
    }
    let e = crate::nvg::nvg_place_light(g, [q[0], 1.2, q[2]]);
    let c = if crate::nvg::tube_vis(g, e) >= g.k.nvg.black_at { 1 } else { 0 };
    g.tab.doortools_door_op[t_key].thr.as_mut().unwrap().tube_c.as_mut().unwrap()[i] = c;
    c == 1
}

/// Where a can leaves the hand: in the opening's span at the thrower's end, a forearm outside the frame line.
pub(super) fn door_tools_hand_at(g: &crate::game::Game, op: DoorOpId, th: GuardId) -> V3 {
    let (d, side) = (g.tab.doortools_door_op[op].door, g.tab.doortools_door_op[op].side);
    let w = g.dr[d].def.width;
    let p = pos_of(g, th);
    let a = v3::clamp(along_of(g, d, p), 0.25, w - 0.25);
    door_pt(g, d, a, 0.35, side, Some(1.3))
}

/// The blind pocket worth a can: the point in the room whose burst has a line to the most floor nobody has seen; none when too little is left.
pub(super) fn door_tools_pocket_of(g: &mut crate::game::Game, op: DoorOpId, th: GuardId) -> Option<crate::doortools::DoorToolsPocketOf> {
    let (thr_pocket, thr_throw_m, thr_flash_m, air_h) = (g.k.doortools.thr_pocket, g.k.doortools.thr_throw_m, g.k.doortools.thr_flash_m, g.k.doortools.air_h);
    let cell = g.col.nav.cell;
    let o = &g.tab.doortools_door_op[op];
    let (d, side) = (o.door, o.side);
    let Some(sr) = o.search.flatten() else {
        return if js::truthy(o.thr.as_ref().unwrap().tossed) { None } else { Some(DoorToolsPocketOf { at: None, n: 0.0 }) };
    };
    let (pts, looked) = (g.tab.doortools_room_search[sr].pts.clone(), g.tab.doortools_room_search[sr].looked.clone());
    let thr = g.tab.doortools_door_op[op].thr.as_mut().unwrap();
    let banged = thr.banged.get_or_insert_with(|| vec![0u8; pts.len()]);
    let mut dark: Vec<V3> = Vec::new();
    for i in 0..pts.len() {
        if looked[i] == 0 && banged[i] == 0 {
            dark.push(pts[i]);
        }
    }
    if (dark.len() as f64) < thr_pocket {
        return None;
    }
    let from = door_tools_hand_at(g, op, th);
    let sd = m::max(1.0, m::ceil(dark.len() as f64 / 40.0));
    let mut spots: Vec<V3> = Vec::new();
    for &p in &pts {
        if on_lattice(p[0], p[2], cell) && -off_of(g, d, p, side) >= 1.0 && v3::dist_xz(from, p) <= thr_throw_m {
            spots.push(p);
        }
    }
    let mut cands: Vec<(V3, f64)> = Vec::new();
    // (a big room: no more than forty spots against forty cells in the one frame)
    let sj = m::max(1.0, m::ceil(spots.len() as f64 / 40.0)) as usize;
    let mut j = 0;
    while j < spots.len() {
        let p = spots[j];
        j += sj;
        let at: V3 = [p[0], air_h - 0.05, p[2]];
        if crate::scene_collision::solid_blocked(g, from, at, None) {
            continue;
        }
        let mut n = 0.0;
        let mut k = 0;
        while k < dark.len() {
            let q = dark[k];
            k += sd as usize;
            if v3::dist_xz(p, q) <= thr_flash_m && !crate::scene_collision::segment_blocked(g, at, [q[0], 1.45, q[2]], None, None, None) {
                n += 1.0;
            }
        }
        if n * sd >= thr_pocket {
            cands.push((at, m::min(dark.len() as f64, n * sd)));
        }
    }
    js::sort_by(&mut cands, |a, b| js::or(b.1 - a.1, v3::dist_xz(a.0, from) - v3::dist_xz(b.0, from)));
    for &(at, n) in cands.iter().take(6) {
        // (the throw's own arc gets there: not into the lintel, not onto a cabinet's top)
        let arc = crate::throwables::throwables_solve(g, from, at, Some(9.0));
        if arc.points.iter().any(|&q| v3::dist_xz(q, at) < 0.35 && m::abs(q[1] - at[1]) < 0.6) {
            return Some(DoorToolsPocketOf { at: Some(at), n });
        }
    }
    None
}

/// A can of the stage's has gone off: the floor its burst had a line to is no pocket any more; the next one's turn, or the count to the step in.
pub(super) fn door_tools_thr_banged(g: &mut crate::game::Game, op: DoorOpId) {
    let (thr_flash_m, thr_next) = (g.k.doortools.thr_flash_m, g.k.doortools.thr_next);
    let at = g.tab.doortools_door_op[op].bang_at;
    if let (Some(sr), Some(at)) = (g.tab.doortools_door_op[op].search.flatten(), at) {
        let (pts, looked) = (g.tab.doortools_room_search[sr].pts.clone(), g.tab.doortools_room_search[sr].looked.clone());
        let (mut lit, mut dark) = (0, 0);
        let thr = g.tab.doortools_door_op[op].thr.as_mut().unwrap();
        let mut banged = thr.banged.take().unwrap_or_else(|| vec![0u8; pts.len()]);
        for i in 0..pts.len() {
            if looked[i] != 0 || banged[i] != 0 {
                continue;
            }
            dark += 1;
            let q = pts[i];
            // (from where it really went off: a can that fell behind a desk lit less)
            if v3::dist_xz(at, q) <= thr_flash_m && !crate::scene_collision::segment_blocked(g, [at[0], m::max(0.15, at[1]), at[2]], [q[0], 1.45, q[2]], None, None, None) {
                banged[i] = 1;
                lit += 1;
            }
        }
        g.tab.doortools_door_op[op].thr.as_mut().unwrap().banged = Some(banged);
        let what = format!("pocket banged · {} of {} unseen cells in its flash", lit, dark);
        door_tools_beat(g, op, &what);
    }
    let time = g.time;
    let o = &mut g.tab.doortools_door_op[op];
    let (door, side) = (o.door, o.side);
    o.phase = Phase::Pocket;
    let thr = o.thr.as_mut().unwrap();
    thr.aim = None;
    thr.from = None;
    thr.next_t = time + thr_next;
    if thr.first == Some(ThresholdFirst::Can) {
        // (a can that clipped the leaf and went off on the team's side blinded nobody in there: the hold behind it is a plain one)
        let wide = match at {
            None => true,
            Some(at) => off_of(g, door, at, side) >= 0.0,
        };
        let thr = g.tab.doortools_door_op[op].thr.as_mut().unwrap();
        thr.first = Some(ThresholdFirst::Burst);
        thr.wide = Some(wide);
    }
}

// ---------------------------------------------------------------- the step in

/// `fits` of shallowPods: free floor, a clear line from the doorway's middle, apart from those already taken, not in the frame.
fn shallow_fits(g: &Game, d: DoorId, mid: V3, out: &[crate::entrypoints::Pod], p: V3) -> bool {
    !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2])
        && !crate::scene_collision::solid_blocked(g, mid, [p[0], 0.45, p[2]], None)
        && !out.iter().any(|o| v3::dist_xz(o.pos, p) < 0.9)
        && !door_tools_in_frame(g, d, mid, p)
}

/// The shallow points a threshold stage steps in to, by the door's own geometry: to either hand hard on the entry wall, then a little deeper,
/// then down the middle, none more than 2.5 m inside; the entry's own points where no floor like that exists.
pub fn door_tools_shallow_pods(g: &mut crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> {
    let n = n.unwrap_or(2.0);
    let w = g.dr[d].def.width;
    let into = -side;
    let mid = door_pt(g, d, w / 2.0, 0.9, into, Some(0.45));
    let mut out: Vec<crate::entrypoints::Pod> = Vec::new();
    for s in [1.0, -1.0] {
        // (L past the jamb: never in the opening itself)
        'find: for dd in [1.0, 1.4, 0.8, 1.9] {
            for l in [w / 2.0 + 0.9, w / 2.0 + 1.4, w / 2.0 + 0.55, w / 2.0 + 2.0] {
                let p = door_pt(g, d, w / 2.0 + s * l, dd, into, None);
                if !shallow_fits(g, d, mid, &out, p) {
                    continue;
                }
                let look = door_pt(g, d, w / 2.0 - s * 0.6, 4.5, into, None);
                out.push(crate::entrypoints::Pod { pos: p, face: v3::yaw_to(p, look), watch: None });
                break 'find;
            }
        }
    }
    for s in [1.0, -1.0] {
        if out.len() as f64 >= n {
            break;
        }
        'find: for dd in [2.2, 1.7, 2.5] {
            for l in [1.7, 1.3, 2.3, 0.9] {
                let p = door_pt(g, d, w / 2.0 + s * l, dd, into, None);
                if !shallow_fits(g, d, mid, &out, p) {
                    continue;
                }
                let look = door_pt(g, d, w / 2.0 - s * 1.2, 5.0, into, None);
                out.push(crate::entrypoints::Pod { pos: p, face: v3::yaw_to(p, look), watch: None });
                break 'find;
            }
        }
    }
    let (i0, i1) = (door_pt(g, d, w / 2.0, 0.0, into, None), door_pt(g, d, w / 2.0, 1.0, into, None));
    let inward = v3::yaw_to(i0, i1);
    for b in [1.5, 2.4, 2.5] {
        if out.len() as f64 >= n {
            break;
        }
        let q = door_pt(g, d, w / 2.0, b, into, None);
        let f = if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { crate::scene_collision::nav_grid_nearest_free_point(g, q[0], q[2]) } else { Some(q) };
        if let Some(f) = f {
            let off = -off_of(g, d, [f[0], 0.0, f[2]], side);
            if off > 0.3 && off <= 2.5 && !out.iter().any(|o| v3::dist_xz(o.pos, f) < 0.9) {
                out.push(crate::entrypoints::Pod { pos: [f[0], 0.0, f[2]], face: inward, watch: None });
            }
        }
    }
    if out.is_empty() {
        return door_tools_pods(g, d, side, Some(n));
    }
    let mut pts = js_slice(&out, 0, n as usize);
    pts
}

/// The step in: the men off the jambs first, each to the shallow point that is the least walk, then the rest in their turn; from here the op
/// is in 'enter'.
pub(super) fn door_tools_step_in(g: &mut crate::game::Game, op: DoorOpId) {
    let (reach, thr_in) = (g.k.doortools.throw_reach, g.k.doortools.thr_in);
    let t = g.time;
    let d = g.tab.doortools_door_op[op].door;
    let fc = g.dr[d].frame_centre;
    door_tools_thr_men(g, op);
    let o = &g.tab.doortools_door_op[op];
    let jamb = o.thr.as_ref().unwrap().jamb.clone();
    if jamb.len() > 1 {
        // (the man off the jambs who has seen most of the room steps in first: not the one who threw the pockets' cans, nor, on a last
        // warning's stack, a man still walking up)
        let (lead, threw, last_stack) = (o.lead, o.threw, o.last_stack.unwrap_or(false));
        let at = o.thr.as_ref().unwrap().pods.first().map(|p| p.pos);
        let able: Vec<GuardId> = jamb
            .iter()
            .copied()
            .filter(|&m| !crate::condition::down(g, g.gd[m].ch) && (Some(m) != threw || m == lead) && (m == lead || !last_stack || v3::dist_xz(pos_of(g, m), fc) < reach))
            .collect();
        let able = crate::gunroles::gun_first(g, &able);
        let f = door_tools_seen_most(g, lead, &able, at);
        if f != jamb[0] {
            let mut first = vec![f];
            first.extend(jamb.iter().copied().filter(|&m| m != f));
            g.tab.doortools_door_op[op].thr.as_mut().unwrap().jamb = first;
            let what = format!("first:{} · has seen most of the room", g.gd[f].callsign);
            door_tools_beat(g, op, &what);
        }
    }
    let o = &g.tab.doortools_door_op[op];
    let (men, lead) = (o.men.clone(), o.lead);
    let thr = o.thr.as_ref().unwrap();
    let (jamb, t_pods) = (thr.jamb.clone(), thr.pods.clone());
    let pods: Vec<crate::entrypoints::Pod> = if crate::orders::squad_orders_big(g, &crate::seats::here(g)) && men.len() <= 2 { door_tools_pods_apart(g, op, &t_pods, men.len() as f64) } else { t_pods };
    if pods.is_empty() {
        crate::game::game_say(g, lead, "no way in.", None, None);
        door_tools_end(g, op, crate::doortools::Ended::NoFloorInside);
        return;
    }
    let mut rest: Vec<GuardId> = men.iter().copied().filter(|m| !jamb.contains(m)).collect();
    {
        let g: &Game = g;
        js::sort_by(&mut rest, |&a, &b| v3::dist_xz(pos_of(g, a), fc) - v3::dist_xz(pos_of(g, b), fc));
    }
    let mut turn = jamb;
    turn.extend(rest);
    let n_in = m::min(pods.len() as f64, m::max(1.0, thr_in)) as usize;
    let in_men = js_slice(&turn, 0, n_in);
    let stay = js_slice(&turn, n_in, turn.len());
    let mut deal: JsMap<GuardId, crate::entrypoints::Pod> = JsMap::new();
    for k in [0, 2] {
        let mm = js_slice(&in_men, k, k + 2);
        let sp = js_slice(&pods, k, k + 2);
        if !mm.is_empty() {
            let spots: Vec<V3> = sp.iter().map(|x| x.pos).collect();
            for (gd, pos) in crate::orders::squad_orders_pair(g, &mm, &spots, None, None) {
                if let Some(x) = sp.iter().find(|x| same_pos(x.pos, pos)) {
                    deal.set(gd, *x);
                }
            }
        }
    }
    if !stay.is_empty() {
        g.tab.doortools_door_op[op].men.retain(|m| !stay.contains(m));
        let what = format!("outside · {}", calls(g, &stay));
        door_tools_beat(g, op, &what);
    }
    let queue: Vec<DoorToolsDoneItemQueueItem> = in_men
        .iter()
        .enumerate()
        .map(|(i, &gd)| DoorToolsDoneItemQueueItem { gd, pod: deal.get(gd).copied().unwrap_or(pods[0]) })
        .collect();
    let o = &mut g.tab.doortools_door_op[op];
    o.rear.clear();
    o.queue = queue;
    o.phase = Phase::Enter;
    o.t0 = t;
    let lead = o.lead;
    let what = format!("step in · {}", calls(g, &in_men));
    door_tools_beat(g, op, &what);
    crate::game::game_say(g, lead, "stepping in.", None, None);
}

// ---------------------------------------------------------------- the HUD, the console

/// The squad's HUD block: what is left, and a charge waiting for the trigger.
pub fn door_tools_hud_notes(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<String> {
    let mut out = vec![format!("charges {} · bangs {}", seat.door_tools.charges, seat.door_tools.bangs)];
    let f = blow_letter(g);
    if g.k.frag.squad > 0.0 {
        out.push(format!("frags {}", seat.door_tools.frags));
    }
    let word = crate::orders::squad_orders_big(g, seat) && {
        let men = door_tools_word_men(g, seat);
        !door_tools_waiting_any(g, seat, &men).is_empty()
    };
    for c in seat.door_tools.set.clone() {
        let door = g.tab.doortools_set_charge[c].door;
        let name = crate::orders::door_words(g, door);
        let tail = if door_tools_on_pending_word(g, seat, c) {
            String::from(" — on the word")
        } else if word {
            String::new()
        } else {
            let key_is = door_tools_key_is(g, seat, c);
            crate::ownblow::blow_tail(key_is, &f, Some(" — "))
        };
        let close = close_caps(g, seat, c);
        out.push(format!("charge set · {}{}{}", name, tail, close));
    }
    let ops = seat.door_tools.ops.clone();
    for &op in &ops {
        let o = &g.tab.doortools_door_op[op];
        if o.thr.as_ref().is_some_and(|t| t.hold) && o.phase == Phase::Thr {
            let (men, door, told) = (o.men.clone(), o.door, o.thr.as_ref().unwrap().told);
            let who = crate::orders::squad_orders_who_words(g, seat, Some(&men));
            let name = crate::orders::door_words(g, door);
            let goes = if crate::orders::squad_orders_big(g, seat) { String::new() } else { format!(" — {} goes", f) };
            out.push(format!("{} {} · {}{}", who, if told { "holding on your word" } else { "working the threshold" }, name, goes));
        }
    }
    for w in seat.door_tools.words.clone() {
        if g.tab.doortools_word[w].go1.is_nan() {
            let late: Vec<DoorOpId> = ops.iter().copied().filter(|&o| g.tab.doortools_door_op[o].word == Some(w) && g.tab.doortools_door_op[o].hold < 1.0).collect();
            if !late.is_empty() {
                let mut words = Vec::new();
                for o in late {
                    let men = alive(g, &g.tab.doortools_door_op[o].men);
                    words.push(crate::orders::squad_orders_who_words(g, seat, Some(&men)));
                }
                out.push(format!("word given · waiting for {}", uniq_plus(words)));
            }
        }
    }
    out
}

/// What hudNotes and a door's heading say that the corner panel would otherwise lose, each as a key and what a press of it does now.
pub fn door_tools_corner_notes(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<crate::view::Note> {
    use crate::view::Note;
    let mut out: Vec<Note> = Vec::new();
    let f = blow_letter(g);
    let word = crate::orders::squad_orders_big(g, seat) && {
        let men = door_tools_word_men(g, seat);
        !door_tools_waiting_any(g, seat, &men).is_empty()
    };
    for c in seat.door_tools.set.clone() {
        let close = close_caps(g, seat, c);
        let door = g.tab.doortools_set_charge[c].door;
        if !word && !door_tools_on_pending_word(g, seat, c) && door_tools_key_is(g, seat, c) {
            let name = crate::orders::door_words(g, door);
            out.push(Note { key: f.clone(), text: format!("blow the charge · {}{}", name, close) });
        } else if !close.is_empty() {
            let name = crate::orders::door_words(g, door);
            out.push(Note { key: String::new(), text: format!("charge set · {}{}", name, close) });
        }
    }
    if !crate::orders::squad_orders_big(g, seat) {
        for op in seat.door_tools.ops.clone() {
            let o = &g.tab.doortools_door_op[op];
            if o.thr.as_ref().is_some_and(|t| t.hold) && o.phase == Phase::Thr && o.thr.as_ref().unwrap().told {
                let (men, door) = (o.men.clone(), o.door);
                let who = crate::orders::squad_orders_who_words(g, seat, Some(&men));
                let name = crate::orders::door_words(g, door);
                out.push(Note { key: f.clone(), text: format!("go · {} waiting · {}", who, name) });
            }
        }
    }
    out
}

/// The word, while somebody is waiting on it: the key that gives it, to whom; with the orders going to somebody else, who is waiting (a set charge is left out of that: it has its own line, and its own key).
pub fn door_tools_word_note(g: &Game, seat: &crate::seats::SeatRef) -> Vec<crate::view::Note> {
    use crate::view::Note;
    let f = blow_letter(g);
    let word_men = door_tools_word_men(g, seat);
    let all = crate::orders::squad_orders_men(g, seat, None, None);
    if !crate::orders::squad_orders_big(g, seat) {   // (the small squad's: its plain stacks, under wordSmall: the key is always the word's there)
        let mine = door_tools_stack_words(g, seat, &word_men);
        if !mine.is_empty() { return vec![Note { key: f, text: format!("go · {} waiting", mine) }]; }
        let rest = door_tools_stack_words(g, seat, &all);
        return if rest.is_empty() { Vec::new() } else { vec![Note { key: String::new(), text: format!("{} waiting", rest) }] };
    }
    let mine = door_tools_waiting_words(g, seat, &word_men);
    if !mine.is_empty() {
        return vec![Note { key: f, text: format!("go · {} waiting", mine) }];
    }
    let mut words = Vec::new();
    for x in door_tools_waiting_any(g, seat, &all) {
        let of = match &x {
            DoorToolsWaitingAnyItem::Arm1(s) => s.men.clone(),
            DoorToolsWaitingAnyItem::DoorOp(op) => {
                if door_tools_charge_on(g, seat, g.tab.doortools_door_op[*op].door).is_some() { continue; }
                g.tab.doortools_door_op[*op].men.clone()
            }
        };
        let living = alive(g, &of);
        words.push(crate::orders::squad_orders_who_words(g, seat, Some(&living)));
    }
    let rest = uniq_plus(words);
    if rest.is_empty() { Vec::new() } else { vec![Note { key: String::new(), text: format!("{} waiting", rest) }] }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The spots a pocket's throw is tried at. A failure means a room is searched at every spot or at far too few.
    #[test]
    fn the_lattice_is_every_other_cell_each_way_on_both_sides_of_zero() {
        let want = [true, false, false, true, true, true, false];
        let got = [
            on_lattice(0.25, 0.25, 0.5),
            on_lattice(0.75, 0.25, 0.5),
            on_lattice(-0.25, 0.25, 0.5),
            on_lattice(-0.75, -0.75, 0.5),
            on_lattice(1.25, 0.25, 0.5),
            on_lattice(1.25, 1.25, 0.5),
            on_lattice(0.5, 1.5, 1.0),
        ];
        assert_eq!(got, want);
    }

    #[test]
    fn a_set_of_words_keeps_first_sight_order() {
        let w = |s: &str| String::from(s);
        assert_eq!(uniq_plus(vec![w("blue"), w("red"), w("blue"), w("gold")]), "blue + red + gold");
        assert_eq!(uniq_plus(Vec::new()), "");
    }

    /// The file is dealt two at a time: a failure means the last man of an odd file brings the game down.
    #[test]
    fn a_pair_off_the_end_of_a_file_is_whoever_is_left() {
        let v = [1, 2, 3];
        assert_eq!(js_slice(&v, 0, 2), vec![1, 2]);
        assert_eq!(js_slice(&v, 2, 4), vec![3]);
        assert_eq!(js_slice(&v, 4, 6), Vec::<i32>::new());
    }
}
