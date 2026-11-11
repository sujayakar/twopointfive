pub use ::game_types::pairorders::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec, vec::Vec};

/// the living row of a man's job: a copy would lose the write
macro_rules! job { ($g:expr, $gd:expr) => { $g.pair.jobs.get_mut($gd).unwrap() } }
/// the assignment under a man, by its ID
macro_rules! asg { ($g:expr, $a:expr) => { $g.tab.orders_assignment[$a] } }

/// a man's name in a line, or 'that man'
pub(super) fn name_of(g: &Game, h: GuardId) -> String {
    if g.gd[h].callsign.is_empty() { String::from("that man") } else { g.gd[h].callsign.clone() }
}

/// where a man is LOOKING on the floor plane: his chest and his head, halved
fn look_of(g: &Game, h: GuardId) -> [f64; 2] {
    let l = crate::guards::look_of(g, g.gd[h].ch);
    [l[0], l[2]]
}

/// The quiet orders for one partner; the squad orders' people and note are hung here   (PairOrders.constructor)
pub fn pair_orders_new(g: &mut Game) -> crate::pairorders::PairOrders {
    // (SquadOrdersPeople has no fields: being set is what orders' squad_orders_people_near and _ray test)
    g.sets[g.seat].orders.people = Some(crate::orders::SquadOrdersPeople::default());
    crate::pairorders::PairOrders::blank()
}

// ------------------------------------------------------------ who, what
/// the rows are on the menu: the switch, a squad on the floor, and not more of it than a pair's orders are for   (PairOrders.up)
pub(super) fn pair_orders_up(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    if !g.k.pair.on { return false; }
    if !crate::orders::squad_orders_active(g, seat) { return false; }
    let n = crate::orders::squad_orders_squad(g, seat).len() as f64;
    n >= 1.0 && n <= g.k.pair.max_squad
}

/// the squad is quiet enough for hands   (PairOrders.quiet)
fn pair_orders_quiet(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    !g.k.silent.on || crate::silent::squad_silent(g, seat)
}

/// a man an order can be about: standing, on the other side from Sam, not in Sam's arm, and seen   (PairOrders.isFoe)
pub(super) fn pair_orders_is_foe(g: &Game, h: GuardId) -> bool {
    let team = g.team(h);
    !crate::condition::down(g, g.gd[h].ch)
        && team.theirs()
        && g.gd[h].held.is_none()
        && !crate::fogofwar::fog_of_war_hides(g, h)
}

/// he has noticed somebody: roused, searching, or his meter past the silent order's own `noticed`   (PairOrders.noticed)
fn pair_orders_noticed(g: &Game, h: GuardId) -> bool {
    let s = g.gd[h].state;
    s == crate::game::GuardState::Alert || s == crate::game::GuardState::Search || g.gd[h].awareness >= g.k.silent.noticed
}

/// the squad block's line while men wait on the mark: who takes whom, and what stands in his way
pub fn pair_orders_mark_note(g: &Game, seat: &crate::seats::SeatRef) -> Vec<crate::view::Note> {
    if !seated_in(g, seat) { return Vec::new(); }
    let rows: Vec<String> = pair_orders_waiting(g, seat).into_iter().map(|(gd, j)| match j.watcher {
        Some(w) => alloc::format!("{} takes {} anyway ({} is watching)", g.gd[gd].callsign, name_of(g, j.foe), name_of(g, w)),
        None => alloc::format!("{} takes {}{}", g.gd[gd].callsign, name_of(g, j.foe), if j.set { "" } else { " (not set)" }),
    }).collect();
    if rows.is_empty() { Vec::new() } else { alloc::vec![crate::view::Note { key: String::from("F"), text: alloc::format!("mark · {}", rows.join(" · ")) }] }
}

/// the men waiting on the mark; each row a COPY: a writer goes back to g.pair.jobs by the man   (PairOrders.waiting)
pub fn pair_orders_waiting(g: &Game, seat: &crate::seats::SeatRef) -> Vec<(GuardId, crate::pairorders::Job)> {
    let t = g.time;
    let mut out = Vec::new();
    for (gd, j) in g.pair.jobs.iter() {
        let waits = crate::orders::squad_orders_mine(g, seat, gd) && j.handed != Some(true)
            && ((j.mode == crate::pairorders::JobMode::Mark && !j.go)
                || (j.watcher.is_some() && !j.force && t - m::max(j.seen_t, j.back_t.unwrap_or(-1e9)) < 1.0))
            && !crate::condition::down(g, g.gd[gd].ch);
        if waits { out.push((gd, *j)); }
    }
    out
}

/// a man's job, to read   (PairOrders.jobOf)
pub fn pair_orders_job_of(g: &Game, gd: GuardId) -> Option<crate::pairorders::Job> {
    g.pair.jobs.get(gd).copied()
}

/// the partner's hands are on this man, for a bounded time   (PairOrders.gripped)
fn pair_orders_gripped(g: &Game, h: GuardId) -> bool {
    let (t, secs) = (g.time, g.k.pair.secs);
    for j in g.pair.jobs.values() {
        if j.foe == h && j.t0 >= 0.0 && t - j.t0 < secs + 0.5 && t >= j.t0 { return true; }
    }
    false
}

/// the spotted rule says the pair is CAUGHT
fn pair_orders_caught(g: &mut Game) -> bool {
    if !g.k.spotted.on { return false; }
    for gd in crate::orders::squad_orders_squad(g, &crate::seats::here(g)) {
        if !crate::condition::down(g, g.gd[gd].ch) && crate::spotted::spotted_mode(g, gd) == crate::spotted::SpottedMode::Caught { return true; }
    }
    false
}

/// his hands are on his man, or he is sent and on the last paces to him   (PairOrders.finishing)
pub(super) fn pair_orders_finishing(g: &Game, gd: GuardId) -> bool {
    let j = match g.pair.jobs.get(gd) { Some(j) => *j, None => return false };
    g.k.spotted.on && (j.t0 >= 0.0 || pair_orders_last_paces(g, j, gd))
}

/// sent, inside the ring, nothing holding him and nothing solid between him and the man   (PairOrders.lastPaces)
fn pair_orders_last_paces(g: &Game, j: crate::pairorders::Job, gd: GuardId) -> bool {
    let p = g.pos_of(gd);
    let c = g.pos_of(j.foe);
    j.going && j.watcher.is_none() && j.wait_t < 0.0 && v3::dist_xz(p, c) < g.k.pair.heel_ring
        && !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [c[0], 1.0, c[2]], None)
}

/// why a man an order pointed at is no longer one (None: he still is)   (PairOrders.gone)
pub(super) fn pair_orders_gone(g: &Game, h: GuardId) -> Option<String> {
    if crate::condition::down(g, g.gd[h].ch) || !g.guards.contains(&h) { return Some(String::from("he's down already.")); }
    if g.gd[h].held.is_some() { return Some(String::from("you've already got him.")); }
    if crate::fogofwar::fog_of_war_hidden(g, h) { return Some(String::from("lost sight of him.")); }
    None
}

/// a clear line from a teammate's eyes to a man's chest   (PairOrders.sees)
pub(super) fn pair_orders_sees(g: &Game, gd: GuardId, h: GuardId) -> bool {
    let a = g.pos_of(gd);
    let b = g.pos_of(h);
    !crate::scene_collision::segment_blocked(g, [a[0], 1.5, a[2]], [b[0], 1.2, b[2]], None, None, None)
}

/// is `p` in the half of the floor behind the way `h` looks now, or (`mostly`) mostly faces   (PairOrders.rear)
fn pair_orders_rear(g: &Game, h: GuardId, p: V3, mostly: Option<bool>) -> bool {
    let y = pair_orders_yaw_of(g, h);
    let [lx, lz] = if mostly.unwrap_or(false) { [m::sin(y), m::cos(y)] } else { look_of(g, h) };
    let c = g.pos_of(h);
    lx * (p[0] - c[0]) + lz * (p[2] - c[2]) < 0.25
}

/// a row of the log the probes and a lesson read; capped   (PairOrders.note)
fn pair_orders_note(g: &mut Game, what: &str) {
    let serial = g.serial();
    g.m.pairorders.pair_log.push(crate::pairorders::PairordersModPAIRLOGItem { what: String::from(what), serial });
    if g.m.pairorders.pair_log.len() > 200 { g.m.pairorders.pair_log.drain(0..50); }
}

