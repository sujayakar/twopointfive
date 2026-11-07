//! src/game/fogofwar.ts lines 355 to 806
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `MKEY.<system> >>> 24`
fn sys_of(k: f64) -> u32 { js::to_u32(k) >> 24 }

/// A box's row (`Box.*`, scene/boxes.ts:66). Skeleton 5 has `g.col.boxes: Vec<BoxId>` and no table of Box rows, while the index says `g.col.boxes[id].c`: once the table stands, this one line is that row.
fn bx<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

/// Which thing a keyed box belongs to, or -1 for one the rule leaves alone. (fogofwar.ts thingKeyOf)
pub fn thing_key_of(g: &mut crate::game::Game, b: BoxId) -> f64 {
    let k = bx(g, b).mkey.expect("thingKeyOf: a box with no mkey (fogofwar.ts:364 `b.mkey!`)");
    key_of_mkey(g, k)
}

/// thingKeyOf from the key itself.
fn key_of_mkey(g: &Game, k: f64) -> f64 {
    let sys = sys_of(k);
    let low = js::to_u32(k) & 0xFFFFFF;
    // (the second test is THING_PASS's `MKEY.skin >>> 24`, line 355: the skeleton's first value of the set stops at device: notes, Skeleton)
    if g.m.fogofwar.thing_pass.has(crate::Num(sys as f64)) || sys == sys_of(crate::scene_boxes::mkey().skin) { return -1.0; }
    if sys as f64 == THING_TEST {
        if low >= 0x4000 && low < 0x5000 { return -1.0; }
        let part = if low >= FIXTURE_LO as u32 && low < FIXTURE_HI as u32 && (low & 31) < FIXTURE_BODY as u32 { low >> 5 } else { low };
        return sys as f64 * 0x1000000 as f64 + part as f64;
    }
    // (THING_BITS[sys] ?? 0: the table stops at the item's system)
    let bits = thing_bits().get(sys as usize).copied().unwrap_or(0.0) as u32;
    sys as f64 * 0x1000000 as f64 + (low >> bits) as f64
}

/// (fogofwar.ts isProp)
pub fn is_prop(thing_key: f64) -> bool {
    m::floor(thing_key / 0x1000000 as f64) == PROP_SYS
}

/// A box copied for the picture's memory; its vectors are values in Rust, so nothing follows the live one. (fogofwar.ts ghostBox)
/// (A copy the record keeps until it takes the thing afresh: `box_kept`, and `box_let_go` for the copies before it: thing_take.)
pub fn ghost_box(g: &mut crate::game::Game, s: BoxId) -> BoxId {
    let copy = g.tab.boxes[s].clone();
    crate::scene_boxes::box_kept(g, copy)
}

/// What the team's sight lines pass: glass and the statics marked thin. (fogofwar.ts fogThrough)
pub fn fog_through(g: &crate::game::Game) -> f64 {
    let f = crate::scene_boxes::box_flag();
    (js::to_i32(f.glass) | js::to_i32(f.thin)) as f64
}

/// A level's script puts a man in the picture, or takes him out. (FogOfWar.show)
pub fn fog_of_war_show(g: &mut crate::game::Game, gd: GuardId, on: bool) {
    if on { g.fog_of_war.shown.add(gd); } else { g.fog_of_war.shown.delete(gd); }
}


/// `blank` is the field initialisers, and the TypeScript's constructor only keeps `g`. (FogOfWar constructor)
pub fn fog_of_war_new(g: &crate::game::Game) -> crate::fogofwar::FogOfWar {
    crate::fogofwar::FogOfWar::blank()
}

/// The switch, and no show running. (FogOfWar.active)
pub fn fog_of_war_active(g: &crate::game::Game) -> bool {
    g.fog_of_war.on && !g.touring && !g.quiet_utility
}

/// Is this man on the team whose sight it is. (FogOfWar.onTeam)
pub fn fog_of_war_on_team(g: &crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative
}

fn fresh_rec() -> crate::fogofwar::Rec {
    crate::fogofwar::Rec { seen: false, framed: None, t: -1e9, at: None, wand_t: None, body: false, smoked: false, smoke_f: -1e9 }
}

fn bone_or(g: &crate::game::Game, c: CharId, b: crate::character::Bone, y: f64) -> V3 {
    match g.ch[c].bones.get(b) {
        Some(v) => *v,
        None => v3::add(g.ch[c].pos, [0.0, y, 0.0]),
    }
}

fn chest_of(g: &crate::game::Game, c: CharId) -> V3 { bone_or(g, c, crate::character::Bone::Chest, 1.1) }

/// `men as players`
fn players(g: &mut crate::game::Game) -> Vec<PlayerId> { crate::seats::men(g) }

fn is_dead(g: &crate::game::Game, gd: GuardId) -> bool { g.gd[gd].state == crate::game::GuardState::Dead }

/// `fow…` key of one eye's smoke line to one man: 'W' for the player's lens, 'W' + id for a teammate's ordered one. (fogofwar.ts:539)
fn smoke_key(e: &crate::fogofwar::FogEye, pid: f64, man: f64) -> String {
    if e.wand == Some(true) {
        if e.id == pid { alloc::format!("fowW_{}", man) } else { alloc::format!("fowW{}_{}", e.id, man) }
    } else { alloc::format!("fow{}_{}", e.id, man) }
}

/// One man's lines waiting on the smoke (update()'s jobs).
struct Job { gd: GuardId, chest: V3, pairs: Vec<(String, crate::fogofwar::FogEye)> }

/// Once a frame after everybody has moved; asked outside a tick, the pass stands the copy of the box world for itself. (FogOfWar.update)
pub fn fog_of_war_update(g: &mut crate::game::Game, dt: f64) {
    if g.col.mirror.live { fog_of_war_pass(g, dt); } else { crate::scene_collision::window(g, &mut |g| fog_of_war_pass(g, dt)); }
}

