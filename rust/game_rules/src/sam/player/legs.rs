//! HIS LEGS: how his keys become a walk, a sprint or a crouch, and which way his body turns for it. What moves him also moves a copy of him (a look ahead), so it works on a `Legs`.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};

/// what of a Legs the door's sprint rule reads
fn door_stamps(m: &crate::player::Legs) -> crate::sprintdoor::DoorStamps {
    crate::sprintdoor::DoorStamps { down: m.down, door_rip_until: m.door_rip_until, door_bash_at: m.door_bash_at, door_bash_lean: m.door_bash_lean }
}
/// The player's own fields as a Legs: the tick hands `g.player` to what a look ahead hands its copy.
fn legs_of(g: &Game, pl: PlayerId) -> crate::player::Legs {
    let (p, c) = (&g.pl[pl], &g.ch[g.pl[pl].ch]);
    crate::player::Legs {
        char: crate::player::LegsChar { pos: c.pos, vel: c.vel, body_yaw: c.body_yaw, aim_yaw: c.aim_yaw },
        safe: p.safe, crouch: p.crouch, sprinting: p.sprinting, dash: p.dash, backpedal: p.backpedal, speed_sm: p.speed_sm, pace: p.pace,
        tow: p.tow, shoved: p.shoved, shoved_t: p.shoved_t, shoved_from: p.shoved_from,
        down: g.cond[p.ch].down, door_rip_until: p.door_rip_until, door_bash_at: p.door_bash_at, door_bash_lean: p.door_bash_lean,
    }
}
/// A call on the player's legs: lent to it as a Legs, and what the Legs lines write put back. Only those fields: what a callee
/// changed on the player through `g` (a body let go of, a door's stamp) stays as it left it.
pub(super) fn own_legs<R>(g: &mut Game, pl: PlayerId, f: impl FnOnce(&mut Game, &mut crate::player::Legs) -> R) -> R {
    let mut m = legs_of(g, pl);
    let r = f(g, &mut m);
    let c = g.pl[pl].ch;
    let ch = &mut g.ch[c];
    ch.pos = m.char.pos; ch.vel = m.char.vel; ch.body_yaw = m.char.body_yaw; ch.aim_yaw = m.char.aim_yaw;
    let p = &mut g.pl[pl];
    p.safe = m.safe; p.crouch = m.crouch; p.sprinting = m.sprinting; p.dash = m.dash; p.backpedal = m.backpedal; p.speed_sm = m.speed_sm;
    p.shoved = m.shoved; p.shoved_t = m.shoved_t; p.shoved_from = m.shoved_from;
    r
}
/// collideMan(g, m, y1) on a Legs: its stub takes the two fields it moves
fn collide_legs(g: &Game, m: &mut crate::player::Legs, y1: f64) {
    let mut pl = crate::player::CollideManPl { char: crate::player::CollideManPlChar { pos: m.char.pos }, safe: m.safe };
    collide_man(g, &mut pl, y1);
    m.char.pos = pl.char.pos; m.safe = pl.safe;
}

/// Is what moves him this tick his walk keys and nothing else. Asked, nothing written.
pub fn legs_own(g: &crate::game::Game) -> bool {
    let pl = g.player;
    let busy = crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].planted() || g.pl[pl].wall.is_some() || g.pl[pl].tow.is_some() || g.pl[pl].door_move().is_some();
    !(busy || crate::handwork::sam_work(g).is_some() || crate::stalk::stalk_on(g, &crate::seats::here(g)).is_some() || crate::rails::rails_allowed(g).is_some())
}

/// The top pace of a walk of his own this tick, by what his legs are doing and what his card weighs.
fn free_pace(g: &crate::game::Game, seat: &crate::seats::SeatRef, m: crate::player::Legs, ml: f64) -> f64 {
    let mut max_speed = if ml == 0.0 { 0.0 }
        else if m.sprinting { g.tune.player_sprint }
        else if let Some(t) = m.tow { if t.haul.is_some() { crate::hands::haul_speed(g, seat) } else { g.tune.player_crouch * 0.6 } }
        else if m.dash { g.k.crouchrun.pace }
        else { (if m.crouch { crouch_pace(g) } else { g.tune.player_walk }) * m.pace };
    crate::kitload::sam_load_pace(g, g.pl[seat.player].ch, max_speed, Some(m.sprinting))
}

