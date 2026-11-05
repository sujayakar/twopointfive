//! src/game/squadthrow.ts lines 1 to 421
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- private helpers ----

/// a body's feet
fn pos_of(g: &Game, gd: GuardId) -> V3 {
    g.ch[g.gd[gd].ch].pos
}

/// the player's feet
fn sam_pos(g: &Game) -> V3 {
    g.ch[g.pl[g.player].ch].pos
}

/// `x.toFixed(2)`: the exact decimal, a tie going to the larger magnitude; a negative's sign is kept however small, −0 has none
fn to_fixed2(x: f64) -> String {
    let a = m::abs(x);
    let eighths = a * 8.0;
    let tie = m::floor(eighths) == eighths && js::rem(eighths, 2.0) == 1.0;   // (of two decimals only an odd number of eighths lies exactly between two)
    let s = if tie {
        let n = (a * 100.0 + 0.5) as u64;
        format!("{}.{:02}", n / 100, n % 100)
    } else {
        format!("{:.2}", a)
    };
    if x < 0.0 { format!("-{}", s) } else { s }
}

/// a whole number in a key, as JavaScript writes it (−0 is "0")
fn int_str(x: f64) -> String {
    format!("{}", x + 0.0)
}

/// (squadFragCatches q) a point on a quarter-metre grid
fn quarter(p: V3) -> String {
    format!("{},{}", int_str(m::round(p[0] * 4.0)), int_str(m::round(p[2] * 4.0)))
}

/// this view's row asks `tossing`: `states.get(g)`, which makes no row, and `tossing` takes no row out of `g` while it holds `g`
fn tossing_here(g: &mut Game, gd: GuardId) -> bool {
    let key = crate::seats::view_key(g);
    let t = g.m.squadthrow.states.get(key).and_then(|s| s.last.get(gd).copied()).unwrap_or(-1e9);
    toss_window(g, t)
}

/// `s?.walks.has(gd)`: on his way to a throw already (makes no row)
fn walking_here(g: &mut Game, gd: GuardId) -> bool {
    let key = crate::seats::view_key(g);
    g.m.squadthrow.states.get(key).map_or(false, |s| s.walks.has(gd))
}

/// his last throw at `t` is not long enough ago
fn toss_window(g: &Game, t: f64) -> bool {
    g.time >= t && g.time - t < g.k.squadthrow.again_secs
}

/// (Lob.clean) a search's test of a throw, by what its closure captured: a frag thrown so catches nobody of ours (the player apart); a stun always passes.
/// Plan: squadThrowPlan's, the thrower where he will stand, the verdict kept unless `fresh`. LookAgain: walkTick's own (squadthrow.ts:496), asked afresh
fn lob_clean(g: &mut Game, clean: &LobClean, t: &SquadThrower) -> bool {
    match *clean {
        LobClean::Plan { kind, at, fresh } => {
            if kind != SquadThrowKind::Frag { return true; }
            let stands = match t.post { Some(p) => p, None => pos_of(g, t.gd) };
            squad_frag_catches(g, t.rest, at, Some(SquadFragCatchesMoved { gd: t.gd, at: stands }), Some(false), Some(!fresh), None).is_none()
        }
        LobClean::LookAgain { kind, at, gd } => {
            if kind != SquadThrowKind::Frag { return true; }
            let stands = t.post.expect("a walk search's throw has a post");
            squad_frag_catches(g, t.rest, at, Some(SquadFragCatchesMoved { gd, at: stands }), Some(false), None, None).is_none()
        }
    }
}

/// throwTo with the search's count lent: it spends flights from it (`Lob.lim` and throw_to's `lim` are two types of one shape)
fn throw_to_lim(g: &mut Game, gd: GuardId, at: V3, kind: SquadThrowKind, stood: Option<V3>, walls: &[V3], lim: &mut LobLim) -> Option<crate::throwplan::ThrowTo> {
    let mut count = crate::throwplan::ThrowToLim { n: lim.n };
    let th = crate::throwplan::throw_to(g, gd, at, kind, stood, Some(walls), Some(&mut count));
    lim.n = count.n;
    th
}

/// `o.owns(gd)` of a row of g.orders.owners: the one closure there is (orders.ts:631, the night breach's drill), asked by the row's name
fn owner_owns(g: &mut Game, owner: &str, gd: GuardId) -> bool {
    match owner {
        "nightbreach" => g.night_breach.active && crate::nightbreach::night_breach_drill_ops(g).contains(&gd) && crate::nightbreach::night_breach_owns_pair(g),
        _ => false,
    }
}

/// (walkSpot side) a short step to a side of a cell
fn side(n: i32) -> (f64, f64) {
    (if n == 0 { 0.3 } else if n == 1 { -0.3 } else { 0.0 }, if n == 2 { 0.3 } else if n == 3 { -0.3 } else { 0.0 })
}

/// (walkSpot mayDuck) he is outside what it kills at, on a search that counts on a duck
fn may_duck(g: &Game, has_lob: bool, pts: &[V3], px: f64, pz: f64) -> bool {
    has_lob && g.k.squadthrow.duck && !pts.iter().any(|l| m::hyp2(l[0] - px, l[2] - pz) < g.k.squadthrow.duck_min_m)
}

/// (walkSpot inOwn) standing there he would be in his own frag / caught by his own canister (`ducks`: the flood's count of duck asks left)
fn in_own(g: &mut Game, kind: SquadThrowKind, pts: &[V3], px: f64, pz: f64, gd: GuardId, has_lob: bool, ducks: &mut f64) -> bool {
    if kind == SquadThrowKind::Frag {
        if !frag_in(g, pts, [px, 0.0, pz], None, None) { return false; }
        if !may_duck(g, has_lob, pts, px, pz) { return true; }
        let had = *ducks;
        *ducks -= 1.0;
        if !(had > 0.0) { return true; }
        let late = g.k.squadthrow.duck_throw_secs;
        !frag_clears(g, gd, [px, 0.0, pz], pts, Some(late), None)
    } else {
        crate::effects::flags().dazzle && stun_in(g, pts, [px, 1.65, pz], false)
    }
}

