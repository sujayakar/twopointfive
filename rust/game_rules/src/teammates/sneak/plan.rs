use crate::v3::dist_xz;
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A wait's first line by why.
fn said_wait(why: Why) -> String {
    let l = lines().wait;
    match why {
        Why::Lit => l.lit,
        Why::Near => l.near,
        Why::Long => l.long,
        Why::Open => l.open,
        Why::Door => l.door,
    }
}
/// A wait's second line by why.
fn said_nag(why: Why) -> String {
    let l = lines().nag;
    match why {
        Why::Lit => l.lit,
        Why::Near => l.near,
        Why::Long => l.long,
        Why::Open => l.open,
        Why::Door => l.door,
    }
}

/// Sam's trail, a crumb every 0.4 m (once a frame, whoever asks first).
fn track(g: &mut crate::game::Game) {
    let now = g.time;
    let c = g.pl[g.player].ch;
    let p = g.ch[c].pos;
    let s = g.m.sneak.states.as_mut().unwrap();
    if s.t == now { return; }
    let dt = now - s.t;
    s.t = now;
    let stray = match s.trail.last() { Some(last) => dist_xz(last.p, p) > 3.0, None => false };
    if dt < 0.0 || dt > 1.0 || stray {
        s.off += s.trail.len() as f64;
        s.trail.clear();
    }
    let max = m::ceil(g.k.sneak.trail_m / 0.4);
    let s = g.m.sneak.states.as_mut().unwrap();
    let far_enough = match s.trail.last() { Some(last) => dist_xz(last.p, p) >= 0.4, None => true };
    if far_enough {
        s.trail.push(Crumb { p: [p[0], 0.0, p[2]], t: -1e9, lit: 0.0, low: true });
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

/// Once a tick with a squad under orders: Sam's trail is kept while the squad sneaks. False: not sneaking, and what was kept is dropped.
pub fn sneak_tick(g: &mut crate::game::Game) -> bool {
    if !g.k.sneak.on { return false; }
    if !sneak_live(g) {
        if let Some(s) = g.m.sneak.states.as_mut() {
            if s.live {
                s.live = false;
                s.trail.clear();
                s.men = JsMap::new();
            }
        }
        return false;
    }
    let s = g.m.sneak.states.get_or_insert_with(|| State { trail: Vec::new(), off: 0.0, t: -1e9, men: JsMap::new(), live: true });
    s.live = true;
    track(g);
    true
}

fn low_pace(g: &Game, v: f64, is_low: bool) -> f64 {
    if is_low { m::min(v, g.k.sneak.low_pace) } else { v }
}
/// A crumb's index on the trail now, from its absolute number; below 0 when it has been cut.
fn idx(k: f64, off: f64) -> i64 {
    (k - off) as i64
}
/// His posture in a mode: low with Sam; low whenever he waits, crosses or backs off, and after a back-off until he has moved on.
fn crouch_out(man: &Man, low: bool, mode: Mode) -> bool {
    low || mode == Mode::Wait || mode == Mode::Cross || mode == Mode::Back || man.hid_i >= 0.0
}

/// What a read of the way asks with: where he stands, whose eyes, his mates, the doors that matter, and each crumb's exposure once read.
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
    w.others.iter().any(|o| dist_xz(g.pos_of(*o), q) < 0.7)
}
/// In a door's funnel: some defender's eye, inside its reach, that would gain on a man at `q`, with the flat line to `q` through the frame's opening.
fn funnel(g: &mut Game, w: &Way, q: V3, doors: &[DoorId], pad: f64, lit: &mut dyn FnMut(&mut Game) -> f64, lo: bool) -> bool {
    for &h in w.eyes.iter() {
        let e = g.pos_of(h);
        let ed = dist_xz(e, q);
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
            if crate::doorpath::cross(e[0], e[2], q[0], q[2], ax, az) * crate::doorpath::cross(e[0], e[2], q[0], q[2], bx, bz) <= 0.0
                && crate::doorpath::cross(ax, az, bx, bz, e[0], e[2]) * crate::doorpath::cross(ax, az, bx, bz, q[0], q[2]) <= 0.0
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
/// A spot's light to a man on it.
fn lit_spot(g: &mut Game, q: V3, lo: bool) -> f64 {
    let y = if lo { low_y() } else { up_y() };
    cpu_lit(g, [q[0], y[0], q[2]], [q[0], y[1], q[2]], None, None)
}
fn unseen_at(g: &mut Game, w: &Way, q: V3, lo: bool) -> bool {
    let mut l = -1.0;
    let mut lit = |g: &mut Game| {
        if l < 0.0 { l = lit_spot(g, q, lo); }
        l
    };
    let pc = stand_in(q, lo);
    if read(g, &w.eyes, &pc, &mut lit, Some(lo), Some(w.card_f)).seen { return false; }
    if !funnel(g, w, q, &w.open_doors, 0.0, &mut lit, lo) { return true; }
    let v = lit(g);
    !(v >= g.k.sneak.lit_at)
}
/// A spot off the trail he may take (`deep`: unseen with that many metres to spare towards the trail) or hold (0).
fn side_ok(g: &mut Game, w: &Way, t: &[Crumb], q: V3, lo: bool, deep: f64) -> bool {
    let p = w.p;
    if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { return false; }
    if dist_xz(q, g.seat_pos(&crate::seats::here(g))) < g.k.sneak.stand_near + 0.3 { return false; }
    if near_other(g, w, q) { return false; }
    if g.doors.list.iter().any(|d| dist_xz(q, g.dr[*d].frame_centre) < 1.0) { return false; }
    if dist_xz(p, q) > 0.3 && (crate::scene_collision::low_blocked(g, p, q) || !crate::scene_collision::nav_grid_walkable(g, p, q)) { return false; }
    let c = t[w.i as usize].p;
    if dist_xz(q, c) > 2.7 || !unseen_at(g, w, q, lo) { return false; }
    let side_stop = g.k.sneak.side_stop;
    if side_stop > 0.0 {
        let dp = dist_xz(p, q);
        if dp > 0.3 {
            for s in [side_stop / 2.0, side_stop] {
                if !unseen_at(g, w, [q[0] + (p[0] - q[0]) * s / dp, 0.0, q[2] + (p[2] - q[2]) * s / dp], lo) { return false; }
            }
        }
    }
    let d = dist_xz(q, c);
    deep <= 0.0 || d <= deep || unseen_at(g, w, [q[0] + (c[0] - q[0]) * deep / d, 0.0, q[2] + (c[2] - q[2]) * deep / d], lo)
}
/// A spot off the trail he can step straight to when no step on it will do. `caught`: seen where he stands, so the spot he is on is no answer.
fn aside(g: &mut Game, w: &Way, t: &[Crumb], man: &mut Man, lo: bool, caught: bool) -> Option<V3> {
    let side_m = g.k.sneak.side_m;
    if side_m <= 0.0 { return None; }
    let p = w.p;
    let now = g.time;
    let at_crumb = t[w.i as usize].p;
    if let Some(ms) = man.side {
        if !(caught && dist_xz(p, ms) < 0.3) && dist_xz(ms, at_crumb) < side_m + 1.0 && side_ok(g, w, t, ms, lo, 0.0) { return Some(ms); }
    }
    if now - man.side_t < 1.0 && now >= man.side_t { return None; }
    let mut best: Option<V3> = None;
    let mut bd = 1e9;
    let dc: Option<V3> = w.sam_doors.first().map(|d| g.dr[*d].frame_centre);
    let d0 = match dc { Some(c) => dist_xz(p, c), None => 0.0 };
    for deep in [0.55, 0.3] {
        if best.is_some() && dc.is_none() { break; }
        let mut r = 0.6;
        while r <= side_m + 1e-6 {
            for a in 0..12 {
                let a = a as f64;
                let q: V3 = [p[0] + r * m::sin(a * m::PI / 6.0), 0.0, p[2] + r * m::cos(a * m::PI / 6.0)];
                let d = r + 0.3 * dist_xz(q, at_crumb) + (match dc {
                    Some(c) => (if deep < 0.5 { 0.8 } else { 0.0 }) + 0.6 * m::max(0.0, dist_xz(q, c) - d0),
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
    if own && dist_xz(p, c) < 0.3 { return false; }
    if ex(g, w, t, k, lo).seen { return false; }
    if near_other(g, w, c) { return false; }
    if in_line(g, w, t, k, lo) { return false; }
    if !strict { return true; }
    if !unseen_at(g, w, c, lo) { return false; }
    // shy: where he would be seen coming, or stop short
    let d = dist_xz(p, c);
    !(d > 0.4 && !unseen_at(g, w, [c[0] + (p[0] - c[0]) * 0.35 / d, 0.0, c[2] + (p[2] - c[2]) * 0.35 / d], lo))
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
    while k < len && fd < stand_m && (k <= w.place || dist_xz(t[k as usize].p, g.seat_pos(&crate::seats::here(g))) >= stand_near) {
        fd += dist_xz(t[k as usize].p, t[(k - 1) as usize].p);
        if dark_ok(g, w, t, k, own, lo, strict) {
            fwd = k;
            break;
        }
        k += 1;
    }
    let mut k = at - 1;
    while k >= 0 && bd < stand_m {
        bd += dist_xz(t[k as usize].p, t[(k + 1) as usize].p);
        if dark_ok(g, w, t, k, own, lo, strict) {
            bwd = k;
            break;
        }
        k -= 1;
    }
    if fwd >= 0 && (bwd < 0 || fd <= bd) { fwd } else { bwd }
}

/// Where a follower is this frame: his place on Sam's trail, and how far he has to go.
#[derive(Clone, Copy)]
struct Follower {
    gd: GuardId,
    p: V3,
    /// the crumb he is on, and the crumb his place in the file is at (indexes on the trail now)
    i: i64,
    place: i64,
    /// the trail's first crumb's number
    off: f64,
    low: bool,
    /// the walk from him to Sam, and the gap he keeps
    rem: f64,
    gap: f64,
    stopped: bool,
}

/// One read of the way for one man: what it has in hand, and what it has come to so far.
struct Reading<'a> {
    man: &'a mut Man,
    t: &'a mut [Crumb],
    at: Follower,
    w: Way,
    low_walk: bool,
    /// how exposed he is where he stands
    here: Exp,
    /// he has further to walk: on, not stood at his place
    going: bool,
    /// some of the eyes are of men walking over where he stands (and he has further to walk): those men, and the rest
    on_walk: bool,
    coming: Vec<GuardId>,
    else_eyes: Vec<GuardId>,
    mode: Mode,
    stand: i64,
    side: Option<V3>,
    why: Why,
    from: i64,
    to: i64,
}

/// The lit stretch of trail ahead of him, from its first exposed crumb.
#[derive(Clone, Copy)]
struct Stretch {
    /// the first crumb past it where he can walk as he walks; below 0: none
    end: i64,
    /// the last crumb that is still part of it
    last: i64,
    metres: f64,
    /// somebody looks at a crouched man on it, on a lit crumb of it, and who first
    watched: bool,
    lit: bool,
    by: Option<GuardId>,
}

/// The read of the way, a few times a second: his mode, his dark step or spot, and the words on a change.
fn plan(g: &mut Game, s: &mut State, man: &mut Man, at: Follower) {
    man.plan_t = g.time;
    let eyes = if g.k.sneak.skill > 0.0 { eyes_for(g, at.gd) } else { Vec::new() };
    let low_walk = crouch_out(man, at.low, Mode::Walk);
    let keen_f = if crate::guards::escalation_of(g) >= 1.0 { crate::guards::KEEN } else { 1.0 };
    let card_f = crate::kitload::load_seen(g, g.gd[at.gd].ch);
    let here = if !eyes.is_empty() { exposure_of(g, &eyes, at.gd) } else { none() };
    let t = s.trail.as_mut_slice();
    if on_a_move_still(g, man, t, &at) { return; }
    let w = the_way(g, &at, eyes, keen_f, card_f);
    let coming: Vec<GuardId> = w.eyes.iter().copied().filter(|&h| sneak_coming(g, at.gd, h)).collect();
    let going = at.rem > at.gap + 0.5 || !at.stopped;
    let on_walk = !coming.is_empty() && going;
    let else_eyes: Vec<GuardId> = if on_walk { w.eyes.iter().copied().filter(|h| !coming.contains(h)).collect() } else { Vec::new() };
    let why = man.why;
    let mut q = Reading { man, t, at, w, low_walk, here, going, on_walk, coming, else_eyes, mode: Mode::Walk, stand: -1, side: None, why, from: -1, to: -1 };
    if !q.w.eyes.is_empty() {
        if q.going { walking_on(g, &mut q); } else { stood_at_his_place(g, &mut q); }
        caught(g, &mut q);
        keeps_his_spot(g, &mut q);
    }
    settles(g, &mut q);
}

/// A cross ends when he is over it, a back-off when he is there or too late. True: he is on one of them still, with nothing to read.
fn on_a_move_still(g: &Game, man: &mut Man, t: &[Crumb], at: &Follower) -> bool {
    let now = g.time;
    let (p, off, len) = (at.p, at.off, t.len() as i64);
    if man.mode == Mode::Cross {
        let e = idx(man.cross_to, off);
        if e < 0 || e >= len || at.i > e || dist_xz(p, t[e as usize].p) < 0.3 || now - man.cross_t > g.k.sneak.cross_secs || now < man.cross_t {
            man.mode = Mode::Walk;
            man.cross_from = -1.0;
            man.cross_to = -1.0;
        }
    }
    if man.mode == Mode::Back {
        let e = idx(man.stand, off);
        let there = match man.side {
            Some(ms) => dist_xz(p, ms) < 0.3,
            None => e < 0 || e >= len || dist_xz(p, t[e as usize].p) < 0.3,
        };
        if there || now - man.back_t > 4.0 || now < man.back_t {
            man.mode = Mode::Walk;
            man.hide_t = now;
            man.hid_i = man.i;
        }
    }
    man.mode == Mode::Back || (man.mode == Mode::Cross && at.i >= idx(man.cross_from, off) - 1)
}

/// What the read asks of the world: whose eyes, his mates, the doors Sam is at and the doors standing open.
fn the_way(g: &mut Game, at: &Follower, eyes: Vec<GuardId>, keen_f: f64, card_f: f64) -> Way {
    let others: Vec<GuardId> = crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None).into_iter().filter(|o| *o != at.gd).collect();
    let sam = g.seat_pos(&crate::seats::here(g));
    let list = g.doors.list.clone();
    let door_m = g.k.sneak.door_m;
    let mut sam_doors: Vec<DoorId> = Vec::new();
    if door_m > 0.0 {
        for &d in list.iter() {
            if !(dist_xz(sam, g.dr[d].frame_centre) < door_m) { continue; }
            if leaf_moved(g, d) || m::abs(g.dr[d].leaf.angle) <= 0.03 { sam_doors.push(d); }
        }
    }
    let mut open_doors: Vec<DoorId> = list.iter().copied().filter(|&d| m::abs(g.dr[d].leaf.angle) > 0.03).collect();
    open_doors.extend(sam_doors.iter().copied());
    Way { p: at.p, i: at.i, place: at.place, eyes, others, sam_doors, open_doors, keen_f, card_f, memo: JsMap::new() }
}

/// Walking: the next stretch of the trail, read a step at a time for the posture he walks in; then the line of a door Sam is at.
fn walking_on(g: &mut Game, q: &mut Reading) {
    if let Some(first) = first_exposed(g, q) {
        let sp = the_stretch(g, q, first);
        cross_or_wait(g, q, first, sp);
    } else if let Some(mm) = q.man.mem.filter(|mm| !(q.on_walk && q.coming.contains(&mm.h))) {
        remembers_a_look(g, q, mm);
    }
    out_of_the_door_line(g, q);
}

/// The first crumb within his look ahead that is exposed to him as he walks.
fn first_exposed(g: &mut Game, q: &mut Reading) -> Option<i64> {
    let reach = g.k.sneak.ahead_m * m::max(0.25, g.k.sneak.skill);
    let (p, i, lo) = (q.at.p, q.at.i, q.low_walk);
    let mut a = 0.0;
    let k0 = if dist_xz(p, q.t[i as usize].p) > 0.45 { i } else { i + 1 };
    let mut k = k0;
    while k <= q.at.place && a < reach {
        a += if k == k0 { dist_xz(p, q.t[k as usize].p) } else { dist_xz(q.t[(k - 1) as usize].p, q.t[k as usize].p) };
        if ex(g, &mut q.w, q.t, k, lo).seen || in_line(g, &q.w, q.t, k, lo) { return Some(k); }
        k += 1;
    }
    None
}

/// The stretch: on to the first step past it where he can walk as he walks, and whether anybody looks at a crouched man on it.
fn the_stretch(g: &mut Game, q: &mut Reading, first: i64) -> Stretch {
    let stand_near = g.k.sneak.stand_near;
    let (place, lo, len) = (q.at.place, q.low_walk, q.t.len() as i64);
    let mut sp = Stretch { end: -1, last: first, metres: 0.0, watched: false, lit: false, by: None };
    let mut k = first;
    while k < len && k < first + 60 {
        if k > place && dist_xz(q.t[k as usize].p, g.seat_pos(&crate::seats::here(g))) < stand_near { break; }
        if !ex(g, &mut q.w, q.t, k, lo).seen && !in_line(g, &q.w, q.t, k, lo) {
            sp.end = k;
            break;
        }
        let x = ex(g, &mut q.w, q.t, k, true);
        sp.watched = sp.watched || x.watched;
        sp.by = sp.by.or(x.by);
        sp.last = k;
        sp.lit = sp.lit || (x.seen && !x.near);
        if k > first { sp.metres += dist_xz(q.t[(k - 1) as usize].p, q.t[k as usize].p); }
        k += 1;
    }
    sp
}

/// He crosses a stretch that is cheap, unwatched and short (or waited out), or seen only by men walking over where he stands; else he waits, and remembers who looked.
fn cross_or_wait(g: &mut Game, q: &mut Reading, first: i64, sp: Stretch) {
    let k_ = g.k.sneak;
    let now = g.time;
    let too_long = sp.metres > k_.cross_m;
    let waited = if q.man.mode == Mode::Wait && q.man.wait_t >= 0.0 { now - q.man.wait_t } else { 0.0 };
    let cheap = sp.end >= 0 && sp.watched && !too_long && {
        let card = crate::kitload::load_cross(g, g.gd[q.at.gd].ch);
        cross_cost(g, &q.w.eyes, q.t, first as f64, sp.end as f64, Some(card)) <= k_.budget
    };
    let walked = q.on_walk
        && ex(g, &mut q.w, q.t, first, q.low_walk).seen
        && sp.by.map_or(true, |b| q.coming.contains(&b))
        && !others_on(g, &q.else_eyes, q.t, first, sp.last, q.w.card_f);
    if sp.end >= 0 && (walked || cheap || (!sp.watched && (!too_long || (k_.patience > 0.0 && waited >= k_.patience)))) {
        q.mode = Mode::Cross;
        q.from = first;
        q.to = sp.end;
    } else if !walked {
        q.mode = Mode::Wait;
        q.why = if sp.watched {
            if sp.lit { Why::Lit } else { Why::Near }
        } else if !ex(g, &mut q.w, q.t, first, q.low_walk).seen {
            Why::Door
        } else if sp.end < 0 {
            Why::Open
        } else {
            Why::Long
        };
    }
    q.man.mem = match sp.by {
        Some(h) if q.mode == Mode::Wait && sp.watched && sp.lit => Some(ManMem { h, a: first as f64 + q.at.off, b: sp.last as f64 + q.at.off, t: now }),
        _ => None,
    };
}

/// Nothing reads exposed ahead, but a moment ago a man was looking at that lit stretch and he cannot see that man now: he waits.
fn remembers_a_look(g: &mut Game, q: &mut Reading, mm: ManMem) {
    let k_ = g.k.sneak;
    let (now, off, p, gd) = (g.time, q.at.off, q.at.p, q.at.gd);
    let a = m::max(0.0, mm.a - off) as i64;
    let b = m::min((q.t.len() as i64 - 1) as f64, mm.b - off) as i64;
    let hc = g.gd[mm.h].ch;
    let mut lit = false;
    let mut k = a;
    while k <= b && !lit {
        lit = lit_at(g, &mut q.t[k as usize], true) >= k_.lit_at;
        k += 1;
    }
    let own_c = g.gd[gd].ch;
    let eye = g.ch[own_c].bone(crate::character::Bone::Head).unwrap_or([p[0], 1.0, p[2]]);
    let hp = g.ch[hc].pos;
    let his = g.ch[hc].bone(crate::character::Bone::Head).unwrap_or([hp[0], 1.65, hp[2]]);
    let sees = !crate::scene_collision::segment_blocked(g, eye, his, None, None, None);
    if now - mm.t > k_.memory || now < mm.t || crate::condition::down(g, g.gd[mm.h].ch) || idx(mm.b, off) < 0 || q.at.i >= a || sees || !lit {
        q.man.mem = None;
    } else {
        q.mode = Mode::Wait;
        q.why = Why::Lit;
    }
}

/// Stood IN the line of a shut door Sam is at: out of it first, and then he waits there.
fn out_of_the_door_line(g: &mut Game, q: &mut Reading) {
    if q.w.sam_doors.is_empty() || q.mode == Mode::Cross { return; }
    let (i, lo, p) = (q.at.i, q.low_walk, q.at.p);
    let in_funnel = {
        let tt = &mut *q.t;
        let pad = g.k.sneak.door_pad;
        funnel(g, &q.w, p, &q.w.sam_doors, pad, &mut |g: &mut Game| lit_at(g, &mut tt[i as usize], lo), lo)
    };
    if in_funnel && lit_at(g, &mut q.t[i as usize], lo) >= g.k.sneak.lit_at {
        let k = dark(g, &mut q.w, q.t, i, true, lo, false);
        step_or_aside(g, q, k, false);
    }
}

/// Sam stopped and he is at his place: if that is lit with a line, the nearest step along the trail that is not, or a spot aside if he is near it.
fn stood_at_his_place(g: &mut Game, q: &mut Reading) {
    let lo = q.low_walk;
    let at = m::min(q.at.place as f64, (q.t.len() as i64 - 1) as f64) as i64;
    if !(ex(g, &mut q.w, q.t, at, lo).seen || in_line(g, &q.w, q.t, at, lo)) { return; }
    let k = dark(g, &mut q.w, q.t, at, false, lo, false);
    if k >= 0 || dist_xz(q.at.p, q.t[at as usize].p) < g.k.sneak.side_m + 0.5 {
        step_or_aside(g, q, k, false);
    }
}

/// He goes to the dark step `k`, or with none (below 0) to a spot off the trail if there is one: backing off if `caught`, else to stand there.
fn step_or_aside(g: &mut Game, q: &mut Reading, k: i64, caught: bool) {
    let mode = if caught { Mode::Back } else { Mode::Stand };
    if k >= 0 {
        q.stand = k;
        q.mode = mode;
        return;
    }
    q.side = aside(g, &q.w, q.t, q.man, caught || q.low_walk, caught);
    if q.side.is_some() { q.mode = mode; }
}

/// CAUGHT: he is seen where he stands: the nearest dark step, low. Not twice inside a few seconds.
fn caught(g: &mut Game, q: &mut Reading) {
    if q.mode == Mode::Stand { return; }
    let by_those_coming = q.on_walk
        && q.here.by.map_or(true, |b| q.coming.contains(&b))
        && !(!q.else_eyes.is_empty() && exposure_of(g, &q.else_eyes, q.at.gd).watched);
    if by_those_coming || !(q.here.watched || (q.here.seen && (q.mode == Mode::Wait || q.at.stopped))) { return; }
    let now = g.time;
    if !(now - q.man.hide_t > g.k.sneak.hide_gap || now < q.man.hide_t) { return; }
    let k = dark(g, &mut q.w, q.t, q.at.i, true, true, true);
    step_or_aside(g, q, k, true);
}

/// Stood aside and not seen there: he keeps the spot while he waits, or while Sam stands.
fn keeps_his_spot(g: &mut Game, q: &mut Reading) {
    if q.side.is_some() { return; }
    let Some(ms) = q.man.side else { return };
    let side_m = g.k.sneak.side_m;
    if (q.mode == Mode::Wait || (q.mode == Mode::Walk && q.at.stopped && q.at.rem <= q.at.gap + side_m + 0.5)) && dist_xz(ms, q.t[q.at.i as usize].p) < side_m + 1.0 {
        let lo = crouch_out(q.man, q.at.low, q.mode);
        if side_ok(g, &q.w, q.t, ms, lo, 0.0) { q.side = Some(ms); }
    }
}

/// What the read came to goes on his row; the words, and the tallies, on the change; and a wait gone on too long is said again.
fn settles(g: &mut Game, q: &mut Reading) {
    let k_ = g.k.sneak;
    let (now, off, gd, mode, why) = (g.time, q.at.off, q.at.gd, q.mode, q.why);
    let man = &mut *q.man;
    man.stand = if q.stand >= 0 { q.stand as f64 + off } else { -1.0 };
    man.side = q.side;
    if mode == Mode::Cross {
        man.cross_from = q.from as f64 + off;
        man.cross_to = q.to as f64 + off;
    }
    if mode != man.mode {
        if man.mode == Mode::Wait {
            if mode == Mode::Cross && now - man.wait_t > 1.5 { cue(g, gd, man, &lines().go, None); }
            man.wait_t = -1.0;
        }
        if mode == Mode::Wait {
            man.why = why;
            man.wait_t = now;
            man.nagged = false;
            let line = said_wait(why);
            cue(g, gd, man, &line, None);
        }
        if mode == Mode::Cross {
            man.cross_t = now;
        }
        if mode == Mode::Back {
            man.back_t = now;
        }
        man.mode = mode;
    } else if mode == Mode::Wait {
        man.why = why;
    }
    if man.mode == Mode::Wait && !man.nagged && now - man.wait_t >= k_.nag_secs && !(now >= man.voice_t && now - man.voice_t < 3.0) {
        let line = said_nag(man.why);
        man.nagged = cue(g, gd, man, &line, Some(true));
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
fn lay(g: &Game, man: &mut Man, t: &[Crumb], p: V3, i: i64, to: i64) -> V3 {
    let goal = t[to as usize].p;
    let mut path: Vec<V3> = Vec::new();
    if to >= i {
        let mut k = if dist_xz(p, t[i as usize].p) > 0.45 { i } else { i + 1 };
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
    man.way = if !path.is_empty() && !crate::scene_collision::low_blocked(g, p, path[0]) { Some(path) } else { None };
    goal
}

/// What the frame asks of a follower.
#[derive(Clone, Copy)]
struct Asked {
    gd: GuardId,
    back: f64,
    sam_v: f64,
    stopped: bool,
    speed0: f64,
}

/// A sneaking follower's feet this frame, with his row and the state in hand (both out of `g`): where he is on the trail, the plan if it is due, and his feet by it.
fn follow(g: &mut Game, s: &mut State, man: &mut Man, ask: Asked) -> Feet {
    let now = g.time;
    let gd = ask.gd;
    let p = g.pos_of(gd);
    let low = crate::unseen::sam_low(g, g.player);
    man.t = now;
    man.way = None;
    let plain = Feet { goal: None, speed: low_pace(g, ask.speed0, low), crouch: low, must: None };
    if s.trail.len() < 3 { return plain; }
    let off = s.off;
    let Some(i) = found_on_the_trail(g, &s.trail, man, p, off) else { return plain };
    let i = further_on(g, &s.trail, p, i);
    let i = back_with_him(man, &s.trail, p, i, off);
    man.i = i as f64 + off;
    if man.hid_i >= 0.0 && m::abs(man.i - man.hid_i) >= 3.0 { man.hid_i = -1.0; }
    let sam = g.seat_pos(&crate::seats::here(g));
    let gap = ask.back + g.k.sneak.back;
    let (rem, place) = the_trail_to_sam(&s.trail, i, sam, gap);
    let at = Follower { gd, p, i, place, off, low, rem, gap, stopped: ask.stopped };
    // the read of the way, a few times a second (the plan stands in between)
    if now - man.plan_t >= g.k.sneak.plan_secs || now < man.plan_t {
        plan(g, s, man, at);
    }
    his_feet(g, man, &s.trail, &at, ask.sam_v)
}

/// His crumb on the trail. Lost: the nearest crumb he can walk straight to, looked for twice a second, and whatever he was doing is forgotten.
fn found_on_the_trail(g: &Game, t: &[Crumb], man: &mut Man, p: V3, off: f64) -> Option<i64> {
    let now = g.time;
    let i = idx(man.i, off);
    if !(i < 0 || i >= t.len() as i64 || dist_xz(p, t[i as usize].p) > 3.0) { return Some(i); }
    if now - man.lost_t < 0.5 && now >= man.lost_t { return None; }
    let Some(i) = nearest_crumb(g, t, p) else {
        man.lost_t = now;
        return None;
    };
    man.mode = Mode::Walk;
    man.cross_from = -1.0;
    man.cross_to = -1.0;
    man.stand = -1.0;
    man.hid_i = -1.0;
    man.mem = None;
    man.side = None;
    Some(i)
}

/// The nearest crumb he can walk straight to, the later of two about as near.
fn nearest_crumb(g: &Game, t: &[Crumb], p: V3) -> Option<i64> {
    let mut best = None;
    let mut bd = 1e9;
    let mut k = t.len() as i64 - 1;
    while k >= 0 {
        let c = t[k as usize].p;
        let d = dist_xz(p, c);
        if d < bd - 0.05 && (d < 0.8 || crate::scene_collision::nav_grid_walkable(g, p, c)) {
            bd = d;
            best = Some(k);
        }
        k -= 1;
    }
    best
}

/// The furthest crumb he is standing on, or the later pass of it a stride away where the trail doubles back.
fn further_on(g: &Game, t: &[Crumb], p: V3, i: i64) -> i64 {
    (i + 1..t.len() as i64).rev().find(|&k| {
        let c = t[k as usize].p;
        let d = dist_xz(p, c);
        d < 0.5 || (d < 0.9 && k - i > 12 && !crate::scene_collision::low_blocked(g, p, c))
    }).unwrap_or(i)
}

/// Walking to a step behind him, his place on the trail goes back with him.
fn back_with_him(man: &Man, t: &[Crumb], p: V3, i: i64, off: f64) -> i64 {
    if man.mode != Mode::Stand && man.mode != Mode::Back { return i; }
    let st = idx(man.stand, off);
    if st < 0 || st >= i { return i; }
    (st..i).find(|&k| dist_xz(p, t[k as usize].p) < 0.45).unwrap_or(i)
}

/// The walk from crumb `i` to Sam, and the crumb his place in the file is at (never behind him).
fn the_trail_to_sam(t: &[Crumb], i: i64, sam: V3, gap: f64) -> (f64, i64) {
    let len = t.len() as i64;
    let mut rem = 0.0;
    let mut k = i;
    while k < len - 1 {
        rem += dist_xz(t[k as usize].p, t[(k + 1) as usize].p);
        k += 1;
    }
    rem += dist_xz(t[(len - 1) as usize].p, sam);
    let mut place = len - 1;
    let mut a = dist_xz(t[(len - 1) as usize].p, sam);
    while place > 0 && a < gap {
        a += dist_xz(t[place as usize].p, t[(place - 1) as usize].p);
        place -= 1;
    }
    (rem, if place < i { i } else { place })
}

/// His feet, by the plan: to his spot aside, stood still on a wait, to his dark step, or on along the trail.
fn his_feet(g: &Game, man: &mut Man, t: &[Crumb], at: &Follower, sam_v: f64) -> Feet {
    let p = at.p;
    let low_now = crouch_out(man, at.low, man.mode);
    let to_it = if man.mode == Mode::Back { m::min(g.k.sneak.low_pace, 1.2) } else { low_pace(g, 1.0, low_now) };
    if let (Some(ms), true) = (man.side, man.mode != Mode::Cross) {
        let speed = if dist_xz(p, ms) < 0.25 { 0.0 } else { to_it };
        return Feet { goal: Some(ms), speed, crouch: low_now, must: Some(true) };
    }
    if man.mode == Mode::Wait { return Feet { goal: None, speed: 0.0, crouch: low_now, must: None }; }
    if man.mode == Mode::Stand || man.mode == Mode::Back {
        let st = idx(man.stand, at.off);
        if st >= 0 && st < t.len() as i64 {
            let c = t[st as usize].p;
            let goal = if dist_xz(p, c) < 0.3 { c } else { lay(g, man, t, p, at.i, st) };
            return Feet { goal: Some(goal), speed: to_it, crouch: low_now, must: Some(true) };
        }
        if man.mode == Mode::Back { return Feet { goal: None, speed: 0.0, crouch: low_now, must: None }; }
    }
    walking_his_steps(g, man, t, at, sam_v)
}

/// Walking Sam's steps: a goal a few crumbs up the trail (never past his place), at Sam's pace plus what closes the gap.
fn walking_his_steps(g: &Game, man: &mut Man, t: &[Crumb], at: &Follower, sam_v: f64) -> Feet {
    let k_ = g.k.sneak;
    let (p, i, place) = (at.p, at.i, at.place);
    let low_now = crouch_out(man, at.low, man.mode);
    let crossing = man.mode == Mode::Cross;
    let to_place = m::max(0.0, at.rem - at.gap);
    if to_place < 0.25 && !crossing {
        let c = t[place as usize].p;
        let goal = if place > i { lay(g, man, t, p, i, place) } else { c };
        let speed = if at.stopped && dist_xz(p, c) < 0.3 { 0.0 } else { low_pace(g, m::max(k_.min_pace, sam_v), low_now) };
        return Feet { goal: Some(goal), speed, crouch: low_now, must: None };
    }
    let mut j = i;
    let mut a = 0.0;
    let lim = if crossing { m::max(place as f64, m::min((t.len() as i64 - 1) as f64, man.cross_to - at.off)) as i64 } else { place };
    while j < lim && a < 2.6 {
        a += dist_xz(t[j as usize].p, t[(j + 1) as usize].p);
        j += 1;
    }
    let v = if crossing { k_.cross_pace } else { m::max(k_.min_pace, m::min(2.2, sam_v + 1.5 * (to_place - 0.3))) };
    let goal = if j > i { lay(g, man, t, p, i, j) } else { t[j as usize].p };
    Feet { goal: Some(goal), speed: low_pace(g, v, low_now && !crossing), crouch: low_now, must: None }
}

/// The sneaking follower's feet this frame. None: he is the build's own follower.
pub fn sneak_follow(g: &mut crate::game::Game, gd: GuardId, back: f64, sam_v: f64, stopped: bool, speed0: f64) -> Option<crate::sneak::Feet> {
    if !g.k.sneak.on || !g.team(gd).ours() { return None; }
    if !sneak_tick(g) { return None; }
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
    let feet = follow(g, &mut s, &mut man, Asked { gd, back, sam_v, stopped, speed0 });
    *s.men.get_mut(gd).unwrap() = man;
    g.m.sneak.states = Some(s);
    Some(feet)
}

/// Right after the order is written: the way to this frame's goal is Sam's own crumbs, not the nav grid's cells.
pub fn sneak_way(g: &mut crate::game::Game, gd: GuardId, goal: V3) {
    if !g.k.sneak.on { return; }
    let has_way = match g.m.sneak.states.as_ref().and_then(|s| s.men.get(gd)) {
        Some(man) => man.way.is_some(),
        None => false,
    };
    if !has_way { return; }
    if let Some(pg) = g.gd[gd].path_goal {
        if dist_xz(pg, goal) < 0.05 && g.gd[gd].path_i < g.gd[gd].path.len() { return; }
    }
    let way = g.m.sneak.states.as_mut().unwrap().men.get_mut(gd).unwrap().way.take().unwrap();
    g.gd[gd].set_path(way);
    g.gd[gd].path_i = 0;
    g.gd[gd].set_path_goal(Some([goal[0], 0.0, goal[2]]));
    g.gd[gd].repath_t = 0.0;
}

/// His words over the net. False only when ONE VOICE kept it back (the line is still owed).
fn cue(g: &mut crate::game::Game, gd: GuardId, m: &mut crate::sneak::Man, line: &str, force: Option<bool>) -> bool {
    let force = force.unwrap_or(false);
    if line.is_empty() || (!force && g.time - m.cue_t < g.k.sneak.cue_gap && g.time >= m.cue_t) { return true; }
    if g.k.spotted.on && crate::spotted::spotted_mode(g, gd) != crate::spotted::SpottedMode::Calm {
        m.voice_t = g.time;
        return false;
    }
    m.cue_t = g.time;
    g.stats.sneak.cues += 1;
    crate::game::game_say(g, gd, line, None, None);
    true
}

