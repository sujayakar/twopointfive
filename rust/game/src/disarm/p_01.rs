//! src/game/disarm.ts lines 1 to 138
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// (disarm.ts lerp)
pub fn lerp(a: f64, b: f64, t: f64) -> f64 { a + (b - a) * t }

/// (disarm.ts sstep)
pub fn sstep(a: f64, b: f64, t: f64) -> f64 {
    let u = m::min(1.0, m::max(0.0, (t - a) / (b - a)));
    u * u * (3.0 - 2.0 * u)
}

/// player.ts startGrab, once the hold exists: the gun still in his hand is the disarm's to take. (disarm.ts disarmBegin)
pub fn disarm_begin(g: &mut crate::game::Game) {
    let pl = g.player;
    let Some((gd, variant)) = g.pl[pl].holding.as_ref().map(|h| (h.g, h.variant)) else { return };
    if crate::guards::in_hand(g, gd) != Some(crate::guards::InHand::Weapon) { return; }
    let def = g.fa[g.gd[gd].weapon].def;
    let (kind, supp) = { let row = g.m.weapons.weapons.get(def).unwrap(); (row.drop, row.suppressed) };
    let at = hand_of(g, gd);
    let yaw = v3::yaw_of(g.ch[g.gd[gd].ch].gun_dir);
    let strip = Strip { t: 0.0, kind, supp, at, item: None, rap: variant == crate::character::HoldVariant::Gun, gone: false, yaw };
    put_strip(g, pl, strip);
    if leads(g, Some(strip)) { g.pl[pl].catch_due = Some(g.time + g.k.hold.first.catch); }
}

/// The grab's first voice (player.ts startGrab, every grab): the strip is heard first when the disarm leads. Its rate is pinned. (disarm.ts disarmCatch)
pub fn disarm_catch(g: &mut crate::game::Game, at: V3) {
    let s = g.pl[g.player].holding.as_ref().and_then(|h| h.strip);
    if leads(g, s) { return; }
    seam_audio_play_rate(g, "grabCatch", at, disarm_gain().catch, 1.0);
}

/// The disarm leads for this move: a gun to take with an empty hand, not the pistol hold's rap. (disarm.ts leads)
pub fn leads(g: &mut crate::game::Game, s: Option<crate::disarm::Strip>) -> bool {
    match s { Some(s) => !s.rap, None => false }
}

/// The rate the move's clock runs at: ahead of the hold's until the let-go, the hold's own pace from there. (disarm.ts lead)
pub fn lead(g: &mut crate::game::Game, s: crate::disarm::Strip) -> f64 {
    if !leads(g, Some(s)) { return 1.0; }
    let (k_take, k_drop) = (g.k.hold.strip.take, g.k.hold.strip.drop);
    let (f_take, f_drop) = (g.k.hold.first.take, g.k.hold.first.drop);
    if s.t < k_take { k_take / m::max(0.02, f_take) }
    else if s.t < k_drop { (k_drop - k_take) / m::max(0.02, f_drop - f_take) }
    else { 1.0 }
}

/// What both figures' hold layers are told of it this frame (player.ts holdTail). (disarm.ts disarmPose)
pub fn disarm_pose(s: crate::disarm::Strip) -> crate::character::StripPose {
    crate::character::StripPose { t: s.t, at: Some(s.at) }
}

/// Sam's right fist is the gun's, not yet the knife's (combat.ts handProps). (disarm.ts disarmHand)
pub fn disarm_hand(g: &mut crate::game::Game, h: &crate::player::Hold) -> bool {
    match h.strip { Some(s) => s.t < g.k.hold.strip.knife, None => false }
}

