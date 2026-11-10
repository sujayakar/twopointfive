use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// a bone of his if it has been baked, else his feet raised by `up`
fn bone_or(g: &Game, c: CharId, b: crate::character::Bone, up: f64) -> V3 {
    match g.ch[c].bones.get(b) { Some(p) => *p, None => v3::add(g.ch[c].pos, [0.0, up, 0.0]) }
}

/// the one thing here the simulation reads: what each man wears, dealt in the order he is drawn. The renderer's half (`draw`) is the picture's, not translated
pub(super) fn game_draw_men(g: &mut crate::game::Game, all: &[CharId]) {
    let mut drawn = crate::fogofwar::fog_of_war_filter(g, all);
    if drawn.len() > 8 && g.staged.facts.fight {
        let mut down: Vec<CharId> = Vec::new();
        let mut i = 0;
        while i < g.guards.len() { let gd = g.guards[i]; i += 1; if crate::condition::down(g, g.gd[gd].ch) { down.push(g.gd[gd].ch); } }
        if !down.is_empty() {
            let mut re: Vec<CharId> = drawn.iter().copied().filter(|c| !down.contains(c)).collect();
            re.extend(drawn.iter().copied().filter(|c| down.contains(c)));
            drawn = re;
        }
    }
    if g.art.rig[g.rig].cast.is_some() {
        for &c in drawn.iter() { let parts = crate::cast::cast_parts(g, c); g.ch[c].parts = parts; }
    }
}

/// what each guard's line to the man he faces has of smoke and weather on it, and how lit each seat's man is
pub(super) fn game_light_meter(g: &mut crate::game::Game, dt: f64) {
    use crate::character::Bone;
    let seats = g.seats.clone(); let one = seats.len() == 1;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i]; i += 1;
        if crate::condition::down(g, g.gd[gd].ch) { continue; }
        let pl = if one { g.player } else { let view = crate::seats::view_for(g, gd); crate::cmd::seat_man(g, view) };
        let pc = g.pl[pl].ch;
        let chest = bone_or(g, pc, Bone::Chest, 1.1);
        let eye = bone_or(g, g.gd[gd].ch, Bone::Head, 1.65);
        g.gd[gd].smoke_trans = crate::sense::clear_between(g, eye, chest);
    }
    for &s in seats.iter() {
        let man = crate::cmd::seat_man(g, s);
        let c = g.pl[man].ch;
        crate::guards::light_meter(g, c, dt);
    }
}

/// put the highlight on the row of `kind` (the one whose verb is `verb`, if given) in this frame's list; false if none is on offer
pub fn game_select_verb(g: &mut crate::game::Game, kind: crate::game::InteractableKind, verb: Option<crate::game::Verb>) -> bool {
    let at = g.sets[g.seat].nearby.iter().position(|r| r.kind == kind && (verb.is_none() || r.verb == verb));
    let Some(i) = at else { return false; };
    g.sets[g.seat].hand.near_sel = i as i32;
    crate::player::pick_hover(g);
    true
}

/// The pause or the end card has come up: the arc, the squad's ring, the hand's ring and the gun's setup of the player whose screen it is are put away, and nothing is sent.
pub fn game_let_go(g: &mut crate::game::Game) {
    let host = game_host_seat(g);
    crate::seats::with_seat(g, host, &mut |g| {
        crate::player::launch_let_go(g);
        crate::orders::squad_orders_let_go(g);
        crate::interact::interact_let_go(g);
        g.sets[g.seat].hand.setup = None;
    });
}

/// the light meter's mapping, irradiance → 0‥1 visibility
pub fn game_visibility_of(g: &crate::game::Game, e: f64) -> f64 {
    visibility_sum(g.tune.vis_slope, g.tune.vis_floor, e)
}
/// visibilityOf's sum
fn visibility_sum(slope: f64, floor: f64, e: f64) -> f64 {
    v3::clamp(slope * m::log(m::max(e, 1e-5) / floor), 0.0, 1.0)
}

/// what touches the leaves this step: each contact and the man it is of (his place is his own: read as the step opens, written back as it ends), each
/// seat's own among them (g.door_owns), and the bodies on the floor
#[derive(Default)]
struct Touching {
    contacts: Vec<crate::doors::DoorContact>,
    of: Vec<Option<CharId>>,
    /// a body's contact: which, the body, the particle
    parts: Vec<(usize, CharId, f64)>,
    bodies: Vec<CharId>,
}
impl Touching {
    fn add(&mut self, of: Option<CharId>, c: crate::doors::DoorContact) {
        self.of.push(of);
        self.contacts.push(c);
    }
}

