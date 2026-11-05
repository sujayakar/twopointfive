//! src/game/orders.ts lines 1354 to 1726
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// (orders.ts:340 `flat = v3.distXZ`)
fn flat(a: V3, b: V3) -> f64 { v3::dist_xz(a, b) }
/// (orders.ts:340 `yawTo = v3.yawTo`)
fn yaw_to(a: V3, b: V3) -> f64 { v3::yaw_to(a, b) }

/// (orders.ts:2629 LOW: `crate::orders::low()` holds the words as Strings; a patch and a pose take the enums)
const LOW: (crate::character::Stance, crate::character::UpperMode) = (crate::character::Stance::LowReady, crate::character::UpperMode::None);
/// (orders.ts:2630 AIM: as LOW)
const AIM: (crate::character::Stance, crate::character::UpperMode) = (crate::character::Stance::None, crate::character::UpperMode::Aim);
/// (orders.ts:2631 LANE_ORDERS: `crate::orders::lane_orders()` is a Vec of Strings made afresh a call; this is asked every frame a man)
static LANE_ORDERS: &[&str] = &["cover", "stack", "move", "hold"];

// ------------------------------------------------------------ what a stored function became

/// (orders.ts:630 the row `owners.push` writes, `owns`; `name` says which row. Nightbreach is the only owner the game pushes)
fn owner_owns(g: &mut Game, owner: usize, gd: GuardId) -> bool {
    if g.orders.owners[owner].name != "nightbreach" { return false; }
    g.night_breach.active && crate::nightbreach::night_breach_drill_ops(g).contains(&gd) && crate::nightbreach::night_breach_owns_pair(g)
}

/// (pairorders.ts:232 the 'watch him' row's `tick`: his gun stays on the man he was told to watch, while he sees him)
fn pair_watch_tick(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) {
    let h = match g.tab.orders_assignment[a].foe { Some(h) => h, None => return };
    if g.gd[h].state == crate::game::GuardState::Dead || !g.guards.contains(&h) { crate::pairorders::pair_orders_stand(g, gd, "holding", None); return; }
    if !crate::pairorders::pair_orders_sees(g, gd, h) { return; }   // (behind something: the gun stays on where he last saw him)
    let (c, p) = (g.ch[g.gd[h].ch].pos, g.ch[g.gd[gd].ch].pos);
    let aa = &mut g.tab.orders_assignment[a];
    if let Some(w) = &mut aa.watch { w[0] = c[0]; w[2] = c[2]; }
    aa.face = Some(yaw_to(p, c));
}

/// (orders.ts:1393 `def.tick(this, gd, A)`: each arm is the closure of the row that holds it)
fn def_tick(g: &mut Game, tick: OrderTick, gd: GuardId, a: OrdersAssignmentId) {
    match tick {
        OrderTick::Move => crate::roomfit::fit_tick(g, gd, a),
        OrderTick::ShutDoor => crate::shutdoor::tick(g, gd, a),
        OrderTick::ThrowRow => crate::squadthrow::walk_tick(g, gd, a),
        OrderTick::Wand => crate::wandorder::tick(g, gd, a),
        OrderTick::Shell40 => crate::squad40::turn_tick(g, gd, a),
        OrderTick::Work => crate::workthings::work_things_tick(g, gd, a),
        OrderTick::PairTake => crate::pairorders::pair_orders_take_tick(g, gd, a),
        OrderTick::PairWatch => pair_watch_tick(g, gd, a),
        OrderTick::PairDark => crate::pairorders::pair_orders_dark_tick(g, gd, a),
    }
}

// ------------------------------------------------------------ the men

/// A man's order goes, and with it the pin the fight seam put on him. (orders.ts SquadOrders.drop)
pub fn squad_orders_drop(g: &mut crate::game::Game, gd: GuardId) {
    g.orders.asg.delete(gd);
    if g.orders.ours.delete(gd) { g.gd[gd].pinned = false; }
}

