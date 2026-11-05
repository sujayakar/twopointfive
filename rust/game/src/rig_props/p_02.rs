//! src/game/rigProps.ts lines 419 to 839
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The hand frame the pistol group lives in: wrist origin, z the bore, y the top of the slide.   (rigProps.ts pistolFrame)
pub fn pistol_frame(g: &mut crate::game::Game, c_2: CharId, hand: V3, p: crate::rig_props::PistolFrameP, in_hand: Option<bool>) -> crate::rig_props::PistolFrame {
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

/// The frame `k` of the way from a to b: origins lerped, axes slerped.   (rigProps.ts mixFrame)
pub fn mix_frame(a: crate::rig_props::Frame, b: crate::rig_props::Frame, k: f64) -> crate::rig_props::Frame {
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

/// The thigh holster's group frame off three joints and the body's forward, in whatever one space they are given.   (rigProps.ts holsterFrameOf)
pub fn holster_frame_of(th: V3, kn: V3, hp: V3, fwd: V3, h: Option<crate::rig_props::RigHolster>) -> crate::rig_props::Frame {
    let h = h.unwrap_or_else(|| crate::rig_props::rig().holster);
    let down = v3::normalize(v3::sub(kn, th));
    let mut outward = v3::sub(th, hp);
    outward = v3::sub(outward, v3::scale(down, v3::dot(outward, down)));
    outward[1] = 0.0;
    outward = if v3::len(outward) > 1e-3 { v3::normalize(outward) } else { v3::cross(fwd, [0.0, 1.0, 0.0]) };
    apply_adj(frame_from(v3::mad(v3::mad(v3::mad(th, down, h.down), outward, h.out), fwd, h.fwd), down, Some(fwd)), h.adj)
}

/// The long model drawn for a gun in his hands, or for the one that is not (slung, hung); none = the group's own parts.   (rigProps.ts longLookOf)
pub fn long_look_of(g: &mut crate::game::Game, c_2: CharId, held: bool) -> Option<crate::rig_props::LongLookOf> {
    let k = g.ch[c_2].gun_look?;
    let a = if held { k.hand } else { k.long };
    if let Some(l) = crate::gunlook::long_model(g, a) {
        return Some(LongLookOf { id: a.unwrap(), l });
    }
    if !held {
        return None;
    }
    // (the frames of a swap: the long gun is still drawn in the hand that is letting go of it)
    crate::gunlook::long_model(g, k.long).map(|l| LongLookOf { id: k.long.unwrap(), l })
}

/// A long model's support-hand grip for the animator; `fit` moves it with the gun as drawn, and a fist round a post into the hook of the fingers.   (rigProps.ts modelGrip)
pub fn model_grip(g: &mut crate::game::Game, id_2: crate::weapons::GunModelId, fit: Option<bool>, rig: Option<crate::rig_props::Rig>) -> Option<crate::rig_props::ModelGrip> {
    let fit = fit.unwrap_or(false);
    let rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    let l = crate::gunlook::long_model(g, Some(id_2))?;
    let gr = long_grip_of(rig.sc20k.adj, LongGripOfG { off: l.grip_l.off, fingers: l.grip_l.fingers, back: l.grip_l.back });
    if !fit {
        return Some(ModelGrip { wrist: gr.wrist, fingers: gr.fingers, back: gr.back });
    }
    let t = gripfit();
    let (d, f) = (t.long, t.fore);
    if !l.post {
        return Some(ModelGrip { wrist: v3::add(gr.wrist, d), fingers: gr.fingers, back: gr.back });
    }
    Some(ModelGrip {
        wrist: v3::mad(v3::mad(v3::mad(v3::add(gr.wrist, d), gr.fingers, -f.back), gr.back, -f.lift), v3::normalize(v3::cross(gr.fingers, gr.back)), -f.down),
        fingers: gr.fingers,
        back: gr.back,
    })
}

/// How far ahead of the wrist along the bore a long model's muzzle is.   (rigProps.ts modelMuzzleAhead)
pub fn model_muzzle_ahead(g: &mut crate::game::Game, id_2: crate::weapons::GunModelId, rig: Option<crate::rig_props::Rig>) -> Option<f64> {
    let rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    let l = crate::gunlook::long_model(g, Some(id_2))?;
    Some(muzzle_ahead_of(rig.sc20k.adj, Seg { l0: 0.0, l1: l.ahead, thick: 0.0, tall: 0.0, adj: None }))
}

/// A vector in the chest's axes [left, up, fwd] as a world one (sheathFrame's `W`).
fn sheath_w(c: &crate::character::CharacterChestFrame, v: V3) -> V3 {
    v3::mad(v3::mad(v3::scale(c.left, v[0]), c.up, v[1]), c.fwd, v[2])
}

/// The sheath's frame in the world this frame: origin the sheathed handle's grip, z handle to tip, x off the chest. None before his first update.   (rigProps.ts sheathFrame)
pub fn sheath_frame(g: &mut crate::game::Game, c_2: CharId, s: Option<crate::rig_props::RigSheath>) -> Option<crate::rig_props::Frame> {
    let s = s.unwrap_or_else(|| crate::rig_props::rig().sheath);
    let c = crate::character::character_chest_frame(g, c_2)?;
    let z = v3::normalize(sheath_w(&c, s.along));
    let x0 = sheath_w(&c, s.out);
    let x = v3::normalize(v3::mad(x0, z, -v3::dot(x0, z)));
    Some(Frame { o: v3::add(c.o, sheath_w(&c, s.at)), x, y: v3::cross(z, x), z })
}

/// The carbine's group frame: the sidearm's hand frame moved by the group adj onto the bore above the trigger.   (rigProps.ts carbineFrame)
pub fn carbine_frame(g: &mut crate::game::Game, c_2: CharId, hand: V3, c_3: crate::rig_props::RigCarbine) -> crate::rig_props::CarbineFrame {
    let f = pistol_frame(g, c_2, hand, PistolFrameP { adj: c_3.adj }, None);
    CarbineFrame { hand_f: f.hand_f, g: f.g }
}

/// What the animator needs of the carbine to put the support hand on the gun.   (rigProps.ts carbineGrip)
pub fn carbine_grip(rig: Option<crate::rig_props::Rig>) -> crate::rig_props::CarbineGrip {
    let rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    let gl = rig.carbine.grip_l;
    let gr = long_grip_of(rig.carbine.adj, LongGripOfG { off: gl.off, fingers: gl.fingers, back: gl.back });
    CarbineGrip { wrist: gr.wrist, fingers: gr.fingers, back: gr.back }
}

/// How far ahead of the wrist along the bore the carbine's muzzle is.   (rigProps.ts carbineMuzzleAhead)
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

/// A long gun's support-hand grip (gripL through the group adj) in the trigger hand's gun frame.   (rigProps.ts longGripOf)
pub fn long_grip_of(group_adj: Option<crate::rig_props::Adj>, g_2: crate::rig_props::LongGripOfG) -> crate::rig_props::LongGripOf {
    let f = apply_adj(unit_frame(), group_adj);
    LongGripOf {
        wrist: v3::mad(v3::mad(v3::mad(f.o, f.x, g_2.off[0]), f.y, g_2.off[1]), f.z, g_2.off[2]),
        fingers: dir_in(&f, g_2.fingers),
        back: dir_in(&f, g_2.back),
    }
}

/// Metres ahead of the wrist along the bore to the far end of a long gun's foremost part.   (rigProps.ts muzzleAheadOf)
pub fn muzzle_ahead_of(group_adj: Option<crate::rig_props::Adj>, part: crate::rig_props::Seg) -> f64 {
    let f = apply_adj(apply_adj(unit_frame(), group_adj), part.adj);
    v3::mad(f.o, f.z, part.l1)[2]
}

/// The right hand that holds the rifle as it hangs in front, in the torso's axes off the mid-back joint.   (rigProps.ts slingFrontHand)
pub fn sling_front_hand(g: &mut crate::game::Game, s: Option<crate::rig_props::RigSc20k>, rest: Option<f64>) -> crate::rig_props::SlingFrontHand {
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
fn bone(g: &mut crate::game::Game, c: CharId, b: crate::character::Bone) -> Option<V3> {
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

/// A node of the viewer's tree in the meta sink, if there is one.
fn node(meta: &mut Option<&mut Vec<RigNodeMeta>>, path: &str, label: &str, depth: f64, parent: Frame, frame: Frame, box_start: usize, box_count: f64) {
    if let Some(m) = meta.as_deref_mut() {
        m.push(RigNodeMeta { path: String::from(path), label: String::from(label), depth, parent, frame, box_start: box_start as f64, box_count });
    }
}

/// `seg(out, f, s, albedo, emissive, meta, path, label, 2, name)`: a named part of a group in the viewer's tree.
fn part(g: &mut crate::game::Game, out: &mut Vec<BoxId>, f: Frame, s: &mut Seg, albedo: V3, emissive: Option<V3>, meta: &mut Option<&mut Vec<RigNodeMeta>>, path: &str, label: &str, name: Option<&str>) {
    seg(g, out, f, s, albedo, emissive, meta.as_deref_mut(), Some(path), Some(label), Some(2.0), name);
}

/// `segAt(out, f, s, adj, albedo, NO_GLOW)`: an unlit part with its adjustment handed in, no node of its own.
fn part_at(g: &mut crate::game::Game, out: &mut Vec<BoxId>, f: Frame, s: Seg, adj: Option<Adj>, albedo: V3) {
    let mut s = s;
    seg_at(g, out, f, &mut s, adj, albedo, no_glow(), None, None, None, None, None);
}

/// A gun group's frame moved by a GRIPFIT shift, in its own axes.   (rigProps.ts fitFrame)
pub fn fit_frame(g_2: crate::rig_props::Frame, d: V3) -> crate::rig_props::Frame {
    Frame { o: v3::mad(v3::mad(v3::mad(g_2.o, g_2.x, d[0]), g_2.y, d[1]), g_2.z, d[2]), ..g_2 }
}

/// Two adjustments mixed, none being zero: a part going over from how it sits in one frame to how it sits in another.   (rigProps.ts mixAdj)
pub fn mix_adj(a: Option<crate::rig_props::Adj>, b: Option<crate::rig_props::Adj>, k: f64) -> Option<crate::rig_props::Adj> {
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

/// The gun group's frame as the hand itself has it (bore wrist to knuckles, top the thumb side), `roll` of the way from the usual frame.   (rigProps.ts kitGunFrame)
pub fn kit_gun_frame(g: &mut crate::game::Game, c_2: CharId, hand: V3, p: crate::rig_props::KitGunFrameP, g_2: crate::rig_props::Frame, roll: f64) -> crate::rig_props::Frame {
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

/// A magazine in the support hand: one dark box up out of the fist; `own` / `col` give a gun's own feed its extents and colour.   (rigProps.ts magBoxL)
pub fn mag_box_l(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, kind: crate::slotsetup::SetupGun, fit: Option<bool>, own: Option<V3>, col: Option<V3>) {
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
        let made = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
            c: v3::mad(v3::mad(v3::mad(a, hf.fingers, f.fwd), hf.back, -f.palm), hf.thumb, f.side),
            h,
            yaw: Some(m::atan2(hf.fingers[0], hf.fingers[2])),
            rot: Some(v3::quat::from_basis(x, hf.thumb, hf.fingers)),
            albedo: Some(albedo),
            flags: Some(PROP_FLAGS),
            owner: Some(owner),
            ..Default::default()
        });
        out.push(made);
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
    let made = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: v3::mad(v3::mad(a, z, 0.085), y, h[1] * 0.35),
        h,
        yaw: Some(m::atan2(z[0], z[2])),
        rot: Some(v3::quat::from_basis(x, y, z)),
        albedo: Some(albedo),
        flags: Some(PROP_FLAGS),
        owner: Some(owner),
        ..Default::default()
    });
    out.push(made);
}

/// Held and worn props for one character as small boxes, appended to `out`. Call after his update: it needs his bones.   (rigProps.ts rigBoxes)
pub fn rig_boxes(g: &mut crate::game::Game, c_2: CharId, o: crate::rig_props::RigOpts, out: &mut Vec<BoxId>, rig: Option<crate::rig_props::Rig>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) {
    let rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    let mut meta = meta;
    kit_boxes(g, c_2, o, out, rig, meta.as_deref_mut());
    if let Some(big) = o.big {
        let kind = match big {
            RigOptsBig::Shotgun => BigBoxesKind::Shotgun,
            RigOptsBig::Hammer => BigBoxesKind::Hammer,
        };
        big_boxes(g, c_2, kind, o.big_k.unwrap_or(0.0), out, Some(rig), Some(o.big_clear.unwrap_or(false)), meta);
    }
}

/// His kit, the big slot's tool apart: Sam's three slots, or a guard's torch, pistol or long gun.   (rigProps.ts kitBoxes)
pub fn kit_boxes(g: &mut crate::game::Game, c_2: CharId, o: crate::rig_props::RigOpts, out: &mut Vec<BoxId>, rig: crate::rig_props::Rig, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) {
    let mut meta = meta;
    let hand = bone(g, c_2, crate::character::Bone::HandR);
    g.m.rig_props.cur_owner = owner_of(g, c_2);
    let cols = col();
    if !o.is_guard {
        if let Some(hand) = hand.filter(|_| o.slot == 1.0) {
            player_pistol(g, c_2, o, out, rig, hand, &mut meta);
        }
        if o.slot != 1.0 {
            if o.holster == Some(true) {
                let parts = HolsterBoxesParts { slide: rig.pistol.slide, grip: rig.pistol.grip, suppressor: Some(rig.pistol.suppressor) };
                holster_boxes(g, c_2, out, parts, meta.as_deref_mut(), Some(rig.holster));
            }
            if let Some(held) = o.held {
                let mut can = rig.can;
                held_can_boxes(g, c_2, held, out, Some(&mut can), meta.as_deref_mut(), o.fit, None);
            }
        }
        let knife = o.knife.unwrap_or(false);
        if knife {
            let mut k = rig.knife;
            knife_boxes(g, c_2, out, Some(&mut k), meta.as_deref_mut());
        }
        if o.sheath.unwrap_or(false) {
            sheath_boxes(g, c_2, out, Some(!knife), Some(rig), meta.as_deref_mut());
        }
        if o.rifle == Some(RigOptsRifle::Hand) && hand.is_some() {
            let lens = if o.rifle_light.unwrap_or(false) { v3::scale(rail_lens_color(), rig.sc20k.lens_glow) } else { [0.0, 0.0, 0.0] };
            let m4 = rifle_m4(g, Some(rig));
            let mut s = rig.sc20k;
            sc20k_boxes(g, c_2, out, Some(&mut s), Some(Sc20kBoxesWhere::Hand), Some(lens), meta.as_deref_mut(), m4, o.kit, o.fit, Some(false), None, o.rifle_black);
        } else if o.rifle == Some(RigOptsRifle::Slung) {
            let m4 = rifle_m4(g, Some(rig));
            let mut s = rig.sc20k;
            sc20k_boxes(g, c_2, out, Some(&mut s), Some(Sc20kBoxesWhere::Slung), Some([0.0, 0.0, 0.0]), meta.as_deref_mut(), m4, None, Some(false), o.sling_front, o.sling_rest, o.rifle_black);
        }
        if let Some(mag_l) = o.kit.and_then(|k| k.mag_l) {
            mag_box_l(g, c_2, out, mag_l, o.fit, None, None);
        }
        if o.goggles {
            let nv = match o.nv_k {
                Some(k) => k > 0.5,
                None => o.nv,
            };
            let mut gg = rig.goggles;
            goggles_boxes(g, c_2, nv, out, &mut gg, meta.as_deref_mut(), o.nv_k);
        }
        return;
    }
    let hand = match hand {
        Some(hand) => hand,
        None => return,
    };
    let hr = match o.holster_out {
        Some(x) if js::truthy(x) => Some(RigHolster { out: rig.holster.out + x, ..rig.holster }),
        _ => None,
    };
    let side = HolsterBoxesParts { slide: rig.guard_pistol.slide, grip: rig.guard_pistol.grip, suppressor: None };
    let wears = o.holster != Some(false);
    let hang = o.hang.unwrap_or(false);
    let lit = |glow: f64| -> V3 {
        match (o.light_on, o.lens_color) {
            (true, Some(c)) => v3::scale(c, glow),
            _ => [0.0, 0.0, 0.0],
        }
    };
    if o.weapon == Some(RigOptsWeapon::Carbine) || o.weapon == Some(RigOptsWeapon::Sc20k) {
        let sc20k = o.weapon == Some(RigOptsWeapon::Sc20k);
        if hang {
            hung_boxes(g, c_2, if sc20k { HungBoxesWeapon::Sc20k } else { HungBoxesWeapon::Carbine }, out, Some(rig), Some(false), meta.as_deref_mut());
        } else if sc20k {
            let m4 = rifle_m4(g, Some(rig));
            let mut s = rig.sc20k;
            sc20k_boxes(g, c_2, out, Some(&mut s), Some(Sc20kBoxesWhere::Hand), Some(lit(rig.sc20k.lens_glow)), meta.as_deref_mut(), m4, None, None, None, None, None);
        } else {
            let mut carbine = rig.carbine;
            carbine_boxes(g, c_2, out, &mut carbine, lit(rig.carbine.lens_glow), meta.as_deref_mut(), None, None);
        }
        if wears {
            holster_boxes(g, c_2, out, side, meta.as_deref_mut(), hr);
        }
        return;
    }
    if hang {
        if wears {
            holster_boxes(g, c_2, out, side, meta.as_deref_mut(), hr);
        }
        return;
    }
    let lens = lit(rig.torch.lens_glow);
    let alive = g.ch[c_2].alive;
    if o.torch == Some(true) && alive {
        let along = o.beam_dir.unwrap_or(g.ch[c_2].gun_dir);
        let hand_f = frame_from(hand, along, None);
        let f = apply_adj(hand_f, rig.torch.adj);
        node(&mut meta, "torch", "hand torch (group + light)", 1.0, hand_f, f, out.len(), 2.0);
        let mut t = rig.torch;
        part(g, out, f, &mut t.body, cols.torch, None, &mut meta, "torch.body", "body", None);
        part(g, out, f, &mut t.lens, cols.lens_body, Some(lens), &mut meta, "torch.lens", "lens", None);
        if wears {
            holster_boxes(g, c_2, out, side, meta.as_deref_mut(), hr);
        }
    } else {
        let in_hand = o.in_hand.unwrap_or(false);
        let pf = pistol_frame(g, c_2, hand, PistolFrameP { adj: rig.guard_pistol.adj }, Some(in_hand));
        let hand_f = pf.hand_f;
        let f = if in_hand { fit_frame(pf.g, gripfit().guard_pistol) } else { pf.g };
        node(&mut meta, "guardPistol", "guard pistol (group + light)", 1.0, hand_f, f, out.len(), if alive { 3.0 } else { 2.0 });
        let gm = pistol_look_of(g, c_2, true);
        let mut gp = rig.guard_pistol;
        if let Some(gm) = gm {
            let (mut slide, mut grip) = (of_model(gm.slide), of_model(gm.grip));
            seg_at(g, out, f, &mut slide, gp.slide.adj, cols.gun, no_glow(), meta.as_deref_mut(), Some("guardPistol.slide"), Some("slide"), Some(2.0), None);
            seg_at(g, out, f, &mut grip, gp.grip.adj, model_col(gm.frame, false), no_glow(), meta.as_deref_mut(), Some("guardPistol.grip"), Some("grip"), Some(2.0), None);
            if let Some(can) = gm.can {
                let can_col = match gm.can_col {
                    Some(c) => model_col(c, false),
                    None => cols.can,
                };
                part_at(g, out, f, of_model(can), gp.slide.adj, can_col);
            }
            if let Some(mag) = gm.mag {
                part_at(g, out, f, of_model(mag), gp.grip.adj, cols.dark_metal);
            }
        } else {
            part(g, out, f, &mut gp.slide, cols.gun, None, &mut meta, "guardPistol.slide", "slide", None);
            part(g, out, f, &mut gp.grip, cols.gun, None, &mut meta, "guardPistol.grip", "grip", None);
        }
        if alive {
            part(g, out, f, &mut gp.lens, cols.lens_body, Some(lens), &mut meta, "guardPistol.lens", "weapon light lens", None);
        }
    }
}

/// Sam's pistol in his right hand (kitBoxes' first branch): a closure's worth of locals, so a function here.
fn player_pistol(g: &mut crate::game::Game, c_2: CharId, o: RigOpts, out: &mut Vec<BoxId>, rig: Rig, hand: V3, meta: &mut Option<&mut Vec<RigNodeMeta>>) {
    let cols = col();
    let pm = pistol_look_of(g, c_2, true);
    let mut p = match pm {
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
    let hand_f = pf.hand_f;
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
    let mut rail = Seg { l0: p.rail.l0, l1: p.rail.l1, thick: p.rail.thick, tall: p.rail.tall, adj: p.rail.adj };
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
            seg(g, out, under, &mut rail, cols.torch, None, None, None, None, None, None);
        }
    } else {
        let box_count = (if o.light_on { 5.0 } else { 4.0 }) - (if pm.is_none() && bare_side { 1.0 } else { 0.0 });
        node(meta, "pistol", "pistol (group + rail light)", 1.0, hand_f, f, out.len(), box_count);
        part(g, out, f, &mut p.slide, cols.gun, None, meta, "pistol.slide", "slide + frame", None);
        part(g, out, f, &mut p.grip, pcol, None, meta, "pistol.grip", "grip", None);
        if has_can {
            part(g, out, f, &mut p.suppressor, ccol, None, meta, "pistol.suppressor", "suppressor", None);
        }
        let kit_no_mag = o.kit.and_then(|k| k.no_mag).unwrap_or(false);
        let show_no_mag = pshow.map(|s| s.no_mag).unwrap_or(false);
        if let Some(mag) = pm.and_then(|pm| pm.mag) {
            if !kit_no_mag && !show_no_mag {
                part_at(g, out, f, of_model(mag), p.grip.adj, cols.dark_metal);
            }
        }
        part(g, out, under, &mut rail, cols.torch, None, meta, "pistol.rail", "rail light body", None);
        if o.light_on {
            let mut lens = Seg { l0: p.lens.l0, l1: p.lens.l1, thick: p.lens.thick, tall: p.lens.tall, adj: p.lens.adj };
            part(g, out, apply_adj(under, p.rail.adj), &mut lens, cols.lens_body, Some(p.lens.glow), meta, "pistol.lens", "rail light lens (lit)", None);
        }
    }
    if let Some(pid) = pid {
        crate::gunreload::reload_hand_boxes(g, c_2, pid, out);
    }
}

/// The pistol group with a model's slide, grip and can in place of its own; each keeps the group part's adjustment.   (rigProps.ts pistolAs)
pub fn pistol_as(p: crate::rig_props::RigPistol, m: crate::gunlook::PistolModel) -> crate::rig_props::RigPistol {
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

/// A model's colour word as a colour.   (rigProps.ts modelCol)
pub fn model_col(c_2: crate::gunlook::ModelCol, black: bool) -> V3 {
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

/// A long gun's model laid out in a group's frame: the lens lit, the magazine left out with `no_mag`, the feed cover's parts swung up `cover`.   (rigProps.ts longModelBoxes)
pub fn long_model_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>, l: &crate::gunlook::LongModel, g_2: crate::rig_props::Frame, lens: V3, black: bool, no_mag: bool, cover: f64) {
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
        let mut s = of_model(p.s);
        seg(g, out, f, &mut s, model_col(p.col, black), Some(emissive), None, Some(""), Some(""), Some(0.0), Some(GUNMODEL_TAG));
    }
}

/// The pistol model drawn for his sidearm: the one in his hand if `held` and it is one, else the one on his thigh.   (rigProps.ts pistolLookOf)
pub fn pistol_look_of(g: &mut crate::game::Game, c_2: CharId, held: bool) -> Option<crate::gunlook::PistolModel> {
    let k = g.ch[c_2].gun_look?;
    if held {
        if let Some(pm) = crate::gunlook::pistol_model(g, k.hand) {
            return Some(pm);
        }
    }
    crate::gunlook::pistol_model(g, k.side)
}

/// What a reload under way shows on the model in his hands, and the thing in his support hand.   (rigProps.ts modelInHands)
pub fn model_in_hands(g: &mut crate::game::Game, c_2: CharId, id_2: crate::weapons::GunModelId, l: &crate::gunlook::LongModel, out: &mut Vec<BoxId>, g_2: crate::rig_props::Frame, lens: V3, black: bool, no_mag: bool) {
    let wid = weapon_of(id_2);
    let sh = crate::gunreload::reload_show(g, c_2, wid);
    let show_no_mag = sh.map(|s| s.no_mag).unwrap_or(false);
    let cover = sh.map(|s| s.cover).unwrap_or(0.0);
    long_model_boxes(g, out, l, g_2, lens, black, no_mag || show_no_mag, cover);
    crate::gunreload::reload_hand_boxes(g, c_2, wid, out);
}

/// The sidearm in a thigh holster: the drawn gun's part boxes hung on the leg's frame.   (rigProps.ts holsterBoxes)
pub fn holster_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, parts: crate::rig_props::HolsterBoxesParts, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, holster_rig: Option<crate::rig_props::RigHolster>) {
    let mut meta = meta;
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
    let threes = match hm {
        Some(hm) => hm.can.is_some(),
        None => can.is_some(),
    };
    node(&mut meta, "holster", "holstered sidearm (thigh)", 1.0, f0, gg, out.len(), if threes { 3.0 } else { 2.0 });
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

/// The combat knife in the right fist: four traced boxes with his owner byte; where it ends up is the hand's doing.   (rigProps.ts knifeBoxes)
pub fn knife_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, k: Option<&mut crate::rig_props::RigKnife>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) {
    let mut meta = meta;
    let hand = match bone(g, c_2, crate::character::Bone::HandR) {
        Some(hand) => hand,
        None => return,
    };
    let mut own = crate::rig_props::rig().knife;
    let k: &mut RigKnife = match k {
        Some(k) => k,
        None => &mut own,
    };
    g.m.rig_props.cur_owner = owner_of(g, c_2);
    let h = crate::character::character_hand_frame_r(g, c_2, None);
    let o = v3::mad(v3::mad(v3::mad(hand, h.fingers, k.grip.fwd), h.back, -k.grip.palm), h.thumb, k.grip.side);
    let fist = Frame { o, x: v3::cross(h.back, h.thumb), y: h.back, z: h.thumb };
    let f = apply_adj(fist, k.adj);
    node(&mut meta, "knife", "combat knife (right fist)", 1.0, fist, f, out.len(), 4.0);
    let cols = col();
    part(g, out, f, &mut k.handle, cols.knife_grip, None, &mut meta, "knife.handle", "handle", Some(KNIFE_TAG));
    part(g, out, f, &mut k.guard, cols.blade, None, &mut meta, "knife.guard", "guard", Some(KNIFE_TAG));
    part(g, out, f, &mut k.blade, cols.blade, None, &mut meta, "knife.blade", "blade", Some(KNIFE_TAG));
    part(g, out, f, &mut k.edge, cols.edge, None, &mut meta, "knife.edge", "edge strip (lighter)", Some(KNIFE_TAG));
}

