//! src/game/throwables.ts lines 201 to 515
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The pool of thrown things, over the game's block `st`.   (throwables.ts Throwables.constructor)
pub fn throwables_new(g: &mut crate::game::Game, st: Option<crate::Opaque>) -> crate::throwables::Throwables {
    let mut t = Throwables::blank();
    t.store = crate::sim_itemshome::item_store_new(g, st.unwrap_or_default());
    t
}

/// Closest hit among statics + door leaves and props.   (throwables.ts Throwables.ray)
pub fn throwables_ray(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::throwables::ThrowablesRay> {
    let st = crate::scene_collision::raycast(g, ro, rd, tmax);
    let ph = crate::props::prop_system_raycast(g, ro, rd, match &st { Some(s) => s.t, None => tmax });
    if let Some(ph) = ph {
        return Some(ThrowablesRay { t: ph.t, n: ph.n, prop: Some(ph.prop) });
    }
    st.map(|s| ThrowablesRay { t: s.t, n: s.n, prop: None })
}

/// The rows are the list: where `items` is not the list the rows were last laid in, what left it leaves with its numbers and the rows are laid anew.   (throwables.ts Throwables.sync)
pub fn throwables_sync(g: &mut crate::game::Game) {
    if g.items.items == g.items.rows {
        return;
    }
    let list = g.items.items.clone();
    let mut rows: Vec<Vec<f64>> = Vec::with_capacity(list.len());
    for &it in &list {
        rows.push(item_numbers(g, it));
    }
    let was = g.items.rows.clone();
    for &it in &was {
        if !list.contains(&it) {
            item_leave(g, it);
        }
    }
    crate::sim_itemshome::item_store_lay(g, &rows);
    for (k, &it) in list.iter().enumerate() {
        item_stand(g, it, k as f64);
    }
    g.items.rows = list;
}

/// A new thing in the pool: the dice in the order they were always drawn and before anything is given up for it; `opts` over the rest.   (throwables.ts Throwables.spawn)
pub fn throwables_spawn(g: &mut crate::game::Game, kind: crate::throwables::ItemKind, pos: V3, vel: V3, opts: Option<crate::throwables::ThrowablesSpawnOpts>) -> ItemId {
    let opts = opts.unwrap_or_default();
    let can = kind == ItemKind::Smoke || kind == ItemKind::Flash;
    let (rifle, ring, frag, shell) = (kind == ItemKind::Rifle, kind == ItemKind::Ring, kind == ItemKind::Frag, kind == ItemKind::Shell);
    let (c, r, rg, f, s) = (can_item(), rifle_item(), ring_item(), frag_item(), shell_item());
    let radius = if can { c.radius } else if frag { f.radius } else if kind == ItemKind::Pistol { 0.05 } else if kind == ItemKind::Torch { 0.045 } else if rifle { r.radius } else if ring { rg.radius } else if shell { s.radius } else { 0.03 };
    let restitution = if can { c.restitution } else if frag { f.restitution } else if kind == ItemKind::Mag { 0.25 } else if rifle { r.restitution } else if ring { rg.restitution } else if shell { s.restitution } else { 0.22 };
    let friction = if can { c.friction } else if kind == ItemKind::Pistol || kind == ItemKind::Torch { 0.5 } else if rifle { r.friction } else if ring { rg.friction } else if frag { f.friction } else if shell { s.friction } else { 0.35 };
    // the dice, one at a time and in this order
    let yaw = crate::math_dice::sim_random(g) * m::PI;
    let spin = (crate::math_dice::sim_random(g) - 0.5) * 14.0;
    let tumble = crate::math_dice::sim_random(g) * 6.28;
    let way = if crate::math_dice::sim_random(g) < 0.5 { -1.0 } else { 1.0 };
    let tumble_rate = way * (7.0 + crate::math_dice::sim_random(g) * 9.0);
    let life = if kind == ItemKind::Mag { 40.0 } else if ring { rg.life } else if shell { s.life } else { 60.0 };
    let heading = v3::yaw_of(vel);
    let serial = g.items.serial;
    g.items.serial += 1.0;
    throwables_sync(g);
    // FIFO cap, before the push so the new item can never evict itself; a can on its fuse or venting is never evicted: spent things first, then the oldest
    // dropped gun or torch, and past the columns' rows the oldest whatever it is.
    if g.items.items.len() as f64 >= crate::sim_itemshome::ITEM_CAP {
        let mut i = first_spent(g);
        if i.is_none() {
            i = first_not_live(g);
        }
        if let Some(i) = i {
            throwables_remove_at(g, i as f64);
        }
        if g.items.items.len() as f64 >= crate::sim_itemshome::ITEM_ROWS {
            throwables_remove_at(g, 0.0);
        }
    }
    let row = crate::sim_itemshome::item_store_enter(g);
    let things = g.props.all.clone();
    let it = item_new(g, row, &things);
    // every field through its own setter: the row is written
    item_set_kind(g, it, opts.kind.unwrap_or(kind));
    item_set_pos(g, it, opts.pos.unwrap_or(pos));
    item_set_vel(g, it, opts.vel.unwrap_or(vel));
    item_set_radius(g, it, opts.radius.unwrap_or(radius));
    item_set_restitution(g, it, opts.restitution.unwrap_or(restitution));
    item_set_friction(g, it, opts.friction.unwrap_or(friction));
    item_set_yaw(g, it, opts.yaw.unwrap_or(yaw));
    item_set_spin(g, it, opts.spin.unwrap_or(spin));
    item_set_tumble(g, it, opts.tumble.unwrap_or(tumble));
    item_set_tumble_rate(g, it, opts.tumble_rate.unwrap_or(tumble_rate));
    item_set_settled(g, it, opts.settled.unwrap_or(false));
    item_set_age(g, it, opts.age.unwrap_or(0.0));
    item_set_fuse(g, it, opts.fuse.unwrap_or(1.3));
    item_set_fired(g, it, opts.fired.unwrap_or(false));
    item_set_life(g, it, opts.life.unwrap_or(life));
    item_set_emit_t(g, it, opts.emit_t.unwrap_or(0.0));
    item_set_fill(g, it, opts.fill.unwrap_or(0.0));
    item_set_bounces(g, it, opts.bounces.unwrap_or(0.0));
    item_set_heading(g, it, opts.heading.unwrap_or(heading));
    item_set_who(g, it, opts.who.unwrap_or(0.0));
    item_set_on(g, it, opts.on);
    item_set_on_pose(g, it, opts.on_pose.unwrap_or(0.0));
    item_set_serial(g, it, opts.serial.unwrap_or(serial));
    // (what only `opts` says: absent stays unset)
    if opts.carried.is_some() {
        item_set_carried(g, it, opts.carried);
    }
    g.tab.item[it].side = opts.side;
    g.tab.item[it].clatters = opts.clatters;
    g.items.items.push(it);
    g.items.rows.push(it);
    it
}

/// The first spent thing in the list: a magazine, a ring, a dud, a can that has gone off and finished.   (throwables.ts:252)
fn first_spent(g: &mut Game) -> Option<usize> {
    for i in 0..g.items.items.len() {
        let x = g.items.items[i];
        let k = item_kind(g, x);
        if k == ItemKind::Mag || k == ItemKind::Ring || k == ItemKind::Shell || ((k == ItemKind::Smoke || k == ItemKind::Flash || k == ItemKind::Frag) && !live_can(g, x)) {
            return Some(i);
        }
    }
    None
}

/// The first thing in the list that is no live can.   (throwables.ts:253)
fn first_not_live(g: &mut Game) -> Option<usize> {
    for i in 0..g.items.items.len() {
        let x = g.items.items[i];
        if !live_can(g, x) {
            return Some(i);
        }
    }
    None
}

/// An item leaves by the pool's own rule: it takes its numbers, its row is given up, the rows behind it close up. (`sync` has run.)   (throwables.ts Throwables.removeAt)
pub fn throwables_remove_at(g: &mut crate::game::Game, i: f64) {
    let i = i as usize;
    let it = g.items.items[i];
    item_leave(g, it);
    crate::sim_itemshome::item_store_close(g, i as f64);
    g.items.items.remove(i);
    g.items.rows.remove(i);
    for k in i..g.items.items.len() {
        let x = g.items.items[k];
        item_stand(g, x, k as f64);
    }
}

/// Launch velocity from `from` to land at `to` with a pleasant arc; also the sampled preview points until first impact.   (throwables.ts Throwables.solve)
pub fn throwables_solve(g: &mut crate::game::Game, from: V3, to: V3, max_speed: Option<f64>) -> crate::throwables::ThrowSolution {
    let max_speed = max_speed.unwrap_or(11.0);
    let d = v3::sub(to, from);
    let dist = m::hyp2(d[0], d[2]);
    let mut t = m::min(1.25, m::max(0.42, dist / 7.5));
    let mut v0: V3 = [d[0] / t, (d[1] + 0.5 * GRAVITY * t * t) / t, d[2] / t];
    let sp = v3::len(v0);
    if sp > max_speed {
        v0 = v3::scale(v0, max_speed / sp);
    }
    // preview: integrate until first hit
    let mut points: Vec<V3> = Vec::new();
    let mut p = v3::copy(from);
    let mut v = v3::copy(v0);
    let mut land = v3::copy(to);
    let h = 1.0 / 30.0;
    for i in 0..90 {
        points.push(v3::copy(p));
        let np: V3 = [p[0] + v[0] * h, p[1] + v[1] * h - 0.5 * GRAVITY * h * h, p[2] + v[2] * h];
        let seg = v3::sub(np, p);
        let len = v3::len(seg);
        if len > 1e-5 {
            if let Some(hit) = throwables_ray(g, p, v3::scale(seg, 1.0 / len), len + 0.05) {
                land = v3::mad(p, seg, hit.t / len);
                points.push(land);
                break;
            }
        }
        if np[1] < 0.02 {
            land = [np[0], 0.02, np[2]];
            points.push(land);
            break;
        }
        v = [v[0], v[1] - GRAVITY * h, v[2]];
        p = np;
        t = i as f64 * h;
    }
    ThrowSolution { v0, t, points, land }
}

/// One tick of the pool: the module walks it from the newest down, then what the steps listed is done in order.   (throwables.ts Throwables.update)
pub fn throwables_update(g: &mut crate::game::Game, dt: f64) {
    throwables_sync(g);
    let mut r = g.items.items.len() as f64 - 1.0;
    if r < 0.0 {
        return;
    }
    throwables_world(g);
    let mut ev: Vec<f64> = Vec::new();
    g.m.sim_itemshome.items_stepped.steps_there += 1.0;
    while r >= 0.0 {
        // the module walks from row r down to the bottom (−1). A row it hands back met a ray that is no number: a mistake, said by name
        r = throwables_walk_there(g, dt, r, &mut ev);
        if r < 0.0 {
            break;
        }
        g.m.sim_itemshome.items_stepped.host += 1.0;
        panic!("a ray that is no number, from a thrown thing's walk (row {})", r);
    }
    throwables_say(g, &ev);
}

/// Whether the walk is the module's to take. In Rust it always is (blitz/doors-kit.md), over the game's one world: nothing to decide, nothing to hand back.   (throwables.ts Throwables.world)
pub fn throwables_world(g: &mut crate::game::Game) {}

/// One number of the walk's hand-over (rust/sim items.rs PAR, by its address as the TypeScript has it: never a reference kept across the walk).
fn par_put(i: usize, v: f64) {
    debug_assert!(i < sim::items::NPAR);
    unsafe { *(sim::items::addr(0) as *mut f64).add(i) = v }
}
fn par_get(i: usize) -> f64 {
    debug_assert!(i < sim::items::NPAR);
    unsafe { *(sim::items::addr(0) as *const f64).add(i) }
}
/// One number of what the walk listed (rust/sim items.rs EVS).
fn ev_get(i: usize) -> f64 {
    debug_assert!(i < sim::items::MAX_EVENTS * sim::items::EV);
    unsafe { *(sim::items::addr(1) as *const f64).add(i) }
}

/// The module's walk from row `from` down: the hand-over written, the walk asked, the tallies kept, what it listed copied onto `ev`. → the row it hands back, or −1.   (throwables.ts Throwables.walkThere)
pub fn throwables_walk_there(g: &mut crate::game::Game, dt: f64, from: f64, ev: &mut Vec<f64>) -> f64 {
    let ask = crate::scene_collision::pane_ask(g);
    debug_assert!(ask >= 0.0, "the pane's rule is not the module's to ask");
    let bog = crate::snow_bog::snow_bog_may(g);
    par_put(sim::items::P_DT, dt);
    par_put(sim::items::P_FROM, from);
    par_put(sim::items::P_ASK, ask);
    par_put(sim::items::P_BOG, if bog { 1.0 } else { 0.0 });
    par_put(sim::items::P_RIFLE_SHARE, rifle_item().prop_share);
    if bog {
        // lying snow, as the module asks it: the level's table in the box world, the rule's numbers and the roofs in force handed over
        let roofs = crate::weather::weather_roofs_now(g);
        debug_assert!(roofs.len() as f64 <= crate::sim_itemshome::ITEM_ROOFS, "more roofs over the snow than a walk is handed");
        let kinds = crate::snow_bog::snow_bog_kinds(g);
        crate::scene_box_mirror::box_mirror_snow(g, &kinds);
        let k = &g.k.snowbog;
        par_put(sim::items::P_SNOW_TOP, k.top);
        par_put(sim::items::P_SNOW_BANK, k.bank_m);
        par_put(sim::items::P_SNOW_PACKED, k.packed);
        par_put(sim::items::P_SNOW_OLD, k.old);
        par_put(sim::items::P_SNOW_DEAD, k.dead);
        par_put(sim::items::P_SNOW_GRIP, k.grip);
        par_put(sim::items::P_SNOW_STICK, k.stick);
        par_put(sim::items::P_ROOFS, roofs.len() as f64);
        for (i, r) in roofs.iter().enumerate() {
            let o = sim::items::P_ROOF + i * 4;
            par_put(o, r[0]);
            par_put(o + 1, r[1]);
            par_put(o + 2, r[2]);
            par_put(o + 3, r[3]);
        }
    }
    let a = sim::items::update(g.sim, g.col.mirror.slot as usize);
    if a != sim::items::HOST && a != sim::items::DONE {
        panic!("the Rust module would not walk the thrown things from row {}", from);
    }
    let n = par_get(sim::items::P_EVENTS);
    let t = &mut g.m.sim_itemshome.items_stepped;
    t.walks += 1.0;
    t.there += par_get(sim::items::P_WALKED);
    t.rays += par_get(sim::items::P_RAYS);
    t.thing_rays += par_get(sim::items::P_THING_RAYS);
    t.substeps += par_get(sim::items::P_SUBSTEPS);
    t.rested += par_get(sim::items::P_RESTED);
    t.events += n;
    for i in 0..(n as usize * sim::items::EV) {
        ev.push(ev_get(i));
    }
    par_get(sim::items::P_ROW)
}

/// What the steps listed, done in the order it was listed: a bounce said by the item's own callback, a share of momentum to a thing of furniture, an item whose time is up taken out.   (throwables.ts Throwables.say)
pub fn throwables_say(g: &mut crate::game::Game, e: &[f64]) {
    if e.is_empty() {
        return;
    }
    // (an event names its item by its row in the list as the steps walked it; what a callback does to the list since is its own)
    let walked = g.items.items.clone();
    let mut o = 0;
    while o < e.len() {
        let at = o;
        o += sim::items::EV;
        let it = walked[e[at + sim::items::E_ROW] as usize];
        let what = e[at + sim::items::E_KIND];
        if what == sim::items::EV_GONE {
            throwables_sync(g);
            if let Some(i) = g.items.items.iter().position(|&x| x == it) {
                throwables_remove_at(g, i as f64);
            }
            continue;
        }
        let p: V3 = [e[at + sim::items::E_P], e[at + sim::items::E_P + 1], e[at + sim::items::E_P + 2]];
        let v: V3 = [e[at + sim::items::E_V], e[at + sim::items::E_V + 1], e[at + sim::items::E_V + 2]];
        if what == sim::items::EV_IMPULSE {
            // the item's lost momentum goes into the chair / carton it struck
            let prop = g.props.all[e[at + sim::items::E_A] as usize];
            let who = item_who(g, it);
            crate::props::prop_system_apply_impulse(g, prop, p, v, who, None);
        } else if let Some(b) = g.tab.item[it].on_bounce.clone() {
            item_bounced(g, b, e[at + sim::items::E_A], p, it, v);
        }
    }
}

/// `it.onBounce(sp, p, g, it, n)`: what each arm stands for, as its setter wrote it.
fn item_bounced(g: &mut Game, b: ItemBounce, sp: f64, p: V3, it: ItemId, n: V3) {
    match b {
        // doortools, tactics, squadthrow and the rest: the can's knock
        ItemBounce::Can => crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(p), Some(m::min(1.0, sp / 4.0)), None),
        // player.ts:814: the player's own throw, and the room hears it
        ItemBounce::CanHeard => {
            crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(p), Some(m::min(1.0, sp / 4.0)), None);
            let e = crate::game::GameEvent { kind: crate::director::EventKind::Shot, pos: v3::copy(p), time: g.time, loud: false, serial: g.serial(), ..Default::default() };
            g.events.push(e);
        }
        // launcher.ts:168
        ItemBounce::RingSkitter => {
            if sp > 1.2 {
                seam_audio_play_rate(g, "canisterBounce", p, m::min(0.5, sp / 12.0), 1.7);
            }
        }
        // he40.ts:123
        ItemBounce::ShellSkitter => {
            if sp > 1.0 {
                seam_audio_play_rate(g, "canisterBounce", p, m::min(0.7, sp / 8.0), 0.8);
            }
        }
        // player.ts:362
        ItemBounce::Mag => crate::audio_iface::game_audio_like_play(g, "magDrop", Some(p), Some(m::min(1.0, sp / 3.0)), None),
        // player.ts:381, guards.ts:2827
        ItemBounce::MagRate { belt } => seam_audio_play_rate(g, "magDrop", p, m::min(1.0, sp / 3.0), if belt { 0.6 } else { 0.8 }),
        // combat.ts:422 dropFromHand; who 0: nobody living made it, a clatter the room cannot explain
        ItemBounce::Dropped { kind, quiet } => {
            if sp < 0.7 {
                return;
            }
            let rate = if kind == ItemKind::Pistol { 0.72 } else if kind == ItemKind::Rifle { 0.6 } else { 0.85 };
            seam_audio_play_rate(g, "magDrop", p, m::min(1.0, sp / 3.0) * (if quiet { 0.5 } else { 1.0 }), rate);
            if !quiet {
                let c = g.tab.item[it].clatters.unwrap_or(0.0) + 1.0;
                g.tab.item[it].clatters = Some(c);
                if c <= 2.0 {
                    let e = crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: v3::copy(p), time: g.time, loud: false, level: Some(m::min(0.5, 0.2 + sp * 0.08)), who: Some(0.0), serial: g.serial(), ..Default::default() };
                    g.events.push(e);
                }
            }
        }
        // disarm.ts:97
        ItemBounce::GunDown => crate::disarm::gun_down(g, sp, p, it, n),
        // savestate.ts:698: disturbed after a load, it clatters as any
        ItemBounce::Loaded { kind } => {
            if sp >= 0.7 {
                let name = if kind == ItemKind::Smoke || kind == ItemKind::Flash { "canisterBounce" } else { "magDrop" };
                crate::audio_iface::game_audio_like_play(g, name, Some(p), Some(m::min(1.0, sp / 3.0)), None);
            }
        }
    }
}

