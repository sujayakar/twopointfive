//! src/game/defuse.ts lines 1 to 286
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Planar distance.   (defuse.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// The player the row is for (one seat today).   (defuse.ts who)
pub fn who(g: &mut crate::game::Game) -> PlayerId { g.player }

/// Seconds as m:ss, rounded up, never below zero.   (defuse.ts mmss)
pub fn mmss(s: f64) -> String {
    let t = m::max(0.0, m::ceil(s));
    format!("{}:{:0>2}", num(m::floor(t / 60.0)), num(js::rem(t, 60.0)))
}

/// A charge's planar normal as a vector.   (defuse.ts n3)
pub fn n3(d: &crate::scene_scenarios::DefuseDevice) -> V3 { [d.n[0], 0.0, d.n[1]] }

/// A fresh Defuse: the field initialisers are the whole of it.   (defuse.ts Defuse.constructor)
pub fn defuse_new(g: &mut crate::game::Game) -> crate::defuse::Defuse { crate::defuse::Defuse::blank() }

/// The block in force (None: not live).   (defuse.ts Defuse.data)
pub fn defuse_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::DefuseData> {
    let has = match g.level.scenarios.as_ref().and_then(|s| s.defuse.as_ref()) {
        Some(d) => !d.devices.is_empty(),
        None => false,
    };
    if !has { return None; }
    if g.touring || g.quiet_utility || g.puppet.is_some() { return None; }
    if !(crate::roomclear::room_clear_scenario_active(g) && !g.room_clear.rooms.is_empty()) { return None; }
    g.level.scenarios.as_ref().and_then(|s| s.defuse.clone())
}

/// Is this pass the running mission's (false between a restart and the first tick after it).   (defuse.ts Defuse.mine)
pub fn defuse_mine(g: &mut crate::game::Game) -> bool { g.defuse.run == Some(g.mission.serial) }

/// (defuse.ts Defuse.active)
pub fn defuse_active(g: &mut crate::game::Game) -> bool { defuse_data(g).is_some() && defuse_mine(g) }

/// (defuse.ts Defuse.started)
pub fn defuse_started(g: &mut crate::game::Game) -> bool { !g.defuse.t0.is_nan() }

/// Seconds a hand must stay on a charge.   (defuse.ts Defuse.workSecs)
pub fn defuse_work_secs(g: &mut crate::game::Game) -> f64 {
    match defuse_data(g).and_then(|d| d.work_secs) {
        Some(s) => s,
        None => g.k.defuse.work_secs,
    }
}

/// Seconds left on the clock (the whole of it until the team moves; where it stopped once it has).   (defuse.ts Defuse.left)
pub fn defuse_left(g: &mut crate::game::Game) -> f64 {
    let d = match defuse_data(g) { Some(d) => d, None => return 0.0 };
    if !defuse_started(g) { return d.secs; }
    d.secs - ((if g.defuse.t_end.is_nan() { g.time } else { g.defuse.t_end }) - g.defuse.t0)
}

/// Has the mission been won: every charge safe and the structure taken.   (defuse.ts Defuse.won)
pub fn defuse_won(g: &mut crate::game::Game) -> bool {
    defuse_active(g) && g.defuse.end == Some(DefuseEnd::Safe) && g.room_clear.end == Some(crate::roomclear::RoomClearEnd::Taken)
}

/// Changes when the list of live charges does.   (defuse.ts Defuse.key)
pub fn defuse_key(g: &mut crate::game::Game) -> String {
    if !defuse_active(g) { return String::new(); }
    let words: Vec<&str> = g.defuse.states.iter().map(|s| s.as_str()).collect();
    words.join(",")
}

/// The charges still live: index, the point a hand goes to, the planar normal toward the man, the menu's name.   (defuse.ts Defuse.armed)
pub fn defuse_armed(g: &mut crate::game::Game) -> Vec<crate::defuse::DefuseArmedItem> {
    let d = match defuse_data(g) { Some(d) => d, None => return Vec::new() };
    if !defuse_mine(g) || g.defuse.end.is_some() { return Vec::new(); }
    let mut out = Vec::new();
    for (i, dev) in d.devices.iter().enumerate() {
        if g.defuse.states.get(i) == Some(&ChargeState::Armed) {
            out.push(DefuseArmedItem { i: i as f64, face: [dev.pos[0], dev.pos[1], dev.pos[2]], n: n3(dev), name: dev.name.clone() });
        }
    }
    out
}

/// Why charge i is no longer there to be worked (None: it is): the word the man says.   (defuse.ts Defuse.moot)
pub fn defuse_moot(g: &mut crate::game::Game, i: f64) -> Option<String> {
    let s = if defuse_active(g) { state_at(g, i) } else { None };
    match s {
        Some(ChargeState::Armed) => None,
        Some(ChargeState::Safe) => Some(String::from("it's already safe.")),
        _ => Some(String::from("it's gone.")),
    }
}

/// Once a frame, after the mission's own tick.   (defuse.ts Defuse.tick)
pub fn defuse_tick(g: &mut crate::game::Game, _dt: f64) {
    let d = match defuse_data(g) {
        Some(d) => d,
        None => { g.defuse.sam = None; return; }
    };
    let pl = who(g);
    let p = g.ch[g.pl[pl].ch].pos;
    if !defuse_mine(g) {   // a fresh run
        g.defuse.run = Some(g.mission.serial);
        let n = d.devices.len();
        g.defuse.states = vec![ChargeState::Armed; n];
        g.defuse.safe_by = vec![None; n];
        g.defuse.end = None;
        g.defuse.t0 = f64::NAN;
        g.defuse.t_end = f64::NAN;
        g.defuse.sam = None;
        g.defuse.offered = -1.0;
        g.defuse.beat = -1.0;
        g.defuse.took_said = false;
        g.defuse.won_said = false;
        g.defuse.log.clear();
        g.defuse.from = Some([p[0], 0.0, p[2]]);
        g.defuse.last_order = g.orders.log.last().map(|r| r.serial);
        defuse_note(g, &format!("armed: {} charges, {} s", n, num(d.secs)));
    }
    defuse_draw(g, &d);
    let taken = g.room_clear.end == Some(crate::roomclear::RoomClearEnd::Taken);
    if g.defuse.end == Some(DefuseEnd::Safe) && !g.defuse.won_said && taken {
        g.defuse.won_said = true;
        g.stats.defuse.won += 1;
        defuse_note(g, "won");
    }
    if g.defuse.end.is_some() { g.defuse.sam = None; return; }
    if g.pl[pl].down || g.room_clear.end == Some(crate::roomclear::RoomClearEnd::Wiped) {
        g.defuse.end = Some(DefuseEnd::Over);
        g.defuse.t_end = g.time;
        g.defuse.sam = None;
        defuse_note(g, "end: over — the run ended first");
        return;
    }
    if !g.defuse.took_said && taken {
        g.defuse.took_said = true;
        let text = if defuse_started(g) {
            format!("the house is taken, but the charges are still live with {} left", mmss(defuse_left(g)))
        } else {
            String::from("the house is taken, but the charges are still live")
        };
        crate::game::game_msg(g, &text);
    }
    if !defuse_started(g) {   // the clock waits for the team's first move
        let moved = match g.defuse.from { Some(from) => flat(p, from) > g.k.defuse.start_m, None => false };
        let told = match g.orders.log.last() { Some(r) => Some(r.serial) != g.defuse.last_order, None => false };
        if moved || told {
            g.defuse.t0 = g.time;
            g.stats.defuse.started += 1;
            defuse_note(g, &format!("clock: {}", if told { "an order" } else { "the player moved" }));
            let text = format!("the clock is running, and you have {} for {}", mmss(d.secs), if d.devices.len() == 2 { "both charges" } else { "every charge" });
            crate::game::game_msg(g, &text);
        }
    }
    defuse_sam_tick(g, &d);
    if !defuse_started(g) { return; }
    let left = defuse_left(g);
    if left <= 0.0 { defuse_blow(g, &d); return; }
    if left <= g.k.defuse.warn_secs {
        let b = m::ceil(left);
        if b != g.defuse.beat {
            g.defuse.beat = b;
            seam_audio_play_rate(g, "click", None, 0.7, 1.6);
        }
    }
}

/// A hand has been on charge i long enough (the player's own row; a teammate's work order).   (defuse.ts Defuse.makeSafe)
pub fn defuse_make_safe(g: &mut crate::game::Game, i: f64, by: crate::defuse::DefuseMakeSafeBy) {
    let d = match defuse_data(g) { Some(d) => d, None => return };
    if !defuse_mine(g) || state_at(g, i) != Some(ChargeState::Armed) || g.defuse.end.is_some() { return; }
    let at = i as usize;
    let dev = &d.devices[at];
    let name = match by {
        DefuseMakeSafeBy::Player => String::from("Player"),
        DefuseMakeSafeBy::Guard(gd) => g.gd[gd].callsign.clone(),
    };
    g.defuse.states[at] = ChargeState::Safe;
    g.defuse.safe_by[at] = Some(name.clone());
    g.stats.defuse.safe += 1;
    if let DefuseMakeSafeBy::Player = by { g.stats.defuse.by_sam += 1; }
    crate::audio_iface::game_audio_like_play(g, "click", Some(dev.pos), Some(0.9), None);
    let left = defuse_left(g);
    defuse_note(g, &format!("safe: {} by {} with {} s left", dev.id, name, fixed1(left)));
    let live = g.defuse.states.iter().filter(|s| **s == ChargeState::Armed).count();
    if live > 0 {
        let togo = if live == 1 { String::from("one") } else { format!("{}", live) };
        let text = format!("{} is safe, with {} to go and {} left", dev.name, togo, mmss(defuse_left(g)));
        crate::game::game_msg(g, &text);
        return;
    }
    g.defuse.end = Some(DefuseEnd::Safe);
    g.defuse.t_end = g.time;
    g.defuse.sam = None;
    let text = if g.room_clear.end == Some(crate::roomclear::RoomClearEnd::Taken) {
        format!("{} is safe. every charge is safe and the house is yours", dev.name)
    } else {
        format!("{} is safe. every charge is safe with {} left. now take the house", dev.name, mmss(defuse_left(g)))
    };
    crate::game::game_msg(g, &text);
    defuse_note(g, "end: safe");
}

/// The clock at zero: every charge still live goes off, the game's own blast, in its room.   (defuse.ts Defuse.blow)
pub fn defuse_blow(g: &mut crate::game::Game, d: &crate::scene_scenarios::DefuseData) {
    let b = d.blast.unwrap_or_default();
    let mut gone: Vec<String> = Vec::new();
    g.defuse.end = Some(DefuseEnd::Failed);
    g.defuse.t_end = g.time;
    g.defuse.sam = None;
    g.stats.defuse.failed += 1;
    for (i, dev) in d.devices.iter().enumerate() {
        if g.defuse.states.get(i) != Some(&ChargeState::Armed) { continue; }
        g.defuse.states[i] = ChargeState::Blown;
        g.stats.defuse.blown += 1;
        gone.push(dev.name.clone());
        let spec = crate::frag::BlastSpec {
            who: 0.0,
            name: Some(String::from("charge")),
            serial: Some((0x5afe + i) as f64),
            lethal_m: Some(b.lethal_m.unwrap_or(g.k.defuse.lethal_m)),
            wound_m: Some(b.wound_m.unwrap_or(g.k.defuse.wound_m)),
            destr: Some(crate::frag::BlastSpecDestr {
                radius: Some(b.radius.unwrap_or(g.k.defuse.radius)),
                power: Some(b.power.unwrap_or(g.k.defuse.power)),
            }),
            ..Default::default()
        };
        let r = crate::frag::blast(g, [dev.pos[0], dev.pos[1], dev.pos[2]], &spec);
        let player = match r.player { Some(x) => format!(", the player {}", x.as_str()), None => String::new() };
        defuse_note(g, &format!("blown: {} — {} killed, {} wounded{}", dev.id, num(r.killed), num(r.wounded), player));
    }
    let text = format!("{} went off — mission failed. press Shift+Enter to restart", gone.join(" and "));
    crate::game::game_msg(g, &text);
    defuse_note(g, "end: failed");
}

/// A pack with a lamp on it: red and blinking faster as the clock runs down while live, steady green once safe; nothing once it has gone off.   (defuse.ts Defuse.draw)
pub fn defuse_draw(g: &mut crate::game::Game, d: &crate::scene_scenarios::DefuseData) {
    use crate::scene_boxes::{box_flag, MakeBoxP};
    let left = defuse_left(g);
    for (i, dev) in d.devices.iter().enumerate() {
        let s = g.defuse.states.get(i).copied();
        if s == Some(ChargeState::Blown) { continue; }
        let yaw = m::atan2(dev.n[0], dev.n[1]);
        let p = dev.pos;
        let cs = m::cos(yaw);
        let sn = m::sin(yaw);
        let flags = box_flag();
        push_fx(g, MakeBoxP {
            c: pack_at(p, cs, sn, 0.0, -0.02),
            h: [0.17, 0.06, 0.11],
            yaw: Some(yaw),
            albedo: Some([0.2, 0.17, 0.11]),
            flags: Some(flags.dynamic),
            ..Default::default()
        });
        push_fx(g, MakeBoxP {
            c: pack_at(p, cs, sn, -0.09, 0.055),
            h: [0.045, 0.02, 0.07],
            yaw: Some(yaw),
            albedo: Some([0.07, 0.07, 0.07]),
            stuff: Some(crate::scene_stuff::BoxStuff::Metal),
            flags: Some(flags.dynamic),
            ..Default::default()
        });
        let rate = if !defuse_started(g) { 1.0 } else if left > 30.0 { 2.0 } else if left > 10.0 { 4.0 } else { 8.0 };
        let safe = s == Some(ChargeState::Safe);
        let lit = safe || js::rem(m::floor(g.time * rate), 2.0) == 0.0;
        if !lit { continue; }
        push_fx(g, MakeBoxP {
            c: pack_at(p, cs, sn, 0.07, 0.05),
            h: [0.014, 0.014, 0.014],
            yaw: Some(yaw),
            albedo: Some(if safe { [0.1, 0.9, 0.2] } else { [0.9, 0.12, 0.08] }),
            emissive: Some(if safe { [0.25, 2.2, 0.4] } else { [2.4, 0.2, 0.1] }),
            flags: Some((js::to_i32(flags.dynamic) | js::to_i32(flags.no_shadow)) as f64),
            ..Default::default()
        });
    }
}

/// player.ts buildInteractables: each live charge near him is a thing with one row, 'make it safe'.   (defuse.ts Defuse.samRows)
pub fn defuse_sam_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::defuse::DefuseSamRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    g.defuse.offered = -1.0;
    g.defuse.marker = None;
    let d = match defuse_data(g) { Some(d) => d, None => return };
    if !defuse_mine(g) || g.defuse.end.is_some() { return; }
    let pl = who(g);
    let mut best = f64::INFINITY;
    for (i, dev) in d.devices.iter().enumerate() {
        if g.defuse.states.get(i) != Some(&ChargeState::Armed) { continue; }
        let dist = flat(p, dev.pos);
        if dist > g.k.defuse.marker_m { continue; }
        let front = (p[0] - dev.pos[0]) * dev.n[0] + (p[2] - dev.pos[2]) * dev.n[1] > 0.0;
        let near = dist < g.k.defuse.reach && front
            && !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [dev.pos[0] + dev.n[0] * 0.2, 1.0, dev.pos[2] + dev.n[1] * 0.2], None);
        let free = !g.pl[pl].sprinting && g.pl[pl].dragging.is_none() && g.pl[pl].holding.is_none() && g.pl[pl].takedown.is_none();
        let reach = near && free;
        let mut it = crate::game::Interactable {
            kind: crate::game::InteractableKind::Objective,
            pos: [dev.pos[0], dev.pos[1] + 0.1, dev.pos[2]],
            line1: format!("{} · {}", dev.name, mmss(defuse_left(g))),
            line2: String::from(if reach { "" } else if near { "hands full" } else { "step closer" }),
            in_reach: reach,
            off: false,
            ..Default::default()
        };
        if g.defuse.sam.map_or(false, |s| s.dev == i as f64) {   // (the running row is runningRow's)
            it.in_reach = true;
            g.defuse.marker = Some(it.clone());
            offer(g, it, Vec::new());
            continue;
        }
        let mut verbs = Vec::new();
        if reach && !busy && g.defuse.sam.is_none() {
            let hints = format!("{} s — stay on it", num(defuse_work_secs(g)));
            verbs.push(row(g, it.clone(), crate::game::Verb::Defuse, String::from("make it safe"), Some(hints), None));
            if dist < best { best = dist; g.defuse.offered = i as f64; }
        }
        offer(g, it, verbs);
    }
}

