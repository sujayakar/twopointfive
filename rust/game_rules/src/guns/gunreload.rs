pub use ::game_types::gunreload::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::V3;
use alloc::vec::Vec;

/// how this row is fed right now; None: a magazine change as it always was
pub fn feed_of(def: &crate::weapons::WeaponDef) -> Option<crate::gunreload::FEEDValue> {
    crate::gunlook::gun_own(def).map(|o| feed_by(o.model))
}

/// how each model is fed
fn feed_by(m: crate::weapons::GunModelId) -> crate::gunreload::FEEDValue {
    feed().of(m.as_str()).unwrap().clone()
}

/// the belt's moments inside a beat
fn belt_times(b: crate::gunreload::FeedBeat) -> crate::gunreload::BeltTimes {
    let o = b.out;
    let i = b.in_;
    let end = if b.pull { b.rack } else { b.dur - 0.3 };
    let up = m::max(0.1, o * 0.45);
    let on = o + (i - o) * 0.62;
    let down = i + m::min(0.5, m::max(0.12, (end - i) * 0.5));
    BeltTimes { up, off: o, pouch: o + (on - o) * 0.5, on, lay: i, down, charge: if b.pull { b.rack } else { -1.0 } }
}

/// the shells' moments: `n` of them, the last home at the beat's `in_`
fn shell_times(b: crate::gunreload::FeedBeat, n: f64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut k = 0.0;
    while k < n {
        out.push(b.out + (k + 1.0) * (b.in_ - b.out) / n);
        k += 1.0;
    }
    out
}

/// a shell's own pitch, dealt off its place in the tube
fn shell_rate(k: f64) -> f64 {
    0.94 + js::rem(k * 5.0, 7.0) * 0.02
}

fn cue(t: f64, sfx: crate::audio_iface::Sfx, gain: f64, rate: f64) -> FeedCue {
    FeedCue { t, sfx, gain, rate }
}

/// what is heard, in order, of a reload of this row on this beat; None: a magazine gun
fn feed_cues(def: &crate::weapons::WeaponDef, b: crate::gunreload::FeedBeat, n: f64, port: bool) -> Option<Vec<crate::gunreload::FeedCue>> {
    let f = feed_of(def)?;
    cues_of(f, b, n, port)
}

/// feedCues past its one read of the game
fn cues_of(f: FEEDValue, b: FeedBeat, n: f64, port: bool) -> Option<Vec<FeedCue>> {
    if f.kind == FeedKind::Mag {
        return None;
    }
    let mut out = Vec::new();
    if f.kind == FeedKind::Belt {
        let t = belt_times(b);
        out.push(cue(t.up, crate::audio_iface::Sfx::BeltCoverUp, 0.8, 1.0));
        out.push(cue(t.off, crate::audio_iface::Sfx::BeltBoxOff, 0.8, 1.0));
        out.push(cue(t.on, crate::audio_iface::Sfx::BeltBoxOn, 0.85, 1.0));
        out.push(cue(t.lay, crate::audio_iface::Sfx::BeltLay, 0.8, 1.0));
        out.push(cue(t.down, crate::audio_iface::Sfx::BeltCoverDown, 0.9, 1.0));
        if b.pull {
            out.push(cue(t.charge, crate::audio_iface::Sfx::ChargeLmg, 0.9, 1.0));
        }
    } else {
        let t = shell_times(b, n);
        let first = port && f.port_first == Some(true);
        let mut k = 0.0;
        let mut i = 0usize;
        while k < n {
            let head = first && i == 0;
            out.push(cue(t[i], crate::audio_iface::Sfx::ShellIn, if head { 0.6 } else { 0.8 }, shell_rate(k)));
            if head {
                out.push(cue(m::min(t[0] + 0.12, if n > 1.0 { t[1] - 0.02 } else { b.in_ + 0.12 }), crate::audio_iface::Sfx::BoltDrop, 0.9, 1.0));
            }
            k += 1.0;
            i += 1;
        }
        if b.pull && !first {
            out.push(if f.pump == Some(true) { cue(b.rack, crate::audio_iface::Sfx::Rack, 0.85, 0.7) } else { cue(b.rack, crate::audio_iface::Sfx::BoltDrop, 0.9, 1.0) });
        }
    }
    js::sort_by(&mut out, |x, y| x.t - y.t);
    Some(out)
}

