//! src/game/sensors.ts lines 1 to 216
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Who keeps a place on a frame that asks the probe for more than it holds. (sensors.ts askRank)
pub fn ask_rank(key: &str, segment: bool) -> f64 {
    if segment {
        if key.starts_with("fow") { 3.0 } else { 1.0 }
    } else if key.starts_with("playerChest") || key.starts_with("playerHead") {
        0.0
    } else {
        2.0
    }
}

/// Queue a light ask for the picture; the defaults are the probe's own. (sensors.ts ProbeLike.query, render/probe.ts 73)
pub fn probe_like_query(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: Option<f64>, rank: Option<f64>) {
    let skip_owner = skip_owner.unwrap_or(0.0);
    let rank = rank.unwrap_or(2.0);
    seam_probe_put(g, key, pos, skip_owner, None, rank);
}

/// Queue a smoke line for the picture; the default is the probe's own. (sensors.ts ProbeLike.querySegment, render/probe.ts 75)
pub fn probe_like_query_segment(g: &mut crate::game::Game, key: &str, a: V3, b: V3, rank: Option<f64>) {
    let rank = rank.unwrap_or(1.0);
    seam_probe_put(g, key, a, 0.0, Some(b), rank);
}

/// The last answer the host filed under `key`: [maxAxis, mean, up]. (sensors.ts ProbeLike.get, render/probe.ts 76)
pub fn probe_like_get(g: &mut crate::game::Game, key: &str) -> Option<Vec<f64>> {
    g.probe.results.get(String::from(key)).map(|r| r.to_vec())
}

/// A delivered value as the journal files it. (sensors.ts encodeValue)
pub fn encode_value(v: Option<f64>) -> Option<crate::sensors::SensorEnc> {
    let v = match v {
        None => return None,
        Some(v) => v,
    };
    if v.is_nan() { return Some(SensorEnc::Nan); }
    if v == f64::INFINITY { return Some(SensorEnc::Inf); }
    if v == f64::NEG_INFINITY { return Some(SensorEnc::Inf2); }
    if v == 0.0 {
        return Some(if 1.0 / v < 0.0 { SensorEnc::V0 } else { SensorEnc::Num(0.0) });
    }
    if m::fround(v) == v {
        // 9 digits always land; the loop only finds the shortest that does
        for p in 1..=9 {
            let r = to_precision_number(v, p);
            if m::fround(r) == v { return Some(SensorEnc::Num(r)); }
        }
        return Some(SensorEnc::List([v]));
    }
    Some(SensorEnc::List([v]))
}

/// `Number(v.toPrecision(p))` for a finite, non-zero single, a tie going up as the language has it (Rust's own formatting takes a tie to even).
/// 120 digits hold a single's whole decimal expansion, so the digit after the cut says all.
fn to_precision_number(v: f64, p: usize) -> f64 {
    let text = format!("{:.120e}", m::abs(v));
    let (mantissa, exp) = text.split_once('e').unwrap();
    let mut exp: i32 = exp.parse().unwrap();
    let digits: Vec<u8> = mantissa.bytes().filter(|b| *b != b'.').collect();
    let mut kept: Vec<u8> = digits[..p].to_vec();
    if digits[p] >= b'5' {
        let mut i = p;
        loop {
            if i == 0 {
                kept.insert(0, b'1');
                kept.pop();
                exp += 1;
                break;
            }
            i -= 1;
            if kept[i] == b'9' {
                kept[i] = b'0';
            } else {
                kept[i] += 1;
                break;
            }
        }
    }
    let r: f64 = format!("{}e{}", String::from_utf8(kept).unwrap(), exp - (p as i32 - 1)).parse().unwrap();
    if v < 0.0 { -r } else { r }
}

/// A filed value back as it was delivered. (sensors.ts decodeValue)
pub fn decode_value(e: Option<crate::sensors::SensorEnc>) -> Option<f64> {
    match e {
        None => None,
        Some(SensorEnc::Num(x)) => Some(m::fround(x)),
        Some(SensorEnc::List(a)) => Some(a[0]),
        Some(SensorEnc::Nan) => Some(f64::NAN),
        Some(SensorEnc::Inf) => Some(f64::INFINITY),
        Some(SensorEnc::V0) => Some(-0.0),
        Some(SensorEnc::Inf2) => Some(f64::NEG_INFINITY),
    }
}

/// `Object.is`: NaN is NaN, −0 is not 0. (sensors.ts sameValue)
pub fn same_value(a: Option<f64>, b: Option<f64>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => (a.is_nan() && b.is_nan()) || a.to_bits() == b.to_bits(),
        _ => false,
    }
}

/// (sensors.ts Sensors.mode)
pub fn sensors_mode(g: &mut crate::game::Game) -> crate::sensors::SensorMode {
    g.sensors.s.mode
}

