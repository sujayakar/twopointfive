//! src/game/destruction.ts lines 764 to 891
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A broken cell's dust: the puff now if the caps allow, else it waits in the burst queue. No dice.   (destruction.ts cellDust)
pub fn destruction_cell_dust(g: &mut crate::game::Game, p: V3, n: V3, scale: f64, surf: crate::scene_materials::Surf) {
    if chaos_of(g) <= 0.0 { return; }
    let r0 = g.stats.destruction.destr_puffs_refused;
    if destruction_dust_puff(g, p, n, scale, surf) { return; }
    if (g.destr.dust_queue.len() as f64) >= g.k.chaos.loud.burst_queue { return; }   // (counted refused by dust_puff)
    g.stats.destruction.destr_puffs_refused = r0;
    let t = g.time;
    g.destr.dust_queue.push(DestructionDustQueueItem { p: v3::copy(p), n: v3::copy(n), scale, surf, t });
    g.destr.any_heat = true;
}

/// A lot of dust from one event: `count` puffs within `spread` of `at`, queued for the frames ahead; the dice move whether or not the queue has room.   (destruction.ts dustBurst)
pub fn destruction_dust_burst(g: &mut crate::game::Game, at: V3, n: V3, count: f64, scale: f64, surf: crate::scene_materials::Surf, spread: Option<f64>) {
    let spread = spread.unwrap_or(0.5);
    if chaos_of(g) <= 0.0 { return; }
    let nn = v3::normalize(n);
    let a: V3 = if m::abs(nn[1]) > 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let u = v3::normalize(v3::cross(nn, a));
    let w = v3::cross(nn, u);
    let mut k = 0.0;
    while k < count {
        let r = spread * m::sqrt(destruction_hash(g));
        let th = destruction_hash(g) * m::PI * 2.0;
        k += 1.0;
        if (g.destr.dust_queue.len() as f64) >= g.k.chaos.loud.burst_queue {
            g.stats.destruction.destr_puffs_refused += 1;
            continue;
        }
        let mut p = v3::add(at, v3::add(v3::scale(u, m::cos(th) * r), v3::scale(w, m::sin(th) * r)));
        p[1] = m::max(0.06, p[1]);
        let t = g.time;
        g.destr.dust_queue.push(DestructionDustQueueItem { p, n: nn, scale, surf, t });
    }
    g.destr.any_heat = true;   // (update runs on while anything waits)
}

/// Small statics near the hit come off the desk.   (destruction.ts clutterNear)
pub fn destruction_clutter_near(g: &mut crate::game::Game, hit_p: V3, dir: V3, chaos: f64, who: f64) {
    let mut list: Vec<Destructible> = Vec::new();
    let near = destruction_near(g, hit_p, &mut list);
    for d in near.iter() {
        if d.kind != DKind::Clutter && d.kind != DKind::Binder { continue; }
        let b = d.box_;
        let (c, h) = (g.col.boxes[b].c, g.col.boxes[b].h);
        if m::hyp3(c[0] - hit_p[0], (c[1] - hit_p[1]) * 0.7, c[2] - hit_p[2]) > g.k.chaos.knock_reach_m + m::max(h[0], h[2]) { continue; }
        let top = top_of(g, b);
        if crate::scene_collision::solid_blocked(g, v3::mad(hit_p, dir, -0.05), top, None) { continue; }   // (not through the partition the round stopped in)
        let dirn = v3::normalize([c[0] - hit_p[0] + dir[0] * 0.5, 0.0, c[2] - hit_p[2] + dir[2] * 0.5]);
        destruction_knock_off(g, d, dirn, chaos, knock_kind(d.kind), Some(who));
    }
}

