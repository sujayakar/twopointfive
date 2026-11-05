//! src/game/shardModel.ts lines 1 to 158
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The kernel's hash, one word to one word: integer ops only. (shardModel.ts pcg)
pub fn pcg(v: f64) -> f64 {
    let s = js::to_u32(m::imul(js::to_i32(v), 747796405) as f64 + 2891336453.0);
    let w = m::imul(((s >> ((s >> 28) + 4)) ^ s) as i32, 277803737) as u32;
    ((w >> 22) ^ w) as f64
}

/// The `k`th hashed draw in 0 to 1 of a seed. (shardModel.ts rnd)
pub fn rnd(seed: f64, k: f64) -> f64 {
    let v = (js::to_i32(seed) ^ m::imul(js::to_i32(k), 0x9E3779B9u32 as i32)) as u32;
    (js::to_u32(pcg(v as f64)) >> 8) as f64 / 16777216.0
}

/// The drag rate of a fragment `size` metres across, 1/s. (shardModel.ts shardDrag)
pub fn shard_drag(size: f64, k: Option<crate::shard_model::SHARD>) -> f64 {
    let k = k.unwrap_or_else(shard);
    m::min(k.drag_max, k.drag * k.ref_size / m::max(size, 0.0015))
}

/// One step of free flight, no collision. (shardModel.ts shardAir)
pub fn shard_air(s: &mut crate::shard_model::ShardState, dt: f64, k: Option<crate::shard_model::SHARD>) {
    let k = k.unwrap_or_else(shard);
    if s.asleep || dt <= 0.0 { return; }
    s.v[1] -= k.gravity * dt;
    let kd = m::exp(-shard_drag(s.size, Some(k)) * dt);
    s.v = [s.v[0] * kd, s.v[1] * kd, s.v[2] * kd];
    let ka = m::exp(-k.ang_damp * dt);
    s.w = [s.w[0] * ka, s.w[1] * ka, s.w[2] * ka];
    let c = v3::cross(s.w, s.n);
    s.n = v3::normalize([s.n[0] + c[0] * dt, s.n[1] + c[1] * dt, s.n[2] + c[2] * dt]);
}

/// A touch on a surface of unit normal `nrm`, the fragment already at the contact. True when it went to sleep. (shardModel.ts shardTouch)
pub fn shard_touch(s: &mut crate::shard_model::ShardState, nrm: V3, k: Option<crate::shard_model::SHARD>) -> bool {
    let k = k.unwrap_or_else(shard);
    s.touches += 1.0;
    let hs = (js::to_i32(s.seed) ^ m::imul(js::to_i32(s.touches), 0x85EBCA6Bu32 as i32)) as u32 as f64;
    let (h0, h1, h2, h3) = (rnd(hs, 41.0), rnd(hs, 42.0), rnd(hs, 43.0), rnd(hs, 44.0));
    let vn = v3::dot(s.v, nrm);
    let floor = nrm[1] > 0.6;
    if vn < 0.0 {
        let mut vt: V3 = [s.v[0] - nrm[0] * vn, s.v[1] - nrm[1] * vn, s.v[2] - nrm[2] * vn];
        if floor {
            let e = k.restitution * (1.0 - 0.5 * k.e_jit + k.e_jit * h0);
            let a = (h1 * 2.0 - 1.0) * k.skitter;
            let (ca, sa) = (m::cos(a), m::sin(a));
            let cr = v3::cross(nrm, vt); // vt turned by a about the surface normal
            vt = [vt[0] * ca + cr[0] * sa, vt[1] * ca + cr[1] * sa, vt[2] * ca + cr[2] * sa];
            s.v = [vt[0] * k.keep - nrm[0] * vn * e, vt[1] * k.keep - nrm[1] * vn * e, vt[2] * k.keep - nrm[2] * vn * e];
        } else {
            s.v = [vt[0] * k.wall_keep - nrm[0] * vn * k.wall_e, vt[1] * k.wall_keep - nrm[1] * vn * k.wall_e, vt[2] * k.wall_keep - nrm[2] * vn * k.wall_e];
        }
    }
    let sp = v3::len(s.v);
    if floor && sp < k.sleep_v {
        let tilt = k.rest_tilt * h2 * h2;
        let az = h3 * 6.2831853;
        let side: V3 = v3::normalize(v3::cross(nrm, if m::abs(nrm[1]) > 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] }));
        let side2 = v3::cross(nrm, side);
        let (caz, saz) = (m::cos(az), m::sin(az));
        let t: V3 = [side[0] * caz + side2[0] * saz, side[1] * caz + side2[1] * saz, side[2] * caz + side2[2] * saz];
        let (ct, st) = (m::cos(tilt), m::sin(tilt));
        s.n = v3::normalize([nrm[0] * ct + t[0] * st, nrm[1] * ct + t[1] * st, nrm[2] * ct + t[2] * st]);
        s.v = [0.0, 0.0, 0.0];
        s.w = [0.0, 0.0, 0.0];
        s.asleep = true;
        return true;
    }
    if floor {
        // dealt a fresh tumble about a hashed in-plane axis
        let az = h2 * 6.2831853;
        let u = v3::normalize(v3::cross(s.n, if m::abs(s.n[1]) > 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] }));
        let w2 = v3::cross(s.n, u);
        let spin = m::min(k.spin_max, k.spin_k * sp) * (0.4 + 0.6 * h3);
        let (caz, saz) = (m::cos(az), m::sin(az));
        s.w = [(u[0] * caz + w2[0] * saz) * spin, (u[1] * caz + w2[1] * saz) * spin, (u[2] * caz + w2[2] * saz) * spin];
    } else {
        s.w = v3::scale(s.w, 0.5);
    }
    false
}

