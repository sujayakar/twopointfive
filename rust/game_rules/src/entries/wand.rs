pub use ::game_types::wand::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};

/// A door the wand goes under: shut home, on its hinges, not the steel fire door.
pub fn wand_fits(g: &crate::game::Game, d: DoorId) -> bool {
    !crate::doors::door_off(g, d) && m::abs(crate::doors::door_angle(g, d)) < g.k.wand.shut_rad && g.dr[d].def.steel != Some(true)
}

/// Is 'look under it' on this door's rows for this player.
pub fn wand_offered(g: &crate::game::Game, pl: PlayerId, d: DoorId) -> bool {
    g.pl[pl].wand && crate::fogofwar::fog_of_war_active(g) && wand_fits(g, d)
}

/// The wait before the lens is through: the squat's own blend in where it is drawn, else the kneel's.
pub fn wand_feed_secs(g: &crate::game::Game, squat: Option<bool>) -> f64 {
    if squat.unwrap_or(false) { g.k.stealthpose.cable_in } else { g.k.wand.quick_secs }
}

/// Is the lens through this step, for a man the caller has found planted. `t` went with the old feed.
pub fn wand_through(g: &crate::game::Game, t: f64, age: f64, crouch: f64, squat: Option<bool>) -> bool {
    age >= wand_feed_secs(g, squat) - 1e-6 && crouch >= g.k.wand.quick_down
}

/// The wedge in force, degrees.
pub fn wand_fov(g: &crate::game::Game) -> f64 {
    let wedge = if g.k.fog.cone { crate::fogofwar::field_deg(g) } else { g.k.wand.fov_deg };
    m::min(170.0, m::max(20.0, wedge))
}

/// The most the lens turns off square, rad.
pub fn wand_pan_max(g: &crate::game::Game) -> f64 {
    m::max(0.0, 90.0 - wand_fov(g) * 0.5 - g.k.wand.blind_deg) * m::PI / 180.0
}

/// Where the lens is and the way it faces square on: the leaf's middle, `pastM` out of the face he is NOT on.
pub fn wand_lens(g: &crate::game::Game, d: DoorId, side: f64) -> crate::wand::WandLens {
    let u = crate::doors::door_dir(g, d);
    lens_at(g.dr[d].def.hinge, u, g.dr[d].def.width * 0.5, side, g.k.wand.past_m, g.k.wand.eye_y)
}

/// wandLens's arithmetic.
fn lens_at(hinge: [f64; 2], u: [f64; 2], w: f64, side: f64, past_m: f64, eye_y: f64) -> crate::wand::WandLens {
    let [hx, hz] = hinge;
    let nx = u[1] * side;
    let nz = -u[0] * side;
    crate::wand::WandLens { pos: [hx + u[0] * w + nx * past_m, eye_y, hz + u[1] * w + nz * past_m], yaw: m::atan2(nx, nz) }
}

/// Turn the lens of this player's look toward the frame's aim point, at its rate, inside its stops.
pub fn wand_turn(g: &mut crate::game::Game, pl: PlayerId, aim: V3, dt: f64) {
    let Some(w) = g.pl[pl].wanding else { return };
    let l = wand_lens(g, w.door, w.side);
    let max = wand_pan_max(g);
    let pan = turned(w.pan, l, aim, max, g.k.wand.pan_rate, dt);
    if let Some(held) = g.pl[pl].wanding.as_mut() { held.pan = pan; }
}

/// wandTurn's arithmetic: the pan after this step.
fn turned(pan: f64, l: crate::wand::WandLens, aim: V3, max: f64, pan_rate: f64, dt: f64) -> f64 {
    let mut want = m::atan2(aim[0] - l.pos[0], aim[2] - l.pos[2]) - l.yaw;
    want = m::atan2(m::sin(want), m::cos(want));
    want = m::max(-max, m::min(max, want));
    let step = pan_rate * dt;
    let pan = pan + m::max(-step, m::min(step, want - pan));
    m::max(-max, m::min(max, pan))
}

