//! THE SIMULATION IN RUST — the game's update, moved here a system at a time and built to WebAssembly for the page, the game server and the headless tools
//! (docs/agents/rust.md says how a system moves, and what holds it to the TypeScript one it replaces).
//!
//! What every line in this crate owes the game:
//!   · one arithmetic. Doubles, and only + − × ÷ √ and what the standard makes exact of them (floor, trunc, abs, comparisons, the bits). Nothing that a
//!     compiler or a machine may round as it likes: no `f64::sin` and friends (rust/maths has the game's own), no `mul_add`, no fast-math, no `f32` where the
//!     TypeScript holds a double.
//!   · one order. Nothing whose order is not written down: no `HashMap` / `HashSet` iteration, no pointer values as keys, no threads.
//!   · no world outside. No clock, no random source of its own (the game's die is in its block: dice.rs), no files.
//!   · state in one place. What a game keeps here from one call to the next is in its block (state.rs: indices, never addresses); every other `static mut`
//!     is scratch of one call or a copy the host proves.
//!
//! WHO HOLDS IT. The compiler: the crate is `no_std`, so no line of it can name a hash map, a thread, a clock or a file (`std::…` does not resolve here).
//! tools/qa/rust-rules-check.ts: the float methods the compiler cannot keep out once any crate links the standard
//! library (`.sin()`, `.powf()`, `.mul_add()` …), an `f32`, and the SIMD that rounds as the machine likes. The reader: the order of the arithmetic.
//! √, floor and trunc, which the core library lacks, come from rust/prim.
#![no_std]

pub mod air;
pub mod bricks;
pub mod clips;
pub mod collide;
pub mod dice;
pub mod doors;
pub mod falls;
pub mod fog;
pub mod glass;
pub mod glow;
pub mod hear;
pub mod knobs;
pub mod layers;
pub mod light;
pub mod lights;
pub mod lines;
pub use ::maths;
pub mod men;
pub mod nav;
pub mod panes;
pub mod pattern;
pub mod pen;
pub mod poses;
pub mod props;
pub mod sight;
pub mod single;
pub mod skeleton;
pub mod items;
pub mod slabs;
pub mod snow;
pub mod state;
pub mod weather;
pub mod wind;
pub mod world;

/// How many doubles the scratch holds.
pub const SCRATCH_LEN: usize = 1024;
/// A few arguments or answers too many for a call's own parameters go through here: the caller writes, calls, reads. Nothing is kept in it between calls.
static mut SCRATCH: [f64; SCRATCH_LEN] = [0.0; SCRATCH_LEN];

/// The version of the calls: the rules name the one they were written against (sim_sim).
pub const ABI: u32 = 34;

pub fn sim_abi() -> u32 {
    ABI
}
pub fn sim_scratch() -> *mut f64 {
    core::ptr::addr_of_mut!(SCRATCH) as *mut f64
}
pub fn sim_scratch_len() -> usize {
    SCRATCH_LEN
}

