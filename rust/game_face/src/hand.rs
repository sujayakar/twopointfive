//! WHAT A PLAYER DID, handed to the game before a step.
use super::Held;
use game::cmd::{PlayerCmd, PlayerCmdView};
use game::handed::{SeatInput, SeatLens};
use game::view::HandsSaid;
use game_start::hands_off::{HandsOff, Plan};
use game_start::host::field::Laid;

/// a record of what a seat's hands did; with no lens of his, his eye and his matrices are left to be worked out
pub fn cmd(i: &SeatInput, lens: Option<&SeatLens>, keys: &str) -> PlayerCmd {
    let codes = |line: Option<&str>| line.unwrap_or("").split(' ').filter(|k| !k.is_empty()).map(String::from).collect::<Vec<String>>();
    let mut lines = keys.split('\n');
    let (vp, ivp, pos, forward) = lens.map_or((vec![f64::NAN; 16], vec![f64::NAN; 16], [0.0; 3], [0.0; 3]), |l| (l.vp.to_vec(), l.ivp.to_vec(), l.pos, l.forward));
    PlayerCmd {
        keys: codes(lines.next()), pressed: codes(lines.next()), buttons: i.buttons, clicked: i.clicked, wheel: i.wheel, mx: i.mx, my: i.my, w: i.w, h: i.h, pw: i.pw, ph: i.ph,
        view: PlayerCmdView { vp, ivp, pos, forward, target: i.target, yaw: i.yaw, pitch: i.pitch, distance: i.distance, fov: i.fov, aspect: i.aspect, aim_lead: i.aim_lead },
    }
}
pub fn hand(x: &mut Held, seat: u32, Laid(input): Laid<SeatInput>, Laid(lens): Laid<SeatLens>, keys: &str) {
    let (seat, mut cmd) = (seat as usize, cmd(&input, Some(&lens), keys));
    // (the host's own camera, as the hand has it: what a rule asks of the host's view, and where one writes to it)
    if let (true, Some(c)) = (x.g.seats[seat] == game::game::game_host_seat(&x.g), x.g.host_cam) {
        x.drawn.desired = lens.desired;
        let (c, v) = (&mut x.g.tab.camera[c], &cmd.view);
        (c.target, c.pos, c.forward) = (v.target, v.pos, v.forward);
        (c.yaw, c.pitch, c.distance, c.desired_distance, c.fov, c.aspect) = (v.yaw, v.pitch, v.distance, x.drawn.desired, v.fov, v.aspect);
    }
    if let Some((_, h)) = x.hands.as_mut().filter(|h| h.0 == seat) { h.hand(&x.g, &mut cmd, 1.0 / 60.0); }
    x.hand_over(seat, cmd);
}
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
