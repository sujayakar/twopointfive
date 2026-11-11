use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `MKEY.<system> >>> 24`
fn sys_of(k: f64) -> u32 { js::to_u32(k) >> 24 }


/// Which thing a keyed box belongs to, or -1 for one the rule leaves alone.
pub(super) fn thing_key_of(g: &crate::game::Game, b: AnyBoxId) -> f64 {
    let k = g.tab[b].mkey.expect("thingKeyOf: a box with no mkey (fogofwar.ts:364 `b.mkey!`)");
    key_of_mkey(g, k)
}

/// thingKeyOf from the key itself.
pub(super) fn key_of_mkey(g: &Game, k: f64) -> f64 {
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
    let bits = THING_BITS.get(sys as usize).copied().unwrap_or(0.0) as u32;
    sys as f64 * 0x1000000 as f64 + (low >> bits) as f64
}

pub(super) fn is_prop(thing_key: f64) -> bool {
    m::floor(thing_key / 0x1000000 as f64) == PROP_SYS
}

/// A box copied for the picture's memory; its vectors are values in Rust, so nothing follows the live one.
/// (A copy the record keeps until it takes the thing afresh: `box_kept`, and `box_let_go` for the copies before it: thing_take.)
pub(super) fn ghost_box(g: &mut crate::game::Game, s: AnyBoxId) -> FrameBoxId {
    let copy = g.tab[s].clone();
    crate::scene_boxes::box_kept(g, copy)
}

/// What the team's sight lines pass: glass and the statics marked thin.
pub fn fog_through() -> f64 {
    let f = crate::scene_boxes::box_flag();
    (js::to_i32(f.glass) | js::to_i32(f.thin)) as f64
}

/// `blank` is the field initialisers, and the TypeScript's constructor only keeps `g`. (FogOfWar constructor)
pub fn fog_of_war_new() -> crate::fogofwar::FogOfWar {
    crate::fogofwar::FogOfWar::blank()
}

/// The switch, and somebody plays.
pub fn fog_of_war_active(g: &crate::game::Game) -> bool {
    g.fog_of_war.on && !g.aside
}

fn fresh_rec() -> crate::fogofwar::Rec {
    crate::fogofwar::Rec { seen: false, framed: None, t: -1e9, at: None, wand_t: None, body: false}
}

fn chest_of(g: &crate::game::Game, c: CharId) -> V3 { g.ch[c].bone_or(crate::character::Bone::Chest, 1.1) }

/// `men as players`
fn players(g: &crate::game::Game) -> Vec<PlayerId> { crate::seats::men(g) }


/// Once a tick after everybody has moved: the eyes, then seen or not for every man off the team, then the doors.
pub fn fog_of_war_update(g: &mut crate::game::Game, dt: f64) {
    if !fog_of_war_active(g) { fog_goes_off(g); return; }
    g.fog_of_war.loaded = false;
    g.fog_of_war.frame_no += 1.0;
    let eyes = teams_eyes(g);
    let view = picture_eyes(g, &eyes);
    doors_mates(g);
    fired_on(g, dt);
    g.fog_of_war.hidden_chars.clear();
    let men = men_judged(g);
    let lines = lines_of_sight(g, &eyes, &men);
    for &gd in men.iter() { judge(g, gd, &lines, view.as_deref()); }
    wands_look(g, &eyes, &men);
    fog_of_war_tell_calls(g, dt);
    doors_seen(g);
    // (who the team is, and each lamp as the team last saw it: the team's own, which rules read (`ours_on`, `ours_id`) and the picture below)
    fog_of_war_team(g);
    fog_of_war_lamps_known(g);
    crate::fogofwar::picture::fill(g);   // (the renderer's block: filled from the game's state and read by nothing in it)
}

/// The fog has gone off since the pass last ran: what the team knew is dropped, and the picture's block with it.
fn fog_goes_off(g: &mut Game) {
    if !g.fog_of_war.live && !g.fog_of_war.loaded { return; }
    crate::fogofwar::picture::off(g);
    let f = &mut g.fog_of_war;
    f.loaded = false; f.props_loaded = false; f.eyes = Vec::new(); f.hidden_chars.clear(); f.door_recs.clear(); f.doors_held = 0.0; f.thing_recs.clear();
    f.fired_n = JsMap::new(); f.fired_at = JsMap::new(); f.fire_clock = 0.0; f.lamp_recs.clear();
    f.live = false;
    let team = &mut g.engine.lights.fog_team;
    team.on = false;
    team.owners.clear();
    team.lights.clear();
}