/// The frame being (or last) simulated in record / replay. (sensors.ts Sensors.frame)
pub fn sensors_frame(g: &mut crate::game::Game) -> f64 {
    g.sensors.s.frame
}

/// (sensors.ts Sensors.stats)
pub fn sensors_stats(g: &mut crate::game::Game) -> crate::sensors::SensorStats {
    g.sensors.s.stats
}

/// Replay: the first divergence of the run. (sensors.ts Sensors.firstDivergence)
pub fn sensors_first_divergence(g: &mut crate::game::Game) -> Option<crate::sensors::SensorDivergence> {
    g.sensors.s.first_divergence.clone()
}

/// (sensors.ts Sensors.divergenceLog)
pub fn sensors_divergence_log(g: &mut crate::game::Game) -> Vec<crate::sensors::SensorDivergence> {
    g.sensors.s.divergence_log.clone()
}

/// Hand the sight lines to the game's own working (None = back to the probe); answers kept from before are dropped. The one function ever handed
/// is the game's own smoke (game.ts 2069), so `f` is never called: only whether one was handed is kept. (sensors.ts Sensors.setOwn)
pub fn sensors_set_own(g: &mut crate::game::Game, f: Option<&mut dyn FnMut(&mut crate::game::Game, V3, V3) -> f64>) {
    g.sensors.own = f.map(|_| SensorsOwn::CpuSmokeAlong);
    g.sensors.own_vals.clear();
}

/// A new encounter: no line's answer outlives the world it was worked out in. (sensors.ts Sensors.ownReset)
pub fn sensors_own_reset(g: &mut crate::game::Game) {
    g.sensors.own_vals.clear();
}

/// The probe is always `g.probe` here: nothing swaps it, so `probe_of` holds nothing. (sensors.ts Sensors.constructor)
pub fn sensors_new(g: &mut crate::game::Game, probe_of: &mut dyn FnMut(&mut crate::game::Game) -> crate::sensors::ProbeLike) -> crate::sensors::Sensors {
    crate::sensors::Sensors::blank()
}

/// Queue an irradiance query at `pos` under `key`; dropped in replay. (sensors.ts Sensors.ask)
pub fn sensors_ask(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: Option<f64>) {
    let skip_owner = skip_owner.unwrap_or(0.0);
    g.sensors.s.stats.asks += 1.0;
    if g.sensors.own.is_some() { return; }
    if g.sensors.s.mode != SensorMode::Replay {
        probe_like_query(g, key, pos, Some(skip_owner), Some(ask_rank(key, false)));
    }
}

/// Queue a smoke-transmittance query along a → b under `key`; dropped in replay. (sensors.ts Sensors.askSegment)
pub fn sensors_ask_segment(g: &mut crate::game::Game, key: &str, a: V3, b: V3) {
    g.sensors.s.stats.asks += 1.0;
    if let Some(own) = g.sensors.own.clone() {
        if g.sensors.s.mode != SensorMode::Replay {
            let t = match own {
                SensorsOwn::CpuSmokeAlong => crate::sense::cpu_smoke_along(g, a, b),
            };
            g.sensors.own_vals.set(String::from(key), t);
        }
        return;
    }
    if g.sensors.s.mode != SensorMode::Replay {
        probe_like_query_segment(g, key, a, b, Some(ask_rank(key, true)));
    }
}

/// Drop a stale answer for `key`; nothing to do in replay. (sensors.ts Sensors.forget)
pub fn sensors_forget(g: &mut crate::game::Game, key: &str) {
    if g.sensors.own.is_some() {
        g.sensors.own_vals.delete(String::from(key));
        return;
    }
    if g.sensors.s.mode != SensorMode::Replay {
        g.probe.results.delete(String::from(key));
    }
}

/// `own`'s answer or the probe's, as it stands now. (sensors.ts Sensors.read, the two sides' common line)
fn answer(g: &mut crate::game::Game, key: &str) -> Option<f64> {
    if g.sensors.own.is_some() {
        g.sensors.own_vals.get(String::from(key)).copied()
    } else {
        let r = probe_like_get(g, key);
        first(r.as_deref())
    }
}

/// THE read: the latest answer for `key`, or None while there is none; journaled in record, served in replay. (sensors.ts Sensors.read)
pub fn sensors_read(g: &mut crate::game::Game, key: &str) -> Option<f64> {
    g.sensors.s.stats.reads += 1.0;
    if g.sensors.s.mode == SensorMode::Live {
        return answer(g, key);
    }
    if g.sensors.s.mode == SensorMode::Record {
        let v = answer(g, key);
        return record_read(&mut g.sensors.s, key, v);
    }
    replay_read(&mut g.sensors.s, key)
}