/// Once a frame before the guards move: each man under an order is walked to it and posed on it. (orders.ts SquadOrders.tick)
pub fn squad_orders_tick(g: &mut crate::game::Game) {
    squad_orders_sync(g);
    crate::fireline::fire_line_tick(g);
    if g.orders.asg.is_empty() { return; }
    if !squad_orders_active(g) { squad_orders_clear(g, None); return; }
    let sam = g.pl[g.player].ch;
    crate::sneak::sneak_tick(g);
    let mates: Vec<GuardId> = if squad_orders_big(g) { squad_orders_men(g, None, None) } else { Vec::new() };
    let mut file: Vec<(GuardId, OrdersAssignmentId)> = Vec::new();
    for (gd, &a) in g.orders.asg.iter() { if g.tab.orders_assignment[a].post.is_none() { file.push((gd, a)); } }
    js::sort_by(&mut file, |x, y| g.tab.orders_assignment[x.1].follow - g.tab.orders_assignment[y.1].follow);
    for (i, x) in file.iter().enumerate() { g.tab.orders_assignment[x.1].follow = i as f64; }
    let followers: Vec<GuardId> = file.iter().map(|x| x.0).collect();
    if !followers.is_empty() {
        let sp = g.ch[sam].pos;
        squad_orders_crumb(g, sp, &followers);
        let (sam_t, sam_p) = (g.orders.sam_t, g.orders.sam_p);
        let step = if sam_t >= 0.0 { flat(sp, sam_p) } else { 0.0 };
        let el = g.time - sam_t;
        let own = if sam_t >= 0.0 { crate::noshove::no_shove_own(g, sp[0] - sam_p[0], sp[2] - sam_p[2]) } else { None };
        if sam_t >= 0.0 && el > 1e-6 && step < 1.0 {   // (a jump of a metre is a teleport, not a pace)
            let ox = match own { Some(o) => o[0], None => sp[0] - sam_p[0] };
            let oz = match own { Some(o) => o[1], None => sp[2] - sam_p[2] };
            let st = if own.is_some() { m::hyp2(ox, oz) } else { step };
            g.orders.sam_v += (st / el - g.orders.sam_v) * 0.25;
            if st > 1e-3 { g.orders.sam_dir[0] = ox / st; g.orders.sam_dir[2] = oz / st; }
        }
        g.orders.sam_p[0] = sp[0]; g.orders.sam_p[2] = sp[2]; g.orders.sam_t = g.time;
    } else {
        g.orders.trail.clear(); g.orders.sam_t = -1.0; g.orders.sam_v = 0.0;
    }

    // a live walk of a Map: a man dropped here, or re-assigned by his order's own tick, is as the TypeScript's `for … of`
    let mut at = 0usize;
    while let Some(gd) = g.orders.asg.next_key(&mut at) {
        let a = *g.orders.asg.get(gd).unwrap();
        if g.gd[gd].state == crate::game::GuardState::Dead { squad_orders_drop(g, gd); continue; }
        let mut owned = false;   // a drill has taken him: its order stands, ours is dropped
        let mut oi = 0usize;
        while oi < g.orders.owners.len() { if owner_owns(g, oi, gd) { owned = true; break; } oi += 1; }
        if owned { squad_orders_drop(g, gd); continue; }
        let c = g.gd[gd].ch;
        // A.goal is a new object at what may be the old spot, which `order` calls a new leg (squad.ts:146) and a value cannot say: the order's first frame, or a line below
        let mut new_leg = g.tab.orders_assignment[a].t0 == g.time;
        if g.tab.orders_assignment[a].thru.is_some() {   // (ORDERS.ownDoor: he is in the room, by its door)
            if let Some(post) = g.tab.orders_assignment[a].post {
                let r = crate::cleared::cleared_rooms_room_at(g, post);
                if js::truthy(r) {
                    let here = g.ch[c].pos;
                    if r == crate::cleared::cleared_rooms_room_at(g, here) { g.tab.orders_assignment[a].thru = None; }
                }
            }
        }
        if g.gd[gd].script.is_none() && g.gd[gd].state == crate::game::GuardState::Alert {   // the fight AI has him, until his fight has gone cold
            if g.tab.orders_assignment[a].yield_t < 0.0 {
                g.tab.orders_assignment[a].yield_t = g.time;
            }
            if g.tab.orders_assignment[a].post.is_some() { g.gd[gd].pinned = true; g.orders.ours.add(gd); }
            let hot = g.gd[gd].reload_t >= 0.0
                || g.time - m::max(m::max(g.gd[gd].last_seen_t, g.gd[gd].witness.heard.last_shot_t), g.tab.orders_assignment[a].yield_t) < g.k.orders.cold_secs;
            if hot { continue; }
            g.gd[gd].state = crate::game::GuardState::Search;
        }
        if g.tab.orders_assignment[a].yield_t >= 0.0 {   // back on his order, by the way an order walks him, from wherever the fight left him
            let (now, here) = (g.time, g.ch[c].pos);
            {
                let aa = &mut g.tab.orders_assignment[a];
                aa.yield_t = -1.0; aa.arrived = false;
                aa.moved_t = now; aa.mark_t = now; aa.mark[0] = here[0]; aa.mark[2] = here[2];
            }
            g.m.orders.follow_word.delete(a);
            if let Some(post) = g.tab.orders_assignment[a].post {
                let (shy, thru) = (g.tab.orders_assignment[a].shy, g.tab.orders_assignment[a].thru);
                let r = squad_orders_route(g, gd, post, shy, thru, None);
                if r.why.is_none() {
                    g.tab.orders_assignment[a].via = r.via;
                    g.tab.orders_assignment[a].goal = match g.tab.orders_assignment[a].via.first() { Some(&q) => q, None => v3::copy(post) };
                    new_leg = true;
                    match r.how {
                        Some(SquadOrdersRouteHow::Round) => g.cleared.stats.round += 1.0,
                        Some(SquadOrdersRouteHow::Through) => g.cleared.stats.through += 1.0,
                        Some(SquadOrdersRouteHow::Mouth) => g.cleared.stats.mouth += 1.0,
                        None => {}
                    }
                }
            } else {
                g.tab.orders_assignment[a].via_t = None; g.tab.orders_assignment[a].via_for = None;
            }
        }
        if g.orders.ours.delete(gd) { g.gd[gd].pinned = false; }
        let order_id = g.tab.orders_assignment[a].order.clone();
        let def = g.orders.defs.iter().copied().find(|&x| g.tab.orders_order_def[x].id == order_id);
        if let Some(tick) = def.and_then(|x| g.tab.orders_order_def[x].tick.clone()) {
            def_tick(g, tick, gd, a);
            if g.orders.asg.get(gd).copied() != Some(a) { continue; }   // (it re-assigned him: his new order's turn is next frame)
        }
        let order_id = g.tab.orders_assignment[a].order.clone();   // (a row's tick may rewrite his Assignment)
        let mut speed = g.k.orders.brisk;
        let mut face = g.tab.orders_assignment[a].face;
        let mut d: f64;
        let mut s_goal: Option<V3> = None;
        let (mut low, mut stepping, mut keep, mut must) = (false, false, false, false);
        if g.tab.orders_assignment[a].post.is_none() {   // fall in: his place back along the trail
            let p = g.ch[c].pos;
            let sam_pos = g.ch[sam].pos;
            let to_sam = flat(p, sam_pos);
            let stopped = squad_orders_sam_stopped(g);
            let follow = g.tab.orders_assignment[a].follow;
            let back_m = squad_orders_back_of(g, follow) + if stopped { squad_orders_clear_back(g, follow) } else { 0.0 };
            let mut slot = squad_orders_slot(g, back_m);
            let on_trail = slot;
            let on_wall = if crate::sneak::sneaking(g, gd) || crate::rearwatch::room_kept(g, gd) { None } else { crate::unseen::unseen_wall_slot(g, gd, back_m, &followers) };
            if let Some(w) = on_wall { slot = w; }
            let ahead = squad_orders_ahead(g, gd, a);
            let off = crate::doorway::doorway_off_lane(g, slot, p, follow, Some(&ahead));
            if let Some(o) = off { slot = o; if on_wall.is_some() { crate::unseen::unseen_wall_lost(g, gd); } }
            let mut spread = false;
            if stopped && off.is_none() && on_wall.is_none() {   // (two places on one spot: his is dealt again)
                let (q, same) = apart_slot_kept(g, gd, a, slot);
                spread = !same; slot = q;
            }
            let aside = squad_orders_aside(g, gd);
            stepping = aside.is_some();
            let st0 = if stopped && aside.is_none() && !crate::sneak::sneaking(g, gd) && !crate::unseen::unseen_man(g, gd) { crate::cover::cover_step_of(g, gd, a) } else { None };
            // asked at most once a man a frame (the TypeScript's `nearV ??=`)
            let mut near_v: Option<bool> = None;
            let mut near_p = |g: &mut Game| -> bool {
                if let Some(v) = near_v { return v; }
                let v = aside.is_none() && on_wall.is_none() && stopped && to_sam > g.k.orders.near_m && to_sam < back_m + g.k.orders.band_m
                    && !squad_orders_in_frame(g, p)
                    && !(crate::doorway::doorway_lane(g, p, None).is_some() && (off.is_some() || crate::doorway::doorway_lane(g, slot, None).is_none()))
                    && !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [sam_pos[0], 0.45, sam_pos[2]], None)
                    && !(crate::doorway::doorway_file(g) && squad_orders_on_top(g, gd, a));
                near_v = Some(v);
                v
            };
            let ps = if st0.is_none() && stopped && aside.is_none() && on_wall.is_none() && !crate::sneak::sneaking(g, gd) && !crate::unseen::unseen_man(g, gd)
                && !crate::rearwatch::room_kept(g, gd) && !crate::roomfight::room_fight_short(g, gd)
            {
                let key = a.0;   // (the TypeScript's key is the Assignment by identity)
                let mark = if near_p(g) { p } else { slot };
                let busy = crate::roomfight::room_fight_busy(g, gd, 0.5);
                crate::picture::picture_step(g, gd, key, mark, Some(&mut |g: &mut Game, q: V3| !(squad_orders_in_frame(g, q) || crate::doorway::doorway_in_way(g, q) || crate::doorway::doorway_lane(g, q, None).is_some())), Some(busy))
            } else { None };
            let (ps_at, ps_low) = match &ps { Some(x) => (Some(x.at), x.low), None => (None, false) };
            let st = ps_at.or(st0);
            let near = st.is_none() && near_p(g);
            if let Some(s) = st {
                if flat(s, g.tab.orders_assignment[a].goal) > 0.05 { g.tab.orders_assignment[a].goal = [s[0], 0.0, s[2]]; }
                g.tab.orders_assignment[a].via.clear();
            } else if near {
                if flat(p, g.tab.orders_assignment[a].goal) > 0.4 { g.tab.orders_assignment[a].goal = [p[0], 0.0, p[2]]; }
                g.tab.orders_assignment[a].via.clear();
            } else {
                let mut want = aside.unwrap_or(slot);
                // (a follower a walk from his place does not cut through a room nobody has been in: round it, a corner at a time)
                if aside.is_none() {
                    let since = g.time - g.tab.orders_assignment[a].via_t.unwrap_or(-1e9);
                    let had = !g.tab.orders_assignment[a].via.is_empty();
                    let ask = if had {
                        match g.tab.orders_assignment[a].via_for { None => true, Some(f) => (flat(f, slot) > 2.5 && since > 0.5) || since > 3.0 }
                    } else {
                        since > 0.75 && (flat(p, slot) > 3.0 || !crate::scene_collision::nav_grid_walkable(g, p, slot))
                    };
                    if ask {
                        g.tab.orders_assignment[a].via_t = Some(g.time);
                        let via = squad_orders_round_rooms(g, gd, slot);
                        g.tab.orders_assignment[a].via = via;
                        g.tab.orders_assignment[a].via_for = Some([slot[0], 0.0, slot[2]]);
                        if !g.tab.orders_assignment[a].via.is_empty() && !had { g.cleared.stats.round += 1.0; }
                        if g.tab.orders_assignment[a].via.is_empty() && on_wall.is_none() {   // (no room to go round: the doors Sam took)
                            let j = squad_orders_slot_crumb(g, back_m);
                            let via = squad_orders_trail_way(g, gd, slot, on_trail, j);
                            g.tab.orders_assignment[a].via = via;
                        }
                    }
                    if !g.tab.orders_assignment[a].via.is_empty() && flat(p, g.tab.orders_assignment[a].via[0]) < 0.9 { g.tab.orders_assignment[a].via.remove(0); }
                    if !g.tab.orders_assignment[a].via.is_empty() { want = g.tab.orders_assignment[a].via[0]; }
                } else if !g.tab.orders_assignment[a].via.is_empty() { g.tab.orders_assignment[a].via.clear(); }
                let bar = if aside.is_some() { 0.25 } else if spread { 0.05 } else { 0.5 };
                if flat(want, g.tab.orders_assignment[a].goal) > bar { g.tab.orders_assignment[a].goal = want; }
            }
            if aside.is_none() && st.is_none() {   // (doorway.ts REST: he is not left standing in a frame or where a leaf swings)
                let place = if near { [p[0], 0.0, p[2]] } else { slot };
                squad_orders_rest_clear(g, gd, a, place, slot, stopped);
            }
            d = flat(p, g.tab.orders_assignment[a].goal);
            if !g.tab.orders_assignment[a].via.is_empty() && st.is_none() && !near && aside.is_none() {   // (his pace and 'in place' are by the whole walk left)
                let via = &g.tab.orders_assignment[a].via;
                let mut q = via[0];
                for i in 1..via.len() { d += flat(q, via[i]); q = via[i]; }
                d += flat(q, slot);
            }
            let sam_v = g.orders.sam_v;
            let (brisk, onto, walk, run, gain) = (g.k.orders.brisk, g.k.orders.onto, g.k.orders.walk, g.k.orders.run, g.k.orders.gain);
            speed = if aside.is_some() { brisk } else if st.is_some() { onto } else {
                match crate::teampace::team_follow(g, gd, sam_v, d, gain) {
                    Some(v) => v,
                    None => m::max(walk, m::min(run, sam_v + gain * (d - 0.3))),
                }
            };
            let sn = if aside.is_some() { None } else { crate::sneak::sneak_follow(g, gd, back_m, sam_v, stopped, speed) };
            let sn_must = match &sn { Some(x) => x.must.unwrap_or(false), None => false };
            if let Some(x) = &sn {
                must = sn_must; speed = x.speed; low = x.crouch;
                if let Some(q) = x.goal { s_goal = Some(q); d = flat(p, q); }
            } else {
                let un = if aside.is_some() { None } else { crate::unseen::unseen_follow(g, gd, sam_v, d, walk, speed) };
                if let Some(u) = &un { speed = u.speed; low = u.crouch; }
                else { low = crate::sneak::sneak_low(g, gd) || crate::unseen::unseen_low(g, gd); }
            }
            if ps_low && d < 0.5 { low = true; }
            keep = aside.is_none() && !sn_must && crate::rearwatch::room_kept(g, gd);
            if keep { speed = 0.0; }
            // (wallcharge.ts WALLCHARGE.menClear: the place he is sent to is inside a charge's blast: a stand-off mark instead. Asked last, of the place settled above)
            let wb = if aside.is_some() || sn_must { None } else {
                let (k0, sg0) = (keep, s_goal);
                crate::wallcharge::wall_charge_clear_of(g, gd, &mut |g: &mut Game| {
                    if k0 { g.ch[c].pos } else { match sg0.or(st) { Some(q) => q, None => if near { g.ch[c].pos } else { slot } } }
                })
            };
            if let Some(w) = wb {
                must = true;
                if flat(w, g.tab.orders_assignment[a].goal) > 0.05 { g.tab.orders_assignment[a].goal = [w[0], 0.0, w[2]]; }
                s_goal = None; keep = false; d = flat(p, w);
                low = crate::sneak::sneak_low(g, gd) || crate::unseen::unseen_low(g, gd);
                speed = if crate::sneak::sneaking(g, gd) || crate::unseen::unseen_man(g, gd) { if low { onto } else { walk } } else { brisk };
            }
            if aside.is_none() && !sn_must && crate::roomfight::room_fight_short(g, gd) { speed = 0.0; }
            // stopped: the first man looks on past Sam, the rest back the way they came
            let sam_pos = g.ch[sam].pos;
            face = if !stopped || d > 0.6 { None }
                else if to_sam < 0.3 { Some(g.tab.orders_assignment[a].face_now.unwrap_or(g.ch[sam].body_yaw)) }
                else if g.tab.orders_assignment[a].follow == 0.0 { Some(yaw_to(p, sam_pos)) }
                else { Some(yaw_to(sam_pos, p)) };
            g.tab.orders_assignment[a].arrived = d < 0.8;
            squad_orders_follow_word(g, a, d);
        } else {
            let p = g.ch[c].pos;
            let post = g.tab.orders_assignment[a].post.unwrap();
            // (cover.ts COVER.step: a plain holder is walked to where he sees through an open door; his MARK stays)
            let plain = order_id == "hold" && g.tab.orders_assignment[a].watch.is_none() && !g.tab.orders_assignment[a].spare.unwrap_or(false) && g.tab.orders_assignment[a].via.is_empty();
            let st = if plain { crate::cover::cover_step_of(g, gd, a) } else { None };
            let mark = st.unwrap_or(post);
            if plain && flat(g.tab.orders_assignment[a].goal, mark) > 0.05 {
                let now = g.time;
                let aa = &mut g.tab.orders_assignment[a];
                aa.goal = [mark[0], 0.0, mark[2]]; aa.mark_t = now; aa.moved_t = now; aa.mark[0] = p[0]; aa.mark[2] = p[2];
            }
            if !g.tab.orders_assignment[a].via.is_empty() {   // (the way round a locked door, a waypoint at a time)
                let aa = &mut g.tab.orders_assignment[a];
                if flat(p, aa.via[0]) < 0.9 { aa.via.remove(0); }
                aa.goal = match aa.via.first() { Some(&q) => q, None => { new_leg = true; v3::copy(post) } };
            }
            d = flat(p, mark);
            let pace = g.tab.orders_assignment[a].pace;
            let (k_brisk, k_brisk_in, k_onto, k_stuck) = (g.k.orders.brisk, g.k.orders.brisk_in, g.k.orders.onto, g.k.orders.stuck_secs);
            speed = match pace {
                Some(v) => v,
                None => if d > k_brisk_in { k_brisk } else { crate::stackline::stack_line_pace(g, gd, a, d, k_brisk).unwrap_or(k_onto) },
            };
            if pace.is_some() { crate::teampace::team_fight(g, gd, None); }
            else if g.tab.orders_assignment[a].thru.is_none() && g.tab.orders_assignment[a].via.is_empty() {
                let is_stack = order_id == "stack" && g.tab.orders_assignment[a].door.is_some();
                speed = crate::teampace::team_dash(g, gd, mark, d, Some(is_stack)).unwrap_or(speed);
            }
            if pace.is_none() && d > k_brisk_in { speed = crate::teampace::team_told(g, gd).unwrap_or(speed); }
            {
                let sp = match crate::sneak::sneak_post(g, gd, &order_id) { Some(v) => Some(v), None => crate::unseen::unseen_post(g, gd, &order_id) };
                if let Some(v) = sp { low = true; speed = m::min(speed, v); }
            }
            let short = crate::roomfight::room_fight_short(g, gd);
            if short { speed = 0.0; mark_clocks(g, a, p); }
            else if (crate::doorway::doorway_held(g, gd) || crate::handleside::handle_held(g, gd)) && squad_orders_held_wait(g, a) { mark_clocks(g, a, p); }   // (doorway.ts SHORT: a wait, not a shut way)
            if crate::clearline::clear_line_live(g, gd) || crate::fireline::fire_line_live(g, gd) { mark_clocks(g, a, p); }   // (off his mark on purpose)
            // has he MOVED (his commanded speed says nothing)
            let now = g.time;
            if flat(p, g.tab.orders_assignment[a].anchor) > 0.05 {
                let aa = &mut g.tab.orders_assignment[a];
                aa.anchor[0] = p[0]; aa.anchor[2] = p[2]; aa.moved_t = now;
            }
            let mut stuck = false;
            if now - g.tab.orders_assignment[a].mark_t >= k_stuck {
                let aa = &mut g.tab.orders_assignment[a];
                stuck = flat(p, aa.mark) < 0.3;
                aa.mark[0] = p[0]; aa.mark[2] = p[2]; aa.mark_t = now;
            }
            let waited = if d < 1.0 && g.tab.orders_assignment[a].via.is_empty() {
                g.time - m::max(g.tab.orders_assignment[a].moved_t, g.tab.orders_assignment[a].t0) > 1.0
            } else { stuck };
            if d >= 0.35 && waited && !(st.is_none() && squad_orders_rest_short(g, gd, a, d)) {   // as near his mark as he will get: the mark comes to him
                if st.is_some() { crate::cover::cover_step_halt(g, gd); }   // (a step nobody ordered: it ends where he stands, and he says nothing)
                else if d < 1.0 && g.tab.orders_assignment[a].had == g.tab.orders_assignment[a].post
                    && g.tab.orders_assignment[a].back_n.unwrap_or(0.0) < BACK_TRIES && g.tab.orders_assignment[a].via.is_empty()
                {   // (doorway.ts BACK ON: he HAD this mark and was shouldered off it: he steps back onto it)
                    let cur = g.tab.orders_assignment[a].post.unwrap();
                    let aa = &mut g.tab.orders_assignment[a];
                    aa.back_n = Some(aa.back_n.unwrap_or(0.0) + 1.0);
                    aa.goal = v3::copy(cur); aa.moved_t = now;
                    new_leg = true;
                    g.m.doorway.stack_own.back += 1.0;
                } else {
                    if d >= 1.0 {
                        g.tab.orders_assignment[a].there = String::from("stopped short");
                        crate::game::game_say(g, gd, "can't get through.", None, None, None);
                    }
                    let np = [p[0], 0.0, p[2]];
                    let aa = &mut g.tab.orders_assignment[a];
                    aa.post = Some(np); aa.goal = v3::copy(np); aa.via.clear();
                    new_leg = true;
                    d = 0.0;
                }
            }
            if st.is_none() && d < 0.35 {
                let aa = &mut g.tab.orders_assignment[a];
                aa.had = aa.post;
                if d < 0.1 { aa.back_n = Some(0.0); }
            }
            let was = g.tab.orders_assignment[a].arrived;
            let mut arrived = d < 0.35;
            if arrived {
                let (w, fit, spare) = (g.tab.orders_assignment[a].watch, g.tab.orders_assignment[a].fit.is_some(), g.tab.orders_assignment[a].spare.unwrap_or(false));
                arrived = match w {
                    None => true,
                    Some(w) => fit || (spare && crate::cover::cover_has(g, gd)) || m::abs(v3::wrap_angle(g.ch[c].aim_yaw - yaw_to(g.ch[c].pos, w))) < 0.2,
                };
            }
            g.tab.orders_assignment[a].arrived = arrived;
            let held_fire = d < 0.35 && order_id == "stack" && crate::holdfire::hold_fire_on_man(g, gd);
            g.tab.orders_assignment[a].held_fire = Some(held_fire);
            if held_fire { g.tab.orders_assignment[a].arrived = true; }
            let held_set = !g.tab.orders_assignment[a].arrived && d < 0.35 && order_id == "stack" && crate::silent::silent_on_man(g, gd);
            g.tab.orders_assignment[a].held_set = Some(held_set);
            if held_set { g.tab.orders_assignment[a].arrived = true; }
            if g.tab.orders_assignment[a].arrived && !was { g.tab.orders_assignment[a].arr_t = g.time; }
        }
        // on the way to a mark: the gun up and leading; the mark's own carriage as he settles onto it
        let (mut pose, mut pose_is_move) = match g.tab.orders_assignment[a].move_ {
            Some(mv) if d > 0.6 => ((mv.stance, mv.upper), true),
            _ => ((g.tab.orders_assignment[a].stance, g.tab.orders_assignment[a].upper), false),
        };
        // UNASKED COVER (cover.ts): a man following, or holding with nothing named to watch, has an opening dealt him; his feet stay the order's
        let mut watch = g.tab.orders_assignment[a].watch;
        let posted = g.tab.orders_assignment[a].post.is_some();
        let mut strafe = g.tab.orders_assignment[a].strafe.unwrap_or(false) && posted && face.is_some();
        let spare = g.tab.orders_assignment[a].spare.unwrap_or(false);
        let cov = if (g.tab.orders_assignment[a].watch.is_none() && (!posted || order_id == "hold")) || spare {
            let still = if spare { d < 0.6 } else { posted || face.is_some() || keep };
            crate::cover::cover_pose_for(g, gd, still, speed)
        } else { None };
        if let Some(x) = &cov { watch = Some(x.at); face = x.face; strafe = x.strafe; pose = AIM; pose_is_move = false; }
        let sp = crate::spotted::spotted_feet(g, gd, !posted, stepping);
        let (sp_goal, sp_speed, sp_crouch, sp_watch) = match &sp { Some(x) => (x.goal, x.speed, x.crouch, x.watch), None => (None, None, false, None) };
        if let Some(w) = sp_watch {
            if !crate::pairorders::pair_orders_finishing(g, gd) { watch = Some(w); face = Some(yaw_to(g.ch[c].pos, w)); strafe = false; pose = AIM; pose_is_move = false; }
        }
        // (teampace.ts TEAMPACE.walkAim: no line to the order's own watch: the gun leads the way he walks. `pose === A.move` is pose_is_move, and with it `watch === A.watch`)
        if pose_is_move {
            if let Some(w) = watch {
                if let Some(ahead) = crate::teampace::walk_lead(g, gd, w) { watch = Some(ahead); strafe = false; }
            }
        }
        // (doorway.ts DOORMANNERS.watch: a back-pedal has no stuck watchdog: his next corner behind a wall and the way is found again; a second without moving and he walks it the plain way for three)
        if strafe && posted && d > 0.6 {
            let q = g.ch[c].pos;
            let wp = g.gd[gd].path.get(g.gd[gd].path_i).copied();
            let t = g.time;
            let mut w = match g.orders.back_fix.get(a) { Some(x) => *x, None => SquadOrdersBackFixValue { plain: -1.0, path: -1.0 } };
            if let Some(wp) = wp {
                if t - w.path > 1.0 && crate::scene_collision::solid_blocked(g, [q[0], 0.45, q[2]], [wp[0], 0.45, wp[2]], None) {
                    g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; w.path = t;
                }
            }
            if t - g.tab.orders_assignment[a].moved_t > 1.0 { w.plain = t + 3.0; }
            g.orders.back_fix.set(a, w);
            if w.plain > t { strafe = false; face = None; }
        }
        g.tab.orders_assignment[a].face_now = face;
        let mut cl = crate::clearline::clear_line_feet(g, gd);
        if cl.is_some() && !posted && squad_orders_aside(g, gd).is_some() { cl = None; }   // (stepping out of Sam's own way still comes first)
        if cl.is_some() && sp.is_some() { cl = None; }   // (…and so does the silent partner's rule)
        let (cl_goal, cl_speed, cl_crouch) = match &cl { Some(x) => (Some(x.goal), Some(x.speed), x.crouch), None => (None, None, false) };
        if let Some(x) = &cl { face = x.face; strafe = x.strafe; }
        let own_goal = if keep { None } else { Some(s_goal.unwrap_or(g.tab.orders_assignment[a].goal)) };
        let fl = if sp.is_some() || cl.is_some() { None }   // (`sp?.speed !== undefined`: every answer of spottedFeet has the key, null or a number)
             else { crate::fireline::fire_line_feet(g, gd, own_goal, speed, stepping || must) };
        let (fl_goal, fl_speed) = match &fl { Some(x) => (x.goal, Some(x.speed)), None => (None, None) };
        if pose.1 == crate::character::UpperMode::Aim || pose.0 == crate::character::Stance::Stack {   // Sam in the line of his gun: the muzzle comes down until he is clear of it
            let p = g.ch[c].pos;
            let yaw = match watch { Some(w) => yaw_to(p, w), None => g.ch[c].body_yaw };
            let (fx, fz) = (m::sin(yaw), m::cos(yaw));
            let len = match watch { Some(w) => flat(p, w), None => 5.0 };
            let lane = if g.tab.orders_assignment[a].low { g.k.orders.lane_out } else { g.k.orders.lane_in };
            let in_line = |q: V3| -> bool {
                let (sx, sz) = (q[0] - p[0], q[2] - p[2]);
                let along = sx * fx + sz * fz;
                along > 0.15 && along < len && m::abs(sx * fz - sz * fx) < lane
            };
            let mut low_now = squad_orders_sams(g) && in_line(g.ch[sam].pos);
            if !low_now && g.tab.orders_assignment[a].arrived && LANE_ORDERS.contains(&order_id.as_str()) {
                for &o in mates.iter() {   // (… a man of the OTHER pair, across a gun that is posted)
                    if o != gd && squad_orders_pair_of(g, o) != squad_orders_pair_of(g, gd) && in_line(g.ch[g.gd[o].ch].pos) { low_now = true; break; }
                }
            }
            g.tab.orders_assignment[a].low = low_now;
            if low_now { pose = LOW; }
        } else { g.tab.orders_assignment[a].low = false; }
        // (who has the follower's feet: spotted.ts, clearline.ts, fireline.ts, else the sneak's step, else the order's own goal)
        let goal = sp_goal.or(cl_goal).or(fl_goal).or(own_goal);
        let speed_out = sp_speed.or(cl_speed).or(fl_speed).unwrap_or(speed);
        let crouch = g.tab.orders_assignment[a].crouch.unwrap_or(false) || low || sp_crouch || cl_crouch;
        // (the lean is written only where one is asked or one is to be taken off: a script patch keeps what it is not told. He leans on his mark, not on the walk to it)
        let a_lean = g.tab.orders_assignment[a].lean.unwrap_or(0.0);
        let script_lean = match &g.gd[gd].script { Some(s) => s.lean.unwrap_or(0.0), None => 0.0 };
        let lean = if js::truthy(a_lean) || js::truthy(script_lean) {
            let on_mark = match g.tab.orders_assignment[a].post { Some(q) => flat(g.ch[c].pos, q) < 0.6, None => false };
            Some(if js::truthy(a_lean) && on_mark { a_lean } else { 0.0 })
        } else { None };
        let (stack_side, light) = (g.tab.orders_assignment[a].stack_side, g.tab.orders_assignment[a].light);
        // (held: everything this call passes is a kept point, compared by value; A.goal made anew this frame is the one exception: see new_leg)
        let own_passed = sp_goal.is_none() && cl_goal.is_none() && fl_goal.is_none() && s_goal.is_none() && !keep;
        crate::squad::order(g, gd, crate::squad::OrderPatch {
            goal: Some(goal), speed: Some(Some(speed_out)), face: Some(face), strafe: Some(Some(strafe)), aim_at: Some(watch),
            stance: Some(Some(pose.0)), upper: Some(Some(pose.1)), stack_side: Some(Some(stack_side)), crouch: Some(Some(crouch)),
            hot: Some(Some(false)), light: Some(light), lean: lean.map(Some),
            held: !(new_leg && own_passed),
            ..Default::default()
        });
        if let Some(q) = s_goal { if sp_goal.is_none() && cl_goal.is_none() && fl_goal.is_none() { crate::sneak::sneak_way(g, gd, q); } }
    }
    // 'set.': once, from the last man on, when everybody the order sent is on his mark
    let mut at = 0usize;
    while let Some(k) = g.orders.asg.next_key(&mut at) {
        let a = *g.orders.asg.get(k).unwrap();
        let line = match &g.tab.orders_assignment[a].set_line { Some(s) if !s.is_empty() => s.clone(), _ => continue };
        if g.tab.orders_assignment[a].set_said || !g.tab.orders_assignment[a].arrived { continue; }
        let group = g.tab.orders_assignment[a].group;
        let (mut all, mut last, mut last_t) = (true, None, f64::NEG_INFINITY);
        for (o, &b) in g.orders.asg.iter() {
            let bb = &g.tab.orders_assignment[b];
            if bb.group == group {
                if !bb.arrived || bb.tries.is_some() { all = false; break; }   // (a man of them still has a door to try)
                if bb.arr_t > last_t { last_t = bb.arr_t; last = Some(o); }
            }
        }
        if let (true, Some(last)) = (all, last) {
            // (one of them said "can't get through": the group is not where it was sent, and nobody calls it set)
            let mut short = false;
            for &b in g.orders.asg.values() { let bb = &g.tab.orders_assignment[b]; if bb.group == group && bb.there == "stopped short" { short = true; break; } }
            if !short { crate::game::game_say(g, last, &line, None, None, None); }
            for (_, &b) in g.orders.asg.iter() { if g.tab.orders_assignment[b].group == group { g.tab.orders_assignment[b].set_said = true; } }
        }
    }
}

