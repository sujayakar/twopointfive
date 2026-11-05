//! src/game/weapons.ts lines 350 to 548
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// a row of the table, by the key a gun keeps (`this.def`)
fn row(g: &mut Game, def: WeaponId) -> &WeaponDef { g.m.weapons.weapons.get(def).unwrap() }

/// `g.audio.play(sfx, at, vol)`: no options
fn play(g: &mut Game, sfx: &str, at: V3, vol: f64) { crate::audio_iface::game_audio_like_play(g, sfx, Some(at), Some(vol), None) }

// ---- seam: what skeleton 3 cannot give yet (see notes.md, ## Skeleton) ----

/// `g.audio.play(sfx, at, vol, { rate })`: the opts of `game_audio_like_play` are a map of `Opaque`, which cannot hold a number (player.ts reloadInHand's callbacks)
fn seam_audio_play_rate(g: &mut Game, sfx: &str, at: V3, vol: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to hold a rate") }

// ---- what a reload tells its owner: the two callers' callbacks (player.ts reloadInHand) ----

/// `ev.magOut(rounds, dropped)`
fn ev_mag_out(g: &mut Game, id: FirearmId, dropped: bool) {
    match g.fa[id].ev.clone() {
        ReloadEvents::None => {}
        ReloadEvents::Pistol { ch, fill } => {
            let p = g.ch[ch].pos;
            play(g, "magOut", p, 0.7);
            if dropped {
                let h = g.ch[ch].bones.get(crate::character::Bone::HandL).copied().unwrap_or(g.ch[ch].pos);
                let at = [h[0], m::max(0.3, h[1]), h[2]];
                let r0 = crate::math_dice::sim_random(g);
                let r1 = crate::math_dice::sim_random(g);
                spawn_mag(g, ch, at, [(r0 - 0.5) * 0.6, -0.2, (r1 - 0.5) * 0.6], fill, crate::throwables::ItemBounce::Mag);
            }
        }
        ReloadEvents::Rifle { ch, feed } => {
            let own = feed.as_ref().map_or(false, |f| f.cues.is_some());
            if !own {
                let p = g.ch[ch].pos;
                seam_audio_play_rate(g, "magOut", p, 0.75, 0.85);
            }
            let def = g.fa[id].def;
            // (`fp?.drop !== null`: no plan at all is not null; a plan whose drop is null drops nothing)
            if dropped && row(g, def).pellets.is_none() && feed.as_ref().map_or(true, |f| f.drop.is_some()) {
                let h = g.ch[ch].bones.get(crate::character::Bone::HandR).copied().unwrap_or(g.ch[ch].pos);
                let d = g.ch[ch].gun_dir;
                let at = [h[0] - d[0] * 0.1, m::max(0.3, h[1] - 0.05), h[2] - d[2] * 0.1];
                let r0 = crate::math_dice::sim_random(g);
                let r1 = crate::math_dice::sim_random(g);
                let drop = feed.as_ref().and_then(|f| f.drop);
                let bounce = crate::throwables::ItemBounce::MagRate { belt: drop == Some(5.0) };
                spawn_mag(g, ch, at, [(r0 - 0.5) * 0.6, -0.3, (r1 - 0.5) * 0.6], drop.unwrap_or(3.0), bounce);
            }
        }
    }
}

/// the empty that falls out of the gun, and its clatter (both callers' `g.items.spawn('mag', …)`; `who` their `g.player.char.id`, the body they closed over)
fn spawn_mag(g: &mut Game, ch: CharId, at: V3, vel: V3, fill: f64, bounce: crate::throwables::ItemBounce) {
    let who = g.ch[ch].id;
    let opts = crate::throwables::ThrowablesSpawnOpts { fill: Some(fill), who: Some(who), ..Default::default() };
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Mag, at, vel, Some(opts));
    g.tab.item[it].on_bounce = Some(bounce);
}

/// `ev.magIn(rounds)`
fn ev_mag_in(g: &mut Game, id: FirearmId) {
    match g.fa[id].ev.clone() {
        ReloadEvents::None => {}
        ReloadEvents::Pistol { ch, .. } => {
            let p = g.ch[ch].pos;
            play(g, "magIn", p, 0.8);
        }
        ReloadEvents::Rifle { ch, feed } => {
            if !feed.as_ref().map_or(false, |f| f.cues.is_some()) {
                let p = g.ch[ch].pos;
                seam_audio_play_rate(g, "magIn", p, 0.85, 0.85);
            }
        }
    }
}