fn eye_cap(g: &Game) -> usize { m::min(crate::render_fogofwar::FOG_EYES, fog_eye_count(g.k.fog.eyes) + g.seats.len() as f64 - 1.0) as usize }

/// Every eye of the team's, as many as the cap: the players' (a lens they look through in place of their own), the teammates', then the cameras stuck up.
fn teams_eyes(g: &mut Game) -> Vec<crate::fogofwar::FogEye> {
    let cap = eye_cap(g);
    let mut eyes = Vec::new();
    crate::seats::each_seat(g, &mut |g| {
        if g.seats.len() != 1 && eyes.len() >= cap { return; }
        let own = fog_of_war_eye_of(g, g.pl[g.player].ch);
        let lens = crate::wand::wand_eye(g, &crate::seats::here(g)).or_else(|| crate::peeksight::peek_eye(g, own));
        eyes.push(lens.unwrap_or(own));
    });
    if g.k.fog.own {   // (only the player's own eyes: an ordered lens is a device's picture, as a stuck camera's is)
        crate::seats::each_seat(g, &mut |g| {
            let lenses = crate::wandorder::mate_wand_eyes(g);
            let room = cap.saturating_sub(eyes.len());
            eyes.extend(lenses.into_iter().take(room));
        });
    } else {
        for i in 0..g.guards.len() {
            if eyes.len() >= cap { break; }
            let gd = g.guards[i];
            if sees_for_us(g, gd) { eyes.push(crate::wandorder::mate_wand_eye(g, gd).unwrap_or_else(|| fog_of_war_eye_of(g, g.gd[gd].ch))); }
        }
    }
    crate::seats::each_seat(g, &mut |g| {
        let room = cap.saturating_sub(eyes.len());
        eyes.extend(crate::stickycam::sticky_eyes(g, &crate::seats::here(g)).into_iter().take(room));
    });
    eyes
}
/// One of the squad who has his eyes.
fn sees_for_us(g: &Game, gd: GuardId) -> bool { g.team(gd).ours() && !crate::condition::down(g, g.gd[gd].ch) && !crate::control::man::blind(g, gd) }

/// The picture's eyes: the team's, and a tutorial step's own where it has one. Returns that one: a living man it shows is nobody's to know.
fn picture_eyes(g: &mut Game, eyes: &[crate::fogofwar::FogEye]) -> Option<Vec<crate::fogofwar::FogEye>> {
    let step = crate::rails::rails_step_eye(g);
    let mut pic = eyes.to_vec();
    if let Some(s) = step { pic.truncate(eye_cap(g) - 1); pic.push(s); }
    g.fog_of_war.eyes = pic;
    step.map(|s| vec![s])
}

/// The squad's own men, for the doors' rule.
fn doors_mates(g: &mut Game) {
    let mates = g.guards.iter().filter(|&&gd| sees_for_us(g, gd)).map(|&gd| g.gd[gd].ch).collect();
    g.fog_of_war.mates = mates;
}

/// The men the squad is shooting at: each op of the room fight that has fired since the last tick. (An op's row is its own for good: the table never
/// shrinks, so a later fight's ops are other rows.)
fn fired_on(g: &mut Game, dt: f64) {
    g.fog_of_war.fire_clock += dt;
    let now = g.fog_of_war.fire_clock;
    for &id in &g.fight.ops {
        let o = &g.tab.roomfight_op[id];
        let (n, tgt, gd) = (o.rounds, o.tgt, o.gd);
        let was = g.fog_of_war.fired_n.get(id.0).copied();
        g.fog_of_war.fired_n.set(id.0, n);
        if let (Some(was), Some(t)) = (was, tgt) {
            if n > was && g.team(gd).ours() && !g.team(t).ours() { g.fog_of_war.fired_at.set(t, now); }
        }
    }
}

/// Everybody off the team who is on the floor.
fn men_judged(g: &Game) -> Vec<GuardId> {
    let here = |cv: GuardId| !g.gd[cv].civ.is_some_and(|c| g.cv[c].state == crate::civilians::CivState::Gone);
    g.guards.iter().copied().filter(|&gd| !g.team(gd).ours()).chain(g.civilians.iter().copied().filter(|&cv| !g.team(cv).ours() && here(cv))).collect()
}