/// The clocks that would call a standing man stuck start again. (the TypeScript's repeated `A.movedT = A.markT = g.time; A.mark[0] = p[0]; A.mark[2] = p[2]`)
fn mark_clocks(g: &mut Game, a: OrdersAssignmentId, p: V3) {
    let now = g.time;
    let aa = &mut g.tab.orders_assignment[a];
    aa.moved_t = now; aa.mark_t = now; aa.mark[0] = p[0]; aa.mark[2] = p[2];
}

/// The spot he was already stepping to, while Sam has not got past him. (orders.ts aside's `held`)
fn aside_held(g: &mut Game, gd: GuardId) -> Option<V3> {
    let (sam, p) = (g.ch[g.pl[g.player].ch].pos, g.ch[g.gd[gd].ch].pos);
    let row = match g.orders.aside_mem.get(gd) { Some(x) => x, None => return None };
    if g.time - row.t > g.k.orders.aside_hold || g.time < row.t { return None; }
    if (p[0] - sam[0]) * row.dir[0] + (p[2] - sam[2]) * row.dir[2] > -0.3 && flat(p, sam) < g.k.orders.aside_m + 0.5 { Some(row.q) } else { None }
}

/// Where a follower stands to let Sam through: Sam walking AT him; None: not in the way, or no clear spot. (orders.ts SquadOrders.aside)
pub fn squad_orders_aside(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> {
    let (sam, p) = (g.ch[g.pl[g.player].ch].pos, g.ch[g.gd[gd].ch].pos);
    if g.orders.sam_v < 0.3 { return aside_held(g, gd); }
    let dir = g.orders.sam_dir;
    let (mx, mz) = (p[0] - sam[0], p[2] - sam[2]);
    let ahead = mx * dir[0] + mz * dir[2];
    let off = mx * dir[2] - mz * dir[0];
    if ahead <= 0.0 || ahead > g.k.orders.aside_m || m::abs(off) > g.k.orders.aside_lane { return aside_held(g, gd); }
    // (ORDERS.asideClear: the side he is already stepping to stays his first side)
    let own = if off >= 0.0 { 1.0 } else { -1.0 };
    let first = match g.orders.aside_mem.get(gd) {
        Some(was) if was.side.map_or(false, js::truthy) && g.time >= was.t && g.time - was.t <= g.k.orders.aside_hold => was.side.unwrap(),
        _ => own,
    };
    for w in [g.k.orders.aside_off, 0.65] {
        for side in [first, -first] {
            let q: V3 = [sam[0] + dir[0] * ahead + dir[2] * side * w, 0.0, sam[2] + dir[2] * ahead - dir[0] * side * w];
            if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
                && !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [q[0], 0.45, q[2]], None)
                && !(squad_orders_in_frame(g, q) || crate::doorway::doorway_in_way(g, q))
            {
                let t = g.time;
                g.orders.aside_mem.set(gd, SquadOrdersAsideMemValue { q, t, dir: [dir[0], 0.0, dir[2]], side: Some(side) });
                return Some(q);
            }
        }
    }
    aside_held(g, gd)
}

