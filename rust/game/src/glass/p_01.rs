//! src/game/glass.ts lines 1 to 255
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- seams: what skeleton 5 still cannot give ----

/// a box's row (Box.* in the TypeScript). Skeleton 5 has `g.col.boxes: Vec<BoxId>` and no table of Box rows; where the table comes back this is the one line `&mut g.<table>[b]`
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

// ---- private helpers ----

/// `g.level.boxes[i]`: undefined off the list, as JavaScript's (a negative, fractional or NaN index included). That list and `g.col.boxes` are one array in the TypeScript (game.ts:1219): the box at place i
fn box_at(g: &mut Game, i: f64) -> Option<BoxId> {
    let ix = i as usize;
    if ix as f64 != i { return None; }
    g.col.boxes.get(ix).copied()
}

/// the seed's word of a box's centre and half sizes (glass.ts:113 boxSeed, with the box read)
fn box_word(c: V3, hs: V3) -> f64 {
    let mut h = 0x9E3779B1u32 as f64;
    for k in 0..3 { h = seed_mix(h, c[k]); }
    for k in 0..3 { h = seed_mix(h, hs[k]); }
    imul_word(h, 0xC2B2AE3D)
}

/// the thin axis's world direction of a box of this turn and these half sizes (glass.ts:77 paneNormal, with the box read)
fn normal_of(yaw: f64, h: V3) -> V3 {
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    if h[0] < h[2] { [cs, 0.0, -sn] } else { [sn, 0.0, cs] }
}

/// `flags & BoxFlag.Glass`, as a test
fn glass_flagged(flags: f64) -> bool {
    (js::to_i32(flags) & js::to_i32(crate::scene_boxes::box_flag().glass)) != 0
}

/// the blower's own noise: `pl.noise = max(pl.noise, GLASS.hearLevel)` where `who` is a man with a seat
fn raise_noise(g: &mut Game, who: f64) {
    if let Some(pl) = crate::seats::man_by_id(g, who) {
        g.pl[pl].noise = m::max(g.pl[pl].noise, g.k.glass.hear_level);
    }
}

/// `Math.imul(h, k) >>> 0`, as a number
fn imul_word(h: f64, k: u32) -> f64 {
    m::imul(js::to_i32(h), k as i32) as u32 as f64
}

/// one draw of a blast's word (glass.ts:211)
fn blast_step(s: u32) -> u32 {
    (m::imul((s ^ (s >> 15)) as i32, 0x2C1B3C6Du32 as i32) as u32).wrapping_add(0x297A2D39)
}

// ---- the part ----

/// what a URL or env word said for this run; the host sets the field as a game starts (glass.ts glassFxForced)
pub fn glass_fx_forced(g: &mut crate::game::Game) -> Option<bool> {
    g.m.glass.fx_forced
}

/// the choice kept on this machine: the browser's storage is the host's, which sets g.k.glassfx and g.k.glass.crack_gap; with no storage the TypeScript's own try does nothing (glass.ts loadGlassFx)
pub fn load_glass_fx(g: &mut crate::game::Game) {}

/// the choice kept on this machine: the browser's storage is the host's; with no storage the TypeScript's own try does nothing (glass.ts saveGlassFx)
pub fn save_glass_fx(g: &mut crate::game::Game) {}

/// before a Game or an Engine exists in the TypeScript: the switch as this boot will have it (glass.ts glassFxBoot)
pub fn glass_fx_boot(g: &mut crate::game::Game) -> bool {
    load_glass_fx(g);
    g.k.glassfx.on
}

/// per-Game glass bookkeeping, made at first asking: the row itself (glass.ts glassState)
pub fn glass_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::glass::GlassState {
    if g.m.glass.states.is_none() {
        let mut panes: Vec<f64> = Vec::new();
        for i in 0..g.col.boxes.len() {
            let b = g.col.boxes[i];
            if glass_flagged(seam_box(g, b).flags) { panes.push(i as f64); }
        }
        g.m.glass.states = Some(GlassState { panes, seen: JsSet::new(), hits: 0.0, last_hit_t: -100.0, log: Vec::new(), event_at: JsMap::new(), crack_at: -1e9 });
    }
    g.m.glass.states.as_mut().unwrap()
}

