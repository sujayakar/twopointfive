use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The hand frame the pistol group lives in: wrist origin, z the bore, y the top of the slide.
pub(super) fn pistol_frame(g: &crate::game::Game, c_2: CharId, hand: V3, p: crate::rig_props::PistolFrameP, in_hand: Option<bool>) -> crate::rig_props::PistolFrame {
    let in_hand = in_hand.unwrap_or(false);
    let hf = if in_hand { Some(crate::character::character_grip_frame_r(g, c_2)) } else { None };
    let hand_f = match hf {
        Some(hf) => frame_from(hand, hf.bore, Some(hf.top)),
        None => {
            let gun_dir = g.ch[c_2].gun_dir;
            let under = crate::character::character_gun_under(g, c_2);
            frame_from(hand, gun_dir, Some(v3::scale(under, -1.0)))
        }
    };
    PistolFrame { hand_f, g: apply_adj(hand_f, p.adj) }
}

/// The frame `k` of the way from a to b: origins lerped, axes slerped.
pub(super) fn mix_frame(a: crate::rig_props::Frame, b: crate::rig_props::Frame, k: f64) -> crate::rig_props::Frame {
    if k <= 0.0 {
        return a;
    }
    if k >= 1.0 {
        return b;
    }
    let q = v3::quat::slerp(v3::quat::from_basis(a.x, a.y, a.z), v3::quat::from_basis(b.x, b.y, b.z), k);
    Frame {
        o: v3::lerp3(a.o, b.o, k),
        x: v3::quat::rotate(q, [1.0, 0.0, 0.0]),
        y: v3::quat::rotate(q, [0.0, 1.0, 0.0]),
        z: v3::quat::rotate(q, [0.0, 0.0, 1.0]),
    }
}

/// The thigh holster's group frame off three joints and the body's forward, in whatever one space they are given.
fn holster_frame_of(th: V3, kn: V3, hp: V3, fwd: V3, h: Option<crate::rig_props::RigHolster>) -> crate::rig_props::Frame {
    let h = h.unwrap_or_else(|| crate::rig_props::rig().holster);
    let down = v3::normalize(v3::sub(kn, th));
    let mut outward = v3::sub(th, hp);
    outward = v3::sub(outward, v3::scale(down, v3::dot(outward, down)));
    outward[1] = 0.0;
    outward = if v3::len(outward) > 1e-3 { v3::normalize(outward) } else { v3::cross(fwd, [0.0, 1.0, 0.0]) };
    apply_adj(frame_from(v3::mad(v3::mad(v3::mad(th, down, h.down), outward, h.out), fwd, h.fwd), down, Some(fwd)), h.adj)
}

/// The long model drawn for a gun in his hands, or for the one that is not (slung, hung); none = the group's own parts.
pub(super) fn long_look_of(g: &crate::game::Game, c_2: CharId, held: bool) -> Option<crate::rig_props::LongLookOf> {
    let k = g.ch[c_2].gun_look?;
    let a = if held { k.hand } else { k.long };
    if let Some(l) = crate::gunlook::long_model(a) {
        return Some(LongLookOf { id: a.unwrap(), l });
    }
    if !held {
        return None;
    }
    // (the frames of a swap: the long gun is still drawn in the hand that is letting go of it)
    crate::gunlook::long_model(k.long).map(|l| LongLookOf { id: k.long.unwrap(), l })
}

/// A long model's support-hand grip for the animator; `fit` moves it with the gun as drawn, and a fist round a post into the hook of the fingers.
pub fn model_grip(g: &crate::game::Game, id_2: crate::weapons::GunModelId, fit: Option<bool>, rig: Option<crate::rig_props::Rig>) -> Option<crate::character::AnimLongGrip> {
    let fit = fit.unwrap_or(false);
    let rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    let l = crate::gunlook::long_model(Some(id_2))?;
    let gr = long_grip_of(rig.sc20k.adj, LongGripOfG { off: l.grip_l.off, fingers: l.grip_l.fingers, back: l.grip_l.back });
    if !fit {
        return Some(gr);
    }
    let t = gripfit();
    let (d, f) = (t.long, t.fore);
    if !l.post {
        return Some(crate::character::AnimLongGrip { wrist: v3::add(gr.wrist, d), fingers: gr.fingers, back: gr.back });
    }
    Some(crate::character::AnimLongGrip {
        wrist: v3::mad(v3::mad(v3::mad(v3::add(gr.wrist, d), gr.fingers, -f.back), gr.back, -f.lift), v3::normalize(v3::cross(gr.fingers, gr.back)), -f.down),
        fingers: gr.fingers,
        back: gr.back,
    })
}

/// How far ahead of the wrist along the bore a long model's muzzle is.
pub fn model_muzzle_ahead(id_2: crate::weapons::GunModelId, rig: Option<crate::rig_props::Rig>) -> Option<f64> {
    let rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    let l = crate::gunlook::long_model(Some(id_2))?;
    Some(muzzle_ahead_of(rig.sc20k.adj, Seg { l0: 0.0, l1: l.ahead, thick: 0.0, tall: 0.0, adj: None }))
}

/// A vector in the chest's axes [left, up, fwd] as a world one (sheathFrame's `W`).
fn sheath_w(c: &crate::character::CharacterChestFrame, v: V3) -> V3 {
    v3::mad(v3::mad(v3::scale(c.left, v[0]), c.up, v[1]), c.fwd, v[2])
}

