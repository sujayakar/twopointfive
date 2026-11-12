//! THE STATE'S WORDS: a hash of the whole game, a word a part, for two machines of one game to be held to each other. A part is a field of the game (of its modules, its
//! tables, a seat's set, a group); what is in no word says so beside itself (`#[bytes(skip)]`): what a host hands in, a step's output, the picture's.
use crate::bytes::{Parts, W};
use crate::game::{Game, Mods, Tabs};
use crate::host_api::HostApi;
use crate::group::Group;
use crate::seats::SeatSet;
use alloc::{format, string::String, vec, vec::Vec};

/// the game's own parts: those that are walked by their own fields, and a seat's set's (which lies in its seat's row), stand elsewhere in the list
fn own(name: &str) -> bool { !matches!(name, "m" | "engine" | "tab" | "sets" | "groups") && !SeatSet::NAMES.contains(&name) }

/// each word's part, in the words' order
pub fn state_names() -> Vec<String> {
    let of = |at: &str, names: &[&str]| names.iter().map(|n| format!("{at}{n}")).collect::<Vec<_>>();
    let mut v: Vec<String> = Game::NAMES.iter().filter(|n| own(n)).map(|n| String::from(*n)).collect();
    v.extend(of("m.", Mods::NAMES));
    v.extend(of("engine.", HostApi::NAMES));
    v.extend(of("tab.", Tabs::NAMES));
    v.extend(of("seats.", SeatSet::NAMES));
    v.extend(of("groups.", Group::NAMES));
    for x in ["x.poses", "x.held", "x.dust", "x.fed", "x.smoke", "x.panes"] { v.push(String::from(x)); }
    v
}

/// `o` and `out` are the caller's to keep, and reach their size once. With the first seat standing, as inside a tick: which seat a machine shows moves no word
pub fn state_words(g: &mut Game, o: &mut W, out: &mut Vec<u32>) {
    out.clear();
    crate::seats::on_first_bare(g, &mut |g| {
        g.sets[g.seat].player = g.player;
        let mut say = |x: &dyn crate::bytes::ToBytes| { o.b.clear(); x.w(o); word(&o.b) };
        g.parts(&mut |name, x| if own(name) { out.push(say(x)); });
        g.m.parts(&mut |_, x| out.push(say(x)));
        g.engine.parts(&mut |_, x| out.push(say(x)));
        g.tab.parts(&mut |_, x| out.push(say(x)));
        // (a part of the sets is one word, whoever many seats there are)
        let at = out.len();
        out.resize(at + SeatSet::NAMES.len(), 0);
        for (_, set) in g.sets.iter() {
            let mut k = at;
            set.parts(&mut |_, x| { out[k] = out[k].rotate_left(5) ^ say(x); k += 1; });
        }
        // (and so is a part of the groups)
        let at = out.len();
        out.resize(at + Group::NAMES.len(), 0);
        for (_, group) in g.groups.iter() {
            let mut k = at;
            group.parts(&mut |_, x| { out[k] = out[k].rotate_left(5) ^ say(x); k += 1; });
        }
        o.b.clear();
        use crate::bytes::ToBytes as _;
        o.b.clear(); for (_, c) in g.ch.iter() { let mut l = g.tab.pose_set[c.anim.poses].list.clone(); if let Some(r) = &c.rd { l.push(r.pose); } for p in l { for x in crate::anim_skeleton::cols_of_read(g, p) { x.w(o); } } } out.push(word(&o.b));
        o.b.clear(); for c in g.ch.ids() { use crate::sim_men_store as m; (m::man_owner(g, c), m::man_rank(g, c), m::man_reason(g, c), m::man_since(g, c)).w(o); } out.push(word(&o.b));
        o.b.clear(); { let d = &::sim::weather::of_read(g.sim).air; d.on.w(o); d.leak.w(o); d.acc.w(o); d.cur.w(o); for v in d.s { v.w(o); } for v in d.stat { v.w(o); } } out.push(word(&o.b));
        o.b.clear(); { let l = &::sim::state::state_read(g.sim).lights; use ::sim::lights::*; for r in 0..l.rows as usize { for v in [l.x[r], l.y[r], l.z[r], l.range[r], l.radius[r], l.kind[r], l.dx[r], l.dy[r], l.dz[r], l.outer[r], l.inner[r], l.r[r], l.g[r], l.b[r], l.intensity[r], l.row_base[r]] { v.w(o); } let b = l.row_bits[r]; (b & R_ON, b & (R_PASSING | R_SCREEN), b >> R_DAY & 7, l.row_target[r]).w(o); ((b & R_ROLE) >> 3).w(o); for v in [l.sx[r], l.sy[r], l.sz[r], l.s_radius[r], l.s_intensity[r]] { v.w(o); } (b >> 5 & 1).w(o); } } out.push(word(&o.b));
        out.extend(moved(g));
    });
}

