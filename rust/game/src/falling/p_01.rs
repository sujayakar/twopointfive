//! src/game/falling.ts lines 1 to 46
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The ray a falling thing asks of what it lands on: the `t` of `{ t } | null`. Statics, or the statics and then the props up to the statics' `t`, the props' answer first. (falling.ts FallWorld.raycast; shards.ts:568 g.col; debris.ts:114 solids)
fn ask(g: &mut Game, world: &crate::falling::FallWorld, ro: V3, rd: V3, tmax: f64) -> Option<f64> {
    let st = crate::scene_collision::raycast(g, ro, rd, tmax).map(|h| h.t);
    match world {
        crate::falling::FallWorld::Statics => st,
        crate::falling::FallWorld::StaticsAndProps => {
            let ph = crate::props::prop_system_raycast(g, ro, rd, match st { Some(t) => t, None => tmax }).map(|h| h.t);
            match ph { Some(t) => Some(t), None => st }
        }
    }
}

/// The time a fall from `h` takes at downward speed `vy`. (falling.ts fallOf's tOf)
fn t_of(vy: f64, gravity: f64, h: f64) -> f64 {
    (-vy + m::sqrt(vy * vy + 2.0 * gravity * h)) / gravity
}

/// Where and when something let go at `c` with velocity `v` reaches what is under it, a share `reach_k` of the way sideways. (falling.ts fallOf)
pub fn fall_of(g: &mut crate::game::Game, c_2: V3, v: V3, gravity: f64, reach_k: f64, world: crate::falling::FallWorld) -> crate::falling::FallOf {
    fall_along(&mut |ro, rd, tmax| ask(g, &world, ro, rd, tmax), c_2, v, gravity, reach_k)
}

/// `fall_of`'s sums, over whatever answers the ray.
fn fall_along(ray: &mut dyn FnMut(V3, V3, f64) -> Option<f64>, c_2: V3, v: V3, gravity: f64, reach_k: f64) -> crate::falling::FallOf {
    let c = c_2;
    let vy = -v[1];
    let mut reach = m::hyp2(v[0], v[2]) * t_of(vy, gravity, m::max(0.02, c[1])) * reach_k;
    let d: V3 = if reach > 1e-4 {
        [
            v[0] * reach_k * t_of(vy, gravity, m::max(0.02, c[1])) / reach,
            0.0,
            v[2] * reach_k * t_of(vy, gravity, m::max(0.02, c[1])) / reach,
        ]
    } else {
        [0.0, 0.0, 0.0]
    };
    if reach > 0.05 {
        if let Some(w) = ray([c[0], c[1], c[2]], d, reach + 0.05) {
            reach = m::max(0.0, w - 0.05);
        }
    }
    let x = c[0] + d[0] * reach;
    let z = c[2] + d[2] * reach;
    let hit = ray([x, c[1], z], [0.0, -1.0, 0.0], c[1] + 0.5);
    let h = m::max(0.02, match hit { Some(t) => t, None => c[1] });
    crate::falling::FallOf { t: t_of(vy, gravity, h), pos: [x, c[1] - h, z] }
}

/// The same question asked along the arc, for a thing thrown: when and where it comes to lie. (falling.ts flightOf)
pub fn flight_of(g: &mut crate::game::Game, c_2: V3, v: V3, gravity: f64, world: crate::falling::FallWorld) -> crate::falling::FlightOf {
    flight_along(&mut |ro, rd, tmax| ask(g, &world, ro, rd, tmax), c_2, v, gravity)
}

