//! THE WALKS: every teammate is going somewhere by the squad's own hand-over, sent when it changes and again whenever he is found with no such order; the card switch, in which the same men walk; and home.
use super::acts::{actor, ask_veil, row, Leaf};
use super::light::{KIT_VIEW_LOOP as K, KIT_VIEW_QUIET as QK};
use super::types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

pub(super) fn flat(a: V3, p: V2) -> f64 { m::hyp2(a[0] - p[0], a[2] - p[1]) }
pub(super) fn free(g: &Game, p: V2) -> bool { !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[1]) }
pub(super) fn v2(p: V3) -> V2 { [p[0], p[2]] }
pub(super) fn v3(p: V2) -> V3 { [p[0], 0.0, p[1]] }
/// a point's place by the door: along the wall toward the stack, out from its line on the team's side
pub(super) fn along(st: &KitStage, p: V2) -> f64 { (p[0] - st.foot[0]) * st.s[0] + (p[1] - st.foot[2]) * st.s[1] }
pub(super) fn out_of(st: &KitStage, p: V2) -> f64 { (p[0] - st.foot[0]) * st.n[0] + (p[1] - st.foot[2]) * st.n[1] }
/// a point by [along the wall toward the stack, out on the team's side, up] from the anchor's middle
pub(super) fn at(st: &KitStage, o: V3) -> V3 { [st.mid[0] + st.s[0] * o[0] + st.n[0] * o[1], o[2], st.mid[1] + st.s[1] * o[0] + st.n[1] * o[1]] }

/// HOW A MAN IS TURNED, wherever he is put: to the middle of the way in, which is what he covers. One rule for every man of every way
pub(super) fn faces(st: &KitStage, p: V2) -> f64 { m::atan2(st.foot[0] - p[0], st.foot[2] - p[1]) }
/// where a man lives between rows and how he is turned there: his place; the fourth man's, while his card is open, a place at Sam's jamb; the quiet picture's partner, behind Sam, until his card is open
pub fn home_of(g: &Game, st: &KitStage, x: GuardId) -> (V2, f64) {
    let k = st.order.iter().position(|&o| o == x);
    if k == Some(0) { if let Some(p) = st.mate { if actor(g, st) != Some(x) { return (p, faces(st, p)); } } }
    let c = g.gd[x].ch;
    let spot = match k { Some(3) if st.four => Some(st.act4), Some(k) => st.spots.get(k).copied(), None => None };
    match spot { Some(p) => (p, faces(st, p)), None => (v2(g.ch[c].pos), g.ch[c].body_yaw) }
}
/// the quiet picture: the partner rests on a knee behind Sam
pub(super) fn low_at(g: &Game, st: &KitStage, x: GuardId) -> bool { st.quiet && st.mate.is_some() && st.order.first() == Some(&x) && actor(g, st) != Some(x) }
pub(super) fn off_home(g: &Game, st: &KitStage, x: GuardId) -> f64 { flat(g.pos_of(x), home_of(g, st, x).0) }
pub(super) fn away(g: &Game, st: &KitStage) -> bool { st.order.iter().any(|&x| off_home(g, st, x) > K.home_m) }

/// a teammate to a mark, at a walk, turned to `face`, his gun on `watch` or at the low ready: the squad's own hand-over, which takes him off whatever had him. `strafe`: his chest kept on `face` all the way
pub(super) fn send(g: &mut Game, x: GuardId, p: V2, face: f64, watch: Option<V3>, pace: f64, strafe: bool, crouch: bool) {
    let mut a = crate::orders::SquadOrdersAssignA::default();
    a.order = String::from("cover");
    a.post = Some([p[0], 0.0, p[1]]);
    a.face = Some(face);
    a.strafe = Some(strafe);
    a.watch = watch;
    a.stance = Some(if watch.is_some() { crate::character::Stance::None } else { crate::character::Stance::LowReady });
    a.upper = Some(if watch.is_some() { crate::character::UpperMode::Aim } else { crate::character::UpperMode::None });
    a.pace = Some(pace);
    a.crouch = Some(crouch);
    crate::orders::squad_orders_assign(g, x, &a);
}
fn assignment(g: &Game, x: GuardId) -> Option<OrdersAssignmentId> { crate::orders::squad_orders_of(&crate::seats::here(g), x) }
fn is_cover(g: &Game, x: GuardId) -> bool { assignment(g, x).map_or(false, |a| g.tab.orders_assignment[a].order == "cover") }

