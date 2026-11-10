//! PASS: one man through a doorway at a time. The pace a man walks his path at by a door, from who else of his is at it.
use super::*;

/// A man on his walk by a door, as the gate takes him this frame.
struct Walker {
    gd: GuardId,
    c: CharId,
    id: f64,
    pos: V3,
    dt: f64,
    speed: f64,
    /// Nobody else has his facing.
    turn: bool,
    /// The frame he is for, if any.
    to: Option<DoorAhead>,
    /// He waits off a jamb of it.
    waiting: bool,
    /// He is by a door at all.
    by_door: bool,
    heading: [f64; 2],
    goal: Option<V3>,
    goal_left: f64,
}

/// What one man of his makes of his pace.
struct Say {
    scale: f64,
    /// The frame is the other man's first.
    hold: bool,
    /// The walker is ahead of the other man for the frame.
    mine: bool,
    /// It is only worth a beat's wait.
    beat: bool,
}

impl Say {
    const NOTHING: Say = Say { scale: 1.0, hold: false, mine: false, beat: false };
    fn pace(scale: f64) -> Say { Say { scale, ..Say::NOTHING } }
    fn hold(scale: f64) -> Say { Say { scale, hold: true, ..Say::NOTHING } }
}

/// What a man ahead of him on his line is to him: nobody to mind, one to follow at a share of his pace, or one to walk round, there and that far off.
enum InFront {
    Nobody,
    Follow(f64),
    Round(V3, f64),
}

/// What they all make of it together.
struct Told {
    scale: f64,
    hold: bool,
    patience: f64,
    by: String,
    round: Option<(V3, f64)>,
}

/// PASS: the pace `gd` walks his path at this frame; `None`: this file has his feet.
pub fn doorway_gate(g: &mut Game, gd: GuardId, dt: f64, speed: f64) -> Option<f64> {
    if !ours(g, gd) || crate::condition::down(g, g.gd[gd].ch) || !(speed > 0.0) { return Some(speed); }
    remembers(g, gd);
    let pos = g.ch[g.gd[gd].ch].pos;
    if frames_by(g, pos).any(|d| in_opening(g, d, pos, None)) {   // in an opening: he clears it, whoever stands about
        release(g, gd, false);
        st!(g, gd).held_t = 0.0;
        return Some(speed);
    }
    let w = walker(g, gd, dt, speed);
    if !w.by_door && !ends_by_door(g, &w) {
        eases(g, &w);
        return Some(speed);
    }
    if g.time < st!(g, gd).pass {   // going anyway: held long enough
        if w.waiting { release(g, gd, true); }
        return Some(speed);
    }
    let told = hears_all(g, &w);
    if let Some((at, _)) = told.round {
        if told.scale > 0.5 && g.time >= st!(g, gd).round_t { goes_round(g, &w, at); }
    }
    let held = told.scale < 0.2 || (w.waiting && told.hold);   // a man at a jamb is held as long as the frame is not his
    if !held {
        eases(g, &w);
    } else if tires(g, &w, &told) {
        return Some(speed);
    }
    if w.waiting {
        if !told.hold {
            release(g, gd, true);
            return Some(speed * told.scale);
        }
        waits(g, &w);
        return None;
    }
    if held && told.hold && steps_aside(g, &w) { return None; }
    if told.scale < 1.0 { return Some(brakes(g, &w, speed * told.scale)); }
    Some(speed)
}

/// His row, made if new: the time he was held and the door he waits at are his only while he keeps asking, and for the walk he was on.
fn remembers(g: &mut Game, gd: GuardId) {
    let (t, walk) = (g.time, g.gd[gd].path_serial);
    let s = state_of(g, gd);
    if t - s.t > 0.5 || t < s.t { s.held_t = 0.0; s.spot = None; s.door = None; }
    s.t = t;
    if s.spot.is_some() && s.path != Some(walk) { s.spot = None; s.door = None; }   // given a new walk while he waited at a jamb: the old door is no longer his business (the time held is kept)
}

