//! src/game/game.ts lines 1856 to 2165
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- seams: what skeleton 3 still cannot do (each has a line in notes.md)

/// game.ts:1869 a light's `enabled`, `outerDeg`, `pos`, `dir`
fn seam_light(g: &mut Game, l: LightId) -> (bool, f64, V3, V3) { todo!("needs: accessors of a LightId (no table, no RtLight in the index)") }
/// game.ts:2067 `O.solver !== this.smoke`, of the owned solver standing
fn seam_owned_solver_is_other(g: &mut Game) -> bool { todo!("needs: an identity for the smoke solver (GameSmokeOwned has no fields)") }
/// game.ts:2068 `O.undo()`
fn seam_smoke_owned_undo(g: &mut Game) { todo!("needs: GameSmokeOwned to hold what own_smoke answers, and FnField to be something that can be called") }
/// game.ts:1970, :1973, :1976, :1977 `this.smoke.push({ pos, dir, speed, radius, body })`
fn seam_smoke_push(g: &mut Game, pos: V3, dir: V3, speed: f64, radius: f64, body: Option<f64>) { todo!("needs: SmokePush's fields (it is an empty struct)") }

/// game.ts `s.view.…`: one call over a seat's view (conventions 4)
fn in_seat<R>(g: &mut Game, s: SeatId, f: &mut dyn FnMut(&mut Game) -> R) -> R {
    let view = crate::cmd::seat_man(g, s);
    crate::seats::with_seat(g, view, f)
}
/// game.ts `WEAPONS[id]`
fn weapon_row(g: &Game, id: crate::weapons::WeaponId) -> &crate::weapons::WeaponDef { g.m.weapons.weapons.get(id).unwrap() }

/// a bone of his if it has been baked, else his feet raised by `up`
fn bone_or(g: &Game, c: CharId, b: crate::character::Bone, up: f64) -> V3 {
    match g.ch[c].bones.get(b) { Some(p) => *p, None => v3::add(g.ch[c].pos, [0.0, up, 0.0]) }
}

/// threat proximity → how deliberately Sam moves   (game.ts threatTick)
pub fn game_threat_tick(g: &mut crate::game::Game) {
    use crate::character::Bone;
    use crate::game::GuardState;
    let pl = g.player; let pc = g.pl[pl].ch;
    let mut near: Option<GuardId> = None; let mut nd = f64::INFINITY; let mut t = 0.0;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i]; i += 1;
        if g.gd[gd].state == GuardState::Dead || g.gd[gd].held.is_some() || g.gd[gd].team == crate::teams::Team::Operative || crate::fogofwar::fog_of_war_hidden(g, gd) { continue; }
        let d = v3::dist_xz(g.ch[g.gd[gd].ch].pos, g.ch[pc].pos);
        if d < nd { nd = d; near = Some(gd); }
    }
    if let Some(nr) = near {
        if !g.pl[pl].down && g.gd[nr].state != GuardState::Alert {
            t = 1.0 - v3::smoothstep(3.0, 7.0, nd);
            if g.gd[nr].state == GuardState::Suspicious || g.gd[nr].state == GuardState::Search { t = m::max(t, 0.6 * (1.0 - v3::smoothstep(7.0, 10.0, nd))); }
            let p = g.ch[pc].pos;
            let spots: [V3; 2] = [[p[0], 0.15, p[2]], bone_or(g, pc, Bone::Chest, 1.1)];
            if g.torch_on_t > g.time { g.torch_on_t = -100.0; }
            let mut i = 0;
            'torch: while i < g.guards.len() {
                let gd = g.guards[i]; i += 1;
                if g.time - g.torch_on_t < 1.5 { break; }
                let fl = g.gd[gd].flashlight;
                if g.gd[gd].state == GuardState::Dead { continue; }
                let (enabled, outer_deg, fl_pos, fl_dir) = seam_light(g, fl);
                if !enabled || g.gd[gd].team == crate::teams::Team::Operative { continue; }
                let cos_o = m::cos(outer_deg * v3::DEG);
                for s in spots {
                    let to = v3::sub(s, fl_pos); let d = v3::len(to);
                    if d < 10.0 && d > 1e-3 && v3::dot(to, fl_dir) / d > cos_o && !crate::scene_collision::segment_blocked(g, fl_pos, s, None, None, None) { g.torch_on_t = g.time; break 'torch; }
                }
            }
            if g.time - g.torch_on_t < 1.5 { t = m::max(t, 0.5); }
        }
    }
    g.threat_proximity = t; g.ch[pc].anim.deliberate = t;
}

/// the muzzle of the stand-in gun in his hands   (game.ts:1895, :1900)
fn stand_in_muzzle(g: &Game, long: Option<crate::guards::LongRigOf>, cb: bool, def: crate::weapons::WeaponId) -> f64 {
    if long == Some(crate::guards::LongRigOf::Sc20k) { return crate::rig_props::sc20k_muzzle_ahead(None); }
    let row = weapon_row(g, def);
    if cb && row.rig != crate::weapons::WeaponDefRig::Carbine { return crate::rig_props::carbine_muzzle_ahead(None); }
    row.muzzle_ahead
}