/// A read while recording: filed under its key, unless it is outside a frame. (sensors.ts Sensors.read)
fn record_read(s: &mut SensorStream, key: &str, v: Option<f64>) -> Option<f64> {
    if !s.in_frame {
        s.stats.strays += 1.0;
        return v;
    }
    let ki = match s.key_idx.get(String::from(key)).copied() {
        Some(ki) => ki,
        None => {
            let ki = s.keys.len() as f64;
            s.keys.push(String::from(key));
            s.key_idx.set(String::from(key), ki);
            s.bytes += key.encode_utf16().count() as f64 + 3.0;
            ki
        }
    };
    s.cur_seq.push(ki);
    s.cur_vals.push(v);
    v
}

/// A read in replay: what the journal delivered for that key on this frame, else a divergence and the key's last value. (sensors.ts Sensors.read)
fn replay_read(s: &mut SensorStream, key: &str) -> Option<f64> {
    if !s.in_frame {
        s.stats.strays += 1.0;
        return s.last_served.get(String::from(key)).copied().flatten();
    }
    if let Some(q) = s.pending.get_mut(String::from(key)) {
        if !q.is_empty() {
            let v = q.remove(0);
            s.last_served.set(String::from(key), v);
            return v;
        }
    }
    let names = s.pending.keys().join(", ");
    let names = if names.is_empty() { String::from("nothing") } else { names };
    let d = SensorDivergence {
        f: s.frame,
        kind: SensorDivergenceKind::UnjournaledRead,
        key: String::from(key),
        detail: format!("the game read '{}' on frame {}; the recording read [{}] there", key, s.frame, names),
    };
    diverge(s, &d);
    s.stats.unjournaled += 1.0;
    s.last_served.get(String::from(key)).copied().flatten()
}

/// A sight line's smoke transmittance: ask along a → b (unless `ask` is false) and read. (sensors.ts Sensors.segment)
pub fn sensors_segment(g: &mut crate::game::Game, key: &str, a: V3, b: V3, ask: Option<bool>) -> Option<f64> {
    if ask.unwrap_or(true) {
        sensors_ask_segment(g, key, a, b);
    }
    sensors_read(g, key)
}

/// A light meter: chest and head, the max of the two when both have finite answers. (sensors.ts Sensors.meterPair)
pub fn sensors_meter_pair(g: &mut crate::game::Game, kc: &str, kh: &str, pc: V3, ph: V3, id_2: f64) -> Option<f64> {
    sensors_ask(g, kc, pc, Some(id_2));
    sensors_ask(g, kh, ph, Some(id_2));
    let r1 = sensors_read(g, kc);
    let r2 = sensors_read(g, kh);
    match (r1, r2) {
        (Some(r1), Some(r2)) if r1.is_finite() && r2.is_finite() => Some(m::max(r1, r2)),
        _ => None,
    }
}

/// The light on a point: ask (unless out of budget) and read. (sensors.ts Sensors.lightAt)
pub fn sensors_light_at(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: Option<f64>, ask: Option<bool>) -> Option<f64> {
    let skip_owner = skip_owner.unwrap_or(0.0);
    if ask.unwrap_or(true) {
        sensors_ask(g, key, pos, Some(skip_owner));
    }
    sensors_read(g, key)
}

/// The top of a simulated frame: record files reads under the new frame number; replay lays out what the recording delivered on it. (sensors.ts Sensors.beginFrame)
pub fn sensors_begin_frame(g: &mut crate::game::Game) {
    begin_frame(&mut g.sensors.s)
}

fn begin_frame(s: &mut SensorStream) {
    if s.mode == SensorMode::Live { return; }
    s.frame += 1.0;
    s.in_frame = true;
    s.stats.frames += 1.0;
    if s.mode == SensorMode::Record {
        s.cur_seq = Vec::new();
        s.cur_vals = Vec::new();
        return;
    }
    // replay: apply this frame's records, then queue the delivered values by key in read order
    let src = s.src.as_ref().unwrap();
    while (s.cursor as usize) < src.recs.len() {
        let r = &src.recs[s.cursor as usize];
        let is_k = matches!(r.first(), Some(Some(SensorJournalRecsItemItem::Text(t))) if t == "K");
        let f = rec_num(&r[if is_k { 1 } else { 0 }]);
        if f > s.frame { break; }
        if is_k {
            s.seq = r[2..].iter().map(rec_num).collect();
            s.vals = s.seq.iter().map(|_| None).collect();
        } else {
            let mut i = 1;
            while i + 1 < r.len() {
                // (a position past the list would only grow an array nothing walks past the list's length)
                let at = rec_num(&r[i]) as usize;
                if at < s.vals.len() { s.vals[at] = decode_value(rec_enc(&r[i + 1])); }
                i += 2;
            }
        }
        s.cursor += 1.0;
    }
    s.pending.clear();
    for (i, ki) in s.seq.iter().enumerate() {
        let k = src.keys[*ki as usize].clone();
        s.pending.get_or_insert_with(k, Vec::new).push(s.vals[i]);
    }
}

