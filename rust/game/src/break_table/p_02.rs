//! src/game/breakTable.ts lines 333 to 738
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- seams: what skeleton 3 still cannot say (see notes.md)
/// BREAK[mat]: the table of rows came out as `pub const BREAK: bool = false` (breakTable.ts:61)
fn seam_break_row(mat: Mat) -> BreakRow { todo!("needs: BREAK's rows by Mat") }
/// P.emit('debris', …): EmitOpts lacks speedJit, spread, lifeJit, sizeJit, aspect, color, owner; gpuFx hands the particles by value
fn seam_emit_debris(g: &mut Game, pos: V3, dir: V3, count: f64, speed: f64, speed_jit: f64, spread: f64, life: f64, life_jit: f64, size: f64, size_jit: f64, aspect: f64, pos_jit: f64, color: V3, owner: f64) { todo!("needs: EmitOpts' whole shape, particles by reference") }
/// P.emit('spark', …): as seam_emit_debris
fn seam_emit_spark(g: &mut Game, pos: V3, dir: V3, count: f64, speed: f64, speed_jit: f64, spread: f64, life: f64, life_jit: f64, size: f64) { todo!("needs: EmitOpts' whole shape, particles by reference") }
/// emitDust(P, …, { shape: 'puff', … }): as seam_emit_debris
fn seam_emit_dust(g: &mut Game, pos: V3, dir: V3, count: f64, speed: f64, speed_jit: f64, lift: f64, life: f64, life_jit: f64, size: f64, size_jit: f64, pos_jit: f64, vadd: Option<V3>, color: V3) { todo!("needs: EmitOpts' whole shape, particles by reference") }

// ---------------------------------------------------------------- small private helpers
fn max3(h: V3) -> f64 { m::max_n(&[h[0], h[1], h[2]]) }
/// `this.look?.on`: EDGE is lent by reference in the TypeScript and read live, so `look` says only that somebody is there
fn look_on(g: &Game) -> bool { g.destr.breaks.look.is_some() && g.m.edge_look.edge.on }
/// SURF_MAT[surf as Surf]: a word that is no Surf finds nothing
fn surf_mat_of_word(s: &str) -> Option<Mat> {
    use crate::scene_materials::Surf;
    for w in [Surf::Carpet, Surf::Metal, Surf::Glass, Surf::Wood, Surf::Plastic, Surf::Cardboard, Surf::Drywall, Surf::Concrete, Surf::Fabric, Surf::Foliage] {
        if w.as_str() == s { return Some(mat_of_surf(w)); }
    }
    None
}
/// matOf(kind, box) for a box of the world
fn mat_of_box(g: &mut Game, kind: crate::debris::ChunkKind, b: BoxId) -> Mat {
    let box_ = MatOfBox { does: g.col.boxes[b].does.map(|w| String::from(w.as_str())) };
    mat_of(g, kind, &box_)
}
/// `c.mat ?? { mat: matOf(c.kind, c.box), gen: 0, paint: c.box.albedo }`
fn meta_of(g: &mut Game, c: DebrisChunkId) -> BreakMeta {
    if let Some(m) = g.tab.debris_chunk[c].mat { return m; }
    let (kind, b) = (g.tab.debris_chunk[c].kind, g.tab.debris_chunk[c].box_);
    let mat = mat_of_box(g, kind, b);
    BreakMeta { mat, gen_: 0.0, paint: g.col.boxes[b].albedo, fixed: None }
}
/// the axes sorted by half extent, smallest first (the sort is stable)
fn axes_by_size(h: V3) -> [usize; 3] {
    let mut ax = [0usize, 1, 2];
    js::sort_by(&mut ax, |i, j| h[*i] - h[*j]);
    ax
}
fn word_of_box(g: &Game, b: BoxId) -> crate::scene_boxes::WordOf {
    let (stuff, does) = (g.col.boxes[b].stuff, g.col.boxes[b].does);
    crate::scene_boxes::word_of(crate::scene_boxes::WordOfB { stuff, does })
}
fn cut_len(cut: &Option<Vec<crate::scene_cut::CutPlane>>) -> i32 { cut.as_ref().map_or(0, |c| c.len() as i32) }
/// the key of a chunk's second drawn box (breakTable.ts:581,603)
fn mate_mkey(g: &Game, c: DebrisChunkId) -> f64 {
    crate::scene_boxes::mkey().debris + (js::to_i32(g.tab.debris_chunk[c].serial + 32768.0) & 65535) as f64
}

// ---------------------------------------------------------------- the dice, the pool's plain questions
/// (breakTable.ts Breaks.constructor)
pub fn breaks_new(g: &mut crate::game::Game) -> crate::break_table::Breaks { Breaks::blank() }

/// The die that lays out what a cell throws. (breakTable.ts Breaks.hash)
pub fn breaks_hash(g: &mut crate::game::Game) -> f64 {
    let n = g.destr.breaks.counter;
    g.destr.breaks.counter = n + 1.0;
    crate::debris::hash01(n * 2246822519.0)
}

/// The pool's own die; with DEBRIS.dress off there is one die. (breakTable.ts Breaks.phash)
pub fn breaks_phash(g: &mut crate::game::Game) -> f64 {
    if g.k.debris.dress {
        let n = g.destr.breaks.pcounter;
        g.destr.breaks.pcounter = n + 1.0;
        crate::debris::hash01(n * 2246822519.0 + 5369127.0)
    } else {
        breaks_hash(g)
    }
}

/// (breakTable.ts Breaks.reset)
pub fn breaks_reset(g: &mut crate::game::Game) {
    let s = &mut g.destr.breaks;
    s.counter = 1.0;
    s.pcounter = 1.0;
    s.queue.clear();
    s.kink_t = -1e9;
}

/// (breakTable.ts Breaks.small)
pub fn breaks_small(g: &mut crate::game::Game) -> bool { crate::debris::debris_cap(g) < 64.0 }

/// A chunk's serial counted from the pool's last clear. (breakTable.ts Breaks.rel)
pub fn breaks_rel(g: &mut crate::game::Game, c_2: DebrisChunkId) -> f64 {
    g.tab.debris_chunk[c_2].serial - g.destr.debris.serial0 + 1.0
}

/// The rigid pool's share of one blast. (breakTable.ts Breaks.share)
pub fn breaks_share(g: &mut crate::game::Game) -> crate::break_table::BreaksShare {
    let s = if breaks_small(g) && !g.k.debris.dress {
        let t = g.k.breaks.share.small;
        BreaksShare { thrown: t.thrown, heap: t.heap, furniture: t.furniture }
    } else {
        let t = g.k.breaks.share.big;
        BreaksShare { thrown: t.thrown, heap: t.heap, furniture: t.furniture }
    };
    if look_on(g) { BreaksShare { thrown: s.thrown + s.heap, heap: 0.0, furniture: s.furniture } } else { s }
}

/// Pieces moving + particle records waiting to be born. (breakTable.ts Breaks.load)
pub fn breaks_load(g: &mut crate::game::Game) -> f64 {
    let waiting = crate::particle_fx::gpu_fx(g).map_or(0.0, |p| p.queue.len() as f64);
    crate::debris::debris_moving(g) + waiting
}

// ---------------------------------------------------------------- the pool's hooks
/// The level and puff of a thrown piece's landing. (breakTable.ts Breaks.lands)
pub fn breaks_lands(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, speed: f64) -> crate::break_table::BreaksLands {
    if kind == crate::debris::ChunkKind::Shard { return BreaksLands { level: 0.0, dust: 0.0 }; }
    let mat = mat_of_box(g, kind, box_);
    let row = seam_break_row(mat);
    let hard = mat == Mat::Tile && row.puff > 0.0 && match row.again { Some(a) => speed >= a.land, None => false };
    BreaksLands { level: row.level, dust: if hard { row.puff } else { 0.0 } }
}

