//! src/game/destruction.ts lines 386 to 764
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ------------------------------------------------------------------ what the skeleton does not reach yet (BODY.md: seams)

/// A box's row, `g.col.boxes[id]` as index/properties.txt has it: skeleton 5 has no table of Box rows (`StaticCollision.boxes` is `Vec<BoxId>`)
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

// ------------------------------------------------------------------ small helpers of this part

/// `a | b` of two box flag words, as the language does it
fn or_bits(a: f64, b: f64) -> f64 { (js::to_i32(a) | js::to_i32(b)) as f64 }

/// `BoxFlag.NoPrimary | BoxFlag.NoShadow`
fn hidden_bits() -> f64 {
    let f = crate::scene_boxes::box_flag();
    or_bits(f.no_primary, f.no_shadow)
}

/// the place `i` is in a list of `len` boxes, or None for `list[i] ?? null` of a number out of range
fn box_at(len: usize, i: f64) -> Option<usize> {
    if i >= 0.0 && (i as usize) < len { Some(i as usize) } else { None }
}

/// a row of LANE_MARK
fn lane(name: &str) -> LaneMark { *lane_mark().get(String::from(name)).unwrap() }

/// `for (const fn of [...this.blastListeners]) fn(ev)`, one of them; what a subscriber adds to `ev.res` stays in the event   (glassbreak.ts:219, winbars.ts:80, snowBlast.ts:119)
fn tell_listener(g: &mut Game, l: &BlastListener, ev: &mut crate::wallbreak::BlastEvent) {
    match l {
        BlastListener::Glass => {
            let n = crate::glassbreak::glass_break_blast(g, ev.at, Some(crate::glassbreak::GlassBlastOpts { radius: Some(ev.radius), power: Some(ev.power), who: Some(ev.who) }));
            ev.res.panes += n;
        }
        BlastListener::Bars => crate::winbars::bars_blast(g, *ev),
        BlastListener::Snow => { crate::snow_blast::snow_blast_at(g, &ev.at, ev.radius); }
    }
}

/// the chunk a knocked thing becomes: binder and clutter are themselves, anything else a housing (destruction.ts:506)
fn chunk_of(kind: DKind) -> crate::debris::ChunkKind {
    match kind {
        DKind::Binder => crate::debris::ChunkKind::Binder,
        DKind::Clutter => crate::debris::ChunkKind::Clutter,
        _ => crate::debris::ChunkKind::Housing,
    }
}

fn item_at(g: &Game, i: f64) -> Option<Destructible> { g.destr.items.get(crate::Num(i)).cloned() }

/// the word pallHash hands to hash01 (destruction.ts:716)
fn pall_hash_word(at: V3, time: f64, k: f64) -> u32 {
    let a = m::imul(js::to_i32(m::round(at[0] * 64.0)), 73856093);
    let b = m::imul(js::to_i32(m::round(at[2] * 64.0)), 19349663);
    let c = m::imul(js::to_i32(m::round(time * 60.0)), 83492791);
    let d = m::imul(js::to_i32(k + 1.0), js::to_i32(2654435761.0));
    (a ^ b ^ c ^ d) as u32
}

/// the directions of PALL_DIRS in which something solid stands within `r` of (px, Y, pz): pallEmit's `near`
fn pall_near(g: &mut Game, px: f64, y: f64, pz: f64, r: f64) -> Vec<[f64; 2]> {
    let mut out = Vec::new();
    for w in pall_dirs() {
        if crate::scene_collision::solid_blocked(g, [px, y, pz], [px + w[0] * r, y, pz + w[1] * r], None) { out.push(w); }
    }
    out
}

// ------------------------------------------------------------------ the translation

/// A round stopped at `hit_p`: decals, a screen killed, paper, bursts and dust, knock-offs, tiles, what lies about, and the wall's share.   (destruction.ts round)
pub fn destruction_round(g: &mut crate::game::Game, shooter: CharId, hit_p: V3, dir: V3, hit: &crate::destruction::DestructionRoundHit, def: &crate::weapons::WeaponDef) {
    let chaos = chaos_of(g);
    if chaos <= 0.0 { return; }
    if hit.index.is_none() && hit.prop.is_none() { return; }
    g.stats.destruction.destr_rounds += 1;
    let amount = if loud_now(g) { g.k.chaos.loud.heat_k } else { 1.0 };
    let e = destruction_stoke(g, hit_p, amount, chaos);
    g.destr.any_heat = true;
    let rifle = def.cls != crate::weapons::WeaponClass::Pistol;
    let k = if rifle { 1.4 } else { 1.0 };
    let n = v3::normalize(hit.n);
    // what was hit
    let mut surf = crate::scene_materials::Surf::Concrete;
    let mut albedo: V3 = [0.5, 0.5, 0.5];
    let mut box_: Option<BoxId> = None;
    let mut bi = -1.0;
    if let Some(prop) = hit.prop {
        surf = match g.tab.props_prop[prop].def.kind {
            crate::scene_level_data::PropKindName::Chair => crate::scene_materials::Surf::Fabric,
            crate::scene_level_data::PropKindName::Cardboard => crate::scene_materials::Surf::Cardboard,
            crate::scene_level_data::PropKindName::Bin => crate::scene_materials::Surf::Metal,
            _ => crate::scene_materials::Surf::Foliage,
        };
        box_ = hit.box_.or_else(|| g.tab.props_prop[prop].boxes.first().copied());
        if let Some(b) = box_ { albedo = seam_box(g, b).albedo; }
    } else if let Some(index) = hit.index {
        if index >= 1e6 {
            surf = crate::scene_materials::Surf::Wood;
            box_ = box_at(g.col.dynamic_boxes.len(), index - 1e6).map(|k| g.col.dynamic_boxes[k]);
            if let Some(b) = box_ { albedo = seam_box(g, b).albedo; }
        } else {
            bi = index;
            box_ = box_at(g.col.boxes.len(), bi).map(|k| g.col.boxes[k]);
            if let Some(b) = box_ {
                albedo = seam_box(g, b).albedo;
                let word = seam_box(g, b).does;
                surf = if crate::scene_materials::does_of(g, word).wall == Some(crate::scene_materials::DoesWall::Screen) { crate::scene_materials::Surf::Fabric } else { crate::particle_fx::surf_at(g, b) };
            }
        }
    }
    let item = if bi >= 0.0 { item_at(g, bi) } else { None };
    // (1) damage decals by material
    if e >= g.k.chaos.t_decal {
        if let Some(b) = box_ {
            let dynamic = hit.prop.is_some() || matches!(hit.index, Some(i) if i >= 1e6);
            destruction_damage_decal(g, hit_p, n, surf, b, dynamic, rifle, item.as_ref());
        }
    }
    // (2) a monitor / TV that took the round
    if let Some(it) = &item {
        if matches!(it.kind, DKind::Screen | DKind::Housing | DKind::Tv) {
            let id = g.ch[shooter].id;
            let by_player = crate::seats::is_man_id(g, id);
            destruction_kill_screen(g, it, hit_p, n, by_player);
        }
    }
    // (3) paper near the hit, or whose air the round crossed
    if e >= g.k.chaos.t_paper {
        let seg = v3::dist(g.ch[shooter].muzzle, hit_p);
        destruction_papers_near(g, hit_p, dir, e, chaos, k, seg);
    }
    // (4) bigger material bursts + (5) dust into the smoke solver
    if e >= g.k.chaos.t_burst {
        let owner = g.ch[shooter].id;
        destruction_burst(g, hit_p, dir, n, surf, albedo, e, k, owner, Some(DestructionBurstHit { index: hit.index, prop: hit.prop, box_: hit.box_ }));
    }
    if e >= g.k.chaos.t_puff && matches!(surf, crate::scene_materials::Surf::Drywall | crate::scene_materials::Surf::Concrete | crate::scene_materials::Surf::Wood | crate::scene_materials::Surf::Cardboard | crate::scene_materials::Surf::Fabric) {
        destruction_dust_puff(g, v3::mad(hit_p, n, 0.05), n, m::min(2.0, 0.7 + 0.2 * e), surf);
    }
    // (6) knock-offs: the housing on a direct hit, clutter near the hit; a chair a round finds is spun
    let who = g.ch[shooter].id;
    if e >= g.k.chaos.t_knock {
        if let Some(it) = &item {
            if it.kind == DKind::Housing {
                destruction_knock_off(g, it, dir, chaos, crate::debris::ChunkKind::Housing, Some(who));
            } else if it.kind == DKind::Screen {
                if let Some(mate) = it.mate {
                    if let Some(hd) = item_at(g, mate) {
                        destruction_knock_off(g, &hd, dir, chaos, crate::debris::ChunkKind::Housing, Some(who));
                    }
                }
            }
            if it.kind == DKind::Clutter || it.kind == DKind::Binder {
                destruction_knock_off(g, it, dir, chaos, chunk_of(it.kind), Some(who));
            }
        }
        destruction_clutter_near(g, hit_p, dir, chaos, who);
        if let Some(prop) = hit.prop {
            if g.tab.props_prop[prop].def.kind == crate::scene_level_data::PropKindName::Chair {
                let w = crate::props::prop_w(g, prop);
                let sign = if destruction_hash(g) < 0.5 { -1.0 } else { 1.0 };
                crate::props::prop_set_w(g, prop, w + sign * (4.0 + 5.0 * chaos));
                crate::props::prop_wake(g, prop);
                g.stats.destruction.destr_chair_spins += 1;
            }
        }
    }
    // (7) the ceiling over a beaten zone
    if e >= g.k.chaos.t_tile { destruction_tile_drop(g, hit_p, chaos, Some(false), Some(who), None); }
    // (8) what already lies about the hit is scooted along
    destruction_impact_impulse(g, hit_p, dir, n, chaos, who);
    // (9) wall breaking: the round's work on what it stopped in
    if let Some(index) = hit.index {
        crate::wallbreak::wall_break_round(g, who, hit_p, dir, index, def, chaos);
    }
}