fn walker(g: &Game, gd: GuardId, dt: f64, speed: f64) -> Walker {
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let s = g.m.doorway.states.get(gd).unwrap();
    let to = match (s.spot, s.door) {
        (Some(_), Some(d)) => Some(DoorAhead { d, side: s.side, dist: flat(pos, g.dr[d].frame_centre) }),
        _ => door_ahead(g, gd),
    };
    let goal = g.gd[gd].path_goal;
    Walker {
        gd,
        c,
        id: g.ch[c].id,
        pos,
        dt,
        speed,
        turn: !g.gd[gd].script.as_ref().and_then(|s| s.over.as_ref()).is_some_and(|o| o.walk == Some(true) && o.face.is_some()),
        to,
        waiting: s.spot.is_some(),
        by_door: to.is_some() || near_frame(g, pos),
        heading: heading(g, gd, pos),
        goal,
        goal_left: goal.map_or(m::INF, |pg| flat(pos, pg)),
    }
}

/// His heading, for the man ahead of him by a door, whether or not his own walk goes through it.
fn heading(g: &Game, gd: GuardId, pos: V3) -> [f64; 2] {
    let Some(wp) = g.gd[gd].path.get(g.gd[gd].path_i) else { return [0.0, 0.0] };
    let (dx, dz) = (wp[0] - pos[0], wp[2] - pos[2]);
    let l = m::hyp2(dx, dz);
    if l > 1e-3 { [dx / l, dz / l] } else { [0.0, 0.0] }
}

fn near_frame(g: &Game, p: V3) -> bool {
    g.doors.list.iter().any(|d| flat(p, g.dr[*d].frame_centre) <= 2.5)
}

/// The last metres of a walk to an order's own mark by a door: there only a mate standing on HIS mark is anything to him.
fn ends_by_door(g: &Game, w: &Walker) -> bool {
    let Some(goal) = w.goal else { return false };
    w.goal_left <= g.k.doormanners.own_end_m && order_post(g, &crate::seats::here(g), w.gd).is_some() && near_frame(g, goal)
}

/// He was not held this frame: what he was runs off, twice as fast.
fn eases(g: &mut Game, w: &Walker) {
    let s = st!(g, w.gd);
    s.held_t = m::max(0.0, s.held_t - 2.0 * w.dt);
}

/// Each man of his in turn: the slowest pace any of them asks, and the nearest to walk round.
fn hears_all(g: &Game, w: &Walker) -> Told {
    let mut told = Told { scale: 1.0, hold: false, patience: g.k.doormanners.patience, by: String::new(), round: None };
    for o in friends(g, w.gd) {
        let (far, pace) = (flat(w.pos, o.pos), m::hyp2(o.vel[0], o.vel[2]));
        let mut say = match w.to { Some(x) => at_frame(g, w, x, &o, far, pace), None => Say::NOTHING };
        if say.scale == 1.0 && !say.hold && !say.mine && !w.waiting {
            match in_front(g, w, &o, far, pace) {
                InFront::Follow(scale) => say.scale = scale,
                InFront::Round(at, far) if told.round.map_or(true, |r| far < r.1) => told.round = Some((at, far)),
                _ => {}
            }
        }
        if say.beat { told.patience = m::min(told.patience, g.k.doormanners.beyond); }
        if say.hold { told.hold = true; }
        if say.scale < told.scale {
            told.scale = say.scale;
            told.by = o.gd.map_or_else(|| String::from("Player"), |od| g.gd[od].callsign.clone());
        }
    }
    told
}

