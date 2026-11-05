//! src/game/orders.ts lines 2475 to 2696
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// orders.ts:340 `const flat = v3.distXZ`
fn flat(a: V3, b: V3) -> f64 { v3::dist_xz(a, b) }

/// where a man's body stands now
fn man_pos(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }

/// a number in a template string, for a key: JavaScript has no "-0" there
fn num_word(x: f64) -> String { format!("{}", x + 0.0) }

// ------------------------------------------------------------ the squad's other owners

/// orders.ts:631 `o.owns(gd)`. SquadOwner holds only its name; the one row ever pushed (orders.ts:630) is the night breach's. `i`: its place in g.orders.owners
fn owner_owns(g: &mut Game, i: usize, gd: GuardId) -> bool {
    match g.orders.owners[i].name.as_str() {
        "nightbreach" => g.night_breach.active && crate::nightbreach::night_breach_drill_ops(g).contains(&gd) && crate::nightbreach::night_breach_owns_pair(g),
        name => unreachable!("no owner is pushed under the name {}", name),
    }
}

/// orders.ts:633 `o.status?.(gd)`: the HUD's words for a man with no order of Sam's. `i`: its place in g.orders.owners
fn owner_status(g: &mut Game, i: usize, gd: GuardId) -> Option<String> {
    use crate::nightbreach::NightBreachState as S;
    match g.orders.owners[i].name.as_str() {
        "nightbreach" => {
            if !g.night_breach.active || !g.night_breach.ops.contains(&gd) { return None; }
            let state = crate::nightbreach::night_breach_state(g);
            if !crate::nightbreach::night_breach_drill_ops(g).contains(&gd) {
                return if state == S::Off { None } else { Some(String::from("standing by")) };
            }
            Some(String::from(match state {
                S::Staged => "waiting",
                S::Stacking => "stacking · the breach",
                S::Set => "set · press F to send them in",
                S::Going | S::Inside => "breaching",
                S::Clear => "holding the room",
                _ => "holding",
            }))
        }
        name => unreachable!("no owner is pushed under the name {}", name),
    }
}

// ------------------------------------------------------------ a walk, asked for a menu

/// What route() says of his walk there, for a menu; the answer for the same two cells is kept a second.   (orders.ts SquadOrders.asked)
pub fn squad_orders_asked(g: &mut crate::game::Game, gd: GuardId, to: V3) -> crate::orders::SquadOrdersAsked {
    let at = man_pos(g, gd);
    let cell = g.col.nav.cell;
    let p = g.player;
    let mut locks = 0.0;
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        if crate::doors::door_locked(g, d) && crate::doors::door_latched(g, d) { locks += 1.0; }
        i += 1;
    }
    let key = format!(
        "{}|{},{}|{},{}|{},{},{},{}",
        num_word(g.ch[g.gd[gd].ch].id),
        num_word(m::floor(at[0] / cell)), num_word(m::floor(at[2] / cell)),
        num_word(m::floor(to[0] / cell)), num_word(m::floor(to[2] / cell)),
        g.pl[p].keys.len(), g.pl[p].codes.len(), num_word(locks), num_word(g.cleared.known),
    );
    let t = g.time;
    if let Some(hit) = g.orders.reached.get(key.clone()) {
        if m::abs(t - hit.t) < 1.0 {
            return SquadOrdersAsked { why: hit.why.clone(), lock: hit.lock.map(|l| SquadOrdersAskedLock { door: l.door, side: l.side }) };
        }
    }
    if g.orders.reached.len() > 96 { g.orders.reached.clear(); }
    let r = squad_orders_route(g, gd, to, None, None, Some(true));
    let kept = SquadOrdersReachedValue { t, why: r.why.clone(), lock: r.lock.map(|l| SquadOrdersReachedValueLock { door: l.door, side: l.side }) };
    g.orders.reached.set(key, kept);
    SquadOrdersAsked { why: r.why, lock: r.lock.map(|l| SquadOrdersAskedLock { door: l.door, side: l.side }) }
}

/// A question asked by the picture: `fn_` with the walks it asks about kept in the asker's own table, not in `reached`.   (orders.ts SquadOrders.askAside)
pub fn squad_orders_ask_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::orders::SquadOrdersAskAsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let mine = core::mem::take(&mut g.orders.reached);
    g.orders.reached = own.r.take().unwrap_or_default();
    let out = fn_(g);
    own.r = Some(core::mem::take(&mut g.orders.reached));
    g.orders.reached = mine;
    out
}

