//! src/game/gunhold.ts lines 1 to 439
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- the lines this file owns

/// (gunhold.ts LINES) the staff's lines for the four moments
fn lines(kind: GunHoldPoolKind) -> &'static [&'static str] {
    match kind {
        GunHoldPoolKind::Gun => &["gun — he's got a gun! get behind something!", "he's armed — cover, get to cover!"],
        GunHoldPoolKind::Shot => &["he's shooting — out of his line, move!", "shots! get off his line — go round him!"],
        GunHoldPoolKind::Round => &["going round him — keep him talking", "I'll get behind him. keep his eyes on you"],
        GunHoldPoolKind::Gone => &["he's moved — they're not there!"],
    }
}
/// (gunhold.ts GUNHOLD_DEFENDER_LINES) the same four from a defender of a squad mission
fn defender_lines(kind: GunHoldPoolKind) -> &'static [&'static str] {
    match kind {
        GunHoldPoolKind::Gun => &["pistol's out behind his shield — get to cover!", "he's drawn from the hold — get behind something!"],
        GunHoldPoolKind::Shot => &["he's firing past his shield — off that line, move!", "rounds from the hold — get off it, go round him!"],
        GunHoldPoolKind::Round => &["going round behind him — keep his eyes on you", "coming at his back. hold him where he is"],
        GunHoldPoolKind::Gone => &["he's moved — nobody where we left them!"],
    }
}
/// (gunhold.ts GUNHOLD_NOW) a line as it is said now, by the line it stands for; written order
static NOW: &[(&str, &str)] = &[
    ("shots! get off his line — go round him!", "shots! get out of his line of fire — go around him!"),
    ("going round him — keep him talking", "going around him — keep him talking"),
    ("he's drawn from the hold — get behind something!", "he's pulled a pistol — get behind something!"),
    ("rounds from the hold — get off it, go round him!", "he's shooting — get out of his line of fire, go around him!"),
    ("going round behind him — keep his eyes on you", "going around behind him — keep his eyes on you"),
    ("coming at his back. hold him where he is", "coming up behind him. keep him where he is"),
    ("he's moved — nobody where we left them!", "he's moved — they're not where we left them!"),
];
/// `GUNHOLD_NOW[was] ?? was`
fn now_of(was: &str) -> &str {
    for &(from, to) in NOW {
        if from == was {
            return to;
        }
    }
    was
}

// ---------------------------------------------------------------- stats, the men

/// a new stats row (gunHoldStats' literal: `freeT` starts at −1, which `GunHoldStats::zeroed` does not)
fn stats_maker() -> GunHoldStats {
    GunHoldStats { armed: 0.0, ran: 0.0, no_cover: 0.0, rounds: 0.0, no_way: 0.0, looks: 0.0, gone: 0.0, lines: 0.0, free_t: -1.0 }
}
/// This game's counters, the row itself. (gunhold.ts gunHoldStats)
pub fn gun_hold_stats<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::gunhold::GunHoldStats {
    g.m.gunhold.stats_of.get_or_insert_with(stats_maker)
}

/// Is the rule running now: never under a show or quiet utility. (gunhold.ts live)
pub fn live(g: &mut crate::game::Game) -> bool {
    !g.touring && !g.quiet_utility
}

/// Distance on the floor. (gunhold.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// The men of the standoff on this held man, living, in roster order. (gunhold.ts menOn)
pub fn men_on(g: &mut crate::game::Game, held: GuardId) -> Vec<GuardId> {
    let mut out = Vec::new();
    for &o in &g.guards {
        if g.gd[o].state == crate::game::GuardState::Dead {
            continue;
        }
        if let Some(s) = g.gd[o].standoff {
            if g.tab.guards_standoff[s].pair == held {
                out.push(o);
            }
        }
    }
    out
}

/// Take a hold's Known out of the table for a call that needs `g` and it together (the row keeps its place, a placeholder in it). Nothing
/// reached while it is out reads the table: see notes, Traps.
fn lend_known(g: &mut Game, key: u32) -> Option<Known> {
    g.m.gunhold.knowns.get_mut(key).map(core::mem::take)
}
fn return_known(g: &mut Game, key: u32, k: Known) {
    g.m.gunhold.knowns.set(key, k);
}

/// This man has the pair in his eyes this frame: what his side knows of the hold is brought up to it. (gunhold.ts gunHoldSee)
pub fn gun_hold_see(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) {
    if !live(g) {
        return;
    }
    let (variant, phase, shot_t) = match g.pl[g.player].holding.as_ref() {
        Some(h) => (h.variant, h.phase, h.shot_t),
        None => return,
    };
    let key = p.key;
    let mut k = match lend_known(g, key) {
        Some(k) => k,
        None => Known {
            gun_t: -1.0,
            shot_t: -1.0,
            sam: v3::copy(p.sam),
            facing: p.facing,
            fix_t: g.time,
            gone_t: -1.0,
            expo: JsMap::new(),
            expo_at: v3::copy(p.sam),
            expo_t: g.time,
            talker: None,
            said_t: -100.0,
            said_gun: false,
            said_shot: false,
        },
    };
    k.sam = v3::copy(p.sam);
    k.facing = p.facing;
    k.fix_t = g.time;
    k.gone_t = -1.0;
    if k.gun_t < 0.0 && variant == crate::character::HoldVariant::Gun && phase != crate::character::HoldPhase::Grab {
        k.gun_t = g.time;
        gun_hold_stats(g).armed += 1.0;
    }
    learn_shot(g, &mut k, shot_t, true);
    let gun_known = k.gun_t >= 0.0;
    return_known(g, key, k);
    if gun_known && g.gd[gd].react_t > 0.3 {
        g.gd[gd].react_t = 0.3;
    }
}

/// A round from the hold reaches the side's knowledge through a man who sensed it, while it is fresh. (gunhold.ts learnShot)
pub fn learn_shot(g: &mut crate::game::Game, k: &mut crate::gunhold::Known, shot_t: f64, sensed: bool) {
    if !(shot_t > k.shot_t) || !sensed || g.time - shot_t > FRESH_S {
        return;
    }
    k.shot_t = shot_t;
    if k.gun_t < 0.0 {
        k.gun_t = g.time;
        gun_hold_stats(g).armed += 1.0;
    }
}

