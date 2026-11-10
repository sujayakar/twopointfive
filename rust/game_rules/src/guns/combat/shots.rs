use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ------------------------------------------------------------ shooting

/// The Firearm this character carries: the player's gun in hand, else his pistol; a guard's weapon; a stray's 9 mm.
pub fn firearm_of(g: &mut crate::game::Game, shooter: CharId) -> FirearmId {
    if let crate::character::Who::Player(p) = g.ch[shooter].who {
        return match crate::game::player_gun_in_hand(g, p) { Some(f) => f, None => g.pl[p].pistol };
    }
    if let Some(x) = guard_of_char(g, shooter) { return g.gd[x].weapon; }
    if let Some(f) = g.m.combat.stray_gun { return f; }
    let row = weapon_row(g, crate::weapons::WeaponId::Guard9mm);
    let f = crate::weapons::firearm_new(g, &row);
    g.m.combat.stray_gun = Some(f);
    f
}

/// a thing run in the game as this seat has it: with_seat, or as it stands when the pass is already about that seat
fn in_seat(g: &mut Game, seat: SeatId, run: &mut dyn FnMut(&mut Game)) {
    if seat == g.seat { run(g); } else { crate::seats::with_seat(g, seat, run); }
}

/// the weapon's row by its key (a Firearm holds the key; WEAPONS the row)
fn weapon_row(g: &Game, id: crate::weapons::WeaponId) -> crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(id).unwrap().clone()
}

/// g.guards.find(x => x.char === c)
fn guard_of_char(g: &Game, c: CharId) -> Option<GuardId> {
    g.guards.iter().copied().find(|&x| g.gd[x].ch == c)
}

/// A guard's own hands: the row's spread off the man he is firing at.
fn ai_wide(g: &crate::game::Game, seat: &crate::seats::SeatRef, sg: Option<GuardId>, shooter: CharId, def: &crate::weapons::WeaponDef) -> f64 {
    let foe = match sg { Some(s) => g.gd[s].foe, None => None };
    let tc = match foe { Some(c) => c, None => g.pl[seat.player].ch };
    let tv = m::hyp2(g.ch[tc].vel[0], g.ch[tc].vel[2]);
    let range = v3::dist(g.ch[shooter].pos, g.ch[tc].pos);
    let s = def.ai_spread;
    s.base + m::min(s.move_cap, tv * s.move_k) + m::max(0.0, range - s.range_from) * s.range_k
}

/// One round from the shooter's muzzle at `target`, in the game as he has it.
pub fn fire_weapon(g: &mut crate::game::Game, shooter: CharId, target: V3, o: Option<&crate::combat::FireOpts>) {
    let none = FireOpts::default();
    let o = o.unwrap_or(&none);
    round_in_view(g, shooter, target, o, None);
}

/// fireWeapon for one pellet of a shell, the shell lent so the round writes on it.
pub(in crate::guns) fn fire_pellet(g: &mut crate::game::Game, shooter: CharId, target: V3, o: &crate::combat::FireOpts, shell: &mut crate::shotgun::Shell) {
    round_in_view(g, shooter, target, o, Some(shell));
}