// ------------------------------------------------------------ the rows
/// `issue` of the two take rows
pub(crate) fn take_issue(g: &mut Game, mode: crate::pairorders::JobMode, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    let foe = match t { crate::orders::OrderTarget::Foe(f) => f.foe, _ => return Some(String::from("nobody there")) };
    // (each mode is one row's)
    let id = match mode { crate::pairorders::JobMode::Mark => crate::orders::rows::pairmark::ORDER.id, crate::pairorders::JobMode::Now => crate::orders::rows::pairtake::ORDER.id };
    pair_orders_quiet_first(g);
    for gd in men.iter().copied() {
        let p = pair_orders_stage(g, foe);
        let was = g.pair.jobs.get(gd).map(|j| j.was_following);
        let was_following = match was {
            Some(w) => w,
            None => match crate::orders::squad_orders_of(&crate::seats::here(g), gd) { Some(a) => asg!(g, a).post.is_none(), None => false },
        };
        let now = g.time;
        g.pair.jobs.set(gd, crate::pairorders::Job {
            foe, mode, go: mode == crate::pairorders::JobMode::Now, set: false, set_said: false, t0: -1.0, born: now, check_t: -1.0, wait_t: -1.0, heel_t: -1.0,
            going: false, scuffled: false, was_following, handed: None, watcher: None, watch_t: -1.0, seen_t: -1.0, force: false, cue_t: -1e9, cued: false,
            back_said: None, back_t: None,
        });
        let name = name_of(g, foe);
        let pace = g.k.pair.pace;
        crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
            order: String::from(id), post: Some(p), face: None, foe: Some(foe), crouch: Some(true), pace: Some(pace),
            stance: Some(crate::character::Stance::LowReady), upper: Some(crate::character::UpperMode::None),
            going: format!("moving up on {}", name).into(), there: format!("on {}", name).into(), ..Default::default()
        });
        let line = format!("{}:{} on {}", mode.as_str(), g.gd[gd].callsign, name);
        pair_orders_note(g, &line);
    }
    None
}

/// `issue` of 'watch him'
pub(crate) fn watch_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    let foe = match t { crate::orders::OrderTarget::Foe(f) => f.foe, _ => return Some(String::from("nobody there")) };
    for gd in men.iter().copied() {
        let p = g.pos_of(gd);
        let c = g.pos_of(foe);
        let words = format!("watching {}", name_of(g, foe));
        g.pair.jobs.delete(gd);
        crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
            order: String::from(crate::orders::rows::pairwatch::ORDER.id), post: Some(crate::v3::on_floor(p)), face: Some(v3::yaw_to(p, c)), watch: Some([c[0], 1.1, c[2]]), foe: Some(foe),
            stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), going: words.clone().into(), there: words.into(), ..Default::default()
        });
    }
    None
}

/// `issue` of 'hold in the dark'
pub(crate) fn dark_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    let at = t.at();
    let sc = g.pl[g.player].ch;
    let sam = g.ch[sc].pos;
    let face = if v3::dist_xz(sam, at) > 0.3 { v3::yaw_to(sam, at) } else { g.ch[sc].body_yaw };
    let d = pair_orders_dark_spot(g, at);
    let to = d.unwrap_or(at);
    let spots = if men.len() > 1 { crate::orders::squad_orders_spots_at(g, to, face, men.len() as f64) } else { vec![to] };
    for (gd, p) in crate::orders::squad_orders_pair(g, men, &spots, None, None) {
        g.pair.jobs.delete(gd);
        g.pair.dark.set(gd, crate::pairorders::PairOrdersDarkValue { check_t: -1.0, lit_t: -1.0, said_t: -1e9 });
        crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
            order: String::from(crate::orders::rows::pairdark::ORDER.id), post: Some(crate::v3::on_floor(p)), face: Some(face), crouch: Some(true),
            stance: Some(crate::character::Stance::LowReady), upper: Some(crate::character::UpperMode::None),
            going: "moving · to the dark".into(), there: "holding in the dark".into(), set_line: Some(String::from("set.")), ..Default::default()
        });
        let line = format!("dark:{} {},{}{}", g.gd[gd].callsign, js::to_fixed(p[0], 1), js::to_fixed(p[2], 1), if d.is_some() { "" } else { " (lit)" });
        pair_orders_note(g, &line);
        if d.is_none() { crate::game::game_say(g, gd, "it's lit there.", None, None); }
    }
    None
}

/// `can` of the two take rows: there is room at his back and the man can reach it
pub fn row_take_can(g: &mut Game, gd: GuardId, t: &crate::orders::OrderTarget) -> Option<String> {
    let crate::orders::OrderTarget::Foe(x) = t else {
        return Some(String::from("nobody there."));
    };
    let to = crate::pairorders::pair_orders_behind(g, x.foe);
    let spot = crate::orders::squad_orders_free_spot(g, &crate::seats::here(g), to).unwrap_or(to);
    if !crate::pairorders::pair_orders_back_open(g, x.foe, spot) || !crate::pairorders::pair_orders_rear(g, x.foe, spot, Some(true)) {
        return Some(String::from("no room behind him."));
    }
    let stage = crate::pairorders::pair_orders_stage(g, x.foe);
    crate::orders::squad_orders_reach(g, gd, stage)
}
/// `only` of the two take rows: the nearest man who can reach his back
pub fn row_take_only(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    let crate::orders::OrderTarget::Foe(x) = t else { return Vec::new() };
    let to = crate::pairorders::pair_orders_stage(g, x.foe);
    let mut out: Vec<GuardId> = Vec::new();
    for &gd in men { if crate::orders::squad_orders_reach(g, gd, to).is_none() { out.push(gd); } }
    let there = g.pos_of(x.foe);
    let rows: &Game = &*g;
    js::sort_by(&mut out, |&a, &b| v3::dist_xz(rows.pos_of(a), there) - v3::dist_xz(rows.pos_of(b), there));
    out.truncate(1);
    out
}
/// `valid` of the two take rows
pub fn row_take_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let crate::orders::OrderTarget::Foe(x) = t else { return Err(None) };
    if let Some(w) = crate::pairorders::pair_orders_gone(g, x.foe) { return Err(Some(w)); }
    if crate::pairorders::pair_orders_noticed(g, x.foe) {
        return Err(Some(String::from("he's fighting. can't grab him.")));
    }
    if crate::pairorders::pair_orders_caught(g) {
        return Err(Some(String::from("they're onto us. can't grab him.")));
    }
    Ok(())
}
/// one of the two take rows
fn take_row(g: &mut Game, row: &'static crate::orders::Row, ack: [&str; 2]) -> OrdersOrderDefId {
    let def = crate::orders::rows::def(row, crate::orders::OrderDef {
        targets: vec![crate::orders::TargetKind::Foe],
        ack: Some(vec![String::from(ack[0]), String::from(ack[1])]),
        ..Default::default()
    });
    crate::orders::squad_orders_row(g, def)
}

/// mark, watch, now, dark: new rows each time they are seated   (PairOrders.rows)
fn pair_orders_rows(g: &mut Game) -> Vec<OrdersOrderDefId> {
    use crate::orders::rows;
    let mark = take_row(g, &rows::pairmark::ORDER, ["moving up on him.", "on him. your mark."]);
    let watch = crate::orders::squad_orders_row(g, rows::def(&rows::pairwatch::ORDER, crate::orders::OrderDef {
        targets: vec![crate::orders::TargetKind::Foe],
        ack: Some(vec![String::from("eyes on him."), String::from("i've got him.")]),
        ..Default::default()
    }));
    let now = take_row(g, &rows::pairtake::ORDER, ["on him.", "moving up on him."]);
    let dark = crate::orders::squad_orders_row(g, rows::def(&rows::pairdark::ORDER, crate::orders::OrderDef {
        targets: vec![crate::orders::TargetKind::Floor],
        ack: Some(vec![String::from("moving."), String::from("on it.")]),
        stands_in: Some(crate::orders::OrderDefStandsIn { for_: String::from(crate::orders::rows::movethere::ORDER.id) }),
        ..Default::default()
    }));
    vec![mark, watch, now, dark]
}

