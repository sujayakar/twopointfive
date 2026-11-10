pub use ::game_types::particle_fx::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec::Vec};

// ---------------------------------------------------------------- dust: every puff goes through one table

/// THE dust emit: the site's numbers times the table; the count rounded, never below 1 where a site asked for any.
pub(super) fn emit_dust(g: &mut crate::game::Game, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) -> f64 {
    match dust_asked(g, count, o) { Some((n, o)) => crate::render_particles::particles_emit(g, crate::render_particles::PKind::Dust, pos, dir, n, o), None => 0.0 }
}

/// How many puffs a site's asking for `count` comes to, and its numbers, by the table; none = no dust at all.
pub(super) fn dust_asked(g: &crate::game::Game, count: f64, o: crate::render_particles::EmitOpts) -> Option<(f64, crate::render_particles::EmitOpts)> {
    let k = g.m.particle_fx.dust;
    let n = dust_count(k.count, count);
    if n <= 0.0 { return None; }
    if dust_shipped(&k) { return Some((n, o)); }
    Some((n, Emit {
        size: o.size * k.size,
        life: o.life * k.life,
        speed: o.speed * k.speed,
        lift: o.lift.map(|x| x * k.rise),
        pos_jit: o.pos_jit.map(|x| x * k.spread),
        vadd: o.vadd.map(|v| v3::scale(v, k.speed)),
        ..o
    }))
}

/// How many puffs: × 0 = none anywhere; any other factor keeps at least one where a site asked for some.
fn dust_count(factor: f64, count: f64) -> f64 {
    if count > 0.0 && factor > 0.0 { m::max(1.0, m::round(count * factor)) } else { 0.0 }
}

/// The shipped table: the site's own record goes untouched.
fn dust_shipped(k: &crate::particle_fx::ParticleFxModDUST) -> bool {
    k.size == 1.0 && k.life == 1.0 && k.speed == 1.0 && k.rise == 1.0 && k.spread == 1.0
}

// ---------------------------------------------------------------- what a hit is made of

/// What a round knocks out of a static: its word's surface, in any weather.
pub fn surf_at(g: &crate::game::Game, b: BoxId) -> crate::scene_materials::Surf {
    crate::scene_materials::does_of(g, g.tab.boxes[b].does).surf
}

/// What `b` says it is, for the picture of a hit; None = the rule is off, it says nothing, its word is not one thing to a round, or it is a pane.
pub(super) fn said_surf(g: &crate::game::Game, b: Option<BoxId>) -> Option<crate::scene_materials::Surf> {
    let b = b?;
    let stuff = g.tab.boxes[b].stuff?;
    if is_pane(g.tab.boxes[b].flags) { return None; }
    surf_of_stuff(stuff)
}

/// What a hit looks like: the surface its sparks, chips and dust go by, the colour they are tinted from, and whether it is solid glass. The box's own word first,
/// then what the hit has always been read by: a prop's kind, a leaf as wood, a pane as glass, what a static does.
pub(super) fn hit_look(g: &crate::game::Game, hit: &crate::combat::RoundMet) -> crate::particle_fx::HitLook {
    let leaf = hit.prop.is_none() && hit.index.is_some_and(|i| i >= 1e6);
    let st = if hit.prop.is_none() && !leaf { hit.index.and_then(|i| js_ix(g.col.boxes.len(), i)).map(|x| g.col.boxes[x]) } else { None };
    let b: Option<BoxId> = if let Some(p) = hit.prop {
        hit.box_.or_else(|| g.tab.props_prop[p].boxes.first().copied())
    } else if leaf {
        let i = hit.index.unwrap() - 1e6;
        js_ix(g.col.dynamic_boxes.len(), i).map(|x| g.col.dynamic_boxes[x])
    } else {
        st
    };
    if let Some(said) = said_surf(g, b) {
        let albedo = g.tab.boxes[b.unwrap()].albedo;
        return crate::particle_fx::HitLook { surf: said, albedo, solid: said == crate::scene_materials::Surf::Glass, said: true };
    }
    if let Some(p) = hit.prop {
        let surf = surf_of_prop(g.tab.props_prop[p].def.kind);
        let albedo = match g.tab.props_prop[p].boxes.first().copied() { Some(x) => g.tab.boxes[x].albedo, None => [0.5, 0.5, 0.5] };
        return crate::particle_fx::HitLook { surf, albedo, solid: false, said: false };
    }
    if leaf {
        let albedo = match b { Some(x) => g.tab.boxes[x].albedo, None => [0.42, 0.3, 0.19] };
        return crate::particle_fx::HitLook { surf: crate::scene_materials::Surf::Wood, albedo, solid: false, said: false };
    }
    if let Some(st) = st {
        let surf = if is_pane(g.tab.boxes[st].flags) { crate::scene_materials::Surf::Glass } else { surf_at(g, st) };
        let albedo = g.tab.boxes[st].albedo;
        return crate::particle_fx::HitLook { surf, albedo, solid: false, said: false };
    }
    crate::particle_fx::HitLook { surf: crate::scene_materials::Surf::Concrete, albedo: [0.5, 0.5, 0.5], solid: false, said: false }
}

