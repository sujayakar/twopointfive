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
    let row = g.weapon(crate::weapons::WeaponId::Guard9mm).clone();
    let f = crate::weapons::firearm_new(g, &row);
    g.m.combat.stray_gun = Some(f);
    f
}

/// a thing run in the game as this seat has it: with_seat, or as it stands when the pass is already about that seat
fn in_seat(g: &mut Game, seat: SeatId, run: &mut dyn FnMut(&mut Game)) {
    if seat == g.seat { run(g); } else { crate::seats::with_seat(g, seat, run); }
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
    crate::deed::shaken_off(g, pl);
    crate::audio_iface::game_audio_like_play(g, "bodyHit", Some(r.hit_p), Some(0.9), None);
    if !g.god_mode {
        let seat = crate::seats::seat_of_man(g, pl);
        in_seat(g, seat, &mut |g: &mut Game| hit_player(g, dir));
        let by = guard_of_char(g, r.shooter).map(|s| g.gd[s].callsign.clone());
        crate::chronicle::chron_hit(g, by.as_deref());
    }
}

/// One round or one pellet: what keeps it in the gun, how it leaves, the way it flies, and what it does where it ends.
fn fire_weapon_raw(g: &mut Game, shooter: CharId, target: V3, o: &FireOpts, shell: Option<&mut crate::shotgun::Shell>) {
    let by_player = crate::character::is_player(g, shooter);
    let sg = if by_player { None } else { guard_of_char(g, shooter) };
    hands_free(g, shooter, sg);
    let def = match &o.weapon { Some(d) => d.clone(), None => { let f = firearm_of(g, shooter); g.weapon(g.fa[f].def).clone() } };
    let ours = by_player || crate::friendlyfire::ff_side(g, sg);
    if shell.is_none() {
        if ours && !by_player && mate_in_the_way(g, sg, shooter, target, o, &def) { return; }
        if def.pellets.is_some() { return crate::shotgun::fire_shell(g, shooter, target, o, &def); }
    }
    let nth = shell.as_ref().map_or(0.0, |p| p.i);
    let lead = shell.is_none() || nth == 0.0;
    if lead { crate::character::anim_fire(g, shooter); }
    if by_player && lead { crate::mission::tally(g, crate::mission::TallyK::Shots); crate::holdfire::hold_fire_shot(g, shooter); }
    let dir = aims(g, shooter, sg, target, o, &def);
    let mut r = Round { shooter, who: g.ch[shooter].id, by_player, ours, blind: o.blind, def, shell, lead, third: js::rem(nth, 3.0) == 0.0, muzzle: g.ch[shooter].muzzle, dir };
    if lead {
        if let Some(shell) = r.shell.as_deref_mut() { shell.dir = Some(dir); }
        shot_shows(g, &r);
    }
    let f = flies(g, &mut r);
    let arm = at_the_arm(g, &mut r, &f);
    let e = ends(g, &r, &f);
    lands(g, &mut r, &f, arm, &e);
}

/// Whatever else was in his hands is out of them.
fn hands_free(g: &mut Game, shooter: CharId, sg: Option<GuardId>) {
    if let Some(s) = sg { if g.gd[s].work_stow.is_some() { crate::workstow::stow_fired(g, s); } }
    if !crate::character::is_player(g, shooter) && g.ch[shooter].anim.big_tool.is_some() { crate::bigkit::big_fired(g, shooter); }
}

/// The squad's own man does not press it with a mate in the cone of it.
fn mate_in_the_way(g: &mut Game, sg: Option<GuardId>, shooter: CharId, target: V3, o: &FireOpts, def: &crate::weapons::WeaponDef) -> bool {
    let wide = if o.steady.unwrap_or(false) { 0.0 } else { ai_wide(g, &crate::seats::here(g), sg, shooter, def) };
    let cone = match &def.pellets { Some(p) => p.cone * 2.0, None => 0.0 };
    let at = g.ch[shooter].muzzle;
    crate::friendlyfire::ff_unpressed(g, sg, at, target, o.spread.unwrap_or(0.0) + wide + cone, def)
}

/// The way it leaves: at its target, thrown off by the gun's spread and by an AI's own hands.
fn aims(g: &mut Game, shooter: CharId, sg: Option<GuardId>, target: V3, o: &FireOpts, def: &crate::weapons::WeaponDef) -> V3 {
    let mut spread = o.spread.unwrap_or(0.0);
    if !crate::character::is_player(g, shooter) && !o.steady.unwrap_or(false) { spread += ai_wide(g, &crate::seats::here(g), sg, shooter, def); }
    let dir = v3::normalize(v3::sub(target, g.ch[shooter].muzzle));
    if !(spread > 0.0) { return dir; }
    let rx = crate::dice::draw_body(g, "combat::fire_weapon_raw#1", shooter);
    let ry = crate::dice::draw_body(g, "combat::fire_weapon_raw#2", shooter);
    let rz = crate::dice::draw_body(g, "combat::fire_weapon_raw#3", shooter);
    v3::normalize([dir[0] + (rx - 0.5) * spread, dir[1] + (ry - 0.5) * spread, dir[2] + (rz - 0.5) * spread])
}

/// What is seen and heard of a shot at the muzzle.
fn shot_shows(g: &mut Game, r: &Round) {
    let mf = muzzle_row(crate::gunlook::flame_of(g, &r.def));
    flash_card(g, r, &mf);
    gas_and_light(g, r, &mf);
    crate::particle_fx::muzzle_sparks_fx(g, r.muzzle, r.dir, r.shooter);
    crate::particle_fx::casing_fx(g, r.shooter, &r.def);
    let voice = crate::gunlook::voice_of(g, &r.def);
    crate::audio_iface::game_audio_like_play(g, &voice, Some(r.muzzle), Some(1.0), None);
}

/// The flash card: one frame of emissive box.
fn flash_card(g: &mut Game, r: &Round, mf: &crate::effects::MuzzleFx) {
    let card = crate::effects::muzzle_card();
    let b = crate::scene_boxes::make_box_kept(g, &crate::scene_boxes::MakeBoxP {
        c: v3::mad(r.muzzle, r.dir, mf.card_ahead),
        h: mf.card_half,
        yaw: Some(v3::yaw_of(r.dir)),
        albedo: Some(card.albedo),
        emissive: Some(mf.card_emissive),
        flags: Some((crate::scene_boxes::box_flag::DYNAMIC as i32 | crate::scene_boxes::box_flag::NO_SHADOW as i32) as f64),
        ..Default::default()
    });
    g.fx.push(crate::game::GameFxItem { box_: b, ttl: card.ttl });
}

/// Gas, light and powder: the shot's effects.
fn gas_and_light(g: &mut Game, r: &Round, mf: &crate::effects::MuzzleFx) {
    g.fx_seed += 1.0;
    let gain = mf.light_gain * (if r.def.suppressed { crate::effects::shot().suppressed_gain } else { 1.0 });
    let gas = !crate::fogofwar::fog_of_war_air_hides(g, r.shooter);
    let lights = crate::effects::spawn_shot(g, r.muzzle, r.dir, &crate::effects::SpawnShotO {
        light_gain: Some(gain),
        light_range: mf.light_range,
        track: Some(crate::effects::Track::Muzzle { by: r.shooter }),
        eject: Some(crate::effects::Track::EjectPort { by: r.shooter }),
        gas: Some(gas),
    });
    for l in lights {
        crate::render_lights::light_set_flash(g, l.pos, l.color, l.intensity, l.range, l.ttl, r.who);
    }
}
