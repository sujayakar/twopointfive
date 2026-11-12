//! WHAT A PANE OF GLASS IS TO PLAY: what a blow does to a pane, which pieces are held and which fall and when, what a blast takes, when the floor hears it and is littered.
//! The same hashes, the same maths (rust/maths) and the same compares in the same order on every machine. The caller is game_rules breakage/glassbreak.rs, and the picture
//! reads the rows (game_draw boxes/glass.rs).
//!
//! WHAT A CALL IS HANDED, AND THE ORDER. A rule's call takes a `Ctx`: the game's world, the time and what it may let go into. `frame_begin` opens a frame's budget, then
//! `step_row` serves the rows in their order, so a row served late may find the budget gone. A question takes the world, or its panes.
//!
//! WHAT IS STATE. `World.panes` (game_types panes.rs): a row a pane that has been touched, what each has queued to leave, the holes rounds have left (pen.rs), the bars
//! taken. A row's first eighty doubles and sixty-four words are what a line or a ray meeting the pane reads (glass.rs `open`), where they stand.
//!
//! WHAT IS NOT HERE. What a fragment becomes (plates, spray, chips) is dressing: the rule lists each fragment it lets go (`Ctx.out`), and the caller hands them on when
//! the pane's call is back, a pane at a time. Nothing a fragment's handling does is read by the rest of that pane's rule. Its one question of the world, where it
//! comes down, is asked here at the moment it lets go, and its landing is listed for the caller (`Ctx.landings`), who keeps it (falls.rs).
//!
//! BOUNDS. A level of more sheets, or of a sheet of more clumps (pattern.rs MAX_CLUMPS), than a game holds is refused as it loads, by name.
use game_types::glassbreak::{Fragment, FragmentCause, FragmentFall, FragmentKind};
use game_types::glasspen::PaneType;
use game_types::knobs::Knobs;
use game_types::v3;
use alloc::vec::Vec;
use crate::falls::{self, FallSound, FallTold};
use crate::glass::{classify_row, dice_out, site_of, GW_COLS, GW_CRATER, GW_DSEED, GW_FLAGS, GW_GOING, GW_N, GW_OPEN, GW_ROWS, GW_SEED, G_DICE, G_DICE_AT, G_GONE, G_RUN, G_SITE};
use crate::lines::{self, ASK_GLASS};
use crate::maths::{hypot2, hypot3, round_half_up};
use crate::pattern::{self, clip_rect, frags_of, hull, order_of, poly_area, Dice, Frag, Pt, FRAGS_PER_SITE, MAX_PTS, REMAINDER};
use crate::world::{js_abs, js_ceil, js_max, js_min, sign_or_one, Over, World, REC, R_C, R_COS, R_H, R_SIN};
use game_types::panes::{Charge, Going, BK, MAX_SITES};
pub use game_types::panes::{Hole, Panes, Row};
use prim::{floor, sqrt};

/// the crossings one pane remembers (GLASSPEN.holes: its knob is under this)
pub const HOLE_PTS: usize = 24;
const MAX_SECTORS: usize = 12;
/// the glass kinds, in their order everywhere (the record the renderer reads says the same): plain float, toughened, laminated
pub const ANNEALED: u32 = 0;
pub const TEMPERED: u32 = 1;
pub const LAMINATED: u32 = 2;
/// what made a piece go
pub const C_HIT: u32 = 0;
pub const C_SUPPORT: u32 = 1;
pub const C_BLAST: u32 = 2;
pub const C_DICE: u32 = 3;
pub const C_RAKE: u32 = 4;
/// what a fragment is
pub const K_PIECE: f64 = 0.0;
pub const K_DAGGER: f64 = 1.0;
pub const K_CLUMP: f64 = 2.0;
pub const K_SHEET: f64 = 3.0;
const BLAST_MIN_K: f64 = 0.05;
const DICE_TIE: f64 = 1e-4;
const EXT_SKY_TOP: f64 = 12.0;
const THICK_GLASS_HALF: f64 = 0.05;
const RHO_TOP: f64 = 0.78 + 0.44;
const PI: f64 = core::f64::consts::PI;
const SKY_UNKNOWN: i32 = 2;
/// a sheet that has no word of its own on what glass it is (`set_type`)
const NO_OVER: u32 = 3;
const D_U: usize = G_DICE_AT;
const D_V: usize = G_DICE_AT + 1;
const D_HU: usize = G_DICE_AT + 2;
const D_HV: usize = G_DICE_AT + 3;
const D_LAST: usize = G_DICE_AT + 4;
const D_LAST_K: usize = G_DICE_AT + 5;
/// (the eighty doubles' last two) when a pane's dice began
const D_T0: usize = G_DICE_AT + 6;
const S_U: usize = 0;
const S_V: usize = 1;
const S_ROT: usize = 2;
const S_RHO: usize = 3;
const S_RZ: usize = 7;
const S_MERGE: usize = 8;

const EVENTS: usize = 32;
const CHARGES: usize = 16;

/// a piece onto a pane's queue, in its order (by when, then by which)
fn q_put(p: &mut Panes, r: usize, e: Going) {
    let run = &mut p.going[r];
    let at = run.iter().rposition(|q| q.at < e.at || (q.at == e.at && q.fid < e.fid)).map_or(0, |i| i + 1);
    run.insert(at, e);
    let row = &mut p.rows[r];
    row.w[GW_GOING + (e.fid >> 5) as usize] |= 1 << (e.fid & 31);
    if e.fid as usize % FRAGS_PER_SITE < MAX_SECTORS {
        row.w[GW_CRATER] |= 1 << (e.fid as usize / FRAGS_PER_SITE);
    }
}
/// the first piece off a pane's queue
fn q_pop(p: &mut Panes, r: usize) -> Going {
    let e = p.going[r].remove(0);
    let row = &mut p.rows[r];
    row.w[GW_GOING + (e.fid >> 5) as usize] &= !(1 << (e.fid & 31));
    let wedge = |fid: u32| (fid as usize % FRAGS_PER_SITE < MAX_SECTORS).then_some(fid as usize / FRAGS_PER_SITE);
    if let Some(site) = wedge(e.fid) {
        if !p.going[r].iter().any(|q| wedge(q.fid) == Some(site)) {
            row.w[GW_CRATER] &= !(1 << site);
        }
    }
    e
}

// ---- the game's hashes
/// the language's `x | 0` of a double
#[inline]
pub fn int32(x: f64) -> i32 {
    if x != x || x == f64::INFINITY || x == f64::NEG_INFINITY {
        return 0;
    }
    let t = prim::trunc(x);
    let m = t - 4294967296.0 * floor(t / 4294967296.0);
    (m as u32) as i32
}
#[inline]
fn seed_mix(h: u32, x: f64) -> u32 {
    let h2 = (h ^ (int32(round_half_up(x * 1000.0)) as u32)).wrapping_mul(0x85EB_CA77);
    h2 ^ (h2 >> 13)
}
/// What a box is as one word, its centre and half sizes to the millimetre
pub fn box_seed(b: &[f64]) -> u32 {
    let mut h = 0x9E37_79B1u32;
    for k in 0..3 {
        h = seed_mix(h, b[R_C + k]);
    }
    for k in 0..3 {
        h = seed_mix(h, b[R_H + k]);
    }
    h.wrapping_mul(0xC2B2_AE3D)
}
pub fn point_seed(word: u32, x: f64, y: f64, z: f64) -> u32 {
    let mut h = word;
    h = seed_mix(h, x);
    h = seed_mix(h, y);
    h = seed_mix(h, z);
    h.wrapping_mul(0xC2B2_AE3D)
}
/// A blast's seed, its height left out
#[inline]
fn blast_seed(word: u32, at: &[f64; 3]) -> u32 {
    point_seed(word, at[0], 0.0, at[2])
}
/// a whole number, however large a double holds it, hashed to 0 ‥ 1
#[inline]
pub fn hash01(x: f64) -> f64 {
    let mut h = (int32(x) as u32 ^ 0x9E37_79B9).wrapping_mul(0x85EB_CA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^= h >> 16;
    (h >> 8) as f64 / 16777216.0
}

// ---- a pane's frame
#[inline]
fn to_uv(r: &Row, x: f64, y: f64, z: f64) -> (f64, f64) {
    let (px, pz) = (x - r.c[0], z - r.c[2]);
    let (lx, lz) = (r.cs * px - r.sn * pz, r.sn * px + r.cs * pz);
    (if r.u_axis == 0 { lx } else { lz }, y - r.c[1])
}
#[inline]
fn to_uv_in(r: &Row, x: f64, y: f64, z: f64) -> (f64, f64) {
    let (u, v) = to_uv(r, x, y, z);
    (js_max(-r.hu * 0.995, js_min(r.hu * 0.995, u)), js_max(-r.hv * 0.995, js_min(r.hv * 0.995, v)))
}
#[inline]
fn to_world(r: &Row, u: f64, v: f64) -> [f64; 3] {
    let (lx, lz) = if r.u_axis == 0 { (u, 0.0) } else { (0.0, u) };
    [r.c[0] + r.cs * lx + r.sn * lz, r.c[1] + v, r.c[2] - r.sn * lx + r.cs * lz]
}
#[inline]
fn normal_of(r: &Row) -> [f64; 3] {
    if r.u_axis == 0 {
        [r.sn, 0.0, r.cs]
    } else {
        [r.cs, 0.0, -r.sn]
    }
}
/// The nearest point of a box (yawed, no more) to p
#[inline]
fn nearest_on(b: &[f64], p: [f64; 3]) -> [f64; 3] { nearest_within(b, p, 1.0) }
/// …on the box drawn in about its middle to `of` of its size
#[inline]
fn nearest_within(b: &[f64], p: [f64; 3], of: f64) -> [f64; 3] {
    let (cs, sn) = (b[R_COS], b[R_SIN]);
    let (px, pz) = (p[0] - b[R_C], p[2] - b[R_C + 2]);
    let (h0, h1, h2) = (of * b[R_H], of * b[R_H + 1], of * b[R_H + 2]);
    let lx = js_max(-h0, js_min(h0, cs * px - sn * pz));
    let lz = js_max(-h2, js_min(h2, sn * px + cs * pz));
    let ly = js_max(-h1, js_min(h1, p[1] - b[R_C + 1]));
    [b[R_C] + cs * lx + sn * lz, b[R_C + 1] + ly, b[R_C + 2] - sn * lx + cs * lz]
}
#[inline]
fn rec(o: &World, bi: usize) -> &[f64] {
    &o.rec[bi * REC..bi * REC + REC]
}
/// a sheet of glass: thin across its thin axis
#[inline]
fn thin(b: &[f64]) -> bool {
    js_min(b[R_H], b[R_H + 2]) <= THICK_GLASS_HALF
}
#[inline]
fn bit(w: &[u32], at: usize, k: u32) -> bool {
    w[at + (k >> 5) as usize] >> (k & 31) & 1 != 0
}
#[inline]
fn site_n(r: &Row) -> usize {
    (r.w[GW_FLAGS] >> 8) as usize
}
#[inline]
fn run(r: &Row) -> bool {
    r.w[GW_FLAGS] & G_RUN != 0
}
#[inline]
fn gone(r: &Row) -> bool {
    r.w[GW_FLAGS] & G_GONE != 0
}
#[inline]
fn dicing(r: &Row) -> bool {
    r.w[GW_FLAGS] & G_DICE != 0
}
#[inline]
fn dice_of(r: &Row) -> Dice {
    Dice { u: r.d[D_U], v: r.d[D_V], cols: r.w[GW_COLS], rows: r.w[GW_ROWS], seed: r.w[GW_DSEED], hu: r.d[D_HU], hv: r.d[D_HV] }
}
#[inline]
fn open_uv(r: &Row, u: f64, v: f64) -> bool {
    if gone(r) {
        return true;
    }
    if dicing(r) {
        return dice_out(&r.d[G_DICE_AT..], &r.w, u, v);
    }
    if r.n_open == 0 {
        return false;
    }
    let fid = classify_row(&r.d, &r.w, u, v);
    fid >= 0 && bit(&r.w, GW_OPEN, fid as u32)
}
/// whose territory (u, v) is in (the least power distance); −1 = no sites
#[inline]
fn nearest_site(r: &Row, u: f64, v: f64) -> i32 {
    let (mut best, mut bd) = (-1, f64::INFINITY);
    for i in 0..site_n(r) {
        let s = &r.d[i * G_SITE..i * G_SITE + G_SITE];
        let (du, dv) = (s[S_U] - u, s[S_V] - v);
        let dd = du * du + dv * dv - s[S_RZ] * s[S_RZ];
        if dd < bd {
            bd = dd;
            best = i as i32;
        }
    }
    best
}
#[inline]
fn tooth(f: &Frag, tune: &Knobs) -> bool {
    f.area < tune.glassbreak.tooth_area && f.frame_sum() > 1e-4
}

// ---- one call's business: whose world (its panes are written, and nothing else of it), what time it is, and what it let go
pub struct Ctx<'a> {
    pub w: &'a mut World,
    pub now: f64,
    /// a fragment's landing is the simulation's (shards: its switch, and glass breaking live)
    pub shards: bool,
    pub gravity: f64,
    /// what was let go, in the order it went
    pub out: Vec<LetGo>,
    /// what comes down, and when, in the order it was let go
    pub landings: Vec<falls::Landing>,
    /// what a walker goes through moved (a pane he now goes through): the caller says so to the box world
    pub wrote: bool,
}
/// a fragment, its outline still in its pane's own plane
pub struct LetGo {
    pub f: Fragment,
    pub pts: Vec<Pt>,
}
/// one line as the rule asks it of the statics and the leaves: is something solid across it
#[inline]
fn solid(w: &World, a: [f64; 3], b: [f64; 3], tune: &Knobs) -> bool {
    lines::solid_line(w, true, ASK_GLASS, a[0], a[1], a[2], b[0], b[1], b[2], tune).unwrap_or(true)
}