/// The knife's sheath on the left chest webbing: the dark body always, and with `full` the knife's handle and guard standing out of its mouth.   (rigProps.ts sheathBoxes)
pub fn sheath_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, full: Option<bool>, rig: Option<crate::rig_props::Rig>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) {
    let mut meta = meta;
    let full = full.unwrap_or(true);
    let mut rig = rig.unwrap_or_else(|| crate::rig_props::rig());
    let f = match sheath_frame(g, c_2, Some(rig.sheath)) {
        Some(f) => f,
        None => return,
    };
    g.m.rig_props.cur_owner = owner_of(g, c_2);
    node(&mut meta, "sheath", "knife sheath (left chest)", 1.0, f, f, out.len(), if full { 3.0 } else { 1.0 });
    let cols = col();
    part(g, out, f, &mut rig.sheath.body, cols.sheath, None, &mut meta, "sheath.body", "sheath", Some(SHEATH_TAG));
    if full {
        part(g, out, f, &mut rig.knife.guard, cols.blade, None, &mut meta, "sheath.guard", "guard (sheathed)", Some(SHEATH_TAG));
        part(g, out, f, &mut rig.knife.handle, cols.knife_grip, None, &mut meta, "sheath.handle", "handle (sheathed)", Some(SHEATH_TAG));
    }
}

