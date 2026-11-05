//! WHAT IS THROWN OR DROPPED, IN THE AIR AND ON THE FLOOR (src/game/throwables.ts Throwables.update) — a can, a grenade, a magazine, a gun out of a dying
//! hand, for one tick: it ages, falls, meets the walls, the leaves and the furniture by a ray from its middle, bounces, hands the furniture a share of
//! what it lost, and comes to rest on a floor or on a chair's seat; lying on a thing that has moved, it looks under itself and drops.
//!
//! STATEMENT FOR STATEMENT, as doors.rs, props.rs and slabs.rs are. tools/qa/item-ident.ts and the missions of tools/qa/rust-check.ts hold the two together.
//!
//! WHAT IS STATE. A thing's own numbers are the `items` part of the game's block (state.rs), laid out as the furniture's are (props.rs says why): a
//! column a number, as narrow as its values, the truths bits of one word, hot before cold, every column whole vectors on a 16-byte boundary. THE ROWS
//! ARE THE GAME'S LIST, IN ITS ORDER: row k is `Throwables.items[k]`, the oldest first. That order is the rule's own (a full pool gives up the FIRST
//! spent thing in it; the boxes the picture draws are listed in it; the step walks it from the newest down), so the rows are not sorted by who is
//! stepped: a pool is thirty-two rows. The host moves rows (src/sim/itemshome.ts ItemStore: a thing spawned behind the last, a thing gone and the
//! rows behind it closed up); no rule here does. Twelve doubles are read for every row on every tick and lie first; what only a bounce reads comes
//! after; the fuse, the venting and what a magazine holds are other rules' and are never read here. 184 bytes a row.
//!
//! ONE CALL WALKS THE POOL from a row down to row 0, as the TypeScript's loop does. What a thing does that is not its own numbers is LISTED, in the
//! order it happened, for the host to do when the call is back (`EV`): a bounce loud enough to be said (the host calls the thing's own callback), a
//! share of momentum for a thing of furniture (PropSystem.applyImpulse), a thing whose time is up (the host takes its row out). None of the three is
//! read by a later row's step: furniture is met by where it STANDS, which an impulse does not move.
//!
//! WHAT ONLY THE HOST CAN JUDGE HANDS A ROW BACK. The step works on a copy of the row, and where it comes to a question this module has no answer
//! to — a bounce on ground that may be lying snow (the snow is the host's map: snowBog.ts), a ray of no length or no number — it writes nothing of
//! that row, lists nothing of it, and returns HOST with the row (a pane of glass is judged here, glass.rs): the TypeScript takes that
//! row's step and calls again for the rows under it.
use crate::lines::{ray_rule, ray_things, RAY_HOST};
use crate::maths::{atan2, hypot2, hypot3, round_half_up};
use crate::state::state;
use crate::world::{js_abs, js_max, js_min, world, MAX_THINGS, TH, TH_X, TH_YAW, TH_Z, TL};
use core::mem::{offset_of, size_of};

/// things a pool holds and still keeps them in the module (throwables.ts spawn: a thirty-third gives one up; a pool that could not, and grew, stays in
/// TypeScript). A multiple of 4.
pub const CAP: usize = 32;
const _: () = assert!(CAP % 4 == 0);

// a thing's truths (`flags`): lying at rest · its fuse has run (another rule's: carried here) · in a hand, placed by whoever carries it
pub const F_SETTLED: u32 = 1;
pub const F_FIRED: u32 = 2;
pub const F_CARRIED: u32 = 4;
// the kinds this step tells apart (throwables.ts ITEM_KINDS, by their place there): what share of a bounce's momentum the furniture gets
pub const K_SMOKE: u32 = 0;
pub const K_FRAG: u32 = 3;
pub const K_RIFLE: u32 = 6;