/// Sam's trail: a crumb every `crumb` metres he has moved, `trailM` metres of it kept, newest last. (orders.ts SquadOrders.crumb)
pub fn squad_orders_crumb(g: &mut crate::game::Game, p: V3, men: &[GuardId]) {
    if let Some(&last) = g.orders.trail.last() { if flat(last, p) > 3.0 { g.orders.trail.clear(); } }   // (a teleport)
    if g.orders.trail.is_empty() {   // no trail yet: 'behind him' is the side his men are on
        let (mut cx, mut cz) = (0.0, 0.0);
        for &gd in men { let q = g.ch[g.gd[gd].ch].pos; cx += q[0] - p[0]; cz += q[2] - p[2]; }
        let l = m::hyp2(cx, cz);
        let yaw = g.ch[g.pl[g.player].ch].body_yaw;
        g.orders.back = if l > 0.3 { [cx / l, 0.0, cz / l] } else { [-m::sin(yaw), 0.0, -m::cos(yaw)] };
    }
    let k_crumb = g.k.orders.crumb;
    let k_trail = g.k.orders.trail_m;
    let t = &mut g.orders.trail;
    if t.is_empty() || flat(t[t.len() - 1], p) >= k_crumb { t.push([p[0], 0.0, p[2]]); }
    let mut len = 0.0;
    for i in (1..t.len()).rev() {
        len += flat(t[i], t[i - 1]);
        if len > k_trail { t.drain(0..i - 1); break; }
    }
}

