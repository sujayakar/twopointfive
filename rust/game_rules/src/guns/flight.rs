//! A THING OFF A TUBE, IN THE AIR: a ring, a 40 mm round and a camera to stick fly by one rule and differ by their rows (`Ballistics`) and by what
//! each does where it ends.
use crate::combat::BodyHitContact;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::launcher::Ballistics;
use crate::m;
use crate::v3::{self, V3};
use alloc::{string::String, vec::Vec};

/// What a substep met, the nearest first: a man wins a tie, and furniture wins one with what stands behind it.
pub enum Met {
    /// how far along the substep
    Man(CharId, f64),
    Prop(crate::props::PropSystemRaycast),
    Static(crate::scene_collision::Raycast),
    /// the ground where no slab is under it
    Floor,
}

/// One substep of a flight.
pub struct Leg {
    pub dir: V3,
    pub len: f64,
    /// where it is at the substep's end, if it met nothing on the way (or the ground there)
    pub to: V3,
    pub met: Option<Met>,
}

/// The bodies a thing `by` sent can find: the living, himself and the squad's own AI men apart.
pub fn bodies(g: &Game, by: CharId) -> Vec<CharId> {
    let men = g.guards.iter().filter(|&&gd| !g.team(gd).ours()).map(|&gd| g.gd[gd].ch);
    let civilians = crate::civilians::civilian_bodies(g).into_iter().map(|cv| g.gd[cv].ch);
    let players = crate::seats::men(g).into_iter().map(|pl| g.pl[pl].ch);
    let all: Vec<CharId> = men.chain(civilians).chain(players).collect();
    all.into_iter().filter(|&c| c != by && g.ch[c].alive && !crate::condition::down(g, c)).collect()
}

/// `h` seconds of it: drag and gravity on its velocity, then the first thing along the way. None: it does not move.
pub fn leg(g: &mut Game, b: &Ballistics, h: f64, pos: V3, vel: &mut V3, bodies: &[CharId]) -> Option<Leg> {
    let k = m::exp(-b.drag * h);
    *vel = [vel[0] * k, vel[1] * k - b.gravity * h, vel[2] * k];
    let seg = v3::scale(*vel, h);
    let len = v3::len(seg);
    if len < 1e-6 { return None; }
    let dir = v3::scale(seg, 1.0 / len);
    let reach = len + b.radius;
    let mut man: Option<(CharId, f64)> = None;
    for &c in bodies {
        let t = crate::combat::body_hit(g, c, pos, dir, reach, Some(BodyHitContact::Flight { radius: b.radius }));
        if t >= 0.0 && man.map_or(true, |(_, best)| t < best) { man = Some((c, t)); }
    }
    let wall = crate::scene_collision::raycast(g, pos, dir, reach);
    let prop = crate::props::prop_system_raycast(g, pos, dir, reach);
    let t_wall = wall.map_or(m::INF, |x| x.t);
    let t_prop = prop.map_or(m::INF, |x| x.t);
    let to = v3::add(pos, seg);
    let met = match (man, prop, wall) {
        (Some((c, t)), _, _) if t <= t_wall && t <= t_prop => Some(Met::Man(c, t)),
        (_, Some(p), _) if t_prop <= t_wall => Some(Met::Prop(p)),
        (_, _, Some(w)) => Some(Met::Static(w)),
        _ if to[1] < b.radius => Some(Met::Floor),
        _ => None,
    };
    Some(Leg { dir, len, to, met })
}

/// A velocity coming back off a surface with normal `n`: the normal share reversed by `restitution`, the slide kept by `keep`.
pub fn bounce(v: V3, n: V3, restitution: f64, keep: f64) -> V3 {
    let vn = v3::dot(v, n);
    let v_n = v3::scale(n, vn);
    let v_t = v3::sub(v, v_n);
    v3::add(v3::scale(v_t, keep), v3::scale(v_n, if vn < 0.0 { -restitution } else { 1.0 }))
}

/// The flight is over at `at`: from here it is a thing on the floor, with its motion capped.
pub fn to_item(g: &mut Game, kind: crate::throwables::ItemKind, by: CharId, at: V3, vel: V3, speed_max: f64, skitter: crate::throwables::ItemBounce) {
    let mut vel = vel;
    let sp = v3::len(vel);
    if sp > speed_max { vel = v3::scale(vel, speed_max / sp); }
    let who = g.ch[by].id;
    let opts = crate::throwables::ThrowablesSpawnOpts { who: Some(who), heading: Some(v3::yaw_of(vel)), ..Default::default() };
    let it = crate::throwables::throwables_spawn(g, kind, v3::copy(at), vel, Some(opts));
    g.tab.item[it].on_bounce = Some(skitter);
}

/// A noise in the room's business at `pos`, laid at `who`'s door (0: one nobody can put a man behind).
pub fn noise(g: &mut Game, pos: V3, level: f64, who: f64, lure: Option<bool>) {
    let (time, serial) = (g.time, g.serial());
    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos, time, loud: false, level: Some(level), who: Some(who), lure, serial, ..crate::game::GameEvent::default() });
}