/// The eyes, then seen or not for every man off the team, then the doors. (FogOfWar.pass)
pub fn fog_of_war_pass(g: &mut crate::game::Game, _dt: f64) {
    if !fog_of_war_active(g) {
        // (the fog has gone off since the pass last ran: what the team knew is dropped, and the picture's block with it)
        if g.fog_of_war.live || g.fog_of_war.loaded {
            crate::fogofwar::picture::off(g);
            let f = &mut g.fog_of_war;
            f.loaded = false; f.props_loaded = false; f.eyes = Vec::new(); f.hidden_chars.clear(); f.pending.clear(); f.door_recs.clear(); f.doors_held = 0.0; f.thing_recs.clear();
            f.fired_n = JsMap::new(); f.fired_at = JsMap::new(); f.fire_clock = 0.0; f.lamp_recs.clear();
            f.live = false;
            g.engine.lights.fog_team.on = false;
            g.engine.lights.fog_team.owners.clear();
            g.engine.lights.fog_team.lights.clear();
        }
        return;
    }
    g.fog_of_war.loaded = false;
    g.fog_of_war.stats.frames += 1.0;
    g.fog_of_war.frame_no += 1.0;
    let frame = g.fog_of_war.frame_no;
    // ---- the eyes
    let mut eyes: Vec<crate::fogofwar::FogEye> = Vec::new();
    let cap = m::min(crate::render_fogofwar::FOG_EYES, fog_eye_count(g.k.fog.eyes) + g.seats.len() as f64 - 1.0);
    if !crate::rails::rails_sky_on(g) {
        crate::seats::each_seat(g, &mut |g| {
            if g.seats.len() != 1 && eyes.len() as f64 >= cap { return; }   // (v !== g: only a view differs, and a game of one lends itself)
            let c = g.pl[g.player].ch;
            let own = fog_of_war_eye_of(g, c);
            let e = match crate::wand::wand_eye(g) {
                Some(e) => e,
                None => crate::peeksight::peek_eye(g, own).unwrap_or(own),
            };
            eyes.push(e);
        });
    }
    if !g.k.fog.own || crate::rails::rails_sky_on(g) {
        let mut i = 0;
        while i < g.guards.len() {
            let gd = g.guards[i];
            i += 1;
            if eyes.len() as f64 >= cap { break; }
            if !fog_of_war_on_team(g, gd) || is_dead(g, gd) || crate::control::man::blind(g, gd) { continue; }
            let e = match crate::wandorder::mate_wand_eye(g, gd) {
                Some(e) => e,
                None => { let c = g.gd[gd].ch; fog_of_war_eye_of(g, c) }
            };
            eyes.push(e);
        }
    } else {
        // (only my own eyes: an ordered lens is a device feed, as a sticky camera's is)
        crate::seats::each_seat(g, &mut |g| {
            for e in crate::wandorder::mate_wand_eyes(g) {
                if eyes.len() as f64 >= cap { break; }
                eyes.push(e);
            }
        });
    }
    crate::seats::each_seat(g, &mut |g| {
        for e in crate::stickycam::sticky_eyes(g) {
            if eyes.len() as f64 >= cap { g.stats.stickycam.blind += 1; break; }
            eyes.push(e);
        }
    });
    // (the frame's eyes: without RAILS.eyeView `pic` IS `eyes`, so what is pushed here is every man's eye too)
    let mut pic_own: Option<Vec<crate::fogofwar::FogEye>> = None;
    let mut view: Option<Vec<crate::fogofwar::FogEye>> = None;
    {
        let e = crate::rails::rails_eye(g);
        let s = crate::rails::rails_step_eye(g);
        let first = s.is_some();
        let own = e.is_some() || s.is_some();
        if own { pic_own = Some(eyes.clone()); view = Some(Vec::new()); }
        let pic: &mut Vec<crate::fogofwar::FogEye> = match pic_own.as_mut() { Some(p) => p, None => &mut eyes };
        if let (Some(s), true) = (s, first) {
            if pic.len() as f64 >= cap { pic.truncate((cap - 1.0) as usize); }
            pic.push(s);
            if let Some(v) = view.as_mut() { v.push(s); }
        }
        if let Some(e) = e {
            if (pic.len() as f64) < cap {
                pic.push(e);
                if let Some(v) = view.as_mut() { v.push(e); }
            }
        }
        if let (Some(s), false) = (s, first) {
            if (pic.len() as f64) < cap {
                pic.push(s);
                if let Some(v) = view.as_mut() { v.push(s); }
            }
        }
    }
    let pic = match pic_own { Some(p) => p, None => eyes.clone() };
    g.fog_of_war.stats.eyes = pic.len() as f64;
    g.fog_of_war.eyes = pic;
    // (the squad's own men for the doors' rule)
    g.fog_of_war.mates.clear();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if fog_of_war_on_team(g, gd) && !is_dead(g, gd) && !crate::control::man::blind(g, gd) {
            let c = g.gd[gd].ch;
            g.fog_of_war.mates.push(c);
        }
    }
    // (the men the squad is shooting at)
    g.fog_of_war.fire_clock += _dt;
    let mut i = 0;
    while i < g.fight.ops.len() {
        // (firedN is keyed by the Op itself: a row of a table that never shrinks, so its number is its identity, and a later fight's ops are other rows)
        let id = g.fight.ops[i];
        let key = id.0;
        let (n, tgt, ogd) = { let o = &g.tab.roomfight_op[id]; (o.rounds, o.tgt, o.gd) };
        i += 1;
        let was = g.fog_of_war.fired_n.get(key).copied();
        g.fog_of_war.fired_n.set(key, n);
        if let (Some(w), Some(t)) = (was, tgt) {
            if n > w && fog_of_war_on_team(g, ogd) && !fog_of_war_on_team(g, t) {
                let now = g.fog_of_war.fire_clock;
                g.fog_of_war.fired_at.set(t, now);
            }
        }
    }
    // ---- everybody else: seen by any of them?
    g.fog_of_war.pending.clear();
    g.fog_of_war.hidden_chars.clear();
    g.fog_of_war.stats.tested = 0.0; g.fog_of_war.stats.seen = 0.0; g.fog_of_war.stats.hidden = 0.0; g.fog_of_war.stats.smoke_hid = 0.0; g.fog_of_war.stats.by_fire = 0.0;
    let pl = players(g);
    let held = g.pl[pl[0]].holding.as_ref().map(|h| h.g);
    let held_too = if pl.len() > 1 { g.pl[pl[1]].holding.as_ref().map(|h| h.g) } else { None };
    let pid = g.ch[g.pl[pl[0]].ch].id;
    // (1) lines of sight
    let mut men: Vec<GuardId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() { let gd = g.guards[i]; i += 1; if !fog_of_war_on_team(g, gd) { men.push(gd); } }
    let mut i = 0;
    while i < g.civilians.len() {
        let cv = g.civilians[i];
        i += 1;
        let gone = match g.gd[cv].civ { Some(c) => g.cv[c].state == crate::civilians::CivState::Gone, None => false };
        if !fog_of_war_on_team(g, cv) && !gone { men.push(cv); }
    }
    let mut lines: JsMap<GuardId, bool> = JsMap::new();
    let mut jobs: Vec<Job> = Vec::new();
    fog_of_war_hand_eyes(g, &eyes);
    for &gd in men.iter() {
        let dead = is_dead(g, gd);
        if held == Some(gd) || held_too == Some(gd) { lines.set(gd, true); continue; }
        if dead {
            let seen = fog_of_war_sees(g, &eyes, IN_ORDER, 1.0, gd, None) >= ONE_EYE;
            lines.set(gd, seen);
            continue;
        }
        let c = g.gd[gd].ch;
        let chest = chest_of(g, c);
        if g.k.fog.smoke <= 0.0 {
            let seen = fog_of_war_sees(g, &eyes, IN_ORDER, 1.0, gd, Some(chest)) >= ONE_EYE;
            lines.set(gd, seen);
            continue;
        }
        // the eyes with a clear line, nearest first (positions in `eyes`: the TypeScript's indexOf of the object)
        let mut near: Vec<usize> = Vec::with_capacity(eyes.len());
        let mut near_d: Vec<f64> = Vec::with_capacity(eyes.len());
        for (ei, e) in eyes.iter().enumerate() {
            let d = m::pow(e.pos[0] - chest[0], 2.0) + m::pow(e.pos[2] - chest[2], 2.0);
            let mut k = near.len();
            near.push(0); near_d.push(0.0);
            while k > 0 && near_d[k - 1] > d { near[k] = near[k - 1]; near_d[k] = near_d[k - 1]; k -= 1; }
            near[k] = ei; near_d[k] = d;
        }
        let mut clear: Vec<usize> = Vec::new();
        let mut order: i32 = 0;
        for (k, &ei) in near.iter().enumerate() { order |= (ei as i32).wrapping_shl(4 * k as u32); }
        let r = js::to_u32(fog_of_war_sees(g, &eyes, order as f64, SMOKE_EYES, gd, Some(chest)));
        let c_n = r >> 28;
        for k in 0..c_n { clear.push(((r >> (4 * k)) & 15) as usize); }
        lines.set(gd, !clear.is_empty());
        if !clear.is_empty() {
            let man = g.ch[c].id;
            let pairs = clear.iter().map(|&ei| (smoke_key(&eyes[ei], pid, man), eyes[ei])).collect();
            jobs.push(Job { gd, chest, pairs });
        }
    }
    // (2) the smoke along those lines, through the sensors
    if !jobs.is_empty() {
        g.fog_of_war.turn = js::rem(g.fog_of_war.turn + 1.0, jobs.len() as f64);
        let turn = g.fog_of_war.turn as usize;
        for round in 0..SMOKE_EYES as usize {
            for i in 0..jobs.len() {
                if g.fog_of_war.pending.len() as f64 >= SMOKE_ASKS { break; }
                let j = (i + turn) % jobs.len();
                if round >= jobs[j].pairs.len() { continue; }
                let at = js::rem(round as f64 + frame + j as f64, jobs[j].pairs.len() as f64) as usize;
                let (key, e) = jobs[j].pairs[at].clone();
                let last = g.fog_of_war.asked.get(key.clone()).copied();
                if match last { None => true, Some(l) => frame - l > SMOKE_FRESH } { crate::sensors::sensors_forget(g, &key); }
                g.fog_of_war.asked.set(key.clone(), frame);
                let chest = jobs[j].chest;
                g.fog_of_war.pending.push(crate::fogofwar::FogOfWarPendingItem { key, a: e.pos, b: chest });
            }
        }
        g.fog_of_war.stats.smoke_asks += g.fog_of_war.pending.len() as f64;
        for jb in jobs.iter() {
            g.fog_of_war.recs.get_or_insert_with(jb.gd, fresh_rec);
            let (mut known, mut open) = (0, false);
            for (key, _) in jb.pairs.iter() {
                let last = g.fog_of_war.asked.get(key.clone()).copied();
                match last { None => continue, Some(l) => if frame - l > SMOKE_FRESH { continue; } }
                let t = match crate::sensors::sensors_read(g, key) { Some(t) => t, None => continue };
                known += 1;
                if t >= g.k.fog.smoke { open = true; }
            }
            let r = g.fog_of_war.recs.get_mut(jb.gd).unwrap();
            if known != 0 { r.smoked = !open; r.smoke_f = frame; }
            if r.smoked && frame - r.smoke_f <= SMOKE_KEEP {
                lines.set(jb.gd, false);
                g.fog_of_war.stats.smoke_hid += 1.0;
            }
        }
        if g.fog_of_war.asked.len() > 256 {
            for k in g.fog_of_war.asked.keys() {
                let f = g.fog_of_war.asked.get(k.clone()).copied().unwrap_or(frame);
                if frame - f > SMOKE_KEEP { g.fog_of_war.asked.delete(k); }
            }
        }
    }
    // (3) the verdicts
    for &gd in men.iter() { judge(g, gd, &lines, view.as_deref()); }
    for eye in eyes.iter() {
        if eye.wand != Some(true) { continue; }
        for &gd in men.iter() {
            let team = g.gd[gd].team;
            if is_dead(g, gd) || !crate::teams::is_hostile(g, team, crate::teams::Team::Player) { continue; }
            match g.fog_of_war.recs.get(gd) { Some(r) if r.seen => {}, _ => continue }
            let c = g.gd[gd].ch;
            let chest = chest_of(g, c);
            if fog_of_war_line_to_man(g, *eye, gd, chest) {
                let now = g.time;
                let wt = g.fog_of_war.recs.get(gd).unwrap().wand_t;
                if match wt { None => true, Some(w) => now - w > 1.0 || now < w } { g.stats.wand.marked += 1; }
                g.fog_of_war.recs.get_mut(gd).unwrap().wand_t = Some(now);
            }
        }
    }
    fog_of_war_tell_calls(g, _dt);
    // ---- the doors: a leaf where the picture has it needs no look; one that is not comes up to date when an eye has it or its place
    g.fog_of_war.doors_held = 0.0;
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        let angle = crate::doors::door_angle(g, d);
        let shown = g.fog_of_war.door_recs.get_or_insert_with(d, || crate::fogofwar::FogOfWarDoorRecsValue { shown: angle, ghost: None }).shown;
        if m::abs(angle - shown) <= DOOR_EPS || fog_of_war_door_seen(g, d, shown) {
            g.fog_of_war.door_recs.get_mut(d).unwrap().shown = angle;
        } else {
            g.fog_of_war.doors_held += 1.0;
        }
    }
    // ---- who the team is, and each lamp as the team last saw it: the team's own, which rules read (`ours_on`, `ours_id`) and the picture below
    fog_of_war_team(g);
    fog_of_war_lamps_known(g);
    // ---- the renderer's block (fogofwar/picture.rs): filled from the game's state and read by nothing in it
    crate::fogofwar::picture::fill(g);
}

