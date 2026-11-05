//! src/game/throwplan.ts lines 1 to 125
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// what `ray` gives back: the prop's hit or the wall's (`prop`: it was a prop's)
struct RayHit {
    t: f64,
    n: V3,
    prop: bool,
}

/// the walls first, then the props no further than the wall; a prop's hit wins (throwplan.ts throwFlight's `ray`)
fn ray(g: &mut Game, ro: V3, rd: V3, tmax: f64) -> Option<RayHit> {
    let st = crate::scene_collision::raycast(g, ro, rd, tmax);
    let reach = match &st { Some(s) => s.t, None => tmax };
    let ph = crate::props::prop_system_raycast(g, ro, rd, reach);
    match ph {
        Some(h) => Some(RayHit { t: h.t, n: h.n, prop: true }),
        None => st.map(|s| RayHit { t: s.t, n: s.n, prop: false }),
    }
}

/// Where a thing thrown from `from` at `v0` goes off: the item pool's own flight, moving nothing and rolling no die.   (throwplan.ts throwFlight)
pub fn throw_flight(g: &mut crate::game::Game, kind: crate::squadthrow::SquadThrowKind, from: V3, v0: V3, step: Option<f64>) -> crate::throwplan::ThrowFlight {
    let step = step.unwrap_or(1.0 / 60.0);
    let frag = kind == crate::squadthrow::SquadThrowKind::Frag;
    let (radius, restitution, friction) = if frag {
        let item = crate::throwables::frag_item();
        (item.radius, item.restitution, item.friction)
    } else {
        (0.055, 0.38, 0.35)
    };
    let mut pos = v3::copy(from);
    let mut vel = v3::copy(v0);
    let mut settled = false;
    let mut bounces: i32 = 0;
    let mut age = 0.0;
    let mut fuse = if frag { g.k.frag.order_fuse } else { crate::combat::flash_fuse(g) };
    let mut land: Option<V3> = None;
    let mut points: Vec<V3> = vec![v3::copy(pos)];
    let h = step / 2.0;
    g.stats.throwplan.sims += 1;
    let n = m::ceil(12.0 / step);
    let mut i: i32 = 0;
    while (i as f64) < n {
        age += step;
        let mut s = 0;
        while s < 2 && !settled {
            s += 1;
            vel[1] -= crate::throwables::GRAVITY * h;
            let mv = v3::scale(vel, h);
            let len = v3::len(mv);
            if len < 1e-6 { continue; }
            let dir = v3::scale(mv, 1.0 / len);
            let mut hit = ray(g, pos, dir, len + radius * 3.5);
            let mut back = 0.0;
            if let Some(ht) = &hit {
                back = radius / m::max(0.3, -v3::dot(dir, ht.n));
                if ht.t - back > len { hit = None; }
            }
            if let Some(ht) = hit {
                pos = v3::mad(pos, dir, m::max(0.0, ht.t - back));
                let nrm = ht.n;
                let vn = v3::dot(vel, nrm);
                if vn < 0.0 {
                    let v_n = v3::scale(nrm, vn);
                    let v_t = v3::sub(vel, v_n);
                    // lying snow under it (snowBog.ts; 0 = none)
                    let bog = if nrm[1] > 0.6 && !ht.prop { crate::snow_bog::snow_bog_at(g, pos[0], pos[1] - radius, pos[2]) } else { 0.0 };
                    vel = v3::add(v3::scale(v_t, m::max(0.0, 1.0 - friction * (0.5 + 0.5 * m::abs(nrm[1])))), v3::scale(v_n, -restitution));
                    bounces += 1;
                    if land.is_none() { land = Some(v3::copy(pos)); }
                    if bog > 0.0 {
                        let up = v3::dot(vel, nrm);
                        let b = crate::throwables::bog_bounce(g, bog);
                        let sl = crate::throwables::bog_slide(g, bog);
                        vel = v3::add(v3::scale(v3::sub(vel, v3::scale(nrm, up)), sl), v3::scale(nrm, up * b));
                    }
                    if nrm[1] > 0.6 && m::abs(vel[1]) < 0.5 && m::hyp2(vel[0], vel[2]) < 0.35 + (if bog > 0.0 { crate::throwables::bog_stick(g, bog) } else { 0.0 }) {
                        settled = true;
                        vel = [0.0, 0.0, 0.0];
                        pos[1] += 0.001;
                    }
                }
                pos = v3::mad(pos, nrm, 0.002);
            } else {
                pos = v3::add(pos, mv);
            }
            if pos[1] < radius {
                let bog = crate::snow_bog::snow_bog_at(g, pos[0], 0.0, pos[2]);
                pos[1] = radius;
                if vel[1] < 0.0 {
                    vel[1] *= -restitution;
                    vel[0] *= 0.7;
                    vel[2] *= 0.7;
                    if bog > 0.0 {
                        vel[1] *= crate::throwables::bog_bounce(g, bog);
                        vel[0] *= crate::throwables::bog_slide(g, bog);
                        vel[2] *= crate::throwables::bog_slide(g, bog);
                    }
                    bounces += 1;
                    if land.is_none() { land = Some(v3::copy(pos)); }
                }
                if m::abs(vel[1]) < 0.5 && m::hyp2(vel[0], vel[2]) < 0.35 + (if bog > 0.0 { crate::throwables::bog_stick(g, bog) } else { 0.0 }) {
                    settled = true;
                    vel = [0.0, 0.0, 0.0];
                }
            }
        }
        if i % 2 == 1 || settled { points.push(v3::copy(pos)); }
        let landed = bounces > 0 || settled;
        if frag || landed { fuse -= step; }
        // (at rest: where it lies is where it goes off)
        if settled || fuse <= 0.0 || (!frag && !landed && age > crate::effects::bang().max_air) { break; }
        i += 1;
    }
    crate::throwplan::ThrowFlight {
        end: pos,
        land: match land { Some(l) => l, None => v3::copy(pos) },
        points,
        landed: bounces > 0 || settled,
    }
}