/// THE BLAST ENTRY POINT: what any charge or grenade calls to break the level where it went off.   (destruction.ts blast)
pub fn destruction_blast(g: &mut crate::game::Game, at: V3, opts: Option<crate::wallbreak::BlastOpts>) -> crate::wallbreak::BlastResult {
    let opts = opts.unwrap_or_default();
    let who = opts.who;
    crate::scene_collision::window(g, &mut |g: &mut Game| crate::fogofwar::fog_of_war_doing(g, who, &mut |g: &mut Game| destruction_blast_raw(g, at, opts)))
}

/// The call list, the doors, the walls and cover, the furniture, then the lane's bang and pall.   (destruction.ts blastRaw)
fn destruction_blast_raw(g: &mut crate::game::Game, at: V3, opts: crate::wallbreak::BlastOpts) -> crate::wallbreak::BlastResult {
    let chaos = chaos_of(g);
    let mut res = crate::wallbreak::BlastResult { walls: 0.0, cover: 0.0, doors: 0.0, panes: 0.0, opened: 0.0, serial: g.destr.walls.serial };
    if chaos <= 0.0 {
        if opts.chaos != Some(false) { destruction_bang(g, at, opts.who, None, None); }
        return res;
    }
    let power = opts.power.unwrap_or(1.0);
    let radius = opts.radius.unwrap_or(g.k.walls.blast.radius) * power;
    let who = js::or(opts.who.unwrap_or(0.0), crate::consts::NOBODY_ID);
    let mut ev = crate::wallbreak::BlastEvent { at, radius, power, who, res };
    for l in g.destr.blast_listeners.clone() { tell_listener(g, &l, &mut ev); }   // (a copy: as `[...this.blastListeners]`)
    crate::snow_blast::snow_blast_at(g, &at, radius);
    res = ev.res;   // (the event's `res` IS `res` in the TypeScript)
    let tear = radius * g.k.walls.blast.door_k;
    res.doors += crate::doors::doors_blast(g, at, tear, who).len() as f64;
    if crate::wallbreak::wall_break_on(g) {
        let w = crate::wallbreak::wall_break_blast(g, at, Some(opts));
        res.walls = w.walls;
        res.cover = w.cover;
        res.opened = w.opened;
    }
    let src = if opts.chaos == Some(false) { crate::furnblast::charge_source(radius) } else { crate::furnblast::BlastSource::Frag };
    crate::furnblast::furniture_blast(g, at, src, who, chaos);
    if opts.chaos != Some(false) {
        destruction_bang(g, at, Some(who), Some(2.0 * power), None);
        destruction_pall_burst(g, at, Some(radius / 2.5));
        crate::floormess::mess_burst(g, at, Some(radius / 2.5));
    }
    res.serial = g.destr.walls.serial;
    res
}

/// The door leaves a blast here would take off their hinges; nothing is touched.   (destruction.ts doorsFor)
pub fn destruction_doors_for(g: &mut crate::game::Game, at: V3, opts: Option<crate::destruction::DestructionDoorsForOpts>) -> Vec<DoorId> {
    let opts = opts.unwrap_or_default();
    if chaos_of(g) <= 0.0 { return Vec::new(); }
    let tear = opts.radius.unwrap_or(g.k.walls.blast.radius) * opts.power.unwrap_or(1.0) * g.k.walls.blast.door_k;
    crate::doorblast::door_blast_reach(g, at, tear).into_iter().map(|x| x.d).collect()
}

/// The walls and cover a blast here would open; nothing is touched.   (destruction.ts wallsFor)
pub fn destruction_walls_for(g: &mut crate::game::Game, at: V3, opts: Option<crate::destruction::DestructionWallsForOpts>) -> Vec<crate::wallbreak::WallOpening> {
    let opts = opts.unwrap_or_default();
    if chaos_of(g) <= 0.0 || !crate::wallbreak::wall_break_on(g) { return Vec::new(); }
    let ask = crate::wallbreak::BlastOpts { radius: opts.radius, power: opts.power, ..Default::default() };
    crate::wallbreak::wall_break_would_open(g, at, Some(ask))
}

