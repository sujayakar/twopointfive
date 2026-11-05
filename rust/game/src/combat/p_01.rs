//! src/game/combat.ts lines 1 to 342
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ------------------------------------------------------------ the seam: what skeleton 4 still cannot give

/// what the round wrote on its shell goes back to the shell the caller holds (`o` is lent read-only, FireOpts.pellet is by value)
fn seam_shell_back(g: &mut Game, o: &FireOpts, shell: crate::shotgun::Shell) { todo!("needs: FireOpts.pellet that writes through (a ShellId, or &mut FireOpts)") }

// ------------------------------------------------------------ shooting

/// `g.engine.lights.flash(…)`: the set is lent out of g for the call (LightSet's methods take g and the set) and goes back with the new light.
fn flash(g: &mut Game, pos: V3, color: V3, intensity: f64, range: f64, ttl: f64, owner: f64) -> LightId {
    let mut set = core::mem::take(&mut g.engine.lights);
    let l = crate::render_lights::light_set_flash(g, &mut set, pos, color, intensity, range, ttl, owner);
    g.engine.lights = set;
    l
}

/// The Firearm this character carries: the player's gun in hand, else his pistol; a guard's weapon; a stray's 9 mm. (combat.ts firearmOf)
pub fn firearm_of(g: &mut crate::game::Game, shooter: CharId, is_player: bool) -> FirearmId {
    if is_player {
        let p = g.player;
        return match crate::game::player_gun_in_hand(g, p) { Some(f) => f, None => g.pl[p].pistol };
    }
    if let Some(x) = guard_of_char(g, shooter) { return g.gd[x].weapon; }
    if let Some(f) = g.m.combat.stray_gun { return f; }
    let row = weapon_row(g, crate::weapons::WeaponId::Guard9mm);
    let f = crate::weapons::firearm_new(g, &row);
    g.m.combat.stray_gun = Some(f);
    f
}

/// a thing run in the game as this seat has it: with_seat, or as it stands when that seat is already on Game (viewOfBody's "a view handed in is kept")
fn in_seat(g: &mut Game, seat: PlayerId, run: &mut dyn FnMut(&mut Game)) {
    if seat == g.player { run(g); } else { crate::seats::with_seat(g, seat, run); }
}

/// the weapon's row by its key (a Firearm holds the key; WEAPONS the row)
fn weapon_row(g: &Game, id: crate::weapons::WeaponId) -> crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(id).unwrap().clone()
}

/// g.guards.find(x => x.char === c)
fn guard_of_char(g: &Game, c: CharId) -> Option<GuardId> {
    g.guards.iter().copied().find(|&x| g.gd[x].ch == c)
}

/// Would the old near-is-a-hit rule have broken this fixture on these legs? A count for the checks. (combat.ts nearLamp)
pub fn near_lamp(g: &mut crate::game::Game, t: FixtureId, legs: &[crate::aimcost::RoundLeg], slack: f64) -> bool {
    use crate::sim_lightshome::FixtureKind;
    let kind = g.tab.sim_lightshome_fixture[t].kind;
    let pos = g.tab.sim_lightshome_fixture[t].pos;
    let rad = if kind == FixtureKind::Emissive { 0.45 } else if kind == FixtureKind::Breaker { 0.4 } else { 0.3 };
    for s in legs {
        let to_t = v3::sub(pos, s.o);
        let depth = v3::dot(to_t, s.d);
        if depth > s.end + slack { continue; }
        if v3::len(v3::sub(to_t, v3::scale(s.d, v3::clamp(depth, 0.0, s.end)))) < rad { return true; }
    }
    false
}

/// A guard's own hands: the row's spread off the man he is firing at. (combat.ts aiWide)
pub fn ai_wide(g: &mut crate::game::Game, sg: Option<GuardId>, shooter: CharId, def: &crate::weapons::WeaponDef) -> f64 {
    let foe = match sg { Some(s) => crate::teams::foe_char(g, s), None => None };
    let tc = match foe { Some(c) => c, None => g.pl[g.player].ch };
    let tv = m::hyp2(g.ch[tc].vel[0], g.ch[tc].vel[2]);
    let range = v3::dist(g.ch[shooter].pos, g.ch[tc].pos);
    let s = def.ai_spread;
    s.base + m::min(s.move_cap, tv * s.move_k) + m::max(0.0, range - s.range_from) * s.range_k
}

/// One round from the shooter's muzzle at `target`, in the game as he has it. (combat.ts fireWeapon)
pub fn fire_weapon(g: &mut crate::game::Game, shooter: CharId, target: V3, is_player: bool, o: Option<&crate::combat::FireOpts>) {
    let none = FireOpts::default();
    let o = o.unwrap_or(&none);
    let id = g.ch[shooter].id;
    let seat = crate::seats::view_of_body(g, crate::seats::ViewOfBodyC { id });
    in_seat(g, seat, &mut |g: &mut Game| {
        let who = g.ch[shooter].id;
        let was = crate::fogofwar::fog_of_war_begin(g, Some(who));
        fire_weapon_raw(g, shooter, target, is_player, o);
        crate::winbars::bars_unmeet(g);
        crate::fogofwar::fog_of_war_done(g, was);
    });
}

/// The round's flight in legs: a RoundPath, with the walls it crossed when wallpen flew it. (`RoundPath & { walls? }`)
struct Pen {
    path: crate::glasspen::RoundPath,
    walls: Option<Vec<crate::wallpen::WallCross>>,
}

fn st_of(h: crate::scene_collision::Raycast) -> crate::glasspen::RoundPathSt {
    crate::glasspen::RoundPathSt { t: h.t, index: h.index, n: h.n }
}

/// `fly` of fireWeapon: the same flight over panes and thin things alike, from what st0 is at this call.
fn fly(g: &mut Game, wall_live: bool, pen_live: bool, muzzle: V3, dir: V3, e_muzzle: f64, st0: Option<crate::glasspen::RoundPathSt>, arm_panes: f64) -> Option<Pen> {
    if wall_live {
        let first = st0.map(|s| crate::wallpen::StaticHit2 { t: s.t, index: s.index, n: s.n });
        return crate::wallpen::thin_path(g, muzzle, dir, 60.0, e_muzzle, first, Some(arm_panes)).map(|p| Pen {
            path: crate::glasspen::RoundPath { segs: p.segs, cross: p.cross, st: p.st, len: p.len },
            walls: Some(p.walls),
        });
    }
    if pen_live {
        let first = st0.map(|s| crate::glasspen::StaticHit2 { t: s.t, index: s.index, n: s.n });
        return crate::glasspen::round_path(g, muzzle, dir, 60.0, e_muzzle, first, Some(arm_panes)).map(|p| Pen { path: p, walls: None });
    }
    None
}

