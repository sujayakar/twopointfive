//! src/game/fogofwar.ts lines 1 to 355
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The field every eye has this frame, degrees in plan: the cone's rows under FOG.cone, else fovDeg.   (fogofwar.ts fieldDeg)
pub fn field_deg(g: &mut crate::game::Game) -> f64 {
    if g.k.fog.cone { m::min(g.k.fog.fov_deg, g.k.fog.cone_deg) } else { g.k.fog.fov_deg }
}

/// Its all-round near field, metres.   (fogofwar.ts fieldNear)
pub fn field_near(g: &mut crate::game::Game) -> f64 {
    if g.k.fog.cone { g.k.fog.cone_near_m } else { g.k.fog.near_m }
}

/// Where a man looks, for the cone (unit, in plan): his head, or his aim with a long gun in the shoulder and level.   (fogofwar.ts gazeOf)
pub fn gaze_of(g: &mut crate::game::Game, c_2: CharId) -> V3 {
    let h = crate::character::character_head_dir(g, c_2);
    let body_yaw = g.ch[c_2].body_yaw;
    let aim = if g.ch[c_2].alive && g.ch[c_2].anim.long_shown == Some(crate::character::LongStance::Shoulder) {
        let gun_yaw = crate::character::character_gun_yaw(g, c_2);
        let (gun_dir, rl) = (g.ch[c_2].gun_dir, g.ch[c_2].anim.rl_shown);
        Some(GazeAim { gun_yaw, gun_x: gun_dir[0], gun_z: gun_dir[2], rl_carry: rl.carry, rl_head: rl.head })
    } else {
        None
    };
    gaze_from(h[0], h[2], body_yaw, aim)
}

/// What gazeOf reads of a shouldered gun.
#[derive(Clone, Copy)]
struct GazeAim { gun_yaw: f64, gun_x: f64, gun_z: f64, rl_carry: f64, rl_head: f64 }

/// gazeOf's arithmetic, on what it read of the man.
fn gaze_from(head_x: f64, head_z: f64, body_yaw: f64, aim: Option<GazeAim>) -> V3 {
    let (mut x, mut z) = (head_x, head_z);
    let hl = m::hyp2(x, z);
    if hl > 1e-6 { x /= hl; z /= hl; } else { x = m::sin(body_yaw); z = m::cos(body_yaw); }
    if let Some(a) = aim {
        let (ax, az, gl) = (m::sin(a.gun_yaw), m::cos(a.gun_yaw), m::hyp2(a.gun_x, a.gun_z));
        let lv = if gl > 1e-6 { (a.gun_x * ax + a.gun_z * az) / gl } else { 0.0 };
        let sights = gaze_sights();
        let t = m::min(1.0, m::max(0.0, (lv - sights[0]) / (sights[1] - sights[0])));
        let rl = m::min(1.0, m::max(a.rl_carry, a.rl_head));
        let w = t * t * (3.0 - 2.0 * t) * (1.0 - rl * rl * (3.0 - 2.0 * rl));
        if w > 0.0 {
            x += (ax - x) * w;
            z += (az - z) * w;
            let l = m::hyp2(x, z);
            if l > 1e-6 { x /= l; z /= l; } else { x = ax; z = az; }
        }
    }
    [x, 0.0, z]
}

/// FOG.eyes as the count the game uses: from five to the room the renderer has; a row that is no number is five.   (fogofwar.ts fogEyeCount)
pub fn fog_eye_count(v: f64) -> f64 {
    if v.is_nan() { FOG_EYES_WAS } else { m::max(FOG_EYES_WAS, m::min(crate::render_fogofwar::FOG_EYES, m::floor(v))) }
}