/// Every chunk the pool takes: its row by what it is. (breakTable.ts Breaks.born)
pub fn breaks_born(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    let (kind, b) = (g.tab.debris_chunk[c_2].kind, g.tab.debris_chunk[c_2].box_);
    if kind == crate::debris::ChunkKind::Shard { return; }
    let mat = mat_of_box(g, kind, b);
    let paint = g.col.boxes[b].albedo;
    breaks_tag(g, c_2, mat, 0.0, paint, Some(kind == crate::debris::ChunkKind::Binder));
}

/// (breakTable.ts Breaks.tag)
pub fn breaks_tag(g: &mut crate::game::Game, c_2: DebrisChunkId, mat: crate::break_table::Mat, gen_: f64, paint: V3, fixed: Option<bool>) {
    let row = seam_break_row(mat);
    let c = &mut g.tab.debris_chunk[c_2];
    c.mat = Some(BreakMeta { mat, gen_, paint, fixed: if fixed.unwrap_or(false) { Some(true) } else { None } });
    c.contact = Some(crate::debris::ChunkContact { keep_floor: 0.55, keep_wall: 0.8, rest: row.rest });
    if row.level > 0.0 { c.level = row.level; }
}

/// A piece's first hard touch: over its row's speed it breaks once more, queued. (breakTable.ts Breaks.landed)
pub fn breaks_landed(g: &mut crate::game::Game, c_2: DebrisChunkId, speed: f64) {
    let Some(me) = g.tab.debris_chunk[c_2].mat else { return };
    if me.gen_ > 0.0 || me.fixed.unwrap_or(false) { return; }
    let a = seam_break_row(me.mat).again;
    let b = g.tab.debris_chunk[c_2].box_;
    let china = me.mat == Mat::Plastic && speed >= g.k.breaks.china_land && max3(g.col.boxes[b].h) <= g.k.breaks.china_max;
    if !china {
        let Some(a) = a else { return };
        if speed < a.land { return; }
        if let Some(chance) = a.chance {
            if crate::debris::hash01(breaks_rel(g, c_2) * 7919.0 + 13.0) >= chance { return; }
        }
    }
    if !g.destr.breaks.queue.contains(&c_2) { g.destr.breaks.queue.push(c_2); }
}

/// An impulse's kick on a piece: over its row's kick it breaks once more. (breakTable.ts Breaks.kicked)
pub fn breaks_kicked(g: &mut crate::game::Game, c_2: DebrisChunkId, kick: f64) {
    let Some(me) = g.tab.debris_chunk[c_2].mat else { return };
    if me.gen_ > 0.0 || me.fixed.unwrap_or(false) { return; }
    let Some(a) = seam_break_row(me.mat).again else { return };
    if kick < a.kick { return; }
    if !g.destr.breaks.queue.contains(&c_2) { g.destr.breaks.queue.push(c_2); }
}

/// The pool took a resting piece's slot. (breakTable.ts Breaks.evicted)
pub fn breaks_evicted(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    crate::floormess::mess_evicted(g, c_2);
    if g.tab.debris_chunk[c_2].glass.is_some() {
        crate::shards::glass_shards_evicted(g, c_2);
        return;
    }
    breaks_dissolve(g, c_2);
}

/// The slot a spawn over the cap takes: the smallest resting piece, the oldest among equals. (breakTable.ts Breaks.pick)
pub fn breaks_pick(g: &mut crate::game::Game, list: &[DebrisChunkId], ok: &mut dyn FnMut(&mut crate::game::Game, DebrisChunkId) -> bool) -> f64 {
    let (mut bi, mut br) = (-1.0, m::INF);
    for i in 0..list.len() {
        let c = list[i];
        if !ok(g, c) || g.tab.debris_chunk[c].age <= 0.0 { continue; }
        let b = g.tab.debris_chunk[c].box_;
        let r = max3(g.col.boxes[b].h);
        if r < br { br = r; bi = i as f64; }
    }
    if bi >= 0.0 { return bi; }
    for i in 0..list.len() {
        if ok(g, list[i]) { return i as f64; }
    }
    -1.0
}

/// An evicted piece gone where it lay, by its size. (breakTable.ts Breaks.dissolve)
pub fn breaks_dissolve(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    let has_fx = crate::particle_fx::gpu_fx(g).is_some();
    g.stats.break_table.crumbled += 1;
    if !has_fx { return; }
    let me = meta_of(g, c_2);
    let row = seam_break_row(me.mat);
    let b = g.tab.debris_chunk[c_2].box_;
    let (bc, h, who) = (g.col.boxes[b].c, g.col.boxes[b].h, g.tab.debris_chunk[c_2].who);
    let il = if h[0] >= h[1] && h[0] >= h[2] { 0 } else if h[1] >= h[2] { 1 } else { 2 };
    let r = h[il];
    let others: Vec<f64> = [0usize, 1, 2].iter().filter(|i| **i != il).map(|i| h[*i]).collect();
    let mid = m::max_n(&others);
    let ([n0, n1], [s0, s1]) = (g.k.breaks.sized_chips, g.k.breaks.sized_span);
    let n = m::round(n0 + (n1 - n0) * m::max(0.0, m::min(1.0, (r - s0) / m::max(1e-3, s1 - s0))));
    let rot = match g.col.boxes[b].rot { Some(q) => q, None => v3::quat::yaw(g.col.boxes[b].yaw) };
    let ax = v3::quat::rotate(rot, if il == 0 { [1.0, 0.0, 0.0] } else if il == 1 { [0.0, 1.0, 0.0] } else { [0.0, 0.0, 1.0] });
    let chip_tint = g.k.breaks.chip_tint;
    let color = breaks_tint(g, &row, me.paint, chip_tint);
    let size_k = m::max(1.0, m::min(2.2, r / 0.08));
    let st = m::max(1.0, m::min(3.0, m::round(r / 0.14)));
    let jit = m::max(0.02, m::min(0.14, mid));
    let mut left = n;
    let mut i = 0.0;
    while i < st {
        let k = m::round(left / (st - i));
        left -= k;
        if k > 0.0 {
            let t = if st == 1.0 { 0.0 } else { (i / (st - 1.0) - 0.5) * 1.3 };
            let u = if st == 1.0 { 0.5 } else { i / (st - 1.0) };
            let at = [bc[0] + ax[0] * r * t, bc[1] + ax[1] * r * t, bc[2] + ax[2] * r * t];
            seam_emit_debris(g, at, [0.0, 1.0, 0.0], k, 0.35 * (1.15 - 0.3 * u), 0.7, 1.1, 17.0, 0.3, lerp(row.small.size, 0.25 + 0.6 * u) * size_k, 0.3, lerp(row.small.aspect, 0.25 + 0.6 * u), jit, color, who);
        }
        i += 1.0;
    }
    g.stats.break_table.chips += n as i32;
    if row.dust > 0.0 {
        let count = m::max(1.0, m::round(row.dust * m::max(0.5, m::min(1.5, r / 0.25))));
        let pfx = (g.m.particle_fx.pfx.dust_speed, g.m.particle_fx.pfx.dust_life, g.m.particle_fx.pfx.dust_size);
        let color = v3::lerp3(breaks_tint(g, &row, me.paint, 0.6), [0.8, 0.79, 0.76], 0.4);
        seam_emit_dust(g, bc, [0.0, 1.0, 0.0], count, pfx.0 * 0.6, 0.5, 0.05, pfx.1, 0.3, pfx.2, 0.35, m::max(0.08, m::min(0.3, r * 0.6)), None, color);
    }
}

/// After the pool's step: the breaks queued this frame. (breakTable.ts Breaks.tick)
pub fn breaks_tick(g: &mut crate::game::Game) {
    if g.destr.breaks.queue.is_empty() { return; }
    let q = core::mem::take(&mut g.destr.breaks.queue);
    for c in q { breaks_break_again(g, c); }
}

// ---------------------------------------------------------------- small and fine
/// (breakTable.ts Breaks.tint)
pub fn breaks_tint(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, paint: V3, k: f64) -> V3 {
    v3::lerp3(paint, core_of(row, paint), k)
}

