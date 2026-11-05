//! src/game/handwork.ts lines 1 to 254
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- seams: what skeleton 3 cannot give yet

/// `g.audio.play(name, at, vol, { rate })` (handwork.ts:105; game_audio_like_play's opts are a map of `Opaque`, which cannot hold a rate)
fn seam_audio_play(g: &mut Game, name: &str, at: V3, vol: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate") }
/// A static box's `c`, `h`, `yaw` and whether it has a `rot` (handwork.ts:125; BoxId has no table: ids.rs "no table here", properties.txt `g.none[id]`)
fn seam_box_read(g: &mut Game, b: BoxId) -> (V3, V3, f64, bool) { todo!("needs: the table a Box lives in, for c, h, yaw, rot by BoxId") }

// ---------------------------------------------------------------- private helpers

/// `BoxFlag.a | BoxFlag.b`: the skeleton's flags are numbers.
fn flag_or(a: f64, b: f64) -> f64 { (js::to_i32(a) | js::to_i32(b)) as f64 }
/// `c.id & 0xff`: a box's owner off the man's id.
fn owner_of(g: &Game, c: CharId) -> f64 { (js::to_i32(g.ch[c].id) & 0xff) as f64 }
/// The rule's word as the animator's `swapHeld` takes it.
fn rule_of(held: SwapFeedHeld) -> crate::character::AnimSwapHeldRule {
    match held { SwapFeedHeld::Torch => crate::character::AnimSwapHeldRule::Torch, SwapFeedHeld::Weapon => crate::character::AnimSwapHeldRule::Weapon }
}

// ---------------------------------------------------------------- the switch

/// Is this half of the switch on? Settled on the page: always. (handwork.ts live, handworkOn)
pub fn live(g: &mut crate::game::Game, half: crate::handwork::LiveHalf) -> bool { true }

// ---------------------------------------------------------------- the work pose

/// Feed one man's work pose for this frame. (handwork.ts workAt)
pub fn work_at(g: &mut crate::game::Game, c_2: CharId, kind: crate::character::WorkKind, p: V3, n: V3, t: f64, dur: f64, more: Option<&crate::handwork::WorkAtMore>) {
    let (keys, dir, hand) = match more { Some(x) => (x.keys.clone(), x.dir, x.hand), None => (None, None, None) };
    g.ch[c_2].anim.work = Some(crate::character::WorkPose { kind, p, n, t, dur, keys, dir, hand });
    g.stats.handwork.work += 1;
}

/// A level normal out of a thing's face toward a man: the axis nearest the line from the spot to him. (handwork.ts faceToward)
pub fn face_toward(spot: V3, from: V3) -> V3 {
    let dx = from[0] - spot[0];
    let dz = from[2] - spot[2];
    if m::abs(dx) > m::abs(dz) { [js::or(m::sign(dx), 1.0), 0.0, 0.0] } else { [0.0, 0.0, js::or(m::sign(dz), 1.0)] }
}

/// His right as he faces a face whose normal (toward him) is n. (handwork.ts rightOf)
pub fn right_of(n: V3) -> V3 { [n[2], 0.0, -n[0]] }

/// Walk a man the step in to arm's length of the thing; returns the way he is to face. Position only: the caller settles him. (handwork.ts workStand)
pub fn work_stand(g: &mut crate::game::Game, c_2: CharId, p: V3, n: V3, dt: f64, hand: Option<f64>, pace: Option<f64>) -> f64 {
    let hand = hand.unwrap_or(1.0);
    let pace = pace.unwrap_or(g.k.handwork.step_pace);
    let (stand_side, stand_m, step_rate) = (g.k.handwork.stand_side, g.k.handwork.stand_m, g.k.handwork.step_rate);
    let r = right_of(n);
    let s = if js::truthy(hand) { -stand_side } else { stand_side };
    let tx = p[0] + n[0] * stand_m + r[0] * s;
    let tz = p[2] + n[2] * stand_m + r[2] * s;
    let pos = g.ch[c_2].pos;
    let d = m::hyp2(tx - pos[0], tz - pos[2]);
    let k = if d > 1e-4 { m::min(1.0 - m::exp(-step_rate * dt), pace * dt / d) } else { 0.0 };
    if d < 1.8 {
        g.ch[c_2].pos[0] += (tx - pos[0]) * k;
        g.ch[c_2].pos[2] += (tz - pos[2]) * k;
    }
    m::atan2(-n[0], -n[2])
}

/// Has his hand got to the thing? handWait counts the wait and gives up at waitSecs. (handwork.ts handThere)
pub fn hand_there(g: &mut crate::game::Game, pl: PlayerId, dt: f64, p: V3, n: V3) -> bool {
    let c = g.pl[pl].ch;
    if crate::character::anim_work_info(g, c).w > 0.85 { return true; }
    let pos = g.ch[c].pos;
    let off = (pos[0] - p[0]) * n[0] + (pos[2] - p[2]) * n[2];
    g.pl[pl].hand_wait += if off > g.k.handwork.stand_m + 0.15 { dt * 0.35 } else { dt };
    g.pl[pl].hand_wait > g.k.handwork.wait_secs
}

/// Which of Sam's hands works a thing: the left while the right is full, else the right. (handwork.ts samWorkHand)
pub fn sam_work_hand(g: &mut crate::game::Game) -> f64 {
    let pl = g.player;
    let full = crate::game::player_pistol_up(g, pl) || crate::game::player_rifle_up(g, pl) || { let c = g.pl[pl].ch; g.ch[c].anim.grip[1] };
    if full { 0.0 } else { 1.0 }
}

/// As workAt is told it: the off hand named where his own is full. (handwork.ts samHand)
pub fn sam_hand(g: &mut crate::game::Game) -> Option<f64> {
    if js::truthy(sam_work_hand(g)) { None } else { Some(0.0) }
}

// ---------------------------------------------------------------- the keypad

/// The four keys a code presses; a code of another length is gone through in turn. (handwork.ts padKeys)
pub fn pad_keys(code: &str) -> Vec<[f64; 2]> {
    let table = key_at();
    let five = *table.get(String::from("5")).unwrap();
    let ks: Vec<[f64; 2]> = code.chars().map(|ch| table.get(String::from(ch)).copied().unwrap_or(five)).collect();
    if ks.is_empty() { return vec![five]; }
    (0..4).map(|k| ks[k % ks.len()]).collect()
}

/// Once a frame while the code goes in: his finger on the pad; returns the seconds the code's clock may run; each key's beep and light on its press. (handwork.ts padHands)
pub fn pad_hands(g: &mut crate::game::Game, pad: &crate::handwork::PadHandsPad, dt: f64, secs: f64) -> f64 {
    let pl = g.player;
    let keys = pad_keys(&pad.def.code);
    let (c, code_t) = (g.pl[pl].ch, g.pl[pl].code_t);
    let hand = sam_hand(g);
    let more = WorkAtMore { keys: Some(keys.clone()), hand, dir: None };
    work_at(g, c, crate::character::WorkKind::Keys, pad.def.pos, pad.n, code_t, secs, Some(&more));
    if g.pl[pl].code_t <= 0.0 && !hand_there(g, pl, dt, pad.def.pos, pad.n) { return 0.0; }
    let before = g.pl[pl].code_t;
    let after = before + dt;
    let r = right_of(pad.n);
    for k in 0..4 {
        let at = crate::character::work_key_beat(k as f64, secs);
        if before < at && after >= at {
            seam_audio_play(g, "padBeep", pad.def.pos, 0.5, 1.0 + 0.03 * k as f64);
            let [x, y] = keys[k];
            let (pos, n) = (pad.def.pos, pad.n);
            let spot: V3 = [pos[0] + n[0] * 0.03 + r[0] * x, pos[1] + y, pos[2] + n[2] * 0.03 + r[2] * x];
            let bf = crate::scene_boxes::box_flag();
            let box_ = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
                c: spot,
                h: [0.008, 0.007, 0.002],
                yaw: Some(pad.def.yaw),
                albedo: Some([0.2, 0.16, 0.05]),
                emissive: Some([2.4, 1.5, 0.3]),
                flags: Some(flag_or(bf.dynamic, bf.no_shadow)),
                ..Default::default()
            });
            g.fx.push(crate::game::GameFxItem { box_, ttl: 0.11 });
        }
    }
    dt
}