/// The row performed starts it; he stays on it workSecs; a step, a sprint, full hands or another verb lets go and the work starts over.   (defuse.ts Defuse.samTick)
pub fn defuse_sam_tick(g: &mut crate::game::Game, d: &crate::scene_scenarios::DefuseData) {
    let pl = who(g);
    let c = g.pl[pl].ch;
    let p = g.ch[c].pos;
    let (acting, mine) = match &g.act {
        Some(a) => (true, a.kind == crate::game::InteractableKind::Objective && a.verb == Some(crate::game::Verb::Defuse)),
        None => (false, false),
    };
    let offered = g.defuse.offered;
    if mine && g.defuse.sam.is_none() && offered >= 0.0 && state_at(g, offered) == Some(ChargeState::Armed) {
        let dev = &d.devices[offered as usize];
        g.defuse.sam = Some(DefuseSam { dev: offered, t0: g.time, at: [p[0], 0.0, p[2]] });
        crate::audio_iface::game_audio_like_play(g, "click", Some(dev.pos), Some(0.8), None);
        defuse_note(g, &format!("the player on {}", dev.id));
        return;
    }
    let s = match g.defuse.sam { Some(s) => s, None => return };
    if state_at(g, s.dev) != Some(ChargeState::Armed) { g.defuse.sam = None; return; }
    if g.pl[pl].down || g.pl[pl].sprinting || g.pl[pl].dragging.is_some() || g.pl[pl].holding.is_some() || g.pl[pl].takedown.is_some()
        || flat(p, s.at) > g.k.defuse.shift || (acting && !mine) {
        defuse_cancel(g);
        return;
    }
    let dev = &d.devices[s.dev as usize];
    let need = defuse_work_secs(g);
    let t = g.time - s.t0;
    let more = crate::handwork::WorkAtMore {
        hand: if js::truthy(crate::handwork::sam_work_hand(g)) { None } else { Some(0.0) },
        ..Default::default()
    };
    crate::handwork::work_at(g, c, crate::character::WorkKind::Take, [dev.pos[0], dev.pos[1], dev.pos[2]], n3(dev), t, need, Some(&more));
    if g.time - s.t0 < need { return; }
    g.defuse.sam = None;
    defuse_make_safe(g, s.dev, DefuseMakeSafeBy::Player);
}