/// The thrown things' part of a game's block (src/sim/itemshome.ts ItemStore lays its own array out the same, byte for byte).
#[repr(C)]
pub struct Items {
    /// things in the pool (rows 0 .. n, the game's list in its order)
    pub n: u32,
    pub pad: [u32; 3],
    // ---- doubles read for every row on every tick
    pub px: [f64; CAP],
    pub py: [f64; CAP],
    pub pz: [f64; CAP],
    pub vx: [f64; CAP],
    pub vy: [f64; CAP],
    pub vz: [f64; CAP],
    pub yaw: [f64; CAP],
    pub spin: [f64; CAP],
    pub tumble: [f64; CAP],
    pub tumble_rate: [f64; CAP],
    pub age: [f64; CAP],
    pub life: [f64; CAP],
    // ---- what a bounce reads (the thing's size and give) and writes (the way it travels; the pose of the thing it lies on, as one number)
    pub radius: [f64; CAP],
    pub restitution: [f64; CAP],
    pub friction: [f64; CAP],
    pub heading: [f64; CAP],
    pub on_pose: [f64; CAP],
    // ---- other rules' own, carried: the fuse, the seconds of venting left, what a magazine holds
    pub fuse: [f64; CAP],
    pub emit_t: [f64; CAP],
    pub fill: [f64; CAP],
    // ---- words
    pub flags: [u32; CAP],
    pub kind: [u32; CAP],
    pub bounces: [u32; CAP],
    pub who: [i32; CAP],
    /// the thing of furniture it lies on: its id PLUS ONE (0: none)
    pub on: [u32; CAP],
    pub serial: [u32; CAP],
}
const _: () = assert!(size_of::<Items>() == 16 + 20 * CAP * 8 + 6 * CAP * 4);
const _: () = assert!(offset_of!(Items, px) % 16 == 0 && offset_of!(Items, flags) % 16 == 0 && (CAP * 4) % 16 == 0);
/// the columns in their order (src/sim/itemshome.ts ITEM_F64 and ITEM_U32 name them the same)
pub const COLS: [usize; 26] = [
    offset_of!(Items, px),
    offset_of!(Items, py),
    offset_of!(Items, pz),
    offset_of!(Items, vx),
    offset_of!(Items, vy),
    offset_of!(Items, vz),
    offset_of!(Items, yaw),
    offset_of!(Items, spin),
    offset_of!(Items, tumble),
    offset_of!(Items, tumble_rate),
    offset_of!(Items, age),
    offset_of!(Items, life),
    offset_of!(Items, radius),
    offset_of!(Items, restitution),
    offset_of!(Items, friction),
    offset_of!(Items, heading),
    offset_of!(Items, on_pose),
    offset_of!(Items, fuse),
    offset_of!(Items, emit_t),
    offset_of!(Items, fill),
    offset_of!(Items, flags),
    offset_of!(Items, kind),
    offset_of!(Items, bounces),
    offset_of!(Items, who),
    offset_of!(Items, on),
    offset_of!(Items, serial),
];

// ---- WHAT THE HOST HANDS IN AND TAKES BACK FOR ONE WALK (scratch)
// in: the frame's dt · the row to start at (the walk goes down from it) · whether the host has a rule for panes · whether the ground may be lying snow
// anywhere now · the share of a bounce's momentum a rifle hands the furniture (throwables.ts RIFLE_ITEM.propShare)
pub const P_DT: usize = 0;
pub const P_FROM: usize = 1;
pub const P_ASK: usize = 2;
pub const P_BOG: usize = 3;
pub const P_RIFLE_SHARE: usize = 4;
// out: the row handed back (HOST) · how many events are listed · counts for the checks (rays asked of the box world, of the furniture, substeps,
// rows walked, rows come to rest)
pub const P_ROW: usize = 8;
pub const P_EVENTS: usize = 9;
pub const P_RAYS: usize = 10;
pub const P_THING_RAYS: usize = 11;
pub const P_SUBSTEPS: usize = 12;
pub const P_WALKED: usize = 13;
pub const P_RESTED: usize = 14;
pub const NPAR: usize = 16;

/// an event: what · the row it is of · a number (a bounce: how fast into the face; an impulse: the id of the thing of furniture) · where · a vector
/// (a bounce: the face's normal; an impulse: the impulse)
pub const EV: usize = 9;
pub const E_KIND: usize = 0;
pub const E_ROW: usize = 1;
pub const E_A: usize = 2;
pub const E_P: usize = 3;
pub const E_V: usize = 6;
pub const EV_BOUNCE: f64 = 0.0;
pub const EV_IMPULSE: f64 = 1.0;
pub const EV_GONE: f64 = 2.0;
/// the most one row lists: its time up, or — a substep — an impulse, a bounce and the floor's bounce, twice
pub const ROW_EVENTS: usize = 6;
pub const MAX_EVENTS: usize = CAP * ROW_EVENTS;

/// the walk's answers: every row down to 0 was taken · row PAR[P_ROW] is the host's to take (nothing of it was written or listed; the rows above it
/// were) · not a walk this could be (no such row)
pub const DONE: u32 = 0;
pub const HOST: u32 = 1;
pub const WRONG: u32 = 2;