/// Has he a line to cover it from where he stands: to a door, the way to it clear at eye height; 'that way', nothing in his face for two metres.   (orders.ts SquadOrders.hasLine)
pub fn squad_orders_has_line(g: &mut crate::game::Game, gd: GuardId, t: &crate::orders::OrderTarget) -> bool {
    let p = man_pos(g, gd);
    let (at, door) = match t {
        OrderTarget::Floor(x) => (x.at, false),
        OrderTarget::Door(x) => (x.at, true),
        OrderTarget::None(x) => (x.at, false),
        OrderTarget::Foe(x) => (x.at, false),
    };
    let d = flat(p, at);
    if d < 0.6 { return true; }
    let k = (if door { d - 0.6 } else { m::min(d, 2.0) }) / d;
    !crate::scene_collision::segment_blocked(g, [p[0], 1.5, p[2]], [p[0] + (at[0] - p[0]) * k, 1.5, p[2] + (at[2] - p[2]) * k], None, None, None)
}

// ------------------------------------------------------------ men to spots

/// orders.ts:2506 `find`: the spot's file
fn find(root: &mut [usize], i: usize) -> usize {
    if root[i] == i { return i; }
    let r = find(root, root[i]);
    root[i] = r;
    r
}

/// orders.ts:2518 `walk`: the metres of his way there
fn file_walk(g: &mut Game, gd: GuardId, to: V3) -> f64 {
    let p = man_pos(g, gd);
    let Some(w) = crate::scene_collision::nav_grid_find_path(g, p, to, None, None) else { return 1e6; };
    let mut l = 0.0;
    let mut q = p;
    for &c in w.iter() { l += flat(q, c); q = c; }
    l + flat(q, to)
}

/// ORDERS.fileQueue, for one deal of a stack: what a dealing costs (`file_cross_cost`), or None where no file among the spots is tight: nothing is priced, no walk is asked for.   (orders.ts SquadOrders.fileCross)
pub fn squad_orders_file_cross(g: &mut crate::game::Game, men: &[GuardId], spots: &[V3]) -> Option<crate::orders::FileCross> {
    let n = spots.len();
    let gap = g.k.orders.file_queue_gap;
    let cost = g.k.orders.file_queue_cost;
    let w_ = g.k.doormanners.round_out;
    if n < 2 || men.len() < 2 { return None; }
    let mut root: Vec<usize> = (0..n).collect();
    for i in 0..n {
        for j in (i + 1)..n {
            if flat(spots[i], spots[j]) <= gap {
                let ri = find(&mut root, i);
                let rj = find(&mut root, j);
                root[ri] = rj;
            }
        }
    }
    let mut tight: Vec<bool> = vec![false; n];
    for i in 0..n {
        let mut o: Option<usize> = None;
        for j in 0..n {
            if j != i && find(&mut root, j) == find(&mut root, i) {
                let closer = match o { None => true, Some(o) => flat(spots[i], spots[j]) < flat(spots[i], spots[o]) };
                if closer { o = Some(j); }
            }
        }
        let Some(o) = o else { continue; };
        let mk = spots[i];
        let l = flat(mk, spots[o]);
        if l < 0.05 { continue; }
        let nx = -(spots[o][2] - mk[2]) / l;
        let nz = (spots[o][0] - mk[0]) / l;
        let mut room = false;
        for sd in [1.0, -1.0] {
            let q: V3 = [mk[0] + nx * sd * w_, 0.0, mk[2] + nz * sd * w_];
            if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) && !crate::scene_collision::solid_blocked(g, [mk[0], 0.45, mk[2]], [q[0], 0.45, q[2]], None) { room = true; }
        }
        if !room { let r = find(&mut root, i); tight[r] = true; }
    }
    if !tight.iter().any(|&x| x) { return None; }
    let file: Vec<usize> = (0..n).map(|j| find(&mut root, j)).collect();
    let mut l: Vec<Vec<f64>> = Vec::with_capacity(men.len());
    for &gd in men {
        let mut row = Vec::with_capacity(n);
        for j in 0..n {
            row.push(if tight[file[j]] { file_walk(g, gd, spots[j]) } else { 0.0 });
        }
        l.push(row);
    }
    Some(FileCross::Tight { file, tight, l, spots: spots.to_vec(), cost })
}