/// the leaves' step and everything round it: who touches them, what they did to whom
pub(super) fn game_update_doors(g: &mut crate::game::Game, dt: f64) {
    let seats = g.seats.clone();
    let n = seats.len();
    let mut t = Touching::default();
    seats_touch(g, &seats, &mut t);
    guards_touch(g, &mut t);
    civilians_touch(g, &mut t);
    bodies_touch(g, &seats, &mut t);
    let angles: Vec<f64> = g.doors.list.iter().map(|&d| g.dr[d].leaf.angle).collect();
    for &s in seats.iter() { let input = g.tab.cmd_seat[s].input; crate::seats::with_seat(g, s, &mut |g| game_door_acts(g, dt, input)); }
    if g.door_was.len() < 2 * n { g.door_was.resize(2 * n, 0.0); }
    for i in 0..n { let pl = crate::cmd::seat_man(g, seats[i]); let p = g.ch[g.pl[pl].ch].pos; g.door_was[2 * i] = p[0]; g.door_was[2 * i + 1] = p[2]; }
    // (…as the step opens: this frame's verbs have moved them)
    for (c, of) in t.contacts.iter_mut().zip(t.of.iter()) { if let Some(m) = *of { c.pos = g.ch[m].pos; if c.vel.is_some() { c.vel = Some(g.ch[m].vel); } } }
    crate::doors::doors_update(g, dt, &mut t.contacts);
    // (…and as it ends: where the leaves put them)
    for (c, of) in t.contacts.iter().zip(t.of.iter()) { if let Some(m) = *of { g.ch[m].pos[0] = c.pos[0]; g.ch[m].pos[2] = c.pos[2]; } }
    crate::doorthrow::door_throw_after(g, &t.contacts);
    crate::doorblast::door_slabs(g, dt);
    game_hands_on_doors(g, &angles);
    for i in 0..n {
        let s = seats[i];
        let pl = crate::cmd::seat_man(g, s); let at = g.ch[g.pl[pl].ch].pos;
        let own = g.door_owns[i];
        if let Some(k) = own { t.contacts[k].pos = at; }   // (his: wherever he now is)
        let (bx, bz) = (g.door_was[2 * i], g.door_was[2 * i + 1]);
        crate::seats::with_seat(g, s, &mut |g| game_door_after(g, own.map(|k| &mut t.contacts[k]), bx, bz));
    }
    bodies_pushed(g, &t);
    bodies_woken(g, &t.bodies, &angles);
}

/// each player on his feet, as he moves
fn seats_touch(g: &mut Game, seats: &[SeatId], t: &mut Touching) {
    g.door_owns.clear();
    for &s in seats {
        let own = crate::seats::with_seat(g, s, &mut |g| game_door_contact(g));
        g.door_owns.push(own.as_ref().map(|_| t.contacts.len()));
        if let Some(c) = own {
            let pl = crate::cmd::seat_man(g, s);
            t.add(Some(g.pl[pl].ch), c);
        }
    }
}

/// each guard on his feet, by what he is doing
fn guards_touch(g: &mut Game, t: &mut Touching) {
    use crate::game::GuardState;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i]; i += 1;
        if crate::condition::down(g, g.gd[gd].ch) { continue; }
        let c = g.gd[gd].ch;
        let speed = g.gd[gd].speed;
        let bash = g.gd[gd].state == GuardState::Alert && crate::kitload::load_gait(g, g.gd[gd].ch, speed) > 2.2;
        let quiet = crate::silent::silent_ease(g, gd);
        let hand = crate::handleside::hand_press(g, gd);
        let round = speed > crate::doorway::ROUND_PACE && crate::doorway::rounds_leaves(g, gd) && !crate::doorway::leaf_stalled(g, gd);
        let contact = crate::doors::DoorContact { pos: g.ch[c].pos, radius: 0.34, bash, who: g.ch[c].id, quiet, weak: Some(crate::doorway::only_brushes(g, gd)), key: Some(true), hand, round: Some(round), ..crate::doors::DoorContact::default() };
        t.add(Some(c), contact);
    }
}