/// the carbine dealt by the sandbox knob, and every man's animator armed with what is then in his hands   (game.ts dealWeapons)
pub fn game_deal_weapons(g: &mut crate::game::Game, dt: Option<f64>) {
    use crate::character::AnimLongGrip;
    use crate::game::{CarbineDeal, GuardState};
    use crate::guards::LongRigOf;
    use crate::teams::Team;
    use crate::weapons::{GunModelId, WeaponId};
    let dt = dt.unwrap_or(0.0);
    let deal = g.tune.carbine_guards;
    let mut grip: Option<AnimLongGrip> = None; let mut grip20k: Option<AnimLongGrip> = None;
    let mut grip_own: Vec<(GunModelId, Option<AnimLongGrip>, Option<f64>)> = Vec::new();
    let show = g.touring || crate::director::director_showing(g);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i]; i += 1;
        let c = g.gd[gd].ch;
        if g.gd[gd].unit.is_none() { g.gd[gd].carbine = deal == CarbineDeal::All || (deal == CarbineDeal::Reyes && g.gd[gd].callsign == "Reyes"); }
        let route = g.gd[gd].route;
        let want = if g.gd[gd].carbine { WeaponId::Aks74u } else { g.tab.route[route].weapon.unwrap_or(WeaponId::Guard9mm) };
        if g.gd[gd].state != GuardState::Dead && g.fa[g.gd[gd].weapon].def != want { let row = weapon_row(g, want).clone(); let w = crate::weapons::firearm_new(g, &row); g.gd[gd].weapon = w; g.gd[gd].reload_t = -1.0; }
        if g.gd[gd].work_stow.is_some() { crate::workstow::mate_stow_tick(g, gd); }
        if g.gd[gd].second.is_some() { crate::gunslots::side_tick(g, gd, Some(dt)); }
        let long = crate::guards::long_rig_of(g, gd); let cb = long.is_some();
        let def = g.fa[g.gd[gd].weapon].def;
        if g.ch[c].anim.long_gun != cb {
            g.ch[c].anim.long_gun = cb;
            g.ch[c].anim.long_steady = if long == Some(LongRigOf::Sc20k) { 1.0 } else { 0.0 };
            let a = stand_in_muzzle(g, long, cb, def);
            g.ch[c].muzzle_ahead = a;
        }
        if long == Some(LongRigOf::Sc20k) {
            let lg = *grip20k.get_or_insert_with(|| { let x = crate::rig_props::sc20k_grip(None, None); AnimLongGrip { wrist: x.wrist, fingers: x.fingers, back: x.back } });
            g.ch[c].anim.long_grip = lg;
        } else if cb {
            let lg = *grip.get_or_insert_with(|| { let x = crate::rig_props::carbine_grip(None); AnimLongGrip { wrist: x.wrist, fingers: x.fingers, back: x.back } });
            g.ch[c].anim.long_grip = lg;
        }
        if crate::gunlook::gun_look_on(g) || g.ch[c].gun_look.is_some() {
            let had = g.ch[c].gun_look.as_ref().and_then(|l| l.hand);
            let into = g.ch[c].gun_look.clone();
            let in_hands = weapon_row(g, def).clone();
            let other = g.gd[gd].second.as_ref().map(|s| weapon_row(g, g.fa[s.gun].def).clone());
            let third = match g.gd[gd].slung.as_ref() { Some(s) if s.out => Some(weapon_row(g, g.fa[s.own].def).clone()), _ => None };
            let look = crate::gunlook::gun_look_of(g, into, Some(&in_hands), other.as_ref(), third.as_ref());
            let hand = if cb { look.as_ref().and_then(|l| l.hand) } else { None };
            g.ch[c].gun_look = look;
            if let Some(mo) = hand {
                let at = match grip_own.iter().position(|r| r.0 == mo) {
                    Some(k) => k,
                    None => {
                        let gr = crate::rig_props::model_grip(g, mo, None, None).map(|x| AnimLongGrip { wrist: x.wrist, fingers: x.fingers, back: x.back });
                        let a = crate::rig_props::model_muzzle_ahead(g, mo, None);
                        grip_own.push((mo, gr, a)); grip_own.len() - 1
                    }
                };
                if let Some(lg) = grip_own[at].1 { g.ch[c].anim.long_grip = lg; }
                if let Some(a) = grip_own[at].2 { g.ch[c].muzzle_ahead = a; }
            } else if had.is_some() && cb {
                let a = stand_in_muzzle(g, long, true, def);
                g.ch[c].muzzle_ahead = a;
            }
            let gun = if cb { Some(g.gd[gd].weapon) } else { None };
            crate::gunreload::feed_deal(g, c, gun, Some(true));
        }
        let team = g.gd[gd].team;
        let trained = if team == Team::Operative { 1.0 } else if g.gd[gd].trained { 0.75 } else if team == Team::Civilian { 0.5 } else { 0.12 };
        g.ch[c].anim.trained = trained;
        let held = crate::guards::in_hand(g, gd).map(|h| match h { crate::guards::InHand::Torch => crate::handwork::SwapFeedHeld::Torch, crate::guards::InHand::Weapon => crate::handwork::SwapFeedHeld::Weapon });
        crate::handwork::swap_feed(g, gd, held, !cb);
        let in_hand = crate::guards::in_hand(g, gd).is_some();
        g.ch[c].anim.grip[1] = in_hand;
        // (GEAR.on, page true: `long === 'sc20k' && !GEAR.on` is false)
        g.ch[c].anim.reload_back = false;
        let pellets = weapon_row(g, def).pellets.is_some();
        g.ch[c].anim.reload_mag = !pellets;
        let (dead, now) = (g.gd[gd].state == GuardState::Dead, g.time);
        g.ch[c].anim.clutch = if !dead && now < g.gd[gd].stagger_until && !(now < g.gd[gd].dazzled_until) { 1.0 } else { 0.0 };
        // (MOVES.on, page 1: the block always runs)
        {
            let w = g.gd[gd].weapon;
            let r = crate::weapons::firearm_reload_row(g, w);
            let reload_t = g.gd[gd].reload_t;
            let fo = if reload_t < 0.0 && !dead { crate::roomfight::room_fight_op_of(g, gd) } else { None };
            let at = if dead { -1.0 }
                else if reload_t >= 0.0 { r.dur - reload_t }
                else { match fo { Some(o) if g.tab.roomfight_op[o].reload_until > now => m::max(0.0, r.dur - (g.tab.roomfight_op[o].reload_until - now)), _ => -1.0 } };
            let row = crate::character::ReloadBeat { dur: r.dur, out: r.out, in_: r.in_, rack: r.rack, keep: None, hand: None, exact: None };
            let an = &mut g.ch[c].anim;
            an.reload_row = Some(row);
            an.reload_at = at;
        }
        // (MOVES.on, page 1)
        let dip = g.k.moves.entry > 0.5 && cb && !show && g.gd[gd].held.is_none() && !dead && team == Team::Operative
            && !crate::roomfight::room_fight_busy(g, gd, 1.5) && !crate::roomfight::room_fight_contact(g, gd) && crate::game::friend_before_muzzle(g, gd);
        g.ch[c].anim.dip = if dip { 1.0 } else { 0.0 };
        if !dead && now < g.gd[gd].dazzled_until {
            // (trap: the TypeScript's `from` may be his own position array, which follows him; here it is this moment's copy: notes.md)
            let from = g.gd[gd].dazzled_at.or(g.gd[gd].last_known).unwrap_or(g.ch[c].pos);
            let k = m::min(1.0, (g.gd[gd].dazzled_until - now) / 0.5);
            let an = &mut g.ch[c].anim;
            let dz = an.dazzle.get_or_insert(crate::character::AnimDazzle { from, k: 0.0 });
            dz.from = from; dz.k = k;
        } else { g.ch[c].anim.dazzle = None; }
        {
            let on = !show && g.gd[gd].held.is_none();
            let tk = g.ch[c].anim.trained;
            let h = crate::weapons::handling_of(weapon_row(g, def));
            let tw = &mut g.ch[c].turn;
            tw.on = on; tw.h = h; tw.k = tk;
        }
    }
    let seats = g.seats.clone();
    for s in seats { let pl = crate::cmd::seat_man(g, s); let c = g.pl[pl].ch; g.ch[c].anim.trained = 1.0; }
    crate::condition::condition_pose(g);
}