/// `n` chips of the row's small debris at `at`. (breakTable.ts Breaks.chips)
pub fn breaks_chips(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, paint: V3, at: V3, dir: V3, n: f64, speed: f64, pos_jit: f64, who: f64, size_k: Option<f64>) {
    let size_k = size_k.unwrap_or(1.0);
    if crate::particle_fx::gpu_fx(g).is_none() || n <= 0.0 { return; }
    let s = row.small;
    let chip_tint = g.k.breaks.chip_tint;
    let color = breaks_tint(g, row, paint, chip_tint);
    let big = if n >= 4.0 && (s.aspect[1] > s.aspect[0] || s.size[1] > s.size[0]) { m::floor(n / 3.0) } else { 0.0 };
    let one = |g: &mut Game, k: f64, t: f64| {
        if k > 0.0 {
            seam_emit_debris(g, at, dir, k, speed * (1.15 - 0.3 * t), 0.7, 1.1, 17.0, 0.3, lerp(s.size, t) * size_k, 0.3, lerp(s.aspect, t), pos_jit, color, who);
        }
    };
    if js::truthy(big) { one(g, n - big, 0.25); one(g, big, 0.85); } else { one(g, n, 0.5); }
    g.stats.break_table.chips += n as i32;
}

/// The row's dust and sparks at `at`. (breakTable.ts Breaks.fines)
pub fn breaks_fines(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, paint: V3, at: V3, dir: V3, k: Option<f64>) {
    let k = k.unwrap_or(1.0);
    if crate::particle_fx::gpu_fx(g).is_none() { return; }
    let (dust_speed, dust_life, dust_size) = (g.m.particle_fx.pfx.dust_speed, g.m.particle_fx.pfx.dust_life, g.m.particle_fx.pfx.dust_size);
    if row.dust > 0.0 {
        let color = v3::lerp3(breaks_tint(g, row, paint, 0.6), [0.8, 0.79, 0.76], 0.4);
        seam_emit_dust(g, at, dir, m::round(row.dust * k), dust_speed, 0.5, 0.05, dust_life, 0.3, dust_size, 0.35, 0.08, Some(v3::scale(dir, 0.45)), color);
    }
    if row.sparks > 0.0 {
        let (spread, life, size) = (g.m.particle_fx.pfx.spark_spread, g.m.particle_fx.pfx.spark_life, g.m.particle_fx.pfx.spark_size);
        seam_emit_spark(g, at, dir, m::round(row.sparks * k), 4.0, 0.6, spread, life, 0.4, size);
    }
}

/// A piece gone into particles of its own material where it is. (breakTable.ts Breaks.crumble)
pub fn breaks_crumble(g: &mut crate::game::Game, c_2: DebrisChunkId, count: [f64; 2], speed: f64) {
    let me = meta_of(g, c_2);
    let row = seam_break_row(me.mat);
    let n = count[0] + m::floor(crate::debris::hash01(breaks_rel(g, c_2) * 577.0 + 3.0) * (count[1] - count[0] + 1.0));
    let b = g.tab.debris_chunk[c_2].box_;
    let r = max3(g.col.boxes[b].h);
    let (at, who) = (g.col.boxes[b].c, g.tab.debris_chunk[c_2].who);
    breaks_chips(g, &row, me.paint, at, [0.0, 1.0, 0.0], n, speed, r * 0.6, who, Some(m::max(1.0, m::min(2.2, r / 0.08))));
    g.stats.break_table.crumbled += 1;
}

/// A piece the pool refused: a burst of chips of its material. (breakTable.ts Breaks.refuse)
pub fn breaks_refuse(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, paint: V3, at: V3, v: V3, who: f64) {
    let dir = v3::normalize(v);
    let [r0, r1] = g.k.breaks.refuse_chips;
    let n = r0 + m::floor(breaks_phash(g) * (r1 - r0 + 1.0));
    breaks_chips(g, row, paint, at, dir, n, 2.2, 0.05, who, Some(1.3));
    g.stats.break_table.refused += 1;
}

// ---------------------------------------------------------------- level B
/// One strong event broke a piece once more: two halves, or its particles. (breakTable.ts Breaks.breakAgain)
pub fn breaks_break_again(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    use crate::scene_cut::{cut_fit, cut_record, cut_through, cut_to_world, CutPlane};
    let Some(i) = g.destr.debris.list.iter().position(|x| *x == c_2) else { return };
    let Some(me) = g.tab.debris_chunk[c_2].mat else { return };
    if me.gen_ > 0.0 { return; }
    let row = seam_break_row(me.mat);
    let again = row.again;
    let b = g.tab.debris_chunk[c_2].box_;
    let h = g.col.boxes[b].h;
    let gone = again.is_none() || (me.mat == Mat::Plastic && max3(h) <= g.k.breaks.china_max);
    if gone || crate::debris::debris_moving(g) > crate::debris::debris_cap(g) / 2.0 {
        g.destr.debris.list.remove(i);
        let count = match again { Some(a) => [a.chips + 4.0, a.chips + 6.0], None => [8.0, 10.0] };
        breaks_crumble(g, c_2, count, 1.2);
        if gone { g.stats.break_table.china += 1; } else { g.stats.break_table.mayhem += 1; }
        return;
    }
    let a = again.unwrap();
    // one plane through the piece: across its length, along it, or on its diagonal
    let thin = if h[0] <= h[1] && h[0] <= h[2] { 0 } else if h[1] <= h[2] { 1 } else { 2 };
    let (ua, va) = ((thin + 1) % 3, (thin + 2) % 3);
    let lg = if h[ua] >= h[va] { ua } else { va };
    let sh = if lg == ua { va } else { ua };
    let rel = breaks_rel(g, c_2);
    let r = |j: f64| crate::debris::hash01(rel * 1013.0 + j * 37.0 + 5.0);
    let (mut n, mut at): (V3, V3) = ([0.0; 3], [0.0; 3]);
    match a.how {
        Split::Across => {
            let t = (8.0 + 20.0 * r(1.0)) * RAD * (if r(2.0) < 0.5 { -1.0 } else { 1.0 });
            n[lg] = m::cos(t); n[sh] = m::sin(t); at[lg] = (r(3.0) - 0.5) * 0.4 * h[lg];
        }
        Split::Along => {
            let t = 7.0 * RAD * (r(1.0) * 2.0 - 1.0);
            n[sh] = m::cos(t); n[lg] = m::sin(t); at[sh] = (r(3.0) - 0.5) * 0.3 * h[sh];
        }
        Split::Diag => {
            let t = 12.0 * RAD * (r(1.0) * 2.0 - 1.0);
            let d = m::atan2(h[lg], h[sh]) * (if r(2.0) < 0.5 { -1.0 } else { 1.0 }) + t;
            n[lg] = m::cos(d); n[sh] = m::sin(d);
        }
    }
    let base: Vec<CutPlane> = g.col.boxes[b].cut.clone().unwrap_or_default();
    let halved = |plane: CutPlane| { let mut p = base.clone(); p.push(plane); cut_record(h, &p) };
    let halves = [halved(cut_through(at, n)), halved(cut_through(at, [-n[0], -n[1], -n[2]]))];
    if halves[0].empty || halves[1].empty { return; }
    g.destr.debris.list.remove(i);
    let (yaw, b_rot) = (g.col.boxes[b].yaw, g.col.boxes[b].rot);
    let rot = match b_rot { Some(q) => q, None => v3::quat::yaw(yaw) };
    let nw = v3::quat::rotate(rot, n);
    let (bc, albedo, rough, metal) = (g.col.boxes[b].c, g.col.boxes[b].albedo, g.col.boxes[b].rough, g.col.boxes[b].metal);
    let (cv, cw, who, age, level) = { let c = &g.tab.debris_chunk[c_2]; (c.v, c.w, c.who, c.age, c.level) };
    for k in 0..2usize {
        let kf = k as f64;
        let f = cut_fit(h, halves[k].cut.as_deref());
        let w = cut_to_world(f.off, yaw, b_rot);
        let sgn = if k == 0 { -1.0 } else { 1.0 };
        let word = word_of_box(g, b);
        let box_ = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
            c: [bc[0] + w[0], bc[1] + w[1], bc[2] + w[2]], h: f.h, yaw: Some(yaw), rot: Some(rot), albedo: Some(albedo), stuff: word.stuff, does: word.does, ..Default::default()
        });
        let box_c = [bc[0] + w[0], bc[1] + w[1], bc[2] + w[2]];
        let sep = 0.35 + 0.5 * r(10.0 + kf);
        let v = [cv[0] + nw[0] * sgn * sep, m::max(cv[1], 0.0) + 0.3 + 0.5 * r(12.0 + kf), cv[2] + nw[2] * sgn * sep];
        let wv = [cw[0] + (r(14.0 + kf) - 0.5) * 5.0, cw[1] + (r(16.0 + kf) - 0.5) * 3.0, cw[2] + (r(18.0 + kf) - 0.5) * 5.0];
        let Some(got) = crate::debris::debris_spawn_raw(g, crate::debris::ChunkKind::Chunk, box_, v, wv, Some(who), None) else {
            breaks_refuse(g, &row, me.paint, box_c, v, who);
            continue;
        };
        let got_box = g.tab.debris_chunk[got].box_;
        let planes = cut_len(&f.cut);
        let has_cut = f.cut.is_some();
        g.col.boxes[got_box].cut = f.cut;
        if has_cut { g.col.boxes[got_box].cut_core = face_core(&row, me.paint); }
        g.col.boxes[got_box].rough = rough;
        g.col.boxes[got_box].metal = metal;
        breaks_tag(g, got, me.mat, 1.0, me.paint, None);
        let sfx = g.tab.debris_chunk[c_2].sfx.clone();
        let t = &mut g.tab.debris_chunk[got];
        t.sounded = true; t.age = m::max(age, 1e-3); t.sfx = sfx; t.level = level;
        g.stats.break_table.planes += planes;
    }
    breaks_chips(g, &row, me.paint, bc, [0.0, 1.0, 0.0], a.chips, 1.6, m::max(h[lg], 0.04) * 0.5, who, None);
    breaks_fines(g, &row, me.paint, bc, [0.0, 1.0, 0.0], Some(0.5));
    let dressing = g.tab.debris_chunk[c_2].dress.unwrap_or(false);
    if row.puff > 0.0 && me.mat == Mat::Tile && !dressing {
        crate::destruction::destruction_cell_dust(g, bc, [0.0, 1.0, 0.0], row.puff, crate::scene_materials::Surf::Drywall);
    }
    if row.mark > 0.0 && bc[1] - crate::debris::support_along(g, b, [0.0, 1.0, 0.0]) < 0.06 {
        let opts = if dressing { crate::render_decals::DecalOpts { look: Some(true), ..Default::default() } } else { Default::default() };
        let pos = [bc[0], bc[1] - crate::debris::support_along(g, b, [0.0, 1.0, 0.0]) + 0.002, bc[2]];
        let d = crate::render_decals::decals_add(g, crate::render_decals::DecalKind::Chip, pos, [0.0, 1.0, 0.0], Some(row.mark * (0.7 + 0.6 * r(20.0))), Some(opts));
        if dressing { crate::look_decals::look_decal(g, d.as_ref()); }
    }
    g.stats.break_table.again += 1;
}

