pub use ::game_types::wand::types;
pub use types::*;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};

/// A door the wand goes under: shut home, on its hinges, not the steel fire door.
pub fn wand_fits(g: &crate::game::Game, d: DoorId) -> bool {
    !g.dr[d].leaf.off && m::abs(g.dr[d].leaf.angle) < g.k.wand.shut_rad && g.dr[d].def.steel != Some(true)
}

/// Is 'look under it' on this door's rows for this player.
pub fn wand_offered(g: &crate::game::Game, pl: PlayerId, d: DoorId) -> bool {
    g.pl[pl].wand && crate::fogofwar::fog_of_war_active(g) && wand_fits(g, d)
}

/// The wait before the lens is through: the squat's own blend in where it is drawn, else the kneel's.
fn wand_feed_secs(g: &crate::game::Game, squat: Option<bool>) -> f64 {
    if squat.unwrap_or(false) { g.k.stealthpose.cable_in } else { g.k.wand.quick_secs }
}

/// Is the lens through this step, for a man the caller has found planted. `t` went with the old feed.
pub fn wand_through(g: &crate::game::Game, age: f64, crouch: f64, squat: Option<bool>) -> bool {
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
    let Some(w) = g.pl[pl].wanding().copied() else { return };
    let l = wand_lens(g, w.door, w.side);
    let max = wand_pan_max(g);
    let pan = turned(w.pan, l, aim, max, g.k.wand.pan_rate, dt);
    if let Some(held) = g.pl[pl].wanding_mut() { held.pan = pan; }
}

/// wandTurn's arithmetic: the pan after this step.
fn turned(pan: f64, l: crate::wand::WandLens, aim: V3, max: f64, pan_rate: f64, dt: f64) -> f64 {
    let mut want = v3::yaw_to(l.pos, aim) - l.yaw;
    want = m::atan2(m::sin(want), m::cos(want));
    want = m::max(-max, m::min(max, want));
    let step = pan_rate * dt;
    let pan = pan + m::max(-step, m::min(step, want - pan));
    m::max(-max, m::min(max, pan))
}

/// The lens as one of the fog's eyes, or None while nobody is looking.
pub fn wand_eye(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<crate::fogofwar::FogEye> {
    let pl = seat.player;
    let w = g.pl[pl].wanding().copied()?;
    if !w.live || crate::condition::down(g, g.pl[pl].ch) { return None; }
    let l = wand_lens(g, w.door, w.side);
    let a = l.yaw + w.pan;
    let fov = wand_fov(g);
    let c = g.pl[pl].ch;
    Some(crate::fogofwar::FogEye { id: g.ch[c].id, pos: l.pos, look: [m::sin(a), 0.0, m::cos(a)], at: l.pos, fov: Some(fov), near: Some(0.0), wand: Some(true) })
}

/// The look let go of; `stop_wand` does this and stops him where he kneels.
pub fn wand_drop(g: &mut crate::game::Game, pl: PlayerId) {
    if g.pl[pl].wanding().is_none() { return; }
    g.pl[pl].wanding_over();
    let c = g.pl[pl].ch;
    g.ch[c].anim.lockpick = false;
    g.pl[pl].equip_t = m::max(g.pl[pl].equip_t, g.k.wand.stow_secs);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A failure means the lens comes up on the man's own side of the door, or looks back at him.
    #[test]
    fn the_lens_goes_under_the_leaf_s_middle_and_looks_out_the_far_side() {
        // a door along +x from (2, 3), half a metre to its middle
        let l = |side: f64| lens_at([2.0, 3.0], [1.0, 0.0], 0.5, side, 0.22, 0.1);
        assert_eq!(l(-1.0).pos, [2.5, 0.1, 3.22]);    // he is on the -1 face: it comes out on +z
        assert_eq!(l(-1.0).yaw, 0.0);                 // and looks along +z
        assert_eq!(l(1.0).pos, [2.5, 0.1, 2.78]);
        assert_eq!(m::abs(l(1.0).yaw), m::PI);
    }

    /// A failure means the view swings at once, past its stops, or the long way round.
    #[test]
    fn the_lens_turns_toward_the_aim_at_its_rate_inside_its_stops() {
        let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-12, "{a} {b}");
        let square = crate::wand::WandLens { pos: [0.0, 0.1, 0.0], yaw: 0.0 };
        let (max, rate) = (0.5, 1.6);
        let (right, left) = ([10.0, 0.0, 10.0], [-10.0, 0.0, 10.0]);
        near(turned(0.0, square, right, max, rate, 1.0 / 60.0), rate / 60.0);    // a step's worth
        near(turned(0.0, square, left, max, rate, 1.0 / 60.0), -rate / 60.0);
        near(turned(0.0, square, right, max, rate, 1.0), max);                   // the aim is past the stop: it stops there
        near(turned(0.49, square, [0.0, 0.0, 5.0], max, rate, 1.0), 0.0);        // and comes back square
        near(turned(0.6, square, right, max, rate, 1.0 / 60.0), max);            // a stop that closed in on it holds it
        // the aim just across the back of a lens that looks nearly behind: the short way
        let back = crate::wand::WandLens { pos: [0.0, 0.1, 0.0], yaw: 3.0 };
        let t = -3.0_f64;
        near(turned(0.0, back, [m::sin(t) * 5.0, 0.0, m::cos(t) * 5.0], max, rate, 1.0), 2.0 * m::PI - 6.0);
    }
}