/// A canister in the fist, the throwable's own body and band.   (rigProps.ts heldCanBoxes)
pub fn held_can_boxes(g: &mut crate::game::Game, c_2: CharId, kind: crate::game::ThrowKind, out: &mut Vec<BoxId>, k: Option<&mut crate::rig_props::RigCan>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, fit: Option<bool>, left: Option<bool>) {
    use crate::game::ThrowKind;
    let mut meta = meta;
    let (fit, left) = (fit.unwrap_or(false), left.unwrap_or(false));
    let hand = bone(g, c_2, if left { crate::character::Bone::HandL } else { crate::character::Bone::HandR });
    let hand = match hand {
        Some(hand) => hand,
        None => return,
    };
    let mut own = crate::rig_props::rig().can;
    let k: &mut RigCan = match k {
        Some(k) => k,
        None => &mut own,
    };
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
    node(&mut meta, "can", label, 1.0, g0, f1, out.len(), 2.0);
    seg(g, out, f1, &mut k.body, body, Some([0.0, 0.0, 0.0]), meta.as_deref_mut(), Some("can.body"), None, None, None);
    seg(g, out, f1, &mut k.band, band, Some([0.0, 0.0, 0.0]), meta.as_deref_mut(), Some("can.band"), None, None, None);
}

/// The can of a throw off the free hand: in the left fist, as a fitted can is in the right.   (rigProps.ts freeCanBoxes)
pub fn free_can_boxes(g: &mut crate::game::Game, c_2: CharId, kind: crate::game::ThrowKind, out: &mut Vec<BoxId>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) {
    g.m.rig_props.cur_owner = owner_of(g, c_2);
    held_can_boxes(g, c_2, kind, out, None, meta, Some(true), Some(true));
}

