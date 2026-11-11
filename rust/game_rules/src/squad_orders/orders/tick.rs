use crate::v3::dist_xz;
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// His order's own tick, if it has one: it may rewrite his Task or re-assign him.
fn order_tick(g: &mut crate::game::Game, d: OrdersOrderDefId, gd: GuardId, a: TaskId) {
    if let Some(f) = g.tab.orders_order_def[d].row.tick { f(g, gd, a); }
}

// ------------------------------------------------------------ the men

/// Once a frame before the guards move: each man under an order is walked to it and posed on it.
pub fn squad_orders_tick(g: &mut crate::game::Game) {
    squad_orders_sync(g);
    crate::fireline::fire_line_tick(g);
    if g.sets[g.seat].orders.asg.is_empty() { return; }
    if !squad_orders_active(g, &crate::seats::here(g)) { squad_orders_clear(g, None); return; }
    crate::sneak::sneak_tick(g);
    let mates: Vec<GuardId> = if squad_orders_big(g, &crate::seats::here(g)) { squad_orders_men(g, &crate::seats::here(g), None, None) } else { Vec::new() };
    let followers = the_file(g);
    note_sams_walk(g, &followers);
    // a live walk of the map: a man dropped here, or re-assigned by his order's own tick, leaves it or stays in it as he should
    let mut at = 0usize;
    while let Some(gd) = g.sets[g.seat].orders.asg.next_key(&mut at) {
        let a = *g.sets[g.seat].orders.asg.get(gd).unwrap();
        man_on_his_order(g, gd, a, &mates, &followers);
    }
    say_set(g);
}

/// The men falling in, in file order and numbered from the front.
fn the_file(g: &mut Game) -> Vec<GuardId> {
    let mut file: Vec<(GuardId, TaskId)> = Vec::new();
    for (gd, &a) in g.sets[g.seat].orders.asg.iter() { if g.tab.task[a].post.is_none() { file.push((gd, a)); } }
    js::sort_by(&mut file, |x, y| g.tab.task[x.1].follow - g.tab.task[y.1].follow);
    for (i, x) in file.iter().enumerate() { g.tab.task[x.1].follow = i as f64; }
    file.iter().map(|x| x.0).collect()
}

/// Sam's trail, pace and heading, while anybody follows him; nobody following, they are forgotten.
fn note_sams_walk(g: &mut Game, followers: &[GuardId]) {
    if followers.is_empty() {
        g.sets[g.seat].orders.trail.clear(); g.sets[g.seat].orders.sam_t = -1.0; g.sets[g.seat].orders.sam_v = 0.0;
        return;
    }
    let sp = g.player_pos(g.player);
    squad_orders_crumb(g, sp, followers);
    let (sam_t, sam_p) = (g.sets[g.seat].orders.sam_t, g.sets[g.seat].orders.sam_p);
    let step = if sam_t >= 0.0 { dist_xz(sp, sam_p) } else { 0.0 };
    let el = g.time - sam_t;
    let own = if sam_t >= 0.0 { crate::noshove::no_shove_own(g, sp[0] - sam_p[0], sp[2] - sam_p[2]) } else { None };
    if sam_t >= 0.0 && el > 1e-6 && step < 1.0 {   // (a jump of a metre is a teleport, not a pace)
        let ox = match own { Some(o) => o[0], None => sp[0] - sam_p[0] };
        let oz = match own { Some(o) => o[1], None => sp[2] - sam_p[2] };
        let st = if own.is_some() { m::hyp2(ox, oz) } else { step };
        g.sets[g.seat].orders.sam_v += (st / el - g.sets[g.seat].orders.sam_v) * 0.25;
        if st > 1e-3 { g.sets[g.seat].orders.sam_dir[0] = ox / st; g.sets[g.seat].orders.sam_dir[2] = oz / st; }
    }
    g.sets[g.seat].orders.sam_p[0] = sp[0]; g.sets[g.seat].orders.sam_p[2] = sp[2]; g.sets[g.seat].orders.sam_t = g.time;
}

/// One man's frame: down or taken by a drill he loses his order; the fight may have him; else his order's own tick, his feet, how he carries himself, and all of it told to him.
fn man_on_his_order(g: &mut Game, gd: GuardId, a: TaskId, mates: &[GuardId], followers: &[GuardId]) {
    if crate::condition::down(g, g.gd[gd].ch) { squad_orders_drop(g, gd); return; }
    if crate::mission::has(g, gd) { squad_orders_drop(g, gd); return; }   // a drill has taken him: its order stands, ours is dropped
    through_his_door(g, gd, a);
    if fight_has_him(g, gd, a) { return; }
    if g.tab.task[a].yield_t >= 0.0 { back_on_his_order(g, gd, a); }
    if g.sets[g.seat].orders.ours.delete(gd) { crate::duty::pin(g, gd, false, "orders"); }
    if order_sent_him_elsewhere(g, gd, a) { return; }   // (his new order's turn is next frame)
    let order_id = g.tab.task[a].order.clone();   // (a row's tick may rewrite his Task)
    let feet = if g.tab.task[a].post.is_none() { falls_in(g, gd, a, followers) } else { walks_to_his_mark(g, gd, a, &order_id) };
    let mut look = order_carriage(g, a, &feet);
    unasked_cover(g, gd, a, &feet, &mut look);
    let sp = crate::spotted::spotted_feet(g, gd, !look.posted, feet.stepping);
    if let Some(w) = sp.as_ref().and_then(|x| x.watch) { eyes_on_the_spotted(g, gd, w, &mut look); }
    gun_leads_the_walk(g, gd, &mut look);
    if look.strafe && look.posted && feet.d > 0.6 { back_pedal_watchdog(g, gd, a, &mut look); }
    g.tab.task[a].face_now = look.face;
    let legs = who_has_his_feet(g, gd, a, &feet, sp.as_ref());
    let across: &[GuardId] = if crate::orders::rows::id_is(&order_id, crate::orders::Is::KeepsLane) { mates } else { &[] };
    muzzle_clear_of_friends(g, gd, a, across, &mut look);
    tell_him(g, gd, a, &legs, &look);
}

/// He is in the room, by its door: the door he was sent through is done with.
fn through_his_door(g: &mut Game, gd: GuardId, a: TaskId) {
    if g.tab.task[a].thru.is_none() { return; }
    let Some(post) = g.tab.task[a].post else { return };
    let r = crate::cleared::cleared_rooms_room_at(g, post);
    if !js::truthy(r) { return; }
    let here = g.pos_of(gd);
    if r == crate::cleared::cleared_rooms_room_at(g, here) { g.tab.task[a].thru = None; }
}

/// The fight AI has an alerted man until his fight has gone cold: true while it is hot. Cold, he goes over to searching.
fn fight_has_him(g: &mut Game, gd: GuardId, a: TaskId) -> bool {
    if g.gd[gd].script.is_some() || g.gd[gd].state != crate::game::GuardState::Alert { return false; }
    if g.tab.task[a].yield_t < 0.0 {
        g.tab.task[a].yield_t = g.time;
    }
    if g.tab.task[a].post.is_some() { crate::duty::pin(g, gd, true, "orders"); g.sets[g.seat].orders.ours.add(gd); }
    let hot = g.gd[gd].reload_t >= 0.0
        || g.time - m::max(m::max(g.gd[gd].last_seen_t, g.gd[gd].witness.heard.last_shot_t), g.tab.task[a].yield_t) < g.k.orders.cold_secs;
    if hot { return true; }
    crate::mind::set_state(g, gd, crate::game::GuardState::Search, "orders");
    false
}

/// Back on his order, by the way an order walks him, from wherever the fight left him.
fn back_on_his_order(g: &mut Game, gd: GuardId, a: TaskId) {
    let (now, here) = (g.time, g.pos_of(gd));
    let aa = &mut g.tab.task[a];
    aa.yield_t = -1.0; aa.arrived = false;
    aa.moved_t = now; aa.mark_t = now; aa.mark[0] = here[0]; aa.mark[2] = here[2];
    g.m.orders.follow_word.delete(a);
    let Some(post) = g.tab.task[a].post else {
        g.tab.task[a].via_t = None; g.tab.task[a].via_for = None;
        return;
    };
    let (shy, thru) = (g.tab.task[a].shy, g.tab.task[a].thru);
    let r = squad_orders_route(g, gd, post, shy, thru, None);
    if r.why.is_some() { return; }
    g.tab.task[a].via = r.via;
    g.tab.task[a].goal = match g.tab.task[a].via.first() { Some(&q) => q, None => v3::copy(post) };
    g.tab.task[a].new_leg = true;   // (unguarded, like the writes on the field's list)
}

/// His order's own tick, if it has one, which may rewrite his Task. True: it gave him another.
fn order_sent_him_elsewhere(g: &mut Game, gd: GuardId, a: TaskId) -> bool {
    let order_id = g.tab.task[a].order.clone();
    let def = g.sets[g.seat].orders.defs.iter().copied().find(|&x| g.tab.orders_order_def[x].id == order_id);
    let Some(d) = def else { return false };
    order_tick(g, d, gd, a);
    g.sets[g.seat].orders.asg.get(gd).copied() != Some(a)
}

