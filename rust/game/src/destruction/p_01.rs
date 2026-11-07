//! src/game/destruction.ts lines 1 to 386
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A box's row (`Box.*`). Skeleton 5 keeps `g.col.boxes: Vec<BoxId>` and no table of Box rows, while the index says `g.col.boxes[id].c`. Once it is the table, the body is `&mut g.col.boxes[b]`.
fn seam_box(g: &mut Game, b: BoxId) -> &mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

/// The knob as the systems read it: a showcase program's own, 0 under the tour, else tune.chaos clamped.   (destruction.ts chaosOf)
pub fn chaos_of(g: &mut crate::game::Game) -> f64 {
    if let Some(s) = g.destr.show_chaos {
        return m::max(0.0, m::min(1.0, s));
    }
    if g.touring {
        return 0.0;
    }
    let c = g.tune.chaos;
    m::max(0.0, m::min(1.0, c))
}

/// The loud levels' rules are live: a scenario is up.   (destruction.ts loudNow)
pub fn loud_now(g: &mut crate::game::Game) -> bool {
    crate::roomclear::room_clear_scenario_active(g) || g.night_breach.active || g.rescue.active
}

/// Impulses at all: chaos above 0 and the switch.   (destruction.ts impulsesOn)
pub fn impulses_on(g: &mut crate::game::Game) -> bool {
    chaos_of(g) > 0.0 && g.tune.debris_impulses
}

/// The impulses' strength scale off the knob.   (destruction.ts impK)
pub fn imp_k(g: &mut crate::game::Game, chaos: f64) -> f64 {
    if chaos <= 0.5 { 1.0 } else { 1.0 + (g.k.chaos.imp.chaos_k - 1.0) * (chaos - 0.5) * 2.0 }
}

/// The point just over a box's top, where line-of-sight targets aim.   (destruction.ts topOf)
pub fn top_of(g: &mut crate::game::Game, b: BoxId) -> V3 {
    let (c, h) = (seam_box(g, b).c, seam_box(g, b).h);
    [c[0], c[1] + h[1] + 0.05, c[2]]
}

/// The lane's capped log, for the panel and the probes.   (destruction.ts dlog)
pub fn dlog(g: &mut crate::game::Game, s: &str) {
    let line = format!("{} {}", js::to_fixed(g.time, 2), s);
    let log = &mut g.m.destruction.destr_log;
    log.push(line);
    if log.len() > 200 {
        log.drain(0..100);
    }
}

/// A cap of n() events in any one-second window.   (destruction.ts RateCap.constructor)
pub fn rate_cap_new(n: crate::destruction::RateCapN) -> crate::destruction::RateCap {
    RateCap { n, ..RateCap::blank() }
}

/// One more event at time t, if the window has room.   (destruction.ts RateCap.take)
pub fn rate_cap_take(this: &mut crate::destruction::RateCap, t: f64, k: &crate::knobs::ChaosK) -> bool {
    let n = match this.n {
        RateCapN::Puff => k.puffs_per_sec,
        RateCapN::Paper => k.papers_per_sec,
        RateCapN::Knock => k.knocks_per_sec,
        RateCapN::Decal => k.decals_per_sec,
    };
    rate_cap_take_n(this, t, n)
}

/// RateCap.take with the knob's value read.
fn rate_cap_take_n(this: &mut RateCap, t: f64, n: f64) -> bool {
    let n = m::max(0.0, m::floor(n));
    if n == 0.0 {
        return false;
    }
    if this.ts.len() as f64 != n {
        this.ts = vec![-1e9; n as usize];
        this.head = 0.0;
    }
    let oldest = this.ts[this.head as usize];
    if t - oldest < 1.0 {
        return false;
    }
    this.ts[this.head as usize] = t;
    this.head = js::rem(this.head + 1.0, n);
    true
}

/// The window afresh.   (destruction.ts RateCap.reset)
pub fn rate_cap_reset(this: &mut crate::destruction::RateCap) {
    for x in this.ts.iter_mut() {
        *x = -1e9;
    }
    this.head = 0.0;
}