/// The frame he is for, between him and another.
fn at_frame(g: &Game, w: &Walker, x: DoorAhead, o: &FriendsItem, far: f64, pace: f64) -> Say {
    let r = rel(g, x.d, o.pos);
    let beyond = r.perp * x.side < 0.0;
    let away = pace > 0.4 && -(o.vel[0] * r.f.nx + o.vel[2] * r.f.nz) * x.side > 0.2 * pace;   // walking the way he is going
    let short = v3::clamp((x.dist - g.k.doormanners.throat) / 0.5, 0.0, 1.0);
    if in_opening(g, x.d, o.pos, None) {
        return if away { Say::pace(gap_k(g, far)) } else { Say::hold(short) };
    }
    let theirs = o.gd.filter(|od| pace > 0.2 || asking(g, *od).is_some()).and_then(|od| through(g, od, x.d));
    let first = |th: &Through| th.dist < x.dist - 0.01 || (m::abs(th.dist - x.dist) <= 0.01 && o.id < w.id);
    if let (true, Some(th)) = (beyond, theirs.filter(|th| th.side != x.side)) {   // coming the other way and nearer the frame: his first
        let at_jamb = o.gd.and_then(|od| g.m.doorway.states.get(od)).is_some_and(|so| so.spot.is_some());
        return if (w.waiting && !(at_jamb && w.id < o.id)) || first(&th) { Say::hold(short) } else { Say::NOTHING };
    }
    if beyond && r.along >= -0.3 && r.along <= r.f.w + 0.3 && m::abs(r.perp) <= g.k.doormanners.half + 0.9 {   // standing just beyond it: a beat short of the frame for him to move
        if away { return Say::pace(gap_k(g, far)); }
        return if pace < 0.4 { Say { beat: true, ..Say::pace(short) } } else { Say::NOTHING };
    }
    let Some(th) = theirs.filter(|th| !beyond && !w.waiting && th.side == x.side) else { return Say::NOTHING };
    match o.gd.and_then(|od| crate::doortools::named_before(g, x.d, w.gd, od)) {   // for the same frame, the same way: the man an entry names goes first, else the nearer
        Some(false) => Say::pace(short),
        None if first(&th) => Say::pace(gap_k(g, far)),
        _ => Say { mine: true, ..Say::NOTHING },
    }
}

/// The row of a man who asked this file for his pace a moment ago.
fn asking(g: &Game, o: GuardId) -> Option<&St> {
    g.m.doorway.states.get(o).filter(|so| g.time - so.t <= 0.25 && g.time >= so.t)
}

/// A man ahead of him on his line by a door: followed at the gap, never walked into; standing with his way on well past him, walked round.
fn in_front(g: &Game, w: &Walker, o: &FriendsItem, far: f64, pace: f64) -> InFront {
    let [hx, hz] = w.heading;
    let k = &g.k.doormanners;
    if !(far < k.gap && (js::truthy(hx) || js::truthy(hz))) { return InFront::Nobody; }
    let (ux, uz) = ((o.pos[0] - w.pos[0]) / js::or(far, 1.0), (o.pos[2] - w.pos[2]) / js::or(far, 1.0));
    if !(ux * hx + uz * hz > 0.7) { return InFront::Nobody; }
    let own = pace < 0.6 && on_own(g, &crate::seats::here(g), o.gd);   // on his own mark: standing, whatever his last steps onto it say
    if !w.by_door && !own { return InFront::Nobody; }
    if !own && pace > 0.4 && o.vel[0] * hx + o.vel[2] * hz > 0.3 * pace { return InFront::Follow(gap_k(g, far)); }
    if !(pace < 0.2 || own) { return InFront::Nobody; }
    if w.goal.is_some_and(|pg| flat(o.pos, pg) < 0.4) { return InFront::Follow(gap_k(g, far)); }   // ON his own place: he stops short
    if w.goal_left > far + 0.8 && far < k.round { InFront::Round(o.pos, far) } else { InFront::Nobody }
}

/// A corner put in his walk beside the man at `at`, on the side he is already off to.
fn goes_round(g: &mut Game, w: &Walker, at: V3) {
    st!(g, w.gd).round_t = g.time + 1.0;
    let (px, pz) = (-w.heading[1], w.heading[0]);
    let out = g.k.doormanners.round_out;
    let side = if (w.pos[0] - at[0]) * px + (w.pos[2] - at[2]) * pz >= 0.0 { 1.0 } else { -1.0 };
    for s in [side, -side] {
        let by: V3 = [at[0] + px * s * out, 0.0, at[2] + pz * s * out];
        if nav_blocked(g, by[0], by[2]) || walled(g, w.pos, by) || !spot_free(g, &crate::seats::here(g), w.gd, by, 0.6) { continue; }
        let i = g.gd[w.gd].path_i;
        g.gd[w.gd].path.insert(i, by);
        return;
    }
}