/// this frame's dynamic boxes, and what steps while they are gathered. Returns whether stature scaled anybody: never here, the row being the picture's   (game.ts stepFxAndDynBoxes)
pub fn game_step_fx_and_dyn_boxes(g: &mut crate::game::Game, all: &[CharId], dt: f64, canvas: crate::host_hud::View) -> bool {
    g.dyn_boxes.clear();
    // (the list is lent to the callees for the whole gathering: none of them reads g.dyn_boxes)
    let mut out = core::mem::take(&mut g.dyn_boxes);
    crate::fogofwar::fog_of_war_door_boxes(g, &mut out);
    crate::props::prop_system_boxes(g, &mut out);
    crate::throwables::throwables_boxes(g, &mut out);
    crate::destruction::destruction_boxes(g, &mut out);
    crate::devices::device_boxes(g, &mut out);
    if !g.rings.is_empty() { crate::launcher::ring_boxes(g, &mut out); }
    let seats = g.seats.clone();
    for &s in seats.iter() {
        let pl = crate::cmd::seat_man(g, s);
        if !g.pl[pl].cams.is_empty() { in_seat(g, s, &mut |g| crate::stickycam::sticky_boxes(g, &mut out)); }
    }
    crate::lightswitch::switch_boxes(g, &mut out);
    if !g.shells.is_empty() { crate::he40::shell_boxes(g, &mut out); }
    let me = crate::game::game_host_seat(g);
    let cam = crate::game::GameViewPxCam { distance: g.tab.cmd_seat[me].cam.distance, fov: g.tab.cmd_seat[me].cam.fov };
    g.m_per_px = game_view_px(g, cam, crate::game::GameViewPxCanvas { height: canvas.height });
    let statured = false;   // (DRESS.stature: statureTick is the picture's)
    let rig0 = out.len();
    crate::combat::hand_props(g, &mut out);
    g.fog_of_war.rig_from = rig0 as f64; g.fog_of_war.rig_to = out.len() as f64;
    g.fx.clear();   // (DRESS.fx down: a host that draws nothing keeps no list of them)
    if !g.tune.tracers && js::truthy(g.tracers.live) { crate::tracers::tracers_clear(g); }
    let px = g.m_per_px;
    crate::tracers::tracers_update(g, dt, &mut out, Some(px));
    crate::sparks::sparks_update(g, dt);
    crate::sparks::sparks_boxes(g, &mut out);
    for &s in seats.iter() { in_seat(g, s, &mut |g| crate::blast_look::blast_look_tick(g, &mut out)); }
    g.dyn_boxes = out;
    statured
}