/// Hide the static and hand its box to the debris pool with a shove along `dirn`; over the pool's cap it becomes a burst of chips. Its group rides on the same slot.   (destruction.ts knockOff)
pub fn destruction_knock_off(g: &mut crate::game::Game, d: &crate::destruction::Destructible, dirn: V3, chaos: f64, kind: crate::debris::ChunkKind, who: Option<f64>) -> bool {
    let who = who.unwrap_or(crate::consts::NOBODY_ID);
    if g.destr.hidden.has(crate::Num(d.i)) || !destruction_take(g, DestructionTakeK::Knock) { return false; }
    let b = d.box_;
    let v_k = g.k.chaos.vel_low * (if chaos <= 0.5 { 1.0 } else { 1.0 + (g.k.chaos.vel_high_k - 1.0) * (chaos - 0.5) * 2.0 });
    let hz = v3::normalize([dirn[0], 0.0, dirn[2]]);
    let v: V3 = [
        hz[0] * v_k * (0.8 + 0.6 * destruction_hash(g)),
        m::sqrt(v_k) * (0.9 + 0.8 * destruction_hash(g)),
        hz[2] * v_k * (0.8 + 0.6 * destruction_hash(g)),
    ];   // (the hop grows slower than the shove)
    let w: V3 = [
        (destruction_hash(g) - 0.5) * 10.0 * v_k,
        (destruction_hash(g) - 0.5) * 6.0,
        (destruction_hash(g) - 0.5) * 10.0 * v_k,
    ];
    let c = crate::debris::debris_spawn(g, kind, b, v, w, Some(who), None);
    destruction_hide(g, d);
    if c.is_none() {   // the pool is full of moving chunks: it still goes, as a burst of chips of its colour
        g.stats.destruction.destr_knocks_demoted += 1;
        if let Some(mut p) = crate::particle_fx::gpu_fx(g) {
            let (bc, bh, albedo) = (g.col.boxes[b].c, g.col.boxes[b].h, g.col.boxes[b].albedo);
            let size = m::max(0.008, m::min_n(&[bh[0], bh[1], bh[2]]) * 0.6);
            seam_emit_debris(g, &mut p, v3::copy(bc), v3::normalize([hz[0], 1.0, hz[2]]), 24.0, 2.5, 0.7, 1.2, 16.0, 0.3, size, 0.5, Some(1.5), m::min(bh[0], bh[2]), v3::copy(albedo));
        }
    }
    if kind == crate::debris::ChunkKind::Housing {
        if let Some(t) = d.target { destruction_break_target(g, t); }   // (chunk or chip burst, the light is off the desk either way)
    }
    g.stats.destruction.destr_knocks += 1;
    g.destr.knocks += 1.0;
    dlog(g, &format!("{} {} knocked off{}", kind.as_str(), d.i, if c.is_some() { "" } else { " (demoted)" }));
    if let Some(group) = &d.group {
        for &oi in group.iter() {
            let o = g.destr.items.get(crate::Num(oi)).cloned();
            if let Some(o) = o {
                if !g.destr.hidden.has(crate::Num(oi)) {
                    g.destr.free_knocks += 1.0;
                    destruction_knock_off(g, &o, dirn, chaos, knock_kind(o.kind), Some(who));   // (a pot and its plant are one thing to the cap)
                }
            }
        }
    }
    true
}