/// Once a frame, after everybody has been posed (game.ts): the move's clock, the take, the gun carried, the gun let go. (disarm.ts disarmTick)
pub fn disarm_tick(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let pl_ch = g.pl[pl].ch;
    // `h` is a copy of the hold: only its guard and phase are read from it, which nothing below writes. `s` is a copy of the move, put back after each write.
    let h = g.pl[pl].holding.clone();
    let s0 = h.as_ref().and_then(|h| h.strip);
    let mut i = 0;
    while i < g.items.items.len() {
        let it = g.items.items[i];
        i += 1;
        // a gun in his hand and no hold to carry it for, however that ended, falls from where it is
        if crate::throwables::item_carried(g, it) == Some(true)
            && Some(it) != s0.and_then(|s| s.item)
            && (g.seats.len() == 1 || !any_man_carries(g, it))
        {
            let_go(g, it, pl_ch, None);
        }
    }
    // the catch's voice as the arm lands, or now if the hold or the move ended first: a grab is never silent
    if let Some(due) = g.pl[pl].catch_due {
        let not_grab = match h.as_ref() { Some(h) => h.phase != crate::character::HoldPhase::Grab, None => false };
        if h.is_none() || s0.is_none() || g.time >= due || not_grab {
            g.pl[pl].catch_due = None;
            let c = match h.as_ref() { Some(h) => g.gd[h.g].ch, None => pl_ch };
            let at = v3::add(g.ch[c].pos, [0.0, 1.4, 0.0]);
            seam_audio_play_rate(g, "grabCatch", at, disarm_gain().catch, 1.0);
        }
    }
    let (Some(h), Some(mut s)) = (h, s0) else { return };
    let gd = h.g;
    // a choke or the knife begun at once: the hand is home in half the time
    let hurry = if s.gone && h.phase != crate::character::HoldPhase::Grab && h.phase != crate::character::HoldPhase::Held { 2.0 } else { 1.0 };
    s.t += dt * lead(g, s) * hurry;
    put_strip(g, pl, s);
    if s.item.is_none() && !s.gone {
        // something else has had it out of his hand since the grab: nothing left to take
        if crate::guards::in_hand(g, gd) != Some(crate::guards::InHand::Weapon) { clear_strip(g, pl); return; }
        s.at = hand_of(g, gd);
        put_strip(g, pl, s);
        if s.t >= g.k.hold.strip.take {
            take(g, pl, &mut s);
            put_strip(g, pl, s);
        }
    }
    if let Some(item) = s.item {
        let (k_drop, k_fall) = (g.k.hold.strip.drop, g.k.hold.strip.fall);
        if s.t < k_drop {
            carry(g, &mut s, pl_ch);
        } else {
            // the disarm leading lets it go sooner: the fall is longer by that many whole steps, and the knock on the frame it was
            let fall = if k_fall > 0.0 {
                k_fall + if leads(g, Some(s)) { m::round((k_drop - g.k.hold.first.drop) * 60.0) / 60.0 } else { 0.0 }
            } else { 0.0 };
            let_go(g, item, pl_ch, Some(fall));
            s.item = None;
            s.gone = true;
            put_strip(g, pl, s);
        }
    }
    if s.t >= g.k.hold.strip.home { clear_strip(g, pl); }
}

/// (disarm.ts handOf)
pub fn hand_of(g: &mut crate::game::Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    let hand = g.ch[c].bones.get(crate::character::Bone::HandR).copied();
    v3::copy(match hand { Some(w) => w, None => v3::add(g.ch[c].pos, [0.0, 1.05, 0.0]) })
}

/// (disarm.ts long)
pub fn long(kind: &str) -> bool { kind == "rifle" }

/// The gun leaves his hand: an item from here on, his `dropped`, his hands empty; in Sam's hand or, rapped out by the pistol hold, falling. (disarm.ts take)
/// Its writes go to `s`: the caller holds the move and puts it back in the hold.
pub fn take(g: &mut crate::game::Game, pl: PlayerId, s: &mut crate::disarm::Strip) {
    let gd = g.pl[pl].holding.as_ref().unwrap().g;
    let c = g.gd[gd].ch;
    let opts = crate::throwables::ThrowablesSpawnOpts {
        who: Some(g.ch[c].id),
        fill: Some(if s.supp { 1.0 } else { 0.0 }),
        heading: Some(s.yaw),
        yaw: Some(s.yaw),
        life: Some(LIFE),
        spin: Some(0.0),
        tumble_rate: Some(0.0),
        tumble: Some(g.k.hold.strip.pitch[0] * DEG),
        restitution: Some(g.k.hold.strip.rest),
        ..Default::default()
    };
    let it = crate::throwables::throwables_spawn(g, item_kind_of(s.kind), s.at, [0.0, 0.0, 0.0], Some(opts));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::GunDown);
    g.gd[gd].dropped = Some(it);
    g.gd[gd].gun_lost_at = g.time;
    let gains = disarm_gain();
    let rate = (if long(s.kind.as_str()) { gains.long } else { 1.0 }) * (if s.rap { 1.12 } else { 1.0 });
    seam_audio_play_rate(g, "gunStrip", s.at, gains.strip, rate);
    if s.rap {
        let_go(g, it, c, None);
        s.gone = true;
    } else {
        crate::throwables::item_set_carried(g, it, Some(true));
        s.item = Some(it);
        let sam = g.pl[g.player].ch;
        carry(g, s, sam);
    }
}