/// men moving through smoke push it. The tick does not call it: DRESS.wake, the picture's   (game.ts smokeWake)
pub fn game_smoke_wake(g: &mut crate::game::Game, all: &[CharId]) {
    use crate::character::Bone;
    let stir = if g.m.render_frame.look_config.on { m::max(0.0, g.m.render_frame.look_config.stir) } else { 0.0 };
    crate::smoke_types::smoke_system_like_set_stir(g, stir);
    for &c in all.iter() {
        let vel = g.ch[c].vel;
        let sp = m::hyp2(vel[0], vel[2]);
        let Some(h) = g.ch[c].bones.get(Bone::Hips).copied() else { continue; };
        if sp < 0.25 { continue; }
        let dirv: V3 = [vel[0] / sp, 0.05, vel[2] / sp];
        if stir > 0.0 {
            let t = crate::game::smoke_stir();
            let (hx, hz) = (dirv[0], dirv[2]);
            let top = h[1] + t.above; let mid = (top + t.foot) / 2.0;
            seam_smoke_push(g, [h[0], mid, h[2]], [hx, 0.0, hz], sp, t.body, Some(m::max(0.0, (top - t.foot) / 2.0 - t.body)));
            let side = m::sin(2.0 * m::PI * g.ch[c].anim.phase);
            if m::abs(side) > t.gap {
                let k = m::sign(side);
                seam_smoke_push(g, [h[0] - hx * t.back, h[1] - t.low, h[2] - hz * t.back], [-hz * k, 0.0, hx * k], m::min(t.most, sp * t.kick * m::min(2.0, stir)), t.kick_r, None);
            }
            continue;
        }
        seam_smoke_push(g, h, dirv, sp * 1.15, 0.3, None);
        if let Some(chest) = g.ch[c].bones.get(Bone::Chest).copied() { seam_smoke_push(g, chest, dirv, sp * 1.1, 0.26, None); }
    }
}

/// characters → what each man wears (the renderer's own half is the picture's), and the frame's dynamic boxes handed to the world   (game.ts drawCharacters)
pub fn game_draw_characters(g: &mut crate::game::Game, all: &[CharId], statured: bool) {
    g.men_all = all.to_vec(); g.men_statured = statured; g.men_bare = false;
    game_draw_men(g, all, statured, false, Some(false));   // (DRESS.men down: dress.ts, "what each WEARS is still dealt")
    g.dyn_raw = g.dyn_boxes.len() as f64;
    let mut list = core::mem::take(&mut g.dyn_boxes);
    let dynamics = crate::fogofwar::fog_of_war_things(g, &mut list);
    g.dyn_boxes = list;
    g.engine.world.dynamics = dynamics;
}

/// the world size of one drawn pixel at a camera's look-at depth, for a view of `canvas`'s height   (game.ts viewPx)
pub fn game_view_px(g: &mut crate::game::Game, cam: crate::game::GameViewPxCam, canvas: crate::game::GameViewPxCanvas) -> f64 {
    let sharp_still = g.engine.sharp_on.unwrap_or(false) && g.engine.settings.sharp_decouple;
    view_px_sum(cam.distance, cam.fov, canvas.height, sharp_still, g.engine.effective_scale)
}
/// viewPx's sum
fn view_px_sum(distance: f64, fov: f64, height: f64, sharp_still: bool, effective_scale: f64) -> f64 {
    2.0 * distance * (if sharp_still { 1.6 } else { 1.0 }) * m::tan(fov / 2.0) / m::max(200.0, height * (if sharp_still { 1.0 } else { js::or(effective_scale, 1.0) }))
}

/// a viewer's picture without the fog of war, right before a draw and between two steps; fogBack right after it. Never in play   (game.ts fogLift)
pub fn game_fog_lift(g: &mut crate::game::Game, want: bool, tube: Option<bool>, px: Option<f64>) {
    let px = px.unwrap_or(0.0);
    if g.fog_held.is_some() { return; }
    let viewer = g.fog_of_war.viewer;
    let bare = want && viewer && crate::fogofwar::fog_of_war_active(g);
    let ir = match tube { Some(t) => viewer && !(g.engine.settings.debug_view > 0.0) && t != crate::irlaser::ir_viewer(g), None => false };
    if bare != g.men_bare && !g.men_all.is_empty() {
        let (all, statured) = (g.men_all.clone(), g.men_statured);
        game_draw_men(g, &all, statured, bare, Some(false));   // (DRESS.men down, as drawCharacters has it)
        g.men_bare = bare;
    }
    let thin = viewer && px > 0.0;
    if !bare && !ir {
        let t = if thin { let list = g.engine.world.dynamics.clone(); crate::thinline::thin_fit(g, &list, px) } else { None };
        let Some(t) = t else { return; };
        let held = core::mem::replace(&mut g.engine.world.dynamics, t);
        g.fog_held = Some(crate::game::GameFogHeld { dyn_: held, on: g.fog_of_war.frame.on });
        return;
    }
    let raw = if bare { crate::fogofwar::fog_of_war_raw(g) } else { None };
    let mut b = match raw { Some(l) => l, None => g.engine.world.dynamics.clone() };
    let n0 = b.len();
    g.fog_held = Some(crate::game::GameFogHeld { dyn_: g.engine.world.dynamics.clone(), on: g.fog_of_war.frame.on });
    if bare {
        crate::combat::hidden_props(g, &mut b);
        if g.men_statured { let all = g.men_all.clone(); crate::stature::drawn_boxes(g, &all, &mut b, n0 as f64); }
        g.fog_of_war.frame.on = false; g.fog_of_war.bared = true;
        crate::fogofwar::fog_of_war_faces_lift(g);
    }
    if ir {
        if tube == Some(true) { crate::combat::ir_beams(g, &mut b); }
        else {
            let mut kept = Vec::with_capacity(b.len());
            for &x in b.iter() { if !crate::irlaser::ir_made(g, x) { kept.push(x); } }
            b = kept;
        }
    }
    if thin { if let Some(t) = crate::thinline::thin_fit(g, &b, px) { b = t; } }
    g.engine.world.dynamics = b;
}