/// `ev.rack()`
fn ev_rack(g: &mut Game, id: FirearmId) {
    match g.fa[id].ev.clone() {
        ReloadEvents::None => {}
        ReloadEvents::Pistol { ch, .. } => {
            let p = g.ch[ch].pos;
            play(g, "rack", p, 0.8);
        }
        ReloadEvents::Rifle { ch, feed } => {
            if !feed.as_ref().map_or(false, |f| f.cues.is_some()) {
                let p = g.ch[ch].pos;
                seam_audio_play_rate(g, "rack", p, 0.85, 0.8);
            }
        }
    }
}

// ---- Firearm ----

/// a shell at a time is how the reload under way (or the beat planned) puts its rounds in (weapons.ts Firearm.shellFed)
pub fn firearm_shell_fed(g: &mut crate::game::Game, id: FirearmId) -> bool {
    g.fa[id].shell_t.is_some()
}

/// the beat the reload under way (or the next plain one) runs on (weapons.ts Firearm.reloadRow)
pub fn firearm_reload_row(g: &mut crate::game::Game, id: FirearmId) -> crate::weapons::WeaponDefReload {
    if let Some(b) = g.fa[id].beat { return b; }
    let def = g.fa[id].def;
    row(g, def).reload
}

/// a row being carried: loaded its way, its spares full, the selector on the first mode. A data-only row throws (weapons.ts Firearm.constructor)
pub fn firearm_new(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> FirearmId {
    if !def.enabled {
        panic!("weapons: '{}' is a data-only row (docs/internal/sc20k-design.md §10) — nothing may carry it until its lane switches it on", def.id.as_str());
    }
    let cap = def.mag_capacity;
    let mut fa = Firearm::blank();
    fa.def = def.id;
    fa.mag = Some(cap - if def.top_up { 0.0 } else { 1.0 });
    fa.spare = if !def.spare_mags.is_finite() { Vec::new() } else { vec![cap; def.spare_mags as usize] };
    fa.mode = def.modes[0];
    g.fa.push(fa)
}

/// the row's full name (weapons.ts Firearm.name)
pub fn firearm_name(g: &mut crate::game::Game, id: FirearmId) -> String {
    let def = g.fa[id].def;
    row(g, def).name.clone()
}

/// Z: the selector to its next position; ends any AUTO string; returns the new mode (weapons.ts Firearm.cycleMode)
pub fn firearm_cycle_mode(g: &mut crate::game::Game, id: FirearmId) -> crate::weapons::FireMode {
    let def = g.fa[id].def;
    let modes = row(g, def).modes.clone();
    let now = g.fa[id].mode;
    // (indexOf's −1, a mode the row does not offer, steps to the first)
    let next = modes.iter().position(|x| *x == now).map_or(0, |i| i + 1) % modes.len();
    let fa = &mut g.fa[id];
    fa.mode = modes[next];
    fa.auto_idx = 0.0;
    fa.mode
}

/// automatic under a held trigger right now (weapons.ts Firearm.automatic)
pub fn firearm_automatic(g: &mut crate::game::Game, id: FirearmId) -> bool {
    if g.fa[id].mode != FireMode::Auto { return false; }
    let def = g.fa[id].def;
    row(g, def).auto.is_some()
}

/// the extra cone the NEXT round of the current AUTO string is drawn with (weapons.ts Firearm.autoSpread)
pub fn firearm_auto_spread(g: &mut crate::game::Game, id: FirearmId) -> f64 {
    let def = g.fa[id].def;
    let auto = row(g, def).auto;
    match auto {
        Some(a) if g.fa[id].mode == FireMode::Auto => m::min(a.spread_cap, a.spread_per_round * g.fa[id].auto_idx),
        _ => 0.0,
    }
}

/// book one round of the string (weapons.ts Firearm.autoFired)
pub fn firearm_auto_fired(g: &mut crate::game::Game, id: FirearmId) {
    let fa = &mut g.fa[id];
    fa.auto_idx += 1.0;
    fa.auto_since = 0.0;
}

/// let the string lapse once the trigger has been up auto.resetAfter (weapons.ts Firearm.autoIdle)
pub fn firearm_auto_idle(g: &mut crate::game::Game, id: FirearmId, dt: f64, trigger_down: bool) {
    g.fa[id].auto_since += dt;
    if trigger_down { return; }
    let def = g.fa[id].def;
    let reset_after = row(g, def).auto.map_or(0.0, |a| a.reset_after);
    if g.fa[id].auto_since > reset_after { g.fa[id].auto_idx = 0.0; }
}

/// never runs out of spares (weapons.ts Firearm.bottomless)
pub fn firearm_bottomless(g: &mut crate::game::Game, id: FirearmId) -> bool {
    let def = g.fa[id].def;
    !row(g, def).spare_mags.is_finite()
}

/// (weapons.ts Firearm.reloading)
pub fn firearm_reloading(g: &mut crate::game::Game, id: FirearmId) -> bool {
    g.fa[id].reload_t >= 0.0
}

/// (weapons.ts Firearm.roundsReady)
pub fn firearm_rounds_ready(g: &mut crate::game::Game, id: FirearmId) -> f64 {
    g.fa[id].chamber + g.fa[id].mag.unwrap_or(0.0)
}

/// rounds gone since the gun was last full (weapons.ts Firearm.spent)
pub fn firearm_spent(g: &mut crate::game::Game, id: FirearmId) -> f64 {
    let def = g.fa[id].def;
    let cap = row(g, def).mag_capacity;
    let top_up = row(g, def).top_up;
    cap + (if top_up { 1.0 } else { 0.0 }) - firearm_rounds_ready(g, id)
}

/// (weapons.ts Firearm.canFire)
pub fn firearm_can_fire(g: &mut crate::game::Game, id: FirearmId) -> bool {
    (g.fa[id].chamber == 1.0 || (g.fa[id].infinite && g.fa[id].mag.is_some())) && !firearm_reloading(g, id)
}

/// fullest-first view of the spare mags (weapons.ts Firearm.spareSorted)
pub fn firearm_spare_sorted(g: &mut crate::game::Game, id: FirearmId) -> Vec<f64> {
    let mut v = g.fa[id].spare.clone();
    js::sort_by(&mut v, |a, b| *b - *a);
    v
}

/// one round gone, the next cycled up; false = it would not fire (weapons.ts Firearm.fire)
pub fn firearm_fire(g: &mut crate::game::Game, id: FirearmId) -> bool {
    if !firearm_can_fire(g, id) { return false; }
    let fa = &mut g.fa[id];
    fa.chamber = 0.0;
    fa.shots_fired += 1.0;
    if fa.infinite && fa.mag.is_some() {
        fa.chamber = 1.0;
    } else if let Some(n) = fa.mag {
        if n > 0.0 {
            fa.mag = Some(n - 1.0);
            fa.chamber = 1.0;
        }
    }
    true
}

/// the most a seated magazine may hold right now by the row's rule (weapons.ts Firearm.magRoom)
pub fn firearm_mag_room(g: &mut crate::game::Game, id: FirearmId) -> f64 {
    let def = g.fa[id].def;
    let cap = row(g, def).mag_capacity;
    let top_up = row(g, def).top_up;
    cap - (if top_up { 0.0 } else { g.fa[id].chamber })
}

/// begin a reload if it makes sense; false if refused (weapons.ts Firearm.startReload)
pub fn firearm_start_reload(g: &mut crate::game::Game, id: FirearmId, ev: Option<&crate::weapons::ReloadEvents>, beat: Option<crate::weapons::WeaponDefReload>, cues: Option<&[f64]>, shells: Option<&crate::weapons::FirearmStartReloadShells>) -> bool {
    if firearm_reloading(g, id) { return false; }
    let ev = ev.cloned().unwrap_or(ReloadEvents::None);
    let cues = cues.filter(|c| !c.is_empty()).map(|c| c.to_vec());
    if let Some(shells) = shells {
        if shells.times.is_empty() && !(g.fa[id].chamber == 0.0 && g.fa[id].mag.unwrap_or(0.0) > 0.0) { return false; }
        let fa = &mut g.fa[id];
        fa.reload_t = 0.0; fa.stage = 2.0; fa.ev = ev; fa.beat = beat; fa.cues = cues; fa.cue_i = 0;
        firearm_plan_shells(g, id, &shells.times, shells.port);
        return true;
    }
    let bottomless = firearm_bottomless(g, id);
    let (chamber, mag) = (g.fa[id].chamber, g.fa[id].mag);
    if g.fa[id].spare.is_empty() && !bottomless && !(chamber == 0.0 && mag.unwrap_or(0.0) > 0.0) { return false; }
    let def = g.fa[id].def;
    let from = if !g.fa[id].spare.is_empty() {
        m::max_n(&g.fa[id].spare)
    } else if bottomless {
        row(g, def).mag_capacity
    } else {
        mag.unwrap_or(0.0)
    };
    let best = m::min(from, firearm_mag_room(g, id));
    if let Some(n) = mag {
        if n >= best && chamber == 1.0 { return false; }
    }
    let fa = &mut g.fa[id];
    fa.reload_t = 0.0; fa.stage = 0.0; fa.ev = ev; fa.beat = beat; fa.cues = cues; fa.cue_i = 0;
    fa.feed_n = m::max(0.0, best - mag.unwrap_or(0.0));
    fa.feed_port = chamber == 0.0 && !mag.map_or(false, js::truthy);
    true
}

/// (weapons.ts Firearm.cancelReload)
pub fn firearm_cancel_reload(g: &mut crate::game::Game, id: FirearmId) {
    let fa = &mut g.fa[id];
    fa.reload_t = -1.0; fa.stage = 0.0; fa.beat = None; fa.cues = None; fa.shell_t = None;
}

/// (weapons.ts Firearm.planShells)
pub fn firearm_plan_shells(g: &mut crate::game::Game, id: FirearmId, times: &[f64], port: bool) {
    let fa = &mut g.fa[id];
    let tube = *fa.mag.get_or_insert(0.0);
    fa.shell_t = Some(times.to_vec());
    fa.shell_i = 0;
    fa.shell_port = port && fa.chamber == 0.0 && tube == 0.0;
    fa.feed_n = times.len() as f64;
    fa.feed_port = fa.chamber == 0.0 && tube == 0.0;
    fa.feed_pull = fa.chamber == 0.0 && !fa.shell_port;
}

/// a man whose reload is a beat of his own feeds a shell at a time on `row` (weapons.ts Firearm.planFeed)
pub fn firearm_plan_feed(g: &mut crate::game::Game, id: FirearmId, row: crate::weapons::WeaponDefReload, times: &[f64], port: bool) {
    g.fa[id].beat = Some(row);
    firearm_plan_shells(g, id, times, port);
}

/// his beat has come to `t` seconds: the shells due by then are in the gun (weapons.ts Firearm.feedTo)
pub fn firearm_feed_to(g: &mut crate::game::Game, id: FirearmId, t: f64, free: Option<bool>) {
    let free = free.unwrap_or(true);
    loop {
        let i = g.fa[id].shell_i;
        let due = g.fa[id].shell_t.as_ref().map_or(false, |s| i < s.len() && t >= s[i]);
        if !due { return; }
        let port = g.fa[id].shell_port;
        firearm_feed_shell(g, id, Some(port), Some(free));
        g.fa[id].shell_i = i + 1;
    }
}

/// one shell into the gun, out of the fewest loose ones he carries; false: no room, or none left (weapons.ts Firearm.feedShell)
pub fn firearm_feed_shell(g: &mut crate::game::Game, id: FirearmId, port: Option<bool>, free: Option<bool>) -> bool {
    let (port, free) = (port.unwrap_or(false), free.unwrap_or(false));
    let def = g.fa[id].def;
    let tube = g.fa[id].mag.unwrap_or(0.0);
    let to_port = g.fa[id].chamber == 0.0 && (free || (port && tube == 0.0));
    if !to_port && tube >= row(g, def).mag_capacity { return false; }
    if !free && !firearm_bottomless(g, id) {
        let fa = &mut g.fa[id];
        let mut bi: Option<usize> = None;
        for i in 0..fa.spare.len() {
            if fa.spare[i] > 0.0 && bi.map_or(true, |b| fa.spare[i] <= fa.spare[b]) { bi = Some(i); }
        }
        let Some(bi) = bi else { return false; };
        fa.spare[bi] -= 1.0;
        if fa.spare[bi] == 0.0 { fa.spare.remove(bi); }
    }
    let fa = &mut g.fa[id];
    if to_port {
        fa.chamber = 1.0;
        fa.mag = Some(tube);
    } else {
        fa.mag = Some(tube + 1.0);
    }
    true
}

/// a full magazine seated and a round chambered, any reload or burst forgotten (weapons.ts Firearm.rearm)
pub fn firearm_rearm(g: &mut crate::game::Game, id: FirearmId) {
    firearm_cancel_reload(g, id);
    let def = g.fa[id].def;
    let cap = row(g, def).mag_capacity;
    let top_up = row(g, def).top_up;
    let fa = &mut g.fa[id];
    fa.burst = None;
    fa.auto_idx = 0.0;
    fa.chamber = 1.0;
    fa.mag = Some(cap - if top_up { 0.0 } else { 1.0 });
}

/// no magazine holding more than the row's magazine does now (weapons.ts Firearm.fit)
pub fn firearm_fit(g: &mut crate::game::Game, id: FirearmId) {
    let def = g.fa[id].def;
    let cap = row(g, def).mag_capacity;
    let room = firearm_mag_room(g, id);
    let fa = &mut g.fa[id];
    if let Some(n) = fa.mag {
        if n > room { fa.mag = Some(room); }
    }
    for s in fa.spare.iter_mut() {
        if *s > cap { *s = cap; }
    }
}

/// the staged reload's clock: out, in, rack, done, each once (weapons.ts Firearm.update)
pub fn firearm_update(g: &mut crate::game::Game, id: FirearmId, dt: f64) {
    if !firearm_reloading(g, id) { return; }
    g.fa[id].reload_t += dt;
    let t = g.fa[id].reload_t;
    let r = firearm_reload_row(g, id);
    firearm_feed_to(g, id, t, Some(false));
    loop {
        let i = g.fa[id].cue_i;
        let due = g.fa[id].cues.as_ref().map_or(false, |c| i < c.len() && t >= c[i]);
        if !due { break; }
        match g.fa[id].ev.clone() {
            ReloadEvents::Rifle { ch, feed } => {
                g.fa[id].cue_i = i + 1;
                if let Some(c) = feed.as_ref().and_then(|f| f.cues.as_ref()).map(|own| &own[i]) {
                    let at = g.ch[ch].pos;
                    let cue = crate::gunreload::FeedCue { t: c.t, sfx: c.sfx.clone(), gain: c.gain, rate: c.rate };
                    crate::gunreload::feed_play(g, at, &cue, None);
                }
            }
            // (the TypeScript's `cue?.(cueI++)` moves nothing with no `cue` and would spin for ever: no caller gives cues without one)
            _ => break,
        }
    }
    let def = g.fa[id].def;
    let bottomless = firearm_bottomless(g, id);
    if g.fa[id].stage < 1.0 && t >= r.out.unwrap_or(r.dur) {
        if let Some(out) = g.fa[id].mag {
            let dropped = out == 0.0;
            if !dropped && !bottomless { g.fa[id].spare.push(out); }
            g.fa[id].mag = None;
            ev_mag_out(g, id, dropped);
        }
        g.fa[id].stage = 1.0;
    }
    if g.fa[id].stage < 2.0 && t >= r.in_.unwrap_or(r.dur) {
        let got = if !g.fa[id].spare.is_empty() {
            let fa = &mut g.fa[id];
            let mut bi = 0;
            for i in 1..fa.spare.len() {
                if fa.spare[i] > fa.spare[bi] { bi = i; }
            }
            fa.spare.remove(bi)
        } else if bottomless {
            row(g, def).mag_capacity
        } else {
            0.0
        };
        let mag = m::min(got, firearm_mag_room(g, id));
        g.fa[id].mag = Some(mag);
        if got > mag && !bottomless { g.fa[id].spare.push(got - mag); }
        g.fa[id].stage = 2.0;
        ev_mag_in(g, id);
    }
    if g.fa[id].stage < 3.0 && t >= r.rack.unwrap_or(r.dur) {
        if let Some(n) = g.fa[id].mag {
            if g.fa[id].chamber == 0.0 && n > 0.0 {
                g.fa[id].mag = Some(n - 1.0);
                g.fa[id].chamber = 1.0;
                ev_rack(g, id);
            }
        }
        g.fa[id].stage = 3.0;
    }
    if t >= r.dur {
        let fa = &mut g.fa[id];
        fa.reload_t = -1.0; fa.stage = 0.0; fa.beat = None; fa.cues = None; fa.shell_t = None;
        // (`ev.done?.()`: no caller gives one, and ReloadEvents has no word for it)
    }
}

// ---- Pistol and Rifle: rows of g.fa. Their stubs have no `g` and no ID: these bodies wait behind them (notes.md, ## Skeleton) ----

/// Sam's sidearm: the Five-seveN row unless a card picks another (weapons.ts Pistol.constructor)
fn pistol_new_in(g: &mut Game, def: Option<&WeaponDef>) -> FirearmId {
    let id = match def {
        Some(d) => firearm_new(g, d),
        None => { let d = row(g, WeaponId::Fiveseven).clone(); firearm_new(g, &d) }
    };
    g.fa[id].kind = GunKind::Pistol;
    id
}

/// Sam's FN2000 row, its loads from the launcher's tables; nothing under a barrel with pellets or `launcher: false` (weapons.ts Rifle.constructor)
fn rifle_new_in(g: &mut Game, def: Option<&WeaponDef>) -> FirearmId {
    let fn2000;
    let def = match def {
        Some(d) => d,
        None => { fn2000 = row(g, WeaponId::Fn2000).clone(); &fn2000 }
    };
    let id = firearm_new(g, def);
    let (rings, shells) = (g.k.launcher.rings, g.k.launcher40.rounds);
    let bare = def.pellets.is_some() || def.launcher == Some(false);
    let fa = &mut g.fa[id];
    fa.kind = GunKind::Rifle;
    fa.attachments = vec![RifleAttachment::Ring, RifleAttachment::He40];
    fa.att_idx = 0;
    fa.rings = rings;
    fa.shells = shells;
    fa.cams = 0.0;
    if bare {
        fa.attachments.clear();
        fa.att_idx = -1;
        fa.rings = 0.0;
        fa.shells = 0.0;
    }
    id
}

/// the attachment the right button fires now (weapons.ts Rifle.attachment)
fn rifle_attachment_of(g: &mut Game, id: FirearmId) -> Option<RifleAttachment> {
    let fa = &g.fa[id];
    if fa.att_idx >= 0 { fa.attachments.get(fa.att_idx as usize).copied() } else { None }
}

/// rounds left of an attachment (weapons.ts Rifle.left)
fn rifle_left_of(g: &mut Game, id: FirearmId, a: RifleAttachment) -> f64 {
    let fa = &g.fa[id];
    match a {
        RifleAttachment::Ring => fa.rings,
        RifleAttachment::Cam => fa.cams,
        RifleAttachment::He40 => fa.shells,
    }
}

/// the next attachment goes on the right button, the last back to the first (weapons.ts Rifle.cycleAttachment)
fn rifle_cycle_attachment_of(g: &mut Game, id: FirearmId) -> Option<RifleAttachment> {
    let fa = &mut g.fa[id];
    let n = fa.attachments.len() as i32;
    fa.att_idx = if n != 0 { (fa.att_idx + 1) % n } else { -1 };
    rifle_attachment_of(g, id)
}

/// (weapons.ts Pistol.constructor)
pub fn pistol_new(def: Option<&crate::weapons::WeaponDef>) -> crate::Opaque { todo!("needs: g and a FirearmId return: pistol_new_in has the body") }

/// (weapons.ts Rifle.constructor)
pub fn rifle_new(def: Option<&crate::weapons::WeaponDef>) -> crate::Opaque { todo!("needs: g and a FirearmId return: rifle_new_in has the body") }

/// (weapons.ts Rifle.attachment)
pub fn rifle_attachment() -> Option<crate::weapons::RifleAttachment> { todo!("needs: g and the FirearmId: rifle_attachment_of has the body") }

/// (weapons.ts Rifle.left)
pub fn rifle_left(a: crate::weapons::RifleAttachment) -> f64 { todo!("needs: g and the FirearmId: rifle_left_of has the body") }

/// (weapons.ts Rifle.cycleAttachment)
pub fn rifle_cycle_attachment() -> Option<crate::weapons::RifleAttachment> { todo!("needs: g and the FirearmId: rifle_cycle_attachment_of has the body") }

// ---- Ocp ----

/// (weapons.ts Ocp.ready)
pub fn ocp_ready(this: &mut crate::weapons::Ocp) -> bool {
    this.charge >= this.cost - 1e-3
}

/// spends the whole charge; false if not ready (weapons.ts Ocp.use)
pub fn ocp_use(this: &mut crate::weapons::Ocp) -> bool {
    if !ocp_ready(this) { return false; }
    this.charge = 0.0;
    true
}

/// (weapons.ts Ocp.update)
pub fn ocp_update(this: &mut crate::weapons::Ocp, dt: f64) {
    this.charge = m::min(1.0, this.charge + dt * this.recharge_per_sec);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every expected value below was printed by the TypeScript classes under bun, with GEAR.on and RELOAD.lastMag at the page's true.

    fn st(g: &Game, id: FirearmId) -> (f64, Option<f64>, Vec<f64>, f64, f64, bool, bool, bool) {
        let f = &g.fa[id];
        (f.chamber, f.mag, f.spare.clone(), f.reload_t, f.feed_n, f.feed_port, f.feed_pull, f.shell_t.is_some())
    }

    fn the_row(g: &mut Game, id: WeaponId) -> WeaponDef { row(g, id).clone() }

    #[test]
    fn a_new_gun_is_loaded_its_rows_way() {
        let mut g = Game::blank();
        let p = pistol_new_in(&mut g, None);
        assert_eq!(st(&g, p), (1.0, Some(15.0), vec![15.0, 15.0, 15.0], -1.0, 0.0, false, false, false));
        assert_eq!(firearm_spent(&mut g, p), 0.0);
        assert_eq!(g.fa[p].kind, GunKind::Pistol);
        let d = the_row(&mut g, WeaponId::Guard9mm);
        let gd = firearm_new(&mut g, &d);
        assert_eq!(st(&g, gd), (1.0, Some(11.0), vec![], -1.0, 0.0, false, false, false));
        assert!(firearm_bottomless(&mut g, gd));
        assert_eq!(firearm_spent(&mut g, gd), 0.0);
        let r = rifle_new_in(&mut g, None);
        assert_eq!(st(&g, r), (1.0, Some(30.0), vec![30.0, 30.0], -1.0, 0.0, false, false, false));
        assert_eq!(rifle_attachment_of(&mut g, r), Some(RifleAttachment::Ring));
        assert_eq!((g.fa[r].rings, g.fa[r].shells, g.fa[r].cams, g.fa[r].att_idx), (5.0, 3.0, 0.0, 0));
        let d = the_row(&mut g, WeaponId::M870);
        let s = rifle_new_in(&mut g, Some(&d));
        assert_eq!(st(&g, s), (1.0, Some(6.0), vec![6.0, 6.0, 6.0], -1.0, 0.0, false, false, false));
        assert_eq!(rifle_attachment_of(&mut g, s), None);
        assert_eq!((g.fa[s].rings, g.fa[s].shells, g.fa[s].att_idx), (0.0, 0.0, -1));
        assert_eq!(rifle_cycle_attachment_of(&mut g, s), None);
    }

    #[test]
    fn the_staged_reload_keeps_its_beats() {
        let mut g = Game::blank();
        let p = pistol_new_in(&mut g, None);
        for _ in 0..3 { assert!(firearm_fire(&mut g, p)); }
        assert_eq!(g.fa[p].mag, Some(12.0));
        assert_eq!(g.fa[p].shots_fired, 3.0);
        assert_eq!(firearm_spent(&mut g, p), 3.0);
        assert!(firearm_start_reload(&mut g, p, None, None, None, None));
        assert_eq!(st(&g, p), (1.0, Some(12.0), vec![15.0, 15.0, 15.0], 0.0, 3.0, false, false, false));
        assert!(!firearm_start_reload(&mut g, p, None, None, None, None));
        firearm_update(&mut g, p, 0.25);
        assert_eq!(g.fa[p].mag, Some(12.0));
        firearm_update(&mut g, p, 0.25);
        assert_eq!((g.fa[p].mag, g.fa[p].spare.clone()), (None, vec![15.0, 15.0, 15.0, 12.0]));
        firearm_update(&mut g, p, 0.25);
        firearm_update(&mut g, p, 0.25);
        assert_eq!(g.fa[p].mag, None);
        firearm_update(&mut g, p, 0.25);
        assert_eq!((g.fa[p].mag, g.fa[p].spare.clone()), (Some(15.0), vec![15.0, 15.0, 12.0]));
        firearm_update(&mut g, p, 0.25);
        assert!(firearm_reloading(&mut g, p));
        firearm_update(&mut g, p, 0.25);
        assert_eq!(st(&g, p), (1.0, Some(15.0), vec![15.0, 15.0, 12.0], -1.0, 3.0, false, false, false));
        assert_eq!(firearm_spare_sorted(&mut g, p), vec![15.0, 15.0, 12.0]);
    }

    #[test]
    fn the_last_magazine_goes_back_in_and_is_racked() {
        let mut g = Game::blank();
        let q = pistol_new_in(&mut g, None);
        g.fa[q].spare.clear();
        assert!(!firearm_start_reload(&mut g, q, None, None, None, None));
        g.fa[q].mag = Some(4.0);
        g.fa[q].chamber = 0.0;
        assert!(firearm_start_reload(&mut g, q, None, None, None, None));
        assert_eq!(st(&g, q), (0.0, Some(4.0), vec![], 0.0, 0.0, false, false, false));
        for _ in 0..16 { if firearm_reloading(&mut g, q) { firearm_update(&mut g, q, 0.25); } }
        assert_eq!(st(&g, q), (1.0, Some(3.0), vec![], -1.0, 0.0, false, false, false));
    }

    #[test]
    fn a_bottomless_gun_seats_one_short_over_a_chambered_round() {
        let mut g = Game::blank();
        let d = the_row(&mut g, WeaponId::Guard9mm);
        let gd = firearm_new(&mut g, &d);
        g.fa[gd].mag = Some(3.0);
        assert!(firearm_start_reload(&mut g, gd, None, None, None, None));
        assert_eq!(st(&g, gd), (1.0, Some(3.0), vec![], 0.0, 8.0, false, false, false));
        for _ in 0..40 { if firearm_reloading(&mut g, gd) { firearm_update(&mut g, gd, 0.25); } }
        assert_eq!(st(&g, gd), (1.0, Some(11.0), vec![], -1.0, 8.0, false, false, false));
        assert_eq!(firearm_spent(&mut g, gd), 0.0);
    }

    #[test]
    fn a_tube_gun_takes_a_shell_at_a_time() {
        let mut g = Game::blank();
        let d = the_row(&mut g, WeaponId::M870);
        let s = rifle_new_in(&mut g, Some(&d));
        g.fa[s].chamber = 0.0;
        g.fa[s].mag = Some(0.0);
        g.fa[s].spare = vec![2.0, 1.0];
        let beat = WeaponDefReload { dur: 1.0, out: None, in_: None, rack: Some(0.9), shell: None };
        let shells = FirearmStartReloadShells { times: vec![0.2, 0.5, 0.7], port: true };
        assert!(firearm_start_reload(&mut g, s, None, Some(beat), None, Some(&shells)));
        assert_eq!(st(&g, s), (0.0, Some(0.0), vec![2.0, 1.0], 0.0, 3.0, true, false, true));
        firearm_update(&mut g, s, 0.2);
        assert_eq!(st(&g, s), (1.0, Some(0.0), vec![2.0], 0.2, 3.0, true, false, true));
        firearm_update(&mut g, s, 0.2);
        assert_eq!((g.fa[s].chamber, g.fa[s].mag, g.fa[s].spare.clone()), (1.0, Some(0.0), vec![2.0]));
        firearm_update(&mut g, s, 0.2);
        assert_eq!((g.fa[s].chamber, g.fa[s].mag, g.fa[s].spare.clone()), (1.0, Some(1.0), vec![1.0]));
        firearm_update(&mut g, s, 0.2);
        assert_eq!((g.fa[s].chamber, g.fa[s].mag, g.fa[s].spare.clone()), (1.0, Some(2.0), vec![]));
        firearm_update(&mut g, s, 0.2);
        assert_eq!(st(&g, s), (1.0, Some(2.0), vec![], -1.0, 3.0, true, false, false));

        let t = rifle_new_in(&mut g, Some(&d));
        g.fa[t].chamber = 0.0;
        g.fa[t].mag = Some(0.0);
        firearm_plan_feed(&mut g, t, WeaponDefReload { dur: 1.0, out: None, in_: None, rack: None, shell: None }, &[0.1, 0.3], true);
        firearm_feed_to(&mut g, t, 0.35, None);
        assert_eq!(st(&g, t), (1.0, Some(1.0), vec![6.0, 6.0, 6.0], -1.0, 2.0, true, false, true));
        assert_eq!(firearm_reload_row(&mut g, t).dur, 1.0);
        firearm_rearm(&mut g, t);
        assert_eq!(st(&g, t), (1.0, Some(6.0), vec![6.0, 6.0, 6.0], -1.0, 2.0, true, false, false));
    }

    #[test]
    fn the_selector_and_the_string() {
        let mut g = Game::blank();
        let r = rifle_new_in(&mut g, None);
        assert!(!firearm_automatic(&mut g, r));
        assert_eq!(firearm_cycle_mode(&mut g, r), FireMode::Auto);
        assert!(firearm_automatic(&mut g, r));
        assert_eq!(firearm_auto_spread(&mut g, r), 0.0);
        firearm_auto_fired(&mut g, r);
        firearm_auto_fired(&mut g, r);
        assert_eq!(firearm_auto_spread(&mut g, r), 0.024);
        firearm_auto_idle(&mut g, r, 100.0, true);
        assert_eq!(g.fa[r].auto_idx, 2.0);
        firearm_auto_idle(&mut g, r, 100.0, false);
        assert_eq!(g.fa[r].auto_idx, 0.0);
        assert_eq!(rifle_cycle_attachment_of(&mut g, r), Some(RifleAttachment::He40));
        assert_eq!(rifle_left_of(&mut g, r, RifleAttachment::He40), 3.0);
        assert_eq!(rifle_cycle_attachment_of(&mut g, r), Some(RifleAttachment::Ring));
    }

    #[test]
    fn ocp_spends_all_and_climbs_back() {
        let mut o = Ocp::blank();
        assert!(ocp_ready(&mut o));
        assert!(ocp_use(&mut o));
        assert_eq!(o.charge, 0.0);
        assert!(!ocp_use(&mut o));
        ocp_update(&mut o, 1.0);
        assert_eq!(o.charge, 0.28);
        ocp_update(&mut o, 100.0);
        assert_eq!(o.charge, 1.0);
    }
}
