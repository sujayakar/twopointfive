use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// a joint of his body by name, if this body has it (`c.bones.x`)
fn bone(g: &Game, c: CharId, k: crate::character::Bone) -> Option<V3> {
    g.ch[c].bones.get(k).copied()
}
/// `c.bones.handR ?? c.muzzle`
fn hand_or_muzzle(g: &Game, c: CharId) -> V3 {
    match bone(g, c, crate::character::Bone::HandR) {
        Some(h) => h,
        None => g.ch[c].muzzle,
    }
}
/// a direction given in the back's frame, in the frame's space (the `inC` closure of slungBase and bigBackOf)
fn in_c(cf: Frame, d: V3) -> V3 {
    v3::mad(v3::mad(v3::scale(cf.x, d[0]), cf.y, d[1]), cf.z, d[2])
}
/// the identity frame longHold and torchHold apply a group's adj to
fn unit_frame() -> Frame {
    Frame { o: [0.0, 0.0, 0.0], x: [1.0, 0.0, 0.0], y: [0.0, 1.0, 0.0], z: [0.0, 0.0, 1.0] }
}

/// The rifle's base place in the back's frame, slung behind or hung in front.
fn slung_base(g: &crate::game::Game, c_2: CharId, p: crate::rig_props::RigSc20kSlung, front: bool, rest: Option<f64>) -> crate::rig_props::SlungBase {
    let (b_hips, b_thigh_l, b_thigh_r, b_ua_l, b_ua_r, b_spine2) = (bone(g, c_2, crate::character::Bone::Hips), bone(g, c_2, crate::character::Bone::ThighL), bone(g, c_2, crate::character::Bone::ThighR), bone(g, c_2, crate::character::Bone::UpperArmL), bone(g, c_2, crate::character::Bone::UpperArmR), bone(g, c_2, crate::character::Bone::Spine2));
    let joints = match (b_hips, b_thigh_l, b_thigh_r, b_ua_l, b_ua_r, b_spine2) {
        (Some(_), Some(tl), Some(tr), Some(al), Some(ar), Some(_)) => Some((tl, tr, al, ar)),
        _ => None,
    };
    let pos = g.ch[c_2].pos;
    let hips = match b_hips { Some(h) => h, None => v3::add(pos, [0.0, 0.95, 0.0]) };
    let top = match joints { Some((_, _, al, ar)) => v3::lerp3(al, ar, 0.5), None => v3::add(pos, [0.0, 1.4, 0.0]) };
    let mid = match b_spine2 { Some(s) => s, None => v3::lerp3(hips, top, 0.5) };
    let mut up = v3::sub(top, hips);
    up = if v3::len(up) > 1e-3 { v3::normalize(up) } else { [0.0, 1.0, 0.0] };
    let fw = crate::character::character_forward(g, c_2);
    let mut left: V3 = [fw[2], 0.0, -fw[0]];
    if let Some((tl, tr, al, ar)) = joints {
        left = v3::lerp3(v3::normalize(v3::sub(tl, tr)), v3::normalize(v3::sub(al, ar)), m::max(0.0, m::min(1.0, p.follow)));
    }
    left = v3::mad(left, up, -v3::dot(left, up));
    left = if v3::len(left) > 1e-3 { v3::normalize(left) } else { [fw[2], 0.0, -fw[0]] };
    let cf = Frame { o: mid, x: left, y: up, z: v3::cross(left, up) };
    let off = if !front && g.tab.rig[g.ch[c_2].rig].cast.is_some() { [p.off[0], p.off[1], p.off[2] - SLUNG_CAST_OUT] } else { p.off };
    let mut o = v3::add(mid, in_c(cf, off));
    let mut z = v3::normalize(in_c(cf, p.bore));
    let mut want = 0.0;
    if front {
        let a = match rest {
            Some(r) => r,
            None => { want = sling_swing(g, c_2, cf, o, z); want }
        };
        if a > 0.0 {
            let f = sling_rest();
            let piv = v3::mad(o, z, -f.pivot);
            z = rot_axis(z, cf.x, -m::min(f.max, a));
            o = v3::mad(piv, z, f.pivot);
        }
    }
    SlungBase { o, z, cf, want }
}