/// the step's own list and the fog's frame handed back after a lifted draw   (game.ts fogBack)
pub fn game_fog_back(g: &mut crate::game::Game) {
    let Some(h) = g.fog_held.take() else { return; };
    if g.fog_of_war.bared { crate::fogofwar::fog_of_war_faces_back(g); }
    g.fog_of_war.bared = false; g.fog_of_war.frame.on = h.on; g.engine.world.dynamics = h.dyn_;
    crate::scene_boxes::box_world_relist(g);
}

/// the one thing here the simulation reads: what each man wears, dealt in the order he is drawn. The renderer's half (`draw`) is the picture's, not translated   (game.ts drawMen)
pub fn game_draw_men(g: &mut crate::game::Game, all: &[CharId], statured: bool, bare: bool, draw: Option<bool>) {
    crate::scene_boxes::box_world_reset_capsules(g);
    let mut drawn = if bare { crate::fogofwar::fog_of_war_everybody(g, all) } else { crate::fogofwar::fog_of_war_filter(g, all) };
    if crate::rails::rails_sky_on(g) { let me = g.pl[g.player].ch; drawn.retain(|c| *c != me); }
    if drawn.len() > 8 && (g.night_breach.active || g.rescue.active) {
        let mut down: Vec<CharId> = Vec::new();
        let mut i = 0;
        while i < g.guards.len() { let gd = g.guards[i]; i += 1; if g.gd[gd].state == crate::game::GuardState::Dead { down.push(g.gd[gd].ch); } }
        if !down.is_empty() {
            let mut re: Vec<CharId> = drawn.iter().copied().filter(|c| !down.contains(c)).collect();
            re.extend(drawn.iter().copied().filter(|c| down.contains(c)));
            drawn = re;
        }
    }
    if g.tab.rig[g.rig].cast.is_some() {
        for &c in drawn.iter() { let parts = crate::cast::cast_parts(g, c); g.ch[c].parts = parts; }
    }
    crate::anim_characters::character_renderer_finish(g, 0.0);
}

/// the top of a step under SENSE.cpu (sense.ts): the game's own smoke put in front of the solver and taken away again, aged by this step   (game.ts senseStep)
pub fn game_sense_step(g: &mut crate::game::Game, dt: f64) {
    let on = g.k.sense.cpu; let had = g.smoke_owned.is_some();
    if on != had || (had && seam_owned_solver_is_other(g)) {
        if had { seam_smoke_owned_undo(g); }
        g.smoke_owned = if on {
            let _undo = crate::sense::own_smoke(g);   // (nothing holds the undo: see the seam)
            Some(crate::game::GameSmokeOwned::zeroed())
        } else { None };
        if on != had {
            crate::sense::cpu_smoke_clear(g);
            if g.cpu_smoke.grid.is_some() { crate::sense::smoke_grid_renew(g); }
            if on { crate::sensors::sensors_set_own(g, Some(&mut |g: &mut Game, a: V3, b: V3| crate::sense::cpu_smoke_along(g, a, b))); }
            else { crate::sensors::sensors_set_own(g, None); }
        }
    }
    if !on { return; }
    crate::sense::cpu_smoke_step(g, dt);
    if g.cpu_smoke.grid.is_some() { let (bounce, drift) = (g.k.sense.bounce, g.k.sense.drift); 
        let mut lamps = |g: &mut Game, _col: crate::scene_collision::StaticCollision| crate::guards::steady_lamps(g);
        let mut wind = |g: &mut Game| crate::wind::wind_storm_drift(g);
        let lamps: Option<&mut dyn FnMut(&mut Game, crate::scene_collision::StaticCollision) -> bool> = if bounce { Some(&mut lamps) } else { None };
        let wind: Option<&mut dyn FnMut(&mut Game) -> Option<crate::scene_collision::GridWind>> = if drift { Some(&mut wind) } else { None };
        crate::sense::smoke_grid_step(g, dt, lamps, wind);
    }
}

/// a seat's word on a sensor's key: its index, none for the first   (game.ts:2085, :2096)
fn seat_key(g: &Game, s: SeatId) -> String {
    let ix = g.tab.cmd_seat[s].index;
    if js::truthy(ix) { alloc::format!("{}", ix) } else { String::new() }
}

