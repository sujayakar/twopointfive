//! src/game/lanes.ts lines 1 to 367
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the side this rule is for   (lanes.ts SIDE)
const SIDE: crate::teams::Team = crate::teams::Team::Hostile;

/// the game's state as a path, once it exists (`g.time` and the like can be read beside it); a call that takes `g` needs it let go first
macro_rules! st {
    ($g:ident) => {
        $g.m.lanes.states.as_mut().unwrap()
    };
}

/// A new game's state: the TypeScript's own literal, whose clocks are not zero.   (lanes.ts fresh)
pub fn fresh(g: &mut crate::game::Game) -> crate::lanes::State {
    State { said_t: -100.0, t: -1.0, ..State::default() }
}

/// The game's state, made if it has none: the row itself.   (lanes.ts S)
pub fn s<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::lanes::State {
    if g.m.lanes.states.is_none() {
        let f = fresh(g);
        g.m.lanes.states = Some(f);
    }
    g.m.lanes.states.as_mut().unwrap()
}

/// The distance on the plan.   (lanes.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// A man the rule is for: a defender of a live room fight, on his feet.   (lanes.ts mine)
pub fn mine(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let r = &g.gd[gd];
    r.team == SIDE && r.unit.is_some() && r.state != crate::game::GuardState::Dead && r.held.is_none()
}

/// One of his who is neither dazzled nor staggering.   (lanes.ts fit)
pub fn fit(g: &mut crate::game::Game, gd: GuardId) -> bool {
    mine(g, gd) && !crate::control::man::blind(g, gd) && !crate::control::man::staggered(g, gd)
}

/// The rule runs: a room fight is live, the AI on. (LANES.on settled on.)   (lanes.ts live)
pub fn live(g: &mut crate::game::Game) -> bool {
    crate::roomfight::fight_live(g) && g.ai_enabled
}

/// WHO THE LANES HAVE: the lane a man held short of it or out for a look round its corner is on, while that lane stands.   (lanes.ts laneOf)
pub fn lane_of(g: &crate::game::Game, gd: GuardId) -> Option<LanesLaneId> {
    let st = g.m.lanes.states.as_ref()?;
    let lane = match st.held.get(gd) {
        Some(h) => h.lane,
        None => st.peek.get(gd)?.lane,
    };
    st.lanes.iter().copied().find(|&l| g.tab.lanes_lane[l].id == lane)
}

/// The owner's record says what this file's state says of him.   (lanes.ts laneSay)
pub fn lane_say(g: &mut crate::game::Game, gd: GuardId) {
    let (rule, t) = (crate::owner::by::LANE, g.time);
    if lane_of(g, gd).is_some() && !crate::owner::has(g, gd, crate::owner::by::RUNNER) {
        crate::owner::claim(g, gd, rule, t, None);
    } else {
        crate::owner::release(g, gd, rule, t);
    }
}

/// He is held short of a lane.   (lanes.ts heldOn)
pub fn held_on(g: &mut crate::game::Game, gd: GuardId, h: crate::lanes::StateHeldValue) {
    s(g).held.set(gd, h);
    lane_say(g, gd);
}

/// He is no longer held.   (lanes.ts heldOff)
pub fn held_off(g: &mut crate::game::Game, gd: GuardId) {
    s(g).held.delete(gd);
    lane_say(g, gd);
}

/// He is out for a look.   (lanes.ts peekOn)
pub fn peek_on(g: &mut crate::game::Game, gd: GuardId, p: crate::lanes::Peek) {
    s(g).peek.set(gd, p);
    lane_say(g, gd);
}

/// He is back from his look.   (lanes.ts peekOff)
pub fn peek_off(g: &mut crate::game::Game, gd: GuardId) {
    s(g).peek.delete(gd);
    lane_say(g, gd);
}

/// Every man this file had, let go of: the state goes with them.   (lanes.ts letGo)
pub fn let_go(g: &mut crate::game::Game) {
    let (rule, t) = (crate::owner::by::LANE, g.time);
    for gd in s(g).held.keys() {
        crate::owner::release(g, gd, rule, t);
    }
    for gd in s(g).peek.keys() {
        crate::owner::release(g, gd, rule, t);
    }
}