/// The gun in Sam's hand this frame: a hand's length ahead of his wrist, swung out to his right and muzzle-down as he tears it clear. (disarm.ts carry)
pub fn carry(g: &mut crate::game::Game, s: &mut crate::disarm::Strip, sam: CharId) {
    let it = s.item.unwrap();
    let Some(w) = g.ch[sam].bones.get(crate::character::Bone::HandR).copied() else { return };
    let tip = g.ch[sam].bones.get(crate::character::Bone::FingerR).copied().unwrap_or(w);
    let d = v3::sub(tip, w);
    let l = v3::len(d);
    let ahead = g.k.hold.strip.hold[if long(s.kind.as_str()) { 1 } else { 0 }];
    let pos = if l > 1e-4 { v3::mad(w, d, ahead / l) } else { v3::copy(w) };
    crate::throwables::item_set_pos(g, it, pos);
    crate::throwables::item_set_vel(g, it, [0.0, 0.0, 0.0]);
    let (k_take, k_out, k_drop, pitch) = (g.k.hold.strip.take, g.k.hold.strip.out, g.k.hold.strip.drop, g.k.hold.strip.pitch);
    let u = sstep(k_take, k_out, s.t);
    let v = sstep(k_out, k_drop, s.t);
    let turned = s.yaw - (70.0 * u + 20.0 * v) * DEG;
    crate::throwables::item_set_heading(g, it, turned);
    crate::throwables::item_set_yaw(g, it, turned);
    crate::throwables::item_set_tumble(g, it, lerp(lerp(pitch[0], pitch[1], u), pitch[2], v) * DEG);
}

/// Let go of: it falls from where it is with the pair's own motion and a little push out to that man's right; never from inside a wall. The throw's fall is
/// timed so the knock stays a beat: he throws it down just hard enough to land `fall` seconds on, and never into a wall. (disarm.ts letGo)
pub fn let_go(g: &mut crate::game::Game, it: ItemId, by: CharId, fall: Option<f64>) {
    let fall = fall.unwrap_or(0.0);
    let y = g.ch[by].body_yaw;
    let by_pos = g.ch[by].pos;
    let pos = crate::throwables::item_pos(g, it);
    let from: V3 = [by_pos[0], m::max(0.3, pos[1]), by_pos[2]];
    let d: V3 = [pos[0] - from[0], 0.0, pos[2] - from[2]];
    let l = m::hyp2(d[0], d[2]);
    let mut push = g.k.hold.strip.push[0];
    if l > 1e-3 {
        let dir: V3 = [d[0] / l, 0.0, d[2] / l];
        let radius = crate::throwables::item_radius(g, it);
        if let Some(hit) = crate::scene_collision::raycast(g, from, dir, l + radius + 0.05) {
            crate::throwables::item_set_pos(g, it, v3::mad(from, dir, m::max(0.0, hit.t - radius - 0.03)));
            push = 0.0;
        }
    }
    crate::throwables::item_set_carried(g, it, Some(false));
    crate::throwables::item_set_settled(g, it, false);
    let spin = if push > 0.0 { g.k.hold.strip.spin } else { 0.0 };
    crate::throwables::item_set_spin(g, it, spin);
    crate::throwables::item_set_tumble_rate(g, it, 0.0);
    let by_vel = g.ch[by].vel;
    let mut down = g.k.hold.strip.push[1];
    let mut vx = by_vel[0] - m::cos(y) * push;
    let mut vz = by_vel[2] + m::sin(y) * push;
    if fall > 0.0 && push > 0.0 {
        let t = fall;
        let sp = m::hyp2(vx, vz);
        let p = crate::throwables::item_pos(g, it);
        let radius = crate::throwables::item_radius(g, it);
        let wall = if sp > 1e-3 { crate::scene_collision::raycast(g, p, [vx / sp, 0.0, vz / sp], sp * t + radius + 0.08) } else { None };
        if let Some(wall) = wall {
            let k = m::max(0.0, wall.t - radius - 0.08) / (sp * t);
            vx *= k;
            vz *= k;
        }
        let under = crate::scene_collision::raycast(g, [p[0] + vx * t, p[1], p[2] + vz * t], [0.0, -1.0, 0.0], p[1] + 0.1);
        let dd = (match under { Some(u) => u.t, None => p[1] }) - radius;
        // below zero it leaves his hand going up a little: a long fall from a low hand is a lob
        if dd > 0.3 { down = m::min(4.0, m::max(-1.5, dd / t - 0.5 * crate::throwables::GRAVITY * t)); }
    }
    crate::throwables::item_set_vel(g, it, [vx, -down, vz]);
}

/// The gun on the floor: one dull knock, the first time it lands, on what it lands on by the face it struck (Item.onBounce). (disarm.ts gunDown)
pub fn gun_down(g: &mut crate::game::Game, sp: f64, p: V3, it: ItemId, n: V3) {
    if n[1] < 0.6 { return; }
    if sp < 0.7 { return; }
    let clatters = g.tab.item[it].clatters.unwrap_or(0.0) + 1.0;
    g.tab.item[it].clatters = Some(clatters);
    if clatters > 1.0 { return; }
    let gains = disarm_gain();
    let rate = if long(crate::throwables::item_kind(g, it).as_str()) { gains.long } else { 1.0 };
    seam_audio_play_rate(g, "gunDown", p, gains.down * m::min(1.0, 0.5 + sp / 8.0), rate);
}