/// Furniture under a blast, for a charge that does not come through blast().   (destruction.ts furnBlast)
pub fn destruction_furn_blast(g: &mut crate::game::Game, at: V3, src: crate::furnblast::BlastSource, who: Option<f64>) {
    let who = who.unwrap_or(crate::consts::NOBODY_ID);
    let chaos = chaos_of(g);
    if chaos > 0.0 { crate::furnblast::furniture_blast(g, at, src, who, chaos); }
}

/// The flags static `i` was authored with if this lane has hidden it, else None.   (destruction.ts flags0)
pub(in crate::breakage) fn destruction_flags0(g: &mut crate::game::Game, i: f64) -> Option<f64> {
    if let Some(&f) = g.destr.hidden.get(crate::Num(i)) { return Some(f); }
    if let Some(f) = crate::wallbreak::wall_break_flags0(g, i) { return Some(f); }
    crate::winbars::bar_flags0(g, i)
}

/// Be told of every blast. (The TypeScript's unsubscribe has no caller and no skeleton.)   (destruction.ts onBlast)
pub(in crate::breakage) fn destruction_on_blast(g: &mut crate::game::Game, fn_: crate::destruction::BlastListener) {
    g.destr.blast_listeners.push(fn_);
}

/// Static `i` has lost what it stood on / hung from: a stack scatters, a screen dies, clutter drops. False = not one of ours.   (destruction.ts dropItem)
pub(in crate::breakage) fn destruction_drop_item(g: &mut crate::game::Game, i: f64, dirn: V3, who: Option<f64>, quiet: Option<bool>) -> bool {
    let who = who.unwrap_or(crate::consts::NOBODY_ID);
    let quiet = quiet.unwrap_or(false);
    let Some(d) = item_at(g, i) else { return false; };
    if g.destr.hidden.has(crate::Num(i)) { return true; }
    if quiet {
        if let Some(t) = d.target { destruction_break_target(g, t); }
        destruction_hide(g, &d);
        return true;
    }
    let chaos = m::max(chaos_of(g), 0.25);
    if d.kind == DKind::Paper {
        if !destruction_convert_paper(g, &d, dirn, 1.5, chaos) { destruction_hide(g, &d); }
        return true;
    }
    g.destr.free_knocks += 1.0;   // (a fall is not a knock off the per-second budget)
    if let Some(t) = d.target { destruction_break_target(g, t); }
    if !destruction_knock_off(g, &d, dirn, chaos, chunk_of(d.kind), Some(who)) && !g.destr.hidden.has(crate::Num(i)) {
        if let Some(t) = d.target { destruction_break_target(g, t); }
        destruction_hide(g, &d);
    }
    true
}

/// A light fixture's box that fell with its wall or desk: its light goes with it.   (destruction.ts breakFixtureBox)
pub(in crate::breakage) fn destruction_break_fixture_box(g: &mut crate::game::Game, i: f64) {
    let mut at = 0;
    while at < g.targets.len() {
        let t = g.targets[at];
        at += 1;
        if g.tab.sim_lightshome_fixture[t].boxes.iter().any(|&b| b == i) && !crate::sim_lightshome::fixture_broken(g, t) {
            if destruction_break_target(g, t) {
                let pos = g.tab.sim_lightshome_fixture[t].pos;
                crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(0.6), None);
            }
        }
    }
}

/// A charge went off at `at`: heat, the desk cleared, papers up, the tile down, dust off the ceiling, what lies about thrown again.   (destruction.ts bang)
pub fn destruction_bang(g: &mut crate::game::Game, at: V3, who: Option<f64>, power: Option<f64>, stun: Option<bool>) {
    let mut who = who.unwrap_or(crate::consts::NOBODY_ID);
    let power = power.unwrap_or(1.0);
    let stun = stun.unwrap_or(false);
    let chaos = chaos_of(g);
    if chaos <= 0.0 { return; }
    if !js::truthy(who) { who = crate::consts::NOBODY_ID; }
    g.stats.destruction.destr_bangs += 1;
    let loud = loud_now(g);
    let pw = if loud { m::max(0.25, m::min(4.0, power)) } else { 1.0 };
    let reach = m::sqrt(pw);
    let e = destruction_stoke(g, at, g.k.chaos.bang_heat * pw, chaos);
    g.destr.any_heat = true;
    // papers within a couple of metres, thrown out and up from the blast
    let mut list: Vec<Destructible> = Vec::new();
    let mut wide: Vec<Destructible> = Vec::new();
    let span = if reach > 1.0 { 2.0 * m::ceil((2.6 * reach - 2.0) / 2.0) } else { 2.0 };
    let mut dz = -span;
    while dz <= span {
        let mut dx = -span;
        while dx <= span {
            for d in destruction_near(g, [at[0] + dx, at[1], at[2] + dz], &mut list) {
                if !wide.iter().any(|w| w.i == d.i) { wide.push(d); }   // (items has one row a static: `i` is its identity)
            }
            dx += 2.0;
        }
        dz += 2.0;
    }
    let eye: V3 = [at[0], m::max(at[1] + 0.3, 0.95), at[2]];
    for d in &wide {
        let c = seam_box(g, d.box_).c;
        let dd = v3::dist(c, at);
        let over = top_of(g, d.box_);
        if d.kind == DKind::Paper && dd < 2.6 * reach && !crate::scene_collision::solid_blocked(g, eye, over, None) {
            destruction_convert_paper(g, d, v3::normalize(v3::add(v3::sub(c, at), [0.0, 0.8, 0.0])), 2.5 + 2.0 * chaos, chaos);
        } else if matches!(d.kind, DKind::Clutter | DKind::Binder | DKind::Housing) && dd < 1.6 * reach && e >= g.k.chaos.t_knock && !crate::scene_collision::solid_blocked(g, eye, over, None) {
            destruction_knock_off(g, d, v3::normalize(v3::sub(c, at)), chaos, chunk_of(d.kind), Some(who));
        }
    }
    let panels = !stun || !loud;
    if e >= g.k.chaos.t_tile {
        let mut ok = destruction_tile_drop(g, at, chaos, Some(true), Some(who), Some(panels));
        if loud {
            let r0 = g.stats.destruction.destr_tiles_refused;
            let mut k = 0;
            while k < 3 && !ok && g.destr.tile_count < g.k.chaos.tiles_per_encounter {
                ok = destruction_tile_drop(g, at, chaos, Some(true), Some(who), Some(panels));
                k += 1;
            }
            g.stats.destruction.destr_tiles_refused = r0;
        }
    }
    if let Some(roof) = destruction_roof_y(g, at[0], at[2]) {
        let cp: V3 = [at[0], roof - 0.15, at[2]];
        if loud { destruction_cell_dust(g, cp, [0.0, -1.0, 0.0], 1.6, crate::scene_materials::Surf::Drywall); } else { destruction_dust_puff(g, cp, [0.0, -1.0, 0.0], 1.6, crate::scene_materials::Surf::Drywall); }
    }
    if loud {
        let l = g.k.chaos.loud;
        destruction_dust_burst(g, [at[0], m::max(0.06, at[1]), at[2]], [0.0, 1.0, 0.0], m::round(l.blast_dust_puffs * pw), l.blast_dust_floor * m::min(1.5, reach), crate::scene_materials::Surf::Concrete, Some(0.5 * reach));
        let r = (if stun { l.stun_fixture_m } else { l.fixture_blast_m }) * reach;
        let mut at_t = 0;
        while at_t < g.targets.len() {
            let t = g.targets[at_t];
            at_t += 1;
            let fire = crate::fire::fire_holds(g, t);
            let pos = g.tab.sim_lightshome_fixture[t].pos;
            if !(fire || destruction_fixture_live(g, t)) || v3::dist(pos, at) > r { continue; }
            let to = v3::sub(pos, eye);
            let len = v3::len(to);
            if len > 0.3 {
                if let Some(hit) = crate::scene_collision::raycast(g, eye, v3::scale(to, 1.0 / len), len) {
                    // (the first solid thing on the line is the fixture itself, or nothing)
                    if hit.t < len - 0.06 && !g.tab.sim_lightshome_fixture[t].boxes.iter().any(|&b| b == hit.index) { continue; }
                }
            }
            if fire { crate::fire::fire_stir(g, t); } else { destruction_kill_fixture(g, t, at, "blast", Some(true)); }
        }
    }
    destruction_bang_impulse(g, at, eye, chaos, who);
}

