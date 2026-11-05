//! src/game/lanes.ts lines 1 to 439
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
    State { said_t: -100.0, t: -1.0, ..State::zeroed() }
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
    mine(g, gd) && g.time >= g.gd[gd].dazzled_until && g.time >= g.gd[gd].stagger_until
}

/// The rule runs: a room fight is live, the AI on, no tour. (LANES.on settled on.)   (lanes.ts live)
pub fn live(g: &mut crate::game::Game) -> bool {
    crate::roomfight::fight_live(g) && g.ai_enabled && !g.touring
}

/// Where he looks from.   (lanes.ts eyeOf)
pub fn eye_of(g: &mut crate::game::Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(crate::character::Bone::Head) {
        Some(p) => *p,
        None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]),
    }
}

/// The nav cell a point lies in, −1 off the grid.   (lanes.ts cellAt)
pub fn cell_at(g: &mut crate::game::Game, p: V3) -> f64 {
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
pub fn in_mask(g: &mut crate::game::Game, m: Option<&[u8]>, p: V3) -> bool {
    let Some(m) = m else { return false };
    let i = cell_at(g, p);
    i >= 0.0 && m.get(i as usize) == Some(&1)
}

/// The side has it on the net, or he has it himself.   (lanes.ts knows)
pub fn knows(g: &mut crate::game::Game, l: &crate::lanes::Lane, gd: GuardId) -> bool {
    knows_row(l, gd)
}

/// knows, for a row still in the table (`g` is not free to lend)
fn knows_row(l: &Lane, gd: GuardId) -> bool {
    l.known || l.knowers.has(gd)
}

/// An enemy he saw this close inside the last second: a fight already.   (lanes.ts closeFight)
pub fn close_fight(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.time - g.gd[gd].last_seen_t >= 1.0 {
        return false;
    }
    let fc = match g.gd[gd].foe {
        Some(crate::game::Foe::Player) => g.pl[g.player].ch,
        Some(crate::game::Foe::Guard(o)) => g.gd[o].ch,
        None => return false,
    };
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
pub fn lane_e(g: &mut crate::game::Game, l: &crate::lanes::Lane) -> Option<f64> {
    e_of(l)
}

/// laneE, for a row still in the table
fn e_of(l: &Lane) -> Option<f64> {
    if l.e > 0.0 { Some(l.e) } else { None }
}

/// His own senses have the fire since `t`.   (lanes.ts roused)
pub fn roused(g: &mut crate::game::Game, gd: GuardId, t: f64) -> bool {
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
pub fn up_id(g: &mut crate::game::Game, id_2: f64) -> bool {
    if crate::seats::is_man_id(g, id_2) {
        return match crate::seats::man_by_id(g, id_2) {
            Some(p) => g.ch[g.pl[p].ch].alive,
            None => false,
        };
    }
    g.guards.iter().any(|&o| g.ch[g.gd[o].ch].id == id_2 && g.gd[o].state != crate::game::GuardState::Dead)
}

/// His row in the room fight.   (lanes.ts defOf)
pub fn def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> {
    g.fight.defs.iter().copied().find(|&d| g.tab.roomfight_def[d].gd == gd)
}

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
    let st = s(g);
    let mut mask: Option<Vec<u8>> = None;
    for l in st.lanes.iter() {
        if !l.known {
            continue;
        }
        let Some(cells) = &l.cells else { continue };
        let into = mask.get_or_insert_with(|| vec![0u8; size]);
        for i in cells.iter() {
            into[i.0 as usize] = 1;
        }
    }
    st.mask = mask;
    st.ver += 1.0;
}

/// The floor of every lane THIS man knows of: the net's, and what he has seen and not yet had said.   (lanes.ts maskOf)
pub fn mask_of(g: &mut crate::game::Game, gd: GuardId) -> Option<Vec<u8>> {
    let size = (g.col.nav.w * g.col.nav.h) as usize;
    let st = s(g);
    let own = st.lanes.iter().any(|l| !l.known && l.cells.is_some() && l.knowers.has(gd));
    if !own {
        return st.mask.clone();
    }
    if let Some(c) = st.own.get(gd) {
        if c.ver == st.ver {
            return Some(c.m.clone());
        }
    }
    let mut mask = match &st.mask {
        Some(x) => x.clone(),
        None => vec![0u8; size],
    };
    for l in st.lanes.iter() {
        if l.known || !l.knowers.has(gd) {
            continue;
        }
        let Some(cells) = &l.cells else { continue };
        for i in cells.iter() {
            mask[i.0 as usize] = 1;
        }
    }
    let ver = st.ver;
    st.own.set(gd, StateOwnValue { ver, m: mask.clone() });
    Some(mask)
}

/// One of a lane's guns has a line of fire to a man standing at `p`.   (lanes.ts gunOn)
pub fn gun_on(g: &mut crate::game::Game, l: &crate::lanes::Lane, p: V3, wide: Option<f64>) -> bool {
    guns_on(g, &l.guns, e_of(l), p, wide.unwrap_or(0.0))
}

/// gunOn on what it reads of the lane, for a row still in the table: a copy of its few guns, not of its floor
fn guns_on(g: &mut Game, guns: &[V3], e: Option<f64>, p: V3, wide: f64) -> bool {
    let q: V3 = [p[0], 1.1, p[2]];
    let reach = g.k.lanes.reach_m;
    for &gun in guns {
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

/// The lane this man knows of that a point of floor lies in. A copy of the row: a caller that writes goes back to the table by its id.   (lanes.ts laneAt)
pub fn lane_at(g: &mut crate::game::Game, p: V3, gd: GuardId) -> Option<crate::lanes::Lane> {
    let i = cell_at(g, p);
    if i < 0.0 {
        return None;
    }
    s(g).lanes.iter().find(|l| knows_row(l, gd) && matches!(&l.cells, Some(c) if c.has(crate::Num(i)))).cloned()
}

/// …or whose guns have a line to it (the floor beside the wedge is no cover). A copy of the row.   (lanes.ts laneOn)
pub fn lane_on(g: &mut crate::game::Game, p: V3, gd: GuardId) -> Option<crate::lanes::Lane> {
    if let Some(l) = lane_at(g, p, gd) {
        return Some(l);
    }
    let mut i = 0;
    while i < s(g).lanes.len() {
        let x = &s(g).lanes[i];
        let (asked, guns, e) = (knows_row(x, gd) && x.cells.is_some(), x.guns.clone(), e_of(x));
        if asked && guns_on(g, &guns, e, p, 0.0) {
            return Some(s(g).lanes[i].clone());
        }
        i += 1;
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
    let found = st!(g).lanes.iter().position(|x| x.dx * dx + x.dz * dz > cos_m && x.guns.iter().any(|&gun| flat(gun, from) <= 5.0));
    let mut li = match found {
        Some(i) => i,
        None => {
            let st = st!(g);
            st.serial += 1.0;
            let lane = Lane {
                id: st.serial,
                from: [from[0], 0.0, from[2]],
                at: [at[0], 0.0, at[2]],
                dx,
                dz,
                t0: now,
                last_t: now,
                known_at: -1.0,
                push_until: -1.0,
                pin_by: -1.0,
                ..Lane::zeroed()
            };
            st.lanes.push(lane);
            st.stats.lanes += 1.0;
            st.lanes.len() - 1
        }
    };
    // its guns, least lately fired first (≤ 4: the stalest is forgotten); where the fire comes FROM is the one that fired last
    let mut gun = false;
    {
        let l = &mut st!(g).lanes[li];
        match l.guns.iter().position(|&q| flat(q, from) < 1.5) {
            None => {
                l.guns.push([from[0], 0.0, from[2]]);
                if l.guns.len() > 4 {
                    l.guns.remove(0);
                }
                gun = true;
            }
            Some(gi) => {
                if gi < l.guns.len() - 1 {
                    let x = l.guns.remove(gi);
                    l.guns.push(x);
                }
            }
        }
        l.from = v3::copy(l.guns[l.guns.len() - 1]);
        l.weight += if down { 1.0 } else { k.round_k };
        if down {
            l.downs += 1.0;
        }
        l.last_t = now;
        l.who.add(crate::Num(who));
        l.moved.delete(crate::Num(who));
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
        let l = &mut st!(g).lanes[li];
        if e > l.e {
            l.e = e;
        }
    }
    // THE GUNS HAVE MOVED: a gun that made another lane has now cost them from well away from it
    for (xi, x) in st!(g).lanes.iter_mut().enumerate() {
        if xi != li && x.who.has(crate::Num(who)) && x.guns.iter().all(|&q| flat(q, from) > k.moved_m) {
            x.moved.add(crate::Num(who));
            if x.moved.len() >= x.who.len() {
                x.until = m::min(x.until, now + k.moved_secs);
            }
        }
    }
    {
        let l = &st!(g).lanes[li];
        if down && l.known {
            let what = format!("lane {}: another down in it ({})", l.id, l.downs);
            note(g, Some(man), &what);
        }
    }
    {
        let l = &mut st!(g).lanes[li];
        l.until = now + k.secs * m::min(3.0, m::max(1.0, l.downs));
    }
    if st!(g).lanes.len() as f64 > k.max {
        // (one too many: the one that matters least goes; the newcomer itself if that is it)
        let st = st!(g);
        let mut o = 0;
        for i in 1..st.lanes.len() {
            let (a, b) = (&st.lanes[i], &st.lanes[o]);
            if rank(a) < rank(b) || (rank(a) == rank(b) && a.last_t < b.last_t) {
                o = i;
            }
        }
        let gone = st.lanes.remove(o);
        st.stats.lapsed += 1.0;
        if gone.known {
            let what = format!("lane {}: dropped for a newer one", gone.id);
            note(g, None, &what);
        }
        if gone.cells.is_some() {
            remask(g);
        }
        if o == li {
            return;
        }
        if o < li {
            li -= 1;
        }
    }
    if st!(g).lanes[li].weight < 1.0 {
        return;
    }
    if st!(g).lanes[li].cells.is_none() {
        // (the gun that fired now; what its other guns cover is read in by the roll, a wedge each half second)
        let e = {
            let l = &mut st!(g).lanes[li];
            l.wedges.push([from[0], 0.0, from[2]]);
            l.wedges.push([at[0], 0.0, at[2]]);
            e_of(l)
        };
        let mut cells = JsSet::new();
        wedge(g, &mut cells, from, at, e);
        let l = &mut st!(g).lanes[li];
        l.cells = Some(cells);
        let guns = l.guns.clone();
        for q in guns {
            if flat(q, from) >= 1.5 {
                l.wedges.push(v3::copy(q));
                l.wedges.push([at[0], 0.0, at[2]]);
            }
        }
        if l.wedges.len() > 2 {
            l.re = Some(LaneRe { set: l.cells.clone().unwrap(), i: 2.0 });
        }
        remask_or_bump(g, li);
    } else if st!(g).lanes[li].wedges.len() < 16 && (gun || !landed_in_it(g, li, at)) {
        // (it reaches further, or wider, or from another gun, than they knew)
        let (e, mut cells) = {
            let l = &mut st!(g).lanes[li];
            l.wedges.push([from[0], 0.0, from[2]]);
            l.wedges.push([at[0], 0.0, at[2]]);
            (e_of(l), l.cells.take().unwrap())
        };
        wedge(g, &mut cells, from, at, e);
        st!(g).lanes[li].cells = Some(cells);
        remask_or_bump(g, li);
    }
    // who of them saw it: the man the rounds passed, and every fit mate with a line to where it happened
    if !st!(g).lanes[li].known {
        let mut i = 0;
        while i < g.guards.len() {
            let o = g.guards[i];
            i += 1;
            if !fit(g, o) || st!(g).lanes[li].saw.has(o) {
                continue;
            }
            if o == man || sees(g, o, at, k.see_m) {
                st!(g).lanes[li].saw.set(o, now);
            }
        }
    }
}

/// where the fire landed is floor of the lane already   (lanes.ts cost, 314)
fn landed_in_it(g: &mut Game, li: usize, at: V3) -> bool {
    let i = cell_at(g, at);
    st!(g).lanes[li].cells.as_ref().unwrap().has(crate::Num(i))
}

/// a lane's floor has changed: the side's mask made again if it is on the net, else a man's own floor is out of date   (lanes.ts 313, 314, 417)
fn remask_or_bump(g: &mut Game, li: usize) {
    if st!(g).lanes[li].known {
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

/// The lane goes on the net. `l` is a copy lent by the caller (who writes it back): the table's row is set to it before the mask is made.   (lanes.ts becomeKnown)
pub fn become_known(g: &mut crate::game::Game, l: &mut crate::lanes::Lane, who: GuardId) {
    let k = g.k.lanes;
    l.known = true;
    l.known_at = g.time;
    l.saw.clear();
    if l.cells.is_none() {
        l.cells = Some(JsSet::new());
    }
    l.round_at = g.time + k.think_secs;
    l.stun_at = g.time + k.think_secs;
    l.frag_at = g.time + k.frag_think;
    let id = l.id;
    if let Some(r) = s(g).lanes.iter_mut().find(|x| x.id == id) {
        *r = l.clone();
    }
    remask(g);
    s(g).stats.known += 1.0;
    // (CUES.on settled on: the place is named wherever it has a name)
    let place = crate::dialogue::place_name(g, l.at);
    let line = if place.is_empty() {
        String::from("they've got that way covered — stay out of their line!")
    } else {
        format!("they've got it covered — {}! stay out of their line!", place)
    };
    crate::game::game_say(g, who, &line, Some(true), None, None);
    let now = g.time;
    s(g).said_t = now;
    let what = format!(
        "lane {}: called — fire from ({:.1}, {:.1}) down to ({:.1}, {:.1}), {} cells, {} down",
        l.id,
        l.from[0],
        l.from[2],
        l.at[0],
        l.at[2],
        l.cells.as_ref().unwrap().len(),
        l.downs
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

/// Once a frame, after the room fight's own tick and before anybody walks.   (lanes.ts lanesTick)
pub fn lanes_tick(g: &mut crate::game::Game, dt: f64) {
    if !live(g) {
        let busy = match &g.m.lanes.states {
            Some(st) => !st.lanes.is_empty() || !st.held.is_empty() || !st.peek.is_empty() || !st.reloads.is_empty() || !st.near.is_empty(),
            None => false,
        };
        if busy {
            lanes_reset(g);
        }
        return;
    }
    let k = g.k.lanes;
    if g.time < s(g).t {
        // (the clock has gone back: a load — nothing kept across it but what a restore puts back)
        lanes_reset(g);
        s(g);
    }
    st!(g).t = g.time;
    if st!(g).skip {
        // (the events a load put back were read before the file was made)
        st!(g).skip = false;
        for i in 0..g.events.len() {
            let key = g.events[i].serial;
            let st = st!(g);
            st.seen.add(key);
            st.banged.add(key);
        }
    }
    // the men the gate stopped last frame: planted where they stand, gun on the lane's mouth (between frames of walking, never inside a man's own step)
    if !st!(g).due.is_empty() {
        let due = core::mem::take(&mut st!(g).due);
        for d in due {
            plant_short(g, d.gd, d.lane, d.look);
        }
    }
    // (their own charges in the air: a side knows whose can it was that burst)
    let mut ii = 0;
    while ii < g.items.items.len() {
        let it = g.items.items[ii];
        ii += 1;
        if crate::throwables::item_kind(g, it) == crate::throwables::ItemKind::Flash && !crate::throwables::item_fired(g, it) && !st!(g).cans.contains(&it) {
            let who = crate::throwables::item_who(g, it);
            if crate::teams::team_of_id(g, who) == SIDE {
                st!(g).cans.push(it);
            }
        }
    }
    read_rounds(g);
    bangs(g);
    if !st!(g).near.is_empty() {
        // rounds that passed a man who had not noticed them: his once his senses have them, nobody's after a beat
        let near = core::mem::take(&mut st!(g).near);
        let mut kept = Vec::new();
        for n in near {
            if !mine(g, n.gd) {
                continue; // (dropped since: the round that dropped him is the one that counts)
            }
            if roused(g, n.gd, n.t) {
                cost(g, n.from, n.at, n.gd, false, n.who);
                continue;
            }
            if g.time - n.t < k.react_secs {
                kept.push(n);
            }
        }
        st!(g).near = kept;
    }
    if !st!(g).cans.is_empty() {
        // (a spent can lies on the floor a minute: it is nobody's bang any more)
        let cans = core::mem::take(&mut st!(g).cans);
        let mut kept = Vec::new();
        for it in cans {
            if !crate::throwables::item_fired(g, it) && g.items.items.contains(&it) {
                kept.push(it);
            }
        }
        st!(g).cans = kept;
    }
    let mut i = 0;
    while i < st!(g).lanes.len() {
        let li = i;
        i += 1;
        if !st!(g).lanes[li].known {
            let mut at = 0;
            while let Some(o) = st!(g).lanes[li].saw.next_key(&mut at) {
                let t = *st!(g).lanes[li].saw.get(o).unwrap();
                if !mine(g, o) {
                    st!(g).lanes[li].saw.delete(o); // (cut down before he could act on it or say it)
                    continue;
                }
                if !fit(g, o) {
                    continue;
                }
                if !st!(g).lanes[li].knowers.has(o) && g.time >= t + k.react_secs {
                    st!(g).lanes[li].knowers.add(o);
                    st!(g).ver += 1.0;
                }
                if g.time >= t + k.call_secs {
                    with_lane(g, li, |g, l| become_known(g, l, o));
                    break;
                }
            }
        }
        // a lane that cost a man nobody lived to tell of: a fit man who comes on the place (the bodies) knows what it means
        if !st!(g).lanes[li].known && st!(g).lanes[li].weight >= 1.0 {
            let lane_at_ = st!(g).lanes[li].at;
            let mut gi = 0;
            while gi < g.guards.len() {
                let o = g.guards[gi];
                gi += 1;
                if fit(g, o) && !st!(g).lanes[li].saw.has(o) && sees(g, o, lane_at_, k.body_m) {
                    st!(g).lanes[li].saw.set(o, g.time);
                }
            }
        }
        let now = g.time;
        let l = &mut st!(g).lanes[li];
        if l.push_until >= 0.0 && now >= l.push_until {
            l.push_until = -1.0;
        }
        if matches!(&l.can, Some(c) if now - c.t > 5.0) {
            l.can = None;
        }
    }
    let old = core::mem::take(&mut st!(g).lanes);
    let n0 = old.len();
    let mut kept = Vec::with_capacity(n0);
    for l in old {
        // (a few rounds past a man and no more: forgotten in ten seconds)
        let keep = if l.weight >= 1.0 { g.time < l.until } else { g.time - l.last_t < 10.0 };
        if !keep && !l.known && l.weight >= 1.0 {
            st!(g).stats.unknown += 1.0;
        }
        if !keep && l.known {
            let what = format!("lane {}: forgotten ({} down, last round {:.0} s ago)", l.id, l.downs, g.time - l.last_t);
            note(g, None, &what);
        }
        if keep {
            kept.push(l);
        }
    }
    let n1 = kept.len();
    st!(g).lanes = kept;
    if n1 != n0 {
        st!(g).stats.lapsed += (n0 - n1) as f64;
        remask(g);
    }
    if !st!(g).peek.is_empty() {
        // (a step only ever ends its own man's row, so each key still has its row when it is reached)
        for gd in st!(g).peek.keys() {
            peek_step(g, gd);
        }
    }
    // a magazine change this rule started behind his corner: the guard code counts a reload down only for a man with somebody to shoot, so it is counted here.
    // ONE COUNT A FRAME: a clock that has moved since this rule last left it is being counted by somebody else, and is theirs while it keeps moving
    if !st!(g).reloads.is_empty() {
        let list: Vec<(GuardId, f64)> = st!(g).reloads.iter().map(|(gd, left)| (gd, *left)).collect();
        for (gd, left) in list {
            if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() || g.gd[gd].reload_t < 0.0 {
                st!(g).reloads.delete(gd); // (in somebody's arm his hands are not on a magazine)
                continue;
            }
            if g.gd[gd].reload_t == left && g.time - g.gd[gd].last_seen_t > 0.5 {
                crate::guards::tick_reload_beat(g, gd, dt);
            }
            if g.gd[gd].reload_t < 0.0 {
                st!(g).reloads.delete(gd);
            } else {
                st!(g).reloads.set(gd, g.gd[gd].reload_t);
            }
        }
    }
    st!(g).tick_t -= dt;
    if st!(g).tick_t > 0.0 {
        return;
    }
    st!(g).tick_t = 0.5;
    let mut hi = 0;
    while let Some(gd) = st!(g).held.next_key(&mut hi) {
        let h = *st!(g).held.get(gd).unwrap();
        let def = def_of(g, gd);
        let keep = mine(g, gd) && st!(g).lanes.iter().any(|l| l.id == h.lane);
        let d = match def {
            Some(d) if keep && g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Free => d,
            _ => {
                st!(g).held.delete(gd);
                continue;
            }
        };
        let mode = g.tab.roomfight_def[d].mode;
        let pos = g.ch[g.gd[gd].ch].pos;
        if h.aimed {
            if mode != crate::roomfight::DefMode::Hold || flat(pos, h.at) > 2.5 {
                st!(g).held.delete(gd); // (somebody else has moved him since: no longer this rule's man)
                continue;
            }
            // …moved a step all the same onto floor a gun of the lane has, or left in a doorway: out of it again (LANES.stopClear settled on)
            if fit(g, gd)
                && !close_fight(g, gd)
                && !pushing(g, gd)
                && !flanking(g, gd)
                && !st!(g).flank_of.has(gd)
                && !st!(g).peek.has(gd)
                && g.time - g.gd[gd].last_seen_t > 1.0
                && g.time - st!(g).back_t.get(gd).copied().unwrap_or(-100.0) > 6.0
            {
                let row = st!(g).lanes.iter().position(|x| x.id == h.lane);
                if let Some(li) = row {
                    let l = &st!(g).lanes[li];
                    let (known, guns, e) = (knows_row(l, gd), l.guns.clone(), e_of(l));
                    if known && (crate::doorway::doorway_in_way(g, pos) || guns_on(g, &guns, e, pos, k.shift_m)) {
                        let l = st!(g).lanes[li].clone();
                        if !back_out(g, gd, &l) {
                            st!(g).back_t.set(gd, g.time); // (no cover found: asked again in six seconds, not every tick)
                        }
                    }
                }
            }
            continue;
        }
        if mode == crate::roomfight::DefMode::Move {
            // (on his way to cover — or sent elsewhere meanwhile: then the gate has him again like anybody)
            let on_way = match &g.gd[gd].script {
                Some(script) => matches!(script.goal, Some(sg) if !(flat(sg, h.at) > 0.8)),
                None => false,
            };
            if !on_way {
                st!(g).held.delete(gd);
            }
            continue;
        }
        if let Some(row) = st!(g).held.get_mut(gd) {
            row.aimed = true;
            row.at = v3::copy(pos);
        }
        // (backed out to cover and arrived: his gun on the lane's mouth — unless he has a man in his sights, which is the fight's)
        if fit(g, gd) && g.time - g.gd[gd].last_seen_t > 1.0 {
            crate::roomfight::room_fight_lane_stop(g, gd, h.look);
            st!(g).stats.held += 1.0;
        }
    }
    // a lane's floor is kept true to the doors and walls as they stand: one wedge read again each half second, its floor swapped when the last is in
    let ls: Vec<usize> = {
        let st = st!(g);
        (0..st.lanes.len()).filter(|&i| st.lanes[i].cells.is_some() && st.lanes[i].wedges.len() >= 2).collect()
    };
    if !ls.is_empty() {
        let st = st!(g);
        let li = ls[js::rem(st.roll, ls.len() as f64) as usize];
        st.roll += 1.0;
        let l = &mut st.lanes[li];
        let wedges = l.wedges.len() as f64;
        let e = e_of(l);
        let mut r = l.re.take().unwrap_or_else(LaneRe::zeroed);
        if r.i + 1.0 < wedges {
            let (from, at) = (l.wedges[r.i as usize], l.wedges[r.i as usize + 1]);
            wedge(g, &mut r.set, from, at, e);
            r.i += 2.0;
        }
        if r.i + 1.0 >= wedges {
            st!(g).lanes[li].cells = Some(r.set);
            remask_or_bump(g, li);
        } else {
            st!(g).lanes[li].re = Some(r);
        }
    }
    // a gun that made a lane and is down has moved for good
    for i in 0..st!(g).lanes.len() {
        let mut cut = false;
        for id in st!(g).lanes[i].who.keys() {
            if !up_id(g, id.0) {
                let l = &mut st!(g).lanes[i];
                l.who.delete(id);
                l.moved.delete(id);
                cut = true;
            }
        }
        let now = g.time;
        let l = &mut st!(g).lanes[i];
        if cut && l.moved.len() >= l.who.len() {
            l.until = m::min(l.until, now + k.moved_secs);
        }
    }
    for i in 0..g.fight.defs.len() {
        let d = g.fight.defs[i];
        let (gd, station) = (g.tab.roomfight_def[d].gd, g.tab.roomfight_def[d].station);
        let st = st!(g);
        if !st.post.has(gd) {
            st.post.set(gd, v3::copy(station));
        }
    }
    if !st!(g).flank_of.is_empty() {
        // (the lane forgotten under them)
        let st = st!(g);
        let list: Vec<(GuardId, f64)> = st.flank_of.iter().map(|(gd, f)| (gd, f.lane)).collect();
        for (gd, lane) in list {
            if !st.lanes.iter().any(|l| l.id == lane && l.known) {
                st.flank_of.delete(gd);
            }
        }
    }
    let mut i = 0;
    while i < st!(g).lanes.len() {
        let li = i;
        i += 1;
        if !st!(g).lanes[li].known || g.time < st!(g).lanes[li].push_until {
            continue;
        }
        let row = st!(g).lanes[li].clone();
        let at = held_at(g, &row);
        // (LANES.stun, round, frag and peek settled on)
        if g.time >= st!(g).lanes[li].stun_at && !at.is_empty() {
            with_lane(g, li, |g, l| stun(g, l, &at));
        }
        if g.time >= st!(g).lanes[li].round_at && st!(g).lanes[li].can.is_none() {
            let st = st!(g);
            let free: Vec<GuardId> = if !st.flank_of.is_empty() { at.iter().copied().filter(|&x| !st.flank_of.has(x)).collect() } else { at.clone() };
            if free.len() >= 2 {
                with_lane(g, li, |g, l| round(g, l, &free));
            }
        }
        if st!(g).lanes[li].frag_at > 0.0 && g.time >= st!(g).lanes[li].frag_at && st!(g).lanes[li].can.is_none() {
            with_lane(g, li, |g, l| frag_out(g, l, &at));
        }
        if !st!(g).flank_of.is_empty() {
            with_lane(g, li, |g, l| flank_tick(g, l));
        }
        let l = &st!(g).lanes[li];
        let (peek_at, none, last_t, known_at) = (l.peek_at, l.can.is_none(), l.last_t, l.known_at);
        if g.time >= peek_at && none && g.time - last_t > k.peek_quiet && g.time - known_at > k.peek_quiet {
            with_lane(g, li, |g, l| peek_start(g, l));
        }
    }
    // a man standing where a lane's guns have him — rounds still coming down it, or the guns in his own sight — and not on his own post: out of the line
    let mut di = 0;
    while di < g.fight.defs.len() {
        let d = g.fight.defs[di];
        di += 1;
        let gd = g.tab.roomfight_def[d].gd;
        if g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Move
            || !fit(g, gd)
            || close_fight(g, gd)
            || pushing(g, gd)
            || flanking(g, gd)
            || st!(g).peek.has(gd)
            || g.time - st!(g).back_t.get(gd).copied().unwrap_or(-100.0) < 6.0
        {
            continue;
        }
        let pos = g.ch[g.gd[gd].ch].pos;
        // (his own post: the room fight's to defend, not this rule's to move)
        if g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold && flat(pos, st!(g).post.get(gd).copied().unwrap_or(pos)) < 3.0 {
            continue;
        }
        let Some(l) = lane_on(g, pos, gd) else { continue };
        if g.time - l.last_t > k.hot_secs && g.time - g.gd[gd].last_seen_t > 1.0 {
            continue;
        }
        back_out(g, gd, &l);
    }
}

/// a function that takes a lane to write: a copy of the row is lent, and is the row again afterwards (found by its id: the row may be gone). The row stays in the table for what the call reads there
fn with_lane(g: &mut Game, i: usize, f: impl FnOnce(&mut Game, &mut Lane)) {
    let mut l = st!(g).lanes[i].clone();
    f(g, &mut l);
    let id = l.id;
    if let Some(r) = st!(g).lanes.iter_mut().find(|x| x.id == id) {
        *r = l;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// lanes.ts:303: who goes first when a side keeps too many
    #[test]
    fn rank_orders_stray_rounds_before_a_lane_before_one_on_the_net() {
        let stray = Lane { weight: 0.5, ..Lane::zeroed() };
        let lane = Lane { weight: 1.0, ..Lane::zeroed() };
        let said = Lane { weight: 0.5, known: true, ..Lane::zeroed() };
        let both = Lane { weight: 2.0, known: true, ..Lane::zeroed() };
        assert!(rank(&stray) < rank(&lane));
        assert!(rank(&lane) < rank(&said));
        assert!(rank(&said) < rank(&both));
    }
}