/// A number of a journal's record (`as number`).
fn rec_num(item: &Option<SensorJournalRecsItemItem>) -> f64 {
    match item {
        Some(SensorJournalRecsItemItem::Num(x)) => *x,
        _ => f64::NAN,
    }
}

/// A journal's record item as the value code it is: any other word is −∞, as decodeValue has it.
fn rec_enc(item: &Option<SensorJournalRecsItemItem>) -> Option<SensorEnc> {
    match item {
        None => None,
        Some(SensorJournalRecsItemItem::Num(x)) => Some(SensorEnc::Num(*x)),
        Some(SensorJournalRecsItemItem::List(a)) => Some(SensorEnc::List(*a)),
        Some(SensorJournalRecsItemItem::Text(t)) => Some(match t.as_str() {
            "nan" => SensorEnc::Nan,
            "inf" => SensorEnc::Inf,
            "-0" => SensorEnc::V0,
            _ => SensorEnc::Inf2,
        }),
    }
}

/// A value code as an item of a record about to be pushed.
fn enc_item(e: Option<SensorEnc>) -> Option<SensorsPushRItem> {
    match e {
        None => None,
        Some(SensorEnc::Num(x)) => Some(SensorsPushRItem::Num(x)),
        Some(SensorEnc::List(a)) => Some(SensorsPushRItem::List(a)),
        Some(SensorEnc::Nan) => Some(SensorsPushRItem::Text(String::from("nan"))),
        Some(SensorEnc::Inf) => Some(SensorsPushRItem::Text(String::from("inf"))),
        Some(SensorEnc::Inf2) => Some(SensorsPushRItem::Text(String::from("-inf"))),
        Some(SensorEnc::V0) => Some(SensorsPushRItem::Text(String::from("-0"))),
    }
}

/// The end of a simulated frame: record folds the frame's reads into the delta stream; replay counts what was delivered but never asked. (sensors.ts Sensors.endFrame)
pub fn sensors_end_frame(g: &mut crate::game::Game) {
    end_frame(&mut g.sensors.s)
}

fn end_frame(s: &mut SensorStream) {
    if s.mode == SensorMode::Live || !s.in_frame { return; }
    s.in_frame = false;
    if s.mode == SensorMode::Record {
        let f = s.frame;
        // (the TypeScript keeps these arrays as both cur and prev; beginFrame gives cur new ones before anything reads it again)
        let seq = core::mem::take(&mut s.cur_seq);
        let vals = core::mem::take(&mut s.cur_vals);
        let same_seq = seq == s.prev_seq;
        let mut vr: Vec<Option<SensorsPushRItem>> = vec![Some(SensorsPushRItem::Num(f))];
        if !same_seq {
            let mut kr: Vec<Option<SensorsPushRItem>> = vec![Some(SensorsPushRItem::Text(String::from("K"))), Some(SensorsPushRItem::Num(f))];
            kr.extend(seq.iter().map(|k| Some(SensorsPushRItem::Num(*k))));
            push(s, &kr);
            for (i, v) in vals.iter().enumerate() {
                if v.is_some() {
                    vr.push(Some(SensorsPushRItem::Num(i as f64)));
                    vr.push(enc_item(encode_value(*v)));
                }
            }
        } else {
            for (i, v) in vals.iter().enumerate() {
                if !same_value(*v, s.prev_vals[i]) {
                    vr.push(Some(SensorsPushRItem::Num(i as f64)));
                    vr.push(enc_item(encode_value(*v)));
                }
            }
        }
        if vr.len() > 1 { push(s, &vr); }
        s.prev_seq = seq;
        s.prev_vals = vals;
        s.stats.recs = s.recs.len() as f64;
        return;
    }
    let mut left = 0usize;
    let mut first_key = String::new();
    for (k, q) in s.pending.iter() {
        if !q.is_empty() {
            left += q.len();
            if first_key.is_empty() { first_key = k; }
        }
    }
    if left != 0 {
        let frame = s.frame;
        s.stats.unasked += left as f64;
        let d = SensorDivergence {
            f: frame,
            kind: SensorDivergenceKind::Unasked,
            detail: format!("{} value(s) the recording delivered on frame {} were never read (first: '{}')", left, frame, first_key),
            key: first_key,
        };
        diverge(s, &d);
    }
}

/// File a record, with a running estimate of the stream as JSON. (sensors.ts Sensors.push)
pub fn sensors_push(g: &mut crate::game::Game, r: &[Option<crate::sensors::SensorsPushRItem>]) {
    push(&mut g.sensors.s, r)
}