/// What his walk comes to this frame, for how he is posed and what he is told.
#[derive(Clone, Copy)]
struct Feet {
    /// metres left to where he is going
    d: f64,
    speed: f64,
    face: Option<f64>,
    /// the sneak's own step, where it has one
    s_goal: Option<V3>,
    low: bool,
    /// stepping aside for Sam
    stepping: bool,
    /// he stays where he is, to keep a room
    keep: bool,
    /// his step has to be taken, whoever else would have his feet
    must: bool,
}

/// A follower's place this frame.
#[derive(Clone, Copy)]
struct FilePlace {
    p: V3,
    sam_pos: V3,
    to_sam: f64,
    stopped: bool,
    back_m: f64,
    /// where he is meant to end up
    slot: V3,
    /// his place on Sam's trail, before anything moved it
    on_trail: V3,
    on_wall: Option<V3>,
    off: Option<V3>,
    /// his place was dealt again, off a man ahead's
    spread: bool,
    /// the spot he stands on to let Sam through
    aside: Option<V3>,
}

/// Fall in: his place back along Sam's trail, and the pace that keeps him on it.
fn falls_in(g: &mut Game, gd: GuardId, a: TaskId, followers: &[GuardId]) -> Feet {
    let pl = file_place(g, gd, a, followers);
    let stand = step_or_stand(g, gd, a, &pl);
    head_for_his_place(g, gd, a, &pl, &stand);
    if pl.aside.is_none() && stand.step.is_none() {   // (he is not left standing in a frame or where a leaf swings)
        let place = if stand.near { [pl.p[0], 0.0, pl.p[2]] } else { pl.slot };
        squad_orders_rest_clear(g, gd, a, place, pl.slot, pl.stopped);
    }
    let d = walk_left(g, a, &pl, &stand);
    let sam_v = g.sets[g.seat].orders.sam_v;
    let speed = follow_speed(g, gd, &pl, &stand, d);
    let mut f = Feet { d, speed, face: None, s_goal: None, low: false, stepping: pl.aside.is_some(), keep: false, must: false };
    let must_step = quiet_gait(g, gd, &pl, sam_v, &mut f);
    if stand.low && f.d < 0.5 { f.low = true; }
    let free = pl.aside.is_none() && !must_step;
    f.keep = free && crate::rearwatch::room_kept(g, &crate::seats::here(g), gd);
    if f.keep { f.speed = 0.0; }
    if free {
        if let Some(w) = charge_stand_off(g, gd, &pl, &stand, &f) {
            f.d = dist_xz(pl.p, w);
            runs_to_stand_off(g, gd, a, w, &mut f);
        }
    }
    if free && crate::roomfight::room_fight_short(g, gd) { f.speed = 0.0; }
    f.face = stopped_face(g, a, &pl, f.d);
    g.tab.task[a].arrived = f.d < 0.8;
    squad_orders_follow_word(g, a, f.d);
    f
}

/// His place: back along the trail by his number, onto the wall if the unseen rule wants him there, out of a doorway's lane, apart from the men ahead.
fn file_place(g: &mut Game, gd: GuardId, a: TaskId, followers: &[GuardId]) -> FilePlace {
    let p = g.pos_of(gd);
    let sam_pos = g.player_pos(g.player);
    let to_sam = dist_xz(p, sam_pos);
    let stopped = squad_orders_sam_stopped(&crate::seats::here(g));
    let follow = g.tab.task[a].follow;
    let back_m = squad_orders_back_of(g, follow) + if stopped { squad_orders_clear_back(g, follow) } else { 0.0 };
    let mut slot = squad_orders_slot(g, back_m);
    let on_trail = slot;
    let on_wall = if crate::sneak::sneaking(g, gd) || crate::rearwatch::room_kept(g, &crate::seats::here(g), gd) { None }
        else { crate::unseen::unseen_wall_slot(g, gd, back_m, followers) };
    if let Some(w) = on_wall { slot = w; }
    let ahead = squad_orders_ahead(g, &crate::seats::here(g), gd, a);
    let off = crate::doorway::doorway_off_lane(g, slot, p, follow, Some(&ahead));
    if let Some(o) = off { slot = o; if on_wall.is_some() { crate::unseen::unseen_wall_lost(g, gd); } }
    let mut spread = false;
    if stopped && off.is_none() && on_wall.is_none() {   // (two places on one spot: his is dealt again)
        if let Some(q) = squad_orders_apart_slot(g, gd, a, slot) { spread = true; slot = q; }
    }
    let aside = squad_orders_aside(g, gd);
    FilePlace { p, sam_pos, to_sam, stopped, back_m, slot, on_trail, on_wall, off, spread, aside }
}

/// A short step or a stop in place in place of the walk to his slot.
#[derive(Clone, Copy)]
struct Stand {
    /// a step to a better spot: cover's, or the picture's
    step: Option<V3>,
    /// the picture's step wants him low
    low: bool,
    /// near enough: he stays where he is
    near: bool,
}

/// With Sam stopped: a step for cover, or for the picture the squad makes, else whether he is near enough where he stands.
fn step_or_stand(g: &mut Game, gd: GuardId, a: TaskId, pl: &FilePlace) -> Stand {
    let (stopped, aside) = (pl.stopped, pl.aside);
    let st0 = if stopped && aside.is_none() && !crate::sneak::sneaking(g, gd) && !crate::unseen::under_unseen(g, gd) { crate::cover::cover_step_of(g, gd, a) } else { None };
    let mut near_asked: Option<bool> = None;   // (asked at most once a man a frame)
    let ps = if st0.is_none() && stopped && aside.is_none() && pl.on_wall.is_none() && !crate::sneak::sneaking(g, gd) && !crate::unseen::under_unseen(g, gd)
        && !crate::rearwatch::room_kept(g, &crate::seats::here(g), gd) && !crate::roomfight::room_fight_short(g, gd)
    {
        let near = near_enough(g, gd, a, pl);
        near_asked = Some(near);
        let mark = if near { pl.p } else { pl.slot };
        let busy = crate::roomfight::room_fight_busy(g, gd, 0.5);
        crate::picture::picture_step(g, gd, a.0, mark, Some(&mut |g: &mut Game, q: V3| !rest_bad(g, q)), Some(busy))
    } else { None };
    let (ps_at, low) = match &ps { Some(x) => (Some(x.at), x.low), None => (None, false) };
    let step = ps_at.or(st0);
    let near = step.is_none() && match near_asked { Some(v) => v, None => near_enough(g, gd, a, pl) };
    Stand { step, low, near }
}

/// Sam stopped and he is close enough, in the open, with a line to Sam and nobody on top of him: where he stands will do.
fn near_enough(g: &Game, gd: GuardId, a: TaskId, pl: &FilePlace) -> bool {
    let (p, sam_pos) = (pl.p, pl.sam_pos);
    pl.aside.is_none() && pl.on_wall.is_none() && pl.stopped && pl.to_sam > g.k.orders.near_m && pl.to_sam < pl.back_m + g.k.orders.band_m
        && !squad_orders_in_frame(g, p)
        && !(crate::doorway::doorway_lane(g, p, None).is_some() && (pl.off.is_some() || crate::doorway::doorway_lane(g, pl.slot, None).is_none()))
        && !crate::scene_collision::low_blocked(g, p, sam_pos)
        && !squad_orders_on_top(g, &crate::seats::here(g), gd, a)
}

/// His goal for the frame: the step he was given, where he stands, or the walk to his slot.
fn head_for_his_place(g: &mut Game, gd: GuardId, a: TaskId, pl: &FilePlace, stand: &Stand) {
    if let Some(s) = stand.step {
        if dist_xz(s, g.tab.task[a].goal) > 0.05 { g.tab.task[a].goal = [s[0], 0.0, s[2]]; }
        g.tab.task[a].via.clear();
        return;
    }
    if stand.near {
        if dist_xz(pl.p, g.tab.task[a].goal) > 0.4 { g.tab.task[a].goal = [pl.p[0], 0.0, pl.p[2]]; }
        g.tab.task[a].via.clear();
        return;
    }
    let want = match pl.aside {
        Some(q) => { g.tab.task[a].via.clear(); q }
        None => { way_round_rooms(g, gd, a, pl); g.tab.task[a].via.first().copied().unwrap_or(pl.slot) }
    };
    let bar = if pl.aside.is_some() { 0.25 } else if pl.spread { 0.05 } else { 0.5 };
    if dist_xz(want, g.tab.task[a].goal) > bar { g.tab.task[a].goal = want; }
}