/// orders.ts:2522 the function fileCross returns: what a dealing (`at[i]` = the spot of man i) costs in men squeezing past one another
fn file_cross_cost(f: &FileCross, at: &[usize]) -> f64 {
    let FileCross::Tight { file, tight, l, spots, cost } = f;
    // x would have to get past y: y's mark lies on x's walk to his own, and y is on it before x comes by
    let past = |x: usize, jx: usize, y: usize, jy: usize| l[x][jy] + flat(spots[jy], spots[jx]) <= l[x][jx] + 0.5 && l[x][jy] > l[y][jy];
    let mut c = 0.0;
    for a in 0..at.len() {
        for b in (a + 1)..at.len() {
            let (ja, jb) = (at[a], at[b]);
            if file[ja] != file[jb] || !tight[file[ja]] { continue; }
            if past(a, ja, b, jb) || past(b, jb, a, ja) { c += *cost; }
        }
    }
    c
}

/// orders.ts:2532 what `rec` of pair() walks over and keeps
struct PairDeal<'a> {
    men: &'a [GuardId],
    spots: &'a [V3],
    groups: Option<&'a [f64]>,
    front: Option<V3>,
    gr: Option<Vec<f64>>,
    ids: Vec<Vec<usize>>,
    cross: Option<FileCross>,
    used: Vec<bool>,
    cur: Vec<usize>,
    best: Vec<usize>,
    bc: f64,
    best0: Vec<usize>,
    bc0: f64,
}

/// orders.ts:2533 `new Set(l.map(m => groups[cur[m]])).size`: Set's SameValueZero, where a missing row is one value of its own
fn group_count(groups: &[f64], cur: &[usize], l: &[usize]) -> f64 {
    let mut seen: Vec<Option<f64>> = Vec::with_capacity(l.len());
    for &m_ in l {
        let v = groups.get(cur[m_]).copied();
        let same = |x: &Option<f64>| match (*x, v) {
            (None, None) => true,
            (Some(a), Some(b)) => a == b || (a.is_nan() && b.is_nan()),
            _ => false,
        };
        if !seen.iter().any(same) { seen.push(v); }
    }
    seen.len() as f64
}

/// orders.ts:2532 `rec`: every way of dealing the men from man `i` on
fn pair_rec(g: &mut Game, s: &mut PairDeal, i: usize, walk: f64) {
    if i == s.men.len() {
        let mut c = walk;
        if let Some(groups) = s.groups {
            for l in s.ids.iter() { c += 1000.0 * (group_count(groups, &s.cur, l) - 1.0); }
        }
        if let Some(gr) = s.gr.as_ref() {
            let at: Vec<V3> = s.cur.iter().map(|&j| s.spots[j]).collect();
            c += crate::gunroles::gun_file_cost(g, gr, &at, s.front.unwrap());
        }
        if s.cross.is_some() {
            if c < s.bc0 { s.bc0 = c; s.best0 = s.cur.clone(); }
            let cur = s.cur.clone();
            c += file_cross_cost(s.cross.as_ref().unwrap(), &cur);
        }
        if c < s.bc { s.bc = c; s.best = s.cur.clone(); }
        return;
    }
    for j in 0..s.spots.len() {
        if s.used[j] { continue; }
        s.used[j] = true;
        s.cur[i] = j;
        let w = walk + flat(man_pos(g, s.men[i]), s.spots[j]);
        pair_rec(g, s, i + 1, w);
        s.used[j] = false;
    }
}