/// The sheath's frame in the world this frame: origin the sheathed handle's grip, z handle to tip, x off the chest. None before his first update.
fn sheath_frame(g: &crate::game::Game, c_2: CharId, s: Option<crate::rig_props::RigSheath>) -> Option<crate::rig_props::Frame> {
    let s = s.unwrap_or_else(|| crate::rig_props::rig().sheath);
    let c = crate::character::character_chest_frame(g, c_2)?;
    let z = v3::normalize(sheath_w(&c, s.along));
    let x0 = sheath_w(&c, s.out);
    let x = v3::normalize(v3::mad(x0, z, -v3::dot(x0, z)));
    Some(Frame { o: v3::add(c.o, sheath_w(&c, s.at)), x, y: v3::cross(z, x), z })
}

/// The carbine's group frame: the sidearm's hand frame moved by the group adj onto the bore above the trigger.
pub(super) fn carbine_frame(g: &crate::game::Game, c_2: CharId, hand: V3, c_3: crate::rig_props::RigCarbine) -> crate::rig_props::CarbineFrame {
    let f = pistol_frame(g, c_2, hand, PistolFrameP { adj: c_3.adj }, None);
    CarbineFrame { hand_f: f.hand_f, g: f.g }
}

/// What the animator needs of the carbine to put the support hand on the gun.
pub fn carbine_grip(rig: Option<crate::rig_props::Rig>) -> crate::character::AnimLongGrip {
    let rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    let gl = rig.carbine.grip_l;
    long_grip_of(rig.carbine.adj, LongGripOfG { off: gl.off, fingers: gl.fingers, back: gl.back })
}

/// How far ahead of the wrist along the bore the carbine's muzzle is.
pub fn carbine_muzzle_ahead(rig: Option<crate::rig_props::Rig>) -> f64 {
    let rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    muzzle_ahead_of(rig.carbine.adj, rig.carbine.muzzle)
}

/// The frame every group adjustment starts from: the origin and the unit axes.
fn unit_frame() -> Frame {
    Frame { o: [0.0, 0.0, 0.0], x: [1.0, 0.0, 0.0], y: [0.0, 1.0, 0.0], z: [0.0, 0.0, 1.0] }
}

/// A direction in the frame's axes as a unit vector (longGripOf's `dirIn`).
fn dir_in(f: &Frame, d: V3) -> V3 {
    v3::normalize(v3::mad(v3::mad(v3::scale(f.x, d[0]), f.y, d[1]), f.z, d[2]))
}

/// A long gun's support-hand grip (gripL through the group adj) in the trigger hand's gun frame.
pub(super) fn long_grip_of(group_adj: Option<crate::rig_props::Adj>, g_2: crate::rig_props::LongGripOfG) -> crate::character::AnimLongGrip {
    let f = apply_adj(unit_frame(), group_adj);
    crate::character::AnimLongGrip {
        wrist: v3::mad(v3::mad(v3::mad(f.o, f.x, g_2.off[0]), f.y, g_2.off[1]), f.z, g_2.off[2]),
        fingers: dir_in(&f, g_2.fingers),
        back: dir_in(&f, g_2.back),
    }
}

/// Metres ahead of the wrist along the bore to the far end of a long gun's foremost part.
pub(super) fn muzzle_ahead_of(group_adj: Option<crate::rig_props::Adj>, part: crate::rig_props::Seg) -> f64 {
    let f = apply_adj(apply_adj(unit_frame(), group_adj), part.adj);
    v3::mad(f.o, f.z, part.l1)[2]
}

/// The right hand that holds the rifle as it hangs in front, in the torso's axes off the mid-back joint.
pub fn sling_front_hand(s: Option<crate::rig_props::RigSc20k>, rest: Option<f64>) -> crate::rig_props::SlingFrontHand {
    let s = s.unwrap_or_else(|| crate::rig_props::rig().sc20k);
    let rest = rest.unwrap_or(0.0);
    let p = sling_front();
    let mut z = v3::normalize(p.bore);
    let mut o: V3 = p.off;
    if rest > 0.0 {
        let r = sling_rest();
        let piv = v3::mad(o, z, -r.pivot);
        z = rot_axis(z, [1.0, 0.0, 0.0], -m::min(r.max, rest));
        o = v3::mad(piv, z, r.pivot);
    }
    let mut x: V3 = [-z[0] * z[2], -z[1] * z[2], 1.0 - z[2] * z[2]];
    x = if v3::len(x) < 1e-3 { [1.0, 0.0, 0.0] } else { v3::normalize(x) };
    x = rot_axis(x, z, p.roll * m::PI / 180.0);
    let y = v3::cross(z, x);
    let a: V3 = match s.adj {
        Some(adj) => adj.off,
        None => [0.0, 0.0, 0.0],
    };
    SlingFrontHand {
        at: [
            o[0] - (x[0] * a[0] + y[0] * a[1] + z[0] * a[2]),
            o[1] - (x[1] * a[0] + y[1] * a[1] + z[1] * a[2]),
            o[2] - (x[2] * a[0] + y[2] * a[1] + z[2] * a[2]),
        ],
        fing: z,
        back: v3::scale(x, -1.0),
    }
}

// ---------------------------------------------------------------- the boxes he carries (the frame's dynamic list: the picture)

/// `c.id & 0xff`: the owner byte every box of his carries.
fn owner_of(g: &mut crate::game::Game, c: CharId) -> f64 {
    (js::to_i32(g.ch[c].id) & 0xff) as f64
}

/// One of his bones, none before his first update.
fn bone(g: &crate::game::Game, c: CharId, b: crate::character::Bone) -> Option<V3> {
    g.ch[c].bones.get(b).copied()
}