/// A lane has gone: whoever it held is nobody's until the half-second's sweep says what becomes of him.   (lanes.ts lanesGone)
pub fn lanes_gone(g: &mut crate::game::Game) {
    for gd in s(g).held.keys() {
        lane_say(g, gd);
    }
    for gd in s(g).peek.keys() {
        lane_say(g, gd);
    }
}

/// Where he looks from.   (lanes.ts eyeOf)
pub fn eye_of(g: &crate::game::Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(crate::character::Bone::Head) {
        Some(p) => *p,
        None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]),
    }
}

/// The nav cell a point lies in, −1 off the grid.   (lanes.ts cellAt)
pub fn cell_at(g: &crate::game::Game, p: V3) -> f64 {
    let n = &g.col.nav;
    let x = m::floor(p[0] / n.cell);
    let z = m::floor(p[2] / n.cell);
    if x < 0.0 || z < 0.0 || x >= n.w || z >= n.h {
        return -1.0;
    }
    // (+ 0: a coordinate of −0 gives a cell of −0, and `Num` keys go by bits where a JavaScript Set takes −0 for 0)
    z * n.w + x + 0.0
}

/// A point lies on a mask's floor.   (lanes.ts inMask)
pub fn in_mask(g: &crate::game::Game, m: Option<&[u8]>, p: V3) -> bool {
    let Some(m) = m else { return false };
    let i = cell_at(g, p);
    i >= 0.0 && m.get(i as usize) == Some(&1)
}

/// The side has it on the net, or he has it himself.   (lanes.ts knows)
pub fn knows(g: &crate::game::Game, l: LanesLaneId, gd: GuardId) -> bool {
    knows_row(&g.tab.lanes_lane[l], gd)
}

/// knows, for a row in hand beside a borrow of the state
fn knows_row(l: &Lane, gd: GuardId) -> bool {
    l.known || l.knowers.has(gd)
}

/// An enemy he saw this close inside the last second: a fight already.   (lanes.ts closeFight)
pub fn close_fight(g: &crate::game::Game, gd: GuardId) -> bool {
    if g.time - g.gd[gd].last_seen_t >= 1.0 {
        return false;
    }
    let Some(fc) = crate::teams::foe_char(g, gd) else { return false };
    flat(g.ch[g.gd[gd].ch].pos, g.ch[fc].pos) < m::max(g.k.lanes.close_m, g.k.roomfight.move_engage_m)
}

/// He is going in behind a charge; a lapsed entry is dropped as it is asked.   (lanes.ts pushing)
pub fn pushing(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(t) = s(g).push.get(gd).copied() else { return false };
    if g.time < t && g.gd[gd].state != crate::game::GuardState::Dead {
        return true;
    }
    s(g).push.delete(gd);
    false
}

/// The energy a lane's lines are asked at: its heaviest gun's, or nothing. (LANES.glass settled on.)   (lanes.ts laneE)
pub fn lane_e(g: &crate::game::Game, l: LanesLaneId) -> Option<f64> {
    e_of(&g.tab.lanes_lane[l])
}

/// laneE, for a row in hand
fn e_of(l: &Lane) -> Option<f64> {
    if l.e > 0.0 { Some(l.e) } else { None }
}

/// His own senses have the fire since `t`.   (lanes.ts roused)
pub fn roused(g: &crate::game::Game, gd: GuardId, t: f64) -> bool {
    let r = &g.gd[gd];
    r.state == crate::game::GuardState::Alert
        || r.state == crate::game::GuardState::Search
        || r.awareness >= 0.3
        || r.sup.src_t >= t - 1e-6
        || r.last_seen_t >= t - 1.0
        || (r.witness.heard.last_shot_t >= 0.0 && r.witness.heard.last_shot_t >= t - 2.5)
}

/// He is sent round at the guns; a lapsed entry is dropped, and his errand unless he is set.   (lanes.ts flanking)
pub fn flanking(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(t) = s(g).flank.get(gd).copied() else { return false };
    if g.time < t && g.gd[gd].state != crate::game::GuardState::Dead {
        return true;
    }
    let st = s(g);
    st.flank.delete(gd);
    if !matches!(st.flank_of.get(gd), Some(f) if f.phase == FlankPhase::Set) {
        st.flank_of.delete(gd);
    }
    false
}