/// (walkSpot wallSeen) a wall by the spot, a metre off or more, that the hand has a line to
fn wall_seen(g: &mut Game, walls: &[V3], h: V3) -> bool {
    for w in walls {
        if m::hyp2(w[0] - h[0], w[2] - h[2]) >= 1.0 && !crate::scene_collision::solid_blocked(g, h, *w, None) { return true; }
    }
    false
}

/// (walkSpot arcAt) an arc for him standing on that cell that still holds a short step to any side of it; worked: or the throw he works out there, the SAME throw made from each side still going off by the spot
fn arc_at(g: &mut Game, gd: GuardId, post: V3, straight: bool, worked: bool, at: V3, kind: SquadThrowKind, roll_k: Option<f64>, walls: &[V3], lim: &mut LobLim) -> Option<Arc> {
    let arc = if straight { crate::squadsmoke::squad_arc_to(g, gd, at, roll_k, Some(post)) } else { None };
    if let Some(arc) = arc {
        let mut ok = true;
        let mut n = 0;
        while n < 4 && ok {
            let (ox, oz) = side(n);
            g.stats.squadthrow.arcs += 1;
            ok = crate::squadsmoke::squad_arc_to(g, gd, at, roll_k, Some([post[0] + ox, 0.0, post[2] + oz])).is_some();
            n += 1;
        }
        if ok { return Some(Arc { from: arc.from, sol: arc.sol, rest: arc.rest }); }
    }
    if !worked { return None; }
    let th = throw_to_lim(g, gd, at, kind, Some(post), walls, lim)?;
    for n in 0..4 {
        let (ox, oz) = side(n);
        lim.n -= 1.0;
        let end = crate::throwplan::throw_flight(g, kind, [th.from[0] + ox, th.from[1], th.from[2] + oz], th.sol.v0, None).end;
        if !crate::throwplan::throw_hits(g, end, at, Some(0.4)) { return None; }
    }
    Some(Arc { from: th.from, sol: th.sol, rest: th.rest })
}

/// (walkSpot duckHolds) a cell he throws a frag from only because he can duck from it: he can from a short step to any of eight sides of it too
fn duck_holds(g: &mut Game, gd: GuardId, arc: &Arc, px: f64, pz: f64, at: V3, kind: SquadThrowKind, has_lob: bool) -> bool {
    if kind != SquadThrowKind::Frag || !has_lob { return true; }
    let pts = frag_pts(arc.rest, at);
    if !frag_in(g, &pts, [px, 0.0, pz], None, None) { return true; }
    for n in 0..8 {
        let q: V3 = if n < 4 {
            let (ox, oz) = side(n);
            [px + ox, 0.0, pz + oz]
        } else {
            [px + (if n & 1 != 0 { 0.25 } else { -0.25 }), 0.0, pz + (if n & 2 != 0 { 0.25 } else { -0.25 })]
        };
        if frag_in(g, &pts, q, None, None) {
            let late = g.k.squadthrow.duck_throw_secs;
            if !frag_clears(g, gd, q, &pts, Some(late), None) { return false; }
        }
    }
    true
}

/// (walkSpot wayOk) the way the ORDERS would walk him there crosses no leaf and no room nobody of the squad has been in
fn way_ok(g: &mut Game, gd: GuardId, post: V3, own: f64, leaf: &[i32], gen: i32, c: f64, w: i32, h: i32) -> bool {
    let l = walk_line(g, gd, post);
    for n in 1..l.len() {
        let (a, e) = (l[n - 1], l[n]);
        let len = v3::dist_xz(a, e);
        let steps = m::max(1.0, m::ceil(len / 0.25));
        let mut i = 1.0;
        while i <= steps {
            let f = i / steps;
            let b: V3 = [a[0] + (e[0] - a[0]) * f, 0.0, a[2] + (e[2] - a[2]) * f];
            let (x, z) = (m::floor(b[0] / c), m::floor(b[2] / c));
            if x < 0.0 || z < 0.0 || x >= w as f64 || z >= h as f64 { return false; }
            if leaf[(z as i32 * w + x as i32) as usize] == gen { return false; }
            let r = crate::cleared::cleared_rooms_room_at(g, b);
            if js::truthy(r) && r != own && !crate::cleared::cleared_rooms_is_known(g, r) { return false; }
            i += 1.0;
        }
    }
    true
}