/// One leaves `by`'s tube: the sound at it, and the noise at him.
pub fn launched(g: &mut Game, by: CharId, from: V3, gain: f64, rate: Option<f64>, level: f64) {
    let rate = rate.map(|rate| crate::audio_iface::PlayOpts::Rate { rate });
    crate::audio_iface::game_audio_like_play(g, "launcher", Some(from), Some(gain), rate.as_ref());
    noise_at(g, by, level);
}

/// A noise where this man stands, of his making: the room hears it, and he is that loud.
pub fn noise_at(g: &mut Game, by: CharId, level: f64) {
    let (pos, who) = (v3::copy(g.ch[by].pos), g.ch[by].id);
    noise(g, pos, level, who, None);
    louder(g, by, level);
}

/// This man is at least this loud.
pub fn louder(g: &mut Game, c: CharId, level: f64) {
    if let crate::character::Who::Player(pl) = g.ch[c].who { g.pl[pl].noise = m::max(g.pl[pl].noise, level); }
}

/// A noise where a man something found stands, laid on whoever sent the thing.
pub fn thump_at(g: &mut Game, c: CharId, level: f64, by: CharId) {
    let (pos, who) = (v3::copy(g.ch[c].pos), g.ch[by].id);
    noise(g, pos, level, who, None);
    louder(g, by, level);
}

/// A BLUNT THING HAS FOUND A MAN (a ring, a dud, a camera, pellets that did not drop him): he jolts, and it is heard where he stands. A player
/// sees it; a civilian takes fright back up the line it came along; anybody else doubles over, with a fix back up that line. A man in an arm only jolts.
pub fn struck(g: &mut Game, c: CharId, dir: V3, by: CharId) {
    if crate::condition::down(g, c) { return; }
    crate::character::anim_hit(g, c);
    let thump = v3::add(g.ch[c].pos, [0.0, 1.1, 0.0]);
    crate::audio_iface::game_audio_like_play(g, "bodyHit", Some(thump), Some(0.55), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.05 }));
    let back_m = g.k.ring.stagger_back;
    let p = g.ch[c].pos;
    let back: V3 = [p[0] - dir[0] * back_m, 0.0, p[2] - dir[2] * back_m];
    match g.ch[c].who {
        crate::character::Who::Player(pl) => g.pl[pl].hit_flash = 1.0,
        crate::character::Who::Guard(gd) if g.gd[gd].held.is_some() => return,
        crate::character::Who::Guard(gd) if g.gd[gd].civ.is_some() => crate::civilians::to_alarmed(g, gd, crate::civilians::CivCause::Noise, back, false),
        crate::character::Who::Guard(gd) => double_over(g, gd, back),
    }
    let level = g.k.ring.hit_noise;
    thump_at(g, c, level, by);
}

/// He doubles over: rooted, gun down, alert, with a fix where it came from; a yelp.
fn double_over(g: &mut Game, gd: GuardId, from: V3) {
    use crate::game::GuardState;
    let c = g.gd[gd].ch;
    if g.gd[gd].script.is_some() {
        use crate::game::GuardState as End;
        let to = match g.gd[gd].state { GuardState::Patrol => End::Suspicious, GuardState::Search => End::Search, GuardState::Alert => End::Alert, _ => End::Suspicious };
        crate::squad::end_guard_script(g, gd, Some(to));
    }
    let (stagger, fire_cd, aware) = (g.k.ring.stagger, g.k.ring.stagger_fire_cd, g.k.ring.stagger_aware);
    crate::condition::fold(g, c, stagger);
    g.gd[gd].react_t = m::max(g.gd[gd].react_t, stagger);
    g.arms[c].fire_cd = m::max(g.arms[c].fire_cd, fire_cd);
    let weapon = g.gd[gd].weapon;
    g.fa[weapon].burst = None;
    crate::condition::stopped_dead(g, gd);
    if g.gd[gd].state != GuardState::Alert || g.gd[gd].last_known.is_none() {
        let mut at = from;
        if crate::scene_collision::nav_grid_is_blocked(g, at[0], at[2]) {
            at = match crate::scene_collision::nav_grid_nearest_free_point(g, at[0], at[2]) { Some(q) => q, None => v3::copy(g.ch[c].pos) };
        }
        crate::belief::word(g, gd, at);
    }
    crate::mind::raise(g, gd, aware, "flight");
    if g.ai_enabled { g.gd[gd].witness.heard.small += 1.0; }
    let all = crate::launcher::stagger_lines_table();
    let now: Vec<String> = if g.gd[gd].state == GuardState::Alert { all.iter().filter(|t| !t.contains("who's there")).cloned().collect() } else { all.clone() };
    let pool = crate::words::w(g, &now);
    let roll = m::floor(crate::dice::draw_of(g, "launcher::stagger_guard#1", gd) * pool.len() as f64);
    let line = &pool[js::rem(roll, pool.len() as f64) as usize];
    crate::game::game_say(g, gd, line, None, None);
}