/// The body with this id is up.   (lanes.ts upId)
pub fn up_id(g: &crate::game::Game, id_2: f64) -> bool {
    if crate::seats::is_man_id(g, id_2) {
        return match crate::seats::man_by_id(g, id_2) {
            Some(p) => g.ch[g.pl[p].ch].alive,
            None => false,
        };
    }
    g.guards.iter().any(|&o| g.ch[g.gd[o].ch].id == id_2 && g.gd[o].state != crate::game::GuardState::Dead)
}

pub use crate::roomfight::def_of;

/// A line in the fight's log.   (lanes.ts note)
pub fn note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) {
    let who = match gd {
        Some(x) => g.gd[x].callsign.clone(),
        None => String::from("-"),
    };
    let t = g.time;
    g.fight.log.push(crate::roomfight::FightNote { t, who, what: format!("[lanes] {}", what) });
    if g.fight.log.len() > 600 {
        g.fight.log.drain(0..200);
    }
}

/// The floor one gun at `from` covers along the line to `at`: nav cells in the wedge with a line of fire from the gun.   (lanes.ts wedge)
pub fn wedge(g: &mut crate::game::Game, into: &mut JsSet<crate::Num>, from: V3, at: V3, e: Option<f64>) {
    let k = g.k.lanes;
    let (c, w, h) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let mut dx = at[0] - from[0];
    let mut dz = at[2] - from[2];
    let len = m::hyp2(dx, dz);
    if len < 0.5 {
        return;
    }
    dx /= len;
    dz /= len;
    let reach = m::max(k.reach_m, len + 2.0);
    let cos_h = m::cos(k.half_deg * m::PI / 180.0);
    let field = k.field_m;
    let x0 = m::max(0.0, m::floor((from[0] - reach) / c));
    let x1 = m::min(w - 1.0, m::floor((from[0] + reach) / c));
    let z0 = m::max(0.0, m::floor((from[2] - reach) / c));
    let z1 = m::min(h - 1.0, m::floor((from[2] + reach) / c));
    let muzzle: V3 = [from[0], 1.4, from[2]];
    let mut p: V3 = [0.0, 1.0, 0.0];
    let mut z = z0;
    while z <= z1 {
        let mut x = x0;
        while x <= x1 {
            let (cx, cz) = (x, z);
            x += 1.0;
            let i = cz * w + cx;
            if g.col.nav.blocked[i as usize] != 0 || into.has(crate::Num(i)) {
                continue;
            }
            let px = (cx + 0.5) * c - from[0];
            let pz = (cz + 0.5) * c - from[2];
            let d = m::hyp2(px, pz);
            if d < k.own_m || d > reach {
                continue;
            }
            if d > field {
                let along = px * dx + pz * dz;
                if along <= 0.0 {
                    continue;
                }
                if along / d < cos_h && m::abs(px * dz - pz * dx) > k.width_m {
                    continue;
                }
            }
            p[0] = px + from[0];
            p[2] = pz + from[2];
            if !crate::scene_collision::shot_blocked(g, muzzle, p, None, e) {
                into.add(crate::Num(i));
            }
        }
        z += 1.0;
    }
}

/// The floor of every lane the side knows.   (lanes.ts remask)
pub fn remask(g: &mut crate::game::Game) {
    let size = (g.col.nav.w * g.col.nav.h) as usize;
    s(g);
    let mut mask: Option<Vec<u8>> = None;
    for &l in st!(g).lanes.iter() {
        let r = &g.tab.lanes_lane[l];
        if !r.known {
            continue;
        }
        let Some(cells) = &r.cells else { continue };
        let into = mask.get_or_insert_with(|| vec![0u8; size]);
        for i in cells.iter() {
            into[i.0 as usize] = 1;
        }
    }
    let st = st!(g);
    st.mask = mask;
    st.ver += 1.0;
}