/// (walkSpot flood) one flood of the walk grid out from `who`: the straight arc, or (`worked`) a throw to work out. `nav`: cell, W, H
fn flood(g: &mut Game, who: &[GuardId], home: &[f64], ours: &[(Option<GuardId>, V3)], at: V3, kind: SquadThrowKind, worked: bool, max: f64, roll_k: Option<f64>, walls: &[V3], lob: &mut Option<&mut Lob>, lim: &mut LobLim, sc: &mut Scratch, nav: (f64, i32, i32)) -> Option<SquadThrower> {
    let (c, w, h) = nav;
    let has_lob = lob.is_some();
    let (mut head, mut tail) = (0usize, 0usize);
    let mut tries = if worked { g.k.squadthrow.lob_cells } else { g.k.squadthrow.tries };
    sc.gen_ += 1.0;
    let gen = sc.gen_ as i32;
    let mut ducks = if has_lob { g.k.squadthrow.duck_cells } else { 0.0 };
    for (k, &gd) in who.iter().enumerate() {
        let p = pos_of(g, gd);
        let (fx_, fz) = (m::floor(p[0] / c), m::floor(p[2] / c));
        if fx_ < 0.0 || fz < 0.0 || fx_ >= w as f64 || fz >= h as f64 { continue; }
        let i = (fz as i32 * w + fx_ as i32) as usize;
        if sc.seen[i] == gen { continue; }
        sc.seen[i] = gen;
        sc.dep[i] = 0;
        sc.who[i] = k as u8;
        sc.q[tail] = i as i32;
        tail += 1;
    }
    for di in 0..g.doors.list.len() {   // (a door's leaf where it stands, shut or open: the cells under it are not walked)
        let d = g.doors.list[di];
        if crate::doors::door_off(g, d) { continue; }
        let (hx, hz, closed, width) = { let def = &g.dr[d].def; (def.hinge[0], def.hinge[1], def.closed_dir, def.width) };
        let th = closed + crate::doors::door_angle(g, d);
        let (ux, uz) = (m::cos(th), m::sin(th));
        let mut u = 0.2;
        while u <= width {
            let (x, z) = (m::floor((hx + ux * u) / c), m::floor((hz + uz * u) / c));
            if x >= 0.0 && z >= 0.0 && x < w as f64 && z < h as f64 { sc.leaf[(z as i32 * w + x as i32) as usize] = gen; }
            u += 0.2;
        }
    }
    let to: V3 = [at[0], at[1] + 1.6, at[2]];
    let floor: V3 = [at[0], at[1] + 0.15, at[2]];
    let over: V3 = [at[0], at[1] + 1.0, at[2]];
    let frag_at = frag_pts(at, at);
    let stun_at = stun_pts(at, at);
    while head < tail && tries > 0.0 && !(worked && lim.n <= 0.0) {
        let i = sc.q[head] as usize;
        head += 1;
        let d = sc.dep[i];
        let k = sc.who[i] as usize;
        let gd = who[k];
        let x = i as i32 % w;
        let z = (i as i32 - x) / w;
        let px = (x as f64 + 0.5) * c;
        let pz = (z as f64 + 0.5) * c;
        let eye: V3 = [px, 1.6, pz];
        let hand: V3 = [px, 1.3, pz];
        g.stats.squadthrow.cells += 1;
        let dist = m::hyp2(px - at[0], pz - at[2]);
        let free = d > 0
            && dist >= g.k.squadthrow.min_m
            && dist <= g.k.squadthrow.max_m
            && !ours.iter().any(|o| o.0 != Some(gd) && m::hyp2(o.1[0] - px, o.1[2] - pz) < 0.6);
        let straight = free && !crate::scene_collision::solid_blocked(g, eye, to, None) && !crate::scene_collision::solid_blocked(g, hand, floor, None);
        let looks = if worked {
            free && (straight || !crate::scene_collision::solid_blocked(g, hand, over, None) || !crate::scene_collision::solid_blocked(g, hand, to, None) || wall_seen(g, walls, hand))
        } else {
            straight
        };
        if looks && !in_own(g, kind, if kind == SquadThrowKind::Frag { &frag_at } else { &stun_at }, px, pz, gd, has_lob, &mut ducks) {
            tries -= 1.0;
            g.stats.squadthrow.arcs += 1;
            let post: V3 = [px, 0.0, pz];
            if let Some(arc) = arc_at(g, gd, post, straight, worked, at, kind, roll_k, walls, lim) {
                let pts = if kind == SquadThrowKind::Frag { frag_pts(arc.rest, at) } else { stun_pts(arc.rest, at) };
                if !in_own(g, kind, &pts, px, pz, gd, has_lob, &mut ducks)
                    && duck_holds(g, gd, &arc, px, pz, at, kind, has_lob)
                    && crate::orders::squad_orders_reach(g, gd, post).is_none()
                    && way_ok(g, gd, post, home[k], &sc.leaf, gen, c, w, h)
                {
                    let th = SquadThrower { gd, from: arc.from, sol: arc.sol, rest: arc.rest, post: Some(post) };
                    let passes = match lob.as_deref() {
                        Some(l) => lob_clean(g, &l.clean, &th),
                        None => true,
                    };
                    if passes { return Some(th); }
                    if let Some(l) = lob.as_deref_mut() {
                        if l.dirty.is_none() { l.dirty = Some(th); }
                    }
                }
            }
        }
        if d as f64 >= max { continue; }
        for n in 0..4 {
            let nx = x + (if n == 0 { 1 } else if n == 1 { -1 } else { 0 });
            let nz = z + (if n == 2 { 1 } else if n == 3 { -1 } else { 0 });
            if nx < 0 || nz < 0 || nx >= w || nz >= h { continue; }
            let j = (nz * w + nx) as usize;
            if sc.seen[j] == gen { continue; }
            if g.col.nav.blocked[j] != 0 || sc.leaf[j] == gen {   // (two free cells side by side have no wall between them)
                sc.seen[j] = gen;
                continue;
            }
            let b: V3 = [(nx as f64 + 0.5) * c, 0.0, (nz as f64 + 0.5) * c];
            let r = crate::cleared::cleared_rooms_room_at(g, b);
            if js::truthy(r) && r != home[k] && !crate::cleared::cleared_rooms_is_known(g, r) { continue; }   // (left unstamped: a man whose room it IS may still take it)
            sc.seen[j] = gen;
            sc.dep[j] = d + 1;
            sc.who[j] = k as u8;
            sc.q[tail] = j as i32;
            tail += 1;
        }
    }
    None
}

// ---- the part ----

/// a page's store for the throw preview (squadthrow.ts squadThrowKept)
pub fn squad_throw_kept() -> crate::squadthrow::SquadThrowKept {
    SquadThrowKept {
        kept: JsMap::new(),
        caught: None,
        scratch: None,
        col: crate::scene_collision::AsideOwn::zeroed(),
        orders: crate::orders::SquadOrdersAskAsideOwn::zeroed(),
        duck: None,
    }
}

/// `fn_` asked aside: what a search keeps in the game's state stands in from the asker's store, and the game's own is put back after (squadthrow.ts aside)
pub fn aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, pl: PlayerId, v: &mut crate::squadthrow::SquadThrowKept, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let r = g.m.squadthrow.states.get_or_insert_with(pl, State::zeroed);
    let mine_kept = core::mem::replace(&mut r.kept, core::mem::take(&mut v.kept));
    let mine_caught = core::mem::replace(&mut r.caught, v.caught.take());
    let mine_scratch = core::mem::replace(&mut r.scratch, v.scratch.take());
    let mine_duck = core::mem::replace(&mut g.m.squadthrow.duck_mates, v.duck.take());
    // (TICK_KEPT: frag.WALLS_FOR and seenwalk.shared stand aside and are put back as they were)
    let had_walls = g.m.frag.walls_for.take();
    let had_shared = g.m.seenwalk.shared.take();
    let col = &mut v.col;
    let out = crate::orders::squad_orders_ask_aside(g, &mut v.orders, &mut |g: &mut Game| crate::scene_collision::aside(g, &mut *col, &mut *fn_));
    let r = g.m.squadthrow.states.get_or_insert_with(pl, State::zeroed);
    v.kept = core::mem::replace(&mut r.kept, mine_kept);
    v.caught = core::mem::replace(&mut r.caught, mine_caught);
    v.scratch = core::mem::replace(&mut r.scratch, mine_scratch);
    v.duck = core::mem::replace(&mut g.m.squadthrow.duck_mates, mine_duck);
    g.m.frag.walls_for = had_walls;
    g.m.seenwalk.shared = had_shared;
    out
}

