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
    if g.tab.rig[g.rig].cast.is_some() {
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

/// the leaves' step and everything round it: who touches them, what they did to whom
pub(super) fn game_update_doors(g: &mut crate::game::Game, dt: f64) {
    use crate::character::Bone;
    use crate::doors::DoorContact;
    use crate::game::GuardState;
    let seats = g.seats.clone(); let n = seats.len();
    g.door_owns.clear();
    let mut contacts: Vec<DoorContact> = Vec::new();
    // (ALIASING: in the TypeScript a contact's `pos`, `vel` ARE his arrays. Each contact's man here: read as the step opens, written back as it ends. g.door_owns: the player's own contact's index in `contacts`)
    let mut of: Vec<Option<CharId>> = Vec::new();
    for i in 0..n {
        let s = seats[i];
        let own = crate::seats::with_seat(g, s, &mut |g| game_door_contact(g));
        if let Some(c) = own {
            let pl = crate::cmd::seat_man(g, s);
            g.door_owns.push(Some(contacts.len())); of.push(Some(g.pl[pl].ch)); contacts.push(c);
        } else { g.door_owns.push(None); }
    }
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
        of.push(Some(c));
        contacts.push(DoorContact { pos: g.ch[c].pos, radius: 0.34, bash, who: g.ch[c].id, quiet, weak: Some(crate::doorway::only_brushes(g, gd)), key: Some(true), hand, round: Some(round), ..DoorContact::default() });
    }
    let n0 = contacts.len();
    crate::civilians::civilian_door_contacts(g, &mut contacts);
    let shown_civs = crate::civilians::civilians_shown(g);
    for k in n0..contacts.len() {   // (their `pos` is their array too: by whose it is)
        let who = contacts[k].who;
        of.push(shown_civs.iter().map(|gd| g.gd[*gd].ch).find(|c| g.ch[*c].id == who));
    }
    // bodies: a few of each ragdoll's particles as weak contacts. (the contact, the body, the particle)
    let mut body_contacts: Vec<(usize, CharId, f64)> = Vec::new(); let mut bodies: Vec<CharId> = Vec::new();
    let mut gi = -(n as isize);
    while gi < (g.guards.len() + shown_civs.len()) as isize {
        let at = gi; gi += 1;
        let ch = if at < 0 { let pl = crate::cmd::seat_man(g, seats[(at + n as isize) as usize]); g.pl[pl].ch }
            else if (at as usize) < g.guards.len() { g.gd[g.guards[at as usize]].ch }
            else { g.gd[shown_civs[at as usize - g.guards.len()]].ch };
        let Some(rag) = g.ch[ch].ragdoll else { continue; };
        let pelvis = crate::ragdoll::rag().pelvis;
        let mut tmp = g.tmp_v;
        let hp = crate::ragdoll::ragdoll_point(g, rag, pelvis, Some(&mut tmp));
        g.tmp_v = tmp;
        let list = g.doors.list.clone();
        let mut near = false;
        for &d in list.iter() { let p = crate::doors::door_pos(g, d); if m::abs(p[0] - hp[0]) < 2.2 && m::abs(p[2] - hp[2]) < 2.2 { near = true; break; } }
        if !near { continue; }
        bodies.push(ch);
        for i in crate::game::body_door_particles() {
            let p = crate::ragdoll::ragdoll_point(g, rag, i, None);
            let radius = crate::ragdoll::ragdoll_radius(i);
            body_contacts.push((contacts.len(), ch, i));
            of.push(None);
            contacts.push(DoorContact { pos: p, radius, bash: false, who: g.ch[ch].id, quiet: true, weak: Some(true), ..DoorContact::default() });
        }
    }
    let list = g.doors.list.clone();
    let mut angles: Vec<f64> = Vec::with_capacity(list.len());
    for &d in list.iter() { angles.push(g.dr[d].leaf.angle); }
    for &s in seats.iter() { let input = g.tab.cmd_seat[s].input; crate::seats::with_seat(g, s, &mut |g| game_door_acts(g, dt, input)); }
    if g.door_was.len() < 2 * n { g.door_was.resize(2 * n, 0.0); }
    for i in 0..n { let pl = crate::cmd::seat_man(g, seats[i]); let p = g.ch[g.pl[pl].ch].pos; g.door_was[2 * i] = p[0]; g.door_was[2 * i + 1] = p[2]; }
    // (…as the step opens: this frame's verbs have moved them)
    for k in 0..contacts.len() { if let Some(c) = of[k] { contacts[k].pos = g.ch[c].pos; if contacts[k].vel.is_some() { contacts[k].vel = Some(g.ch[c].vel); } } }
    crate::doors::doors_update(g, dt, &mut contacts);
    // (…and as it ends: where the leaves put them)
    for k in 0..contacts.len() { if let Some(c) = of[k] { let p = contacts[k].pos; g.ch[c].pos[0] = p[0]; g.ch[c].pos[2] = p[2]; } }
    crate::doorthrow::door_throw_after(g, &contacts);
    crate::doorblast::door_slabs(g, dt);
    game_hands_on_doors(g, &angles);
    for i in 0..n {
        let s = seats[i];
        let pl = crate::cmd::seat_man(g, s); let at = g.ch[g.pl[pl].ch].pos;
        let own = g.door_owns[i];
        if let Some(k) = own { contacts[k].pos = at; }   // (his array: wherever he now is)
        let (bx, bz) = (g.door_was[2 * i], g.door_was[2 * i + 1]);
        crate::seats::with_seat(g, s, &mut |g| game_door_after(g, match own { Some(k) => Some(&mut contacts[k]), None => None }, bx, bz));
    }
    for &(k, ch, i) in body_contacts.iter() {
        let c = contacts[k].pos;
        let rag = g.ch[ch].ragdoll.unwrap();
        let at = i as usize;
        let (dx, dz) = (c[0] - g.tab.ragdoll[rag].x[at * 3], c[2] - g.tab.ragdoll[rag].x[at * 3 + 2]);
        crate::ragdoll::ragdoll_nudge(g, rag, i, dx, dz);
    }
    if !bodies.is_empty() {
        let list = g.doors.list.clone();
        for (k, &d) in list.iter().enumerate() {
            let slab = g.dr[d].slab.filter(|s| !s.rest);
            if slab.is_none() && m::abs(g.dr[d].leaf.angle - angles[k]) < 1e-3 { continue; }
            let (hinge, width, height) = (g.dr[d].def.hinge, g.dr[d].def.width, g.dr[d].height);
            for &ch in bodies.iter() {
                let h = g.ch[ch].bones.get(Bone::Hips).copied().unwrap_or(g.ch[ch].pos);
                let rag = g.ch[ch].ragdoll.unwrap();
                let sleeping = g.tab.ragdoll[rag].sleeping;
                if sleeping && (m::hyp2(h[0] - hinge[0], h[2] - hinge[1]) < width + 1.2
                    || slab.as_ref().map_or(false, |s| m::hyp2(h[0] - s.p[0], h[2] - s.p[2]) < height / 2.0 + 0.8)) {
                    crate::ragdoll::ragdoll_wake(g, rag);
                }
            }
        }
    }
}