/// each civilian in sight
fn civilians_touch(g: &mut Game, t: &mut Touching) {
    let n0 = t.contacts.len();
    crate::civilians::civilian_door_contacts(g, &mut t.contacts);
    let shown = crate::civilians::civilians_shown(g);
    for k in n0..t.contacts.len() {
        let who = t.contacts[k].who;
        t.of.push(shown.iter().map(|gd| g.gd[*gd].ch).find(|c| g.ch[*c].id == who));
    }
}

/// each body down by a door: a few of its particles, as weak contacts
fn bodies_touch(g: &mut Game, seats: &[SeatId], t: &mut Touching) {
    let mut men: Vec<CharId> = seats.iter().map(|&s| g.pl[crate::cmd::seat_man(g, s)].ch).collect();
    men.extend(g.guards.iter().map(|&gd| g.gd[gd].ch));
    men.extend(crate::civilians::civilians_shown(g).iter().map(|&gd| g.gd[gd].ch));
    for ch in men {
        let Some(rag) = g.ch[ch].ragdoll else { continue; };
        let pelvis = crate::ragdoll::rag().pelvis;
        let mut tmp = g.tmp_v;
        let hp = crate::ragdoll::ragdoll_point(g, rag, pelvis, Some(&mut tmp));
        g.tmp_v = tmp;
        let near = g.doors.list.iter().any(|&d| { let p = crate::doors::door_pos(g, d); m::abs(p[0] - hp[0]) < 2.2 && m::abs(p[2] - hp[2]) < 2.2 });
        if !near { continue; }
        t.bodies.push(ch);
        for i in crate::game::body_door_particles() {
            let p = crate::ragdoll::ragdoll_point(g, rag, i, None);
            let radius = crate::ragdoll::ragdoll_radius(i);
            t.parts.push((t.contacts.len(), ch, i));
            t.add(None, crate::doors::DoorContact { pos: p, radius, bash: false, who: g.ch[ch].id, quiet: true, weak: Some(true), ..crate::doors::DoorContact::default() });
        }
    }
}

/// what the leaves pushed of a body's is pushed
fn bodies_pushed(g: &mut Game, t: &Touching) {
    for &(k, ch, i) in t.parts.iter() {
        let c = t.contacts[k].pos;
        let rag = g.ch[ch].ragdoll.unwrap();
        let at = i as usize;
        let (dx, dz) = (c[0] - g.tab.ragdoll[rag].x[at * 3], c[2] - g.tab.ragdoll[rag].x[at * 3 + 2]);
        crate::ragdoll::ragdoll_nudge(g, rag, i, dx, dz);
    }
}

/// a body asleep by a leaf that swung, or by a slab that fell, is woken
fn bodies_woken(g: &mut Game, bodies: &[CharId], angles: &[f64]) {
    if bodies.is_empty() { return; }
    let list = g.doors.list.clone();
    for (k, &d) in list.iter().enumerate() {
        let slab = g.dr[d].slab.filter(|s| !s.rest);
        if slab.is_none() && m::abs(g.dr[d].leaf.angle - angles[k]) < 1e-3 { continue; }
        let (hinge, width, height) = (g.dr[d].def.hinge, g.dr[d].def.width, g.dr[d].height);
        for &ch in bodies.iter() {
            let h = g.ch[ch].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[ch].pos);
            let rag = g.ch[ch].ragdoll.unwrap();
            let sleeping = g.tab.ragdoll[rag].sleeping;
            if sleeping && (m::hyp2(h[0] - hinge[0], h[2] - hinge[1]) < width + 1.2
                || slab.as_ref().map_or(false, |s| m::hyp2(h[0] - s.p[0], h[2] - s.p[2]) < height / 2.0 + 0.8)) {
                crate::ragdoll::ragdoll_wake(g, rag);
            }
        }
    }
}

/// a seat's half of updateDoors, run over its view: the player's own contact on the leaves this frame (none: he is down)
fn game_door_contact(g: &crate::game::Game) -> Option<crate::doors::DoorContact> {
    let pl = g.player; let pc = g.pl[pl].ch;
    if crate::condition::down(g, g.pl[pl].ch) { return None; }
    let run = crate::sprintdoor::sprint_door_run(g, pl);
    // (DOORWAY.solid, DOORWAY.oneWay: both true on the page)
    Some(crate::doors::DoorContact {
        pos: g.ch[pc].pos, radius: 0.34, bash: g.pl[pl].sprinting, who: g.ch[pc].id, quiet: g.pl[pl].crouch && !g.pl[pl].dash,
        keys: Some(g.pl[pl].keys.clone()), run, solid: Some(true), vel: Some(g.ch[pc].vel), ..crate::doors::DoorContact::default()
    })
}