/// The floor of every lane THIS man knows of: the net's, and what he has seen and not yet had said. A copy, by the review's answer.   (lanes.ts maskOf)
pub fn mask_of(g: &mut crate::game::Game, gd: GuardId) -> Option<Vec<u8>> {
    let size = (g.col.nav.w * g.col.nav.h) as usize;
    s(g);
    let own = st!(g).lanes.iter().any(|&l| {
        let r = &g.tab.lanes_lane[l];
        !r.known && r.cells.is_some() && r.knowers.has(gd)
    });
    if !own {
        return st!(g).mask.clone();
    }
    let st = st!(g);
    if let Some(c) = st.own.get(gd) {
        if c.ver == st.ver {
            return Some(c.m.clone());
        }
    }
    let mut mask = match &st!(g).mask {
        Some(x) => x.clone(),
        None => vec![0u8; size],
    };
    for &l in st!(g).lanes.iter() {
        let r = &g.tab.lanes_lane[l];
        if r.known || !r.knowers.has(gd) {
            continue;
        }
        let Some(cells) = &r.cells else { continue };
        for i in cells.iter() {
            mask[i.0 as usize] = 1;
        }
    }
    let st = st!(g);
    let ver = st.ver;
    st.own.set(gd, StateOwnValue { ver, m: mask.clone() });
    Some(mask)
}

/// One of a lane's guns has a line of fire to a man standing at `p`.   (lanes.ts gunOn)
pub fn gun_on(g: &mut crate::game::Game, l: LanesLaneId, p: V3, wide: Option<f64>) -> bool {
    let wide = wide.unwrap_or(0.0);
    let q: V3 = [p[0], 1.1, p[2]];
    let reach = g.k.lanes.reach_m;
    let e = e_of(&g.tab.lanes_lane[l]);
    let guns = g.tab.lanes_lane[l].guns.clone(); // (a copy of its few guns: nothing asked below writes a lane)
    for gun in guns {
        let d = flat(gun, p);
        if d > reach || d < 0.3 {
            continue;
        }
        let muzzle: V3 = [gun[0], 1.4, gun[2]];
        if !crate::scene_collision::shot_blocked(g, muzzle, q, None, e) {
            return true;
        }
        if wide > 0.0 {
            // (the same gun a stride to either side, where there is floor to stand on)
            let sx = -(p[2] - gun[2]) / d * wide;
            let sz = (p[0] - gun[0]) / d * wide;
            for k in [1.0, -1.0] {
                let a: V3 = [gun[0] + sx * k, 1.4, gun[2] + sz * k];
                let i = cell_at(g, a);
                if i < 0.0 || g.col.nav.blocked[i as usize] != 0 || crate::scene_collision::shot_blocked(g, muzzle, a, None, None) {
                    continue;
                }
                if !crate::scene_collision::shot_blocked(g, a, q, None, e) {
                    return true;
                }
            }
        }
    }
    false
}

/// The lane this man knows of that a point of floor lies in.   (lanes.ts laneAt)
pub fn lane_at(g: &mut crate::game::Game, p: V3, gd: GuardId) -> Option<LanesLaneId> {
    let i = cell_at(g, p);
    if i < 0.0 {
        return None;
    }
    s(g);
    st!(g).lanes.iter().copied().find(|&l| {
        let r = &g.tab.lanes_lane[l];
        knows_row(r, gd) && matches!(&r.cells, Some(c) if c.has(crate::Num(i)))
    })
}

/// …or whose guns have a line to it (the floor beside the wedge is no cover).   (lanes.ts laneOn)
pub fn lane_on(g: &mut crate::game::Game, p: V3, gd: GuardId) -> Option<LanesLaneId> {
    if let Some(l) = lane_at(g, p, gd) {
        return Some(l);
    }
    let mut i = 0;
    while i < st!(g).lanes.len() {
        let x = st!(g).lanes[i];
        i += 1;
        let r = &g.tab.lanes_lane[x];
        if knows_row(r, gd) && r.cells.is_some() && gun_on(g, x, p, None) {
            return Some(x);
        }
    }
    None
}

/// which lane goes first when a side keeps too many: stray rounds before a lane, an unsaid one before one on the net   (lanes.ts cost, rank)
fn rank(x: &Lane) -> i32 {
    (if x.known { 2 } else { 0 }) + (if x.weight >= 1.0 { 1 } else { 0 })
}