/// A Shell handed on as the ShellGlass it is in the TypeScript: its `glass` is lent for the call and comes back with what the call wrote.
fn with_glass<R>(pel: &mut Option<crate::shotgun::Shell>, run: impl FnOnce(Option<&mut crate::glasspen::ShellGlass>) -> R) -> R {
    match pel {
        None => run(None),
        Some(p) => {
            let mut lent = crate::glasspen::ShellGlass { glass: core::mem::take(&mut p.glass) };
            let r = run(Some(&mut lent));
            p.glass = lent.glass;
            r
        }
    }
}

/// `fx.muzzleFx[key]`
fn muzzle_row(key: crate::effects::MuzzleFxKey) -> crate::effects::MuzzleFx {
    use crate::effects::MuzzleFxKey as K;
    let t = crate::effects::muzzle_fx();
    match key {
        K::Mk12 => t.mk12,
        K::Mp7 => t.mp7,
        K::Mk46 => t.mk46,
        K::M1014 => t.m1014,
        K::Hk45 => t.hk45,
        K::G18 => t.g18,
        K::Carbine => t.carbine,
        K::Sc20k => t.sc20k,
        K::PistolSuppressed => t.pistol_suppressed,
        K::PistolBare => t.pistol_bare,
    }
}

/// What `scan` of fireWeapon closes over and does not change once the scan starts.
struct ScanOf<'a> {
    pen: Option<&'a Pen>,
    shooter: CharId,
    muzzle: V3,
    dir: V3,
    st0: Option<crate::glasspen::RoundPathSt>,
    arm_panes: f64,
}

/// the first leg's contact: a man of the shooter's own side, the entering hit only; else fireWeapon's `contact`, which body_hit asks only once the muzzle is inside him
fn first_leg_contact(f: &ScanOf, own: bool) -> BodyHitContact {
    if own { return BodyHitContact::Entering; }
    BodyHitContact::Muzzle { st0_t: f.st0.map(|s| s.t), muzzle: f.muzzle, glass_arm: f.arm_panes > 0.0, shooter: f.shooter }
}

/// Where the round enters this man, in metres of its path, or -1. `own`: a man of the shooter's own side, the entering hit only. (fireWeapon's scan)
fn scan(g: &mut Game, f: &ScanOf, t_hit: f64, c: CharId, own: bool) -> f64 {
    let Some(pen) = f.pen else {
        return body_hit(g, c, f.muzzle, f.dir, t_hit, Some(first_leg_contact(f, own)));
    };
    for k in 0..pen.path.segs.len() {
        let s = pen.path.segs[k];
        if s.t0 >= t_hit { break; }
        let contact = if k == 0 { first_leg_contact(f, own) } else { BodyHitContact::LegCrown };
        let t = body_hit(g, c, s.o, s.d, m::min(s.len, t_hit - s.t0), Some(contact));
        if t >= 0.0 { return s.t0 + t; }
    }
    -1.0
}

/// What a round in a man does: at once for a single round, once the whole pattern has flown for a pellet. (fireWeapon's manDown and samHit, the rows of Shell.late)
fn run_shell_late(g: &mut Game, late: &crate::shotgun::ShellLate) {
    match *late {
        crate::shotgun::ShellLate::ManDown { hit_guard, sg, shooter, is_player, ff_ours, hostage_hit, muzzle, dir, t_hit, hit_p } =>
            man_down(g, hit_guard, sg, shooter, is_player, ff_ours, hostage_hit, muzzle, dir, t_hit, hit_p),
        crate::shotgun::ShellLate::SamHit { hit_view, sg, dir, hit_p } => sam_hit(g, hit_view, sg, dir, hit_p),
    }
}

/// The man the round found: where it landed decides, then the log's line, the sound, whose round it was. (fireWeapon's manDown)
fn man_down(g: &mut Game, hit_guard: GuardId, sg: Option<GuardId>, shooter: CharId, is_player: bool, ff_ours: bool, hostage_hit: bool, muzzle: V3, dir: V3, t_hit: f64, hit_p: V3) {
    use crate::game::{DownBy, DownKind, GuardState};
    use crate::wounds::RoundDoes;
    let was_up = g.gd[hit_guard].state != GuardState::Dead;
    if was_up && g.gd[hit_guard].civ.is_some() { crate::snatch::snatch_round_on(g, hit_guard, sg); }
    let team = g.gd[hit_guard].team;
    let foe = match sg { Some(s) => { let st = g.gd[s].team; crate::teams::is_hostile(g, st, team) } None => false };
    let sign = g.gd[hit_guard].callsign.clone();
    let by_sign = sg.map(|s| g.gd[s].callsign.clone());
    let by = by_sign.as_deref().unwrap_or("");
    let foe_line = if foe { Some(alloc::format!("{} down — {}'s round", sign, by)) } else { None };
    let own_hit = ff_ours && g.gd[hit_guard].civ.is_none() && !crate::teams::is_hostile(g, crate::teams::Team::Player, team);
    let ro = if v3::dist(v3::mad(muzzle, dir, t_hit), hit_p) < 1e-6 { muzzle } else { v3::mad(hit_p, dir, -t_hit) };
    let does = crate::wounds::round_does(g, hit_guard, ro, dir, t_hit, hostage_hit, is_player);
    if does != RoundDoes::None {
        let line: Option<String> = if does != RoundDoes::Shot {
            Some(if foe_line.is_some() {
                alloc::format!("{} hit, down and breathing — {}'s round", sign, by)
            } else if own_hit {
                if is_player { alloc::format!("{} hit, down and breathing — your round", sign) } else { alloc::format!("{} hit, down and breathing — {}'s round", sign, by) }
            } else {
                alloc::format!("{} down — wounded, out of the fight", sign)
            })
        } else if foe_line.is_some() {
            foe_line
        } else if own_hit && !is_player {
            Some(alloc::format!("{} down — {}'s round", sign, by))
        } else {
            None
        };
        let zone = match does { RoundDoes::Side => Some(KillGuardZone::Side), RoundDoes::Leg => Some(KillGuardZone::Leg), _ => None };
        let how = if does == RoundDoes::Shot { DownKind::Shot } else { DownKind::Wounded };
        let by_who = if is_player { DownBy::Player } else { DownBy::Guard };
        kill_guard(g, hit_guard, dir, Some(false), Some(how), Some(by_who), line.as_deref(), zone);
    }
    crate::audio_iface::game_audio_like_play(g, "bodyHit", Some(hit_p), Some(0.8), None);
    if was_up { g.gd[hit_guard].killer_id = g.ch[shooter].id; }
    if hostage_hit {
        if let Some(sh) = sg {
            if g.gd[sh].state != GuardState::Dead { crate::guards::hostage_hit_bark(g, sh, hit_guard); }
        }
    }
}