fn push(s: &mut SensorStream, r: &[Option<SensorsPushRItem>]) {
    let is_k = matches!(r.first(), Some(Some(SensorsPushRItem::Text(t))) if t == "K");
    let rec = r.iter().map(|item| item.as_ref().map(|item| match item {
        SensorsPushRItem::Text(t) => SensorStreamRecsItemItem::Text(t.clone()),
        SensorsPushRItem::Num(x) => SensorStreamRecsItemItem::Num(*x),
        SensorsPushRItem::List(a) => SensorStreamRecsItemItem::List(*a),
    })).collect();
    s.recs.push(rec);
    s.bytes += 3.0 + r.len() as f64 * if is_k { 3.0 } else { 7.0 };
}

/// (sensors.ts Sensors.diverge)
pub fn sensors_diverge(g: &mut crate::game::Game, d: &crate::sensors::SensorDivergence) {
    diverge(&mut g.sensors.s, d)
}

fn diverge(s: &mut SensorStream, d: &SensorDivergence) {
    s.stats.divergences += 1.0;
    if s.first_divergence.is_none() { s.first_divergence = Some(d.clone()); }
    if s.divergence_log.len() < 20 { s.divergence_log.push(d.clone()); }
}

/// Back to the plain probe. (sensors.ts Sensors.live)
pub fn sensors_live(g: &mut crate::game::Game) {
    live(&mut g.sensors.s)
}

fn live(s: &mut SensorStream) {
    s.mode = SensorMode::Live;
    s.in_frame = false;
    s.src = None;
    s.pending.clear();
}

fn zero_stats(recs: f64) -> SensorStats {
    SensorStats { reads: 0.0, asks: 0.0, strays: 0.0, frames: 0.0, recs, divergences: 0.0, unasked: 0.0, unjournaled: 0.0 }
}

/// Start journaling: the next beginFrame is frame 0. (sensors.ts Sensors.record)
pub fn sensors_record(g: &mut crate::game::Game) {
    record(&mut g.sensors.s)
}

fn record(s: &mut SensorStream) {
    s.mode = SensorMode::Record;
    s.frame = -1.0;
    s.in_frame = false;
    s.keys = Vec::new();
    s.key_idx.clear();
    s.recs = Vec::new();
    s.bytes = 16.0;
    s.prev_seq = Vec::new();
    s.prev_vals = Vec::new();
    s.cur_seq = Vec::new();
    s.cur_vals = Vec::new();
    s.stats = zero_stats(0.0);
}

/// The journal so far: the file's `sensors` block. (sensors.ts Sensors.journal)
pub fn sensors_journal(g: &mut crate::game::Game) -> crate::sensors::SensorJournal {
    journal(&g.sensors.s)
}

fn journal(s: &SensorStream) -> SensorJournal {
    SensorJournal {
        keys: s.keys.clone(),
        recs: s.recs.iter().map(|r| r.iter().map(|item| item.as_ref().map(|item| match item {
            SensorStreamRecsItemItem::Text(t) => SensorJournalRecsItemItem::Text(t.clone()),
            SensorStreamRecsItemItem::Num(x) => SensorJournalRecsItemItem::Num(*x),
            SensorStreamRecsItemItem::List(a) => SensorJournalRecsItemItem::List(*a),
        })).collect()).collect(),
        frames: if s.mode == SensorMode::Record { s.frame + if s.in_frame { 0.0 } else { 1.0 } } else { s.stats.frames },
    }
}

/// Rough size of the sensor stream as JSON so far. (sensors.ts Sensors.approxBytes)
pub fn sensors_approx_bytes(g: &mut crate::game::Game) -> f64 {
    g.sensors.s.bytes
}

/// Serve a recording: the next beginFrame is its frame 0. (sensors.ts Sensors.replay)
pub fn sensors_replay(g: &mut crate::game::Game, src: &crate::sensors::SensorJournal) {
    replay(&mut g.sensors.s, src)
}

fn replay(s: &mut SensorStream, src: &SensorJournal) {
    s.mode = SensorMode::Replay;
    s.src = Some(src.clone());
    s.frame = -1.0;
    s.in_frame = false;
    s.cursor = 0.0;
    s.seq = Vec::new();
    s.vals = Vec::new();
    s.pending.clear();
    s.last_served.clear();
    s.first_divergence = None;
    s.divergence_log = Vec::new();
    s.stats = zero_stats(src.recs.len() as f64);
}

/// The panel's line. (sensors.ts Sensors.summary)
pub fn sensors_summary(g: &mut crate::game::Game) -> String {
    summary(&g.sensors.s)
}