/// The underside of the roof over (x, z), or none under the open sky: five rays a 0.6 m square, remembered per square.   (destruction.ts Destruction.roofY)
pub fn destruction_roof_y(g: &mut crate::game::Game, x: f64, z: f64) -> Option<f64> {
    let key = crate::Num(m::floor(x / 0.6) * 4096.0 + m::floor(z / 0.6) + 0.0);
    if let Some(&c) = g.destr.roofs.get(key) {
        return if c < 0.0 { None } else { Some(c) };
    }
    let cx = (m::floor(x / 0.6) + 0.5) * 0.6;
    let cz = (m::floor(z / 0.6) + 0.5) * 0.6;
    let mut n = 0;
    let mut top = -1.0;
    let tmax = crate::scene_world::world().max[1] - 2.3;
    for [dx, dz] in roof_rays() {
        if let Some(hit) = crate::scene_collision::raycast(g, [cx + dx, 2.3, cz + dz], [0.0, 1.0, 0.0], tmax) {
            n += 1;
            top = m::max(top, 2.3 + hit.t);
        }
    }
    let y = if n >= 4 { top } else { -1.0 };
    g.destr.roofs.set(key, y);
    if y < 0.0 { None } else { Some(y) }
}

/// Destructibles registered in the 3×3 cells round p, hidden ones left out.   (destruction.ts Destruction.near)
pub fn destruction_near(g: &mut crate::game::Game, p: V3, out: &mut Vec<crate::destruction::Destructible>) -> Vec<crate::destruction::Destructible> {
    out.clear();
    let world = crate::scene_world::world();
    let cx = m::floor((p[0] - world.min[0]) / g.destr.cs);
    let cz = m::floor((p[2] - world.min[2]) / g.destr.cs);
    for dz in -1..=1 {
        for dx in -1..=1 {
            let x = cx + dx as f64;
            let z = cz + dz as f64;
            if x < 0.0 || z < 0.0 || x >= g.destr.w || z >= world.grid_h {
                continue;
            }
            for &i in g.destr.cells[(z * g.destr.w + x) as usize].iter() {
                if let Some(d) = g.destr.items.get(crate::Num(i)) {
                    if !g.destr.hidden.has(crate::Num(i)) {
                        out.push(d.clone());
                    }
                }
            }
        }
    }
    out.clone()
}

/// The destructible at a static's index.   (destruction.ts Destruction.itemOf)
pub fn destruction_item_of(g: &mut crate::game::Game, i: f64) -> Option<crate::destruction::Destructible> {
    g.destr.items.get(crate::Num(i)).cloned()
}

/// The heat cell of a point, clamped to the grid.   (destruction.ts Destruction.hidx)
pub fn destruction_hidx(g: &mut crate::game::Game, p: V3) -> f64 {
    let world = crate::scene_world::world();
    let cell = g.k.chaos.cell;
    let x = m::min(g.destr.hw - 1.0, m::max(0.0, m::floor((p[0] - world.min[0]) / cell)));
    let z = m::min(g.destr.hh - 1.0, m::max(0.0, m::floor((p[2] - world.min[2]) / cell)));
    z * g.destr.hw + x
}

/// Adds heat at p and gives the escalation there after it.   (destruction.ts Destruction.stoke)
pub fn destruction_stoke(g: &mut crate::game::Game, p: V3, amount: f64, chaos: f64) -> f64 {
    let i = destruction_hidx(g, p) as usize;
    g.destr.heat[i] = (g.destr.heat[i] as f64 + amount) as f32;
    g.destr.heat[i] as f64 * 2.0 * chaos
}

/// The module's own hashed choice, counting off its serial.   (destruction.ts Destruction.hash)
pub fn destruction_hash(g: &mut crate::game::Game) -> f64 {
    let s = g.destr.serial;
    g.destr.serial = s + 1.0;
    crate::debris::hash01(s * 2654435761.0)
}

/// One event out of a cap; a knock-off riding on its group leader's slot is free.   (destruction.ts Destruction.take)
pub fn destruction_take(g: &mut crate::game::Game, k: crate::destruction::DestructionTakeK) -> bool {
    if k == DestructionTakeK::Knock && g.destr.free_knocks > 0.0 {
        g.destr.free_knocks -= 1.0;
        return true;
    }
    let t = g.time;
    let (caps, knobs) = (&mut g.destr.caps, &g.k.chaos);
    match k {
        DestructionTakeK::Puff => rate_cap_take(&mut caps.puff, t, knobs),
        DestructionTakeK::Paper => rate_cap_take(&mut caps.paper, t, knobs),
        DestructionTakeK::Knock => rate_cap_take(&mut caps.knock, t, knobs),
        DestructionTakeK::Decal => rate_cap_take(&mut caps.decal, t, knobs),
    }
}