/// flags & BoxFlag.Glass
fn is_pane(flags: f64) -> bool {
    js::to_i32(flags) & js::to_i32(crate::scene_boxes::box_flag().glass) != 0
}

/// STUFF_SURF by the word; None = it is not one thing to a round. Exhaustive, so a new word fails to compile here. The test holds it to the skeleton's table.
fn surf_of_stuff(s: crate::scene_stuff::BoxStuff) -> Option<crate::scene_materials::Surf> {
    use crate::scene_stuff::BoxStuff;
    match s {
        BoxStuff::Carpaint | BoxStuff::Steel | BoxStuff::Metal | BoxStuff::Sheet | BoxStuff::Brushed | BoxStuff::Louvre => Some(crate::scene_materials::Surf::Metal),
        BoxStuff::Glass => Some(crate::scene_materials::Surf::Glass),
        BoxStuff::Rubber | BoxStuff::Plastic => Some(crate::scene_materials::Surf::Plastic),
        BoxStuff::Wood | BoxStuff::Veneer => Some(crate::scene_materials::Surf::Wood),
        BoxStuff::Fabric => Some(crate::scene_materials::Surf::Fabric),
        BoxStuff::Cloth | BoxStuff::Carpet => Some(crate::scene_materials::Surf::Carpet),
        BoxStuff::Plaster | BoxStuff::Skim => Some(crate::scene_materials::Surf::Drywall),
        BoxStuff::Stucco | BoxStuff::MudWall | BoxStuff::Block | BoxStuff::GlossBlock | BoxStuff::BoardConcrete | BoxStuff::CastPanel | BoxStuff::Slab | BoxStuff::Paver | BoxStuff::Tarmac => Some(crate::scene_materials::Surf::Concrete),
        BoxStuff::Grass | BoxStuff::Enamel | BoxStuff::Paint | BoxStuff::Earth | BoxStuff::Gravel | BoxStuff::Plain | BoxStuff::Waits => None,
    }
}

/// PROP_SURF by the kind. The test holds it to the skeleton's table.
fn surf_of_prop(kind: crate::scene_level_data::PropKindName) -> crate::scene_materials::Surf {
    use crate::scene_level_data::PropKindName;
    match kind {
        PropKindName::Chair => crate::scene_materials::Surf::Fabric,
        PropKindName::Cardboard => crate::scene_materials::Surf::Cardboard,
        PropKindName::Bin => crate::scene_materials::Surf::Metal,
        PropKindName::Plant => crate::scene_materials::Surf::Foliage,
    }
}

/// `list[i]`'s index as JavaScript reads it: anything but a whole number in range is undefined.
fn js_ix(len: usize, i: f64) -> Option<usize> {
    if i >= 0.0 && i < len as f64 && m::floor(i) == i { Some(i as usize) } else { None }
}

// ---------------------------------------------------------------- the sites