/// Who the team is this frame, for the lights' classes and for what the team's own doing shows (`ours_id`): every seat's man and every teammate, and the
/// light each carries. From here the rule is live (`ours_on`).   (fogofwar.ts FogOfWar.fullBlock, its first half)
fn fog_of_war_team(g: &mut crate::game::Game) {
    g.fog_of_war.live = true;
    g.engine.lights.fog_team.on = true;
    g.engine.lights.fog_team.owners.clear();
    g.engine.lights.fog_team.lights.clear();
    for pl in crate::seats::men(g) {
        let id = g.ch[g.pl[pl].ch].id;
        let light = g.pl[pl].light;
        g.engine.lights.fog_team.owners.add(crate::Num(id));
        g.engine.lights.fog_team.lights.add(light);
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if fog_of_war_on_team(g, gd) {
            let id = g.ch[g.gd[gd].ch].id;
            let light = g.gd[gd].flashlight;
            g.engine.lights.fog_team.owners.add(crate::Num(id));
            g.engine.lights.fog_team.lights.add(light);
        }
    }
}

/// Every fixture that has a light, as the team last saw it (`lamp_recs`): once a frame, in the level's order. The picture reads the record and asks nothing.
fn fog_of_war_lamps_known(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        let row = &g.tab.sim_lightshome_fixture[t];
        if row.light.is_none() && row.area_lights.is_none() { continue; }
        let lit = fog_of_war_lamp_lit(g, t);
        fog_of_war_lamp_known(g, t, lit);
    }
}