/// the mark's own row: 'go' under the word's header   (PairOrders.goRow)
fn pair_orders_go_row(g: &mut Game) -> OrdersOrderDefId {
    use crate::orders::rows;
    crate::orders::squad_orders_row(g, rows::def(&rows::pairgo::ORDER, crate::orders::OrderDef {
        word: true, free: Some(true),
        targets: vec![crate::orders::TargetKind::Floor, crate::orders::TargetKind::Door, crate::orders::TargetKind::None, crate::orders::TargetKind::Foe],
        ..Default::default()
    }))
}

/// a row here given while the squad is loud: silent first, with the squad's own line for it   (PairOrders.quietFirst)
fn pair_orders_quiet_first(g: &mut Game) {
    if pair_orders_quiet(g, &crate::seats::here(g)) { return; }
    crate::silent::set_silent(g, true);
}

/// The rows on the menu while `up` and off it otherwise; asked every frame, work only when something changes   (PairOrders.seat)
fn pair_orders_seat(g: &mut Game) {
    let up = pair_orders_up(g, &crate::seats::here(g));
    if up != seated_in(g, &crate::seats::here(g)) {
        for r in crate::orders::rows::ROWS.iter().filter(|r| r.is.contains(&crate::orders::Is::Pair)) { crate::orders::squad_orders_unregister(g, r.id); }
        for id in [crate::orders::rows::fallin::ORDER.id, crate::orders::rows::hold::ORDER.id] {
            let found = g.sets[g.seat].orders.defs.iter().copied().find(|x| g.tab.orders_order_def[*x].id == id);
            let d = match found { Some(d) => d, None => continue };
            let has = g.tab.orders_order_def[d].targets.contains(&crate::orders::TargetKind::Foe);
            if up && !has { g.tab.orders_order_def[d].targets.push(crate::orders::TargetKind::Foe); }
            else if !up && has { g.tab.orders_order_def[d].targets.retain(|k| *k != crate::orders::TargetKind::Foe); }
        }
        if up {
            let r = pair_orders_rows(g);
            crate::orders::squad_orders_register(g, r[0], Some(crate::orders::rows::cover::ORDER.id));
            crate::orders::squad_orders_register(g, r[1], Some(crate::orders::rows::cover::ORDER.id));
            crate::orders::squad_orders_register(g, r[2], None);
            crate::orders::squad_orders_register(g, r[3], None);
        } else {
            pair_orders_reset(g);
        }
    }
    let waits = up && !pair_orders_waiting(g, &crate::seats::here(g)).is_empty();
    if waits != super::squad40::def_listed(g, &crate::seats::here(g), crate::orders::rows::pairgo::ORDER.id) {
        crate::orders::squad_orders_unregister(g, crate::orders::rows::pairgo::ORDER.id);
        if waits { let r = pair_orders_go_row(g); crate::orders::squad_orders_register(g, r, None); }
    }
}

/// the rows are off the menu: nobody is left on one of them   (PairOrders.reset)
fn pair_orders_reset(g: &mut Game) {
    for gd in crate::orders::squad_orders_squad(g, &crate::seats::here(g)) {
        if let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), gd) {
            if crate::orders::rows::is(g, a, crate::orders::Is::Pair) { pair_orders_stand(g, gd, "holding", None); }
        }
    }
    let seat = crate::seats::here(g);
    let gone: Vec<GuardId> = g.pair.jobs.keys().into_iter().filter(|gd| crate::orders::squad_orders_mine(g, &seat, *gd)).collect();
    for gd in gone { g.pair.jobs.delete(gd); }
    g.pair.dark_memo.clear();
    let pl = g.player;
    g.pl[pl].pair_hands_was = false;
}

// ------------------------------------------------------------ once a frame
/// F: the mark, while somebody waits on it; the press is used here and a menu that is up goes away   (PairOrders.input)
pub fn pair_orders_input(g: &mut Game, input: InputId) {
    if !seated_in(g, &crate::seats::here(g)) || !g.tab.input[input].pressed.holds("KeyF") || crate::condition::down(g, g.pl[g.player].ch) || pair_orders_waiting(g, &crate::seats::here(g)).is_empty() { return; }
    g.tab.input[input].pressed.delete(String::from("KeyF"));
    crate::orders::squad_orders_close(g);
    pair_orders_mark(g, Some(crate::pairorders::PairOrdersMarkBy::F));
}

/// Once a frame before the orders pose the men: the rows, where every defender mostly looks, Sam's hands as the mark, stale jobs forgotten   (PairOrders.tick)
pub fn pair_orders_tick(g: &mut Game, dt: f64) {
    crate::seats::each_seat(g, &mut |g| pair_orders_seat(g));
    pair_orders_summary(g);
    if !g.pair.seated { pair_orders_hands_summary(g); return; }
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        if crate::condition::down(g, g.gd[h].ch) || !g.team(h).theirs() { continue; }
        let [fx, fz] = look_of(g, h);
        let mut hb = match g.pair.habit.get(h) {
            Some(v) => *v,
            None => {
                g.pair.habit.set(h, [fx, fz]);
                g.pair.glance.set(h, crate::pairorders::PairOrdersGlanceValue { on: false, end_t: -1e9, h0: None });
                continue;
            }
        };
        let vel = g.ch[g.gd[h].ch].vel;
        let st = if m::hyp2(vel[0], vel[2]) > 0.3 { 0.0 } else { g.pair.stood.get(h).copied().unwrap_or(0.0) + dt };
        g.pair.stood.set(h, st);
        let k = m::min(1.0, dt / (if st > 0.0 { m::max(0.5, m::min(6.0, st)) } else { 6.0 }));
        hb[0] += (fx - hb[0]) * k;
        hb[1] += (fz - hb[1]) * k;
        g.pair.habit.set(h, hb);
        let l = js::or(m::hyp2(hb[0], hb[1]), 1.0);
        let c = (fx * hb[0] + fz * hb[1]) / l;
        let sw = if st > 2.5 { m::max(m::acos(m::max(-1.0, m::min(1.0, c))), g.pair.swept.get(h).copied().unwrap_or(0.0) - dt * 0.025) } else { 0.0 };
        g.pair.swept.set(h, sw);
        let now = g.time;
        let mut forget = false;
        let gl = g.pair.glance.get_mut(h).unwrap();
        if !gl.on && c < 0.77 {
            gl.on = true;
            gl.h0 = Some([hb[0] / l, hb[1] / l]);
        } else if gl.on && c > 0.94 {
            gl.on = false;
            gl.end_t = now;
            if let Some(h0) = gl.h0 { forget = (h0[0] * hb[0] + h0[1] * hb[1]) / l < 0.866; }
        }
        if forget { g.pair.swept.set(h, 0.0); }
    }
    crate::seats::each_seat(g, &mut |g| if seated_in(g, &crate::seats::here(g)) { pair_orders_mine(g); });
    pair_orders_hands_summary(g);
}

fn seated_in(g: &Game, seat: &crate::seats::SeatRef) -> bool { super::squad40::def_listed(g, seat, crate::orders::rows::pairmark::ORDER.id) }
/// the game's own summary of the rows: up on some seat's menu
fn pair_orders_summary(g: &mut Game) {
    let up = g.seats.iter().any(|&s| seated_in(g, &crate::seats::seat(g, s)));
    g.pair.seated = up;
}
fn pair_orders_hands_summary(g: &mut Game) { g.pair.hands_was = crate::seats::men(g).into_iter().any(|p| g.pl[p].pair_hands_was); }

/// this seat's man and this seat's jobs, standing on the seat
fn pair_orders_mine(g: &mut Game) {
    let pl = g.player;
    let hands = g.pl[pl].on_a_man();
    if hands && !g.pl[pl].pair_hands_was { pair_orders_mark(g, Some(crate::pairorders::PairOrdersMarkBy::Hands)); }
    g.pl[pl].pair_hands_was = hands;
    let mut at = 0;
    while let Some(gd) = g.pair.jobs.next_key(&mut at) {
        if !crate::orders::squad_orders_mine(g, &crate::seats::here(g), gd) { continue; }
        let foe = g.pair.jobs.get(gd).unwrap().foe;
        let a = crate::orders::squad_orders_of(&crate::seats::here(g), gd);
        if crate::condition::down(g, g.gd[gd].ch) { g.pair.jobs.delete(gd); continue; }
        let kept = match a { Some(a) => crate::orders::rows::is(g, a, crate::orders::Is::Take) && asg!(g, a).foe == Some(foe), None => false };
        if !kept {
            g.pair.jobs.delete(gd);
            let foe_name = name_of(g, foe);
            let line = format!("cancel:{} off {}", g.gd[gd].callsign, foe_name);
            pair_orders_note(g, &line);
        }
    }
}