/// The RUBBLE table as Debris.feet's knobs: the TypeScript hands the table itself.
fn foot_knobs(g: &Game) -> crate::debris::FootKnobs {
    let r = &g.k.rubble;
    crate::debris::FootKnobs {
        radius: r.radius,
        reach: r.reach,
        lead: r.lead,
        slide: r.slide,
        inside: r.inside,
        floor: r.floor,
        heavy: r.heavy,
        walk: r.walk,
        base: r.base,
        gain: r.gain,
        aside: r.aside,
        v_max: r.v_max,
        off: r.off,
        spin: r.spin,
        per_frame: r.per_frame,
        cooldown: r.cooldown,
        give_up: r.give_up,
        give_up_secs: r.give_up_secs,
        scuff: r.scuff,
    }
}

/// Per frame: rubble at men's feet, the pools, the heat's decay, the queued dust.   (destruction.ts Destruction.update)
pub fn destruction_update(g: &mut crate::game::Game, dt: f64) {
    if !g.touring || g.destr.show_chaos.is_some() {
        let men: Vec<crate::debris::FootMan> = destruction_on_foot(g).into_iter().map(|c| crate::debris::FootMan { pos: g.ch[c].pos, vel: g.ch[c].vel }).collect();
        let knobs = foot_knobs(g);
        if !g.destr.debris.list.is_empty() {
            crate::debris::debris_feet(g, &men, knobs, None, None);
        }
        crate::edge_look::edge_look_feet(g, &men, knobs, dt);
    }
    if js::truthy(crate::sim_fallshome::fall_store_waiting_in(g, crate::sim_fallshome::FALL_PIECES)) {
        crate::debris::debris_landings(g);
    }
    if !g.destr.debris.list.is_empty() {
        crate::debris::debris_update(g, dt);
    }
    crate::break_table::breaks_tick(g);
    crate::edge_look::edge_look_update(g, dt);
    crate::wallskin::wall_skin_update(g);
    g.destr.thuds_this_frame = 0.0;
    if chaos_of(g) <= 0.0 && !g.destr.any_heat {
        return;
    }
    let k = m::exp(-dt / g.k.chaos.heat_tau);
    let mut any = false;
    for x in g.destr.heat.iter_mut() {
        let h = *x as f64 * k;
        *x = if h < 0.02 { 0.0 } else { h as f32 };
        if h >= 0.02 {
            any = true;
        }
    }
    g.destr.puffs_this_frame = 0.0;
    g.destr.free_knocks = 0.0;
    if chaos_of(g) <= 0.0 {
        g.destr.dust_queue.clear();
    }
    g.destr.any_heat = any || !g.destr.dust_queue.is_empty();
    // queued dust, oldest first: a puff the caps hold back this frame waits, and is counted refused only when it is let go
    while !g.destr.dust_queue.is_empty() {
        let q = g.destr.dust_queue[0];
        if g.time - q.t > g.k.chaos.loud.burst_secs {
            g.destr.dust_queue.remove(0);
            g.stats.destruction.destr_puffs_refused += 1;
            continue;
        }
        if g.destr.puffs_this_frame >= g.k.chaos.puffs_per_frame {
            break;
        }
        let r0 = g.stats.destruction.destr_puffs_refused;
        if !destruction_dust_puff(g, q.p, q.n, q.scale, q.surf) {
            g.stats.destruction.destr_puffs_refused = r0;
            break;
        }
        g.destr.dust_queue.remove(0);
    }
}

/// The men on their feet this frame, bar a seated civilian.   (destruction.ts Destruction.onFoot)
pub fn destruction_on_foot(g: &mut crate::game::Game) -> Vec<CharId> {
    let mut men: Vec<CharId> = crate::seats::men_up(g).into_iter().map(|pl| g.pl[pl].ch).collect();
    for gd in g.guards.iter() {
        if g.gd[*gd].state != crate::game::GuardState::Dead {
            men.push(g.gd[*gd].ch);
        }
    }
    for cv in crate::civilians::civilians_shown(g) {
        if g.gd[cv].state != crate::game::GuardState::Dead && !crate::civilians::civilian_seated(g, cv) {
            men.push(g.gd[cv].ch);
        }
    }
    men
}