/// `flight_of`'s sums, over whatever answers the ray.
fn flight_along(ray: &mut dyn FnMut(V3, V3, f64) -> Option<f64>, c_2: V3, v: V3, gravity: f64) -> crate::falling::FlightOf {
    let c = c_2;
    let up = v[1];
    let t_all = (up + m::sqrt(up * up + 2.0 * gravity * m::max(0.02, c[1]))) / gravity;
    let (mut px, mut py, mut pz, mut at) = (c[0], c[1], c[2], 0.0);
    let mut k = 1.0;
    while k <= CHORDS {
        let tk = t_all * k / CHORDS;
        let qx = c[0] + v[0] * tk;
        let qy = m::max(0.02, c[1] + up * tk - 0.5 * gravity * tk * tk);
        let qz = c[2] + v[2] * tk;
        let (dx, dy, dz) = (qx - px, qy - py, qz - pz);
        let len = m::sqrt(dx * dx + dy * dy + dz * dz);
        if len > 1e-6 {
            if let Some(hit) = ray([px, py, pz], [dx / len, dy / len, dz / len], len + 0.05) {
                let s = m::max(0.0, hit - 0.05) / len;
                let (x, y, z) = (px + dx * s, py + dy * s, pz + dz * s);
                let under = ray([x, y, z], [0.0, -1.0, 0.0], y + 0.5);
                let h = m::max(0.0, match under { Some(t) => t, None => y });
                return crate::falling::FlightOf {
                    t: at + (tk - at) * m::min(1.0, hit / len) + m::sqrt(2.0 * h / gravity),
                    pos: [x, y - h, z],
                };
            }
        }
        px = qx;
        py = qy;
        pz = qz;
        at = tk;
        k += 1.0;
    }
    crate::falling::FlightOf { t: t_all, pos: [px, 0.0, pz] }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The test's world, the same sums as the script that printed the bits: a wall at x = 2, a floor at y = 0 that stops at x = 10.
    fn ray(ro: V3, rd: V3, tmax: f64) -> Option<f64> {
        let mut best = f64::INFINITY;
        if rd[0] > 0.0 && ro[0] <= 2.0 {
            let t = (2.0 - ro[0]) / rd[0];
            if t < best { best = t; }
        }
        if rd[1] < 0.0 && ro[1] >= 0.0 {
            let t = ro[1] / -rd[1];
            if ro[0] + rd[0] * t < 10.0 && t < best { best = t; }
        }
        if best <= tmax { Some(best) } else { None }
    }

    /// c, v, reach_k, then fallOf's t and pos, flightOf's t and pos: the TypeScript's own doubles under bun, EXACT on.
    const TS: &[([u64; 3], [u64; 3], u64, u64, [u64; 3], u64, [u64; 3])] = &[
    ([0x0000000000000000, 0x4008cccccccccccd, 0x0000000000000000], [0x0000000000000000, 0x0000000000000000, 0x0000000000000000], 0x3fe999999999999a, 0x3fe9708d9f30bdb9, [0x0000000000000000, 0x0000000000000000, 0x0000000000000000], 0x3fecaba5f6ab08fa, [0x0000000000000000, 0x0000000000000000, 0x0000000000000000]),
    ([0x3fd3333333333333, 0x3ff5eb851eb851ec, 0xbfe6666666666666], [0x3f847ae147ae147b, 0xbfc999999999999a, 0x3f847ae147ae147b], 0x3fe999999999999a, 0x3fe045a30cf2e227, [0x3fd375d9a6185dbb, 0x0000000000000000, 0xbfe645132cf3d122], 0x3fe380bae0beef86, [0x3fd3856de7bd1bb4, 0x0000000000000000, 0xbfe63d490c217226]),
    ([0x3fd3333333333333, 0x3ff5eb851eb851ec, 0xbfe6666666666666], [0x3ff4cccccccccccd, 0x3fd999999999999a, 0xc000cccccccccccd], 0x3fe999999999999a, 0x3fe24452a7ca686a, [0x3fec98f9cd38e778, 0x0000000000000000, 0xbffa8b4fabaa05ca], 0x3fe54b2e7cb3dde8, [0x3ff0904ae484e0ed, 0x0000000000000000, 0xbffe33eb32217b19]),
    ([0x3ff3333333333333, 0x3ff5eb851eb851ec, 0x3fd0000000000000], [0x4012cccccccccccd, 0x3feccccccccccccd, 0x3ff4cccccccccccd], 0x3fe999999999999a, 0x3fe419bcff923fa8, [0x3fff3a9c8a1f308e, 0x0000000000000000, 0x3fdd4f0229b36490], 0x3fe663b6f46093ac, [0x3fff3c4adffd84fb, 0x0000000000000000, 0x3fdd50de4757ac30]),
    ([0x3ffe666666666666, 0x3feccccccccccccd, 0x0000000000000000], [0x403f4ccccccccccd, 0x3fb999999999999a, 0x3fc999999999999a], 0x3ff0000000000000, 0x3fdc13227d836b40, [0x3fff3334453127d6, 0x0000000000000000, 0x3f34f03e09474da7], 0x3fdb9cde5a5cddd4, [0x3fff33360f444be5, 0x0000000000000000, 0x3f34f06cde405b7a]),
    ([0x4023666666666666, 0x400199999999999a, 0x0000000000000000], [0x8000000000000000, 0xbff199999999999a, 0x0000000000000000], 0x3fe0000000000000, 0x3fe2241f2febceb2, [0x4023666666666666, 0x0000000000000000, 0x0000000000000000], 0x3fe55f37876619f3, [0x4023666666666666, 0x0000000000000000, 0x0000000000000000]),
    ([0x4025000000000000, 0x400199999999999a, 0x3ff0000000000000], [0x3fd999999999999a, 0x3ff199999999999a, 0x3fd3333333333333], 0x3fe999999999999a, 0x3fe9514481144185, [0x402581a0170fc3df, 0x0000000000000000, 0x3ff309c08a5e9739], 0x3fe9514481144185, [0x4025a2081cd3b4d7, 0x0000000000000000, 0x3ff3cc30acf63d07]),
    ([0x3fb999999999999a, 0x3f747ae147ae147b, 0x3fb999999999999a], [0x3fc999999999999a, 0x0000000000000000, 0x3fd3333333333333], 0x3fe999999999999a, 0x3fb058ce90af57d9, [0x3fbc372b4a597eb3, 0xbf8eb851eb851eb8, 0x3fbd85f422b9713f], 0x3fb058ce90af57d9, [0x3fbcde8fb68977f9, 0x0000000000000000, 0x3fbe810ac5016728]),
    ([0xc008000000000000, 0x3ff199999999999a, 0x0000000000000000], [0x3ff199999999999a, 0x400599999999999a, 0x3fe3333333333333], 0x3fe999999999999a, 0x3fea55b6879b08d5, [0xc00234d2b934eeb2, 0x0000000000000000, 0x3fd9480b63761cf5], 0x3fed81dc2639364a, [0xc000cffb403d8008, 0x0000000000000000, 0x3fdf5d2bfef3a2c3]),
    ([0x3fe0000000000000, 0x3ff3333333333333, 0x3fe0000000000000], [0x401f99999999999a, 0x400a666666666666, 0xbff3333333333333], 0x3fe999999999999a, 0x3fede7edaeaaa01e, [0x3fff3585c7f8dfd5, 0x0000000000000000, 0x3fd1e5f601a31d44], 0x3fe87be784fb1c3a, [0x3fff371d570eb59e, 0x0000000000000000, 0x3fd1e4fe602ae926]),
    ([0x3ff8000000000000, 0x3fd999999999999a, 0x0000000000000000], [0x4023cccccccccccd, 0xc010666666666666, 0x3fb999999999999a], 0x3fe999999999999a, 0x3fb69737567d4329, [0x3fff3335dfdcb05a, 0x0000000000000000, 0x3f729e4814548760], 0x3fd03221005373dc, [0x3fff46504ccdbc51, 0x0000000000000000, 0x3f72cfae073ff67b]),
    ([0x3fc999999999999a, 0x3f947ae147ae147b, 0x3fc999999999999a], [0x0000000000000000, 0x0000000000000000, 0x0000000000000000], 0x3fe999999999999a, 0x3fb058ce90af57d9, [0x3fc999999999999a, 0x0000000000000000, 0x3fc999999999999a], 0x3fb058ce90af57d9, [0x3fc999999999999a, 0x0000000000000000, 0x3fc999999999999a]),
    ([0x4025000000000000, 0x3ff3333333333333, 0x0000000000000000], [0x3fe6666666666666, 0x3feccccccccccccd, 0x3fc999999999999a], 0x3fe999999999999a, 0x3fe30897b39a91a7, [0x4025aa8a6df75475, 0x0000000000000000, 0x3fb85ceb23553552], 0x3fe30897b39a91a7, [0x4025d52d09752992, 0x0000000000000000, 0x3fbe7425ec2a82a5]),
    ([0x3fc999999999999a, 0x3ff999999999999a, 0x3fd999999999999a], [0x3fd3333333333333, 0x3fb999999999999a, 0xbfc999999999999a], 0x3fe999999999999a, 0x3fe29b021494647b, [0x3fd5bb0b3d140746, 0x0000000000000000, 0x3fd3a56ff96a1d49], 0x3fe5d4ff536083be, [0x3fd7d8c7afed4a32, 0x0000000000000000, 0x3fd23c47acd94602]),
    ];

    fn v3_of(a: [u64; 3]) -> V3 { [f64::from_bits(a[0]), f64::from_bits(a[1]), f64::from_bits(a[2])] }
    fn bits(a: V3) -> [u64; 3] { [a[0].to_bits(), a[1].to_bits(), a[2].to_bits()] }

    #[test]
    fn the_typescript_s_own_bits() {
        for (i, &(c, v, k, ft, fp, lt, lp)) in TS.iter().enumerate() {
            let a = fall_along(&mut ray, v3_of(c), v3_of(v), 9.81, f64::from_bits(k));
            assert_eq!((a.t.to_bits(), bits(a.pos)), (ft, fp), "fall_of, case {}", i);
            let b = flight_along(&mut ray, v3_of(c), v3_of(v), 9.81);
            assert_eq!((b.t.to_bits(), bits(b.pos)), (lt, lp), "flight_of, case {}", i);
        }
    }
}