/// the row a click with empty hands is on: the first guard in reach who is no hostage
fn click_row(g: &Game, seat: &crate::seats::SeatRef) -> Option<Interactable> {
    let mut i = 0;
    while i < seat.nearby.len() {
        if seat.nearby[i].kind == InteractableKind::Guard && seat.nearby[i].in_reach {
            let taken = match seat.nearby[i].guard {
                Some(gd) => g.staged.captives.contains(&gd),
                None => false,
            };
            if !taken { return Some(seat.nearby[i].clone()); }
        }
        i += 1;
    }
    None
}

/// the way in is read where he stands: Hold.from, off his body now
fn hold_from_here(g: &mut Game, pl: PlayerId) {
    let c = g.pl[pl].ch;
    let p = g.ch[c].pos;
    if let Some(h) = g.pl[pl].holding_mut() { h.from = Some([p[0], p[1], p[2]]); }
}

/// whether the man in his arm is this one: pl.holding?.g === gd
fn holds(g: &Game, pl: PlayerId, gd: GuardId) -> bool {
    match g.pl[pl].holding() { Some(h) => h.g == gd, None => false }
}

/// this frame's verb of his, done
fn game_door_acts(g: &mut crate::game::Game, dt: f64, input: InputId) {
    let pl = g.player;
    let act: Option<Interactable> = if crate::condition::down(g, g.pl[pl].ch) { None } else { g.sets[g.seat].act.clone() };
    if let Some(a) = &act { act_thing(g, a); }
    crate::handwork::breaker_tick(g, dt);
    if let Some(a) = &act { act_device(g, a); }
    act_grab(g, act.as_ref(), dt, input);
    if let Some(a) = &act { act_door(g, a); }
    eases_leaf(g, dt);
    crate::hands::door_move_tick(g, dt);
}

/// a body taken up or put down, a breaker thrown
fn act_thing(g: &mut Game, a: &Interactable) {
    let verb = a.verb;
    if a.kind == InteractableKind::Body && a.guard.is_some() {
        if verb == Some(Verb::Drop) {
            if g.pl[g.player].dragging().is_some() && !crate::hands::haul_set_down(g) { crate::player::toggle_drag(g, None); }
        } else if verb == Some(Verb::Drag) {
            crate::player::toggle_drag(g, a.guard);
        }
    }
    if a.kind == InteractableKind::Breaker && !crate::handwork::breaker_start(g, verb == Some(Verb::BreakerOn)) {
        if verb == Some(Verb::BreakerOn) && g.light_home.mains.active && !g.light_home.mains.permanent {
            crate::game::game_end_blackout(g);
        } else if verb == Some(Verb::BreakerOff) && !g.light_home.mains.active {
            crate::game::game_set_blackout(g, 86400.0, crate::lighting::SetBlackoutCause::Hand);
        }
    }
}

/// a camera taken back, a charge of his own, a light switch
fn act_device(g: &mut Game, a: &Interactable) {
    let pl = g.player;
    match a.kind {
        InteractableKind::Cam if a.verb == Some(Verb::TakeCam) => { crate::stickycam::take_cam(g, pl); }
        InteractableKind::Charge => { crate::ownblow::own_blow_act(g); }
        InteractableKind::Switch => { if let Some(v) = a.verb { crate::lightswitch::switch_act(g, pl, v); } }
        _ => {}
    }
}

