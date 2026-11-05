//! src/game/walkahead.ts lines 1 to 83
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `seat`'s man walked through `cmds` (oldest first, `dt` seconds each) from where the game has him, on a copy of him: writes nothing to the game.
/// `path`, when given, gets his place after each record; its length is set. (walkahead.ts walkAhead)
pub fn walk_ahead(g: &mut crate::game::Game, seat: SeatId, cmds: &[crate::cmd::PlayerCmd], dt: f64, path: Option<&mut Vec<V3>>) -> crate::walkahead::Ahead {
    let mut path = path;
    let man = crate::cmd::seat_man(g, seat);
    crate::seats::with_seat(g, man, &mut |g: &mut Game| walk_in_view(g, seat, cmds, dt, path.as_deref_mut()))
}

/// The body of walkAhead, with `g` as the seat's view.
fn walk_in_view(g: &mut Game, seat: SeatId, cmds: &[crate::cmd::PlayerCmd], dt: f64, path: Option<&mut Vec<V3>>) -> Ahead {
    let mut path = path;
    let pl = g.player;
    let ch = g.pl[pl].ch;
    let n = cmds.len();
    let free = crate::player::legs_own(g);
    let (ch_pos, ch_vel, ch_body_yaw, ch_aim_yaw) = (g.ch[ch].pos, g.ch[ch].vel, g.ch[ch].body_yaw, g.ch[ch].aim_yaw);
    let mut m = crate::player::Legs {
        char: crate::player::LegsChar { pos: ch_pos, vel: ch_vel, body_yaw: ch_body_yaw, aim_yaw: ch_aim_yaw },
        safe: g.pl[pl].safe,
        crouch: g.pl[pl].crouch,
        sprinting: g.pl[pl].sprinting,
        dash: g.pl[pl].dash,
        backpedal: g.pl[pl].backpedal,
        speed_sm: g.pl[pl].speed_sm,
        pace: 1.0, // TWOGEARS.on is settled on the page
        dragging: None,
        haul: None,
        shoved: [0.0, 0.0],
        shoved_t: -1.0,
        shoved_from: [0.0, 0.0],
        down: g.pl[pl].down,
        door_rip_until: g.pl[pl].door_rip_until,
        door_bash_at: g.pl[pl].door_bash_at,
        door_bash_lean: g.pl[pl].door_bash_lean,
    };
    let mut aim: Option<V3> = Some(g.aim_point);
    if let Some(p) = path.as_deref_mut() { p.resize(n, [0.0; 3]); }
    if free && n > 0 {
        crate::game::game_aside(g, &mut |g: &mut Game| {
            walk_records(g, seat, pl, ch, cmds, dt, &mut m, &mut aim, path.as_deref_mut());
        });
    } else if let Some(p) = path.as_deref_mut() {
        for i in 0..n { p[i] = g.ch[ch].pos; }
    }
    let f = crate::game::game_follow_pos(g, Some(m.char.pos));
    Ahead { pos: m.char.pos, vel: m.char.vel, aim_yaw: m.char.aim_yaw, body_yaw: m.char.body_yaw, crouch: m.crouch, follow: f, aim, free }
}