/// `id as WeaponId`: every gun model is a weapon row of the same word.
fn weapon_of(id: crate::weapons::GunModelId) -> crate::weapons::WeaponId {
    use crate::weapons::{GunModelId, WeaponId};
    match id {
        GunModelId::M870 => WeaponId::M870,
        GunModelId::Mk12 => WeaponId::Mk12,
        GunModelId::Mp7 => WeaponId::Mp7,
        GunModelId::Mk46 => WeaponId::Mk46,
        GunModelId::M1014 => WeaponId::M1014,
        GunModelId::Hk45 => WeaponId::Hk45,
        GunModelId::G18 => WeaponId::G18,
    }
}

/// A model's Seg as the rig's: the two modules each have a type for the one TypeScript Seg.
fn of_model(s: crate::gunlook::Seg) -> Seg {
    Seg { l0: s.l0, l1: s.l1, thick: s.thick, tall: s.tall, adj: s.adj.map(|a| Adj { off: a.off, rot: a.rot }) }
}


/// `seg(out, f, s, albedo, emissive, path, label, 2, name)`: a named part of a group in the viewer's tree.
fn part(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>, f: Frame, s: Seg, albedo: V3, emissive: Option<V3>, name: Option<&str>) {
    seg(g, out, f, s, albedo, emissive, name);
}

/// `segAt(out, f, s, adj, albedo, NO_GLOW)`: an unlit part with its adjustment handed in, no node of its own.
fn part_at(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>, f: Frame, s: Seg, adj: Option<Adj>, albedo: V3) {
    seg_at(g, out, f, s, adj, albedo, no_glow(), None);
}

/// A gun group's frame moved by a GRIPFIT shift, in its own axes.
pub(super) fn fit_frame(g_2: crate::rig_props::Frame, d: V3) -> crate::rig_props::Frame {
    Frame { o: v3::mad(v3::mad(v3::mad(g_2.o, g_2.x, d[0]), g_2.y, d[1]), g_2.z, d[2]), ..g_2 }
}

/// Two adjustments mixed, none being zero: a part going over from how it sits in one frame to how it sits in another.
fn mix_adj(a: Option<crate::rig_props::Adj>, b: Option<crate::rig_props::Adj>, k: f64) -> Option<crate::rig_props::Adj> {
    if k <= 0.0 || (a.is_none() && b.is_none()) {
        return a;
    }
    let none = adj_none();
    let (aa, bb) = (a.unwrap_or(none), b.unwrap_or(none));
    let l = |x: V3, y: V3, i: usize| x[i] + (y[i] - x[i]) * k;
    Some(Adj {
        off: [l(aa.off, bb.off, 0), l(aa.off, bb.off, 1), l(aa.off, bb.off, 2)],
        rot: [l(aa.rot, bb.rot, 0), l(aa.rot, bb.rot, 1), l(aa.rot, bb.rot, 2)],
    })
}

/// The gun group's frame as the hand itself has it (bore wrist to knuckles, top the thumb side), `roll` of the way from the usual frame.
pub(super) fn kit_gun_frame(g: &crate::game::Game, c_2: CharId, hand: V3, p: crate::rig_props::KitGunFrameP, g_2: crate::rig_props::Frame, roll: f64) -> crate::rig_props::Frame {
    let kn = bone(g, c_2, crate::character::Bone::FingerR);
    let kn = match kn {
        Some(kn) if roll > 0.0 => kn,
        _ => return g_2,
    };
    let mut z = v3::sub(kn, hand);
    if v3::len(z) < 1e-4 {
        return g_2;
    }
    let mut th = crate::character::character_hand_frame_r(g, c_2, None).thumb;
    if let Some(lie) = crate::character::character_kit_lie_world(g, c_2) {
        z = v3::quat::rotate(lie, z);
        th = v3::quat::rotate(lie, th);
    }
    mix_frame(g_2, apply_adj(frame_from(hand, z, Some(th)), p.adj), roll)
}

/// A magazine in the support hand: one dark box up out of the fist; `own` / `col` give a gun's own feed its extents and colour.
pub fn mag_box_l(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<AnyBoxId>, kind: crate::slotsetup::SetupGun, fit: Option<bool>, own: Option<V3>, col: Option<V3>) {
    let fit = fit.unwrap_or(false);
    let (a, b) = (bone(g, c_2, crate::character::Bone::HandL), bone(g, c_2, crate::character::Bone::FingerL));
    let (a, b) = match (a, b) {
        (Some(a), Some(b)) => (a, b),
        _ => return,
    };
    let h: V3 = own.unwrap_or(if kind == crate::slotsetup::SetupGun::Rifle { [0.012, 0.085, 0.03] } else { [0.008, 0.055, 0.016] });
    let albedo = col.unwrap_or(crate::rig_props::col().gun);
    let owner = owner_of(g, c_2);
    if fit {
        let hf = crate::character::character_hand_frame_l(g, c_2);
        let f = gripfit().mag;
        let x = v3::cross(hf.thumb, hf.fingers);
        let made = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP {
            c: v3::mad(v3::mad(v3::mad(a, hf.fingers, f.fwd), hf.back, -f.palm), hf.thumb, f.side),
            h,
            yaw: Some(m::atan2(hf.fingers[0], hf.fingers[2])),
            rot: Some(v3::quat::from_basis(x, hf.thumb, hf.fingers)),
            albedo: Some(albedo),
            flags: Some(PROP_FLAGS),
            owner: Some(owner),
            ..Default::default()
        });
        out.push(made.into());
        return;
    }
    let f = v3::sub(b, a);
    let l = v3::len(f);
    if l < 1e-4 {
        return;
    }
    let z = v3::scale(f, 1.0 / l);
    let mut y: V3 = v3::mad([0.0, 1.0, 0.0], z, -z[1]);
    y = if v3::len(y) > 1e-3 { v3::normalize(y) } else { [1.0, 0.0, 0.0] };
    let x = v3::cross(y, z);
    let made = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP {
        c: v3::mad(v3::mad(a, z, 0.085), y, h[1] * 0.35),
        h,
        yaw: Some(m::atan2(z[0], z[2])),
        rot: Some(v3::quat::from_basis(x, y, z)),
        albedo: Some(albedo),
        flags: Some(PROP_FLAGS),
        owner: Some(owner),
        ..Default::default()
    });
    out.push(made.into());
}