/// A follower a walk from his place does not cut through a room nobody has been in: he goes round it a corner at a time, or by the doors Sam took.
fn way_round_rooms(g: &mut Game, gd: GuardId, a: TaskId, pl: &FilePlace) {
    if asks_the_way_again(g, a, pl) {
        g.tab.task[a].via_t = Some(g.time);
        let via = squad_orders_round_rooms(g, gd, pl.slot);
        g.tab.task[a].via = via;
        g.tab.task[a].via_for = Some([pl.slot[0], 0.0, pl.slot[2]]);
        if g.tab.task[a].via.is_empty() && pl.on_wall.is_none() {   // (no room to go round: the doors Sam took)
            let j = squad_orders_slot_crumb(g, &crate::seats::here(g), pl.back_m);
            let via = squad_orders_trail_way(g, gd, pl.slot, pl.on_trail, j);
            g.tab.task[a].via = via;
        }
    }
    if !g.tab.task[a].via.is_empty() && dist_xz(pl.p, g.tab.task[a].via[0]) < 0.9 { g.tab.task[a].via.remove(0); }
}

/// Is his way stale: his place has moved off it or it is old; with none, a walk has opened up or a straight line has shut.
fn asks_the_way_again(g: &Game, a: TaskId, pl: &FilePlace) -> bool {
    let since = g.time - g.tab.task[a].via_t.unwrap_or(-1e9);
    if !g.tab.task[a].via.is_empty() {
        return match g.tab.task[a].via_for { None => true, Some(f) => (dist_xz(f, pl.slot) > 2.5 && since > 0.5) || since > 3.0 };
    }
    since > 0.75 && (dist_xz(pl.p, pl.slot) > 3.0 || !crate::scene_collision::nav_grid_walkable(g, pl.p, pl.slot))
}

/// How far he has to go: to his goal, and on a way round the whole walk left, since his pace and 'in place' go by that.
fn walk_left(g: &Game, a: TaskId, pl: &FilePlace, stand: &Stand) -> f64 {
    let mut d = dist_xz(pl.p, g.tab.task[a].goal);
    if !g.tab.task[a].via.is_empty() && stand.step.is_none() && !stand.near && pl.aside.is_none() {
        let via = &g.tab.task[a].via;
        let mut q = via[0];
        for i in 1..via.len() { d += dist_xz(q, via[i]); q = via[i]; }
        d += dist_xz(q, pl.slot);
    }
    d
}

/// Brisk out of Sam's way, onto a step at the settling pace, else the team's pace or Sam's own plus what closes the gap.
fn follow_speed(g: &mut Game, gd: GuardId, pl: &FilePlace, stand: &Stand, d: f64) -> f64 {
    let sam_v = g.sets[g.seat].orders.sam_v;
    let (brisk, onto, walk, run, gain) = (g.k.orders.brisk, g.k.orders.onto, g.k.orders.walk, g.k.orders.run, g.k.orders.gain);
    if pl.aside.is_some() { return brisk; }
    if stand.step.is_some() { return onto; }
    match crate::teampace::team_follow(g, gd, sam_v, d, gain) {
        Some(v) => v,
        None => m::max(walk, m::min(run, sam_v + gain * (d - 0.3))),
    }
}

/// The sneak's gait, or else the unseen rule's, or else just how low they want him. True: the sneak says this step must be taken.
fn quiet_gait(g: &mut Game, gd: GuardId, pl: &FilePlace, sam_v: f64, f: &mut Feet) -> bool {
    let sn = if pl.aside.is_some() { None } else { crate::sneak::sneak_follow(g, gd, pl.back_m, sam_v, pl.stopped, f.speed) };
    if let Some(x) = &sn {
        let must = x.must.unwrap_or(false);
        f.must = must; f.speed = x.speed; f.low = x.crouch;
        if let Some(q) = x.goal { f.s_goal = Some(q); f.d = dist_xz(pl.p, q); }
        return must;
    }
    let walk = g.k.orders.walk;
    let un = if pl.aside.is_some() { None } else { crate::unseen::unseen_follow(g, gd, sam_v, f.d, walk) };
    if let Some(u) = &un { f.speed = u.speed; f.low = u.crouch; }
    else { f.low = crate::sneak::sneak_low(g, gd) || crate::unseen::unseen_low(g, gd); }
    false
}

/// The place he is sent to is inside a charge's blast: a stand-off mark instead. Asked last, of the place settled above.
fn charge_stand_off(g: &mut Game, gd: GuardId, pl: &FilePlace, stand: &Stand, f: &Feet) -> Option<V3> {
    let c = g.gd[gd].ch;
    let (keep, s_goal) = (f.keep, f.s_goal);
    crate::wallcharge::wall_charge_clear_of(g, gd, &mut |g: &mut Game| {
        if keep { g.ch[c].pos } else { match s_goal.or(stand.step) { Some(q) => q, None => if stand.near { g.ch[c].pos } else { pl.slot } } }
    })
}

/// He goes to the stand-off mark: nothing keeps him, and only sneaking slows him.
fn runs_to_stand_off(g: &mut Game, gd: GuardId, a: TaskId, w: V3, f: &mut Feet) {
    let (brisk, onto, walk) = (g.k.orders.brisk, g.k.orders.onto, g.k.orders.walk);
    f.must = true;
    if dist_xz(w, g.tab.task[a].goal) > 0.05 { g.tab.task[a].goal = [w[0], 0.0, w[2]]; }
    f.s_goal = None; f.keep = false;
    f.low = crate::sneak::sneak_low(g, gd) || crate::unseen::unseen_low(g, gd);
    f.speed = if crate::sneak::sneaking(g, gd) || crate::unseen::under_unseen(g, gd) { if f.low { onto } else { walk } } else { brisk };
}

/// Stopped: the first man looks on past Sam, the rest back the way they came. On the walk, no facing of his own.
fn stopped_face(g: &Game, a: TaskId, pl: &FilePlace, d: f64) -> Option<f64> {
    if !pl.stopped || d > 0.6 { return None; }
    let sam = g.pl[g.player].ch;
    let sam_pos = g.ch[sam].pos;
    if pl.to_sam < 0.3 { return Some(g.tab.task[a].face_now.unwrap_or(g.ch[sam].body_yaw)); }
    if g.tab.task[a].follow == 0.0 { return Some(v3::yaw_to(pl.p, sam_pos)); }
    Some(v3::yaw_to(sam_pos, pl.p))
}

/// A man's mark this frame.
#[derive(Clone, Copy)]
struct OnMark {
    p: V3,
    /// where he is walked to: his post, or cover's step off it
    at: V3,
    /// cover's step, a step nobody ordered
    step: Option<V3>,
    d: f64,
}

/// A man with a post: he walks to his mark, and when he will get no nearer, the mark comes to him; then is he on it.
fn walks_to_his_mark(g: &mut Game, gd: GuardId, a: TaskId, order_id: &str) -> Feet {
    let p = g.pos_of(gd);
    let post = g.tab.task[a].post.unwrap();
    let face = g.tab.task[a].face;
    let step = cover_step_off_mark(g, gd, a, order_id, p);
    let at = step.unwrap_or(post);
    next_waypoint(g, a, p, post);
    let mut mark = OnMark { p, at, step, d: dist_xz(p, at) };
    let mut speed = pace_to_mark(g, gd, a, order_id, &mark);
    let mut low = false;
    let quiet = match crate::sneak::sneak_post(g, gd, order_id) { Some(v) => Some(v), None => crate::unseen::unseen_post(g, gd, order_id) };
    if let Some(v) = quiet { low = true; speed = m::min(speed, v); }
    if stands_on_purpose(g, gd, a, p) { speed = 0.0; }
    let waited = has_waited(g, a, &mark);
    if mark.d >= 0.35 && waited && !(step.is_none() && squad_orders_rest_short(g, gd, a, mark.d)) { mark.d = gets_no_nearer(g, gd, a, &mark); }
    if step.is_none() && mark.d < 0.35 {
        let aa = &mut g.tab.task[a];
        aa.had = aa.post;
        if mark.d < 0.1 { aa.back_n = Some(0.0); }
    }
    on_his_mark_yet(g, gd, a, order_id, mark.d);
    Feet { d: mark.d, speed, face, s_goal: None, low, stepping: false, keep: false, must: false }
}

/// A plain holder is walked to where he sees through an open door; his mark stays.
fn cover_step_off_mark(g: &mut Game, gd: GuardId, a: TaskId, order_id: &str, p: V3) -> Option<V3> {
    let plain = crate::orders::rows::id_is(order_id, crate::orders::Is::Holds) && g.tab.task[a].watch.is_none()
        && !g.tab.task[a].spare.unwrap_or(false) && g.tab.task[a].via.is_empty();
    if !plain { return None; }
    let post = g.tab.task[a].post.unwrap();
    let st = crate::cover::cover_step_of(g, gd, a);
    let mark = st.unwrap_or(post);
    if dist_xz(g.tab.task[a].goal, mark) > 0.05 {
        let now = g.time;
        let aa = &mut g.tab.task[a];
        aa.goal = [mark[0], 0.0, mark[2]]; aa.mark_t = now; aa.moved_t = now; aa.mark[0] = p[0]; aa.mark[2] = p[2];
    }
    st
}

