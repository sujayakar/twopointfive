//! THE CLIPS A POSE IS SAMPLED FROM (src/anim/gltf.ts Clip, Track; src/anim/skeleton.ts sampleClip) — a character file's animations, copied in ONCE as the
//! host first samples one, so that a sample is one question whose answer is written straight into a pose (the bench's, or one at home in a game's pool:
//! skeleton.rs `ask_sample`), and so that a layer of the animator written here samples with no crossing.
//!
//! AUTHORED DATA, NOT STATE: nothing writes a track after its file is loaded, a clip is the same for every game, and no snapshot carries it. So this is
//! outside the games' blocks (state.rs) and has no part: a COPY (tools/sim/build.ts STATICS), held by tools/qa/pose-ident.ts, which reads every single of
//! every clip back and samples each both ways. A clip is known by the number `end` gave it; the host keeps which clip that is.
//!
//! A CLIP WRITTEN TWICE IS ONE CLIP. The host knows a clip by its object and a character file may be loaded more than once in a process (a game stood up
//! again), so `end` looks for a clip already held with the same length, the same tracks and the same singles (found by a word folded from them, then compared
//! whole) and gives ITS number, keeping nothing of the second writing. The table grows with the clips there are, not with how often they are loaded.
//! To be found again a clip has to be written again, so the table always KEEPS ROOM FOR ITS LARGEST CLIP: a new clip that would leave less is refused
//! (`end`), and a full table still knows every clip it holds.
//!
//! A TRACK is one node's translation, rotation or scale over time: `keys` times, then `keys × comps` values (3, or 4 for a rotation), singles as the
//! file's Float32Arrays hold them (single.rs), one after another in `pool`.
//!
//! WHERE A SAMPLE'S TIME FALLS AMONG A TRACK'S KEYS depends on the track's times alone, and a clip's tracks share theirs (the game's rig: 159 tracks a clip
//! on two sets of times). So `end` gives every track the number of its LINE — the first track of the clip with the same times, single for single — and a
//! sample finds its place on a line once (`Place`), where the TypeScript searches each track: the same place, so the same numbers.
use crate::single::{get, put, S};
use crate::skeleton::{store_unit, MAX_NODES, NODE};
use crate::world::{js_max, js_min};

/// clips the table holds (a character file has some fifty)
pub const CLIPS: usize = 256;
/// tracks, over every clip (the game's rig: 7,314)
pub const TRACKS: usize = 1 << 15;
/// singles of times and values, over every track (the game's rig: 310,527)
pub const POOL: usize = 1 << 20;