/// What a side has just suffered: fire from `from` landed at `at`, on a man dropped (`down`) or past a living one.   (lanes.ts cost)
pub fn cost(g: &mut crate::game::Game, from: V3, at: V3, man: GuardId, down: bool, who: f64) {
    let k = g.k.lanes;
    let now = g.time;
    let len = flat(from, at);
    if len < k.min_m {
        return; // (shot at arm's length is a fight, not a lane)
    }
    let dx = (at[0] - from[0]) / len;
    let dz = (at[2] - from[2]) / len;
    let cos_m = m::cos((k.half_deg + 12.0) * m::PI / 180.0);
    let found = st!(g).lanes.iter().copied().find(|&x| {
        let r = &g.tab.lanes_lane[x];
        r.dx * dx + r.dz * dz > cos_m && r.guns.iter().any(|&gun| flat(gun, from) <= 5.0)
    });
    let l = match found {
        Some(l) => l,
        None => {
            let st = st!(g);
            st.serial += 1.0;
            let id = st.serial;
            let l = g.tab.lanes_lane.push(Lane {
                id,
                from: [from[0], 0.0, from[2]],
                at: [at[0], 0.0, at[2]],
                dx,
                dz,
                last_t: now,
                known_at: -1.0,
                push_until: -1.0,
                pin_by: -1.0,
                ..Lane::default()
            });
            let st = st!(g);
            st.lanes.push(l);
            st.stats.lanes += 1.0;
            l
        }
    };
    // its guns, least lately fired first (≤ 4: the stalest is forgotten); where the fire comes FROM is the one that fired last
    let mut gun = false;
    {
        let r = &mut g.tab.lanes_lane[l];
        match r.guns.iter().position(|&q| flat(q, from) < 1.5) {
            None => {
                r.guns.push([from[0], 0.0, from[2]]);
                if r.guns.len() > 4 {
                    r.guns.remove(0);
                }
                gun = true;
            }
            Some(gi) => {
                if gi < r.guns.len() - 1 {
                    let x = r.guns.remove(gi);
                    r.guns.push(x);
                }
            }
        }
        r.from = v3::copy(r.guns[r.guns.len() - 1]);
        r.weight += if down { 1.0 } else { k.round_k };
        if down {
            r.downs += 1.0;
        }
        r.last_t = now;
        r.who.add(crate::Num(who));
        r.moved.delete(crate::Num(who));
    }
    // (LANES.glass settled on: the gun in his hands as the round left is what a pane is to this lane)
    let c = match crate::seats::man_by_id(g, who) {
        Some(p) => Some(g.pl[p].ch),
        None => g.guards.iter().map(|&o| g.gd[o].ch).find(|&c| g.ch[c].id == who),
    };
    let e = match c {
        Some(c) => crate::glasspen::shot_energy(g, c, None),
        None => None,
    };
    if let Some(e) = e {
        let r = &mut g.tab.lanes_lane[l];
        if e > r.e {
            r.e = e;
        }
    }
    // THE GUNS HAVE MOVED: a gun that made another lane has now cost them from well away from it
    for &x in st!(g).lanes.iter() {
        let r = &mut g.tab.lanes_lane[x];
        if x != l && r.who.has(crate::Num(who)) && r.guns.iter().all(|&q| flat(q, from) > k.moved_m) {
            r.moved.add(crate::Num(who));
            if r.moved.len() >= r.who.len() {
                r.until = m::min(r.until, now + k.moved_secs);
            }
        }
    }
    if down && g.tab.lanes_lane[l].known {
        let what = format!("lane {}: another down in it ({})", g.tab.lanes_lane[l].id, g.tab.lanes_lane[l].downs);
        note(g, Some(man), &what);
    }
    {
        let r = &mut g.tab.lanes_lane[l];
        r.until = now + k.secs * m::min(3.0, m::max(1.0, r.downs));
    }
    if st!(g).lanes.len() as f64 > k.max {
        // (one too many: the one that matters least goes; the newcomer itself if that is it)
        let st = st!(g);
        let mut o = 0;
        for i in 1..st.lanes.len() {
            let (a, b) = (&g.tab.lanes_lane[st.lanes[i]], &g.tab.lanes_lane[st.lanes[o]]);
            if rank(a) < rank(b) || (rank(a) == rank(b) && a.last_t < b.last_t) {
                o = i;
            }
        }
        let gone = st.lanes.remove(o);
        lanes_gone(g);
        st!(g).stats.lapsed += 1.0;
        if g.tab.lanes_lane[gone].known {
            let what = format!("lane {}: dropped for a newer one", g.tab.lanes_lane[gone].id);
            note(g, None, &what);
        }
        if g.tab.lanes_lane[gone].cells.is_some() {
            remask(g);
        }
        if gone == l {
            return;
        }
    }
    if g.tab.lanes_lane[l].weight < 1.0 {
        return;
    }
    if g.tab.lanes_lane[l].cells.is_none() {
        // (the gun that fired now; what its other guns cover is read in by the roll, a wedge each half second)
        let e = {
            let r = &mut g.tab.lanes_lane[l];
            r.wedges.push([from[0], 0.0, from[2]]);
            r.wedges.push([at[0], 0.0, at[2]]);
            e_of(r)
        };
        let mut cells = JsSet::new();
        wedge(g, &mut cells, from, at, e);
        let r = &mut g.tab.lanes_lane[l];
        r.cells = Some(cells);
        for i in 0..r.guns.len() {
            let q = r.guns[i];
            if flat(q, from) >= 1.5 {
                r.wedges.push(v3::copy(q));
                r.wedges.push([at[0], 0.0, at[2]]);
            }
        }
        if r.wedges.len() > 2 {
            r.re = Some(LaneRe { set: r.cells.clone().unwrap(), i: 2.0 });
        }
        remask_or_bump(g, l);
    } else if g.tab.lanes_lane[l].wedges.len() < 16 && (gun || !landed_in_it(g, l, at)) {
        // (it reaches further, or wider, or from another gun, than they knew)
        let (e, mut cells) = {
            let r = &mut g.tab.lanes_lane[l];
            r.wedges.push([from[0], 0.0, from[2]]);
            r.wedges.push([at[0], 0.0, at[2]]);
            (e_of(r), r.cells.take().unwrap())
        };
        wedge(g, &mut cells, from, at, e);
        g.tab.lanes_lane[l].cells = Some(cells);
        remask_or_bump(g, l);
    }
    // who of them saw it: the man the rounds passed, and every fit mate with a line to where it happened
    if !g.tab.lanes_lane[l].known {
        let mut i = 0;
        while i < g.guards.len() {
            let o = g.guards[i];
            i += 1;
            if !fit(g, o) || g.tab.lanes_lane[l].saw.has(o) {
                continue;
            }
            if o == man || sees(g, o, at, k.see_m) {
                g.tab.lanes_lane[l].saw.set(o, now);
            }
        }
    }
}