/// The way round a locked door, a waypoint at a time.
fn next_waypoint(g: &mut Game, a: TaskId, p: V3, post: V3) {
    let aa = &mut g.tab.task[a];
    if aa.via.is_empty() { return; }
    if dist_xz(p, aa.via[0]) < 0.9 { aa.via.remove(0); }
    aa.goal = match aa.via.first() { Some(&q) => q, None => { aa.new_leg = true; v3::copy(post) } };
}

/// His pace: the order's own, else brisk till he is near, then onto it; the team's rules have their say.
fn pace_to_mark(g: &mut Game, gd: GuardId, a: TaskId, order_id: &str, mark: &OnMark) -> f64 {
    let (pace, d) = (g.tab.task[a].pace, mark.d);
    let (k_brisk, k_brisk_in, k_onto) = (g.k.orders.brisk, g.k.orders.brisk_in, g.k.orders.onto);
    let mut speed = match pace {
        Some(v) => v,
        None => if d > k_brisk_in { k_brisk } else { crate::stackline::stack_line_pace(g, gd, a, d, k_brisk).unwrap_or(k_onto) },
    };
    if pace.is_some() { crate::teampace::team_fight(g, gd, None); }
    else if g.tab.task[a].thru.is_none() && g.tab.task[a].via.is_empty() {
        let is_stack = crate::orders::rows::id_is(order_id, crate::orders::Is::Stacks) && g.tab.task[a].door.is_some();
        speed = crate::teampace::team_dash(g, gd, mark.at, d, Some(is_stack)).unwrap_or(speed);
    }
    if pace.is_none() && d > k_brisk_in { speed = crate::teampace::team_told(g, gd).unwrap_or(speed); }
    speed
}

/// A man kept short by the room fight, held at a door or off his mark to fire is not stuck: his clocks start again. True: the room fight keeps him still.
fn stands_on_purpose(g: &mut Game, gd: GuardId, a: TaskId, p: V3) -> bool {
    let short = crate::roomfight::room_fight_short(g, gd);
    if short { mark_clocks(g, a, p); }
    else if (crate::doorway::door_gate_stopped(g, gd) || crate::handleside::handle_gate_stopped(g, gd)) && squad_orders_held_wait(g, a) {
        mark_clocks(g, a, p);   // (a wait, not a shut way)
    }
    if crate::fireline::fire_line_live(g, gd) { mark_clocks(g, a, p); }   // (off his mark on purpose)
    short
}

/// Has he stood too long: by his own moving (his commanded speed says nothing), near the mark for a second, else no ground made over the stuck clock's beat.
fn has_waited(g: &mut Game, a: TaskId, mark: &OnMark) -> bool {
    let (now, p, k_stuck) = (g.time, mark.p, g.k.orders.stuck_secs);
    if dist_xz(p, g.tab.task[a].anchor) > 0.05 {
        let aa = &mut g.tab.task[a];
        aa.anchor[0] = p[0]; aa.anchor[2] = p[2]; aa.moved_t = now;
    }
    let mut stuck = false;
    if now - g.tab.task[a].mark_t >= k_stuck {
        let aa = &mut g.tab.task[a];
        stuck = dist_xz(p, aa.mark) < 0.3;
        aa.mark[0] = p[0]; aa.mark[2] = p[2]; aa.mark_t = now;
    }
    if mark.d < 1.0 && g.tab.task[a].via.is_empty() {
        return g.time - m::max(g.tab.task[a].moved_t, g.tab.task[a].t0) > 1.0;
    }
    stuck
}

/// As near his mark as he will get: cover's step just ends; shouldered off a mark he had, he steps back on it; else the mark comes to him. Gives the distance left.
fn gets_no_nearer(g: &mut Game, gd: GuardId, a: TaskId, mark: &OnMark) -> f64 {
    let (now, p, d) = (g.time, mark.p, mark.d);
    if mark.step.is_some() {   // (a step nobody ordered: it ends where he stands, and he says nothing)
        crate::cover::cover_step_halt(g, gd);
        return d;
    }
    if d < 1.0 && g.tab.task[a].had == g.tab.task[a].post
        && g.tab.task[a].back_n.unwrap_or(0.0) < BACK_TRIES && g.tab.task[a].via.is_empty()
    {
        let cur = g.tab.task[a].post.unwrap();
        let aa = &mut g.tab.task[a];
        aa.back_n = Some(aa.back_n.unwrap_or(0.0) + 1.0);
        aa.goal = v3::copy(cur); aa.moved_t = now; aa.new_leg = true;
        return d;
    }
    if d >= 1.0 {
        g.tab.task[a].there = crate::orders::Doing::new(crate::orders::At::StoppedShort, "stopped short");
        crate::game::game_say(g, gd, "can't get through.", None, None);
    }
    let np = [p[0], 0.0, p[2]];
    let aa = &mut g.tab.task[a];
    aa.post = Some(np); if aa.had == Some(np) { aa.had = None; } aa.goal = v3::copy(np); aa.via.clear(); aa.new_leg = true;
    0.0
}

/// On his mark and looking where he was told, or held there by a stack's fire or silence rule; the moment he first is, noted.
fn on_his_mark_yet(g: &mut Game, gd: GuardId, a: TaskId, order_id: &str, d: f64) {
    let c = g.gd[gd].ch;
    let was = g.tab.task[a].arrived;
    let mut arrived = d < 0.35;
    if arrived {
        let (w, fit, spare) = (g.tab.task[a].watch, g.tab.task[a].fit.is_some(), g.tab.task[a].spare.unwrap_or(false));
        arrived = match w {
            None => true,
            Some(w) => fit || (spare && crate::cover::cover_has(g, gd)) || m::abs(v3::wrap_angle(g.ch[c].aim_yaw - v3::yaw_to(g.ch[c].pos, w))) < 0.2,
        };
    }
    g.tab.task[a].arrived = arrived;
    let held_fire = d < 0.35 && crate::orders::rows::id_is(order_id, crate::orders::Is::Stacks) && crate::holdfire::hold_fire_holds_target(g, gd);
    g.tab.task[a].held_fire = Some(held_fire);
    if held_fire { g.tab.task[a].arrived = true; }
    let held_set = !g.tab.task[a].arrived && d < 0.35 && crate::orders::rows::id_is(order_id, crate::orders::Is::Stacks) && crate::silent::silent_holds_target(g, gd);
    g.tab.task[a].held_set = Some(held_set);
    if held_set { g.tab.task[a].arrived = true; }
    if g.tab.task[a].arrived && !was { g.tab.task[a].arr_t = g.time; }
}

/// How he carries himself: what he is told to look at and how he holds the gun.
#[derive(Clone, Copy)]
struct Carriage {
    pose: (crate::character::Stance, crate::character::UpperMode),
    /// the pose is the order's one for the walk, and the watch the order's own
    pose_is_move: bool,
    watch: Option<V3>,
    posted: bool,
    strafe: bool,
    face: Option<f64>,
}

/// The gun up and leading on the way to a mark, the mark's own carriage as he settles onto it.
fn order_carriage(g: &Game, a: TaskId, feet: &Feet) -> Carriage {
    let aa = &g.tab.task[a];
    let (pose, pose_is_move) = match aa.move_ {
        Some(mv) if feet.d > 0.6 => ((mv.stance, mv.upper), true),
        _ => ((aa.stance, aa.upper), false),
    };
    let posted = aa.post.is_some();
    Carriage { pose, pose_is_move, watch: aa.watch, posted, strafe: aa.strafe.unwrap_or(false) && posted && feet.face.is_some(), face: feet.face }
}

/// the gun up on what he watches
fn aim_pose() -> (crate::character::Stance, crate::character::UpperMode) {
    let k = crate::orders::aim();
    (k.stance, k.upper)
}

/// A man with nothing named to watch has an opening dealt him, on the walk or on his mark; his feet stay the order's.
fn unasked_cover(g: &mut Game, gd: GuardId, a: TaskId, feet: &Feet, look: &mut Carriage) {
    let spare = g.tab.task[a].spare.unwrap_or(false);
    if !(crate::cover::cover_eyes_free(g, gd, a) || spare) { return; }
    let still = if look.posted { feet.d < 0.6 } else { feet.face.is_some() || feet.keep };
    let Some(x) = crate::cover::cover_pose_for(g, gd, still, feet.speed) else { return };
    look.watch = Some(x.at); look.face = x.face; look.strafe = x.strafe; look.pose = aim_pose(); look.pose_is_move = false;
}

/// He has his gun on the enemy he has spotted, unless a pair's order is finishing him.
fn eyes_on_the_spotted(g: &Game, gd: GuardId, w: V3, look: &mut Carriage) {
    if crate::pairorders::pair_orders_finishing(g, gd) { return; }
    look.watch = Some(w); look.face = Some(v3::yaw_to(g.pos_of(gd), w)); look.strafe = false; look.pose = aim_pose(); look.pose_is_move = false;
}

/// On the walk with no line to the order's own watch, the gun leads the way he walks.
fn gun_leads_the_walk(g: &mut Game, gd: GuardId, look: &mut Carriage) {
    if !look.pose_is_move { return; }
    let Some(w) = look.watch else { return };
    if let Some(ahead) = crate::teampace::walk_lead(g, gd, w) { look.watch = Some(ahead); look.strafe = false; }
}