// ---- which glass a pane is
/// Which face of the pane has the open sky (+1 the side its normal points to, −1 the other, 0 neither or both). Asked of the level's own
/// geometry once, and kept: statics do not move.
pub fn sky_side(w: &World, s: usize, tune: &Knobs) -> i32 {
    let (bi, known) = {
        let h = &w.sheets[s];
        (h.bi as usize, h.sky)
    };
    if known != SKY_UNKNOWN {
        return known;
    }
    let (n, a, hu, b_c) = {
        let o = w;
        let b = rec(o, bi);
        let (cs, sn) = (b[R_COS], b[R_SIN]);
        let wide = b[R_H] >= b[R_H + 2];
        let n = if wide { [sn, 0.0, cs] } else { [cs, 0.0, -sn] };
        let a = if wide { [cs, 0.0, -sn] } else { [sn, 0.0, cs] };
        (n, a, js_max(b[R_H], b[R_H + 2]), [b[R_C], b[R_C + 1], b[R_C + 2]])
    };
    let mut sky = [0u32; 2];
    for k in 0..2 {
        for off in [0.5, 1.0, 1.6] {
            for t in [-0.5, 0.0, 0.5] {
                let d = (if k == 1 { 1.0 } else { -1.0 }) * off;
                let p = [b_c[0] + n[0] * d + a[0] * t * hu, b_c[1], b_c[2] + n[2] * d + a[2] * t * hu];
                if !solid(w, p, [p[0], EXT_SKY_TOP, p[2]], tune) {
                    sky[k] += 1;
                }
            }
        }
    }
    let e = if (sky[0] >= 5) == (sky[1] >= 5) {
        0
    } else if sky[1] >= 5 {
        1
    } else {
        -1
    };
    e
}
/// which of the level's sheets is static `bi` (they are listed in ascending order)
#[inline]
fn sheet_of(o: &World, bi: u32) -> Option<usize> {
    let (mut lo, mut hi) = (0usize, o.sheets.len());
    while lo < hi {
        let m = (lo + hi) / 2;
        let v = o.sheets[m].bi;
        if v == bi {
            return Some(m);
        }
        if v < bi {
            lo = m + 1;
        } else {
            hi = m;
        }
    }
    None
}
/// a word of the level's on this pane's glass, or none
fn over_of(p: &Panes, bi: u32) -> u32 {
    p.kinds.iter().find(|k| k[0] == bi).map_or(NO_OVER, |k| k[1])
}
/// The type a pane breaks as when nobody has said. Outside walls are safety glass, a low clerestory strip laminated; inside, a narrow
/// sidelight is laminated and a partition float; under GLASSREAL.place the pane's own place decides between toughened and float.
fn type_by_place(w: &World, s: usize, h0: f64, h1: f64, h2: f64, cy: f64, tune: &Knobs) -> u32 {
    place_rule(sky_side(w, s, tune) != 0, h0, h1, h2, cy, w.sheets[s].gap, tune)
}
/// the rule itself, on a pane's sizes, the wall it is in and how far it stands from a door
pub fn place_rule(outside: bool, h0: f64, h1: f64, h2: f64, cy: f64, gap: f64, tune: &Knobs) -> u32 {
    if if outside { h1 < 0.4 } else { js_max(h0, h2) < 0.35 } {
        return LAMINATED;
    }
    let foot = cy - h1;
    if foot < tune.glassreal.low_m || (foot < tune.glassreal.door_foot_m && gap <= tune.glassreal.door_m) {
        TEMPERED
    } else {
        ANNEALED
    }
}
/// The type of static `bi` as it will break, or NONE: it is no sheet of glass
pub const NO_TYPE: u32 = 9;
pub fn type_of_pane(w: &World, bi: u32, tune: &Knobs) -> u32 {
    let s = match sheet_of(w, bi) {
        Some(s) => s,
        None => return NO_TYPE,
    };
    let (h0, h1, h2, cy) = {
        let b = rec(w, bi as usize);
        if !thin(b) {
            return NO_TYPE;
        }
        (b[R_H], b[R_H + 1], b[R_H + 2], b[R_C + 1])
    };
    let over = over_of(&w.panes, bi);
    if over != NO_OVER {
        return over;
    }
    type_by_place(w, s, h0, h1, h2, cy, tune)
}
/// Pane `bi` breaks as this from here on, made whole first, at the back of the order (NO_OVER: back to its place's word)
pub fn set_type(p: &mut Panes, bi: u32, ty: u32) {
    match p.kinds.iter().position(|k| k[0] == bi) {
        Some(i) if ty == NO_OVER => {
            p.kinds.remove(i);
        }
        Some(i) => p.kinds[i][1] = ty,
        None if ty != NO_OVER => p.kinds.push([bi, ty]),
        None => {}
    }
    if let Some(r) = p.row_of(bi) {
        p.rows.remove(r);
        p.going.remove(r);
    }
    p.charges.clear();
}
/// Every pane whole (a word a level's table gave a pane stands, and the holes rounds have left)
pub fn reset(p: &mut Panes) {
    p.rows.clear();
    p.going.clear();
    p.charges.clear();
    p.gust = false;
    p.released = 0;
}

// ---- a pane's first touch
/// The pane's row, made if this is its first touch; None: it is no sheet of glass
fn state_of(w: &mut World, bi: u32, tune: &Knobs) -> Option<usize> {
    if let Some(r) = w.panes.row_of(bi) {
        return Some(r);
    }
    let ty = type_of_pane(w, bi, tune);
    if ty == NO_TYPE {
        return None;
    }
    let exterior = sky_side(w, sheet_of(w, bi)?, tune) != 0;
    let b = rec(w, bi as usize);
    let along_x = b[R_H] >= b[R_H + 2];
    let row = Row {
        key: box_seed(b),
        ty,
        u_axis: if along_x { 0 } else { 2 },
        hu: if along_x { b[R_H] } else { b[R_H + 2] },
        hv: b[R_H + 1],
        ht: js_min(b[R_H], b[R_H + 2]),
        exterior: exterior as u32,
        c: [b[R_C], b[R_C + 1], b[R_C + 2]],
        cs: b[R_COS],
        sn: b[R_SIN],
        sound_at: -1e9,
        last_dir: [0.0, 0.0, 1.0],
        ..Row::of(bi)
    };
    w.panes.rows.push(row);
    w.panes.going.push(Vec::new());
    Some(w.panes.rows.len() - 1)
}