/// beside decals.clear(): the encounter's hit count, last-hit clock and log go; the pane list and the seen set stand (glass.ts resetGlass)
pub fn reset_glass(g: &mut crate::game::Game) {
    let s = match g.m.glass.states.as_mut() { Some(s) => s, None => return };
    s.hits = 0.0;
    s.last_hit_t = -100.0;
    s.log.clear();
    s.event_at.clear();
    s.crack_at = -1e9;
}

/// every static Glass box index of this level (glass.ts glassPanes)
pub fn glass_panes(g: &mut crate::game::Game) -> Vec<f64> {
    glass_state(g).panes.clone()
}

/// a box flagged Glass (glass.ts isPane)
pub fn is_pane(g: &mut crate::game::Game, b: Option<BoxId>) -> bool {
    match b {
        Some(b) => glass_flagged(seam_box(g, b).flags),
        None => false,
    }
}

/// a pane that can craze (thin); solid glass is glass but no sheet (glass.ts isSheet)
pub fn is_sheet(g: &mut crate::game::Game, b: Option<BoxId>) -> bool {
    if !is_pane(g, b) { return false; }
    let h = seam_box(g, b.unwrap()).h;
    m::min(h[0], h[2]) <= THICK_GLASS_HALF
}

/// the pane's unit normal: the world direction of its thin axis, sign arbitrary (glass.ts paneNormal)
pub fn pane_normal(g: &mut crate::game::Game, b: BoxId) -> V3 {
    let (yaw, h) = { let row = seam_box(g, b); (row.yaw, row.h) };
    normal_of(yaw, h)
}

/// files a pane's crash for the floor's ears, heard from both its faces (glass.ts glassEvent)
pub fn glass_event(g: &mut crate::game::Game, b: BoxId, at: V3, level: f64, who: f64) {
    let serial = g.serial();
    let mut e = crate::game::GameEvent { kind: crate::director::EventKind::Glass, pos: [at[0], 0.0, at[2]], time: g.time, loud: true, level: Some(level), who: Some(who), serial, ..crate::game::GameEvent::zeroed() };
    let n = pane_normal(g, b);
    let (c, h) = { let row = seam_box(g, b); (row.c, row.h) };
    let off = v3::dot(v3::sub(at, c), n);
    let r = m::min(h[0], h[2]) + g.k.glass.ear_off;
    e.ears = Some([
        [at[0] + n[0] * (r - off), 0.0, at[2] + n[2] * (r - off)],
        [at[0] - n[0] * (r + off), 0.0, at[2] - n[2] * (r + off)],
    ]);
    g.events.push(e);
}

/// nearest point of box b (yawed, no rot) to p, and the distance (glass.ts nearestOnBox)
pub fn nearest_on_box(g: &mut crate::game::Game, b: BoxId, p: V3) -> crate::glass::NearestOnBox {
    let (yaw, c, h) = { let row = seam_box(g, b); (row.yaw, row.c, row.h) };
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    let (dx, dy, dz) = (p[0] - c[0], p[1] - c[1], p[2] - c[2]);
    let lx = v3::clamp(cs * dx - sn * dz, -h[0], h[0]);
    let ly = v3::clamp(dy, -h[1], h[1]);
    let lz = v3::clamp(sn * dx + cs * dz, -h[2], h[2]);
    let q: V3 = [c[0] + cs * lx + sn * lz, c[1] + ly, c[2] - sn * lx + cs * lz];
    NearestOnBox { q, d: v3::dist(p, q) }
}

/// one number of a seed, to the millimetre, folded into the word so far (glass.ts seedMix)
pub fn seed_mix(h: f64, x: f64) -> f64 {
    let h = m::imul(js::to_i32(h) ^ js::to_i32(m::round(x * 1000.0)), 0x85EBCA77u32 as i32) as u32;
    (h ^ (h >> 13)) as f64
}

/// what a box is, as one 32-bit word: its centre and half sizes to the millimetre (glass.ts boxSeed)
pub fn box_seed(g: &mut crate::game::Game, b: BoxId) -> f64 {
    let (c, hs) = { let row = seam_box(g, b); (row.c, row.h) };
    box_word(c, hs)
}

/// a box's word and a point to the millimetre, as one 32-bit word (glass.ts pointSeed)
pub fn point_seed(word: f64, p: V3) -> f64 {
    let mut h = js::to_u32(word) as f64;
    for k in 0..3 { h = seed_mix(h, p[k]); }
    imul_word(h, 0xC2B2AE3D)
}