/// eight bytes at a time, how many there are first
fn word(b: &[u8]) -> u32 {
    let mut h: u64 = 0xcbf29ce484222325 ^ b.len() as u64;
    let mut eights = b.chunks_exact(8);
    for x in &mut eights {
        h = (h ^ u64::from_le_bytes(x.try_into().unwrap())).wrapping_mul(0x100000001b3);
        h ^= h >> 29;
    }
    for x in eights.remainder() { h = (h ^ *x as u64).wrapping_mul(0x100000001b3); }
    (h ^ (h >> 32)) as u32
}

/// (for a hold alone: the parts that move, said the same way before and after)
fn eat(h: &mut u64, v: u64) { *h = (*h ^ v).wrapping_mul(0x0000_0100_0000_01b3); *h ^= *h >> 29; }
pub fn moved(g: &Game) -> [u32; 2] {
    let st = ::sim::state::state_read(g.sim);
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    {
        use ::sim::bricks::*;
        let s = &st.smoke.st;
        for i in [2, 3, 1, 4, 5, 6] { eat(&mut h, s[i].to_bits()); }
        let n = s[0] as usize;
        eat(&mut h, n as u64);
        for v in &s[O_EMIT..O_EMIT + n * EMIT] { eat(&mut h, v.to_bits()); }
        let top = (0..MAX_BRICKS).rev().find(|b| s[O_KEYS + b * 2] != 0.0).map_or(0, |b| b + 1);
        eat(&mut h, top as u64);
        for b in 0..top {
            for i in 0..2 { eat(&mut h, s[O_KEYS + b * 2 + i] as u64); }
            if s[O_KEYS + b * 2] == 0.0 { continue; }
            for i in 0..3 { eat(&mut h, s[O_DYE + b * 3 + i].to_bits()); }
            for c in 0..CELLS { eat(&mut h, s[O_RHO + b * CELLS + c].to_bits()); }
        }
        for o in [O_GLOW_LAMPS, O_GLOW_AT, O_GLOW_LEFT] { eat(&mut h, s[o].to_bits()); }
        let metres = (s[2] * s[3]) as usize * 4;
        for o in [O_GLOW_MAX, O_GLOW_MEAN, O_GLOW_A, O_GLOW_UP, O_GLOW_PLACE] { for m in 0..metres { eat(&mut h, s[o + m].to_bits()); } }
    }
    let mut k = 0xcbf2_9ce4_8422_2325u64;
    let p = &st.panes;
    let used = p.used != 0;
    let n = if used { p.n as usize } else { 0 };
    eat(&mut k, n as u64);
    // (the queue lies after the holes in the body, a piece seven words: rows 0.., holes, queue)
    let row_w = core::mem::size_of::<::sim::panes::Row>() / 8;
    let hole_w = core::mem::size_of::<::sim::panes::Hole>() / 8;
    let q_at = n * row_w + (if used { p.nh as usize } else { 0 }) * hole_w;
    for r in 0..n {
        let x = p.row_read(r);
        for v in x.d.iter().chain(&x.e).chain(&x.hits).chain(&x.c).chain(&[x.cs, x.sn, x.hu, x.hv, x.ht, x.energy, x.damage, x.next_think]).chain(&x.last_dir).chain(&x.last_uv).chain(&[x.sound_at, x.who]).chain(&x.bk) { eat(&mut k, v.to_bits()); }
        for v in x.w.iter().chain(&[x.key, x.ty, x.u_axis, x.exterior, x.hit_n, x.n_open, x.d_next, x.walkable, x.bk_at, x.qn]) { eat(&mut k, *v as u64); }
        for q in 0..x.qn as usize {
            let at = q_at + (x.q0 as usize + q) * 6;
            for i in 0..6 { eat(&mut k, p.body[at + i]); }
        }
    }
    let nh = if used { p.nh as usize } else { 0 };
    eat(&mut k, nh as u64);
    for i in 0..nh { let x = p.hole_read(i); eat(&mut k, x.bi as u64); eat(&mut k, x.n as u64); for v in &x.p[..3 * x.n as usize] { eat(&mut k, v.to_bits()); } }
    let c = |v: u32| if used { v as usize } else { 0 };
    for b in &p.bars[..c(p.nbars)] { eat(&mut k, b[0] as u64); eat(&mut k, b[1] as u64); }
    for b in &p.overs[..c(p.nover)] { eat(&mut k, b[0] as u64); eat(&mut k, b[1] as u64); }
    for v in &p.ev[..2 * c(p.nev)] { eat(&mut k, v.to_bits()); }
    for v in &p.bl[..5 * c(p.nbl)] { eat(&mut k, v.to_bits()); }
    for v in [p.seq, p.released, p.flags] { eat(&mut k, c(v) as u64); }
    [(h ^ (h >> 32)) as u32, (k ^ (k >> 32)) as u32]
}