// ---- sites
/// A new site at (u, v) for a blow of energy e, its ring zone fitted to the room the others leave; −1 = no room
fn add_site(r: &mut Row, u: f64, v: f64, e: f64, seed32: u32, tune: &Knobs) -> i32 {
    let ns = site_n(r);
    if ns as f64 >= js_min(MAX_SITES as f64, tune.glassbreak.max_sites) || run(r) {
        return -1;
    }
    let web = r.ty == LAMINATED;
    let base = if web { [tune.glassbreak.rho.get(game_types::knobs::GlassbreakKRhoKey::Laminated).unwrap()[0], tune.glassbreak.rho.get(game_types::knobs::GlassbreakKRhoKey::Laminated).unwrap()[1], tune.glassbreak.rho.get(game_types::knobs::GlassbreakKRhoKey::Laminated).unwrap()[2], tune.glassbreak.rho.get(game_types::knobs::GlassbreakKRhoKey::Laminated).unwrap()[3]] } else { [tune.glassbreak.rho.get(game_types::knobs::GlassbreakKRhoKey::Annealed).unwrap()[0], tune.glassbreak.rho.get(game_types::knobs::GlassbreakKRhoKey::Annealed).unwrap()[1], tune.glassbreak.rho.get(game_types::knobs::GlassbreakKRhoKey::Annealed).unwrap()[2], tune.glassbreak.rho.get(game_types::knobs::GlassbreakKRhoKey::Annealed).unwrap()[3]] };
    let kk = js_max(0.8, js_min(1.4, 0.75 + 0.25 * e));
    let mut rho = [base[0] * kk, base[1] * kk, base[2] * kk, base[3] * kk];
    let mut rz = rho[2] * RHO_TOP * 1.001;
    let mut room = f64::INFINITY;
    for q in 0..ns {
        let s = &r.d[q * G_SITE..q * G_SITE + G_SITE];
        room = js_min(room, hypot2(s[S_U] - u, s[S_V] - v) - s[S_RZ]);
    }
    if room < (if web { tune.glassbreak.rz_min * 0.4 } else { tune.glassbreak.rz_min }) {
        return -1;
    }
    if rz > room {
        let f = room / rz;
        rho[0] *= f;
        rho[1] *= f;
        rho[2] *= f;
        rz = room;
    }
    rho[3] = js_max(rho[3], rho[2] * 2.0);
    let seed = seed32 >> 8;
    let (r0, r1) = if web { (tune.glassbreak.radials.get(game_types::knobs::GlassbreakKRadialsKey::Laminated).unwrap()[0], tune.glassbreak.radials.get(game_types::knobs::GlassbreakKRadialsKey::Laminated).unwrap()[1]) } else { (tune.glassbreak.radials.get(game_types::knobs::GlassbreakKRadialsKey::Annealed).unwrap()[0], tune.glassbreak.radials.get(game_types::knobs::GlassbreakKRadialsKey::Annealed).unwrap()[1]) };
    let spread = js_max(1.0, r1) as u32;
    let n = js_min(MAX_SECTORS as f64, r0 + ((seed >> 12) % spread) as f64);
    let d = &mut r.d[ns * G_SITE..ns * G_SITE + G_SITE];
    d[S_U] = u;
    d[S_V] = v;
    d[S_ROT] = (seed & 0xfff) as f64 / 4096.0 * PI * 2.0;
    d[S_RHO] = rho[0];
    d[S_RHO + 1] = rho[1];
    d[S_RHO + 2] = rho[2];
    d[S_RHO + 3] = rho[3];
    d[S_RZ] = rz;
    d[S_MERGE] = if web { 0.0 } else { tune.glassbreak.merge };
    r.w[GW_N + ns] = n as u32;
    r.w[GW_SEED + ns] = seed;
    r.e[ns] = e;
    r.hits[ns] = 1.0;
    r.w[GW_FLAGS] = (r.w[GW_FLAGS] & 0xff) | ((ns as u32 + 1) << 8);
    ns as i32
}

/// fragments one call lists at most, and the points of their outlines (one past the points' room goes with no outline)
const MAX_OUT: usize = pattern::MAX_CLUMPS + 16;
const MAX_OUT_PTS: usize = MAX_OUT * 12;

#[derive(Clone, Copy)]
struct Fr {
    kind: f64,
    centre: [f64; 3],
    normal: [f64; 3],
    area: f64,
    velocity: [f64; 3],
    spin: [f64; 3],
    grain: f64,
    cause: u32,
    seed: u32,
    key: f64,
}
/// Where and when something let go at c with velocity v reaches what is under it (statics and leaves as the ray has them, the world's floor if nothing is)
pub fn fall_of(w: &World, c: [f64; 3], v: [f64; 3], gravity: f64, reach_k: f64, tune: &Knobs) -> (f64, [f64; 3]) {
    let vy = -v[1];
    let t_of = |h: f64| (-vy + sqrt(vy * vy + 2.0 * gravity * h)) / gravity;
    let mut reach = hypot2(v[0], v[2]) * t_of(js_max(0.02, c[1])) * reach_k;
    let d = if reach > 1e-4 { [v[0] * reach_k * t_of(js_max(0.02, c[1])) / reach, 0.0, v[2] * reach_k * t_of(js_max(0.02, c[1])) / reach] } else { [0.0, 0.0, 0.0] };
    let mut out = [0.0; 4];
    if reach > 0.05 {
        let m = lines::ray_rule(w, c[0], c[1], c[2], d[0], d[1], d[2], reach + 0.05, -1, true, &mut out, tune);
        if m >= 0 {
            reach = js_max(0.0, out[0] - 0.05);
        }
    }
    let (x, z) = (c[0] + d[0] * reach, c[2] + d[2] * reach);
    let m = lines::ray_rule(w, x, c[1], z, 0.0, -1.0, 0.0, c[1] + 0.5, -1, true, &mut out, tune);
    let h = js_max(0.02, if m >= 0 { out[0] } else { c[1] });
    (t_of(h), [x, c[1] - h, z])
}
/// a piece let go: the break heard, its landing, and the fragment listed for the host
fn emit(c: &mut Ctx, ri: usize, f: Fr, pts: &[Pt], rev: bool, tune: &Knobs) {
    let (bi, ty, who, thick, heard) = {
        let r = &mut c.w.panes.rows[ri];
        let mut heard = false;
        if f.cause != C_HIT && c.now - r.sound_at >= tune.glassbreak.crack_gap {
            r.sound_at = c.now;
            heard = true;
        }
        (r.w[0], r.ty, r.who, 2.0 * r.ht, heard)
    };
    let (mut fall, mut landed) = (None, false); // (no fall: not asked, for nothing will read it)
    if c.shards {
        let (t, pos) = fall_of(c.w, f.centre, f.velocity, c.gravity, 0.8, tune);
        // (a time that is none is no fall)
        fall = if t < 0.0 { None } else { Some(FragmentFall { t, pos }) };
        landing(c, &f, ty, who, t, pos, tune);
        landed = true;
    }
    if c.out.len() >= MAX_OUT {
        return;
    }
    let listed: usize = c.out.iter().map(|l| l.pts.len()).sum();
    let outline: Vec<Pt> = if listed + pts.len() > MAX_OUT_PTS { Vec::new() } else if rev { pts.iter().rev().copied().collect() } else { pts.to_vec() };
    let (kind, cause) = (
        if f.kind == K_PIECE { FragmentKind::Piece } else if f.kind == K_DAGGER { FragmentKind::Dagger } else if f.kind == K_CLUMP { FragmentKind::Clump } else { FragmentKind::Sheet },
        match f.cause {
            C_HIT => FragmentCause::Hit,
            C_SUPPORT => FragmentCause::Support,
            C_BLAST => FragmentCause::Blast,
            C_DICE => FragmentCause::Dice,
            _ => FragmentCause::Rake,
        },
    );
    let type_ = match ty {
        ANNEALED => PaneType::Annealed,
        TEMPERED => PaneType::Tempered,
        _ => PaneType::Laminated,
    };
    let f = Fragment {
        bi: bi as f64, type_, kind, cause, outline: Vec::new(), centre: f.centre, normal: f.normal, thickness: thick, area: f.area, velocity: f.velocity, spin: f.spin, tint: [0.0; 3],
        grain: f.grain, who, seed: f.seed as f64, key: f.key, heard, fall, landed,
    };
    c.out.push(LetGo { f, pts: outline });
}
/// a fragment's one landing — a sound for the ear if it has one of its own, the glass it leaves, the floor told
fn landing(c: &mut Ctx, f: &Fr, ty: u32, who: f64, t: f64, p: [f64; 3], tune: &Knobs) {
    let a = js_max(1e-4, f.area);
    let dice = f.kind == K_CLUMP || ty == TEMPERED;
    let powder = !dice && f.kind != K_SHEET && f.grain > 0.0;
    let plateable = !dice && !powder && a >= tune.shards.plate_min_area;
    let at = c.now + t;
    let soon = c.now + (t + 0.05);
    let mut put = |t: f64, sound: FallSound, gain: f64, rate: f64, amount: f64, told: FallTold| {
        c.landings.push(falls::Landing { t, pos: p, sound, gain, rate, amount, who, told, level: 0.0, dust: 0.0 });
    };
    if powder {
        put(soon, FallSound::GlassTinkle, 0.0, 1.0, a, FallTold::Never);
    } else if dice {
        put(at, FallSound::GlassShatter, js_min(1.0, 0.35 + a * 4.0), 1.15 + 0.3 * hash01(f.seed as f64), a * 0.8, FallTold::IfItSounds);
    } else if f.kind == K_SHEET {
        put(at, FallSound::GlassTinkle, if plateable { 0.0 } else { 0.5 }, 0.8, a * 0.3, FallTold::Always);
    } else {
        put(at, FallSound::GlassTinkle, if plateable { 0.0 } else { js_min(0.9, 0.3 + a * 8.0) }, 0.9 + 0.5 * hash01(f.seed as f64), a, FallTold::Always);
    }
}