/// The blast's push on what already lies about.   (destruction.ts bangImpulse)
fn destruction_bang_impulse(g: &mut crate::game::Game, at: V3, eye: V3, chaos: f64, who: f64) {
    if !impulses_on(g) { return; }
    let b = g.k.chaos.imp.bang;
    let strength = b.strength * imp_k(g, chaos);
    g.stats.destruction.destr_imp_bang += 1;
    crate::render_particles::particles_impulse(g, at, b.radius, strength, Some(&crate::render_particles::ImpulseOpts { up: Some(b.up), occlude: Some(true), eye: Some(eye), ..Default::default() }));
    let imp = crate::sheet_model::ImpulseLike { pos: at, radius: b.radius, strength, up: b.up, bias: [0.0, 0.0, 0.0] };
    let mut blocked = |g: &mut Game, p: V3| crate::scene_collision::solid_blocked(g, eye, [p[0], p[1] + 0.05, p[2]], None);
    let kicked = crate::debris::debris_impulse(g, imp, Some(&mut blocked), Some(who));
    g.stats.destruction.destr_debris_kicked += kicked as i32;
}

/// A round stopped in a face of normal `n`: what lies beside the hole jumps and scoots along the round's travel.   (destruction.ts impactImpulse)
fn destruction_impact_impulse(g: &mut crate::game::Game, hit_p: V3, dir: V3, n: V3, chaos: f64, who: f64) {
    if !impulses_on(g) { return; }
    let i = g.k.chaos.imp.impact;
    let strength = i.strength * imp_k(g, chaos);
    let dn = v3::dot(dir, n);
    let bias: V3 = [(dir[0] - n[0] * dn) * i.bias_k, (dir[1] - n[1] * dn) * i.bias_k, (dir[2] - n[2] * dn) * i.bias_k];
    let at = v3::mad(hit_p, n, 0.03);
    g.stats.destruction.destr_imp_impact += 1;
    crate::render_particles::particles_impulse(g, at, i.radius, strength, Some(&crate::render_particles::ImpulseOpts { up: Some(i.up), bias: Some(bias), ..Default::default() }));
    if !g.destr.debris.list.is_empty() {
        let imp = crate::sheet_model::ImpulseLike { pos: at, radius: i.debris_m, strength, up: i.up, bias };
        let kicked = crate::debris::debris_impulse(g, imp, None, Some(who));
        g.stats.destruction.destr_debris_kicked += kicked as i32;
    }
}

/// A thump on the floor at `pos`: the litter within a stride twitches.   (destruction.ts thud)
pub fn destruction_thud(g: &mut crate::game::Game, pos: V3, scale: Option<f64>, who: Option<f64>) {
    let scale = scale.unwrap_or(1.0);
    if !impulses_on(g) { return; }
    let t = g.k.chaos.imp.thud;
    if g.destr.thuds_this_frame >= t.per_frame {
        g.stats.destruction.destr_imp_thud_dropped += 1;
        return;
    }
    g.destr.thuds_this_frame += 1.0;
    g.stats.destruction.destr_imp_thud += 1;
    let chaos = chaos_of(g);
    let strength = t.strength * m::max(0.5, m::min(1.5, scale)) * imp_k(g, chaos);
    let at: V3 = [pos[0], m::min(pos[1], 0.3) + 0.03, pos[2]];
    crate::render_particles::particles_impulse(g, at, t.radius, strength, Some(&crate::render_particles::ImpulseOpts { up: Some(t.up), ..Default::default() }));
    if !g.destr.debris.list.is_empty() {
        // (a thump rocks a chunk, it does not re-blame it: `who` is not handed on, as in the TypeScript)
        let imp = crate::sheet_model::ImpulseLike { pos: at, radius: t.radius * 0.5, strength, up: t.up, bias: [0.0, 0.0, 0.0] };
        let kicked = crate::debris::debris_impulse(g, imp, None, None);
        g.stats.destruction.destr_debris_kicked += kicked as i32;
    }
}