/// Is the player at a charge right now: a sustained verb.   (defuse.ts Defuse.working)
pub fn defuse_working(g: &mut crate::game::Game) -> bool { g.defuse.sam.is_some() && defuse_active(g) }

/// The charge's face he is planted at and squared to while he works it (`at`: the spot he must not shift from).   (defuse.ts Defuse.samJob)
pub fn defuse_sam_job(g: &mut crate::game::Game) -> Option<crate::defuse::DefuseSamJob> {
    let s = g.defuse.sam;
    let d = defuse_data(g);
    let (s, d) = match (s, d) { (Some(s), Some(d)) => (s, d), _ => return None };
    if !defuse_mine(g) { return None; }
    let dev = &d.devices[s.dev as usize];
    Some(DefuseSamJob { p: [dev.pos[0], dev.pos[1], dev.pos[2]], n: n3(dev), at: s.at })
}

/// Space again, Esc or a step: let go, the work starts over (true: there was one to let go of).   (defuse.ts Defuse.cancel)
pub fn defuse_cancel(g: &mut crate::game::Game) -> bool {
    if g.defuse.sam.is_none() { return false; }
    g.defuse.sam = None;
    crate::game::game_msg(g, "you let go, so the charge is still live");
    true
}

/// player.ts buildInteractables: the one row while he is at it.   (defuse.ts Defuse.runningRow)
pub fn defuse_running_row(g: &mut crate::game::Game, running: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, f64) -> crate::game::Interactable) -> crate::game::Interactable {
    let s = g.defuse.sam.unwrap();
    let d = defuse_data(g).unwrap();
    let dev = &d.devices[s.dev as usize];
    let k = (g.time - s.t0) / defuse_work_secs(g);
    let it = match g.defuse.marker.clone() {
        Some(it) => it,
        None => crate::game::Interactable {
            kind: crate::game::InteractableKind::Objective,
            pos: [dev.pos[0], dev.pos[1] + 0.1, dev.pos[2]],
            line1: dev.name.clone(),
            line2: String::new(),
            in_reach: true,
            off: false,
            ..Default::default()
        },
    };
    let text = format!("making it safe… {}% — stay on it (Space or a step lets go)", num(m::min(99.0, m::round(k * 100.0))));
    running(g, it, crate::game::Verb::Defuse, text, k)
}