/// THE MARK: every man waiting on it goes for his man (false: nobody was waiting)   (PairOrders.mark)
pub fn pair_orders_mark(g: &mut Game, by: Option<crate::pairorders::PairOrdersMarkBy>) -> bool {
    use crate::pairorders::PairOrdersMarkBy as By;
    let by = by.unwrap_or(By::Api);
    let w = pair_orders_waiting(g, &crate::seats::here(g));
    if w.is_empty() { return false; }
    let (gd0, j0) = w[0];
    let again = j0.watcher.is_some();
    let set = j0.set;
    for (gd, j) in w.iter().copied() {
        let near = by == By::Hands && v3::dist_xz(g.pos_of(gd), g.pos_of(j.foe)) < g.k.pair.heel_ring;
        if j.watcher.is_some() || near {
            if j.watcher.is_some() {
                let foe_name = name_of(g, j.foe);
                let line = format!("anyway:{} on {}", g.gd[gd].callsign, foe_name);
                pair_orders_note(g, &line);
            }
            job!(g, gd).force = true;
        }
        let now = g.time;
        let r = job!(g, gd);
        r.watcher = None;
        r.go = true;
        r.going = true;
        r.born = now;
        r.check_t = -1.0;
        if let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), gd) { asg!(g, a).pace = Some(g.k.pair.go_pace); }
    }
    g.stats.pairorders.marks += 1;
    let line = format!("go:{}", by.as_str());
    pair_orders_note(g, &line);
    // J0 is the row the walk above wrote to: read it again, a copy from before would still hold the watcher and no force
    let j0 = *g.pair.jobs.get(gd0).unwrap();
    if again { crate::game::game_say(g, gd0, "going anyway.", None, None); }
    else if j0.force || !pair_orders_held(g, j0, gd0) { crate::game::game_say(g, gd0, if set { "moving." } else { "moving. not set yet." }, None, None); }
    true
}

/// WHO WOULD SEE IT: the first other defender with eyes on `gd` and light enough, or the floor the man will lie on in his open view   (PairOrders.watcher)
fn pair_orders_watcher(g: &mut Game, foe: GuardId, gd: GuardId) -> Option<GuardId> {
    let fc = g.gd[foe].ch;
    let hp = g.ch[fc].bone(crate::character::Bone::Hips).unwrap_or(g.ch[fc].pos);
    let at: V3 = [hp[0], 0.35, hp[2]];
    let gc = g.gd[gd].ch;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let team = g.team(o);
        if o == foe || crate::condition::down(g, g.gd[o].ch) || g.gd[o].held.is_some() || !team.theirs() { continue; }
        let s = crate::guards::can_see_char(g, o, gc);
        if s.visible && crate::guards::sight_gain_on(g, o, gd, s.dist) * g.k.pair.watch_secs >= g.k.pair.watch_aw { return Some(o); }
        if team != g.team(foe) || !crate::guards::point_in_view(g, o, at, None, None) { continue; }
        if pair_orders_body_lit(g, o, foe, at) { return Some(o); }
    }
    None
}

/// the eye of a man: his head bone, or where it would be
fn eye_of(g: &Game, o: GuardId) -> V3 {
    let c = g.gd[o].ch;
    let p = g.ch[c].pos;
    g.ch[c].bone(crate::character::Bone::Head).unwrap_or([p[0], p[1] + 1.65, p[2]])
}

/// the bodies-by-light bar from `o`'s eye, met by the light now on the man   (PairOrders.bodyLit)
fn pair_orders_body_lit(g: &mut Game, o: GuardId, foe: GuardId, at: V3) -> bool {
    let eye = eye_of(g, o);
    let d = m::hyp3(eye[0] - at[0], eye[1] - at[1], eye[2] - at[2]);
    let (near_m, far_m, lit, lit_near) = (g.k.body_light.near_m, g.k.body_light.far_m, g.k.body_light.lit, g.k.body_light.lit_near);
    let bar = if !g.tune.body_light || d <= near_m { -1.0 } else { lit_near + (lit - lit_near) * m::min(1.0, (d - near_m) / m::max(0.1, far_m - near_m)) };
    let vis = g.shown[g.gd[foe].ch].visibility;
    m::max(vis, pair_orders_lit_at(g, at)) >= bar
}

/// inside what `w`'s eyes sweep, with a line to it: the distance, or −1
fn swept_d(g: &Game, eye: V3, hb: [f64; 2], l: f64, half: f64, x: f64, y: f64, z: f64) -> f64 {
    let dx = x - eye[0];
    let dz = z - eye[2];
    let d = m::hyp2(dx, dz);
    if d > g.k.guard_eye.range || d < 0.05 { return -1.0; }
    if m::acos(m::max(-1.0, m::min(1.0, (dx * hb[0] + dz * hb[1]) / (d * l)))) < half && !crate::scene_collision::segment_blocked(g, [eye[0], eye[1], eye[2]], [x, y, z], None, None, None) { d } else { -1.0 }
}

/// HE'LL LOOK BACK: the man who was watching has only turned his head   (PairOrders.looksBack)
fn pair_orders_looks_back(g: &mut Game, w: GuardId, foe: GuardId, gd: GuardId) -> bool {
    if crate::condition::down(g, g.gd[w].ch) || g.gd[w].held.is_some() || g.pair.stood.get(w).copied().unwrap_or(0.0) < 0.5 { return false; }
    let hb = match g.pair.habit.get(w) { Some(v) => *v, None => return false };
    let l = m::hyp2(hb[0], hb[1]);
    if l < 0.3 { return false; }
    let eye = eye_of(g, w);
    let half = g.k.guard_eye.half_deg * m::PI / 180.0 + g.pair.swept.get(w).copied().unwrap_or(0.0);
    let fc = g.gd[foe].ch;
    let hp = g.ch[fc].bone(crate::character::Bone::Hips).unwrap_or(g.ch[fc].pos);
    let at: V3 = [hp[0], 0.35, hp[2]];
    if g.team(w) == g.team(foe) && swept_d(g, eye, hb, l, half, at[0], at[1], at[2]) >= 0.0 && pair_orders_body_lit(g, w, foe, at) { return true; }
    let c = g.pos_of(gd);
    let d = swept_d(g, eye, hb, l, half, c[0], c[1] + 1.0, c[2]);
    d >= 0.0 && crate::guards::sight_gain_on(g, w, gd, d) * g.k.pair.watch_secs >= g.k.pair.watch_aw
}

/// would the mark, given now, find this man held up by a watcher   (PairOrders.held)
fn pair_orders_held(g: &mut Game, j: crate::pairorders::Job, gd: GuardId) -> bool {
    if j.force || j.t0 >= 0.0 { return false; }
    if !(j.watcher.is_some() || v3::dist_xz(g.pos_of(gd), g.pos_of(j.foe)) < g.k.pair.heel_ring) { return false; }
    if pair_orders_watcher(g, j.foe, gd).is_some() { return true; }
    match j.watcher { Some(w) => pair_orders_looks_back(g, w, j.foe, gd), None => false }
}

/// off the job, onto a plain hold where he stands, with his gun on `aim` when the job ended in the man being roused   (PairOrders.stand)
pub(super) fn pair_orders_stand(g: &mut Game, gd: GuardId, words: &str, aim: Option<GuardId>) {
    let p = g.pos_of(gd);
    let here = crate::v3::on_floor(p);
    g.pair.jobs.delete(gd);
    let a = match aim {
        Some(aim) => {
            let c = g.pos_of(aim);
            crate::orders::SquadOrdersAssignA {
                order: String::from(crate::orders::rows::hold::ORDER.id), post: Some(here), face: Some(v3::yaw_to(p, c)), watch: Some([c[0], 1.1, c[2]]),
                stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), going: String::from(words).into(), there: String::from(words).into(), ..Default::default()
            }
        }
        None => crate::orders::SquadOrdersAssignA {
            order: String::from(crate::orders::rows::hold::ORDER.id), post: Some(here), face: Some(g.ch[g.gd[gd].ch].body_yaw),
            stance: Some(crate::character::Stance::LowReady), upper: Some(crate::character::UpperMode::None), going: String::from(words).into(), there: String::from(words).into(), ..Default::default()
        },
    };
    crate::orders::squad_orders_assign(g, gd, &a);
}