// ---- the box world's copy (world.rs) and the line tests through it (lines.rs)
/// how many rows a table holds
pub fn world_cap(table: u32) -> usize {
    world::cap(table)
}
/// the grid of world `w` (cells across, cells deep, a cell's side) and how many statics and tilted statics its tables now hold
pub fn world_shape(w: usize, gw: u32, gh: u32, cs: f64, boxes: u32, tilted: u32) {
    let o = world::world(w);
    o.w = gw;
    o.h = gh;
    o.cs = cs;
    o.boxes = boxes;
    o.n_tilted = tilted;
}
/// how many leaves list `set` of world `w` now holds
pub fn world_leaves(w: usize, set: usize, n: u32) {
    if set < world::LEAF_SETS {
        world::world(w).leaves[set] = n;
    }
}
/// how many things of furniture stand in world `w` now (their parts are list THING_SET of the leaves: world_leaves says how many)
pub fn world_things(w: usize, n: u32) {
    world::world(w).things = if (n as usize) < world::MAX_THINGS { n } else { world::MAX_THINGS as u32 };
}
/// how many panes of world `w` are no longer whole (the rows of its glass tables that are written: glass.rs)
pub fn world_glass(w: usize, n: u32) {
    world::world(w).glass_n = if (n as usize) < world::MAX_GLASS { n } else { world::MAX_GLASS as u32 };
}
/// the level's sheets of glass for world `w` (scratch: a static, how far it stands from a door's opening, which face has the sky, each)
pub fn world_sheets(w: usize, n: usize) {
    let (o, d) = (world::world(w), scratch());
    let n = if n < world::MAX_SHEETS { n } else { world::MAX_SHEETS };
    for i in 0..n {
        o.sheets[i] = world::Sheet { bi: d[3 * i] as u32, gap: d[3 * i + 1], sky: d[3 * i + 2] as i32 };
    }
    o.sheet_n = n as u32;
}
/// a burst's view of the glass of world `w` for the judgment in hand (glass.rs VIEW_*; 0 = none), and how many panes its rows name
pub fn world_view(w: usize, kind: u32, n: u32) {
    let o = world::world(w);
    o.view_kind = kind;
    o.view_n = if (n as usize) < world::MAX_VIEW { n } else { world::MAX_VIEW as u32 };
}
/// glassbreak.ts `openAt(bi, X) || also(bi, X)`: is static `bi`, a pane of glass, open at the point (1) or not (0)
pub fn glass_open(w: usize, bi: u32, x: f64, y: f64, z: f64) -> u32 {
    let o = world::world(w);
    ((bi as usize) < world::MAX_BOXES && glass::open(o, bi as usize, x, y, z)) as u32
}
// ---- the game's state in the module (state.rs: a block a game, bound to a slot by its host)
/// a block's length in bytes
pub fn state_len() -> usize {
    state::LEN
}
/// how many blocks the module holds at once
pub fn state_games() -> usize {
    state::GAMES
}
/// the layout's word
pub fn state_layout() -> u32 {
    state::LAYOUT
}
/// how many parts a block has
pub fn state_parts() -> usize {
    state::PARTS.len()
}
/// part `i` of a block: where it starts (`what` 0) or how long it is (1), in bytes; whether it is a pool, its first word the bytes of it in use (2)
pub fn state_part(i: usize, what: u32) -> usize {
    if i >= state::PARTS.len() {
        return 0;
    }
    match what {
        0 => state::PARTS[i].off,
        1 => state::PARTS[i].len,
        _ => state::PARTS[i].pool as usize,
    }
}

// ---- the level's lights (lights.rs: fixtures, the mains, the lamps they feed, a camera's and a tripwire's machine, in the game's block)
/// how many a part holds, how long it is, where a column is (lights.rs cap)
pub fn lights_cap(table: u32) -> usize {
    lights::cap(table)
}

// ---- the doors' step (doors.rs: the leaves' state in the game's block, the men near them handed in, each leaf's row of the box world written)
/// how many rows a table holds (5: the columns of the doors' part of a block)
pub fn doors_cap(table: u32) -> usize {
    doors::cap(table)
}
// ---- the furniture men shove (props.rs: each thing's state in the game's block, the men, the leaves and the door frames handed in)
/// how many rows a table holds (0, 2 … 6), how wide a row of one is (9 … 12: a thing's facts, a man, a frame, a noise), how long the part is (13), and
/// where column k of the part starts (100 + k)
pub fn props_cap(table: u32) -> usize {
    props::cap(table)
}
// ---- the senses (sight.rs, light.rs, bricks.rs, hear.rs, nav.rs): what a man sees, how lit he is, what the smoke hides, how a sound reaches him
/// the scratch, for an answer of more than one number (one thread, and nothing else holds it across a call)
#[inline(always)]
fn scratch() -> &'static mut [f64; SCRATCH_LEN] {
    unsafe { &mut *core::ptr::addr_of_mut!(SCRATCH) }
}
pub fn light_cap() -> usize {
    light::MAX_LAMPS
}
/// what a search holds: 0 the cells of the largest floor it can be asked over (nav::MAX_FLOOR: on such a floor no table of a search is outgrown) ·
/// 1 a way's waypoints
pub fn nav_cap(table: u32) -> usize {
    match table {
        0 => nav::MAX_FLOOR,
        1 => nav::MAX_WAY,
        _ => 0,
    }
}
// ---- the game's own smoke on the coarse grid (bricks.rs): its state is a part of the game's block (state.rs), so a call names the block's slot `g`;
// one that reads the tables made from the state names the block's `stamp` too (`state::stamp`), and a step the box world `w` it is
// taken over and that copy's number `copy`
/// a static of box world `w` was written inside this rectangle of the plan: the faces near it are asked again at the next step
pub fn grid_dirty(w: usize, x0: f64, z0: f64, x1: f64, z1: f64) {
    bricks::dirty(w, x0, z0, x1, z1)
}
pub fn grid_wind_cap() -> usize {
    bricks::MAX_GLOW
}