/// the walk keys, ahead, left, back and right
pub const WALK_KEYS: [crate::input::Key; 4] = [crate::input::Key::W, crate::input::Key::A, crate::input::Key::S, crate::input::Key::D];
/// Which of them a hand holds to go from `from` toward `to`, in a view that looks along `forward`: the nearest of the eight ways they can go.
pub fn walk_keys(from: V3, to: [f64; 2], forward: V3) -> [bool; 4] {
    let (dx, dz) = (to[0] - from[0], to[1] - from[2]);
    let (d, l) = (m::hyp2(dx, dz), m::hyp2(forward[0], forward[2]));
    if d <= 0.0 || l <= 0.0 { return [false; 4]; }
    let (fx, fz) = (forward[0] / l, forward[2] / l);
    let (ahead, right) = ((dx * fx + dz * fz) / d, (dz * fx - dx * fz) / d);
    [ahead > 0.3827, right < -0.3827, ahead < -0.3827, right > 0.3827]
}

/// His move keys as a way across the floor, in the view's basis, not yet of unit length.
pub fn keys_way(fwd: V3, right: V3, w: bool, s: bool, d: bool, a: bool) -> [f64; 2] {
    let (mut mx, mut mz) = (0.0, 0.0);
    if w { mx += fwd[0]; mz += fwd[2]; }
    if s { mx -= fwd[0]; mz -= fwd[2]; }
    if d { mx += right[0]; mz += right[2]; }
    if a { mx -= right[0]; mz -= right[2]; }
    [mx, mz]
}

/// The way his walk keys point him now, in the view.
pub(super) fn keys_held(g: &crate::game::Game, input: InputId, cam: CameraId) -> [f64; 2] {
    let basis = crate::camera::follow_camera_planar_basis(g, cam);
    let [w, a, s, d] = WALK_KEYS.map(|k| crate::input::input_down(g, input, k));
    keys_way(basis.fwd, basis.right, w, s, d, a)
}

/// C: down, or up where there is the headroom; false: no room to stand.
pub fn crouch_key(g: &crate::game::Game, m: &mut crate::player::Legs) -> bool {
    if !m.crouch { m.crouch = true; }
    else if can_stand_at(g, m.char.pos) { m.crouch = false; }
    else { return false; }
    true
}

/// His stance for this stride: Shift and a direction stands him up to run, or keeps him down and quick; a body in his hands sets it.
pub fn sprint_stance(g: &crate::game::Game, m: &mut crate::player::Legs, want_sprint: bool, stalk: bool) {
    let dragging = m.tow.is_some();
    if dragging { m.crouch = crate::hands::haul_crouch(g); }
    m.sprinting = want_sprint && !m.crouch;
    m.dash = want_sprint && m.crouch && !dragging && !stalk;
}

/// One stride: his pace taken up or let go, his place moved by it, the walls and then the men given their say.
pub fn stride_step(g: &mut crate::game::Game, m: &mut crate::player::Legs, mx: f64, mz: f64, ml: f64, stalk: Option<crate::player::StrideStepStalk>, dt: f64, now: Option<f64>) {
    let now = now.unwrap_or(g.time);
    let mut max_speed = free_pace(g, &crate::seats::here(g), *m, ml);
    if let Some(s) = stalk { max_speed = if ml == 0.0 { 0.0 } else { s.pace }; }
    if m.sprinting { max_speed *= crate::sprintdoor::sprint_door_pace(g, door_stamps(m), Some(now)); }
    let rate = match stalk {
        Some(s) => if s.keep.unwrap_or(false) { 12.0 } else if max_speed > m.speed_sm { g.k.stalk.ease_up } else { g.k.stalk.ease_down },
        None => if max_speed > m.speed_sm { 6.0 } else { 12.0 },
    };
    m.speed_sm = v3::damp(m.speed_sm, max_speed, rate, dt);
    let moving = ml > 0.0;
    if moving { m.char.vel = [mx * m.speed_sm, 0.0, mz * m.speed_sm]; } else { m.char.vel = v3::scale(m.char.vel, m::exp(-14.0 * dt)); }
    m.char.pos = v3::mad(m.char.pos, m.char.vel, dt);
    let headroom = if m.crouch { 1.0 } else { 1.7 };
    collide_legs(g, m, headroom);
    keep_off_guards(g, dt, headroom, Some(m));
}