// ---------------------------------------------------------------- a charge going on

/// A setter's hands on the charge while he sets it: the pose, and the slab under his palms once they have pressed it there. (handwork.ts chargeHands)
pub fn charge_hands(g: &mut crate::game::Game, c_2: CharId, p: V3, n: V3, t: f64, dur: f64, hand: Option<f64>) {
    let more = WorkAtMore { hand, ..Default::default() };
    work_at(g, c_2, crate::character::WorkKind::Charge, p, n, t, dur, Some(&more));
    if t > 0.35 && crate::character::anim_work_info(g, c_2).w > 0.5 {
        let owner = owner_of(g, c_2);
        let box_ = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
            c: v3::mad(p, n, 0.02),
            h: [0.07, 0.11, 0.02],
            yaw: Some(m::atan2(n[0], n[2])),
            albedo: Some([0.09, 0.1, 0.08]),
            flags: Some(crate::scene_boxes::box_flag().dynamic),
            owner: Some(owner),
            ..Default::default()
        });
        g.fx.push(crate::game::GameFxItem { box_, ttl: 1e-3 });
    }
}

// ---------------------------------------------------------------- Sam's plants: the breaker, the drive, a wall charge

/// The face of the breaker panel toward a man: the panel box's thin axis when it has one, else the axis nearest him. (handwork.ts breakerFace)
pub fn breaker_face(g: &mut crate::game::Game, from: V3) -> crate::handwork::BreakerFace {
    let pos = g.level.breaker.pos;
    // `statics[boxes[0]]`: undefined past the end, or with no boxes at all
    let b = match g.level.breaker.boxes.first().copied() {
        Some(i) if i >= 0.0 && (i as usize) as f64 == i => g.engine.world.statics.get(i as usize).copied(),
        _ => None,
    };
    if let Some(b) = b {
        let (bc, bh, yaw, rot) = seam_box_read(g, b);
        if !js::truthy(yaw) && !rot && m::min(bh[0], bh[2]) < 0.5 * m::max(bh[0], bh[2]) {
            let n: V3 = if bh[2] < bh[0] { [0.0, 0.0, js::or(m::sign(from[2] - bc[2]), 1.0)] } else { [js::or(m::sign(from[0] - bc[0]), 1.0), 0.0, 0.0] };
            return BreakerFace { p: [bc[0] + n[0] * bh[0], bc[1], bc[2] + n[2] * bh[2]], n };
        }
    }
    BreakerFace { p: pos, n: face_toward(pos, from) }
}