/// A round stopped in something solid: what it knocks out, by what it hit.
pub fn impact_fx(g: &mut crate::game::Game, hit_p: V3, dir: V3, hit: crate::combat::RoundMet, def: &crate::weapons::WeaponDef) {
    use crate::particle_fx::{ImpactRecipeChipTint as Chip, ImpactRecipeDustTint as Puff};
    let look = hit_look(g, &hit);
    let (surf, albedo, solid) = (look.surf, look.albedo, look.solid);
    let r = impact_row(surf);
    let pfx = g.m.particle_fx.pfx;
    let chips = if surf == crate::scene_materials::Surf::Metal { 0.0 } else if solid { SOLID_GLASS_CHIPS } else { r.chips };
    let k = if def.cls == crate::weapons::WeaponClass::Pistol { 1.0 } else { pfx.impact_scale_rifle };
    let n = v3::normalize(hit.n);
    let dn = v3::dot(dir, n);
    let refl = v3::normalize(v3::sub(dir, v3::scale(n, 2.0 * dn)));
    let at = v3::mad(hit_p, n, 0.015);
    if r.sparks > 0.0 {
        crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, at, v3::normalize(v3::add(v3::scale(refl, 0.6), v3::scale(n, 0.4))), m::round(r.sparks * k), Emit {
            speed: r.spark_speed.unwrap_or(5.0), speed_jit: Some(0.6), spread: Some(pfx.spark_spread), life: pfx.spark_life, life_jit: Some(0.4), size: pfx.spark_size, ..Emit::default()
        });
    }
    if chips > 0.0 {
        let tint: V3 = match r.chip_tint { Chip::Albedo => albedo, Chip::Pale => v3::lerp3(albedo, [0.86, 0.85, 0.82], 0.55), Chip::List(c) => c };
        crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, at, v3::normalize(v3::add(n, v3::scale(refl, 0.3))), m::round(chips * k), Emit {
            speed: pfx.chip_speed, speed_jit: Some(0.7), spread: Some(pfx.chip_spread), life: pfx.chip_life, life_jit: Some(0.3), size: r.chip_size.unwrap_or(pfx.chip_size), size_jit: Some(0.5), color: Some(tint), ..Emit::default()
        });
    }
    if r.dust > 0.0 {
        let tint: V3 = match r.dust_tint { Puff::Albedo => v3::lerp3(albedo, [0.5, 0.5, 0.5], 0.5), Puff::List(c) => c };
        emit_dust(g, v3::mad(hit_p, n, 0.07), n, m::round(r.dust * k), Emit {
            shape: Some(crate::render_particles::EmitShape::Puff), speed: pfx.dust_speed, speed_jit: Some(0.5), lift: Some(0.05), life: pfx.dust_life, life_jit: Some(0.3), size: pfx.dust_size, size_jit: Some(0.35), pos_jit: Some(0.04), vadd: Some(v3::scale(n, 0.45)), color: Some(tint), ..Emit::default()
        });
    }
}

/// IMPACT[surf]: the table has every word of Surf.
fn impact_row(surf: crate::scene_materials::Surf) -> crate::particle_fx::ImpactRecipe {
    *crate::particle_fx::impact().get(String::from(surf.as_str())).unwrap()
}

/// The muzzle's unburnt powder.
pub fn muzzle_sparks_fx(g: &mut crate::game::Game, muzzle: V3, dir: V3, shooter: CharId) {
    let s = crate::effects::shot();
    if !crate::effects::flags().shot_sparks || s.spark_count <= 0.0 { return; }
    let d = v3::normalize(dir);
    let (vel, id) = (g.ch[shooter].vel, g.ch[shooter].id);
    let size = g.m.particle_fx.pfx.muzzle_spark_size;
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, v3::mad(muzzle, d, 0.04), [d[0], d[1] + 0.08, d[2]], s.spark_count, Emit {
        speed: s.spark_speed, speed_jit: Some(0.55), spread: Some(s.spark_cone), life: s.spark_life, life_jit: Some(0.35), size, vadd: Some(vel), owner: Some(id), ..Emit::default()
    });
}

