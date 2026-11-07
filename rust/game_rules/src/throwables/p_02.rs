//! src/game/throwables.ts lines 201 to 515
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The pool of thrown things, over the game's block `st`.   (throwables.ts Throwables.constructor)
pub fn throwables_new(g: &crate::game::Game, st: Option<crate::Opaque>) -> crate::throwables::Throwables {
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
    let it = item_new(g, row);
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

/// The box world's copy stood for the walk, with the game's own leaves and the furniture in it. In Rust the walk is always the module's: nothing to decide, nothing to hand back.   (throwables.ts Throwables.world)
pub fn throwables_world(g: &mut crate::game::Game) {
    let dyn_ = g.col.dynamic_boxes.clone();
    crate::scene_collision::rules_world(g, &dyn_);
    debug_assert!(crate::scene_collision::rules_panes(g) >= 0.0, "the pane's rule is not the module's to ask");
    let p = furniture(g);
    if !crate::scene_box_mirror::box_mirror_things(g, &p) {
        panic!("the box world's copy would not take the level's furniture: more things, or more boxes of them, than its tables hold, and a level of more is not loaded (game/levelfit.ts levelOver)");
    }
}

/// `this.props` as the copy reads it: every thing by its id, and the ones in the level in the furniture's own order (throwables.ts:318 hands the PropSystem itself).
fn furniture(g: &mut Game) -> crate::scene_box_mirror::Furniture {
    let (all, props) = (g.props.all.clone(), g.props.props.clone());
    crate::scene_box_mirror::Furniture { all: things_of(g, &all), props: things_of(g, &props) }
}
fn things_of(g: &mut Game, ids: &[PropId]) -> Vec<crate::scene_box_mirror::FurnitureThing> {
    let mut out = Vec::with_capacity(ids.len());
    for &p in ids {
        let (id, radius, boxes) = { let r = &g.tab.props_prop[p]; (r.id, r.radius, r.boxes.clone()) };
        let (x, z, yaw) = (crate::props::prop_x(g, p), crate::props::prop_z(g, p), crate::props::prop_yaw(g, p));
        out.push(crate::scene_box_mirror::FurnitureThing { id, x, z, yaw, radius, boxes });
    }
    out
}

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
    let ask = crate::scene_collision::rules_panes(g);
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
        // player.ts:815: the player's own throw, and the room hears it
        ItemBounce::CanHeard => {
            crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(p), Some(m::min(1.0, sp / 4.0)), None);
            let e = crate::game::GameEvent { kind: crate::director::EventKind::Shot, pos: v3::copy(p), time: g.time, loud: false, serial: g.serial(), ..Default::default() };
            g.events.push(e);
        }
        // launcher.ts:169
        ItemBounce::RingSkitter => {
            if sp > 1.2 {
                crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(p), Some(m::min(0.5, sp / 12.0)), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.7 }));
            }
        }
        // he40.ts:123
        ItemBounce::ShellSkitter => {
            if sp > 1.0 {
                crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(p), Some(m::min(0.7, sp / 8.0)), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.8 }));
            }
        }
        // player.ts:363
        ItemBounce::Mag => crate::audio_iface::game_audio_like_play(g, "magDrop", Some(p), Some(m::min(1.0, sp / 3.0)), None),
        // player.ts:382, guards.ts:2782
        ItemBounce::MagRate { belt } => crate::audio_iface::game_audio_like_play(g, "magDrop", Some(p), Some(m::min(1.0, sp / 3.0)), Some(&crate::audio_iface::PlayOpts::Rate { rate: if belt { 0.6 } else { 0.8 } })),
        // combat.ts:423 dropFromHand; who 0: nobody living made it, a clatter the room cannot explain
        ItemBounce::Dropped { kind, quiet } => {
            if sp < 0.7 {
                return;
            }
            let rate = if kind == ItemKind::Pistol { 0.72 } else if kind == ItemKind::Rifle { 0.6 } else { 0.85 };
            crate::audio_iface::game_audio_like_play(g, "magDrop", Some(p), Some(m::min(1.0, sp / 3.0) * (if quiet { 0.5 } else { 1.0 })), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
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

/// `out[k].mkey = key` of a box made this frame: it has no row to write yet (skeleton 5 has g.col.boxes: Vec<BoxId> and no Tab<BoxId, Box>).   (throwables.ts:507)
fn seam_box_set_mkey(g: &mut Game, b: BoxId, mkey: f64) { g.tab.boxes[b].mkey = Some(mkey); }

/// Fill 4 to 7's magazines (gunreload.ts): half length, half width, half thickness, colour.   (throwables.ts:502)
const OWN_MAGS: [(f64, f64, f64, V3); 4] = [
    (0.11, 0.016, 0.012, [0.06, 0.06, 0.065]),
    (0.06, 0.045, 0.035, [0.16, 0.17, 0.1]),
    (0.065, 0.014, 0.012, [0.03, 0.03, 0.033]),
    (0.1, 0.011, 0.012, [0.03, 0.03, 0.033]),
];

/// A thrown thing's parts as boxes into this frame's list, each keyed off its serial.   (throwables.ts Throwables.boxes)
pub fn throwables_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    let flag = crate::scene_boxes::box_flag();
    let dy = flag.dynamic;
    let dn = (js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64;
    let item_key = crate::scene_boxes::mkey().item;
    let x_axis: V3 = [1.0, 0.0, 0.0];
    for i in 0..g.items.items.len() {
        let it = g.items.items[i];
        // (its row, read once)
        let (p, kind, settled, yaw, tumble, radius, fill, serial) = (item_pos(g, it), item_kind(g, it), item_settled(g, it), item_yaw(g, it), item_tumble(g, it), item_radius(g, it), item_fill(g, it), item_serial(g, it));
        let n0 = out.len();
        let flying = v3::quat::mul(v3::quat::yaw(yaw), v3::quat::axis_angle(x_axis, tumble));
        match kind {
            ItemKind::Smoke | ItemKind::Flash => {
                let (body, band): (V3, V3) = if kind == ItemKind::Flash {
                    if item_fired(g, it) { ([0.03, 0.03, 0.03], [0.12, 0.11, 0.1]) } else { ([0.07, 0.075, 0.08], [0.7, 0.7, 0.66]) }
                } else {
                    ([0.25, 0.29, 0.2], [0.6, 0.08, 0.06])
                };
                let c0: V3 = if settled { [p[0], p[1] - radius + 0.034, p[2]] } else { v3::copy(p) };
                out.push(item_box(g, c0, [0.032, 0.032, 0.07], yaw, flying, body, dy));
                out.push(item_box(g, v3::add(c0, v3::quat::rotate(flying, [0.0, 0.0, 0.045])), [0.034, 0.034, 0.012], yaw, flying, band, dn));
            }
            ItemKind::Frag => {
                let f = frag_item();
                let c0: V3 = if settled { [p[0], p[1] - radius + f.half[0] + 0.002, p[2]] } else { v3::copy(p) };
                out.push(item_box(g, c0, f.half, yaw, flying, f.body, dy));
                out.push(item_box(g, v3::add(c0, v3::quat::rotate(flying, [0.0, 0.0, 0.012])), [0.034, 0.034, 0.006], yaw, flying, f.band, dn));
                out.push(item_box(g, v3::add(c0, v3::quat::rotate(flying, [0.0, 0.0, 0.05])), [0.012, 0.012, 0.014], yaw, flying, f.fuze, dn));
            }
            ItemKind::Pistol | ItemKind::Torch | ItemKind::Rifle => {
                // in the air it flips end over end; settled, on its side with that face on the floor
                let thick = if kind == ItemKind::Pistol { 0.016 } else if kind == ItemKind::Rifle { if fill == 1.0 { sc20_k_item().thick } else { rifle_item().thick } } else { 0.022 };
                let rot = if settled {
                    v3::quat::mul(v3::quat::mul(v3::quat::yaw(yaw), v3::quat::axis_angle(x_axis, m::round(tumble / m::PI) * m::PI)), v3::quat::axis_angle([0.0, 0.0, 1.0], m::PI / 2.0))
                } else {
                    flying
                };
                let c: V3 = if settled { [p[0], p[1] - radius + thick + 0.004, p[2]] } else { v3::copy(p) };
                let at = |off: V3| -> V3 { v3::add(c, v3::quat::rotate(rot, off)) };
                if kind == ItemKind::Pistol {
                    out.push(item_box(g, at([0.0, 0.0, 0.0]), [0.016, 0.026, 0.095], yaw, rot, [0.05, 0.05, 0.055], dy));
                    let grip_rot = v3::quat::mul(rot, v3::quat::axis_angle(x_axis, 108.0 * m::PI / 180.0));
                    out.push(item_box(g, v3::add(at([0.0, -0.024, -0.045]), v3::quat::rotate(grip_rot, [0.0, 0.0, 0.047])), [0.014, 0.017, 0.047], yaw, grip_rot, [0.05, 0.05, 0.055], dy));
                    if fill > 0.0 {
                        out.push(item_box(g, at([0.0, 0.0, 0.17]), [0.019, 0.019, 0.075], yaw, rot, [0.09, 0.09, 0.09], dy));
                    }
                } else if kind == ItemKind::Rifle && fill == 1.0 {
                    let s = m4_item();
                    out.push(item_box(g, at([0.0, 0.0, s.hull.z]), s.hull.h, yaw, rot, s.polymer, dy));
                    out.push(item_box(g, at([0.0, 0.0, s.fore.z]), s.fore.h, yaw, rot, s.fore_col, dy));
                    out.push(item_box(g, at([0.0, s.mag.y, s.mag.z]), s.mag.h, yaw, rot, s.metal, dy));
                    out.push(item_box(g, at([0.0, s.optic.y, s.optic.z]), s.optic.h, yaw, rot, s.metal, dn));
                } else if kind == ItemKind::Rifle {
                    let r = rifle_item();
                    out.push(item_box(g, at([0.0, 0.0, r.receiver.z]), r.receiver.h, yaw, rot, r.steel, dy));
                    out.push(item_box(g, at([0.0, 0.0, r.fore.z]), r.fore.h, yaw, rot, r.wood, dy));
                    let mag_rot = v3::quat::mul(rot, v3::quat::axis_angle(x_axis, r.mag.cant));
                    out.push(item_box(g, at([0.0, r.mag.y, r.mag.z]), r.mag.h, yaw, mag_rot, r.wood, dy));
                    out.push(item_box(g, at([0.0, -0.012, r.stock.z]), r.stock.h, yaw, rot, r.steel, dn));
                } else {
                    out.push(item_box(g, at([0.0, 0.0, 0.0]), [0.022, 0.022, 0.09], yaw, rot, [0.09, 0.09, 0.1], dy));
                    out.push(item_box(g, at([0.0, 0.0, 0.095]), [0.026, 0.026, 0.008], yaw, rot, [0.16, 0.16, 0.15], dn));
                }
            }
            // GEAR.on, settled true on the page: the 40 mm sponge round
            ItemKind::Ring => {
                let c: V3 = if settled { [p[0], p[1] - radius + sponge_item().wide + 0.002, p[2]] } else { v3::copy(p) };
                sponge_boxes(g, out, c, if settled { v3::quat::yaw(yaw) } else { flying }, yaw);
            }
            ItemKind::Shell => {
                let s = shell_item();
                let c0: V3 = if settled { [p[0], p[1] - radius + s.half[0] + 0.002, p[2]] } else { v3::copy(p) };
                out.push(item_box(g, c0, s.half, yaw, flying, s.body, dy));
                out.push(item_box(g, v3::add(c0, v3::quat::rotate(flying, [0.0, 0.0, s.half[2] * 0.62])), [s.half[0] + 0.001, s.half[1] + 0.001, s.half[2] * 0.4], yaw, flying, s.nose, dn));
            }
            // a magazine. fill 2: a rifleman's empty thirty; 3: the FN2000's black thirty; 4 to 7: OWN_MAGS; else a pistol's (0 empty, 0 to 1 part full)
            ItemKind::Mag => {
                let (box30, box20k) = (fill > 1.0, fill >= 3.0);
                let own = if fill >= 4.0 { Some(OWN_MAGS[m::min(3.0, fill - 4.0) as usize]) } else { None };
                let col: V3 = match own {
                    Some(o) => o.3,
                    None => if box20k { sc20_k_item().metal } else if box30 { rifle_item().wood } else if fill > 0.0 { [0.09, 0.09, 0.1] } else { [0.06, 0.06, 0.065] },
                };
                let hl = match own { Some(o) => o.0, None => if box30 { 0.095 } else { 0.055 } };
                let hw = match own { Some(o) => o.1, None => if box30 { 0.014 } else { 0.018 } };
                let th = match own { Some(o) => o.2, None => 0.012 };
                if settled {
                    let b = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP { c: [p[0], p[1] + th, p[2]], h: [hw, th, hl], yaw: Some(yaw), albedo: Some(col), flags: Some(dn), ..Default::default() });
                    out.push(b);
                } else {
                    let deep = match own { Some(o) => m::max(0.03, o.2), None => 0.03 };
                    out.push(item_box(g, v3::copy(p), [hw, hl, deep], yaw, flying, col, dn));
                }
            }
        }
        // motion identity per part: the g-buffer reprojects the flying can through its own last transform, whatever spawned or died around it in the list
        for k in n0..out.len() {
            let b = out[k];
            seam_box_set_mkey(g, b, item_key + (js::to_i32(serial) & 8191) as f64 * 8.0 + m::min(7.0, (k - n0) as f64));
        }
    }
}

/// A canister that is still something: its fuse not yet gone, or (smoke) gone and still venting.   (throwables.ts liveCan)
pub fn live_can(g: &mut crate::game::Game, x: ItemId) -> bool {
    let k = item_kind(g, x);
    (k == ItemKind::Smoke || k == ItemKind::Flash || k == ItemKind::Frag) && (!item_fired(g, x) || item_emit_t(g, x) > 0.0)
}