/// The pane has a piece less
fn after_release(c: &mut Ctx, ri: usize, tune: &Knobs) {
    let (w, was, ext) = {
        let r = &mut c.w.panes.rows[ri];
        (walkable_now(r, tune), r.walkable, r.exterior)
    };
    if w as u32 != was {
        c.w.panes.rows[ri].walkable = w as u32;
        c.wrote = true;
    }
    if ext != 0 {
        c.w.panes.gust = true;
    }
}
fn walkable_now(r: &Row, tune: &Knobs) -> bool {
    if gone(r) {
        return true;
    }
    if dicing(r) || r.ty != ANNEALED || !run(r) {
        return false;
    }
    let v_clear = tune.glassbreak.walk_clear_h - r.c[1];
    let frags = frags_of(&r.d, &r.w, r.hu, r.hv);
    for f in frags.list() {
        if f.v_min < v_clear && !bit(&r.w, GW_OPEN, f.fid) && !tooth(f, tune) {
            return false;
        }
    }
    true
}
fn set_open(r: &mut Row, fid: u32) {
    r.w[GW_OPEN + (fid >> 5) as usize] |= 1 << (fid & 31);
    r.n_open += 1;
}
/// A site's crater wedges go together, as one fragment. → fragments let go
fn release_crater(c: &mut Ctx, ri: usize, i: usize, cause: u32, tune: &Knobs) -> u32 {
    let (hull_n, hp, frag) = {
        let r = &mut c.w.panes.rows[ri];
        let n = r.w[GW_N + i] as usize;
        let mut any = false;
        for s in 0..n {
            let fid = (i * FRAGS_PER_SITE + s) as u32;
            if !bit(&r.w, GW_OPEN, fid) {
                set_open(r, fid);
                any = true;
            }
        }
        if !any {
            return 0;
        }
        let site = site_of(&r.d, &r.w, i);
        let mut pts = [(0.0, 0.0); MAX_SECTORS];
        for s in 0..n {
            pts[s] = site.chord(s as i32, 0);
        }
        let mut hl = [(0.0, 0.0); 32];
        let m = hull(&mut pts[..n], &mut hl);
        let g = clip_rect(&hl[..m], r.hu, r.hv);
        if g.n < 3 {
            return 0;
        }
        let (area, cu, cv) = poly_area(&g.p[..g.n]);
        let nr = normal_of(r);
        let out_sign = if v3::dot(nr, r.last_dir) >= 0.0 { 1.0 } else { -1.0 };
        let nn = v3::scale(nr, out_sign);
        let seed = r.w[GW_SEED + i];
        let h = hash01(seed as f64);
        (
            g.n,
            g.p,
            (Fr { kind: K_PIECE, centre: to_world(r, cu, cv), normal: nn, area, velocity: v3::add(v3::scale(nn, 2.5 + 2.0 * h), [0.0, -0.2, 0.0]), spin: [0.0, 0.0, 0.0], grain: 0.004, cause, seed: seed ^ 0xC2A7E5, key: (i * FRAGS_PER_SITE) as f64 }, out_sign),
        )
    };
    let rev = outline_rev(&c.w.panes.rows[ri], frag.1);
    emit(c, ri, frag.0, &hp[..hull_n], rev, tune);
    after_release(c, ri, tune);
    1
}
/// (u, v, w) is right-handed for a pane along x and left-handed for one along z: is the (u, v) polygon to be walked backwards to run anticlockwise from the side it leaves by
#[inline]
fn outline_rev(r: &Row, out_sign: f64) -> bool {
    (out_sign > 0.0) != (r.u_axis == 0)
}
fn release_frag(c: &mut Ctx, ri: usize, fid: u32, cause: u32, from: Option<[f64; 3]>, kk: f64, tune: &Knobs) {
    let (f, fr, rev) = {
        let r = &mut c.w.panes.rows[ri];
        if bit(&r.w, GW_OPEN, fid) {
            return;
        }
        let f = match frags_of(&r.d, &r.w, r.hu, r.hv).by(fid) {
            Some(f) => *f,
            None => return,
        };
        set_open(r, fid);
        let n = normal_of(r);
        let cw = to_world(r, f.cu, f.cv);
        let mut dir_out = r.last_dir;
        if let Some(fr) = from {
            let d = v3::sub(cw, fr);
            if hypot3(d[0], d[1], d[2]) > 1e-6 {
                dir_out = v3::normalize(d);
            }
        }
        let out_sign = if v3::dot(n, dir_out) >= 0.0 { 1.0 } else { -1.0 };
        let nn = v3::scale(n, out_sign);
        let key = r.key as f64;
        let fd = fid as f64;
        let (h1, h2, h3) = (hash01(key * 7919.0 + fd * 131.0), hash01(key * 104729.0 + fd * 193.0), hash01(fd * 7.0 + key));
        let speed = if cause == C_BLAST {
            tune.glassbreak.blast_out * kk * (0.6 + 0.8 * h3)
        } else if cause == C_HIT {
            tune.glassbreak.out * (0.8 + 1.2 * h3)
        } else if cause == C_RAKE {
            tune.glassbreak.out * (1.2 + 0.8 * h3)
        } else {
            tune.glassbreak.out * (0.15 + 0.5 * h3)
        };
        let base = if (cause == C_BLAST || cause == C_RAKE) && from.is_some() { v3::scale(v3::normalize(v3::add(v3::scale(dir_out, 0.7), v3::scale(nn, 0.5))), speed) } else { v3::scale(nn, speed) };
        let velocity = v3::add(base, [0.0, -0.2 - 0.6 * h1, 0.0]);
        let spin = tune.glassbreak.spin;
        let seed = point_seed(r.key, cw[0], cw[1], cw[2]) ^ fid;
        (
            f,
            Fr { kind: if f.b >= 3 { K_DAGGER } else { K_PIECE }, centre: cw, normal: nn, area: f.area, velocity, spin: [(h1 - 0.5) * spin, (h2 - 0.5) * 2.0, (h3 - 0.5) * spin], grain: 0.0, cause, seed, key: fd },
            outline_rev(r, out_sign),
        )
    };
    emit(c, ri, fr, &f.p[..f.n as usize], rev, tune);
    after_release(c, ri, tune);
}
/// A hard blow on a piece: it goes now (a crater's wedge takes the whole crater with it). → fragments let go
fn knock_out(c: &mut Ctx, ri: usize, fid: u32, tune: &Knobs) -> u32 {
    let r = &mut c.w.panes.rows[ri];
    if bit(&r.w, GW_OPEN, fid) {
        return 0;
    }
    let (i, b) = (fid as usize / FRAGS_PER_SITE, (fid as usize % FRAGS_PER_SITE) / MAX_SECTORS);
    if b == 0 {
        return release_crater(c, ri, i, C_HIT, tune);
    }
    if frags_of(&r.d, &r.w, r.hu, r.hv).by(fid).is_none() {
        return 0;
    }
    release_frag(c, ri, fid, C_HIT, None, 1.0, tune);
    1
}
/// The radials reach the frame
fn set_run(c: &mut Ctx, ri: usize, tune: &Knobs) {
    let r = &mut c.w.panes.rows[ri];
    if run(r) || r.ty != ANNEALED || site_n(r) == 0 {
        return;
    }
    r.w[GW_FLAGS] |= G_RUN;
    r.next_think = c.now + tune.glassbreak.think;
}