/// Men to spots, the pairing that walks the least in all (`groups`: which spots belong together; `front`: the frame a stack's marks stand on).   (orders.ts SquadOrders.pair)
pub fn squad_orders_pair(g: &mut crate::game::Game, men: &[GuardId], spots: &[V3], groups: Option<&[f64]>, front: Option<V3>) -> Vec<(GuardId, V3)> {
    let mut cross = if front.is_some() { squad_orders_file_cross(g, men, spots) } else { None };
    if men.len() > 2 && men.len() <= 4 && spots.len() >= men.len() {
        let n = men.len();
        let gr = if front.is_some() { crate::gunroles::gun_file_ranks(g, men) } else { None };
        let mut ids: Vec<Vec<usize>> = Vec::new();
        for id in squad_orders_pairs(g) {
            let mut l = Vec::new();
            for (i, &gd) in men.iter().enumerate() {
                if squad_orders_pair_of(g, gd) == Some(id) { l.push(i); }
            }
            if l.len() > 1 { ids.push(l); }
        }
        let mut s = PairDeal {
            men, spots, groups, front, gr, ids, cross: cross.take(),
            used: vec![false; spots.len()], cur: vec![0; n],
            best: Vec::new(), bc: m::INF, best0: Vec::new(), bc0: m::INF,
        };
        pair_rec(g, &mut s, 0, 0.0);
        if s.cross.is_some() && s.best.len() == n && s.best.iter().zip(s.best0.iter()).any(|(a, b)| a != b) {
            g.m.doorway.stack_own.queue += 1.0;
        }
        if s.best.len() == n {
            return men.iter().enumerate().map(|(i, &gd)| (gd, spots[s.best[i]])).collect();
        }
        cross = s.cross;
    }
    if men.len() == 2 && spots.len() == 2 {
        let (p0, p1) = (man_pos(g, men[0]), man_pos(g, men[1]));
        let (d00, d11, d01, d10) = (flat(p0, spots[0]), flat(p1, spots[1]), flat(p0, spots[1]), flat(p1, spots[0]));
        let gr = if front.is_some() { crate::gunroles::gun_file_ranks(g, men) } else { None };
        let (gc01, gc10) = match gr.as_ref() {
            Some(r) => (crate::gunroles::gun_file_cost(g, r, &[spots[0], spots[1]], front.unwrap()), crate::gunroles::gun_file_cost(g, r, &[spots[1], spots[0]], front.unwrap())),
            None => (0.0, 0.0),
        };
        let (xc01, xc10) = match cross.as_ref() {
            Some(f) => (file_cross_cost(f, &[0, 1]), file_cross_cost(f, &[1, 0])),
            None => (0.0, 0.0),
        };
        let keeps = d00 + d11 + gc01 + xc01 <= d01 + d10 + gc10 + xc10;
        if cross.is_some() && (d00 + d11 + gc01 <= d01 + d10 + gc10) != keeps { g.m.doorway.stack_own.queue += 1.0; }
        return if keeps { vec![(men[0], spots[0]), (men[1], spots[1])] } else { vec![(men[0], spots[1]), (men[1], spots[0])] };
    }
    let mut left: Vec<V3> = spots.to_vec();
    let mut out = Vec::with_capacity(men.len());
    for &gd in men {
        let p = man_pos(g, gd);
        let mut bi = 0;
        for (i, &s) in left.iter().enumerate() {
            if flat(p, s) < flat(p, left[bi]) { bi = i; }
        }
        let spot = if left.len() > 1 { left.remove(bi) } else { left[0] };
        out.push((gd, spot));
    }
    out
}

// ------------------------------------------------------------ the HUD's words

/// What one man is doing, in a few words.   (orders.ts SquadOrders.statusOf)
pub fn squad_orders_status_of(g: &mut crate::game::Game, gd: GuardId) -> String {
    if g.gd[gd].state == crate::game::GuardState::Dead {
        return String::from(if g.gd[gd].wound.is_some() { "wounded" } else { "down" });
    }
    if let Some(a) = g.orders.asg.get(gd).copied() {
        let w = if g.tab.orders_assignment[a].post.is_none() && crate::pairup::pair_up(g, crate::pairup::PairUpRule::Word) {
            crate::sneak::sneak_word(g, gd)
        } else { None };
        let word = if squad_orders_shown_there(g, a) { g.tab.orders_assignment[a].there.clone() } else { g.tab.orders_assignment[a].going.clone() };
        if g.gd[gd].script.is_none() && g.gd[gd].state == crate::game::GuardState::Alert { return String::from("in contact"); }
        return match w {
            Some(w) if !w.is_empty() => format!("{} · {}", word, w),
            _ => word,
        };
    }
    for own in [true, false] {
        let mut i = 0;
        while i < g.orders.owners.len() {
            if owner_owns(g, i, gd) == own {
                if let Some(s) = owner_status(g, i, gd) {
                    if !s.is_empty() { return s; }
                }
            }
            i += 1;
        }
    }
    String::from("no orders")
}