#[derive(Clone, Copy)]
pub struct Clip {
    pub duration: f64,
    /// its tracks: `tracks` of them from `track0`
    pub track0: u32,
    pub tracks: u32,
    /// one more than the highest node a track of it writes: a pose of fewer nodes is not asked
    pub nodes: u32,
    /// its length, its tracks and every single of them, folded: what a clip written again is looked up by (then compared whole)
    pub word: u32,
}
#[derive(Clone, Copy)]
pub struct Track {
    /// where its times start in `pool`; its values follow them
    pub at: u32,
    pub keys: u32,
    /// where in a pose its values go, as ONE NUMBER OF THE CLIP'S OWN (what a clip is known by): node × NODE + its first channel (0 translation ·
    /// 3 rotation · 7 scale) …
    pub out: u32,
    /// … and as a sample needs it: the node, and that first channel. The place in a pose is `chan * rows + node`, the pose's own rows (skeleton.rs)
    pub node: u16,
    pub chan: u16,
    /// 3, or 4 for a rotation
    pub comps: u32,
    /// the value of the key before, not the line between two
    pub step: bool,
    /// which set of times of its clip these are (0, 1, … in the order first met; LINES and up = one of its own, searched by itself)
    pub line: u32,
}
/// the sets of times of one clip a sample keeps its place on
pub const LINES: usize = 8;
/// where a time falls on a line: before its first key (or it has one key) · at or past its last · between key `lo` and the next, `f` of the way
#[derive(Clone, Copy)]
struct Place {
    how: u32,
    lo: usize,
    f: f64,
}
const UNSET: u32 = 0;
const FIRST: u32 = 1;
const LAST: u32 = 2;
const BETWEEN: u32 = 3;
/// skeleton.ts sampleClip's own steps for one track's times: the ends, the search, the fraction
#[inline(always)]
fn place(pool: &[S], times: usize, n: usize, t: f64) -> Place {
    if n == 1 || t <= get(pool, times) {
        return Place { how: FIRST, lo: 0, f: 0.0 };
    }
    if t >= get(pool, times + n - 1) {
        return Place { how: LAST, lo: n - 1, f: 0.0 };
    }
    // the key at or before t, and the one after
    let (mut lo, mut hi) = (0usize, n - 1);
    while hi - lo > 1 {
        let mid = (lo + hi) >> 1;
        if get(pool, times + mid) <= t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Place { how: BETWEEN, lo, f: (t - get(pool, times + lo)) / js_max(1e-6, get(pool, times + hi) - get(pool, times + lo)) }
}
pub struct Table {
    /// (one more than it holds: the slot a clip is written into before it is known to be new)
    pub clips: [Clip; CLIPS + 1],
    pub tracks: [Track; TRACKS],
    pub pool: [S; POOL],
    pub n_clips: usize,
    pub n_tracks: usize,
    pub used: usize,
    /// the most tracks and the most singles a held clip has: the room always kept free, so that any of them can be written again and found
    pub most_tracks: usize,
    pub most_singles: usize,
    /// a clip is being written (into the slot after the last held, and the pool from `from`): the tracks it still owes (`begin` … `end`), and whether
    /// one was refused
    pub from: usize,
    pub owed: usize,
    pub open: bool,
    pub bad: bool,
}
static mut ALL: Table = Table {
    clips: [Clip { duration: 0.0, track0: 0, tracks: 0, nodes: 0, word: 0 }; CLIPS + 1],
    tracks: [Track { at: 0, keys: 0, out: 0, node: 0, chan: 0, comps: 0, step: false, line: 0 }; TRACKS],
    pool: [S::ZERO; POOL],
    n_clips: 0,
    n_tracks: 0,
    used: 0,
    most_tracks: 0,
    most_singles: 0,
    from: 0,
    owed: 0,
    open: false,
    bad: false,
};
#[inline(always)]
pub fn table() -> &'static mut Table {
    unsafe { &mut *core::ptr::addr_of_mut!(ALL) }
}

