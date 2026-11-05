//! src/game/character.ts lines 3085 to 3539
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- the one seam left: one step the skeleton cannot yet do (notes.md, ## Skeleton)

/// pose.at, the pose's place for a layer of rust/sim: a by-value Pose has none (index: no Pose.at)
fn seam_pose_at(pose: &crate::anim_skeleton::Pose) -> usize { crate::anim_skeleton::b_01::stage_pose(pose) } // SKELETON-GAP: Pose.at (skeleton.ts:145): the bring-up stages the pose, anim_skeleton/b_01.rs

// ---- small helpers

/// a clip of the animator's by the name the TypeScript reads it (this.clips.<name>)
fn clip_of(g: &Game, c: CharId, name: &'static str) -> ClipId { *g.ch[c].anim.clips.get(name).unwrap() }
/// POSE_BENCH.args
fn args_at(g: &Game) -> f64 { g.m.anim_skeleton.pose_bench.args }
/// where a layer's columns go on the bench's args (colsToBench's default `at`)
const COLS: usize = crate::anim_skeleton::LAYER_COLS_AT as usize;
/// where the hand pass's columns go on the bench's args
const HANDS: usize = crate::anim_skeleton::HAND_COLS_AT as usize;
/// doubles onto the bench's args from offset `at` (the TypeScript's a[POSE_BENCH.args + at])
fn args_put(at: usize, v: &[f64]) { ::sim::skeleton::bench().args[at..at + v.len()].copy_from_slice(v); }
/// doubles back off the bench's args from offset `at`
fn args_read(at: usize, out: &mut [f64]) { let n = out.len(); out.copy_from_slice(&::sim::skeleton::bench().args[at..at + n]); }
/// layerSlot(this.ch, this.pA, this.cast): pose and cast live in g, so they stand out of it for the call
fn slot_pa(g: &mut Game, c: CharId) -> f64 {
    let rig = g.ch[c].anim.rig;
    let pose = core::mem::replace(&mut g.ch[c].anim.p_a, crate::anim_skeleton::Pose::blank());
    let cast = core::mem::replace(&mut g.ch[c].anim.cast, crate::anim_skeleton::LayerCast::blank());
    let slot = crate::anim_skeleton::layer_slot(g, rig, &pose, &cast);
    g.ch[c].anim.p_a = pose;
    g.ch[c].anim.cast = cast;
    slot
}
/// layerSlot(this.ch, pose, this.cast)
fn slot_for(g: &mut Game, c: CharId, pose: &crate::anim_skeleton::Pose) -> f64 {
    let rig = g.ch[c].anim.rig;
    let cast = core::mem::replace(&mut g.ch[c].anim.cast, crate::anim_skeleton::LayerCast::blank());
    let slot = crate::anim_skeleton::layer_slot(g, rig, pose, &cast);
    g.ch[c].anim.cast = cast;
    slot
}
/// layerRan(code, this.fk): the fk lives in g, so it stands out of it for the call
fn ran(g: &mut Game, c: CharId, code: f64) {
    let mut fk = core::mem::replace(&mut g.ch[c].anim.fk, crate::anim_skeleton::PoseFK::blank());
    crate::anim_skeleton::layer_ran(g, code, &mut fk);
    g.ch[c].anim.fk = fk;
}
/// a layer's slot as rust/sim takes it: layer_slot answers -1 for the bench, and `-1.0 as usize` would name game slot 0
fn slot_no(slot: f64) -> usize { if slot < 0.0 { ::sim::skeleton::ON_BENCH } else { slot as usize } }
/// SIGNAL_NO[kind]
fn signal_no_of(kind: SignalKind) -> f64 { *crate::character::signal_no().get(String::from(kind.as_str())).unwrap() }
/// SIGNAL_DUR[kind]
fn signal_dur_of(kind: SignalKind) -> f64 { *crate::character::signal_dur().get(String::from(kind.as_str())).unwrap() }
/// TELL_DUR[kind]
fn tell_dur_of(kind: TellKind) -> f64 { *crate::character::tell_dur().get(String::from(kind.as_str())).unwrap() }

// ---- the layers of rust/sim (args: their numbers already on the bench)

fn ask_crouch_pose(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::crouch_pose(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_b), seam_pose_at(&a.p_c)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_run_arms(slot: f64, pose: &crate::anim_skeleton::Pose) -> f64 { ({ let ran__ = ::sim::layers::run_arms(slot_no(slot), seam_pose_at(pose)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_soft_knees(slot: f64, pose: &crate::anim_skeleton::Pose) -> f64 { ({ let ran__ = ::sim::layers::soft_knees(slot_no(slot), seam_pose_at(pose)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_run_plant(g: &Game, c: CharId, slot: f64) -> f64 { ({ let ran__ = ::sim::layers::run_plant(slot_no(slot), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_life(g: &Game, c: CharId, slot: f64) -> f64 { ({ let ran__ = ::sim::layers::life_layer(slot_no(slot), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_sneak_body(g: &Game, c: CharId, slot: f64) -> f64 { ({ let ran__ = ::sim::layers::sneak_body(slot_no(slot), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_sneak_arms(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::sneak_arms(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_b)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_steady(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::steady_layer(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_c)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_carry_arms(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::carry_arms(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_b), seam_pose_at(&a.p_aim)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_long_body(g: &Game, c: CharId, slot: f64) -> f64 { ({ let ran__ = ::sim::layers::long_body(slot_no(slot), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_long_arms(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::long_arms(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_b), seam_pose_at(&a.p_aim), seam_pose_at(&a.rest)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_wall_body(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::wall_body(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_c), seam_pose_at(&a.p_b), seam_pose_at(&a.rest)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_wall_arms(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::wall_arms(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_b), seam_pose_at(&a.rest)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_kick_body(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::kick_body(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_c)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_signal(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::signal_layer(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_b), seam_pose_at(&a.rest)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_smoke_pose(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::smoke_pose(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_b), seam_pose_at(&a.rest)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_nv_pose(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::nv_pose(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_b), seam_pose_at(&a.rest)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }
fn ask_tell(g: &Game, c: CharId, slot: f64) -> f64 { let a = &g.ch[c].anim; ({ let ran__ = ::sim::layers::tell_layer(slot_no(slot), seam_pose_at(&a.p_a), seam_pose_at(&a.p_b), seam_pose_at(&a.rest)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64 }

// ---- the layers' columns (the static LayerCols tables: get and put, by the order rust/sim/src/layers.rs gives)

/// (character.ts CharacterAnimator.runPlantCols get)
fn run_plant_cols_get(a: &Anim) -> [f64; 4] { [a.plant_l.dev, a.plant_l.pz, a.plant_r.dev, a.plant_r.pz] }
/// (character.ts CharacterAnimator.runPlantCols put)
fn run_plant_cols_put(a: &mut Anim, c: &[f64; 4]) { a.plant_l.dev = c[0]; a.plant_l.pz = c[1]; a.plant_r.dev = c[2]; a.plant_r.pz = c[3]; }

/// (character.ts CharacterAnimator.lifeCols get)
fn life_cols_get(a: &Anim) -> [f64; 40] {
    let (l, f) = (&a.life, &a.feet);
    let mut m = [0.0; 40];
    m[0] = l.breath; m[1] = l.exert; m[2] = l.prev_s; m[3] = l.accel; m[4] = l.mass.x; m[5] = l.mass.v; m[6] = l.mass_v.x; m[7] = l.mass_v.v; m[8] = l.still_t; m[9] = l.shift_t;
    m[10] = l.shift_side; m[11] = l.shift.x; m[12] = l.shift.v; m[13] = l.yaw_prev; m[14] = l.yaw_rate; m[15] = l.prev_pos[0]; m[16] = l.prev_pos[1]; m[17] = l.prev_pos[2]; m[18] = l.root_v; m[19] = l.shift_n;
    m[20] = f.w; m[21] = f.next;
    let q = 22;
    m[q] = f.l.p[0]; m[q + 1] = f.l.p[1]; m[q + 2] = f.l.p[2]; m[q + 3] = f.l.yaw; m[q + 4] = f.l.swing; m[q + 5] = f.l.from[0]; m[q + 6] = f.l.from[1]; m[q + 7] = f.l.from[2]; m[q + 8] = f.l.from_yaw;
    let q = 31;
    m[q] = f.r.p[0]; m[q + 1] = f.r.p[1]; m[q + 2] = f.r.p[2]; m[q + 3] = f.r.yaw; m[q + 4] = f.r.swing; m[q + 5] = f.r.from[0]; m[q + 6] = f.r.from[1]; m[q + 7] = f.r.from[2]; m[q + 8] = f.r.from_yaw;
    m
}
/// (character.ts CharacterAnimator.lifeCols put)
fn life_cols_put(a: &mut Anim, m: &[f64; 40]) {
    let l = &mut a.life;
    l.breath = m[0]; l.exert = m[1]; l.prev_s = m[2]; l.accel = m[3]; l.mass.x = m[4]; l.mass.v = m[5]; l.mass_v.x = m[6]; l.mass_v.v = m[7]; l.still_t = m[8]; l.shift_t = m[9];
    l.shift_side = m[10]; l.shift.x = m[11]; l.shift.v = m[12]; l.yaw_prev = m[13]; l.yaw_rate = m[14]; l.prev_pos = [m[15], m[16], m[17]]; l.root_v = m[18]; l.shift_n = m[19];
    let f = &mut a.feet;
    f.w = m[20]; f.next = m[21];
    let q = 22;
    f.l.p = [m[q], m[q + 1], m[q + 2]]; f.l.yaw = m[q + 3]; f.l.swing = m[q + 4]; f.l.from = [m[q + 5], m[q + 6], m[q + 7]]; f.l.from_yaw = m[q + 8];
    let q = 31;
    f.r.p = [m[q], m[q + 1], m[q + 2]]; f.r.yaw = m[q + 3]; f.r.swing = m[q + 4]; f.r.from = [m[q + 5], m[q + 6], m[q + 7]]; f.r.from_yaw = m[q + 8];
}

/// (character.ts CharacterAnimator.longArmCols get)
fn long_arm_cols_get(a: &Anim) -> [f64; 6] { [a.lg.lat[0], a.lg.lat[1], a.lg.pull[0], a.lg.pull[1], a.lg.pull[2], a.sights_lift] }
/// (character.ts CharacterAnimator.longArmCols put)
fn long_arm_cols_put(a: &mut Anim, c: &[f64; 6]) { a.lg.lat[0] = c[0]; a.lg.lat[1] = c[1]; a.lg.pull[0] = c[2]; a.lg.pull[1] = c[3]; a.lg.pull[2] = c[4]; a.sights_lift = c[5]; }

/// (character.ts CharacterAnimator.wallCols get)
fn wall_cols_get(a: &Anim) -> [f64; 13] {
    let mut c = [0.0; 13];
    c[0] = a.wall_gait; c[1] = if a.wtk_hold { 1.0 } else { 0.0 }; c[2] = if a.wtk.is_some() { 1.0 } else { 0.0 };
    if let Some(k) = &a.wtk {
        for i in 0..3 { c[3 + i] = k.l.p[i]; c[7 + i] = k.r.p[i]; }
        c[6] = k.l.yaw; c[10] = k.r.yaw; c[11] = k.first; c[12] = k.last;
    }
    c
}
/// (character.ts CharacterAnimator.wallCols put)
fn wall_cols_put(a: &mut Anim, c: &[f64; 13]) {
    a.wall_gait = c[0]; a.wtk_hold = c[1] != 0.0;
    a.wtk = if js::truthy(c[2]) {
        Some(AnimWtk {
            l: AnimWtkL { p: [c[3], c[4], c[5]], yaw: c[6] },
            r: AnimWtkR { p: [c[7], c[8], c[9]], yaw: c[10] },
            first: if js::truthy(c[11]) { 1.0 } else { 0.0 },
            last: c[12],
        })
    } else { None };
}

/// (character.ts CharacterAnimator.handCols get)
fn hand_cols_get(a: &Anim) -> [f64; 130] {
    let mut c = [0.0; 130];
    let h = &a.hn;
    for sd in 0..2 {
        let (b, nt) = (&h.base[sd], &h.note[sd]);
        for i in 0..60 { c[sd * 60 + i] = b[i] as f64; }
        for i in 0..5 { c[120 + sd * 5 + i] = nt[i] as f64; }
    }
    c
}
/// (character.ts CharacterAnimator.handCols put: only the notes come back)
fn hand_cols_put(a: &mut Anim, c: &[f64; 130]) {
    let h = &mut a.hn;
    for sd in 0..2 {
        let nt = &mut h.note[sd];
        for i in 0..5 { nt[i] = c[120 + sd * 5 + i] as f32; }
    }
}

/// (character.ts CharacterAnimator.kickCols get)
fn kick_cols_get(a: &Anim) -> [f64; 15] {
    let mut c = [0.0; 15];
    if let Some(k) = &a.kick_from {
        c[0] = 1.0;
        for i in 0..3 { c[1 + i] = k.l[i]; c[8 + i] = k.r[i]; }
        for i in 0..4 { c[4 + i] = k.r_l[i]; c[11 + i] = k.r_r[i]; }
    }
    c
}
/// (character.ts CharacterAnimator.kickCols put)
fn kick_cols_put(a: &mut Anim, c: &[f64; 15]) {
    a.kick_from = if js::truthy(c[0]) {
        Some(AnimKickFrom { l: [c[1], c[2], c[3]], r_l: [c[4], c[5], c[6], c[7]], r: [c[8], c[9], c[10]], r_r: [c[11], c[12], c[13], c[14]] })
    } else { None };
}

// ---- the crouch's lay

/// the crouch over the stand pose: its numbers onto the bench and the layer   (character.ts crouchLaidAsk)
pub fn anim_crouch_laid_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, ph: f64, w_move: f64, crouch_pose: f64) {
    let n = g.ch[c].anim.p_a.num_nodes as f64;
    let idle_t = g.ch[c].anim.idle_t;
    let idle = clip_of(g, c, "Crouch_Idle_Loop");
    let fwd = clip_of(g, c, "Crouch_Fwd_Loop");
    let idle_no = crate::anim_skeleton::clip_no(g, idle, n);
    let fwd_no = crate::anim_skeleton::clip_no(g, fwd, n);
    let fwd_t = ph * g.tab.clip[fwd].duration;
    args_put(0, &[w_move, crouch_pose, idle_t, idle_no, fwd_no, fwd_t]);
    let code = ask_crouch_pose(g, c, g_2);
    ran(g, c, code);
}

/// RUNARMS: the run's own arms carried close   (character.ts runArms)
pub fn anim_run_arms(g: &mut crate::game::Game, c: CharId, pose: &crate::anim_skeleton::Pose, w: f64, ws: f64) {
    let slot = slot_for(g, c, pose);
    anim_run_arms_ask(g, c, slot, pose, w, ws);
}
/// (character.ts runArmsAsk)
pub fn anim_run_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, pose: &crate::anim_skeleton::Pose, w: f64, ws: f64) {
    args_put(0, &[w, ws]);
    let code = ask_run_arms(g_2, pose);
    ran(g, c, code);
}

/// the sum of a run clip's stances from its toes, with no animator in it   (character.ts stanceOf)
fn stance_of(ty: &[Vec<f64>], tz: &[Vec<f64>]) -> Option<AnimStanceOf> {
    let mm = ty[0].len();
    let mut cs: Vec<f64> = Vec::with_capacity(ty.len());
    for k in 0..ty.len() {
        let ys = &ty[k];
        let lo = m::min_n(ys) + 0.02;
        let (mut sx, mut cx, mut n) = (0.0, 0.0, 0);
        for i in 0..mm {
            if ys[i] <= lo && tz[k][(i + 1) % mm] < tz[k][(i + mm - 1) % mm] {
                sx += m::sin(2.0 * m::PI * i as f64 / mm as f64);
                cx += m::cos(2.0 * m::PI * i as f64 / mm as f64);
                n += 1;
            }
        }
        cs.push(if n != 0 { js::rem(m::atan2(sx, cx) / (2.0 * m::PI) + 1.0, 1.0) } else { m::NAN });
    }
    if !cs[0].is_finite() || !cs[1].is_finite() { return None; }
    let cc = js::rem(m::atan2(m::sin(4.0 * m::PI * cs[0]) + m::sin(4.0 * m::PI * cs[1]), m::cos(4.0 * m::PI * cs[0]) + m::cos(4.0 * m::PI * cs[1])) / (4.0 * m::PI) + 1.0, 0.5);
    Some(AnimStanceOf { c: cc, c_l: cs[0], c_r: cs[1] })
}
/// a run clip's stances from its toes; None if either foot never qualifies   (character.ts stanceOf)
pub fn anim_stance_of(g: &mut crate::game::Game, c: CharId, ty: &[Vec<f64>], tz: &[Vec<f64>]) -> Option<crate::character::AnimStanceOf> {
    stance_of(ty, tz)
}

/// Jog_Fwd_Loop at phase ph with its loop seam closed   (character.ts sampleJog)
pub fn anim_sample_jog(g: &mut crate::game::Game, c: CharId, ph: f64, out: &mut crate::anim_skeleton::Pose) {
    let cj = clip_of(g, c, "Jog_Fwd_Loop");
    let t = ph * g.tab.clip[cj].duration;
    crate::anim_skeleton::sample_clip(&g.tab.clip[cj], t, true, out);
    let k = ph - m::floor(ph) - 0.5;
    let nn = out.rows;
    let seams = g.ch[c].anim.jog_seam.len();
    for at in 0..seams {
        let s = g.ch[c].anim.jog_seam[at];
        if let Some(d) = s.d {
            let o = s.node as usize;
            out.cols[o] = (out.cols[o] as f64 + d[0] * k) as f32;
            out.cols[o + nn] = (out.cols[o + nn] as f64 + d[1] * k) as f32;
            out.cols[o + 2 * nn] = (out.cols[o + 2 * nn] as f64 + d[2] * k) as f32;
        } else {
            let o = 3 * nn + s.node as usize;
            let q = v3::quat::mul(
                [out.cols[o] as f64, out.cols[o + nn] as f64, out.cols[o + 2 * nn] as f64, out.cols[o + 3 * nn] as f64],
                v3::quat::axis_angle(s.axis.unwrap(), s.ang.unwrap() * k),
            );
            out.cols[o] = q[0] as f32;
            out.cols[o + nn] = q[1] as f32;
            out.cols[o + 2 * nn] = q[2] as f32;
            out.cols[o + 3 * nn] = q[3] as f32;
        }
    }
}

/// SOFT_KNEE: each leg re-solved with its reach eased below full extension   (character.ts softKnees)
pub fn anim_soft_knees(g: &mut crate::game::Game, c: CharId, pose: &crate::anim_skeleton::Pose, w: f64) {
    let slot = slot_for(g, c, pose);
    anim_soft_knees_ask(g, c, slot, pose, w);
}
/// (character.ts softKneesAsk)
pub fn anim_soft_knees_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, pose: &crate::anim_skeleton::Pose, w: f64) {
    args_put(0, &[w]);
    let code = ask_soft_knees(g_2, pose);
    ran(g, c, code);
}

/// the run's stance feet (RUN_LINGER)   (character.ts runPlant)
pub fn anim_run_plant(g: &mut crate::game::Game, c: CharId, ph: f64, s: f64, dt: f64, w_run: f64, w_sprint: f64) {
    let slot = slot_pa(g, c);
    anim_run_plant_ask(g, c, slot, ph, s, dt, w_run, w_sprint);
}
/// (character.ts runPlantAsk)
pub fn anim_run_plant_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, ph: f64, s: f64, dt: f64, w_run: f64, w_sprint: f64) {
    let a = &g.ch[c].anim;
    let nums = [ph, s, dt, w_run, w_sprint, if a.reverse { 1.0 } else { 0.0 }, a.run_gait.jog.c_l, a.run_gait.sprint.c_l, a.run_gait.jog.c_r, a.run_gait.sprint.c_r];
    let cols = run_plant_cols_get(a);
    args_put(0, &nums);
    args_put(COLS, &cols);
    let code = ask_run_plant(g, c, g_2);
    ran(g, c, code);
    let mut back = [0.0; 4];
    args_read(COLS, &mut back);
    run_plant_cols_put(&mut g.ch[c].anim, &back);
}

/// idle life and weight: breath, the body's mass, the weight shift, the foot lock   (character.ts lifeLayer)
pub fn anim_life_layer(g: &mut crate::game::Game, c: CharId, dt: f64, s: f64, crouch_pose: f64, w: f64, w_f: f64) {
    let slot = slot_pa(g, c);
    anim_life_layer_ask(g, c, slot, dt, s, crouch_pose, w, w_f);
}
/// (character.ts lifeLayerAsk)
pub fn anim_life_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, s: f64, crouch_pose: f64, w: f64, w_f: f64) {
    let p0 = g.ch[c].pos;   // (posIn is his position array itself: Character.update)
    let a = &g.ch[c].anim;
    let nums = [dt, s, crouch_pose, w, w_f, a.body_yaw_in, p0[0], p0[1], p0[2], a.cr_t];
    let cols = life_cols_get(a);
    args_put(0, &nums);
    args_put(COLS, &cols);
    let code = ask_life(g, c, g_2);
    ran(g, c, code);
    let mut back = [0.0; 40];
    args_read(COLS, &mut back);
    life_cols_put(&mut g.ch[c].anim, &back);
}

/// deliberate movement's body and, if they are in, its arms   (character.ts sneakLayer)
pub fn anim_sneak_layer(g: &mut crate::game::Game, c: CharId, dl: f64, pace: f64, soft: f64, arms_in: f64, surge: f64, dsh: Option<f64>) {
    let dsh = dsh.unwrap_or(0.0);
    anim_sneak_body(g, c, dl, pace, soft, surge, dsh);
    if arms_in > 0.003 { anim_sneak_arms(g, c, arms_in); }
}
/// …its legs and trunk   (character.ts sneakBody)
pub fn anim_sneak_body(g: &mut crate::game::Game, c: CharId, dl: f64, pace: f64, soft: f64, surge: f64, dsh: f64) {
    let slot = slot_pa(g, c);
    anim_sneak_body_ask(g, c, slot, dl, pace, soft, surge, dsh);
}
/// (character.ts sneakBodyAsk)
pub fn anim_sneak_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dl: f64, pace: f64, soft: f64, surge: f64, dsh: f64) {
    let a = &g.ch[c].anim;
    let nums = [dl, pace, soft, surge, dsh, a.upper_w.aim, a.gait.mid];
    args_put(0, &nums);
    let code = ask_sneak_body(g, c, g_2);
    ran(g, c, code);
}
/// …and its arms   (character.ts sneakArms)
pub fn anim_sneak_arms(g: &mut crate::game::Game, c: CharId, arms_in: f64) {
    let slot = slot_pa(g, c);
    anim_sneak_arms_ask(g, c, slot, arms_in);
}
/// (character.ts sneakArmsAsk)
pub fn anim_sneak_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, arms_in: f64) {
    let a = &g.ch[c].anim;
    let nums = [arms_in, a.crouch, a.upper_w.relaxed];
    args_put(0, &nums);
    let code = ask_sneak_arms(g, c, g_2);
    ran(g, c, code);
}

/// MOVES.steady: the crouch walk's waddle taken out; the cycle's own mean is measured once off the clip   (character.ts steadyLayer)
pub fn anim_steady_layer(g: &mut crate::game::Game, c: CharId, w: f64, base: Option<f64>, sunk: Option<f64>) {
    let (base, sunk) = (base.unwrap_or(1.0), sunk.unwrap_or(0.0));
    if g.ch[c].anim.steady_mean.is_none() {
        let (thigh_l, thigh_r, hips) = { let nd = &g.ch[c].anim.nodes; (nd.thigh_l, nd.thigh_r, nd.hips) };
        let clip = clip_of(g, c, "Crouch_Fwd_Loop");
        let dur = g.tab.clip[clip].duration;
        let nn = 24.0;
        let (mut roll, mut x) = (0.0, 0.0);
        let mut i = 0.0;
        while i < nn {
            crate::anim_skeleton::sample_clip(&g.tab.clip[clip], i / nn * dur, true, &mut g.ch[c].anim.p_upper);
            let (p, list) = (g.ch[c].anim.p_upper.clone(), g.ch[c].anim.fk_legs.clone());
            anim_fk_part(g, c, &p, &list);
            let fk = &g.ch[c].anim.fk;
            let (a, b) = (fk.pos[thigh_l], fk.pos[thigh_r]);
            roll += m::atan2(a[1] - b[1], a[0] - b[0]);
            x += fk.pos[hips][0];
            i += 1.0;
        }
        g.ch[c].anim.steady_mean = Some([roll / nn, x / nn]);
    }
    let slot = slot_pa(g, c);
    anim_steady_layer_ask(g, c, slot, w, base, sunk);
}
/// (character.ts steadyLayerAsk)
pub fn anim_steady_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, base: f64, sunk: f64) {
    let a = &g.ch[c].anim;
    let mm = a.steady_mean.unwrap();
    let nums = [w, base, sunk, mm[0], mm[1], a.steady_up];
    args_put(0, &nums);
    let code = ask_steady(g, c, g_2);
    ran(g, c, code);
}

/// both arms onto the pistol   (character.ts carryArms)
pub fn anim_carry_arms(g: &mut crate::game::Game, c: CharId, w: f64, kick_t: f64) {
    let slot = slot_pa(g, c);
    anim_carry_arms_ask(g, c, slot, w, kick_t);
}
/// (character.ts carryArmsAsk)
pub fn anim_carry_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, kick_t: f64) {
    let a = &g.ch[c].anim;
    let (st, gu) = (&a.st, a.grip_up);
    let nums = [w, kick_t, a.idle_t, st.fwd.x, st.up.x, st.side.x, st.pitch.x, st.blade.x, st.tuck.x, gu[0], gu[1], gu[2]];
    args_put(0, &nums);
    let code = ask_carry_arms(g, c, g_2);
    ran(g, c, code);
}

// ---- the long gun

/// which row of LONG_RELOAD this gun's carry takes   (character.ts rlRow)
pub fn anim_rl_row(g: &mut crate::game::Game, c: CharId) -> crate::character::LONGRELOADBox {
    let (back, has_feed) = (g.ch[c].anim.reload_back, g.ch[c].anim.feed.is_some());
    let r = crate::character::long_reload();
    if back && !(has_feed && crate::gunreload::feed_live(g)) {
        let b = r.back;
        crate::character::LONGRELOADBox { at: b.at, pitch: b.pitch, aim_k: b.aim_k, yaw: b.yaw }
    } else {
        r.box_
    }
}
/// the carriage's channels as they are posed: the springs', or those with a reload carry's share taken   (character.ts lgChannels)
pub fn anim_lg_channels(g: &mut crate::game::Game, c: CharId) -> Vec<crate::character::AnimLgChannelsItem> {
    let a = &g.ch[c].anim;
    let cc = &a.rlc;
    if cc.k > 0.0 || cc.h > 0.0 {
        if let Some(ch) = &cc.ch {
            return ch.iter().map(|s| AnimLgChannelsItem { x: s.x }).collect();
        }
    }
    a.lg.ch.iter().map(|s| AnimLgChannelsItem { x: s.x }).collect()
}
/// GUN HAND (`longReload`), once a frame after the carriage's springs: how much of the reload carry the long gun shows   (character.ts reloadCarry)
pub fn anim_reload_carry(g: &mut crate::game::Game, c: CharId, dt: f64, want: crate::character::LongStance) {
    let r = crate::character::long_reload();
    let a = &mut g.ch[c].anim;
    let (mut t, mut d) = (-1.0, 0.0);
    let (out, in_) = (a.rl_b.out, a.rl_b.in_);
    if a.long_gun {
        // (the hand's own clock, as mvArms picks it)
        if a.reload_at >= 0.0 && a.reload_row.is_some() { t = a.reload_at; d = m::max(0.3, a.reload_row.as_ref().unwrap().dur); }
        else if a.rl_t >= 0.0 { t = a.rl_t; d = a.rl_b.dur; }
    }
    a.rlc.still = if t >= 0.0 && t == a.rlc.at { a.rlc.still + dt } else { 0.0 };
    a.rlc.at = t;
    if a.rlc.still > 0.25 || a.sig_t >= 0.0 || a.tell_t >= 0.0 || a.mvs.ts_t >= 0.0 || a.throw_t >= 0.0 || a.strike_t >= 0.0 || a.big_tool.is_some() { t = -1.0; }
    let tuck = if want == LongStance::LowLong || want == LongStance::PatrolLong { v3::clamp(a.gun_wall, 0.0, 1.0) } else { 0.0 };
    let k_t = if t >= 0.0 { bump(t, 0.0, r.in_, d - r.end - r.out, d - r.end) * (1.0 - tuck) } else { 0.0 };
    let h_t = if t >= 0.0 { bump(t, m::max(0.0, out - r.head_in), m::max(0.05, out - 0.05), m::min(in_ + 0.1, d - r.end - r.head_out), m::min(in_ + 0.1 + r.head_out, d - r.end)) } else { 0.0 };
    let step = m::max(0.0, r.rate * dt);
    a.rlc.k += v3::clamp(k_t - a.rlc.k, -step, step);
    a.rlc.h += v3::clamp(h_t - a.rlc.h, -step, step);
    a.rl_shown.carry = a.rlc.k;
    a.rl_shown.head = a.rlc.h;
    if a.rlc.k > 0.0 || a.rlc.h > 0.0 {
        let l = crate::character::lc();
        let count = l.count as usize;
        let (lp, la, ly, lhy, lhp, lhr) = (l.pitch as usize, l.aim_k as usize, l.yaw as usize, l.head_yaw as usize, l.head_pitch as usize, l.head_roll as usize);
        if g.ch[c].anim.rlc.ch.is_none() {
            g.ch[c].anim.rlc.ch = Some((0..count).map(|_| AnimRlcChItem { x: 0.0 }).collect());
        }
        let row = anim_rl_row(g, c);
        let a = &mut g.ch[c].anim;
        let (ck, ch) = (a.rlc.k, a.rlc.h);
        let lg_ch = &a.lg.ch;
        let x = a.rlc.ch.as_mut().unwrap();
        for i in 0..count { x[i].x = lg_ch[i].x; }
        x[lp].x += (row.pitch - x[lp].x) * ck;
        x[la].x += (row.aim_k - x[la].x) * ck;
        x[ly].x += (row.yaw - x[ly].x) * ck;
        x[lhy].x += (r.head.yaw - x[lhy].x) * ch;
        x[lhp].x += (r.head.pitch - x[lhp].x) * ch;
        x[lhr].x += (r.head.roll - x[lhr].x) * ch;
    }
}
/// the long gun, trunk half   (character.ts longBody)
pub fn anim_long_body(g: &mut crate::game::Game, c: CharId, w: f64, run: Option<f64>, sprint: Option<f64>) {
    let (run, sprint) = (run.unwrap_or(0.0), sprint.unwrap_or(0.0));
    if w < 0.003 { return; }
    let slot = slot_pa(g, c);
    anim_long_body_ask(g, c, slot, w, run, sprint);
}
/// (character.ts longBodyAsk)
pub fn anim_long_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, run: f64, sprint: f64) {
    let ch = anim_lg_channels(g, c);
    let l = crate::character::lc();
    let nums = [w, run, sprint, g.ch[c].anim.long_steady, ch[l.blade as usize].x, ch[l.lean as usize].x, ch[l.head_yaw as usize].x, ch[l.head_pitch as usize].x, ch[l.head_roll as usize].x];
    args_put(0, &nums);
    let code = ask_long_body(g, c, g_2);
    ran(g, c, code);
}
/// the long gun, arms   (character.ts longArms)
pub fn anim_long_arms(g: &mut crate::game::Game, c: CharId, w: f64, kick_t: f64, dt: f64, yield_door: bool, keep: bool) {
    if w < 0.003 { return; }
    let slot = slot_pa(g, c);
    anim_long_arms_ask(g, c, slot, w, kick_t, dt, yield_door, keep);
}
/// (character.ts longArmsAsk)
pub fn anim_long_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, kick_t: f64, dt: f64, yield_door: bool, keep: bool) {
    let ch = anim_lg_channels(g, c);
    let at = anim_rl_row(g, c).at;
    let a = &g.ch[c].anim;
    let (gr, mvs, u, f) = (&a.long_grip, &a.mvs, a.grip_up, a.hand_fix[0]);
    let count = crate::character::lc().count as usize;
    let mut mm = vec![0.0; 37 + count];
    mm[0] = w; mm[1] = kick_t; mm[2] = dt; mm[3] = if yield_door { 1.0 } else { 0.0 }; mm[4] = if keep { 1.0 } else { 0.0 }; mm[5] = a.idle_t; mm[6] = a.upper_w.aim; mm[7] = a.aim_pitch; mm[8] = a.shoot_t;
    mm[9] = a.long_steady; mm[10] = a.trained; mm[11] = mvs.yaw; mm[12] = mvs.climb; mm[13] = mvs.back; mm[14] = a.mv2_shown.dip; mm[15] = a.rlc.k;
    mm[19] = a.dp.w_l.x; mm[20] = a.dp.w_r.x;
    for i in 0..3 { mm[16 + i] = at[i]; mm[21 + i] = u[i]; mm[28 + i] = gr.wrist[i]; mm[31 + i] = gr.fingers[i]; mm[34 + i] = gr.back[i]; }
    for i in 0..4 { mm[24 + i] = f[i]; }
    for i in 0..count { mm[37 + i] = ch[i].x; }
    let cols = long_arm_cols_get(a);
    args_put(0, &mm);
    args_put(COLS, &cols);
    anim_hands_ask(g, c, &mut |g: &mut Game| ask_long_arms(g, c, g_2));
    let mut back = [0.0; 6];
    args_read(COLS, &mut back);
    long_arm_cols_put(&mut g.ch[c].anim, &back);
}

// ---- the wall hold

/// the wall hold, body half   (character.ts wallBody)
pub fn anim_wall_body(g: &mut crate::game::Game, c: CharId, w: f64) {
    let slot = slot_pa(g, c);
    anim_wall_body_ask(g, c, slot, w);
}
/// (character.ts wallBodyAsk)
pub fn anim_wall_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64) {
    let p0 = g.ch[c].pos;   // (posIn is his position array itself: Character.update)
    let n = g.ch[c].anim.p_a.num_nodes as f64;
    let idle = clip_of(g, c, "Idle_Loop");
    let crouch_idle = clip_of(g, c, "Crouch_Idle_Loop");
    let idle_no = crate::anim_skeleton::clip_no(g, idle, n);
    let crouch_idle_no = crate::anim_skeleton::clip_no(g, crouch_idle, n);
    let an = &g.ch[c].anim;
    let (ws, cc, gz, t, f) = (&an.wst, an.chest_rest_inv, an.gaze_local, &an.wall_take_feet, &an.idle_feet);
    let mut a = [0.0; 47];
    a[0] = w; a[1] = an.crouch; a[2] = ws.side.x; a[3] = ws.side.v; a[4] = ws.peek.x; a[5] = ws.park.x; a[6] = an.upper_w.aim; a[7] = an.wall_yaw;
    a[8] = an.phase; a[9] = an.wall_hit; a[10] = an.aim_yaw_offset; a[11] = an.look_yaw_extra;
    for k in 0..4 { a[12 + k] = cc[k]; a[34 + k] = f.r_l[k]; a[39 + k] = f.r_r[k]; }
    for k in 0..3 {
        a[16 + k] = gz[k]; a[22 + k] = p0[k];
        a[26 + k] = match t { Some(t) => t.l.p[k], None => 0.0 };
        a[30 + k] = match t { Some(t) => t.r.p[k], None => 0.0 };
    }
    a[19] = an.wall_take; a[20] = if an.stance == Stance::Wall { 1.0 } else { 0.0 }; a[21] = an.body_yaw_in; a[25] = if t.is_some() { 1.0 } else { 0.0 };
    a[29] = match t { Some(t) => t.l.yaw, None => 0.0 };
    a[33] = match t { Some(t) => t.r.yaw, None => 0.0 };
    a[38] = f.l[1]; a[43] = f.r[1];
    a[44] = an.idle_t; a[45] = idle_no; a[46] = crouch_idle_no;
    let cols = wall_cols_get(an);
    args_put(0, &a);
    args_put(COLS, &cols);
    let code = ask_wall_body(g, c, g_2);
    ran(g, c, code);
    let mut back = [0.0; 13];
    args_read(COLS, &mut back);
    wall_cols_put(&mut g.ch[c].anim, &back);
}

/// the wall hold, arms   (character.ts wallArms)
pub fn anim_wall_arms(g: &mut crate::game::Game, c: CharId, w: f64) {
    let slot = slot_pa(g, c);
    anim_wall_arms_ask(g, c, slot, w);
}
/// (character.ts wallArmsAsk)
pub fn anim_wall_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64) {
    let a = &g.ch[c].anim;
    let (ws, f) = (&a.wst, &a.hand_fix);
    let mut mm = [0.0; 15];
    mm[0] = w; mm[1] = a.crouch; mm[2] = ws.side.x; mm[3] = ws.peek.x; mm[4] = a.wall_yaw; mm[5] = a.wall_gait; mm[6] = ws.park.x;
    for sd in 0..2 { for i in 0..4 { mm[7 + sd * 4 + i] = f[sd][i]; } }
    args_put(0, &mm);
    anim_hands_ask(g, c, &mut |g: &mut Game| ask_wall_arms(g, c, g_2));
}

/// ask a layer that shapes a hand: the hand pass's columns over, the layer, the notes back   (character.ts handsAsk)
pub fn anim_hands_ask(g: &mut crate::game::Game, c: CharId, layer: &mut dyn FnMut(&mut crate::game::Game) -> f64) {
    let at = args_at(g) + crate::anim_skeleton::HAND_COLS_AT;
    let cols = hand_cols_get(&g.ch[c].anim);
    args_put(COLS, &cols);
    let code = layer(g);
    ran(g, c, code);
    let mut back = [0.0; 130];
    args_read(COLS, &mut back);
    hand_cols_put(&mut g.ch[c].anim, &back);
}

// ---- the kick, signals, a smoke, the goggles, a tell

/// the kick, body half   (character.ts kickBody)
pub fn anim_kick_body(g: &mut crate::game::Game, c: CharId, t: f64, w: f64) {
    let slot = slot_pa(g, c);
    anim_kick_body_ask(g, c, slot, t, w);
}
/// (character.ts kickBodyAsk)
pub fn anim_kick_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64, w: f64) {
    let an = &g.ch[c].anim;
    let f = &an.idle_feet;
    let mut a = [0.0; 17];
    a[0] = t; a[1] = w; a[2] = if an.kick_leg == AnimKickLeg::Left { 1.0 } else { 0.0 };
    for k in 0..3 { a[3 + k] = f.l[k]; a[10 + k] = f.r[k]; }
    for k in 0..4 { a[6 + k] = f.r_l[k]; a[13 + k] = f.r_r[k]; }
    let cols = kick_cols_get(an);
    args_put(0, &a);
    args_put(COLS, &cols);
    let code = ask_kick_body(g, c, g_2);
    ran(g, c, code);
    let mut back = [0.0; 15];
    args_read(COLS, &mut back);
    kick_cols_put(&mut g.ch[c].anim, &back);
}

/// left-hand signals   (character.ts signalLayer)
pub fn anim_signal_layer(g: &mut crate::game::Game, c: CharId, t: f64) {
    let slot = slot_pa(g, c);
    anim_signal_layer_ask(g, c, slot, t);
}
/// (character.ts signalLayerAsk)
pub fn anim_signal_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64) {
    let a = &g.ch[c].anim;
    let (f, kind) = (a.hand_fix[0], a.sig_kind);
    let nums = [t, signal_no_of(kind), signal_dur_of(kind), a.sig_side, f[0], f[1], f[2], f[3]];
    args_put(0, &nums);
    anim_hands_ask(g, c, &mut |g: &mut Game| ask_signal(g, c, g_2));
}

/// a smoke's arm, its clock   (character.ts smokeLayer)
pub fn anim_smoke_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool) {
    let k = crate::character::smokearm();
    let a = &mut g.ch[c].anim;
    let want = match a.smoke { Some(s) if free => v3::clamp(s, 0.0, 1.0), _ => 0.0 };
    let step = (if free { k.rate } else { k.away }) * dt;
    a.smoke_arm += v3::clamp(want - a.smoke_arm, -step, step);
    if a.kit.is_some() { a.smoke_arm = 0.0; }
    if a.smoke_arm <= 0.004 { if want <= 0.0 { a.smoke_arm = 0.0; } return; }
    anim_smoke_pose(g, c);
}
/// the smoke's arm itself   (character.ts smokePose)
pub fn anim_smoke_pose(g: &mut crate::game::Game, c: CharId) {
    let slot = slot_pa(g, c);
    anim_smoke_pose_ask(g, c, slot);
}
/// what a hand brought to the head is handed after its own numbers: `o` is the bench's index, as in the TypeScript   (character.ts headHandToBench)
pub fn anim_head_hand_to_bench(g: &mut crate::game::Game, c: CharId, o: f64) {
    let a = &g.ch[c].anim;
    let (tw, r, f) = (a.nv_tw, a.head_rest_inv, a.hand_fix[0]);
    let mut v = [0.0; 12];
    for i in 0..4 { v[i] = tw[i]; v[4 + i] = r[i]; v[8 + i] = f[i]; }
    args_put(0, &v);
}
/// (character.ts smokePoseAsk)
pub fn anim_smoke_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64) {
    let o = args_at(g);
    let arm = g.ch[c].anim.smoke_arm;
    args_put(0, &[arm]);
    anim_head_hand_to_bench(g, c, o + 1.0);
    anim_hands_ask(g, c, &mut |g: &mut Game| ask_smoke_pose(g, c, g_2));
}

/// KIT.nv: one frame of a goggle flip   (character.ts nvLayer)
pub fn anim_nv_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool) {
    let (h, o) = (g.k.kit.nv.hand, g.k.kit.nv.nod);
    let a = &mut g.ch[c].anim;
    let nv_nod = a.nv_nod;
    {
        let n = a.nv.as_mut().unwrap();
        if n.hand && !free && n.t < h.flip[0] { n.hand = false; n.t = 0.0; n.c0 = nv_nod; }
        n.t += dt;
    }
    let n = a.nv.unwrap();
    let dd = if n.hand { h.dur } else { o.dur };
    let fw = if n.hand { h.flip } else { o.flip };
    let shown = n.k0 + ((if n.down { 1.0 } else { 0.0 }) - n.k0) * v3::smoothstep(fw[0], fw[1], n.t);
    a.nv_shown = Some(shown);
    let want = if n.hand && free { bump(n.t, 0.0, h.reach, dd - h.out, dd) } else { 0.0 };
    let step = (if n.hand && free { h.rate } else if a.reload_t >= 0.0 || a.rl_t >= 0.0 || a.dp.w_l.x >= 0.05 { h.clear } else { h.away }) * dt;
    a.nv_arm += v3::clamp(want - a.nv_arm, -step, step);
    if a.kit.is_some() { a.nv_arm = 0.0; }
    if n.t >= dd && a.nv_arm < 0.004 { anim_nv_cancel(g, c); return; }
    if a.nv_arm > 0.004 { anim_nv_pose(g, c, shown); }
    let aa = if n.down { o.on } else { o.off };
    let nod = (if n.hand { 0.0 } else { (aa[0] * bump(n.t, 0.0, o.pre, o.pre, o.pre * 2.0) + aa[1] * bump(n.t, o.pre, o.peak, o.peak, dd)) * v3::DEG }) + n.c0 * (1.0 - v3::smoothstep(0.0, o.carry, n.t));
    g.ch[c].anim.nv_nod = nod;
}
/// the goggle flip's arm itself   (character.ts nvPose)
pub fn anim_nv_pose(g: &mut crate::game::Game, c: CharId, k: f64) {
    let slot = slot_pa(g, c);
    anim_nv_pose_ask(g, c, slot, k);
}
/// (character.ts nvPoseAsk)
pub fn anim_nv_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, k: f64) {
    let o = args_at(g);
    let arm = g.ch[c].anim.nv_arm;
    args_put(0, &[arm, k]);
    anim_head_hand_to_bench(g, c, o + 2.0);
    anim_hands_ask(g, c, &mut |g: &mut Game| ask_nv_pose(g, c, g_2));
}
/// a goggle nod's tip of the HEAD joint on `pose` (+ = chin down)   (character.ts nvTip)
pub fn anim_nv_tip(g: &mut crate::game::Game, c: CharId, pose: &mut crate::anim_skeleton::Pose, nod: f64, g_2: Option<Quat>) {
    let gq = match g_2 { Some(q) => q, None => anim_nv_head(g, c, pose) };
    let (hd, rest_inv) = (g.ch[c].anim.nodes.head, g.ch[c].anim.head_rest_inv);
    let z = v3::quat::rotate(v3::quat::mul(gq, rest_inv), [0.0, 0.0, 1.0]);
    let l = m::hyp2(z[0], z[2]);
    let ax: V3 = if l > 1e-4 { [z[2] / l, 0.0, -z[0] / l] } else { v3::quat::rotate(v3::quat::mul(gq, rest_inv), [1.0, 0.0, 0.0]) };
    let turned = v3::quat::normalize(v3::quat::mul(crate::anim_skeleton::pose_rot(pose, hd as f64), v3::quat::axis_angle(v3::quat::rotate(v3::quat::conj(gq), ax), nod)));
    crate::anim_skeleton::pose_set_rot(pose, hd as f64, turned);
}
/// a yaw and a pitch of the twists as one turn   (character.ts nvHead twq)
fn twq(y: f64, p: f64) -> Quat {
    let mut q = v3::quat::yaw(y);
    if p != 0.0 { q = v3::quat::mul(v3::quat::axis_angle(v3::quat::rotate(q, [1.0, 0.0, 0.0]), p), q); }
    q
}
/// the head joint's rotation on `pose` as it will be DRAWN   (character.ts nvHead)
pub fn anim_nv_head(g: &mut crate::game::Game, c: CharId, pose: &crate::anim_skeleton::Pose) -> Quat {
    let rig = g.ch[c].anim.rig;
    crate::anim_skeleton::pose_fk_run(&mut g.ch[c].anim.fk, pose, &g.tab.rig[rig]);
    let a = &g.ch[c].anim;
    let tw = a.nv_tw;
    v3::quat::mul(twq(tw[2], tw[3]), v3::quat::mul(twq(tw[0], tw[1]), a.fk.rot[a.nodes.head]))
}

/// the idle tells   (character.ts tellLayer)
pub fn anim_tell_layer(g: &mut crate::game::Game, c: CharId, t: f64) {
    let slot = slot_pa(g, c);
    anim_tell_layer_ask(g, c, slot, t);
}
/// (character.ts tellLayerAsk)
pub fn anim_tell_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64) {
    let a = &g.ch[c].anim;
    let (f, gl, kind) = (a.hand_fix[0], a.gaze_local, a.tell_k);
    let nums = [t, if kind == TellKind::Watch { 0.0 } else { 1.0 }, tell_dur_of(kind), gl[0], gl[1], gl[2], f[0], f[1], f[2], f[3]];
    let shown = a.tell_shown;
    args_put(0, &nums);
    args_put(COLS, &[shown]);
    anim_hands_ask(g, c, &mut |g: &mut Game| ask_tell(g, c, g_2));
    let mut back = [0.0; 1];
    args_read(COLS, &mut back);
    g.ch[c].anim.tell_shown = back[0];
}

/// the held gait for a save file; nothing to say outside the mode   (character.ts heldGaitSnap)
pub fn anim_held_gait_snap(g: &mut crate::game::Game, c: CharId) -> Option<Vec<f64>> {
    let a = &g.ch[c].anim;
    if a.cr_t > 0.0 || a.cr_w > 0.0 { Some(vec![a.cr_t, a.cr_w, a.cr_pace, if a.creep { 1.0 } else { 0.0 }]) } else { None }
}
/// the held gait back from a save file; anything else is ignored   (character.ts heldGaitBack)
pub fn anim_held_gait_back(g: &mut crate::game::Game, c: CharId, a: Option<&[f64]>) {
    let Some(a) = a else { return };
    if a.len() != 4 || !a.iter().all(|x| x.is_finite()) { return; }
    let an = &mut g.ch[c].anim;
    an.cr_t = v3::clamp(a[0], 0.0, 1.0);
    an.cr_w = v3::clamp(a[1], 0.0, 1.0);
    an.cr_pace = m::max(0.0, a[2]);
    an.creep = a[3] == 1.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// stanceOf's arithmetic against the bits the TypeScript prints under EXACT
    #[test]
    fn stance_of_bits() {
        let n = 37;
        let mut ty = vec![vec![0.0; n], vec![0.0; n]];
        let mut tz = vec![vec![0.0; n], vec![0.0; n]];
        for i in 0..n {
            let p = i as f64 / n as f64;
            ty[0][i] = 0.1 + 0.08 * m::sin(2.0 * m::PI * p + 0.3) + 0.01 * m::sin(4.0 * m::PI * p + 1.0);
            tz[0][i] = -0.2 * m::cos(2.0 * m::PI * p + 0.3);
            ty[1][i] = 0.1 + 0.08 * m::sin(2.0 * m::PI * (p + 0.52) + 0.3) + 0.01 * m::sin(4.0 * m::PI * p + 2.0);
            tz[1][i] = -0.2 * m::cos(2.0 * m::PI * (p + 0.52) + 0.3);
        }
        let s = stance_of(&ty, &tz).unwrap();
        assert_eq!(s.c.to_bits(), 0x3fcacf914c1bacf8);
        assert_eq!(s.c_l.to_bits(), 0x3fe759f22983759f);
        assert_eq!(s.c_r.to_bits(), 0x3fc83759f2298370);
    }

    /// a foot that never qualifies is None
    #[test]
    fn stance_of_none() {
        let ty = vec![vec![0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0]];
        let tz = vec![vec![1.0, 1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0, 1.0]];
        assert!(stance_of(&ty, &tz).is_none());
    }

    /// the columns out are the columns in, every slot its own number
    #[test]
    fn columns_round_trip() {
        let mut a = Anim::blank();
        let mut life = [0.0; 40];
        for i in 0..40 { life[i] = i as f64 + 1.5; }
        life_cols_put(&mut a, &life);
        assert_eq!(life_cols_get(&a), life);
        let mut wall = [0.0; 13];
        for i in 0..13 { wall[i] = i as f64 + 1.0; }
        wall[1] = 1.0; wall[2] = 1.0; wall[11] = 1.0;
        wall_cols_put(&mut a, &wall);
        assert_eq!(wall_cols_get(&a), wall);
        wall[2] = 0.0;
        wall_cols_put(&mut a, &wall);
        assert!(a.wtk.is_none());
        let mut kick = [0.0; 15];
        for i in 0..15 { kick[i] = i as f64 + 1.0; }
        kick[0] = 1.0;
        kick_cols_put(&mut a, &kick);
        assert_eq!(kick_cols_get(&a), kick);
        kick[0] = 0.0;
        kick_cols_put(&mut a, &kick);
        assert_eq!(kick_cols_get(&a), [0.0; 15]);
    }
}