/// Fly one shard to rest over a floor and, optionally, an upright wall, at a fixed step. The probes' and the trajectory sheet's flight. (shardModel.ts flyShard)
pub fn fly_shard(s: &mut crate::shard_model::ShardState, opts: Option<crate::shard_model::FlyShardOpts>, k: Option<crate::shard_model::SHARD>) -> crate::shard_model::FlyShard {
    let k = k.unwrap_or_else(shard);
    let opts = opts.unwrap_or_default();
    let (dt, max_t, floor_y, every) = (opts.dt.unwrap_or(1.0 / 120.0), opts.max_t.unwrap_or(12.0), opts.floor_y.unwrap_or(0.0), opts.every.unwrap_or(2.0));
    let mut path: Vec<V3> = vec![v3::copy(s.p)];
    let p0 = v3::copy(s.p);
    let (mut rest_t, mut hops, mut max_h, mut i) = (-1.0, 0.0, s.p[1], 0.0);
    let mut t = 0.0;
    while t < max_t && !s.asleep {
        shard_air(s, dt, Some(k));
        let mut np: V3 = [s.p[0] + s.v[0] * dt, s.p[1] + s.v[1] * dt, s.p[2] + s.v[2] * dt];
        if let Some(wall) = opts.wall {
            let d1 = v3::dot(np, wall.n) - wall.d;
            if d1 > 0.0 && v3::dot(s.v, wall.n) > 0.0 {
                np = [np[0] - wall.n[0] * d1, np[1] - wall.n[1] * d1, np[2] - wall.n[2] * d1];
                s.p = np;
                shard_touch(s, v3::scale(wall.n, -1.0), Some(k));
                np = s.p;
            }
        }
        if np[1] <= floor_y && s.v[1] < 0.0 {
            np[1] = floor_y;
            s.p = np;
            hops += 1.0;
            if shard_touch(s, [0.0, 1.0, 0.0], Some(k)) { rest_t = t + dt; }
        } else {
            s.p = np;
        }
        s.age += dt;
        max_h = m::max(max_h, s.p[1]);
        if js::rem(i, every) == 0.0 || s.asleep { path.push(v3::copy(s.p)); }
        t += dt;
        i += 1.0;
    }
    crate::shard_model::FlyShard { path, rest_t, hops, max_h, reach: m::hyp2(s.p[0] - p0[0], s.p[2] - p0[2]) }
}