/// The stun charge coming apart: the fragments, a ring of dust along the floor, a puff over the can.
pub fn bang_fx(g: &mut crate::game::Game, at: V3) {
    let s = crate::effects::sparks();
    let pfx = g.m.particle_fx.pfx;
    if !crate::blast_look::blast_sparks(g, at) {
        crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, at, [0.0, 1.0, 0.0], pfx.bang_sparks, Emit {
            shape: Some(crate::render_particles::EmitShape::Burst), spread: Some(s.spread), lift: Some(s.lift), speed: s.speed, speed_jit: Some(0.55), life: s.life, life_jit: Some(0.35), size: pfx.bang_spark_size, ..Emit::default()
        });
    }
    emit_dust(g, [at[0], 0.1, at[2]], [0.0, 1.0, 0.0], pfx.bang_ring, Emit {
        shape: Some(crate::render_particles::EmitShape::Ring), radius: Some(pfx.bang_ring_radius), speed: pfx.bang_ring_speed, speed_jit: Some(0.4), lift: Some(0.12), spread: Some(0.3), life: pfx.bang_ring_life, life_jit: Some(0.3), size: pfx.bang_ring_size, size_jit: Some(0.3), color: Some([0.7, 0.68, 0.64]), ..Emit::default()
    });
    emit_dust(g, v3::add(at, [0.0, 0.15, 0.0]), [0.0, 1.0, 0.0], pfx.bang_puff, Emit {
        shape: Some(crate::render_particles::EmitShape::Puff), speed: 1.2, speed_jit: Some(0.5), lift: Some(0.4), life: 2.0, life_jit: Some(0.3), size: 0.22, size_jit: Some(0.3), pos_jit: Some(0.1), color: Some([0.62, 0.6, 0.57]), ..Emit::default()
    });
}

/// A boot through the lock: plaster dust and splinters off the jamb, blown away from the kicker.
pub fn kick_dust_fx(g: &mut crate::game::Game, d: DoorId, from: V3) {
    let (hinge, width, cd) = (g.dr[d].def.hinge, g.dr[d].def.width, g.dr[d].def.closed_dir);
    let pfx = g.m.particle_fx.pfx;
    let u: V3 = [m::cos(cd), 0.0, m::sin(cd)];
    let jamb: V3 = [hinge[0] + u[0] * (width - 0.06), 1.0, hinge[1] + u[2] * (width - 0.06)];
    let mut mv: V3 = [-u[2], 0.0, u[0]];
    if v3::dot(mv, v3::sub(from, jamb)) > 0.0 { mv = v3::scale(mv, -1.0); }
    emit_dust(g, v3::mad(jamb, mv, 0.1), mv, pfx.kick_dust, Emit {
        shape: Some(crate::render_particles::EmitShape::Puff), speed: 1.1, speed_jit: Some(0.5), lift: Some(0.1), life: 1.6, life_jit: Some(0.3), size: 0.14, size_jit: Some(0.35), pos_jit: Some(0.18), vadd: Some(v3::scale(mv, 1.3)), color: Some([0.78, 0.75, 0.7]), ..Emit::default()
    });
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, v3::mad(jamb, mv, 0.05), mv, pfx.kick_splinters, Emit {
        speed: 3.2, speed_jit: Some(0.7), spread: Some(0.7), life: 16.0, life_jit: Some(0.3), size: 0.012, size_jit: Some(0.6), color: Some([0.62, 0.48, 0.3]), ..Emit::default()
    });
    emit_dust(g, [jamb[0] + mv[0] * 0.3, 0.08, jamb[2] + mv[2] * 0.3], [0.0, 1.0, 0.0], 5.0, Emit {
        shape: Some(crate::render_particles::EmitShape::Puff), speed: 0.8, speed_jit: Some(0.5), lift: Some(0.3), life: 1.4, life_jit: Some(0.3), size: 0.18, size_jit: Some(0.3), pos_jit: Some(0.25), vadd: Some(v3::scale(mv, 0.8)), color: Some([0.6, 0.58, 0.55]), ..Emit::default()
    });
}