/// A back-pedal has no stuck watchdog: his next corner behind a wall and the way is found again; a second without moving and he walks it the plain way for three.
fn back_pedal_watchdog(g: &mut Game, gd: GuardId, a: TaskId, look: &mut Carriage) {
    let q = g.pos_of(gd);
    let wp = g.gd[gd].path.get(g.gd[gd].path_i).copied();
    let t = g.time;
    let mut w = match g.sets[g.seat].orders.back_fix.get(a) { Some(x) => *x, None => SquadOrdersBackFixValue { plain: -1.0, path: -1.0 } };
    if let Some(wp) = wp {
        if t - w.path > 1.0 && crate::scene_collision::low_blocked(g, q, wp) {
            let man = &mut g.gd[gd];
            man.drop_path();
            w.path = t;
        }
    }
    if t - g.tab.task[a].moved_t > 1.0 { w.plain = t + 3.0; }
    g.sets[g.seat].orders.back_fix.set(a, w);
    if w.plain > t { look.strafe = false; look.face = None; }
}

/// Where his feet go and how, all told.
#[derive(Clone, Copy)]
struct Legs {
    goal: Option<V3>,
    speed: f64,
    /// asked for by the sneak or by what he has spotted
    low: bool,
    /// the goal is the Task's own
    own_goal: bool,
    /// the sneak's step, where it is the one he takes
    sneak_step: Option<V3>,
}

/// Who has his feet: what he has spotted, the line of fire, else the sneak's step, else the order's own goal.
fn who_has_his_feet(g: &mut Game, gd: GuardId, a: TaskId, feet: &Feet, sp: Option<&crate::spotted::SpottedFeet>) -> Legs {
    let own_goal = if feet.keep { None } else { Some(feet.s_goal.unwrap_or(g.tab.task[a].goal)) };
    let fl = if sp.is_some() { None }   // (whenever spotted_feet answers, the line of fire is not asked)
         else { crate::fireline::fire_line_feet(g, gd, own_goal, feet.speed, feet.stepping || feet.must) };
    let (sp_goal, sp_speed, sp_crouch) = match sp { Some(x) => (x.goal, x.speed, x.crouch), None => (None, None, false) };
    let (fl_goal, fl_speed) = match &fl { Some(x) => (x.goal, Some(x.speed)), None => (None, None) };
    let taken = sp_goal.is_some() || fl_goal.is_some();
    Legs {
        goal: sp_goal.or(fl_goal).or(own_goal),
        speed: sp_speed.or(fl_speed).unwrap_or(feet.speed),
        low: feet.low || sp_crouch,
        own_goal: !taken && feet.s_goal.is_none() && !feet.keep,
        sneak_step: if taken { None } else { feet.s_goal },
    }
}

/// Sam in the line of his gun, or on the mark across a gun that is posted a man of the other pair: the muzzle comes down until he is clear of it.
fn muzzle_clear_of_friends(g: &mut Game, gd: GuardId, a: TaskId, across: &[GuardId], look: &mut Carriage) {
    if !(look.pose.1 == crate::character::UpperMode::Aim || look.pose.0 == crate::character::Stance::Stack) { g.tab.task[a].low = false; return; }
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let yaw = match look.watch { Some(w) => v3::yaw_to(p, w), None => g.ch[c].body_yaw };
    let (fx, fz) = (m::sin(yaw), m::cos(yaw));
    let len = match look.watch { Some(w) => dist_xz(p, w), None => 5.0 };
    let lane = if g.tab.task[a].low { g.k.orders.lane_out } else { g.k.orders.lane_in };
    let in_line = |q: V3| -> bool {
        let (sx, sz) = (q[0] - p[0], q[2] - p[2]);
        let along = sx * fx + sz * fz;
        along > 0.15 && along < len && m::abs(sx * fz - sz * fx) < lane
    };
    let mut low_now = squad_orders_sams(&crate::seats::here(g)) && in_line(g.player_pos(g.player));
    if !low_now && g.tab.task[a].arrived {
        low_now = across.iter().any(|&o| o != gd && squad_orders_pair_of(g, o) != squad_orders_pair_of(g, gd) && in_line(g.pos_of(o)));
    }
    g.tab.task[a].low = low_now;
    if low_now { let k = crate::orders::low(); look.pose = (k.stance, k.upper); }
}

/// The lean is written only where one is asked or one is to be taken off: a script patch keeps what it is not told. He leans on his mark, not on the walk to it.
fn lean_word(g: &Game, gd: GuardId, a: TaskId) -> Option<f64> {
    let a_lean = g.tab.task[a].lean.unwrap_or(0.0);
    let script_lean = match &g.gd[gd].script { Some(s) => s.lean.unwrap_or(0.0), None => 0.0 };
    if !js::truthy(a_lean) && !js::truthy(script_lean) { return None; }
    let on_mark = match g.tab.task[a].post { Some(q) => dist_xz(g.pos_of(gd), q) < 0.6, None => false };
    Some(if js::truthy(a_lean) && on_mark { a_lean } else { 0.0 })
}

/// All of it goes to him as one order. Everything it passes is a kept point, compared by value; a goal written anew is the one exception (`new_leg`, spent when it is the arm
/// chosen).
fn tell_him(g: &mut Game, gd: GuardId, a: TaskId, legs: &Legs, look: &Carriage) {
    let crouch = g.tab.task[a].crouch.unwrap_or(false) || legs.low;
    let lean = lean_word(g, gd, a);
    let (stack_side, light) = (g.tab.task[a].stack_side, g.tab.task[a].light);
    let fresh = legs.own_goal && g.tab.task[a].new_leg;
    if fresh { g.tab.task[a].new_leg = false; }
    crate::squad::order(g, gd, crate::squad::OrderPatch {
        goal: Some(legs.goal), speed: Some(Some(legs.speed)), face: Some(look.face), strafe: Some(Some(look.strafe)), aim_at: Some(look.watch),
        stance: Some(Some(look.pose.0)), upper: Some(Some(look.pose.1)), stack_side: Some(Some(stack_side)), crouch: Some(Some(crouch)),
        hot: Some(Some(false)), light: Some(light), lean: lean.map(Some),
        held: !fresh,
        ..Default::default()
    });
    if let Some(q) = legs.sneak_step { crate::sneak::sneak_way(g, gd, q); }
}

/// 'set.': once, from the last man on, when everybody the order sent is on his mark.
fn say_set(g: &mut Game) {
    let mut at = 0usize;
    while let Some(k) = g.sets[g.seat].orders.asg.next_key(&mut at) {
        let a = *g.sets[g.seat].orders.asg.get(k).unwrap();
        let line = match &g.tab.task[a].set_line { Some(s) if !s.is_empty() => s.clone(), _ => continue };
        if g.tab.task[a].set_said || !g.tab.task[a].arrived { continue; }
        let group = g.tab.task[a].group;
        let Some(last) = last_man_on(g, group) else { continue };
        // (one of them said "can't get through": the group is not where it was sent, and nobody calls it set)
        let short = g.sets[g.seat].orders.asg.values().any(|&b| { let bb = &g.tab.task[b]; bb.group == group && bb.there.at == crate::orders::At::StoppedShort });
        if !short { crate::game::game_say(g, last, &line, None, None); }
        for (_, &b) in g.sets[g.seat].orders.asg.iter() { if g.tab.task[b].group == group { g.tab.task[b].set_said = true; } }
    }
}

/// The last man of the group onto his mark, when all of them are and none has a door still to try.
fn last_man_on(g: &Game, group: f64) -> Option<GuardId> {
    let (mut last, mut last_t) = (None, f64::NEG_INFINITY);
    for (o, &b) in g.sets[g.seat].orders.asg.iter() {
        let bb = &g.tab.task[b];
        if bb.group == group {
            if !bb.arrived || bb.tries.is_some() { return None; }
            if bb.arr_t > last_t { last_t = bb.arr_t; last = Some(o); }
        }
    }
    last
}

/// The clocks that would call a standing man stuck start again.
fn mark_clocks(g: &mut Game, a: TaskId, p: V3) {
    let now = g.time;
    let aa = &mut g.tab.task[a];
    aa.moved_t = now; aa.mark_t = now; aa.mark[0] = p[0]; aa.mark[2] = p[2];
}

/// The spot he was already stepping to, while Sam has not got past him.
fn aside_held(g: &Game, seat: &crate::seats::SeatRef, gd: GuardId) -> Option<V3> {
    let (sam, p) = (g.seat_pos(seat), g.pos_of(gd));
    let row = match seat.orders.aside_mem.get(gd) { Some(x) => x, None => return None };
    if g.time - row.t > g.k.orders.aside_hold || g.time < row.t { return None; }
    if (p[0] - sam[0]) * row.dir[0] + (p[2] - sam[2]) * row.dir[2] > -0.3 && dist_xz(p, sam) < g.k.orders.aside_m + 0.5 { Some(row.q) } else { None }
}

