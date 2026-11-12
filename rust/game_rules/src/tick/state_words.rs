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
        o.b.clear(); for (_, c) in g.ch.iter() { for x in &c.poses.d { x.w(o); } if let Some(r) = &c.rd { for x in &r.pose { x.w(o); } } } out.push(word(&o.b));
        o.b.clear(); for (_, c) in g.ch.iter() { let h = c.hold; (h.by.0, h.rank, h.reason, h.since).w(o); } out.push(word(&o.b));
        o.b.clear(); { let d = &g.air_dust; (d.on as u8 as f64).w(o); d.leak.w(o); d.acc.w(o); (d.cur as f64).w(o); for i in 0..::sim::air::N { d.share.get(i).copied().unwrap_or(1.0).w(o); } for i in 0..2 * ::sim::air::N { d.open.get(i).copied().unwrap_or(0).w(o); } } out.push(word(&o.b));
        o.b.clear(); for l in &g.light_home.rows { for v in [l.pos[0], l.pos[1], l.pos[2], l.range, l.radius, l.kind, l.dir[0], l.dir[1], l.dir[2], l.outer, l.inner, l.color[0], l.color[1], l.color[2], l.intensity, l.base] { v.w(o); } (l.on as u32, l.leave, l.day, l.target.0).w(o); (l.feed as u32).w(o); let s = l.steady.unwrap_or_default(); for v in [s.pos[0], s.pos[1], s.pos[2], s.radius, s.intensity] { v.w(o); } (l.steady.is_some() as u32).w(o); } out.push(word(&o.b));
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
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    {
        let s = &g.smoke;
        for v in [s.wide as f64, s.deep as f64, s.steps as f64, s.out, s.given, s.clock] { eat(&mut h, v.to_bits()); }
        eat(&mut h, s.emitters.len() as u64);
        for e in &s.emitters { for v in [e.at[0], e.at[1], e.at[2], e.rate, e.ttl, e.age, e.radius, e.dye[0], e.dye[1], e.dye[2]] { eat(&mut h, v.to_bits()); } }
        let top = s.bricks.iter().rposition(|b| b.at.is_some()).map_or(0, |b| b + 1);
        eat(&mut h, top as u64);
        for b in 0..top {
            let at = s.bricks[b].at.map_or([0, 0], |(x, z)| [x as u64 + 1, z as u64 + 1]);
            for v in at { eat(&mut h, v); }
            if at[0] == 0 { continue; }
            for v in s.bricks[b].dye { eat(&mut h, v.to_bits()); }
            for c in 0..128 { eat(&mut h, s.rho[b * 128 + c].to_bits()); }
        }
        let l = &s.glow;
        for v in [l.lamps, l.at, l.left] { eat(&mut h, v.to_bits()); }
        let metres = (s.wide * s.deep) as usize * 4;
        for c in [&l.best, &l.mean, &l.all, &l.up, &l.place] { for m in 0..metres { eat(&mut h, c.get(m).copied().unwrap_or(0.0).to_bits()); } }
    }
    let mut k = 0xcbf2_9ce4_8422_2325u64;
    let p = &g.tab.boxes.world().panes;
    eat(&mut k, p.rows.len() as u64);
    for (x, going) in p.rows.iter().zip(&p.going) {
        for v in x.d.iter().chain(&x.e).chain(&x.hits).chain(&x.c).chain(&[x.cs, x.sn, x.hu, x.hv, x.ht, x.energy, x.damage, x.next_think]).chain(&x.last_dir).chain(&x.last_uv).chain(&[x.sound_at, x.who]).chain(&x.bk) { eat(&mut k, v.to_bits()); }
        for v in x.w.iter().chain(&[x.key, x.ty, x.u_axis, x.exterior, x.hit_n, x.n_open, x.d_next, x.walkable, x.bk_at, going.len() as u32]) { eat(&mut k, *v as u64); }
        for q in going {
            for v in [q.at, q.from[0], q.from[1], q.from[2], q.k] { eat(&mut k, v.to_bits()); }
            eat(&mut k, q.fid as u64 | (q.cause as u64) << 32);
        }
    }
    eat(&mut k, p.holes.len() as u64);
    for x in &p.holes { eat(&mut k, x.bi as u64); eat(&mut k, x.at.len() as u64); for v in x.at.iter().flatten() { eat(&mut k, v.to_bits()); } }
    for b in p.bars.iter().chain(&p.kinds) { eat(&mut k, b[0] as u64); eat(&mut k, b[1] as u64); }
    for v in p.told.iter().flatten() { eat(&mut k, v.to_bits()); }
    for c in &p.charges { for v in [c.t, c.at[0], c.at[1], c.at[2], c.seq] { eat(&mut k, v.to_bits()); } }
    for v in [p.seq, p.released, p.gust as u32] { eat(&mut k, v as u64); }
    [(h ^ (h >> 32)) as u32, (k ^ (k >> 32)) as u32]
}