/// A tube, panel or lamp shot out: pale glass and phosphor raining off it, a puff of powder, a fizz off the ballast.
pub fn fixture_shatter_fx(g: &mut crate::game::Game, t: &crate::particle_fx::FixtureShatterFxT, from: V3) {
    let pfx = g.m.particle_fx.pfx;
    let b: Option<BoxId> = t.boxes.first().and_then(|&i| js_ix(g.engine.world.statics.len(), i)).map(|i| g.engine.world.statics[i]);
    let (at, spread): (V3, f64) = match b {
        Some(b) => {
            let (c, h) = (g.tab.boxes[b].c, g.tab.boxes[b].h);
            ([c[0], m::min(c[1] - h[1] - 0.03, 2.9), c[2]], m::min(0.6, m::max(h[0], h[2])))
        }
        None => ([t.pos[0], m::min(t.pos[1], 2.9), t.pos[2]], 0.15),
    };
    let away = v3::normalize([at[0] - from[0], 0.0, at[2] - from[2]]);
    let shards = if t.kind == "emissive" { pfx.tube_shards } else { pfx.bulb_shards };
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, at, [away[0] * 0.25, -1.0, away[2] * 0.25], shards, Emit {
        speed: 1.6, speed_jit: Some(0.8), spread: Some(1.2), life: 25.0, life_jit: Some(0.3), size: 0.006, size_jit: Some(0.7), aspect: Some(1.8), pos_jit: Some(spread), color: Some([0.9, 0.92, 0.88]), ..Emit::default()
    });
    emit_dust(g, v3::add(at, [0.0, -0.05, 0.0]), [0.0, -1.0, 0.0], 6.0, Emit {
        shape: Some(crate::render_particles::EmitShape::Puff), speed: 0.5, speed_jit: Some(0.5), lift: Some(-0.4), life: 2.4, life_jit: Some(0.3), size: 0.16, size_jit: Some(0.35), pos_jit: Some(spread * 0.7), color: Some([0.92, 0.92, 0.9]), ..Emit::default()
    });
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, v3::add(at, [0.0, 0.02, 0.0]), v3::normalize([away[0] * 0.8, -0.55, away[2] * 0.8]), pfx.tube_sparks, Emit {
        speed: 5.5, speed_jit: Some(0.65), spread: Some(1.15), life: 0.4, life_jit: Some(0.5), size: 0.002, ..Emit::default()
    });
}

/// A vehicle's lamp shot out: the lens's own glass, back toward whoever broke it and down, in the lens's colour.
pub fn lens_shatter_fx(g: &mut crate::game::Game, t: crate::particle_fx::LensShatterFxT, from: V3) {
    let out = v3::normalize([from[0] - t.pos[0], 0.0, from[2] - t.pos[2]]);
    let l = t.lens.unwrap_or([0.9, 0.9, 0.9]);
    let n = g.m.particle_fx.pfx.bulb_shards;
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, v3::mad(t.pos, out, 0.05), [out[0] * 0.6, -0.5, out[2] * 0.6], n, Emit {
        speed: 1.4, speed_jit: Some(0.8), spread: Some(0.9), life: 25.0, life_jit: Some(0.3), size: 0.005, size_jit: Some(0.7), aspect: Some(1.8), pos_jit: Some(0.08), color: Some([0.3 + l[0] * 0.65, 0.3 + l[1] * 0.65, 0.3 + l[2] * 0.65]), ..Emit::default()
    });
}

/// The breaker shot: a hard burst of sparks out of the panel, molten copper dripping, a cough of soot.
pub fn breaker_spark_fx(g: &mut crate::game::Game, at: V3, from: V3) {
    let out = v3::normalize([from[0] - at[0], 0.15, from[2] - at[2]]);
    let n = g.m.particle_fx.pfx.breaker_sparks;
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, v3::mad(at, out, 0.06), out, n, Emit {
        speed: 5.5, speed_jit: Some(0.65), spread: Some(0.9), life: 0.7, life_jit: Some(0.5), size: 0.003, ..Emit::default()
    });
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, v3::mad(at, out, 0.06), [out[0], -0.4, out[2]], 25.0, Emit {
        speed: 2.0, speed_jit: Some(0.7), spread: Some(1.3), life: 1.1, life_jit: Some(0.4), size: 0.0034, ..Emit::default()
    });
    emit_dust(g, v3::mad(at, out, 0.12), out, 7.0, Emit {
        shape: Some(crate::render_particles::EmitShape::Puff), speed: 0.9, speed_jit: Some(0.5), lift: Some(0.5), life: 2.2, life_jit: Some(0.3), size: 0.14, size_jit: Some(0.35), pos_jit: Some(0.08), vadd: Some(v3::scale(out, 0.4)), color: Some([0.12, 0.11, 0.1]), ..Emit::default()
    });
}