/// Where the men ahead of him in the fall-in file are bound. (orders.ts SquadOrders.ahead)
pub fn squad_orders_ahead(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) -> Vec<V3> {
    let mut out: Vec<V3> = Vec::new();
    let follow = g.tab.orders_assignment[a].follow;
    for (o, &b) in g.orders.asg.iter() {
        let bb = &g.tab.orders_assignment[b];
        if o != gd && bb.post.is_none() && bb.follow < follow && g.gd[o].state != crate::game::GuardState::Dead { out.push(bb.goal); }
    }
    out
}

/// doorway.ts STAND: stopped 'near enough' within a body and a bit of a man ahead of him, or one on a post. (orders.ts SquadOrders.onTop)
pub fn squad_orders_on_top(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) -> bool {
    let follow = g.tab.orders_assignment[a].follow;
    let (mine, apart) = (g.ch[g.gd[gd].ch].pos, g.k.doormanners.apart);
    for (o, &b) in g.orders.asg.iter() {
        let bb = &g.tab.orders_assignment[b];
        if o != gd && g.gd[o].state != crate::game::GuardState::Dead && (bb.post.is_some() || bb.follow < follow) && flat(g.ch[g.gd[o].ch].pos, mine) < apart { return true; }
    }
    false
}

/// The open doorway `p` stands in, for a place in the file. (orders.ts SquadOrders.inFrame)
pub fn squad_orders_in_frame(g: &mut crate::game::Game, p: V3) -> bool {
    if crate::doorway::doorway_stand(g, p) { return true; }   // (doorway.ts STAND)
    let k = g.k.orders.off_door;
    if !(k > 0.0) { return false; }
    let mut i = 0usize;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        if crate::doors::door_latched(g, d) || m::abs(crate::doors::door_angle(g, d)) < 0.6 || flat(p, g.dr[d].frame_centre) > g.dr[d].def.width + k { continue; }
        let f = crate::guards::frame_of(g, d);
        let along = (p[0] - f.hx) * f.ux + (p[2] - f.hz) * f.uz;
        let off = (p[0] - f.hx) * f.nx + (p[2] - f.hz) * f.nz;
        if along > -0.05 && along < f.w + 0.05 && m::abs(off) < k { return true; }
    }
    false
}