/// the sensors' asks (Sam's meter, the guards' smoke sight lines) and Sam's visibility off the last answer   (game.ts lightMeter)
pub fn game_light_meter(g: &mut crate::game::Game, dt: f64) {
    use crate::character::Bone;
    let seats = g.seats.clone(); let one = seats.len() == 1;
    for &s in seats.iter() {
        let pl = crate::cmd::seat_man(g, s); let pc = g.pl[pl].ch; let k = seat_key(g, s);
        let id = g.ch[pc].id;
        let chest = bone_or(g, pc, Bone::Chest, 1.2);
        crate::sensors::sensors_ask(g, &alloc::format!("playerChest{}", k), chest, Some(id));
        let head = bone_or(g, pc, Bone::Head, 1.6);
        crate::sensors::sensors_ask(g, &alloc::format!("playerHead{}", k), head, Some(id));
    }
    let tight = g.guards.len() > 8;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i]; let at = i; i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead { continue; }
        let pl = if one { g.player } else { crate::seats::view_for(g, gd) };
        let pc = g.pl[pl].ch;
        let chest = bone_or(g, pc, Bone::Chest, 1.1);
        let eye = bone_or(g, g.gd[gd].ch, Bone::Head, 1.65);
        let ask = !tight || crate::teams::hostile_to(g, gd, crate::teams::HostileToO::Player);
        let seen = crate::sensors::sensors_segment(g, &alloc::format!("smoke{}", at), eye, chest, Some(ask)).unwrap_or(1.0);
        let t = seen * crate::weather::air_trans(g, eye, chest);
        g.gd[gd].smoke_trans = t;
    }
    crate::fogofwar::fog_of_war_flush_asks(g);
    for &s in seats.iter() { let k = seat_key(g, s); in_seat(g, s, &mut |g| game_meter_read(g, dt, &k)); }
}

/// a seat's half of lightMeter, over its view: the player's visibility off the last answer to his two asks   (game.ts meterRead)
pub fn game_meter_read(g: &mut crate::game::Game, dt: f64, k: &str) {
    use crate::character::Bone;
    let pl = g.player; let pc = g.pl[pl].ch;
    let mut r1 = crate::sensors::sensors_read(g, &alloc::format!("playerChest{}", k));
    let mut r2 = crate::sensors::sensors_read(g, &alloc::format!("playerHead{}", k));
    if g.sensors.own.is_some() {
        let p = bone_or(g, pc, Bone::Chest, 1.2); r1 = Some(crate::guards::light_estimate_at(g, p, None, None, None, None));
        let p = bone_or(g, pc, Bone::Head, 1.6); r2 = Some(crate::guards::light_estimate_at(g, p, None, None, None, None));
    }
    if let (Some(a), Some(b)) = (r1, r2) {
        if a.is_finite() && b.is_finite() {
            let e = m::max(a, b); g.pl[pl].irradiance = e;
            let to = game_visibility_of(g, e);
            g.pl[pl].visibility = v3::damp(g.pl[pl].visibility, to, 6.0, dt);
        }
    }
    if !g.pl[pl].visibility.is_finite() {
        let ir = g.pl[pl].irradiance;
        g.pl[pl].visibility = if ir.is_finite() { game_visibility_of(g, ir) } else { 0.0 };
    }
    if !g.pl[pl].irradiance.is_finite() { g.pl[pl].irradiance = 0.0; }
}

/// put the highlight on the row of `kind` (the one whose verb is `verb`, if given) in this frame's list; false if none is on offer   (game.ts selectVerb)
pub fn game_select_verb(g: &mut crate::game::Game, kind: crate::game::GameSelectVerbKind, verb: Option<crate::game::Verb>) -> bool {
    let at = g.nearby.iter().position(|r| r.kind.as_str() == kind.as_str() && (verb.is_none() || r.verb == verb));
    let Some(i) = at else { return false; };
    let pl = g.player; g.pl[pl].near_sel = i as i32;
    crate::player::pick_hover(g);
    true
}

/// Esc, before the pause menu gets it: true = used here   (game.ts escape)
pub fn game_escape(g: &mut crate::game::Game) -> bool {
    crate::orders::squad_orders_escape(g) || crate::player::launch_cancel(g) || crate::interact::escape_interact(g) || crate::slotsetup::slot_setup_shut(g)
}

/// is there anything the back key would back out of right now   (game.ts backable)
pub fn game_backable(g: &mut crate::game::Game) -> bool {
    g.orders.menu.is_some() || g.pl[g.player].launch_held.is_some() || crate::interact::backable_interact(g) || crate::slotsetup::slot_setup_open(g)
}

/// the light meter's mapping, irradiance → 0‥1 visibility   (game.ts visibilityOf)
pub fn game_visibility_of(g: &mut crate::game::Game, e: f64) -> f64 {
    visibility_sum(g.tune.vis_slope, g.tune.vis_floor, e)
}
/// visibilityOf's sum
fn visibility_sum(slope: f64, floor: f64, e: f64) -> f64 {
    v3::clamp(slope * m::log(m::max(e, 1e-5) / floor), 0.0, 1.0)
}