/// Held and worn props for one character as small boxes, appended to `out`. Call after his update: it needs his bones.
pub fn rig_boxes(g: &mut crate::game::Game, c_2: CharId, o: crate::rig_props::RigOpts, out: &mut Vec<AnyBoxId>, rig: Option<crate::rig_props::Rig>) {
    let rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    kit_boxes(g, c_2, o, out, rig);
    if let Some(big) = o.big {
        let kind = big;
        big_boxes(g, c_2, kind, o.big_k.unwrap_or(0.0), out, Some(rig), Some(o.big_clear.unwrap_or(false)));
    }
}

/// His kit, the big slot's tool apart: Sam's three slots, or a guard's torch, pistol or long gun.
fn kit_boxes(g: &mut crate::game::Game, c: CharId, o: RigOpts, out: &mut Vec<AnyBoxId>, rig: Rig) {
    let hand = bone(g, c, crate::character::Bone::HandR);
    g.m.rig_props.cur_owner = owner_of(g, c);
    if !o.is_guard { sam_kit(g, c, o, out, rig, hand); } else if let Some(hand) = hand { guard_kit(g, c, o, out, rig, hand); }
}

/// What Sam carries: what is in his hand, and the rest of it where he wears it.
fn sam_kit(g: &mut Game, c: CharId, o: RigOpts, out: &mut Vec<AnyBoxId>, rig: Rig, hand: Option<V3>) {
    if o.slot == 1.0 {
        if let Some(hand) = hand { player_pistol(g, c, o, out, rig, hand); }
    } else {
        if o.holster == Some(true) {
            let parts = HolsterBoxesParts { slide: rig.pistol.slide, grip: rig.pistol.grip, suppressor: Some(rig.pistol.suppressor) };
            holster_boxes(g, c, out, parts, Some(rig.holster));
        }
        if let Some(held) = o.held { held_can_boxes(g, c, held, out, Some(rig.can), o.fit, None); }
    }
    let knife = o.knife == Some(true);
    if knife { knife_boxes(g, c, out, Some(rig.knife)); }
    if o.sheath == Some(true) { sheath_boxes(g, c, out, Some(!knife), Some(rig)); }
    match o.rifle {
        Some(RifleAt::Hand) if hand.is_some() => {
            let lens = if o.rifle_light == Some(true) { v3::scale(rail_lens_color(), rig.sc20k.lens_glow) } else { [0.0, 0.0, 0.0] };
            sc20k_boxes(g, c, out, Some(rig.sc20k), Some(RifleAt::Hand), Some(lens), rifle_m4(Some(rig)), o.kit, o.fit, Some(false), None, o.rifle_black);
        }
        Some(RifleAt::Slung) => sc20k_boxes(g, c, out, Some(rig.sc20k), Some(RifleAt::Slung), Some([0.0, 0.0, 0.0]), rifle_m4(Some(rig)), None, Some(false), o.sling_front, o.sling_rest, o.rifle_black),
        _ => {}
    }
    if let Some(mag_l) = o.kit.and_then(|k| k.mag_l) { mag_box_l(g, c, out, mag_l, o.fit, None, None); }
    if o.goggles { goggles_boxes(g, c, o.nv_k.map_or(o.nv, |k| k > 0.5), out, rig.goggles, o.nv_k); }
}

/// What a guard carries: a long gun in his hands or hung on him, or a torch, with his pistol in its holster if he wears one; or the pistol.
fn guard_kit(g: &mut Game, c: CharId, o: RigOpts, out: &mut Vec<AnyBoxId>, rig: Rig, hand: V3) {
    let lit = |glow: f64| -> V3 { match o.lens_color { Some(col) if o.light_on => v3::scale(col, glow), _ => [0.0, 0.0, 0.0] } };
    let hung = o.hang == Some(true);
    let alive = g.ch[c].alive;
    match o.weapon {
        Some(w @ (RigWeapon::Carbine | RigWeapon::Sc20k)) => {
            let sc20k = w == RigWeapon::Sc20k;
            if hung { hung_boxes(g, c, if sc20k { LongRig::Sc20k } else { LongRig::Carbine }, out, Some(rig), Some(false)); }
            else if sc20k { sc20k_boxes(g, c, out, Some(rig.sc20k), Some(RifleAt::Hand), Some(lit(rig.sc20k.lens_glow)), rifle_m4(Some(rig)), None, None, None, None, None); }
            else { carbine_boxes(g, c, out, rig.carbine, lit(rig.carbine.lens_glow), None, None); }
        }
        _ if hung => {}
        _ if o.torch == Some(true) && alive => guard_torch(g, c, o, out, rig, hand, lit(rig.torch.lens_glow)),
        _ => return guard_pistol(g, c, o, out, rig, hand, if alive { Some(lit(rig.torch.lens_glow)) } else { None }),
    }
    if o.holster == Some(false) { return; }
    let at = o.holster_out.filter(|&x| js::truthy(x)).map(|x| RigHolster { out: rig.holster.out + x, ..rig.holster });
    holster_boxes(g, c, out, HolsterBoxesParts { slide: rig.guard_pistol.slide, grip: rig.guard_pistol.grip, suppressor: None }, at);
}
fn guard_torch(g: &mut Game, c: CharId, o: RigOpts, out: &mut Vec<AnyBoxId>, rig: Rig, hand: V3, lens: V3) {
    let (cols, t) = (col(), rig.torch);
    let f = apply_adj(frame_from(hand, o.beam_dir.unwrap_or(g.ch[c].gun_dir), None), t.adj);
    part(g, out, f, t.body, cols.torch, None, None);
    part(g, out, f, t.lens, cols.lens_body, Some(lens), None);
}
fn guard_pistol(g: &mut Game, c: CharId, o: RigOpts, out: &mut Vec<AnyBoxId>, rig: Rig, hand: V3, lens: Option<V3>) {
    let (cols, gp) = (col(), rig.guard_pistol);
    let in_hand = o.in_hand == Some(true);
    let at = pistol_frame(g, c, hand, PistolFrameP { adj: gp.adj }, Some(in_hand)).g;
    let f = if in_hand { fit_frame(at, gripfit().guard_pistol) } else { at };
    if let Some(gm) = pistol_look_of(g, c, true) {
        seg_at(g, out, f, of_model(gm.slide), gp.slide.adj, cols.gun, no_glow(), None);
        seg_at(g, out, f, of_model(gm.grip), gp.grip.adj, model_col(gm.frame, false), no_glow(), None);
        if let Some(can) = gm.can { part_at(g, out, f, of_model(can), gp.slide.adj, gm.can_col.map_or(cols.can, |c| model_col(c, false))); }
        if let Some(mag) = gm.mag { part_at(g, out, f, of_model(mag), gp.grip.adj, cols.dark_metal); }
    } else {
        part(g, out, f, gp.slide, cols.gun, None, None);
        part(g, out, f, gp.grip, cols.gun, None, None);
    }
    if let Some(lens) = lens { part(g, out, f, gp.lens, cols.lens_body, Some(lens), None); }
}