/// a spot in a key (squadthrow.ts keyOf)
pub fn key_of(at: V3) -> String {
    format!("{},{}", to_fixed2(at[0]), to_fixed2(at[2]))
}

/// how many are left in the one bag (squadthrow.ts left)
pub fn left(g: &mut crate::game::Game, kind: crate::squadthrow::SquadThrowKind) -> f64 {
    if kind == SquadThrowKind::Frag { g.door_tools.frags } else { g.door_tools.bangs }
}

/// the order menu's one word for this can (squadthrow.ts word)
pub fn word(kind: crate::squadthrow::SquadThrowKind) -> crate::squadthrow::Word {
    if kind == SquadThrowKind::Frag { Word::Frag } else { Word::Bang }
}

/// the row's words with none left (squadthrow.ts none)
pub fn none(g: &mut crate::game::Game, kind: crate::squadthrow::SquadThrowKind) -> crate::squadthrow::None {
    if kind == SquadThrowKind::Frag { crate::squadthrow::None::NoFragsLeft } else { crate::squadthrow::None::NoBangsLeft }
}

/// the menu's row of this can (squadthrow.ts rowOf)
pub fn row_of(kind: crate::squadthrow::SquadThrowKind) -> crate::squadthrow::RowOf {
    if kind == SquadThrowKind::Frag { RowOf::FragThere } else { RowOf::StunThere }
}

/// how far it runs on past its landing: a frag's own, a canister the smoke can's (squadthrow.ts rollOf)
pub fn roll_of(g: &mut crate::game::Game, kind: crate::squadthrow::SquadThrowKind) -> Option<f64> {
    if kind == SquadThrowKind::Frag { Some(g.k.squadthrow.frag_roll_k) } else { None }
}

/// his own throw is still in the air: the toss has to finish (squadthrow.ts tossing)
pub fn tossing(g: &mut crate::game::Game, s: Option<&crate::squadthrow::State>, gd: GuardId) -> bool {
    let t = s.and_then(|s| s.last.get(gd).copied()).unwrap_or(-1e9);
    toss_window(g, t)
}

/// his hands are on something else: a charge, a door's handle or boot, a thing he works, a magazine (squadthrow.ts handsBusy)
pub fn hands_busy(g: &mut crate::game::Game, gd: GuardId) -> bool {
    use crate::doortools::Phase as DoorPhase;
    if g.door_tools.ops.iter().any(|&o| { let op = &g.tab.doortools_door_op[o]; op.lead == gd && (op.phase == DoorPhase::Place || op.phase == DoorPhase::Kick) }) { return true; }
    if g.wall_charge.ops.iter().any(|&o| { let op = &g.tab.wallcharge_wall_op[o]; op.lead == gd && op.phase == crate::wallcharge::Phase::Place }) { return true; }
    let c = g.gd[gd].ch;
    if crate::workthings::work_hand(g, c).is_some() { return true; }
    let o = crate::roomfight::room_fight_op_of(g, gd);
    g.gd[gd].reload_t > 0.0 || o.map_or(false, |o| g.tab.roomfight_op[o].reload_until > g.time)
}

/// the man of `men` who would throw it, with his throw: the nearest to the spot with a hand free and an arc that brings the thing to rest on it (squadthrow.ts squadThrower)
pub fn squad_thrower(g: &mut crate::game::Game, men: &[GuardId], at: V3, kind: crate::squadthrow::SquadThrowKind) -> Option<crate::squadthrow::SquadThrower2> {
    let th = stand_throw(g, men, at, kind, None)?;
    Some(SquadThrower2 { gd: th.gd, from: th.from, sol: th.sol, rest: th.rest })
}

/// … `lob`: each man's worked-out throw too, and a throw that would catch one of ours is passed over for the next man (squadthrow.ts standThrow)
pub fn stand_throw(g: &mut crate::game::Game, men: &[GuardId], at: V3, kind: crate::squadthrow::SquadThrowKind, lob: Option<&mut crate::squadthrow::Lob>) -> Option<crate::squadthrow::StandThrow> {
    let mut lob = lob;
    let mut sorted: Vec<GuardId> = men.to_vec();
    { let r: &Game = &*g; js::sort_by(&mut sorted, |a, b| v3::dist_xz(pos_of(r, *a), at) - v3::dist_xz(pos_of(r, *b), at)); }
    for gd in sorted {
        if !crate::squadsmoke::squad_hand_free(g, gd) || hands_busy(g, gd) || tossing_here(g, gd) { continue; }
        if walking_here(g, gd) { continue; }   // (on his way to a throw already: that one first)
        let roll = roll_of(g, kind);
        let arc = match crate::squadsmoke::squad_arc_to(g, gd, at, roll, None) {
            Some(a) => Some(Arc { from: a.from, sol: a.sol, rest: a.rest }),
            None => match lob.as_deref_mut() {
                Some(l) => throw_to_lim(g, gd, at, kind, None, &l.walls, &mut l.lim).map(|a| Arc { from: a.from, sol: a.sol, rest: a.rest }),
                None => None,
            },
        };
        let arc = match arc { Some(a) => a, None => continue };
        let th = StandThrow { gd, from: arc.from, sol: arc.sol, rest: arc.rest };
        let l = match lob.as_deref_mut() { Some(l) => l, None => return Some(th) };
        let as_thrower = SquadThrower { gd: th.gd, from: th.from, sol: th.sol.clone(), rest: th.rest, post: None };
        if lob_clean(g, &l.clean, &as_thrower) { return Some(th); }
        if l.dirty.is_none() { l.dirty = Some(as_thrower); }
    }
    None
}