/// The walls that stand by a spot, for a bank: where rays out from over it first meet something solid, a hand's width off its face, the nearest first.   (throwplan.ts throwWalls)
pub fn throw_walls(g: &mut crate::game::Game, at: V3) -> Vec<V3> {
    let (lob_walls, lob_wall_h, lob_wall_m) = (g.k.squadthrow.lob_walls, g.k.squadthrow.lob_wall_h, g.k.squadthrow.lob_wall_m);
    let o: V3 = [at[0], at[1] + lob_wall_h, at[2]];
    let mut out: Vec<(V3, f64)> = Vec::new();
    let mut i = 0.0;
    while i < lob_walls {
        let a = i * 2.0 * m::PI / lob_walls;
        let d: V3 = [m::sin(a), 0.0, m::cos(a)];
        if let Some(hit) = crate::scene_collision::raycast(g, o, d, lob_wall_m) {
            if hit.t > 0.4 { out.push((v3::mad(o, d, hit.t - 0.08), hit.t)); }
        }
        i += 1.0;
    }
    js::sort_by(&mut out, |a, b| a.1 - b.1);
    out.into_iter().map(|x| x.0).collect()
}

/// The hand a throw at `aim` leaves, for a man stood at `p`: ahead of him toward it, under his head. None: it would be in a wall.   (throwplan.ts handAt)
pub fn hand_at(g: &mut crate::game::Game, gd: GuardId, p: V3, aim: V3) -> Option<V3> {
    let k = crate::squadsmoke::squadsmoke();
    let d = v3::dist_xz(p, aim);
    let ahead = if d > 1e-3 { k.hand_ahead / d } else { 0.0 };
    let c = g.gd[gd].ch;
    let head = g.ch[c].bones.get(crate::character::Bone::Head).copied();
    let high = match head { Some(hd) => hd[1] - g.ch[c].pos[1] - k.head_drop, None => k.hand_up };
    let up = m::max(0.6, m::min(k.hand_up, high));
    let from: V3 = [p[0] + (aim[0] - p[0]) * ahead, p[1] + up, p[2] + (aim[2] - p[2]) * ahead];
    if crate::scene_collision::solid_blocked(g, [p[0], p[1] + up, p[2]], from, None) { None } else { Some(from) }
}

/// It went off by the spot: within lobNearM of it over the floor, not up on something tall, nothing solid between the two.   (throwplan.ts throwHits)
pub fn throw_hits(g: &mut crate::game::Game, end: V3, at: V3, slack: Option<f64>) -> bool {
    let slack = slack.unwrap_or(0.0);
    v3::dist_xz(end, at) <= g.k.squadthrow.lob_near_m + slack
        && end[1] - at[1] <= g.k.squadthrow.lob_up_m
        && !crate::scene_collision::solid_blocked(g, [end[0], end[1] + 0.25, end[2]], [at[0], at[1] + 0.25, at[2]], None)
}