// ---- toughened glass
fn start_dice(c: &mut Ctx, ri: usize, u: f64, v: f64, seed: u32, tune: &Knobs) {
    let d = {
        let r = &mut c.w.panes.rows[ri];
        if dicing(r) || gone(r) {
            return;
        }
        let cols = js_max(1.0, round_half_up(2.0 * r.hu / tune.glassbreak.clump_m)) as u32;
        let rows = js_max(1.0, round_half_up(2.0 * r.hv / tune.glassbreak.clump_m)) as u32;
        r.w[GW_COLS] = cols;
        r.w[GW_ROWS] = rows;
        r.w[GW_DSEED] = seed >> 8;
        r.d[D_U] = u;
        r.d[D_V] = v;
        r.d[D_HU] = r.hu;
        r.d[D_HV] = r.hv;
        r.d[D_LAST] = f64::NEG_INFINITY;
        r.d[D_LAST_K] = -1.0;
        r.d[D_T0] = c.now;
        r.w[GW_FLAGS] |= G_DICE;
        r.d_next = 0;
        dice_of(r)
    };
    let order = order_of(&d);
    let k0 = d.clump(u, v);
    let mut key0 = 0.0;
    for cl in order {
        if cl.k == k0 {
            key0 = cl.key;
            break;
        }
    }
    let mut next = 0;
    while next < order.len() && order[next].key <= key0 + DICE_TIE {
        release_clump(c, ri, &d, order[next], tune);
        next += 1;
        c.w.panes.rows[ri].d_next = next as u32;
    }
    if next >= order.len() {
        dice_done(c, ri);
    }
}
fn dice_done(c: &mut Ctx, ri: usize) {
    let r = &mut c.w.panes.rows[ri];
    r.w[GW_FLAGS] |= G_GONE;
    r.walkable = 1;
    c.wrote = true;
    if r.exterior != 0 {
        c.w.panes.gust = true;
    }
}
fn step_dice(c: &mut Ctx, ri: usize, tune: &Knobs) {
    let (d, t0) = {
        let r = &mut c.w.panes.rows[ri];
        (dice_of(r), r.d[D_T0])
    };
    if c.now - t0 >= tune.glassreal.hang {
        drop_all(c, ri, &d, tune);
    } else {
        let r = &mut c.w.panes.rows[ri];
        if r.walkable != 0 {
            r.walkable = 0;
            c.wrote = true;
        }
    }
}
fn release_clump(c: &mut Ctx, ri: usize, d: &Dice, cl: pattern::Clump, tune: &Knobs) {
    {
        let r = &mut c.w.panes.rows[ri];
        r.d[D_LAST] = cl.key;
        r.d[D_LAST_K] = cl.k as f64;
    }
    let mut pts = [(0.0, 0.0); MAX_PTS];
    let np = d.poly(cl.c as u32, cl.r as u32, &mut pts);
    if np < 3 {
        return;
    }
    let (area, cu, cv) = poly_area(&pts[..np]);
    if area < 1e-8 {
        return;
    }
    let (fr, rev) = {
        let r = &mut c.w.panes.rows[ri];
        let n = normal_of(r);
        let out_sign = if v3::dot(n, r.last_dir) >= 0.0 { 1.0 } else { -1.0 };
        let nn = v3::scale(n, out_sign);
        let (key, ck) = (r.key as f64, cl.k as f64);
        let (h1, h2, h3) = (hash01(key * 7919.0 + ck * 131.0), hash01(key * 104729.0 + ck * 193.0), hash01(ck * 7.0 + key));
        let rad = js_max(1e-3, hypot2(cu - d.u, cv - d.v));
        let (a, b) = (to_world(r, 0.0, 0.0), to_world(r, (cu - d.u) / rad, (cv - d.v) / rad));
        let spread = v3::sub(b, a);
        let velocity = v3::add(v3::add(v3::scale(nn, tune.glassbreak.out * (0.3 + 0.9 * h1)), v3::scale(spread, 0.5 * h2)), [0.0, -0.3 - 0.5 * h3, 0.0]);
        let spin = tune.glassbreak.spin;
        let centre = to_world(r, cu, cv);
        (
            Fr { kind: K_CLUMP, centre, normal: nn, area, velocity, spin: [(h1 - 0.5) * spin, (h2 - 0.5) * 2.0, (h3 - 0.5) * spin], grain: tune.glassbreak.dice_grain, cause: C_DICE, seed: d.seed ^ (cl.k + 1).wrapping_mul(0x9E37_79B1), key: ck },
            outline_rev(r, out_sign),
        )
    };
    emit(c, ri, fr, &pts[..np], rev, tune);
}
/// (GLASSREAL.drop) every clump still standing leaves on this step, handed on in square blocks of them
fn drop_all(c: &mut Ctx, ri: usize, d: &Dice, tune: &Knobs) {
    let order = order_of(d);
    const MAX_BLOCKS: usize = 1024;
    let blocks = js_min(MAX_BLOCKS as f64, js_max(1.0, tune.glassreal.blocks));
    let (cols, rows) = (d.cols as f64, d.rows as f64);
    let mut m = 1.0;
    while js_ceil(cols / m) * js_ceil(rows / m) > blocks {
        m += 1.0;
    }
    let (du, dv) = (2.0 * d.hu / cols, 2.0 * d.hv / rows);
    let (bc, br) = (js_ceil(cols / m) as usize, js_ceil(rows / m) as usize);
    let mut n = [0u32; MAX_BLOCKS];
    let mut su = [0.0; MAX_BLOCKS];
    let mut sv = [0.0; MAX_BLOCKS];
    let mi = m as usize;
    let mut next = c.w.panes.rows[ri].d_next as usize;
    while next < order.len() {
        let cl = order[next];
        let b = (cl.r as usize / mi) * bc + cl.c as usize / mi;
        let p = d.centre(cl.c as u32, cl.r as u32);
        n[b] += 1;
        su[b] += p.0;
        sv[b] += p.1;
        next += 1;
    }
    {
        let r = &mut c.w.panes.rows[ri];
        r.d_next = next as u32;
        r.d[D_LAST] = f64::INFINITY;
        r.d[D_LAST_K] = f64::INFINITY;
    }
    for b in 0..bc * br {
        if n[b] == 0 {
            continue;
        }
        let (c0, r0) = ((b % bc) * mi, (b / bc) * mi);
        let u0 = -d.hu + c0 as f64 * du;
        let u1 = js_min(d.hu, u0 + m * du);
        let v0 = -d.hv + r0 as f64 * dv;
        let v1 = js_min(d.hv, v0 + m * dv);
        let (cu, cv) = (su[b] / n[b] as f64, sv[b] / n[b] as f64);
        let (fr, rev) = {
            let r = &mut c.w.panes.rows[ri];
            let nr = normal_of(r);
            let out_sign = if v3::dot(nr, r.last_dir) >= 0.0 { 1.0 } else { -1.0 };
            let nn = v3::scale(nr, out_sign);
            let (key, bd) = (r.key as f64, b as f64);
            let (h1, h3) = (hash01(key * 7919.0 + bd * 131.0), hash01(bd * 7.0 + key));
            let near = js_max(0.0, 1.0 - hypot2(cu - d.u, cv - d.v) / 0.6);
            let spin = tune.glassbreak.spin;
            (
                Fr { kind: K_CLUMP, centre: to_world(r, cu, cv), normal: nn, area: n[b] as f64 * du * dv, velocity: v3::add(v3::scale(nn, tune.glassbreak.out * (0.1 + 0.25 * h1 + 0.8 * near)), [0.0, -0.3 - 0.5 * h3, 0.0]), spin: [(h1 - 0.5) * spin, 0.0, (h3 - 0.5) * spin], grain: tune.glassbreak.dice_grain, cause: C_DICE, seed: d.seed ^ (b as u32 + 1).wrapping_mul(0x9E37_79B1), key: bd },
                outline_rev(r, out_sign),
            )
        };
        emit(c, ri, fr, &[(u0, v0), (u1, v0), (u1, v1), (u0, v1)], rev, tune);
    }
    dice_done(c, ri);
}
/// The laminated sheet leaves its frame whole
fn release_sheet(c: &mut Ctx, ri: usize, away: [f64; 3], kk: f64, tune: &Knobs) {
    let (fr, pts, rev) = {
        let p = &mut c.w.panes;
        if gone(&p.rows[ri]) {
            return;
        }
        p.rows[ri].w[GW_FLAGS] |= G_GONE;
        p.rows[ri].walkable = 1;
        c.wrote = true;
        while !p.going[ri].is_empty() {
            q_pop(p, ri);
        }
        p.gust |= p.rows[ri].exterior != 0;
        let r = &p.rows[ri];
        let n = normal_of(r);
        let out_sign = if v3::dot(n, away) >= 0.0 { 1.0 } else { -1.0 };
        let nn = v3::scale(n, out_sign);
        let (hu, hv) = (r.hu, r.hv);
        let area = 4.0 * hu * hv;
        let seed = point_seed(r.key, r.c[0], r.c[1], r.c[2]);
        (
            Fr { kind: K_SHEET, centre: r.c, normal: nn, area, velocity: v3::add(v3::scale(nn, tune.glassbreak.blast_out * kk * 0.6), [0.0, 0.4, 0.0]), spin: [1.5 * kk, 0.0, 0.4], grain: 0.0, cause: C_BLAST, seed, key: 0.0 },
            [(-hu, -hv), (hu, -hv), (hu, hv), (-hu, hv)],
            outline_rev(r, out_sign),
        )
    };
    emit(c, ri, fr, &pts, rev, tune);
}

// ---- the queue
/// a piece's turn to leave (a pane's queue is kept in that order)
fn queue(c: &mut Ctx, ri: usize, fid: u32, at: f64, cause: u32, from: [f64; 3], kk: f64) {
    q_put(&mut c.w.panes, ri, Going { at, from, k: kk, fid, cause });
}

// ---- a blow
/// what impact() answers
#[derive(Clone, Copy, Debug, Default)]
pub struct Blow {
    pub live: bool,
    pub ty: u32,
    pub was_open: bool,
    pub punched: bool,
    pub diced: bool,
    pub released: u32,
    pub cracked: bool,
}
/// A blow of `energy` (1 = one pistol round square on) met pane `bi` at (x, y, z) travelling `dir`. `through`: the caller's word on the round
/// (1 went on · 0 stopped · −1 none: the glass decides by energy). The glass does what it does; what the round does next is the caller's.
pub fn impact(c: &mut Ctx, bi: u32, x: [f64; 3], dir: Option<[f64; 3]>, energy: f64, who: f64, through: i32, tune: &Knobs) -> Blow {
    let mut res = Blow { ty: LAMINATED, ..Blow::default() };
    let ty = type_of_pane(c.w, bi, tune);
    res.ty = if ty == NO_TYPE { LAMINATED } else { ty };
    let Some(ri) = state_of(c.w, bi, tune) else {
        return res;
    };
    res.live = true;
    let r = &mut c.w.panes.rows[ri];
    let (u, v) = to_uv_in(r, x[0], x[1], x[2]);
    if open_uv(r, u, v) {
        res.was_open = true;
        res.punched = true;
        return res;
    }
    r.hit_n += 1;
    r.energy += energy;
    r.damage = js_min(1.0, r.damage + 0.12 * energy);
    r.who = who;
    r.last_uv = [u, v];
    if let Some(d) = dir {
        if hypot3(d[0], d[1], d[2]) > 1e-6 {
            r.last_dir = v3::normalize(d);
        }
    }
    let blow = At { ri, u, v, seed: point_seed(r.key, x[0], x[1], x[2]), energy };
    match r.ty {
        TEMPERED => on_tempered(c, &blow, through, &mut res, tune),
        LAMINATED => on_laminated(c, &blow, &mut res, tune),
        _ => on_float(c, &blow, through, &mut res, tune),
    }
    res
}
/// where a blow lands on a pane, the seed of the spot, and how hard
struct At {
    ri: usize,
    u: f64,
    v: f64,
    seed: u32,
    energy: f64,
}
impl At {
    /// the blow counts on the site nearest to it. → whether there is one
    fn counts_on_nearest(&self, r: &mut Row) -> bool {
        let i = nearest_site(r, self.u, self.v);
        if i >= 0 {
            r.e[i as usize] += self.energy;
            r.hits[i as usize] += 1.0;
        }
        i >= 0
    }
}
/// Tempered glass: all of it goes to dice, once it has taken enough or a round has gone through.
fn on_tempered(c: &mut Ctx, b: &At, through: i32, res: &mut Blow, tune: &Knobs) {
    let r = &mut c.w.panes.rows[b.ri];
    if r.energy >= tune.glassbreak.break_e.tempered || through == 1 {
        let started = !dicing(r);
        start_dice(c, b.ri, b.u, b.v, b.seed, tune);
        res.diced = true;
        res.punched = open_uv(&c.w.panes.rows[b.ri], b.u, b.v);
        res.released = if started { 1 } else { 0 };
        res.cracked = true;
    }
}
/// Laminated glass: a new star on the uncut sheet, or the blow on the nearest.
fn on_laminated(c: &mut Ctx, b: &At, res: &mut Blow, tune: &Knobs) {
    let r = &mut c.w.panes.rows[b.ri];
    if classify_row(&r.d, &r.w, b.u, b.v) < 0 && add_site(r, b.u, b.v, b.energy, b.seed, tune) >= 0 {
        res.cracked = true;
        return;
    }
    b.counts_on_nearest(r);
}
/// Float glass: a star, a piece knocked out, or the whole of it running.
fn on_float(c: &mut Ctx, b: &At, through: i32, res: &mut Blow, tune: &Knobs) {
    let (ri, u, v, energy) = (b.ri, b.u, b.v, b.energy);
    let hard = if through >= 0 { through == 1 } else { energy >= tune.glassbreak.punch_e };
    let no_room = star_or_piece(c, b, hard, res, tune);
    let r = &mut c.w.panes.rows[ri];
    if !run(r) && (r.energy >= tune.glassbreak.break_e.annealed || energy >= tune.glassbreak.run_single_e || (through == 1 && no_room)) {
        runs_from(c, b, hard, res, tune);
    }
    let r = &mut c.w.panes.rows[ri];
    res.punched = open_uv(r, u, v);
    if res.released > 0 {
        res.cracked = true;
    }
    if r.next_think <= 0.0 {
        r.next_think = c.now + tune.glassbreak.think;
    }
}
/// On a piece that is still in, the blow counts on its site, and a hard one knocks the piece out; on the uncut sheet it is a new star if there is room
/// for one. → there was none
fn star_or_piece(c: &mut Ctx, b: &At, hard: bool, res: &mut Blow, tune: &Knobs) -> bool {
    let (ri, u, v) = (b.ri, b.u, b.v);
    let r = &mut c.w.panes.rows[ri];
    let fid = classify_row(&r.d, &r.w, u, v);
    if fid >= 0 {
        let i = fid as usize / FRAGS_PER_SITE;
        r.e[i] += b.energy;
        r.hits[i] += 1.0;
        if hard {
            res.released += knock_out(c, ri, fid as u32, tune);
        }
        return false;
    }
    let i = add_site(r, u, v, b.energy, b.seed, tune);
    if i < 0 {
        return b.counts_on_nearest(r);
    }
    res.cracked = true;
    if hard {
        res.released += release_crater(c, ri, i as usize, C_HIT, tune);
    }
    false
}
/// The pane runs: a hard blow knocks out the piece it made there, and stars the site it is nearest to.
fn runs_from(c: &mut Ctx, b: &At, hard: bool, res: &mut Blow, tune: &Knobs) {
    let (ri, u, v) = (b.ri, b.u, b.v);
    set_run(c, ri, tune);
    let r = &mut c.w.panes.rows[ri];
    let (open_now, fid) = (open_uv(r, u, v), classify_row(&r.d, &r.w, u, v));
    if hard && !open_now && fid >= 0 {
        res.released += knock_out(c, ri, fid as u32, tune);
    }
    if hard {
        let j = nearest_site(&c.w.panes.rows[ri], u, v);
        if j >= 0 && tune.glassbreak.run_star > 0.0 {
            ring_out(c, ri, j as usize, tune);
        }
    }
}
/// the blow that RUNS a pane breaks its own star out: the rings round the site it landed on follow the crater over the next few frames
fn ring_out(c: &mut Ctx, ri: usize, j: usize, tune: &Knobs) {
    let (frags, key) = {
        let r = &mut c.w.panes.rows[ri];
        (frags_of(&r.d, &r.w, r.hu, r.hv), r.key as f64)
    };
    let top = js_min(2.0, tune.glassbreak.run_star);
    for f in frags.list() {
        if f.i as usize == j && f.b >= 1 && f.b as f64 <= top {
            let r = &mut c.w.panes.rows[ri];
            if !bit(&r.w, GW_OPEN, f.fid) && !bit(&r.w, GW_GOING, f.fid) {
                let at = c.now + tune.glassbreak.run_ring_secs * (f.b as f64 + hash01(key * 53.0 + f.fid as f64));
                queue(c, ri, f.fid, at, C_HIT, [0.0; 3], 1.0);
            }
        }
    }
}
/// A pellet through glass its own shell has just broken: the piece it passed through, if one still stands, goes with it. → fragments let go
pub fn widen(c: &mut Ctx, bi: u32, x: [f64; 3], tune: &Knobs) -> u32 {
    let ri = match c.w.panes.row_of(bi) {
        Some(r) => r,
        None => return 0,
    };
    let fid = {
        let r = &mut c.w.panes.rows[ri];
        if r.ty != ANNEALED {
            return 0;
        }
        let (u, v) = to_uv_in(r, x[0], x[1], x[2]);
        if open_uv(r, u, v) {
            return 0;
        }
        let fid = classify_row(&r.d, &r.w, u, v);
        if fid < 0 {
            return 0;
        }
        fid as u32
    };
    let n = knock_out(c, ri, fid, tune);
    let r = &mut c.w.panes.rows[ri];
    if n > 0 && r.next_think <= 0.0 {
        r.next_think = c.now + tune.glassbreak.think;
    }
    n
}

