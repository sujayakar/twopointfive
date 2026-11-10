use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `s.view` handed to `f`. A game of one seat has no view: its seat's view IS the Game.
fn in_seat<R>(g: &mut Game, s: SeatId, mut f: impl FnMut(&mut Game) -> R) -> R {
    crate::seats::on_seat(g, Some(s), &mut f)
}

/// the character of a seat's man
fn man_char(g: &Game, s: SeatId) -> CharId { let pl = crate::cmd::seat_man(g, s); g.pl[pl].ch }

/// game.ts `WEAPONS[id]`
fn weapon_row(g: &Game, id: crate::weapons::WeaponId) -> &crate::weapons::WeaponDef { g.m.weapons.weapons.get(id).unwrap() }

/// a bone of his if it has been baked, else his feet raised by `up`
fn bone_or(g: &Game, c: CharId, b: crate::character::Bone, up: f64) -> V3 {
    match g.ch[c].bones.get(b) { Some(p) => *p, None => v3::add(g.ch[c].pos, [0.0, up, 0.0]) }
}

/// THE TICK, off the players' plain records. The order of these lines is the game.
pub fn game_step_seats(g: &mut crate::game::Game, dt: f64, cmds: &[crate::cmd::PlayerCmd]) {
    let n = g.seats.len();
    if cmds.len() != n { panic!("stepSeats: {} record(s) for {} seat(s)", cmds.len(), n); }
    // (the kit picture's stage, where one is up, has the hand before the tick and what it left after it)
    if g.kit_stage.is_some() {
        let mut held = cmds.to_vec();
        crate::kitstage::before(g, &mut held[0]);
        step_listed(g, dt, &held);
        crate::kitstage::after(g);
        return;
    }
    step_listed(g, dt, cmds);
}
fn step_listed(g: &mut crate::game::Game, dt: f64, cmds: &[crate::cmd::PlayerCmd]) {
    let n = g.seats.len();
    // (inside a tick the Game's own fields of a seat's name answer the FIRST seat's, whoever this host shows: seats.ts seatsStand)
    // (bare: no seat's view is open while the world's own lines run)
    // (the verdict's tick comes last, with the first seat still standing, so the end it latches is the same on every machine: verdict.ts tick)
    // (the tick opens by listing the box world, on every host at the same moment: this tick's rays meet the last tick's boxes and whatever was written since, and
    // the rows of the frames gone by are free again, after whoever showed the last frame has read it)
    crate::scene_boxes::box_world_listed(g);
    if n == 1 { step_seats_on_first(g, dt, cmds); crate::verdict::verdict_tick(g); game_tick_end(g, dt); return; }
    crate::seats::on_first_bare(g, &mut |g| { step_seats_on_first(g, dt, cmds); crate::verdict::verdict_tick(g); game_tick_end(g, dt); });
}

/// THE TICK'S LAST LINE: every flash's envelope for the frame, and its retirement. The next tick's light reads what it leaves, so it is a rule's state and the tick steps it, on every host at
/// the same moment, with no host's call round it.
fn game_tick_end(g: &mut crate::game::Game, dt: f64) {
    let now = g.time;
    crate::render_lights::light_set_update(g, dt, now);
}