/// a 32-bit hash of where a box was hit (glass.ts hitSeed)
pub fn hit_seed(g: &mut crate::game::Game, b: BoxId, p: V3) -> f64 {
    point_seed(box_seed(g, b), p)
}

/// the seed of a blast on a box: the charge's point on the floor plan, its height left out (glass.ts blastSeed)
pub fn blast_seed(word: f64, at: V3) -> f64 {
    point_seed(word, [at[0], 0.0, at[2]])
}

/// THE IMPACT ENTRY POINT: a blow of `energy` met pane `pane` at `point` flying `dir`: the fracture filed, the crack decal, the crack heard, the event on the floor's ears (glass.ts glassImpact)
pub fn glass_impact(g: &mut crate::game::Game, pane: f64, point: V3, dir: Option<V3>, energy: f64, who: f64, cause: Option<crate::glass::GlassImpactCause>, quiet: Option<bool>, opts: Option<crate::glass::GlassImpactOpts>) -> crate::glassbreak::GlassImpact {
    let cause = cause.unwrap_or(GlassImpactCause::Shot);
    let quiet = quiet.unwrap_or(false);
    let through = opts.and_then(|o| o.through);
    let bx = box_at(g, pane);
    let mut res = crate::glassbreak::GlassImpact { pane: is_pane(g, bx), live: false, type_: crate::glassbreak::GlassType::Laminated, thickness: 0.0, was_open: false, punched: false, diced: false, released: 0.0, cracked: false };
    if !res.pane { return res; }
    let b = bx.unwrap();
    let h = seam_box(g, b).h;
    res.thickness = 2.0 * m::min(h[0], h[2]);
    if m::min(h[0], h[2]) > THICK_GLASS_HALF { return res; }   // thick glass: the round stops, no star, no crack, no event
    // glass breaking first: a blow through a hole stars nothing, a pane that dices takes no star; a blast files its own
    if cause != GlassImpactCause::Bang {
        res = crate::glassbreak::glass_break_impact(g, pane, point, dir, energy, who, through);
    } else {
        res.type_ = crate::glassbreak::glass_break_type_of_pane(g, pane).unwrap_or(crate::glassbreak::GlassType::Laminated);
    }
    if res.was_open { return res; }
    glass_state(g);   // made here at the latest, as `const S = glassState(g)`; written below through the row
    let seed = hit_seed(g, b, point);
    let word = js::to_u32(seed);
    let c = seam_box(g, b).c;
    let mut n = pane_normal(g, b);
    let flip = match dir { Some(d) => v3::dot(n, d) > 0.0, None => v3::dot(n, v3::sub(point, c)) < 0.0 };
    if flip { n = v3::scale(n, -1.0); }
    if res.live && cause == GlassImpactCause::Shot && !res.diced && res.released == 0.0 {
        let albedo = seam_box(g, b).albedo;
        crate::shards::glass_shards_spall(g, point, n, dir, res.thickness, albedo, seed, Some(through != Some(true)));
    }
    let off = v3::dot(v3::sub(point, c), n);
    let pos = v3::mad(point, n, -off);   // onto the mid-plane
    let u = (word & 0xffff) as f64 / 65535.0;
    let w = (word >> 16) as f64 / 65535.0;
    let radius = g.k.glass.r_min + (g.k.glass.r_max - g.k.glass.r_min) * u;
    let ang = w * m::PI * 2.0;
    let up: V3 = [0.0, 1.0, 0.0];
    let side = v3::normalize(v3::cross(up, n));
    let tangent: V3 = v3::add(v3::scale(side, m::cos(ang)), v3::scale(up, m::sin(ang)));   // the star's orientation in the pane's plane
    if !res.live || !res.cracked || g.engine.glass_fracture != Some(true) {
        let param = if cause == GlassImpactCause::Bang { 0.6 } else { 1.0 };
        let o = crate::render_decals::DecalOpts { tangent: Some(tangent), seed: Some(((word >> 8) & 0xfff) as f64), pane: Some(pane), param: Some(param), ..Default::default() };
        crate::render_decals::decals_add(g, crate::render_decals::DecalKind::GlassHit, pos, n, Some(radius), Some(o));
    }
    let time = g.time;
    let s = glass_state(g);
    s.hits += 1.0;
    s.last_hit_t = time;
    let what = if cause == GlassImpactCause::Bang { "starred by a blast" } else { "hit" };
    let by = if js::truthy(who) { format!(" (who {})", js::num_str(who)) } else { String::new() };
    s.log.push(format!("{}s pane {} {}{} r {}", js::to_fixed(time, 1), js::num_str(pane), what, by, js::to_fixed(radius, 2)));
    if s.log.len() > 24 { s.log.remove(0); }
    if quiet { return res; }
    // with the fracture model live a pane is hit by bursts: one 'glass' event per pane per shooter per gap, no two crack voices inside crackGap across the floor
    if res.live {
        let (crack_gap, live_gap, hear_level) = (g.k.glass.crack_gap, g.k.glass.live_gap, g.k.glass.hear_level);
        let s = glass_state(g);
        let since_any = time - s.crack_at;
        let sound = !(crack_gap > 0.0 && since_any >= 0.0 && since_any < crack_gap);
        if sound { s.crack_at = time; }
        let key = pane * 1024.0 + (js::to_i32(who) & 1023) as f64;
        let since = match s.event_at.get(crate::Num(key)) { None => 1e9, Some(last) => time - *last };
        if live_gap > 0.0 && since >= 0.0 && since < live_gap {
            if sound { crate::audio_iface::game_audio_like_play(g, "glassCrack", Some(point), Some(0.5), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.94 + 0.08 * u + 0.3 * w })); }
            raise_noise(g, who);
            return res;
        }
        if live_gap > 0.0 { s.event_at.set(crate::Num(key), time); }
        if !sound {   // the event is the floor's, the voice is skipped
            glass_event(g, b, point, hear_level, who);
            raise_noise(g, who);
            return res;
        }
    }
    // the crack: at the hole, on the floor's ears as its own kind; small panes ring higher
    let area = 4.0 * h[1] * m::max(h[0], h[2]);
    crate::audio_iface::game_audio_like_play(g, "glassCrack", Some(point), Some(0.9), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.94 + m::min(0.22, 0.4 / m::max(0.5, area)) + 0.08 * u }));
    let hear_level = g.k.glass.hear_level;
    glass_event(g, b, point, hear_level, who);
    raise_noise(g, who);
    res
}