/// Who the next order goes to, as the panel names it.   (orders.ts SquadOrders.whoTag)
pub fn squad_orders_who_tag(g: &mut crate::game::Game) -> crate::orders::SquadOrdersWhoTag {
    if g.orders.menu.as_ref().map_or(false, |x| x.part.is_some()) {
        let men = g.orders.menu.as_ref().unwrap().men.clone();
        let tag = if !men.is_empty() { squad_orders_who_words(g, Some(&men)) } else { String::from("nobody") };
        return SquadOrdersWhoTag { tag, one: true };
    }
    let menu_man = match g.orders.menu.as_ref().map(|x| &x.target) {
        Some(OrderTarget::None(t)) => t.man,
        _ => None,
    };
    let man = match (menu_man, g.orders.who) {
        (Some(x), _) => Some(x),
        (None, Who::Num(n)) => squad_orders_squad(g).get(n as usize).copied(),
        _ => None,
    };
    if let Some(man) = man { return SquadOrdersWhoTag { tag: g.gd[man].callsign.clone(), one: true }; }
    let tag = match g.orders.who {
        Who::All => String::from(if squad_orders_paired(g) { "GOLD" } else { "all" }),
        Who::Red => String::from("RED"),
        Who::Blue => String::from("BLUE"),
        Who::Num(n) => num_word(n),
    };
    SquadOrdersWhoTag { tag, one: false }
}

/// The panel's line for T: its choices in the order it steps them, the standing one in brackets.   (orders.ts SquadOrders.whoStrip)
pub fn squad_orders_who_strip(g: &mut crate::game::Game) -> String {
    let SquadOrdersWhoTag { tag, one } = squad_orders_who_tag(g);
    let menu_man = matches!(g.orders.menu.as_ref().map(|x| &x.target), Some(OrderTarget::None(t)) if t.man.is_some());
    let paired = squad_orders_paired(g);
    if one && (paired || menu_man) {
        return format!("[{}] · {}", tag, if paired { "gold" } else { "all" });
    }
    let mut out: Vec<String> = Vec::new();
    if paired {
        let mut names = vec![String::from("GOLD")];
        for id in squad_orders_pairs(g) {
            let sel = match id { crate::game::PairId::Red => SquadOrdersMenSel::Red, crate::game::PairId::Blue => SquadOrdersMenSel::Blue };
            if !squad_orders_men(g, Some(sel), None).is_empty() { names.push(id.as_str().to_uppercase()); }
        }
        for x in names { out.push(if x == tag { format!("[{}]", x) } else { x.to_lowercase() }); }
        return out.join(" · ");
    }
    let mut names = vec![String::from("all")];
    for gd in squad_orders_men(g, None, None) { names.push(g.gd[gd].callsign.clone()); }
    for x in names { out.push(if x == tag { format!("[{}]", x) } else { x }); }
    out.join(" · ")
}

/// A target in the panel's words: what an order from the menu would be about.   (orders.ts SquadOrders.hoverWords)
pub fn squad_orders_hover_words(g: &mut crate::game::Game, t: Option<&crate::orders::SquadOrdersHoverWordsT>) -> String {
    match t {
        None => String::new(),
        Some(SquadOrdersHoverWordsT::Door(t)) => door_words(g, t.door),
        Some(SquadOrdersHoverWordsT::Floor(t)) => match (t.thing.as_ref(), t.wall.is_some()) {
            (Some(x), _) => x.name.clone(),
            (None, true) => String::from("that wall"),
            (None, false) => String::from("that spot"),
        },
        Some(SquadOrdersHoverWordsT::Foe(t)) => {
            let c = &g.gd[t.foe].callsign;
            if c.is_empty() { String::from("that man") } else { c.clone() }
        }
        Some(SquadOrdersHoverWordsT::None(t)) => match t.man {
            Some(x) => format!("{} alone", g.gd[x].callsign),
            None => String::from("the squad itself"),
        },
    }
}

/// orders.ts hoverWords' parameter is the same union as OrderTarget, in a type of its own
fn hover_of(t: &OrderTarget) -> SquadOrdersHoverWordsT {
    match t {
        OrderTarget::Floor(x) => SquadOrdersHoverWordsT::Floor(x.clone()),
        OrderTarget::Door(x) => SquadOrdersHoverWordsT::Door(*x),
        OrderTarget::None(x) => SquadOrdersHoverWordsT::None(*x),
        OrderTarget::Foe(x) => SquadOrdersHoverWordsT::Foe(*x),
    }
}