/// pass()'s verdict on one man.
fn judge(g: &mut crate::game::Game, gd: GuardId, lines: &JsMap<GuardId, bool>, view: Option<&[crate::fogofwar::FogEye]>) {
    g.fog_of_war.stats.tested += 1.0;
    let rt = g.fog_of_war.recs.get_or_insert_with(gd, fresh_rec).t;
    let dead = is_dead(g, gd);
    let seen = lines.get(gd).copied() == Some(true);
    let since = g.time - rt;
    let fired = !seen && fog_of_war_fired_on(g, gd);
    let mut framed = false;
    if !seen {
        if let Some(v) = view {
            let c = g.gd[gd].ch;
            for &e in v.iter() {
                let got = if dead { fog_of_war_eye_on_body(g, e, gd) } else { let chest = chest_of(g, c); fog_of_war_line_to_man(g, e, gd, chest) };
                if got { framed = true; break; }
            }
        }
    }
    let shown = g.fog_of_war.shown.has(gd);
    let (now, linger, p) = (g.time, g.k.fog.linger, g.ch[g.gd[gd].ch].pos);
    let r = g.fog_of_war.recs.get_mut(gd).unwrap();
    r.framed = Some(framed);
    if dead && (seen || fired || framed || shown || (since >= 0.0 && since < m::max(linger, 0.5))) { r.body = true; }
    r.seen = seen;
    if seen {
        r.t = now;
        if !dead { r.at = Some(p); }
        g.fog_of_war.stats.seen += 1.0;
    }
    if !fog_of_war_drawn(g, gd) {
        let c = g.gd[gd].ch;
        g.fog_of_war.hidden_chars.add(c);
        g.fog_of_war.stats.hidden += 1.0;
    } else if fired && !dead && !(since >= 0.0 && since < linger) {
        g.fog_of_war.stats.by_fire += 1.0;
    }
}

/// A man as an eye: off his head, looking where he looks (`gaze_of`). (FogOfWar.eyeOf)
pub fn fog_of_war_eye_of(g: &crate::game::Game, c_2: CharId) -> crate::fogofwar::FogEye {
    let pos = bone_or(g, c_2, crate::character::Bone::Head, 1.65);
    let look = gaze_of(g, c_2);
    let p = g.ch[c_2].pos;
    let y = if g.ch[c_2].alive { EYE_STAND + (EYE_CROUCH - EYE_STAND) * m::min(1.0, m::max(0.0, g.ch[c_2].anim.crouch)) } else { EYE_CROUCH };
    crate::fogofwar::FogEye { id: g.ch[c_2].id, pos, look, at: [p[0], p[1] + y, p[2]], fov: None, near: None, wand: None }
}

/// The player a call is worded for: with two, the first one up. (FogOfWar.caller)
pub fn fog_of_war_caller(g: &mut crate::game::Game) -> PlayerId {
    if g.seats.len() == 1 { return g.player; }
    match crate::seats::men_up(g).first() {
        Some(&p) => p,
        None => players(g)[0],
    }
}

/// Where `p` lies for the player's man, by the way he looks. (FogOfWar.sideOf)
pub fn fog_of_war_side_of(g: &mut crate::game::Game, p: V3) -> crate::close::CloseId {
    let pl = fog_of_war_caller(g);
    let c = g.pl[pl].ch;
    let l = gaze_of(g, c);
    let at = g.ch[c].pos;
    let (dx, dz) = (p[0] - at[0], p[2] - at[2]);
    let d = js::or(m::hyp2(dx, dz), 1.0);
    let dot = (dx * l[0] + dz * l[2]) / d;
    let cross = (dx * l[2] - dz * l[0]) / d;
    if dot > COS_FRONT { crate::close::CloseId::Front }
    else if dot < COS_BEHIND { crate::close::CloseId::Behind }
    else if cross > 0.0 { crate::close::CloseId::Left }
    else { crate::close::CloseId::Right }
}