/// fireWeapon's one line: the view, the fog's begin, the round, the bars unmet, the fog's done.
fn round_in_view(g: &mut Game, shooter: CharId, target: V3, o: &FireOpts, mut pel: Option<&mut crate::shotgun::Shell>) {
    let id = g.ch[shooter].id;
    let seat = crate::seats::view_of_body(g, crate::seats::ViewOfBodyC { id });
    in_seat(g, seat, &mut |g: &mut Game| {
        let who = g.ch[shooter].id;
        let was = crate::fogofwar::fog_of_war_begin(g, Some(who));
        fire_weapon_raw(g, shooter, target, o, pel.as_deref_mut());
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
fn fly(g: &mut Game, muzzle: V3, dir: V3, e_muzzle: f64, st0: Option<crate::glasspen::RoundPathSt>, arm_panes: f64) -> Option<Pen> {
    crate::wallpen::thin_path(g, muzzle, dir, 60.0, e_muzzle, Some(st0), Some(arm_panes)).map(|p| Pen {
        path: crate::glasspen::RoundPath { segs: p.segs, cross: p.cross, st: p.st, len: p.len },
        walls: Some(p.walls),
    })
}

/// A Shell handed on as the ShellGlass it is in the TypeScript: its `glass` is lent for the call and comes back with what the call wrote.
fn with_glass<R>(pel: &mut Option<&mut crate::shotgun::Shell>, run: impl FnOnce(Option<&mut crate::glasspen::ShellGlass>) -> R) -> R {
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

/// The man in the arm of the player whose seat this is.
fn shield_of(g: &Game) -> Option<CharId> { g.pl[g.player].holding.as_ref().map(|h| g.gd[h.g].ch) }

/// Whom a round of this man's can find, his enemies first, each with whether he is of the shooter's own side. A man in a player's arm is in front of
/// him, for anybody's rounds but his; a civilian is found by a player's alone; a teammate and Sam by each other's (friendlyfire).
fn whom_it_can_find(g: &Game, shooter: CharId, ours: bool) -> Vec<(CharId, bool)> {
    let by_player = crate::character::is_player(g, shooter);
    let side = crate::teams::team_of_char(g, shooter);
    let shield = shield_of(g);
    let (mut foes, mut own) = (Vec::new(), Vec::new());
    for pl in crate::seats::men(g) {
        let c = g.pl[pl].ch;
        if by_player || crate::condition::down(g, c) { continue; }
        if crate::teams::is_hostile(g, side, crate::teams::Team::Player) { foes.push((c, false)); } else if ours { own.push((c, true)); }
    }
    if let Some(c) = shield.filter(|&c| !by_player && c != shooter && !crate::condition::down(g, c)) { foes.push((c, false)); }
    for &gd in &g.guards {
        let c = g.gd[gd].ch;
        if Some(c) == shield || c == shooter || !g.ch[c].alive || crate::condition::down(g, c) { continue; }
        if crate::teams::is_hostile(g, side, g.gd[gd].team) {
            if by_player || !g.gd[gd].god { foes.push((c, false)); }
        } else if ours && crate::friendlyfire::ff_body(g, gd) {
            own.push((c, true));
        }
    }
    if by_player { foes.extend(crate::civilians::civilian_bodies(g).into_iter().map(|cv| (g.gd[cv].ch, false))); }
    foes.extend(own);
    foes
}

/// What a round in a man does: at once for a single round, once the whole pattern has flown for a pellet.
pub(in crate::guns) fn round_in(g: &mut crate::game::Game, r: &crate::shotgun::RoundIn) {
    match g.ch[r.c].who {
        crate::character::Who::Player(pl) => sam_hit(g, pl, r),
        crate::character::Who::Guard(gd) => man_down(g, gd, r),
    }
}

/// The man the round found: where it landed decides, then the sound, whose round it was.
fn man_down(g: &mut Game, gd: GuardId, r: &crate::shotgun::RoundIn) {
    use crate::game::{DownBy, DownKind};
    let was_up = !crate::condition::down(g, r.c);
    let by_player = crate::character::is_player(g, r.shooter);
    let sg = if by_player { None } else { guard_of_char(g, r.shooter) };
    let by_who = if by_player { DownBy::Player } else { DownBy::Guard };
    kill_guard(g, gd, r.dir, Some(false), Some(DownKind::Shot), Some(by_who), crate::game::With::Hand);
    crate::audio_iface::game_audio_like_play(g, "bodyHit", Some(r.hit_p), Some(0.8), None);
    if was_up { g.gd[gd].killer_id = g.ch[r.shooter].id; }
    if let Some(s) = sg.filter(|_| r.shield) {
        if !crate::condition::down(g, g.gd[s].ch) { crate::guards::hostage_hit_bark(g, s, gd); }
    }
}

/// A round in a player's man: the flinch, the flash, lock work ended, and the hit unless god mode.
fn sam_hit(g: &mut Game, pl: PlayerId, r: &crate::shotgun::RoundIn) {
    let (c, dir) = (r.c, r.dir);
    crate::character::anim_hit_from(g, c, Some(dir));
    g.pl[pl].hit_flash = 1.0;
    if g.pl[pl].picking.is_some() { g.pl[pl].picking = None; g.pl[pl].f_held = true; }
    if g.pl[pl].wanding.is_some() { crate::wand::wand_drop(g, pl); g.pl[pl].f_held = true; }
    crate::audio_iface::game_audio_like_play(g, "bodyHit", Some(r.hit_p), Some(0.9), None);
    if !g.god_mode {
        let seat = crate::seats::seat_of_man(g, pl);
        in_seat(g, seat, &mut |g: &mut Game| hit_player(g, dir));
        let by = guard_of_char(g, r.shooter).map(|s| g.gd[s].callsign.clone());
        crate::chronicle::chron_hit(g, by.as_deref());
    }
}

/// A shot-out light: the breaker's blackout, a dark lens's glass, or a fixture gone. (fireWeapon's shootOut)
fn shoot_out(g: &mut Game, t: FixtureId, muzzle: V3) {
    use crate::fixtures::FixtureKind;
    let kind = g.tab.fixture[t].kind;
    let pos = g.tab.fixture[t].pos;
    if kind == FixtureKind::Breaker {
        crate::render_lights::light_set_flash(g, pos, [0.6, 0.7, 1.0], 10.0, 5.0, 0.2, 0.0);
        crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(1.0), None);
        crate::game::game_set_blackout(g, f64::INFINITY, crate::game::GameSetBlackoutCause::Shot);
        g.tab.fixture[t].broken = true;
        crate::particle_fx::breaker_spark_fx(g, pos, muzzle);
        return;
    }
    let lens = g.tab.fixture[t].lens;
    if crate::carlamps::dead_lamp(g, t) {
        g.tab.fixture[t].broken = true;
        crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(0.8), None);
        crate::particle_fx::lens_shatter_fx(g, crate::particle_fx::LensShatterFxT { pos, lens }, muzzle);
        return;
    }
    g.tab.fixture[t].broken = true;
    crate::game::game_light_event(g, t, None, None);
    crate::chronicle::chron_light_shot(g, t);
    crate::render_lights::light_set_flash(g, pos, [0.7, 0.8, 1.0], 6.0, 4.0, 0.12, 0.0);
    crate::audio_iface::game_audio_like_play(g, "lightBreak", Some(pos), Some(1.0), None);
    if crate::carlamps::lamp_glass(g, t) {
        crate::particle_fx::lens_shatter_fx(g, crate::particle_fx::LensShatterFxT { pos, lens }, muzzle);
    } else if kind != FixtureKind::Device {
        let boxes = g.tab.fixture[t].boxes.clone();
        crate::particle_fx::fixture_shatter_fx(g, &crate::particle_fx::FixtureShatterFxT { pos, kind: String::from(kind.as_str()), boxes }, muzzle);
    }
}

/// `live` of the real-hit search: a target a round can put out.
fn lamp_live(g: &Game, t: FixtureId) -> bool {
    !(g.tab.fixture[t].broken
        || !g.tab.fixture[t].interactive
        || g.tab.fixture[t].factor <= 0.0 && !g.tab.fixture[t].on && !crate::carlamps::lamp_glass(g, t)
        || crate::fire::fire_holds(g, t))
}

/// `own` of the real-hit search: the breaker's target lists its LED alone, a round into the panel is a round into the breaker.
fn lamp_own(g: &Game, t: FixtureId) -> Vec<f64> {
    if g.tab.fixture[t].kind == crate::fixtures::FixtureKind::Breaker { g.level.breaker.boxes.clone() } else { g.tab.fixture[t].boxes.clone() }
}

/// The round itself, the shooter's muzzle to where it stops: what it found, what it broke, what the floor heard.
fn fire_weapon_raw(g: &mut crate::game::Game, shooter: CharId, target: V3, o: &crate::combat::FireOpts, pel: Option<&mut crate::shotgun::Shell>) {
    let is_player = crate::character::is_player(g, shooter);
    use crate::game::GuardState;
    use crate::glasspen::{path_at, seg_at, spent_in_pane};
    use crate::fixtures::FixtureKind;

    let sg = if is_player { None } else { guard_of_char(g, shooter) };
    if let Some(s) = sg { if g.gd[s].work_stow.is_some() { crate::workstow::stow_fired(g, s); } }
    if !is_player && g.ch[shooter].anim.big_tool.is_some() { crate::bigkit::big_fired(g, shooter); }
    let def: crate::weapons::WeaponDef = match &o.weapon { Some(d) => d.clone(), None => { let f = firearm_of(g, shooter); let id = g.fa[f].def; weapon_row(g, id) } };
    let mut spread = o.spread.unwrap_or(0.0);
    let steady = o.steady.unwrap_or(false);
    let mut pel = pel;
    let pellets = def.pellets;
    let who = g.ch[shooter].id;

    let ff_ours = is_player || crate::friendlyfire::ff_side(g, sg) ;
    if ff_ours && !is_player && pel.is_none() {
        let wide = if steady { 0.0 } else { ai_wide(g, &crate::seats::here(g), sg, shooter, &def) };
        let cone = match &pellets { Some(p) => p.cone * 2.0, None => 0.0 };
        let at = g.ch[shooter].muzzle;
        if crate::friendlyfire::ff_unpressed(g, sg, at, target, spread + wide + cone, &def) { return; }
    }
    if pellets.is_some() && pel.is_none() { crate::shotgun::fire_shell(g, shooter, target, o, &def); return; }
    let lead = match &pel { None => true, Some(p) => p.i == 0.0 };
    let third = match &pel { None => true, Some(p) => js::rem(p.i, 3.0) == 0.0 };
    if lead { crate::character::anim_fire(g, shooter); }
    if is_player && lead { crate::mission::tally(g, crate::mission::TallyK::Shots); crate::holdfire::hold_fire_shot(g, shooter); }
    let muzzle = g.ch[shooter].muzzle;
    let mut dir = v3::normalize(v3::sub(target, muzzle));
    if !is_player && !steady { spread += ai_wide(g, &crate::seats::here(g), sg, shooter, &def); }
    if spread > 0.0 {
        let rx = crate::dice::draw_body(g, "combat::fire_weapon_raw#1", shooter);
        let ry = crate::dice::draw_body(g, "combat::fire_weapon_raw#2", shooter);
        let rz = crate::dice::draw_body(g, "combat::fire_weapon_raw#3", shooter);
        dir = v3::normalize([dir[0] + (rx - 0.5) * spread, dir[1] + (ry - 0.5) * spread, dir[2] + (rz - 0.5) * spread]);
    }
    if lead { if let Some(p) = &mut pel { p.dir = Some(dir); } }
    // the flash card: one frame of emissive box
    let flame = crate::gunlook::flame_of(g, &def);
    let mf = muzzle_row(flame);
    if lead {
        let card = crate::effects::muzzle_card();
        let bf = crate::scene_boxes::box_flag();
        let b = crate::scene_boxes::make_box_kept(g, &crate::scene_boxes::MakeBoxP {
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
    if lead {
        g.fx_seed += 1.0;
        let gain = mf.light_gain * (if def.suppressed { crate::effects::shot().suppressed_gain } else { 1.0 });
        let gas = !crate::fogofwar::fog_of_war_air_hides(g, shooter);
        let lights = crate::effects::spawn_shot(g, muzzle, dir, &crate::effects::SpawnShotO {
            light_gain: Some(gain),
            light_range: mf.light_range,
            track: Some(crate::effects::Track::Muzzle { by: shooter }),
            eject: Some(crate::effects::Track::EjectPort { by: shooter }),
            gas: Some(gas),
        });
        for l in lights {
            crate::render_lights::light_set_flash(g, l.pos, l.color, l.intensity, l.range, l.ttl, who);
        }
    }
    if lead { crate::particle_fx::muzzle_sparks_fx(g, muzzle, dir, shooter); crate::particle_fx::casing_fx(g, shooter, &def); }
    if lead { let voice = crate::gunlook::voice_of(g, &def); crate::audio_iface::game_audio_like_play(g, &voice, Some(muzzle), Some(1.0), None); }

    // --- hit scan
    let mut t_hit = 60.0;
    let mut hit: Option<CharId> = None;
    let mut st0 = crate::scene_collision::raycast(g, muzzle, dir, t_hit).map(st_of);
    if let Some(s) = st0 { t_hit = s.t; }
    let mut st = st0;
    let e0 = crate::glasspen::round_energy(g, def.id);
    let mut e_k = 1.0;
    let mut through = arm_through(g, shooter);
    let mut bars: Option<Vec<crate::winbars::BarMet>> = None;
    if through {
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
    if through {
        let arm = with_glass(&mut pel, |sh| crate::glasspen::arm_through_glass(g, shooter, dir, e0, sh));
        if let Some(arm) = arm { through = false; e_muzzle = arm.e; arm_panes = arm.panes; }
    }
    if through { t_hit = 0.0; }
    let mut pen = if through { None } else { fly(g, muzzle, dir, e_muzzle, st0, arm_panes) };
    if !through {
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
            pen = fly(g, muzzle, dir, e_muzzle, st0, arm_panes);
            n += 1.0;
        }
        if bars.is_some() { st = st0; t_hit = match st0 { Some(s) => s.t, None => 60.0 }; }
    }
    if let Some(p) = &pen { t_hit = p.path.len; st = p.path.st; }
    let arm_at = if through && pel.is_some() { across_arm(g, shooter) } else { None };
    match arm_at {
        Some(a) => {
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
    if !through {   // (through: into the panel at the muzzle, nobody beyond it)
        for (c, own) in whom_it_can_find(g, shooter, ff_ours) {
            let t = scan(g, &f, t_hit, c, own);
            if t >= 0.0 && t < t_hit { t_hit = t; hit = Some(c); }
        }
    }
    if let Some(h) = &ph { if hit.is_none() { t_hit = h.t; } }
    let hit_p = match &pen { Some(p) => path_at(&p.path, t_hit), None => v3::mad(muzzle, dir, t_hit) };
    let flown = dir;
    if let Some(b) = &bars { crate::winbars::bars_settle(g, b, t_hit, who, lead); }
    if let Some(p) = &pen {
        dir = seg_at(&p.path, t_hit).d;
        with_glass(&mut pel, |sh| crate::glasspen::commit_crossings(g, &p.path.cross, t_hit, who, sh));
        if let Some(w) = &p.walls { crate::wallpen::commit_walls(g, w, t_hit, shooter, &def, lead, third, pel.as_deref_mut()); }
    }
    // (st, stopped there: the round's end is in that static, not a man, not the panel at the muzzle)
    let st_stop = st.filter(|s| m::abs(t_hit - s.t) < 1e-6);
    let bare_stop = hit.is_none() && !through;
    if bare_stop {
        if let Some(s) = st_stop {
            if s.index < 1e6 {
                let e = match &pen { Some(p) => seg_at(&p.path, t_hit).e, None => e_muzzle };
                with_glass(&mut pel, |sh| spent_in_pane(g, s.index, hit_p, dir, e, who, sh));
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
        crate::suppression::note_round(g, shooter, muzzle, hit_p, hit);
    }
    match (&pen, &ph) {
        (Some(p), Some(h)) => { e_k = seg_at(&p.path, h.t).e / e0; }
        _ => { { e_k = e_muzzle / e0; } }
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
    // a round through the air: each straight leg is one line for the host's solver, which stirs its smoke along it or has none there to stir
    if third {
        let far = m::min(t_hit, 25.0);
        if let Some(p) = &pen {
            for k in &p.path.segs {
                let a = m::max(k.t0, 0.3);
                let b = m::min(k.t0 + k.len, far);
                if b > a { crate::smoke_types::smoke_system_like_round(g, v3::mad(k.o, k.d, a - k.t0), k.d, b - a); }
            }
        } else if far > 0.3 {
            crate::smoke_types::smoke_system_like_round(g, v3::mad(muzzle, dir, 0.3), dir, far - 0.3);
        }
    }
    // impact flash + sound
    let blind = hit.and_then(|c| crate::character::guard_of(g, c)).map_or(false, |h| crate::fogofwar::fog_of_war_hidden(g, h));
    if blind { crate::fogofwar::fog_of_war_hush(g); }
    if lead {
        let fl = crate::render_lights::light_set_flash(g, v3::mad(hit_p, dir, -0.05), [1.0, 0.8, 0.5], 1.5, 2.5, 0.05, 0.0);
        if !blind { crate::fogofwar::fog_of_war_light_by(g, fl, Some(who)); }
    }
    let mut in_glass = false;
    if !through {
        if let Some(s) = st_stop {
            if s.index < 1e6 {
                // (`g.level.boxes` is `g.col.boxes`; `(b?.flags ?? 0) & BoxFlag.Glass` is glass.ts:70 isPane, word for word)
                let b = g.col.boxes.get(s.index as usize).copied();
                in_glass = crate::glass::is_pane(g, b);
            }
        }
    }
    let flesh = hit.is_some();
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
        let hit = match (ph_met, st) {
            (Some(h), _) => Hit { n: h.n, index: None, prop: Some(h.prop), box_: Some(h.box_) },
            (None, Some(s)) => Hit { n: s.n, index: Some(s.index), prop: None, box_: None },
            (None, None) => Hit { n: [0.0, 1.0, 0.0], index: None, prop: None, box_: None },
        };
        crate::destruction::destruction_round(g, shooter, hit_p, dir, &hit, &def);
    }
    if def.tracer.is_some() && g.tune.tracers {
        use crate::tracers::TracerImpact;
        let gun = firearm_of(g, shooter);
        let along = if pen.is_some() { v3::normalize(v3::sub(hit_p, muzzle)) } else { dir };
        let far = if pen.is_some() { v3::dist(muzzle, hit_p) } else { t_hit };
        let impact = if flesh { TracerImpact::Flesh } else if st.is_some() || ph.is_some() || through { TracerImpact::Solid } else { TracerImpact::None };
        crate::tracers::tracers_round(g, Some(gun), &def, muzzle, along, far, impact, who, None);
    }
    if ff_ours && hit.map_or(false, |c| crate::friendlyfire::on_our_side(g, c)) { crate::friendlyfire::ff_note(g, shooter, hit); }
    if is_player && lead { crate::friendlyfire::ff_past(g, muzzle, hit_p, hit); }
    let was_down = hit.map_or(false, |c| crate::condition::down(g, c));
    crate::deathrecap::recap_round(g, hit, was_down, shooter, muzzle, hit_p, &def, o.blind);
    if let Some(c) = hit {
        let shield = Some(c) == shield_of(g);
        let r = crate::shotgun::RoundIn { c, shooter, shield, dir, hit_p };
        match &mut pel {
            None => round_in(g, &r),
            Some(p) => { if crate::shotgun::pellet_drops(g, p, c, t_hit, dir) { p.late.push(r); } }
        }
    }
    // lights: a shot that meets a fixture breaks it
    if is_player {
        let legs: Vec<crate::aimcost::RoundLeg> = match &pen {
            None => vec![crate::aimcost::RoundLeg { o: muzzle, d: flown, end: t_hit }],
            Some(p) => p.path.segs.iter().filter(|s| s.t0 <= t_hit).map(|s| crate::aimcost::RoundLeg { o: s.o, d: s.d, end: m::min(s.len, t_hit - s.t0) }).collect(),
        };
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
                let pos = g.tab.fixture[t].pos;
                if !lamp_live(g, t) || crate::aimcost::legs_near(pos, &legs) > 3.0 { continue; }
                let kind = g.tab.fixture[t].kind;
                let slack = if kind == FixtureKind::Breaker { 0.08 } else { 0.3 };
                let boxes: Vec<Option<BoxId>> = lamp_own(g, t).iter().map(|&b| g.engine.world.statics.get(b as usize).copied()).collect();
                let bare = if kind == FixtureKind::Device { g.k.aimcost.bare_device } else { g.k.aimcost.bare };
                let tight = m::min(slack, g.k.aimcost.slack);
                if crate::aimcost::lamp_struck(g, pos, &boxes, false, &legs, Some(bare), Some(tight)) { got = Some(t); break; }
            }
        }
        if let Some(t) = got {  shoot_out(g, t, muzzle); }
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
            ..crate::game::GameEvent::default()
        };
        g.events.push(ev);
    }
    if is_player && lead {
        let p = g.player;
        g.pl[p].noise = m::max(g.pl[p].noise, report.level);
        let short = crate::gear::plain_short(&def);
        crate::chronicle::chron_shot(g, &short, hit.is_some(), hit_p);
    }
}

/// What the arm is through and where: a leaf's index (>= 1e6) is what the lock rule wants of a muzzle pressed to the keep.
fn across_arm(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::combat::AcrossArm> {
    let from = match g.ch[c_2].bones.get(crate::character::Bone::UpperArmR) { Some(&b) => b, None => v3::add(g.ch[c_2].pos, [0.0, 1.4, 0.0]) };
    let d = v3::sub(g.ch[c_2].muzzle, from);
    let l = v3::len(d);
    if l < 1e-3 { return None; }
    let rd = v3::scale(d, 1.0 / l);
    let h = crate::scene_collision::raycast(g, from, rd, l);
    h.map(|h| crate::combat::AcrossArm { index: h.index, at: v3::mad(from, rd, h.t) })
}

/// The panel between shoulder and muzzle, if a glass pane, takes the round point-blank.
fn glass_across_arm(g: &mut crate::game::Game, c_2: CharId, dir: V3) {
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

/// The static between shoulder and muzzle takes the round as the wall engine counts one.
fn wall_across_arm(g: &mut crate::game::Game, c_2: CharId, def: &crate::weapons::WeaponDef) {
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
        let mut shell = crate::shotgun::Shell::default();
        let mut pel = Some(&mut shell);
        with_glass(&mut pel, |sh| { sh.unwrap().glass.set(crate::Num(7.0), vec![1.0]); });
        assert_eq!(shell.glass.get(crate::Num(7.0)).map(|v| v.len()), Some(1));
        let mut none: Option<&mut crate::shotgun::Shell> = None;
        assert!(with_glass(&mut none, |sh| sh.is_none()));
    }
}
