use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A broken cell's dust: the puff now if the caps allow, else it waits in the burst queue. No dice.
pub(in crate::breakage) fn destruction_cell_dust(g: &mut crate::game::Game, p: V3, n: V3, scale: f64, surf: crate::scene_materials::Surf) {
    if chaos_of(g) <= 0.0 { return; }
    let r0 = g.stats.destruction.destr_puffs_refused;
    if destruction_dust_puff(g, p, n, scale, surf) { return; }
    if (g.destr.dust_queue.len() as f64) >= g.k.chaos.loud.burst_queue { return; }   // (counted refused by dust_puff)
    g.stats.destruction.destr_puffs_refused = r0;
    let t = g.time;
    g.destr.dust_queue.push(DestructionDustQueueItem { p: v3::copy(p), n: v3::copy(n), scale, surf, t });
    g.destr.any_heat = true;
}

/// A lot of dust from one event: `count` puffs within `spread` of `at`, queued for the frames ahead; the dice move whether or not the queue has room.
pub(super) fn destruction_dust_burst(g: &mut crate::game::Game, at: V3, n: V3, count: f64, scale: f64, surf: crate::scene_materials::Surf, spread: Option<f64>) {
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

/// Small statics near the hit come off the desk.
pub(super) fn destruction_clutter_near(g: &mut crate::game::Game, hit_p: V3, dir: V3, chaos: f64, who: f64) {
    let mut list: Vec<Destructible> = Vec::new();
    let near = destruction_near(g, hit_p, &mut list);
    for d in near.iter() {
        if d.kind != DKind::Clutter && d.kind != DKind::Binder { continue; }
        let b = d.box_;
        let (c, h) = (seam_box(g, b).c, seam_box(g, b).h);
        if m::hyp3(c[0] - hit_p[0], (c[1] - hit_p[1]) * 0.7, c[2] - hit_p[2]) > g.k.chaos.knock_reach_m + m::max(h[0], h[2]) { continue; }
        let top = top_of(g, b);
        if crate::scene_collision::solid_blocked(g, v3::mad(hit_p, dir, -0.05), top, None) { continue; }   // (not through the partition the round stopped in)
        let dirn = v3::normalize([c[0] - hit_p[0] + dir[0] * 0.5, 0.0, c[2] - hit_p[2] + dir[2] * 0.5]);
        destruction_knock_off(g, d, dirn, chaos, knock_kind(d.kind), Some(who));
    }
}

/// Hide the static and hand its box to the debris pool with a shove along `dirn`; over the pool's cap it becomes a burst of chips. Its group rides on the same slot.
pub(super) fn destruction_knock_off(g: &mut crate::game::Game, d: &crate::destruction::Destructible, dirn: V3, chaos: f64, kind: crate::debris::ChunkKind, who: Option<f64>) -> bool {
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
    if c.is_none() {   
        let (bc, bh, albedo) = (seam_box(g, b).c, seam_box(g, b).h, seam_box(g, b).albedo);
        crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, v3::copy(bc), v3::normalize([hz[0], 1.0, hz[2]]), 24.0, crate::render_particles::EmitOpts {
            speed: 2.5,
            speed_jit: Some(0.7),
            spread: Some(1.2),
            life: 16.0,
            life_jit: Some(0.3),
            size: m::max(0.008, m::min_n(&[bh[0], bh[1], bh[2]]) * 0.6),
            size_jit: Some(0.5),
            aspect: Some(1.5),
            pos_jit: Some(m::min(bh[0], bh[2])),
            color: Some(v3::copy(albedo)),
            ..Default::default()
        });
    }
    if kind == crate::debris::ChunkKind::Housing {
        if let Some(t) = d.target { destruction_break_target(g, t); }   // (chunk or chip burst, the light is off the desk either way)
    }
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

/// A tile of the ceiling grid over `p` lets go: a slab under the ceiling, crumbs, dust; or the light panel in that square does.
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
            let b = g.col.boxes[ix as usize];
            let (yaw, bh, bc) = (seam_box(g, b).yaw, seam_box(g, b).h, seam_box(g, b).c);
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
    crate::debris::debris_spawn(g, crate::debris::ChunkKind::Tile, box_, v, w, Some(who), None);
    g.destr.tiles_down.add(key.clone());
    g.destr.last_tile_t = g.time;
    g.destr.tile_count += 1.0;
    dlog(g, &format!("tile {} dropped", key));
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, [cx, y - 0.05, cz], [0.0, -1.0, 0.0], 18.0, crate::render_particles::EmitOpts {   // crumbs of the tile's edge
        speed: 1.2,
        speed_jit: Some(0.8),
        spread: Some(1.3),
        life: 20.0,
        life_jit: Some(0.3),
        size: 0.009,
        size_jit: Some(0.6),
        pos_jit: Some(0.25),
        color: Some([0.8, 0.79, 0.75]),
        ..Default::default()
    });
    crate::particle_fx::emit_dust(g, [cx, y - 0.12, cz], [0.0, -1.0, 0.0], 10.0, crate::render_particles::EmitOpts {
        shape: Some(crate::render_particles::EmitShape::Puff),
        speed: 0.7,
        speed_jit: Some(0.5),
        lift: Some(-0.3),
        life: 2.2,
        life_jit: Some(0.3),
        size: 0.2,
        size_jit: Some(0.35),
        pos_jit: Some(0.3),
        color: Some([0.78, 0.76, 0.72]),
        ..Default::default()
    });
    destruction_dust_puff(g, [cx, y - 0.2, cz], [0.0, -1.0, 0.0], 1.4, crate::scene_materials::Surf::Drywall);
    true
}