// ---------------------------------------------------------------- furniture: a plain piece takes its row's outline
/// A plain piece somebody else cut out of a thing takes its row's broken outline where it is. (breakTable.ts Breaks.dress)
pub fn breaks_dress(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    let (kind, b, mate, mat) = { let c = &g.tab.debris_chunk[c_2]; (c.kind, c.box_, c.mate, c.mat) };
    if kind != crate::debris::ChunkKind::Chunk || g.col.boxes[b].cut.is_some() || mate.is_some() { return; }
    let Some(me) = mat else { return };
    let row = seam_break_row(me.mat);
    let h = g.col.boxes[b].h;
    let [thin, mid, long] = axes_by_size(h);
    if h[long] < 0.04 { return; }
    if me.mat == Mat::Metal { breaks_kink(g, c_2, thin as f64, mid as f64, long as f64, None, None); return; }
    let Some(sb) = breaks_outline(g, me.mat, &row, h, thin as f64, mid as f64, long as f64, &mut |g: &mut Game| breaks_phash(g)) else {
        g.stats.break_table.plain += 1;
        return;
    };
    let Some(cut) = sb.cut else { g.stats.break_table.plain += 1; return };
    let rot = match g.col.boxes[b].rot { Some(q) => q, None => v3::quat::yaw(g.col.boxes[b].yaw) };
    let o = v3::quat::rotate(rot, sb.off);
    let bc = g.col.boxes[b].c;
    let planes = cut.len() as i32;
    g.col.boxes[b].c = [bc[0] + o[0], bc[1] + o[1], bc[2] + o[2]];
    g.col.boxes[b].h = sb.h;
    g.col.boxes[b].cut = Some(cut);
    g.col.boxes[b].cut_core = face_core(&row, me.paint);
    let core_tint = g.k.breaks.core_tint;
    let tinted = breaks_tint(g, &row, me.paint, core_tint);
    g.col.boxes[b].albedo = tinted;
    g.stats.break_table.dressed += 1;
    g.stats.break_table.planes += planes;
}