/// a seat's half of updateDoors, run over its view: the player's own contact on the leaves this frame (none: he is down)
fn game_door_contact(g: &mut crate::game::Game) -> Option<crate::doors::DoorContact> {
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
    if let Some(h) = g.pl[pl].holding.as_mut() { h.from = Some([p[0], p[1], p[2]]); }
}

/// whether the man in his arm is this one: pl.holding?.g === gd
fn holds(g: &Game, pl: PlayerId, gd: GuardId) -> bool {
    match g.pl[pl].holding.as_ref() { Some(h) => h.g == gd, None => false }
}

/// this frame's verb of his, done
fn game_door_acts(g: &mut crate::game::Game, dt: f64, input: InputId) {
    let pl = g.player;
    let pc = g.pl[pl].ch;
    let act: Option<Interactable> = if crate::condition::down(g, g.pl[pl].ch) { None } else { g.sets[g.seat].act.clone() };
    let verb: Option<Verb> = act.as_ref().and_then(|a| a.verb);
    let kind: Option<InteractableKind> = act.as_ref().map(|a| a.kind);
    let act_guard: Option<GuardId> = act.as_ref().and_then(|a| a.guard);
    let act_door: Option<DoorId> = act.as_ref().and_then(|a| a.door);
    let act_close: Option<crate::close::CloseKind> = act.as_ref().and_then(|a| a.close);

    if kind == Some(InteractableKind::Body) && act_guard.is_some() {
        if verb == Some(Verb::Drop) {
            if g.pl[pl].dragging.is_some() && !crate::hands::haul_set_down(g) { crate::player::toggle_drag(g, None); }
        } else if verb == Some(Verb::Drag) {
            crate::player::toggle_drag(g, act_guard);
        }
    }
    if kind == Some(InteractableKind::Breaker) && !crate::handwork::breaker_start(g, verb == Some(Verb::BreakerOn)) {
        if verb == Some(Verb::BreakerOn) && g.light_home.mains.active && !g.light_home.mains.permanent {
            crate::game::game_end_blackout(g);
        } else if verb == Some(Verb::BreakerOff) && !g.light_home.mains.active {
            crate::game::game_set_blackout(g, 86400.0, crate::lighting::SetBlackoutCause::Hand);
        }
    }
    crate::handwork::breaker_tick(g, dt);
    if kind == Some(InteractableKind::Cam) && verb == Some(Verb::TakeCam) { crate::stickycam::take_cam(g, pl); }
    if kind == Some(InteractableKind::Charge) { crate::ownblow::own_blow_act(g); }
    if kind == Some(InteractableKind::Switch) {
        if let Some(v) = verb { crate::lightswitch::switch_act(g, pl, v); }
    }

    // the grab and the takedown, with one gate
    {
        let free = g.pl[pl].takedown.is_none() && !crate::condition::down(g, g.pl[pl].ch) && g.pl[pl].holding.is_none() && !crate::samstun::sam_staggered(g, &crate::seats::here(g), None) && g.pl[pl].breaker_move.is_none();
        let in_reach: Option<Interactable> = if g.pl[pl].holstered { click_row(g, &crate::seats::here(g)) } else { None };
        let reach_guard: Option<GuardId> = in_reach.as_ref().and_then(|r| r.guard);
        let choke_on = match reach_guard {
            Some(gd) if g.gd[gd].civ.is_none() && crate::rails::rails_offers(g, "grab") => {
                let p = g.ch[pc].pos;
                crate::close::close_at(g, gd, &p).grab
            }
            _ => false,
        };
        let rmb_on = g.pl[pl].holstered && free;
        let rmb_down = crate::input::input_rmb(g, input);
        let rmb_hit = crate::input::input_rmb_hit(g, input);
        let dropped = crate::input::input_hit(g, input, "Blur") || crate::input::input_hit(g, input, "Paused");
        let rb = crate::close::close_rmb(g, rmb_on, rmb_down, rmb_hit, dt, in_reach.is_some(), choke_on, dropped);
        let blow = rb.is_some();
        let clicked: Option<Interactable> = if blow { in_reach } else { None };
        let via_click: Option<GuardId> = clicked.as_ref().and_then(|c| c.guard);
        let clicked_close = clicked.as_ref().and_then(|c| c.close);
        let stalked = crate::stalk::stalk_grab(g, &crate::seats::here(g));
        if let (Some(st), true) = (stalked, free) {
            crate::player::start_grab(g, st, None);
            if holds(g, pl, st) { hold_from_here(g, pl); }
        } else if let (Some(vc), true, Some(crate::close::CloseRmb::Hold)) = (via_click, free, rb) {
            let keyed = crate::stalk::stalk_key_can(g, vc);
            crate::player::start_grab(g, vc, clicked_close);
            if holds(g, pl, vc) {
                if keyed {
                    crate::stalk::stalk_key_took(g);
                    hold_from_here(g, pl);
                }
                let tap = g.k.close.rmb_tap_secs;
                if let Some(h) = g.pl[pl].holding.as_mut() {
                    h.rmb_choke = Some(1.0);
                    h.rmb_t = Some(tap);
                }
            }
        } else if let (Some(vc), true) = (via_click, free) {
            let loud = rb == Some(crate::close::CloseRmb::Tap) && choke_on;
            crate::player::start_takedown(g, vc, clicked_close, Some(false), Some(loud));
        } else if let (Some(InteractableKind::Guard), Some(Verb::Grab), Some(ag), true) = (kind, verb, act_guard, free) {
            let keyed = crate::stalk::stalk_key_can(g, ag);
            crate::player::start_grab(g, ag, act_close);
            if keyed && holds(g, pl, ag) {
                crate::stalk::stalk_key_took(g);
                hold_from_here(g, pl);
            }
        }
    }

    // a door in reach, one row per thing it can be made to do
    if let (Some(InteractableKind::Door), Some(d)) = (kind, act_door) {
        let pos = g.ch[pc].pos;
        let id = g.ch[pc].id;
        let keys = g.pl[pl].keys.clone();
        match verb {
            Some(Verb::Push) | Some(Verb::Pull) | Some(Verb::Unlock) => {
                if !crate::hands::door_move_begin(g, d) {
                    g.pl[pl].door_move = None;
                    let pos = g.ch[pc].pos;
                    let keys = g.pl[pl].keys.clone();
                    crate::doors::doors_use(g, d, pos, id, Some(&keys));
                }
            }
            Some(Verb::Ease) => {
                if !crate::doors::doors_locked_out(g, d, pos, Some(&keys)) {
                    g.pl[pl].door_move = None;
                    g.pl[pl].easing = Some(d);
                    g.pl[pl].ease_stall_t = 0.0;
                    let a = g.dr[d].leaf.angle;
                    g.pl[pl].ease_best = m::abs(a);
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

    // the leaf he is easing: on round at the crack rate while he stays within reach of it
    if let Some(d) = g.pl[pl].easing {
        let pos = g.ch[pc].pos;
        let dp = crate::doors::door_pos(g, d);
        let in_reach = m::min(v3::dist_xz(dp, pos), v3::dist_xz(g.dr[d].frame_centre, pos)) < crate::doors::DOORS_USE_REACH + 0.25
            && !crate::condition::down(g, g.pl[pl].ch) && g.pl[pl].holding.is_none() && g.pl[pl].takedown.is_none() && g.pl[pl].kick.is_none();
        let keys = g.pl[pl].keys.clone();
        if !in_reach || crate::doors::doors_locked_out(g, d, pos, Some(&keys)) {
            g.pl[pl].easing = None;
        } else {
            let id = g.ch[pc].id;
            crate::doors::doors_crack(g, d, pos, id, m::min(dt, 0.05), Some(&keys), Some(crate::player::EASE_RATE), Some(crate::player::EASE_MAX));
            let angle = g.dr[d].leaf.angle;
            let a1 = m::abs(angle);
            let stop = if angle < 0.0 { -crate::doors::door_min_a(g, d) } else { crate::doors::door_max_a(g, d) };
            if a1 >= m::min(crate::player::EASE_MAX, stop) - 1e-3 {
                g.pl[pl].easing = None;
            } else if a1 <= g.pl[pl].ease_best + 1e-5 {
                g.pl[pl].ease_stall_t += dt;
                if g.pl[pl].ease_stall_t > 0.5 {
                    g.pl[pl].easing = None;
                }
            } else {
                g.pl[pl].ease_stall_t = 0.0;
                g.pl[pl].ease_best = a1;
            }
        }
    }
    if g.pl[pl].door_move.is_some() { crate::hands::door_move_tick(g, dt); }
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
        if p.sprinting && p.kick.is_none() && p.picking.is_none() && p.coding.is_none() && p.takedown.is_none() && p.holding.is_none() {
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