/// Flies a throw at `aim` from `from` at each of `paces`, keeping in `best` the one that goes off nearest `at`.   (throwplan.ts throwTo's `fly`)
fn fly(g: &mut Game, kind: crate::squadthrow::SquadThrowKind, at: V3, from: V3, aim: V3, paces: &[f64], miss: &mut f64, best: &mut Option<ThrowTo>, lim: &mut Option<&mut ThrowToLim>) {
    let (dx, dy, dz) = (aim[0] - from[0], aim[1] - from[1], aim[2] - from[2]);
    let dist = m::hyp2(dx, dz);
    let mut last_t = -1.0;
    for &u in paces {
        if *miss <= 0.5 || lim.as_deref().map_or(false, |l| l.n <= 0.0) { return; }
        let t = m::min(1.4, m::max(0.3, dist / u));
        if m::abs(t - last_t) < 0.02 { continue; }
        last_t = t;
        let v0: V3 = [dx / t, (dy + 0.5 * crate::throwables::GRAVITY * t * t) / t, dz / t];
        if v3::len(v0) > g.k.squadthrow.lob_speed { continue; }
        if let Some(l) = lim.as_deref_mut() { l.n -= 1.0; }
        let f = throw_flight(g, kind, from, v0, None);
        let mm = v3::dist_xz(f.end, at);
        if !(mm < *miss && throw_hits(g, f.end, at, None)) { continue; }
        // (the page steps at the display's rate, not this forecast's sixtieth: a throw is taken only if it still goes off by the spot stepped finer and coarser)
        if let Some(l) = lim.as_deref_mut() { l.n -= 2.0; }
        let steps = g.k.squadthrow.lob_steps.clone();
        let mut holds = true;
        for &h in steps.iter() {
            let end = throw_flight(g, kind, from, v0, Some(h)).end;
            if !throw_hits(g, end, at, Some(0.4)) { holds = false; break; }
        }
        if !holds { continue; }
        *miss = mm;
        *best = Some(ThrowTo { from, sol: crate::throwables::ThrowSolution { v0, t, points: f.points, land: f.land }, rest: f.end });
    }
}

/// One man's worked-out throw at a spot: the hand, the throw and where it goes off. None: no way of throwing there.   (throwplan.ts throwTo)
pub fn throw_to(g: &mut crate::game::Game, gd: GuardId, at: V3, kind: crate::squadthrow::SquadThrowKind, stood: Option<V3>, walls: Option<&[V3]>, lim: Option<&mut crate::throwplan::ThrowToLim>) -> Option<crate::throwplan::ThrowTo> {
    let mut lim = lim;
    let c = g.gd[gd].ch;
    let p = match stood { Some(s) => s, None => g.ch[c].pos };
    let mut best: Option<ThrowTo> = None;
    let mut miss = m::INF;
    // straight at it: only with a line from the hand to somewhere over the spot (what he cannot see through he does not throw through)
    let to: V3 = [at[0], at[1] + 0.05, at[2]];
    let from = if v3::dist_xz(p, at) > 0.5 { hand_at(g, gd, p, to) } else { None };
    if let Some(from) = from {
        let seen = [0.3, 1.0, 1.7].iter().any(|y| !crate::scene_collision::solid_blocked(g, from, [at[0], at[1] + *y, at[2]], None));
        if seen {
            let paces = g.k.squadthrow.lob_paces.clone();
            fly(g, kind, at, from, to, &paces, &mut miss, &mut best, &mut lim);
        }
    }
    // … and off a wall that stands by it: one his hand has a line to
    let worked_out;
    let list: &[V3] = match walls {
        Some(w) => w,
        None => { worked_out = throw_walls(g, at); &worked_out }
    };
    for &w in list {
        if miss <= 0.5 || lim.as_deref().map_or(false, |l| l.n <= 0.0) { break; }
        if v3::dist_xz(p, w) < 1.0 { continue; }
        if let Some(hand) = hand_at(g, gd, p, w) {
            if !crate::scene_collision::solid_blocked(g, hand, w, None) {
                let paces = g.k.squadthrow.lob_bank_paces.clone();
                fly(g, kind, at, hand, w, &paces, &mut miss, &mut best, &mut lim);
            }
        }
    }
    best
}
