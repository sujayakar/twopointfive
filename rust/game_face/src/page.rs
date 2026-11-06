//! THE FACE'S EXPORTS FOR A HOST THAT SHOWS THE GAME (the page, src/host/rustgame.ts): the Rust game steps, and after each step only what the picture
//! reads goes back, as the filtered export (`game::puppet::export_puppet`, tools/blitz/PUPPET.md) with the step's out events on its tail.
//!
//!   face_new_at(state, n, slot) -> h   a game from a start (the bridge's bytes) OVER THE BLOCK THAT ALREADY STANDS in rust/sim's slot `slot`: the host's
//!                                      own code stood the level through this same module's `rust/sim` exports, so the block is not handed over and not
//!                                      copied anywhere else. The host says which slot (src/sim/state.ts holds the slots); the game is pinned to it.
//!   face_puppet(h, start)              the filtered export at face_out. `start` = 1: the long list and every row (after face_new_at, and it fills the
//!                                      memo of what was last sent); 0: what changed since the last call. The out events ride on its tail, and the
//!                                      game's list of them is emptied here: they are a step's output, read once.
//!   face_probe_asks(h)                 THE LIGHT PROBE IS THE HOST'S (render/probe.ts: the GPU answers a frame or more later). What the tick asked of it
//!                                      and the host has not yet taken, at face_out: u32 count, then for each its key (u32 length, utf-8), the point
//!                                      (3 f64), whose own proxies are skipped (f64), a byte and the far end of a line (3 f64) where the byte is 1,
//!                                      and its rank (f64). Nothing is taken: the host calls this after each step and holds its own table to it.
//!                                      Then u32 count and the keys the tick still holds an ANSWER for: a key it forgot inside the tick (a sight
//!                                      line picked up again starts from no answer: `sensors_forget`) is gone from the list, and the host lets its
//!                                      own answer for it go, or it would hand the stale one straight back before the next step.
//!   face_probe_took(h)                 the host's frame has sent the table to the GPU: it is empty again here, and whose turn it is on a full frame is
//!                                      kept (`probe_like_take_asks`)
//!   face_probe_results(h, p, n)        the answers the host holds now, before a step: u32 count, each a key and its three numbers
//!   face_lights_out(h)                 STAND-IN: THE PICTURE'S LIGHTS. The renderer's lights (render/lights.ts RtLight: a fixture, a torch, a muzzle
//!                                      flash) are written by the tick and are on neither list of the filtered export, so the puppet's never move, go
//!                                      out or flash. At face_out: u32 count, then for each light of the engine's list in its order its row's number
//!                                      (u32) and 32 f64: kind, pos, dir, color, intensity, range, inner, outer, radius, volumetric, owner, enabled,
//!                                      fixture_box, flicker, row, ttl, age, peak, transient, broad, mark_ref, card, glass_mask, fog, fog_ghost, fog_by
//!                                      (an Option that is None is NaN). Belongs in the filtered export (RtLight and LightSet.lights on its lists).
//!   face_poke(h, what, v) -> 1 | 0     a host's switch written between ticks, by number: 0 god mode · 1 the AI on · 2 infinite ammunition · 3 the
//!                                      player unseen · 4 the launcher's held button let go unsent (the pause card: host/loop.ts). `v` != 0 = on.
//!   face_host_has(h, bits)             STAND-IN: WHICH OF THE HOST'S OBJECTS EXIST (src/host/api.ts HostApi: the fields the bridge's engine list leaves
//!                                      out). The tick asks `engine.particles` and `engine.reach` only whether they are there (particleFx.ts `gpuFx`: with
//!                                      a particle pool a hit's embers, a blast's fragments and every puff go to it as out events, and without one the
//!                                      sparks' own pool is filled; wallbreak.ts: a breached wall tells the lights' reach keeper), and of the pool its two
//!                                      switches a door charge's fragments are thrown by (blastLook.ts). bit 0 a particle pool · bit 1 a reach keeper ·
//!                                      bit 2 the pool's particles collide · bit 3 its sparks bounce. Handed over with the host's half before every step; a bit that goes down takes the
//!                                      object away again. Belongs in the bridge's host half (`hostHalf` → `io::take_host_half`).
//!   face_wind_out(h)                   STAND-IN: THE WIND'S TWO LISTS OF THE STEP (wind.ts `update`: the sources that blow, and the ones picked for the
//!                                      particles: the jets nearest the view, then the storm). The tick makes both and the every-tick export carries
//!                                      the sources' rows and neither list. At face_out: u32 count and each source's row number (u32), twice: the
//!                                      active list, then the particles'. Belongs in the filtered export (`Wind.active`, `Wind.to_pool` on its step list).
//!                                      Then u32 count of all the sources and for each its row (u32) and its record as the consumers read it, 13 f64:
//!                                      kind (0 a jet, 1 the storm), speed, dir 3, pos 3, reach, tan_spread, r0, d_max, floor_y (NaN none): the
//!                                      step list has a source's `base`, `on`, `gust`, `why` and not the fields of the record it extends
//!                                      (smoke/types.ts WindGpuSource), so the puppet's sources stood at speed 0. Belongs there too.
//!   face_smoke_out(h)                  STAND-IN: WHAT THE BRIDGE LEAVES OUT OF A SMOKE SOURCE (smoke/types.ts SmokeEmitter: its `kind`, its `lattice`
//!                                      and what it rides, `track`; the schema of the `SmokeSystemLikeEmit` event has none of the three, and the
//!                                      solver picks the grid by the first two and moves the source by the third). Called BEFORE face_puppet (which
//!                                      lets the events go). At face_out: u32 count, then for each such event of the step, in order, 16 bytes: kind
//!                                      (u8, the type's order), lattice (u8: 0 none, 1 fine, 2 coarse), track (u8: 0 none, 1 a man's muzzle, 2 his
//!                                      ejection port, 3 a spark), a pad, the man's row or the spark's slot (u32), the spark's generation (f64).
//!                                      Belongs in the event's schema.
//!   face_picture_out(h)                STAND-IN: THE PICTURE'S FRAME OF THE WEATHER AND THE SKY. The tick writes `engine.weather` (render/frame.ts
//!                                      WeatherFrame: the rain, the wet, the lightning's flash, the roofs) and five rows of `engine.settings` (the sky's
//!                                      two colours, the haze, the exposure, the gloss: lighting.ts, hours.ts, weather.ts), and the filtered export's
//!                                      engine list holds the box world and the lights alone, so the puppet's stay as the start left them. At face_out,
//!                                      f64s: 1 or 0 (a weather frame is there), its 28 numbers (rain, snow, shear x z, wet, flash 3, rain_top,
//!                                      interior 4, annex 4, roof_c, roof_d, fog, near_fade 4, phase 4; an Option that is None is NaN), then the sky's
//!                                      zenith 3 and horizon 3, the haze, the exposure, the gloss (1 or 0); and last the box world's geometry
//!                                      serial (`BoxWorld.geom_serial`: moved by every wall or pane the tick opens; on the whole list alone, and the
//!                                      page's tidy of a holed wall's paint and its plan of the openings go by it). Belongs in the filtered export
//!                                      (`HostApi.weather`, `HostApi.settings` or its five rows, and `BoxWorld.geom_serial` on its step list).
//!   face_notes_out(h)                  STAND-IN: WHAT THE TICK NOTED BESIDE THE FRAME'S BOXES (thinline.ts `thinNote` / `thinAir`: which boxes are a
//!                                      laser's beam or a tracer's streak, and a weapon laser's bead, so that the renderer can draw the pair as light
//!                                      in the air and hide the boxes; irlaser.ts `irNote`: which are infrared lines). The notes are module state the
//!                                      bridge does not carry (`g.m.thinline`, `g.m.irlaser`: not-carried.json). At face_out: u32 count, then for each
//!                                      noted box of the frame's list its row (u32), bits (u32: 1 a thin line, 2 a weapon laser's line, 4 infrared),
//!                                      the line's true half-width, its pixels, its three emissions (5 f64) and its bead's row (i32, -1 none).
//!                                      Belongs in the filtered export.
//!   face_emits_ours(h)                 STAND-IN: THE FOG'S MARK OF EACH PARTICLE RECORD OF THE STEP (fogofwar.ts FOG.ours: a record filed between the
//!                                      fog's `begin` and `done` is the team's own doing, and the pool then draws it over floor the team does not see).
//!                                      `OutParticlesEmit` has no such field. At face_out: a byte a record (bit 0 = ours), in the order the step's
//!                                      `ParticlesEmit` events were pushed; the kept marks are emptied here. Belongs in the event.
//!                                      AND THE RECORD'S SHAPE, in the same byte's bits 1 to 3 (0 none, 1 cone, 2 burst, 3 ring, 4 puff, 5 the
//!                                      rain's disc): the event's options have no `shape` in the schema, and the pool reads none as a cone.
//!   face_mem() -> bytes                the module's memory now (a host holds its typed arrays to this: a memory that grew has detached them)
//!
//! A GAME KEPT (a checkpoint, the start a restart goes back to). The host's save controller (src/game/savestate.ts) decides when and under which label; what
//! it keeps is the Rust game's own, held here and named by a number the controller's slot holds:
//!   face_keep(h, how) -> k             how 0: THE GAME'S OWN VALUE (a clone of the whole `Game`, every field the bridge does not carry with it: a closure
//!                                      in flight, the probe's table, the counters) and its block of rust/sim's. Nothing is lost by it, and it lives only
//!                                      as long as this module. how 1: THE BRIDGE'S BYTES (`io::export`) and the block: what a file holds.
//!   face_keep_given(s, n, b, bn) -> k  a kept game from a file's two parts
//!   face_kept_state(k) · face_kept_block(k)   at face_out: the bridge's bytes of kept `k` (a clone is exported now) · its block
//!   face_kept_drop(k) · face_kept_n()  let go · how many are held
//!   face_back(h, k) -> 1 | 0           game `h` IS kept `k` again, in the slot `h` stands in (the kept one stays kept). A clone is cloned back; bytes go
//!                                      through the start's own lines (`game_run::start`: `io::import`, so what the bridge does not carry starts blank).
//!                                      What `face_puppet` last sent is forgotten (memo.rs `forget`): the host applies the whole state next.
//!   face_loaded(h, label, n, loads, saves)   STAND-IN: WHAT THE SAVE CONTROLLER DOES TO THE GAME ONCE A SAVE IN MEMORY IS PUT BACK (savestate.ts loadMem,
//!                                      the lines after the snapshot is restored: :1124 to :1134). A ring of the hand's that merely stood at the save is
//!                                      put away, a gun's setup is not up, the squad's ring is put away, the run's counts of loads and saves are the
//!                                      abandoned moment's (handed in: the kept game holds the saved moment's), the chronicle takes its row, and a
//!                                      tutorial's program is to be started again. After `face_back`, before the next step. Belongs in rust/game's
//!                                      savestate (`checkpoints_load_mem`, an empty body today: the controller is the host's).
//!   face_saves_in(h, p, n)             THE CONTROLLER'S SHELF AS THE TICK READS IT (`g.saves`: Enter once down asks for this run's newest checkpoint,
//!                                      player.ts:493): the run's name and each slot's row. u32 count-prefixed utf-8 for a text; the run, u32 rows, then
//!                                      each: key, slot (u8: 0 auto, 1 manual), n (f64), label, saved_at, mission_t (f64), stage, its run, a byte and
//!                                      the regret's words where the byte is 1.
//!   face_saves_asked(h) -> bits        what the tick asked of the controller, read once: 1 a save in memory is due back (its key at face_out) ·
//!                                      2 the player restarted (`game_restart` ran: the host's camera owes a cut)
use std::cell::RefCell;