/// A lock kicked through: long splinters of the jamb and the strike plate's screws, a few back past the boot.
pub fn lock_splinter_fx(g: &mut crate::game::Game, d: DoorId, from: V3) {
    let (hinge, width, cd) = (g.dr[d].def.hinge, g.dr[d].def.width, g.dr[d].def.closed_dir);
    let pfx = g.m.particle_fx.pfx;
    let u: V3 = [m::cos(cd), 0.0, m::sin(cd)];
    let keep: V3 = [hinge[0] + u[0] * (width + 0.02), 1.0, hinge[1] + u[2] * (width + 0.02)];
    let mut mv: V3 = [-u[2], 0.0, u[0]];
    if v3::dot(mv, v3::sub(from, keep)) > 0.0 { mv = v3::scale(mv, -1.0); }
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, v3::mad(keep, mv, 0.05), v3::normalize(v3::add(mv, [0.0, 0.15, 0.0])), pfx.lock_splinters, Emit {
        speed: 4.0, speed_jit: Some(0.6), spread: Some(0.55), life: 30.0, life_jit: Some(0.3), size: 0.009, size_jit: Some(0.6), aspect: Some(3.5), color: Some([0.7, 0.56, 0.36]), ..Emit::default()
    });
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, v3::mad(keep, mv, -0.04), v3::normalize(v3::add(v3::scale(mv, -1.0), [0.0, 0.4, 0.0])), pfx.lock_splinters_back, Emit {
        speed: 1.4, speed_jit: Some(0.7), spread: Some(1.0), life: 30.0, life_jit: Some(0.3), size: 0.007, size_jit: Some(0.5), aspect: Some(2.5), color: Some([0.62, 0.48, 0.3]), ..Emit::default()
    });
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, v3::mad(keep, mv, 0.04), mv, pfx.lock_screws, Emit {
        speed: 2.6, speed_jit: Some(0.5), spread: Some(0.5), life: 40.0, life_jit: Some(0.2), size: 0.005, size_jit: Some(0.3), color: Some([0.5, 0.5, 0.47]), ..Emit::default()
    });
}

/// The ring airfoil meeting a wall or a desk: a few sparks off the clack.
pub fn clack_fx(g: &mut crate::game::Game, at: V3) {
    let pfx = g.m.particle_fx.pfx;
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, v3::add(at, [0.0, 0.02, 0.0]), [0.0, 1.0, 0.0], pfx.clack_sparks, Emit {
        speed: 2.5, speed_jit: Some(0.6), spread: Some(1.3), life: 0.3, life_jit: Some(0.4), size: pfx.spark_size, ..Emit::default()
    });
}

/// The tracer's ember where it stops in geometry: the same few sparks back off the surface.
pub fn ember_fx(g: &mut crate::game::Game, at: V3, dir: V3, o: crate::particle_fx::EmberFxO) {
    let size = g.m.particle_fx.pfx.ember_size;
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, at, dir, o.count, Emit {
        speed: o.speed, speed_jit: Some(0.55), spread: Some(o.cone), life: o.life, life_jit: Some(0.35), size, ..Emit::default()
    });
}

// ---------------------------------------------------------------- spent cases

/// The empty a row's gun throws, or None.
fn casing_of(def: &crate::weapons::WeaponDef) -> Option<crate::particle_fx::CasingSpec> {
    let c = casing_row(def.id)?;
    Some(c)
}

/// CASINGS[id]
fn casing_row(id: crate::weapons::WeaponId) -> Option<crate::particle_fx::CasingSpec> {
    use crate::weapons::WeaponId;
    let t = crate::particle_fx::casings();
    match id {
        WeaponId::Fiveseven => t.fiveseven,
        WeaponId::Guard9mm => t.guard9mm,
        WeaponId::Aks74u => t.aks74u,
        WeaponId::Ak74m => t.ak74m,
        WeaponId::Akms => t.akms,
        WeaponId::Fn2000 => t.fn2000,
        WeaponId::M870 => t.m870,
        WeaponId::Mk12 => t.mk12,
        WeaponId::Mp7 => t.mp7,
        WeaponId::Mk46 => t.mk46,
        WeaponId::M1014 => t.m1014,
        WeaponId::Hk45 => t.hk45,
        WeaponId::G18 => t.g18,
        WeaponId::Pkm => t.pkm,
        WeaponId::Mk18bare => t.mk18bare,
        WeaponId::G19bare => t.g19bare,
    }
}