/// The outline row `m` gives a plain piece of these proportions; None = it stays whole. (breakTable.ts Breaks.outline)
pub fn breaks_outline(g: &mut crate::game::Game, m: crate::break_table::Mat, row: &crate::break_table::BreakRow, h: V3, thin: f64, mid: f64, long: f64, hash: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> Option<crate::break_table::Shaped> {
    let (t, d, l) = (thin as usize, mid as usize, long as usize);
    if row.cell.is_empty() || (m == Mat::Fabric && h[t] > 0.35 * h[d]) { return None; }
    let shape = if h[t] > 0.6 * h[d] && h[l] < 2.5 * h[d] { Shape::Lump }
        else if m == Mat::Wood && h[l] >= 2.5 * h[d] { if h[d] < 0.035 { Shape::Strip } else { Shape::Snapped } }
        else { Shape::Slab };
    shape_box(g, shape, thin, h[t], h[l], h[d], (t + 1) % 3 == l, row, hash, None)
}

/// A loose box somebody else owns takes the outline its material's row gives; keyed, none of the table's dice. (breakTable.ts Breaks.shapeLoose)
pub fn breaks_shape_loose(g: &mut crate::game::Game, b: BoxId, surf: &str, key: f64) -> bool {
    if g.col.boxes[b].cut.is_some() { return false; }
    let Some(mat) = surf_mat_of_word(surf) else { return false };
    if mat == Mat::Metal { return false; }
    let row = seam_break_row(mat);
    let h = g.col.boxes[b].h;
    let [a0, a1, a2] = axes_by_size(h);
    if h[a2] < g.k.breaks.loose_min { return false; }
    let mut n = 0.0;
    let base = m::imul(js::to_i32(key), 40503) as f64;
    let sb = breaks_outline(g, mat, &row, h, a0 as f64, a1 as f64, a2 as f64, &mut |_g: &mut Game| {
        let v = crate::debris::hash01(base + 977.0 * n + 11.0);
        n += 1.0;
        v
    });
    let Some(sb) = sb else { return false };
    let Some(cut) = sb.cut else { return false };
    let rot = match g.col.boxes[b].rot { Some(q) => q, None => v3::quat::yaw(g.col.boxes[b].yaw) };
    let o = v3::quat::rotate(rot, sb.off);
    let (mut low0, mut low1) = (0.0, m::INF);
    for k in 0..8u32 {
        let p = [if k & 1 != 0 { h[0] } else { -h[0] }, if k & 2 != 0 { h[1] } else { -h[1] }, if k & 4 != 0 { h[2] } else { -h[2] }];
        low0 = m::min(low0, v3::quat::rotate(rot, p)[1]);
    }
    for v in crate::scene_cut::cut_verts(sb.h, Some(&cut)) {
        low1 = m::min(low1, v3::quat::rotate(rot, v)[1] + o[1]);
    }
    let bc = g.col.boxes[b].c;
    let planes = cut.len() as i32;
    g.col.boxes[b].c = [bc[0] + o[0], bc[1] + o[1] + (if low1.is_finite() { low0 - low1 } else { 0.0 }), bc[2] + o[2]];
    g.col.boxes[b].h = sb.h;
    g.col.boxes[b].cut = Some(cut);
    let albedo = g.col.boxes[b].albedo;
    g.col.boxes[b].cut_core = face_core(&row, albedo);
    g.stats.break_table.loose += 1;
    g.stats.break_table.planes += planes;
    true
}

/// What a piece IS, from whoever knows better than its paint. (breakTable.ts Breaks.retag)
pub fn breaks_retag(g: &mut crate::game::Game, c_2: DebrisChunkId, mat: crate::break_table::Mat) {
    let Some(me) = g.tab.debris_chunk[c_2].mat else { return };
    if me.mat == mat { return; }
    breaks_tag(g, c_2, mat, me.gen_, me.paint, me.fixed);
}

/// A steel sheet torn off furniture carries a shallow fold, every one. (breakTable.ts Breaks.sheet)
pub fn breaks_sheet(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    let (kind, b, mate, has_mat) = { let c = &g.tab.debris_chunk[c_2]; (c.kind, c.box_, c.mate, c.mat.is_some()) };
    if kind != crate::debris::ChunkKind::Chunk || g.col.boxes[b].cut.is_some() || mate.is_some() || !has_mat { return; }
    breaks_retag(g, c_2, Mat::Metal);
    let h = g.col.boxes[b].h;
    let ax = axes_by_size(h);
    let deg = g.k.breaks.steel.kink;
    if breaks_kink(g, c_2, ax[0] as f64, ax[1] as f64, ax[2] as f64, Some(deg), Some(true)) { g.stats.break_table.sheets += 1; }
}

/// A steel carcass that goes over whole: dented along its edges, open on the face its door came off. (breakTable.ts Breaks.carcass)
pub fn breaks_carcass(g: &mut crate::game::Game, c_2: DebrisChunkId, axis: f64, sign: f64, seed: Option<f64>) {
    use crate::scene_cut::{cut_record, cut_through, CutPlane};
    let seed = seed.unwrap_or(0.0);
    let (kind, b, mate, has_mat) = { let c = &g.tab.debris_chunk[c_2]; (c.kind, c.box_, c.mate, c.mat.is_some()) };
    if kind != crate::debris::ChunkKind::Chunk || g.col.boxes[b].cut.is_some() || mate.is_some() || !has_mat { return; }
    breaks_retag(g, c_2, Mat::Metal);
    let h = g.col.boxes[b].h;
    let st = g.k.breaks.steel;
    let base = m::imul(js::to_i32(seed + 1.0), 2749) as f64;
    let r = |j: f64| crate::debris::hash01(base + j * 61.0 + 7.0);
    let (mut dents, mut deeper): (Vec<CutPlane>, Vec<CutPlane>) = (Vec::new(), Vec::new());
    let n = st.dents[0] + m::floor(r(0.0) * (st.dents[1] - st.dents[0] + 1.0));
    let mut k = 0.0;
    while k < n {
        let e = js::rem(m::floor(r(10.0 + k) * 3.0), 3.0) as usize;
        let (u, v) = ((e + 1) % 3, (e + 2) % 3);
        let su = if r(20.0 + k) < 0.5 { -1.0 } else { 1.0 };
        let sv = if r(30.0 + k) < 0.5 { -1.0 } else { 1.0 };
        let se = if r(40.0 + k) < 0.5 { -1.0 } else { 1.0 };
        let deep = m::min(lerp(st.dent, r(50.0 + k)), st.dent_most * 2.0 * m::min(h[u], h[v]));
        let lean = 0.6 + r(60.0 + k);
        let mut n3: V3 = [0.0; 3];
        n3[u] = su; n3[v] = sv * lean; n3[e] = se * deep * (0.7 + 0.5 * r(70.0 + k)) / m::max(h[e], 0.05);
        let mut at: V3 = [0.0; 3];
        at[u] = su * (h[u] - deep * 0.5); at[v] = sv * h[v]; at[e] = 0.0;
        let p = cut_through(at, n3);
        dents.push(p);
        deeper.push(CutPlane { n: p.n, d: p.d - 0.004 });
        k += 1.0;
    }
    let ax = axis as usize;
    let mut face: V3 = [0.0; 3];
    face[ax] = sign;
    let mut inner = vec![CutPlane { n: [-face[0], -face[1], -face[2]], d: h[ax] - st.rim }];
    for a in [0usize, 1, 2] {
        if a == ax { continue; }
        for sg in [-1.0, 1.0] {
            let mut n3: V3 = [0.0; 3];
            n3[a] = sg;
            inner.push(CutPlane { n: n3, d: h[a] - st.rim });
        }
    }
    let mut shell_planes = vec![CutPlane { n: face, d: h[ax] - st.lip }];
    shell_planes.extend_from_slice(&dents);
    inner.extend_from_slice(&deeper);
    let (shell, dark) = (cut_record(h, &shell_planes), cut_record(h, &inner));
    if shell.empty || dark.empty { return; }
    let (Some(shell_cut), Some(dark_cut)) = (shell.cut, dark.cut) else { return };
    let planes = shell_cut.len() as i32 + dark_cut.len() as i32;
    g.col.boxes[b].cut = Some(shell_cut);
    let p = crate::scene_boxes::MakeBoxP {
        c: g.col.boxes[b].c, h, yaw: Some(g.col.boxes[b].yaw), rot: g.col.boxes[b].rot, albedo: Some(v3::scale(g.col.boxes[b].albedo, st.inside)),
        flags: Some(g.col.boxes[b].flags), name: Some(String::from("debris")), mkey: Some(mate_mkey(g, c_2)), ..Default::default()
    };
    let mate = crate::scene_boxes::make_box(g, &p);
    g.tab.debris_chunk[c_2].mate = Some(mate);
    g.col.boxes[mate].cut = Some(dark_cut);
    g.col.boxes[mate].rough = Some(1.0);
    g.col.boxes[mate].metal = Some(false);
    g.stats.break_table.carcasses += 1;
    g.stats.break_table.planes += planes;
}

/// One half of the kinked plate: the side sg·s ≥ 0 of the fold. (breakTable.ts Breaks.kink's `half`)
fn kink_half(big_h: V3, f: V3, thin: usize, ta: f64, len: f64, tc: f64, tau: f64, u0: f64, sg: f64) -> Option<Vec<crate::scene_cut::CutPlane>> {
    use crate::scene_cut::CutPlane;
    let mut nt: V3 = [f[0] * sg * ta / len, f[1] * sg * ta / len, f[2] * sg * ta / len];
    nt[thin] = 1.0 / len;
    crate::scene_cut::cut_record(big_h, &[
        CutPlane { n: [-sg * f[0], -sg * f[1], -sg * f[2]], d: -sg * u0 },
        CutPlane { n: nt, d: (tc + tau + sg * ta * u0) / len },
        CutPlane { n: [-nt[0], -nt[1], -nt[2]], d: -(tc - tau + sg * ta * u0) / len },
    ]).cut
}

/// The kinked panel: the plate becomes two cut boxes in one thicker box. (breakTable.ts Breaks.kink)
pub fn breaks_kink(g: &mut crate::game::Game, c_2: DebrisChunkId, thin: f64, mid: f64, long: f64, deg: Option<[f64; 2]>, every: Option<bool>) -> bool {
    let k_ = g.k.breaks.kink;
    let deg = deg.unwrap_or(k_.deg);
    let every = every.unwrap_or(false);
    let (t, d, l) = (thin as usize, mid as usize, long as usize);
    let b = g.tab.debris_chunk[c_2].box_;
    let h = g.col.boxes[b].h;
    if h[t] > 0.25 * h[d] || h[l] < k_.min_long || (!every && m::abs(g.time - g.destr.breaks.kink_t) < k_.gap_secs) { return false; }
    if !every { g.destr.breaks.kink_t = g.time; }
    let alpha = lerp(deg, breaks_phash(g)) / 2.0 * RAD;
    let phi = (breaks_phash(g) * 2.0 - 1.0) * k_.skew * RAD;
    let u0 = (breaks_phash(g) - 0.5) * k_.off * h[l];
    let mut f: V3 = [0.0; 3];
    f[l] = m::cos(phi); f[d] = m::sin(phi);
    let mut s_max = 0.0;
    for sl in [-1.0, 1.0] {
        for sm in [-1.0, 1.0] {
            s_max = m::max(s_max, m::abs(f[l] * sl * h[l] + f[d] * sm * h[d] - u0));
        }
    }
    let ta = m::tan(alpha);
    let tau = h[t] / m::cos(alpha);
    let tc = s_max * ta / 2.0;
    let len = m::hyp2(1.0, ta);
    let mut big_h: V3 = h;
    big_h[t] = tc + tau;
    let c0 = kink_half(big_h, f, t, ta, len, tc, tau, u0, 1.0);
    let c1 = kink_half(big_h, f, t, ta, len, tc, tau, u0, -1.0);
    let (Some(c0), Some(c1)) = (c0, c1) else { return false };
    let planes = c0.len() as i32 + c1.len() as i32;
    g.col.boxes[b].h = big_h;
    g.col.boxes[b].cut = Some(c0);
    let word = word_of_box(g, b);
    let p = crate::scene_boxes::MakeBoxP {
        c: g.col.boxes[b].c, h: big_h, yaw: Some(g.col.boxes[b].yaw), rot: g.col.boxes[b].rot, albedo: Some(g.col.boxes[b].albedo), stuff: word.stuff, does: word.does,
        flags: Some(g.col.boxes[b].flags), name: Some(String::from("debris")), mkey: Some(mate_mkey(g, c_2)), ..Default::default()
    };
    let mate = crate::scene_boxes::make_box(g, &p);
    g.tab.debris_chunk[c_2].mate = Some(mate);
    g.col.boxes[mate].cut = Some(c1);
    let (rough, metal) = (g.col.boxes[b].rough, g.col.boxes[b].metal);
    g.col.boxes[mate].rough = rough;
    g.col.boxes[mate].metal = metal;
    g.stats.break_table.kinked += 1;
    g.stats.break_table.planes += planes;
    true
}

// ---------------------------------------------------------------- level A: a broken cell of a wall / of cover
/// What a broken cell throws: its row's shapes as cut boxes while the budget lasts, its chips, its fines. (breakTable.ts Breaks.shedCell)
pub fn breaks_shed_cell(g: &mut crate::game::Game, surf: crate::scene_materials::Surf, cell: BoxId, dir: Option<V3>, who: f64, budget: &mut crate::break_table::ShedBudget, as_: Option<crate::break_table::Mat>) -> crate::break_table::BreaksShedCell {
    let mat = match as_ { Some(a) => a, None => mat_of_surf(surf) };
    let row = seam_break_row(mat);
    let mut res = BreaksShedCell { thrown: 0.0, refused: 0.0 };
    let (cell_c, cell_h, cell_yaw, paint, cell_rough, cell_metal) = (g.col.boxes[cell].c, g.col.boxes[cell].h, g.col.boxes[cell].yaw, g.col.boxes[cell].albedo, g.col.boxes[cell].rough, g.col.boxes[cell].metal);
    // a wall's cell is thin across the wall; a crate's is a block
    let flat: i32 = if cell_h[0] < cell_h[2] * 0.6 { 0 } else if cell_h[2] < cell_h[0] * 0.6 { 2 } else { -1 };
    let away: Option<V3> = match budget.from { Some(from) => Some([cell_c[0] - from[0], 0.0, cell_c[2] - from[2]]), None => dir };
    let out: V3 = match away {
        None => [0.0, 0.0, 0.0],
        Some(a) => if flat >= 0 { wall_normal(g, cell, a, Some(0.3)) } else { v3::normalize([a[0], 0.0, a[2]]) },
    };
    let cells = row.cell.len() as f64;
    let n = m::min_n(&[m::max(0.0, budget.chunks), cells, budget.per.unwrap_or(cells)]);
    let k0 = if n < cells && cells > 1.0 && breaks_hash(g) < 0.5 { 1 } else { 0 };
    let core_tint = g.k.breaks.core_tint;
    let tinted = breaks_tint(g, &row, paint, core_tint);
    // two wedges of one cell are the two halves of one panel: (seed, thin, vert, L, S), the second replays the first's draws
    let mut twin: Option<(f64, usize, bool, f64, f64)> = None;
    for k in 0..(m::ceil(n) as usize) {
        let shape = row.cell[(k + k0) % row.cell.len()];
        let second = shape == Shape::Wedge && twin.is_some();
        let kept = if second { twin } else { None };
        let thin: usize = match kept { Some(w) => w.1, None => if flat >= 0 { flat as usize } else if breaks_hash(g) < 0.5 { 0 } else { 2 } };
        let along: usize = if thin == 0 { 2 } else { 0 };
        let vert = match kept { Some(w) => w.2, None => breaks_hash(g) < 0.5 || (flat >= 0 && (shape == Shape::Snapped || shape == Shape::Strip)) };
        let room = if vert { cell_h[1] } else { cell_h[along] };
        let cross = if vert { cell_h[along] } else { cell_h[1] };
        let l = match kept { Some(w) => w.3, None => { let t = breaks_hash(g); m::min(lerp(row.span, t * t) / 2.0, m::max(0.05, room)) } };
        let s = match kept { Some(w) => w.4, None => m::min(l * lerp(row.slim, breaks_hash(g)) * (if shape == Shape::Strip { 0.45 } else { 1.0 }), m::max(0.02, cross)) };
        let half_t = m::max(0.004, m::min(row.thick, cell_h[thin]));
        let long_first = if thin == 0 { vert } else { !vert };
        if shape == Shape::Wedge && twin.is_none() { twin = Some((m::floor(breaks_hash(g) * 1073741823.0), thin, vert, l, s)); }
        let ws = match twin { Some(w) => w.0, None => 0.0 };
        let sb = if shape == Shape::Wedge {
            let mut wi = 0.0;
            shape_box(g, shape, thin as f64, half_t, l, s, long_first, &row, &mut |_g: &mut Game| {
                let v = crate::debris::hash01(ws + wi * 7919.0);
                wi += 1.0;
                v
            }, Some(second))
        } else {
            shape_box(g, shape, thin as f64, half_t, l, s, long_first, &row, &mut |g: &mut Game| breaks_hash(g), Some(second))
        };
        if second { twin = None; }
        let cx = cell_c[0] + (breaks_hash(g) - 0.5) * cell_h[0];
        let cy = cell_c[1] + (breaks_hash(g) - 0.3) * cell_h[1];
        let cz = cell_c[2] + (breaks_hash(g) - 0.5) * cell_h[2];
        let c = [cx, cy, cz];
        let sp = 1.2 + 2.2 * breaks_hash(g);
        let v0 = out[0] * sp + (breaks_hash(g) - 0.5) * 0.8;
        let v1 = 0.6 + 1.4 * breaks_hash(g);
        let v2 = out[2] * sp + (breaks_hash(g) - 0.5) * 0.8;
        let v = [v0, v1, v2];
        let w0 = (breaks_hash(g) - 0.5) * 9.0;
        let w1 = (breaks_hash(g) - 0.5) * 5.0;
        let w2 = (breaks_hash(g) - 0.5) * 9.0;
        let h: V3 = match &sb {
            Some(q) => q.h,
            None => { let mut hh = [s, s, s]; hh[thin] = half_t; hh[if vert { 1 } else { along }] = l; hh }
        };
        let box_ = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP { c, h, yaw: Some(cell_yaw), albedo: Some(tinted), ..Default::default() });
        let got = crate::debris::debris_spawn(g, crate::debris::ChunkKind::Chunk, box_, v, [w0, w1, w2], Some(who), Some(row.level));
        budget.chunks -= 1.0;
        let Some(got) = got else {
            breaks_refuse(g, &row, paint, c, v, who);
            res.refused += 1.0;
            continue;
        };
        let got_box = g.tab.debris_chunk[got].box_;
        let cut = sb.and_then(|q| q.cut);
        let planes = cut_len(&cut);
        let has_cut = cut.is_some();
        g.col.boxes[got_box].cut = cut;
        if has_cut { g.col.boxes[got_box].cut_core = face_core(&row, paint); }
        g.col.boxes[got_box].rough = cell_rough;
        g.col.boxes[got_box].metal = cell_metal;
        breaks_tag(g, got, mat, 0.0, paint, None);
        res.thrown += 1.0;
        if has_cut { g.stats.break_table.shaped += 1; g.stats.break_table.planes += planes; } else { g.stats.break_table.plain += 1; }
    }
    let chip_n = m::round(row.small.n * (if breaks_small(g) { g.k.breaks.small_pool_chip_k } else { 1.0 }));
    let up: V3 = [0.0, 1.0, 0.0];
    let chip_dir = if dir.is_some() { v3::normalize(v3::add(out, [0.0, 0.5, 0.0])) } else { up };
    breaks_chips(g, &row, paint, cell_c, chip_dir, chip_n, 2.4, m::max(cell_h[0], cell_h[2]) * 0.6, who, None);
    let fine_dir = if dir.is_some() { out } else { up };
    breaks_fines(g, &row, paint, cell_c, fine_dir, None);
    if row.puff > 0.0 { crate::destruction::destruction_cell_dust(g, cell_c, fine_dir, row.puff, surf); }
    res
}