/// The objective while a charge is live or one has gone off (None: the room clear's own).   (defuse.ts Defuse.objective)
pub fn defuse_objective(g: &mut crate::game::Game) -> Option<crate::defuse::DefuseObjective> {
    let d = defuse_data(g)?;
    if !defuse_mine(g) { return None; }
    if g.defuse.end == Some(DefuseEnd::Safe) || g.defuse.end == Some(DefuseEnd::Over) { return None; }
    let pl = who(g);
    if g.pl[pl].down || g.room_clear.end == Some(crate::roomclear::RoomClearEnd::Wiped) { return None; }   // (the run is over another way: the room clear's own word)
    if g.defuse.end == Some(DefuseEnd::Failed) {
        let mut names: Vec<&str> = Vec::new();
        for (i, dev) in d.devices.iter().enumerate() {
            if g.defuse.states.get(i) == Some(&ChargeState::Blown) { names.push(&dev.name); }
        }
        return Some(DefuseObjective { text: format!("objective ▸ FAILED — {} went off · press shift+enter to restart", names.join(" and ")), pos: None });
    }
    let mut live: Vec<(usize, Option<String>)> = Vec::new();
    for i in 0..d.devices.len() {
        if g.defuse.states.get(i) == Some(&ChargeState::Armed) {
            let by = defuse_on_it(g, i as f64);
            live.push((i, by));
        }
    }
    let p = g.ch[g.pl[pl].ch].pos;
    if live.is_empty() { return None; }   // (every charge settled and the end not yet written: the tick writes it this frame)
    // (the HUD's marker carries ONE place: the live charge nobody is at or on his way to, else the nearest)
    let open: Vec<usize> = live.iter().filter(|x| x.1.is_none()).map(|x| x.0).collect();
    let every: Vec<usize> = live.iter().map(|x| x.0).collect();
    let pool = if !open.is_empty() { &open } else { &every };
    let mut mark = pool[0];
    for &b in &pool[1..] {
        if flat(p, d.devices[b].pos) < flat(p, d.devices[mark].pos) { mark = b; }
    }
    let mark = &d.devices[mark];
    let mut safe: Vec<String> = Vec::new();
    for (i, dev) in d.devices.iter().enumerate() {
        if g.defuse.states.get(i) == Some(&ChargeState::Safe) { safe.push(format!("{} is safe", dev.name)); }
    }
    let clock = if defuse_started(g) {
        format!("{} on the clock", mmss(defuse_left(g)))
    } else {
        format!("{} on the clock, and it starts when you move", mmss(d.secs))
    };
    let any_by = live.iter().any(|x| x.1.is_some());
    let what = if any_by {
        let rows: Vec<String> = live.iter().map(|x| format!("{} · {}", d.devices[x.0].name, x.1.as_deref().unwrap_or("nobody on it"))).collect();
        rows.join(" · ")
    } else if safe.is_empty() && live.len() > 1 {
        let many = if live.len() == 2 { String::from("both") } else { format!("all {}", live.len()) };
        format!("make {} charges safe", many)
    } else {
        let names: Vec<&str> = live.iter().map(|x| d.devices[x.0].name.as_str()).collect();
        format!("make {} safe", names.join(" and "))
    };
    let how = if !safe.is_empty() || any_by {
        String::new()
    } else {
        format!(" · press Space at one{}", if crate::orders::squad_orders_active(g) { ", or X on it" } else { "" })
    };
    let done = if !safe.is_empty() { format!(" · {}", safe.join(" · ")) } else { String::new() };
    Some(DefuseObjective {
        text: format!("objective ▸ {} · {}{}{}", clock, what, done, how),
        pos: Some([mark.pos[0], mark.pos[1], mark.pos[2]]),
    })
}