// ------------------------------------------------------------ taking a man
/// the way a man mostly faces, as a yaw   (PairOrders.yawOf)
fn pair_orders_yaw_of(g: &Game, foe: GuardId) -> f64 {
    if let Some(hb) = g.pair.habit.get(foe) {
        if m::hyp2(hb[0], hb[1]) > 0.3 { return m::atan2(hb[0], hb[1]); }
    }
    g.ch[g.gd[foe].ch].body_yaw
}

/// `m` behind the way he mostly faces   (PairOrders.back)
fn pair_orders_back(g: &Game, foe: GuardId, m: f64) -> V3 {
    let c = g.pos_of(foe);
    let y = pair_orders_yaw_of(g, foe);
    [c[0] - m::sin(y) * m, 0.0, c[2] - m::cos(y) * m]
}

/// is `p` floor at `foe`'s back with nothing solid between it and him   (PairOrders.backOpen)
fn pair_orders_back_open(g: &Game, foe: GuardId, p: V3) -> bool {
    let c = g.pos_of(foe);
    !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [c[0], 1.0, c[2]], None)
}

/// where the hands go on from: a short pace behind the way he MOSTLY faces, as far as there is floor   (PairOrders.behind)
fn pair_orders_behind(g: &Game, foe: GuardId) -> V3 {
    for d in [0.8, 0.6, 0.45] {
        let p = pair_orders_back(g, foe, d);
        if pair_orders_back_open(g, foe, p) { return p; }
    }
    pair_orders_back(g, foe, 0.45)
}

/// where a man waits on the mark: `stageM` behind him, nearer where the floor is short   (PairOrders.stage)
fn pair_orders_stage(g: &Game, foe: GuardId) -> V3 {
    for k in [1.0, 0.8, 0.6] {
        let p = pair_orders_back(g, foe, g.k.pair.stage_m * k);
        if !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) && pair_orders_back_open(g, foe, p) { return p; }
    }
    pair_orders_behind(g, foe)
}

/// the job ends in words, and he stands where he is
fn take_off(g: &mut Game, me: &Take, why: &str, line: Option<&str>, aim: bool) {
    if let Some(l) = line { crate::game::game_say(g, me.gd, l, None, None); }
    let n = format!("off:{}", why);
    pair_orders_note(g, &n);
    if aim { let w = format!("covering {}", me.name); pair_orders_stand(g, me.gd, &w, Some(me.foe)); } else { pair_orders_stand(g, me.gd, "holding", None); }
}

/// sent from Sam's shoulder, he comes back to it
fn take_rejoin(g: &mut Game, gd: GuardId, was_following: bool) -> bool {
    if !crate::pairup::pair_up(g) || !was_following || crate::condition::down(g, g.pl[g.player].ch) { return false; }
    crate::game::game_say(g, gd, "he's down. on you.", None, None);
    g.pair.jobs.delete(gd);
    let n = format!("rejoin:{}", g.gd[gd].callsign);
    pair_orders_note(g, &n);
    crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
        order: String::from(crate::orders::rows::fallin::ORDER.id), post: None, follow: Some(0.0), face: None, stance: Some(crate::character::Stance::LowReady), upper: Some(crate::character::UpperMode::None),
        going: "falling in".into(), there: "with you".into(), ..Default::default()
    });
    true
}

/// A new post where he is told, with a fresh leg to it and no way round.
fn new_post(g: &mut Game, a: OrdersAssignmentId, at: V3, face: Option<f64>) {
    let r = &mut asg!(g, a);
    r.post = Some(at);
    if r.had == Some(at) { r.had = None; }   // (`had` is asked by value: a mark he had that holds these numbers is not this one, so that one goes; any other stays)
    r.goal = [at[0], 0.0, at[2]]; r.new_leg = true;   // (a new goal is a new leg, even at the same spot)
    r.via.clear();
    r.face = face;
}

/// parked where he stands, square to the man
fn park_on(g: &mut Game, me: &Take) {
    let c = g.ch[me.fc].pos;
    new_post(g, me.a, me.here, Some(v3::yaw_to(me.p, c)));
}

fn set_words(g: &mut Game, a: OrdersAssignmentId, words: String) {
    asg!(g, a).there = words.clone().into();
    asg!(g, a).going = words.into();
}

/// One man's take: who is on whom, this frame.
struct Take {
    gd: GuardId,
    a: OrdersAssignmentId,
    foe: GuardId,
    was_following: bool,
    gc: CharId,
    fc: CharId,
    /// where he stands: nothing here moves him before the frame's end
    p: V3,
    here: V3,
    name: String,
    /// his hands are on the man
    on: bool,
}

/// One man's job, a frame at a time: it ends if the man is down, had or has noticed; he holds while watched; his hands go on when he is in reach; else the walk to the man's back.
pub fn pair_orders_take_tick(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) {
    let Some(j0) = g.pair.jobs.get(gd).copied() else { pair_orders_stand(g, gd, "holding", None); return };
    let foe = j0.foe;
    let (gc, fc) = (g.gd[gd].ch, g.gd[foe].ch);
    let p = g.ch[gc].pos;
    let me = Take { gd, a, foe, was_following: j0.was_following, gc, fc, p, here: crate::v3::on_floor(p), name: name_of(g, foe), on: j0.t0 >= 0.0 };
    if crate::condition::down(g, g.gd[foe].ch) || !g.guards.contains(&foe) { his_man_is_down(g, &me); return; }
    if g.gd[foe].held.is_some() { take_off(g, &me, "held", Some("he's yours."), false); return; }
    if !me.on && (j0.handed == Some(true) || pair_orders_noticed(g, foe)) { blown(g, &me); return; }
    if !me.on && !pair_orders_quiet(g, &crate::seats::here(g)) { take_off(g, &me, "loud", Some("going loud."), true); return; }
    let d = v3::dist_xz(p, g.ch[fc].pos);
    let behind_him = pair_orders_rear(g, foe, p, None);
    let live = *job!(g, gd);
    if !me.on && g.k.spotted.on && crate::spotted::spotted_mode(g, gd) == crate::spotted::SpottedMode::Caught && !pair_orders_last_paces(g, live, gd) { caught(g, &me); return; }
    if live.go && !me.on && pair_orders_held(g, live, gd) { he_is_watched(g, &me); return; }
    if watcher_just_gone(g, gd) { return; }
    if job!(g, gd).watcher.is_some() { clear_to_move(g, &me); }
    if in_reach(g, &me, d, behind_him) { hands_on(g, &me, d); return; }
    if job!(g, gd).t0 >= 0.0 { crate::takedown::partner_cancel(g, gd, foe); }
    job!(g, gd).t0 = -1.0;
    let staging = !job!(g, gd).go;
    if !(staging && job!(g, gd).set) && g.time - job!(g, gd).born > g.k.pair.take_secs {
        take_off(g, &me, "time", Some("can't get behind him."), false);
        return;
    }
    if g.time - job!(g, gd).check_t < g.k.pair.stalk_every && asg!(g, a).post.is_some() { return; }
    let t = g.time;
    job!(g, gd).check_t = t;
    let want = if staging { pair_orders_stage(g, foe) } else { pair_orders_behind(g, foe) };
    let spot = crate::orders::squad_orders_free_spot(g, &crate::seats::here(g), want).unwrap_or(want);
    if staging && job!(g, gd).set && behind_him && v3::dist_xz(p, spot) < 0.7 && !crate::scene_collision::low_blocked(g, me.here, spot) { park_on(g, &me); return; }
    // the way to his back: he WAITS where he is while the only way lies across somebody's eyes, goes when it is clear, and says so if it never is
    let pace = if job!(g, gd).going { g.k.pair.go_pace } else { g.k.pair.pace };
    let way = pair_orders_stalk(g, gd, spot, Some(pace));
    if (way.exposed && !job!(g, gd).force) || !pair_orders_back_open(g, foe, spot) { waits_for_a_way(g, &me, &way); return; }
    job!(g, gd).wait_t = -1.0;
    let stops_here = if staging { set_on_the_stage(g, &me, spot) } else { waits_on_a_glance(g, &me, d) };
    if !stops_here { walks_on(g, &me, spot, &way); }
}