/// The words of one call: in the player's own room the side of him, else the place by the level's room names, behind him said first. (FogOfWar.callWords)
pub fn fog_of_war_call_words(g: &mut crate::game::Game, kind: crate::fogofwar::FogOfWarCallWordsKind, p: V3) -> String {
    let side = fog_of_war_side_of(g, p);
    let there = crate::dialogue::place_name(g, p);
    let me = fog_of_war_caller(g);
    let here_at = g.ch[g.pl[me].ch].pos;
    let here = crate::dialogue::place_name(g, here_at);
    let odd = js::to_i32(g.fog_of_war.call_n) & 1 != 0;
    let contact = kind == crate::fogofwar::FogOfWarCallWordsKind::Contact;
    let behind = side == crate::close::CloseId::Behind;
    if there == here {
        return if contact { alloc::format!("contact {}!", side.as_str()) }
            else if behind { String::from("movement behind us!") }
            else { alloc::format!("movement \u{2014} {}!", side.as_str()) };
    }
    let at = if there.starts_with("outside ") { there.clone() } else { alloc::format!("in {}", there) };
    if contact {
        return if behind { alloc::format!("behind us \u{2014} contact {}!", at) }
            else if odd { alloc::format!("eyes on one {}!", at) }
            else { alloc::format!("contact \u{2014} {}!", at) };
    }
    if behind { alloc::format!("movement behind us \u{2014} {}!", there) }
    else if odd { alloc::format!("steps \u{2014} {}!", there) }
    else { alloc::format!("movement \u{2014} {}!", there) }
}

/// tellCalls' `fresh`: not called lately, and the player's own eye has not had him lately.
fn call_fresh(g: &Game, h: GuardId, now: f64) -> bool {
    let called = g.fog_of_war.called_t.get(h).copied().unwrap_or(-1e9);
    let known = g.fog_of_war.known_t.get(h).copied().unwrap_or(-1e9);
    now - called >= g.k.fog.call_secs && now - known >= g.k.fog.call_known
}

/// tellCalls' `say`: one call, logged and on the air.
fn call_say(g: &mut Game, gd: GuardId, h: GuardId, kind: crate::fogofwar::FogOfWarCallWordsKind, at: V3, now: f64) {
    let text = fog_of_war_call_words(g, kind, at);
    let (who, of, t) = (g.gd[gd].callsign.clone(), g.gd[h].callsign.clone(), to_fixed2(g.time));
    let f = &mut g.fog_of_war;
    f.call_n += 1.0;
    f.last_call = now;
    f.called_t.set(h, now);
    let log_kind = match kind {
        FogOfWarCallWordsKind::Contact => { f.call_stats.contact += 1.0; FogOfWarCallLogItemKind::Contact }
        FogOfWarCallWordsKind::Heard => { f.call_stats.heard += 1.0; FogOfWarCallLogItemKind::Heard }
    };
    f.call_log.push(FogOfWarCallLogItem { t, who, of, kind: log_kind, text: text.clone() });
    if f.call_log.len() > 200 { f.call_log.drain(0..50); }
    crate::cues::cues_call(g, gd, &text);
}

/// `+x.toFixed(2)`, for the QA's log only.
fn to_fixed2(x: f64) -> f64 { js::to_fixed(x, 2).parse().unwrap_or(x) }

/// What the cone hides is called: a contact from a teammate's eyes, else a step in his ears. Under cones or own eyes only (hard). (FogOfWar.tellCalls)
pub fn fog_of_war_tell_calls(g: &mut crate::game::Game, dt: f64) {
    g.fog_of_war.call_clock += dt;
    let now = g.fog_of_war.call_clock;
    if !(g.k.fog.cone || g.k.fog.own) || g.fog_of_war.eyes.is_empty() || crate::seats::all_down(g) || js::truthy(js::rem(g.fog_of_war.frame_no, CALL_EVERY)) { return; }
    let me = g.fog_of_war.eyes[0];
    let mut foes: Vec<GuardId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        if is_dead(g, h) || !g.ch[g.gd[h].ch].alive { continue; }
        let team = g.gd[h].team;
        if !crate::teams::is_hostile(g, team, crate::teams::Team::Player) { continue; }
        let pl = players(g);
        if pl.iter().any(|&p| g.pl[p].holding.as_ref().map(|x| x.g) == Some(h)) { continue; }
        if crate::runner::runner_nobody(g, h) { continue; }
        foes.push(h);
    }
    // (whom the player's own eye has, in the picture: nobody need tell him)
    for &h in foes.iter() {
        if !g.fog_of_war.recs.get(h).map_or(false, |r| r.seen) { continue; }
        let chest = chest_of(g, g.gd[h].ch);
        if fog_of_war_line_to_man(g, me, h, chest) { g.fog_of_war.known_t.set(h, now); }
    }
    if now - g.fog_of_war.last_call < g.k.fog.call_gap { return; }
    let mut mates: Vec<GuardId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if fog_of_war_on_team(g, gd) && !is_dead(g, gd) && !crate::control::man::blind(g, gd) && g.gd[gd].held.is_none() { mates.push(gd); }
    }
    // contact: the first teammate, in list order, with one of them in his eyes
    for &gd in mates.iter() {
        let c = g.gd[gd].ch;
        let e = fog_of_war_eye_of(g, c);
        for &h in foes.iter() {
            if !call_fresh(g, h, now) { continue; }
            let chest = chest_of(g, g.gd[h].ch);
            if !fog_of_war_line_to_man(g, e, h, chest) { continue; }
            call_say(g, gd, h, crate::fogofwar::FogOfWarCallWordsKind::Contact, chest, now);
            return;
        }
    }
    // heard: a step in a teammate's ears from somebody no eye of the fog's has
    let mut i = 0;
    while i < g.fight.ops.len() {
        let id = g.fight.ops[i];
        i += 1;
        let (ogd, sb) = { let o = &g.tab.roomfight_op[id]; (o.gd, o.step_by.clone()) };
        let sb = match sb { Some(s) => s, None => continue };
        if is_dead(g, ogd) || g.fog_of_war.told_step.get(ogd).copied() == Some(sb.t) { continue; }
        g.fog_of_war.told_step.set(ogd, sb.t);
        if is_dead(g, sb.by) || !call_fresh(g, sb.by, now) || g.fog_of_war.recs.get(sb.by).map_or(false, |r| r.seen) || crate::runner::runner_nobody(g, sb.by) { continue; }
        call_say(g, ogd, sb.by, crate::fogofwar::FogOfWarCallWordsKind::Heard, sb.at, now);
        return;
    }
}

