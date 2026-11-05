//! src/game/rigProps.ts lines 839 to 1099
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

/// The rifle's base place in the back's frame, slung behind or hung in front. (rigProps.ts slungBase)
pub fn slung_base(g: &mut crate::game::Game, c_2: CharId, p: crate::rig_props::RigSc20kSlung, front: bool, rest: Option<f64>) -> crate::rig_props::SlungBase {
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

/// How far a rifle hanging in front must swing forward to lie over both thighs as posed. (rigProps.ts slingSwing)
pub fn sling_swing(g: &mut crate::game::Game, c_2: CharId, cf: crate::rig_props::Frame, o: V3, z: V3) -> f64 {
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

/// The swing a front-slung rifle wants on this pose. (rigProps.ts slingRestWant)
pub fn sling_rest_want(g: &mut crate::game::Game, c_2: CharId) -> f64 {
    slung_base(g, c_2, sling_front(), true, None).want
}

/// The rifle's group frame, in the hands or slung. (rigProps.ts sc20kFrame)
pub fn sc20k_frame(g: &mut crate::game::Game, c_2: CharId, s: crate::rig_props::RigSc20k, where_: crate::rig_props::Sc20kFrameWhere, front: Option<bool>, rest: Option<f64>) -> crate::rig_props::Sc20kFrame {
    let front = front.unwrap_or(false);
    if where_ == Sc20kFrameWhere::Hand {
        let hand = hand_or_muzzle(g, c_2);
        let fr = pistol_frame(g, c_2, hand, PistolFrameP { adj: s.adj }, None);
        return Sc20kFrame { parent: fr.hand_f, g: fr.g };
    }
    if !front {
        if let Some(side) = pack_side(g, c_2, PackSideWhat::Rifle, None) {
            return Sc20kFrame { parent: side, g: side };
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
    Sc20kFrame { parent: base, g: apply_adj(base, p.adj) }
}

/// The can's place as a bare barrel: the same length to the same muzzle, a finger thick. (rigProps.ts bareBarrel)
pub fn bare_barrel(can: crate::rig_props::Seg) -> crate::rig_props::Seg {
    Seg { thick: BARE_BARREL, tall: BARE_BARREL, ..can }
}

/// The rifle's boxes (or its model's), off the right hand or slung, stamped with this man. They are not only a picture: they reach the world's dynamics, which a laser's reach and the cursor's ray read. (rigProps.ts sc20kBoxes)
pub fn sc20k_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, s: Option<&mut crate::rig_props::RigSc20k>, where_: Option<crate::rig_props::Sc20kBoxesWhere>, lens: Option<V3>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, m: Option<crate::rig_props::RigM4>, kit: Option<crate::rig_props::KitDraw>, fit: Option<bool>, front: Option<bool>, rest: Option<f64>, black: Option<bool>) {
    let s: RigSc20k = match s { Some(s) => *s, None => crate::rig_props::rig().sc20k };
    let where_ = where_.unwrap_or(Sc20kBoxesWhere::Hand);
    let lens = lens.unwrap_or([0.0, 0.0, 0.0]);
    let mut meta = meta;
    let (fit, front, black) = (fit.unwrap_or(false), front.unwrap_or(false), black.unwrap_or(false));
    let slung = where_ == Sc20kBoxesWhere::Slung;
    stamp_owner(g, c_2);
    let fr = sc20k_frame(g, c_2, s, if slung { Sc20kFrameWhere::Slung } else { Sc20kFrameWhere::Hand }, Some(front), rest);
    let parent = fr.parent;
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
    if let Some(sink) = meta.as_deref_mut() {
        let label = match (is_m4, slung) {
            (true, true) => "M4 (slung across the back)",
            (true, false) => "M4 (group + weapon light)",
            (false, true) => "SC-20K (slung across the back)",
            (false, false) => "SC-20K (group + weapon light)",
        };
        sink.push(RigNodeMeta { path: String::from(if slung { "sc20k.slung" } else { "sc20k" }), label: String::from(label), depth: 1.0, parent, frame: gf, box_start: out.len() as f64, box_count: if is_m4 { M4_BOXES } else { 11.0 } });
    }
    let (hull, butt, mag, fore, rail, optic) = match m {
        Some(mm) => (mm.hull, mm.butt, mm.mag, mm.fore, mm.rail, mm.optic),
        None => (s.hull, s.butt, s.mag, s.fore, s.rail, s.optic),
    };
    let bare = g.ch[c_2].gun_look.as_ref().and_then(|l| l.bare_long).unwrap_or(false);
    sc20k_part(g, out, gf, hull, body, None, &mut meta, if is_m4 { "m4.hull" } else { "sc20k.hull" }, if is_m4 { "receiver" } else { "hull (action + stock, polymer)" });
    sc20k_part(g, out, gf, butt, furn, None, &mut meta, if is_m4 { "m4.butt" } else { "sc20k.butt" }, if is_m4 { "stock" } else { "buttpad" });
    if !no_mag || meta.is_some() { sc20k_part(g, out, gf, mag, metal, None, &mut meta, if is_m4 { "m4.mag" } else { "sc20k.mag" }, if is_m4 { "magazine (ahead of the grip)" } else { "magazine (behind the grip)" }); }
    sc20k_part(g, out, gf, s.grip, furn, None, &mut meta, "sc20k.grip", "pistol grip");
    sc20k_part(g, out, gf, fore, furn, None, &mut meta, if is_m4 { "m4.fore" } else { "sc20k.fore" }, if is_m4 { "handguard" } else { "fore-end" });
    sc20k_part(g, out, gf, if bare { bare_barrel(s.suppressor) } else { s.suppressor }, metal, None, &mut meta, "sc20k.suppressor", if is_m4 { "suppressor" } else { "integral suppressor" });
    sc20k_part(g, out, gf, rail, if is_m4 { col.anodised } else { col.dark_metal }, None, &mut meta, if is_m4 { "m4.rail" } else { "sc20k.rail" }, "top rail");
    sc20k_part(g, out, gf, optic, metal, None, &mut meta, if is_m4 { "m4.optic" } else { "sc20k.optic" }, if is_m4 { "sight" } else { "sight block" });
    sc20k_part(g, out, gf, s.foregrip, furn, None, &mut meta, "sc20k.foregrip", "foregrip (vertical post)");
    sc20k_part(g, out, gf, s.light, col.torch, None, &mut meta, "sc20k.light", "weapon light body");
    sc20k_part(g, out, gf, s.lens, col.lens_body, Some(lens), &mut meta, "sc20k.lens", "weapon light lens");
    if let Some(mm) = m {
        sc20k_part(g, out, gf, mm.tube, metal, None, &mut meta, "m4.tube", "buffer tube");
        sc20k_part(g, out, gf, mm.peq, if black { col.polymer } else { col.fde }, None, &mut meta, "m4.peq", "laser box");
        sc20k_part(g, out, gf, mm.gl, metal, None, &mut meta, "m4.gl", "40 mm launcher tube");
    }
    if !slung {
        if let Some(sink) = meta.as_deref_mut() {
            let off = s.grip_l.off;
            let o = v3::mad(v3::mad(v3::mad(gf.o, gf.x, off[0]), gf.y, off[1]), gf.z, off[2]);
            sink.push(RigNodeMeta { path: String::from("sc20k.gripL"), label: String::from("support hand grip point (arrows only)"), depth: 2.0, parent: gf, frame: Frame { o, ..gf }, box_start: out.len() as f64, box_count: 0.0 });
        }
    }
}
/// One part of the rifle: seg() at depth 2 with the rifle's tag. The part is a copy: seg writes a part only through a node's getAdj, which RigNodeMeta does not hold.
fn sc20k_part(g: &mut Game, out: &mut Vec<BoxId>, gf: Frame, part: Seg, albedo: V3, emissive: Option<V3>, meta: &mut Option<&mut Vec<RigNodeMeta>>, path: &str, label: &str) {
    let mut part = part;
    seg(g, out, gf, &mut part, albedo, emissive, meta.as_deref_mut(), Some(path), Some(label), Some(2.0), Some(SC20K_TAG));
}
/// `curOwner = c.id & 0xff`: the character id the boxes being emitted are stamped with
fn stamp_owner(g: &mut Game, c: CharId) {
    g.m.rig_props.cur_owner = (js::to_i32(g.ch[c].id) & 0xff) as f64;
}

/// The 2010s carbine's parts (GEAR.on is settled on). (rigProps.ts rifleM4)
pub fn rifle_m4(g: &mut crate::game::Game, rig: Option<crate::rig_props::Rig>) -> Option<crate::rig_props::RigM4> {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    Some(rig.m4)
}

/// The SC-20K's support-hand grip for the animator. (rigProps.ts sc20kGrip)
pub fn sc20k_grip(rig: Option<crate::rig_props::Rig>, fit: Option<bool>) -> crate::rig_props::Sc20kGrip {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let gl = rig.sc20k.grip_l;
    let gr = long_grip_of(rig.sc20k.adj, LongGripOfG { off: gl.off, fingers: gl.fingers, back: gl.back });
    if !fit.unwrap_or(false) {
        return Sc20kGrip { wrist: gr.wrist, fingers: gr.fingers, back: gr.back };
    }
    let fits = gripfit();
    let d = fits.long;
    let f = fits.fore;
    Sc20kGrip {
        wrist: v3::mad(v3::mad(v3::mad(v3::add(gr.wrist, d), gr.fingers, -f.back), gr.back, -f.lift), v3::normalize(v3::cross(gr.fingers, gr.back)), -f.down),
        fingers: gr.fingers,
        back: gr.back,
    }
}

/// What the hand pass is told of a long gun in his hands. (rigProps.ts longHold)
pub fn long_hold(weapon: crate::rig_props::LongHoldWeapon, rig: Option<crate::rig_props::Rig>) -> crate::inhand::InHandWord {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let fr = apply_adj(unit_frame(), if weapon == LongHoldWeapon::Sc20k { rig.sc20k.adj } else { rig.carbine.adj });
    let d = gripfit().long;
    crate::inhand::InHandWord::Long(crate::inhand::InHandWordLong {
        by: v3::scale(v3::mad(v3::mad(v3::scale(fr.x, d[0]), fr.y, d[1]), fr.z, d[2]), -1.0),
    })
}

/// …and of a torch: its beam, the point of its body the fist goes round, the way its body lies. (rigProps.ts torchHold)
pub fn torch_hold(beam: V3, rig: Option<crate::rig_props::Rig>) -> crate::inhand::InHandWord {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let fr = apply_adj(unit_frame(), rig.torch.adj);
    let t = gripfit().torch;
    crate::inhand::InHandWord::Torch(crate::inhand::InHandWordTorch { beam, at: fr.o, axis: fr.z, fwd: t.fwd, palm: t.palm })
}

/// How far ahead of the wrist along the bore its muzzle is. (rigProps.ts sc20kMuzzleAhead)
pub fn sc20k_muzzle_ahead(rig: Option<crate::rig_props::Rig>) -> f64 {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    muzzle_ahead_of(rig.sc20k.adj, rig.sc20k.suppressor)
}

/// Its weapon light: at the lens, pointing down the bore. (rigProps.ts sc20kLightPose)
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

/// Where a ring leaves it: the launcher port under the fore-end's nose. (rigProps.ts sc20kLauncherMuzzle)
pub fn sc20k_launcher_muzzle(g: &mut crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> V3 {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let s = rig.sc20k;
    let fr = sc20k_frame(g, c_2, s, Sc20kFrameWhere::Hand, None, None).g;
    let f = apply_adj(fr, s.fore.adj);
    v3::mad(v3::mad(f.o, f.z, s.fore.l1), f.y, -(s.fore.tall + 0.02))
}

/// The carbine's weapon light: at the lens part, pointing down the bore. (rigProps.ts carbineLightPose)
pub fn carbine_light_pose(g: &mut crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::CarbineLightPose {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let cb = rig.carbine;
    let hand = hand_or_muzzle(g, c_2);
    let fr = carbine_frame(g, c_2, hand, cb).g;
    let l = apply_adj(fr, cb.lens.adj);
    CarbineLightPose { pos: v3::mad(l.o, l.z, cb.light_ahead), dir: fr.z }
}

/// The player's rail light: its position and direction are the gun's. (rigProps.ts rigLightPose)
pub fn rig_light_pose(g: &mut crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::RigLightPose {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let hand = hand_or_muzzle(g, c_2);
    let fr = pistol_frame(g, c_2, hand, PistolFrameP { adj: rig.pistol.adj }, None).g;
    let r = apply_adj(Frame { o: v3::mad(fr.o, fr.y, -rig.pistol.light_under), ..fr }, rig.pistol.rail.adj);
    RigLightPose { pos: v3::mad(r.o, r.z, rig.pistol.light_ahead), dir: r.z }
}

/// A guard's light origin along its beam. (rigProps.ts rigTorchPos)
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

/// The player's rail light, parked and off until posed. (rigProps.ts railLightDef)
pub fn rail_light_def(owner: f64, intensity: f64) -> LightId {
    seam_rt_light("pistol_light", "player", 1.0, [0.0, -10.0, 0.0], [0.0, 0.0, 1.0], [0.9, 0.96, 1.0], intensity, 18.0, 5.0, 15.0, 0.035, 1.0, owner, false, -1.0, 0.0, -1.0, 0.0, intensity, true)
}

/// A guard's torch / weapon light, warm and wider. (rigProps.ts torchLightDef)
pub fn torch_light_def(name: &str, owner: f64, intensity: f64, inner_deg: f64, outer_deg: f64) -> LightId {
    seam_rt_light(name, "flashlights", 1.0, [0.0, -10.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.92, 0.78], intensity, 16.0, inner_deg, outer_deg, 0.05, 1.0, owner, true, -1.0, 0.0, -1.0, 0.0, intensity, true)
}

/// An `RtLight` literal, in the TypeScript's field order: a LightId is a row of rust/sim and these functions have no `g` to make one in.
fn seam_rt_light(name: &str, group: &str, kind: f64, pos: V3, dir: V3, color: V3, intensity: f64, range: f64, inner_deg: f64, outer_deg: f64, radius: f64, volumetric: f64, owner: f64, enabled: bool, fixture_box: f64, flicker: f64, ttl: f64, age: f64, peak_intensity: f64, card: bool) -> LightId {
    todo!("needs: a value type for an RtLight literal, or a g and a maker that gives a LightId row")
}

/// The relaxed torch carry's beam, eased from last frame's. (rigProps.ts torchCarryDir)
pub fn torch_carry_dir(g: &mut crate::game::Game, c_2: CharId, prev: V3, dt: Option<f64>, low: Option<crate::rig_props::TorchCarryDirLow>) -> V3 {
    let dt = dt.unwrap_or(1.0 / 60.0);
    let want = torch_carry_want(g, c_2, low.map(|l| -l.tan));
    let rate = match low { Some(l) => l.rate, None => TORCH_CARRY_RATE };
    v3::normalize(v3::lerp3(prev, want, 1.0 - m::exp(-rate * m::max(0.0, dt))))
}

/// Where the relaxed carry is easing to: half his aim heading, half his chest. (rigProps.ts torchCarryWant)
pub fn torch_carry_want(g: &mut crate::game::Game, c_2: CharId, tan: Option<f64>) -> V3 {
    let (aim_yaw, aim_pitch, chest) = (g.ch[c_2].aim_yaw, g.ch[c_2].aim_pitch, g.ch[c_2].chest_dir);
    let f: V3 = [m::sin(aim_yaw), 0.0, m::cos(aim_yaw)];
    let y = match tan { Some(t) => t, None => m::tan(aim_pitch) };
    v3::normalize([f[0] * 0.5 + chest[0] * 0.5, y, f[2] * 0.5 + chest[2] * 0.5])
}

/// How far off his back the big slot's tool rides with his rifle slung under it. (rigProps.ts bigClearOf)
pub fn big_clear_of(g: &mut crate::game::Game, c_2: CharId, kind: crate::rig_props::BigClearOfKind, rig: Option<crate::rig_props::Rig>) -> f64 {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let p = rig.sc20k.slung;
    let s = slung_base(g, c_2, p, false, None);
    let out = p.off[2] - v3::dot(v3::sub(s.o, s.cf.o), s.cf.z);
    let clear = bigrig().clear;
    (match kind { BigClearOfKind::Hammer => clear.hammer, BigClearOfKind::Shotgun => clear.shotgun }) + (if m::abs(out) < 1e-6 { 0.0 } else { out })
}

/// The sizes of the thing the pack-side rule stands at a pack's side. (rigProps.ts packSideDims)
pub fn pack_side_dims(g: &mut crate::game::Game, what: crate::rig_props::PackSideDimsWhat, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::PackSideDims {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    if what == PackSideDimsWhat::Hammer {
        let h = bigrig().hammer;
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
    let m4 = rifle_m4(g, Some(rig));
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

/// The pack-side place in the bind space of the mesh he wears, pitched to his back as posed. (rigProps.ts packSideBind)
pub fn pack_side_bind(g: &mut crate::game::Game, c_2: CharId, what: crate::rig_props::PackSideBindWhat, rig: Option<crate::rig_props::Rig>) -> Option<crate::packside::SidePlace> {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let parts = g.ch[c_2].parts;
    let e = seam_pack_of(g, c_2, parts)?;
    let (dims_what, side_what) = match what {
        PackSideBindWhat::Hammer => (PackSideDimsWhat::Hammer, crate::packside::SideBindWhat::Hammer),
        PackSideBindWhat::Shotgun => (PackSideDimsWhat::Shotgun, crate::packside::SideBindWhat::Shotgun),
        PackSideBindWhat::Rifle => (PackSideDimsWhat::Rifle, crate::packside::SideBindWhat::Rifle),
    };
    let d = pack_side_dims(g, dims_what, Some(rig));
    let dims = crate::packside::SideBindDims { grip: d.grip, thick: d.thick, head: d.head, rear: d.rear, top: d.top, len: Some(d.len) };
    let p = crate::packside::side_bind(g, e, side_what, dims);
    let th = seam_side_pitch(g, c_2, p);
    Some(crate::packside::side_pitched(p, th))
}

/// …carried into the world by the chest joint's skin matrix. (rigProps.ts packSide)
pub fn pack_side(g: &mut crate::game::Game, c_2: CharId, what: crate::rig_props::PackSideWhat, rig: Option<crate::rig_props::Rig>) -> Option<crate::rig_props::Frame> {
    let bind_what = match what {
        PackSideWhat::Hammer => PackSideBindWhat::Hammer,
        PackSideWhat::Shotgun => PackSideBindWhat::Shotgun,
        PackSideWhat::Rifle => PackSideBindWhat::Rifle,
    };
    let p = pack_side_bind(g, c_2, bind_what, rig)?;
    let j = seam_chest_joint(g, c_2);
    if j < 0.0 { return None; }
    let jd: Vec<f64> = g.ch[c_2].skel.joint_data.iter().map(|x| *x as f64).collect();   // (the stub takes doubles: a Float32Array read is its double, so this is exact)
    let w = crate::packside::side_skinned(p, &jd, j * 16.0);
    Some(Frame { o: w.o, x: w.x, y: w.y, z: w.z })
}

/// `packOf(c.ch, c.parts)`: the stub takes a `&PackChar` and `c.ch` is a RigId, a GltfCharacter row.
fn seam_pack_of(g: &mut Game, c: CharId, mask: f64) -> Option<crate::packside::PackExt> {
    todo!("needs: pack_of taking a RigId, or a PackChar view of g.tab.rig[..]")
}
/// `sidePitch(c.ch, p, c.skel.jointData)`: `&PackChar` from a RigId, and `jd: &[f64]` where joint_data is `Vec<f32>`.
fn seam_side_pitch(g: &mut Game, c: CharId, p: crate::packside::SidePlace) -> f64 {
    todo!("needs: side_pitch taking a RigId and jd: &[f32]")
}
/// `chestJoint(c.ch)`: the stub takes a `&PackChar` and `c.ch` is a RigId.
fn seam_chest_joint(g: &mut Game, c: CharId) -> f64 {
    todo!("needs: chest_joint taking a RigId")
}

/// Where a thing across the back is, off six joints in whatever one space they are given in. (rigProps.ts bigBackOf)
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

/// The big slot's tool on a man: on his back, in his hands, or carried between by `k`. (rigProps.ts bigBoxes)
pub fn big_boxes(g: &mut crate::game::Game, c_2: CharId, kind: crate::rig_props::BigBoxesKind, k: f64, out: &mut Vec<BoxId>, rig: Option<crate::rig_props::Rig>, clear: Option<bool>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) {
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
            let table = bigrig();
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
                let grip = bigrig().hammer.grip;
                gf = mix_frame(on_back, Frame { o: v3::mad(v3::mad(v3::mad(hand, h.fingers, grip.fwd), h.back, -grip.palm), h.thumb, grip.side), x: v3::cross(h.back, h.thumb), y: h.back, z: h.thumb }, k);
            } else {
                gf = mix_frame(on_back, carbine_frame(g, c_2, hand, rig.carbine).g, k);
            }
        }
    }
    let mut meta = meta;
    if kind == BigBoxesKind::Shotgun {
        let mut cb = rig.carbine;
        carbine_boxes(g, c_2, out, &mut cb, [0.0, 0.0, 0.0], meta, Some(gf), Some(crate::weapons::GunModelId::M870));
        return;
    }
    if let Some(sink) = meta.as_deref_mut() {
        sink.push(RigNodeMeta { path: String::from("big.hammer"), label: String::from("dynamic hammer (big slot)"), depth: 1.0, parent: on_back, frame: gf, box_start: out.len() as f64, box_count: 2.0 });
    }
    let (table, col) = (bigrig(), crate::rig_props::col());
    let (mut handle, mut head) = (table.hammer.handle, table.hammer.head);
    seg_at(g, out, gf, &mut handle, None, col.haft, no_glow(), None, Some(""), Some(""), Some(0.0), Some(BIG_TAG));
    seg_at(g, out, gf, &mut head, None, col.sledge, no_glow(), None, Some(""), Some(""), Some(0.0), Some(BIG_TAG));
}

/// A man's own long gun let go of for the big slot's tool: on its sling down his front, dark. (rigProps.ts hungBoxes)
pub fn hung_boxes(g: &mut crate::game::Game, c_2: CharId, weapon: crate::rig_props::HungBoxesWeapon, out: &mut Vec<BoxId>, rig: Option<crate::rig_props::Rig>, black: Option<bool>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) {
    let rig = rig.unwrap_or_else(crate::rig_props::rig);
    let black = black.unwrap_or(false);
    if weapon == HungBoxesWeapon::Sc20k {
        let mut s = rig.sc20k;
        let m4 = rifle_m4(g, Some(rig));
        sc20k_boxes(g, c_2, out, Some(&mut s), Some(Sc20kBoxesWhere::Slung), Some([0.0, 0.0, 0.0]), meta, m4, None, Some(false), Some(true), None, Some(black));
        return;
    }
    stamp_owner(g, c_2);
    let at = sc20k_frame(g, c_2, rig.sc20k, Sc20kFrameWhere::Slung, Some(true), None).g;
    let mut cb = rig.carbine;
    carbine_boxes(g, c_2, out, &mut cb, [0.0, 0.0, 0.0], meta, Some(at), None);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn bits(v: V3) -> [u64; 3] { [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()] }
    /// the TypeScript's own bits: bigBackOf under bun with EXACT.on, hammer's back table, clear 0.11
    #[test]
    fn big_back_of_matches_the_typescript() {
        let p = RigSc20kSlung { off: [-0.055, 0.01, -0.19], bore: [-0.42, 0.91, 0.0], roll: 90.0, follow: 0.5, adj: None };
        let r = big_back_of([0.013, 0.951, -0.027], [0.021, 1.203, -0.041], [0.11, 0.93, 0.02], [-0.09, 0.94, -0.05], [0.19, 1.41, 0.03], [-0.17, 1.39, -0.08], p, Some(0.11));
        assert_eq!(bits(r.o), [0x3fafc710302174ea, 0x3ff36f160fac0603, 0xbfd5f669903797aa]);
        assert_eq!(bits(r.z), [0xbfd9dfde376f41da, 0x3fecfcba56fa7336, 0xbfc02ef46dfdf683]);
        assert_eq!(bits(r.cf.x), [0x3fee68b0f04a8024, 0x3f7453772f825c4a, 0x3fd3ed4abc383865]);
        assert_eq!(bits(r.cf.y), [0xbf7b5dd98f7a23ce, 0x3fefffbc6318d0a4, 0x3f723e910a516d40]);
        assert_eq!(bits(r.cf.z), [0xbfd3ecc3ec27e84e, 0xbf79dbac476a2b7e, 0x3fee68b638234821]);
    }
}