/// He is held another frame; true once he has been long enough and goes anyway.
fn tires(g: &mut Game, w: &Walker, told: &Told) -> bool {
    let pass = g.time + g.k.doormanners.rest;
    let s = st!(g, w.gd);
    s.by = Some(told.by.clone());
    s.held_t += w.dt;
    if s.held_t < told.patience { return false; }
    s.held_t = 0.0;
    s.pass = pass;
    if w.waiting { release(g, w.gd, true); }
    true
}

/// At his jamb, or on to it, facing the frame.
fn waits(g: &mut Game, w: &Walker) {
    let s = st!(g, w.gd);
    let (spot, door) = (s.spot.unwrap(), s.door.unwrap());
    if aside(g, w, spot) && w.turn {
        let (fc, at) = (g.dr[door].frame_centre, g.ch[w.c].pos);
        g.ch[w.c].body_yaw = v3::approach_angle(g.ch[w.c].body_yaw, m::atan2(fc[0] - at[0], fc[2] - at[2]), 5.5 * w.dt);
    }
}

/// Held short of the frame long enough: off to a jamb, out of the other's way. False where there is no place for it.
fn steps_aside(g: &mut Game, w: &Walker) -> bool {
    let Some(x) = w.to else { return false };
    let (after, t) = (g.k.doormanners.aside, g.time);
    let s = st!(g, w.gd);
    if s.held_t < after || t < s.no_spot { return false; }
    let Some(p) = jamb_spot(g, w.gd, x.d, x.side) else { st!(g, w.gd).no_spot = t + 1.0; return false };
    let walk = g.gd[w.gd].path_serial;
    let s = st!(g, w.gd);
    s.spot = Some(p);
    s.door = Some(x.d);
    s.side = x.side;
    s.path = Some(walk);
    aside(g, w, p);
    true
}

/// A step to a place off the walk; true = there.
fn aside(g: &mut Game, w: &Walker, p: V3) -> bool {
    let pace = m::min(w.speed, g.k.doormanners.step);
    step_to(g, w.gd, p, pace, w.dt, w.turn)
}

/// Down to the pace he is let, no faster than a man can slow.
fn brakes(g: &mut Game, w: &Walker, pace: f64) -> f64 {
    let now = g.gd[w.gd].speed;
    g.gd[w.gd].speed = m::min(now, m::max(pace, now - g.k.doormanners.brake * w.dt));
    pace
}

/// His own side about him, living, other than himself: the men, and Sam with the squad.
fn friends(g: &Game, gd: GuardId) -> Vec<FriendsItem> {
    let team = g.team(gd);
    let at = |c: CharId, gd| FriendsItem { pos: g.ch[c].pos, vel: g.ch[c].vel, gd, id: g.ch[c].id };
    let mates = g.guards.iter().copied().filter(|o| *o != gd && g.team(*o) == team && !crate::condition::down(g, g.gd[*o].ch));
    let mut out: Vec<FriendsItem> = mates.map(|o| at(g.gd[o].ch, Some(o))).collect();
    let sam = g.pl[g.player].ch;
    if team.ours() && !crate::condition::down(g, sam) { out.push(at(sam, None)); }
    out
}

/// Does `o`'s walk go through `d` within reach: metres of walk to the line and the face he comes from.
fn through(g: &Game, o: GuardId, d: DoorId) -> Option<Through> {
    let reach = g.k.doormanners.reach;
    let (mut a, mut run) = (g.ch[g.gd[o].ch].pos, 0.0);
    let mut i = g.gd[o].path_i;
    while i < g.gd[o].path.len() && run < reach {
        let b = g.gd[o].path[i];
        if let Some(x) = crate::guards::crosses_door(g, a, b, d) {
            let dist = run + flat(a, x.at);
            return if dist <= reach { Some(Through { dist, side: x.side }) } else { None };
        }
        run += flat(a, b);
        a = b;
        i += 1;
    }
    None
}