/// A round in a player's man: the flinch, the flash, lock work ended, and the hit unless god mode. (fireWeapon's samHit)
fn sam_hit(g: &mut Game, pl: PlayerId, sg: Option<GuardId>, dir: V3, hit_p: V3) {
    let c = g.pl[pl].ch;
    crate::character::anim_hit_from(g, c, Some(dir));
    g.pl[pl].hit_flash = 1.0;
    if g.pl[pl].picking.is_some() { g.pl[pl].picking = None; g.pl[pl].f_held = true; }
    if g.pl[pl].wanding.is_some() { crate::wand::wand_drop(g, pl); g.pl[pl].f_held = true; }
    crate::audio_iface::game_audio_like_play(g, "bodyHit", Some(hit_p), Some(0.9), None);
    if !g.god_mode {
        in_seat(g, pl, &mut |g: &mut Game| hit_player(g, dir));
        let by = sg.map(|s| g.gd[s].callsign.clone());
        crate::chronicle::chron_hit(g, by.as_deref());
    }
}

/// A shot-out light: the breaker's blackout, a dark lens's glass, or a fixture gone. (fireWeapon's shootOut)
fn shoot_out(g: &mut Game, t: FixtureId, muzzle: V3) {
    use crate::sim_lightshome::FixtureKind;
    let kind = g.tab.sim_lightshome_fixture[t].kind;
    let pos = g.tab.sim_lightshome_fixture[t].pos;
    let break_ = crate::sim_lightshome::light_in().break_;
    if kind == FixtureKind::Breaker {
        flash(g, pos, [0.6, 0.7, 1.0], 10.0, 5.0, 0.2, 0.0);
        crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(1.0), None);
        crate::game::game_set_blackout(g, f64::INFINITY, crate::game::GameSetBlackoutCause::Shot);
        crate::sim_lightshome::light_home_in(g, break_, Some(t), None, None, None, None);
        crate::particle_fx::breaker_spark_fx(g, pos, muzzle);
        return;
    }
    if crate::carlamps::lamp_glass(g, t) { g.stats.carlamps.shot += 1; }
    let name = g.tab.sim_lightshome_fixture[t].name.clone();
    let lens = g.tab.sim_lightshome_fixture[t].lens;
    if crate::carlamps::dead_lamp(g, t) {
        crate::sim_lightshome::light_home_in(g, break_, Some(t), None, None, None, None);
        crate::game::game_msg(g, &alloc::format!("shot out {}", name));
        crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(0.8), None);
        crate::particle_fx::lens_shatter_fx(g, crate::particle_fx::LensShatterFxT { pos, lens }, muzzle);
        return;
    }
    crate::sim_lightshome::light_home_in(g, break_, Some(t), None, None, None, None);
    crate::game::game_msg(g, &alloc::format!("shot out {}", name));
    crate::game::game_light_event(g, t, None, None);
    crate::chronicle::chron_light_shot(g, t);
    flash(g, pos, [0.7, 0.8, 1.0], 6.0, 4.0, 0.12, 0.0);
    crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(1.0), None);
    if crate::carlamps::lamp_glass(g, t) {
        crate::particle_fx::lens_shatter_fx(g, crate::particle_fx::LensShatterFxT { pos, lens }, muzzle);
    } else if kind != FixtureKind::Device {
        let boxes = g.tab.sim_lightshome_fixture[t].boxes.clone();
        crate::particle_fx::fixture_shatter_fx(g, &crate::particle_fx::FixtureShatterFxT { pos, kind: String::from(kind.as_str()), boxes }, muzzle);
    }
}

/// `live` of the real-hit search: a target a round can put out.
fn lamp_live(g: &mut Game, t: FixtureId) -> bool {
    !(crate::sim_lightshome::fixture_broken(g, t)
        || !g.tab.sim_lightshome_fixture[t].interactive
        || crate::sim_lightshome::fixture_factor(g, t) <= 0.0 && !crate::sim_lightshome::fixture_on(g, t) && !crate::carlamps::lamp_glass(g, t)
        || crate::fire::fire_holds(g, t))
}

/// `own` of the real-hit search: the breaker's target lists its LED alone, a round into the panel is a round into the breaker.
fn lamp_own(g: &Game, t: FixtureId) -> Vec<f64> {
    if g.tab.sim_lightshome_fixture[t].kind == crate::sim_lightshome::FixtureKind::Breaker { g.level.breaker.boxes.clone() } else { g.tab.sim_lightshome_fixture[t].boxes.clone() }
}

