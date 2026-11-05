//! src/game/sneak.ts lines 414 to 645
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The lines he says when he waits and when the wait drags on (sneak.ts LINES): this part is the only reader.
static LINES_WAIT: &[(Why, &str)] = &[
    (Why::Lit, "holding. it's lit and he's looking."),
    (Why::Near, "holding. too close while he's looking."),
    (Why::Long, "holding. too much open light to cross."),
    (Why::Open, "holding in the dark. you're stood in the light."),
    (Why::Door, "i'm off the door. ready when you are."),
];
static LINES_NAG: &[(Why, &str)] = &[
    (Why::Lit, "can't cross while it's lit. he's still on it."),
    (Why::Near, "he's still facing me. waiting for his back."),
    (Why::Long, "it's a long lit stretch. kill the light or go on."),
    (Why::Open, "you're stood in the light. i'll hold in the dark."),
    (Why::Door, ""),
];
static LINE_GO: &str = "moving.";
/// The lines as they are said now, where one changed (sneak.ts LINES_NOW).
static LINES_NOW_WAIT: &[(Why, &str)] = &[(Why::Open, "holding in the dark. you're standing in the light.")];
static LINES_NOW_NAG: &[(Why, &str)] = &[
    (Why::Near, "he's still facing me. waiting for him to turn."),
    (Why::Open, "you're standing in the light. i'll hold in the dark."),
];