/// The lens as one of the fog's eyes, or None while nobody is looking.
pub fn wand_eye(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<crate::fogofwar::FogEye> {
    let pl = seat.player;
    let w = g.pl[pl].wanding?;
    if !w.live || g.pl[pl].down { return None; }
    let l = wand_lens(g, w.door, w.side);
    let a = l.yaw + w.pan;
    let fov = wand_fov(g);
    let c = g.pl[pl].ch;
    Some(crate::fogofwar::FogEye { id: g.ch[c].id, pos: l.pos, look: [m::sin(a), 0.0, m::cos(a)], at: l.pos, fov: Some(fov), near: Some(0.0), wand: Some(true) })
}

/// The look let go of with nothing said; player.ts stopWand is the worded way out.
pub fn wand_drop(g: &mut crate::game::Game, pl: PlayerId) {
    if g.pl[pl].wanding.is_none() { return; }
    g.pl[pl].wanding = None;
    let c = g.pl[pl].ch;
    g.ch[c].anim.lockpick = false;
    g.pl[pl].equip_t = m::max(g.pl[pl].equip_t, g.k.wand.stow_secs);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Bits printed by src/game/wand.ts itself under bun with EXACT.on.
    fn b(x: u64) -> f64 { f64::from_bits(x) }
    const U_A: [u64; 2] = [0x3fe87996529f9d92, 0x3fe49d6e694619b8]; // cos 0.7, sin 0.7
    const PAST_M: f64 = 0.22;
    const EYE_Y: f64 = 0.1;
    const PAN_RATE: f64 = 1.6;

    fn door_a(side: f64) -> crate::wand::WandLens { lens_at([3.25, -7.5], [b(U_A[0]), b(U_A[1])], 0.95 * 0.5, side, PAST_M, EYE_Y) }
    fn door_b(side: f64) -> crate::wand::WandLens { lens_at([-11.125, 2.0625], [-0.6, 0.8], 1.2 * 0.5, side, PAST_M, EYE_Y) }
    fn bits(l: crate::wand::WandLens) -> [u64; 4] { [l.pos[0].to_bits(), l.pos[1].to_bits(), l.pos[2].to_bits(), l.yaw.to_bits()] }

    #[test]
    fn the_lens_has_the_typescripts_bits() {
        assert_eq!(bits(door_a(1.0)), [0x400e0a4c155c91e2, 0x3fb999999999999a, 0xc01d72f4c738b50c, 0x40038861baaa937e]);
        assert_eq!(bits(door_a(-1.0)), [0x400bc5c79e42f056, 0x3fb999999999999a, 0xc01c1a594f7a2b1a, 0xbfe6666666666666]);
        assert_eq!(bits(door_b(1.0)), [0xc0269e353f7ced91, 0x3fb999999999999a, 0x400565604189374c, 0x3fedac670561bb50]);
        assert_eq!(bits(door_b(-1.0)), [0xc027526e978d4fdf, 0x3fb999999999999a, 0x400348b439581062, 0xc001b6e192ebbe44]);
    }

    #[test]
    fn the_turn_has_the_typescripts_bits() {
        let max = m::max(0.0, 90.0 - 90.0 * 0.5 - 10.0) * m::PI / 180.0;
        assert_eq!(turned(0.0, door_a(1.0), [9.0, 0.0, 4.0], max, PAN_RATE, 1.0 / 60.0).to_bits(), 0xbf9b4e81b4e81b4f);
        assert_eq!(turned(0.3, door_a(-1.0), [-20.0, 1.0, -3.0], max, PAN_RATE, 1.0 / 60.0).to_bits(), 0x3fd17e4b17e4b17e);
        assert_eq!(turned(-0.2, door_b(1.0), [40.0, 0.0, 40.0], max, PAN_RATE, 0.5).to_bits(), 0x3f8ddba57d757380);
        assert_eq!(turned(0.61, door_b(-1.0), [-11.0, 0.0, 2.0], max, PAN_RATE, 1.0 / 120.0).to_bits(), 0x3fe317e4b17e4b18);
    }
}