/// The mark a round leaves by material; the dice and the bucket move whether decals are drawn or not.   (destruction.ts damageDecal)
fn destruction_damage_decal(g: &mut crate::game::Game, p: V3, n: V3, surf: crate::scene_materials::Surf, box_: BoxId, dynamic: bool, rifle: bool, item: Option<&crate::destruction::Destructible>) {
    let is_crack = matches!(item, Some(it) if it.kind == DKind::Screen || it.kind == DKind::Tv);   // (`m === LANE_MARK.crack`)
    let mut mark = if is_crack { lane("crack") } else if surf == crate::scene_materials::Surf::Metal { lane("dent") } else if surf == crate::scene_materials::Surf::Fabric { lane("shred") } else { return; };
    if js::to_i32(seam_box(g, box_).flags) & js::to_i32(hidden_bits()) != 0 && !is_crack { return; }
    let rj = destruction_hash(g);
    if !destruction_take(g, DestructionTakeK::Decal) { return; }
    g.stats.destruction.destr_decals += 1;
    if !g.decals.enabled { return; }
    let attach = if dynamic { Some(box_) } else { None };
    let said = if is_crack { None } else { crate::particle_fx::said_surf(g, Some(box_)) };
    match said {
        Some(crate::scene_materials::Surf::Metal) => mark = lane("dent"),
        Some(crate::scene_materials::Surf::Fabric) | Some(crate::scene_materials::Surf::Carpet) => mark = lane("shred"),
        Some(crate::scene_materials::Surf::Glass) => {
            let c = crate::particle_fx::solid_glass_crack();
            mark = LaneMark { look: c.look, r: [c.pistol, c.rifle], s: [c.strength, c.strength] };
        }
        Some(_) => {
            crate::render_decals::decals_add(g, crate::render_decals::DecalKind::Chip, p, n, Some(0.013), Some(crate::render_decals::DecalOpts { clamp_to: Some(box_), attach, ..Default::default() }));
            return;
        }
        None => {}
    }
    let i = if rifle { 1 } else { 0 };
    crate::render_decals::decals_add(g, crate::render_decals::DecalKind::Damage, p, n, Some(mark.r[i] * (0.85 + 0.3 * rj)), Some(crate::render_decals::DecalOpts { param: Some(mark.look + m::min(0.999, mark.s[i])), clamp_to: Some(box_), attach, ..Default::default() }));
}

/// A lit screen's light target out, remembered for reset(). True = a light went out.   (destruction.ts breakTarget)
pub(super) fn destruction_break_target(g: &mut crate::game::Game, t: FixtureId) -> bool {
    if crate::sim_lightshome::fixture_broken(g, t) { return false; }
    let was_lit = g.tab.sim_lightshome_fixture[t].interactive && (crate::sim_lightshome::fixture_on(g, t) || crate::sim_lightshome::fixture_factor(g, t) > 0.0);
    crate::sim_lightshome::light_home_in(g, crate::sim_lightshome::light_in().break_, Some(t), None, None, None, None);
    g.destr.broke.add(t);
    if was_lit { crate::game::game_light_event(g, t, None, None); }
    was_lit
}

/// A monitor or TV that took a round: dead, a fizz of sparks.   (destruction.ts killScreen)
fn destruction_kill_screen(g: &mut crate::game::Game, item: &crate::destruction::Destructible, hit_p: V3, n: V3, by_player: bool) {
    if let Some(t) = item.target {
        if !crate::sim_lightshome::fixture_broken(g, t) {
            let lit = destruction_break_target(g, t);
            let pos = g.tab.sim_lightshome_fixture[t].pos;
            crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(if lit { 0.8 } else { 0.4 }), None);
            if lit {
                crate::render_lights::light_set_flash(g, pos, [0.6, 0.75, 1.0], 4.0, 3.0, 0.1, 0.0);
                let name = g.tab.sim_lightshome_fixture[t].name.clone();
                dlog(g, &alloc::format!("screen {} shot dead", name));
                if by_player {
                    crate::game::game_msg(g, &alloc::format!("shot out {}", name));
                    crate::chronicle::chron_light_shot(g, t);
                }
            }
        }
    }
    g.stats.destruction.destr_screens += 1;
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Spark, v3::mad(hit_p, n, 0.02), v3::normalize(v3::add(n, [0.0, 0.3, 0.0])), if item.target.is_some() { 22.0 } else { 8.0 }, crate::render_particles::EmitOpts { speed: 3.5, speed_jit: Some(0.7), spread: Some(1.2), life: 0.45, life_jit: Some(0.5), size: 0.0028, ..Default::default() });
}

/// Paper stacks within reach of the hit, or whose column of air the round passed through, become sheets.   (destruction.ts papersNear)
fn destruction_papers_near(g: &mut crate::game::Game, hit_p: V3, dir: V3, e: f64, chaos: f64, k: f64, seg_len: f64) {
    let mut list: Vec<Destructible> = Vec::new();
    for d in destruction_near(g, hit_p, &mut list) {
        if d.kind != DKind::Paper { continue; }
        let c = seam_box(g, d.box_).c;
        let h = seam_box(g, d.box_).h;
        let dd = m::hyp3(c[0] - hit_p[0], (c[1] - hit_p[1]) * 0.5, c[2] - hit_p[2]);
        let mut hit_it = dd < g.k.chaos.paper_reach_m * (0.6 + 0.4 * m::min(1.0, e / 3.0)) && !crate::scene_collision::solid_blocked(g, v3::mad(hit_p, dir, -0.05), [c[0], c[1] + h[1] + 0.05, c[2]], None);
        if !hit_it {
            // the segment through the air just over the stack: closest approach of the ray to the stack's centre
            let to = v3::sub(c, hit_p);
            let along = -v3::dot(to, dir);
            if along > 0.0 && along < seg_len {
                let q = v3::mad(hit_p, dir, -along);
                if m::hyp2(q[0] - c[0], q[2] - c[2]) < m::max(h[0], h[2]) + 0.06 && q[1] > c[1] - 0.05 && q[1] < c[1] + 0.35 { hit_it = true; }
            }
        }
        if !hit_it { continue; }
        let away = v3::normalize([c[0] - hit_p[0] + dir[0] * 0.6, 0.9, c[2] - hit_p[2] + dir[2] * 0.6]);
        destruction_convert_paper(g, &d, away, (1.6 + 0.5 * m::min(3.0, e)) * (0.8 + 0.3 * k), chaos);
    }
}

/// Hide the stack's box and throw its sheets.   (destruction.ts convertPaper)
pub fn destruction_convert_paper(g: &mut crate::game::Game, d: &crate::destruction::Destructible, dirn: V3, speed: f64, chaos: f64) -> bool {
    if g.destr.hidden.has(crate::Num(d.i)) || !destruction_take(g, DestructionTakeK::Paper) { return false; }
    destruction_hide(g, d);
    g.stats.destruction.destr_papers += 1;
    dlog(g, &alloc::format!("paper {} scattered", d.i));
    let (c, h, albedo) = { let b = seam_box(g, d.box_); (b.c, b.h, b.albedo) };
    let count = m::round(g.k.chaos.sheets_per_stack * m::min(1.6, m::max(0.5, (h[0] * h[2]) / (0.11 * 0.15))) * (0.7 + 0.6 * chaos));
    g.stats.destruction.destr_sheets += count as i32;
    let w = m::min(0.22, m::max(0.12, 2.0 * m::min(h[0], h[2]) * 0.95));
    let asp = m::max(1.0, m::max(h[0], h[2]) / m::max(0.04, m::min(h[0], h[2])));
    let pj = m::max(h[0], h[2]) * 0.8;
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Sheet, [c[0], c[1] + h[1] + pj + 0.02, c[2]], dirn, count, crate::render_particles::EmitOpts { shape: Some(crate::render_particles::EmitShape::Cone), spread: Some(0.9), speed, speed_jit: Some(0.75), life: 60.0, life_jit: Some(0.25), size: w, size_jit: Some(0.06), aspect: Some(m::min(1.5, asp)), tumble: Some(5.0 + 4.0 * chaos), nrm_jit: Some(0.55), pos_jit: Some(pj), color: Some(v3::copy(albedo)), ..Default::default() });
    // a little puff of the dust that lay on it
    crate::particle_fx::emit_dust(g, [c[0], c[1] + 0.08, c[2]], [0.0, 1.0, 0.0], 3.0, crate::render_particles::EmitOpts { shape: Some(crate::render_particles::EmitShape::Puff), speed: 0.6, speed_jit: Some(0.5), lift: Some(0.2), life: 1.2, life_jit: Some(0.3), size: 0.12, size_jit: Some(0.3), pos_jit: Some(0.1), color: Some([0.7, 0.68, 0.64]), ..Default::default() });
    true
}