/// The address's words for the fog, read by the Game's constructor. With no page it does nothing, as the TypeScript's catch does; the host owns the address.   (fogofwar.ts fogFlags)
pub fn fog_flags(g: &mut crate::game::Game) {}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(b: u64) -> f64 { f64::from_bits(b) }

    /// The TypeScript's own gazeOf under EXACT, called with a made-up man: ([head x, head z, body yaw], shouldered,
    /// [gun yaw, gun dir x, gun dir z, rl carry, rl head], [out x, out z]).
    const GAZE: &[([u64; 3], bool, [u64; 5], [u64; 2])] = &[
    ([0x3fd3333333333333, 0xbfe6666666666666, 0x3ff199999999999a], false, [0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000], [0x3fd935f94a2a4a4b, 0xbfed69a2d686ac02]),
    ([0x0000000000000000, 0x0000000000000000, 0x4004000000000000], false, [0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000], [0x3fe326af0dcfcab0, 0xbfe9a2f7ef858b7d]),
    ([0x3fe3333333333333, 0x3fe999999999999a, 0x0000000000000000], true, [0x3feccccccccccccd, 0x3fe91103985da841, 0x3fe3e43a9692e21d, 0x0000000000000000, 0x0000000000000000], [0x3fe91103985da841, 0x3fe3e43a9692e21d]),
    ([0x3fe3333333333333, 0x3fe999999999999a, 0x0000000000000000], true, [0x3feccccccccccccd, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000], [0x3fe3333333333333, 0x3fe999999999999a]),
    ([0x3fe3333333333333, 0x3fe999999999999a, 0x0000000000000000], true, [0x3feccccccccccccd, 0x3fe91103985da841, 0x3fe3e43a9692e21d, 0x3ff0000000000000, 0x0000000000000000], [0x3fe3333333333333, 0x3fe999999999999a]),
    ([0x3fe3333333333333, 0x3fe999999999999a, 0x0000000000000000], true, [0x3feccccccccccccd, 0x3fe91103985da841, 0x3fe3e43a9692e21d, 0x3fc999999999999a, 0x3fd6666666666666], [0x3fe791f5a64d41d2, 0x3fe5a4c6979ca6a1]),
    ([0xbfd8ec3ae92b676b, 0xbfed7954e7dba2f7, 0x0000000000000000], true, [0x3fd999999999999a, 0x3fec17385bac4059, 0x3fdea75907b65840, 0x0000000000000000, 0x0000000000000000], [0x3fd8ec3ae92b676b, 0x3fed7954e7dba2f7]),
    ([0xbfeeb1b8ef000000, 0xbfeef0e14b400000, 0x3fd1585d83d1eb88], true, [0x40149991ead47ae1, 0xbfe8cd6d92400000, 0xbf80d64560000000, 0x3fe6e3e710333333, 0x3fa87c8ac8000000], [0xbfeac7b876e908e0, 0xbfe18430a0d14ce0]),
    ([0x3fdf34e0e2800000, 0x3fedcfd218000000, 0xc0056b3fd4ea8f5c], true, [0xbfdfeac9183851f0, 0xbfca800e6c000000, 0x3fed68cbf8c00000, 0x0000000000000000, 0x3ff31420b3cccccd], [0x3fddad8ad46cb05a, 0x3fec59f99c772ed0]),
    ([0x3fd4f423e3800000, 0xbfc3bd1d86000000, 0xbf3f04a49999a000], true, [0xbfeed3882b35c290, 0x3fd670d827000000, 0xbfe60f812ac00000, 0x0000000000000000, 0x3fd7ed9203cccccd], [0x3fecf312c51ab38b, 0xbfdb455cd91ad875]),
    ([0xbfef8c27a9c00000, 0xbfb31d4a38000000, 0xbff4eea43f57ae15], true, [0x4017398b94e428f7, 0xbfef029d6a000000, 0x3fe1e78275c00000, 0x0000000000000000, 0x0000000000000000], [0xbfe03a7fd5c5f6a6, 0x3feb9461e2811795]),
    ([0x3fed1e85c3000000, 0x3fb7f9cdf6000000, 0x3ff0ce7667ef5c2a], true, [0x3ff8cd3e15c28f5c, 0xbfe5ae16e8400000, 0x3feb75c98c800000, 0x3fe18d26d4999999, 0x3fd97aec41cccccd], [0x3fefd4f4395dabed, 0x3fba3593820f4aa7]),
    ([0x3fdb382a96800000, 0x3fc37834a8000000, 0x3ffa4487d777ae16], true, [0xc00d8b680f01eb85, 0xbfb60a8348000000, 0x3fd5ffa305800000, 0x3ff0c18062600000, 0x0000000000000000], [0x3fee2188186de9c4, 0x3fd58d5a2bad9c60]),
    ([0x3feb3dbb1bc00000, 0x3fdf021781000000, 0xc001f166acbbd70a], true, [0xbffe1c624967ae14, 0xbfd2259f55000000, 0x3fe2c2c22f400000, 0x3fcb0c82c7666666, 0x0000000000000000], [0x3febcfa615a5a613, 0x3fdfa8301aeaacfe]),
    ([0xbfc88f2ebf000000, 0x3fe7f89e6b000000, 0x3ff6746bf95147ae], true, [0xc009596612847ae2, 0xbfcba98e60000000, 0x3fe7ae1e1fc00000, 0x3fdc8d9ab5e66666, 0x3fea95136ba66666], [0xbfcfc28799428264, 0x3feeffd268ec5b28]),
    ([0x3facf48c98000000, 0x3fe9503813400000, 0x3fea63b9e7e147b0], true, [0xbff754a7aa266668, 0x3fe220b674400000, 0x3fe4d1923f000000, 0x0000000000000000, 0x3fe7767e3cc00000], [0x3fb241607fcb1865, 0x3fefeb250ffb290a]),
    ([0xbfe8cffc84000000, 0x3fc42d488f000000, 0x40038c4c7fcae147], false, [0xc006769a67a147ae, 0x3fe1ebdd4cc00000, 0x3fdd829c7b000000, 0x0000000000000000, 0x0000000000000000], [0xbfef5bca9428d3e9, 0x3fc9800207ad9130]),
    ([0xbfdbe4a2f4800000, 0x3fe866a422800000, 0xbfe5b4cc97b1eb84], true, [0x4005e2bbe35147ae, 0xbfea351228800000, 0x3fde725b92800000, 0x3fe57b5ab1733333, 0x0000000000000000], [0xbfdfc2210f7a4a09, 0x3febc840405c52a8]),
    ([0x3fe9d11c8a400000, 0xbfc999ea8c000000, 0xbffda2ec646c28f6], true, [0x40101e5957a19999, 0x3feffec97a000000, 0xbfefc36bf6400000, 0x3f9af207b8000000, 0x0000000000000000], [0x3fef0f4c532e65b3, 0xbfcecce4cb859514]),
    ([0xbfe901456d400000, 0x3f96cdd970000000, 0x4004b76ae8a428f5], true, [0xc003a46ed778f5c3, 0xbfd48792df000000, 0x3feb84952e400000, 0x3fcc65f1fe999999, 0x3fe723163f8ccccd], [0xbfeffcacd83d86ed, 0x3f9d2bee5ba2e108]),
    ([0xbfc48d0bc8000000, 0xbfd3fa2ae4800000, 0x4007fc2c6cb8f5c3], true, [0x3fff56d8089eb854, 0xbfb9a9454a000000, 0xbfa5203408000000, 0x0000000000000000, 0x0000000000000000], [0xbfdd460b97b781ff, 0xbfec74d3605d204a]),
    ([0xbfd29b57a0800000, 0x3fec5ec9c4800000, 0xbfe78a5c804b8520], false, [0x3ff93a83dd133334, 0x3fd5bed933000000, 0xbfba48b2e6000000, 0x3fc59bde91000000, 0x0000000000000000], [0xbfd3f146d840d751, 0x3fee6824ec1cbc55]),
    ([0x3fca50e411000000, 0x3fc3124e3c000000, 0x3febc8c0591d70a4], true, [0xbfe04bbc80f33330, 0x3fc34bc730000000, 0xbfd1d56798800000, 0x3fc2d3104f000000, 0x0000000000000000], [0x3fe9e93c8b5bf521, 0x3fe2c72f9f95e431]),
    ([0xbfe4390fa3400000, 0x3fc4269836000000, 0xc0090efb49075c29], false, [0x4012c3596ddfae15, 0xbfe012ef0d800000, 0x3fea94b828400000, 0x0000000000000000, 0x0000000000000000], [0xbfef0d128c90ee53, 0x3fcef0b7dd18fca8]),
    ([0x3fbb71fd3c000000, 0xbfef92caa6c00000, 0xbffcd8e04199999a], false, [0x400b280105dd70a6, 0xbfe1d3aae5c00000, 0xbfef2b3b53000000, 0x0000000000000000, 0x3fdd29c3b3e66666], [0x3fbba7407535744b, 0xbfefd010cb4e669c]),
    ([0x3fd6fdab94000000, 0x3fdcea2833800000, 0xc0032c00826ae148], false, [0x3ff5e3f227947ae0, 0xbfe7e24c5d400000, 0xbfcbaa2d92000000, 0x0000000000000000, 0x0000000000000000], [0x3fe3ea6f4b698efb, 0x3fe90c159f58ab64]),
    ([0x3fe155af3bc00000, 0xbfddc59d3f000000, 0x3fc5345c4b4cccd0], true, [0xbff19f657448f5c4, 0x3fdde3bfeb000000, 0x3fdf32da9e800000, 0x0000000000000000, 0x3ff302a99b266666], [0x3fe846ee04686fd2, 0xbfe4d8fea2e9b12b]),
    ([0xbfea02fe8fc00000, 0xbfe08236f5000000, 0xbfe7019ad1d851ec], true, [0x4015b4c45b619999, 0xbfc59b7be0000000, 0x3fc74e68ff000000, 0x0000000000000000, 0x0000000000000000], [0xbfe82e555a399ea3, 0x3fe4f58119392a9d]),
    ([0xbfe2efbdab000000, 0x3fdb53f251800000, 0x3ff78a4053b70a3e], true, [0xc002ddf225570a3e, 0xbfc4941779000000, 0xbfcb4173c6000000, 0x3fc6e9587d999999, 0x3ff2f6ae74400000], [0xbfe9f325bc0cd4e9, 0x3fe2b97b0aff4498]),
    ([0x3fdcce530a800000, 0xbfda319fc8000000, 0xbff1a9122a1ccccc], true, [0xbff66171e2228f5c, 0x3fe555d8a9000000, 0x3fa4c136cc000000, 0x3fe8a45e0df33333, 0x3feb4eba63733333], [0x3fe7ace580a7227e, 0xbfe5874c1a66d932]),
    ([0x3fecbb4dc2800000, 0xbfb33a8a3e000000, 0x4000da92dfb70a3d], false, [0x4001f26af1600002, 0xbfecc83316c00000, 0x3fe75d5430000000, 0x0000000000000000, 0x3fe60c44ffc00000], [0x3fefe37bf6da3353, 0xbfb5576a45b00e00]),
    ([0x3fe84253fd000000, 0xbfe45a314f400000, 0x3fe3495c0a266668], true, [0x401428be5ea66667, 0xbfe68b5bd6400000, 0xbfe47b8a89800000, 0x3fe1f1b3b7666666, 0x0000000000000000], [0x3fe883e12e231f3a, 0xbfe4912ffe2e03fc]),
    ];

    #[test]
    fn gaze_has_the_typescripts_bits() {
        for (i, (h, shoulder, a, want)) in GAZE.iter().enumerate() {
            let aim = shoulder.then(|| GazeAim { gun_yaw: f(a[0]), gun_x: f(a[1]), gun_z: f(a[2]), rl_carry: f(a[3]), rl_head: f(a[4]) });
            let o = gaze_from(f(h[0]), f(h[1]), f(h[2]), aim);
            assert_eq!([o[0].to_bits(), o[2].to_bits()], *want, "case {i}");
            assert_eq!(o[1].to_bits(), 0, "case {i}");
        }
    }

    #[test]
    fn eye_count_clamps_as_the_typescript_does() {
        for (v, want) in [(f64::NAN, 5.0), (f64::NEG_INFINITY, 5.0), (0.0, 5.0), (4.9, 5.0), (5.0, 5.0), (6.9, 6.0), (7.0, 7.0), (7.5, 7.0), (f64::INFINITY, 7.0)] {
            assert_eq!(fog_eye_count(v), want, "{v}");
        }
    }
}