/// Who of them an eye has a line to. A man held in an arm is in view of who holds him.
fn lines_of_sight(g: &mut Game, eyes: &[crate::fogofwar::FogEye], men: &[GuardId]) -> JsMap<GuardId, bool> {
    let held: Vec<GuardId> = players(g).into_iter().take(2).filter_map(|p| g.pl[p].holding().map(|h| h.g)).collect();
    let mut lines: JsMap<GuardId, bool> = JsMap::new();
    fog_of_war_hand_eyes(g, eyes);
    let hazy = g.k.fog.smoke > 0.0 && crate::sense::hazy(g);
    for &gd in men {
        let seen = held.contains(&gd) || {
            let down = crate::condition::down(g, g.gd[gd].ch);
            let at = if down { body_at(g, gd) } else { chest_of(g, g.gd[gd].ch) };
            let chest = if down { None } else { Some(at) };
            if hazy { seen_through_air(g, eyes, gd, at, chest) } else { fog_of_war_sees(g, eyes, IN_ORDER, 1.0, gd, chest) >= ONE_EYE }
        };
        lines.set(gd, seen);
    }
    lines
}
/// There is something in the air: he is in view of any eye with a clear line to him that the air lets through, the nearest asked first, and
/// the rest only where that one has too much of it before it.
fn seen_through_air(g: &mut Game, eyes: &[crate::fogofwar::FogEye], gd: GuardId, at: V3, chest: Option<V3>) -> bool {
    let order = nearest_first(eyes, at);
    let mut clear = 0;
    for want in [1.0, eyes.len() as f64] {
        let r = js::to_u32(fog_of_war_sees(g, eyes, order as f64, want, gd, chest));
        let had = clear;
        clear = r >> 28;
        if (had..clear).any(|k| fog_of_war_clear(g, eyes[((r >> (4 * k)) & 15) as usize].pos, at)) { return true; }
        if clear == 0 { return false; }
    }
    false
}
/// The eyes' numbers by how near they are, four bits each, the nearest lowest.
fn nearest_first(eyes: &[crate::fogofwar::FogEye], at: V3) -> i32 {
    let far = |e: &crate::fogofwar::FogEye| m::pow(e.pos[0] - at[0], 2.0) + m::pow(e.pos[2] - at[2], 2.0);
    let mut near: Vec<(f64, i32)> = eyes.iter().enumerate().map(|(i, e)| (far(e), i as i32)).collect();
    js::sort_by(&mut near, |a, b| a.0 - b.0);
    near.iter().enumerate().fold(0, |order, (k, &(_, i))| order | i.wrapping_shl(4 * k as u32))
}

/// When a lens under a door last had each of theirs the team sees.
fn wands_look(g: &mut Game, eyes: &[crate::fogofwar::FogEye], men: &[GuardId]) {
    for eye in eyes.iter().filter(|e| e.wand == Some(true)) {
        for &gd in men {
            let c = g.gd[gd].ch;
            if crate::condition::down(g, c) || !g.team(gd).theirs() || !g.fog_of_war.recs.get(gd).is_some_and(|r| r.seen) { continue; }
            let chest = chest_of(g, c);
            if !fog_of_war_eye_on_man(g, *eye, gd, chest) { continue; }
            let now = g.time;
            g.fog_of_war.recs.get_mut(gd).unwrap().wand_t = Some(now);
        }
    }
}

/// The doors: a leaf where the picture has it needs no look; one that is not comes up to date when an eye has it or its place.
fn doors_seen(g: &mut Game) {
    g.fog_of_war.doors_held = 0.0;
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        let angle = g.dr[d].leaf.angle;
        let shown = g.fog_of_war.door_recs.get_or_insert_with(d, || crate::fogofwar::FogOfWarDoorRecsValue { shown: angle, ghost: None }).shown;
        if m::abs(angle - shown) <= DOOR_EPS || fog_of_war_door_seen(g, d, shown) { g.fog_of_war.door_recs.get_mut(d).unwrap().shown = angle; }
        else { g.fog_of_war.doors_held += 1.0; }
    }
}