/// a game kept: its own value, or the bridge's bytes of it; and its block
enum Kept { Own(Box<game::game::Game>, Vec<u8>), Bytes(Vec<u8>, Vec<u8>) }

thread_local! {
    /// the games kept, by number less one
    static KEPT: RefCell<Vec<Option<Kept>>> = const { RefCell::new(Vec::new()) };
}
fn block_len() -> usize { core::mem::size_of::<sim::state::State>() }
fn block_of(slot: usize) -> Vec<u8> { unsafe { std::slice::from_raw_parts(sim::state::addr(slot) as *const u8, block_len()).to_vec() } }
fn keep(k: Kept) -> u32 {
    KEPT.with(|v| { let mut v = v.borrow_mut(); match v.iter().position(|x| x.is_none()) { Some(i) => { v[i] = Some(k); i as u32 + 1 } None => { v.push(Some(k)); v.len() as u32 } } })
}

/// # Safety
/// `state`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_new_at(state: *const u8, n: usize, slot: usize) -> u32 {
    super::hook();
    if slot >= sim::state::GAMES { return 0; }
    // (the start's own lines take a block as bytes and write it into the slot: handed the slot's own bytes, the block stands as it stood)
    let len = core::mem::size_of::<sim::state::State>();
    let block = std::slice::from_raw_parts(sim::state::addr(slot) as *const u8, len).to_vec();
    let (g, said) = game_run::start(super::bytes(state, n), &block, slot);
    super::out(said.join("\n").into_bytes());
    let h = super::put(super::Held { g, cmd: None, cmds: None, after: Vec::new() });
    crate::memo::forget(h);
    h
}