/// a round (or anything else that stars glass) met pane `bi` = glassImpact at energy 1; false if `bi` is not a pane (glass.ts glassHit)
pub fn glass_hit(g: &mut crate::game::Game, bi: f64, hit_p: V3, dir: Option<V3>, who: f64, cause: Option<crate::glass::GlassHitCause>, quiet: Option<bool>) -> bool {
    let cause = match cause.unwrap_or(GlassHitCause::Shot) {
        GlassHitCause::Shot => GlassImpactCause::Shot,
        GlassHitCause::Bang => GlassImpactCause::Bang,
        GlassHitCause::Hand => GlassImpactCause::Hand,
    };
    glass_impact(g, bi, hit_p, dir, 1.0, who, Some(cause), quiet, None).pane
}

/// stars every sheet whose nearest point is within `radius` of `at`, GLASS.bangCracks quiet hits each, seeded off the blast; how many panes took it (glass.ts crackNear)
pub fn crack_near(g: &mut crate::game::Game, at: V3, radius: f64, who: f64) -> f64 {
    let mut n = 0.0;
    for bi in glass_panes(g) {
        let bx = box_at(g, bi);
        if !is_sheet(g, bx) { continue; }   // solid glass does not craze
        let b = bx.unwrap();
        let near = nearest_on_box(g, b, at);
        if near.d > radius { continue; }
        let q = near.q;
        let word = box_seed(g, b);
        let mut s = js::to_u32(blast_seed(word, at));
        let (c, h, yaw) = { let row = seam_box(g, b); (row.c, row.h, row.yaw) };
        let wide = if h[0] < h[2] { 2 } else { 0 };   // the pane's long horizontal local axis
        let (cs, sn) = (m::cos(yaw), m::sin(yaw));
        let axis: V3 = if wide == 0 { [cs, 0.0, -sn] } else { [sn, 0.0, cs] };
        let mut k = 0.0;
        while k < g.k.glass.bang_cracks {
            s = blast_step(s);
            let fu = ((s & 0xffff) as f64 / 65535.0) * 2.0 - 1.0;
            let fv = ((s >> 16) as f64 / 65535.0) * 2.0 - 1.0;
            let anywhere = v3::mad(v3::mad(c, axis, fu * 0.9 * h[wide]), [0.0, 1.0, 0.0], fv * 0.9 * h[1]);   // a point on the pane's face, inside its frame by a tenth
            let p = v3::lerp3(anywhere, q, 0.35);   // pulled a third of the way toward where the blast was nearest
            glass_hit(g, bi, p, Some(v3::sub(p, at)), who, Some(GlassHitCause::Bang), Some(true));
            k += 1.0;
        }
        n += 1.0;
    }
    n
}