/// A tile of the ceiling grid over `p` lets go: a slab under the ceiling, crumbs, dust; or the light panel in that square does.   (destruction.ts tileDrop)
pub fn destruction_tile_drop(g: &mut crate::game::Game, p: V3, chaos: f64, force: Option<bool>, who: Option<f64>, panels: Option<bool>) -> bool {
    let force = force.unwrap_or(false);
    let who = who.unwrap_or(crate::consts::NOBODY_ID);
    let panels = panels.unwrap_or(true);
    if !force && g.time - g.destr.last_tile_t < g.k.chaos.tile_gap_secs { return false; }
    if g.destr.tile_count >= g.k.chaos.tiles_per_encounter {
        g.stats.destruction.destr_tiles_refused += 1;
        return false;
    }
    if destruction_roof_y(g, p[0], p[2]).is_none() { return false; }
    // the square over the beaten spot, jittered a square either way
    let jx = m::round((destruction_hash(g) - 0.5) * 2.4);
    let jz = m::round((destruction_hash(g) - 0.5) * 2.4);
    let tx = m::floor(p[0] / 0.6) + jx;
    let tz = m::floor(p[2] / 0.6) + jz;
    let key = tile_key(tx, tz);
    if g.destr.tiles_down.has(key.clone()) { return false; }
    let cx = (tx + 0.5) * 0.6;
    let cz = (tz + 0.5) * 0.6;
    let roof = match destruction_roof_y(g, cx, cz) {
        Some(r) => r,
        None => return false,   // (the jitter walked it out through the wall)
    };
    let top = m::max(roof, destruction_roof_y(g, p[0], p[2]).unwrap_or(roof));   // (the slab's own underside: a panel in the square answers the roof ray 4 cm low)
    let y = top - 0.03;
    {
        // anything in the tile's own slot under the slab: that square is not a free tile (gather is cell-granular: re-test the footprint)
        let mut scratch: Vec<i32> = vec![0; 32];
        let n = crate::scene_collision::gather(g, cx - 0.29, cz - 0.29, cx + 0.29, cz + 0.29, top - 0.14, top - 0.001, &mut scratch);
        let mut k = 0usize;
        while (k as f64) < n {
            let ix = scratch[k];
            let b = BoxId::from_ix(ix as usize);
            let (yaw, bh, bc) = (g.col.boxes[b].yaw, g.col.boxes[b].h, g.col.boxes[b].c);
            let ac = m::abs(m::cos(yaw));
            let as_ = m::abs(m::sin(yaw));
            let ex = ac * bh[0] + as_ * bh[2];
            let ez = as_ * bh[0] + ac * bh[2];
            if m::abs(bc[0] - cx) < ex + 0.29 && m::abs(bc[2] - cz) < ez + 0.29 {
                if panels && loud_now(g) {
                    // the loud levels: the square is a light panel's, the PANEL lets go; it takes the tile gap like a tile
                    if let Some(t) = g.destr.target_of_box.get(crate::Num(ix as f64)).copied() {
                        if destruction_fixture_live(g, t) {
                            let ok = destruction_kill_fixture(g, t, p, "shaken", Some(force));
                            if ok { g.destr.last_tile_t = g.time; }
                            return ok;
                        }
                    }
                }
                return false;
            }
            k += 1;
        }
    }
    if crate::scene_collision::solid_blocked(g, [cx, y - 0.05, cz], [cx, y - 0.6, cz], None) { return false; }   // nor with something standing right under it
    let (half, albedo) = (g.k.chaos.tile.half, g.k.chaos.tile.albedo);
    let box_ = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: [cx, y, cz],
        h: v3::copy(half),
        yaw: Some(0.0),
        albedo: Some(v3::copy(albedo)),
        emissive: Some([0.0, 0.0, 0.0]),
        flags: Some(crate::scene_boxes::box_flag().dynamic),
        owner: Some(0.0),
        stuff: Some(crate::scene_stuff::BoxStuff::Paint),
        ..Default::default()
    });   // (TypeScript: boxOf, then `box.stuff = 'paint'`; boxOf has no g here, and makeBox is the same box)
    let v: V3 = [(destruction_hash(g) - 0.5) * 0.4, -0.2, (destruction_hash(g) - 0.5) * 0.4];
    let w: V3 = [(destruction_hash(g) - 0.5) * 3.0, (destruction_hash(g) - 0.5) * 1.0, (destruction_hash(g) - 0.5) * 3.0];
    let c = crate::debris::debris_spawn(g, crate::debris::ChunkKind::Tile, box_, v, w, Some(who), None);
    if c.is_none() && !g.k.debris.dress {   // (dress: the tile is down whether or not the pool had room to show it)
        g.stats.destruction.destr_tiles_refused += 1;
        return false;
    }
    g.destr.tiles_down.add(key.clone());
    g.destr.last_tile_t = g.time;
    g.destr.tile_count += 1.0;
    g.stats.destruction.destr_tiles += 1;
    dlog(g, &format!("tile {} dropped", key));
    if let Some(mut pf) = crate::particle_fx::gpu_fx(g) {
        seam_emit_debris(g, &mut pf, [cx, y - 0.05, cz], [0.0, -1.0, 0.0], 18.0, 1.2, 0.8, 1.3, 20.0, 0.3, 0.009, 0.6, None, 0.25, [0.8, 0.79, 0.75]);   // crumbs of the tile's edge
        seam_emit_dust(g, &mut pf, [cx, y - 0.12, cz], [0.0, -1.0, 0.0], 10.0, 0.7, 0.5, -0.3, 2.2, 0.3, 0.2, 0.35, 0.3, [0.78, 0.76, 0.72]);
    }
    destruction_dust_puff(g, [cx, y - 0.2, cz], [0.0, -1.0, 0.0], 1.4, crate::scene_materials::Surf::Drywall);
    true
}

/// A fixture a stray round, a blast or a shaken ceiling can put out: a lit, interactive lamp or panel, not one already dead or one a fire holds.   (destruction.ts fixtureLive)
pub fn destruction_fixture_live(g: &mut crate::game::Game, t: FixtureId) -> bool {
    use crate::sim_lightshome::{fixture_broken, fixture_factor, fixture_on, FixtureKind};
    !fixture_broken(g, t)
        && g.tab.sim_lightshome_fixture[t].interactive
        && (g.tab.sim_lightshome_fixture[t].kind == FixtureKind::Emissive || g.tab.sim_lightshome_fixture[t].kind == FixtureKind::Analytic)
        && (fixture_on(g, t) || fixture_factor(g, t) > 0.0 || crate::carlamps::lamp_glass(g, t))
        && !crate::fire::fire_holds(g, t)
}