/// His man is down: a finish of his own is counted, and he comes back to Sam or stands where he is.
fn his_man_is_down(g: &mut Game, me: &Take) {
    if job!(g, me.gd).handed == Some(true) && g.gd[me.foe].killer_id == g.ch[me.gc].id {
        g.stats.pairup.finishes += 1;
        let n = format!("finish:{} on {}", g.gd[me.gd].callsign, me.name);
        pair_orders_note(g, &n);
    }
    if take_rejoin(g, me.gd, me.was_following) { pair_orders_note(g, "off:down"); } else { take_off(g, me, "down", Some("he's down."), false); }
}

/// The man has noticed: his partner takes him by hand if he can, else the job ends with his gun on the man.
fn blown(g: &mut Game, me: &Take) {
    let gd = me.gd;
    if crate::pairup::pair_up(g) && crate::spotted::spotted_take(g, gd, me.foe) {
        if job!(g, gd).handed != Some(true) {
            job!(g, gd).handed = Some(true);
            let n = format!("blown:{} by {}", g.gd[gd].callsign, me.name);
            pair_orders_note(g, &n);
            let n = format!("handed:{} on {}", g.gd[gd].callsign, me.name);
            pair_orders_note(g, &n);
        }
        asg!(g, me.a).crouch = Some(false);
        set_words(g, me.a, format!("taking {} by hand", me.name));
        return;
    }
    if job!(g, gd).handed != Some(true) {
        let n = format!("blown:{} by {}", g.gd[gd].callsign, me.name);
        pair_orders_note(g, &n);
    }
    let seen_me = g.gd[me.foe].foe == Some(g.gd[gd].ch);
    take_off(g, me, "blown", Some(if seen_me { "he's seen me!" } else { "he's onto us." }), true);
}

/// The pair is caught short of the last paces: the job ends with his gun on whoever caught him.
fn caught(g: &mut Game, me: &Take) {
    let gd = me.gd;
    let by = crate::spotted::spotted_by(g, gd);
    let tail = match by { Some(b) => format!(" by {}", name_of(g, b)), None => String::new() };
    let n = format!("blown:{} caught{}", g.gd[gd].callsign, tail);
    pair_orders_note(g, &n);
    pair_orders_note(g, "off:caught");
    let w = match by { Some(b) => format!("covering {}", name_of(g, b)), None => String::from("holding") };
    pair_orders_stand(g, gd, &w, by);
}

/// HE'S WATCHED: he does not go; he says who, stays where he is, and goes the moment that man looks away. A take on the spot gives up when it has waited too long.
fn he_is_watched(g: &mut Game, me: &Take) {
    let gd = me.gd;
    let now = pair_orders_watcher(g, me.foe, gd);
    let Some(w) = now.or(job!(g, gd).watcher) else { return };
    let wn = name_of(g, w);
    let words = if now.is_none() {
        will_look_back(g, me, &wn);
        format!("on {} · {} will look back · press F to go anyway", me.name, wn)
    } else {
        eyes_on_him(g, me, w, &wn);
        format!("on {} · WATCHED by {} · press F to go anyway", me.name, wn)
    };
    park_on(g, me);
    set_words(g, me.a, words);
    if job!(g, gd).mode == crate::pairorders::JobMode::Now && g.time - job!(g, gd).watch_t > g.k.pair.heel_wait {
        take_off(g, me, "watched", Some("someone's watching him. no opening."), false);
    }
}

/// The watcher has only turned his head: he says so, once, if he had called the watcher.
fn will_look_back(g: &mut Game, me: &Take, wn: &str) {
    let gd = me.gd;
    let (seen_t, cued, back_said) = { let r = job!(g, gd); (r.seen_t, r.cued, r.back_said) };
    if g.time - seen_t > 0.4 && cued && back_said != Some(true) {
        crate::game::game_say(g, gd, "he'll look back. your call.", None, None);
        job!(g, gd).back_said = Some(true);
        let n = format!("lookback:{} by {}", me.name, wn);
        pair_orders_note(g, &n);
    }
    let t = g.time;
    job!(g, gd).back_t = Some(t);
}

/// A man has eyes on him: noted, and called if it has been a while since the last call.
fn eyes_on_him(g: &mut Game, me: &Take, w: GuardId, wn: &str) {
    let gd = me.gd;
    if job!(g, gd).watcher.is_none() {
        let t = g.time;
        if t - job!(g, gd).seen_t > 2.0 { job!(g, gd).watch_t = t; }
        let n = format!("watched:{} by {}", me.name, wn);
        pair_orders_note(g, &n);
        if t - job!(g, gd).cue_t > 8.0 {
            let l = format!("hold. {} has eyes on him.", wn);
            crate::game::game_say(g, gd, &l, None, None);
            let r = job!(g, gd);
            r.cue_t = t;
            r.cued = true;
        }
    }
    let t = g.time;
    let r = job!(g, gd);
    r.watcher = Some(w);
    r.seen_t = t;
}

/// The watcher looked away less than a moment ago: he holds on a little yet, unless he was told to go anyway.
fn watcher_just_gone(g: &mut Game, gd: GuardId) -> bool {
    let t = g.time;
    let r = job!(g, gd);
    r.watcher.is_some() && !r.force && t - r.seen_t < 0.4
}

/// The watcher has gone: he says so if he had called him, and his clocks start over.
fn clear_to_move(g: &mut Game, me: &Take) {
    let gd = me.gd;
    job!(g, gd).watcher = None;
    if job!(g, gd).cued { crate::game::game_say(g, gd, "clear. moving.", None, None); }
    let t = g.time;
    let r = job!(g, gd);
    r.cued = false;
    r.back_said = Some(false);
    r.born = t;
    r.wait_t = -1.0;
    r.heel_t = -1.0;
    let n = format!("clear:{} on {}", g.gd[gd].callsign, me.name);
    pair_orders_note(g, &n);
    if v3::dist_xz(asg!(g, me.a).post.unwrap_or(me.here), me.here) < 0.05 { asg!(g, me.a).moved_t = t; asg!(g, me.a).mark_t = t; }
}

/// Sent, close enough, at the man's back or already on him, and nothing solid between them.
fn in_reach(g: &mut Game, me: &Take, d: f64, behind_him: bool) -> bool {
    let (p, fp) = (me.p, g.ch[me.fc].pos);
    job!(g, me.gd).go && d < g.k.pair.reach + (if me.on { 0.4 } else { 0.0 }) && (me.on || behind_him)
        && !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [fp[0], 1.0, fp[2]], None)
}

/// His hands go on and stay on, with a scuffle that can be heard, until the seconds it takes are up and the man goes down.
fn hands_on(g: &mut Game, me: &Take, d: f64) {
    let gd = me.gd;
    if !me.on {
        let t = g.time;
        job!(g, gd).t0 = t;
        crate::character::anim_strike(g, me.gc, None, None);
        crate::takedown::partner_begin(g, gd, me.foe, false);
        let n = format!("hands:{} on {}", g.gd[gd].callsign, me.name);
        pair_orders_note(g, &n);
    }
    if g.k.pair.scuffle > 0.0 && !job!(g, gd).scuffled {
        job!(g, gd).scuffled = true;
        let (pos, t, level, serial) = (v3::copy(g.ch[me.fc].pos), g.time, g.k.pair.scuffle, g.serial());
        g.events.push(crate::game::GameEvent {
            kind: crate::director::EventKind::Prop, pos, time: t, loud: false, level: Some(level), who: Some(0.0), serial, ..Default::default()
        });
    }
    let to = if d > 0.7 { pair_orders_behind(g, me.foe) } else { me.here };
    let go_pace = g.k.pair.go_pace;
    let c = g.ch[me.fc].pos;
    new_post(g, me.a, to, Some(v3::yaw_to(me.p, c)));
    let r = &mut asg!(g, me.a);
    r.arrived = false;
    r.pace = Some(go_pace);
    r.crouch = Some(false);
    if g.time - job!(g, gd).t0 >= g.k.pair.secs { takes_him_down(g, me); }
}

/// The man is down, by his hands; he comes back to Sam or holds where he is.
fn takes_him_down(g: &mut Game, me: &Take) {
    let gd = me.gd;
    crate::takedown::partner_fall(g, me.foe, false, Some(gd));
    let c = g.ch[me.fc].pos;
    let dir = v3::normalize([c[0] - me.p[0], 0.0, c[2] - me.p[2]]);
    crate::combat::kill_guard(g, me.foe, dir, Some(true), Some(crate::game::DownKind::Struck), Some(crate::game::DownBy::Guard), crate::game::With::Hand);
    let n = format!("down:{} by {}", me.name, g.gd[gd].callsign);
    pair_orders_note(g, &n);
    if !take_rejoin(g, gd, me.was_following) {
        crate::game::game_say(g, gd, "he's down.", None, None);
        pair_orders_stand(g, gd, "holding", None);
    }
}