/// How far a rifle hanging in front must swing forward to lie over both thighs as posed.
fn sling_swing(g: &crate::game::Game, c_2: CharId, cf: crate::rig_props::Frame, o: V3, z: V3) -> f64 {
    let (thigh_l, thigh_r, shin_l, shin_r) = match (bone(g, c_2, crate::character::Bone::ThighL), bone(g, c_2, crate::character::Bone::ThighR), bone(g, c_2, crate::character::Bone::ShinL), bone(g, c_2, crate::character::Bone::ShinR)) {
        (Some(a), Some(b), Some(c), Some(d)) => (a, b, c, d),
        _ => return 0.0,
    };
    let f = sling_rest();
    let piv = v3::mad(o, z, -f.pivot);
    let zx = v3::dot(z, cf.x);
    let zs = m::sqrt(m::max(1e-6, 1.0 - zx * zx));
    let a0 = swing_ang(v3::dot(z, cf.z), v3::dot(z, cf.y));
    let mut a = 0.0;
    for (j0, j1) in [(thigh_l, shin_l), (thigh_r, shin_r)] {
        let mut i = 1.0;
        while i <= f.n {
            let t = i / f.n;
            i += 1.0;
            let v = v3::sub(v3::lerp3(j0, j1, t), piv);
            let l = v3::dot(v, cf.x);
            let fw = v3::dot(v, cf.z);
            let u = v3::dot(v, cf.y);
            let r = m::hyp2(fw, u);
            if r < 1e-3 { continue; }
            let k = zs / (if m::abs(zx) > 1e-3 { zx } else { 1e-3 });
            let r0 = m::max(0.0, m::min((l - f.clear) * k, (l + f.clear) * k));
            let r1 = m::min(f.len * zs, m::max((l - f.clear) * k, (l + f.clear) * k));
            let d = if r1 < r0 { m::INF } else if r < r0 { r0 - r } else if r > r1 { r - r1 } else { 0.0 };
            let w = m::max(0.0, m::min(1.0, 1.0 - (d - f.clear) / f.soft));
            if w <= 0.0 { continue; }
            a = m::max(a, w * (swing_ang(fw, u) + m::asin(m::min(1.0, f.clear / r)) - a0));
        }
    }
    m::min(f.max, a)
}
/// the `ang` closure of slingSwing
fn swing_ang(f: f64, u: f64) -> f64 {
    m::atan2(f, -u)
}

/// The swing a front-slung rifle wants on this pose.
pub(in crate::outfit) fn sling_rest_want(g: &crate::game::Game, c_2: CharId) -> f64 {
    slung_base(g, c_2, sling_front(), true, None).want
}

/// The rifle's group frame, in the hands or slung.
fn sc20k_frame(g: &mut crate::game::Game, c_2: CharId, s: crate::rig_props::RigSc20k, where_: crate::rig_props::Sc20kFrameWhere, front: Option<bool>, rest: Option<f64>) -> crate::rig_props::Sc20kFrame {
    let front = front.unwrap_or(false);
    if where_ == Sc20kFrameWhere::Hand {
        let hand = hand_or_muzzle(g, c_2);
        let fr = pistol_frame(g, c_2, hand, PistolFrameP { adj: s.adj }, None);
        return Sc20kFrame { g: fr.g };
    }
    if !front {
        if let Some(side) = pack_side(g, c_2, PackSideWhat::Rifle, None) {
            return Sc20kFrame { g: side };
        }
    }
    let p = if front { sling_front() } else { s.slung };
    let SlungBase { o, z, cf, .. } = slung_base(g, c_2, p, front, rest);
    let out = if front { 1.0 } else { -1.0 };
    let mut x = v3::mad(v3::scale(cf.z, out), z, -out * v3::dot(cf.z, z));
    if v3::len(x) < 1e-3 { x = cf.x; }
    x = v3::normalize(x);
    x = rot_axis(x, z, p.roll * m::PI / 180.0);
    let base = Frame { o, x, y: v3::cross(z, x), z };
    Sc20kFrame { g: apply_adj(base, p.adj) }
}

/// The can's place as a bare barrel: the same length to the same muzzle, a finger thick.
fn bare_barrel(can: crate::rig_props::Seg) -> crate::rig_props::Seg {
    Seg { thick: BARE_BARREL, tall: BARE_BARREL, ..can }
}