/// `S` is one object in the TypeScript, shared with the hold: a copy here, so each write is put back through the table.
fn put_strip(g: &mut Game, pl: PlayerId, s: Strip) {
    if let Some(h) = g.pl[pl].holding.as_mut() { h.strip = Some(s); }
}

/// `H.strip = undefined`
fn clear_strip(g: &mut Game, pl: PlayerId) {
    if let Some(h) = g.pl[pl].holding.as_mut() { h.strip = None; }
}

/// `men(g).some(o => o.holding?.strip?.item === it)`
fn any_man_carries(g: &mut Game, it: ItemId) -> bool {
    let of = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    let men = crate::seats::men(g, &of);
    men.iter().any(|&o| g.pl[o].holding.as_ref().and_then(|h| h.strip).and_then(|s| s.item) == Some(it))
}

/// DropKind's words are two of crate::throwables::ItemKind's: the gun's own kind of item.
fn item_kind_of(kind: crate::slotsetup::SetupGun) -> crate::throwables::ItemKind {
    match kind { crate::slotsetup::SetupGun::Pistol => crate::throwables::ItemKind::Pistol, crate::slotsetup::SetupGun::Rifle => crate::throwables::ItemKind::Rifle }
}

/// `g.audio.play(name, at, gain, { rate })` (disarm.ts:56, 76, 99, 136): `game_audio_like_play`'s opts hold `crate::Opaque`, which cannot carry a rate.
fn seam_audio_play_rate(g: &mut Game, name: &str, at: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_is_the_rifle() {
        assert!(long("rifle"));
        assert!(!long("pistol"));
    }

    #[test]
    fn pose_tells_the_clock_and_the_hand() {
        let s = Strip { t: 0.25, at: [1.0, 2.0, 3.0], ..Strip::zeroed() };
        let p = disarm_pose(s);
        assert_eq!(p.t.to_bits(), 0.25f64.to_bits());
        assert_eq!(p.at, Some([1.0, 2.0, 3.0]));
    }

    /// the clock's rate by its three spans, on a blank game with knobs set here (a Game is too big for the test's own stack)
    #[test]
    fn the_disarm_leads_until_the_let_go() {
        extern crate std;
        std::thread::Builder::new().stack_size(1 << 30).spawn(|| {
            let mut g = Game::blank();
            g.k.hold.strip.take = 0.3; g.k.hold.strip.drop = 0.6;
            g.k.hold.first.take = 0.1; g.k.hold.first.drop = 0.25;
            let s = Strip { t: 0.0, ..Strip::zeroed() };
            assert!(!leads(&mut g, None));
            assert!(!leads(&mut g, Some(Strip { rap: true, ..s })));
            assert!(leads(&mut g, Some(s)));
            assert_eq!(lead(&mut g, Strip { rap: true, ..s }).to_bits(), 1.0f64.to_bits());
            assert_eq!(lead(&mut g, s).to_bits(), (0.3f64 / 0.1).to_bits());
            assert_eq!(lead(&mut g, Strip { t: 0.3, ..s }).to_bits(), ((0.6f64 - 0.3) / (0.25 - 0.1)).to_bits());
            assert_eq!(lead(&mut g, Strip { t: 0.6, ..s }).to_bits(), 1.0f64.to_bits());
        }).unwrap().join().unwrap();
    }

    /// the TypeScript's own bits, printed under bun with EXACT on
    #[test]
    fn arithmetic_is_the_typescripts() {
        assert_eq!(DEG.to_bits(), 0x3f91df46a2529d39);
        assert_eq!(sstep(0.26, 0.35, 0.3).to_bits(), 0x3fdab0490ae2da6b);
        assert_eq!(sstep(0.35, 0.41, 0.4).to_bits(), 0x3feda12f684bda18);
        assert_eq!(sstep(0.26, 0.35, 0.1).to_bits(), 0x0000000000000000);
        assert_eq!(sstep(0.26, 0.35, 0.9).to_bits(), 0x3ff0000000000000);
        assert!(sstep(1.0, 1.0, 1.0).is_nan());
        assert_eq!(sstep(1.0, 1.0, 0.5).to_bits(), 0x0000000000000000);
        assert_eq!(lerp(-40.0, 25.0, 0.37).to_bits(), 0xc02fe66666666666);
        assert_eq!(lerp(0.1, 0.2, 0.3).to_bits(), 0x3fc0a3d70a3d70a4);
        assert_eq!(lerp(1e300, -1e300, 0.5).to_bits(), 0x0000000000000000);
    }
}
