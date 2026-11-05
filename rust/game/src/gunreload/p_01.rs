//! src/game/gunreload.ts lines 1 to 233
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// a point of a gun's model in the hand's own space (gunreload.ts g2w)
pub fn g2w(p: V3, d: Option<V3>) -> V3 {
    let d = d.unwrap_or([0.0, 0.0, 0.0]);
    let o = group();
    [p[0] + o[0] + d[0], p[1] + o[1] + d[1], p[2] + o[2] + d[2]]
}

/// a gun's row (`gun.def`), a copy that can be lent beside `g`
fn gun_def(g: &Game, gun: FirearmId) -> crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(g.fa[gun].def).unwrap().clone()
}

/// how this row is fed right now; None: a magazine change as it always was (gunreload.ts feedOf)
pub fn feed_of(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> Option<crate::gunreload::FEEDValue> {
    crate::gunlook::gun_own(g, def).map(|o| feed_by(o.model))
}

/// how each model is fed (gunreload.ts FEED, feedBy)
pub fn feed_by(m: crate::weapons::GunModelId) -> crate::gunreload::FEEDValue {
    feed().get(String::from(m.as_str())).unwrap().clone()
}

/// the belt's moments inside a beat (gunreload.ts beltTimes)
pub fn belt_times(b: crate::gunreload::FeedBeat) -> crate::gunreload::BeltTimes {
    let o = b.out;
    let i = b.in_;
    let end = if b.pull { b.rack } else { b.dur - 0.3 };
    let up = m::max(0.1, o * 0.45);
    let on = o + (i - o) * 0.62;
    let down = i + m::min(0.5, m::max(0.12, (end - i) * 0.5));
    BeltTimes { up, off: o, pouch: o + (on - o) * 0.5, on, lay: i, down, charge: if b.pull { b.rack } else { -1.0 } }
}

/// the shells' moments: `n` of them, the last home at the beat's `in_` (gunreload.ts shellTimes)
pub fn shell_times(b: crate::gunreload::FeedBeat, n: f64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut k = 0.0;
    while k < n {
        out.push(b.out + (k + 1.0) * (b.in_ - b.out) / n);
        k += 1.0;
    }
    out
}

/// a shell's own pitch, dealt off its place in the tube (gunreload.ts shellRate)
pub fn shell_rate(k: f64) -> f64 {
    0.94 + js::rem(k * 5.0, 7.0) * 0.02
}

fn cue(t: f64, sfx: &str, gain: f64, rate: f64) -> FeedCue {
    FeedCue { t, sfx: String::from(sfx), gain, rate }
}

/// what is heard, in order, of a reload of this row on this beat; None: a magazine gun (gunreload.ts feedCues)
pub fn feed_cues(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef, b: crate::gunreload::FeedBeat, n: f64, port: bool) -> Option<Vec<crate::gunreload::FeedCue>> {
    let f = feed_of(g, def)?;
    cues_of(f, b, n, port)
}

/// feedCues past its one read of the game
fn cues_of(f: FEEDValue, b: FeedBeat, n: f64, port: bool) -> Option<Vec<FeedCue>> {
    if f.kind == "mag" {
        return None;
    }
    let mut out = Vec::new();
    if f.kind == "belt" {
        let t = belt_times(b);
        out.push(cue(t.up, "beltCoverUp", 0.8, 1.0));
        out.push(cue(t.off, "beltBoxOff", 0.8, 1.0));
        out.push(cue(t.on, "beltBoxOn", 0.85, 1.0));
        out.push(cue(t.lay, "beltLay", 0.8, 1.0));
        out.push(cue(t.down, "beltCoverDown", 0.9, 1.0));
        if b.pull {
            out.push(cue(t.charge, "chargeLmg", 0.9, 1.0));
        }
    } else {
        let t = shell_times(b, n);
        let first = port && f.port_first == Some(true);
        let mut k = 0.0;
        let mut i = 0usize;
        while k < n {
            let head = first && i == 0;
            out.push(cue(t[i], "shellIn", if head { 0.6 } else { 0.8 }, shell_rate(k)));
            if head {
                out.push(cue(m::min(t[0] + 0.12, if n > 1.0 { t[1] - 0.02 } else { b.in_ + 0.12 }), "boltDrop", 0.9, 1.0));
            }
            k += 1.0;
            i += 1;
        }
        if b.pull && !first {
            out.push(if f.pump == Some(true) { cue(b.rack, "rack", 0.85, 0.7) } else { cue(b.rack, "boltDrop", 0.9, 1.0) });
        }
    }
    js::sort_by(&mut out, |x, y| x.t - y.t);
    Some(out)
}

fn put(k: &mut Vec<FeedKey>, t: f64, at: FeedKeyAt, how: FeedKeyHow, grip: f64) {
    k.push(FeedKey { t, at, how, grip });
}

/// where the support hand is over a reload of this model: its stations, or a magazine gun's own marks (gunreload.ts feedPath)
pub fn feed_path(g: &mut crate::game::Game, model: crate::weapons::GunModelId, b: crate::gunreload::FeedBeat, n: f64, port: bool) -> Option<crate::gunreload::FeedPath> {
    Some(path_of(model, b, n, port))
}

/// feedPath past the switch (GUNLOOK.on is settled on)
fn path_of(model: crate::weapons::GunModelId, b: FeedBeat, n: f64, port: bool) -> FeedPath {
    let f = feed_by(model);
    if f.kind == "mag" {
        return FeedPath::Arm1(FeedPathArm1 { marks: f.marks });
    }
    use FeedKeyAt::{List, Pouch};
    let (mag, top) = (FeedKeyHow::Mag, FeedKeyHow::Top);
    let mut k: Vec<FeedKey> = Vec::new();
    if f.kind == "belt" {
        let t = belt_times(b);
        let s = belt();
        put(&mut k, t.up - 0.08, List(s.cover), top, 0.3);
        put(&mut k, t.up + 0.14, List(s.lifted), top, 0.5);
        put(&mut k, t.off - 0.12, List(s.box_), mag, 0.3);
        put(&mut k, t.off, List(s.box_), mag, 1.0);
        put(&mut k, t.off + 0.14, List(s.strip), mag, 1.0);
        put(&mut k, t.pouch - 0.08, Pouch, FeedKeyHow::Pouch, 0.2);
        put(&mut k, t.pouch + 0.08, Pouch, FeedKeyHow::Pouch, 1.0);
        put(&mut k, t.on - 0.14, List(s.strip), mag, 1.0);
        put(&mut k, t.on, List(s.box_), mag, 1.0);
        put(&mut k, t.on + 0.1, List(s.box_), mag, 0.3);
        put(&mut k, t.lay - 0.1, List(s.tray), top, 0.6);
        put(&mut k, t.lay + 0.06, List(s.tray), top, 0.3);
        put(&mut k, t.down - 0.14, List(s.lifted), top, 0.4);
        put(&mut k, t.down, List(s.cover), top, 0.2);
        put(&mut k, t.down + 0.06, List(s.cover), top, 0.2);
        if b.pull {
            put(&mut k, t.charge - 0.1, List(s.handle), mag, 0.9);
            put(&mut k, t.charge, List(s.handle), mag, 1.0);
            put(&mut k, t.charge + 0.08, List(s.pull), mag, 1.0);
            put(&mut k, t.charge + 0.17, List(s.handle), mag, 0.4);
        }
    } else {
        let t = shell_times(b, n);
        let per = if n > 0.0 { (b.in_ - b.out) / n } else { 0.0 };
        let s = tube();
        let first = port && f.port_first == Some(true);
        let mut c = 0.0;
        let mut i = 0usize;
        while c < n {
            let head = first && i == 0;
            let half = m::min(per * 0.5, 0.3);
            let to = if head { s.eject } else { s.port };
            let home = if head { s.eject } else { s.home };
            put(&mut k, t[i] - half - m::min(0.06, per * 0.12), List(s.saddle), mag, 0.4);
            put(&mut k, t[i] - half, List(s.saddle), mag, 0.9);
            put(&mut k, t[i] - m::min(0.07, per * 0.2), List(to), mag, 0.9);
            put(&mut k, t[i], List(home), mag, 0.7);
            c += 1.0;
            i += 1;
        }
        if b.pull && !first && f.pump != Some(true) {
            put(&mut k, b.rack - 0.12, List(s.eject), mag, 0.6);
            put(&mut k, b.rack, List(s.eject), mag, 0.9);
        }
    }
    // in order, a frame apart at the least, done before the hand's way back to the gun
    let mut last = 0.06;
    for key in k.iter_mut() {
        key.t = m::min(m::max(key.t, last + 0.02), b.dur - 0.1);
        last = key.t;
    }
    // the filter's K[i - 1] is the list before filtering, not the last one kept
    let mut keys = Vec::new();
    for i in 0..k.len() {
        if i == 0 || k[i].t > k[i - 1].t + 1e-6 {
            keys.push(k[i]);
        }
    }
    FeedPath::Arm0(FeedPathArm0 { keys })
}

/// dealt to a man's animator: the long gun in his hands or None, how the support hand is to feed it (gunreload.ts feedDeal)
pub fn feed_deal(g: &mut crate::game::Game, ch: CharId, gun: Option<FirearmId>, dry: Option<bool>) {
    let dry = dry.unwrap_or(false);
    let o = match gun {
        Some(id) => {
            let def = gun_def(g, id);
            crate::gunlook::gun_own(g, &def)
        }
        None => None,
    };
    let (Some(gun), Some(o)) = (gun, o) else {
        g.ch[ch].anim.feed = None;
        return;
    };
    let plain = dry && !crate::weapons::firearm_shell_fed(g, gun);
    let (n, port) = if plain {
        (gun_def(g, gun).mag_capacity, true)
    } else {
        (g.fa[gun].feed_n, g.fa[gun].feed_port)
    };
    let an = &mut g.ch[ch].anim;
    an.feed = Some(o.model);
    an.feed_n = n;
    an.feed_port = port;
}

/// the switch is up (gunreload.ts feedLive)
pub fn feed_live(g: &mut crate::game::Game) -> bool {
    crate::gunlook::gun_look_on(g)
}

/// what the player's reload of `gun` on `row` is to sound like and to drop, asked before startReload (gunreload.ts feedPlan)
pub fn feed_plan(g: &mut crate::game::Game, gun: FirearmId, row: crate::weapons::WeaponDefReload, shells: Option<crate::gunreload::FeedPlanShells>) -> Option<crate::gunreload::FeedPlan> {
    let def = gun_def(g, gun);
    let f = feed_of(g, &def)?;
    let n = match shells {
        Some(s) => s.n,
        None => {
            let b = plan_best(g, gun, &def);
            m::max(0.0, b - g.fa[gun].mag.unwrap_or(0.0))
        }
    };
    let port = g.fa[gun].chamber == 0.0 && !g.fa[gun].mag.map_or(false, js::truthy);
    let pull = match shells {
        Some(s) => s.pull,
        None => g.fa[gun].chamber == 0.0,
    };
    let cues = feed_cues(g, &def, beat_of(row, pull), n, port);
    let times = match &cues {
        Some(c) => c.iter().map(|q| q.t).collect(),
        None => Vec::new(),
    };
    Some(FeedPlan { cues, times, drop: f.drop })
}

/// feedPlan's `best`: the same sums as startReload's, the fullest spare, as much of it as the gun may hold
fn plan_best(g: &mut Game, gun: FirearmId, def: &crate::weapons::WeaponDef) -> f64 {
    let x = if !g.fa[gun].spare.is_empty() {
        m::max_n(&g.fa[gun].spare)
    } else if crate::weapons::firearm_bottomless(g, gun) {
        def.mag_capacity
    } else {
        g.fa[gun].mag.unwrap_or(0.0)
    };
    let room = crate::weapons::firearm_mag_room(g, gun);
    m::min(x, room)
}

/// a weapon's reload row as a beat; a row that gives only its length does everything at the end (gunreload.ts beatOf)
pub fn beat_of(r: crate::weapons::WeaponDefReload, pull: bool) -> crate::gunreload::FeedBeat {
    FeedBeat { dur: r.dur, out: r.out.unwrap_or(r.dur), in_: r.in_.unwrap_or(r.dur), rack: r.rack.unwrap_or(r.dur), pull }
}

/// a made row as beat_of reads it (dur, out, in_, rack; `exact` is nothing to a beat)
fn beat_of_shell_row(r: &ShellRow, pull: bool) -> FeedBeat {
    beat_of(crate::weapons::WeaponDefReload { dur: r.dur, out: r.out, in_: r.in_, rack: r.rack, shell: r.shell }, pull)
}

/// a tube gun's reload as it stands now, by the shell; None: not a tube gun with `reload.shell` (gunreload.ts shellPlan)
pub fn shell_plan(g: &mut crate::game::Game, gun: FirearmId, free: Option<bool>, max: Option<f64>) -> Option<crate::gunreload::ShellPlan> {
    let free = free.unwrap_or(false);
    let max = max.unwrap_or(f64::INFINITY);
    let def = gun_def(g, gun);
    let s = def.reload.shell?;
    let f = feed_of(g, &def)?;
    if f.kind != "shells" {
        return None;
    }
    let tube = g.fa[gun].mag.unwrap_or(0.0);
    let port = f.port_first == Some(true);
    let have = if free || crate::weapons::firearm_bottomless(g, gun) {
        f64::INFINITY
    } else {
        g.fa[gun].spare.iter().fold(0.0, |a, b| a + *b)
    };
    let chamber = g.fa[gun].chamber;
    let (row, times, n, pull) = shell_row(s, def.mag_capacity, tube, have, max, port, chamber);
    Some(ShellPlan { row, times, n, port, pull, idle: n == 0.0 && !pull })
}

/// shellPlan's sums, past its reads of the game
fn shell_row(s: ShellRowShell, cap: f64, tube: f64, have: f64, max: f64, port: bool, chamber: f64) -> (ShellRow, Vec<f64>, f64, bool) {
    let n = m::max(0.0, m::min_n(&[cap - tube, have, max]));
    let first = port && chamber == 0.0 && tube == 0.0 && n > 0.0;
    let pull = chamber == 0.0 && !first && tube + n > 0.0;
    let last = s.lead + n * s.each;
    let row = if n > 0.0 {
        ShellRow { dur: last + s.tail, out: Some(s.lead), in_: Some(last), rack: if pull { Some(last + s.tail - s.bolt) } else { None }, shell: None, exact: true }
    } else {
        ShellRow { dur: s.bolt, out: Some(s.bolt * 0.05), in_: Some(s.bolt * 0.1), rack: Some(s.bolt * 0.5), shell: None, exact: true }
    };
    let times = shell_times(beat_of_shell_row(&row, pull), n);
    (row, times, n, pull)
}

/// a man whose reload is a beat begins one: the row it runs on, planned on the gun under the shells key (gunreload.ts feedBegin)
pub fn feed_begin(g: &mut crate::game::Game, gun: FirearmId) -> crate::weapons::WeaponDefReload {
    crate::weapons::firearm_cancel_reload(g, gun);
    let p = match shell_plan(g, gun, Some(true), None) {
        Some(p) if !p.idle => p,
        _ => return gun_def(g, gun).reload,
    };
    let row = seam_shell_row_on_gun(p.row);
    crate::weapons::firearm_plan_feed(g, gun, row, &p.times, p.port);
    row
}

/// one cue, played at him (gunreload.ts feedPlay)
pub fn feed_play(g: &mut crate::game::Game, at: V3, c_2: &crate::gunreload::FeedCue, vol: Option<f64>) {
    let vol = vol.unwrap_or(1.0);
    seam_audio_play(g, &c_2.sfx, at, c_2.gain * vol, c_2.rate);
}

/// his reload's clock went from `before` to `now`: the sounds of his gun's own feeding that fall between are played at him (gunreload.ts feedTick)
pub fn feed_tick(g: &mut crate::game::Game, c_2: CharId, def: &crate::weapons::WeaponDef, before: f64, now: f64, vol: Option<f64>, gun: Option<FirearmId>) {
    let vol = vol.unwrap_or(0.7);
    let Some(f) = feed_of(g, def) else { return };
    if f.kind == "mag" || !(now > before) {
        return;
    }
    let by = match gun {
        Some(id) if crate::weapons::firearm_shell_fed(g, id) => Some(id),
        _ => None,
    };
    let cues = if let Some(by) = by {
        crate::weapons::firearm_feed_to(g, by, now, None);
        let row = crate::weapons::firearm_reload_row(g, by);
        let (pull, n, port) = (g.fa[by].feed_pull, g.fa[by].feed_n, g.fa[by].feed_port);
        feed_cues(g, def, beat_of(row, pull), n, port)
    } else {
        feed_cues(g, def, beat_of(def.reload, true), def.mag_capacity, true)
    };
    let Some(cues) = cues else { return };
    for q in &cues {
        if q.t > before && q.t <= now {
            let at = g.ch[c_2].pos;
            feed_play(g, at, q, Some(vol));
        }
    }
}

/// what falls when his beat crosses `out`: the fill it is drawn by; None: nothing falls (gunreload.ts feedDrop)
pub fn feed_drop(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> Option<f64> {
    match feed_of(g, def) {
        Some(f) => f.drop,
        None => seam_drop_undefined(),
    }
}

/// his reload begins: this gun's first sound is its own, on its beat (gunreload.ts feedStarts)
pub fn feed_starts(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> bool {
    matches!(feed_of(g, def), Some(f) if f.kind != "mag")
}

/// the row's feeding, for the gun that is in his hands (gunreload.ts ownOf)
pub fn own_of(g: &mut crate::game::Game, c_2: CharId, id_2: crate::weapons::WeaponId) -> Option<crate::gunreload::FEEDValue> {
    let d = g.m.weapons.weapons.get(id_2)?.clone();
    if (d.cls == crate::weapons::WeaponClass::Pistol) != g.ch[c_2].anim.long_gun { feed_of(g, &d) } else { None }
}

/// the animator's clock as the beat the sums take
fn clock_beat(b: crate::character::AnimFeedClock) -> FeedBeat {
    FeedBeat { dur: b.dur, out: b.out, in_: b.in_, rack: b.rack, pull: b.pull }
}

/// what the drawer shows of a reload under way on this man's gun of row `id_2` (gunreload.ts reloadShow)
pub fn reload_show(g: &mut crate::game::Game, c_2: CharId, id_2: crate::weapons::WeaponId) -> Option<crate::gunreload::ReloadShow> {
    let f = own_of(g, c_2, id_2)?;
    let b = crate::character::anim_feed_clock(g, c_2)?;
    let t = b.t;
    if f.kind == "belt" {
        let q = belt_times(clock_beat(b));
        return Some(ReloadShow { no_mag: t >= q.off && t < q.on, cover: m::min(sstep(q.up, q.up + 0.18, t), 1.0 - sstep(q.down - 0.12, q.down, t)) });
    }
    if f.kind == "shells" {
        return Some(ReloadShow { no_mag: false, cover: 0.0 });
    }
    Some(ReloadShow { no_mag: t >= b.out && t < b.in_, cover: 0.0 })
}

/// (gunreload.ts sstep)
pub fn sstep(a: f64, b: f64, x: f64) -> f64 {
    let u = m::min(1.0, m::max(0.0, (x - a) / (b - a)));
    u * u * (3.0 - 2.0 * u)
}

/// the thing in his support hand this frame, as one box in the fist, appended to `out` (gunreload.ts reloadHandBoxes)
pub fn reload_hand_boxes(g: &mut crate::game::Game, c_2: CharId, id_2: crate::weapons::WeaponId, out: &mut Vec<BoxId>) {
    let Some(f) = own_of(g, c_2, id_2) else { return };
    let Some(b) = crate::character::anim_feed_clock(g, c_2) else { return };
    let t = b.t;
    let held = if f.kind == "belt" {
        let q = belt_times(clock_beat(b));
        t >= q.pouch + 0.04 && t < q.on
    } else if f.kind == "shells" {
        let n = g.ch[c_2].anim.feed_n;
        let per = if n > 0.0 { (b.in_ - b.out) / n } else { 0.0 };
        let half = m::min(per * 0.5, 0.3);
        shell_times(clock_beat(b), n).iter().any(|&at| t >= at - half && t < at)
    } else {
        (b.keep >= 0.0 && t >= b.out && t < b.keep) || (t >= b.fresh + 0.04 && t < b.in_)
    };
    if held {
        let kind = if f.hand[1] > 0.07 { crate::slotsetup::SetupGun::Rifle } else { crate::slotsetup::SetupGun::Pistol };
        crate::rig_props::mag_box_l(g, c_2, out, kind, Some(true), Some(f.hand), Some(f.col));
    }
}

// The steps the skeleton blocks (notes.md, Skeleton).

/// gunreload.ts:188: `g.audio.play(sfx, at, gain, { rate })`
fn seam_audio_play(g: &mut Game, sfx: &str, at: V3, gain: f64, rate: f64) {
    todo!("needs: game_audio_like_play's opts to hold the rate (JsMap<String, Opaque> cannot hold a number)")
}
/// gunreload.ts:201: the row has nothing of its own, `undefined`: the box that always fell, which the return type cannot tell from `null`
fn seam_drop_undefined() -> Option<f64> {
    todo!("needs: feedDrop -> Option<Option<f64>> (guards.ts:2824, :2826 tell undefined from null)")
}
/// gunreload.ts:185: the made row goes on the gun and is returned as `WeaponDef['reload']`, keeping `exact`
fn seam_shell_row_on_gun(row: ShellRow) -> crate::weapons::WeaponDefReload {
    todo!("needs: WeaponDefReload.exact (character.ts:2023 reads it off Firearm.reloadRow, game.ts:1911)")
}

#[cfg(test)]
mod tests {
    // Bits from the TypeScript under bun with EXACT.on, GUNLOOK.on and GUNLOOK.shells (gunreload.ts itself): FNV-1a 64 over the words in the order below.
    use super::*;
    use crate::weapons::GunModelId as M;

    struct H(u64);
    impl H {
        fn new() -> H { H(0xcbf29ce484222325) }
        fn w(&mut self, x: u64) { self.0 = (self.0 ^ x).wrapping_mul(0x100000001b3); }
        fn f(&mut self, x: f64) { self.w(x.to_bits()); }
        fn s(&mut self, t: &str) { for c in t.bytes() { self.w(c as u64); } }
    }
    fn b(dur: f64, out: f64, in_: f64, rack: f64, pull: bool) -> FeedBeat { FeedBeat { dur, out, in_, rack, pull } }
    fn long(pull: bool) -> FeedBeat { b(6.0, 1.2, 4.4, 5.4, pull) }
    fn m1014(pull: bool) -> FeedBeat { b(3.6, 0.4, 2.9, 3.3, pull) }
    fn m870() -> FeedBeat { b(3.2, 0.4, 2.5, 2.9, true) }

    fn path(model: M, beat: FeedBeat, n: f64, port: bool) -> (usize, u64) {
        let FeedPath::Arm0(p) = path_of(model, beat, n, port) else { panic!("keys") };
        let mut h = H::new();
        for k in &p.keys {
            h.f(k.t);
            match k.at { FeedKeyAt::Pouch => h.w(1), FeedKeyAt::List(a) => { h.f(a[0]); h.f(a[1]); h.f(a[2]); } }
            h.s(k.how.as_str());
            h.f(k.grip);
        }
        (p.keys.len(), h.0)
    }
    fn cues(model: M, beat: FeedBeat, n: f64, port: bool) -> (usize, u64) {
        let c = cues_of(feed_by(model), beat, n, port).unwrap();
        let mut h = H::new();
        for q in &c { h.f(q.t); h.s(&q.sfx); h.f(q.gain); h.f(q.rate); }
        (c.len(), h.0)
    }
    fn row(s: ShellRowShell, cap: f64, tube: f64, have: f64, max: f64, port: bool, chamber: f64) -> (f64, bool, u64) {
        let (r, times, n, pull) = shell_row(s, cap, tube, have, max, port, chamber);
        let mut h = H::new();
        h.f(r.dur); h.f(r.out.unwrap()); h.f(r.in_.unwrap());
        match r.rack { None => h.w(1), Some(x) => h.f(x) }
        for t in &times { h.f(*t); }
        h.f(n);
        (n, pull, h.0)
    }
    const S1014: ShellRowShell = ShellRowShell { lead: 0.4, each: 0.36, tail: 0.7, bolt: 0.4 };
    const S870: ShellRowShell = ShellRowShell { lead: 0.4, each: 0.35, tail: 0.7, bolt: 0.4 };
    const INF: f64 = f64::INFINITY;

    #[test]
    fn belt_moments_to_the_bit() {
        let t = belt_times(long(true));
        assert_eq!([t.up, t.off, t.pouch, t.on, t.lay, t.down, t.charge].map(f64::to_bits),
            [0x3fe147ae147ae148, 0x3ff3333333333333, 0x400189374bc6a7f0, 0x400978d4fdf3b646, 0x401199999999999a, 0x401399999999999a, 0x401599999999999a]);
        let t = belt_times(b(1.0, 0.2, 0.4, 0.6, true));
        assert_eq!([t.up, t.off, t.pouch, t.on, t.lay, t.down, t.charge].map(f64::to_bits),
            [0x3fb999999999999a, 0x3fc999999999999a, 0x3fd0c49ba5e353f8, 0x3fd4bc6a7ef9db23, 0x3fd999999999999a, 0x3fe0a3d70a3d70a4, 0x3fe3333333333333]);
    }
    #[test]
    fn shell_moments_to_the_bit() {
        let t: Vec<u64> = shell_times(m1014(false), 7.0).iter().map(|x| x.to_bits()).collect();
        assert_eq!(t, vec![0x3fe83a83a83a83a8, 0x3ff1d41d41d41d42, 0x3ff78af8af8af8b0, 0x3ffd41d41d41d41e, 0x40017c57c57c57c6, 0x400457c57c57c57c, 0x4007333333333333]);
    }
    #[test]
    fn the_hands_stations_to_the_bit() {
        assert_eq!(path(M::Mk46, long(true), 100.0, true), (19, 0xadfd59b69901a0d9));
        assert_eq!(path(M::Mk46, long(false), 100.0, false), (15, 0xc65ea4df60786530));
        assert_eq!(path(M::Mk46, b(1.0, 0.2, 0.4, 0.6, true), 100.0, true), (19, 0xaba8c561e062a73d));
        assert_eq!(path(M::Mk46, b(0.3, 0.1, 0.15, 0.2, true), 100.0, true), (2, 0x7cd7590eaf653c01));   // (the clamp and the filter against the list before filtering)
        assert_eq!(path(M::M1014, m1014(false), 7.0, true), (28, 0x8f9d373a53fe2993));
        assert_eq!(path(M::M1014, m1014(true), 3.0, false), (14, 0x08a6d9a54577785f));
        assert_eq!(path(M::M1014, m1014(false), 1.0, true), (4, 0x6c4358ce7d8e3729));
        assert_eq!(path(M::M870, m870(), 6.0, true), (24, 0x4815b5e6cdbc9ba5));
        assert_eq!(path(M::M870, b(0.4, 0.02, 0.04, 0.2, true), 0.0, false), (0, 0xcbf29ce484222325));
    }
    #[test]
    fn a_magazine_guns_marks_to_the_bit() {
        let FeedPath::Arm1(p) = path_of(M::Mp7, long(true), 40.0, false) else { panic!("marks") };
        let k = p.marks.unwrap();
        let mut h = H::new();
        for a in [k.mag, k.strip, k.handle, k.pull, k.hf, k.hb] { for x in a { h.f(x); } }
        assert_eq!(h.0, 0xa47a5d23edc22763);
        assert!(matches!(path_of(M::Mk12, long(true), 20.0, false), FeedPath::Arm1(FeedPathArm1 { marks: None })));
    }
    #[test]
    fn what_is_heard_to_the_bit() {
        assert_eq!(cues(M::Mk46, long(true), 100.0, true), (6, 0x29406aaec1a02ff1));
        assert_eq!(cues(M::Mk46, long(false), 100.0, false), (5, 0x990d3607ce5b8dc6));
        assert_eq!(cues(M::M1014, m1014(false), 7.0, true), (8, 0xd7ca8903e88dcbe2));
        assert_eq!(cues(M::M1014, m1014(false), 1.0, true), (2, 0x845f8f4144057e20));
        assert_eq!(cues(M::M1014, m1014(true), 3.0, false), (4, 0x99765c96ca3e30b5));
        assert_eq!(cues(M::M870, m870(), 6.0, true), (7, 0xfb2dc0ae1c2ae76d));
        assert!(cues_of(feed_by(M::G18), m870(), 33.0, true).is_none());
    }
    #[test]
    fn a_tube_guns_row_to_the_bit() {
        assert_eq!(row(S1014, 7.0, 0.0, 21.0, INF, true, 0.0), (7.0, false, 0x2c6b80dd321606bd));
        assert_eq!(row(S1014, 7.0, 3.0, 2.0, INF, true, 1.0), (2.0, false, 0x8b0403987b60d167));
        assert_eq!(row(S870, 6.0, 0.0, INF, INF, false, 0.0), (6.0, true, 0x9f7fae51a19f2584));
        assert_eq!(row(S870, 6.0, 2.0, 6.0, 0.0, false, 0.0), (0.0, true, 0x749ac9880845266f));
        assert_eq!(row(S870, 6.0, 6.0, 6.0, INF, false, 1.0), (0.0, false, 0x749ac9880845266f));
    }
}