/// Take the thing out of the picture and the shadows, and tell the smoke its obstacle field is stale there.   (destruction.ts hide)
pub(super) fn destruction_hide(g: &mut crate::game::Game, d: &crate::destruction::Destructible) {
    let bits = hidden_bits();
    let b = g.col.boxes[d.i as usize];
    let f = seam_box(g, b).flags;
    g.destr.hidden.set(crate::Num(d.i), f);
    seam_box(g, b).flags = or_bits(f, bits);
    if let (Some(mate), DKind::Housing) = (d.mate, d.kind) {
        let s = g.col.boxes[mate as usize];
        if !g.destr.hidden.has(crate::Num(mate)) {
            let sf = seam_box(g, s).flags;
            g.destr.hidden.set(crate::Num(mate), sf);
            seam_box(g, s).flags = or_bits(sf, bits);
        }
    }
    crate::scene_collision::wrote(g);
}

/// The escalated material burst on top of impactFx's, scaled by E and the rifle factor: what flies is the picture's, by what the box says it is made of.   (destruction.ts burst)
fn destruction_burst(g: &mut crate::game::Game, hit_p: V3, dir: V3, n: V3, surf: crate::scene_materials::Surf, albedo: V3, e: f64, k: f64, owner: f64, hit: Option<crate::destruction::DestructionBurstHit>) {
    g.stats.destruction.destr_bursts += 1;
    let (mut surf, mut albedo) = (surf, albedo);
    if let Some(h) = hit {
        let see = crate::particle_fx::hit_look(g, crate::particle_fx::HitLookHit { index: h.index, prop: h.prop, box_: h.box_ });
        if see.solid { return; }
        if see.said {
            surf = if see.surf == crate::scene_materials::Surf::Carpet { crate::scene_materials::Surf::Fabric } else { see.surf };
            albedo = see.albedo;
        }
    }
    let s = m::min(3.0, 0.5 + 0.5 * (e - g.k.chaos.t_burst)) * k;
    let at = v3::mad(hit_p, n, 0.015);
    let dn = v3::dot(dir, n);
    let refl = v3::normalize(v3::sub(dir, v3::scale(n, 2.0 * dn)));
    let out = v3::normalize(v3::add(n, v3::scale(refl, 0.35)));
    let emit = crate::render_particles::particles_emit;
    let dust = crate::particle_fx::emit_dust;
    if surf == crate::scene_materials::Surf::Wood || surf == crate::scene_materials::Surf::Cardboard {
        emit(g, crate::render_particles::PKind::Debris, at, out, m::round(7.0 * s), crate::render_particles::EmitOpts { speed: 2.4, speed_jit: Some(0.7), spread: Some(0.9), life: 20.0, life_jit: Some(0.3), size: 0.0055, size_jit: Some(0.4), aspect: Some(3.2), color: Some(if surf == crate::scene_materials::Surf::Wood { [0.66, 0.52, 0.34] } else { v3::copy(albedo) }), owner: Some(owner), ..Default::default() });
        dust(g, v3::mad(hit_p, n, 0.07), n, m::round(1.0 + s), crate::render_particles::EmitOpts { shape: Some(crate::render_particles::EmitShape::Puff), speed: 0.3, speed_jit: Some(0.5), lift: Some(0.25), life: 1.0, life_jit: Some(0.3), size: 0.11, size_jit: Some(0.35), pos_jit: Some(0.06), vadd: Some(v3::scale(n, 0.2)), color: Some([0.62, 0.54, 0.42]), ..Default::default() });
    } else if surf == crate::scene_materials::Surf::Drywall {
        emit(g, crate::render_particles::PKind::Debris, at, out, m::round(8.0 * s), crate::render_particles::EmitOpts { speed: 2.0, speed_jit: Some(0.7), spread: Some(1.1), life: 18.0, life_jit: Some(0.3), size: 0.008, size_jit: Some(0.6), color: Some(v3::lerp3(albedo, [0.86, 0.85, 0.82], 0.55)), owner: Some(owner), ..Default::default() });
        dust(g, v3::mad(hit_p, n, 0.08), n, m::round(2.0 + s), crate::render_particles::EmitOpts { shape: Some(crate::render_particles::EmitShape::Puff), speed: 0.35, speed_jit: Some(0.5), lift: Some(0.25), life: 1.2, life_jit: Some(0.3), size: 0.13, size_jit: Some(0.35), pos_jit: Some(0.08), vadd: Some(v3::scale(n, 0.25)), color: Some([0.82, 0.8, 0.76]), ..Default::default() });
    } else if surf == crate::scene_materials::Surf::Fabric {
        dust(g, v3::mad(hit_p, n, 0.05), out, m::round(9.0 * s), crate::render_particles::EmitOpts { shape: Some(crate::render_particles::EmitShape::Cone), spread: Some(1.0), speed: 1.3, speed_jit: Some(0.7), life: 2.6, life_jit: Some(0.4), size: 0.022, size_jit: Some(0.5), pos_jit: Some(0.03), color: Some(v3::lerp3(albedo, [0.8, 0.78, 0.72], 0.5)), ..Default::default() });
        emit(g, crate::render_particles::PKind::Debris, at, out, m::round(3.0 * s), crate::render_particles::EmitOpts { speed: 1.5, speed_jit: Some(0.7), spread: Some(1.0), life: 14.0, life_jit: Some(0.3), size: 0.006, size_jit: Some(0.5), aspect: Some(2.0), color: Some(v3::copy(albedo)), owner: Some(owner), ..Default::default() });
    } else if surf == crate::scene_materials::Surf::Metal {
        emit(g, crate::render_particles::PKind::Spark, at, v3::normalize(v3::add(v3::scale(refl, 0.6), v3::scale(n, 0.4))), m::round(14.0 * s), crate::render_particles::EmitOpts { speed: 6.5, speed_jit: Some(0.6), spread: Some(1.0), life: 0.42, life_jit: Some(0.4), size: 0.003, ..Default::default() });
    } else if surf == crate::scene_materials::Surf::Concrete {
        emit(g, crate::render_particles::PKind::Debris, at, out, m::round(6.0 * s), crate::render_particles::EmitOpts { speed: 2.2, speed_jit: Some(0.7), spread: Some(1.0), life: 18.0, life_jit: Some(0.3), size: 0.007, size_jit: Some(0.5), color: Some([0.45, 0.44, 0.42]), owner: Some(owner), ..Default::default() });
        dust(g, v3::mad(hit_p, n, 0.08), n, m::round(1.0 + s), crate::render_particles::EmitOpts { shape: Some(crate::render_particles::EmitShape::Puff), speed: 0.3, speed_jit: Some(0.5), lift: Some(0.25), life: 1.1, life_jit: Some(0.3), size: 0.12, size_jit: Some(0.35), pos_jit: Some(0.06), vadd: Some(v3::scale(n, 0.2)), color: Some([0.55, 0.54, 0.52]), ..Default::default() });
    } else if surf == crate::scene_materials::Surf::Glass {
        emit(g, crate::render_particles::PKind::Debris, at, out, m::round(6.0 * s), crate::render_particles::EmitOpts { speed: 2.0, speed_jit: Some(0.7), spread: Some(1.0), life: 16.0, life_jit: Some(0.3), size: 0.005, size_jit: Some(0.5), color: Some([0.1, 0.11, 0.12]), owner: Some(owner), ..Default::default() });
    }
}