/// The module's row for this game, made if there is none.
fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::particle_fx::FxState {
    g.m.particle_fx.state.get_or_insert_with(|| crate::particle_fx::FxState { tinks: Vec::new(), n: 0.0 })
}

/// One spent case out of the shooter's gun, and the sound of it landing `tink` seconds later where it will roughly come down.
pub fn casing_fx(g: &mut crate::game::Game, shooter: CharId, def: &crate::weapons::WeaponDef) {
    let Some(c) = casing_of(def) else { return; };
    let (muzzle, gun_dir, vel, id) = (g.ch[shooter].muzzle, g.ch[shooter].gun_dir, g.ch[shooter].vel, g.ch[shooter].id);
    let f = crate::effects::muzzle_frame(muzzle, gun_dir);
    let right = v3::scale(f.x, -1.0);
    let base = if c.from == crate::particle_fx::CasingSpecFrom::Hand { g.ch[shooter].bone(crate::character::Bone::HandR).unwrap_or(muzzle) } else { muzzle };
    let port = v3::mad(v3::mad(v3::mad(base, f.z, c.ahead), f.y, c.up), right, c.right);
    let dir = v3::normalize(v3::add(v3::add(v3::scale(right, c.dir[0]), v3::scale(f.y, c.dir[1])), v3::scale(f.z, c.dir[2])));
    let life = g.m.particle_fx.pfx.casing_life;
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Casing, port, dir, 1.0, Emit {
        speed: c.speed, speed_jit: Some(0.3), spread: Some(0.25), life, life_jit: Some(0.1), size: c.diameter, size_jit: Some(0.0), vadd: Some(vel), owner: Some(id), color: Some(c.color), ..Emit::default()
    });
    let t = g.time + c.tink;
    let land: V3 = [port[0] + dir[0] * c.speed * c.tink, 0.02, port[2] + dir[2] * c.speed * c.tink];
    let st = state_of(g);
    let rate = tink_rate(st.n);
    st.n += 1.0;
    st.tinks.push(crate::particle_fx::FxStateTinksItem { t, pos: land, rate });
}

/// A little spread in pitch, dealt not drawn.
fn tink_rate(n: f64) -> f64 { 0.9 + js::rem(n * 7.0, 11.0) * 0.03 }

/// Once a frame: the cases due to land are heard landing.
pub fn update_particle_fx(g: &mut crate::game::Game) {
    let mut i = match &g.m.particle_fx.state { Some(st) if !st.tinks.is_empty() => st.tinks.len(), _ => return };
    let gain = g.m.particle_fx.pfx.tink_gain;
    let now = g.time;
    while i > 0 {
        i -= 1;
        let st = g.m.particle_fx.state.as_mut().unwrap();
        let k = st.tinks[i];
        if k.t > now { continue; }
        st.tinks.remove(i);
        crate::audio_iface::game_audio_like_play(g, "casingTink", Some(k.pos), Some(gain), Some(&crate::audio_iface::PlayOpts::Rate { rate: k.rate }));
    }
}

/// Encounter restart: nothing in the air, nothing on the floor, nothing waiting to be heard.
pub fn clear_particle_fx(g: &mut crate::game::Game) {
    if let Some(st) = g.m.particle_fx.state.as_mut() { st.tinks.clear(); }
}