/// No way to his back: he waits where he is, says why on the row, and after a while gives up.
fn waits_for_a_way(g: &mut Game, me: &Take, way: &crate::pairorders::PairOrdersStalk) {
    let (gd, foe) = (me.gd, me.foe);
    let t = g.time;
    if job!(g, gd).wait_t < 0.0 { job!(g, gd).wait_t = t; }
    job!(g, gd).set = false;
    let wait_t = job!(g, gd).wait_t;
    let force = job!(g, gd).force;
    let face = if t - wait_t < g.k.pair.steady_face_secs { None } else { Some(v3::yaw_to(me.p, g.ch[me.fc].pos)) };
    new_post(g, me.a, me.here, face);
    let words = if !way.exposed || force { format!("waiting · no room at {}'s back", me.name) }
        else { match way.by {
            None => format!("waiting · no way to {} unseen", me.name),
            Some(b) if b == foe => format!("waiting · {} is facing this way", me.name),
            Some(b) => format!("waiting · {} is looking across the way to {}", name_of(g, b), me.name),
        } };
    set_words(g, me.a, words);
    if g.time - wait_t > g.k.pair.stalk_wait {
        let line = if !way.exposed || force { "no room behind him." } else { "can't get behind him unseen." };
        take_off(g, me, "noway", Some(line), false);
    }
}

/// Waiting on the mark: on his spot he is set and says so, once (true: he stops there); short of it he moves up.
fn set_on_the_stage(g: &mut Game, me: &Take, spot: V3) -> bool {
    let gd = me.gd;
    if v3::dist_xz(me.p, spot) >= 0.45 {
        job!(g, gd).set = false;
        set_words(g, me.a, format!("moving up on {}", me.name));
        return false;
    }
    job!(g, gd).set = true;
    park_on(g, me);
    set_words(g, me.a, format!("set on {} · press F to give the mark", me.name));
    if !job!(g, gd).set_said {
        job!(g, gd).set_said = true;
        crate::game::game_say(g, gd, "set.", None, None);
        let n = format!("set:{} on {}", g.gd[gd].callsign, me.name);
        pair_orders_note(g, &n);
    }
    true
}

/// The last yards, inside his near field, when nobody gave the moment: on the heels of a glance, or after `heel_wait` of none, anyway. True: he waits this frame.
fn waits_on_a_glance(g: &mut Game, me: &Take, d: f64) -> bool {
    let gd = me.gd;
    set_words(g, me.a, format!("on {}", me.name));
    let gl = g.pair.glance.get(me.foe).copied();
    let t = g.time;
    if let Some(gl) = gl {
        if job!(g, gd).mode == crate::pairorders::JobMode::Now && !job!(g, gd).going && d > g.k.pair.reach && d < g.k.pair.heel_ring
            && (gl.on || t - gl.end_t > g.k.pair.heel_fresh)
        {
            if job!(g, gd).heel_t < 0.0 { job!(g, gd).heel_t = t; }
            if t - job!(g, gd).heel_t < g.k.pair.heel_wait {
                park_on(g, me);
                return true;
            }
        }
    }
    if d < g.k.pair.heel_ring { job!(g, gd).going = true; }
    false
}

/// On his way to the spot, crouched, by the way the stalk found.
fn walks_on(g: &mut Game, me: &Take, spot: V3, way: &crate::pairorders::PairOrdersStalk) {
    if let Some(post) = asg!(g, me.a).post {
        if v3::dist_xz(post, me.here) < 0.05 { let t = g.time; asg!(g, me.a).moved_t = t; asg!(g, me.a).mark_t = t; }
    }
    let pace = if job!(g, me.gd).going { g.k.pair.go_pace } else { g.k.pair.pace };
    let r = &mut asg!(g, me.a);
    r.pace = Some(pace);
    r.crouch = Some(true);
    r.face = None;
    r.post = Some(spot); if r.had == Some(spot) { r.had = None; }
    r.arrived = false;
    r.via.clear();
    let n = way.path.len();
    if n > 0 { for q in way.path[..n - 1].iter() { r.via.push(*q); } }
    r.goal = v3::copy(r.via.first().copied().unwrap_or(spot));
    // (a fresh way is a new leg even to the same corner, so his path is dropped and found again from where he stands. Without this a way searched afresh to the same corner left
    // him on the path of the old one.)
    r.new_leg = true;
}

/// the sweep's ring: is this floor watched by a standing man whose look swings (rows: x, z, hx, hz, half)
fn ring_at(g: &Game, ring: &[[f64; 5]], x: f64, z: f64) -> bool {
    for r in ring.iter() {
        let dx = x - r[0];
        let dz = z - r[1];
        let d = m::hyp2(dx, dz);
        if d > g.k.pair.ring_m || d < 0.05 { continue; }
        if m::acos(m::max(-1.0, m::min(1.0, (dx * r[2] + dz * r[3]) / d))) > r[4] { continue; }
        if !crate::scene_collision::segment_blocked(g, [r[0], 1.6, r[1]], [x, 1.0, z], None, None, None) { return true; }
    }
    false
}

/// the length of a walk
fn len_of(pts: &[V3]) -> f64 {
    let mut d = 0.0;
    let mut i = 1;
    while i < pts.len() { d += v3::dist_xz(pts[i - 1], pts[i]); i += 1; }
    d
}

/// `from` and then the way
fn from_on(from: V3, way: &[V3]) -> Vec<V3> {
    let mut v = Vec::with_capacity(way.len() + 1);
    v.push(from);
    v.extend_from_slice(way);
    v
}

