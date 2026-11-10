//! WHAT A PLAYER DID, handed to the game before a step.
//!
//!   face_hand(h, seat, p, pn, keys, n) WHAT A PLAYER DID, for the next step: `HAND` numbers (`pn` bytes; src/draw/hand.ts says them in the same order) and his keys' codes as
//!                                      text, those held and, after a line feed, those that went down. A seat nobody hands is as it last was, with
//!                                      nothing pressed.
//!   face_commands(h, p, n) -> 1 | 0    WHAT A PLAYER OR A PANEL DID THAT IS NOT INPUT, before a step: the typed list (game_start::host::command), done in its order.
//!                                      0 = a number no command has (said at face_out)
//!   face_hands_off(h, seat, p, n) · face_hands_said(h) · face_hands_trace(h)   nobody's hands are on a seat (game_start hands_off): his plan, how he stands, his timeline

/// the numbers of a hand: buttons clicked wheel, cursor x y, the view's size in CSS pixels and in its own, the view-projection and its inverse, the eye, where it
/// looks, the look-at, yaw pitch distance, field of view, aspect, how far it leads the aim, and the distance it is on its way to
const HAND: usize = 57;
/// # Safety
/// `p`, `pn` and `keys`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_hand(h: u32, seat: usize, p: *const f64, pn: usize, keys: *const u8, n: usize) {
    use game::cmd::{PlayerCmd, PlayerCmdView};
    if pn != HAND * 8 { panic!("face_hand: {} bytes, and a hand is {} numbers", pn, HAND); }
    let v = std::slice::from_raw_parts(p, HAND);
    let text = super::text(keys, n);
    let mut lines = text.split('\n');
    let mut codes = || lines.next().unwrap_or("").split(' ').filter(|k| !k.is_empty()).map(String::from).collect::<Vec<String>>();
    let three = |i: usize| [v[i], v[i + 1], v[i + 2]];
    let cmd = PlayerCmd {
        keys: codes(), pressed: codes(), buttons: v[0], clicked: v[1], wheel: v[2], mx: v[3], my: v[4], w: v[5], h: v[6], pw: v[7], ph: v[8],
        view: PlayerCmdView { vp: v[9..25].to_vec(), ivp: v[25..41].to_vec(), pos: three(41), forward: three(44), target: three(47), yaw: v[50], pitch: v[51], distance: v[52], fov: v[53], aspect: v[54], aim_lead: v[55] },
    };
    super::with(h, |x| {
        // (the host's own camera, as the hand has it: what a rule asks of the host's view, and where one writes to it)
        if let (true, Some(c)) = (x.g.seats[seat] == game::game::game_host_seat(&x.g), x.g.host_cam) {
            x.drawn.desired = v[56];
            let (c, v) = (&mut x.g.tab.camera[c], &cmd.view);
            (c.target, c.pos, c.forward) = (v.target, v.pos, v.forward);
            (c.yaw, c.pitch, c.distance, c.desired_distance, c.fov, c.aspect) = (v.yaw, v.pitch, v.distance, x.drawn.desired, v.fov, v.aspect);
        }
        let mut cmd = cmd;
        if x.hands.as_ref().is_some_and(|h| h.0 == seat) {
            if let Some((_, h)) = &mut x.hands { h.hand(&x.g, &mut cmd, 1.0 / 60.0); }
        }
        x.hand_over(seat, cmd);
    });
}
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_commands(h: u32, p: *const u8, n: usize) -> u32 {
    let b = super::bytes(p, n);
    // (a restart done between two ticks: the picture of the game before it is not blended into this one's)
    match super::with(h, |x| { let done = game_start::host::command::apply(&mut x.g, b); if x.g.restart_cut { x.drawn.cut(); } done }) { Ok(()) => 1, Err(why) => { super::out(why.into_bytes()); 0 } }
}
/// NOBODY'S HANDS ARE ON A SEAT (game_start hands_off): the hands it is handed from here are his own. `p`, `n`: his plan, a word a line: the seed, the seconds he plays, 1 to throw the breaker
/// first, then doors by name to take first. An empty plan: the seat is the host's again.
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_hands_off(h: u32, seat: usize, p: *const u8, n: usize) {
    let text = super::text(p, n);
    let w: Vec<&str> = text.split('\n').collect();
    super::with(h, |x| x.hands = if text.is_empty() { None } else {
        let num = |i: usize| w.get(i).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
        let plan = game_start::hands_off::Plan { seed: num(0) as u32, cap: num(1), breaker: num(2) != 0.0, route: w.iter().skip(3).map(|s| String::from(*s)).collect() };
        Some((seat, game_start::hands_off::HandsOff::new(x.g.seats[seat], plan)))
    });
}
/// how he stands, as JSON at face_out (empty: nobody is hands off): seconds played, how it ended, orders given, when a round last left a gun, the squad's men up and of how many, defenders left
#[no_mangle]
pub extern "C" fn face_hands_said(h: u32) {
    let s = super::with(h, |x| match &x.hands { None => String::new(), Some((seat, p)) => {
        let g = &x.g;
        let squad = game::orders::squad_orders_squad(g, &game::seats::seat(g, g.seats[*seat]));
        let up = squad.iter().filter(|&&m| g.gd[m].state != game::game::GuardState::Dead).count();
        format!("{{\"t\":{},\"end\":{},\"given\":{},\"shotAt\":{},\"up\":{},\"of\":{},\"left\":{}}}", p.t(), p.end().map_or(String::from("null"), |e| format!("\"{}\"", e.as_str())), p.given(), p.shot_at(), up, squad.len(),
            g.guards.iter().filter(|&&m| g.team(m) == game::teams::Team::Hostile && g.gd[m].state != game::game::GuardState::Dead).count())
    } });
    super::out(s.into_bytes());
}
/// his timeline, a line an entry, at face_out
#[no_mangle]
pub extern "C" fn face_hands_trace(h: u32) { let s = super::with(h, |x| x.hands.as_ref().map_or(String::new(), |p| p.1.trace())); super::out(s.into_bytes()); }