/// partOf without the copy: the keys of the hold's Known and of this man's St, when the rule has him.
fn part_keys(g: &mut Game, gd: GuardId) -> Option<(u32, GuardsStandoffId)> {
    if !live(g) {
        return None;
    }
    let s = g.gd[gd].standoff?;
    let (held, key) = g.pl[g.player].holding.as_ref().map(|h| (h.g, h.serial))?;
    if held != g.tab.guards_standoff[s].pair {
        return None;
    }
    if g.m.gunhold.knowns.has(key) && g.m.gunhold.states.has(s) {
        Some((key, s))
    } else {
        None
    }
}

/// Is this man's standoff one the rule has (the pistol known in the hold, a part dealt him); copies of both rows. (gunhold.ts partOf)
pub fn part_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::gunhold::PartOf> {
    let (key, s) = part_keys(g, gd)?;
    let k = g.m.gunhold.knowns.get(key)?.clone();
    let st = *g.m.gunhold.states.get(s)?;
    Some(PartOf { k, st })
}

/// Behind cover on purpose the standoff is kept while somebody of his side has seen the pair lately and no look found them gone. (gunhold.ts gunHoldKeeps)
pub fn gun_hold_keeps(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let (key, s) = match part_keys(g, gd) {
        Some(p) => p,
        None => return false,
    };
    let (gone_t, fix_t) = match g.m.gunhold.knowns.get(key) {
        Some(k) => (k.gone_t, k.fix_t),
        None => return false,
    };
    gone_t < 0.0 && g.time - m::max(g.tab.guards_standoff[s].seen_t, fix_t) < g.k.gunhold.keep_secs
}

/// A man this rule has on his feet for cover is not rooted in the open by rounds past his head. (gunhold.ts gunHoldRuns)
pub fn gun_hold_runs(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let (key, s) = match part_keys(g, gd) {
        Some(p) => p,
        None => return false,
    };
    let st = match g.m.gunhold.states.get(s) {
        Some(st) => *st,
        None => return false,
    };
    if st.phase != Phase::Cover && st.phase != Phase::Line {
        return true;
    }
    let pos = g.ch[g.gd[gd].ch].pos;
    let k = match lend_known(g, key) {
        Some(k) => k,
        None => return false,
    };
    let r = shown(g, &k, pos, st.phase == Phase::Cover && st.low);
    return_known(g, key, k);
    r
}

/// Is the clean line at Sam's back this man's to use: Sam has fired from behind the man, and the man is no civilian. (gunhold.ts gunHoldFree)
pub fn gun_hold_free(g: &mut crate::game::Game, p: crate::guards::Pair) -> bool {
    if !live(g) || g.gd[p.held].team == crate::teams::Team::Civilian {
        return false;
    }
    let key = p.key;
    match g.m.gunhold.knowns.get(key) {
        Some(k) if !(k.shot_t < 0.0) => {}
        _ => return false,
    }
    let now = g.time;
    gun_hold_stats(g).free_t = now;
    true
}

// ---------------------------------------------------------------- the floor his gun covers

/// `!g.col.shotBlocked(from, [x, h, z], 0.02, PISTOL_ENERGY)`: expo's and shown's `open`
fn open_at(g: &mut Game, from: V3, x: f64, h: f64, z: f64) -> bool {
    !crate::scene_collision::shot_blocked(g, from, [x, h, z], Some(0.02), Some(PISTOL_ENERGY))
}

/// What a man on nav cell `i` shows Sam's gun from where the pair was last seen: 2 open, 1 open standing, 0 hidden; kept per cell. (gunhold.ts expo)
pub fn expo(g: &mut crate::game::Game, k: &mut crate::gunhold::Known, i: f64) -> f64 {
    if flat(k.sam, k.expo_at) > 0.5 || g.time - k.expo_t > 1.5 {
        k.expo.clear();
        k.expo_at = v3::copy(k.sam);
        k.expo_t = g.time;
    }
    if let Some(&e) = k.expo.get(crate::Num(i)) {
        return e;
    }
    let (w, cell) = (g.col.nav.w, g.col.nav.cell);
    let x = (js::rem(i, w) + 0.5) * cell;
    let z = (js::to_i32(i / w) as f64 + 0.5) * cell;
    let gun: V3 = [k.expo_at[0], GUN_Y, k.expo_at[2]];
    let e = if m::hyp2(x - gun[0], z - gun[2]) > 24.0 {
        0.0
    } else if open_at(g, gun, x, 0.8, z) || open_at(g, gun, x, 1.05, z) {
        2.0
    } else if open_at(g, gun, x, 1.5, z) || open_at(g, gun, x, 1.2, z) {
        1.0
    } else {
        0.0
    };
    k.expo.set(crate::Num(i), e);
    e
}

/// The nav cell under a point, −1 off the grid. (gunhold.ts cellOf)
pub fn cell_of(g: &mut crate::game::Game, p: V3) -> f64 {
    let n = &g.col.nav;
    let x = m::floor(p[0] / n.cell);
    let z = m::floor(p[2] / n.cell);
    if x < 0.0 || z < 0.0 || x >= n.w || z >= n.h {
        -1.0
    } else {
        z * n.w + x
    }
}

/// The floor point at a nav cell's middle. (gunhold.ts centre)
pub fn centre(g: &mut crate::game::Game, i: f64) -> V3 {
    let n = &g.col.nav;
    [(js::rem(i, n.w) + 0.5) * n.cell, 0.0, (js::to_i32(i / n.w) as f64 + 0.5) * n.cell]
}

/// Is the pistol turned this man's way: within faceDeg of where the pair faces. (gunhold.ts facesHim)
pub fn faces_him(g: &mut crate::game::Game, k: &crate::gunhold::Known, p: V3) -> bool {
    m::abs(v3::wrap_angle(m::atan2(p[0] - k.sam[0], p[2] - k.sam[2]) - k.facing)) < g.k.gunhold.face_deg * v3::DEG
}

/// Forward of Sam's rear arc. (gunhold.ts front)
pub fn front(g: &mut crate::game::Game, k: &crate::gunhold::Known, p: V3) -> bool {
    m::abs(v3::wrap_angle(m::atan2(p[0] - k.sam[0], p[2] - k.sam[2]) - (k.facing + m::PI))) > g.k.gunhold.rear_deg * v3::DEG
}