/// The guards' carbine: eleven boxes off the right hand, or a model of its own in the group's frame; `at` is the frame when it is somewhere other than his hands.   (rigProps.ts carbineBoxes)
pub fn carbine_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, c_3: &mut crate::rig_props::RigCarbine, lens: V3, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, at: Option<crate::rig_props::Frame>, as_: Option<crate::weapons::GunModelId>) {
    let mut meta = meta;
    let hand = match bone(g, c_2, crate::character::Bone::HandR) {
        Some(hand) => hand,
        None => return,
    };
    let (hand_f, f) = match at {
        Some(at) => (at, at),
        None => {
            let cf = carbine_frame(g, c_2, hand, *c_3);
            (cf.hand_f, cf.g)
        }
    };
    let look = match as_ {
        Some(id) => crate::gunlook::long_model(g, Some(id)).map(|l| LongLookOf { id, l }),
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
    node(&mut meta, "carbine", "carbine (group + weapon light)", 1.0, hand_f, f, out.len(), 11.0);
    part(g, out, f, &mut c_3.receiver, cols.gun, None, &mut meta, "carbine.receiver", "receiver + cover", None);
    part(g, out, f, &mut c_3.handguard, cols.furniture, None, &mut meta, "carbine.handguard", "handguard (wood)", None);
    part(g, out, f, &mut c_3.barrel, cols.gun, None, &mut meta, "carbine.barrel", "barrel", None);
    part(g, out, f, &mut c_3.muzzle, cols.gun, None, &mut meta, "carbine.muzzle", "muzzle booster", None);
    part(g, out, f, &mut c_3.mag, cols.furniture, None, &mut meta, "carbine.mag", "magazine (upper)", None);
    part(g, out, f, &mut c_3.mag2, cols.furniture, None, &mut meta, "carbine.mag2", "magazine (lower, the curve)", None);
    part(g, out, f, &mut c_3.grip, cols.furniture, None, &mut meta, "carbine.grip", "pistol grip", None);
    part(g, out, f, &mut c_3.stock_top, cols.gun, None, &mut meta, "carbine.stockTop", "stock, top strut", None);
    part(g, out, f, &mut c_3.stock_low, cols.gun, None, &mut meta, "carbine.stockLow", "stock, lower strut", None);
    part(g, out, f, &mut c_3.light, cols.torch, None, &mut meta, "carbine.light", "weapon light body", None);
    part(g, out, f, &mut c_3.lens, cols.lens_body, Some(lens), &mut meta, "carbine.lens", "weapon light lens", None);
    if meta.is_some() {
        let gl = c_3.grip_l;
        let o = v3::mad(v3::mad(v3::mad(f.o, f.x, gl.off[0]), f.y, gl.off[1]), f.z, gl.off[2]);
        node(&mut meta, "carbine.gripL", "support hand grip point (arrows only)", 2.0, f, Frame { o, ..f }, out.len(), 0.0);
    }
}

/// The grip point as an Adj for the viewer's gizmo: off is gripL's off, rot is ignored.   (rigProps.ts gripAdj)
pub fn grip_adj(g: &mut crate::game::Game, g_2: crate::rig_props::RigCarbineGripL) -> crate::rig_props::Adj {
    Adj { off: g_2.off, rot: [0.0, 0.0, 0.0] }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Frame = Frame { o: [1.25, -0.5, 3.1], x: [0.8, 0.0, -0.6], y: [0.0, 1.0, 0.0], z: [0.6, 0.0, 0.8] };
    const B: Frame = Frame { o: [-0.7, 2.2, 0.4], x: [0.0, 0.6, 0.8], y: [0.0, 0.8, -0.6], z: [-1.0, 0.0, 0.0] };

    /// the TypeScript's own bits, printed by bun with EXACT on: mixFrame(A, B, 0.37)
    #[test]
    fn mix_frame_matches_the_typescript_bits() {
        let f = mix_frame(A, B, 0.37);
        let bits = |v: V3| [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()];
        assert_eq!(bits(f.o), MID_O);
        assert_eq!(bits(f.x), MID_X);
        assert_eq!(bits(f.y), MID_Y);
        assert_eq!(bits(f.z), MID_Z);
    }

    /// at or past either end it hands back that end untouched
    #[test]
    fn mix_frame_ends_are_the_frames_themselves() {
        assert_eq!(mix_frame(A, B, 0.0).o, A.o);
        assert_eq!(mix_frame(A, B, -1.0).z, A.z);
        assert_eq!(mix_frame(A, B, 1.0).o, B.o);
        assert_eq!(mix_frame(A, B, 2.0).z, B.z);
    }

    const MID_O: [u64; 3] = [0x3fe0e978d4fdf3b6, 0x3fdfef9db22d0e56, 0x4000ced916872b02];
    const MID_X: [u64; 3] = [0x3fef03932a5d9b0e, 0x3fcb4485f28fb63e, 0x3fbfa97058d4b1af];
    const MID_Y: [u64; 3] = [0xbfc784c8591cd392, 0x3feeb527da2a8ef6, 0xbfcb4485f28fb63e];
    const MID_Z: [u64; 3] = [0xbfc50018c3c01cfe, 0x3fc784c8591cd392, 0x3fef03932a5d9b0e];
}