pub(super) fn go(g: &Game, st: &mut KitStage, x: GuardId, pts: Vec<V2>, pace: f64, delay: f64, lane: bool) {
    let w = Walk { man: x, pts, k: 0, pace, at: g.time + delay, lane, sent: None, watch: None, t0: g.time, face: None, dealt: false, low: false };
    match st.a.walks.iter().position(|w| w.man == x) { Some(i) => st.a.walks[i] = w, None => st.a.walks.push(w) }
}
pub(super) fn walk_of<'a>(st: &'a mut KitStage, x: GuardId) -> Option<&'a mut Walk> { st.a.walks.iter_mut().find(|w| w.man == x) }
/// down on a knee, or up: the squad's own word on his order, kept on his walk so that an order sent again has it
pub(super) fn low(g: &mut Game, st: &mut KitStage, x: GuardId, on: bool) {
    if let Some(w) = walk_of(st, x) { w.low = on; }
    if let Some(a) = assignment(g, x) { let r = &mut g.tab.orders_assignment[a]; if r.order == "cover" { r.crouch = Some(on); } }
}
/// a man the running row has not placed has one walk: to his place, on his feet, whatever dealt him another or took his order
fn placed(st: &KitStage, x: GuardId) -> bool {
    match st.a.beat.as_ref() { Some(b) => b.man == Some(x) || b.moved.contains(&x), None => st.a.apart && st.a.home_men.contains(&x) }
}
fn homeward(g: &mut Game, st: &mut KitStage, x: GuardId) {
    if placed(st, x) { return; }
    let h = home_of(g, st, x).0;
    let last = st.a.walks.iter().find(|w| w.man == x).and_then(|w| w.pts.last().copied().map(|l| (l, w.low)));
    match last {
        Some((l, was)) if m::hyp2(l[0] - h[0], l[1] - h[1]) <= 1e-3 => {
            let want = low_at(g, st, x);
            let kneeling = st.a.beat.as_ref().map_or(false, |b| st.a.kneel == Some(b.n));
            if was != want && !kneeling { low(g, st, x, want); }
        }
        _ => go(g, st, x, vec![h], K.pace, 0.0, false),
    }
}
/// each tick: every man's walk, the order sent again where it is not his
pub(super) fn walks(g: &mut Game, st: &mut KitStage) {
    let (team, loose, man, none_door) = match st.a.beat.as_ref() { Some(b) => { let r = row(b.row); (r.team, b.loose, b.man, r.door == Leaf::None) } None => (false, false, None, false) };
    if team && !loose { return; }   // (the squad's own order has every man: the wall charge)
    let now = g.time;
    for i in 0..st.order.len() {
        let x = st.order[i];
        if man == Some(x) && none_door { continue; }   // (the door tools' own order has him)
        homeward(g, st, x);
        let Some(mut w) = st.a.walks.iter().find(|w| w.man == x).cloned() else { continue };
        let c = g.gd[x].ch;
        let p = g.ch[c].pos;
        if now < w.at {
            // (a walk not yet due: he stands for it)
            if !is_cover(g, x) { let y = g.ch[c].body_yaw; send(g, x, v2(p), y, None, K.pace, true, false); }
            continue;
        }
        if w.k + 1 < w.pts.len() && flat(p, w.pts[w.k]) <= K.way_m { w.k += 1; w.sent = None; }
        let q = w.pts[w.k];
        if w.lane && w.k + 1 == w.pts.len() && flat(p, q) <= K.on_slot { w.lane = false; }
        if !is_cover(g, x) || w.sent != Some(q) {
            let face = w.face.unwrap_or(home_of(g, st, x).1);
            send(g, x, q, face, w.watch, w.pace, !w.lane, w.low);
            w.sent = Some(q);
        }
        *walk_of(st, x).unwrap() = w;
    }
}
/// the open card's man is still on his way to the head of the file. Sam's card: no row of his begins while a man's card-switch walk has yet to arrive. `swap_secs` at most.
pub(super) fn arriving(g: &Game, st: &KitStage) -> bool {
    let now = g.time;
    if let Some(x) = actor(g, st) { return st.a.walks.iter().find(|w| w.man == x).map_or(false, |w| w.lane && now - w.t0 < K.swap_secs); }
    st.order.iter().any(|&x| st.a.walks.iter().find(|w| w.man == x).map_or(false, |w| {
        w.dealt && now - w.t0 < K.swap_secs && (w.k + 1 < w.pts.len() || flat(g.pos_of(x), *w.pts.last().unwrap()) > K.on_slot + 0.1)
    }))
}
/// a hand more than two bodies, so that nobody is shouldered
const ROOM: f64 = 0.2;
/// THE CARD SWITCH: no restage, the SAME men walk. The man whose card opened steps out a lane, walks up and cuts in at the head place; the men ahead of his old place each step back one, rearmost first. Sam never moves;
/// the fourth man (behind Sam, across the door) does not cross it: he steps round Sam's yard side to `act4` and back when his card shuts.
pub(super) fn rotate(g: &Game, st: &mut KitStage, who: Option<GuardId>) {
    let (s, n) = (st.s, st.n);
    let pp = |p: V2, a: f64, o: f64| -> V2 { [p[0] + s[0] * a + n[0] * o, p[1] + s[1] * a + n[1] * o] };
    if let Some(&x) = st.order.get(3) {
        let act = who == Some(x);
        if st.four != act {
            st.four = act;
            // (round Sam on the yard side, and past him on the door's: a straight line from the yard to `act4` runs through him)
            let round = pp(st.sam, 0.0, K.round + ROOM);
            let past = pp(st.sam, m::sign(along(st, st.act4) - along(st, st.sam)) * (K.round + ROOM), 0.0);
            let mut pts = Vec::new();
            if act { if free(g, round) { pts.push(round); if free(g, past) { pts.push(past); } } pts.push(st.act4); }
            else { if free(g, round) { if free(g, past) { pts.push(past); } pts.push(round); } pts.push(st.spots[3]); }
            go(g, st, x, pts, K.pace, 0.0, act);
            walk_of(st, x).unwrap().dealt = true;
        }
    }
    let ns = m::min(3.0, st.order.len() as f64) as usize;
    let Some(who) = who else { return };
    let Some(j) = st.order.iter().position(|&o| o == who) else { return };
    if j == 0 || j >= ns { return; }
    let was = st.order.clone();
    let mut stack: Vec<GuardId> = was[..ns].to_vec();
    stack.remove(j);
    stack.insert(0, who);
    stack.extend_from_slice(&was[ns..]);
    st.order = stack;
    let lane_out = out_of(st, st.spots[0]) + K.lane;
    let mut rank = 0.0;
    for k in (0..ns).rev() {
        let x = st.order[k];
        if was[k] == x { continue; }
        let t = st.spots[k];
        let p = v2(g.pos_of(x));
        if x == who {
            let a = pp(p, -K.cut_in, lane_out - out_of(st, p));
            let b = pp(t, K.cut_in, K.lane);
            let pts = if free(g, a) && free(g, b) && along(st, a) > along(st, b) { vec![a, b, t] } else if free(g, b) { vec![b, t] } else { vec![t] };
            go(g, st, x, pts, K.swap_pace, 0.0, true);
        } else {
            let b = pp(t, 0.0, K.lane);
            let pts = if out_of(st, p) > out_of(st, t) + 0.3 && free(g, b) { vec![b, t] } else { vec![t] };
            go(g, st, x, pts, K.pace, K.shuffle[0] + K.shuffle[1] * rank, false);
            rank += 1.0;
        }
        walk_of(st, x).unwrap().dealt = true;
    }
}
/// Sam walked to a mark, nothing else asked of him
pub(super) fn sam_goes(st: &mut KitStage, p: V2) { st.a.led = Some(Led { to: Some(p), ..Led::default() }); }
pub(super) fn sam_goes_low(st: &mut KitStage, p: V2, low: bool) { st.a.led = Some(Led { to: Some(p), low, ..Led::default() }); }
/// Sam's quiet rows are played from his place: a row cut with him off it left him there for the whole of the next
pub(super) fn sam_stand(g: &Game, st: &mut KitStage) {
    let c = g.pl[g.player].ch;
    if flat(g.ch[c].pos, st.sam) > K.on_slot { let home = st.sam; sam_goes(st, home); } else { st.a.led = Some(Led::default()); }
}
/// HIS KEYS AND HIS HAND FOR THIS TICK, as anybody's who plays a seat: the walk keys toward the place, in the view's own directions, the cursor on the point, and down once
pub(super) fn sam_hand(g: &mut Game, st: &mut KitStage, cmd: &mut crate::cmd::PlayerCmd) {
    g.hand_at = st.a.led.and_then(|l| l.at);
    let Some(led) = st.a.led else { return };
    let pl = g.player;
    if led.low && !g.pl[pl].crouch { st.a.press.push("KeyC"); }
    let Some(to) = led.to else { return };
    let p = g.player_pos(pl);
    if flat(p, to) <= 0.1 { return; }
    for (on, key) in crate::player::walk_keys(p, to, cmd.view.forward).into_iter().zip(crate::player::WALK_KEYS) { if on { cmd.keys.push(alloc::string::String::from(key)); } }
}
/// the half step a man takes toward the front of the door for a throw or his light: from where he lives, `metres` at most toward the old post
pub(super) fn step_out(st: &KitStage, b: &Beat, metres: f64) -> V2 {
    let g0 = at(st, [K.post[0] * st.latch, K.post[1], 0.0]);
    let (dx, dz) = (g0[0] - b.home[0], g0[2] - b.home[1]);
    let l = m::hyp2(dx, dz);
    let l = if l != 0.0 { l } else { 1.0 };
    let k = m::min(metres, l) / l;
    [b.home[0] + dx * k, b.home[1] + dz * k]
}
/// AT A WALL LEFT OPEN a restage would stand the wall, so nobody is put back: the row after the charge waits for the men and Sam on their own legs, `swap_secs + home_secs` at most, and from then on a row waits only for the men the row before it moved, as long at most. A man whom a row's walk does not get home holds nothing again. True: the row waits.
fn homing_at_hole(g: &Game, st: &mut KitStage) -> bool {
    let now = g.time;
    let (on, act, all) = (K.on_slot + 0.1, actor(g, st), st.a.hole_all);
    let mut men = core::mem::take(&mut st.a.hole_men);
    men.retain(|&x| off_home(g, st, x) > on);
    let waiting = men.iter().any(|&x| all || act != Some(x)) || (all && flat(g.player_pos(g.player), st.sam) > on);
    if waiting && st.a.home_t.is_nan() { st.a.home_t = now; }
    if !waiting || now - st.a.home_t >= K.swap_secs + K.home_secs { st.a.hole_all = false; st.a.home_t = f64::NAN; return false; }
    st.a.hole_men = men;
    true
}
/// HOME: no row begins with a man far from his place, and a row of Sam's waits for the men on their own legs, `home_secs` at most: then, and at once for a man `far_m` off, the room is staged again under the dissolve, and the row that waited begins at once. A man found off his place again after that is not staged again until he has been seen home. True: the row waits.
pub(super) fn homing(g: &Game, st: &mut KitStage) -> bool {
    let now = g.time;
    if st.hole { return homing_at_hole(g, st); }
    if st.a.seated { st.a.seated = false; st.a.home_t = f64::NAN; return false; }
    if !st.a.seat_tried && st.order.iter().any(|&x| !st.a.walks.iter().find(|w| w.man == x).map_or(false, |w| w.lane) && off_home(g, st, x) > K.far_m) {
        st.a.home_t = f64::NAN;
        seat(g, st);
        return true;
    }
    let is_away = away(g, st);
    if !is_away { st.a.seat_tried = false; }
    if actor(g, st).is_some() || !is_away { st.a.home_t = f64::NAN; return false; }
    if st.a.home_t.is_nan() { st.a.home_t = now; }
    for i in 0..st.order.len() {
        let x = st.order[i];
        let off = off_home(g, st, x);
        if let Some(w) = walk_of(st, x) {
            if !w.lane && off > K.home_m && (w.pace < K.swap_pace || w.at > now) { w.pace = K.swap_pace; w.at = m::min(w.at, now); w.sent = None; }
        }
    }
    if now - st.a.home_t >= K.home_secs {
        st.a.home_t = f64::NAN;
        if st.a.seat_tried { return false; }
        seat(g, st);
    }
    true
}
/// every man put back on his place under the dissolve, for the row that waited
fn seat(g: &Game, st: &mut KitStage) { st.a.seat_tried = true; st.a.seated = true; ask_veil(g, st); }
/// Where the leaf rests is its own business: the quiet picture rests it ajar, eased back there by the host's own hand after a row moved it
pub(super) fn ease_leaf(g: &mut Game, st: &KitStage, d: DoorId) {
    let a0 = g.dr[d].leaf.angle;
    let da = st.leaf_rest - a0;
    let step = QK.leaf_rate * st.a.step_dt;
    g.dr[d].leaf.latched = false;
    g.dr[d].leaf.closing = false;
    g.dr[d].leaf.vel = 0.0;
    g.dr[d].leaf.angle = if m::abs(da) <= step { st.leaf_rest } else { a0 + m::sign(da) * step };
}