/// where the fire landed is floor of the lane already   (lanes.ts cost, 329)
fn landed_in_it(g: &Game, l: LanesLaneId, at: V3) -> bool {
    let i = cell_at(g, at);
    g.tab.lanes_lane[l].cells.as_ref().unwrap().has(crate::Num(i))
}

/// a lane's floor has changed: the side's mask made again if it is on the net, else a man's own floor is out of date   (lanes.ts 328, 329)
fn remask_or_bump(g: &mut Game, l: LanesLaneId) {
    if g.tab.lanes_lane[l].known {
        remask(g);
    } else {
        st!(g).ver += 1.0;
    }
}

/// A fit man of the side has a line to what happened at `at`, inside `reach` and not behind his back.   (lanes.ts sees)
pub fn sees(g: &mut crate::game::Game, o: GuardId, at: V3, reach: f64) -> bool {
    let c = g.gd[o].ch;
    let pos = g.ch[c].pos;
    let d = flat(pos, at);
    if d >= reach {
        return false;
    }
    if d > 4.0 {
        let f = crate::character::character_forward(g, c);
        if ((at[0] - pos[0]) * f[0] + (at[2] - pos[2]) * f[2]) / d < -0.35 {
            return false;
        }
    }
    let eye = eye_of(g, o);
    !crate::scene_collision::segment_blocked(g, eye, [at[0], 0.9, at[2]], None, None, None)
}