/// Who is at charge i or on his way to it by an order, as the row says it (None: nobody).   (defuse.ts Defuse.onIt)
pub fn defuse_on_it(g: &mut crate::game::Game, i: f64) -> Option<String> {
    if g.defuse.sam.map_or(false, |s| s.dev == i) { return Some(String::from("you on it")); }
    let w = crate::workthings::work_things_on_device(g, i)?;
    Some(format!("{} {}", g.gd[w.gd].callsign, if w.on { "on it" } else { "going" }))
}

/// (defuse.ts Defuse.note)
pub fn defuse_note(g: &mut crate::game::Game, what: &str) {
    let t = g.time;
    g.defuse.log.push(DefuseLogItem { t, what: String::from(what) });
    if g.defuse.log.len() > 200 { g.defuse.log.drain(0..50); }
}

/// player.ts buildInteractables.   (defuse.ts defuseRows)
pub fn defuse_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::defuse::DefuseRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    // (samRows, the only caller of this `row`, never passes `over`: the skeleton's two same-shaped patch types need no conversion)
    let mut sam_row = |g: &mut Game, base: crate::game::Interactable, verb: crate::game::Verb, text: String, hints: Option<String>, _over: Option<DefuseSamRowsRowOver>| {
        row(g, base, verb, text, hints, None)
    };
    defuse_sam_rows(g, p, busy, &mut sam_row, offer);
}