/// `g.audio.play(name, pos, gain, { rate })`: `opts` of game_audio_like_play holds `crate::Opaque`, which cannot carry the rate.
fn seam_audio_play_rate(g: &mut Game, name: &str, pos: V3, gain: f64, rate: f64) {
    todo!("needs: game_audio_like_play's opts to carry a rate")
}

/// Picture only (blitz/doors-kit.md "Throwables.boxes only draws: leave it"): not translated, as the skeleton's stub.   (throwables.ts Throwables.boxes)
pub fn throwables_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    // BRING-UP: game_step_fx_and_dyn_boxes calls this (p_game_07 holds that the dyn boxes are state). With nothing thrown the TypeScript's loop makes no box
    // (throwables.ts:437), so that case returns; a thrown thing's boxes are still not translated.
    if g.items.items.is_empty() { return; }
    todo!("picture only by blitz/doors-kit.md, yet the tick calls it: the boxes of a thrown thing (throwables.ts:436) are not translated")
}

/// A canister that is still something: its fuse not yet gone, or (smoke) gone and still venting.   (throwables.ts liveCan)
pub fn live_can(g: &mut crate::game::Game, x: ItemId) -> bool {
    let k = item_kind(g, x);
    (k == ItemKind::Smoke || k == ItemKind::Flash || k == ItemKind::Frag) && (!item_fired(g, x) || item_emit_t(g, x) > 0.0)
}