/// the grab and the takedown, with one gate: by the stalk, by a click with empty hands (held or tapped), or by the row's own verb
fn act_grab(g: &mut Game, act: Option<&Interactable>, dt: f64, input: InputId) {
    let pl = g.player;
    let free = !g.pl[pl].planted() && !crate::condition::down(g, g.pl[pl].ch) && !crate::samstun::sam_staggered(g, &crate::seats::here(g), None);
    let in_reach: Option<Interactable> = if g.pl[pl].holstered { click_row(g, &crate::seats::here(g)) } else { None };
    let choke_on = match in_reach.as_ref().and_then(|r| r.guard) {
        Some(gd) if g.gd[gd].civ.is_none() && crate::rails::rails_offers(g, "grab") => {
            let p = g.ch[g.pl[pl].ch].pos;
            crate::close::close_at(g, gd, &p).grab
        }
        _ => false,
    };
    let rmb_on = g.pl[pl].holstered && free;
    let rmb_down = crate::input::input_rmb(g, input);
    let rmb_hit = crate::input::input_rmb_hit(g, input);
    let dropped = crate::input::input_hit(g, input, "Blur") || crate::input::input_hit(g, input, "Paused");
    let rb = crate::close::close_rmb(g, rmb_on, rmb_down, rmb_hit, dt, in_reach.is_some(), choke_on, dropped);
    let clicked: Option<Interactable> = if rb.is_some() { in_reach } else { None };
    let via_click: Option<GuardId> = clicked.as_ref().and_then(|c| c.guard);
    let clicked_close = clicked.as_ref().and_then(|c| c.close);
    let stalked = crate::stalk::stalk_grab(g, &crate::seats::here(g));
    if !free { return; }
    if let Some(st) = stalked {
        crate::player::start_grab(g, st, None);
        if holds(g, pl, st) { hold_from_here(g, pl); }
    } else if let Some(vc) = via_click {
        if rb == Some(crate::close::CloseRmb::Hold) {
            let keyed = crate::stalk::stalk_key_can(g, vc);
            crate::player::start_grab(g, vc, clicked_close);
            if holds(g, pl, vc) {
                if keyed {
                    crate::stalk::stalk_key_took(g);
                    hold_from_here(g, pl);
                }
                let tap = g.k.close.rmb_tap_secs;
                if let Some(h) = g.pl[pl].holding_mut() {
                    h.rmb_choke = Some(1.0);
                    h.rmb_t = Some(tap);
                }
            }
        } else {
            let loud = rb == Some(crate::close::CloseRmb::Tap) && choke_on;
            crate::player::start_takedown(g, vc, clicked_close, Some(false), Some(loud));
        }
    } else if let Some((ag, close)) = act.filter(|a| a.kind == InteractableKind::Guard && a.verb == Some(Verb::Grab)).and_then(|a| Some((a.guard?, a.close))) {
        let keyed = crate::stalk::stalk_key_can(g, ag);
        crate::player::start_grab(g, ag, close);
        if keyed && holds(g, pl, ag) {
            crate::stalk::stalk_key_took(g);
            hold_from_here(g, pl);
        }
    }
}

/// a door in reach, one row per thing it can be made to do
fn act_door(g: &mut Game, a: &Interactable) {
    let (InteractableKind::Door, Some(d)) = (a.kind, a.door) else { return; };
    let pl = g.player;
    let pc = g.pl[pl].ch;
    let pos = g.ch[pc].pos;
    let id = g.ch[pc].id;
    let keys = g.pl[pl].keys.clone();
    match a.verb {
        Some(Verb::Push) | Some(Verb::Pull) | Some(Verb::Unlock) => {
            if !crate::hands::door_move_begin(g, d) {
                g.pl[pl].door_move_over();
                let pos = g.ch[pc].pos;
                let keys = g.pl[pl].keys.clone();
                crate::doors::doors_use(g, d, pos, id, Some(&keys));
            }
        }
        Some(Verb::Ease) => {
            if !crate::doors::doors_locked_out(g, d, pos, Some(&keys)) {
                let a = g.dr[d].leaf.angle;
                crate::deed::begin(g, crate::game::Deed::Ease(crate::game::Easing { door: d, best: m::abs(a), stall_t: 0.0 }));
            }
        }
        Some(Verb::Pick) => {
            if crate::doors::doors_locked_out(g, d, pos, Some(&keys)) { crate::player::start_picking(g, d); }
        }
        Some(Verb::Kick) => {
            if !crate::doors::doors_boot_holds(g, d, pos) { crate::player::start_kick(g, d, None, None); }
        }
        Some(Verb::Rattle) => {
            crate::doors::doors_rattle(g, d, id, None);
        }
        Some(Verb::Code) => { crate::devices::start_coding(g, d); }
        Some(Verb::Nocode) => crate::devices::deny_pad(g, d),
        Some(Verb::Breach) | Some(Verb::Hammer) => crate::bigkit::sam_big_start(g, d),
        Some(Verb::Wand) => crate::player::start_wand(g, d),
        _ => {}
    }
}