/// Is `p` inside this eye's field in plan. (FogOfWar.inField)
pub fn fog_of_war_in_field(g: &crate::game::Game, e: crate::fogofwar::FogEye, p: V3) -> bool {
    let fov = match e.fov { Some(f) => f, None => field_deg(g) };
    if fov >= 360.0 { return true; }
    let (dx, dz) = (p[0] - e.pos[0], p[2] - e.pos[2]);
    let d = m::hyp2(dx, dz);
    let near = match e.near { Some(n) => n, None => field_near(g) };
    if d < near || d < 1e-4 { return true; }
    let l = js::or(m::hyp2(e.look[0], e.look[2]), 1.0);
    (dx * e.look[0] + dz * e.look[2]) / (d * l) > m::cos(fov * 0.5 * DEG)
}

/// The frame's smoke segments, queued for the probe after everybody else's. (FogOfWar.flushAsks)
pub fn fog_of_war_flush_asks(g: &mut crate::game::Game) {
    if g.fog_of_war.pending.is_empty() { return; }
    let pending = core::mem::take(&mut g.fog_of_war.pending);
    for p in pending.iter() {
        let room = if g.sensors.own.is_some() { None } else { Some(crate::sensors::probe_like_room(g)) };
        let full = match room { Some(r) => r <= 0.0, None => false };
        crate::sensors::sensors_ask_segment(g, &p.key, p.a, p.b);
        if full { crate::sensors::sensors_forget(g, &p.key); }
    }
    g.fog_of_war.pending.clear();
}

/// A fresh encounter: nobody has been seen, no pair asked. (FogOfWar.reset)
pub fn fog_of_war_reset(g: &mut crate::game::Game) {
    crate::fogofwar::picture::reset(g);
    let f = &mut g.fog_of_war;
    f.lamp_recs.clear(); f.door_recs.clear(); f.doors_held = 0.0; f.thing_recs.clear(); f.props_loaded = false; f.loaded = false; f.our_landed.clear(); f.thing_since = -1.0;
    f.recs = JsMap::new(); f.asked.clear(); f.pending.clear(); f.turn = 0.0; f.frame_no = 0.0; f.hidden_chars.clear(); f.fired_n = JsMap::new(); f.fired_at = JsMap::new(); f.shown = JsSet::new(); f.fire_clock = 0.0;
    f.call_clock = 0.0; f.last_call = -1e9; f.called_t = JsMap::new(); f.known_t = JsMap::new(); f.told_step = JsMap::new(); f.call_n = 0.0; f.call_log = Vec::new(); f.call_stats = crate::fogofwar::FogOfWarCallStats { contact: 0.0, heard: 0.0 };
}

/// The fog's own line: anything solid between an eye and a point. (FogOfWar.sightBlocked)
pub fn fog_of_war_sight_blocked(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    let through = fog_through(g);
    crate::scene_collision::segment_blocked(g, a, b, Some(0.02), Some(through), None)
}


/// One eye on one living man, walls and leaves only, inside the eye's field. (FogOfWar.lineToMan)
pub fn fog_of_war_line_to_man(g: &mut crate::game::Game, e: crate::fogofwar::FogEye, gd: GuardId, chest: V3) -> bool {
    let pc = g.gd[gd].ch;
    if !fog_of_war_in_field(g, e, chest) { return false; }
    let to = v3::sub(chest, e.pos);
    let l = m::hyp2(to[0], to[2]);
    if l <= 1e-3 { return true; }
    if !g.ch[pc].alive { return false; }
    if !fog_of_war_sight_blocked(g, e.pos, chest) { return true; }
    let head = bone_or(g, pc, crate::character::Bone::Head, 1.6);
    !fog_of_war_sight_blocked(g, e.pos, head)
}

/// The eyes with a line to a man (`chest` given) or his body (None), one question to the module: how many × 2^28 + those eyes, four bits each. (FogOfWar.sees)
pub fn fog_of_war_sees(g: &mut crate::game::Game, eyes: &[crate::fogofwar::FogEye], order: f64, want: f64, gd: GuardId, chest: Option<V3>) -> f64 {
    let pc = g.gd[gd].ch;
    let (a, b, kind): (V3, V3, f64) = match chest {
        Some(c) => (c, bone_or(g, pc, crate::character::Bone::Head, 1.6), if g.ch[pc].alive { 3.0 } else { 1.0 }),
        None => {
            let hp = match g.ch[pc].bones.get(crate::character::Bone::Hips) { Some(v) => *v, None => g.ch[pc].pos };
            let at = [hp[0], m::max(hp[1], 0.0) + 0.3, hp[2]];
            (at, at, 0.0)
        }
    };
    let through = fog_through(g);
    crate::scene_collision::eyes_see(g, order, eyes.len() as f64, want, kind, through, a, b)
}

/// The eyes of a pass handed to the module, each with the field `in_field` holds it to. (FogOfWar.handEyes)
pub fn fog_of_war_hand_eyes(g: &mut crate::game::Game, eyes: &[crate::fogofwar::FogEye]) {
    crate::scene_collision::sight_open(g, eyes.len() as f64);
    for (k, e) in eyes.iter().enumerate() {
        let fov = match e.fov { Some(f) => f, None => field_deg(g) };
        let near = match e.near { Some(n) => n, None => field_near(g) };
        let look_len = js::or(m::hyp2(e.look[0], e.look[2]), 1.0);
        crate::scene_collision::sight_eye(g, k as f64, e.pos, e.look, look_len, m::cos(fov * 0.5 * DEG), near, fov >= 360.0);
    }
}