/// once a frame: a stun charge beside the glazing stars the panes near it, each 'bang' event looked at once (glass.ts updateGlass)
pub fn update_glass(g: &mut crate::game::Game) {
    glass_state(g);
    // a live index: glass_break_bang can file 'glass' events on the list under the walk
    let mut i = 0;
    while i < g.events.len() {
        let e = g.events[i];
        i += 1;
        if e.kind != crate::director::EventKind::Bang { continue; }
        let s = glass_state(g);
        if s.seen.has(e.serial) { continue; }
        s.seen.add(e.serial);
        let at: V3 = [e.pos[0], m::max(0.3, e.pos[1]), e.pos[2]];
        let who = e.who.unwrap_or(0.0);
        let bang_m = g.k.glass.bang_m;
        crack_near(g, at, bang_m, who);
        crate::glassbreak::glass_break_bang(g, at, who);
    }
}

/// the panel's 'crack test': star the sheet on the line from Sam's chest through the aim point (and 6 m beyond), else the pane nearest the aim point within 4 m; the pane or -1 (glass.ts crackTest)
pub fn crack_test(g: &mut crate::game::Game) -> f64 {
    let aim = g.aim_point;
    let c = g.pl[g.player].ch;
    let from: V3 = v3::add(g.ch[c].pos, [0.0, 1.3, 0.0]);
    let d = v3::sub(aim, from);
    let l = v3::len(d);
    if l > 1e-3 {
        let rd = v3::scale(d, 1.0 / l);
        if let Some(h) = crate::scene_collision::raycast(g, from, rd, l + 6.0) {
            let bx = box_at(g, h.index);
            if h.index < 1e6 && is_sheet(g, bx) {
                glass_hit(g, h.index, v3::mad(from, rd, h.t), Some(rd), 0.0, Some(GlassHitCause::Hand), None);
                return h.index;
            }
        }
    }
    let (mut best, mut best_d, mut best_q) = (-1.0, 4.0, None);
    for bi in glass_panes(g) {
        let b = g.col.boxes[bi as usize];
        let near = nearest_on_box(g, b, aim);
        if near.d < best_d { best_d = near.d; best = bi; best_q = Some(near.q); }
    }
    if let Some(q) = best_q {
        glass_hit(g, best, q, Some(v3::sub(q, from)), 0.0, Some(GlassHitCause::Hand), None);
    }
    best
}

/// how many crack decals are live (glass.ts crackCount)
pub fn crack_count(g: &mut crate::game::Game) -> f64 {
    let mut n = 0.0;
    for d in g.decals.list.iter() {
        if d.kind == crate::render_decals::DecalKind::GlassHit { n += 1.0; }
    }
    n
}