fn put(k: &mut Vec<FeedKey>, t: f64, at: FeedKeyAt, how: FeedKeyHow, grip: f64) {
    k.push(FeedKey { t, at, how, grip });
}

/// where the support hand is over a reload of this model: its stations, or a magazine gun's own marks
pub fn feed_path(model: crate::weapons::GunModelId, b: crate::gunreload::FeedBeat, n: f64, port: bool) -> Option<crate::gunreload::FeedPath> {
    Some(path_of(model, b, n, port))
}

/// feedPath past the switch (GUNLOOK.on is settled on)
fn path_of(model: crate::weapons::GunModelId, b: FeedBeat, n: f64, port: bool) -> FeedPath {
    let f = feed_by(model);
    if f.kind == FeedKind::Mag {
        return FeedPath::Arm1(FeedPathArm1 { marks: f.marks });
    }
    use FeedKeyAt::{List, Pouch};
    let (mag, top) = (FeedKeyHow::Mag, FeedKeyHow::Top);
    let mut k: Vec<FeedKey> = Vec::new();
    if f.kind == FeedKind::Belt {
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

/// dealt to a man's animator: the long gun in his hands or None, how the support hand is to feed it
pub fn feed_deal(g: &mut crate::game::Game, ch: CharId, gun: Option<FirearmId>, dry: Option<bool>) {
    let dry = dry.unwrap_or(false);
    let o = match gun {
        Some(id) => {
            let def = g.weapon(g.fa[id].def).clone();
            crate::gunlook::gun_own(&def)
        }
        None => None,
    };
    let (Some(gun), Some(o)) = (gun, o) else {
        g.ch[ch].anim.feed = None;
        return;
    };
    let plain = dry && !crate::weapons::firearm_shell_fed(g, gun);
    let (n, port) = if plain {
        (g.weapon(g.fa[gun].def).mag_capacity, true)
    } else {
        (g.fa[gun].feed_n, g.fa[gun].feed_port)
    };
    let an = &mut g.ch[ch].anim;
    an.feed = Some(o.model);
    an.feed_n = n;
    an.feed_port = port;
}

/// what the player's reload of `gun` on `row` is to sound like and to drop, asked before startReload
pub fn feed_plan(g: &crate::game::Game, gun: FirearmId, row: crate::weapons::WeaponDefReload, shells: Option<crate::gunreload::FeedPlanShells>) -> Option<crate::gunreload::FeedPlan> {
    let def = g.weapon(g.fa[gun].def).clone();
    let f = feed_of(&def)?;
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
    let cues = feed_cues(&def, beat_of(row, pull), n, port);
    let times = match &cues {
        Some(c) => c.iter().map(|q| q.t).collect(),
        None => Vec::new(),
    };
    Some(FeedPlan { cues, times, drop: f.drop })
}

/// feedPlan's `best`: the same sums as startReload's, the fullest spare, as much of it as the gun may hold
fn plan_best(g: &Game, gun: FirearmId, def: &crate::weapons::WeaponDef) -> f64 {
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

/// a weapon's reload row as a beat; a row that gives only its length does everything at the end
fn beat_of(r: crate::weapons::WeaponDefReload, pull: bool) -> crate::gunreload::FeedBeat {
    FeedBeat { dur: r.dur, out: r.out.unwrap_or(r.dur), in_: r.in_.unwrap_or(r.dur), rack: r.rack.unwrap_or(r.dur), pull }
}

/// a tube gun's reload as it stands now, by the shell; None: not a tube gun with `reload.shell`
pub fn shell_plan(g: &crate::game::Game, gun: FirearmId, free: Option<bool>, max: Option<f64>) -> Option<crate::gunreload::ShellPlan> {
    let free = free.unwrap_or(false);
    let max = max.unwrap_or(f64::INFINITY);
    let def = g.weapon(g.fa[gun].def).clone();
    let s = def.reload.shell?;
    let f = feed_of(&def)?;
    if f.kind != FeedKind::Shells {
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
    let times = shell_times(beat_of(crate::player::shell_row_as_weapon_row(&row), pull), n);
    (row, times, n, pull)
}

/// a man whose reload is a beat begins one: the row it runs on, planned on the gun under the shells key
pub fn feed_begin(g: &mut crate::game::Game, gun: FirearmId) -> crate::weapons::WeaponDefReload {
    crate::weapons::firearm_cancel_reload(g, gun);
    let p = match shell_plan(g, gun, Some(true), None) {
        Some(p) if !p.idle => p,
        _ => return g.weapon(g.fa[gun].def).reload,
    };
    let row = crate::player::shell_row_as_weapon_row(&p.row);
    crate::weapons::firearm_plan_feed(g, gun, row, &p.times, p.port);
    row
}

/// one cue, played at him
pub(super) fn feed_play(g: &mut crate::game::Game, at: V3, c_2: &crate::gunreload::FeedCue, vol: Option<f64>) {
    let vol = vol.unwrap_or(1.0);
    crate::audio_iface::game_audio_like_play(g, c_2.sfx, Some(at), Some(c_2.gain * vol), Some(&crate::audio_iface::PlayOpts::Rate { rate: c_2.rate }));
}

/// his reload's clock went from `before` to `now`: the sounds of his gun's own feeding that fall between are played at him
pub fn feed_tick(g: &mut crate::game::Game, c_2: CharId, def: &crate::weapons::WeaponDef, before: f64, now: f64, vol: Option<f64>, gun: Option<FirearmId>) {
    let vol = vol.unwrap_or(0.7);
    let Some(f) = feed_of(def) else { return };
    if f.kind == FeedKind::Mag || !(now > before) {
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
        feed_cues(def, beat_of(row, pull), n, port)
    } else {
        feed_cues(def, beat_of(def.reload, true), def.mag_capacity, true)
    };
    let Some(cues) = cues else { return };
    for q in &cues {
        if q.t > before && q.t <= now {
            let at = g.ch[c_2].pos;
            feed_play(g, at, q, Some(vol));
        }
    }
}

/// what falls when his beat crosses `out`: the fill it is drawn by; Some(None): nothing falls; None (`undefined`): the row has nothing of its own, the box that always fell
pub fn feed_drop(def: &crate::weapons::WeaponDef) -> Option<Option<f64>> {
    feed_of(def).map(|f| f.drop)
}

/// his reload begins: this gun's first sound is its own, on its beat
pub fn feed_starts(def: &crate::weapons::WeaponDef) -> bool {
    matches!(feed_of(def), Some(f) if f.kind != FeedKind::Mag)
}

/// the row's feeding, for the gun that is in his hands
fn own_of(g: &crate::game::Game, c_2: CharId, id_2: crate::weapons::WeaponId) -> Option<crate::gunreload::FEEDValue> {
    let d = g.m.weapons.weapons.get(id_2)?.clone();
    if (d.cls == crate::weapons::WeaponClass::Pistol) != g.ch[c_2].anim.long_gun { feed_of(&d) } else { None }
}

/// the animator's clock as the beat the sums take
fn clock_beat(b: crate::character::AnimFeedClock) -> FeedBeat {
    FeedBeat { dur: b.dur, out: b.out, in_: b.in_, rack: b.rack, pull: b.pull }
}

/// what the drawer shows of a reload under way on this man's gun of row `id_2`
pub fn reload_show(g: &crate::game::Game, c_2: CharId, id_2: crate::weapons::WeaponId) -> Option<crate::gunreload::ReloadShow> {
    let f = own_of(g, c_2, id_2)?;
    let b = crate::character::anim_feed_clock(g, c_2)?;
    let t = b.t;
    if f.kind == FeedKind::Belt {
        let q = belt_times(clock_beat(b));
        return Some(ReloadShow { no_mag: t >= q.off && t < q.on, cover: m::min(sstep(q.up, q.up + 0.18, t), 1.0 - sstep(q.down - 0.12, q.down, t)) });
    }
    if f.kind == FeedKind::Shells {
        return Some(ReloadShow { no_mag: false, cover: 0.0 });
    }
    Some(ReloadShow { no_mag: t >= b.out && t < b.in_, cover: 0.0 })
}

fn sstep(a: f64, b: f64, x: f64) -> f64 {
    let u = m::min(1.0, m::max(0.0, (x - a) / (b - a)));
    u * u * (3.0 - 2.0 * u)
}

/// the thing in his support hand this frame, as one box in the fist, appended to `out`
pub fn reload_hand_boxes(g: &crate::game::Game, c_2: CharId, id_2: crate::weapons::WeaponId, out: &mut crate::rig_props::Carried) {
    let Some(f) = own_of(g, c_2, id_2) else { return };
    let Some(b) = crate::character::anim_feed_clock(g, c_2) else { return };
    let t = b.t;
    let held = if f.kind == FeedKind::Belt {
        let q = belt_times(clock_beat(b));
        t >= q.pouch + 0.04 && t < q.on
    } else if f.kind == FeedKind::Shells {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weapons::GunModelId as M;

    fn b(dur: f64, out: f64, in_: f64, rack: f64, pull: bool) -> FeedBeat { FeedBeat { dur, out, in_, rack, pull } }
    fn long(pull: bool) -> FeedBeat { b(6.0, 1.2, 4.4, 5.4, pull) }
    fn m1014(pull: bool) -> FeedBeat { b(3.6, 0.4, 2.9, 3.3, pull) }
    fn m870() -> FeedBeat { b(3.2, 0.4, 2.5, 2.9, true) }
    fn near(a: f64, b: f64) -> bool { (a - b).abs() < 1e-9 }

    /// A belt change goes cover up, box off, new box from the pouch, box on, belt laid, cover down, and only a dry gun is charged last.
    #[test]
    fn a_belt_change_runs_in_the_order_a_gunner_works() {
        let t = belt_times(long(true));
        let order = [t.up, t.off, t.pouch, t.on, t.lay, t.down, t.charge];
        assert!(order.windows(2).all(|w| w[0] < w[1]), "{:?}", order);
        assert_eq!((t.off, t.lay, t.charge), (1.2, 4.4, 5.4));
        assert_eq!(belt_times(long(false)).charge, -1.0, "a gun with a round up is not charged");
        // however short the beat, the cover lifts a beat in, not at the start
        assert_eq!(belt_times(b(1.0, 0.2, 0.4, 0.6, true)).up, 0.1);
    }

    /// The shells go in evenly, one by one, and the last is home at the beat's `in_`.
    #[test]
    fn shells_go_in_evenly_and_the_last_is_home_on_the_beat() {
        let beat = m1014(false);
        let t = shell_times(beat, 7.0);
        assert_eq!(t.len(), 7);
        let per = (beat.in_ - beat.out) / 7.0;
        assert!(near(t[0], beat.out + per));
        assert!(t.windows(2).all(|w| near(w[1] - w[0], per)));
        assert!(near(t[6], beat.in_));
        assert!(shell_times(beat, 0.0).is_empty());
    }

    fn keys(model: M, beat: FeedBeat, n: f64, port: bool) -> Vec<FeedKey> {
        let FeedPath::Arm0(p) = path_of(model, beat, n, port) else { panic!("a magazine gun has no stations") };
        p.keys
    }

    /// The hand never goes back in time or holds two places at once, and is done a tenth of a second before the beat ends, however short the beat.
    #[test]
    fn the_hand_keeps_time_inside_the_beat() {
        let beats = [
            (M::Mk46, long(true), 100.0, true), (M::Mk46, long(false), 100.0, false), (M::Mk46, b(1.0, 0.2, 0.4, 0.6, true), 100.0, true),
            (M::Mk46, b(0.3, 0.1, 0.15, 0.2, true), 100.0, true), (M::M1014, m1014(false), 7.0, true), (M::M1014, m1014(true), 3.0, false),
            (M::M1014, m1014(false), 1.0, true), (M::M870, m870(), 6.0, true),
        ];
        for (model, beat, n, port) in beats {
            let k = keys(model, beat, n, port);
            assert!(!k.is_empty());
            assert!(k.windows(2).all(|w| w[0].t < w[1].t), "{:?} out of order", model);
            assert!(k[0].t >= 0.08 - 1e-9 && k[k.len() - 1].t <= beat.dur - 0.1 + 1e-9, "{:?} {} to {}", model, k[0].t, k[k.len() - 1].t);
        }
        assert_eq!(keys(M::Mk46, b(0.3, 0.1, 0.15, 0.2, true), 100.0, true).len(), 2, "a beat too short for the belt's stations still ends in time");
        assert!(keys(M::M870, b(0.4, 0.02, 0.04, 0.2, true), 0.0, false).is_empty(), "no shells: the hand stays home");
    }

    /// The hand goes to the pouch once on a belt change, reaches for the charging handle only on a dry gun.
    #[test]
    fn a_belts_hand_goes_to_the_pouch_and_only_a_dry_gun_to_the_handle() {
        let s = belt();
        let to_handle = |k: &[FeedKey]| k.iter().any(|q| matches!(q.at, FeedKeyAt::List(a) if a == s.handle));
        let dry = keys(M::Mk46, long(true), 100.0, true);
        assert!(dry.iter().any(|q| matches!(q.at, FeedKeyAt::Pouch)));
        assert!(to_handle(&dry));
        assert!(!to_handle(&keys(M::Mk46, long(false), 100.0, false)));
    }

    /// The first shell of an empty M1014 goes into the open port, and the bolt then drops on it, so the hand has no bolt to work at the end. A pump gun's fore end is worked by the other arm.
    #[test]
    fn a_tube_guns_hand_goes_to_the_port_only_when_it_is_empty() {
        let s = tube();
        let at_eject = |k: &[FeedKey]| k.iter().filter(|q| matches!(q.at, FeedKeyAt::List(a) if a == s.eject)).count();
        let from_empty = keys(M::M1014, m1014(true), 3.0, true);
        assert_eq!(at_eject(&from_empty), 2, "the head shell's two stations, and no bolt");
        assert_eq!(at_eject(&keys(M::M1014, m1014(false), 3.0, true)), 2, "the head shell goes in at the port");
        assert_eq!(at_eject(&keys(M::M1014, m1014(true), 3.0, false)), 2, "no shell at the port, the bolt pulled at the end");
        assert_eq!(at_eject(&keys(M::M1014, m1014(false), 3.0, false)), 0);
        assert_eq!(at_eject(&keys(M::M870, m870(), 6.0, true)), 0);
    }

    /// A magazine gun's hand follows its own marks where it has them, and otherwise the old magazine change.
    #[test]
    fn a_magazine_gun_has_its_marks_or_none() {
        let FeedPath::Arm1(p) = path_of(M::Mp7, long(true), 40.0, false) else { panic!("marks") };
        assert!(p.marks.is_some());
        assert!(matches!(path_of(M::Mk12, long(true), 20.0, false), FeedPath::Arm1(FeedPathArm1 { marks: None })));
    }

    fn heard(model: M, beat: FeedBeat, n: f64, port: bool) -> Vec<FeedCue> { cues_of(feed_by(model), beat, n, port).unwrap() }
    fn said(c: &[FeedCue]) -> Vec<crate::audio_iface::Sfx> { c.iter().map(|q| q.sfx).collect() }

    /// What is heard of a belt change is the belt change, and in order, with the charge only on a dry gun.
    #[test]
    fn a_belt_change_is_heard_in_order() {
        let dry = heard(M::Mk46, long(true), 100.0, true);
        assert_eq!(said(&dry), [crate::audio_iface::Sfx::BeltCoverUp, crate::audio_iface::Sfx::BeltBoxOff, crate::audio_iface::Sfx::BeltBoxOn, crate::audio_iface::Sfx::BeltLay, crate::audio_iface::Sfx::BeltCoverDown, crate::audio_iface::Sfx::ChargeLmg]);
        assert!(dry.windows(2).all(|w| w[0].t <= w[1].t));
        assert_eq!(said(&heard(M::Mk46, long(false), 100.0, false)), [crate::audio_iface::Sfx::BeltCoverUp, crate::audio_iface::Sfx::BeltBoxOff, crate::audio_iface::Sfx::BeltBoxOn, crate::audio_iface::Sfx::BeltLay, crate::audio_iface::Sfx::BeltCoverDown]);
    }

    /// A shell is heard for each shell, each a little different in pitch. The M1014's first into the open port is softer and the bolt drops after it, once. Otherwise a dry gun is worked at the end: the bolt, or the pump, slower.
    #[test]
    fn shells_are_heard_one_by_one_and_then_the_bolt_or_the_pump() {
        let c = heard(M::M1014, m1014(false), 7.0, true);
        assert_eq!(said(&c), [crate::audio_iface::Sfx::ShellIn, crate::audio_iface::Sfx::BoltDrop, crate::audio_iface::Sfx::ShellIn, crate::audio_iface::Sfx::ShellIn, crate::audio_iface::Sfx::ShellIn, crate::audio_iface::Sfx::ShellIn, crate::audio_iface::Sfx::ShellIn, crate::audio_iface::Sfx::ShellIn]);
        assert!(c[0].gain < c[2].gain, "the shell at the port is softer than the ones into the tube");
        assert!(c[0].t < c[1].t && c[1].t < c[2].t, "the bolt drops between the first shell and the second");
        assert!(c[0].rate != c[2].rate);
        // the only shell: the bolt still drops after it
        assert_eq!(said(&heard(M::M1014, m1014(false), 1.0, true)), [crate::audio_iface::Sfx::ShellIn, crate::audio_iface::Sfx::BoltDrop]);
        // a shell into the tube and a dry gun: the bolt goes last
        let c = heard(M::M1014, m1014(true), 3.0, false);
        assert_eq!(said(&c), [crate::audio_iface::Sfx::ShellIn, crate::audio_iface::Sfx::ShellIn, crate::audio_iface::Sfx::ShellIn, crate::audio_iface::Sfx::BoltDrop]);
        assert_eq!(c[3].t, m1014(true).rack);
        // the pump, at its own slower rate
        let c = heard(M::M870, m870(), 6.0, true);
        assert_eq!(said(&c[6..]), [crate::audio_iface::Sfx::Rack]);
        assert_eq!((c[6].t, c[6].rate), (m870().rack, 0.7));
        assert!(cues_of(feed_by(M::G18), m870(), 33.0, true).is_none(), "a magazine gun has no cues of its own");
    }

    const S1014: ShellRowShell = ShellRowShell { lead: 0.4, each: 0.36, tail: 0.7, bolt: 0.4 };
    const S870: ShellRowShell = ShellRowShell { lead: 0.4, each: 0.35, tail: 0.7, bolt: 0.4 };
    const INF: f64 = f64::INFINITY;

    /// He loads what the tube has room for, what he has and what he was told, the least of the three, and the beat is as long as those shells.
    #[test]
    fn a_tube_takes_the_least_of_its_room_his_shells_and_the_order() {
        assert_eq!(shell_row(S1014, 7.0, 0.0, 21.0, INF, true, 0.0).2, 7.0, "room");
        assert_eq!(shell_row(S1014, 7.0, 3.0, 2.0, INF, true, 1.0).2, 2.0, "his shells");
        assert_eq!(shell_row(S870, 6.0, 0.0, INF, 4.0, false, 0.0).2, 4.0, "the order");
        assert_eq!(shell_row(S870, 6.0, 7.0, INF, INF, false, 1.0).2, 0.0, "never a negative count");
        let (r, times, n, _) = shell_row(S870, 6.0, 0.0, INF, INF, false, 0.0);
        assert_eq!(times.len(), n as usize);
        let last = S870.lead + n * S870.each;
        assert_eq!((r.dur, r.out, r.in_), (last + S870.tail, Some(S870.lead), Some(last)));
        assert!(near(times[times.len() - 1], last));
    }

    /// A dry gun has to be worked unless the first shell goes in at the port. An empty gun with nothing to load can't be, and a full chamber never is.
    #[test]
    fn a_dry_tube_gun_is_worked_unless_a_shell_went_in_at_the_port() {
        assert!(!shell_row(S1014, 7.0, 0.0, 21.0, INF, true, 0.0).3, "the shell at the port chambers it");
        assert!(shell_row(S870, 6.0, 0.0, INF, INF, false, 0.0).3, "no port: the pump");
        assert!(shell_row(S1014, 7.0, 2.0, 3.0, INF, true, 0.0).3, "shells in the tube: the port is shut");
        assert!(!shell_row(S1014, 7.0, 2.0, 3.0, INF, true, 1.0).3, "one chambered");
        assert!(!shell_row(S870, 6.0, 0.0, 0.0, INF, false, 0.0).3, "nothing to chamber");
        let (r, _, _, pull) = shell_row(S870, 6.0, 0.0, INF, INF, false, 0.0);
        assert_eq!((pull, r.rack), (true, Some(r.dur - S870.bolt)));
        assert_eq!(shell_row(S870, 6.0, 0.0, INF, INF, true, 1.0).0.rack, None, "no bolt to work on the row when none is due");
    }

    /// Nothing to load but a round to work: the beat is the bolt alone.
    #[test]
    fn a_full_tube_with_a_dry_chamber_is_only_the_bolt() {
        let (r, times, n, pull) = shell_row(S870, 6.0, 6.0, 6.0, INF, false, 0.0);
        assert_eq!((n, pull, r.dur), (0.0, true, S870.bolt));
        assert!(times.is_empty());
        assert!(r.out.unwrap() < r.in_.unwrap() && r.in_.unwrap() < r.rack.unwrap() && r.rack.unwrap() < r.dur);
    }
}