/// The face of the objective's holder toward a man: the office's rack faces ±z; a quiet mission's holder faces the way he came at it. (handwork.ts rackFace)
pub fn rack_face(g: &mut crate::game::Game, from: V3) -> crate::handwork::RackFace {
    let rf = g.level.mission.rack.front;
    let n = if g.quiet.active { face_toward(rf, from) } else { [0.0, 0.0, js::or(m::sign(from[2] - rf[2]), 1.0)] };
    RackFace { p: rf, n }
}

/// What Sam's hands are on, if a verb of his that plants him is running. (handwork.ts samWork)
pub fn sam_work(g: &mut crate::game::Game) -> Option<crate::handwork::SamWork> {
    if g.puppet.is_some() { return None; }
    let pl = g.player;
    let c = g.pl[pl].ch;
    let p = g.ch[c].pos;
    if g.pl[pl].breaker_move.is_some() {
        let f = breaker_face(g, p);
        return Some(SamWork { kind: SamWorkKind::Breaker, p: f.p, n: f.n, at: None });
    }
    if g.mission.stage == crate::game::MissionStage::Drive && g.mission.drive_t >= 0.0 {
        let f = rack_face(g, p);
        return Some(SamWork { kind: SamWorkKind::Drive, p: f.p, n: f.n, at: None });
    }
    if let Some(j) = crate::wallcharge::wall_charge_sam_job(g) { return Some(SamWork { kind: SamWorkKind::Wall, p: j.p, n: j.n, at: Some(j.at) }); }
    if let Some(d) = crate::doortools::door_tools_sam_job(g) { return Some(SamWork { kind: SamWorkKind::Door, p: d.p, n: d.n, at: Some(d.at) }); }
    if let Some(j) = crate::defuse::defuse_sam_job(g) { return Some(SamWork { kind: SamWorkKind::Defuse, p: j.p, n: j.n, at: Some(j.at) }); }
    if let Some(s) = crate::lightswitch::switch_hand(g) { return Some(SamWork { kind: SamWorkKind::Switch, p: s.p, n: s.n, at: None }); }
    None
}