/// The interaction panel's rows. Picture only: read by ui/overlay.ts and ui/tabcard.ts alone, and it writes nothing the game reads (triage: dress), so not translated.   (orders.ts SquadOrders.hud)
pub fn squad_orders_hud(g: &mut crate::game::Game, kept: Option<&mut crate::orders::HoverKept>) -> crate::orders::SquadOrdersHud { todo!("picture only, not translated: conventions 6") }

/// Where the menu that is up is frozen, for a mark in the world: the point, a word for it, the addressed element's colour.   (orders.ts SquadOrders.mark)
pub fn squad_orders_mark(g: &mut crate::game::Game) -> Option<crate::orders::SquadOrdersMark> {
    if g.orders.cmd_ring.is_some() { return None; }
    let (t, men) = match g.orders.menu.as_ref() { Some(x) => (x.target.clone(), x.men.clone()), None => return None };
    let (at, text) = match &t {
        OrderTarget::None(n) => {
            let man = n.man?;
            let p = man_pos(g, man);
            ([p[0], 1.0, p[2]], g.gd[man].callsign.clone())
        }
        OrderTarget::Foe(f) => {
            let p = man_pos(g, f.foe);
            let who = squad_orders_who_tag(g).tag;
            let what = squad_orders_hover_words(g, Some(&hover_of(&t)));
            ([p[0], 1.0, p[2]], format!("{} → {}", who, what))
        }
        OrderTarget::Door(d) => {
            let who = squad_orders_who_tag(g).tag;
            let what = squad_orders_hover_words(g, Some(&hover_of(&t)));
            ([d.at[0], 1.0, d.at[2]], format!("{} → {}", who, what))
        }
        OrderTarget::Floor(f) => {
            let who = squad_orders_who_tag(g).tag;
            let what = squad_orders_hover_words(g, Some(&hover_of(&t)));
            ([f.at[0], 0.03, f.at[2]], format!("{} → {}", who, what))
        }
    };
    let tone = squad_orders_tone_of(g, &men);
    Some(SquadOrdersMark { at, text, tone })
}

/// One line for the console. Host only, reached by the debugger's `__orders` alone (triage: host), so not translated.   (orders.ts SquadOrders.summary)
pub fn squad_orders_summary(g: &mut crate::game::Game) -> String { todo!("host only, not translated: conventions 6") }

// ------------------------------------------------------------ ORDERS.propRound

/// orders.ts:2645 `len`: the metres of a way from `from`
fn way_len(from: V3, w: &[V3]) -> f64 {
    let mut l = 0.0;
    let mut q = from;
    for &c in w { l += flat(q, c); q = c; }
    l
}

/// orders.ts:2647 `doors`: the doors a way crosses, each once, in the order it crosses them
fn way_doors(g: &mut Game, from: V3, w: &[V3]) -> Vec<DoorId> {
    let mut out: Vec<DoorId> = Vec::new();
    let mut q = from;
    for &c in w {
        let mut i = 0;
        while i < g.doors.list.len() {
            let d = g.doors.list[i];
            if !out.contains(&d) && crate::guards::crosses_door(g, q, c, d).is_some() { out.push(d); }
            i += 1;
        }
        q = c;
    }
    out
}

/// Of the way over the statics alone and the way laid with movable things dear, the one he walks.   (orders.ts propWay)
pub fn prop_way(g: &mut crate::game::Game, from: V3, plain: &[V3], round: Option<&[V3]>) -> Vec<V3> {
    let round = match round { Some(r) if !r.is_empty() => r, _ => return plain.to_vec() };
    if way_len(from, round) > way_len(from, plain) + g.k.orders.prop_round_more { return plain.to_vec(); }
    let a = way_doors(g, from, plain);
    let b = way_doors(g, from, round);
    if a.len() == b.len() && a.iter().all(|d| b.contains(d)) { round.to_vec() } else { plain.to_vec() }
}