/// SQUADTHROW.duck: a man of ours on his feet at `p` is off this frag's line before it bursts (squadthrow.ts fragClears)
pub fn frag_clears(g: &mut crate::game::Game, gd: GuardId, p: V3, pts: &[V3], late: Option<f64>, taken: Option<&mut Vec<V3>>) -> bool {
    let late = late.unwrap_or(0.0);
    let (lob_on, duck_on, duck_min_m) = (g.k.squadthrow.lob, g.k.squadthrow.duck, g.k.squadthrow.duck_min_m);
    if !lob_on || !duck_on || g.gd[gd].held.is_some() || g.time < g.gd[gd].stagger_until || g.time < g.gd[gd].dazzled_until { return false; }
    if pts.iter().any(|l| v3::dist_xz(p, *l) < duck_min_m) { return false; }   // (inside what it kills at nobody is asked to duck)
    let run = g.k.frag.run_speed;
    let blast = crate::teampace::team_blast_pace(g, gd, run);
    let speed = crate::condition::condition_pace(g, gd, blast);
    let l = pts[0];
    g.stats.squadthrow.ducks += 1;
    let rest: V3 = [l[0], l[1] - 0.25, l[2]];
    let time = g.time;
    let kept = match &g.m.squadthrow.duck_mates {
        Some(d) if d.gd == gd && d.t == time && d.x == rest[0] && d.y == rest[1] && d.z == rest[2] => Some(d.m.clone()),
        _ => None,
    };
    let apart = match kept {
        Some(a) => a,
        None => {
            let a = crate::frag::flee_mates(g, gd, rest);
            g.m.squadthrow.duck_mates = Some(SquadThrowKeptDuck { gd, t: time, x: rest[0], y: rest[1], z: rest[2], m: a.clone() });
            a
        }
    };
    // (a forecast is not a man acting on his picture: the picture's counts are put back)
    let (asks, lines, moved) = (g.stats.picture.asks, g.stats.picture.lines, g.stats.picture.moved);
    let legs = crate::kitload::load_pace(g, gd, speed);
    let time_left = g.k.frag.order_fuse - g.k.squadthrow.duck_secs - late;
    let accel = g.k.pace.accel * crate::kitload::load_pace_on(g, crate::kitload::LoadPaceOnWho::Guard(gd));
    let wise = crate::frag::FleeSpotWise { path: Vec::new(), keep: None, not: None, apart: apart.map(|a| crate::frag::FleeSpotWiseApart { stay: a.stay, go: a.go }) };
    let spot = crate::frag::flee_spot(g, crate::frag::FleeSpotC { pos: p }, rest, legs, time_left, Some(accel), Some(&wise), Some(gd));
    g.stats.picture.asks = asks;
    g.stats.picture.lines = lines;
    g.stats.picture.moved = moved;
    let spot = match spot { Some(q) => q, None => return false };
    if frag_in(g, pts, spot, Some(1.15), None) { return false; }
    // (… and one nobody of ours stands on or is going to: two men to one corner is one man left out in it)
    let apart_m = g.k.squadthrow.duck_apart_m;
    let near = |q: V3| v3::dist_xz(q, spot) < apart_m;
    if g.guards.iter().any(|&o| o != gd && g.gd[o].team == crate::teams::Team::Operative && g.gd[o].state != crate::game::GuardState::Dead && near(pos_of(g, o))) { return false; }
    if !g.pl[g.player].down && near(sam_pos(g)) { return false; }
    if let Some(t) = taken {
        if t.iter().any(|q| near(*q)) { return false; }
        t.push(spot);
    }
    true
}

/// where a frag would go off: where it rests, and the spot (squadthrow.ts fragPts)
pub fn frag_pts(rest: V3, at: V3) -> Vec<V3> {
    vec![[rest[0], at[1] + 0.3, rest[2]], [at[0], at[1] + 0.3, at[2]]]
}

/// a body at `p` is inside a frag at one of `pts`: the door tools' own test (squadthrow.ts fragIn)
pub fn frag_in(g: &mut crate::game::Game, pts: &[V3], p: V3, head: Option<f64>, reach: Option<f64>) -> bool {
    let head = head.unwrap_or(1.6);
    let reach = match reach { Some(r) => r, None => g.k.frag.wound_m + 1.0 };
    for l in pts {
        if v3::dist_xz(p, *l) < reach
            && (crate::frag::exposed_to(g, *l, p, [p[0], p[1] + head, p[2]], None) > 0.0
                || !crate::scene_collision::solid_blocked(g, [l[0], l[1] + 0.6, l[2]], [p[0], p[1] + 1.1, p[2]], None))
        {
            return true;
        }
    }
    false
}

/// where a stun canister would go off: where it rests, and the spot (squadthrow.ts stunPts)
pub fn stun_pts(rest: V3, at: V3) -> Vec<V3> {
    let height = crate::effects::bang().height;
    vec![[rest[0], at[1] + 0.055 + height, rest[2]], [at[0], at[1] + 0.055 + height, at[2]]]
}

/// an eye is caught by a stun canister going off at one of `pts`: detonateStun's tests, each reach slackM long (squadthrow.ts stunIn)
pub fn stun_in(g: &mut crate::game::Game, pts: &[V3], eye: V3, full: bool) -> bool {
    let dazzle_radius = crate::effects::bang().dazzle_radius;
    let entry = crate::roomfight::fight_live(g);
    let slack = g.k.squadthrow.slack_m;
    let (lift, own_m, concuss_m) = (g.k.flash.lift, g.k.flash.own_m, g.k.flash.concuss_m);
    for c in pts {
        let c = *c;
        let d = v3::dist(eye, c);
        if d > dazzle_radius + slack { continue; }
        let from: V3 = [c[0], c[1] + 0.3, c[2]];
        let over: V3 = [c[0], c[1] + lift, c[2]];
        if !entry {
            if !crate::scene_collision::segment_blocked(g, from, eye, None, None, None) { return true; }
            continue;
        }
        let seen = !crate::scene_collision::segment_blocked(g, from, eye, None, None, None) || (!crate::scene_collision::segment_blocked(g, from, over, None, None, None) && !crate::scene_collision::segment_blocked(g, over, eye, None, None, None));
        if seen && full { return true; }
        if d > own_m + slack { continue; }
        if seen { return true; }
        let pr = crate::scene_collision::sound_paths_propagate(g, c, eye, None);
        if pr.muffle <= 0.6 && pr.path_len <= concuss_m + slack { return true; }
    }
    false
}