/// A blast that holed a wall leaves slabs of it dealt at rest at the wall's foot on the far side. (breakTable.ts Breaks.heap)
pub fn breaks_heap(g: &mut crate::game::Game, surf: crate::scene_materials::Surf, wall: BoxId, at: V3, from: V3, half_w: f64, who: Option<f64>, budget: Option<&mut crate::break_table::ShedBudget>, as_: Option<crate::break_table::Mat>) -> f64 {
    let who = who.unwrap_or(crate::consts::NOBODY_ID);
    let mut budget = budget;
    let mat = match as_ { Some(a) => a, None => mat_of_surf(surf) };
    let row = seam_break_row(mat);
    if row.cell.is_empty() { return 0.0; }
    if look_on(g) { return 0.0; }
    let paint = g.col.boxes[wall].albedo;
    let o = wall_normal(g, wall, [at[0] - from[0], 0.0, at[2] - from[2]], None);
    let lat: V3 = [-o[2], 0.0, o[0]];
    let k_ = g.k.breaks.heap;
    let mut dealt = 0.0;
    let wall_h = g.col.boxes[wall].h;
    let face = m::min(wall_h[0], wall_h[2]);
    let core_tint = g.k.breaks.core_tint;
    let tinted = breaks_tint(g, &row, paint, core_tint);
    let (wall_rough, wall_metal) = (g.col.boxes[wall].rough, g.col.boxes[wall].metal);
    let n = m::min(breaks_share(g).heap, budget.as_deref().and_then(|b| b.heap).unwrap_or(99.0));
    for _k in 0..(m::ceil(n) as usize) {
        let shape = if mat == Mat::Wood { Shape::Snapped } else { Shape::Slab };
        let l = lerp(k_.span, breaks_hash(g)) / 2.0;
        let s = l * lerp(row.slim, breaks_hash(g));
        let long_first = breaks_hash(g) < 0.5;
        let Some(sb) = shape_box(g, shape, 1.0, m::max(0.004, row.thick), l, s, long_first, &row, &mut |g: &mut Game| breaks_hash(g), None) else { continue };
        let d = face + lerp(k_.out, breaks_hash(g));
        let sd = (breaks_hash(g) - 0.5) * 2.0 * m::max(0.15, half_w) * 0.8;
        let x = at[0] + o[0] * d + lat[0] * sd;
        let z = at[2] + o[2] * d + lat[2] * sd;
        let yaw = breaks_hash(g) * m::PI * 2.0;
        let tip_a = breaks_hash(g) * m::PI * 2.0;
        let tip = lerp(k_.tip, breaks_hash(g));
        let rot = v3::quat::normalize(v3::quat::mul(v3::quat::axis_angle([m::cos(tip_a), 0.0, m::sin(tip_a)], tip), v3::quat::yaw(yaw)));
        let box_ = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP { c: [x, 0.0, z], h: sb.h, yaw: Some(yaw), rot: Some(rot), albedo: Some(tinted), ..Default::default() });
        // the floor (or the low thing) under it; a spot the hole cannot see along the floor is skipped
        let ro: V3 = [x, 0.5, z];
        let st = crate::scene_collision::raycast(g, ro, [0.0, -1.0, 0.0], 0.6);
        let ph = crate::props::prop_system_raycast(g, ro, [0.0, -1.0, 0.0], match &st { Some(q) => q.t, None => 0.6 });
        let t = match (&ph, &st) { (Some(p), _) => p.t, (None, Some(q)) => q.t, (None, None) => 0.5 };
        if t < 0.03 || crate::scene_collision::solid_blocked(g, [at[0], 0.25, at[2]], [x, 0.25, z], None) { continue; }
        if crate::props::prop_system_raycast(g, [x, 1.7, z], [0.0, -1.0, 0.0], 1.15).is_some() { continue; }
        let mut low = 0.0;
        for v in crate::scene_cut::cut_verts(sb.h, sb.cut.as_deref()) { low = m::min(low, v3::quat::rotate(rot, v)[1]); }
        let box_c: V3 = [x, ro[1] - t - low + 0.0015, z];
        g.col.boxes[box_].c = box_c;
        if let Some(b) = budget.as_deref_mut() {
            if let Some(left) = b.heap { b.heap = Some(left - 1.0); }
        }
        let Some(got) = crate::debris::debris_spawn_raw(g, crate::debris::ChunkKind::Chunk, box_, [0.0; 3], [0.0; 3], Some(who), None) else {
            breaks_refuse(g, &row, paint, box_c, [0.0, 0.5, 0.0], who);
            continue;
        };
        let got_box = g.tab.debris_chunk[got].box_;
        let planes = cut_len(&sb.cut);
        let has_cut = sb.cut.is_some();
        g.col.boxes[got_box].cut = sb.cut;
        if has_cut { g.col.boxes[got_box].cut_core = face_core(&row, paint); }
        g.col.boxes[got_box].rough = wall_rough;
        g.col.boxes[got_box].metal = wall_metal;
        breaks_tag(g, got, mat, 0.0, paint, None);
        let c = &mut g.tab.debris_chunk[got];
        c.rest = true;
        c.sounded = true;
        if let Some(p) = &ph { c.on = Some(p.prop); c.on_pose = m::NAN; }
        dealt += 1.0;
        g.stats.break_table.heap += 1;
        if has_cut { g.stats.break_table.shaped += 1; g.stats.break_table.planes += planes; } else { g.stats.break_table.plain += 1; }
    }
    dealt
}