/// Is a man standing (or, behind low cover, crouched) at `p` in Sam's line of fire, forward of his rear arc. (gunhold.ts shown)
pub fn shown(g: &mut crate::game::Game, k: &crate::gunhold::Known, p: V3, low: bool) -> bool {
    if !front(g, k, p) {
        return false;
    }
    let gun: V3 = [k.sam[0], GUN_Y, k.sam[2]];
    if low {
        open_at(g, gun, p[0], 0.8, p[2]) || open_at(g, gun, p[0], 1.05, p[2])
    } else {
        open_at(g, gun, p[0], 1.3, p[2]) || open_at(g, gun, p[0], 1.6, p[2]) || open_at(g, gun, p[0], 0.8, p[2])
    }
}

/// The closure `price` returns, its captures read where it runs. (gunhold.ts price)
fn price_at(g: &mut Game, k: &mut Known, ix: f64, iz: f64) -> f64 {
    let (cell, w) = (g.col.nav.cell, g.col.nav.w);
    let near2 = g.k.gunhold.near_m * g.k.gunhold.near_m;
    let x = (ix + 0.5) * cell;
    let z = (iz + 0.5) * cell;
    let dx = x - k.sam[0];
    let dz = z - k.sam[2];
    if dx * dx + dz * dz < near2 {
        return 20.0;
    }
    let e = expo(g, k, iz * w + ix);
    if e == 2.0 {
        if front(g, k, [x, 0.0, z]) { 8.0 } else { 1.4 }
    } else if e == 1.0 {
        1.3
    } else {
        1.0
    }
}

/// The TypeScript's price only makes the closure, which is `price_at` here, and writes nothing; its only callers are in this file. (gunhold.ts price)
pub fn price(g: &mut crate::game::Game, k: &mut crate::gunhold::Known) {}

/// A way to `goal` at that price; its length and the longest unbroken stretch of it across floor Sam's gun covers in front of him. (gunhold.ts wayTo)
pub fn way_to(g: &mut crate::game::Game, gd: GuardId, k: &mut crate::gunhold::Known, goal: V3) -> Option<crate::gunhold::WayTo> {
    let start = g.ch[g.gd[gd].ch].pos;
    let path = crate::scene_collision::nav_grid_find_path(g, start, goal, Some(8000.0), Some(&mut |g: &mut Game, ix: f64, iz: f64| price_at(g, k, ix, iz)))?;
    if path.is_empty() || flat(path[path.len() - 1], goal) > 0.6 {
        return None;
    }
    let (mut len, mut show, mut cur) = (0.0, 0.0, 0.0);
    let mut a = start;
    for &b in &path {
        let d = flat(a, b);
        let steps = m::max(1.0, m::ceil(d / 0.4));
        let mut j = 1.0;
        while j <= steps {
            let q: V3 = [a[0] + (b[0] - a[0]) * j / steps, 0.0, a[2] + (b[2] - a[2]) * j / steps];
            let i = cell_of(g, q);
            if i >= 0.0 && expo(g, k, i) == 2.0 && front(g, k, q) {
                cur += d / steps;
                if cur > show {
                    show = cur;
                }
            } else {
                cur = 0.0;
            }
            j += 1.0;
        }
        len += d;
        a = b;
    }
    Some(WayTo { path, len, show })
}

/// Sets his path: no pathGoal, so a man stuck on this way is planned again at this rule's price. (gunhold.ts take)
pub fn take(g: &mut crate::game::Game, gd: GuardId, path: &[V3], goal: V3) {
    let start = g.ch[g.gd[gd].ch].pos;
    let threaded = crate::doors::doors_thread_path(g, path, start, Some(&mut |g: &mut Game, x: f64, z: f64| !crate::scene_collision::nav_grid_is_blocked(g, x, z)));
    let man = &mut g.gd[gd];
    man.path = threaded;
    man.path_i = 0;
    man.path_goal = None;
    man.repath_t = 0.0;
    man.stall_ref = None;
    man.line_side = 0.0;
}

// ---------------------------------------------------------------- cover

/// `n.blocked[i]` as a condition: past either end of the grid is undefined, so false (a cell off the grid can start the walk: nearestFreePoint's).
fn blocked_at(g: &Game, i: f64) -> bool {
    if i < 0.0 {
        return false;
    }
    g.col.nav.blocked.get(i as usize).map_or(false, |&b| b != 0)
}