/// may this man be walked off to a throw: no drill, door tool, wall charge or fight has him, and his order can be handed back (squadthrow.ts mayWalk)
pub fn may_walk(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let a = crate::orders::squad_orders_of(g, gd);
    let owners: Vec<String> = g.orders.owners.iter().map(|o| o.name.clone()).collect();
    for name in &owners {
        if owner_owns(g, name, gd) { return false; }
    }
    if g.door_tools.ops.iter().any(|&o| g.tab.doortools_door_op[o].men.contains(&gd)) || g.wall_charge.ops.iter().any(|&o| g.tab.wallcharge_wall_op[o].men.contains(&gd)) { return false; }
    // (a room fight drives a teammate in 'patrol' with no yield stamp: the man it has a gun on is its `tgt`; a stack's or a stand-off's rear man keeps the squad's back)
    if g.gd[gd].state == crate::game::GuardState::Alert { return false; }
    if let Some(a) = a {
        if g.tab.orders_assignment[a].yield_t >= 0.0 { return false; }
    }
    if crate::roomfight::room_fight_op_of(g, gd).map_or(false, |o| g.tab.roomfight_op[o].tgt.is_some()) { return false; }
    match a {
        Some(a) => {
            if g.tab.orders_assignment[a].spare == Some(true) { return false; }
            let order = g.tab.orders_assignment[a].order.as_str();
            resumed().iter().any(|r| r == order)
        }
        None => true,
    }
}

/// NOBODY HAS A THROW FROM WHERE HE STANDS: the nearest floor one of `men` has one from, by the walk grid (squadthrow.ts walkSpot)
pub fn walk_spot(g: &mut crate::game::Game, men: &[GuardId], at: V3, kind: crate::squadthrow::SquadThrowKind, forced: Option<bool>, lob: Option<&mut crate::squadthrow::Lob>) -> Option<crate::squadthrow::SquadThrower> {
    let forced = forced.unwrap_or(false);
    let mut lob = lob;
    let (c, w, h) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    state(g);   // (the row is made here if it is not there)
    let mut who: Vec<GuardId> = Vec::new();
    for &gd in men {
        if g.gd[gd].state != crate::game::GuardState::Dead
            && crate::squadsmoke::squad_hand_free(g, gd)
            && !hands_busy(g, gd)
            && !tossing_here(g, gd)
            && (forced || (!state(g).walks.has(gd) && may_walk(g, gd)))
        {
            who.push(gd);
        }
    }
    { let r: &Game = &*g; js::sort_by(&mut who, |a, b| v3::dist_xz(pos_of(r, *a), at) - v3::dist_xz(pos_of(r, *b), at)); }
    who.truncate(8);
    if who.is_empty() { return None; }
    // (the search's marks are the row's, and stay there between searches: out of the row while the flood lends `g`; nothing a flood calls runs another walkSpot)
    let n = w * h;   // (the grid's size is a number on the table, as in the TypeScript)
    let mut sc = match state(g).scratch.take() {
        Some(x) if x.n == n => x,
        _ => {
            let cells = n as usize;
            Scratch { n, gen_: 0.0, seen: vec![0; cells], leaf: vec![0; cells], dep: vec![0; cells], who: vec![0; cells], q: vec![0; cells] }
        }
    };
    let max = m::min(250.0, m::round(g.k.squadthrow.walk_m / c));
    let mut home: Vec<f64> = Vec::with_capacity(who.len());
    for &gd in &who {
        let p = pos_of(g, gd);
        home.push(crate::cleared::cleared_rooms_room_at(g, p));
    }
    // (each with whose feet they are: `o !== gd.char.pos` was an identity test of the array)
    let mut ours: Vec<(Option<GuardId>, V3)> = g.guards.iter().filter(|&&o| g.gd[o].team == crate::teams::Team::Operative && g.gd[o].state != crate::game::GuardState::Dead).map(|&o| (Some(o), pos_of(g, o))).collect();
    if !g.pl[g.player].down { ours.push((None, sam_pos(g))); }
    g.stats.squadthrow.searches += 1;
    let roll_k = roll_of(g, kind);
    let walls: Vec<V3> = match lob.as_deref() { Some(l) => l.walls.clone(), None => Vec::new() };
    let mut lim = match lob.as_deref() { Some(l) => l.lim, None => LobLim { n: 0.0 } };
    let nav = (c, w as i32, h as i32);
    let mut found = flood(g, &who, &home, &ours, at, kind, false, max, roll_k, &walls, &mut lob, &mut lim, &mut sc, nav);
    if found.is_none() && lob.is_some() {
        found = flood(g, &who, &home, &ours, at, kind, true, max, roll_k, &walls, &mut lob, &mut lim, &mut sc, nav);
    }
    if let Some(l) = lob.as_deref_mut() { l.lim = lim; }
    state(g).scratch = Some(sc);
    found
}