static mut PAR: [f64; NPAR] = [0.0; NPAR];
static mut EVS: [f64; MAX_EVENTS * EV] = [0.0; MAX_EVENTS * EV];
#[inline(always)]
fn par() -> &'static mut [f64; NPAR] {
    unsafe { &mut *core::ptr::addr_of_mut!(PAR) }
}
#[inline(always)]
fn evs() -> &'static mut [f64; MAX_EVENTS * EV] {
    unsafe { &mut *core::ptr::addr_of_mut!(EVS) }
}
/// where the hand-over's tables are (byte addresses): 0 par · 1 the events
pub fn addr(table: u32) -> usize {
    match table {
        0 => par().as_ptr() as usize,
        1 => evs().as_ptr() as usize,
        _ => 0,
    }
}
/// 0 = rows · 1 = the hand-over's length · 2 = events the list holds · 3 = doubles an event · 13 = the part's length in bytes · 100 + k = where
/// column k of the part starts (COLS)
pub fn cap(table: u32) -> usize {
    match table {
        0 => CAP,
        1 => NPAR,
        2 => MAX_EVENTS,
        3 => EV,
        13 => size_of::<Items>(),
        k if k >= 100 && ((k - 100) as usize) < COLS.len() => COLS[(k - 100) as usize],
        _ => 0,
    }
}

// ---- throwables.ts's own numbers (tools/qa/rust-check.ts reads both sources and is red where one differs)
const PI: f64 = core::f64::consts::PI;
const GRAVITY: f64 = 9.81;
const SUB: f64 = 2.0;

/// the walk's world, what the host said of it, and what it has asked so far
struct Walk {
    w: usize,
    ask: bool,
    bog: bool,
    rifle_share: f64,
    /// a question came up that only the host can judge
    host: bool,
    rays: f64,
    thing_rays: f64,
    substeps: f64,
}

/// throwables.ts poseKey of the thing of furniture with this id, as the world's copy has it: any change of pose changes it
#[inline(always)]
fn pose_key(w: usize, id: usize) -> f64 {
    let o = world(w);
    o.thing[id * TH + TH_X] * 7919.0 + o.thing[id * TH + TH_Z] * 104729.0 + o.thing[id * TH + TH_YAW]
}
/// PropSystem.raycast as the walk asks it: how far, the normal, and the ID of the thing met
fn things(d: &mut Walk, x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64, tmax: f64) -> Option<(f64, f64, f64, f64, usize)> {
    let mut out = [0.0f64; 5];
    d.thing_rays += 1.0;
    let r = ray_things(d.w, x, y, z, dx, dy, dz, tmax, &mut out);
    if r < 0 {
        return None;
    }
    Some((out[0], out[1], out[2], out[3], world(d.w).thing_live[r as usize * TL] as usize))
}
/// Throwables.ray: the closest hit among the statics and the leaves (collision.ts raycast) and the furniture: how far, the face's normal, and the id
/// of the thing of furniture PLUS ONE (0: a wall, a floor, a leaf)
#[allow(clippy::too_many_arguments)]
fn hit_of(d: &mut Walk, x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64, tmax: f64) -> Option<(f64, f64, f64, f64, usize)> {
    let mut out = [0.0f64; 4];
    d.rays += 1.0;
    let k = ray_rule(d.w, x, y, z, dx, dy, dz, tmax, 0, -1, d.ask, &mut out);
    if k == RAY_HOST {
        d.host = true; // (a ray collision.ts walks itself: the host's question — lines.rs ray_rule)
        return None;
    }
    let st = if k >= 0 { Some((out[0], out[1], out[2], out[3])) } else { None };
    let ph = things(d, x, y, z, dx, dy, dz, if let Some((t, _, _, _)) = st { t } else { tmax });
    if let Some((t, n0, n1, n2, id)) = ph {
        return Some((t, n0, n1, n2, id + 1));
    }
    st.map(|(t, n0, n1, n2)| (t, n0, n1, n2, 0))
}

/// one event onto the list
#[allow(clippy::too_many_arguments)]
#[inline]
fn list(n: &mut usize, kind: f64, row: usize, a: f64, p0: f64, p1: f64, p2: f64, v0: f64, v1: f64, v2: f64) {
    if *n >= MAX_EVENTS {
        return; // (never: a row lists ROW_EVENTS at most, and the list holds that many a row)
    }
    let e = &mut evs()[*n * EV..*n * EV + EV];
    e[E_KIND] = kind;
    e[E_ROW] = row as f64;
    e[E_A] = a;
    e[E_P] = p0;
    e[E_P + 1] = p1;
    e[E_P + 2] = p2;
    e[E_V] = v0;
    e[E_V + 1] = v1;
    e[E_V + 2] = v2;
    *n += 1;
}