/// The 'breaker' rows: his hand goes to the lever and throws it. True = taken here. (handwork.ts breakerStart)
pub fn breaker_start(g: &mut crate::game::Game, on: bool) -> bool {
    if g.puppet.is_some() { return false; }
    let pl = g.player;
    if g.pl[pl].breaker_move.is_some() { return true; }
    g.pl[pl].breaker_move = Some(crate::game::PlayerBreakerMove { on, t: 0.0, thrown: false });
    g.pl[pl].hand_wait = 0.0;
    g.stats.handwork.levers += 1;
    true
}

/// Once a frame: the lever's move waits for the hand, the mains go over at leverAt, the hand comes away at leverSecs. (handwork.ts breakerTick)
pub fn breaker_tick(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let Some(mut mv) = g.pl[pl].breaker_move else { return };
    let bp = g.level.breaker.pos;
    let c = g.pl[pl].ch;
    if g.pl[pl].down
        || g.pl[pl].holding.is_some()
        || g.pl[pl].takedown.is_some()
        || g.pl[pl].dragging.is_some()
        || crate::samstun::sam_staggered(g, None)
        || v3::dist_xz(bp, g.ch[c].pos) > 1.6
        || (crate::sim_lightshome::mains_active(g) && crate::sim_lightshome::mains_permanent(g))
    {
        g.pl[pl].breaker_move = None;
        return;
    }
    let pos = g.ch[c].pos;
    let f = breaker_face(g, pos);
    let hand = sam_hand(g);
    let more = WorkAtMore { dir: Some(if mv.on { -1.0 } else { 1.0 }), hand, keys: None };
    let lever_secs = g.k.handwork.lever_secs;
    work_at(g, c, crate::character::WorkKind::Lever, f.p, f.n, mv.t, lever_secs, Some(&more));
    if mv.t <= 0.0 && !hand_there(g, pl, dt, f.p, f.n) { return; }
    mv.t += dt;
    g.pl[pl].breaker_move = Some(mv);
    if !mv.thrown && mv.t >= m::min(g.k.handwork.lever_at, lever_secs - 0.1) {
        mv.thrown = true;
        g.pl[pl].breaker_move = Some(mv);
        if mv.on {
            if crate::sim_lightshome::mains_active(g) && !crate::sim_lightshome::mains_permanent(g) { crate::game::game_end_blackout(g); }
        } else if !crate::sim_lightshome::mains_active(g) {
            crate::game::game_set_blackout(g, 86400.0, crate::game::GameSetBlackoutCause::Hand);
        }
    }
    if mv.t >= lever_secs { g.pl[pl].breaker_move = None; }
}

/// A click while a plant runs lets go of it; the breaker's half second is committed: false = not let go of. (handwork.ts samWorkLetGo)
pub fn sam_work_let_go(g: &mut crate::game::Game, w: crate::handwork::SamWork) -> bool {
    match w.kind {
        SamWorkKind::Breaker => return false,
        SamWorkKind::Switch => crate::lightswitch::switch_let_go(g),
        SamWorkKind::Drive => crate::mission::cancel_drive(g),
        SamWorkKind::Defuse => { crate::defuse::defuse_cancel(g); }
        SamWorkKind::Door => { crate::doortools::door_tools_cancel_sam(g); }
        SamWorkKind::Wall => { crate::wallcharge::wall_charge_cancel(g); }
    }
    true
}