#[no_mangle]
pub extern "C" fn face_mem() -> usize {
    core::arch::wasm32::memory_size(0) * 65536
}

#[no_mangle]
pub extern "C" fn face_probe_asks(h: u32) {
    let b = super::with(h, |x| {
        let mut o: Vec<u8> = Vec::new();
        o.extend_from_slice(&(x.g.probe.pending.len() as u32).to_le_bytes());
        for a in x.g.probe.pending.iter() {
            o.extend_from_slice(&(a.key.len() as u32).to_le_bytes());
            o.extend_from_slice(a.key.as_bytes());
            for v in a.pos.iter() { o.extend_from_slice(&v.to_le_bytes()); }
            o.extend_from_slice(&a.skip_owner.to_le_bytes());
            match a.b.as_ref() { None => o.push(0), Some(b) => { o.push(1); for v in b.iter() { o.extend_from_slice(&v.to_le_bytes()); } } }
            o.extend_from_slice(&a.rank.to_le_bytes());
        }
        // (…and the keys the tick still holds an answer for: one it forgot inside the tick, `sensors_forget`, is not among them)
        let keys = x.g.probe.results.keys();
        o.extend_from_slice(&(keys.len() as u32).to_le_bytes());
        for k in keys.iter() { o.extend_from_slice(&(k.len() as u32).to_le_bytes()); o.extend_from_slice(k.as_bytes()); }
        o
    });
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_probe_took(h: u32) {
    super::with(h, |x| { let _ = game::sensors::probe_like_take_asks(&mut x.g); });
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_probe_results(h: u32, p: *const u8, n: usize) {
    let b = super::bytes(p, n);
    let u32_at = |at: usize| u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]) as usize;
    let f64_at = |at: usize| f64::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3], b[at + 4], b[at + 5], b[at + 6], b[at + 7]]);
    super::with(h, |x| {
        let mut at = 4;
        for _ in 0..u32_at(0) {
            let len = u32_at(at); at += 4;
            let key = String::from_utf8_lossy(&b[at..at + len]).into_owned(); at += len;
            x.g.probe.results.set(key, [f64_at(at), f64_at(at + 8), f64_at(at + 16)]); at += 24;
        }
    });
}