// ---------------------------------------------------------------- what still stands
/// The colour a standing piece's ragged end shows. (breakTable.ts Breaks.endCore)
pub fn breaks_end_core(g: &mut crate::game::Game, surf: &str, paint: V3) -> Option<V3> {
    let mat = surf_mat_of_word(surf)?;
    face_core(&seam_break_row(mat), paint)
}

/// The cut record of a standing piece from the faces that look into opened cells. (breakTable.ts Breaks.ragged)
pub fn breaks_ragged(g: &mut crate::game::Game, b: &crate::break_table::CellGrid, rect: &[f64], h: V3) -> Option<Vec<crate::scene_cut::CutPlane>> {
    let row = seam_break_row(mat_of_surf(b.surf));
    if row.ragged.is_none() { return None; }
    let (x0, x1, y0, y1, z0, z1) = (rect[0], rect[1], rect[2], rect[3], rect[4], rect[5]);
    let (nx, ny, nz) = (b.nx, b.ny, b.nz);
    let open = |ix: f64, iy: f64, iz: f64| b.open.get(((iy * nz + iz) * nx + ix) as usize).copied() == Some(1);
    let mut faces: Vec<OpenFace> = Vec::new();
    let wall_like = ny > 1.0;
    let box_h = g.col.boxes[b.box_].h;
    let thin = if box_h[0] <= box_h[2] { 0.0 } else { 2.0 };
    if wall_like && look_on(g) { return None; }
    for axis in [0.0, 2.0] {
        for sign in [-1.0, 1.0] {
            let n_ = if axis == 0.0 { nx } else { nz };
            let i = if sign < 0.0 { (if axis == 0.0 { x0 } else { z0 }) - 1.0 } else { (if axis == 0.0 { x1 } else { z1 }) + 1.0 };
            if i < 0.0 || i >= n_ { continue; }
            let (mut all, mut upper, mut any) = (true, true, false);
            let (j0, j1) = if axis == 0.0 { (z0, z1) } else { (x0, x1) };
            let mut iy = y0;
            while iy <= y1 {
                let mut j = j0;
                while j <= j1 {
                    let o = if axis == 0.0 { open(i, iy, j) } else { open(j, iy, i) };
                    if o { any = true; } else { all = false; if iy > 0.0 { upper = false; } }
                    j += 1.0;
                }
                iy += 1.0;
            }
            if !any { continue; }
            let other = if axis == 0.0 { 2.0 } else { 0.0 };
            let seed = b.id * 7349.0 + axis * 131.0 + (if sign > 0.0 { 17.0 } else { 0.0 }) + i * 977.0;
            if all {
                faces.push(OpenFace { axis, sign, lat: vec![1.0, other], seed, lo: None, head: None });
            } else if wall_like && upper && y0 == 0.0 && y1 > 0.0 {
                let lo = b.yb[1] - (b.yb[y0 as usize] + b.yb[y1 as usize + 1]) / 2.0;
                faces.push(OpenFace { axis, sign, lat: vec![1.0], seed, lo: Some(lo), head: None });
            }
        }
    }
    if y1 < ny - 1.0 {
        let mut all = true;
        let mut iz = z0;
        while iz <= z1 && all {
            let mut ix = x0;
            while ix <= x1 {
                if !open(ix, y1 + 1.0, iz) { all = false; break; }
                ix += 1.0;
            }
            iz += 1.0;
        }
        if all {
            let lat = if thin == 0.0 { vec![2.0, 0.0] } else { vec![0.0, 2.0] };
            faces.push(OpenFace { axis: 1.0, sign: 1.0, lat, seed: b.id * 7349.0 + 5003.0 + (if thin == 0.0 { z0 } else { x0 }) * 977.0, lo: None, head: None });
        }
    }
    let cut = ragged_cut(g, &row, h, &faces);
    if cut.is_some() { g.stats.break_table.ragged += 1; }
    cut
}