fn summary(s: &SensorStream) -> String {
    let t = s.stats;
    if s.mode == SensorMode::Live {
        return format!("sensors {}", s.mode.as_str());
    }
    let strays = if js::truthy(t.strays) { format!(" / {} stray", t.strays) } else { String::new() };
    let tail = if s.mode == SensorMode::Record {
        // `.toFixed(1)` of a whole count over 1024, a tie going up as the language has it (`{:.1}` would take it to even)
        let tenths = (s.bytes as u64 * 10 + 512) / 1024;
        format!("{} keys, {} records ≈ {}.{} KB", s.keys.len(), t.recs, tenths / 10, tenths % 10)
    } else {
        let first_on = match &s.first_divergence {
            Some(d) => format!(" — first on frame {}: {}", d.f, d.detail),
            None => String::new(),
        };
        format!("{} divergence(s){}", t.divergences, first_on)
    };
    format!("sensors {} · frame {} · {} reads / {} asks{} · {}", s.mode.as_str(), s.frame, t.reads, t.asks, strays, tail)
}

/// (sensors.ts first)
pub fn first(r: Option<&[f64]>) -> Option<f64> {
    r.and_then(|r| r.first().copied())
}

/// `this.put({ key, pos, skipOwner, b, rank })` (render/probe.ts 73, 75, 81): ProbeLike holds only `results`, so the frame's table of asks has no home.
fn seam_probe_put(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: f64, b: Option<V3>, rank: f64) {
    // BOUNDARY (bring-up): the ask goes to the picture's probe (render/probe.ts put), whose table of asks is the host's and no part of the game's state.
    // Nothing is recorded for the host yet: a headless host answers none, so `g.probe.results` stays as it is and every read is "no answer".
    // SKELETON-GAP: a host seam that takes the ask (key, pos, skip_owner, b, rank), and `room` back for the callers that test it.
    let _ = (g, key, pos, skip_owner, b, rank);
    super::b_01::host_probe_put();   // TEMP-BRINGUP: the count `room` is read off (b_01.rs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enc_text(e: Option<SensorEnc>) -> String {
        match e {
            None => String::from("null"),
            Some(SensorEnc::Num(x)) => format!("n{:016x}", x.to_bits()),
            Some(SensorEnc::List(a)) => format!("l{:016x}", a[0].to_bits()),
            Some(SensorEnc::Nan) => String::from("nan"),
            Some(SensorEnc::Inf) => String::from("inf"),
            Some(SensorEnc::Inf2) => String::from("-inf"),
            Some(SensorEnc::V0) => String::from("-0"),
        }
    }

    // each row: the value, then what the TypeScript's encodeValue and decodeValue give (bits off bun; a NaN's decoded bits are not compared)
    #[test]
    fn encoding_has_the_typescripts_bits() {
        let fr = m::fround;
        let rows: &[(f64, &str, u64)] = &[
            (1.0, "n3ff0000000000000", 0x3ff0000000000000),
            (0.5, "n3fe0000000000000", 0x3fe0000000000000),
            (0.1, "l3fb999999999999a", 0x3fb999999999999a),
            (fr(0.1), "n3fb999999999999a", 0x3fb99999a0000000),
            (fr(0.7), "n3fe6666666666666", 0x3fe6666660000000),
            (1234567.25, "n4132d6874ccccccd", 0x4132d68740000000),
            (-1234567.25, "nc132d6874ccccccd", 0xc132d68740000000),
            (33554436.0, "n4180000020000000", 0x4180000020000000),
            (16777216.0, "n4170000000000000", 0x4170000000000000),
            (0.25, "n3fd0000000000000", 0x3fd0000000000000),
            (2.5, "n4004000000000000", 0x4004000000000000),
            (9.5, "n4023000000000000", 0x4023000000000000),
            (0.95, "l3fee666666666666", 0x3fee666666666666),
            (fr(0.95), "n3fee666666666666", 0x3fee666660000000),
            (99999.5, "n40f869f800000000", 0x40f869f800000000),
            (fr(99999.99), "n40f869ffd70a3d71", 0x40f869ffe0000000),
            (fr(1e-40), "n37a16c262777579c", 0x37a16c2000000000),
            (fr(3.3e38), "n47ef087605722464", 0x47ef087600000000),
            (1.401298464324817e-45, "n3696d601ad376ab9", 0x36a0000000000000),
            (0.0, "n0000000000000000", 0),
            (-0.0, "-0", 0x8000000000000000),
            (f64::INFINITY, "inf", 0x7ff0000000000000),
            (f64::NEG_INFINITY, "-inf", 0xfff0000000000000),
            (1e300, "l7e37e43c8800759c", 0x7e37e43c8800759c),
        ];
        for (v, e, d) in rows {
            let got = encode_value(Some(*v));
            assert_eq!(enc_text(got), *e, "encode {}", v);
            assert_eq!(decode_value(got).unwrap().to_bits(), *d, "decode {}", v);
        }
        assert_eq!(enc_text(encode_value(Some(f64::NAN))), "nan");
        assert!(decode_value(encode_value(Some(f64::NAN))).unwrap().is_nan());
        assert!(encode_value(None).is_none() && decode_value(None).is_none());
    }

    // 20,000 singles from a small generator, the encoded doubles folded by FNV-1a: the fold is the TypeScript's
    #[test]
    fn many_singles_encode_to_the_typescripts_fold() {
        let (mut h, mut st) = (0xcbf29ce484222325u64, 12345u32);
        for _ in 0..20000 {
            st = st.wrapping_mul(1664525).wrapping_add(1013904223);
            let v = f32::from_bits(st) as f64;
            if !v.is_finite() || v == 0.0 { continue; }
            let r = match encode_value(Some(v)) {
                Some(SensorEnc::Num(r)) => r,
                Some(SensorEnc::List(a)) => a[0],
                _ => unreachable!(),
            };
            h = (h ^ r.to_bits()).wrapping_mul(0x100000001b3);
        }
        assert_eq!(format!("{:x}", h), "5f200f02baadbd6b");
    }

    // what `g` adds to the stream: whose answers a read gets, and which asks stop before the picture. No path here reaches the seam or the game's smoke.
    #[test]
    fn a_read_goes_to_the_probes_table_or_to_own() {
        let mut g = Game::blank();
        let g = &mut g;
        assert_eq!(sensors_read(g, "k"), None);
        g.probe.results.set(String::from("k"), [0.25, 8.0, 9.0]);
        assert_eq!(probe_like_get(g, "k"), Some(vec![0.25, 8.0, 9.0]));
        assert_eq!(sensors_read(g, "k"), Some(0.25));
        sensors_forget(g, "k");
        assert_eq!(sensors_read(g, "k"), None);

        g.probe.results.set(String::from("k"), [0.25, 8.0, 9.0]);
        g.sensors.own_vals.set(String::from("old"), 1.0);
        sensors_set_own(g, Some(&mut |_: &mut Game, _: V3, _: V3| 0.0));
        assert!(matches!(g.sensors.own, Some(SensorsOwn::CpuSmokeAlong)));
        assert_eq!(sensors_read(g, "old"), None, "setOwn drops what was kept");
        assert_eq!(sensors_read(g, "k"), None, "nothing of the probe's is served");
        sensors_ask(g, "k", [0.0; 3], None);   // own set: a light ask stops here
        g.sensors.own_vals.set(String::from("line"), 0.5);
        assert_eq!(sensors_segment(g, "line", [0.0; 3], [1.0; 3], Some(false)), Some(0.5));
        sensors_forget(g, "k");
        assert!(g.probe.results.get(String::from("k")).is_some(), "own's forget leaves the probe's table");
        sensors_own_reset(g);
        assert_eq!(sensors_read(g, "line"), None);

        sensors_set_own(g, None);
        assert_eq!(sensors_read(g, "k"), Some(0.25));
        sensors_replay(g, &SensorJournal::zeroed());
        sensors_ask(g, "k", [0.0; 3], None);   // replay: nothing is asked of the picture
        sensors_ask_segment(g, "k", [0.0; 3], [1.0; 3]);
        sensors_forget(g, "k");
        assert!(g.probe.results.get(String::from("k")).is_some(), "replay's forget does nothing");
        assert_eq!(sensors_stats(g).asks, 2.0);
    }

    #[test]
    fn ranks_and_same_value() {
        let row = |k| (ask_rank(k, false), ask_rank(k, true));
        assert_eq!(row("playerChest"), (0.0, 1.0));
        assert_eq!(row("playerHead2"), (0.0, 1.0));
        assert_eq!(row("fowx"), (2.0, 3.0));
        assert_eq!(row("x"), (2.0, 1.0));
        assert!(same_value(Some(f64::NAN), Some(-f64::NAN)) && same_value(None, None));
        assert!(!same_value(Some(0.0), Some(-0.0)) && !same_value(None, Some(0.0)));
    }

    fn show(v: Option<f64>) -> String {
        match v {
            None => String::from("u"),
            Some(x) => format!("{}", x),
        }
    }
    fn journal_text(j: &SensorJournal) -> String {
        let quoted: Vec<String> = j.keys.iter().map(|k| format!("\"{}\"", k)).collect();
        let recs: Vec<String> = j.recs.iter().map(|r| {
            let items: Vec<String> = r.iter().map(|i| match i {
                None => String::from("null"),
                Some(SensorJournalRecsItemItem::Text(t)) => format!("\"{}\"", t),
                Some(SensorJournalRecsItemItem::Num(x)) => format!("{}", x),
                Some(SensorJournalRecsItemItem::List(a)) => format!("[{}]", a[0]),
            }).collect();
            format!("[{}]", items.join(","))
        }).collect();
        format!("{{\"keys\":[{}],\"recs\":[{}],\"frames\":{}}}", quoted.join(","), recs.join(","), j.frames)
    }
    fn stats_text(s: SensorStats) -> String {
        format!("{{\"reads\":{},\"asks\":{},\"strays\":{},\"frames\":{},\"recs\":{},\"divergences\":{},\"unasked\":{},\"unjournaled\":{}}}", s.reads, s.asks, s.strays, s.frames, s.recs, s.divergences, s.unasked, s.unjournaled)
    }

    // the same script as the TypeScript's, every line of what it printed: a tape recorded with the game's own answers planted, then served back with a
    // game that strays from it. On the stream and not through `g`, since these rows are all the stream's.
    #[test]
    fn a_recording_and_its_replay_say_what_the_typescript_says() {
        let got = {
            let mut z = SensorStream::blank();
            let s = &mut z;
            let mut own: JsMap<String, f64> = JsMap::new();
            let mut l: Vec<String> = Vec::new();
            macro_rules! rd { ($k:expr) => {
                s.stats.reads += 1.0;
                let v = if s.mode == SensorMode::Replay { replay_read(s, $k) } else { let a = own.get(String::from($k)).copied(); record_read(s, $k, a) };
                l.push(format!("{}={}", $k, show(v)));
            } }
            macro_rules! put { ($k:expr, $v:expr) => { own.set(String::from($k), $v); } }
            record(s);
            begin_frame(s); put!("a", 0.5); put!("b", m::fround(0.7)); rd!("a"); rd!("b"); rd!("c"); end_frame(s);
            begin_frame(s); put!("b", f64::NAN); rd!("a"); rd!("b"); rd!("c"); end_frame(s);
            begin_frame(s); put!("c", 0.1); put!("a", -0.0); rd!("a"); rd!("b"); rd!("c"); end_frame(s);
            begin_frame(s); put!("b", 1.0); own.delete(String::from("c")); rd!("b"); end_frame(s);
            begin_frame(s); put!("b", 1.0); rd!("b"); own.delete(String::from("b")); rd!("b"); end_frame(s);
            rd!("a");
            l.push(summary(s));
            l.push(format!("{}", s.bytes));
            let j = journal(s);
            l.push(journal_text(&j));
            l.push(stats_text(s.stats));
            replay(s, &j);
            begin_frame(s); rd!("a"); rd!("b"); rd!("c"); end_frame(s);
            begin_frame(s); rd!("a"); rd!("b"); end_frame(s);
            begin_frame(s); rd!("z"); rd!("a"); rd!("b"); rd!("c"); end_frame(s);
            begin_frame(s); rd!("b"); rd!("b"); end_frame(s);
            begin_frame(s); rd!("b"); rd!("b"); end_frame(s);
            begin_frame(s); rd!("q"); end_frame(s);
            rd!("b");
            l.push(summary(s));
            l.push(stats_text(s.stats));
            for d in s.divergence_log.clone() {
                l.push(format!("{}|{}|{}|{}", d.f, d.kind.as_str(), d.key, d.detail));
            }
            l.push(journal_text(&journal(s)));
            live(s);
            l.push(summary(s));
            l.join("\n")
        };
        assert_eq!(got, r#"a=0.5
b=0.699999988079071
c=u
a=0.5
b=NaN
c=u
a=-0
b=NaN
c=0.1
b=1
b=1
b=u
a=-0
sensors record · frame 4 · 13 reads / 0 asks / 1 stray · 3 keys, 8 records ≈ 0.2 KB
221
{"keys":["a","b","c"],"recs":[["K",0,0,1,2],[0,0,0.5,1,0.7],[1,1,"nan"],[2,0,"-0",2,[0.1]],["K",3,1],[3,0,1],["K",4,1,1],[4,0,1]],"frames":5}
{"reads":13,"asks":0,"strays":1,"frames":5,"recs":8,"divergences":0,"unasked":0,"unjournaled":0}
a=0.5
b=0.699999988079071
c=u
a=0.5
b=NaN
z=u
a=-0
b=NaN
c=0.1
b=1
b=1
b=1
b=u
q=u
b=u
sensors replay · frame 5 · 15 reads / 0 asks / 1 stray · 5 divergence(s) — first on frame 1: 1 value(s) the recording delivered on frame 1 were never read (first: 'c')
{"reads":15,"asks":0,"strays":1,"frames":6,"recs":8,"divergences":5,"unasked":3,"unjournaled":3}
1|unasked|c|1 value(s) the recording delivered on frame 1 were never read (first: 'c')
2|unjournaled-read|z|the game read 'z' on frame 2; the recording read [a, b, c] there
3|unjournaled-read|b|the game read 'b' on frame 3; the recording read [b] there
5|unjournaled-read|q|the game read 'q' on frame 5; the recording read [b] there
5|unasked|b|2 value(s) the recording delivered on frame 5 were never read (first: 'b')
{"keys":["a","b","c"],"recs":[["K",0,0,1,2],[0,0,0.5,1,0.7],[1,1,"nan"],[2,0,"-0",2,[0.1]],["K",3,1],[3,0,1],["K",4,1,1],[4,0,1]],"frames":6}
sensors live"#);
    }
}