/// a clip of `tracks` tracks is about to be written: false = no room to write it (the host samples that clip itself). A clip left half written (its `end`
/// never came) is dropped here: the table is never held shut by one.
pub fn begin(duration: f64, tracks: usize) -> bool {
    let t = table();
    if t.open {
        t.used = t.from;
    }
    t.open = false;
    if tracks > TRACKS - t.n_tracks {
        return false;
    }
    t.clips[t.n_clips] = Clip { duration, track0: t.n_tracks as u32, tracks: 0, nodes: 0, word: 0 };
    t.from = t.used;
    t.owed = tracks;
    t.open = true;
    t.bad = false;
    true
}
/// the next track of the clip being written: where the host writes its `keys` times (an index into the module's singles; its values follow them) —
/// `u32::MAX` = not a track this table takes (no clip open, a node no pose has, no key, no room): the clip is then refused whole by `end`
pub fn track(node: usize, path: u32, step: bool, keys: usize) -> u32 {
    let t = table();
    let comps = if path == 1 { 4 } else { 3 };
    if !t.open || t.owed == 0 || node >= MAX_NODES || path > 2 || keys == 0 || keys > (POOL - t.used) / (1 + comps) {
        t.bad = true;
        return u32::MAX;
    }
    let c = &mut t.clips[t.n_clips];
    let k = (c.track0 + c.tracks) as usize;
    let chan = if path == 0 { 0 } else if path == 1 { 3 } else { 7 };
    t.tracks[k] = Track { at: t.used as u32, keys: keys as u32, out: (node * NODE + chan) as u32, node: node as u16, chan: chan as u16, comps: comps as u32, step, line: LINES as u32 };
    c.tracks += 1;
    if node as u32 + 1 > c.nodes {
        c.nodes = node as u32 + 1;
    }
    let at = t.used;
    t.used += keys * (1 + comps);
    t.owed -= 1;
    ((t.pool.as_ptr() as usize) / 4 + at) as u32
}
/// the clip being written is whole: its number (that of the clip already held, if this one was written before) — or −1, and nothing of it is kept (a track
/// was refused, one is missing, it has none, or it is new and the table has no room for it beside the room it keeps)
pub fn end() -> i32 {
    let t = table();
    if !t.open {
        return -1;
    }
    t.open = false;
    let c = t.clips[t.n_clips];
    let (from, to) = (t.from, t.used);
    t.used = from;
    if t.bad || t.owed != 0 || c.tracks == 0 {
        return -1;
    }
    let (k0, k1) = (c.track0 as usize, (c.track0 + c.tracks) as usize);
    // written before? Then that clip's number, and nothing of this writing is kept
    let word = fold(t, &c, from, to);
    for e in 0..t.n_clips {
        if t.clips[e].word == word && alike(t, &t.clips[e], &c, to - from) {
            return e as i32;
        }
    }
    // new: taken only if the largest clip held (this one counted) could still be written again after it
    let (most_tracks, most_singles) = (t.most_tracks.max(k1 - k0), t.most_singles.max(to - from));
    if t.n_clips >= CLIPS || TRACKS - k1 < most_tracks || POOL - to < most_singles {
        return -1;
    }
    t.most_tracks = most_tracks;
    t.most_singles = most_singles;
    t.clips[t.n_clips].word = word;
    // the lines: a track whose times are those of the first track of a line, single for single, is on that line
    let mut first = [0usize; LINES];
    let mut lines = 0usize;
    for k in k0..k1 {
        let (at, n) = (t.tracks[k].at as usize, t.tracks[k].keys as usize);
        let mut line = LINES;
        for l in 0..lines {
            let o = t.tracks[first[l]];
            if o.keys as usize == n && (0..n).all(|i| get(&t.pool, o.at as usize + i).to_bits() == get(&t.pool, at + i).to_bits()) {
                line = l;
                break;
            }
        }
        if line == LINES && lines < LINES {
            first[lines] = k;
            line = lines;
            lines += 1;
        }
        t.tracks[k].line = line as u32;
    }
    t.used = to;
    t.n_tracks = k1;
    t.n_clips += 1;
    (t.n_clips - 1) as i32
}
/// a stored single as a word (the double it stands for, its two halves folded: singles alike give words alike)
#[inline(always)]
fn bits(pool: &[S], i: usize) -> u32 {
    let b = get(pool, i).to_bits();
    b as u32 ^ (b >> 32) as u32
}
/// FNV-1a over a clip: its length, each track's shape, and the singles `from..to` of the pool
fn fold(t: &Table, c: &Clip, from: usize, to: usize) -> u32 {
    let mut h = 0x811c_9dc5u32;
    let mut eat = |w: u32| h = (h ^ w).wrapping_mul(0x0100_0193);
    let d = c.duration.to_bits();
    eat(d as u32);
    eat((d >> 32) as u32);
    for k in c.track0 as usize..(c.track0 + c.tracks) as usize {
        let tr = &t.tracks[k];
        eat(tr.keys);
        eat(tr.out);
        eat(tr.comps | (tr.step as u32) << 8);
    }
    for i in from..to {
        eat(bits(&t.pool, i));
    }
    h
}
/// are clip `a` (held) and clip `b` (just written, `len` singles) the same clip: length, tracks and every single
fn alike(t: &Table, a: &Clip, b: &Clip, len: usize) -> bool {
    if a.duration.to_bits() != b.duration.to_bits() || a.tracks != b.tracks {
        return false;
    }
    // (tracks alike in keys and values a key lie alike in the pool, one after another: comparing the singles in order compares track with track)
    for k in 0..a.tracks as usize {
        let (x, y) = (&t.tracks[a.track0 as usize + k], &t.tracks[b.track0 as usize + k]);
        if x.keys != y.keys || x.out != y.out || x.comps != y.comps || x.step != y.step {
            return false;
        }
    }
    let (fa, fb) = (t.tracks[a.track0 as usize].at as usize, t.tracks[b.track0 as usize].at as usize);
    (0..len).all(|i| get(&t.pool, fa + i).to_bits() == get(&t.pool, fb + i).to_bits())
}
/// one more than the highest node clip `c` writes (0 = no such clip)
pub fn nodes_of(c: usize) -> usize {
    let t = table();
    if c < t.n_clips {
        t.clips[c].nodes as usize
    } else {
        0
    }
}