// ---- a blast
/// the charges in hand: is this arrival one already seen. → which of them
fn charge_of(p: &mut Panes, now: f64, at: [f64; 3]) -> usize {
    p.charges.retain(|c| now >= c.t && now - c.t < 0.25);
    if let Some(i) = p.charges.iter().position(|c| hypot2(c.at[0] - at[0], c.at[2] - at[2]) < 0.05 && js_abs(c.at[1] - at[1]) < 0.6) {
        return i;
    }
    if p.charges.len() == CHARGES {
        // (sixteen charges inside a quarter of a second: the oldest is forgotten, and a second arrival of it would count as a blow of its own)
        p.charges.remove(0);
    }
    p.seq += 1;
    p.charges.push(Charge { t: now, at, seq: p.seq as f64 });
    p.charges.len() - 1
}
#[inline]
fn prev_k(r: &Row, seq: f64) -> f64 {
    for i in 0..BK {
        if r.bk[2 * i] == seq {
            return r.bk[2 * i + 1];
        }
    }
    0.0
}
#[inline]
fn set_k(r: &mut Row, seq: f64, kk: f64) {
    for i in 0..BK {
        if r.bk[2 * i] == seq {
            r.bk[2 * i + 1] = kk;
            return;
        }
    }
    let at = r.bk_at as usize % BK;
    r.bk[2 * at] = seq;
    r.bk[2 * at + 1] = kk;
    r.bk_at = r.bk_at.wrapping_add(1);
}
/// the arrival in hand: the charge's number, and where it went off
#[derive(Clone, Copy)]
pub struct Arrival {
    seq: f64,
    at: [f64; 3],
}
/// its charge seen, ready for the sheets one at a time
pub fn blast_begin(c: &mut Ctx, at: [f64; 3]) -> Arrival {
    let i = charge_of(&mut c.w.panes, c.now, at);
    let seen = &c.w.panes.charges[i];
    Arrival { seq: seen.seq, at: seen.at }
}
/// WHERE A BLAST AT `at` MEETS PANE `bi`: the nearest point of the pane that it has a line to, of the nearest of its whole face and of the face drawn in (a sill
/// or a jamb stands before the edge of most), its middle and four toward its corners. Walls and leaves are in its way; other glass is not (each pane is judged as
/// if the others were not there). None = no line to a point of it within `radius`.
pub fn blast_meets(w: &World, v: &Over, bi: usize, at: [f64; 3], radius: f64, tune: &Knobs) -> Option<[f64; 3]> {
    let b = rec(w, bi);
    let off = |p: [f64; 3]| hypot3(p[0] - at[0], p[1] - at[1], p[2] - at[2]);
    if off(nearest_on(b, at)) > radius {
        return None;
    }
    let met = |p: [f64; 3], d: f64| d < 1e-4 || lines::blocked_over(w, v, at[0], at[1], at[2], p[0], p[1], p[2], d, 0.02, crate::world::F_GLASS, 0, tune) != -1;
    let (cs, sn, long) = (b[R_COS], b[R_SIN], b[R_H] >= b[R_H + 2]);
    let toward = |u: f64, y: f64| {
        let (lx, lz) = if long { (u * b[R_H], 0.0) } else { (0.0, u * b[R_H + 2]) };
        [b[R_C] + cs * lx + sn * lz, b[R_C + 1] + y * b[R_H + 1], b[R_C + 2] - sn * lx + cs * lz]
    };
    let faces = [1.0, 0.9, 0.75, 0.5].map(|of| nearest_within(b, at, of));
    let spots = [(0.0, 0.0), (0.7, 0.7), (-0.7, 0.7), (0.7, -0.7), (-0.7, -0.7)].map(|(u, y)| toward(u, y));
    let mut best: Option<([f64; 3], f64)> = None;
    for p in faces.into_iter().chain(spots) {
        let d = off(p);
        if d <= radius && best.is_none_or(|(_, nearer)| d < nearer) && met(p, d) {
            best = Some((p, d));
        }
    }
    best.map(|(p, _)| p)
}
/// a blast over one sheet it meets, at the point it meets it (`blast_meets`). → it took the blow (blast()'s count of panes changed)
pub fn blast_sheet(c: &mut Ctx, arrival: Arrival, (bi, q): (u32, [f64; 3]), at: [f64; 3], radius: f64, power: f64, who: f64, tune: &Knobs) -> bool {
    let d = hypot3(q[0] - at[0], q[1] - at[1], q[2] - at[2]);
    let kk = power * (1.0 - d / radius);
    if kk <= BLAST_MIN_K {
        return false;
    }
    let had_row = c.w.panes.row_of(bi);
    let prev = match had_row {
        Some(r) => prev_k(&c.w.panes.rows[r], arrival.seq),
        None => 0.0,
    };
    if prev >= kk {
        return false;
    }
    let ty = type_of_pane(c.w, bi, tune);
    if ty == NO_TYPE || (ty == TEMPERED && kk <= tune.glassbreak.blast_dice_k && had_row.is_none()) {
        return false;
    }
    let Some(ri) = state_of(c.w, bi, tune) else {
        return false;
    };
    let r = &mut c.w.panes.rows[ri];
    if gone(r) {
        return false;
    }
    set_k(r, arrival.seq, kk);
    let away = if d > 1e-6 { v3::normalize(v3::sub(q, at)) } else { normal_of(r) };
    r.last_dir = away;
    r.who = who;
    let (qu, qv) = to_uv(r, q[0], q[1], q[2]);
    r.last_uv = [qu, qv];
    r.damage = js_min(1.0, r.damage + 0.3 * (kk - prev));
    if r.ty == TEMPERED {
        if kk > tune.glassbreak.blast_dice_k && !dicing(r) {
            r.energy += 1.0 + 2.0 * kk;
            let seed = blast_seed(r.key, &arrival.at);
            start_dice(c, ri, qu, qv, seed, tune);
            return true;
        }
        return false;
    }
    stars_toward(r, arrival.at, q, kk, prev, tune);
    r.energy += if prev > 0.0 { 2.0 * (kk - prev) } else { 1.0 + 2.0 * kk };
    if r.ty == LAMINATED {
        if kk > tune.glassbreak.blast_sheet_k {
            release_sheet(c, ri, away, kk, tune);
        }
        return true;
    }
    if kk > tune.glassbreak.blast_run_k {
        set_run(c, ri, tune);
    }
    pieces_go(c, ri, at, (qu, qv), kk, tune);
    let r = &mut c.w.panes.rows[ri];
    if r.next_think <= 0.0 {
        r.next_think = c.now + tune.glassbreak.think;
    }
    true
}
/// Stars toward the blast (a hashed point on the face inside its frame by a tenth, pulled a third of the way to the
/// meeting point `q`), those a blow of `prev` did not make.
fn stars_toward(r: &mut Row, twin_at: [f64; 3], q: [f64; 3], kk: f64, prev: f64, tune: &Knobs) {
    let web = r.ty == LAMINATED;
    let stars_of = |x: f64| if web { 3.0 } else { 1.0 + round_half_up(2.0 * js_min(1.0, x)) };
    let mut s = blast_seed(r.key, &twin_at);
    let stars = stars_of(kk) as u32;
    let had = if prev > 0.0 { stars_of(prev) as u32 } else { 0 };
    for n in 0..stars {
        s = (s ^ (s >> 15)).wrapping_mul(0x2C1B_3C6D).wrapping_add(0x297A_2D39);
        if n < had {
            continue;
        }
        let fu = ((s & 0xffff) as f64 / 65535.0) * 2.0 - 1.0;
        let fv = ((s >> 16) as f64 / 65535.0) * 2.0 - 1.0;
        let a = to_world(r, fu * 0.9 * r.hu, fv * 0.9 * r.hv);
        let m = [v3::lerp(a[0], q[0], 0.35), v3::lerp(a[1], q[1], 0.35), v3::lerp(a[2], q[2], 0.35)];
        let (pu, pv) = to_uv(r, m[0], m[1], m[2]);
        if open_uv(r, pu, pv) || classify_row(&r.d, &r.w, pu, pv) >= 0 {
            continue;
        }
        add_site(r, pu, pv, 0.6 + kk, s, tune);
    }
}
/// Every piece within the blast's reach of where it meets the pane goes, the near ones first.
fn pieces_go(c: &mut Ctx, ri: usize, at: [f64; 3], (qu, qv): (f64, f64), kk: f64, tune: &Knobs) {
    let reach = blast_reach(kk);
    let r = &mut c.w.panes.rows[ri];
    let frags = frags_of(&r.d, &r.w, r.hu, r.hv);
    for f in frags.list() {
        let r = &mut c.w.panes.rows[ri];
        if bit(&r.w, GW_OPEN, f.fid) || bit(&r.w, GW_GOING, f.fid) {
            continue;
        }
        let dd = hypot2(f.cu - qu, f.cv - qv);
        if dd > reach {
            continue;
        }
        if kk < tune.glassbreak.tooth_k && tooth(f, tune) {
            continue;
        }
        queue(c, ri, f.fid, c.now + dd * 0.03, C_BLAST, at, kk);
    }
}
/// metres round where a blast of strength k meets a pane within which its pieces are taken
#[inline]
fn blast_reach(kk: f64) -> f64 {
    1.4 * kk + 0.15
}