/// A fixture a stray round, a blast or a shaken ceiling can put out: a lit, interactive lamp or panel, not one already dead or one a fire holds.
pub(super) fn destruction_fixture_live(g: &crate::game::Game, t: FixtureId) -> bool {
    use crate::fixtures::FixtureKind;
    !g.tab.fixture[t].broken
        && g.tab.fixture[t].interactive
        && (g.tab.fixture[t].kind == FixtureKind::Emissive || g.tab.fixture[t].kind == FixtureKind::Analytic)
        && (g.tab.fixture[t].on || g.tab.fixture[t].factor > 0.0 || crate::carlamps::lamp_glass(g, t))
        && !crate::fire::fire_holds(g, t)
}

/// Put `t` out: its broken bit, the glass down, counted against the encounter's cap (`force`: a blast ignores the gap, not the cap). Whether a light went.
pub(super) fn destruction_kill_fixture(g: &mut crate::game::Game, t: FixtureId, from: V3, why: &str, force: Option<bool>) -> bool {
    let force = force.unwrap_or(false);
    if g.destr.fixture_count >= g.k.chaos.loud.fixtures_per_encounter || (!force && g.time - g.destr.last_fixture_t < g.k.chaos.loud.fixture_gap_secs) {
        return false;
    }
    if !destruction_break_target(g, t) {
        if crate::carlamps::lamp_glass(g, t) && g.tab.fixture[t].broken {   // (a vehicle's dark lens: its glass and its sound, no light gone out)
            let pos = g.tab.fixture[t].pos;
            crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(0.8), None);
            lens_shatter(g, t, from);
            let name = g.tab.fixture[t].name.clone();
            dlog(g, &format!("lens {} broken ({})", name, why));
        }
        return false;
    }
    g.destr.fixture_count += 1.0;
    g.destr.last_fixture_t = g.time;
    let name = g.tab.fixture[t].name.clone();
    dlog(g, &format!("fixture {} out ({})", name, why));
    let pos = g.tab.fixture[t].pos;
    crate::render_lights::light_set_flash(g, pos, [0.7, 0.8, 1.0], 6.0, 4.0, 0.12, 0.0);
    crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(1.0), None);
    if crate::carlamps::lamp_glass(g, t) {
        lens_shatter(g, t, from);
    } else {
        let shard = crate::particle_fx::FixtureShatterFxT {
            pos,
            kind: String::from(g.tab.fixture[t].kind.as_str()),
            boxes: g.tab.fixture[t].boxes.clone(),
        };
        crate::particle_fx::fixture_shatter_fx(g, &shard, from);
    }
    true
}

/// A round that is not the player's flew muzzle to muzzle + dir * tHit and stopped in static `index`: a fixture it stopped in, or passed within fixtureHitM of, goes dark.
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
        let to_t = v3::sub(g.tab.fixture[t].pos, muzzle);
        let depth = v3::dot(to_t, dir);
        if depth < 0.0 || depth > t_hit + r { continue; }
        let along = m::min(depth, t_hit);
        if v3::len(v3::sub(to_t, v3::scale(dir, along))) < r {
            destruction_kill_fixture(g, t, muzzle, "round", None);
            return;
        }
    }
}

/// The word a knocked thing's chunk is filed under, for clutter and binders.   (destruction.ts: `d.kind === 'binder' ? 'binder' : 'clutter'`, three sites)
fn knock_kind(kind: DKind) -> crate::debris::ChunkKind {
    if kind == DKind::Binder { crate::debris::ChunkKind::Binder } else { crate::debris::ChunkKind::Clutter }
}

/// The key of a tile's square, in the language's own words for a number (a -0 is "0").
fn tile_key(tx: f64, tz: f64) -> String {
    format!("{},{}", js::num_str(tx), js::num_str(tz))
}

/// A vehicle lamp's glass, from the fixture's row.
fn lens_shatter(g: &mut Game, t: FixtureId, from: V3) {
    let lens = crate::particle_fx::LensShatterFxT { pos: g.tab.fixture[t].pos, lens: g.tab.fixture[t].lens };
    crate::particle_fx::lens_shatter_fx(g, lens, from);
}

/// a box's row
fn seam_box(g: &mut Game, b: BoxId) -> &mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

#[cfg(test)]
mod tests {
    use super::*;

    /// A tile that has dropped stays dropped: a square by the wall is one key, whichever side of nought it rounds from.
    #[test]
    fn a_tile_s_square_is_one_key_either_side_of_nought() {
        assert_eq!(tile_key(-0.0, 0.0), "0,0");
        assert_eq!(tile_key(-3.0, 12.0), "-3,12");
    }
}