/// Who the team is this frame, for the lights' classes and for what the team's own doing shows (`ours_id`): every seat's man and every teammate, and the
/// light each carries. From here the rule is live (`ours_on`).
fn fog_of_war_team(g: &mut crate::game::Game) {
    g.fog_of_war.live = true;
    g.engine.lights.fog_team.on = true;
    g.engine.lights.fog_team.owners.clear();
    g.engine.lights.fog_team.lights.clear();
    for pl in crate::seats::men(g) {
        let id = g.player_id(pl);
        let light = g.pl[pl].light;
        g.engine.lights.fog_team.owners.add(crate::Num(id));
        g.engine.lights.fog_team.lights.add(light);
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.team(gd).ours() {
            let id = g.char_id(gd);
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
        let row = &g.tab.fixture[t];
        if row.light.is_none() && row.area_lights.is_none() { continue; }
        let lit = fog_of_war_lamp_lit(g, t);
        fog_of_war_lamp_known(g, t, lit);
    }
}

/// pass()'s verdict on one man.
fn judge(g: &mut crate::game::Game, gd: GuardId, lines: &JsMap<GuardId, bool>, view: Option<&[crate::fogofwar::FogEye]>) {
    let rt = g.fog_of_war.recs.get_or_insert_with(gd, fresh_rec).t;
    let dead = crate::condition::down(g, g.gd[gd].ch);
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
    let (now, linger, p) = (g.time, g.k.fog.linger, g.pos_of(gd));
    let r = g.fog_of_war.recs.get_mut(gd).unwrap();
    r.framed = Some(framed);
    if dead && (seen || fired || framed || (since >= 0.0 && since < m::max(linger, 0.5))) { r.body = true; }
    r.seen = seen;
    if seen {
        r.t = now;
        if !dead { r.at = Some(p); }
    }
    if !fog_of_war_drawn(g, gd) {
        let c = g.gd[gd].ch;
        g.fog_of_war.hidden_chars.add(c);
    } else if fired && !dead && !(since >= 0.0 && since < linger) {
    }
}

/// A man as an eye: off his head, looking where he looks (`gaze_of`). (FogOfWar.eyeOf)
fn fog_of_war_eye_of(g: &crate::game::Game, c_2: CharId) -> crate::fogofwar::FogEye {
    let pos = g.ch[c_2].bone_or(crate::character::Bone::Head, 1.65);
    let look = gaze_of(g, c_2);
    let p = g.ch[c_2].pos;
    let y = if g.ch[c_2].alive { EYE_STAND + (EYE_CROUCH - EYE_STAND) * m::min(1.0, m::max(0.0, g.ch[c_2].anim.crouch)) } else { EYE_CROUCH };
    crate::fogofwar::FogEye { id: g.ch[c_2].id, pos, look, at: [p[0], p[1] + y, p[2]], fov: None, near: None, wand: None }
}

/// The player a call is worded for: with two, the first one up. (FogOfWar.caller)
fn fog_of_war_caller(g: &crate::game::Game) -> PlayerId {
    if g.seats.len() == 1 { return g.player; }
    match crate::seats::men_up(g).first() {
        Some(&p) => p,
        None => players(g)[0],
    }
}

/// Where `p` lies for the player's man, by the way he looks. (FogOfWar.sideOf)
pub fn fog_of_war_side_of(g: &crate::game::Game, p: V3) -> crate::close::CloseId {
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
fn fog_of_war_call_words(g: &crate::game::Game, kind: crate::fogofwar::FogOfWarCallWordsKind, p: V3) -> String {
    let side = fog_of_war_side_of(g, p);
    let there = crate::dialogue::place_name(g, p);
    let me = fog_of_war_caller(g);
    let here_at = g.player_pos(me);
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
    let f = &mut g.fog_of_war;
    f.call_n += 1.0;
    f.last_call = now;
    f.called_t.set(h, now);
    crate::cues::cues_call(g, gd, &text);
}

/// What the cone hides is called: a contact from a teammate's eyes, else a step in his ears. Under cones or own eyes only (hard). (FogOfWar.tellCalls)
fn fog_of_war_tell_calls(g: &mut crate::game::Game, dt: f64) {
    g.fog_of_war.call_clock += dt;
    let now = g.fog_of_war.call_clock;
    if !(g.k.fog.cone || g.k.fog.own) || g.fog_of_war.eyes.is_empty() || crate::seats::all_down(g) || js::truthy(js::rem(g.fog_of_war.frame_no, CALL_EVERY)) { return; }
    let me = g.fog_of_war.eyes[0];
    let mut foes: Vec<GuardId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        if crate::condition::down(g, g.gd[h].ch) || !g.ch[g.gd[h].ch].alive { continue; }
        if !g.team(h).theirs() { continue; }
        let pl = players(g);
        if pl.iter().any(|&p| g.pl[p].holding().map(|x| x.g) == Some(h)) { continue; }
        if crate::runner::runner_nobody(g, h) { continue; }
        foes.push(h);
    }
    // (whom the player's own eye has, in the picture: nobody need tell him)
    for &h in foes.iter() {
        if !g.fog_of_war.recs.get(h).map_or(false, |r| r.seen) { continue; }
        let chest = chest_of(g, g.gd[h].ch);
        if fog_of_war_eye_on_man(g, me, h, chest) { g.fog_of_war.known_t.set(h, now); }
    }
    if now - g.fog_of_war.last_call < g.k.fog.call_gap { return; }
    let mut mates: Vec<GuardId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if sees_for_us(g, gd) && g.gd[gd].held.is_none() { mates.push(gd); }
    }
    // contact: the first teammate, in list order, with one of them in his eyes
    for &gd in mates.iter() {
        let c = g.gd[gd].ch;
        let e = fog_of_war_eye_of(g, c);
        for &h in foes.iter() {
            if !call_fresh(g, h, now) { continue; }
            let chest = chest_of(g, g.gd[h].ch);
            if !fog_of_war_eye_on_man(g, e, h, chest) { continue; }
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
        if crate::condition::down(g, g.gd[ogd].ch) || g.fog_of_war.told_step.get(ogd).copied() == Some(sb.t) { continue; }
        g.fog_of_war.told_step.set(ogd, sb.t);
        if crate::condition::down(g, g.gd[sb.by].ch) || !call_fresh(g, sb.by, now) || g.fog_of_war.recs.get(sb.by).map_or(false, |r| r.seen) || crate::runner::runner_nobody(g, sb.by) { continue; }
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

/// A fresh encounter: nobody has been seen, no pair asked. (FogOfWar.reset)
pub fn fog_of_war_reset(g: &mut crate::game::Game) {
    crate::fogofwar::picture::reset(g);
    let f = &mut g.fog_of_war;
    f.lamp_recs.clear(); f.door_recs.clear(); f.doors_held = 0.0; f.thing_recs.clear(); f.props_loaded = false; f.loaded = false; f.our_landed.clear(); f.thing_since = -1.0;
    f.recs = JsMap::new(); f.frame_no = 0.0; f.hidden_chars.clear(); f.fired_n = JsMap::new(); f.fired_at = JsMap::new(); f.fire_clock = 0.0;
    f.call_clock = 0.0; f.last_call = -1e9; f.called_t = JsMap::new(); f.known_t = JsMap::new(); f.told_step = JsMap::new(); f.call_n = 0.0;  
}

/// The fog's own line: anything solid between an eye and a point. (FogOfWar.sightBlocked)
pub(super) fn fog_of_war_sight_blocked(g: &crate::game::Game, a: V3, b: V3) -> bool {
    let through = fog_through();
    crate::scene_collision::segment_blocked(g, a, b, Some(0.02), Some(through), None)
}

/// One eye on one living man, walls and leaves only, inside the eye's field. (FogOfWar.lineToMan)
fn fog_of_war_line_to_man(g: &crate::game::Game, e: crate::fogofwar::FogEye, gd: GuardId, chest: V3) -> bool {
    let pc = g.gd[gd].ch;
    if !fog_of_war_in_field(g, e, chest) { return false; }
    let to = v3::sub(chest, e.pos);
    let l = m::hyp2(to[0], to[2]);
    if l <= 1e-3 { return true; }
    if !g.ch[pc].alive { return false; }
    if !fog_of_war_sight_blocked(g, e.pos, chest) { return true; }
    let head = g.ch[pc].bone_or(crate::character::Bone::Head, 1.6);
    !fog_of_war_sight_blocked(g, e.pos, head)
}

/// Does the air between an eye and a point let it through: the smoke, the fog and the dust a man's own eyes are held to, at their bar.
pub fn fog_of_war_clear(g: &mut crate::game::Game, eye: V3, p: V3) -> bool {
    g.k.fog.smoke <= 0.0 || crate::sense::clear_between(g, eye, p) >= g.k.fog.smoke
}

/// One eye of the team's on one living man: a line to him, through air that lets it. (A lesson step's eye asks for the line alone: it is the picture's and not a man's.)
fn fog_of_war_eye_on_man(g: &mut crate::game::Game, e: crate::fogofwar::FogEye, gd: GuardId, chest: V3) -> bool {
    fog_of_war_line_to_man(g, e, gd, chest) && fog_of_war_clear(g, e.pos, chest)
}

/// Where a body is looked for: a hand's height over where it lies.
fn body_at(g: &crate::game::Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    let hp = g.ch[c].bone(crate::character::Bone::Hips).unwrap_or(g.ch[c].pos);
    [hp[0], m::max(hp[1], 0.0) + 0.3, hp[2]]
}

/// The eyes with a line to a man (`chest` given) or his body (None), one question to the module: how many × 2^28 + those eyes, four bits each. (FogOfWar.sees)
fn fog_of_war_sees(g: &crate::game::Game, eyes: &[crate::fogofwar::FogEye], order: f64, want: f64, gd: GuardId, chest: Option<V3>) -> f64 {
    let pc = g.gd[gd].ch;
    let (a, b, kind): (V3, V3, f64) = match chest {
        Some(c) => (c, g.ch[pc].bone_or(crate::character::Bone::Head, 1.6), if g.ch[pc].alive { 3.0 } else { 1.0 }),
        None => {
            let at = body_at(g, gd);
            (at, at, 0.0)
        }
    };
    let through = fog_through();
    crate::scene_collision::eyes_see(g, order, eyes.len() as f64, want, kind, through, a, b)
}

/// The eyes of a pass handed to the module, each with the field `in_field` holds it to. (FogOfWar.handEyes)
fn fog_of_war_hand_eyes(g: &crate::game::Game, eyes: &[crate::fogofwar::FogEye]) {
    crate::scene_collision::sight_open(eyes.len() as f64);
    for (k, e) in eyes.iter().enumerate() {
        let fov = match e.fov { Some(f) => f, None => field_deg(g) };
        let near = match e.near { Some(n) => n, None => field_near(g) };
        let look_len = js::or(m::hyp2(e.look[0], e.look[2]), 1.0);
        crate::scene_collision::sight_eye(k as f64, e.pos, e.look, look_len, m::cos(fov * 0.5 * DEG), near, fov >= 360.0);
    }
}

/// One eye on a body: a clear line to a hand's height over where it lies, inside the eye's field. (FogOfWar.eyeOnBody)
fn fog_of_war_eye_on_body(g: &crate::game::Game, e: crate::fogofwar::FogEye, gd: GuardId) -> bool {
    let at = body_at(g, gd);
    fog_of_war_in_field(g, e, at) && !fog_of_war_sight_blocked(g, e.pos, at)
}

/// Does anybody on the team see this door: a line to the leaf where it stands or where the picture has it. (FogOfWar.doorSeen)
fn fog_of_war_door_seen(g: &crate::game::Game, d: DoorId, shown: f64) -> bool {
    let (hinge, w, closed) = { let def = &g.dr[d].def; (def.hinge, def.width, def.closed_dir) };
    let (hx, hz) = (hinge[0], hinge[1]);
    let eyes = g.fog_of_war.eyes.clone();
    let mates = g.fog_of_war.mates.clone();
    let reach = g.k.fog.squad_doors_m * g.k.fog.squad_doors_m;
    let (fs, ys) = (door_f(), door_y());
    for ai in 0..2 {
        let turn = if ai != 0 { shown } else if g.dr[d].leaf.off { 0.0 } else { g.dr[d].leaf.angle };
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
                    let from = g.ch[c].bone_or(crate::character::Bone::Head, 1.6);
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
pub fn fog_of_war_door_held(g: &crate::game::Game, d: DoorId) -> bool {
    let shown = match g.fog_of_war.door_recs.get(d) { Some(r) => r.shown, None => return false };
    fog_of_war_active(g) && m::abs(g.dr[d].leaf.angle - shown) > DOOR_EPS
}

/// The angle the picture has this door at. (FogOfWar.doorShown)
pub fn fog_of_war_door_shown(g: &crate::game::Game, d: DoorId) -> f64 {
    if let Some(s) = g.fog_of_war.door_recs.get(d).map(|r| r.shown) {
        if fog_of_war_active(g) { return s; }
    }
    g.dr[d].leaf.angle
}

/// The door a box in the picture's list is the remembered leaf of. (FogOfWar.doorOfBox)
pub fn fog_of_war_door_of_box(g: &crate::game::Game, box_: AnyBoxId) -> Option<DoorId> {
    for (d, r) in g.fog_of_war.door_recs.iter() {
        if let Some(gh) = r.ghost.as_ref() {
            if AnyBoxId::World(gh[0]) == box_ { return Some(d); }
        }
    }
    None
}

/// The doors' boxes for the picture: each leaf where it stands, or for one moved unseen, copies at the angle last seen. (FogOfWar.doorBoxes)
pub fn fog_of_war_door_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
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
        let angle = g.dr[d].leaf.angle;
        let shown = match g.fog_of_war.door_recs.get(d).map(|r| r.shown) {
            Some(s) if m::abs(angle - s) > DOOR_EPS => s,
            _ => { out.push(bx_.into()); out.push(h0.into()); out.push(h1.into()); continue; }
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
        let flags = g.tab.boxes[bx_].flags;
        let had_rot = g.tab[gh[0].into()].rot.is_some();
        { let mut q = g.tab.edit(gh[0]); q.c = pose.c; q.yaw = pose.yaw; q.flags = flags; }
        if g.dr[d].leaf.off || had_rot {
            let (h, marks) = (g.dr[d].leaf_h, g.dr[d].leaf_marks.clone());
            { let mut q = g.tab.edit(gh[0]); q.h = h; q.marks = marks; q.rot = None; }
            g.tab.edit(gh[1]).rot = None;
            g.tab.edit(gh[2]).rot = None;
        }
        for s in 0..2 { let mut q = g.tab.edit(gh[1 + s]); q.c = pose.handles[s]; q.yaw = pose.yaw; }
        out.push(gh[0].into()); out.push(gh[1].into()); out.push(gh[2].into());
    }
    g.fog_of_war.door_n = out.len() as f64 - g.fog_of_war.door_from;
}

/// Has the team a look at this fixture: an eye on the floor under it, a line to the fixture itself, or seen floor within two metres that it lights. (FogOfWar.lampSeen)
pub(super) fn fog_of_war_lamp_seen(g: &mut crate::game::Game, t: FixtureId) -> bool {
    let pos = g.tab.fixture[t].pos;
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

    /// A prop's parts share a key, and thing keys sit above 2^24 by system.
    #[test]
    fn is_prop_goes_by_the_system() {
        assert!(is_prop(PROP_SYS * 0x1000000 as f64 + 1234.0));
        assert!(!is_prop((PROP_SYS + 1.0) * 0x1000000 as f64));
        assert!(!is_prop(-1.0));
    }

    /// What the fog remembers as one thing: a prop's or an item's parts together, a broken fixture's body as the fixture and what came off it by itself; the things it leaves alone have no key.
    #[test]
    fn the_fog_remembers_a_thing_by_its_key() {
        let g = Game::blank();
        const SYS: f64 = 0x1000000 as f64;
        for (k, want) in [
            (1.0 * SYS + 5.0, -1.0),                   // a leaf
            (4.0 * SYS + 5.0, -1.0),                   // what a man carries
            (5.0 * SYS + 5.0, -1.0),                   // a wall device
            (10.0 * SYS + 5.0, -1.0),                  // a holed wall's paint
            (7.0 * SYS + 0x4001 as f64, -1.0),         // a moth
            (2.0 * SYS + 35.0, 2.0 * SYS + 2.0),       // a prop's parts share a key
            (3.0 * SYS + 35.0, 3.0 * SYS + 4.0),       // an item's
            (8.0 * SYS + 35.0, 8.0 * SYS + 35.0),      // debris: a thing by itself
            (7.0 * SYS + (0x5000 + (3 << 5) + 2) as f64, 7.0 * SYS + ((0x5000 + (3 << 5) + 2) >> 5) as f64), // a broken fixture's body
            (7.0 * SYS + (0x5000 + (3 << 5) + 6) as f64, 7.0 * SYS + (0x5000 + (3 << 5) + 6) as f64),        // what came off it
            (7.0 * SYS + 0x8000 as f64, 7.0 * SYS + 0x8000 as f64),                                          // a piece cut off
        ] { assert_eq!(key_of_mkey(&g, k), want, "mkey {}", k); }
    }

}
