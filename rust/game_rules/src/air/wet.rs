pub use ::game_types::wet::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use alloc::vec::Vec;

/// A man's own bookkeeping, made the first time he is asked for.
fn mem_at(g: &mut Game, c: CharId) -> crate::wet::WetMem {
    if !g.m.wet.mem.has(c) {
        g.m.wet.mem.set(c, crate::wet::WetMem { sky_t: -1.0, open: true, prints_left: 0.0, last_out_wet_t: -1e9, drip_acc: 0.0 });
    }
    *g.m.wet.mem.get(c).unwrap()
}

/// how high his head is, as the rule reads it
fn head_at(g: &Game, c: CharId) -> f64 {
    let ch = &g.ch[c];
    let head = ch.bones.get(crate::character::Bone::Head).map(|b| b[1]);
    if ch.ragdoll.is_some() { m::max(0.3, head.unwrap_or(0.3)) } else { head.unwrap_or(ch.pos[1] + 1.6) }
}

/// Once a tick, after the weather has written this tick's rain: soak / dry everybody. Sam's drips and prints after it (samAftermath) tell the host.
pub fn update_wetness(g: &mut crate::game::Game, dt: f64) {
    if dt <= 0.0 { return; }
    crate::weather::weather_roofs_now(g);
    // everyCharacter: Sam (every seat's man), then the guards, living or not
    let mut all: Vec<CharId> = crate::seats::men(g).into_iter().map(|p| g.pl[p].ch).collect();
    all.extend(g.guards.iter().map(|gd| g.gd[*gd].ch));
    for c in all {
        let mem = mem_at(g, c);
        let head = head_at(g, c);
        wet_step(g, c, head, mem, dt);
    }
    crate::seats::each_seat(g, &mut |v: &mut Game| sam_aftermath(v, dt));
}

/// how high a sky's ray goes, how far round him a forearm is, and how long an answer of the sky holds
const SKY_TOP: f64 = 9.0;
const SKY_RING: f64 = 0.35;
const WET_SKY_SECS: f64 = 0.25;
const SKY_OFFSETS: [[f64; 2]; 5] = [[0.0, 0.0], [SKY_RING, 0.0], [-SKY_RING, 0.0], [0.0, SKY_RING], [0.0, -SKY_RING]];

/// Is a man under the open sky: outside every footprint, and most of what is over him and a forearm round him is sky. The answer holds a quarter of a second a man.
fn open_sky(g: &Game, c: CharId, head: f64, mem: &mut WetMem) -> bool {
    let (x, z) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    if ::sim::weather::under_any(&g.weather.roof_rects, x, z) { return false; }
    let time = g.time;
    if time - mem.sky_t >= WET_SKY_SECS || mem.sky_t < 0.0 || time < mem.sky_t {
        mem.sky_t = time;
        let covered = SKY_OFFSETS.iter().filter(|o| ::sim::lines::solid_line(g.tab.boxes.world(), true, ::sim::lines::ASK_GLASS, x + o[0], head + 0.15, z + o[1], x + o[0], SKY_TOP, z + o[1], &g.k) == Some(true)).count();
        mem.open = covered < 3;
    }
    mem.open
}

/// One man soaked or dried by this tick's weather.
fn wet_step(g: &mut Game, c: CharId, head: f64, mem: WetMem, dt: f64) {
    let rain = m::max(0.0, g.weather.rain_now);
    let mut mem = mem;
    let wetting = rain > 0.001 && open_sky(g, c, head, &mut mem);
    *g.m.wet.mem.get_mut(c).unwrap() = mem;
    let (k, ch) = (&g.k.wet, &mut g.ch[c]);
    let runoff = m::max(k.bead_runoff_secs, 0.1);
    if wetting {
        ch.wet_line = f64::INFINITY;
        let soak = k.soak_per_sec;
        ch.wet = v3::clamp(ch.wet + soak * (0.4 + 0.6 * rain) * dt, 0.0, 1.0);
        let beads_to = v3::clamp(0.35 + 0.65 * rain, 0.0, 1.0);
        ch.beads = if ch.beads < beads_to { m::min(beads_to, ch.beads + 3.0 * soak * (0.4 + 0.6 * rain) * dt) } else { m::max(beads_to, ch.beads - dt / runoff) };
        ch.wet_rain = ch.wet_rain + (rain - ch.wet_rain) * m::min(1.0, dt * 2.0);
    } else {
        ch.wet = v3::clamp(ch.wet - k.dry_per_sec * dt, 0.0, 1.0);
        ch.beads = v3::clamp(ch.beads - dt / runoff, 0.0, 1.0);
    }
    if ch.wet <= 0.0 && ch.beads <= 0.0 { (ch.wet_rain, ch.wet_line) = (0.0, f64::INFINITY); }
}

/// is `c` standing under the open sky, by his sky's memory.
fn open_sky_of(g: &mut Game, c: CharId) -> bool {
    let mem = mem_at(g, c);
    crate::weather::weather_roofs_now(g);
    let head = head_at(g, c);
    let mut mem = mem;
    let open = open_sky(g, c, head, &mut mem);
    *g.m.wet.mem.get_mut(c).unwrap() = mem;
    open
}