/// The nearest floor, by walking, where Sam's gun has no line to him; `edge` = the open cell he came off to get there. (gunhold.ts pickCover)
pub fn pick_cover(g: &mut crate::game::Game, gd: GuardId, s: GuardsStandoffId, k: &mut crate::gunhold::Known, round: bool) -> Option<crate::gunhold::PickCover> {
    let (w, h, cell) = (g.col.nav.w, g.col.nav.h, g.col.nav.cell);
    let pos = g.ch[g.gd[gd].ch].pos;
    let mut start = cell_of(g, pos);
    if start < 0.0 {
        return None;
    }
    if blocked_at(g, start) {
        let f = crate::scene_collision::nav_grid_nearest_free_point(g, pos[0], pos[2])?;
        start = cell_of(g, f);
    }
    let pair = g.tab.guards_standoff[s].pair;
    let mut mates: Vec<V3> = Vec::new();
    let mut rivals: Vec<V3> = Vec::new();
    for &o in &g.guards {
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead {
            continue;
        }
        let of = g.gd[o].standoff;
        let spot = of.and_then(|os| g.m.gunhold.states.get(os)).and_then(|r| r.spot);
        if let Some(spot) = spot {
            mates.push(spot);
        } else {
            let at = g.ch[g.gd[o].ch].pos;
            mates.push(at);
            if let Some(os) = of {
                if g.tab.guards_standoff[os].pair == pair {
                    rivals.push(at);
                }
            }
        }
    }
    let d0 = flat(pos, k.sam);
    let rear = k.facing + m::PI;
    let near = g.k.gunhold.near_m;
    let st_bad = g.m.gunhold.states.get(s).and_then(|r| r.bad); // `st.bad`: nothing below writes the row
    let mut came: JsMap<crate::Num, f64> = JsMap::new();
    came.set(crate::Num(start), -1.0);
    let mut q: Vec<f64> = vec![start];
    let (mut best, mut best_score, mut first_d) = (-1.0, m::INF, -1.0);
    let max_d = m::ceil(g.k.gunhold.cover_m / cell);
    let mut depth = 1.0;
    while depth <= max_d && !q.is_empty() {
        let mut nx: Vec<f64> = Vec::new();
        for &cur in &q {
            let cx = js::rem(cur, w);
            let cz = js::to_i32(cur / w) as f64;
            for dz in [-1.0, 0.0, 1.0] {
                for dx in [-1.0, 0.0, 1.0] {
                    let diagonal = dx != 0.0 && dz != 0.0;
                    if dx == 0.0 && dz == 0.0 {
                        continue;
                    }
                    let x = cx + dx;
                    let z = cz + dz;
                    if x < 0.0 || z < 0.0 || x >= w || z >= h {
                        continue;
                    }
                    let i = z * w + x;
                    if blocked_at(g, i) || came.has(crate::Num(i)) {
                        continue;
                    }
                    if diagonal && (blocked_at(g, cz * w + x) || blocked_at(g, z * w + cx)) {
                        continue;
                    }
                    let p = centre(g, i);
                    let d_s = flat(p, k.sam);
                    if d_s < near && d_s < d0 {
                        continue;
                    }
                    came.set(crate::Num(i), cur);
                    nx.push(i);
                    let e = expo(g, k, i);
                    if e == 2.0 && front(g, k, p) {
                        continue;
                    }
                    if d_s < near + 0.5 {
                        continue;
                    }
                    if let Some(bad) = st_bad {
                        if flat(p, bad) < 0.8 {
                            continue;
                        }
                    }
                    if mates.iter().any(|&m8| flat(m8, p) < 0.9) {
                        continue;
                    }
                    let mut score = depth * cell + (if e == 1.0 { 1.0 } else { 0.0 }) + (if d_s < d0 - 1.5 { 4.0 } else { 0.0 });
                    for (ax, az) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
                        let xx = x + ax;
                        let zz = z + az;
                        if xx < 0.0 || zz < 0.0 || xx >= w || zz >= h {
                            continue;
                        }
                        let j = zz * w + xx;
                        if !blocked_at(g, j) && expo(g, k, j) == 2.0 {
                            let cj = centre(g, j);
                            if front(g, k, cj) {
                                score += 0.6;
                            }
                        }
                    }
                    if e == 2.0 {
                        score += 2.5;
                    }
                    for &r in &rivals {
                        if flat(r, p) + 0.5 < flat(pos, p) {
                            score += 3.0;
                            break;
                        }
                    }
                    if round {
                        score += 0.7 * m::abs(v3::wrap_angle(rear - m::atan2(p[0] - k.sam[0], p[2] - k.sam[2])));
                    }
                    if score < best_score {
                        best_score = score;
                        best = i;
                    }
                    if first_d < 0.0 {
                        first_d = depth;
                    }
                }
            }
        }
        q = nx;
        if first_d >= 0.0 && depth - first_d >= 6.0 {
            break;
        }
        depth += 1.0;
    }
    if best < 0.0 {
        return None;
    }
    let mut edge: Option<V3> = None;
    let mut i = came.get(crate::Num(best)).copied().unwrap_or(-1.0);
    let mut n = 0;
    while i >= 0.0 && n < 4 {
        if expo(g, k, i) == 2.0 {
            edge = Some(centre(g, i));
            break;
        }
        i = came.get(crate::Num(i)).copied().unwrap_or(-1.0);
        n += 1;
    }
    let spot = centre(g, best);
    let low = expo(g, k, best) == 1.0;
    Some(PickCover { spot, low, edge })
}

// ---------------------------------------------------------------- the way round

/// A way to Sam's back that keeps out of his line; three path plans a look at most. Sets his path; `st` is the table's row, by `s`. (gunhold.ts planRound)
pub fn plan_round(g: &mut crate::game::Game, gd: GuardId, s: GuardsStandoffId, k: &mut crate::gunhold::Known) -> bool {
    let rear = k.facing + m::PI;
    let mut plans = 0;
    let pos = g.ch[g.gd[gd].ch].pos;
    let max_run = if !faces_him(g, k, pos) {
        g.k.gunhold.dash_m
    } else if k.shot_t >= 0.0 && g.time - k.shot_t < g.k.gunhold.shy_s {
        g.k.gunhold.show_m * 0.4
    } else {
        g.k.gunhold.show_m
    };
    let mut mates: Vec<V3> = Vec::new();
    for &o in &g.guards {
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead {
            continue;
        }
        if let Some(goal) = g.gd[o].standoff.and_then(|os| g.m.gunhold.states.get(os)).and_then(|r| r.goal) {
            mates.push(goal);
        }
    }
    let side = g.tab.guards_standoff[s].side;
    for off in [40.0, 20.0, 60.0, 0.0, -25.0] {
        for r in [4.2, 3.5, 4.8] {
            let b = rear - side * off * v3::DEG;
            let p: V3 = [k.sam[0] + m::sin(b) * r, 0.0, k.sam[2] + m::cos(b) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) {
                continue;
            }
            // a place he can SHOOT from: his gun shoulder, either side of the spot, to Sam's chest
            let sx = m::cos(b) * 0.22;
            let sz = -m::sin(b) * 0.22;
            let to: V3 = [k.sam[0], AIM_Y, k.sam[2]];
            let mut shut = false;
            for side_k in [-1.0, 0.0, 1.0] {
                if crate::scene_collision::shot_blocked(g, [p[0] + sx * side_k, 1.3, p[2] + sz * side_k], to, Some(0.02), None) {
                    shut = true;
                    break;
                }
            }
            if shut {
                continue;
            }
            if mates.iter().any(|&m8| flat(m8, p) < 1.2) {
                continue;
            }
            plans += 1;
            if plans > 3 {
                return false;
            }
            let way = match way_to(g, gd, k, p) {
                Some(way) => way,
                None => continue,
            };
            if way.len > g.k.gunhold.flank_max_m || way.show > max_run {
                continue;
            }
            take(g, gd, &way.path, p);
            let (sam, facing) = (v3::copy(k.sam), k.facing);
            if let Some(row) = g.m.gunhold.states.get_mut(s) {
                row.goal = Some(p);
                row.plan_sam = Some(sam);
                row.plan_facing = facing;
            }
            return true;
        }
    }
    false
}