/// Sam's pistol in his right hand (kitBoxes' first branch): a closure's worth of locals, so a function here.
fn player_pistol(g: &mut crate::game::Game, c_2: CharId, o: RigOpts, out: &mut Vec<AnyBoxId>, rig: Rig, hand: V3) {
    let cols = col();
    let pm = pistol_look_of(g, c_2, true);
    let p = match pm {
        Some(pm) => pistol_as(rig.pistol, pm),
        None => rig.pistol,
    };
    let pcol = match pm {
        Some(pm) => model_col(pm.frame, false),
        None => cols.gun,
    };
    let ccol = match pm.and_then(|pm| pm.can_col) {
        Some(c) => model_col(c, false),
        None => cols.can,
    };
    let look = g.ch[c_2].gun_look;
    let bare_side = look.and_then(|k| k.bare_side).unwrap_or(false);
    let pid = if pm.is_some() { look.and_then(|k| k.hand).map(weapon_of) } else { None };
    let pshow = match pid {
        Some(pid) => crate::gunreload::reload_show(g, c_2, pid),
        None => None,
    };
    let in_hand = o.in_hand.unwrap_or(false);
    let fit = o.fit.unwrap_or(false);
    let pf = pistol_frame(g, c_2, hand, PistolFrameP { adj: p.adj }, Some(in_hand));
    let mut f = pf.g;
    if let Some(kit) = o.kit {
        f = kit_gun_frame(g, c_2, hand, KitGunFrameP { adj: p.adj }, f, kit.roll.unwrap_or(0.0));
        if fit {
            f = fit_frame(f, gripfit().pistol);
        }
        let th = bone(g, c_2, crate::character::Bone::ThighR);
        let kn = bone(g, c_2, crate::character::Bone::ShinR);
        let hp = bone(g, c_2, crate::character::Bone::Hips);
        if let (Some(th), Some(kn), Some(hp)) = (th, kn, hp) {
            if kit.pistol.unwrap_or(0.0) > 0.0 {
                let fwd = crate::character::character_forward(g, c_2);
                f = mix_frame(f, holster_frame_of(th, kn, hp, fwd, Some(rig.holster)), kit.pistol.unwrap());
            }
        }
    } else if fit || in_hand {
        f = fit_frame(f, gripfit().pistol);
    }
    let rail_adj_z = match p.rail.adj {
        Some(a) => a.off[2],
        None => 0.0,
    };
    let rail_back = if pm.is_none() && bare_side { m::max(0.0, p.rail.l1 + rail_adj_z - p.slide.l1) } else { 0.0 };
    let hk = o.kit.and_then(|k| k.pistol).unwrap_or(0.0);
    let has_can = match pm {
        Some(pm) => pm.can.is_some(),
        None => !bare_side,
    };
    let under = Frame { o: v3::mad(v3::mad(f.o, f.y, -p.rail.under), f.z, -rail_back), ..f };
    let rail = Seg { l0: p.rail.l0, l1: p.rail.l1, thick: p.rail.thick, tall: p.rail.tall, adj: p.rail.adj };
    if hk > 0.0 {
        part_at(g, out, f, p.slide, mix_adj(p.slide.adj, None, hk), cols.gun);
        part_at(g, out, f, p.grip, mix_adj(p.grip.adj, Some(grip_hung()), hk), pcol);
        if has_can {
            part_at(g, out, f, p.suppressor, mix_adj(p.suppressor.adj, None, hk), ccol);
        }
        if let Some(mag) = pm.and_then(|pm| pm.mag) {
            part_at(g, out, f, of_model(mag), mix_adj(p.grip.adj, Some(grip_hung()), hk), cols.dark_metal);
        }
        if hk < 0.5 {
            seg(g, out, under, rail, cols.torch, None, None);
        }
    } else {
        part(g, out, f, p.slide, cols.gun, None, None);
        part(g, out, f, p.grip, pcol, None, None);
        if has_can {
            part(g, out, f, p.suppressor, ccol, None, None);
        }
        let kit_no_mag = o.kit.and_then(|k| k.no_mag).unwrap_or(false);
        let show_no_mag = pshow.map(|s| s.no_mag).unwrap_or(false);
        if let Some(mag) = pm.and_then(|pm| pm.mag) {
            if !kit_no_mag && !show_no_mag {
                part_at(g, out, f, of_model(mag), p.grip.adj, cols.dark_metal);
            }
        }
        part(g, out, under, rail, cols.torch, None, None);
        if o.light_on {
            let lens = Seg { l0: p.lens.l0, l1: p.lens.l1, thick: p.lens.thick, tall: p.lens.tall, adj: p.lens.adj };
            part(g, out, apply_adj(under, p.rail.adj), lens, cols.lens_body, Some(p.lens.glow), None);
        }
    }
    if let Some(pid) = pid {
        crate::gunreload::reload_hand_boxes(g, c_2, pid, out);
    }
}