/// Put `t` out: its broken bit, the glass down, counted against the encounter's cap (`force`: a blast ignores the gap, not the cap). Whether a light went.   (destruction.ts killFixture)
pub fn destruction_kill_fixture(g: &mut crate::game::Game, t: FixtureId, from: V3, why: &str, force: Option<bool>) -> bool {
    let force = force.unwrap_or(false);
    if g.destr.fixture_count >= g.k.chaos.loud.fixtures_per_encounter || (!force && g.time - g.destr.last_fixture_t < g.k.chaos.loud.fixture_gap_secs) {
        g.stats.destruction.destr_fixtures_refused += 1;
        return false;
    }
    if !destruction_break_target(g, t) {
        if crate::carlamps::lamp_glass(g, t) && crate::sim_lightshome::fixture_broken(g, t) {   // (a vehicle's dark lens: its glass and its sound, no light gone out)
            let pos = g.tab.sim_lightshome_fixture[t].pos;
            crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(0.8), None);
            lens_shatter(g, t, from);
            let name = g.tab.sim_lightshome_fixture[t].name.clone();
            dlog(g, &format!("lens {} broken ({})", name, why));
        }
        return false;
    }
    g.destr.fixture_count += 1.0;
    g.destr.last_fixture_t = g.time;
    g.stats.destruction.destr_fixtures += 1;
    let name = g.tab.sim_lightshome_fixture[t].name.clone();
    dlog(g, &format!("fixture {} out ({})", name, why));
    let pos = g.tab.sim_lightshome_fixture[t].pos;
    crate::render_lights::light_set_flash(&mut g.engine.lights, pos, [0.7, 0.8, 1.0], 6.0, 4.0, 0.12, 0.0);
    crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(1.0), None);
    if crate::carlamps::lamp_glass(g, t) {
        lens_shatter(g, t, from);
    } else {
        let shard = crate::particle_fx::FixtureShatterFxT {
            pos,
            kind: String::from(g.tab.sim_lightshome_fixture[t].kind.as_str()),
            boxes: g.tab.sim_lightshome_fixture[t].boxes.clone(),
        };
        crate::particle_fx::fixture_shatter_fx(g, &shard, from);
    }
    true
}

/// A round that is not the player's flew muzzle to muzzle + dir * tHit and stopped in static `index`: a fixture it stopped in, or passed within fixtureHitM of, goes dark.   (destruction.ts fixtureRound)
pub fn destruction_fixture_round(g: &mut crate::game::Game, muzzle: V3, dir: V3, t_hit: f64, index: Option<f64>) {
    if !loud_now(g) || chaos_of(g) <= 0.0 { return; }
    if let Some(index) = index {
        if index < 1e6 {
            let t = g.destr.target_of_box.get(crate::Num(index)).copied();
            if let Some(t) = t {
                if crate::fire::fire_holds(g, t) { crate::fire::fire_stir(g, t); return; }
                if destruction_fixture_live(g, t) { destruction_kill_fixture(g, t, muzzle, "round", None); return; }
            }
            let at = v3::mad(muzzle, dir, t_hit);   // (… or in the drum a fire burns in, which is no box of the fire's)
            let mut k = 0;
            while k < g.targets.len() {
                let x = g.targets[k];
                if crate::fire::fire_holds(g, x) && crate::fire::fire_struck(g, x, at, index) {
                    crate::fire::fire_stir(g, x);
                    return;
                }
                k += 1;
            }
        }
    }
    let r = g.k.chaos.loud.fixture_hit_m;
    let mut k = 0;
    while k < g.targets.len() {
        let t = g.targets[k];
        k += 1;
        if !destruction_fixture_live(g, t) { continue; }
        let to_t = v3::sub(g.tab.sim_lightshome_fixture[t].pos, muzzle);
        let depth = v3::dot(to_t, dir);
        if depth < 0.0 || depth > t_hit + r { continue; }
        let along = m::min(depth, t_hit);
        if v3::len(v3::sub(to_t, v3::scale(dir, along))) < r {
            destruction_kill_fixture(g, t, muzzle, "round", None);
            return;
        }
    }
}