/// (orders.ts restClear's `bad`)
fn rest_bad(g: &mut Game, q: V3) -> bool { squad_orders_in_frame(g, q) || crate::doorway::doorway_in_way(g, q) || crate::doorway::doorway_lane(g, q, None).is_some() }
/// (orders.ts restClear's `others`)
fn rest_others(g: &mut Game, gd: GuardId) -> Vec<V3> {
    let mut out: Vec<V3> = vec![squad_orders_sam_stood(g)];
    for (o, &b) in g.orders.asg.iter() {
        if o != gd && g.gd[o].state != crate::game::GuardState::Dead { out.push(g.ch[g.gd[o].ch].pos); out.push(g.tab.orders_assignment[b].goal); }
    }
    out
}

/// doorway.ts REST: a follower is not left standing in a doorway or where a leaf swings. (orders.ts SquadOrders.restClear)
pub fn squad_orders_rest_clear(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, place: V3, trail: V3, stopped: bool) {
    let (p, t) = (g.ch[g.gd[gd].ch].pos, g.time);
    let apart = g.k.doormanners.apart;
    if !g.orders.rest_of.has(a) {
        g.orders.rest_of.set(a, SquadOrdersRestOfValue { x: p[0], z: p[2], t, step: None, until: 0.0, from: [0.0, 0.0, 0.0], n: 0.0, hold: None, hold_for: [0.0, 0.0, 0.0] });
    }
    {
        let r = g.orders.rest_of.get_mut(a).unwrap();
        if m::hyp2(p[0] - r.x, p[2] - r.z) > 0.2 { r.x = p[0]; r.z = p[2]; r.t = t; }
    }
    if !g.tab.orders_assignment[a].via.is_empty() {
        let r = g.orders.rest_of.get_mut(a).unwrap();
        r.step = None; r.hold = None;
        return;
    }
    {
        let r = g.orders.rest_of.get_mut(a).unwrap();
        if let Some(step) = r.step {
            if t > r.until || flat(trail, r.from) > 1.0 { r.step = None; r.t = t; }
            else { g.tab.orders_assignment[a].goal = step; return; }   // (the same point every frame: one leg)
        }
        if js::truthy(r.n) && flat(trail, r.from) > 1.0 { r.n = 0.0; }
        if !stopped || flat(trail, r.hold_for) > 0.5 { r.hold = None; }
    }
    let goal = g.tab.orders_assignment[a].goal;
    if stopped && (squad_orders_in_frame(g, goal) || crate::doorway::doorway_in_way(g, goal)) {
        if let Some(h) = g.orders.rest_of.get(a).unwrap().hold { g.tab.orders_assignment[a].goal = h; return; }   // (dealt once for where the file stands)
        let others = rest_others(g, gd);
        let q = if !rest_bad(g, place) && others.iter().all(|&o| flat(o, place) >= apart) { Some(place) }
            else { crate::doorway::doorway_clear_near(g, goal, &mut |g: &mut Game, x: V3| rest_bad(g, x), Some(&others), Some(apart), Some(p)) };
        if let Some(q) = q {
            g.tab.orders_assignment[a].goal = q;
            let r = g.orders.rest_of.get_mut(a).unwrap();
            r.hold = Some(q); r.hold_for = [trail[0], 0.0, trail[2]];
            g.stats.doorway.rest += 1;
        }
        return;
    }
    if crate::doorway::doorway_held(g, gd) { g.orders.rest_of.get_mut(a).unwrap().t = t; return; }   // (short of a frame for a man in it)
    let (rt, rn) = { let r = g.orders.rest_of.get(a).unwrap(); (r.t, r.n) };
    if t - rt >= g.k.doormanners.rest_secs * (1.0 + rn) && flat(p, goal) < 1.0 && crate::doorway::doorway_in_way(g, p) {   // (at rest by his place)
        let others = rest_others(g, gd);
        let sam = g.ch[g.pl[g.player].ch].pos;
        let q = crate::doorway::doorway_clear_near(g, p, &mut |g: &mut Game, x: V3| rest_bad(g, x), Some(&others), Some(apart), Some(sam));
        if let Some(q) = q {
            {
                let r = g.orders.rest_of.get_mut(a).unwrap();
                r.step = Some(q); r.until = t + 4.0; r.from = [trail[0], 0.0, trail[2]]; r.n = m::min(r.n + 1.0, 4.0);
            }
            g.tab.orders_assignment[a].goal = q;
            g.stats.doorway.rest += 1;
        } else { g.orders.rest_of.get_mut(a).unwrap().t = t; }
    }
}