/// The round itself, the shooter's muzzle to where it stops: what it found, what it broke, what the floor heard. (combat.ts fireWeaponRaw)
pub fn fire_weapon_raw(g: &mut crate::game::Game, shooter: CharId, target: V3, is_player: bool, o: &crate::combat::FireOpts) {
    use crate::game::GuardState;
    use crate::glasspen::{path_at, seg_at, spent_in_pane};
    use crate::sim_lightshome::FixtureKind;

    let sg = if is_player { None } else { guard_of_char(g, shooter) };
    if let Some(s) = sg { if g.gd[s].work_stow.is_some() { crate::workstow::stow_fired(g, s); } }
    if !is_player && g.ch[shooter].anim.big_tool.is_some() { crate::bigkit::big_fired(g, shooter); }
    let sg_team = if is_player { crate::teams::Team::Player } else { match sg { Some(s) => g.gd[s].team, None => crate::teams::Team::Guard } };
    let def: crate::weapons::WeaponDef = match &o.weapon { Some(d) => d.clone(), None => { let f = firearm_of(g, shooter, is_player); let id = g.fa[f].def; weapon_row(g, id) } };
    let mut spread = o.spread.unwrap_or(0.0);
    let steady = o.steady.unwrap_or(false);
    let mut pel: Option<crate::shotgun::Shell> = o.pellet.clone();
    let pellets = def.pellets;
    let who = g.ch[shooter].id;

    let ff_on = crate::friendlyfire::ff_live(g);
    let ff_ours = ff_on && (is_player || crate::friendlyfire::ff_side(g, sg));
    if ff_ours && !is_player && pel.is_none() {
        let wide = if steady { 0.0 } else { ai_wide(g, sg, shooter, &def) };
        let cone = match &pellets { Some(p) => p.cone * 2.0, None => 0.0 };
        let at = g.ch[shooter].muzzle;
        if crate::friendlyfire::ff_unpressed(g, sg, at, target, spread + wide + cone, &def) { return; }
    }
    if pellets.is_some() && pel.is_none() { crate::shotgun::fire_shell(g, shooter, target, is_player, o, &def); return; }
    let lead = match &pel { None => true, Some(p) => p.i == 0.0 };
    let third = match &pel { None => true, Some(p) => js::rem(p.i, 3.0) == 0.0 };
    if lead { crate::character::anim_fire(g, shooter); }
    if is_player && lead { crate::mission::tally(g, crate::mission::TallyK::Shots); crate::holdfire::hold_fire_shot(g, shooter); }
    let muzzle = g.ch[shooter].muzzle;
    let mut dir = v3::normalize(v3::sub(target, muzzle));
    if !is_player && !steady { spread += ai_wide(g, sg, shooter, &def); }
    if spread > 0.0 {
        let rx = crate::math_dice::sim_random(g);
        let ry = crate::math_dice::sim_random(g);
        let rz = crate::math_dice::sim_random(g);
        dir = v3::normalize([dir[0] + (rx - 0.5) * spread, dir[1] + (ry - 0.5) * spread, dir[2] + (rz - 0.5) * spread]);
    }
    if lead { if let Some(p) = &mut pel { p.dir = Some(dir); } }
    // the flash card: one frame of emissive box
    let flame = crate::gunlook::flame_of(g, &def);
    let mf = muzzle_row(flame);
    if lead {
        let card = crate::effects::muzzle_card();
        let bf = crate::scene_boxes::box_flag();
        let b = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
            c: v3::mad(muzzle, dir, mf.card_ahead),
            h: mf.card_half,
            yaw: Some(v3::yaw_of(dir)),
            albedo: Some(card.albedo),
            emissive: Some(mf.card_emissive),
            flags: Some((js::to_i32(bf.dynamic) | js::to_i32(bf.no_shadow)) as f64),
            ..Default::default()
        });
        g.fx.push(crate::game::GameFxItem { box_: b, ttl: card.ttl });
    }
    // gas, light and powder: the shot from effects.ts
    let gpu = crate::particle_fx::gpu_fx(g).is_some();
    if lead {
        let seed = g.fx_seed;
        g.fx_seed += 1.0;
        let gain = mf.light_gain * (if def.suppressed { crate::effects::shot().suppressed_gain } else { 1.0 });
        let gas = !crate::fogofwar::fog_of_war_air_hides(g, shooter);
        let lights = crate::effects::spawn_shot(g, muzzle, dir, &crate::effects::SpawnShotO {
            seed,
            light_gain: Some(gain),
            light_range: mf.light_range,
            track: Some(crate::effects::Track::Muzzle { by: shooter }),
            eject: Some(crate::effects::Track::EjectPort { by: shooter }),
            gas: Some(gas),
        });
        for l in lights {
            flash(g, l.pos, l.color, l.intensity, l.range, l.ttl, who);
        }
    }
    if gpu && lead { crate::particle_fx::muzzle_sparks_fx(g, muzzle, dir, shooter); crate::particle_fx::casing_fx(g, shooter, &def); }
    if lead { let voice = crate::gunlook::voice_of(g, &def); crate::audio_iface::game_audio_like_play(g, &voice, Some(muzzle), Some(1.0), None); }

    // --- hit scan
    let mut t_hit = 60.0;
    let mut hit_guard: Option<GuardId> = None;
    let mut player_hit = false;
    let mut hit_view: PlayerId = g.player;
    let mut st0 = crate::scene_collision::raycast(g, muzzle, dir, t_hit).map(st_of);
    if let Some(s) = st0 { t_hit = s.t; }
    let mut st = st0;
    let pen_live = crate::glasspen::glass_pen_live(g);
    let wall_live = crate::wallpen::wall_pen_live(g);
    let e0 = if pen_live || wall_live { crate::glasspen::round_energy(g, &def) } else { 1.0 };
    let mut e_k = 1.0;
    let mut through = arm_through(g, shooter);
    let bar_live = crate::winbars::bars_live(g);
    let mut bars: Option<Vec<crate::winbars::BarMet>> = None;
    if through && bar_live {
        if let Some(a) = across_arm(g, shooter) {
            if crate::winbars::bar_at(g, a.index) {
                let met = crate::winbars::bar_meet(g, a.index, 0.0, a.at, dir);
                bars = Some(vec![met]);
                through = arm_through(g, shooter);
                st0 = crate::scene_collision::raycast(g, muzzle, dir, 60.0).map(st_of);
                t_hit = match st0 { Some(s) => s.t, None => 60.0 };
                st = st0;
            }
        }
    }
    let mut e_muzzle = e0;
    let mut arm_panes = 0.0;
    if through && pen_live {
        let arm = with_glass(&mut pel, |sh| crate::glasspen::arm_through_glass(g, shooter, dir, e0, sh));
        if let Some(arm) = arm { through = false; e_muzzle = arm.e; arm_panes = arm.panes; }
    }
    if through { t_hit = 0.0; }
    let mut pen = if through { None } else { fly(g, wall_live, pen_live, muzzle, dir, e_muzzle, st0, arm_panes) };
    if bar_live && !through {
        let mut n = match &bars { Some(b) => b.len() as f64, None => 0.0 };
        while n < g.k.winbars.per_round {
            let stop = match &pen { Some(p) => p.path.st, None => st0 };
            let Some(stop) = stop else { break };
            if !crate::winbars::bar_at(g, stop.index) { break; }
            let at = match &pen { Some(p) => path_at(&p.path, stop.t), None => v3::mad(muzzle, dir, stop.t) };
            let d = match &pen { Some(p) => seg_at(&p.path, stop.t).d, None => dir };
            let met = crate::winbars::bar_meet(g, stop.index, stop.t, at, d);
            bars.get_or_insert_with(Vec::new).push(met);
            st0 = crate::scene_collision::raycast(g, muzzle, dir, 60.0).map(st_of);
            pen = fly(g, wall_live, pen_live, muzzle, dir, e_muzzle, st0, arm_panes);
            n += 1.0;
        }
        if bars.is_some() { st = st0; t_hit = match st0 { Some(s) => s.t, None => 60.0 }; }
    }
    if let Some(p) = &pen { t_hit = p.path.len; st = p.path.st; }
    let arm_at = if through && pel.is_some() { across_arm(g, shooter) } else { None };
    match arm_at {
        Some(a) if pen_live => {
            if a.index < 1e6 { with_glass(&mut pel, |sh| spent_in_pane(g, a.index, a.at, dir, e0, who, sh)); }
        }
        _ => { if through && lead { glass_across_arm(g, shooter, dir); } }
    }
    if through && third { wall_across_arm(g, shooter, &def); }
    // loose furniture in the line of fire: knocked below; it stops the round only with nobody behind it
    let mut ph = if through || pen.is_some() { None } else { crate::props::prop_system_raycast(g, muzzle, dir, t_hit) };
    if let Some(p) = &pen {
        for s in &p.path.segs {
            if let Some(mut h) = crate::props::prop_system_raycast(g, s.o, s.d, s.len) {
                h.t = s.t0 + h.t;
                ph = Some(h);
                break;
            }
        }
    }
    let f = ScanOf { pen: pen.as_ref(), shooter, muzzle, dir, st0, arm_panes };
    let held = g.pl[g.player].holding.as_ref().map(|h| h.g);
    let mut hostage_hit = false;
    if through {
        // into the panel at the muzzle: nobody beyond it
    } else if is_player {
        let mut i = 0;
        while i < g.guards.len() {
            let gd = g.guards[i];
            i += 1;
            let team = g.gd[gd].team;
            let own = !crate::teams::is_hostile(g, crate::teams::Team::Player, team);
            if Some(gd) == held || (own && !(ff_on && crate::friendlyfire::ff_body(g, gd))) { continue; }
            let c = g.gd[gd].ch;
            let t = scan(g, &f, t_hit, c, own);
            if t >= 0.0 && t < t_hit { t_hit = t; hit_guard = Some(gd); }
        }
        for cv in crate::civilians::civilian_bodies(g) {
            let c = g.gd[cv].ch;
            let t = scan(g, &f, t_hit, c, false);
            if t >= 0.0 && t < t_hit { t_hit = t; hit_guard = Some(cv); }
        }
    } else {
        let mut t_h = -1.0;
        if let Some(h) = held {
            if g.gd[h].state != GuardState::Dead && g.gd[h].ch != shooter {
                let c = g.gd[h].ch;
                t_h = scan(g, &f, t_hit, c, false);
            }
        }
        let t = if crate::teams::is_hostile(g, sg_team, crate::teams::Team::Player) { let c = g.pl[g.player].ch; scan(g, &f, t_hit, c, false) } else { -1.0 };
        if t_h >= 0.0 && (t < 0.0 || t_h < t) { t_hit = t_h; hit_guard = held; hostage_hit = true; }
        else if t >= 0.0 { t_hit = t; player_hit = true; }
        if g.seats.len() > 1 && crate::teams::is_hostile(g, sg_team, crate::teams::Team::Player) {
            let mut i = 0;
            while i < g.seats.len() {
                let s = g.seats[i];
                i += 1;
                let pl = crate::cmd::seat_man(g, s);
                if pl == g.player || g.pl[pl].down { continue; }
                let c = g.pl[pl].ch;
                let to = scan(g, &f, t_hit, c, false);
                if to >= 0.0 && to < t_hit { t_hit = to; player_hit = true; hit_guard = None; hostage_hit = false; hit_view = pl; }
            }
        }
        let mut i = 0;
        while i < g.guards.len() {
            let od = g.guards[i];
            i += 1;
            let c = g.gd[od].ch;
            let team = g.gd[od].team;
            if Some(od) == held || g.gd[od].state == GuardState::Dead || !g.ch[c].alive || c == shooter || !crate::teams::is_hostile(g, sg_team, team) || g.gd[od].god { continue; }
            let to = scan(g, &f, t_hit, c, false);
            if to >= 0.0 && to < t_hit { t_hit = to; hit_guard = Some(od); hostage_hit = false; player_hit = false; }
        }
        if ff_ours {
            let mut i = 0;
            while i < g.seats.len() {
                let s = g.seats[i];
                i += 1;
                let pl = crate::cmd::seat_man(g, s);
                if g.pl[pl].down { continue; }
                let c = g.pl[pl].ch;
                let t_s = scan(g, &f, t_hit, c, true);
                if t_s >= 0.0 && t_s < t_hit { t_hit = t_s; player_hit = true; hit_guard = None; hostage_hit = false; hit_view = pl; }
            }
            let mut i = 0;
            while i < g.guards.len() {
                let od = g.guards[i];
                i += 1;
                let c = g.gd[od].ch;
                if Some(od) == held || c == shooter || !crate::friendlyfire::ff_body(g, od) { continue; }
                let to = scan(g, &f, t_hit, c, true);
                if to >= 0.0 && to < t_hit { t_hit = to; hit_guard = Some(od); hostage_hit = false; player_hit = false; }
            }
        }
    }
    if let Some(h) = &ph { if hit_guard.is_none() && !player_hit { t_hit = h.t; } }
    let hit_p = match &pen { Some(p) => path_at(&p.path, t_hit), None => v3::mad(muzzle, dir, t_hit) };
    let flown = dir;
    if let Some(b) = &bars { crate::winbars::bars_settle(g, b, t_hit, who, lead); }
    if let Some(p) = &pen {
        dir = seg_at(&p.path, t_hit).d;
        with_glass(&mut pel, |sh| crate::glasspen::commit_crossings(g, &p.path.cross, t_hit, who, sh));
        if let Some(w) = &p.walls { crate::wallpen::commit_walls(g, w, t_hit, shooter, &def, lead, third, pel.as_mut()); }
    }
    // (st, stopped there: the round's end is in that static, not a man, not the panel at the muzzle)
    let st_stop = st.filter(|s| m::abs(t_hit - s.t) < 1e-6);
    let bare_stop = hit_guard.is_none() && !player_hit && !through;
    if wall_live && bare_stop { if let Some(s) = st_stop { crate::wallpen::spent_in_wall(g, s.index); } }
    if bare_stop && (pen_live || third) {
        if let Some(s) = st_stop {
            if s.index < 1e6 {
                if pen_live {
                    let e = match &pen { Some(p) => seg_at(&p.path, t_hit).e, None => e_muzzle };
                    with_glass(&mut pel, |sh| spent_in_pane(g, s.index, hit_p, dir, e, who, sh));
                } else {
                    crate::glass::glass_hit(g, s.index, hit_p, Some(dir), who, None, None);
                }
            }
        }
    }
    if let Some(p) = &mut pel {
        let index = match arm_at {
            Some(a) => a.index,
            None => match st_stop { Some(s) if bare_stop => s.index, _ => -1.0 },
        };
        let at = match arm_at { Some(a) => a.at, None => hit_p };
        crate::shotgun::pellet_end(p, index, t_hit, at);
    }
    if third {
        let hit_c = match hit_guard { Some(h) => Some(g.gd[h].ch), None => if player_hit { Some(g.pl[hit_view].ch) } else { None } };
        crate::suppression::note_round(g, shooter, muzzle, hit_p, hit_c);
    }
    match (&pen, &ph) {
        (Some(p), Some(h)) => { e_k = seg_at(&p.path, h.t).e / e0; }
        _ => { if pen_live { e_k = e_muzzle / e0; } }
    }
    // (ph, in front of where the round stopped: not a prop standing behind the body it stopped in)
    let ph_met = ph.filter(|h| h.t <= t_hit + 1e-3);
    if third {
        if let Some(h) = ph_met {
            let j = def.prop_impulse * e_k;
            let pd = match &pen { Some(p) => seg_at(&p.path, h.t).d, None => dir };
            let point = match &pen { Some(p) => path_at(&p.path, h.t), None => v3::mad(muzzle, dir, h.t) };
            crate::props::prop_system_apply_impulse(g, h.prop, point, [pd[0] * j, 0.0, pd[2] * j], who, Some(true));
        }
    }
    // bullet wake through smoke: each straight leg one line on the look page, else the row of soft dots
    let mut lined = false;
    if third && g.m.render_frame.look_config.on && g.m.render_frame.look_config.stir > 0.0 {
        let far = m::min(t_hit, 25.0);
        if let Some(p) = &pen {
            for k in &p.path.segs {
                let a = m::max(k.t0, 0.3);
                let b = m::min(k.t0 + k.len, far);
                if b > a { lined = crate::smoke_types::smoke_system_like_round(g, v3::mad(k.o, k.d, a - k.t0), k.d, b - a) || lined; }
            }
        } else if far > 0.3 {
            lined = crate::smoke_types::smoke_system_like_round(g, v3::mad(muzzle, dir, 0.3), dir, far - 0.3);
        }
    }
    if third && !lined {
        let mut d = 0.3;
        while d < m::min(t_hit, 25.0) {
            let p = match &pen { Some(q) => path_at(&q.path, d), None => v3::mad(muzzle, dir, d) };
            if crate::smoke_types::smoke_system_like_in_smoke_domain(g, p) {
                let along = match &pen { Some(q) => seg_at(&q.path, d).d, None => dir };
                crate::smoke_types::smoke_system_like_push(g, crate::smoke_types::SmokePush { pos: p, dir: along, speed: 5.5, radius: 0.07 });
            }
            d += 0.28;
        }
    }
    // impact flash + sound
    let blind = match hit_guard { Some(h) => crate::fogofwar::fog_of_war_hidden(g, h), None => false };
    if blind { crate::fogofwar::fog_of_war_hush(g); }
    if lead {
        let fl = flash(g, v3::mad(hit_p, dir, -0.05), [1.0, 0.8, 0.5], 1.5, 2.5, 0.05, 0.0);
        if !blind { crate::fogofwar::fog_of_war_light_by(g, fl, Some(who)); }
    }
    let mut in_glass = false;
    if !through {
        if let Some(s) = st_stop {
            if s.index < 1e6 {
                let flags = match g.level.boxes.get(s.index as usize) { Some(b) => b.flags, None => 0.0 };
                in_glass = (js::to_i32(flags) & js::to_i32(crate::scene_boxes::box_flag().glass)) != 0;
            }
        }
    }
    let flesh = hit_guard.is_some() || player_hit;
    if !flesh && !in_glass && lead { crate::audio_iface::game_audio_like_play(g, "bulletImpact", Some(hit_p), Some(0.8), None); }
    if bare_stop && !in_glass {
        let n = match (ph_met, st) { (Some(h), _) => h.n, (None, Some(s)) => s.n, (None, None) => [0.0, 1.0, 0.0] };
        let surface = match (ph_met, st) {
            (Some(h), _) => Some(crate::marks::Surface2 { box_: h.box_, dynamic: true }),
            (None, Some(s)) if s.t <= t_hit + 1e-3 => crate::marks::static_surface(g, s.index),
            _ => None,
        };
        crate::marks::bullet_mark(g, hit_p, n, surface, &def);
    }
    if bare_stop {
        if let Some(h) = ph_met {
            crate::particle_fx::impact_fx(g, hit_p, dir, crate::particle_fx::ImpactFxHit { n: h.n, index: None, prop: Some(h.prop), box_: Some(h.box_) }, &def);
        } else if let Some(s) = st {
            crate::particle_fx::impact_fx(g, hit_p, dir, crate::particle_fx::ImpactFxHit { n: s.n, index: Some(s.index), prop: None, box_: None }, &def);
        }
    }
    if bare_stop && third {
        use crate::destruction::DestructionRoundHit as Hit;
        let mut hit = match (ph_met, st) {
            (Some(h), _) => Hit { n: h.n, index: None, prop: Some(h.prop), box_: Some(h.box_) },
            (None, Some(s)) => Hit { n: s.n, index: Some(s.index), prop: None, box_: None },
            (None, None) => Hit { n: [0.0, 1.0, 0.0], index: None, prop: None, box_: None },
        };
        crate::destruction::destruction_round(g, shooter, hit_p, dir, &mut hit, &def);
    }
    if def.tracer.is_some() && g.tune.tracers {
        use crate::tracers::TracerImpact;
        let gun = firearm_of(g, shooter, is_player);
        let along = if pen.is_some() { v3::normalize(v3::sub(hit_p, muzzle)) } else { dir };
        let far = if pen.is_some() { v3::dist(muzzle, hit_p) } else { t_hit };
        let impact = if flesh { TracerImpact::Flesh } else if st.is_some() || ph.is_some() || through { TracerImpact::Solid } else { TracerImpact::None };
        crate::tracers::tracers_round(g, Some(gun), &def, muzzle, along, far, impact, who, None);
    }
    if ff_ours {
        let own_hit = match hit_guard {
            Some(h) if g.gd[h].civ.is_none() => { let team = g.gd[h].team; !crate::teams::is_hostile(g, crate::teams::Team::Player, team) }
            _ => false,
        };
        if player_hit || own_hit { crate::friendlyfire::ff_note(g, sg, is_player, player_hit); }
    }
    let hit_c = hit_guard.map(|h| g.gd[h].ch);
    if ff_on && is_player && lead { crate::friendlyfire::ff_past(g, muzzle, hit_p, hit_c); }
    let victim = if player_hit { Some(g.pl[hit_view].ch) } else { hit_c };
    let was_down = if player_hit { g.pl[hit_view].down } else { match hit_guard { Some(h) => g.gd[h].state == GuardState::Dead, None => false } };
    crate::deathrecap::recap_round(g, victim, was_down, shooter, muzzle, hit_p, &def, o.blind);
    if let Some(hg) = hit_guard {
        let late = crate::shotgun::ShellLate::ManDown { hit_guard: hg, sg, shooter, is_player, ff_ours, hostage_hit, muzzle, dir, t_hit, hit_p };
        match &mut pel {
            None => run_shell_late(g, &late),
            Some(p) => { if crate::shotgun::pellet_drops(g, p, Some(hg), t_hit, dir) { p.late.push(late); } }
        }
    }
    if player_hit {
        let late = crate::shotgun::ShellLate::SamHit { hit_view, sg, dir, hit_p };
        match &mut pel {
            None => run_shell_late(g, &late),
            Some(p) => { if crate::shotgun::pellet_drops(g, p, None, t_hit, dir) { p.late.push(late); } }
        }
    }
    // lights: a shot that meets a fixture breaks it
    if is_player {
        let legs: Option<Vec<crate::aimcost::RoundLeg>> = if g.touring {
            None
        } else {
            match &pen {
                None => Some(vec![crate::aimcost::RoundLeg { o: muzzle, d: flown, end: t_hit }]),
                Some(p) => Some(p.path.segs.iter().filter(|s| s.t0 <= t_hit).map(|s| crate::aimcost::RoundLeg { o: s.o, d: s.d, end: m::min(s.len, t_hit - s.t0) }).collect()),
            }
        };
        if let Some(legs) = legs {
            let st_ix = match st_stop { Some(s) => s.index, None => -1.0 };
            if st_ix >= 0.0 {
                let mut i = 0;
                while i < g.targets.len() {
                    let t = g.targets[i];
                    i += 1;
                    if crate::fire::fire_holds(g, t) && crate::fire::fire_struck(g, t, hit_p, st_ix) {
                        crate::fire::fire_stir(g, t);
                        break;
                    }
                }
            }
            let mut got: Option<FixtureId> = None;
            if st_ix >= 0.0 {
                let mut i = 0;
                while i < g.targets.len() {
                    let t = g.targets[i];
                    i += 1;
                    if lamp_live(g, t) && lamp_own(g, t).contains(&st_ix) { got = Some(t); break; }
                }
            }
            if got.is_none() {
                let mut i = 0;
                while i < g.targets.len() {
                    let t = g.targets[i];
                    i += 1;
                    let pos = g.tab.sim_lightshome_fixture[t].pos;
                    if !lamp_live(g, t) || crate::aimcost::legs_near(pos, &legs) > 3.0 { continue; }
                    let kind = g.tab.sim_lightshome_fixture[t].kind;
                    let slack = if kind == FixtureKind::Breaker { 0.08 } else { 0.3 };
                    let boxes: Vec<Option<BoxId>> = lamp_own(g, t).iter().map(|&b| g.engine.world.statics.get(b as usize).copied()).collect();
                    let bare = if kind == FixtureKind::Device { g.k.aimcost.bare_device } else { g.k.aimcost.bare };
                    let tight = m::min(slack, g.k.aimcost.slack);
                    if crate::aimcost::lamp_struck(g, pos, &boxes, false, &legs, Some(bare), Some(tight)) { got = Some(t); break; }
                    if near_lamp(g, t, &legs, slack) { g.stats.aimcost.lamp_near += 1; }
                }
            }
            if let Some(t) = got { g.stats.aimcost.lamp_hits += 1; shoot_out(g, t, muzzle); }
        } else {
            // a show on the stage: the old near-is-a-hit loop
            let mut i = 0;
            while i < g.targets.len() {
                let t = g.targets[i];
                i += 1;
                if crate::sim_lightshome::fixture_broken(g, t) || !g.tab.sim_lightshome_fixture[t].interactive
                    || crate::sim_lightshome::fixture_factor(g, t) <= 0.0 && !crate::sim_lightshome::fixture_on(g, t) && !crate::carlamps::lamp_glass(g, t) { continue; }
                if crate::fire::fire_holds(g, t) {
                    if let Some(s) = st_stop { if crate::fire::fire_struck(g, t, hit_p, s.index) { crate::fire::fire_stir(g, t); } }
                    continue;
                }
                let pos = g.tab.sim_lightshome_fixture[t].pos;
                let kind = g.tab.sim_lightshome_fixture[t].kind;
                let slack = if kind == FixtureKind::Breaker { 0.08 } else { 0.3 };
                let mut dist = -1.0;
                match &pen {
                    None => {
                        let to_t = v3::sub(pos, muzzle);
                        let depth = v3::dot(to_t, flown);
                        if depth > t_hit + slack { continue; }
                        let along = v3::clamp(depth, 0.0, t_hit);
                        dist = v3::len(v3::sub(to_t, v3::scale(flown, along)));
                    }
                    Some(p) => {
                        for s in &p.path.segs {
                            if s.t0 > t_hit { break; }
                            let end = m::min(s.len, t_hit - s.t0);
                            let to_t = v3::sub(pos, s.o);
                            let depth = v3::dot(to_t, s.d);
                            if depth > end + slack { continue; }
                            let d = v3::len(v3::sub(to_t, v3::scale(s.d, v3::clamp(depth, 0.0, end))));
                            if dist < 0.0 || d < dist { dist = d; }
                        }
                        if dist < 0.0 { continue; }
                    }
                }
                let rad = if crate::carlamps::lamp_glass(g, t) { 0.2 } else if kind == FixtureKind::Emissive { 0.45 } else if kind == FixtureKind::Breaker { 0.4 } else { 0.3 };
                if dist < rad { shoot_out(g, t, muzzle); break; }
            }
        }
    } else if !through {
        let ix = st_stop.map(|s| s.index);
        match &pen {
            None => crate::destruction::destruction_fixture_round(g, muzzle, dir, t_hit, ix),
            Some(p) => {
                let mut k = p.path.segs.len();
                while k > 0 {
                    k -= 1;
                    let s = p.path.segs[k];
                    if s.t0 > t_hit { continue; }
                    let here = if s.t0 + s.len >= t_hit - 1e-6 { ix } else { None };
                    crate::destruction::destruction_fixture_round(g, s.o, s.d, m::min(s.len, t_hit - s.t0), here);
                }
            }
        }
    }
    let report = &def.report;
    if lead {
        let kind = if is_player { crate::director::EventKind::Shot } else { crate::director::EventKind::GuardShot };
        let serial = g.serial();
        let ev = crate::game::GameEvent {
            kind,
            pos: g.ch[shooter].pos,
            time: g.time,
            loud: report.loud,
            level: Some(report.level),
            weapon: Some(def.id),
            from: Some(muzzle),
            to: Some(hit_p),
            who: Some(who),
            blind: if o.blind.unwrap_or(false) { Some(true) } else { None },
            serial,
            ..crate::game::GameEvent::zeroed()
        };
        g.events.push(ev);
    }
    if is_player && lead {
        let p = g.player;
        g.pl[p].noise = m::max(g.pl[p].noise, report.level);
        let short = crate::gear::plain_short(&crate::gear::PlainShortDef { id: String::from(def.id.as_str()), short: def.short.clone() });
        crate::chronicle::chron_shot(g, &short, hit_guard, hit_p);
    }
    if let Some(p) = pel { seam_shell_back(g, o, p); }
}

