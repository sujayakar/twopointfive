//! The finger and thumb joints, last of all.
use super::*;

/// the angle between two rotations, off two tables of doubles
#[inline(always)]
fn q_ang_of(a: &[f64], i: usize, b: &[f64], k: usize) -> f64 {
    let dot = abs(a[i] * b[k] + a[i + 1] * b[k + 1] + a[i + 2] * b[k + 2] + a[i + 3] * b[k + 3]);
    if dot >= 1.0 {
        0.0
    } else {
        2.0 * acos(dot)
    }
}
/// Node i's stored rotation turned on by the share f of a further local rotation, in place
fn q_turn_by(d: &mut [S], i: usize, q: [f64; 4], f: f64) {
    let [mut x, mut y, mut z, mut w] = q;
    if w < 0.0 {
        x = -x;
        y = -y;
        z = -z;
        w = -w;
    }
    let th = acos(min(1.0, w));
    let (mut sc, mut cw) = (f, 1.0);
    if th > 1e-4 {
        sc = sin(f * th) / sin(th);
        cw = cos(f * th);
    }
    x *= sc;
    y *= sc;
    z *= sc;
    let [qx, qy, qz, qw] = own_rot(d, i);
    set_own_rot(d, i, &[qw * x + qx * cw + qy * z - qz * y, qw * y - qx * z + qy * cw + qz * x, qw * z + qx * y - qy * x + qz * cw, qw * cw - qx * x - qy * y - qz * z]);
}
/// Node i's stored rotation turned the share f of the way to the local rotation at t[k‥k+3], in place
fn q_turn_to(d: &mut [S], i: usize, t: &[f64], k: usize, f: f64) {
    let (cx, cy, cz, cw) = (-ch(d, i, 3), -ch(d, i, 4), -ch(d, i, 5), ch(d, i, 6));
    // quat::conj(q) · target
    let q = [
        cw * t[k] + cx * t[k + 3] + cy * t[k + 2] - cz * t[k + 1],
        cw * t[k + 1] - cx * t[k + 2] + cy * t[k + 3] + cz * t[k],
        cw * t[k + 2] + cx * t[k + 1] - cy * t[k] + cz * t[k + 3],
        cw * t[k + 3] - cx * t[k] - cy * t[k + 1] - cz * t[k + 2],
    ];
    q_turn_by(d, i, q, f);
}
/// The hand pass, after every other layer and on the digit joints alone: a hand the library keyed as a fist and no layer has shaped is opened to the
/// man's relaxed hand, by how much of it is the fist, how free the hand is and how little a layer made of it; the thumb of a hand that is not the fist
/// goes to the relaxed thumb; and a drilled man's trigger finger lies straight along the frame until he fires.
pub fn hands_pass(body: &mut Body, an: &mut Anim, dt: f64, full: [f64; 2], gun_r: f64) {
    let (at, held, trained) = (an.p_a.at as usize, an.grip, an.trained);
    let tb = an.hn_flat.as_ref().expect("a man's hands are posed before they were read off his rig");
    let hn = &mut an.hn;
    let (fist, open, own, thumb, trig) = (&tb[..120], &tb[120..240], &tb[240..250], &tb[250..274], &tb[274..]);
    let base: [[f64; 60]; 2] = core::array::from_fn(|sd| core::array::from_fn(|i| hn.base[sd][i] as f64));
    let far = HAND.far * DEGR;
    body.pose(at, |d, rig, _fk| {
        for sd in 0..2 {
            hn.grip[sd] = damp(hn.grip[sd], if held[sd] { 1.0 } else { 0.0 }, 14.0, dt);
            let free = (1.0 - clamp(full[sd], 0.0, 1.0)) * (1.0 - hn.grip[sd]);
            if free <= 0.003 {
                continue;
            }
            let (ns, b, fs, op, nt, ow) = (rig.digits(sd), &base[sd], &fist[sd * 60..sd * 60 + 60], &open[sd * 60..sd * 60 + 60], &hn.note[sd], &own[sd * 5..sd * 5 + 5]);
            for dg in 0..5 {
                let kk = dg * 12;
                let (o0, o1, o2) = (ns[dg * 3] as usize, ns[dg * 3 + 1] as usize, ns[dg * 3 + 2] as usize);
                // how much of the hand under the layers is the library's fist (none of it, crouched: that clip's hand is its own)
                let fist_k = 1.0 - clamp((q_ang_of(b, kk, fs, kk) + q_ang_of(b, kk + 4, fs, kk + 4) + q_ang_of(b, kk + 8, fs, kk + 8)) / 3.0 / (ow[dg] * 0.85), 0.0, 1.0);
                if fist_k <= 0.003 && dg > 0 {
                    continue;
                }
                // …less what a layer has made of it
                let moved = (q_ang(d, o0, b, kk) + q_ang(d, o1, b, kk + 4) + q_ang(d, o2, b, kk + 8)) / 3.0;
                let noted = nt[dg] < f32::INFINITY;
                let dd = if noted { nt[dg] as f64 } else { far };
                let lay = 1.0 - if dd < 0.03 { if noted { 1.0 } else { 0.0 } } else { clamp(moved / dd, 0.0, 1.0) };
                let f = fist_k * free * lay;
                if f > 0.003 {
                    q_turn_by(d, o0, [op[kk], op[kk + 1], op[kk + 2], op[kk + 3]], f);
                    q_turn_by(d, o1, [op[kk + 4], op[kk + 5], op[kk + 6], op[kk + 7]], f);
                    q_turn_by(d, o2, [op[kk + 8], op[kk + 9], op[kk + 10], op[kk + 11]], f);
                }
                if dg == 0 {
                    // the thumb of a hand that is not the fist: to the relaxed hand's thumb
                    let (t, th) = ((1.0 - fist_k) * free * lay * HAND.thumb_own, &thumb[sd * 12..sd * 12 + 12]);
                    if t > 0.003 {
                        q_turn_to(d, o0, th, 0, t);
                        q_turn_to(d, o1, th, 4, t);
                        q_turn_to(d, o2, th, 8, t);
                    }
                }
            }
        }
        // the trigger hand's index, a drilled man's: straight along the frame until he fires, on the trigger while he does and for a moment after
        hn.fired += dt;
        let off_t = if hn.fired < HAND.trig_hold { 0.0 } else { 1.0 };
        hn.off = damp(hn.off, off_t, if off_t < hn.off { 40.0 } else { 9.0 }, dt);
        // (eased: the caller's gate for a throw, a reload or a strike is a step)
        hn.gun = damp(hn.gun, clamp(gun_r, 0.0, 1.0), 18.0, dt);
        let tr = hn.gun * smoothstep(HAND.trig_trained, HAND.trig_trained + 0.1, trained) * hn.off;
        if tr > 0.003 {
            let ns = rig.digits(1);
            for j in 0..3 {
                q_turn_to(d, ns[3 + j] as usize, trig, j * 4, tr);
            }
        }
    })
}