/// The pistol group with a model's slide, grip and can in place of its own; each keeps the group part's adjustment.
fn pistol_as(p: crate::rig_props::RigPistol, m: crate::gunlook::PistolModel) -> crate::rig_props::RigPistol {
    RigPistol {
        slide: Seg { adj: p.slide.adj, ..of_model(m.slide) },
        grip: Seg { adj: p.grip.adj, ..of_model(m.grip) },
        suppressor: match m.can {
            Some(can) => Seg { adj: p.suppressor.adj, ..of_model(can) },
            None => p.suppressor,
        },
        ..p
    }
}

/// A model's colour word as a colour.
fn model_col(c_2: crate::gunlook::ModelCol, black: bool) -> V3 {
    use crate::gunlook::ModelCol;
    let cols = col();
    match c_2 {
        ModelCol::Black => cols.anodised,
        ModelCol::Metal => cols.dark_metal,
        ModelCol::Steel => cols.gun,
        ModelCol::Poly => cols.polymer,
        ModelCol::Furn => {
            if black {
                cols.polymer
            } else {
                cols.fde
            }
        }
        ModelCol::Brass => cols.brass,
        ModelCol::Shell => cols.shell,
        ModelCol::Cloth => cols.cloth,
        ModelCol::Torch => cols.torch,
        ModelCol::Lens => cols.lens_body,
    }
}

/// A long gun's model laid out in a group's frame: the lens lit, the magazine left out with `no_mag`, the feed cover's parts swung up `cover`.
pub(super) fn long_model_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>, l: &crate::gunlook::LongModel, g_2: crate::rig_props::Frame, lens: V3, black: bool, no_mag: bool, cover: f64) {
    use crate::gunlook::ModelPartIs;
    let mut cf: Option<Frame> = None;
    if cover > 0.001 {
        if let Some(h) = l.hinge {
            let f = apply_adj(g_2, Some(Adj { off: h, rot: [-COVER_OPEN * cover, 0.0, 0.0] }));
            cf = Some(Frame { o: v3::mad(v3::mad(v3::mad(f.o, f.x, -h[0]), f.y, -h[1]), f.z, -h[2]), ..f });
        }
    }
    for p in l.parts.iter() {
        if no_mag && p.is == Some(ModelPartIs::Mag) {
            continue;
        }
        let f = match cf {
            Some(cf) if p.is == Some(ModelPartIs::Cover) => cf,
            _ => g_2,
        };
        let emissive = if p.is == Some(ModelPartIs::Lens) { lens } else { no_glow() };
        seg(g, out, f, of_model(p.s), model_col(p.col, black), Some(emissive), Some(GUNMODEL_TAG));
    }
}

/// The pistol model drawn for his sidearm: the one in his hand if `held` and it is one, else the one on his thigh.
fn pistol_look_of(g: &crate::game::Game, c_2: CharId, held: bool) -> Option<crate::gunlook::PistolModel> {
    let k = g.ch[c_2].gun_look?;
    if held {
        if let Some(pm) = crate::gunlook::pistol_model(k.hand) {
            return Some(pm);
        }
    }
    crate::gunlook::pistol_model(k.side)
}

/// What a reload under way shows on the model in his hands, and the thing in his support hand.
pub(super) fn model_in_hands(g: &mut crate::game::Game, c_2: CharId, id_2: crate::weapons::GunModelId, l: &crate::gunlook::LongModel, out: &mut Vec<AnyBoxId>, g_2: crate::rig_props::Frame, lens: V3, black: bool, no_mag: bool) {
    let wid = weapon_of(id_2);
    let sh = crate::gunreload::reload_show(g, c_2, wid);
    let show_no_mag = sh.map(|s| s.no_mag).unwrap_or(false);
    let cover = sh.map(|s| s.cover).unwrap_or(0.0);
    long_model_boxes(g, out, l, g_2, lens, black, no_mag || show_no_mag, cover);
    crate::gunreload::reload_hand_boxes(g, c_2, wid, out);
}

