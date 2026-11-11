//! A CLIP SAMPLED: where a time falls among its keys, and each track's node put there. A clip is a character file's, read once and written by nobody after (game_types anim_gltf `Clip`).
//!
//! WHERE A SAMPLE'S TIME FALLS AMONG A TRACK'S KEYS depends on the track's times alone, and a clip's tracks share theirs (the game's rig: 159 tracks a clip on two sets of times). So every
//! track has the number of its LINE, the set of times it is on, and a sample finds its place on a line once.
use crate::single::{get, step3, step4, weight, S};
use crate::skeleton::NODE;
use crate::world::{js_max, js_min};
use game_types::anim_gltf::{Clip, LINES};

/// where a time falls on a line: before its first key (or it has one key) · at or past its last · between key `lo` and the next, `f` of the way
#[derive(Clone, Copy)]
enum Place { First, Last(usize), Between(usize, f64) }

fn place(times: &[S], t: f64) -> Place {
    let n = times.len();
    if n == 1 || t <= get(times, 0) {
        return Place::First;
    }
    if t >= get(times, n - 1) {
        return Place::Last(n - 1);
    }
    // the key at or before t, and the one after
    let (mut lo, mut hi) = (0usize, n - 1);
    while hi - lo > 1 {
        let mid = (lo + hi) >> 1;
        if get(times, mid) <= t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Place::Between(lo, (t - get(times, lo)) / js_max(1e-6, get(times, hi) - get(times, lo)))
}

/// The clip at time `t` (seconds; wrapped where `looped`, else held inside the clip) into the pose `d`: only the nodes the clip has tracks for are written.
pub fn sample(clip: &Clip, mut t: f64, looped: bool, d: &mut [S]) {
    if looped {
        t = t % clip.duration;
        if t < 0.0 {
            t += clip.duration;
        }
    } else {
        t = js_min(js_max(t, 0.0), clip.duration);
    }
    let rows = d.len() / NODE;
    let mut on = [None::<Place>; LINES];
    for tr in &clip.tracks {
        // the track's first channel of its node; the next is a column on
        let o = tr.chan() * rows + tr.node;
        let (times, vals) = (S::of(&tr.times), S::of(&tr.values));
        let p = match on.get_mut(tr.line) {
            Some(l) => *l.get_or_insert_with(|| place(times, t)),
            None => place(times, t),
        };
        let comps = tr.comps();
        let (lo, f) = match p {
            Place::Between(lo, f) if !tr.step => (lo, weight(f)),
            // a key's own values: the first, the last, or the one before
            Place::First | Place::Last(_) | Place::Between(..) => {
                let k = match p { Place::First => 0, Place::Last(k) | Place::Between(k, _) => k };
                for c in 0..comps {
                    d[o + c * rows] = vals[k * comps + c];
                }
                continue;
            }
        };
        // (the step between the two keys in singles: single.rs)
        if comps == 3 {
            step3(d, o, rows, vals, lo * comps, (lo + 1) * comps, f);
        } else {
            step4(d, o, rows, vals, lo * comps, (lo + 1) * comps, f);
        }
    }
}