/// The studs of a holed plasterboard partition, read off the cells alone. (breakTable.ts Breaks.studs)
pub fn breaks_studs(g: &mut crate::game::Game, b: &crate::break_table::CellGrid) -> Vec<crate::break_table::BreaksStudsItem> {
    if b.ny < 2.0 || mat_of_surf(b.surf) != Mat::Plasterboard || b.n_open == Some(0.0) { return Vec::new(); }
    let k_ = g.k.breaks.stud;
    let box_h = g.col.boxes[b.box_].h;
    let along_x = b.nx > 1.0 || (b.nz == 1.0 && box_h[0] >= box_h[2]);
    let n_ = if along_x { b.nx } else { b.nz };
    let mut out: Vec<BreaksStudsItem> = Vec::new();
    let open = |k: f64, iy: f64| b.open.get((if along_x { (iy * b.nz) * b.nx + k } else { (iy * b.nz + k) * b.nx }) as usize).copied() == Some(1);
    let l = if along_x { box_h[0] } else { box_h[2] };
    let t = if along_x { box_h[2] } else { box_h[0] };
    let (y0, y1) = (b.yb[1], b.yb[b.ny as usize]);
    let mut k = 0.0;
    while k + 1.0 < n_ {
        if open(k, 1.0) && open(k + 1.0, 1.0) && !open(k, 0.0) && !open(k + 1.0, 0.0) {
            let u = -l + 2.0 * l * (k + 1.0) / n_;
            let yl = y0 - k_.foot;
            let c: V3 = if along_x { [u, (yl + y1) / 2.0, 0.0] } else { [0.0, (yl + y1) / 2.0, u] };
            let h: V3 = if along_x { [k_.half, (y1 - yl) / 2.0, t * k_.depth_k] } else { [t * k_.depth_k, (y1 - yl) / 2.0, k_.half] };
            out.push(BreaksStudsItem { c, h, albedo: k_.albedo, rough: k_.rough, metal: k_.metal, does: k_.does });
        }
        k += 1.0;
    }
    out
}

/// The pool's line for the panel. (breakTable.ts Breaks.summary)
pub fn breaks_summary(g: &mut crate::game::Game) -> String {
    let load = breaks_load(g);
    let s = &g.stats.break_table;
    format!("breaks: shaped {} ({} planes, {} dropped) \u{b7} plain {} \u{b7} heap {} \u{b7} again {} \u{b7} crumbled {} \u{b7} refused {} \u{b7} mayhem {} \u{b7} ragged ends {} \u{b7} load {}", s.shaped, s.planes, s.dropped, s.plain, s.heap, s.again, s.crumbled, s.refused, s.mayhem, s.ragged, load)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn piece(g: &mut Game, side: f64, age: f64) -> DebrisChunkId {
        let mut b = crate::scene_boxes::Box::zeroed();
        b.h = [side, side / 2.0, side / 4.0];
        let box_ = g.col.boxes.push(b);
        let mut c = crate::debris::Chunk::zeroed();
        c.box_ = box_;
        c.age = age;
        g.tab.debris_chunk.push(c)
    }

    /// `[0, 1, 2].sort((i, j) => h[i] - h[j])`: smallest first, ties keep their order
    #[test]
    fn axes_sort_stably_by_half_extent() {
        assert_eq!(axes_by_size([0.3, 0.1, 0.1]), [1, 2, 0]);
        assert_eq!(axes_by_size([0.1, 0.1, 0.1]), [0, 1, 2]);
        assert_eq!(axes_by_size([0.5, 0.2, 0.3]), [1, 2, 0]);
    }

    /// breakTable.ts:396: the smallest that may go, the oldest among equals; not one dealt this frame unless nothing else may go
    #[test]
    fn pick_takes_the_smallest_resting_piece() {
        let mut g = Game::blank();
        let (big, small_a, small_b, fresh) = (piece(&mut g, 0.5, 1.0), piece(&mut g, 0.1, 1.0), piece(&mut g, 0.1, 2.0), piece(&mut g, 0.01, 0.0));
        let list = [big, small_a, small_b, fresh];
        assert_eq!(breaks_pick(&mut g, &list, &mut |_g, _c| true), 1.0);
        assert_eq!(breaks_pick(&mut g, &list, &mut |_g, c| c != small_a), 2.0);
        assert_eq!(breaks_pick(&mut g, &[fresh], &mut |_g, _c| true), 0.0);
        assert_eq!(breaks_pick(&mut g, &list, &mut |_g, _c| false), -1.0);
    }
}