/// THE THROW AS IT WOULD BE ORDERED NOW: the nearest man with a throw from where he stands, else the man with the shortest walk to one (squadthrow.ts squadThrowPlan)
pub fn squad_throw_plan(g: &mut crate::game::Game, men: &[GuardId], at: V3, kind: crate::squadthrow::SquadThrowKind, fresh: Option<bool>, view: Option<bool>) -> Option<crate::squadthrow::SquadThrower> {
    let (fresh, view) = (fresh.unwrap_or(false), view.unwrap_or(false));
    let (lob_on, walk_on) = (g.k.squadthrow.lob, g.k.squadthrow.walk);
    let th: Option<SquadThrower> = squad_thrower(g, men, at, kind).map(|t| SquadThrower { gd: t.gd, from: t.from, sol: t.sol, rest: t.rest, post: None });
    let clean = LobClean::Plan { kind, at, fresh };
    if lob_on {
        if let Some(t) = &th {
            if lob_clean(g, &clean, t) { return th; }
        }
    } else if th.is_some() || !walk_on {
        return th;
    }
    // (one kept answer a kind, and the PICTURE's kept apart from the menu's and the order's: what is drawn never decides what the game reads)
    let slot = format!("{}{}{}", if view { "v" } else { "s" }, kind.as_str(), if lob_on && men.len() == 1 { "1" } else { "" });
    let keys: Vec<String> = men.iter().map(|&m_| {
        let c = g.gd[m_].ch;
        let p = g.ch[c].pos;
        format!("{}:{},{}", int_str(g.ch[c].id), int_str(m::floor(p[0] * 2.0)), int_str(m::floor(p[2] * 2.0)))
    }).collect();
    let key = format!("{}|{}|{}", kind.as_str(), key_of(at), keys.join(";"));
    let keep = if lob_on { g.k.squadthrow.lob_keep_secs } else { g.k.squadthrow.keep_secs };
    let time = g.time;
    if !fresh {
        if let Some(c) = state(g).kept.get(slot.clone()) {
            if c.key == key && time >= c.t && time - c.t < keep { return c.th.clone(); }
        }
    }
    // (… then a throw one of them works out from where he stands, then a walk to the straight arc, then a walk to a worked-out throw — the first that catches
    //  nobody of ours; nothing clean: the first throw found all the same, and the row says who is in it)
    let mut l: Option<Lob> = if lob_on {
        let walls = crate::throwplan::throw_walls(g, at);
        let half = (js::to_i32(g.k.squadthrow.lob_sims) >> 1) as f64;
        Some(Lob { walls, lim: LobLim { n: half }, clean, dirty: th.clone() })
    } else {
        None
    };
    let mut w: Option<SquadThrower> = match l.as_mut() {
        Some(lob) => stand_throw(g, men, at, kind, Some(lob)).map(|t| SquadThrower { gd: t.gd, from: t.from, sol: t.sol, rest: t.rest, post: None }),
        None => None,
    };
    if w.is_none() && walk_on {
        if let Some(lob) = l.as_mut() { lob.lim = LobLim { n: g.k.squadthrow.lob_sims }; }
        w = walk_spot(g, men, at, kind, Some(false), l.as_mut());
    }
    if w.is_none() {
        w = match l { Some(lob) => lob.dirty, None => th };
    }
    let t = g.time;
    state(g).kept.set(slot, StateKeptValue { key, t, th: w.clone(), line: None });
    w
}

/// his way to a spot as the orders would walk him: a line on the floor (squadthrow.ts walkLine)
pub fn walk_line(g: &mut crate::game::Game, gd: GuardId, post: V3) -> Vec<V3> {
    let r = crate::orders::squad_orders_route(g, gd, post, None, None, None);
    let start = pos_of(g, gd);
    let mut out: Vec<V3> = vec![v3::copy(start)];
    let mut a = start;
    let mut stops: Vec<V3> = if r.why.is_some() { Vec::new() } else { r.via };
    stops.push(post);
    for b in stops {
        if let Some(leg) = crate::scene_collision::nav_grid_find_path(g, a, b, None, None) {
            for p in leg { out.push(p); }
        }
        out.push(b);
        a = b;
    }
    out
}

/// WHO OF OURS A FRAG RESTING THERE WOULD HAVE IN IT (none: nobody), in the row's words; `kept`: the verdict of the last 0.3 s for the same throw with everybody where he was (squadthrow.ts squadFragCatches)
pub fn squad_frag_catches(g: &mut crate::game::Game, rest: V3, at: V3, moved: Option<crate::squadthrow::SquadFragCatchesMoved>, sam: Option<bool>, kept: Option<bool>, burst: Option<&crate::squadthrow::SquadFragCatchesBurst>) -> Option<String> {
    let (sam, kept) = (sam.unwrap_or(true), kept.unwrap_or(false));
    let moved2 = moved.map(|q| FragCatchesMoved { gd: q.gd, at: q.at });
    if !kept || burst.is_some() || !g.k.squadthrow.lob || !g.k.squadthrow.duck {
        let burst2 = burst.map(|b| FragCatchesBurst { reach: b.reach, no: b.no.clone() });
        return frag_catches(g, rest, at, moved2, sam, burst2.as_ref());
    }
    let moved_key = match moved { Some(q) => format!("{}:{}", int_str(g.ch[g.gd[q.gd].ch].id), quarter(q.at)), None => String::new() };
    let sam_key = if g.pl[g.player].down { String::new() } else { quarter(sam_pos(g)) };
    let mates: Vec<String> = g.guards.iter().map(|&o| {
        if g.gd[o].team == crate::teams::Team::Operative {
            format!("{}{}", if g.gd[o].state == crate::game::GuardState::Dead { "x" } else { "" }, quarter(pos_of(g, o)))
        } else {
            String::new()
        }
    }).collect();
    let key = format!("{}|{}|{}|{}|{}|{}", quarter(rest), quarter(at), moved_key, sam, sam_key, mates.join(";"));
    let time = g.time;
    if let Some(c) = &state(g).caught {
        if c.key == key && time >= c.t && time - c.t < 0.3 { return c.who.clone(); }
    }
    let who = frag_catches(g, rest, at, moved2, sam, None);
    let t = g.time;
    state(g).caught = Some(StateCaught { key, t, who: who.clone() });
    who
}