/// A shard in the state the emit kernel would deal one: speed along `dir` inside a cone, tumbling. (shardModel.ts makeShard)
pub fn make_shard(seed: f64, p: V3, dir: V3, speed: f64, spread: f64, size: f64, tumble: f64) -> crate::shard_model::ShardState {
    let dn = v3::normalize(dir);
    let u = v3::normalize(v3::cross(if m::abs(dn[1]) > 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] }, dn));
    let w = v3::cross(dn, u);
    let cy = 1.0 - rnd(seed, 1.0) * (1.0 - m::cos(spread));
    let sr = m::sqrt(m::max(0.0, 1.0 - cy * cy));
    let th = rnd(seed, 2.0) * 6.2831853;
    let (cth, sth) = (m::cos(th), m::sin(th));
    let d: V3 = [dn[0] * cy + u[0] * cth * sr + w[0] * sth * sr, dn[1] * cy + u[1] * cth * sr + w[1] * sth * sr, dn[2] * cy + u[2] * cth * sr + w[2] * sth * sr];
    let wa = rnd(seed, 15.0) * 6.2831853;
    let spin = tumble * (0.3 + 0.7 * rnd(seed, 16.0));
    let (cwa, swa) = (m::cos(wa), m::sin(wa));
    crate::shard_model::ShardState {
        p: v3::copy(p),
        v: v3::scale(d, speed),
        n: dn,
        w: [(u[0] * cwa + w[0] * swa) * spin, (u[1] * cwa + w[1] * swa) * spin, (u[2] * cwa + w[2] * swa) * spin],
        size,
        asleep: false,
        age: 0.0,
        touches: 0.0,
        seed: js::to_u32(seed) as f64,
    }
}

/// The shards one emit record deals, by the emit kernel's own draws. The probes' and the trajectory sheet's births. (shardModel.ts sprayShards)
pub fn spray_shards(r: crate::shard_model::ShardSpray) -> Vec<crate::shard_model::ShardState> {
    let mut out: Vec<crate::shard_model::ShardState> = Vec::new();
    let dn = v3::normalize(r.dir);
    let mut j = 0.0;
    while j < r.count {
        let stir = js::to_u32(m::imul(js::to_i32(j), 0x85EBCA6Bu32 as i32) as f64 + 1.0);
        let seed = pcg((js::to_i32(r.seed) ^ stir as i32) as u32 as f64);
        let sp = r.speed * (1.0 - r.speed_jit * rnd(seed, 3.0));
        let size = m::max(0.0005, r.size * (1.0 - r.size_jit + 2.0 * r.size_jit * rnd(seed, 10.0)));
        let mut s = make_shard(seed, r.pos, r.dir, sp, r.spread, size, r.tumble);
        if let Some(vadd) = r.vadd { s.v = v3::add(s.v, vadd); }
        if r.pos_jit > 0.0 {
            let mut jit: V3 = [(rnd(seed, 6.0) - 0.5) * 2.0 * r.pos_jit, (rnd(seed, 7.0) - 0.5) * 2.0 * r.pos_jit, (rnd(seed, 8.0) - 0.5) * 2.0 * r.pos_jit];
            let d = v3::dot(jit, dn) * 0.9;
            jit = [jit[0] - dn[0] * d, jit[1] - dn[1] * d, jit[2] - dn[2] * d];
            s.p = v3::add(s.p, jit);
        }
        out.push(s);
        j += 1.0;
    }
    out
}

#[cfg(test)]
mod tests {
    //! Against the TypeScript's own bits: tools run under bun with EXACT.on, each double printed as hex; a flight's path is folded by FNV over the bits.
    use super::*;
    use crate::shard_model::{FlyShardOpts, FlyShardOptsWall, ShardSpray, ShardState};