/// skeleton.ts sampleClip: clip `c` at time `t` (seconds; wrapped where `looped`, else held inside the clip) into the pose `d` — only the nodes the clip
/// has tracks for are written. The caller has made sure `d` holds `nodes_of(c)` nodes; a number the table does not hold writes nothing.
pub fn sample(d: &mut [S], c: usize, mut t: f64, looped: bool) {
    let tb = table();
    if c >= tb.n_clips {
        return;
    }
    let cl = tb.clips[c];
    let dur = cl.duration;
    if looped {
        t = t % dur;
        if t < 0.0 {
            t += dur;
        }
    } else {
        t = js_min(js_max(t, 0.0), dur);
    }
    let pool = &tb.pool[..];
    let rows = d.len() / NODE;
    let mut on = [Place { how: UNSET, lo: 0, f: 0.0 }; LINES];
    for k in cl.track0 as usize..(cl.track0 + cl.tracks) as usize {
        let tr = tb.tracks[k];
        let (n, comps) = (tr.keys as usize, tr.comps as usize);
        // the track's first channel of its node; the next is a column on
        let o = tr.chan as usize * rows + tr.node as usize;
        let times = tr.at as usize;
        let vals = times + n;
        let p = if (tr.line as usize) < LINES {
            let l = tr.line as usize;
            if on[l].how == UNSET {
                on[l] = place(pool, times, n, t);
            }
            on[l]
        } else {
            place(pool, times, n, t)
        };
        let (a, b) = (vals + p.lo * comps, vals + (p.lo + 1) * comps);
        if p.how != BETWEEN || tr.step {
            // a key's own values: the first, the last, or the one before
            for c in 0..comps {
                d[o + c * rows] = pool[a + c];
            }
            continue;
        }
        let f = p.f;
        if comps == 3 {
            for c in 0..3 {
                let (av, bv) = (get(pool, a + c), get(pool, b + c));
                put(d, o + c * rows, av + (bv - av) * f);
            }
        } else {
            // the shorter way round, then made a unit again
            let (mut bx, mut by, mut bz, mut bw) = (get(pool, b), get(pool, b + 1), get(pool, b + 2), get(pool, b + 3));
            let (ax, ay, az, aw) = (get(pool, a), get(pool, a + 1), get(pool, a + 2), get(pool, a + 3));
            if ax * bx + ay * by + az * bz + aw * bw < 0.0 {
                bx = -bx;
                by = -by;
                bz = -bz;
                bw = -bw;
            }
            store_unit(d, o, rows, ax + (bx - ax) * f, ay + (by - ay) * f, az + (bz - az) * f, aw + (bw - aw) * f);
        }
    }
}