/// The sidearm in a thigh holster: the drawn gun's part boxes hung on the leg's frame.
pub fn holster_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<AnyBoxId>, parts: crate::rig_props::HolsterBoxesParts, holster_rig: Option<crate::rig_props::RigHolster>) {
    let th = bone(g, c_2, crate::character::Bone::ThighR);
    let kn = bone(g, c_2, crate::character::Bone::ShinR);
    let hp = bone(g, c_2, crate::character::Bone::Hips);
    let (th, kn, hp) = match (th, kn, hp) {
        (Some(th), Some(kn), Some(hp)) => (th, kn, hp),
        _ => return,
    };
    g.m.rig_props.cur_owner = owner_of(g, c_2);
    let down = v3::normalize(v3::sub(kn, th));
    let fwd = crate::character::character_forward(g, c_2);
    let mut outward = v3::sub(th, hp);
    outward = v3::sub(outward, v3::scale(down, v3::dot(outward, down)));
    outward[1] = 0.0;
    outward = if v3::len(outward) > 1e-3 { v3::normalize(outward) } else { v3::cross(fwd, [0.0, 1.0, 0.0]) };
    let h = holster_rig.unwrap_or_else(|| crate::rig_props::rig().holster);
    let anchor = v3::mad(v3::mad(v3::mad(th, down, h.down), outward, h.out), fwd, h.fwd);
    let f0 = frame_from(anchor, down, Some(fwd));
    let gg = apply_adj(f0, h.adj);
    let hm = pistol_look_of(g, c_2, false);
    let bare = g.ch[c_2].gun_look.and_then(|k| k.bare_side).unwrap_or(false);
    let can = if bare { None } else { parts.suppressor };
    let cols = col();
    if let Some(hm) = hm {
        part_at(g, out, gg, of_model(hm.slide), None, cols.gun);
        part_at(g, out, gg, of_model(hm.grip), Some(grip_hung()), model_col(hm.frame, false));
        if let Some(c) = hm.can {
            let can_col = match hm.can_col {
                Some(k) => model_col(k, false),
                None => cols.can,
            };
            part_at(g, out, gg, of_model(c), None, can_col);
        }
        if let Some(mag) = hm.mag {
            part_at(g, out, gg, of_model(mag), Some(grip_hung()), cols.dark_metal);
        }
        return;
    }
    part_at(g, out, gg, parts.slide, None, cols.gun);
    part_at(g, out, gg, parts.grip, Some(grip_hung()), cols.gun);
    if let Some(c) = can {
        part_at(g, out, gg, c, None, cols.can);
    }
}

/// The combat knife in the right fist: four traced boxes with his owner byte; where it ends up is the hand's doing.
fn knife_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<AnyBoxId>, k: Option<crate::rig_props::RigKnife>) {
    let hand = match bone(g, c_2, crate::character::Bone::HandR) {
        Some(hand) => hand,
        None => return,
    };
    let k = k.unwrap_or_else(|| crate::rig_props::rig().knife);
    g.m.rig_props.cur_owner = owner_of(g, c_2);
    let h = crate::character::character_hand_frame_r(g, c_2, None);
    let o = v3::mad(v3::mad(v3::mad(hand, h.fingers, k.grip.fwd), h.back, -k.grip.palm), h.thumb, k.grip.side);
    let fist = Frame { o, x: v3::cross(h.back, h.thumb), y: h.back, z: h.thumb };
    let f = apply_adj(fist, k.adj);
    let cols = col();
    part(g, out, f, k.handle, cols.knife_grip, None, Some(KNIFE_TAG));
    part(g, out, f, k.guard, cols.blade, None, Some(KNIFE_TAG));
    part(g, out, f, k.blade, cols.blade, None, Some(KNIFE_TAG));
    part(g, out, f, k.edge, cols.edge, None, Some(KNIFE_TAG));
}

/// The knife's sheath on the left chest webbing: the dark body always, and with `full` the knife's handle and guard standing out of its mouth.
fn sheath_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<AnyBoxId>, full: Option<bool>, rig: Option<crate::rig_props::Rig>) {
    let full = full.unwrap_or(true);
    let rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    let f = match sheath_frame(g, c_2, Some(rig.sheath)) {
        Some(f) => f,
        None => return,
    };
    g.m.rig_props.cur_owner = owner_of(g, c_2);
    let cols = col();
    part(g, out, f, rig.sheath.body, cols.sheath, None, Some(SHEATH_TAG));
    if full {
        part(g, out, f, rig.knife.guard, cols.blade, None, Some(SHEATH_TAG));
        part(g, out, f, rig.knife.handle, cols.knife_grip, None, Some(SHEATH_TAG));
    }
}

/// A canister in the fist, the throwable's own body and band.
fn held_can_boxes(g: &mut crate::game::Game, c_2: CharId, kind: crate::game::ThrowKind, out: &mut Vec<AnyBoxId>, k: Option<crate::rig_props::RigCan>, fit: Option<bool>, left: Option<bool>) {
    use crate::game::ThrowKind;
    let (fit, left) = (fit.unwrap_or(false), left.unwrap_or(false));
    let hand = bone(g, c_2, if left { crate::character::Bone::HandL } else { crate::character::Bone::HandR });
    let hand = match hand {
        Some(hand) => hand,
        None => return,
    };
    let k = k.unwrap_or_else(|| crate::rig_props::rig().can);
    let f = crate::character::character_forward(g, c_2);
    let axis = v3::normalize([f[0] * k.tilt, 1.0, f[2] * k.tilt]);
    let mut g0 = frame_from(v3::mad(hand, axis, k.sink), axis, Some(f));
    if fit {
        let (fingers, back, thumb) = if left {
            let hf = crate::character::character_hand_frame_l(g, c_2);
            (hf.fingers, hf.back, hf.thumb)
        } else {
            let hf = crate::character::character_hand_frame_r(g, c_2, None);
            (hf.fingers, hf.back, hf.thumb)
        };
        let fc = gripfit().can;
        let mid = (k.body.l0 + k.body.l1) / 2.0;
        g0 = frame_from(v3::mad(v3::mad(v3::mad(hand, fingers, fc.fwd), back, -fc.palm), thumb, fc.side - mid), thumb, Some(fingers));
    }
    let f1 = apply_adj(g0, k.adj);
    let (body, band, label): (V3, V3, &str) = match kind {
        ThrowKind::Frag => ([0.13, 0.17, 0.08], [0.75, 0.62, 0.1], "canister in the fist (frag)"),
        ThrowKind::Flash => ([0.07, 0.075, 0.08], [0.7, 0.7, 0.66], "canister in the fist (stun)"),
        ThrowKind::Smoke => ([0.25, 0.29, 0.2], [0.6, 0.08, 0.06], "canister in the fist (smoke)"),
    };
    seg(g, out, f1, k.body, body, Some([0.0, 0.0, 0.0]), None);
    seg(g, out, f1, k.band, band, Some([0.0, 0.0, 0.0]), None);
}