fn line_of(table: &[(Why, &'static str)], why: Why) -> Option<&'static str> {
    table.iter().find(|row| row.0 == why).map(|row| row.1)
}
/// `wg(NOW[why] ?? WAS[why], WAS[why])`
fn said(g: &mut Game, now: &[(Why, &'static str)], was: &[(Why, &'static str)], why: Why) -> &'static str {
    let w = line_of(was, why).unwrap();
    let n = line_of(now, why).unwrap_or(w);
    crate::words::wg(g, &n, &w)
}

fn seam_add_wait_s(g: &mut Game, secs: f64) { todo!("needs: SneakStats.wait_s as f64") }

/// Sam's trail, a crumb every 0.4 m (once a frame, whoever asks first).   (sneak.ts track)
pub fn track(g: &mut crate::game::Game) {
    let now = g.time;
    let c = g.pl[g.player].ch;
    let p = g.ch[c].pos;
    let s = g.m.sneak.states.as_mut().unwrap();
    if s.t == now { return; }
    let dt = now - s.t;
    s.t = now;
    let stray = match s.trail.last() { Some(last) => flat(last.p, p) > 3.0, None => false };
    if dt < 0.0 || dt > 1.0 || stray {
        s.off += s.trail.len() as f64;
        s.trail.clear();
        s.amb = 0.0;
    }
    let chest = g.ch[c].bones.get(crate::character::Bone::Chest).copied().unwrap_or([p[0], 1.2, p[2]]);
    let head = g.ch[c].bones.get(crate::character::Bone::Head).copied().unwrap_or([p[0], 1.6, p[2]]);
    let id = g.ch[c].id;
    let cpu = cpu_lit(g, chest, head, Some(&mut |_g: &mut Game, o: f64| o == id), Some(false));
    let visibility = g.pl[g.player].visibility;
    let max = m::ceil(g.k.sneak.trail_m / 0.4);
    let s = g.m.sneak.states.as_mut().unwrap();
    s.amb = m::min(m::max(0.0, visibility - cpu - 0.05), s.amb + 0.5 * m::max(0.0, m::min(dt, 0.1)));
    let far_enough = match s.trail.last() { Some(last) => flat(last.p, p) >= 0.4, None => true };
    if far_enough {
        s.trail.push(Crumb { p: [p[0], 0.0, p[2]], t: -1e9, lit: 0.0, low: true, amb: s.amb, cpu0: cpu });
    }
    let len = s.trail.len() as f64;
    if len > max {
        // never cut from under a man on it: every man followed inside five seconds (S.list; see the notes)
        let mut keep = len;
        for man in s.men.values() {
            if now - man.t < 5.0 && now >= man.t { keep = m::min(keep, man.i - s.off - 2.0); }
        }
        let cut = m::max(len - 4.0 * max, m::min(len - max, keep));
        if cut > 0.0 {
            s.trail.drain(0..cut as usize);
            s.off += cut;
        }
    }
}

/// `sneakTick` without the copy of the state its signature hands back. False: not sneaking.
fn tick(g: &mut Game) -> bool {
    if !g.k.sneak.on { return false; }
    if !sneak_live(g) {
        if let Some(s) = g.m.sneak.states.as_mut() {
            if s.live {
                s.live = false;
                s.trail.clear();
                s.men = JsMap::new();
                s.amb = 0.0;
            }
        }
        return false;
    }
    let s = g.m.sneak.states.get_or_insert_with(|| State { trail: Vec::new(), off: 0.0, t: -1e9, men: JsMap::new(), list: Vec::new(), live: true, amb: 0.0 });
    s.live = true;
    track(g);
    true
}

/// Once a tick with a squad under orders: Sam's trail is kept while the squad sneaks. None: not sneaking.   (sneak.ts sneakTick)
pub fn sneak_tick(g: &mut crate::game::Game) -> Option<crate::sneak::State> {
    if !tick(g) { return None; }
    g.m.sneak.states.clone()
}

fn low_pace(g: &Game, v: f64, is_low: bool) -> f64 {
    if is_low { m::min(v, g.k.sneak.low_pace) } else { v }
}
fn sam_pos(g: &Game) -> V3 {
    g.ch[g.pl[g.player].ch].pos
}
/// A crumb's index on the trail now, from its absolute number; below 0 when it has been cut.
fn idx(k: f64, off: f64) -> i64 {
    (k - off) as i64
}
fn straight(g: &mut Game, p: V3, q: V3) -> bool {
    !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [q[0], 0.45, q[2]], None)
}
/// His posture in a mode: low with Sam; low whenever he waits, crosses or backs off, and after a back-off until he has moved on.
fn crouch_out(man: &Man, low: bool, mode: Mode) -> bool {
    low || mode == Mode::Wait || mode == Mode::Cross || mode == Mode::Back || man.hid_i >= 0.0
}

/// What the read of the way has in hand: the closures of sneakFollow's plan captured these.
struct Way {
    p: V3,
    i: i64,
    place: i64,
    eyes: Vec<GuardId>,
    others: Vec<GuardId>,
    sam_doors: Vec<DoorId>,
    open_doors: Vec<DoorId>,
    keen_f: f64,
    card_f: f64,
    memo: JsMap<i64, Exp>,
}

/// `ex`: a crumb's exposure, read at most once a plan for each posture.
fn ex(g: &mut Game, w: &mut Way, t: &mut [Crumb], k: i64, lo: bool) -> Exp {
    let key = k * 2 + if lo { 1 } else { 0 };
    if let Some(v) = w.memo.get(key) { return *v; }
    let v = exposure(g, &w.eyes, &mut t[k as usize], lo, Some(w.card_f));
    w.memo.set(key, v);
    v
}
fn near_other(g: &Game, w: &Way, q: V3) -> bool {
    w.others.iter().any(|o| flat(g.ch[g.gd[*o].ch].pos, q) < 0.7)
}
/// The sign of the turn from p to q to r on the floor (funnel's `o`).
fn turn(px: f64, pz: f64, qx: f64, qz: f64, rx: f64, rz: f64) -> f64 {
    (qx - px) * (rz - pz) - (qz - pz) * (rx - px)
}
/// In a door's funnel: some defender's eye, inside its reach, that would gain on a man at `q`, with the flat line to `q` through the frame's opening.
fn funnel(g: &mut Game, w: &Way, q: V3, doors: &[DoorId], pad: f64, lit: &mut dyn FnMut(&mut Game) -> f64, lo: bool) -> bool {
    for &h in w.eyes.iter() {
        let e = g.ch[g.gd[h].ch].pos;
        let ed = flat(e, q);
        if ed > g.k.guard_eye.range { continue; }
        if ed >= g.k.sneak.near_m {
            let l = lit(g);
            if !eye_gains(g, h, l, ed, w.keen_f, lo, Some(w.card_f), None) { continue; }
        }
        for &d in doors {
            let (closed_dir, hinge, width) = { let def = &g.dr[d].def; (def.closed_dir, def.hinge, def.width) };
            let cx = m::cos(closed_dir);
            let cz = m::sin(closed_dir);
            let ax = hinge[0] - cx * pad;
            let az = hinge[1] - cz * pad;
            let bx = ax + cx * (width + 2.0 * pad);
            let bz = az + cz * (width + 2.0 * pad);
            if turn(e[0], e[2], q[0], q[2], ax, az) * turn(e[0], e[2], q[0], q[2], bx, bz) <= 0.0
                && turn(ax, az, bx, bz, e[0], e[2]) * turn(ax, az, bx, bz, q[0], q[2]) <= 0.0
            {
                return true;
            }
        }
    }
    false
}
/// Crumb k is lit and in the line of a shut door Sam is about to open.
fn in_line(g: &mut Game, w: &Way, t: &mut [Crumb], k: i64, lo: bool) -> bool {
    if w.sam_doors.is_empty() { return false; }
    let k = k as usize;
    let q = t[k].p;
    let mut l = -1.0;
    let mut lit = |g: &mut Game| {
        if l < 0.0 { l = lit_at(g, &mut t[k], lo); }
        l
    };
    let pad = g.k.sneak.door_pad;
    if !funnel(g, w, q, &w.sam_doors, pad, &mut lit, lo) { return false; }
    let v = lit(g);
    v >= g.k.sneak.lit_at
}
/// A spot's light to a man on it: the CPU's estimate plus the extra Sam's meter read on the crumb he is at.
fn lit_spot(g: &mut Game, c: Crumb, q: V3, lo: bool) -> f64 {
    let y = if lo { low_y() } else { up_y() };
    let cpu = cpu_lit(g, [q[0], y[0], q[2]], [q[0], y[1], q[2]], None, None);
    m::min(1.0, cpu + c.amb * (if c.cpu0 > 0.03 { m::min(1.0, cpu / c.cpu0) } else { 1.0 }))
}
fn unseen_at(g: &mut Game, w: &Way, t: &[Crumb], q: V3, lo: bool) -> bool {
    let c = t[w.i as usize];
    let mut l = -1.0;
    let mut lit = |g: &mut Game| {
        if l < 0.0 { l = lit_spot(g, c, q, lo); }
        l
    };
    let pc = stand_in(q, lo);
    if read(g, &w.eyes, pc, &mut lit, Some(lo), Some(w.card_f)).seen { return false; }
    if !funnel(g, w, q, &w.open_doors, 0.0, &mut lit, lo) { return true; }
    let v = lit(g);
    !(v >= g.k.sneak.lit_at)
}
/// A spot off the trail he may take (`deep`: unseen with that many metres to spare towards the trail) or hold (0).
fn side_ok(g: &mut Game, w: &Way, t: &[Crumb], q: V3, lo: bool, deep: f64) -> bool {
    let p = w.p;
    if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { return false; }
    if flat(q, sam_pos(g)) < g.k.sneak.stand_near + 0.3 { return false; }
    if near_other(g, w, q) { return false; }
    if g.doors.list.iter().any(|d| flat(q, g.dr[*d].frame_centre) < 1.0) { return false; }
    if flat(p, q) > 0.3 && (!straight(g, p, q) || !crate::scene_collision::nav_grid_walkable(g, p, q)) { return false; }
    let c = t[w.i as usize].p;
    if flat(q, c) > 2.7 || !unseen_at(g, w, t, q, lo) { return false; }
    let side_stop = g.k.sneak.side_stop;
    if side_stop > 0.0 {
        let dp = flat(p, q);
        if dp > 0.3 {
            for s in [side_stop / 2.0, side_stop] {
                if !unseen_at(g, w, t, [q[0] + (p[0] - q[0]) * s / dp, 0.0, q[2] + (p[2] - q[2]) * s / dp], lo) { return false; }
            }
        }
    }
    let d = flat(q, c);
    deep <= 0.0 || d <= deep || unseen_at(g, w, t, [q[0] + (c[0] - q[0]) * deep / d, 0.0, q[2] + (c[2] - q[2]) * deep / d], lo)
}
/// A spot off the trail he can step straight to when no step on it will do. `caught`: seen where he stands, so the spot he is on is no answer.
fn aside(g: &mut Game, w: &Way, t: &[Crumb], man: &mut Man, lo: bool, caught: bool) -> Option<V3> {
    let side_m = g.k.sneak.side_m;
    if side_m <= 0.0 { return None; }
    let p = w.p;
    let now = g.time;
    let at_crumb = t[w.i as usize].p;
    if let Some(ms) = man.side {
        if !(caught && flat(p, ms) < 0.3) && flat(ms, at_crumb) < side_m + 1.0 && side_ok(g, w, t, ms, lo, 0.0) { return Some(ms); }
    }
    if now - man.side_t < 1.0 && now >= man.side_t { return None; }
    let mut best: Option<V3> = None;
    let mut bd = 1e9;
    let dc: Option<V3> = w.sam_doors.first().map(|d| g.dr[*d].frame_centre);
    let d0 = match dc { Some(c) => flat(p, c), None => 0.0 };
    for deep in [0.55, 0.3] {
        if best.is_some() && dc.is_none() { break; }
        let mut r = 0.6;
        while r <= side_m + 1e-6 {
            for a in 0..12 {
                let a = a as f64;
                let q: V3 = [p[0] + r * m::sin(a * m::PI / 6.0), 0.0, p[2] + r * m::cos(a * m::PI / 6.0)];
                let d = r + 0.3 * flat(q, at_crumb) + (match dc {
                    Some(c) => (if deep < 0.5 { 0.8 } else { 0.0 }) + 0.6 * m::max(0.0, flat(q, c) - d0),
                    None => 0.0,
                });
                if d < bd && side_ok(g, w, t, q, lo, deep) {
                    bd = d;
                    best = Some(q);
                }
            }
            r += 0.4;
        }
    }
    if best.is_none() { man.side_t = now; }
    best
}
/// Is crumb k a step he may take in `dark`'s search?
fn dark_ok(g: &mut Game, w: &mut Way, t: &mut [Crumb], k: i64, own: bool, lo: bool, strict: bool) -> bool {
    let p = w.p;
    let c = t[k as usize].p;
    if own && flat(p, c) < 0.3 { return false; }
    if ex(g, w, t, k, lo).seen { return false; }
    if near_other(g, w, c) { return false; }
    if in_line(g, w, t, k, lo) { return false; }
    if !strict { return true; }
    if !unseen_at(g, w, t, c, lo) { return false; }
    // shy: where he would be seen coming, or stop short
    let d = flat(p, c);
    !(d > 0.4 && !unseen_at(g, w, t, [c[0] + (p[0] - c[0]) * 0.35 / d, 0.0, c[2] + (p[2] - c[2]) * 0.35 / d], lo))
}
/// The nearest step from crumb `at` that is not exposed: on towards Sam or back the way they came, within `standM`; -1: none.
fn dark(g: &mut Game, w: &mut Way, t: &mut [Crumb], at: i64, own: bool, lo: bool, strict: bool) -> i64 {
    let len = t.len() as i64;
    let (stand_m, stand_near) = (g.k.sneak.stand_m, g.k.sneak.stand_near);
    let mut fwd = -1;
    let mut fd = 0.0;
    let mut bwd = -1;
    let mut bd = 0.0;
    let mut k = at + 1;
    while k < len && fd < stand_m && (k <= w.place || flat(t[k as usize].p, sam_pos(g)) >= stand_near) {
        fd += flat(t[k as usize].p, t[(k - 1) as usize].p);
        if dark_ok(g, w, t, k, own, lo, strict) {
            fwd = k;
            break;
        }
        k += 1;
    }
    let mut k = at - 1;
    while k >= 0 && bd < stand_m {
        bd += flat(t[k as usize].p, t[(k + 1) as usize].p);
        if dark_ok(g, w, t, k, own, lo, strict) {
            bwd = k;
            break;
        }
        k -= 1;
    }
    if fwd >= 0 && (bwd < 0 || fd <= bd) { fwd } else { bwd }
}

/// The read of the way, a few times a second: his mode, his dark step or spot, and the words on a change (sneakFollow's plan).
fn plan(g: &mut Game, s: &mut State, man: &mut Man, gd: GuardId, p: V3, i: i64, place: i64, low: bool, rem: f64, gap: f64, stopped: bool) {
    let now = g.time;
    let k_ = g.k.sneak;
    man.plan_t = now;
    let eyes = if k_.skill > 0.0 { eyes_for(g, gd) } else { Vec::new() };
    let low_walk = crouch_out(man, low, Mode::Walk);
    let keen_f = if crate::guards::escalation_of(g) >= 1.0 { crate::guards::KEEN } else { 1.0 };
    let card_f = crate::kitload::load_seen(g, crate::kitload::LoadSeenWho::Guard(gd));
    let here = if !eyes.is_empty() { exposure_of(g, &eyes, gd) } else { none() };
    let off = s.off;
    let t = &mut s.trail;
    let len = t.len() as i64;
    if man.mode == Mode::Cross {
        let e = idx(man.cross_to, off);
        if e < 0 || e >= len || i > e || flat(p, t[e as usize].p) < 0.3 || now - man.cross_t > k_.cross_secs || now < man.cross_t {
            man.mode = Mode::Walk;
            man.cross_from = -1.0;
            man.cross_to = -1.0;
        }
    }
    if man.mode == Mode::Back {
        let e = idx(man.stand, off);
        let there = match man.side {
            Some(ms) => flat(p, ms) < 0.3,
            None => e < 0 || e >= len || flat(p, t[e as usize].p) < 0.3,
        };
        if there || now - man.back_t > 4.0 || now < man.back_t {
            man.mode = Mode::Walk;
            man.hide_t = now;
            man.hid_i = man.i;
        }
    }
    if man.mode == Mode::Back || (man.mode == Mode::Cross && i >= idx(man.cross_from, off) - 1) { return; }

    let mut mode = Mode::Walk;
    let mut stand: i64 = -1;
    let mut side: Option<V3> = None;
    let mut why = man.why;
    let mut from: i64 = -1;
    let mut to: i64 = -1;
    let others: Vec<GuardId> = crate::orders::squad_orders_men(g, None, None).into_iter().filter(|o| *o != gd).collect();
    let sam = sam_pos(g);
    let list = g.doors.list.clone();
    let mut sam_doors: Vec<DoorId> = Vec::new();
    if k_.door_m > 0.0 {
        for &d in list.iter() {
            if !(flat(sam, g.dr[d].frame_centre) < k_.door_m) { continue; }
            let angle = crate::doors::door_angle(g, d);
            if leaf_moved(g, LeafMovedD { angle }) || m::abs(crate::doors::door_angle(g, d)) <= 0.03 { sam_doors.push(d); }
        }
    }
    let mut open_doors: Vec<DoorId> = Vec::new();
    for &d in list.iter() {
        if m::abs(crate::doors::door_angle(g, d)) > 0.03 { open_doors.push(d); }
    }
    open_doors.extend(sam_doors.iter().copied());
    let mut w = Way { p, i, place, eyes, others, sam_doors, open_doors, keen_f, card_f, memo: JsMap::new() };
    let mut coming: Vec<GuardId> = Vec::new();
    for &h in w.eyes.iter() {
        if sneak_coming(g, gd, h) { coming.push(h); }
    }
    let on_walk = !coming.is_empty() && (rem > gap + 0.5 || !stopped);
    let else_eyes: Vec<GuardId> = if on_walk { w.eyes.iter().copied().filter(|h| !coming.contains(h)).collect() } else { Vec::new() };

    if !w.eyes.is_empty() {
        if rem > gap + 0.5 || !stopped {
            // walking: the next stretch of the trail, read a step at a time for the posture he walks in
            let reach = k_.ahead_m * m::max(0.25, k_.skill);
            let mut a = 0.0;
            let mut first: i64 = -1;
            let k0 = if flat(p, t[i as usize].p) > 0.45 { i } else { i + 1 };
            let mut k = k0;
            while k <= place && a < reach {
                a += if k == k0 { flat(p, t[k as usize].p) } else { flat(t[(k - 1) as usize].p, t[k as usize].p) };
                if ex(g, &mut w, t, k, low_walk).seen || in_line(g, &w, t, k, low_walk) {
                    first = k;
                    break;
                }
                k += 1;
            }
            if first >= 0 {
                // the stretch: on to the first step past it where he can walk as he walks, and whether anybody looks at a crouched man on it
                let mut end: i64 = -1;
                let mut stretch = 0.0;
                let mut watched = false;
                let mut lit = false;
                let mut by: Option<GuardId> = None;
                let mut last = first;
                let mut k = first;
                while k < len && k < first + 60 {
                    if k > place && flat(t[k as usize].p, sam_pos(g)) < k_.stand_near { break; }
                    if !ex(g, &mut w, t, k, low_walk).seen && !in_line(g, &w, t, k, low_walk) {
                        end = k;
                        break;
                    }
                    let x = ex(g, &mut w, t, k, true);
                    watched = watched || x.watched;
                    by = by.or(x.by);
                    last = k;
                    lit = lit || (x.seen && !x.near);
                    if k > first { stretch += flat(t[(k - 1) as usize].p, t[k as usize].p); }
                    k += 1;
                }
                let too_long = stretch > k_.cross_m;
                let waited = if man.mode == Mode::Wait && man.wait_t >= 0.0 { now - man.wait_t } else { 0.0 };
                let cheap = end >= 0 && watched && !too_long && {
                    let card = crate::kitload::load_cross(g, crate::kitload::LoadCrossWho::Guard(gd));
                    cross_cost(g, &w.eyes, t, first as f64, end as f64, Some(card)) <= k_.budget
                };
                let walked = on_walk
                    && ex(g, &mut w, t, first, low_walk).seen
                    && by.map_or(true, |b| coming.contains(&b))
                    && !others_on(g, &else_eyes, t, first, last, card_f);
                if end >= 0 && (walked || cheap || (!watched && (!too_long || (k_.patience > 0.0 && waited >= k_.patience)))) {
                    mode = Mode::Cross;
                    from = first;
                    to = end;
                } else if !walked {
                    mode = Mode::Wait;
                    why = if watched {
                        if lit { Why::Lit } else { Why::Near }
                    } else if !ex(g, &mut w, t, first, low_walk).seen {
                        Why::Door
                    } else if end < 0 {
                        Why::Open
                    } else {
                        Why::Long
                    };
                }
                man.mem = match by {
                    Some(h) if mode == Mode::Wait && watched && lit => Some(ManMem { h, a: first as f64 + off, b: last as f64 + off, t: now }),
                    _ => None,
                };
            } else if let Some(mm) = man.mem.filter(|mm| !(on_walk && coming.contains(&mm.h))) {
                // nothing reads exposed ahead, but a moment ago a man was looking at that lit stretch and he cannot see that man now
                let a = m::max(0.0, mm.a - off) as i64;
                let b = m::min((len - 1) as f64, mm.b - off) as i64;
                let hc = g.gd[mm.h].ch;
                let mut lit = false;
                let mut k = a;
                while k <= b && !lit {
                    lit = lit_at(g, &mut t[k as usize], true) >= k_.lit_at;
                    k += 1;
                }
                let own_c = g.gd[gd].ch;
                let eye = g.ch[own_c].bones.get(crate::character::Bone::Head).copied().unwrap_or([p[0], 1.0, p[2]]);
                let hp = g.ch[hc].pos;
                let his = g.ch[hc].bones.get(crate::character::Bone::Head).copied().unwrap_or([hp[0], 1.65, hp[2]]);
                let sees = !crate::scene_collision::segment_blocked(g, eye, his, None, None, None);
                if now - mm.t > k_.memory || now < mm.t || g.gd[mm.h].state == crate::game::GuardState::Dead || idx(mm.b, off) < 0 || i >= a || sees || !lit {
                    man.mem = None;
                } else {
                    mode = Mode::Wait;
                    why = Why::Lit;
                }
            }
            // stood IN the line of a shut door Sam is at: out of it first, and then he waits there
            if !w.sam_doors.is_empty() && mode != Mode::Cross {
                let in_funnel = {
                    let tt = &mut *t;
                    let pad = k_.door_pad;
                    funnel(g, &w, p, &w.sam_doors, pad, &mut |g: &mut Game| lit_at(g, &mut tt[i as usize], low_walk), low_walk)
                };
                if in_funnel && lit_at(g, &mut t[i as usize], low_walk) >= k_.lit_at {
                    let k = dark(g, &mut w, t, i, true, low_walk, false);
                    if k >= 0 {
                        stand = k;
                        mode = Mode::Stand;
                    } else {
                        side = aside(g, &w, t, man, low_walk, false);
                        if side.is_some() { mode = Mode::Stand; }
                    }
                }
            }
        } else {
            let at = m::min(place as f64, (len - 1) as f64) as i64;
            if ex(g, &mut w, t, at, low_walk).seen || in_line(g, &w, t, at, low_walk) {
                // Sam stopped and he is at his place: if that is lit with a line, the nearest step along the trail that is not
                let k = dark(g, &mut w, t, at, false, low_walk, false);
                if k >= 0 {
                    stand = k;
                    mode = Mode::Stand;
                } else if flat(p, t[at as usize].p) < k_.side_m + 0.5 {
                    side = aside(g, &w, t, man, low_walk, false);
                    if side.is_some() { mode = Mode::Stand; }
                }
            }
        }
        // CAUGHT: he is seen where he stands: the nearest dark step, low. Not twice inside a few seconds.
        if mode != Mode::Stand
            && !(on_walk
                && here.by.map_or(true, |b| coming.contains(&b))
                && !(!else_eyes.is_empty() && exposure_of(g, &else_eyes, gd).watched))
            && (here.watched || (here.seen && (mode == Mode::Wait || stopped)))
            && (now - man.hide_t > k_.hide_gap || now < man.hide_t)
        {
            let k = dark(g, &mut w, t, i, true, true, true);
            if k >= 0 {
                stand = k;
                mode = Mode::Back;
            } else {
                side = aside(g, &w, t, man, true, true);
                if side.is_some() { mode = Mode::Back; }
            }
        }
        // stood aside and not seen there: he keeps the spot while he waits, or while Sam stands
        if side.is_none() {
            if let Some(ms) = man.side {
                if (mode == Mode::Wait || (mode == Mode::Walk && stopped && rem <= gap + k_.side_m + 0.5)) && flat(ms, t[i as usize].p) < k_.side_m + 1.0 {
                    let lo = crouch_out(man, low, mode);
                    if side_ok(g, &w, t, ms, lo, 0.0) { side = Some(ms); }
                }
            }
        }
    }
    man.stand = if stand >= 0 { stand as f64 + off } else { -1.0 };
    man.side = side;
    if mode == Mode::Cross {
        man.cross_from = from as f64 + off;
        man.cross_to = to as f64 + off;
    }
    // the words, and the tallies, on the change
    if mode != man.mode {
        if man.mode == Mode::Wait {
            seam_add_wait_s(g, now - man.wait_t);
            if mode == Mode::Cross && now - man.wait_t > 1.5 { cue(g, gd, man, LINE_GO, None); }
            man.wait_t = -1.0;
        }
        if mode == Mode::Wait {
            man.why = why;
            man.wait_t = now;
            man.nagged = false;
            g.stats.sneak.waits += 1;
            note(g, gd, &format!("waits ({})", why.as_str()));
            let line = said(g, LINES_NOW_WAIT, LINES_WAIT, why);
            cue(g, gd, man, line, None);
        }
        if mode == Mode::Cross {
            man.cross_t = now;
            g.stats.sneak.crossings += 1;
            note(g, gd, "crosses");
        }
        if mode == Mode::Stand {
            g.stats.sneak.shade += 1;
            if side.is_some() { g.stats.sneak.asides += 1; }
            note(g, gd, if side.is_some() { "steps aside, out of the line" } else { "stands back in the dark" });
        }
        if mode == Mode::Back {
            g.stats.sneak.back_offs += 1;
            if side.is_some() { g.stats.sneak.asides += 1; }
            man.back_t = now;
            note(g, gd, if side.is_some() { "backs off, aside" } else { "backs off" });
        }
        man.mode = mode;
    } else if mode == Mode::Wait {
        man.why = why;
    }
    if man.mode == Mode::Wait && !man.nagged && now - man.wait_t >= k_.nag_secs && !(now >= man.voice_t && now - man.voice_t < 3.0) {
        let line = said(g, LINES_NOW_NAG, LINES_NAG, man.why);
        man.nagged = cue(g, gd, man, line, Some(true));
    }
}

/// `othersOn`: another man's eyes on the stretch, asked on their own.
fn others_on(g: &mut Game, else_eyes: &[GuardId], t: &mut [Crumb], a: i64, b: i64, card: f64) -> bool {
    if else_eyes.is_empty() { return false; }
    let mut k = a;
    while k <= b {
        if exposure(g, else_eyes, &mut t[k as usize], true, Some(card)).watched { return true; }
        k += 1;
    }
    false
}

/// `lay`: the way to crumb `to`, laid crumb by crumb for sneakWay; the goal.
fn lay(g: &mut Game, man: &mut Man, t: &[Crumb], p: V3, i: i64, to: i64) -> V3 {
    let goal = t[to as usize].p;
    let mut path: Vec<V3> = Vec::new();
    if to >= i {
        let mut k = if flat(p, t[i as usize].p) > 0.45 { i } else { i + 1 };
        while k <= to {
            let c = t[k as usize].p;
            path.push([c[0], 0.0, c[2]]);
            k += 1;
        }
    } else {
        let mut k = i - 1;
        while k >= to {
            let c = t[k as usize].p;
            path.push([c[0], 0.0, c[2]]);
            k -= 1;
        }
    }
    man.way = if !path.is_empty() && straight(g, p, path[0]) { Some(path) } else { None };
    goal
}

/// sneakFollow from the point his row and the state are in hand (both out of `g`).
fn follow(g: &mut Game, s: &mut State, man: &mut Man, gd: GuardId, back: f64, sam_v: f64, stopped: bool, speed0: f64) -> Feet {
    let now = g.time;
    let k_ = g.k.sneak;
    let p = g.ch[g.gd[gd].ch].pos;
    let low = g.pl[g.player].crouch;
    man.t = now;
    man.way = None;
    let plain = Feet { goal: None, speed: low_pace(g, speed0, low), crouch: low, must: None };
    if s.trail.len() < 3 { return plain; }
    let off = s.off;
    let len = s.trail.len() as i64;
    // where he is on the trail. Lost: the nearest crumb he can walk straight to, looked for twice a second
    let mut i = idx(man.i, off);
    if i < 0 || i >= len || flat(p, s.trail[i as usize].p) > 3.0 {
        if now - man.lost_t < 0.5 && now >= man.lost_t { return plain; }
        i = -1;
        let mut bd = 1e9;
        let mut k = len - 1;
        while k >= 0 {
            let c = s.trail[k as usize].p;
            let d = flat(p, c);
            if d < bd - 0.05 && (d < 0.8 || crate::scene_collision::nav_grid_walkable(g, p, c)) {
                bd = d;
                i = k;
            }
            k -= 1;
        }
        if i < 0 {
            man.lost_t = now;
            return plain;
        }
        man.mode = Mode::Walk;
        man.cross_from = -1.0;
        man.cross_to = -1.0;
        man.stand = -1.0;
        man.hid_i = -1.0;
        man.mem = None;
        man.side = None;
    }
    // on: the furthest crumb he is standing on, or the later pass of it a stride away where the trail doubles back
    let mut k = len - 1;
    while k > i {
        let c = s.trail[k as usize].p;
        let d = flat(p, c);
        if d < 0.5 || (d < 0.9 && k - i > 12 && straight(g, p, c)) {
            i = k;
            break;
        }
        k -= 1;
    }
    // back: walking to a step behind him, his place on the trail goes back with him
    if man.mode == Mode::Stand || man.mode == Mode::Back {
        let st = idx(man.stand, off);
        if st >= 0 && st < i {
            let mut k = st;
            while k < i {
                if flat(p, s.trail[k as usize].p) < 0.45 {
                    i = k;
                    break;
                }
                k += 1;
            }
        }
    }
    man.i = i as f64 + off;
    if man.hid_i >= 0.0 && m::abs(man.i - man.hid_i) >= 3.0 { man.hid_i = -1.0; }
    // the trail from him to Sam, and his place on it
    let sam = sam_pos(g);
    let gap = back + k_.back;
    let mut rem = 0.0;
    let mut k = i;
    while k < len - 1 {
        rem += flat(s.trail[k as usize].p, s.trail[(k + 1) as usize].p);
        k += 1;
    }
    rem += flat(s.trail[(len - 1) as usize].p, sam);
    let mut place = len - 1;
    {
        let mut a = flat(s.trail[(len - 1) as usize].p, sam);
        while place > 0 && a < gap {
            a += flat(s.trail[place as usize].p, s.trail[(place - 1) as usize].p);
            place -= 1;
        }
    }
    if place < i { place = i; }
    // the read of the way, a few times a second (the plan stands in between)
    if now - man.plan_t >= k_.plan_secs || now < man.plan_t {
        plan(g, s, man, gd, p, i, place, low, rem, gap, stopped);
    }
    // his feet, by the plan: the way laid crumb by crumb
    let t = &s.trail;
    let low_now = crouch_out(man, low, man.mode);
    if let (Some(ms), true) = (man.side, man.mode != Mode::Cross) {
        let speed = if flat(p, ms) < 0.25 {
            0.0
        } else if man.mode == Mode::Back {
            m::min(k_.low_pace, 1.2)
        } else {
            low_pace(g, 1.0, low_now)
        };
        return Feet { goal: Some(ms), speed, crouch: low_now, must: Some(true) };
    }
    if man.mode == Mode::Wait { return Feet { goal: None, speed: 0.0, crouch: low_now, must: None }; }
    if man.mode == Mode::Stand || man.mode == Mode::Back {
        let st = idx(man.stand, off);
        if st >= 0 && st < len {
            let c = t[st as usize].p;
            let goal = if flat(p, c) < 0.3 { c } else { lay(g, man, t, p, i, st) };
            let speed = if man.mode == Mode::Back { m::min(k_.low_pace, 1.2) } else { low_pace(g, 1.0, low_now) };
            return Feet { goal: Some(goal), speed, crouch: low_now, must: Some(true) };
        }
        if man.mode == Mode::Back { return Feet { goal: None, speed: 0.0, crouch: low_now, must: None }; }
    }
    // walking Sam's steps: a goal a few crumbs up the trail (never past his place), at Sam's pace plus what closes the gap
    let to_place = m::max(0.0, rem - gap);
    if to_place < 0.25 && man.mode != Mode::Cross {
        let c = t[place as usize].p;
        let goal = if place > i { lay(g, man, t, p, i, place) } else { c };
        let speed = if stopped && flat(p, c) < 0.3 { 0.0 } else { low_pace(g, m::max(k_.min_pace, sam_v), low_now) };
        return Feet { goal: Some(goal), speed, crouch: low_now, must: None };
    }
    let mut j = i;
    let mut a = 0.0;
    let lim = if man.mode == Mode::Cross { m::max(place as f64, m::min((len - 1) as f64, man.cross_to - off)) as i64 } else { place };
    while j < lim && a < 2.6 {
        a += flat(t[j as usize].p, t[(j + 1) as usize].p);
        j += 1;
    }
    let v = if man.mode == Mode::Cross { k_.cross_pace } else { m::max(k_.min_pace, m::min(2.2, sam_v + 1.5 * (to_place - 0.3))) };
    let goal = if j > i { lay(g, man, t, p, i, j) } else { t[j as usize].p };
    Feet { goal: Some(goal), speed: low_pace(g, v, low_now && man.mode != Mode::Cross), crouch: low_now, must: None }
}

/// The sneaking follower's feet this frame. None: he is the build's own follower.   (sneak.ts sneakFollow)
pub fn sneak_follow(g: &mut crate::game::Game, gd: GuardId, back: f64, sam_v: f64, stopped: bool, speed0: f64) -> Option<crate::sneak::Feet> {
    if !g.k.sneak.on || !mate(g, gd) { return None; }
    if !tick(g) { return None; }
    let now = g.time;
    // the state and his row are out of `g` for the frame's read: nothing the read calls reads them
    let mut s = g.m.sneak.states.take().unwrap();
    if !s.men.has(gd) {
        s.men.set(gd, Man {
            i: -1.0,
            t: now,
            lost_t: -1e9,
            mode: Mode::Walk,
            plan_t: -1e9,
            wait_t: -1.0,
            why: Why::Lit,
            nagged: false,
            cue_t: -1e9,
            voice_t: -1e9,
            back_t: -1.0,
            hide_t: -1e9,
            hid_i: -1.0,
            mem: None,
            cross_from: -1.0,
            cross_to: -1.0,
            cross_t: -1.0,
            stand: -1.0,
            side: None,
            side_t: -1e9,
            way: None,
        });
    }
    let mut man = core::mem::take(s.men.get_mut(gd).unwrap());
    let feet = follow(g, &mut s, &mut man, gd, back, sam_v, stopped, speed0);
    *s.men.get_mut(gd).unwrap() = man;
    g.m.sneak.states = Some(s);
    Some(feet)
}

/// Right after the order is written: the way to this frame's goal is Sam's own crumbs, not the nav grid's cells.   (sneak.ts sneakWay)
pub fn sneak_way(g: &mut crate::game::Game, gd: GuardId, goal: V3) {
    if !g.k.sneak.on { return; }
    let has_way = match g.m.sneak.states.as_ref().and_then(|s| s.men.get(gd)) {
        Some(man) => man.way.is_some(),
        None => false,
    };
    if !has_way { return; }
    if let Some(pg) = g.gd[gd].path_goal {
        if flat(pg, goal) < 0.05 && g.gd[gd].path_i < g.gd[gd].path.len() { return; }
    }
    let way = g.m.sneak.states.as_mut().unwrap().men.get_mut(gd).unwrap().way.take().unwrap();
    g.gd[gd].path = way;
    g.gd[gd].path_i = 0;
    g.gd[gd].path_goal = Some([goal[0], 0.0, goal[2]]);
    g.gd[gd].repath_t = 0.0;
}

/// His words over the net. False only when ONE VOICE kept it back (the line is still owed).   (sneak.ts cue)
pub fn cue(g: &mut crate::game::Game, gd: GuardId, m: &mut crate::sneak::Man, line: &str, force: Option<bool>) -> bool {
    let force = force.unwrap_or(false);
    if line.is_empty() || (!force && g.time - m.cue_t < g.k.sneak.cue_gap && g.time >= m.cue_t) { return true; }
    if g.k.spotted.on && crate::spotted::spotted_mode(g, gd) != crate::spotted::SpottedMode::Calm {
        m.voice_t = g.time;
        return false;
    }
    m.cue_t = g.time;
    g.stats.sneak.cues += 1;
    crate::game::game_say(g, gd, line, None, None, None);
    true
}

/// The panel's line.   (sneak.ts sneakSummary)
pub fn sneak_summary(g: &mut crate::game::Game) -> String {
    if !g.k.sneak.on { return String::from("off"); }
    if !sneak_live(g) { return String::from("on · not live (the squad is loud, found out, or a defender is hunting)"); }
    let men = crate::orders::squad_orders_men(g, None, None);
    let mut rows: Vec<String> = Vec::new();
    for gd in men {
        let mode = match g.m.sneak.states.as_ref().and_then(|s| s.men.get(gd)) {
            Some(man) => man.mode.as_str(),
            None => "not following",
        };
        rows.push(format!("{}: {}", g.gd[gd].callsign, mode));
    }
    let squad = if rows.is_empty() { String::from("no squad") } else { rows.join(" · ") };
    let trail = match g.m.sneak.states.as_ref() { Some(s) => s.trail.len() as f64 * 0.4, None => 0.0 };
    let st = &g.stats.sneak;
    format!("on · {} · trail {} m · waits {} crossings {} shade {} back-offs {}", squad, m::round(trail) as i64, st.waits, st.crossings, st.shade, st.back_offs)
}