/// the leaves' step and everything round it: who touches them, what they did to whom   (game.ts updateDoors)
pub fn game_update_doors(g: &mut crate::game::Game, dt: f64) {
    use crate::character::Bone;
    use crate::doors::DoorContact;
    use crate::game::GuardState;
    let seats = g.seats.clone(); let n = seats.len();
    g.door_owns.clear();
    let mut contacts: Vec<DoorContact> = Vec::new();
    // (ALIASING: in the TypeScript a contact's `pos`, `vel` ARE his arrays. Each contact's man here: read as the step opens, written back as it ends. `own_at`: the same object is in `owns`)
    let mut of: Vec<Option<CharId>> = Vec::new();
    let mut own_at: Vec<Option<usize>> = Vec::new();
    for i in 0..n {
        let s = seats[i];
        let own = in_seat(g, s, &mut |g| game_door_contact(g));
        g.door_owns.push(own.clone());
        if let Some(c) = own {
            let pl = crate::cmd::seat_man(g, s);
            own_at.push(Some(contacts.len())); of.push(Some(g.pl[pl].ch)); contacts.push(c);
        } else { own_at.push(None); }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i]; i += 1;
        if g.gd[gd].state == GuardState::Dead { continue; }
        let c = g.gd[gd].ch;
        let speed = g.gd[gd].speed;
        let bash = g.gd[gd].state == GuardState::Alert && crate::kitload::load_gait(g, crate::kitload::LoadGaitWho::Guard(gd), speed) > 2.2;
        let quiet = crate::silent::silent_ease(g, gd);
        let hand = crate::handleside::hand_press(g, gd);
        let round = speed > crate::doorway::ROUND_PACE && crate::doorway::rounds_leaves(g, gd) && !crate::doorway::leaf_stalled(g, gd);
        of.push(Some(c));
        contacts.push(DoorContact { pos: g.ch[c].pos, radius: 0.34, bash, who: g.ch[c].id, quiet, key: Some(true), hand, round: Some(round), ..DoorContact::zeroed() });
    }
    // (a copy of where a wounded man's BODY is: `at` is his contact's own pos, so what the leaf did to it is read from there. (the man, his contact, x, z))
    let mut wounded_at: Vec<(GuardId, usize, f64, f64)> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i]; i += 1;
        let Some(pose) = g.gd[gd].wound.as_ref().map(|w| w.pose) else { continue; };
        let c = g.gd[gd].ch; let p = g.ch[c].pos;
        let h = if pose == crate::wounds::WoundPose::Back { g.ch[c].bones.get(Bone::Hips).copied() } else { None };
        let at: V3 = match h { Some(h) => [h[0], p[1], h[2]], None => v3::copy(p) };
        wounded_at.push((gd, contacts.len(), at[0], at[2]));
        of.push(None);
        contacts.push(DoorContact { pos: at, radius: 0.3, bash: false, who: g.ch[c].id, quiet: true, weak: Some(true), ..DoorContact::zeroed() });
    }
    let n0 = contacts.len();
    crate::civilians::civilian_door_contacts(g, &mut contacts);
    let shown_civs = crate::civilians::civilians_shown(g);
    for k in n0..contacts.len() {   // (their `pos` is their array too: by whose it is)
        let who = contacts[k].who;
        of.push(shown_civs.iter().map(|gd| g.gd[*gd].ch).find(|c| g.ch[*c].id == who));
    }
    // bodies: a few of each ragdoll's particles as weak contacts. (the contact, the body, the particle)
    let mut body_contacts: Vec<(usize, CharId, f64)> = Vec::new(); let mut bodies: Vec<CharId> = Vec::new();
    let mut gi = -(n as isize);
    while gi < (g.guards.len() + shown_civs.len()) as isize {
        let at = gi; gi += 1;
        let ch = if at < 0 { let pl = crate::cmd::seat_man(g, seats[(at + n as isize) as usize]); g.pl[pl].ch }
            else if (at as usize) < g.guards.len() { g.gd[g.guards[at as usize]].ch }
            else { g.gd[shown_civs[at as usize - g.guards.len()]].ch };
        let Some(rag) = g.ch[ch].ragdoll else { continue; };
        let pelvis = crate::ragdoll::rag().pelvis;
        let mut tmp = g.tmp_v;
        let hp = crate::ragdoll::ragdoll_point(g, rag, pelvis, Some(&mut tmp));
        g.tmp_v = tmp;
        let list = g.doors.list.clone();
        let mut near = false;
        for &d in list.iter() { let p = crate::doors::door_pos(g, d); if m::abs(p[0] - hp[0]) < 2.2 && m::abs(p[2] - hp[2]) < 2.2 { near = true; break; } }
        if !near { continue; }
        bodies.push(ch);
        for i in crate::game::body_door_particles() {
            let p = crate::ragdoll::ragdoll_point(g, rag, i, None);
            let radius = crate::ragdoll::ragdoll_radius(g, rag, i);
            body_contacts.push((contacts.len(), ch, i));
            of.push(None);
            contacts.push(DoorContact { pos: p, radius, bash: false, who: g.ch[ch].id, quiet: true, weak: Some(true), ..DoorContact::zeroed() });
        }
    }
    let list = g.doors.list.clone();
    let mut angles: Vec<f64> = Vec::with_capacity(list.len());
    for &d in list.iter() { angles.push(crate::doors::door_angle(g, d)); }
    for &s in seats.iter() { let input = g.tab.cmd_seat[s].input; in_seat(g, s, &mut |g| game_door_acts(g, dt, input)); }
    if g.door_was.len() < 2 * n { g.door_was.resize(2 * n, 0.0); }
    for i in 0..n { let pl = crate::cmd::seat_man(g, seats[i]); let p = g.ch[g.pl[pl].ch].pos; g.door_was[2 * i] = p[0]; g.door_was[2 * i + 1] = p[2]; }
    // (…as the step opens: this frame's verbs have moved them)
    for k in 0..contacts.len() { if let Some(c) = of[k] { contacts[k].pos = g.ch[c].pos; if contacts[k].vel.is_some() { contacts[k].vel = Some(g.ch[c].vel); } } }
    crate::doors::doors_update(g, dt, &mut contacts);
    // (…and as it ends: where the leaves put them, and each player's own contact as the step left it)
    for k in 0..contacts.len() { if let Some(c) = of[k] { let p = contacts[k].pos; g.ch[c].pos[0] = p[0]; g.ch[c].pos[2] = p[2]; } }
    for i in 0..n { if let Some(k) = own_at[i] { g.door_owns[i] = Some(contacts[k].clone()); } }
    crate::doorthrow::door_throw_after(g, &contacts);
    for &(gd, k, x, z) in wounded_at.iter() {
        let mut at = contacts[k].pos;
        if at[0] == x && at[2] == z { continue; }
        crate::scene_collision::collide_circle(g, &mut at, 0.3, 0.2, 1.5, None); crate::scene_collision::clamp_to_world(g, &mut at, 0.3);
        let mut dx = at[0] - x; let mut dz = at[2] - z;
        let d = m::hyp2(dx, dz); let f = if d > 0.25 { 0.25 / d } else { 1.0 };
        dx *= f; dz *= f;
        let c = g.gd[gd].ch;
        let mut p = g.ch[c].pos; p[0] += dx; p[2] += dz;
        g.ch[c].pos = p;
        crate::scene_collision::collide_circle(g, &mut p, 0.3, 0.2, 1.5, None); g.ch[c].pos = p;
        crate::scene_collision::clamp_to_world(g, &mut p, 0.3); g.ch[c].pos = p;
        if d > 0.005 { crate::character::character_update(g, c, 0.0); }
    }
    crate::doorblast::door_slabs(g, dt);
    game_hands_on_doors(g, &angles);
    for i in 0..n {
        let s = seats[i];
        let pl = crate::cmd::seat_man(g, s); let at = g.ch[g.pl[pl].ch].pos;
        let mut own = g.door_owns[i].clone();
        if let Some(o) = own.as_mut() { o.pos = at; }   // (his array: wherever he now is)
        let (bx, bz) = (g.door_was[2 * i], g.door_was[2 * i + 1]);
        in_seat(g, s, &mut |g| game_door_after(g, own.as_mut(), bx, bz));
        g.door_owns[i] = own;
    }
    for &(k, ch, i) in body_contacts.iter() {
        let c = contacts[k].pos;
        let rag = g.ch[ch].ragdoll.unwrap();
        let at = i as usize;
        let (dx, dz) = (c[0] - g.tab.ragdoll[rag].x[at * 3], c[2] - g.tab.ragdoll[rag].x[at * 3 + 2]);
        crate::ragdoll::ragdoll_nudge(g, rag, i, dx, dz);
    }
    if !bodies.is_empty() {
        let list = g.doors.list.clone();
        for (k, &d) in list.iter().enumerate() {
            let slab = crate::doors::door_slab(g, d).filter(|s| !s.rest);
            if slab.is_none() && m::abs(crate::doors::door_angle(g, d) - angles[k]) < 1e-3 { continue; }
            let (hinge, width, height) = (g.dr[d].def.hinge, g.dr[d].def.width, g.dr[d].height);
            for &ch in bodies.iter() {
                let h = g.ch[ch].bones.get(Bone::Hips).copied().unwrap_or(g.ch[ch].pos);
                let rag = g.ch[ch].ragdoll.unwrap();
                let sleeping = g.tab.ragdoll[rag].sleeping;
                if sleeping && (m::hyp2(h[0] - hinge[0], h[2] - hinge[1]) < width + 1.2
                    || slab.as_ref().map_or(false, |s| m::hyp2(h[0] - s.p[0], h[2] - s.p[2]) < height / 2.0 + 0.8)) {
                    crate::ragdoll::ragdoll_wake(g, rag);
                }
            }
        }
    }
}