/// Is his gun up to aim: the pistol just fired or with its light on, the rifle braced.
pub fn gun_raised(g: &crate::game::Game, pl: PlayerId, now: Option<f64>, m: Option<crate::player::GunRaisedM>) -> bool {
    let now = now.unwrap_or(g.time);
    let sprinting = match m { Some(m) => m.sprinting, None => g.pl[pl].sprinting };
    let pistol = g.pl[pl].pistol;
    (crate::game::player_pistol_up(g, pl) && (now - g.pl[pl].last_fire_t < 1.6 || g.fa[pistol].light_on))
        || crate::game::player_braced_at(g, pl, sprinting)
}

/// The way a leaf he is pushing open turns him: square into its face, unless the gun is up or it merely rests on him.
pub fn leaf_facing(dp: Option<&crate::player::LeafFacingDp>, aiming: bool) -> Option<f64> {
    let dp = dp?;
    if dp.w >= 1.0 && !aiming { Some(m::atan2(-dp.n[0], -dp.n[2])) } else { None }
}

/// A sprint's facing: gun down, the body follows the run and the aim is the body's. Returns the shoulder's turn into a door just run open.
pub fn sprint_facing(g: &crate::game::Game, m: &mut crate::player::Legs, dt: f64, now: Option<f64>) -> Option<f64> {
    let now = now.unwrap_or(g.time);
    let lean = crate::sprintdoor::sprint_door_lean(g, door_stamps(m), Some(now));
    m.char.body_yaw = v3::approach_angle(m.char.body_yaw, v3::yaw_of(m.char.vel) + lean.unwrap_or(0.0), 14.0 * dt);
    m.char.aim_yaw = m.char.body_yaw;
    m.backpedal = false;
    lean
}