/// The rigid debris' and the blown walls' shadow-casting boxes, for the picture.   (destruction.ts Destruction.boxes)
pub fn destruction_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    crate::debris::debris_boxes(g, out);
    crate::edge_look::edge_look_boxes(g, out);
    crate::wallskin::wall_skin_boxes(g, out);   // (…and what stands of a holed wall's paint: wallskin.ts)
}

/// Level reset: every hidden static back as authored, pools emptied, heat, caps and the counter back to the start.   (destruction.ts Destruction.reset)
pub fn destruction_reset(g: &mut crate::game::Game) {
    for i in g.destr.hidden.keys() {
        let f = *g.destr.hidden.get(i).unwrap();
        seam_box(g, BoxId::from_ix(i.0 as usize)).flags = f;
    }
    crate::scene_collision::wrote(g);
    g.destr.hidden.clear();
    crate::debris::debris_clear(g);
    for x in g.destr.heat.iter_mut() {
        *x = 0.0;
    }
    g.destr.any_heat = false;
    g.destr.tiles_down.clear();
    g.destr.last_tile_t = -100.0;
    g.destr.tile_count = 0.0;
    g.destr.serial = 1.0;
    g.destr.knocks = 0.0;
    rate_cap_reset(&mut g.destr.caps.puff);
    rate_cap_reset(&mut g.destr.caps.paper);
    rate_cap_reset(&mut g.destr.caps.knock);
    rate_cap_reset(&mut g.destr.caps.decal);
    g.destr.free_knocks = 0.0;
    for t in g.destr.broke.keys() {
        crate::sim_lightshome::light_home_in(g, crate::sim_lightshome::light_in().mend, Some(t), None, None, None, None);
    }
    g.destr.broke.clear();
    g.destr.fixture_count = 0.0;
    g.destr.last_fixture_t = -100.0;
    g.destr.dust_queue.clear();
    g.destr.pall_at.clear();
    crate::break_table::breaks_reset(g);
    crate::edge_look::edge_look_clear(g);
    crate::wallbreak::wall_break_reset(g, None);
    crate::furnblast::furniture_reset(g);
}

/// The tour's cuts: nothing tumbling carries over; hidden statics stay hidden.   (destruction.ts Destruction.clearMoving)
pub fn destruction_clear_moving(g: &mut crate::game::Game) {
    crate::debris::debris_clear(g);
    g.destr.dust_queue.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_cap_holds_n_in_any_second() {
        let mut c = RateCap::blank();
        assert!(rate_cap_take_n(&mut c, 0.0, 2.0));
        assert!(rate_cap_take_n(&mut c, 0.5, 2.0));
        assert!(!rate_cap_take_n(&mut c, 0.9, 2.0));
        assert!(rate_cap_take_n(&mut c, 1.0, 2.0));
        assert!(!rate_cap_take_n(&mut c, 1.4, 2.0));
        assert!(rate_cap_take_n(&mut c, 1.5, 2.0));
        // a floored knob of nothing refuses; a changed cap starts the window afresh
        assert!(!rate_cap_take_n(&mut c, 9.0, 0.5));
        assert!(rate_cap_take_n(&mut c, 1.6, 3.0));
        assert_eq!(c.ts.len(), 3);
        assert_eq!(c.head, 1.0);
        rate_cap_reset(&mut c);
        assert_eq!(c.head, 0.0);
        assert!(c.ts.iter().all(|&t| t == -1e9));
    }

    #[test]
    fn rate_cap_reads_its_own_knob_live() {
        let mut k = crate::knobs::ChaosK::zeroed();
        k.papers_per_sec = 1.0;
        k.puffs_per_sec = 5.0;
        let mut c = rate_cap_new(RateCapN::Paper);
        assert!(rate_cap_take(&mut c, 0.0, &k));
        assert!(!rate_cap_take(&mut c, 0.5, &k));
        // the panel raised the cap: the window starts afresh
        k.papers_per_sec = 2.0;
        assert!(rate_cap_take(&mut c, 0.6, &k));
        assert_eq!(c.ts.len(), 2);
    }

}