/// A cold short cloud of dust into the smoke solver at `p`. Returns whether it went.   (destruction.ts dustPuff)
pub fn destruction_dust_puff(g: &mut crate::game::Game, p: V3, n: V3, scale: f64, surf: crate::scene_materials::Surf) -> bool {
    if g.destr.puffs_this_frame >= g.k.chaos.puffs_per_frame || crate::smoke_types::smoke_system_like_budget(g) < 4.0 || !destruction_take(g, DestructionTakeK::Puff) {
        g.stats.destruction.destr_puffs_refused += 1;
        return false;
    }
    g.destr.puffs_this_frame += 1.0;
    g.stats.destruction.destr_puffs += 1;
    let k = g.k.chaos.puff;
    let by_surf = match surf { crate::scene_materials::Surf::Drywall => 1.0, crate::scene_materials::Surf::Concrete => 0.7, crate::scene_materials::Surf::Fabric => 0.35, _ => 0.5 };
    let dens = k.density * by_surf * m::min(2.0, scale);
    let dir = v3::normalize([n[0], n[1] + 0.25, n[2]]);
    let e = crate::smoke_types::SmokeEmitter { pos: v3::mad(p, n, 0.1), dir, speed: k.speed, radius: k.radius * m::sqrt(scale), density: dens, temperature: k.temp, ttl: k.ttl, age: 0.0, kind: crate::smoke_types::SmokeEmitterKind::Puff, push: Some(k.push), expand: Some(k.expand), attack: Some(0.03), jitter: Some(0.35), lattice: Some(crate::smoke_types::SmokeEmitterLattice::Coarse), prio: Some(0.0), min_voxels: Some(2.0), tint: Some(crate::dust_look::dust_tint(surf)), ..Default::default() };
    crate::smoke_types::smoke_system_like_emit(g, &e);
    crate::out_events::push_smoke_system_like_emit(g, &e);
    true
}

/// May one more pall go now? Counts it when it may. `always`: the wall charge's, never refused, still counted.   (destruction.ts pallTake)
fn destruction_pall_take(g: &mut crate::game::Game, always: Option<bool>) -> bool {
    let always = always.unwrap_or(false);
    if g.destr.show_chaos.is_some() || chaos_of(g) <= 0.0 { return false; }
    let k = g.k.chaos.pall;
    let now = g.time;
    g.destr.pall_at.retain(|&t| now - t < k.window_secs && t <= now);
    if !always && g.destr.pall_at.len() as f64 >= k.per_window {
        g.stats.destruction.pall_refused += 1;
        return false;
    }
    g.destr.pall_at.push(now);
    g.stats.destruction.pall_palls += 1;
    true
}

/// The pall's dice: a hash of the place, the moment and `k`; none of the lane's own are drawn.   (destruction.ts pallHash)
fn destruction_pall_hash(g: &crate::game::Game, at: V3, k: f64) -> f64 {
    crate::debris::hash01(pall_hash_word(at, g.time, k) as f64)
}

/// One emitter of a pall into the solver; whether it was asked. Mirrored out of a blocked place, pushed off what stands by, tightened where it has no room.   (destruction.ts pallEmit)
fn destruction_pall_emit(g: &mut crate::game::Game, r: crate::destruction::DestructionPallEmitR, from: V3, pos: V3, dir: V3, j: f64, tint: V3, gain: Option<f64>) -> bool {
    let gain = gain.unwrap_or(1.0);
    let k = g.k.chaos.pall;
    if crate::smoke_types::smoke_system_like_budget(g) < k.slack {
        g.stats.destruction.pall_starved += 1;
        return false;
    }
    let mut p: V3 = v3::copy(pos);
    let mut d = dir;
    if crate::scene_collision::solid_blocked(g, from, p, None) {
        p = [2.0 * from[0] - pos[0], pos[1], 2.0 * from[2] - pos[2]];
        d = [-dir[0], dir[1], -dir[2]];
        if crate::scene_collision::solid_blocked(g, from, p, None) {
            g.stats.destruction.pall_shut += 1;
            return false;
        }
    }
    p[1] = m::max(0.2, p[1]);
    let y = m::max(k.clear_y, p[1]);
    for _round in 0..2 {
        for w in pall_near(g, p[0], y, p[2], r.radius * k.clear) {
            let qx = p[0] - w[0] * r.radius * 0.5;
            let qz = p[2] - w[1] * r.radius * 0.5;
            if !crate::scene_collision::solid_blocked(g, [p[0], y, p[2]], [qx, y, qz], None) && !crate::scene_collision::solid_blocked(g, p, [qx, p[1], qz], None) {
                p[0] = qx;
                p[2] = qz;
            }
        }
    }
    let mut radius = r.radius;
    while radius > r.radius * k.min_radius && !pall_near(g, p[0], y, p[2], radius * k.clear).is_empty() {
        radius = m::max(r.radius * k.min_radius, radius * 0.8);
    }
    let ttl = r.ttl * (0.8 + 0.4 * j);
    let density = r.density * gain * m::min(k.max_gain, m::pow(r.radius / radius, 1.5));
    let e = crate::smoke_types::SmokeEmitter { pos: p, dir: d, speed: r.speed * (0.75 + 0.5 * j), radius, density, temperature: r.temp, ttl, age: 0.0, kind: crate::smoke_types::SmokeEmitterKind::Puff, push: Some(r.push), expand: Some(r.expand), attack: Some(0.05), jitter: Some(0.35), lattice: Some(crate::smoke_types::SmokeEmitterLattice::Coarse), prio: Some(1.0), min_voxels: Some(3.0), tint: Some(tint), ..Default::default() };
    crate::smoke_types::smoke_system_like_emit(g, &e);
    crate::out_events::push_smoke_system_like_emit(g, &e);
    g.stats.destruction.pall_emitters += 1;
    true
}