// ---------------------------------------------------------------- private helpers

/// `this.states[i]` with a number for the index: anything not an index is undefined.
fn state_at(g: &mut Game, i: f64) -> Option<ChargeState> {
    if i >= 0.0 && i == m::floor(i) { g.defuse.states.get(i as usize).copied() } else { None }
}

/// (draw's `at`) a part's place along the pack's own long side: a box's local +X is (cos, 0, -sin).
fn pack_at(p: V3, cs: f64, sn: f64, lx: f64, y: f64) -> V3 { [p[0] + lx * cs, p[1] + y, p[2] - lx * sn] }

/// `g.fx.push({ box: makeBox(…), ttl: 1e-3 })`: one frame's drawn box.
fn push_fx(g: &mut Game, p: crate::scene_boxes::MakeBoxP) {
    let b = crate::scene_boxes::make_box(g, &p);
    g.fx.push(crate::game::GameFxItem { box_: b, ttl: 1e-3 });
}

/// A number in text, as the language writes one, for this file's numbers (seconds, counts, a percentage: never an exponent's range).
fn num(x: f64) -> String {
    if x == 0.0 { String::from("0") } else { format!("{}", x) }
}

/// `x.toFixed(1)`: the nearest tenth, an exact tie going away from zero (Rust's own formatting takes a tie to the even digit), and -0 as "0.0".
fn fixed1(x: f64) -> String {
    if x == 0.0 { return String::from("0.0"); }
    let a = m::abs(x);
    let q = a * 4.0;
    let tie = q == m::floor(q) && js::rem(q, 2.0) == 1.0;   // (only a quarter or three quarters is a tie that a double can hold)
    let body = format!("{:.1}", if tie { a + 0.05 } else { a });
    if x < 0.0 { format!("-{}", body) } else { body }
}