/// Where a follower stands to let Sam through: Sam walking AT him; None: not in the way, or no clear spot.
fn squad_orders_aside(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> {
    let (sam, p) = (g.player_pos(g.player), g.pos_of(gd));
    if g.sets[g.seat].orders.sam_v < 0.3 { return aside_held(g, &crate::seats::here(g), gd); }
    let dir = g.sets[g.seat].orders.sam_dir;
    let (mx, mz) = (p[0] - sam[0], p[2] - sam[2]);
    let ahead = mx * dir[0] + mz * dir[2];
    let off = mx * dir[2] - mz * dir[0];
    if ahead <= 0.0 || ahead > g.k.orders.aside_m || m::abs(off) > g.k.orders.aside_lane { return aside_held(g, &crate::seats::here(g), gd); }
    // (ORDERS.asideClear: the side he is already stepping to stays his first side)
    let own = if off >= 0.0 { 1.0 } else { -1.0 };
    let first = match g.sets[g.seat].orders.aside_mem.get(gd) {
        Some(was) if was.side.map_or(false, js::truthy) && g.time >= was.t && g.time - was.t <= g.k.orders.aside_hold => was.side.unwrap(),
        _ => own,
    };
    for w in [g.k.orders.aside_off, 0.65] {
        for side in [first, -first] {
            let q: V3 = [sam[0] + dir[0] * ahead + dir[2] * side * w, 0.0, sam[2] + dir[2] * ahead - dir[0] * side * w];
            if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
                && !crate::scene_collision::low_blocked(g, p, q)
                && !(squad_orders_in_frame(g, q) || crate::doorway::doorway_stand(g, q))
            {
                let t = g.time;
                g.sets[g.seat].orders.aside_mem.set(gd, SquadOrdersAsideMemValue { q, t, dir: [dir[0], 0.0, dir[2]], side: Some(side) });
                return Some(q);
            }
        }
    }
    aside_held(g, &crate::seats::here(g), gd)
}

/// Sam's trail: a crumb every `crumb` metres he has moved, `trailM` metres of it kept, newest last.
fn squad_orders_crumb(g: &mut crate::game::Game, p: V3, men: &[GuardId]) {
    if let Some(&last) = g.sets[g.seat].orders.trail.last() { if dist_xz(last, p) > 3.0 { g.sets[g.seat].orders.trail.clear(); } }   // (a teleport)
    if g.sets[g.seat].orders.trail.is_empty() {   // no trail yet: 'behind him' is the side his men are on
        let (mut cx, mut cz) = (0.0, 0.0);
        for &gd in men { let q = g.pos_of(gd); cx += q[0] - p[0]; cz += q[2] - p[2]; }
        let l = m::hyp2(cx, cz);
        let yaw = g.ch[g.pl[g.player].ch].body_yaw;
        g.sets[g.seat].orders.back = if l > 0.3 { [cx / l, 0.0, cz / l] } else { [-m::sin(yaw), 0.0, -m::cos(yaw)] };
    }
    let k_crumb = g.k.orders.crumb;
    let k_trail = g.k.orders.trail_m;
    let t = &mut g.sets[g.seat].orders.trail;
    if t.is_empty() || dist_xz(t[t.len() - 1], p) >= k_crumb { t.push([p[0], 0.0, p[2]]); }
    let mut len = 0.0;
    for i in (1..t.len()).rev() {
        len += dist_xz(t[i], t[i - 1]);
        if len > k_trail { t.drain(0..i - 1); break; }
    }
}

/// Where the men ahead of him in the fall-in file are bound.
fn squad_orders_ahead(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId, a: TaskId) -> Vec<V3> {
    let mut out: Vec<V3> = Vec::new();
    let follow = g.tab.task[a].follow;
    for (o, &b) in seat.orders.asg.iter() {
        let bb = &g.tab.task[b];
        if o != gd && bb.post.is_none() && bb.follow < follow && !crate::condition::down(g, g.gd[o].ch) { out.push(bb.goal); }
    }
    out
}

/// Stopped 'near enough' within a body and a bit of a man ahead of him, or one on a post.
fn squad_orders_on_top(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId, a: TaskId) -> bool {
    let follow = g.tab.task[a].follow;
    let (mine, apart) = (g.pos_of(gd), g.k.doormanners.apart);
    for (o, &b) in seat.orders.asg.iter() {
        let bb = &g.tab.task[b];
        if o != gd && !crate::condition::down(g, g.gd[o].ch) && (bb.post.is_some() || bb.follow < follow) && dist_xz(g.pos_of(o), mine) < apart { return true; }
    }
    false
}

/// The open doorway `p` stands in, for a place in the file.
fn squad_orders_in_frame(g: &crate::game::Game, p: V3) -> bool {
    if crate::doorway::doorway_stand(g, p) { return true; }
    let k = g.k.orders.off_door;
    if !(k > 0.0) { return false; }
    let mut i = 0usize;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        if g.dr[d].leaf.latched || m::abs(g.dr[d].leaf.angle) < 0.6 || dist_xz(p, g.dr[d].frame_centre) > g.dr[d].def.width + k { continue; }
        let f = crate::guards::frame_of(g, d);
        let along = (p[0] - f.hx) * f.ux + (p[2] - f.hz) * f.uz;
        let off = (p[0] - f.hx) * f.nx + (p[2] - f.hz) * f.nz;
        if along > -0.05 && along < f.w + 0.05 && m::abs(off) < k { return true; }
    }
    false
}

fn rest_bad(g: &Game, q: V3) -> bool { squad_orders_in_frame(g, q) || crate::doorway::doorway_stand(g, q) || crate::doorway::doorway_lane(g, q, None).is_some() }
fn rest_others(g: &Game, seat: &crate::seats::SeatRef, gd: GuardId) -> Vec<V3> {
    let mut out: Vec<V3> = vec![squad_orders_sam_stood(g, seat)];
    for (o, &b) in seat.orders.asg.iter() {
        if o != gd && !crate::condition::down(g, g.gd[o].ch) { out.push(g.pos_of(o)); out.push(g.tab.task[b].goal); }
    }
    out
}

/// A follower is not left standing in a doorway or where a leaf swings.
fn squad_orders_rest_clear(g: &mut crate::game::Game, gd: GuardId, a: TaskId, place: V3, trail: V3, stopped: bool) {
    let (p, t) = (g.pos_of(gd), g.time);
    let apart = g.k.doormanners.apart;
    if !g.sets[g.seat].orders.rest_of.has(a) {
        g.sets[g.seat].orders.rest_of.set(a, SquadOrdersRestOfValue { x: p[0], z: p[2], t, step: None, until: 0.0, from: [0.0, 0.0, 0.0], n: 0.0, hold: None, hold_for: [0.0, 0.0, 0.0] });
    }
    {
        let r = g.sets[g.seat].orders.rest_of.get_mut(a).unwrap();
        if m::hyp2(p[0] - r.x, p[2] - r.z) > 0.2 { r.x = p[0]; r.z = p[2]; r.t = t; }
    }
    if !g.tab.task[a].via.is_empty() {
        let r = g.sets[g.seat].orders.rest_of.get_mut(a).unwrap();
        r.step = None; r.hold = None;
        return;
    }
    {
        let r = g.sets[g.seat].orders.rest_of.get_mut(a).unwrap();
        if let Some(step) = r.step {
            if t > r.until || dist_xz(trail, r.from) > 1.0 { r.step = None; r.t = t; }
            else { g.tab.task[a].goal = step; return; }   // (the same point every frame: one leg)
        }
        if js::truthy(r.n) && dist_xz(trail, r.from) > 1.0 { r.n = 0.0; }
        if !stopped || dist_xz(trail, r.hold_for) > 0.5 { r.hold = None; }
    }
    let goal = g.tab.task[a].goal;
    if stopped && (squad_orders_in_frame(g, goal) || crate::doorway::doorway_stand(g, goal)) {
        if let Some(h) = g.sets[g.seat].orders.rest_of.get(a).unwrap().hold { g.tab.task[a].goal = h; return; }   // (dealt once for where the file stands)
        let others = rest_others(g, &crate::seats::here(g), gd);
        let q = if !rest_bad(g, place) && others.iter().all(|&o| dist_xz(o, place) >= apart) { Some(place) }
            else { crate::doorway::doorway_clear_near(g, goal, &mut |g: &mut Game, x: V3| rest_bad(g, x), Some(&others), Some(apart), Some(p)) };
        if let Some(q) = q {
            g.tab.task[a].goal = q;
            let r = g.sets[g.seat].orders.rest_of.get_mut(a).unwrap();
            r.hold = Some(q); r.hold_for = [trail[0], 0.0, trail[2]];
        }
        return;
    }
    if crate::doorway::door_gate_stopped(g, gd) { g.sets[g.seat].orders.rest_of.get_mut(a).unwrap().t = t; return; }   // (short of a frame for a man in it)
    let (rt, rn) = { let r = g.sets[g.seat].orders.rest_of.get(a).unwrap(); (r.t, r.n) };
    if t - rt >= g.k.doormanners.rest_secs * (1.0 + rn) && dist_xz(p, goal) < 1.0 && crate::doorway::doorway_stand(g, p) {   // (at rest by his place)
        let others = rest_others(g, &crate::seats::here(g), gd);
        let sam = g.player_pos(g.player);
        let q = crate::doorway::doorway_clear_near(g, p, &mut |g: &mut Game, x: V3| rest_bad(g, x), Some(&others), Some(apart), Some(sam));
        if let Some(q) = q {
            {
                let r = g.sets[g.seat].orders.rest_of.get_mut(a).unwrap();
                r.step = Some(q); r.until = t + 4.0; r.from = [trail[0], 0.0, trail[2]]; r.n = m::min(r.n + 1.0, 4.0);
            }
            g.tab.task[a].goal = q;
        } else { g.sets[g.seat].orders.rest_of.get_mut(a).unwrap().t = t; }
    }
}