/// The rifle's boxes (or its model's), off the right hand or slung, stamped with this man. They are not only a picture: they reach the world's dynamics, which a laser's reach and the cursor's ray read.
pub(super) fn sc20k_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<AnyBoxId>, s: Option<crate::rig_props::RigSc20k>, where_: Option<crate::rig_props::Sc20kBoxesWhere>, lens: Option<V3>, m: Option<crate::rig_props::RigM4>, kit: Option<crate::rig_props::KitDraw>, fit: Option<bool>, front: Option<bool>, rest: Option<f64>, black: Option<bool>) {
    let s: RigSc20k = s.unwrap_or_else(|| crate::rig_props::rig().sc20k);
    let where_ = where_.unwrap_or(Sc20kBoxesWhere::Hand);
    let lens = lens.unwrap_or([0.0, 0.0, 0.0]);
    let (fit, front, black) = (fit.unwrap_or(false), front.unwrap_or(false), black.unwrap_or(false));
    let slung = where_ == Sc20kBoxesWhere::Slung;
    stamp_owner(g, c_2);
    let fr = sc20k_frame(g, c_2, s, if slung { Sc20kFrameWhere::Slung } else { Sc20kFrameWhere::Hand }, Some(front), rest);
    let mut gf = fr.g;
    let hand = bone(g, c_2, crate::character::Bone::HandR);
    match (kit, slung, hand) {
        (Some(kd), false, Some(hand)) => {
            gf = kit_gun_frame(g, c_2, hand, KitGunFrameP { adj: s.adj }, gf, kd.roll.unwrap_or(0.0));
            if fit { gf = fit_frame(gf, gripfit().long); }
            let rifle = kd.rifle.unwrap_or(0.0);
            if rifle > 0.0 { gf = mix_frame(gf, sc20k_frame(g, c_2, s, Sc20kFrameWhere::Slung, Some(kd.front.unwrap_or(false)), kd.rest).g, rifle); }
        }
        _ => if fit && !slung { gf = fit_frame(gf, gripfit().long); },
    }
    let no_mag = kit.map_or(false, |kd| kd.no_mag.unwrap_or(false));
    if let Some(look) = long_look_of(g, c_2, !slung) {
        if slung { long_model_boxes(g, out, &look.l, gf, no_glow(), black, false, 0.0); } else { model_in_hands(g, c_2, look.id, &look.l, out, gf, lens, black, no_mag); }
        return;
    }
    let col = crate::rig_props::col();
    let is_m4 = m.is_some();
    let body = if is_m4 { col.anodised } else { col.polymer };
    let furn = if is_m4 { if black { col.polymer } else { col.fde } } else { col.polymer };
    let metal = if is_m4 { col.anodised } else { col.dark_metal };
    let (hull, butt, mag, fore, rail, optic) = match m {
        Some(mm) => (mm.hull, mm.butt, mm.mag, mm.fore, mm.rail, mm.optic),
        None => (s.hull, s.butt, s.mag, s.fore, s.rail, s.optic),
    };
    let bare = g.ch[c_2].gun_look.as_ref().and_then(|l| l.bare_long).unwrap_or(false);
    sc20k_part(g, out, gf, hull, body, None);
    sc20k_part(g, out, gf, butt, furn, None);
    if !no_mag { sc20k_part(g, out, gf, mag, metal, None); }
    sc20k_part(g, out, gf, s.grip, furn, None);
    sc20k_part(g, out, gf, fore, furn, None);
    sc20k_part(g, out, gf, if bare { bare_barrel(s.suppressor) } else { s.suppressor }, metal, None);
    sc20k_part(g, out, gf, rail, if is_m4 { col.anodised } else { col.dark_metal }, None);
    sc20k_part(g, out, gf, optic, metal, None);
    sc20k_part(g, out, gf, s.foregrip, furn, None);
    sc20k_part(g, out, gf, s.light, col.torch, None);
    sc20k_part(g, out, gf, s.lens, col.lens_body, Some(lens));
    if let Some(mm) = m {
        sc20k_part(g, out, gf, mm.tube, metal, None);
        sc20k_part(g, out, gf, mm.peq, if black { col.polymer } else { col.fde }, None);
        sc20k_part(g, out, gf, mm.gl, metal, None);
    }
}
/// One part of the rifle: seg() at depth 2 with the rifle's tag.
fn sc20k_part(g: &mut Game, out: &mut Vec<AnyBoxId>, gf: Frame, part: Seg, albedo: V3, emissive: Option<V3>) {
    seg(g, out, gf, part, albedo, emissive, Some(SC20K_TAG));
}
/// `curOwner = c.id & 0xff`: the character id the boxes being emitted are stamped with
fn stamp_owner(g: &mut Game, c: CharId) {
    g.m.rig_props.cur_owner = (js::to_i32(g.ch[c].id) & 0xff) as f64;
}