/// The lane goes on the net.   (lanes.ts becomeKnown)
pub fn become_known(g: &mut crate::game::Game, l: LanesLaneId, who: GuardId) {
    let k = g.k.lanes;
    let now = g.time;
    {
        let r = &mut g.tab.lanes_lane[l];
        r.known = true;
        r.known_at = now;
        r.saw.clear();
        if r.cells.is_none() {
            r.cells = Some(JsSet::new());
        }
        r.round_at = now + k.think_secs;
        r.stun_at = now + k.think_secs;
        r.frag_at = now + k.frag_think; // (LANES.frag settled on)
    }
    remask(g);
    s(g).stats.known += 1.0;
    // (CUES.on settled on: the place is named wherever it has a name)
    let at = g.tab.lanes_lane[l].at;
    let place = crate::dialogue::place_name(g, at);
    let line = if place.is_empty() {
        String::from("they've got that way covered — stay out of their line!")
    } else {
        format!("they've got it covered — {}! stay out of their line!", place)
    };
    crate::game::game_say(g, who, &line, Some(true), None, None);
    let now = g.time;
    s(g).said_t = now;
    let r = &g.tab.lanes_lane[l];
    let what = format!(
        "lane {}: called — fire from ({}, {}) down to ({}, {}), {} cells, {} down",
        r.id,
        js::to_fixed(r.from[0], 1),
        js::to_fixed(r.from[2], 1),
        js::to_fixed(r.at[0], 1),
        js::to_fixed(r.at[2], 1),
        r.cells.as_ref().unwrap().len(),
        r.downs
    );
    note(g, Some(who), &what);
}