/// doorway.ts SHORT: the seconds of one order's walk the PASS gate's holds may keep its stuck clocks from running. (orders.ts SquadOrders.heldWait)
pub fn squad_orders_held_wait(g: &mut crate::game::Game, a: OrdersAssignmentId) -> bool {
    let t = g.time;
    if !g.orders.held_of.has(a) { g.orders.held_of.set(a, SquadOrdersHeldOfValue { last: t, total: 0.0 }); }
    let cap = g.k.doormanners.held_cap;
    let h = g.orders.held_of.get_mut(a).unwrap();
    h.total += m::min(m::max(t - h.last, 0.0), 0.1);
    h.last = t;
    h.total <= cap
}

/// doorway.ts SHORT: as near his mark as he will get: his mark does not come to him while he stands in a doorway's way. True = dealt with here. (orders.ts SquadOrders.restShort)
pub fn squad_orders_rest_short(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, d: f64) -> bool {
    let p = g.ch[g.gd[gd].ch].pos;
    let post = match g.tab.orders_assignment[a].post { Some(q) => q, None => return false };
    if d < 1.0 || !crate::doorway::doorway_in_way(g, p) { return false; }   // (a stride from his mark he has as good as made it)
    if !g.orders.short_of.has(a) { g.orders.short_of.set(a, SquadOrdersShortOfValue { x: p[0], z: p[2], n: 0.0, k: 0.0 }); }
    let first = {
        let r = g.orders.short_of.get_mut(a).unwrap();
        if r.k >= 2.0 { return false; }   // (his mark has been moved for him twice on this order: the old rule ends it)
        if m::hyp2(p[0] - r.x, p[2] - r.z) > 1.0 { r.n = 0.0; }
        r.x = p[0]; r.z = p[2];
        let n = r.n; r.n += 1.0;
        n == 0.0
    };
    if first {
        let now = g.time;
        let aa = &mut g.tab.orders_assignment[a];
        aa.mark_t = now; aa.moved_t = now;
        return true;
    }
    let mut taken: Vec<V3> = vec![squad_orders_sam_stood(g)];
    for (o, &b) in g.orders.asg.iter() {
        if o != gd && g.gd[o].state != crate::game::GuardState::Dead {
            taken.push(g.ch[g.gd[o].ch].pos);
            if let Some(q) = g.tab.orders_assignment[b].post { taken.push(q); }
        }
    }
    let sam = squad_orders_sam_stood(g);
    let face: V3 = if flat(sam, p) < 1.3 { [2.0 * p[0] - sam[0], 0.0, 2.0 * p[2] - sam[2]] } else { post };   // (in a frame Sam stands in the mouth of: back off it, away from him)
    let apart = g.k.doormanners.apart;
    let q = crate::doorway::doorway_clear_near(g, p, &mut |g: &mut Game, x: V3| squad_orders_in_frame(g, x) || crate::doorway::doorway_in_way(g, x), Some(&taken), Some(apart), Some(face));
    let q = match q { Some(q) => q, None => return false };
    g.orders.short_of.get_mut(a).unwrap().k += 1.0;
    g.tab.orders_assignment[a].there = String::from("stopped short");
    crate::game::game_say(g, gd, "can't get through.", None, None, None);
    let now = g.time;
    let aa = &mut g.tab.orders_assignment[a];
    aa.post = Some(q); aa.goal = v3::copy(q); aa.via.clear(); aa.mark_t = now; aa.moved_t = now;
    g.stats.doorway.rest += 1;
    true
}

/// Sam stopped: how much further back along the trail the `follow`-th place stands so that it and every place ahead of it is clear of a doorway. (orders.ts SquadOrders.clearBack)
pub fn squad_orders_clear_back(g: &mut crate::game::Game, follow: f64) -> f64 {
    if !(g.k.orders.off_door > 0.0) { return 0.0; }
    if g.orders.clear_t != g.time { g.orders.clear_t = g.time; g.orders.clear_by.clear(); }
    let mut i = g.orders.clear_by.len();
    while (i as f64) <= follow {
        let mut e = if i > 0 { g.orders.clear_by[i - 1] } else { 0.0 };
        let base = squad_orders_back_of(g, i as f64);
        for _ in 0..8 {
            let q = squad_orders_slot(g, base + e);
            if !squad_orders_in_frame(g, q) { break; }
            let further = squad_orders_slot(g, base + e + 0.3);
            if flat(q, further) < 0.01 { break; }   // (ORDERS.backEnd: the trail and the floor behind it have run out)
            e += 0.3;
        }
        g.orders.clear_by.push(e);
        i += 1;
    }
    g.orders.clear_by[follow as usize]
}

/// A follower's place in the file, in metres back along the trail. (orders.ts SquadOrders.backOf)
pub fn squad_orders_back_of(g: &mut crate::game::Game, follow: f64) -> f64 {
    let list = &g.k.orders.follow_back;
    let n = list.len() as f64;
    list[m::min(follow, n - 1.0) as usize] + m::max(0.0, follow - n + 1.0) * 1.1
}

/// The way they came: the point `extra` metres further back along Sam's trail than this follower's own place. None: he is not following. (orders.ts SquadOrders.trailBack)
pub fn squad_orders_trail_back(g: &mut crate::game::Game, gd: GuardId, extra: f64) -> Option<V3> {
    let a = *g.orders.asg.get(gd)?;
    if g.tab.orders_assignment[a].post.is_some() { return None; }
    let follow = g.tab.orders_assignment[a].follow;
    let back = squad_orders_back_of(g, follow) + extra;
    Some(squad_orders_slot(g, back))
}

/// (orders.ts apartSlot's `open`)
fn apart_open(g: &mut Game, a: V3, b: V3) -> bool { !crate::scene_collision::solid_blocked(g, [a[0], 0.45, a[2]], [b[0], 0.45, b[2]], None) }
/// (orders.ts apartSlot's `bad`)
fn apart_bad(g: &mut Game, x: V3) -> bool { squad_orders_in_frame(g, x) || crate::doorway::doorway_lane(g, x, None).is_some() }
/// (orders.ts apartSlot's `clear`)
fn apart_clear(t: &[V3], x: V3, by: f64) -> bool { t.iter().all(|&o| flat(o, x) >= by - 0.05) }
/// (orders.ts apartSlot's `short`)
fn apart_short(g: &mut Game, p: V3, slot: V3, t: &[V3], k: f64) -> Option<V3> {
    if flat(p, slot) < 1.5 && !apart_bad(g, p) && apart_clear(t, p, k) && apart_open(g, p, slot) { Some([p[0], 0.0, p[2]]) } else { None }
}
/// (orders.ts apartSlot's `hold`, ORDERS.fileHold.) The bool: it is `slot` itself, what the tick's `q !== slot` asks.
fn apart_hold(g: &mut Game, p: V3, sam: V3, slot: V3, t: &[V3], k: f64) -> (V3, bool) {
    if flat(p, slot) >= 1.5 || apart_bad(g, p) || !apart_open(g, p, slot) { return (slot, true); }
    let here: V3 = [p[0], 0.0, p[2]];
    let by = m::min(k, g.k.orders.file_tight);
    let mut near: Option<usize> = None;   // (an index: `o === t` below is the identity of an entry)
    for (i, &o) in t.iter().enumerate() {
        if flat(o, p) < by - 0.05 && apart_open(g, o, p) && (match near { None => true, Some(n) => flat(o, p) < flat(t[n], p) }) { near = Some(i); }
    }
    let ti = match near { Some(n) => n, None => return (here, false) };
    let to = t[ti];
    let d = flat(to, p);
    if d < 1e-3 { return (here, false); }
    let mut r = by;
    while r > d + 0.02 {
        for ang in [0.0, 0.4, -0.4, 0.8, -0.8] {   // (straight away first, then a little either side of it)
            let (c, sn) = (m::cos(ang), m::sin(ang));
            let (ux, uz) = ((p[0] - to[0]) / d, (p[2] - to[2]) / d);
            let x: V3 = [to[0] + (ux * c + uz * sn) * r, 0.0, to[2] + (uz * c - ux * sn) * r];
            if !apart_bad(g, x) && !crate::scene_collision::nav_grid_is_blocked(g, x[0], x[2]) && apart_open(g, p, x)
                && t.iter().enumerate().all(|(i, &o)| i == ti || flat(o, x) >= by - 0.05) && flat(sam, x) >= by - 0.05
            {
                return (x, false);
            }
        }
        r -= 0.05;
    }
    (here, false)
}

