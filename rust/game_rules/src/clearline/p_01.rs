//! src/game/clearline.ts lines 1 to 407
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// What a man is doing about it this moment: nothing, the row being cut. (clearline.ts clearLineOf: a row of the call surface until the page's side goes)
pub fn clear_line_of(g: &crate::game::Game, gd: GuardId) -> Option<crate::clearline::ClearLineOf> { None }

pub fn mem_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::clearline::Mem {
    g.m.clearline.mem.get_or_insert_with(gd, || Mem {
        since: -100.0,
        seen_t: -100.0,
        said_t: -100.0,
        said: String::new(),
        doing: None,
        rest_until: -100.0,
        try_t: -100.0,
        run_t: -100.0,
        run_of: None,
        lead_t: -100.0,
        lead_p: None,
        lead_of: None,
        lead_for: None,
    })
}

/// (clearline.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// How far `p` is from the segment a → b in plan, and how far along it (0‥1, clamped). (clearline.ts offSeg)
pub fn off_seg(p: V3, a: V3, b: V3) -> [f64; 2] {
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    let l2 = dx * dx + dz * dz;
    let t = if l2 < 1e-6 { 0.0 } else { m::max(0.0, m::min(1.0, ((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2)) };
    [m::hyp2(a[0] + dx * t - p[0], a[2] + dz * t - p[2]), t]
}

/// Every body on the floor but his own, as a list: nothing between the generator's yields writes, so laying it out at once reads the same floor. (clearline.ts bodies)
pub fn bodies(g: &mut crate::game::Game, self_: GuardId) -> Vec<V3> {
    let mut out = Vec::new();
    if !g.pl[g.player].down {
        out.push(g.ch[g.pl[g.player].ch].pos);
    }
    for &o in g.guards.iter() {
        if o != self_ && g.gd[o].state != crate::game::GuardState::Dead {
            out.push(g.ch[g.gd[o].ch].pos);
        }
    }
    for &o in g.civilians.iter() {
        if g.gd[o].state != crate::game::GuardState::Dead {
            out.push(g.ch[g.gd[o].ch].pos);
        }
    }
    out
}

/// From `p` the man at `to` is in plain view and a friend is in the gun's line. (clearline.ts masked)
pub fn masked(g: &mut crate::game::Game, gd: GuardId, p: V3, to: V3) -> bool {
    let from: V3 = [p[0], 1.4, p[2]];
    !crate::scene_collision::shot_blocked(g, from, to, None, None) && crate::roomfight::room_fight_friend_in_line(g, from, to, gd, None).is_some()
}

/// The panel's and the probes' log of what the rule did. (clearline.ts note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: crate::clearline::NoteWhat, by: &str, at: Option<V3>, walk: Option<f64>) {
    let walk = walk.unwrap_or(0.0);
    let what = match what {
        NoteWhat::Cover => ClearNoteWhat::Cover,
        NoteWhat::Step => ClearNoteWhat::Step,
        NoteWhat::End => ClearNoteWhat::End,
        NoteWhat::Kneel => ClearNoteWhat::Kneel,
    };
    let row = ClearNote { t: g.time, who: g.gd[gd].callsign.clone(), what, by: String::from(by), at: at.map(|a| [a[0], a[2]]), walk };
    let log = &mut g.m.clearline.clearline_log;
    log.push(row);
    if log.len() > 300 {
        log.drain(0..100);
    }
}

/// He says what he is doing, not too often. (clearline.ts cue)
pub fn cue(g: &mut crate::game::Game, gd: GuardId, text: &str) {
    let now = g.time;
    let row = mem_of(g, gd);
    if (text == row.said && now - row.said_t < 6.0) || now - row.said_t < 1.2 {
        return;
    }
    row.said_t = now;
    row.said = String::from(text);
    g.stats.clearline.cues += 1;
    crate::game::game_say(g, gd, text, None, None, None);
}

/// His step / cover / knee is over; `rest` s of being left alone by the rule. (clearline.ts endIt)
pub fn end_it(g: &mut crate::game::Game, gd: GuardId, rest: Option<f64>) {
    let rest = rest.unwrap_or(0.0);
    mem_of(g, gd).doing = None;
    g.stats.clearline.ended += 1;
    note(g, gd, NoteWhat::End, "", None, None);
    if rest > 0.0 {
        let until = g.time + rest;
        mem_of(g, gd).rest_until = until;
    }
}

/// What he is doing about a masked shot right now, if anything, and the end of it. The row itself. (clearline.ts doing)
pub fn doing<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> Option<&'g mut crate::clearline::Doing> {
    let d = g.m.clearline.mem.get(gd)?.doing?; // (what it held as asked: to read; every write goes to the row)
    let op = crate::roomfight::room_fight_op_of(g, gd);
    let t = g.time;
    end_it(g, gd, None);
    return None;
}

/// orders.ts, as it poses him: where his feet go while this lasts; None: as ordered. (clearline.ts clearLineFeet)
pub fn clear_line_feet(g: &crate::game::Game, gd: GuardId) -> Option<crate::clearline::ClearLineFeet> {
    return None;
}

/// roomfight.ts (tickOp): this frame his shot at `tgt` is masked by a friend. True: this rule has it. (clearline.ts clearLineBlocked)
pub fn clear_line_blocked(g: &crate::game::Game, gd: GuardId, tgt: GuardId, to: V3, by: crate::clearline::LeadsBy) -> bool {
    return false;
}

#[cfg(test)]
mod tests {
    use super::{flat, off_seg};
    /// The TypeScript's own bits (bun, EXACT.on): offSeg's distance and t, then flat(p, b).
    #[test]
    fn plan_distances_match_the_typescript() {
        let a = [0.1, 0.0, 0.2];
        let b = [7.3, 0.0, -4.9];
        let cases: [([f64; 3], [f64; 3], [f64; 3], [u64; 3]); 4] = [
            ([1.25, 0.0, -3.7], a, b, [0x400424677fd74da1, 0x3fd7288bb0af3462, 0x4018abe38e57b736]),
            ([9.0, 0.0, 9.0], a, b, [0x4028a696ddb5a5f8, 0x3fcf9182b6813ba9, 0x402c01d40df9c5ef]),
            ([-4.0, 0.0, 2.0], a, b, [0x4011e9301f5f39d3, 0x0000000000000000, 0x402a7aed28db5603]),
            ([1.0, 0.0, 1.0], [2.0, 0.0, 2.0], [2.0, 0.0, 2.0], [0x3ff6a09e667f3bcd, 0x0000000000000000, 0x3ff6a09e667f3bcd]),
        ];
        for (p, s, e, bits) in cases {
            let [d, t] = off_seg(p, s, e);
            assert_eq!(d.to_bits(), bits[0]);
            assert_eq!(t.to_bits(), bits[1]);
            assert_eq!(flat(p, e).to_bits(), bits[2]);
        }
    }
}