/// The 2010s carbine's parts (GEAR.on is settled on).
pub(super) fn rifle_m4(rig: Option<crate::rig_props::Rig>) -> Option<crate::rig_props::RigM4> {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    Some(rig.m4)
}

/// The SC-20K's support-hand grip for the animator.
pub fn sc20k_grip(rig: Option<crate::rig_props::Rig>, fit: Option<bool>) -> crate::character::AnimLongGrip {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let gl = rig.sc20k.grip_l;
    let gr = long_grip_of(rig.sc20k.adj, LongGripOfG { off: gl.off, fingers: gl.fingers, back: gl.back });
    if !fit.unwrap_or(false) {
        return gr;
    }
    let fits = gripfit();
    let d = fits.long;
    let f = fits.fore;
    crate::character::AnimLongGrip {
        wrist: v3::mad(v3::mad(v3::mad(v3::add(gr.wrist, d), gr.fingers, -f.back), gr.back, -f.lift), v3::normalize(v3::cross(gr.fingers, gr.back)), -f.down),
        fingers: gr.fingers,
        back: gr.back,
    }
}

/// What the hand pass is told of a long gun in his hands.
pub fn long_hold(weapon: crate::rig_props::LongHoldWeapon, rig: Option<crate::rig_props::Rig>) -> crate::inhand::InHandWord {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let fr = apply_adj(unit_frame(), if weapon == LongHoldWeapon::Sc20k { rig.sc20k.adj } else { rig.carbine.adj });
    let d = gripfit().long;
    crate::inhand::InHandWord::Long(crate::inhand::InHandWordLong {
        by: v3::scale(v3::mad(v3::mad(v3::scale(fr.x, d[0]), fr.y, d[1]), fr.z, d[2]), -1.0),
    })
}

/// …and of a torch: its beam, the point of its body the fist goes round, the way its body lies.
pub fn torch_hold(beam: V3, rig: Option<crate::rig_props::Rig>) -> crate::inhand::InHandWord {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let fr = apply_adj(unit_frame(), rig.torch.adj);
    let t = gripfit().torch;
    crate::inhand::InHandWord::Torch(crate::inhand::InHandWordTorch { beam, at: fr.o, axis: fr.z, fwd: t.fwd, palm: t.palm })
}

/// How far ahead of the wrist along the bore its muzzle is.
pub fn sc20k_muzzle_ahead(rig: Option<crate::rig_props::Rig>) -> f64 {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    muzzle_ahead_of(rig.sc20k.adj, rig.sc20k.suppressor)
}