/// Sam indoors after the rain: his print budget and his drip clock. The drops themselves are GPU particles (`drip` below: nothing with no particle
/// system), but the ask of the sky is the rule's and moves his sky's memory, which the next soak reads.
fn sam_aftermath(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    mem_at(g, c);
    let raining = m::max(0.0, g.weather.rain_now) > 0.001 && open_sky_of(g, c);
    if raining {
        if g.ch[c].wet > 0.5 {
            let (prints, now) = (g.k.wet.prints, g.time);
            let m = g.m.wet.mem.get_mut(c).unwrap();
            m.prints_left = prints;
            m.last_out_wet_t = now;
        }
        return;
    }
    let (beads, wet, down, now, secs) = (g.ch[c].beads, g.ch[c].wet, crate::condition::down(g, g.pl[pl].ch), g.time, g.k.wet.drip_secs);
    let last = g.m.wet.mem.get(c).unwrap().last_out_wet_t;
    if beads > 0.02 && wet > 0.3 && now - last < secs && !down {
        let mut acc = g.m.wet.mem.get(c).unwrap().drip_acc + dt * (2.0 + 10.0 * beads);
        while acc >= 1.0 { acc -= 1.0; drip(g, c); }
        g.m.wet.mem.get_mut(c).unwrap().drip_acc = acc;
    } else {
        g.m.wet.mem.get_mut(c).unwrap().drip_acc = 0.0;
    }
}

/// where a drop leaves him, dealt in turn
const DRIP_FROM: [crate::character::Bone; 5] = [crate::character::Bone::HandL, crate::character::Bone::HandR, crate::character::Bone::Head, crate::character::Bone::Hips, crate::character::Bone::Hips];

/// One drop off him: from a hand, the chin or the hem of the jacket.
fn drip(g: &mut crate::game::Game, c_2: CharId) {
    let n = g.m.wet.drip_n;
    g.m.wet.drip_n = n + 1.0;
    let k = DRIP_FROM[js::rem(n, 5.0) as usize];
    let Some(b) = g.ch[c_2].bone(k) else { return; };
    let j = g.m.wet.drip_n * 0.618034;
    let down = match k { crate::character::Bone::Head => -0.12, crate::character::Bone::Hips => -0.05, _ => -0.06 };
    let off: V3 = [m::cos(j * 6.2831853) * 0.09, down, m::sin(j * 6.2831853) * 0.09];
    let (vel, id) = (g.ch[c_2].vel, g.ch[c_2].id);
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, v3::add(b, off), [0.0, -1.0, 0.0], 1.0, crate::render_particles::EmitOpts {
        speed: 0.4,
        speed_jit: Some(0.5),
        spread: Some(0.15),
        life: 0.55,
        life_jit: Some(0.3),
        size: 0.0045,
        size_jit: Some(0.3),
        vadd: Some(vel),
        owner: Some(id),
        color: Some([0.42, 0.47, 0.55]),
        ..crate::render_particles::EmitOpts::default()
    });
}

/// A footfall of Sam's: if he has come in wet, a print under the foot that landed, fainter each step. The foot ahead along his motion wins over `left_hint`
/// whenever he is moving.
pub fn wet_footfall(g: &mut crate::game::Game, left_hint: bool) {
    use crate::character::Bone;
    let pl = g.player;
    let c = g.pl[pl].ch;
    let prints_left = mem_at(g, c).prints_left;
    if prints_left <= 0.0 || g.ch[c].wet < 0.15 { return; }
    if m::max(0.0, g.weather.rain_now) > 0.001 && open_sky_of(g, c) { return; } // still out in it: the ground is wetter than his soles
    let bone = |g: &Game, b: Bone| g.ch[c].bone(b);
    let mut left = left_hint;
    if let (Some(fl), Some(fr)) = (bone(g, Bone::FootL), bone(g, Bone::FootR)) {
        let vel = g.ch[c].vel;
        if m::abs(vel[0]) + m::abs(vel[2]) > 0.2 { left = (fl[0] - fr[0]) * vel[0] + (fl[2] - fr[2]) * vel[2] > 0.0; }
    }
    let (Some(foot), Some(toe)) = (bone(g, if left { Bone::FootL } else { Bone::FootR }), bone(g, if left { Bone::ToeL } else { Bone::ToeR })) else { return; };
    let k = (prints_left / m::max(1.0, g.k.wet.prints)) * v3::clamp(g.ch[c].wet * 1.3, 0.0, 1.0); // strength: the budget left × how wet he still is
    g.m.wet.mem.get_mut(c).unwrap().prints_left = prints_left - 1.0;
    let dir = v3::normalize([toe[0] - foot[0], 0.0, toe[2] - foot[2]]);
    let mut centre: V3 = [foot[0] * 0.45 + toe[0] * 0.55, 0.005, foot[2] * 0.45 + toe[2] * 0.55];
    let hit = crate::scene_collision::raycast(g, [centre[0], 0.4, centre[2]], [0.0, -1.0, 0.0], 0.8);
    let y = match hit { Some(h) => 0.4 - h.t, None => 0.0 };
    centre[1] = y + 0.003;
    // seed bit 0 = which foot (the sole is mirrored); a print's look, and how it dries, are the mark's own (render_decals set.rs)
    let seed = if left { 0 } else { 1 };
    crate::render_decals::decals_add(g, crate::render_decals::DecalKind::Wetprint, centre, [0.0, 1.0, 0.0], Some(0.17), Some(crate::render_decals::DecalOpts {
        tangent: Some(dir),
        param: Some(k),
        look: Some(true),
        seed: Some(seed as f64),
        ..crate::render_decals::DecalOpts::default()
    }));
}