/// A wall charge's pall: `p` on the wall's face at the floor, `n` off it, `t` along it; the far side's part only if a way was opened.   (destruction.ts pallWall)
pub fn destruction_pall_wall(g: &mut crate::game::Game, p: V3, n: V3, t: V3, thick: f64, through: bool, surf: Option<crate::scene_materials::Surf>, gain: Option<f64>) {
    let surf = surf.unwrap_or(crate::scene_materials::Surf::Drywall);
    let gain = gain.unwrap_or(1.0);
    if !destruction_pall_take(g, Some(true)) { return; }
    let w = g.k.chaos.pall.wall;
    let r = DestructionPallEmitR { radius: w.radius, density: w.density, ttl: w.ttl, speed: w.speed, temp: w.temp, push: w.push, expand: w.expand };
    let tint = crate::dust_look::dust_tint(surf);
    for side in [1.0, -1.0] {
        if side < 0.0 && !through { break; }
        let near_side = side > 0.0;
        let count_n = if near_side { w.near } else { w.far };
        let base = v3::mad(p, n, if near_side { 0.08 } else { -(thick + 0.08) });
        let nn = v3::scale(n, side);
        let mut took = 0;
        let mut k = 0.0;
        while k < count_n {
            let j = destruction_pall_hash(g, p, k * 2.0 + (if near_side { 0.0 } else { 41.0 }));
            let j2 = destruction_pall_hash(g, p, k * 2.0 + (if near_side { 1.0 } else { 42.0 }));
            let du = if count_n > 1.0 { (k / (count_n - 1.0) - 0.5) * 2.0 * w.spread } else { 0.0 };
            let pos: V3 = [base[0] + nn[0] * w.out * (0.6 + 0.8 * j2) + t[0] * du, 0.45 + 1.1 * j, base[2] + nn[2] * w.out * (0.6 + 0.8 * j2) + t[2] * du];
            let from: V3 = [base[0] + nn[0] * 0.1, 1.0, base[2] + nn[2] * 0.1];
            let dir = v3::normalize([nn[0] + t[0] * du * 0.35, 0.3, nn[2] + t[2] * du * 0.35]);
            if destruction_pall_emit(g, r, from, pos, dir, j2, tint, Some(gain)) { took += 1; }
            k += 1.0;
        }
        let count = m::round(w.dust[if near_side { 0 } else { 1 }] * gain);
        if count > 0.0 && took > 0 {
            let dust = crate::particle_fx::emit_dust(g, [base[0] + nn[0] * 0.3, 0.9, base[2] + nn[2] * 0.3], v3::normalize([nn[0], 0.25, nn[2]]), count, crate::render_particles::EmitOpts { shape: Some(crate::render_particles::EmitShape::Puff), speed: w.dust_speed, speed_jit: Some(0.7), lift: Some(0.12), life: w.dust_life, life_jit: Some(0.35), size: w.dust_size, size_jit: Some(0.4), pos_jit: Some(0.6), color: Some(v3::scale(tint, 0.8)), ..Default::default() });
            g.stats.destruction.pall_dust += dust as i32;
        }
    }
}

/// A burst's pall: the dust it kicks up in a ring round `at` and the smoke over it; `size` 1 = the hand grenade's.   (destruction.ts pallBurst)
fn destruction_pall_burst(g: &mut crate::game::Game, at: V3, size: Option<f64>) {
    let size = size.unwrap_or(1.0);
    if !destruction_pall_take(g, None) { return; }
    let b = g.k.chaos.pall.burst;
    let r = DestructionPallEmitR { radius: b.radius, density: b.density, ttl: b.ttl, speed: b.speed, temp: b.temp, push: b.push, expand: b.expand };
    let s = m::max(0.6, m::min(1.6, size));
    let tint = crate::dust_look::dust_tint(crate::scene_materials::Surf::Concrete);
    let c: V3 = [at[0], m::max(0.3, at[1]), at[2]];
    let ring = m::max(0.0, m::round(b.ring * s));
    let a0 = destruction_pall_hash(g, at, 97.0) * m::PI * 2.0;
    let mut took = 0;
    let mut k = 0.0;
    while k < ring {
        let j = destruction_pall_hash(g, at, k * 2.0);
        let j2 = destruction_pall_hash(g, at, k * 2.0 + 1.0);
        let a = a0 + (k + 0.5 * j) * m::PI * 2.0 / ring;
        let d: V3 = [m::sin(a), 0.0, m::cos(a)];
        let rr = b.spread * s * (0.7 + 0.6 * j2);
        if destruction_pall_emit(g, r, c, [c[0] + d[0] * rr, c[1] + 0.1 + 0.5 * j, c[2] + d[2] * rr], v3::normalize([d[0], 0.45, d[2]]), j2, tint, None) { took += 1; }
        k += 1.0;
    }
    let core = DestructionPallEmitR { temp: b.temp * 2.5, ..r };
    let jc = destruction_pall_hash(g, at, 201.0);
    if destruction_pall_emit(g, core, c, [c[0], c[1] + 0.45, c[2]], [0.0, 1.0, 0.0], jc, b.smoke, Some(b.core_gain)) { took += 1; }
    if b.dust > 0.0 && took > 0 {
        let dust = crate::particle_fx::emit_dust(g, [c[0], c[1] + 0.2, c[2]], [0.0, 1.0, 0.0], m::round(b.dust * s), crate::render_particles::EmitOpts { shape: Some(crate::render_particles::EmitShape::Puff), speed: b.dust_speed, speed_jit: Some(0.8), lift: Some(0.18), life: b.dust_life, life_jit: Some(0.35), size: b.dust_size, size_jit: Some(0.4), pos_jit: Some(0.5 * s), color: Some(v3::scale(tint, 0.75)), ..Default::default() });
        g.stats.destruction.pall_dust += dust as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the words the TypeScript's own expression gives (run under bun), Math.round's halves and imul's wrap among them
    #[test]
    fn pall_hash_word_is_the_typescripts() {
        assert_eq!(pall_hash_word([1.37, 0.5, -2.19], 12.345, 97.0), 729214853);
        assert_eq!(pall_hash_word([-40.125, 1.0, 33.3], 0.0, 0.0), 3501533476);
        assert_eq!(pall_hash_word([0.0078125, 0.0, -0.0078125], 601.25, 201.0), 2599756042);
        assert_eq!(pall_hash_word([123.456, 0.0, -987.654], 3600.5, 83.0), 2154646853);
    }

    #[test]
    fn a_box_index_out_of_range_is_no_box() {
        assert_eq!(box_at(1, 0.0), Some(0));
        assert_eq!(box_at(1, 1.0), None);
        assert_eq!(box_at(1, -1.0), None);
    }
}