#[no_mangle]
pub extern "C" fn face_lights_out(h: u32) {
    let b = super::with(h, |x| {
        let g = &x.g;
        let mut o: Vec<u8> = Vec::with_capacity(4 + g.engine.lights.lights.len() * 228);
        o.extend_from_slice(&(g.engine.lights.lights.len() as u32).to_le_bytes());
        let opt = |v: Option<f64>| v.unwrap_or(f64::NAN);
        let bit = |v: bool| if v { 1.0 } else { 0.0 };
        for id in g.engine.lights.lights.iter() {
            use game::ids::Id;
            let l = &g.tab.light[*id];
            o.extend_from_slice(&(id.ix() as u32).to_le_bytes());
            let f = [l.kind, l.pos[0], l.pos[1], l.pos[2], l.dir[0], l.dir[1], l.dir[2], l.color[0], l.color[1], l.color[2], l.intensity, l.range, l.inner_deg, l.outer_deg, l.radius,
                l.volumetric, l.owner, bit(l.enabled), l.fixture_box, l.flicker, opt(l.row), l.ttl, l.age, l.peak_intensity, opt(l.transient.map(bit)), bit(l.broad), opt(l.mark_ref), bit(l.card),
                opt(l.glass_mask), opt(l.fog), opt(l.fog_ghost), opt(l.fog_by)];
            for v in f.iter() { o.extend_from_slice(&v.to_le_bytes()); }
        }
        o
    });
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_poke(h: u32, what: u32, v: f64) -> u32 {
    super::with(h, |x| {
        let g = &mut x.g;
        match what {
            0 => g.god_mode = v != 0.0,
            1 => g.ai_enabled = v != 0.0,
            2 => g.infinite_ammo = v != 0.0,
            3 => g.player_invisible = v != 0.0,
            4 => { let pl = g.player; g.pl[pl].launch_held = None; }
            _ => return 0,
        }
        1
    })
}

#[no_mangle]
pub extern "C" fn face_emits_ours(h: u32) {
    let b = super::with(h, |x| match x.g.engine.particles.as_mut() {
        Some(p) => {
            use game::render_particles::EmitShape as Sh;
            let o: Vec<u8> = p.queue.iter().map(|r| (r.ours as u8) | (match r.o.shape { None => 0u8, Some(Sh::Cone) => 1, Some(Sh::Burst) => 2, Some(Sh::Ring) => 3, Some(Sh::Puff) => 4, Some(Sh::RainDisc) => 5 } << 1)).collect();
            p.queue.clear(); o
        }
        None => Vec::new(),
    });
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_notes_out(h: u32) {
    let b = super::with(h, |x| {
        use game::ids::Id;
        let g = &x.g;
        let (thin, air, ir) = (&g.m.thinline.thin, &g.m.thinline.air, &g.m.irlaser.ir_made);
        let mut o: Vec<u8> = vec![0; 4];
        let mut n = 0u32;
        if thin.is_empty() && air.is_empty() && ir.is_empty() { return o; }
        for id in g.dyn_boxes.iter() {
            let (t, a, i) = (thin.get(*id), air.get(*id), ir.has(*id));
            if t.is_none() && a.is_none() && !i { continue; }
            n += 1;
            o.extend_from_slice(&(id.ix() as u32).to_le_bytes());
            o.extend_from_slice(&((t.is_some() as u32) | ((a.is_some() as u32) << 1) | ((i as u32) << 2)).to_le_bytes());
            let t = t.copied().unwrap_or_default();
            for v in [t.hw0, t.k, t.e[0], t.e[1], t.e[2]] { o.extend_from_slice(&v.to_le_bytes()); }
            let bead: i32 = match a { Some(Some(b)) => b.ix() as i32, _ => -1 };
            o.extend_from_slice(&bead.to_le_bytes());
        }
        o[0..4].copy_from_slice(&n.to_le_bytes());
        o
    });
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_picture_out(h: u32) {
    let b = super::with(h, |x| {
        let e = &x.g.engine;
        let opt = |v: Option<f64>| v.unwrap_or(f64::NAN);
        let mut f: Vec<f64> = Vec::with_capacity(40);
        match e.weather.as_ref() {
            Some(w) => {
                f.push(1.0);
                f.extend_from_slice(&[w.rain, opt(w.snow), w.shear_x, w.shear_z, w.wet, w.flash[0], w.flash[1], w.flash[2], w.rain_top]);
                f.extend_from_slice(&w.interior); f.extend_from_slice(&w.annex);
                f.extend_from_slice(&[opt(w.roof_c), opt(w.roof_d), opt(w.fog)]);
                f.extend_from_slice(&w.near_fade);
                match w.phase { Some(p) => f.extend_from_slice(&p), None => f.extend_from_slice(&[f64::NAN; 4]) }
            }
            None => { f.push(0.0); f.extend_from_slice(&[f64::NAN; 28]); }
        }
        let s = &e.settings;
        f.extend_from_slice(&s.sky_zenith); f.extend_from_slice(&s.sky_horizon);
        f.extend_from_slice(&[s.haze_density, s.exposure, if s.gloss { 1.0 } else { 0.0 }]);
        f.push(e.world.geom_serial);
        let mut o: Vec<u8> = Vec::with_capacity(f.len() * 8);
        for v in f.iter() { o.extend_from_slice(&v.to_le_bytes()); }
        o
    });
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_wind_out(h: u32) {
    let b = super::with(h, |x| {
        use game::ids::Id;
        let w = &x.g.wind;
        let mut o: Vec<u8> = Vec::with_capacity(8 + 4 * (w.active.len() + w.to_pool.list.len()));
        for l in [&w.active, &w.to_pool.list] {
            o.extend_from_slice(&(l.len() as u32).to_le_bytes());
            for id in l.iter() { o.extend_from_slice(&(id.ix() as u32).to_le_bytes()); }
        }
        // every source's record as the two consumers read it (the bridge carries a source's `base`, `on`, `gust` and `why` and not the record they are
        // folded into): its row, then 13 f64
        o.extend_from_slice(&(w.all.len() as u32).to_le_bytes());
        for id in w.all.iter() {
            let s = &x.g.tab.wind_wind_source[*id];
            o.extend_from_slice(&(id.ix() as u32).to_le_bytes());
            let kind = match s.kind { game::wind::WindSourceKind::Jet => 0.0, _ => 1.0 };
            for v in [kind, s.speed, s.dir[0], s.dir[1], s.dir[2], s.pos[0], s.pos[1], s.pos[2], s.reach, s.tan_spread, s.r0, s.d_max, s.floor_y.unwrap_or(f64::NAN)] {
                o.extend_from_slice(&v.to_le_bytes());
            }
        }
        o
    });
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_smoke_out(h: u32) {
    let b = super::with(h, |x| {
        use game::ids::Id;
        use game::effects::Track;
        use game::smoke_types::{SmokeEmitterKind as K, SmokeEmitterLattice as L};
        let mut o: Vec<u8> = vec![0; 4];
        let mut n = 0u32;
        for e in x.g.out.iter() {
            let game::out_events::OutEvent::SmokeSystemLikeEmit(ev) = e else { continue };
            let e = &ev.e;
            n += 1;
            let kind: u8 = match e.kind { K::Flash => 0, K::Wisp => 1, K::Canister => 2, K::Burst => 3, K::Cloud => 4, K::Jet => 5, K::Trail => 6, K::Vent => 7, K::Puff => 8, K::Breath => 9 };
            let lat: u8 = match e.lattice { None => 0, Some(L::Fine) => 1, Some(L::Coarse) => 2 };
            let (t, a, gen): (u8, u32, f64) = match e.track {
                None => (0, 0, 0.0),
                Some(Track::Muzzle { by }) => (1, by.ix() as u32, 0.0),
                Some(Track::EjectPort { by }) => (2, by.ix() as u32, 0.0),
                Some(Track::Spark { slot, gen_ }) => (3, slot as u32, gen_),
            };
            o.extend_from_slice(&[kind, lat, t, 0]);
            o.extend_from_slice(&a.to_le_bytes());
            o.extend_from_slice(&gen.to_le_bytes());
        }
        o[0..4].copy_from_slice(&n.to_le_bytes());
        o
    });
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_host_has(h: u32, bits: u32) {
    super::with(h, |x| {
        let e = &mut x.g.engine;
        if bits & 1 != 0 {
            let p = e.particles.get_or_insert_with(game::render_particles::Particles::blank);
            p.collide = bits & 4 != 0; p.spark_bounce = bits & 8 != 0;
        } else { e.particles = None; }
        if bits & 2 != 0 { if e.reach.is_none() { e.reach = Some(game::render_reach_keep::ReachKeep::blank()); } } else { e.reach = None; }
    })
}

// ---- a game kept
#[no_mangle]
pub extern "C" fn face_keep(h: u32, how: u32) -> u32 {
    let k = super::with(h, |x| {
        let block = block_of(x.g.sim);
        if how == 0 { let mut g = x.g.clone(); g.out.clear(); Kept::Own(g, block) } else { Kept::Bytes(game::io::export(&x.g), block) }
    });
    keep(k)
}
/// # Safety
/// the two pointers name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_keep_given(state: *const u8, n: usize, block: *const u8, bn: usize) -> u32 {
    super::hook();
    keep(Kept::Bytes(super::bytes(state, n).to_vec(), super::bytes(block, bn).to_vec()))
}
#[no_mangle]
pub extern "C" fn face_kept_state(k: u32) {
    let b = KEPT.with(|v| match v.borrow().get((k as usize).wrapping_sub(1)).and_then(|x| x.as_ref()) { Some(Kept::Own(g, _)) => game::io::export(g), Some(Kept::Bytes(s, _)) => s.clone(), None => Vec::new() });
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_kept_block(k: u32) {
    let b = KEPT.with(|v| match v.borrow().get((k as usize).wrapping_sub(1)).and_then(|x| x.as_ref()) { Some(Kept::Own(_, b)) | Some(Kept::Bytes(_, b)) => b.clone(), None => Vec::new() });
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_kept_drop(k: u32) {
    KEPT.with(|v| if let Some(x) = v.borrow_mut().get_mut((k as usize).wrapping_sub(1)) { *x = None; });
}
#[no_mangle]
pub extern "C" fn face_kept_n() -> usize {
    KEPT.with(|v| v.borrow().iter().flatten().count())
}
#[no_mangle]
pub extern "C" fn face_back(h: u32, k: u32) -> u32 {
    let slot = super::with(h, |x| x.g.sim);
    let made: Option<Box<game::game::Game>> = KEPT.with(|v| match v.borrow().get((k as usize).wrapping_sub(1)).and_then(|x| x.as_ref()) {
        Some(Kept::Own(g, block)) => {
            unsafe { core::ptr::copy_nonoverlapping(block.as_ptr(), sim::state::addr(slot) as *mut u8, block.len().min(block_len())); }
            let mut g = g.clone();
            game::io::set_slot(&mut g, slot);
            // (the box world's copy in rust/sim stands outside the block and is the abandoned moment's: written whole again as the next tick opens, as the
            //  TypeScript's is after a snapshot is put back: boxMirror.ts `putBacks`)
            g.col.mirror.put_backs = -1.0;
            Some(g)
        }
        Some(Kept::Bytes(state, block)) => Some(game_run::start(state, block, slot).0),
        None => None,
    });
    let Some(g) = made else { return 0 };
    super::with(h, |x| { x.g = g; x.cmd = None; x.cmds = None; x.after.clear(); });
    crate::memo::forget(h);
    1
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_saves_in(h: u32, p: *const u8, n: usize) {
    use game::savestate::{MemSave, SlotInfo, SlotInfoKind, SlotInfoSlot};
    let b = super::bytes(p, n);
    let mut at = 0usize;
    let mut u32_ = |at: &mut usize| { let v = u32::from_le_bytes([b[*at], b[*at + 1], b[*at + 2], b[*at + 3]]) as usize; *at += 4; v };
    let f64_ = |at: &mut usize| { let mut w = [0u8; 8]; w.copy_from_slice(&b[*at..*at + 8]); *at += 8; f64::from_le_bytes(w) };
    let mut text = |at: &mut usize| { let len = u32_(at); let s = String::from_utf8_lossy(&b[*at..*at + len]).into_owned(); *at += len; s };
    let run = text(&mut at);
    let rows = { let v = u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]) as usize; at += 4; v };
    let mut list: Vec<SlotInfo> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let key = text(&mut at);
        let manual = b[at] != 0; at += 1;
        let nn = f64_(&mut at);
        let label = text(&mut at);
        let saved_at = f64_(&mut at);
        let mission_t = f64_(&mut at);
        let stage = text(&mut at);
        let run_id = text(&mut at);
        let regret = { let has = b[at] != 0; at += 1; if has { Some(text(&mut at)) } else { None } };
        list.push(SlotInfo { key, slot: if manual { SlotInfoSlot::Manual } else { SlotInfoSlot::Auto }, n: nn, label, kind: if manual { SlotInfoKind::Manual } else { SlotInfoKind::Auto },
            saved_at, mission_t, stage, run_id, bytes: 0.0, regret, mem: Some(true) });
    }
    super::with(h, |x| {
        let s = &mut x.g.saves;
        s.run_id = run;
        s.shelf.m.clear();
        for info in list { s.shelf.m.set(info.key.clone(), MemSave { info, pic: None, take: 0.0 }); }
    });
}
#[no_mangle]
pub extern "C" fn face_saves_asked(h: u32) -> u32 {
    let (bits, key) = super::with(h, |x| {
        let g = &mut x.g;
        let due = g.saves.due.take();
        let cut = core::mem::replace(&mut g.restart_cut, false);
        ((due.is_some() as u32) | ((cut as u32) << 1), due.unwrap_or_default())
    });
    super::out(key.into_bytes());
    bits
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_loaded(h: u32, p: *const u8, n: usize, loads: f64, saves: f64) {
    let label = super::text(p, n);
    super::with(h, |x| {
        let g = &mut x.g;
        let pl = g.player;
        // (handring.ts: a ring that merely stood at the save is nothing of the saved moment)
        if g.pl[pl].sel.ring.as_ref().is_some_and(|r| r.free == Some(true)) {
            let s = &mut g.pl[pl].sel;
            s.open = false; s.ring = None; s.down_t = -1.0; s.moved = false;
            g.pl[pl].near_sel = 0;
        }
        g.pl[pl].setup = None;   // (slotsetup.ts: a gun's setup that was up at the save is not up after the load)
        game::orders::squad_orders_loaded(g);
        g.chronicle.counts.loads = loads;
        g.chronicle.counts.saves = saves;
        let _ = game::chronicle::chron_checkpoint(g, game::chronicle::ChronCheckpointWhat::Load, Some(&label));
        game::rails::rails_put_back(g);
    });
}