/// a seat's half of updateDoors, run over its view: the player's own contact on the leaves this frame (none: he is down)   (game.ts doorContact)
pub fn game_door_contact(g: &mut crate::game::Game) -> Option<crate::doors::DoorContact> {
    let pl = g.player; let pc = g.pl[pl].ch;
    if g.pl[pl].down { return None; }
    let run = crate::sprintdoor::sprint_door_run(g, pl);
    // (DOORWAY.solid, DOORWAY.oneWay: both true on the page)
    let solid = g.puppet.is_none() && !g.touring;
    Some(crate::doors::DoorContact {
        pos: g.ch[pc].pos, radius: 0.34, bash: g.pl[pl].sprinting, who: g.ch[pc].id, quiet: g.pl[pl].crouch && !g.pl[pl].dash,
        keys: Some(g.pl[pl].keys.clone()), run, solid: Some(solid), vel: Some(g.ch[pc].vel), ..crate::doors::DoorContact::zeroed()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    // the bits are the TypeScript's own, under bun with EXACT.on (the formulas of game.ts visibilityOf, viewPx, with vec.ts clamp)
    #[test]
    fn visibility_is_the_typescripts_bits() {
        for (s, f, e, bits) in [(0.37, 0.013, 0.2, 0x3ff0000000000000u64), (0.37, 0.013, 0.0, 0x0000000000000000), (0.5, 0.02, 0.0213, 0x3fa01f1dcedece09)] {
            assert_eq!(visibility_sum(s, f, e).to_bits(), bits);
        }
    }
    #[test]
    fn view_px_is_the_typescripts_bits() {
        for (d, fov, h, sharp, sc, bits) in [(6.5, 0.9, 1080.0, false, 0.75, 0x3f7fc1551f25ec42u64), (6.5, 0.9, 1080.0, true, 0.75, 0x3f830d997916c0f5), (4.25, 1.1, 150.0, false, 0.0, 0x3f9aaeadc2d0442a)] {
            assert_eq!(view_px_sum(d, fov, h, sharp, sc).to_bits(), bits);
        }
    }
}