/// The tick's lines, with the first seat on Game.
fn step_seats_on_first(g: &mut crate::game::Game, dt: f64, cmds: &[crate::cmd::PlayerCmd]) {
    let seats: Vec<SeatId> = g.seats.clone();
    let n = seats.len();
    g.ticking = true;
    crate::scene_collision::tick_open(g);
    // (game.ts:1591 `this.sim.settle(this)`: the die's move is use_dice below; the pool's tenant tends it: a man who left the lists is out of it, one who
    //  arrived is in)
    crate::posehome::pose_home_tend(g);
    crate::sense::smoke_home(g);
    // a record from anyone made one an honest seat could have sent, before anything reads it
    let mut checked: Vec<crate::cmd::PlayerCmd> = Vec::with_capacity(n);
    for i in 0..n {
        let s = seats[i];
        let near = if g.tab.cmd_seat[s].remote { let c = man_char(g, s); Some(g.ch[c].pos) } else { None };
        let r = crate::cmd::check_cmd(g, Some(&cmds[i]), near);
        if js::truthy(r.fixed) { g.tab.cmd_seat[s].fixed += r.fixed; g.tab.cmd_seat[s].fixed_cmds += 1.0; }
        checked.push(r.cmd);
    }
    crate::math_dice::use_dice(g);
    crate::quiet::quiet_mission_current(g);
    crate::sense::smoke_step(g, dt);
    for i in 0..n { crate::cmd::seat_load(g, seats[i], &checked[i]); }
    let (input, cam) = (g.tab.cmd_seat[seats[0]].input, g.tab.cmd_seat[seats[0]].cam);   // (what a ONE-PLAYER part of the tick reads is the first seat's)
    crate::rails::rails_gate(g, input);
    crate::opening::opening_start(g);
    if n > 1 { crate::seats::deal_seats(g); }
    for &s in &seats {
        let lens = g.tab.cmd_seat[s].cam;
        let vp = g.tab.camera[lens].view_proj;
        in_seat(g, s, |g| g.view_proj = Some(crate::game::GameViewProj::Float32Array(vp.to_vec())));
    }
    g.time += dt;
    g.ocp_cooldown = m::max(0.0, g.ocp_cooldown - dt);
    let now = g.time;
    crate::scene_collision::sound_paths_tick(g, now);
    crate::belief::duties_anew(g);   // what a man covers is said anew each tick
    // each seat's half of the tick, over that seat's view
    for &s in &seats {
        let (input, cam, canvas) = (g.tab.cmd_seat[s].input, g.tab.cmd_seat[s].cam, g.tab.cmd_seat[s].canvas);
        in_seat(g, s, |g| {
            crate::player::update_aim(g, input, cam, canvas);
            crate::orders::squad_orders_input(g, dt, input, cam, canvas);
            if crate::doortools::door_tools_menu_key(g, input) { let pl = g.player; g.pl[pl].throw_eaten = true; }
            crate::squadsmoke::squad_smoke_key(g, input);
            crate::wandorder::wand_order_key(g, input);
            crate::doortools::door_tools_big_key(g, input);
            if crate::squadthrow::squad_throw_key(g, input) { let pl = g.player; g.pl[pl].throw_eaten = true; }
            crate::orders::rows::frame(g);
            crate::orders::shown::keep(g, input, cam, canvas);
            crate::interact::update_interact(g, dt, input, Some(canvas), Some(cam));
            crate::aim::track_player(g, dt);
            crate::game::game_threat_tick(g);
        });
    }
    crate::suppression::update_suppression(g, dt);
    for &s in &seats {
        let (input, cam) = (g.tab.cmd_seat[s].input, g.tab.cmd_seat[s].cam);
        in_seat(g, s, |g| { crate::player::update_player(g, dt, input, cam); crate::samlook::sam_look_step(g); });
    }
    if !g.rings.is_empty() { crate::launcher::update_rings(g, dt); }
    for &s in &seats {
        let pl = crate::cmd::seat_man(g, s);
        if !g.pl[pl].cams.is_empty() { in_seat(g, s, |g| crate::stickycam::update_stickies(g, dt)); }
    }
    if !g.shells.is_empty() { crate::he40::update_shells(g, dt); }
    g.search_probe_n = 0.0;
    crate::game::game_deal_weapons(g, Some(dt));
    crate::director::director_tick(g, dt, None, None);
    for &s in &seats {
        let input = g.tab.cmd_seat[s].input;
        in_seat(g, s, |g| crate::pairorders::pair_orders_input(g, input));
    }
    for &s in &seats {
        let input = g.tab.cmd_seat[s].input;
        in_seat(g, s, |g| {
            crate::wallcharge::wall_charge_input(g, input);
            crate::doortools::door_tools_input(g, input);
            crate::wallcharge::wall_charge_input_after(g, input);
        });
    }
    crate::nightbreach::night_breach_tick(g, dt, input, Some(cam));
    crate::quiet::quiet_mission_tick(g, Some(cam));
    crate::enemysquads::squads_tick(g);
    crate::roomclear::room_clear_scenario_tick(g, dt);
    crate::rescue::rescue_tick(g, dt, input, Some(cam));
    crate::wallreact::wall_react_tick_2(g);
    crate::entrypoints::entry_points_tick(g);
    crate::cleared::cleared_rooms_tick(g);
    for &s in &seats { in_seat(g, s, |g| { crate::doortools::door_tools_tick(g); crate::wallcharge::wall_charge_tick(g); }); }
    for &s in &seats { in_seat(g, s, |g| crate::rearwatch::rear_watch_tick(g)); }
    crate::cover::cover_tick(g);
    for &s in &seats { in_seat(g, s, |g| crate::silent::silent_tick(g)); }
    crate::pairorders::pair_orders_tick(g, dt);
    for &s in &seats { in_seat(g, s, |g| crate::spotted::spotted_tick(g)); }
    for &s in &seats { in_seat(g, s, |g| crate::orders::squad_orders_tick(g)); }
    crate::doorpath::door_path_tick(g, dt);
    crate::powerout::power_out_tick(g, dt);
    crate::roomfight::room_fight_tick(g, dt);
    for &s in &seats { in_seat(g, s, |g| crate::showwhy::show_why_tick(g)); }
    crate::morale::morale_tick(g, dt);
    crate::flinch::flinch_tick(g, dt);
    crate::runner::runner_tick(g, dt);
    crate::darkdoor::dark_door_tick(g, dt);
    crate::netquiet::net_quiet_tick(g, dt);
    crate::lanes::lanes_tick(g, dt);
    crate::darkfoe::dark_foe_tick(g, dt);
    crate::nvg::nvg_tick(g);
    crate::tactics::tactics_tick(g, dt);
    crate::doze::doze_tick(g, dt);
    crate::close::close_seat_tick(g, dt);
    crate::needs::needs_tick(g, dt);
    crate::frag::frag_throw_tick(g, dt);
    crate::frag::frag_tick(g, dt);
    // (a live index: `for … of` over the list, which a man's update may grow)
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        crate::seats::standing_for(g, gd, &mut |g| crate::guards::update_guard(g, gd, dt));
        i += 1;
    }
    crate::picture::picture_tick(g, dt);
    crate::doorforce::door_force_tick(g, dt);
    crate::coverfire::coverfire_tick(g, dt);
    crate::civilians::update_civilians(g, dt);
    crate::guards::update_escalation(g, dt);
    crate::qrf::update_qrf(g, dt);
    crate::game::game_update_doors(g, dt);
    crate::lighting::lens_recheck(g);
    crate::irlight::ir_light_tick(g);
    in_seat(g, seats[0], |g| crate::mission::update_mission(g, dt, input));
    crate::par::par_tick(g);   // (right after the mission's update: this tick's orders and the room clear's end are in)
    crate::lesson::lesson_tick(g, dt);
    crate::rails::rails_tick(g, dt);
    crate::game::game_update_props(g, dt);
    crate::destruction::destruction_update(g, dt);
    crate::air_dust::air_dust_update(g, dt);
    let now = g.time;
    g.events.retain(|e| now - e.time < 0.5);
    crate::lightswitch::light_switch_tick(g);
    crate::lighting::update_targets(g, dt);
    crate::fire::update_fire(g, dt);
    crate::devices::update_devices(g, dt);
    crate::weather::weather_update(g, dt);
    crate::takedown::takedown_tick(g);
    for &s in &seats { in_seat(g, s, |g| crate::disarm::disarm_tick(g, dt)); }
    // (wet characters: the coats are the picture's, but the soak's ask stands the rule's copy of the box world and may cast the sky's rays: game.ts updateWetness)
    crate::wet::update_wetness(g, dt);
    // THE DRESSING ROWS WHOSE STEPS THE TICK ITSELF READS. The wind's sources are read by the moths' pull (`wind.at`), and
    // the moths are keyed boxes of the frame's list, which the fog of war's records, the frame's room and the kit's place in the list count: so both
    // run here under their rows, as in the TypeScript. The rows between them (the air's field, the snow's look, the particle rain, the wet ground's
    // words for the renderer) write what only the picture reads and stay with the host.
    if g.m.dress.dress.wind { crate::wind::wind_update(g, dt); }
    if g.m.dress.dress.moths { crate::moths::moths_update(g, dt); }
    crate::combat::update_items(g, dt);
    crate::glass::update_glass(g);
    crate::glassbreak::glass_break_update(g, dt);
    crate::shards::glass_shards_update(g, dt);
    crate::fogofwar::fog_of_war_update(g, dt);
    // characters: the seats' men, the guards, civilians last
    let mut all: Vec<CharId> = Vec::new();
    for &s in &seats { let c = man_char(g, s); all.push(c); }
    for j in 0..g.guards.len() { let gd = g.guards[j]; all.push(g.gd[gd].ch); }
    for gd in crate::civilians::civilians_shown(g) { all.push(g.gd[gd].ch); }
    let statured = crate::game::game_step_fx_and_dyn_boxes(g, &all, dt);
    crate::game::game_draw_characters(g, &all, statured);
    crate::smokebreak::update_smoke_break(g, dt);
    crate::game::game_light_meter(g, dt);
    // (every seat's list of things its men can work, before anybody's orders are read: what a point or a cursor is on is asked of it, inside a tick and between two)
    for &s in &seats { in_seat(g, s, |g| crate::workthings::work_things_keep(g)); }
    for &s in &seats {
        let pl = crate::cmd::seat_man(g, s);
        g.pl[pl].hit_flash = m::max(0.0, g.pl[pl].hit_flash - dt * 3.0);
        in_seat(g, s, |g| g.act = None);   // (this frame's Space verb has been read by everyone who reads it)
    }
    // (game.ts:1720, :1721: the squad's net and the defenders' cues, read off the settled frame. Their books are the same books the rules' own entries
    //  write (a line said, a cue called), so the step that settles them runs where those do, under its row.)
    if g.m.dress.dress.net { crate::squadnet::squad_net_tick(g); }
    if g.m.dress.dress.cues { crate::cues::cues_tick(g); }
    crate::teamtally::team_tally_tick(g);
    crate::deathrecap::recap_tick(g);   // (last: it reads the frame as it ended, and moves nothing a rule reads)
    crate::render_decals::marks_settle(g);   // (the marks' arms ask the world as the tick left it)
    g.ticking = false;
    crate::scene_collision::tick_shut(g);
}