/// The seconds of one order's walk the PASS gate's holds may keep its stuck clocks from running.
fn squad_orders_held_wait(g: &mut crate::game::Game, a: TaskId) -> bool {
    let t = g.time;
    if !g.sets[g.seat].orders.held_of.has(a) { g.sets[g.seat].orders.held_of.set(a, SquadOrdersHeldOfValue { last: t, total: 0.0 }); }
    let cap = g.k.doormanners.held_cap;
    let h = g.sets[g.seat].orders.held_of.get_mut(a).unwrap();
    h.total += m::min(m::max(t - h.last, 0.0), 0.1);
    h.last = t;
    h.total <= cap
}

/// As near his mark as he will get: his mark does not come to him while he stands in a doorway's way. True = dealt with here.
fn squad_orders_rest_short(g: &mut crate::game::Game, gd: GuardId, a: TaskId, d: f64) -> bool {
    let p = g.pos_of(gd);
    let post = match g.tab.task[a].post { Some(q) => q, None => return false };
    if d < 1.0 || !crate::doorway::doorway_stand(g, p) { return false; }   // (a stride from his mark he has as good as made it)
    if !g.sets[g.seat].orders.short_of.has(a) { g.sets[g.seat].orders.short_of.set(a, SquadOrdersShortOfValue { x: p[0], z: p[2], n: 0.0, k: 0.0 }); }
    let first = {
        let r = g.sets[g.seat].orders.short_of.get_mut(a).unwrap();
        if r.k >= 2.0 { return false; }   // (his mark has been moved for him twice on this order: the old rule ends it)
        if m::hyp2(p[0] - r.x, p[2] - r.z) > 1.0 { r.n = 0.0; }
        r.x = p[0]; r.z = p[2];
        let n = r.n; r.n += 1.0;
        n == 0.0
    };
    if first {
        let now = g.time;
        let aa = &mut g.tab.task[a];
        aa.mark_t = now; aa.moved_t = now;
        return true;
    }
    let mut taken: Vec<V3> = vec![squad_orders_sam_stood(g, &crate::seats::here(g))];
    for (o, &b) in g.sets[g.seat].orders.asg.iter() {
        if o != gd && !crate::condition::down(g, g.gd[o].ch) {
            taken.push(g.pos_of(o));
            if let Some(q) = g.tab.task[b].post { taken.push(q); }
        }
    }
    let sam = squad_orders_sam_stood(g, &crate::seats::here(g));
    let face: V3 = if dist_xz(sam, p) < 1.3 { [2.0 * p[0] - sam[0], 0.0, 2.0 * p[2] - sam[2]] } else { post };   // (in a frame Sam stands in the mouth of: back off it, away from him)
    let apart = g.k.doormanners.apart;
    let q = crate::doorway::doorway_clear_near(g, p, &mut |g: &mut Game, x: V3| squad_orders_in_frame(g, x) || crate::doorway::doorway_stand(g, x), Some(&taken), Some(apart), Some(face));
    let q = match q { Some(q) => q, None => return false };
    g.sets[g.seat].orders.short_of.get_mut(a).unwrap().k += 1.0;
    g.tab.task[a].there = crate::orders::Doing::new(crate::orders::At::StoppedShort, "stopped short");
    crate::game::game_say(g, gd, "can't get through.", None, None);
    let now = g.time;
    let aa = &mut g.tab.task[a];
    aa.post = Some(q); if aa.had == Some(q) { aa.had = None; } aa.goal = v3::copy(q); aa.via.clear(); aa.mark_t = now; aa.moved_t = now; aa.new_leg = true;
    true
}

/// Sam stopped: how much further back along the trail the `follow`-th place stands so that it and every place ahead of it is clear of a doorway.
fn squad_orders_clear_back(g: &mut crate::game::Game, follow: f64) -> f64 {
    if !(g.k.orders.off_door > 0.0) { return 0.0; }
    if g.sets[g.seat].orders.clear_t != g.time { g.sets[g.seat].orders.clear_t = g.time; g.sets[g.seat].orders.clear_by.clear(); }
    let mut i = g.sets[g.seat].orders.clear_by.len();
    while (i as f64) <= follow {
        let mut e = if i > 0 { g.sets[g.seat].orders.clear_by[i - 1] } else { 0.0 };
        let base = squad_orders_back_of(g, i as f64);
        for _ in 0..8 {
            let q = squad_orders_slot(g, base + e);
            if !squad_orders_in_frame(g, q) { break; }
            let further = squad_orders_slot(g, base + e + 0.3);
            if dist_xz(q, further) < 0.01 { break; }   // (ORDERS.backEnd: the trail and the floor behind it have run out)
            e += 0.3;
        }
        g.sets[g.seat].orders.clear_by.push(e);
        i += 1;
    }
    g.sets[g.seat].orders.clear_by[follow as usize]
}

/// A follower's place in the file, in metres back along the trail.
fn squad_orders_back_of(g: &crate::game::Game, follow: f64) -> f64 {
    let list = &g.k.orders.follow_back;
    let n = list.len() as f64;
    list[m::min(follow, n - 1.0) as usize] + m::max(0.0, follow - n + 1.0) * 1.1
}

/// The way they came: the point `extra` metres further back along Sam's trail than this follower's own place. None: he is not following.
pub fn squad_orders_trail_back(g: &crate::game::Game, gd: GuardId, extra: f64) -> Option<V3> {
    let a = *g.sets[g.seat].orders.asg.get(gd)?;
    if g.tab.task[a].post.is_some() { return None; }
    let follow = g.tab.task[a].follow;
    let back = squad_orders_back_of(g, follow) + extra;
    Some(squad_orders_slot(g, back))
}

fn apart_bad(g: &Game, x: V3) -> bool { squad_orders_in_frame(g, x) || crate::doorway::doorway_lane(g, x, None).is_some() }
fn apart_clear(t: &[V3], x: V3, by: f64) -> bool { t.iter().all(|&o| dist_xz(o, x) >= by - 0.05) }
fn apart_short(g: &Game, p: V3, slot: V3, t: &[V3], k: f64) -> Option<V3> {
    if dist_xz(p, slot) < 1.5 && !apart_bad(g, p) && apart_clear(t, p, k) && !crate::scene_collision::low_blocked(g, p, slot) { Some([p[0], 0.0, p[2]]) } else { None }
}
///  None: the slot itself.
fn apart_hold(g: &Game, p: V3, sam: V3, slot: V3, t: &[V3], k: f64) -> Option<V3> {
    if dist_xz(p, slot) >= 1.5 || apart_bad(g, p) || crate::scene_collision::low_blocked(g, p, slot) { return None; }
    let here: V3 = [p[0], 0.0, p[2]];
    let by = m::min(k, g.k.orders.file_tight);
    let mut near: Option<usize> = None;   // (an index: `o === t` below is the identity of an entry)
    for (i, &o) in t.iter().enumerate() {
        if dist_xz(o, p) < by - 0.05 && !crate::scene_collision::low_blocked(g, o, p) && (match near { None => true, Some(n) => dist_xz(o, p) < dist_xz(t[n], p) }) { near = Some(i); }
    }
    let ti = match near { Some(n) => n, None => return Some(here) };
    let to = t[ti];
    let d = dist_xz(to, p);
    if d < 1e-3 { return Some(here); }
    let mut r = by;
    while r > d + 0.02 {
        for ang in [0.0, 0.4, -0.4, 0.8, -0.8] {   // (straight away first, then a little either side of it)
            let (c, sn) = (m::cos(ang), m::sin(ang));
            let (ux, uz) = ((p[0] - to[0]) / d, (p[2] - to[2]) / d);
            let x: V3 = [to[0] + (ux * c + uz * sn) * r, 0.0, to[2] + (uz * c - ux * sn) * r];
            if !apart_bad(g, x) && !crate::scene_collision::nav_grid_is_blocked(g, x[0], x[2]) && !crate::scene_collision::low_blocked(g, p, x)
                && t.iter().enumerate().all(|(i, &o)| i == ti || dist_xz(o, x) >= by - 0.05) && dist_xz(sam, x) >= by - 0.05
            {
                return Some(x);
            }
        }
        r -= 0.05;
    }
    Some(here)
}