/// The closure handed to Game.aside: every record, walked on `m`. (walkahead.ts:49 to 78)
fn walk_records(g: &mut Game, seat: SeatId, pl: PlayerId, ch: CharId, cmds: &[crate::cmd::PlayerCmd], dt: f64, m: &mut crate::player::Legs, aim: &mut Option<V3>, path: Option<&mut Vec<V3>>) {
    let mut path = path;
    crate::game::game_aside_lens(g); // the lens is g.alone_own.lens: the seams below act on it, never on the seat's own camera
    let mut swing = crate::aimweight::AimSwingW { vel: g.pl[pl].aimw.vel };
    let weighs = crate::aimweight::aim_weighs(g);
    let top = if weighs { crate::aimweight::aim_top(g, pl) } else { 0.0 };
    let mut weighed = g.pl[pl].aimw.live;
    let mut now = g.time;
    for i in 0..cmds.len() {
        now += dt; // the tick's clock has moved on before it reads him (game.ts stepSeats)
        let near = if g.tab.cmd_seat[seat].remote { Some(m.char.pos) } else { None };
        let r = crate::cmd::honest_copy(g, &cmds[i], near);
        seam_load_lens(g, &r.view);
        // the cursor's point, before he moves (updateAim)
        let u = r.mx / m::max(1.0, r.w);
        let w = r.my / m::max(1.0, r.h);
        let ray = seam_lens_ray(g, u, w);
        let own = g.ch[ch].id;
        let hit = crate::player::cursor_hit(g, crate::player::CursorHitRay { ro: ray.ro, rd: ray.rd }, own);
        let hit = hit.map(|h| crate::player::CursorPointHit { t: h.t });
        let at = match seam_cursor_point(g, u, w, crate::player::CursorPointRay { ro: ray.ro, rd: ray.rd }, hit) {
            Some(p) => p,
            None => crate::player::aim_level(crate::player::AimLevelC { pos: m.char.pos, body_yaw: m.char.body_yaw }),
        };
        *aim = Some(at);
        // his keys, his stance, his stride (updatePlayer's movement block for a man whose legs are his own)
        no_shove_begin_legs(g, m);
        let (mut mx, mut mz) = (0.0, 0.0);
        if !crate::game::player_list_open(g, pl) && !crate::samstun::sam_rooted(g, Some(now)) {
            let b = seam_lens_planar_basis(g);
            let way = crate::player::keys_way(b.fwd, b.right, has(&r.keys, "KeyW"), has(&r.keys, "KeyS"), has(&r.keys, "KeyD"), has(&r.keys, "KeyA"));
            mx = way[0];
            mz = way[1];
        }
        let ml = m::hyp2(mx, mz);
        if ml > 0.0 { mx /= ml; mz /= ml; }
        if has(&r.pressed, "KeyC") { crate::player::crouch_key(g, m); }
        let want_sprint = !crate::condition::is_hurt(g, crate::condition::Man::Player(pl), Some(now))
            && (has(&r.keys, "ShiftLeft") || has(&r.keys, "ShiftRight"))
            && ml > 0.0;
        crate::player::sprint_stance(g, m, want_sprint, false);
        if g.k.tapstep.on && m.crouch && !want_sprint {
            // tapstep.ts: a crouch that begins inside the records; his keys are steps from here, not walked
            m.char.vel = [0.0, 0.0, 0.0];
            m.speed_sm = 0.0;
        } else {
            crate::player::stride_step(g, m, mx, mz, ml, None, None, dt, Some(now));
        }
        // his facing: the run's in a sprint, else the aim's swing to the cursor and the hips after it
        if m.sprinting {
            crate::player::sprint_facing(g, m, dt, Some(now));
            weighed = false;
        } else {
            let dx = at[0] - m.char.pos[0];
            let dz = at[2] - m.char.pos[2];
            let d = m::hyp2(dx, dz);
            if weighs {
                if !weighed { swing.vel = 0.0; }
                weighed = true;
                if d > 0.25 && dt > 0.0 {
                    let err0 = v3::wrap_angle(m::atan2(dx, dz) - m.char.aim_yaw);
                    let left = crate::aimweight::aim_swing(g, &mut swing, err0, top, dt);
                    m.char.aim_yaw += err0 - left;
                } else {
                    swing.vel = 0.0;
                }
            } else if d > 0.25 {
                m.char.aim_yaw = crate::player::aim_ease(m.char.aim_yaw, [dx, 0.0, dz], dt);
            }
            let aiming = crate::player::gun_raised(g, pl, Some(now), Some(crate::player::GunRaisedM { sprinting: m.sprinting }));
            let dp = g.ch[ch].anim.door_push.as_ref().map(|d| crate::player::LeafFacingDp { w: d.w, n: d.n.to_vec(), hold_only: d.hold_only });
            let into_door = crate::player::leaf_facing(dp.as_ref(), aiming);
            let speed = m::hyp2(m.char.vel[0], m.char.vel[2]);
            crate::player::hips_turn(m, speed, aiming, false, into_door, false, dt);
        }
        if let Some(p) = path.as_deref_mut() { p[i] = m.char.pos; }
    }
}

/// `r.keys.includes(k)`
fn has(list: &[String], k: &str) -> bool {
    list.iter().any(|x| x == k)
}

/// noShoveBegin on the copy of him: the rule's own shape lent his fields and given them back (Legs extends Shoved in the TypeScript).
fn no_shove_begin_legs(g: &mut Game, m: &mut crate::player::Legs) {
    let mut s = crate::noshove::Shoved {
        char: crate::noshove::ShovedChar { pos: vec![m.char.pos[0], m.char.pos[1], m.char.pos[2]] },
        shoved: m.shoved,
        shoved_t: m.shoved_t,
        shoved_from: m.shoved_from,
    };
    crate::noshove::no_shove_begin(g, Some(&mut s));
    m.shoved = s.shoved;
    m.shoved_t = s.shoved_t;
    m.shoved_from = s.shoved_from;
}

// The seams: each is one step on the aside lens that the skeleton cannot yet say. See notes.md, ## Skeleton.

/// `loadLens(cam, r.view)` on the aside lens g.alone_own.lens (walkahead.ts:54).
fn seam_load_lens(g: &mut Game, v: &crate::cmd::PlayerCmdView) {
    todo!("needs: load_lens to take cam: &mut FollowCamera; the stub dropped cam")
}
/// `cam.ray(u, w)` of the aside lens (walkahead.ts:56).
fn seam_lens_ray(g: &mut Game, u: f64, w: f64) -> crate::camera::FollowCameraRay {
    todo!("needs: follow_camera_ray to take this: &FollowCamera")
}
/// `cursorPoint(cam, u, w, ray, hit)` with the aside lens (walkahead.ts:57).
fn seam_cursor_point(g: &mut Game, u: f64, w: f64, ray: crate::player::CursorPointRay, hit: Option<crate::player::CursorPointHit>) -> Option<V3> {
    todo!("needs: cursor_point to take cam: &FollowCamera; the stub dropped cam")
}
/// `cam.planarBasis()` of the aside lens (walkahead.ts:61).
fn seam_lens_planar_basis(g: &mut Game) -> crate::camera::FollowCameraPlanarBasis {
    todo!("needs: follow_camera_planar_basis to take this: &FollowCamera")
}