// ---- WHAT A PANE OF GLASS IS TO PLAY (panes.rs, pen.rs): the state in the game's block, the rule, a round's toll. A rule's call names its game, its world, the leaves
// and the time once (`panes_ctx`), and what it let go is read from `panes_out_*` and cleared before the next pane's.
static mut PANE_CTX: panes::Ctx = panes::Ctx { g: 0, w: 0, set: -1, now: 0.0, shards: false, gravity: 9.81 };
#[inline(always)]
fn cx() -> panes::Ctx {
    unsafe { PANE_CTX }
}
pub fn panes_ctx(g: usize, w: usize, set: i32, now: f64, shards: u32, gravity: f64) {
    unsafe { PANE_CTX = panes::Ctx { g, w, set, now, shards: shards != 0, gravity } };
}
pub fn panes_world(w: usize, g: i32) {
    world::world(w).panes_g = (g + 1) as u32;
}
pub fn panes_view(w: usize, kind: u32, x: f64, y: f64, z: f64, radius: f64, power: f64, sure: f64) {
    let o = world::world(w);
    o.view_kind = kind;
    o.view_n = 0;
    o.view_at = [x, y, z, radius, power, sure];
}
/// glassbreak.ts impact: live 1 · was open 2 · punched 4 · diced 8 · cracked 16 · the glass × 32 · fragments let go × 256
pub fn panes_impact(bi: u32, x: f64, y: f64, z: f64, has_dir: u32, dx: f64, dy: f64, dz: f64, energy: f64, who: f64, through: i32) -> u32 {
    let r = panes::impact(&cx(), bi, [x, y, z], if has_dir != 0 { Some([dx, dy, dz]) } else { None }, energy, who, through);
    r.live as u32 | (r.was_open as u32) << 1 | (r.punched as u32) << 2 | (r.diced as u32) << 3 | (r.cracked as u32) << 4 | r.ty << 5 | r.released << 8
}
pub fn panes_widen(bi: u32, x: f64, y: f64, z: f64) -> u32 {
    panes::widen(&cx(), bi, [x, y, z])
}
pub fn panes_blast_begin(x: f64, y: f64, z: f64) {
    panes::blast_begin(&cx(), [x, y, z]);
}
pub fn panes_blast_sheet(s: usize, x: f64, y: f64, z: f64, radius: f64, power: f64, who: f64) -> u32 {
    panes::blast_sheet(&cx(), s, [x, y, z], radius, power, who) as u32
}
pub fn panes_sheets() -> u32 {
    panes::sheet_count(&cx())
}
pub fn panes_step(r: usize) {
    panes::step_row(&cx(), r);
}
pub fn panes_type(bi: u32) -> u32 {
    panes::type_of_pane(&cx(), bi)
}
pub fn panes_sky(bi: u32) -> i32 {
    panes::sky_known(&cx(), bi)
}
/// glass.ts boxSeed of a box of the world; pointSeed
pub fn panes_box_seed(cx0: f64, cy: f64, cz: f64, hx: f64, hy: f64, hz: f64) -> u32 {
    let mut b = [0.0; world::REC];
    b[world::R_C] = cx0;
    b[world::R_C + 1] = cy;
    b[world::R_C + 2] = cz;
    b[world::R_H] = hx;
    b[world::R_H + 1] = hy;
    b[world::R_H + 2] = hz;
    panes::box_seed(&b)
}
/// shards.ts crossed: a round's exit jet, its glass comes down where the jet does. `amount`: m² of it.
pub fn panes_jet(x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64, speed: f64, amount: f64, who: f64) {
    panes::jet(&cx(), [x, y, z], [dx, dy, dz], speed, amount, who);
}

/// what a game holds of what levelfit.ts counts: 0 window's bars gone · 1 clumps of a sheet
pub fn panes_cap(i: u32) -> u32 {
    (if i == 0 { panes::MAX_BARS } else { pattern::MAX_CLUMPS }) as u32
}

// ---- a window's bar
/// winbars.ts breakBar: 1 this bar goes, its flags as it stood filed
pub fn bars_take(bi: u32, flags: u32) -> u32 {
    panes::bar_take(&cx(), bi, flags) as u32
}
/// winbars.ts barsBlast: how many bars a charge takes, and in scratch a static and the point each
pub fn bars_blast(x: f64, y: f64, z: f64, reach: f64, statics: usize) -> usize {
    panes::bars_blast(&cx(), [x, y, z], reach, statics, scratch())
}