/// One eye on a body: a clear line to a hand's height over where it lies, inside the eye's field. (FogOfWar.eyeOnBody)
pub fn fog_of_war_eye_on_body(g: &mut crate::game::Game, e: crate::fogofwar::FogEye, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    let hp = match g.ch[c].bones.get(crate::character::Bone::Hips) { Some(v) => *v, None => g.ch[c].pos };
    let at = [hp[0], m::max(hp[1], 0.0) + 0.3, hp[2]];
    fog_of_war_in_field(g, e, at) && !fog_of_war_sight_blocked(g, e.pos, at)
}

/// Can any eye see this point. (FogOfWar.eyeOn)
pub fn fog_of_war_eye_on(g: &mut crate::game::Game, p: V3) -> bool {
    let mut i = 0;
    while i < g.fog_of_war.eyes.len() {
        let e = g.fog_of_war.eyes[i];
        i += 1;
        if fog_of_war_in_field(g, e, p) && !fog_of_war_sight_blocked(g, e.pos, p) { return true; }
    }
    false
}

/// Does anybody on the team see this door: a line to the leaf where it stands or where the picture has it. (FogOfWar.doorSeen)
pub fn fog_of_war_door_seen(g: &mut crate::game::Game, d: DoorId, shown: f64) -> bool {
    let (hinge, w, closed) = { let def = &g.dr[d].def; (def.hinge, def.width, def.closed_dir) };
    let (hx, hz) = (hinge[0], hinge[1]);
    let eyes = g.fog_of_war.eyes.clone();
    let mates = g.fog_of_war.mates.clone();
    let reach = g.k.fog.squad_doors_m * g.k.fog.squad_doors_m;
    let (fs, ys) = (door_f(), door_y());
    for ai in 0..2 {
        let turn = if ai != 0 { shown } else if crate::doors::door_off(g, d) { 0.0 } else { crate::doors::door_angle(g, d) };
        let a = closed + turn;
        let (ux, uz) = (m::cos(a), m::sin(a));
        for fi in 0..3 {
            for yi in 0..2 {
                let f = fs[fi];
                let p: V3 = [hx + ux * w * f, ys[yi], hz + uz * w * f];
                for &e in eyes.iter() {
                    if !fog_of_war_in_field(g, e, p) { continue; }
                    let l = v3::dist(e.pos, p);
                    if l < 0.3 { return true; }
                    if !fog_of_war_sight_blocked(g, e.pos, v3::lerp3(e.pos, p, 1.0 - 0.12 / l)) { return true; }
                }
                // (a door the squad works: a teammate at it sees it move, whatever he is looking at)
                for &c in mates.iter() {
                    let at = g.ch[c].pos;
                    let (dx, dz) = (at[0] - p[0], at[2] - p[2]);
                    if dx * dx + dz * dz > reach { continue; }
                    let from = bone_or(g, c, crate::character::Bone::Head, 1.6);
                    let l = v3::dist(from, p);
                    if l < 0.3 { return true; }
                    if !fog_of_war_sight_blocked(g, from, v3::lerp3(from, p, 1.0 - 0.12 / l)) { return true; }
                }
            }
        }
    }
    false
}

/// Is this door drawn where the team last saw it rather than where it stands. (FogOfWar.doorHeld)
pub fn fog_of_war_door_held(g: &mut crate::game::Game, d: DoorId) -> bool {
    let shown = match g.fog_of_war.door_recs.get(d) { Some(r) => r.shown, None => return false };
    fog_of_war_active(g) && m::abs(crate::doors::door_angle(g, d) - shown) > DOOR_EPS
}

/// The angle the picture has this door at. (FogOfWar.doorShown)
pub fn fog_of_war_door_shown(g: &mut crate::game::Game, d: DoorId) -> f64 {
    if let Some(s) = g.fog_of_war.door_recs.get(d).map(|r| r.shown) {
        if fog_of_war_active(g) { return s; }
    }
    crate::doors::door_angle(g, d)
}

/// The door a box in the picture's list is the remembered leaf of. (FogOfWar.doorOfBox)
pub fn fog_of_war_door_of_box(g: &crate::game::Game, box_: BoxId) -> Option<DoorId> {
    for (d, r) in g.fog_of_war.door_recs.iter() {
        if let Some(gh) = r.ghost.as_ref() {
            if gh[0] == box_ { return Some(d); }
        }
    }
    None
}

/// The doors' boxes for the picture: each leaf where it stands, or for one moved unseen, copies at the angle last seen. (FogOfWar.doorBoxes)
pub fn fog_of_war_door_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    g.fog_of_war.door_n = 0.0;
    if !fog_of_war_active(g) || !js::truthy(g.fog_of_war.doors_held) {
        crate::doors::doors_boxes(g, out);
        return;
    }
    g.fog_of_war.door_from = out.len() as f64;
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        let (bx_, h0, h1) = { let r = &g.dr[d]; (r.box_, r.handle_boxes[0], r.handle_boxes[1]) };
        let angle = crate::doors::door_angle(g, d);
        let shown = match g.fog_of_war.door_recs.get(d).map(|r| r.shown) {
            Some(s) if m::abs(angle - s) > DOOR_EPS => s,
            _ => { out.push(bx_); out.push(h0); out.push(h1); continue; }
        };
        let pose = crate::doors::door_pose_at(g, d, shown);
        let had = g.fog_of_war.door_recs.get(d).and_then(|r| r.ghost.clone());
        let gh = match had {
            Some(gh) => gh,
            None => {
                let gh = vec![crate::scene_boxes::copy_box(g, bx_), crate::scene_boxes::copy_box(g, h0), crate::scene_boxes::copy_box(g, h1)];
                g.fog_of_war.door_recs.get_mut(d).unwrap().ghost = Some(gh.clone());
                gh
            }
        };
        let flags = bx(g, bx_).flags;
        let q = bx(g, gh[0]);
        q.c = pose.c; q.yaw = pose.yaw; q.flags = flags;
        let had_rot = q.rot.is_some();
        if crate::doors::door_off(g, d) || had_rot {
            let (h, marks) = (g.dr[d].leaf_h, g.dr[d].leaf_marks.clone());
            let q = bx(g, gh[0]);
            q.h = h; q.marks = marks; q.rot = None;
            bx(g, gh[1]).rot = None;
            bx(g, gh[2]).rot = None;
        }
        for s in 0..2 { let q = bx(g, gh[1 + s]); q.c = pose.handles[s]; q.yaw = pose.yaw; }
        out.push(gh[0]); out.push(gh[1]); out.push(gh[2]);
    }
    g.fog_of_war.door_n = out.len() as f64 - g.fog_of_war.door_from;
}