// ---- support
static mut HELD_BITS: [u32; 21] = [0; 21];
/// Which intact pieces of a float pane are HELD, grown outward from the frame and the uncut sheet, so every held piece is joined to them through held pieces
fn held_set(r: &Row, without: i32, tune: &Knobs) -> &'static [u32; 21] {
    let held = unsafe { &mut *core::ptr::addr_of_mut!(HELD_BITS) };
    *held = [0; 21];
    let frags = frags_of(&r.d, &r.w, r.hu, r.hv);
    let ac = pattern::across(frags, &r.d, &r.w);
    let (side, top, wedged, stand_ny, stand_foot, tol, hang) = (tune.glassbreak.grip.side, tune.glassbreak.grip.top, tune.glassbreak.grip.wedged, tune.glassbreak.stand_ny, tune.glassbreak.stand_foot, tune.glassbreak.balance_tol, tune.glassbreak.hang_area);
    let mut changed = true;
    while changed {
        changed = false;
        for (at, f) in frags.list().iter().enumerate() {
            if bit(&r.w, GW_OPEN, f.fid) || f.fid as i32 == without || held[(f.fid >> 5) as usize] >> (f.fid & 31) & 1 != 0 {
                continue;
            }
            let mut ok = false;
            let (mut u0, mut u1) = (f64::INFINITY, f64::NEG_INFINITY);
            let mut grip = (f.frame[pattern::F_LEFT] + f.frame[pattern::F_RIGHT] + f.frame[pattern::F_BOTTOM]) * side + f.frame[pattern::F_TOP] * top;
            if f.has_foot {
                u0 = f.foot.0;
                u1 = f.foot.1;
            }
            let mut e = ac.head[at];
            while e != 0 {
                let n = &ac.nbs[e as usize - 1];
                e = n.next;
                if n.dead {
                    continue;
                }
                if n.fid == REMAINDER {
                    ok = true;
                    break;
                }
                if n.fid < 0 {
                    continue;
                }
                let o = n.fid as u32;
                if frags.place(o) == 0 || bit(&r.w, GW_OPEN, o) || o as i32 == without || held[(o >> 5) as usize] >> (o & 31) & 1 == 0 {
                    continue;
                }
                grip += n.len * wedged;
                if n.ny < -stand_ny {
                    u0 = js_min(u0, n.u0);
                    u1 = js_max(u1, n.u1);
                }
            }
            if !ok && u1 > u0 {
                ok = f.cu >= u0 - tol && f.cu <= u1 + tol && u1 - u0 >= stand_foot * (f.u_max - f.u_min);
            }
            if !ok {
                ok = grip > 0.0 && f.area <= hang * grip;
            }
            if ok {
                held[(f.fid >> 5) as usize] |= 1 << (f.fid & 31);
                changed = true;
            }
        }
    }
    unsafe { &*core::ptr::addr_of!(HELD_BITS) }
}
/// What a change left unheld falls, a beat later and staggered
fn support_pass(c: &mut Ctx, ri: usize, tune: &Knobs) {
    let (frags, key, lu, lv) = {
        let r = &mut c.w.panes.rows[ri];
        if r.ty != ANNEALED || r.n_open == 0 {
            return;
        }
        (frags_of(&r.d, &r.w, r.hu, r.hv), r.key as f64, r.last_uv[0], r.last_uv[1])
    };
    let held = *held_set(&c.w.panes.rows[ri], -1, tune);
    let (per, jit) = (tune.glassbreak.stagger_per_m, tune.glassbreak.stagger_jit);
    for f in frags.list() {
        let r = &mut c.w.panes.rows[ri];
        if bit(&r.w, GW_OPEN, f.fid) || held[(f.fid >> 5) as usize] >> (f.fid & 31) & 1 != 0 || bit(&r.w, GW_GOING, f.fid) {
            continue;
        }
        let d = hypot2(f.cu - lu, f.cv - lv);
        queue(c, ri, f.fid, c.now + d * per + jit * hash01(key * 31.0 + f.fid as f64), C_SUPPORT, [0.0; 3], 1.0);
    }
}

// ---- per frame
/// a new frame's budget
#[inline]
pub fn frame_begin(p: &mut Panes) {
    p.released = 0;
}
/// one who may rake: a man on his feet with his hands free
#[derive(Clone, Copy)]
pub struct Walker {
    pub pos: [f64; 3],
    pub vel: [f64; 3],
    pub id: f64,
}
/// as many as one step hears of
pub const MAX_MEN: usize = 16;
/// a frame of one pane
pub fn step_row(c: &mut Ctx, ri: usize, men: &[Walker], tune: &Knobs) {
    {
        let r = &mut c.w.panes.rows[ri];
        if gone(r) {
            return;
        }
        if dicing(r) {
            step_dice(c, ri, tune);
            return;
        }
    }
    let now = c.now;
    if !c.w.panes.going[ri].is_empty() {
        let mut went = false;
        let budget = tune.glassbreak.releases_per_frame;
        loop {
            let p = &mut c.w.panes;
            if p.going[ri].first().is_none_or(|q| q.at > now) || p.released as f64 >= budget {
                break;
            }
            let e = q_pop(p, ri);
            let r = &p.rows[ri];
            let exists = frags_of(&r.d, &r.w, r.hu, r.hv).by(e.fid).map(|f| (f.b, f.i));
            if let Some((b, i)) = exists {
                if !bit(&r.w, GW_OPEN, e.fid) {
                    let from = if e.cause == C_BLAST || e.cause == C_RAKE { Some(e.from) } else { None };
                    if b == 0 {
                        release_crater(c, ri, i as usize, e.cause, tune);
                    } else {
                        release_frag(c, ri, e.fid, e.cause, from, e.k, tune);
                    }
                    c.w.panes.released += 1;
                    went = true;
                }
            }
        }
        let r = &mut c.w.panes.rows[ri];
        if went && r.next_think <= 0.0 {
            r.next_think = now + tune.glassbreak.think;
        }
    }
    {
        let r = &mut c.w.panes.rows[ri];
        if r.next_think > 0.0 && now >= r.next_think {
            r.next_think = 0.0;
            support_pass(c, ri, tune);
        }
    }
    let rake_it = {
        let r = &mut c.w.panes.rows[ri];
        run(r) && r.walkable == 0 && tune.glassbreak.rake_open > 0.0 && r.ty == ANNEALED
    };
    if rake_it {
        for m in men.iter().take(MAX_MEN) {
            rake_by(c, ri, m, tune);
        }
    }
}
/// A man on his feet walks into what is left of a run pane that is mostly hole: the pieces in his way go, away from him, nearest first
fn rake_by(c: &mut Ctx, ri: usize, m: &Walker, tune: &Knobs) {
    let (pos, vel) = (m.pos, m.vel);
    let (frags, mu, key, v_clear) = {
        let r = &mut c.w.panes.rows[ri];
        let n = normal_of(r);
        let w = v3::dot(v3::sub(pos, r.c), n);
        if js_abs(w) > tune.glassbreak.rake_m || pos[1] >= r.c[1] + r.hv {
            return;
        }
        let (mu, _) = to_uv(r, pos[0], pos[1], pos[2]);
        if js_abs(mu) > r.hu + 0.1 {
            return;
        }
        let vn = -v3::dot(vel, n) * sign_or_one(w);
        let sp = hypot2(vel[0], vel[2]);
        if vn <= 0.3 || vn < 0.6 * sp {
            return;
        }
        (frags_of(&r.d, &r.w, r.hu, r.hv), mu, r.key as f64, tune.glassbreak.walk_clear_h - r.c[1])
    };
    {
        let p = &c.w.panes;
        if p.going[ri].iter().any(|q| q.cause == C_RAKE) || open_fraction_row(&p.rows[ri]) < tune.glassbreak.rake_open {
            return;
        }
    }
    let from = [pos[0], 1.0, pos[2]];
    let mut took = 0.0;
    for f in frags.list() {
        let r = &mut c.w.panes.rows[ri];
        if bit(&r.w, GW_OPEN, f.fid) || f.v_min >= v_clear || bit(&r.w, GW_GOING, f.fid) {
            continue;
        }
        if tooth(f, tune) {
            continue;
        }
        queue(c, ri, f.fid, c.now + 0.04 + 0.12 * js_abs(f.cu - mu) + 0.05 * hash01(key * 71.0 + f.fid as f64), C_RAKE, from, 1.0);
        took += 1.0;
    }
    if took > 0.0 {
        c.w.panes.rows[ri].who = m.id;
    }
}