/// Does this man go round, or stay behind his cover in front: a lone man goes; of two or more the man most in front of Sam stays. (gunhold.ts goesRound)
pub fn goes_round(g: &mut crate::game::Game, gd: GuardId, s: GuardsStandoffId, k: &mut crate::gunhold::Known) -> bool {
    let pair = g.tab.guards_standoff[s].pair;
    let men = men_on(g, pair);
    if men.len() <= 1 {
        return true;
    }
    if k.talker.map_or(true, |t| !men.contains(&t)) {
        let mut b = m::INF;
        k.talker = None;
        for &man in &men {
            let at = g.ch[g.gd[man].ch].pos;
            let a = m::abs(v3::wrap_angle(m::atan2(at[0] - k.sam[0], at[2] - k.sam[2]) - k.facing));
            if a < b {
                b = a;
                k.talker = Some(man);
            }
        }
    }
    k.talker != Some(gd)
}

// ---------------------------------------------------------------- the lines

/// The pool a man draws one of the four moments from: a defender's own, else the staff's. (gunhold.ts gunHoldPool)
pub fn gun_hold_pool(g: &mut crate::game::Game, gd: GuardId, kind: crate::gunhold::GunHoldPoolKind) -> Vec<String> {
    let own = if crate::teams::defender_words(g, gd) { Some(defender_lines(kind)) } else { None };
    let pool = match own {
        Some(own) if !own.is_empty() => own,
        _ => lines(kind),
    };
    pool.iter().map(|&s| String::from(s)).collect()
}

/// One line of a moment, at most one every second and a half for the side; the one roll of the die is drawn either table. (gunhold.ts bark)
pub fn bark(g: &mut crate::game::Game, gd: GuardId, k: &mut crate::gunhold::Known, kind: crate::gunhold::BarkKind, radio: Option<bool>) -> bool {
    let radio = radio.unwrap_or(false);
    if g.time - k.said_t < 1.5 {
        return false;
    }
    k.said_t = g.time;
    let kind = match kind {
        BarkKind::Shot => GunHoldPoolKind::Shot,
        BarkKind::Round => GunHoldPoolKind::Round,
        BarkKind::Gone => GunHoldPoolKind::Gone,
        BarkKind::Gun => GunHoldPoolKind::Gun,
    };
    let pool = gun_hold_pool(g, gd, kind);
    let n = pool.len() as f64;
    let was = &pool[js::rem(m::floor(crate::math_dice::sim_random(g) * n), n) as usize];
    crate::game::game_say(g, gd, now_of(was), Some(radio), None, None);
    true
}

// ---------------------------------------------------------------- his feet

/// what gunHoldFeet's closures captured
#[derive(Clone, Copy)]
struct Feet {
    gd: GuardId,
    s: GuardsStandoffId,
    c: CharId,
    now: f64,
    look: V3,
    dt: f64,
}

/// gunHoldFeet's `stand`
fn stand(g: &mut Game, f: Feet) {
    g.gd[f.gd].speed = v3::damp(g.gd[f.gd].speed, 0.0, 8.0, f.dt);
    g.ch[f.c].vel = [0.0, 0.0, 0.0];
    g.gd[f.gd].stall_ref = None;
}

/// gunHoldFeet's `alongPath`: on a run, eyes and gun on where Sam is while the way keeps him anywhere ahead, else up the way
fn along_path(g: &mut Game, f: Feet, k: &Known) -> V3 {
    let (path_i, len) = (g.gd[f.gd].path_i, g.gd[f.gd].path.len());
    if path_i >= len {
        return f.look;
    }
    let wp = g.gd[f.gd].path[path_i];
    let pos = g.ch[f.c].pos;
    let yaw = m::atan2(wp[0] - pos[0], wp[2] - pos[2]);
    if g.tab.guards_standoff[f.s].seen_t == f.now && !front(g, k, pos) {
        return f.look;
    }
    if m::abs(v3::wrap_angle(m::atan2(f.look[0] - pos[0], f.look[2] - pos[2]) - yaw)) > 75.0 * v3::DEG {
        [pos[0] + m::sin(yaw) * 4.0, f.look[1], pos[2] + m::cos(yaw) * 4.0]
    } else {
        f.look
    }
}

/// gunHoldFeet's `out`: not in his eyes this frame, his gun stays on where they were last seen
fn out(g: &Game, f: Feet, move_: f64, run: bool, lk: Option<V3>, crouch: f64, quiet: bool) -> Option<GunHoldFeet> {
    let look = lk.or(if g.tab.guards_standoff[f.s].seen_t == f.now { None } else { Some(f.look) });
    Some(GunHoldFeet { move_, run, look, crouch, quiet })
}

/// `gd.path = []; gd.pathGoal = null`
fn drop_path(g: &mut Game, gd: GuardId) {
    g.gd[gd].path = Vec::new();
    g.gd[gd].path_goal = None;
}

/// `now + GUNHOLD.peekEvery * (0.6 + 0.8 * simRandom())`
fn next_peek(g: &mut Game, now: f64) -> f64 {
    let every = g.k.gunhold.peek_every;
    now + every * (0.6 + 0.8 * crate::math_dice::sim_random(g))
}

/// This man's feet while the pistol is known in the hold; None = the ring as ever. (gunhold.ts gunHoldFeet)
pub fn gun_hold_feet(g: &mut crate::game::Game, gd: GuardId, dt: f64, p: crate::guards::Pair, s: GuardsStandoffId, fs: crate::gunhold::GunHoldFeetFs) -> Option<crate::gunhold::GunHoldFeet> {
    if !live(g) {
        return None;
    }
    let key = p.key;
    let mut k = lend_known(g, key)?;
    let r = feet(g, gd, dt, &p, s, fs, &mut k);
    return_known(g, key, k);
    r
}