/// A site's record: fields not written are None, and each site writes speed, life and size.
type Emit = crate::render_particles::EmitOpts;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_stuff::BoxStuff;

    /// What a round knocks out of a thing is what the thing says it is made of. A failure means sparks off a plaster wall or plaster dust off a steel door.
    #[test]
    fn a_round_knocks_out_what_a_thing_says_it_is_made_of() {
        use crate::scene_materials::Surf;
        for (stuff, surf) in [
            (BoxStuff::Steel, Surf::Metal), (BoxStuff::Carpaint, Surf::Metal), (BoxStuff::Glass, Surf::Glass), (BoxStuff::Veneer, Surf::Wood), (BoxStuff::Fabric, Surf::Fabric),
            (BoxStuff::Cloth, Surf::Carpet), (BoxStuff::Skim, Surf::Drywall), (BoxStuff::MudWall, Surf::Concrete), (BoxStuff::Rubber, Surf::Plastic),
        ] {
            assert!(surf_of_stuff(stuff) == Some(surf), "{}", stuff.as_str());
        }
        for coat in [BoxStuff::Paint, BoxStuff::Enamel, BoxStuff::Plain] {
            assert!(surf_of_stuff(coat).is_none(), "a coat says nothing of what is under it: {}", coat.as_str());
        }
    }

    #[test]
    fn a_prop_is_hit_as_its_kind() {
        use crate::scene_level_data::PropKindName as P;
        use crate::scene_materials::Surf;
        for (k, surf) in [(P::Chair, Surf::Fabric), (P::Cardboard, Surf::Cardboard), (P::Bin, Surf::Metal), (P::Plant, Surf::Foliage)] {
            assert!(surf_of_prop(k) == surf, "{}", k.as_str());
        }
    }

    /// Every surface has what a round knocks out of it, or a shot at it brings the game down.
    #[test]
    fn every_surface_has_an_impact() {
        for s in [crate::scene_materials::Surf::Carpet, crate::scene_materials::Surf::Metal, crate::scene_materials::Surf::Glass, crate::scene_materials::Surf::Wood, crate::scene_materials::Surf::Plastic, crate::scene_materials::Surf::Cardboard, crate::scene_materials::Surf::Drywall, crate::scene_materials::Surf::Concrete, crate::scene_materials::Surf::Fabric, crate::scene_materials::Surf::Foliage] {
            let _ = impact_row(s);
        }
    }

    /// Eleven cases in a row do not ring alike.
    #[test]
    fn a_spent_case_tinks_at_a_dealt_pitch_and_the_deal_goes_round() {
        for n in 0..11 {
            let r = tink_rate(n as f64);
            assert!((0.9..=1.2 + 1e-12).contains(&r), "{n}: {r}");
            assert_eq!(r, tink_rate((n + 11) as f64), "{n}");
            for o in (n + 1)..11 { assert_ne!(r, tink_rate(o as f64), "{n} {o}"); }
        }
    }

    /// A failure means dust where the table says none, or a small puff lost to rounding.
    #[test]
    fn a_dust_table_of_nought_is_no_dust_and_any_other_keeps_at_least_a_puff() {
        assert_eq!(dust_count(1.0, 5.0), 5.0);
        assert_eq!(dust_count(0.5, 5.0), 3.0);
        assert_eq!(dust_count(0.1, 1.0), 1.0);
        assert_eq!(dust_count(0.0, 5.0), 0.0);
        assert_eq!(dust_count(1.0, 0.0), 0.0);
    }

    /// The cases due are heard and gone; the rest wait.
    #[test]
    fn due_tinks_leave_the_list() {
        let mut g = crate::game::Game::blank();
        g.time = 1.0;
        for t in [0.5, 2.0, 1.0] { state_of(&mut g).tinks.push(crate::particle_fx::FxStateTinksItem { t, pos: [0.0; 3], rate: 1.0 }); }
        update_particle_fx(&mut g);
        let left: Vec<f64> = state_of(&mut g).tinks.iter().map(|k| k.t).collect();
        assert_eq!(left, [2.0]);
    }

    /// A round in a leaf with no box looks like wood, in anything unknown like concrete.
    #[test]
    fn a_hit_on_what_has_no_box_looks_like_wood_or_concrete() {
        let mut g = crate::game::Game::blank();
        let at = |i: f64| crate::combat::RoundMet { index: Some(i), ..Default::default() };
        let l = hit_look(&mut g, &at(1e6));
        assert!(l.surf == crate::scene_materials::Surf::Wood && !l.said && l.albedo == [0.42, 0.3, 0.19]);
        let l = hit_look(&mut g, &at(99.0));
        assert!(l.surf == crate::scene_materials::Surf::Concrete && l.albedo == [0.5, 0.5, 0.5]);
        let l = hit_look(&mut g, &crate::combat::RoundMet::default());
        assert!(l.surf == crate::scene_materials::Surf::Concrete);
    }
}