/// Throwables.update for the rows PAR[P_FROM] down to 0 of the game in slot `g`, over world `w` (which stands with the game's own leaves and its
/// furniture in it). → DONE · HOST (row PAR[P_ROW]: nothing of it written) · WRONG.
pub fn update(g: usize, w: usize) -> u32 {
    let c = &mut state(g).items;
    let p = par();
    p[P_EVENTS] = 0.0;
    p[P_ROW] = -1.0;
    // (the row as the host wrote it, a double: a number that is no row of the pool is no walk; −1 is a pool with nothing left to walk)
    let n = c.n as usize;
    if p[P_FROM] == -1.0 {
        return DONE;
    }
    if !(p[P_FROM] >= 0.0 && p[P_FROM] < n as f64 && n <= CAP) {
        return WRONG;
    }
    let dt = p[P_DT];
    let mut d = Walk { w, ask: p[P_ASK] != 0.0, bog: p[P_BOG] != 0.0, rifle_share: p[P_RIFLE_SHARE], host: false, rays: 0.0, thing_rays: 0.0, substeps: 0.0 };
    let (mut events, mut walked, mut rested) = (0usize, 0.0, 0.0);
    let mut r = p[P_FROM] as usize;
    let answer = loop {
        // ---- the row, copied: nothing is written back unless the step is this module's to take
        let (mut px, mut py, mut pz, mut vx, mut vy, mut vz) = (c.px[r], c.py[r], c.pz[r], c.vx[r], c.vy[r], c.vz[r]);
        let (mut yaw, mut spin, mut tumble, mut tumble_rate) = (c.yaw[r], c.spin[r], c.tumble[r], c.tumble_rate[r]);
        let (mut heading, mut on_pose, mut bounces, mut on) = (c.heading[r], c.on_pose[r], c.bounces[r], c.on[r]);
        let (radius, restitution, friction, kind, flags) = (c.radius[r], c.restitution[r], c.friction[r], c.kind[r], c.flags[r]);
        let mut settled = flags & F_SETTLED != 0;
        let at = events;
        let age = c.age[r] + dt;
        let life = c.life[r] - dt;
        let mut fly = true;
        if life <= 0.0 {
            list(&mut events, EV_GONE, r, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            fly = false;
        } else if flags & F_CARRIED != 0 {
            fly = false; // (in a hand: it goes where the hand goes)
        } else if settled {
            tumble_rate = 0.0;
            let flat_a = round_half_up(tumble / PI) * PI;
            tumble += (flat_a - tumble) * js_min(1.0, dt * 12.0); // comes to rest lying along its axis (0 or half a turn)
            if on == 0 {
                // lying on a floor, as most of a pool is for most of its life: nothing else of the row is written
                c.age[r] = age;
                c.life[r] = life;
                c.tumble_rate[r] = tumble_rate;
                c.tumble[r] = tumble;
                walked += 1.0;
                if r == 0 {
                    break DONE;
                }
                r -= 1;
                continue;
            }
            // resting on a chair seat / carton: stays put while that thing is still underneath, drops once it has moved out from under it
            fly = false;
            if on != 0 && (on as usize) <= MAX_THINGS && pose_key(w, on as usize - 1) != on_pose {
                let under = things(&mut d, px, py + 0.05, pz, 0.0, -1.0, 0.0, 0.15);
                if matches!(under, Some((_, _, _, _, id)) if id + 1 == on as usize) {
                    on_pose = pose_key(w, on as usize - 1);
                } else {
                    settled = false;
                    on = 0;
                    vx = 0.0;
                    vy = 0.0;
                    vz = 0.0;
                    fly = true;
                }
            }
        }
        if fly {
            let h = dt / SUB;
            let mut s = 0.0;
            while s < SUB && !settled {
                s += 1.0;
                d.substeps += 1.0;
                vy -= GRAVITY * h;
                let (m0, m1, m2) = (vx * h, vy * h, vz * h);
                let len = hypot3(m0, m1, m2);
                if len < 1e-6 {
                    continue;
                }
                let il = 1.0 / len;
                let (d0, d1, d2) = (m0 * il, m1 * il, m2 * il);
                // sphere-vs-box CCD from the centre ray: contact happens radius/cos(incidence) before the centre would hit
                let mut hit = hit_of(&mut d, px, py, pz, d0, d1, d2, len + radius * 3.5);
                if d.host {
                    break;
                }
                let mut back = 0.0;
                if let Some((t, n0, n1, n2, _)) = hit {
                    back = radius / js_max(0.3, -(d0 * n0 + d1 * n1 + d2 * n2));
                    if t - back > len {
                        hit = None;
                    }
                }
                if let Some((t, n0, n1, n2, prop)) = hit {
                    // place at contact, reflect
                    let t_hit = js_max(0.0, t - back);
                    px += d0 * t_hit;
                    py += d1 * t_hit;
                    pz += d2 * t_hit;
                    let vn = vx * n0 + vy * n1 + vz * n2;
                    if vn < 0.0 {
                        if prop != 0 {
                            // the thing's lost momentum goes into the chair / carton it struck
                            let j = vn * (1.0 + restitution) * (if kind == K_SMOKE || kind == K_FRAG { 0.35 } else if kind == K_RIFLE { d.rifle_share } else { 0.1 });
                            list(&mut events, EV_IMPULSE, r, (prop - 1) as f64, px, py, pz, n0 * j, n1 * j, n2 * j);
                        }
                        let (vn0, vn1, vn2) = (n0 * vn, n1 * vn, n2 * vn);
                        let (vt0, vt1, vt2) = (vx - vn0, vy - vn1, vz - vn2);
                        let speed = -vn;
                        if d.bog && n1 > 0.6 && prop == 0 {
                            d.host = true; // (lying snow may be under it: the host's map)
                            break;
                        }
                        let f = js_max(0.0, 1.0 - friction * (0.5 + 0.5 * js_abs(n1)));
                        vx = vt0 * f + vn0 * -restitution;
                        vy = vt1 * f + vn1 * -restitution;
                        vz = vt2 * f + vn2 * -restitution;
                        spin *= 0.6;
                        tumble_rate *= 0.45;
                        bounces = bounces.wrapping_add(1);
                        if hypot2(vx, vz) > 0.3 {
                            heading = atan2(vx, vz);
                        }
                        if speed > 0.6 {
                            list(&mut events, EV_BOUNCE, r, speed, px, py, pz, n0, n1, n2);
                        }
                        // settle when slow on an upward-facing surface
                        if n1 > 0.6 && js_abs(vy) < 0.5 && hypot2(vx, vz) < 0.35 {
                            settled = true;
                            on = prop as u32;
                            on_pose = if prop != 0 { pose_key(w, prop - 1) } else { 0.0 };
                            vx = 0.0;
                            vy = 0.0;
                            vz = 0.0;
                            spin = 0.0;
                            py += 0.001;
                        }
                    }
                    px += n0 * 0.002;
                    py += n1 * 0.002;
                    pz += n2 * 0.002;
                } else {
                    px += m0;
                    py += m1;
                    pz += m2;
                }
                if py < radius {
                    // safety floor
                    if d.bog {
                        d.host = true;
                        break;
                    }
                    py = radius;
                    if vy < 0.0 {
                        let sp = -vy;
                        vy *= -restitution;
                        vx *= 0.7;
                        vz *= 0.7;
                        bounces = bounces.wrapping_add(1);
                        if sp > 0.6 {
                            list(&mut events, EV_BOUNCE, r, sp, px, py, pz, 0.0, 1.0, 0.0);
                        }
                    }
                    if js_abs(vy) < 0.5 && hypot2(vx, vz) < 0.35 {
                        settled = true;
                        vx = 0.0;
                        vy = 0.0;
                        vz = 0.0;
                    }
                }
                yaw += spin * h;
                tumble += tumble_rate * h;
            }
        }
        if d.host {
            events = at; // (what the row listed is not said: its step is the host's, whole)
            d.host = false;
            p[P_ROW] = r as f64;
            break HOST;
        }
        c.age[r] = age;
        c.life[r] = life;
        c.px[r] = px;
        c.py[r] = py;
        c.pz[r] = pz;
        c.vx[r] = vx;
        c.vy[r] = vy;
        c.vz[r] = vz;
        c.yaw[r] = yaw;
        c.spin[r] = spin;
        c.tumble[r] = tumble;
        c.tumble_rate[r] = tumble_rate;
        c.heading[r] = heading;
        c.on_pose[r] = on_pose;
        c.bounces[r] = bounces;
        c.on[r] = on;
        if settled && flags & F_SETTLED == 0 {
            rested += 1.0;
        }
        c.flags[r] = (flags & !F_SETTLED) | if settled { F_SETTLED } else { 0 };
        walked += 1.0;
        if r == 0 {
            break DONE;
        }
        r -= 1;
    };
    p[P_EVENTS] = events as f64;
    p[P_RAYS] = d.rays;
    p[P_THING_RAYS] = d.thing_rays;
    p[P_SUBSTEPS] = d.substeps;
    p[P_WALKED] = walked;
    p[P_RESTED] = rested;
    answer
}