/// the leaf he is easing: on round at the crack rate while he stays within reach of it
fn eases_leaf(g: &mut Game, dt: f64) {
    let pl = g.player;
    let pc = g.pl[pl].ch;
    let Some(&ease) = g.pl[pl].easing() else { return; };
    let d = ease.door;
    let pos = g.ch[pc].pos;
    let dp = crate::doors::door_pos(g, d);
    let in_reach = m::min(v3::dist_xz(dp, pos), v3::dist_xz(g.dr[d].frame_centre, pos)) < crate::doors::DOORS_USE_REACH + 0.25
        && !crate::condition::down(g, g.pl[pl].ch);
    let keys = g.pl[pl].keys.clone();
    if !in_reach || crate::doors::doors_locked_out(g, d, pos, Some(&keys)) {
        g.pl[pl].easing_over();
        return;
    }
    let id = g.ch[pc].id;
    crate::doors::doors_crack(g, d, pos, id, m::min(dt, 0.05), Some(&keys), Some(crate::player::EASE_RATE), Some(crate::player::EASE_MAX));
    let angle = g.dr[d].leaf.angle;
    let a1 = m::abs(angle);
    let stop = if angle < 0.0 { -crate::doors::door_min_a(g, d) } else { crate::doors::door_max_a(g, d) };
    if a1 >= m::min(crate::player::EASE_MAX, stop) - 1e-3 {
        g.pl[pl].easing_over();
    } else if let Some(e) = g.pl[pl].easing_mut() {
        if a1 <= e.best + 1e-5 {
            e.stall_t += dt;
            if e.stall_t > 0.5 { g.pl[pl].easing_over(); }
        } else {
            e.stall_t = 0.0;
            e.best = a1;
        }
    }
}

/// what the leaves did to him once they have swung
fn game_door_after(g: &mut crate::game::Game, own: Option<&mut crate::doors::DoorContact>, bx: f64, bz: f64) {
    let own: Option<&crate::doors::DoorContact> = own.as_deref();
    let pl = g.player;
    let pc = g.pl[pl].ch;
    if own.is_some() {
        crate::player::settle_player(g);
        crate::player::settle_held(g);
        let p = g.ch[pc].pos;
        if m::abs(p[0] - bx) + m::abs(p[2] - bz) > 0.005 { crate::character::character_update(g, pc, 0.0); }
    }
    if g.pl[pl].door_head != 0.0 && g.pl[pl].door_head_t != g.time {
        if g.ch[pc].anim.look_yaw_extra == g.pl[pl].door_head { g.ch[pc].anim.look_yaw_extra = 0.0; }
        g.pl[pl].door_head = 0.0;
    }
    if let Some(o) = own {
        if o.run.is_some() { crate::sprintdoor::sprint_door_after(g, pl, o); }
    }
    if own.is_some() {
        if let Some(kd) = crate::sprintdoor::sprint_door_kick(g, pl) { crate::player::start_kick(g, kd, Some(true), None); }
    }
    if let Some(d) = own.and_then(|o| o.blocked_by) {
        let p = &g.pl[pl];
        if p.sprinting && !p.planted() {
            let u = crate::doors::door_dir(g, d);
            let (pos, vel) = (g.ch[pc].pos, g.ch[pc].vel);
            let into = -(-u[1] * vel[0] + u[0] * vel[2]) * crate::doors::doors_side(g, d, pos);
            if into > 1.5 && !crate::doors::doors_boot_holds(g, d, pos) { crate::player::start_kick(g, d, None, None); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What the guards and his meter go by: nothing in the dark, all of it in good light, and between them by the log of the light.
    /// A FAILURE MEANS: the dark does not hide him, or a lamp does not show him.
    #[test]
    fn the_light_meter_reads_nothing_in_the_dark_and_rises_by_the_log_of_the_light_to_one() {
        let (slope, floor) = (0.37, 0.013);
        let meter = |e: f64| visibility_sum(slope, floor, e);
        for e in [0.0, -1.0, floor * 0.5, floor] { assert_eq!(meter(e), 0.0, "{e} of light"); }
        assert_eq!(meter(1e6), 1.0);
        let (a, b, c) = (meter(floor * 2.0), meter(floor * 4.0), meter(floor * 8.0));
        assert!(a > 0.0 && a < 1.0);
        assert!(m::abs((b - a) - (c - b)) < 1e-12 && b > a, "each doubling of the light adds the same: {a} {b} {c}");
    }
}