/// Has the team a look at this fixture: an eye on the floor under it, a line to the fixture itself, or seen floor within two metres that it lights. (FogOfWar.lampSeen)
pub fn fog_of_war_lamp_seen(g: &mut crate::game::Game, t: FixtureId) -> bool {
    let pos = g.tab.sim_lightshome_fixture[t].pos;
    let (x, y, z) = (pos[0], pos[1], pos[2]);
    if fog_of_war_spot_seen(g, x, z) { return true; }
    let mut n = 0;
    while n < g.fog_of_war.eyes.len() {
        let e = g.fog_of_war.eyes[n];
        n += 1;
        if !fog_of_war_in_field(g, e, pos) { continue; }
        let dx = x - e.at[0];
        let dy = y - e.at[1];
        let dz = z - e.at[2];
        let d = m::hyp3(dx, dy, dz);
        if d < 0.5 { return true; }
        let k = 1.0 - 0.3 / d;
        if !fog_of_war_sight_blocked(g, e.at, [e.at[0] + dx * k, e.at[1] + dy * k, e.at[2] + dz * k]) { return true; }
    }
    let from: V3 = [x, m::max(0.3, y - 0.3), z];
    let mut i = 0;
    while i < 8 {
        let a = i as f64 * m::PI / 4.0;
        let r = if i & 1 != 0 { 2.0 } else { 1.0 };
        let px = x + m::cos(a) * r;
        let pz = z + m::sin(a) * r;
        if !crate::scene_collision::segment_blocked(g, from, [px, 0.3, pz], None, None, None) && fog_of_war_spot_seen(g, px, pz) { return true; }
        i += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eye(id: f64, wand: Option<bool>) -> crate::fogofwar::FogEye {
        crate::fogofwar::FogEye { id, pos: [0.0; 3], look: [0.0, 0.0, 1.0], at: [0.0; 3], fov: Some(90.0), near: Some(1.0), wand }
    }

    /// JavaScript's text of a whole number, under the three shapes of line 539: a key the sensors and the TypeScript's journal agree on.
    #[test]
    fn smoke_keys_read_as_the_typescripts() {
        assert_eq!(smoke_key(&eye(5.0, None), 5.0, 9.0), "fow5_9");
        assert_eq!(smoke_key(&eye(5.0, Some(true)), 5.0, 9.0), "fowW_9");
        assert_eq!(smoke_key(&eye(7.0, Some(true)), 5.0, 9.0), "fowW7_9");
    }

    /// inField's three ways in: all round, the near field, the wedge (cos 45 degrees either side of the look).
    #[test]
    fn in_field_takes_the_near_field_and_the_wedge() {
        let mut g = Game::blank();
        let e = eye(1.0, None);
        assert!(fog_of_war_in_field(&mut g, e, [0.0, 0.0, 5.0]));
        assert!(fog_of_war_in_field(&mut g, e, [0.5, 0.0, -0.5]), "behind him but inside the near field");
        assert!(!fog_of_war_in_field(&mut g, e, [0.0, 0.0, -5.0]));
        assert!(fog_of_war_in_field(&mut g, e, [4.0, 0.0, 5.0]));
        assert!(!fog_of_war_in_field(&mut g, e, [6.0, 0.0, 5.0]));
        let round = crate::fogofwar::FogEye { fov: Some(360.0), ..e };
        assert!(fog_of_war_in_field(&mut g, round, [0.0, 0.0, -5.0]));
    }

    /// A prop's parts share a key, and thing keys sit above 2^24 by system (fogofwar.ts:382).
    #[test]
    fn is_prop_goes_by_the_system() {
        assert!(is_prop(PROP_SYS * 0x1000000 as f64 + 1234.0));
        assert!(!is_prop((PROP_SYS + 1.0) * 0x1000000 as f64));
        assert!(!is_prop(-1.0));
    }

    /// The keys thingKeyOf gives, as the snapshot's own function gives them under bun (values in the test).
    #[test]
    fn thing_keys_are_the_typescripts() {
        let g = Game::blank();
        const SYS: f64 = 0x1000000 as f64;
        for (k, want) in [
            (1.0 * SYS + 5.0, -1.0),                   // a leaf
            (4.0 * SYS + 5.0, -1.0),                   // what a man carries
            (5.0 * SYS + 5.0, -1.0),                   // a wall device
            (10.0 * SYS + 5.0, -1.0),                  // a holed wall's paint (line 355)
            (7.0 * SYS + 0x4001 as f64, -1.0),         // a moth
            (2.0 * SYS + 35.0, 2.0 * SYS + 2.0),       // a prop's parts share a key
            (3.0 * SYS + 35.0, 3.0 * SYS + 4.0),       // an item's
            (8.0 * SYS + 35.0, 8.0 * SYS + 35.0),      // debris: a thing by itself
            (7.0 * SYS + (0x5000 + (3 << 5) + 2) as f64, 7.0 * SYS + ((0x5000 + (3 << 5) + 2) >> 5) as f64), // a broken fixture's body
            (7.0 * SYS + (0x5000 + (3 << 5) + 6) as f64, 7.0 * SYS + (0x5000 + (3 << 5) + 6) as f64),        // what came off it
            (7.0 * SYS + 0x8000 as f64, 7.0 * SYS + 0x8000 as f64),                                          // main.ts's cut test pieces
        ] { assert_eq!(key_of_mkey(&g, k), want, "mkey {}", k); }
    }

    /// `+x.toFixed(2)`: a tie goes up, as the language has it, not to even.
    #[test]
    fn to_fixed2_rounds_as_the_log_shows() {
        assert_eq!(to_fixed2(12.3456), 12.35);
        assert_eq!(to_fixed2(3.0), 3.0);
        assert_eq!(to_fixed2(0.125), 0.13);
    }
}