// ---- a round and a pane (pen.rs)
/// glasspen.ts paneCrossing: 1 and the crossing → scratch 0 … 24, what it is born in → scratch 25 · 0 spent in it
pub fn pen_cross(bi: u32, ox: f64, oy: f64, oz: f64, dx: f64, dy: f64, dz: f64, t: f64, e: f64, nth: usize) -> u32 {
    match pen::pane_crossing(&cx(), bi, [ox, oy, oz], [dx, dy, dz], t, e, nth) {
        Some(x) => {
            let s = scratch();
            pen::write_cross(&x, &mut s[..pen::CROSS]);
            s[pen::CROSS] = x.into as f64;
            1
        }
        None => 0,
    }
}
/// glasspen.ts armThroughGlass's walk: −1 the old rule stands · n panes took, their crossings → scratch; the energy left → the last double of the scratch
pub fn pen_arm(fx: f64, fy: f64, fz: f64, mx: f64, my: f64, mz: f64, dx: f64, dy: f64, dz: f64, energy: f64) -> i32 {
    let s = scratch();
    let mut left = 0.0;
    let n = pen::arm_walk(&cx(), [fx, fy, fz], [mx, my, mz], [dx, dy, dz], energy, &mut s[..SCRATCH_LEN - 1], &mut left);
    s[SCRATCH_LEN - 1] = left;
    n
}
/// glasspen.ts shotPasses over the panes in scratch (a static and how far along each)
pub fn pen_passes(ax: f64, ay: f64, az: f64, rx: f64, ry: f64, rz: f64, n: usize, energy: f64) -> u32 {
    let s = scratch();
    let mut list = [0.0; 2 * world::MAX_PANES];
    let n = if n > world::MAX_PANES { world::MAX_PANES } else { n };
    list[..2 * n].copy_from_slice(&s[..2 * n]);
    pen::shot_passes(&cx(), [ax, ay, az], [rx, ry, rz], &list[..2 * n], energy) as u32
}
// ---- the weather that play reads (wind.rs, weather.rs, air.rs, snow.rs)
/// sizes the host holds its own to: 0 roofs · 1 thunder rows · 2 the plan's cells across · 3 down
pub fn weather_cap(i: u32) -> usize {
    match i {
        0 => weather::ROOFS,
        1 => weather::THUNDERS,
        2 => air::AIR_W,
        _ => air::AIR_H,
    }
}
/// the dust's table of the block in slot `g`, whose stamp is `stamp`, and the slot's tables made from it (air.rs plan)
fn air_of(g: usize, stamp: f64) -> (&'static mut air::Air, &'static mut air::Plan) {
    let wx = weather::of(g);
    let p = air::plan(g, stamp, &wx.air, &wx.roofs, wx.roofs_n as usize);
    (&mut wx.air, p)
}
/// the dust's field stood up afresh: every cell the open air's, no opening asked, no list
pub fn air_clear(g: usize, stamp: f64) {
    let (a, p) = air_of(g, stamp);
    a.clear(p)
}
/// the field begun: its switch, nothing owed, no opening known
pub fn air_begin(g: usize, stamp: f64, on: u32) {
    let (a, p) = air_of(g, stamp);
    a.begin(p, on != 0)
}
/// the field seeded under the roofs in force at the leak given
pub fn air_seed(g: usize, stamp: f64, leak: f64) {
    let (a, p) = air_of(g, stamp);
    let wx = weather::of(g);
    let (roofs, n) = (wx.roofs, wx.roofs_n as usize);
    a.seed(p, leak, |x, z| weather::under(&roofs, n, x, z))
}
/// …every opening asked and run on until it stands still. Said → scratch 0‥3 (air.rs Said)
pub fn air_settle(g: usize, stamp: f64, w: usize, set: i32, ask: u32) {
    let mut said = air::Said { moved: 0.0, steps: 0.0, probes: 0.0, open: 0.0 };
    let (a, p) = air_of(g, stamp);
    a.settle(p, w, set, ask, &mut said);
    said.tell(scratch());
}
/// one tick of it: said → scratch 0‥3
pub fn air_update(g: usize, stamp: f64, w: usize, set: i32, ask: u32, dt: f64) {
    let mut said = air::Said { moved: 0.0, steps: 0.0, probes: 0.0, open: 0.0 };
    let (a, p) = air_of(g, stamp);
    a.update(p, w, set, ask, dt, &mut said);
    said.tell(scratch());
}