/// the test itself: inside FRAG.woundM + 1, a line by the burst's own exposure or a plain one, from where it rests and from the spot (squadthrow.ts fragCatches)
pub fn frag_catches(g: &mut crate::game::Game, rest: V3, at: V3, moved: Option<crate::squadthrow::FragCatchesMoved>, sam: bool, burst: Option<&crate::squadthrow::FragCatchesBurst>) -> Option<String> {
    let pts = frag_pts(rest, at);
    let reach = burst.map(|b| b.reach);
    if (sam || sam_near(g, &pts)) && !g.pl[g.player].down {
        let p = sam_pos(g);
        if frag_in(g, &pts, p, None, reach) { return Some(String::from("you're in it.")); }
    }
    let mut taken: Vec<V3> = Vec::new();
    let mut mate: Option<GuardId> = None;
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if g.gd[gd].team != crate::teams::Team::Operative { continue; }
        let hit = if g.gd[gd].state != crate::game::GuardState::Dead {
            // (duck: … and cannot be off its line before it bursts)
            let p = match moved { Some(q) if q.gd == gd => q.at, _ => pos_of(g, gd) };
            if !frag_in(g, &pts, p, None, reach) {
                false
            } else if burst.is_some() {
                true
            } else {
                let late = match moved {
                    Some(q) if q.gd == gd => g.k.squadthrow.duck_throw_secs,
                    Some(q) => m::max(0.0, v3::dist_xz(q.at, p) - g.k.frag.see_m) / 5.0,   // (a man is told when it is within seeM of him: one far from the hand hears late, by the slowest pace it is thrown at)
                    None => 0.0,
                };
                !frag_clears(g, gd, p, &pts, Some(late), Some(&mut taken))
            }
        } else if crate::game::is_breathing(g, gd) {   // (a teammate down and breathing is still one of ours)
            let b = crate::guards::body_pos(g, gd);
            frag_in(g, &pts, b, None, reach)
        } else {
            false
        };
        if hit { mate = Some(gd); break; }
    }
    if let Some(gd) = mate { return Some(format!("{} is in it.", g.gd[gd].callsign)); }
    let no: String = match burst { Some(b) => b.no.clone(), None => String::from("no frag.") };
    let mut hostage = false;
    for i in 0..g.rescue.hostages.len() {
        let o = g.rescue.hostages[i];
        if live_in(g, o, &pts, reach) { hostage = true; break; }
    }
    let man = g.snatch.man;
    let wanted = match man {
        Some(o) if live_in(g, o, &pts, reach) => Some(o),
        _ => None,
    };
    if hostage { return Some(format!("hostages. {}", no)); }
    if let Some(o) = wanted {
        let name = match g.gd[o].civ { Some(c) => g.cv[c].name.clone(), None => String::from("our man") };
        return Some(format!("{} is in it. {}", name, no));
    }
    for i in 0..g.civilians.len() {
        let o = g.civilians[i];
        if live_in(g, o, &pts, reach) { return Some(format!("a civilian is in it. {}", no)); }
    }
    None
}

/// (fragCatches live) alive and inside it
fn live_in(g: &mut Game, o: GuardId, pts: &[V3], reach: Option<f64>) -> bool {
    if g.gd[o].state == crate::game::GuardState::Dead { return false; }
    let p = pos_of(g, o);
    frag_in(g, pts, p, None, reach)
}

/// WHO OF OURS A STUN CANISTER GOING OFF THERE WOULD CATCH: 'you' and callsigns (squadthrow.ts squadStunCatches)
pub fn squad_stun_catches(g: &mut crate::game::Game, rest: V3, at: V3) -> Vec<String> {
    if !crate::effects::flags().dazzle { return Vec::new(); }
    let pts = stun_pts(rest, at);
    let mut out: Vec<String> = Vec::new();
    if !g.pl[g.player].down && g.puppet.is_none() && !g.touring {
        let c = g.pl[g.player].ch;
        let eye = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(h) => *h, None => v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]) };
        if stun_in(g, &pts, eye, true) { out.push(String::from("you")); }
    }
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if g.gd[gd].team != crate::teams::Team::Operative || g.gd[gd].state == crate::game::GuardState::Dead { continue; }
        let c = g.gd[gd].ch;
        let eye = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(h) => *h, None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]) };
        if stun_in(g, &pts, eye, false) { out.push(g.gd[gd].callsign.clone()); }
    }
    out
}

/// SQUADTHROW.samCall: a frag the player has a line to is his call (squadthrow.ts samCalls)
pub fn sam_calls(g: &mut crate::game::Game) -> bool {
    g.k.squadthrow.lob && g.k.squadthrow.sam_call
}

/// … is he inside it, where it would go off or at the spot asked for? (squadthrow.ts squadFragOnSam)
pub fn squad_frag_on_sam(g: &mut crate::game::Game, rest: V3, at: V3) -> bool {
    if g.pl[g.player].down { return false; }
    let p = sam_pos(g);
    frag_in(g, &frag_pts(rest, at), p, None, None)
}

/// … and never his call inside what it kills at (squadthrow.ts samNear)
pub fn sam_near(g: &mut crate::game::Game, pts: &[V3]) -> bool {
    let p = sam_pos(g);
    pts.iter().any(|l| v3::dist_xz(p, *l) < g.k.squadthrow.duck_min_m)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the TypeScript's own answers: bun -e 'console.log((0.125).toFixed(2), …)'
    #[test]
    fn to_fixed_as_javascript() {
        for (x, want) in [(0.125, "0.13"), (0.375, "0.38"), (2.5, "2.50"), (1.005, "1.00"), (-0.001, "-0.00"), (-0.0, "0.00"), (-0.125, "-0.13"), (12.345, "12.35"), (3.0, "3.00"), (10.625, "10.63")] {
            assert_eq!(to_fixed2(x), want, "{}", x);
        }
        assert_eq!(key_of([0.125, 9.0, -0.0]), "0.13,0.00");
    }

    #[test]
    fn whole_numbers_as_javascript() {
        assert_eq!(int_str(-0.0), "0");
        assert_eq!(int_str(-3.0), "-3");
        assert_eq!(quarter([-0.1, 0.0, 1.0]), "0,4");
    }

    #[test]
    fn a_frag_goes_off_at_two_points() {
        assert_eq!(frag_pts([1.0, 7.0, 2.0], [3.0, 0.5, 4.0]), vec![[1.0, 0.8, 2.0], [3.0, 0.8, 4.0]]);
    }

    #[test]
    fn a_step_to_each_side() {
        assert_eq!([side(0), side(1), side(2), side(3)], [(0.3, 0.0), (-0.3, 0.0), (0.0, 0.3), (0.0, -0.3)]);
    }
}