/// The panel's one line of what the lane has done.   (destruction.ts summary)
pub fn destruction_summary(g: &mut crate::game::Game) -> String {
    let inv = destruction_inventory(g);
    let ps = match crate::particle_fx::gpu_fx(g) {
        Some(p) => format!(" · particle impulses {} queued / {} merged / {} dropped, {} kicks", p.stats.imp_queued, p.stats.imp_merged, p.stats.imp_dropped, p.stats.kicked),
        None => String::new(),
    };
    let chaos = chaos_of(g);
    let imp = if impulses_on(g) { "" } else { " (impulses off)" };
    let mut heat = 0.0;
    for &h in g.destr.heat.iter() { heat = m::max(heat, h as f64); }
    let debris = crate::debris::debris_summary(g);
    let s = &g.stats.destruction;
    format!(
        "chaos {:.2}{} · heat max {:.1} · {} · papers {} ({} sheets) · puffs {} (−{}) · knocks {} ({} demoted) · screens {} · tiles {} · fixtures {} (−{}) · decals {} · impulses bang {} / impact {} / thud {} (−{}), {} chunks kicked{} · index {}",
        chaos, imp, heat, debris, s.destr_papers, s.destr_sheets, s.destr_puffs, s.destr_puffs_refused, s.destr_knocks, s.destr_knocks_demoted, s.destr_screens, s.destr_tiles,
        s.destr_fixtures, s.destr_fixtures_refused, s.destr_decals, s.destr_imp_bang, s.destr_imp_impact, s.destr_imp_thud, s.destr_imp_thud_dropped, s.destr_debris_kicked, ps, inventory_json(&inv),
    )
}

/// `JSON.stringify` of the inventory's counts, in the order its keys were first set.   (destruction.ts summary)
fn inventory_json(inv: &JsMap<String, f64>) -> String {
    let mut out = String::from("{");
    for (k, v) in inv.iter() {
        if out.len() > 1 { out.push(','); }
        out.push_str(&format!("\"{}\":{}", k, v));
    }
    out.push('}');
    out
}

/// The word a knocked thing's chunk is filed under, for clutter and binders.   (destruction.ts: `d.kind === 'binder' ? 'binder' : 'clutter'`, three sites)
fn knock_kind(kind: DKind) -> crate::debris::ChunkKind {
    if kind == DKind::Binder { crate::debris::ChunkKind::Binder } else { crate::debris::ChunkKind::Clutter }
}

/// The key of a tile's square: JavaScript writes a -0 as "0", Rust's format as "-0", so through an integer.   (destruction.ts tileDrop `${tx},${tz}`)
fn tile_key(tx: f64, tz: f64) -> String {
    format!("{},{}", tx as i64, tz as i64)
}

/// A vehicle lamp's glass, from the fixture's row.   (destruction.ts killFixture, two sites)
fn lens_shatter(g: &mut Game, t: FixtureId, from: V3) {
    let lens = crate::particle_fx::LensShatterFxT { pos: g.tab.sim_lightshome_fixture[t].pos, lens: g.tab.sim_lightshome_fixture[t].lens };
    crate::particle_fx::lens_shatter_fx(g, lens, from);
}

/// `P.emit('debris', pos, dir, count, { speed, speedJit, spread, life, lifeJit, size, sizeJit, aspect, posJit, color })` (destruction.ts:812,850).
fn seam_emit_debris(g: &mut Game, p: &mut crate::render_particles::Particles, pos: V3, dir: V3, count: f64, speed: f64, speed_jit: f64, spread: f64, life: f64, life_jit: f64, size: f64, size_jit: f64, aspect: Option<f64>, pos_jit: f64, color: V3) { todo!("needs: EmitOpts.speedJit, spread, lifeJit, sizeJit, aspect, color") }
/// `emitDust(P, pos, dir, count, { shape: 'puff', speed, speedJit, lift, life, lifeJit, size, sizeJit, posJit, color })` (destruction.ts:851).
fn seam_emit_dust(g: &mut Game, p: &mut crate::render_particles::Particles, pos: V3, dir: V3, count: f64, speed: f64, speed_jit: f64, lift: f64, life: f64, life_jit: f64, size: f64, size_jit: f64, pos_jit: f64, color: V3) { todo!("needs: EmitOpts.shape, speedJit, lifeJit, sizeJit, color") }

#[cfg(test)]
mod tests {
    use super::*;

    /// Math.round(-0.3) is -0 and Math.floor(-0) is -0, and `${-0}` is "0": the square must be one key either way.
    /// What JSON.stringify writes of `{ screen: 3, clutter: 12 }`, and of nothing.
    #[test]
    fn the_inventory_writes_as_json() {
        let mut inv: JsMap<String, f64> = JsMap::new();
        assert_eq!(inventory_json(&inv), "{}");
        inv.set(String::from("screen"), 3.0);
        inv.set(String::from("clutter"), 12.0);
        assert_eq!(inventory_json(&inv), "{\"screen\":3,\"clutter\":12}");
    }

    #[test]
    fn a_tile_key_has_no_negative_zero() {
        assert_eq!(tile_key(-0.0, 0.0), "0,0");
        assert_eq!(tile_key(-3.0, 12.0), "-3,12");
    }
}