/// A guard at the thrown breaker, re-arming it: his hand on the lever for the `dur` he fumbles. (handwork.ts leverHands)
pub fn lever_hands(g: &mut crate::game::Game, c_2: CharId, t: f64, dur: f64) {
    let pos = g.ch[c_2].pos;
    let f = breaker_face(g, pos);
    let more = WorkAtMore { dir: Some(-1.0), ..Default::default() };
    work_at(g, c_2, crate::character::WorkKind::Lever, f.p, f.n, t, dur, Some(&more));
}

/// A teammate sent to work a thing: the work pose in the door hand's place, his left working. True = this layer has the hand. (handwork.ts teamWorkHand)
pub fn team_work_hand(g: &mut crate::game::Game, c_2: CharId, h: Option<crate::handwork::TeamWorkHandH>, kind: Option<crate::handwork::TeamWorkHandKind>, t: Option<f64>, dur: Option<f64>, dir: Option<f64>) -> bool {
    let Some(h) = h else { return false };
    let kind = match kind.unwrap_or(TeamWorkHandKind::Lever) {
        TeamWorkHandKind::Lever => crate::character::WorkKind::Lever,
        TeamWorkHandKind::Take => crate::character::WorkKind::Take,
    };
    let (t, dur, dir) = (t.unwrap_or(0.0), dur.unwrap_or(1.0), dir.unwrap_or(1.0));
    if h.w > 0.0 {
        let more = WorkAtMore { dir: Some(dir), hand: Some(0.0), keys: None };
        work_at(g, c_2, kind, h.p, h.n, t, dur, Some(&more));
    }
    true
}

/// The man sent to the breaker: in the re-arm's reach he steps up and squares to it; the way to face and the step's pace, or null. (handwork.ts leverStand)
pub fn lever_stand(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<crate::handwork::LeverStand> {
    let c = g.gd[gd].ch;
    if !crate::lighting::breaker_reset_near(g, gd) { return None; }
    let pos = g.ch[c].pos;
    let f = breaker_face(g, pos);
    let (x, z) = (pos[0], pos[2]);
    let yaw = work_stand(g, c, f.p, f.n, dt, Some(0.0), Some(1.3));
    let now = g.ch[c].pos;
    Some(LeverStand { yaw, speed: if dt > 0.0 { m::hyp2(now[0] - x, now[2] - z) / dt } else { 0.0 } })
}

/// Once a frame of the drive stage: both hands in the rack, then the thing put away at his hip. `pulled` = this is the frame it came free. (handwork.ts driveHands)
pub fn drive_hands(g: &mut crate::game::Game, secs: f64, pulled: bool) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    if pulled {
        let stow = g.k.handwork.stow_secs;
        crate::character::anim_work_after(g, c, stow);
        return;
    }
    if g.mission.drive_t < 0.0 { return; }
    let pos = g.ch[c].pos;
    let f = rack_face(g, pos);
    let t = g.mission.drive_t;
    let hand = sam_hand(g);
    let more = WorkAtMore { hand, ..Default::default() };
    work_at(g, c, crate::character::WorkKind::Take, f.p, f.n, t, secs, Some(&more));
}