/// The cells about every movable thing made dear, for a teammate only. None: `pen` itself, untouched (guards.ts:1491 `round !== pen`), and the one search is the search it was.   (orders.ts propPenalty)
pub fn prop_penalty(g: &mut crate::game::Game, gd: GuardId, pen: Option<&crate::lanes::LanePenalty>) -> Option<crate::orders::PropPenalty> {
    if g.gd[gd].team != crate::teams::Team::Operative || g.props.props.is_empty() { return None; }
    let mut i = 0;
    while i < g.orders.owners.len() {
        if owner_owns(g, i, gd) { return None; }
        i += 1;
    }
    let (pad, by) = (g.k.orders.prop_round_pad, g.k.orders.prop_round_door);
    let cs = g.col.nav.cell;
    let mut dear: JsSet<crate::Num> = JsSet::new();
    let mut pi = 0;
    while pi < g.props.props.len() {
        let p = g.props.props[pi];
        pi += 1;
        let (px, pz) = (crate::props::prop_x(g, p), crate::props::prop_z(g, p));
        let mut near = false;
        for &d in g.doors.list.iter() {
            let fc = g.dr[d].frame_centre;
            if m::hyp2(fc[0] - px, fc[2] - pz) < by { near = true; break; }
        }
        if !near { continue; }
        let r = g.tab.props_prop[p].radius + pad;
        let (x0, x1) = (m::floor((px - r) / cs), m::floor((px + r) / cs));
        let (z0, z1) = (m::floor((pz - r) / cs), m::floor((pz + r) / cs));
        let (w_, h_) = (g.col.nav.w, g.col.nav.h);
        let mut z = z0;
        while z <= z1 {
            let mut x = x0;
            while x <= x1 {
                if x >= 0.0 && z >= 0.0 && x < w_ && z < h_ && m::hyp2((x + 0.5) * cs - px, (z + 0.5) * cs - pz) < r {
                    dear.add(crate::Num(crate::scene_collision::nav_grid_idx(g, x, z)));
                }
                x += 1.0;
            }
            z += 1.0;
        }
    }
    let at = man_pos(g, gd);
    let here = crate::scene_collision::nav_grid_idx(g, m::floor(at[0] / cs), m::floor(at[2] / cs));
    dear.delete(crate::Num(here));
    if dear.is_empty() { return None; }
    let cost = g.k.orders.prop_round_cost;
    Some(PropPenalty::Dear { dear, cost, pen: pen.cloned() })
}

/// ORDERS.sameWay: how far way `a` gets from way `b`, asked every half metre of `a`, and where on `a` that is.   (orders.ts farOff)
pub fn far_off(a: &[V3], b: &[V3]) -> crate::orders::FarOff {
    let off = |x: f64, z: f64| -> f64 {
        let mut mn = m::INF;
        for i in 1..b.len() {
            let (p, q) = (b[i - 1], b[i]);
            let (ux, uz) = (q[0] - p[0], q[2] - p[2]);
            let l = ux * ux + uz * uz;
            let k = if l > 1e-9 { m::max(0.0, m::min(1.0, ((x - p[0]) * ux + (z - p[2]) * uz) / l)) } else { 0.0 };
            mn = m::min(mn, m::hyp2(x - p[0] - ux * k, z - p[2] - uz * k));
        }
        mn
    };
    let mut d = -1.0;
    let mut at: V3 = a.first().copied().unwrap_or_default();
    for i in 1..a.len() {
        let (p, q) = (a[i - 1], a[i]);
        let l = flat(p, q);
        let n = m::max(1.0, m::ceil(l / 0.5));
        let mut s = 1.0;
        while s <= n {
            let x = p[0] + (q[0] - p[0]) * s / n;
            let z = p[2] + (q[2] - p[2]) * s / n;
            let o = off(x, z);
            if o > d { d = o; at = [x, 0.0, z]; }
            s += 1.0;
        }
    }
    FarOff { d, at }
}

// ------------------------------------------------------------ a door's frame and its name