    fn hex(x: f64) -> String { format!("{:016x}", x.to_bits()) }
    fn vh(v: V3) -> String { format!("{},{},{}", hex(v[0]), hex(v[1]), hex(v[2])) }
    fn state(s: &ShardState) -> String {
        format!("{},{},{},{},{},{},{},{},{}", vh(s.p), vh(s.v), vh(s.n), vh(s.w), hex(s.size), if s.asleep { "T" } else { "F" }, hex(s.age), hex(s.touches), hex(s.seed))
    }
    fn fold(path: &[V3]) -> String {
        let mut h = 0xcbf29ce484222325u64;
        for p in path { for x in p { h = (h ^ x.to_bits()).wrapping_mul(0x100000001b3); } }
        format!("{:016x}", h)
    }
    fn list(v: &[f64]) -> String { v.iter().map(|x| hex(*x)).collect::<Vec<_>>().join(",") }
    fn touch(seed: f64, nrm: V3, vel: V3) -> String {
        let mut s = make_shard(seed, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0, 0.5, 0.01, 20.0);
        s.v = vel;
        let r = shard_touch(&mut s, nrm, None);
        format!("{},{}", if r { "T" } else { "F" }, state(&s))
    }
    fn rec() -> ShardSpray {
        ShardSpray { pos: [0.0, 1.5, 0.0], dir: [0.0, 0.0, 1.0], count: 40.0, speed: 2.8, speed_jit: 0.75, spread: 0.55, size: 0.008, size_jit: 0.55, tumble: 22.0, pos_jit: 0.1, vadd: Some([0.0, -0.2, 0.4]), seed: 20949.0 }
    }
    fn fly(opts: Option<FlyShardOpts>) -> String {
        let mut rows = Vec::new();
        for (i, mut s) in spray_shards(rec()).into_iter().enumerate() {
            let r = fly_shard(&mut s, opts, None);
            if i % 13 != 0 { continue; }
            rows.push(format!("{},{},{},{},{},{},{}", fold(&r.path), r.path.len(), hex(r.rest_t), hex(r.hops), hex(r.max_h), hex(r.reach), state(&s)));
        }
        rows.join(",")
    }