// ---- questions
/// how much of a row's pane is open
fn open_fraction_row(r: &Row) -> f64 {
    if gone(r) {
        return 1.0;
    }
    if dicing(r) {
        return r.d_next as f64 / (r.w[GW_COLS] * r.w[GW_ROWS]) as f64;
    }
    if r.n_open == 0 {
        return 0.0;
    }
    let mut a = 0.0;
    for f in frags_of(&r.d, &r.w, r.hu, r.hv).list() {
        if bit(&r.w, GW_OPEN, f.fid) {
            a += f.area;
        }
    }
    js_min(1.0, a / (4.0 * r.hu * r.hv))
}
pub fn open_fraction(p: &Panes, bi: u32) -> f64 {
    p.row_of(bi).map_or(0.0, |ri| open_fraction_row(&p.rows[ri]))
}
#[inline]
pub fn walkable(p: &Panes, bi: u32) -> bool {
    p.row_of(bi).is_some_and(|ri| p.rows[ri].walkable != 0)
}
/// A number that changes whenever what is open in any pane changes
pub fn open_sig(p: &Panes) -> i32 {
    let mut s = p.rows.len() as i32;
    for r in &p.rows {
        let v = r.w[0] as i32 * 7 + r.n_open as i32 + (if dicing(r) { r.d_next as i32 } else { 0 }) + (if gone(r) { 4096 } else { 0 });
        s = s.wrapping_mul(31).wrapping_add(v);
    }
    s
}
/// EVERY PANE THAT IS OPEN SOMEWHERE: its static, and a word of all that says where (what glass.rs `open` reads of its row with no burst's view in hand)
pub fn open_panes(p: &Panes) -> impl Iterator<Item = (u32, u64)> + '_ {
    p.rows.iter().filter(|r| r.n_open != 0 || dicing(r) || gone(r)).map(|r| {
        let mut h = 0xcbf29ce484222325u64;
        if !gone(r) {
            for v in r.w[..GW_GOING].iter().map(|w| *w as u64).chain(r.d.iter().map(|d| d.to_bits())) {
                h = (h ^ v).wrapping_mul(0x9e3779b97f4a7c15);
                h ^= h >> 29;
            }
        }
        (r.w[0], h)
    })
}
/// Is the point where a piece of static `bi` has fallen out
pub fn open_at(p: &Panes, bi: u32, x: f64, y: f64, z: f64) -> bool {
    p.row_of(bi).is_some_and(|ri| {
        let r = &p.rows[ri];
        let (u, v) = to_uv(r, x, y, z);
        open_uv(r, u, v)
    })
}
/// what a line or a ray meeting static `bi` reads of it (glass.rs `open`): None = nothing of it is out or on its way
#[inline]
pub fn row_for(p: &Panes, bi: u32) -> Option<&Row> {
    let ri = p.row_of(bi)?;
    let r = &p.rows[ri];
    (r.n_open != 0 || !p.going[ri].is_empty() || dicing(r) || gone(r)).then_some(r)
}
/// would a burst of this size take pane `bi` out of the way at (x, y, z). `sure`: a man counting on it (the near reach alone), else a wary one (the whole pane or nothing).
pub fn gives(o: &World, bi: usize, x: f64, y: f64, z: f64, tune: &Knobs) -> bool {
    let (at, radius, power, sure) = ([o.view_at[0], o.view_at[1], o.view_at[2]], o.view_at[3], o.view_at[4], o.view_at[5] != 0.0);
    let s = match sheet_of(o, bi as u32) {
        Some(s) => s,
        None => return false,
    };
    let b = rec(o, bi);
    if !thin(b) || !(radius > 0.0) || !(power > 0.0) {
        return false;
    }
    let Some(&(_, q)) = o.view_met.iter().find(|m| m.0 == bi as u32) else {
        return false;
    };
    let kk = power * (1.0 - hypot3(q[0] - at[0], q[1] - at[1], q[2] - at[2]) / radius);
    if kk < js_min(js_min(tune.glassbreak.blast_dice_k, tune.glassbreak.blast_sheet_k), if sure { tune.glassbreak.tooth_k } else { BLAST_MIN_K }) {
        return false;
    }
    let (over, sky, gap) = (over_of(&o.panes, bi as u32), o.sheets[s].sky, o.sheets[s].gap);
    // (every sheet's sky was settled as the level stood up: a line asks no line of its own here)
    let ty = if over != NO_OVER { over } else { place_rule(sky != 0 && sky != SKY_UNKNOWN, b[R_H], b[R_H + 1], b[R_H + 2], b[R_C + 1], gap, tune) };
    if ty == TEMPERED {
        return kk > tune.glassbreak.blast_dice_k;
    }
    if ty == LAMINATED {
        return kk > tune.glassbreak.blast_sheet_k;
    }
    if !sure {
        return kk > BLAST_MIN_K;
    }
    let reach = blast_reach(kk);
    kk >= tune.glassbreak.tooth_k && reach >= 0.0 && hypot3(x - q[0], y - q[1], z - q[2]) <= reach
}
/// is `bi` a sheet the wind should ask of: its face to the sky, or 0
pub fn sky_known(w: &World, bi: u32, tune: &Knobs) -> i32 {
    match sheet_of(w, bi) {
        Some(s) => sky_side(w, s, tune),
        None => 0,
    }
}
/// the floor's last 'glass' event a pane a shooter: a hit on this pane by this shooter may be told now. Entries past the gap say nothing and go.
pub fn heard_gap(p: &mut Panes, pane: u32, who: f64, now: f64, gap: f64) -> bool {
    let key = pane as f64 * 1024.0 + (int32(who) & 1023) as f64;
    let fresh = |t: f64| now - t >= 0.0 && now - t < gap;
    let said = p.told.iter().rfind(|t| t[0] == key).is_some_and(|t| fresh(t[1]));
    p.told.retain(|t| fresh(t[1]));
    if said {
        return false;
    }
    if p.told.len() < EVENTS {
        p.told.push([key, now]);
    }
    true
}
/// where the jet off a sheet's far face comes down (the air has most of a fine's speed inside a stride)
pub fn jet(c: &mut Ctx, at: [f64; 3], d: [f64; 3], speed: f64, amount: f64, who: f64, tune: &Knobs) {
    let v = [d[0] * (speed * 0.35), d[1] * (speed * 0.35), d[2] * (speed * 0.35)];
    let (t, p) = fall_of(c.w, at, v, c.gravity, 0.8, tune);
    c.landings.push(falls::Landing { t: c.now + t, pos: p, sound: FallSound::GlassTinkle, gain: 0.0, rate: 1.0, amount, who, told: FallTold::Never, level: 0.0, dust: 0.0 });
}
/// a dicing pane's open threshold as the picture reads it: the midpoint between the last clump out and the next in
pub fn dice_threshold(r: &Row) -> f64 {
    let d = dice_of(r);
    let next = r.d_next as usize;
    if next >= d.cols as usize * d.rows as usize {
        return 1e9;
    }
    if next == 0 {
        return -1.0;
    }
    let order = order_of(&d);
    (order[next - 1].key + order[next].key) / 2.0
}

// ---- a window's bar (the pieces it throws are the caller's)
const F_BAR: u32 = 1 << 19;
const NO_SHADOW: u32 = crate::world::F_NO_SHADOW;
/// is static `bi` a window's bar that still stands? (its row keeps NoShadow of the two bits a broken bar is hidden by)
#[inline]
fn bar_stands(o: &World, bi: usize) -> bool {
    o.flags[bi] & (F_BAR | NO_SHADOW) == F_BAR
}
/// bar `bi` goes, unless it is gone already or is no standing bar. `flags`: the box's flags as it stands (its row keeps a part of them)
pub fn bar_take(w: &mut World, bi: u32, flags: u32) -> bool {
    let at = w.panes.bars.partition_point(|r| r[0] < bi);
    if w.panes.bars.get(at).is_some_and(|r| r[0] == bi) || !bar_stands(w, bi as usize) {
        return false;
    }
    w.panes.bars.insert(at, [bi, flags]);
    true
}
/// what bars a charge at `from` takes: every standing bar with a point of its length inside its reach and nothing solid between, each judged as if the others
/// were not there. A static and the point each, ascending.
pub fn bars_blast(w: &World, v: &Over, from: [f64; 3], reach: f64, statics: usize, tune: &Knobs) -> Vec<(u32, [f64; 3])> {
    let near: Vec<u32> = (0..statics as u32)
        .filter(|&bi| {
            let b = rec(w, bi as usize);
            bar_stands(w, bi as usize) && hypot2(b[R_C] - from[0], b[R_C + 2] - from[2]) <= reach + js_max(b[R_H], b[R_H + 2])
        })
        .collect();
    if near.is_empty() {
        return Vec::new();
    }
    let open: Vec<u32> = near.iter().chain(v.open).copied().collect();
    let no_bars = Over { open: &open, ..*v };
    let mut took = Vec::new();
    for &bi in &near {
        let b = rec(w, bi as usize);
        for u in [0.0, 0.6, -0.6] {
            let q = [b[R_C], b[R_C + 1] + u * b[R_H + 1], b[R_C + 2]];
            if hypot3(from[0] - q[0], from[1] - q[1], from[2] - q[2]) > reach + js_max(b[R_H], b[R_H + 2]) {
                continue;
            }
            if lines::solid_over(w, &no_bars, ASK_GLASS, from[0], from[1], from[2], q[0], q[1], q[2], tune).unwrap_or(true) {
                continue;
            }
            took.push((bi, q));
            break;
        }
    }
    took
}