/// gunHoldFeet from its fourth line on, with the hold's Known in hand. His St is always the table's row (`st!()`): plan_round writes it there.
fn feet(g: &mut Game, gd: GuardId, dt: f64, p: &crate::guards::Pair, s: GuardsStandoffId, fs: GunHoldFeetFs, k: &mut Known) -> Option<GunHoldFeet> {
    let c = g.gd[gd].ch;
    if let Some(shot_t) = g.pl[g.player].holding.as_ref().map(|h| h.shot_t) {
        let sensed = g.tab.guards_standoff[s].seen_t == g.time || flat(g.ch[c].pos, p.sam) < HEAR_M;
        learn_shot(g, k, shot_t, sensed);
    }
    if k.gun_t < 0.0 {
        return None;
    }
    let now = g.time;
    gun_hold_stats(g); // the row is made here whether or not this frame counts anything (line 303)
    macro_rules! st {
        () => {
            g.m.gunhold.states.get_mut(s).unwrap()
        };
    }
    macro_rules! pos {
        () => {
            g.ch[c].pos
        };
    }
    if !g.m.gunhold.states.has(s) {
        let r0 = g.k.gunhold.react[0];
        let r1 = g.k.gunhold.react[1];
        let go_t = now + r0 + crate::math_dice::sim_random(g) * (r1 - r0);
        g.m.gunhold.states.set(
            s,
            St {
                phase: Phase::React,
                go_t,
                spot: None,
                low: false,
                edge: None,
                goal: None,
                plan_sam: None,
                plan_facing: 0.0,
                plan_t: 0.0,
                peek_t: 0.0,
                peek_end: -1.0,
                peek_from: 0.0,
                check_t: 0.0,
                run_t: 0.0,
                line_t: -1.0,
                shown_t: 0.0,
                retry_t: 0.0,
                cover_bad: false,
                bad: None,
                said_round: false,
            },
        );
        g.tab.guards_standoff[s].orbit_goal = None;
        drop_path(g, gd);
        g.gd[gd].stall_ref = None;
        if !k.said_gun && bark(g, gd, k, if k.shot_t >= 0.0 { BarkKind::Shot } else { BarkKind::Gun }, None) {
            k.said_gun = true;
            if k.shot_t >= 0.0 {
                k.said_shot = true;
            }
        }
    }
    if k.shot_t >= 0.0 && !k.said_shot && g.tab.guards_standoff[s].seen_t == now && bark(g, gd, k, BarkKind::Shot, None) {
        k.said_shot = true;
    }
    let f = Feet { gd, s, c, now, look: [k.sam[0], p.aim[1], k.sam[2]], dt };
    let low = if crate::suppression::flinching(g, gd) { g.k.suppression.displace_crouch } else { 0.0 };
    let here_e = {
        let at = pos!();
        let i = cell_of(g, at);
        if i >= 0.0 { expo(g, k, i) } else { 0.0 }
    };
    // the man who stays is the other half of the pincer: when Sam comes about, HIS back is to this one
    let round = goes_round(g, gd, s, k) || {
        let at = pos!();
        !front(g, k, at)
    };

    // his line, at Sam's back: plant on it (the caller's round, if it is free)
    let line_ok = fs.ok && {
        let at = pos!();
        !shown(g, k, at, false)
    };
    let row = *st!();
    let sound = line_ok && (row.phase != Phase::Flank || row.goal.map_or(true, |goal| flat(pos!(), goal) < 0.8) || fs.rear < (g.k.gunhold.rear_deg - 20.0) * v3::DEG);
    let keeps = row.phase == Phase::Line
        && (line_ok || (now - row.line_t < 0.4 && {
            let at = pos!();
            !faces_him(g, k, at)
        }));
    if sound || keeps {
        if line_ok {
            st!().line_t = now;
        }
        if st!().phase != Phase::Line {
            st!().phase = Phase::Line;
            gun_hold_stats(g).lines += 1.0;
            drop_path(g, gd);
        }
        stand(g, f);
        return out(g, f, 0.0, false, None, 0.0, false);
    }
    if st!().phase == Phase::Line {
        st!().phase = Phase::Plan;
        st!().plan_t = 0.0;
    }

    if st!().phase == Phase::React {
        if now < st!().go_t {
            stand(g, f);
            return out(g, f, 0.0, false, None, 0.0, false);
        }
        st!().phase = Phase::Plan;
    }

    if st!().phase == Phase::Plan {
        if now < st!().retry_t {
            return None; // no cover in reach a moment ago: the ring as ever until the next look for some
        }
        let at = pos!();
        let open = shown(g, k, at, false);
        if open && here_e == 1.0 && !shown(g, k, at, true) {
            // something low between them already: down behind it, here
            let peek_t = next_peek(g, now);
            let row = st!();
            row.spot = Some(v3::copy(at));
            row.low = true;
            row.edge = None;
            row.phase = Phase::Cover;
            row.peek_t = peek_t;
            row.check_t = now + 0.5;
            drop_path(g, gd);
        } else if open {
            let cv = match pick_cover(g, gd, s, k, round) {
                Some(cv) => cv,
                None => {
                    gun_hold_stats(g).no_cover += 1.0;
                    st!().retry_t = now + 1.5;
                    return None;
                }
            };
            let row = st!();
            row.spot = Some(cv.spot);
            row.low = cv.low;
            row.edge = cv.edge;
            row.goal = None;
            let way = crate::scene_collision::nav_grid_find_path(g, at, cv.spot, Some(6000.0), Some(&mut |g: &mut Game, ix: f64, iz: f64| price_at(g, k, ix, iz)));
            let way = way.unwrap_or_else(|| vec![v3::copy(cv.spot)]);
            take(g, gd, &way, cv.spot);
            let row = st!();
            row.phase = Phase::Run;
            row.run_t = now;
            gun_hold_stats(g).ran += 1.0;
        } else {
            let peek_t = next_peek(g, now);
            let row = st!();
            row.spot = Some(v3::copy(at));
            row.low = false;
            row.edge = None;
            row.phase = Phase::Cover;
            row.peek_t = peek_t;
            row.check_t = 0.0;
            drop_path(g, gd);
        }
    }

    if st!().phase == Phase::Run {
        if st!().spot.is_none() {
            st!().phase = Phase::Plan;
            stand(g, f);
            return out(g, f, 0.0, false, None, low, false);
        }
        // the first strides: he sets off at once the way he has to go (backwards, sideways) and turns into it as he runs
        let mut scramble = false;
        if now - st!().run_t < 1.2 && g.gd[gd].path_i < g.gd[gd].path.len() {
            let wp = g.gd[gd].path[g.gd[gd].path_i];
            let at = pos!();
            let yaw_to = m::atan2(wp[0] - at[0], wp[2] - at[2]);
            if m::abs(v3::wrap_angle(yaw_to - g.ch[c].body_yaw)) > 50.0 * v3::DEG {
                scramble = true;
                g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw_to, 5.5 * dt);
            }
        }
        let chase = g.tune.guard_chase;
        let arrived = if scramble { crate::guards::step_along_path(g, gd, dt, chase) } else { crate::guards::follow_path(g, gd, dt, chase) };
        if arrived {
            let peek_t = next_peek(g, now);
            let row = st!();
            row.phase = Phase::Cover;
            row.peek_t = peek_t;
            row.check_t = now + 0.4;
            row.plan_t = m::max(row.plan_t, now + 0.3);
            drop_path(g, gd);
        } else if crate::guards::not_getting_anywhere(g, gd, dt, 1.5) {
            let row = st!();
            row.bad = row.spot;
            row.spot = None;
            row.phase = Phase::Plan;
            drop_path(g, gd);
            g.gd[gd].stall_ref = None;
        }
        let speed = g.gd[gd].speed;
        let look = along_path(g, f, k);
        return out(g, f, speed, true, Some(look), if here_e == 1.0 { 1.0 } else { m::max(low, 0.35) }, true);
    }

    if st!().phase == Phase::Cover {
        // the walk here stops within a quarter metre of the spot, on the edge of its cell: the last step onto it
        let spot = st!().spot;
        let off = match spot {
            Some(spot) => flat(pos!(), spot),
            None => 0.0,
        };
        match spot {
            Some(spot) if off > 0.06 && off < 0.6 && dt > 0.0 => {
                let step = m::min(1.0, 1.2 * dt / off);
                let at = pos!();
                g.ch[c].vel = [(spot[0] - at[0]) * step / dt, 0.0, (spot[2] - at[2]) * step / dt];
                g.ch[c].pos = [at[0] + (spot[0] - at[0]) * step, at[1], at[2] + (spot[2] - at[2]) * step];
                g.gd[gd].speed = 1.2;
                g.gd[gd].stall_ref = None;
            }
            _ => stand(g, f),
        }
        if now >= st!().check_t {
            // twice a second: has Sam walked the pair round his cover (two looks running, a beat apart)
            st!().check_t = now + 0.5;
            let at = pos!();
            let st_low = st!().low;
            if !shown(g, k, at, st_low) {
                st!().cover_bad = false;
            } else if st!().cover_bad {
                let row = st!();
                row.cover_bad = false;
                row.phase = Phase::Plan;
                row.bad = None;
                return out(g, f, 0.0, false, None, if st_low { 1.0 } else { 0.0 }, false);
            } else {
                let row = st!();
                row.cover_bad = true;
                row.check_t = now + 0.15;
            }
        }
        if round && now >= st!().plan_t && g.m.gunhold.planned != Some(now) {
            // one man's way round a frame, floor-wide
            g.m.gunhold.planned = Some(now);
            let at = pos!();
            let wait = if front(g, k, at) { 2.0 } else { 0.6 };
            st!().plan_t = now + wait;
            if plan_round(g, gd, s, k) {
                st!().phase = Phase::Flank;
                g.tab.guards_standoff[s].cornered = false;
                gun_hold_stats(g).rounds += 1.0;
                if !st!().said_round && bark(g, gd, k, BarkKind::Round, Some(true)) {
                    st!().said_round = true;
                }
                let look = along_path(g, f, k);
                return out(g, f, 0.0, true, Some(look), 0.0, true);
            }
            gun_hold_stats(g).no_way += 1.0;
            g.tab.guards_standoff[s].cornered = true;
        }
        if now >= st!().peek_t && now - k.fix_t > g.k.gunhold.peek_every * 0.5 && g.tab.guards_standoff[s].seen_t != now {
            // nobody's eyes on them for a while: a look
            let row = st!();
            row.peek_from = now;
            row.peek_end = -1.0;
            gun_hold_stats(g).looks += 1.0;
            let (st_low, edge) = (st!().low, st!().edge);
            match edge {
                Some(edge) if !st_low => {
                    st!().phase = Phase::Peek;
                    take(g, gd, &[v3::copy(edge)], edge);
                }
                _ => {
                    let peek_secs = g.k.gunhold.peek_secs;
                    let row = st!();
                    row.phase = Phase::Peek;
                    row.peek_end = now + peek_secs * (if st_low { 0.6 } else { 1.0 }); // over the top of something low: a glance
                }
            }
        }
        let at = pos!();
        let down = st!().low && front(g, k, at);
        return out(g, f, 0.0, false, None, if down { 1.0 } else { 0.0 }, false);
    }

    if st!().phase == Phase::Peek {
        if st!().peek_end < 0.0 {
            let walk = g.tune.guard_walk;
            let there = crate::guards::follow_path(g, gd, dt, walk);
            if there || crate::guards::not_getting_anywhere(g, gd, dt, 1.2) {
                let peek_secs = g.k.gunhold.peek_secs;
                st!().peek_end = now + peek_secs;
                drop_path(g, gd);
            }
        } else {
            stand(g, f);
        }
        let rattled = crate::suppression::flinching(g, gd);
        if (st!().peek_end >= 0.0 && now >= st!().peek_end) || rattled {
            // looked, and they are not where they were left: the standoff is lost for everybody
            if !rattled && g.tab.guards_standoff[s].seen_t < st!().peek_from {
                let at = pos!();
                let eye = match g.ch[c].bones.get(crate::character::Bone::Head) {
                    Some(&head) => head,
                    None => v3::add(at, [0.0, 1.65, 0.0]),
                };
                if !crate::scene_collision::segment_blocked(g, eye, [k.sam[0], 1.2, k.sam[2]], None, None, None) {
                    k.gone_t = now;
                    gun_hold_stats(g).gone += 1.0;
                    bark(g, gd, k, BarkKind::Gone, Some(true));
                }
            }
            let peek_t = next_peek(g, now);
            st!().peek_t = peek_t;
            let spot = st!().spot;
            match spot {
                Some(spot) if flat(pos!(), spot) > 0.3 => {
                    st!().phase = Phase::Back;
                    take(g, gd, &[v3::copy(spot)], spot);
                }
                _ => {
                    let row = st!();
                    row.phase = Phase::Cover;
                    row.check_t = now + 0.5;
                }
            }
        }
        let speed = g.gd[gd].speed;
        return out(g, f, speed, false, None, 0.0, false);
    }

    if st!().phase == Phase::Back {
        let done = st!().spot.is_none() || {
            let investigate = g.tune.guard_investigate;
            crate::guards::follow_path(g, gd, dt, investigate) || crate::guards::not_getting_anywhere(g, gd, dt, 1.2)
        };
        if done {
            let row = st!();
            row.phase = Phase::Cover;
            row.check_t = now + 0.5;
            drop_path(g, gd);
        }
        let speed = g.gd[gd].speed;
        return out(g, f, speed, false, None, low, false);
    }

    // 'flank': the way round; planned again when the pair has moved or come about; a man in the open with the pistol on him breaks for cover
    let at = pos!();
    let here = shown(g, k, at, here_e == 1.0);
    if here && faces_him(g, k, at) {
        st!().shown_t += dt;
    } else {
        let row = st!();
        row.shown_t = m::max(0.0, row.shown_t - dt);
    }
    let stale = match st!().plan_sam {
        Some(plan_sam) => flat(plan_sam, k.sam) > 1.2 || m::abs(v3::wrap_angle(k.facing - st!().plan_facing)) > 40.0 * v3::DEG,
        None => false,
    };
    if st!().shown_t > 0.3 || (stale && now >= st!().plan_t && g.m.gunhold.planned != Some(now)) {
        g.m.gunhold.planned = Some(now);
        let row = st!();
        let caught = row.shown_t > 0.3;
        row.shown_t = 0.0;
        row.plan_t = now + 1.0;
        row.goal = None;
        if caught || !plan_round(g, gd, s, k) {
            let row = st!();
            row.phase = Phase::Plan;
            row.bad = None;
            drop_path(g, gd);
            stand(g, f);
            return out(g, f, 0.0, false, None, low, true);
        }
    }
    let speed = if here { g.tune.guard_chase } else { g.tune.guard_investigate };
    let arrived = crate::guards::follow_path(g, gd, dt, speed);
    if arrived || crate::guards::not_getting_anywhere(g, gd, dt, 2.5) {
        let row = st!();
        row.phase = Phase::Plan;
        row.plan_t = now + 0.5;
        row.goal = None;
        drop_path(g, gd);
        g.gd[gd].stall_ref = None;
    }
    let speed = g.gd[gd].speed;
    let look = along_path(g, f, k);
    out(g, f, speed, true, Some(look), if here_e == 1.0 { 1.0 } else { m::max(low, if here { 0.35 } else { 0.0 }) }, true)
}