    const PCG: &str = "419eecbf88000000,41e517dd47800000,41df99c487000000,41df9a9ffec00000,41ecc54920400000,41ecc54920400000,41c0e39323000000,41e517dd47800000";
    const RND: &str = "3f9eecbc00000000,3fb5911300000000,3fd26cd340000000,3fedf5fdc0000000,3fc5262300000000,3fe32d48e0000000";
    const DRAG: &str = "3fe0000000000000,3ffaaaaaaaaaaaab,400aaaaaaaaaaaab,3fa9999999999999";
    const AIR: &str = "0000000000000000,3ff8000000000000,0000000000000000,3ff31559df024b51,bfeb2b32061f7423,4005ef873d9626d7,3fd2d0f50af70218,3fc25518705ffc20,3fee3d8b97419519,403014df971029f2,402812f78918c432,c01b4f4764b56413,3f847ae147ae147b,F,0000000000000000,0000000000000000,4053400000000000";
    const AIRASLEEP: &str = "0000000000000000,3ff0000000000000,0000000000000000,bfabe6469bfb5dc4,3f8c6f523af36075,3feff309b53766bb,0000000000000000,0000000000000000,3ff0000000000000,3fd79733b3232e11,bfe5eb364929aa6b,0000000000000000,3f847ae147ae147b,T,0000000000000000,0000000000000000,401c000000000000";
    const MAKE: &str = "3ff0000000000000,4000000000000000,4008000000000000,3ff49c8b7256dda6,40011bb2d8b3b1da,bfc0c9fcd9d1458c,0000000000000000,3ff0000000000000,0000000000000000,400aa7e69556e574,0000000000000000,c01d6679b696c941,3f80624dd2f1a9fc,F,0000000000000000,0000000000000000,40d4754000000000";
    const TOUCHFLOOR: &str = "F,0000000000000000,0000000000000000,0000000000000000,3fe5fc5b343bba80,3fe87ad0a3d70a3e,3fe12fc2d09a065f,0000000000000000,0000000000000000,3ff0000000000000,40357d1c15f3ace8,4030e2ebe29ed5c9,8000000000000000,3f847ae147ae147b,F,0000000000000000,3ff0000000000000,4026000000000000";
    const TOUCHWALL: &str = "F,0000000000000000,0000000000000000,0000000000000000,3fb47ae147ae147c,bfc47ae147ae147c,bfdccccccccccccc,0000000000000000,0000000000000000,3ff0000000000000,c01e0880a58448ca,40119181b0fe25fe,0000000000000000,3f847ae147ae147b,F,0000000000000000,3ff0000000000000,4028000000000000";
    const TOUCHREST: &str = "T,0000000000000000,0000000000000000,0000000000000000,0000000000000000,0000000000000000,0000000000000000,bfd1ead7346795b8,3fee8614a4496bd4,3fbbc3182aebe1fc,0000000000000000,0000000000000000,0000000000000000,3f847ae147ae147b,T,0000000000000000,3ff0000000000000,402a000000000000";
    const SPRAY0: &str = "3fada1d666666667,3ff752485ccccccd,bf1b0228f5c28f58,3fee4a05620a0b4d,3fd72cbc5bbd5a3f,40052985b63063f5,0000000000000000,0000000000000000,3ff0000000000000,400e15052968f85b,c032f87f1bca8983,0000000000000000,3f76204487fcb923,F,0000000000000000,0000000000000000,41c7e0f84f000000";
    const SPRAY39: &str = "bfa86aa600000000,3ff88a24a999999a,3f70568d70a3d708,bfe8de8cc27316fe,bfe2826e75f7c978,3fff298778de5164,0000000000000000,0000000000000000,3ff0000000000000,c02e6bf061e62f22,c01a37a8a0459cd7,8000000000000000,3f817b56f6944674,F,0000000000000000,0000000000000000,41e24f7977c00000";
    const FLY: &str = "dda7bd4d629305a4,61,3fefbbbbbbbbbbb2,4018000000000000,3ff767547ef837c0,3ffab3c26f09f434,3fe3892d5bc50eed,0000000000000000,3ff931b26bf3770c,0000000000000000,0000000000000000,0000000000000000,bf7546b532454212,3feffabd14fe9221,3fa227497a36cd90,0000000000000000,0000000000000000,0000000000000000,3f76204487fcb923,T,3fefbbbbbbbbbbb2,4018000000000000,41c7e0f84f000000,86299bb2b103b292,79,3ff488888888887f,401c000000000000,3ff66a7ebdf077a8,4000597ae2397ff5,3fe05c74e9c08d5b,0000000000000000,4000015483c117f5,0000000000000000,0000000000000000,0000000000000000,bfc3711e31be773d,3fef618dff2a3f36,3fbf9a0559d58723,0000000000000000,0000000000000000,0000000000000000,3f83f7652a305533,T,3ff488888888887f,401c000000000000,41a88526fe000000,0c95cebb5e3f40f8,67,3ff155555555554f,4010000000000000,3ff89e828999999a,3fed33c15ea0863e,3f82b024dcad6818,0000000000000000,3fed4e1497378936,0000000000000000,0000000000000000,0000000000000000,bf8c6bc3268a7895,3fefff35e6728b86,3f38feb64f18f298,0000000000000000,0000000000000000,0000000000000000,3f76efe1b71758e1,T,3ff155555555554f,4010000000000000,41eb532897800000,0efbd993d825cd97,77,3ff4222222222219,4014000000000000,3ff88a24a999999a,3ff9479498f32303,bfe048ca1dc97846,0000000000000000,3ff83e124776993c,0000000000000000,0000000000000000,0000000000000000,bfd36619009a1f00,3fee72c4d0a16a51,3faae037ce364371,0000000000000000,0000000000000000,0000000000000000,3f817b56f6944674,T,3ff4222222222219,4014000000000000,41e24f7977c00000";
    const FLYWALL: &str = "7f833a923230740b,119,3ff7555555555544,4010000000000000,3ff76a1aead7ae67,3fec9668c8513697,3fe63124251dfb66,bfd0000000000000,3fe415b8fc6dd994,0000000000000000,0000000000000000,0000000000000000,3f8f2f108d25cf77,3feffef6a1eb2aac,3f72dd667a4a4add,0000000000000000,0000000000000000,0000000000000000,3f76204487fcb923,T,3ff7555555555544,4014000000000000,41c7e0f84f000000,70da424ee9df343a,105,3ff499999999998b,4010000000000000,3ff66b4a62c9b5b2,3fed4652731cf1c3,3fe943dfae056e20,bfd0000000000000,3fe2efccedff6ff1,0000000000000000,0000000000000000,0000000000000000,bfc0e2b0d3551370,3fef3cf4006c2926,3fc60a5772ff6452,0000000000000000,0000000000000000,0000000000000000,3f83f7652a305533,T,3ff499999999998b,4014000000000000,41a88526fe000000,3f9136e1c6f3a9b9,90,3ff188888888887d,4008000000000000,3ff89e828999999a,3fec44d71ac92f37,3f8508ee555dd835,bfd0000000000000,3fec5ef6f6e2044b,0000000000000000,0000000000000000,0000000000000000,bf8c6bc3268a7895,3fefff35e6728b86,3f38feb64f18f298,0000000000000000,0000000000000000,0000000000000000,3f76efe1b71758e1,T,3ff188888888887d,4010000000000000,41eb532897800000,f7c33e3f064d5d27,96,3ff2bbbbbbbbbbaf,4008000000000000,3ff88a24a999999a,3feefad93a4fd8f9,bfe40b4d21aa3c33,bfd0000000000000,3fe8f6b8860244e7,0000000000000000,0000000000000000,0000000000000000,bfafb1c675b00803,3fefeec6914aa032,bf93a9d2a8d9b3d8,0000000000000000,0000000000000000,0000000000000000,3f817b56f6944674,T,3ff2bbbbbbbbbbaf,4010000000000000,41e24f7977c00000";

