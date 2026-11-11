//! WHAT A PLAYER DID, handed to the game before a step.
use super::Held;
use game::cmd::{PlayerCmd, PlayerCmdView};
use game::view::HandsSaid;
use game_start::hands_off::{HandsOff, Plan};

/// the numbers of a hand: buttons clicked wheel, cursor x y, the view's size in CSS pixels and in its own, the view-projection and its inverse, the eye, where it
/// looks, the look-at, yaw pitch distance, field of view, aspect, how far it leads the aim, and the distance it is on its way to
const HAND: usize = 57;
pub fn hand(x: &mut Held, seat: u32, v: Vec<f64>, keys: &str) {
    let seat = seat as usize;
    assert!(v.len() == HAND, "hand: {} numbers, and a hand is {}", v.len(), HAND);
    let mut lines = keys.split('\n');
    let mut codes = || codes(lines.next().unwrap_or(""));
    let three = |i: usize| [v[i], v[i + 1], v[i + 2]];
    let mut cmd = PlayerCmd {
        keys: codes(), pressed: codes(), buttons: v[0], clicked: v[1], wheel: v[2], mx: v[3], my: v[4], w: v[5], h: v[6], pw: v[7], ph: v[8],
        view: PlayerCmdView { vp: v[9..25].to_vec(), ivp: v[25..41].to_vec(), pos: three(41), forward: three(44), target: three(47), yaw: v[50], pitch: v[51], distance: v[52], fov: v[53], aspect: v[54], aim_lead: v[55] },
    };
    // (the host's own camera, as the hand has it: what a rule asks of the host's view, and where one writes to it)
    if let (true, Some(c)) = (x.g.seats[seat] == game::game::game_host_seat(&x.g), x.g.host_cam) {
        x.drawn.desired = v[56];
        let (c, v) = (&mut x.g.tab.camera[c], &cmd.view);
        (c.target, c.pos, c.forward) = (v.target, v.pos, v.forward);
        (c.yaw, c.pitch, c.distance, c.desired_distance, c.fov, c.aspect) = (v.yaw, v.pitch, v.distance, x.drawn.desired, v.fov, v.aspect);
    }
    if let Some((_, h)) = x.hands.as_mut().filter(|h| h.0 == seat) { h.hand(&x.g, &mut cmd, 1.0 / 60.0); }
    x.hand_over(seat, cmd);
}
/// a line of keys' codes
pub fn codes(line: &str) -> Vec<String> { line.split(' ').filter(|k| !k.is_empty()).map(String::from).collect() }

pub fn commands(x: &mut Held, list: &[u8]) -> Result<(), String> {
    let done = game_start::host::command::apply(&mut x.g, list);
    // (a restart done between two ticks: the picture of the game before it is not blended into this one's)
    if x.g.restart_cut { x.drawn.cut(); }
    done
}
pub fn hands_off(x: &mut Held, seat: u32, seed: u32, cap: f64, breaker: bool, route: Vec<&str>) {
    let plan = Plan { seed, cap, breaker, route: route.into_iter().map(String::from).collect() };
    x.hands = Some((seat as usize, HandsOff::new(x.g.seats[seat as usize], plan)));
}
pub fn hands_on(x: &mut Held) { x.hands = None; }
pub fn hands_said(x: &Held) -> Option<HandsSaid> {
    let (seat, p) = x.hands.as_ref()?;
    let g = &x.g;
    let dead = |m: &game::ids::GuardId| g.gd[*m].state == game::game::GuardState::Dead;
    let squad = game::orders::squad_orders_squad(g, &game::seats::seat(g, g.seats[*seat]));
    let left = g.guards.iter().filter(|&m| g.team(*m) == game::teams::Team::Hostile && !dead(m)).count();
    Some(HandsSaid { t: p.t(), end: p.end(), given: p.given(), shot_at: p.shot_at(), up: squad.iter().filter(|m| !dead(m)).count() as u32, of: squad.len() as u32, left: left as u32 })
}
pub fn hands_trace(x: &Held) -> String { x.hands.as_ref().map_or(String::new(), |p| p.1.trace()) }