/// The thing in his hand while it comes out and is put away: a caddy-sized box off the working hand. (handwork.ts workProps)
pub fn work_props(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>) {
    let i = crate::character::anim_work_info(g, c_2);
    let Some(l) = i.pose else { return };
    if l.kind != crate::character::WorkKind::Take || i.w < 0.4 { return; }
    let u = l.t / l.dur;
    if u < 0.5 || l.t > l.dur + g.k.handwork.stow_secs * 0.8 { return; }
    let right = js::truthy(i.hand);
    let bone = if right { crate::character::Bone::HandR } else { crate::character::Bone::HandL };
    let Some(h) = g.ch[c_2].bones.get(bone).copied() else { return };
    let f = crate::character::character_forward(g, c_2);
    let r: V3 = [-f[2], 0.0, f[0]];
    let s = if i.two { if right { 0.045 } else { -0.045 } } else { 0.0 };
    let done = l.t > l.dur;
    let bf = crate::scene_boxes::box_flag();
    let owner = owner_of(g, c_2);
    let b = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: if done { [h[0], h[1] - 0.07, h[2]] } else { [h[0] + f[0] * 0.09 - r[0] * s, h[1], h[2] + f[2] * 0.09 - r[2] * s] },
        h: [0.05, 0.013, 0.07],
        yaw: Some(m::atan2(f[0], f[2])),
        albedo: Some([0.13, 0.14, 0.15]),
        rough: Some(0.4),
        metal: Some(true),
        flags: Some(flag_or(bf.dynamic, bf.no_shadow)),
        owner: Some(owner),
        ..Default::default()
    });
    out.push(b);
}

// ---------------------------------------------------------------- the radio hand

/// A line that goes over the net: his hand goes to the handset for about as long as the line stands. (handwork.ts radioHand)
pub fn radio_hand(g: &mut crate::game::Game, gd: GuardId, secs: f64) {
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].civ.is_some() { return; }
    g.stats.handwork.radio += 1;
    let c = g.gd[gd].ch;
    let secs = m::min(g.k.handwork.radio_max, m::max(g.k.handwork.radio_min, secs * g.k.handwork.radio_k));
    crate::character::anim_radio(g, c, secs);
}

// ---------------------------------------------------------------- the guard's torch / sidearm swap

/// Once a frame per man before anybody is posed: what the rule has in his gun hand; the animator makes a move of a change. (handwork.ts swapFeed)
pub fn swap_feed(g: &mut crate::game::Game, gd: GuardId, held: Option<crate::handwork::SwapFeedHeld>, plain: bool) {
    if !plain || g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() { return; }
    let c = g.gd[gd].ch;
    g.ch[c].anim.swap_lag = crate::character::swap().lag * js::rem(g.ch[c].id * 0.6180339887, 1.0);
    let si = crate::character::anim_swap_info(g, c);
    if let Some(h) = held {
        if si.a < 0.0 && !si.pending {
            let shown = crate::character::anim_swap_held(g, c, Some(rule_of(h)));
            let same = matches!(
                (shown, h),
                (Some(crate::character::AnimSwapHeld::Torch), SwapFeedHeld::Torch) | (Some(crate::character::AnimSwapHeld::Weapon), SwapFeedHeld::Weapon)
            );
            if !same { g.stats.handwork.swaps += 1; }
        }
    }
    g.ch[c].anim.carry = held.map(|h| match h {
        SwapFeedHeld::Torch => crate::character::AnimCarry::Torch,
        SwapFeedHeld::Weapon => crate::character::AnimCarry::Weapon,
    });
}

/// What his gun hand visibly holds this frame: the rule's answer, but for the frames of a swap. (handwork.ts swapHeld)
pub fn swap_held(g: &mut crate::game::Game, gd: GuardId, held: Option<crate::handwork::SwapHeldHeld>) -> Option<crate::handwork::SwapHeld> {
    let c = g.gd[gd].ch;
    let rule = held.map(|h| match h {
        SwapHeldHeld::Torch => crate::character::AnimSwapHeldRule::Torch,
        SwapHeldHeld::Weapon => crate::character::AnimSwapHeldRule::Weapon,
    });
    crate::character::anim_swap_held(g, c, rule).map(|h| match h {
        crate::character::AnimSwapHeld::Torch => SwapHeld::Torch,
        crate::character::AnimSwapHeld::Weapon => SwapHeld::Weapon,
    })
}

/// Is a swap running on him: 0..1 through it, -1 none. (handwork.ts swapping)
pub fn swapping(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let c = g.gd[gd].ch;
    crate::character::anim_swap_info(g, c).a
}