/// What the arm is through and where: a leaf's index (>= 1e6) is what the lock rule wants of a muzzle pressed to the keep. (combat.ts acrossArm)
pub fn across_arm(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::combat::AcrossArm> {
    let from = match g.ch[c_2].bones.get(crate::character::Bone::UpperArmR) { Some(&b) => b, None => v3::add(g.ch[c_2].pos, [0.0, 1.4, 0.0]) };
    let d = v3::sub(g.ch[c_2].muzzle, from);
    let l = v3::len(d);
    if l < 1e-3 { return None; }
    let rd = v3::scale(d, 1.0 / l);
    let h = crate::scene_collision::raycast(g, from, rd, l);
    h.map(|h| crate::combat::AcrossArm { index: h.index, at: v3::mad(from, rd, h.t) })
}

/// The panel between shoulder and muzzle, if a glass pane, takes the round point-blank. (combat.ts glassAcrossArm)
pub fn glass_across_arm(g: &mut crate::game::Game, c_2: CharId, dir: V3) {
    let from = match g.ch[c_2].bones.get(crate::character::Bone::UpperArmR) { Some(&b) => b, None => v3::add(g.ch[c_2].pos, [0.0, 1.4, 0.0]) };
    let d = v3::sub(g.ch[c_2].muzzle, from);
    let l = v3::len(d);
    if l < 1e-3 { return; }
    let rd = v3::scale(d, 1.0 / l);
    if let Some(h) = crate::scene_collision::raycast(g, from, rd, l) {
        if h.index < 1e6 {
            let who = g.ch[c_2].id;
            crate::glass::glass_hit(g, h.index, v3::mad(from, rd, h.t), Some(dir), who, None, None);
        }
    }
}

/// The static between shoulder and muzzle takes the round as the wall engine counts one. (combat.ts wallAcrossArm)
pub fn wall_across_arm(g: &mut crate::game::Game, c_2: CharId, def: &crate::weapons::WeaponDef) {
    let chaos = crate::destruction::chaos_of(g);
    if chaos <= 0.0 { return; }
    let from = match g.ch[c_2].bones.get(crate::character::Bone::UpperArmR) { Some(&b) => b, None => v3::add(g.ch[c_2].pos, [0.0, 1.4, 0.0]) };
    let d = v3::sub(g.ch[c_2].muzzle, from);
    let l = v3::len(d);
    if l < 1e-3 { return; }
    let rd = v3::scale(d, 1.0 / l);
    if let Some(h) = crate::scene_collision::raycast(g, from, rd, l) {
        let who = g.ch[c_2].id;
        crate::wallbreak::wall_break_round(g, who, v3::mad(from, rd, h.t), rd, h.index, def, chaos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// what a call writes on the lent glass is on the shell afterwards (the pellets of one shell share it)
    #[test]
    fn a_shell_gets_its_glass_back() {
        let mut pel = Some(crate::shotgun::Shell::zeroed());
        with_glass(&mut pel, |sh| { sh.unwrap().glass.set(crate::Num(7.0), vec![1.0]); });
        assert_eq!(pel.unwrap().glass.get(crate::Num(7.0)).map(|v| v.len()), Some(1));
        let mut none: Option<crate::shotgun::Shell> = None;
        assert!(with_glass(&mut none, |sh| sh.is_none()));
    }
}
