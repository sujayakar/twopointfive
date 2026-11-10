//! THE SKY TURNED BY HAND, for a visit: the cover of the weather that is picked, and the level's moon. A panel's rows; nothing of a run asks.
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3;

/// the cover of the weather that is picked, or nothing: none is
pub fn cover(g: &Game) -> Option<f64> { g.weather.kind_now.map(|_| crate::weather::weather_cloud(g)) }

/// the picked weather covers this much of the sky, and the hour's sun follows it
pub fn set_cover(g: &mut Game, c: f64) {
    let Some(k) = g.weather.kind_now else { return };
    if !c.is_finite() { return; }
    if let Some(p) = g.m.weather_kinds.weather_presets.get_mut(k) { p.cloud = v3::clamp(c, 0.0, 1.0); }
    super::sun_to_cloud(g);
    crate::host_api::host_api_invalidate_world_cache(g);
}

/// the level's steady directional light, up or not, which is not the hour's sun
pub fn moon(g: &Game) -> Option<LightId> {
    let sun = g.m.hours.states.as_ref().map(|s| s.sun);
    g.engine.lights.lights.iter().copied().find(|&l| { let r = &g.tab.light[l]; r.kind == 2.0 && r.ttl < 0.0 && !r.transient.unwrap_or(false) && Some(l) != sun })
}

/// how high it stands and where, in degrees (0 north, 90 east: the sun's convention)
pub fn moon_at(g: &Game, l: LightId) -> (f64, f64) {
    let d = g.tab.light[l].dir;
    (m::asin(v3::clamp(-d[1], -1.0, 1.0)).to_degrees(), (m::atan2(-d[0], d[2]).to_degrees() + 360.0) % 360.0)
}

/// the moon's strength, height and bearing; a number that is none leaves it
pub fn set_moon(g: &mut Game, k: f64, elev: f64, bearing: f64) {
    let Some(l) = moon(g) else { return };
    let (was_e, was_b) = moon_at(g, l);
    let e = (if elev.is_finite() { v3::clamp(elev, 3.0, 88.0) } else { was_e }).to_radians();
    let b = (if bearing.is_finite() { bearing } else { was_b }).to_radians();
    let r = &mut g.tab.light[l];
    if k.is_finite() { r.intensity = m::max(0.0, k); r.peak_intensity = r.intensity; }
    r.dir = [-m::sin(b) * m::cos(e), -m::sin(e), m::cos(b) * m::cos(e)];
}

#[cfg(test)]
mod tests {
    use super::*;

    /// THE RULE: the moon is turned by the numbers given and by no other, and stands where it was told.
    #[test]
    fn the_moon_goes_where_it_is_told() {
        let mut g = Game::blank();
        let l = g.tab.light.push(crate::render_lights::RtLight { kind: 2.0, dir: [0.0, -1.0, 0.0], intensity: 0.06, enabled: true, ttl: -1.0, ..crate::render_lights::RtLight::default() });
        g.engine.lights.lights.push(l);
        assert_eq!(moon(&g), Some(l));
        set_moon(&mut g, f64::NAN, 30.0, 90.0);
        let (e, b) = moon_at(&g, l);
        assert!((e - 30.0).abs() < 1e-6 && (b - 90.0).abs() < 1e-6 && g.tab.light[l].intensity == 0.06, "{e} {b}");
        set_moon(&mut g, 0.9, f64::NAN, f64::NAN);
        let (e, b) = moon_at(&g, l);
        assert!((e - 30.0).abs() < 1e-6 && (b - 90.0).abs() < 1e-6 && g.tab.light[l].peak_intensity == 0.9);
        assert_eq!(cover(&g), None);
    }
}