/// Panel / QA: the rule's state in a line. (gunhold.ts gunHoldSummary)
pub fn gun_hold_summary(g: &mut crate::game::Game) -> String {
    let held = g.pl[g.player].holding.as_ref().map(|h| (h.g, h.serial));
    let known = held.and_then(|(_, key)| g.m.gunhold.knowns.get(key).map(|k| (k.gun_t, k.shot_t)));
    let s = *gun_hold_stats(g);
    let tail = alloc::format!("{} ran for cover · {} went round · {} lines · {} looks", s.ran, s.rounds, s.lines, s.looks);
    let (held, gun_t, shot_t) = match (held, known) {
        (Some((h, _)), Some((gun_t, shot_t))) if !(gun_t < 0.0) => (h, gun_t, shot_t),
        _ => return alloc::format!("gun in the hold ▸ — ({})", tail),
    };
    let mut parts: Vec<String> = vec![String::from(if shot_t >= 0.0 { "he has fired" } else { "pistol seen" })];
    for gd in men_on(g, held) {
        let st = match g.gd[gd].standoff.and_then(|os| g.m.gunhold.states.get(os)) {
            Some(st) => *st,
            None => continue,
        };
        let what = match st.phase {
            Phase::Run => String::from("running for cover"),
            Phase::Cover => alloc::format!("behind cover{}", if st.low { " (low)" } else { "" }),
            Phase::Peek | Phase::Back => String::from("taking a look"),
            Phase::Flank => match st.goal {
                Some(goal) => alloc::format!("going round, {:.1} m off his spot", flat(g.ch[g.gd[gd].ch].pos, goal)),
                None => String::from("going round"),
            },
            Phase::Line => String::from("on the line at his back"),
            _ => String::from("reacting"),
        };
        parts.push(alloc::format!("{}: {}", g.gd[gd].callsign, what));
    }
    alloc::format!("gun in the hold ▸ {} ({})", parts.join(" · "), tail)
}