/// This frame's rounds (the shot events' own muzzle → impact segments): who of the side they dropped, whom they passed.   (lanes.ts readRounds)
pub fn read_rounds(g: &mut crate::game::Game) {
    let mut ei = 0;
    while ei < g.events.len() {
        let i = ei;
        ei += 1;
        let (key, kind, from, to, who, time) = {
            let e = &g.events[i];
            (e.serial, e.kind, e.from, e.to, e.who, e.time)
        };
        if !st!(g).seen.add(key) {
            continue;
        }
        if kind != crate::director::EventKind::Shot && kind != crate::director::EventKind::GuardShot {
            continue;
        }
        let (Some(a), Some(b), Some(who)) = (from, to, who) else { continue };
        let team = crate::teams::team_of_id(g, who);
        if !crate::teams::is_hostile(g, SIDE, team) {
            continue;
        }
        let sx = b[0] - a[0];
        let sz = b[2] - a[2];
        let l2 = sx * sx + sz * sz;
        if l2 < 4.0 {
            continue;
        }
        let mut gi = 0;
        while gi < g.guards.len() {
            let gd = g.guards[gi];
            gi += 1;
            if g.gd[gd].team != SIDE || g.gd[gd].unit.is_none() || g.gd[gd].held.is_some() {
                continue;
            }
            let dead = g.gd[gd].state == crate::game::GuardState::Dead;
            // (dropped by this very round — counted once, however many rounds were in the air — or standing)
            if dead && (st!(g).dropped.has(gd) || !(g.gd[gd].died_at >= time - 1e-6 && g.gd[gd].died_at <= g.time)) {
                continue;
            }
            let c = g.gd[gd].ch;
            let p = if dead {
                match g.ch[c].bones.get(crate::character::Bone::Hips) {
                    Some(x) => *x,
                    None => g.ch[c].pos,
                }
            } else {
                g.ch[c].pos
            };
            let len = m::sqrt(l2);
            let t = ((p[0] - a[0]) * sx + (p[2] - a[2]) * sz) / l2;
            if t * len < 1.0 || t * len > len + (if dead { 1.2 } else { 0.4 }) {
                continue; // (a round that stopped in him ends at his chest: his hips lie a little past it)
            }
            let d = m::hyp2(a[0] + sx * t - p[0], a[2] + sz * t - p[2]);
            if d > (if dead { 1.0 } else { g.k.lanes.near_m }) {
                continue;
            }
            if dead {
                st!(g).dropped.add(gd);
            } else {
                let u = m::min(1.0, m::max(0.0, t));
                // (a round that passed on the far side of a wall passed nobody)
                if crate::scene_collision::shot_blocked(g, [a[0] + sx * u, 1.2, a[2] + sz * u], [p[0], 1.2, p[2]], None, None) {
                    continue;
                }
            }
            // (LANES.roused settled on: not his yet, his senses get `reactSecs` to have it)
            if !dead && !roused(g, gd, time) {
                st!(g).near.push(StateNearItem { gd, from: v3::copy(a), at: v3::copy(p), who, t: time });
                continue;
            }
            cost(g, a, p, gd, dead, who);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// lanes.ts:303: who goes first when a side keeps too many
    #[test]
    fn rank_orders_stray_rounds_before_a_lane_before_one_on_the_net() {
        let stray = Lane { weight: 0.5, ..Lane::default() };
        let lane = Lane { weight: 1.0, ..Lane::default() };
        let said = Lane { weight: 0.5, known: true, ..Lane::default() };
        let both = Lane { weight: 2.0, known: true, ..Lane::default() };
        assert!(rank(&stray) < rank(&lane));
        assert!(rank(&lane) < rank(&said));
        assert!(rank(&said) < rank(&both));
    }

    /// a lane in the table and on the list, with one cell of floor
    fn add_lane(g: &mut Game, id: f64, known: bool, cell: f64) -> LanesLaneId {
        let mut cells = JsSet::new();
        cells.add(crate::Num(cell));
        let l = g.tab.lanes_lane.push(Lane { id, known, cells: Some(cells), ..Lane::default() });
        s(g).lanes.push(l);
        l
    }

    /// lanes.ts:226: the held row first, then the look; a lane off the list holds nobody, though its row stays in the table
    #[test]
    fn lane_of_goes_by_the_lanes_on_the_list() {
        let mut g = Game::blank();
        let gd = GuardId(0);
        assert!(lane_of(&mut g, gd).is_none());
        let a = add_lane(&mut g, 1.0, true, 0.0);
        let b = add_lane(&mut g, 2.0, true, 1.0);
        assert!(lane_of(&mut g, gd).is_none());
        s(&mut g).peek.set(gd, Peek { lane: 2.0, ..Peek::default() });
        assert!(lane_of(&mut g, gd) == Some(b));
        s(&mut g).held.set(gd, StateHeldValue { lane: 1.0, ..StateHeldValue::default() });
        assert!(lane_of(&mut g, gd) == Some(a));
        s(&mut g).lanes.remove(0);
        assert!(lane_of(&mut g, gd).is_none());
    }

    /// lanes.ts:268, 274: the net's floor for all; a man's own adds what only he knows, filed against the version
    #[test]
    fn masks_read_the_rows_through_the_list() {
        let mut g = Game::blank();
        g.col.nav.w = 2.0;
        g.col.nav.h = 2.0;
        let (gd, other) = (GuardId(0), GuardId(1));
        s(&mut g);
        remask(&mut g);
        assert!(st!(g).mask.is_none());
        assert_eq!(st!(g).ver, 1.0);
        add_lane(&mut g, 1.0, true, 1.0);
        let own = add_lane(&mut g, 2.0, false, 2.0);
        g.tab.lanes_lane[own].knowers.add(gd);
        remask(&mut g);
        assert_eq!(st!(g).ver, 2.0);
        assert_eq!(mask_of(&mut g, other), Some(vec![0, 1, 0, 0]));
        assert_eq!(mask_of(&mut g, gd), Some(vec![0, 1, 1, 0]));
        assert_eq!(st!(g).own.get(gd).unwrap().ver, 2.0);
        // (a filed copy is good until the version moves)
        g.tab.lanes_lane[own].cells.as_mut().unwrap().add(crate::Num(3.0));
        assert_eq!(mask_of(&mut g, gd), Some(vec![0, 1, 1, 0]));
        st!(g).ver += 1.0;
        assert_eq!(mask_of(&mut g, gd), Some(vec![0, 1, 1, 1]));
    }
}