// ---------------------------------------------------------------- seams: what skeleton 3 still cannot do

/// defuse.ts:135 `g.audio.play(name, at, vol, { rate })`.
fn seam_audio_play_rate(g: &mut Game, name: &str, at: Option<V3>, vol: f64, rate: f64) { todo!("needs: game_audio_like_play's opts is JsMap<String, Opaque>, which cannot hold the number `rate`") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mmss_as_the_typescript_writes_it() {
        for (s, want) in [(0.0, "0:00"), (-0.5, "0:00"), (-3.0, "0:00"), (0.2, "0:01"), (59.2, "1:00"), (60.0, "1:00"), (61.0, "1:01"), (600.1, "10:01"), (90.0, "1:30")] {
            assert_eq!(mmss(s), want, "{}", s);
        }
    }

    #[test]
    fn to_fixed_as_the_language_has_it() {
        for (x, want) in FIXED {
            assert_eq!(fixed1(*x), *want, "{}", x);
        }
    }

    #[test]
    fn a_number_in_text() {
        for (x, want) in [(0.0, "0"), (-0.0, "0"), (90.0, "90"), (6.5, "6.5"), (0.1 + 0.2, "0.30000000000000004")] {
            assert_eq!(num(x), want);
        }
    }

    #[test]
    fn a_run_is_the_mission_it_took_by_serial() {
        let mut g = crate::game::Game::blank();
        assert!(!defuse_mine(&mut g), "a fresh Defuse belongs to no mission, even one whose serial is the blank 0");
        g.defuse.run = Some(g.mission.serial);
        assert!(defuse_mine(&mut g));
        g.mission.serial = g.serial();
        assert!(!defuse_mine(&mut g), "a restart deals a new serial");
    }

    #[test]
    fn the_log_is_cut_from_the_front_past_200() {
        let mut g = crate::game::Game::blank();
        for i in 0..201 { defuse_note(&mut g, &format!("{}", i)); }
        assert_eq!(g.defuse.log.len(), 151);
        assert_eq!(g.defuse.log[0].what, "50");
    }

    const FIXED: &[(f64, &str)] = &[(0.0, "0.0"), (-0.0, "0.0"), (0.25, "0.3"), (0.75, "0.8"), (1.25, "1.3"), (-0.25, "-0.3"), (-0.04, "-0.0"), (0.05, "0.1"), (0.15, "0.1"), (12.34, "12.3"), (2.5, "2.5"), (59.95, "60.0"), (99.96, "100.0"), (3.75, "3.8"), (-1.75, "-1.8"), (0.1, "0.1"), (0.001, "0.0")];
}