    #[test]
    fn the_hash_is_the_kernel_s() {
        assert_eq!(list(&[0.0, 1.0, 7.0, 20949.0, 4294967295.0, -1.0, 2147483648.0, 1.5].map(pcg)), PCG);
        assert_eq!(list(&[(0.0, 0.0), (1.0, 1.0), (20949.0, 41.0), (4294967295.0, 16.0), (99.0, 3.5), (-5.0, 7.0)].map(|(a, b)| rnd(a, b))), RND);
    }
    #[test]
    fn drag_and_air() {
        assert_eq!(list(&[0.01, 0.003, 0.0005, 0.1].map(|s| shard_drag(s, None))), DRAG);
        let mut a = make_shard(77.0, [0.0, 1.5, 0.0], [0.2, 0.3, 1.0], 3.1, 0.6, 0.01, 22.0);
        shard_air(&mut a, 1.0 / 120.0, None);
        assert_eq!(state(&a), AIR);
        let mut z = make_shard(7.0, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 1.0, 0.1, 0.01, 1.0);
        z.asleep = true;
        shard_air(&mut z, 1.0 / 60.0, None);
        assert_eq!(state(&z), AIRASLEEP);
    }
    #[test]
    fn a_shard_is_made_as_the_kernel_makes_it() {
        assert_eq!(state(&make_shard(20949.0, [1.0, 2.0, 3.0], [0.0, 1.0, 0.0], 2.5, 0.55, 0.008, 22.0)), MAKE);
        let sp = spray_shards(rec());
        assert_eq!(sp.len(), 40);
        assert_eq!(state(&sp[0]), SPRAY0);
        assert_eq!(state(&sp[39]), SPRAY39);
    }
    #[test]
    fn touches() {
        assert_eq!(touch(11.0, [0.0, 1.0, 0.0], [1.0, -2.0, 0.5]), TOUCHFLOOR);
        assert_eq!(touch(12.0, [0.0, 0.0, -1.0], [0.1, -0.2, 1.5]), TOUCHWALL);
        assert_eq!(touch(13.0, [0.0, 1.0, 0.0], [0.05, -0.1, 0.02]), TOUCHREST);
    }
    #[test]
    fn flights_to_rest() {
        assert_eq!(fly(None), FLY);
        let o = FlyShardOpts { dt: Some(1.0 / 240.0), max_t: Some(15.0), floor_y: Some(-0.25), every: Some(3.0), wall: Some(FlyShardOptsWall { n: [0.0, 0.0, 1.0], d: 0.9 }) };
        assert_eq!(fly(Some(o)), FLYWALL);
    }
}