/// threat proximity → how deliberately Sam moves
pub(super) fn game_threat_tick(g: &mut crate::game::Game) {
    use crate::character::Bone;
    use crate::game::GuardState;
    let pl = g.player; let pc = g.pl[pl].ch;
    let mut near: Option<GuardId> = None; let mut nd = f64::INFINITY; let mut t = 0.0;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i]; i += 1;
        if crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].held.is_some() || g.gd[gd].team == crate::teams::Team::Operative || crate::fogofwar::fog_of_war_hidden(g, gd) { continue; }
        let d = v3::dist_xz(g.ch[g.gd[gd].ch].pos, g.ch[pc].pos);
        if d < nd { nd = d; near = Some(gd); }
    }
    if let Some(nr) = near {
        if !crate::condition::down(g, g.pl[pl].ch) && g.gd[nr].state != GuardState::Alert {
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
                if crate::condition::down(g, g.gd[gd].ch) || !g.tab.light[fl].enabled || g.gd[gd].team == crate::teams::Team::Operative { continue; }
                let (fl_pos, fl_dir) = (g.tab.light[fl].pos, g.tab.light[fl].dir);
                let cos_o = m::cos(g.tab.light[fl].outer_deg * v3::DEG);
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

/// the muzzle of the stand-in gun in his hands
fn stand_in_muzzle(g: &Game, long: Option<crate::guards::LongRigOf>, cb: bool, def: crate::weapons::WeaponId) -> f64 {
    if long == Some(crate::guards::LongRigOf::Sc20k) { return crate::rig_props::sc20k_muzzle_ahead(None); }
    let row = weapon_row(g, def);
    if cb && row.rig != crate::weapons::WeaponDefRig::Carbine { return crate::rig_props::carbine_muzzle_ahead(None); }
    row.muzzle_ahead
}

/// the carbine dealt by the sandbox knob, and every man's animator armed with what is then in his hands
pub(super) fn game_deal_weapons(g: &mut crate::game::Game, dt: Option<f64>) {
    use crate::character::AnimLongGrip;
    use crate::game::{CarbineDeal, GuardState};
    use crate::guards::LongRigOf;
    use crate::teams::Team;
    use crate::weapons::{GunModelId, WeaponId};
    let dt = dt.unwrap_or(0.0);
    let deal = g.tune.carbine_guards;
    let mut grip: Option<AnimLongGrip> = None; let mut grip20k: Option<AnimLongGrip> = None;
    let mut grip_own: Vec<(GunModelId, Option<AnimLongGrip>, Option<f64>)> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i]; i += 1;
        let c = g.gd[gd].ch;
        if g.gd[gd].unit.is_none() { g.gd[gd].carbine = deal == CarbineDeal::All || (deal == CarbineDeal::Reyes && g.gd[gd].callsign == "Reyes"); }
        let route = g.gd[gd].route;
        let want = if g.gd[gd].carbine { WeaponId::Aks74u } else { g.tab.route[route].weapon.unwrap_or(WeaponId::Guard9mm) };
        if !crate::condition::down(g, g.gd[gd].ch) && g.fa[g.gd[gd].weapon].def != want { let row = weapon_row(g, want).clone(); let w = crate::weapons::firearm_new(g, &row); g.gd[gd].weapon = w; g.gd[gd].reload_t = -1.0; }
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
        {
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
        let (dead, now) = (crate::condition::down(g, g.gd[gd].ch), g.time);
        // (MOVES.on, page 1: the block always runs)
        {
            let w = g.gd[gd].weapon;
            let r = crate::weapons::firearm_reload_row(g, w);
            let reload_t = g.gd[gd].reload_t;
            let at = if dead || reload_t < 0.0 { -1.0 } else { r.dur - reload_t };
            let row = crate::character::ReloadBeat { dur: r.dur, out: r.out, in_: r.in_, rack: r.rack.map(Some), keep: None, hand: None, exact: r.exact };
            let an = &mut g.ch[c].anim;
            an.reload_row = Some(row);
            an.reload_at = at;
        }
        // (MOVES.on, page 1)
        let dip = g.k.moves.entry > 0.5 && cb && g.gd[gd].held.is_none() && !dead && team == Team::Operative
            && !crate::roomfight::room_fight_busy(g, gd, 1.5) && !crate::roomfight::room_fight_contact(g, gd) && crate::game::friend_before_muzzle(g, gd);
        g.ch[c].anim.dip = if dip { 1.0 } else { 0.0 };
        let from = g.gd[gd].last_known.unwrap_or(g.ch[c].pos);
        crate::condition::stun_pose(g, c, !dead, from);
        {
            let on = g.gd[gd].held.is_none();
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

/// this frame's dynamic boxes, and what steps while they are gathered. Returns whether stature scaled anybody
pub(super) fn game_step_fx_and_dyn_boxes(g: &mut crate::game::Game, all: &[CharId], dt: f64) -> bool {
    g.dyn_boxes.clear();
    // (the list is lent to the callees for the whole gathering: none of them reads g.dyn_boxes)
    let mut out = core::mem::take(&mut g.dyn_boxes);
    crate::fogofwar::fog_of_war_door_boxes(g, &mut out);
    crate::props::prop_system_boxes(g, &mut out);
    crate::throwables::throwables_boxes(g, &mut out);
    crate::destruction::destruction_boxes(g, &mut out);
    crate::devices::device_boxes(g, &mut out);
    if g.m.dress.dress.moths { crate::moths::moths_boxes(g, &mut out); }
    if !g.rings.is_empty() { crate::launcher::ring_boxes(g, &mut out); }
    let seats = g.seats.clone();
    for &s in seats.iter() {
        let pl = crate::cmd::seat_man(g, s);
        if !g.pl[pl].cams.is_empty() { in_seat(g, s, |g| crate::stickycam::sticky_boxes(g, &mut out)); }
    }
    crate::lightswitch::switch_boxes(g, &mut out);
    if !g.shells.is_empty() { crate::he40::shell_boxes(g, &mut out); }
    // STATURE: each man's drawn scale about his feet this frame; false with no cast loaded, and nothing below is asked
    let statured = g.m.dress.dress.stature && crate::stature::stature_tick(g, all, dt);
    let rig0 = out.len();
    crate::combat::hand_props(g, &mut out);
    g.fog_of_war.rig_from = rig0 as f64; g.fog_of_war.rig_to = out.len() as f64;
    // (game.ts:1816, whole and in its order: last row first, its box on the frame's list, then its time; with the DRESS row down a host that draws nothing
    //  keeps no list of them and no row's time is stepped)
    if g.m.dress.dress.fx {
        let mut i = g.fx.len();
        while i > 0 { i -= 1; out.push(g.fx[i].box_.into()); g.fx[i].ttl -= dt; if g.fx[i].ttl <= 0.0 { let gone = g.fx.remove(i); crate::scene_boxes::box_let_go(g, gone.box_); } }
    } else { for gone in core::mem::take(&mut g.fx) { crate::scene_boxes::box_let_go(g, gone.box_); } }
    // (A ROW OF `g.fx` IS THE ONLY HOLDER OF ITS BOX: the TypeScript drops the object with the row. Here the box is a row of the table, so as the fx row
    //  goes its box joins the frame's rows, and the next listing frees it once it is on no list: box_world_listed. Most fx rows live one frame
    //  (a charge's slab being placed, a stack's marker), so without this every such frame left a row for good.)
    // (game.ts:1818: his rig and a can in his fist are drawn where his drawn hands are: Box.dc / dh; the boxes themselves stay the simulation's)
    if statured { crate::stature::drawn_boxes(g, all, &out, rig0 as f64); }
    if !g.tune.tracers && js::truthy(g.tracers.live) { crate::tracers::tracers_clear(g); }
    crate::tracers::tracers_update(g, dt, &mut out);
    crate::sparks::sparks_update(g, dt);
    crate::sparks::sparks_boxes(g, &mut out);
    // (game.ts:1822: spent cases heard landing when they are due: the list is the tick's own (a shot adds to it), so the step that empties it is here too)
    crate::particle_fx::update_particle_fx(g);
    for &s in seats.iter() { in_seat(g, s, |g| crate::blast_look::blast_look_tick(g, &mut out)); }
    // (game.ts:1824: the feel of a wall charge going off; what of it is the tick's is blast_feel's own head)
    if g.m.dress.dress.blast_feel { for &s in seats.iter() { let cam = g.tab.cmd_seat[s].cam; in_seat(g, s, |g| crate::blast_feel::blast_feel_tick(g, cam)); } }
    // (game.ts:1825: a fixture that broke, LAST in the list: keyed boxes of the frame's list, fixture_break's own head)
    if g.m.dress.dress.fixtures { crate::fixture_break::fixture_break_boxes(g, &mut out, dt); }
    g.dyn_boxes = out;
    statured
}

/// characters → what each man wears (the renderer's own half is the picture's), and the frame's dynamic boxes handed to the world
pub(super) fn game_draw_characters(g: &mut crate::game::Game, all: &[CharId], statured: bool) {
    game_draw_men(g, all, statured, Some(false));   // (DRESS.men down: dress.ts, "what each WEARS is still dealt")
    let mut list = core::mem::take(&mut g.dyn_boxes);
    crate::fogofwar::picture::keep_raw(g, &list);
    let dynamics = crate::fogofwar::fog_of_war_things(g, &mut list);
    g.dyn_boxes = list;
    g.engine.world.dynamics = dynamics;
}

/// the world size of one drawn pixel at the look-at depth of the shown seat's view. A drawer's: no rule reads it
pub fn game_shown_px(g: &crate::game::Game) -> f64 {
    let seat = &g.tab.cmd_seat[game_host_seat(g)];
    let cam = &g.tab.camera[seat.cam];
    view_px_sum(cam.distance, cam.fov, seat.canvas.height)
}
/// its sum
fn view_px_sum(distance: f64, fov: f64, height: f64) -> f64 { 2.0 * distance * m::tan(fov / 2.0) / m::max(200.0, height) }

#[cfg(test)]
mod tests {
    use super::*;
    // the bits are the TypeScript's own, under bun with EXACT.on
    #[test]
    fn view_px_is_the_typescripts_bits() {
        assert_eq!(view_px_sum(4.25, 1.1, 150.0).to_bits(), 0x3f9aaeadc2d0442a);
    }
}
