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
//!   face_reserve(bytes)                room asked for and given back, so that the allocator holds it and a step need not grow the memory
//!   face_mem() -> bytes                the module's memory now (a host holds its typed arrays to this: a memory that grew has detached them)
use std::cell::RefCell;

thread_local! {
    /// what `face_puppet` last sent, a game: by handle
    static MEMOS: RefCell<Vec<game::puppet::PuppetMemo>> = const { RefCell::new(Vec::new()) };
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
    MEMOS.with(|m| { let mut m = m.borrow_mut(); while m.len() <= h as usize { m.push(Default::default()); } m[h as usize] = Default::default(); });
    h
}

#[no_mangle]
pub extern "C" fn face_puppet(h: u32, start: u32) {
    let mut memo = MEMOS.with(|m| { let mut m = m.borrow_mut(); while m.len() <= h as usize { m.push(Default::default()); } core::mem::take(&mut m[h as usize]) });
    let b = super::with(h, |x| { let b = game::puppet::export_puppet(&x.g, &mut memo, start != 0); x.g.out.clear(); b });
    MEMOS.with(|m| m.borrow_mut()[h as usize] = memo);
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_reserve(bytes: usize) {
    drop(Vec::<u8>::with_capacity(bytes.max(1)));
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