/// apartSlot, with what the tick's `q !== slot` needs: true when the answer is `slot` itself and not a point of the same value. (orders.ts SquadOrders.apartSlot)
fn apart_slot_kept(g: &mut Game, gd: GuardId, a: OrdersAssignmentId, slot: V3) -> (V3, bool) {
    let k = g.k.orders.file_apart;
    let (sam, p) = (g.ch[g.pl[g.player].ch].pos, g.ch[g.gd[gd].ch].pos);
    let mut t = squad_orders_ahead(g, gd, a);
    for (o, &b) in g.orders.asg.iter() {
        let bb = &g.tab.orders_assignment[b];
        if o != gd && g.gd[o].state != crate::game::GuardState::Dead && bb.post.is_some() { t.push(bb.goal); }
    }
    let mut any = false;
    for &q in t.iter() { if flat(q, slot) < k && apart_open(g, q, slot) { any = true; break; } }
    if !any { return (slot, true); }
    if let Some(&row) = g.orders.apart_of.get(a) {
        if flat(row.at, slot) < 0.5 {
            if let Some(q) = row.q {
                if (row.short.unwrap_or(false) || apart_clear(&t, q, row.m)) && !apart_bad(g, q) { return (q, false); }
            } else if m::abs(g.time - row.t) < 1.0 {
                let q = match apart_short(g, p, slot, &t, k) { Some(q) => q, None => return apart_hold(g, p, sam, slot, &t, k) };
                let r = g.orders.apart_of.get_mut(a).unwrap();
                r.q = Some(q); r.short = Some(true);
                return (q, false);
            }
        }
    }
    let mut taken = t.clone();
    taken.push([sam[0], 0.0, sam[2]]);
    let (mut q, mut by, mut stop): (Option<V3>, f64, bool) = (None, k, false);
    let tight = g.k.orders.file_tight;
    let tries: Vec<f64> = if tight < k { vec![k, tight] } else { vec![k] };
    for ap in tries {   // (doorwayClearNear itself wants a clear line from the place to the point)
        q = crate::doorway::doorway_clear_near(g, slot, &mut |g: &mut Game, x: V3| apart_bad(g, x) || !apart_open(g, x, sam), Some(&taken), Some(ap), Some(p));
        if q.is_none() { q = crate::doorway::doorway_clear_near(g, slot, &mut |g: &mut Game, x: V3| apart_bad(g, x), Some(&taken), Some(ap), Some(p)); }
        if q.is_some() { by = ap; break; }
    }
    if q.is_none() { q = apart_short(g, p, slot, &t, k); stop = q.is_some(); }
    let now = g.time;
    g.orders.apart_of.set(a, SquadOrdersApartOfValue { at: [slot[0], 0.0, slot[2]], q, t: now, m: by, short: Some(stop) });
    match q { Some(q) => (q, false), None => apart_hold(g, p, sam, slot, &t, k) }
}

/// ORDERS.fileSpread: a follower's place within a body's room of where a man ahead will stand: the nearest clear floor. (orders.ts SquadOrders.apartSlot)
pub fn squad_orders_apart_slot(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, slot: V3) -> Option<V3> {
    Some(apart_slot_kept(g, gd, a, slot).0)
}

/// (orders.ts slot's `reach`): straight on, a quarter metre at a time, while a man can stand there and walk there
fn slot_reach(g: &mut Game, prev: V3, dir: V3, left: f64, ang: f64) -> (V3, f64) {
    let (c, sn) = (m::cos(ang), m::sin(ang));
    let d: V3 = [dir[0] * c + dir[2] * sn, 0.0, dir[2] * c - dir[0] * sn];
    let (mut p, mut rem) = (prev, left);
    while rem > 1e-3 {
        let s = m::min(0.25, rem);
        let nx: V3 = [p[0] + d[0] * s, 0.0, p[2] + d[2] * s];
        if crate::scene_collision::nav_grid_is_blocked(g, nx[0], nx[2]) || crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [nx[0], 0.45, nx[2]], None) { break; }
        p = nx;
        rem -= s;
    }
    (p, rem)
}

/// The point `back` metres behind Sam along his trail; past its end, on straight the way it was going, as far as the floor is free. (orders.ts SquadOrders.slot)
pub fn squad_orders_slot(g: &mut crate::game::Game, back: f64) -> V3 {
    let sam = g.ch[g.pl[g.player].ch].pos;
    let mut prev: V3 = [sam[0], 0.0, sam[2]];
    let mut left = back;
    let mut dir: V3 = g.orders.back;
    for i in (0..g.orders.trail.len()).rev() {
        let ti = g.orders.trail[i];
        let seg = flat(prev, ti);
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

/// The free floor at p, else the nearest within floorSnapM (the candidate nearest Sam among equals). None: nowhere. (orders.ts SquadOrders.freeSpot)
pub fn squad_orders_free_spot(g: &mut crate::game::Game, p: V3) -> Option<V3> {
    if !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return Some([p[0], 0.0, p[2]]); }
    let sam = g.ch[g.pl[g.player].ch].pos;
    let (mut best, mut bs): (Option<V3>, f64) = (None, f64::INFINITY);
    let reach = g.k.orders.floor_snap_m + 1e-6;
    let mut r = 0.25;
    while r <= reach {
        for k in 0..12 {
            let a = (k as f64) * m::PI / 6.0;
            let q: V3 = [p[0] + m::sin(a) * r, 0.0, p[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { continue; }
            let s = r + 0.05 * flat(q, sam);
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

    /// the file's places: the two the table holds, then 1.1 m apart behind them (orders.ts backOf)
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

    /// a hold's seconds count at most a tenth a frame, and stop being a wait past the cap (orders.ts heldWait)
    #[test]
    fn held_wait_caps_each_frame_and_the_whole() {
        run(|g| {
            let a = OrdersAssignmentId(0);
            g.k.doormanners.held_cap = 0.15;
            g.time = 10.0;
            assert!(squad_orders_held_wait(g, a));
            g.time = 11.0;
            assert!(squad_orders_held_wait(g, a));
            assert_eq!(g.orders.held_of.get(a).unwrap().total, 0.1);
            g.time = 12.0;
            assert!(!squad_orders_held_wait(g, a));
        });
    }

    /// the trail takes a crumb every `crumb` metres, on the floor, and keeps `trailM` metres of them (orders.ts crumb)
    #[test]
    fn crumb_spaces_and_trims() {
        run(|g| {
            g.k.orders.crumb = 0.4;
            g.k.orders.trail_m = 1.0;
            g.orders.trail = vec![[0.0, 0.0, 0.0]];
            squad_orders_crumb(g, [0.3, 1.5, 0.0], &[]);   // short of a crumb: nothing
            assert_eq!(g.orders.trail.len(), 1);
            for i in 1..6 { squad_orders_crumb(g, [0.5 * i as f64, 1.5, 0.0], &[]); }
            // (the walk back from the newest passes 1 m at the third crumb; splice(0, i - 1) keeps one crumb past it)
            assert_eq!(g.orders.trail.iter().map(|q| q[0]).collect::<Vec<_>>(), vec![1.0, 1.5, 2.0, 2.5]);
            assert_eq!(g.orders.trail[0][1], 0.0);   // (on the floor, whatever his height)
        });
    }
}