/// The can of a throw off the free hand: in the left fist, as a fitted can is in the right.
pub fn free_can_boxes(g: &mut crate::game::Game, c_2: CharId, kind: crate::game::ThrowKind, out: &mut Vec<AnyBoxId>) {
    g.m.rig_props.cur_owner = owner_of(g, c_2);
    held_can_boxes(g, c_2, kind, out, None, Some(true), Some(true));
}

/// The guards' carbine: eleven boxes off the right hand, or a model of its own in the group's frame; `at` is the frame when it is somewhere other than his hands.
pub(super) fn carbine_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<AnyBoxId>, c_3: crate::rig_props::RigCarbine, lens: V3, at: Option<crate::rig_props::Frame>, as_: Option<crate::weapons::GunModelId>) {
    let hand = match bone(g, c_2, crate::character::Bone::HandR) {
        Some(hand) => hand,
        None => return,
    };
    let (hand_f, f) = match at {
        Some(at) => (at, at),
        None => {
            let cf = carbine_frame(g, c_2, hand, c_3);
            (cf.hand_f, cf.g)
        }
    };
    let look = match as_ {
        Some(id) => crate::gunlook::long_model(Some(id)).map(|l| LongLookOf { id, l }),
        None => long_look_of(g, c_2, at.is_none()),
    };
    if let Some(look) = look {
        if at.is_some() {
            long_model_boxes(g, out, &look.l, f, no_glow(), false, false, 0.0);
        } else {
            model_in_hands(g, c_2, look.id, &look.l, out, f, lens, false, false);
        }
        return;
    }
    let cols = col();
    part(g, out, f, c_3.receiver, cols.gun, None, None);
    part(g, out, f, c_3.handguard, cols.furniture, None, None);
    part(g, out, f, c_3.barrel, cols.gun, None, None);
    part(g, out, f, c_3.muzzle, cols.gun, None, None);
    part(g, out, f, c_3.mag, cols.furniture, None, None);
    part(g, out, f, c_3.mag2, cols.furniture, None, None);
    part(g, out, f, c_3.grip, cols.furniture, None, None);
    part(g, out, f, c_3.stock_top, cols.gun, None, None);
    part(g, out, f, c_3.stock_low, cols.gun, None, None);
    part(g, out, f, c_3.light, cols.torch, None, None);
    part(g, out, f, c_3.lens, cols.lens_body, Some(lens), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Frame = Frame { o: [1.25, -0.5, 3.1], x: [0.8, 0.0, -0.6], y: [0.0, 1.0, 0.0], z: [0.6, 0.0, 0.8] };
    const B: Frame = Frame { o: [-0.7, 2.2, 0.4], x: [0.0, 0.6, 0.8], y: [0.0, 0.8, -0.6], z: [-1.0, 0.0, 0.0] };

    /// A FAILURE MEANS: a hand on its way between two grips shears or jumps: the frames between are true frames, each as far along as its k, turning the short way.
    #[test]
    fn a_mix_of_frames_is_a_true_frame_between_the_two() {
        let f = mix_frame(A, B, 0.37);
        for (i, a) in [f.x, f.y, f.z].iter().enumerate() {
            assert!((v3::len(*a) - 1.0).abs() < 1e-9, "axis {}", i);
        }
        assert!(v3::dot(f.x, f.y).abs() < 1e-9 && v3::dot(f.y, f.z).abs() < 1e-9);
        assert!(v3::len(v3::sub(f.o, v3::lerp3(A.o, B.o, 0.37))) < 1e-9);
        // the angle gone is its share of the whole way
        let half = mix_frame(A, B, 0.5);
        assert!((v3::dot(half.x, A.x) - v3::dot(half.x, B.x)).abs() < 1e-9);
        assert!(v3::dot(f.x, A.x) > v3::dot(f.x, B.x), "0.37 of the way lies nearer the first");
    }

    /// at or past either end it hands back that end untouched
    #[test]
    fn mix_frame_ends_are_the_frames_themselves() {
        assert_eq!(mix_frame(A, B, 0.0).o, A.o);
        assert_eq!(mix_frame(A, B, -1.0).z, A.z);
        assert_eq!(mix_frame(A, B, 1.0).o, B.o);
        assert_eq!(mix_frame(A, B, 2.0).z, B.z);
    }

    /// no adjustment is zero; k at or below 0, or none on either side, hands back the first as it came
    #[test]
    fn mix_adj_goes_over_from_none() {
        let a = Adj { off: [1.0, 2.0, 4.0], rot: [8.0, 16.0, 32.0] };
        assert!(mix_adj(None, None, 0.5).is_none());
        assert!(mix_adj(None, Some(a), 0.0).is_none());
        let half = mix_adj(None, Some(a), 0.5).unwrap();
        assert_eq!(half.off, [0.5, 1.0, 2.0]);
        assert_eq!(half.rot, [4.0, 8.0, 16.0]);
        let back = mix_adj(Some(a), None, 0.25).unwrap();
        assert_eq!(back.off, [0.75, 1.5, 3.0]);
    }
}