/// The torch on his belt while the sidearm is out: at the front of the right hip, lens down, dark. (handwork.ts beltTorch)
pub fn belt_torch(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>) {
    let Some(hp) = g.ch[c_2].bones.get(crate::character::Bone::Hips).copied() else { return };
    let f = crate::character::character_forward(g, c_2);
    let r: V3 = [-f[2], 0.0, f[0]];
    let spot = crate::inhand::belt_torch().at;
    let (o, a) = (spot[0], spot[1]);
    let at: V3 = [hp[0] + r[0] * o + f[0] * a, hp[1] - 0.045, hp[2] + r[2] * o + f[2] * a];
    let bf = crate::scene_boxes::box_flag();
    let owner = owner_of(g, c_2);
    let b = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: at,
        h: [0.017, 0.062, 0.017],
        yaw: Some(m::atan2(f[0], f[2])),
        albedo: Some([0.05, 0.05, 0.055]),
        flags: Some(flag_or(bf.dynamic, bf.no_shadow)),
        owner: Some(owner),
        ..Default::default()
    });
    out.push(b);
    let b = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: [at[0], at[1] - 0.07, at[2]],
        h: [0.021, 0.012, 0.021],
        yaw: Some(m::atan2(f[0], f[2])),
        albedo: Some([0.07, 0.07, 0.075]),
        stuff: Some(crate::scene_stuff::BoxStuff::Metal),
        flags: Some(flag_or(bf.dynamic, bf.no_shadow)),
        owner: Some(owner),
        ..Default::default()
    });
    out.push(b);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(v: &[[f64; 2]]) -> Vec<[u64; 2]> { v.iter().map(|k| [k[0].to_bits(), k[1].to_bits()]).collect() }

    // the TypeScript's own bits: padKeys under bun with EXACT.on
    #[test]
    fn pad_keys_match_the_typescript() {
        assert_eq!(bits(&pad_keys("1937")), PAD_1937.to_vec());
        assert_eq!(bits(&pad_keys("*0")), PAD_STAR0.to_vec());
        assert_eq!(bits(&pad_keys("7x")), PAD_7X.to_vec());
        assert_eq!(bits(&pad_keys("")), PAD_EMPTY.to_vec());
    }

    #[test]
    fn face_toward_takes_the_nearer_axis_and_never_zero() {
        assert_eq!(face_toward([0.0; 3], [2.0, 0.0, 1.0]), [1.0, 0.0, 0.0]);
        assert_eq!(face_toward([0.0; 3], [-2.0, 0.0, 1.0]), [-1.0, 0.0, 0.0]);
        assert_eq!(face_toward([0.0; 3], [1.0, 0.0, -2.0]), [0.0, 0.0, -1.0]);
        // a tie goes to z, and standing on the spot still faces somewhere
        assert_eq!(face_toward([0.0; 3], [1.0, 0.0, 1.0]), [0.0, 0.0, 1.0]);
        assert_eq!(face_toward([0.0; 3], [0.0; 3]), [0.0, 0.0, 1.0]);
    }

    const PAD_1937: [[u64; 2]; 4] = [[0xbf947ae147ae147b, 0x3f870a3d70a3d70c], [0x3f947ae147ae147b, 0xbf9851eb851eb852], [0x3f947ae147ae147b, 0x3f870a3d70a3d70c], [0xbf947ae147ae147b, 0xbf9851eb851eb852]];
    const PAD_STAR0: [[u64; 2]; 4] = [[0xbf947ae147ae147b, 0xbfa51eb851eb851f], [0x0000000000000000, 0xbfa51eb851eb851f], [0xbf947ae147ae147b, 0xbfa51eb851eb851f], [0x0000000000000000, 0xbfa51eb851eb851f]];
    const PAD_7X: [[u64; 2]; 4] = [[0xbf947ae147ae147b, 0xbf9851eb851eb852], [0x0000000000000000, 0xbf79999999999998], [0xbf947ae147ae147b, 0xbf9851eb851eb852], [0x0000000000000000, 0xbf79999999999998]];
    const PAD_EMPTY: [[u64; 2]; 1] = [[0x0000000000000000, 0xbf79999999999998]];
}