/// Its weapon light: at the lens, pointing down the bore.
pub fn sc20k_light_pose(g: &mut crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::Sc20kLightPose {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let s = rig.sc20k;
    let fr = sc20k_frame(g, c_2, s, Sc20kFrameWhere::Hand, None, None).g;
    let look = long_look_of(g, c_2, true);
    let (adj, ahead) = match look.as_ref().and_then(|lk| lk.l.lens.map(|lens| (lens, lk.l.lens_ahead))) {
        Some((lens, lens_ahead)) => (lens.adj.map(|a| Adj { off: a.off, rot: a.rot }), lens_ahead.unwrap_or(s.light_ahead)),
        None => (s.lens.adj, s.light_ahead),
    };
    let l = apply_adj(fr, adj);
    Sc20kLightPose { pos: v3::mad(l.o, l.z, ahead), dir: fr.z }
}

/// Where a ring leaves it: the launcher port under the fore-end's nose.
pub fn sc20k_launcher_muzzle(g: &mut crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> V3 {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let s = rig.sc20k;
    let fr = sc20k_frame(g, c_2, s, Sc20kFrameWhere::Hand, None, None).g;
    let f = apply_adj(fr, s.fore.adj);
    v3::mad(v3::mad(f.o, f.z, s.fore.l1), f.y, -(s.fore.tall + 0.02))
}

/// The carbine's weapon light: at the lens part, pointing down the bore.
fn carbine_light_pose(g: &crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::CarbineLightPose {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let cb = rig.carbine;
    let hand = hand_or_muzzle(g, c_2);
    let fr = carbine_frame(g, c_2, hand, cb).g;
    let l = apply_adj(fr, cb.lens.adj);
    CarbineLightPose { pos: v3::mad(l.o, l.z, cb.light_ahead) }
}

/// The player's rail light: its position and direction are the gun's.
pub fn rig_light_pose(g: &crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::RigLightPose {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let hand = hand_or_muzzle(g, c_2);
    let fr = pistol_frame(g, c_2, hand, PistolFrameP { adj: rig.pistol.adj }, None).g;
    let r = apply_adj(Frame { o: v3::mad(fr.o, fr.y, -rig.pistol.light_under), ..fr }, rig.pistol.rail.adj);
    RigLightPose { pos: v3::mad(r.o, r.z, rig.pistol.light_ahead), dir: r.z }
}

/// A guard's light origin along its beam.
pub fn rig_torch_pos(g: &mut crate::game::Game, c_2: CharId, dir: V3, torch: bool, rig: Option<crate::rig_props::Rig>, weapon: Option<crate::rig_props::RigTorchPosWeapon>) -> V3 {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let weapon = weapon.unwrap_or(RigTorchPosWeapon::Pistol);
    if weapon == RigTorchPosWeapon::Carbine { return carbine_light_pose(g, c_2, Some(rig)).pos; }
    if weapon == RigTorchPosWeapon::Sc20k { return sc20k_light_pose(g, c_2, Some(rig)).pos; }
    let hand = hand_or_muzzle(g, c_2);
    let fr = if torch {
        apply_adj(apply_adj(frame_from(hand, dir, None), rig.torch.adj), rig.torch.lens.adj)
    } else {
        apply_adj(pistol_frame(g, c_2, hand, PistolFrameP { adj: rig.guard_pistol.adj }, None).g, rig.guard_pistol.lens.adj)
    };
    v3::mad(fr.o, fr.z, if torch { rig.torch.light_ahead } else { rig.guard_pistol.light_ahead })
}

/// The player's rail light, parked and off until posed.
pub fn rail_light_def(g: &mut crate::game::Game, owner: f64, intensity: f64) -> LightId {
    g.tab.light.push(rail_light(owner, intensity))
}
fn rail_light(owner: f64, intensity: f64) -> crate::render_lights::RtLight {
    (crate::render_lights::RtLight { name: String::from("pistol_light"), group: String::from("player"), kind: 1.0, pos: [0.0, -10.0, 0.0], dir: [0.0, 0.0, 1.0], color: [0.9, 0.96, 1.0], intensity, range: 18.0, inner_deg: 5.0, outer_deg: 15.0, radius: 0.035, volumetric: 1.0, owner, enabled: false, fixture_box: -1.0, flicker: 0.0, ttl: -1.0, age: 0.0, peak_intensity: intensity, card: true, ..crate::render_lights::RtLight::default() })
}

/// A guard's torch / weapon light, warm and wider.
pub fn torch_light_def(g: &mut crate::game::Game, name: &str, owner: f64, intensity: f64, inner_deg: f64, outer_deg: f64) -> LightId {
    g.tab.light.push(torch_light(name, owner, intensity, inner_deg, outer_deg))
}
fn torch_light(name: &str, owner: f64, intensity: f64, inner_deg: f64, outer_deg: f64) -> crate::render_lights::RtLight {
    (crate::render_lights::RtLight { name: String::from(name), group: String::from("flashlights"), kind: 1.0, pos: [0.0, -10.0, 0.0], dir: [0.0, 0.0, 1.0], color: [1.0, 0.92, 0.78], intensity, range: 16.0, inner_deg, outer_deg, radius: 0.05, volumetric: 1.0, owner, enabled: true, fixture_box: -1.0, flicker: 0.0, ttl: -1.0, age: 0.0, peak_intensity: intensity, card: true, ..crate::render_lights::RtLight::default() })
}

/// The relaxed torch carry's beam, eased from last frame's.
pub fn torch_carry_dir(g: &crate::game::Game, c_2: CharId, prev: V3, dt: Option<f64>, low: Option<crate::rig_props::TorchCarryDirLow>) -> V3 {
    let dt = dt.unwrap_or(1.0 / 60.0);
    let want = torch_carry_want(g, c_2, low.map(|l| -l.tan));
    let rate = match low { Some(l) => l.rate, None => TORCH_CARRY_RATE };
    v3::normalize(v3::lerp3(prev, want, 1.0 - m::exp(-rate * m::max(0.0, dt))))
}

/// Where the relaxed carry is easing to: half his aim heading, half his chest.
pub fn torch_carry_want(g: &crate::game::Game, c_2: CharId, tan: Option<f64>) -> V3 {
    let (aim_yaw, aim_pitch, chest) = (g.ch[c_2].aim_yaw, g.ch[c_2].aim_pitch, g.ch[c_2].chest_dir);
    let f: V3 = [m::sin(aim_yaw), 0.0, m::cos(aim_yaw)];
    let y = match tan { Some(t) => t, None => m::tan(aim_pitch) };
    v3::normalize([f[0] * 0.5 + chest[0] * 0.5, y, f[2] * 0.5 + chest[2] * 0.5])
}

/// How far off his back the big slot's tool rides with his rifle slung under it.
pub fn big_clear_of(g: &crate::game::Game, c_2: CharId, kind: crate::rig_props::BigClearOfKind, rig: Option<crate::rig_props::Rig>) -> f64 {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let p = rig.sc20k.slung;
    let s = slung_base(g, c_2, p, false, None);
    let out = p.off[2] - v3::dot(v3::sub(s.o, s.cf.o), s.cf.z);
    let clear = BIGRIG.clear;
    (match kind { BigClearOfKind::Hammer => clear.hammer, BigClearOfKind::Shotgun => clear.shotgun }) + (if m::abs(out) < 1e-6 { 0.0 } else { out })
}

/// The sizes of the thing the pack-side rule stands at a pack's side.
fn pack_side_dims(what: crate::rig_props::PackSideDimsWhat, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::PackSideDims {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    if what == PackSideDimsWhat::Hammer {
        let h = BIGRIG.hammer;
        return PackSideDims {
            grip: Some(h.head_at),
            thick: h.handle.thick,
            head: Some([h.head.thick, h.head.tall, (h.head.l1 - h.head.l0) / 2.0]),
            rear: None,
            top: None,
            len: h.head_at - h.handle.l0,
        };
    }
    if what == PackSideDimsWhat::Shotgun {
        let cb = rig.carbine;
        return PackSideDims {
            grip: None,
            rear: Some(-cb.stock_top.l0),
            thick: m::max(cb.receiver.thick, cb.handguard.thick),
            head: None,
            top: Some(cb.receiver.tall + cb.receiver.adj.map_or(0.0, |a| a.off[1])),
            len: cb.muzzle.l1 - cb.stock_top.l0,
        };
    }
    let s = rig.sc20k;
    let m4 = rifle_m4(Some(rig));
    let o = match m4 { Some(mm) => mm.optic, None => s.optic };
    let butt_l0 = match m4 { Some(mm) => mm.butt.l0, None => s.butt.l0 };
    PackSideDims {
        grip: None,
        rear: Some(-butt_l0),
        thick: m::max(s.hull.thick, -s.light.adj.map_or(0.0, |a| a.off[0]) + s.lens.thick),
        head: None,
        top: Some(o.adj.map_or(0.0, |a| a.off[1]) + o.tall),
        len: s.suppressor.l1 - butt_l0,
    }
}

/// The pack-side place in the bind space of the mesh he wears, pitched to his back as posed.
pub fn pack_side_bind(g: &mut crate::game::Game, c_2: CharId, what: crate::rig_props::PackSideBindWhat, rig: Option<crate::rig_props::Rig>) -> Option<crate::packside::SidePlace> {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let (ch, parts) = (g.ch[c_2].rig, g.ch[c_2].parts);
    let e = crate::packside::pack_of(g, ch, parts)?;
    let (dims_what, side_what) = match what {
        PackSideBindWhat::Hammer => (PackSideDimsWhat::Hammer, crate::packside::SideBindWhat::Hammer),
        PackSideBindWhat::Shotgun => (PackSideDimsWhat::Shotgun, crate::packside::SideBindWhat::Shotgun),
        PackSideBindWhat::Rifle => (PackSideDimsWhat::Rifle, crate::packside::SideBindWhat::Rifle),
    };
    let d = pack_side_dims(dims_what, Some(rig));
    let dims = crate::packside::SideBindDims { grip: d.grip, thick: d.thick, head: d.head, rear: d.rear, top: d.top, len: Some(d.len) };
    let p = crate::packside::side_bind(g, e, side_what, dims);
    let th = crate::packside::side_pitch(g, p, c_2);
    Some(crate::packside::side_pitched(p, th))
}

/// …carried into the world by the chest joint's skin matrix.
fn pack_side(g: &mut crate::game::Game, c_2: CharId, what: crate::rig_props::PackSideWhat, rig: Option<crate::rig_props::Rig>) -> Option<crate::rig_props::Frame> {
    let bind_what = match what {
        PackSideWhat::Hammer => PackSideBindWhat::Hammer,
        PackSideWhat::Shotgun => PackSideBindWhat::Shotgun,
        PackSideWhat::Rifle => PackSideBindWhat::Rifle,
    };
    let p = pack_side_bind(g, c_2, bind_what, rig)?;
    let ch = g.ch[c_2].rig;
    let j = crate::packside::chest_joint(g, ch);
    if j < 0.0 { return None; }
    let w = crate::packside::side_skinned(p, &g.ch[c_2].skel.joint_data, j * 16.0);
    Some(Frame { o: w.o, x: w.x, y: w.y, z: w.z })
}

/// Where a thing across the back is, off six joints in whatever one space they are given in.
pub fn big_back_of(hips: V3, mid: V3, th_l: V3, th_r: V3, ua_l: V3, ua_r: V3, p: crate::rig_props::RigSc20kSlung, clear: Option<f64>) -> crate::rig_props::BigBackOf {
    let clear = clear.unwrap_or(0.0);
    let mut up = v3::sub(v3::lerp3(ua_l, ua_r, 0.5), hips);
    up = if v3::len(up) > 1e-3 { v3::normalize(up) } else { [0.0, 1.0, 0.0] };
    let mut left = v3::lerp3(v3::normalize(v3::sub(th_l, th_r)), v3::normalize(v3::sub(ua_l, ua_r)), m::max(0.0, m::min(1.0, p.follow)));
    left = v3::mad(left, up, -v3::dot(left, up));
    left = if v3::len(left) > 1e-3 { v3::normalize(left) } else { v3::normalize(v3::sub(ua_l, ua_r)) };
    let cf = Frame { o: mid, x: left, y: up, z: v3::cross(left, up) };
    BigBackOf { o: v3::add(mid, in_c(cf, [p.off[0], p.off[1], p.off[2] - clear])), z: v3::normalize(in_c(cf, p.bore)), cf }
}

/// The big slot's tool on a man: on his back, in his hands, or carried between by `k`.
pub fn big_boxes(g: &mut crate::game::Game, c_2: CharId, kind: crate::rig_props::BigBoxesKind, k: f64, out: &mut Vec<AnyBoxId>, rig: Option<crate::rig_props::Rig>, clear: Option<bool>) {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let clear = clear.unwrap_or(false);
    let (hips, spine2, th_l, th_r, ua_l, ua_r) = match (bone(g, c_2, crate::character::Bone::Hips), bone(g, c_2, crate::character::Bone::Spine2), bone(g, c_2, crate::character::Bone::ThighL), bone(g, c_2, crate::character::Bone::ThighR), bone(g, c_2, crate::character::Bone::UpperArmL), bone(g, c_2, crate::character::Bone::UpperArmR)) {
        (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) => (a, b, c, d, e, f),
        _ => return,
    };
    stamp_owner(g, c_2);
    let side_what = match kind { BigBoxesKind::Hammer => PackSideWhat::Hammer, BigBoxesKind::Shotgun => PackSideWhat::Shotgun };
    let on_back = match pack_side(g, c_2, side_what, Some(rig)) {
        Some(side) => side,
        None => {
            let table = BIGRIG;
            let p = match kind { BigBoxesKind::Hammer => table.hammer.back, BigBoxesKind::Shotgun => table.shotgun.back };
            let clear_kind = match kind { BigBoxesKind::Hammer => BigClearOfKind::Hammer, BigBoxesKind::Shotgun => BigClearOfKind::Shotgun };
            let by = if clear { big_clear_of(g, c_2, clear_kind, Some(rig)) } else { 0.0 };
            let bk = big_back_of(hips, spine2, th_l, th_r, ua_l, ua_r, p, Some(by));
            let x = v3::mad(v3::scale(bk.cf.z, -1.0), bk.z, v3::dot(bk.cf.z, bk.z));
            let x = rot_axis(if v3::len(x) < 1e-3 { bk.cf.x } else { v3::normalize(x) }, bk.z, p.roll * m::PI / 180.0);
            apply_adj(Frame { o: bk.o, x, y: v3::cross(bk.z, x), z: bk.z }, p.adj)
        }
    };
    let mut gf = on_back;
    if k > 0.0 {
        if let Some(hand) = bone(g, c_2, crate::character::Bone::HandR) {
            if kind == BigBoxesKind::Hammer {
                let h = crate::character::character_hand_frame_r(g, c_2, None);
                let grip = BIGRIG.hammer.grip;
                gf = mix_frame(on_back, Frame { o: v3::mad(v3::mad(v3::mad(hand, h.fingers, grip.fwd), h.back, -grip.palm), h.thumb, grip.side), x: v3::cross(h.back, h.thumb), y: h.back, z: h.thumb }, k);
            } else {
                gf = mix_frame(on_back, carbine_frame(g, c_2, hand, rig.carbine).g, k);
            }
        }
    }
    if kind == BigBoxesKind::Shotgun {
        carbine_boxes(g, c_2, out, rig.carbine, [0.0, 0.0, 0.0], Some(gf), Some(crate::weapons::GunModelId::M870));
        return;
    }
    let (table, col) = (BIGRIG, crate::rig_props::col());
    seg_at(g, out, gf, table.hammer.handle, None, col.haft, no_glow(), Some(BIG_TAG));
    seg_at(g, out, gf, table.hammer.head, None, col.sledge, no_glow(), Some(BIG_TAG));
}

/// A man's own long gun let go of for the big slot's tool: on its sling down his front, dark.
pub fn hung_boxes(g: &mut crate::game::Game, c_2: CharId, weapon: crate::rig_props::HungBoxesWeapon, out: &mut Vec<AnyBoxId>, rig: Option<crate::rig_props::Rig>, black: Option<bool>) {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let black = black.unwrap_or(false);
    if weapon == HungBoxesWeapon::Sc20k {
        let m4 = rifle_m4(Some(rig));
        sc20k_boxes(g, c_2, out, Some(rig.sc20k), Some(Sc20kBoxesWhere::Slung), Some([0.0, 0.0, 0.0]), m4, None, Some(false), Some(true), None, Some(black));
        return;
    }
    stamp_owner(g, c_2);
    let at = sc20k_frame(g, c_2, rig.sc20k, Sc20kFrameWhere::Slung, Some(true), None).g;
    carbine_boxes(g, c_2, out, rig.carbine, [0.0, 0.0, 0.0], Some(at), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: V3, b: V3) -> bool { v3::len(v3::sub(a, b)) < 1e-9 }
    const P: RigSc20kSlung = RigSc20kSlung { off: [-0.055, 0.01, -0.19], bore: [-0.42, 0.91, 0.0], roll: 90.0, follow: 0.5, adj: None };
    const HIPS: V3 = [0.013, 0.951, -0.027];
    const MID: V3 = [0.021, 1.203, -0.041];
    fn back(p: RigSc20kSlung, clear: Option<f64>) -> BigBackOf {
        big_back_of(HIPS, MID, [0.11, 0.93, 0.02], [-0.09, 0.94, -0.05], [0.19, 1.41, 0.03], [-0.17, 1.39, -0.08], p, clear)
    }

    /// A FAILURE MEANS: the big tool on his back is skewed off his chest, or points otherwise than its bore says.
    #[test]
    fn a_slung_tool_sits_in_a_frame_of_his_chest_along_its_bore() {
        let r = back(P, None);
        for a in [r.cf.x, r.cf.y, r.cf.z] { assert!((v3::len(a) - 1.0).abs() < 1e-9); }
        assert!(v3::dot(r.cf.x, r.cf.y).abs() < 1e-9 && v3::dot(r.cf.y, r.cf.z).abs() < 1e-9);
        assert!(r.cf.y[1] > 0.9, "the chest's up runs from his hips to his shoulders");
        assert!(near(r.z, v3::normalize(in_c(r.cf, P.bore))));
        assert!(near(r.o, v3::add(MID, in_c(r.cf, P.off))));
    }

    /// A FAILURE MEANS: a thing on his back goes through what he wears: clear moves it straight off his back.
    #[test]
    fn clear_moves_a_slung_tool_off_his_back_by_that_much() {
        let (none, clear) = (back(P, None), back(P, Some(0.11)));
        assert!(near(clear.o, v3::add(none.o, v3::scale(none.cf.z, -0.11))));
    }

    /// A FAILURE MEANS: the tool does not turn with him as its row says: follow goes from his hips' way to his shoulders' way.
    #[test]
    fn a_slung_tool_follows_his_shoulders_as_far_as_its_row_says() {
        let at = |follow: f64| back(RigSc20kSlung { follow, ..P }, None).cf.x;
        let shoulders = v3::normalize(v3::sub([0.19, 1.41, 0.03], [-0.17, 1.39, -0.08]));
        assert!(v3::dot(at(1.0), shoulders) > v3::dot(at(0.0), shoulders));
        assert_eq!(at(2.0), at(1.0));
    }
}