/// the panel's line (glass.ts glassSummary)
pub fn glass_summary(g: &mut crate::game::Game) -> String {
    let panes = glass_panes(g).len();
    let cracks = crack_count(g);
    let listed = g.engine.world.glass_list.len();
    let s = glass_state(g);
    let last = match s.log.last() { Some(l) => format!(" · last: {}", l), None => String::new() };
    format!("{} panes (laminated: rounds stop, glass stays) · {} crack stars live, {} hits this encounter · listed to GPU {}{}", panes, js::num_str(cracks), js::num_str(s.hits), listed, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every expected value below is the TypeScript's own, under bun with EXACT.on.

    #[test]
    fn point_and_blast_seeds_are_the_typescript_s() {
        let cases: [(f64, V3, f64, f64); 6] = [
            (0.0, [0.0, 0.0, 0.0], 0.0, 0.0),
            (2654435761.0, [1.2345, -0.0005, 0.0005], 2481012589.0, 2481012589.0),
            (4294967295.0, [-12.75, 3.3, 1000.125], 2188744601.0, 52689921.0),
            (123456789.0, [3000000.5, -3000000.5, 2.5e-4], 121053404.0, 3273712062.0),
            (-1.0, [0.1, 0.2, 0.3], 3580937442.0, 2818706876.0),
            (4294967296.5, [7.0, -7.0, 14.0014], 2336966134.0, 2956857472.0),
        ];
        for (word, p, point, blast) in cases {
            assert_eq!(point_seed(word, p).to_bits(), f64::to_bits(point), "point {} {:?}", word, p);
            assert_eq!(blast_seed(word, p).to_bits(), f64::to_bits(blast), "blast {} {:?}", word, p);
        }
    }

    #[test]
    fn a_blast_s_draws_are_the_typescript_s() {
        let cases: [(u32, u32); 6] = [
            (0, 695872825),
            (1, 1435855270),
            (2147483648, 3857132857),
            (4294967295, 2963287353),
            (2654435761, 4051382572),
            (305419896, 4231691785),
        ];
        for (s, next) in cases { assert_eq!(blast_step(s), next, "{}", s); }
    }

    // bun's own values for four boxes: two sheets, a solid pane, a box that is no glass. The bodies that read a box's row wait on seam_box; the arithmetic they do with it is here.
    const FOUR: [(V3, V3, f64, f64); 4] = [
        ([1.2345, 1.5, -3.25], [0.02, 1.5, 2.0], 0.0, 131072.0),
        ([10.0, 0.4, 7.777], [1.1, 0.4, 0.05], 0.7, 131074.0),
        ([-4.5, 2.0, 0.001], [0.06, 1.0, 0.5], 1.3, 131072.0),
        ([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], 0.0, 1.0),
    ];

    #[test]
    fn a_box_s_seeds_and_normal_are_the_typescript_s() {
        let want: [(f64, f64, bool, [u64; 3]); 4] = [
            (3611120674.0, 455261860.0, true, [0x3ff0000000000000, 0x0, 0x8000000000000000]),
            (4014362709.0, 3716110216.0, true, [0x3fe49d6e694619b8, 0x0, 0x3fe87996529f9d92]),
            (1438644873.0, 1021307691.0, true, [0x3fd11eb3682a4c5f, 0x0, 0xbfeed577f9c51e4b]),
            (3504899977.0, 1130125206.0, false, [0x0, 0x0, 0x3ff0000000000000]),
        ];
        for (i, (word, hit, pane, n)) in want.into_iter().enumerate() {
            let (c, h, yaw, flags) = FOUR[i];
            assert_eq!(box_word(c, h), word, "{}", i);
            assert_eq!(point_seed(box_word(c, h), [0.5, 1.25, -0.125]), hit, "{}", i);
            assert_eq!(glass_flagged(flags), pane, "{}", i);
            let got = normal_of(yaw, h);
            assert_eq!([got[0].to_bits(), got[1].to_bits(), got[2].to_bits()], n, "{}", i);
        }
    }

    #[test]
    fn the_row_is_the_row() {
        let mut g = Game::blank();
        assert_eq!(glass_panes(&mut g), Vec::<f64>::new());
        glass_state(&mut g).hits = 3.0;
        glass_state(&mut g).log.push(String::from("x"));
        assert_eq!(g.m.glass.states.as_ref().unwrap().hits, 3.0);
        reset_glass(&mut g);
        let s = g.m.glass.states.as_ref().unwrap();
        assert_eq!((s.hits, s.last_hit_t, s.log.len(), s.crack_at), (0.0, -100.0, 0, -1e9));
    }

    #[test]
    fn an_event_of_another_kind_is_not_looked_at() {
        let mut g = Game::blank();
        let e = crate::game::GameEvent { kind: crate::director::EventKind::Glass, serial: 7, ..crate::game::GameEvent::zeroed() };
        g.events.push(e);
        update_glass(&mut g);
        assert!(!glass_state(&mut g).seen.has(7));
    }

    #[test]
    fn off_the_list_is_no_box() {
        let mut g = Game::blank();
        for i in [-1.0, 0.5, f64::NAN, 3.0] { assert_eq!(box_at(&mut g, i), None, "{}", i); }
    }
}