/// The hips of a walk of his own, his aim already turned; a leaf he is walking open or working by hand turns them instead.
pub fn hips_turn(m: &mut crate::player::Legs, speed: f64, aiming: bool, orbit: bool, into_door: Option<f64>, by_hand: bool, dt: f64) {
    if speed > 0.2 {
        let move_yaw = v3::yaw_of(m.char.vel);
        let d = m::cos(v3::wrap_angle(m.char.aim_yaw - move_yaw));
        if by_hand { m.backpedal = m::cos(v3::wrap_angle(into_door.unwrap() - move_yaw)) < 0.0; }
        else if into_door.is_some() { m.backpedal = false; }
        else if if m.backpedal { d > -0.05 } else { d < -0.35 } { m.backpedal = !m.backpedal; }
        let base = if m.backpedal { v3::wrap_angle(move_yaw + m::PI) } else { move_yaw };
        let body_target = if by_hand { into_door.unwrap() }
            else if let Some(door) = into_door { base + v3::clamp(v3::wrap_angle(door - base), -0.35, 0.35) }
            else { base + v3::clamp(v3::wrap_angle(m.char.aim_yaw - base), -0.8, 0.8) * (if aiming || orbit { 1.0 } else { 0.6 }) };
        m.char.body_yaw = v3::approach_angle(m.char.body_yaw, body_target, (if into_door.is_some() { 12.0 } else { 9.0 }) * dt);
    } else {
        m.backpedal = false;
        let toward = into_door.unwrap_or(m.char.aim_yaw);
        let off = v3::wrap_angle(toward - m.char.body_yaw);
        let limit = if by_hand { 0.1 } else if into_door.is_some() { 0.25 } else if aiming { 0.6 } else { 1.2 };
        if m::abs(off) > limit {
            m.char.body_yaw = v3::approach_angle(m.char.body_yaw, toward - m::sign(off) * limit * 0.7, (if by_hand { 12.0 } else { 7.0 }) * dt);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn legs(body: f64, aim: f64, vx: f64, vz: f64, backpedal: bool) -> crate::player::Legs {
        let mut m = crate::player::Legs::default();
        m.char.vel = [vx, 0.0, vz]; m.char.body_yaw = body; m.char.aim_yaw = aim; m.backpedal = backpedal;
        m
    }

    const LONG: f64 = 1.0;   // a frame long enough that his hips get where they are going

    /// Walking on his own, hips go the way he walks and lean toward the aim, further with the gun up.
    #[test]
    fn walking_his_hips_follow_his_way_and_lean_to_his_aim() {
        let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-12, "{a} {b}");
        let at = |aim: f64, aiming: bool| { let mut m = legs(0.0, aim, 0.0, 1.5, false); hips_turn(&mut m, 1.5, aiming, false, None, false, LONG); m.char.body_yaw };
        near(at(0.5, false), 0.3);
        near(at(0.5, true), 0.5);
        near(at(1.5, true), 0.8);     // no further round than the waist goes
        let mut m = legs(0.0, 0.5, 0.0, 1.5, false);
        hips_turn(&mut m, 1.5, true, false, None, false, 0.01);
        near(m.char.body_yaw, 0.09);  // and at a rate, not at once
    }

    /// A failure means he flickers between walking forward and backing up as the cursor crosses his flank: it takes a clear look behind to start backing up, and a clear look ahead to stop.
    #[test]
    fn he_backs_up_with_his_aim_behind_him_and_does_not_flicker() {
        let after = |aim: f64, back: bool| { let mut m = legs(0.0, aim, 0.0, 1.5, back); hips_turn(&mut m, 1.5, true, false, None, false, LONG); (m.backpedal, m.char.body_yaw) };
        let (back, body) = after(m::PI, false);
        assert!(back && (body.abs() - m::PI).abs() < 1e-9, "aim straight behind: back up facing it ({body})");
        assert!(!after(1.7, false).0, "a little behind the shoulder: still walking forward");
        assert!(after(1.7, true).0, "...and still backing up");
        assert!(!after(1.0, true).0, "a clear look ahead ends it");
    }

    /// A failure means he turns away from the leaf he works: walked open it pulls his hips a little and he never backs up; by hand he faces it, and backs up if he walks away from it.
    #[test]
    fn a_leaf_turns_his_hips_more_than_his_aim_does() {
        let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-12, "{a} {b}");
        let mut m = legs(0.0, 2.9, 0.0, 1.5, true);
        hips_turn(&mut m, 1.5, false, true, Some(1.0), false, LONG);
        assert!(!m.backpedal);
        near(m.char.body_yaw, 0.35);
        let mut m = legs(0.0, 2.9, 0.0, 1.5, false);
        hips_turn(&mut m, 1.5, false, false, Some(1.3), true, LONG);
        assert!(!m.backpedal);
        near(m.char.body_yaw, 1.3);
        let mut m = legs(0.0, 2.9, 0.0, 1.5, false);
        hips_turn(&mut m, 1.5, false, false, Some(3.0), true, LONG);
        assert!(m.backpedal);
    }

    /// Standing, his hips stay until the aim has gone past what the waist takes, less with the gun up; then they come round short of it.
    #[test]
    fn standing_his_hips_stay_until_the_aim_passes_the_waist() {
        let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-12, "{a} {b}");
        let at = |aim: f64, aiming: bool, into: Option<f64>, hand: bool| { let mut m = legs(0.0, aim, 0.0, 0.0, true); hips_turn(&mut m, 0.0, aiming, false, into, hand, LONG); assert!(!m.backpedal); m.char.body_yaw };
        assert_eq!(at(1.0, false, None, false), 0.0);
        near(at(1.0, true, None, false), 1.0 - 0.6 * 0.7);
        near(at(2.0, false, None, false), 2.0 - 1.2 * 0.7);
        assert_eq!(at(0.0, false, Some(0.2), false), 0.0);
        near(at(0.0, false, Some(0.4), false), 0.4 - 0.25 * 0.7);
        near(at(0.0, false, Some(0.4), true), 0.4 - 0.1 * 0.7);
    }

    #[test]
    fn his_keys_add_up_and_opposite_keys_cancel() {
        let (fwd, right) = ([0.5, 0.0, 0.75], [0.75, 0.0, -0.5]);
        assert_eq!(keys_way(fwd, right, true, false, true, false), [1.25, 0.25]);
        assert_eq!(keys_way(fwd, right, false, true, false, true), [-1.25, -0.25]);
        assert_eq!(keys_way(fwd, right, true, true, true, true), [0.0, 0.0]);
        assert_eq!(keys_way(fwd, right, false, false, false, false), [0.0, 0.0]);
    }

    #[test]
    fn a_leaf_turns_him_square_into_it_only_when_it_pushes() {
        let dp = |w| crate::player::LeafFacingDp { w, n: vec![1.0, 0.0, 0.0] };
        assert!((leaf_facing(Some(&dp(1.0)), false).unwrap() + m::PI / 2.0).abs() < 1e-12, "a yaw of minus a quarter faces -x, into a face whose normal is +x");
        assert_eq!(leaf_facing(Some(&dp(0.5)), false), None);    // it merely rests on him
        assert_eq!(leaf_facing(Some(&dp(1.0)), true), None);     // the gun is up
        assert_eq!(leaf_facing(None, false), None);
    }
}
