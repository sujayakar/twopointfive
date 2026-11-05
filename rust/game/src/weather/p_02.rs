//! src/game/weather.ts lines 469 to 503
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The direction a stroke's light travels, for a stroke seen at this bearing and elevation.   (weather.ts dirOf)
pub fn dir_of(bearing: f64, elev: f64) -> V3 {
    let to: V3 = [m::sin(bearing) * m::cos(elev), m::sin(elev), -m::cos(bearing) * m::cos(elev)];
    [-to[0], -to[1], -to[2]]
}

/// The weather's frame with everything off.   (weather.ts zero)
pub fn zero(f: &mut crate::render_frame::WeatherFrame) {
    f.rain = 0.0;
    f.snow = Some(0.0);
    f.wet = 0.0;
    f.flash = [0.0, 0.0, 0.0];
    f.shear_x = 0.0;
    f.shear_z = 0.0;
    f.roof_c = Some(0.0);
    f.roof_d = Some(0.0);
    f.fog = Some(0.0);
}

/// Hearing through the weather: a sound's way with its length stretched by the noise at the listener.   (weather.ts hearPath)
pub fn hear_path(g: &mut crate::game::Game, src: V3, ear: V3, keep: Option<bool>) -> crate::scene_collision::Propagation {
    let p = crate::scene_collision::sound_paths_propagate(g, src, ear, Some(keep.unwrap_or(true)));
    let k = weather_hear_mask(g, ear);
    if k == 1.0 { p } else { crate::scene_collision::Propagation { path_len: p.path_len * k, ..p } }
}

/// The length a loud sound's path is taken at: the weather's stretch taken back out.   (weather.ts loudLen)
pub fn loud_len(g: &mut crate::game::Game, path_len: f64, ear: V3) -> f64 {
    let k = weather_hear_mask(g, ear);
    if k == 1.0 { path_len } else { path_len / k }
}

/// A stroke of lightning under a picked weather, added to the six faces of the light meter's stand-in.   (weather.ts strokeLight)
pub fn stroke_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) {
    let l = if g.weather.kind.is_some() { g.weather.flash.as_ref().map(|f| f.light) } else { None };
    let Some(l) = l else { return };
    let lamp = seam_stroke_lamp(g, l);
    if !lamp.enabled { return; }
    let i = (0.2126 * lamp.color[0] + 0.7152 * lamp.color[1] + 0.0722 * lamp.color[2]) * lamp.intensity;
    if !(i > 0.002) { return; }
    let (tx, ty, tz) = (-lamp.dir[0], -lamp.dir[1], -lamp.dir[2]);
    let t = if ty > 0.05 { m::min(60.0, m::max(0.0, (crate::scene_world::world().ceiling_y + 6.0 - p[1]) / ty)) } else { 60.0 };
    if t > 0.05 && crate::scene_collision::segment_blocked(g, p, [p[0] + tx * t, p[1] + ty * t, p[2] + tz * t], None, None, None) { return; }
    if tx > 0.0 { acc[0] += i * tx; } else { acc[1] -= i * tx; }
    if ty > 0.0 { acc[2] += i * ty; } else { acc[3] -= i * ty; }
    if tz > 0.0 { acc[4] += i * tz; } else { acc[5] -= i * tz; }
}

/// Sight through the weather: the fog's transmittance along a to b.   (weather.ts airTrans)
pub fn air_trans(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    weather_air_trans(g, a, b)
}

/// What `stroke_light` reads of the stroke's lamp (`RtLight`).
struct StrokeLamp {
    enabled: bool,
    color: V3,
    intensity: f64,
    dir: V3,
}

/// `l.enabled`, `l.color`, `l.intensity`, `l.dir` of an `RtLight` (weather.ts:493 to 495)
fn seam_stroke_lamp(g: &mut Game, l: LightId) -> StrokeLamp {
    todo!("needs: a way to read an RtLight's fields from its LightId")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `dirOf`'s doubles from the TypeScript, under bun with EXACT on.
    #[test]
    fn dir_of_has_the_typescript_bits() {
        let rows: [(u64, u64, [u64; 3]); 5] = [
            (0x3fe6666666666666, 0x3fd999999999999a, [0xbfe2fcd60b856028, 0xbfd8ec3ae92b676b, 0x3fe68afcebc95d21]),
            (0x4002666666666666, 0x3ff199999999999a, [0xbfd5a5df199aaf8d, 0xbfec84c54c2d6338, 0xbfd3579379410117]),
            (0x0000000000000000, 0x3fe0000000000000, [0x8000000000000000, 0xbfdeaee8744b05f0, 0x3fec1528065b7d50]),
            (0xbffe666666666666, 0x3fa999999999999a, [0x3fee3e66f827a4f7, 0xbfa996dea2ff643c, 0xbfd4aa281baef78e]),
            (0x3ff921fb54442d18, 0x3feccccccccccccd, [0xbfe3e43a9692e21d, 0xbfe91103985da841, 0x3c85f10d9d62342f]),
        ];
        for (b, e, want) in rows {
            let d = dir_of(f64::from_bits(b), f64::from_bits(e));
            assert_eq!([d[0].to_bits(), d[1].to_bits(), d[2].to_bits()], want);
        }
    }
}