/// How far off a door's frame line a point is (signed: sideOf's sign).   (orders.ts offLine)
pub fn off_line(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 {
    let (th, h) = (g.dr[d].def.closed_dir, g.dr[d].def.hinge);
    m::cos(th) * (p[2] - h[1]) - m::sin(th) * (p[0] - h[0])
}

/// Which face of the door's frame a point is on (+1 = the CCW side of the closed leaf).   (orders.ts sideOf)
pub fn side_of(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 {
    let (th, h) = (g.dr[d].def.closed_dir, g.dr[d].def.hinge);
    if m::cos(th) * (p[2] - h[1]) - m::sin(th) * (p[0] - h[0]) >= 0.0 { 1.0 } else { -1.0 }
}

/// A door's name as the squad says it.   (orders.ts doorWords)
pub fn door_words(g: &mut crate::game::Game, d: DoorId) -> String {
    if let Some(said) = crate::doornames::door_said_name(g, d) { return said; }
    let name = g.dr[d].def.name.clone();
    door_id_words(g, &name)
}

/// orders.ts:2692 the words of a door's id; nothing the game holds is read
fn id_words(id: &str) -> String {
    let said = door_said();
    let mut t: Vec<String> = id.split('_').map(|x| said.get(String::from(x)).cloned().unwrap_or_else(|| String::from(x))).collect();
    if t[t.len() - 1] != "door" { t.push(String::from("door")); }
    t.join(" ")
}

/// A door's id said as words: compass letters as words, no token dropped.   (orders.ts doorIdWords)
pub fn door_id_words(g: &mut crate::game::Game, id_2: &str) -> String { id_words(id_2) }

#[cfg(test)]
mod tests {
    use super::*;

    const A: [V3; 4] = [[0.1, 0.0, 0.2], [3.7, 0.0, 1.3], [4.1, 0.0, 6.9], [-2.25, 0.0, 7.77]];
    const B: [V3; 4] = [[0.3, 0.0, -0.4], [2.2, 0.0, 4.4], [2.2, 0.0, 4.4], [5.5, 0.0, 8.125]];

    fn bits(r: FarOff) -> [u64; 3] { [r.d.to_bits(), r.at[0].to_bits(), r.at[2].to_bits()] }

    /// the TypeScript's own farOff under EXACT, its doubles' bits
    #[test]
    fn far_off_has_the_typescripts_bits() {
        assert_eq!(bits(far_off(&A, &B)), [0x40165407ae62fe8f, 0xc002000000000000, 0x401f147ae147ae14]);
        assert_eq!(bits(far_off(&B, &A)), [0x4001200d1bb31168, 0x3ffae8ba2e8ba2e9, 0x4008ba2e8ba2e8bc]);
        assert_eq!(bits(far_off(&A, &[[1.0, 0.0, 1.0]])), [0x7ff0000000000000, 0x3fe199999999999a, 0x3fd599999999999a]);
        assert_eq!(bits(far_off(&[[1.5, 0.0, 2.5]], &B)), [0xbff0000000000000, 0x3ff8000000000000, 0x4004000000000000]);
    }

    /// the TypeScript's own doorIdWords, its strings
    #[test]
    fn id_words_say_what_the_typescript_says() {
        assert_eq!(id_words("breakroom_w"), "break room west door");
        assert_eq!(id_words("server_manager"), "server manager door");
        assert_eq!(id_words("n4a"), "n4a door");
        assert_eq!(id_words("mgr_door"), "manager's door");
        assert_eq!(id_words("ext_ne"), "outside north-east door");
        assert_eq!(id_words(""), " door");
    }

    #[test]
    fn a_set_of_groups_counts_as_sets_do() {
        assert_eq!(group_count(&[0.0, -0.0, 2.0], &[0, 1], &[0, 1]), 1.0);
        assert_eq!(group_count(&[0.0, 1.0, 2.0], &[0, 1, 2], &[0, 1, 2]), 3.0);
        assert_eq!(group_count(&[m::NAN, m::NAN], &[0, 1], &[0, 1]), 1.0);
        assert_eq!(group_count(&[1.0], &[0, 5, 6], &[0, 1, 2]), 2.0);
    }

    /// two men on one tight file: the one who must pass the other's mark costs, the other dealing does not
    #[test]
    fn a_squeeze_past_is_priced_one_way() {
        let spots: Vec<V3> = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        // man 0 walks through spot 0 to spot 1; man 1 is already at spot 0's far side, nearer spot 0 than man 0
        let f = FileCross::Tight { file: vec![1, 1], tight: vec![false, true], l: vec![vec![1.0, 2.0], vec![0.5, 1.5]], spots, cost: 500.0 };
        assert_eq!(file_cross_cost(&f, &[1, 0]), 500.0);
        assert_eq!(file_cross_cost(&f, &[0, 1]), 0.0);
    }
}