/// QA and the standoff's muzzle: this man's part, by name. (gunhold.ts gunHoldPhase)
pub fn gun_hold_phase(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::gunhold::Phase> {
    let (_, s) = part_keys(g, gd)?;
    g.m.gunhold.states.get(s).map(|st| st.phase)
}

#[cfg(test)]
mod tests {
    use super::*;

    // the TypeScript's own bits: bun, EXACT.on, Math.hypot(a[0] - b[0], a[2] - b[2])
    #[test]
    fn flat_matches_the_typescript() {
        let d = flat([1.25, 9.0, -3.5], [4.75, -2.0, 7.125]);
        assert_eq!(d.to_bits(), FLAT_BITS);
    }

    #[test]
    fn a_line_as_it_is_said_now() {
        assert_eq!(now_of("going round him — keep him talking"), "going around him — keep him talking");
        assert_eq!(now_of("he's moved — nobody where we left them!"), "he's moved — they're not where we left them!");
        // a line the guide did not reword stands as it was
        assert_eq!(now_of("he's armed — cover, get to cover!"), "he's armed — cover, get to cover!");
    }

    #[test]
    fn every_reworded_line_is_a_line_of_a_table() {
        let kinds = [GunHoldPoolKind::Gun, GunHoldPoolKind::Shot, GunHoldPoolKind::Round, GunHoldPoolKind::Gone];
        for &(from, _) in NOW {
            assert!(kinds.iter().any(|&k| lines(k).contains(&from) || defender_lines(k).contains(&from)), "{}", from);
        }
    }

    #[test]
    fn a_new_stats_row_has_no_free_round_yet() {
        assert_eq!(stats_maker().free_t, -1.0);
    }

    const FLAT_BITS: u64 = 0x40265f8d95065bd3;
}