/// THE WAY TO A MAN'S BACK: A* with the floor the sneak's eye reads as watched priced all but shut, and read whole by the same eye: `exposed` is the one answer to "is this way seen"   (PairOrders.stalk)
fn pair_orders_stalk(g: &mut Game, gd: GuardId, to: V3, pace: Option<f64>) -> crate::pairorders::PairOrdersStalk {
    let pace = pace.unwrap_or(g.k.pair.pace);
    let c = g.col.nav.cell;
    let mut habit = |g: &mut Game, h: GuardId| -> Option<[f64; 2]> {
        let hb = *g.pair.habit.get(h)?;
        let l = m::hyp2(hb[0], hb[1]);
        if l > 0.3 { Some([hb[0] / l, hb[1] / l]) } else { None }
    };
    let mut w = crate::sneak::sneak_walk(g, gd, true, Some(&mut habit));
    let from = crate::v3::on_floor(g.pos_of(gd));
    let mut ring: Vec<[f64; 5]> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let e = g.guards[i];
        i += 1;
        if e == gd || crate::condition::down(g, g.gd[e].ch) || g.gd[e].held.is_some() || !g.team(e).theirs() { continue; }
        let sw = g.pair.swept.get(e).copied().unwrap_or(0.0);
        let hb = g.pair.habit.get(e).copied();
        let l = match hb { Some(v) => m::hyp2(v[0], v[1]), None => 0.0 };
        let hb = match hb { Some(v) if sw >= 0.1 && l >= 0.3 => v, _ => continue };
        let ep = g.pos_of(e);
        ring.push([ep[0], ep[2], hb[0] / l, hb[1] / l, m::min(m::PI * 155.0 / 180.0, m::PI * 104.0 / 180.0 + sw)]);
    }
    let budget = g.k.sneak.budget;
    let was = g.pair.ways.get(gd).cloned();
    let mut rest: Option<Vec<V3>> = None;
    if let Some(was) = was.as_ref() {
        if g.time >= was.t && v3::dist_xz(was.to, to) < 0.6 {
            let n = was.path.len();
            let mut i = 0usize;
            let mut bd = m::INF;
            let mut k = 0;
            while k < n { let d = v3::dist_xz(from, was.path[k]); if d < bd { bd = d; i = k; } k += 1; }
            if bd < 0.9 && i + 1 < n { i += 1; }
            let mut past = false;
            if !(bd < 0.9) && i + 1 < n {
                let (a, b) = (was.path[i], was.path[i + 1]);
                past = (from[0] - a[0]) * (b[0] - a[0]) + (from[2] - a[2]) * (b[2] - a[2]) > 0.0;
            }
            let tries: &[usize] = if past { &[i + 1, i] } else { &[i] };
            for k in tries.iter().copied() {
                let mut r: Vec<V3> = if k <= n { was.path[k..].to_vec() } else { Vec::new() };
                if !r.is_empty() { let last = r.len() - 1; r[last] = crate::v3::on_floor(to); }
                if r.is_empty() { continue; }
                let first = r[0];
                if !crate::scene_collision::nav_grid_walkable(g, from, first) { continue; }
                if r.len() >= 2 && !crate::scene_collision::nav_grid_walkable(g, r[r.len() - 2], r[r.len() - 1]) { continue; }
                let pts = from_on(from, &r);
                if crate::sneak::sneak_walk_cost(g, &mut w, &pts, pace, Some(to), None) <= budget {
                    if g.time - was.t < 6.0 {
                        return crate::pairorders::PairOrdersStalk { path: r, exposed: false, by: None };
                    }
                    rest = Some(r);
                    break;
                }
            }
        }
    }
    let mut path = {
        let mut pen = |g: &mut &mut Game, ix: f64, iz: f64| -> f64 {
            let (x, z) = ((ix + 0.5) * c, (iz + 0.5) * c);
            if crate::sneak::sneak_walk_watched(g, &mut w, x, z, Some(true)) || ring_at(g, &ring, x, z) { 14.0 } else { 1.0 }
        };
        crate::scene_collision::way_priced(&mut *g, from, to, 6000.0, &mut pen)
    };
    if path.is_none() { path = crate::scene_collision::nav_grid_find_path(g, from, to, None, None); }
    if let Some(rest) = rest {
        let ok = match path.as_ref() {
            Some(pa) => { let pts = from_on(from, pa); crate::sneak::sneak_walk_cost(g, &mut w, &pts, pace, Some(to), None) <= budget }
            None => false,
        };
        // (`ok` false with no path: `||` stops there and never reads path!)
        let keep_rest = !ok || len_of(&from_on(from, path.as_ref().unwrap())) > 0.8 * len_of(&from_on(from, &rest));
        if keep_rest {
            let t = g.time;
            g.pair.ways.set(gd, crate::pairorders::PairOrdersWaysValue { to: crate::v3::on_floor(to), path: rest.clone(), t });
            return crate::pairorders::PairOrdersStalk { path: rest, exposed: false, by: None };
        }
    }
    let path = match path {
        Some(p) => p,
        None => { g.pair.ways.delete(gd); return crate::pairorders::PairOrdersStalk { path: Vec::new(), exposed: true, by: None }; }
    };
    let pts = from_on(from, &path);
    let exposed = crate::sneak::sneak_walk_cost(g, &mut w, &pts, pace, Some(to), None) > budget;
    if exposed { g.pair.ways.delete(gd); } else { let t = g.time; g.pair.ways.set(gd, crate::pairorders::PairOrdersWaysValue { to: crate::v3::on_floor(to), path: path.clone(), t }); }
    return crate::pairorders::PairOrdersStalk { path, exposed, by: w.by };
}

// ------------------------------------------------------------ holding in the dark
/// how lit a spot of floor is on the light meter's scale: the place's steady lamps, direct only   (PairOrders.litAt)
fn pair_orders_lit_at(g: &mut Game, p: V3) -> f64 {
    let at = [p[0], g.k.pair.dark_h, p[2]];
    let mut any = |_g: &Game, _o: f64| -> bool { true };
    let e = crate::guards::light_estimate_at(g, at, Some(&mut any), Some(true), None);
    crate::game::game_visibility_of(g, e)
}

/// the search behind darkSpot
fn dark_find(g: &mut Game, at: V3) -> Option<V3> {
    if pair_orders_lit_at(g, at) <= g.k.pair.dark_vis { return Some(crate::v3::on_floor(at)); }
    let mut r = 0.5;
    while r <= g.k.pair.dark_m + 1e-6 {
        let mut best: Option<V3> = None;
        let mut bv = g.k.pair.dark_vis;
        let mut k = 0;
        while k < 8 {
            let a = (k as f64) * m::PI / 4.0;
            k += 1;
            let q: V3 = [at[0] + m::sin(a) * r, 0.0, at[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || crate::scene_collision::low_blocked(g, at, q) { continue; }
            let v = pair_orders_lit_at(g, q);
            if v < bv || (v <= bv && best.is_none()) { bv = v; best = Some(q); }
        }
        if best.is_some() { return best; }
        r += 0.5;
    }
    None
}

/// the dark floor an order for `at` means; None: none is dark   (PairOrders.darkSpot)
pub(super) fn pair_orders_dark_spot(g: &mut Game, at: V3) -> Option<V3> {
    let key = format!("{},{}", js::num_str(m::round(at[0] * 4.0)), js::num_str(m::round(at[2] * 4.0)));
    let t = g.time;
    if let Some(was) = g.pair.dark_memo.get(key.clone()) {
        if m::abs(t - was.t) < 1.0 { return was.spot; }
    }
    if g.pair.dark_memo.len() > 64 { g.pair.dark_memo.clear(); }
    let spot = dark_find(g, at);
    g.pair.dark_memo.set(key, crate::pairorders::PairOrdersDarkMemoValue { t, spot });
    spot
}

/// on his dark spot: the place read once a second, and said when it is no longer dark   (PairOrders.darkTick)
pub fn pair_orders_dark_tick(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) {
    if g.pair.dark.get(gd).is_none() { g.pair.dark.set(gd, crate::pairorders::PairOrdersDarkValue { check_t: -1.0, lit_t: -1.0, said_t: -1e9 }); }
    let t = g.time;
    if !asg!(g, a).arrived || t - g.pair.dark.get(gd).unwrap().check_t < 1.0 { return; }
    g.pair.dark.get_mut(gd).unwrap().check_t = t;
    let p = g.pos_of(gd);
    if pair_orders_lit_at(g, p) <= g.k.pair.dark_vis + 0.1 {
        g.pair.dark.get_mut(gd).unwrap().lit_t = -1.0;
        asg!(g, a).there = "holding in the dark".into();
        return;
    }
    if g.pair.dark.get(gd).unwrap().lit_t < 0.0 { g.pair.dark.get_mut(gd).unwrap().lit_t = t; }
    asg!(g, a).there = "holding · LIT".into();
    let d = *g.pair.dark.get(gd).unwrap();
    if t - d.lit_t >= g.k.pair.lit_secs && t - d.said_t >= g.k.pair.lit_every {
        g.pair.dark.get_mut(gd).unwrap().said_t = t;
        let n = format!("lit:{}", g.gd[gd].callsign);
        pair_orders_note(g, &n);
        crate::game::game_say(g, gd, "lit up here.", None, None);
    }
}

/// A teammate on a take job takes no man as a target who has not noticed anybody, nor the man his hands are on.
pub fn pair_passes(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool {
    if !g.k.pair.on { return false; }
    let j = pair_orders_job_of(g, gd);
    let j = match j { Some(j) => j, None => return false };
    if crate::silent::urgent(g) { return false; }
    if j.foe == h && j.t0 >= 0.0 { return true; }
    if pair_orders_finishing(g, gd) { return true; }
    !pair_orders_noticed(g, h)
}

/// A man in the partner's hands finds no body.
pub fn pair_grip_body(g: &mut crate::game::Game, h: GuardId) -> bool {
    pair_gripped(g, h)
}

/// The partner's hands (or the spotted rule's, while pair_up is on) are on this man: he sees nobody until they are off.
pub fn pair_gripped(g: &mut crate::game::Game, h: GuardId) -> bool {
    (g.k.pair.on && pair_orders_gripped(g, h))
        || (crate::pairup::pair_up(g) && crate::spotted::spotted_gripped(g, h))
}

/// One told to WATCH a man fires no round while that man has not noticed.
pub fn pair_holds_round(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool {
    if !g.k.pair.on { return false; }
    let a = match crate::orders::squad_orders_of(&crate::seats::here(g), gd) { Some(a) => a, None => return false };
    if g.tab.orders_assignment[a].order != crate::orders::rows::pairwatch::ORDER.id || g.tab.orders_assignment[a].foe != Some(h) || pair_orders_noticed(g, h) || crate::silent::urgent(g) {
        return false;
    }
    true
}