/// ORDERS.fileSpread: a follower's place within a body's room of where a man ahead will stand: the nearest clear floor. None: the slot itself, what the tick's `q !== slot` asks; Some: any other point, of the same value or not.
fn squad_orders_apart_slot(g: &mut crate::game::Game, gd: GuardId, a: TaskId, slot: V3) -> Option<V3> {
    let k = g.k.orders.file_apart;
    let (sam, p) = (g.player_pos(g.player), g.pos_of(gd));
    let mut t = squad_orders_ahead(g, &crate::seats::here(g), gd, a);
    for (o, &b) in g.sets[g.seat].orders.asg.iter() {
        let bb = &g.tab.task[b];
        if o != gd && !crate::condition::down(g, g.gd[o].ch) && bb.post.is_some() { t.push(bb.goal); }
    }
    let mut any = false;
    for &q in t.iter() { if dist_xz(q, slot) < k && !crate::scene_collision::low_blocked(g, q, slot) { any = true; break; } }
    if !any { return None; }
    if let Some(&row) = g.sets[g.seat].orders.apart_of.get(a) {
        if dist_xz(row.at, slot) < 0.5 {
            if let Some(q) = row.q {
                if (row.short.unwrap_or(false) || apart_clear(&t, q, row.m)) && !apart_bad(g, q) { return Some(q); }
            } else if m::abs(g.time - row.t) < 1.0 {
                let q = match apart_short(g, p, slot, &t, k) { Some(q) => q, None => return apart_hold(g, p, sam, slot, &t, k) };
                let r = g.sets[g.seat].orders.apart_of.get_mut(a).unwrap();
                r.q = Some(q); r.short = Some(true);
                return Some(q);
            }
        }
    }
    let mut taken = t.clone();
    taken.push([sam[0], 0.0, sam[2]]);
    let (mut q, mut by, mut stop): (Option<V3>, f64, bool) = (None, k, false);
    let tight = g.k.orders.file_tight;
    let tries: Vec<f64> = if tight < k { vec![k, tight] } else { vec![k] };
    for ap in tries {   // (doorwayClearNear itself wants a clear line from the place to the point)
        q = crate::doorway::doorway_clear_near(g, slot, &mut |g: &mut Game, x: V3| apart_bad(g, x) || crate::scene_collision::low_blocked(g, x, sam), Some(&taken), Some(ap), Some(p));
        if q.is_none() { q = crate::doorway::doorway_clear_near(g, slot, &mut |g: &mut Game, x: V3| apart_bad(g, x), Some(&taken), Some(ap), Some(p)); }
        if q.is_some() { by = ap; break; }
    }
    if q.is_none() { q = apart_short(g, p, slot, &t, k); stop = q.is_some(); }
    let now = g.time;
    g.sets[g.seat].orders.apart_of.set(a, SquadOrdersApartOfValue { at: [slot[0], 0.0, slot[2]], q, t: now, m: by, short: Some(stop) });
    match q { Some(q) => Some(q), None => apart_hold(g, p, sam, slot, &t, k) }
}

/// : straight on, a quarter metre at a time, while a man can stand there and walk there
fn slot_reach(g: &Game, prev: V3, dir: V3, left: f64, ang: f64) -> (V3, f64) {
    let (c, sn) = (m::cos(ang), m::sin(ang));
    let d: V3 = [dir[0] * c + dir[2] * sn, 0.0, dir[2] * c - dir[0] * sn];
    let (mut p, mut rem) = (prev, left);
    while rem > 1e-3 {
        let s = m::min(0.25, rem);
        let nx: V3 = [p[0] + d[0] * s, 0.0, p[2] + d[2] * s];
        if crate::scene_collision::nav_grid_is_blocked(g, nx[0], nx[2]) || crate::scene_collision::low_blocked(g, p, nx) { break; }
        p = nx;
        rem -= s;
    }
    (p, rem)
}

/// The point `back` metres behind Sam along his trail; past its end, on straight the way it was going, as far as the floor is free.
fn squad_orders_slot(g: &crate::game::Game, back: f64) -> V3 {
    let sam = g.player_pos(g.player);
    let mut prev: V3 = [sam[0], 0.0, sam[2]];
    let mut left = back;
    let mut dir: V3 = g.sets[g.seat].orders.back;
    for i in (0..g.sets[g.seat].orders.trail.len()).rev() {
        let ti = g.sets[g.seat].orders.trail[i];
        let seg = dist_xz(prev, ti);
        if seg < 1e-4 { continue; }
        let sd: V3 = [(ti[0] - prev[0]) / seg, 0.0, (ti[2] - prev[2]) / seg];
        if seg > 0.2 { dir = sd; }
        if seg >= left { return [prev[0] + sd[0] * left, 0.0, prev[2] + sd[2] * left]; }
        left -= seg;
        prev = ti;
    }
    if left <= 1e-3 { return [prev[0], 0.0, prev[2]]; }
    // off the end of the trail; where straight on is cut short, the first of a few headings either side of it that has the room (else the longest)
    let mut best = slot_reach(g, prev, dir, left, 0.0);
    for ang in [0.7, -0.7, 1.4, -1.4] {
        if best.1 <= 1e-3 { break; }
        let r = slot_reach(g, prev, dir, left, ang);
        if r.1 < best.1 - 0.2 { best = r; }
    }
    [best.0[0], 0.0, best.0[2]]
}

// ------------------------------------------------------------ geometry

/// The free floor at p, else the nearest within floorSnapM (the candidate nearest Sam among equals). None: nowhere.
pub fn squad_orders_free_spot(g: &crate::game::Game, seat: &crate::seats::SeatRef, p: V3) -> Option<V3> {
    if !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return Some([p[0], 0.0, p[2]]); }
    let sam = g.seat_pos(seat);
    let (mut best, mut bs): (Option<V3>, f64) = (None, f64::INFINITY);
    let reach = g.k.orders.floor_snap_m + 1e-6;
    let mut r = 0.25;
    while r <= reach {
        for k in 0..12 {
            let a = (k as f64) * m::PI / 6.0;
            let q: V3 = [p[0] + m::sin(a) * r, 0.0, p[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { continue; }
            let s = r + 0.05 * dist_xz(q, sam);
            if s < bs { bs = s; best = Some(q); }
        }
        if best.is_some() { break; }
        r += 0.25;
    }
    best
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    fn run(f: fn(&mut Game)) {
        let h = std::thread::Builder::new().stack_size(1 << 28).spawn(move || { let mut g = Game::blank(); f(&mut g); }).unwrap();
        h.join().unwrap();
    }

    /// the file's places: the two the table holds, then 1.1 m apart behind them
    #[test]
    fn back_of_runs_on_past_the_table() {
        run(|g| {
            g.k.orders.follow_back = vec![1.3, 2.4];
            assert_eq!(squad_orders_back_of(g, 0.0), 1.3);
            assert_eq!(squad_orders_back_of(g, 1.0), 2.4);
            assert_eq!(squad_orders_back_of(g, 2.0), 2.4 + 1.0 * 1.1);
            assert_eq!(squad_orders_back_of(g, 3.0), 2.4 + 2.0 * 1.1);
        });
    }

    /// a hold's seconds count at most a tenth a frame, and stop being a wait past the cap
    #[test]
    fn held_wait_caps_each_frame_and_the_whole() {
        run(|g| {
            let a = TaskId(0);
            g.k.doormanners.held_cap = 0.15;
            g.time = 10.0;
            assert!(squad_orders_held_wait(g, a));
            g.time = 11.0;
            assert!(squad_orders_held_wait(g, a));
            assert_eq!(g.sets[g.seat].orders.held_of.get(a).unwrap().total, 0.1);
            g.time = 12.0;
            assert!(!squad_orders_held_wait(g, a));
        });
    }

    /// the trail takes a crumb every `crumb` metres, on the floor, and keeps `trail_m` metres of them
    #[test]
    fn crumb_spaces_and_trims() {
        run(|g| {
            g.k.orders.crumb = 0.4;
            g.k.orders.trail_m = 1.0;
            g.sets[g.seat].orders.trail = vec![[0.0, 0.0, 0.0]];
            squad_orders_crumb(g, [0.3, 1.5, 0.0], &[]);   // short of a crumb: nothing
            assert_eq!(g.sets[g.seat].orders.trail.len(), 1);
            for i in 1..6 { squad_orders_crumb(g, [0.5 * i as f64, 1.5, 0.0], &[]); }
            // (the walk back from the newest passes 1 m at the third crumb, and one crumb past it is kept)
            assert_eq!(g.sets[g.seat].orders.trail.iter().map(|q| q[0]).collect::<Vec<_>>(), vec![1.0, 1.5, 2.0, 2.5]);
            assert_eq!(g.sets[g.seat].orders.trail[0][1], 0.0);   // (on the floor, whatever his height)
        });
    }
}
